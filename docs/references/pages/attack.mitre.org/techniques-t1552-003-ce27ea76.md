---
url: "https://attack.mitre.org/techniques/T1552/003/"
title: "Unsecured Credentials: Shell History, Sub-technique T1552.003 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:52:29Z"
via: "live"
sha256: "46b963309f0a67820bb5d813a71143df296b504f7ceec3ff35ed8477c7acb387"
wayback: "https://web.archive.org/web/20261023090826/https://attack.mitre.org/techniques/T1552/003/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Unsecured Credentials: Shell History, Sub-technique T1552.003 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1552/003/>
> Snapshot: <https://web.archive.org/web/20261023090826/https://attack.mitre.org/techniques/T1552/003/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Unsecured Credentials](https://attack.mitre.org/techniques/T1552)
- Shell History

#  Unsecured Credentials: Shell History

##### Other sub-techniques of Unsecured Credentials (8)

| ID  | Name

|  [T1552.001](https://attack.mitre.org/techniques/T1552/001/)   |  [Credentials In Files](https://attack.mitre.org/techniques/T1552/001/)

|  [T1552.002](https://attack.mitre.org/techniques/T1552/002/)   |  [Credentials in Registry](https://attack.mitre.org/techniques/T1552/002/)

|  T1552.003   |  Shell History

|  [T1552.004](https://attack.mitre.org/techniques/T1552/004/)   |  [Private Keys](https://attack.mitre.org/techniques/T1552/004/)

|  [T1552.005](https://attack.mitre.org/techniques/T1552/005/)   |  [Cloud Instance Metadata API](https://attack.mitre.org/techniques/T1552/005/)

|  [T1552.006](https://attack.mitre.org/techniques/T1552/006/)   |  [Group Policy Preferences](https://attack.mitre.org/techniques/T1552/006/)

|  [T1552.007](https://attack.mitre.org/techniques/T1552/007/)   |  [Container API](https://attack.mitre.org/techniques/T1552/007/)

|  [T1552.008](https://attack.mitre.org/techniques/T1552/008/)   |  [Chat Messages](https://attack.mitre.org/techniques/T1552/008/)

Adversaries may search the command history on compromised systems for insecurely stored credentials.

On Linux and macOS systems, shells such as Bash and Zsh keep track of the commands users type on the command-line with the "history" utility. Once a user logs out, the history is flushed to the user's history file. For each user, this file resides at the same location: for example, ~/.bash_history or ~/.zsh_history. Typically, these files keeps track of the user's last 1000 commands.

On Windows, PowerShell has both a command history that is wiped after the session ends, and one that contains commands used in all sessions and is persistent. The default location for persistent history can be found in %userprofile%\AppData\Roaming\Microsoft\Windows\PowerShell\PSReadline\ConsoleHost_history.txt, but command history can also be accessed with Get-History. Command Prompt (CMD) on Windows does not have persistent history.[[1]](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_history?view=powershell-7.5)[[2]](https://michaelkoczwara.medium.com/windows-privilege-escalation-dbb908cce8d4)

Users often type usernames and passwords on the command-line as parameters to programs, which then get saved to this file when they log out. Adversaries can abuse this by looking through the file for potential credentials.[[3]](https://www.slideshare.net/slideshow/external-to-da-the-os-x-way/62021418)

ID: T1552.003

Sub-technique of:  [T1552](https://attack.mitre.org/techniques/T1552)

ⓘ

Tactic: [Credential Access](https://attack.mitre.org/tactics/TA0006)

ⓘ

Platforms: Linux, Windows, macOS

Contributors: Avioo360; Raja Singh (raja-singh-r3v-sh3ll)

Version: 2.0

Created: 04 February 2020

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1552/003/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1552/003/)

## Procedure Examples

| ID  | Name  | Description

|  [C0064](https://attack.mitre.org/campaigns/C0064)   |  [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064)   |

During the [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064), the adversary utilized Claude Code to gather data within bash histories.[[4]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has gathered data and credentials stored in Shell History.[[5]](https://www.microsoft.com/en-us/security/blog/2026/06/17/postinstall-payload-inside-mastra-npm-supply-chain-compromise/)

|  [S9045](https://attack.mitre.org/software/S9045)   |  [BACKUPOSINT v9.0 APEX PREDATOR](https://attack.mitre.org/software/S9045)   |

[BACKUPOSINT v9.0 APEX PREDATOR](https://attack.mitre.org/software/S9045) has extracted system data to include histories.[[4]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

|  [S0599](https://attack.mitre.org/software/S0599)   |  [Kinsing](https://attack.mitre.org/software/S0599)   |

[Kinsing](https://attack.mitre.org/software/S0599) has searched bash_history for credentials.[[6]](https://blog.aquasec.com/threat-alert-kinsing-malware-container-vulnerability)

|  [S9041](https://attack.mitre.org/software/S9041)   |  [TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041)   |

[TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041) can target credentials in shell history on self-hosted runners.[[7]](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)

## Mitigations

| ID  | Mitigation  | Description

|  [M1028](https://attack.mitre.org/mitigations/M1028)   |  [Operating System Configuration](https://attack.mitre.org/mitigations/M1028)   |

There are multiple methods of preventing a user's command history from being flushed to their .bash_history file, including use of the following commands:
set +o history and set -o history to start logging again;
unset HISTFILE being added to a user's .bash_rc file; and
ln -s /dev/null ~/.bash_history to write commands to /dev/null instead.

In Zsh, fc -p can be used to create a private history session. However, previous history will be unavailable to the user until the session ends. Using unset HISTFILE and writing commands to /dev/null can also be used, similarly to Bash.

In PowerShell, users can utilize Set-PSReadLineOption to modify how commands are saved into history. Setting -HistorySaveStyle SaveNothing prevents command history from being saved onto the file. Note that setting it from SaveNothing to SaveIncrementally in the same session will cause all commands from that session to be saved. Alternatively, -AddToHistoryHandler can be used to filter certain commands from being saved into the history file.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0385](https://attack.mitre.org/detectionstrategies/DET0385)   |  [Detect Access and Parsing of .bash_history Files for Credential Harvesting](https://attack.mitre.org/detectionstrategies/DET0385)   |  [AN1085](https://attack.mitre.org/detectionstrategies/DET0385#AN1085)   |

A process outside of interactive shell context reads ~/.bash_history directly (e.g., using cat, less, grep), often shortly after privilege escalation or user switch (su/sudo). This may be followed by credential scanning in memory or file writes to new locations.

|  [AN1086](https://attack.mitre.org/detectionstrategies/DET0385#AN1086)   |

A process or terminal command outside of standard shell utilities reads the user's .bash_history file. On macOS, unified logs or telemetry tools like EndpointSecurity (ESF) may observe file read APIs or terminal process lineage that shows non-user-initiated access.

## References

-    [Microsoft. (2024, January 19). about_History. Retrieved June 13, 2025.](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_history?view=powershell-7.5)
-    [Michael Koczwara. (2021, March 14). Windows privilege escalation via PowerShell History. Retrieved June 13, 2025.](https://michaelkoczwara.medium.com/windows-privilege-escalation-dbb908cce8d4)
-    [Alex Rymdeko-Harvey, Steve Borosh. (2016, May 14). External to DA, the OS X Way. Retrieved September 12, 2024.](https://www.slideshare.net/slideshow/external-to-da-the-os-x-way/62021418)
-    [Eyal Sela. (2026, April). The AI-Assisted Breach of Mexico's Government Infrastructure. Retrieved September 2, 2026.](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

-    [Microsoft Threat Intelligence. (2026, June 17). From package to postinstall payload: Inside the Mastra npm supply chain compromise by Sapphire Sleet. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/06/17/postinstall-payload-inside-mastra-npm-supply-chain-compromise/)
-    [Singer, G. (2020, April 3). Threat Alert: Kinsing Malware Attacks Targeting Container Environments. Retrieved April 1, 2021.](https://blog.aquasec.com/threat-alert-kinsing-malware-container-vulnerability)
-    [Aqua Team. (2026, April 1). Update: Ongoing Investigation and Continued Remediation. Retrieved July 1, 2026.](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)

×
