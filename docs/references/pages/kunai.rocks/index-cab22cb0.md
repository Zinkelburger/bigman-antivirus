---
url: "https://kunai.rocks/"
title: "Bring your Linux Threat-Hunting capabilities to the next level | Kunai"
captured: "2026-11-02T18:33:43Z"
via: "live"
source_url: "https://why.kunai.rocks/"
sha256: "bf8726829cfec1785aaa674d4df324849910aa244fd4f935705c9ded5d3c5b29"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# Bring your Linux Threat-Hunting capabilities to the next level | Kunai

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://kunai.rocks/>

## Built For

### Security Monitoring

Feed your SIEM with high quality Linux events.

### Threat Hunting

Explore what really happens on your Linux hosts.

### Incident Response

Reconstruct process trees and attacker activity.

### Detection Engineering

Write and test rules against real events.

## What You Get

### [Rich Security Events](https://why.kunai.rocks/docs/events)

Process execution, file activity, network connections, DNS queries, BPF program loads, module loading, mmap/mprotect with exec rights and more.

### [Context Out of the Box](https://why.kunai.rocks/docs/events/generalities)

Every event comes with process ancestry, parent command line, executable hashes and container information. No post-processing needed.

### [Detection & Filtering Rules](https://why.kunai.rocks/docs/advanced/rule_configuration)

Write simple YAML rules to tag detections with MITRE ATT&CK ids and severity, or to filter out noise right on the host.

### [IoCs, YARA & Actions](https://why.kunai.rocks/docs/advanced/ioc_configuration)

Match indicators of compromise live, scan files with YARA and trigger actions such as killing a malicious process.

### [Container Aware](https://why.kunai.rocks/docs/events/generalities)

Monitor activity inside your containers and apply all your threat-hunting rules to them seamlessly.

### [Powered by Rust and eBPF](https://why.kunai.rocks/docs/compatibility)

Built with the [Aya](https://github.com/aya-rs) library: low overhead, memory safe and running on a wide range of kernels.

## How It Works

Linux KerneleBPF probes capture security relevant activity

→

KunaiReorders, enriches, filters and detects

→

JSON EventsOne event per line, easy to parse

→

Your Stackstdout, file, SIEM, threat-hunting tools

## Detect What Matters

Detection rules are simple YAML files matching on any event field. This one catches binaries masquerading as kernel threads, a technique sometimes used by malware to hide itself.

[Write your first rule](https://why.kunai.rocks/docs/advanced/rule_configuration)

mimic.kthread.yaml

```
name: mimic.kthread
type: detection
meta:
    tags: [ 'os:linux' ]
    attack: [ T1036 ]
match-on:
    events:
        kunai: [execve, execve_script]
matches:
    $task_is_kthread: .info.task.flags &= '0x200000'
    $kthread_names: .info.task.name ~= '^(kworker)'
condition: not $task_is_kthread and $kthread_names
severity: 10

```

## Ecosystem

[RulezetFind and share community Kunai detection rules.](https://rulezet.org/)[Kunai SandboxRun Linux samples in a VM monitored by Kunai to build detection rules.](https://sandbox.kunai.rocks)[GitHubSource code of Kunai and its related projects.](https://github.com/kunai-project)
