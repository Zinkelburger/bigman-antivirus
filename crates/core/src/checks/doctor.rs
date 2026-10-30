use crate::{
    model::{now, Report, Severity},
    runner::{self, Cancel},
    store::Store,
};
use anyhow::Result;
use std::{fs, path::PathBuf, process::Command};

pub fn doctor(store: &Store, stop: &Cancel) -> Result<Report> {
    let mut r = Report::new("Security checkup", "Local Linux workstation");
    for (tool, purpose) in [
        ("clamscan", "Known-malware file scanning"),
        ("freshclam", "Signature updates"),
        ("shellcheck", "Shell correctness checks"),
        ("bwrap", "Restricted execution"),
        ("strace", "Execution traces"),
        ("lynis", "System hardening audit"),
        ("osv-scanner", "Dependency vulnerability checks"),
        ("osqueryi", "System inventory"),
        ("yr", "YARA-X rules"),
        ("pdfid.py", "PDF triage"),
        ("pdf-parser.py", "PDF structural inspection"),
        ("pdfinfo", "PDF metadata and links"),
        ("opensnitch-ui", "Application firewall interface"),
    ] {
        if runner::available(tool) {
            r.details
                .push_str(&format!("Available: {tool} — {purpose}\n"));
        } else {
            r.add("tool.missing",Severity::Info,&format!("{tool} is not installed"),purpose,"Open Tools for a supported installation plan. Optional tools are not required for unrelated features.");
        }
    }
    let mut newest = 0;
    for root in [crate::checks::scan::own_database(store)
        .unwrap_or_else(|| PathBuf::from("/var/lib/clamav"))]
    {
        for name in ["daily.cld", "daily.cvd"] {
            if let Ok(m) = fs::metadata(root.join(name)) {
                if let Ok(t) = m.modified().and_then(|t| {
                    t.duration_since(std::time::UNIX_EPOCH)
                        .map_err(std::io::Error::other)
                }) {
                    newest = newest.max(t.as_secs());
                }
            }
        }
    }
    if newest == 0 {
        r.add(
            "clamav.no-definitions",
            Severity::High,
            "No definitions found in known locations",
            "Private Bigman signatures and /var/lib/clamav checked",
            "Update definitions or confirm a custom ClamAV database location.",
        );
    } else if now().saturating_sub(newest) > 3 * 86400 {
        r.add(
            "clamav.stale",
            Severity::Review,
            "Definitions appear stale",
            format!(
                "Daily database file is {} days old",
                now().saturating_sub(newest) / 86400
            ),
            "Update definitions. File age is a freshness signal, not a verified database version.",
        );
    }
    match store.read::<crate::checks::canary::Health>("monitor-health.json") {
        Ok(h) if now().saturating_sub(h.updated) < 10 => {
            r.details.push_str(&format!(
                "Canary monitor: {}/{} configured paths watched\n",
                h.watched, h.configured
            ));
            r.errors.extend(h.errors);
        }
        _ => r.add(
            "monitor.stopped",
            Severity::Review,
            "Canary monitoring is not reporting a fresh heartbeat",
            "No heartbeat within ten seconds",
            "Start monitoring from Canaries or enable the user service.",
        ),
    }
    if runner::available("git") {
        if let Ok(o) = runner::run(
            Command::new("git").args(["config", "--global", "--get-all", "credential.helper"]),
            5,
            stop,
        ) {
            if o.stdout
                .lines()
                .any(|l| l.trim() == "store" || l.trim().starts_with("store "))
            {
                r.add("git.plaintext-credentials",Severity::Review,"Git uses plaintext credential storage","Global credential.helper includes store; credential contents were not read","Migrate credentials to a supported desktop keyring helper and revoke obsolete tokens.");
            }
        }
    }
    for (name, args) in [
        ("getenforce", vec![]),
        ("ss", vec!["-lntup"]),
        (
            "systemctl",
            vec!["is-active", "firewalld", "ufw", "opensnitch"],
        ),
    ] {
        if runner::available(name) {
            let o = runner::run(Command::new(name).args(args), 10, stop)?;
            r.details.push_str(&format!(
                "\n{name} (exit {:?})\n{}{}",
                o.code, o.stdout, o.stderr
            ));
        }
    }
    r.coverage.push("Read-only local tool/configuration check. No vulnerability feeds refreshed, no credentials read, and no claim that existing malware is absent. Listening sockets are inventory, not detections.".into());
    store.save(&r)?;
    Ok(r)
}
