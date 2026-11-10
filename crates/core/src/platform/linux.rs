//! Linux implementations of the platform traits, plus their pure parsers.
use super::*;
use crate::runner::{self, Cancel};
use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    fs,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    os::unix::fs::MetadataExt,
    path::Path,
    process::Command,
};

/// `/sys/class/net/*/statistics`, skipping the loopback interface.
pub struct SysNetCounters;

impl NetworkCounters for SysNetCounters {
    fn counters(&self) -> Result<Vec<InterfaceCounters>> {
        let mut out = vec![];
        for entry in fs::read_dir("/sys/class/net")? {
            let iface = entry?.file_name().to_string_lossy().into_owned();
            if iface == "lo" {
                continue;
            }
            let read = |name: &str| -> Option<u64> {
                fs::read_to_string(format!("/sys/class/net/{iface}/statistics/{name}"))
                    .ok()?
                    .trim()
                    .parse()
                    .ok()
            };
            if let (Some(rx_bytes), Some(tx_bytes)) = (read("rx_bytes"), read("tx_bytes")) {
                out.push(InterfaceCounters {
                    iface,
                    rx_bytes,
                    tx_bytes,
                });
            }
        }
        out.sort_by(|a, b| a.iface.cmp(&b.iface));
        Ok(out)
    }
}

const TCP_STATES: [&str; 12] = [
    "unknown",
    "established",
    "syn_sent",
    "syn_recv",
    "fin_wait1",
    "fin_wait2",
    "time_wait",
    "close",
    "close_wait",
    "last_ack",
    "listen",
    "closing",
];

/// `0100007F:698A` → 127.0.0.1:27018. IPv6 addresses are four little-endian words.
pub fn parse_proc_addr(text: &str) -> Option<SocketAddr> {
    let (ip, port) = text.split_once(':')?;
    let port = u16::from_str_radix(port, 16).ok()?;
    let ip = match ip.len() {
        8 => IpAddr::V4(Ipv4Addr::from(
            u32::from_str_radix(ip, 16).ok()?.swap_bytes(),
        )),
        32 => {
            let mut bytes = [0u8; 16];
            for word in 0..4 {
                let w = u32::from_str_radix(&ip[word * 8..word * 8 + 8], 16).ok()?;
                bytes[word * 4..word * 4 + 4].copy_from_slice(&w.to_le_bytes());
            }
            IpAddr::V6(Ipv6Addr::from(bytes))
        }
        _ => return None,
    };
    Some(SocketAddr::new(ip, port))
}

/// One `/proc/net/{tcp,tcp6,udp,udp6}` table, with the socket inode for each row.
pub fn parse_proc_net(text: &str, protocol: Protocol) -> Vec<(Socket, u64)> {
    let mut out = vec![];
    for line in text.lines().skip(1) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 10 {
            continue;
        }
        let (Some(local), Some(remote)) = (parse_proc_addr(fields[1]), parse_proc_addr(fields[2]))
        else {
            continue;
        };
        let state_code = usize::from_str_radix(fields[3], 16).unwrap_or(0);
        let state = match protocol {
            Protocol::Tcp => TCP_STATES.get(state_code).copied().unwrap_or("unknown"),
            Protocol::Udp if remote.port() == 0 => "unconnected",
            Protocol::Udp => "established",
        };
        out.push((
            Socket {
                protocol,
                local,
                remote: (remote.port() != 0).then_some(remote),
                state: state.into(),
                uid: fields[7].parse().unwrap_or(u32::MAX),
                process: None,
            },
            fields[9].parse().unwrap_or(0),
        ));
    }
    out
}

/// The account Bigman runs as.
pub fn own_uid() -> u32 {
    unsafe { libc::getuid() }
}

/// Sockets from /proc/net, matched to your own processes through their file
/// descriptors. Other users' sockets are listed without a process (rootless).
pub struct ProcSockets;

