//! Archives listed without extracting (DS-13): zip, and tar plain or compressed
//! with gzip, xz, zstd or bzip2. Names, link targets, modes and sizes are read;
//! no entry is written to disk. Launchers inside go through the `desktop` rules.
use crate::{
    checks::desktop,
    model::{Report, Severity, SkipReason},
    runner::{self, Cancel},
};
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
    process::Stdio,
    sync::atomic::Ordering,
};

const MAX_ENTRIES: usize = 100_000;
/// How much of a compressed tar stream is read before the listing stops.
const MAX_STREAM: u64 = 2 * 1024 * 1024 * 1024;
/// Declared expansion at or above this ratio and size reads as a bomb.
const BOMB_RATIO: u64 = 100;
const BOMB_SIZE: u64 = 64 * 1024 * 1024;
/// Findings of one kind show this many entries, then a count.
const EXAMPLES: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Zip,
    Tar,
    TarGz,
    TarXz,
    TarZst,
    TarBz2,
    /// Recognised, not listed: 7z, RAR.
    Unsupported(&'static str),
}

impl Format {
    fn label(self) -> &'static str {
        match self {
            Format::Zip => "zip",
            Format::Tar => "tar",
            Format::TarGz => "tar.gz",
            Format::TarXz => "tar.xz",
            Format::TarZst => "tar.zst",
            Format::TarBz2 => "tar.bz2",
            Format::Unsupported(name) => name,
        }
    }
}

/// The format from the first bytes; the file name does not count.
pub fn format_of(path: &Path) -> Option<Format> {
    format_of_head(&head(&mut File::open(path).ok()?))
}

/// Up to the first 512 bytes the reader gives.
fn head(reader: &mut impl Read) -> Vec<u8> {
    let mut head = vec![0u8; 512];
    let mut n = 0;
    while n < head.len() {
        match reader.read(&mut head[n..]) {
            Ok(0) | Err(_) => break,
            Ok(read) => n += read,
        }
    }
    head.truncate(n);
    head
}

