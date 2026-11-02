//! Everything OS-specific, behind small traits. Rules and sources consume the
//! parsed data these return and never touch paths or commands themselves; tests
//! substitute fakes. Only a Linux implementation exists (plan C4).
pub mod linux;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, path::PathBuf};
use ts_rs::TS;

/// Cumulative byte counters for one network interface.
#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceCounters {
    pub iface: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

pub trait NetworkCounters: Send + Sync {
    fn counters(&self) -> Result<Vec<InterfaceCounters>>;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Protocol {
    Tcp,
    Udp,
}

/// The program behind a socket. Known only for your own processes without the helper.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, TS)]
#[ts(export)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub exe: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct Socket {
    pub protocol: Protocol,
    #[ts(type = "string")]
    pub local: SocketAddr,
    #[ts(type = "string | null")]
    pub remote: Option<SocketAddr>,
    /// "listen", "established", "time_wait", ... ("unconnected" for idle UDP).
    pub state: String,
    pub uid: u32,
    pub process: Option<ProcessInfo>,
}

impl Socket {
    pub fn listening(&self) -> bool {
        self.state == "listen"
    }
}

pub trait ProcessSockets: Send + Sync {
    fn sockets(&self) -> Result<Vec<Socket>>;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum ChangeKind {
    Install,
    Upgrade,
    Downgrade,
    Reinstall,
    Remove,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct PackageChange {
    pub name: String,
    /// The version after the change; the removed version for a removal.
    pub version: String,
    pub kind: ChangeKind,
}

/// One package-manager transaction, however it was started.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct Transaction {
    /// Stable within the package manager, e.g. dnf5's transaction number.
    pub id: String,
    pub at: u64,
    /// "dnf", "pacman" or "apt".
    pub manager: String,
    /// The command line, when the package manager records it.
    pub command: String,
    pub changes: Vec<PackageChange>,
}

pub trait PackageHistory: Send + Sync {
    /// Transactions after `cursor` (a transaction id) in order, oldest first.
    fn transactions_after(&self, cursor: Option<&str>) -> Result<Vec<Transaction>>;
}

/// One audit or kernel log line.
#[derive(Debug, Clone, PartialEq)]
pub struct JournalRecord {
    pub at: u64,
    pub message: String,
}

pub trait Journal: Send + Sync {
    /// SELinux and AppArmor denial records after `cursor`, and the new cursor.
    /// `Ok(None)` means the journal cannot be read as this user.
    fn denials_after(
        &self,
        cursor: Option<&str>,
    ) -> Result<Option<(Vec<JournalRecord>, Option<String>)>>;
}

pub trait Notifier: Send + Sync {
    fn notify(&self, summary: &str, body: &str);
}
