//! Lockfile linter: what a lockfile pins, and with diff mode what a change adds.
//! npm (package-lock.json v2/v3), Yarn v1 (yarn.lock), pnpm (pnpm-lock.yaml),
//! Bun (bun.lock) and Cargo (Cargo.lock), read as text; nothing is installed.
use crate::{
    model::{Report, Severity},
    store::{read_file, MAX_INPUT},
};
use anyhow::{bail, Context, Result};
use regex::Regex;
use std::{collections::BTreeMap, path::Path, sync::LazyLock};

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
const REGISTRIES: [&str; 6] = [
    "https://registry.npmjs.org/",
    "https://registry.yarnpkg.com/",
    "https://registry.npmmirror.com/",
    "registry+https://github.com/rust-lang/crates.io-index",
    "sparse+https://index.crates.io/",
    // Yarn Berry writes registry packages as `name@npm:1.2.3`.
    "npm:",
];

/// A source that is not a package registry: git, a plain URL, a local path.
fn non_registry(resolved: &str) -> bool {
    !REGISTRIES.iter().any(|r| resolved.starts_with(r))
}

/// The part of a registry address after the registry itself, so a move between
/// npmjs.org and its yarnpkg.com mirror is not a change of source.
fn registry_path(resolved: &str) -> &str {
    REGISTRIES
        .iter()
        .find_map(|r| resolved.strip_prefix(r))
        .unwrap_or(resolved)
}

/// Splits `name@spec` at the `@` after the name; a scope's leading `@` is part of the name.
fn split_spec(spec: &str) -> Option<(&str, &str)> {
    let at = spec.get(1..)?.find('@')? + 1;
    Some((&spec[..at], &spec[at + 1..]))
}

/// SRI hashes (`sha512-…`) as opposed to other strings that happen to start with "sha".
static SRI: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^sha(1|256|384|512)-[A-Za-z0-9+/=]+$").expect("valid regex"));

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

/// Yarn: `"name@^1", name@~1:` headers, then indented fields. v1 writes
/// `version "1.2.0"`, `resolved "url"` and `integrity sha512-…`; Berry (v2+,
/// marked by `__metadata:`) writes `version: 1.2.0`, `resolution: "name@npm:1.2.0"`
/// and `checksum: …`.
fn yarn(text: &str) -> Vec<Locked> {
    let berry = text.lines().any(|l| l == "__metadata:");
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
            if spec == "__metadata" {
                continue;
            }
            current = Some(Locked {
                name: split_spec(spec).map_or(spec, |(n, _)| n).to_string(),
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
            if berry {
                if let Some(v) = value("version:") {
                    c.version = v;
                } else if let Some(v) = value("resolution:") {
                    c.resolved = Some(berry_source(&v));
                } else if let Some(v) = value("checksum:") {
                    c.integrity = Some(v);
                }
            } else if let Some(v) = value("version ") {
                c.version = v;
            } else if let Some(v) = value("resolved ") {
                c.resolved = Some(v);
            } else if let Some(v) = value("integrity ") {
                c.integrity = Some(v);
            }
        }
    }
    out.extend(current);
    // Workspace packages are the project itself.
    out.retain(|p| {
        !p.resolved
            .as_deref()
            .is_some_and(|r| r.starts_with("workspace:"))
    });
    out
}

/// Where a Berry `resolution` downloads from: `npm:1.2.0`, `https://…`,
/// `git+ssh://…`, `file:…`. A `patch:` wraps the package it patches, whose
/// source is what counts; the patch itself is a file in the project.
fn berry_source(resolution: &str) -> String {
    let spec = split_spec(resolution).map_or(resolution, |(_, s)| s);
    match spec.strip_prefix("patch:") {
        Some(inner) => {
            let inner = inner
                .split('#')
                .next()
                .unwrap_or(inner)
                .replace("%3A", ":")
                .replace("%40", "@");
            split_spec(&inner).map_or(inner.clone(), |(_, s)| s.to_string())
        }
        None => spec.to_string(),
    }
}

