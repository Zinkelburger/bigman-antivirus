//! Reads what the hardening rules need, without administrator rights. Every
//! command has fixed arguments; parsers are public so tests feed them real output.
use super::{
    firewall::{self, FirewallState, Zone},
    Autologin, Desktop, HomeState, Inputs, LoginHistory, MountEncryption, NpmState, RemoteLogin,
    ScreenLock, SysctlSetting, SysctlState, UnitState, KNOBS,
};
use crate::{
    checks::network::system_default,
    model::{now, SkipReason},
    platform::{
        linux::{journal_readable, Loginctl, ProcSockets},
        packagedb, ProcessSockets, Protocol, Sessions,
    },
    runner::{self, Cancel},
    store::home,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process::Command,
};

/// How far back login history is read.
const LOGIN_DAYS: u64 = 90;

fn output(cmd: &mut Command, seconds: u64, stop: &Cancel) -> Option<runner::Output> {
    let program = cmd.get_program().to_string_lossy().into_owned();
    if !runner::available(&program) {
        return None;
    }
    runner::run(cmd, seconds, stop).ok()
}

/// `systemctl show` blocks for several units, in the order asked.
pub fn parse_units(text: &str) -> BTreeMap<String, UnitState> {
    text.split("\n\n")
        .filter_map(|block| {
            let mut id = None;
            let mut s = UnitState::default();
            for line in block.lines() {
                match line.split_once('=') {
                    Some(("Id", v)) => id = Some(v.to_string()),
                    Some(("LoadState", v)) => s.load = v.into(),
                    Some(("UnitFileState", v)) => s.enabled = v.into(),
                    Some(("ActiveState", v)) => s.active = v.into(),
                    _ => {}
                }
            }
            Some((id?, s))
        })
        .collect()
}

fn units(stop: &Cancel) -> BTreeMap<String, UnitState> {
    let mut cmd = Command::new("systemctl");
    cmd.args([
        "show",
        "--no-pager",
        "-p",
        "Id,LoadState,UnitFileState,ActiveState",
    ])
    .args(super::known_units());
    output(&mut cmd, 15, stop)
        .map(|o| parse_units(&o.stdout))
        .unwrap_or_default()
}

fn firewall_state(units: &BTreeMap<String, UnitState>, stop: &Cancel) -> FirewallState {
    let state = |u: &str| units.get(u).cloned().unwrap_or_default();
    if state("firewalld.service").is_active() {
        return firewalld_zones(stop);
    }
    let ufw_on = fs::read_to_string("/etc/ufw/ufw.conf")
        .map(|t| {
            t.lines()
                .any(|l| l.trim().eq_ignore_ascii_case("ENABLED=yes"))
        })
        .unwrap_or(false);
    if ufw_on {
        return FirewallState::Ufw;
    }
    if let Some(other) = ["nftables.service", "iptables.service"]
        .into_iter()
        .find(|u| state(u).is_active())
    {
        return FirewallState::Unknown {
            reason: format!(
                "{other} manages the firewall, and its rules are readable only by an administrator"
            ),
        };
    }
    let installed = if state("firewalld.service").installed() {
        Some("firewalld".into())
    } else if state("ufw.service").installed() || runner::available("ufw") {
        Some("ufw".into())
    } else {
        None
    };
    FirewallState::Off { installed }
}

/// One firewalld D-Bus call through busctl's JSON output: the first returned value.
fn firewalld_call(
    iface: &str,
    method: &str,
    args: &[&str],
    stop: &Cancel,
) -> Option<serde_json::Value> {
    let mut cmd = Command::new("busctl");
    cmd.args([
        "--system",
        "--json=short",
        "call",
        "org.fedoraproject.FirewallD1",
        "/org/fedoraproject/FirewallD1",
    ])
    .arg(format!("org.fedoraproject.FirewallD1{iface}"))
    .arg(method)
    .args(args);
    let o = output(&mut cmd, 10, stop).filter(|o| o.ok())?;
    let mut v: serde_json::Value = serde_json::from_str(&o.stdout).ok()?;
    Some(v["data"][0].take())
}

fn strings(v: &serde_json::Value) -> Vec<String> {
    v.as_array()
        .into_iter()
        .flatten()
        .filter_map(|s| s.as_str().map(String::from))
        .collect()
}

/// `a(ss)` port lists from firewalld's settings dictionaries.
pub fn dbus_ports(v: &serde_json::Value) -> Vec<firewall::PortRange> {
    v["data"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|p| {
            firewall::PortRange::parse(&format!("{}/{}", p[0].as_str()?, p[1].as_str()?))
        })
        .collect()
}

