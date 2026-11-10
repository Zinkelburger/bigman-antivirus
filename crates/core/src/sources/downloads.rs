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
    /// Files that arrived while nothing watched, past the `catch_up` checked.
    missed: usize,
    catch_up: usize,
    stop: Cancel,
}

impl Downloads {
    pub fn new(dir: PathBuf, stop: Cancel) -> Self {
        Self {
            dir,
            watcher: None,
            pending: BTreeMap::new(),
            settle: 3,
            missed: 0,
            catch_up: CATCH_UP,
            stop,
        }
    }

    #[cfg(test)]
    pub(crate) fn settle_after(mut self, secs: u64) -> Self {
        self.settle = secs;
        self
    }

    #[cfg(test)]
    pub(crate) fn catch_up(mut self, files: usize) -> Self {
        self.catch_up = files;
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
                libc::IN_CLOSE_WRITE
                    | libc::IN_MOVED_TO
                    | libc::IN_DELETE_SELF
                    | libc::IN_MOVE_SELF
                    | libc::IN_ONLYDIR
                    | libc::IN_DONT_FOLLOW,
            )
            .map_err(|e| gap(SkipReason::PermissionDenied, format!("{e:#}")))?;
        // What arrived while nothing watched (the daemon was not running, or the
        // watch was lost); the first run ever starts from now rather than
        // checking the whole folder, and records that at once so a watch lost
        // before the first collect ends still has a place to resume from.
        let since = cursor(ctx.db, "downloads").and_then(|c| c.parse::<i64>().ok());
        if since.is_none() {
            set_cursor(ctx.db, "downloads", &ctx.now.to_string());
        }
        if let Some(since) = since {
            let mut missed: Vec<(i64, PathBuf)> = std::fs::read_dir(&self.dir)
                .into_iter()
                .flatten()
                .flatten()
                .filter_map(|e| {
                    let meta = e.metadata().ok()?;
                    (meta.is_file() && meta.ctime() >= since).then(|| (meta.ctime(), e.path()))
                })
                .filter(|(_, path)| !partial(path))
                // The cursor is a whole second, so a file from that second may
                // have arrived after the last look, or been checked in it.
                .filter(|(ctime, path)| *ctime > since || !already_posted(ctx, path))
                .collect();
            missed.sort();
            self.missed += missed.len().saturating_sub(self.catch_up);
            for (_, path) in missed.into_iter().rev().take(self.catch_up) {
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
        IN_FLIGHT.lock().unwrap().insert(subject.clone());
        let routed = route::choose(&subject);
        let result = if routed == route::Inspector::Installer && !script(path) {
            scan::scan(ctx.store, path, &self.stop)
        } else {
            match route::inspect_any(ctx.store, &subject, &self.stop) {
                // A file named or shaped like a package or archive that its
                // inspector cannot read still gets a signature scan.
                Err(e) if routed != route::Inspector::Installer => {
                    scan::scan(ctx.store, path, &self.stop).and_then(|mut r| {
                        r.coverage
                            .skip(routed.label(), SkipReason::Unreadable, format!("{e:#}"));
                        ctx.store.save(&r)?;
                        Ok(r)
                    })
                }
                other => other,
            }
        };
        // Most inspectors hash the file already; only hash it again if not.
        let digest = result
            .as_ref()
            .ok()
            .and_then(|r| r.sha256.clone())
            .unwrap_or_else(|| file_digest(path).unwrap_or_default());
        if let Ok(r) = &result {
            let _ = ctx
                .db
                .touch(&format!("report-id:{}", r.id), SourceId::Scan, r.created);
        }
        IN_FLIGHT.lock().unwrap().remove(&subject);
        observation(ctx.now, &name, &subject, &digest, result)
    }
}

/// Whether this file's contents were posted as a download before.
fn already_posted(ctx: &Ctx, path: &Path) -> bool {
    file_digest(path).is_ok_and(|d| ctx.db.seen(&format!("download:{d}")).unwrap_or(false))
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
    fn turned_off(&mut self, ctx: &Ctx) {
        self.watcher = None;
        self.pending.clear();
        self.missed = 0;
        set_cursor(ctx.db, "downloads", &ctx.now.to_string());
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
        // Set when the watch can no longer be trusted: too many changes at once
        // (the queue overflowed), or the folder itself was deleted or moved. The
        // watch starts again next time and looks at the folder from the cursor,
        // so the cursor must not move past what this watch missed.
        let mut lost = false;
        for (path, mask) in events {
            if mask
                & (libc::IN_Q_OVERFLOW
                    | libc::IN_IGNORED
                    | libc::IN_DELETE_SELF
                    | libc::IN_MOVE_SELF)
                != 0
            {
                self.watcher = None;
                lost = true;
                break;
            }
            if let Some(path) = path.filter(|_| mask & libc::IN_ISDIR == 0) {
                self.pending.insert(path, ctx.now);
            }
        }
        let mut observations = vec![];
        if self.missed > 0 {
            let missed = std::mem::take(&mut self.missed);
            let (files, were) = if missed == 1 {
                ("file", "was")
            } else {
                ("files", "were")
            };
            observations.push(Observation {
                at: ctx.now,
                source: SourceId::Download,
                key: format!("download:missed:{}", ctx.now),
                rule: None,
                severity: Severity::Info,
                summary: format!("{missed} more new {files} in Downloads {were} not checked"),
                evidence: format!(
                    "{}\nMore files arrived while the folder was not watched than are checked at once ({}); the newest were checked. Inspect the others from the Inspect page.",
                    self.dir.display(),
                    self.catch_up
                ),
                link: None,
            });
        }
        let settled: Vec<PathBuf> = self
            .pending
            .iter()
            .filter(|(_, seen)| *seen + self.settle <= ctx.now)
            .map(|(p, _)| p.clone())
            .collect();
        // A decoy set up in this folder is not a download: checking it would
        // read it, and so set off its own alert. Scans leave decoys out too.
        let decoys = crate::checks::canary::list(ctx.store).unwrap_or_default();
        for path in settled {
            if self.stop.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            if decoys.iter().any(|c| c.path == path) {
                self.pending.remove(&path);
                continue;
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
        if !lost {
            set_cursor(ctx.db, "downloads", &resume.to_string());
        }
        Ok(Collected {
            observations,
            samples: vec![],
        })
    }
}
