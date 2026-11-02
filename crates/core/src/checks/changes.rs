use crate::{
    model::{now, Report, Severity},
    store::{digest, home, read_file, Store, MAX_INPUT},
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::PathBuf};
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    pub hash: String,
    pub bytes: u64,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Baseline {
    pub created: u64,
    pub entries: BTreeMap<PathBuf, Entry>,
    pub errors: Vec<String>,
}
fn snapshot(store: &Store) -> Baseline {
    let roots = [
        home().join(".config/systemd/user"),
        home().join(".config/autostart"),
        home().join(".ssh/authorized_keys"),
        home().join(".bashrc"),
        home().join(".zshrc"),
        home().join(".profile"),
        PathBuf::from("/etc/systemd/system"),
        PathBuf::from("/etc/cron.d"),
        PathBuf::from("/etc/crontab"),
        PathBuf::from("/etc/ld.so.preload"),
    ];
    let mut b = Baseline {
        created: now(),
        entries: BTreeMap::new(),
        errors: vec![],
    };
    let decoys = crate::checks::canary::list(store).unwrap_or_default();
    let mut visited = 0;
    let mut total_bytes = 0usize;
    for root in roots {
        if !root.exists() {
            continue;
        }
        for e in walkdir::WalkDir::new(root).follow_links(false).max_depth(8) {
            visited += 1;
            if visited > 10000 {
                b.errors.push("Snapshot entry limit exceeded".into());
                return b;
            }
            let e = match e {
                Ok(e) => e,
                Err(e) => {
                    b.errors.push(e.to_string());
                    continue;
                }
            };
            if e.file_type().is_dir() {
                continue;
            }
            if decoys.iter().any(|c| c.path == e.path()) {
                continue;
            }
            let bytes = if e.file_type().is_symlink() {
                fs::read_link(e.path())
                    .map(|p| p.as_os_str().as_encoded_bytes().to_vec())
                    .map_err(anyhow::Error::from)
            } else {
                read_file(e.path(), MAX_INPUT)
            };
            match bytes {
                Ok(bytes) => {
                    total_bytes += bytes.len();
                    if total_bytes > 64 * 1024 * 1024 {
                        b.errors.push("Snapshot byte limit exceeded".into());
                        return b;
                    }
                    b.entries.insert(
                        e.path().into(),
                        Entry {
                            hash: digest(&bytes),
                            bytes: bytes.len() as u64,
                        },
                    );
                }
                Err(e) => b.errors.push(e.to_string()),
            }
        }
    }
    b
}
pub fn changes(store: &Store) -> Result<Report> {
    let _lock = store.lock("baseline.lock")?;
    let current = snapshot(store);
    let mut r = Report::new("Persistence changes", "Selected startup configuration");
    r.errors = current.errors.clone();
    r.sha256 = Some(digest(&serde_json::to_vec(&current.entries)?));
    r.coverage.push("Hashes selected user/system startup files and symlink destinations. Baseline means previously observed, not trusted. Does not identify the writer or cover every persistence mechanism; inspect initial entries too.".into());
    let old = if store.root.join("baseline.json").exists() {
        Some(store.read::<Baseline>("baseline.json")?)
    } else {
        None
    };
    compare(&mut r, old.as_ref(), &current);
    Ok(r)
}

/// The startup-file rules: what is new, changed or removed against the baseline,
/// or the initial inventory when there is none.
pub fn compare(r: &mut Report, old: Option<&Baseline>, current: &Baseline) {
    let Some(old) = old else {
        for (p, e) in &current.entries {
            r.add(
                "persistence.initial",
                Severity::Info,
                "Initial startup inventory",
                format!("{}\nSHA-256 {}", p.display(), e.hash),
                "Review before accepting this as the initial baseline.",
            );
        }
        r.coverage.push(
            "No baseline exists yet. Accept baseline only after reviewing this inventory.".into(),
        );
        return;
    };
    for (p, v) in &current.entries {
        if old.entries.get(p) != Some(v) {
            r.add(
                "persistence.change",
                Severity::Review,
                if old.entries.contains_key(p) {
                    "Startup entry changed"
                } else {
                    "New startup entry"
                },
                format!("{}\nSHA-256 {}", p.display(), v.hash),
                "Review the file and its owning package before accepting a new baseline.",
            );
        }
    }
    // An unreadable entry is not evidence that it was removed.
    if current.errors.is_empty() {
        for p in old
            .entries
            .keys()
            .filter(|p| !current.entries.contains_key(*p))
        {
            r.add(
                "persistence.removed",
                Severity::Review,
                "Startup entry removed",
                p.display(),
                "Check whether an intentional configuration or package change explains this.",
            );
        }
    }
}
/// Puts back the baseline that the last acceptance replaced.
pub fn restore_previous_baseline(store: &Store) -> Result<()> {
    let _lock = store.lock("baseline.lock")?;
    let previous: Baseline = store
        .read("baseline-previous.json")
        .context("There is no earlier baseline to restore")?;
    store.write("baseline.json", &previous)?;
    std::fs::remove_file(store.root.join("baseline-previous.json"))?;
    Ok(())
}
pub fn has_previous_baseline(store: &Store) -> bool {
    store.root.join("baseline-previous.json").exists()
}
pub fn accept_reviewed_baseline(store: &Store, id: &str) -> Result<Report> {
    let _lock = store.lock("baseline.lock")?;
    let original = store.report(id)?;
    if original.kind != "Persistence changes" || !original.errors.is_empty() {
        bail!("Choose a complete startup inventory report first");
    }
    let current = snapshot(store);
    if !current.errors.is_empty()
        || original.sha256 != Some(digest(&serde_json::to_vec(&current.entries)?))
    {
        bail!("Startup inventory changed or became unreadable after review; compare it again before accepting");
    }
    // Keep the baseline being replaced so the acceptance can be undone.
    if store.root.join("baseline.json").exists() {
        let previous: Baseline = store.read("baseline.json")?;
        store.write("baseline-previous.json", &previous)?;
    }
    store.write("baseline.json", &current)?;
    let mut r = Report::new("Baseline accepted", "Reviewed startup inventory");
    r.sha256 = original.sha256;
    r.coverage.push(format!("Accepted exactly the inventory from report {id}. Baseline is prior state, not proof of trust."));
    store.save(&r)?;
    Ok(r)
}
