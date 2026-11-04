//! `repo-check URL`: read a hosted repository before cloning it. Fake job
//! interviews send a repository to clone and run; the advice is to read it first,
//! and this does that reading for you. Git fetches the latest commit into a bare
//! repository, which has no working tree, so nothing is checked out, no hook or
//! filter runs, and no editor opens it. The files the repo-check rules read are
//! taken straight from git's object store.
use crate::{
    checks::{packages, repo, source},
    model::{Report, SkipReason},
    runner::{self, Cancel},
    store::{Store, MAX_INPUT},
};
use anyhow::{bail, Context, Result};
use std::{
    collections::BTreeMap,
    fs,
    path::{Component, Path, PathBuf},
    process::Command,
};

/// Code hosts whose repository URLs are read this way. Other HTTPS URLs are
/// installers or downloads, and go to the installer inspection.
const HOSTS: [&str; 4] = ["github.com", "gitlab.com", "codeberg.org", "bitbucket.org"];
/// Blobs larger than this are not downloaded at all: no rule reads a file that big.
const BLOB_LIMIT: &str = "blob:limit=2m";
/// Bytes of file content read from the object store, in total.
const MAX_READ: u64 = 256 * 1024 * 1024;
/// Environment variables passed to git. Everything else, including your proxies,
/// credentials and any GIT_* settings, is left out.
const KEEP_ENV: [&str; 4] = ["PATH", "HOME", "SSL_CERT_FILE", "SSL_CERT_DIR"];

/// The repository's clone URL, if `target` is a repository page on a known host:
/// `https://github.com/OWNER/REPO`, optionally ending in `.git` or `/`. GitLab
/// allows groups within groups.
pub fn repo_url(target: &str) -> Option<String> {
    let rest = target.strip_prefix("https://")?;
    let (host, path) = rest.split_once('/')?;
    let host = host.to_ascii_lowercase();
    let host = host.strip_prefix("www.").unwrap_or(&host);
    if !HOSTS.contains(&host) {
        return None;
    }
    let path = path.trim_end_matches('/');
    let path = path.strip_suffix(".git").unwrap_or(path);
    let parts: Vec<&str> = path.split('/').collect();
    let ok_part = |p: &&str| {
        !p.is_empty()
            && !p.starts_with(['.', '-'])
            && p.chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
    };
    let shape = if host == "gitlab.com" {
        parts.len() >= 2
    } else {
        parts.len() == 2
    };
    (shape && parts.iter().all(ok_part)).then(|| format!("https://{host}/{}.git", parts.join("/")))
}

/// git with only HTTPS allowed, no hooks, no credential helpers, no prompts and no
/// user or system configuration, run against `bare` when given.
fn git(bare: Option<&Path>) -> Command {
    let mut c = Command::new("git");
    c.env_clear();
    for key in KEEP_ENV {
        if let Some(v) = std::env::var_os(key) {
            c.env(key, v);
        }
    }
    c.env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        // A blob left out by the size filter stays out; reading it must not fetch it.
        .env("GIT_NO_LAZY_FETCH", "1")
        .args([
            "-c",
            "protocol.allow=never",
            "-c",
            "protocol.https.allow=always",
            "-c",
            "core.hooksPath=/dev/null",
            "-c",
            "credential.helper=",
        ]);
    if let Some(dir) = bare {
        c.arg("--git-dir").arg(dir);
    }
    c
}

/// One entry of the commit's tree.
struct Blob {
    mode: String,
    oid: String,
    path: PathBuf,
}

fn tree(raw: &[u8]) -> Vec<Blob> {
    raw.split(|b| *b == 0)
        .filter_map(|rec| {
            let rec = std::str::from_utf8(rec).ok()?;
            let (meta, path) = rec.split_once('\t')?;
            let mut m = meta.split(' ');
            let (mode, kind, oid) = (m.next()?, m.next()?, m.next()?);
            (kind == "blob").then(|| Blob {
                mode: mode.into(),
                oid: oid.into(),
                path: PathBuf::from(path),
            })
        })
        .collect()
}

