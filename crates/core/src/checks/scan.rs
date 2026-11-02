use crate::{
    model::{now, Report, Severity},
    runner::{self, Cancel},
    store::{digest, private_dir, read_file, Store, MAX_INPUT},
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    os::unix::fs::{MetadataExt, OpenOptionsExt},
    path::{Path, PathBuf},
    process::Command,
};
use ts_rs::TS;
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ScanItem {
    pub path: PathBuf,
    pub hash: String,
    pub detected: bool,
    pub signature: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Quarantined {
    pub id: String,
    pub original: PathBuf,
    pub hash: String,
    pub created: u64,
    pub restored: bool,
}
pub fn own_database(store: &Store) -> Option<PathBuf> {
    let p = store.root.join("signatures");
    if p.join("daily.cvd").exists() || p.join("daily.cld").exists() {
        Some(p)
    } else {
        None
    }
}
fn scan_inner(store: &Store, path: &Path, stop: &Cancel) -> Result<Report> {
    let mut r = Report::new("File scan", path.display());
    r.coverage.push("ClamAV signature/heuristic scan of snapshots. At most 2,000 files, 64 MiB/file, 512 MiB total; archives limited to 256 MiB expansion, 256 members and depth 16. Symlinks, canaries and Bigman state are excluded. Findings do not prove the host is uncompromised.".into());
    let meta = fs::symlink_metadata(path)?;
    if meta.file_type().is_symlink() {
        bail!("Select a regular file or directory, not a symlink");
    }
    let path = fs::canonicalize(path)?;
    let temp = tempfile::tempdir_in(&store.root)?;
    let mut items = vec![];
    let mut scan_paths = vec![];
    let mut total = 0usize;
    let canaries = crate::checks::canary::list(store)?;
    let walker = walkdir::WalkDir::new(&path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !e.path().starts_with(&store.root));
    for (visited, entry) in walker.enumerate() {
        if visited >= 50000 || items.len() >= 2000 {
            r.errors.push("Scan traversal/file limit reached".into());
            break;
        }
        if stop.load(std::sync::atomic::Ordering::Relaxed) {
            r.errors.push("Scan cancelled".into());
            break;
        }
        let e = match entry {
            Ok(e) => e,
            Err(e) => {
                r.errors.push(e.to_string());
                continue;
            }
        };
        if e.file_type().is_dir() {
            continue;
        }
        if !e.file_type().is_file() {
            r.errors
                .push(format!("Skipped non-regular file {}", e.path().display()));
            continue;
        }
        if canaries.iter().any(|c| e.path() == c.path) {
            continue;
        }
        let bytes = match read_file(e.path(), 64 * 1024 * 1024) {
            Ok(b) => b,
            Err(e) => {
                r.errors.push(e.to_string());
                continue;
            }
        };
        if total + bytes.len() > 512 * 1024 * 1024 {
            r.errors.push("Total scan byte limit reached".into());
            break;
        }
        total += bytes.len();
        let p = temp.path().join(format!("sample-{}", items.len()));
        fs::write(&p, &bytes)?;
        scan_paths.push(p);
        items.push(ScanItem {
            path: e.path().into(),
            hash: digest(&bytes),
            detected: false,
            signature: String::new(),
        });
    }
    if items.is_empty() {
        r.errors.push("No eligible files were scanned".into());
        store.save(&r)?;
        return Ok(r);
    }
    let list = temp.path().join("files");
    fs::write(
        &list,
        scan_paths
            .iter()
            .map(|p| p.to_string_lossy())
            .collect::<Vec<_>>()
            .join("\n"),
    )?;
    let mut cmd = Command::new("clamscan");
    cmd.args([
        "--stdout",
        "--infected",
        "--alert-exceeds-max=yes",
        "--alert-encrypted=yes",
        "--max-filesize=64M",
        "--max-scansize=256M",
        "--max-files=256",
        "--max-recursion=16",
        "--bytecode-timeout=10000",
        "--file-list",
    ])
    .arg(list);
    if let Some(db) = own_database(store) {
        cmd.arg("--database").arg(db);
    }
    match runner::run(&mut cmd, 600, stop) {
        Err(e) => r.errors.push(e.to_string()),
        Ok(o) => {
            if !matches!(o.code, Some(0 | 1)) || o.interrupted || o.truncated {
                r.errors.push(format!(
                    "ClamAV incomplete: exit {:?}, cancelled/timed out {}, truncated {}. {}",
                    o.code, o.interrupted, o.truncated, o.stderr
                ));
            }
            let scanned = o.stdout.lines().find_map(|l| {
                l.strip_prefix("Scanned files:")
                    .and_then(|n| n.trim().parse::<usize>().ok())
            });
            if scanned.is_none_or(|n| n < items.len()) {
                r.errors
                    .push("ClamAV did not confirm examination of every submitted snapshot".into());
            }
            for (index, p) in scan_paths.iter().enumerate() {
                let prefix = format!("{}: ", p.display());
                for line in o.stdout.lines() {
                    if let Some(signature) = line
                        .strip_prefix(&prefix)
                        .and_then(|s| s.strip_suffix(" FOUND"))
                    {
                        items[index].detected = true;
                        items[index].signature = signature.into();
                        r.add("clamav.match",Severity::High,"A file matches a ClamAV signature",format!("{}\n{}\nSHA-256 {}",items[index].path.display(),signature,items[index].hash),"If you did not expect this file, quarantine it: Bigman keeps a private recovery copy you can restore. A Limits or Encrypted signature means the content was not fully examined.");
                        if signature.contains("Limits.Exceeded") || signature.contains("Encrypted")
                        {
                            r.errors.push(format!(
                                "Content inspection limited for {}",
                                items[index].path.display()
                            ));
                        }
                    }
                }
            }
            if o.code == Some(1) && !items.iter().any(|i| i.detected) {
                r.errors.push(
                    "ClamAV reported detections that could not be mapped to a snapshot".into(),
                );
            }
            r.details = format!("{}\n{}", o.stdout, o.stderr);
        }
    }
    r.coverage.push(format!(
        "{} file snapshots, {} bytes. See saved inventory for original paths and hashes.",
        items.len(),
        total
    ));
    store.write(&format!("artifacts/{}.scan.json", r.id), &items)?;
    store.save(&r)?;
    Ok(r)
}
pub fn update(store: &Store, stop: &Cancel) -> Result<Report> {
    let mut r = Report::new("Signature update", "Private ClamAV definitions");
    let db = store.root.join("signatures");
    private_dir(&db)?;
    let _lock = store.lock("update.lock")?;
    let config = tempfile::NamedTempFile::new_in(&store.root)?;
    fs::write(
        config.path(),
        "DatabaseMirror database.clamav.net\nTestDatabases yes\n",
    )?;
    let o = runner::run(
        Command::new("freshclam")
            .arg("--config-file")
            .arg(config.path())
            .arg("--datadir")
            .arg(&db),
        300,
        stop,
    )?;
    if !o.ok() {
        r.errors
            .push(format!("Definition update failed: {:?}", o.code));
    }
    r.details = format!("{}\n{}", o.stdout, o.stderr);
    r.coverage.push(
        "Uses a private signature directory; existing system updater configuration is preserved."
            .into(),
    );
    store.save(&r)?;
    Ok(r)
}
pub fn inventory(store: &Store, id: &str) -> Result<Vec<ScanItem>> {
    uuid::Uuid::parse_str(id)?;
    store.read(&format!("artifacts/{id}.scan.json"))
}
pub fn quarantine(store: &Store, id: &str, index: usize) -> Result<Quarantined> {
    let _lock = store.lock("quarantine.lock")?;
    let items = inventory(store, id)?;
    let item = items.get(index).context("Invalid scan item")?;
    if !item.detected {
        bail!("Only files with a scan finding can be quarantined");
    }
    let before = fs::symlink_metadata(&item.path)?;
    let bytes = read_file(&item.path, 64 * 1024 * 1024)?;
    if digest(&bytes) != item.hash {
        bail!("File changed since scan. Scan it again.");
    }
    let q = Quarantined {
        id: uuid::Uuid::new_v4().to_string(),
        original: item.path.clone(),
        hash: item.hash.clone(),
        created: now(),
        restored: false,
    };
    // Persist the recovery copy and manifest before removing anything.
    store.bytes(&format!("quarantine/{}.bin", q.id), &bytes)?;
    store.write(&format!("quarantine/{}.json", q.id), &q)?;
    let after = fs::symlink_metadata(&item.path)?;
    if before.ino() != after.ino() || before.dev() != after.dev() || after.file_type().is_symlink()
    {
        bail!("File identity changed; recovery copy retained, original untouched");
    }
    fs::remove_file(&item.path)?;
    Ok(q)
}
pub fn quarantined(store: &Store) -> Result<Vec<Quarantined>> {
    let mut out = vec![];
    for e in fs::read_dir(store.root.join("quarantine"))?.take(10000) {
        let p = e?.path();
        if p.extension().is_some_and(|e| e == "json") {
            out.push(serde_json::from_slice(&read_file(&p, MAX_INPUT)?)?);
        }
    }
    Ok(out)
}
pub fn restore(store: &Store, id: &str) -> Result<PathBuf> {
    uuid::Uuid::parse_str(id)?;
    let _lock = store.lock("quarantine.lock")?;
    let mut q: Quarantined = store.read(&format!("quarantine/{id}.json"))?;
    if q.restored {
        bail!("Already restored");
    }
    let b = read_file(
        &store.root.join(format!("quarantine/{id}.bin")),
        64 * 1024 * 1024,
    )?;
    if digest(&b) != q.hash {
        bail!("Recovery copy integrity check failed");
    }
    let parent = q.original.parent().context("Missing parent")?;
    if fs::canonicalize(parent)? != parent {
        bail!("Restore parent contains symlinks; choose a trusted destination manually");
    }
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(&q.original)
        .context("Restore refuses to overwrite an existing path")?;
    f.write_all(&b)?;
    f.sync_all()?;
    q.restored = true;
    store.write(&format!("quarantine/{id}.json"), &q)?;
    Ok(q.original)
}

pub fn scan(store: &Store, path: &Path, stop: &Cancel) -> Result<Report> {
    match scan_inner(store, path, stop) {
        Ok(r) => Ok(r),
        Err(e) => {
            let mut r = Report::new("File scan", path.display());
            r.errors.push(e.to_string());
            store.save(&r)?;
            Ok(r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quarantine_requires_matching_hash_and_restore_never_overwrites() {
        let d = tempfile::tempdir().unwrap();
        let s = Store::open(Some(d.path().join("state"))).unwrap();
        let p = d.path().join("sample");
        fs::write(&p, b"fixture").unwrap();
        let r = Report::new("File scan", "fixture");
        let items = vec![ScanItem {
            path: p.clone(),
            hash: digest(b"fixture"),
            detected: true,
            signature: "test".into(),
        }];
        s.write(&format!("artifacts/{}.scan.json", r.id), &items)
            .unwrap();
        fs::write(&p, b"changed").unwrap();
        assert!(quarantine(&s, &r.id, 0).is_err());
        fs::write(&p, b"fixture").unwrap();
        let q = quarantine(&s, &r.id, 0).unwrap();
        assert!(!p.exists());
        fs::write(&p, b"unrelated").unwrap();
        assert!(restore(&s, &q.id).is_err());
        assert_eq!(fs::read(&p).unwrap(), b"unrelated");
        fs::remove_file(&p).unwrap();
        restore(&s, &q.id).unwrap();
        assert_eq!(fs::read(p).unwrap(), b"fixture");
    }
}