fn format_of_head(head: &[u8]) -> Option<Format> {
    Some(match head {
        [b'P', b'K', 3, 4, ..] | [b'P', b'K', 5, 6, ..] => Format::Zip,
        [0x1f, 0x8b, ..] => Format::TarGz,
        [0xfd, b'7', b'z', b'X', b'Z', 0, ..] => Format::TarXz,
        [0x28, 0xb5, 0x2f, 0xfd, ..] => Format::TarZst,
        [b'B', b'Z', b'h', ..] => Format::TarBz2,
        [b'7', b'z', 0xbc, 0xaf, 0x27, 0x1c, ..] => Format::Unsupported("7z"),
        [b'R', b'a', b'r', b'!', 0x1a, 0x07, ..] => Format::Unsupported("RAR"),
        _ if head.len() >= 262 && &head[257..262] == b"ustar" => Format::Tar,
        _ => return None,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    File,
    Dir,
    Symlink,
    Hardlink,
    Device,
    Fifo,
    Other,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    pub name: String,
    pub kind: Kind,
    /// Where a symlink points, or the entry a hardlink repeats.
    #[serde(default)]
    pub target: String,
    #[serde(default)]
    pub mode: u32,
    /// Size once extracted, as the archive declares it.
    #[serde(default)]
    pub size: u64,
    /// The text of a small `.desktop` entry, for the launcher rules.
    #[serde(default)]
    pub content: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Listing {
    /// Size of the archive file itself.
    pub compressed: u64,
    pub entries: Vec<Entry>,
    /// The listing stopped at a limit; later entries were not seen.
    #[serde(default)]
    pub truncated: Option<String>,
}

/// Inspect's archive check: list, then apply the rules.
pub fn check(path: &Path, stop: &Cancel) -> Result<(Report, Listing)> {
    check_with(path, stop, &LAUNCHERS)
}

/// The archive check, keeping the text of the entries `want` names as well, for
/// inspectors that look further at particular archives (npm, wheels, crates).
pub fn check_with(path: &Path, stop: &Cancel, want: &Want) -> Result<(Report, Listing)> {
    let mut r = Report::new("Archive check", path.display());
    let Some(format) = format_of(path) else {
        std::fs::metadata(path).with_context(|| format!("Cannot read {}", path.display()))?;
        bail!("Not a zip or tar archive");
    };
    if let Format::Unsupported(name) = format {
        r.coverage.skip(
            format!("{name} archive"),
            SkipReason::NotApplicable,
            format!("{name} archives cannot be listed yet; nothing inside was checked."),
        );
        return Ok((r, Listing::default()));
    }
    r.sha256 = Some(file_digest(path)?);
    let listing = list_with(path, format, stop, want)?;
    let n = listing.entries.len();
    let entries = if n == 1 { "entry" } else { "entries" };
    r.coverage
        .checked(format!("{n} {entries} ({})", format.label()));
    r.coverage.push(notes().into());
    if let Some(why) = &listing.truncated {
        r.coverage.skip(
            "Later entries",
            if stop.load(Ordering::Relaxed) {
                SkipReason::Cancelled
            } else {
                SkipReason::LimitReached
            },
            why.clone(),
        );
    }
    assess(&mut r, &listing);
    Ok((r, listing))
}

pub fn notes() -> &'static str {
    "Lists the archive without extracting it: entry names, link targets, permissions and declared sizes. Launchers inside are read as text. Other file contents are not checked; scan the archive with ClamAV for known malware."
}

pub(crate) fn file_digest(path: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    std::io::copy(&mut File::open(path)?, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

/// Which entries' text a listing keeps, and up to how many bytes each.
pub struct Want {
    pub entry: fn(&str) -> bool,
    /// Entries kept by their exact name as well, for names only known after a
    /// first listing (a crate's custom build script).
    pub names: Vec<String>,
    pub max: u64,
}

impl Want {
    fn keeps(&self, name: &str) -> bool {
        (self.entry)(name) || self.names.iter().any(|n| n == name)
    }
}

pub const LAUNCHERS: Want = Want {
    entry: is_launcher,
    names: Vec::new(),
    max: desktop::MAX_LAUNCHER,
};

/// Kept text across one listing; beyond it, later entries are listed without text.
const MAX_KEPT: u64 = 128 * 1024 * 1024;

pub fn list(path: &Path, format: Format, stop: &Cancel) -> Result<Listing> {
    list_with(path, format, stop, &LAUNCHERS)
}

/// A listing that also keeps the text of the entries `want` names.
pub fn list_with(path: &Path, format: Format, stop: &Cancel, want: &Want) -> Result<Listing> {
    let compressed = std::fs::metadata(path)?.len();
    let mut listing = match format {
        Format::Zip => zip(&mut File::open(path)?, want)?,
        Format::Tar => tar(File::open(path)?, stop, want)?,
        Format::TarGz => tar(
            flate2::read::MultiGzDecoder::new(File::open(path)?),
            stop,
            want,
        )?,
        Format::TarXz => decompressed("xz", path, stop, want)?,
        Format::TarZst => decompressed("zstd", path, stop, want)?,
        Format::TarBz2 => decompressed("bzip2", path, stop, want)?,
        Format::Unsupported(name) => bail!("{name} archives cannot be listed"),
    };
    listing.compressed = compressed;
    Ok(listing)
}

/// A compressed tar stream that is `size` bytes of `path` from `offset`, such as
/// a .deb member, listed in place without copying it out.
pub fn list_part(
    path: &Path,
    offset: u64,
    size: u64,
    stop: &Cancel,
    want: &Want,
) -> Result<Listing> {
    let part = || -> Result<std::io::Take<File>> {
        let mut file = File::open(path)?;
        file.seek(SeekFrom::Start(offset))?;
        Ok(file.take(size))
    };
    let format = format_of_head(&head(&mut part()?)).context("Unknown compression")?;
    let mut listing = match format {
        Format::Tar => tar(part()?, stop, want)?,
        Format::TarGz => tar(flate2::read::MultiGzDecoder::new(part()?), stop, want)?,
        Format::TarXz | Format::TarZst | Format::TarBz2 => {
            let tool = tool(format);
            let cmd = runner::stream_parser_command(tool, &["-dc"])?;
            piped(tool, cmd, Some(Box::new(part()?)), stop, want)?
        }
        Format::Zip | Format::Unsupported(_) => bail!("{} is not a tar stream", format.label()),
    };
    listing.compressed = size;
    Ok(listing)
}

fn tool(format: Format) -> &'static str {
    match format {
        Format::TarXz => "xz",
        Format::TarZst => "zstd",
        _ => "bzip2",
    }
}

/// xz, zstd and bzip2 streams go through the system tool, isolated like the
/// PDF parsers, and are read as they come out.
fn decompressed(tool: &str, path: &Path, stop: &Cancel, want: &Want) -> Result<Listing> {
    let cmd = runner::parser_command(tool, &["-dc"], path)?;
    piped(tool, cmd, None, stop, want)
}

/// Runs a decompressor, feeding it `input` on stdin when given, and lists the
/// tar stream it writes.
fn piped(
    tool: &str,
    mut cmd: std::process::Command,
    input: Option<Box<dyn Read + Send>>,
    stop: &Cancel,
    want: &Want,
) -> Result<Listing> {
    let mut child = cmd
        .stdin(if input.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| format!("Cannot start {tool}"))?;
    let feeder = input.zip(child.stdin.take()).map(|(mut from, mut to)| {
        std::thread::spawn(move || {
            let _ = std::io::copy(&mut from, &mut to);
        })
    });
    let listing = tar(child.stdout.take().unwrap(), stop, want);
    let _ = child.kill();
    let _ = child.wait();
    if let Some(feeder) = feeder {
        let _ = feeder.join();
    }
    listing
}

/// Counts what the stream has produced, so the listing can stop at a limit.
struct Counted<R> {
    inner: R,
    read: u64,
}

impl<R: Read> Read for Counted<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.read >= MAX_STREAM {
            return Err(std::io::Error::other("Uncompressed data limit reached"));
        }
        let n = self.inner.read(buf)?;
        self.read += n as u64;
        Ok(n)
    }
}

fn tar(reader: impl Read, stop: &Cancel, want: &Want) -> Result<Listing> {
    let mut archive = tar::Archive::new(Counted {
        inner: reader,
        read: 0,
    });
    let mut listing = Listing::default();
    let mut kept = 0;
    let entries = archive.entries().context("Not a tar archive")?;
    for (i, entry) in entries.enumerate() {
        if stop.load(Ordering::Relaxed) {
            listing.truncated = Some("Cancelled".into());
            break;
        }
        if i >= MAX_ENTRIES {
            listing.truncated = Some(format!("Stopped after {MAX_ENTRIES} entries."));
            break;
        }
        let mut e = match entry {
            Ok(e) => e,
            Err(_) if i == 0 => bail!("The compressed file holds a single file, not a tar archive"),
            Err(e) => {
                listing.truncated = Some(format!("The archive is damaged after entry {i}: {e}"));
                break;
            }
        };
        let header = e.header();
        let tar_type = header.entry_type();
        let kind = if tar_type.is_dir() {
            Kind::Dir
        } else if tar_type.is_symlink() {
            Kind::Symlink
        } else if tar_type.is_hard_link() {
            Kind::Hardlink
        } else if tar_type.is_character_special() || tar_type.is_block_special() {
            Kind::Device
        } else if tar_type.is_fifo() {
            Kind::Fifo
        } else if tar_type.is_file() || tar_type.is_gnu_sparse() || tar_type.is_contiguous() {
            Kind::File
        } else {
            Kind::Other
        };
        let name = String::from_utf8_lossy(&e.path_bytes()).into_owned();
        let target = e
            .link_name_bytes()
            .map(|t| String::from_utf8_lossy(&t).into_owned())
            .unwrap_or_default();
        let mode = header.mode().unwrap_or(0);
        let size = e.size();
        let keep = kind == Kind::File && want.keeps(&name) && size <= want.max;
        let keep = keep && kept + size <= MAX_KEPT;
        kept += if keep { size } else { 0 };
        let content = keep
            .then(|| {
                let mut b = vec![];
                e.by_ref()
                    .take(want.max)
                    .read_to_end(&mut b)
                    .ok()
                    .map(|_| String::from_utf8_lossy(&b).into_owned())
            })
            .flatten();
        listing.entries.push(Entry {
            name,
            kind,
            target,
            mode,
            size,
            content,
        });
    }
    if archive.into_inner().read >= MAX_STREAM {
        listing.truncated = Some(format!(
            "Stopped after {} GiB of uncompressed data.",
            MAX_STREAM >> 30
        ));
    }
    Ok(listing)
}

fn le16(b: &[u8], at: usize) -> Option<u64> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?) as u64)
}
fn le32(b: &[u8], at: usize) -> Option<u64> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?) as u64)
}
fn le64(b: &[u8], at: usize) -> Option<u64> {
    Some(u64::from_le_bytes(b.get(at..at + 8)?.try_into().ok()?))
}

