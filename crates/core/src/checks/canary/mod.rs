use crate::{
    model::{now, Report, Severity},
    runner::{self, Cancel},
    store::{read_file, Store, MAX_INPUT},
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    ffi::CString,
    fs,
    io::{Read, Write},
    os::{
        fd::{AsRawFd, FromRawFd},
        unix::{
            ffi::OsStrExt,
            fs::{MetadataExt, OpenOptionsExt},
        },
    },
    path::{Path, PathBuf},
    sync::atomic::Ordering,
    time::{Duration, Instant},
};
use ts_rs::TS;
pub mod helper;
pub mod respond;
pub mod setup;
pub use respond::{Access, Reader, Responses};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Canary {
    pub id: String,
    pub path: PathBuf,
    pub created: u64,
    pub kind: String,
    /// What happens beyond logging and a notification when the decoy is touched.
    #[serde(default)]
    pub responses: Responses,
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
    /// The reader-identification helper, as the monitor last saw it.
    #[serde(default)]
    pub helper: HelperHealth,
}
/// The monitor's link to the optional root helper that names readers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct HelperHealth {
    pub connected: bool,
    /// Decoys the helper confirmed it watches.
    pub watched_ids: Vec<String>,
    pub errors: Vec<String>,
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
    /// The root helper's system unit is installed.
    pub helper_installed: bool,
    /// Why "lock your account" cannot work here, when it cannot.
    pub lock_unavailable: Option<String>,
    /// What locking this account costs here beyond what the choice says, when
    /// you are the only one who could unlock it.
    pub lock_warning: Option<String>,
    /// An administrator has let the helper lock this account. Saving a decoy
    /// with "lock your account" asks for that once.
    pub lock_allowed: bool,
}

pub fn state(store: &Store) -> Result<CanaryState> {
    Ok(CanaryState {
        canaries: list(store)?,
        monitor: setup::health(store),
        default_path: setup::default_path(),
        helper_installed: Path::new(bigman_canary_helper::UNIT_PATH).exists(),
        lock_unavailable: lock_unavailable(Path::new("/etc/passwd"), unsafe { libc::getuid() }),
        lock_warning: lock_warning_here(),
        lock_allowed: helper::lock_allowed(),
    })
}

/// The warning for this account on this computer, when locking can work at all.
pub fn lock_warning_here() -> Option<String> {
    let uid = unsafe { libc::getuid() };
    let passwd = Path::new("/etc/passwd");
    lock_unavailable(passwd, uid)
        .is_none()
        .then(|| lock_warning(passwd, Path::new("/etc/group"), uid))
        .flatten()
}

/// A locked account cannot use sudo or answer a password prompt either. When
/// this account is the only member of the administrators' group (wheel or
/// sudo), nobody is left to unlock it from inside the running system.
pub fn lock_warning(passwd: &Path, group: &Path, uid: u32) -> Option<String> {
    let name = bigman_canary_helper::server::local_user(passwd, uid)?;
    let groups = fs::read_to_string(group).ok()?;
    let mut admins = BTreeSet::new();
    let mut gids = vec![];
    for fields in groups.lines().map(|l| l.split(':').collect::<Vec<_>>()) {
        if fields.len() >= 4 && matches!(fields[0], "wheel" | "sudo") {
            gids.push(fields[2].to_string());
            admins.extend(
                fields[3]
                    .split(',')
                    .filter(|m| !m.is_empty())
                    .map(str::to_string),
            );
        }
    }
    // Members through their primary group are not in the group's own list.
    for fields in fs::read_to_string(passwd)
        .ok()?
        .lines()
        .map(|l| l.split(':').collect::<Vec<_>>())
    {
        if fields.len() >= 4 && gids.iter().any(|g| g == fields[3]) {
            admins.insert(fields[0].to_string());
        }
    }
    admins.remove("root");
    (admins.len() == 1 && admins.contains(&name)).then(|| {
        "You are this computer's only administrator. Unless root has a password of its own, unlocking the account then means starting the computer from a rescue system.".to_string()
    })
}