pub(super) fn own_socket_owners() -> HashMap<u64, ProcessInfo> {
    let uid = unsafe { libc::getuid() };
    let mut owners = HashMap::new();
    let Ok(procs) = fs::read_dir("/proc") else {
        return owners;
    };
    for entry in procs.flatten() {
        let name = entry.file_name();
        let Some(pid) = name.to_str().and_then(|n| n.parse::<u32>().ok()) else {
            continue;
        };
        let dir = entry.path();
        if !fs::metadata(&dir).is_ok_and(|m| m.uid() == uid) {
            continue;
        }
        let Ok(fds) = fs::read_dir(dir.join("fd")) else {
            continue;
        };
        let mut info: Option<ProcessInfo> = None;
        for fd in fds.flatten() {
            let Ok(target) = fs::read_link(fd.path()) else {
                continue;
            };
            let target = target.to_string_lossy();
            let Some(inode) = target
                .strip_prefix("socket:[")
                .and_then(|t| t.strip_suffix(']'))
                .and_then(|t| t.parse().ok())
            else {
                continue;
            };
            let info = info.get_or_insert_with(|| ProcessInfo {
                pid,
                name: fs::read_to_string(dir.join("comm"))
                    .map(|c| c.trim().to_string())
                    .unwrap_or_default(),
                exe: fs::read_link(dir.join("exe")).ok(),
            });
            owners.insert(inode, info.clone());
        }
    }
    owners
}

impl ProcessSockets for ProcSockets {
    fn sockets(&self) -> Result<Vec<Socket>> {
        let owners = own_socket_owners();
        let mut out = vec![];
        for (file, protocol) in [
            ("tcp", Protocol::Tcp),
            ("tcp6", Protocol::Tcp),
            ("udp", Protocol::Udp),
            ("udp6", Protocol::Udp),
        ] {
            let Ok(text) = fs::read_to_string(format!("/proc/net/{file}")) else {
                continue;
            };
            for (mut socket, inode) in parse_proc_net(&text, protocol) {
                socket.process = owners.get(&inode).cloned();
                out.push(socket);
            }
        }
        Ok(out)
    }
}

/// dnf5's transaction history database, opened read-only.
pub struct Dnf5History {
    pub path: std::path::PathBuf,
}

impl Default for Dnf5History {
    fn default() -> Self {
        Dnf5History {
            path: "/usr/lib/sysimage/libdnf5/transaction_history.sqlite".into(),
        }
    }
}

