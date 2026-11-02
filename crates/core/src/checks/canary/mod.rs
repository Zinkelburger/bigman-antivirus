use crate::{
    model::{now, Report, Severity},
    runner::{self, Cancel},
    store::{read_file, Store, MAX_INPUT},
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    ffi::CString,
    fs,
    io::{Read, Write},
    os::{
        fd::{AsRawFd, FromRawFd},
        unix::{ffi::OsStrExt, fs::OpenOptionsExt},
    },
    path::{Path, PathBuf},
    sync::atomic::Ordering,
    time::{Duration, Instant},
};
use ts_rs::TS;
pub mod setup;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Canary {
    pub id: String,
    pub path: PathBuf,
    pub created: u64,
    pub kind: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export, rename = "MonitorHealth")]
pub struct Health {
    pub updated: u64,
    pub pid: u32,
    pub watched: usize,
    pub configured: usize,
    pub errors: Vec<String>,
    #[serde(default)]
    pub watched_ids: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReadReceipt {
    pub id: String,
    pub canary_id: String,
    pub monitor_pid: u32,
    pub created: u64,
}
/// Registered decoys, the monitor heartbeat and the suggested path for a new one.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CanaryState {
    pub canaries: Vec<Canary>,
    pub monitor: Option<Health>,
    pub default_path: PathBuf,
}

pub fn state(store: &Store) -> Result<CanaryState> {
    Ok(CanaryState {
        canaries: list(store)?,
        monitor: setup::health(store),
        default_path: setup::default_path(),
    })
}

pub fn list(store: &Store) -> Result<Vec<Canary>> {
    if !store.root.join("canaries.json").exists() {
        return Ok(vec![]);
    }
    store.read("canaries.json")
}
pub fn create(store: &Store, path: &Path, remote_url: Option<&str>) -> Result<Canary> {
    let _lock = store.lock("canaries.lock")?;
    let parent = fs::canonicalize(
        path.parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new(".")),
    )?;
    let p = parent.join(path.file_name().context("Choose a filename")?);
    if p.starts_with(&store.root) && !p.starts_with(store.root.join("canaries")) {
        bail!("Choose a decoy path outside application metadata");
    }
    let id = uuid::Uuid::new_v4().to_string();
    let (kind, contents) = if let Some(link) = remote_url {
        let u = url::Url::parse(link)?;
        if u.scheme() != "https" || !u.username().is_empty() || u.password().is_some() {
            bail!("Use an HTTPS token URL without embedded credentials");
        }
        ("hosted-url", format!("[InternetShortcut]\nURL={u}\n"))
    } else {
        ("local-decoy",format!("# Synthetic Bigman canary. Not a functioning credential.\nBIGMAN_CANARY_ID={id}\nDECOY_API_TOKEN=bigman_decoy_{}\n",uuid::Uuid::new_v4()))
    };
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(&p)
        .context("Canaries never overwrite an existing file")?;
    file.write_all(contents.as_bytes())?;
    file.sync_all()?;
    let c = Canary {
        id,
        path: p,
        created: now(),
        kind: kind.into(),
    };
    let mut entries = list(store)?;
    entries.push(c.clone());
    store.write("canaries.json", &entries)?;
    Ok(c)
}
pub fn disarm(store: &Store, id: &str) -> Result<()> {
    let _lock = store.lock("canaries.lock")?;
    let mut entries = list(store)?;
    let old = entries.len();
    entries.retain(|c| c.id != id);
    if old == entries.len() {
        bail!("Unknown canary ID");
    }
    store.write("canaries.json", &entries)?;
    Ok(())
}
pub fn self_test(store: &Store, id: &str) -> Result<()> {
    let entries = list(store)?;
    let c = entries
        .iter()
        .find(|c| c.id == id)
        .context("Unknown canary ID")?;
    let _ = read_file(&c.path, MAX_INPUT)?;
    Ok(())
}
pub fn notify(summary: &str, body: &str) {
    if runner::available("notify-send") {
        let _ = runner::run(
            std::process::Command::new("notify-send").args([
                "--app-name=Bigman",
                "--",
                summary,
                body,
            ]),
            3,
            &runner::cancel(),
        );
    }
}

