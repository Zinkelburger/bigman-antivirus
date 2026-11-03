use super::{cursor, gap, reports::report_outcome, set_cursor, Collected, Ctx, Gap, Source};
use crate::{
    checks::{archive::file_digest, canary::Watcher, route, scan},
    model::{Link, Observation, Report, Severity, SkipReason, SourceId},
    runner::Cancel,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    sync::{LazyLock, Mutex},
    time::Duration,
};

/// Files the watch is checking right now. The reports source leaves their
/// reports alone, so a download shows up once, as a download.
pub(crate) static IN_FLIGHT: LazyLock<Mutex<BTreeSet<String>>> =
    LazyLock::new(|| Mutex::new(BTreeSet::new()));

/// Names browsers give a download until it completes.
const PARTIAL: &[&str] = &[
    ".part",
    ".crdownload",
    ".download",
    ".partial",
    ".opdownload",
    ".tmp",
];
/// After a restart, at most this many files that arrived meanwhile are checked.
const CATCH_UP: usize = 50;

/// New files in the Downloads folder, each sent to the inspector Inspect would
/// pick, or to a ClamAV scan when no inspector applies, and posted once.
pub struct Downloads {
    dir: PathBuf,
    watcher: Option<Watcher>,
    /// Path -> when it was last seen changing.
    pending: BTreeMap<PathBuf, u64>,
    /// Seconds a file must stay unchanged before it is checked.
    settle: u64,
    stop: Cancel,
}

impl Downloads {
    pub fn new(dir: PathBuf, stop: Cancel) -> Self {
        Self {
            dir,
            watcher: None,
            pending: BTreeMap::new(),
            settle: 3,
            stop,
        }
    }

    #[cfg(test)]
    pub(crate) fn settle_after(mut self, secs: u64) -> Self {
        self.settle = secs;
        self
    }

    fn start(&mut self, ctx: &Ctx) -> Result<(), Gap> {
        if !self.dir.is_dir() {
            return Err(gap(
                SkipReason::NotApplicable,
                format!("There is no Downloads folder at {}.", self.dir.display()),
            ));
        }
        let mut watcher = Watcher::new().map_err(|e| gap(SkipReason::Failed, format!("{e:#}")))?;
        watcher
            .add_dir(
                &self.dir,
                libc::IN_CLOSE_WRITE | libc::IN_MOVED_TO | libc::IN_ONLYDIR | libc::IN_DONT_FOLLOW,
            )
            .map_err(|e| gap(SkipReason::PermissionDenied, format!("{e:#}")))?;
        // What arrived while the daemon was not running; the first run ever
        // starts from now rather than checking the whole folder.
        if let Some(since) = cursor(ctx.db, "downloads").and_then(|c| c.parse::<i64>().ok()) {
            let mut missed: Vec<(i64, PathBuf)> = std::fs::read_dir(&self.dir)
                .into_iter()
                .flatten()
                .flatten()
                .filter_map(|e| {
                    let meta = e.metadata().ok()?;
                    (meta.is_file() && meta.ctime() > since).then(|| (meta.ctime(), e.path()))
                })
                .collect();
            missed.sort();
            for (_, path) in missed.into_iter().rev().take(CATCH_UP) {
                self.pending.insert(path, 0);
            }
        }
        self.watcher = Some(watcher);
        Ok(())
    }

    fn check(&self, ctx: &Ctx, path: &Path) -> Observation {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let subject = path.display().to_string();
        let digest = file_digest(path).unwrap_or_default();
        IN_FLIGHT.lock().unwrap().insert(subject.clone());
        let result = if route::choose(&subject) == route::Inspector::Installer && !script(path) {
            scan::scan(ctx.store, path, &self.stop)
        } else {
            route::inspect_any(ctx.store, &subject, &self.stop)
        };
        if let Ok(r) = &result {
            let _ = ctx
                .db
                .touch(&format!("report-id:{}", r.id), SourceId::Scan, r.created);
        }
        IN_FLIGHT.lock().unwrap().remove(&subject);
        observation(ctx.now, &name, &subject, &digest, result)
    }
}

