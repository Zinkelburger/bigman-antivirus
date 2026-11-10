//! New project folders: a repository cloned, or an archive unpacked, into a
//! folder where you keep code, checked with the project inspection as soon as it
//! lands. Fake job interviews hand over a repository and ask you to open it;
//! this checks it before you have to remember to.
use super::{cursor, gap, reports::report_outcome, set_cursor, Collected, Ctx, Gap, Source};
use crate::{
    checks::{canary::Watcher, route},
    model::{Link, Observation, Severity, SkipReason, SourceId},
    runner::Cancel,
    store::home,
};
use std::{
    collections::BTreeMap,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    time::{Duration, UNIX_EPOCH},
};

/// Folders in your home where people usually keep code. Each one that exists is
/// watched, along with the Downloads folder, where archives get unpacked.
const ROOTS: [&str; 11] = [
    "Projects",
    "projects",
    "src",
    "code",
    "Code",
    "git",
    "dev",
    "repos",
    "workspace",
    "Developer",
    "work",
];
/// A folder with none of these is not a project, and is not checked.
const MARKERS: [&str; 17] = [
    ".git",
    "package.json",
    ".vscode",
    ".devcontainer",
    ".claude",
    ".mcp.json",
    ".envrc",
    "pyproject.toml",
    "setup.py",
    "requirements.txt",
    "Cargo.toml",
    "go.mod",
    "composer.json",
    "Gemfile",
    "pom.xml",
    "build.gradle",
    "Makefile",
];
/// A clone that has not finished after this long is checked as it stands.
const GIVE_UP: u64 = 600;
/// After a restart, at most this many folders that arrived meanwhile are checked.
const CATCH_UP: usize = 20;

/// The folders watched on this machine.
pub fn roots() -> Vec<PathBuf> {
    let h = home();
    let mut out: Vec<PathBuf> = vec![];
    for dir in ROOTS
        .iter()
        .map(|r| h.join(r))
        .chain([crate::checks::setup::downloads()])
    {
        // Case-insensitive file systems, and links between these names, give one folder twice.
        if let Ok(real) = dir.canonicalize() {
            if real.is_dir() && !out.contains(&real) {
                out.push(real);
            }
        }
    }
    out
}

pub struct Projects {
    roots: Vec<PathBuf>,
    watcher: Option<Watcher>,
    /// Folder -> when it was first seen.
    pending: BTreeMap<PathBuf, u64>,
    /// Seconds a folder must stay unchanged before it is checked.
    settle: u64,
    stop: Cancel,
}

impl Projects {
    pub fn new(roots: Vec<PathBuf>, stop: Cancel) -> Self {
        Self {
            roots,
            watcher: None,
            pending: BTreeMap::new(),
            settle: 10,
            stop,
        }
    }

    #[cfg(test)]
    pub(crate) fn settle_after(mut self, secs: u64) -> Self {
        self.settle = secs;
        self
    }

    fn start(&mut self, ctx: &Ctx) -> Result<(), Gap> {
        if self.roots.is_empty() {
            return Err(gap(
                SkipReason::NotApplicable,
                "None of the usual project folders (~/Projects, ~/src, ~/code and similar) exist.",
            ));
        }
        let mut watcher = Watcher::new().map_err(|e| gap(SkipReason::Failed, format!("{e:#}")))?;
        for root in &self.roots {
            watcher
                .add_dir(
                    root,
                    libc::IN_CREATE | libc::IN_MOVED_TO | libc::IN_ONLYDIR | libc::IN_DONT_FOLLOW,
                )
                .map_err(|e| gap(SkipReason::PermissionDenied, format!("{e:#}")))?;
        }
        // Folders created while the daemon was not running, by creation time, so a
        // project you edited meanwhile is not taken for a new one. The first run
        // ever starts from now.
        if let Some(since) = cursor(ctx.db, "projects").and_then(|c| c.parse::<u64>().ok()) {
            let mut missed: Vec<(u64, PathBuf)> = self
                .roots
                .iter()
                .filter_map(|r| std::fs::read_dir(r).ok())
                .flatten()
                .flatten()
                .filter_map(|e| {
                    let meta = e.metadata().ok()?;
                    let born = meta
                        .created()
                        .ok()?
                        .duration_since(UNIX_EPOCH)
                        .ok()?
                        .as_secs();
                    (meta.is_dir() && born > since).then(|| (born, e.path()))
                })
                .collect();
            missed.sort();
            for (_, path) in missed.into_iter().rev().take(CATCH_UP) {
                self.pending.insert(path, ctx.now);
            }
        }
        self.watcher = Some(watcher);
        Ok(())
    }

    fn check(&self, ctx: &Ctx, dir: &Path) -> Observation {
        let subject = dir.display().to_string();
        super::downloads::IN_FLIGHT
            .lock()
            .unwrap()
            .insert(subject.clone());
        let result = route::project(ctx.store, dir);
        if let Ok(r) = &result {
            let _ = ctx
                .db
                .touch(&format!("report-id:{}", r.id), SourceId::Scan, r.created);
        }
        super::downloads::IN_FLIGHT.lock().unwrap().remove(&subject);
        observation(ctx.now, dir, result)
    }
}

