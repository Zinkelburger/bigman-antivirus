use super::Severity;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// What noticed something. Drives the Activity filter chips; adding a source
/// needs no UI change.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum SourceId {
    Network,
    Listener,
    Receipt,
    Startup,
    Canary,
    Denial,
    Flatpak,
    Container,
    Scan,
    Download,
    /// A project folder that was cloned or unpacked.
    Project,
    Advisory,
    Extension,
    Import,
    /// SSH keys, tunnels and network logins.
    Ssh,
    /// AI tool skills and plugins that appeared or changed.
    Skill,
}

impl SourceId {
    pub fn as_str(self) -> &'static str {
        match self {
            SourceId::Network => "network",
            SourceId::Listener => "listener",
            SourceId::Receipt => "receipt",
            SourceId::Startup => "startup",
            SourceId::Canary => "canary",
            SourceId::Denial => "denial",
            SourceId::Flatpak => "flatpak",
            SourceId::Container => "container",
            SourceId::Scan => "scan",
            SourceId::Download => "download",
            SourceId::Project => "project",
            SourceId::Advisory => "advisory",
            SourceId::Extension => "extension",
            SourceId::Import => "import",
            SourceId::Ssh => "ssh",
            SourceId::Skill => "skill",
        }
    }
    pub fn parse(s: &str) -> Option<SourceId> {
        ALL_SOURCES.iter().copied().find(|id| id.as_str() == s)
    }
}

pub const ALL_SOURCES: [SourceId; 16] = [
    SourceId::Network,
    SourceId::Listener,
    SourceId::Receipt,
    SourceId::Startup,
    SourceId::Canary,
    SourceId::Denial,
    SourceId::Flatpak,
    SourceId::Container,
    SourceId::Scan,
    SourceId::Download,
    SourceId::Project,
    SourceId::Advisory,
    SourceId::Extension,
    SourceId::Import,
    SourceId::Ssh,
    SourceId::Skill,
];

/// Where an event's evidence lives.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[ts(export)]
pub enum Link {
    Report { id: String },
    Receipt { id: String },
}

/// Something a source noticed, before the store numbers it and counts repeats.
#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    pub at: u64,
    pub source: SourceId,
    /// What makes two observations "the same thing" for first-seen counting.
    pub key: String,
    /// The rule behind it, when it is a finding.
    pub rule: Option<String>,
    pub severity: Severity,
    /// One plain sentence, e.g. "Firefox connected to a new host".
    pub summary: String,
    /// Verbatim detail, cleaned like report evidence.
    pub evidence: String,
    pub link: Option<Link>,
}

/// One row of the timeline.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct Event {
    pub id: i64,
    pub at: u64,
    pub source: SourceId,
    pub key: String,
    pub rule: Option<String>,
    pub severity: Severity,
    pub summary: String,
    pub evidence: String,
    /// Times this key was seen before; 0 means first seen.
    pub seen_before: u32,
    /// When this key was first seen.
    pub first_at: u64,
    pub link: Option<Link>,
}

/// Throughput of one interface over one interval, in bytes per second.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct Sample {
    pub at: u64,
    pub iface: String,
    pub rx: f64,
    pub tx: f64,
}

/// What arrived since the user last marked Activity as seen.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default, TS)]
#[ts(export)]
pub struct Unread {
    /// Things seen for the first time.
    pub first_seen: u32,
    /// Entries rated Review or High.
    pub to_review: u32,
    /// The newest event id counted; pass it back to mark exactly these as seen.
    pub through: i64,
    /// The id everything up to was already seen; passing it back undoes a mark.
    pub seen_through: i64,
}

/// Throughput samples averaged into buckets of `bucket` seconds. A missing bucket
/// is a time the daemon was not running.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct SampleSeries {
    pub bucket: u64,
    pub samples: Vec<Sample>,
}
