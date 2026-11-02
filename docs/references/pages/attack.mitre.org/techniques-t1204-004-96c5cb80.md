---
url: "https://attack.mitre.org/techniques/T1204/004/"
title: "User Execution: Malicious Copy and Paste, Sub-technique T1204.004 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:41:46Z"
via: "live"
sha256: "fab36a0b8bd18c012561612cf7ca9fd62d1bc88a658e5b4b9d098845c12d59f8"
wayback: "https://web.archive.org/web/20261023090735/https://attack.mitre.org/techniques/T1204/004/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# User Execution: Malicious Copy and Paste, Sub-technique T1204.004 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1204/004/>
> Snapshot: <https://web.archive.org/web/20261023090735/https://attack.mitre.org/techniques/T1204/004/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [User Execution](https://attack.mitre.org/techniques/T1204)
- Malicious Copy and Paste

#  User Execution: Malicious Copy and Paste

##### Other sub-techniques of User Execution (5)

| ID  | Name

|  [T1204.001](https://attack.mitre.org/techniques/T1204/001/)   |  [Malicious Link](https://attack.mitre.org/techniques/T1204/001/)

|  [T1204.002](https://attack.mitre.org/techniques/T1204/002/)   |  [Malicious File](https://attack.mitre.org/techniques/T1204/002/)

|  [T1204.003](https://attack.mitre.org/techniques/T1204/003/)   |  [Malicious Image](https://attack.mitre.org/techniques/T1204/003/)

|  T1204.004   |  Malicious Copy and Paste

|  [T1204.005](https://attack.mitre.org/techniques/T1204/005/)   |  [Malicious Library](https://attack.mitre.org/techniques/T1204/005/)

An adversary may rely upon a user copying and pasting code in order to gain execution. Users may be subjected to social engineering to get them to copy and paste code directly into a [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059). One such strategy is "ClickFix," in which adversaries present users with seemingly helpful solutions—such as prompts to fix errors or complete CAPTCHAs—that instead instruct the user to copy and paste malicious code.

Malicious websites, such as those used in [Drive-by Compromise](https://attack.mitre.org/techniques/T1189), may present fake error messages or CAPTCHA prompts that instruct users to open a terminal or the Windows Run Dialog box and execute an arbitrary command. These commands may be obfuscated using encoding or other techniques to conceal malicious intent. Once executed, the adversary will typically be able to establish a foothold on the victim's machine.[[1]](https://www.cloudsek.com/blog/unmasking-the-danger-lumma-stealer-malware-exploits-fake-captcha-pages)[[2]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)[[3]](https://www.reliaquest.com/blog/using-captcha-for-compromise/)[[4]](https://asec.ahnlab.com/en/85699/)

Adversaries may also leverage phishing emails for this purpose. When a user attempts to open an attachment, they may be presented with a fake error and offered a malicious command to paste as a solution, consistent with the "ClickFix" strategy.[[5]](https://www.proofpoint.com/us/blog/threat-insight/security-brief-clickfix-social-engineering-technique-floods-threat-landscape)[[6]](https://asec.ahnlab.com/en/73952/)

Tricking a user into executing a command themselves may help to bypass email filtering, browser sandboxing, or other mitigations designed to protect users against malicious downloaded files.

ID: T1204.004

Sub-technique of:  [T1204](https://attack.mitre.org/techniques/T1204)

ⓘ

Tactic: [Execution](https://attack.mitre.org/tactics/TA0002)

ⓘ

Platforms: Linux, Windows, macOS

Contributors: Ale Houspanossian; Fernando Bacchin; Gabriel Currie; Harikrishnan Muthu, Cyble; Menachem Goldstein; ReliaQuest; SeungYoul Yoo, AhnLab

Version: 1.1

Created: 18 March 2025

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1204/004/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1204/004/)

## Procedure Examples

| ID  | Name  | Description

|  [G1052](https://attack.mitre.org/groups/G1052)   |  [Contagious Interview](https://attack.mitre.org/groups/G1052)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has leveraged ClickFix type tactics enticing victims to copy and paste malicious code.[[7]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)[[2]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)[[8]](https://www.validin.com/blog/inoculating_contagious_interview_with_validin/)

|  [S1229](https://attack.mitre.org/software/S1229)   |  [Havoc](https://attack.mitre.org/software/S1229)   |

The [Havoc](https://attack.mitre.org/software/S1229) infection chain has been initiated via ClickFix lures in phishing emails.[[9]](https://www.fortinet.com/blog/threat-research/havoc-sharepoint-with-microsoft-graph-api-turns-into-fud-c2)

|  [S9044](https://attack.mitre.org/software/S9044)   |  [Kali365](https://attack.mitre.org/software/S9044)   |

[Kali365](https://attack.mitre.org/software/S9044) has dynamically generated legitimate device codes that displays on the victims screen alongside instructions to copy and paste the device code to initiate and complete a successful authentication process.[[10]](https://arcticwolf.com/resources/blog/token-bingo-dont-let-your-code-be-the-winner/)[[11]](https://arcticwolf.com/resources/blog/kali365-expands-into-aws-microsoft-okta-xerox-max-messenger/)[[12]](https://www.ic3.gov/PSA/2026/PSA260521)[[13]](https://www.huntress.com/blog/kali365-device-code-phishing-kit)[[14]](https://spycloud.com/blog/kali365-anatomy-of-a-microsoft365-phishing-as-a-service-kit/)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has leveraged ClickFix type tactics enticing victims to copy and paste malicious code.[[15]](https://www.proofpoint.com/us/blog/threat-insight/around-world-90-days-state-sponsored-actors-try-clickfix)

|  [G0069](https://attack.mitre.org/groups/G0069)   |  [MuddyWater](https://attack.mitre.org/groups/G0069)   |

[MuddyWater](https://attack.mitre.org/groups/G0069) has leveraged ClickFix type tactics enticing victims to copy and paste malicious PowerShell code.[[15]](https://www.proofpoint.com/us/blog/threat-insight/around-world-90-days-state-sponsored-actors-try-clickfix)

|  [S9050](https://attack.mitre.org/software/S9050)   |  [Vidar](https://attack.mitre.org/software/S9050)   |

[Vidar](https://attack.mitre.org/software/S9050) has used fake CAPTCHAs to deliver and execute itself to victims.[[16]](https://www.cyber.gov.au/about-us/view-all-content/alerts-and-advisories/clickfix-distributing-vidar-stealer-via-wordpress-targeting-australian-infrastructure)

## Mitigations

| ID  | Mitigation  | Description

|  [M1038](https://attack.mitre.org/mitigations/M1038)   |  [Execution Prevention](https://attack.mitre.org/mitigations/M1038)   |

Use application control where appropriate. PowerShell Constrained Language mode can be used to restrict access to sensitive or otherwise dangerous language elements such as those used to execute arbitrary Windows APIs or files (e.g., Add-Type).[[17]](https://devblogs.microsoft.com/powershell/powershell-constrained-language-mode/)

|  [M1031](https://attack.mitre.org/mitigations/M1031)   |  [Network Intrusion Prevention](https://attack.mitre.org/mitigations/M1031)   |

If a link is being requested by a user, network intrusion prevention systems and systems designed to scan and remove malicious downloads can be used to block activity.

|  [M1021](https://attack.mitre.org/mitigations/M1021)   |  [Restrict Web-Based Content](https://attack.mitre.org/mitigations/M1021)   |

If a link is being requested by a user, block unknown or unused files in transit by default that should not be downloaded or by policy from suspicious sites as a best practice to prevent some vectors, such as .scr, .exe, .pif, .cpl, etc.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0340](https://attack.mitre.org/detectionstrategies/DET0340)   |  [User Execution – Malicious Copy & Paste (browser/email → shell with obfuscated one-liner) – T1204.004](https://attack.mitre.org/detectionstrategies/DET0340)   |  [AN0962](https://attack.mitre.org/detectionstrategies/DET0340#AN0962)   |

A user is socially engineered (web page, email, document) to open Run/PowerShell/CMD and paste an obfuscated one-liner. The chain is: (1) user context active in a browser/email/office app → (2) process creation of a command interpreter with suspicious arguments (base64/Invoke-Expression/web download/pipeline to shell) → (3) optional file drop in %TEMP% or %APPDATA% → (4) outbound network connection to an external domain. Events are correlated within a short window and with consistent user/session.

|  [AN0963](https://attack.mitre.org/detectionstrategies/DET0340#AN0963)   |

User pastes a multi-line or one-liner into a terminal (bash/zsh) that downloads/decodes and executes content. Chain: terminal exec of curl/wget/bash/sh with pipe to interpreter or base64-decode → transient file under /tmp|~/.cache → immediate outbound egress.

|  [AN0964](https://attack.mitre.org/detectionstrategies/DET0340#AN0964)   |

User pastes an obfuscated command into Terminal.app/iTerm2 that decodes or downloads code and executes. Detects Terminal/iTerm2 spawning bash/zsh/python with suspicious pipeline/base64 patterns followed by file writes in ~/Library or /tmp and outbound network connections.

## References

-    [CloudSEK TRIAD. (2024, September 19). Unmasking the Danger: Lumma Stealer Malware Exploits Fake CAPTCHA Pages. Retrieved March 18, 2025.](https://www.cloudsek.com/blog/unmasking-the-danger-lumma-stealer-malware-exploits-fake-captcha-pages)
-    [Amaury G., Coline Chavane, Felix Aimé and Sekoia TDR. (2025, March 31). From Contagious to ClickFake Interview: Lazarus leveraging the ClickFix tactic. Retrieved April 1, 2025.](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)
-    [Alex Capraro. (2024, December 17). Using CAPTCHA for Compromise: Hackers Flip the Script. Retrieved March 18, 2025.](https://www.reliaquest.com/blog/using-captcha-for-compromise/)
-    [AhnLab SEcurity intelligence Center. (2025, January 8). Infostealer LummaC2 Spreading Through Fake CAPTCHA Verification Page. Retrieved April 23, 2025.](https://asec.ahnlab.com/en/85699/)
-    [Tommy Madjar, Selena Larson and The Proofpoint Threat Research Team. (2024, November 18). Security Brief: ClickFix Social Engineering Technique Floods Threat Landscape. Retrieved March 18, 2025.](https://www.proofpoint.com/us/blog/threat-insight/security-brief-clickfix-social-engineering-technique-floods-threat-landscape)
-    [AhnLab SEcurity intelligence Center. (2024, May 23). Warning Against Phishing Emails Prompting Execution of Commands via Paste (CTRL+V). Retrieved April 23, 2025.](https://asec.ahnlab.com/en/73952/)
-    [Aleksandar Milenkoski, Sreekar Madabushi, Kenneth Kinion. (2025, September 4). Contagious Interview | North Korean Threat Actors Reveal Plans and Ops by Abusing Cyber Intel Platforms. Retrieved October 20, 2025.](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)
-    [Efstratios Lontzetidis. (2025, January 16). Lazarus APT: Techniques for Hunting Contagious Interview. Retrieved October 20, 2025.](https://www.validin.com/blog/inoculating_contagious_interview_with_validin/)
-    [Wan, Y. (2025, March 3). Havoc: SharePoint with Microsoft Graph API turns into FUD C2. Retrieved August 4, 2025.](https://www.fortinet.com/blog/threat-research/havoc-sharepoint-with-microsoft-graph-api-turns-into-fud-c2)

-    [Artic Wolf Labs. (2026, April 24). Token Bingo: Don’t Let Your Code be the Winner. Retrieved July 30, 2026.](https://arcticwolf.com/resources/blog/token-bingo-dont-let-your-code-be-the-winner/)
-    [Artic Wolf Labs. (2026, June 2). Retrieved July 30, 2026.](https://arcticwolf.com/resources/blog/kali365-expands-into-aws-microsoft-okta-xerox-max-messenger/)
-    [Federal Bureau of Investigation. (2026, May 21). Alert Number: I-052126-PSA: Kali365 Phishing-as-a-Service Kit Hijacks Microsoft 365 Access Tokens. Retrieved July 30, 2026.](https://www.ic3.gov/PSA/2026/PSA260521)
-    [Tanner Flip. (2026, June 11). Hook, Line, and Token: Anatomy of the Kali365 / Octopi365 Phishing-as-a-Service Kit. Retrieved July 30, 2026.](https://www.huntress.com/blog/kali365-device-code-phishing-kit)
-    [Trevor Hilligoss. (2026, June 11). Kali365: Anatomy of a Microsoft 365 Phishing-as-a-Service Kit – From Telegram Hype to FBI Takedown Theater. Retrieved July 30, 2026.](https://spycloud.com/blog/kali365-anatomy-of-a-microsoft365-phishing-as-a-service-kit/)
-    [Naumaan, S., et al. (2025, April 17). Around the World in 90 Days: State-Sponsored Actors Try ClickFix . Retrieved January 21, 2026.](https://www.proofpoint.com/us/blog/threat-insight/around-world-90-days-state-sponsored-actors-try-clickfix)
-    [Australian Signals Directorate’s Australian Cyber Security Centre. (2026, May 7). ClickFix distributing Vidar Stealer via WordPress targeting Australian infrastructure. Retrieved May 8, 2026.](https://www.cyber.gov.au/about-us/view-all-content/alerts-and-advisories/clickfix-distributing-vidar-stealer-via-wordpress-targeting-australian-infrastructure)
-    [PowerShell Team. (2017, November 2). PowerShell Constrained Language Mode. Retrieved March 27, 2023.](https://devblogs.microsoft.com/powershell/powershell-constrained-language-mode/)

×
