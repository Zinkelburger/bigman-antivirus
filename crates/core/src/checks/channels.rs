//! Where packages come from (plan D4): apt one-line and deb822 sources and
//! pins, dnf/yum and zypper `.repo` files, `pacman.conf`, Flatpak remotes and
//! Nix caches and channels. Configuration is read; no repository is contacted.
use crate::{
    model::{age, now, Report, Severity},
    runner,
    store::{home, read_file, MAX_INPUT},
};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    process::Command,
    time::UNIX_EPOCH,
};

/// One package source, whatever the package manager.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Channel {
    pub manager: String,
    pub name: String,
    #[serde(default)]
    pub url: String,
    pub file: String,
    /// Package signatures are verified.
    #[serde(default = "yes")]
    pub signed: bool,
    /// apt: the key is scoped to this source with signed-by.
    #[serde(default)]
    pub scoped_key: Option<bool>,
    /// apt: metadata expiry (Valid-Until) is enforced.
    #[serde(default = "yes")]
    pub checks_expiry: bool,
    /// Run by the distribution (or Flathub), recognised by host or repo name.
    #[serde(default)]
    pub official: bool,
    /// A one-person repository: PPA, COPR, OBS home project.
    #[serde(default)]
    pub personal: bool,
    /// Ranked above the distribution's own packages, and how.
    #[serde(default)]
    pub overrides: Option<String>,
}

fn yes() -> bool {
    true
}

/// An apt preferences stanza that ranks a source above the distribution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pin {
    pub file: String,
    pub package: String,
    pub pin: String,
    pub priority: i64,
}

/// When the distribution's newest package metadata was published, and where
/// that was read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metadata {
    pub source: String,
    pub published: u64,
}

/// Everything the channel rules look at, gathered from this machine or a fixture.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Inputs {
    #[serde(default)]
    pub channels: Vec<Channel>,
    #[serde(default)]
    pub pins: Vec<Pin>,
    /// Third-party keyrings apt trusts for every source.
    #[serde(default)]
    pub global_keys: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Metadata>,
}

/// Hosts run by distributions; subdomains count (mirror names such as
/// us.archive.ubuntu.com or ftp.de.debian.org).
const OFFICIAL: [&str; 12] = [
    "debian.org",
    "archive.ubuntu.com",
    "security.ubuntu.com",
    "ports.ubuntu.com",
    "fedoraproject.org",
    "download.opensuse.org",
    "cdn.opensuse.org",
    "pkgbuild.com",
    "archlinux.org",
    "flathub.org",
    "nixos.org",
    "cache.nixos.org",
];

fn host(url: &str) -> &str {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    let authority = rest.split('/').next().unwrap_or("");
    let authority = authority.rsplit_once('@').map_or(authority, |(_, h)| h);
    authority.split(':').next().unwrap_or("")
}

fn path(url: &str) -> &str {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    rest.find('/').map_or("", |i| &rest[i..])
}

fn official(url: &str) -> bool {
    let h = host(url).to_ascii_lowercase();
    // OBS projects share the distribution's host but are not the distribution.
    if h == "download.opensuse.org" && path(url).starts_with("/repositories/") {
        return false;
    }
    h != "people.debian.org"
        && OFFICIAL
            .iter()
            .any(|o| h == *o || h.ends_with(&format!(".{o}")))
}

fn personal(name: &str, url: &str) -> bool {
    let h = host(url).to_ascii_lowercase();
    ["ppa.launchpad.net", "ppa.launchpadcontent.net"].contains(&h.as_str())
        || h.ends_with("copr.fedorainfracloud.org")
        || name.starts_with("copr:")
        || path(url).starts_with("/repositories/home:")
}

fn falsy(v: &str) -> bool {
    ["0", "false", "no", "off"].contains(&v.trim().to_ascii_lowercase().as_str())
}

/// The fields every manager shares; the parsers fill in the rest.
fn channel(manager: &str, name: &str, url: &str, file: &str) -> Channel {
    Channel {
        manager: manager.into(),
        name: name.into(),
        url: url.into(),
        file: file.into(),
        signed: true,
        scoped_key: None,
        checks_expiry: true,
        official: official(url),
        personal: personal(name, url),
        overrides: None,
    }
}