impl PackageHistory for Dnf5History {
    fn transactions_after(&self, cursor: Option<&str>) -> Result<Vec<Transaction>> {
        use rusqlite::{Connection, OpenFlags};
        let conn = Connection::open_with_flags(
            &self.path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .with_context(|| format!("Cannot read {}", self.path.display()))?;
        let after: i64 = cursor.and_then(|c| c.parse().ok()).unwrap_or(0);
        let mut trans = conn.prepare("SELECT id, dt_begin, coalesce(description, '') FROM trans WHERE id > ?1 AND state_id = 2 ORDER BY id")?;
        let mut items = conn.prepare(
            "SELECT pn.name, r.epoch, r.version, r.release, act.name FROM trans_item ti
             JOIN rpm r ON r.item_id = ti.item_id JOIN pkg_name pn ON pn.id = r.name_id
             JOIN trans_item_action act ON act.id = ti.action_id WHERE ti.trans_id = ?1 ORDER BY pn.name",
        )?;
        let rows: Vec<(i64, i64, String)> = trans
            .query_map([after], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
            .collect::<rusqlite::Result<_>>()?;
        let mut out = vec![];
        for (id, at, command) in rows {
            let changes = items
                .query_map([id], |r| {
                    let epoch: i64 = r.get(1)?;
                    let version = format!(
                        "{}{}-{}",
                        if epoch > 0 {
                            format!("{epoch}:")
                        } else {
                            String::new()
                        },
                        r.get::<_, String>(2)?,
                        r.get::<_, String>(3)?
                    );
                    let kind = match r.get::<_, String>(4)?.as_str() {
                        "Install" => Some(ChangeKind::Install),
                        "Upgrade" => Some(ChangeKind::Upgrade),
                        "Downgrade" => Some(ChangeKind::Downgrade),
                        "Reinstall" => Some(ChangeKind::Reinstall),
                        "Remove" => Some(ChangeKind::Remove),
                        _ => None, // "Replaced" rows are the old side of an upgrade.
                    };
                    let name: String = r.get(0)?;
                    Ok(kind.map(|kind| PackageChange {
                        name,
                        version,
                        kind,
                    }))
                })?
                .filter_map(|c| c.transpose())
                .collect::<rusqlite::Result<Vec<_>>>()?;
            out.push(Transaction {
                id: id.to_string(),
                at: at as u64,
                manager: "dnf".into(),
                command,
                changes,
            });
        }
        Ok(out)
    }
}

/// Transactions from `/var/log/pacman.log`, numbered by their starting line.
pub fn parse_pacman_log(text: &str) -> Vec<Transaction> {
    let mut out: Vec<Transaction> = vec![];
    let mut open: Option<Transaction> = None;
    let mut command = String::new();
    for (n, line) in text.lines().enumerate() {
        let Some((stamp, rest)) = line.strip_prefix('[').and_then(|l| l.split_once("] ")) else {
            continue;
        };
        if let Some(cmd) = rest.strip_prefix("[PACMAN] Running '") {
            command = cmd.trim_end_matches('\'').into();
        }
        let Some(event) = rest.strip_prefix("[ALPM] ") else {
            continue;
        };
        if event == "transaction started" {
            open = Some(Transaction {
                id: (n + 1).to_string(),
                at: parse_iso8601(stamp),
                manager: "pacman".into(),
                command: command.clone(),
                changes: vec![],
            });
        } else if event == "transaction completed" {
            out.extend(open.take());
        } else if let Some(t) = open.as_mut() {
            let (kind, rest) = match event.split_once(' ') {
                Some(("installed", r)) => (ChangeKind::Install, r),
                Some(("upgraded", r)) => (ChangeKind::Upgrade, r),
                Some(("downgraded", r)) => (ChangeKind::Downgrade, r),
                Some(("reinstalled", r)) => (ChangeKind::Reinstall, r),
                Some(("removed", r)) => (ChangeKind::Remove, r),
                _ => continue,
            };
            if let Some((name, versions)) = rest.split_once(" (") {
                let version = versions
                    .trim_end_matches(')')
                    .rsplit(" -> ")
                    .next()
                    .unwrap_or_default();
                t.changes.push(PackageChange {
                    name: name.into(),
                    version: version.into(),
                    kind,
                });
            }
        }
    }
    out
}

/// Transactions from `/var/log/dpkg.log`: one per run of consecutive package lines.
pub fn parse_dpkg_log(text: &str) -> Vec<Transaction> {
    let mut out: Vec<Transaction> = vec![];
    for (n, line) in text.lines().enumerate() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 6 {
            if line.contains(" startup ") {
                out.push(Transaction {
                    id: (n + 1).to_string(),
                    at: 0,
                    manager: "apt".into(),
                    command: String::new(),
                    changes: vec![],
                });
            }
            continue;
        }
        let kind = match fields[2] {
            "install" => ChangeKind::Install,
            "upgrade" => ChangeKind::Upgrade,
            "remove" | "purge" => ChangeKind::Remove,
            _ => continue,
        };
        if out.is_empty() {
            out.push(Transaction {
                id: (n + 1).to_string(),
                at: 0,
                manager: "apt".into(),
                command: String::new(),
                changes: vec![],
            });
        }
        let t = out.last_mut().expect("transaction");
        if t.at == 0 {
            t.at = parse_iso8601(&format!("{}T{}Z", fields[0], fields[1]));
        }
        let name = fields[3].split(':').next().unwrap_or(fields[3]);
        let version = if kind == ChangeKind::Remove {
            fields[4]
        } else {
            fields[5]
        };
        t.changes.push(PackageChange {
            name: name.into(),
            version: version.into(),
            kind,
        });
    }
    out.retain(|t| !t.changes.is_empty());
    out
}

/// Parses `2026-10-29T14:02:00+0000`, `...Z` or `...+00:00` to Unix seconds (UTC offsets only
/// matter to the hour, which is all package logs need). Unparseable stamps give 0.
pub fn parse_iso8601(stamp: &str) -> u64 {
    let digits = |s: &str| s.parse::<i64>().ok();
    let (Some(date), Some(time)) = (stamp.get(0..10), stamp.get(11..19)) else {
        return 0;
    };
    let d: Vec<i64> = date.split('-').filter_map(digits).collect();
    let t: Vec<i64> = time.split(':').filter_map(digits).collect();
    if d.len() != 3 || t.len() != 3 {
        return 0;
    }
    // Days from the civil date (Howard Hinnant's algorithm).
    let (y, m, day) = (if d[1] <= 2 { d[0] - 1 } else { d[0] }, d[1], d[2]);
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    let offset = stamp
        .get(19..)
        .and_then(|o| {
            let sign = if o.starts_with('-') { -1 } else { 1 };
            let o = o.trim_start_matches(['+', '-']).replace(':', "");
            Some(
                sign * (o.get(0..2)?.parse::<i64>().ok()? * 3600
                    + o.get(2..4)?.parse::<i64>().ok()? * 60),
            )
        })
        .unwrap_or(0);
    (days * 86_400 + t[0] * 3_600 + t[1] * 60 + t[2] - offset).max(0) as u64
}

/// A package log read whole each time; transactions after the cursor are new.
pub struct LogHistory {
    pub path: std::path::PathBuf,
    pub parse: fn(&str) -> Vec<Transaction>,
}

impl PackageHistory for LogHistory {
    fn transactions_after(&self, cursor: Option<&str>) -> Result<Vec<Transaction>> {
        let text = fs::read_to_string(&self.path)
            .with_context(|| format!("Cannot read {}", self.path.display()))?;
        let after: usize = cursor.and_then(|c| c.parse().ok()).unwrap_or(0);
        Ok((self.parse)(&text)
            .into_iter()
            .filter(|t| t.id.parse::<usize>().unwrap_or(0) > after)
            .collect())
    }
}

/// The package history for this system, if it has one Bigman understands.
pub fn package_history() -> Option<Box<dyn PackageHistory>> {
    let dnf = Dnf5History::default();
    if dnf.path.exists() {
        return Some(Box::new(dnf));
    }
    for (path, parse) in [
        (
            "/var/log/pacman.log",
            parse_pacman_log as fn(&str) -> Vec<Transaction>,
        ),
        ("/var/log/dpkg.log", parse_dpkg_log),
    ] {
        if Path::new(path).exists() {
            return Some(Box::new(LogHistory {
                path: path.into(),
                parse,
            }));
        }
    }
    None
}

/// The system journal through journalctl. Readable to members of wheel, adm or
/// systemd-journal; for anyone else Bigman reports the gap instead of "no denials".
pub struct Journalctl {
    pub stop: Cancel,
}

/// Whether this user may read the system journal.
pub fn journal_readable() -> bool {
    let mut groups = vec![0 as libc::gid_t; 256];
    let n = unsafe { libc::getgroups(groups.len() as i32, groups.as_mut_ptr()) };
    if n < 0 {
        return false;
    }
    groups.truncate(n as usize);
    let names = fs::read_to_string("/etc/group").unwrap_or_default();
    names.lines().any(|line| {
        let mut f = line.split(':');
        let (Some(name), _, Some(gid)) = (f.next(), f.next(), f.next()) else {
            return false;
        };
        ["wheel", "adm", "systemd-journal"].contains(&name)
            && gid
                .parse::<libc::gid_t>()
                .is_ok_and(|g| groups.contains(&g))
    })
}

/// Denial lines out of `journalctl -o json` output, and the last cursor.
pub fn parse_journal_json(output: &str) -> (Vec<JournalRecord>, Option<String>) {
    let mut records = vec![];
    let mut cursor = None;
    for line in output.lines() {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        cursor = v["__CURSOR"].as_str().map(String::from).or(cursor);
        let Some(message) = v["MESSAGE"].as_str() else {
            continue;
        };
        if message.contains("avc:  denied") || message.contains("apparmor=\"DENIED\"") {
            let at = v["__REALTIME_TIMESTAMP"]
                .as_str()
                .and_then(|t| t.parse::<u64>().ok())
                .unwrap_or(0)
                / 1_000_000;
            records.push(JournalRecord {
                at,
                message: message.into(),
            });
        }
    }
    (records, cursor)
}

impl Journal for Journalctl {
    fn denials_after(
        &self,
        cursor: Option<&str>,
    ) -> Result<Option<(Vec<JournalRecord>, Option<String>)>> {
        if !journal_readable() {
            return Ok(None);
        }
        let mut cmd = Command::new("journalctl");
        cmd.args(["--no-pager", "--quiet", "-o", "json"]);
        match cursor {
            Some(c) => cmd.arg(format!("--after-cursor={c}")),
            None => cmd.arg("--since=-24h"),
        };
        cmd.args(["_TRANSPORT=audit", "+", "_TRANSPORT=kernel"]);
        let out = runner::run(&mut cmd, 20, &self.stop)?;
        if out.code != Some(0) {
            anyhow::bail!("journalctl failed: {}", out.stderr.trim());
        }
        let (records, new_cursor) = parse_journal_json(&out.stdout);
        Ok(Some((
            records,
            new_cursor.or_else(|| cursor.map(String::from)),
        )))
    }
}

/// Your own processes from /proc: parent, name, executable and command line.
pub struct ProcList;

/// The parent pid out of `/proc/PID/stat`. The name field may hold spaces and
/// parentheses, so fields are counted after the last ')'.
pub fn parse_stat_ppid(stat: &str) -> Option<u32> {
    let rest = &stat[stat.rfind(')')? + 1..];
    rest.split_whitespace().nth(1)?.parse().ok()
}

impl Processes for ProcList {
    fn processes(&self) -> Result<Vec<ProcessEntry>> {
        let uid = unsafe { libc::getuid() };
        let mut out = vec![];
        for entry in fs::read_dir("/proc").context("/proc")?.flatten() {
            let Some(pid) = entry
                .file_name()
                .to_str()
                .and_then(|n| n.parse::<u32>().ok())
            else {
                continue;
            };
            let dir = entry.path();
            if !fs::metadata(&dir).is_ok_and(|m| m.uid() == uid) {
                continue;
            }
            // A process can exit between these reads; skip it rather than guess.
            let (Ok(stat), Ok(cmdline)) = (
                fs::read_to_string(dir.join("stat")),
                fs::read(dir.join("cmdline")),
            ) else {
                continue;
            };
            let Some(ppid) = parse_stat_ppid(&stat) else {
                continue;
            };
            out.push(ProcessEntry {
                pid,
                ppid,
                name: fs::read_to_string(dir.join("comm"))
                    .map(|c| c.trim().to_string())
                    .unwrap_or_default(),
                exe: fs::read_link(dir.join("exe")).ok(),
                args: cmdline
                    .split(|b| *b == 0)
                    .filter(|a| !a.is_empty())
                    .map(|a| String::from_utf8_lossy(a).into_owned())
                    .collect(),
            });
        }
        Ok(out)
    }
}

/// Login sessions through `loginctl`, logind's public interface. (logind's
/// files under /run/systemd/sessions are marked private and are not read.)
pub struct Loginctl {
    pub stop: Cancel,
}

const SESSION_PROPERTIES: [&str; 8] = [
    "Id",
    "Name",
    "Service",
    "Remote",
    "RemoteHost",
    "RemoteUser",
    "TTY",
    "Class",
];

/// `loginctl show-session -p ...` output for several sessions: blocks of
/// `Key=value` lines separated by blank lines.
pub fn parse_show_session(text: &str) -> Vec<LoginSession> {
    text.split("\n\n")
        .filter_map(|block| {
            let mut f: HashMap<&str, &str> = HashMap::new();
            for line in block.lines() {
                if let Some((k, v)) = line.split_once('=') {
                    f.insert(k.trim(), v.trim());
                }
            }
            let value = |k: &str| f.get(k).filter(|v| !v.is_empty()).map(|v| v.to_string());
            Some(LoginSession {
                id: value("Id")?,
                user: value("Name").unwrap_or_default(),
                service: value("Service").unwrap_or_default(),
                remote: f.get("Remote") == Some(&"yes"),
                remote_host: value("RemoteHost"),
                remote_user: value("RemoteUser"),
                tty: value("TTY"),
                class: value("Class").unwrap_or_default(),
            })
        })
        .collect()
}

impl Sessions for Loginctl {
    fn sessions(&self) -> Result<Vec<LoginSession>> {
        if !runner::available("loginctl") {
            anyhow::bail!("loginctl is not installed");
        }
        let list = runner::run(
            Command::new("loginctl").args(["list-sessions", "--no-legend", "--no-pager"]),
            10,
            &self.stop,
        )?;
        if list.code != Some(0) {
            anyhow::bail!("loginctl list-sessions failed: {}", list.stderr.trim());
        }
        let ids: Vec<&str> = list
            .stdout
            .lines()
            .filter_map(|l| l.split_whitespace().next())
            .collect();
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let mut cmd = Command::new("loginctl");
        cmd.args(["show-session", "--no-pager"]).args(&ids);
        for p in SESSION_PROPERTIES {
            cmd.args(["-p", p]);
        }
        let out = runner::run(&mut cmd, 10, &self.stop)?;
        // A session can end between the two calls; loginctl then fails for that
        // id but still prints the others.
        if out.code != Some(0) && out.stdout.trim().is_empty() {
            anyhow::bail!("loginctl show-session failed: {}", out.stderr.trim());
        }
        Ok(parse_show_session(&out.stdout))
    }
}

/// Boot id and suspended time from the kernel.
pub struct KernelClock;

fn clock_secs(clock: libc::clockid_t) -> u64 {
    let mut t = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if unsafe { libc::clock_gettime(clock, &mut t) } != 0 {
        return 0;
    }
    t.tv_sec.max(0) as u64
}

impl Clock for KernelClock {
    fn boot_id(&self) -> Option<String> {
        fs::read_to_string("/proc/sys/kernel/random/boot_id")
            .ok()
            .map(|s| s.trim().to_string())
    }
    fn asleep_secs(&self) -> u64 {
        // CLOCK_BOOTTIME counts suspend; CLOCK_MONOTONIC does not.
        clock_secs(libc::CLOCK_BOOTTIME).saturating_sub(clock_secs(libc::CLOCK_MONOTONIC))
    }
}

/// Desktop notifications through notify-send, when it is installed.
pub struct NotifySend;

impl Notifier for NotifySend {
    fn notify(&self, summary: &str, body: &str) {
        crate::checks::canary::notify(summary, body);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_proc_net_addresses() {
        assert_eq!(
            parse_proc_addr("0100007F:698A").unwrap().to_string(),
            "127.0.0.1:27018"
        );
        assert_eq!(
            parse_proc_addr("00000000:1F90").unwrap().to_string(),
            "0.0.0.0:8080"
        );
        assert_eq!(
            parse_proc_addr("00000000000000000000000001000000:0035")
                .unwrap()
                .to_string(),
            "[::1]:53"
        );
        let table = "  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode\n   0: 00000000:1F90 00000000:0000 0A 00000000:00000000 00:00000000 00000000  1000        0 4242 1 0 100 0 0 10 0\n";
        let rows = parse_proc_net(table, Protocol::Tcp);
        assert_eq!(rows.len(), 1);
        assert!(rows[0].0.listening() && rows[0].0.remote.is_none() && rows[0].1 == 4242);
    }

    #[test]
    fn reads_package_logs() {
        let pacman = "[2026-10-29T14:02:00+0000] [PACMAN] Running 'pacman -S ripgrep'\n[2026-10-29T14:02:01+0000] [ALPM] transaction started\n[2026-10-29T14:02:02+0000] [ALPM] installed ripgrep (14.1.1-1)\n[2026-10-29T14:02:02+0000] [ALPM] upgraded pcre2 (10.43-1 -> 10.44-1)\n[2026-10-29T14:02:03+0000] [ALPM] transaction completed\n";
        let t = parse_pacman_log(pacman);
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].command, "pacman -S ripgrep");
        assert_eq!(t[0].changes[1].version, "10.44-1");
        assert_eq!(t[0].at, 1_793_282_521);
        let dpkg = "2026-10-29 14:02:00 startup archives unpack\n2026-10-29 14:02:01 install curl:amd64 <none> 8.5.0-2\n2026-10-29 14:02:02 status installed curl:amd64 8.5.0-2\n";
        let t = parse_dpkg_log(dpkg);
        assert_eq!(
            t[0].changes,
            vec![PackageChange {
                name: "curl".into(),
                version: "8.5.0-2".into(),
                kind: ChangeKind::Install
            }]
        );
    }

    #[test]
    fn keeps_only_denials_from_the_journal() {
        let out = r#"{"__CURSOR":"s=1","__REALTIME_TIMESTAMP":"1793300000000000","MESSAGE":"BPF prog-id=1 op=LOAD"}
{"__CURSOR":"s=2","__REALTIME_TIMESTAMP":"1793300001000000","MESSAGE":"avc:  denied  { read } for  pid=1 comm=\"firefox\" name=\".ssh\" scontext=a:b:mozilla_t:s0 tcontext=a:b:ssh_home_t:s0 tclass=dir permissive=0"}"#;
        let (records, cursor) = parse_journal_json(out);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].at, 1_793_300_001);
        assert_eq!(cursor.as_deref(), Some("s=2"));
    }
}
