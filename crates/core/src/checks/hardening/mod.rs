//! Hardening suggestions: settings on this computer that you could change to
//! close a path an attacker would use, each with the evidence that it applies
//! here, what it costs you, and either a previewed, reversible change Bigman can
//! make or the steps to make it yourself. docs/research/hardening.md has the
//! reasoning and sources for every rule.
//!
//! `gather` reads the facts without root; `assess` turns them into suggestions
//! and is pure, so fixtures drive it directly.
pub mod change;
pub mod firewall;
pub mod gather;

use crate::{
    model::{now, Coverage, Report, Severity, SkipReason},
    platform::{Protocol, Socket},
    runner::Cancel,
    store::Store,
};
use anyhow::{bail, Context, Result};
use change::{run, Change, Step};
use firewall::{reach, FirewallState, Reach};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};
use ts_rs::TS;

/// The state of one systemd unit, from `systemctl show`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct UnitState {
    /// "loaded" or "not-found".
    pub load: String,
    /// "enabled", "disabled", "static", "masked" ...
    pub enabled: String,
    /// "active", "inactive", "failed" ...
    pub active: String,
}

impl UnitState {
    pub fn installed(&self) -> bool {
        self.load == "loaded"
    }
    pub fn is_enabled(&self) -> bool {
        self.installed() && matches!(self.enabled.as_str(), "enabled" | "enabled-runtime")
    }
    pub fn is_active(&self) -> bool {
        self.installed() && matches!(self.active.as_str(), "active" | "activating" | "reloading")
    }
}

/// A network login found in the logs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RemoteLogin {
    pub at: u64,
    pub user: String,
    pub from: String,
}

/// Network logins in the logs Bigman could read.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct LoginHistory {
    /// The oldest moment the logs cover.
    pub since: u64,
    pub logins: Vec<RemoteLogin>,
    /// "the system journal", "wtmp".
    pub read: Vec<String>,
}

/// A sysctl drop-in that sets a key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SysctlSetting {
    pub file: PathBuf,
    pub value: String,
    /// The package that installed the file, when one did.
    pub package: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct SysctlState {
    pub value: String,
    /// Files that set the key, in the order systemd-sysctl applies them.
    pub set_by: Vec<SysctlSetting>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Desktop {
    Kde,
    Gnome,
}

/// Whether the screen locks by itself. `raw` holds each setting as stored
/// (absent: the default), so an undo writes back exactly what was there.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScreenLock {
    pub desktop: Desktop,
    pub auto_lock: bool,
    /// Idle minutes before locking; 0 means never.
    pub minutes: u32,
    pub on_resume: bool,
    #[serde(default)]
    pub raw: BTreeMap<String, Option<String>>,
    /// GNOME keys an administrator locked with dconf: a user cannot change them.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locked: Vec<String>,
}

/// A display-manager file that logs someone in without a password.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Autologin {
    pub file: PathBuf,
    pub user: String,
    /// "gdm", "sddm", "plasmalogin" or "lightdm".
    pub manager: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MountEncryption {
    pub mount: String,
    pub encrypted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NpmState {
    pub version: String,
    pub ignore_scripts: String,
}

impl NpmState {
    /// npm 12 blocks dependency install scripts unless the project allows them.
    pub fn blocks_by_default(&self) -> bool {
        self.version
            .split('.')
            .next()
            .and_then(|m| m.parse::<u32>().ok())
            .is_some_and(|m| m >= 12)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HomeState {
    pub path: PathBuf,
    pub mode: u32,
    /// Other people's login accounts on this computer.
    pub other_accounts: Vec<String>,
}

/// Everything the hardening rules read. `None` means Bigman could not find out.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Inputs {
    pub now: u64,
    pub user: String,
    pub units: BTreeMap<String, UnitState>,
    /// Listening TCP sockets and bound UDP sockets.
    pub listeners: Vec<Socket>,
    pub firewall: FirewallState,
    pub logins: Option<LoginHistory>,
    /// Your current sessions that came over the network.
    pub remote_sessions: Vec<String>,
    pub sysctl: BTreeMap<String, SysctlState>,
    /// systemd-resolved's global LLMNR mode ("yes", "resolve", "no").
    pub llmnr: Option<String>,
    pub screen_lock: Option<ScreenLock>,
    pub autologin: Vec<Autologin>,
    /// Groups your account belongs to.
    pub groups: Vec<String>,
    pub home: Option<HomeState>,
    /// npm's version and effective `ignore-scripts`, when you use npm.
    pub npm: Option<NpmState>,
    pub encryption: Option<Vec<MountEncryption>>,
    /// None when the computer did not boot with UEFI or the variable is unreadable.
    pub secure_boot: Option<bool>,
    /// What could not be read, and why.
    pub unread: Vec<(String, SkipReason, String)>,
}

/// How a suggestion gets done.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[ts(export)]
pub enum Fix {
    /// Bigman can make the change after a preview. `admin` when polkit asks for a password.
    Bigman { admin: bool, verb: String },
    /// Steps for you to follow; Bigman cannot make this change safely.
    Manual { steps: String },
    /// Bigman could make it, but not now.
    Blocked { reason: String },
}

/// One thing you could change.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Suggestion {
    /// Stable: the rule, plus the unit or zone when one rule suggests several changes.
    pub id: String,
    pub rule: String,
    pub severity: Severity,
    /// What to do, imperative: "Turn off the SSH server".
    pub title: String,
    /// What was found here, in one or two sentences.
    pub summary: String,
    /// The facts, verbatim.
    pub evidence: String,
    /// What the change protects against.
    pub why: String,
    /// What stops working, and how to get it back.
    pub tradeoff: String,
    pub fix: Fix,
    /// You chose to keep this setting.
    pub kept: bool,
    #[serde(skip)]
    #[ts(skip)]
    pub change: Option<(Change, Change)>,
}

/// A change Bigman made, kept so it can be undone later.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AppliedChange {
    pub id: String,
    pub title: String,
    pub at: u64,
    /// The undo, as it will run.
    pub undo: String,
    #[ts(skip)]
    pub revert: Change,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HardeningReview {
    /// Findings for the open suggestions, with coverage, like any check.
    pub report: Report,
    /// Every suggestion, open first; kept ones last.
    pub suggestions: Vec<Suggestion>,
    /// Checks whose setting is already in place, with what Bigman saw.
    pub in_place: Vec<InPlace>,
    /// Changes Bigman made that can still be undone.
    pub applied: Vec<AppliedChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct InPlace {
    pub check: String,
    pub detail: String,
}

/// A network service Bigman knows how to suggest turning off.
struct Daemon {
    key: &'static str,
    rule: &'static str,
    name: &'static str,
    units: &'static [&'static str],
    ports: &'static [(Protocol, u16)],
    does: &'static str,
    tradeoff: &'static str,
}

use Protocol::{Tcp, Udp};

const SSH_UNITS: &[&str] = &["sshd.service", "sshd.socket", "ssh.service", "ssh.socket"];

const DAEMONS: &[Daemon] = &[
    Daemon {
        key: "sshd",
        rule: "harden.sshd",
        name: "the SSH server",
        units: SSH_UNITS,
        ports: &[(Tcp, 22)],
        does: "lets people log in to this computer from other computers",
        tradeoff: "You can no longer log in to this computer from another one with ssh. Your own ssh to other computers, git over SSH and scp to servers keep working.",
    },
    Daemon {
        key: "cups-browsed",
        rule: "harden.service.cups_browsed",
        name: "cups-browsed",
        units: &["cups-browsed.service"],
        ports: &[(Udp, 631)],
        does: "adds printers it hears about on the network to your printer list",
        tradeoff: "Network printers no longer get permanent entries by themselves, and ones cups-browsed added may leave the list. Printers you added yourself keep working, and most programs still find driverless network printers through CUPS directly.",
    },
    Daemon {
        key: "rpcbind",
        rule: "harden.service.network",
        name: "rpcbind",
        units: &["rpcbind.service", "rpcbind.socket"],
        ports: &[(Tcp, 111), (Udp, 111)],
        does: "is needed only for NFS version 3 and NIS",
        tradeoff: "NFS version 3 shares and NIS stop working. NFS version 4 does not need it.",
    },
    Daemon {
        key: "samba",
        rule: "harden.service.network",
        name: "Samba file sharing",
        units: &["smb.service", "nmb.service", "smbd.service", "nmbd.service"],
        ports: &[(Tcp, 139), (Tcp, 445), (Udp, 137), (Udp, 138)],
        does: "shares folders on this computer with Windows and other computers",
        tradeoff: "Folders you share from this computer stop being reachable. Opening shares on other computers keeps working.",
    },
    Daemon {
        key: "vsftpd",
        rule: "harden.service.network",
        name: "the vsftpd FTP server",
        units: &["vsftpd.service"],
        ports: &[(Tcp, 21)],
        does: "serves files over FTP, which sends passwords unencrypted",
        tradeoff: "Nobody can fetch files from this computer over FTP any more.",
    },
    Daemon {
        key: "telnet",
        rule: "harden.service.network",
        name: "the telnet server",
        units: &["telnet.socket"],
        ports: &[(Tcp, 23)],
        does: "accepts remote logins with no encryption at all",
        tradeoff: "Telnet logins to this computer stop. Use SSH if you need remote logins.",
    },
    Daemon {
        key: "xrdp",
        rule: "harden.service.network",
        name: "the xrdp remote desktop server",
        units: &["xrdp.service"],
        ports: &[(Tcp, 3389)],
        does: "lets people open a desktop on this computer from other computers",
        tradeoff: "Remote desktop connections to this computer stop.",
    },
    Daemon {
        key: "cockpit",
        rule: "harden.service.network",
        name: "the Cockpit web console",
        units: &["cockpit.socket"],
        ports: &[(Tcp, 9090)],
        does: "offers an administration website for this computer",
        tradeoff: "The Cockpit website on port 9090 stops answering.",
    },
];

/// Every unit `gather` asks systemd about.
pub fn known_units() -> Vec<&'static str> {
    let mut units: Vec<&str> = DAEMONS
        .iter()
        .flat_map(|d| d.units.iter().copied())
        .collect();
    units.extend([
        "firewalld.service",
        "ufw.service",
        "nftables.service",
        "iptables.service",
        "systemd-resolved.service",
    ]);
    units
}

