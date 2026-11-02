//! Install receipts: what one package transaction did, however it was started.
//! Built after the fact from the package database (files and install scripts of
//! each changed package) and the timeline (what started listening within 60 s).
//! Deterministic `receipt.*` rules triage it; the score selects the reviewer mode.
use crate::{
    model::{Coverage, Finding, Severity, SkipReason, Skipped},
    platform::{ChangeKind, PackageChange, PackageDb, PackageScript, Transaction},
    runner::Cancel,
    store::digest,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    sync::LazyLock,
};
use ts_rs::TS;

/// After this long, what a transaction started is attached to its receipt.
pub const AFTER_SECS: u64 = 60;
/// Files listed per receipt; a kernel update alone ships thousands.
const MAX_FILES: usize = 5_000;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct ReceiptFile {
    pub path: PathBuf,
    pub package: String,
    /// "/etc", "systemd", "cron", "PAM", "libraries", "binaries" or "other".
    pub category: String,
    /// "setuid", "setgid" or "file capabilities", when the installed file has them.
    pub privilege: Option<String>,
}

/// A rule that matched one line of one install script.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct ScriptHit {
    /// Index into `Receipt::scripts`.
    pub script: usize,
    /// 1-based line number.
    pub line: u32,
    pub rule: String,
    pub severity: crate::model::Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct ReceiptScript {
    #[serde(flatten)]
    pub script: PackageScript,
    /// Differs from the same package's script in an earlier receipt.
    pub changed: bool,
}

/// What appeared within a minute of the transaction.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct AfterEffects {
    /// False until the minute after the transaction has been checked.
    pub checked: bool,
    /// New listening programs, as timeline sentences.
    pub listeners: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct Receipt {
    pub id: String,
    pub at: u64,
    pub manager: String,
    pub transaction: String,
    pub command: String,
    pub packages: Vec<PackageChange>,
    pub files: Vec<ReceiptFile>,
    pub scripts: Vec<ReceiptScript>,
    pub hits: Vec<ScriptHit>,
    /// systemd services and timers, and autostart entries, the packages ship.
    pub services: Vec<PathBuf>,
    /// The repository each package comes from, when the package manager says.
    #[serde(default)]
    pub repositories: std::collections::BTreeMap<String, String>,
    pub after: AfterEffects,
    pub findings: Vec<Finding>,
    pub coverage: Coverage,
    /// High 3, Review 1 per finding; selects the reviewer mode (plan Part G).
    pub score: u32,
}

/// Packages whose changes matter more than most (plan Part F).
fn sensitive_package(name: &str) -> bool {
    const EXACT: [&str; 14] = [
        "sudo",
        "pam",
        "glibc",
        "libc6",
        "xz",
        "xz-libs",
        "xz-utils",
        "liblzma5",
        "curl",
        "bash",
        "zsh",
        "dash",
        "polkit",
        "shadow-utils",
    ];
    const PREFIX: [&str; 8] = [
        "openssh",
        "systemd",
        "openssl",
        "libcurl",
        "kernel",
        "linux-image",
        "libpam",
        "libssl",
    ];
    EXACT.contains(&name) || PREFIX.iter().any(|p| name.starts_with(p)) || name == "linux"
}

pub fn file_category(path: &Path) -> &'static str {
    let p = path.to_string_lossy();
    if p.contains("/systemd/") {
        "systemd"
    } else if p.contains("/cron") {
        "cron"
    } else if p.starts_with("/etc/pam.d")
        || p.contains("/security/pam_")
        || p.contains("/lib64/security/")
        || p.contains("/lib/security/")
    {
        "PAM"
    } else if p.starts_with("/etc/") {
        "/etc"
    } else if p.contains("/lib/")
        || p.contains("/lib64/")
        || p.ends_with(".so")
        || p.contains(".so.")
    {
        "libraries"
    } else if p.contains("/bin/") || p.contains("/sbin/") || p.contains("/libexec/") {
        "binaries"
    } else {
        "other"
    }
}

/// Startup units and autostart entries a package ships.
fn starts_things(path: &Path) -> bool {
    let p = path.to_string_lossy();
    (p.contains("/systemd/system/") || p.contains("/systemd/user/"))
        && (p.ends_with(".service") || p.ends_with(".timer") || p.ends_with(".socket"))
        || p.contains("/xdg/autostart/") && p.ends_with(".desktop")
}

