//! Picks the inspector for a target from the target itself, so the user does not
//! have to. Deterministic: URL scheme, file type and the first bytes, nothing else.
use crate::{
    checks::{inspect, packages, pdf, repo},
    model::Report,
    runner::Cancel,
    store::Store,
};
use anyhow::Result;
use std::{fs, io::Read, path::Path};

#[derive(Debug, PartialEq, Eq)]
pub enum Inspector {
    Installer,
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
    match fs::symlink_metadata(path) {
        Ok(meta) if meta.is_dir() => Inspector::Packages,
        _ if name.ends_with(".tgz") || name.ends_with(".tar.gz") || name == "package.json" => {
            Inspector::Packages
        }
        Ok(meta) if meta.is_file() && starts_with(path, b"%PDF-") => Inspector::Pdf,
        // Everything else, including unreadable paths, goes to the installer rules,
        // which report the read error themselves.
        _ => Inspector::Installer,
    }
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
        Inspector::Packages if Path::new(target).is_dir() => project(store, Path::new(target)),
        Inspector::Packages => packages::packages(store, Path::new(target)),
        Inspector::Pdf => pdf::scan_pdf(store, Path::new(target), stop),
    }
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
        assert_eq!(choose("/downloads/left-pad-1.3.0.tgz"), Inspector::Packages);
        assert_eq!(choose(&pdf.to_string_lossy()), Inspector::Pdf);
        assert_eq!(choose(&script.to_string_lossy()), Inspector::Installer);
    }
}
