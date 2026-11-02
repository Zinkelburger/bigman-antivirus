//! Live SSH access signals: keys that can log in, reverse tunnels, network
//! logins and private keys outside ~/.ssh. Each is cheap enough to poll every
//! few seconds, so a planted key is on the timeline (and in a notification)
//! within seconds rather than at the next five-minute startup-file comparison.
use super::{gap, Collected, Ctx, Gap, Source};
use crate::{
    checks::ssh::{self, AuthorizedKey},
    model::{Observation, Report, Severity, SkipReason, SourceId},
    platform::{Clock, ProcessEntry, Processes, Sessions},
    store::home,
};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    time::Duration,
};

/// The finding a check wrote into a scratch report, as a timeline observation.
fn observe(r: &Report, now: u64, key: String) -> Option<Observation> {
    let f = r.findings.iter().max_by_key(|f| f.severity)?;
    Some(Observation {
        at: now,
        source: SourceId::Ssh,
        key,
        rule: Some(f.rule.clone()),
        severity: f.severity,
        summary: f.summary.clone(),
        evidence: f.evidence.clone(),
        link: None,
    })
}

fn info(now: u64, key: String, summary: String, evidence: String) -> Observation {
    Observation {
        at: now,
        source: SourceId::Ssh,
        key,
        rule: None,
        severity: Severity::Info,
        summary,
        evidence,
        link: None,
    }
}

// --------------------------------------------------------- authorized keys

/// Keys in `~/.ssh/authorized_keys` (and the legacy `authorized_keys2`).
///
/// Known fingerprints are kept in the database, so a key added while bigmand
/// was not running is still reported when it starts.
pub struct AuthorizedKeys {
    files: Vec<PathBuf>,
    /// (mtime, ctime, size, inode) per file at the last read; unchanged files are not reread.
    stamps: HashMap<PathBuf, Option<(i64, i64, u64, u64)>>,
}

const KNOWN_KEYS: &str = "authorized_keys.known";

impl AuthorizedKeys {
    pub fn new(files: Vec<PathBuf>) -> Self {
        AuthorizedKeys {
            files,
            stamps: HashMap::new(),
        }
    }
    pub fn for_home() -> Self {
        let ssh = home().join(".ssh");
        Self::new(vec![
            ssh.join("authorized_keys"),
            ssh.join("authorized_keys2"),
        ])
    }
}

fn stamp(path: &Path) -> Option<(i64, i64, u64, u64)> {
    let m = fs::metadata(path).ok()?;
    Some((
        m.mtime_nsec() + m.mtime() * 1_000_000_000,
        m.ctime_nsec() + m.ctime() * 1_000_000_000,
        m.size(),
        m.ino(),
    ))
}

