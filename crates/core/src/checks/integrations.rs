use crate::{
    model::{Report, Severity},
    runner::{self, Cancel},
    store::{read_file, Store, MAX_INPUT},
};
use anyhow::{bail, Result};
use std::{fs, path::Path, process::Command};

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
