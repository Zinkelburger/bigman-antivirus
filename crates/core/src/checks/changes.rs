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

/// The format snapshots are written in. Format 3 added your own bin folders,
/// format 4 developer tool folders and editor extensions.
pub const FORMAT: u32 = 4;
/// Baselines written before this format are treated as absent (plan Part K, decision 2).
const OLDEST_FORMAT: u32 = 2;
/// System folders whose commands a file in your own bin folder can hide.
const SYSTEM_BIN: [&str; 4] = ["/usr/bin", "/usr/sbin", "/bin", "/sbin"];
/// Files up to this size are hashed; larger tool binaries and extension bundles
/// are identified by size, inode and change time instead (see `fingerprint`).
const HASHED: u64 = 256 * 1024;
/// Where editors built on VS Code keep installed extensions, under your home.
const EDITOR_EXTENSIONS: [&str; 7] = [
    ".vscode/extensions",
    ".vscode-insiders/extensions",
    ".vscode-oss/extensions",
    ".cursor/extensions",
    ".windsurf/extensions",
    ".var/app/com.visualstudio.code/data/vscode/extensions",
    ".var/app/com.vscodium.codium/data/codium/extensions",
];

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
    /// Programs in your own bin folders, which your shell finds before the system's.
    Commands,
    /// Programs in folders language toolchains and version managers install into
    /// (npm's global prefix, nvm, pnpm, Cargo, Go and similar).
    Tools,
    /// Editor extensions, one entry per extension: its version and its code.
    Extensions,
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
            Category::Commands => "command in your own bin folder",
            Category::Tools => "program in a developer tool folder",
            Category::Extensions => "editor extension",
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
    /// For a command in your own bin folder, the system command of the same name it hides.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shadows: Option<PathBuf>,
    /// For an editor extension, the version its manifest declares.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl Entry {
    /// The same file as `other`. What it shadows depends on what the system has
    /// installed since, not on the file.
    fn same(&self, other: &Entry) -> bool {
        Entry {
            shadows: None,
            ..self.clone()
        } == Entry {
            shadows: None,
            ..other.clone()
        }
    }
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
        (h.join(".local/bin"), Commands),
        (h.join("bin"), Commands),
    ];
    roots.extend(tool_roots().into_iter().map(|d| (d, Tools)));
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

/// Folders language toolchains and version managers put programs in. Shells often
/// find them before the system's folders, so a program swapped here runs in place
/// of the one you meant. Only folders under your home are taken: a global npm
/// prefix of /usr is the system's own.
fn tool_roots() -> Vec<PathBuf> {
    let h = home();
    let mut dirs = vec![];
    let npmrc = fs::read_to_string(h.join(".npmrc")).unwrap_or_default();
    let prefix = std::env::var("NPM_CONFIG_PREFIX")
        .or_else(|_| std::env::var("npm_config_prefix"))
        .ok()
        .or_else(|| {
            npmrc.lines().find_map(|l| {
                let (key, value) = l.split_once('=')?;
                (key.trim() == "prefix").then(|| value.trim().to_string())
            })
        });
    if let Some(p) = prefix {
        let p = p
            .strip_prefix("~/")
            .or_else(|| p.strip_prefix("${HOME}/"))
            .or_else(|| p.strip_prefix("$HOME/"))
            .map(|rest| h.join(rest))
            .unwrap_or_else(|| PathBuf::from(&p));
        dirs.push(p.join("bin"));
    }
    for d in [
        ".npm-global/bin",
        ".npm-packages/bin",
        ".volta/bin",
        ".bun/bin",
        ".yarn/bin",
        ".deno/bin",
        ".cargo/bin",
        "go/bin",
        ".local/share/pnpm",
    ] {
        dirs.push(h.join(d));
    }
    for var in ["PNPM_HOME", "GOPATH"] {
        if let Some(v) = std::env::var_os(var) {
            for d in std::env::split_paths(&v) {
                dirs.push(if var == "GOPATH" { d.join("bin") } else { d });
            }
        }
    }
    for (parent, tail) in [
        (".nvm/versions/node", "bin"),
        (".local/share/fnm/node-versions", "installation/bin"),
    ] {
        if let Ok(versions) = fs::read_dir(h.join(parent)) {
            dirs.extend(versions.flatten().map(|v| v.path().join(tail)));
        }
    }
    // ~/.local/bin and ~/bin are already watched as your own bin folders.
    let own = [h.join(".local/bin"), h.join("bin")];
    dirs.retain(|d| d.starts_with(&h) && !own.contains(d));
    dirs.sort();
    dirs.dedup();
    dirs
}

