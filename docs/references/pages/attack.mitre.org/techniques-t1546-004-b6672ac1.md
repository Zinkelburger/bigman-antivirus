---
url: "https://attack.mitre.org/techniques/T1546/004/"
title: "Event Triggered Execution: Unix Shell Configuration Modification, Sub-technique T1546.004 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:47:45Z"
via: "live"
sha256: "0a5fa1a235f4dfde2aaa2691a8fb0f3502bda0c012fab305aefb56a99ca87fd7"
wayback: "https://web.archive.org/web/20261023090801/https://attack.mitre.org/techniques/T1546/004/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
  - "docs/threats/05-malware-families-and-detection.md"
---

# Event Triggered Execution: Unix Shell Configuration Modification, Sub-technique T1546.004 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1546/004/>
> Snapshot: <https://web.archive.org/web/20261023090801/https://attack.mitre.org/techniques/T1546/004/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Event Triggered Execution](https://attack.mitre.org/techniques/T1546)
- Unix Shell Configuration Modification

#  Event Triggered Execution: Unix Shell Configuration Modification

##### Other sub-techniques of Event Triggered Execution (18)

| ID  | Name

|  [T1546.001](https://attack.mitre.org/techniques/T1546/001/)   |  [Change Default File Association](https://attack.mitre.org/techniques/T1546/001/)

|  [T1546.002](https://attack.mitre.org/techniques/T1546/002/)   |  [Screensaver](https://attack.mitre.org/techniques/T1546/002/)

|  [T1546.003](https://attack.mitre.org/techniques/T1546/003/)   |  [Windows Management Instrumentation Event Subscription](https://attack.mitre.org/techniques/T1546/003/)

|  T1546.004   |  Unix Shell Configuration Modification

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

|  [T1546.017](https://attack.mitre.org/techniques/T1546/017/)   |  [Udev Rules](https://attack.mitre.org/techniques/T1546/017/)

|  [T1546.018](https://attack.mitre.org/techniques/T1546/018/)   |  [Python Startup Hooks](https://attack.mitre.org/techniques/T1546/018/)

Adversaries may establish persistence through executing malicious commands triggered by a user’s shell. User [Unix Shell](https://attack.mitre.org/techniques/T1059/004)s execute several configuration scripts at different points throughout the session based on events. For example, when a user opens a command-line interface or remotely logs in (such as via SSH) a login shell is initiated. The login shell executes scripts from the system (/etc) and the user’s home directory (~/) to configure the environment. All login shells on a system use /etc/profile when initiated. These configuration scripts run at the permission level of their directory and are often used to set environment variables, create aliases, and customize the user’s environment. When the shell exits or terminates, additional shell scripts are executed to ensure the shell exits appropriately.

Adversaries may attempt to establish persistence by inserting commands into scripts automatically executed by shells. Using bash as an example, the default shell for most GNU/Linux systems, adversaries may add commands that launch malicious binaries into the /etc/profile and /etc/profile.d files.[[1]](https://www.intezer.com/blog/research/kaiji-new-chinese-linux-malware-turning-to-golang/)[[2]](https://web.archive.org/web/20220316014323/http://bencane.com/2013/09/16/understanding-a-little-more-about-etcprofile-and-etcbashrc/) These files typically require root permissions to modify and are executed each time any shell on a system launches. For user level permissions, adversaries can insert malicious commands into ~/.bash_profile, ~/.bash_login, or ~/.profile which are sourced when a user opens a command-line interface or connects remotely.[[3]](https://www.anomali.com/blog/illicit-cryptomining-threat-actor-rocke-changes-tactics-now-more-difficult-to-detect)[[4]](https://wiki.archlinux.org/index.php/Bash#Invocation) Since the system only executes the first existing file in the listed order, adversaries have used ~/.bash_profile to ensure execution. Adversaries have also leveraged the ~/.bashrc file which is additionally executed if the connection is established remotely or an additional interactive shell is opened, such as a new tab in the command-line interface.[[5]](https://unit42.paloaltonetworks.com/unit42-new-iotlinux-malware-targets-dvrs-forms-botnet/)[[3]](https://www.anomali.com/blog/illicit-cryptomining-threat-actor-rocke-changes-tactics-now-more-difficult-to-detect)[[6]](https://www.anomali.com/blog/pulling-linux-rabbit-rabbot-malware-out-of-a-hat)[[7]](https://blog.sucuri.net/2018/05/shell-logins-as-a-magento-reinfection-vector.html) Some malware targets the termination of a program to trigger execution, adversaries can use the ~/.bash_logout file to execute malicious commands at the end of a session.

For macOS, the functionality of this technique is similar but may leverage zsh, the default shell for macOS 10.15+. When the Terminal.app is opened, the application launches a zsh login shell and a zsh interactive shell. The login shell configures the system environment using /etc/profile, /etc/zshenv, /etc/zprofile, and /etc/zlogin.[[8]](https://scriptingosx.com/2019/06/moving-to-zsh-part-2-configuration-files/)[[9]](https://posts.specterops.io/persistent-jxa-66e1c3cd1cf5)[[10]](https://github.com/D00MFist/PersistentJXA/blob/master/BashProfilePersist.js)[[11]](https://cedowens.medium.com/macos-ms-office-sandbox-brain-dump-4509b5fed49a) The login shell then configures the user environment with ~/.zprofile and ~/.zlogin. The interactive shell uses the ~/.zshrc to configure the user environment. Upon exiting, /etc/zlogout and ~/.zlogout are executed. For legacy programs, macOS executes /etc/bashrc on startup.

ID: T1546.004

Sub-technique of:  [T1546](https://attack.mitre.org/techniques/T1546)

ⓘ

Tactics: [Privilege Escalation](https://attack.mitre.org/tactics/TA0004), [Persistence](https://attack.mitre.org/tactics/TA0003)

ⓘ

Platforms: Linux, macOS

Contributors: Robert Wilson; Tony Lambert, Red Canary

Version: 2.2

Created: 24 January 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1546/004/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1546/004/)

## Procedure Examples

| ID  | Name  | Description

|  [G1052](https://attack.mitre.org/groups/G1052)   |  [Contagious Interview](https://attack.mitre.org/groups/G1052)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has targeted macOS victim hosts using a bash downloader coremedia.sh and a bash script cloud.sh.[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)

|  [S0690](https://attack.mitre.org/software/S0690)   |  [Green Lambert](https://attack.mitre.org/software/S0690)   |

[Green Lambert](https://attack.mitre.org/software/S0690) can establish persistence on a compromised host through modifying the profile, login, and run command (rc) files associated with the bash, csh, and tcsh shells. [[13]](https://objective-see.com/blog/blog_0x68.html)[[14]](https://web.archive.org/web/20211018145402/https://www.glitch-cat.com/blog/green-lambert-and-attack)

|  [S0362](https://attack.mitre.org/software/S0362)   |  [Linux Rabbit](https://attack.mitre.org/software/S0362)   |

[Linux Rabbit](https://attack.mitre.org/software/S0362) maintains persistence on an infected machine through rc.local and .bashrc files. [[15]](https://www.anomali.com/blog/pulling-linux-rabbit-rabbot-malware-out-of-a-hat)

|  [S9014](https://attack.mitre.org/software/S9014)   |  [PHASEJAM](https://attack.mitre.org/software/S9014)   |

[PHASEJAM](https://attack.mitre.org/software/S9014) has used a bash script to modify components on Ivanti Connect Secure appliances and execute files via /bin/bash.[1] It has also used the Linux stream editor (sed) to execute commands.[[16]](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-connect-secure-vpn-zero-day)

|  [S1078](https://attack.mitre.org/software/S1078)   |  [RotaJakiro](https://attack.mitre.org/software/S1078)   |

When executing with non-root level permissions, [RotaJakiro](https://attack.mitre.org/software/S1078) can install persistence by adding a command to the .bashrc file that executes a binary in the ${HOME}/.gvfsd/.profile/ folder.[[17]](https://blog.netlab.360.com/stealth_rotajakiro_backdoor_en/)

|  [C0045](https://attack.mitre.org/campaigns/C0045)   |  [ShadowRay](https://attack.mitre.org/campaigns/C0045)   |

During [ShadowRay](https://attack.mitre.org/campaigns/C0045), threat actors executed commands on interactive and reverse shells.[[18]](https://www.oligo.security/blog/shadowray-attack-ai-workloads-actively-exploited-in-the-wild)

|  [S0658](https://attack.mitre.org/software/S0658)   |  [XCSSET](https://attack.mitre.org/software/S0658)   |

Using [AppleScript](https://attack.mitre.org/techniques/T1059/002), [XCSSET](https://attack.mitre.org/software/S0658) adds it's executable to the user's ~/.zshrc_aliases file ("echo " & payload & " > ~/zshrc_aliases"), it then adds a line to the .zshrc file to source the .zshrc_aliases file ([ -f $HOME/.zshrc_aliases ] && . $HOME/.zshrc_aliases). Each time the user starts a new zsh terminal session, the .zshrc file executes the .zshrc_aliases file.[[19]](https://www.microsoft.com/en-us/security/blog/2025/03/11/new-xcsset-malware-adds-new-obfuscation-persistence-techniques-to-infect-xcode-projects/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1022](https://attack.mitre.org/mitigations/M1022)   |  [Restrict File and Directory Permissions](https://attack.mitre.org/mitigations/M1022)   |

Making these files immutable and only changeable by certain administrators will limit the ability for adversaries to easily create user level persistence.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0020](https://attack.mitre.org/detectionstrategies/DET0020)   |  [Detect Shell Configuration Modification for Persistence via Event-Triggered Execution](https://attack.mitre.org/detectionstrategies/DET0020)   |  [AN0059](https://attack.mitre.org/detectionstrategies/DET0020#AN0059)   |

Detects modification of shell startup/logout scripts such as ~/.bashrc, ~/.bash_profile, or /etc/profile, followed by anomalous process execution or network connections upon interactive or remote shell login.

|  [AN0060](https://attack.mitre.org/detectionstrategies/DET0020#AN0060)   |

Correlates zsh shell configuration file changes (e.g., ~/.zshrc, ~/.zlogin, /etc/zprofile) with execution of unauthorized binaries or unexpected network activity triggered on Terminal.app launch.

## References

-    [Paul Litvak. (2020, May 4). Kaiji: New Chinese Linux malware turning to Golang. Retrieved December 17, 2020.](https://www.intezer.com/blog/research/kaiji-new-chinese-linux-malware-turning-to-golang/)
-    [Benjamin Cane. (2013, September 16). Understanding a little more about /etc/profile and /etc/bashrc. Retrieved September 25, 2024.](https://web.archive.org/web/20220316014323/http://bencane.com/2013/09/16/understanding-a-little-more-about-etcprofile-and-etcbashrc/)
-    [Anomali Threat Research. (2019, October 15). Illicit Cryptomining Threat Actor Rocke Changes Tactics, Now More Difficult to Detect. Retrieved December 17, 2020.](https://www.anomali.com/blog/illicit-cryptomining-threat-actor-rocke-changes-tactics-now-more-difficult-to-detect)
-    [ArchWiki. (2021, January 19). Bash. Retrieved February 25, 2021.](https://wiki.archlinux.org/index.php/Bash#Invocation)
-    [Claud Xiao and Cong Zheng. (2017, April 6). New IoT/Linux Malware Targets DVRs, Forms Botnet. Retrieved December 17, 2020.](https://unit42.paloaltonetworks.com/unit42-new-iotlinux-malware-targets-dvrs-forms-botnet/)
-    [Anomali Threat Research. (2018, December 6). Pulling Linux Rabbit/Rabbot Malware Out of a Hat. Retrieved December 17, 2020.](https://www.anomali.com/blog/pulling-linux-rabbit-rabbot-malware-out-of-a-hat)
-    [Cesar Anjos. (2018, May 31). Shell Logins as a Magento Reinfection Vector. Retrieved December 17, 2020.](https://blog.sucuri.net/2018/05/shell-logins-as-a-magento-reinfection-vector.html)
-    [Armin Briegel. (2019, June 5). Moving to zsh, part 2: Configuration Files. Retrieved February 25, 2021.](https://scriptingosx.com/2019/06/moving-to-zsh-part-2-configuration-files/)
-    [Leo Pitt. (2020, August 6). Persistent JXA - A poor man's Powershell for macOS. Retrieved January 11, 2021.](https://posts.specterops.io/persistent-jxa-66e1c3cd1cf5)
-    [Leo Pitt. (2020, November 11). Github - PersistentJXA/BashProfilePersist.js. Retrieved January 11, 2021.](https://github.com/D00MFist/PersistentJXA/blob/master/BashProfilePersist.js)

-    [Cedric Owens. (2021, May 22). macOS MS Office Sandbox Brain Dump. Retrieved August 20, 2021.](https://cedowens.medium.com/macos-ms-office-sandbox-brain-dump-4509b5fed49a)
-    [Amaury G., Coline Chavane, Felix Aimé and Sekoia TDR. (2025, March 31). From Contagious to ClickFake Interview: Lazarus leveraging the ClickFix tactic. Retrieved April 1, 2025.](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)
-    [Sandvik, Runa. (2021, October 1). Made In America: Green Lambert for OS X. Retrieved March 21, 2022.](https://objective-see.com/blog/blog_0x68.html)
-    [Sandvik, Runa. (2021, October 18). Green Lambert and ATT&CK. Retrieved November 17, 2024.](https://web.archive.org/web/20211018145402/https://www.glitch-cat.com/blog/green-lambert-and-attack)
-    [Anomali Labs. (2018, December 6). Pulling Linux Rabbit/Rabbot Malware Out of a Hat. Retrieved March 4, 2019.](https://www.anomali.com/blog/pulling-linux-rabbit-rabbot-malware-out-of-a-hat)
-    [John Wolfram, Josh Murchie, Matt Lin, Daniel Ainsworth, Robert Wallace, Dimiter Andonov, Dhanesh Kizhakkinan, Jacob Thompson. (2025, January 8). Ivanti Connect Secure VPN Targeted in New Zero-Day Exploitation. Retrieved April 14, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-connect-secure-vpn-zero-day)
-    [Alex Turing, Hui Wang. (2021, April 28). RotaJakiro: A long live secret backdoor with 0 VT detection. Retrieved June 14, 2023.](https://blog.netlab.360.com/stealth_rotajakiro_backdoor_en/)
-    [Lumelsly, A. et al. (2024, March 26). ShadowRay: First Known Attack Campaign Targeting AI Workloads Actively Exploited In The Wild. Retrieved December 2, 2024.](https://www.oligo.security/blog/shadowray-attack-ai-workloads-actively-exploited-in-the-wild)
-    [Microsoft Threat Intelligence. (2025, March 11). New XCSSET malware adds new obfuscation, persistence techniques to infect Xcode projects. Retrieved April 2, 2025.](https://www.microsoft.com/en-us/security/blog/2025/03/11/new-xcsset-malware-adds-new-obfuscation-persistence-techniques-to-infect-xcode-projects/)

×
