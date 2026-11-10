//! One connection: the peer's user ID comes from the kernel (SO_PEERCRED), never
//! from the message, and bounds everything the connection may do.
use crate::{
    encode,
    fanotify::{Event, Group},
    lock_allowed_owned_by, reader, receive, ClientMessage, HelperMessage, Reader, WatchFile,
    LOCK_ALLOWED, MAX_FILES, MAX_ID, MAX_PATH,
};
use anyhow::{anyhow, Context, Result};
use std::{
    collections::{HashMap, HashSet},
    io::BufReader,
    net::Shutdown,
    os::{
        fd::AsRawFd,
        unix::{ffi::OsStrExt, net::UnixStream},
    },
    path::{Path, PathBuf},
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, MutexGuard, PoisonError,
    },
    time::{Duration, Instant},
};

/// What the helper answers when the peer's account never turned locking on.
pub const LOCK_NOT_ALLOWED: &str = "Account locking is not turned on for this account. Choose Lock your account for a decoy in Bigman to turn it on.";
/// How many refusals one Watch answer spells out; the rest are counted.
const MAX_ERRORS: usize = 16;
/// How long a client may leave an answer, or the rest of a line already begun,
/// unread before the connection is closed.
const WRITE_TIMEOUT: Duration = Duration::from_secs(5);

pub struct Config {
    pub proc: PathBuf,
    pub passwd: PathBuf,
    /// Read to tell whether an account is already locked.
    pub shadow: PathBuf,
    pub usermod: PathBuf,
    /// Accounts below this UID (system accounts) are never locked.
    pub uid_min: u32,
    /// Nor are accounts above this one (`nobody`, container and service ranges).
    pub uid_max: u32,
    /// The opt-in file: only the UIDs it lists may lock their account.
    pub lock_allowed: PathBuf,
    /// Who must own the opt-in file: root. Tests cannot create root's files.
    pub lock_allowed_owner: u32,
    /// The shortest time between two usermod runs for one UID.
    pub lock_interval: Duration,
    /// What was tried, for every connection of this process.
    pub locks: Locks,
}

impl Default for Config {
    fn default() -> Self {
        let login_defs = Path::new("/etc/login.defs");
        Self {
            proc: "/proc".into(),
            passwd: "/etc/passwd".into(),
            shadow: "/etc/shadow".into(),
            usermod: "/usr/sbin/usermod".into(),
            uid_min: uid_min(login_defs),
            uid_max: uid_max(login_defs),
            lock_allowed: LOCK_ALLOWED.into(),
            lock_allowed_owner: 0,
            lock_interval: Duration::from_secs(60),
            locks: Locks::default(),
        }
    }
}

/// When usermod last ran for each UID and whether it locked the account. Only
/// UIDs that passed every check get an entry, so it stays small.
#[derive(Default)]
pub struct Locks(Mutex<HashMap<u32, LockRecord>>);

struct LockRecord {
    tried: Instant,
    locked: bool,
}

/// A mutex here guards plain data that stays valid if a holder panicked, so one
/// panic must not turn every later use into another.
fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

fn login_defs_value(login_defs: &Path, key: &str) -> Option<u32> {
    std::fs::read_to_string(login_defs)
        .ok()?
        .lines()
        .find_map(|l| {
            let mut parts = l.split_whitespace();
            (parts.next() == Some(key))
                .then(|| parts.next()?.parse().ok())
                .flatten()
        })
}

/// UID_MIN from login.defs, 1000 when absent.
pub fn uid_min(login_defs: &Path) -> u32 {
    login_defs_value(login_defs, "UID_MIN").unwrap_or(1000)
}

/// UID_MAX from login.defs, 60000 when absent.
pub fn uid_max(login_defs: &Path) -> u32 {
    login_defs_value(login_defs, "UID_MAX").unwrap_or(60000)
}

/// The user ID of the process at the other end of a Unix socket.
pub fn peer_uid(stream: &UnixStream) -> std::io::Result<u32> {
    let mut cred: libc::ucred = unsafe { std::mem::zeroed() };
    let mut len = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
    let r = unsafe {
        libc::getsockopt(
            stream.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_PEERCRED,
            (&mut cred as *mut libc::ucred).cast(),
            &mut len,
        )
    };
    if r != 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(cred.uid)
}

/// The account name for `uid` when /etc/passwd itself defines it: usermod can
/// only lock local accounts, not ones from SSSD, LDAP or systemd-homed.
pub fn local_user(passwd: &Path, uid: u32) -> Option<String> {
    std::fs::read_to_string(passwd).ok()?.lines().find_map(|l| {
        let fields: Vec<&str> = l.split(':').collect();
        (fields.len() >= 3 && fields[2].parse() == Ok(uid) && !fields[0].starts_with('-'))
            .then(|| fields[0].to_string())
            .filter(|n| !n.is_empty())
    })
}

