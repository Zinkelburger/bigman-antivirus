//! Stop one of your programs and watch what starts it again.
//!
//! Bigman never stops a program on its own. `Action::StopAndWatch` asks one
//! process, chosen by you after a preview, to stop with SIGTERM (what closing it
//! from a task manager sends; never SIGKILL). It then looks at the process list
//! ten times a second for a minute, and when a new process runs the same
//! program it records what started it: the parent, the parent's parent, and the
//! service-manager unit each runs in. A program started again by another
//! program within seconds is how some unwanted software keeps itself running.
//!
//! `Action::StartAgain` starts the stopped program again the way it ran: the same
//! command line, folder and environment, or its service when the service
//! manager ran it.
//!
//! `assess` turns a finished watch into a report and is pure, so fixtures drive it.
use crate::{
    model::{clean, now, Report, Severity, SkipReason},
    runner::{self, Cancel},
    store::{private_dir, Store},
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    os::unix::fs::MetadataExt,
    path::Path,
    process::{Command, Stdio},
    sync::atomic::Ordering,
    time::{Duration, Instant},
};
use ts_rs::TS;

pub const KIND: &str = "Stop and watch";
/// How long Bigman watches after asking the program to stop.
pub const WATCH_SECONDS: u64 = 60;
const POLL: Duration = Duration::from_millis(100);
/// Restarts recorded at most; a program caught in a restart loop says so.
const MAX_RESTARTS: usize = 10;
/// How many ancestors of a restarted process are recorded.
const MAX_CHAIN: usize = 8;
/// How long what is needed to start a stopped program again is kept.
const KEEP_RECORD: u64 = 7 * 86_400;
const RECORDS: &str = "stopped";

/// Programs whose stopping would end or break the desktop session.
const SESSION: &[&str] = &[
    "kwin_wayland",
    "kwin_wayland_wrapper",
    "kwin_x11",
    "plasmashell",
    "ksmserver",
    "plasma_session",
    "startplasma-wayland",
    "startplasma-x11",
    "gnome-shell",
    "gnome-session-binary",
    "gnome-session-ctl",
    "gnome-session-service",
    "mutter",
    "Xwayland",
    "Xorg",
    "dbus-broker",
    "dbus-broker-launch",
    "dbus-daemon",
    "gdm-wayland-session",
    "gdm-x-session",
    "sddm-helper",
    "pipewire",
    "wireplumber",
    "(sd-pam)",
];

/// Variables the service manager sets for one run of a unit; the new run gets its own.
const UNIT_ENV: &[&str] = &[
    "INVOCATION_ID",
    "JOURNAL_STREAM",
    "MANAGERPID",
    "MANAGERPIDFDID",
    "SYSTEMD_EXEC_PID",
    "LISTEN_FDS",
    "LISTEN_PID",
    "LISTEN_FDNAMES",
    "NOTIFY_SOCKET",
    "WATCHDOG_PID",
    "WATCHDOG_USEC",
    "MEMORY_PRESSURE_WATCH",
    "MEMORY_PRESSURE_WRITE",
];

/// Bigman's own programs.
const BIGMAN: &[&str] = &["bigman", "bigmand", "bigman-app", "bigman-native-host"];

/// Interpreters: the program is the script, so a match needs the same command line.
fn interpreter(exe: &str) -> bool {
    let name = exe.rsplit('/').next().unwrap_or(exe);
    let base = name.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.');
    matches!(
        base,
        "python"
            | "node"
            | "nodejs"
            | "perl"
            | "ruby"
            | "bash"
            | "sh"
            | "dash"
            | "zsh"
            | "fish"
            | "java"
            | "bun"
            | "deno"
            | "lua"
            | "luajit"
            | "php"
            | "electron"
            | "env"
    )
}

/// One of your programs, as listed for choosing which one to stop.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct Program {
    pub pid: u32,
    /// When it started, in clock ticks after boot, so a reused process ID is
    /// never taken for it.
    pub start: u64,
    pub name: String,
    /// The program file, when it can be read.
    pub exe: Option<String>,
    pub command: String,
    pub parent_pid: u32,
    pub parent: String,
    /// The service-manager unit it runs in: an app's scope or a service.
    pub unit: Option<String>,
    /// When it started, in seconds since 1970.
    pub started: u64,
    /// Why Bigman will not stop it, when it will not.
    pub refused: Option<String>,
}

/// One process read from /proc.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Proc {
    pid: u32,
    ppid: u32,
    start: u64,
    uid: u32,
    comm: String,
    exe: Option<String>,
    argv: Vec<String>,
    unit: Option<String>,
    /// Whether the unit belongs to your service manager rather than the system's.
    user_unit: bool,
}

impl Proc {
    fn program(&self) -> Option<&str> {
        self.exe
            .as_deref()
            .map(|e| e.strip_suffix(" (deleted)").unwrap_or(e))
    }
    fn command(&self) -> String {
        if self.argv.is_empty() {
            format!("[{}]", self.comm)
        } else {
            self.argv.join(" ")
        }
    }
    /// Your service manager, or the system's (process 1).
    fn manager(&self) -> bool {
        self.pid == 1
            || (self.program().is_some_and(|e| e.ends_with("/systemd"))
                && self.argv.iter().any(|a| a == "--user"))
    }
}

/// The name, whether it has exited, the parent and the start time (field 22)
/// from /proc/PID/stat. The name is in parentheses and may hold spaces and
/// parentheses itself.
pub fn parse_stat(stat: &str) -> Option<(String, bool, u32, u64)> {
    let open = stat.find('(')?;
    let close = stat.rfind(')')?;
    let comm = stat.get(open + 1..close)?.to_string();
    let rest: Vec<&str> = stat.get(close + 1..)?.split_whitespace().collect();
    // rest[0] is the state (field 3); the parent is field 4, the start field 22.
    // A zombie (Z) or dead (X) process has exited and waits to be collected.
    let exited = matches!(*rest.first()?, "Z" | "X");
    let ppid = rest.get(1)?.parse().ok()?;
    let start = rest.get(19)?.parse().ok()?;
    Some((comm, exited, ppid, start))
}