/// A file's content when it is at most 256 KiB. A larger one (node, rustc, an
/// extension bundle) is identified by its size, inode and change time: replacing
/// or rewriting it changes the change time, which you cannot set back, and
/// hashing hundreds of megabytes every five minutes would cost more than it adds.
fn fingerprint(path: &Path) -> Result<Vec<u8>> {
    let meta = fs::metadata(path)?;
    if !meta.is_file() {
        bail!("{} is not a regular file", path.display());
    }
    if meta.len() <= HASHED {
        return read_file(path, HASHED);
    }
    Ok(format!(
        "size {} inode {}:{} changed {}.{}",
        meta.len(),
        meta.dev(),
        meta.ino(),
        meta.ctime(),
        meta.ctime_nsec()
    )
    .into_bytes())
}

/// One extension of an editor: where it is, its ID and version.
struct Installed {
    dir: PathBuf,
    id: String,
    version: String,
}

/// The extensions an editor has installed: from its `extensions.json` when it
/// keeps one, otherwise each folder's manifest. Folders the editor has marked
/// obsolete (an older version awaiting cleanup) are left out.
fn installed_extensions(root: &Path) -> Vec<Installed> {
    let read_json = |p: &Path| -> Option<serde_json::Value> {
        serde_json::from_slice(&read_file(p, HASHED).ok()?).ok()
    };
    if let Some(serde_json::Value::Array(list)) = read_json(&root.join("extensions.json")) {
        return list
            .iter()
            .filter_map(|x| {
                let id = x["identifier"]["id"].as_str()?.to_ascii_lowercase();
                let version = x["version"].as_str().unwrap_or_default().to_string();
                let rel = x["relativeLocation"]
                    .as_str()
                    .map(|r| root.join(r))
                    .or_else(|| x["location"]["path"].as_str().map(PathBuf::from))?;
                Some(Installed {
                    dir: rel,
                    id,
                    version,
                })
            })
            .collect();
    }
    let obsolete = read_json(&root.join(".obsolete")).unwrap_or_default();
    let Ok(dirs) = fs::read_dir(root) else {
        return vec![];
    };
    dirs.flatten()
        .filter(|d| d.path().is_dir())
        .filter(|d| {
            let name = d.file_name().to_string_lossy().into_owned();
            !name.starts_with('.') && obsolete.get(&name).is_none()
        })
        .filter_map(|d| {
            let manifest = read_json(&d.path().join("package.json"))?;
            Some(Installed {
                id: format!(
                    "{}.{}",
                    manifest["publisher"].as_str()?,
                    manifest["name"].as_str()?
                )
                .to_ascii_lowercase(),
                version: manifest["version"].as_str().unwrap_or_default().into(),
                dir: d.path(),
            })
        })
        .collect()
}

/// An extension's entry: keyed by its ID, so an update that moves it to a new
/// folder compares with the version before. The hash covers the code the editor
/// loads (`main`), not the manifest, which the editor rewrites with metadata.
fn extension_entry(e: &Installed) -> Option<(Entry, u64)> {
    let manifest: serde_json::Value =
        serde_json::from_slice(&read_file(&e.dir.join("package.json"), HASHED).ok()?).ok()?;
    let main = manifest["main"].as_str().map(|m| e.dir.join(m));
    let code = main.as_ref().and_then(|m| {
        [m.clone(), m.with_extension("js"), m.join("index.js")]
            .into_iter()
            .find(|c| c.is_file())
    });
    let meta = code
        .as_ref()
        .and_then(|c| fs::metadata(c).ok())
        .or_else(|| fs::metadata(&e.dir).ok())?;
    let bytes = match &code {
        Some(c) => fingerprint(c).ok()?,
        None => b"no code".to_vec(),
    };
    let read = bytes.len() as u64;
    Some((
        Entry {
            hash: Some(digest(&bytes)),
            bytes: meta.len(),
            mode: meta.permissions().mode(),
            uid: meta.uid(),
            gid: meta.gid(),
            caps: None,
            category: Category::Extensions,
            shadows: None,
            version: Some(if e.version.is_empty() {
                manifest["version"].as_str().unwrap_or_default().into()
            } else {
                e.version.clone()
            }),
        },
        read,
    ))
}

/// The system command a file in your own bin folder hides, unless it is only a
/// link to that same command.
fn shadowed(path: &Path) -> Option<PathBuf> {
    let name = path.file_name()?;
    let system = SYSTEM_BIN
        .iter()
        .map(|d| Path::new(d).join(name))
        .find(|p| p.symlink_metadata().is_ok())?;
    let same = matches!(
        (fs::canonicalize(path), fs::canonicalize(&system)),
        (Ok(a), Ok(b)) if a == b
    );
    (!same).then_some(system)
}

