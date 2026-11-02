//! Startup and system-configuration files against a baseline you accepted. Each
//! entry records content, mode, owner and file capabilities; changes are ranked by
//! package attribution, so an update the package database explains stays quiet and
//! an edited PAM file owned by a package stands out.
use crate::{
    model::{now, Report, Severity},
    platform::{packagedb, Attribution, FileState},
    runner::{self, Cancel},
    store::{digest, home, read_file, Store, MAX_INPUT},
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
    process::Command,
};

/// Baselines written before this format are treated as absent (plan Part K, decision 2).
pub const FORMAT: u32 = 2;

/// What a startup or configuration file is for.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Services,
    Autostart,
    Shell,
    Cron,
    Boot,
    Login,
    Pam,
    Loader,
    KernelModules,
    Devices,
    Ssh,
    NameResolution,
    Desktop,
}

impl Category {
    pub fn label(self) -> &'static str {
        match self {
            Category::Services => "systemd unit",
            Category::Autostart => "autostart entry",
            Category::Shell => "shell profile or environment",
            Category::Cron => "scheduled job",
            Category::Boot => "boot script",
            Category::Login => "login message script",
            Category::Pam => "PAM authentication",
            Category::Loader => "dynamic loader",
            Category::KernelModules => "kernel module loading",
            Category::Devices => "udev device rule",
            Category::Ssh => "SSH configuration",
            Category::NameResolution => "name resolution",
            Category::Desktop => "desktop file handler",
        }
    }
    /// Where a package-owned file that differs from its package is High, not Review.
    pub fn sensitive(self) -> bool {
        matches!(self, Category::Pam | Category::Loader | Category::Ssh)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    /// SHA-256 of the content (or of a symlink's target path); none when only root can read it.
    pub hash: Option<String>,
    pub bytes: u64,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    /// The `security.capability` attribute, hex-encoded, when present.
    pub caps: Option<String>,
    pub category: Category,
}