/// The unit a process runs in, from its cgroup v2 path, and whether that unit
/// is under your service manager.
pub fn parse_cgroup(text: &str) -> (Option<String>, bool) {
    let Some(path) = text.lines().find_map(|l| l.strip_prefix("0::")) else {
        return (None, false);
    };
    let user = path.contains("/user@");
    let unit = path
        .rsplit('/')
        .find(|c| c.ends_with(".service") || c.ends_with(".scope"))
        .map(str::to_string);
    (unit, user)
}

fn read_proc(pid: u32) -> Option<Proc> {
    let dir = format!("/proc/{pid}");
    let (comm, exited, ppid, start) = parse_stat(&fs::read_to_string(format!("{dir}/stat")).ok()?)?;
    if exited {
        return None;
    }
    let uid = fs::metadata(&dir).ok()?.uid();
    let exe = fs::read_link(format!("{dir}/exe"))
        .ok()
        .map(|p| p.to_string_lossy().into_owned());
    let argv = fs::read(format!("{dir}/cmdline"))
        .map(|b| {
            b.split(|&c| c == 0)
                .filter(|a| !a.is_empty())
                .map(|a| clean(&String::from_utf8_lossy(a)))
                .collect()
        })
        .unwrap_or_default();
    let (unit, user_unit) = fs::read_to_string(format!("{dir}/cgroup"))
        .map(|t| parse_cgroup(&t))
        .unwrap_or((None, false));
    Some(Proc {
        pid,
        ppid,
        start,
        uid,
        comm: clean(&comm),
        exe,
        argv,
        unit,
        user_unit,
    })
}

fn pids() -> Vec<u32> {
    fs::read_dir("/proc")
        .map(|d| {
            d.flatten()
                .filter_map(|e| e.file_name().to_str()?.parse().ok())
                .collect()
        })
        .unwrap_or_default()
}

fn me() -> u32 {
    unsafe { libc::getuid() }
}

/// A kernel thread has no command line and descends from kthreadd (process 2).
fn kernel_thread(p: &Proc) -> bool {
    p.pid == 2 || p.ppid == 2 || (p.argv.is_empty() && p.exe.is_none())
}

/// Process IDs from this process up to process 1.
fn own_ancestry() -> BTreeSet<u32> {
    let mut out = BTreeSet::new();
    let mut pid = std::process::id();
    while pid > 1 && out.insert(pid) {
        match read_proc(pid) {
            Some(p) => pid = p.ppid,
            None => break,
        }
    }
    out
}

fn refusal(p: &Proc, ancestry: &BTreeSet<u32>) -> Option<String> {
    let name = p
        .program()
        .map(|e| e.rsplit('/').next().unwrap_or(e))
        .unwrap_or(&p.comm);
    if p.uid != me() {
        return Some("It runs as another account; Bigman runs as you.".into());
    }
    if BIGMAN.contains(&name) {
        return Some("Part of Bigman. Stop Bigman from Setup.".into());
    }
    if ancestry.contains(&p.pid) {
        return Some("Bigman itself runs inside it.".into());
    }
    if p.manager() {
        return Some("Your service manager; stopping it ends your session.".into());
    }
    if SESSION.contains(&name) || SESSION.contains(&p.comm.as_str()) {
        return Some("Your desktop session needs it.".into());
    }
    None
}

fn clock_ticks() -> u64 {
    match unsafe { libc::sysconf(libc::_SC_CLK_TCK) } {
        t if t > 0 => t as u64,
        _ => 100,
    }
}

fn boot_time() -> u64 {
    fs::read_to_string("/proc/stat")
        .ok()
        .and_then(|s| {
            s.lines()
                .find_map(|l| l.strip_prefix("btime ")?.trim().parse().ok())
        })
        .unwrap_or(0)
}

fn as_program(
    p: &Proc,
    parents: &BTreeMap<u32, String>,
    ancestry: &BTreeSet<u32>,
    boot: u64,
    ticks: u64,
) -> Program {
    Program {
        pid: p.pid,
        start: p.start,
        name: p.comm.clone(),
        exe: p.program().map(str::to_string),
        command: p.command(),
        parent_pid: p.ppid,
        parent: parents.get(&p.ppid).cloned().unwrap_or_default(),
        unit: p.unit.clone(),
        started: boot + p.start / ticks,
        refused: refusal(p, ancestry),
    }
}

/// Your programs, by name, each with whether Bigman will stop it.
pub fn programs() -> Vec<Program> {
    let all: Vec<Proc> = pids().into_iter().filter_map(read_proc).collect();
    let parents: BTreeMap<u32, String> = all.iter().map(|p| (p.pid, p.comm.clone())).collect();
    let ancestry = own_ancestry();
    let (boot, ticks) = (boot_time(), clock_ticks());
    let mut out: Vec<Program> = all
        .iter()
        .filter(|p| p.uid == me() && !kernel_thread(p))
        .map(|p| as_program(p, &parents, &ancestry, boot, ticks))
        .collect();
    out.sort_by(|a, b| {
        a.name
            .to_lowercase()
            .cmp(&b.name.to_lowercase())
            .then(a.pid.cmp(&b.pid))
    });
    out
}

/// The process the preview showed, still running with the same start time.
fn target(pid: u32, start: u64) -> Result<(Proc, Program)> {
    let p = read_proc(pid)
        .filter(|p| p.start == start)
        .context("That program is no longer running, or its process ID now belongs to another program. Choose it again from the list.")?;
    let parents: BTreeMap<u32, String> = read_proc(p.ppid)
        .map(|q| (q.pid, q.comm))
        .into_iter()
        .collect();
    let program = as_program(&p, &parents, &own_ancestry(), boot_time(), clock_ticks());
    if let Some(why) = &program.refused {
        bail!("{why}");
    }
    Ok((p, program))
}

/// One process up the chain from a restarted process.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Link {
    pub pid: u32,
    pub name: String,
    pub exe: Option<String>,
    pub command: String,
    pub unit: Option<String>,
    /// Your service manager, or the system's.
    pub manager: bool,
    /// Runs as someone else.
    pub other_account: bool,
}

