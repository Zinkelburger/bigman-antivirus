---
url: "https://attack.mitre.org/techniques/T1685/004/"
title: "Disable or Modify Tools: Disable or Modify Linux Audit System Log, Sub-technique T1685.004 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T19:04:42Z"
via: "live"
sha256: "d5804c573435e4c3f616af93294472de6a11d1ad1b862fb82d098259907b332b"
wayback: "https://web.archive.org/web/20261023090819/https://attack.mitre.org/techniques/T1685/004/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Disable or Modify Tools: Disable or Modify Linux Audit System Log, Sub-technique T1685.004 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1685/004/>
> Snapshot: <https://web.archive.org/web/20261023090819/https://attack.mitre.org/techniques/T1685/004/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Disable or Modify Tools](https://attack.mitre.org/techniques/T1685)
- Disable or Modify Linux Audit System Log

#  Disable or Modify Tools: Disable or Modify Linux Audit System Log

##### Other sub-techniques of Disable or Modify Tools (7)

| ID  | Name

|  [T1685.001](https://attack.mitre.org/techniques/T1685/001/)   |  [Disable or Modify Windows Event Log](https://attack.mitre.org/techniques/T1685/001/)

|  [T1685.002](https://attack.mitre.org/techniques/T1685/002/)   |  [Disable or Modify Cloud Log](https://attack.mitre.org/techniques/T1685/002/)

|  [T1685.003](https://attack.mitre.org/techniques/T1685/003/)   |  [Modify or Spoof Tool UI](https://attack.mitre.org/techniques/T1685/003/)

|  T1685.004   |  Disable or Modify Linux Audit System Log

|  [T1685.005](https://attack.mitre.org/techniques/T1685/005/)   |  [Clear Windows Event Logs](https://attack.mitre.org/techniques/T1685/005/)

|  [T1685.006](https://attack.mitre.org/techniques/T1685/006/)   |  [Clear Linux or Mac System Logs](https://attack.mitre.org/techniques/T1685/006/)

|  [T1685.007](https://attack.mitre.org/techniques/T1685/007/)   |  [Impair Autonomous Defenses](https://attack.mitre.org/techniques/T1685/007/)

Adversaries may disable or modify the Linux Audit system to hide malicious activity and avoid detection. Linux admins use the Linux Audit system to track security-relevant information on a system. The Linux Audit system operates at the kernel-level and maintains event logs on application and system activity such as process, network, file, and login events based on pre-configured rules.

Often referred to as auditd, this is the name of the daemon used to write events to disk and is governed by the parameters set in the audit.conf configuration file. Two primary ways to configure the log generation rules are through the command line auditctl utility and the file /etc/audit/audit.rules, containing a sequence of auditctl commands loaded at boot time.[[1]](https://izyknows.medium.com/linux-auditd-for-threat-detection-d06c8b941505)[[2]](https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/6/html/security_guide/chap-system_auditing)

With root privileges, adversaries may be able to ensure their activity is not logged through disabling the Audit system service, editing the configuration/rule files, or by hooking the Audit system library functions. Using the command line, adversaries can disable the Audit system service through killing processes associated with auditd daemon or use systemctl to stop the Audit service. Adversaries can also hook Audit system functions to disable logging or modify the rules contained in the /etc/audit/audit.rules or audit.conf files to ignore malicious activity.[[3]](https://www.welivesecurity.com/2014/02/21/an-in-depth-analysis-of-linuxebury/)

ID: T1685.004

Sub-technique of:  [T1685](https://attack.mitre.org/techniques/T1685)

ⓘ

Tactic: [Defense Impairment](https://attack.mitre.org/tactics/TA0112)

ⓘ

Platforms: Linux

Contributors: Tim (Wadhwa-)Brown

Version: 1.0

Created: 14 April 2026

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1685/004/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1685/004/)

## Procedure Examples

| ID  | Name  | Description

|  [S0377](https://attack.mitre.org/software/S0377)   |  [Ebury](https://attack.mitre.org/software/S0377)   |

[Ebury](https://attack.mitre.org/software/S0377) disables OpenSSH, system (systemd), and audit logs (/sbin/auditd) when the backdoor is active.[[4]](https://web-assets.esetstatic.com/wls/en/papers/white-papers/ebury-is-alive-but-unseen.pdf)

## Mitigations

| ID  | Mitigation  | Description

|  [M1047](https://attack.mitre.org/mitigations/M1047)   |  [Audit](https://attack.mitre.org/mitigations/M1047)   |

Routinely check account role permissions to ensure only expected users and roles have permission to modify logging settings.

To ensure Audit rules can not be modified at runtime, add the auditctl -e 2 as the last command in the audit.rules files. Once started, any attempt to change the configuration in this mode will be audited and denied. The configuration can only be changed by rebooting the machine.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

An adversary must already have root level access on the local system to make full use of this technique; be sure to restrict users and accounts to the least privileges they require.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0062](https://attack.mitre.org/detectionstrategies/DET0062)   |  [Detection Strategy for Disable or Modify Linux Audit System Log](https://attack.mitre.org/detectionstrategies/DET0062)   |  [AN0171](https://attack.mitre.org/detectionstrategies/DET0062#AN0171)   |

Disabling or modifying the Linux Audit system through process termination (auditd killed), service management (systemctl stop auditd), or tampering with rule/configuration files (/etc/audit/audit.rules, audit.conf). Defender view: suspicious execution of auditctl/systemctl commands, file modifications to audit rules, or sudden absence of audit logs correlated with privileged execution.

## References

-    [IzySec. (2022, January 26). Linux auditd for Threat Detection. Retrieved September 29, 2023.](https://izyknows.medium.com/linux-auditd-for-threat-detection-d06c8b941505)
-    [Red Hat. (n.d.). Retrieved April 15, 2026.](https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/6/html/security_guide/chap-system_auditing)

-    [M.Léveillé, M.. (2014, February 21). An In-depth Analysis of Linux/Ebury. Retrieved April 19, 2019.](https://www.welivesecurity.com/2014/02/21/an-in-depth-analysis-of-linuxebury/)
-    [Marc-Etienne M.Léveillé. (2024, May 1). Ebury is alive but unseen. Retrieved May 21, 2024.](https://web-assets.esetstatic.com/wls/en/papers/white-papers/ebury-is-alive-but-unseen.pdf)

×
