use super::{gap, Collected, Ctx, Gap, Source};
use crate::{
    model::{Coverage, FlowSample, SkipReason, TrafficReport},
    platform::{SocketCounters, TcpTable},
    store::db::Db,
};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    net::IpAddr,
    time::Duration,
};

/// Seconds each stored interval covers while it is fresh.
pub const STEP: u64 = 5;
/// A pause longer than this (suspend, a stalled collector) is not spread over the
/// time in between: what arrived is booked where the pause ended.
const LONGEST_SPREAD_MS: u64 = 30_000;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Key {
    program: Option<String>,
    uid: u32,
    remote: IpAddr,
    port: u16,
    inbound: bool,
}

/// Bytes each program moves over TCP, by address and service port. Read from the
/// kernel's per-connection counters every two seconds and booked into fixed
/// five-second intervals, so the totals do not depend on when the reads happened.
pub struct Traffic {
    counters: Box<dyn SocketCounters>,
    /// Counters at the last read, by socket.
    last: HashMap<u64, (u64, u64)>,
    last_ms: Option<u64>,
    /// Intervals still filling, by their start.
    pending: BTreeMap<u64, HashMap<Key, (f64, f64)>>,
}

fn loopback(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v) => v.is_loopback() || v.is_unspecified(),
        IpAddr::V6(v) => v.is_loopback() || v.is_unspecified(),
    }
}

impl Traffic {
    pub fn new(counters: Box<dyn SocketCounters>) -> Self {
        Traffic {
            counters,
            last: HashMap::new(),
            last_ms: None,
            pending: BTreeMap::new(),
        }
    }

    /// Books what moved since the last read and returns the intervals that are
    /// now complete. `now_ms` is Unix milliseconds.
    pub fn step(&mut self, now_ms: u64, table: &TcpTable) -> Vec<FlowSample> {
        let listening: HashSet<u16> = table.listening.iter().copied().collect();
        let mut current = HashMap::with_capacity(table.sockets.len());
        let mut moved: HashMap<Key, (u64, u64)> = HashMap::new();
        for s in &table.sockets {
            // Traffic between programs on this computer never reaches the network.
            if loopback(s.remote.ip()) || s.remote.ip() == s.local.ip() {
                continue;
            }
            current.insert(s.id, (s.sent, s.received));
            // The first read only learns where each counter stands.
            if self.last_ms.is_none() {
                continue;
            }
            // A socket not seen before opened since the last read: all of it is new.
            let (sent, received) = self.last.get(&s.id).copied().unwrap_or((0, 0));
            let (tx, rx) = (
                s.sent.saturating_sub(sent),
                s.received.saturating_sub(received),
            );
            if tx == 0 && rx == 0 {
                continue;
            }
            let inbound = listening.contains(&s.local.port());
            let entry = moved
                .entry(Key {
                    program: s.process.as_ref().map(|p| p.name.clone()),
                    uid: s.uid,
                    remote: s.remote.ip(),
                    port: if inbound {
                        s.local.port()
                    } else {
                        s.remote.port()
                    },
                    inbound,
                })
                .or_default();
            entry.0 += rx;
            entry.1 += tx;
        }
        if let Some(from) = self.last_ms {
            for (start, share) in shares(from, now_ms) {
                let interval = self.pending.entry(start).or_default();
                for (key, (rx, tx)) in &moved {
                    let booked = interval.entry(key.clone()).or_default();
                    booked.0 += *rx as f64 * share;
                    booked.1 += *tx as f64 * share;
                }
            }
        }
        self.last = current;
        self.last_ms = Some(now_ms);
        // Nothing later can add to an interval that ended before this read.
        let open = now_ms / 1000 / STEP * STEP;
        let still_filling = self.pending.split_off(&open);
        let done = std::mem::replace(&mut self.pending, still_filling);
        done.into_iter()
            .flat_map(|(at, flows)| {
                flows.into_iter().map(move |(k, (rx, tx))| FlowSample {
                    at,
                    program: k.program,
                    uid: k.uid,
                    remote: k.remote,
                    port: k.port,
                    inbound: k.inbound,
                    rx,
                    tx,
                })
            })
            .collect()
    }
}

/// The intervals `(from, to]` overlaps and the share of it in each.
fn shares(from: u64, to: u64) -> Vec<(u64, f64)> {
    let step = STEP * 1000;
    let last = to.saturating_sub(1) / step * STEP;
    if to <= from || to - from > LONGEST_SPREAD_MS {
        return vec![(last, 1.0)];
    }
    let span = (to - from) as f64;
    (from / step..=to.saturating_sub(1) / step)
        .map(|cell| {
            let (start, end) = (cell * step, (cell + 1) * step);
            (cell * STEP, (to.min(end) - from.max(start)) as f64 / span)
        })
        .collect()
}

impl Source for Traffic {
    fn name(&self) -> &'static str {
        "traffic"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(2)
    }
    fn collect(&mut self, _: &Ctx) -> Result<Collected, Gap> {
        let table = self
            .counters
            .tcp()
            .map_err(|e| gap(SkipReason::Unreadable, format!("{e:#}")))?;
        // Sub-second time: a read lands anywhere inside a five-second interval.
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_millis() as u64);
        Ok(Collected {
            traffic: self.step(now_ms, &table),
            ..Default::default()
        })
    }
}