/// One zone from `getZoneSettings2` (an `a{sv}` dictionary). Service ports are filled in separately.
pub fn dbus_zone(name: &str, default: bool, settings: &serde_json::Value) -> Zone {
    Zone {
        name: name.into(),
        default,
        interfaces: strings(&settings["interfaces"]["data"]),
        sources: strings(&settings["sources"]["data"]),
        target: settings["target"]["data"]
            .as_str()
            .unwrap_or("default")
            .into(),
        services: strings(&settings["services"]["data"])
            .into_iter()
            .map(|name| firewall::ZoneService {
                name,
                ports: vec![],
            })
            .collect(),
        ports: dbus_ports(&settings["ports"]),
    }
}

/// firewalld's runtime state over D-Bus, the read-only queries any local user may make.
fn firewalld_dbus(stop: &Cancel) -> Option<Vec<Zone>> {
    let active = firewalld_call(".zone", "getActiveZones", &[], stop)?;
    let default = firewalld_call("", "getDefaultZone", &[], stop)
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_default();
    let mut ports: BTreeMap<String, Vec<firewall::PortRange>> = BTreeMap::new();
    let mut zones = vec![];
    for name in active
        .as_object()?
        .keys()
        .filter(|z| firewall::valid_zone(z))
    {
        let settings = firewalld_call(".zone", "getZoneSettings2", &["s", name], stop)?;
        let mut zone = dbus_zone(name, *name == default, &settings);
        // Bound interfaces come from the active-zone map, which includes NetworkManager's.
        zone.interfaces = strings(&active[name]["interfaces"]);
        zone.sources = strings(&active[name]["sources"]);
        for s in &mut zone.services {
            if !firewall::valid_zone(&s.name) {
                continue;
            }
            if !ports.contains_key(&s.name) {
                let p = firewalld_call("", "getServiceSettings2", &["s", &s.name], stop)
                    .map(|v| dbus_ports(&v["ports"]))
                    .unwrap_or_default();
                ports.insert(s.name.clone(), p);
            }
            s.ports = ports[&s.name].clone();
        }
        zones.push(zone);
    }
    Some(zones)
}

fn firewalld_zones(stop: &Cancel) -> FirewallState {
    if let Some(zones) = firewalld_dbus(stop) {
        return FirewallState::Firewalld { zones };
    }
    let cmd = |args: &[String]| output(Command::new("firewall-cmd").args(args), 10, stop);
    let Some(active) = cmd(&["--get-active-zones".into()]).filter(|o| o.ok()) else {
        return FirewallState::Unknown {
            reason: "firewalld is running, but firewall-cmd did not answer".into(),
        };
    };
    let mut zones: Vec<Zone> = vec![];
    let mut ports: BTreeMap<String, Vec<firewall::PortRange>> = BTreeMap::new();
    for (name, default) in firewall::parse_active_zones(&active.stdout) {
        let Some(o) = cmd(&[format!("--zone={name}"), "--list-all".into()]).filter(|o| o.ok())
        else {
            continue;
        };
        let mut zone = firewall::parse_list_all(&name, default, &o.stdout);
        for s in &mut zone.services {
            if !firewall::valid_zone(&s.name) {
                continue;
            }
            if !ports.contains_key(&s.name) {
                let p = cmd(&[format!("--info-service={}", s.name)])
                    .map(|o| firewall::parse_info_service(&o.stdout))
                    .unwrap_or_default();
                ports.insert(s.name.clone(), p);
            }
            s.ports = ports[&s.name].clone();
        }
        zones.push(zone);
    }
    FirewallState::Firewalld { zones }
}

/// "Accepted publickey for alice from 192.0.2.4 port 50122 ssh2: ..." → (user, from).
pub fn parse_accepted(message: &str) -> Option<(String, String)> {
    let rest = message.strip_prefix("Accepted ")?;
    let (_, rest) = rest.split_once(" for ")?;
    let (user, rest) = rest.split_once(" from ")?;
    let from = rest.split_whitespace().next()?;
    Some((user.into(), from.into()))
}

/// Accepted SSH logins from `journalctl -o json` output.
pub fn parse_journal_logins(text: &str) -> Vec<RemoteLogin> {
    text.lines()
        .filter_map(|line| {
            let v: serde_json::Value = serde_json::from_str(line).ok()?;
            let (user, from) = parse_accepted(v["MESSAGE"].as_str()?)?;
            let at = v["__REALTIME_TIMESTAMP"].as_str()?.parse::<u64>().ok()? / 1_000_000;
            Some(RemoteLogin { at, user, from })
        })
        .collect()
}

