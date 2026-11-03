//! Timeline sources. The daemon runs each on its interval; a source turns what it
//! sees into Observations (numbered and counted by the store) and Samples. Adding a
//! source is one module here and its fixtures; the UI renders any Event.
mod connections;
mod denials;
mod downloads;
mod listeners;
mod packages;
mod reports;
mod skills;
mod snapshot;
mod throughput;
mod updates;

pub use connections::Connections;
pub use denials::Denials;
pub use downloads::Downloads;
pub use listeners::Listeners;
pub use packages::{transaction_summary, Packages};
pub use reports::Reports;
pub use skills::Skills;
pub use snapshot::{network_snapshot, ConnectionRow, ListenerRow, NetworkSnapshot};
pub use throughput::Throughput;
pub use updates::{Preferences, Updates};

use crate::{
    model::{Observation, Sample, SkipReason},
    store::{db::Db, Store},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use ts_rs::TS;

/// What a source needs on each run.
pub struct Ctx<'a> {
    pub db: &'a Db,
    pub store: &'a Store,
    pub now: u64,
}

#[derive(Default)]
pub struct Collected {
    pub observations: Vec<Observation>,
    pub samples: Vec<Sample>,
}

/// Why a source could not see what it is meant to.
#[derive(Debug, Clone)]
pub struct Gap {
    pub why: SkipReason,
    pub detail: String,
}

pub trait Source: Send {
    /// Stable name, shown in Setup and used for cursors.
    fn name(&self) -> &'static str;
    fn interval(&self) -> Duration;
    /// One run. `Err(Gap)` when the source cannot see anything right now, which is
    /// reported as incomplete coverage rather than as "nothing happened".
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap>;
}

/// A source's health, for Setup and the coverage of the timeline.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct SourceStatus {
    pub name: String,
    pub label: String,
    pub last_run: Option<u64>,
    pub ok: bool,
    pub why: Option<SkipReason>,
    pub detail: String,
}

pub fn label(name: &str) -> &'static str {
    match name {
        "throughput" => "Network throughput",
        "connections" => "Your programs' connections",
        "listeners" => "Listening programs",
        "packages" => "Package installs and updates",
        "denials" => "SELinux and AppArmor denials",
        "reports" => "Checks you run",
        "downloads" => "New files in Downloads",
        "updates" => "Security updates, daily and after package changes",
        "skills" => "AI skills and plugins, as they are installed or change",
        _ => "Other",
    }
}

/// Sources a cursor lets resume where they stopped, across daemon restarts.
fn cursor(db: &Db, source: &str) -> Option<String> {
    db.setting(&format!("cursor.{source}")).ok().flatten()
}

fn set_cursor(db: &Db, source: &str, value: &str) {
    let _ = db.set_setting(&format!("cursor.{source}"), value);
}

fn gap(why: SkipReason, detail: impl Into<String>) -> Gap {
    Gap {
        why,
        detail: detail.into(),
    }
}

/// The default source set for this machine.
pub fn linux_sources(stop: crate::runner::Cancel) -> Vec<Box<dyn Source>> {
    use crate::platform::linux;
    let mut sources: Vec<Box<dyn Source>> = vec![
        Box::new(Throughput::new(Box::new(linux::SysNetCounters))),
        Box::new(Connections::new(Box::new(linux::ProcSockets))),
        Box::new(Listeners::new(Box::new(linux::ProcSockets))),
        Box::new(Denials::new(Box::new(linux::Journalctl {
            stop: stop.clone(),
        }))),
        Box::new(Downloads::new(crate::checks::setup::downloads(), stop)),
        Box::<Reports>::default(),
        Box::<Updates>::default(),
        Box::new(Skills::new(crate::checks::skills::Places::current())),
    ];
    if let Some(history) = linux::package_history() {
        sources.push(Box::new(Packages::new(
            history,
            crate::platform::packagedb::package_db(),
        )));
    }
    sources
}

#[cfg(test)]
mod tests;