/// A new process running the stopped program.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Restart {
    pub pid: u32,
    /// Milliseconds after the program was asked to stop.
    pub after_ms: u64,
    pub command: String,
    pub unit: Option<String>,
    pub user_unit: bool,
    /// Its parent first, then the parent's parent, up to the service manager.
    pub chain: Vec<Link>,
    /// Still running when the watch ended.
    pub running: bool,
}

/// What was stopped and what the watch saw.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Watch {
    pub pid: u32,
    pub name: String,
    pub exe: Option<String>,
    pub command: String,
    pub unit: Option<String>,
    pub user_unit: bool,
    /// Who started it originally.
    pub parent: String,
    /// Milliseconds after the request when it had gone; none when still running.
    pub stopped_ms: Option<u64>,
    pub watched_ms: u64,
    /// The watch ended early because the request was cancelled.
    pub cancelled: bool,
    pub restarts: Vec<Restart>,
    /// More restarts happened than were recorded.
    pub more: bool,
}

fn seconds(ms: u64) -> String {
    if ms < 10_000 {
        format!("{:.1} seconds", ms as f64 / 1000.0)
    } else {
        format!("{} seconds", ms / 1000)
    }
}

fn link_line(l: &Link) -> String {
    let what = l.exe.as_deref().unwrap_or(&l.command);
    let unit = l
        .unit
        .as_deref()
        .map(|u| format!(", in {u}"))
        .unwrap_or_default();
    let who = if l.other_account {
        ", another account"
    } else {
        ""
    };
    if l.manager {
        format!(
            "{} (process {}, the service manager{unit}{who})",
            l.name, l.pid
        )
    } else {
        format!("{} (process {}: {what}{unit}{who})", l.name, l.pid)
    }
}

fn unit_command(unit: &str, user: bool, verb: &str) -> String {
    if user {
        format!("systemctl --user {verb} {unit}")
    } else {
        format!("sudo systemctl {verb} {unit}")
    }
}

/// The report for a finished watch.
pub fn assess(w: &Watch) -> Report {
    let mut r = Report::new(KIND, format!("{} (process {})", w.name, w.pid));
    let program = w.exe.as_deref().unwrap_or(&w.command);
    let origin = format!(
        "Stopped: {} (process {})\nProgram: {program}\nCommand line: {}\nStarted by: {}{}",
        w.name,
        w.pid,
        w.command,
        if w.parent.is_empty() {
            "not readable"
        } else {
            &w.parent
        },
        w.unit
            .as_deref()
            .map(|u| format!("\nRan in: {u}"))
            .unwrap_or_default()
    );
    if w.stopped_ms.is_none() {
        r.add_command(
            "respawn.did-not-stop",
            Severity::Info,
            &format!("{} was still running {} after it was asked to stop", w.name, seconds(w.watched_ms)),
            &origin,
            "It may still be saving its work, or it may ignore the request. Close it from its own window, or end it without waiting with this command if you are sure. Check that the process ID is still the same program first.",
            &format!("kill -KILL {}", w.pid),
        );
    }
    for x in &w.restarts {
        let chain = x
            .chain
            .iter()
            .map(link_line)
            .collect::<Vec<_>>()
            .join("\n  started by ");
        let evidence = format!(
            "{origin}\n\nStarted again {} after the stop request: process {}\nCommand line: {}{}\nStarted by {chain}{}",
            seconds(x.after_ms),
            x.pid,
            x.command,
            x.unit.as_deref().map(|u| format!("\nRuns in: {u}")).unwrap_or_default(),
            if x.running { "\nStill running when the watch ended." } else { "\nIt had stopped again before the watch ended." }
        );
        let parent = x.chain.first();
        match parent {
            Some(p) if p.manager && x.unit.as_deref().is_some_and(|u| u.ends_with(".service")) => {
                let unit = x.unit.clone().unwrap_or_default();
                r.add_command(
                    "respawn.service-restart",
                    Severity::Info,
                    &format!("The service manager started {} again, as {unit}", w.name),
                    evidence,
                    "The service is set up to be restarted when it stops. To stop it until the next login, use this command; to keep it from starting at all, replace stop with disable --now. Protect › Startup files lists the unit file.",
                    &unit_command(&unit, x.user_unit, "stop"),
                );
            }
            Some(p) if p.manager => r.add(
                "respawn.orphan-restart",
                Severity::Review,
                &format!("{} was started again by a program that let go of it at once", w.name),
                format!(
                    "{evidence}\n\nIts parent is the service manager, which takes over a process when the program that started it exits straight away. {}",
                    x.unit.as_deref().map(|u| format!("It runs in {u}; whatever started it ran there too.")).unwrap_or_default()
                ),
                "Starting a program and letting go of it at once hides who started it. If you know what restarts it (an updater, a launcher, your own script), nothing needs doing. If not, look at what else runs in the same unit, and at Protect › Startup files.",
            ),
            Some(p) => {
                let same = p.exe.is_some() && p.exe == w.exe;
                r.add(
                    "respawn.program-restart",
                    Severity::Review,
                    &format!(
                        "{} was started again by {}, {} later",
                        w.name,
                        if same { format!("another copy of itself ({})", p.name) } else { p.name.clone() },
                        seconds(x.after_ms)
                    ),
                    evidence,
                    &format!(
                        "A program that starts another again as soon as it stops is how some unwanted software stays running, often as two copies that watch each other. Launchers, updaters and development tools do it too. If you recognise {}, nothing needs doing. If not, check its program file from Inspect and look at Protect › Startup files for what starts it.",
                        p.name
                    ),
                );
            }
            None => r.add(
                "respawn.orphan-restart",
                Severity::Review,
                &format!("{} was started again, and what started it could not be read", w.name),
                evidence,
                "Its parent had already exited, or runs as another account. Check again with Stop and watch, and look at Protect › Startup files.",
            ),
        }
    }
    if w.more {
        r.coverage.push(format!(
            "It was started again more than {MAX_RESTARTS} times in {}; the first {MAX_RESTARTS} are listed.",
            seconds(w.watched_ms)
        ));
    }
    if w.restarts.is_empty() && w.stopped_ms.is_some() {
        r.add(
            "respawn.not-restarted",
            Severity::Info,
            &format!("Nothing started {} again within {}", w.name, seconds(w.watched_ms)),
            &origin,
            "Something that starts it later is not seen here: at the next login, from a timer or a scheduled job. Protect › Startup files lists those.",
        );
    }
    if w.cancelled {
        r.coverage.skip(
            "The full minute of watching",
            SkipReason::Cancelled,
            format!(
                "The watch ended after {} because it was cancelled.",
                seconds(w.watched_ms)
            ),
        );
    } else {
        r.coverage.checked(format!(
            "The process list, ten times a second for {}",
            seconds(w.watched_ms)
        ));
    }
    r.coverage.push("Bigman sent the stop request (SIGTERM) because you asked it to; it never stops a program on its own and never forces one.".into());
    r.coverage.push("A program that is started and stops again within a tenth of a second can be missed. Programs that start it later, at the next login or on a timer, are not seen in this minute.".into());
    r.coverage.push("Who started a restart is read from its parent; a program started by another account shows that account's process by name only.".into());
    r.sha256 = None;
    r
}

