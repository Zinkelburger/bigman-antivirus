use crate::{
    analyzer,
    model::{Report, Severity},
    runner::{self, Cancel},
    store::{digest, read_file, Store, MAX_INPUT},
};
use anyhow::{bail, Context, Result};
use std::{
    fs,
    io::Read,
    net::{IpAddr, ToSocketAddrs},
    path::{Path, PathBuf},
    process::Command,
};
use url::Url;

pub fn public_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v) => {
            let o = v.octets();
            !(v.is_private()
                || v.is_loopback()
                || v.is_link_local()
                || v.is_unspecified()
                || v.is_multicast()
                || v.is_broadcast()
                || v.is_documentation()
                || o[0] == 0
                || o[0] >= 240
                || (o[0] == 100 && (64..=127).contains(&o[1]))
                || (o[0] == 198 && (o[1] == 18 || o[1] == 19)))
        }
        // Restrict fetches to global IPv6 unicast, exclude documentation and transition ranges.
        IpAddr::V6(v) => {
            let s = v.segments();
            (s[0] & 0xe000) == 0x2000
                && !(s[0] == 0x2001 && (s[1] < 0x0200 || s[1] == 0x0db8))
                && s[0] != 0x2002
        }
    }
}
fn remote(store: &Store, input: &str, stop: &Cancel) -> Result<(Vec<u8>, Vec<String>)> {
    let mut url = Url::parse(input)?;
    let mut chain = vec![];
    for _ in 0..6 {
        if url.scheme() != "https" || !url.username().is_empty() || url.password().is_some() {
            bail!("Installer URLs must use HTTPS without embedded credentials");
        }
        let host = url.host_str().context("URL has no hostname")?;
        let port = url.port_or_known_default().unwrap_or(443);
        let addresses: Vec<_> = (host, port).to_socket_addrs()?.collect();
        if addresses.is_empty() || addresses.iter().any(|a| !public_ip(a.ip())) {
            bail!("Refusing private, local, reserved or unresolved destination");
        }
        let addr = addresses[0].ip();
        let addr = if addr.is_ipv6() {
            format!("[{addr}]")
        } else {
            addr.to_string()
        };
        let tmp = tempfile::tempdir_in(&store.root)?;
        let body = tmp.path().join("body");
        let headers = tmp.path().join("headers");
        let output = runner::run(
            Command::new("curl")
                .args([
                    "--disable",
                    "--silent",
                    "--show-error",
                    "--proto",
                    "=https",
                    "--noproxy",
                    "*",
                    "--connect-timeout",
                    "10",
                    "--max-time",
                    "30",
                    "--max-filesize",
                    "8388608",
                    "--resolve",
                    &format!("{host}:{port}:{addr}"),
                    "--dump-header",
                ])
                .arg(&headers)
                .arg("--output")
                .arg(&body)
                .arg("--")
                .arg(url.as_str()),
            35,
            stop,
        )?;
        if !output.ok() {
            bail!("Download failed: {}", output.stderr);
        }
        let h = String::from_utf8_lossy(&read_file(&headers, 65536)?).into_owned();
        let status = h
            .lines()
            .find(|l| l.starts_with("HTTP/"))
            .and_then(|l| l.split_whitespace().nth(1))
            .unwrap_or("");
        chain.push(url.to_string());
        if ["301", "302", "303", "307", "308"].contains(&status) {
            let location = h
                .lines()
                .find_map(|l| {
                    l.split_once(':')
                        .filter(|(k, _)| k.eq_ignore_ascii_case("location"))
                        .map(|(_, v)| v.trim())
                })
                .context("Redirect without Location")?;
            url = url.join(location)?;
            continue;
        }
        if status != "200" {
            bail!("Server returned HTTP {status}");
        }
        return Ok((read_file(&body, MAX_INPUT)?, chain));
    }
    bail!("Too many redirects")
}