/// The first `n` bytes of a regular file, without following a final link.
fn read_head(path: &Path, n: u64) -> Result<Vec<u8>> {
    use std::{io::Read, os::unix::fs::OpenOptionsExt};
    let file = fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)?;
    if !file.metadata()?.is_file() {
        bail!("{} is not a regular file", path.display());
    }
    let mut out = vec![];
    file.take(n).read_to_end(&mut out)?;
    Ok(out)
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
        // Tool folders are flat; pnpm's keeps its whole store below its programs.
        let depth = if category == Category::Tools { 1 } else { 8 };
        for e in walkdir::WalkDir::new(&root)
            .follow_links(false)
            .max_depth(depth)
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
            let content = if category == Category::Tools {
                // Global npm packages and version managers link their programs to
                // files elsewhere: the link and what it points to are both recorded.
                if e.file_type().is_symlink() {
                    fs::read_link(e.path())
                        .map_err(anyhow::Error::from)
                        .map(|target| {
                            let mut b = target.as_os_str().as_encoded_bytes().to_vec();
                            b.push(0);
                            b.extend(
                                fingerprint(e.path()).unwrap_or_else(|_| b"unreadable".to_vec()),
                            );
                            b
                        })
                } else {
                    fingerprint(e.path())
                }
            } else if e.file_type().is_symlink() {
                fs::read_link(e.path())
                    .map(|p| p.as_os_str().as_encoded_bytes().to_vec())
                    .map_err(anyhow::Error::from)
            } else if category == Category::Commands && meta.len() > MAX_INPUT {
                // A large program: its first 8 MiB and its size stand in for its content.
                read_head(e.path(), MAX_INPUT).map(|mut b| {
                    b.extend_from_slice(format!("\0size {}", meta.len()).as_bytes());
                    b
                })
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
                    shadows: (category == Category::Commands)
                        .then(|| shadowed(e.path()))
                        .flatten(),
                    version: None,
                },
            );
        }
    }
    let h = home();
    for root in EDITOR_EXTENSIONS.map(|d| h.join(d)) {
        for x in installed_extensions(&root) {
            visited += 1;
            if visited > 10000 {
                b.errors.push("Snapshot entry limit exceeded".into());
                return b;
            }
            if let Some((entry, read)) = extension_entry(&x) {
                total_bytes += read as usize;
                if total_bytes > 64 * 1024 * 1024 {
                    b.errors.push("Snapshot byte limit exceeded".into());
                    return b;
                }
                b.entries.insert(root.join(&x.id), entry);
            }
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
                shadows: None,
                version: None,
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
    if old.format < OLDEST_FORMAT {
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
        "{}\n{}{}\n{}\n{}\nSHA-256 {}",
        path.display(),
        e.category.label(),
        e.shadows
            .as_ref()
            .map(|s| format!(", run instead of {}", s.display()))
            .or_else(|| e.version.as_ref().map(|v| format!(", version {v}")))
            .unwrap_or_default(),
        attribution_line(a),
        e.describe(),
        e.hash.as_deref().unwrap_or("not readable as you")
    )
}

/// A command in your own bin folder that hides a system command: High when it is
/// new or changed, Review in an inventory being taken for the first time.
fn shadowing(
    r: &mut Report,
    path: &Path,
    e: &Entry,
    a: Option<&Attribution>,
    first_seen: bool,
) -> bool {
    let Some(system) = &e.shadows else {
        return false;
    };
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy())
        .unwrap_or_default();
    r.add(
        "persistence.shadowed-command",
        if first_seen { Severity::Review } else { Severity::High },
        &format!(
            "{} {} {}",
            path.display(),
            if first_seen { "runs instead of" } else { "now runs instead of" },
            system.display()
        ),
        evidence(path, e, a),
        &format!("Every `{name}` you type now runs this file. Read it; if you did not put it there, delete it and check what you ran just before it appeared."),
    );
    true
}

