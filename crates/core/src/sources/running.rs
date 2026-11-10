use super::{gap, Collected, Ctx, Gap, Source};
use crate::{
    checks::{
        known::{self, Standing},
        running::{self, RunningItem, Snapshot},
    },
    model::{Link, Observation, Report, Severity, SkipReason, SourceId},
    runner::Cancel,
};
use std::{collections::BTreeSet, time::Duration};

const SEEN: &str = "running.seen";

/// The running system, checked every ten minutes: a process, module or socket
/// that one view shows and another does not, a program running from memory or
/// a deleted file, a new holder of a packet socket, a new watcher of a
/// credential folder. A row is posted once, when it first needs review; rows
/// you marked as known are posted when they change. Info rows stay in the
/// Running system view.
pub struct Running {
    snapshot: Box<dyn FnMut() -> Option<Snapshot> + Send>,
}

impl Running {
    pub fn new(stop: Cancel) -> Self {
        Self::reading(move || {
            let snapshot = running::gather::gather(&stop);
            // A cancelled read is partial; the next run reads again.
            (!stop.load(std::sync::atomic::Ordering::Relaxed)).then_some(snapshot)
        })
    }
    /// With another reader of the system; tests substitute snapshots.
    pub fn reading(snapshot: impl FnMut() -> Option<Snapshot> + Send + 'static) -> Self {
        Self {
            snapshot: Box::new(snapshot),
        }
    }
}

fn seen_key(item: &RunningItem) -> String {
    format!("{}\0{}", item.key, known::pin(&item.facts))
}

/// One timeline entry for a row that needs review.
pub fn observation(at: u64, item: &RunningItem, report: &Report) -> Observation {
    let mut evidence = item.evidence.clone();
    if item.standing == Standing::Changed {
        evidence.push_str(&format!("\nChanged: {}", item.changed.join("; ")));
    }
    Observation {
        at,
        source: SourceId::Running,
        key: format!("running:{}", seen_key(item).replace('\0', ":")),
        rule: Some(item.rule.clone()),
        severity: item.severity,
        summary: if item.standing == Standing::Changed {
            format!("{} (changed since you marked it as known)", item.title)
        } else {
            item.title.clone()
        },
        evidence,
        link: Some(Link::Report {
            id: report.id.clone(),
        }),
    }
}

/// The rows to post: those to review that the last run did not have.
pub fn fresh<'a>(items: &'a [RunningItem], previous: &BTreeSet<String>) -> Vec<&'a RunningItem> {
    items
        .iter()
        .filter(|i| i.standing != Standing::Known && i.severity != Severity::Info)
        .filter(|i| !previous.contains(&seen_key(i)))
        .collect()
}

impl Source for Running {
    fn name(&self) -> &'static str {
        "running"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(600)
    }
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap> {
        let Some(snapshot) = (self.snapshot)() else {
            return Ok(Collected::default());
        };
        let mut items = running::assess(&snapshot);
        let marked =
            known::load(ctx.store).map_err(|e| gap(SkipReason::Failed, format!("{e:#}")))?;
        running::apply_known(&mut items, &marked);
        let previous: BTreeSet<String> = ctx
            .db
            .setting(SEEN)
            .ok()
            .flatten()
            .and_then(|v| serde_json::from_str(&v).ok())
            .unwrap_or_default();
        let now: BTreeSet<String> = items
            .iter()
            .filter(|i| i.standing != Standing::Known && i.severity != Severity::Info)
            .map(seen_key)
            .collect();
        let new = fresh(&items, &previous);
        if new.is_empty() {
            if now != previous {
                let _ = ctx
                    .db
                    .set_setting(SEEN, &serde_json::to_string(&now).unwrap_or_default());
            }
            return Ok(Collected::default());
        }
        let report = running::save(ctx.store, &items, &snapshot)
            .map_err(|e| gap(SkipReason::Failed, format!("{e:#}")))?;
        // The report belongs to these entries, not to "Checks you run".
        let _ = ctx.db.touch(
            &format!("report-id:{}", report.id),
            SourceId::Scan,
            report.created,
        );
        let observations = new
            .into_iter()
            .map(|i| observation(ctx.now, i, &report))
            .collect();
        let _ = ctx
            .db
            .set_setting(SEEN, &serde_json::to_string(&now).unwrap_or_default());
        Ok(Collected {
            observations,
            samples: vec![],
        })
    }
}
