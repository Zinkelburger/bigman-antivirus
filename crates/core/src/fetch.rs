//! Bounded HTTPS downloads through curl: HTTPS only, no credentials in URLs, only
//! public addresses (pinned with --resolve so DNS cannot change between the check
//! and the connection), at most six redirects, and size and time limits. Shared
//! by installer inspection and advisory data.
use crate::{
    runner::{self, Cancel},
    store::read_file,
};
use anyhow::{bail, Context, Result};
use std::{
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
pub struct Limits {
    pub bytes: u64,
    pub seconds: u64,
}

pub struct Fetched {
    /// 200, or 304 when `if_modified_since` was given and nothing changed.
    pub status: u16,
    pub last_modified: Option<String>,
    /// Every URL requested, redirects included.
    pub chain: Vec<String>,
}

fn header<'a>(headers: &'a str, name: &str) -> Option<&'a str> {
    headers.lines().find_map(|l| {
        l.split_once(':')
            .filter(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.trim())
    })
}

/// Downloads `input` to `dest`. With `if_modified_since`, a 304 leaves `dest` alone.
pub fn download(
    input: &str,
    dest: &Path,
    limits: &Limits,
    if_modified_since: Option<&str>,
    stop: &Cancel,
) -> Result<Fetched> {
    let mut url = Url::parse(input)?;
    let mut chain = vec![];
    let dir = dest
        .parent()
        .context("Download destination has no folder")?;
    for _ in 0..6 {
        if url.scheme() != "https" || !url.username().is_empty() || url.password().is_some() {
            bail!("Downloads must use HTTPS without embedded credentials");
        }
        let host = url.host_str().context("URL has no hostname")?;
        let port = url.port_or_known_default().unwrap_or(443);
        let addresses: Vec<_> = (host, port).to_socket_addrs()?.collect();
        if addresses.is_empty() || addresses.iter().any(|a| !public_ip(a.ip())) {
            bail!("Refusing private, local, reserved or unresolved destination");
        }
        // Every checked address, so curl can fall back (e.g. from IPv6 without a route).
        let pinned = addresses
            .iter()
            .map(|a| {
                if a.is_ipv6() {
                    format!("[{}]", a.ip())
                } else {
                    a.ip().to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(",");
        let tmp = tempfile::tempdir_in(dir)?;
        let body = tmp.path().join("body");
        let headers = tmp.path().join("headers");
        let mut cmd = Command::new("curl");
        cmd.args([
            "--disable",
            "--silent",
            "--show-error",
            "--proto",
            "=https",
            "--noproxy",
            "*",
            "--compressed",
            "--connect-timeout",
            "10",
            "--max-time",
            &limits.seconds.to_string(),
            "--max-filesize",
            &limits.bytes.to_string(),
            "--resolve",
            &format!("{host}:{port}:{pinned}"),
            "--dump-header",
        ])
        .arg(&headers)
        .arg("--output")
        .arg(&body);
        if let Some(since) = if_modified_since {
            cmd.args(["--header", &format!("If-Modified-Since: {since}")]);
        }
        let output = runner::run(cmd.arg("--").arg(url.as_str()), limits.seconds + 5, stop)?;
        if !output.ok() {
            bail!("Download failed: {}", output.stderr.trim());
        }
        let h = String::from_utf8_lossy(&read_file(&headers, 65536)?).into_owned();
        let status = h
            .lines()
            .rfind(|l| l.starts_with("HTTP/"))
            .and_then(|l| l.split_whitespace().nth(1))
            .unwrap_or("");
        chain.push(url.to_string());
        if ["301", "302", "303", "307", "308"].contains(&status) {
            url = url.join(header(&h, "location").context("Redirect without Location")?)?;
            continue;
        }
        let last_modified = header(&h, "last-modified").map(str::to_string);
        return match status {
            "304" if if_modified_since.is_some() => Ok(Fetched {
                status: 304,
                last_modified,
                chain,
            }),
            "200" => {
                // --compressed decodes on the fly; the decoded size is bounded too.
                if std::fs::metadata(&body)?.len() > limits.bytes * 8 {
                    bail!("Download is larger than allowed");
                }
                std::fs::rename(&body, dest)?;
                // Private like everything else in the store.
                std::fs::set_permissions(
                    dest,
                    std::os::unix::fs::PermissionsExt::from_mode(0o600),
                )?;
                Ok(Fetched {
                    status: 200,
                    last_modified,
                    chain,
                })
            }
            other => bail!("Server returned HTTP {other}"),
        };
    }
    bail!("Too many redirects")
}
