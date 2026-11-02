---
url: "https://attack.mitre.org/techniques/T1547/013/"
title: "Boot or Logon Autostart Execution: XDG Autostart Entries, Sub-technique T1547.013 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:49:05Z"
via: "live"
sha256: "e4b8fde09eddb11efb396e5b02fc0aff55d20beaf4f0e60c324fe3ad3b50070c"
wayback: "https://web.archive.org/web/20261023090749/https://attack.mitre.org/techniques/T1547/013/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
  - "docs/threats/05-malware-families-and-detection.md"
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Boot or Logon Autostart Execution: XDG Autostart Entries, Sub-technique T1547.013 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1547/013/>
> Snapshot: <https://web.archive.org/web/20261023090749/https://attack.mitre.org/techniques/T1547/013/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Boot or Logon Autostart Execution](https://attack.mitre.org/techniques/T1547)
- XDG Autostart Entries

#  Boot or Logon Autostart Execution: XDG Autostart Entries

##### Other sub-techniques of Boot or Logon Autostart Execution (14)

| ID  | Name

|  [T1547.001](https://attack.mitre.org/techniques/T1547/001/)   |  [Registry Run Keys / Startup Folder](https://attack.mitre.org/techniques/T1547/001/)

|  [T1547.002](https://attack.mitre.org/techniques/T1547/002/)   |  [Authentication Package](https://attack.mitre.org/techniques/T1547/002/)

|  [T1547.003](https://attack.mitre.org/techniques/T1547/003/)   |  [Time Providers](https://attack.mitre.org/techniques/T1547/003/)

|  [T1547.004](https://attack.mitre.org/techniques/T1547/004/)   |  [Winlogon Helper DLL](https://attack.mitre.org/techniques/T1547/004/)

|  [T1547.005](https://attack.mitre.org/techniques/T1547/005/)   |  [Security Support Provider](https://attack.mitre.org/techniques/T1547/005/)

|  [T1547.006](https://attack.mitre.org/techniques/T1547/006/)   |  [Kernel Modules and Extensions](https://attack.mitre.org/techniques/T1547/006/)

|  [T1547.007](https://attack.mitre.org/techniques/T1547/007/)   |  [Re-opened Applications](https://attack.mitre.org/techniques/T1547/007/)

|  [T1547.008](https://attack.mitre.org/techniques/T1547/008/)   |  [LSASS Driver](https://attack.mitre.org/techniques/T1547/008/)

|  [T1547.009](https://attack.mitre.org/techniques/T1547/009/)   |  [Shortcut Modification](https://attack.mitre.org/techniques/T1547/009/)

|  [T1547.010](https://attack.mitre.org/techniques/T1547/010/)   |  [Port Monitors](https://attack.mitre.org/techniques/T1547/010/)

|  [T1547.012](https://attack.mitre.org/techniques/T1547/012/)   |  [Print Processors](https://attack.mitre.org/techniques/T1547/012/)

|  T1547.013   |  XDG Autostart Entries

|  [T1547.014](https://attack.mitre.org/techniques/T1547/014/)   |  [Active Setup](https://attack.mitre.org/techniques/T1547/014/)

|  [T1547.015](https://attack.mitre.org/techniques/T1547/015/)   |  [Login Items](https://attack.mitre.org/techniques/T1547/015/)

Adversaries may add or modify XDG Autostart Entries to execute malicious programs or commands when a user’s desktop environment is loaded at login. XDG Autostart entries are available for any XDG-compliant Linux system. XDG Autostart entries use Desktop Entry files (.desktop) to configure the user’s desktop environment upon user login. These configuration files determine what applications launch upon user login, define associated applications to open specific file types, and define applications used to open removable media.[[1]](https://specifications.freedesktop.org/autostart-spec/autostart-spec-latest.html)[[2]](https://specifications.freedesktop.org/desktop-entry-spec/latest/recognized-keys.html)

Adversaries may abuse this feature to establish persistence by adding a path to a malicious binary or command to the Exec directive in the .desktop configuration file. When the user’s desktop environment is loaded at user login, the .desktop files located in the XDG Autostart directories are automatically executed. System-wide Autostart entries are located in the /etc/xdg/autostart directory while the user entries are located in the ~/.config/autostart directory.

Adversaries may combine this technique with [Masquerading](https://attack.mitre.org/techniques/T1036) to blend malicious Autostart entries with legitimate programs.[[3]](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)

ID: T1547.013

Sub-technique of:  [T1547](https://attack.mitre.org/techniques/T1547)

ⓘ

Tactics: [Persistence](https://attack.mitre.org/tactics/TA0003), [Privilege Escalation](https://attack.mitre.org/tactics/TA0004)

ⓘ

Platforms: Linux

Contributors: Tony Lambert, Red Canary

Version: 1.2

Created: 10 September 2019

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1547/013/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1547/013/)

## Procedure Examples

| ID  | Name  | Description

|  [G1052](https://attack.mitre.org/groups/G1052)   |  [Contagious Interview](https://attack.mitre.org/groups/G1052)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has established persistence using [InvisibleFerret](https://attack.mitre.org/software/S1245) malware to create a .desktop entry to run on startup on GNOME-based Linux devices.[[4]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  [S0235](https://attack.mitre.org/software/S0235)   |  [CrossRAT](https://attack.mitre.org/software/S0235)   |

[CrossRAT](https://attack.mitre.org/software/S0235) can use an XDG Autostart to establish persistence.[[3]](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)

|  [S0410](https://attack.mitre.org/software/S0410)   |  [Fysbis](https://attack.mitre.org/software/S0410)   |

If executing without root privileges, [Fysbis](https://attack.mitre.org/software/S0410) adds a .desktop configuration file to the user's ~/.config/autostart directory.[[3]](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)[[5]](https://vms.drweb.com/virus/?i=4276269)

|  [S1245](https://attack.mitre.org/software/S1245)   |  [InvisibleFerret](https://attack.mitre.org/software/S1245)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has established persistence within GNOME-based Linux environments by placing entries within .desktop that run on Startup.[[4]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  [S0198](https://attack.mitre.org/software/S0198)   |  [NETWIRE](https://attack.mitre.org/software/S0198)   |

[NETWIRE](https://attack.mitre.org/software/S0198) can use XDG Autostart Entries to establish persistence on Linux systems.[[6]](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)

|  [S0192](https://attack.mitre.org/software/S0192)   |  [Pupy](https://attack.mitre.org/software/S0192)   |

[Pupy](https://attack.mitre.org/software/S0192) can use an XDG Autostart to establish persistence.[[3]](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)

|  [S1078](https://attack.mitre.org/software/S1078)   |  [RotaJakiro](https://attack.mitre.org/software/S1078)   |

When executing with user-level permissions, [RotaJakiro](https://attack.mitre.org/software/S1078) can install persistence using a .desktop file under the $HOME/.config/autostart/ folder.[[7]](https://blog.netlab.360.com/stealth_rotajakiro_backdoor_en/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1033](https://attack.mitre.org/mitigations/M1033)   |  [Limit Software Installation](https://attack.mitre.org/mitigations/M1033)   |

Restrict software installation to trusted repositories only and be cautious of orphaned software packages.

|  [M1022](https://attack.mitre.org/mitigations/M1022)   |  [Restrict File and Directory Permissions](https://attack.mitre.org/mitigations/M1022)   |

Restrict write access to XDG autostart entries to only select privileged users.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Limit privileges of user accounts so only authorized privileged users can create and modify XDG autostart entries.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0390](https://attack.mitre.org/detectionstrategies/DET0390)   |  [Linux Detection Strategy for T1547.013 - XDG Autostart Entries](https://attack.mitre.org/detectionstrategies/DET0390)   |  [AN1096](https://attack.mitre.org/detectionstrategies/DET0390#AN1096)   |

Correlation of file creation/modification of .desktop files within XDG autostart directories, followed by execution of processes at user login initiated by the desktop environment. Malicious entries typically include suspicious Exec paths or anomalous names and are not associated with installed packages.

## References

-    [Free Desktop. (2006, February 13). Desktop Application Autostart Specification. Retrieved September 12, 2019.](https://specifications.freedesktop.org/autostart-spec/autostart-spec-latest.html)
-    [Free Desktop. (2017, December 24). Recognized Desktop Entry Keys. Retrieved November 17, 2024.](https://specifications.freedesktop.org/desktop-entry-spec/latest/recognized-keys.html)
-    [TONY LAMBERT. (2022, June 7). Trapping the Netwire RAT on Linux. Retrieved September 28, 2023.](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)
-    [Seongsu Park. (2024, November 4). From Pyongyang to Your Payroll: The Rise of North Korean Remote Workers in the West. Retrieved October 17, 2025.](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

-    [Doctor Web. (2014, November 21). Linux.BackDoor.Fysbis.1. Retrieved December 7, 2017.](https://vms.drweb.com/virus/?i=4276269)
-    [Lambert, T. (2020, January 29). Intro to Netwire. Retrieved January 7, 2021.](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)
-    [Alex Turing, Hui Wang. (2021, April 28). RotaJakiro: A long live secret backdoor with 0 VT detection. Retrieved June 14, 2023.](https://blog.netlab.360.com/stealth_rotajakiro_backdoor_en/)

×
