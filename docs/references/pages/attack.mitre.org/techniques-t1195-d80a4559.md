---
url: "https://attack.mitre.org/techniques/T1195/"
title: "Supply Chain Compromise, Technique T1195 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:39:19Z"
via: "live"
sha256: "6d54218e49a988e93a2f6c35d4dc8b25a12c12631b1d2153d000859f479d7117"
wayback: "https://web.archive.org/web/20261023145942/https://attack.mitre.org/techniques/T1195/"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# Supply Chain Compromise, Technique T1195 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1195/>
> Snapshot: <https://web.archive.org/web/20261023145942/https://attack.mitre.org/techniques/T1195/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Supply Chain Compromise

#  Supply Chain Compromise

##### Sub-techniques (3)

| ID  | Name

|  [T1195.001](https://attack.mitre.org/techniques/T1195/001/)   |  [Compromise Software Dependencies and Development Tools](https://attack.mitre.org/techniques/T1195/001/)

|  [T1195.002](https://attack.mitre.org/techniques/T1195/002/)   |  [Compromise Software Supply Chain](https://attack.mitre.org/techniques/T1195/002/)

|  [T1195.003](https://attack.mitre.org/techniques/T1195/003/)   |  [Compromise Hardware Supply Chain](https://attack.mitre.org/techniques/T1195/003/)

Adversaries may manipulate products or product delivery mechanisms prior to receipt by a final consumer for the purpose of data or system compromise.

Supply chain compromise can take place at any stage of the supply chain including:

- Manipulation of development tools
- Manipulation of a development environment
- Manipulation of source code repositories (public or private)
- Manipulation of source code in open-source dependencies
- Manipulation of software update/distribution mechanisms
- Compromised/infected system images (removable media infected at the factory)[[1]](https://www-01.ibm.com/support/docview.wss?uid=ssg1S1010146&myns=s028&mynp=OCSTHGUJ&mynp=OCSTLM5A&mynp=OCSTLM6B&mynp=OCHW206&mync=E&cm_sp=s028-_-OCSTHGUJ-OCSTLM5A-OCSTLM6B-OCHW206-_-E)[[2]](https://www.se.com/us/en/download/document/SESN-2018-236-01/)
- Replacement of legitimate software with modified versions
- Sales of modified/counterfeit products to legitimate distributors
- Shipment interdiction

While supply chain compromise can impact any component of hardware or software, adversaries looking to gain execution have often focused on malicious additions to legitimate software in software distribution or update channels.[[3]](https://blog.avast.com/new-investigations-in-ccleaner-incident-point-to-a-possible-third-stage-that-had-keylogger-capacities)[[4]](https://cloudblogs.microsoft.com/microsoftsecure/2018/03/07/behavior-monitoring-combined-with-machine-learning-spoils-a-massive-dofoil-coin-mining-campaign/)[[5]](https://web.archive.org/web/20160309235002/https://www.commandfive.com/papers/C5_APT_SKHack.pdf) Adversaries may limit targeting to a desired victim set or distribute malicious software to a broad set of consumers but only follow up with specific victims.[[6]](https://web.archive.org/web/20190717233006/http:/www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/the-elderwood-project.pdf)[[3]](https://blog.avast.com/new-investigations-in-ccleaner-incident-point-to-a-possible-third-stage-that-had-keylogger-capacities)[[5]](https://web.archive.org/web/20160309235002/https://www.commandfive.com/papers/C5_APT_SKHack.pdf) Popular open-source projects that are used as dependencies in many applications may also be targeted as a means to add malicious code to users of the dependency.[[7]](https://www.trendmicro.com/vinfo/dk/security/news/cybercrime-and-digital-threats/hacker-infects-node-js-package-to-steal-from-bitcoin-wallets)

In some cases, adversaries may conduct "second-order" supply chain compromises by leveraging the access gained from an initial supply chain compromise to further compromise a software component.[[8]](https://krebsonsecurity.com/2023/04/3cx-breach-was-a-double-supply-chain-compromise/) This may allow the threat actor to spread to even more victims.

ID: T1195

Sub-techniques:  [T1195.001](https://attack.mitre.org/techniques/T1195/001), [T1195.002](https://attack.mitre.org/techniques/T1195/002), [T1195.003](https://attack.mitre.org/techniques/T1195/003)

ⓘ

Tactic: [Initial Access](https://attack.mitre.org/tactics/TA0001)

ⓘ

Platforms: Linux, SaaS, Windows, macOS

Contributors: Damien Miller-McAndrews; Menachem Goldstein; Veeral Patel

Version: 1.7

Created: 18 April 2018

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1195/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1195/)

## Procedure Examples

| ID  | Name  | Description

|  [G1003](https://attack.mitre.org/groups/G1003)   |  [Ember Bear](https://attack.mitre.org/groups/G1003)   |

[Ember Bear](https://attack.mitre.org/groups/G1003) has compromised information technology providers and software developers providing services to targets of interest, building initial access to ultimate victims at least in part through compromise of service providers that work with the victim organizations.[[9]](https://www.microsoft.com/en-us/security/blog/2023/06/14/cadet-blizzard-emerges-as-a-novel-and-distinct-russian-threat-actor/)

|  [S1213](https://attack.mitre.org/software/S1213)   |  [Lumma Stealer](https://attack.mitre.org/software/S1213)   |

[Lumma Stealer](https://attack.mitre.org/software/S1213) has been delivered through cracked software downloads.[[10]](https://www.cybereason.com/blog/threat-analysis-rise-of-lummastealer)[[11]](https://www.fortinet.com/blog/threat-research/lumma-variant-on-youtube)[[12]](https://www.trendmicro.com/en_us/research/25/a/lumma-stealers-github-based-delivery-via-mdr.html)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) has leveraged compromised organizations to conduct supply chain attacks on government entities.[[13]](https://www.trendmicro.com/en_us/research/24/j/earth-simnavaz-cyberattacks.html)

|  [S1148](https://attack.mitre.org/software/S1148)   |  [Raccoon Stealer](https://attack.mitre.org/software/S1148)   |

[Raccoon Stealer](https://attack.mitre.org/software/S1148) has been distributed through cracked software downloads.[[14]](https://medium.com/s2wblog/raccoon-stealer-is-back-with-a-new-version-5f436e04b20d)

|  [G0034](https://attack.mitre.org/groups/G0034)   |  [Sandworm Team](https://attack.mitre.org/groups/G0034)   |

[Sandworm Team](https://attack.mitre.org/groups/G0034) staged compromised versions of legitimate software installers on forums to achieve initial, untargetetd access in victim environments.[[15]](https://services.google.com/fh/files/misc/apt44-unearthing-sandworm.pdf)

## Mitigations

| ID  | Mitigation  | Description

|  [M1013](https://attack.mitre.org/mitigations/M1013)   |  [Application Developer Guidance](https://attack.mitre.org/mitigations/M1013)   |

Application developers should be cautious when selecting third-party libraries to integrate into their application. Additionally, where possible, developers should lock software dependencies to specific versions rather than pulling the latest version on build.[[16]](https://web.archive.org/web/20220316130828/https://www.cidersecurity.io/top-10-cicd-security-risks/)

|  [M1046](https://attack.mitre.org/mitigations/M1046)   |  [Boot Integrity](https://attack.mitre.org/mitigations/M1046)   |

Use secure methods to boot a system and verify the integrity of the operating system and loading mechanisms.

|  [M1033](https://attack.mitre.org/mitigations/M1033)   |  [Limit Software Installation](https://attack.mitre.org/mitigations/M1033)   |

Where possible, consider requiring developers to pull from internal repositories containing verified and approved packages rather than from external ones.[[16]](https://web.archive.org/web/20220316130828/https://www.cidersecurity.io/top-10-cicd-security-risks/)

|  [M1051](https://attack.mitre.org/mitigations/M1051)   |  [Update Software](https://attack.mitre.org/mitigations/M1051)   |

A patch management process should be implemented to check unused dependencies, unmaintained and/or previously vulnerable dependencies, unnecessary features, components, files, and documentation.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Implement robust user account management practices to limit permissions associated with software execution. Ensure that software runs with the lowest necessary privileges, avoiding the use of root or administrator accounts when possible. By restricting permissions, you can minimize the risk of propagation and unauthorized actions in the event of a supply chain compromise, reducing the attack surface for adversaries to exploit within compromised systems.

|  [M1016](https://attack.mitre.org/mitigations/M1016)   |  [Vulnerability Scanning](https://attack.mitre.org/mitigations/M1016)   |

Continuous monitoring of vulnerability sources and the use of automatic and manual code review tools should also be implemented as well.[[17]](https://www.owasp.org/index.php/Category:OWASP_Top_Ten_Project)

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0537](https://attack.mitre.org/detectionstrategies/DET0537)   |  [Behavioral detection for Supply Chain Compromise (package/update tamper → install → first-run)](https://attack.mitre.org/detectionstrategies/DET0537)   |  [AN1480](https://attack.mitre.org/detectionstrategies/DET0537#AN1480)   |

1) New or updated software is delivered/installed from atypical sources or with signature/hash mismatches; 2) installer/updater writes binaries to unexpected paths or replaces existing signed files; 3) first run causes unsigned/abnormally signed modules to load or child processes to execute, optionally followed by network egress to new destinations.

|  [AN1481](https://attack.mitre.org/detectionstrategies/DET0537#AN1481)   |

1) Package manager or curl/wget installs/upgrades from non-approved repos or unsigned packages; 2) new ELF written into PATH directories or replacement of existing binaries/libraries; 3) first run leads to unexpected child processes or outbound connections.

|  [AN1482](https://attack.mitre.org/detectionstrategies/DET0537#AN1482)   |

1) pkg/notarization installs from atypical sources or with Gatekeeper/AMFI warnings; 2) new Mach-O written into /Applications or ~/Library paths or substitution of signed components; 3) first run from installer spawns unsigned children or exfil.

## References

-    [IBM Support. (2017, April 26). Storwize USB Initialization Tool may contain malicious code. Retrieved May 28, 2019.](https://www-01.ibm.com/support/docview.wss?uid=ssg1S1010146&myns=s028&mynp=OCSTHGUJ&mynp=OCSTLM5A&mynp=OCSTLM6B&mynp=OCHW206&mync=E&cm_sp=s028-_-OCSTHGUJ-OCSTLM5A-OCSTLM6B-OCHW206-_-E)
-    [Schneider Electric. (2018, August 24). Security Notification – USB Removable Media Provided With Conext Combox and Conext Battery Monitor. Retrieved May 28, 2019.](https://www.se.com/us/en/download/document/SESN-2018-236-01/)
-    [Avast Threat Intelligence Team. (2018, March 8). New investigations into the CCleaner incident point to a possible third stage that had keylogger capacities. Retrieved March 15, 2018.](https://blog.avast.com/new-investigations-in-ccleaner-incident-point-to-a-possible-third-stage-that-had-keylogger-capacities)
-    [Windows Defender Research. (2018, March 7). Behavior monitoring combined with machine learning spoils a massive Dofoil coin mining campaign. Retrieved March 20, 2018.](https://cloudblogs.microsoft.com/microsoftsecure/2018/03/07/behavior-monitoring-combined-with-machine-learning-spoils-a-massive-dofoil-coin-mining-campaign/)
-    [Command Five Pty Ltd. (2011, September). SK Hack by an Advanced Persistent Threat. Retrieved November 17, 2024.](https://web.archive.org/web/20160309235002/https://www.commandfive.com/papers/C5_APT_SKHack.pdf)
-    [O'Gorman, G., and McDonald, G.. (2012, September 6). The Elderwood Project. Retrieved November 17, 2024.](https://web.archive.org/web/20190717233006/http:/www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/the-elderwood-project.pdf)
-    [Trendmicro. (2018, November 29). Hacker Infects Node.js Package to Steal from Bitcoin Wallets. Retrieved April 10, 2019.](https://www.trendmicro.com/vinfo/dk/security/news/cybercrime-and-digital-threats/hacker-infects-node-js-package-to-steal-from-bitcoin-wallets)
-    [Brian Krebs. (2023, April 20). 3CX Breach Was a Double Supply Chain Compromise. Retrieved May 22, 2025.](https://krebsonsecurity.com/2023/04/3cx-breach-was-a-double-supply-chain-compromise/)
-    [Microsoft Threat Intelligence. (2023, June 14). Cadet Blizzard emerges as a novel and distinct Russian threat actor. Retrieved July 10, 2023.](https://www.microsoft.com/en-us/security/blog/2023/06/14/cadet-blizzard-emerges-as-a-novel-and-distinct-russian-threat-actor/)

-    [Cybereaon Security Services Team. (n.d.). Your Data Is Under New Lummanagement: The Rise of LummaStealer. Retrieved March 22, 2025.](https://www.cybereason.com/blog/threat-analysis-rise-of-lummastealer)
-    [Cara Lin, Fortinet. (2024, January 8). Deceptive Cracked Software Spreads Lumma Variant on YouTube. Retrieved March 22, 2025.](https://www.fortinet.com/blog/threat-research/lumma-variant-on-youtube)
-    [Buddy Tancio, Fe Cureg, and Jovit Samaniego, Trend Micro. (2025, January 30). Lumma Stealer’s GitHub-Based Delivery Explored via Managed Detection and Response. Retrieved March 22, 2025.](https://www.trendmicro.com/en_us/research/25/a/lumma-stealers-github-based-delivery-via-mdr.html)
-    [Fahmy, M. et al. (2024, October 11). Earth Simnavaz (aka APT34) Levies Advanced Cyberattacks Against Middle East. Retrieved November 27, 2024.](https://www.trendmicro.com/en_us/research/24/j/earth-simnavaz-cyberattacks.html)
-    [S2W TALON. (2022, June 16). Raccoon Stealer is Back with a New Version. Retrieved August 1, 2024.](https://medium.com/s2wblog/raccoon-stealer-is-back-with-a-new-version-5f436e04b20d)
-    [Roncone, G. et al. (n.d.). APT44: Unearthing Sandworm. Retrieved July 11, 2024.](https://services.google.com/fh/files/misc/apt44-unearthing-sandworm.pdf)
-    [Daniel Krivelevich and Omer Gil. (n.d.). Top 10 CI/CD Security Risks. Retrieved November 17, 2024.](https://web.archive.org/web/20220316130828/https://www.cidersecurity.io/top-10-cicd-security-risks/)
-    [OWASP. (2018, February 23). OWASP Top Ten Project. Retrieved April 3, 2018.](https://www.owasp.org/index.php/Category:OWASP_Top_Ten_Project)

×
