//! System package files read without installing them (DP-08, DP-09, DP-11,
//! DP-17): `.deb`, `.rpm`, built Arch packages, PKGBUILDs and AppImages.
//! Everything is parsed here: no dpkg, rpm, makepkg or package code runs.
//! Install scripts go through the same line rules as install receipts.
use crate::{
    checks::{
        archive::{self, file_digest, Entry, Group, Kind as EntryKind, Want},
        receipt, shell,
    },
    model::{Report, Severity, SkipReason},
    runner::Cancel,
    store::{read_file, Store, MAX_INPUT},
};
use anyhow::{bail, Context, Result};
use regex::Regex;
use serde::Deserialize;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
    sync::LazyLock,
};

/// Install scripts and control files are read up to this size.
const MAX_SCRIPT: u64 = 1024 * 1024;
/// Script text shown in a finding; the rest is counted.
const SHOWN_LINES: usize = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Deb,
    Rpm,
    Arch,
    Pkgbuild,
    AppImage,
}

pub fn kind_of(path: &Path) -> Option<Kind> {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    if name == "PKGBUILD" {
        return Some(Kind::Pkgbuild);
    }
    let mut head = [0u8; 24];
    let n = File::open(path).and_then(|mut f| f.read(&mut head)).ok()?;
    let head = &head[..n];
    if head.starts_with(b"!<arch>\ndebian-binary") {
        Some(Kind::Deb)
    } else if head.starts_with(&[0xed, 0xab, 0xee, 0xdb]) {
        Some(Kind::Rpm)
    } else if head.starts_with(b"\x7fELF") && head.get(8..11) == Some(b"AI\x02") {
        Some(Kind::AppImage)
    } else if [".pkg.tar.zst", ".pkg.tar.xz", ".pkg.tar.gz"]
        .iter()
        .any(|s| name.ends_with(s))
    {
        Some(Kind::Arch)
    } else {
        None
    }
}

pub fn inspect(store: &Store, path: &Path, stop: &Cancel) -> Result<Report> {
    let r = match kind_of(path) {
        Some(Kind::Pkgbuild) => pkgbuild(path)?,
        Some(Kind::AppImage) => appimage(path)?,
        Some(kind) => {
            let mut r = Report::new("Package file", path.display());
            r.sha256 = Some(file_digest(path)?);
            let p = match kind {
                Kind::Deb => deb(path, stop)?,
                Kind::Rpm => rpm(path)?,
                _ => arch(path, stop)?,
            };
            let files = p.files.iter().filter(|f| f.kind != EntryKind::Dir).count();
            r.coverage.checked(format!(
                "{} and {} ({})",
                count(files, "file"),
                count(p.scripts.len(), "install script"),
                p.format
            ));
            r.coverage.push(notes().into());
            if let Some(why) = &p.truncated {
                r.coverage
                    .skip("Later files", SkipReason::LimitReached, why.clone());
            }
            r.details = p.identity();
            assess(&mut r, &p);
            r
        }
        None => bail!("Not a .deb, .rpm, Arch package, PKGBUILD or AppImage"),
    };
    store.save(&r)?;
    Ok(r)
}

fn count(n: usize, what: &str) -> String {
    format!("{n} {what}{}", if n == 1 { "" } else { "s" })
}

pub fn notes() -> &'static str {
    "Reads the package's description, file list and install scripts without installing it or running anything. Script rules are static: a script that downloads or decodes more code is reported, but what that code does is not. A signature, when present, is noted, not verified."
}

#[derive(Debug, Clone, Deserialize)]
pub struct Script {
    /// preinst, %post, .INSTALL, ...
    pub phase: String,
    pub text: String,
}

/// What a package file says about itself and what installing it would do.
#[derive(Debug, Default, Deserialize)]
pub struct Package {
    pub format: String,
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub maintainer: String,
    #[serde(default)]
    pub homepage: String,
    #[serde(default)]
    pub scripts: Vec<Script>,
    /// Paths are absolute, as installed.
    #[serde(default)]
    pub files: Vec<Entry>,
    /// rpm: the file carries a signature (not checked here).
    #[serde(default)]
    pub signed: Option<bool>,
    #[serde(default)]
    pub truncated: Option<String>,
}

impl Package {
    fn identity(&self) -> String {
        let mut out = format!("Package: {} {}", self.name, self.version);
        for (label, value) in [
            ("Maintainer", &self.maintainer),
            ("Homepage", &self.homepage),
        ] {
            if !value.is_empty() {
                out.push_str(&format!("\n{label}: {value}"));
            }
        }
        if let Some(signed) = self.signed {
            out.push_str(if signed {
                "\nSignature: present (rpm -K checks it against the keys you trust)"
            } else {
                "\nSignature: none"
            });
        }
        out
    }
}