/// The zip central directory, which is what extractors go by.
fn zip(file: &mut File, want: &Want) -> Result<Listing> {
    let len = file.metadata()?.len();
    // The end record sits in the last 22 bytes plus a comment of up to 64 KiB.
    let tail_len = len.min(22 + 65535);
    file.seek(SeekFrom::Start(len - tail_len))?;
    let mut tail = vec![0; tail_len as usize];
    file.read_exact(&mut tail)?;
    let end = (0..tail.len().saturating_sub(21))
        .rev()
        .find(|&i| tail[i..i + 4] == [b'P', b'K', 5, 6])
        .context("No zip central directory; the file is damaged or not a zip")?;
    let mut count = le16(&tail, end + 10).unwrap_or(0);
    let mut size = le32(&tail, end + 12).unwrap_or(0);
    let mut offset = le32(&tail, end + 16).unwrap_or(0);
    if (count == 0xffff || size == 0xffff_ffff || offset == 0xffff_ffff) && end >= 20 {
        // Zip64: a locator just before the end record points at the larger record.
        let locator = &tail[end - 20..end];
        if locator[..4] == [b'P', b'K', 6, 7] {
            let at = le64(locator, 8).context("zip64 locator")?;
            let mut record = [0u8; 56];
            file.seek(SeekFrom::Start(at))?;
            file.read_exact(&mut record)?;
            if record[..4] == [b'P', b'K', 6, 6] {
                count = le64(&record, 32).unwrap_or(count);
                size = le64(&record, 40).unwrap_or(size);
                offset = le64(&record, 48).unwrap_or(offset);
            }
        }
    }
    if size > 256 * 1024 * 1024 || offset.checked_add(size).is_none_or(|e| e > len) {
        bail!("The zip central directory is larger than the file; it is damaged");
    }
    let mut dir = vec![0; size as usize];
    file.seek(SeekFrom::Start(offset))?;
    file.read_exact(&mut dir)?;
    let mut listing = Listing::default();
    let mut kept = 0;
    let mut at = 0;
    while at + 46 <= dir.len() && dir[at..at + 4] == [b'P', b'K', 1, 2] {
        if listing.entries.len() >= MAX_ENTRIES {
            listing.truncated = Some(format!("Stopped after {MAX_ENTRIES} entries."));
            break;
        }
        let host = dir[at + 5];
        let flags = le16(&dir, at + 8).unwrap_or(0);
        let method = le16(&dir, at + 10).unwrap_or(0);
        let mut packed = le32(&dir, at + 20).unwrap_or(0);
        let mut size = le32(&dir, at + 24).unwrap_or(0);
        let name_len = le16(&dir, at + 28).unwrap_or(0) as usize;
        let extra_len = le16(&dir, at + 30).unwrap_or(0) as usize;
        let comment_len = le16(&dir, at + 32).unwrap_or(0) as usize;
        let external = le32(&dir, at + 38).unwrap_or(0);
        let mut local = le32(&dir, at + 42).unwrap_or(0);
        let name_end = at + 46 + name_len;
        let Some(raw) = dir.get(at + 46..name_end) else {
            break;
        };
        let name = String::from_utf8_lossy(raw).into_owned();
        // Zip64 sizes and offset follow, in order, for each field saturated above.
        if let Some(extra) = dir.get(name_end..name_end + extra_len) {
            let mut i = 0;
            while i + 4 <= extra.len() {
                let id = le16(extra, i).unwrap_or(0);
                let n = le16(extra, i + 2).unwrap_or(0) as usize;
                if id == 1 {
                    let mut j = i + 4;
                    for field in [&mut size, &mut packed, &mut local] {
                        if *field == 0xffff_ffff {
                            *field = le64(extra, j).unwrap_or(*field);
                            j += 8;
                        }
                    }
                }
                i += 4 + n;
            }
        }
        let mode = if host == 3 {
            (external >> 16) as u32
        } else {
            0
        };
        let kind = match mode & 0o170000 {
            0o120000 => Kind::Symlink,
            0o040000 => Kind::Dir,
            0o020000 | 0o060000 => Kind::Device,
            0o010000 => Kind::Fifo,
            0o140000 => Kind::Other,
            _ if name.ends_with('/') => Kind::Dir,
            _ => Kind::File,
        };
        // A symlink's target and a launcher's text are the entry's data.
        let max = if kind == Kind::Symlink {
            4096
        } else {
            want.max
        };
        let wanted = kind == Kind::Symlink || (kind == Kind::File && want.keeps(&name));
        let keep = wanted && flags & 1 == 0 && size <= max && kept + size <= MAX_KEPT;
        kept += if keep { size } else { 0 };
        let data = keep
            .then(|| zip_data(file, local, packed, method, max).ok())
            .flatten();
        let (target, content) = match kind {
            Kind::Symlink => (data.unwrap_or_default(), None),
            _ => (String::new(), data),
        };
        listing.entries.push(Entry {
            name,
            kind,
            target,
            mode,
            size,
            content,
        });
        at = name_end + extra_len + comment_len;
    }
    if (listing.entries.len() as u64) < count && listing.truncated.is_none() {
        listing.truncated = Some(format!(
            "The directory lists {count} entries but only {} could be read.",
            listing.entries.len()
        ));
    }
    Ok(listing)
}

