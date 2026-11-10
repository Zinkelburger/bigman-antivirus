use super::{gap, Collected, Ctx, Gap, Source};
use crate::model::{Link, Observation, Report, Severity, SkipReason, SourceId};
use std::time::Duration;

/// Every saved report, from the CLI or the daemon, enters the timeline.
#[derive(Default)]
pub struct Reports {
    changed: Option<std::time::SystemTime>,
}

const SEEDED: &str = "cursor.reports";

/// The same check of the same thing with the same kind of result folds together;
/// a run with findings never folds into later clean runs.
pub fn report_key(r: &Report) -> String {
    let outcome = if r.is_incomplete() {
        "incomplete".to_string()
    } else {
        r.findings
            .iter()
            .map(|f| f.severity)
            .filter(|s| *s != Severity::Info)
            .max()
            .map_or("clear".to_string(), |s| format!("{s:?}").to_lowercase())
    };
    format!("report:{}:{}:{outcome}", r.kind, r.subject)
}

/// One plain sentence for a report, e.g. "Scanned /home/sam/Downloads · no findings".
pub fn report_sentence(r: &Report) -> String {
    let what = match r.kind.as_str() {
        "Security checkup" => "Ran a checkup".to_string(),
        "Canary triggered" => format!("A canary file was opened: {}", r.subject),
        "File scan" => format!("Scanned {}", r.subject),
        "PDF triage" => format!("Read the PDF {}", r.subject),
        "Installer inspection" => format!("Inspected {}", r.subject),
        "Package script inventory" => format!("Listed package scripts in {}", r.subject),
        "Persistence changes" => "Compared startup files".to_string(),
        "Baseline accepted" => "Accepted the startup files as they are".to_string(),
        "Signature update" => "Updated virus signatures".to_string(),
        "Canary verification" => format!("Tested the canary {}", r.subject),
        "Local canary setup" => "Set up canary files".to_string(),
        "Monitor event loss" => "The canary monitor missed some events".to_string(),
        "YARA-X scan" => format!("Ran YARA rules on {}", r.subject),
        "Restricted installer rehearsal" => format!("Rehearsed {} in a sandbox", r.subject),
        "Stop and watch" => format!("Stopped {} and watched what started it again", r.subject),
        kind => format!("{kind}: {}", r.subject),
    };
    if r.kind == "Canary triggered" {
        what
    } else {
        format!("{what} · {}", report_outcome(r))
    }
}

/// "no findings", "2 findings to review" or "incomplete".
pub fn report_outcome(r: &Report) -> String {
    let to_review = r.summary.map(|s| s.to_review as usize).unwrap_or_else(|| {
        r.findings
            .iter()
            .filter(|f| f.severity != Severity::Info)
            .count()
    });
    if r.is_incomplete() {
        "incomplete".to_string()
    } else if to_review == 0 {
        "no findings".to_string()
    } else {
        format!(
            "{to_review} finding{} to review",
            if to_review == 1 { "" } else { "s" }
        )
    }
}

impl Source for Reports {
    fn name(&self) -> &'static str {
        "reports"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(3)
    }
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap> {
        // Saving a report rewrites its summary last, so an unchanged directory means
        // nothing new and the summaries need not be read again.
        let changed = ctx
            .store
            .path("summaries")
            .and_then(|p| Ok(std::fs::metadata(p)?.modified()?))
            .ok();
        if changed.is_some() && changed == self.changed {
            return Ok(Collected::default());
        }
        let mut reports = ctx
            .store
            .history()
            .map_err(|e| gap(SkipReason::Unreadable, format!("{e:#}")))?;
        let first_run = ctx.db.setting(SEEDED).ok().flatten().is_none();
        reports.sort_by_key(|r| r.created);
        let mut observations = vec![];
        for r in &reports {
            // The Downloads watch posts its own reports; leave them for it.
            if super::downloads::IN_FLIGHT
                .lock()
                .unwrap()
                .contains(&r.subject)
            {
                continue;
            }
            // Keyed by id, not time: a long scan saved late still arrives once.
            let id_key = format!("report-id:{}", r.id);
            if ctx.db.seen(&id_key).unwrap_or(false) {
                continue;
            }
            let _ = ctx.db.touch(&id_key, SourceId::Scan, r.created);
            // Bookkeeping from the monitor itself is not a timeline entry, and the
            // first run brings in one week rather than all history.
            let bookkeeping = matches!(
                r.kind.as_str(),
                "Monitor started" | "Monitor startup" | "Monitor stop"
            );
            if bookkeeping || (first_run && r.created + 7 * 86_400 < ctx.now) {
                continue;
            }
            observations.push(Observation {
                at: r.created,
                source: if r.kind == "Canary triggered" {
                    SourceId::Canary
                } else {
                    SourceId::Scan
                },
                key: report_key(r),
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
                summary: report_sentence(r),
                evidence: r.subject.clone(),
                link: Some(Link::Report { id: r.id.clone() }),
            });
        }
        if first_run {
            let _ = ctx.db.set_setting(SEEDED, "1");
        }
        self.changed = changed;
        Ok(Collected {
            observations,
            ..Default::default()
        })
    }
}
