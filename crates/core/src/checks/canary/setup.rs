use crate::{
    checks::canary::{self, Health, ReadReceipt},
    model::{now, Report, Severity},
    runner::{self, Cancel},
    store::{home, read_file, Store, MAX_INPUT},
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::atomic::Ordering,
    time::{Duration, Instant},
};

pub fn default_path() -> PathBuf {
    let documents = home().join("Documents");
    if documents.is_dir() {
        documents
    } else {
        home()
    }
    .join("backup-access.env")
}
pub fn health(store: &Store) -> Option<Health> {
    store
        .read::<Health>("monitor-health.json")
        .ok()
        .filter(|h| now().saturating_sub(h.updated) < 10)
}

/// Whether bigmand should run the monitor. The monitor has no process or login
/// service of its own, so this is what survives a restart.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
enum Mode {
    #[default]
    Off,
    On,
    /// On until the computer restarts (`--session-only`).
    ThisBoot {
        boot: String,
    },
    /// Off until the computer restarts, then on (`stop --session-only`).
    PausedThisBoot {
        boot: String,
    },
}
impl Mode {
    fn wanted(&self, now: &str) -> bool {
        match self {
            Mode::Off => false,
            Mode::On => true,
            Mode::ThisBoot { boot } => boot == now,
            Mode::PausedThisBoot { boot } => boot != now,
        }
    }
    fn started(self, session_only: bool, boot: String) -> Mode {
        match (self, session_only) {
            (Mode::On | Mode::PausedThisBoot { .. }, true) | (_, false) => Mode::On,
            (_, true) => Mode::ThisBoot { boot },
        }
    }
    fn stopped(self, session_only: bool, boot: String) -> Mode {
        match (self, session_only) {
            (Mode::On | Mode::PausedThisBoot { .. }, true) => Mode::PausedThisBoot { boot },
            _ => Mode::Off,
        }
    }
}
const MODE: &str = "monitor-mode.json";
fn boot() -> String {
    fs::read_to_string("/proc/sys/kernel/random/boot_id")
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}
fn mode(store: &Store) -> Mode {
    store.read(MODE).unwrap_or_default()
}
/// Whether the monitor is switched on, which bigmand follows within a second.
pub fn wanted(store: &Store) -> bool {
    mode(store).wanted(&boot())
}
/// Switches the monitor on for good, for a login service from an earlier version.
pub fn keep_on(store: &Store) -> Result<()> {
    store.write(MODE, &Mode::On)
}
fn pause(stop: &Cancel) -> Result<()> {
    if stop.load(Ordering::Relaxed) {
        bail!("Cancelled; any created decoy and started monitor remain available");
    }
    std::thread::sleep(Duration::from_millis(100));
    Ok(())
}
/// Starts bigmand if it is not running; the monitor runs inside it.
fn launch(store: &Store) -> Result<()> {
    if crate::ipc::client::Client::connect().is_ok() {
        return Ok(());
    }
    let log = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(store.root.join("monitor.log"))?;
    let mut command = Command::new(runner::sibling_executable("bigmand")?);
    command
        .arg("--state")
        .arg(&store.root)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(log);
    // Detach from the invoking terminal so closing the CLI does not stop protection.
    runner::spawn_detached(&mut command)
}

