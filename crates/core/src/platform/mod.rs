//! Everything OS-specific, behind small traits. Rules and sources consume the
//! parsed data these return and never touch paths or commands themselves; tests
//! substitute fakes. Only a Linux implementation exists (plan C4).
pub mod linux;
pub mod packagedb;
pub mod sockdiag;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, net::SocketAddr, path::PathBuf};
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

/// One connected TCP socket and the bytes it has carried since it opened.
#[derive(Debug, Clone, PartialEq)]
pub struct TcpCounters {
    /// The kernel's socket cookie: unique to this socket until the next boot.
    pub id: u64,
    pub local: SocketAddr,
    pub remote: SocketAddr,
    pub uid: u32,
    /// Bytes the other side has acknowledged.
    pub sent: u64,
    pub received: u64,
    pub process: Option<ProcessInfo>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TcpTable {
    pub sockets: Vec<TcpCounters>,
    /// Ports a program is listening on. A connection to one of them came in.
    pub listening: Vec<u16>,
}

/// Per-connection byte counters. Readable without privileges for every
/// account's sockets; the program is known only for your own.
pub trait SocketCounters: Send + Sync {
    fn tcp(&self) -> Result<TcpTable>;
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

/// How a file compares with the package that installed it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(tag = "state", rename_all = "snake_case")]
#[ts(export)]
pub enum FileState {
    /// Matches the package database (a changed modification time alone counts as a match).
    Verified,
    /// Differs from what the package installed: "content", "mode", "owner", "missing" ...
    Modified { changes: Vec<String> },
    /// Owned, but the file could not be read to compare it.
    Unverifiable { detail: String },
    /// No installed package claims it.
    Unowned,
}

/// Which package installed a file, and whether it still matches.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct Attribution {
    /// "sudo-1.9.17-10.p2.fc44"; none when unowned.
    pub package: Option<String>,
    /// Whether the package marks the file as configuration the admin may edit.
    pub config: bool,
    pub state: FileState,
}

/// One install script a package runs, verbatim.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct PackageScript {
    pub package: String,
    /// "preinstall", "postinstall", "preuninstall", "postuninstall", "posttrans", ...
    pub phase: String,
    /// The interpreter, e.g. "/bin/sh".
    pub interpreter: String,
    pub text: String,
}

/// The installed-package database: rpm, dpkg or pacman.
pub trait PackageDb: Send + Sync {
    /// "rpm", "dpkg" or "pacman".
    fn manager(&self) -> &'static str;
    /// Owner and verification for each path. Paths no package claims are `Unowned`.
    fn attribute(
        &self,
        paths: &[PathBuf],
        stop: &crate::runner::Cancel,
    ) -> Result<BTreeMap<PathBuf, Attribution>>;
    /// The files an installed package owns.
    fn files(&self, package: &str, stop: &crate::runner::Cancel) -> Result<Vec<PathBuf>>;
    /// The install scripts of an installed package.
    fn scripts(&self, package: &str, stop: &crate::runner::Cancel) -> Result<Vec<PackageScript>>;
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

/// One of your own processes, as /proc shows it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessEntry {
    pub pid: u32,
    pub ppid: u32,
    /// The kernel's short name (`comm`).
    pub name: String,
    pub exe: Option<PathBuf>,
    /// The command line, one argument per entry.
    pub args: Vec<String>,
}

pub trait Processes: Send + Sync {
    /// Your own processes. Other users' command lines are not read.
    fn processes(&self) -> Result<Vec<ProcessEntry>>;
}

/// One login session as the login manager (systemd-logind) reports it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginSession {
    pub id: String,
    pub user: String,
    /// The PAM service that opened it, e.g. "sshd", "gdm-password", "systemd-user".
    pub service: String,
    /// Whether the session came over the network.
    pub remote: bool,
    pub remote_host: Option<String>,
    pub remote_user: Option<String>,
    pub tty: Option<String>,
    /// "user", "greeter", "manager", "background" ...
    pub class: String,
}

pub trait Sessions: Send + Sync {
    /// Every current login session on the machine, for every user.
    fn sessions(&self) -> Result<Vec<LoginSession>>;
}

/// What tells awake time from suspended time and one boot from the next.
pub trait Clock: Send + Sync {
    /// Changes on every boot.
    fn boot_id(&self) -> Option<String>;
    /// Seconds spent suspended since boot.
    fn asleep_secs(&self) -> u64;
}