/// What to do with a folder that appeared.
#[derive(Debug, PartialEq)]
enum Next {
    Wait,
    Check,
    Drop,
}

fn changed_at(path: &Path) -> u64 {
    std::fs::symlink_metadata(path).map_or(0, |m| m.ctime().max(0) as u64)
}

/// A clone is finished once git has written its index and let go of the lock; an
/// unpacked or new folder once its top level stops changing.
fn next(dir: &Path, first_seen: u64, now: u64, settle: u64) -> Next {
    if !std::fs::symlink_metadata(dir).is_ok_and(|m| m.is_dir()) {
        return Next::Drop;
    }
    let git = dir.join(".git");
    let cloning = git.is_dir() && (!git.join("index").exists() || git.join("index.lock").exists());
    let settled = !cloning && changed_at(dir) + settle <= now;
    if !settled && first_seen + GIVE_UP > now {
        return Next::Wait;
    }
    if MARKERS
        .iter()
        .any(|m| dir.join(m).symlink_metadata().is_ok())
    {
        Next::Check
    } else {
        Next::Drop
    }
}

/// One timeline entry per project folder, keyed by the folder and when it was
/// made, so the same folder cloned again is a new entry.
fn observation(at: u64, dir: &Path, result: anyhow::Result<crate::model::Report>) -> Observation {
    let name = dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let place = dir
        .parent()
        .map(|p| {
            p.strip_prefix(home())
                .map(|rel| format!("~/{}", rel.display()))
                .unwrap_or_else(|_| p.display().to_string())
        })
        .unwrap_or_default();
    let key = format!(
        "project:{}:{}",
        dir.display(),
        changed_at(&dir.join(".git"))
    );
    let subject = dir.display().to_string();
    match result {
        Ok(r) => Observation {
            at,
            source: SourceId::Project,
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
            summary: format!("New project in {place}: {name} · {}", report_outcome(&r)),
            evidence: format!(
                "{subject}\nChecked with: {}{}",
                r.kind,
                if r.findings.iter().any(|f| f.severity == Severity::High) {
                    "\nDo not open it in an editor or run it on this computer until you have read the findings."
                } else {
                    ""
                }
            ),
            link: Some(Link::Report { id: r.id }),
        },
        Err(e) => Observation {
            at,
            source: SourceId::Project,
            key,
            rule: None,
            severity: Severity::Info,
            summary: format!("New project in {place}: {name} · not checked"),
            evidence: format!("{subject}\n{e:#}"),
            link: None,
        },
    }
}

impl Source for Projects {
    fn name(&self) -> &'static str {
        "projects"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(2)
    }
    fn turned_off(&mut self, ctx: &Ctx) {
        self.watcher = None;
        self.pending.clear();
        set_cursor(ctx.db, "projects", &ctx.now.to_string());
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
                // Too many changes at once: look at the folders again from the cursor.
                self.watcher = None;
                break;
            }
            if let Some(path) = path.filter(|_| mask & libc::IN_ISDIR != 0) {
                self.pending.entry(path).or_insert(ctx.now);
            }
        }
        let mut observations = vec![];
        let due: Vec<(PathBuf, u64)> = self.pending.iter().map(|(p, t)| (p.clone(), *t)).collect();
        for (dir, first_seen) in due {
            if self.stop.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            match next(&dir, first_seen, ctx.now, self.settle) {
                Next::Wait => {}
                Next::Drop => {
                    self.pending.remove(&dir);
                }
                Next::Check => {
                    self.pending.remove(&dir);
                    observations.push(self.check(ctx, &dir));
                }
            }
        }
        // Resume from the oldest folder still waiting, or from now.
        let resume = self.pending.values().min().copied().unwrap_or(ctx.now);
        set_cursor(ctx.db, "projects", &resume.saturating_sub(1).to_string());
        Ok(Collected {
            observations,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waits_for_git_to_finish_and_skips_folders_that_are_not_projects() {
        let d = tempfile::tempdir().unwrap();
        let repo = d.path().join("take-home");
        std::fs::create_dir_all(repo.join(".git")).unwrap();
        let now = crate::model::now() + 100;
        // Objects still arriving: no index yet.
        assert_eq!(next(&repo, now, now, 0), Next::Wait);
        std::fs::write(repo.join(".git/index"), "").unwrap();
        std::fs::write(repo.join(".git/index.lock"), "").unwrap();
        assert_eq!(next(&repo, now, now, 0), Next::Wait);
        std::fs::remove_file(repo.join(".git/index.lock")).unwrap();
        assert_eq!(next(&repo, now, now, 0), Next::Check);
        // A clone that never finishes is checked after GIVE_UP.
        std::fs::remove_file(repo.join(".git/index")).unwrap();
        assert_eq!(next(&repo, now - GIVE_UP, now, 0), Next::Check);
        let notes = d.path().join("holiday-photos");
        std::fs::create_dir(&notes).unwrap();
        assert_eq!(next(&notes, now, now, 0), Next::Drop);
        assert_eq!(next(&d.path().join("gone"), now, now, 0), Next::Drop);
    }
}
