use super::{gap, Collected, Ctx, Gap, Source};
use crate::{
    checks::skills::{self, Places, SkillItem, Status},
    model::{Link, Observation, Report, Severity, SkipReason, SourceId},
};
use std::{collections::BTreeSet, os::unix::fs::MetadataExt, time::Duration};

const SEEN: &str = "skills.seen";
/// Entries looked at per fingerprint, across every skill and plugin folder.
const MAX_ENTRIES: usize = 50_000;

/// Skills and plugins that appear or change. Every few minutes a cheap
/// fingerprint (names, sizes and modification times) of each folder is
/// compared with the last; when it moves, the inventory runs and each skill or
/// plugin that is new, or different from what was there, gets one entry. The
/// first run records what is installed without posting it.
pub struct Skills {
    places: Places,
    last: Option<String>,
}

impl Skills {
    pub fn new(places: Places) -> Self {
        Self { places, last: None }
    }
}

fn fingerprint(units: &[skills::Unit]) -> String {
    let mut lines = String::new();
    let mut seen = 0;
    for u in units {
        lines.push_str(&format!(
            "{:?}\0{}\0{:?}\n",
            u.kind,
            u.root.display(),
            u.source
        ));
        for e in walkdir::WalkDir::new(&u.root)
            .max_depth(skills::MAX_DEPTH)
            .sort_by_file_name()
            .into_iter()
            .filter_entry(|e| e.file_name() != "node_modules" && e.file_name() != ".git")
            .flatten()
        {
            seen += 1;
            if seen > MAX_ENTRIES {
                break;
            }
            if let Ok(m) = e.metadata() {
                lines.push_str(&format!(
                    "{}\0{}\0{}\0{}\0{:o}\n",
                    e.path().display(),
                    m.len(),
                    m.mtime(),
                    m.mtime_nsec(),
                    m.mode()
                ));
            }
        }
    }
    crate::store::digest(lines.as_bytes())
}

/// One timeline entry for a skill or plugin that is new or changed.
pub fn observation(at: u64, item: &SkillItem, before: bool, report: &Report) -> Observation {
    let label = format!(
        "{} {}",
        item.tools.first().map_or("AI tool", |t| t.label()),
        item.kind.label()
    );
    let to_review = item
        .findings
        .iter()
        .filter(|f| f.severity != Severity::Info)
        .count();
    let outcome = match to_review {
        0 => "no findings".to_string(),
        1 => "1 finding to review".to_string(),
        n => format!("{n} findings to review"),
    };
    let summary = match (item.status, before) {
        (Status::Changed, _) => format!("{} changed since you approved it · {outcome}", item.name),
        (_, true) => format!("{} changed · {outcome}", item.name),
        _ => format!("New {label}: {} · {outcome}", item.name),
    };
    let severity = item
        .findings
        .iter()
        .map(|f| f.severity)
        .max()
        .unwrap_or(Severity::Info)
        .max(if item.status == Status::Changed {
            Severity::Review
        } else {
            Severity::Info
        });
    let mut evidence = format!("{label}\n{}", item.root.display());
    for run in item.runs.iter().take(10) {
        evidence.push_str(&format!("\nRuns without asking: {run}"));
    }
    if !item.changed.is_empty() {
        evidence.push_str(&format!("\nChanged: {}", item.changed.join(", ")));
    }
    Observation {
        at,
        source: SourceId::Skill,
        key: format!("skill:{}:{}", item.key, item.hash),
        rule: item
            .findings
            .iter()
            .max_by_key(|f| f.severity)
            .map(|f| f.rule.clone())
            .or_else(|| (item.status == Status::Changed).then(|| "skill.changed".to_string())),
        severity,
        summary,
        evidence,
        link: Some(Link::Report {
            id: report.id.clone(),
        }),
    }
}

impl Source for Skills {
    fn name(&self) -> &'static str {
        "skills"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(120)
    }
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap> {
        let units = skills::discover(&self.places);
        let fp = fingerprint(&units);
        if self.last.as_deref() == Some(fp.as_str()) {
            return Ok(Collected::default());
        }
        let previous: Option<BTreeSet<String>> = ctx
            .db
            .setting(SEEN)
            .ok()
            .flatten()
            .and_then(|v| serde_json::from_str(&v).ok());
        let subject = "Installed skills and plugins".to_string();
        super::downloads::IN_FLIGHT
            .lock()
            .unwrap()
            .insert(subject.clone());
        let result = skills::inventory_at(ctx.store, &self.places);
        if let Ok((r, _)) = &result {
            let _ = ctx
                .db
                .touch(&format!("report-id:{}", r.id), SourceId::Scan, r.created);
        }
        super::downloads::IN_FLIGHT.lock().unwrap().remove(&subject);
        let (report, items) = result.map_err(|e| gap(SkipReason::Failed, format!("{e:#}")))?;
        self.last = Some(fp);
        let now: BTreeSet<String> = items
            .iter()
            .map(|i| format!("{}\0{}", i.key, i.hash))
            .collect();
        let _ = ctx
            .db
            .set_setting(SEEN, &serde_json::to_string(&now).unwrap_or_default());
        let Some(previous) = previous else {
            return Ok(Collected::default());
        };
        let known: BTreeSet<&str> = previous
            .iter()
            .filter_map(|p| p.split('\0').next())
            .collect();
        let observations = items
            .iter()
            .filter(|i| !previous.contains(&format!("{}\0{}", i.key, i.hash)))
            .map(|i| observation(ctx.now, i, known.contains(i.key.as_str()), &report))
            .collect();
        Ok(Collected {
            observations,
            samples: vec![],
        })
    }
}
