//! What happens when a decoy is touched: every access is written to the decoy's
//! access log, and the responses its owner chose run in a fixed order, ending
//! with the one that ends the session.
use crate::{model::now, runner, store::Store};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};
use ts_rs::TS;

/// The responses chosen for one decoy. All off by default: a touch is then
/// logged and notified only.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, rename = "CanaryResponses")]
pub struct Responses {
    /// Lock every graphical and console session of yours.
    #[serde(default)]
    pub lock_screen: bool,
    /// End every session of yours and every program you run, the monitor
    /// included. Needs reader identification: a file manager showing the
    /// decoy's folder reads the decoy too, and that must not end your session.
    #[serde(default)]
    pub log_out: bool,
    /// Turn NetworkManager networking off for the whole computer.
    #[serde(default)]
    pub network_off: bool,
    /// Kill the process that touched the decoy. Needs reader identification.
    #[serde(default)]
    pub kill_reader: bool,
    /// Lock your account when your account touched the decoy. Needs reader identification.
    #[serde(default)]
    pub lock_account: bool,
    /// An executable of yours to run, with the event in BIGMAN_* variables.
    #[serde(default)]
    pub script: Option<PathBuf>,
}

impl Responses {
    pub fn any(&self) -> bool {
        self.lock_screen
            || self.log_out
            || self.network_off
            || self.kill_reader
            || self.lock_account
            || self.script.is_some()
    }
    /// Plain-language list, in the order they run.
    pub fn describe(&self) -> Vec<String> {
        let mut out = vec![];
        if let Some(s) = &self.script {
            out.push(format!("Run {}", s.display()));
        }
        if self.kill_reader {
            out.push("Kill the process that touched it".into());
        }
        if self.network_off {
            out.push("Turn networking off".into());
        }
        if self.lock_account {
            out.push("Lock your account if it was yours".into());
        }
        if self.lock_screen {
            out.push("Lock the screen".into());
        }
        if self.log_out {
            out.push("Log out every session".into());
        }
        out
    }
    /// Refuses a script Bigman could not run safely: it must be an absolute path
    /// to an executable regular file that only you (or root) can change.
    pub fn validate(&self) -> Result<()> {
        if let Some(script) = &self.script {
            validate_script(script)?;
        }
        Ok(())
    }
}

pub fn validate_script(script: &Path) -> Result<()> {
    if !script.is_absolute() {
        bail!("Give the script's full path, starting with /");
    }
    let meta = script
        .metadata()
        .with_context(|| format!("Cannot find the script {}", script.display()))?;
    if !meta.is_file() {
        bail!("{} is not a regular file", script.display());
    }
    let me = unsafe { libc::getuid() };
    if meta.uid() != me && meta.uid() != 0 {
        bail!(
            "{} belongs to another user; use a script you own",
            script.display()
        );
    }
    if meta.permissions().mode() & 0o022 != 0 {
        bail!(
            "Other users can change {}; run chmod go-w on it first",
            script.display()
        );
    }
    if meta.permissions().mode() & 0o100 == 0 {
        bail!(
            "{} is not executable; run chmod u+x on it first",
            script.display()
        );
    }
    Ok(())
}

/// The process behind an access, as the optional helper reported it.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, rename = "CanaryReader")]
pub struct Reader {
    pub pid: u32,
    pub uid: Option<u32>,
    pub user: Option<String>,
    pub login_uid: Option<u32>,
    pub name: String,
    pub exe: Option<String>,
    pub cmdline: String,
    pub start_time: Option<u64>,
    pub parents: Vec<String>,
}

impl From<bigman_canary_helper::Reader> for Reader {
    fn from(r: bigman_canary_helper::Reader) -> Self {
        Self {
            pid: r.pid,
            uid: r.uid,
            user: r.user,
            login_uid: r.login_uid,
            name: r.name,
            exe: r.exe,
            cmdline: r.cmdline,
            start_time: r.start_time,
            parents: r.parents,
        }
    }
}