/// An editor extension that is new, updated, or whose code changed while its
/// version stayed the same, which editors never do.
fn extension_change(r: &mut Report, path: &Path, e: &Entry, before: Option<&Entry>) {
    let id = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let version = e.version.as_deref().unwrap_or("unknown version");
    match before {
        None => r.add(
            "persistence.extension-new",
            Severity::Review,
            &format!("New editor extension: {id} {version}"),
            evidence(path, e, None),
            "It runs with your permissions whenever the editor starts. If you did not install it, remove it from the editor and check what ran just before it appeared.",
        ),
        Some(b) if b.version != e.version => r.add(
            "persistence.extension-updated",
            Severity::Info,
            &format!(
                "{id} updated from {} to {version}",
                b.version.as_deref().unwrap_or("an unknown version")
            ),
            evidence(path, e, None),
            "Nothing to do if the editor updates its extensions.",
        ),
        Some(b) if b.hash != e.hash => r.add(
            "persistence.extension-modified",
            Severity::High,
            &format!("{id} {version}: its code changed without an update"),
            evidence(path, e, None),
            "Editors do not change an installed extension's code. Quit the editor, reinstall the extension from its marketplace, and check what ran just before the change.",
        ),
        Some(_) => {}
    }
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
            shadowing(r, p, e, a, true);
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
    // A baseline from before your bin folders were recorded has nothing to compare them with.
    let unrecorded = |c: Category| {
        (old.format < 3 && c == Category::Commands)
            || (old.format < 4 && matches!(c, Category::Tools | Category::Extensions))
    };
    for (p, e) in &current.entries {
        let before = old.entries.get(p);
        if before.is_some_and(|b| b.same(e)) {
            continue;
        }
        if unrecorded(e.category) {
            r.add(
                "persistence.initial",
                Severity::Info,
                &format!("Recorded: {}", e.category.label()),
                evidence(p, e, attribution.get(p)),
                "Newly watched. Review before accepting the baseline again.",
            );
            shadowing(r, p, e, attribution.get(p), true);
            continue;
        }
        if e.category == Category::Extensions {
            extension_change(r, p, e, before);
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
        if shadowing(r, p, e, a, false) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extensions_are_keyed_by_id_and_skip_obsolete_folders() {
        let d = tempfile::tempdir().unwrap();
        let root = d.path();
        for (dir, version, main) in [
            ("example.formatter-2.3.0", "2.3.0", "./dist/extension"),
            ("example.formatter-2.2.0", "2.2.0", "./dist/extension"),
            ("example.theme-1.0.0", "1.0.0", ""),
        ] {
            let ext = root.join(dir);
            fs::create_dir_all(ext.join("dist")).unwrap();
            let (publisher, name) = dir.split_once('.').unwrap();
            let name = name.rsplit_once('-').unwrap().0;
            let main = if main.is_empty() {
                String::new()
            } else {
                format!(r#","main":"{main}""#)
            };
            fs::write(
                ext.join("package.json"),
                format!(
                    r#"{{"publisher":"{publisher}","name":"{name}","version":"{version}"{main}}}"#
                ),
            )
            .unwrap();
            fs::write(ext.join("dist/extension.js"), format!("// {version}")).unwrap();
        }
        fs::write(
            root.join(".obsolete"),
            r#"{"example.formatter-2.2.0":true}"#,
        )
        .unwrap();
        let mut found: Vec<(String, String)> = installed_extensions(root)
            .into_iter()
            .map(|x| (x.id, x.version))
            .collect();
        found.sort();
        assert_eq!(
            found,
            [
                ("example.formatter".to_string(), "2.3.0".to_string()),
                ("example.theme".to_string(), "1.0.0".to_string())
            ]
        );
        let formatter = installed_extensions(root)
            .into_iter()
            .find(|x| x.id == "example.formatter")
            .unwrap();
        let (before, _) = extension_entry(&formatter).unwrap();
        fs::write(
            root.join("example.formatter-2.3.0/dist/extension.js"),
            "// changed",
        )
        .unwrap();
        let (after, _) = extension_entry(&formatter).unwrap();
        assert_ne!(
            before.hash, after.hash,
            "the main file's content is what is compared"
        );
        assert_eq!(after.version.as_deref(), Some("2.3.0"));
        // The editor's list wins when there is one.
        fs::write(
            root.join("extensions.json"),
            r#"[{"identifier":{"id":"Example.Formatter"},"version":"2.3.0","relativeLocation":"example.formatter-2.3.0"}]"#,
        )
        .unwrap();
        let listed: Vec<String> = installed_extensions(root)
            .into_iter()
            .map(|x| x.id)
            .collect();
        assert_eq!(listed, ["example.formatter"]);
    }

    #[test]
    fn large_files_are_fingerprinted_by_identity_not_read() {
        let d = tempfile::tempdir().unwrap();
        let small = d.path().join("small");
        let large = d.path().join("large");
        fs::write(&small, "tool").unwrap();
        fs::write(&large, vec![0u8; HASHED as usize + 1]).unwrap();
        assert_eq!(fingerprint(&small).unwrap(), b"tool");
        let before = fingerprint(&large).unwrap();
        assert!(String::from_utf8_lossy(&before).starts_with("size "));
        // Replacing the file, even with the same bytes, gives it a new identity.
        let copy = d.path().join("copy");
        fs::copy(&large, &copy).unwrap();
        fs::rename(&copy, &large).unwrap();
        assert_ne!(fingerprint(&large).unwrap(), before);
    }
}