/// Network logins from `last -i -w --time-format iso` output. Local logins show
/// the address 0.0.0.0; system reboots are not logins.
pub fn parse_last(text: &str) -> (Vec<RemoteLogin>, Option<u64>) {
    let mut logins = vec![];
    let mut begins = None;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("wtmp begins ") {
            begins = Some(crate::platform::linux::parse_iso8601(rest.trim())).filter(|t| *t > 0);
            continue;
        }
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 4 || f[0] == "reboot" || f[0] == "shutdown" {
            continue;
        }
        let (user, from, at) = (f[0], f[2], f[3]);
        if from == "0.0.0.0" || from == "::" || !from.contains(['.', ':']) {
            continue;
        }
        let at = crate::platform::linux::parse_iso8601(at);
        if at > 0 {
            logins.push(RemoteLogin {
                at,
                user: user.into(),
                from: from.into(),
            });
        }
    }
    (logins, begins)
}

fn first_journal_entry(stop: &Cancel) -> Option<u64> {
    let o = output(
        Command::new("journalctl").args(["--list-boots", "-o", "json", "--no-pager"]),
        15,
        stop,
    )?;
    let boots: Vec<serde_json::Value> = serde_json::from_str(o.stdout.trim()).ok()?;
    boots
        .iter()
        .filter_map(|b| b["first_entry"].as_u64())
        .min()
        .map(|us| us / 1_000_000)
}

fn login_history(now: u64, stop: &Cancel) -> Option<LoginHistory> {
    let window = now.saturating_sub(LOGIN_DAYS * 86_400);
    let mut history = LoginHistory {
        since: now,
        ..Default::default()
    };
    if journal_readable() {
        let since = format!("--since=-{LOGIN_DAYS}d");
        let journal = output(
            Command::new("journalctl").args([
                "--no-pager",
                "--quiet",
                "-o",
                "json",
                &since,
                "_COMM=sshd-session",
                "+",
                "_COMM=sshd",
            ]),
            30,
            stop,
        )
        .filter(|o| o.ok());
        if let Some(o) = journal {
            history.logins.extend(parse_journal_logins(&o.stdout));
            history.since = first_journal_entry(stop).unwrap_or(window).max(window);
            history.read.push("the system journal".into());
        }
    }
    let wtmp = output(
        Command::new("last").args([
            "-i",
            "-w",
            "--time-format",
            "iso",
            "-s",
            &format!("-{LOGIN_DAYS}days"),
        ]),
        15,
        stop,
    )
    .filter(|o| o.ok());
    if let Some(o) = wtmp {
        let (logins, begins) = parse_last(&o.stdout);
        let known: BTreeSet<(u64, String)> = history
            .logins
            .iter()
            .map(|l| (l.at, l.from.clone()))
            .collect();
        history.logins.extend(
            logins
                .into_iter()
                .filter(|l| !known.contains(&(l.at, l.from.clone()))),
        );
        let since = begins.unwrap_or(window).max(window);
        history.since = history.since.min(since);
        history.read.push("wtmp".into());
    }
    history.logins.sort_by_key(|l| l.at);
    (!history.read.is_empty()).then_some(history)
}

/// One sysctl.d file's `key = value` lines, with keys in dotted form.
pub fn parse_sysctl_file(text: &str) -> BTreeMap<String, String> {
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with(['#', ';']))
        .filter_map(|l| {
            let (k, v) = l.split_once('=')?;
            let k = k.trim().trim_start_matches('-').replace('/', ".");
            Some((k, v.trim().to_string()))
        })
        .collect()
}

/// The drop-ins systemd-sysctl reads, in order: one per file name, /etc over
/// /run over /usr/local/lib over /usr/lib, sorted by name; then /etc/sysctl.conf.
fn sysctl_files() -> Vec<PathBuf> {
    let mut by_name: BTreeMap<String, PathBuf> = BTreeMap::new();
    for dir in [
        "/usr/lib/sysctl.d",
        "/usr/local/lib/sysctl.d",
        "/run/sysctl.d",
        "/etc/sysctl.d",
    ] {
        let Ok(entries) = fs::read_dir(dir) else {
            continue;
        };
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            if name.ends_with(".conf") {
                by_name.insert(name, e.path());
            }
        }
    }
    let mut files: Vec<PathBuf> = by_name.into_values().collect();
    let conf = PathBuf::from("/etc/sysctl.conf");
    if conf.exists()
        && !files
            .iter()
            .any(|f| fs::canonicalize(f).ok() == fs::canonicalize(&conf).ok())
    {
        files.push(conf);
    }
    files
}

