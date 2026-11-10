//! The optional canary helper: a small root service that tells a user's canary
//! monitor which process and user touched a decoy.
//!
//! inotify, which the per-user monitor uses, reports that a file was read but
//! not by whom. fanotify reports the reading process, but only to root. The
//! helper bridges the two: a monitor connects to [`SOCKET`], names the decoys it
//! owns, and receives one [`HelperMessage::Event`] per access with the reader's
//! PID, user and command line.
//!
//! Limits, enforced by the helper rather than trusted from the client:
//! - a connection may only watch regular files its own user owns, so nobody can
//!   learn who reads someone else's files, and a refusal never says whether a
//!   path the user cannot see exists;
//! - [`ClientMessage::LockAccount`] only ever locks the connecting user's own
//!   local account, never root or a system account, only when root listed that
//!   user in [`LOCK_ALLOWED`], and at most once a minute;
//! - a connection holds a bounded amount of memory: IDs, paths and the number
//!   of decoys are capped, and events a client does not read are dropped (it is
//!   told with [`HelperMessage::Lost`]) rather than queued.
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, Read, Write},
    os::unix::{
        fs::{MetadataExt, OpenOptionsExt},
        net::UnixStream,
    },
    path::{Path, PathBuf},
    time::Duration,
};

pub mod fanotify;
pub mod reader;
pub mod server;

/// Where the helper listens. systemd creates the directory (RuntimeDirectory).
pub const SOCKET: &str = "/run/bigman-canary/helper.sock";
/// Where Setup installs the helper binary, owned by root.
pub const INSTALLED_BINARY: &str = "/usr/local/libexec/bigman/bigman-canary-helper";
/// The helper's system unit.
pub const UNIT_PATH: &str = "/etc/systemd/system/bigman-canary-helper.service";
/// A line longer than this is a protocol error, so one client cannot exhaust memory.
pub const MAX_LINE: usize = 256 * 1024;
/// How many decoys one connection may watch.
pub const MAX_FILES: usize = 256;
/// The longest decoy ID, in bytes. Bigman's IDs are UUIDs (36 bytes). The ID is
/// copied into every event, so it must be small.
pub const MAX_ID: usize = 64;
/// The longest decoy path, in bytes (PATH_MAX).
pub const MAX_PATH: usize = 4096;
/// The accounts that turned on account locking: one decimal UID per line.
/// Honoured only when root owns the file and nobody else can write it, so a
/// process running as the user cannot opt the user in.
pub const LOCK_ALLOWED: &str = "/etc/bigman-canary/lock-uids";
/// An opt-in file larger than this is ignored rather than read in part.
const LOCK_ALLOWED_MAX: u64 = 64 * 1024;

/// Whether root turned on account locking for `uid` in the opt-in file `path`
/// (normally [`LOCK_ALLOWED`]). A missing file, or one that is not a regular
/// file owned by root and writable only by root, allows nobody.
pub fn lock_allowed(path: &Path, uid: u32) -> bool {
    lock_allowed_owned_by(path, uid, 0)
}

/// [`lock_allowed`] with the UID that must own the file given, so tests that
/// cannot create a root-owned file can exercise the rest.
pub fn lock_allowed_owned_by(path: &Path, uid: u32, owner: u32) -> bool {
    // The checks and the read use one descriptor, so the file cannot be swapped
    // between them. O_NONBLOCK: opening a FIFO put there must not hang.
    let Ok(file) = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)
    else {
        return false;
    };
    let Ok(meta) = file.metadata() else {
        return false;
    };
    if !meta.is_file()
        || meta.uid() != owner
        || meta.mode() & 0o022 != 0
        || meta.len() > LOCK_ALLOWED_MAX
    {
        return false;
    }
    let mut text = String::new();
    if file
        .take(LOCK_ALLOWED_MAX)
        .read_to_string(&mut text)
        .is_err()
    {
        return false;
    }
    lists_uid(&text, uid)
}

