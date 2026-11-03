//! Picks the inspector for a target from the target itself, so the user does not
//! have to. Deterministic: URL scheme, file name and the first bytes, nothing else.
use crate::{
    checks::{archive, desktop, inspect, langpkg, lockfile, packages, pdf, pkgfile, repo},
    model::Report,
    runner::Cancel,
    store::Store,
};
use anyhow::Result;
use std::{fs, io::Read, path::Path};

#[derive(Debug, PartialEq, Eq)]
pub enum Inspector {
    Archive,
    Installer,
    Launcher,
    Lockfile,
    /// .deb, .rpm, Arch packages, PKGBUILDs and AppImages.
    PackageFile,
    Packages,
    Pdf,
}

pub fn choose(target: &str) -> Inspector {
    if target.starts_with("https://") {
        return Inspector::Installer;
    }
    let path = Path::new(target);
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    if lockfile::format_of(path).is_some() {
        return Inspector::Lockfile;
    }
    let file = fs::symlink_metadata(path).is_ok_and(|m| m.is_file());
    match fs::symlink_metadata(path) {
        Ok(meta) if meta.is_dir() => Inspector::Packages,
        _ if name == "package.json" => Inspector::Packages,
        _ if file && pkgfile::kind_of(path).is_some() => Inspector::PackageFile,
        _ if path.file_name().is_some_and(|n| n == "PKGBUILD")
            || [".deb", ".rpm", ".appimage", ".pkg.tar.zst", ".pkg.tar.xz"]
                .iter()
                .any(|s| name.ends_with(s)) =>
        {
            Inspector::PackageFile
        }
        _ if name.ends_with(".desktop") || (file && launcher_text(path)) => Inspector::Launcher,
        _ if file && archive::format_of(path).is_some() => Inspector::Archive,
        _ if [
            ".zip", ".tar", ".tgz", ".tar.gz", ".tar.xz", ".tar.zst", ".tar.bz2",
        ]
        .iter()
        .any(|s| name.ends_with(s)) =>
        {
            Inspector::Archive
        }
        Ok(meta) if meta.is_file() && starts_with(path, b"%PDF-") => Inspector::Pdf,
        // Everything else, including unreadable paths, goes to the installer rules,
        // which report the read error themselves.
        _ => Inspector::Installer,
    }
}

fn launcher_text(path: &Path) -> bool {
    let mut head = vec![];
    fs::File::open(path)
        .and_then(|f| f.take(4096).read_to_end(&mut head))
        .is_ok()
        && desktop::looks_like_launcher(&head)
}

fn starts_with(path: &Path, magic: &[u8]) -> bool {
    let mut head = vec![0; magic.len()];
    fs::File::open(path)
        .and_then(|mut f| f.read_exact(&mut head))
        .is_ok()
        && head == magic
}

pub fn inspect_any(store: &Store, target: &str, stop: &Cancel) -> Result<Report> {
    match choose(target) {
        Inspector::Installer => inspect::inspect(store, target, stop),
        Inspector::Lockfile => {
            let r = lockfile::lockfile(Path::new(target), None)?;
            store.save(&r)?;
            Ok(r)
        }
        Inspector::Packages if Path::new(target).is_dir() => project(store, Path::new(target)),
        Inspector::Packages => packages::packages(store, Path::new(target)),
        Inspector::Pdf => pdf::scan_pdf(store, Path::new(target), stop),
        Inspector::Launcher => desktop::launcher(store, Path::new(target)),
        Inspector::PackageFile => pkgfile::inspect(store, Path::new(target), stop),
        Inspector::Archive => archive_or_package(store, Path::new(target), stop),
    }
}

/// Archive entries whose text an inspector reads: launchers and the files the
/// language-package rules look at.
const ARCHIVE_TEXT: archive::Want = archive::Want {
    entry: |name| archive::is_launcher(name) || langpkg::wanted(name),
    max: crate::store::MAX_INPUT,
};