/// The contents `git cat-file --batch` printed for each requested object.
fn batch(raw: &[u8]) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    let mut rest = raw;
    while let Some(nl) = rest.iter().position(|b| *b == b'\n') {
        let header = String::from_utf8_lossy(&rest[..nl]).into_owned();
        rest = &rest[nl + 1..];
        let mut h = header.split(' ');
        let (Some(oid), Some(kind), Some(size)) = (h.next(), h.next(), h.next()) else {
            // "<oid> missing": left out by the size filter.
            continue;
        };
        let Ok(size) = size.parse::<usize>() else {
            continue;
        };
        if rest.len() < size || kind != "blob" {
            break;
        }
        out.insert(oid.to_string(), rest[..size].to_vec());
        rest = rest.get(size + 1..).unwrap_or_default();
    }
    out
}

/// Reads the named objects in one git process.
fn read_objects(
    bare: &Path,
    scratch: &Path,
    oids: &[&str],
    stop: &Cancel,
) -> Result<BTreeMap<String, Vec<u8>>> {
    if oids.is_empty() {
        return Ok(BTreeMap::new());
    }
    let out = scratch.join("objects");
    let o = runner::run_to_file(
        git(Some(bare)).args(["cat-file", "--batch"]),
        format!("{}\n", oids.join("\n")).as_bytes(),
        fs::File::create(&out)?,
        120,
        stop,
    )?;
    if !o.ok() {
        bail!(
            "git could not read the repository's files: {}",
            o.stderr.trim()
        );
    }
    Ok(batch(&fs::read(&out)?))
}

/// A link's target, as a path inside the repository, if it stays inside.
fn resolve(link: &Path, target: &[u8]) -> Option<PathBuf> {
    let target = Path::new(std::str::from_utf8(target).ok()?);
    if target.is_absolute() {
        return None;
    }
    let mut out = link.parent().unwrap_or(Path::new("")).to_path_buf();
    for c in target.components() {
        match c {
            Component::Normal(n) => out.push(n),
            Component::CurDir => {}
            Component::ParentDir => {
                if !out.pop() {
                    return None;
                }
            }
            _ => return None,
        }
    }
    Some(out)
}

/// How a file is read: in full (configuration, manifests, source) or only its head.
fn want(path: &Path) -> Option<source::Want> {
    let p = path.to_string_lossy();
    if p.split('/')
        .any(|c| repo::SKIP.contains(&c) || c == "site-packages")
        || path.components().count() > 9
    {
        return None;
    }
    if repo::interesting(path) || path.file_name().is_some_and(|n| n == "package.json") {
        return Some(source::Want::Text);
    }
    source::wanted(path)
}