fn privilege(path: &Path) -> Option<String> {
    let meta = path.symlink_metadata().ok()?;
    if !meta.is_file() {
        return None;
    }
    let mode = meta.permissions().mode();
    if mode & 0o4000 != 0 {
        return Some("setuid".into());
    }
    if mode & 0o2000 != 0 {
        return Some("setgid".into());
    }
    use std::os::unix::ffi::OsStrExt;
    let c_path = std::ffi::CString::new(path.as_os_str().as_bytes()).ok()?;
    let n = unsafe {
        libc::lgetxattr(
            c_path.as_ptr(),
            c"security.capability".as_ptr(),
            std::ptr::null_mut(),
            0,
        )
    };
    (n > 0).then(|| "file capabilities".into())
}

struct LineRule {
    id: &'static str,
    severity: Severity,
    summary: &'static str,
    action: &'static str,
    matches: fn(&str) -> bool,
}

static SENSITIVE_PATH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(/etc/(profile\.d|profile\b|bashrc|bash\.bashrc|environment|ld\.so\.preload|ld\.so\.conf\.d|pam\.d|security/|sudoers|cron|systemd/system|xdg/autostart|udev/rules\.d|ssh/sshd_config|modprobe\.d|modules-load\.d|rc\.local|update-motd\.d)|/var/spool/cron|/root/|\.ssh/authorized_keys|/home/)")
        .expect("valid regex")
});
static WRITE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(>>?\s*\S|\b(tee|cp|install|mv|ln|touch|chmod|chown|mkdir)\b|\bsed\s+(-[a-z]*i|--in-place))").expect("valid regex")
});
static DOWNLOAD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(curl|wget|fetch|aria2c)\b").expect("valid regex"));
static OBFUSCATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(base64\s+(-d|--decode)|\beval\b|[A-Za-z0-9+/]{200,}={0,2}|(\\x[0-9a-fA-F]{2}){16,}|[\u{200B}-\u{200F}\u{202A}-\u{202E}\u{2066}-\u{2069}])").expect("valid regex")
});

const LINE_RULES: [LineRule; 3] = [
    LineRule {
        id: "receipt.hook.sensitive_path",
        severity: Severity::Review,
        summary: "An install script writes to a startup or authentication path",
        action: "Read the line: packages rarely need to change these paths from a script. If you did not expect it, reinstall from your distribution's repository or remove the package.",
        matches: |line| SENSITIVE_PATH.is_match(line) && WRITE.is_match(line),
    },
    LineRule {
        id: "receipt.hook.download",
        severity: Severity::High,
        summary: "An install script downloads something",
        action: "Distribution packages ship their files; a script that fetches more at install time runs code nobody reviewed. Find out what it downloads before trusting the package.",
        matches: |line| DOWNLOAD.is_match(line),
    },
    LineRule {
        id: "receipt.hook.obfuscation",
        severity: Severity::Review,
        summary: "An install script contains encoded or hidden code",
        action: "Decode the line by hand (never by running it) and check what it does.",
        matches: |line| OBFUSCATION.is_match(line),
    },
];

fn summary_of(t: &Transaction) -> String {
    crate::sources::transaction_summary(t)
}

/// Everything a receipt needs that is not in the package database.
pub struct History<'a> {
    /// The digest of a package's script for a phase in an earlier receipt.
    pub hook: &'a dyn Fn(&str, &str) -> Option<String>,
}

