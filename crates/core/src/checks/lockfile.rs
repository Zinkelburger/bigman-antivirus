//! Lockfile linter: what a lockfile pins, and with diff mode what a change adds.
//! npm (package-lock.json v2/v3), Yarn v1 (yarn.lock), pnpm (pnpm-lock.yaml),
//! Bun (bun.lock) and Cargo (Cargo.lock), read as text; nothing is installed.
use crate::{
    model::{Report, Severity},
    store::{read_file, MAX_INPUT},
};
use anyhow::{bail, Context, Result};
use regex::Regex;
use std::{collections::BTreeMap, path::Path};

/// One locked package, in a shape every format fits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locked {
    pub name: String,
    pub version: String,
    /// Where the package manager downloads it from.
    pub resolved: Option<String>,
    pub integrity: Option<String>,
    /// The lockfile records that it runs an install script.
    pub install_script: bool,
}

/// Registries whose packages need no remark; anything else is named.
const REGISTRIES: [&str; 5] = [
    "https://registry.npmjs.org/",
    "https://registry.yarnpkg.com/",
    "https://registry.npmmirror.com/",
    "registry+https://github.com/rust-lang/crates.io-index",
    "sparse+https://index.crates.io/",
];

/// A source that is not a package registry: git, a plain URL, a local path.
fn non_registry(resolved: &str) -> bool {
    !REGISTRIES.iter().any(|r| resolved.starts_with(r))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Npm,
    Yarn,
    Pnpm,
    Bun,
    Cargo,
}

pub fn format_of(path: &Path) -> Option<Format> {
    match path.file_name()?.to_str()? {
        "package-lock.json" | "npm-shrinkwrap.json" => Some(Format::Npm),
        "yarn.lock" => Some(Format::Yarn),
        "pnpm-lock.yaml" => Some(Format::Pnpm),
        "bun.lock" => Some(Format::Bun),
        "Cargo.lock" => Some(Format::Cargo),
        _ => None,
    }
}

fn npm(text: &str) -> Result<Vec<Locked>> {
    let v: serde_json::Value =
        serde_json::from_str(text).context("package-lock.json is not JSON")?;
    let packages = v.get("packages").and_then(|p| p.as_object()).context(
        "Only lockfileVersion 2 and 3 are read; regenerate older lockfiles with a current npm",
    )?;
    Ok(packages
        .iter()
        .filter(|(path, entry)| {
            !path.is_empty() && entry.get("link").and_then(|l| l.as_bool()) != Some(true)
        })
        .map(|(path, entry)| {
            let s = |k: &str| entry.get(k).and_then(|x| x.as_str()).map(str::to_string);
            Locked {
                name: s("name").unwrap_or_else(|| {
                    path.rsplit("node_modules/")
                        .next()
                        .unwrap_or(path)
                        .to_string()
                }),
                version: s("version").unwrap_or_default(),
                resolved: s("resolved"),
                integrity: s("integrity"),
                install_script: entry.get("hasInstallScript").and_then(|x| x.as_bool())
                    == Some(true),
            }
        })
        .collect())
}

/// Yarn v1: `"name@^1", name@~1:` headers, then indented `version`, `resolved`, `integrity`.
fn yarn(text: &str) -> Vec<Locked> {
    let mut out = vec![];
    let mut current: Option<Locked> = None;
    for line in text.lines() {
        if !line.starts_with(' ') && line.ends_with(':') && !line.starts_with('#') {
            out.extend(current.take());
            let spec = line
                .trim_end_matches(':')
                .split(", ")
                .next()
                .unwrap_or("")
                .trim_matches('"');
            let name = match spec.rfind('@') {
                Some(0) | None => spec.to_string(),
                Some(i) => spec[..i].to_string(),
            };
            current = Some(Locked {
                name,
                version: String::new(),
                resolved: None,
                integrity: None,
                install_script: false,
            });
        } else if let Some(c) = current.as_mut() {
            let t = line.trim();
            let value = |k: &str| {
                t.strip_prefix(k)
                    .map(|v| v.trim().trim_matches('"').to_string())
            };
            if let Some(v) = value("version ") {
                c.version = v;
            } else if let Some(v) = value("resolved ") {
                c.resolved = Some(v);
            } else if let Some(v) = value("integrity ") {
                c.integrity = Some(v);
            }
        }
    }
    out.extend(current);
    out
}

