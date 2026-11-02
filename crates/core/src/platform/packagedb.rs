//! Package attribution: which installed package owns a file, and whether the file
//! still matches what the package installed. Read-only queries of rpm, dpkg and
//! pacman, run as you. Parsers are separate from the commands so fixtures test them.
use super::{Attribution, FileState, PackageDb, PackageScript};
use crate::runner::{self, Cancel};
use anyhow::{bail, Result};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    process::Command,
};

/// Arguments per command, well under ARG_MAX.
const CHUNK: usize = 200;
const TIMEOUT: u64 = 120;

fn run(program: &str, args: &[&str], paths: &[String], stop: &Cancel) -> Result<runner::Output> {
    let out = runner::run(Command::new(program).args(args).args(paths), TIMEOUT, stop)?;
    if out.interrupted || out.truncated {
        bail!("{program} did not finish: {}", out.stderr.trim());
    }
    Ok(out)
}

fn text(paths: &[PathBuf]) -> Vec<String> {
    paths
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect()
}

/// rpm and `dpkg --verify` share a nine-column flag field: S M 5 D L U G T P.
/// A `?` means the test could not run (usually an unreadable file).
pub fn verify_flags(flags: &str) -> (Vec<String>, bool) {
    const NAMES: [&str; 9] = [
        "size",
        "mode",
        "content",
        "device",
        "link target",
        "owner",
        "group",
        "mtime",
        "capabilities",
    ];
    let mut changes = vec![];
    let mut unknown = false;
    for (c, name) in flags.chars().zip(NAMES) {
        match c {
            '.' => {}
            '?' => unknown = true,
            // A new modification time alone is not a change to what the file says.
            _ if name == "mtime" => {}
            _ => changes.push(name.to_string()),
        }
    }
    (changes, unknown)
}

/// One line of `rpm -V` or `dpkg --verify`: `S.5....T.  c /etc/pam.d/sudo`,
/// or `missing   c /etc/foo (Permission denied)`.
pub fn parse_verify_line(line: &str) -> Option<(PathBuf, bool, FileState)> {
    let (flags, rest) = line.split_once(' ')?;
    let rest = rest.trim_start();
    let (config, rest) = match rest.split_once(' ') {
        Some((marker, path)) if marker.len() == 1 && "cdglr".contains(marker) => {
            (marker == "c", path.trim_start())
        }
        _ => (false, rest),
    };
    if flags == "missing" {
        let (path, reason) = match rest.split_once(" (") {
            Some((path, reason)) => (path, reason.trim_end_matches(')')),
            None => (rest, ""),
        };
        let state = if reason.is_empty() {
            FileState::Modified {
                changes: vec!["missing".into()],
            }
        } else {
            FileState::Unverifiable {
                detail: reason.into(),
            }
        };
        return Some((PathBuf::from(path), config, state));
    }
    if flags.len() != 9 || !rest.starts_with('/') {
        return None;
    }
    let (changes, unknown) = verify_flags(flags);
    let state = if !changes.is_empty() {
        FileState::Modified { changes }
    } else if unknown {
        FileState::Unverifiable {
            detail: "Could not be read to compare with the package".into(),
        }
    } else {
        FileState::Verified
    };
    Some((PathBuf::from(rest), config, state))
}

/// Applies verify output to attributions that start as `Verified`.
fn apply_verify(out: &str, attributions: &mut BTreeMap<PathBuf, Attribution>) {
    for (path, config, state) in out.lines().filter_map(parse_verify_line) {
        if let Some(a) = attributions.get_mut(&path) {
            a.config |= config;
            a.state = state;
        }
    }
}

fn mark_config<'a>(
    paths: impl Iterator<Item = &'a str>,
    attributions: &mut BTreeMap<PathBuf, Attribution>,
) {
    for path in paths {
        if let Some(a) = attributions.get_mut(Path::new(path.trim())) {
            a.config = true;
        }
    }
}