/// Programs that read the files of any folder they show or index. GTK and KDE
/// look inside a file whose name does not say what it is (`backup-access.env`)
/// to choose its icon, so opening the decoy's folder reads the decoy. Matched
/// by the program's own file under /usr, which only root can replace.
const FOLDER_BROWSERS: &[&str] = &[
    "nautilus",
    "nemo",
    "caja",
    "thunar",
    "Thunar",
    "dolphin",
    "pcmanfm",
    "pcmanfm-qt",
    "xdg-desktop-portal-gtk",
    "xdg-desktop-portal-gnome",
    "xdg-desktop-portal-kde",
    "tracker-miner-fs-3",
    "tracker-extract-3",
    "localsearch-3",
    "localsearch-extractor-3",
    "baloo_file",
    "baloo_file_extractor",
];

/// Bigman's own programs, which hash and scan files as part of their checks.
const BIGMAN: &[&str] = &["bigman", "bigmand", "bigman-app", "bigman-native-host"];

impl Reader {
    /// The program's file, without the mark the kernel adds once a package
    /// update has replaced it under a running process.
    fn program(&self) -> Option<&Path> {
        self.exe
            .as_deref()
            .map(|e| Path::new(e.strip_suffix(" (deleted)").unwrap_or(e)))
    }
    fn program_named(&self, names: &[&str]) -> bool {
        self.program()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .is_some_and(|n| names.contains(&n))
    }
    /// A file manager, file chooser or search indexer installed by the system.
    pub fn folder_browser(&self) -> bool {
        self.program().is_some_and(|p| p.starts_with("/usr")) && self.program_named(FOLDER_BROWSERS)
    }
    /// One of Bigman's programs, from the folder this one runs from.
    pub fn bigman(&self) -> bool {
        let here = std::env::current_exe().ok();
        let here = here.as_deref().and_then(Path::parent);
        self.program_named(BIGMAN)
            && here.is_some()
            && self.program().and_then(Path::parent) == here
    }
    /// "firefox (PID 4242, user ana)".
    pub fn label(&self) -> String {
        let name = if self.name.is_empty() {
            "process"
        } else {
            &self.name
        };
        match &self.user {
            Some(u) => format!("{name} (PID {}, user {u})", self.pid),
            None => format!("{name} (PID {})", self.pid),
        }
    }
}

/// Why the responses stay off for an access, when every process named for it
/// reads files as a matter of course. The access is still logged and notified.
pub fn routine_readers(readers: &[Reader]) -> Option<String> {
    if readers.is_empty() || !readers.iter().all(|r| r.bigman() || r.folder_browser()) {
        return None;
    }
    let mut names: Vec<&str> = readers.iter().map(|r| r.name.as_str()).collect();
    names.dedup();
    Some(if readers.iter().all(Reader::bigman) {
        format!("The reader was Bigman itself ({})", names.join(", "))
    } else {
        format!(
            "The reader was {}: a file manager, file chooser or search indexer reads the files of any folder it shows",
            names.join(", ")
        )
    })
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, rename = "CanaryResponseOutcome")]
pub struct Outcome {
    pub action: String,
    pub ok: bool,
    pub detail: String,
}

/// One access to a decoy, as recorded in its access log.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, rename = "CanaryAccess")]
pub struct Access {
    pub id: String,
    pub time: u64,
    pub kinds: Vec<String>,
    /// Empty without reader identification, or when the process exited first.
    pub readers: Vec<Reader>,
    /// Bigman's own test read; responses do not run for it.
    pub self_test: bool,
    /// Why responses did not run, when they were chosen but skipped.
    pub skipped: Option<String>,
    pub outcomes: Vec<Outcome>,
}

/// How many accesses each decoy's log keeps; the oldest are dropped first.
pub const LOG_LIMIT: usize = 500;

fn log_name(id: &str) -> Result<String> {
    uuid::Uuid::parse_str(id).context("Invalid canary ID")?;
    Ok(format!("canaries/{id}.log.json"))
}

/// The decoy's access log, newest first.
pub fn log(store: &Store, id: &str) -> Result<Vec<Access>> {
    let name = log_name(id)?;
    if !store.path(&name)?.exists() {
        return Ok(vec![]);
    }
    let mut entries: Vec<Access> = store.read(&name)?;
    entries.reverse();
    Ok(entries)
}

