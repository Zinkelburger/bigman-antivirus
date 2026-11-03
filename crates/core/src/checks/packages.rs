use crate::{
    checks::shell,
    model::{Report, Severity},
    store::{digest, read_file, Store, MAX_INPUT},
};
use anyhow::{bail, Context, Result};
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};

pub fn packages(store: &Store, path: &Path) -> Result<Report> {
    let r = inventory(path)?;
    store.save(&r)?;
    Ok(r)
}

/// The package-script inventory, not yet saved.
pub fn inventory(path: &Path) -> Result<Report> {
    let mut r = Report::new("Package script inventory", path.display());
    r.coverage.push("Reads package manifests and script source as data; never invokes npm, build backends, or lifecycle hooks. Unknown/generated commands remain unresolved.".into());
    if path.extension().is_some_and(|e| e == "tgz" || e == "gz") {
        let bytes = read_file(path, 32 * MAX_INPUT)?;
        r.sha256 = Some(digest(&bytes));
        let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(bytes.as_slice()));
        let mut total = 0usize;
        let mut manifests = vec![];
        let mut gyps = vec![];
        for (i, entry) in archive.entries()?.enumerate() {
            if i >= 10000 {
                r.errors.push("Archive entry limit reached".into());
                break;
            }
            let mut e = entry?;
            if !e.header().entry_type().is_file() {
                continue;
            }
            let p = e.path()?.into_owned();
            if e.size() > MAX_INPUT {
                r.errors
                    .push(format!("Oversized archive member {}", p.display()));
                continue;
            }
            let mut b = vec![];
            e.by_ref().take(MAX_INPUT + 1).read_to_end(&mut b)?;
            total += b.len();
            if total > 32 * MAX_INPUT as usize {
                r.errors.push("Archive expansion limit reached".into());
                break;
            }
            if p.file_name().is_some_and(|n| n == "package.json") {
                manifest(&mut r, &p, &b)?;
                manifests.push((p, b));
            } else if p.file_name().is_some_and(|n| n == "binding.gyp") {
                gyps.push((p, b));
            } else if p
                .extension()
                .is_some_and(|e| ["js", "cjs", "mjs", "sh"].iter().any(|x| e == *x))
            {
                source_findings(&mut r, &p, &b);
            }
        }
        for (gyp, text) in &gyps {
            let dir = gyp.parent().unwrap_or(Path::new(""));
            if let Some((m, b)) = manifests.iter().find(|(m, _)| m.parent() == Some(dir)) {
                gyp_build(
                    &mut r,
                    m,
                    &serde_json::from_slice(b).unwrap_or_default(),
                    gyp,
                    text,
                );
            }
        }
    } else {
        let root = fs::canonicalize(path)?;
        let mut identity = vec![];
        let mut count = 0;
        let mut paths: Vec<PathBuf> = if root.is_file() {
            vec![root.clone()]
        } else {
            walkdir::WalkDir::new(&root)
                .follow_links(false)
                .max_depth(16)
                .into_iter()
                .filter_entry(|e| {
                    !matches!(e.file_name().to_str(), Some(".git" | "target" | ".venv"))
                })
                .take(50000)
                .filter_map(|e| match e {
                    Ok(e) if e.file_type().is_file() && e.file_name() == "package.json" => {
                        Some(e.into_path())
                    }
                    Err(e) => {
                        r.errors.push(e.to_string());
                        None
                    }
                    _ => None,
                })
                .collect()
        };
        paths.sort();
        for p in paths.iter().take(2000) {
            let b = read_file(p, MAX_INPUT)?;
            identity.extend_from_slice(p.as_os_str().as_encoded_bytes());
            identity.extend_from_slice(&b);
            manifest(&mut r, p, &b)?;
            count += 1;
            let value: serde_json::Value = serde_json::from_slice(&b)?;
            let gyp = p.with_file_name("binding.gyp");
            if let Ok(text) = read_file(&gyp, MAX_INPUT) {
                gyp_build(&mut r, p, &value, &gyp, &text);
            }
            if let Some(scripts) = value["scripts"].as_object() {
                for command in scripts.values().filter_map(|s| s.as_str()) {
                    for token in command.split_whitespace().filter(|t| {
                        t.ends_with(".js")
                            || t.ends_with(".cjs")
                            || t.ends_with(".mjs")
                            || t.ends_with(".sh")
                    }) {
                        let candidate = p.parent().unwrap().join(token.trim_matches(['\'', '"']));
                        if let Ok(c) = fs::canonicalize(candidate) {
                            if c.starts_with(p.parent().unwrap()) {
                                if let Ok(bytes) = read_file(&c, MAX_INPUT) {
                                    identity.extend_from_slice(&bytes);
                                    source_findings(&mut r, &c, &bytes);
                                }
                            }
                        }
                    }
                }
            }
        }
        if paths.len() > 2000 {
            r.errors.push("Package manifest limit reached".into());
        }
        if count == 0 {
            r.errors.push("No package.json files found. Provide a project, installed dependency tree, or npm .tgz archive.".into());
        }
        if root.is_dir() {
            let lock = root.join("package-lock.json");
            if lock.exists() {
                let v: serde_json::Value =
                    serde_json::from_slice(&read_file(&lock, 32 * MAX_INPUT)?)?;
                if let Some(pkgs) = v["packages"].as_object() {
                    for (name, meta) in pkgs {
                        if meta["hasInstallScript"] == true
                            && !root.join(name).join("package.json").is_file()
                        {
                            r.add(
                                "package.unresolved-hook",
                                Severity::Review,
                                "Lockfile declares an unavailable installation script",
                                format!("{name} {}", meta["version"]),
                                "Inspect the package archive without running lifecycle scripts.",
                            );
                        }
                    }
                }
            }
            r.coverage.push(format!(
                "{count} manifests read. Dependencies absent from disk are not fully inspected."
            ));
        }
        r.sha256 = Some(digest(&identity));
    }
    Ok(r)
}
pub(crate) fn manifest(r: &mut Report, p: &Path, b: &[u8]) -> Result<()> {
    let v: serde_json::Value =
        serde_json::from_slice(b).with_context(|| format!("Invalid manifest {}", p.display()))?;
    if let Some(scripts) = v["scripts"].as_object() {
        for (name, command) in scripts {
            let lifecycle = [
                "preinstall",
                "install",
                "postinstall",
                "prepare",
                "prepublish",
                "prepack",
            ]
            .contains(&name.as_str());
            r.add(if lifecycle{"package.lifecycle"}else{"package.script"},if lifecycle{Severity::Review}else{Severity::Info},&format!("{}@{}: {name}",v["name"].as_str().unwrap_or("unnamed"),v["version"].as_str().unwrap_or("unknown")),format!("{}\n{}",p.display(),command.as_str().unwrap_or("invalid script")),"Review referenced source. Installation approvals do not constrain later runtime behavior.");
            if let Some(s) = command.as_str() {
                source_findings(r, p, s.as_bytes());
            }
        }
    }
    Ok(())
}
/// npm runs `node-gyp rebuild` at install for a package with a binding.gyp and
/// no install or preinstall script of its own, so the hook is not in `scripts`.
fn gyp_build(r: &mut Report, manifest: &Path, value: &serde_json::Value, gyp: &Path, text: &[u8]) {
    let scripts = &value["scripts"];
    if scripts.get("install").is_some() || scripts.get("preinstall").is_some() {
        return;
    }
    let text = String::from_utf8_lossy(text);
    let mut evidence = format!(
        "{}\nnpm runs node-gyp rebuild when it installs this package",
        gyp.display()
    );
    // <!( and <!@( run a shell command while gyp reads the file.
    for (n, line) in text
        .lines()
        .enumerate()
        .filter(|(_, l)| l.contains("<!(") || l.contains("<!@("))
        .take(5)
    {
        evidence.push_str(&format!("\nline {}: {}", n + 1, line.trim()));
    }
    r.add(
        "package.gyp-build",
        Severity::Review,
        &format!(
            "{}@{}: builds native code at install",
            value["name"].as_str().unwrap_or("unnamed"),
            value["version"].as_str().unwrap_or("unknown")
        ),
        format!("{}\n{evidence}", manifest.display()),
        "The build runs a compiler and any command binding.gyp names, with your permissions, during npm install. Read binding.gyp, or install with --ignore-scripts.",
    );
}