fn inspect_inner(store: &Store, target: &str, stop: &Cancel) -> Result<Report> {
    let (bytes, chain) = if target.starts_with("https://") {
        remote(store, target, stop)?
    } else {
        (read_file(Path::new(target), MAX_INPUT)?, vec![])
    };
    let content = std::str::from_utf8(&bytes).context("Installer must be UTF-8 text")?;
    let mut r = analyzer::analyze_content(content);
    r.subject = target.into();
    let artifact = format!("artifacts/{}.sh", r.id);
    store.bytes(&artifact, &bytes)?;
    r.coverage.push(format!(
        "Saved exact artifact: {}",
        store.root.join(&artifact).display()
    ));
    if !chain.is_empty() {
        r.coverage
            .push(format!("Download chain: {}", chain.join(" → ")));
    }
    if runner::available("shellcheck") {
        let o = runner::run(
            Command::new("shellcheck")
                .args(["--format=json", "--shell=bash"])
                .arg(store.root.join(artifact)),
            20,
            stop,
        )?;
        if o.interrupted || o.truncated || !matches!(o.code, Some(0 | 1)) {
            r.errors
                .push(format!("ShellCheck did not complete: {}", o.stderr));
        } else {
            let messages: serde_json::Value = serde_json::from_str(&o.stdout)?;
            if let Some(ms) = messages.as_array() {
                for m in ms {
                    r.add(
                        "shellcheck",
                        Severity::Review,
                        &format!("ShellCheck SC{}", m["code"]),
                        format!("Line {}: {}", m["line"], m["message"]),
                        "Review the shell correctness warning; it is not a malware verdict.",
                    );
                }
            }
        }
    } else {
        r.errors
            .push("ShellCheck is not installed; only built-in capability rules ran.".into());
    }
    store.save(&r)?;
    Ok(r)
}