/// Adds or replaces (by entry id) one access in the decoy's log.
pub fn record(store: &Store, canary_id: &str, entry: &Access) -> Result<()> {
    let name = log_name(canary_id)?;
    let _lock = store.lock("canary-log.lock")?;
    let mut entries: Vec<Access> = if store.path(&name)?.exists() {
        store.read(&name).unwrap_or_default()
    } else {
        vec![]
    };
    match entries.iter_mut().find(|e| e.id == entry.id) {
        Some(e) => *e = entry.clone(),
        None => entries.push(entry.clone()),
    }
    if entries.len() > LOG_LIMIT {
        entries.drain(..entries.len() - LOG_LIMIT);
    }
    store.write(&name, &entries)
}

/// Marks the next read of a decoy as Bigman's own test.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestMarker {
    pub pid: u32,
    pub until: u64,
}

/// How long a marker waits for the monitor to reach the test read. The monitor
/// uses the marker up, so it can wait long: a busy monitor that gets to the
/// read late must still find it, or it would treat Bigman's own read as real.
const TEST_MARKER_SECONDS: u64 = 120;

fn marker_name(id: &str) -> String {
    format!("canaries/{id}.test.json")
}

pub fn mark_test(store: &Store, id: &str) -> Result<()> {
    uuid::Uuid::parse_str(id)?;
    store.write(
        &marker_name(id),
        &TestMarker {
            pid: std::process::id(),
            until: now() + TEST_MARKER_SECONDS,
        },
    )
}

/// Removes a decoy's test marker, if it has one.
pub fn drop_test_marker(store: &Store, id: &str) {
    if let Ok(path) = store.path(&marker_name(id)) {
        let _ = std::fs::remove_file(path);
    }
}

/// True when the access is Bigman's own test read: a marker is waiting, the
/// access is a read and nothing else, and the testing process is among the
/// readers the helper named (or none was named). The marker is used up, so one
/// test accounts for one access.
pub fn take_self_test(store: &Store, id: &str, kinds: &[&str], readers: &[Reader]) -> bool {
    let Ok(marker) = store.read::<TestMarker>(&marker_name(id)) else {
        return false;
    };
    if marker.until < now() {
        drop_test_marker(store, id);
        return false;
    }
    if kinds != ["content read"]
        || !(readers.is_empty() || readers.iter().any(|r| r.pid == marker.pid))
    {
        return false;
    }
    drop_test_marker(store, id);
    true
}

/// How long after a decoy logged you out it will not do so again. The guard is
/// read from the decoy's access log, so it holds across the log-out itself:
/// something that reads the decoy at every login cannot keep you out.
pub const LOG_OUT_HOLD: u64 = 600;

const LOG_OUT: &str = "Log out every session";

/// When this decoy last logged you out, if that was within the hold.
pub fn logged_out_recently(store: &Store, id: &str) -> Option<u64> {
    let since = now().saturating_sub(LOG_OUT_HOLD);
    log(store, id)
        .ok()?
        .into_iter()
        .find(|a| a.time >= since && a.outcomes.iter().any(|o| o.ok && o.action == LOG_OUT))
        .map(|a| a.time)
}

/// Where the commands come from, so tests can substitute harmless ones.
pub struct Commands {
    pub loginctl: PathBuf,
    pub nmcli: PathBuf,
    pub proc: PathBuf,
    pub helper_socket: PathBuf,
    pub script_timeout: u64,
}

impl Default for Commands {
    fn default() -> Self {
        Self {
            loginctl: "loginctl".into(),
            nmcli: "nmcli".into(),
            proc: "/proc".into(),
            helper_socket: bigman_canary_helper::SOCKET.into(),
            script_timeout: 30,
        }
    }
}

fn run(action: &str, cmd: &mut Command, seconds: u64) -> Outcome {
    match runner::run(cmd, seconds, &runner::cancel()) {
        Ok(o) if o.ok() => Outcome {
            action: action.into(),
            ok: true,
            detail: o.stdout.trim().chars().take(2000).collect(),
        },
        Ok(o) => Outcome {
            action: action.into(),
            ok: false,
            detail: format!(
                "Exit {}: {}",
                o.code.map_or("signal".into(), |c| c.to_string()),
                format!("{}{}", o.stderr.trim(), o.stdout.trim())
                    .chars()
                    .take(2000)
                    .collect::<String>()
            ),
        },
        Err(e) => Outcome {
            action: action.into(),
            ok: false,
            detail: format!("{e:#}"),
        },
    }
}

