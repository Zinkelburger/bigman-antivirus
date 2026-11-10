//! Reads the running system for `assess`, without root. Each comparison that
//! finds a difference is read a second time, and only what differs both times is
//! kept, so a process or socket that appeared between two reads is not reported.
use super::{
    rawsys, BusName, Holder, ModuleMismatch, ModuleViews, PacketSocket, PidViews, Preload, Proc,
    RawSocket, Snapshot, SocketRow, SocketViews, StdioSocket, Unlisted, Watch, WatchKind,
};
use crate::{
    model::{SkipReason, Skipped},
    platform::{linux::parse_proc_net, Protocol},
    runner::{self, Cancel},
};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    ffi::CString,
    fs, io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    os::unix::{ffi::OsStrExt, fs::MetadataExt},
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::Ordering,
    time::Duration,
};

/// The kernel's upper bound on process IDs.
const PID_LIMIT: u32 = 4_194_304;
/// `PF_KTHREAD` in the flags field of /proc/PID/stat.
const PF_KTHREAD: u64 = 0x0020_0000;
const KALLSYMS_MAX: usize = 256 * 1024 * 1024;

/// What differs in a comparison on two passes in a row. The second pass runs
/// only when the first found something.
pub fn twice<T: Ord + Clone>(mut pass: impl FnMut() -> BTreeSet<T>) -> Vec<T> {
    let first = pass();
    if first.is_empty() {
        return vec![];
    }
    std::thread::sleep(Duration::from_millis(150));
    let second = pass();
    first.intersection(&second).cloned().collect()
}

fn gap(what: &str, why: SkipReason, detail: impl ToString) -> Skipped {
    Skipped {
        what: what.into(),
        why,
        detail: detail.to_string(),
    }
}

fn numeric(names: impl IntoIterator<Item = Vec<u8>>) -> BTreeSet<u32> {
    names
        .into_iter()
        .filter_map(|n| std::str::from_utf8(&n).ok()?.parse().ok())
        .collect()
}

fn pids_kernel() -> io::Result<BTreeSet<u32>> {
    Ok(numeric(rawsys::list_dir(Path::new("/proc"))?))
}

fn pids_libc() -> io::Result<BTreeSet<u32>> {
    Ok(numeric(
        fs::read_dir("/proc")?
            .flatten()
            .map(|e| e.file_name().as_bytes().to_vec()),
    ))
}

/// Every process and thread ID /proc lists.
fn tasks_kernel() -> io::Result<BTreeSet<u32>> {
    let pids = pids_kernel()?;
    let mut all = pids.clone();
    for pid in pids {
        if let Ok(names) = rawsys::list_dir(Path::new(&format!("/proc/{pid}/task"))) {
            all.extend(numeric(names));
        }
    }
    Ok(all)
}

/// Whether /proc hides other users' processes, from a mountinfo table.
pub fn hides_pids(mountinfo: &str) -> bool {
    mountinfo.lines().any(|line| {
        let Some((mount, fs)) = line.split_once(" - ") else {
            return false;
        };
        mount.split_whitespace().nth(4) == Some("/proc")
            && fs.starts_with("proc ")
            && fs
                .split([' ', ','])
                .filter_map(|o| o.strip_prefix("hidepid="))
                .any(|v| v != "0" && v != "off")
    })
}

fn unlisted(pid: u32) -> Unlisted {
    Unlisted {
        pid,
        name: fs::read_to_string(format!("/proc/{pid}/comm"))
            .ok()
            .map(|c| c.trim().to_string()),
        uid: fs::metadata(format!("/proc/{pid}")).ok().map(|m| m.uid()),
    }
}

fn pid_views(
    gaps: &mut Vec<Skipped>,
    checked: &mut Vec<String>,
    stop: &Cancel,
) -> Option<PidViews> {
    let listed = match tasks_kernel() {
        Ok(l) => l,
        Err(e) => {
            gaps.push(gap(
                "The process list",
                SkipReason::Unreadable,
                format!("/proc: {e}"),
            ));
            return None;
        }
    };
    let mut views = PidViews {
        listed: listed.len() as u32,
        ..Default::default()
    };
    if rawsys::DIRECT {
        views.libc_missing = twice(|| {
            let (Ok(kernel), Ok(libc)) = (pids_kernel(), pids_libc()) else {
                return BTreeSet::new();
            };
            kernel.difference(&libc).copied().collect()
        })
        .into_iter()
        .map(unlisted)
        .collect();
    }
    let hidden_by_mount = fs::read_to_string("/proc/self/mountinfo").is_ok_and(|m| hides_pids(&m));
    if hidden_by_mount {
        gaps.push(gap(
            "Processes missing from the process list",
            SkipReason::NotApplicable,
            "/proc is mounted with hidepid, which hides other accounts' processes from you on purpose, so a signal probe would report all of them.",
        ));
        return Some(views);
    }
    let max = fs::read_to_string("/proc/sys/kernel/pid_max")
        .ok()
        .and_then(|t| t.trim().parse::<u32>().ok())
        .unwrap_or(PID_LIMIT)
        .min(PID_LIMIT);
    let mut alive = BTreeSet::new();
    for pid in 1..=max {
        if pid % 65_536 == 0 && stop.load(Ordering::Relaxed) {
            gaps.push(gap(
                "Processes missing from the process list",
                SkipReason::Cancelled,
                "The check was stopped before every process ID was probed.",
            ));
            return Some(views);
        }
        if rawsys::exists(pid) {
            alive.insert(pid);
        }
    }
    views.probed = Some(alive.len() as u32);
    // A process that started after the listing was read is in the next one.
    let mut candidates: BTreeSet<u32> = alive.difference(&listed).copied().collect();
    if !candidates.is_empty() {
        for _ in 0..2 {
            std::thread::sleep(Duration::from_millis(150));
            candidates.retain(|pid| rawsys::exists(*pid));
            let now = tasks_kernel().unwrap_or_default();
            if now.is_empty() {
                candidates.clear();
            }
            candidates.retain(|pid| !now.contains(pid));
        }
    }
    views.unlisted = candidates.into_iter().map(unlisted).collect();
    checked.push("Process list: /proc against a signal probe of every process ID".into());
    Some(views)
}

/// One line of /proc/modules: name, whether it finished loading, and its taint letters.
pub fn parse_modules(text: &str) -> Vec<(String, bool, String)> {
    text.lines()
        .filter_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let name = fields.first()?;
            let live = fields.get(4).is_none_or(|s| *s == "Live");
            let taints = fields
                .iter()
                .skip(5)
                .find_map(|f| f.strip_prefix('(')?.strip_suffix(')'))
                .unwrap_or("");
            Some((name.to_string(), live, taints.to_string()))
        })
        .collect()
}

