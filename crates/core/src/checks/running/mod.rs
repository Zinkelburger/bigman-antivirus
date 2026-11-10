//! The running system, looked at more than one way.
//!
//! `ps`, `ss` and `lsmod` each ask the kernel one question through one
//! interface, and a kernel module or an injected library can leave things out of
//! that one answer. This check asks the same question a second way and reports
//! where the answers differ: a process that answers a signal and is not listed,
//! a module in sysfs that the module list omits, a connection in one socket
//! table and not the other, a file the C library reads differently from the
//! kernel. It also lists what the ordinary tools do not show at all: programs
//! running from memory or from a deleted file, programs that read packets below
//! TCP and UDP, the BPF programs loaded in the kernel, and who watches your
//! credential folders.
//!
//! A row can be marked as known (`checks::known`), pinned to what it was, so it
//! stops counting until it changes. Rows where two views disagree cannot: no
//! ordinary program makes the kernel's own answers differ, so such a row means
//! something is hiding or Bigman misreads something, and it keeps counting.
//!
//! `gather` reads the facts without root; `assess` turns them into rows and is
//! pure, so fixtures drive it directly. A kernel module that filters every view
//! at once defeats the comparisons; the report says so.
pub mod gather;
pub mod rawsys;

use crate::{
    checks::known::{self, Fact, KnownMap, Standing},
    model::{Report, Severity, Skipped},
    runner::Cancel,
    store::Store,
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use ts_rs::TS;

pub const KIND: &str = "Running system";
pub const SUBJECT: &str = "Programs, kernel and network on this computer";

/// Which part of the running system a row is about.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Section {
    /// Two ways of asking the same question gave different answers.
    Disagreements,
    Programs,
    Kernel,
    Network,
    Watches,
}

impl Section {
    pub fn label(self) -> &'static str {
        match self {
            Section::Disagreements => "Two views that disagree",
            Section::Programs => "Programs",
            Section::Kernel => "Kernel",
            Section::Network => "Packet access",
            Section::Watches => "Watched files",
        }
    }

    /// Whether a row in this section can be marked as known.
    pub fn markable(self) -> bool {
        self != Section::Disagreements
    }
}

/// Why a row where two views disagree cannot be marked as known.
pub const NOT_MARKABLE: &str = "Rows where two views disagree cannot be marked as known. No ordinary program makes the kernel's own answers disagree, so a row that stays here means something is hiding or Bigman reads something wrongly, and it keeps counting either way. Check again: a difference that was a coincidence of timing goes away.";

/// One observed thing, with its standing against what you marked as known.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct RunningItem {
    pub section: Section,
    /// What makes this "the same thing" from one check to the next.
    pub key: String,
    pub rule: String,
    pub severity: Severity,
    pub title: String,
    pub evidence: String,
    pub action: String,
    /// Commands the action refers to, shown as copyable text. Never run.
    pub command: Option<String>,
    pub facts: Vec<Fact>,
    pub standing: Standing,
    /// When you marked it as known.
    pub known_at: Option<u64>,
    /// What differs from the version you marked.
    pub changed: Vec<String>,
}

// --- What gathering reads -----------------------------------------------------------------