/// One small entry's bytes, stored or deflated.
fn zip_data(file: &mut File, local: u64, packed: u64, method: u64, max: u64) -> Result<String> {
    let mut header = [0u8; 30];
    file.seek(SeekFrom::Start(local))?;
    file.read_exact(&mut header)?;
    if header[..4] != [b'P', b'K', 3, 4] {
        bail!("no local header");
    }
    let skip = le16(&header, 26).unwrap_or(0) + le16(&header, 28).unwrap_or(0);
    file.seek(SeekFrom::Current(skip as i64))?;
    let raw = file.by_ref().take(packed);
    let mut out = vec![];
    match method {
        0 => raw.take(max).read_to_end(&mut out)?,
        8 => flate2::read::DeflateDecoder::new(raw)
            .take(max)
            .read_to_end(&mut out)?,
        other => bail!("compression method {other}"),
    };
    Ok(String::from_utf8_lossy(&out).into_owned())
}

pub fn is_launcher(name: &str) -> bool {
    name.to_ascii_lowercase().ends_with(".desktop")
}

/// Splits on both separators: some extractors treat `\` as one.
fn parts(name: &str) -> impl Iterator<Item = &str> {
    name.split(['/', '\\'])
        .filter(|p| !p.is_empty() && *p != ".")
}