fn source_findings(r: &mut Report, p: &Path, b: &[u8]) {
    let scan = shell::analyze_content(&String::from_utf8_lossy(b));
    for error in scan.errors {
        r.errors.push(format!("{}: {error}", p.display()));
    }
    for mut f in scan.findings {
        if r.findings.len() >= 10000 {
            let limit = "Package finding limit reached; additional findings omitted".to_owned();
            if !r.errors.contains(&limit) {
                r.errors.push(limit);
            }
            break;
        }
        f.evidence = format!("{}: {}", p.display(), f.evidence);
        r.findings.push(f);
    }
}

pub fn policy(path: &Path, apply: bool) -> Result<String> {
    let root = fs::canonicalize(path)?;
    if !root.is_dir() {
        bail!("Select a project directory");
    }
    let p = root.join(".npmrc");
    let old = if p.exists() {
        String::from_utf8(read_file(&p, MAX_INPUT)?)?
    } else {
        String::new()
    };
    let new = format!(
        "{}{}# Bigman: disable automatic lifecycle execution for npm/pnpm\nignore-scripts=true\n",
        old,
        if old.ends_with('\n') || old.is_empty() {
            ""
        } else {
            "\n"
        }
    );
    if apply {
        if p.exists() {
            let backup = root.join(format!(".npmrc.bigman-{}.bak", uuid::Uuid::new_v4()));
            fs::copy(&p, &backup)?;
        }
        let mut tmp = tempfile::NamedTempFile::new_in(&root)?;
        use std::io::Write;
        tmp.write_all(new.as_bytes())?;
        tmp.persist(&p).map_err(|e| e.error)?;
    }
    Ok(format!("{} {}\nSets ignore-scripts=true for npm/pnpm; command-line overrides and explicit npm run commands can still execute code.{}",if apply{"Updated"}else{"Would update"},p.display(),if apply{" Existing configuration backed up when present."}else{" Use Apply to write this project-local setting."}))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inventories_without_executing() {
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        let project = d.path().join("project");
        fs::create_dir(&project).unwrap();
        fs::write(
            project.join("package.json"),
            r#"{"name":"fixture","version":"1","scripts":{"postinstall":"touch MUST_NOT_EXIST"}}"#,
        )
        .unwrap();
        let r = packages(&store, &project).unwrap();
        assert!(r.findings.iter().any(|f| f.rule == "package.lifecycle"));
        assert!(!project.join("MUST_NOT_EXIST").exists());
    }
}