impl Entry {
    fn privileged(&self) -> bool {
        self.mode & 0o6000 != 0 || self.caps.is_some()
    }
    fn describe(&self) -> String {
        format!(
            "mode {:04o}, owner {}:{}{}",
            self.mode & 0o7777,
            self.uid,
            self.gid,
            self.caps
                .as_ref()
                .map(|c| format!(", file capabilities {c}"))
                .unwrap_or_default()
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Baseline {
    #[serde(default)]
    pub format: u32,
    pub created: u64,
    pub entries: BTreeMap<PathBuf, Entry>,
    pub errors: Vec<String>,
}

/// The rootless set: everything here is readable, or at least listable, as you.
fn roots() -> Vec<(PathBuf, Category)> {
    use Category::*;
    let h = home();
    let mut roots = vec![
        (h.join(".config/systemd/user"), Services),
        (PathBuf::from("/etc/systemd/system"), Services),
        (PathBuf::from("/etc/systemd/user"), Services),
        (h.join(".config/autostart"), Autostart),
        (PathBuf::from("/etc/xdg/autostart"), Autostart),
        (PathBuf::from("/etc/crontab"), Cron),
        (PathBuf::from("/etc/anacrontab"), Cron),
        (PathBuf::from("/etc/rc.local"), Boot),
        (PathBuf::from("/etc/rc.d/rc.local"), Boot),
        (PathBuf::from("/etc/init.d"), Boot),
        (PathBuf::from("/etc/update-motd.d"), Login),
        (PathBuf::from("/etc/pam.d"), Pam),
        (PathBuf::from("/etc/ld.so.preload"), Loader),
        (PathBuf::from("/etc/ld.so.conf"), Loader),
        (PathBuf::from("/etc/ld.so.conf.d"), Loader),
        (PathBuf::from("/etc/modules-load.d"), KernelModules),
        (PathBuf::from("/etc/modprobe.d"), KernelModules),
        (PathBuf::from("/etc/modules"), KernelModules),
        (PathBuf::from("/etc/udev/rules.d"), Devices),
        (h.join(".ssh/authorized_keys"), Ssh),
        (h.join(".ssh/config"), Ssh),
        (h.join(".ssh/rc"), Ssh),
        (PathBuf::from("/etc/ssh/sshd_config"), Ssh),
        (PathBuf::from("/etc/ssh/sshd_config.d"), Ssh),
        (PathBuf::from("/etc/ssh/ssh_config.d"), Ssh),
        (PathBuf::from("/etc/hosts"), NameResolution),
        (PathBuf::from("/etc/resolv.conf"), NameResolution),
        (PathBuf::from("/etc/nsswitch.conf"), NameResolution),
        (h.join(".config/mimeapps.list"), Desktop),
        (h.join(".local/share/applications"), Desktop),
    ];
    for dir in [
        "cron.d",
        "cron.hourly",
        "cron.daily",
        "cron.weekly",
        "cron.monthly",
    ] {
        roots.push((Path::new("/etc").join(dir), Cron));
    }
    for file in [
        ".bashrc",
        ".bash_profile",
        ".bash_login",
        ".zshrc",
        ".zprofile",
        ".profile",
        ".config/environment.d",
    ] {
        roots.push((h.join(file), Shell));
    }
    for file in [
        "profile",
        "profile.d",
        "bashrc",
        "bash.bashrc",
        "zshrc",
        "zprofile",
        "environment",
        "environment.d",
    ] {
        roots.push((Path::new("/etc").join(file), Shell));
    }
    roots
}

/// `security.capability`, as hex, read without following symlinks.
fn capabilities(path: &Path) -> Option<String> {
    use std::os::unix::ffi::OsStrExt;
    let c_path = std::ffi::CString::new(path.as_os_str().as_bytes()).ok()?;
    let name = c"security.capability";
    let mut buf = [0u8; 64];
    let n = unsafe {
        libc::lgetxattr(
            c_path.as_ptr(),
            name.as_ptr(),
            buf.as_mut_ptr().cast(),
            buf.len(),
        )
    };
    (n > 0).then(|| {
        buf[..n as usize]
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    })
}

/// Your own crontab, which lives in a root-owned spool that `crontab -l` reads for you.
fn user_crontab(stop: &Cancel) -> Option<(PathBuf, Vec<u8>)> {
    if !runner::available("crontab") {
        return None;
    }
    let out = runner::run(Command::new("crontab").arg("-l"), 10, stop).ok()?;
    let user = std::env::var("USER").unwrap_or_else(|_| "you".into());
    out.ok().then(|| {
        (
            PathBuf::from(format!("crontab -l ({user})")),
            out.stdout.into_bytes(),
        )
    })
}

pub fn snapshot(store: &Store) -> Baseline {
    let mut b = Baseline {
        format: FORMAT,
        created: now(),
        entries: BTreeMap::new(),
        errors: vec![],
    };
    let decoys = crate::checks::canary::list(store).unwrap_or_default();
    let mut visited = 0;
    let mut total_bytes = 0usize;
    for (root, category) in roots() {
        if root.symlink_metadata().is_err() {
            continue;
        }
        for e in walkdir::WalkDir::new(&root)
            .follow_links(false)
            .max_depth(8)
        {
            visited += 1;
            if visited > 10000 {
                b.errors.push("Snapshot entry limit exceeded".into());
                return b;
            }
            let e = match e {
                Ok(e) => e,
                // A directory only root may list is a gap in coverage, not a failure.
                Err(e)
                    if e.io_error()
                        .is_some_and(|io| io.kind() == std::io::ErrorKind::PermissionDenied) =>
                {
                    continue
                }
                Err(e) => {
                    b.errors.push(e.to_string());
                    continue;
                }
            };
            if e.file_type().is_dir() || decoys.iter().any(|c| c.path == e.path()) {
                continue;
            }
            let Ok(meta) = e.path().symlink_metadata() else {
                continue;
            };
            let content = if e.file_type().is_symlink() {
                fs::read_link(e.path())
                    .map(|p| p.as_os_str().as_encoded_bytes().to_vec())
                    .map_err(anyhow::Error::from)
            } else {
                read_file(e.path(), MAX_INPUT)
            };
            let hash = match content {
                Ok(bytes) => {
                    total_bytes += bytes.len();
                    if total_bytes > 64 * 1024 * 1024 {
                        b.errors.push("Snapshot byte limit exceeded".into());
                        return b;
                    }
                    Some(digest(&bytes))
                }
                // Readable only by root: its metadata is still compared.
                Err(err)
                    if err
                        .downcast_ref::<std::io::Error>()
                        .is_some_and(|io| io.kind() == std::io::ErrorKind::PermissionDenied) =>
                {
                    None
                }
                Err(err) => {
                    b.errors.push(format!("{}: {err:#}", e.path().display()));
                    continue;
                }
            };
            b.entries.insert(
                e.path().into(),
                Entry {
                    hash,
                    bytes: meta.len(),
                    mode: meta.permissions().mode(),
                    uid: meta.uid(),
                    gid: meta.gid(),
                    caps: capabilities(e.path()),
                    category,
                },
            );
        }
    }
    if let Some((path, bytes)) = user_crontab(&runner::cancel()) {
        b.entries.insert(
            path,
            Entry {
                hash: Some(digest(&bytes)),
                bytes: bytes.len() as u64,
                mode: 0o600,
                uid: unsafe { libc::geteuid() },
                gid: unsafe { libc::getegid() },
                caps: None,
                category: Category::Cron,
            },
        );
    }
    b
}

/// The baseline you accepted, if it is in the current format.
fn accepted(store: &Store, r: &mut Report) -> Result<Option<Baseline>> {
    if !store.root.join("baseline.json").exists() {
        return Ok(None);
    }
    let old: Baseline = store.read("baseline.json")?;
    if old.format < FORMAT {
        r.coverage.push("An older baseline format was found. It is replaced by this inventory once you review and accept it.".into());
        return Ok(None);
    }
    Ok(Some(old))
}

pub fn changes(store: &Store, stop: &Cancel) -> Result<Report> {
    let _lock = store.lock("baseline.lock")?;
    let current = snapshot(store);
    let mut r = Report::new("Persistence changes", "Startup and system configuration");
    r.errors = current.errors.clone();
    r.sha256 = Some(digest(&serde_json::to_vec(&current.entries)?));
    r.coverage
        .checked
        .push("Startup and configuration files".into());
    r.coverage.push("Records content, mode, owner and file capabilities of startup, scheduling, login, PAM, loader, kernel-module, udev, SSH, name-resolution and desktop-handler files. Baseline means previously observed, not trusted; it does not identify the writer.".into());
    let metadata_only = current
        .entries
        .values()
        .filter(|e| e.hash.is_none())
        .count();
    if metadata_only > 0 {
        r.coverage.push(format!("{metadata_only} files are readable only by root, so their mode, owner and size are compared but not their content."));
    }
    r.coverage.push("sudoers and root's crontab need the optional helper; project workflow files are not covered yet.".into());
    let old = accepted(store, &mut r)?;
    // Only what is new or different needs attribution; the first inventory checks everything.
    let wanted: Vec<PathBuf> = current
        .entries
        .iter()
        .filter(|(p, e)| old.as_ref().is_none_or(|o| o.entries.get(*p) != Some(e)))
        .filter(|(p, _)| p.is_absolute())
        .map(|(p, _)| p.clone())
        .collect();
    let attribution = match packagedb::package_db() {
        Some(db) => match db.attribute(&wanted, stop) {
            Ok(a) => {
                r.coverage
                    .checked
                    .push(format!("Package attribution ({})", db.manager()));
                a
            }
            Err(e) => {
                r.coverage.skipped.push(crate::model::Skipped {
                    what: "Package attribution".into(),
                    why: crate::model::SkipReason::Failed,
                    detail: format!("{e:#}"),
                });
                BTreeMap::new()
            }
        },
        None => {
            r.coverage.push("No rpm, dpkg or pacman database was found, so files are not attributed to packages.".into());
            BTreeMap::new()
        }
    };
    compare(&mut r, old.as_ref(), &current, &attribution);
    Ok(r)
}

fn attribution_line(a: Option<&Attribution>) -> String {
    let Some(a) = a else {
        return "Package attribution unavailable".into();
    };
    let owner = a.package.as_deref().unwrap_or("no package");
    let config = if a.config {
        " (a configuration file)"
    } else {
        ""
    };
    match &a.state {
        FileState::Verified => format!("Owned by {owner}{config}, matches the package"),
        FileState::Modified { changes } => format!(
            "Owned by {owner}{config}, but modified: {}",
            changes.join(", ")
        ),
        FileState::Unverifiable { detail } => {
            format!("Owned by {owner}{config}; not compared with the package: {detail}")
        }
        FileState::Unowned => "Not owned by any package".into(),
    }
}

fn evidence(path: &Path, e: &Entry, a: Option<&Attribution>) -> String {
    format!(
        "{}\n{}\n{}\n{}\nSHA-256 {}",
        path.display(),
        e.category.label(),
        attribution_line(a),
        e.describe(),
        e.hash.as_deref().unwrap_or("not readable as you")
    )
}

/// Owned but no longer what the package installed: High where authentication or
/// the loader is concerned, Review elsewhere.
fn owned_modified(r: &mut Report, path: &Path, e: &Entry, a: &Attribution) -> bool {
    let FileState::Modified { changes } = &a.state else {
        return false;
    };
    if !changes
        .iter()
        .any(|c| matches!(c.as_str(), "content" | "size" | "missing" | "link target"))
    {
        return false;
    }
    let package = a.package.as_deref().unwrap_or("a package");
    if e.category.sensitive() {
        r.add(
            "persistence.sensitive_modified",
            Severity::High,
            &format!("{} differs from what {package} installed", path.display()),
            evidence(path, e, Some(a)),
            "Compare it with the package's version (rpm -V, dpkg --verify or pacman -Qkk) and restore it if you did not make the change.",
        );
    } else {
        r.add(
            "persistence.owned_modified",
            Severity::Review,
            &format!("{} differs from what {package} installed", path.display()),
            evidence(path, e, Some(a)),
            if a.config {
                "Configuration files are often edited on purpose. Check the change is one you or your administrator made."
            } else {
                "Packages do not expect this file to change. Reinstall the package if you did not change it."
            },
        );
    }
    true
}

/// New setuid or setgid bits or file capabilities on a startup or configuration file.
fn privilege(r: &mut Report, path: &Path, e: &Entry, a: Option<&Attribution>) {
    r.add(
        "persistence.privilege",
        Severity::High,
        &format!("{} gained elevated privileges", path.display()),
        evidence(path, e, a),
        "Startup and configuration files have no reason to run with raised privileges. Remove the setuid bit or capability unless you know why it is there.",
    );
}

/// The startup-file rules: what is new, changed or removed against the baseline,
/// or the initial inventory when there is none. `attribution` covers the paths
/// that are new or differ.
pub fn compare(
    r: &mut Report,
    old: Option<&Baseline>,
    current: &Baseline,
    attribution: &BTreeMap<PathBuf, Attribution>,
) {
    let Some(old) = old else {
        for (p, e) in &current.entries {
            let a = attribution.get(p);
            r.add(
                "persistence.initial",
                Severity::Info,
                &format!("Recorded: {}", e.category.label()),
                evidence(p, e, a),
                "Review before accepting this as the initial baseline.",
            );
            if let Some(a) = a {
                owned_modified(r, p, e, a);
            }
            if e.privileged() {
                privilege(r, p, e, a);
            }
        }
        r.coverage.push(
            "No baseline exists yet. Accept baseline only after reviewing this inventory.".into(),
        );
        return;
    };
    for (p, e) in &current.entries {
        let before = old.entries.get(p);
        if before == Some(e) {
            continue;
        }
        let a = attribution.get(p);
        let raised = e.privileged() && !before.is_some_and(Entry::privileged);
        if raised {
            privilege(r, p, e, a);
        }
        let content_changed = before.is_none_or(|b| b.hash != e.hash);
        if !content_changed {
            if raised {
                continue;
            }
            r.add(
                "persistence.metadata",
                Severity::Review,
                &format!("{} changed mode or owner", p.display()),
                format!(
                    "{}\nwas {}",
                    evidence(p, e, a),
                    before.map(Entry::describe).unwrap_or_default()
                ),
                "Check why its permissions or owner changed before accepting the new baseline.",
            );
            continue;
        }
        if a.is_some_and(|a| owned_modified(r, p, e, a)) {
            continue;
        }
        match a {
            Some(
                a @ Attribution {
                    package: Some(package),
                    state: FileState::Verified,
                    ..
                },
            ) => r.add(
                "persistence.package_update",
                Severity::Info,
                &format!(
                    "{} {} by {package}",
                    p.display(),
                    if before.is_some() {
                        "updated"
                    } else {
                        "installed"
                    }
                ),
                evidence(p, e, Some(a)),
                "Nothing to do: the package database accounts for this change.",
            ),
            _ => r.add(
                "persistence.change",
                Severity::Review,
                &if before.is_some() {
                    format!("{} changed", p.display())
                } else {
                    format!("New: {}", p.display())
                },
                evidence(p, e, a),
                "Review the file and the package that owns it before accepting a new baseline.",
            ),
        }
    }
    // An unreadable entry is not evidence that it was removed.
    if current.errors.is_empty() {
        for p in old
            .entries
            .keys()
            .filter(|p| !current.entries.contains_key(*p))
        {
            r.add(
                "persistence.removed",
                Severity::Review,
                &format!("{} was removed", p.display()),
                p.display(),
                "Check whether an intentional configuration or package change explains this.",
            );
        }
    }
}
/// Puts back the baseline that the last acceptance replaced.
pub fn restore_previous_baseline(store: &Store) -> Result<()> {
    let _lock = store.lock("baseline.lock")?;
    let previous: Baseline = store
        .read("baseline-previous.json")
        .context("There is no earlier baseline to restore")?;
    store.write("baseline.json", &previous)?;
    std::fs::remove_file(store.root.join("baseline-previous.json"))?;
    Ok(())
}
pub fn has_previous_baseline(store: &Store) -> bool {
    store.root.join("baseline-previous.json").exists()
}
pub fn accept_reviewed_baseline(store: &Store, id: &str) -> Result<Report> {
    let _lock = store.lock("baseline.lock")?;
    let original = store.report(id)?;
    if original.kind != "Persistence changes" || !original.errors.is_empty() {
        bail!("Choose a complete startup inventory report first");
    }
    let current = snapshot(store);
    if !current.errors.is_empty()
        || original.sha256 != Some(digest(&serde_json::to_vec(&current.entries)?))
    {
        bail!("Startup inventory changed or became unreadable after review; compare it again before accepting");
    }
    // Keep the baseline being replaced so the acceptance can be undone.
    if store.root.join("baseline.json").exists() {
        let previous: Baseline = store.read("baseline.json")?;
        store.write("baseline-previous.json", &previous)?;
    }
    store.write("baseline.json", &current)?;
    let mut r = Report::new("Baseline accepted", "Reviewed startup inventory");
    r.sha256 = original.sha256;
    r.coverage.push(format!("Accepted exactly the inventory from report {id}. Baseline is prior state, not proof of trust."));
    store.save(&r)?;
    Ok(r)
}

#[cfg(test)]
mod live {
    use super::*;

    /// Times the real snapshot and attribution; run with `--ignored`.
    #[test]
    #[ignore]
    fn times_a_real_comparison() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(Some(dir.path().join("s"))).unwrap();
        let t = std::time::Instant::now();
        let b = snapshot(&store);
        eprintln!("snapshot {} entries in {:?}", b.entries.len(), t.elapsed());
        let paths: Vec<PathBuf> = b
            .entries
            .keys()
            .filter(|p| p.is_absolute())
            .cloned()
            .collect();
        let t = std::time::Instant::now();
        let a = packagedb::package_db()
            .unwrap()
            .attribute(&paths, &runner::cancel())
            .unwrap();
        eprintln!("attribution of {} in {:?}", a.len(), t.elapsed());
    }
}
