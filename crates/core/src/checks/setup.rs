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
    ])
}

/// Units earlier versions installed: the daily Downloads scan, which the
/// daemon's Downloads watch replaces. Installing or removing clears them.
const LEGACY: [&str; 2] = ["bigman-scan.timer", "bigman-scan.service"];

/// Where per-user units live.
fn unit_dir() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".config"))
        .join("systemd/user")
}

/// Whether the background services are installed, so bigmand and the signature
/// update also start at login. The timer tells this from canary setup, which
/// installs bigmand.service alone.
pub fn services_installed() -> bool {
    unit_dir().join("bigman-update.timer").exists()
}

pub fn services(store: &Store, apply: bool, remove: bool, stop: &Cancel) -> Result<String> {
    if apply {
        adopt_monitor_service(store, stop)?;
    }
    let plan = service_plan(store)?;
    let root = unit_dir();
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
                "bigman-update.timer",
            ]),
            30,
            stop,
        )?;
        if !o.ok() {
            bail!("Units written, but enable failed: {}", o.stderr);
        }
    }
    Ok(if remove{"User services removed"}else{"The collector daemon, with the canary and startup monitor inside it, and the daily signature update are enabled. They follow your user session; no system-wide service or lingering was enabled. A daemon the app already started keeps running until you log out, and the service takes over from the next login."}.into())
}
/// Configures only bigmand at login, which carries the monitor; canary setup must
/// not enable the other units. Removing leaves it when the full background
/// services are installed, and never stops a running bigmand.
pub fn daemon_login_service(store: &Store, remove: bool, stop: &Cancel) -> Result<()> {
    let root = unit_dir();
    if remove && services_installed() {
        return Ok(());
    }
    let (name, contents) = service_plan(store)?
        .into_iter()
        .find(|(name, _)| name == "bigmand.service")
        .unwrap();
    let path = root.join(&name);
    if path.symlink_metadata().is_ok() {
        let existing = crate::store::read_file(&path, crate::store::MAX_INPUT)?;
        if existing != contents.as_bytes() {
            bail!("Existing bigmand unit has different settings. Review it in Tools before replacing it: {}", path.display());
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
            Command::new("systemctl").args(["--user", "disable", &name]),
            10,
            stop,
        )?;
        if !result.ok() {
            bail!("Could not disable bigmand at login: {}", result.stderr);
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
        bail!("bigmand unit changed, but reload failed: {}", reload.stderr);
    }
    if !remove {
        let result = runner::run(
            Command::new("systemctl").args(["--user", "enable", &name]),
            10,
            stop,
        )?;
        if !result.ok() {
            bail!("Could not enable bigmand at login: {}", result.stderr);
        }
    }
    Ok(())
}

/// Earlier versions ran the monitor as its own login service, which kept running
/// after Bigman was stopped. Moves one Bigman wrote over to bigmand: the monitor
/// stays on, and only bigmand starts at login. A unit with other settings is left.
pub fn adopt_monitor_service(store: &Store, stop: &Cancel) -> Result<()> {
    let name = "bigman-monitor.service";
    let path = unit_dir().join(name);
    if path.symlink_metadata().is_err() {
        return Ok(());
    }
    let base = format!(
        "{} --state {}",
        quote(&runner::bigman_executable()?.to_string_lossy())?,
        quote(&store.root.to_string_lossy())?
    );
    let ours = format!("[Unit]\nDescription=Bigman local canary and persistence monitor\n[Service]\nType=simple\nExecStart={base} watch\nRestart=on-failure\nRestartSec=5\nNoNewPrivileges=true\nUMask=0077\n[Install]\nWantedBy=default.target\n");
    if crate::store::read_file(&path, crate::store::MAX_INPUT)? != ours.as_bytes() {
        return Ok(());
    }
    crate::checks::canary::setup::keep_on(store)?;
    let result = runner::run(
        Command::new("systemctl").args(["--user", "disable", "--now", name]),
        10,
        stop,
    )?;
    if !result.ok() {
        bail!(
            "Could not disable the old monitor service: {}",
            result.stderr
        );
    }
    fs::remove_file(&path)?;
    daemon_login_service(store, false, stop)
}
pub fn open_tool(tool: &str) -> Result<()> {
    match tool {
        "firewall" => runner::spawn_detached(&mut Command::new("opensnitch-ui"))?,
        _ => bail!("Unsupported application"),
    };
    Ok(())
}
/// The Firefox build's add-on ID, set in browser-extension/manifest.json. Firefox
/// starts the host with the manifest's path and this ID as its two arguments.
pub const FIREFOX_EXTENSION_ID: &str = "link-detector@bigman-antivirus";
const HOST_MANIFEST: &str = "com.bigman.pdf_scanner.json";
fn config_dir() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".config"))
}
/// Chrome and Chromium per-user native-messaging host manifests.
pub fn chrome_manifest_paths() -> Vec<PathBuf> {
    ["google-chrome", "chromium"]
        .iter()
        .map(|browser| {
            config_dir()
                .join(browser)
                .join("NativeMessagingHosts")
                .join(HOST_MANIFEST)
        })
        .collect()
}
/// Firefox per-user host manifests. Firefox 159 reads only ~/.mozilla, even when
/// its profiles live under ~/.config/mozilla; the XDG copy is for when it reads
/// that location too (Mozilla bug 2005167).
pub fn firefox_manifest_paths() -> Vec<PathBuf> {
    vec![
        home()
            .join(".mozilla/native-messaging-hosts")
            .join(HOST_MANIFEST),
        config_dir()
            .join("mozilla/native-messaging-hosts")
            .join(HOST_MANIFEST),
    ]
}
/// The manifests `register_native_hosts` writes: Firefox's always, Chrome's when
/// a Chrome extension ID is given.
pub fn native_manifest_paths(chrome_extension: Option<&str>) -> Vec<PathBuf> {
    let mut paths = if chrome_extension.is_some() {
        chrome_manifest_paths()
    } else {
        Vec::new()
    };
    paths.extend(firefox_manifest_paths());
    paths
}
fn valid_chrome_id(extension: &str) -> Result<()> {
    if extension.len() != 32 || !extension.bytes().all(|c| (b'a'..=b'p').contains(&c)) {
        bail!("Chrome extension ID must contain exactly 32 letters a–p");
    }
    Ok(())
}
fn write_host_manifest(path: &Path, allowed: (&str, String)) -> Result<()> {
    let mut manifest = serde_json::json!({"name":"com.bigman.pdf_scanner","description":"Bigman deterministic local security checks","path":runner::bigman_executable()?,"type":"stdio"});
    manifest[allowed.0] = serde_json::json!([allowed.1]);
    fs::create_dir_all(
        path.parent()
            .ok_or_else(|| anyhow::anyhow!("Missing parent"))?,
    )?;
    fs::write(path, serde_json::to_vec_pretty(&manifest)?)?;
    Ok(())
}
/// Lets the browser extension start this binary as its native host: in Firefox,
/// and in Chrome and Chromium too when given the extension's Chrome ID. Each
/// manifest allows only that one extension. Returns the files written.
pub fn register_native_hosts(chrome_extension: Option<&str>) -> Result<Vec<PathBuf>> {
    if let Some(id) = chrome_extension {
        valid_chrome_id(id)?;
    }
    let mut written = Vec::new();
    if let Some(id) = chrome_extension {
        for path in chrome_manifest_paths() {
            write_host_manifest(
                &path,
                ("allowed_origins", format!("chrome-extension://{id}/")),
            )?;
            written.push(path);
        }
    }
    for path in firefox_manifest_paths() {
        write_host_manifest(
            &path,
            ("allowed_extensions", FIREFOX_EXTENSION_ID.to_string()),
        )?;
        written.push(path);
    }
    Ok(written)
}
/// True when the process arguments are a browser starting the native host:
/// Chromium passes the extension's origin, Firefox the host manifest's path and
/// the add-on ID.
pub fn started_as_native_host(args: &[String]) -> bool {
    match args {
        [first, ..] if first.starts_with("chrome-extension://") => true,
        [manifest, id, ..] => manifest.ends_with(HOST_MANIFEST) && id == FIREFOX_EXTENSION_ID,
        _ => false,
    }
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
    fn browsers_starting_the_native_host_are_recognised() {
        let args = |list: &[&str]| list.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(started_as_native_host(&args(&[
            "chrome-extension://abcdefghijklmnopabcdefghijklmnop/"
        ])));
        assert!(started_as_native_host(&args(&[
            "/home/sam/.mozilla/native-messaging-hosts/com.bigman.pdf_scanner.json",
            FIREFOX_EXTENSION_ID
        ])));
        assert!(!started_as_native_host(&args(&[
            "scan",
            FIREFOX_EXTENSION_ID
        ])));
        assert!(!started_as_native_host(&args(&[
            "/home/sam/.mozilla/native-messaging-hosts/com.bigman.pdf_scanner.json",
            "other@example.org"
        ])));
        assert!(!started_as_native_host(&args(&[])));
        assert!(!started_as_native_host(&args(&[
            "inspect",
            "https://example.org"
        ])));
    }
    #[test]
    fn firefox_is_always_registered_and_chrome_only_with_an_id() {
        assert!(native_manifest_paths(None)
            .iter()
            .all(|p| p.to_string_lossy().contains("mozilla")));
        assert_eq!(native_manifest_paths(Some("x")).len(), 4);
        assert!(register_native_hosts(Some("not-an-id")).is_err());
    }
    #[test]
    fn service_argument_escaping() {
        assert_eq!(quote("/home/a%/my tool").unwrap(), "\"/home/a%%/my tool\"");
        assert!(quote("bad\nExecStart=x").is_err());
    }
}
