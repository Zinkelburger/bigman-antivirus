---
url: "https://attack.mitre.org/techniques/T1546/017/"
title: "Event Triggered Execution: Udev Rules, Sub-technique T1546.017 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:48:55Z"
via: "live"
sha256: "402e32eb213eab6c35ddea9c03ed83db636f3ff8571352ae215c7a451973fcb3"
wayback: "https://web.archive.org/web/20261023090808/https://attack.mitre.org/techniques/T1546/017/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
  - "docs/threats/05-malware-families-and-detection.md"
---

# Event Triggered Execution: Udev Rules, Sub-technique T1546.017 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1546/017/>
> Snapshot: <https://web.archive.org/web/20261023090808/https://attack.mitre.org/techniques/T1546/017/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Event Triggered Execution](https://attack.mitre.org/techniques/T1546)
- Udev Rules

#  Event Triggered Execution: Udev Rules

##### Other sub-techniques of Event Triggered Execution (18)

| ID  | Name

|  [T1546.001](https://attack.mitre.org/techniques/T1546/001/)   |  [Change Default File Association](https://attack.mitre.org/techniques/T1546/001/)

|  [T1546.002](https://attack.mitre.org/techniques/T1546/002/)   |  [Screensaver](https://attack.mitre.org/techniques/T1546/002/)

|  [T1546.003](https://attack.mitre.org/techniques/T1546/003/)   |  [Windows Management Instrumentation Event Subscription](https://attack.mitre.org/techniques/T1546/003/)

|  [T1546.004](https://attack.mitre.org/techniques/T1546/004/)   |  [Unix Shell Configuration Modification](https://attack.mitre.org/techniques/T1546/004/)

|  [T1546.005](https://attack.mitre.org/techniques/T1546/005/)   |  [Trap](https://attack.mitre.org/techniques/T1546/005/)

|  [T1546.006](https://attack.mitre.org/techniques/T1546/006/)   |  [LC_LOAD_DYLIB Addition](https://attack.mitre.org/techniques/T1546/006/)

|  [T1546.007](https://attack.mitre.org/techniques/T1546/007/)   |  [Netsh Helper DLL](https://attack.mitre.org/techniques/T1546/007/)

|  [T1546.008](https://attack.mitre.org/techniques/T1546/008/)   |  [Accessibility Features](https://attack.mitre.org/techniques/T1546/008/)

|  [T1546.009](https://attack.mitre.org/techniques/T1546/009/)   |  [AppCert DLLs](https://attack.mitre.org/techniques/T1546/009/)

|  [T1546.010](https://attack.mitre.org/techniques/T1546/010/)   |  [AppInit DLLs](https://attack.mitre.org/techniques/T1546/010/)

|  [T1546.011](https://attack.mitre.org/techniques/T1546/011/)   |  [Application Shimming](https://attack.mitre.org/techniques/T1546/011/)

|  [T1546.012](https://attack.mitre.org/techniques/T1546/012/)   |  [Image File Execution Options Injection](https://attack.mitre.org/techniques/T1546/012/)

|  [T1546.013](https://attack.mitre.org/techniques/T1546/013/)   |  [PowerShell Profile](https://attack.mitre.org/techniques/T1546/013/)

|  [T1546.014](https://attack.mitre.org/techniques/T1546/014/)   |  [Emond](https://attack.mitre.org/techniques/T1546/014/)

|  [T1546.015](https://attack.mitre.org/techniques/T1546/015/)   |  [Component Object Model Hijacking](https://attack.mitre.org/techniques/T1546/015/)

|  [T1546.016](https://attack.mitre.org/techniques/T1546/016/)   |  [Installer Packages](https://attack.mitre.org/techniques/T1546/016/)

|  T1546.017   |  Udev Rules

|  [T1546.018](https://attack.mitre.org/techniques/T1546/018/)   |  [Python Startup Hooks](https://attack.mitre.org/techniques/T1546/018/)

Adversaries may maintain persistence through executing malicious content triggered using udev rules. Udev is the Linux kernel device manager that dynamically manages device nodes, handles access to pseudo-device files in the /dev directory, and responds to hardware events, such as when external devices like hard drives or keyboards are plugged in or removed. Udev uses rule files with match keys to specify the conditions a hardware event must meet and action keys to define the actions that should follow. Root permissions are required to create, modify, or delete rule files located in /etc/udev/rules.d/, /run/udev/rules.d/, /usr/lib/udev/rules.d/, /usr/local/lib/udev/rules.d/, and /lib/udev/rules.d/. Rule priority is determined by both directory and by the digit prefix in the rule filename.[[1]](https://ch4ik0.github.io/en/posts/leveraging-Linux-udev-for-persistence/)[[2]](https://www.elastic.co/security-labs/sequel-on-persistence-mechanisms)

Adversaries may abuse the udev subsystem by adding or modifying rules in udev rule files to execute malicious content. For example, an adversary may configure a rule to execute their binary each time the pseudo-device file, such as /dev/random, is accessed by an application. Although udev is limited to running short tasks and is restricted by systemd-udevd's sandbox (blocking network and filesystem access), attackers may use scripting commands under the action key RUN+= to detach and run the malicious content’s process in the background to bypass these controls.[[3]](https://www.aon.com/en/insights/cyber-labs/unveiling-sedexp)

ID: T1546.017

Sub-technique of:  [T1546](https://attack.mitre.org/techniques/T1546)

ⓘ

Tactics: [Persistence](https://attack.mitre.org/tactics/TA0003), [Privilege Escalation](https://attack.mitre.org/tactics/TA0004)

ⓘ

Platforms: Linux

Contributors: @grahamhelton3; Eder Pérez Ignacio, @ch4ik0; Eduardo González Hernández (@codexlynx); Ruben Groenewoud (@RFGroenewoud); Wirapong Petshagun

Version: 1.0

Created: 26 September 2024

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1546/017/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1546/017/)

## Procedure Examples

| ID  | Name  | Description

|  [S1219](https://attack.mitre.org/software/S1219)   |  [REPTILE](https://attack.mitre.org/software/S1219)   |

[REPTILE](https://attack.mitre.org/software/S1219) has used udev for persistence.[[4]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

## Mitigations

This type of attack technique cannot be easily mitigated with preventive controls since it is based on the abuse of system features.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0375](https://attack.mitre.org/detectionstrategies/DET0375)   |  [Detection Strategy for T1546.017 - Udev Rules (Linux)](https://attack.mitre.org/detectionstrategies/DET0375)   |  [AN1056](https://attack.mitre.org/detectionstrategies/DET0375#AN1056)   |

Monitor for creation or modification of udev rules files in key directories (/etc/udev/rules.d/, /lib/udev/rules.d/, /usr/lib/udev/rules.d/). Look for RUN+= or IMPORT keys invoking suspicious binaries or scripts. Correlate this with process execution from systemd-udevd context, and file writes near udev reload/restart events. Combine this with unexpected background process spawning from udevd-related forks.

## References

-    [Eder P. Ignacio. (2024, February 21). Leveraging Linux udev for persistence. Retrieved September 26, 2024.](https://ch4ik0.github.io/en/posts/leveraging-Linux-udev-for-persistence/)
-    [Ruben Groenewoud. (2024, August 29). Linux Detection Engineering - A Sequel on Persistence Mechanisms. Retrieved October 16, 2024.](https://www.elastic.co/security-labs/sequel-on-persistence-mechanisms)

-    [Zachary Reichert. (2024, August 19). Unveiling "sedexp": A Stealthy Linux Malware Exploiting udev Rules. Retrieved September 26, 2024.](https://www.aon.com/en/insights/cyber-labs/unveiling-sedexp)
-    [Punsaen Boonyakarn, Shawn Chew, Logeswaran Nadarajan, Mathew Potaczek, Jakub Jozwiak, and Alex Marvi. (2024, June 18). Cloaked and Covert: Uncovering UNC3886 Espionage Operations. Retrieved September 24, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

×
