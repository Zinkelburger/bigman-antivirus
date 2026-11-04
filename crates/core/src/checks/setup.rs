use crate::{
    runner::{self, Cancel},
    store::{home, Store},
};
use anyhow::{bail, Result};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
/// The Downloads folder: XDG_DOWNLOAD_DIR from user-dirs.dirs, which desktops
/// localise (~/Téléchargements), else ~/Downloads.
pub fn downloads() -> PathBuf {
    let config = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".config"));
    fs::read_to_string(config.join("user-dirs.dirs"))
        .ok()
        .and_then(|text| user_dir(&text, "XDG_DOWNLOAD_DIR", &home()))
        .unwrap_or_else(|| home().join("Downloads"))
}

/// One `XDG_..._DIR="$HOME/..."` line; only $HOME-relative or absolute paths.
fn user_dir(text: &str, key: &str, home: &Path) -> Option<PathBuf> {
    let value = text
        .lines()
        .find_map(|l| l.trim().strip_prefix(key)?.strip_prefix('='))?
        .trim()
        .trim_matches('"');
    let path = match value.strip_prefix("$HOME") {
        Some(rest) => home.join(rest.trim_start_matches('/')),
        None if value.starts_with('/') => PathBuf::from(value),
        None => return None,
    };
    // "$HOME/" alone means the desktop turned the folder off.
    (path != home).then_some(path)
}
/// Optional tool groups Setup can install, with the binaries that show they are present.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, ts_rs::TS)]
#[ts(export)]
pub struct ToolGroup {
    pub id: String,
    pub label: String,
    pub purpose: String,
    pub installed: bool,
}

const TOOL_GROUPS: [(&str, &str, &str, &[&str]); 7] = [
    (
        "clamav",
        "ClamAV",
        "Known-malware file scanning and signature updates",
        &["clamscan", "freshclam"],
    ),
    (
        "shellcheck",
        "ShellCheck",
        "Shell correctness checks during installer inspection",
        &["shellcheck"],
    ),
    (
        "sandbox",
        "Bubblewrap and strace",
        "Offline installer rehearsal and isolated PDF parsing",
        &["bwrap", "strace"],
    ),
    (
        "pdf",
        "Poppler utilities",
        "PDF metadata and link extraction",
        &["pdfinfo"],
    ),
    (
        "container",
        "Podman",
        "Disposable containers for untrusted projects",
        &["podman"],
    ),
    ("lynis", "Lynis", "System hardening audit", &["lynis"]),
    (
        "firewall",
        "OpenSnitch",
        "Application firewall",
        &["opensnitch-ui"],
    ),
];

pub fn tool_groups() -> Vec<ToolGroup> {
    TOOL_GROUPS
        .iter()
        .map(|(id, label, purpose, binaries)| ToolGroup {
            id: (*id).into(),
            label: (*label).into(),
            purpose: (*purpose).into(),
            installed: binaries.iter().all(|b| runner::available(b)),
        })
        .collect()
}

pub fn tool_plan(tool: &str) -> Result<Vec<String>> {
    let os = fs::read_to_string("/etc/os-release").unwrap_or_default();
    let fedora = os.lines().any(|l| l == "ID=fedora");
    let debian = os.lines().any(|l| l == "ID=ubuntu" || l == "ID=debian");
    let packages:Vec<&str>=match tool{"clamav"=>vec!["clamav"],"shellcheck"=>vec!["ShellCheck"],"sandbox"=>vec!["bubblewrap","strace"],"lynis"=>vec!["lynis"],"pdf"=>vec!["poppler-utils"],"container"=>vec!["podman"],"firewall"=>vec!["opensnitch"],_=>bail!("Install this optional tool using its upstream instructions; no verified package mapping is configured.")};
    if !fedora && !debian {
        bail!("Automatic package setup supports Fedora, Debian and Ubuntu. Use your distribution's package manager on this system.");
    }
    let mut cmd = if fedora {
        vec!["pkexec", "dnf", "install", "-y"]
    } else {
        vec!["pkexec", "apt-get", "install", "-y"]
    }
    .into_iter()
    .map(String::from)
    .collect::<Vec<_>>();
    for p in packages {
        cmd.push(
            if debian && p == "ShellCheck" {
                "shellcheck"
            } else {
                p
            }
            .into(),
        );
    }
    Ok(cmd)
}
/// Your login name, from the password database rather than the environment.
fn login_name() -> Result<String> {
    let entry = unsafe { libc::getpwuid(libc::geteuid()) };
    if entry.is_null() {
        bail!("Your account is not in the password database");
    }
    let name = unsafe { std::ffi::CStr::from_ptr((*entry).pw_name) }
        .to_string_lossy()
        .into_owned();
    if name.is_empty()
        || !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "._-".contains(c))
    {
        bail!("Unexpected login name {name:?}");
    }
    Ok(name)
}

