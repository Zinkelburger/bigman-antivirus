---
url: "https://attack.mitre.org/techniques/T1219/"
title: "Remote Access Tools, Technique T1219 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:42:50Z"
via: "live"
sha256: "ca4d1997562aab2f8d357da6e911053eeeb0e549560e82e36794f6dd3dda6ade"
wayback: "https://web.archive.org/web/20261023102114/https://attack.mitre.org/techniques/T1219/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Remote Access Tools, Technique T1219 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1219/>
> Snapshot: <https://web.archive.org/web/20261023102114/https://attack.mitre.org/techniques/T1219/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Remote Access Tools

#  Remote Access Tools

##### Sub-techniques (3)

| ID  | Name

|  [T1219.001](https://attack.mitre.org/techniques/T1219/001/)   |  [IDE Tunneling](https://attack.mitre.org/techniques/T1219/001/)

|  [T1219.002](https://attack.mitre.org/techniques/T1219/002/)   |  [Remote Desktop Software](https://attack.mitre.org/techniques/T1219/002/)

|  [T1219.003](https://attack.mitre.org/techniques/T1219/003/)   |  [Remote Access Hardware](https://attack.mitre.org/techniques/T1219/003/)

An adversary may use legitimate remote access tools to establish an interactive command and control channel within a network. Remote access tools create a session between two trusted hosts through a graphical interface, a command line interaction, a protocol tunnel via development or management software, or hardware-level access such as KVM (Keyboard, Video, Mouse) over IP solutions. Desktop support software (usually graphical interface) and remote management software (typically command line interface) allow a user to control a computer remotely as if they are a local user inheriting the user or software permissions. This software is commonly used for troubleshooting, software installation, and system management.[[1]](https://www.symantec.com/content/dam/symantec/docs/security-center/white-papers/istr-living-off-the-land-and-fileless-attack-techniques-en.pdf)[[2]](https://go.crowdstrike.com/rs/281-OBQ-266/images/15GlobalThreatReport.pdf)[[3]](https://blog.crysys.hu/2013/03/teamspy/) Adversaries may similarly abuse response features included in EDR and other defensive tools that enable remote access.

Remote access tools may be installed and used post-compromise as an alternate communications channel for redundant access or to establish an interactive remote desktop session with the target system. It may also be used as a malware component to establish a reverse connection or back-connect to a service or adversary-controlled system.

Installation of many remote access tools may also include persistence (e.g., the software's installation routine creates a [Windows Service](https://attack.mitre.org/techniques/T1543/003)). Remote access modules/features may also exist as part of otherwise existing software (e.g., Google Chrome’s Remote Desktop).[[4]](https://support.google.com/chrome/answer/1649523)[[5]](https://www.huntress.com/blog/slashandgrab-screen-connect-post-exploitation-in-the-wild-cve-2024-1709-cve-2024-1708)

ID: T1219

Sub-techniques:  [T1219.001](https://attack.mitre.org/techniques/T1219/001), [T1219.002](https://attack.mitre.org/techniques/T1219/002), [T1219.003](https://attack.mitre.org/techniques/T1219/003)

ⓘ

Tactic: [Command and Control](https://attack.mitre.org/tactics/TA0011)

ⓘ

Platforms: Linux, Windows, macOS

Contributors: Dray Agha, @Purp1eW0lf, Huntress Labs; Matt Kelly, @breakersall; Zachary Stanford, @svch0st

Version: 3.0

Created: 18 April 2018

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1219/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1219/)

## Procedure Examples

| ID  | Name  | Description

|  [G1024](https://attack.mitre.org/groups/G1024)   |  [Akira](https://attack.mitre.org/groups/G1024)   |

[Akira](https://attack.mitre.org/groups/G1024) uses legitimate utilities such as AnyDesk and PuTTy for maintaining remote access to victim environments.[[6]](https://www.secureworks.com/research/threat-profiles/gold-sahara)[[7]](https://arcticwolf.com/resources/blog/conti-and-akira-chained-together/)

|  [G1043](https://attack.mitre.org/groups/G1043)   |  [BlackByte](https://attack.mitre.org/groups/G1043)   |

[BlackByte](https://attack.mitre.org/groups/G1043) has used tools such as AnyDesk in victim environments.[[8]](https://www.picussecurity.com/resource/ttps-used-by-blackbyte-ransomware-targeting-critical-infrastructure)[[9]](https://www.microsoft.com/en-us/security/blog/2023/07/06/the-five-day-job-a-blackbyte-ransomware-intrusion-case-study/)

|  [S0030](https://attack.mitre.org/software/S0030)   |  [Carbanak](https://attack.mitre.org/software/S0030)   |

[Carbanak](https://attack.mitre.org/software/S0030) has a plugin for VNC and Ammyy Admin Tool.[[10]](https://www.fireeye.com/blog/threat-research/2017/06/behind-the-carbanak-backdoor.html)

|  [G0008](https://attack.mitre.org/groups/G0008)   |  [Carbanak](https://attack.mitre.org/groups/G0008)   |

[Carbanak](https://attack.mitre.org/groups/G0008) used legitimate programs such as AmmyyAdmin and Team Viewer for remote interactive C2 to target systems.[[11]](http://www.group-ib.com/files/Anunak_APT_against_financial_institutions.pdf)

|  [G0080](https://attack.mitre.org/groups/G0080)   |  [Cobalt Group](https://attack.mitre.org/groups/G0080)   |

[Cobalt Group](https://attack.mitre.org/groups/G0080) used the Ammyy Admin tool as well as TeamViewer for remote access, including to preserve remote access if a Cobalt Strike module was lost.[[12]](https://www.ptsecurity.com/upload/corporate/ww-en/analytics/Cobalt-2017-eng.pdf)[[13]](https://www.ptsecurity.com/upload/corporate/ww-en/analytics/Cobalt-Snatch-eng.pdf)[[14]](https://www.group-ib.com/blog/cobalt)

|  [G0105](https://attack.mitre.org/groups/G0105)   |  [DarkVishnya](https://attack.mitre.org/groups/G0105)   |

[DarkVishnya](https://attack.mitre.org/groups/G0105) used DameWare Mini Remote Control for lateral movement.[[15]](https://securelist.com/darkvishnya/89169/)

|  [S0384](https://attack.mitre.org/software/S0384)   |  [Dridex](https://attack.mitre.org/software/S0384)   |

[Dridex](https://attack.mitre.org/software/S0384) contains a module for VNC.[[16]](https://www.secureworks.com/research/dridex-bugat-v5-botnet-takeover-operation)

|  [S0554](https://attack.mitre.org/software/S0554)   |  [Egregor](https://attack.mitre.org/software/S0554)   |

[Egregor](https://attack.mitre.org/software/S0554) has checked for the LogMein event log in an attempt to encrypt files in remote machines.[[17]](https://cybleinc.com/2020/10/31/egregor-ransomware-a-deep-dive-into-its-activities-and-techniques/)

|  [G0046](https://attack.mitre.org/groups/G0046)   |  [FIN7](https://attack.mitre.org/groups/G0046)   |

[FIN7](https://attack.mitre.org/groups/G0046) has utilized the remote management tool Atera to download malware to a compromised system.[[18]](https://www.mandiant.com/resources/evolution-of-fin7)

|  [G0115](https://attack.mitre.org/groups/G0115)   |  [GOLD SOUTHFIELD](https://attack.mitre.org/groups/G0115)   |

[GOLD SOUTHFIELD](https://attack.mitre.org/groups/G0115) has used the cloud-based remote management and monitoring tool "ConnectWise Control" to deploy [REvil](https://attack.mitre.org/software/S0496).[[19]](https://web.archive.org/web/20210414101816/https://tetradefense.com/incident-response-services/cause-and-effect-sodinokibi-ransomware-analysis/)

|  [S0601](https://attack.mitre.org/software/S0601)   |  [Hildegard](https://attack.mitre.org/software/S0601)   |

[Hildegard](https://attack.mitre.org/software/S0601) has established tmate sessions for C2 communications.[[20]](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)

|  [G1032](https://attack.mitre.org/groups/G1032)   |  [INC Ransom](https://attack.mitre.org/groups/G1032)   |

[INC Ransom](https://attack.mitre.org/groups/G1032) has used AnyDesk and PuTTY on compromised systems.[[21]](https://www.huntress.com/blog/investigating-new-inc-ransom-group-activity)[[22]](https://socradar.io/dark-web-profile-inc-ransom/)[[23]](https://www.huntress.com/blog/lolbin-to-inc-ransomware)[[24]](https://www.sentinelone.com/anthology/inc-ransom/)

|  [S1245](https://attack.mitre.org/software/S1245)   |  [InvisibleFerret](https://attack.mitre.org/software/S1245)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has utilized remote access software including AnyDesk client through the "adc" module.[[25]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[26]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[27]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also downloaded the AnyDesk client should it not already exist on the compromised host by searching for C:/Program Files(x86)/AnyDesk/AnyDesk.exe.[[28]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  [G1051](https://attack.mitre.org/groups/G1051)   |  [Medusa Group](https://attack.mitre.org/groups/G1051)   |

[Medusa Group](https://attack.mitre.org/groups/G1051) has leveraged Remote Access Software for lateral movement and data exfiltration.[[29]](https://unit42.paloaltonetworks.com/medusa-ransomware-escalation-new-leak-site/)[[30]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-071a)[[31]](https://www.security.com/threat-intelligence/medusa-ransomware-attacks)[[32]](https://securityscorecard.com/wp-content/uploads/2024/01/deep-dive-into-medusa-ransomware.pdf) [Medusa Group](https://attack.mitre.org/groups/G1051) has also been known to utilize Remote Access Software such as AnyDesk, Atera, ConnectWise, eHorus, N-Able, PDQ Deploy, PDQ Inventory, SimpleHelp and Splashtop.[[30]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-071a)

|  [C0002](https://attack.mitre.org/campaigns/C0002)   |  [Night Dragon](https://attack.mitre.org/campaigns/C0002)   |

During [Night Dragon](https://attack.mitre.org/campaigns/C0002), threat actors used several remote administration tools as persistent infiltration channels.[[33]](https://scadahacker.com/library/Documents/Cyber_Events/McAfee%20-%20Night%20Dragon%20-%20Global%20Energy%20Cyberattacks.pdf)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) has incorporated remote monitoring and management (RMM) tools into their operations including [ngrok](https://attack.mitre.org/software/S0508).[[34]](https://www.trendmicro.com/en_us/research/24/j/earth-simnavaz-cyberattacks.html)

|  [C0060](https://attack.mitre.org/campaigns/C0060)   |  [Operation AkaiRyū](https://attack.mitre.org/campaigns/C0060)   |

During [Operation AkaiRyū](https://attack.mitre.org/campaigns/C0060), [MirrorFace](https://attack.mitre.org/groups/G1054) used remote access tools including PuTTY.[[35]](https://www.welivesecurity.com/en/eset-research/operation-akairyu-mirrorface-invites-europe-expo-2025-revives-anel-backdoor/)

|  [S0148](https://attack.mitre.org/software/S0148)   |  [RTM](https://attack.mitre.org/software/S0148)   |

[RTM](https://attack.mitre.org/software/S0148) has the capability to download a VNC module from command and control (C2).[[36]](https://www.welivesecurity.com/wp-content/uploads/2017/02/Read-The-Manual.pdf)

|  [G0034](https://attack.mitre.org/groups/G0034)   |  [Sandworm Team](https://attack.mitre.org/groups/G0034)   |

[Sandworm Team](https://attack.mitre.org/groups/G0034) has used remote administration tools or remote industrial control system client software for execution and to maliciously release electricity breakers.[[37]](https://www.us-cert.gov/ics/alerts/IR-ALERT-H-16-056-01)[[38]](https://www.microsoft.com/en-us/security/blog/2022/10/14/new-prestige-ransomware-impacts-organizations-in-ukraine-and-poland/)

|  [G1057](https://attack.mitre.org/groups/G1057)   |  [ShinyHunters](https://attack.mitre.org/groups/G1057)   |

[ShinyHunters](https://attack.mitre.org/groups/G1057) has used MeshCentral and [ConnectWise](https://attack.mitre.org/software/S0591) to gain initial access, to run administrative command queries and to deploy the custom lateral movement and defacement script [victim_abbreviation]_fanout.sh.[[39]](https://cloud.google.com/blog/topics/threat-intelligence/shinyhunters-targets-education-sector-oracle-exploit)[[40]](https://blog.eclecticiq.com/shinyhunters-calling-financially-motivated-data-extortion-group-targeting-enterprise-cloud-applications)

|  [G0139](https://attack.mitre.org/groups/G0139)   |  [TeamTNT](https://attack.mitre.org/groups/G0139)   |

[TeamTNT](https://attack.mitre.org/groups/G0139) has established tmate sessions for C2 communications.[[20]](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)[[41]](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/)

|  [S0266](https://attack.mitre.org/software/S0266)   |  [TrickBot](https://attack.mitre.org/software/S0266)   |

[TrickBot](https://attack.mitre.org/software/S0266) uses vncDll module to remote control the victim machine.[[42]](https://www.welivesecurity.com/2020/10/12/eset-takes-part-global-operation-disrupt-trickbot/)[[43]](https://www.bitdefender.com/files/News/CaseStudies/study/316/Bitdefender-Whitepaper-TrickBot-en-EN-interactive.pdf)

## Mitigations

| ID  | Mitigation  | Description

|  [M1042](https://attack.mitre.org/mitigations/M1042)   |  [Disable or Remove Feature or Program](https://attack.mitre.org/mitigations/M1042)   |

Consider disabling unnecessary remote connection functionality, including both unapproved software installations and specific features built into supported applications.

|  [M1038](https://attack.mitre.org/mitigations/M1038)   |  [Execution Prevention](https://attack.mitre.org/mitigations/M1038)   |

Use application control to mitigate installation and use of unapproved software that can be used for remote access.

|  [M1037](https://attack.mitre.org/mitigations/M1037)   |  [Filter Network Traffic](https://attack.mitre.org/mitigations/M1037)   |

Properly configure firewalls, application firewalls, and proxies to limit outgoing traffic to sites and services used by remote access software.

|  [M1034](https://attack.mitre.org/mitigations/M1034)   |  [Limit Hardware Installation](https://attack.mitre.org/mitigations/M1034)   |

Block the use of IP-based KVM devices within the network if they are not required.

|  [M1031](https://attack.mitre.org/mitigations/M1031)   |  [Network Intrusion Prevention](https://attack.mitre.org/mitigations/M1031)   |

Network intrusion detection and prevention systems that use network signatures may be able to prevent traffic to remote access services.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0496](https://attack.mitre.org/detectionstrategies/DET0496)   |  [Behavior-Chain Detection for Remote Access Tools (Tool-Agnostic)](https://attack.mitre.org/detectionstrategies/DET0496)   |  [AN1366](https://attack.mitre.org/detectionstrategies/DET0496#AN1366)   |

Chain of remote access tool behavior: (1) initial execution of remote-control/assist agent or GUI under user context; (2) persistence via service or autorun; (3) long-lived outbound connection/tunnel to external infrastructure; (4) interactive control signals such as shell or file-manager child processes spawned by the RAT parent.

|  [AN1367](https://attack.mitre.org/detectionstrategies/DET0496#AN1367)   |

Sequence of RAT agent execution, systemd persistence, and long-lived external egress; optional interactive shells spawned from the agent.

|  [AN1368](https://attack.mitre.org/detectionstrategies/DET0496#AN1368)   |

Electron/GUI or headless RAT execution followed by LaunchAgent/Daemon persistence and persistent external connections; interactive children (osascript/sh/curl) spawned by parent.

## References

-    [Wueest, C., Anand, H. (2017, July). Living off the land and fileless attack techniques. Retrieved April 10, 2018.](https://www.symantec.com/content/dam/symantec/docs/security-center/white-papers/istr-living-off-the-land-and-fileless-attack-techniques-en.pdf)
-    [CrowdStrike Intelligence. (2016). 2015 Global Threat Report. Retrieved April 11, 2018.](https://go.crowdstrike.com/rs/281-OBQ-266/images/15GlobalThreatReport.pdf)
-    [CrySyS Lab. (2013, March 20). TeamSpy – Obshie manevri. Ispolzovat’ tolko s razreshenija S-a. Retrieved April 11, 2018.](https://blog.crysys.hu/2013/03/teamspy/)
-    [Google. (n.d.). Retrieved March 14, 2024.](https://support.google.com/chrome/answer/1649523)
-    [Huntress. (n.d.). Retrieved March 14, 2024.](https://www.huntress.com/blog/slashandgrab-screen-connect-post-exploitation-in-the-wild-cve-2024-1709-cve-2024-1708)
-    [Secureworks. (n.d.). GOLD SAHARA. Retrieved February 20, 2024.](https://www.secureworks.com/research/threat-profiles/gold-sahara)
-    [Steven Campbell, Akshay Suthar, & Connor Belfiorre. (2023, July 26). Conti and Akira: Chained Together. Retrieved February 20, 2024.](https://arcticwolf.com/resources/blog/conti-and-akira-chained-together/)
-    [Huseyin Can Yuceel. (2022, February 21). TTPs used by BlackByte Ransomware Targeting Critical Infrastructure. Retrieved December 16, 2024.](https://www.picussecurity.com/resource/ttps-used-by-blackbyte-ransomware-targeting-critical-infrastructure)
-    [Microsoft Incident Response. (2023, July 6). The five-day job: A BlackByte ransomware intrusion case study. Retrieved December 16, 2024.](https://www.microsoft.com/en-us/security/blog/2023/07/06/the-five-day-job-a-blackbyte-ransomware-intrusion-case-study/)
-    [Bennett, J., Vengerik, B. (2017, June 12). Behind the CARBANAK Backdoor. Retrieved June 11, 2018.](https://www.fireeye.com/blog/threat-research/2017/06/behind-the-carbanak-backdoor.html)
-    [Group-IB and Fox-IT. (2014, December). Anunak: APT against financial institutions. Retrieved April 20, 2016.](http://www.group-ib.com/files/Anunak_APT_against_financial_institutions.pdf)
-    [Positive Technologies. (2017, August 16). Cobalt Strikes Back: An Evolving Multinational Threat to Finance. Retrieved September 5, 2018.](https://www.ptsecurity.com/upload/corporate/ww-en/analytics/Cobalt-2017-eng.pdf)
-    [Positive Technologies. (2016, December 16). Cobalt Snatch. Retrieved October 9, 2018.](https://www.ptsecurity.com/upload/corporate/ww-en/analytics/Cobalt-Snatch-eng.pdf)
-    [Matveeva, V. (2017, August 15). Secrets of Cobalt. Retrieved October 10, 2018.](https://www.group-ib.com/blog/cobalt)
-    [Golovanov, S. (2018, December 6). DarkVishnya: Banks attacked through direct connection to local network. Retrieved May 15, 2020.](https://securelist.com/darkvishnya/89169/)
-    [Dell SecureWorks Counter Threat Unit Threat Intelligence. (2015, October 13). Dridex (Bugat v5) Botnet Takeover Operation. Retrieved May 31, 2019.](https://www.secureworks.com/research/dridex-bugat-v5-botnet-takeover-operation)
-    [Cybleinc. (2020, October 31). Egregor Ransomware – A Deep Dive Into Its Activities and Techniques. Retrieved December 29, 2020.](https://cybleinc.com/2020/10/31/egregor-ransomware-a-deep-dive-into-its-activities-and-techniques/)
-    [Abdo, B., et al. (2022, April 4). FIN7 Power Hour: Adversary Archaeology and the Evolution of FIN7. Retrieved April 5, 2022.](https://www.mandiant.com/resources/evolution-of-fin7)
-    [Tetra Defense. (2020, March). CAUSE AND EFFECT: SODINOKIBI RANSOMWARE ANALYSIS. Retrieved November 17, 2024.](https://web.archive.org/web/20210414101816/https://tetradefense.com/incident-response-services/cause-and-effect-sodinokibi-ransomware-analysis/)
-    [Chen, J. et al. (2021, February 3). Hildegard: New TeamTNT Cryptojacking Malware Targeting Kubernetes. Retrieved April 5, 2021.](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)
-    [Team Huntress. (2023, August 11). Investigating New INC Ransom Group Activity. Retrieved June 5, 2024.](https://www.huntress.com/blog/investigating-new-inc-ransom-group-activity)
-    [SOCRadar. (2024, January 24). Dark Web Profile: INC Ransom. Retrieved June 5, 2024.](https://socradar.io/dark-web-profile-inc-ransom/)

-    [Carvey, H. (2024, May 1). LOLBin to INC Ransomware. Retrieved June 5, 2024.](https://www.huntress.com/blog/lolbin-to-inc-ransomware)
-    [SentinelOne. (n.d.). What Is Inc. Ransomware?. Retrieved June 5, 2024.](https://www.sentinelone.com/anthology/inc-ransom/)
-    [eSentire Threat Response Unit (TRU). (2024, November 14). Bored BeaverTail & InvisibleFerret Yacht Club – A Lazarus Lure Pt.2. Retrieved October 17, 2025.](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)
-    [Matej Havranek. (2025, February 20). DeceptiveDevelopment targets freelance developers. Retrieved October 17, 2025.](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)
-    [Unit 42. (2023, November 21). Hacking Employers and Seeking Employment: Two Job-Related Campaigns Bear Hallmarks of North Korean Threat Actors. Retrieved October 17, 2025.](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)
-    [Seongsu Park. (2024, November 4). From Pyongyang to Your Payroll: The Rise of North Korean Remote Workers in the West. Retrieved October 17, 2025.](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)
-    [Anthony Galiette, Doel Santos. (2024, January 11). Medusa Ransomware Turning Your Files into Stone. Retrieved October 15, 2025.](https://unit42.paloaltonetworks.com/medusa-ransomware-escalation-new-leak-site/)
-    [Cybersecurity and Infrastructure Security Agency. (2025, March 12). AA25-071A #StopRansomware: Medusa Ransomware. Retrieved October 15, 2025.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-071a)
-    [Threat Hunter Team Symantec and Carbon Black. (2025, March 6). Medusa Ransomware Activity Continues to Increase. Retrieved October 15, 2025.](https://www.security.com/threat-intelligence/medusa-ransomware-attacks)
-    [Vlad Pasca. (2024, January 1). A Deep Dive into Medusa Ransomware. Retrieved October 15, 2025.](https://securityscorecard.com/wp-content/uploads/2024/01/deep-dive-into-medusa-ransomware.pdf)
-    [McAfee® Foundstone® Professional Services and McAfee Labs™. (2011, February 10). Global Energy Cyberattacks: “Night Dragon”. Retrieved February 19, 2018.](https://scadahacker.com/library/Documents/Cyber_Events/McAfee%20-%20Night%20Dragon%20-%20Global%20Energy%20Cyberattacks.pdf)
-    [Fahmy, M. et al. (2024, October 11). Earth Simnavaz (aka APT34) Levies Advanced Cyberattacks Against Middle East. Retrieved November 27, 2024.](https://www.trendmicro.com/en_us/research/24/j/earth-simnavaz-cyberattacks.html)
-    [Dominik Breitenbacher. (2025, March 18). Operation AkaiRyū: MirrorFace invites Europe to Expo 2025 and revives ANEL backdoor. Retrieved May 22, 2025.](https://www.welivesecurity.com/en/eset-research/operation-akairyu-mirrorface-invites-europe-expo-2025-revives-anel-backdoor/)
-    [Faou, M. and Boutin, J. (2017, February). Read The Manual: A Guide to the RTM Banking Trojan. Retrieved March 9, 2017.](https://www.welivesecurity.com/wp-content/uploads/2017/02/Read-The-Manual.pdf)
-    [US-CERT. (2016, February 25). ICS Alert (IR-ALERT-H-16-056-01) Cyber-Attack Against Ukrainian Critical Infrastructure. Retrieved June 10, 2020.](https://www.us-cert.gov/ics/alerts/IR-ALERT-H-16-056-01)
-    [MSTIC. (2022, October 14). New “Prestige” ransomware impacts organizations in Ukraine and Poland. Retrieved January 19, 2023.](https://www.microsoft.com/en-us/security/blog/2022/10/14/new-prestige-ransomware-impacts-organizations-in-ukraine-and-poland/)
-    [Mandiant, Google Threat Intelligence Group. (2026, June 11). ShinyHunters Targets Education Sector with Oracle PeopleSoft Exploit. Retrieved June 11, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/shinyhunters-targets-education-sector-oracle-exploit)
-    [Büyükkaya, A. (2025, September 22). ShinyHunters Calling: Financially Motivated Data Extortion Group Targeting Enterprise Cloud Applications. Retrieved May 18, 2026.](https://blog.eclecticiq.com/shinyhunters-calling-financially-motivated-data-extortion-group-targeting-enterprise-cloud-applications)
-    [Darin Smith. (2022, April 21). TeamTNT targeting AWS, Alibaba. Retrieved August 4, 2022.](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/)
-    [Boutin, J. (2020, October 12). ESET takes part in global operation to disrupt Trickbot. Retrieved March 15, 2021.](https://www.welivesecurity.com/2020/10/12/eset-takes-part-global-operation-disrupt-trickbot/)
-    [Tudorica, R., Maximciuc, A., Vatamanu, C. (2020, March 18). New TrickBot Module Bruteforces RDP Connections, Targets Select Telecommunication Services in US and Hong Kong. Retrieved March 15, 2021.](https://www.bitdefender.com/files/News/CaseStudies/study/316/Bitdefender-Whitepaper-TrickBot-en-EN-interactive.pdf)

×