pub fn verify(store: &Store, id: &str, stop: &Cancel) -> Result<Report> {
    let mut report = Report::new("Canary verification", id);
    if let Err(error) = verify_inner(store, id, stop, &mut report) {
        report.errors.push(error.to_string());
    }
    store.save(&report)?;
    Ok(report)
}
fn verify_inner(store: &Store, id: &str, stop: &Cancel, report: &mut Report) -> Result<()> {
    uuid::Uuid::parse_str(id)?;
    let canary = canary::list(store)?
        .into_iter()
        .find(|c| c.id == id)
        .context("Unknown canary ID")?;
    report.subject = canary.path.display().to_string();
    let started = Instant::now();
    let monitor = loop {
        if let Some(h) = health(store).filter(|h| h.watched_ids.iter().any(|i| i == id)) {
            break h;
        }
        if started.elapsed() > Duration::from_secs(8) {
            bail!("Monitor has not confirmed this path. Start monitoring and retry; see monitor.log and monitor health for errors");
        }
        pause(stop)?;
    };
    let receipt_path = format!("canaries/{id}.read.json");
    let previous = store.read::<ReadReceipt>(&receipt_path).ok().map(|r| r.id);
    canary::self_test(store, id)?;
    let started = Instant::now();
    loop {
        if let Ok(receipt) = store.read::<ReadReceipt>(&receipt_path) {
            if Some(&receipt.id) != previous.as_ref()
                && receipt.canary_id == id
                && receipt.monitor_pid == monitor.pid
            {
                report.add("canary.verified", Severity::Info, "Local read detection verified",
                    format!("Canary {id}\nKernel read receipt {} from monitor {}", receipt.id, receipt.monitor_pid),
                    "Leave monitoring running. Use History to review later alerts; normal applications can also read decoys.");
                report.coverage.push("A deliberate read was followed by a new receipt from the real filesystem monitor. This verifies local detection and durable evidence, not notification delivery, reader identity, or remote token use.".into());
                if !runner::available("notify-send") {
                    report.add("canary.notifications-unavailable", Severity::Info, "Alerts are available in History",
                        "notify-send is not installed", "Install your distribution's desktop notification utilities if you also want pop-up alerts.");
                }
                return Ok(());
            }
        }
        if started.elapsed() > Duration::from_secs(5) {
            bail!("Test read produced no new monitor receipt. Local detection is not verified");
        }
        pause(stop)?;
    }
}

pub fn arm(store: &Store, path: &Path, session_only: bool, stop: &Cancel) -> Result<Report> {
    let _lock = store.lock("canary-setup.lock")?;
    let mut report = Report::new("Local canary setup", path.display());
    let result = (|| -> Result<()> {
        let parent = fs::canonicalize(
            path.parent()
                .filter(|p| !p.as_os_str().is_empty())
                .unwrap_or(Path::new(".")),
        )?;
        let path = parent.join(path.file_name().context("Choose a decoy filename")?);
        let existing = canary::list(store)?.into_iter().find(|c| c.path == path);
        let token = if let Some(c) = existing {
            if c.kind != "local-decoy" {
                bail!("This path is a hosted shortcut; choose a new local decoy path");
            }
            // Validate the existing file without reading it before the monitor is ready.
            if !fs::symlink_metadata(&path)?.is_file() {
                bail!("Registered decoy is missing or replaced; choose a new path");
            }
            c
        } else {
            canary::create(store, &path, None)?
        };
        report.details = format!("Canary ID: {}\nPath: {}", token.id, token.path.display());
        report.coverage.push("Token generated locally with random UUIDs and no usable credentials. No account, API key, network request or callback server is required.".into());
        start_inner(store, session_only, stop, &mut report)?;
        let checked = verify(store, &token.id, stop)?;
        report.findings.extend(checked.findings);
        report.errors.extend(checked.errors);
        report.coverage.extend(checked.coverage);
        Ok(())
    })();
    if let Err(error) = result {
        report.errors.push(error.to_string());
    }
    store.save(&report)?;
    Ok(report)
}

pub fn start_monitor(store: &Store, session_only: bool, stop: &Cancel) -> Result<Report> {
    let _lock = store.lock("canary-setup.lock")?;
    let mut report = Report::new(
        "Monitor startup",
        "Registered local canaries and startup changes",
    );
    if let Err(error) = start_inner(store, session_only, stop, &mut report) {
        report.errors.push(error.to_string());
    }
    store.save(&report)?;
    Ok(report)
}
fn start_inner(
    store: &Store,
    session_only: bool,
    stop: &Cancel,
    report: &mut Report,
) -> Result<()> {
    store.write(MODE, &mode(store).started(session_only, boot()))?;
    if !session_only {
        match crate::checks::setup::daemon_login_service(store, false, stop) {
            Ok(()) => report.coverage.push("Bigman's background checks, which include the monitor, now start at each login using the current binary path. Keep that binary installed; logout/reboot gaps are not monitored.".into()),
            Err(error) => report.errors.push(format!("Automatic startup unavailable: {error}. The monitor runs until Bigman's background checks stop.")),
        }
    } else {
        report.coverage.push("Session-only setup: no login service configured. The monitor runs with Bigman's background checks until the computer restarts.".into());
    }
    if stop.load(Ordering::Relaxed) {
        bail!("Monitor startup cancelled");
    }
    launch(store)?;
    let started = Instant::now();
    while health(store).is_none() {
        if started.elapsed() > Duration::from_secs(8) {
            bail!("Monitor did not report ready; see monitor.log and service status");
        }
        pause(stop)?;
    }
    report
        .coverage
        .push("Monitor heartbeat received. Test each token to verify its read detection.".into());
    Ok(())
}