/// A kernel setting that limits what a program running as you can reach.
struct Knob {
    key: &'static str,
    rule: &'static str,
    file: &'static str,
    value: &'static str,
    severity: Severity,
    title: &'static str,
    found: &'static str,
    why: &'static str,
    tradeoff: &'static str,
}

const KNOBS: &[Knob] = &[
    Knob {
        key: "kernel.yama.ptrace_scope",
        rule: "harden.sysctl.ptrace_scope",
        file: "/etc/sysctl.d/90-bigman-ptrace-scope.conf",
        value: "1",
        severity: Severity::Review,
        title: "Stop programs from reading each other's memory",
        found: "Any program running as you can attach to your other programs (ptrace_scope is 0) and read their memory: your browser's cookies, an unlocked password manager, ssh-agent keys.",
        why: "With ptrace_scope at 1, a program can only debug the programs it started itself. Malware that arrives through a package or an editor extension can no longer read secrets out of programs that are already running.",
        tradeoff: "Attaching to a program that is already running (gdb -p, strace -p, reptyr, py-spy --pid) needs sudo. Starting a program under the debugger, which is what IDEs do, still works.",
    },
    Knob {
        key: "kernel.kptr_restrict",
        rule: "harden.sysctl.kptr_restrict",
        file: "/etc/sysctl.d/90-bigman-kptr-restrict.conf",
        value: "1",
        severity: Severity::Info,
        title: "Hide kernel addresses from programs",
        found: "Kernel memory addresses are visible to every program (kptr_restrict is 0).",
        why: "Kernel exploits need to know where the kernel is in memory. Hiding the addresses makes a local privilege-escalation bug harder to use.",
        tradeoff: "perf and other profilers show kernel symbols only when run with sudo.",
    },
    Knob {
        key: "kernel.dmesg_restrict",
        rule: "harden.sysctl.dmesg_restrict",
        file: "/etc/sysctl.d/90-bigman-dmesg-restrict.conf",
        value: "1",
        severity: Severity::Info,
        title: "Keep the kernel log private",
        found: "Every program can read the kernel log (dmesg_restrict is 0).",
        why: "The kernel log can reveal addresses and hardware details that help an exploit. Most distributions already restrict it.",
        tradeoff: "Reading dmesg needs sudo; journalctl -k keeps working for members of wheel or adm.",
    },
];

/// A file in a directory and its sort key: systemd-sysctl applies files in
/// filename order, so a later name wins.
fn file_name(path: &std::path::Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn ago_days(now: u64, at: u64) -> String {
    let days = now.saturating_sub(at) / 86_400;
    match days {
        0 => "today".into(),
        1 => "yesterday".into(),
        d => format!("{d} days ago"),
    }
}

/// Collects suggestions, in-place checks and coverage as rules run.
struct Builder<'a> {
    inputs: &'a Inputs,
    suggestions: Vec<Suggestion>,
    in_place: Vec<InPlace>,
    coverage: Coverage,
}