/// What Bigman needs to start a stopped program again. It holds the program's
/// environment, so it is kept only for a week, in the private store, and deleted
/// once used.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Launch {
    pub name: String,
    pub exe: String,
    pub argv: Vec<String>,
    pub cwd: Option<String>,
    pub env: Vec<(String, String)>,
    pub unit: Option<String>,
    pub user_unit: bool,
    /// The service manager was its parent, so starting the unit starts it.
    pub service: bool,
    pub at: u64,
}

fn record_name(report: &str) -> Result<String> {
    if report.is_empty() || !report.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
        bail!("Invalid report ID");
    }
    Ok(format!("{RECORDS}/{report}.json"))
}

fn read_launch(p: &Proc) -> Option<Launch> {
    let dir = format!("/proc/{}", p.pid);
    // A program whose file was deleted cannot be started again.
    if p.exe.as_deref().is_some_and(|e| e.ends_with(" (deleted)")) {
        return None;
    }
    let exe = p.program()?.to_string();
    let env = fs::read(format!("{dir}/environ"))
        .map(|b| {
            b.split(|&c| c == 0)
                .filter_map(|kv| {
                    let kv = String::from_utf8_lossy(kv);
                    let (k, v) = kv.split_once('=')?;
                    (!k.is_empty() && !UNIT_ENV.contains(&k))
                        .then(|| (k.to_string(), v.to_string()))
                })
                .collect()
        })
        .unwrap_or_default();
    let cwd = fs::read_link(format!("{dir}/cwd"))
        .ok()
        .map(|c| c.to_string_lossy().into_owned());
    let service = read_proc(p.ppid).is_some_and(|q| q.manager())
        && p.unit.as_deref().is_some_and(|u| u.ends_with(".service"));
    Some(Launch {
        name: p.comm.clone(),
        exe,
        argv: p.argv.clone(),
        cwd,
        env,
        unit: p.unit.clone(),
        user_unit: p.user_unit,
        service,
        at: now(),
    })
}

/// Deletes records older than a week.
fn prune(store: &Store) {
    let Ok(dir) = fs::read_dir(store.root.join(RECORDS)) else {
        return;
    };
    for e in dir.flatten() {
        let old = e
            .metadata()
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.elapsed().ok())
            .is_some_and(|age| age.as_secs() > KEEP_RECORD);
        if old {
            let _ = fs::remove_file(e.path());
        }
    }
}

/// What the preview of a stop shows.
pub struct StopPlan {
    pub summary: String,
    /// A limit worth reading before confirming.
    pub note: String,
    /// Labelled facts about the program.
    pub facts: Vec<(String, String)>,
    /// The exact request sent.
    pub details: String,
}

/// What the preview of a stop shows: the plan's facts and the exact request.
pub fn stop_plan(pid: u32, start: u64) -> Result<StopPlan> {
    let (p, program) = target(pid, start)?;
    let mut facts = vec![
        (
            "Program".to_string(),
            program.exe.clone().unwrap_or_else(|| p.comm.clone()),
        ),
        ("Process ID".into(), pid.to_string()),
        ("Command line".into(), program.command.clone()),
        (
            "Started by".into(),
            format!(
                "{} (process {})",
                if program.parent.is_empty() {
                    "not readable"
                } else {
                    &program.parent
                },
                p.ppid
            ),
        ),
    ];
    if let Some(u) = &p.unit {
        facts.push(("Runs in".into(), u.clone()));
    }
    facts.push((
        "Running for".into(),
        crate::model::age(now().saturating_sub(program.started))
            .replace("0 minutes", "less than a minute"),
    ));
    let summary = format!(
        "Ask {} to stop, then watch for {WATCH_SECONDS} seconds for anything that starts it again. Bigman records what started it; the result appears under Stopped recently and in Activity.",
        p.comm
    );
    let mut note = "Unsaved work in it may be lost. Bigman sends the request a task manager sends when you end a program, and never forces it.".to_string();
    if p.exe.as_deref().is_some_and(|e| e.ends_with(" (deleted)")) {
        note.push_str(&format!(" Its program file was deleted, so Bigman cannot start it again afterwards. To keep a copy first: cp /proc/{pid}/exe ~/copy-of-process-{pid}"));
    }
    Ok(StopPlan {
        summary,
        note,
        facts,
        details: format!(
            "kill -TERM {pid}    # {}, started at clock tick {start}",
            p.comm
        ),
    })
}

fn pidfd_open(pid: u32) -> Option<i32> {
    let fd = unsafe { libc::syscall(libc::SYS_pidfd_open, pid as libc::pid_t, 0) };
    (fd >= 0).then_some(fd as i32)
}