pub fn packages(store: &Store, path: &Path) -> Result<Report> {
    let mut r = Report::new("Package script inventory", path.display());
    r.coverage.push("Reads package manifests and script source as data; never invokes npm, build backends, or lifecycle hooks. Unknown/generated commands remain unresolved.".into());
    if path.extension().is_some_and(|e| e == "tgz" || e == "gz") {
        let bytes = read_file(path, 32 * MAX_INPUT)?;
        r.sha256 = Some(digest(&bytes));
        let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(bytes.as_slice()));
        let mut total = 0usize;
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
            } else if p
                .extension()
                .is_some_and(|e| ["js", "cjs", "mjs", "sh"].iter().any(|x| e == *x))
            {
                source_findings(&mut r, &p, &b);
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
    store.save(&r)?;
    Ok(r)
}
fn manifest(r: &mut Report, p: &Path, b: &[u8]) -> Result<()> {
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
fn source_findings(r: &mut Report, p: &Path, b: &[u8]) {
    let scan = analyzer::analyze_content(&String::from_utf8_lossy(b));
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

pub fn sandbox(store: &Store, id: &str, hash: &str, stop: &Cancel) -> Result<Report> {
    let original = store.report(id)?;
    if original.kind != "Installer inspection" || original.sha256.as_deref() != Some(hash) {
        bail!("Approve the exact SHA-256 of a saved installer report");
    }
    let bytes = read_file(&store.root.join(format!("artifacts/{id}.sh")), MAX_INPUT)?;
    if digest(&bytes) != hash {
        bail!("Saved artifact changed since inspection");
    }
    let scratch = tempfile::tempdir_in(&store.root)?;
    let input = scratch.path().join("installer.sh");
    fs::write(&input, &bytes)?;
    let mut cmd = Command::new("bwrap");
    cmd.args([
        "--unshare-all",
        "--die-with-parent",
        "--new-session",
        "--cap-drop",
        "ALL",
        "--clearenv",
        "--setenv",
        "PATH",
        "/usr/bin:/bin",
        "--setenv",
        "HOME",
        "/home/user",
        "--setenv",
        "LANG",
        "C",
        "--tmpfs",
        "/",
        "--ro-bind",
        "/usr",
        "/usr",
    ]);
    for p in ["/bin", "/lib", "/lib64"] {
        if Path::new(p).exists() {
            cmd.args(["--ro-bind", p, p]);
        }
    }
    cmd.args([
        "--proc",
        "/proc",
        "--dev",
        "/dev",
        "--tmpfs",
        "/tmp",
        "--dir",
        "/home",
        "--dir",
        "/home/user",
        "--dir",
        "/work",
        "--ro-bind",
    ])
    .arg(&input)
    .arg("/installer.sh")
    .args(["--chdir", "/work", "--"]);
    if runner::available("strace") {
        cmd.args([
            "strace",
            "-f",
            "-qq",
            "-e",
            "trace=%file,%process,%network",
            "--",
        ]);
    }
    cmd.args(["/bin/bash", "--noprofile", "--norc", "/installer.sh"]);
    // Enforce resource limits in the child before the sandbox starts.
    use std::os::unix::process::CommandExt;
    let process_limit = runner::process_budget()?;
    unsafe {
        cmd.pre_exec(move || {
            for (resource, value) in [
                (libc::RLIMIT_CPU, 60),
                (libc::RLIMIT_AS, 1024 * 1024 * 1024),
                (libc::RLIMIT_FSIZE, 64 * 1024 * 1024),
                (libc::RLIMIT_NOFILE, 128),
                (libc::RLIMIT_NPROC, process_limit),
            ] {
                let lim = libc::rlimit {
                    rlim_cur: value,
                    rlim_max: value,
                };
                if libc::setrlimit(resource, &lim) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
            }
            Ok(())
        });
    }
    let output = runner::run(&mut cmd, 90, stop)?;
    let mut r = Report::new("Restricted installer rehearsal", &original.subject);
    r.sha256 = Some(hash.into());
    r.coverage.push("Disposable filesystem, no host home/configuration/credentials/service sockets, no network, no privileges; 90-second timeout and resource limits. A rehearsal is not proof of safety. No automatic host install or artifact promotion.".into());
    if !output.ok() {
        r.errors.push(format!("Rehearsal did not complete successfully (exit {:?}, timeout/cancel={}, truncated={}). It was not retried unrestricted.",output.code,output.interrupted,output.truncated));
    }
    if !runner::available("strace") {
        r.errors
            .push("strace missing: syscall observation unavailable".into());
    }
    r.coverage.push("Process count is constrained relative to current usage of your Unix user, not by a dedicated per-job cgroup. Do not use this feature for arbitrary malware detonation.".into());
    for line in output.stderr.lines().take(2000) {
        if r.findings.len() >= 100 {
            r.errors.push("Trace finding limit reached".into());
            break;
        }
        if [
            ".aws/",
            ".ssh/",
            ".npmrc",
            "Login Data",
            ".mozilla",
            ".git-credentials",
        ]
        .iter()
        .any(|s| line.contains(s))
        {
            r.add("trace.sensitive-path",Severity::Review,"Execution referenced a sensitive path",line,"Check the syscall result: an attempted access may have been denied. The rehearsal does not expose real home credentials.");
        } else if line.contains("connect(") {
            r.add(
                "trace.network",
                Severity::Review,
                "Execution attempted a network connection",
                line,
                "The rehearsal has no external network. Review why the connection was attempted.",
            );
        } else if line.contains("execve(") {
            r.add(
                "trace.process",
                Severity::Info,
                "Program execution observed",
                line,
                "Review the process and its arguments.",
            );
        }
    }
    r.details = format!(
        "Output:\n{}\nTrace/errors:\n{}",
        output.stdout, output.stderr
    );
    store.save(&r)?;
    Ok(r)
}

pub fn inspect(store: &Store, target: &str, stop: &Cancel) -> Result<Report> {
    match inspect_inner(store, target, stop) {
        Ok(r) => Ok(r),
        Err(e) => {
            let mut r = Report::new("Installer inspection", target);
            r.errors.push(e.to_string());
            store.save(&r)?;
            Ok(r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn blocks_local_fetch_destinations() {
        for ip in [
            "127.0.0.1",
            "10.0.0.1",
            "169.254.169.254",
            "100.64.0.1",
            "::1",
            "::ffff:127.0.0.1",
            "2001:db8::1",
        ] {
            assert!(!public_ip(ip.parse().unwrap()), "{ip}");
        }
        assert!(public_ip("1.1.1.1".parse().unwrap()));
    }
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