/// Whether /etc/shadow shows `name` the way [`lock_account`] leaves it: password
/// locked and account expired. None when that cannot be read.
pub fn account_locked(shadow: &Path, name: &str) -> Option<bool> {
    std::fs::read_to_string(shadow).ok()?.lines().find_map(|l| {
        let fields: Vec<&str> = l.split(':').collect();
        (fields.len() >= 8 && fields[0] == name)
            .then(|| fields[1].starts_with('!') && fields[7] == "1")
    })
}

/// Locks the account of `uid` (the peer's own): password locked and account
/// expired, so neither passwords nor SSH keys log in. Existing sessions continue.
///
/// Refused unless `uid` is an ordinary local account that root listed in the
/// opt-in file. usermod runs at most once per [`Config::lock_interval`] for a
/// UID and not at all for an account that is already locked, so a client that
/// repeats the request starts nothing.
pub fn lock_account(config: &Config, uid: u32) -> (bool, String) {
    if uid == 0 || uid < config.uid_min || uid > config.uid_max {
        return (
            false,
            format!(
                "UID {uid} is not an ordinary account (UID {} to {} in /etc/login.defs). Bigman never locks root or system accounts.",
                config.uid_min, config.uid_max
            ),
        );
    }
    if !lock_allowed_owned_by(&config.lock_allowed, uid, config.lock_allowed_owner) {
        return (false, LOCK_NOT_ALLOWED.into());
    }
    let Some(name) = local_user(&config.passwd, uid) else {
        return (false, format!("UID {uid} is not a local account in {}: it comes from a directory service such as SSSD or LDAP. Lock it there.", config.passwd.display()));
    };
    let undo =
        format!("An administrator can undo it with: usermod --unlock --expiredate '' {name}");
    {
        // Decided and recorded under one lock, so requests on several
        // connections at once still start usermod once.
        let mut locks = lock(&config.locks.0);
        let record = locks.get(&uid);
        // /etc/shadow decides when it can be read: an administrator may have
        // unlocked the account since, and then it must be lockable again.
        let locked = account_locked(&config.shadow, &name)
            .unwrap_or_else(|| record.is_some_and(|r| r.locked));
        if locked {
            return (
                true,
                format!("The account {name} is already locked. {undo}"),
            );
        }
        if record.is_some_and(|r| r.tried.elapsed() < config.lock_interval) {
            return (
                false,
                format!(
                    "Not tried again: Bigman tried to lock the account {name} less than {} seconds ago.",
                    config.lock_interval.as_secs()
                ),
            );
        }
        locks.insert(
            uid,
            LockRecord {
                tried: Instant::now(),
                locked: false,
            },
        );
    }
    match Command::new(&config.usermod)
        .args(["--lock", "--expiredate", "1", &name])
        .output()
    {
        Ok(o) if o.status.success() => {
            if let Some(r) = lock(&config.locks.0).get_mut(&uid) {
                r.locked = true;
            }
            (
                true,
                format!("Locked and expired the account {name}. {undo}"),
            )
        }
        Ok(o) => (
            false,
            format!(
                "usermod failed: {}",
                String::from_utf8_lossy(&o.stderr).trim()
            ),
        ),
        Err(e) => (false, format!("Cannot run usermod: {e}")),
    }
}

/// Writes `line` whole or not at all, never waiting on a client for long.
///
/// With `patient` false, a socket whose buffer is full takes nothing and the
/// answer is Ok(false): the caller drops the message. Otherwise, and once part
/// of a line is written (the rest must follow or the stream is garbage), the
/// client has [`WRITE_TIMEOUT`] to make room before the write fails.
fn write_line(stream: &UnixStream, line: &[u8], patient: bool) -> std::io::Result<bool> {
    let fd = stream.as_raw_fd();
    let deadline = Instant::now() + WRITE_TIMEOUT;
    let mut written = 0;
    while written < line.len() {
        let rest = &line[written..];
        let n = unsafe {
            libc::send(
                fd,
                rest.as_ptr().cast(),
                rest.len(),
                libc::MSG_DONTWAIT | libc::MSG_NOSIGNAL,
            )
        };
        if n >= 0 {
            written += n as usize;
            continue;
        }
        let e = std::io::Error::last_os_error();
        match e.kind() {
            std::io::ErrorKind::Interrupted => {}
            std::io::ErrorKind::WouldBlock => {
                if written == 0 && !patient {
                    return Ok(false);
                }
                let left = deadline.saturating_duration_since(Instant::now());
                if left.is_zero() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "The client stopped reading",
                    ));
                }
                let mut p = libc::pollfd {
                    fd,
                    events: libc::POLLOUT,
                    revents: 0,
                };
                unsafe { libc::poll(&mut p, 1, left.as_millis() as i32) };
            }
            _ => return Err(e),
        }
    }
    Ok(true)
}

/// Passes events to the client as they come and keeps none: an event the
/// client has no room for is dropped, and the client is told once it reads
/// again that something is missing.
struct Forwarder<'a> {
    writer: &'a Mutex<UnixStream>,
    lost: bool,
}

