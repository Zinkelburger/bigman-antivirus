//! Gaps in Bigman's own monitoring, on the timeline. See `checks::heartbeat`.
use super::{Collected, Ctx, Gap, Source};
use crate::{
    checks::{
        canary,
        heartbeat::{self, GapEvent, Tick, Tracker},
    },
    model::{Observation, Report, SourceId},
    platform::Clock,
};
use std::time::Duration;

/// Where the daemon records a clean shutdown (see bigmand's main).
pub const DAEMON_STOPPED: &str = "daemon.stopped";

pub struct Heartbeat {
    clock: Box<dyn Clock>,
}

impl Heartbeat {
    pub fn new(clock: Box<dyn Clock>) -> Self {
        Heartbeat { clock }
    }
}

impl Source for Heartbeat {
    fn name(&self) -> &'static str {
        "heartbeat"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(5)
    }
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap> {
        let setting = |k: &str| ctx.db.setting(k).ok().flatten();
        let mut tracker: Tracker = setting(Tracker::KEY)
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        let tick = Tick {
            now: ctx.now,
            boot: self.clock.boot_id(),
            asleep: self.clock.asleep_secs(),
            monitor_updated: ctx
                .store
                .read::<canary::Health>("monitor-health.json")
                .ok()
                .map(|h| h.updated),
            monitor_expected: canary::setup::wanted(ctx.store)
                && canary::list(ctx.store).is_ok_and(|c| !c.is_empty()),
            daemon_stopped: setting(DAEMON_STOPPED).and_then(|s| s.parse().ok()),
        };
        let events = tracker.step(&tick);
        let _ = ctx.db.set_setting(
            Tracker::KEY,
            &serde_json::to_string(&tracker).unwrap_or_default(),
        );
        let observations = events
            .iter()
            .filter_map(|e| {
                let mut r = Report::new("Monitoring", "heartbeat");
                heartbeat::finding(&mut r, e);
                let f = r.findings.pop()?;
                let key = match e {
                    GapEvent::MonitorStopped { .. } => "monitor:stopped",
                    GapEvent::MonitorGap { .. } => "monitor:gap",
                    GapEvent::DaemonGap { .. } => "daemon:gap",
                };
                Some(Observation {
                    at: ctx.now,
                    source: SourceId::Canary,
                    key: key.into(),
                    rule: Some(f.rule),
                    severity: f.severity,
                    summary: f.summary,
                    evidence: f.evidence,
                    link: None,
                })
            })
            .collect();
        Ok(Collected {
            observations,
            ..Default::default()
        })
    }
}