/// The command that adds you to (or removes you from) systemd-journal, the group
/// that lets the denials source read the system journal without root.
pub fn journal_group_plan(remove: bool) -> Result<Vec<String>> {
    let group = std::ffi::CString::new("systemd-journal")?;
    if unsafe { libc::getgrnam(group.as_ptr()) }.is_null() {
        bail!("This system has no systemd-journal group; the journal may use adm or wheel instead");
    }
    let user = login_name()?;
    Ok(if remove {
        vec![
            "pkexec".into(),
            "gpasswd".into(),
            "-d".into(),
            user,
            "systemd-journal".into(),
        ]
    } else {
        vec![
            "pkexec".into(),
            "usermod".into(),
            "-aG".into(),
            "systemd-journal".into(),
            user,
        ]
    })
}

pub fn journal_group(remove: bool, stop: &Cancel) -> Result<String> {
    let plan = journal_group_plan(remove)?;
    let o = runner::run(Command::new(&plan[0]).args(&plan[1..]), 120, stop)?;
    if !o.ok() {
        bail!(
            "The group change did not complete: {:?}\n{}\n{}",
            o.code,
            o.stdout,
            o.stderr
        );
    }
    Ok(if remove {
        "You are no longer in systemd-journal. Denials stop being collected from your next login."
    } else {
        "You are now in systemd-journal. Log out and back in; denials are collected from then on."
    }
    .into())
}

pub fn install_tool(tool: &str, apply: bool, stop: &Cancel) -> Result<String> {
    let plan = tool_plan(tool)?;
    if !apply {
        return Ok(format!("Install from configured distribution repositories:\n{}\nAdministrator authentication will be handled by polkit.",plan.join(" ")));
    }
    let o = runner::run(Command::new(&plan[0]).args(&plan[1..]), 600, stop)?;
    if !o.ok() {
        bail!(
            "Installation did not complete: {:?}\n{}\n{}",
            o.code,
            o.stdout,
            o.stderr
        );
    }
    Ok(format!("{}\n{}", o.stdout, o.stderr))
}
fn quote(s: &str) -> Result<String> {
    if s.contains(['\n', '\r', '\0']) {
        bail!("Invalid service argument");
    }
    Ok(format!(
        "\"{}\"",
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('%', "%%")
            .replace('$', "$$")
    ))
}
pub fn service_plan(store: &Store) -> Result<Vec<(String, String)>> {
    let exe = runner::bigman_executable()?;
    let base = format!(
        "{} --state {}",
        quote(&exe.to_string_lossy())?,
        quote(&store.root.to_string_lossy())?
    );
    let monitor=format!("[Unit]\nDescription=Bigman local canary and persistence monitor\n[Service]\nType=simple\nExecStart={base} watch\nRestart=on-failure\nRestartSec=5\nNoNewPrivileges=true\nUMask=0077\n[Install]\nWantedBy=default.target\n");
    let update=format!("[Unit]\nDescription=Bigman private signature update\n[Service]\nType=oneshot\nExecStart={base} update\nNoNewPrivileges=true\nUMask=0077\n");
    let daemon = format!(
        "[Unit]\nDescription=Bigman collector daemon\n[Service]\nType=simple\nExecStart={} --state {}\nRestart=on-failure\nRestartSec=10\nNoNewPrivileges=true\nUMask=0077\n[Install]\nWantedBy=default.target\n",
        quote(&runner::sibling_executable("bigmand")?.to_string_lossy())?,
        quote(&store.root.to_string_lossy())?
    );
    let update_timer="[Unit]\nDescription=Daily Bigman definitions update\n[Timer]\nOnCalendar=daily\nPersistent=true\nRandomizedDelaySec=5m\n[Install]\nWantedBy=timers.target\n";
    Ok(vec![
        ("bigmand.service".into(), daemon),
        ("bigman-update.service".into(), update),
        ("bigman-update.timer".into(), update_timer.into()),
        ("bigman-monitor.service".into(), monitor),
    ])
}

