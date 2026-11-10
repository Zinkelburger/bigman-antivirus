//! Installing the optional root helper that names the process behind a canary
//! access. It is the only system-wide part of canaries, so it is opt-in and
//! removable, and installs a root-owned copy of the binary: root must never
//! run a program your account can change.
use crate::{runner, runner::Cancel};
use anyhow::{bail, Context, Result};
use bigman_canary_helper::{INSTALLED_BINARY, LOCK_ALLOWED, UNIT_PATH};
use sha2::{Digest, Sha256};
use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::{Mutex, PoisonError},
};

/// The helper's unit. It must not contain a single quote: the install script
/// and its preview carry it as one quoted word.
///
/// - No PrivateTmp: the helper uses no temporary files, and with a private
///   /tmp it could not see, and so could not watch, a decoy under /tmp or
///   /var/tmp.
/// - CapabilityBoundingSet: CAP_SYS_ADMIN for fanotify with reader PIDs;
///   CAP_DAC_READ_SEARCH to reach decoys inside home directories closed to
///   others; CAP_SYS_PTRACE for /proc/PID/exe of other users' processes (and
///   all of /proc under hidepid); the rest is what usermod needs to rewrite
///   /etc/passwd and /etc/shadow (mode 000 on Fedora, group shadow on Debian)
///   and write its audit record.
/// - RestrictAddressFamilies: AF_NETLINK is usermod's audit socket.
/// - TasksMax: 64 connections of two threads each, the listener, and one
///   usermod per locking account. MemoryMax: a connection holds well under
///   3 MB at worst (a 256 KiB request, its parsed form, a full kernel event
///   queue), so no one user's four connections come near it.
pub const UNIT: &str = "\
[Unit]
Description=Bigman canary reader identification
Documentation=man:fanotify(7)

[Service]
ExecStart=/usr/local/libexec/bigman/bigman-canary-helper
RuntimeDirectory=bigman-canary
RuntimeDirectoryMode=0755
Restart=on-failure
NoNewPrivileges=yes
PrivateNetwork=yes
ProtectHome=read-only
ProtectSystem=true
RestrictAddressFamilies=AF_UNIX AF_NETLINK
CapabilityBoundingSet=CAP_SYS_ADMIN CAP_DAC_READ_SEARCH CAP_SYS_PTRACE CAP_DAC_OVERRIDE CAP_CHOWN CAP_FOWNER CAP_FSETID CAP_SETUID CAP_SETGID CAP_AUDIT_WRITE
MemoryMax=64M
TasksMax=256

[Install]
WantedBy=multi-user.target
";

/// The SHA-256 the last install preview in this process showed.
static PREVIEWED: Mutex<Option<String>> = Mutex::new(None);

fn quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', r"'\''"))
}

fn quote_path(p: &Path) -> String {
    quote(&p.to_string_lossy())
}

pub fn installed() -> bool {
    Path::new(UNIT_PATH).exists()
}

/// What installing (or removing) does: the root shell script, and for an
/// install the exact binary bytes it copies and their SHA-256.
pub struct Plan {
    pub script: String,
    pub binary: Vec<u8>,
    pub sha256: Option<String>,
    pub source: Option<PathBuf>,
}

/// The helper program to install: only the one next to the running program,
/// where install.sh puts it. PATH is not searched: the first match there can be
/// any directory the environment names.
fn source() -> Result<PathBuf> {
    let source = std::env::current_exe()?.with_file_name("bigman-canary-helper");
    if !source.is_file() {
        bail!(
            "Cannot find {}. Install bigman-canary-helper next to bigman first (install.sh does).",
            source.display()
        );
    }
    Ok(source)
}

/// The root commands that put the helper program at `target`. Standard input
/// goes to a new file in the target directory, which only root can write, is
/// checked there against `sha256`, and takes the program's name only when it
/// matches: root never installs bytes other than the ones that were hashed.
fn stage_script(target: &Path, sha256: &str, user: &str, group: &str) -> String {
    let dir = target.parent().unwrap_or(Path::new("/"));
    let name = target.file_name().unwrap_or_default().to_string_lossy();
    format!(
        "set -eu
umask 022
install -d -m 0755 -o {user} -g {group} {dir}
new=$(mktemp {template})
trap 'rm -f \"$new\"' EXIT
cat > \"$new\"
echo {sha256}'  '\"$new\" | sha256sum -c - > /dev/null || {{ echo 'The helper program is not the one with the SHA-256 in the preview. Nothing was installed.' >&2; exit 1; }}
chown {user}:{group} \"$new\"
chmod 0755 \"$new\"
mv -f \"$new\" {target}",
        dir = quote_path(dir),
        template = quote_path(&dir.join(format!(".{name}.XXXXXX"))),
        sha256 = quote(sha256),
        target = quote_path(target),
    )
}