/// apt one-line sources: `deb [opt=val ...] URI suite components`.
pub fn apt_list(file: &str, text: &str) -> Vec<Channel> {
    text.lines()
        .map(str::trim)
        .filter(|l| l.starts_with("deb ") || l.starts_with("deb-src "))
        .filter_map(|l| {
            let rest = l.split_once(' ')?.1.trim();
            let (options, rest) = match rest.strip_prefix('[') {
                Some(r) => r.split_once(']').map(|(o, r)| (o, r.trim()))?,
                None => ("", rest),
            };
            let url = rest.split_whitespace().next()?;
            let opt = |k: &str| {
                options
                    .split_whitespace()
                    .find_map(|o| o.strip_prefix(k)?.strip_prefix('='))
            };
            let mut c = channel("apt", url, url, file);
            apt_options(&mut c, |k| opt(k).map(str::to_string));
            Some(c)
        })
        .fold(vec![], |mut out: Vec<Channel>, c| {
            // `deb` and `deb-src` lines for one source with the same options are one source.
            if !out.contains(&c) {
                out.push(c);
            }
            out
        })
}

/// The apt source options Bigman judges, by their one-line names; deb822 uses
/// the same words capitalised.
fn apt_options(c: &mut Channel, opt: impl Fn(&str) -> Option<String>) {
    let is_yes = |k: &str| opt(k).is_some_and(|v| v.eq_ignore_ascii_case("yes"));
    c.signed = ![
        "trusted",
        "allow-insecure",
        "allow-weak",
        "allow-downgrade-to-insecure",
    ]
    .iter()
    .any(|k| is_yes(k));
    c.scoped_key = Some(opt("signed-by").is_some_and(|v| !v.trim().is_empty()));
    c.checks_expiry = !opt("check-valid-until").is_some_and(|v| falsy(&v));
}

/// apt deb822 `.sources`: stanzas of `Key: value` lines separated by blank lines.
pub fn apt_deb822(file: &str, text: &str) -> Vec<Channel> {
    let mut out = vec![];
    for stanza in stanzas(text) {
        let field = |k: &str| {
            stanza
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(k))
                .map(|(_, v)| v.clone())
        };
        if field("Enabled").is_some_and(|e| falsy(&e)) {
            continue;
        }
        for url in field("URIs").unwrap_or_default().split_whitespace() {
            let mut c = channel("apt", url, url, file);
            apt_options(&mut c, field);
            out.push(c);
        }
    }
    out
}