fn absolute(name: &str) -> bool {
    let b = name.as_bytes();
    name.starts_with(['/', '\\'])
        || (b.len() >= 3
            && b[0].is_ascii_alphabetic()
            && b[1] == b':'
            && (b[2] == b'/' || b[2] == b'\\'))
}

/// True when walking `parts` from the archive root ever climbs above it.
fn climbs<'a>(start: Vec<&'a str>, parts: impl Iterator<Item = &'a str>) -> bool {
    let mut depth = start.len() as i64;
    for p in parts {
        depth += if p == ".." { -1 } else { 1 };
        if depth < 0 {
            return true;
        }
    }
    false
}

/// Home-relative places where an extracted file runs at login or grants access.
const STARTUP: &[&str] = &[
    ".config/autostart/",
    ".config/autostart-scripts/",
    ".config/systemd/user/",
    ".config/environment.d/",
    ".config/plasma-workspace/env/",
    ".config/fish/config.fish",
    ".local/share/applications/",
    ".ssh/authorized_keys",
    ".ssh/rc",
    ".ssh/config",
    ".bashrc",
    ".bash_profile",
    ".bash_login",
    ".bash_logout",
    ".profile",
    ".zshrc",
    ".zshenv",
    ".zprofile",
    ".zlogin",
    ".xprofile",
    ".xinitrc",
    ".xsessionrc",
    ".pam_environment",
    "etc/cron.d/",
    "etc/cron.daily/",
    "etc/cron.hourly/",
    "etc/crontab",
    "etc/profile.d/",
    "etc/ld.so.preload",
    "etc/systemd/system/",
    "etc/xdg/autostart/",
    "var/spool/cron/",
];