pub struct Watcher {
    file: fs::File,
    watches: HashMap<i32, PathBuf>,
}
impl Watcher {
    pub fn new() -> Result<Self> {
        let fd = unsafe { libc::inotify_init1(libc::IN_NONBLOCK | libc::IN_CLOEXEC) };
        if fd < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        Ok(Self {
            file: unsafe { fs::File::from_raw_fd(fd) },
            watches: HashMap::new(),
        })
    }
    pub fn configure(&mut self, entries: &[Canary]) -> Vec<String> {
        let parents: BTreeMap<PathBuf, ()> = entries
            .iter()
            .filter_map(|c| c.path.parent().map(|p| (p.to_path_buf(), ())))
            .collect();
        let obsolete: Vec<_> = self
            .watches
            .iter()
            .filter(|(_, p)| !parents.contains_key(*p))
            .map(|(wd, _)| *wd)
            .collect();
        for wd in obsolete {
            unsafe {
                libc::inotify_rm_watch(self.file.as_raw_fd(), wd);
            };
            self.watches.remove(&wd);
        }
        let mut errors = vec![];
        for p in parents.keys() {
            if self.watches.values().any(|v| v == p) {
                continue;
            }
            let c = match CString::new(p.as_os_str().as_bytes()) {
                Ok(c) => c,
                Err(e) => {
                    errors.push(e.to_string());
                    continue;
                }
            };
            let mask = libc::IN_ACCESS
                | libc::IN_MODIFY
                | libc::IN_ATTRIB
                | libc::IN_DELETE
                | libc::IN_MOVED_FROM
                | libc::IN_MOVED_TO
                | libc::IN_CREATE
                | libc::IN_DELETE_SELF
                | libc::IN_MOVE_SELF
                | libc::IN_ONLYDIR
                | libc::IN_DONT_FOLLOW;
            let wd = unsafe { libc::inotify_add_watch(self.file.as_raw_fd(), c.as_ptr(), mask) };
            if wd < 0 {
                errors.push(format!(
                    "Cannot watch {}: {}",
                    p.display(),
                    std::io::Error::last_os_error()
                ));
            } else {
                self.watches.insert(wd, p.clone());
            }
        }
        errors
    }
    pub fn events(&mut self) -> Result<Vec<(Option<PathBuf>, u32)>> {
        let mut buffer = [0u8; 65536];
        let mut out = vec![];
        loop {
            let size = match self.file.read(&mut buffer) {
                Ok(n) => n,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e.into()),
            };
            if size == 0 {
                break;
            }
            let mut offset = 0;
            while offset + std::mem::size_of::<libc::inotify_event>() <= size {
                let event = unsafe {
                    std::ptr::read_unaligned(
                        buffer.as_ptr().add(offset).cast::<libc::inotify_event>(),
                    )
                };
                let start = offset + std::mem::size_of::<libc::inotify_event>();
                let end = start + event.len as usize;
                if end > size {
                    bail!("Truncated inotify event");
                }
                let name = &buffer[start..end];
                let name = &name[..name.iter().position(|b| *b == 0).unwrap_or(name.len())];
                let path = self.watches.get(&event.wd).map(|p| {
                    if name.is_empty() {
                        p.clone()
                    } else {
                        p.join(std::ffi::OsStr::from_bytes(name))
                    }
                });
                if event.mask & (libc::IN_IGNORED | libc::IN_MOVE_SELF | libc::IN_DELETE_SELF) != 0
                {
                    if event.mask & libc::IN_IGNORED == 0 {
                        unsafe {
                            libc::inotify_rm_watch(self.file.as_raw_fd(), event.wd);
                        }
                    }
                    self.watches.remove(&event.wd);
                }
                out.push((path, event.mask));
                offset = end;
            }
        }
        Ok(out)
    }
}
pub fn watch(store: &Store, stop: &Cancel) -> Result<()> {
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(store.root.join("monitor.lock"))?;
    fs2::FileExt::try_lock_exclusive(&lock).context("A monitor is already running")?;
    let mut watcher = Watcher::new()?;
    let mut last: HashMap<String, Instant> = HashMap::new();
    let mut persist = Instant::now();
    let startup = Report::new("Monitor started", "Canary and persistence monitoring");
    store.save(&startup)?;
    let result = (|| -> Result<()> {
        while !stop.load(Ordering::Relaxed) {
            let entries = list(store)?;
            let mut errors = watcher.configure(&entries);
            for c in &entries {
                if !fs::symlink_metadata(&c.path).is_ok_and(|m| m.is_file()) {
                    errors.push(format!(
                        "Canary missing or replaced by a non-regular file: {}",
                        c.path.display()
                    ));
                }
            }
            store.write(
                "monitor-health.json",
                &Health {
                    updated: now(),
                    pid: std::process::id(),
                    watched: entries
                        .iter()
                        .filter(|c| {
                            c.path
                                .parent()
                                .is_some_and(|p| watcher.watches.values().any(|v| v == p))
                        })
                        .count(),
                    configured: entries.len(),
                    errors,
                    watched_ids: entries
                        .iter()
                        .filter(|c| {
                            fs::symlink_metadata(&c.path).is_ok_and(|m| m.is_file())
                                && c.path
                                    .parent()
                                    .is_some_and(|p| watcher.watches.values().any(|v| v == p))
                        })
                        .map(|c| c.id.clone())
                        .collect(),
                },
            )?;
            for (path, mask) in watcher.events()? {
                if mask & libc::IN_Q_OVERFLOW != 0 {
                    let mut r = Report::new("Monitor event loss", "inotify queue");
                    r.errors.push(
                        "Kernel event queue overflowed. Some access events were lost.".into(),
                    );
                    store.save(&r)?;
                    notify(
                        "Bigman monitoring incomplete",
                        "Kernel event queue overflowed",
                    );
                    continue;
                }
                let Some(path) = path else {
                    continue;
                };
                for c in entries
                    .iter()
                    .filter(|c| c.path == path || c.path.parent() == Some(path.as_path()))
                {
                    // A receipt proves a new kernel read event, even when notifications are debounced.
                    if c.path == path && mask & libc::IN_ACCESS != 0 {
                        store.write(
                            &format!("canaries/{}.read.json", c.id),
                            &ReadReceipt {
                                id: uuid::Uuid::new_v4().to_string(),
                                canary_id: c.id.clone(),
                                monitor_pid: std::process::id(),
                                created: now(),
                            },
                        )?;
                    }
                    if last
                        .get(&c.id)
                        .is_some_and(|t| t.elapsed() < Duration::from_secs(3))
                    {
                        continue;
                    }
                    last.insert(c.id.clone(), Instant::now());
                    let mut kinds = vec![];
                    if mask & libc::IN_ACCESS != 0 {
                        kinds.push("content read");
                    }
                    if mask & libc::IN_MODIFY != 0 {
                        kinds.push("content changed");
                    }
                    if mask & libc::IN_ATTRIB != 0 {
                        kinds.push("metadata changed");
                    }
                    if mask
                        & (libc::IN_DELETE
                            | libc::IN_DELETE_SELF
                            | libc::IN_MOVED_FROM
                            | libc::IN_MOVE_SELF)
                        != 0
                    {
                        kinds.push("removed or moved");
                    }
                    if mask & (libc::IN_CREATE | libc::IN_MOVED_TO) != 0 {
                        kinds.push("created or replaced");
                    }
                    if kinds.is_empty() {
                        continue;
                    }
                    let mut r = Report::new("Canary triggered", c.path.display());
                    r.add("canary.event",Severity::High,"A decoy file was accessed or changed",format!("Canary {}: {}",c.id,kinds.join(", ")),"Check recent activity. Backup/indexing tools and the Test button can also trigger decoys. inotify cannot identify the responsible process.");
                    r.coverage.push("Local events while monitor is running only; not access prevention, reader attribution, or proof of exfiltration. Repeated events within three seconds are grouped.".into());
                    store.save(&r)?;
                    notify(
                        "Bigman canary triggered",
                        &format!("{}: {}", c.path.display(), kinds.join(", ")),
                    );
                }
            }
            if persist.elapsed() > Duration::from_secs(300) {
                let r = crate::checks::changes::changes(store)?;
                let fingerprint =
                    crate::store::digest(&serde_json::to_vec(&(&r.findings, &r.errors))?);
                let previous = store
                    .read::<String>("last-change-fingerprint.json")
                    .unwrap_or_default();
                if previous != fingerprint {
                    if !r.findings.is_empty() || !r.errors.is_empty() {
                        store.save(&r)?;
                    }
                    if r.findings.iter().any(|f| f.severity != Severity::Info)
                        || !r.errors.is_empty()
                    {
                        notify("Bigman system changes", r.status());
                    }
                    store.write("last-change-fingerprint.json", &fingerprint)?;
                }
                persist = Instant::now();
            }
            std::thread::sleep(Duration::from_millis(500));
        }
        Ok(())
    })();
    let _ = fs::remove_file(store.root.join("monitor-health.json"));
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reattaches_after_parent_directory_replacement() {
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        let parent = d.path().join("documents");
        fs::create_dir(&parent).unwrap();
        let c = create(&store, &parent.join("decoy.env"), None).unwrap();
        let mut watcher = Watcher::new().unwrap();
        assert!(watcher.configure(std::slice::from_ref(&c)).is_empty());
        fs::rename(&parent, d.path().join("old-documents")).unwrap();
        fs::create_dir(&parent).unwrap();
        fs::write(&c.path, "replacement fixture").unwrap();
        watcher.events().unwrap();
        assert!(watcher.configure(std::slice::from_ref(&c)).is_empty());
        self_test(&store, &c.id).unwrap();
        assert!(watcher
            .events()
            .unwrap()
            .iter()
            .any(|(path, mask)| path.as_ref() == Some(&c.path) && mask & libc::IN_ACCESS != 0));
    }
    #[test]
    fn real_kernel_read_event_and_no_overwrite() {
        let d = tempfile::tempdir().unwrap();
        let s = Store::open(Some(d.path().join("state"))).unwrap();
        let p = d.path().join("decoy.env");
        let c = create(&s, &p, None).unwrap();
        assert!(create(&s, &p, None).is_err());
        let mut w = Watcher::new().unwrap();
        assert!(w.configure(std::slice::from_ref(&c)).is_empty());
        self_test(&s, &c.id).unwrap();
        let events = w.events().unwrap();
        assert!(events
            .iter()
            .any(|(p, m)| p.as_ref() == Some(&c.path) && m & libc::IN_ACCESS != 0));
        disarm(&s, &c.id).unwrap();
        assert!(p.exists());
        assert!(list(&s).unwrap().is_empty());
    }
}
