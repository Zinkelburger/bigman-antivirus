//! A fanotify group that watches individual decoy files and says which decoy
//! each event was for and which process caused it.
//!
//! The group reports file identifiers (FAN_REPORT_FID) rather than open file
//! descriptors, so watching never opens a decoy and works the same with and
//! without root. Without root the kernel still delivers events but hides the
//! PID of every process except the listener's own.
use anyhow::{anyhow, bail, Context, Result};
use std::{
    ffi::CString,
    fs::File,
    io::Read,
    os::{
        fd::{AsRawFd, FromRawFd, OwnedFd},
        unix::{ffi::OsStrExt, fs::MetadataExt},
    },
    path::Path,
};

/// Every kind of access that matters for a decoy.
pub const MASK: u64 = libc::FAN_ACCESS
    | libc::FAN_OPEN
    | libc::FAN_MODIFY
    | libc::FAN_CLOSE_WRITE
    | libc::FAN_ATTRIB
    | libc::FAN_DELETE_SELF
    | libc::FAN_MOVE_SELF;

/// A marked file: what the kernel will send back to identify it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Mark {
    id: String,
    fsid: [i32; 2],
    handle_type: i32,
    handle: Vec<u8>,
}

/// One access to a marked file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub mask: u64,
    /// None when the kernel withheld the PID.
    pub pid: Option<u32>,
}

pub struct Group {
    file: File,
    marks: Vec<Mark>,
    buffer: Vec<u8>,
}

const HANDLE_MAX: usize = 128;
/// How much of the kernel queue one [`Group::events`] call takes: a few hundred
/// events at most, so what a connection holds does not grow with the queue.
pub const READ_BUFFER: usize = 16 * 1024;

/// The one answer to every refusal that depends on what is at `path`. Root
/// resolves the path, so saying why (no such file, a directory, someone else's)
/// would tell the peer about places it cannot look into itself.
fn refused(path: &Path) -> anyhow::Error {
    anyhow!(
        "Cannot watch {}: it must be an existing regular file that you own.",
        path.display()
    )
}

#[repr(C)]
struct Handle {
    bytes: u32,
    kind: i32,
    data: [u8; HANDLE_MAX],
}

impl Group {
    pub fn new() -> Result<Self> {
        let fd = unsafe {
            libc::fanotify_init(
                libc::FAN_CLASS_NOTIF
                    | libc::FAN_CLOEXEC
                    | libc::FAN_NONBLOCK
                    | libc::FAN_REPORT_FID,
                (libc::O_RDONLY | libc::O_LARGEFILE) as u32,
            )
        };
        if fd < 0 {
            return Err(std::io::Error::last_os_error()).context("fanotify is unavailable");
        }
        Ok(Self {
            file: File::from(unsafe { OwnedFd::from_raw_fd(fd) }),
            marks: vec![],
            buffer: vec![0; READ_BUFFER],
        })
    }

    pub fn as_raw_fd(&self) -> i32 {
        self.file.as_raw_fd()
    }

    /// Removes every mark. Marks on files that no longer exist are already gone.
    pub fn clear(&mut self) {
        unsafe {
            libc::fanotify_mark(
                self.file.as_raw_fd(),
                libc::FAN_MARK_FLUSH,
                0,
                libc::AT_FDCWD,
                std::ptr::null(),
            );
        }
        self.marks.clear();
    }