impl Builder<'_> {
    fn ok(&mut self, check: &str, detail: impl Into<String>) {
        self.coverage.checked(check);
        self.in_place.push(InPlace {
            check: check.into(),
            detail: detail.into(),
        });
    }
    fn checked(&mut self, check: &str) {
        self.coverage.checked(check);
    }
    fn skip(&mut self, check: &str, why: SkipReason, detail: impl Into<String>) {
        self.coverage.skip(check, why, detail);
    }
    #[allow(clippy::too_many_arguments)]
    fn suggest(
        &mut self,
        id: String,
        rule: &str,
        severity: Severity,
        title: &str,
        summary: String,
        evidence: String,
        why: &str,
        tradeoff: String,
        fix: Fix,
        change: Option<(Change, Change)>,
    ) {
        self.suggestions.push(Suggestion {
            id,
            rule: rule.into(),
            severity,
            title: title.into(),
            summary,
            evidence,
            why: why.into(),
            tradeoff,
            fix,
            kept: false,
            change,
        });
    }

    fn reach_of(&self, protocol: Protocol, port: u16) -> Vec<(Socket, Reach)> {
        self.inputs
            .listeners
            .iter()
            .filter(|s| s.protocol == protocol && s.local.port() == port)
            .map(|s| {
                (
                    s.clone(),
                    reach(&self.inputs.firewall, s.local.ip(), protocol, port),
                )
            })
            .collect()
    }
}

fn proto(p: Protocol) -> &'static str {
    match p {
        Tcp => "tcp",
        Udp => "udp",
    }
}

fn units_fix(d: &Daemon, units: &BTreeMap<String, UnitState>) -> (Change, Change, Vec<String>) {
    let on: Vec<(&str, &UnitState)> = d
        .units
        .iter()
        .filter_map(|u| units.get(*u).map(|s| (*u, s)))
        .filter(|(_, s)| s.is_enabled() || s.is_active())
        .collect();
    let mut disable = vec!["systemctl", "disable", "--now"];
    disable.extend(on.iter().map(|(u, _)| *u));
    let enabled: Vec<&str> = on
        .iter()
        .filter(|(_, s)| s.is_enabled())
        .map(|(u, _)| *u)
        .collect();
    let started: Vec<&str> = on
        .iter()
        .filter(|(_, s)| s.is_active() && !s.is_enabled())
        .map(|(u, _)| *u)
        .collect();
    let mut undo = vec![];
    if !enabled.is_empty() {
        let mut a = vec!["systemctl", "enable", "--now"];
        a.extend(&enabled);
        undo.push(run(&a));
    }
    if !started.is_empty() {
        let mut a = vec!["systemctl", "start"];
        a.extend(&started);
        undo.push(run(&a));
    }
    let lines = on
        .iter()
        .map(|(u, s)| format!("{u}: {}, {}", s.enabled, s.active))
        .collect();
    (
        Change::admin(vec![run(&disable)]),
        Change::admin(undo),
        lines,
    )
}

fn daemons(b: &mut Builder) {
    for d in DAEMONS {
        let check = format!("Network service: {}", d.name);
        let present: Vec<&UnitState> = d
            .units
            .iter()
            .filter_map(|u| b.inputs.units.get(*u))
            .filter(|s| s.installed())
            .collect();
        if b.inputs.units.is_empty() {
            b.skip(
                &check,
                SkipReason::Failed,
                "systemd unit states could not be read",
            );
            continue;
        }
        if present.is_empty() {
            b.checked(&check);
            continue;
        }
        if !present.iter().any(|s| s.is_enabled() || s.is_active()) {
            b.ok(&check, "Installed, but off.");
            continue;
        }
        let (change, undo, unit_lines) = units_fix(d, &b.inputs.units);
        let mut evidence = unit_lines;
        let mut exposed = false;
        let mut reaches = vec![];
        for (p, port) in d.ports {
            for (s, r) in b.reach_of(*p, *port) {
                exposed |= r.exposed();
                evidence.push(format!("{} {} {}", proto(*p), s.state, s.local));
                reaches.push(r.sentence());
            }
        }
        reaches.dedup();
        let id = if d.rule == "harden.service.network" {
            format!("{}:{}", d.rule, d.key)
        } else {
            d.rule.to_string()
        };
        let mut summary = format!("{} is on. It {}.", capital(d.name), d.does);
        if reaches.is_empty() {
            summary.push_str(" It is not listening on its usual port right now.");
        } else {
            summary.push(' ');
            summary.push_str(&reaches.join(" "));
        }
        let mut severity = if exposed {
            Severity::Review
        } else {
            Severity::Info
        };
        let mut fix = Fix::Bigman {
            admin: true,
            verb: "Turn off".into(),
        };
        let mut tradeoff = d.tradeoff.to_string();
        if d.key == "sshd" {
            match &b.inputs.logins {
                Some(h) => {
                    let days = b.inputs.now.saturating_sub(h.since) / 86_400;
                    let span = if days == 1 { "day".to_string() } else { format!("{days} days") };
                    evidence.push(format!("Login history: {}, the last {span}", h.read.join(" and ")));
                    match h.logins.iter().max_by_key(|l| l.at) {
                        None => summary.push_str(&format!(
                            " Nobody has logged in over the network in the last {span}, as far as the logs Bigman can read show."
                        )),
                        Some(last) => {
                            severity = Severity::Info;
                            summary.push_str(&format!(
                                " The last network login was {} by {} from {}. If that was you and you still need it, keep it.",
                                ago_days(b.inputs.now, last.at),
                                last.user,
                                last.from
                            ));
                            for l in h.logins.iter().rev().take(5) {
                                evidence.push(format!("Login {} by {} from {}", ago_days(b.inputs.now, l.at), l.user, l.from));
                            }
                        }
                    }
                }
                None => summary.push_str(" Bigman could not read the login history, so it cannot tell whether you use it."),
            }
            if let Some(s) = b.inputs.remote_sessions.first() {
                fix = Fix::Blocked {
                    reason: format!("You are logged in over the network right now ({s}). Turning the SSH server off would stop you from reconnecting, so do it from this computer's own keyboard."),
                };
            }
            tradeoff.push_str(" If you need it again, the undo turns it back on.");
        }
        let title = format!("Turn off {}", d.name);
        b.suggest(
            id,
            d.rule,
            severity,
            &title,
            summary,
            evidence.join("\n"),
            "Every service that accepts connections is a way in. One you do not use only adds risk: a weak password, a leaked key or an unpatched bug in it is enough.",
            tradeoff,
            fix,
            Some((change, undo)),
        );
        b.coverage.checked(check);
    }
}

fn capital(s: &str) -> String {
    let mut c = s.chars();
    c.next()
        .map(|f| f.to_uppercase().collect::<String>() + c.as_str())
        .unwrap_or_default()
}

/// Listeners other computers could connect to, as evidence lines.
fn exposed_listeners(b: &Builder, only: impl Fn(&Socket) -> bool) -> Vec<String> {
    b.inputs
        .listeners
        .iter()
        .filter(|s| s.listening() || s.protocol == Udp)
        .filter(|s| only(s))
        .filter(|s| reach(&b.inputs.firewall, s.local.ip(), s.protocol, s.local.port()).exposed())
        .map(|s| {
            let who = s
                .process
                .as_ref()
                .map(|p| p.name.clone())
                .unwrap_or_else(|| {
                    if s.uid == 0 {
                        "a system program".into()
                    } else {
                        format!("uid {}", s.uid)
                    }
                });
            format!("{who} on {} {}", proto(s.protocol), s.local)
        })
        .collect()
}

