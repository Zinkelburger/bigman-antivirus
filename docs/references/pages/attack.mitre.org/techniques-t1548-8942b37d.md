---
url: "https://attack.mitre.org/techniques/T1548/"
title: "Abuse Elevation Control Mechanism, Technique T1548 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:50:02Z"
via: "live"
sha256: "779d9de0e0d5838bb455cfd8e42fe4d2237cf5e1fc223cf30c2b1503568a0bbc"
wayback: "https://web.archive.org/web/20261102192510/https://attack.mitre.org/techniques/T1548/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Abuse Elevation Control Mechanism, Technique T1548 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1548/>
> Snapshot: <https://web.archive.org/web/20261102192510/https://attack.mitre.org/techniques/T1548/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Abuse Elevation Control Mechanism

#  Abuse Elevation Control Mechanism

##### Sub-techniques (6)

| ID  | Name

|  [T1548.001](https://attack.mitre.org/techniques/T1548/001/)   |  [Setuid and Setgid](https://attack.mitre.org/techniques/T1548/001/)

|  [T1548.002](https://attack.mitre.org/techniques/T1548/002/)   |  [Bypass User Account Control](https://attack.mitre.org/techniques/T1548/002/)

|  [T1548.003](https://attack.mitre.org/techniques/T1548/003/)   |  [Sudo and Sudo Caching](https://attack.mitre.org/techniques/T1548/003/)

|  [T1548.004](https://attack.mitre.org/techniques/T1548/004/)   |  [Elevated Execution with Prompt](https://attack.mitre.org/techniques/T1548/004/)

|  [T1548.005](https://attack.mitre.org/techniques/T1548/005/)   |  [Temporary Elevated Cloud Access](https://attack.mitre.org/techniques/T1548/005/)

|  [T1548.006](https://attack.mitre.org/techniques/T1548/006/)   |  [TCC Manipulation](https://attack.mitre.org/techniques/T1548/006/)

Adversaries may circumvent mechanisms designed to control privilege elevation to gain higher-level permissions. Most modern systems contain native elevation control mechanisms that are intended to limit privileges that a user can perform on a machine. Authorization has to be granted to specific users in order to perform tasks that can be considered of higher risk.[[1]](https://technet.microsoft.com/en-us/itpro/windows/keep-secure/how-user-account-control-works)[[2]](https://www.sudo.ws/) An adversary can perform several methods to take advantage of built-in control mechanisms in order to escalate privileges on a system.[[3]](https://www.welivesecurity.com/2016/07/06/new-osxkeydnap-malware-hungry-credentials/)[[4]](https://blog.fortinet.com/2016/12/16/malicious-macro-bypasses-uac-to-elevate-privilege-for-fareit-malware)

ID: T1548

Sub-techniques:  [T1548.001](https://attack.mitre.org/techniques/T1548/001), [T1548.002](https://attack.mitre.org/techniques/T1548/002), [T1548.003](https://attack.mitre.org/techniques/T1548/003), [T1548.004](https://attack.mitre.org/techniques/T1548/004), [T1548.005](https://attack.mitre.org/techniques/T1548/005), [T1548.006](https://attack.mitre.org/techniques/T1548/006)

ⓘ

Tactic: [Privilege Escalation](https://attack.mitre.org/tactics/TA0004)

ⓘ

Platforms: IaaS, Identity Provider, Linux, Office Suite, Windows, macOS

Version: 2.0

Created: 30 January 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1548/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1548/)

## Procedure Examples

| ID  | Name  | Description

|  [S1130](https://attack.mitre.org/software/S1130)   |  [Raspberry Robin](https://attack.mitre.org/software/S1130)   |

[Raspberry Robin](https://attack.mitre.org/software/S1130) implements a variation of the ucmDccwCOMMethod technique abusing the Windows AutoElevate backdoor to bypass UAC while elevating privileges.[[5]](https://www.trendmicro.com/en_us/research/22/l/raspberry-robin-malware-targets-telecom-governments.html)

|  [G1048](https://attack.mitre.org/groups/G1048)   |  [UNC3886](https://attack.mitre.org/groups/G1048)   |

[UNC3886](https://attack.mitre.org/groups/G1048) has used vSphere Installation Bundles (VIBs) that contained modified descriptor XML files with the acceptance-level set to partner which allowed for privilege escalation.[[6]](https://cloud.google.com/blog/topics/threat-intelligence/esxi-hypervisors-malware-persistence)

## Mitigations

| ID  | Mitigation  | Description

|  [M1047](https://attack.mitre.org/mitigations/M1047)   |  [Audit](https://attack.mitre.org/mitigations/M1047)   |

Check for common UAC bypass weaknesses on Windows systems to be aware of the risk posture and address issues where appropriate.[[7]](https://github.com/hfiref0x/UACME)

|  [M1038](https://attack.mitre.org/mitigations/M1038)   |  [Execution Prevention](https://attack.mitre.org/mitigations/M1038)   |

System settings can prevent applications from running that haven't been downloaded from legitimate repositories which may help mitigate some of these issues. Not allowing unsigned applications from being run may also mitigate some risk.

|  [M1028](https://attack.mitre.org/mitigations/M1028)   |  [Operating System Configuration](https://attack.mitre.org/mitigations/M1028)   |

Applications with known vulnerabilities or known shell escapes should not have the setuid or setgid bits set to reduce potential damage if an application is compromised. Additionally, the number of programs with setuid or setgid bits set should be minimized across a system. Ensuring that the sudo tty_tickets setting is enabled will prevent this leakage across tty sessions.

|  [M1026](https://attack.mitre.org/mitigations/M1026)   |  [Privileged Account Management](https://attack.mitre.org/mitigations/M1026)   |

Remove users from the local administrator group on systems.

By requiring a password, even if an adversary can get terminal access, they must know the password to run anything in the sudoers file. Setting the timestamp_timeout to 0 will require the user to input their password every time sudo is executed.

|  [M1022](https://attack.mitre.org/mitigations/M1022)   |  [Restrict File and Directory Permissions](https://attack.mitre.org/mitigations/M1022)   |

The sudoers file should be strictly edited such that passwords are always required and that users can't spawn risky processes as users with higher privilege.

|  [M1051](https://attack.mitre.org/mitigations/M1051)   |  [Update Software](https://attack.mitre.org/mitigations/M1051)   |

Perform regular software updates to mitigate exploitation risk.

|  [M1052](https://attack.mitre.org/mitigations/M1052)   |  [User Account Control](https://attack.mitre.org/mitigations/M1052)   |

Although UAC bypass techniques exist, it is still prudent to use the highest enforcement level for UAC when possible and mitigate bypass opportunities that exist with techniques such as [DLL](https://attack.mitre.org/techniques/T1574/001).

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Limit the privileges of cloud accounts to assume, create, or impersonate additional roles, policies, and permissions to only those required. Where just-in-time access is enabled, consider requiring manual approval for temporary elevation of privileges.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0345](https://attack.mitre.org/detectionstrategies/DET0345)   |  [Detection Strategy for Abuse Elevation Control Mechanism (T1548)](https://attack.mitre.org/detectionstrategies/DET0345)   |  [AN0975](https://attack.mitre.org/detectionstrategies/DET0345#AN0975)   |

Correlate registry modifications (e.g., UAC bypass registry keys), unusual parent-child process relationships (e.g., control.exe spawning cmd.exe), and unsigned elevated process executions with non-standard tokens or elevation flags.

|  [AN0976](https://attack.mitre.org/detectionstrategies/DET0345#AN0976)   |

Monitor audit logs for setuid/setgid bit changes, executions where UID ≠ EUID (indicative of sudo or privilege escalation), and high-integrity binaries launched by unprivileged users.

|  [AN0977](https://attack.mitre.org/detectionstrategies/DET0345#AN0977)   |

Detect execution of /usr/libexec/security_authtrampoline or use of AuthorizationExecuteWithPrivileges API, and monitor process lineage for unusual launches of GUI apps with escalated privileges.

|  [AN0978](https://attack.mitre.org/detectionstrategies/DET0345#AN0978)   |

Monitor for unexpected privilege elevation operations via SAML assertion manipulation, role injection, or changes to identity mappings that result in access escalation.

|  [AN0979](https://attack.mitre.org/detectionstrategies/DET0345#AN0979)   |

Detect sudden privilege escalations such as IAM role changes, user-assigned privilege boundaries, or elevation via assumed roles beyond normal behavior.

## References

-    [Lich, B. (2016, May 31). How User Account Control Works. Retrieved June 3, 2016.](https://technet.microsoft.com/en-us/itpro/windows/keep-secure/how-user-account-control-works)
-    [Todd C. Miller. (2018). Sudo Man Page. Retrieved March 19, 2018.](https://www.sudo.ws/)
-    [Marc-Etienne M.Leveille. (2016, July 6). New OSX/Keydnap malware is hungry for credentials. Retrieved July 3, 2017.](https://www.welivesecurity.com/2016/07/06/new-osxkeydnap-malware-hungry-credentials/)
-    [Salvio, J., Joven, R. (2016, December 16). Malicious Macro Bypasses UAC to Elevate Privilege for Fareit Malware. Retrieved December 27, 2016.](https://blog.fortinet.com/2016/12/16/malicious-macro-bypasses-uac-to-elevate-privilege-for-fareit-malware)

-    [Christopher So. (2022, December 20). Raspberry Robin Malware Targets Telecom, Governments. Retrieved May 17, 2024.](https://www.trendmicro.com/en_us/research/22/l/raspberry-robin-malware-targets-telecom-governments.html)
-    [Alexander Marvi, Jeremy Koppen, Tufail Ahmed, and Jonathan Lepore. (2022, September 29). Bad VIB(E)s Part One: Investigating Novel Malware Persistence Within ESXi Hypervisors. Retrieved March 26, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/esxi-hypervisors-malware-persistence)
-    [UACME Project. (2016, June 16). UACMe. Retrieved July 26, 2016.](https://github.com/hfiref0x/UACME)

×