pub fn stop_monitor(store: &Store, session_only: bool, stop: &Cancel) -> Result<Report> {
    let _lock = store.lock("canary-setup.lock")?;
    let mut report = Report::new("Monitor stop", "Local canary and startup monitoring");
    store.write(MODE, &mode(store).stopped(session_only, boot()))?;
    if !session_only {
        if let Err(error) = crate::checks::setup::daemon_login_service(store, true, stop) {
            report
                .errors
                .push(format!("Login service removal failed: {error}"));
        }
    }
    if let Err(error) = stop_process(store, stop) {
        report.errors.push(error.to_string());
    } else {
        report.coverage.push("Monitoring stopped. Token files, registration and history were preserved. Reads while stopped are not recorded.".into());
    }
    if session_only {
        report.coverage.push(
            "The monitor stays off until the computer restarts, if it was set to run at each login."
                .into(),
        );
    }
    store.save(&report)?;
    Ok(report)
}
fn stop_process(store: &Store, stop: &Cancel) -> Result<()> {
    if store.root.join("monitor-health.json").exists() {
        let h = store.read::<Health>("monitor-health.json")?;
        let proc = PathBuf::from(format!("/proc/{}", h.pid));
        if !proc.exists() {
            fs::remove_file(store.root.join("monitor-health.json"))?;
            return Ok(());
        }
        // A heartbeat this old has no monitor behind it, whatever now has that PID.
        if now().saturating_sub(h.updated) >= 10 {
            fs::remove_file(store.root.join("monitor-health.json"))?;
            return Ok(());
        }
        // bigmand follows the switch by itself. Only a foreground `bigman watch`
        // needs a signal, and only a process whose argv identifies it gets one.
        let argv = read_file(&proc.join("cmdline"), MAX_INPUT)?;
        let args: Vec<_> = argv.split(|b| *b == 0).filter(|b| !b.is_empty()).collect();
        use std::os::unix::ffi::OsStrExt;
        let executable = runner::bigman_executable()?;
        if h.pid >= 2
            && args.len() == 4
            && (args[0] == executable.as_os_str().as_bytes()
                || fs::read_link(proc.join("exe")).ok().as_ref() == Some(&executable))
            && args[1] == b"--state"
            && args[2] == store.root.as_os_str().as_bytes()
            && args[3] == b"watch"
            && unsafe { libc::kill(h.pid as i32, libc::SIGINT) } != 0
        {
            return Err(std::io::Error::last_os_error().into());
        }
        let started = Instant::now();
        while store.root.join("monitor-health.json").exists() {
            if started.elapsed() > Duration::from_secs(5) {
                bail!("Monitor did not stop; check service status");
            }
            pause(stop)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Mode;
    fn this(boot: &str) -> Mode {
        Mode::ThisBoot { boot: boot.into() }
    }
    fn paused(boot: &str) -> Mode {
        Mode::PausedThisBoot { boot: boot.into() }
    }
    #[test]
    fn session_only_lasts_until_a_restart() {
        let m = Mode::Off.started(true, "a".into());
        assert_eq!(m, this("a"));
        assert!(m.wanted("a") && !m.wanted("b"));
    }
    #[test]
    fn a_session_only_stop_comes_back_after_a_restart() {
        let m = Mode::On.stopped(true, "a".into());
        assert_eq!(m, paused("a"));
        assert!(!m.wanted("a") && m.wanted("b"));
        assert_eq!(m.started(true, "a".into()), Mode::On);
    }
    #[test]
    fn a_full_stop_stays_off_and_nothing_starts_it_by_default() {
        assert_eq!(Mode::On.stopped(false, "a".into()), Mode::Off);
        assert_eq!(this("a").stopped(true, "a".into()), Mode::Off);
        assert!(!Mode::default().wanted("a"));
    }
}