fn firewall_rules(b: &mut Builder) {
    let check = "Firewall";
    match &b.inputs.firewall {
        FirewallState::Off { installed } => {
            let exposed = exposed_listeners(b, |s| s.protocol == Tcp);
            let severity = if exposed.is_empty() {
                Severity::Info
            } else {
                Severity::Review
            };
            let (fix, change) = match installed.as_deref() {
                Some("firewalld") => (
                    Fix::Bigman { admin: true, verb: "Turn on".into() },
                    Some((
                        Change::admin(vec![run(&["systemctl", "enable", "--now", "firewalld.service"])]),
                        Change::admin(vec![run(&["systemctl", "disable", "--now", "firewalld.service"])]),
                    )),
                ),
                Some("ufw") => (
                    Fix::Bigman { admin: true, verb: "Turn on".into() },
                    Some((
                        Change::admin(vec![run(&["ufw", "--force", "enable"])]),
                        Change::admin(vec![run(&["ufw", "disable"])]),
                    )),
                ),
                _ => (
                    Fix::Manual {
                        steps: "Install firewalld (Fedora, openSUSE) or ufw (Debian, Ubuntu, Arch) with your package manager, then turn it on: sudo systemctl enable --now firewalld, or sudo ufw enable.".into(),
                    },
                    None,
                ),
            };
            let mut summary = "No firewall is running.".to_string();
            summary.push_str(if exposed.is_empty() {
                " Nothing is listening for other computers right now, but anything that starts listening later is reachable at once."
            } else {
                " These programs are reachable from every network you join:"
            });
            b.suggest(
                "harden.firewall.off".into(),
                "harden.firewall.off",
                severity,
                "Turn on the firewall",
                summary,
                exposed.join("\n"),
                "A firewall decides which listening programs other computers may reach, so a program that opens a port by mistake, or a service you forgot, is not exposed on café or office Wi-Fi.",
                "Programs that accept connections from other computers (game servers, KDE Connect, development servers) stop being reachable until you allow them. Outgoing connections are not affected.".into(),
                fix,
                change,
            );
            b.checked(check);
        }
        FirewallState::Firewalld { zones } => {
            b.ok(
                check,
                format!(
                    "firewalld is running: {}.",
                    zones
                        .iter()
                        .map(|z| z.place())
                        .collect::<Vec<_>>()
                        .join("; ")
                ),
            );
            for z in zones
                .iter()
                .filter(|z| !z.interfaces.is_empty() || !z.sources.is_empty())
            {
                let wide: Vec<&firewall::PortRange> = z
                    .ports
                    .iter()
                    .filter(|r| r.width() >= 1000 && r.to > 1024)
                    .collect();
                if wide.is_empty() {
                    continue;
                }
                let through = exposed_listeners(b, |s| {
                    wide.iter().any(|r| r.contains(s.protocol, s.local.port()))
                        && z.allows(s.protocol, s.local.port())
                            .is_some_and(|w| w.starts_with("it opens"))
                });
                let specs: Vec<String> = wide.iter().map(|r| r.spec()).collect();
                let mut remove = vec![
                    "firewall-cmd".to_string(),
                    "--permanent".into(),
                    format!("--zone={}", z.name),
                ];
                let mut add = remove.clone();
                remove.extend(specs.iter().map(|s| format!("--remove-port={s}")));
                add.extend(specs.iter().map(|s| format!("--add-port={s}")));
                let reload = run(&["firewall-cmd", "--reload"]);
                let change = Change::admin(vec![Step::Run { argv: remove }, reload.clone()]);
                let undo = Change::admin(vec![Step::Run { argv: add }, reload]);
                let mut summary = format!(
                    "The {} zone lets other computers reach any program that listens on {}. It applies to {}.",
                    z.name,
                    specs.join(" and "),
                    z.place().trim_start_matches(&format!("the {} zone ", z.name))
                );
                summary.push_str(if through.is_empty() {
                    " Nothing uses that opening right now."
                } else {
                    " These programs are reachable only because of it:"
                });
                b.suggest(
                    format!("harden.firewall.open_range:{}", z.name),
                    "harden.firewall.open_range",
                    if through.is_empty() { Severity::Info } else { Severity::Review },
                    &format!("Close the open port range in the {} zone", z.name),
                    summary,
                    through.join("\n"),
                    "Fedora Workstation opens every port above 1024 so that programs you start just work. It also means a development server, a debugger or malware listening on a high port is reachable from the whole network.",
                    format!(
                        "Programs that wait for connections from other computers on high ports (KDE Connect and GSConnect, Spotify Connect, Steam Remote Play, game and development servers) stop being reachable on these networks. Allow one again with: sudo firewall-cmd --permanent --zone={} --add-port=PORT/tcp && sudo firewall-cmd --reload",
                        z.name
                    ),
                    Fix::Bigman { admin: true, verb: "Close".into() },
                    Some((change, undo)),
                );
            }
        }
        FirewallState::Ufw => b.ok(
            check,
            "ufw is on. Its rules are readable only by an administrator.",
        ),
        FirewallState::Unknown { reason } => {
            b.skip(check, SkipReason::PermissionDenied, reason.clone())
        }
    }
}

fn sysctl_rules(b: &mut Builder) {
    for k in KNOBS {
        let check = format!("Kernel setting {}", k.key);
        let Some(state) = b.inputs.sysctl.get(k.key) else {
            b.skip(
                &check,
                SkipReason::NotApplicable,
                "This kernel does not have the setting",
            );
            continue;
        };
        if state.value.trim() != "0" {
            b.ok(&check, format!("{} is {}.", k.key, state.value.trim()));
            continue;
        }
        let ours = file_name(std::path::Path::new(k.file));
        let mut evidence = vec![format!("{} = {}", k.key, state.value.trim())];
        for s in &state.set_by {
            evidence.push(format!(
                "Set to {} by {}{}",
                s.value,
                s.file.display(),
                s.package
                    .as_deref()
                    .map(|p| format!(" (installed by the {p} package)"))
                    .unwrap_or_default()
            ));
        }
        let mut summary = k.found.to_string();
        if let Some(s) = state.set_by.iter().rev().find(|s| s.value == "0") {
            summary.push_str(&match &s.package {
                Some(p) => format!(" The {p} package sets it, in {}.", s.file.display()),
                None => format!(" {} sets it.", s.file.display()),
            });
        }
        let later = state
            .set_by
            .iter()
            .find(|s| file_name(&s.file) > ours && s.value != k.value);
        let fix = match later {
            Some(s) => Fix::Blocked {
                reason: format!(
                    "{} sets it to {} and is read after Bigman's file, so it would undo the change at the next boot. Edit that file instead.",
                    s.file.display(),
                    s.value
                ),
            },
            None => Fix::Bigman { admin: true, verb: "Change".into() },
        };
        let content = format!(
            "# Written by Bigman. Remove this file to undo.\n{} = {}\n",
            k.key, k.value
        );
        let change = Change::admin(vec![
            Step::Write {
                path: k.file.into(),
                content,
            },
            run(&["sysctl", "-w", &format!("{}={}", k.key, k.value)]),
        ]);
        let undo = Change::admin(vec![
            Step::Remove {
                path: k.file.into(),
            },
            run(&["sysctl", "-w", &format!("{}={}", k.key, state.value.trim())]),
        ]);
        b.suggest(
            k.rule.into(),
            k.rule,
            k.severity,
            k.title,
            summary,
            evidence.join("\n"),
            k.why,
            k.tradeoff.into(),
            fix,
            Some((change, undo)),
        );
        b.checked(&check);
    }
}

