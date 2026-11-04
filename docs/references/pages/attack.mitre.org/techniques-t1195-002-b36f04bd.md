---
url: "https://attack.mitre.org/techniques/T1195/002/"
title: "Supply Chain Compromise: Compromise Software Supply Chain, Sub-technique T1195.002 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:39:28Z"
via: "live"
sha256: "eae1da994652ab823741e5bbd74fd556020bbf3555f36940273ab552c6090bb9"
wayback: "https://web.archive.org/web/20261016115626/https://attack.mitre.org/techniques/T1195/002/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
  - "docs/threats/05-malware-families-and-detection.md"
---

# Supply Chain Compromise: Compromise Software Supply Chain, Sub-technique T1195.002 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1195/002/>
> Snapshot: <https://web.archive.org/web/20261016115626/https://attack.mitre.org/techniques/T1195/002/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Supply Chain Compromise](https://attack.mitre.org/techniques/T1195)
- Compromise Software Supply Chain

#  Supply Chain Compromise: Compromise Software Supply Chain

##### Other sub-techniques of Supply Chain Compromise (3)

| ID  | Name

|  [T1195.001](https://attack.mitre.org/techniques/T1195/001/)   |  [Compromise Software Dependencies and Development Tools](https://attack.mitre.org/techniques/T1195/001/)

|  T1195.002   |  Compromise Software Supply Chain

|  [T1195.003](https://attack.mitre.org/techniques/T1195/003/)   |  [Compromise Hardware Supply Chain](https://attack.mitre.org/techniques/T1195/003/)

Adversaries may manipulate application software prior to receipt by a final consumer for the purpose of data or system compromise. Supply chain compromise of software can take place in a number of ways, including manipulation of the application source code, manipulation of the update/distribution mechanism for that software, or replacing compiled releases with a modified version.

Targeting may be specific to a desired victim set or may be distributed to a broad set of consumers but only move on to additional tactics on specific victims.[[1]](https://blog.avast.com/new-investigations-in-ccleaner-incident-point-to-a-possible-third-stage-that-had-keylogger-capacities)[[2]](https://web.archive.org/web/20160309235002/https://www.commandfive.com/papers/C5_APT_SKHack.pdf)

ID: T1195.002

Sub-technique of:  [T1195](https://attack.mitre.org/techniques/T1195)

ⓘ

Tactic: [Initial Access](https://attack.mitre.org/tactics/TA0001)

ⓘ

Platforms: Linux, Windows, macOS

Version: 1.1

Created: 11 March 2020

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1195/002/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1195/002/)

## Procedure Examples

| ID  | Name  | Description

|  [C0057](https://attack.mitre.org/campaigns/C0057)   |  [3CX Supply Chain Attack](https://attack.mitre.org/campaigns/C0057)   |

During the [3CX Supply Chain Attack](https://attack.mitre.org/campaigns/C0057), [AppleJeus](https://attack.mitre.org/groups/G1049) first compromised an "end-of-life" trading software application which was downloaded and executed inside the 3CX enterprise environment. The second compromise modified the Windows and macOS build environments used to distribute the 3CX software to their customer base.[[3]](https://cloud.google.com/blog/topics/threat-intelligence/3cx-software-supply-chain-compromise/)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) gained access to production environments where they could inject malicious code into legitimate, signed files and widely distribute them to end users.[[4]](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)

|  [S0222](https://attack.mitre.org/software/S0222)   |  [CCBkdr](https://attack.mitre.org/software/S0222)   |

[CCBkdr](https://attack.mitre.org/software/S0222) was added to a legitimate, signed version 5.33 of the CCleaner software and distributed on CCleaner's distribution site.[[5]](http://blog.talosintelligence.com/2017/09/avast-distributes-malware.html)[[6]](http://www.intezer.com/evidence-aurora-operation-still-active-supply-chain-attack-through-ccleaner/)[[1]](https://blog.avast.com/new-investigations-in-ccleaner-incident-point-to-a-possible-third-stage-that-had-keylogger-capacities)

|  [G0080](https://attack.mitre.org/groups/G0080)   |  [Cobalt Group](https://attack.mitre.org/groups/G0080)   |

[Cobalt Group](https://attack.mitre.org/groups/G0080) has compromised legitimate web browser updates to deliver a backdoor. [[7]](https://go.crowdstrike.com/rs/281-OBQ-266/images/Report2020CrowdStrikeGlobalThreatReport.pdf)

|  [G1034](https://attack.mitre.org/groups/G1034)   |  [Daggerfly](https://attack.mitre.org/groups/G1034)   |

[Daggerfly](https://attack.mitre.org/groups/G1034) is associated with several supply chain compromises using malicious updates to compromise victims.[[8]](https://www.welivesecurity.com/2023/04/26/evasive-panda-apt-group-malware-updates-popular-chinese-software/)[[9]](https://www.welivesecurity.com/en/eset-research/evasive-panda-leverages-monlam-festival-target-tibetans/)

|  [G0035](https://attack.mitre.org/groups/G0035)   |  [Dragonfly](https://attack.mitre.org/groups/G0035)   |

[Dragonfly](https://attack.mitre.org/groups/G0035) has placed trojanized installers for control system software on legitimate vendor app stores.[[10]](https://www.secureworks.com/research/resurgent-iron-liberty-targeting-energy-sector)[[11]](https://vblocalhost.com/uploads/VB2021-Slowik.pdf)

|  [G0046](https://attack.mitre.org/groups/G0046)   |  [FIN7](https://attack.mitre.org/groups/G0046)   |

[FIN7](https://attack.mitre.org/groups/G0046) has gained initial access by compromising a victim's software supply chain.[[12]](https://www.mandiant.com/resources/evolution-of-fin7)

|  [G0115](https://attack.mitre.org/groups/G0115)   |  [GOLD SOUTHFIELD](https://attack.mitre.org/groups/G0115)   |

[GOLD SOUTHFIELD](https://attack.mitre.org/groups/G0115) has distributed ransomware by backdooring software installers via a strategic web compromise of the site hosting Italian WinRAR.[[13]](https://www.secureworks.com/research/revil-sodinokibi-ransomware)[[14]](https://www.secureworks.com/blog/revil-the-gandcrab-connection)[[15]](https://www.secureworks.com/research/threat-profiles/gold-southfield)

|  [S0493](https://attack.mitre.org/software/S0493)   |  [GoldenSpy](https://attack.mitre.org/software/S0493)   |

[GoldenSpy](https://attack.mitre.org/software/S0493) has been packaged with a legitimate tax preparation software.[[16]](https://www.trustwave.com/en-us/resources/library/documents/the-golden-tax-department-and-the-emergence-of-goldenspy-malware/)

|  [G1036](https://attack.mitre.org/groups/G1036)   |  [Moonstone Sleet](https://attack.mitre.org/groups/G1036)   |

[Moonstone Sleet](https://attack.mitre.org/groups/G1036) has distributed a trojanized version of PuTTY software for initial access to victims.[[17]](https://www.microsoft.com/en-us/security/blog/2024/05/28/moonstone-sleet-emerges-as-new-north-korean-threat-actor-with-new-bag-of-tricks/)

|  [G0034](https://attack.mitre.org/groups/G0034)   |  [Sandworm Team](https://attack.mitre.org/groups/G0034)   |

[Sandworm Team](https://attack.mitre.org/groups/G0034) has distributed [NotPetya](https://attack.mitre.org/software/S0368) by compromising the legitimate Ukrainian accounting software M.E.Doc and replacing a legitimate software update with a malicious one.[[18]](https://www.secureworks.com/blog/notpetya-campaign-what-we-know-about-the-latest-global-ransomware-attack)[[19]](https://www.welivesecurity.com/2017/06/30/telebots-back-supply-chain-attacks-against-ukraine/)[[20]](https://www.justice.gov/opa/press-release/file/1328521/download)

|  [C0024](https://attack.mitre.org/campaigns/C0024)   |  [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024)   |

During the [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024), [APT29](https://attack.mitre.org/groups/G0016) gained initial network access to some victims via a trojanized update of SolarWinds Orion software.[[21]](https://orangematter.solarwinds.com/2021/01/11/new-findings-from-our-investigation-of-sunburst/)[[22]](https://www.fireeye.com/blog/threat-research/2020/12/evasive-attacker-leverages-solarwinds-supply-chain-compromises-with-sunburst-backdoor.html)[[23]](https://www.ncsc.gov.uk/files/Advisory-further-TTPs-associated-with-SVR-cyber-actors.pdf)[[24]](https://www.microsoft.com/security/blog/2021/01/20/deep-dive-into-the-solorigate-second-stage-activation-from-sunburst-to-teardrop-and-raindrop/)

|  [S0562](https://attack.mitre.org/software/S0562)   |  [SUNSPOT](https://attack.mitre.org/software/S0562)   |

[SUNSPOT](https://attack.mitre.org/software/S0562) malware was designed and used to insert [SUNBURST](https://attack.mitre.org/software/S0559) into software builds of the SolarWinds Orion IT management product.[[25]](https://www.crowdstrike.com/blog/sunspot-malware-technical-analysis/)

|  [G0027](https://attack.mitre.org/groups/G0027)   |  [Threat Group-3390](https://attack.mitre.org/groups/G0027)   |

[Threat Group-3390](https://attack.mitre.org/groups/G0027) has compromised the Able Desktop installer to gain access to victim's environments.[[26]](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)

## Mitigations

| ID  | Mitigation  | Description

|  [M1051](https://attack.mitre.org/mitigations/M1051)   |  [Update Software](https://attack.mitre.org/mitigations/M1051)   |

A patch management process should be implemented to check unused applications, unmaintained and/or previously vulnerable software, unnecessary features, components, files, and documentation.

|  [M1016](https://attack.mitre.org/mitigations/M1016)   |  [Vulnerability Scanning](https://attack.mitre.org/mitigations/M1016)   |

Continuous monitoring of vulnerability sources and the use of automatic and manual code review tools should also be implemented as well.[[27]](https://www.owasp.org/index.php/Category:OWASP_Top_Ten_Project)

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0309](https://attack.mitre.org/detectionstrategies/DET0309)   |  [Compromised software/update chain (installer/write → first-run/child → egress/signature anomaly)](https://attack.mitre.org/detectionstrategies/DET0309)   |  [AN0862](https://attack.mitre.org/detectionstrategies/DET0309#AN0862)   |

Adversary ships a tampered application or update: an updater/installer (msiexec/setup/update.exe/vendor service) writes or replaces binaries; on first run it spawns scripts/shells or unsigned DLLs and beacons to non-approved update CDNs/hosts. Detection correlates: (1) process creation of installer/updater → (2) file metadata changes in program paths → (3) first-run children and module/signature anomalies → (4) outbound connections to unexpected hosts within a short window.

|  [AN0863](https://attack.mitre.org/detectionstrategies/DET0309#AN0863)   |

A compromised package/update (deb/rpm/tarball/AppImage/vendor updater) is installed, writing/overwriting files in /usr/local/bin, /usr/bin, /opt, or ~/.local; first run executes unexpected shells/curl/wget and connects to unapproved hosts. Correlate package/updater execution → file writes/replace → first-run child processes → egress.

|  [AN0864](https://attack.mitre.org/detectionstrategies/DET0309#AN0864)   |

A tampered app/pkg/notarized update is installed via installer, softwareupdated, Homebrew, or vendor updater; new Mach-O or bundle contents appear in /Applications, /Library, /usr/local or /opt/homebrew; first run spawns sh/zsh/osascript/curl and makes egress to unfamiliar domains; AMFI/Gatekeeper may log signature/notarization problems.

## References

-    [Avast Threat Intelligence Team. (2018, March 8). New investigations into the CCleaner incident point to a possible third stage that had keylogger capacities. Retrieved March 15, 2018.](https://blog.avast.com/new-investigations-in-ccleaner-incident-point-to-a-possible-third-stage-that-had-keylogger-capacities)
-    [Command Five Pty Ltd. (2011, September). SK Hack by an Advanced Persistent Threat. Retrieved November 17, 2024.](https://web.archive.org/web/20160309235002/https://www.commandfive.com/papers/C5_APT_SKHack.pdf)
-    [Jeff Johnson, Fred Plan, Adrian Sanchez, Renato Fontana, Jake Nicastro, Dimiter Andonov, Marius Fodoreanu, Daniel Scott. (2023, April 20). 3CX Software Supply Chain Compromise Initiated by a Prior Software Supply Chain Compromise; Suspected North Korean Actor Responsible. Retrieved August 25, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/3cx-software-supply-chain-compromise/)
-    [Fraser, N., et al. (2019, August 7). Double DragonAPT41, a dual espionage and cyber crime operation APT41. Retrieved September 23, 2019.](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)
-    [Brumaghin, E. et al. (2017, September 18). CCleanup: A Vast Number of Machines at Risk. Retrieved March 9, 2018.](http://blog.talosintelligence.com/2017/09/avast-distributes-malware.html)
-    [Rosenberg, J. (2017, September 20). Evidence Aurora Operation Still Active: Supply Chain Attack Through CCleaner. Retrieved February 13, 2018.](http://www.intezer.com/evidence-aurora-operation-still-active-supply-chain-attack-through-ccleaner/)
-    [Crowdstrike. (2020, March 2). 2020 Global Threat Report. Retrieved December 11, 2020.](https://go.crowdstrike.com/rs/281-OBQ-266/images/Report2020CrowdStrikeGlobalThreatReport.pdf)
-    [Facundo Muñoz. (2023, April 26). Evasive Panda APT group delivers malware via updates for popular Chinese software. Retrieved July 25, 2024.](https://www.welivesecurity.com/2023/04/26/evasive-panda-apt-group-malware-updates-popular-chinese-software/)
-    [Ahn Ho, Facundo Muñoz, & Marc-Etienne M.Léveillé. (2024, March 7). Evasive Panda leverages Monlam Festival to target Tibetans. Retrieved July 25, 2024.](https://www.welivesecurity.com/en/eset-research/evasive-panda-leverages-monlam-festival-target-tibetans/)
-    [Secureworks. (2019, July 24). Resurgent Iron Liberty Targeting Energy Sector. Retrieved August 12, 2020.](https://www.secureworks.com/research/resurgent-iron-liberty-targeting-energy-sector)
-    [Slowik, J. (2021, October). THE BAFFLING BERSERK BEAR: A DECADE’S ACTIVITY TARGETING CRITICAL INFRASTRUCTURE. Retrieved December 6, 2021.](https://vblocalhost.com/uploads/VB2021-Slowik.pdf)
-    [Abdo, B., et al. (2022, April 4). FIN7 Power Hour: Adversary Archaeology and the Evolution of FIN7. Retrieved April 5, 2022.](https://www.mandiant.com/resources/evolution-of-fin7)
-    [Counter Threat Unit Research Team. (2019, September 24). REvil/Sodinokibi Ransomware. Retrieved August 4, 2020.](https://www.secureworks.com/research/revil-sodinokibi-ransomware)
-    [Secureworks . (2019, September 24). REvil: The GandCrab Connection. Retrieved August 4, 2020.](https://www.secureworks.com/blog/revil-the-gandcrab-connection)

-    [Secureworks. (n.d.). GOLD SOUTHFIELD. Retrieved October 6, 2020.](https://www.secureworks.com/research/threat-profiles/gold-southfield)
-    [Trustwave SpiderLabs. (2020, June 25). The Golden Tax Department and Emergence of GoldenSpy Malware. Retrieved July 23, 2020.](https://www.trustwave.com/en-us/resources/library/documents/the-golden-tax-department-and-the-emergence-of-goldenspy-malware/)
-    [Microsoft Threat Intelligence. (2024, May 28). Moonstone Sleet emerges as new North Korean threat actor with new bag of tricks. Retrieved August 26, 2024.](https://www.microsoft.com/en-us/security/blog/2024/05/28/moonstone-sleet-emerges-as-new-north-korean-threat-actor-with-new-bag-of-tricks/)
-    [Counter Threat Research Team. (2017, June 28). NotPetya Campaign: What We Know About the Latest Global Ransomware Attack. Retrieved June 11, 2020.](https://www.secureworks.com/blog/notpetya-campaign-what-we-know-about-the-latest-global-ransomware-attack)
-    [Cherepanov, A.. (2017, June 30). TeleBots are back: Supply chain attacks against Ukraine. Retrieved June 11, 2020.](https://www.welivesecurity.com/2017/06/30/telebots-back-supply-chain-attacks-against-ukraine/)
-    [Scott W. Brady. (2020, October 15). United States vs. Yuriy Sergeyevich Andrienko et al.. Retrieved November 25, 2020.](https://www.justice.gov/opa/press-release/file/1328521/download)
-    [Sudhakar Ramakrishna . (2021, January 11). New Findings From Our Investigation of SUNBURST. Retrieved January 13, 2021.](https://orangematter.solarwinds.com/2021/01/11/new-findings-from-our-investigation-of-sunburst/)
-    [FireEye. (2020, December 13). Highly Evasive Attacker Leverages SolarWinds Supply Chain to Compromise Multiple Global Victims With SUNBURST Backdoor. Retrieved January 4, 2021.](https://www.fireeye.com/blog/threat-research/2020/12/evasive-attacker-leverages-solarwinds-supply-chain-compromises-with-sunburst-backdoor.html)
-    [NCSC, CISA, FBI, NSA. (2021, May 7). Further TTPs associated with SVR cyber actors. Retrieved July 29, 2021.](https://www.ncsc.gov.uk/files/Advisory-further-TTPs-associated-with-SVR-cyber-actors.pdf)
-    [MSTIC, CDOC, 365 Defender Research Team. (2021, January 20). Deep dive into the Solorigate second-stage activation: From SUNBURST to TEARDROP and Raindrop . Retrieved January 22, 2021.](https://www.microsoft.com/security/blog/2021/01/20/deep-dive-into-the-solorigate-second-stage-activation-from-sunburst-to-teardrop-and-raindrop/)
-    [CrowdStrike Intelligence Team. (2021, January 11). SUNSPOT: An Implant in the Build Process. Retrieved January 11, 2021.](https://www.crowdstrike.com/blog/sunspot-malware-technical-analysis/)
-    [Lunghi, D. and Lu, K. (2021, April 9). Iron Tiger APT Updates Toolkit With Evolved SysUpdate Malware. Retrieved November 12, 2021.](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)
-    [OWASP. (2018, February 23). OWASP Top Ten Project. Retrieved April 3, 2018.](https://www.owasp.org/index.php/Category:OWASP_Top_Ten_Project)

×