/// Reads the helper program once. The bytes returned are the ones hashed, the
/// ones the script checks, and the ones `apply` pipes to it.
fn install_plan(source: PathBuf) -> Result<Plan> {
    let binary =
        std::fs::read(&source).with_context(|| format!("Cannot read {}", source.display()))?;
    if binary.is_empty() {
        bail!("{} is empty", source.display());
    }
    let sha256 = format!("{:x}", Sha256::digest(&binary));
    let script = format!(
        "{stage}
printf '%s' {unit} > {unit_path}
chmod 0644 {unit_path}
systemctl daemon-reload
systemctl enable --now bigman-canary-helper.service
systemctl restart bigman-canary-helper.service",
        stage = stage_script(Path::new(INSTALLED_BINARY), &sha256, "root", "root"),
        unit = quote(UNIT),
        unit_path = quote(UNIT_PATH),
    );
    Ok(Plan {
        script,
        binary,
        sha256: Some(sha256),
        source: Some(source),
    })
}

fn removal_plan() -> Plan {
    let lock_dir = Path::new(LOCK_ALLOWED).parent().unwrap_or(Path::new("/"));
    Plan {
        // Removing the opt-in file turns account locking off for everyone, so
        // a later reinstall starts with nobody opted in.
        script: format!(
            "systemctl disable --now bigman-canary-helper.service; rm -f {} {} {} && {{ rmdir {} 2> /dev/null || true; }} && systemctl daemon-reload",
            quote(UNIT_PATH),
            quote(INSTALLED_BINARY),
            quote(LOCK_ALLOWED),
            quote_path(lock_dir),
        ),
        binary: vec![],
        sha256: None,
        source: None,
    }
}

/// The install plan for a preview, remembering in `previewed` the hash it shows.
fn preview_install(source: PathBuf, previewed: &Mutex<Option<String>>) -> Result<Plan> {
    let p = install_plan(source)?;
    *previewed.lock().unwrap_or_else(PoisonError::into_inner) = p.sha256.clone();
    Ok(p)
}

/// The install plan to apply: the program is read again, once, and refused
/// when a preview in this process showed a different hash. Without a preview
/// (the CLI previews and applies in separate runs) the hash of the bytes read
/// now is the one the root script checks and the result reports.
fn checked_install(source: PathBuf, previewed: &Mutex<Option<String>>) -> Result<Plan> {
    let p = install_plan(source)?;
    let shown = previewed
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    if shown.is_some() && shown != p.sha256 {
        bail!("The helper program changed since the preview. Open the preview again.");
    }
    Ok(p)
}

/// The plan a preview shows. For an install it reads and hashes the helper
/// program, and `apply` later refuses a program that hashes differently.
pub fn plan(remove: bool) -> Result<Plan> {
    if remove {
        return Ok(removal_plan());
    }
    if !runner::available("systemctl") {
        bail!("The helper runs as a systemd service, and systemctl was not found");
    }
    preview_install(source()?, &PREVIEWED)
}

pub fn apply(remove: bool, stop: &Cancel) -> Result<String> {
    let p = if remove {
        removal_plan()
    } else {
        if !runner::available("systemctl") {
            bail!("The helper runs as a systemd service, and systemctl was not found");
        }
        checked_install(source()?, &PREVIEWED)?
    };
    let mut cmd = Command::new("pkexec");
    cmd.args(["/bin/sh", "-c", &p.script]);
    let out = runner::run_with_input(&mut cmd, &p.binary, 300, stop)?;
    if !out.ok() {
        bail!(
            "The helper was not {}: {}",
            if remove { "removed" } else { "installed" },
            out.stderr.trim()
        );
    }
    Ok(if remove {
        "Reader identification is off. Accesses are still logged, without the process that made them.".into()
    } else {
        // The preview is used up: a later install starts from its own preview.
        *PREVIEWED.lock().unwrap_or_else(PoisonError::into_inner) = None;
        format!(
            "Reader identification is on (SHA-256 {}). The monitor connects within ten seconds; later accesses name the process and user.",
            p.sha256.unwrap_or_default()
        )
    })
}