/// This process and every process it runs under, up to init: the monitor, the
/// service manager that started it and the session around it.
fn own_ancestry(proc: &Path) -> Vec<u32> {
    let mut chain = vec![std::process::id()];
    while let Some(parent) = chain
        .last()
        .and_then(|pid| std::fs::read_to_string(proc.join(pid.to_string()).join("stat")).ok())
        // The parent is the second field after the command, which ends at the last ')'.
        .and_then(|stat| {
            stat.rsplit_once(')')?
                .1
                .split_whitespace()
                .nth(1)?
                .parse::<u32>()
                .ok()
        })
        .filter(|p| *p > 0 && !chain.contains(p) && chain.len() < 64)
    {
        chain.push(parent);
    }
    chain
}

fn kill_readers(commands: &Commands, readers: &[Reader]) -> Outcome {
    let me = unsafe { libc::getuid() };
    let own = own_ancestry(&commands.proc);
    let mut lines = vec![];
    let mut ok = !readers.is_empty();
    if readers.is_empty() {
        lines.push("No process was identified. Install reader identification under Monitor, or the process exited before it could be named.".into());
    }
    for r in readers {
        // Killing the service manager the monitor runs under ends the whole session.
        let refuse = if r.pid <= 1 || own.contains(&r.pid) || r.bigman() {
            Some("is Bigman, or what Bigman runs under")
        } else if r.uid != Some(me) {
            Some("belongs to another user")
        } else if r.start_time.is_none()
            || bigman_canary_helper::reader::start_time(&commands.proc, r.pid) != r.start_time
        {
            Some("already exited")
        } else {
            None
        };
        if let Some(why) = refuse {
            lines.push(format!("Did not kill {}: it {why}", r.label()));
            ok = false;
            continue;
        }
        if unsafe { libc::kill(r.pid as i32, libc::SIGKILL) } == 0 {
            lines.push(format!("Killed {}", r.label()));
        } else {
            lines.push(format!(
                "Could not kill {}: {}",
                r.label(),
                std::io::Error::last_os_error()
            ));
            ok = false;
        }
    }
    Outcome {
        action: "Kill the process".into(),
        ok,
        detail: lines.join("\n"),
    }
}

fn lock_account(commands: &Commands, readers: &[Reader]) -> Outcome {
    let me = unsafe { libc::getuid() };
    let action = "Lock your account".to_string();
    if !readers.iter().any(|r| r.uid == Some(me)) {
        return Outcome {
            action,
            ok: false,
            detail: if readers.is_empty() {
                "Not locked: no process was identified, so Bigman cannot tell it was your account."
                    .into()
            } else {
                "Not locked: the processes that touched the decoy did not run as your account."
                    .into()
            },
        };
    }
    let reply = bigman_canary_helper::Client::connect(&commands.helper_socket).and_then(|mut c| {
        c.request(
            &bigman_canary_helper::ClientMessage::LockAccount,
            Duration::from_secs(15),
            |m| matches!(m, bigman_canary_helper::HelperMessage::Locked { .. }),
        )
    });
    match reply {
        Ok((Some(bigman_canary_helper::HelperMessage::Locked { ok, detail }), _)) => {
            Outcome { action, ok, detail }
        }
        Ok(_) => Outcome {
            action,
            ok: false,
            detail: "The reader-identification helper did not answer.".into(),
        },
        Err(e) => Outcome {
            action,
            ok: false,
            detail: format!("Cannot reach the reader-identification helper: {e}"),
        },
    }
}

