use super::{cursor, gap, set_cursor, Collected, Ctx, Gap, Source};
use crate::{
    model::{Observation, Severity, SkipReason, SourceId},
    platform::{ProcessSockets, Protocol, Socket},
};
use std::{collections::HashSet, net::IpAddr, time::Duration};

/// First sightings of your programs on the network: a program's first connection,
/// and a known program reaching a new address range. Repeats are not logged.
pub struct Connections {
    sockets: Box<dyn ProcessSockets>,
    previous: HashSet<(String, String)>,
}

impl Connections {
    pub fn new(sockets: Box<dyn ProcessSockets>) -> Self {
        Connections {
            sockets,
            previous: HashSet::new(),
        }
    }
}

/// 93.184.215.14 → "93.184.215.x"; IPv6 by its /48. Groups CDN neighbours so a
/// browser does not produce an entry per server.
pub fn address_range(ip: IpAddr) -> String {
    match ip {
        IpAddr::V4(v) => {
            let o = v.octets();
            format!("{}.{}.{}.x", o[0], o[1], o[2])
        }
        IpAddr::V6(v) => {
            if let Some(mapped) = v.to_ipv4_mapped() {
                return address_range(IpAddr::V4(mapped));
            }
            let s = v.segments();
            format!("{:x}:{:x}:{:x}::/48", s[0], s[1], s[2])
        }
    }
}

/// The first-seen key for a program reaching an address range on a port.
pub fn range_key(program: &str, ip: IpAddr, port: u16) -> String {
    format!("net:{program}:{}:{port}", address_range(ip))
}

/// Connections the timeline and the Network tab care about: your programs, over
/// TCP, to somewhere other than this computer.
pub fn outbound(s: &Socket) -> bool {
    s.process.is_some()
        && s.protocol == Protocol::Tcp
        && s.state == "established"
        && s.remote.is_some_and(|r| !private(r.ip()))
}

fn private(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v) => v.is_loopback() || v.is_unspecified(),
        IpAddr::V6(v) => {
            v.is_loopback()
                || v.is_unspecified()
                || v.to_ipv4_mapped().is_some_and(|m| m.is_loopback())
        }
    }
}

impl Source for Connections {
    fn name(&self) -> &'static str {
        "connections"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(2)
    }
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap> {
        let sockets = self
            .sockets
            .sockets()
            .map_err(|e| gap(SkipReason::Unreadable, e.to_string()))?;
        let mut current = HashSet::new();
        let mut observations = vec![];
        let mut announced = HashSet::new();
        // The very first run learns what is already connected instead of announcing
        // every open browser tab as a first connection.
        let seeding = cursor(ctx.db, self.name()).is_none();
        for s in sockets {
            let (true, Some(process), Some(remote)) = (outbound(&s), &s.process, s.remote) else {
                continue;
            };
            let range = address_range(remote.ip());
            let pair = (process.name.clone(), format!("{range}:{}", remote.port()));
            // Several sockets to one range are one sighting.
            if !current.insert(pair.clone()) || self.previous.contains(&pair) {
                continue;
            }
            let program_key = format!("net:{}", process.name);
            let range_key = range_key(&process.name, remote.ip(), remote.port());
            let exe = process
                .exe
                .as_ref()
                .map(|e| format!(" {}", e.display()))
                .unwrap_or_default();
            let evidence = format!(
                "{} (pid {}){exe}\n{} → {remote}",
                process.name, process.pid, s.local
            );
            if seeding {
                let _ = ctx.db.touch(&program_key, SourceId::Network, ctx.now);
                let _ = ctx.db.touch(&range_key, SourceId::Network, ctx.now);
                continue;
            }
            let (key, summary) = if announced.contains(&process.name) {
                // Its first time online was just announced; the other places it went
                // that run are part of that entry.
                let _ = ctx.db.touch(&range_key, SourceId::Network, ctx.now);
                continue;
            } else if !ctx.db.seen(&program_key).unwrap_or(false) {
                let _ = ctx.db.touch(&range_key, SourceId::Network, ctx.now);
                announced.insert(process.name.clone());
                (
                    program_key,
                    format!(
                        "{} made its first network connection, to {remote}",
                        process.name
                    ),
                )
            } else if !ctx.db.seen(&range_key).unwrap_or(false) {
                let summary = format!(
                    "{} connected to a new address range, {range} port {}",
                    process.name,
                    remote.port()
                );
                (range_key, summary)
            } else {
                continue;
            };
            observations.push(Observation {
                at: ctx.now,
                source: SourceId::Network,
                key,
                rule: None,
                severity: Severity::Info,
                summary,
                evidence,
                link: None,
            });
        }
        if seeding {
            set_cursor(ctx.db, self.name(), "seeded");
        }
        self.previous = current;
        Ok(Collected {
            observations,
            ..Default::default()
        })
    }
}
