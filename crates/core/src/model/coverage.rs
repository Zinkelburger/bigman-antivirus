use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// What a check examined and what it could not. "Incomplete" is derived from
/// `skipped` (and from verbatim tool errors on the report), never from prose.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(from = "CoverageRepr")]
#[ts(export)]
pub struct Coverage {
    /// Names of the checks that ran to completion.
    pub checked: Vec<String>,
    /// Scope statements: what the checks can and cannot establish.
    pub notes: Vec<String>,
    pub skipped: Vec<Skipped>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct Skipped {
    pub what: String,
    pub why: SkipReason,
    /// Verbatim detail from the tool or OS, shown as evidence.
    pub detail: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum SkipReason {
    ToolMissing,
    PermissionDenied,
    NeedsHelper,
    LimitReached,
    Cancelled,
    Unreadable,
    Failed,
    NotApplicable,
    DaemonUnavailable,
}

impl Coverage {
    pub fn push(&mut self, note: String) {
        self.notes.push(note);
    }
    pub fn checked(&mut self, what: impl Into<String>) {
        self.checked.push(what.into());
    }
    pub fn skip(&mut self, what: impl Into<String>, why: SkipReason, detail: impl Into<String>) {
        self.skipped.push(Skipped {
            what: what.into(),
            why,
            detail: detail.into(),
        });
    }
    pub fn extend(&mut self, other: Coverage) {
        self.checked.extend(other.checked);
        self.notes.extend(other.notes);
        self.skipped.extend(other.skipped);
    }
}

/// Reports written before coverage was structured stored a list of notes.
#[derive(Deserialize)]
#[serde(untagged)]
enum CoverageRepr {
    Notes(Vec<String>),
    Full {
        #[serde(default)]
        checked: Vec<String>,
        #[serde(default)]
        notes: Vec<String>,
        #[serde(default)]
        skipped: Vec<Skipped>,
    },
}

impl From<CoverageRepr> for Coverage {
    fn from(r: CoverageRepr) -> Self {
        match r {
            CoverageRepr::Notes(notes) => Coverage {
                notes,
                ..Default::default()
            },
            CoverageRepr::Full {
                checked,
                notes,
                skipped,
            } => Coverage {
                checked,
                notes,
                skipped,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reads_legacy_note_lists() {
        let old: Coverage = serde_json::from_str(r#"["Static rules only."]"#).unwrap();
        assert_eq!(old.notes, vec!["Static rules only."]);
        let mut new = Coverage::default();
        new.skip("ShellCheck", SkipReason::ToolMissing, "not installed");
        let back: Coverage = serde_json::from_str(&serde_json::to_string(&new).unwrap()).unwrap();
        assert_eq!(back, new);
    }
}