/// A process that one view shows and another does not.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Unlisted {
    pub pid: u32,
    /// From looking up `/proc/PID` directly, which often still works.
    pub name: Option<String>,
    pub uid: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct PidViews {
    /// Processes and threads the kernel lists in /proc.
    pub listed: u32,
    /// How many process IDs answered a signal probe; none when the probe did not run.
    pub probed: Option<u32>,
    /// Answer a signal, and are in no listing, twice in a row.
    pub unlisted: Vec<Unlisted>,
    /// Listed by the kernel, and missing from the C library's listing.
    pub libc_missing: Vec<Unlisted>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ModuleMismatch {
    pub name: String,
    pub seen_in: Vec<String>,
    pub missing_from: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ModuleViews {
    /// Modules in /proc/modules.
    pub listed: u32,
    pub mismatched: Vec<ModuleMismatch>,
    /// In the kernel's /proc/modules, missing from the C library's reading of it.
    pub libc_missing: Vec<String>,
    /// `/proc/sys/kernel/tainted`.
    pub taint: Option<u64>,
    /// Loaded modules that carry taint marks, with their letters ("OE").
    pub tainting: BTreeMap<String, String>,
    /// Loaded modules with no file under the running kernel's module folder;
    /// none when that folder's index could not be read.
    pub no_file: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(default)]
pub struct SocketRow {
    /// "tcp" or "udp".
    pub protocol: String,
    pub local: String,
    pub remote: Option<String>,
    pub uid: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct SocketViews {
    /// Sockets in /proc/net that were compared.
    pub compared: u32,
    /// In the kernel's socket-diagnostics answer, missing from /proc/net.
    pub only_diag: Vec<SocketRow>,
    /// In /proc/net, missing from the socket-diagnostics answer.
    pub only_procfs: Vec<SocketRow>,
    /// In the kernel's /proc/net, missing from the C library's reading of it.
    pub libc_missing: Vec<SocketRow>,
}

/// `/etc/ld.so.preload` read both ways; `None` is "no such file".
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Preload {
    pub kernel: Option<String>,
    pub libc: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct StdioSocket {
    /// "TCP", "TCPv6", "UDP" ...
    pub protocol: String,
    pub remote: Option<String>,
    pub loopback: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Proc {
    pub pid: u32,
    pub uid: u32,
    /// The kernel's short name.
    pub comm: String,
    pub args: Vec<String>,
    /// A kernel thread, from the flags in /proc/PID/stat.
    pub kthread: bool,
    /// The rest is read only for your own processes.
    pub mine: bool,
    /// The executable link as the kernel prints it: "/usr/bin/x",
    /// "/tmp/x (deleted)", "/memfd:name (deleted)".
    pub exe: Option<String>,
    /// Device, inode, size and times of the file behind the link.
    pub exe_id: Option<String>,
    /// A file exists again at a deleted executable's path.
    pub replaced: bool,
    /// The AppImage file this was started from, when it was.
    pub appimage: Option<String>,
    /// Set when standard input and output are one network socket.
    pub stdio_socket: Option<StdioSocket>,
    /// Entries of LD_PRELOAD and LD_AUDIT in its environment.
    pub preload: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Holder {
    pub pid: u32,
    pub comm: String,
    pub exe: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct PacketSocket {
    /// The EtherType it receives; 3 is every packet.
    pub protocol: u16,
    /// Interface name, or "any".
    pub iface: String,
    pub uid: u32,
    /// Known only for your own processes that do not hide their descriptors.
    pub holder: Option<Holder>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RawSocket {
    /// The IP protocol number it was opened for (1 ICMP, 255 any).
    pub protocol: u16,
    pub v6: bool,
    pub uid: u32,
    pub holder: Option<Holder>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WatchKind {
    #[default]
    Credentials,
    /// The watcher's own executable file.
    OwnProgram,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Watch {
    pub pid: u32,
    pub comm: String,
    pub exe: Option<String>,
    pub exe_id: Option<String>,
    pub target: String,
    pub kind: WatchKind,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct BusName {
    pub name: String,
    pub pid: u32,
    pub comm: String,
    pub exe: Option<String>,
}

/// Everything one run read. Fixtures are this, as JSON.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Snapshot {
    pub uid: u32,
    pub home: String,
    pub pids: Option<PidViews>,
    pub modules: Option<ModuleViews>,
    pub sockets: Option<SocketViews>,
    pub preload: Option<Preload>,
    pub processes: Vec<Proc>,
    pub packet_sockets: Vec<PacketSocket>,
    pub raw_sockets: Vec<RawSocket>,
    /// Your programs that hide their open files and may open packet and raw
    /// sockets; `exe` is the first word of the command line.
    pub net_capable: Vec<Holder>,
    /// The file behind each preloaded library that is in a folder you can
    /// write to: device, inode, size and times.
    pub preload_ids: BTreeMap<String, String>,
    /// Names of the BPF programs the kernel lists among its symbols.
    pub bpf: Option<Vec<String>>,
    pub watches: Vec<Watch>,
    pub bus_names: Option<Vec<BusName>>,
    /// Comparisons and inventories that ran.
    pub checked: Vec<String>,
    /// What could not be read, and why.
    pub gaps: Vec<Skipped>,
}

// --- Rules --------------------------------------------------------------------------------

fn remedy(rule: &str) -> String {
    crate::rules::get(rule)
        .map_or("", |r| r.remediation)
        .to_string()
}

/// The commands a row's advice refers to, for the reader to run themselves.
fn command(rule: &str, pid: Option<u32>) -> Option<String> {
    let pid = pid.map_or("PID".to_string(), |p| p.to_string());
    Some(match rule {
        "process.exe-deleted" | "process.exe-memfd" | "process.exe-shm" => {
            format!("cp /proc/{pid}/exe ~/copy-of-process-{pid}\nps -o ppid=,args= -p {pid}")
        }
        "process.fake-kernel-thread" | "process.daemon-name" | "process.shell-socket" => {
            format!("ls -l /proc/{pid}/exe\nps -o ppid=,args= -p {pid}")
        }
        "bpf.programs" => "sudo bpftool prog list".into(),
        "net.packet-socket" => "sudo ss -0 -p".into(),
        "net.raw-socket" => "sudo ss -w -p".into(),
        "module.taint" | "module.no-file" => "journalctl -k -g 'module|taint'".into(),
        _ => return None,
    })
}

fn item(
    section: Section,
    rule: &str,
    key: String,
    severity: Severity,
    title: String,
    evidence: String,
    facts: Vec<Fact>,
) -> RunningItem {
    RunningItem {
        section,
        key,
        rule: rule.into(),
        severity,
        title,
        evidence: crate::model::clean(&evidence),
        action: remedy(rule),
        command: command(rule, None),
        facts,
        standing: Standing::New,
        known_at: None,
        changed: vec![],
    }
}

const SYSTEM_PREFIXES: [&str; 12] = [
    "/usr/",
    "/bin/",
    "/sbin/",
    "/lib/",
    "/lib64/",
    "/opt/",
    "/snap/",
    "/var/lib/flatpak/",
    "/var/lib/snapd/",
    "/nix/store/",
    "/gnu/store/",
    // Inside a Flatpak sandbox the application's files are under /app.
    "/app/",
];

/// Whether a path is somewhere only an administrator or a package manager writes.
pub fn system_path(path: &str) -> bool {
    SYSTEM_PREFIXES.iter().any(|p| path.starts_with(p))
}

const TEMP_PREFIXES: [&str; 4] = ["/tmp/", "/var/tmp/", "/dev/shm/", "/run/user/"];

fn temp_path(path: &str) -> bool {
    TEMP_PREFIXES.iter().any(|p| path.starts_with(p))
}

/// Names kernel threads have, which programs borrow to look like one.
const KERNEL_THREADS: [&str; 20] = [
    "kworker/",
    "ksoftirqd/",
    "migration/",
    "watchdog/",
    "cpuhp/",
    "irq/",
    "jbd2/",
    "kthreadd",
    "kswapd",
    "kcompactd",
    "khugepaged",
    "kdevtmpfs",
    "kauditd",
    "kintegrityd",
    "khungtaskd",
    "kblockd",
    "ksmd",
    "rcu_sched",
    "rcu_preempt",
    "rcu_gp",
];

fn kernel_thread_name(name: &str) -> bool {
    KERNEL_THREADS.iter().any(|k| name.starts_with(k))
}

/// `ps` prints a kernel thread's name in brackets; a program can set its own
/// title to the same text.
fn bracketed(arg: &str) -> Option<&str> {
    arg.strip_prefix('[')?.strip_suffix(']')
}

/// System services whose name on a program from somewhere else is worth a look.
const DAEMON_NAMES: [&str; 16] = [
    "sshd",
    "systemd",
    "systemd-journald",
    "systemd-logind",
    "systemd-udevd",
    "systemd-resolved",
    "cron",
    "crond",
    "dbus-daemon",
    "dbus-broker",
    "NetworkManager",
    "polkitd",
    "rsyslogd",
    "auditd",
    "agetty",
    "login",
];

const INTERPRETERS: [&str; 16] = [
    "sh", "bash", "dash", "zsh", "fish", "ksh", "ash", "busybox", "python", "python3", "perl",
    "ruby", "php", "node", "lua", "pwsh",
];

fn interpreter(comm: &str) -> bool {
    INTERPRETERS.contains(&comm)
        || ["python3.", "python2.", "lua5."]
            .iter()
            .any(|p| comm.starts_with(p))
}

/// The bus names other programs ask for your saved passwords and for
/// permission prompts.
const SENSITIVE_BUS_NAMES: [(&str, &str); 5] = [
    ("org.freedesktop.secrets", "saved passwords and keys"),
    ("org.gnome.keyring", "saved passwords and keys"),
    ("org.kde.kwalletd5", "saved passwords and keys"),
    ("org.kde.kwalletd6", "saved passwords and keys"),
    (
        "org.freedesktop.portal.Desktop",
        "file choosers, screen sharing and permission prompts",
    ),
];

/// The kernel's taint bits: letter and meaning, by bit number.
const TAINT: [(char, &str); 20] = [
    ('P', "a proprietary module was loaded"),
    ('F', "a module was force-loaded"),
    ('S', "the kernel runs on hardware outside its specification"),
    ('R', "a module was force-unloaded"),
    ('M', "the processor reported a machine check"),
    ('B', "a bad memory page was referenced"),
    ('U', "a program asked for the mark"),
    ('D', "the kernel hit an internal error (oops) since boot"),
    ('A', "an ACPI table was overridden"),
    ('W', "the kernel printed a warning"),
    ('C', "a staging driver was loaded"),
    ('I', "a firmware workaround was applied"),
    ('O', "a module built outside the kernel tree was loaded"),
    ('E', "an unsigned module was loaded"),
    ('L', "a soft lockup happened"),
    ('K', "the kernel was live-patched"),
    ('X', "a distribution-specific mark"),
    ('T', "the kernel was built with structure randomisation"),
    ('N', "an in-kernel test was run"),
    ('J', "a program used a mutating firmware-debug operation"),
];

/// The marks a module leaves, which a loaded module should account for.
const MODULE_TAINTS: [char; 5] = ['P', 'F', 'C', 'O', 'E'];

/// The set bits of a taint value as (letter, meaning).
pub fn decode_taint(value: u64) -> Vec<(char, &'static str)> {
    (0..64)
        .filter(|bit| value & (1 << bit) != 0)
        .map(|bit| TAINT.get(bit).copied().unwrap_or(('?', "an unknown mark")))
        .collect()
}

fn ethertype(protocol: u16) -> String {
    let name = match protocol {
        0x0003 => "every packet",
        0x0800 => "IPv4",
        0x0806 => "ARP",
        0x86dd => "IPv6",
        0x888e => "Wi-Fi and wired sign-in (EAPOL)",
        0x88cc => "LLDP",
        0x890d => "Wi-Fi direct links (TDLS)",
        0x88c7 => "Wi-Fi pre-authentication",
        _ => return format!("EtherType 0x{protocol:04x}"),
    };
    format!("{name} (0x{protocol:04x})")
}

fn ip_protocol(protocol: u16) -> String {
    let name = match protocol {
        1 => "ICMP",
        2 => "IGMP",
        6 => "TCP",
        17 => "UDP",
        58 => "ICMPv6",
        89 => "OSPF",
        255 => "any protocol",
        _ => return format!("IP protocol {protocol}"),
    };
    format!("{name} ({protocol})")
}

fn who(uid: u32, mine: u32) -> String {
    match uid {
        0 => "the administrator (root)".into(),
        u if u == mine => "you".into(),
        u => format!("user {u}"),
    }
}

fn pids(list: &[u32]) -> String {
    let mut text: Vec<String> = list.iter().take(12).map(|p| p.to_string()).collect();
    if list.len() > 12 {
        text.push(format!("and {} more", list.len() - 12));
    }
    text.join(", ")
}

fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

fn exe_name(exe: &str) -> &str {
    let path = exe.strip_suffix(" (deleted)").unwrap_or(exe);
    path.rsplit('/').next().unwrap_or(path)
}

fn program(comm: &str, exe: Option<&str>) -> String {
    match exe {
        Some(e) => e.strip_suffix(" (deleted)").unwrap_or(e).to_string(),
        None => comm.to_string(),
    }
}

fn disagreements(s: &Snapshot, out: &mut Vec<RunningItem>) {
    if let Some(p) = &s.pids {
        let mut by_name: BTreeMap<String, Vec<&Unlisted>> = BTreeMap::new();
        for u in &p.unlisted {
            by_name
                .entry(u.name.clone().unwrap_or_default())
                .or_default()
                .push(u);
        }
        for (name, group) in by_name {
            let ids: Vec<u32> = group.iter().map(|u| u.pid).collect();
            let shown = if name.is_empty() { "A program" } else { &name };
            let owner = group[0].uid.map(|u| who(u, s.uid));
            out.push(item(
                Section::Disagreements,
                "hidden.process",
                format!("hidden.process:{name}"),
                Severity::High,
                format!("{shown} is running and missing from the process list"),
                format!(
                    "Process {} answers a signal probe.\n/proc does not list it.\nBoth were read twice with the same result.{}",
                    pids(&ids),
                    owner.as_ref().map_or(String::new(), |o| format!("\nRuns as: {o}"))
                ),
                vec![
                    Fact::pinned("Name", if name.is_empty() { "not readable" } else { &name }),
                    Fact::shown("Process IDs", pids(&ids)),
                    Fact::shown("Listed processes and threads", p.listed),
                ],
            ));
        }
        if !p.libc_missing.is_empty() {
            let ids: Vec<u32> = p.libc_missing.iter().map(|u| u.pid).collect();
            let names: Vec<String> = p
                .libc_missing
                .iter()
                .filter_map(|u| u.name.clone())
                .collect();
            out.push(item(
                Section::Disagreements,
                "hidden.libc-view",
                "hidden.libc-view:processes".into(),
                Severity::High,
                "The process list Bigman is given leaves processes out".into(),
                format!(
                    "The kernel lists process {} in /proc.\nThe C library loaded into Bigman does not return {}.{}",
                    pids(&ids),
                    if ids.len() == 1 { "it" } else { "them" },
                    if names.is_empty() { String::new() } else { format!("\nNames: {}", names.join(", ")) }
                ),
                vec![
                    Fact::pinned("What differs", "the process list"),
                    Fact::shown("Process IDs", pids(&ids)),
                ],
            ));
        }
    }
    if let Some(m) = &s.modules {
        for mm in &m.mismatched {
            out.push(item(
                Section::Disagreements,
                "hidden.module",
                format!("hidden.module:{}", mm.name),
                Severity::High,
                format!("Kernel module {} is in one list and not another", mm.name),
                format!(
                    "{} shows a loaded module named {}.\n{} does not.\nEach was read twice with the same result.",
                    mm.seen_in.join(" and "),
                    mm.name,
                    mm.missing_from.join(" and ")
                ),
                vec![
                    Fact::pinned("Module", &mm.name),
                    Fact::pinned("Shown by", mm.seen_in.join(", ")),
                    Fact::pinned("Missing from", mm.missing_from.join(", ")),
                ],
            ));
        }
        if !m.libc_missing.is_empty() {
            out.push(item(
                Section::Disagreements,
                "hidden.libc-view",
                "hidden.libc-view:modules".into(),
                Severity::High,
                "The module list Bigman is given leaves modules out".into(),
                format!(
                    "The kernel's /proc/modules names {}.\nThe C library loaded into Bigman returns the file without {}.",
                    m.libc_missing.join(", "),
                    if m.libc_missing.len() == 1 { "it" } else { "them" }
                ),
                vec![
                    Fact::pinned("What differs", "the kernel module list"),
                    Fact::pinned("Left out", m.libc_missing.join(", ")),
                ],
            ));
        }
    }
    if let Some(v) = &s.sockets {
        for (rows, seen, missing) in [
            (
                &v.only_diag,
                "the kernel's socket diagnostics (what ss reads)",
                "/proc/net (what netstat reads)",
            ),
            (
                &v.only_procfs,
                "/proc/net (what netstat reads)",
                "the kernel's socket diagnostics (what ss reads)",
            ),
        ] {
            for r in rows {
                let what = match &r.remote {
                    Some(remote) => {
                        format!("A {} connection to {remote}", r.protocol.to_uppercase())
                    }
                    None => format!("A {} socket on {}", r.protocol.to_uppercase(), r.local),
                };
                out.push(item(
                    Section::Disagreements,
                    "hidden.net-socket",
                    format!(
                        "hidden.net-socket:{}:{}",
                        r.protocol,
                        r.remote.as_deref().unwrap_or(&r.local)
                    ),
                    Severity::High,
                    format!("{what} is in one socket table and not the other"),
                    format!(
                        "Shown by {seen}: local {}, owned by {}.\nMissing from {missing}.\nEach was read twice with the same result.",
                        r.local,
                        who(r.uid, s.uid)
                    ),
                    vec![
                        Fact::pinned("Protocol", r.protocol.to_uppercase()),
                        Fact::pinned("Remote", r.remote.as_deref().unwrap_or("none (listening or unconnected)")),
                        Fact::shown("Local", &r.local),
                        Fact::pinned("Shown by", seen),
                        Fact::pinned("Missing from", missing),
                    ],
                ));
            }
        }
        if !v.libc_missing.is_empty() {
            let rows: Vec<String> = v
                .libc_missing
                .iter()
                .map(|r| match &r.remote {
                    Some(remote) => format!("{} {} to {remote}", r.protocol, r.local),
                    None => format!("{} {}", r.protocol, r.local),
                })
                .collect();
            out.push(item(
                Section::Disagreements,
                "hidden.libc-view",
                "hidden.libc-view:sockets".into(),
                Severity::High,
                "The socket table Bigman is given leaves sockets out".into(),
                format!(
                    "In the kernel's /proc/net tables, and missing from what the C library loaded into Bigman returns:\n{}",
                    rows.join("\n")
                ),
                vec![
                    Fact::pinned("What differs", "the socket tables in /proc/net"),
                    Fact::shown("Left out", rows.join("; ")),
                ],
            ));
        }
    }
    if let Some(p) = &s.preload {
        if p.kernel != p.libc {
            let show = |v: &Option<String>| match v {
                None => "no such file".to_string(),
                Some(t) if t.trim().is_empty() => "an empty file".to_string(),
                Some(t) => format!("\"{}\"", t.trim()),
            };
            out.push(item(
                Section::Disagreements,
                "hidden.libc-view",
                "hidden.libc-view:preload".into(),
                Severity::High,
                "/etc/ld.so.preload reads differently through the C library".into(),
                format!(
                    "Read directly from the kernel: {}\nRead through the C library loaded into Bigman: {}",
                    show(&p.kernel),
                    show(&p.libc)
                ),
                vec![
                    Fact::pinned("What differs", "/etc/ld.so.preload"),
                    Fact::pinned("From the kernel", show(&p.kernel)),
                    Fact::pinned("Through the C library", show(&p.libc)),
                ],
            ));
        }
    }
}

/// The libraries `/etc/ld.so.preload` names: separated by spaces, colons or
/// newlines, with `#` comments.
pub fn preload_entries(text: &str) -> Vec<String> {
    text.lines()
        .map(|l| l.split('#').next().unwrap_or(""))
        .flat_map(|l| l.split(|c: char| c.is_whitespace() || c == ':'))
        .filter(|e| !e.is_empty())
        .map(str::to_string)
        .collect()
}

fn kernel(s: &Snapshot, out: &mut Vec<RunningItem>) {
    if let Some(text) = s.preload.as_ref().and_then(|p| p.kernel.as_ref()) {
        let entries = preload_entries(text);
        if !entries.is_empty() {
            out.push(item(
                Section::Programs,
                "linker.preload-present",
                "linker.preload-present".into(),
                Severity::High,
                format!(
                    "{} loaded into every program by /etc/ld.so.preload",
                    if entries.len() == 1 {
                        "A library is"
                    } else {
                        "Libraries are"
                    }
                ),
                format!("/etc/ld.so.preload names:\n{}", entries.join("\n")),
                vec![Fact::pinned("Libraries", entries.join(", "))],
            ));
        }
    }
    if let Some(names) = s.bpf.as_ref().filter(|n| !n.is_empty()) {
        let mut sorted = names.clone();
        sorted.sort();
        sorted.dedup();
        out.push(item(
            Section::Kernel,
            "bpf.programs",
            "bpf.programs".into(),
            Severity::Info,
            format!(
                "{} loaded in the kernel",
                plural(sorted.len(), "BPF program name is", "BPF program names are")
            ),
            format!(
                "Names, from the kernel's symbol list (/proc/kallsyms):\n{}",
                sorted.join("\n")
            ),
            vec![Fact::pinned("Names", sorted.join(", "))],
        ));
    }
    let Some(m) = &s.modules else {
        return;
    };
    if let Some(taint) = m.taint.filter(|t| *t != 0) {
        let bits = decode_taint(taint);
        let carried: String = m.tainting.values().flat_map(|v| v.chars()).collect();
        let unexplained: Vec<char> = bits
            .iter()
            .map(|(letter, _)| *letter)
            .filter(|l| MODULE_TAINTS.contains(l) && !carried.contains(*l))
            .collect();
        let mut evidence = format!("/proc/sys/kernel/tainted is {taint}:\n");
        for (letter, meaning) in &bits {
            evidence.push_str(&format!("  {letter}: {meaning}\n"));
        }
        if m.tainting.is_empty() {
            evidence.push_str("No loaded module carries a mark.");
        } else {
            evidence.push_str("Loaded modules that carry marks:");
            for (name, letters) in &m.tainting {
                evidence.push_str(&format!("\n  {name} ({letters})"));
            }
        }
        let letters: String = bits.iter().map(|(l, _)| *l).collect();
        let (severity, title) = if unexplained.is_empty() {
            (
                Severity::Info,
                format!(
                    "The kernel carries marks ({letters}) that loaded modules or events explain"
                ),
            )
        } else {
            evidence.push_str(&format!(
                "\nNo module in the list accounts for {}. The module was unloaded since boot, or it is running and not in the list.",
                unexplained.iter().map(|l| l.to_string()).collect::<Vec<_>>().join(" and ")
            ));
            (
                Severity::Review,
                format!(
                    "The kernel recorded a module load ({}) that no loaded module accounts for",
                    unexplained.iter().collect::<String>()
                ),
            )
        };
        out.push(item(
            Section::Kernel,
            "module.taint",
            "module.taint".into(),
            severity,
            title,
            evidence,
            vec![
                Fact::pinned("Marks", letters),
                Fact::pinned(
                    "Modules carrying marks",
                    if m.tainting.is_empty() {
                        "none".to_string()
                    } else {
                        m.tainting
                            .iter()
                            .map(|(n, l)| format!("{n} ({l})"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    },
                ),
            ],
        ));
    }
    for name in m.no_file.iter().flatten() {
        out.push(item(
            Section::Kernel,
            "module.no-file",
            format!("module.no-file:{name}"),
            Severity::Review,
            format!("Kernel module {name} was not loaded from the kernel's module folder"),
            format!(
                "{name} is in /proc/modules.\nThe running kernel's module index (modules.dep and modules.builtin under /lib/modules) has no file for it."
            ),
            vec![Fact::pinned("Module", name)],
        ));
    }
}

fn programs(s: &Snapshot, out: &mut Vec<RunningItem>) {
    // Rows that share a key are one row with several process IDs.
    struct Group {
        rule: &'static str,
        severity: Severity,
        title: String,
        evidence: String,
        facts: Vec<Fact>,
        pids: Vec<u32>,
    }
    let mut groups: BTreeMap<String, Group> = BTreeMap::new();
    let mut add = |key: String,
                   rule: &'static str,
                   severity: Severity,
                   title: String,
                   evidence: String,
                   facts: Vec<Fact>,
                   pid: u32| {
        groups
            .entry(key)
            .or_insert(Group {
                rule,
                severity,
                title,
                evidence,
                facts,
                pids: vec![],
            })
            .pids
            .push(pid);
    };
    let mut replaced: Vec<String> = vec![];
    let mut preloads: BTreeMap<String, BTreeMap<String, Vec<u32>>> = BTreeMap::new();
    for p in &s.processes {
        if !p.kthread {
            let title_name = p.args.first().and_then(|a| bracketed(a));
            let borrowed = if kernel_thread_name(&p.comm) {
                Some(p.comm.as_str())
            } else {
                title_name.filter(|n| kernel_thread_name(n))
            };
            if let Some(name) = borrowed {
                let from = p
                    .exe
                    .clone()
                    .unwrap_or_else(|| format!("a program run by {}", who(p.uid, s.uid)));
                add(
                    format!("process.fake-kernel-thread:{name}:{}", p.exe.as_deref().unwrap_or("")),
                    "process.fake-kernel-thread",
                    Severity::High,
                    format!("A program is named like the kernel thread {name}"),
                    format!(
                        "Name: {}\nTitle: {}\nThe kernel does not mark it as a kernel thread.\nProgram: {from}",
                        p.comm,
                        p.args.join(" ")
                    ),
                    vec![
                        Fact::pinned("Name", name),
                        Fact::pinned("Program", &from),
                        Fact::pinned("File", p.exe_id.as_deref().unwrap_or("not readable")),
                    ],
                    p.pid,
                );
            }
        }
        if !p.mine {
            continue;
        }
        for lib in &p.preload {
            preloads
                .entry(lib.clone())
                .or_default()
                .entry(p.comm.clone())
                .or_default()
                .push(p.pid);
        }
        if let Some(sock) = p.stdio_socket.as_ref().filter(|_| interpreter(&p.comm)) {
            let remote = sock.remote.as_deref().unwrap_or("an unconnected socket");
            add(
                format!("process.shell-socket:{}:{remote}", p.comm),
                "process.shell-socket",
                if sock.loopback { Severity::Review } else { Severity::High },
                format!("{} reads its commands from a network connection", p.comm),
                format!(
                    "Program: {}\nStandard input and output: one {} socket connected to {remote}{}\nCommand line: {}",
                    program(&p.comm, p.exe.as_deref()),
                    sock.protocol,
                    if sock.loopback { ", on this computer" } else { "" },
                    p.args.join(" ")
                ),
                vec![
                    Fact::pinned("Program", program(&p.comm, p.exe.as_deref())),
                    Fact::pinned("Connected to", remote),
                    Fact::shown("Command line", p.args.join(" ")),
                ],
                p.pid,
            );
        }
        let Some(exe) = p.exe.as_deref() else {
            continue;
        };
        if let Some(name) = exe
            .strip_prefix("/memfd:")
            .map(|n| n.strip_suffix(" (deleted)").unwrap_or(n))
        {
            let (severity, started, note) = match (&p.appimage, name.starts_with("runc_cloned")) {
                (Some(path), _) => (
                    Severity::Info,
                    path.clone(),
                    "Started from an AppImage, whose launcher copies itself into memory before it runs the application.",
                ),
                (None, true) => (
                    Severity::Info,
                    p.comm.clone(),
                    "The container runtime runc copies itself into memory so a container cannot overwrite it.",
                ),
                (None, false) => (
                    Severity::Review,
                    p.args.first().cloned().unwrap_or_else(|| p.comm.clone()),
                    "No AppImage or container runtime accounts for it.",
                ),
            };
            add(
                format!("process.exe-memfd:{name}:{started}"),
                "process.exe-memfd",
                severity,
                format!(
                    "{} runs from memory, with no file on disk",
                    p.appimage.as_deref().map_or(p.comm.as_str(), exe_name)
                ),
                format!(
                    "Process name: {}\nExecutable: memory file \"{name}\"\n{note}\nCommand line: {}",
                    p.comm,
                    p.args.join(" ")
                ),
                vec![
                    Fact::pinned("Memory file", name),
                    Fact::pinned("Started from", &started),
                    Fact::shown("Command line", p.args.join(" ")),
                ],
                p.pid,
            );
            continue;
        }
        if let Some(path) = exe.strip_suffix(" (deleted)") {
            if p.replaced {
                replaced.push(format!("{} ({path}, process {})", p.comm, p.pid));
                continue;
            }
            add(
                format!("process.exe-deleted:{path}"),
                "process.exe-deleted",
                if temp_path(path) {
                    Severity::High
                } else {
                    Severity::Review
                },
                format!("{} is running and its file was deleted", exe_name(path)),
                format!(
                    "{path} started this program and no longer exists.{}\nCommand line: {}",
                    if temp_path(path) {
                        " It ran from a temporary folder."
                    } else {
                        ""
                    },
                    p.args.join(" ")
                ),
                vec![
                    Fact::pinned("Program", path),
                    Fact::pinned("File", p.exe_id.as_deref().unwrap_or("not readable")),
                    Fact::shown("Command line", p.args.join(" ")),
                ],
                p.pid,
            );
            continue;
        }
        if exe.starts_with("/dev/shm/") {
            add(
                format!("process.exe-shm:{exe}"),
                "process.exe-shm",
                Severity::Review,
                format!("{} runs from shared memory (/dev/shm)", exe_name(exe)),
                format!("Program: {exe}\nCommand line: {}", p.args.join(" ")),
                vec![
                    Fact::pinned("Program", exe),
                    Fact::pinned("File", p.exe_id.as_deref().unwrap_or("not readable")),
                ],
                p.pid,
            );
        }
        if DAEMON_NAMES.contains(&p.comm.as_str()) && !system_path(exe) {
            add(
                format!("process.daemon-name:{}:{exe}", p.comm),
                "process.daemon-name",
                Severity::Review,
                format!("A program named {} runs from {exe}", p.comm),
                format!(
                    "Name: {}\nProgram: {exe}\nCommand line: {}",
                    p.comm,
                    p.args.join(" ")
                ),
                vec![
                    Fact::pinned("Name", &p.comm),
                    Fact::pinned("Program", exe),
                    Fact::pinned("File", p.exe_id.as_deref().unwrap_or("not readable")),
                ],
                p.pid,
            );
        }
    }
    for (key, g) in groups {
        let mut facts = g.facts;
        facts.push(Fact::shown(
            if g.pids.len() == 1 {
                "Process ID"
            } else {
                "Process IDs"
            },
            pids(&g.pids),
        ));
        let title = if g.pids.len() > 1 {
            format!("{} ({} processes)", g.title, g.pids.len())
        } else {
            g.title
        };
        let mut row = item(
            Section::Programs,
            g.rule,
            key,
            g.severity,
            title,
            g.evidence,
            facts,
        );
        row.command = command(g.rule, g.pids.first().copied());
        out.push(row);
    }
    if !replaced.is_empty() {
        replaced.sort();
        out.push(item(
            Section::Programs,
            "process.exe-deleted",
            "process.exe-deleted:replaced".into(),
            Severity::Info,
            format!(
                "{} replaced on disk and still run the old version",
                plural(
                    replaced.len(),
                    "running program was",
                    "running programs were"
                )
            ),
            format!("Replaced on disk while running:\n{}", replaced.join("\n")),
            vec![Fact::shown("Programs", replaced.len())],
        ));
        if let Some(row) = out.last_mut() {
            row.command = None;
        }
    }
    // Libraries from system folders are listed together; one from a folder
    // your account can write to gets a row of its own.
    let mut system: Vec<String> = vec![];
    let mut system_users: BTreeMap<String, usize> = BTreeMap::new();
    for (lib, users) in preloads {
        let writable = lib.starts_with('/')
            && (temp_path(&lib)
                || lib.starts_with("/home/")
                || (!s.home.is_empty() && lib.starts_with(&format!("{}/", s.home))));
        let all: Vec<u32> = users.values().flatten().copied().collect();
        if !writable {
            system.push(lib);
            for (comm, ids) in users {
                *system_users.entry(comm).or_default() += ids.len();
            }
            continue;
        }
        let names: Vec<String> = users
            .iter()
            .map(|(comm, ids)| match ids.len() {
                1 => comm.clone(),
                n => format!("{comm} ({n})"),
            })
            .collect();
        out.push(item(
            Section::Programs,
            "process.preload",
            format!("process.preload:{lib}"),
            Severity::Review,
            format!(
                "{} {} with {} loaded first, from a folder you can write to",
                plural(all.len(), "program", "programs"),
                if all.len() == 1 { "runs" } else { "run" },
                exe_name(&lib)
            ),
            format!(
                "LD_PRELOAD or LD_AUDIT in the environment of {} names {lib}.",
                names.join(", "),
            ),
            vec![
                Fact::pinned("Library", &lib),
                Fact::pinned(
                    "File",
                    s.preload_ids
                        .get(&lib)
                        .map_or("not readable", |i| i.as_str()),
                ),
                Fact::shown("Programs", names.join(", ")),
                Fact::shown("Process IDs", pids(&all)),
            ],
        ));
    }
    if !system.is_empty() {
        let users: Vec<String> = system_users
            .iter()
            .map(|(comm, n)| match n {
                1 => comm.clone(),
                n => format!("{comm} ({n})"),
            })
            .collect();
        out.push(item(
            Section::Programs,
            "process.preload",
            "process.preload:system".into(),
            Severity::Info,
            format!(
                "{} loaded first into some of your programs",
                plural(system.len(), "library is", "libraries are")
            ),
            format!(
                "Libraries named by LD_PRELOAD or LD_AUDIT:\n{}\nPrograms: {}",
                system.join("\n"),
                users.join(", ")
            ),
            vec![
                Fact::pinned("Libraries", system.join(", ")),
                Fact::shown("Programs", users.join(", ")),
            ],
        ));
    }
    for b in s.bus_names.iter().flatten() {
        let Some((_, purpose)) = SENSITIVE_BUS_NAMES.iter().find(|(n, _)| *n == b.name) else {
            continue;
        };
        let Some(exe) = b.exe.as_deref().filter(|e| !system_path(e)) else {
            continue;
        };
        out.push(item(
            Section::Programs,
            "bus.name-owner",
            format!("bus.name-owner:{}:{exe}", b.name),
            Severity::Review,
            format!(
                "{} answers for {} on your session bus",
                exe_name(exe),
                b.name
            ),
            format!(
                "Bus name: {} (programs ask it for {purpose})\nOwner: {exe} (process {})",
                b.name, b.pid
            ),
            vec![
                Fact::pinned("Bus name", &b.name),
                Fact::pinned("Owner", exe),
                Fact::shown("Process ID", b.pid),
            ],
        ));
    }
}

/// Who holds a socket, as a short name and as a sentence, and whether every
/// program it could be is installed under a system folder.
fn holder(h: &Option<Holder>, uid: u32, s: &Snapshot) -> (String, String, bool) {
    if let Some(h) = h {
        let path = h.exe.as_deref().unwrap_or(&h.comm);
        return (
            exe_name(path).to_string(),
            format!("{path} (process {})", h.pid),
            h.exe.as_deref().is_some_and(system_path),
        );
    }
    if uid == s.uid && !s.net_capable.is_empty() {
        // Programs started with extra privileges hide their open files, even
        // from the account that runs them; these are the ones that could hold it.
        let names: Vec<&str> = s
            .net_capable
            .iter()
            .map(|c| exe_name(c.exe.as_deref().unwrap_or(&c.comm)))
            .collect();
        let shown: Vec<String> = s
            .net_capable
            .iter()
            .map(|c| {
                format!(
                    "{} (process {})",
                    c.exe.as_deref().unwrap_or(&c.comm),
                    c.pid
                )
            })
            .collect();
        return (
            names.join(" or "),
            format!(
                "one of your programs that hides its open files and has the privilege to open one: {}",
                shown.join(", ")
            ),
            s.net_capable
                .iter()
                .all(|c| c.exe.as_deref().is_some_and(system_path)),
        );
    }
    let owner = match uid {
        0 => "A program run by the administrator".to_string(),
        u if u == s.uid => "A program run by you".to_string(),
        u => format!("A program run by user {u}"),
    };
    (
        owner.clone(),
        format!(
            "{}, which Bigman cannot name without the administrator's view",
            owner.to_lowercase()
        ),
        false,
    )
}

fn network(s: &Snapshot, out: &mut Vec<RunningItem>) {
    struct Held {
        shown: String,
        system: bool,
        uid: u32,
        kinds: Vec<String>,
        ifaces: Vec<String>,
        everything: bool,
    }
    let mut packet: BTreeMap<String, Held> = BTreeMap::new();
    for p in &s.packet_sockets {
        let (name, shown, system) = holder(&p.holder, p.uid, s);
        let e = packet.entry(name).or_insert(Held {
            shown,
            system,
            uid: p.uid,
            kinds: vec![],
            ifaces: vec![],
            everything: false,
        });
        let kind = ethertype(p.protocol);
        if !e.kinds.contains(&kind) {
            e.kinds.push(kind);
        }
        if !e.ifaces.contains(&p.iface) {
            e.ifaces.push(p.iface.clone());
        }
        e.everything |= p.protocol == 0x0003;
    }
    for (name, mut h) in packet {
        h.kinds.sort();
        h.ifaces.sort();
        // A capture socket of yours held by a packaged tool is that tool at work.
        let review = h.everything && !(h.uid == s.uid && h.system);
        out.push(item(
            Section::Network,
            "net.packet-socket",
            format!("net.packet-socket:{name}"),
            if review {
                Severity::Review
            } else {
                Severity::Info
            },
            if h.everything {
                format!("{name} receives a copy of every packet")
            } else {
                format!("{name} reads network frames directly")
            },
            format!(
                "Packet sockets for: {}\nInterfaces: {}\nHeld by: {}",
                h.kinds.join(", "),
                h.ifaces.join(", "),
                h.shown
            ),
            vec![
                Fact::pinned("Held by", &name),
                Fact::pinned("Frames", h.kinds.join(", ")),
                Fact::shown("Interfaces", h.ifaces.join(", ")),
            ],
        ));
    }
    let mut raw: BTreeMap<String, Held> = BTreeMap::new();
    for r in &s.raw_sockets {
        let (name, shown, system) = holder(&r.holder, r.uid, s);
        let e = raw.entry(name).or_insert(Held {
            shown,
            system,
            uid: r.uid,
            kinds: vec![],
            ifaces: vec![],
            everything: false,
        });
        let kind = format!(
            "{} over IPv{}",
            ip_protocol(r.protocol),
            if r.v6 { 6 } else { 4 }
        );
        if !e.kinds.contains(&kind) {
            e.kinds.push(kind);
        }
    }
    for (name, mut h) in raw {
        h.kinds.sort();
        let review = h.uid != 0 && !h.system;
        out.push(item(
            Section::Network,
            "net.raw-socket",
            format!("net.raw-socket:{name}"),
            if review {
                Severity::Review
            } else {
                Severity::Info
            },
            format!("{name} sends and receives raw IP packets"),
            format!(
                "Raw sockets for: {}\nHeld by: {}",
                h.kinds.join(", "),
                h.shown
            ),
            vec![
                Fact::pinned("Held by", &name),
                Fact::pinned("Protocols", h.kinds.join(", ")),
            ],
        ));
    }
}

fn watches(s: &Snapshot, out: &mut Vec<RunningItem>) {
    let mut seen = std::collections::BTreeSet::new();
    for w in &s.watches {
        let name = program(&w.comm, w.exe.as_deref());
        let key = match w.kind {
            WatchKind::Credentials => format!("watch.credentials:{name}:{}", w.target),
            WatchKind::OwnProgram => format!("watch.own-program:{name}"),
        };
        if !seen.insert(key.clone()) {
            continue;
        }
        let facts = vec![
            Fact::pinned("Program", &name),
            Fact::pinned("File", w.exe_id.as_deref().unwrap_or("not readable")),
            Fact::pinned("Watches", &w.target),
            Fact::shown("Process ID", w.pid),
        ];
        match w.kind {
            WatchKind::Credentials => {
                // A program from a system folder is most often the desktop's key
                // agent or a file manager; one from anywhere else is worth reading.
                let system = w.exe.as_deref().is_some_and(system_path);
                out.push(item(
                    Section::Watches,
                    "watch.credentials",
                    key,
                    if system {
                        Severity::Info
                    } else {
                        Severity::Review
                    },
                    format!("{} is told whenever {} changes", exe_name(&name), w.target),
                    format!(
                        "{name} (process {}) holds an inotify watch on {}.",
                        w.pid, w.target
                    ),
                    facts,
                ));
            }
            WatchKind::OwnProgram => out.push(item(
                Section::Watches,
                "watch.own-program",
                key,
                Severity::Review,
                format!("{} watches its own program file", exe_name(&name)),
                format!(
                    "{name} (process {}) holds an inotify watch on the file it runs from, {}.",
                    w.pid, w.target
                ),
                facts,
            )),
        }
    }
}

/// Every row the snapshot supports, most severe first within each section.
pub fn assess(s: &Snapshot) -> Vec<RunningItem> {
    let mut out = vec![];
    disagreements(s, &mut out);
    programs(s, &mut out);
    kernel(s, &mut out);
    network(s, &mut out);
    watches(s, &mut out);
    out
}

/// Sets each row's standing from what you marked. A changed row counts again, at
/// Review or above. A mark on a row where two views disagree, made before such
/// rows stopped being markable, is ignored.
pub fn apply_known(items: &mut [RunningItem], map: &KnownMap) {
    for i in items.iter_mut() {
        let (standing, at, changed) = if i.section.markable() {
            known::standing(map, &i.key, &i.facts)
        } else {
            (Standing::New, None, vec![])
        };
        i.standing = standing;
        i.known_at = at;
        i.changed = changed;
        if standing == Standing::Changed {
            i.severity = i.severity.max(Severity::Review);
        }
    }
    items.sort_by(|a, b| {
        (
            a.standing == Standing::Known,
            a.section,
            std::cmp::Reverse(a.severity),
            &a.key,
        )
            .cmp(&(
                b.standing == Standing::Known,
                b.section,
                std::cmp::Reverse(b.severity),
                &b.key,
            ))
    });
}

/// The report for a set of rows: one finding per row you have not marked, or
/// that changed since you did.
pub fn report(items: &[RunningItem], s: &Snapshot) -> Report {
    let mut r = Report::new(KIND, SUBJECT);
    let mut known = 0;
    for i in items {
        match i.standing {
            Standing::Known => known += 1,
            Standing::New => r.add_command(
                &i.rule,
                i.severity,
                &i.title,
                &i.evidence,
                &i.action,
                i.command.as_deref().unwrap_or(""),
            ),
            Standing::Changed => r.add_command(
                &i.rule,
                i.severity,
                &format!("{} (changed since you marked it as known)", i.title),
                format!("{}\nChanged: {}", i.evidence, i.changed.join("; ")),
                &i.action,
                i.command.as_deref().unwrap_or(""),
            ),
        }
    }
    r.sha256 = Some(crate::store::digest(
        items
            .iter()
            .map(|i| format!("{}\0{}\n", i.key, known::pin(&i.facts)))
            .collect::<String>()
            .as_bytes(),
    ));
    for c in &s.checked {
        r.coverage.checked(c.clone());
    }
    for g in &s.gaps {
        r.coverage.skip(g.what.clone(), g.why, g.detail.clone());
    }
    if known > 0 {
        r.coverage.push(format!(
            "{} you marked as known {} unchanged and not counted.",
            plural(known, "row", "rows"),
            if known == 1 { "is" } else { "are" }
        ));
    }
    r.coverage.push("Program details (files, sockets, watches, environment) are read for your own programs only; other accounts' programs are checked by name.".into());
    r.coverage.push("A kernel module that filters every view at once defeats these comparisons. They find libraries injected into programs and modules that hide from one list; checking the disk from other trusted media is the reliable test.".into());
    r
}

fn artifact(id: &str) -> String {
    format!("artifacts/{id}.running.json")
}

/// Saves the report for a set of rows, with the rows beside it.
pub fn save(store: &Store, items: &[RunningItem], snapshot: &Snapshot) -> Result<Report> {
    let r = report(items, snapshot);
    store.save(&r)?;
    store.write(&artifact(&r.id), &items)?;
    Ok(r)
}

/// Reads the running system, compares it with what you marked, and saves the
/// report with its rows.
pub fn check(store: &Store, stop: &Cancel) -> Result<(Report, Vec<RunningItem>)> {
    let snapshot = gather::gather(stop);
    let mut items = assess(&snapshot);
    apply_known(&mut items, &known::load(store)?);
    let r = save(store, &items, &snapshot)?;
    Ok((r, items))
}

pub fn run(store: &Store, stop: &Cancel) -> Result<Report> {
    Ok(check(store, stop)?.0)
}

/// The rows saved with a report.
pub fn items(store: &Store, report: &str) -> Result<Vec<RunningItem>> {
    let r = store.report(report)?;
    if r.kind != KIND {
        bail!("Choose a Running system report");
    }
    store
        .read(&artifact(report))
        .context("This report's rows are not available")
}

/// Every row with its number, standing, key and details, for `bigman running --rows`.
pub fn rows_text(items: &[RunningItem]) -> String {
    let mut out = String::new();
    let mut section = None;
    for (n, i) in items.iter().enumerate() {
        if section != Some(i.section) && i.standing != Standing::Known {
            section = Some(i.section);
            out.push_str(&format!("\n{}\n", i.section.label()));
        }
        let standing = match i.standing {
            Standing::New => String::new(),
            Standing::Known => "known, ".into(),
            Standing::Changed => "changed since you marked it, ".into(),
        };
        out.push_str(&format!(
            "  {n}. [{standing}{:?}] {}\n     key: {}\n",
            i.severity, i.title, i.key
        ));
        for f in &i.facts {
            out.push_str(&format!(
                "     {}: {}{}\n",
                f.label,
                f.value,
                if f.pinned { " (pinned)" } else { "" }
            ));
        }
    }
    out.push('\n');
    out
}

/// A row's key from the CLI: its number in the report's list, or the key itself.
pub fn key_for(store: &Store, report: &str, item: &str) -> Result<String> {
    let all = items(store, report)?;
    if let Ok(n) = item.parse::<usize>() {
        return all
            .get(n)
            .map(|i| i.key.clone())
            .with_context(|| format!("The report lists {}", plural(all.len(), "row", "rows")));
    }
    all.iter()
        .find(|i| i.key == item)
        .map(|i| i.key.clone())
        .context("That row is not in this report")
}

/// The row a report showed, checked against the system as it is now: marking
/// records what you read, so it is refused once that is no longer what is there.
fn current(store: &Store, report: &str, key: &str, stop: &Cancel) -> Result<RunningItem> {
    let shown = items(store, report)?
        .into_iter()
        .find(|i| i.key == key)
        .context("That row is not in this report")?;
    if !shown.section.markable() {
        bail!("{NOT_MARKABLE}");
    }
    let now = assess(&gather::gather(stop))
        .into_iter()
        .find(|i| i.key == key)
        .with_context(|| format!("\"{}\" is no longer there; check again", shown.title))?;
    if known::pin(&now.facts) != known::pin(&shown.facts) {
        bail!(
            "\"{}\" changed after this report; check again and read the new details before marking it",
            shown.title
        );
    }
    Ok(shown)
}

/// What marking shows before anything is written: the summary, what the mark
/// records, and a note for rows that are rated Review or High.
pub fn mark_plan(
    store: &Store,
    report: &str,
    key: &str,
) -> Result<(String, String, Option<String>)> {
    let shown = items(store, report)?
        .into_iter()
        .find(|i| i.key == key)
        .context("That row is not in this report")?;
    if !shown.section.markable() {
        bail!("{NOT_MARKABLE}");
    }
    let summary = format!(
        "Record \"{}\" as something you recognise, as it is now. It stays in the list and stops counting as something to review. If one of the details below changes, it counts again and says what changed.",
        shown.title
    );
    let details: String = shown
        .facts
        .iter()
        .filter(|f| f.pinned)
        .map(|f| format!("{}: {}\n", f.label, f.value))
        .collect();
    let note = (shown.severity != Severity::Info).then(|| {
        "Marking does not change its rating or make it harmless. It records that you recognise it.".to_string()
    });
    Ok((summary, details, note))
}

pub fn mark(store: &Store, report: &str, key: &str, stop: &Cancel) -> Result<String> {
    let shown = current(store, report, key, stop)?;
    known::mark(store, &shown.key, &shown.title, &shown.facts)?;
    Ok(format!("Marked as known: {}", shown.title))
}

pub fn forget(store: &Store, key: &str) -> Result<String> {
    let (title, restored) = known::forget(store, key)?;
    Ok(if restored {
        format!("The earlier mark is back for: {title}")
    } else {
        format!("No longer marked as known: {title}")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(json: serde_json::Value) -> Snapshot {
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn taint_is_decoded_and_explained_by_the_modules_that_carry_it() {
        assert_eq!(
            decode_taint(4096 | 8192)
                .iter()
                .map(|(l, _)| *l)
                .collect::<String>(),
            "OE"
        );
        let explained = snapshot(serde_json::json!({
            "modules": {"taint": 12289, "tainting": {"nvidia": "POE"}}
        }));
        let items = assess(&explained);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].severity, Severity::Info);
        let unexplained = snapshot(serde_json::json!({
            "modules": {"taint": 12288, "tainting": {}}
        }));
        let items = assess(&unexplained);
        assert_eq!(items[0].severity, Severity::Review);
        assert!(items[0]
            .evidence
            .contains("No module in the list accounts for O and E"));
        // A warning or a userspace-requested mark is not a module's doing.
        let other = snapshot(serde_json::json!({"modules": {"taint": 576}}));
        assert_eq!(assess(&other)[0].severity, Severity::Info);
    }

    #[test]
    fn preload_file_entries_ignore_comments_and_separators() {
        assert_eq!(
            preload_entries("# managed\n/usr/lib/a.so:/usr/lib/b.so  /opt/c.so # why\n\n"),
            vec!["/usr/lib/a.so", "/usr/lib/b.so", "/opt/c.so"]
        );
        assert!(preload_entries("\n# nothing\n").is_empty());
    }

    #[test]
    fn processes_with_one_cause_are_one_row() {
        let s = snapshot(serde_json::json!({
            "uid": 1000,
            "processes": [
                {"pid": 10, "uid": 1000, "mine": true, "comm": "firefox", "exe": "/usr/lib64/firefox/firefox", "preload": ["libmozsandbox.so"]},
                {"pid": 11, "uid": 1000, "mine": true, "comm": "firefox", "exe": "/usr/lib64/firefox/firefox", "preload": ["libmozsandbox.so"]},
                {"pid": 12, "uid": 1000, "mine": true, "comm": "bigmand", "exe": "/home/u/.local/bin/bigmand (deleted)", "replaced": true},
                {"pid": 13, "uid": 1000, "mine": true, "comm": "bash", "exe": "/usr/bin/bash (deleted)", "replaced": true}
            ]
        }));
        let items = assess(&s);
        assert_eq!(items.len(), 2, "{items:#?}");
        let preload = items.iter().find(|i| i.rule == "process.preload").unwrap();
        assert_eq!(preload.severity, Severity::Info);
        assert!(preload.title.starts_with("1 library is loaded first"));
        assert!(preload.evidence.ends_with("Programs: firefox (2)"));
        let replaced = items
            .iter()
            .find(|i| i.rule == "process.exe-deleted")
            .unwrap();
        assert_eq!(replaced.severity, Severity::Info);
        assert!(replaced
            .title
            .starts_with("2 running programs were replaced"));
    }

    #[test]
    fn a_kernel_thread_name_on_an_ordinary_program_is_reported_for_any_user() {
        let s = snapshot(serde_json::json!({
            "uid": 1000,
            "processes": [
                {"pid": 2, "uid": 0, "comm": "kthreadd", "kthread": true},
                {"pid": 40, "uid": 0, "comm": "kworker/0:1", "kthread": true},
                {"pid": 900, "uid": 0, "comm": "kworker/0:9", "args": ["[kworker/0:9]"]},
                {"pid": 901, "uid": 1000, "mine": true, "comm": "python3", "args": ["[celeryd: worker]"], "exe": "/usr/bin/python3.13"}
            ]
        }));
        let items = assess(&s);
        assert_eq!(items.len(), 1, "{items:#?}");
        assert_eq!(items[0].rule, "process.fake-kernel-thread");
        assert!(items[0].evidence.contains("the administrator (root)"));
    }

    #[test]
    fn a_known_row_stops_counting_until_a_pinned_fact_changes() {
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        let before = snapshot(serde_json::json!({
            "uid": 1000,
            "packet_sockets": [{"protocol": 3, "iface": "wlan0", "uid": 1000, "holder": {"pid": 5, "comm": "capture", "exe": "/home/u/bin/capture"}}]
        }));
        let mut items = assess(&before);
        apply_known(&mut items, &known::load(&store).unwrap());
        assert_eq!(report(&items, &before).status(), "Findings to review");
        known::mark(&store, &items[0].key, &items[0].title, &items[0].facts).unwrap();

        // Another interface and process ID: the same thing.
        let same = snapshot(serde_json::json!({
            "uid": 1000,
            "packet_sockets": [{"protocol": 3, "iface": "eth0", "uid": 1000, "holder": {"pid": 77, "comm": "capture", "exe": "/home/u/bin/capture"}}]
        }));
        let mut items = assess(&same);
        apply_known(&mut items, &known::load(&store).unwrap());
        assert_eq!(items[0].standing, Standing::Known);
        let r = report(&items, &same);
        assert_eq!(r.status(), "No findings from these checks");
        assert!(r.coverage.notes[0].starts_with("1 row you marked as known is unchanged"));

        // The BPF program list is one row; a new name changes it.
        let bpf = |names: &[&str]| snapshot(serde_json::json!({"bpf": names}));
        let mut items = assess(&bpf(&["sd_devices", "sd_fw_egress"]));
        assert_eq!(items[0].severity, Severity::Info);
        known::mark(&store, &items[0].key, &items[0].title, &items[0].facts).unwrap();
        items = assess(&bpf(&["sd_devices", "sd_fw_egress", "hide_fixture"]));
        apply_known(&mut items, &known::load(&store).unwrap());
        assert_eq!(items[0].standing, Standing::Changed);
        assert_eq!(items[0].severity, Severity::Review);
        assert!(items[0].changed[0].contains("hide_fixture"));
        let r = report(&items, &bpf(&[]));
        assert!(r.findings[0]
            .summary
            .ends_with("(changed since you marked it as known)"));
    }

    #[test]
    fn a_row_where_two_views_disagree_cannot_be_marked_and_an_old_mark_is_ignored() {
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        let hidden = snapshot(serde_json::json!({
            "uid": 1000,
            "pids": {"listed": 10, "probed": 11, "unlisted": [{"pid": 4242, "name": "worker", "uid": 1000}]}
        }));
        let mut items = assess(&hidden);
        assert_eq!(items[0].section, Section::Disagreements);
        // A mark written before the rule, straight into the store.
        known::mark(&store, &items[0].key, &items[0].title, &items[0].facts).unwrap();
        apply_known(&mut items, &known::load(&store).unwrap());
        assert_eq!(items[0].standing, Standing::New);
        assert_eq!(items[0].known_at, None);
        let r = save(&store, &items, &hidden).unwrap();
        assert_eq!(r.status(), "Findings to review");
        let refused = mark_plan(&store, &r.id, &items[0].key).unwrap_err();
        assert!(refused
            .to_string()
            .starts_with("Rows where two views disagree"));
        let refused = mark(&store, &r.id, &items[0].key, &crate::runner::cancel()).unwrap_err();
        assert!(refused
            .to_string()
            .starts_with("Rows where two views disagree"));
    }
}