/// Module names and BPF program names in a /proc/kallsyms table. A symbol's
/// module is the bracketed word that ends its line.
pub fn parse_kallsyms(text: &str) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut modules = BTreeSet::new();
    let mut bpf = BTreeSet::new();
    for line in text.lines() {
        let Some(tag) = line
            .strip_suffix(']')
            .and_then(|l| l.rsplit_once('['))
            .map(|(_, tag)| tag)
        else {
            continue;
        };
        if tag == "bpf" {
            // "bpf_prog_<16 hex digits>_<name>"; a program with no name has no suffix.
            let symbol = line.split_whitespace().nth(2).unwrap_or("");
            if let Some(rest) = symbol.strip_prefix("bpf_prog_") {
                let name = rest.get(17..).unwrap_or("");
                bpf.insert(if name.is_empty() {
                    "(unnamed)".to_string()
                } else {
                    name.to_string()
                });
            }
        } else if !tag.starts_with("__builtin__") && !modules.contains(tag) {
            modules.insert(tag.to_string());
        }
    }
    (modules, bpf)
}

/// Module names with a file under a kernel's module folder, from its
/// `modules.dep` and `modules.builtin` indexes.
pub fn module_files(dep: &str, builtin: &str) -> BTreeSet<String> {
    dep.lines()
        .filter_map(|l| l.split(':').next())
        .chain(builtin.lines())
        .filter_map(|path| {
            let file = path.trim().rsplit('/').next()?;
            let name = file.split(".ko").next()?;
            (!name.is_empty()).then(|| name.replace('-', "_"))
        })
        .collect()
}

fn sysfs_modules() -> io::Result<BTreeSet<String>> {
    Ok(rawsys::list_dir(Path::new("/sys/module"))?
        .into_iter()
        .filter_map(|n| String::from_utf8(n).ok())
        // Built-in code with parameters has a folder too; a loadable module has `initstate`.
        .filter(|n| Path::new(&format!("/sys/module/{n}/initstate")).exists())
        .collect())
}

fn kernel_text(path: &str, max: usize) -> io::Result<String> {
    Ok(String::from_utf8_lossy(&rawsys::read_file(Path::new(path), max)?).into_owned())
}

fn module_views(
    gaps: &mut Vec<Skipped>,
    checked: &mut Vec<String>,
) -> (Option<ModuleViews>, Option<Vec<String>>) {
    let listed = match kernel_text("/proc/modules", 16 * 1024 * 1024) {
        Ok(t) => parse_modules(&t),
        Err(e) => {
            gaps.push(gap(
                "Kernel modules",
                SkipReason::Unreadable,
                format!("/proc/modules: {e}"),
            ));
            return (None, None);
        }
    };
    let mut views = ModuleViews {
        listed: listed.len() as u32,
        tainting: listed
            .iter()
            .filter(|(_, _, taints)| !taints.is_empty())
            .map(|(name, _, taints)| (name.clone(), taints.clone()))
            .collect(),
        taint: fs::read_to_string("/proc/sys/kernel/tainted")
            .ok()
            .and_then(|t| t.trim().parse().ok()),
        ..Default::default()
    };
    let names = || -> BTreeSet<String> {
        kernel_text("/proc/modules", 16 * 1024 * 1024)
            .map(|t| parse_modules(&t).into_iter().map(|(n, _, _)| n).collect())
            .unwrap_or_default()
    };
    let live = || -> BTreeSet<String> {
        kernel_text("/proc/modules", 16 * 1024 * 1024)
            .map(|t| {
                parse_modules(&t)
                    .into_iter()
                    .filter(|(_, live, _)| *live)
                    .map(|(n, _, _)| n)
                    .collect()
            })
            .unwrap_or_default()
    };
    let mut mismatched: BTreeMap<String, ModuleMismatch> = BTreeMap::new();
    let mut note = |name: String, seen: &str, missing: &str| {
        let m = mismatched.entry(name.clone()).or_insert(ModuleMismatch {
            name,
            ..Default::default()
        });
        if !m.seen_in.iter().any(|s| s == seen) {
            m.seen_in.push(seen.into());
        }
        if !m.missing_from.iter().any(|s| s == missing) {
            m.missing_from.push(missing.into());
        }
    };
    let sysfs_ok = sysfs_modules().is_ok();
    if sysfs_ok {
        for name in twice(|| {
            let listed = names();
            if listed.is_empty() {
                return BTreeSet::new();
            }
            sysfs_modules()
                .unwrap_or_default()
                .difference(&listed)
                .cloned()
                .collect()
        }) {
            note(name, "/sys/module", "/proc/modules");
        }
        for name in twice(|| {
            let Ok(sysfs) = sysfs_modules() else {
                return BTreeSet::new();
            };
            live().difference(&sysfs).cloned().collect()
        }) {
            note(name, "/proc/modules", "/sys/module");
        }
    } else {
        gaps.push(gap(
            "Kernel modules in /sys/module",
            SkipReason::Unreadable,
            "/sys/module could not be listed.",
        ));
    }
    // The symbol table is large; it is read once for modules and BPF names, and
    // again only when it names a module the list does not.
    let mut bpf = None;
    match kernel_text("/proc/kallsyms", KALLSYMS_MAX) {
        Ok(text) => {
            let (symbol_modules, programs) = parse_kallsyms(&text);
            bpf = Some(programs.into_iter().collect());
            let first: BTreeSet<String> = symbol_modules.difference(&names()).cloned().collect();
            if !first.is_empty() {
                std::thread::sleep(Duration::from_millis(150));
                let again = kernel_text("/proc/kallsyms", KALLSYMS_MAX)
                    .map(|t| parse_kallsyms(&t).0)
                    .unwrap_or_default();
                let listed = names();
                for name in first {
                    if again.contains(&name) && !listed.contains(&name) && !listed.is_empty() {
                        note(
                            name,
                            "the kernel's symbol list (/proc/kallsyms)",
                            "/proc/modules",
                        );
                    }
                }
            }
            checked.push(
                "Kernel modules: /proc/modules against /sys/module and the kernel's symbol list"
                    .into(),
            );
        }
        Err(e) => {
            gaps.push(gap(
                "Kernel modules in the symbol list, and BPF program names",
                SkipReason::Unreadable,
                format!("/proc/kallsyms: {e}"),
            ));
            if sysfs_ok {
                checked.push("Kernel modules: /proc/modules against /sys/module".into());
            }
        }
    }
    views.mismatched = mismatched.into_values().collect();
    if rawsys::DIRECT {
        views.libc_missing = twice(|| {
            let Ok(libc) = fs::read_to_string("/proc/modules") else {
                return BTreeSet::new();
            };
            let libc: BTreeSet<String> = parse_modules(&libc)
                .into_iter()
                .map(|(n, _, _)| n)
                .collect();
            names().difference(&libc).cloned().collect()
        });
    }
    let release = fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|r| r.trim().to_string())
        .unwrap_or_default();
    let folder = format!("/lib/modules/{release}");
    match fs::read_to_string(format!("{folder}/modules.dep")) {
        Ok(dep) => {
            let builtin = fs::read_to_string(format!("{folder}/modules.builtin")).unwrap_or_default();
            let files = module_files(&dep, &builtin);
            views.no_file = Some(
                listed
                    .iter()
                    .filter(|(name, live, _)| *live && !files.contains(name))
                    .map(|(name, _, _)| name.clone())
                    .collect(),
            );
            checked.push("Kernel modules: each loaded module has a file in the running kernel's module folder".into());
        }
        Err(e) => gaps.push(gap(
            "Whether each loaded module came from the kernel's module folder",
            SkipReason::Unreadable,
            format!("{folder}/modules.dep: {e}. The folder is removed when an update replaces the running kernel; it is read again after a restart."),
        )),
    }
    (Some(views), bpf)
}

