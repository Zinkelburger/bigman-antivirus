//! What the firewall lets in, read without root: firewalld answers its
//! read-only queries to any local user, ufw only says whether it is on.
use crate::platform::Protocol;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

/// A port or range in one protocol, as firewalld writes it ("1025-65535/tcp").
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortRange {
    pub protocol: Protocol,
    pub from: u16,
    pub to: u16,
}

impl PortRange {
    pub fn parse(text: &str) -> Option<Self> {
        let (ports, proto) = text.split_once('/')?;
        let protocol = match proto {
            "tcp" => Protocol::Tcp,
            "udp" => Protocol::Udp,
            _ => return None,
        };
        let (from, to) = match ports.split_once('-') {
            Some((a, b)) => (a.parse().ok()?, b.parse().ok()?),
            None => {
                let p = ports.parse().ok()?;
                (p, p)
            }
        };
        (from <= to).then_some(Self { protocol, from, to })
    }
    pub fn contains(&self, protocol: Protocol, port: u16) -> bool {
        self.protocol == protocol && (self.from..=self.to).contains(&port)
    }
    /// How firewalld writes it, for evidence and commands.
    pub fn spec(&self) -> String {
        let proto = match self.protocol {
            Protocol::Tcp => "tcp",
            Protocol::Udp => "udp",
        };
        if self.from == self.to {
            format!("{}/{proto}", self.from)
        } else {
            format!("{}-{}/{proto}", self.from, self.to)
        }
    }
    pub fn width(&self) -> u32 {
        u32::from(self.to) - u32::from(self.from) + 1
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ZoneService {
    pub name: String,
    pub ports: Vec<PortRange>,
}

/// One active firewalld zone: where it applies and what it lets in.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Zone {
    pub name: String,
    pub default: bool,
    pub interfaces: Vec<String>,
    pub sources: Vec<String>,
    /// "default", "ACCEPT", "REJECT" or "DROP".
    pub target: String,
    pub services: Vec<ZoneService>,
    pub ports: Vec<PortRange>,
}

impl Zone {
    /// Where this zone applies, in words.
    pub fn place(&self) -> String {
        let mut on: Vec<String> = self.interfaces.clone();
        on.extend(self.sources.iter().map(|s| format!("from {s}")));
        if on.is_empty() {
            format!("the {} zone", self.name)
        } else {
            format!("the {} zone ({})", self.name, on.join(", "))
        }
    }
    /// Why this zone lets the port in, if it does.
    pub fn allows(&self, protocol: Protocol, port: u16) -> Option<String> {
        if self.target == "ACCEPT" {
            return Some("it accepts every connection".into());
        }
        if let Some(s) = self
            .services
            .iter()
            .find(|s| s.ports.iter().any(|r| r.contains(protocol, port)))
        {
            return Some(format!("it allows the {} service", s.name));
        }
        self.ports
            .iter()
            .find(|r| r.contains(protocol, port))
            .map(|r| format!("it opens {}", r.spec()))
    }
}

/// What Bigman could learn about the firewall.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum FirewallState {
    /// firewalld is running; its active zones.
    Firewalld { zones: Vec<Zone> },
    /// ufw is enabled. Its rules are readable only by root.
    Ufw,
    /// No firewalld or ufw is running, and no other firewall service is active.
    Off {
        /// "firewalld" or "ufw" when one is installed but not running.
        installed: Option<String>,
    },
    /// Something else manages the firewall, or it could not be read.
    Unknown { reason: String },
}

impl Default for FirewallState {
    fn default() -> Self {
        Self::Unknown {
            reason: "The firewall was not checked".into(),
        }
    }
}

/// Who can reach a listening port from other computers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reach {
    /// Bound to loopback: only this computer.
    ThisComputer,
    /// The firewall rejects it on every active zone.
    Blocked,
    /// Each entry says where and why: "the FedoraWorkstation zone (wlp9s0) because it allows the ssh service".
    Open(Vec<String>),
    /// No firewall is running, so any network this computer joins can reach it.
    NoFirewall,
    Unknown(String),
}

impl Reach {
    /// Whether other computers can, or may, connect.
    pub fn exposed(&self) -> bool {
        matches!(self, Reach::Open(_) | Reach::NoFirewall)
    }
    pub fn sentence(&self) -> String {
        match self {
            Reach::ThisComputer => "Only programs on this computer can connect.".into(),
            Reach::Blocked => {
                "The firewall blocks it on every network you are connected to.".into()
            }
            Reach::Open(via) => format!("Other computers can connect through {}.", via.join("; ")),
            Reach::NoFirewall => {
                "No firewall is running, so other computers on any network you join can connect."
                    .into()
            }
            Reach::Unknown(why) => {
                format!("Bigman could not tell whether the firewall allows it: {why}.")
            }
        }
    }
}

fn loopback(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v) => v.is_loopback(),
        IpAddr::V6(v) => v.is_loopback() || v.to_ipv4_mapped().is_some_and(|m| m.is_loopback()),
    }
}

