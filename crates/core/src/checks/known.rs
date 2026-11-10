//! Things you marked as known. One record per key, pinned to what the thing
//! was when you marked it, so the same name with different contents is not the
//! thing you marked: it comes back as changed, with what differs.
//!
//! Marking records what you saw; it is not a judgement that the thing is harmless.
use crate::{model::now, store::Store};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use ts_rs::TS;

const FILE: &str = "known.json";
const LOCK: &str = "known.lock";

/// One labelled value describing a row, shown as a key and value.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct Fact {
    pub label: String,
    pub value: String,
    /// Whether a different value means a different thing. Process IDs and counts
    /// change on their own and are not pinned.
    #[serde(default)]
    pub pinned: bool,
}

impl Fact {
    pub fn pinned(label: &str, value: impl ToString) -> Fact {
        Fact {
            label: label.into(),
            value: value.to_string(),
            pinned: true,
        }
    }
    pub fn shown(label: &str, value: impl ToString) -> Fact {
        Fact {
            label: label.into(),
            value: value.to_string(),
            pinned: false,
        }
    }
}

/// The digest of a row's pinned facts.
pub fn pin(facts: &[Fact]) -> String {
    let mut text = String::new();
    for f in facts.iter().filter(|f| f.pinned) {
        text.push_str(&format!("{}\0{}\n", f.label, f.value));
    }
    crate::store::digest(text.as_bytes())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Known {
    pub pin: String,
    pub title: String,
    pub facts: Vec<Fact>,
    pub at: u64,
    /// The record this one replaced, for Undo.
    #[serde(default)]
    pub previous: Option<Box<Known>>,
}

pub type KnownMap = BTreeMap<String, Known>;

/// How a row compares with what you marked.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Standing {
    /// Different from the version you marked as known.
    Changed,
    /// Not marked.
    New,
    /// Exactly what you marked as known.
    Known,
}

pub fn load(store: &Store) -> Result<KnownMap> {
    if !store.root.join(FILE).exists() {
        return Ok(KnownMap::new());
    }
    store.read(FILE)
}

/// A row's standing, when it was marked, and which pinned facts differ now.
pub fn standing(map: &KnownMap, key: &str, facts: &[Fact]) -> (Standing, Option<u64>, Vec<String>) {
    let Some(k) = map.get(key) else {
        return (Standing::New, None, vec![]);
    };
    if k.pin == pin(facts) {
        return (Standing::Known, Some(k.at), vec![]);
    }
    let was: BTreeMap<&str, &str> = k
        .facts
        .iter()
        .filter(|f| f.pinned)
        .map(|f| (f.label.as_str(), f.value.as_str()))
        .collect();
    let mut changed = vec![];
    for f in facts.iter().filter(|f| f.pinned) {
        match was.get(f.label.as_str()) {
            Some(old) if *old != f.value => changed.push(
                list_change(old, &f.value)
                    .map(|c| format!("{}: {c}", f.label))
                    .unwrap_or_else(|| format!("{}: was {}, now {}", f.label, old, f.value)),
            ),
            None => changed.push(format!("{}: now {}", f.label, f.value)),
            _ => {}
        }
    }
    for (label, old) in &was {
        if !facts.iter().any(|f| f.pinned && f.label == *label) {
            changed.push(format!("{label}: was {old}, now absent"));
        }
    }
    (Standing::Changed, Some(k.at), changed)
}

/// For values that are lists ("a, b, c"), what was added and removed.
fn list_change(old: &str, new: &str) -> Option<String> {
    if !old.contains(", ") && !new.contains(", ") {
        return None;
    }
    let was: std::collections::BTreeSet<&str> = old.split(", ").collect();
    let now: std::collections::BTreeSet<&str> = new.split(", ").collect();
    let added: Vec<&str> = now.difference(&was).copied().collect();
    let removed: Vec<&str> = was.difference(&now).copied().collect();
    let mut parts = vec![];
    if !added.is_empty() {
        parts.push(format!("added {}", added.join(", ")));
    }
    if !removed.is_empty() {
        parts.push(format!("removed {}", removed.join(", ")));
    }
    (!parts.is_empty()).then(|| parts.join("; "))
}

/// Records a row as known, keeping the record it replaces for Undo.
pub fn mark(store: &Store, key: &str, title: &str, facts: &[Fact]) -> Result<()> {
    let _lock = store.lock(LOCK)?;
    let mut all = load(store)?;
    let previous = all.remove(key).map(|mut p| {
        p.previous = None;
        Box::new(p)
    });
    all.insert(
        key.to_string(),
        Known {
            pin: pin(facts),
            title: title.to_string(),
            facts: facts.to_vec(),
            at: now(),
            previous,
        },
    );
    store.write(FILE, &all)
}

/// Takes a mark back, restoring the one it replaced. Returns the title and
/// whether an earlier mark came back.
pub fn forget(store: &Store, key: &str) -> Result<(String, bool)> {
    let _lock = store.lock(LOCK)?;
    let mut all = load(store)?;
    let k = all.remove(key).context("That is not marked as known")?;
    let restored = k.previous.is_some();
    if let Some(p) = k.previous {
        all.insert(key.to_string(), *p);
    }
    store.write(FILE, &all)?;
    Ok((k.title, restored))
}

pub fn title(store: &Store, key: &str) -> Option<String> {
    load(store).ok()?.get(key).map(|k| k.title.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facts(path: &str, pid: u32) -> Vec<Fact> {
        vec![
            Fact::pinned("Program", path),
            Fact::pinned("File", "inode 7, 120 bytes"),
            Fact::shown("Process ID", pid),
        ]
    }

    #[test]
    fn a_mark_holds_until_a_pinned_fact_changes() {
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        let key = "process.exe-memfd:tool";
        assert_eq!(
            standing(&load(&store).unwrap(), key, &facts("/opt/tool", 10)).0,
            Standing::New
        );
        mark(
            &store,
            key,
            "tool runs from memory",
            &facts("/opt/tool", 10),
        )
        .unwrap();
        let map = load(&store).unwrap();
        // A new process ID is the same thing.
        assert_eq!(
            standing(&map, key, &facts("/opt/tool", 99)).0,
            Standing::Known
        );
        // A different program under the same key is not.
        let (s, at, changed) = standing(&map, key, &facts("/tmp/tool", 99));
        assert_eq!(s, Standing::Changed);
        assert!(at.is_some());
        assert_eq!(changed, vec!["Program: was /opt/tool, now /tmp/tool"]);
        assert_eq!(
            list_change("a, b, c", "a, c, d, e").unwrap(),
            "added d, e; removed b"
        );
    }

    #[test]
    fn forgetting_restores_the_mark_it_replaced() {
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        mark(&store, "k", "first", &facts("/a", 1)).unwrap();
        mark(&store, "k", "second", &facts("/b", 1)).unwrap();
        assert_eq!(forget(&store, "k").unwrap(), ("second".to_string(), true));
        assert_eq!(
            standing(&load(&store).unwrap(), "k", &facts("/a", 5)).0,
            Standing::Known
        );
        assert_eq!(forget(&store, "k").unwrap(), ("first".to_string(), false));
        assert!(forget(&store, "k").is_err());
        assert!(load(&store).unwrap().is_empty());
    }
}
