use crate::{
    model::{now, Report, Severity},
    runner::{self, Cancel},
    store::{digest, home, read_file, Store, MAX_INPUT},
};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
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
    let decoys = crate::canary::list(store).unwrap_or_default();
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
    if store.root.join("baseline.json").exists() {
        let old: Baseline = store.read("baseline.json")?;
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
    } else {
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
    }
    Ok(r)
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
    store.write("baseline.json", &current)?;
    let mut r = Report::new("Baseline accepted", "Reviewed startup inventory");
    r.sha256 = original.sha256;
    r.coverage.push(format!("Accepted exactly the inventory from report {id}. Baseline is prior state, not proof of trust."));
    store.save(&r)?;
    Ok(r)
}

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
    for root in
        [crate::scanner::own_database(store).unwrap_or_else(|| PathBuf::from("/var/lib/clamav"))]
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
    match store.read::<crate::canary::Health>("monitor-health.json") {
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
pub fn integration(store: &Store, tool: &str, target: &Path, stop: &Cancel) -> Result<Report> {
    let path = fs::canonicalize(target)?;
    let tmp = tempfile::tempdir_in(&store.root)?;
    let mut r = Report::new(&format!("{tool} integration"), path.display());
    let mut cmd = match tool {
        "lynis" => {
            let mut c = Command::new("lynis");
            c.args(["audit", "system", "--quick", "--no-colors", "--report-file"])
                .arg(tmp.path().join("lynis-report.dat"))
                .arg("--log-file")
                .arg(tmp.path().join("lynis.log"));
            c
        }
        "osv" => {
            let mut c = Command::new("osv-scanner");
            c.args(["scan", "source", "--format=json", "--recursive"])
                .arg(&path);
            c
        }
        "osquery" => {
            let mut c = Command::new("osqueryi");
            c.args(["--json", "SELECT pid, name, path, uid FROM processes;"]);
            c
        }
        "yara" => {
            bail!("YARA requires an explicit reviewed rules file; use yara RULES TARGET");
        }
        _ => bail!("Unsupported integration"),
    };
    let o = runner::run(&mut cmd, 300, stop)?;
    if !(o.ok() || (tool == "osv" && o.code == Some(1) && !o.interrupted && !o.truncated)) {
        r.errors.push(format!("Tool returned {:?}; read output for findings or errors. timeout/cancel={}, truncated={}",o.code,o.interrupted,o.truncated));
    }
    r.details = format!("{}\n{}", o.stdout, o.stderr);
    if tool == "lynis" {
        if let Ok(bytes) = read_file(&tmp.path().join("lynis-report.dat"), MAX_INPUT) {
            for line in String::from_utf8_lossy(&bytes)
                .lines()
                .filter(|l| l.starts_with("warning[]=") || l.starts_with("suggestion[]="))
            {
                r.add(
                    "lynis.finding",
                    Severity::Review,
                    "Lynis recommendation",
                    line,
                    "Review the upstream test ID before changing configuration.",
                );
            }
        }
    }
    if tool == "osv" {
        match serde_json::from_str::<serde_json::Value>(&o.stdout) {
            Ok(v) if v["results"].is_array() => {
                for source in v["results"].as_array().unwrap() {
                    if let Some(packages)=source["packages"].as_array() {
                        for package in packages { if let Some(vulns)=package["vulnerabilities"].as_array() { for vuln in vulns {
                            r.add("osv.advisory", Severity::Review, vuln["id"].as_str().unwrap_or("Dependency advisory"), format!("{}@{}\n{}", package["package"]["name"], package["package"]["version"], vuln["summary"]), "Review affected and fixed versions in the advisory. A vulnerability is not evidence of installed malware.");
                        } } }
                    }
                }
            }
            _ => r.errors.push("OSV JSON output could not be interpreted; inspect raw output and installed tool version".into()),
        }
        r.coverage.push("OSV performs deterministic advisory matching and may contact vulnerability services. Dependency vulnerabilities and malware are different findings. Raw JSON retained.".into());
    }
    store.save(&r)?;
    Ok(r)
}
pub fn yara(store: &Store, rules: &Path, target: &Path, stop: &Cancel) -> Result<Report> {
    let rules = fs::canonicalize(rules)?;
    let target = fs::canonicalize(target)?;
    let mut r = Report::new("YARA-X scan", target.display());
    let o = runner::run(
        Command::new("yr").arg("scan").arg(&rules).arg(&target),
        120,
        stop,
    )?;
    if !o.ok() {
        r.errors
            .push(format!("YARA-X incomplete: {:?} {}", o.code, o.stderr));
    }
    if !o.stdout.trim().is_empty() {
        r.add(
            "yara.output",
            Severity::Review,
            "YARA-X returned output",
            &o.stdout,
            "Review matched rule provenance and evidence; rule matches are not proof of malware.",
        );
    }
    r.details = o.stderr;
    store.save(&r)?;
    Ok(r)
}