/// pnpm: package keys under `packages:`. v5 writes `/name/1.0.0_peer:`, v6
/// `/name@1.0.0(peer):` and v9 `name@1.0.0:`; non-registry packages may be keyed
/// by their source (`github.com/o/r/sha:`) and carry `name:` and `version:`
/// fields. Each has a `resolution: {integrity: …}`, `{tarball: …}`,
/// `{repo: …}` or `{directory: …}`, and `requiresBuild: true` for install scripts.
fn pnpm(text: &str) -> Vec<Locked> {
    let v5 = text
        .lines()
        .find_map(|l| l.strip_prefix("lockfileVersion:"))
        .is_some_and(|v| v.trim().trim_matches('\'').starts_with('5'));
    let mut out = vec![];
    let mut in_packages = false;
    let mut current: Option<Locked> = None;
    let indent = |l: &str| l.len() - l.trim_start_matches(' ').len();
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
        let t = line.trim();
        if indent(line) == 2 && t.ends_with(':') {
            out.extend(current.take());
            let key = t.trim_end_matches(':').trim_matches(['\'', '"']);
            let (name, version) = pnpm_key(key, v5);
            current = Some(Locked {
                name: name.into(),
                version: version.into(),
                resolved: None,
                integrity: None,
                install_script: false,
            });
        } else if let Some(c) = current.as_mut().filter(|_| indent(line) == 4) {
            if let Some(rest) = t.strip_prefix("resolution: {") {
                for part in rest.trim_end_matches('}').split(", ") {
                    match part.split_once(": ") {
                        Some(("integrity", v)) => c.integrity = Some(v.into()),
                        Some(("tarball", v)) => c.resolved = Some(v.into()),
                        Some(("repo", v)) => c.resolved = Some(format!("git+{v}")),
                        Some(("directory", v)) => c.resolved = Some(format!("file:{v}")),
                        _ => {}
                    }
                }
            } else if let Some(v) = t.strip_prefix("name: ") {
                c.name = v.trim_matches(['\'', '"']).into();
            } else if let Some(v) = t.strip_prefix("version: ") {
                c.version = v.trim_matches(['\'', '"']).into();
            } else if t == "requiresBuild: true" {
                c.install_script = true;
            }
        }
    }
    out.extend(current);
    out
}

/// Name and version from a pnpm package key, leaving both empty when the key is
/// a source address (the entry's `name:` and `version:` fields fill them in).
fn pnpm_key(key: &str, v5: bool) -> (&str, &str) {
    let key = key.split('(').next().unwrap_or(key);
    if v5 {
        // `/name/1.0.0_peer@2.0.0` or `/@scope/name/1.0.0`.
        return match key.strip_prefix('/').and_then(|k| k.rsplit_once('/')) {
            Some((name, version)) => (name, version.split('_').next().unwrap_or(version)),
            None => ("", ""),
        };
    }
    split_spec(key.strip_prefix('/').unwrap_or(key)).unwrap_or(("", ""))
}

