use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Info,
    Review,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub rule: String,
    pub severity: Severity,
    pub summary: String,
    pub evidence: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: String,
    pub created: u64,
    pub kind: String,
    pub subject: String,
    pub sha256: Option<String>,
    pub findings: Vec<Finding>,
    pub coverage: Vec<String>,
    pub errors: Vec<String>,
    pub details: String,
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
            coverage: vec![],
            errors: vec![],
            details: String::new(),
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
    pub fn status(&self) -> &'static str {
        if !self.errors.is_empty() {
            "Incomplete"
        } else if !self.findings.is_empty() {
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
        for c in &self.coverage {
            out.push_str(&format!("\nCoverage: {c}\n"));
        }
        if !self.details.is_empty() {
            out.push_str(&format!("\nDetails\n{}", clean(&self.details)));
        }
        out
    }
}