/// Builds the receipt for one transaction. Removed packages are gone from the
/// database, so only their names are recorded.
pub fn build(
    id: String,
    t: &Transaction,
    db: &dyn PackageDb,
    history: &History,
    stop: &Cancel,
) -> Receipt {
    let mut r = Receipt {
        id,
        at: t.at,
        manager: t.manager.clone(),
        transaction: t.id.clone(),
        command: t.command.clone(),
        packages: t.changes.clone(),
        files: vec![],
        scripts: vec![],
        hits: vec![],
        services: vec![],
        repositories: Default::default(),
        after: AfterEffects::default(),
        findings: vec![],
        coverage: Coverage::default(),
        score: 0,
    };
    r.coverage
        .checked
        .push(format!("Package database ({})", db.manager()));
    r.coverage.push(
        "Built after the transaction from the package database: the files and install scripts of each installed or updated package. Bigman did not see the transaction run, and old package versions are not compared yet.".into(),
    );
    let mut listed = 0usize;
    for change in t.changes.iter().filter(|c| c.kind != ChangeKind::Remove) {
        match db.files(&change.name, stop) {
            Ok(files) => {
                for path in files {
                    if starts_things(&path) {
                        r.services.push(path.clone());
                    }
                    let privilege = privilege(&path);
                    if let Some(kind) = &privilege {
                        r.findings.push(finding(
                            "receipt.file.privileged",
                            Severity::Review,
                            &format!("{} installs {} with {kind}", change.name, path.display()),
                            format!("{}\n{} {}", path.display(), change.name, change.version),
                            "Programs that run with raised privileges are worth knowing about. Check this one is expected for the package.",
                        ));
                    }
                    listed += 1;
                    if listed <= MAX_FILES {
                        r.files.push(ReceiptFile {
                            category: file_category(&path).into(),
                            package: change.name.clone(),
                            path,
                            privilege,
                        });
                    }
                }
            }
            Err(e) => r.coverage.skipped.push(Skipped {
                what: format!("Files of {}", change.name),
                why: SkipReason::Failed,
                detail: format!("{e:#}"),
            }),
        }
        match db.scripts(&change.name, stop) {
            Ok(scripts) => {
                for script in scripts {
                    let changed = (history.hook)(&script.package, &script.phase)
                        .is_some_and(|d| d != digest(script.text.as_bytes()));
                    r.scripts.push(ReceiptScript { script, changed });
                }
            }
            Err(e) => r.coverage.skipped.push(Skipped {
                what: format!("Install scripts of {}", change.name),
                why: SkipReason::Failed,
                detail: format!("{e:#}"),
            }),
        }
        if sensitive_package(&change.name) {
            r.findings.push(finding(
                "receipt.sensitive_package",
                Severity::Info,
                &format!("{} is part of how this computer authenticates, connects or boots", change.name),
                format!("{:?} {} {}", change.kind, change.name, change.version),
                "Nothing to do when the update came from your distribution; it is listed so the change is easy to find later.",
            ));
        }
    }
    if listed > MAX_FILES {
        r.coverage.push(format!(
            "{listed} files changed; the first {MAX_FILES} are listed."
        ));
    }
    let removed: Vec<&str> = t
        .changes
        .iter()
        .filter(|c| c.kind == ChangeKind::Remove)
        .map(|c| c.name.as_str())
        .collect();
    if !removed.is_empty() {
        r.coverage.push(format!("Removed packages are gone from the database, so their files and scripts are not listed: {}.", removed.join(", ")));
    }
    triage_scripts(&mut r);
    r.score = score(&r.findings);
    r
}

fn finding(
    rule: &str,
    severity: Severity,
    summary: &str,
    evidence: String,
    action: &str,
) -> Finding {
    Finding {
        rule: rule.into(),
        severity,
        summary: summary.into(),
        evidence: crate::model::clean(&evidence),
        action: action.into(),
    }
}

/// The line rules over every script, and changed hooks.
pub fn triage_scripts(r: &mut Receipt) {
    for (index, s) in r.scripts.iter().enumerate() {
        let name = format!("{} {} script", s.script.package, s.script.phase);
        if s.changed {
            r.findings.push(finding(
                "receipt.hook.changed",
                Severity::Review,
                &format!("The {name} changed since the last version"),
                s.script.text.clone(),
                "Read what the script does now; a changed install hook in an ordinary update is worth a look.",
            ));
        }
        for (n, line) in s.script.text.lines().enumerate() {
            for rule in LINE_RULES.iter().filter(|rule| (rule.matches)(line)) {
                r.hits.push(ScriptHit {
                    script: index,
                    line: n as u32 + 1,
                    rule: rule.id.into(),
                    severity: rule.severity,
                });
                r.findings.push(finding(
                    rule.id,
                    rule.severity,
                    rule.summary,
                    format!("{name}, line {}: {}", n + 1, line.trim()),
                    rule.action,
                ));
            }
        }
    }
}

pub fn score(findings: &[Finding]) -> u32 {
    findings
        .iter()
        .map(|f| match f.severity {
            Severity::High => 3,
            Severity::Review => 1,
            Severity::Info => 0,
        })
        .sum()
}

