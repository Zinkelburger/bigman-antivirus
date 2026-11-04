---
url: "https://www.sshguard.net/"
title: "SSHGuard"
captured: "2026-11-02T18:59:51Z"
via: "live"
sha256: "ba072dff6a1b670579fd934cf65bbd71db2ddc4090a3d4aa2c57173064d771fb"
wayback: "https://web.archive.org/web/20261021115111/https://www.sshguard.net/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# SSHGuard

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.sshguard.net/>
> Snapshot: <https://web.archive.org/web/20261021115111/https://www.sshguard.net/>

-  [Home](https://www.sshguard.net/)
-  [Get Started](https://www.sshguard.net/download/)
-  [Documentation](https://www.sshguard.net/docs/)
-  [News](https://www.sshguard.net/news/)
-  [Community](https://www.sshguard.net/support/)

SSHGuard protects hosts from brute-force attacks by:

- Monitoring system logs
- Detecting attacks
- Blocking attackers using a firewall

Fast, simple, secure, and protects a wide range of services out of the box!

Internet-connected hosts are subject to constant probing and attacks from malicious actors. Brute-force attacks represent a large threat to account compromise for hosts and their networks.

Blocking brute-force attackers at the firewall level reduces system load and log noise, especially for high-traffic and deeply embedded systems.

Brute-force attacks without SSHGuard

SSHGuard blocks brute-force attacks

# How Does SSHGuard Work?

## Log Ingestion

SSHGuard monitors system log files and journal logs. It can work with multiple log sources, including:

- cockpit
- Common Log Format
- macOS log
- metalog
- multilog
- raw log files
- syslog
- syslog-ng
- systemd journal

## Attack Detection

SSHGuard parses logs for recognized attacks. Unlike other brute-force blockers, SSHGuard's parser is:

- Fast. Attack signatures are compiled into a full lexical analyzer which does not slow down when more signatures are added.
- Sandboxed. It runs as a separate, unprivileged, and sandboxed (where supported) process.
- Secure. It is not susceptible to regular expression denial of service (ReDoS) attacks because signatures are compiled into a deterministic finite-state machine that does not exhibit unbounded behavior even with specially crafted inputs.

## Blocking

SSHGuard blocks repeat attackers using one of many firewall backends, including:

- [firewalld](http://www.firewalld.org/) (Linux)
- [netfilter/iptables](http://www.netfilter.org/) (Linux)
- [netfilter/ipset](http://ipset.netfilter.org/) (Linux)
- [ipfw](https://www.freebsd.org/doc/en_US.ISO8859-1/books/handbook/firewalls-ipfw.html) (FreeBSD, macOS)
- [PF](https://www.openbsd.org/faq/pf/) (OpenBSD, FreeBSD, NetBSD, DragonFly BSD)
- IPFILTER (FreeBSD, NetBSD, Solaris)
- hosts.allow (fallback)

SSHGuard doesn't lock you out. By default, attackers are unblocked after a certain amount of time. But if you chose, you can configure blacklisting to block attackers permanently.

## Functional spotlights

- Touchiness and automatic blacklisting
- Full IPv6 support
- Monitors multiple log files
- Small system footprint
- Sophisticated whitelisting
- Recognizes many logging formats transparently
- Handles host names or addresses in log files

## Non-functional spotlights

- Easy to set up, simple one-line command to use
- Written in small, portable C and Bourne shell with ~3000 LOC
- Simple, extensible firewall interface

Copyright ⓒ 2024. All rights reserved.