fn owned(package: String) -> Attribution {
    Attribution {
        package: Some(package),
        config: false,
        state: FileState::Verified,
    }
}

fn unowned() -> Attribution {
    Attribution {
        package: None,
        config: false,
        state: FileState::Unowned,
    }
}

/// `rpm -q --scripts` output: headers like `postinstall scriptlet (using /bin/sh):`
/// or `preinstall program: /bin/sh`, each followed by the script text.
pub fn parse_rpm_scripts(package: &str, out: &str) -> Vec<PackageScript> {
    let header = regex::Regex::new(r"^([a-z]+) (?:scriptlet \(using ([^)]+)\):|program: (.+))$")
        .expect("valid regex");
    let mut scripts: Vec<PackageScript> = vec![];
    for line in out.lines() {
        if let Some(c) = header.captures(line) {
            scripts.push(PackageScript {
                package: package.into(),
                phase: c[1].into(),
                interpreter: c.get(2).or(c.get(3)).map_or("", |m| m.as_str()).into(),
                text: String::new(),
            });
        } else if let Some(current) = scripts.last_mut() {
            current.text.push_str(line);
            current.text.push('\n');
        }
    }
    for s in &mut scripts {
        s.text = s.text.trim_end().to_string();
    }
    // A "program:" header with no body runs the interpreter alone; nothing to read.
    scripts.retain(|s| !s.text.is_empty());
    scripts
}

/// Owners of each path in one `rpm -qf` call. rpm prints one line per owner
/// (several for a file shared by multilib packages) and names nothing, so an
/// existing file no package owns goes after each path: its "file … is not owned"
/// line marks where that path's answer ends.
fn rpm_owners(paths: &[PathBuf], stop: &Cancel) -> Result<Vec<(PathBuf, Option<String>)>> {
    let marker = tempfile::NamedTempFile::new()?;
    let marker_line = format!(
        "file {} is not owned by any package",
        marker.path().display()
    );
    let mut args = vec![];
    for p in paths {
        args.push(p.to_string_lossy().into_owned());
        args.push(marker.path().to_string_lossy().into_owned());
    }
    let out = run("rpm", &["-qf", "--qf", "%{NAME}-%{EVR}\\n"], &args, stop)?;
    let answers = parse_rpm_owner_runs(&out.stdout, &marker_line);
    if answers.len() != paths.len() {
        bail!(
            "rpm answered {} of {} ownership queries",
            answers.len(),
            paths.len()
        );
    }
    Ok(paths.iter().cloned().zip(answers).collect())
}

/// Splits `rpm -qf` output at each marker line; the first owner of each run is kept.
pub fn parse_rpm_owner_runs(out: &str, marker: &str) -> Vec<Option<String>> {
    let mut answers = vec![];
    let mut current: Option<String> = None;
    for line in out.lines() {
        if line == marker {
            answers.push(current.take());
        } else if current.is_none() && !line.starts_with("file ") && !line.is_empty() {
            current = Some(line.to_string());
        }
    }
    answers
}

/// Packages per file-list query: large packages list thousands of files, and the
/// runner keeps at most 2 MiB of output.
const FILE_CHUNK: usize = 32;

/// The database's own digests, modes and owners for `wanted` paths: a read, unlike
/// `rpm -V`, which re-hashes every file of every package. A batch whose listing
/// outgrows the runner's output limit is split in half.
fn rpm_files(
    names: &[String],
    wanted: &BTreeSet<PathBuf>,
    stop: &Cancel,
) -> Result<BTreeMap<PathBuf, RpmFile>> {
    let out = runner::run(
        Command::new("rpm")
            .args(["-q", "--qf", RPM_FILES])
            .args(names),
        TIMEOUT,
        stop,
    )?;
    if out.truncated && !out.interrupted && names.len() > 1 {
        let (left, right) = names.split_at(names.len() / 2);
        let mut files = rpm_files(left, wanted, stop)?;
        files.extend(rpm_files(right, wanted, stop)?);
        return Ok(files);
    }
    if !out.ok() {
        bail!("rpm could not list package files: {}", out.stderr.trim());
    }
    Ok(parse_rpm_files(&out.stdout)
        .into_iter()
        .filter(|(p, _)| wanted.contains(p))
        .collect())
}