/// Sends SIGTERM through a pidfd opened after the start time was checked, so a
/// process ID reused in between is never signalled.
fn terminate(pid: u32, start: u64) -> Result<()> {
    let fd = pidfd_open(pid).context("That program has already stopped")?;
    let result = (|| {
        if read_proc(pid).map(|p| p.start) != Some(start) {
            bail!("That program has already stopped");
        }
        let sent = unsafe {
            libc::syscall(
                libc::SYS_pidfd_send_signal,
                fd,
                libc::SIGTERM,
                std::ptr::null::<libc::siginfo_t>(),
                0,
            )
        };
        if sent != 0 {
            bail!(
                "The stop request was refused: {}",
                std::io::Error::last_os_error()
            );
        }
        Ok(())
    })();
    unsafe { libc::close(fd) };
    result
}

fn chain(from: u32) -> Vec<Link> {
    let mut out = vec![];
    let mut pid = from;
    while pid > 0 && out.len() < MAX_CHAIN {
        let Some(p) = read_proc(pid) else { break };
        let manager = p.manager();
        out.push(Link {
            pid,
            name: p.comm.clone(),
            exe: p.program().map(str::to_string),
            command: p.command(),
            unit: p.unit.clone(),
            manager,
            other_account: p.uid != me(),
        });
        if manager {
            break;
        }
        pid = p.ppid;
    }
    out
}

/// A new process is the stopped program again when it runs the same file as the
/// same account with the same command line, or, for a program that is not an
/// interpreter, in the same unit. An unrelated `sleep 5` elsewhere is not
/// `sleep 300` started again.
fn same_program(target: &Proc, p: &Proc) -> bool {
    let program = p.program();
    p.uid == target.uid
        && program.is_some()
        && program == target.program()
        && (p.argv == target.argv
            || (!interpreter(program.unwrap_or_default())
                && p.unit.is_some()
                && p.unit == target.unit))
}

/// A program that was asked to stop and is being watched.
pub struct Pending {
    p: Proc,
    start: u64,
    launch: Option<Launch>,
    before: BTreeSet<(u32, u64)>,
    began: Instant,
    w: Watch,
}

/// Asks the program to stop. What it was is read first, so it can be started
/// again and its restarts told apart from what was already running.
pub fn stop(pid: u32, start: u64) -> Result<Pending> {
    let (p, program) = target(pid, start)?;
    let launch = read_launch(&p);
    // Everything already running is not a restart, including the stopped
    // program's own children.
    let before: BTreeSet<(u32, u64)> = pids()
        .into_iter()
        .filter_map(|q| read_proc(q).map(|q| (q.pid, q.start)))
        .collect();
    terminate(pid, start)?;
    let w = Watch {
        pid,
        name: p.comm.clone(),
        exe: p.program().map(str::to_string),
        command: p.command(),
        unit: p.unit.clone(),
        user_unit: p.user_unit,
        parent: if program.parent.is_empty() {
            String::new()
        } else {
            format!("{} (process {})", program.parent, p.ppid)
        },
        ..Watch::default()
    };
    Ok(Pending {
        p,
        start,
        launch,
        before,
        began: Instant::now(),
        w,
    })
}

/// Stops the program and watches, in this thread. The report is saved, with
/// what is needed to start the program again beside it when nothing else did.
pub fn stop_and_watch(
    store: &Store,
    pid: u32,
    start: u64,
    stop_flag: &Cancel,
) -> Result<(Report, Watch)> {
    watch(store, stop(pid, start)?, stop_flag)
}

/// Watches a stopped program for a minute and saves the report.
pub fn watch(store: &Store, pending: Pending, stop: &Cancel) -> Result<(Report, Watch)> {
    let Pending {
        p,
        start,
        launch,
        mut before,
        began,
        mut w,
    } = pending;
    let pid = p.pid;
    let window = Duration::from_secs(WATCH_SECONDS);
    // Recorded restarts and their start times.
    let mut seen: BTreeMap<u32, u64> = BTreeMap::new();
    while began.elapsed() < window {
        if stop.load(Ordering::Relaxed) {
            w.cancelled = true;
            break;
        }
        let elapsed = || began.elapsed().as_millis() as u64;
        let gone = || read_proc(pid).map(|q| q.start) != Some(start);
        if w.stopped_ms.is_none() && gone() {
            w.stopped_ms = Some(elapsed());
        }
        for q in pids() {
            let Some(q) = read_proc(q) else { continue };
            if before.contains(&(q.pid, q.start)) {
                continue;
            }
            // A child keeps its parent's program until it starts its own, so a
            // new process that does not match yet is looked at again next time.
            if !same_program(&p, &q) {
                continue;
            }
            before.insert((q.pid, q.start));
            let found = elapsed();
            // Its stop can be seen only now, when the list took a while to read.
            if w.stopped_ms.is_none() && gone() {
                w.stopped_ms = Some(found);
            }
            let up = chain(q.ppid);
            // A restarted program's own children (an app's helper processes) are
            // part of that restart.
            if up.iter().any(|l| seen.contains_key(&l.pid)) {
                continue;
            }
            if w.restarts.len() >= MAX_RESTARTS {
                w.more = true;
                continue;
            }
            seen.insert(q.pid, q.start);
            w.restarts.push(Restart {
                pid: q.pid,
                after_ms: found,
                command: q.command(),
                unit: q.unit.clone(),
                user_unit: q.user_unit,
                chain: up,
                running: true,
            });
        }
        std::thread::sleep(POLL);
    }
    w.watched_ms = began.elapsed().as_millis() as u64;
    // A watch that ended early may not have looked since the program went.
    if w.stopped_ms.is_none() && read_proc(pid).map(|q| q.start) != Some(start) {
        w.stopped_ms = Some(w.watched_ms);
    }
    for x in &mut w.restarts {
        x.running = read_proc(x.pid).map(|q| q.start) == seen.get(&x.pid).copied();
    }
    let report = assess(&w);
    store.save(&report)?;
    private_dir(&store.root.join(RECORDS))?;
    prune(store);
    // Its environment is kept only when starting it again makes sense.
    if let Some(l) = launch.filter(|_| can_start_again(&w)) {
        store.write(&record_name(&report.id)?, &l)?;
    }
    Ok((report, w))
}