const LLMNR_DROPIN: &str = "/etc/systemd/resolved.conf.d/90-bigman-llmnr.conf";

fn llmnr_rule(b: &mut Builder) {
    let check = "Name lookups on the local network (LLMNR)";
    let resolved = b
        .inputs
        .units
        .get("systemd-resolved.service")
        .is_some_and(|u| u.is_active());
    match b.inputs.llmnr.as_deref() {
        _ if !resolved => b.checked(check),
        None => b.skip(
            check,
            SkipReason::Failed,
            "resolvectl status did not report the LLMNR setting",
        ),
        Some("no") => b.ok(check, "systemd-resolved does not use LLMNR."),
        Some(mode) => {
            let restart = run(&["systemctl", "restart", "systemd-resolved.service"]);
            let change = Change::admin(vec![
                run(&["mkdir", "-p", "/etc/systemd/resolved.conf.d"]),
                Step::Write {
                    path: LLMNR_DROPIN.into(),
                    content:
                        "# Written by Bigman. Remove this file to undo.\n[Resolve]\nLLMNR=no\n"
                            .into(),
                },
                restart.clone(),
            ]);
            let undo = Change::admin(vec![
                Step::Remove {
                    path: LLMNR_DROPIN.into(),
                },
                restart,
            ]);
            b.suggest(
                "harden.name.llmnr".into(),
                "harden.name.llmnr",
                Severity::Info,
                "Stop asking the local network for computer names",
                format!("When a name is not in DNS, this computer asks everyone on the local network for it (LLMNR is \"{mode}\"). Any device on the same Wi-Fi can answer and send you to itself."),
                format!("resolvectl status: LLMNR={mode}"),
                "On public Wi-Fi, any device on the network can answer these questions, to collect sign-in attempts or redirect connections. Names ending in .local (mDNS) and normal DNS are not affected.",
                "Short names of computers on a network without a DNS server (\"nas\", \"printer\") may stop resolving. Use their .local name or address instead.".into(),
                Fix::Bigman { admin: true, verb: "Turn off".into() },
                Some((change, undo)),
            );
            b.checked(check);
        }
    }
}

fn screen_lock_rule(b: &mut Builder) {
    let check = "Screen lock";
    let Some(lock) = &b.inputs.screen_lock else {
        b.skip(
            check,
            SkipReason::NotApplicable,
            "Only GNOME and KDE Plasma lock settings are read",
        );
        return;
    };
    let never = !lock.auto_lock || lock.minutes == 0;
    if !never && lock.on_resume {
        b.ok(
            check,
            format!(
                "The screen locks after {} minutes and on resume.",
                lock.minutes
            ),
        );
        return;
    }
    let mut found = vec![];
    if !lock.auto_lock {
        found.push("does not lock by itself when you step away".to_string());
    } else if lock.minutes == 0 {
        found.push("never goes idle, so it never locks by itself".to_string());
    }
    if !lock.on_resume {
        found.push("does not lock when the computer wakes from sleep".to_string());
    }
    let raw = |k: &str| lock.raw.get(k).cloned().flatten();
    let (change, undo, evidence) = match lock.desktop {
        Desktop::Kde => {
            let mut set = vec![];
            let mut put_back = vec![];
            let write = |key: &str, value: &str| {
                run(&[
                    "kwriteconfig6",
                    "--file",
                    "kscreenlockerrc",
                    "--group",
                    "Daemon",
                    "--key",
                    key,
                    value,
                ])
            };
            let mut touch = |key: &str, value: &str| {
                set.push(write(key, value));
                put_back.push(match raw(key) {
                    Some(old) => write(key, &old),
                    None => run(&[
                        "kwriteconfig6",
                        "--file",
                        "kscreenlockerrc",
                        "--group",
                        "Daemon",
                        "--key",
                        key,
                        "--delete",
                    ]),
                });
            };
            if !lock.auto_lock {
                touch("Autolock", "true");
            }
            if lock.minutes == 0 {
                touch("Timeout", "5");
            }
            if !lock.on_resume {
                touch("LockOnResume", "true");
            }
            let reload = run(&[
                "dbus-send",
                "--session",
                "--type=method_call",
                "--dest=org.freedesktop.ScreenSaver",
                "/ScreenSaver",
                "org.kde.screensaver.configure",
            ]);
            set.push(reload.clone());
            put_back.push(reload);
            let evidence = ["Autolock", "Timeout", "LockOnResume"]
                .iter()
                .map(|k| {
                    format!(
                        "kscreenlockerrc [Daemon] {k}={}",
                        raw(k).unwrap_or_else(|| "(default)".into())
                    )
                })
                .collect::<Vec<_>>();
            (Change::user(set), Change::user(put_back), evidence)
        }
        Desktop::Gnome => {
            let mut set = vec![];
            let mut put_back = vec![];
            let mut touch = |schema: &str, key: &str, value: &str| {
                set.push(run(&["gsettings", "set", schema, key, value]));
                put_back.push(match raw(key) {
                    Some(old) => run(&["gsettings", "set", schema, key, &old]),
                    None => run(&["gsettings", "reset", schema, key]),
                });
            };
            if !lock.auto_lock {
                touch("org.gnome.desktop.screensaver", "lock-enabled", "true");
            }
            if lock.minutes == 0 {
                touch("org.gnome.desktop.session", "idle-delay", "uint32 300");
            }
            if !lock.on_resume {
                touch(
                    "org.gnome.desktop.screensaver",
                    "ubuntu-lock-on-suspend",
                    "true",
                );
            }
            let evidence = ["lock-enabled", "idle-delay", "ubuntu-lock-on-suspend"]
                .iter()
                .filter_map(|k| raw(k).map(|v| format!("{k} {v}")))
                .collect();
            (Change::user(set), Change::user(put_back), evidence)
        }
    };
    // A key an administrator locked cannot be written as you, so the change would fail.
    let locked: Vec<&str> = [
        (!lock.auto_lock, "lock-enabled"),
        (lock.minutes == 0, "idle-delay"),
        (!lock.on_resume, "ubuntu-lock-on-suspend"),
    ]
    .into_iter()
    .filter(|(needed, key)| *needed && lock.locked.iter().any(|l| l == key))
    .map(|(_, key)| key)
    .collect();
    let mut evidence = evidence;
    evidence.extend(
        locked
            .iter()
            .map(|k| format!("{k}: locked by an administrator (dconf)")),
    );
    let fix = if locked.is_empty() {
        Fix::Bigman {
            admin: false,
            verb: "Turn on".into(),
        }
    } else {
        Fix::Blocked {
            reason: format!(
                "An administrator has locked {} for every account on this computer (a dconf lock under /etc/dconf/db), so only they can change it.",
                locked.join(" and ")
            ),
        }
    };
    b.suggest(
        "harden.screen_lock".into(),
        "harden.screen_lock",
        Severity::Review,
        "Lock the screen when you step away",
        format!("Your screen {}.", found.join(" and ")),
        evidence.join("\n"),
        "An unlocked session lets anyone at the keyboard read your mail, use your logged-in accounts or run a command as you in seconds.",
        "You type your password when you come back after five minutes away, or after the computer sleeps. You can pick a longer delay in your desktop's screen-locking settings.".into(),
        fix,
        Some((change, undo)),
    );
    b.checked(check);
}