/// rpm's PGPHASHALGO number for SHA-256.
const SHA256: u32 = 8;

/// One line per file of each queried package; scalar tags repeat on every line.
const RPM_FILES: &str = "[%{=NAME}-%{=EVR}\\t%{=FILEDIGESTALGO}\\t%{FILENAMES}\\t%{FILEDIGESTS}\\t%{FILEMODES:octal}\\t%{FILEUSERNAME}\\t%{FILEGROUPNAME}\\t%{FILELINKTOS}\\t%{FILEFLAGS:fflags}\\n]";

/// What the rpm database says a file should be.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpmFile {
    pub package: String,
    pub algo: u32,
    pub digest: String,
    pub mode: u32,
    pub user: String,
    pub group: String,
    pub link: String,
    pub config: bool,
    /// Created at runtime, so there is no content to compare.
    pub ghost: bool,
}

pub fn parse_rpm_files(out: &str) -> BTreeMap<PathBuf, RpmFile> {
    out.lines()
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            let [package, algo, path, digest, mode, user, group, link, flags] = f.as_slice() else {
                return None;
            };
            Some((
                PathBuf::from(path),
                RpmFile {
                    package: (*package).into(),
                    algo: algo.parse().unwrap_or(0),
                    digest: (*digest).into(),
                    mode: u32::from_str_radix(mode, 8).unwrap_or(0),
                    user: (*user).into(),
                    group: (*group).into(),
                    link: (*link).into(),
                    config: flags.contains('c'),
                    ghost: flags.contains('g'),
                },
            ))
        })
        .collect()
}

/// The file as it is on disk now.
pub struct Actual {
    pub exists: bool,
    pub mode: u32,
    pub user: String,
    pub group: String,
    /// SHA-256 of the content; `Err` when it cannot be read as you.
    pub digest: std::result::Result<String, String>,
    pub link: Option<String>,
}

fn account_name(id: u32, group: bool) -> String {
    let name = unsafe {
        if group {
            let g = libc::getgrgid(id);
            (!g.is_null()).then(|| {
                std::ffi::CStr::from_ptr((*g).gr_name)
                    .to_string_lossy()
                    .into_owned()
            })
        } else {
            let u = libc::getpwuid(id);
            (!u.is_null()).then(|| {
                std::ffi::CStr::from_ptr((*u).pw_name)
                    .to_string_lossy()
                    .into_owned()
            })
        }
    };
    name.unwrap_or_else(|| id.to_string())
}

impl Actual {
    pub fn read(path: &Path) -> Actual {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let Ok(meta) = path.symlink_metadata() else {
            return Actual {
                exists: false,
                mode: 0,
                user: String::new(),
                group: String::new(),
                digest: Err(String::new()),
                link: None,
            };
        };
        let link = meta.file_type().is_symlink().then(|| {
            std::fs::read_link(path)
                .map(|t| t.to_string_lossy().into_owned())
                .unwrap_or_default()
        });
        let digest = if link.is_some() || meta.is_dir() {
            Ok(String::new())
        } else {
            crate::store::read_file(path, 64 * 1024 * 1024)
                .map(|b| crate::store::digest(&b))
                .map_err(|e| format!("{e:#}"))
        };
        Actual {
            exists: true,
            mode: meta.permissions().mode(),
            user: account_name(meta.uid(), false),
            group: account_name(meta.gid(), true),
            digest,
            link,
        }
    }
}