/// Fetches the latest commit of a hosted repository without checking it out, and
/// runs the project rules on its files.
pub fn check(store: &Store, target: &str, stop: &Cancel) -> Result<Report> {
    let url = repo_url(target)
        .context("Give the repository's page address, such as https://github.com/OWNER/REPO")?;
    if !runner::available("git") {
        bail!("git is not installed; it is needed to read a repository before cloning it");
    }
    let scratch = tempfile::tempdir_in(&store.root)?;
    let bare = scratch.path().join("bare.git");
    let o = runner::run(
        git(None)
            .args([
                "clone",
                "--bare",
                "--depth=1",
                "--single-branch",
                "--no-tags",
                "--quiet",
            ])
            .arg(format!("--filter={BLOB_LIMIT}"))
            .arg("--")
            .arg(&url)
            .arg(&bare),
        180,
        stop,
    )?;
    if o.interrupted {
        bail!("Reading the repository was stopped or took longer than 3 minutes");
    }
    if !o.ok() {
        bail!(
            "git could not fetch {url}. It may be private, renamed or deleted; Bigman does not use your credentials. {}",
            o.stderr.trim()
        );
    }
    let commit = runner::run(git(Some(&bare)).args(["rev-parse", "HEAD"]), 30, stop)?
        .stdout
        .trim()
        .to_string();
    let listing = scratch.path().join("tree");
    let o = runner::run_to_file(
        git(Some(&bare)).args(["ls-tree", "-r", "-z", "--full-tree", "HEAD"]),
        &[],
        fs::File::create(&listing)?,
        60,
        stop,
    )?;
    if !o.ok() {
        bail!("git could not list the repository: {}", o.stderr.trim());
    }
    let entries = tree(&fs::read(&listing)?);

    let mut r = Report::new("Project inspection", target);
    r.coverage.push(format!(
        "Read before cloning: git fetched the latest commit of the default branch into a bare repository, so nothing was checked out and nothing in the project ran. The owner can push different code at any time, so clone this exact commit ({commit}) if you go ahead, and still not on a computer that holds keys, wallets or work access."
    ));
    r.coverage.push("Reads the project's files as data: IDE tasks, devcontainers, git attributes, AI-tool settings, MCP servers, instruction files, CI workflows, package scripts, and its own source for code that runs fetched or decoded text on start. Lockfiles and Python and Rust build files are read only in a cloned folder.".into());

    // Links inside the repository are followed one level, the way an editor would.
    let links: Vec<&Blob> = entries.iter().filter(|e| e.mode == "120000").collect();
    let link_targets = read_objects(
        &bare,
        scratch.path(),
        &links.iter().map(|l| l.oid.as_str()).collect::<Vec<_>>(),
        stop,
    )?;
    let mut files: Vec<(PathBuf, &str)> = entries
        .iter()
        .filter(|e| e.mode.starts_with("100"))
        .map(|e| (e.path.clone(), e.oid.as_str()))
        .collect();
    let mut outside = vec![];
    for link in &links {
        let Some(to) = link_targets
            .get(&link.oid)
            .and_then(|t| resolve(&link.path, t))
        else {
            outside.push(link.path.display().to_string());
            continue;
        };
        for e in entries.iter().filter(|e| e.mode.starts_with("100")) {
            if e.path == to {
                files.push((link.path.clone(), &e.oid));
            } else if let Ok(rest) = e.path.strip_prefix(&to) {
                files.push((link.path.join(rest), &e.oid));
            }
        }
    }
    if !outside.is_empty() {
        r.coverage.skip(
            format!("{} links", outside.len()),
            SkipReason::NotApplicable,
            format!(
                "Point outside the repository; not followed: {}",
                outside.join(", ")
            ),
        );
    }

    // Configuration at the root first, so no number of other files can push it out.
    files.sort_by_key(|(p, _)| (!repo::interesting(p), p.clone()));
    let mut wanted: Vec<(PathBuf, &str, source::Want)> = vec![];
    let (mut sources, mut skipped) = (0, 0);
    for (path, oid) in files {
        let Some(w) = want(&path) else { continue };
        let config = repo::interesting(&path) || path.ends_with("package.json");
        if !config {
            if sources >= repo::MAX_SOURCE_FILES {
                skipped += 1;
                continue;
            }
            sources += 1;
        }
        wanted.push((path, oid, w));
    }
    let mut oids: Vec<&str> = wanted.iter().map(|(_, o, _)| *o).collect();
    oids.sort_unstable();
    oids.dedup();
    let blobs = read_objects(&bare, scratch.path(), &oids, stop)?;

    let (mut read, mut missing, mut over) = (0u64, 0, 0);
    let mut project = vec![];
    let mut manifests = 0;
    for (path, oid, w) in wanted {
        let Some(bytes) = blobs.get(oid) else {
            missing += 1;
            continue;
        };
        let keep = match w {
            source::Want::Head => &bytes[..bytes.len().min(source::ASSET_HEAD as usize)],
            source::Want::Text
                if bytes.len() as u64 > MAX_INPUT
                    || (!repo::interesting(&path)
                        && bytes.len() as u64 > repo::MAX_SOURCE_FILE) =>
            {
                over += 1;
                continue;
            }
            source::Want::Text => bytes.as_slice(),
        };
        read += keep.len() as u64;
        if read > MAX_READ {
            over += 1;
            continue;
        }
        if path.file_name().is_some_and(|n| n == "package.json") {
            manifests += 1;
            if let Err(e) = packages::manifest(&mut r, &path, keep) {
                r.errors.push(format!("{e:#}"));
            }
        }
        project.push(repo::File {
            path,
            text: String::from_utf8_lossy(keep).into_owned(),
        });
    }
    if missing + over + skipped > 0 {
        r.coverage.skip(
            format!("{} files", missing + over + skipped),
            SkipReason::LimitReached,
            "Files over 2 MiB are not downloaded, and source files past the first 5,000 are not read",
        );
    }
    r.coverage.checked(format!("Commit {commit}"));
    r.coverage.checked(format!(
        "{} files from {} in the repository",
        project.len(),
        entries.len()
    ));
    if manifests > 0 {
        r.coverage.checked(format!("{manifests} package manifests"));
    }
    repo::assess(&mut r, &project, None);
    r.findings.sort_by_key(|f| std::cmp::Reverse(f.severity));
    store.save(&r)?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_repository_pages_only() {
        assert_eq!(
            repo_url("https://github.com/owner/take-home").as_deref(),
            Some("https://github.com/owner/take-home.git")
        );
        assert_eq!(
            repo_url("https://GitHub.com/owner/take-home.git/").as_deref(),
            Some("https://github.com/owner/take-home.git")
        );
        assert_eq!(
            repo_url("https://gitlab.com/group/sub/dex-mvp").as_deref(),
            Some("https://gitlab.com/group/sub/dex-mvp.git")
        );
        for not in [
            "https://github.com/owner",
            "https://github.com/owner/repo/blob/main/install.sh",
            "https://raw.githubusercontent.com/owner/repo/main/install.sh",
            "https://example.org/owner/repo",
            "http://github.com/owner/repo",
            "https://github.com/owner/--upload-pack=x",
            "https://github.com/owner/repo?x=1",
            "https://gitlab.com/group/repo/-/raw/main/x",
        ] {
            assert_eq!(repo_url(not), None, "{not}");
        }
    }

    #[test]
    fn parses_tree_and_batch_output() {
        let t = tree(b"100644 blob aaaa\t.vscode/tasks.json\x00120000 blob bbbb\tlink\x00160000 commit cccc\tsub\x00");
        assert_eq!(t.len(), 2);
        assert_eq!(t[0].path, Path::new(".vscode/tasks.json"));
        let b = batch(b"aaaa blob 3\nabc\ndddd missing\nbbbb blob 2\nxy\n");
        assert_eq!(b["aaaa"], b"abc");
        assert_eq!(b["bbbb"], b"xy");
        assert!(!b.contains_key("dddd"));
    }

    #[test]
    fn links_resolve_inside_the_repository_only() {
        assert_eq!(
            resolve(Path::new(".vscode"), b"config/editor"),
            Some(PathBuf::from("config/editor"))
        );
        assert_eq!(resolve(Path::new("a/b"), b"../c"), Some(PathBuf::from("c")));
        assert_eq!(resolve(Path::new("a"), b"../../etc"), None);
        assert_eq!(resolve(Path::new("a"), b"/etc/passwd"), None);
    }

    /// A real bare clone from a local repository, served over file:// only for this
    /// test: proves nothing is checked out and the rules see the files.
    #[test]
    fn reads_a_repository_without_checking_it_out() {
        if !runner::available("git") {
            return;
        }
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("src");
        fs::create_dir_all(src.join(".vscode")).unwrap();
        fs::create_dir_all(src.join("webfonts")).unwrap();
        fs::write(
            src.join(".vscode/tasks.json"),
            r#"{"tasks":[{"label":"x","command":"touch MUST_NOT_EXIST","runOptions":{"runOn":"folderOpen"}}]}"#,
        )
        .unwrap();
        fs::write(src.join("webfonts/fa.woff2"), "6869".repeat(40)).unwrap();
        let sh = |args: &[&str]| {
            assert!(Command::new("git")
                .args([
                    "-c",
                    "user.name=t",
                    "-c",
                    "user.email=t@t",
                    "-c",
                    "init.defaultBranch=main"
                ])
                .args(args)
                .current_dir(&src)
                .status()
                .unwrap()
                .success());
        };
        sh(&["init", "-q"]);
        sh(&["add", "."]);
        sh(&["commit", "-qm", "x"]);
        let bare = d.path().join("bare.git");
        let o = runner::run(
            Command::new("git")
                .args(["clone", "--bare", "-q"])
                .arg(&src)
                .arg(&bare),
            60,
            &runner::cancel(),
        )
        .unwrap();
        assert!(o.ok(), "{}", o.stderr);
        let mut listing = vec![];
        let o = runner::run_to_file(
            git(Some(&bare)).args(["ls-tree", "-r", "-z", "HEAD"]),
            &[],
            {
                let p = d.path().join("t");
                let f = fs::File::create(&p).unwrap();
                listing.push(p);
                f
            },
            30,
            &runner::cancel(),
        )
        .unwrap();
        assert!(o.ok(), "{}", o.stderr);
        let entries = tree(&fs::read(&listing[0]).unwrap());
        let oids: Vec<&str> = entries.iter().map(|e| e.oid.as_str()).collect();
        let blobs = read_objects(&bare, d.path(), &oids, &runner::cancel()).unwrap();
        assert_eq!(blobs.len(), 2);
        assert!(!bare.join("MUST_NOT_EXIST").exists() && !d.path().join("MUST_NOT_EXIST").exists());
    }
}