/// A watch running in the daemon.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct Watching {
    pub pid: u32,
    pub name: String,
    /// When it was asked to stop, in seconds since 1970.
    pub started: u64,
    /// When the watch ends.
    pub ends: u64,
}

/// A stopped program that can be started again.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct Startable {
    /// The Stop and watch report.
    pub report: String,
    pub name: String,
    /// Copies of it running now, by process ID.
    pub running: Vec<u32>,
}

/// Watches in progress and stopped programs that can be started again.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct Stops {
    pub watching: Vec<Watching>,
    pub startable: Vec<Startable>,
}

static WATCHING: std::sync::Mutex<Vec<Watching>> = std::sync::Mutex::new(Vec::new());

fn watching() -> std::sync::MutexGuard<'static, Vec<Watching>> {
    WATCHING.lock().unwrap_or_else(|e| e.into_inner())
}

/// Takes a watch off the list when its thread ends, however it ends.
struct Listed(u32);

impl Drop for Listed {
    fn drop(&mut self) {
        watching().retain(|w| w.pid != self.0);
    }
}

/// Stops the program now and watches in a thread of its own, so nobody waits
/// the minute. The report lands in history, and so in Activity.
pub fn stop_in_background(store: &Store, pid: u32, start: u64) -> Result<String> {
    let pending = stop(pid, start)?;
    let name = pending.w.name.clone();
    let at = now();
    watching().push(Watching {
        pid,
        name: name.clone(),
        started: at,
        ends: at + WATCH_SECONDS,
    });
    let store = store.clone();
    std::thread::Builder::new()
        .name(format!("watch-{pid}"))
        .spawn(move || {
            let _listed = Listed(pid);
            if let Err(e) = watch(&store, pending, &runner::cancel()) {
                eprintln!("bigmand: the watch of process {pid} did not finish: {e:#}");
            }
        })
        .context("Cannot start the watch")?;
    Ok(format!(
        "{name} was asked to stop. Bigman watches for {WATCH_SECONDS} seconds; the result appears under Stopped recently and in Activity."
    ))
}

/// What the app shows beside recent stops.
pub fn stops(store: &Store) -> Stops {
    let mut startable = vec![];
    if let Ok(dir) = fs::read_dir(store.root.join(RECORDS)) {
        for e in dir.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            let Some(report) = name.strip_suffix(".json") else {
                continue;
            };
            let Ok(l) = launch(store, report) else {
                continue;
            };
            // Listed only when starting it again would work.
            if start_plan(store, report).is_err() {
                continue;
            }
            startable.push(Startable {
                report: report.to_string(),
                name: l.name.clone(),
                running: running_copies(&l),
            });
        }
    }
    Stops {
        watching: watching().clone(),
        startable,
    }
}

/// One line for the result: what happened, in order.
pub fn outcome(w: &Watch) -> String {
    let stopped = match w.stopped_ms {
        Some(ms) => format!("{} stopped after {}.", w.name, seconds(ms)),
        None => format!("{} was still running when the watch ended.", w.name),
    };
    let restarted = match w.restarts.first() {
        None if w.stopped_ms.is_some() => format!(
            " Nothing started it again within {}.",
            seconds(w.watched_ms)
        ),
        None => String::new(),
        Some(x) => format!(
            " It was started again after {} by {}.",
            seconds(x.after_ms),
            x.chain
                .first()
                .map_or("a program Bigman could not read".to_string(), |l| {
                    if l.manager {
                        "the service manager".into()
                    } else {
                        l.name.clone()
                    }
                })
        ),
    };
    format!("{stopped}{restarted}")
}

/// Whether "Start it again" makes sense after this watch: it stopped and nothing
/// that started it again is still running.
pub fn can_start_again(w: &Watch) -> bool {
    w.stopped_ms.is_some() && !w.restarts.iter().any(|x| x.running)
}

fn launch(store: &Store, report: &str) -> Result<Launch> {
    let name = record_name(report)?;
    if !store.root.join(&name).exists() {
        bail!("Bigman no longer has what it needs to start this program again. It keeps that for a week after a stop, and only until the program is started again.");
    }
    store.read(&name)
}

/// The command that starts it again, as shown in the preview. Environment
/// values are not shown: they can hold secrets.
fn start_command(l: &Launch) -> Vec<String> {
    if l.service {
        let unit = l.unit.clone().unwrap_or_default();
        return if l.user_unit {
            vec!["systemctl".into(), "--user".into(), "start".into(), unit]
        } else {
            vec!["systemctl".into(), "start".into(), unit]
        };
    }
    let mut c: Vec<String> = vec![
        "systemd-run".into(),
        "--user".into(),
        "--collect".into(),
        "--quiet".into(),
    ];
    if let Some(d) = &l.cwd {
        c.push(format!("--working-directory={d}"));
    }
    c.push("--".into());
    c.push(l.exe.clone());
    c.extend(l.argv.iter().skip(1).cloned());
    c
}

fn shell_word(w: &str) -> String {
    if !w.is_empty()
        && w.chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_./=:,+@%".contains(c))
    {
        w.to_string()
    } else {
        format!("'{}'", w.replace('\'', "'\\''"))
    }
}

/// Copies of the program running now.
fn running_copies(l: &Launch) -> Vec<u32> {
    let shape = Proc {
        uid: me(),
        exe: Some(l.exe.clone()),
        argv: l.argv.clone(),
        ..Proc::default()
    };
    pids()
        .into_iter()
        .filter_map(read_proc)
        .filter(|q| same_program(&shape, q))
        .map(|q| q.pid)
        .collect()
}