fn sysctls(stop: &Cancel) -> BTreeMap<String, SysctlState> {
    let files: Vec<(PathBuf, BTreeMap<String, String>)> = sysctl_files()
        .into_iter()
        .filter_map(|f| Some((f.clone(), parse_sysctl_file(&fs::read_to_string(&f).ok()?))))
        .collect();
    let mut owners: BTreeMap<PathBuf, String> = BTreeMap::new();
    let setting: Vec<PathBuf> = files
        .iter()
        .filter(|(_, keys)| KNOBS.iter().any(|k| keys.contains_key(k.key)))
        .map(|(f, _)| f.clone())
        .collect();
    if let Some(db) = packagedb::package_db() {
        if let Ok(attr) = db.attribute(&setting, stop) {
            owners.extend(attr.into_iter().filter_map(|(p, a)| Some((p, a.package?))));
        }
    }
    KNOBS
        .iter()
        .filter_map(|k| {
            let value =
                fs::read_to_string(Path::new("/proc/sys").join(k.key.replace('.', "/"))).ok()?;
            let set_by = files
                .iter()
                .filter_map(|(f, keys)| {
                    Some(SysctlSetting {
                        file: f.clone(),
                        value: keys.get(k.key)?.clone(),
                        package: owners.get(f).map(|p| package_name(p)),
                    })
                })
                .collect();
            Some((
                k.key.to_string(),
                SysctlState {
                    value: value.trim().into(),
                    set_by,
                },
            ))
        })
        .collect()
}

/// "cursor-3.27.22-1793067132.el8.x86_64" → "cursor": the name before the version.
pub fn package_name(nevra: &str) -> String {
    let parts: Vec<&str> = nevra.split('-').collect();
    let version_at = parts
        .iter()
        .position(|p| p.starts_with(|c: char| c.is_ascii_digit()))
        .unwrap_or(parts.len());
    if version_at == 0 {
        return nevra.into();
    }
    parts[..version_at].join("-")
}

/// The global LLMNR mode out of `resolvectl status`: "Protocols: LLMNR=resolve -mDNS" or "+LLMNR".
pub fn parse_llmnr(text: &str) -> Option<String> {
    let global = text.split("\n\n").next()?;
    let protocols = global
        .lines()
        .find_map(|l| l.trim().strip_prefix("Protocols:"))?;
    protocols.split_whitespace().find_map(|t| match t {
        "+LLMNR" => Some("yes".into()),
        "-LLMNR" => Some("no".into()),
        t => t.strip_prefix("LLMNR=").map(String::from),
    })
}

fn desktop() -> Option<Desktop> {
    let current = std::env::var("XDG_CURRENT_DESKTOP").ok().or_else(|| {
        // bigmand runs as a systemd user service; the session exports its desktop there.
        let o = Command::new("systemctl")
            .args(["--user", "show-environment"])
            .output()
            .ok()?;
        String::from_utf8_lossy(&o.stdout)
            .lines()
            .find_map(|l| l.strip_prefix("XDG_CURRENT_DESKTOP=").map(String::from))
    })?;
    let current = current.to_ascii_lowercase();
    if current.split(':').any(|d| d == "kde") {
        Some(Desktop::Kde)
    } else if current
        .split(':')
        .any(|d| d == "gnome" || d == "ubuntu" || d == "unity")
    {
        Some(Desktop::Gnome)
    } else {
        None
    }
}

/// `[Group]` / `key=value` files: the value of `key` in `group`, last one winning.
pub fn ini_value(text: &str, group: &str, key: &str) -> Option<String> {
    let mut in_group = false;
    let mut found = None;
    for line in text.lines().map(str::trim) {
        if line.starts_with('[') {
            in_group = line
                .trim_start_matches('[')
                .trim_end_matches(']')
                .eq_ignore_ascii_case(group);
        } else if in_group {
            if let Some((k, v)) = line.split_once('=') {
                if k.trim() == key {
                    found = Some(v.trim().to_string());
                }
            }
        }
    }
    found
}

fn config_home() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".config"))
}

