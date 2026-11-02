//! The rule registry: every finding Bigman can produce, with its stable ID,
//! default severity, plain explanation, remediation and the threats it covers.
//! `fixtures/<rule-id>/` holds a synthetic input that fires it (tests/fixtures.rs),
//! and `docs/rules.md` is generated from this table.
use crate::model::Severity::{self, High, Info, Review};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub struct Rule {
    pub id: &'static str,
    /// The usual severity. A few rules vary by context and say so in `explanation`.
    pub severity: Severity,
    pub title: &'static str,
    /// What the finding means, in plain language.
    pub explanation: &'static str,
    /// What the reader can do about it.
    pub remediation: &'static str,
    /// IDs from docs/threats this rule covers in part or whole.
    pub threats: &'static [&'static str],
    /// Operating systems the rule applies to.
    pub platforms: &'static [&'static str],
}

const LINUX: &[&str] = &["linux"];

macro_rules! rule {
    ($id:literal, $severity:expr, $title:literal, $explanation:literal, $remediation:literal, [$($threat:literal),*]) => {
        Rule {
            id: $id,
            severity: $severity,
            title: $title,
            explanation: $explanation,
            remediation: $remediation,
            threats: &[$($threat),*],
            platforms: LINUX,
        }
    };
}

pub const RULES: &[Rule] = &[
    rule!("shell.download", Review, "Downloads remote content",
        "The script fetches something from the network with curl or wget. What it fetches is not part of this inspection.",
        "Inspect each downloaded payload and confirm where it comes from before running the script.", ["DP-18"]),
    rule!("shell.execute-download", High, "Downloads and runs code",
        "The script pipes downloaded content straight into a shell, so whatever the server sends at that moment runs on your machine.",
        "Download the payload separately, inspect the exact bytes, and run only what you reviewed.", ["DP-18", "PH-17"]),
    rule!("shell.privilege", Review, "Asks for administrator rights",
        "The script runs commands with sudo, doas or su. Those commands can change anything on the system.",
        "Check why each privileged command is needed and whether it could run without administrator rights.", ["DP-18"]),
    rule!("shell.credentials", High, "Touches credentials or browser data",
        "The script refers to SSH keys, cloud or registry tokens, or browser profile data. Installers rarely need these.",
        "Run it without your real credentials present, and find out why it reads these paths.", ["CR-01", "CR-02", "CR-03", "CR-18", "SC-23"]),
    rule!("shell.persistence", Review, "Sets something up to run later",
        "The script writes cron jobs, systemd units, autostart entries, shell profiles or SSH authorized keys, so its effect outlasts the install.",
        "Review exactly what will run in future sessions and compare the startup files afterwards.", ["HI-02", "HI-03", "HI-04", "HI-05", "HI-10"]),
    rule!("shell.dynamic", Review, "Runs code that is built or decoded at run time",
        "The script uses eval, source, base64 or an interpreter on generated input. Static rules cannot see what that code will be.",
        "Decode or expand the dynamic part by hand and inspect it before running the script.", ["DP-18"]),
    rule!("shell.permissions", High, "Sets broad or privileged file permissions",
        "The script makes files world-writable or setuid. Either can let other programs or users gain access they should not have.",
        "Prefer the narrowest permissions the program needs, and never setuid a file you did not build.", ["HI-17"]),
    rule!("shell.delete", Review, "Deletes files recursively or by wildcard",
        "The script removes files with rm -r, rm -f or a wildcard. A wrong variable can expand to the wrong path.",
        "Check what each deleted path expands to before running the script.", ["DP-18"]),
    rule!("shell.network-listener", Review, "Uses a tool that can open or forward network connections",
        "The script runs nc, ncat, netcat or socat, which can listen for connections or relay a shell.",
        "Check which address and port it uses, and whether it listens to the whole network.", ["NE-14"]),
    rule!("shellcheck.warning", Review, "ShellCheck warning",
        "ShellCheck found a shell correctness problem. It is about how the script is written, not whether it is malicious.",
        "Read the ShellCheck code linked in the message to see what can go wrong.", ["DP-18"]),
    rule!("package.lifecycle", Review, "Package runs a script during install",
        "A preinstall, install, postinstall or prepare script runs automatically when the package is installed.",
        "Read the script and the files it calls. Turning off automatic install scripts for the project stops them running.", ["SC-01"]),
    rule!("package.script", Info, "Package defines a script",
        "A script that runs only when you ask for it, for example npm test.",
        "Read it before you run it.", ["SC-01"]),
    rule!("package.unresolved-hook", Review, "Lockfile lists an install script that is not on disk",
        "The lockfile says a dependency has an install script, but the package is not installed here, so its script could not be read.",
        "Inspect that package's archive without running its scripts.", ["SC-01", "SC-02"]),
    rule!("trace.sensitive-path", Review, "The rehearsal touched a sensitive path",
        "While running in the offline sandbox, the installer opened or looked for credential or browser paths. The sandbox has no real ones.",
        "Check the system call result; the access may have failed. Find out why the installer wanted that path.", ["CR-18", "CR-22"]),
    rule!("trace.network", Review, "The rehearsal tried to connect to the network",
        "While running in the offline sandbox, the installer attempted a network connection. The sandbox has no network.",
        "Find out what it was trying to reach and why.", ["NE-14"]),
    rule!("trace.process", Info, "The rehearsal started a program",
        "A program started while the installer ran in the offline sandbox.",
        "Review the program and its arguments.", ["DP-18"]),
    rule!("pdf.structure", Info, "The PDF contains an active or external element",
        "The document has an open action, JavaScript, a launch action, an embedded file, a link or encryption. JavaScript and launch actions are Review; the rest are Info.",
        "Check the parser output and the link destinations. A tag alone is not evidence of malware.", ["DS-12"]),
    rule!("clamav.match", High, "A file matches a ClamAV signature",
        "ClamAV recognised the file as matching a known-malware or heuristic signature. Limit or encryption signatures mean the content was not fully examined.",
        "If you did not expect the file, quarantine it; Bigman keeps a recovery copy you can restore.", ["DP-08", "MW-01"]),
    rule!("clamav.no-definitions", High, "No malware signatures found",
        "Neither Bigman's private signature folder nor /var/lib/clamav has ClamAV definitions, so file scans cannot run.",
        "Update signatures from Protect > Scans, or point ClamAV at your own database.", ["DP-20"]),
    rule!("clamav.stale", Review, "Malware signatures are out of date",
        "The newest ClamAV definitions file is more than three days old. File age is a freshness signal, not a verified version.",
        "Update signatures from Protect > Scans.", ["DP-20"]),
    rule!("tool.missing", Info, "An optional tool is not installed",
        "One of the tools Bigman can use is not installed. Only the checks that need it are affected.",
        "Install it from Setup if you want those checks.", []),
    rule!("monitor.stopped", Review, "Canary monitoring is not reporting",
        "The canary monitor has not written a heartbeat in the last ten seconds, so reads of decoy files are not being noticed.",
        "Start monitoring from Protect > Canaries, or install the background services from Setup.", ["CR-22", "HI-24"]),
    rule!("git.plaintext-credentials", Review, "Git stores credentials in plain text",
        "Git's global credential helper is 'store', which writes tokens unencrypted to ~/.git-credentials. The file itself was not read.",
        "Switch to your desktop keyring's credential helper and revoke tokens you no longer use.", ["CR-01", "CR-08"]),
    rule!("persistence.change", Review, "A startup file changed",
        "A file that starts programs for you (a systemd unit, autostart entry, shell profile, cron file) is new or different from the baseline you accepted.",
        "Review the file and the package that owns it before accepting it into the baseline.", ["HI-01", "HI-02", "HI-03", "HI-04", "HI-05", "HI-10"]),
    rule!("persistence.removed", Review, "A startup file was removed",
        "A startup file in your baseline is gone.",
        "Check whether an update or your own change explains it.", ["HI-01", "HI-02", "HI-03", "HI-04", "HI-05"]),
    rule!("persistence.initial", Info, "Startup file recorded for the first baseline",
        "There is no baseline yet, so every startup file is listed for review.",
        "Review the list, then accept it as the baseline to compare future changes against.", ["HI-01", "HI-02", "HI-03", "HI-04", "HI-05"]),
    rule!("canary.event", High, "A decoy file was opened or changed",
        "A canary decoy was read, changed, moved or removed while the monitor was running. inotify cannot say which program did it.",
        "Check what was running at that time. Backups, indexers and your own tests can also trigger decoys.", ["CR-22"]),
    rule!("canary.verified", Info, "Local read detection verified",
        "A deliberate read of the decoy produced a fresh receipt from the running monitor, so local detection works.",
        "Leave monitoring running.", ["CR-22"]),
    rule!("canary.notifications-unavailable", Info, "Desktop pop-ups are not available",
        "notify-send is not installed, so canary alerts appear in Activity but not as pop-ups.",
        "Install your distribution's desktop notification utilities if you want pop-ups.", []),
    rule!("lynis.finding", Review, "Lynis recommendation",
        "Lynis reported a warning or suggestion. The line is shown exactly as Lynis wrote it.",
        "Look up the Lynis test ID before changing configuration.", ["HI-24"]),
    rule!("osv.advisory", Review, "A dependency has a published advisory",
        "OSV lists a known vulnerability for a version this project depends on. A vulnerability is not evidence of malware.",
        "Read the advisory's affected and fixed versions and update the dependency.", ["SC-05", "SC-10"]),
    rule!("yara.output", Review, "YARA-X rules matched",
        "Rules you supplied matched the target. A match is the result of those rules, not proof of malware.",
        "Check where the rules came from and what they look for.", ["MW-01"]),
];