/// Whether root turned on account locking for the current account: the helper
/// refuses to lock an account that is not in its opt-in file.
pub fn lock_allowed() -> bool {
    bigman_canary_helper::lock_allowed(Path::new(LOCK_ALLOWED), unsafe { libc::getuid() })
}

fn allow_lock_script_in(file: &Path, uid: u32, user: &str, group: &str) -> String {
    let dir = file.parent().unwrap_or(Path::new("/"));
    let name = file.file_name().unwrap_or_default().to_string_lossy();
    // The list is rebuilt in a new file and renamed into place, so the helper
    // never reads half of it, and whatever was at the path before (other
    // owner, other mode, lines that are not UIDs) does not carry over.
    format!(
        "set -eu
umask 022
install -d -m 0755 -o {user} -g {group} {dir}
new=$(mktemp {template})
trap 'rm -f \"$new\"' EXIT
if [ -f {file} ]; then grep -Ex '[0-9]+' {file} | grep -vx {uid} > \"$new\" || true; fi
echo {uid} >> \"$new\"
chown {user}:{group} \"$new\"
chmod 0644 \"$new\"
mv -f \"$new\" {file}",
        dir = quote_path(dir),
        template = quote_path(&dir.join(format!(".{name}.XXXXXX"))),
        file = quote_path(file),
    )
}

/// The exact root commands that turn on account locking for `uid`: they add it
/// to the helper's opt-in file, once, keeping the accounts already there.
pub fn allow_lock_script(uid: u32) -> String {
    allow_lock_script_in(Path::new(LOCK_ALLOWED), uid, "root", "root")
}