/// Compares a file with its rpm record the way `rpm -V` does, minus mtime.
pub fn rpm_state(actual: &Actual, expected: &RpmFile) -> FileState {
    if !actual.exists {
        return if expected.ghost {
            FileState::Verified
        } else {
            FileState::Modified {
                changes: vec!["missing".into()],
            }
        };
    }
    let mut changes = vec![];
    if actual.mode & 0o7777 != expected.mode & 0o7777
        || actual.mode & 0o170000 != expected.mode & 0o170000
    {
        changes.push("mode".to_string());
    }
    if actual.user != expected.user {
        changes.push("owner".into());
    }
    if actual.group != expected.group {
        changes.push("group".into());
    }
    match (&actual.link, expected.link.is_empty()) {
        (Some(target), false) if *target != expected.link => changes.push("link target".into()),
        _ => {}
    }
    let unreadable = match &actual.digest {
        Ok(d) if !expected.ghost && !expected.digest.is_empty() && *d != expected.digest => {
            changes.push("content".into());
            None
        }
        Err(e) if !expected.ghost && !expected.digest.is_empty() => Some(e.clone()),
        _ => None,
    };
    if !changes.is_empty() {
        return FileState::Modified { changes };
    }
    match unreadable {
        Some(detail) => FileState::Unverifiable { detail },
        None => FileState::Verified,
    }
}

#[derive(Default)]
pub struct Rpm;

impl PackageDb for Rpm {
    fn manager(&self) -> &'static str {
        "rpm"
    }

    fn attribute(
        &self,
        paths: &[PathBuf],
        stop: &Cancel,
    ) -> Result<BTreeMap<PathBuf, Attribution>> {
        let mut result = BTreeMap::new();
        let mut packages = BTreeSet::new();
        for chunk in paths.chunks(CHUNK) {
            for (path, package) in rpm_owners(chunk, stop)? {
                if let Some(package) = &package {
                    packages.insert(package.clone());
                }
                result.insert(path, package.map_or_else(unowned, owned));
            }
        }
        let names: Vec<String> = packages.into_iter().collect();
        let wanted: BTreeSet<PathBuf> = result.keys().cloned().collect();
        let mut files = BTreeMap::new();
        for chunk in names.chunks(FILE_CHUNK) {
            files.extend(rpm_files(chunk, &wanted, stop)?);
        }
        let mut legacy = BTreeSet::new();
        for (path, a) in result.iter_mut().filter(|(_, a)| a.package.is_some()) {
            // Never assume a match: an owned file the database does not list is unverified.
            let Some(expected) = files.get(path) else {
                a.state = FileState::Unverifiable {
                    detail: "Not listed in the package database".into(),
                };
                continue;
            };
            a.config = expected.config;
            if expected.algo != SHA256 {
                legacy.insert(expected.package.clone());
                continue;
            }
            a.state = rpm_state(&Actual::read(path), expected);
        }
        // Packages hashed with something other than SHA-256 are checked by rpm itself.
        let legacy: Vec<String> = legacy.into_iter().collect();
        for chunk in legacy.chunks(CHUNK) {
            let out = run("rpm", &["-V", "--nodeps", "--noscripts"], chunk, stop)?;
            apply_verify(&out.stdout, &mut result);
        }
        Ok(result)
    }

    fn files(&self, package: &str, stop: &Cancel) -> Result<Vec<PathBuf>> {
        let out = run("rpm", &["-ql"], &[package.into()], stop)?;
        if out.code != Some(0) {
            bail!("rpm could not list {package}: {}", out.stdout.trim());
        }
        Ok(out
            .stdout
            .lines()
            .filter(|l| l.starts_with('/'))
            .map(PathBuf::from)
            .collect())
    }

    fn scripts(&self, package: &str, stop: &Cancel) -> Result<Vec<PackageScript>> {
        let out = run("rpm", &["-q", "--scripts"], &[package.into()], stop)?;
        if out.code != Some(0) {
            bail!(
                "rpm could not read the scripts of {package}: {}",
                out.stdout.trim()
            );
        }
        Ok(parse_rpm_scripts(package, &out.stdout))
    }
}

