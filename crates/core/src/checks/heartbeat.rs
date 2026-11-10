//! Gaps in monitoring, shown as findings rather than as silence.
//!
//! The canary monitor rewrites its heartbeat about twice a second and bigmand runs
//! the tracker every few seconds, so "nothing happened" can be told apart from
//! "nothing was watching". Time the computer spent suspended is not a gap, and a
//! reboot starts over. Malware running as you can stop either process; it cannot
//! stop the other one from noticing afterwards without also stopping that.
use crate::model::{Report, Severity};
use serde::{Deserialize, Serialize};

/// Awake seconds without a canary heartbeat before it counts as stopped.
pub const MONITOR_GAP_SECS: u64 = 60;
/// Awake seconds between two daemon runs before it counts as a gap.
pub const DAEMON_GAP_SECS: u64 = 120;
/// A heartbeat older than this is not fresh (the same as `monitor.stopped` in doctor).
pub const FRESH_SECS: u64 = 10;

/// What the daemon sees on one run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tick {
    pub now: u64,
    pub boot: Option<String>,
    /// Seconds suspended since boot.
    pub asleep: u64,
    /// When the canary monitor last wrote its heartbeat.
    pub monitor_updated: Option<u64>,
    /// Whether the monitor should be running: switched on, with decoys registered.
    pub monitor_expected: bool,
    /// When the daemon last shut down cleanly.
    pub daemon_stopped: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GapEvent {
    /// The monitor's heartbeat stopped. `since` is the last one seen.
    MonitorStopped { since: Option<u64>, now: u64 },
    /// The monitor is back after being off for `awake` seconds of awake time.
    MonitorGap { from: u64, to: u64, awake: u64 },
    /// bigmand itself was not running for `awake` seconds of awake time.
    DaemonGap {
        from: u64,
        to: u64,
        awake: u64,
        clean: bool,
    },
}

/// Persisted between runs and across daemon restarts.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tracker {
    boot: Option<String>,
    daemon_last: Option<u64>,
    daemon_asleep: u64,
    monitor_last: Option<u64>,
    down_since: Option<u64>,
    down_asleep: u64,
    stopped_reported: bool,
}

fn awake(from: u64, from_asleep: u64, to: u64, to_asleep: u64) -> u64 {
    to.saturating_sub(from)
        .saturating_sub(to_asleep.saturating_sub(from_asleep))
}

impl Tracker {
    pub const KEY: &'static str = "heartbeat.tracker";

    pub fn step(&mut self, t: &Tick) -> Vec<GapEvent> {
        let mut events = vec![];
        if self.boot != t.boot {
            // A new boot: earlier gaps belong to the previous one. Start counting the
            // monitor from now, so one that never starts after login is still noticed.
            *self = Tracker {
                boot: t.boot.clone(),
                ..Default::default()
            };
            if t.monitor_expected {
                self.down_since = Some(t.now);
                self.down_asleep = t.asleep;
            }
        } else if let Some(last) = self.daemon_last {
            let gap = awake(last, self.daemon_asleep, t.now, t.asleep);
            if gap >= DAEMON_GAP_SECS {
                events.push(GapEvent::DaemonGap {
                    from: last,
                    to: t.now,
                    awake: gap,
                    // Stopped within a few seconds of its last run means a clean stop.
                    clean: t
                        .daemon_stopped
                        .is_some_and(|s| s + 15 >= last && s <= t.now),
                });
                // The monitor runs inside bigmand, so this gap already covers it.
                self.monitor_last = None;
                self.down_since = t.monitor_expected.then_some(t.now);
                self.down_asleep = t.asleep;
                self.stopped_reported = false;
            }
        }
        let previous_asleep = self.daemon_asleep;
        self.daemon_last = Some(t.now);
        self.daemon_asleep = t.asleep;

        if !t.monitor_expected {
            self.monitor_last = None;
            self.down_since = None;
            self.stopped_reported = false;
            return events;
        }
        let fresh = t
            .monitor_updated
            .filter(|u| t.now.saturating_sub(*u) < FRESH_SECS);
        if let Some(updated) = fresh {
            if let Some(since) = self.down_since.take() {
                let gap = awake(since, self.down_asleep, t.now, t.asleep);
                if gap >= MONITOR_GAP_SECS {
                    events.push(GapEvent::MonitorGap {
                        from: since,
                        to: t.now,
                        awake: gap,
                    });
                }
            }
            self.stopped_reported = false;
            self.monitor_last = Some(updated);
        } else {
            if self.down_since.is_none() {
                // The last heartbeat came before this run, so measure suspend from the
                // previous run rather than from now.
                self.down_since = Some(self.monitor_last.unwrap_or(t.now));
                self.down_asleep = previous_asleep.min(t.asleep);
            }
            let since = self.down_since.unwrap_or(t.now);
            if !self.stopped_reported
                && awake(since, self.down_asleep, t.now, t.asleep) >= MONITOR_GAP_SECS
            {
                events.push(GapEvent::MonitorStopped {
                    since: self.monitor_last,
                    now: t.now,
                });
                self.stopped_reported = true;
            }
        }
        events
    }
}

/// "38 minutes", "2 hours 5 minutes".
pub fn duration(secs: u64) -> String {
    let plural = |n: u64, unit: &str| format!("{n} {unit}{}", if n == 1 { "" } else { "s" });
    let (h, m) = (secs / 3600, (secs % 3600) / 60);
    match (h, m) {
        (0, 0) => plural(secs, "second"),
        (0, m) => plural(m, "minute"),
        (h, 0) => plural(h, "hour"),
        (h, m) => format!("{} {}", plural(h, "hour"), plural(m, "minute")),
    }
}