/// Turns on account locking for the current account, with administrator
/// approval through polkit.
pub fn allow_lock() -> Result<()> {
    let mut cmd = Command::new("pkexec");
    cmd.args([
        "/bin/sh",
        "-c",
        &allow_lock_script(unsafe { libc::getuid() }),
    ]);
    let out = runner::run(&mut cmd, 300, &runner::cancel())?;
    if !out.ok() {
        bail!("Account locking was not turned on: {}", out.stderr.trim());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    /// This account, as the scripts' owner where the real ones say root.
    fn me() -> (u32, String, String) {
        let (uid, gid) = unsafe { (libc::getuid(), libc::getgid()) };
        (uid, uid.to_string(), gid.to_string())
    }

    fn sh(script: &str, input: &[u8]) -> runner::Output {
        let mut cmd = Command::new("/bin/sh");
        cmd.args(["-c", script]);
        runner::run_with_input(&mut cmd, input, 30, &runner::cancel()).unwrap()
    }

    fn sha(bytes: &[u8]) -> String {
        format!("{:x}", Sha256::digest(bytes))
    }

    #[test]
    fn the_unit_runs_the_root_owned_copy() {
        assert!(UNIT.contains(&format!("ExecStart={INSTALLED_BINARY}\n")));
        let removal = plan(true).unwrap();
        assert!(removal.script.contains("disable --now"));
        assert!(removal.binary.is_empty());
    }

    #[test]
    fn the_unit_bounds_what_the_helper_may_do_and_use() {
        // The preview replaces the quoted unit by a placeholder, which only
        // works while quoting leaves it unchanged.
        assert!(!UNIT.contains('\''));
        assert_eq!(quote(UNIT), format!("'{UNIT}'"));
        let line = |key: &str| {
            UNIT.lines()
                .find_map(|l| l.strip_prefix(key)?.strip_prefix('='))
                .unwrap_or_else(|| panic!("no {key}"))
                .split_whitespace()
                .collect::<Vec<_>>()
        };
        assert_eq!(line("RestrictAddressFamilies"), ["AF_UNIX", "AF_NETLINK"]);
        let caps = line("CapabilityBoundingSet");
        for needed in [
            "CAP_SYS_ADMIN",
            "CAP_DAC_READ_SEARCH",
            "CAP_SYS_PTRACE",
            "CAP_DAC_OVERRIDE",
            "CAP_CHOWN",
            "CAP_FOWNER",
            "CAP_FSETID",
            "CAP_AUDIT_WRITE",
        ] {
            assert!(caps.contains(&needed), "{needed}");
        }
        for never in [
            "CAP_SYS_MODULE",
            "CAP_NET_ADMIN",
            "CAP_SYS_RAWIO",
            "CAP_SYS_BOOT",
            "CAP_MKNOD",
        ] {
            assert!(!caps.contains(&never), "{never}");
        }
        assert_eq!(line("MemoryMax"), ["64M"]);
        assert_eq!(line("TasksMax"), ["256"]);
        // A private /tmp would hide decoys under /tmp from the helper.
        assert!(!UNIT.contains("PrivateTmp"));
    }

    #[test]
    fn quoting_survives_the_unit_text() {
        let q = quote("a'b");
        assert_eq!(q, r"'a'\''b'");
    }

    #[test]
    fn removal_also_turns_account_locking_off() {
        let script = removal_plan().script;
        assert!(script.contains("rm -f"));
        for path in [UNIT_PATH, INSTALLED_BINARY, LOCK_ALLOWED] {
            assert!(script.contains(&quote(path)), "{path}");
        }
        assert!(script.contains("rmdir '/etc/bigman-canary'"));
    }

    #[test]
    fn the_install_checks_the_hash_of_exactly_the_bytes_it_was_given() {
        let d = tempfile::tempdir().unwrap();
        let source = d.path().join("bigman-canary-helper");
        std::fs::write(&source, b"the helper").unwrap();
        let p = install_plan(source.clone()).unwrap();
        assert_eq!(p.binary, b"the helper");
        assert_eq!(p.sha256.as_deref(), Some(sha(b"the helper").as_str()));
        assert_eq!(p.source.as_deref(), Some(source.as_path()));
        // The root script checks that hash on the file it received, in the
        // root-owned directory, before the file gets the program's name.
        let check = p.script.find("sha256sum -c").unwrap();
        let rename = p
            .script
            .find(&format!("mv -f \"$new\" '{INSTALLED_BINARY}'"))
            .unwrap();
        assert!(check < rename);
        assert!(p.script.contains(&format!(
            "echo '{}''  '\"$new\" | sha256sum -c -",
            sha(b"the helper")
        )));
        assert!(p
            .script
            .contains("mktemp '/usr/local/libexec/bigman/.bigman-canary-helper.XXXXXX'"));
        assert!(p
            .script
            .contains("install -d -m 0755 -o root -g root '/usr/local/libexec/bigman'"));
        assert!(p.script.contains("chown root:root \"$new\""));
        assert!(p.script.contains(&quote(UNIT)));
        assert!(!p.script.contains("/dev/stdin"));
        std::fs::write(&source, b"").unwrap();
        assert!(install_plan(source).is_err(), "an empty program");
        assert!(install_plan(d.path().join("missing")).is_err());
    }

    #[test]
    fn a_program_that_changed_since_the_preview_is_not_installed() {
        let d = tempfile::tempdir().unwrap();
        let source = d.path().join("bigman-canary-helper");
        let previewed = Mutex::new(None);
        std::fs::write(&source, b"previewed").unwrap();
        let shown = preview_install(source.clone(), &previewed).unwrap();
        assert_eq!(*previewed.lock().unwrap(), shown.sha256);
        // Unchanged: the same bytes and hash go to root.
        let p = checked_install(source.clone(), &previewed).unwrap();
        assert_eq!((&p.binary, &p.sha256), (&shown.binary, &shown.sha256));
        // Swapped after the preview: refused, also on a second try.
        std::fs::write(&source, b"swapped").unwrap();
        for _ in 0..2 {
            let e = checked_install(source.clone(), &previewed)
                .err()
                .expect("a changed program was accepted");
            assert_eq!(
                e.to_string(),
                "The helper program changed since the preview. Open the preview again."
            );
        }
        // A new preview shows the new hash and is what gets applied.
        let shown = preview_install(source.clone(), &previewed).unwrap();
        assert_eq!(shown.sha256.as_deref(), Some(sha(b"swapped").as_str()));
        assert!(checked_install(source.clone(), &previewed).is_ok());
        // No preview in this process: the bytes read now are hashed and checked.
        let p = checked_install(source, &Mutex::new(None)).unwrap();
        assert_eq!(p.binary, b"swapped");
        assert!(p.script.contains(&sha(b"swapped")));
    }

    #[test]
    fn the_staging_commands_install_only_bytes_with_the_expected_hash() {
        let d = tempfile::tempdir().unwrap();
        let (_, user, group) = me();
        let target = d.path().join("libexec/bigman/bigman-canary-helper");
        let dir = target.parent().unwrap();
        let script = stage_script(&target, &sha(b"the helper"), &user, &group);
        let out = sh(&script, b"the helper");
        assert!(out.ok(), "{}", out.stderr);
        assert_eq!(std::fs::read(&target).unwrap(), b"the helper");
        assert_eq!(target.metadata().unwrap().mode() & 0o7777, 0o755);
        assert_eq!(dir.metadata().unwrap().mode() & 0o7777, 0o755);
        // Other bytes, more bytes or fewer: refused, and what was installed stays.
        for wrong in [&b"another program"[..], b"the helper\n", b"the helpe"] {
            let out = sh(&script, wrong);
            assert!(!out.ok());
            assert!(
                out.stderr.contains("Nothing was installed"),
                "{}",
                out.stderr
            );
            assert_eq!(std::fs::read(&target).unwrap(), b"the helper");
        }
        // No temporary file is left behind either way.
        let left: Vec<_> = std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        assert_eq!(left, ["bigman-canary-helper"]);
        // An upgrade replaces the program.
        let out = sh(
            &stage_script(&target, &sha(b"version two"), &user, &group),
            b"version two",
        );
        assert!(out.ok(), "{}", out.stderr);
        assert_eq!(std::fs::read(&target).unwrap(), b"version two");
    }

    #[test]
    fn turning_locking_on_lists_the_account_once_and_keeps_the_others() {
        let d = tempfile::tempdir().unwrap();
        let (uid, user, group) = me();
        let file = d.path().join("bigman-canary/lock-uids");
        let allowed = |who: u32| bigman_canary_helper::lock_allowed_owned_by(&file, who, uid);
        assert!(!allowed(1234));
        for _ in 0..3 {
            let out = sh(&allow_lock_script_in(&file, 1234, &user, &group), b"");
            assert!(out.ok(), "{}", out.stderr);
        }
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "1234\n");
        assert_eq!(file.metadata().unwrap().mode() & 0o7777, 0o644);
        assert!(allowed(1234) && !allowed(123) && !allowed(12345));
        let out = sh(&allow_lock_script_in(&file, 123, &user, &group), b"");
        assert!(out.ok(), "{}", out.stderr);
        let out = sh(&allow_lock_script_in(&file, 1234, &user, &group), b"");
        assert!(out.ok(), "{}", out.stderr);
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "123\n1234\n");
        assert!(allowed(1234) && allowed(123));
        // A file left writable by others with other text in it is replaced by
        // a clean one; the UIDs it listed stay.
        std::fs::write(&file, "junk\n1234 x\n55\n\n").unwrap();
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o666)).unwrap();
        let out = sh(&allow_lock_script_in(&file, 77, &user, &group), b"");
        assert!(out.ok(), "{}", out.stderr);
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "55\n77\n");
        assert_eq!(file.metadata().unwrap().mode() & 0o7777, 0o644);
        let left: Vec<_> = std::fs::read_dir(file.parent().unwrap())
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        assert_eq!(left, ["lock-uids"]);
    }

    #[test]
    fn the_real_opt_in_commands_write_roots_file() {
        let script = allow_lock_script(1000);
        assert!(script.contains("install -d -m 0755 -o root -g root '/etc/bigman-canary'"));
        assert!(script.contains("mktemp '/etc/bigman-canary/.lock-uids.XXXXXX'"));
        assert!(script.contains("echo 1000 >> \"$new\""));
        assert!(script.contains("chown root:root \"$new\""));
        assert!(script.contains("chmod 0644 \"$new\""));
        assert!(script.ends_with(&format!("mv -f \"$new\" '{LOCK_ALLOWED}'")));
        // Without the file there is nothing turned on.
        if !Path::new(LOCK_ALLOWED).exists() {
            assert!(!lock_allowed());
        }
    }
}