/// deb822 paragraphs as (key, value) pairs; continuation lines (an inline
/// Signed-By key block) join their field.
fn stanzas(text: &str) -> Vec<Vec<(String, String)>> {
    let mut out = vec![];
    let mut current: Vec<(String, String)> = vec![];
    for line in text.lines() {
        if line.trim().is_empty() {
            if !current.is_empty() {
                out.push(std::mem::take(&mut current));
            }
        } else if line.starts_with('#') {
        } else if line.starts_with([' ', '\t']) {
            if let Some((_, v)) = current.last_mut() {
                v.push('\n');
                v.push_str(line.trim());
            }
        } else if let Some((k, v)) = line.split_once(':') {
            current.push((k.trim().to_string(), v.trim().to_string()));
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

/// apt preferences: pins of a non-distribution source above the distribution's
/// own 500, or at 1000 and above, where apt also downgrades to the pinned version.
pub fn apt_preferences(file: &str, text: &str) -> Vec<Pin> {
    stanzas(text)
        .into_iter()
        .filter_map(|s| {
            let field = |k: &str| {
                s.iter()
                    .find(|(key, _)| key.eq_ignore_ascii_case(k))
                    .map(|(_, v)| v.clone())
            };
            let priority: i64 = field("Pin-Priority")?.parse().ok()?;
            let pin = field("Pin")?;
            let distro = match pin.split_once(' ') {
                Some(("origin", h)) => official(&format!("https://{}", h.trim_matches('"'))),
                Some(("release", r)) => r
                    .split(',')
                    .any(|kv| matches!(kv.trim(), "o=Debian" | "o=Ubuntu" | "o=Debian Backports")),
                _ => false,
            };
            (priority >= 1000 || (priority > 500 && !distro)).then(|| Pin {
                file: file.into(),
                package: field("Package").unwrap_or_else(|| "*".into()),
                pin,
                priority,
            })
        })
        .collect()
}

/// INI sections as (name, keys), with `[main]` defaults kept separately.
fn ini(text: &str) -> Vec<(String, Vec<(String, String)>)> {
    let mut out: Vec<(String, Vec<(String, String)>)> = vec![];
    for line in text.lines().map(str::trim) {
        if line.starts_with(['#', ';']) {
            continue;
        }
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            out.push((name.trim().to_string(), vec![]));
        } else if let (Some((_, keys)), Some((k, v))) = (out.last_mut(), line.split_once('=')) {
            keys.push((k.trim().to_ascii_lowercase(), v.trim().to_string()));
        }
    }
    out
}

/// Whether a `[main]` section (dnf.conf, zypp.conf) turns package signature
/// checks off for every repository that does not say otherwise.
pub fn main_unsigned(text: &str) -> Option<bool> {
    ini(text)
        .into_iter()
        .filter(|(name, _)| name == "main")
        .flat_map(|(_, keys)| keys)
        .filter(|(k, _)| k == "gpgcheck" || k == "pkg_gpgcheck")
        .map(|(_, v)| falsy(&v))
        .next_back()
}

/// dnf/yum and zypper `.repo` files: sections with `baseurl`, `metalink` or
/// `mirrorlist`. Both managers default to priority 99, lower winning.
pub fn ini_repos(manager: &str, file: &str, text: &str, unsigned_default: bool) -> Vec<Channel> {
    ini(text)
        .into_iter()
        .filter_map(|(name, keys)| {
            let get = |k: &str| {
                keys.iter()
                    .rev()
                    .find(|(key, _)| key == k)
                    .map(|(_, v)| v.as_str())
            };
            if get("enabled").is_some_and(falsy) {
                return None;
            }
            let url = get("baseurl")
                .or(get("metalink"))
                .or(get("mirrorlist"))
                .unwrap_or_default()
                .split_whitespace()
                .next()
                .unwrap_or_default();
            let mut c = channel(manager, &name, url, file);
            c.signed = match get("pkg_gpgcheck").or(get("gpgcheck")) {
                Some(v) => !falsy(v),
                None => !unsigned_default,
            };
            c.overrides = get("priority")
                .and_then(|p| p.parse::<i64>().ok())
                .filter(|p| *p < 99 && !c.official)
                .map(|p| format!("priority={p}, the distribution's is 99"));
            Some(c)
        })
        .collect()
}

/// `pacman.conf`: repositories in the order pacman prefers them, so a
/// third-party repository listed above `[core]` overrides the distribution.
pub fn pacman_conf(file: &str, text: &str) -> Vec<Channel> {
    const ARCH: [&str; 6] = [
        "core",
        "extra",
        "multilib",
        "core-testing",
        "extra-testing",
        "multilib-testing",
    ];
    let mut global = pacman_package_level("Optional", None);
    let mut core_seen = false;
    let mut out: Vec<Channel> = vec![];
    for (name, keys) in ini(text) {
        let siglevel = keys
            .iter()
            .rev()
            .find(|(k, _)| k == "siglevel")
            .map(|(_, v)| v.as_str());
        if name == "options" {
            global = pacman_package_level(siglevel.unwrap_or(""), Some(global));
            continue;
        }
        let url = keys
            .iter()
            .find(|(k, _)| k == "server")
            .map_or("", |(_, v)| v.as_str());
        let mut c = channel("pacman", &name, url, file);
        c.official = ARCH.contains(&name.as_str());
        // Package signatures are required; database signatures are pacman's own concern.
        c.signed = pacman_package_level(siglevel.unwrap_or(""), Some(global)) == "Required";
        core_seen |= name == "core";
        if !c.official && !core_seen {
            c.overrides = Some("listed above [core]".into());
        }
        out.push(c);
    }
    out
}

/// The package part of a pacman SigLevel (`Never`, `Optional` or `Required`).
/// A repository's SigLevel changes only what it names and inherits the rest
/// from `[options]`, whose own unset parts keep pacman's built-in `Optional`.
fn pacman_package_level(siglevel: &str, inherited: Option<&'static str>) -> &'static str {
    let mut level = inherited.unwrap_or("Optional");
    for word in siglevel.split_whitespace() {
        if word.starts_with("Database") {
            continue;
        }
        match word.strip_prefix("Package").unwrap_or(word) {
            "Never" => level = "Never",
            "Optional" => level = "Optional",
            "Required" => level = "Required",
            _ => {}
        }
    }
    level
}

/// `flatpak remotes --columns=name,url,options`, tab-separated.
pub fn flatpak_remotes(text: &str) -> Vec<Channel> {
    text.lines()
        .filter_map(|l| {
            let f: Vec<&str> = l.split('\t').map(str::trim).collect();
            let (name, url) = (*f.first()?, *f.get(1)?);
            let mut c = channel("flatpak", name, url, "flatpak remotes");
            c.signed = !f
                .get(2)
                .is_some_and(|o| o.split(',').any(|o| o == "no-gpg-verify"));
            Some(c)
        })
        .collect()
}

/// `nix.conf` binary caches; `require-sigs = false` accepts unsigned store paths.
pub fn nix_conf(file: &str, text: &str) -> Vec<Channel> {
    let get = |k: &str| {
        text.lines()
            .filter_map(|l| l.split_once('='))
            .filter(|(key, _)| key.trim() == k)
            .map(|(_, v)| v.trim().to_string())
            .next_back()
    };
    let signed = !get("require-sigs").is_some_and(|v| falsy(&v));
    let caches = [get("substituters"), get("extra-substituters")];
    caches
        .iter()
        .flatten()
        .flat_map(|v| v.split_whitespace())
        .map(|url| {
            let mut c = channel("nix", url, url, file);
            c.signed = signed;
            c
        })
        .collect()
}

/// `~/.nix-channels`: `URL name` per line.
pub fn nix_channels(file: &str, text: &str) -> Vec<Channel> {
    text.lines()
        .filter_map(|l| {
            let mut f = l.split_whitespace();
            let url = f.next()?;
            Some(channel("nix", f.next().unwrap_or(url), url, file))
        })
        .collect()
}

/// Metadata older than this hides updates published since: the machine is not
/// refreshing, or a mirror is serving a frozen copy.
const STALE: u64 = 14 * 86_400;

/// The channel rules.
pub fn assess(r: &mut Report, inputs: &Inputs, now: u64) {
    let (mut third, mut personal) = (vec![], vec![]);
    for c in &inputs.channels {
        let evidence = format!("{}\n{} {}", c.file, c.name, c.url);
        if !c.signed {
            r.add(
                "channel.signature-disabled",
                Severity::High,
                &format!("{} installs packages without checking signatures", c.name),
                &evidence,
                "Anyone who controls the server, or the network between, can hand you any package. Turn signature checks back on: remove trusted=yes, set gpgcheck=1, or raise SigLevel to Required.",
            );
        }
        if c.url.starts_with("http://") {
            r.add(
                "channel.plain-http",
                Severity::Info,
                &format!("{} is downloaded over plain HTTP", c.name),
                &evidence,
                "Signatures still protect the packages, but the network can see what you install and replay old metadata. Use the https:// address if the server has one.",
            );
        }
        if c.scoped_key == Some(false) && !c.official {
            r.add(
                "channel.unscoped-key",
                Severity::Review,
                &format!("{} has no signed-by key", c.name),
                &evidence,
                "apt accepts this source's packages signed by any trusted key, and its vendor's key is trusted for every source. Put the vendor's key in /etc/apt/keyrings and add signed-by= to this source.",
            );
        }
        if !c.checks_expiry {
            r.add(
                "channel.expiry-check-off",
                Severity::Review,
                &format!("{} accepts expired metadata", c.name),
                &evidence,
                "With check-valid-until=no, a mirror can keep serving an old snapshot that hides security updates. Remove the option unless this is an archive snapshot you pinned on purpose.",
            );
        }
        if let Some(how) = &c.overrides {
            r.add(
                "channel.priority-override",
                Severity::Review,
                &format!("{} outranks the distribution's own packages", c.name),
                format!("{evidence}\n{how}"),
                "A third-party repository that wins over the distribution can replace system packages with its own builds on the next update. Make sure you mean it.",
            );
        }
        if c.personal {
            personal.push(format!("{} ({}) {}", c.name, c.manager, c.url));
        } else if !c.official {
            third.push(format!("{} ({}) {}", c.name, c.manager, c.url));
        }
    }
    for p in &inputs.pins {
        r.add(
            "channel.priority-override",
            Severity::Review,
            &format!(
                "{} is pinned above the distribution's packages",
                if p.package == "*" { "Every package from a source".to_string() } else { p.package.clone() }
            ),
            format!("{}\nPackage: {}\nPin: {}\nPin-Priority: {}", p.file, p.package, p.pin, p.priority),
            if p.priority >= 1000 {
                "At 1000 or more apt also downgrades to the pinned version. Make sure you mean it."
            } else {
                "This source wins over the distribution for the packages it matches. Make sure you mean it."
            },
        );
    }
    if !inputs.global_keys.is_empty() {
        r.add(
            "channel.global-key",
            Severity::Review,
            &format!("{} trusted by apt for every source", count(inputs.global_keys.len(), "third-party key is", "third-party keys are")),
            inputs.global_keys.join("\n"),
            "A key in trusted.gpg.d can sign packages for any source, Debian's own included. Move each to /etc/apt/keyrings and reference it with signed-by= from its own source.",
        );
    }
    if let Some(m) = inputs
        .metadata
        .as_ref()
        .filter(|m| now.saturating_sub(m.published) > STALE)
    {
        r.add(
            "channel.stale-metadata",
            Severity::Review,
            &format!("Package lists are {} old", age(now.saturating_sub(m.published))),
            &m.source,
            "Updates published since then are not visible. Refresh the package lists (for example dnf check-update or apt update); if they stay old, the mirror may be serving a frozen copy.",
        );
    }
    if !personal.is_empty() {
        r.add(
            "channel.personal-repo",
            Severity::Info,
            &count(personal.len(), "personal repository", "personal repositories"),
            personal.join("\n"),
            "Each is run by one person or project, and the distribution does not review it. Remove the ones you no longer use.",
        );
    }
    if !third.is_empty() {
        r.add(
            "channel.third-party",
            Severity::Info,
            &count(
                third.len(),
                "third-party package source",
                "third-party package sources",
            ),
            third.join("\n"),
            "Each can ship updates for anything it packages. Remove sources you no longer use.",
        );
    }
}

fn count(n: usize, one: &str, many: &str) -> String {
    if n == 1 {
        format!("1 {one}")
    } else {
        format!("{n} {many}")
    }
}

fn files(dir: &Path, suffix: &str) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(dir)
        .map(|d| {
            d.flatten()
                .map(|e| e.path())
                .filter(|p| p.to_string_lossy().ends_with(suffix))
                .collect()
        })
        .unwrap_or_default();
    out.sort();
    out
}

fn read(path: &Path) -> Option<String> {
    read_file(path, MAX_INPUT)
        .ok()
        .map(|b| String::from_utf8_lossy(&b).into_owned())
}

fn modified(path: &Path) -> Option<u64> {
    let t = std::fs::metadata(path).ok()?.modified().ok()?;
    Some(t.duration_since(UNIX_EPOCH).ok()?.as_secs())
}

/// dnf5's configuration files in the order it applies them, the last one winning:
/// drop-ins from the distribution and from `/etc` sorted by name (an `/etc` file
/// masks a distribution file of the same name), then the main file.
fn dnf_config_files(
    etc: &Path,
    distribution: &Path,
    drop_ins: &str,
    suffix: &str,
    main: &[&str],
) -> Vec<PathBuf> {
    let mut by_name: std::collections::BTreeMap<std::ffi::OsString, PathBuf> =
        std::collections::BTreeMap::new();
    for p in files(distribution, suffix)
        .into_iter()
        .chain(files(&etc.join(drop_ins), suffix))
    {
        if let Some(n) = p.file_name() {
            by_name.insert(n.to_owned(), p);
        }
    }
    by_name
        .into_values()
        .chain(main.iter().map(|m| etc.join(m)))
        .collect()
}

/// Whether dnf skips package signatures for repositories that do not say. Its
/// built-in default is not to check (`pkg_gpgcheck` False); distributions turn
/// checks on in dnf.conf or a drop-in.
fn dnf_unsigned(etc: &Path) -> bool {
    dnf_config_files(
        etc,
        Path::new("/usr/share/dnf5/libdnf.conf.d"),
        "dnf/libdnf5.conf.d",
        ".conf",
        &["yum.conf", "dnf/dnf.conf"],
    )
    .iter()
    .filter_map(|p| read(p).and_then(|t| main_unsigned(&t)))
    .next_back()
    .unwrap_or(true)
}

/// dnf5 repository overrides (`repos.override.d`), which change existing
/// repositories by ID or glob: signature checks, priority and enabled.
fn dnf_overrides(etc: &Path, mut channels: Vec<Channel>) -> Vec<Channel> {
    let files = dnf_config_files(
        etc,
        Path::new("/usr/share/dnf5/repos.override.d"),
        "dnf/repos.override.d",
        ".repo",
        &[],
    );
    for file in files {
        let Some(text) = read(&file) else { continue };
        for (pattern, keys) in ini(&text) {
            let get = |k: &str| {
                keys.iter()
                    .rev()
                    .find(|(key, _)| key == k)
                    .map(|(_, v)| v.as_str())
            };
            for c in channels.iter_mut().filter(|c| glob(&pattern, &c.name)) {
                if let Some(v) = get("pkg_gpgcheck").or(get("gpgcheck")) {
                    c.signed = !falsy(v);
                    c.file = file.display().to_string();
                }
                if let Some(p) = get("priority").and_then(|p| p.parse::<i64>().ok()) {
                    c.overrides = (p < 99 && !c.official)
                        .then(|| format!("priority={p}, the distribution's is 99"));
                }
                if get("enabled").is_some_and(falsy) {
                    c.name.clear();
                }
            }
        }
    }
    channels.retain(|c| !c.name.is_empty());
    channels
}

/// Shell-style `*` and `?` matching for dnf repository IDs.
fn glob(pattern: &str, text: &str) -> bool {
    let (p, t): (Vec<char>, Vec<char>) = (pattern.chars().collect(), text.chars().collect());
    let (mut pi, mut ti, mut star, mut mark) = (0, 0, None, 0);
    while ti < t.len() {
        if pi < p.len() && (p[pi] == '?' || p[pi] == t[ti]) {
            pi += 1;
            ti += 1;
        } else if pi < p.len() && p[pi] == '*' {
            star = Some(pi);
            mark = ti;
            pi += 1;
        } else if let Some(s) = star {
            pi = s + 1;
            mark += 1;
            ti = mark;
        } else {
            return false;
        }
    }
    p[pi..].iter().all(|&c| c == '*')
}

/// Keyrings in apt's global trust that no distribution ships.
fn global_keys(etc: &Path) -> Vec<String> {
    const DISTRO: [&str; 3] = ["debian-archive-", "ubuntu-keyring-", "ubuntu-archive-"];
    let mut keys: Vec<String> = files(&etc.join("apt/trusted.gpg.d"), "")
        .into_iter()
        .filter(|p| {
            let n = p.file_name().unwrap_or_default().to_string_lossy();
            (n.ends_with(".gpg") || n.ends_with(".asc")) && !DISTRO.iter().any(|d| n.starts_with(d))
        })
        .map(|p| p.display().to_string())
        .collect();
    let legacy = etc.join("apt/trusted.gpg");
    if std::fs::metadata(&legacy).is_ok_and(|m| m.len() > 0) {
        keys.push(format!(
            "{} (the legacy keyring apt-key wrote to)",
            legacy.display()
        ));
    }
    keys
}

/// The newest publication time of the distribution's own metadata: apt
/// InRelease files and pacman sync databases by mtime (both keep the server's
/// time), dnf by the repomd.xml revision of each official repository.
fn metadata(channels: &[Channel]) -> Option<Metadata> {
    let newest = |found: Vec<(u64, String)>| found.into_iter().max_by_key(|(t, _)| *t);
    let apt = newest(
        files(Path::new("/var/lib/apt/lists"), "Release")
            .into_iter()
            .filter(|p| {
                let n = p
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .replace('_', "/");
                official(&format!("https://{n}"))
            })
            .filter_map(|p| Some((modified(&p)?, p.display().to_string())))
            .collect(),
    );
    let pacman = newest(
        ["core", "extra"]
            .iter()
            .map(|n| PathBuf::from(format!("/var/lib/pacman/sync/{n}.db")))
            .filter_map(|p| Some((modified(&p)?, p.display().to_string())))
            .collect(),
    );
    let ids: Vec<&str> = channels
        .iter()
        .filter(|c| c.manager == "dnf" && c.official)
        .map(|c| c.name.as_str())
        .collect();
    let dnf = newest(
        ["/var/cache/libdnf5", "/var/cache/dnf"]
            .iter()
            .flat_map(|d| files(Path::new(d), ""))
            .filter(|d| {
                let n = d.file_name().unwrap_or_default().to_string_lossy();
                n.rsplit_once('-').is_some_and(|(id, _)| ids.contains(&id))
            })
            .filter_map(|d| {
                let repomd = d.join("repodata/repomd.xml");
                let text = read(&repomd)?;
                let revision = text
                    .split_once("<revision>")?
                    .1
                    .split_once('<')?
                    .0
                    .trim()
                    .parse()
                    .ok()?;
                Some((revision, repomd.display().to_string()))
            })
            .collect(),
    );
    [apt, pacman, dnf]
        .into_iter()
        .flatten()
        .max_by_key(|(t, _)| *t)
        .map(|(published, source)| Metadata { source, published })
}

/// Reads this machine's package sources and applies the rules.
pub fn channels() -> Report {
    let mut r = Report::new("Package sources", "This computer");
    r.coverage.push("Reads package-source configuration only; no repository is contacted. The distribution's own repositories are recognised by host or repository name, so a private mirror of them is listed as third-party.".into());
    let etc = Path::new("/etc");
    let mut inputs = Inputs::default();
    let mut take = |r: &mut Report, label: &str, found: Vec<Channel>| {
        if !found.is_empty() {
            r.coverage
                .checked
                .push(format!("{label} ({})", found.len()));
        }
        inputs.channels.extend(found);
    };
    let each = |dir: &str, suffix: &str, parse: &dyn Fn(&str, &str) -> Vec<Channel>| {
        files(&etc.join(dir), suffix)
            .iter()
            .filter_map(|p| Some(parse(&p.display().to_string(), &read(p)?)))
            .flatten()
            .collect::<Vec<_>>()
    };
    let mut apt = read(&etc.join("apt/sources.list"))
        .map(|t| apt_list("/etc/apt/sources.list", &t))
        .unwrap_or_default();
    apt.extend(each("apt/sources.list.d", ".list", &apt_list));
    apt.extend(each("apt/sources.list.d", ".sources", &apt_deb822));
    take(&mut r, "apt sources", apt);
    let dnf_default = dnf_unsigned(etc);
    take(
        &mut r,
        "dnf repositories",
        dnf_overrides(
            etc,
            each("yum.repos.d", ".repo", &|f, t| {
                ini_repos("dnf", f, t, dnf_default)
            }),
        ),
    );
    let zypp_default = read(&etc.join("zypp/zypp.conf"))
        .and_then(|t| main_unsigned(&t))
        .unwrap_or(false);
    take(
        &mut r,
        "zypper repositories",
        each("zypp/repos.d", ".repo", &|f, t| {
            ini_repos("zypper", f, t, zypp_default)
        }),
    );
    if let Some(t) = read(&etc.join("pacman.conf")) {
        take(
            &mut r,
            "pacman repositories",
            pacman_conf("/etc/pacman.conf", &t),
        );
    }
    if runner::available("flatpak") {
        match runner::run(
            Command::new("flatpak").args(["remotes", "--columns=name,url,options"]),
            30,
            &runner::cancel(),
        ) {
            Ok(out) => take(&mut r, "Flatpak remotes", flatpak_remotes(&out.stdout)),
            Err(e) => r.errors.push(format!("flatpak remotes: {e:#}")),
        }
    }
    if let Some(t) = read(&etc.join("nix/nix.conf")) {
        take(
            &mut r,
            "Nix binary caches",
            nix_conf("/etc/nix/nix.conf", &t),
        );
    }
    let channels_file = home().join(".nix-channels");
    if let Some(t) = read(&channels_file) {
        take(
            &mut r,
            "Nix channels",
            nix_channels(&channels_file.display().to_string(), &t),
        );
    }
    let mut prefs = files(&etc.join("apt/preferences.d"), "");
    prefs.insert(0, etc.join("apt/preferences"));
    inputs.pins = prefs
        .iter()
        .filter_map(|p| Some(apt_preferences(&p.display().to_string(), &read(p)?)))
        .flatten()
        .collect();
    if etc.join("apt").is_dir() {
        inputs.global_keys = global_keys(etc);
    }
    inputs.metadata = metadata(&inputs.channels);
    if let Some(m) = &inputs.metadata {
        r.coverage
            .push(format!("Newest distribution metadata: {}", m.source));
    }
    assess(&mut r, &inputs, now());
    r.findings.sort_by_key(|f| std::cmp::Reverse(f.severity));
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_apt_sources() {
        let one = apt_list(
            "s.list",
            "deb [arch=amd64 signed-by=/etc/apt/keyrings/x.gpg] https://example.org/apt stable main\n# deb http://old stable main\ndeb [trusted=yes check-valid-until=no] http://deb.debian.org/debian trixie main\n",
        );
        assert_eq!(one.len(), 2);
        assert_eq!(
            (one[0].scoped_key, one[0].official, one[0].signed),
            (Some(true), false, true)
        );
        assert_eq!(
            (one[1].official, one[1].signed, one[1].checks_expiry),
            (true, false, false)
        );
        let deb822 = apt_deb822(
            "d.sources",
            "Types: deb\nURIs: http://deb.debian.org/debian http://ftp.de.debian.org/debian\nSuites: trixie\nSigned-By:\n -----BEGIN PGP PUBLIC KEY BLOCK-----\n .\n mQINBF\n\nTypes: deb\nURIs: https://x.example\nEnabled: no\n",
        );
        assert_eq!(
            deb822.len(),
            2,
            "one channel per URI, disabled stanzas skipped"
        );
        assert!(deb822
            .iter()
            .all(|c| c.official && c.scoped_key == Some(true)));
    }

    #[test]
    fn reads_repo_files_and_pins() {
        let dnf = ini_repos(
            "dnf",
            "f.repo",
            "[fedora]\nmetalink=https://mirrors.fedoraproject.org/metalink?repo=fedora-$releasever\ngpgcheck=1\n[off]\nbaseurl=https://x\nenabled=0\n[copr:copr.fedorainfracloud.org:me:tool]\nbaseurl = https://download.copr.fedorainfracloud.org/results/me/tool/fedora-$releasever-$basearch/\npriority=10\n",
            false,
        );
        assert_eq!(dnf.len(), 2);
        assert!(dnf[0].official && dnf[0].signed && dnf[0].overrides.is_none());
        assert!(dnf[1].personal && dnf[1].overrides.is_some());
        assert_eq!(main_unsigned("[main]\ngpgcheck=False\n"), Some(true));
        let obs = channel(
            "zypper",
            "x",
            "https://download.opensuse.org/repositories/home:me/openSUSE_Tumbleweed/",
            "z",
        );
        assert!(obs.personal && !obs.official);
        let pins = apt_preferences(
            "p",
            "Package: *\nPin: origin packages.mozilla.org\nPin-Priority: 1000\n\nPackage: *\nPin: release o=Debian\nPin-Priority: 900\n\nPackage: *\nPin: origin example.org\nPin-Priority: 100\n",
        );
        assert_eq!(pins.len(), 1);
    }

    #[test]
    fn reads_pacman_flatpak_and_nix() {
        let pacman = pacman_conf(
            "pacman.conf",
            "[options]\nSigLevel = Required DatabaseOptional\n[mine]\nSigLevel = Optional TrustAll\nServer = http://repo.example/$arch\n[core]\nInclude = /etc/pacman.d/mirrorlist\n[chaotic-aur]\nInclude = /etc/pacman.d/chaotic-mirrorlist\n",
        );
        assert_eq!(pacman.len(), 3);
        assert!(!pacman[0].signed && pacman[0].overrides.is_some());
        assert!(pacman[1].official && pacman[1].signed);
        assert!(pacman[2].overrides.is_none() && !pacman[2].official);
        let flatpak = flatpak_remotes("fedora\toci+https://registry.fedoraproject.org\tsystem,oci\nflathub\thttps://dl.flathub.org/repo/\tsystem,filtered\nx\thttps://x.example/repo\tuser,no-gpg-verify\n");
        assert_eq!(
            flatpak
                .iter()
                .map(|c| (c.official, c.signed))
                .collect::<Vec<_>>(),
            [(true, true), (true, true), (false, false)]
        );
        let nix = nix_conf(
            "nix.conf",
            "substituters = https://cache.nixos.org https://cachix.example\nrequire-sigs = false\n",
        );
        assert_eq!(nix.len(), 2);
        assert!(nix.iter().all(|c| !c.signed));
        assert!(nix[0].official && !nix[1].official);
    }

    #[test]
    fn pacman_repositories_inherit_unset_parts_of_siglevel() {
        let signed = |conf: &str| {
            pacman_conf("/etc/pacman.conf", conf)
                .into_iter()
                .map(|c| (c.name, c.signed))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            signed("[options]\nSigLevel = PackageNever\n[thirdparty]\nSigLevel = TrustAll\n"),
            [("thirdparty".to_string(), false)]
        );
        assert_eq!(
            signed("[options]\nSigLevel = Required DatabaseOptional\n[core]\n[x]\nSigLevel = DatabaseNever\n"),
            [("core".to_string(), true), ("x".to_string(), true)]
        );
        assert_eq!(
            signed("[core]\n"),
            [("core".to_string(), false)],
            "pacman's built-in level is Optional"
        );
    }

    #[test]
    fn apt_deb_and_deb_src_for_one_source_count_once() {
        let got = apt_list("f", "deb https://x.example/ stable main\ndeb-src https://x.example/ stable main\ndeb-src [trusted=yes] https://x.example/ stable main\n");
        assert_eq!(
            got.len(),
            2,
            "a deb-src with different options stays separate"
        );
    }

    #[test]
    fn dnf_defaults_and_overrides() {
        let t = tempfile::tempdir().unwrap();
        let etc = t.path();
        assert!(
            dnf_unsigned(etc) || Path::new("/usr/share/dnf5/libdnf.conf.d").is_dir(),
            "no configuration means dnf's own default: unchecked"
        );
        std::fs::create_dir_all(etc.join("dnf/libdnf5.conf.d")).unwrap();
        std::fs::write(
            etc.join("dnf/libdnf5.conf.d/99-mine.conf"),
            "[main]\npkg_gpgcheck=0\n",
        )
        .unwrap();
        std::fs::write(etc.join("dnf/dnf.conf"), "[main]\ngpgcheck=1\n").unwrap();
        assert!(!dnf_unsigned(etc), "dnf.conf is applied last");
        std::fs::create_dir_all(etc.join("dnf/repos.override.d")).unwrap();
        std::fs::write(
            etc.join("dnf/repos.override.d/50-x.repo"),
            "[vendor-*]\ngpgcheck=0\n",
        )
        .unwrap();
        let repos = ini_repos("dnf", "f", "[vendor-tools]\nbaseurl=https://v.example/\ngpgcheck=1\n[other]\nbaseurl=https://o.example/\ngpgcheck=1\n", false);
        let got: Vec<_> = dnf_overrides(etc, repos)
            .into_iter()
            .map(|c| (c.name, c.signed))
            .collect();
        assert_eq!(
            got,
            [
                ("vendor-tools".to_string(), false),
                ("other".to_string(), true)
            ]
        );
        assert!(
            glob("copr:*:me:*", "copr:copr.fedorainfracloud.org:me:tool") && !glob("a?c", "abbc")
        );
    }
}