/// Unix seconds as `2026-11-02T18:02:11Z`.
pub fn utc(secs: u64) -> String {
    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    // Civil-from-days (H. Hinnant), valid for every date after 1970.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

/// The finding for one gap event.
pub fn finding(r: &mut Report, e: &GapEvent) {
    match *e {
        GapEvent::MonitorStopped { since, now } => r.add(
            "monitor.stopped",
            Severity::Review,
            "Canary monitoring stopped reporting",
            match since {
                Some(s) => format!("Last heartbeat {}; still none at {}.", utc(s), utc(now)),
                None => format!("No heartbeat since this login; still none at {}.", utc(now)),
            },
            "Start monitoring from Protect > Canaries. If you did not stop it, check what else changed since then.",
        ),
        GapEvent::MonitorGap { from, to, awake } => r.add(
            "monitor.gap",
            Severity::Review,
            &format!("Canary monitoring was off for {} while this computer was awake", duration(awake)),
            format!("No heartbeat from {} to {}. Decoy reads in that time were not seen.", utc(from), utc(to)),
            "If you did not stop monitoring, review Activity and the startup files for changes made in that window.",
        ),
        GapEvent::DaemonGap { from, to, awake, clean } => r.add(
            "monitor.gap",
            if clean { Severity::Info } else { Severity::Review },
            &if clean {
                format!("Bigman's background service was stopped for {}", duration(awake))
            } else {
                format!("Bigman's background service ended without shutting down and was off for {}", duration(awake))
            },
            format!(
                "Last run {}, next run {}. {}",
                utc(from),
                utc(to),
                if clean {
                    "It stopped normally, as at logout or after systemctl --user stop."
                } else {
                    "It did not record a normal stop: it crashed or was killed."
                }
            ),
            "Live activity in that window was not recorded. If you did not stop it, check the startup files and Activity for changes from that time.",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tick(now: u64, asleep: u64, monitor: Option<u64>) -> Tick {
        Tick {
            now,
            boot: Some("b1".into()),
            asleep,
            monitor_updated: monitor,
            monitor_expected: true,
            daemon_stopped: None,
        }
    }

    /// Runs ticks every 5 seconds from `from` to `to`, with the monitor alive or not.
    fn run(t: &mut Tracker, from: u64, to: u64, asleep: u64, alive: bool) -> Vec<GapEvent> {
        (from..=to)
            .step_by(5)
            .flat_map(|now| t.step(&tick(now, asleep, alive.then_some(now))))
            .collect()
    }

    #[test]
    fn a_stop_is_reported_once_and_the_gap_when_it_returns() {
        let mut t = Tracker::default();
        assert!(run(&mut t, 1000, 1100, 0, true).is_empty());
        let stopped = run(&mut t, 1105, 1400, 0, false);
        assert_eq!(
            stopped,
            [GapEvent::MonitorStopped {
                since: Some(1100),
                now: 1160
            }]
        );
        let back = run(&mut t, 1405, 1500, 0, true);
        assert_eq!(
            back,
            [GapEvent::MonitorGap {
                from: 1100,
                to: 1405,
                awake: 305
            }]
        );
    }

    #[test]
    fn suspend_is_not_a_gap() {
        let mut t = Tracker::default();
        run(&mut t, 1000, 1100, 0, true);
        // Asleep for an hour: neither process ran, and boottime moved on.
        let after = run(&mut t, 4700, 4800, 3600, true);
        assert!(after.is_empty(), "{after:?}");
    }

    #[test]
    fn short_restarts_are_quiet() {
        let mut t = Tracker::default();
        run(&mut t, 1000, 1100, 0, true);
        assert!(run(&mut t, 1105, 1130, 0, false).is_empty());
        assert!(run(&mut t, 1135, 1200, 0, true).is_empty());
    }

    #[test]
    fn daemon_gaps_tell_clean_stops_from_kills() {
        let mut t = Tracker::default();
        run(&mut t, 1000, 1100, 0, true);
        let mut back = tick(2000, 0, Some(2000));
        let killed = t.clone().step(&back);
        assert!(matches!(
            killed[0],
            GapEvent::DaemonGap {
                clean: false,
                awake: 900,
                ..
            }
        ));
        back.daemon_stopped = Some(1102);
        assert!(matches!(
            t.step(&back)[0],
            GapEvent::DaemonGap { clean: true, .. }
        ));
    }

    #[test]
    fn a_reboot_starts_over_and_a_monitor_that_never_starts_is_noticed() {
        let mut t = Tracker::default();
        run(&mut t, 1000, 1100, 0, true);
        let mut first = tick(90_000, 0, None);
        first.boot = Some("b2".into());
        assert!(
            t.step(&first).is_empty(),
            "the old boot's gap is not reported"
        );
        let mut later = tick(90_065, 0, None);
        later.boot = Some("b2".into());
        assert_eq!(
            t.step(&later),
            [GapEvent::MonitorStopped {
                since: None,
                now: 90_065
            }]
        );
    }

    #[test]
    fn nothing_is_expected_without_decoys() {
        let mut t = Tracker::default();
        let mut quiet = tick(1000, 0, None);
        quiet.monitor_expected = false;
        for now in (1000..2000).step_by(5) {
            quiet.now = now;
            assert!(t.step(&quiet).is_empty());
        }
    }

    #[test]
    fn utc_and_durations() {
        assert_eq!(utc(0), "1970-01-01T00:00:00Z");
        assert_eq!(utc(1_793_642_977), "2026-11-02T18:09:37Z");
        assert_eq!(duration(305), "5 minutes");
        assert_eq!(duration(7500), "2 hours 5 minutes");
        assert_eq!(duration(42), "42 seconds");
    }
}