/// Traffic by program over `[since, until]` in about `points` intervals, with
/// what the counters can and cannot see.
pub fn traffic_report(
    db: &Db,
    since: u64,
    until: u64,
    points: u64,
) -> anyhow::Result<TrafficReport> {
    let mut report = db.traffic(since, until, points)?;
    report.own_uid = crate::platform::linux::own_uid();
    for p in &mut report.programs {
        // The connections source enters a program's first connection under this key.
        if let Some(name) = &p.program {
            p.first_at = db.first_at(&format!("net:{name}")).ok().flatten();
        }
    }
    let mut coverage = Coverage::default();
    coverage.checked(
        "TCP connections of every program, from the kernel's per-connection byte counters"
            .to_string(),
    );
    coverage.push("UDP is not counted per program: video calls, most streaming over QUIC, VPN tunnels and DNS. The interface graph includes it.".into());
    coverage.push("A connection that opens and closes within two seconds can be missed.".into());
    coverage.push("Other accounts' programs are counted but not named, and so are your own when they run in a sandbox that hides them. Traffic between programs on this computer is left out.".into());
    if report.flows_omitted > 0 {
        coverage.push(format!(
            "Addresses and traffic types are added up from the {} largest program, address and port combinations; {} smaller ones are left out of those two lists.",
            report.flows.len(),
            report.flows_omitted
        ));
    }
    report.coverage = coverage;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::{ProcessInfo, TcpCounters};

    struct Never;
    impl SocketCounters for Never {
        fn tcp(&self) -> anyhow::Result<TcpTable> {
            anyhow::bail!("not used")
        }
    }

    fn socket(id: u64, name: Option<&str>, remote: &str, sent: u64, received: u64) -> TcpCounters {
        TcpCounters {
            id,
            local: "10.0.0.5:50000".parse().unwrap(),
            remote: remote.parse().unwrap(),
            uid: if name.is_some() { 1000 } else { 0 },
            sent,
            received,
            process: name.map(|n| ProcessInfo {
                pid: 42,
                name: n.into(),
                exe: None,
            }),
        }
    }

    fn table(sockets: Vec<TcpCounters>) -> TcpTable {
        TcpTable {
            sockets,
            listening: vec![22],
        }
    }

    fn total(samples: &[FlowSample]) -> (f64, f64) {
        samples
            .iter()
            .fold((0.0, 0.0), |(rx, tx), s| (rx + s.rx, tx + s.tx))
    }

    #[test]
    fn books_what_moved_since_the_last_read_into_five_second_intervals() {
        let mut t = Traffic::new(Box::new(Never));
        let firefox =
            |sent, received| socket(1, Some("firefox"), "93.184.215.14:443", sent, received);
        assert!(
            t.step(100_000, &table(vec![firefox(900, 5_000)]))
                .is_empty(),
            "the first read only sets the baseline"
        );
        // Two seconds later, inside the same interval: booked, not yet complete.
        assert!(t
            .step(102_000, &table(vec![firefox(1_000, 9_000)]))
            .is_empty());
        // A read that crosses 105 s closes the interval that started at 100 s.
        let done = t.step(106_000, &table(vec![firefox(1_400, 13_000)]));
        assert_eq!(done.len(), 1);
        assert_eq!(done[0].at, 100);
        assert_eq!(done[0].program.as_deref(), Some("firefox"));
        assert_eq!(
            (done[0].remote.to_string().as_str(), done[0].port),
            ("93.184.215.14", 443)
        );
        // All of the first read's 4,000 bytes, and three quarters of the second's.
        assert_eq!((done[0].rx, done[0].tx), (4_000.0 + 3_000.0, 100.0 + 300.0));
        let rest = t.step(110_000, &table(vec![firefox(1_400, 13_000)]));
        assert_eq!(
            total(&rest),
            (1_000.0, 100.0),
            "nothing is lost or counted twice"
        );
    }

    #[test]
    fn a_new_connection_counts_from_zero_and_a_closed_one_is_forgotten() {
        let mut t = Traffic::new(Box::new(Never));
        t.step(100_000, &table(vec![]));
        let done = [
            t.step(
                101_000,
                &table(vec![socket(9, Some("curl"), "1.1.1.1:443", 300, 70_000)]),
            ),
            t.step(106_000, &table(vec![])),
            // The same address again on a new socket: its counter starts over.
            t.step(
                111_000,
                &table(vec![socket(10, Some("curl"), "1.1.1.1:443", 5, 10)]),
            ),
            t.step(116_000, &table(vec![])),
        ]
        .concat();
        assert_eq!(total(&done), (70_010.0, 305.0));
    }

    #[test]
    fn names_the_service_port_and_leaves_local_traffic_out() {
        let mut t = Traffic::new(Box::new(Never));
        t.step(100_000, &table(vec![]));
        let mut incoming = socket(2, None, "192.168.1.20:51515", 2_000, 100);
        incoming.local = "192.168.1.5:22".parse().unwrap();
        t.step(
            101_000,
            &table(vec![
                incoming,
                socket(3, Some("psql"), "127.0.0.1:5432", 9_999, 9_999),
            ]),
        );
        let done = t.step(106_000, &table(vec![]));
        assert_eq!(done.len(), 1, "loopback traffic is not network traffic");
        assert!(done[0].inbound && done[0].port == 22 && done[0].program.is_none());
        assert_eq!(done[0].uid, 0);
    }

    #[test]
    fn a_long_pause_is_booked_where_it_ended() {
        assert_eq!(shares(100_000, 102_500), vec![(100, 1.0)]);
        assert_eq!(shares(104_000, 106_000), vec![(100, 0.5), (105, 0.5)]);
        // The interval is open on the left: a read exactly on a boundary belongs before it.
        assert_eq!(shares(103_000, 105_000), vec![(100, 1.0)]);
        assert_eq!(shares(100_000, 100_000 + 3_600_000), vec![(3_695, 1.0)]);
    }
}
