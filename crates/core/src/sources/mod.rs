//! Timeline sources. The daemon runs each on its interval; a source turns what it
//! sees into Observations (numbered and counted by the store) and Samples. Adding a
//! source is one module here and its fixtures; the UI renders any Event.
mod connections;
mod denials;
mod downloads;
mod heartbeat;
mod listeners;
mod packages;
mod preferences;
mod projects;
mod reports;
mod running;
mod skills;
mod snapshot;
mod ssh;
mod throughput;
mod traffic;
mod updates;

pub use connections::Connections;
pub use denials::Denials;
pub use downloads::Downloads;
pub use heartbeat::{Heartbeat, DAEMON_STOPPED};
pub use listeners::Listeners;
pub use packages::{transaction_summary, Packages};
pub use preferences::Preferences;
pub use projects::Projects;
pub use reports::Reports;
pub use running::Running;
pub use skills::Skills;
pub use snapshot::{network_snapshot, ConnectionRow, ListenerRow, NetworkSnapshot};
pub use ssh::{AuthorizedKeys, RemoteLogins, StrayKeys, Tunnels};
pub use throughput::Throughput;
pub use traffic::{traffic_report, Traffic, STEP as TRAFFIC_STEP};
pub use updates::Updates;

use crate::{
    model::{FlowSample, Observation, Sample, SkipReason},
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
    pub traffic: Vec<FlowSample>,
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
    /// Runs instead of `collect` while Preferences turn this source off. Sources
    /// that watch let go here, so turning them back on starts from then rather
    /// than catching up on what arrived while they were off.
    fn turned_off(&mut self, _ctx: &Ctx) {}
}

/// A source's health, for Setup and the coverage of the timeline.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct SourceStatus {
    pub name: String,
    pub label: String,
    pub last_run: Option<u64>,
    pub ok: bool,
    /// Turned off in Setup. Not a gap: nothing is being missed by accident.
    pub off: bool,
    pub why: Option<SkipReason>,
    pub detail: String,
}

pub fn label(name: &str) -> &'static str {
    match name {
        "throughput" => "Network throughput",
        "traffic" => "Traffic by program",
        "connections" => "Your programs' connections",
        "listeners" => "Listening programs",
        "packages" => "Package installs and updates",
        "denials" => "SELinux and AppArmor denials",
        "reports" => "Checks you run",
        "downloads" => "New files in Downloads",
        "projects" => "New project folders",
        "updates" => "Security updates, daily and after package changes",
        "ssh_keys" => "SSH keys that can log in",
        "ssh_tunnels" => "SSH tunnels back to this computer",
        "ssh_logins" => "Network logins",
        "ssh_stray_keys" => "SSH private keys outside ~/.ssh",
        "heartbeat" => "Gaps in Bigman's monitoring",
        "skills" => "AI skills and plugins, as they are installed or change",
        "running" => "Running programs, kernel modules and sockets, every ten minutes",
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
        Box::new(Traffic::new(
            Box::<crate::platform::sockdiag::SockDiag>::default(),
        )),
        Box::new(Connections::new(Box::new(linux::ProcSockets))),
        Box::new(Listeners::new(Box::new(linux::ProcSockets))),
        Box::new(Denials::new(Box::new(linux::Journalctl {
            stop: stop.clone(),
        }))),
        Box::new(Downloads::new(
            crate::checks::setup::downloads(),
            stop.clone(),
        )),
        Box::new(Projects::new(projects::roots(), stop.clone())),
        Box::<Reports>::default(),
        Box::<Updates>::default(),
        Box::new(AuthorizedKeys::for_home()),
        Box::new(Tunnels::new(Box::new(linux::ProcList))),
        Box::new(RemoteLogins::new(
            Box::new(linux::Loginctl { stop: stop.clone() }),
            Box::new(linux::KernelClock),
        )),
        Box::new(StrayKeys::default_dirs()),
        Box::new(Heartbeat::new(Box::new(linux::KernelClock))),
        Box::new(Skills::new(crate::checks::skills::Places::current())),
        Box::new(Running::new(stop.clone())),
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