/// Account locking uses usermod, which only works on local accounts.
pub fn lock_unavailable(passwd: &Path, uid: u32) -> Option<String> {
    let defs = Path::new("/etc/login.defs");
    if uid < bigman_canary_helper::server::uid_min(defs)
        || uid > bigman_canary_helper::server::uid_max(defs)
    {
        return Some("Your account is a system account, which Bigman never locks.".into());
    }
    if bigman_canary_helper::server::local_user(passwd, uid).is_none() {
        return Some("Your account comes from a directory service (such as SSSD, LDAP or Active Directory), not this computer's /etc/passwd, so it can only be locked there.".into());
    }
    None
}

pub fn list(store: &Store) -> Result<Vec<Canary>> {
    if !store.root.join("canaries.json").exists() {
        return Ok(vec![]);
    }
    store.read("canaries.json")
}
pub fn create(store: &Store, path: &Path, remote_url: Option<&str>) -> Result<Canary> {
    create_with(store, path, remote_url, Responses::default())
}
pub fn create_with(
    store: &Store,
    path: &Path,
    remote_url: Option<&str>,
    responses: Responses,
) -> Result<Canary> {
    responses.validate()?;
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
        responses,
    };
    let mut entries = list(store)?;
    entries.push(c.clone());
    store.write("canaries.json", &entries)?;
    Ok(c)
}
/// Replaces a decoy's responses. The monitor picks them up within a second.
pub fn set_responses(store: &Store, id: &str, responses: Responses) -> Result<Canary> {
    responses.validate()?;
    let _lock = store.lock("canaries.lock")?;
    let mut entries = list(store)?;
    let c = entries
        .iter_mut()
        .find(|c| c.id == id)
        .context("Unknown canary ID")?;
    c.responses = responses;
    let out = c.clone();
    store.write("canaries.json", &entries)?;
    Ok(out)
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
    // Never from this crate's own tests: they run the real monitor loop, and a
    // test run must not post to the desktop of whoever is sitting at it.
    if !cfg!(test) && runner::available("notify-send") {
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
    /// Watches one directory with `mask`, for watchers other than the canaries'.
    pub fn add_dir(&mut self, dir: &Path, mask: u32) -> Result<()> {
        let c = CString::new(dir.as_os_str().as_bytes())?;
        let wd = unsafe { libc::inotify_add_watch(self.file.as_raw_fd(), c.as_ptr(), mask) };
        if wd < 0 {
            bail!(
                "Cannot watch {}: {}",
                dir.display(),
                std::io::Error::last_os_error()
            );
        }
        self.watches.insert(wd, dir.to_path_buf());
        Ok(())
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
/// What an inotify event mask means for a decoy, in plain words.
pub fn event_kinds(mask: u32) -> Vec<&'static str> {
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
    if mask & (libc::IN_DELETE | libc::IN_DELETE_SELF | libc::IN_MOVED_FROM | libc::IN_MOVE_SELF)
        != 0
    {
        kinds.push("removed or moved");
    }
    if mask & (libc::IN_CREATE | libc::IN_MOVED_TO) != 0 {
        kinds.push("created or replaced");
    }
    kinds
}

/// The canary rule: a decoy was touched while the monitor ran.
pub fn event_finding(r: &mut Report, canary_id: &str, kinds: &[&str], readers: &[Reader]) {
    let mut evidence = format!("Canary {}: {}", canary_id, kinds.join(", "));
    for reader in readers {
        evidence.push_str(&format!("\nBy {}", reader.label()));
        if !reader.cmdline.is_empty() {
            evidence.push_str(&format!(": {}", reader.cmdline));
        }
        if !reader.parents.is_empty() {
            evidence.push_str(&format!("\n  started by {}", reader.parents.join(" < ")));
        }
    }
    let advice = if readers.is_empty() {
        "Check recent activity. Backup/indexing tools and the Test button can also trigger decoys. Without reader identification, inotify cannot say which process did it."
    } else {
        "Check whether you recognize the process. Backup/indexing tools and the Test button can also trigger decoys."
    };
    r.add(
        "canary.event",
        Severity::High,
        "A decoy file was accessed or changed",
        evidence,
        advice,
    );
}

/// A decoy the helper was told to watch, with its device and inode then.
type Marked = (bigman_canary_helper::WatchFile, Option<(u64, u64)>);

/// The monitor's connection to the optional root helper. Absent or broken, the
/// monitor works as before and accesses are logged without a reader.
struct HelperLink {
    socket: PathBuf,
    client: Option<bigman_canary_helper::Client>,
    tried: Option<Instant>,
    /// What the helper was last told to watch, with each file's device and
    /// inode: its mark is on the inode, so a replaced decoy needs a new one.
    sent: Option<Vec<Marked>>,
    health: HelperHealth,
    /// Readers reported recently, per decoy, waiting for inotify's event.
    readers: Vec<(Instant, String, Reader)>,
    lost: bool,
}

/// How long a reader report waits for the matching inotify event.
const READER_WINDOW: Duration = Duration::from_secs(4);

impl HelperLink {
    fn new(socket: PathBuf) -> Self {
        Self {
            socket,
            client: None,
            tried: None,
            sent: None,
            health: HelperHealth::default(),
            readers: vec![],
            lost: false,
        }
    }
    fn drop_link(&mut self, error: impl std::fmt::Display) {
        self.client = None;
        self.sent = None;
        self.health = HelperHealth {
            errors: vec![format!("Lost the reader-identification helper: {error}")],
            ..HelperHealth::default()
        };
    }
    /// Connects when the helper is there (retrying every ten seconds), tells it
    /// the current decoys, and collects what it reported.
    fn sync(&mut self, entries: &[Canary]) {
        if self.client.is_none() {
            if self
                .tried
                .is_some_and(|t| t.elapsed() < Duration::from_secs(10))
            {
                return;
            }
            self.tried = Some(Instant::now());
            match bigman_canary_helper::Client::connect(&self.socket) {
                Ok(c) => self.client = Some(c),
                Err(e) => {
                    self.health = HelperHealth::default();
                    if self.socket.exists() {
                        self.health.errors = vec![format!(
                            "Cannot reach the reader-identification helper: {e}"
                        )];
                    }
                    return;
                }
            }
        }
        let files: Vec<_> = entries
            .iter()
            .filter(|c| c.kind == "local-decoy")
            .map(|c| {
                let inode = fs::symlink_metadata(&c.path)
                    .ok()
                    .filter(|m| m.is_file())
                    .map(|m| (m.dev(), m.ino()));
                let file = bigman_canary_helper::WatchFile {
                    id: c.id.clone(),
                    path: c.path.clone(),
                };
                (file, inode)
            })
            .collect();
        // Sent again when a decoy was replaced (an atomic save, a restore, a
        // copy moved over it), or reader identification would silently end.
        if self.sent.as_ref() != Some(&files) {
            let message = bigman_canary_helper::ClientMessage::Watch {
                files: files.iter().map(|(f, _)| f.clone()).collect(),
            };
            if let Err(e) = self.client.as_mut().map_or(Ok(()), |c| c.send(&message)) {
                return self.drop_link(e);
            }
            self.sent = Some(files);
        }
        self.poll();
    }
    fn poll(&mut self) {
        use bigman_canary_helper::HelperMessage;
        let Some(client) = self.client.as_mut() else {
            return;
        };
        match client.poll() {
            Ok(messages) => {
                for m in messages {
                    match m {
                        HelperMessage::Watching { ids, errors } => {
                            self.health = HelperHealth {
                                connected: true,
                                watched_ids: ids,
                                errors,
                            }
                        }
                        HelperMessage::Event {
                            id,
                            reader: Some(r),
                            ..
                        } => self.readers.push((Instant::now(), id, r.into())),
                        HelperMessage::Event { .. } | HelperMessage::Locked { .. } => {}
                        HelperMessage::Lost => self.lost = true,
                        HelperMessage::Error { message } => self.health.errors.push(message),
                    }
                }
            }
            Err(e) => self.drop_link(e),
        }
        self.readers.retain(|(t, ..)| t.elapsed() < READER_WINDOW);
    }
    /// The processes reported for decoy `id`, waiting briefly for the helper's
    /// report when it watches that decoy. Each is returned once.
    fn take_readers(&mut self, id: &str) -> Vec<Reader> {
        // Read what the helper has sent first: an older report for this decoy
        // must not stand in for the reader of this access.
        self.poll();
        if self.health.watched_ids.iter().any(|w| w == id) {
            let started = Instant::now();
            while self.client.is_some()
                && !self.readers.iter().any(|(_, i, _)| i == id)
                && started.elapsed() < Duration::from_millis(300)
            {
                std::thread::sleep(Duration::from_millis(20));
                self.poll();
            }
        }
        let mut out: Vec<Reader> = vec![];
        self.readers.retain(|(_, i, r)| {
            if i != id {
                return true;
            }
            if !out.iter().any(|o| o.pid == r.pid) {
                out.push(r.clone());
            }
            false
        });
        out
    }
    /// Drops the reports for an access that was grouped with an earlier one, so
    /// they are not taken for the readers of the next.
    fn discard_readers(&mut self, id: &str) {
        self.poll();
        self.readers.retain(|(_, i, _)| i != id);
    }
}

/// True when an inotify event on `path` concerns decoy `c`. Events on the
/// folder itself only count when the folder is moved or deleted: listing a
/// folder (ls) reports a read of the folder, not of the decoy. A file manager
/// or file chooser showing the folder is different: it opens files whose name
/// does not say what they are, the decoy among them, and that is a read.
fn concerns(c: &Canary, path: &Path, mask: u32) -> bool {
    c.path == path
        || (c.path.parent() == Some(path)
            && mask & (libc::IN_DELETE_SELF | libc::IN_MOVE_SELF) != 0)
}

/// How long after a decoy's responses ran they are held back for it, so one
/// burst of reads runs a script or logs out once.
const RESPONSE_COOLDOWN: Duration = Duration::from_secs(60);

pub fn watch(store: &Store, stop: &Cancel) -> Result<()> {
    watch_with(store, stop, respond::Commands::default())
}

pub fn watch_with(store: &Store, stop: &Cancel, commands: respond::Commands) -> Result<()> {
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
    let mut helper = HelperLink::new(commands.helper_socket.clone());
    let commands = std::sync::Arc::new(commands);
    let mut last: HashMap<String, Instant> = HashMap::new();
    let mut responded: HashMap<String, Instant> = HashMap::new();
    // The decoys the previous pass knew. Events for one that is new this pass
    // are its own creation, queued before it was registered: not a touch.
    let mut known: BTreeSet<String> = BTreeSet::new();
    let mut persist = Instant::now();
    let checking = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let startup = Report::new("Monitor started", "Canary and persistence monitoring");
    store.save(&startup)?;
    let result = (|| -> Result<()> {
        while !stop.load(Ordering::Relaxed) {
            let entries = list(store)?;
            let mut errors = watcher.configure(&entries);
            helper.sync(&entries);
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
                    helper: helper.health.clone(),
                },
            )?;
            if std::mem::take(&mut helper.lost) {
                let mut r = Report::new("Monitor event loss", "Reader identification");
                r.errors.push(
                    "The reader-identification queue overflowed. Some readers were not named."
                        .into(),
                );
                store.save(&r)?;
            }
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
                    .filter(|c| known.contains(&c.id) && concerns(c, &path, mask))
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
                        // Grouped with the access just logged. What belongs to
                        // it is used up with it: a test read's marker and the
                        // helper's reports must not attach to a later access.
                        if mask & libc::IN_ACCESS != 0 {
                            respond::drop_test_marker(store, &c.id);
                        }
                        helper.discard_readers(&c.id);
                        continue;
                    }
                    let kinds = event_kinds(mask);
                    if kinds.is_empty() {
                        continue;
                    }
                    last.insert(c.id.clone(), Instant::now());
                    triggered(store, &commands, &mut helper, &mut responded, c, &kinds)?;
                }
            }
            known = entries.iter().map(|c| c.id.clone()).collect();
            // The startup-file comparison hashes thousands of files. It runs
            // beside the loop, so decoy events (and a test read's marker) are
            // never left waiting behind it.
            if persist.elapsed() > Duration::from_secs(300)
                && !checking.swap(true, Ordering::Relaxed)
            {
                persist = Instant::now();
                let (store, checking) = (store.clone(), checking.clone());
                std::thread::spawn(move || {
                    if let Err(e) = persistence_check(&store) {
                        eprintln!("bigman: the startup-file comparison did not finish: {e:#}");
                    }
                    checking.store(false, Ordering::Relaxed);
                });
            }
            std::thread::sleep(Duration::from_millis(500));
        }
        Ok(())
    })();
    let _ = fs::remove_file(store.root.join("monitor-health.json"));
    result
}

