//! The contract between `bigmand` and its clients (CLI, desktop app, native host).
//!
//! Transport: a Unix stream socket in `$XDG_RUNTIME_DIR/bigman/`, mode 0600, with the
//! peer uid checked by the server. Framing: one JSON object per line, at most
//! `MAX_FRAME` bytes. A client sends `ClientFrame`s and reads `ServerFrame`s.
pub mod client;

use crate::{
    checks::{
        canary::Health as MonitorHealth,
        doctor::{SignatureState, ToolState},
    },
    model::Report,
};
use anyhow::{bail, Context, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    io::{BufRead, Read, Write},
    path::PathBuf,
};
use ts_rs::TS;

/// Largest accepted frame. Reports with full evidence stay well below this.
pub const MAX_FRAME: u64 = 16 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[ts(export)]
pub enum Request {
    Health,
    /// Protection state for the Overview tiles. Read-only and fast.
    Status,
    /// Run the read-only security checkup and save its report.
    Doctor,
    /// Most recent reports first, as history summaries (full evidence via the report id).
    History {
        limit: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
#[ts(export)]
pub enum Response {
    Health(DaemonHealth),
    Status(Box<ProtectionStatus>),
    Report(Box<Report>),
    History(Vec<Report>),
    Error(ErrorBody),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct DaemonHealth {
    pub version: String,
    pub pid: u32,
    pub started: u64,
    pub state_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProtectionStatus {
    pub checked_at: u64,
    pub tools: Vec<ToolState>,
    pub signatures: Option<SignatureState>,
    /// The canary monitor's latest heartbeat, when it is fresh.
    pub monitor: Option<MonitorHealth>,
    pub last_scan: Option<Report>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct ErrorBody {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct ClientFrame {
    pub id: u64,
    pub request: Request,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ServerFrame {
    pub id: u64,
    pub response: Response,
}

/// `BIGMAN_SOCKET` overrides the location for tests and separate profiles.
pub fn socket_path() -> Result<PathBuf> {
    if let Some(p) = std::env::var_os("BIGMAN_SOCKET") {
        return Ok(PathBuf::from(p));
    }
    let runtime = std::env::var_os("XDG_RUNTIME_DIR").context(
        "XDG_RUNTIME_DIR is not set; the daemon socket needs a private runtime directory",
    )?;
    Ok(PathBuf::from(runtime).join("bigman/bigmand.sock"))
}

pub fn write_frame<T: Serialize>(out: &mut impl Write, frame: &T) -> Result<()> {
    let mut line = serde_json::to_vec(frame)?;
    if line.len() as u64 > MAX_FRAME {
        bail!("Frame exceeds {MAX_FRAME} bytes");
    }
    line.push(b'\n');
    out.write_all(&line)?;
    out.flush()?;
    Ok(())
}

/// Reads one frame. `Ok(None)` means the peer closed the connection cleanly.
pub fn read_frame<T: DeserializeOwned>(input: &mut impl BufRead) -> Result<Option<T>> {
    let mut line = Vec::new();
    let n = input
        .by_ref()
        .take(MAX_FRAME + 1)
        .read_until(b'\n', &mut line)?;
    if n == 0 {
        return Ok(None);
    }
    if line.last() != Some(&b'\n') {
        bail!("Frame too large or truncated");
    }
    Ok(Some(serde_json::from_slice(&line)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frames_roundtrip_and_oversize_is_rejected() {
        let mut buf = Vec::new();
        let frame = ClientFrame {
            id: 7,
            request: Request::Health,
        };
        write_frame(&mut buf, &frame).unwrap();
        let back: ClientFrame = read_frame(&mut buf.as_slice()).unwrap().unwrap();
        assert_eq!(back, frame);
        assert!(read_frame::<ClientFrame>(&mut b"".as_slice())
            .unwrap()
            .is_none());
        assert!(read_frame::<ClientFrame>(&mut b"{\"id\":1".as_slice()).is_err());
    }
}