    /// Watches `path` when it is a regular file owned by `owner`. The last path
    /// component is never followed, and the ownership check and the mark apply
    /// to the same opened object, so a swapped path cannot redirect the mark.
    ///
    /// Until the ownership check has passed, every refusal is the same sentence
    /// (see [`refused`]). Later failures are about a file `owner` owns and may
    /// say what went wrong.
    pub fn watch(&mut self, id: &str, path: &Path, owner: u32) -> Result<()> {
        // The peer wrote the path, so this says nothing it does not know.
        if !path.is_absolute() {
            bail!("{} is not an absolute path", path.display());
        }
        let c = CString::new(path.as_os_str().as_bytes()).map_err(|_| refused(path))?;
        let fd = unsafe {
            libc::open(
                c.as_ptr(),
                libc::O_PATH | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            return Err(refused(path));
        }
        let object = File::from(unsafe { OwnedFd::from_raw_fd(fd) });
        let meta = object.metadata().map_err(|_| refused(path))?;
        if !meta.file_type().is_file() || meta.uid() != owner {
            return Err(refused(path));
        }
        let mut handle = Handle {
            bytes: HANDLE_MAX as u32,
            kind: 0,
            data: [0; HANDLE_MAX],
        };
        let mut mount_id = 0;
        let empty = CString::default();
        let r = unsafe {
            libc::name_to_handle_at(
                object.as_raw_fd(),
                empty.as_ptr(),
                (&mut handle as *mut Handle).cast(),
                &mut mount_id,
                libc::AT_EMPTY_PATH,
            )
        };
        if r != 0 {
            return Err(std::io::Error::last_os_error())
                .with_context(|| format!("Cannot identify {}", path.display()));
        }
        let mut fs: libc::statfs = unsafe { std::mem::zeroed() };
        if unsafe { libc::fstatfs(object.as_raw_fd(), &mut fs) } != 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        let fsid: [i32; 2] = unsafe { std::mem::transmute(fs.f_fsid) };
        // fanotify_mark cannot take an O_PATH descriptor alone; its /proc link
        // resolves to exactly the object opened above.
        let link = CString::new(format!("/proc/self/fd/{}", object.as_raw_fd()))?;
        let r = unsafe {
            libc::fanotify_mark(
                self.file.as_raw_fd(),
                libc::FAN_MARK_ADD,
                MASK,
                libc::AT_FDCWD,
                link.as_ptr(),
            )
        };
        if r != 0 {
            return Err(std::io::Error::last_os_error())
                .with_context(|| format!("Cannot watch {}", path.display()));
        }
        let handle_bytes = handle
            .data
            .get(..handle.bytes as usize)
            .context("The kernel returned an oversized file handle")?;
        self.marks.push(Mark {
            id: id.into(),
            fsid,
            handle_type: handle.kind,
            handle: handle_bytes.to_vec(),
        });
        Ok(())
    }

    /// The events from one read of the kernel queue, without blocking: at most
    /// [`READ_BUFFER`] bytes of them, and none when nothing is waiting. Call
    /// again for the rest; the queue is never drained into memory in one go.
    /// A queue overflow is reported as an event with FAN_Q_OVERFLOW and an
    /// empty id.
    pub fn events(&mut self) -> Result<Vec<Event>> {
        loop {
            return match self.file.read(&mut self.buffer) {
                Ok(size) => self.parse(&self.buffer[..size]),
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(vec![]),
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => Err(e.into()),
            };
        }
    }

    /// Parses one read's worth of events. Anything that is not laid out as this
    /// program expects ends the parse with an error: nothing here may index
    /// past the buffer, whatever lengths the records claim.
    fn parse(&self, buffer: &[u8]) -> Result<Vec<Event>> {
        let meta_size = std::mem::size_of::<libc::fanotify_event_metadata>();
        let mut out = vec![];
        let mut offset = 0;
        while buffer.len() - offset >= meta_size {
            let m: libc::fanotify_event_metadata =
                unsafe { std::ptr::read_unaligned(buffer.as_ptr().add(offset).cast()) };
            // A descriptor is only sent without FAN_REPORT_FID; never keep one.
            if m.fd >= 0 {
                unsafe { libc::close(m.fd) };
            }
            if m.vers != libc::FANOTIFY_METADATA_VERSION {
                bail!("Unsupported fanotify version {}", m.vers);
            }
            let len = m.event_len as usize;
            if m.metadata_len as usize != meta_size
                || len < meta_size
                || len > buffer.len() - offset
            {
                bail!("Malformed fanotify event");
            }
            let pid = u32::try_from(m.pid).ok().filter(|p| *p > 0);
            if m.mask & libc::FAN_Q_OVERFLOW != 0 {
                out.push(Event {
                    id: String::new(),
                    mask: m.mask,
                    pid,
                });
            } else if let Some(mark) = self.identify(&buffer[offset + meta_size..offset + len]) {
                out.push(Event {
                    id: mark.id.clone(),
                    mask: m.mask,
                    pid,
                });
            }
            offset += len;
        }
        Ok(out)
    }

    /// Finds the mark an event's file-identifier record names.
    fn identify(&self, mut info: &[u8]) -> Option<&Mark> {
        // header (4) + fsid (8) + handle_bytes (4) + handle_type (4)
        while info.len() >= 20 {
            let kind = info[0];
            let len = u16::from_ne_bytes([info[2], info[3]]) as usize;
            if len < 4 || len > info.len() {
                return None;
            }
            if kind == libc::FAN_EVENT_INFO_TYPE_FID && len >= 20 {
                let int = |at: usize| i32::from_ne_bytes(info[at..at + 4].try_into().unwrap());
                let fsid = [int(4), int(8)];
                let bytes = u32::from_ne_bytes(info[12..16].try_into().unwrap()) as usize;
                let handle_type = int(16);
                let handle = info[..len].get(20..20usize.checked_add(bytes)?)?;
                return self.marks.iter().find(|m| {
                    m.fsid == fsid && m.handle_type == handle_type && m.handle == handle
                });
            }
            info = &info[len..];
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn me() -> u32 {
        unsafe { libc::getuid() }
    }

    #[test]
    fn names_the_decoy_and_this_process_on_a_real_read() {
        let d = tempfile::tempdir().unwrap();
        let decoy = d.path().join("decoy.env");
        let other = d.path().join("other.env");
        std::fs::write(&decoy, "fake").unwrap();
        std::fs::write(&other, "fake").unwrap();
        let mut g = Group::new().unwrap();
        g.watch("decoy", &decoy, me()).unwrap();
        std::fs::read(&other).unwrap();
        std::fs::read(&decoy).unwrap();
        let events = g.events().unwrap();
        assert!(!events.is_empty());
        assert!(events.iter().all(|e| e.id == "decoy"), "{events:?}");
        assert!(events
            .iter()
            .any(|e| e.mask & libc::FAN_ACCESS != 0 && e.pid == Some(std::process::id())));
    }

    #[test]
    fn refuses_files_the_watcher_does_not_own_and_symlinks() {
        let d = tempfile::tempdir().unwrap();
        let decoy = d.path().join("decoy.env");
        std::fs::write(&decoy, "fake").unwrap();
        let link = d.path().join("link.env");
        std::os::unix::fs::symlink(&decoy, &link).unwrap();
        let mut g = Group::new().unwrap();
        assert!(g.watch("x", &decoy, me() + 1).is_err());
        assert!(g.watch("x", &link, me()).is_err());
        assert!(g.watch("x", d.path(), me()).is_err());
        assert!(g.watch("x", Path::new("relative.env"), me()).is_err());
        assert!(g.marks.is_empty());
        std::fs::set_permissions(&decoy, std::fs::Permissions::from_mode(0o600)).unwrap();
        g.watch("x", &decoy, me()).unwrap();
        g.clear();
        std::fs::read(&decoy).unwrap();
        assert!(g.events().unwrap().is_empty());
    }

    #[test]
    fn every_refusal_about_what_is_at_a_path_reads_the_same() {
        use std::os::unix::ffi::OsStringExt;
        let d = tempfile::tempdir().unwrap();
        let mine = d.path().join("mine.env");
        std::fs::write(&mine, "fake").unwrap();
        let link = d.path().join("link.env");
        std::os::unix::fs::symlink(&mine, &link).unwrap();
        let fifo = d.path().join("fifo");
        let c = CString::new(fifo.as_os_str().as_bytes()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(c.as_ptr(), 0o600) }, 0);
        let nul = std::path::PathBuf::from(std::ffi::OsString::from_vec(b"/tmp/a\0b".to_vec()));
        let mut g = Group::new().unwrap();
        let cases: Vec<(std::path::PathBuf, u32)> = vec![
            (d.path().join("missing.env"), me()),
            (d.path().join("missing-dir/x.env"), me()),
            (mine.join("below-a-file"), me()),
            (d.path().to_path_buf(), me()),
            (link, me()),
            (fifo, me()),
            (nul, me()),
            // Someone else's file, seen from the other side: this file, another watcher.
            (mine.clone(), me() + 1),
            // Root's files, where the test can see them.
            ("/etc/passwd".into(), me().max(1)),
            ("/root/.bashrc".into(), me().max(1)),
        ];
        for (path, owner) in cases {
            let e = g.watch("x", &path, owner).unwrap_err();
            assert_eq!(
                format!("{e:#}"),
                format!(
                    "Cannot watch {}: it must be an existing regular file that you own.",
                    path.display()
                )
            );
        }
        assert!(g.marks.is_empty());
    }

    fn metadata(event_len: u32, metadata_len: u16) -> Vec<u8> {
        let m = libc::fanotify_event_metadata {
            event_len,
            vers: libc::FANOTIFY_METADATA_VERSION,
            reserved: 0,
            metadata_len,
            mask: libc::FAN_ACCESS,
            fd: libc::FAN_NOFD,
            pid: 1,
        };
        let size = std::mem::size_of::<libc::fanotify_event_metadata>();
        let mut out = vec![0u8; size];
        unsafe { std::ptr::write_unaligned(out.as_mut_ptr().cast(), m) };
        out
    }

    #[test]
    fn a_malformed_event_is_an_error_not_a_panic() {
        let g = Group::new().unwrap();
        let size = std::mem::size_of::<libc::fanotify_event_metadata>() as u32;
        // A well-formed event for a file nobody marked is skipped.
        assert!(g.parse(&metadata(size, size as u16)).unwrap().is_empty());
        let mut two = metadata(size, size as u16);
        two.extend(metadata(size, size as u16));
        assert!(g.parse(&two).unwrap().is_empty());
        // metadata_len past the event, short of the header, or not this layout.
        for bad in [u16::MAX, 0, size as u16 - 4, size as u16 + 4] {
            let mut b = metadata(size + 8, bad);
            b.extend([0u8; 8]);
            assert!(g.parse(&b).is_err(), "metadata_len {bad}");
        }
        // event_len past the buffer, or shorter than its own header.
        assert!(g.parse(&metadata(size + 1, size as u16)).is_err());
        assert!(g.parse(&metadata(u32::MAX, size as u16)).is_err());
        assert!(g.parse(&metadata(size - 1, size as u16)).is_err());
        assert!(g.parse(&metadata(0, size as u16)).is_err());
        let mut wrong_version = metadata(size, size as u16);
        wrong_version[4] ^= 0xff;
        assert!(g.parse(&wrong_version).is_err());
        // Information records whose lengths lie are ignored, not followed.
        for record in [
            [libc::FAN_EVENT_INFO_TYPE_FID, 0, 0xff, 0xff],
            [libc::FAN_EVENT_INFO_TYPE_FID, 0, 0, 0],
            [libc::FAN_EVENT_INFO_TYPE_FID, 0, 24, 0],
        ] {
            let mut b = metadata(size + 24, size as u16);
            b.extend(record);
            // fsid, then a handle length far past the record.
            b.extend([0u8; 8]);
            b.extend(u32::MAX.to_ne_bytes());
            b.extend([0u8; 8]);
            assert!(g.parse(&b).unwrap().is_empty());
        }
    }

    #[test]
    fn one_call_takes_one_buffer_and_the_rest_waits() {
        let d = tempfile::tempdir().unwrap();
        let mut g = Group::new().unwrap();
        // The kernel merges repeated events for one file, so many files it is:
        // far more events than one buffer holds.
        const FILES: usize = 1000;
        let files: Vec<_> = (0..FILES)
            .map(|n| d.path().join(format!("decoy-{n}.env")))
            .collect();
        for (n, f) in files.iter().enumerate() {
            std::fs::write(f, "fake").unwrap();
            g.watch(&n.to_string(), f, me()).unwrap();
        }
        for f in &files {
            std::fs::read(f).unwrap();
        }
        let smallest = std::mem::size_of::<libc::fanotify_event_metadata>();
        let mut seen = std::collections::HashSet::new();
        let mut reads = 0;
        loop {
            let batch = g.events().unwrap();
            if batch.is_empty() {
                break;
            }
            assert!(batch.len() <= READ_BUFFER / smallest);
            seen.extend(batch.into_iter().map(|e| e.id));
            reads += 1;
        }
        assert!(reads > 1, "every event came in one read");
        assert_eq!(seen.len(), FILES);
    }
}