/// Places where an installed file adds a package source, trusts a key, runs at
/// boot or login, or changes how people sign in.
const SOURCE_PATHS: &[&str] = &[
    "/etc/apt/sources.list",
    "/etc/apt/sources.list.d/",
    "/etc/yum.repos.d/",
    "/etc/zypp/repos.d/",
    "/etc/pacman.conf",
    "/etc/pacman.d/",
];
const KEY_PATHS: &[&str] = &[
    "/etc/apt/trusted.gpg",
    "/etc/apt/trusted.gpg.d/",
    "/etc/apt/keyrings/",
    "/usr/share/keyrings/",
    "/etc/pki/rpm-gpg/",
    "/usr/share/pacman/keyrings/",
];
const STARTUP_PATHS: &[&str] = &[
    "/etc/cron.d/",
    "/etc/cron.hourly/",
    "/etc/cron.daily/",
    "/etc/cron.weekly/",
    "/etc/cron.monthly/",
    "/etc/crontab",
    "/var/spool/cron/",
    "/etc/systemd/system/",
    "/etc/systemd/user/",
    "/usr/lib/systemd/system/",
    "/usr/lib/systemd/user/",
    "/lib/systemd/system/",
    "/etc/xdg/autostart/",
    "/etc/profile.d/",
    "/etc/environment",
    "/etc/ld.so.preload",
    "/etc/ld.so.conf.d/",
    "/etc/sudoers.d/",
    "/etc/pam.d/",
    "/etc/security/",
    "/etc/udev/rules.d/",
    "/usr/lib/udev/rules.d/",
    "/lib/udev/rules.d/",
    "/etc/modules-load.d/",
];
const USER_PATHS: &[&str] = &["/home/", "/root/", "/tmp/", "/var/tmp/", "/dev/shm/"];

fn under(path: &str, places: &[&str]) -> bool {
    places.iter().any(|p| {
        if p.ends_with('/') {
            path.starts_with(p) && path.len() > p.len()
        } else {
            path == *p
        }
    })
}

static SOURCE_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(sources\.list\.d/|Dir::Etc::source(list|parts)|yum\.repos\.d/|zypp/repos\.d/|\badd-apt-repository\b|config-manager\s+(--add-repo|addrepo)|\bzypper\s+(ar|addrepo)\b)")
        .expect("valid regex")
});
static KEY_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\bapt-key\s+add\b|Dir::Etc::trusted(parts)?|\brpm(keys)?\s+--import\b|trusted\.gpg|/usr/share/keyrings/|/etc/apt/keyrings/|/etc/pki/rpm-gpg/|\bpacman-key\s+(--add|-a|--lsign-key)\b)")
        .expect("valid regex")
});

pub fn assess(r: &mut Report, p: &Package) {
    let mut sources = Group::default();
    let mut keys = Group::default();
    let mut setuid = Group::default();
    let mut startup = Group::default();
    let mut user = Group::default();
    for s in &p.scripts {
        let lines: Vec<&str> = s.text.lines().collect();
        let mut shown = lines
            .iter()
            .take(SHOWN_LINES)
            .copied()
            .collect::<Vec<_>>()
            .join("\n");
        if lines.len() > SHOWN_LINES {
            shown.push_str(&format!("\n… {} more lines", lines.len() - SHOWN_LINES));
        }
        r.add(
            "pkg.install-script",
            Severity::Review,
            &format!("{} runs a {} script as root", p.name, s.phase),
            format!("{} script:\n{shown}", s.phase),
            "Read the script before installing. It runs with full control of the system, and a signature only says who built the package.",
        );
        let name = format!("{} {} script", p.name, s.phase);
        for (_, f) in receipt::line_findings(&name, &s.text) {
            r.findings.push(f);
        }
        for (n, line) in lines.iter().enumerate() {
            if line.trim_start().starts_with('#') {
                continue;
            }
            let at = || format!("{} script, line {}: {}", s.phase, n + 1, line.trim());
            if SOURCE_LINE.is_match(line) {
                sources.push(at());
            }
            if KEY_LINE.is_match(line) {
                keys.push(at());
            }
        }
    }
    for f in &p.files {
        if f.kind == EntryKind::Dir {
            continue;
        }
        let path = f.name.as_str();
        if under(path, SOURCE_PATHS) {
            sources.push(format!("installs {path}"));
        }
        if under(path, KEY_PATHS) {
            keys.push(format!("installs {path}"));
        }
        if f.kind == EntryKind::File && f.mode & 0o6000 != 0 {
            let bit = if f.mode & 0o4000 != 0 {
                "setuid"
            } else {
                "setgid"
            };
            setuid.push(format!("{path} ({bit})"));
        }
        if under(path, STARTUP_PATHS) {
            startup.push(path.to_string());
        }
        if under(path, USER_PATHS) {
            user.push(path.to_string());
        }
    }
    let groups = [
        (user, "pkg.user-path", Severity::High,
            "The package puts files in home or temporary folders",
            "Packages install into system folders; files placed in a home folder or /tmp are outside what a package manager tracks. Do not install it unless the vendor explains why."),
        (sources, "pkg.adds-source", Severity::Review,
            "Installing it adds a package source",
            "Updates for this and other packages can then come from that source. Check it is the vendor's own, and see Protect > Updates > Package sources after installing."),
        (keys, "pkg.adds-key", Severity::Review,
            "Installing it trusts a new signing key",
            "Packages signed with this key will install without a warning. Check the key belongs to the vendor, and that apt sources using it say signed-by."),
        (setuid, "pkg.setuid", Severity::Review,
            "The package installs programs that run with raised privileges",
            "A setuid or setgid program runs with its owner's rights whoever starts it. Check the package is meant to need it."),
        (startup, "pkg.startup-path", Severity::Review,
            "The package installs files that run at boot or login, or change how people sign in",
            "Services, timers, cron jobs, login scripts, PAM and sudo rules take effect without anything being opened. Check each one is expected for this package."),
    ];
    for (group, rule, severity, summary, action) in groups {
        if !group.0.is_empty() {
            r.add(rule, severity, summary, group.evidence(), action);
        }
    }
    r.findings.sort_by_key(|f| std::cmp::Reverse(f.severity));
}