/// Whether the opt-in text has a line that is exactly `uid` in decimal.
fn lists_uid(text: &str, uid: u32) -> bool {
    text.lines()
        .map(str::trim)
        .any(|l| !l.is_empty() && l.bytes().all(|b| b.is_ascii_digit()) && l.parse() == Ok(uid))
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WatchFile {
    pub id: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    /// Replace the set of decoys this connection watches.
    Watch { files: Vec<WatchFile> },
    /// Lock the connecting user's own account (password locked, account expired).
    LockAccount,
}

/// The process behind one access, read from /proc the moment the event arrived.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Reader {
    pub pid: u32,
    /// Real user ID of the process, when it was still running to be read.
    pub uid: Option<u32>,
    pub user: Option<String>,
    /// The audit login UID: who logged in, even through sudo. None when unset.
    pub login_uid: Option<u32>,
    /// The kernel's short process name.
    pub name: String,
    /// The executable path. Only shown for the watcher's own processes.
    pub exe: Option<String>,
    pub cmdline: String,
    /// Process start time in clock ticks since boot, so a later kill can check
    /// the PID was not reused.
    pub start_time: Option<u64>,
    /// Parent processes, nearest first, as "name (pid)".
    pub parents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum HelperMessage {
    /// The answer to a Watch: which decoys are now watched and why others are not.
    Watching {
        ids: Vec<String>,
        errors: Vec<String>,
    },
    /// One access to a watched decoy. `reader` is None when the kernel withheld
    /// the PID (the helper is not running as root).
    Event {
        id: String,
        mask: u64,
        reader: Option<Reader>,
    },
    /// Some accesses were not reported: the kernel queue overflowed, or the
    /// client was not reading and the helper dropped them rather than keep them.
    Lost,
    /// The answer to LockAccount.
    Locked { ok: bool, detail: String },
    /// A request the helper refused.
    Error { message: String },
}

/// One message as a JSON line.
pub(crate) fn encode<T: Serialize>(message: &T) -> std::io::Result<Vec<u8>> {
    let mut line = serde_json::to_vec(message)?;
    line.push(b'\n');
    Ok(line)
}

/// Writes one message as a JSON line.
pub fn send<T: Serialize>(stream: &mut impl Write, message: &T) -> std::io::Result<()> {
    stream.write_all(&encode(message)?)
}

/// Reads one JSON line, refusing lines over [`MAX_LINE`]. Ok(None) at end of stream.
pub fn receive<T: for<'de> Deserialize<'de>>(
    reader: &mut impl BufRead,
) -> std::io::Result<Option<T>> {
    let mut line = Vec::new();
    let n = reader
        .by_ref()
        .take(MAX_LINE as u64 + 1)
        .read_until(b'\n', &mut line)?;
    if n == 0 {
        return Ok(None);
    }
    if line.len() > MAX_LINE || line.last() != Some(&b'\n') {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Message too long or cut off",
        ));
    }
    serde_json::from_slice(&line)
        .map(Some)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

/// The monitor's side of a connection.
pub struct Client {
    writer: UnixStream,
    reader: BufReader<UnixStream>,
    pending: Vec<u8>,
}

