//! The Network tab's live tables, read on request rather than from the timeline.
use super::{connections, listeners};
use crate::{
    checks::network::{listener_findings, reach},
    model::{Coverage, Finding, Report, SkipReason, Skipped},
    platform::{ProcessSockets, Protocol, Socket},
    store::db::Db,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ConnectionRow {
    pub socket: Socket,
    /// The address range first-seen is counted by, e.g. "93.184.215.x".
    pub range: String,
    pub first_at: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ListenerRow {
    pub socket: Socket,
    /// "this computer only", "one network interface" or "every network interface".
    pub reach: String,
    pub findings: Vec<Finding>,
    pub first_at: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct NetworkSnapshot {
    pub at: u64,
    pub connections: Vec<ConnectionRow>,
    pub listeners: Vec<ListenerRow>,
    pub coverage: Coverage,
}

pub fn network_snapshot(sockets: &dyn ProcessSockets, db: &Db, now: u64) -> NetworkSnapshot {
    let mut coverage = Coverage::default();
    let all = match sockets.sockets() {
        Ok(all) => {
            coverage.checked.push("Sockets in /proc/net".into());
            all
        }
        Err(e) => {
            coverage.skipped.push(Skipped {
                what: "Sockets".into(),
                why: SkipReason::Unreadable,
                detail: format!("{e:#}"),
            });
            vec![]
        }
    };
    coverage.push(
        "Connections list your own programs. Listeners list every program, but other users' programs show without a name.".into(),
    );
    let mut connections: Vec<ConnectionRow> = all
        .iter()
        .filter(|s| connections::outbound(s))
        .filter_map(|s| {
            let (process, remote) = (s.process.as_ref()?, s.remote?);
            Some(ConnectionRow {
                socket: s.clone(),
                range: connections::address_range(remote.ip()),
                first_at: db
                    .first_at(&connections::range_key(
                        &process.name,
                        remote.ip(),
                        remote.port(),
                    ))
                    .ok()
                    .flatten(),
            })
        })
        .collect();
    connections.sort_by(|a, b| {
        let name = |r: &ConnectionRow| r.socket.process.as_ref().map(|p| p.name.clone());
        name(a)
            .cmp(&name(b))
            .then(a.socket.remote.cmp(&b.socket.remote))
    });
    let mut listeners: Vec<ListenerRow> = all
        .iter()
        .filter(|s| s.protocol == Protocol::Tcp && s.listening())
        .map(|s| {
            let mut r = Report::new("Listener", s.local);
            listener_findings(&mut r, s);
            r.findings.sort_by_key(|f| std::cmp::Reverse(f.severity));
            ListenerRow {
                socket: s.clone(),
                reach: reach(s).into(),
                findings: r.findings,
                first_at: db.first_at(&listeners::listener_key(s)).ok().flatten(),
            }
        })
        .collect();
    // Most exposed first: findings, then reach beyond this computer, then by port.
    listeners.sort_by_key(|l| {
        (
            std::cmp::Reverse(l.findings.first().map(|f| f.severity)),
            l.reach == "this computer only",
            l.socket.local.port(),
        )
    });
    NetworkSnapshot {
        at: now,
        connections,
        listeners,
        coverage,
    }
}
