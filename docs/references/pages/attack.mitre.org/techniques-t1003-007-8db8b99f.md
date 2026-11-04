---
url: "https://attack.mitre.org/techniques/T1003/007/"
title: "OS Credential Dumping: Proc Filesystem, Sub-technique T1003.007 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:24:00Z"
via: "live"
sha256: "4d07a06daa76dce82ea046400e331ca79edc2a9ee92e8eadf4792cf9bed2ce48"
wayback: "https://web.archive.org/web/20261023090825/https://attack.mitre.org/techniques/T1003/007/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# OS Credential Dumping: Proc Filesystem, Sub-technique T1003.007 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1003/007/>
> Snapshot: <https://web.archive.org/web/20261023090825/https://attack.mitre.org/techniques/T1003/007/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [OS Credential Dumping](https://attack.mitre.org/techniques/T1003)
- Proc Filesystem

#  OS Credential Dumping: Proc Filesystem

##### Other sub-techniques of OS Credential Dumping (8)

| ID  | Name

|  [T1003.001](https://attack.mitre.org/techniques/T1003/001/)   |  [LSASS Memory](https://attack.mitre.org/techniques/T1003/001/)

|  [T1003.002](https://attack.mitre.org/techniques/T1003/002/)   |  [Security Account Manager](https://attack.mitre.org/techniques/T1003/002/)

|  [T1003.003](https://attack.mitre.org/techniques/T1003/003/)   |  [NTDS](https://attack.mitre.org/techniques/T1003/003/)

|  [T1003.004](https://attack.mitre.org/techniques/T1003/004/)   |  [LSA Secrets](https://attack.mitre.org/techniques/T1003/004/)

|  [T1003.005](https://attack.mitre.org/techniques/T1003/005/)   |  [Cached Domain Credentials](https://attack.mitre.org/techniques/T1003/005/)

|  [T1003.006](https://attack.mitre.org/techniques/T1003/006/)   |  [DCSync](https://attack.mitre.org/techniques/T1003/006/)

|  T1003.007   |  Proc Filesystem

|  [T1003.008](https://attack.mitre.org/techniques/T1003/008/)   |  [/etc/passwd and /etc/shadow](https://attack.mitre.org/techniques/T1003/008/)

Adversaries may gather credentials from the proc filesystem or /proc. The proc filesystem is a pseudo-filesystem used as an interface to kernel data structures for Linux based systems managing virtual memory. For each process, the /proc/<PID>/maps file shows how memory is mapped within the process’s virtual address space. And /proc/<PID>/mem, exposed for debugging purposes, provides access to the process’s virtual address space.[[1]](https://www.picussecurity.com/resource/the-mitre-attck-t1003-os-credential-dumping-technique-and-its-adversary-use)[[2]](https://www.baeldung.com/linux/proc-id-maps)

When executing with root privileges, adversaries can search these memory locations for all processes on a system that contain patterns indicative of credentials. Adversaries may use regex patterns, such as grep -E "^[0-9a-f-]* r" /proc/"$pid"/maps | cut -d' ' -f 1, to look for fixed strings in memory structures or cached hashes.[[3]](https://github.com/redcanaryco/atomic-red-team/blob/master/atomics/T1003.007/T1003.007.md) When running without privileged access, processes can still view their own virtual memory locations. Some services or programs may save credentials in clear text inside the process’s memory.[[4]](https://github.com/huntergregal/mimipenguin)[[5]](https://book.hacktricks.xyz/linux-hardening/privilege-escalation#proc-usdpid-maps-and-proc-usdpid-mem)

If running as or with the permissions of a web browser, a process can search the /maps & /mem locations for common website credential patterns (that can also be used to find adjacent memory within the same structure) in which hashes or cleartext credentials may be located.

ID: T1003.007

Sub-technique of:  [T1003](https://attack.mitre.org/techniques/T1003)

ⓘ

Tactic: [Credential Access](https://attack.mitre.org/tactics/TA0006)

ⓘ

Platforms: Linux

Contributors: Tim (Wadhwa-)Brown

Version: 1.2

Created: 11 February 2020

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1003/007/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1003/007/)

## Procedure Examples

| ID  | Name  | Description

|  [S0349](https://attack.mitre.org/software/S0349)   |  [LaZagne](https://attack.mitre.org/software/S0349)   |

[LaZagne](https://attack.mitre.org/software/S0349) can use the <PID>/maps and <PID>/mem files to identify regex patterns to dump cleartext passwords from the browser's process memory.[[6]](https://github.com/AlessandroZ/LaZagne)[[1]](https://www.picussecurity.com/resource/the-mitre-attck-t1003-os-credential-dumping-technique-and-its-adversary-use)

|  [S0179](https://attack.mitre.org/software/S0179)   |  [MimiPenguin](https://attack.mitre.org/software/S0179)   |

[MimiPenguin](https://attack.mitre.org/software/S0179) can use the <PID>/maps and <PID>/mem file to search for regex patterns and dump the process memory.[[4]](https://github.com/huntergregal/mimipenguin)[[1]](https://www.picussecurity.com/resource/the-mitre-attck-t1003-os-credential-dumping-technique-and-its-adversary-use)

|  [S9043](https://attack.mitre.org/software/S9043)   |  [Mini Shai-Hulud](https://attack.mitre.org/software/S9043)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has scraped runner process memory to extract short-lived identity tokens, which it then exchanged for per-package npm trusted-publisher tokens.[[7]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  [S1109](https://attack.mitre.org/software/S1109)   |  [PACEMAKER](https://attack.mitre.org/software/S1109)   |

[PACEMAKER](https://attack.mitre.org/software/S1109) has the ability to extract credentials from OS memory.[[8]](https://www.mandiant.com/resources/blog/suspected-apt-actors-leverage-bypass-techniques-pulse-secure-zero-day)

|  [S9041](https://attack.mitre.org/software/S9041)   |  [TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041)   |

[TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041) can scrape memory from the Runner.Worker process by reading /proc/<pid>/mem to extract secrets including plaintext tokens.[[9]](https://www.wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack)[[10]](https://github.com/aquasecurity/trivy/security/advisories/GHSA-69fq-xp46-6x23)[[11]](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)[[12]](https://www.sysdig.com/blog/teampcp-expands-supply-chain-compromise-spreads-from-trivy-to-checkmarx-github-actions)[[13]](https://www.wiz.io/blog/teampcp-attack-kics-github-action)[[14]](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1027](https://attack.mitre.org/mitigations/M1027)   |  [Password Policies](https://attack.mitre.org/mitigations/M1027)   |

Ensure that root accounts have complex, unique passwords across all systems on the network.

|  [M1026](https://attack.mitre.org/mitigations/M1026)   |  [Privileged Account Management](https://attack.mitre.org/mitigations/M1026)   |

Follow best practices in restricting access to privileged accounts to avoid hostile programs from accessing sensitive information.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0593](https://attack.mitre.org/detectionstrategies/DET0593)   |  [Detecting OS Credential Dumping via /proc Filesystem Access on Linux](https://attack.mitre.org/detectionstrategies/DET0593)   |  [AN1631](https://attack.mitre.org/detectionstrategies/DET0593#AN1631)   |

Monitoring adversary access to sensitive process memory via the /proc filesystem to extract credential material, often involving multi-step access to /proc/[pid]/mem or /proc/[pid]/maps combined with privilege escalation or credential scraping binaries.

## References

-    [Huseyin Can YUCEEL & Picus Labs. (2022, March 22). Retrieved March 31, 2023.](https://www.picussecurity.com/resource/the-mitre-attck-t1003-os-credential-dumping-technique-and-its-adversary-use)
-    [baeldung. (2022, April 8). Understanding the Linux /proc/id/maps File. Retrieved March 31, 2023.](https://www.baeldung.com/linux/proc-id-maps)
-    [Atomic Red Team. (2023, November). T1003.007 - OS Credential Dumping: Proc Filesystem. Retrieved March 28, 2024.](https://github.com/redcanaryco/atomic-red-team/blob/master/atomics/T1003.007/T1003.007.md)
-    [Gregal, H. (2017, May 12). MimiPenguin. Retrieved December 5, 2017.](https://github.com/huntergregal/mimipenguin)
-    [Carlos Polop. (2023, March 5). Linux Privilege Escalation. Retrieved March 31, 2023.](https://book.hacktricks.xyz/linux-hardening/privilege-escalation#proc-usdpid-maps-and-proc-usdpid-mem)
-    [Zanni, A. (n.d.). The LaZagne Project !!!. Retrieved December 14, 2018.](https://github.com/AlessandroZ/LaZagne)
-    [Flashpoint. (2026, May 28). The Mini Shai-Hulud Worm and the New Era of CI/CD Exploitation. Retrieved July 16, 2026.](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

-    [Perez, D. et al. (2021, April 20). Check Your Pulse: Suspected APT Actors Leverage Authentication Bypass Techniques and Pulse Secure Zero-Day. Retrieved February 5, 2024.](https://www.mandiant.com/resources/blog/suspected-apt-actors-leverage-bypass-techniques-pulse-secure-zero-day)
-    [McCarthy, R. (2026, March 20). Trivy Compromised: Everything You Need to Know about the Latest Supply Chain Attack. Retrieved July 1, 2026.](https://www.wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack)
-    [Aqua Security . (2026, March 21). Trivy ecosystem supply chain temporarily compromised. Retrieved July 1, 2026.](https://github.com/aquasecurity/trivy/security/advisories/GHSA-69fq-xp46-6x23)
-    [Aqua Team. (2026, April 1). Update: Ongoing Investigation and Continued Remediation. Retrieved July 1, 2026.](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)
-    [Sysdig Threat Research Team. (2026, March 23). TeamPCP expands: Supply chain compromise spreads from Trivy to Checkmarx GitHub Actions. Retrieved July 1, 2026.](https://www.sysdig.com/blog/teampcp-expands-supply-chain-compromise-spreads-from-trivy-to-checkmarx-github-actions)
-    [McCarthy, R., Haughom, J., Read, B. (2026, March 23). KICS GitHub Action Compromised: TeamPCP Strikes Again in Supply Chain Attack. Retrieved July 1, 2026.](https://www.wiz.io/blog/teampcp-attack-kics-github-action)
-    [Unit 42. (2026, March 31). Weaponizing the Protectors: TeamPCP’s Multi-Stage Supply Chain Attack on Security Infrastructure. Retrieved July 1, 2026.](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)

×