/// pnpm: `  /name@1.0.0:` or `  name@1.0.0:` under `packages:`, with a
/// `resolution: {integrity: …}` or `{tarball: …}` and `requiresBuild: true`.
fn pnpm(text: &str) -> Vec<Locked> {
    let header = Regex::new(r"^  '?/?(@?[^@\s']+)@([^:'\s(]+)[^:]*'?:$").expect("valid regex");
    let mut out = vec![];
    let mut in_packages = false;
    let mut current: Option<Locked> = None;
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if !line.starts_with(' ') {
            out.extend(current.take());
            in_packages = line == "packages:";
            continue;
        }
        if !in_packages {
            continue;
        }
        if let Some(c) = header.captures(line) {
            out.extend(current.take());
            current = Some(Locked {
                name: c[1].into(),
                version: c[2].into(),
                resolved: None,
                integrity: None,
                install_script: false,
            });
        } else if let Some(c) = current.as_mut() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("resolution: {") {
                for part in rest.trim_end_matches('}').split(", ") {
                    match part.split_once(": ") {
                        Some(("integrity", v)) => c.integrity = Some(v.into()),
                        Some(("tarball", v)) => c.resolved = Some(v.into()),
                        Some(("repo", v)) => c.resolved = Some(format!("git+{v}")),
                        _ => {}
                    }
                }
            } else if t == "requiresBuild: true" {
                c.install_script = true;
            }
        }
    }
    out.extend(current);
    out
}

/// Bun: JSONC with `"packages": { "name": ["name@1.0.0", "url-or-empty", {…}, "sha512-…"] }`.
fn bun(text: &str) -> Result<Vec<Locked>> {
    let v: serde_json::Value =
        serde_json::from_str(&super::repo::strip_jsonc(text)).context("bun.lock is not JSON")?;
    let packages = v
        .get("packages")
        .and_then(|p| p.as_object())
        .context("bun.lock has no packages")?;
    Ok(packages
        .values()
        .filter_map(|entry| {
            let a = entry.as_array()?;
            let spec = a.first()?.as_str()?;
            let at = spec.rfind('@').filter(|&i| i > 0)?;
            let resolved = a
                .get(1)
                .and_then(|x| x.as_str())
                .filter(|r| !r.is_empty())
                .map(str::to_string);
            Some(Locked {
                name: spec[..at].into(),
                version: spec[at + 1..].into(),
                resolved,
                integrity: a
                    .iter()
                    .filter_map(|x| x.as_str())
                    .find(|s| s.starts_with("sha"))
                    .map(str::to_string),
                install_script: false,
            })
        })
        .collect())
}

/// Cargo: `[[package]]` tables with `name`, `version`, `source`, `checksum`.
fn cargo(text: &str) -> Vec<Locked> {
    let mut out = vec![];
    let mut current: Option<Locked> = None;
    for line in text.lines() {
        if line.trim() == "[[package]]" {
            out.extend(current.take());
            current = Some(Locked {
                name: String::new(),
                version: String::new(),
                resolved: None,
                integrity: None,
                install_script: false,
            });
        } else if let Some(c) = current.as_mut() {
            let Some((k, v)) = line.split_once(" = ") else {
                continue;
            };
            let v = v.trim().trim_matches('"').to_string();
            match k.trim() {
                "name" => c.name = v,
                "version" => c.version = v,
                "source" => c.resolved = Some(v),
                "checksum" => c.integrity = Some(v),
                _ => {}
            }
        }
    }
    out.extend(current);
    // Workspace members have no source; they are the project itself.
    out.retain(|p| p.resolved.is_some());
    out
}

pub fn parse(format: Format, text: &str) -> Result<Vec<Locked>> {
    Ok(match format {
        Format::Npm => npm(text)?,
        Format::Yarn => yarn(text),
        Format::Pnpm => pnpm(text),
        Format::Bun => bun(text)?,
        Format::Cargo => cargo(text),
    })
}

fn key(p: &Locked) -> String {
    format!("{}@{}", p.name, p.version)
}