/// KDE's kscreenlockerrc, with Plasma's defaults for missing keys.
pub fn kde_lock(text: &str) -> ScreenLock {
    let raw: BTreeMap<String, Option<String>> = ["Autolock", "Timeout", "LockOnResume"]
        .iter()
        .map(|k| (k.to_string(), ini_value(text, "Daemon", k)))
        .collect();
    let flag = |k: &str| raw[k].as_deref() != Some("false");
    ScreenLock {
        desktop: Desktop::Kde,
        auto_lock: flag("Autolock"),
        minutes: raw["Timeout"]
            .as_deref()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5),
        on_resume: flag("LockOnResume"),
        raw,
        locked: vec![],
    }
}

fn gsetting(schema: &str, key: &str, stop: &Cancel) -> Option<String> {
    output(
        Command::new("gsettings").args(["get", schema, key]),
        5,
        stop,
    )
    .filter(|o| o.ok())
    .map(|o| o.stdout.trim().to_string())
}

fn screen_lock(stop: &Cancel) -> Option<ScreenLock> {
    match desktop()? {
        Desktop::Kde => {
            let text =
                fs::read_to_string(config_home().join("kscreenlockerrc")).unwrap_or_default();
            Some(kde_lock(&text))
        }
        Desktop::Gnome => {
            let enabled = gsetting("org.gnome.desktop.screensaver", "lock-enabled", stop)?;
            let idle = gsetting("org.gnome.desktop.session", "idle-delay", stop)?;
            let resume = gsetting(
                "org.gnome.desktop.screensaver",
                "ubuntu-lock-on-suspend",
                stop,
            );
            let seconds: u32 = idle
                .trim_start_matches("uint32")
                .trim()
                .parse()
                .unwrap_or(300);
            let mut raw = BTreeMap::new();
            raw.insert("lock-enabled".to_string(), Some(enabled.clone()));
            raw.insert("idle-delay".to_string(), Some(idle));
            raw.insert("ubuntu-lock-on-suspend".to_string(), resume.clone());
            // An administrator can lock keys for every account (dconf locks); then
            // gsettings set fails, so the rule says so instead of offering a change.
            let mut keys = vec![
                ("org.gnome.desktop.screensaver", "lock-enabled"),
                ("org.gnome.desktop.session", "idle-delay"),
            ];
            if resume.is_some() {
                keys.push(("org.gnome.desktop.screensaver", "ubuntu-lock-on-suspend"));
            }
            let locked = keys
                .into_iter()
                .filter(|(schema, key)| {
                    output(
                        Command::new("gsettings").args(["writable", schema, key]),
                        5,
                        stop,
                    )
                    .is_some_and(|o| o.ok() && o.stdout.trim() == "false")
                })
                .map(|(_, key)| key.to_string())
                .collect();
            Some(ScreenLock {
                desktop: Desktop::Gnome,
                auto_lock: enabled != "false",
                minutes: seconds.div_ceil(60),
                on_resume: resume.as_deref() != Some("false"),
                raw,
                locked,
            })
        }
    }
}

fn conf_files(main: &[&str], dirs: &[&str]) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = vec![];
    for d in dirs {
        if let Ok(entries) = fs::read_dir(d) {
            let mut v: Vec<PathBuf> = entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|e| e == "conf"))
                .collect();
            v.sort();
            files.extend(v);
        }
    }
    files.extend(main.iter().map(PathBuf::from));
    files
}

/// Display-manager files that log someone in without a password.
pub fn autologin_in(manager: &str, file: &Path, text: &str) -> Option<Autologin> {
    let user = match manager {
        "gdm" => {
            let on = ini_value(text, "daemon", "AutomaticLoginEnable")
                .or_else(|| ini_value(text, "daemon", "TimedLoginEnable"))?;
            if !on.eq_ignore_ascii_case("true") {
                return None;
            }
            ini_value(text, "daemon", "AutomaticLogin")
                .or_else(|| ini_value(text, "daemon", "TimedLogin"))?
        }
        "lightdm" => ini_value(text, "Seat:*", "autologin-user")
            .or_else(|| ini_value(text, "SeatDefaults", "autologin-user"))?,
        _ => ini_value(text, "Autologin", "User")?,
    };
    (!user.is_empty()).then(|| Autologin {
        file: file.into(),
        user,
        manager: manager.into(),
    })
}

