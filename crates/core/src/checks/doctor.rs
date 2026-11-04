use crate::{
    model::{now, Report, Severity},
    runner::{self, Cancel},
    store::Store,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::Command};
use ts_rs::TS;

/// Optional tools Bigman can drive, with what each one adds.
pub const TOOLS: [(&str, &str); 14] = [
    ("clamscan", "Known-malware file scanning"),
    ("freshclam", "Signature updates"),
    ("shellcheck", "Shell correctness checks"),
    ("bwrap", "Restricted execution"),
    ("strace", "Execution traces"),
    ("podman", "Disposable containers"),
    ("lynis", "System hardening audit"),
    ("osv-scanner", "Dependency vulnerability checks"),
    ("osqueryi", "System inventory"),
    ("yr", "YARA-X rules"),
    ("pdfid.py", "PDF triage"),
    ("pdf-parser.py", "PDF structural inspection"),
    ("pdfinfo", "PDF metadata and links"),
    ("opensnitch-ui", "Application firewall interface"),
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct ToolState {
    pub name: String,
    pub purpose: String,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct SignatureState {
    /// Modification time of the newest daily database file.
    pub updated: u64,
    pub location: PathBuf,
}

pub fn tools() -> Vec<ToolState> {
    TOOLS
        .iter()
        .map(|(name, purpose)| ToolState {
            name: (*name).into(),
            purpose: (*purpose).into(),
            available: runner::available(name),
        })
        .collect()
}

/// The private Bigman database when present, otherwise the distro default.
pub fn signatures(store: &Store) -> Option<SignatureState> {
    let root = crate::checks::scan::own_database(store)
        .unwrap_or_else(|| PathBuf::from("/var/lib/clamav"));
    let newest = ["daily.cld", "daily.cvd"]
        .iter()
        .filter_map(|name| fs::metadata(root.join(name)).ok()?.modified().ok())
        .filter_map(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .max()?;
    Some(SignatureState {
        updated: newest,
        location: root,
    })
}

/// What the checkup's rules read. Gathered by `doctor`; fixtures supply it directly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorInputs {
    pub now: u64,
    pub tools: Vec<ToolState>,
    pub signatures: Option<SignatureState>,
    /// The monitor heartbeat, only when it is fresh.
    pub monitor: Option<crate::checks::canary::Health>,
    /// Global git credential helpers, or None when git is not installed.
    pub git_helpers: Option<Vec<String>>,
}

/// The checkup's rules, over gathered inputs. No commands, no paths.
pub fn assess(r: &mut Report, inputs: &DoctorInputs) {
    for tool in &inputs.tools {
        if tool.available {
            r.details
                .push_str(&format!("Available: {} — {}\n", tool.name, tool.purpose));
        } else {
            r.add("tool.missing",Severity::Info,&format!("{} is not installed", tool.name),&tool.purpose,"Open Setup for an installation plan. Optional tools are not required for unrelated features.");
        }
    }
    match &inputs.signatures {
        None => r.add(
            "clamav.no-definitions",
            Severity::High,
            "No definitions found in known locations",
            "Private Bigman signatures and /var/lib/clamav checked",
            "Update definitions or confirm a custom ClamAV database location.",
        ),
        Some(sig) if inputs.now.saturating_sub(sig.updated) > 3 * 86400 => r.add(
            "clamav.stale",
            Severity::Review,
            "Definitions appear stale",
            format!(
                "Daily database file is {} days old",
                inputs.now.saturating_sub(sig.updated) / 86400
            ),
            "Update definitions. File age is a freshness signal, not a verified database version.",
        ),
        Some(_) => {}
    }
    match &inputs.monitor {
        Some(h) => {
            r.details.push_str(&format!(
                "Canary monitor: {}/{} configured paths watched\n",
                h.watched, h.configured
            ));
            r.errors.extend(h.errors.iter().cloned());
        }
        None => r.add(
            "monitor.stopped",
            Severity::Review,
            "Canary monitoring is not reporting a fresh heartbeat",
            "No heartbeat within ten seconds",
            "Start monitoring from Canaries or enable the user service.",
        ),
    }
    if inputs
        .git_helpers
        .iter()
        .flatten()
        .any(|l| l.trim() == "store" || l.trim().starts_with("store "))
    {
        r.add(
            "git.plaintext-credentials",
            Severity::Review,
            "Git uses plaintext credential storage",
            "Global credential.helper includes store; credential contents were not read",
            "Migrate credentials to a supported desktop keyring helper and revoke obsolete tokens.",
        );
    }
}

pub fn doctor(store: &Store, stop: &Cancel) -> Result<Report> {
    let mut r = Report::new("Security checkup", "Local Linux workstation");
    let git_helpers = runner::available("git").then(|| {
        runner::run(
            Command::new("git").args(["config", "--global", "--get-all", "credential.helper"]),
            5,
            stop,
        )
        .map(|o| o.stdout.lines().map(String::from).collect())
        .unwrap_or_default()
    });
    let inputs = DoctorInputs {
        now: now(),
        tools: tools(),
        signatures: signatures(store),
        monitor: store
            .read::<crate::checks::canary::Health>("monitor-health.json")
            .ok()
            .filter(|h| now().saturating_sub(h.updated) < 10),
        git_helpers,
    };
    assess(&mut r, &inputs);
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