fn startup_path(name: &str) -> Option<&'static str> {
    let path = parts(name)
        .filter(|p| *p != "..")
        .collect::<Vec<_>>()
        .join("/");
    let path = format!("/{path}");
    STARTUP.iter().copied().find(|s| {
        let s = s.trim_end_matches('/');
        let at = path.find(&format!("/{s}"));
        at.is_some_and(|at| {
            let rest = &path[at + 1 + s.len()..];
            rest.is_empty() || rest.starts_with('/')
        })
    })
}

/// Groups entries under one finding per rule, so a thousand bad names stay readable.
#[derive(Default)]
pub(crate) struct Group(pub(crate) Vec<String>);

impl Group {
    pub(crate) fn push(&mut self, line: String) {
        self.0.push(line);
    }
    pub(crate) fn evidence(&self) -> String {
        let mut lines: Vec<String> = self.0.iter().take(EXAMPLES).cloned().collect();
        if self.0.len() > EXAMPLES {
            lines.push(format!("and {} more", self.0.len() - EXAMPLES));
        }
        lines.join("\n")
    }
}

pub fn assess(r: &mut Report, listing: &Listing) {
    let mut traversal = Group::default();
    let mut absolute_names = Group::default();
    let mut escapes = Group::default();
    let mut special = Group::default();
    let mut startup = Group::default();
    // Links already seen that leave the archive, by their normalised name.
    let mut escaping_links = std::collections::HashSet::<String>::new();
    let mut expanded: u64 = 0;
    for e in &listing.entries {
        if e.kind == Kind::File {
            expanded = expanded.saturating_add(e.size);
        }
        if absolute(&e.name) {
            absolute_names.push(e.name.clone());
        } else if climbs(vec![], parts(&e.name)) {
            traversal.push(e.name.clone());
        }
        if matches!(e.kind, Kind::Symlink | Kind::Hardlink) {
            // A symlink resolves from its own folder; a hardlink names another entry.
            let mut from: Vec<&str> = parts(&e.name).collect();
            if e.kind == Kind::Symlink {
                from.pop();
            } else {
                from.clear();
            }
            if absolute(&e.target) || climbs(from, parts(&e.target)) {
                let what = if e.kind == Kind::Symlink {
                    "links to"
                } else {
                    "is a hard link to"
                };
                escapes.push(format!("{} {what} {}", e.name, e.target));
                escaping_links.insert(parts(&e.name).collect::<Vec<_>>().join("/"));
            }
        }
        // Each folder above the entry, shortest first, looked up rather than
        // compared against every link, so a large archive stays linear.
        if !escaping_links.is_empty() {
            let names: Vec<&str> = parts(&e.name).collect();
            if let Some(through) = (1..names.len())
                .map(|n| names[..n].join("/"))
                .find(|folder| escaping_links.contains(folder))
            {
                escapes.push(format!("{} is written through {through}", e.name));
            }
        }
        match e.kind {
            Kind::Device => special.push(format!("{} is a device file", e.name)),
            Kind::Fifo => special.push(format!("{} is a named pipe", e.name)),
            _ if e.mode & 0o4000 != 0 => special.push(format!("{} is setuid", e.name)),
            _ if e.mode & 0o2000 != 0 => special.push(format!("{} is setgid", e.name)),
            _ => {}
        }
        if e.kind != Kind::Dir {
            if let Some(place) = startup_path(&e.name) {
                startup.push(format!(
                    "{} (under {})",
                    e.name,
                    place.trim_end_matches('/')
                ));
            }
        }
        if e.kind == Kind::File && is_launcher(&e.name) {
            let location = format!("In the archive: {}", e.name);
            let file_name = parts(&e.name).last().unwrap_or(&e.name).to_string();
            // Unread (too large, encrypted, unusual compression) still counts as a launcher.
            let text = e.content.as_deref().unwrap_or(desktop::UNREAD);
            desktop::assess(r, &location, &file_name, text, e.mode & 0o111 != 0);
        }
    }
    let groups = [
        (absolute_names, "archive.absolute-path", Severity::High,
            "Entries are named with an absolute path",
            "An extractor that keeps these names writes the files there, not into the folder you chose. Current tar and unzip strip the leading slash; some tools have not. Do not extract it with an unfamiliar tool, and list it first: bsdtar -tvf FILE."),
        (traversal, "archive.path-traversal", Severity::High,
            "Entries climb out of the folder they are extracted into",
            "Names with .. can write outside the destination, for example over ~/.bashrc. Ordinary archives never need them. Do not extract it."),
        (escapes, "archive.link-escape", Severity::High,
            "Links inside point outside the archive",
            "A link that leaves the extraction folder lets later entries, or the next program that follows it, write anywhere you can. Do not extract it."),
        (special, "archive.special-entry", Severity::Review,
            "Entries are devices, pipes or setuid programs",
            "A user archive rarely needs these. Extracting as an ordinary user drops setuid bits and refuses devices; extracting as root does not."),
        (startup, "archive.startup-path", Severity::Review,
            "Entries would land in startup or login files",
            "If extracted into your home folder or /, these files run at login or change who can sign in. Check each one before extracting there."),
    ];
    for (group, rule, severity, summary, action) in groups {
        if !group.0.is_empty() {
            r.add(rule, severity, summary, group.evidence(), action);
        }
    }
    let ratio = expanded / listing.compressed.max(1);
    if ratio >= BOMB_RATIO && expanded >= BOMB_SIZE {
        r.add(
            "archive.bomb-ratio",
            Severity::Review,
            "The archive expands to far more than its size",
            format!(
                "{} becomes {} when extracted, {ratio} times larger.",
                size(listing.compressed),
                size(expanded)
            ),
            "Extracting it can fill your disk. Ordinary archives shrink by 2 to 10 times, not hundreds. Extract it only somewhere with a size limit, or not at all.",
        );
    }
    r.findings.sort_by_key(|f| std::cmp::Reverse(f.severity));
}

