use super::Coverage;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Bytes one program moved to or from one address on one service port during
/// one stored interval. What the traffic source hands the store.
#[derive(Debug, Clone, PartialEq)]
pub struct FlowSample {
    /// Start of the interval, Unix seconds.
    pub at: u64,
    /// The program's name; none when another account runs it.
    pub program: Option<String>,
    pub uid: u32,
    pub remote: std::net::IpAddr,
    /// The service's port: the remote one, or the local one for a connection that came in.
    pub port: u16,
    pub inbound: bool,
    pub rx: f64,
    pub tx: f64,
}

/// Bytes a program received and sent over a time range.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct ProgramTotal {
    /// The program's name; none when another account runs it.
    pub program: Option<String>,
    pub uid: u32,
    pub rx: f64,
    pub tx: f64,
    /// When this program was first seen on the network, if the timeline recorded it.
    pub first_at: Option<u64>,
}

/// Bytes between one program and one address on one service port over a time range.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct Flow {
    pub program: Option<String>,
    pub uid: u32,
    pub remote: String,
    /// The service's port: the remote one, or the local one when `inbound`.
    pub port: u16,
    /// The other computer opened the connection.
    pub inbound: bool,
    pub rx: f64,
    pub tx: f64,
}

/// One program's bytes per interval, aligned to `TrafficReport::at`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct ProgramSeries {
    pub program: Option<String>,
    pub uid: u32,
    pub rx: Vec<f64>,
    pub tx: Vec<f64>,
}

/// TCP traffic by program over `[since, until]`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct TrafficReport {
    pub since: u64,
    pub until: u64,
    /// The account Bigman runs as. A flow with this uid and no program is one of
    /// your own programs that could not be named, not another account's.
    pub own_uid: u32,
    /// Seconds each interval covers.
    pub bucket: u64,
    /// The start of every interval in the range, oldest first.
    pub at: Vec<u64>,
    /// Whether the collector was running during each interval. Nothing moving
    /// and nothing being counted are different things.
    pub recorded: Vec<bool>,
    /// The programs that moved the most data, most first.
    pub series: Vec<ProgramSeries>,
    /// Every other program together, per interval.
    pub rest_rx: Vec<f64>,
    pub rest_tx: Vec<f64>,
    /// Every program seen in the range, most data first.
    pub programs: Vec<ProgramTotal>,
    /// Program, address and port combinations, most data first, up to a limit.
    pub flows: Vec<Flow>,
    /// Combinations left out of `flows` because of the limit.
    pub flows_omitted: u32,
    pub coverage: Coverage,
}