/// Who can reach `port` on a socket bound to `ip`, through this firewall.
pub fn reach(firewall: &FirewallState, ip: IpAddr, protocol: Protocol, port: u16) -> Reach {
    if loopback(ip) {
        return Reach::ThisComputer;
    }
    match firewall {
        FirewallState::Firewalld { zones } => {
            let via: Vec<String> = zones
                .iter()
                .filter_map(|z| {
                    z.allows(protocol, port)
                        .map(|why| format!("{} because {why}", z.place()))
                })
                .collect();
            if via.is_empty() {
                Reach::Blocked
            } else {
                Reach::Open(via)
            }
        }
        FirewallState::Off { .. } => Reach::NoFirewall,
        FirewallState::Ufw => {
            Reach::Unknown("ufw is on, and its rules are readable only by an administrator".into())
        }
        FirewallState::Unknown { reason } => Reach::Unknown(reason.clone()),
    }
}

/// Zone names and the interfaces they apply to, from `firewall-cmd --get-active-zones`.
pub fn parse_active_zones(text: &str) -> Vec<(String, bool)> {
    text.lines()
        .filter(|l| !l.starts_with(char::is_whitespace) && !l.trim().is_empty())
        .filter_map(|l| {
            let mut parts = l.split_whitespace();
            let name = parts.next()?.to_string();
            let default = parts.any(|p| p.contains("default"));
            valid_zone(&name).then_some((name, default))
        })
        .collect()
}

/// Zone names are passed to firewall-cmd; accept only what firewalld itself allows.
pub fn valid_zone(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "_-".contains(c))
}

fn field<'a>(text: &'a str, key: &str) -> Vec<&'a str> {
    text.lines()
        .find_map(|l| l.trim().strip_prefix(key)?.strip_prefix(':'))
        .map(|v| v.split_whitespace().collect())
        .unwrap_or_default()
}

/// One zone from `firewall-cmd --zone=Z --list-all`. Service ports are filled in separately.
pub fn parse_list_all(name: &str, default: bool, text: &str) -> Zone {
    Zone {
        name: name.into(),
        default,
        interfaces: field(text, "interfaces")
            .into_iter()
            .map(String::from)
            .collect(),
        sources: field(text, "sources")
            .into_iter()
            .map(String::from)
            .collect(),
        target: field(text, "target")
            .first()
            .map_or("default".into(), |t| t.to_string()),
        services: field(text, "services")
            .into_iter()
            .map(|s| ZoneService {
                name: s.into(),
                ports: vec![],
            })
            .collect(),
        ports: field(text, "ports")
            .into_iter()
            .filter_map(PortRange::parse)
            .collect(),
    }
}

/// The ports of one service, from `firewall-cmd --info-service=S`.
pub fn parse_info_service(text: &str) -> Vec<PortRange> {
    field(text, "ports")
        .into_iter()
        .filter_map(PortRange::parse)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORKSTATION: &str = "FedoraWorkstation (default, active)\n  target: default\n  ingress-priority: 0\n  icmp-block-inversion: no\n  interfaces: enp0s13f0u1u1 wlp9s0f0\n  sources: \n  services: dhcpv6-client samba-client ssh\n  ports: 1025-65535/udp 1025-65535/tcp\n  protocols: \n  forward: yes\n";

    #[test]
    fn a_fedora_workstation_zone_lets_high_ports_and_ssh_in() {
        let mut zone = parse_list_all("FedoraWorkstation", true, WORKSTATION);
        assert_eq!(zone.interfaces, ["enp0s13f0u1u1", "wlp9s0f0"]);
        assert_eq!(zone.ports.len(), 2);
        zone.services[2].ports = parse_info_service("ssh\n  ports: 22/tcp\n  protocols: \n");
        let fw = FirewallState::Firewalld { zones: vec![zone] };
        let any: IpAddr = "0.0.0.0".parse().unwrap();
        let Reach::Open(via) = reach(&fw, any, Protocol::Tcp, 22) else {
            panic!("ssh is allowed")
        };
        assert!(
            via[0].ends_with("because it allows the ssh service"),
            "{via:?}"
        );
        assert!(reach(&fw, any, Protocol::Tcp, 4000).exposed());
        assert_eq!(reach(&fw, any, Protocol::Tcp, 631), Reach::Blocked);
        assert_eq!(
            reach(&fw, "::1".parse().unwrap(), Protocol::Tcp, 22),
            Reach::ThisComputer
        );
    }

    #[test]
    fn active_zones_and_ranges_parse() {
        let zones = parse_active_zones(
            "FedoraWorkstation (default)\n  interfaces: wlp9s0f0\nredhat\n  interfaces: tun0 wt0\n",
        );
        assert_eq!(
            zones,
            [("FedoraWorkstation".into(), true), ("redhat".into(), false)]
        );
        assert!(!valid_zone("a;b"));
        let r = PortRange::parse("1025-65535/tcp").unwrap();
        assert_eq!((r.width(), r.spec().as_str()), (64511, "1025-65535/tcp"));
        assert_eq!(PortRange::parse("22/sctp"), None);
    }
}
