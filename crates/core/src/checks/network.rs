//! Rules over sockets: what a listening program exposes, and to whom.
use crate::{
    model::{Report, Severity},
    platform::{Protocol, Socket},
};
use std::net::IpAddr;

/// Container daemon API ports.
const DOCKER_API: [u16; 2] = [2375, 2376];
/// Remote-debugging ports: whoever reaches them can run code in that program.
const DEBUGGERS: [(u16, &str); 3] = [
    (9229, "Node.js inspector"),
    (9222, "Chromium DevTools"),
    (5678, "Python debugpy"),
];
/// Local AI and notebook servers that are often started for one machine only.
const LAN_SERVICES: [(u16, &str); 7] = [
    (11434, "Ollama"),
    (8888, "Jupyter"),
    (7860, "Gradio"),
    (8188, "ComfyUI"),
    (1234, "LM Studio"),
    (6006, "TensorBoard"),
    (8501, "Streamlit"),
];

/// Listeners every desktop of a distribution has by default, which a wildcard
/// finding would only teach people to ignore.
const SYSTEM_DEFAULTS: [(u16, &str); 1] = [(5355, "LLMNR name resolution from systemd-resolved")];

/// What a well-known default listener is, when it is one.
pub fn system_default(socket: &Socket) -> Option<&'static str> {
    SYSTEM_DEFAULTS
        .iter()
        .find(|(port, _)| *port == socket.local.port() && socket.process.is_none())
        .map(|(_, what)| *what)
}

/// What to do, from the registry, so a finding and its timeline entry say the same.
fn remedy(rule: &str) -> &'static str {
    crate::rules::get(rule).map_or("", |r| r.remediation)
}

fn loopback(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v) => v.is_loopback(),
        IpAddr::V6(v) => v.is_loopback() || v.to_ipv4_mapped().is_some_and(|m| m.is_loopback()),
    }
}

/// Who a listening socket is reachable from, in words.
pub fn reach(socket: &Socket) -> &'static str {
    let ip = socket.local.ip();
    if loopback(ip) {
        "this computer only"
    } else if ip.is_unspecified() {
        "every network interface"
    } else {
        "one network interface"
    }
}

pub fn process_name(socket: &Socket) -> String {
    socket
        .process
        .as_ref()
        .map(|p| p.name.clone())
        .unwrap_or_else(|| {
            if socket.uid == 0 {
                "A system program (root)".into()
            } else {
                "A program run by another account".into()
            }
        })
}

/// The listener rules for one socket. Loopback-only listeners never fire.
pub fn listener_findings(r: &mut Report, socket: &Socket) {
    if socket.protocol != Protocol::Tcp
        || !socket.listening()
        || loopback(socket.local.ip())
        || system_default(socket).is_some()
    {
        return;
    }
    let who = process_name(socket);
    let port = socket.local.port();
    let evidence = format!(
        "{} {}\n{}",
        "tcp listen",
        socket.local,
        socket
            .process
            .as_ref()
            .map(|p| format!(
                "{} (pid {}) {}",
                p.name,
                p.pid,
                p.exe
                    .as_ref()
                    .map(|e| e.display().to_string())
                    .unwrap_or_default()
            ))
            .unwrap_or_else(|| format!("uid {}", socket.uid))
    );
    if socket.local.ip().is_unspecified() {
        r.add(
            "net.listener.wildcard",
            Severity::Review,
            &format!("{who} listens on every network interface, port {port}"),
            &evidence,
            remedy("net.listener.wildcard"),
        );
    }
    if DOCKER_API.contains(&port) {
        r.add(
            "net.listener.docker_api",
            Severity::High,
            &format!("The container API is reachable on port {port}"),
            &evidence,
            remedy("net.listener.docker_api"),
        );
    }
    if let Some((_, name)) = DEBUGGERS.iter().find(|(p, _)| *p == port) {
        r.add(
            "net.listener.debugger",
            Severity::High,
            &format!("A {name} port is reachable from the network"),
            &evidence,
            remedy("net.listener.debugger"),
        );
    }
    if let Some((_, name)) = LAN_SERVICES.iter().find(|(p, _)| *p == port) {
        r.add(
            "net.listener.lan_service",
            Severity::Review,
            &format!("{name} is reachable from the network on port {port}"),
            &evidence,
            remedy("net.listener.lan_service"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn system_listener(local: &str) -> Socket {
        Socket {
            protocol: Protocol::Tcp,
            local: local.parse().unwrap(),
            remote: None,
            state: "listen".into(),
            uid: 193,
            process: None,
        }
    }

    #[test]
    fn distribution_defaults_are_not_findings_but_sshd_is() {
        let mut r = Report::new("Listener", "test");
        listener_findings(&mut r, &system_listener("0.0.0.0:5355"));
        assert!(r.findings.is_empty());
        listener_findings(&mut r, &system_listener("0.0.0.0:22"));
        assert_eq!(r.findings[0].rule, "net.listener.wildcard");
    }
}