/// `dpkg -S` output: `sudo: /etc/pam.d/sudo` (several packages are comma-separated).
pub fn parse_dpkg_search(out: &str) -> BTreeMap<PathBuf, String> {
    out.lines()
        .filter_map(|l| l.split_once(": "))
        .filter(|(_, path)| path.starts_with('/'))
        .map(|(packages, path)| {
            let first = packages.split(", ").next().unwrap_or(packages);
            (
                PathBuf::from(path),
                first.split(':').next().unwrap_or(first).to_string(),
            )
        })
        .collect()
}

#[derive(Default)]
pub struct Dpkg;

impl Dpkg {
    fn info(package: &str) -> PathBuf {
        Path::new("/var/lib/dpkg/info").join(package)
    }
}

impl PackageDb for Dpkg {
    fn manager(&self) -> &'static str {
        "dpkg"
    }

    fn attribute(
        &self,
        paths: &[PathBuf],
        stop: &Cancel,
    ) -> Result<BTreeMap<PathBuf, Attribution>> {
        let mut result: BTreeMap<PathBuf, Attribution> =
            paths.iter().map(|p| (p.clone(), unowned())).collect();
        let mut packages = BTreeSet::new();
        for chunk in paths.chunks(CHUNK) {
            // dpkg -S exits 1 when some paths are unowned; the found ones are still printed.
            let out = run("dpkg", &["-S"], &text(chunk), stop)?;
            for (path, package) in parse_dpkg_search(&out.stdout) {
                if let Some(a) = result.get_mut(&path) {
                    packages.insert(package.clone());
                    *a = owned(package);
                }
            }
        }
        let names: Vec<String> = packages.into_iter().collect();
        for chunk in names.chunks(CHUNK) {
            let out = run("dpkg", &["--verify"], chunk, stop)?;
            apply_verify(&out.stdout, &mut result);
        }
        for package in &names {
            if let Ok(bytes) = crate::store::read_file(
                &Self::info(&format!("{package}.conffiles")),
                crate::store::MAX_INPUT,
            ) {
                mark_config(String::from_utf8_lossy(&bytes).lines(), &mut result);
            }
        }
        Ok(result)
    }

    fn files(&self, package: &str, stop: &Cancel) -> Result<Vec<PathBuf>> {
        let out = run("dpkg", &["-L"], &[package.into()], stop)?;
        if out.code != Some(0) {
            bail!("dpkg could not list {package}: {}", out.stderr.trim());
        }
        Ok(out
            .stdout
            .lines()
            .filter(|l| l.starts_with('/'))
            .map(PathBuf::from)
            .collect())
    }

    fn scripts(&self, package: &str, _stop: &Cancel) -> Result<Vec<PackageScript>> {
        let mut scripts = vec![];
        for phase in [
            "preinst", "postinst", "prerm", "postrm", "config", "triggers",
        ] {
            let path = Self::info(&format!("{package}.{phase}"));
            let Ok(bytes) = crate::store::read_file(&path, crate::store::MAX_INPUT) else {
                continue;
            };
            let text = String::from_utf8_lossy(&bytes).into_owned();
            let interpreter = text
                .lines()
                .next()
                .and_then(|l| l.strip_prefix("#!"))
                .unwrap_or("")
                .trim()
                .to_string();
            scripts.push(PackageScript {
                package: package.into(),
                phase: phase.into(),
                interpreter,
                text,
            });
        }
        Ok(scripts)
    }
}

/// `pacman -Qo` output: `/etc/pam.d/sudo is owned by sudo 1.9.17-1`.
pub fn parse_pacman_owners(out: &str) -> BTreeMap<PathBuf, String> {
    out.lines()
        .filter_map(|l| l.split_once(" is owned by "))
        .map(|(path, owner)| (PathBuf::from(path), owner.replacen(' ', "-", 1)))
        .collect()
}