/// The event a script sees, as environment variables.
pub fn script_env(
    canary_id: &str,
    path: &Path,
    kinds: &[String],
    readers: &[Reader],
) -> Vec<(String, String)> {
    let mut env = vec![
        ("BIGMAN_CANARY_ID".into(), canary_id.to_string()),
        ("BIGMAN_CANARY_PATH".into(), path.display().to_string()),
        ("BIGMAN_CANARY_EVENTS".into(), kinds.join(", ")),
        ("BIGMAN_CANARY_TIME".into(), now().to_string()),
    ];
    if let Some(r) = readers.first() {
        env.push(("BIGMAN_READER_PID".into(), r.pid.to_string()));
        env.push(("BIGMAN_READER_NAME".into(), r.name.clone()));
        env.push(("BIGMAN_READER_CMDLINE".into(), r.cmdline.clone()));
        if let Some(u) = r.uid {
            env.push(("BIGMAN_READER_UID".into(), u.to_string()));
        }
        if let Some(u) = &r.user {
            env.push(("BIGMAN_READER_USER".into(), u.clone()));
        }
        if let Some(e) = &r.exe {
            env.push(("BIGMAN_READER_EXE".into(), e.clone()));
        }
    }
    env
}

/// One access to a decoy, as the responses see it.
pub struct Touch<'a> {
    pub canary_id: &'a str,
    pub path: &'a Path,
    pub kinds: &'a [String],
    pub readers: &'a [Reader],
    /// When this decoy last logged you out, if recently: it is not done again.
    pub logged_out: Option<u64>,
}