/// Install-time absolute path from an archive member name like `./usr/bin/x`.
fn absolute(name: &str) -> String {
    format!("/{}", name.trim_start_matches("./").trim_start_matches('/'))
}

/// `ar` members as (name, offset, size).
fn ar_members(file: &mut File) -> Result<Vec<(String, u64, u64)>> {
    let len = file.metadata()?.len();
    let mut out = vec![];
    let mut at = 8;
    while at + 60 <= len && out.len() < 64 {
        let mut header = [0u8; 60];
        file.seek(SeekFrom::Start(at))?;
        file.read_exact(&mut header)?;
        if &header[58..60] != b"`\n" {
            bail!("The .deb is damaged: bad ar header at byte {at}");
        }
        let name = String::from_utf8_lossy(&header[..16])
            .trim_end()
            .trim_end_matches('/')
            .to_string();
        let size: u64 = String::from_utf8_lossy(&header[48..58])
            .trim()
            .parse()
            .context("The .deb is damaged: bad ar member size")?;
        let data = at + 60;
        if data.checked_add(size).is_none_or(|end| end > len) {
            bail!("The .deb is damaged: {name} runs past the end of the file");
        }
        out.push((name, data, size));
        at = data + size + size % 2;
    }
    Ok(out)
}

/// Lists one compressed tar member of a .deb by copying it to a private file.
fn deb_member(
    file: &mut File,
    offset: u64,
    size: u64,
    stop: &Cancel,
    want: &Want,
) -> Result<archive::Listing> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("member");
    file.seek(SeekFrom::Start(offset))?;
    std::io::copy(&mut file.by_ref().take(size), &mut File::create(&path)?)?;
    let format = archive::format_of(&path).context("Unknown compression inside the .deb")?;
    archive::list_with(&path, format, stop, want)
}

const DEB_CONTROL: Want = Want {
    entry: |name| {
        matches!(
            name.trim_start_matches("./"),
            "control" | "preinst" | "postinst" | "prerm" | "postrm" | "config" | "triggers"
        )
    },
    max: MAX_SCRIPT,
};
const NOTHING: Want = Want {
    entry: |_| false,
    max: 0,
};

/// RFC 822-style fields: deb `control`, one per line, continuation lines skipped.
fn field<'a>(text: &'a str, key: &str) -> &'a str {
    text.lines()
        .find_map(|l| l.strip_prefix(key).and_then(|v| v.strip_prefix(':')))
        .map(str::trim)
        .unwrap_or("")
}

fn deb(path: &Path, stop: &Cancel) -> Result<Package> {
    let mut file = File::open(path)?;
    let members = ar_members(&mut file)?;
    let mut p = Package {
        format: "deb".into(),
        ..Default::default()
    };
    let control = members
        .iter()
        .find(|m| m.0.starts_with("control.tar"))
        .context("The .deb has no control archive")?;
    for e in deb_member(&mut file, control.1, control.2, stop, &DEB_CONTROL)?.entries {
        let Some(text) = e.content else { continue };
        match e.name.trim_start_matches("./") {
            "control" => {
                p.name = field(&text, "Package").into();
                p.version = field(&text, "Version").into();
                p.maintainer = field(&text, "Maintainer").into();
                p.homepage = field(&text, "Homepage").into();
            }
            phase => p.scripts.push(Script {
                phase: phase.into(),
                text,
            }),
        }
    }
    let data = members
        .iter()
        .find(|m| m.0.starts_with("data.tar"))
        .context("The .deb has no data archive")?;
    let listing = deb_member(&mut file, data.1, data.2, stop, &NOTHING)?;
    p.truncated = listing.truncated;
    p.files = listing
        .entries
        .into_iter()
        .map(|mut e| {
            e.name = absolute(&e.name);
            e
        })
        .filter(|e| e.name != "/")
        .collect();
    Ok(p)
}

const ARCH_META: Want = Want {
    entry: |name| name == ".PKGINFO" || name == ".INSTALL",
    max: MAX_SCRIPT,
};

fn arch(path: &Path, stop: &Cancel) -> Result<Package> {
    let format = archive::format_of(path).context("Not a compressed tar archive")?;
    let listing = archive::list_with(path, format, stop, &ARCH_META)?;
    let mut p = Package {
        format: "pacman".into(),
        truncated: listing.truncated,
        ..Default::default()
    };
    for mut e in listing.entries {
        let meta = e.name.starts_with('.') && !e.name.contains('/');
        if !meta {
            e.name = absolute(&e.name);
            p.files.push(e);
            continue;
        }
        match (e.name.as_str(), e.content) {
            (".PKGINFO", Some(text)) => {
                let get = |key: &str| {
                    text.lines()
                        .find_map(|l| l.strip_prefix(key)?.trim_start().strip_prefix('='))
                        .map(|v| v.trim().to_string())
                        .unwrap_or_default()
                };
                p.name = get("pkgname");
                p.version = get("pkgver");
                p.maintainer = get("packager");
                p.homepage = get("url");
            }
            (".INSTALL", Some(text)) => p.scripts.push(Script {
                phase: ".INSTALL".into(),
                text,
            }),
            _ => {}
        }
    }
    Ok(p)
}

/// One rpm header: tag -> (type, offset, count) and the data store after it.
struct Header {
    index: Vec<(u32, u32, u32, u32)>,
    store: Vec<u8>,
}