/// `pacman -Qkk` warnings: `warning: sudo: /etc/pam.d/sudo (SHA256 checksum mismatch)`.
pub fn parse_pacman_check(out: &str) -> BTreeMap<PathBuf, FileState> {
    let mut states: BTreeMap<PathBuf, Vec<String>> = BTreeMap::new();
    let mut unreadable: BTreeMap<PathBuf, String> = BTreeMap::new();
    for line in out.lines() {
        let Some(rest) = line.strip_prefix("warning: ") else {
            continue;
        };
        let Some((_, rest)) = rest.split_once(": ") else {
            continue;
        };
        let Some((path, reason)) = rest.rsplit_once(" (") else {
            continue;
        };
        let reason = reason.trim_end_matches(')');
        let change = match reason {
            r if r.contains("checksum mismatch") || r.contains("Size mismatch") => "content",
            r if r.contains("Permissions mismatch") => "mode",
            r if r.contains("UID mismatch") => "owner",
            r if r.contains("GID mismatch") => "group",
            r if r.contains("Symlink path mismatch") => "link target",
            r if r.contains("No such file") => "missing",
            r if r.contains("Modification time mismatch") => continue,
            r => {
                unreadable.insert(PathBuf::from(path), r.to_string());
                continue;
            }
        };
        let list = states.entry(PathBuf::from(path)).or_default();
        if !list.iter().any(|c| c == change) {
            list.push(change.into());
        }
    }
    let mut out: BTreeMap<PathBuf, FileState> = unreadable
        .into_iter()
        .map(|(p, detail)| (p, FileState::Unverifiable { detail }))
        .collect();
    out.extend(
        states
            .into_iter()
            .map(|(p, changes)| (p, FileState::Modified { changes })),
    );
    out
}

#[derive(Default)]
pub struct Pacman;

impl PackageDb for Pacman {
    fn manager(&self) -> &'static str {
        "pacman"
    }

    fn attribute(
        &self,
        paths: &[PathBuf],
        stop: &Cancel,
    ) -> Result<BTreeMap<PathBuf, Attribution>> {
        let mut result: BTreeMap<PathBuf, Attribution> =
            paths.iter().map(|p| (p.clone(), unowned())).collect();
        let mut packages = BTreeSet::new();
        for chunk in paths.chunks(CHUNK) {
            let out = run("pacman", &["-Qo"], &text(chunk), stop)?;
            for (path, package) in parse_pacman_owners(&out.stdout) {
                if let Some(a) = result.get_mut(&path) {
                    packages.insert(
                        package
                            .rsplitn(3, '-')
                            .last()
                            .unwrap_or(&package)
                            .to_string(),
                    );
                    *a = owned(package);
                }
            }
        }
        let names: Vec<String> = packages.into_iter().collect();
        for chunk in names.chunks(CHUNK) {
            let out = run("pacman", &["-Qkk"], chunk, stop)?;
            for (path, state) in parse_pacman_check(&format!("{}\n{}", out.stdout, out.stderr)) {
                if let Some(a) = result.get_mut(&path) {
                    a.state = state;
                }
            }
        }
        // pacman's backup (configuration) marks live in the local database; not read yet.
        Ok(result)
    }

    fn files(&self, package: &str, stop: &Cancel) -> Result<Vec<PathBuf>> {
        let out = run("pacman", &["-Qlq"], &[package.into()], stop)?;
        if out.code != Some(0) {
            bail!("pacman could not list {package}: {}", out.stderr.trim());
        }
        Ok(out
            .stdout
            .lines()
            .filter(|l| !l.ends_with('/'))
            .map(PathBuf::from)
            .collect())
    }

    fn scripts(&self, package: &str, stop: &Cancel) -> Result<Vec<PackageScript>> {
        let out = run("pacman", &["-Q"], &[package.into()], stop)?;
        let Some(version) = out.stdout.split_whitespace().nth(1) else {
            bail!("pacman does not know {package}");
        };
        let install = Path::new("/var/lib/pacman/local")
            .join(format!("{package}-{version}"))
            .join("install");
        let Ok(bytes) = crate::store::read_file(&install, crate::store::MAX_INPUT) else {
            return Ok(vec![]);
        };
        Ok(vec![PackageScript {
            package: package.into(),
            phase: "install".into(),
            interpreter: "/bin/sh".into(),
            text: String::from_utf8_lossy(&bytes).into_owned(),
        }])
    }
}