fn size(bytes: u64) -> String {
    match bytes {
        b if b >= 1 << 30 => format!("{:.1} GiB", b as f64 / (1u64 << 30) as f64),
        b if b >= 1 << 20 => format!("{:.1} MiB", b as f64 / (1u64 << 20) as f64),
        b if b >= 1 << 10 => format!("{:.1} KiB", b as f64 / 1024.0),
        b => format!("{b} bytes"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn rules(r: &Report) -> Vec<&str> {
        r.findings.iter().map(|f| f.rule.as_str()).collect()
    }

    /// A stored (uncompressed) zip, written by hand: local headers, then the
    /// central directory with Unix modes.
    fn zip_file(path: &Path, entries: &[(&str, u32, &[u8])]) {
        let mut out = vec![];
        let mut central = vec![];
        for (name, mode, data) in entries {
            let offset = out.len() as u32;
            let crc = 0u32;
            out.extend_from_slice(b"PK\x03\x04");
            out.extend_from_slice(&[20, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            out.extend_from_slice(&crc.to_le_bytes());
            out.extend_from_slice(&(data.len() as u32).to_le_bytes());
            out.extend_from_slice(&(data.len() as u32).to_le_bytes());
            out.extend_from_slice(&(name.len() as u16).to_le_bytes());
            out.extend_from_slice(&0u16.to_le_bytes());
            out.extend_from_slice(name.as_bytes());
            out.extend_from_slice(data);
            central.extend_from_slice(b"PK\x01\x02");
            central.extend_from_slice(&[20, 3, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            central.extend_from_slice(&crc.to_le_bytes());
            central.extend_from_slice(&(data.len() as u32).to_le_bytes());
            central.extend_from_slice(&(data.len() as u32).to_le_bytes());
            central.extend_from_slice(&(name.len() as u16).to_le_bytes());
            central.extend_from_slice(&[0; 8]);
            central.extend_from_slice(&(mode << 16).to_le_bytes());
            central.extend_from_slice(&offset.to_le_bytes());
            central.extend_from_slice(name.as_bytes());
        }
        let at = out.len() as u32;
        out.extend_from_slice(&central);
        out.extend_from_slice(b"PK\x05\x06\0\0\0\0");
        out.extend_from_slice(&(entries.len() as u16).to_le_bytes());
        out.extend_from_slice(&(entries.len() as u16).to_le_bytes());
        out.extend_from_slice(&(central.len() as u32).to_le_bytes());
        out.extend_from_slice(&at.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        std::fs::write(path, out).unwrap();
    }

    #[test]
    fn a_zip_is_listed_without_extracting() {
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join("notice.zip");
        let launcher =
            b"[Desktop Entry]\nType=Application\nIcon=application-pdf\nExec=sh -c \"echo test\"\n";
        zip_file(
            &path,
            &[
                ("../escape.txt", 0o100644, b"x"),
                ("link", 0o120777, b"/tmp"),
                ("link/x", 0o100644, b"x"),
                ("Meeting.pdf.desktop", 0o100755, launcher),
            ],
        );
        assert_eq!(format_of(&path), Some(Format::Zip));
        let (r, _) = check(&path, &runner::cancel()).unwrap();
        assert_eq!(
            rules(&r),
            [
                "desktop.masquerade",
                "archive.path-traversal",
                "archive.link-escape"
            ]
        );
        let escape = &r.findings[2].evidence;
        assert!(escape.contains("link links to /tmp"));
        assert!(escape.contains("link/x is written through link"));
        assert!(r.findings[0].evidence.contains("marked executable"));
        // Nothing was extracted next to the archive.
        assert_eq!(std::fs::read_dir(d.path()).unwrap().count(), 1);
    }

    #[test]
    fn a_gzipped_tar_is_listed_and_a_bomb_is_measured() {
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join("data.tar.gz");
        let gz = flate2::write::GzEncoder::new(
            File::create(&path).unwrap(),
            flate2::Compression::best(),
        );
        let mut tar = tar::Builder::new(gz);
        let mut header = tar::Header::new_gnu();
        header.set_size(BOMB_SIZE);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(&mut header, "zeros.bin", std::io::repeat(0).take(BOMB_SIZE))
            .unwrap();
        let mut header = tar::Header::new_gnu();
        header.set_size(1);
        header.set_mode(0o4755);
        header.set_cksum();
        tar.append_data(&mut header, "home/u/.config/autostart/x.desktop", &b"x"[..])
            .unwrap();
        let mut big = tar::Header::new_gnu();
        big.set_size(desktop::MAX_LAUNCHER + 1);
        big.set_mode(0o644);
        big.set_cksum();
        tar.append_data(
            &mut big,
            "big.desktop",
            std::io::repeat(b'#').take(desktop::MAX_LAUNCHER + 1),
        )
        .unwrap();
        tar.into_inner().unwrap().finish().unwrap().flush().unwrap();
        assert_eq!(format_of(&path), Some(Format::TarGz));
        let (r, _) = check(&path, &runner::cancel()).unwrap();
        assert_eq!(
            rules(&r),
            [
                "desktop.launcher",
                "archive.special-entry",
                "archive.startup-path",
                "archive.bomb-ratio"
            ]
        );
        // A launcher too large to read is still named; one that is not a launcher is not.
        assert!(r.findings[0]
            .evidence
            .contains("big.desktop\nName: \nRuns: (not read)"));
    }

    #[test]
    fn ordinary_archives_raise_nothing() {
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join("project.zip");
        zip_file(
            &path,
            &[
                ("project/", 0o040755, b""),
                ("project/README.md", 0o100644, b"hello"),
                ("project/current", 0o120777, b"v1/bin"),
                ("project/v1/../v1/bin/run.sh", 0o100755, b"echo"),
            ],
        );
        let (r, _) = check(&path, &runner::cancel()).unwrap();
        assert!(r.findings.is_empty(), "{:?}", rules(&r));
        assert_eq!(r.coverage.checked, ["4 entries (zip)"]);
    }

    #[test]
    fn names_are_judged_by_their_parts() {
        assert!(climbs(vec![], parts("a/../../b")));
        assert!(!climbs(vec![], parts("a/../b")));
        assert!(climbs(vec!["a"], parts("../../etc")));
        assert!(absolute("C:\\Users\\x") && absolute("/etc/x") && !absolute("etc/x"));
        assert_eq!(startup_path("dotfiles/.bashrc"), Some(".bashrc"));
        assert_eq!(startup_path("docs/bashrc-notes.txt"), None);
        assert_eq!(
            startup_path("x/.ssh/authorized_keys"),
            Some(".ssh/authorized_keys")
        );
        assert_eq!(startup_path("my.profile.txt"), None);
    }
}
