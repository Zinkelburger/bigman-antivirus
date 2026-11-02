//! When the daemon raises a desktop notification. Deterministic and quiet: entries
//! gather for 30 seconds into one notification, at most one a minute, and only for
//! the categories the user keeps on. Activity always has everything.
use crate::model::{Event, Severity, SourceId};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Gather this long after the first entry before notifying.
pub const BATCH_SECS: u64 = 30;
/// Never notify more often than this.
pub const MIN_GAP_SECS: u64 = 60;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct NotificationSettings {
    /// Anything rated Review or High, from any source.
    pub findings: bool,
    /// A program on the network for the first time, or reaching somewhere new.
    pub new_connections: bool,
    /// A program starting to listen for connections.
    pub new_listeners: bool,
    /// Package installs, updates and removals.
    pub packages: bool,
    /// SELinux and AppArmor stopping a program.
    pub denials: bool,
    /// Unix seconds until which nothing is shown.
    pub paused_until: Option<u64>,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        NotificationSettings {
            findings: true,
            new_connections: false,
            new_listeners: false,
            packages: false,
            denials: false,
            paused_until: None,
        }
    }
}

impl NotificationSettings {
    pub const KEY: &'static str = "notifications";

    pub fn wants(&self, e: &Event) -> bool {
        if e.severity != Severity::Info {
            return self.findings;
        }
        // Repeats are in Activity's count, not worth a notification.
        if e.seen_before > 0 {
            return false;
        }
        match e.source {
            SourceId::Network => self.new_connections,
            SourceId::Listener => self.new_listeners,
            SourceId::Receipt => self.packages,
            SourceId::Denial => self.denials,
            _ => false,
        }
    }

    pub fn paused(&self, now: u64) -> bool {
        self.paused_until.is_some_and(|until| now < until)
    }
}

/// Entries waiting to be announced.
#[derive(Debug, Default)]
pub struct Batch {
    pending: Vec<Event>,
    since: Option<u64>,
    last_sent: Option<u64>,
}

/// One notification: a title and a body of plain sentences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notice {
    pub title: String,
    pub body: String,
}

impl Batch {
    pub fn push(&mut self, settings: &NotificationSettings, e: &Event, now: u64) {
        if settings.wants(e) {
            self.since.get_or_insert(now);
            self.pending.push(e.clone());
        }
    }

    /// The notification to show now, if one is due. While paused, entries are
    /// dropped rather than saved up for a burst when the pause ends.
    pub fn due(&mut self, settings: &NotificationSettings, now: u64) -> Option<Notice> {
        let since = self.since?;
        if settings.paused(now) {
            self.pending.clear();
            self.since = None;
            return None;
        }
        if now < since + BATCH_SECS || self.last_sent.is_some_and(|t| now < t + MIN_GAP_SECS) {
            return None;
        }
        let events = std::mem::take(&mut self.pending);
        self.since = None;
        self.last_sent = Some(now);
        Some(notice(&events))
    }
}

fn notice(events: &[Event]) -> Notice {
    let to_review = events
        .iter()
        .filter(|e| e.severity != Severity::Info)
        .count();
    let title = match (to_review, events.len()) {
        (0, 1) => "1 new entry in Activity".to_string(),
        (0, n) => format!("{n} new entries in Activity"),
        (1, _) => "1 entry to review".to_string(),
        (n, _) => format!("{n} entries to review"),
    };
    // Most severe first, then oldest first; three lines at most.
    let mut ordered: Vec<&Event> = events.iter().collect();
    ordered.sort_by_key(|e| (std::cmp::Reverse(e.severity), e.at));
    let mut lines: Vec<String> = ordered.iter().take(3).map(|e| e.summary.clone()).collect();
    if events.len() > 3 {
        lines.push(format!("and {} more", events.len() - 3));
    }
    Notice {
        title,
        body: lines.join("\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(source: SourceId, severity: Severity, seen_before: u32, summary: &str) -> Event {
        Event {
            id: 1,
            at: 0,
            source,
            key: summary.into(),
            rule: None,
            severity,
            summary: summary.into(),
            evidence: String::new(),
            seen_before,
            first_at: 0,
            link: None,
        }
    }

    #[test]
    fn findings_notify_by_default_and_everything_else_is_opt_in() {
        let s = NotificationSettings::default();
        assert!(s.wants(&event(SourceId::Listener, Severity::High, 3, "dockerd")));
        assert!(!s.wants(&event(SourceId::Network, Severity::Info, 0, "curl")));
        let s = NotificationSettings {
            new_connections: true,
            ..s
        };
        assert!(s.wants(&event(SourceId::Network, Severity::Info, 0, "curl")));
        assert!(
            !s.wants(&event(SourceId::Network, Severity::Info, 2, "curl")),
            "repeats never notify"
        );
    }

    #[test]
    fn entries_gather_for_thirty_seconds_and_notify_at_most_once_a_minute() {
        let s = NotificationSettings::default();
        let mut b = Batch::default();
        b.push(
            &s,
            &event(
                SourceId::Listener,
                Severity::High,
                0,
                "dockerd is listening on 2375",
            ),
            100,
        );
        b.push(
            &s,
            &event(
                SourceId::Scan,
                Severity::Review,
                0,
                "Scanned Downloads · 1 finding to review",
            ),
            110,
        );
        assert_eq!(b.due(&s, 129), None);
        let n = b.due(&s, 130).unwrap();
        assert_eq!(n.title, "2 entries to review");
        assert_eq!(
            n.body,
            "dockerd is listening on 2375\nScanned Downloads · 1 finding to review"
        );
        b.push(
            &s,
            &event(SourceId::Scan, Severity::Review, 0, "later"),
            140,
        );
        assert_eq!(b.due(&s, 175), None, "within a minute of the last one");
        assert!(b.due(&s, 190).is_some());
    }

    #[test]
    fn a_pause_drops_entries_instead_of_saving_a_burst() {
        let s = NotificationSettings {
            paused_until: Some(1000),
            ..Default::default()
        };
        let mut b = Batch::default();
        b.push(&s, &event(SourceId::Scan, Severity::High, 0, "x"), 100);
        assert_eq!(b.due(&s, 200), None);
        assert_eq!(b.due(&s, 1100), None);
    }
}
