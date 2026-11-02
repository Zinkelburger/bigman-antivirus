---
url: "https://attack.mitre.org/techniques/T1496/"
title: "Resource Hijacking, Technique T1496 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:43:14Z"
via: "live"
sha256: "d60e3e503c24edc11382c21aaf2f79b24d2fa7c808bb5f7c54b013a747a8f720"
wayback: "https://web.archive.org/web/20261023090833/https://attack.mitre.org/techniques/T1496/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Resource Hijacking, Technique T1496 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1496/>
> Snapshot: <https://web.archive.org/web/20261023090833/https://attack.mitre.org/techniques/T1496/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Resource Hijacking

#  Resource Hijacking

##### Sub-techniques (4)

| ID  | Name

|  [T1496.001](https://attack.mitre.org/techniques/T1496/001/)   |  [Compute Hijacking](https://attack.mitre.org/techniques/T1496/001/)

|  [T1496.002](https://attack.mitre.org/techniques/T1496/002/)   |  [Bandwidth Hijacking](https://attack.mitre.org/techniques/T1496/002/)

|  [T1496.003](https://attack.mitre.org/techniques/T1496/003/)   |  [SMS Pumping](https://attack.mitre.org/techniques/T1496/003/)

|  [T1496.004](https://attack.mitre.org/techniques/T1496/004/)   |  [Cloud Service Hijacking](https://attack.mitre.org/techniques/T1496/004/)

Adversaries may leverage the resources of co-opted systems to complete resource-intensive tasks, which may impact system and/or hosted service availability.

Resource hijacking may take a number of different forms. For example, adversaries may:

- Leverage compute resources in order to mine cryptocurrency
- Sell network bandwidth to proxy networks
- Generate SMS traffic for profit
- Abuse cloud-based messaging services to send large quantities of spam messages

In some cases, adversaries may leverage multiple types of Resource Hijacking at once.[[1]](https://sysdig.com/blog/labrat-cryptojacking-proxyjacking-campaign/)

ID: T1496

Sub-techniques:  [T1496.001](https://attack.mitre.org/techniques/T1496/001), [T1496.002](https://attack.mitre.org/techniques/T1496/002), [T1496.003](https://attack.mitre.org/techniques/T1496/003), [T1496.004](https://attack.mitre.org/techniques/T1496/004)

ⓘ

Tactic: [Impact](https://attack.mitre.org/tactics/TA0040)

ⓘ

Platforms: Containers, IaaS, Linux, SaaS, Windows, macOS

ⓘ

Impact Type: Availability

Contributors: Alfredo Oliveira, Trend Micro; David Fiser, @anu4is, Trend Micro; Jay Chen, Palo Alto Networks; Magno Logan, @magnologan, Trend Micro; Menachem Goldstein; Vishwas Manral, McAfee; Yossi Weizman, Azure Defender Research Team

Version: 2.0

Created: 17 April 2019

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1496/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1496/)

## Mitigations

This type of attack technique cannot be easily mitigated with preventive controls since it is based on the abuse of system features.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0267](https://attack.mitre.org/detectionstrategies/DET0267)   |  [Resource Hijacking Detection Strategy](https://attack.mitre.org/detectionstrategies/DET0267)   |  [AN0741](https://attack.mitre.org/detectionstrategies/DET0267#AN0741)   |

Persistent high CPU utilization combined with suspicious command-line execution (e.g., mining tools or obfuscated scripts) and outbound connections to mining/proxy networks.

|  [AN0742](https://attack.mitre.org/detectionstrategies/DET0267#AN0742)   |

Abnormal CPU/memory usage by unauthorized processes with outbound connections to known mining pools or using cron jobs/scripts to maintain persistence.

|  [AN0743](https://attack.mitre.org/detectionstrategies/DET0267#AN0743)   |

Background launch agents/daemons with high CPU use and network access to external mining services.

|  [AN0744](https://attack.mitre.org/detectionstrategies/DET0267#AN0744)   |

Sudden spikes in cloud VM CPU usage with outbound traffic to mining pools and unauthorized instance creation.

|  [AN0745](https://attack.mitre.org/detectionstrategies/DET0267#AN0745)   |

High CPU usage by unauthorized containers running mining binaries or public proxy tools.

|  [AN0746](https://attack.mitre.org/detectionstrategies/DET0267#AN0746)   |

Abuse of cloud messaging platforms to send mass spam or consume quota-based resources.

## References

-    [Miguel Hernandez. (2023, August 17). LABRAT: Stealthy Cryptojacking and Proxyjacking Campaign Targeting GitLab . Retrieved September 25, 2024.](https://sysdig.com/blog/labrat-cryptojacking-proxyjacking-campaign/)

×
