//! `repo-check DIR`: what a cloned project could run on your machine before you
//! run anything yourself. IDE tasks that start when a folder opens, direnv,
//! devcontainer commands, git hooks and filters, AI coding-tool configuration,
//! instruction files with hidden characters, and risky CI workflows. Reads files
//! as data; never runs, sources or trusts anything in the tree.
use crate::{
    model::{Report, Severity},
    store::{read_file, MAX_INPUT},
};
use anyhow::{bail, Result};
use regex::Regex;
use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

/// Directories that hold dependencies or build output, not the project's own files.
const SKIP: [&str; 6] = ["node_modules", ".git", "target", "vendor", "dist", ".venv"];
const MAX_FILES: usize = 20_000;

/// Text that hides from a reader: zero-width, bidirectional overrides, tag characters.
static HIDDEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[\u{200B}-\u{200F}\u{202A}-\u{202E}\u{2060}-\u{2064}\u{2066}-\u{2069}\u{FEFF}\u{E0000}-\u{E007F}]").expect("valid regex")
});
static USES: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"^\s*-?\s*uses:\s*['"]?([^\s'"#]+)"##).expect("valid regex"));
static PINNED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"@[0-9a-f]{40}$").expect("valid regex"));

/// Removes `//` and `/* */` comments outside strings, for VS Code's JSONC files.
pub fn strip_jsonc(text: &str) -> String {
    let (mut out, mut chars) = (String::with_capacity(text.len()), text.chars().peekable());
    let mut in_string = false;
    while let Some(c) = chars.next() {
        if in_string {
            out.push(c);
            if c == '\\' {
                if let Some(n) = chars.next() {
                    out.push(n);
                }
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        match (c, chars.peek()) {
            ('"', _) => {
                in_string = true;
                out.push(c);
            }
            ('/', Some('/')) => {
                for n in chars.by_ref() {
                    if n == '\n' {
                        out.push('\n');
                        break;
                    }
                }
            }
            ('/', Some('*')) => {
                chars.next();
                let mut last = ' ';
                for n in chars.by_ref() {
                    if last == '*' && n == '/' {
                        break;
                    }
                    last = n;
                }
            }
            _ => out.push(c),
        }
    }
    // Trailing commas are allowed in JSONC too.
    Regex::new(r",(\s*[}\]])")
        .expect("valid regex")
        .replace_all(&out, "$1")
        .into_owned()
}

fn json(text: &str) -> Option<serde_json::Value> {
    serde_json::from_str(&strip_jsonc(text)).ok()
}

/// One file of the project, by its path relative to the project root.
pub struct File {
    pub path: PathBuf,
    pub text: String,
}

fn line_of(text: &str, needle: &str) -> usize {
    text.lines()
        .position(|l| l.contains(needle))
        .map_or(1, |i| i + 1)
}

fn evidence(f: &File, line: usize, extra: &str) -> String {
    let shown = f
        .text
        .lines()
        .nth(line.saturating_sub(1))
        .unwrap_or("")
        .trim();
    format!(
        "{}, line {line}: {}{extra}",
        f.path.display(),
        crate::model::clean(shown)
    )
}

/// The rules over one project's files. `git_config` is `.git/config`, which git
/// never clones but a shared or downloaded tree may carry.
pub fn assess(r: &mut Report, files: &[File], git_config: Option<&str>) {
    for f in files {
        let p = f.path.to_string_lossy().replace('\\', "/");
        let name = f
            .path
            .file_name()
            .map(|n| n.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        if p == ".vscode/tasks.json" {
            if let Some(tasks) = json(&f.text)
                .and_then(|v| v.get("tasks").cloned())
                .and_then(|t| t.as_array().cloned())
            {
                for t in tasks.iter().filter(|t| {
                    t.pointer("/runOptions/runOn").and_then(|v| v.as_str()) == Some("folderOpen")
                }) {
                    let label = t.get("label").and_then(|v| v.as_str()).unwrap_or("a task");
                    let command = t.get("command").and_then(|v| v.as_str()).unwrap_or("");
                    r.add(
                        "repo.autorun-task",
                        Severity::Review,
                        &format!("Opening this folder in VS Code runs \"{label}\""),
                        evidence(f, line_of(&f.text, "folderOpen"), &format!("\ncommand: {}", crate::model::clean(command))),
                        "Read the task before opening the folder in a trusted window; VS Code asks only once per folder.",
                    );
                }
            }
        }
        if name == ".envrc" {
            r.add(
                "repo.envrc",
                Severity::Review,
                "direnv runs .envrc as a shell script when you enter the folder",
                evidence(f, 1, ""),
                "Read it before running `direnv allow`; it runs with your permissions every time you cd into the folder.",
            );
        }
        if p.ends_with("devcontainer.json") {
            if let Some(v) = json(&f.text) {
                for key in [
                    "initializeCommand",
                    "onCreateCommand",
                    "postCreateCommand",
                    "postStartCommand",
                    "postAttachCommand",
                ] {
                    let Some(cmd) = v.get(key) else { continue };
                    // initializeCommand runs on your machine, not in the container.
                    let host = key == "initializeCommand";
                    r.add(
                        "repo.devcontainer-command",
                        if host { Severity::High } else { Severity::Review },
                        &if host {
                            "The devcontainer runs a command on this computer before the container starts".to_string()
                        } else {
                            format!("The devcontainer runs a command ({key}) inside the container")
                        },
                        evidence(f, line_of(&f.text, key), &format!("\n{key}: {}", crate::model::clean(&cmd.to_string()))),
                        if host {
                            "initializeCommand runs outside the container with your permissions. Read it before reopening the folder in a container."
                        } else {
                            "The container can usually read the project and anything mounted into it. Read the command before building."
                        },
                    );
                }
            }
        }
        if name == ".gitattributes" {
            for (i, line) in f.text.lines().enumerate() {
                if line
                    .split_whitespace()
                    .any(|a| a.starts_with("filter=") && a != "filter=lfs")
                {
                    r.add(
                        "repo.git-filter",
                        Severity::Review,
                        "The project names a git filter that runs on checkout or commit",
                        evidence(f, i + 1, ""),
                        "A filter runs only if your git configuration defines it; check no setup script asks you to add one you have not read.",
                    );
                }
            }
        }
        if p.starts_with(".claude/") && name.starts_with("settings") && name.ends_with(".json") {
            if let Some(hooks) = json(&f.text).and_then(|v| v.get("hooks").cloned()) {
                r.add(
                    "repo.ai-hooks",
                    Severity::Review,
                    "Claude Code settings in the project define hooks that run commands",
                    evidence(f, line_of(&f.text, "\"hooks\""), &format!("\n{}", crate::model::clean(&hooks.to_string()))),
                    "Project hooks run shell commands while the AI tool works in this folder. Read them before trusting the project.",
                );
            }
        }
        if name == ".mcp.json" || p == ".cursor/mcp.json" || p == ".vscode/mcp.json" {
            if let Some(servers) = json(&f.text)
                .and_then(|v| v.get("mcpServers").or_else(|| v.get("servers")).cloned())
                .and_then(|s| s.as_object().cloned())
            {
                for (server, spec) in servers {
                    let command = spec.get("command").and_then(|c| c.as_str()).unwrap_or("");
                    if command.is_empty() {
                        continue;
                    }
                    let args = spec.get("args").map(|a| a.to_string()).unwrap_or_default();
                    r.add(
                        "repo.mcp-server",
                        Severity::Review,
                        &format!("The project configures an MCP server \"{server}\" that starts a program"),
                        evidence(f, line_of(&f.text, &server), &format!("\ncommand: {} {}", crate::model::clean(command), crate::model::clean(&args))),
                        "AI tools start project MCP servers on their own. Check the package and pin its version before allowing it.",
                    );
                }
            }
        }
        let instructions = [
            "claude.md",
            "agents.md",
            ".cursorrules",
            "copilot-instructions.md",
            "gemini.md",
            ".windsurfrules",
        ];
        if instructions.contains(&name.as_str()) || p.starts_with(".cursor/rules/") {
            if let Some((i, line)) = f.text.lines().enumerate().find(|(_, l)| HIDDEN.is_match(l)) {
                let chars: Vec<String> = HIDDEN
                    .find_iter(line)
                    .map(|m| {
                        format!(
                            "U+{:04X}",
                            m.as_str().chars().next().map_or(0, |c| c as u32)
                        )
                    })
                    .collect();
                r.add(
                    "repo.hidden-instructions",
                    Severity::High,
                    "An AI instruction file contains invisible characters",
                    format!("{}, line {}: {}", f.path.display(), i + 1, chars.join(" ")),
                    "Invisible characters can carry instructions a reader never sees. Open the file in an editor that shows them, and remove them before an AI tool reads it.",
                );
            }
        }
        if p.starts_with(".github/workflows/")
            && (name.ends_with(".yml") || name.ends_with(".yaml"))
        {
            workflow(r, f);
        }
    }
    if let Some(config) = git_config {
        if let Some((i, line)) = config
            .lines()
            .enumerate()
            .find(|(_, l)| l.trim_start().to_lowercase().starts_with("hookspath"))
        {
            r.add(
                "repo.git-hooks",
                Severity::Review,
                "This copy of the repository sends git hooks to a folder in the project",
                format!(".git/config, line {}: {}", i + 1, line.trim()),
                "Every commit, checkout or merge will run the scripts there. Read them, or remove core.hooksPath.",
            );
        }
    }
}

fn workflow(r: &mut Report, f: &File) {
    // One finding per action, naming every line that uses it.
    let mut unpinned: Vec<(String, Vec<usize>)> = vec![];
    for (i, line) in f.text.lines().enumerate() {
        let Some(c) = USES.captures(line) else {
            continue;
        };
        let action = &c[1];
        if action.starts_with("./") || action.starts_with("docker://") || PINNED.is_match(action) {
            continue;
        }
        match unpinned.iter_mut().find(|(a, _)| a == action) {
            Some((_, lines)) => lines.push(i + 1),
            None => unpinned.push((action.to_string(), vec![i + 1])),
        }
    }
    for (action, lines) in unpinned {
        let at = lines
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        r.add(
            "repo.workflow-unpinned",
            Severity::Review,
            &format!("A workflow uses {action} by tag, not by commit"),
            format!(
                "{}, line{} {at}: uses: {action}",
                f.path.display(),
                if lines.len() == 1 { "" } else { "s" }
            ),
            "A tag can be moved to different code. Pin third-party actions to a full commit SHA.",
        );
    }
    if f.text.contains("pull_request_target") {
        // The dangerous pattern: privileged trigger plus the pull request's own code.
        let checks_out_head = f.text.contains("github.event.pull_request.head");
        r.add(
            "repo.workflow-pull-request-target",
            if checks_out_head { Severity::High } else { Severity::Review },
            if checks_out_head {
                "A workflow runs pull-request code with the repository's secrets"
            } else {
                "A workflow uses the privileged pull_request_target trigger"
            },
            evidence(f, line_of(&f.text, if checks_out_head { "github.event.pull_request.head" } else { "pull_request_target" }), ""),
            "pull_request_target runs with write access and secrets. Never check out or run the pull request's code in it.",
        );
    }
}

/// Collects the project's text files (bounded) and assesses them.
pub fn repo_check(dir: &Path) -> Result<Report> {
    if !dir.is_dir() {
        bail!("{} is not a folder", dir.display());
    }
    let mut r = Report::new("Repository check", dir.display());
    r.coverage.push("Reads the project's files as data: IDE tasks, direnv, devcontainers, git attributes and hooks, AI-tool settings, MCP servers, instruction files and CI workflows. Nothing in the project is run.".into());
    let mut files = vec![];
    let mut seen = 0;
    let walker = walkdir::WalkDir::new(dir)
        .follow_links(false)
        .max_depth(8)
        .into_iter()
        .filter_entry(|e| {
            e.depth() == 0
                || !(e.file_type().is_dir()
                    && SKIP.contains(&e.file_name().to_string_lossy().as_ref()))
        });
    for e in walker.flatten() {
        if !e.file_type().is_file() {
            continue;
        }
        seen += 1;
        if seen > MAX_FILES {
            r.coverage.skipped.push(crate::model::Skipped {
                what: "Files past the first 20,000".into(),
                why: crate::model::SkipReason::LimitReached,
                detail: dir.display().to_string(),
            });
            break;
        }
        let rel = e.path().strip_prefix(dir).unwrap_or(e.path()).to_path_buf();
        let interesting = {
            let p = rel.to_string_lossy();
            let n = e.file_name().to_string_lossy().to_lowercase();
            p.starts_with(".vscode/")
                || p.starts_with(".claude/")
                || p.starts_with(".cursor/")
                || p.starts_with(".github/")
                || p.contains(".devcontainer")
                || [
                    ".envrc",
                    ".gitattributes",
                    ".mcp.json",
                    "claude.md",
                    "agents.md",
                    ".cursorrules",
                    "gemini.md",
                    ".windsurfrules",
                ]
                .contains(&n.as_str())
        };
        if !interesting {
            continue;
        }
        if let Ok(bytes) = read_file(e.path(), MAX_INPUT) {
            files.push(File {
                path: rel,
                text: String::from_utf8_lossy(&bytes).into_owned(),
            });
        }
    }
    let git_config = read_file(&dir.join(".git/config"), MAX_INPUT)
        .ok()
        .map(|b| String::from_utf8_lossy(&b).into_owned());
    r.coverage
        .checked
        .push(format!("{} configuration files", files.len()));
    assess(&mut r, &files, git_config.as_deref());
    r.findings.sort_by_key(|f| std::cmp::Reverse(f.severity));
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_comments_and_trailing_commas_outside_strings() {
        let v: serde_json::Value = serde_json::from_str(&strip_jsonc(
            "{\n // a comment\n \"url\": \"https://x//y\", /* gone */ \"a\": [1,],\n}",
        ))
        .unwrap();
        assert_eq!(v["url"], "https://x//y");
        assert_eq!(v["a"][0], 1);
    }
}
