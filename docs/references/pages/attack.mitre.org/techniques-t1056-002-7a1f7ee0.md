---
url: "https://attack.mitre.org/techniques/T1056/002/"
title: "Input Capture: GUI Input Capture, Sub-technique T1056.002 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:29:01Z"
via: "live"
sha256: "dba4c0094e7fb006ceb666e3296fde76bb9b3c011f4395b1d2069ec736d9fa47"
wayback: "https://web.archive.org/web/20261019131351/https://attack.mitre.org/techniques/T1056/002/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Input Capture: GUI Input Capture, Sub-technique T1056.002 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1056/002/>
> Snapshot: <https://web.archive.org/web/20261019131351/https://attack.mitre.org/techniques/T1056/002/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Input Capture](https://attack.mitre.org/techniques/T1056)
- GUI Input Capture

#  Input Capture: GUI Input Capture

##### Other sub-techniques of Input Capture (4)

| ID  | Name

|  [T1056.001](https://attack.mitre.org/techniques/T1056/001/)   |  [Keylogging](https://attack.mitre.org/techniques/T1056/001/)

|  T1056.002   |  GUI Input Capture

|  [T1056.003](https://attack.mitre.org/techniques/T1056/003/)   |  [Web Portal Capture](https://attack.mitre.org/techniques/T1056/003/)

|  [T1056.004](https://attack.mitre.org/techniques/T1056/004/)   |  [Credential API Hooking](https://attack.mitre.org/techniques/T1056/004/)

Adversaries may mimic common operating system GUI components to prompt users for credentials with a seemingly legitimate prompt. When programs are executed that need additional privileges than are present in the current user context, it is common for the operating system to prompt the user for proper credentials to authorize the elevated privileges for the task (ex: [Bypass User Account Control](https://attack.mitre.org/techniques/T1548/002)).

Adversaries may mimic this functionality to prompt users for credentials with a seemingly legitimate prompt for a number of reasons that mimic normal usage, such as a fake installer requiring additional access or a fake malware removal suite.[[1]](https://baesystemsai.blogspot.com/2015/06/new-mac-os-malware-exploits-mackeeper.html) This type of prompt can be used to collect credentials via various languages such as [AppleScript](https://attack.mitre.org/techniques/T1059/002)[[2]](https://logrhythm.com/blog/do-you-trust-your-computer/)[[3]](https://www.welivesecurity.com/2016/07/06/new-osxkeydnap-malware-hungry-credentials/)[[4]](https://embracethered.com/blog/posts/2021/spoofing-credential-dialogs/) and [PowerShell](https://attack.mitre.org/techniques/T1059/001).[[2]](https://logrhythm.com/blog/do-you-trust-your-computer/)[[5]](https://enigma0x3.net/2015/01/21/phishing-for-credentials-if-you-want-it-just-ask/)[[4]](https://embracethered.com/blog/posts/2021/spoofing-credential-dialogs/) On Linux systems adversaries may launch dialog boxes prompting users for credentials from malicious shell scripts or the command line (i.e. [Unix Shell](https://attack.mitre.org/techniques/T1059/004)).[[4]](https://embracethered.com/blog/posts/2021/spoofing-credential-dialogs/)

Adversaries may also mimic common software authentication requests, such as those from browsers or email clients. This may also be paired with user activity monitoring (i.e., [Browser Information Discovery](https://attack.mitre.org/techniques/T1217) and/or [Application Window Discovery](https://attack.mitre.org/techniques/T1010)) to spoof prompts when users are naturally accessing sensitive sites/data.

ID: T1056.002

Sub-technique of:  [T1056](https://attack.mitre.org/techniques/T1056)

ⓘ

Tactics: [Collection](https://attack.mitre.org/tactics/TA0009), [Credential Access](https://attack.mitre.org/tactics/TA0006)

ⓘ

Platforms: Linux, Windows, macOS

Contributors: Matthew Molyett, @s1air, Cisco Talos

Version: 1.3

Created: 11 February 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1056/002/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1056/002/)

## Procedure Examples

| ID  | Name  | Description

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has leveraged a fictitious native macOS password dialog prompt to entice victims to enter credentials.[[6]](https://www.microsoft.com/en-us/security/blog/2026/06/17/postinstall-payload-inside-mastra-npm-supply-chain-compromise/)

|  [S0482](https://attack.mitre.org/software/S0482)   |  [Bundlore](https://attack.mitre.org/software/S0482)   |

[Bundlore](https://attack.mitre.org/software/S0482) prompts the user for their credentials.[[7]](https://mackeeper.com/blog/post/610-macos-bundlore-adware-analysis/)

|  [S0274](https://attack.mitre.org/software/S0274)   |  [Calisto](https://attack.mitre.org/software/S0274)   |

[Calisto](https://attack.mitre.org/software/S0274) presents an input prompt asking for the user's login and password.[[8]](https://web.archive.org/web/20190111082249/https://www.symantec.com/security-center/writeup/2018-073014-2512-99?om_rssid=sr-latestthreats30days)

|  [S1153](https://attack.mitre.org/software/S1153)   |  [Cuckoo Stealer](https://attack.mitre.org/software/S1153)   |

[Cuckoo Stealer](https://attack.mitre.org/software/S1153) has captured passwords by prompting victims with a "macOS needs to access System Settings" GUI window.[[9]](https://www.kandji.io/blog/malware-cuckoo-infostealer-spyware)

|  [S0281](https://attack.mitre.org/software/S0281)   |  [Dok](https://attack.mitre.org/software/S0281)   |

[Dok](https://attack.mitre.org/software/S0281) prompts the user for credentials.[[10]](https://objective-see.com/blog/blog_0x25.html)

|  [G0085](https://attack.mitre.org/groups/G0085)   |  [FIN4](https://attack.mitre.org/groups/G0085)   |

[FIN4](https://attack.mitre.org/groups/G0085) has presented victims with spoofed Windows Authentication prompts to collect their credentials.[[11]](https://web.archive.org/web/20220313062515/https://www.mandiant.com/sites/default/files/2021-09/rpt-fin4.pdf)[[12]](https://www2.fireeye.com/WBNR-14Q4NAMFIN4.html)

|  [S0278](https://attack.mitre.org/software/S0278)   |  [iKitten](https://attack.mitre.org/software/S0278)   |

[iKitten](https://attack.mitre.org/software/S0278) prompts the user for their credentials.[[10]](https://objective-see.com/blog/blog_0x25.html)

|  [S0276](https://attack.mitre.org/software/S0276)   |  [Keydnap](https://attack.mitre.org/software/S0276)   |

[Keydnap](https://attack.mitre.org/software/S0276) prompts the users for credentials.[[13]](https://objective-see.org/blog/blog_0x16.html)

|  [S9036](https://attack.mitre.org/software/S9036)   |  [LP-Notes](https://attack.mitre.org/software/S9036)   |

[LP-Notes](https://attack.mitre.org/software/S9036) has displayed a fake Windows Security dialog box to prompt for Windows credentials.[[14]](https://www.welivesecurity.com/en/eset-research/muddywater-snakes-riverbank/)

|  [S0455](https://attack.mitre.org/software/S0455)   |  [Metamorfo](https://attack.mitre.org/software/S0455)   |

[Metamorfo](https://attack.mitre.org/software/S0455) has displayed fake forms on top of banking sites to intercept credentials from victims.[[15]](https://www.fireeye.com/blog/threat-research/2018/04/metamorfo-campaign-targeting-brazilian-users.html)

|  [S1122](https://attack.mitre.org/software/S1122)   |  [Mispadu](https://attack.mitre.org/software/S1122)   |

[Mispadu](https://attack.mitre.org/software/S1122) can monitor browser activity for online banking actions and display full-screen overlay images to block user access to the intended site or present additional data fields.[[16]](https://seguranca-informatica.pt/threat-analysis-the-emergent-ursa-trojan-impacts-many-countries-using-a-sophisticated-loader/)[[17]](https://blog.scilabs.mx/en/cyber-threat-profile-malteiro/)

|  [S9032](https://attack.mitre.org/software/S9032)   |  [MuddyViper](https://attack.mitre.org/software/S9032)   |

[MuddyViper](https://attack.mitre.org/software/S9032) has displayed a fake Windows Security dialog to gather credentials.[[14]](https://www.welivesecurity.com/en/eset-research/muddywater-snakes-riverbank/)

|  [S0279](https://attack.mitre.org/software/S0279)   |  [Proton](https://attack.mitre.org/software/S0279)   |

[Proton](https://attack.mitre.org/software/S0279) prompts users for their credentials.[[10]](https://objective-see.com/blog/blog_0x25.html)

|  [G1039](https://attack.mitre.org/groups/G1039)   |  [RedCurl](https://attack.mitre.org/groups/G1039)   |

[RedCurl](https://attack.mitre.org/groups/G1039) prompts the user for credentials through a Microsoft Outlook pop-up.[[18]](https://www.group-ib.com/resources/research-hub/red-curl/)[[19]](https://www.group-ib.com/resources/research-hub/red-curl-2/)

|  [S0692](https://attack.mitre.org/software/S0692)   |  [SILENTTRINITY](https://attack.mitre.org/software/S0692)   |

[SILENTTRINITY](https://attack.mitre.org/software/S0692)'s credphisher.py module can prompt a current user for their credentials.[[20]](https://github.com/byt3bl33d3r/SILENTTRINITY/tree/master/silenttrinity/core/teamserver/modules/boo)

|  [S9064](https://attack.mitre.org/software/S9064)   |  [TRUSTTRAP](https://attack.mitre.org/software/S9064)   |

[TRUSTTRAP](https://attack.mitre.org/software/S9064) has captured victims’ credentials using fake login popup and stores captured data in a file.[[21]](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)

|  [S0658](https://attack.mitre.org/software/S0658)   |  [XCSSET](https://attack.mitre.org/software/S0658)   |

[XCSSET](https://attack.mitre.org/software/S0658) prompts the user to input credentials using a native macOS dialog box leveraging the system process /Applications/Safari.app/Contents/MacOS/SafariForWebKitDevelopment.[[22]](https://documents.trendmicro.com/assets/pdf/XCSSET_Technical_Brief.pdf)

## Mitigations

| ID  | Mitigation  | Description

|  [M1017](https://attack.mitre.org/mitigations/M1017)   |  [User Training](https://attack.mitre.org/mitigations/M1017)   |

Use user training as a way to bring awareness and raise suspicion for potentially malicious events and dialog boxes (ex: Office documents prompting for credentials).

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0521](https://attack.mitre.org/detectionstrategies/DET0521)   |  [Behavioral Detection of Spoofed GUI Credential Prompts](https://attack.mitre.org/detectionstrategies/DET0521)   |  [AN1440](https://attack.mitre.org/detectionstrategies/DET0521#AN1440)   |

Detects suspicious use of PowerShell, .NET, or script interpreters to spawn processes that mimic UAC prompts, often with credential capture dialogue boxes invoked from non-standard parent processes.

|  [AN1441](https://attack.mitre.org/detectionstrategies/DET0521#AN1441)   |

Detects GUI-based credential prompts invoked via zenity/kdialog/dialog or X11 APIs from non-user-facing scripts or background shell sessions, often with authentication-related text.

|  [AN1442](https://attack.mitre.org/detectionstrategies/DET0521#AN1442)   |

Detects AppleScript or Objective-C usage to generate fake authentication windows (e.g., using display dialog or NSAlert) from user-launched or persistence-related processes.

## References

-    [Sergei Shevchenko. (2015, June 4). New Mac OS Malware Exploits Mackeeper. Retrieved July 3, 2017.](https://baesystemsai.blogspot.com/2015/06/new-mac-os-malware-exploits-mackeeper.html)
-    [Foss, G. (2014, October 3). Do You Trust Your Computer?. Retrieved December 17, 2018.](https://logrhythm.com/blog/do-you-trust-your-computer/)
-    [Marc-Etienne M.Leveille. (2016, July 6). New OSX/Keydnap malware is hungry for credentials. Retrieved July 3, 2017.](https://www.welivesecurity.com/2016/07/06/new-osxkeydnap-malware-hungry-credentials/)
-    [Johann Rehberger. (2021, April 18). Spoofing credential dialogs on macOS Linux and Windows. Retrieved August 19, 2021.](https://embracethered.com/blog/posts/2021/spoofing-credential-dialogs/)
-    [Nelson, M. (2015, January 21). Phishing for Credentials: If you want it, just ask!. Retrieved December 17, 2018.](https://enigma0x3.net/2015/01/21/phishing-for-credentials-if-you-want-it-just-ask/)
-    [Microsoft Threat Intelligence. (2026, June 17). From package to postinstall payload: Inside the Mastra npm supply chain compromise by Sapphire Sleet. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/06/17/postinstall-payload-inside-mastra-npm-supply-chain-compromise/)
-    [Sushko, O. (2019, April 17). macOS Bundlore: Mac Virus Bypassing macOS Security Features. Retrieved June 30, 2020.](https://mackeeper.com/blog/post/610-macos-bundlore-adware-analysis/)
-    [Pantig, J. (2018, July 30). OSX.Calisto. Retrieved September 7, 2018.](https://web.archive.org/web/20190111082249/https://www.symantec.com/security-center/writeup/2018-073014-2512-99?om_rssid=sr-latestthreats30days)
-    [Kohler, A. and Lopez, C. (2024, April 30). Malware: Cuckoo Behaves Like Cross Between Infostealer and Spyware. Retrieved August 20, 2024.](https://www.kandji.io/blog/malware-cuckoo-infostealer-spyware)
-    [Patrick Wardle. (n.d.). Mac Malware of 2017. Retrieved September 21, 2018.](https://objective-see.com/blog/blog_0x25.html)
-    [Vengerik, B. et al.. (2014, December 5). Hacking the Street? FIN4 Likely Playing the Market. Retrieved December 17, 2018.](https://web.archive.org/web/20220313062515/https://www.mandiant.com/sites/default/files/2021-09/rpt-fin4.pdf)

-    [Vengerik, B. & Dennesen, K.. (2014, December 5). Hacking the Street? FIN4 Likely Playing the Market. Retrieved January 15, 2019.](https://www2.fireeye.com/WBNR-14Q4NAMFIN4.html)
-    [Patrick Wardle. (2017, January 1). Mac Malware of 2016. Retrieved September 21, 2018.](https://objective-see.org/blog/blog_0x16.html)
-    [ESET Research. (2025, December 2). MuddyWater: Snakes by the riverbank. Retrieved February 17, 2026.](https://www.welivesecurity.com/en/eset-research/muddywater-snakes-riverbank/)
-    [Sierra, E., Iglesias, G.. (2018, April 24). Metamorfo Campaigns Targeting Brazilian Users. Retrieved July 30, 2020.](https://www.fireeye.com/blog/threat-research/2018/04/metamorfo-campaign-targeting-brazilian-users.html)
-    [Pedro Tavares (Segurança Informática). (2020, September 15). Threat analysis: The emergent URSA trojan impacts many countries using a sophisticated loader. Retrieved March 13, 2024.](https://seguranca-informatica.pt/threat-analysis-the-emergent-ursa-trojan-impacts-many-countries-using-a-sophisticated-loader/)
-    [SCILabs. (2021, December 23). Cyber Threat Profile Malteiro. Retrieved March 13, 2024.](https://blog.scilabs.mx/en/cyber-threat-profile-malteiro/)
-    [Group-IB. (2020, August). RedCurl: The Pentest You Didn’t Know About. Retrieved August 9, 2024.](https://www.group-ib.com/resources/research-hub/red-curl/)
-    [Group-IB. (2021, November). RedCurl: The Awakening. Retrieved August 14, 2024.](https://www.group-ib.com/resources/research-hub/red-curl-2/)
-    [Salvati, M. (2019, August 6). SILENTTRINITY Modules. Retrieved March 24, 2022.](https://github.com/byt3bl33d3r/SILENTTRINITY/tree/master/silenttrinity/core/teamserver/modules/boo)
-    [El-Banna, M., et al. (2025, November 17). Frontline Intelligence: Analysis of UNC1549 TTPs, Custom Tools, and Malware Targeting the Aerospace and Defense Ecosystem. Retrieved September 28, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)
-    [Mac Threat Response, Mobile Research Team. (2020, August 13). The XCSSET Malware: Inserts Malicious Code Into Xcode Projects, Performs UXSS Backdoor Planting in Safari, and Leverages Two Zero-day Exploits. Retrieved October 5, 2021.](https://documents.trendmicro.com/assets/pdf/XCSSET_Technical_Brief.pdf)

×