/// The package database of this system, if it has one Bigman reads.
pub fn package_db() -> Option<Box<dyn PackageDb>> {
    let has = runner::available;
    if has("rpm")
        && (Path::new("/var/lib/rpm").exists() || Path::new("/usr/lib/sysimage/rpm").exists())
    {
        return Some(Box::new(Rpm));
    }
    if has("dpkg") && Path::new("/var/lib/dpkg/status").exists() {
        return Some(Box::new(Dpkg));
    }
    if has("pacman") && Path::new("/var/lib/pacman/local").exists() {
        return Some(Box::new(Pacman));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_rpm_and_dpkg_verify_lines() {
        // Real lines from Fedora 44 as an ordinary user.
        assert_eq!(
            parse_verify_line("..?......  c /etc/sudoers"),
            Some((
                "/etc/sudoers".into(),
                true,
                FileState::Unverifiable {
                    detail: "Could not be read to compare with the package".into()
                }
            ))
        );
        assert_eq!(
            parse_verify_line("S.5....T.  c /etc/pam.d/sudo"),
            Some((
                "/etc/pam.d/sudo".into(),
                true,
                FileState::Modified {
                    changes: vec!["size".into(), "content".into()]
                }
            ))
        );
        assert_eq!(
            parse_verify_line(".......T.    /usr/bin/sudo").map(|v| v.2),
            Some(FileState::Verified)
        );
        assert_eq!(
            parse_verify_line("missing     /var/db/sudo/lectured (Permission denied)").map(|v| v.2),
            Some(FileState::Unverifiable {
                detail: "Permission denied".into()
            })
        );
        assert_eq!(
            parse_verify_line("missing   c /etc/sudo.conf").map(|v| v.2),
            Some(FileState::Modified {
                changes: vec!["missing".into()]
            })
        );
        // dpkg --verify shares the format.
        assert_eq!(
            parse_verify_line("??5?????? c /etc/pam.d/common-auth").map(|v| v.2),
            Some(FileState::Modified {
                changes: vec!["content".into()]
            })
        );
    }

    #[test]
    fn compares_files_with_the_rpm_database() {
        let files = parse_rpm_files("sudo-1.9.17-10.p2.fc44\t8\t/etc/pam.d/sudo\tfb76\t100644\troot\troot\t\tcn\nsudo-1.9.17-10.p2.fc44\t8\t/run/sudo\t\t40711\troot\troot\t\tg\n");
        let pam = &files[Path::new("/etc/pam.d/sudo")];
        assert!(pam.config && !pam.ghost && pam.mode == 0o100644);
        let on_disk = |digest: &str, mode: u32| Actual {
            exists: true,
            mode,
            user: "root".into(),
            group: "root".into(),
            digest: Ok(digest.into()),
            link: None,
        };
        assert_eq!(
            rpm_state(&on_disk("fb76", 0o100644), pam),
            FileState::Verified
        );
        assert_eq!(
            rpm_state(&on_disk("0000", 0o100644), pam),
            FileState::Modified {
                changes: vec!["content".into()]
            }
        );
        assert_eq!(
            rpm_state(&on_disk("fb76", 0o100666), pam),
            FileState::Modified {
                changes: vec!["mode".into()]
            }
        );
        let unreadable = Actual {
            digest: Err("Permission denied".into()),
            ..on_disk("", 0o100644)
        };
        assert_eq!(
            rpm_state(&unreadable, pam),
            FileState::Unverifiable {
                detail: "Permission denied".into()
            }
        );
        let gone = Actual {
            exists: false,
            ..on_disk("", 0)
        };
        assert_eq!(
            rpm_state(&gone, pam),
            FileState::Modified {
                changes: vec!["missing".into()]
            }
        );
        assert_eq!(
            rpm_state(&gone, &files[Path::new("/run/sudo")]),
            FileState::Verified,
            "ghost files may be absent"
        );
    }

    #[test]
    fn splits_rpm_owner_answers_at_the_marker() {
        let marker = "file /tmp/m is not owned by any package";
        let out = format!("glibc-2.43-9.fc44\nglibc-2.43-9.fc44\n{marker}\nfile /home/u/.bashrc is not owned by any package\n{marker}\nsudo-1.9.17\n{marker}\n");
        assert_eq!(
            parse_rpm_owner_runs(&out, marker),
            [
                Some("glibc-2.43-9.fc44".to_string()),
                None,
                Some("sudo-1.9.17".to_string())
            ]
        );
    }

    #[test]
    fn reads_rpm_scriptlets() {
        let out = "preinstall program: /bin/sh\npostinstall scriptlet (using /bin/sh):\nif [ $1 -gt 1 ]; then\n  touch /var/lib/x\nfi\n\npreuninstall scriptlet (using /bin/sh):\n\n";
        let scripts = parse_rpm_scripts("openssh-server", out);
        assert_eq!(scripts.len(), 1);
        assert_eq!(scripts[0].phase, "postinstall");
        assert_eq!(scripts[0].interpreter, "/bin/sh");
        assert_eq!(
            scripts[0].text,
            "if [ $1 -gt 1 ]; then\n  touch /var/lib/x\nfi"
        );
    }

    #[test]
    fn reads_dpkg_and_pacman_owners() {
        let dpkg = parse_dpkg_search("sudo: /etc/pam.d/sudo\nlibc6:amd64, libc6:i386: /etc/ld.so.conf.d/x86_64-linux-gnu.conf\n");
        assert_eq!(dpkg[Path::new("/etc/pam.d/sudo")], "sudo");
        assert_eq!(
            dpkg[Path::new("/etc/ld.so.conf.d/x86_64-linux-gnu.conf")],
            "libc6"
        );
        let pacman = parse_pacman_owners("/etc/pam.d/sudo is owned by sudo 1.9.17.p2-1\n");
        assert_eq!(pacman[Path::new("/etc/pam.d/sudo")], "sudo-1.9.17.p2-1");
        let check = parse_pacman_check(
            "warning: sudo: /etc/pam.d/sudo (SHA256 checksum mismatch)\nwarning: sudo: /etc/pam.d/sudo (Modification time mismatch)\nwarning: sudo: /etc/sudoers (Permission denied)\nsudo: 50 total files, 1 altered file\n",
        );
        assert_eq!(
            check[Path::new("/etc/pam.d/sudo")],
            FileState::Modified {
                changes: vec!["content".into()]
            }
        );
        assert_eq!(
            check[Path::new("/etc/sudoers")],
            FileState::Unverifiable {
                detail: "Permission denied".into()
            }
        );
    }
}

#[cfg(test)]
mod live {
    use super::*;

    /// Against this machine's package database; run with `--ignored`.
    #[test]
    #[ignore]
    fn attributes_real_files() {
        let db = package_db().expect("a package database");
        let paths: Vec<PathBuf> = [
            "/etc/pam.d/sudo",
            "/etc/hosts",
            "/etc/os-release",
            "/etc/ld.so.conf",
        ]
        .iter()
        .map(PathBuf::from)
        .chain(std::env::var_os("HOME").map(|h| Path::new(&h).join(".bashrc")))
        .collect();
        let started = std::time::Instant::now();
        let result = db.attribute(&paths, &runner::cancel()).unwrap();
        for (p, a) in &result {
            eprintln!(
                "{} {:?} config={} {:?}",
                p.display(),
                a.package,
                a.config,
                a.state
            );
        }
        eprintln!("{} in {:?}", db.manager(), started.elapsed());
        eprintln!(
            "{:#?}",
            db.scripts("openssh-server", &runner::cancel())
                .map(|s| s.len())
        );
    }
}