/// The rules over one lockfile, or with `before`, over what changed since it.
pub fn assess(r: &mut Report, file: &str, now: &[Locked], before: Option<&[Locked]>) {
    let old: BTreeMap<String, &Locked> =
        before.unwrap_or(&[]).iter().map(|p| (key(p), p)).collect();
    let changed = |p: &Locked| before.is_none() || !old.contains_key(&key(p));
    for p in now.iter().filter(|p| changed(p)) {
        let at = format!(
            "{file}: {}@{}{}",
            p.name,
            p.version,
            p.resolved
                .as_ref()
                .map(|r| format!("\nresolved {r}"))
                .unwrap_or_default()
        );
        if let Some(resolved) = p.resolved.as_deref().filter(|r| non_registry(r)) {
            r.add(
                "lockfile.non-registry",
                Severity::Review,
                &format!("{} comes from outside the package registry", p.name),
                &at,
                &format!(
                    "Dependencies from git, a plain URL or a local path skip the registry's checks and can change without a version bump. Make sure {} is meant to come from {}.",
                    p.name,
                    resolved.split(['#', '?']).next().unwrap_or(resolved)
                ),
            );
        }
        let registry = p.resolved.as_deref().is_some_and(|r| !non_registry(r));
        if registry && p.integrity.is_none() {
            r.add(
                "lockfile.missing-integrity",
                Severity::Review,
                &format!("{} has no integrity hash", p.name),
                &at,
                "Without a hash the package manager cannot tell if the download changed. Regenerate the lockfile with a current package manager.",
            );
        }
        if p.install_script {
            let new = before.is_some();
            r.add(
                if new { "lockfile.new-install-hook" } else { "lockfile.install-hook" },
                if new { Severity::Review } else { Severity::Info },
                &if new {
                    format!("New dependency {} runs an install script", p.name)
                } else {
                    format!("{} runs an install script", p.name)
                },
                &at,
                "Install scripts run with your permissions during npm install. Read the package's scripts, or install with --ignore-scripts.",
            );
        }
    }
}

/// Lints `path`, or with `against` reports what changed since that older copy.
pub fn lockfile(path: &Path, against: Option<&Path>) -> Result<Report> {
    let format = format_of(path).with_context(|| format!("{} is not a lockfile Bigman reads (package-lock.json, yarn.lock, pnpm-lock.yaml, bun.lock, Cargo.lock)", path.display()))?;
    let read = |p: &Path| -> Result<Vec<Locked>> {
        parse(
            format,
            &String::from_utf8_lossy(&read_file(p, 4 * MAX_INPUT)?),
        )
    };
    let now = read(path)?;
    let before = match against {
        Some(p) if format_of(p).is_some_and(|f| f != format) => {
            bail!("Compare lockfiles of the same kind")
        }
        Some(p) => Some(read(p)?),
        None => None,
    };
    let mut r = Report::new("Lockfile check", path.display());
    r.coverage
        .checked
        .push(format!("{} locked packages", now.len()));
    r.coverage.push(if before.is_some() {
        "Only packages that are new or changed since the older lockfile are checked.".into()
    } else {
        "Reads the lockfile as text; nothing is installed. Install hooks are listed where the lockfile records them (npm and pnpm).".into()
    });
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    assess(&mut r, &name, &now, before.as_deref());
    r.findings.sort_by_key(|f| std::cmp::Reverse(f.severity));
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_each_format() {
        let yarn = yarn("# yarn lockfile v1\n\n\"@scope/a@^1.0.0\", \"@scope/a@^1.1.0\":\n  version \"1.2.0\"\n  resolved \"https://registry.yarnpkg.com/@scope/a/-/a-1.2.0.tgz#abc\"\n  integrity sha512-xyz\n");
        assert_eq!(
            (
                yarn[0].name.as_str(),
                yarn[0].version.as_str(),
                yarn[0].integrity.as_deref()
            ),
            ("@scope/a", "1.2.0", Some("sha512-xyz"))
        );
        let pnpm = pnpm("lockfileVersion: '9.0'\n\npackages:\n\n  esbuild@0.21.5:\n    resolution: {integrity: sha512-abc}\n    requiresBuild: true\n\n  /left-pad@1.3.0:\n    resolution: {tarball: https://example.org/left-pad.tgz}\n");
        assert_eq!(pnpm.len(), 2);
        assert!(pnpm[0].install_script);
        assert_eq!(
            pnpm[1].resolved.as_deref(),
            Some("https://example.org/left-pad.tgz")
        );
        let cargo = cargo("[[package]]\nname = \"bigman\"\nversion = \"0.2.0\"\n\n[[package]]\nname = \"serde\"\nversion = \"1.0.0\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\nchecksum = \"abc\"\n");
        assert_eq!(cargo.len(), 1, "workspace members are the project itself");
    }
}