pub fn get(id: &str) -> Option<&'static Rule> {
    RULES.iter().find(|r| r.id == id)
}

/// A rule as the UI shows it beside a finding.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct RuleInfo {
    pub id: String,
    pub severity: Severity,
    pub title: String,
    pub explanation: String,
    pub remediation: String,
    pub threats: Vec<String>,
}

impl From<&Rule> for RuleInfo {
    fn from(r: &Rule) -> Self {
        RuleInfo {
            id: r.id.into(),
            severity: r.severity,
            title: r.title.into(),
            explanation: r.explanation.into(),
            remediation: r.remediation.into(),
            threats: r.threats.iter().map(|t| (*t).into()).collect(),
        }
    }
}

pub fn infos() -> Vec<RuleInfo> {
    RULES.iter().map(RuleInfo::from).collect()
}

/// docs/rules.md, generated from the registry; a test keeps the file current.
pub fn markdown() -> String {
    let mut out = String::from(
        "# Bigman rules\n\n> Generated from `crates/core/src/rules/mod.rs`. Do not edit by hand: run `BIGMAN_UPDATE_RULES_DOC=1 cargo test -p bigman-core --test fixtures`.\n\nEvery finding Bigman shows comes from one of these rules. Each has a synthetic fixture in `fixtures/<rule-id>/` that must fire it.\n\n| Rule | Severity | Title | Threats |\n|---|---|---|---|\n",
    );
    for r in RULES {
        out.push_str(&format!(
            "| `{}` | {:?} | {} | {} |\n",
            r.id,
            r.severity,
            r.title,
            r.threats.join(", ")
        ));
    }
    for r in RULES {
        out.push_str(&format!(
            "\n## `{}`\n\n**{}** ({:?})\n\n{}\n\nWhat you can do: {}\n",
            r.id, r.title, r.severity, r.explanation, r.remediation
        ));
    }
    out
}