fn be32(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

impl Header {
    /// Reads a header at the file's current position; returns it and its size.
    fn read(file: &mut File) -> Result<(Header, u64)> {
        let mut intro = [0u8; 16];
        file.read_exact(&mut intro)?;
        if intro[..3] != [0x8e, 0xad, 0xe8] {
            bail!("The .rpm is damaged: bad header magic");
        }
        let count = be32(&intro, 8).unwrap_or(0) as u64;
        let size = be32(&intro, 12).unwrap_or(0) as u64;
        if count > 100_000 || size > 64 * 1024 * 1024 {
            bail!("The .rpm header is implausibly large");
        }
        let mut index = vec![0u8; (count * 16) as usize];
        file.read_exact(&mut index)?;
        let mut store = vec![0u8; size as usize];
        file.read_exact(&mut store)?;
        let index = index
            .as_chunks::<16>()
            .0
            .iter()
            .map(|e| {
                (
                    be32(e, 0).unwrap_or(0),
                    be32(e, 4).unwrap_or(0),
                    be32(e, 8).unwrap_or(0),
                    be32(e, 12).unwrap_or(0),
                )
            })
            .collect();
        Ok((Header { index, store }, 16 + count * 16 + size))
    }
    fn has(&self, tag: u32) -> bool {
        self.index.iter().any(|e| e.0 == tag)
    }
    fn entry(&self, tag: u32) -> Option<(u32, usize, usize)> {
        self.index
            .iter()
            .find(|e| e.0 == tag)
            .map(|e| (e.1, e.2 as usize, e.3 as usize))
    }
    /// STRING, STRING_ARRAY and I18NSTRING values, NUL-separated.
    fn strings(&self, tag: u32) -> Vec<String> {
        let Some((kind, offset, count)) = self.entry(tag) else {
            return vec![];
        };
        if !matches!(kind, 6 | 8 | 9) {
            return vec![];
        }
        let count = if kind == 6 { 1 } else { count };
        self.store
            .get(offset..)
            .unwrap_or_default()
            .split(|b| *b == 0)
            .take(count.min(1_000_000))
            .map(|s| String::from_utf8_lossy(s).into_owned())
            .collect()
    }
    fn string(&self, tag: u32) -> String {
        self.strings(tag).into_iter().next().unwrap_or_default()
    }
    /// INT16 and INT32 arrays.
    fn ints(&self, tag: u32) -> Vec<u32> {
        let Some((kind, offset, count)) = self.entry(tag) else {
            return vec![];
        };
        let width = match kind {
            3 => 2,
            4 => 4,
            _ => return vec![],
        };
        let Some(bytes) = self
            .store
            .get(offset..offset.saturating_add(count.saturating_mul(width)))
        else {
            return vec![];
        };
        bytes
            .chunks_exact(width)
            .map(|c| {
                if width == 2 {
                    u16::from_be_bytes([c[0], c[1]]) as u32
                } else {
                    u32::from_be_bytes([c[0], c[1], c[2], c[3]])
                }
            })
            .collect()
    }
}

/// Script tags and the interpreter tags that go with them.
const RPM_SCRIPTS: &[(&str, u32, u32)] = &[
    ("%pretrans", 1151, 1153),
    ("%pre", 1023, 1085),
    ("%post", 1024, 1086),
    ("%preun", 1025, 1087),
    ("%postun", 1026, 1088),
    ("%posttrans", 1152, 1154),
];

fn rpm(path: &Path) -> Result<Package> {
    let mut file = File::open(path)?;
    // A 96-byte lead, the signature header padded to 8 bytes, then the header.
    file.seek(SeekFrom::Start(96))?;
    let (signature, size) = Header::read(&mut file)?;
    file.seek(SeekFrom::Start(96 + size.div_ceil(8) * 8))?;
    let (h, _) = Header::read(&mut file)?;
    let mut p = Package {
        format: "rpm".into(),
        name: h.string(1000),
        version: format!("{}-{}", h.string(1001), h.string(1002)),
        maintainer: [h.string(1011), h.string(1015)]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(", "),
        homepage: h.string(1020),
        // RSA, DSA, PGP, GPG and OpenPGP signature tags.
        signed: Some(
            [267, 268, 278, 1002, 1005]
                .iter()
                .any(|t| signature.has(*t)),
        ),
        ..Default::default()
    };
    for (phase, body, program) in RPM_SCRIPTS {
        let text = h.string(*body);
        let program = h.strings(*program).join(" ");
        if !text.trim().is_empty() {
            p.scripts.push(Script {
                phase: (*phase).into(),
                text,
            });
        } else if !program.is_empty() {
            p.scripts.push(Script {
                phase: (*phase).into(),
                text: format!("(runs {program})"),
            });
        }
    }
    for (tag, phase) in [
        (1065, "trigger"),
        (5066, "file trigger"),
        (5076, "transaction file trigger"),
    ] {
        for text in h.strings(tag).into_iter().filter(|t| !t.trim().is_empty()) {
            p.scripts.push(Script {
                phase: phase.into(),
                text,
            });
        }
    }
    let modes = h.ints(1030);
    let links = h.strings(1036);
    let names: Vec<String> = if h.has(1117) {
        let dirs = h.strings(1118);
        let index = h.ints(1116);
        h.strings(1117)
            .into_iter()
            .enumerate()
            .map(|(i, base)| {
                let dir = index.get(i).and_then(|d| dirs.get(*d as usize));
                format!("{}{base}", dir.map(String::as_str).unwrap_or(""))
            })
            .collect()
    } else {
        h.strings(1027)
    };
    p.files = names
        .into_iter()
        .enumerate()
        .map(|(i, name)| {
            let mode = modes.get(i).copied().unwrap_or(0);
            Entry {
                name,
                kind: match mode & 0o170000 {
                    0o040000 => EntryKind::Dir,
                    0o120000 => EntryKind::Symlink,
                    0o020000 | 0o060000 => EntryKind::Device,
                    0o010000 => EntryKind::Fifo,
                    _ => EntryKind::File,
                },
                target: links.get(i).cloned().unwrap_or_default(),
                mode,
                size: 0,
                content: None,
            }
        })
        .collect();
    Ok(p)
}

/// Bash arrays such as `source=(...)`, read as text: name -> words. Quoting is
/// honoured; variables are left as written.
fn arrays(text: &str) -> Vec<(String, Vec<String>)> {
    static START: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?m)^\s*([A-Za-z_][A-Za-z0-9_]*)=\(").expect("valid regex"));
    let mut out = vec![];
    for cap in START.captures_iter(text) {
        let name = cap[1].to_string();
        let body = &text[cap.get(0).unwrap().end()..];
        let mut words = vec![];
        let mut word = String::new();
        let mut quote = None;
        let mut chars = body.chars();
        let mut comment = false;
        while let Some(c) = chars.next() {
            if comment {
                comment = c != '\n';
                continue;
            }
            match (quote, c) {
                (None, ')') => break,
                (None, '#') if word.is_empty() => comment = true,
                (None, '\'' | '"') => quote = Some(c),
                (Some(q), c) if c == q => quote = None,
                (None, '\\') | (Some('"'), '\\') => {
                    if let Some(next) = chars.next() {
                        if next != '\n' {
                            word.push(next);
                        }
                    }
                }
                (None, c) if c.is_whitespace() => {
                    if !word.is_empty() {
                        words.push(std::mem::take(&mut word));
                    }
                }
                (_, c) => word.push(c),
            }
        }
        if !word.is_empty() {
            words.push(word);
        }
        out.push((name, words));
    }
    out
}

/// A scalar assignment such as `install=foo.install`.
fn scalar(text: &str, key: &str) -> Option<String> {
    text.lines().find_map(|l| {
        let v = l.trim_start().strip_prefix(key)?.strip_prefix('=')?;
        let v = v.split('#').next().unwrap_or("").trim();
        Some(v.trim_matches(['\'', '"']).to_string()).filter(|v| !v.is_empty())
    })
}

const SUMS: &[&str] = &[
    "cksums",
    "md5sums",
    "sha1sums",
    "sha224sums",
    "sha256sums",
    "sha384sums",
    "sha512sums",
    "b2sums",
];

fn vcs(source: &str) -> bool {
    let url = source.split_once("::").map_or(source, |(_, u)| u);
    ["git+", "svn+", "hg+", "bzr+", "fossil+"]
        .iter()
        .any(|p| url.starts_with(p))
}

/// Variables a PKGBUILD's metadata can name, such as `install=$pkgname.install`.
fn expand(text: &str, value: &str) -> String {
    let mut out = value.to_string();
    for key in ["pkgbase", "pkgname", "pkgver", "pkgrel"] {
        if let Some(v) = scalar(text, key) {
            out = out
                .replace(&format!("${{{key}}}"), &v)
                .replace(&format!("${key}"), &v);
        }
    }
    out
}

/// Metadata that is data, not code: package lists, checksums, descriptions.
const METADATA: &[&str] = &[
    "pkgname",
    "pkgbase",
    "pkgver",
    "pkgrel",
    "epoch",
    "pkgdesc",
    "url",
    "license",
    "arch",
    "groups",
    "depends",
    "makedepends",
    "checkdepends",
    "optdepends",
    "provides",
    "conflicts",
    "replaces",
    "backup",
    "options",
    "install",
    "changelog",
    "noextract",
    "validpgpkeys",
    "source",
    "cksums",
    "md5sums",
    "sha1sums",
    "sha224sums",
    "sha256sums",
    "sha384sums",
    "sha512sums",
    "b2sums",
];

/// The PKGBUILD with metadata assignments blanked, line numbers kept, so the
/// installer rules see only what runs.
fn code(text: &str) -> String {
    let mut out = vec![];
    let mut depth: Option<i32> = None;
    for line in text.lines() {
        if let Some(d) = depth.as_mut() {
            *d += line.matches('(').count() as i32 - line.matches(')').count() as i32;
            if *d <= 0 {
                depth = None;
            }
            out.push("");
            continue;
        }
        let key = line
            .trim_start()
            .split(['=', '+'])
            .next()
            .unwrap_or("")
            .trim();
        let base = key.split('_').next().unwrap_or(key);
        let assignment = line.contains('=') && !key.contains(char::is_whitespace);
        if assignment && METADATA.contains(&base) {
            let open = line.matches('(').count() as i32 - line.matches(')').count() as i32;
            if open > 0 {
                depth = Some(open);
            }
            out.push("");
        } else {
            out.push(line);
        }
    }
    out.join("\n")
}

/// The PKGBUILD rules over its text and, when given, its install file.
pub fn assess_pkgbuild(r: &mut Report, text: &str, install: Option<(&str, &str)>) {
    let scan = shell::analyze_content(&code(text));
    // Removing and installing under $pkgdir or $srcdir is what packaging is.
    let packaging = |f: &crate::model::Finding| {
        matches!(f.rule.as_str(), "shell.delete" | "shell.persistence")
            && (f.evidence.contains("pkgdir") || f.evidence.contains("srcdir"))
    };
    for mut f in scan.findings.into_iter().filter(|f| !packaging(f)) {
        f.evidence = format!("PKGBUILD, {}", f.evidence.replacen("Line", "line", 1));
        r.findings.push(f);
    }
    r.errors.extend(scan.errors);
    let arrays = arrays(text);
    let mut skipped = Group::default();
    for (name, sources) in arrays.iter().filter(|(n, _)| n.starts_with("source")) {
        let suffix = &name["source".len()..];
        for (sums, values) in arrays
            .iter()
            .filter(|(n, _)| SUMS.iter().any(|s| n.strip_prefix(s) == Some(suffix)))
        {
            for (source, sum) in sources.iter().zip(values) {
                let signature = [".sig", ".asc", ".sign", ".gpg"]
                    .iter()
                    .any(|e| source.ends_with(e));
                if sum == "SKIP" && source.contains("://") && !vcs(source) && !signature {
                    skipped.push(format!("{source} ({sums} is SKIP)"));
                }
            }
        }
    }
    if !skipped.0.is_empty() {
        r.add(
            "pkgbuild.skip-checksum",
            Severity::Review,
            "A download is not checked against a checksum",
            skipped.evidence(),
            "SKIP means makepkg builds with whatever the server returns that day. Ask the maintainer for a real checksum, or check the file yourself before building.",
        );
    }
    if let Some((name, script)) = install {
        let p = Package {
            name: scalar(text, "pkgname").unwrap_or_else(|| "The package".into()),
            scripts: vec![Script {
                phase: name.into(),
                text: script.into(),
            }],
            ..Default::default()
        };
        assess(r, &p);
    }
    r.findings.sort_by_key(|f| std::cmp::Reverse(f.severity));
}

fn pkgbuild(path: &Path) -> Result<Report> {
    let bytes = read_file(path, MAX_INPUT)?;
    let text = String::from_utf8_lossy(&bytes).into_owned();
    let mut r = Report::new("PKGBUILD check", path.display());
    r.sha256 = Some(crate::store::digest(&bytes));
    r.coverage.checked("PKGBUILD");
    let install = scalar(&text, "install").map(|i| expand(&text, &i));
    let mut script = None;
    if let Some(name) = &install {
        let file = path.with_file_name(name.trim_start_matches("./"));
        match read_file(&file, MAX_SCRIPT) {
            Ok(b) => {
                r.coverage.checked(name.clone());
                script = Some((name.clone(), String::from_utf8_lossy(&b).into_owned()));
            }
            Err(e) => r
                .coverage
                .skip(name.clone(), SkipReason::Unreadable, format!("{e:#}")),
        }
    }
    r.coverage.push("Reads the PKGBUILD as text; makepkg is not run and the file is not sourced, so values built at run time are not resolved. Files listed in source= are not downloaded.".into());
    assess_pkgbuild(
        &mut r,
        &text,
        script.as_ref().map(|(n, t)| (n.as_str(), t.as_str())),
    );
    r.details = crate::model::clean(&text);
    Ok(r)
}

/// ELF sections by name, as (name, bytes), for the few an AppImage uses.
fn elf_sections(file: &mut File, wanted: &[&str]) -> Result<Vec<(String, Vec<u8>)>> {
    let mut ident = [0u8; 64];
    file.seek(SeekFrom::Start(0))?;
    file.read_exact(&mut ident)?;
    if ident[5] != 1 {
        bail!("Only little-endian AppImages are read");
    }
    let wide = ident[4] == 2;
    let le16 = |b: &[u8], at: usize| u16::from_le_bytes([b[at], b[at + 1]]) as u64;
    let le32 = |b: &[u8], at: usize| u32::from_le_bytes(b[at..at + 4].try_into().unwrap()) as u64;
    let le64 = |b: &[u8], at: usize| u64::from_le_bytes(b[at..at + 8].try_into().unwrap());
    let (shoff, entsize, count, strndx) = if wide {
        (
            le64(&ident, 0x28),
            le16(&ident, 0x3a),
            le16(&ident, 0x3c),
            le16(&ident, 0x3e),
        )
    } else {
        (
            le32(&ident, 0x20),
            le16(&ident, 0x2e),
            le16(&ident, 0x30),
            le16(&ident, 0x32),
        )
    };
    if count == 0 || count > 4096 || entsize < if wide { 64 } else { 40 } {
        bail!("The AppImage's section table is damaged");
    }
    let mut table = vec![0u8; (entsize * count) as usize];
    file.seek(SeekFrom::Start(shoff))?;
    file.read_exact(&mut table)?;
    let section = |i: u64| {
        let s = &table[(i * entsize) as usize..];
        if wide {
            (le32(s, 0), le64(s, 0x18), le64(s, 0x20))
        } else {
            (le32(s, 0), le32(s, 0x10), le32(s, 0x14))
        }
    };
    let mut read = |offset: u64, size: u64| -> Result<Vec<u8>> {
        let mut b = vec![0u8; size.min(1024 * 1024) as usize];
        file.seek(SeekFrom::Start(offset))?;
        file.read_exact(&mut b)?;
        Ok(b)
    };
    let (_, str_offset, str_size) = section(strndx);
    let names = read(str_offset, str_size)?;
    let mut out = vec![];
    for i in 0..count {
        let (name_at, offset, size) = section(i);
        let name = names
            .get(name_at as usize..)
            .and_then(|n| n.split(|b| *b == 0).next())
            .map(|n| String::from_utf8_lossy(n).into_owned())
            .unwrap_or_default();
        if wanted.contains(&name.as_str()) {
            out.push((name, read(offset, size)?));
        }
    }
    Ok(out)
}

/// The AppImage rules over its signature and update sections.
pub fn assess_appimage(r: &mut Report, sections: &[(String, Vec<u8>)]) {
    let get = |name: &str| {
        sections
            .iter()
            .find(|s| s.0 == name)
            .map(|s| s.1.as_slice())
    };
    let signed = get(".sha256_sig").is_some_and(|b| b.iter().any(|x| *x != 0));
    if !signed {
        r.add(
            "appimage.unsigned",
            Severity::Info,
            "The AppImage is not signed",
            "No embedded signature (.sha256_sig is missing or empty).",
            "Most AppImages are unsigned. Download it only from the project's own site, and compare its checksum with the one published there.",
        );
    }
    if let Some(update) = get(".upd_info") {
        let text = String::from_utf8_lossy(update)
            .trim_matches(char::from(0))
            .trim()
            .to_string();
        if !text.is_empty() {
            r.add(
                "appimage.update-url",
                Severity::Info,
                "The AppImage can update itself",
                format!("Update information: {text}"),
                "An update tool replaces the file with whatever that address serves. Check the address belongs to the project.",
            );
        }
    }
}

fn appimage(path: &Path) -> Result<Report> {
    let mut r = Report::new("AppImage check", path.display());
    r.sha256 = Some(file_digest(path)?);
    let sections = elf_sections(
        &mut File::open(path)?,
        &[".sha256_sig", ".sig_key", ".upd_info"],
    )?;
    r.coverage.checked("AppImage sections");
    r.coverage.push("Reads the AppImage's signature and update sections. Running it gives the program your full permissions, without a sandbox.".into());
    r.coverage.skip(
        "Files inside",
        SkipReason::NotApplicable,
        "The program's own files (its squashfs image) are not listed yet.",
    );
    assess_appimage(&mut r, &sections);
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn rules(r: &Report) -> Vec<&str> {
        r.findings.iter().map(|f| f.rule.as_str()).collect()
    }

    fn tar_gz(files: &[(&str, u32, &[u8])]) -> Vec<u8> {
        let mut tar = tar::Builder::new(flate2::write::GzEncoder::new(
            vec![],
            flate2::Compression::default(),
        ));
        for (name, mode, body) in files {
            let mut h = tar::Header::new_gnu();
            h.set_size(body.len() as u64);
            h.set_mode(*mode);
            h.set_path(name).unwrap();
            h.set_cksum();
            tar.append(&h, *body).unwrap();
        }
        tar.into_inner().unwrap().finish().unwrap()
    }

    fn ar(path: &Path, members: &[(&str, Vec<u8>)]) {
        let mut out = b"!<arch>\n".to_vec();
        for (name, body) in members {
            out.extend_from_slice(
                format!(
                    "{name:<16}{:<12}{:<6}{:<6}{:<8}{:<10}`\n",
                    0,
                    0,
                    0,
                    100644,
                    body.len()
                )
                .as_bytes(),
            );
            out.extend_from_slice(body);
            if body.len() % 2 == 1 {
                out.push(b'\n');
            }
        }
        std::fs::write(path, out).unwrap();
    }

    #[test]
    fn a_deb_is_read_without_dpkg() {
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join("fixture_1.0_amd64.deb");
        let control = tar_gz(&[
            ("./control", 0o644, b"Package: fixture\nVersion: 1.0\nMaintainer: Someone <a@example.invalid>\nHomepage: https://example.invalid\n"),
            ("./postinst", 0o755, b"#!/bin/sh\necho fixture\n"),
        ]);
        let data = tar_gz(&[
            ("./etc/cron.d/fixture", 0o644, b"* * * * * root true\n"),
            (
                "./etc/apt/sources.list.d/fixture.list",
                0o644,
                b"deb https://example.invalid stable main\n",
            ),
            ("./usr/bin/fixture", 0o4755, b"x"),
        ]);
        ar(
            &path,
            &[
                ("debian-binary", b"2.0\n".to_vec()),
                ("control.tar.gz", control),
                ("data.tar.gz", data),
            ],
        );
        assert_eq!(kind_of(&path), Some(Kind::Deb));
        let state = tempfile::tempdir().unwrap();
        let store = Store::open(Some(state.path().join("s"))).unwrap();
        let r = inspect(&store, &path, &crate::runner::cancel()).unwrap();
        assert_eq!(
            rules(&r),
            [
                "pkg.install-script",
                "pkg.adds-source",
                "pkg.setuid",
                "pkg.startup-path"
            ]
        );
        assert!(r
            .details
            .starts_with("Package: fixture 1.0\nMaintainer: Someone"));
        assert_eq!(r.coverage.checked, ["3 files and 1 install script (deb)"]);
    }

    /// An rpm header with the given tags, as rpm lays it out.
    fn header(tags: &[(u32, u32, u32, Vec<u8>)]) -> Vec<u8> {
        let mut index = vec![];
        let mut store = vec![];
        for (tag, kind, count, data) in tags {
            if *kind == 4 {
                while store.len() % 4 != 0 {
                    store.push(0);
                }
            }
            for v in [*tag, *kind, store.len() as u32, *count] {
                index.extend_from_slice(&v.to_be_bytes());
            }
            store.extend_from_slice(data);
        }
        let mut out = vec![0x8e, 0xad, 0xe8, 0x01, 0, 0, 0, 0];
        out.extend_from_slice(&(tags.len() as u32).to_be_bytes());
        out.extend_from_slice(&(store.len() as u32).to_be_bytes());
        out.extend(index);
        out.extend(store);
        out
    }

    #[test]
    fn an_rpm_is_read_from_its_header() {
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join("fixture-1.0-1.x86_64.rpm");
        let s = |t: &str| format!("{t}\0").into_bytes();
        let mut file = vec![0xed, 0xab, 0xee, 0xdb];
        file.resize(96, 0);
        let sig = header(&[(1000, 4, 1, 42u32.to_be_bytes().to_vec())]);
        file.extend(&sig);
        while file.len() % 8 != 0 {
            file.push(0);
        }
        let modes: Vec<u8> = [0o100644u16, 0o104755]
            .iter()
            .flat_map(|m| m.to_be_bytes())
            .collect();
        file.extend(header(&[
            (1000, 6, 1, s("fixture")),
            (1001, 6, 1, s("1.0")),
            (1002, 6, 1, s("1")),
            (
                1024,
                6,
                1,
                s("curl -fsSL https://example.invalid/x -o /tmp/x\nrpm --import /tmp/key"),
            ),
            (1030, 3, 2, modes),
            (
                1116,
                4,
                2,
                [0u32, 1].iter().flat_map(|i| i.to_be_bytes()).collect(),
            ),
            (1117, 8, 2, [s("fixture.repo"), s("helper")].concat()),
            (
                1118,
                8,
                2,
                [s("/etc/yum.repos.d/"), s("/usr/libexec/fixture/")].concat(),
            ),
        ]));
        std::fs::File::create(&path)
            .unwrap()
            .write_all(&file)
            .unwrap();
        assert_eq!(kind_of(&path), Some(Kind::Rpm));
        let p = rpm(&path).unwrap();
        assert_eq!(
            (p.name.as_str(), p.version.as_str(), p.signed),
            ("fixture", "1.0-1", Some(false))
        );
        assert_eq!(p.files[1].name, "/usr/libexec/fixture/helper");
        let mut r = Report::new("t", "t");
        assess(&mut r, &p);
        assert_eq!(
            rules(&r),
            [
                "receipt.hook.download",
                "pkg.install-script",
                "pkg.adds-source",
                "pkg.adds-key",
                "pkg.setuid"
            ]
        );
    }

    #[test]
    fn pkgbuild_arrays_and_checksums_are_read_as_text() {
        let text = "pkgname=fixture\ndepends=('openssl'\n         'socat')\nsource=(\"fixture::git+https://example.invalid/f.git\"\n        'https://example.invalid/patches.tar.gz' # patches\n        local.patch)\nsha256sums=('SKIP'\n            'SKIP'\n            'SKIP')\ninstall=fixture.install\n";
        let mut r = Report::new("t", "t");
        assess_pkgbuild(
            &mut r,
            text,
            Some(("fixture.install", "post_install() {\n  echo fixture\n}\n")),
        );
        assert_eq!(rules(&r), ["pkgbuild.skip-checksum", "pkg.install-script"]);
        assert_eq!(
            r.findings[0].evidence,
            "https://example.invalid/patches.tar.gz (sha256sums is SKIP)"
        );
        assert_eq!(
            r.findings[1].summary,
            "fixture runs a fixture.install script as root"
        );
        assert_eq!(expand(text, "$pkgname.install"), "fixture.install");
        let packaging = "package() {\n  rm -rf \"$pkgdir/usr/share/doc\"\n  find \"$pkgdir\" -type d -exec chmod 755 {} +\n}\n";
        let mut r = Report::new("t", "t");
        assess_pkgbuild(&mut r, packaging, None);
        assert!(r.findings.is_empty(), "{:?}", rules(&r));
    }

    #[test]
    fn an_appimage_reports_its_signature_and_update_address() {
        // A minimal ELF64 with a section-name table and an update section.
        let names = b"\0.shstrtab\0.upd_info\0";
        let update = b"zsync|https://example.invalid/x.zsync\0\0\0";
        let mut f = vec![0u8; 64];
        f[..4].copy_from_slice(b"\x7fELF");
        f[4] = 2;
        f[5] = 1;
        f[8..11].copy_from_slice(b"AI\x02");
        let names_at = f.len() as u64;
        f.extend_from_slice(names);
        let update_at = f.len() as u64;
        f.extend_from_slice(update);
        let shoff = f.len() as u64;
        f[0x28..0x30].copy_from_slice(&shoff.to_le_bytes());
        f[0x3a..0x3c].copy_from_slice(&64u16.to_le_bytes());
        f[0x3c..0x3e].copy_from_slice(&3u16.to_le_bytes());
        f[0x3e..0x40].copy_from_slice(&1u16.to_le_bytes());
        for (name, offset, size) in [
            (0u32, 0u64, 0u64),
            (1, names_at, names.len() as u64),
            (11, update_at, update.len() as u64),
        ] {
            let mut s = vec![0u8; 64];
            s[..4].copy_from_slice(&name.to_le_bytes());
            s[0x18..0x20].copy_from_slice(&offset.to_le_bytes());
            s[0x20..0x28].copy_from_slice(&size.to_le_bytes());
            f.extend(s);
        }
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join("Tool.AppImage");
        std::fs::write(&path, f).unwrap();
        assert_eq!(kind_of(&path), Some(Kind::AppImage));
        let r = appimage(&path).unwrap();
        assert_eq!(rules(&r), ["appimage.unsigned", "appimage.update-url"]);
        assert_eq!(
            r.findings[1].evidence,
            "Update information: zsync|https://example.invalid/x.zsync"
        );
    }
}