// --- Sockets ------------------------------------------------------------------------------

const NETLINK_SOCK_DIAG: i32 = 4;
const SOCK_DIAG_BY_FAMILY: u16 = 20;
const NLM_F_REQUEST: u16 = 0x01;
const NLM_F_DUMP: u16 = 0x300;
const NLMSG_ERROR: u16 = 2;
const NLMSG_DONE: u16 = 3;
/// `struct inet_diag_msg` and the netlink header before it.
const DIAG_MSG: usize = 72;
const NL_HEADER: usize = 16;
const TCP_ESTABLISHED: u32 = 1;
const TCP_LISTEN: u32 = 10;

fn row(protocol: &str, local: SocketAddr, remote: SocketAddr, uid: u32) -> SocketRow {
    SocketRow {
        protocol: protocol.into(),
        local: local.to_string(),
        remote: (remote.port() != 0).then(|| remote.to_string()),
        uid,
    }
}

/// The sockets in one socket-diagnostics answer (`struct inet_diag_msg` records).
pub fn parse_diag(buffer: &[u8], protocol: &str, out: &mut Vec<SocketRow>) -> io::Result<bool> {
    let mut at = 0;
    while at + NL_HEADER <= buffer.len() {
        let len = u32::from_ne_bytes(buffer[at..at + 4].try_into().unwrap()) as usize;
        let kind = u16::from_ne_bytes(buffer[at + 4..at + 6].try_into().unwrap());
        if len < NL_HEADER || at + len > buffer.len() {
            break;
        }
        match kind {
            NLMSG_DONE => return Ok(true),
            NLMSG_ERROR => {
                let code = i32::from_ne_bytes(
                    buffer
                        .get(at + NL_HEADER..at + NL_HEADER + 4)
                        .and_then(|b| b.try_into().ok())
                        .unwrap_or([0; 4]),
                );
                return Err(io::Error::from_raw_os_error(-code));
            }
            SOCK_DIAG_BY_FAMILY if len >= NL_HEADER + DIAG_MSG => {
                let m = &buffer[at + NL_HEADER..at + NL_HEADER + DIAG_MSG];
                let port = |o: usize| u16::from_be_bytes([m[o], m[o + 1]]);
                let addr = |o: usize| -> IpAddr {
                    if m[0] == libc::AF_INET as u8 {
                        IpAddr::V4(Ipv4Addr::new(m[o], m[o + 1], m[o + 2], m[o + 3]))
                    } else {
                        let bytes: [u8; 16] = m[o..o + 16].try_into().unwrap();
                        IpAddr::V6(Ipv6Addr::from(bytes))
                    }
                };
                // family, state, timer, retrans; then sport, dport, src[16], dst[16],
                // interface, cookie[8]; then expires, rqueue, wqueue, uid, inode.
                out.push(row(
                    protocol,
                    SocketAddr::new(addr(8), port(4)),
                    SocketAddr::new(addr(24), port(6)),
                    u32::from_ne_bytes(m[64..68].try_into().unwrap()),
                ));
            }
            _ => {}
        }
        at += (len + 3) & !3;
    }
    Ok(false)
}

/// Asks the kernel's socket diagnostics (the interface `ss` uses) for one
/// family and protocol.
fn diag_dump(family: i32, protocol: i32, states: u32, name: &str) -> io::Result<Vec<SocketRow>> {
    let fd = unsafe {
        libc::socket(
            libc::AF_NETLINK,
            libc::SOCK_DGRAM | libc::SOCK_CLOEXEC,
            NETLINK_SOCK_DIAG,
        )
    };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    struct Close(i32);
    impl Drop for Close {
        fn drop(&mut self) {
            unsafe { libc::close(self.0) };
        }
    }
    let _close = Close(fd);
    let timeout = libc::timeval {
        tv_sec: 5,
        tv_usec: 0,
    };
    unsafe {
        libc::setsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_RCVTIMEO,
            &timeout as *const _ as *const libc::c_void,
            std::mem::size_of::<libc::timeval>() as u32,
        );
    }
    // struct nlmsghdr, then struct inet_diag_req_v2 (8 bytes and a 48-byte socket id).
    let mut request = [0u8; NL_HEADER + 56];
    let len = request.len() as u32;
    request[0..4].copy_from_slice(&len.to_ne_bytes());
    request[4..6].copy_from_slice(&SOCK_DIAG_BY_FAMILY.to_ne_bytes());
    request[6..8].copy_from_slice(&(NLM_F_REQUEST | NLM_F_DUMP).to_ne_bytes());
    request[8..12].copy_from_slice(&1u32.to_ne_bytes());
    request[16] = family as u8;
    request[17] = protocol as u8;
    request[20..24].copy_from_slice(&states.to_ne_bytes());
    let sent = unsafe {
        libc::send(
            fd,
            request.as_ptr() as *const libc::c_void,
            request.len(),
            0,
        )
    };
    if sent < 0 {
        return Err(io::Error::last_os_error());
    }
    let mut out = vec![];
    let mut buffer = vec![0u8; 256 * 1024];
    loop {
        let n = unsafe {
            libc::recv(
                fd,
                buffer.as_mut_ptr() as *mut libc::c_void,
                buffer.len(),
                0,
            )
        };
        if n < 0 {
            return Err(io::Error::last_os_error());
        }
        if n == 0 || parse_diag(&buffer[..n as usize], name, &mut out)? {
            return Ok(out);
        }
    }
}

type Rows = BTreeMap<(String, String, Option<String>), SocketRow>;