/// Attaches what started listening in the minute after the transaction.
pub fn attach_after(r: &mut Receipt, listener_summaries: Vec<String>) {
    for summary in &listener_summaries {
        r.findings.push(finding(
            "receipt.after.listener",
            Severity::Review,
            "A program started listening right after the transaction",
            summary.clone(),
            "Check whether the package is meant to run a server, and who can reach it (Network > Listening programs).",
        ));
    }
    r.after = AfterEffects {
        checked: true,
        listeners: listener_summaries,
    };
    r.score = score(&r.findings);
}

/// One plain sentence for the timeline: "dnf installed nix and nix-libs · 1 finding to review".
pub fn sentence(r: &Receipt, t: &Transaction) -> String {
    let to_review = r
        .findings
        .iter()
        .filter(|f| f.severity != Severity::Info)
        .count();
    let what = summary_of(t);
    match to_review {
        0 => what,
        1 => format!("{what} · 1 finding to review"),
        n => format!("{what} · {n} findings to review"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::Attribution;
    use std::collections::BTreeMap;

    struct Fake;
    impl PackageDb for Fake {
        fn manager(&self) -> &'static str {
            "pacman"
        }
        fn attribute(
            &self,
            _: &[PathBuf],
            _: &Cancel,
        ) -> anyhow::Result<BTreeMap<PathBuf, Attribution>> {
            Ok(BTreeMap::new())
        }
        fn files(&self, _: &str, _: &Cancel) -> anyhow::Result<Vec<PathBuf>> {
            Ok(vec![
                "/usr/bin/tool".into(),
                "/usr/lib/systemd/system/tool.service".into(),
            ])
        }
        fn scripts(&self, package: &str, _: &Cancel) -> anyhow::Result<Vec<PackageScript>> {
            Ok(vec![PackageScript {
                package: package.into(),
                phase: "install".into(),
                interpreter: "/bin/sh".into(),
                text: "post_install() {\n  echo 'export TOOL=1' > /etc/profile.d/tool.sh\n}".into(),
            }])
        }
    }

    #[test]
    fn the_plan_acceptance_receipt() {
        let t = Transaction {
            id: "1".into(),
            at: 100,
            manager: "pacman".into(),
            command: "pacman -S tool".into(),
            changes: vec![PackageChange {
                name: "tool".into(),
                version: "1.0-1".into(),
                kind: ChangeKind::Install,
            }],
        };
        let r = build(
            "r".into(),
            &t,
            &Fake,
            &History { hook: &|_, _| None },
            &crate::runner::cancel(),
        );
        assert_eq!(r.scripts.len(), 1, "one hook");
        assert_eq!(
            r.hits,
            [ScriptHit {
                script: 0,
                line: 2,
                rule: "receipt.hook.sensitive_path".into(),
                severity: Severity::Review
            }]
        );
        assert_eq!(
            r.services,
            [PathBuf::from("/usr/lib/systemd/system/tool.service")]
        );
        assert_eq!(
            sentence(&r, &t),
            "pacman installed tool · 1 finding to review"
        );
        assert_eq!(r.score, 1);
    }

    #[test]
    fn a_changed_hook_is_noticed_against_the_last_receipt() {
        let t = Transaction {
            id: "2".into(),
            at: 1,
            manager: "pacman".into(),
            command: String::new(),
            changes: vec![PackageChange {
                name: "tool".into(),
                version: "1.1-1".into(),
                kind: ChangeKind::Upgrade,
            }],
        };
        let r = build(
            "r".into(),
            &t,
            &Fake,
            &History {
                hook: &|_, _| Some("an older digest".into()),
            },
            &crate::runner::cancel(),
        );
        assert!(r.findings.iter().any(|f| f.rule == "receipt.hook.changed"));
    }

    #[test]
    fn categorises_files_like_the_file_tree() {
        assert_eq!(file_category(Path::new("/etc/pam.d/sudo")), "PAM");
        assert_eq!(
            file_category(Path::new("/usr/lib64/security/pam_unix.so")),
            "PAM"
        );
        assert_eq!(
            file_category(Path::new("/usr/lib/systemd/system/sshd.service")),
            "systemd"
        );
        assert_eq!(
            file_category(Path::new("/usr/lib64/libssl.so.3")),
            "libraries"
        );
        assert_eq!(file_category(Path::new("/usr/bin/ssh")), "binaries");
        assert_eq!(file_category(Path::new("/usr/share/doc/x")), "other");
    }
}