impl Client {
    pub fn connect(socket: &Path) -> std::io::Result<Self> {
        let stream = UnixStream::connect(socket)?;
        stream.set_write_timeout(Some(Duration::from_secs(2)))?;
        let reader = BufReader::new(stream.try_clone()?);
        reader.get_ref().set_nonblocking(true)?;
        Ok(Self {
            writer: stream,
            reader,
            pending: Vec::new(),
        })
    }
    pub fn send(&mut self, message: &ClientMessage) -> std::io::Result<()> {
        send(&mut self.writer, message)
    }
    /// Every complete message that has arrived, without waiting. Err when the
    /// helper went away or broke the protocol.
    pub fn poll(&mut self) -> std::io::Result<Vec<HelperMessage>> {
        let mut out = vec![];
        loop {
            let mut chunk = Vec::new();
            match self.reader.read_until(b'\n', &mut chunk) {
                Ok(0) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "The canary helper closed the connection",
                    ))
                }
                Ok(_) => {
                    self.pending.extend_from_slice(&chunk);
                    if self.pending.last() == Some(&b'\n') {
                        let line = std::mem::take(&mut self.pending);
                        out.push(serde_json::from_slice(&line).map_err(|e| {
                            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
                        })?);
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // read_until keeps what it read before WouldBlock in `chunk`.
                    self.pending.extend_from_slice(&chunk);
                    break;
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            }
            if self.pending.len() > MAX_LINE {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Message too long",
                ));
            }
        }
        Ok(out)
    }
    /// Sends a request and waits up to `timeout` for a message `want` accepts;
    /// other messages that arrive meanwhile are returned too, in order.
    pub fn request(
        &mut self,
        message: &ClientMessage,
        timeout: Duration,
        want: impl Fn(&HelperMessage) -> bool,
    ) -> std::io::Result<(Option<HelperMessage>, Vec<HelperMessage>)> {
        self.send(message)?;
        let started = std::time::Instant::now();
        let mut others = vec![];
        while started.elapsed() < timeout {
            for m in self.poll()? {
                if want(&m) {
                    return Ok((Some(m), others));
                }
                others.push(m);
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        Ok((None, others))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_round_trip_as_tagged_json_lines() {
        let m = ClientMessage::Watch {
            files: vec![WatchFile {
                id: "a".into(),
                path: "/home/u/decoy.env".into(),
            }],
        };
        let mut buf = vec![];
        send(&mut buf, &m).unwrap();
        assert!(String::from_utf8_lossy(&buf).starts_with(r#"{"type":"watch""#));
        let back: ClientMessage = receive(&mut &buf[..]).unwrap().unwrap();
        assert_eq!(back, m);
    }

    #[test]
    fn the_opt_in_file_lists_whole_decimal_uids() {
        assert!(lists_uid("1000\n1001\n", 1001));
        assert!(lists_uid("  1000  \n", 1000));
        assert!(!lists_uid("10001\n100\n", 1000));
        assert!(!lists_uid("+1000\n1000 # ana\nuid=1000\n\n", 1000));
        assert!(!lists_uid("", 0));
    }

    #[test]
    fn the_opt_in_file_counts_only_when_its_owner_alone_can_write_it() {
        use std::os::unix::fs::PermissionsExt;
        let me = unsafe { libc::getuid() };
        let d = tempfile::tempdir().unwrap();
        let file = d.path().join("lock-uids");
        let mode = |m| std::fs::set_permissions(&file, std::fs::Permissions::from_mode(m)).unwrap();
        assert!(!lock_allowed_owned_by(&file, me, me), "a missing file");
        std::fs::write(&file, format!("{me}\n")).unwrap();
        mode(0o644);
        assert!(lock_allowed_owned_by(&file, me, me));
        assert!(!lock_allowed_owned_by(&file, me + 1, me), "an unlisted UID");
        if me != 0 {
            assert!(!lock_allowed(&file, me), "a file root does not own");
        }
        assert!(!lock_allowed_owned_by(&file, me, me + 1));
        mode(0o664);
        assert!(!lock_allowed_owned_by(&file, me, me), "group-writable");
        mode(0o646);
        assert!(!lock_allowed_owned_by(&file, me, me), "world-writable");
        mode(0o644);
        let link = d.path().join("link");
        std::os::unix::fs::symlink(&file, &link).unwrap();
        assert!(!lock_allowed_owned_by(&link, me, me), "a symlink");
        assert!(!lock_allowed_owned_by(d.path(), me, me), "a directory");
        std::fs::write(&file, format!("{me}\n").repeat(LOCK_ALLOWED_MAX as usize)).unwrap();
        assert!(!lock_allowed_owned_by(&file, me, me), "an oversized file");
    }

    #[test]
    fn an_endless_line_is_refused() {
        let long = vec![b'a'; MAX_LINE + 10];
        assert!(receive::<ClientMessage>(&mut &long[..]).is_err());
        assert!(receive::<ClientMessage>(&mut &b""[..]).unwrap().is_none());
    }
}