fn keyed(rows: Vec<SocketRow>) -> Rows {
    rows.into_iter()
        .map(|r| ((r.protocol.clone(), r.local.clone(), r.remote.clone()), r))
        .collect()
}

fn diag_rows() -> io::Result<Rows> {
    let states = (1 << TCP_ESTABLISHED) | (1 << TCP_LISTEN);
    let mut all = vec![];
    for family in [libc::AF_INET, libc::AF_INET6] {
        all.extend(diag_dump(family, libc::IPPROTO_TCP, states, "tcp")?);
        all.extend(diag_dump(family, libc::IPPROTO_UDP, u32::MAX, "udp")?);
    }
    Ok(keyed(all))
}

/// The sockets in /proc/net tables that both views are expected to hold:
/// listening and established TCP, and every UDP socket.
pub fn procfs_rows(tables: &[(&str, String)]) -> Vec<SocketRow> {
    let mut out = vec![];
    for (file, text) in tables {
        let (protocol, name) = if file.starts_with("tcp") {
            (Protocol::Tcp, "tcp")
        } else {
            (Protocol::Udp, "udp")
        };
        for (s, _) in parse_proc_net(text, protocol) {
            if protocol == Protocol::Tcp && s.state != "listen" && s.state != "established" {
                continue;
            }
            let remote = s
                .remote
                .unwrap_or_else(|| SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0));
            out.push(row(name, s.local, remote, s.uid));
        }
    }
    out
}

const NET_TABLES: [&str; 4] = ["tcp", "tcp6", "udp", "udp6"];