/// Bun: JSONC with `"packages": { "key": [spec, …] }`. Registry packages are
/// `["name@1.0.0", "registry-or-empty", {…}, "sha512-…"]`; git, GitHub, tarball
/// and path packages put the source in the spec (`name@github:o/r#sha`).
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
            let (name, spec) = split_spec(a.first()?.as_str()?)?;
            if spec.starts_with("workspace:") {
                return None;
            }
            // A spec with a protocol is its own source; a plain version is
            // from the registry named in the second element, or the default.
            let resolved = if spec.contains(':') {
                Some(spec.to_string())
            } else {
                a.get(1)
                    .and_then(|x| x.as_str())
                    .filter(|r| !r.is_empty())
                    .map(str::to_string)
            };
            Some(Locked {
                name: name.into(),
                version: spec.into(),
                resolved,
                integrity: a
                    .iter()
                    .filter_map(|x| x.as_str())
                    .find(|s| SRI.is_match(s))
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

/// Why a lockfile Bigman can open would still be misread, so the report says
/// Incomplete instead of passing a file it did not understand.
pub fn unsupported(format: Format, text: &str, parsed: &[Locked]) -> Option<String> {
    if format == Format::Pnpm {
        let version = text
            .lines()
            .find_map(|l| l.strip_prefix("lockfileVersion:"))
            .map(|v| v.trim().trim_matches(['\'', '"']).to_string())
            .unwrap_or_default();
        if !["5", "6", "9"].iter().any(|v| version.starts_with(v)) {
            return Some(format!(
                "pnpm lockfileVersion {version:?} is not one Bigman reads (5, 6 and 9 are)"
            ));
        }
    }
    let unnamed = parsed
        .iter()
        .filter(|p| p.name.is_empty() || p.version.is_empty())
        .count();
    // Yarn headers at the left margin, without Berry's metadata and workspaces.
    let yarn_entries = || {
        text.lines()
            .filter(|l| !l.starts_with([' ', '#']) && l.ends_with(':'))
            .filter(|l| *l != "__metadata:" && !l.contains("@workspace:"))
            .count()
    };
    match format {
        Format::Yarn if parsed.is_empty() && yarn_entries() > 0 => Some(format!(
            "None of its {} entries were recognised; the file may use a newer format than Bigman reads",
            yarn_entries()
        )),
        Format::Yarn | Format::Pnpm if unnamed > 0 => Some(format!(
            "{unnamed} of {} entries had no recognisable name and version",
            parsed.len()
        )),
        _ => None,
    }
}

/// A change of download source or hash at the same version, ignoring a move
/// between registry mirrors and an upgrade of the hash algorithm.
fn source_changed(old: &Locked, new: &Locked) -> bool {
    let place = |p: &Locked| p.resolved.as_deref().map(registry_path).map(str::to_string);
    let algorithm = |i: &str| i.split(['-', '/']).next().unwrap_or("").to_string();
    let hash_changed = match (&old.integrity, &new.integrity) {
        (Some(a), Some(b)) => a != b && algorithm(a) == algorithm(b),
        _ => false,
    };
    place(old) != place(new) || hash_changed
}

/// The rules over one lockfile, or with `before`, over what changed since it.
pub fn assess(r: &mut Report, file: &str, now: &[Locked], before: Option<&[Locked]>) {
    let mut old: BTreeMap<&str, Vec<&Locked>> = BTreeMap::new();
    for p in before.unwrap_or(&[]) {
        old.entry(p.name.as_str()).or_default().push(p);
    }
    for p in now {
        let prior = old.get(p.name.as_str()).map(Vec::as_slice).unwrap_or(&[]);
        if before.is_some() && prior.contains(&p) {
            continue;
        }
        let at = format!(
            "{file}: {}@{}{}",
            p.name,
            p.version,
            p.resolved
                .as_ref()
                .map(|r| format!("\nresolved {r}"))
                .unwrap_or_default()
        );
        if let Some(o) = prior
            .iter()
            .find(|o| o.version == p.version && source_changed(o, p))
        {
            let show = |l: &Locked| {
                format!(
                    "resolved {}\nintegrity {}",
                    l.resolved.as_deref().unwrap_or("(none)"),
                    l.integrity.as_deref().unwrap_or("(none)")
                )
            };
            r.add(
                "lockfile.source-changed",
                Severity::High,
                &format!("{}@{} changed where it downloads from without a new version", p.name, p.version),
                format!("{file}: {}@{}\nbefore:\n{}\nnow:\n{}", p.name, p.version, show(o), show(p)),
                "A published version never changes, so a new address or hash for the same version means the lockfile was edited by hand or by a tool. Find out who changed it before installing.",
            );
        }
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
            let title = match (
                before.is_some(),
                prior.is_empty(),
                prior.iter().any(|o| o.install_script),
            ) {
                (false, _, _) => format!("{} runs an install script", p.name),
                (true, true, _) => format!("New dependency {} runs an install script", p.name),
                (true, false, false) => format!("{} now runs an install script", p.name),
                (true, false, true) => format!("{} changed and runs an install script", p.name),
            };
            let new = before.is_some();
            r.add(
                if new { "lockfile.new-install-hook" } else { "lockfile.install-hook" },
                if new { Severity::Review } else { Severity::Info },
                &title,
                &at,
                "Install scripts run with your permissions during npm install. Read the package's scripts, or install with --ignore-scripts.",
            );
        }
    }
}

/// Lints `path`, or with `against` reports what changed since that older copy.
pub fn lockfile(path: &Path, against: Option<&Path>) -> Result<Report> {
    let format = format_of(path).with_context(|| format!("{} is not a lockfile Bigman reads (package-lock.json, yarn.lock, pnpm-lock.yaml, bun.lock, Cargo.lock)", path.display()))?;
    let read = |p: &Path| -> Result<(Vec<Locked>, Option<String>)> {
        let text = String::from_utf8_lossy(&read_file(p, 4 * MAX_INPUT)?).into_owned();
        let parsed = parse(format, &text)?;
        let problem = unsupported(format, &text, &parsed);
        Ok((parsed, problem))
    };
    let (now, problem) = read(path)?;
    let (before, before_problem) = match against {
        Some(p) if format_of(p).is_some_and(|f| f != format) => {
            bail!("Compare lockfiles of the same kind")
        }
        Some(p) => {
            let (b, problem) = read(p)?;
            (Some(b), problem)
        }
        None => (None, None),
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
    for (which, problem) in [(path, problem), (against.unwrap_or(path), before_problem)] {
        if let Some(detail) = problem {
            r.coverage.skip(
                format!("Part of {}", which.display()),
                crate::model::SkipReason::Unreadable,
                detail,
            );
        }
    }
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

    fn pkg(name: &str, version: &str, resolved: &str, integrity: &str) -> Locked {
        Locked {
            name: name.into(),
            version: version.into(),
            resolved: Some(resolved.into()),
            integrity: (!integrity.is_empty()).then(|| integrity.into()),
            install_script: false,
        }
    }

    fn rules(now: &[Locked], before: &[Locked]) -> Vec<String> {
        let mut r = Report::new("Lockfile check", "test");
        assess(&mut r, "package-lock.json", now, Some(before));
        r.findings.into_iter().map(|f| f.rule).collect()
    }

    #[test]
    fn diff_sees_a_new_source_at_the_same_version() {
        let old = pkg(
            "lodash",
            "4.17.21",
            "https://registry.npmjs.org/lodash/-/lodash-4.17.21.tgz",
            "sha512-A",
        );
        let swapped = pkg("lodash", "4.17.21", "https://evil.example/lodash.tgz", "");
        assert_eq!(
            rules(std::slice::from_ref(&swapped), std::slice::from_ref(&old)),
            ["lockfile.source-changed", "lockfile.non-registry"]
        );
        assert!(
            rules(std::slice::from_ref(&old), std::slice::from_ref(&old)).is_empty(),
            "unchanged entries are skipped"
        );
        // A move to the yarnpkg.com mirror and an upgrade of the hash algorithm are not changes of source.
        let mirror = pkg(
            "lodash",
            "4.17.21",
            "https://registry.yarnpkg.com/lodash/-/lodash-4.17.21.tgz",
            "sha1-B",
        );
        assert!(rules(&[mirror], &[old]).is_empty());
    }

    #[test]
    fn diff_names_an_install_script_on_an_existing_package() {
        let old = pkg(
            "esbuild",
            "0.21.0",
            "https://registry.npmjs.org/esbuild/-/esbuild-0.21.0.tgz",
            "sha512-A",
        );
        let mut new = old.clone();
        new.install_script = true;
        let mut r = Report::new("Lockfile check", "test");
        assess(&mut r, "f", &[new], Some(&[old]));
        assert_eq!(r.findings[0].summary, "esbuild now runs an install script");
    }

    #[test]
    fn reads_yarn_berry() {
        let text = "__metadata:\n  version: 8\n  cacheKey: 10c0\n\n\"app@workspace:.\":\n  version: 0.0.0-use.local\n  resolution: \"app@workspace:.\"\n\n\"lodash@npm:^4.17.21\":\n  version: 4.17.21\n  resolution: \"lodash@npm:4.17.21\"\n  checksum: 10c0/abc\n\n\"x@git+ssh://git@github.com/o/x.git\":\n  version: 1.0.0\n  resolution: \"x@git+ssh://git@github.com/o/x.git#commit=abc\"\n\n\"left-pad@patch:left-pad@npm%3A1.3.0#~/.yarn/patches/l.patch\":\n  version: 1.3.0\n  resolution: \"left-pad@patch:left-pad@npm%3A1.3.0#~/.yarn/patches/l.patch::version=1.3.0\"\n  checksum: 10c0/def\n";
        let got = yarn(text);
        assert_eq!(got.len(), 3, "the workspace is the project itself");
        assert_eq!(
            (
                got[0].name.as_str(),
                got[0].version.as_str(),
                got[0].resolved.as_deref()
            ),
            ("lodash", "4.17.21", Some("npm:4.17.21"))
        );
        assert!(!non_registry(got[0].resolved.as_deref().unwrap()));
        assert!(non_registry(got[1].resolved.as_deref().unwrap()));
        assert_eq!(
            got[2].resolved.as_deref(),
            Some("npm:1.3.0"),
            "a patch is judged by what it patches"
        );
        assert_eq!(unsupported(Format::Yarn, text, &got), None);
    }

    #[test]
    fn reads_pnpm_v5_and_source_keyed_entries() {
        let text = "lockfileVersion: 5.4\n\npackages:\n\n  /@scope/a/1.2.0_react@18.0.0:\n    resolution: {integrity: sha512-abc}\n    dependencies:\n      name: 1.0.0\n\n  github.com/o/r/abc123:\n    resolution: {tarball: https://codeload.github.com/o/r/tar.gz/abc123}\n    name: r\n    version: 2.0.0\n";
        let got = pnpm(text);
        assert_eq!(
            (got[0].name.as_str(), got[0].version.as_str()),
            ("@scope/a", "1.2.0")
        );
        assert_eq!(
            (got[1].name.as_str(), got[1].version.as_str()),
            ("r", "2.0.0")
        );
        assert_eq!(unsupported(Format::Pnpm, text, &got), None);
        let v9 = pnpm("lockfileVersion: '9.0'\n\npackages:\n\n  '@scope/b@1.0.0(peer@2.0.0)':\n    resolution: {integrity: sha512-abc}\n");
        assert_eq!(
            (v9[0].name.as_str(), v9[0].version.as_str()),
            ("@scope/b", "1.0.0")
        );
        assert!(unsupported(Format::Pnpm, "lockfileVersion: '10.0'\n", &[]).is_some());
    }

    #[test]
    fn reads_bun_sources_from_the_spec() {
        let got = bun(r#"{"lockfileVersion": 1, "packages": {
            "app": ["app@workspace:."],
            "a": ["a@1.0.0", "", {}, "sha512-abc"],
            "x": ["x@github:attacker/x#abc123", {}, "shadcn-x-abc123"]
        }}"#)
        .unwrap();
        assert_eq!(got.len(), 2);
        let x = got.iter().find(|p| p.name == "x").unwrap();
        assert_eq!(x.resolved.as_deref(), Some("github:attacker/x#abc123"));
        assert_eq!(
            x.integrity, None,
            "an owner name starting with sha is not a hash"
        );
    }

    #[test]
    fn unreadable_yarn_is_incomplete() {
        let text = "pkg@^1:\n  something: else\n";
        assert!(unsupported(Format::Yarn, text, &yarn(text)).is_some());
    }
}