/// What the preview of "Start it again" shows.
pub fn start_plan(store: &Store, report: &str) -> Result<(String, String, Option<String>)> {
    let l = launch(store, report)?;
    if !l.service && !Path::new(&l.exe).exists() {
        bail!(
            "Its program file, {}, is gone, so it cannot be started again.",
            l.exe
        );
    }
    if l.service && !l.user_unit {
        bail!(
            "It ran as a system service. Start it with: sudo systemctl start {}",
            l.unit.unwrap_or_default()
        );
    }
    let summary = if l.service {
        format!(
            "Start {} again through your service manager, as {}.",
            l.name,
            l.unit.clone().unwrap_or_default()
        )
    } else {
        format!(
            "Start {} again with the same command line, folder and {} environment variables it had. It runs on its own, not inside Bigman.",
            l.name,
            l.env.len()
        )
    };
    let copies = running_copies(&l);
    let note = (!copies.is_empty()).then(|| {
        format!(
            "It is running again already (process {}). Starting it again makes another copy.",
            copies
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )
    });
    let details = start_command(&l)
        .iter()
        .map(|w| shell_word(w))
        .collect::<Vec<_>>()
        .join(" ");
    Ok((summary, details, note))
}

/// Starts the stopped program again and deletes the record.
pub fn start_again(store: &Store, report: &str) -> Result<String> {
    let l = launch(store, report)?;
    start_plan(store, report)?;
    let command = start_command(&l);
    let mut c = Command::new(&command[0]);
    c.args(&command[1..]).stdin(Stdio::null());
    if !l.service {
        if !runner::available("systemd-run") {
            // Without a service manager it runs as a detached child.
            let mut direct = Command::new(&l.exe);
            direct
                .args(l.argv.iter().skip(1))
                .env_clear()
                .envs(l.env.iter().cloned());
            if let Some(d) = &l.cwd {
                direct.current_dir(d);
            }
            runner::spawn_detached(&mut direct)?;
            let _ = fs::remove_file(store.path(&record_name(report)?)?);
            return Ok(format!("{} is starting again.", l.name));
        }
        // systemd-run takes the environment as --setenv options, before "--".
        let at = command
            .iter()
            .position(|w| w == "--")
            .unwrap_or(command.len());
        let mut args: Vec<String> = command[1..at].to_vec();
        for (k, v) in &l.env {
            args.push(format!("--setenv={k}={v}"));
        }
        args.extend(command[at..].iter().cloned());
        c = Command::new(&command[0]);
        c.args(&args).stdin(Stdio::null());
    }
    let out = c
        .output()
        .with_context(|| format!("Cannot run {}", command[0]))?;
    if !out.status.success() {
        bail!(
            "It did not start: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    let _ = fs::remove_file(store.path(&record_name(report)?)?);
    Ok(format!("{} is starting again.", l.name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stat_names_with_spaces_and_parentheses_are_read_whole() {
        let stat = "4242 (my (odd) prog) S 1200 4242 4242 0 -1 4194560 100 0 0 0 1 2 0 0 20 0 3 0 987654 1000 200";
        assert_eq!(
            parse_stat(stat),
            Some(("my (odd) prog".into(), false, 1200, 987654))
        );
        assert_eq!(
            parse_stat(&stat.replace(") S ", ") Z ")).map(|s| s.1),
            Some(true)
        );
        assert_eq!(parse_stat("garbage"), None);
    }

    #[test]
    fn the_unit_is_the_innermost_service_or_scope() {
        let app = "0::/user.slice/user-1000.slice/user@1000.service/app.slice/app-org.kde.konsole-1234.scope\n";
        assert_eq!(
            parse_cgroup(app),
            (Some("app-org.kde.konsole-1234.scope".into()), true)
        );
        let sys = "0::/system.slice/sshd.service\n";
        assert_eq!(parse_cgroup(sys), (Some("sshd.service".into()), false));
        assert_eq!(parse_cgroup("1:name=systemd:/\n"), (None, false));
    }

    #[test]
    fn an_interpreter_matches_only_with_the_same_script() {
        let target = Proc {
            uid: 1000,
            exe: Some("/usr/bin/python3.13".into()),
            argv: vec!["python3".into(), "a.py".into()],
            ..Proc::default()
        };
        let other = Proc {
            argv: vec!["python3".into(), "b.py".into()],
            ..target.clone()
        };
        assert!(!same_program(&target, &other));
        assert!(same_program(&target, &target.clone()));
        let bin = Proc {
            uid: 1000,
            exe: Some("/home/u/.local/bin/miner".into()),
            argv: vec!["miner".into()],
            unit: Some("app-m.scope".into()),
            ..Proc::default()
        };
        let flags = Proc {
            argv: vec!["miner".into(), "--quiet".into()],
            ..bin.clone()
        };
        assert!(same_program(&bin, &flags), "other flags in the same unit");
        let elsewhere = Proc {
            unit: Some("app-other.scope".into()),
            ..flags.clone()
        };
        assert!(
            !same_program(&bin, &elsewhere),
            "other flags in another unit"
        );
        let anywhere = Proc {
            unit: None,
            ..bin.clone()
        };
        assert!(
            same_program(&bin, &anywhere),
            "the same command line anywhere"
        );
        let deleted = Proc {
            exe: Some("/home/u/.local/bin/miner (deleted)".into()),
            ..bin.clone()
        };
        assert!(same_program(&bin, &deleted));
        assert!(!same_program(
            &bin,
            &Proc {
                uid: 0,
                ..bin.clone()
            }
        ));
    }

    #[test]
    fn the_session_and_bigman_itself_are_refused() {
        let ancestry = BTreeSet::from([77]);
        let base = Proc {
            pid: 500,
            ppid: 400,
            uid: me(),
            comm: "worker".into(),
            exe: Some("/usr/bin/worker".into()),
            argv: vec!["worker".into()],
            ..Proc::default()
        };
        assert_eq!(refusal(&base, &ancestry), None);
        let kwin = Proc {
            comm: "kwin_wayland".into(),
            exe: Some("/usr/bin/kwin_wayland".into()),
            ..base.clone()
        };
        assert!(refusal(&kwin, &ancestry)
            .unwrap()
            .contains("desktop session"));
        let daemon = Proc {
            exe: Some("/home/u/.local/bin/bigmand".into()),
            ..base.clone()
        };
        assert!(refusal(&daemon, &ancestry)
            .unwrap()
            .contains("Part of Bigman"));
        let shell = Proc {
            pid: 77,
            ..base.clone()
        };
        assert!(refusal(&shell, &ancestry)
            .unwrap()
            .contains("runs inside it"));
        let manager = Proc {
            exe: Some("/usr/lib/systemd/systemd".into()),
            argv: vec!["/usr/lib/systemd/systemd".into(), "--user".into()],
            ..base.clone()
        };
        assert!(refusal(&manager, &ancestry)
            .unwrap()
            .contains("service manager"));
        let other = Proc {
            uid: me() + 1,
            ..base.clone()
        };
        assert!(refusal(&other, &ancestry)
            .unwrap()
            .contains("another account"));
        // A program running from a deleted file can be stopped; it just cannot
        // be started again.
        let gone = Proc {
            exe: Some("/tmp/x (deleted)".into()),
            ..base
        };
        assert_eq!(refusal(&gone, &ancestry), None);
        assert_eq!(read_launch(&gone), None);
    }

    fn watch() -> Watch {
        Watch {
            pid: 4242,
            name: "miner".into(),
            exe: Some("/home/u/.local/bin/miner".into()),
            command: "miner --quiet".into(),
            parent: "systemd (process 1200)".into(),
            stopped_ms: Some(300),
            watched_ms: 60_000,
            ..Watch::default()
        }
    }

    fn rules(r: &Report) -> Vec<&str> {
        r.findings.iter().map(|f| f.rule.as_str()).collect()
    }

    #[test]
    fn the_report_names_who_started_it_again() {
        assert_eq!(rules(&assess(&watch())), ["respawn.not-restarted"]);
        let mut w = watch();
        w.restarts.push(Restart {
            pid: 5000,
            after_ms: 1200,
            command: "miner --quiet".into(),
            chain: vec![Link {
                pid: 4300,
                name: "helper".into(),
                exe: Some("/home/u/.cache/helper".into()),
                ..Link::default()
            }],
            running: true,
            ..Restart::default()
        });
        let r = assess(&w);
        assert_eq!(rules(&r), ["respawn.program-restart"]);
        assert_eq!(
            r.findings[0].summary,
            "miner was started again by helper, 1.2 seconds later"
        );
        assert!(r.findings[0]
            .evidence
            .contains("Started by helper (process 4300: /home/u/.cache/helper)"));
        assert!(!can_start_again(&w));
        assert_eq!(
            outcome(&w),
            "miner stopped after 0.3 seconds. It was started again after 1.2 seconds by helper."
        );

        w.restarts[0].chain[0].exe = w.exe.clone();
        assert!(assess(&w).findings[0]
            .summary
            .contains("another copy of itself"));

        w.restarts[0].chain = vec![Link {
            pid: 1200,
            name: "systemd".into(),
            manager: true,
            ..Link::default()
        }];
        w.restarts[0].unit = Some("miner.service".into());
        w.restarts[0].user_unit = true;
        let r = assess(&w);
        assert_eq!(rules(&r), ["respawn.service-restart"]);
        assert_eq!(
            r.findings[0].command.as_deref(),
            Some("systemctl --user stop miner.service")
        );

        w.restarts[0].unit = Some("app-x.scope".into());
        assert_eq!(rules(&assess(&w)), ["respawn.orphan-restart"]);

        let mut stuck = watch();
        stuck.stopped_ms = None;
        assert_eq!(rules(&assess(&stuck)), ["respawn.did-not-stop"]);
        assert!(!can_start_again(&stuck));
    }

    #[test]
    fn a_service_starts_through_its_unit_and_a_program_through_systemd_run() {
        let mut l = Launch {
            name: "miner".into(),
            exe: "/opt/m/miner".into(),
            argv: vec!["miner".into(), "--pool".into(), "a b".into()],
            cwd: Some("/home/u".into()),
            ..Launch::default()
        };
        assert_eq!(
            start_command(&l).iter().map(|w| shell_word(w)).collect::<Vec<_>>().join(" "),
            "systemd-run --user --collect --quiet --working-directory=/home/u -- /opt/m/miner --pool 'a b'"
        );
        l.service = true;
        l.user_unit = true;
        l.unit = Some("miner.service".into());
        assert_eq!(
            start_command(&l).join(" "),
            "systemctl --user start miner.service"
        );
    }

    #[test]
    fn report_ids_cannot_leave_the_records_folder() {
        assert!(record_name("../known").is_err());
        assert!(record_name("").is_err());
        assert_eq!(record_name("1f2e-3d").unwrap(), "stopped/1f2e-3d.json");
    }

    #[test]
    fn a_real_child_is_stopped_and_not_restarted() {
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        let mut child = Command::new("sleep").arg("300").spawn().unwrap();
        let pid = child.id();
        let start = read_proc(pid).unwrap().start;
        assert!(
            stop_and_watch(&store, pid, start + 1, &runner::cancel()).is_err(),
            "a different start time is refused"
        );
        let stop = runner::cancel();
        let flag = stop.clone();
        // End the watch early: reaping the child makes its stop visible first.
        let reaper = std::thread::spawn(move || {
            let status = child.wait().unwrap();
            std::thread::sleep(Duration::from_millis(400));
            flag.store(true, Ordering::Relaxed);
            status
        });
        let (report, w) = stop_and_watch(&store, pid, start, &stop).unwrap();
        assert!(!reaper.join().unwrap().success());
        assert!(w.stopped_ms.is_some());
        assert!(w.cancelled);
        assert!(w.restarts.is_empty());
        assert_eq!(rules(&report), ["respawn.not-restarted"]);
        assert!(store.root.join(record_name(&report.id).unwrap()).exists());
        let (summary, details, _) = start_plan(&store, &report.id).unwrap();
        assert!(summary.starts_with("Start sleep again"));
        assert!(
            details.ends_with("-- /usr/bin/sleep 300") || details.ends_with("sleep 300"),
            "{details}"
        );
    }
}