impl Forwarder<'_> {
    fn offer(&self, message: &HelperMessage) -> std::io::Result<bool> {
        write_line(&lock(self.writer), &encode(message)?, false)
    }

    /// Reports a gap when there is one and the client has room to hear of it.
    fn report_lost(&mut self) -> std::io::Result<()> {
        if self.lost && self.offer(&HelperMessage::Lost)? {
            self.lost = false;
        }
        Ok(())
    }

    /// `describe` reads /proc for one PID. It is called just before its event
    /// is sent, and not at all for events that are dropped.
    fn forward(
        &mut self,
        events: Vec<Event>,
        describe: impl Fn(u32) -> Reader,
    ) -> std::io::Result<()> {
        for e in events {
            self.report_lost()?;
            if e.mask & libc::FAN_Q_OVERFLOW != 0 {
                self.lost = true;
            }
            if self.lost {
                continue;
            }
            let message = HelperMessage::Event {
                id: e.id,
                mask: e.mask,
                reader: e.pid.map(&describe),
            };
            self.lost = !self.offer(&message)?;
        }
        self.report_lost()
    }
}

/// Closes the connection when the events thread ends, however it ends: the
/// client then sees the helper go away and reconnects, instead of waiting on a
/// connection that will never report anything again.
struct Hangup(UnixStream);

impl Drop for Hangup {
    fn drop(&mut self) {
        let _ = self.0.shutdown(Shutdown::Both);
    }
}

/// Replaces the connection's decoys with `files`, as far as the limits allow.
fn watch(group: &mut Group, files: Vec<WatchFile>, uid: u32) -> HelperMessage {
    group.clear();
    let mut ids = vec![];
    let mut errors = vec![];
    let mut unsaid = 0;
    let mut refuse = |error: String| {
        if errors.len() < MAX_ERRORS {
            errors.push(error);
        } else {
            unsaid += 1;
        }
    };
    if files.len() > MAX_FILES {
        refuse(format!("Only the first {MAX_FILES} decoys are watched"));
    }
    let mut seen = HashSet::new();
    for f in files.into_iter().take(MAX_FILES) {
        if f.id.is_empty() || f.id.len() > MAX_ID {
            refuse(format!(
                "A decoy ID must be 1 to {MAX_ID} bytes long; one of {} bytes was not watched",
                f.id.len()
            ));
            continue;
        }
        let length = f.path.as_os_str().as_bytes().len();
        if length > MAX_PATH {
            refuse(format!(
                "A decoy path of {length} bytes was not watched; the longest allowed is {MAX_PATH}"
            ));
            continue;
        }
        if !seen.insert(f.id.clone()) {
            continue;
        }
        match group.watch(&f.id, &f.path, uid) {
            Ok(()) => ids.push(f.id),
            Err(e) => refuse(format!("{e:#}")),
        }
    }
    if unsaid > 0 {
        errors.push(format!("{unsaid} more decoys could not be watched"));
    }
    HelperMessage::Watching { ids, errors }
}