/// An archive's entries. An npm package archive also gets its script
/// inventory; wheels, sdists, crates and VS Code extensions their own rules.
fn archive_or_package(store: &Store, path: &Path, stop: &Cancel) -> Result<Report> {
    let (mut listed, listing) = archive::check_with(path, stop, &ARCHIVE_TEXT)?;
    let npm = path.extension().is_some_and(|e| e == "tgz" || e == "gz")
        && listing
            .entries
            .iter()
            .any(|e| e.name == "package/package.json");
    let r = if npm {
        let mut r = packages::inventory(path)?;
        r.findings.extend(listed.findings);
        r.findings.sort_by_key(|f| std::cmp::Reverse(f.severity));
        r.coverage.extend(listed.coverage);
        r
    } else {
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let names: Vec<&str> = listing.entries.iter().map(|e| e.name.as_str()).collect();
        if let Some(eco) = langpkg::detect(&file_name, &names) {
            listed.kind = "Package archive".into();
            listed.coverage.checked(eco.label());
            listed.coverage.push(langpkg::notes(eco).into());
            langpkg::assess(&mut listed, eco, &listing);
        }
        listed
    };
    store.save(&r)?;
    Ok(r)
}

/// A project folder: its package scripts and what the tree could run on open.
pub fn project(store: &Store, dir: &Path) -> Result<Report> {
    let mut r = packages::inventory(dir)?;
    let checked = repo::repo_check(dir)?;
    r.kind = "Project inspection".into();
    r.findings.extend(checked.findings);
    r.findings.sort_by_key(|f| std::cmp::Reverse(f.severity));
    r.coverage.checked.extend(checked.coverage.checked);
    r.coverage.notes.extend(checked.coverage.notes);
    r.coverage.skipped.extend(checked.coverage.skipped);
    if let Some((eco, listing)) = langpkg::folder(dir) {
        let files: Vec<&str> = listing
            .entries
            .iter()
            .map(|e| e.name.trim_start_matches("./"))
            .collect();
        r.coverage
            .checked(format!("{}: {}", eco.label(), files.join(", ")));
        r.coverage.push(langpkg::notes(eco).into());
        langpkg::assess(&mut r, eco, &listing);
    }
    // Lockfiles at the project root, linted in place.
    for name in [
        "package-lock.json",
        "npm-shrinkwrap.json",
        "yarn.lock",
        "pnpm-lock.yaml",
        "bun.lock",
        "Cargo.lock",
    ] {
        let path = dir.join(name);
        if !path.is_file() {
            continue;
        }
        match lockfile::lockfile(&path, None) {
            Ok(l) => {
                r.findings.extend(l.findings);
                r.coverage
                    .checked
                    .push(format!("{name}: {}", l.coverage.checked.join(", ")));
            }
            Err(e) => r.coverage.skipped.push(crate::model::Skipped {
                what: name.into(),
                why: crate::model::SkipReason::Unreadable,
                detail: format!("{e:#}"),
            }),
        }
    }
    r.findings.sort_by_key(|f| std::cmp::Reverse(f.severity));
    store.save(&r)?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn routes_by_scheme_type_and_magic_bytes() {
        let d = tempfile::tempdir().unwrap();
        let pdf = d.path().join("invoice");
        fs::write(&pdf, b"%PDF-1.7\n").unwrap();
        let script = d.path().join("setup.sh");
        fs::write(&script, b"echo hi\n").unwrap();
        assert_eq!(
            choose("https://example.org/install.sh"),
            Inspector::Installer
        );
        assert_eq!(choose(&d.path().to_string_lossy()), Inspector::Packages);
        assert_eq!(choose("/downloads/left-pad-1.3.0.tgz"), Inspector::Archive);
        assert_eq!(choose(&pdf.to_string_lossy()), Inspector::Pdf);
        assert_eq!(choose(&script.to_string_lossy()), Inspector::Installer);
        // Launchers and archives by content, whatever they are called.
        let launcher = d.path().join("invoice");
        fs::write(&launcher, b"[Desktop Entry]\nExec=sh\n").unwrap();
        assert_eq!(choose(&launcher.to_string_lossy()), Inspector::Launcher);
        let zip = d.path().join("photos");
        fs::write(&zip, b"PK\x05\x06").unwrap();
        assert_eq!(choose(&zip.to_string_lossy()), Inspector::Archive);
        assert_eq!(choose("/downloads/notes.pdf.desktop"), Inspector::Launcher);
        assert_eq!(
            choose("/downloads/tool_1.0_amd64.deb"),
            Inspector::PackageFile
        );
        assert_eq!(choose("/aur/fixture/PKGBUILD"), Inspector::PackageFile);
    }

    #[test]
    fn crates_get_language_rules_as_archives_and_as_folders() {
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join("fixture-1.0.0.crate");
        let mut tar = tar::Builder::new(flate2::write::GzEncoder::new(
            fs::File::create(&path).unwrap(),
            flate2::Compression::default(),
        ));
        let build = b"fn main() { let _ = std::net::TcpStream::connect(\"192.0.2.1:80\"); }\n";
        for (name, body) in [
            (
                "fixture-1.0.0/Cargo.toml",
                &b"[package]\nname = \"fixture\"\n"[..],
            ),
            ("fixture-1.0.0/build.rs", &build[..]),
        ] {
            let mut h = tar::Header::new_gnu();
            h.set_size(body.len() as u64);
            h.set_mode(0o644);
            h.set_path(name).unwrap();
            h.set_cksum();
            tar.append(&h, body).unwrap();
        }
        tar.into_inner().unwrap().finish().unwrap();
        let state = tempfile::tempdir().unwrap();
        let store = Store::open(Some(state.path().join("s"))).unwrap();
        let r = inspect_any(&store, &path.to_string_lossy(), &crate::runner::cancel()).unwrap();
        assert_eq!(r.kind, "Package archive");
        assert_eq!(
            r.findings
                .iter()
                .map(|f| (f.rule.as_str(), f.severity))
                .collect::<Vec<_>>(),
            [("rust.build-script", crate::model::Severity::High)]
        );
        let folder = d.path().join("fixture");
        fs::create_dir(&folder).unwrap();
        fs::write(folder.join("Cargo.toml"), "[package]\nname = \"fixture\"\n").unwrap();
        fs::write(folder.join("build.rs"), build).unwrap();
        let r = project(&store, &folder).unwrap();
        assert!(r.findings.iter().any(|f| f.rule == "rust.build-script"));
        assert!(r
            .coverage
            .checked
            .contains(&"Rust crate: Cargo.toml, build.rs".to_string()));
    }

    #[test]
    fn an_npm_archive_gets_scripts_and_entry_rules() {
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join("left-pad-1.3.0.tgz");
        let gz = flate2::write::GzEncoder::new(
            fs::File::create(&path).unwrap(),
            flate2::Compression::default(),
        );
        let mut tar = tar::Builder::new(gz);
        for (name, body) in [
            (
                "package/package.json",
                &br#"{"name":"left-pad","version":"1.3.0","scripts":{"postinstall":"node x.js"}}"#
                    [..],
            ),
            ("package/../../.bashrc", &b"x"[..]),
        ] {
            let mut h = tar::Header::new_gnu();
            h.set_size(body.len() as u64);
            h.set_mode(0o644);
            // set_path refuses .., as an extractor should; write the raw name.
            h.as_old_mut().name[..name.len()].copy_from_slice(name.as_bytes());
            h.set_cksum();
            tar.append(&h, body).unwrap();
        }
        tar.into_inner().unwrap().finish().unwrap();
        let state = tempfile::tempdir().unwrap();
        let store = Store::open(Some(state.path().join("s"))).unwrap();
        let r = inspect_any(&store, &path.to_string_lossy(), &crate::runner::cancel()).unwrap();
        let rules: Vec<_> = r.findings.iter().map(|f| f.rule.as_str()).collect();
        assert_eq!(r.kind, "Package script inventory");
        assert_eq!(
            rules,
            [
                "archive.path-traversal",
                "package.lifecycle",
                "archive.startup-path"
            ]
        );
    }
}