fn root_encrypted(inputs: &Inputs) -> Option<bool> {
    inputs.encryption.as_ref().map(|m| {
        m.iter()
            .filter(|m| m.mount == "/" || m.mount == "/home")
            .all(|m| m.encrypted)
    })
}

fn boot_rules(b: &mut Builder) {
    let check = "Automatic login";
    if b.inputs.autologin.is_empty() {
        b.ok(check, "Nobody is logged in automatically at startup.");
    } else {
        let a = &b.inputs.autologin[0];
        let unencrypted = root_encrypted(b.inputs) == Some(false);
        let steps = match a.manager.as_str() {
            "gdm" => format!("Open Settings > Users and turn off Automatic Login, or, as administrator, delete the AutomaticLoginEnable and AutomaticLogin lines from {}.", a.file.display()),
            "lightdm" => format!("As administrator, delete the autologin-user line from {}.", a.file.display()),
            _ => format!("Open System Settings > Login Screen > Behavior and turn off automatic login, or, as administrator, delete the User= line under [Autologin] in {}.", a.file.display()),
        };
        b.suggest(
            "harden.autologin".into(),
            "harden.autologin",
            if unencrypted { Severity::Review } else { Severity::Info },
            "Ask for a password at startup",
            format!(
                "{} logs in {} at startup without a password.{}",
                a.manager,
                a.user,
                if unencrypted { " The disk is not encrypted either, so anyone who switches the computer on gets your session." } else { "" }
            ),
            b.inputs.autologin.iter().map(|a| format!("{}: logs in {}", a.file.display(), a.user)).collect::<Vec<_>>().join("\n"),
            "Whoever turns the computer on, or steals it, is logged in as you with your browser sessions and keys.",
            "You type your password once after each start.".into(),
            Fix::Manual { steps },
            None,
        );
        b.checked(check);
    }

    let check = "Disk encryption";
    match root_encrypted(b.inputs) {
        None => b.skip(
            check,
            SkipReason::Failed,
            "lsblk did not describe the disks",
        ),
        Some(true) => b.ok(
            check,
            "The system and home folders are on an encrypted disk.",
        ),
        Some(false) => {
            let mounts = b
                .inputs
                .encryption
                .iter()
                .flatten()
                .filter(|m| m.mount == "/" || m.mount == "/home")
                .map(|m| {
                    format!(
                        "{}: {}",
                        m.mount,
                        if m.encrypted {
                            "encrypted"
                        } else {
                            "not encrypted"
                        }
                    )
                })
                .collect::<Vec<_>>();
            b.suggest(
                "harden.disk_encryption".into(),
                "harden.disk_encryption",
                Severity::Info,
                "Encrypt the disk",
                "Your files are stored unencrypted. Anyone who takes the disk out, or boots the laptop from a USB stick, can read them.".into(),
                mounts.join("\n"),
                "Full-disk encryption protects everything on a lost or stolen computer, including saved passwords and SSH keys.",
                "You type a passphrase every time the computer starts.".into(),
                Fix::Manual {
                    steps: "Encryption is chosen when installing. Back up your files, then reinstall with \"Encrypt my data\" (Fedora) or \"Use LVM and encryption\" (Ubuntu, Debian). Bigman cannot encrypt a disk that is in use.".into(),
                },
                None,
            );
            b.checked(check);
        }
    }

    let check = "Secure Boot";
    match b.inputs.secure_boot {
        None => b.skip(
            check,
            SkipReason::NotApplicable,
            "This computer did not start with UEFI, or the Secure Boot variable is unreadable",
        ),
        Some(true) => b.ok(check, "Secure Boot is on."),
        Some(false) => {
            b.suggest(
                "harden.secure_boot".into(),
                "harden.secure_boot",
                Severity::Info,
                "Turn on Secure Boot",
                "Secure Boot is off, so the firmware starts any boot loader and kernel, including a modified one.".into(),
                "SecureBoot EFI variable: 0".into(),
                "Secure Boot stops boot-level malware (bootkits) that would load before Linux and hide from everything running after it.",
                "Kernel modules you built or installed outside the distribution (NVIDIA from a third-party repository, VirtualBox) stop loading until their signing key is enrolled.".into(),
                Fix::Manual {
                    steps: "If you use out-of-tree kernel modules, enroll their signing key first (for example sudo mokutil --import with the key your distribution documents). Then restart, open the firmware setup (usually F2, F12 or Delete at power-on) and turn Secure Boot on.".into(),
                },
                None,
            );
            b.checked(check);
        }
    }
}