/// Runs the chosen responses in order. `progress` gets every outcome so far
/// after each step, so the log is current even if the last step ends this
/// process (logging out does).
pub fn respond(
    commands: &Commands,
    responses: &Responses,
    touch: &Touch,
    mut progress: impl FnMut(&[Outcome]),
) -> Vec<Outcome> {
    let Touch {
        canary_id,
        path,
        kinds,
        readers,
        logged_out,
    } = *touch;
    let me = unsafe { libc::getuid() }.to_string();
    let mut out: Vec<Outcome> = vec![];
    let mut step = |o: Outcome, out: &mut Vec<Outcome>| {
        out.push(o);
        progress(out);
    };
    if let Some(script) = &responses.script {
        let o = match validate_script(script) {
            Err(e) => Outcome {
                action: "Run your script".into(),
                ok: false,
                detail: format!("Not run: {e:#}"),
            },
            Ok(()) => {
                let mut cmd = Command::new(script);
                cmd.envs(script_env(canary_id, path, kinds, readers))
                    .stdin(std::process::Stdio::null());
                run("Run your script", &mut cmd, commands.script_timeout)
            }
        };
        step(o, &mut out);
    }
    if responses.kill_reader {
        step(kill_readers(commands, readers), &mut out);
    }
    if responses.network_off {
        step(
            run(
                "Turn networking off",
                Command::new(&commands.nmcli).args(["networking", "off"]),
                15,
            ),
            &mut out,
        );
    }
    if responses.lock_account {
        step(lock_account(commands, readers), &mut out);
    }
    if responses.lock_screen {
        let sessions = runner::run(
            Command::new(&commands.loginctl).args(["show-user", &me, "-p", "Sessions", "--value"]),
            10,
            &runner::cancel(),
        );
        let o = match sessions {
            Ok(s) if s.ok() && !s.stdout.trim().is_empty() => {
                let results: Vec<Outcome> = s
                    .stdout
                    .split_whitespace()
                    .map(|id| {
                        run(
                            "Lock the screen",
                            Command::new(&commands.loginctl).args(["lock-session", id]),
                            10,
                        )
                    })
                    .collect();
                Outcome {
                    action: "Lock the screen".into(),
                    ok: results.iter().all(|r| r.ok),
                    detail: format!(
                        "Sessions {}{}",
                        s.stdout.trim(),
                        results
                            .iter()
                            .filter(|r| !r.ok)
                            .map(|r| format!("\n{}", r.detail))
                            .collect::<String>()
                    ),
                }
            }
            Ok(s) => Outcome {
                action: "Lock the screen".into(),
                ok: false,
                detail: format!("No sessions to lock. {}", s.stderr.trim()),
            },
            Err(e) => Outcome {
                action: "Lock the screen".into(),
                ok: false,
                detail: format!("{e:#}"),
            },
        };
        step(o, &mut out);
    }
    if responses.log_out {
        let held = if readers.is_empty() {
            Some("Not logged out: no process was identified, so this may be a file manager, a file chooser or a backup reading the decoy. Turn on reader identification under Monitor.".to_string())
        } else {
            logged_out.map(|at| {
                let minutes = now().saturating_sub(at) / 60;
                format!(
                    "Not logged out: this decoy logged you out {} ago. It is held back for {} minutes, so a program that reads the decoy at every login cannot keep you out.",
                    match minutes {
                        0 => "less than a minute".to_string(),
                        1 => "a minute".to_string(),
                        n => format!("{n} minutes"),
                    },
                    LOG_OUT_HOLD / 60
                )
            })
        };
        if let Some(detail) = held {
            step(
                Outcome {
                    action: LOG_OUT.into(),
                    ok: false,
                    detail,
                },
                &mut out,
            );
            return out;
        }
        // Recorded before it runs: ending the session ends this process too.
        step(
            Outcome {
                action: LOG_OUT.into(),
                ok: true,
                detail: "Started. Every program of yours, Bigman's monitor included, is ending."
                    .into(),
            },
            &mut out,
        );
        let o = run(
            LOG_OUT,
            Command::new(&commands.loginctl).args(["terminate-user", &me]),
            15,
        );
        if !o.ok {
            out.pop();
            step(o, &mut out);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn executable(path: &Path, body: &str) {
        std::fs::write(path, body).unwrap();
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).unwrap();
    }

    fn fake_commands(dir: &Path) -> Commands {
        let loginctl = dir.join("loginctl");
        let nmcli = dir.join("nmcli");
        let calls = dir.join("calls");
        executable(
            &loginctl,
            &format!(
                "#!/bin/sh\necho \"loginctl $*\" >> {}\ncase \"$1\" in show-user) echo '2 5';; esac\n",
                calls.display()
            ),
        );
        executable(
            &nmcli,
            &format!("#!/bin/sh\necho \"nmcli $*\" >> {}\n", calls.display()),
        );
        Commands {
            loginctl,
            nmcli,
            proc: "/proc".into(),
            helper_socket: dir.join("no-helper.sock"),
            script_timeout: 5,
        }
    }

    #[test]
    fn refuses_scripts_that_are_relative_shared_or_not_executable() {
        let d = tempfile::tempdir().unwrap();
        let s = d.path().join("respond.sh");
        assert!(validate_script(Path::new("respond.sh")).is_err());
        assert!(validate_script(&s).is_err());
        std::fs::write(&s, "#!/bin/sh\n").unwrap();
        std::fs::set_permissions(&s, std::fs::Permissions::from_mode(0o600)).unwrap();
        assert!(validate_script(&s)
            .unwrap_err()
            .to_string()
            .contains("not executable"));
        std::fs::set_permissions(&s, std::fs::Permissions::from_mode(0o777)).unwrap();
        assert!(validate_script(&s)
            .unwrap_err()
            .to_string()
            .contains("chmod go-w"));
        std::fs::set_permissions(&s, std::fs::Permissions::from_mode(0o700)).unwrap();
        validate_script(&s).unwrap();
        assert!(validate_script(d.path()).is_err());
    }

    #[test]
    fn runs_every_choice_in_order_and_logs_out_last() {
        let d = tempfile::tempdir().unwrap();
        let commands = fake_commands(d.path());
        let script = d.path().join("respond.sh");
        let seen = d.path().join("env");
        executable(
            &script,
            &format!(
                "#!/bin/sh\necho \"$BIGMAN_CANARY_ID $BIGMAN_CANARY_EVENTS $BIGMAN_READER_PID\" > {}\necho script >> {}\n",
                seen.display(),
                d.path().join("calls").display()
            ),
        );
        let responses = Responses {
            lock_screen: true,
            log_out: true,
            network_off: true,
            script: Some(script),
            ..Responses::default()
        };
        let reader = Reader {
            pid: 4242,
            ..Reader::default()
        };
        let mut steps = 0;
        let path = Path::new("/home/u/decoy.env");
        let kinds = ["content read".to_string()];
        let touch = |readers, logged_out| Touch {
            canary_id: "id-1",
            path,
            kinds: &kinds,
            readers,
            logged_out,
        };
        let named = std::slice::from_ref(&reader);
        let out = respond(&commands, &responses, &touch(named, None), |_| steps += 1);
        assert_eq!(steps, 4);
        assert!(out.iter().all(|o| o.ok), "{out:?}");
        assert_eq!(
            std::fs::read_to_string(&seen).unwrap().trim(),
            "id-1 content read 4242"
        );
        let me = unsafe { libc::getuid() };
        let calls = std::fs::read_to_string(d.path().join("calls")).unwrap();
        assert_eq!(
            calls.lines().collect::<Vec<_>>(),
            [
                "script".to_string(),
                "nmcli networking off".into(),
                format!("loginctl show-user {me} -p Sessions --value"),
                "loginctl lock-session 2".into(),
                "loginctl lock-session 5".into(),
                format!("loginctl terminate-user {me}"),
            ]
        );

        // Without a named reader, or soon after a log-out, the session is left alone.
        let only_log_out = Responses {
            log_out: true,
            ..Responses::default()
        };
        let unnamed = respond(&commands, &only_log_out, &touch(&[], None), |_| {});
        assert!(!unnamed[0].ok && unnamed[0].detail.contains("no process was identified"));
        let held = respond(
            &commands,
            &only_log_out,
            &touch(named, Some(now() - 120)),
            |_| {},
        );
        assert!(!held[0].ok && held[0].detail.contains("logged you out 2 minutes ago"));
        let calls = std::fs::read_to_string(d.path().join("calls")).unwrap();
        assert_eq!(calls.matches("terminate-user").count(), 1);
    }

    #[test]
    fn a_recent_log_out_is_read_back_from_the_access_log() {
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let access = |time: u64, action: &str, ok: bool| Access {
            id: time.to_string(),
            time,
            kinds: vec!["content read".into()],
            readers: vec![],
            self_test: false,
            skipped: None,
            outcomes: vec![Outcome {
                action: action.into(),
                ok,
                detail: String::new(),
            }],
        };
        assert_eq!(logged_out_recently(&store, &id), None);
        record(
            &store,
            &id,
            &access(now() - LOG_OUT_HOLD - 60, LOG_OUT, true),
        )
        .unwrap();
        record(&store, &id, &access(now() - 30, "Lock the screen", true)).unwrap();
        record(&store, &id, &access(now() - 20, LOG_OUT, false)).unwrap();
        assert_eq!(logged_out_recently(&store, &id), None);
        record(&store, &id, &access(now() - 10, LOG_OUT, true)).unwrap();
        assert_eq!(logged_out_recently(&store, &id), Some(now() - 10));
    }

    #[test]
    fn file_managers_and_bigman_itself_are_routine_readers() {
        let reader = |name: &str, exe: &str| Reader {
            pid: 7,
            name: name.into(),
            exe: Some(exe.into()),
            ..Reader::default()
        };
        let files = reader("nautilus", "/usr/bin/nautilus");
        let updated = reader("localsearch-3", "/usr/libexec/localsearch-3 (deleted)");
        // The name alone proves nothing: anyone can call a program nautilus.
        let imposter = reader("nautilus", "/home/u/.cache/nautilus");
        let other = reader("rg", "/usr/bin/rg");
        assert!(files.folder_browser() && updated.folder_browser());
        assert!(!imposter.folder_browser() && !other.folder_browser());
        assert_eq!(routine_readers(&[]), None);
        assert_eq!(routine_readers(&[files.clone(), other]), None);
        assert_eq!(routine_readers(&[imposter]), None);
        assert!(routine_readers(&[files, updated])
            .unwrap()
            .contains("nautilus, localsearch-3"));
        let here = std::env::current_exe().unwrap();
        let daemon = reader("bigmand", here.with_file_name("bigmand").to_str().unwrap());
        assert!(daemon.bigman() && !reader("bigmand", "/tmp/bigmand").bigman());
        assert!(routine_readers(&[daemon])
            .unwrap()
            .contains("Bigman itself"));
    }

    #[test]
    fn kills_only_a_live_identified_process_of_yours() {
        let d = tempfile::tempdir().unwrap();
        let commands = fake_commands(d.path());
        let me = unsafe { libc::getuid() };
        let mut child = Command::new("sleep").arg("30").spawn().unwrap();
        let pid = child.id();
        let live = Reader {
            pid,
            uid: Some(me),
            start_time: bigman_canary_helper::reader::start_time(Path::new("/proc"), pid),
            ..Reader::default()
        };
        let reused = Reader {
            start_time: live.start_time.map(|t| t + 1),
            ..live.clone()
        };
        let other = Reader {
            uid: Some(me + 1),
            ..live.clone()
        };
        let o = kill_readers(&commands, &[reused, other]);
        assert!(!o.ok && o.detail.contains("already exited") && o.detail.contains("another user"));
        assert!(
            child.try_wait().unwrap().is_none(),
            "a mismatched reader must not be killed"
        );
        let o = kill_readers(&commands, &[live]);
        assert!(o.ok, "{o:?}");
        assert!(child.wait().unwrap().code().is_none());
        assert!(!kill_readers(&commands, &[]).ok);
    }

    #[test]
    fn an_unidentified_reader_never_locks_the_account() {
        let d = tempfile::tempdir().unwrap();
        let commands = fake_commands(d.path());
        let o = lock_account(&commands, &[]);
        assert!(!o.ok && o.detail.contains("no process was identified"));
        let root = Reader {
            pid: 9,
            uid: Some(0),
            ..Reader::default()
        };
        assert!(lock_account(&commands, &[root])
            .detail
            .contains("did not run as your account"));
    }

    #[test]
    fn the_log_keeps_the_newest_entries_and_updates_in_place() {
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let entry = |n: usize| Access {
            id: n.to_string(),
            time: n as u64,
            kinds: vec!["content read".into()],
            readers: vec![],
            self_test: false,
            skipped: None,
            outcomes: vec![],
        };
        for n in 0..LOG_LIMIT + 3 {
            record(&store, &id, &entry(n)).unwrap();
        }
        let mut last = entry(LOG_LIMIT + 2);
        last.outcomes.push(Outcome {
            action: "Lock the screen".into(),
            ok: true,
            detail: String::new(),
        });
        record(&store, &id, &last).unwrap();
        let log = log(&store, &id).unwrap();
        assert_eq!(log.len(), LOG_LIMIT);
        assert_eq!(log[0], last);
        assert_eq!(log.last().unwrap().id, "3");
        assert!(super::log(&store, "../escape").is_err());
    }

    #[test]
    fn a_test_marker_accounts_for_one_plain_read_by_bigman() {
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let read = ["content read"];
        assert!(!take_self_test(&store, &id, &read, &[]));
        mark_test(&store, &id).unwrap();
        let me = Reader {
            pid: std::process::id(),
            ..Reader::default()
        };
        let other = Reader {
            pid: 1,
            ..Reader::default()
        };
        // Someone else's read, or anything more than a read, leaves the marker for the test.
        assert!(!take_self_test(
            &store,
            &id,
            &read,
            std::slice::from_ref(&other)
        ));
        assert!(!take_self_test(
            &store,
            &id,
            &["content read", "content changed"],
            &[]
        ));
        // A report left over from another process does not turn the test into a real access.
        assert!(take_self_test(&store, &id, &read, &[other, me]));
        // The marker is used up: the next read is real.
        assert!(!take_self_test(&store, &id, &read, &[]));

        mark_test(&store, &id).unwrap();
        assert!(take_self_test(&store, &id, &read, &[]));
        // A marker past its time is removed, not honoured.
        store
            .write(
                &marker_name(&id),
                &TestMarker {
                    pid: std::process::id(),
                    until: now() - 1,
                },
            )
            .unwrap();
        assert!(!take_self_test(&store, &id, &read, &[]));
        assert!(!store.path(&marker_name(&id)).unwrap().exists());
    }

    #[test]
    fn never_kills_bigman_or_what_it_runs_under() {
        let d = tempfile::tempdir().unwrap();
        let commands = fake_commands(d.path());
        let me = unsafe { libc::getuid() };
        let chain = own_ancestry(Path::new("/proc"));
        assert_eq!(chain[0], std::process::id());
        assert!(chain.len() > 1, "the test runner has a parent");
        // The parent of this test process, named exactly as the helper would.
        let parent = Reader {
            pid: chain[1],
            uid: Some(me),
            start_time: bigman_canary_helper::reader::start_time(Path::new("/proc"), chain[1]),
            ..Reader::default()
        };
        let o = kill_readers(&commands, &[parent]);
        assert!(
            !o.ok && o.detail.contains("what Bigman runs under"),
            "{o:?}"
        );
    }
}
