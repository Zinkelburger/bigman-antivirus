mod coverage;
pub use coverage::{Coverage, SkipReason, Skipped};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Severity {
    Info,
    Review,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Finding {
    pub rule: String,
    pub severity: Severity,
    pub summary: String,
    pub evidence: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Report {
    pub id: String,
    pub created: u64,
    pub kind: String,
    pub subject: String,
    pub sha256: Option<String>,
    pub findings: Vec<Finding>,
    pub coverage: Coverage,
    pub errors: Vec<String>,
    pub details: String,
    /// Present only on history summaries, which are shortened: open the report by
    /// id for full evidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub summary: Option<SummaryCounts>,
}

/// The real counts behind a shortened history summary.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct SummaryCounts {
    pub findings: u32,
    /// Findings at Review or High severity.
    pub to_review: u32,
    pub errors: u32,
}

pub fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
pub fn clean(s: &str) -> String {
    s.chars()
        .flat_map(|c| {
            if (c.is_control() && c != '\n' && c != '\t') || matches!(c, '\u{061c}' | '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}') {
                c.escape_unicode().collect::<Vec<_>>()
            } else {
                vec![c]
            }
        })
        .collect()
}

impl Report {
    pub fn new(kind: &str, subject: impl ToString) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created: now(),
            kind: kind.into(),
            subject: subject.to_string(),
            sha256: None,
            findings: vec![],
            coverage: Coverage::default(),
            errors: vec![],
            details: String::new(),
            summary: None,
        }
    }
    pub fn add(
        &mut self,
        rule: &str,
        severity: Severity,
        summary: &str,
        evidence: impl ToString,
        action: &str,
    ) {
        self.findings.push(Finding {
            rule: rule.into(),
            severity,
            summary: summary.into(),
            evidence: clean(&evidence.to_string()),
            action: action.into(),
        });
    }
    /// Tool errors or skipped coverage make a result incomplete.
    pub fn is_incomplete(&self) -> bool {
        !self.errors.is_empty() || !self.coverage.skipped.is_empty()
    }
    /// Info findings are notes; only Review and High are something to review,
    /// matching the CLI exit codes.
    pub fn status(&self) -> &'static str {
        if self.is_incomplete() {
            "Incomplete"
        } else if self.findings.iter().any(|f| f.severity != Severity::Info) {
            "Findings to review"
        } else {
            "No findings from these checks"
        }
    }
    pub fn text(&self) -> String {
        let mut out = format!(
            "{} — {}\n{}\nReport {}\n",
            self.kind,
            clean(&self.subject),
            self.status(),
            self.id
        );
        if let Some(hash) = &self.sha256 {
            out.push_str(&format!("SHA-256: {hash}\n"));
        }
        for f in &self.findings {
            out.push_str(&format!(
                "\n[{:?}] {} ({})\n{}\nNext: {}\n",
                f.severity, f.summary, f.rule, f.evidence, f.action
            ));
        }
        for e in &self.errors {
            out.push_str(&format!("\nIncomplete: {}\n", clean(e)));
        }
        for s in &self.coverage.skipped {
            out.push_str(&format!(
                "\nNot checked: {} ({:?}) {}\n",
                s.what,
                s.why,
                clean(&s.detail)
            ));
        }
        for c in &self.coverage.notes {
            out.push_str(&format!("\nCoverage: {c}\n"));
        }
        if !self.details.is_empty() {
            out.push_str(&format!("\nDetails\n{}", clean(&self.details)));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn info_notes_are_not_findings_to_review() {
        let mut r = Report::new("Security checkup", "fixture");
        r.add(
            "tool.missing",
            Severity::Info,
            "lynis is not installed",
            "",
            "",
        );
        assert_eq!(r.status(), "No findings from these checks");
        r.add(
            "shell.privilege",
            Severity::Review,
            "Requests elevated privileges",
            "",
            "",
        );
        assert_eq!(r.status(), "Findings to review");
        r.coverage
            .skip("ShellCheck", SkipReason::ToolMissing, "not installed");
        assert_eq!(r.status(), "Incomplete");
    }
}