fn autologins(stop: &Cancel) -> Vec<Autologin> {
    let manager = output(
        Command::new("systemctl").args(["show", "display-manager.service", "-p", "Id", "--value"]),
        5,
        stop,
    )
    .map(|o| o.stdout.trim().trim_end_matches(".service").to_string())
    .unwrap_or_default();
    let files = match manager.as_str() {
        "gdm" | "gdm3" => conf_files(
            &[
                "/etc/gdm/custom.conf",
                "/etc/gdm3/custom.conf",
                "/etc/gdm3/daemon.conf",
            ],
            &[],
        ),
        "sddm" => conf_files(
            &["/etc/sddm.conf"],
            &["/usr/lib/sddm/sddm.conf.d", "/etc/sddm.conf.d"],
        ),
        "plasmalogin" => conf_files(
            &["/etc/plasmalogin.conf"],
            &[
                "/usr/lib/plasmalogin/plasmalogin.conf.d",
                "/etc/plasmalogin.conf.d",
            ],
        ),
        "lightdm" => conf_files(
            &["/etc/lightdm/lightdm.conf"],
            &[
                "/usr/share/lightdm/lightdm.conf.d",
                "/etc/lightdm/lightdm.conf.d",
            ],
        ),
        _ => vec![],
    };
    let kind = if manager.starts_with("gdm") {
        "gdm"
    } else {
        manager.as_str()
    };
    // The last file that names a user wins, as the display manager reads them.
    files
        .iter()
        .rev()
        .find_map(|f| autologin_in(kind, f, &fs::read_to_string(f).ok()?))
        .into_iter()
        .collect()
}

/// Groups that list `user` as a member, or are its primary group, from /etc/group.
pub fn groups_of(user: &str, primary: u32, group_file: &str) -> Vec<String> {
    group_file
        .lines()
        .filter_map(|l| {
            let f: Vec<&str> = l.split(':').collect();
            let (name, gid, members) = (f.first()?, f.get(2)?, f.get(3).unwrap_or(&""));
            let member = members.split(',').any(|m| m.trim() == user);
            (member || gid.parse::<u32>().ok() == Some(primary)).then(|| name.to_string())
        })
        .collect()
}

/// Other people's login accounts in /etc/passwd: regular uids with a real shell.
pub fn other_accounts(user: &str, passwd: &str) -> Vec<String> {
    passwd
        .lines()
        .filter_map(|l| {
            let f: Vec<&str> = l.split(':').collect();
            let (name, uid, shell) = (f.first()?, f.get(2)?.parse::<u32>().ok()?, f.get(6)?);
            let login =
                !shell.ends_with("nologin") && !shell.ends_with("/false") && !shell.is_empty();
            ((1000..60000).contains(&uid) && login && *name != user).then(|| name.to_string())
        })
        .collect()
}

/// Whether `/` and `/home` sit on an encrypted device, from `lsblk -J -o NAME,TYPE,MOUNTPOINTS`.
pub fn parse_lsblk(text: &str) -> Option<Vec<MountEncryption>> {
    let v: serde_json::Value = serde_json::from_str(text).ok()?;
    fn walk(node: &serde_json::Value, crypt: bool, out: &mut Vec<MountEncryption>) {
        let crypt = crypt || node["type"].as_str() == Some("crypt");
        for m in node["mountpoints"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|m| m.as_str())
        {
            if m == "/" || m == "/home" {
                out.push(MountEncryption {
                    mount: m.into(),
                    encrypted: crypt,
                });
            }
        }
        for c in node["children"].as_array().into_iter().flatten() {
            walk(c, crypt, out);
        }
    }
    let mut out = vec![];
    for d in v["blockdevices"].as_array()? {
        walk(d, false, &mut out);
    }
    out.sort_by(|a, b| a.mount.cmp(&b.mount));
    out.dedup();
    (!out.is_empty()).then_some(out)
}

fn secure_boot() -> Option<bool> {
    let bytes =
        fs::read("/sys/firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c")
            .ok()?;
    // Four bytes of attributes, then the value.
    bytes.get(4).map(|b| *b == 1)
}

fn current_user() -> Option<(String, u32)> {
    let entry = unsafe { libc::getpwuid(libc::geteuid()) };
    if entry.is_null() {
        return None;
    }
    let name = unsafe { std::ffi::CStr::from_ptr((*entry).pw_name) }
        .to_string_lossy()
        .into_owned();
    let gid = unsafe { (*entry).pw_gid };
    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || "._-".contains(c))
        .then_some((name, gid))
}

