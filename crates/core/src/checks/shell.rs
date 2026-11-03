use crate::{
    model::{Report, Severity},
    store::digest,
};
use regex::Regex;

/// Deterministic capability rules: observations, never a malware/safety verdict.
pub fn analyze_content(content: &str) -> Report {
    let mut r = Report::new("Installer inspection", "stdin");
    r.sha256 = Some(digest(content.as_bytes()));
    r.coverage.push("Static capability rules only. Computed commands, sourced files, downloaded payloads and runtime behavior may be unresolved. No code was executed.".into());
    let rules = [
        (
            "shell.download",
            r"\b(curl|wget)\b",
            Severity::Review,
            "Downloads remote content",
            "Inspect each payload and verify its origin before execution.",
        ),
        (
            "shell.execute-download",
            r"\b(curl|wget)\b[^\n]*\|\s*(sudo\s+)?(ba|z|da)?sh\b",
            Severity::High,
            "Downloads and executes code",
            "Save and inspect the exact downloaded bytes first.",
        ),
        (
            "shell.privilege",
            r"\b(sudo|doas|su)\s",
            Severity::Review,
            "Requests elevated privileges",
            "Check why this operation needs administrator access.",
        ),
        (
            "shell.credentials",
            r"(?i)(\.ssh/|\.aws/|\.npmrc|\.git-credentials|Login Data|Cookies|\.mozilla|\.config/(google-chrome|chromium)|SSH_AUTH_SOCK|AWS_SECRET_ACCESS_KEY|NPM_TOKEN)",
            Severity::High,
            "References credentials or browser data",
            "Run without real credentials and investigate why these paths are needed.",
        ),
        (
            "shell.persistence",
            r"(crontab|systemctl\s+(--user\s+)?enable|\.config/(systemd|autostart)|\.(bashrc|zshrc|profile)|/etc/(cron|systemd)|authorized_keys)",
            Severity::Review,
            "References startup or persistent configuration",
            "Review the command that will run in future sessions.",
        ),
        (
            "shell.dynamic",
            r"(\beval\b|\bsource\s|\b(base64|openssl)\b|\b(python[23]?|node|perl|ruby)\s+(-[ec]|-))",
            Severity::Review,
            "Dynamic or encoded code requires further inspection",
            "The complete behavior is unresolved by static rules.",
        ),
        (
            "shell.permissions",
            // setuid or setgid (u+s, 4755, 2755) or world-writable (777, 666, o+w,
            // a+w). A bare +w honours the umask, so it is the owner's write bit.
            r"\bchmod\s+(-[A-Za-z]+\s+)*([ugoa]*[+=][rwxXt]*s|[ugoa]*[oa][ugoa]*[+=][rxXt]*w|[0-7]?[2-7][0-7]{3}\b|0?[0-7]{2}[2367]\b)",
            Severity::High,
            "Broad or privileged file permissions",
            "Prefer the minimum permissions required.",
        ),
        (
            "shell.delete",
            r"\brm\s+[^\n]*(-[a-zA-Z]*[rf]|\*)",
            Severity::Review,
            "Potentially destructive file removal",
            "Verify the expanded target paths.",
        ),
        (
            "shell.network-listener",
            r"\b(nc|ncat|netcat|socat)\b",
            Severity::Review,
            "Network utility can expose or forward services",
            "Review destinations and listening interfaces.",
        ),
    ];
    let compiled: Vec<_> = rules
        .iter()
        .map(|(id, pattern, severity, summary, action)| {
            (
                *id,
                Regex::new(pattern).expect("built-in rule"),
                severity,
                *summary,
                *action,
            )
        })
        .collect();
    for (line_no, line) in content.lines().enumerate() {
        if r.findings.len() >= 2000 {
            r.errors
                .push("Finding limit reached; remaining lines were not examined".into());
            break;
        }
        if line.trim_start().starts_with('#') {
            continue;
        }
        for (id, pattern, severity, summary, action) in &compiled {
            if pattern.is_match(line) {
                r.add(
                    id,
                    **severity,
                    summary,
                    format!("Line {}: {}", line_no + 1, line),
                    action,
                );
            }
        }
    }
    r.details = crate::model::clean(content);
    r
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capabilities_are_not_safety_claims() {
        let r = analyze_content("echo hello\n# sudo ignored");
        assert!(r.findings.is_empty());
        assert!(!r.text().contains("SAFE"));
    }
    #[test]
    fn only_setuid_and_world_writable_modes_are_broad() {
        let broad = |cmd: &str| {
            analyze_content(cmd)
                .findings
                .iter()
                .any(|f| f.rule == "shell.permissions")
        };
        for cmd in [
            "chmod 777 x",
            "chmod 0666 x",
            "chmod 4755 x",
            "chmod -R 2775 d",
            "chmod u+s x",
            "chmod o+w x",
            "chmod a+rw x",
            "chmod a+w x",
            "chmod go+w x",
        ] {
            assert!(broad(cmd), "{cmd}");
        }
        for cmd in [
            "chmod 755 x",
            "chmod 0644 x",
            "chmod 1755 d",
            "chmod +x x",
            "chmod u+rwx x",
            "chmod -R go-w d",
            "chmod +w x",
            "chmod -R u+w d",
        ] {
            assert!(!broad(cmd), "{cmd}");
        }
    }
    #[test]
    fn reports_credential_reference_with_line() {
        let r = analyze_content("echo hello\ncat ~/.aws/credentials");
        assert!(r
            .findings
            .iter()
            .any(|f| f.rule == "shell.credentials" && f.evidence.starts_with("Line 2")));
    }
}