impl Source for AuthorizedKeys {
    fn name(&self) -> &'static str {
        "ssh_keys"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(2)
    }
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap> {
        let changed: Vec<PathBuf> = self
            .files
            .iter()
            .filter(|f| self.stamps.get(*f) != Some(&stamp(f)))
            .cloned()
            .collect();
        if changed.is_empty() {
            return Ok(Collected::default());
        }
        let stored = ctx.db.setting(KNOWN_KEYS).ok().flatten();
        let first_run = stored.is_none();
        let mut known: BTreeMap<String, BTreeSet<String>> = stored
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        let mut observations = vec![];
        for path in &changed {
            let keys: Vec<AuthorizedKey> = match fs::read(path) {
                Ok(bytes) => ssh::parse_authorized_keys(&String::from_utf8_lossy(&bytes)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => vec![],
                Err(e) => {
                    return Err(gap(
                        SkipReason::Unreadable,
                        format!("{}: {e}", path.display()),
                    ))
                }
            };
            let name = path.display().to_string();
            let before = known.get(&name).cloned().unwrap_or_default();
            let now: BTreeSet<String> = keys.iter().map(|k| k.fingerprint.clone()).collect();
            for k in keys.iter().filter(|k| !before.contains(&k.fingerprint)) {
                let key = format!("ssh-key:{}", k.fingerprint);
                if first_run {
                    // Keys already present when Bigman first looks are listed, not alarmed.
                    observations.push(info(
                        ctx.now,
                        key,
                        "An SSH key can log in to your account".into(),
                        format!("{}. Recorded when Bigman first checked; review it if you do not recognise it.", ssh::describe_key(path, k)),
                    ));
                } else {
                    let mut r = Report::new("SSH keys", &name);
                    ssh::key_added_finding(&mut r, path, k);
                    observations.extend(observe(&r, ctx.now, key));
                }
            }
            for gone in before.difference(&now) {
                observations.push(info(
                    ctx.now,
                    format!("ssh-key-removed:{gone}"),
                    "An SSH key was removed from your account".into(),
                    format!("{name}: {gone}"),
                ));
            }
            if now.is_empty() {
                known.remove(&name);
            } else {
                known.insert(name, now);
            }
            self.stamps.insert(path.clone(), stamp(path));
        }
        let _ = ctx.db.set_setting(
            KNOWN_KEYS,
            &serde_json::to_string(&known).unwrap_or_default(),
        );
        Ok(Collected {
            observations,
            ..Default::default()
        })
    }
}

// ----------------------------------------------------------------- tunnels

/// Your own ssh and autossh processes that forward connections back to this computer.
pub struct Tunnels {
    processes: Box<dyn Processes>,
    reported: BTreeSet<u32>,
}

impl Tunnels {
    pub fn new(processes: Box<dyn Processes>) -> Self {
        Tunnels {
            processes,
            reported: BTreeSet::new(),
        }
    }
}

/// Names of a process's parents, nearest first, at most six.
pub fn ancestry(pid: u32, all: &HashMap<u32, &ProcessEntry>) -> Vec<String> {
    let mut out = vec![];
    let mut at = all.get(&pid).map(|p| p.ppid);
    while let Some(ppid) = at.filter(|p| *p > 1 && out.len() < 6) {
        let Some(p) = all.get(&ppid) else { break };
        let exe = p
            .exe
            .as_ref()
            .and_then(|e| e.file_name())
            .map(|n| n.to_string_lossy().into_owned());
        // Electron editors often show as "electron" or a helper; the exe names the editor.
        out.push(match exe {
            Some(e) if ssh::EDITORS.contains(&e.as_str()) => e,
            _ => p.name.clone(),
        });
        at = Some(p.ppid);
    }
    out
}

impl Source for Tunnels {
    fn name(&self) -> &'static str {
        "ssh_tunnels"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(3)
    }
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap> {
        let procs = self
            .processes
            .processes()
            .map_err(|e| gap(SkipReason::Unreadable, format!("{e:#}")))?;
        let by_pid: HashMap<u32, &ProcessEntry> = procs.iter().map(|p| (p.pid, p)).collect();
        let mut observations = vec![];
        let mut alive = BTreeSet::new();
        for p in &procs {
            let Some(t) = ssh::parse_tunnel(&p.args, p.exe.as_deref()) else {
                continue;
            };
            alive.insert(p.pid);
            if self.reported.contains(&p.pid) {
                continue;
            }
            let command = crate::model::clean(&p.args.join(" "));
            let mut r = Report::new("SSH tunnel", p.pid);
            ssh::tunnel_finding(&mut r, &t, p.pid, &command, &ancestry(p.pid, &by_pid));
            // autossh restarts ssh with the same arguments; those fold into one entry.
            let key = format!(
                "ssh-tunnel:{}:{}",
                t.destination.as_deref().unwrap_or("?"),
                t.remote_forwards.join(",")
            );
            observations.extend(observe(&r, ctx.now, key));
        }
        self.reported = alive;
        Ok(Collected {
            observations,
            ..Default::default()
        })
    }
}

// ------------------------------------------------------------------ logins