fn net_tables(read: impl Fn(&str) -> io::Result<String>) -> io::Result<Rows> {
    let mut tables = vec![];
    for file in NET_TABLES {
        match read(&format!("/proc/net/{file}")) {
            Ok(text) => tables.push((file, text)),
            // No IPv6 on this computer.
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    Ok(keyed(procfs_rows(&tables)))
}

fn kernel_net() -> io::Result<Rows> {
    net_tables(|p| kernel_text(p, 64 * 1024 * 1024))
}

fn socket_views(gaps: &mut Vec<Skipped>, checked: &mut Vec<String>) -> Option<SocketViews> {
    let procfs = match kernel_net() {
        Ok(p) => p,
        Err(e) => {
            gaps.push(gap(
                "Socket tables",
                SkipReason::Unreadable,
                format!("/proc/net: {e}"),
            ));
            return None;
        }
    };
    let mut views = SocketViews {
        compared: procfs.len() as u32,
        ..Default::default()
    };
    let only = |a: &Rows, b: &Rows| -> BTreeSet<SocketRow> {
        a.iter()
            .filter(|(k, _)| !b.contains_key(*k))
            .map(|(_, r)| r.clone())
            .collect()
    };
    match diag_rows() {
        Ok(_) => {
            views.only_diag = twice(|| match (diag_rows(), kernel_net()) {
                (Ok(d), Ok(p)) => only(&d, &p),
                _ => BTreeSet::new(),
            });
            views.only_procfs = twice(|| match (kernel_net(), diag_rows()) {
                (Ok(p), Ok(d)) => only(&p, &d),
                _ => BTreeSet::new(),
            });
            checked.push("Sockets: /proc/net against the kernel's socket diagnostics".into());
        }
        Err(e) => gaps.push(gap(
            "Sockets in the kernel's socket diagnostics",
            SkipReason::Unreadable,
            format!("sock_diag: {e}"),
        )),
    }
    if rawsys::DIRECT {
        views.libc_missing =
            twice(
                || match (kernel_net(), net_tables(|p| fs::read_to_string(p))) {
                    (Ok(k), Ok(l)) => only(&k, &l),
                    _ => BTreeSet::new(),
                },
            );
    }
    Some(views)
}

fn preload(gaps: &mut Vec<Skipped>) -> Option<Preload> {
    let path = Path::new("/etc/ld.so.preload");
    let text = |r: io::Result<Vec<u8>>| -> io::Result<Option<String>> {
        match r {
            Ok(bytes) => Ok(Some(String::from_utf8_lossy(&bytes).into_owned())),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    };
    let kernel = match text(rawsys::read_file(path, 1024 * 1024)) {
        Ok(k) => k,
        Err(e) => {
            gaps.push(gap("/etc/ld.so.preload", SkipReason::Unreadable, e));
            return None;
        }
    };
    // Equal when the C library cannot be read around, so nothing is reported.
    let libc = text(fs::read(path)).unwrap_or_else(|_| kernel.clone());
    Some(Preload { kernel, libc })
}

// --- Your own programs --------------------------------------------------------------------

/// The parent, and the flags field, out of /proc/PID/stat. The name may hold
/// spaces and parentheses, so fields are counted after the last ')'.
pub fn parse_stat_flags(stat: &str) -> Option<u64> {
    stat[stat.rfind(')')? + 1..]
        .split_whitespace()
        .nth(6)?
        .parse()
        .ok()
}

/// A file's handle as the kernel encodes it, which /proc/PID/fdinfo prints for
/// each inotify watch. Equal handles are the same file.
fn handle(path: &Path) -> Option<(i32, String)> {
    #[repr(C)]
    struct Handle {
        bytes: libc::c_uint,
        kind: libc::c_int,
        data: [u8; 128],
    }
    let c = CString::new(path.as_os_str().as_bytes()).ok()?;
    let mut h = Handle {
        bytes: 128,
        kind: 0,
        data: [0; 128],
    };
    let mut mount = 0;
    let ret = unsafe {
        libc::name_to_handle_at(
            libc::AT_FDCWD,
            c.as_ptr(),
            &mut h as *mut Handle as *mut libc::file_handle,
            &mut mount,
            libc::AT_SYMLINK_FOLLOW,
        )
    };
    (ret == 0).then(|| {
        let hex: String = h.data[..h.bytes as usize]
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        (h.kind, hex)
    })
}

/// The file handles an inotify descriptor watches, from its fdinfo text.
pub fn parse_inotify(fdinfo: &str) -> Vec<(i32, String)> {
    fdinfo
        .lines()
        .filter(|l| l.starts_with("inotify "))
        .filter_map(|l| {
            let field = |name: &str| l.split_whitespace().find_map(|f| f.strip_prefix(name));
            Some((
                i32::from_str_radix(field("fhandle-type:")?, 16).ok()?,
                field("f_handle:")?.to_string(),
            ))
        })
        .collect()
}

/// Folders and files that hold credentials, which few programs have a reason to watch.
fn credential_paths(home: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = [
        ".ssh",
        ".gnupg",
        ".gnupg/private-keys-v1.d",
        ".aws",
        ".aws/credentials",
        ".config/gcloud",
        ".kube",
        ".kube/config",
        ".docker",
        ".docker/config.json",
        ".password-store",
        ".local/share/keyrings",
        ".config/gh",
        ".config/gh/hosts.yml",
        ".git-credentials",
        ".netrc",
        ".npmrc",
        ".pypirc",
        ".cargo/credentials.toml",
        ".cargo/credentials",
    ]
    .iter()
    .map(|p| home.join(p))
    .collect();
    if let Ok(entries) = fs::read_dir(home.join(".ssh")) {
        paths.extend(
            entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().is_none_or(|e| e != "pub"))
                .take(64),
        );
    }
    paths
}

fn file_id(path: &Path) -> Option<String> {
    let m = fs::metadata(path).ok()?;
    Some(format!(
        "device {}:{}, inode {}, {} bytes, changed {}",
        libc::major(m.dev()),
        libc::minor(m.dev()),
        m.ino(),
        m.len(),
        crate::checks::heartbeat::utc(m.ctime().max(0) as u64)
    ))
}

fn is_appimage(path: &str) -> bool {
    use std::io::Read;
    let mut head = [0u8; 11];
    fs::File::open(path)
        .and_then(|mut f| f.read_exact(&mut head))
        .is_ok()
        && head.starts_with(b"\x7fELF")
        && &head[8..10] == b"AI"
}

fn socket_protocol(path: &Path) -> Option<String> {
    let c = CString::new(path.as_os_str().as_bytes()).ok()?;
    let mut buf = [0u8; 64];
    let n = unsafe {
        libc::getxattr(
            c.as_ptr(),
            c"system.sockprotoname".as_ptr(),
            buf.as_mut_ptr() as *mut libc::c_void,
            buf.len(),
        )
    };
    (n > 0).then(|| {
        String::from_utf8_lossy(&buf[..n as usize])
            .trim_end_matches('\0')
            .to_string()
    })
}

fn socket_inode(target: &str) -> Option<u64> {
    target
        .strip_prefix("socket:[")?
        .strip_suffix(']')?
        .parse()
        .ok()
}

fn loopback(addr: &SocketAddr) -> bool {
    match addr.ip() {
        IpAddr::V4(v) => v.is_loopback(),
        IpAddr::V6(v) => v.is_loopback() || v.to_ipv4_mapped().is_some_and(|m| m.is_loopback()),
    }
}

struct Walk {
    processes: Vec<Proc>,
    holders: HashMap<u64, Holder>,
    watches: Vec<Watch>,
    net_capable: Vec<Holder>,
    preload_ids: BTreeMap<String, String>,
}

/// CAP_NET_RAW and CAP_NET_ADMIN, the privileges that open packet and raw sockets.
const NET_CAPS: u64 = (1 << 13) | (1 << 12);

/// Whether a /proc/PID/status text shows a privilege to open packet or raw sockets.
pub fn net_capable(status: &str) -> bool {
    status
        .lines()
        .filter_map(|l| {
            l.strip_prefix("CapPrm:")
                .or_else(|| l.strip_prefix("CapEff:"))
        })
        .filter_map(|v| u64::from_str_radix(v.trim(), 16).ok())
        .any(|caps| caps & NET_CAPS != 0)
}

fn walk(uid: u32, home: &Path) -> Walk {
    let mut out = Walk {
        processes: vec![],
        holders: HashMap::new(),
        watches: vec![],
        net_capable: vec![],
        preload_ids: BTreeMap::new(),
    };
    // Remote ends of TCP sockets, by inode, for programs wired to one.
    let mut remotes: HashMap<u64, SocketAddr> = HashMap::new();
    for file in ["tcp", "tcp6"] {
        if let Ok(text) = fs::read_to_string(format!("/proc/net/{file}")) {
            for (s, inode) in parse_proc_net(&text, Protocol::Tcp) {
                if let Some(remote) = s.remote {
                    remotes.insert(inode, remote);
                }
            }
        }
    }
    let credentials: HashMap<(i32, String), String> = credential_paths(home)
        .into_iter()
        .filter_map(|p| Some((handle(&p)?, p.display().to_string())))
        .collect();
    let own_dir = std::env::current_exe()
        .ok()
        .and_then(|e| e.parent().map(Path::to_path_buf));
    let me = std::process::id();
    let pids = pids_kernel().unwrap_or_default();
    for pid in pids {
        let dir = PathBuf::from(format!("/proc/{pid}"));
        let (Ok(meta), Ok(stat)) = (fs::metadata(&dir), fs::read_to_string(dir.join("stat")))
        else {
            continue;
        };
        let kthread = parse_stat_flags(&stat).is_some_and(|f| f & PF_KTHREAD != 0);
        if kthread {
            continue;
        }
        let mut p = Proc {
            pid,
            uid: meta.uid(),
            comm: fs::read_to_string(dir.join("comm"))
                .map(|c| c.trim().to_string())
                .unwrap_or_default(),
            args: fs::read(dir.join("cmdline"))
                .unwrap_or_default()
                .split(|b| *b == 0)
                .filter(|a| !a.is_empty())
                .map(|a| String::from_utf8_lossy(a).into_owned())
                .collect(),
            mine: meta.uid() == uid,
            ..Default::default()
        };
        if !p.mine {
            out.processes.push(p);
            continue;
        }
        let exe_link = fs::read_link(dir.join("exe")).ok();
        p.exe = exe_link.as_ref().map(|e| e.to_string_lossy().into_owned());
        p.exe_id = file_id(&dir.join("exe"));
        if let Some(exe) = &p.exe {
            if let Some(path) = exe.strip_suffix(" (deleted)") {
                if exe.starts_with("/memfd:") {
                    p.appimage = p.args.first().filter(|a| is_appimage(a)).cloned();
                } else {
                    p.replaced = Path::new(path).exists();
                }
            }
        }
        if let Ok(environ) = fs::read(dir.join("environ")) {
            for var in environ.split(|b| *b == 0) {
                let var = String::from_utf8_lossy(var);
                if let Some(value) = var
                    .strip_prefix("LD_PRELOAD=")
                    .or_else(|| var.strip_prefix("LD_AUDIT="))
                {
                    for entry in value.split([':', ' ']).filter(|e| !e.is_empty()) {
                        if !p.preload.iter().any(|e| e == entry) {
                            p.preload.push(entry.to_string());
                        }
                        // Through the program's own root, so a sandboxed
                        // program's path resolves to the file it loads.
                        if entry.starts_with('/') && !out.preload_ids.contains_key(entry) {
                            if let Some(id) = file_id(&dir.join("root").join(&entry[1..])) {
                                out.preload_ids.insert(entry.to_string(), id);
                            }
                        }
                    }
                }
            }
        }
        // Bigman's own monitor watches folders on purpose.
        let ours = pid == me
            || exe_link.as_ref().is_some_and(|e| {
                e.parent().map(Path::to_path_buf) == own_dir
                    && e.file_name()
                        .is_some_and(|n| n.to_string_lossy().starts_with("bigman"))
            });
        let mut stdio = [None, None];
        let mut own_handle: Option<Option<(i32, String)>> = None;
        let fds = fs::read_dir(dir.join("fd"));
        if fds.is_err() {
            // A program started with extra privileges hides its open files even
            // from its own account. Its privileges still show.
            if fs::read_to_string(dir.join("status")).is_ok_and(|t| net_capable(&t)) {
                out.net_capable.push(Holder {
                    pid,
                    comm: p.comm.clone(),
                    exe: p.args.first().cloned(),
                });
            }
        }
        for fd in fds.into_iter().flatten().flatten() {
            let Ok(target) = fs::read_link(fd.path()) else {
                continue;
            };
            let target = target.to_string_lossy();
            let number = fd.file_name().to_string_lossy().parse::<u32>().ok();
            if let Some(inode) = socket_inode(&target) {
                if let Some(n) = number.filter(|n| *n < 2) {
                    stdio[n as usize] = Some(inode);
                }
                out.holders.entry(inode).or_insert_with(|| Holder {
                    pid,
                    comm: p.comm.clone(),
                    exe: p.exe.clone(),
                });
            } else if target == "anon_inode:inotify" && !ours {
                let Some(n) = number else { continue };
                let Ok(info) = fs::read_to_string(dir.join(format!("fdinfo/{n}"))) else {
                    continue;
                };
                for h in parse_inotify(&info) {
                    let hit = if let Some(path) = credentials.get(&h) {
                        Some((path.clone(), WatchKind::Credentials))
                    } else {
                        let own = own_handle.get_or_insert_with(|| handle(&dir.join("exe")));
                        (own.as_ref() == Some(&h))
                            .then(|| (p.exe.clone().unwrap_or_default(), WatchKind::OwnProgram))
                    };
                    if let Some((target, kind)) = hit {
                        out.watches.push(Watch {
                            pid,
                            comm: p.comm.clone(),
                            exe: p.exe.clone(),
                            exe_id: p.exe_id.clone(),
                            target,
                            kind,
                        });
                    }
                }
            }
        }
        if let (Some(a), Some(b)) = (stdio[0], stdio[1]) {
            if a == b {
                if let Some(protocol) = socket_protocol(&dir.join("fd/0")) {
                    if ["TCP", "UDP", "MPTCP", "SCTP"]
                        .iter()
                        .any(|p| protocol.starts_with(p))
                    {
                        let remote = remotes.get(&a);
                        p.stdio_socket = Some(StdioSocket {
                            protocol,
                            remote: remote.map(|r| r.to_string()),
                            loopback: remote.is_some_and(loopback),
                        });
                    }
                }
            }
        }
        out.processes.push(p);
    }
    out
}

// --- Packet and raw sockets ---------------------------------------------------------------

/// `/proc/net/packet` rows: (EtherType, interface index, uid, inode).
pub fn parse_packet(text: &str) -> Vec<(u16, u32, u32, u64)> {
    text.lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split_whitespace().collect();
            Some((
                u16::from_str_radix(f.get(3)?, 16).ok()?,
                f.get(4)?.parse().ok()?,
                f.get(7)?.parse().ok()?,
                f.get(8)?.parse().ok()?,
            ))
        })
        .collect()
}