fn account_rules(b: &mut Builder) {
    let check = "Groups that grant administrator access";
    let user = b.inputs.user.clone();
    if b.inputs.groups.iter().any(|g| g == "docker") && !user.is_empty() {
        b.suggest(
            "harden.group.docker".into(),
            "harden.group.docker",
            Severity::Review,
            "Leave the docker group",
            format!("{user} is in the docker group. Any program you run can start a container that mounts the whole disk, which is the same as having administrator rights without a password."),
            format!("{user} is a member of docker"),
            "Malware running as you would otherwise become root through the Docker socket, without any prompt.",
            "docker commands need sudo, or switch to rootless Docker or Podman, which run containers without root. The change applies from your next login.".into(),
            Fix::Bigman { admin: true, verb: "Leave".into() },
            Some((
                Change::admin(vec![run(&["gpasswd", "-d", &user, "docker"])]),
                Change::admin(vec![run(&["usermod", "-aG", "docker", &user])]),
            )),
        );
        b.checked(check);
    } else if b.inputs.groups.is_empty() {
        b.skip(
            check,
            SkipReason::Unreadable,
            "/etc/group could not be read",
        );
    } else {
        b.ok(check, "You are not in the docker group.");
    }

    let check = "Home folder privacy";
    match &b.inputs.home {
        None => b.skip(
            check,
            SkipReason::Unreadable,
            "The home folder's permissions could not be read",
        ),
        Some(h) if h.mode & 0o007 == 0 => b.ok(
            check,
            format!(
                "Only you can open {} (mode {:o}).",
                h.path.display(),
                h.mode & 0o777
            ),
        ),
        Some(h) => {
            let path = h.path.to_string_lossy().into_owned();
            let new = h.mode & 0o770;
            b.suggest(
                "harden.home.readable".into(),
                "harden.home.readable",
                if h.other_accounts.is_empty() { Severity::Info } else { Severity::Review },
                "Make your home folder private",
                format!(
                    "Other accounts on this computer can open {path} (mode {:o}).{}",
                    h.mode & 0o777,
                    if h.other_accounts.is_empty() { String::new() } else { format!(" Accounts that can: {}.", h.other_accounts.join(", ")) }
                ),
                format!("{path} mode {:o}", h.mode & 0o777),
                "Files you never shared, such as documents, shell history and project folders, are readable by other people's accounts and by system services that get compromised.",
                "Other accounts can no longer open files in your home folder, including a ~/public_html website if you serve one.".into(),
                Fix::Bigman { admin: false, verb: "Make private".into() },
                Some((
                    Change::user(vec![run(&["chmod", &format!("{new:o}"), &path])]),
                    Change::user(vec![run(&["chmod", &format!("{:o}", h.mode & 0o7777), &path])]),
                )),
            );
            b.checked(check);
        }
    }

    let check = "npm install scripts";
    match b.inputs.npm.as_ref() {
        None => b.checked(check),
        Some(n) if n.ignore_scripts == "true" => b.ok(
            check,
            "npm does not run dependency install scripts (ignore-scripts=true).",
        ),
        Some(n) if n.blocks_by_default() => b.ok(
            check,
            format!(
                "npm {} runs dependency install scripts only for packages your project allows.",
                n.version
            ),
        ),
        Some(n) => {
            let value = &n.ignore_scripts;
            b.suggest(
                "harden.npm.ignore_scripts".into(),
                "harden.npm.ignore_scripts",
                Severity::Info,
                "Stop npm from running install scripts",
                format!("npm runs every dependency's install scripts during npm install (ignore-scripts is {value})."),
                format!("npm {}\nnpm config get ignore-scripts: {value}", n.version),
                "Self-spreading npm worms (Shai-Hulud in 2025) ran from postinstall scripts, stealing tokens the moment a poisoned version was installed. npm 12, pnpm 10 and Bun skip these scripts by default; this npm does not.",
                "Packages that build something at install time (native add-ons, esbuild, sharp, Prisma) need npm rebuild <package> or an explicit allow. Bigman's Inspect tab shows which of your packages have install scripts.".into(),
                Fix::Bigman { admin: false, verb: "Turn off scripts".into() },
                Some((
                    Change::user(vec![run(&["npm", "config", "set", "ignore-scripts", "true", "--location=user"])]),
                    Change::user(vec![run(&["npm", "config", "delete", "ignore-scripts", "--location=user"])]),
                )),
            );
            b.checked(check);
        }
    }
}

/// The hardening rules over gathered inputs. Suggestions in `kept` are marked
/// and listed last.
pub fn assess(
    inputs: &Inputs,
    kept: &BTreeSet<String>,
) -> (Vec<Suggestion>, Vec<InPlace>, Coverage) {
    let mut b = Builder {
        inputs,
        suggestions: vec![],
        in_place: vec![],
        coverage: Coverage::default(),
    };
    daemons(&mut b);
    firewall_rules(&mut b);
    sysctl_rules(&mut b);
    llmnr_rule(&mut b);
    screen_lock_rule(&mut b);
    boot_rules(&mut b);
    account_rules(&mut b);
    for (what, why, detail) in &inputs.unread {
        b.skip(what, *why, detail.clone());
    }
    for s in &mut b.suggestions {
        s.kept = kept.contains(&s.id);
    }
    b.suggestions
        .sort_by_key(|s| (s.kept, std::cmp::Reverse(s.severity)));
    b.coverage.push("Read-only: settings, service states and logs were read without administrator rights. Nothing changes until you confirm a previewed change.".into());
    (b.suggestions, b.in_place, b.coverage)
}

const KEPT: &str = "hardening-kept.json";
const APPLIED: &str = "hardening-applied.json";

fn kept(store: &Store) -> BTreeSet<String> {
    store.read(KEPT).unwrap_or_default()
}

fn applied(store: &Store) -> Vec<AppliedChange> {
    store.read(APPLIED).unwrap_or_default()
}

/// The report form of a review: one finding per open suggestion.
pub fn report_of(suggestions: &[Suggestion], coverage: Coverage) -> Report {
    let mut r = Report::new("Hardening review", "This computer's settings");
    for s in suggestions.iter().filter(|s| !s.kept) {
        let next = match &s.fix {
            Fix::Bigman { .. } => format!(
                "{}. Preview the change in Protect > Hardening or with bigman harden {}.",
                s.title, s.id
            ),
            Fix::Manual { steps } => steps.clone(),
            Fix::Blocked { reason } => reason.clone(),
        };
        r.add(&s.rule, s.severity, &s.summary, &s.evidence, &next);
    }
    let kept = suggestions.iter().filter(|s| s.kept).count();
    r.coverage = coverage;
    if kept > 0 {
        r.coverage.push(format!(
            "You chose to leave {kept} setting{} as {}; {} not counted.",
            if kept == 1 { "" } else { "s" },
            if kept == 1 { "it is" } else { "they are" },
            if kept == 1 { "it is" } else { "they are" }
        ));
    }
    r
}

/// Gathers, assesses and reads Bigman's own records.
pub fn review(store: &Store, stop: &Cancel) -> Result<HardeningReview> {
    let inputs = gather::gather(stop);
    let (suggestions, in_place, coverage) = assess(&inputs, &kept(store));
    Ok(HardeningReview {
        report: report_of(&suggestions, coverage),
        suggestions,
        in_place,
        applied: applied(store),
    })
}

/// The open suggestion with this id and the change Bigman would make for it.
pub fn find(store: &Store, id: &str, stop: &Cancel) -> Result<(Suggestion, Change, Change)> {
    let review = review(store, stop)?;
    let s = review
        .suggestions
        .into_iter()
        .find(|s| s.id == id)
        .context(
            "This suggestion no longer applies: the setting may have changed since you last looked",
        )?;
    match &s.fix {
        Fix::Bigman { .. } => {}
        Fix::Manual { .. } => {
            bail!("Bigman does not make this change itself; follow the steps shown with it")
        }
        Fix::Blocked { reason } => bail!("{reason}"),
    }
    let (change, undo) = s
        .change
        .clone()
        .context("This suggestion has no change Bigman can make")?;
    Ok((s, change, undo))
}

/// Makes the change, records how to undo it, and checks that it took.
pub fn apply(store: &Store, id: &str, stop: &Cancel) -> Result<String> {
    let (s, change, undo) = find(store, id, stop)?;
    change.apply(stop)?;
    let mut done = applied(store);
    done.retain(|a| a.id != s.id);
    done.push(AppliedChange {
        id: s.id.clone(),
        title: s.title.clone(),
        at: now(),
        undo: undo.describe(),
        revert: undo,
    });
    store.write(APPLIED, &done)?;
    let after = review(store, stop)?;
    Ok(if after.suggestions.iter().any(|x| x.id == s.id) {
        format!("{}: the commands finished, but Bigman still sees the old setting. Some changes apply from your next login; check again then.", s.title)
    } else {
        format!(
            "{}: done. Bigman checked again and the new setting is in place.",
            s.title
        )
    })
}