/// Units earlier versions installed: the daily Downloads scan, which the
/// daemon's Downloads watch replaces. Installing or removing clears them.
const LEGACY: [&str; 2] = ["bigman-scan.timer", "bigman-scan.service"];
pub fn services(store: &Store, apply: bool, remove: bool, stop: &Cancel) -> Result<String> {
    let plan = service_plan(store)?;
    let root = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".config"))
        .join("systemd/user");
    let legacy: Vec<&str> = LEGACY
        .iter()
        .copied()
        .filter(|n| root.join(n).exists())
        .collect();
    if !apply {
        let replaced = if legacy.is_empty() {
            String::new()
        } else {
            format!(
                "\n\nAlso stops and deletes {}, the daily Downloads scan from an earlier version; the collector daemon now checks each new download as it arrives.",
                legacy.join(" and ")
            )
        };
        return Ok(format!(
            "{} per-user units in {}{replaced}\n{}",
            if remove {
                "Remove"
            } else {
                "Install and enable"
            },
            root.display(),
            plan.iter()
                .map(|(n, c)| format!("\n{n}\n{c}"))
                .collect::<String>()
        ));
    }
    fs::create_dir_all(&root)?;
    if legacy.contains(&"bigman-scan.timer") {
        let o = runner::run(
            Command::new("systemctl").args(["--user", "disable", "--now", "bigman-scan.timer"]),
            30,
            stop,
        )?;
        if !o.ok() {
            bail!("Could not disable the old daily scan: {}", o.stderr);
        }
    }
    for n in &legacy {
        fs::remove_file(root.join(n))?;
    }
    if remove {
        let o = runner::run(
            Command::new("systemctl").args([
                "--user",
                "disable",
                "--now",
                "bigmand.service",
                "bigman-monitor.service",
                "bigman-update.timer",
            ]),
            30,
            stop,
        )?;
        if !o.ok() {
            bail!("Could not disable units: {}", o.stderr);
        }
        for (n, _) in &plan {
            let p = root.join(n);
            if p.exists() {
                fs::remove_file(p)?;
            }
        }
    } else {
        for (n, c) in &plan {
            let p = root.join(n);
            if p.exists() {
                let existing = fs::read_to_string(&p)?;
                if existing != *c {
                    fs::copy(&p, root.join(format!("{n}.{}.bak", crate::model::now())))?;
                }
            }
            let mut tmp = tempfile::NamedTempFile::new_in(&root)?;
            use std::io::Write;
            tmp.write_all(c.as_bytes())?;
            tmp.persist(p).map_err(|e| e.error)?;
        }
    }
    let o = runner::run(
        Command::new("systemctl").args(["--user", "daemon-reload"]),
        30,
        stop,
    )?;
    if !o.ok() {
        bail!("Units written, but daemon reload failed: {}", o.stderr);
    }
    if !remove {
        let o = runner::run(
            Command::new("systemctl").args([
                "--user",
                "enable",
                "--now",
                "bigmand.service",
                "bigman-monitor.service",
                "bigman-update.timer",
            ]),
            30,
            stop,
        )?;
        if !o.ok() {
            bail!("Units written, but enable failed: {}", o.stderr);
        }
    }
    Ok(if remove{"User services removed"}else{"The collector daemon, monitoring, daily scan and definition updates are enabled. They follow your user session; no system-wide service or lingering was enabled. A daemon the app already started keeps running until you log out, and the service takes over from the next login."}.into())
}
/// Configure only the monitor; easy canary setup must not enable unrelated scan timers.
pub fn monitor_service(
    store: &Store,
    remove: bool,
    already_running: bool,
    stop: &Cancel,
) -> Result<()> {
    let root = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".config"))
        .join("systemd/user");
    let (name, contents) = service_plan(store)?
        .into_iter()
        .find(|(name, _)| name == "bigman-monitor.service")
        .unwrap();
    let path = root.join(&name);
    if path.symlink_metadata().is_ok() {
        let existing = crate::store::read_file(&path, crate::store::MAX_INPUT)?;
        if existing != contents.as_bytes() {
            bail!("Existing monitor unit has different settings. Review it in Tools before replacing it: {}", path.display());
        }
    } else if remove {
        return Ok(());
    }
    let probe = runner::run(
        Command::new("systemctl").args(["--user", "show-environment"]),
        5,
        stop,
    )?;
    if !probe.ok() {
        bail!("User service manager unavailable: {}", probe.stderr);
    }
    if remove {
        let result = runner::run(
            Command::new("systemctl").args(["--user", "disable", "--now", &name]),
            10,
            stop,
        )?;
        if !result.ok() {
            bail!("Could not disable monitor: {}", result.stderr);
        }
        fs::remove_file(&path)?;
    } else {
        fs::create_dir_all(&root)?;
        let mut file = tempfile::NamedTempFile::new_in(&root)?;
        use std::io::Write;
        file.write_all(contents.as_bytes())?;
        file.persist(&path).map_err(|e| e.error)?;
    }
    let reload = runner::run(
        Command::new("systemctl").args(["--user", "daemon-reload"]),
        10,
        stop,
    )?;
    if !reload.ok() {
        bail!("Monitor unit changed, but reload failed: {}", reload.stderr);
    }
    if !remove {
        let mut cmd = Command::new("systemctl");
        cmd.args(["--user", "enable"]);
        // An existing detached monitor holds the lock until logout; enable its successor at login.
        if !already_running {
            cmd.arg("--now");
        }
        let result = runner::run(cmd.arg(&name), 10, stop)?;
        if !result.ok() {
            bail!("Could not enable monitor: {}", result.stderr);
        }
    }
    Ok(())
}
pub fn open_tool(tool: &str) -> Result<()> {
    match tool {
        "firewall" => runner::spawn_detached(&mut Command::new("opensnitch-ui"))?,
        _ => bail!("Unsupported application"),
    };
    Ok(())
}
/// Chrome and Chromium per-user native-messaging host manifests.
pub fn native_manifest_paths() -> Vec<PathBuf> {
    let config = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".config"));
    ["google-chrome", "chromium"]
        .iter()
        .map(|browser| {
            config
                .join(browser)
                .join("NativeMessagingHosts/com.bigman.pdf_scanner.json")
        })
        .collect()
}
pub fn write_native_manifest(path: &Path, extension: &str) -> Result<()> {
    if extension.len() != 32 || !extension.bytes().all(|c| (b'a'..=b'p').contains(&c)) {
        bail!("Chrome extension ID must contain exactly 32 letters a–p");
    }
    let manifest = serde_json::json!({"name":"com.bigman.pdf_scanner","description":"Bigman deterministic local security checks","path":runner::bigman_executable()?,"type":"stdio","allowed_origins":[format!("chrome-extension://{extension}/")]});
    fs::create_dir_all(
        path.parent()
            .ok_or_else(|| anyhow::anyhow!("Missing parent"))?,
    )?;
    fs::write(path, serde_json::to_vec_pretty(&manifest)?)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn the_downloads_folder_follows_user_dirs() {
        let home = Path::new("/home/sam");
        let text = "# comment\nXDG_DESKTOP_DIR=\"$HOME/Bureau\"\nXDG_DOWNLOAD_DIR=\"$HOME/Téléchargements\"\n";
        assert_eq!(
            user_dir(text, "XDG_DOWNLOAD_DIR", home),
            Some(PathBuf::from("/home/sam/Téléchargements"))
        );
        assert_eq!(
            user_dir("XDG_DOWNLOAD_DIR=\"$HOME/\"\n", "XDG_DOWNLOAD_DIR", home),
            None
        );
        assert_eq!(
            user_dir("XDG_DOWNLOAD_DIR=\"/data/dl\"\n", "XDG_DOWNLOAD_DIR", home),
            Some(PathBuf::from("/data/dl"))
        );
    }

    use super::*;
    #[test]
    fn service_argument_escaping() {
        assert_eq!(quote("/home/a%/my tool").unwrap(), "\"/home/a%%/my tool\"");
        assert!(quote("bad\nExecStart=x").is_err());
    }
}