/// A downloaded script goes to the installer rules; other unknown files to ClamAV.
fn script(path: &Path) -> bool {
    let mut head = [0u8; 2];
    let shebang = std::fs::File::open(path)
        .and_then(|mut f| f.read_exact(&mut head))
        .is_ok()
        && &head == b"#!";
    shebang
        || path
            .extension()
            .is_some_and(|e| e == "sh" || e == "bash" || e == "zsh" || e == "command")
}

fn partial(path: &Path) -> bool {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    name.starts_with('.') || PARTIAL.iter().any(|p| name.ends_with(p))
}

/// One timeline entry per downloaded file, keyed by its contents, so the same
/// file downloaded again folds into the first sighting.
pub fn observation(
    at: u64,
    name: &str,
    subject: &str,
    digest: &str,
    result: anyhow::Result<Report>,
) -> Observation {
    let key = if digest.is_empty() {
        format!("download:{subject}")
    } else {
        format!("download:{digest}")
    };
    match result {
        Ok(r) => Observation {
            at,
            source: SourceId::Download,
            key,
            rule: r
                .findings
                .iter()
                .max_by_key(|f| f.severity)
                .map(|f| f.rule.clone()),
            severity: r
                .findings
                .iter()
                .map(|f| f.severity)
                .max()
                .unwrap_or(Severity::Info),
            summary: format!("New in Downloads: {name} · {}", report_outcome(&r)),
            evidence: format!("{subject}\nChecked with: {}", r.kind),
            link: Some(Link::Report { id: r.id }),
        },
        Err(e) => Observation {
            at,
            source: SourceId::Download,
            key,
            rule: None,
            severity: Severity::Info,
            summary: format!("New in Downloads: {name} · not checked"),
            evidence: format!("{subject}\n{e:#}"),
            link: None,
        },
    }
}

impl Source for Downloads {
    fn name(&self) -> &'static str {
        "downloads"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(2)
    }
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap> {
        if self.watcher.is_none() {
            self.start(ctx)?;
        }
        let events = match self.watcher.as_mut().map(Watcher::events) {
            Some(Ok(events)) => events,
            Some(Err(e)) => {
                self.watcher = None;
                return Err(gap(SkipReason::Failed, format!("{e:#}")));
            }
            None => vec![],
        };
        for (path, mask) in events {
            if mask & libc::IN_Q_OVERFLOW != 0 {
                // Too many changes at once: look at the folder again from the cursor.
                self.watcher = None;
                break;
            }
            if let Some(path) = path.filter(|_| mask & libc::IN_ISDIR == 0) {
                self.pending.insert(path, ctx.now);
            }
        }
        let mut observations = vec![];
        let settled: Vec<PathBuf> = self
            .pending
            .iter()
            .filter(|(_, seen)| *seen + self.settle <= ctx.now)
            .map(|(p, _)| p.clone())
            .collect();
        for path in settled {
            if self.stop.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            let Ok(meta) = std::fs::symlink_metadata(&path) else {
                self.pending.remove(&path);
                continue;
            };
            // Still being written: wait for it to stay unchanged. ctime, because a
            // download's mtime can be set from the server.
            if meta.ctime() as u64 + self.settle > ctx.now {
                self.pending.insert(path, ctx.now);
                continue;
            }
            self.pending.remove(&path);
            if meta.is_file() && !partial(&path) {
                observations.push(self.check(ctx, &path));
            }
        }
        let resume = self
            .pending
            .keys()
            .filter_map(|p| std::fs::metadata(p).ok().map(|m| m.ctime() - 1))
            .min()
            .unwrap_or(ctx.now as i64);
        set_cursor(ctx.db, "downloads", &resume.to_string());
        Ok(Collected {
            observations,
            samples: vec![],
        })
    }
}