/// Compares startup files with the reviewed baseline and reports what changed
/// since the last comparison.
fn persistence_check(store: &Store) -> Result<()> {
    let r = crate::checks::changes::changes(store, &crate::runner::cancel())?;
    let fingerprint = crate::store::digest(&serde_json::to_vec(&(&r.findings, &r.errors))?);
    let previous = store
        .read::<String>("last-change-fingerprint.json")
        .unwrap_or_default();
    if previous != fingerprint {
        if !r.findings.is_empty() || !r.errors.is_empty() {
            store.save(&r)?;
        }
        if r.findings.iter().any(|f| f.severity != Severity::Info) || !r.errors.is_empty() {
            notify("Bigman system changes", r.status());
        }
        store.write("last-change-fingerprint.json", &fingerprint)?;
    }
    Ok(())
}

/// One noticed access: log it, report it, run the decoy's responses in the
/// background so the monitor keeps watching meanwhile, then notify.
fn triggered(
    store: &Store,
    commands: &std::sync::Arc<respond::Commands>,
    helper: &mut HelperLink,
    responded: &mut HashMap<String, Instant>,
    c: &Canary,
    kinds: &[&str],
) -> Result<()> {
    let readers = helper.take_readers(&c.id);
    let self_test = respond::take_self_test(store, &c.id, kinds, &readers);
    let routine = respond::routine_readers(&readers);
    let cooling = responded
        .get(&c.id)
        .is_some_and(|t| t.elapsed() < RESPONSE_COOLDOWN);
    let run = c.responses.any() && !self_test && routine.is_none() && !cooling;
    let mut entry = Access {
        id: uuid::Uuid::new_v4().to_string(),
        time: now(),
        kinds: kinds.iter().map(|k| k.to_string()).collect(),
        readers: readers.clone(),
        self_test,
        skipped: (c.responses.any() && !run).then(|| {
            if self_test {
                "Bigman's own test read".to_string()
            } else if let Some(why) = &routine {
                why.clone()
            } else {
                "The responses already ran for this decoy in the last minute".to_string()
            }
        }),
        outcomes: vec![],
    };
    // Read before this access is recorded, and before a log-out could end the monitor.
    let logged_out = respond::logged_out_recently(store, &c.id);
    respond::record(store, &c.id, &entry)?;
    let mut r = Report::new("Canary triggered", c.path.display());
    event_finding(&mut r, &c.id, kinds, &readers);
    if self_test {
        r.details = "Bigman's own test read.".into();
    } else if run {
        r.details = format!("Responses started: {}", c.responses.describe().join("; "));
    } else if let Some(why) = routine.as_ref().filter(|_| c.responses.any()) {
        r.details = format!("Responses not run. {why}.");
    }
    r.coverage.push("Local events while monitor is running only; not access prevention or proof of exfiltration. Repeated events within three seconds are grouped. The decoy's access log records each access and what its responses did.".into());
    if readers.is_empty() {
        r.coverage.push("No reader identified: without the optional root helper, inotify cannot say which process did it.".into());
    }
    store.save(&r)?;
    let by = readers
        .first()
        .map(|r| format!(" by {}", r.label()))
        .unwrap_or_default();
    let summary = format!("{}: {}{by}", c.path.display(), kinds.join(", "));
    if run {
        responded.insert(c.id.clone(), Instant::now());
        let (store, commands, c) = (store.clone(), commands.clone(), c.clone());
        // Started before the notification, which can take seconds to post.
        std::thread::spawn(move || {
            let (kinds, readers) = (entry.kinds.clone(), entry.readers.clone());
            let touch = respond::Touch {
                canary_id: &c.id,
                path: &c.path,
                kinds: &kinds,
                readers: &readers,
                logged_out,
            };
            respond::respond(&commands, &c.responses, &touch, |outcomes| {
                entry.outcomes = outcomes.to_vec();
                let _ = respond::record(&store, &c.id, &entry);
            });
        });
    }
    notify("Bigman canary triggered", &summary);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    fn wait_for<T>(mut f: impl FnMut() -> Option<T>) -> T {
        let started = Instant::now();
        loop {
            if let Some(v) = f() {
                return v;
            }
            assert!(started.elapsed() < Duration::from_secs(10), "timed out");
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    #[test]
    fn the_helper_names_the_process_in_the_access_log() {
        use std::os::unix::net::UnixListener;
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        // Socket paths under the temp dir can exceed the 107-byte limit.
        let runtime = std::env::var_os("XDG_RUNTIME_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir);
        let sockets = tempfile::tempdir_in(runtime).unwrap();
        let socket = sockets.path().join("h.sock");
        let listener = UnixListener::bind(&socket).unwrap();
        std::thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let uid = bigman_canary_helper::server::peer_uid(&stream).unwrap();
                std::thread::spawn(move || {
                    let config = bigman_canary_helper::server::Config {
                        usermod: "/bin/false".into(),
                        ..Default::default()
                    };
                    let _ = bigman_canary_helper::server::serve(&config, stream, uid);
                });
            }
        });
        let c = create(&store, &d.path().join("decoy.env"), None).unwrap();
        let stop = runner::cancel();
        let commands = respond::Commands {
            helper_socket: socket,
            ..Default::default()
        };
        let monitor = {
            let (store, stop) = (store.clone(), stop.clone());
            std::thread::spawn(move || watch_with(&store, &stop, commands))
        };
        wait_for(|| {
            setup::health(&store)
                .filter(|h| h.helper.connected && h.helper.watched_ids.contains(&c.id))
        });
        // The helper runs in this process without root, so the kernel names
        // only this process: exactly the reader here.
        fs::read(&c.path).unwrap();
        let access = wait_for(|| respond::log(&store, &c.id).unwrap().pop());
        assert_eq!(access.readers.len(), 1, "{access:?}");
        assert_eq!(access.readers[0].pid, std::process::id());
        assert_eq!(access.readers[0].uid, Some(unsafe { libc::getuid() }));
        // The log entry is written just before the report.
        let report = wait_for(|| {
            store
                .history()
                .unwrap()
                .into_iter()
                .find(|r| r.kind == "Canary triggered")
        });
        let full = store.report(&report.id).unwrap();
        assert!(full.findings[0]
            .evidence
            .contains(&format!("PID {}", std::process::id())));

        // The decoy is replaced by a copy moved over it, as an editor's save
        // or a restore does. The helper's mark was on the old file: the monitor
        // asks for a new one, so the next read still names its reader.
        let copy = d.path().join("decoy.env.new");
        fs::copy(&c.path, &copy).unwrap();
        fs::rename(&copy, &c.path).unwrap();
        // Past the three seconds in which events are grouped with the replacement.
        std::thread::sleep(Duration::from_millis(3500));
        fs::read(&c.path).unwrap();
        let read = wait_for(|| {
            respond::log(&store, &c.id)
                .unwrap()
                .into_iter()
                .find(|a| a.kinds == ["content read"] && a.id != access.id)
        });
        assert_eq!(read.readers.len(), 1, "{read:?}");
        assert_eq!(read.readers[0].pid, std::process::id());
        stop.store(true, Ordering::Relaxed);
        monitor.join().unwrap().unwrap();
    }

    #[test]
    fn listing_the_folder_is_not_a_read_of_the_decoy() {
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        let c = create(&store, &d.path().join("decoy.env"), None).unwrap();
        let mut w = Watcher::new().unwrap();
        assert!(w.configure(std::slice::from_ref(&c)).is_empty());
        fs::read_dir(d.path()).unwrap().for_each(drop);
        let events = w.events().unwrap();
        assert!(
            !events.is_empty(),
            "listing a watched folder reports a folder read"
        );
        assert!(events
            .iter()
            .all(|(p, m)| !p.as_ref().is_some_and(|p| concerns(&c, p, *m))));
        self_test(&store, &c.id).unwrap();
        assert!(w
            .events()
            .unwrap()
            .iter()
            .any(|(p, m)| p.as_ref().is_some_and(|p| concerns(&c, p, *m))));
    }

    #[test]
    fn a_touch_is_logged_and_runs_the_chosen_responses_but_a_test_read_does_not() {
        use std::os::unix::fs::PermissionsExt;
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        let decoys = d.path().join("documents");
        fs::create_dir(&decoys).unwrap();
        let calls = d.path().join("calls");
        let fake = |name: &str| {
            let p = d.path().join(name);
            fs::write(
                &p,
                format!("#!/bin/sh\necho \"{name} $*\" >> {}\n", calls.display()),
            )
            .unwrap();
            fs::set_permissions(&p, fs::Permissions::from_mode(0o700)).unwrap();
            p
        };
        let commands = respond::Commands {
            loginctl: fake("loginctl"),
            nmcli: fake("nmcli"),
            proc: "/proc".into(),
            helper_socket: d.path().join("no-helper.sock"),
            script_timeout: 5,
        };
        let c = create_with(
            &store,
            &decoys.join("decoy.env"),
            None,
            Responses {
                network_off: true,
                script: Some(fake("script")),
                ..Responses::default()
            },
        )
        .unwrap();
        let stop = runner::cancel();
        let monitor = {
            let (store, stop) = (store.clone(), stop.clone());
            std::thread::spawn(move || watch_with(&store, &stop, commands))
        };
        wait_for(|| setup::health(&store).filter(|h| h.watched_ids.contains(&c.id)));

        respond::mark_test(&store, &c.id).unwrap();
        self_test(&store, &c.id).unwrap();
        let test = wait_for(|| respond::log(&store, &c.id).unwrap().pop());
        assert!(test.self_test);
        assert_eq!(test.skipped.as_deref(), Some("Bigman's own test read"));
        assert!(
            !store
                .path(&format!("canaries/{}.test.json", c.id))
                .unwrap()
                .exists(),
            "the monitor uses the test marker up"
        );

        std::thread::sleep(Duration::from_secs(3));
        fs::read_dir(&decoys).unwrap().for_each(drop);
        fs::read(&c.path).unwrap();
        let touch = wait_for(|| {
            respond::log(&store, &c.id)
                .unwrap()
                .into_iter()
                .find(|a| !a.self_test && a.outcomes.len() == 2)
        });
        assert_eq!(touch.kinds, ["content read"]);
        assert!(touch.outcomes.iter().all(|o| o.ok), "{touch:?}");
        assert_eq!(
            fs::read_to_string(&calls)
                .unwrap()
                .lines()
                .collect::<Vec<_>>(),
            ["script ", "nmcli networking off"]
        );
        assert_eq!(
            respond::log(&store, &c.id).unwrap().len(),
            2,
            "the folder listing was not logged"
        );
        let reports = store.history().unwrap();
        assert!(reports.iter().any(|r| r.kind == "Canary triggered"));

        // A second decoy in the folder already watched: writing it is not a
        // touch, so its responses stay quiet until something reads it.
        let second = create_with(
            &store,
            &decoys.join("second.env"),
            None,
            Responses {
                network_off: true,
                ..Responses::default()
            },
        )
        .unwrap();
        wait_for(|| setup::health(&store).filter(|h| h.watched_ids.contains(&second.id)));
        std::thread::sleep(Duration::from_millis(1500));
        assert!(
            respond::log(&store, &second.id).unwrap().is_empty(),
            "creating a decoy was logged as an access"
        );
        assert_eq!(fs::read_to_string(&calls).unwrap().lines().count(), 2);
        fs::read(&second.path).unwrap();
        let read = wait_for(|| {
            respond::log(&store, &second.id)
                .unwrap()
                .into_iter()
                .find(|a| a.outcomes.len() == 1)
        });
        assert_eq!(read.kinds, ["content read"]);
        stop.store(true, Ordering::Relaxed);
        monitor.join().unwrap().unwrap();
    }

    #[test]
    fn warns_only_the_one_administrator_about_locking() {
        let d = tempfile::tempdir().unwrap();
        let (passwd, group) = (d.path().join("passwd"), d.path().join("group"));
        fs::write(
            &passwd,
            "root:x:0:0::/root:/bin/bash\nana:x:1000:1000::/home/ana:/bin/bash\nben:x:1001:1001::/home/ben:/bin/bash\nops:x:1002:10::/home/ops:/bin/bash\n",
        )
        .unwrap();
        fs::write(&group, "root:x:0:\nwheel:x:10:root,ana\nana:x:1000:\n").unwrap();
        // ops is in wheel through its primary group, so ana is not the only one.
        assert_eq!(lock_warning(&passwd, &group, 1000), None);
        fs::write(&group, "root:x:0:\nwheel:x:27:root,ana\nana:x:1000:\n").unwrap();
        assert!(lock_warning(&passwd, &group, 1000)
            .unwrap()
            .contains("only administrator"));
        // Someone who is not an administrator has one to turn to.
        assert_eq!(lock_warning(&passwd, &group, 1001), None);
        fs::write(&group, "sudo:x:27:ana,ben\n").unwrap();
        assert_eq!(lock_warning(&passwd, &group, 1000), None);
    }
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
