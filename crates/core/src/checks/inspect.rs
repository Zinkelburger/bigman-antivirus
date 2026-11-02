use crate::{
    checks::shell,
    model::{Report, Severity},
    runner::{self, Cancel},
    store::{digest, read_file, Store, MAX_INPUT},
};
use anyhow::{bail, Context, Result};
use std::{
    fs,
    net::{IpAddr, ToSocketAddrs},
    path::Path,
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
    let mut r = shell::analyze_content(content);
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
            shellcheck_findings(&mut r, &o.stdout)?;
        }
    } else {
        r.errors
            .push("ShellCheck is not installed; only built-in capability rules ran.".into());
    }
    store.save(&r)?;
    Ok(r)
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
    r.coverage.push("Disposable filesystem, no host home/configuration/credentials/service sockets, no network, no privileges; 90-second timeout and resource limits. A rehearsal shows what the script tried in this one run, not what it would do elsewhere. No automatic host install or artifact promotion.".into());
    if !output.ok() {
        r.errors.push(format!("Rehearsal did not complete successfully (exit {:?}, timeout/cancel={}, truncated={}). It was not retried unrestricted.",output.code,output.interrupted,output.truncated));
    }
    if !runner::available("strace") {
        r.errors
            .push("strace missing: syscall observation unavailable".into());
    }
    r.coverage.push("Process count is constrained relative to current usage of your Unix user, not by a dedicated per-job cgroup. Do not use this feature for arbitrary malware detonation.".into());
    trace_findings(&mut r, &output.stderr);
    r.details = format!(
        "Output:\n{}\nTrace/errors:\n{}",
        output.stdout, output.stderr
    );
    store.save(&r)?;
    Ok(r)
}

/// The rehearsal rules, over strace output: sensitive paths, network attempts and
/// started programs.
pub fn trace_findings(r: &mut Report, trace: &str) {
    for line in trace.lines().take(2000) {
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
}

/// ShellCheck's JSON as findings. Errors when the output is not ShellCheck JSON.
pub fn shellcheck_findings(r: &mut Report, json: &str) -> Result<()> {
    let messages: serde_json::Value = serde_json::from_str(json)?;
    for m in messages.as_array().into_iter().flatten() {
        r.add(
            "shellcheck.warning",
            Severity::Review,
            &format!("ShellCheck SC{}", m["code"]),
            format!("Line {}: {}", m["line"], m["message"]),
            "Review the shell correctness warning; it is not a malware verdict.",
        );
    }
    Ok(())
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
}