/// Network logins to this computer, for any user, as logind reports them.
pub struct RemoteLogins {
    sessions: Box<dyn Sessions>,
    clock: Box<dyn Clock>,
}

impl RemoteLogins {
    pub fn new(sessions: Box<dyn Sessions>, clock: Box<dyn Clock>) -> Self {
        RemoteLogins { sessions, clock }
    }
}

impl Source for RemoteLogins {
    fn name(&self) -> &'static str {
        "ssh_logins"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(5)
    }
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap> {
        let sessions = self
            .sessions
            .sessions()
            .map_err(|e| gap(SkipReason::ToolMissing, format!("{e:#}")))?;
        let boot = self.clock.boot_id().unwrap_or_default();
        let mut observations = vec![];
        for s in sessions.iter().filter(|s| s.remote) {
            // Session ids restart every boot. Remembered in the store, so a login
            // while bigmand was down is still reported, and only once.
            let seen = format!("ssh-session:{boot}:{}", s.id);
            if ctx.db.seen(&seen).unwrap_or(false) {
                continue;
            }
            let _ = ctx.db.touch(&seen, SourceId::Ssh, ctx.now);
            let mut r = Report::new("Remote login", &s.id);
            ssh::remote_login_finding(
                &mut r,
                &s.user,
                s.remote_host.as_deref(),
                &s.service,
                s.tty.as_deref(),
            );
            let key = format!(
                "ssh-login:{}:{}",
                s.user,
                s.remote_host.as_deref().unwrap_or("?")
            );
            observations.extend(observe(&r, ctx.now, key));
        }
        Ok(Collected {
            observations,
            ..Default::default()
        })
    }
}

// --------------------------------------------------------------- stray keys

/// Private SSH keys sitting directly in temporary or configuration folders,
/// where tools rarely keep them and droppers often do.
pub struct StrayKeys {
    dirs: Vec<PathBuf>,
}

impl StrayKeys {
    pub fn new(dirs: Vec<PathBuf>) -> Self {
        StrayKeys { dirs }
    }
    pub fn default_dirs() -> Self {
        let h = home();
        let mut dirs = vec![
            std::env::temp_dir(),
            PathBuf::from("/tmp"),
            PathBuf::from("/var/tmp"),
            PathBuf::from("/dev/shm"),
            h.join(".config"),
            h.join(".cache"),
            h.join(".local/share"),
        ];
        if let Some(run) = std::env::var_os("XDG_RUNTIME_DIR") {
            dirs.push(run.into());
        }
        dirs.sort();
        dirs.dedup();
        StrayKeys { dirs }
    }
}

const MAX_KEY_BYTES: u64 = 32 * 1024;

impl Source for StrayKeys {
    fn name(&self) -> &'static str {
        "ssh_stray_keys"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(30)
    }
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap> {
        use std::io::Read;
        let uid = unsafe { libc::getuid() };
        let mut observations = vec![];
        for dir in &self.dirs {
            let Ok(entries) = fs::read_dir(dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                // symlink_metadata: a link is never followed, and only your own files are opened.
                let Ok(m) = fs::symlink_metadata(&path) else {
                    continue;
                };
                if !m.is_file() || m.uid() != uid || m.size() > MAX_KEY_BYTES || m.size() < 64 {
                    continue;
                }
                let mut head = [0u8; 64];
                let read = fs::File::open(&path).and_then(|mut f| f.read(&mut head));
                if !read.is_ok_and(|n| ssh::is_ssh_private_key(&head[..n])) {
                    continue;
                }
                let mut r = Report::new("Stray SSH key", path.display());
                ssh::stray_key_finding(&mut r, &path, m.size(), m.mode());
                let key = format!("ssh-stray-key:{}:{}", path.display(), m.ino());
                if ctx.db.seen(&key).unwrap_or(false) {
                    continue;
                }
                observations.extend(observe(&r, ctx.now, key));
            }
        }
        Ok(Collected {
            observations,
            ..Default::default()
        })
    }
}