/// The recorded undo for a change Bigman made.
pub fn applied_change(store: &Store, id: &str) -> Result<AppliedChange> {
    applied(store)
        .into_iter()
        .find(|a| a.id == id)
        .context("Bigman has no record of making this change")
}

pub fn undo(store: &Store, id: &str, stop: &Cancel) -> Result<String> {
    let a = applied_change(store, id)?;
    a.revert.apply(stop)?;
    let mut done = applied(store);
    done.retain(|x| x.id != id);
    store.write(APPLIED, &done)?;
    Ok(format!("Put back as it was before \"{}\".", a.title))
}

/// Stops (or resumes) suggesting this change.
pub fn keep(store: &Store, id: &str, keep: bool) -> Result<()> {
    let mut set = kept(store);
    if keep {
        set.insert(id.into());
    } else {
        set.remove(id);
    }
    store.write(KEPT, &set)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sshd_on() -> Inputs {
        let mut units = BTreeMap::new();
        units.insert(
            "sshd.service".into(),
            UnitState {
                load: "loaded".into(),
                enabled: "enabled".into(),
                active: "active".into(),
            },
        );
        units.insert(
            "sshd.socket".into(),
            UnitState {
                load: "loaded".into(),
                enabled: "disabled".into(),
                active: "inactive".into(),
            },
        );
        Inputs {
            now: 1_800_000_000,
            user: "user".into(),
            units,
            listeners: serde_json::from_str(r#"[{"protocol":"tcp","local":"0.0.0.0:22","remote":null,"state":"listen","uid":0,"process":null}]"#).unwrap(),
            firewall: FirewallState::Off { installed: None },
            logins: Some(LoginHistory { since: 1_800_000_000 - 40 * 86_400, logins: vec![], read: vec!["the system journal".into()] }),
            ..Default::default()
        }
    }

    #[test]
    fn an_unused_ssh_server_open_to_every_network_is_worth_a_look() {
        let (s, _, _) = assess(&sshd_on(), &BTreeSet::new());
        let ssh = s.iter().find(|s| s.id == "harden.sshd").unwrap();
        assert_eq!(ssh.severity, Severity::Review);
        assert!(
            ssh.summary.contains("No firewall is running"),
            "{}",
            ssh.summary
        );
        assert!(
            ssh.summary.contains("in the last 40 days"),
            "{}",
            ssh.summary
        );
        let (change, undo) = ssh.change.clone().unwrap();
        assert_eq!(
            change.admin_command(),
            ["pkexec", "systemctl", "disable", "--now", "sshd.service"]
        );
        assert_eq!(
            undo.admin_command(),
            ["pkexec", "systemctl", "enable", "--now", "sshd.service"]
        );
    }

    #[test]
    fn recent_logins_and_a_remote_session_change_the_advice() {
        let mut i = sshd_on();
        i.logins.as_mut().unwrap().logins.push(RemoteLogin {
            at: i.now - 86_400,
            user: "user".into(),
            from: "192.0.2.4".into(),
        });
        i.remote_sessions.push("session 7 from 192.0.2.4".into());
        let (s, _, _) = assess(&i, &BTreeSet::new());
        let ssh = s.iter().find(|s| s.id == "harden.sshd").unwrap();
        assert_eq!(ssh.severity, Severity::Info);
        assert!(ssh.summary.contains("yesterday by user from 192.0.2.4"));
        assert!(matches!(ssh.fix, Fix::Blocked { .. }));
    }

    #[test]
    fn kept_suggestions_go_last_and_leave_the_report() {
        let mut i = sshd_on();
        i.groups = vec!["docker".into()];
        let kept: BTreeSet<String> = ["harden.sshd".to_string()].into();
        let (s, _, coverage) = assess(&i, &kept);
        assert!(s.last().unwrap().kept);
        let r = report_of(&s, coverage);
        assert!(r.findings.iter().all(|f| f.rule != "harden.sshd"));
        assert!(r
            .coverage
            .notes
            .iter()
            .any(|n| n.contains("leave 1 setting")));
    }

    #[test]
    fn a_screen_lock_key_an_administrator_locked_is_not_offered_as_a_change() {
        let lock = |locked: Vec<String>| Inputs {
            screen_lock: Some(ScreenLock {
                desktop: Desktop::Gnome,
                auto_lock: false,
                minutes: 5,
                on_resume: true,
                raw: [("lock-enabled".to_string(), Some("false".to_string()))].into(),
                locked,
            }),
            ..Default::default()
        };
        let (s, _, _) = assess(&lock(vec![]), &BTreeSet::new());
        assert!(matches!(s[0].fix, Fix::Bigman { admin: false, .. }));
        // A lock on a key the change does not touch changes nothing.
        let (s, _, _) = assess(&lock(vec!["idle-delay".into()]), &BTreeSet::new());
        assert!(matches!(s[0].fix, Fix::Bigman { .. }));
        let (s, _, _) = assess(&lock(vec!["lock-enabled".into()]), &BTreeSet::new());
        assert!(
            matches!(&s[0].fix, Fix::Blocked { reason } if reason.contains("locked lock-enabled"))
        );
        assert!(s[0]
            .evidence
            .contains("lock-enabled: locked by an administrator"));
    }

    #[test]
    fn a_later_sysctl_file_blocks_the_change() {
        let mut i = Inputs::default();
        i.sysctl.insert(
            "kernel.yama.ptrace_scope".into(),
            SysctlState {
                value: "0".into(),
                set_by: vec![SysctlSetting {
                    file: "/etc/sysctl.d/50-cursor.conf".into(),
                    value: "0".into(),
                    package: Some("cursor".into()),
                }],
            },
        );
        let (s, _, _) = assess(&i, &BTreeSet::new());
        assert!(s[0]
            .summary
            .contains("The cursor package sets it, in /etc/sysctl.d/50-cursor.conf."));
        assert!(matches!(s[0].fix, Fix::Bigman { .. }));
        i.sysctl
            .get_mut("kernel.yama.ptrace_scope")
            .unwrap()
            .set_by
            .push(SysctlSetting {
                file: "/etc/sysctl.d/99-sysctl.conf".into(),
                value: "0".into(),
                package: None,
            });
        let (s, _, _) = assess(&i, &BTreeSet::new());
        assert!(matches!(&s[0].fix, Fix::Blocked { reason } if reason.contains("99-sysctl.conf")));
    }

    #[test]
    fn keeping_and_records_survive_in_the_store() {
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        keep(&store, "harden.sshd", true).unwrap();
        assert!(kept(&store).contains("harden.sshd"));
        keep(&store, "harden.sshd", false).unwrap();
        assert!(kept(&store).is_empty());
        assert!(applied_change(&store, "harden.sshd").is_err());
    }
}
