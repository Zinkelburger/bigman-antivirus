use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    io::Read,
    os::fd::AsRawFd,
    os::unix::process::CommandExt,
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
pub type Cancel = Arc<AtomicBool>;
pub fn cancel() -> Cancel {
    Arc::new(AtomicBool::new(false))
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub interrupted: bool,
    pub truncated: bool,
}
impl Output {
    pub fn ok(&self) -> bool {
        self.code == Some(0) && !self.interrupted && !self.truncated
    }
}
fn drain(mut input: impl Read + AsRawFd, finished: Cancel) -> (Vec<u8>, bool) {
    // Nonblocking readers cannot hang forever if a descendant keeps a pipe open.
    unsafe {
        let flags = libc::fcntl(input.as_raw_fd(), libc::F_GETFL);
        libc::fcntl(input.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK);
    }
    let mut buf = [0u8; 8192];
    let mut bytes = vec![];
    let mut truncated = false;
    loop {
        match input.read(&mut buf) {
            Ok(0) => break,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                if finished.load(Ordering::Relaxed) {
                    break;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(_) => {
                truncated = true;
                break;
            }
            Ok(n) => {
                let retain = n.min((2 * 1024 * 1024usize).saturating_sub(bytes.len()));
                bytes.extend_from_slice(&buf[..retain]);
                truncated |= retain < n;
            }
        }
    }
    (bytes, truncated)
}
pub fn run(cmd: &mut Command, seconds: u64, cancel: &Cancel) -> Result<Output> {
    if cancel.load(Ordering::Relaxed) {
        return Ok(Output {
            code: None,
            stdout: String::new(),
            stderr: "Cancelled before start".into(),
            interrupted: true,
            truncated: false,
        });
    }
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("LC_ALL", "C")
        .process_group(0);
    let mut child = cmd.spawn().with_context(|| {
        format!(
            "Cannot start {:?}; install the tool or check its configuration",
            cmd.get_program()
        )
    })?;
    let pid = child.id() as i32;
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let finished = crate::runner::cancel();
    let fa = finished.clone();
    let fb = finished.clone();
    let a = std::thread::spawn(move || drain(stdout, fa));
    let b = std::thread::spawn(move || drain(stderr, fb));
    let start = Instant::now();
    let mut interrupted = false;
    let mut cancellation_note = String::new();
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break Some(status);
        }
        if cancel.load(Ordering::Relaxed) || start.elapsed() >= Duration::from_secs(seconds) {
            interrupted = true;
            let killed = unsafe { libc::kill(-pid, libc::SIGKILL) };
            if killed != 0 && std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH) {
                cancellation_note="Cancellation was requested but permission to stop the process was denied. The privileged operation may still be running; check its package-manager/service status.".into();
                break None;
            }
            break Some(child.wait()?);
        }
        std::thread::sleep(Duration::from_millis(30));
    };
    // Reap descendants that kept pipes open or attempted to outlive the job.
    unsafe {
        libc::kill(-pid, libc::SIGKILL);
    }
    if status.is_none() {
        std::thread::spawn(move || {
            let _ = child.wait();
        });
    }
    finished.store(true, Ordering::Relaxed);
    let (stdout, ta) = a.join().unwrap_or_default();
    let (stderr, tb) = b.join().unwrap_or_default();
    Ok(Output {
        code: status.and_then(|s| s.code()),
        stdout: String::from_utf8_lossy(&stdout).into(),
        stderr: format!("{}{}", String::from_utf8_lossy(&stderr), cancellation_note),
        interrupted,
        truncated: ta || tb,
    })
}
pub fn available(name: &str) -> bool {
    std::env::var_os("PATH").is_some_and(|p| {
        std::env::split_paths(&p).any(|d| {
            let p = d.join(name);
            p.is_file() && {
                use std::os::unix::fs::PermissionsExt;
                p.metadata()
                    .is_ok_and(|m| m.permissions().mode() & 0o111 != 0)
            }
        })
    })
}
/// The `bigman` CLI binary. Service units, native-host manifests and the detached
/// monitor must name it even when the caller is the daemon or the desktop app,
/// which are installed next to it.
pub fn bigman_executable() -> Result<std::path::PathBuf> {
    let current = std::env::current_exe()?;
    if current.file_name().is_some_and(|n| n == "bigman") {
        return Ok(current);
    }
    let sibling = current.with_file_name("bigman");
    if sibling.is_file() {
        Ok(sibling)
    } else {
        anyhow::bail!(
            "Cannot find the bigman binary next to {}",
            current.display()
        )
    }
}
/// Start a long-running process in its own session and reap it in the background.
/// Used for the monitor and for desktop tools; bounded jobs use `run`.
pub fn spawn_detached(cmd: &mut Command) -> Result<()> {
    unsafe {
        cmd.pre_exec(|| {
            if libc::setsid() < 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let mut child = cmd
        .spawn()
        .with_context(|| format!("Cannot start {:?}", cmd.get_program()))?;
    std::thread::spawn(move || {
        let _ = child.wait();
    });
    Ok(())
}
/// A parser is exposed only to its input and system runtime. No home or network.
pub fn parser_command(tool: &str, args: &[&str], input: &std::path::Path) -> Result<Command> {
    if !available("bwrap") {
        anyhow::bail!("bwrap is required to isolate {tool}; external parsing skipped");
    }
    let executable = ["/usr/bin", "/usr/local/bin"]
        .iter()
        .map(|d| std::path::Path::new(d).join(tool))
        .find(|p| p.is_file())
        .context(format!(
            "Install {tool} under /usr/bin or /usr/local/bin for isolated parsing"
        ))?;
    let mut c = Command::new("bwrap");
    c.args([
        "--unshare-all",
        "--die-with-parent",
        "--new-session",
        "--cap-drop",
        "ALL",
        "--clearenv",
        "--setenv",
        "PATH",
        "/usr/bin:/usr/local/bin:/bin",
        "--setenv",
        "HOME",
        "/tmp",
        "--tmpfs",
        "/",
        "--ro-bind",
        "/usr",
        "/usr",
    ]);
    for p in ["/bin", "/lib", "/lib64"] {
        if std::path::Path::new(p).exists() {
            c.args(["--ro-bind", p, p]);
        }
    }
    c.args([
        "--proc",
        "/proc",
        "--dev",
        "/dev",
        "--tmpfs",
        "/tmp",
        "--ro-bind",
    ])
    .arg(input)
    .arg("/input.pdf")
    .args(["--chdir", "/tmp", "--"])
    .arg(executable)
    .args(args)
    .arg("/input.pdf");
    unsafe {
        c.pre_exec(|| {
            for (resource, value) in [
                (libc::RLIMIT_CPU, 25),
                (libc::RLIMIT_AS, 1024 * 1024 * 1024),
                (libc::RLIMIT_FSIZE, 64 * 1024 * 1024),
                (libc::RLIMIT_NOFILE, 128),
            ] {
                let limit = libc::rlimit {
                    rlim_cur: value,
                    rlim_max: value,
                };
                if libc::setrlimit(resource, &limit) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
            }
            Ok(())
        });
    }
    Ok(c)
}

pub fn process_budget() -> Result<libc::rlim_t> {
    use std::os::unix::fs::MetadataExt;
    let uid = unsafe { libc::getuid() };
    let mut count = 0;
    for entry in std::fs::read_dir("/proc")? {
        let entry = entry?;
        if entry
            .file_name()
            .to_string_lossy()
            .bytes()
            .all(|b| b.is_ascii_digit())
            && entry.metadata().is_ok_and(|m| m.uid() == uid)
        {
            if let Ok(tasks) = std::fs::read_dir(entry.path().join("task")) {
                count += tasks.count() as u64;
            }
        }
    }
    let mut limit = libc::rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    if unsafe { libc::getrlimit(libc::RLIMIT_NPROC, &mut limit) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok((count + 128).min(limit.rlim_cur))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timeout_kills_job() {
        let r = run(Command::new("sleep").arg("5"), 0, &cancel()).unwrap();
        assert!(r.interrupted);
    }
    #[test]
    fn captures_nonzero_without_claiming_success() {
        let r = run(
            Command::new("sh").args(["-c", "echo evidence; echo problem >&2; exit 2"]),
            2,
            &cancel(),
        )
        .unwrap();
        assert_eq!(r.code, Some(2));
        assert!(r.stdout.contains("evidence"));
        assert!(!r.ok());
    }
}