fn interfaces() -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for entry in fs::read_dir("/sys/class/net")
        .into_iter()
        .flatten()
        .flatten()
    {
        if let Some(index) = fs::read_to_string(entry.path().join("ifindex"))
            .ok()
            .and_then(|t| t.trim().parse().ok())
        {
            out.insert(index, entry.file_name().to_string_lossy().into_owned());
        }
    }
    out
}

fn packet_and_raw(
    holders: &HashMap<u64, Holder>,
    gaps: &mut Vec<Skipped>,
    checked: &mut Vec<String>,
) -> (Vec<PacketSocket>, Vec<RawSocket>) {
    let mut packet = vec![];
    let mut raw = vec![];
    match kernel_text("/proc/net/packet", 16 * 1024 * 1024) {
        Ok(text) => {
            let names = interfaces();
            for (protocol, index, uid, inode) in parse_packet(&text) {
                packet.push(PacketSocket {
                    protocol,
                    iface: match index {
                        0 => "any interface".into(),
                        i => names
                            .get(&i)
                            .cloned()
                            .unwrap_or_else(|| format!("interface {i}")),
                    },
                    uid,
                    holder: holders.get(&inode).cloned(),
                });
            }
        }
        Err(e) => gaps.push(gap(
            "Packet sockets",
            SkipReason::Unreadable,
            format!("/proc/net/packet: {e}"),
        )),
    }
    for (file, v6) in [("raw", false), ("raw6", true)] {
        match kernel_text(&format!("/proc/net/{file}"), 16 * 1024 * 1024) {
            // The table has the columns of /proc/net/udp, with the protocol in the port's place.
            Ok(text) => {
                for (s, inode) in parse_proc_net(&text, Protocol::Udp) {
                    raw.push(RawSocket {
                        protocol: s.local.port(),
                        v6,
                        uid: s.uid,
                        holder: holders.get(&inode).cloned(),
                    });
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => gaps.push(gap(
                "Raw sockets",
                SkipReason::Unreadable,
                format!("/proc/net/{file}: {e}"),
            )),
        }
    }
    checked.push("Programs that read packets below TCP and UDP (packet and raw sockets)".into());
    (packet, raw)
}

/// The owners of bus names in `busctl --json=short list` output.
pub fn parse_bus_names(json: &str) -> Vec<(String, u32, String)> {
    let Ok(rows) = serde_json::from_str::<Vec<serde_json::Value>>(json) else {
        return vec![];
    };
    rows.iter()
        .filter_map(|r| {
            let name = r["name"].as_str()?;
            // Connection names like ":1.42" are not names a program asks for.
            (!name.starts_with(':')).then_some(())?;
            Some((
                name.to_string(),
                r["pid"].as_u64()? as u32,
                r["process"].as_str().unwrap_or("").to_string(),
            ))
        })
        .collect()
}

fn bus_names(
    gaps: &mut Vec<Skipped>,
    checked: &mut Vec<String>,
    stop: &Cancel,
) -> Option<Vec<BusName>> {
    let what = "Which programs own the password-store and portal bus names";
    if !runner::available("busctl") {
        gaps.push(gap(
            what,
            SkipReason::ToolMissing,
            "busctl is not installed.",
        ));
        return None;
    }
    let mut cmd = Command::new("busctl");
    cmd.args(["--user", "--no-pager", "--json=short", "list"]);
    match runner::run(&mut cmd, 10, stop) {
        Ok(out) if out.ok() => {
            checked
                .push("Owners of the password-store and portal names on your session bus".into());
            Some(
                parse_bus_names(&out.stdout)
                    .into_iter()
                    .map(|(name, pid, comm)| BusName {
                        exe: fs::read_link(format!("/proc/{pid}/exe"))
                            .ok()
                            .map(|e| e.to_string_lossy().into_owned()),
                        name,
                        pid,
                        comm,
                    })
                    .collect(),
            )
        }
        Ok(out) => {
            gaps.push(gap(what, SkipReason::Failed, out.stderr.trim()));
            None
        }
        Err(e) => {
            gaps.push(gap(what, SkipReason::Failed, format!("{e:#}")));
            None
        }
    }
}

/// Everything `assess` needs, read as you.
pub fn gather(stop: &Cancel) -> Snapshot {
    let uid = unsafe { libc::getuid() };
    let home = crate::store::home();
    let mut gaps = vec![];
    let mut checked = vec![];
    if !rawsys::DIRECT {
        gaps.push(gap(
            "The C library's view against the kernel's",
            SkipReason::NotApplicable,
            "Direct system calls are implemented for x86-64 and 64-bit ARM.",
        ));
    } else {
        checked.push("Process list, module list, socket tables and /etc/ld.so.preload: the C library's reading against the kernel's".into());
    }
    let pids = pid_views(&mut gaps, &mut checked, stop);
    let (modules, bpf) = module_views(&mut gaps, &mut checked);
    if bpf.is_some() {
        checked.push("BPF program names in the kernel's symbol list".into());
    }
    let sockets = socket_views(&mut gaps, &mut checked);
    let preload = preload(&mut gaps);
    let walked = walk(uid, &home);
    checked.push(
        "Your programs: executable file, standard input and output, preloaded libraries".into(),
    );
    checked.push("Inotify watches on credential folders and on a program's own file".into());
    let (packet_sockets, raw_sockets) = packet_and_raw(&walked.holders, &mut gaps, &mut checked);
    let bus_names = bus_names(&mut gaps, &mut checked, stop);
    Snapshot {
        uid,
        home: home.display().to_string(),
        pids,
        modules,
        sockets,
        preload,
        processes: walked.processes,
        packet_sockets,
        raw_sockets,
        bpf,
        watches: walked.watches,
        net_capable: walked.net_capable,
        preload_ids: walked.preload_ids,
        bus_names,
        checked,
        gaps,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::process::CommandExt;

    #[test]
    fn a_difference_must_hold_on_two_passes() {
        let mut passes = vec![BTreeSet::from([1, 2, 3]), BTreeSet::from([2, 3, 4])].into_iter();
        assert_eq!(twice(|| passes.next().unwrap()), vec![2, 3]);
        let mut calls = 0;
        let none: Vec<u32> = twice(|| {
            calls += 1;
            BTreeSet::new()
        });
        assert!(none.is_empty());
        assert_eq!(calls, 1, "nothing differed, so nothing is read again");
    }

    #[test]
    fn module_tables_are_parsed() {
        let modules = parse_modules(
            "nft_masq 12288 4 - Live 0x0000000000000000\nnvidia 9000 2 nvidia_uvm, Live 0x0000000000000000 (POE)\nloading 100 0 - Loading 0x0000000000000000\n",
        );
        assert_eq!(
            modules,
            vec![
                ("nft_masq".into(), true, String::new()),
                ("nvidia".into(), true, "POE".into()),
                ("loading".into(), false, String::new()),
            ]
        );
        let (mods, bpf) = parse_kallsyms(
            "0000000000000000 T do_exit\n0000000000000000 t nft_masq_init\t[nft_masq]\n0000000000000000 t bpf_prog_0123456789abcdef_sd_devices\t[bpf]\n0000000000000000 t bpf_prog_0123456789abcdef\t[bpf]\n0000000000000000 t bpf_trampoline_1\t[bpf]\n0000000000000000 t ftrace_trampoline\t[__builtin__ftrace]\n",
        );
        assert_eq!(mods, BTreeSet::from(["nft_masq".to_string()]));
        assert_eq!(
            bpf,
            BTreeSet::from(["sd_devices".to_string(), "(unnamed)".to_string()])
        );
        let files = module_files(
            "kernel/net/netfilter/nft-masq.ko.xz: kernel/net/nf_tables.ko.xz\nextra/nvidia.ko:\n",
            "kernel/fs/ext4/ext4.ko\n",
        );
        assert_eq!(
            files,
            BTreeSet::from([
                "nft_masq".to_string(),
                "nvidia".to_string(),
                "ext4".to_string()
            ])
        );
    }

    #[test]
    fn hidepid_is_read_from_the_proc_mount_only() {
        let plain = "54 43 0:25 / /proc rw,nosuid shared:13 - proc proc rw\n";
        assert!(!hides_pids(plain));
        assert!(hides_pids(
            "54 43 0:25 / /proc rw,nosuid - proc proc rw,hidepid=2\n"
        ));
        assert!(hides_pids(
            "54 43 0:25 / /proc rw - proc proc rw,hidepid=invisible\n"
        ));
        assert!(!hides_pids(
            "54 43 0:25 / /proc rw - proc proc rw,hidepid=off\n"
        ));
        assert!(!hides_pids(
            "99 43 0:25 / /srv/jail/proc rw - proc proc rw,hidepid=2\n"
        ));
    }

    #[test]
    fn packet_inotify_stat_and_bus_tables_are_parsed() {
        let packet = "sk               RefCnt Type Proto  Iface R Rmem   User   Inode\n00000000bf64a50b 3      2    890d   3     1 0      0      9842320\n00000000cc956a25 3      3    0003   0     1 0      1000   77\n";
        assert_eq!(
            parse_packet(packet),
            vec![(0x890d, 3, 0, 9842320), (3, 0, 1000, 77)]
        );
        let fdinfo = "pos:\t0\nflags:\t02000000\nmnt_id:\t19\nino:\t1056\ninotify wd:1 ino:594f sdev:25 mask:100 ignored_mask:0 fhandle-bytes:14 fhandle-type:4d f_handle:4f59000000000000000100000000000094000000\ninotify wd:3 ino:1 sdev:35 mask:100 ignored_mask:0 fhandle-bytes:c fhandle-type:1 f_handle:7c1e95690100000000000000\n";
        assert_eq!(
            parse_inotify(fdinfo),
            vec![
                (0x4d, "4f59000000000000000100000000000094000000".to_string()),
                (1, "7c1e95690100000000000000".to_string()),
            ]
        );
        // The name holds spaces and a parenthesis; flags are the ninth field.
        assert_eq!(
            parse_stat_flags("2 (kthreadd) S 0 0 0 0 -1 2129984 0 0 0 0"),
            Some(2129984)
        );
        assert_eq!(
            parse_stat_flags("77 (a) b) S 1 77 77 0 -1 4194560 12 0"),
            Some(4194560)
        );
        assert!(net_capable(
            "Name:\tx\nCapPrm:\t0000000000002000\nCapEff:\t0000000000000000\n"
        ));
        assert!(!net_capable(
            "CapPrm:\t0000000000800000\nCapEff:\t0000000000800000\n"
        ));
        let bus = r#"[{"name":":1.1","pid":5084,"process":"systemd"},{"name":"org.freedesktop.secrets","pid":7001,"process":"gnome-keyring-d","user":"u"}]"#;
        assert_eq!(
            parse_bus_names(bus),
            vec![(
                "org.freedesktop.secrets".into(),
                7001,
                "gnome-keyring-d".into()
            )]
        );
    }

    #[test]
    fn socket_diagnostics_records_are_parsed() {
        // One IPv4 record: 127.0.0.1:8080 listening, uid 1000; then the end marker.
        let mut record = vec![0u8; NL_HEADER + DIAG_MSG];
        let len = record.len() as u32;
        record[0..4].copy_from_slice(&len.to_ne_bytes());
        record[4..6].copy_from_slice(&SOCK_DIAG_BY_FAMILY.to_ne_bytes());
        let m = NL_HEADER;
        record[m] = libc::AF_INET as u8;
        record[m + 4..m + 6].copy_from_slice(&8080u16.to_be_bytes());
        record[m + 8..m + 12].copy_from_slice(&[127, 0, 0, 1]);
        record[m + 64..m + 68].copy_from_slice(&1000u32.to_ne_bytes());
        let mut done = vec![0u8; NL_HEADER + 4];
        done[0..4].copy_from_slice(&20u32.to_ne_bytes());
        done[4..6].copy_from_slice(&NLMSG_DONE.to_ne_bytes());
        record.extend(done);
        let mut rows = vec![];
        assert!(parse_diag(&record, "tcp", &mut rows).unwrap());
        assert_eq!(
            rows,
            vec![SocketRow {
                protocol: "tcp".into(),
                local: "127.0.0.1:8080".into(),
                remote: None,
                uid: 1000
            }]
        );
        let table = "  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode\n   0: 0100007F:1F90 00000000:0000 0A 00000000:00000000 00:00000000 00000000  1000        0 4242 1 0 100 0 0 10 0\n   1: 0100007F:9C40 0100007F:1F90 06 00000000:00000000 00:00000000 00000000  1000        0 0 1 0 100 0 0 10 0\n";
        assert_eq!(
            procfs_rows(&[("tcp", table.to_string())]),
            rows,
            "a closing connection is in neither comparison"
        );
    }

    /// On a computer that hides nothing, every pair of views agrees. This is
    /// the test that the comparisons do not report ordinary activity.
    #[test]
    fn the_views_of_this_computer_agree() {
        let stop = runner::cancel();
        let s = gather(&stop);
        let p = s.pids.as_ref().expect("the process list");
        assert!(p.listed > 0);
        assert_eq!(p.unlisted, vec![], "signal probe against /proc");
        assert_eq!(p.libc_missing, vec![], "C library against the kernel");
        if let Some(v) = &s.sockets {
            assert!(v.compared > 0);
            assert_eq!(v.only_diag, vec![], "socket diagnostics against /proc/net");
            assert_eq!(
                v.only_procfs,
                vec![],
                "/proc/net against socket diagnostics"
            );
            assert_eq!(v.libc_missing, vec![]);
        }
        if let Some(m) = &s.modules {
            assert_eq!(m.mismatched, vec![]);
            assert_eq!(m.libc_missing, Vec::<String>::new());
        }
        let p = s.preload.expect("/etc/ld.so.preload");
        assert_eq!(p.kernel, p.libc);
    }

    /// The live checks on a harmless program: `sleep` copied to a temporary
    /// folder, started, and its file deleted; and a watch on its own file.
    #[test]
    fn a_program_whose_file_was_deleted_is_seen() {
        let d = tempfile::tempdir().unwrap();
        let copy = d.path().join("bigman-fixture-sleep");
        fs::copy("/usr/bin/sleep", &copy).unwrap();
        let mut child = Command::new(&copy)
            .arg("30")
            .arg0("bigman-fixture-sleep")
            .spawn()
            .unwrap();
        // Wait for the exec, then delete the file it runs from.
        for _ in 0..100 {
            if fs::read_link(format!("/proc/{}/exe", child.id())).is_ok_and(|e| e == copy) {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        fs::remove_file(&copy).unwrap();
        let walked = walk(unsafe { libc::getuid() }, d.path());
        let _ = child.kill();
        let _ = child.wait();
        let p = walked
            .processes
            .iter()
            .find(|p| p.pid == child.id())
            .expect("the child is listed");
        assert!(p.mine);
        assert_eq!(
            p.exe.as_deref(),
            Some(format!("{} (deleted)", copy.display()).as_str())
        );
        assert!(!p.replaced);
        assert!(
            p.exe_id.is_some(),
            "the deleted file is still identified through /proc"
        );
        let items = super::super::assess(&Snapshot {
            uid: p.uid,
            processes: vec![p.clone()],
            ..Default::default()
        });
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].rule, "process.exe-deleted");
    }

    #[test]
    fn an_inotify_watch_is_matched_to_the_file_by_its_handle() {
        let d = tempfile::tempdir().unwrap();
        let watched = d.path().join("secrets");
        fs::create_dir(&watched).unwrap();
        fs::create_dir(d.path().join("other")).unwrap();
        let fd = unsafe { libc::inotify_init1(libc::IN_CLOEXEC) };
        assert!(fd >= 0);
        let c = CString::new(watched.as_os_str().as_bytes()).unwrap();
        assert!(unsafe { libc::inotify_add_watch(fd, c.as_ptr(), libc::IN_CREATE) } >= 0);
        let info = fs::read_to_string(format!("/proc/self/fdinfo/{fd}")).unwrap();
        unsafe { libc::close(fd) };
        let watches = parse_inotify(&info);
        assert_eq!(watches.len(), 1);
        assert_eq!(Some(&watches[0]), handle(&watched).as_ref());
        assert_ne!(Some(&watches[0]), handle(&d.path().join("other")).as_ref());
    }
}