/// Serves one connection until the peer disconnects.
pub fn serve(config: &Config, stream: UnixStream, uid: u32) -> Result<()> {
    let group = Arc::new(Mutex::new(Group::new()?));
    let writer = Arc::new(Mutex::new(stream.try_clone()?));
    let done = Arc::new(AtomicBool::new(false));
    let hangup = Hangup(stream.try_clone()?);
    let events = {
        let (group, writer, done) = (group.clone(), writer.clone(), done.clone());
        let proc = config.proc.clone();
        let fd = lock(&group).as_raw_fd();
        std::thread::Builder::new()
            .name("events".into())
            .spawn(move || -> Result<()> {
                let _hangup = hangup;
                let mut forwarder = Forwarder {
                    writer: &writer,
                    lost: false,
                };
                while !done.load(Ordering::Relaxed) {
                    let mut p = libc::pollfd {
                        fd,
                        events: libc::POLLIN,
                        revents: 0,
                    };
                    if unsafe { libc::poll(&mut p, 1, 500) } <= 0 {
                        forwarder.report_lost()?;
                        continue;
                    }
                    // One buffer of events at a time, each sent or dropped
                    // before the next is read: nothing piles up here.
                    let batch = lock(&group).events()?;
                    forwarder.forward(batch, |pid| reader::describe(&proc, pid, uid))?;
                }
                Ok(())
            })
            .context("Cannot start a thread for the connection")?
    };
    let mut input = BufReader::new(stream);
    let result = (|| -> Result<()> {
        while let Some(message) = receive::<ClientMessage>(&mut input)? {
            let reply = match message {
                ClientMessage::Watch { files } => watch(&mut lock(&group), files, uid),
                ClientMessage::LockAccount => {
                    let (ok, detail) = lock_account(config, uid);
                    HelperMessage::Locked { ok, detail }
                }
            };
            write_line(&lock(&writer), &encode(&reply)?, true)?;
        }
        Ok(())
    })();
    done.store(true, Ordering::Relaxed);
    let events = events
        .join()
        .unwrap_or_else(|_| Err(anyhow!("The events thread panicked")));
    // A client that left is not an error, whichever side notices. One that
    // left with events it never read still queued shows up as a reset, on the
    // write of the next event or on the read for its next request.
    let left = |e: anyhow::Error| match e.downcast_ref::<std::io::Error>() {
        Some(io)
            if matches!(
                io.kind(),
                std::io::ErrorKind::BrokenPipe | std::io::ErrorKind::ConnectionReset
            ) =>
        {
            Ok(())
        }
        _ => Err(e),
    };
    result.or_else(left).and(events.or_else(left))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Client, MAX_LINE};
    use std::{io::Read, os::unix::fs::PermissionsExt};

    fn me() -> u32 {
        unsafe { libc::getuid() }
    }

    const PASSWD: &str = "root:x:0:0::/root:/bin/bash\nsvc:x:500:500::/:/sbin/nologin\nana:x:1001:1001::/home/ana:/bin/bash\nbo:x:1002:1002::/home/bo:/bin/bash\nnobody:x:65534:65534::/:/sbin/nologin\n";

    /// A helper whose files are all under `d`. Account locking is turned on
    /// for ana (1001) and nobody (65534), not for bo (1002), and "usermod" is
    /// a program that does nothing.
    fn config(d: &Path) -> Config {
        std::fs::write(d.join("passwd"), PASSWD).unwrap();
        allow(d, "0\n500\n1001\n65534\n");
        Config {
            proc: "/proc".into(),
            passwd: d.join("passwd"),
            shadow: d.join("shadow"),
            usermod: "/bin/true".into(),
            uid_min: 1000,
            uid_max: 60000,
            lock_allowed: d.join("lock-uids"),
            lock_allowed_owner: me(),
            lock_interval: Duration::from_secs(60),
            locks: Locks::default(),
        }
    }

    fn allow(d: &Path, uids: &str) {
        let file = d.join("lock-uids");
        std::fs::write(&file, uids).unwrap();
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o644)).unwrap();
    }

    /// A stand-in for usermod that records its arguments on each run and exits
    /// with `code`. A child process writes it: a file this threaded test
    /// process had open for writing could still be open in another test's
    /// fork when it is run, which fails with "text file busy".
    fn fake_usermod(d: &Path, code: i32) -> PathBuf {
        let path = d.join(format!("usermod-{code}"));
        let status = Command::new("/bin/sh")
            .arg("-c")
            .arg(r#"printf '#!/bin/sh\necho "$@" >> "$0.runs"\nexit %s\n' "$2" > "$1" && chmod 755 "$1""#)
            .arg("sh")
            .arg(&path)
            .arg(code.to_string())
            .status()
            .unwrap();
        assert!(status.success());
        path
    }

    fn runs(usermod: &Path) -> Vec<String> {
        std::fs::read_to_string(format!("{}.runs", usermod.display()))
            .unwrap_or_default()
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn reads_the_uid_range_and_finds_only_local_accounts() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(
            d.path().join("defs"),
            "# c\nUID_MIN   2000\nUID_MAX 59999\nSYS_UID_MAX 999\n#UID_MAX 5\n",
        )
        .unwrap();
        assert_eq!(uid_min(&d.path().join("defs")), 2000);
        assert_eq!(uid_max(&d.path().join("defs")), 59999);
        assert_eq!(uid_min(&d.path().join("missing")), 1000);
        assert_eq!(uid_max(&d.path().join("missing")), 60000);
        std::fs::write(
            d.path().join("passwd"),
            "root:x:0:0::/root:/bin/bash\nana:x:1001:1001::/home/ana:/bin/bash\n-evil:x:1002:1002::/:/bin/sh\n",
        )
        .unwrap();
        assert_eq!(
            local_user(&d.path().join("passwd"), 1001).as_deref(),
            Some("ana")
        );
        assert_eq!(local_user(&d.path().join("passwd"), 1002), None);
        assert_eq!(local_user(&d.path().join("passwd"), 4242), None);
    }

    #[test]
    fn never_locks_root_system_nobody_or_directory_accounts() {
        let d = tempfile::tempdir().unwrap();
        let usermod = fake_usermod(d.path(), 0);
        let c = Config {
            usermod: usermod.clone(),
            ..config(d.path())
        };
        // All of these are in the opt-in file and in passwd: the range decides.
        for uid in [0, 500, 999, 60001, 65534, u32::MAX] {
            let (ok, detail) = lock_account(&c, uid);
            assert!(
                !ok && detail.contains("not an ordinary account"),
                "{detail}"
            );
        }
        allow(d.path(), "5000\n60000\n");
        let (ok, detail) = lock_account(&c, 5000);
        assert!(!ok && detail.contains("directory service"), "{detail}");
        let (ok, detail) = lock_account(&c, 60000);
        assert!(!ok && detail.contains("directory service"), "{detail}");
        assert!(runs(&usermod).is_empty());
    }

    #[test]
    fn locks_only_accounts_that_root_turned_locking_on_for() {
        let d = tempfile::tempdir().unwrap();
        let usermod = fake_usermod(d.path(), 0);
        let c = Config {
            usermod: usermod.clone(),
            ..config(d.path())
        };
        // bo is a local account in range, but not in the opt-in file.
        assert_eq!(lock_account(&c, 1002), (false, LOCK_NOT_ALLOWED.into()));
        // A file that is not its required owner's counts for nobody.
        let strangers = Config {
            usermod: usermod.clone(),
            lock_allowed_owner: me() + 1,
            ..config(d.path())
        };
        assert_eq!(
            lock_account(&strangers, 1001),
            (false, LOCK_NOT_ALLOWED.into())
        );
        // Nor does one others can write, or none at all.
        std::fs::set_permissions(
            d.path().join("lock-uids"),
            std::fs::Permissions::from_mode(0o666),
        )
        .unwrap();
        assert_eq!(lock_account(&c, 1001), (false, LOCK_NOT_ALLOWED.into()));
        std::fs::remove_file(d.path().join("lock-uids")).unwrap();
        assert_eq!(lock_account(&c, 1001), (false, LOCK_NOT_ALLOWED.into()));
        assert!(runs(&usermod).is_empty(), "usermod ran for a refusal");

        allow(d.path(), "1001\n");
        let (ok, detail) = lock_account(&c, 1001);
        assert!(ok, "{detail}");
        assert!(detail.contains("usermod --unlock --expiredate '' ana"));
        assert_eq!(runs(&usermod), ["--lock --expiredate 1 ana"]);
        // The default is root's file at the fixed path.
        let default = Config::default();
        assert_eq!(default.lock_allowed, Path::new(LOCK_ALLOWED));
        assert_eq!(default.lock_allowed_owner, 0);
        assert_eq!(default.lock_interval, Duration::from_secs(60));
    }

    #[test]
    fn a_repeated_request_does_not_run_usermod_again() {
        let d = tempfile::tempdir().unwrap();
        let failing = fake_usermod(d.path(), 1);
        let c = Config {
            usermod: failing.clone(),
            ..config(d.path())
        };
        let (ok, detail) = lock_account(&c, 1001);
        assert!(!ok && detail.contains("usermod failed"), "{detail}");
        for _ in 0..50 {
            let (ok, detail) = lock_account(&c, 1001);
            assert!(!ok && detail.contains("Not tried again"), "{detail}");
        }
        assert_eq!(runs(&failing).len(), 1);
        // Another account has its own minute.
        allow(d.path(), "1001\n1002\n");
        assert!(lock_account(&c, 1002).1.contains("usermod failed"));
        assert_eq!(runs(&failing).len(), 2);
        // Once the interval has passed, a failed lock may be tried again.
        let patient = Config {
            usermod: failing.clone(),
            lock_interval: Duration::ZERO,
            ..config(d.path())
        };
        assert!(lock_account(&patient, 1001).1.contains("usermod failed"));
        assert!(lock_account(&patient, 1001).1.contains("usermod failed"));
        assert_eq!(runs(&failing).len(), 4);
    }

    #[test]
    fn requests_on_many_connections_at_once_run_usermod_once() {
        let d = tempfile::tempdir().unwrap();
        let usermod = fake_usermod(d.path(), 1);
        let c = Config {
            usermod: usermod.clone(),
            ..config(d.path())
        };
        std::thread::scope(|s| {
            for _ in 0..8 {
                s.spawn(|| lock_account(&c, 1001));
            }
        });
        assert_eq!(runs(&usermod).len(), 1);
    }

    #[test]
    fn a_locked_account_is_not_locked_again() {
        let d = tempfile::tempdir().unwrap();
        let usermod = fake_usermod(d.path(), 0);
        // No interval, so only the lock itself can hold a repeat back.
        let c = Config {
            usermod: usermod.clone(),
            lock_interval: Duration::ZERO,
            ..config(d.path())
        };
        assert!(lock_account(&c, 1001).1.contains("Locked and expired"));
        for _ in 0..5 {
            let (ok, detail) = lock_account(&c, 1001);
            assert!(ok && detail.contains("already locked"), "{detail}");
            assert!(detail.contains("usermod --unlock --expiredate '' ana"));
        }
        assert_eq!(runs(&usermod).len(), 1);

        // Where /etc/shadow can be read, it decides. An administrator unlocked
        // the account: the next request locks it again.
        std::fs::write(
            d.path().join("shadow"),
            "root:!::0:99999:7:::\nana:$y$hash:19000:0:99999:7:::\n",
        )
        .unwrap();
        assert!(lock_account(&c, 1001).1.contains("Locked and expired"));
        assert_eq!(runs(&usermod).len(), 2);
        // Locked but not expired is not what a lock leaves: it runs.
        std::fs::write(d.path().join("shadow"), "ana:!$y$hash:19000:0:99999:7:::\n").unwrap();
        assert!(lock_account(&c, 1001).1.contains("Locked and expired"));
        assert_eq!(runs(&usermod).len(), 3);
        // Locked and expired: nothing runs, for a helper that never locked it too.
        std::fs::write(
            d.path().join("shadow"),
            "ana:!$y$hash:19000:0:99999:7::1:\n",
        )
        .unwrap();
        let fresh = Config {
            usermod: usermod.clone(),
            ..config(d.path())
        };
        for c in [&c, &fresh] {
            let (ok, detail) = lock_account(c, 1001);
            assert!(ok && detail.contains("already locked"), "{detail}");
        }
        assert_eq!(runs(&usermod).len(), 3);
    }

    #[test]
    fn reads_the_lock_state_from_shadow() {
        let d = tempfile::tempdir().unwrap();
        let shadow = d.path().join("shadow");
        assert_eq!(account_locked(&shadow, "ana"), None);
        std::fs::write(
            &shadow,
            "ana:!$y$hash:19000:0:99999:7::1:\nbo:$y$hash:19000:0:99999:7::1:\ncy:!!:19000:0:99999:7:::\nshort:!\n",
        )
        .unwrap();
        assert_eq!(account_locked(&shadow, "ana"), Some(true));
        assert_eq!(account_locked(&shadow, "bo"), Some(false));
        assert_eq!(account_locked(&shadow, "cy"), Some(false));
        assert_eq!(account_locked(&shadow, "short"), None);
        assert_eq!(account_locked(&shadow, "an"), None);
    }

    fn file(id: impl Into<String>, path: impl Into<PathBuf>) -> WatchFile {
        WatchFile {
            id: id.into(),
            path: path.into(),
        }
    }

    #[test]
    fn a_watch_is_held_to_the_id_path_and_count_limits() {
        let d = tempfile::tempdir().unwrap();
        let decoy = d.path().join("decoy.env");
        let other = d.path().join("other.env");
        std::fs::write(&decoy, "fake").unwrap();
        std::fs::write(&other, "fake").unwrap();
        let mut group = Group::new().unwrap();
        let longest = "i".repeat(MAX_ID);
        let reply = watch(
            &mut group,
            vec![
                file("decoy", &decoy),
                file("i".repeat(MAX_ID + 1), &decoy),
                file("", &decoy),
                file("i".repeat(100_000), &decoy),
                file(longest.clone(), &other),
                file("long-path", format!("/{}", "p".repeat(MAX_PATH))),
                file("decoy", &other),
            ],
            me(),
        );
        let HelperMessage::Watching { ids, errors } = reply else {
            panic!("not a Watching reply")
        };
        assert_eq!(ids, ["decoy".to_string(), longest]);
        assert_eq!(
            errors,
            [
                "A decoy ID must be 1 to 64 bytes long; one of 65 bytes was not watched",
                "A decoy ID must be 1 to 64 bytes long; one of 0 bytes was not watched",
                "A decoy ID must be 1 to 64 bytes long; one of 100000 bytes was not watched",
                "A decoy path of 4097 bytes was not watched; the longest allowed is 4096",
            ]
        );
    }

    #[test]
    fn the_answer_to_a_watch_stays_small_whatever_was_asked() {
        let mut group = Group::new().unwrap();
        // More files than allowed, every one refused with the longest message.
        let files: Vec<_> = (0..MAX_FILES + 44)
            .map(|n| file(n.to_string(), format!("/{}/{n}", "p".repeat(MAX_PATH - 8))))
            .collect();
        let reply = watch(&mut group, files, me());
        assert!(encode(&reply).unwrap().len() < MAX_LINE / 2);
        let HelperMessage::Watching { ids, errors } = reply else {
            panic!("not a Watching reply")
        };
        assert!(ids.is_empty());
        assert_eq!(errors.len(), MAX_ERRORS + 1);
        assert_eq!(errors[0], "Only the first 256 decoys are watched");
        assert!(errors[1].ends_with("it must be an existing regular file that you own."));
        // 256 were looked at; 15 of those refusals are spelled out.
        assert_eq!(errors[MAX_ERRORS], "241 more decoys could not be watched");
    }

    fn event(pid: u32) -> Event {
        Event {
            id: "decoy".into(),
            mask: libc::FAN_ACCESS,
            pid: Some(pid),
        }
    }

    /// Everything waiting on `stream`, as messages. Only whole lines may be there.
    fn waiting(stream: &mut UnixStream) -> Vec<HelperMessage> {
        stream.set_nonblocking(true).unwrap();
        let mut bytes = vec![];
        let mut chunk = [0u8; 64 * 1024];
        loop {
            match stream.read(&mut chunk) {
                Ok(0) => break,
                Ok(n) => bytes.extend_from_slice(&chunk[..n]),
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) => panic!("{e}"),
            }
        }
        assert!(bytes.is_empty() || bytes.ends_with(b"\n"), "a cut-off line");
        bytes
            .split(|b| *b == b'\n')
            .filter(|l| !l.is_empty())
            .map(|l| serde_json::from_slice(l).unwrap())
            .collect()
    }

    #[test]
    fn a_client_that_stops_reading_loses_events_and_is_told() {
        let (mut client, server) = UnixStream::pair().unwrap();
        let writer = Mutex::new(server);
        let mut forwarder = Forwarder {
            writer: &writer,
            lost: false,
        };
        let described = std::cell::Cell::new(0u32);
        let describe = |pid| {
            described.set(described.get() + 1);
            Reader {
                pid,
                cmdline: "x".repeat(4000),
                ..Reader::default()
            }
        };
        // About 2 MB of events for a client that reads none of them.
        let started = Instant::now();
        forwarder
            .forward((1..=500).map(event).collect(), describe)
            .unwrap();
        assert!(
            started.elapsed() < WRITE_TIMEOUT,
            "it waited for the client"
        );
        assert!(forwarder.lost);
        // Described: the events that fitted and the first one that did not.
        let fitted = described.get() - 1;
        assert!((1..499).contains(&fitted), "{fitted}");
        // While the client is not reading, further events cost no /proc reads.
        forwarder
            .forward((501..=600).map(event).collect(), describe)
            .unwrap();
        forwarder.report_lost().unwrap();
        assert!(forwarder.lost);
        assert_eq!(described.get(), fitted + 1);
        // What did arrive is whole and in order.
        let got = waiting(&mut client);
        let pids: Vec<u32> = got
            .iter()
            .map(|m| match m {
                HelperMessage::Event {
                    reader: Some(r), ..
                } => r.pid,
                other => panic!("{other:?}"),
            })
            .collect();
        assert_eq!(pids, (1..=fitted).collect::<Vec<_>>());
        // The client reads again: it hears of the gap first, then of new events.
        forwarder.forward(vec![event(601)], describe).unwrap();
        assert!(!forwarder.lost);
        let got = waiting(&mut client);
        assert_eq!(got.len(), 2, "{got:?}");
        assert_eq!(got[0], HelperMessage::Lost);
        assert!(matches!(&got[1], HelperMessage::Event { reader: Some(r), .. } if r.pid == 601));
    }

    #[test]
    fn a_kernel_overflow_is_reported_as_lost() {
        let (mut client, server) = UnixStream::pair().unwrap();
        let writer = Mutex::new(server);
        let mut forwarder = Forwarder {
            writer: &writer,
            lost: false,
        };
        let overflow = Event {
            id: String::new(),
            mask: libc::FAN_Q_OVERFLOW,
            pid: None,
        };
        forwarder
            .forward(vec![event(1), overflow, event(2)], |pid| Reader {
                pid,
                ..Reader::default()
            })
            .unwrap();
        let got = waiting(&mut client);
        assert_eq!(got.len(), 3, "{got:?}");
        assert_eq!(got[1], HelperMessage::Lost);
    }

    #[test]
    fn an_answer_waits_for_a_slow_client_but_an_event_does_not() {
        let (mut client, server) = UnixStream::pair().unwrap();
        let line = vec![b'a'; 32 * 1024];
        while write_line(&server, &line, false).unwrap() {}
        // Full. An event is refused at once; an answer goes out when a reader
        // makes room.
        let started = Instant::now();
        assert!(!write_line(&server, &line, false).unwrap());
        assert!(started.elapsed() < Duration::from_secs(1));
        let reader = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(200));
            let mut total = 0;
            let mut chunk = [0u8; 64 * 1024];
            while let Ok(n) = client.read(&mut chunk) {
                if n == 0 {
                    break;
                }
                total += n;
            }
            total
        });
        assert!(write_line(&server, b"answer\n", true).unwrap());
        drop(server);
        assert_eq!(reader.join().unwrap() % line.len(), b"answer\n".len());
    }

    fn connect(stream: UnixStream) -> Client {
        let client = Client {
            writer: stream.try_clone().unwrap(),
            reader: BufReader::new(stream),
            pending: vec![],
        };
        client.reader.get_ref().set_nonblocking(true).unwrap();
        client
    }

    fn watching(client: &mut Client, files: Vec<WatchFile>) -> (Vec<String>, Vec<String>) {
        let (reply, _) = client
            .request(
                &ClientMessage::Watch { files },
                Duration::from_secs(5),
                |m| matches!(m, HelperMessage::Watching { .. }),
            )
            .unwrap();
        let Some(HelperMessage::Watching { ids, errors }) = reply else {
            panic!("no Watching reply")
        };
        (ids, errors)
    }

    #[test]
    fn a_connection_watches_its_own_files_and_reports_reads() {
        let d = tempfile::tempdir().unwrap();
        let decoy = d.path().join("decoy.env");
        std::fs::write(&decoy, "fake").unwrap();
        let (client, server) = UnixStream::pair().unwrap();
        assert_eq!(peer_uid(&server).unwrap(), me());
        let c = config(d.path());
        let handle = std::thread::spawn(move || serve(&c, server, me()));
        let mut client = connect(client);
        let (ids, errors) = watching(
            &mut client,
            vec![
                file("decoy", &decoy),
                file("passwd", "/etc/passwd"),
                file("i".repeat(MAX_ID + 1), &decoy),
            ],
        );
        assert_eq!(ids, ["decoy"]);
        assert_eq!(
            errors,
            [
                "Cannot watch /etc/passwd: it must be an existing regular file that you own.",
                "A decoy ID must be 1 to 64 bytes long; one of 65 bytes was not watched",
            ]
        );
        std::fs::read(&decoy).unwrap();
        let started = Instant::now();
        let mut seen = vec![];
        while started.elapsed() < Duration::from_secs(5) && seen.is_empty() {
            seen = client.poll().unwrap();
            std::thread::sleep(Duration::from_millis(20));
        }
        assert!(seen
            .iter()
            .any(|m| matches!(m, HelperMessage::Event { id, .. } if id == "decoy")));
        drop(client);
        handle.join().unwrap().unwrap();
    }

    #[test]
    fn a_client_that_leaves_with_events_unread_is_not_an_error() {
        let d = tempfile::tempdir().unwrap();
        let decoy = d.path().join("decoy.env");
        std::fs::write(&decoy, "fake").unwrap();
        let (client, server) = UnixStream::pair().unwrap();
        let c = config(d.path());
        let handle = std::thread::spawn(move || serve(&c, server, me()));
        let mut client = connect(client);
        let (ids, _) = watching(&mut client, vec![file("decoy", &decoy)]);
        assert_eq!(ids, ["decoy"]);
        std::fs::read(&decoy).unwrap();
        // Long enough for the event to be written to the socket, and never read.
        std::thread::sleep(Duration::from_millis(800));
        drop(client);
        handle.join().unwrap().unwrap();
    }

    #[test]
    fn a_connection_locks_its_own_account_only_when_that_is_turned_on() {
        let d = tempfile::tempdir().unwrap();
        let usermod = fake_usermod(d.path(), 0);
        let c = Config {
            usermod: usermod.clone(),
            // Whatever account runs the tests counts as an ordinary one.
            uid_min: 0,
            uid_max: u32::MAX,
            ..config(d.path())
        };
        if me() == 0 {
            return;
        }
        std::fs::write(&c.passwd, format!("tester:x:{}:1::/:/bin/sh\n", me())).unwrap();
        allow(d.path(), "");
        let (client, server) = UnixStream::pair().unwrap();
        let handle = std::thread::spawn(move || serve(&c, server, me()));
        let mut client = connect(client);
        let mut ask = || {
            let (reply, _) = client
                .request(&ClientMessage::LockAccount, Duration::from_secs(5), |m| {
                    matches!(m, HelperMessage::Locked { .. })
                })
                .unwrap();
            match reply {
                Some(HelperMessage::Locked { ok, detail }) => (ok, detail),
                other => panic!("{other:?}"),
            }
        };
        assert_eq!(ask(), (false, LOCK_NOT_ALLOWED.into()));
        assert!(runs(&usermod).is_empty());
        allow(d.path(), &format!("{}\n", me()));
        let (ok, detail) = ask();
        assert!(ok && detail.contains("Locked and expired"), "{detail}");
        let (ok, detail) = ask();
        assert!(ok && detail.contains("already locked"), "{detail}");
        assert_eq!(runs(&usermod), ["--lock --expiredate 1 tester"]);
        drop(client);
        handle.join().unwrap().unwrap();
    }

    #[test]
    fn the_connection_closes_when_its_events_can_no_longer_be_sent() {
        let d = tempfile::tempdir().unwrap();
        let decoy = d.path().join("decoy.env");
        std::fs::write(&decoy, "fake").unwrap();
        let (client, server) = UnixStream::pair().unwrap();
        let c = config(d.path());
        let (ended, end) = std::sync::mpsc::channel();
        std::thread::spawn(move || ended.send(serve(&c, server, me()).is_ok()));
        let mut client = connect(client);
        assert_eq!(
            watching(&mut client, vec![file("decoy", &decoy)]).0,
            ["decoy"]
        );
        // The events thread fails on its next write. The client keeps its side
        // open and sends nothing, so only the helper can end the connection.
        client.writer.shutdown(Shutdown::Read).unwrap();
        std::fs::read(&decoy).unwrap();
        assert_eq!(end.recv_timeout(Duration::from_secs(5)), Ok(true));
        drop(client);
    }
}