/// Reads everything the rules need. What cannot be read is listed in `unread`.
pub fn gather(stop: &Cancel) -> Inputs {
    let now = now();
    let mut unread = vec![];
    let units = units(stop);
    let (user, primary) = current_user().unwrap_or_default();
    let listeners = ProcSockets
        .sockets()
        .unwrap_or_default()
        .into_iter()
        .filter(|s| {
            (s.protocol == Protocol::Tcp && s.listening())
                || (s.protocol == Protocol::Udp && s.state == "unconnected")
        })
        .filter(|s| system_default(s).is_none())
        .collect();
    let firewall = firewall_state(&units, stop);
    let ssh = super::SSH_UNITS
        .iter()
        .any(|u| units.get(*u).is_some_and(|s| s.installed()));
    let logins = if ssh { login_history(now, stop) } else { None };
    let remote_sessions = Loginctl { stop: stop.clone() }
        .sessions()
        .unwrap_or_default()
        .into_iter()
        .filter(|s| s.remote && s.user == user)
        .map(|s| {
            format!(
                "session {} from {}",
                s.id,
                s.remote_host.unwrap_or_else(|| "an unknown address".into())
            )
        })
        .collect();
    let llmnr = output(
        Command::new("resolvectl").args(["status", "--no-pager"]),
        10,
        stop,
    )
    .and_then(|o| parse_llmnr(&o.stdout));
    let groups = fs::read_to_string("/etc/group")
        .map(|g| groups_of(&user, primary, &g))
        .unwrap_or_default();
    let home_path = home();
    let home = fs::metadata(&home_path).ok().map(|m| HomeState {
        path: home_path.clone(),
        mode: m.mode() & 0o7777,
        other_accounts: fs::read_to_string("/etc/passwd")
            .map(|p| other_accounts(&user, &p))
            .unwrap_or_default(),
    });
    let npm = if home_path.join(".npm").is_dir() && runner::available("npm") {
        let get = |args: &[&str]| {
            output(Command::new("npm").args(args), 20, stop)
                .filter(|o| o.ok())
                .map(|o| o.stdout.trim().to_string())
        };
        match (
            get(&["--version"]),
            get(&["config", "get", "ignore-scripts"]),
        ) {
            (Some(version), Some(ignore_scripts)) => Some(NpmState {
                version,
                ignore_scripts,
            }),
            _ => {
                unread.push((
                    "npm install scripts".into(),
                    SkipReason::Failed,
                    "npm did not answer".into(),
                ));
                None
            }
        }
    } else {
        None
    };
    let encryption = output(
        Command::new("lsblk").args(["-J", "-o", "NAME,TYPE,MOUNTPOINTS"]),
        10,
        stop,
    )
    .and_then(|o| parse_lsblk(&o.stdout));
    let autologin = autologins(stop);
    Inputs {
        now,
        user,
        listeners,
        firewall,
        logins,
        remote_sessions,
        sysctl: sysctls(stop),
        llmnr,
        screen_lock: screen_lock(stop),
        autologin,
        groups,
        home,
        npm,
        encryption,
        secure_boot: if Path::new("/sys/firmware/efi").exists() {
            secure_boot()
        } else {
            None
        },
        unread,
        units,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_states_parse_from_systemctl_show() {
        let u = parse_units("Id=sshd.service\nLoadState=loaded\nActiveState=active\nUnitFileState=enabled\n\nId=xrdp.service\nLoadState=not-found\nActiveState=inactive\nUnitFileState=\n");
        assert!(u["sshd.service"].is_enabled() && u["sshd.service"].is_active());
        assert!(!u["xrdp.service"].installed());
    }

    #[test]
    fn logins_come_from_the_journal_and_wtmp() {
        assert_eq!(
            parse_accepted(
                "Accepted publickey for alice from 192.0.2.4 port 50122 ssh2: ED25519 SHA256:x"
            ),
            Some(("alice".into(), "192.0.2.4".into()))
        );
        assert_eq!(parse_accepted("Connection closed by ::1 port 33514"), None);
        let journal = r#"{"MESSAGE":"Accepted password for bob from 198.51.100.7 port 2 ssh2","__REALTIME_TIMESTAMP":"1793000000000000"}"#;
        assert_eq!(parse_journal_logins(journal)[0].at, 1_793_000_000);
        let (logins, begins) = parse_last(
            "anbernal pts/0        0.0.0.0          2026-10-28T11:22:40-04:00   still logged in\nreboot   system boot  0.0.0.0          2026-10-28T11:22:09-04:00   still running\nalice    pts/3        192.0.2.4        2026-10-20T09:00:00-04:00 - 2026-10-20T10:00:00-04:00  (01:00)\n\nwtmp begins 2026-07-27T15:13:49-04:00\n",
        );
        assert_eq!(logins.len(), 1);
        assert_eq!(logins[0].from, "192.0.2.4");
        assert!(begins.is_some());
    }

    #[test]
    fn settings_files_parse() {
        let keys =
            parse_sysctl_file("# comment\nkernel.yama.ptrace_scope = 0\n-kernel/kptr_restrict=1\n");
        assert_eq!(keys["kernel.yama.ptrace_scope"], "0");
        assert_eq!(keys["kernel.kptr_restrict"], "1");
        assert_eq!(
            package_name("cursor-3.27.22-1793067132.el8.x86_64"),
            "cursor"
        );
        assert_eq!(
            package_name("elfutils-default-yama-scope-0.196-1.fc44.noarch"),
            "elfutils-default-yama-scope"
        );
        assert_eq!(
            parse_llmnr("Global\n         Protocols: LLMNR=resolve -mDNS -DNSOverTLS DNSSEC=no/unsupported\n  resolv.conf mode: stub\n\nLink 3 (wlp9s0f0)\n Protocols: +DefaultRoute +LLMNR\n").as_deref(),
            Some("resolve")
        );
        assert_eq!(
            parse_llmnr("Global\n  Protocols: +LLMNR -mDNS\n").as_deref(),
            Some("yes")
        );
        let lock = kde_lock("[Daemon]\nAutolock=false\nTimeout=0\n");
        assert!(!lock.auto_lock && lock.minutes == 0 && lock.on_resume);
        assert_eq!(lock.raw["LockOnResume"], None);
        let defaults = kde_lock("");
        assert!(defaults.auto_lock && defaults.minutes == 5);
    }

    #[test]
    fn display_manager_autologin_parses() {
        let gdm = autologin_in(
            "gdm",
            Path::new("/etc/gdm/custom.conf"),
            "[daemon]\nAutomaticLoginEnable=True\nAutomaticLogin=alice\n",
        );
        assert_eq!(gdm.unwrap().user, "alice");
        assert!(autologin_in("gdm", Path::new("x"), "[daemon]\nWaylandEnable=true\n").is_none());
        let sddm = autologin_in(
            "plasmalogin",
            Path::new("x"),
            "[Autologin]\nUser=bob\nSession=plasma\n",
        );
        assert_eq!(sddm.unwrap().user, "bob");
        assert!(autologin_in("sddm", Path::new("x"), "[Autologin]\n").is_none());
    }

    #[test]
    fn accounts_groups_and_disks_parse() {
        let groups = groups_of(
            "alice",
            1000,
            "wheel:x:10:alice\ndocker:x:975:bob,alice\nalice:x:1000:\nkvm:x:36:\n",
        );
        assert_eq!(groups, ["wheel", "docker", "alice"]);
        let others = other_accounts("alice", "root:x:0:0::/root:/bin/bash\nalice:x:1000:1000::/home/alice:/bin/bash\nbob:x:1001:1001::/home/bob:/bin/zsh\nsvc:x:1002:1002::/:/usr/sbin/nologin\n");
        assert_eq!(others, ["bob"]);
        let settings: serde_json::Value = serde_json::from_str(r#"{"target":{"type":"s","data":"default"},"services":{"type":"as","data":["ssh"]},"ports":{"type":"a(ss)","data":[["1025-65535","udp"],["1025-65535","tcp"]]},"interfaces":{"type":"as","data":["wlp9s0f0"]}}"#).unwrap();
        let zone = dbus_zone("FedoraWorkstation", true, &settings);
        assert_eq!(zone.services[0].name, "ssh");
        assert_eq!(zone.ports.len(), 2);
        assert_eq!(zone.interfaces, ["wlp9s0f0"]);
        let disks = parse_lsblk(r#"{"blockdevices":[{"name":"nvme0n1","type":"disk","mountpoints":[null],"children":[{"name":"nvme0n1p3","type":"part","mountpoints":[],"children":[{"name":"luks-1","type":"crypt","mountpoints":["/home","/"]}]}]},{"name":"sda1","type":"part","mountpoints":["/mnt"]}]}"#).unwrap();
        assert_eq!(
            disks,
            [
                MountEncryption {
                    mount: "/".into(),
                    encrypted: true
                },
                MountEncryption {
                    mount: "/home".into(),
                    encrypted: true
                }
            ]
        );
    }
}
