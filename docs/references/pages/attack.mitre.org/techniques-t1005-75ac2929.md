---
url: "https://attack.mitre.org/techniques/T1005/"
title: "Data from Local System, Technique T1005 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:24:07Z"
via: "live"
sha256: "da407c6e5d92c260812faa714376d84b979f6ab3aac3a097bfe8f6f4f9bba3c8"
wayback: "https://web.archive.org/web/20261023025719/https://attack.mitre.org/techniques/T1005/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Data from Local System, Technique T1005 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1005/>
> Snapshot: <https://web.archive.org/web/20261023025719/https://attack.mitre.org/techniques/T1005/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Data from Local System

#  Data from Local System

Adversaries may search local system sources, such as file systems, configuration files, local databases, virtual machine files, or process memory, to find files of interest and sensitive data prior to Exfiltration.

Adversaries may do this using a [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059), such as [cmd](https://attack.mitre.org/software/S0106) as well as a [Network Device CLI](https://attack.mitre.org/techniques/T1059/008), which have functionality to interact with the file system to gather information.[[1]](https://www.cisco.com/c/en/us/td/docs/ios-xml/ios/fundamentals/command/cf_command_ref/show_protocols_through_showmon.html#wp2760878733) Adversaries may also use [Automated Collection](https://attack.mitre.org/techniques/T1119) on the local system.

ID: T1005

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Collection](https://attack.mitre.org/tactics/TA0009)

ⓘ

Platforms: ESXi, Linux, Network Devices, Windows, macOS

Contributors: Austin Clark, @c2defense; Liran Ravich, CardinalOps; William Cain

Version: 1.8

Created: 31 May 2017

Last Modified: 11 September 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1005/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1005/)

## Procedure Examples

| ID  | Name  | Description

|  [S1028](https://attack.mitre.org/software/S1028)   |  [Action RAT](https://attack.mitre.org/software/S1028)   |

[Action RAT](https://attack.mitre.org/software/S1028) can collect local data from an infected machine.[[2]](https://www.malwarebytes.com/blog/news/2021/12/sidecopy-apt-connecting-lures-to-victims-payloads-to-infrastructure)

|  [G1030](https://attack.mitre.org/groups/G1030)   |  [Agrius](https://attack.mitre.org/groups/G1030)   |

[Agrius](https://attack.mitre.org/groups/G1030) gathered data from database and other critical servers in victim environments, then used wiping mechanisms as an anti-analysis and anti-forensics mechanism.[[3]](https://unit42.paloaltonetworks.com/agonizing-serpens-targets-israeli-tech-higher-ed-sectors/)

|  [S1025](https://attack.mitre.org/software/S1025)   |  [Amadey](https://attack.mitre.org/software/S1025)   |

[Amadey](https://attack.mitre.org/software/S1025) can collect information from a compromised host.[[4]](https://blogs.blackberry.com/en/2020/01/threat-spotlight-amadey-bot)

|  [G0138](https://attack.mitre.org/groups/G0138)   |  [Andariel](https://attack.mitre.org/groups/G0138)   |

[Andariel](https://attack.mitre.org/groups/G0138) has collected large numbers of files from compromised network systems for later extraction.[[5]](https://fsiceat.tistory.com/2)

|  [C0062](https://attack.mitre.org/campaigns/C0062)   |  [Anthropic AI-orchestrated Campaign](https://attack.mitre.org/campaigns/C0062)   |

During the [Anthropic AI-orchestrated Campaign](https://attack.mitre.org/campaigns/C0062), the adversary tasked Claude Code to automatically gather sensitive data stored within the local system to include credentials, system configurations and sensitive operational data.[[6]](https://assets.anthropic.com/m/ec212e6566a0d47/original/Disrupting-the-first-reported-AI-orchestrated-cyber-espionage-campaign.pdf)

|  [S0622](https://attack.mitre.org/software/S0622)   |  [AppleSeed](https://attack.mitre.org/software/S0622)   |

[AppleSeed](https://attack.mitre.org/software/S0622) can collect data on a compromised host.[[7]](https://blog.malwarebytes.com/threat-analysis/2021/06/kimsuky-apt-continues-to-target-south-korean-government-using-appleseed-backdoor/)[[8]](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)

|  [G0006](https://attack.mitre.org/groups/G0006)   |  [APT1](https://attack.mitre.org/groups/G0006)   |

[APT1](https://attack.mitre.org/groups/G0006) has collected files from a local victim.[[9]](https://www.fireeye.com/content/dam/fireeye-www/services/pdfs/mandiant-apt1-report.pdf)

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[APT28](https://attack.mitre.org/groups/G0007) has retrieved internal documents from machines inside victim environments, including by using [Forfiles](https://attack.mitre.org/software/S0193) to stage documents before exfiltration.[[10]](https://netzpolitik.org/2015/digital-attack-on-german-parliament-investigative-report-on-the-hack-of-the-left-party-infrastructure-in-bundestag/)[[11]](https://cdn.cnn.com/cnn/2018/images/07/13/gru.indictment.pdf)[[12]](https://documents.trendmicro.com/assets/white_papers/wp-pawn-storm-in-2019.pdf)[[13]](https://media.defense.gov/2021/Jul/01/2002753896/-1/-1/1/CSA_GRU_GLOBAL_BRUTE_FORCE_CAMPAIGN_UOO158036-21.PDF)

|  [G0016](https://attack.mitre.org/groups/G0016)   |  [APT29](https://attack.mitre.org/groups/G0016)   |

[APT29](https://attack.mitre.org/groups/G0016) has stolen data from compromised hosts.[[14]](https://www.mandiant.com/resources/blog/unc3524-eye-spy-email)

|  [G0022](https://attack.mitre.org/groups/G0022)   |  [APT3](https://attack.mitre.org/groups/G0022)   |

[APT3](https://attack.mitre.org/groups/G0022) will identify Microsoft Office documents on the victim's computer.[[15]](http://carnal0wnage.attackresearch.com/2012/09/more-on-aptsim.html)

|  [G0067](https://attack.mitre.org/groups/G0067)   |  [APT37](https://attack.mitre.org/groups/G0067)   |

[APT37](https://attack.mitre.org/groups/G0067) has collected data from victims' local systems.[[16]](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has collected data from a compromised host.[[17]](https://us-cert.cisa.gov/ncas/alerts/aa20-239a)

|  [G0087](https://attack.mitre.org/groups/G0087)   |  [APT39](https://attack.mitre.org/groups/G0087)   |

[APT39](https://attack.mitre.org/groups/G0087) has used various tools to steal files from the compromised host.[[18]](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/chafer-latest-attacks-reveal-heightened-ambitions)[[19]](https://www.iranwatch.org/sites/default/files/public-intelligence-alert.pdf)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) has uploaded files and data from a compromised host.[[20]](https://www.group-ib.com/blog/colunmtk-apt41/)

|  [G0143](https://attack.mitre.org/groups/G0143)   |  [Aquatic Panda](https://attack.mitre.org/groups/G0143)   |

[Aquatic Panda](https://attack.mitre.org/groups/G0143) captured local Windows security event log data from victim machines using the wevtutil utility to extract contents to an evtx output file.[[21]](https://go.crowdstrike.com/rs/281-OBQ-266/images/2022OverWatchThreatHuntingReport.pdf)

|  [S1029](https://attack.mitre.org/software/S1029)   |  [AuTo Stealer](https://attack.mitre.org/software/S1029)   |

[AuTo Stealer](https://attack.mitre.org/software/S1029) can collect data such as PowerPoint files, Word documents, Excel files, PDF files, text files, database files, and image files from an infected machine.[[2]](https://www.malwarebytes.com/blog/news/2021/12/sidecopy-apt-connecting-lures-to-victims-payloads-to-infrastructure)

|  [G0001](https://attack.mitre.org/groups/G0001)   |  [Axiom](https://attack.mitre.org/groups/G0001)   |

[Axiom](https://attack.mitre.org/groups/G0001) has collected data from a compromised network.[[22]](https://web.archive.org/web/20230115144216/http://www.novetta.com/wp-content/uploads/2014/11/Executive_Summary-Final_1.pdf)

|  [S0642](https://attack.mitre.org/software/S0642)   |  [BADFLICK](https://attack.mitre.org/software/S0642)   |

[BADFLICK](https://attack.mitre.org/software/S0642) has uploaded files from victims' machines.[[23]](https://web.archive.org/web/20210722005437/https://www.accenture.com/_acnmedia/PDF-96/Accenture-Security-MUDCARP.pdf)

|  [S0128](https://attack.mitre.org/software/S0128)   |  [BADNEWS](https://attack.mitre.org/software/S0128)   |

When it first starts, [BADNEWS](https://attack.mitre.org/software/S0128) crawls the victim's local drives and collects documents with the following extensions: .doc, .docx, .pdf, .ppt, .pptx, and .txt.[[24]](https://www.forcepoint.com/sites/default/files/resources/files/forcepoint-security-labs-monsoon-analysis-report.pdf)[[25]](https://researchcenter.paloaltonetworks.com/2018/03/unit42-patchwork-continues-deliver-badnews-indian-subcontinent/)

|  [S0337](https://attack.mitre.org/software/S0337)   |  [BadPatch](https://attack.mitre.org/software/S0337)   |

[BadPatch](https://attack.mitre.org/software/S0337) collects files from the local system that have the following extensions, then prepares them for exfiltration: .xls, .xlsx, .pdf, .mdb, .rar, .zip, .doc, .docx.[[26]](https://researchcenter.paloaltonetworks.com/2017/10/unit42-badpatch/)

|  [S0234](https://attack.mitre.org/software/S0234)   |  [Bandook](https://attack.mitre.org/software/S0234)   |

[Bandook](https://attack.mitre.org/software/S0234) can collect local files from the system .[[27]](https://research.checkpoint.com/2020/bandook-signed-delivered/)

|  [S0239](https://attack.mitre.org/software/S0239)   |  [Bankshot](https://attack.mitre.org/software/S0239)   |

[Bankshot](https://attack.mitre.org/software/S0239) collects files from the local system.[[28]](https://securingtomorrow.mcafee.com/mcafee-labs/hidden-cobra-targets-turkish-financial-sector-new-bankshot-implant/)

|  [S0534](https://attack.mitre.org/software/S0534)   |  [Bazar](https://attack.mitre.org/software/S0534)   |

[Bazar](https://attack.mitre.org/software/S0534) can retrieve information from the infected machine.[[29]](https://www.cybereason.com/blog/a-bazar-of-tricks-following-team9s-development-cycles)

|  [S1246](https://attack.mitre.org/software/S1246)   |  [BeaverTail](https://attack.mitre.org/software/S1246)   |

[BeaverTail](https://attack.mitre.org/software/S1246) has exfiltrated data collected from local systems.[[30]](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)[[31]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[32]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[33]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)

|  [S0268](https://attack.mitre.org/software/S0268)   |  [Bisonal](https://attack.mitre.org/software/S0268)   |

[Bisonal](https://attack.mitre.org/software/S0268) has collected information from a compromised host.[[34]](https://blog.talosintelligence.com/2020/03/bisonal-10-years-of-play.html)

|  [S0564](https://attack.mitre.org/software/S0564)   |  [BlackMould](https://attack.mitre.org/software/S0564)   |

[BlackMould](https://attack.mitre.org/software/S0564) can copy files on a compromised host.[[35]](https://www.microsoft.com/security/blog/2019/12/12/gallium-targeting-global-telecom/)

|  [S0520](https://attack.mitre.org/software/S0520)   |  [BLINDINGCAN](https://attack.mitre.org/software/S0520)   |

[BLINDINGCAN](https://attack.mitre.org/software/S0520) has uploaded files from victim machines.[[36]](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-232a)

|  [S0651](https://attack.mitre.org/software/S0651)   |  [BoxCaon](https://attack.mitre.org/software/S0651)   |

[BoxCaon](https://attack.mitre.org/software/S0651) can upload files from a compromised host.[[37]](https://research.checkpoint.com/2021/indigozebra-apt-continues-to-attack-central-asia-with-evolving-tools/)

|  [S9015](https://attack.mitre.org/software/S9015)   |  [BRICKSTORM](https://attack.mitre.org/software/S9015)   |

[BRICKSTORM](https://attack.mitre.org/software/S9015) has commands that allow the actor download files from the compromised host to the C2 server, and to also download specific sections of a file.[[38]](https://www.cisa.gov/news-events/analysis-reports/ar25-338a)

|  [G0060](https://attack.mitre.org/groups/G0060)   |  [BRONZE BUTLER](https://attack.mitre.org/groups/G0060)   |

[BRONZE BUTLER](https://attack.mitre.org/groups/G0060) has exfiltrated files stolen from local systems.[[39]](https://www.secureworks.com/research/bronze-butler-targets-japanese-businesses)

|  [S1063](https://attack.mitre.org/software/S1063)   |  [Brute Ratel C4](https://attack.mitre.org/software/S1063)   |

[Brute Ratel C4](https://attack.mitre.org/software/S1063) has the ability to upload files from a compromised system.[[40]](https://unit42.paloaltonetworks.com/brute-ratel-c4-tool/)

|  [S1039](https://attack.mitre.org/software/S1039)   |  [Bumblebee](https://attack.mitre.org/software/S1039)   |

[Bumblebee](https://attack.mitre.org/software/S1039) can capture and compress stolen credentials from the Registry and volume shadow copies.[[41]](https://www.cybereason.com/blog/threat-analysis-report-bumblebee-loader-the-high-road-to-enterprise-domain-control)

|  [C0015](https://attack.mitre.org/campaigns/C0015)   |  [C0015](https://attack.mitre.org/campaigns/C0015)   |

During [C0015](https://attack.mitre.org/campaigns/C0015), the threat actors obtained files and data from the compromised network.[[42]](https://thedfirreport.com/2021/11/29/continuing-the-bazar-ransomware-story/)

|  [C0017](https://attack.mitre.org/campaigns/C0017)   |  [C0017](https://attack.mitre.org/campaigns/C0017)   |

During [C0017](https://attack.mitre.org/campaigns/C0017), [APT41](https://attack.mitre.org/groups/G0096) collected information related to compromised machines as well as Personal Identifiable Information (PII) from victim networks.[[43]](https://www.mandiant.com/resources/apt41-us-state-governments)

|  [C0026](https://attack.mitre.org/campaigns/C0026)   |  [C0026](https://attack.mitre.org/campaigns/C0026)   |

During [C0026](https://attack.mitre.org/campaigns/C0026), the threat actors collected documents from compromised hosts.[[44]](https://www.mandiant.com/resources/blog/turla-galaxy-opportunity)

|  [S0274](https://attack.mitre.org/software/S0274)   |  [Calisto](https://attack.mitre.org/software/S0274)   |

[Calisto](https://attack.mitre.org/software/S0274) can collect data from user directories.[[45]](https://securelist.com/calisto-trojan-for-macos/86543/)

|  [S1224](https://attack.mitre.org/software/S1224)   |  [CASTLETAP](https://attack.mitre.org/software/S1224)   |

[CASTLETAP](https://attack.mitre.org/software/S1224) can execute a C2 command to transfer files from victim machines.[[46]](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)

|  [S0572](https://attack.mitre.org/software/S0572)   |  [Caterpillar WebShell](https://attack.mitre.org/software/S0572)   |

[Caterpillar WebShell](https://attack.mitre.org/software/S0572) has a module to collect information from the local database.[[47]](https://www.clearskysec.com/wp-content/uploads/2021/01/Lebanese-Cedar-APT.pdf)

|  [S1043](https://attack.mitre.org/software/S1043)   |  [ccf32](https://attack.mitre.org/software/S1043)   |

[ccf32](https://attack.mitre.org/software/S1043) can collect files from a compromised host.[[48]](https://www.bitdefender.com/files/News/CaseStudies/study/379/Bitdefender-Whitepaper-Chinese-APT.pdf)

|  [S0674](https://attack.mitre.org/software/S0674)   |  [CharmPower](https://attack.mitre.org/software/S0674)   |

[CharmPower](https://attack.mitre.org/software/S0674) can collect data and files from a compromised host.[[49]](https://research.checkpoint.com/2022/apt35-exploits-log4j-vulnerability-to-distribute-new-modular-powershell-toolkit/)

|  [S1149](https://attack.mitre.org/software/S1149)   |  [CHIMNEYSWEEP](https://attack.mitre.org/software/S1149)   |

[CHIMNEYSWEEP](https://attack.mitre.org/software/S1149) can collect files from compromised hosts.[[50]](https://cloud.google.com/blog/topics/threat-intelligence/likely-iranian-threat-actor-conducts-politically-motivated-disruptive-activity-against/)

|  [S0020](https://attack.mitre.org/software/S0020)   |  [China Chopper](https://attack.mitre.org/software/S0020)   |

[China Chopper](https://attack.mitre.org/software/S0020)'s server component can upload local files.[[51]](https://www.fireeye.com/blog/threat-research/2018/03/suspected-chinese-espionage-group-targeting-maritime-and-engineering-industries.html)[[52]](https://www.fireeye.com/blog/threat-research/2013/08/breaking-down-the-china-chopper-web-shell-part-i.html)[[53]](https://www.ncsc.gov.uk/report/joint-report-on-publicly-available-hacking-tools)[[54]](https://www.rapid7.com/blog/post/2021/03/23/defending-against-the-zero-day-analyzing-attacker-behavior-post-exploitation-of-microsoft-exchange/)

|  [S0667](https://attack.mitre.org/software/S0667)   |  [Chrommme](https://attack.mitre.org/software/S0667)   |

[Chrommme](https://attack.mitre.org/software/S0667) can collect data from a local system.[[55]](https://www.welivesecurity.com/wp-content/uploads/2021/06/eset_gelsemium.pdf)

|  [S0660](https://attack.mitre.org/software/S0660)   |  [Clambling](https://attack.mitre.org/software/S0660)   |

[Clambling](https://attack.mitre.org/software/S0660) can collect information from a compromised host.[[56]](https://documents.trendmicro.com/assets/white_papers/wp-uncovering-DRBcontrol.pdf)

|  [S0154](https://attack.mitre.org/software/S0154)   |  [Cobalt Strike](https://attack.mitre.org/software/S0154)   |

[Cobalt Strike](https://attack.mitre.org/software/S0154) can collect data from a local system.[[57]](https://web.archive.org/web/20210924171429/https://www.cobaltstrike.com/downloads/reports/tacticstechniquesandprocedures.pdf)[[58]](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)

|  [S0492](https://attack.mitre.org/software/S0492)   |  [CookieMiner](https://attack.mitre.org/software/S0492)   |

[CookieMiner](https://attack.mitre.org/software/S0492) has retrieved iPhone text messages from iTunes phone backup files.[[59]](https://unit42.paloaltonetworks.com/mac-malware-steals-cryptocurrency-exchanges-cookies/)

|  [S0050](https://attack.mitre.org/software/S0050)   |  [CosmicDuke](https://attack.mitre.org/software/S0050)   |

[CosmicDuke](https://attack.mitre.org/software/S0050) steals user files from local hard drives with file extensions that match a predefined list.[[60]](https://blog.f-secure.com/wp-content/uploads/2019/10/CosmicDuke.pdf)

|  [C0004](https://attack.mitre.org/campaigns/C0004)   |  [CostaRicto](https://attack.mitre.org/campaigns/C0004)   |

During [CostaRicto](https://attack.mitre.org/campaigns/C0004), the threat actors collected data and files from compromised networks.[[61]](https://blogs.blackberry.com/en/2020/11/the-costaricto-campaign-cyber-espionage-outsourced)

|  [S1023](https://attack.mitre.org/software/S1023)   |  [CreepyDrive](https://attack.mitre.org/software/S1023)   |

[CreepyDrive](https://attack.mitre.org/software/S1023) can upload files to C2 from victim machines.[[62]](https://www.microsoft.com/security/blog/2022/06/02/exposing-polonium-activity-and-infrastructure-targeting-israeli-organizations/)

|  [S0115](https://attack.mitre.org/software/S0115)   |  [Crimson](https://attack.mitre.org/software/S0115)   |

[Crimson](https://attack.mitre.org/software/S0115) can collect information from a compromised host.[[63]](https://blog.talosintelligence.com/2022/07/transparent-tribe-targets-education.html)

|  [S0538](https://attack.mitre.org/software/S0538)   |  [Crutch](https://attack.mitre.org/software/S0538)   |

[Crutch](https://attack.mitre.org/software/S0538) can exfiltrate files from compromised systems.[[64]](https://www.welivesecurity.com/2020/12/02/turla-crutch-keeping-back-door-open/)

|  [S0498](https://attack.mitre.org/software/S0498)   |  [Cryptoistic](https://attack.mitre.org/software/S0498)   |

[Cryptoistic](https://attack.mitre.org/software/S0498) can retrieve files from the local file system.[[65]](https://www.sentinelone.com/blog/four-distinct-families-of-lazarus-malware-target-apples-macos-platform/)

|  [G1012](https://attack.mitre.org/groups/G1012)   |  [CURIUM](https://attack.mitre.org/groups/G1012)   |

[CURIUM](https://attack.mitre.org/groups/G1012) has exfiltrated data from a compromised machine.[[66]](https://www.microsoft.com/en-us/security/blog/2021/11/16/evolving-trends-in-iranian-threat-actor-activity-mstic-presentation-at-cyberwarcon-2021)

|  [C0029](https://attack.mitre.org/campaigns/C0029)   |  [Cutting Edge](https://attack.mitre.org/campaigns/C0029)   |

During [Cutting Edge](https://attack.mitre.org/campaigns/C0029), threat actors stole the running configuration and cache data from targeted Ivanti Connect Secure VPNs.[[67]](https://www.volexity.com/blog/2024/01/10/active-exploitation-of-two-zero-day-vulnerabilities-in-ivanti-connect-secure-vpn/)[[68]](https://www.mandiant.com/resources/blog/investigating-ivanti-zero-day-exploitation)

|  [S0687](https://attack.mitre.org/software/S0687)   |  [Cyclops Blink](https://attack.mitre.org/software/S0687)   |

[Cyclops Blink](https://attack.mitre.org/software/S0687) can upload files from a compromised host.[[69]](https://www.ncsc.gov.uk/files/Cyclops-Blink-Malware-Analysis-Report.pdf)

|  [S1014](https://attack.mitre.org/software/S1014)   |  [DanBot](https://attack.mitre.org/software/S1014)   |

[DanBot](https://attack.mitre.org/software/S1014) can upload files from compromised hosts.[[70]](https://www.secureworks.com/blog/lyceum-takes-center-stage-in-middle-east-campaign)

|  [G0070](https://attack.mitre.org/groups/G0070)   |  [Dark Caracal](https://attack.mitre.org/groups/G0070)   |

[Dark Caracal](https://attack.mitre.org/groups/G0070) collected complete contents of the 'Pictures' folder from compromised Windows systems.[[71]](https://info.lookout.com/rs/051-ESQ-475/images/Lookout_Dark-Caracal_srr_20180118_us_v.1.0.pdf)

|  [S1111](https://attack.mitre.org/software/S1111)   |  [DarkGate](https://attack.mitre.org/software/S1111)   |

[DarkGate](https://attack.mitre.org/software/S1111) has stolen sitemanager.xml and recentservers.xml from %APPDATA%\FileZilla\ if present.[[72]](https://www.rapid7.com/blog/post/2024/12/04/black-basta-ransomware-campaign-drops-zbot-darkgate-and-custom-malware/)

|  [S0673](https://attack.mitre.org/software/S0673)   |  [DarkWatchman](https://attack.mitre.org/software/S0673)   |

[DarkWatchman](https://attack.mitre.org/software/S0673) can collect files from a compromised host.[[73]](https://web.archive.org/web/20220629230035/https://www.prevailion.com/darkwatchman-new-fileless-techniques/)

|  [S9065](https://attack.mitre.org/software/S9065)   |  [DCSYNCER.SLICK](https://attack.mitre.org/software/S9065)   |

[DCSYNCER.SLICK](https://attack.mitre.org/software/S9065) has gathered stolen credentials in hardcoded filenames and pathways to include C:\users\public\LOG.txt and C:\Program Files\VMware\VMware Tools\VMware VGAuth\LOG.txt.[[74]](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)

|  [S1021](https://attack.mitre.org/software/S1021)   |  [DnsSystem](https://attack.mitre.org/software/S1021)   |

[DnsSystem](https://attack.mitre.org/software/S1021) can upload files from infected machines after receiving a command with uploaddd in the string.[[75]](https://www.zscaler.com/blogs/security-research/lyceum-net-dns-backdoor)

|  [G0035](https://attack.mitre.org/groups/G0035)   |  [Dragonfly](https://attack.mitre.org/groups/G0035)   |

[Dragonfly](https://attack.mitre.org/groups/G0035) has collected data from local victim systems.[[76]](https://www.us-cert.gov/ncas/alerts/TA18-074A)

|  [S0694](https://attack.mitre.org/software/S0694)   |  [DRATzarus](https://attack.mitre.org/software/S0694)   |

[DRATzarus](https://attack.mitre.org/software/S0694) can collect information from a compromised host.[[77]](https://www.clearskysec.com/wp-content/uploads/2020/08/Dream-Job-Campaign.pdf)

|  [S0502](https://attack.mitre.org/software/S0502)   |  [Drovorub](https://attack.mitre.org/software/S0502)   |

[Drovorub](https://attack.mitre.org/software/S0502) can transfer files from the victim machine.[[78]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

|  [S0567](https://attack.mitre.org/software/S0567)   |  [Dtrack](https://attack.mitre.org/software/S0567)   |

[Dtrack](https://attack.mitre.org/software/S0567) can collect a variety of information from victim machines.[[79]](https://www.cyberbit.com/blog/endpoint-security/dtrack-apt-malware-found-in-nuclear-power-plant/)

|  [S1159](https://attack.mitre.org/software/S1159)   |  [DUSTTRAP](https://attack.mitre.org/software/S1159)   |

[DUSTTRAP](https://attack.mitre.org/software/S1159) can gather data from infected systems.[[80]](https://cloud.google.com/blog/topics/threat-intelligence/apt41-arisen-from-dust)

|  [G1003](https://attack.mitre.org/groups/G1003)   |  [Ember Bear](https://attack.mitre.org/groups/G1003)   |

[Ember Bear](https://attack.mitre.org/groups/G1003) gathers victim system information such as enumerating the volume of a given device or extracting system and security event logs for analysis.[[81]](https://www.microsoft.com/en-us/security/blog/2023/06/14/cadet-blizzard-emerges-as-a-novel-and-distinct-russian-threat-actor/)[[82]](https://www.cisa.gov/sites/default/files/2024-09/aa24-249a-russian-military-cyber-actors-target-us-and-global-critical-infrastructure.pdf)

|  [S0634](https://attack.mitre.org/software/S0634)   |  [EnvyScout](https://attack.mitre.org/software/S0634)   |

[EnvyScout](https://attack.mitre.org/software/S0634) can collect sensitive NTLM material from a compromised host.[[83]](https://www.microsoft.com/security/blog/2021/05/28/breaking-down-nobeliums-latest-early-stage-toolset/)

|  [S0404](https://attack.mitre.org/software/S0404)   |  [esentutl](https://attack.mitre.org/software/S0404)   |

[esentutl](https://attack.mitre.org/software/S0404) can be used to collect data from local file systems.[[84]](https://resource.redcanary.com/rs/003-YRU-314/images/2021-Threat-Detection-Report.pdf?mkt_tok=MDAzLVlSVS0zMTQAAAF_PIlmhNTaG2McG4X_foM-cIr20UfyB12MIQ10W0HbtMRwxGOJaD0Xj6CRTNg_S-8KniRxtf9xzhz_ACvm_TpbJAIgWCV8yIsFgbhb8cuaZA)

|  [S0512](https://attack.mitre.org/software/S0512)   |  [FatDuke](https://attack.mitre.org/software/S0512)   |

[FatDuke](https://attack.mitre.org/software/S0512) can copy files and directories from a compromised host.[[85]](https://www.welivesecurity.com/wp-content/uploads/2019/10/ESET_Operation_Ghost_Dukes.pdf)

|  [G1016](https://attack.mitre.org/groups/G1016)   |  [FIN13](https://attack.mitre.org/groups/G1016)   |

[FIN13](https://attack.mitre.org/groups/G1016) has gathered stolen credentials, sensitive data such as point-of-sale (POS), and ATM data from a compromised network before exfiltration.[[86]](https://www.mandiant.com/resources/blog/fin13-cybercriminal-mexico)[[87]](https://web.archive.org/web/20220105132433/https://f.hubspotusercontent30.net/hubfs/8776530/Sygnia-%20Elephant%20Beetle_Jan2022.pdf)

|  [G0037](https://attack.mitre.org/groups/G0037)   |  [FIN6](https://attack.mitre.org/groups/G0037)   |

[FIN6](https://attack.mitre.org/groups/G0037) has collected and exfiltrated payment card data from compromised systems.[[88]](https://www.trendmicro.com/en_us/research/19/j/fin6-compromised-e-commerce-platform-via-magecart-to-inject-credit-card-skimmers-into-thousands-of-online-shops.html)[[89]](https://web.archive.org/web/20181231220607/https://riskiq.com/blog/labs/magecart-british-airways-breach/)[[90]](https://web.archive.org/web/20181209083100/https://www.riskiq.com/blog/labs/magecart-newegg/)

|  [G0046](https://attack.mitre.org/groups/G0046)   |  [FIN7](https://attack.mitre.org/groups/G0046)   |

[FIN7](https://attack.mitre.org/groups/G0046) has collected files and other sensitive information from a compromised network.[[91]](https://www.crowdstrike.com/blog/carbon-spider-embraces-big-game-hunting-part-1/)

|  [S0696](https://attack.mitre.org/software/S0696)   |  [Flagpro](https://attack.mitre.org/software/S0696)   |

[Flagpro](https://attack.mitre.org/software/S0696) can collect data from a compromised host, including Windows authentication information.[[92]](https://insight-jp.nttsecurity.com/post/102hf3q/flagpro-the-new-malware-used-by-blacktech)

|  [S0036](https://attack.mitre.org/software/S0036)   |  [FLASHFLOOD](https://attack.mitre.org/software/S0036)   |

[FLASHFLOOD](https://attack.mitre.org/software/S0036) searches for interesting files (either a default or customized set of file extensions) on the local system. [FLASHFLOOD](https://attack.mitre.org/software/S0036) will scan the My Recent Documents, Desktop, Temporary Internet Files, and TEMP directories. [FLASHFLOOD](https://attack.mitre.org/software/S0036) also collects information stored in the Windows Address Book.[[93]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2015/05/20081935/rpt-apt30.pdf)

|  [S0381](https://attack.mitre.org/software/S0381)   |  [FlawedAmmyy](https://attack.mitre.org/software/S0381)   |

[FlawedAmmyy](https://attack.mitre.org/software/S0381) has collected information and files from a compromised machine.[[94]](https://www.fsec.or.kr/user/bbs/fsec/163/344/bbsDataView/1382.do?page=1&column=&search=&searchSDate=&searchEDate=&bbsDataCategory=)

|  [S0661](https://attack.mitre.org/software/S0661)   |  [FoggyWeb](https://attack.mitre.org/software/S0661)   |

[FoggyWeb](https://attack.mitre.org/software/S0661) can retrieve configuration data from a compromised AD FS server.[[95]](https://www.microsoft.com/security/blog/2021/09/27/foggyweb-targeted-nobelium-malware-leads-to-persistent-backdoor/)

|  [S0193](https://attack.mitre.org/software/S0193)   |  [Forfiles](https://attack.mitre.org/software/S0193)   |

[Forfiles](https://attack.mitre.org/software/S0193) can be used to act on (ex: copy, move, etc.) files/directories in a system during (ex: copy files into a staging area before).[[10]](https://netzpolitik.org/2015/digital-attack-on-german-parliament-investigative-report-on-the-hack-of-the-left-party-infrastructure-in-bundestag/)

|  [G0117](https://attack.mitre.org/groups/G0117)   |  [Fox Kitten](https://attack.mitre.org/groups/G0117)   |

[Fox Kitten](https://attack.mitre.org/groups/G0117) has searched local system resources to access sensitive documents.[[96]](https://us-cert.cisa.gov/ncas/alerts/aa20-259a)

|  [S0503](https://attack.mitre.org/software/S0503)   |  [FrameworkPOS](https://attack.mitre.org/software/S0503)   |

[FrameworkPOS](https://attack.mitre.org/software/S0503) can collect elements related to credit card data from process memory.[[97]](https://labs.sentinelone.com/fin6-frameworkpos-point-of-sale-malware-analysis-internals-2/)

|  [C0001](https://attack.mitre.org/campaigns/C0001)   |  [Frankenstein](https://attack.mitre.org/campaigns/C0001)   |

During [Frankenstein](https://attack.mitre.org/campaigns/C0001), the threat actors used [Empire](https://attack.mitre.org/software/S0363) to gather various local system information.[[98]](https://blog.talosintelligence.com/2019/06/frankenstein-campaign.html)

|  [S1044](https://attack.mitre.org/software/S1044)   |  [FunnyDream](https://attack.mitre.org/software/S1044)   |

[FunnyDream](https://attack.mitre.org/software/S1044) can upload files from victims' machines.[[48]](https://www.bitdefender.com/files/News/CaseStudies/study/379/Bitdefender-Whitepaper-Chinese-APT.pdf)[[99]](https://securelist.com/apt-trends-report-q1-2020/96826/)

|  [G0093](https://attack.mitre.org/groups/G0093)   |  [GALLIUM](https://attack.mitre.org/groups/G0093)   |

[GALLIUM](https://attack.mitre.org/groups/G0093) collected data from the victim's local system, including password hashes from the SAM hive in the Registry.[[100]](https://www.cybereason.com/blog/operation-soft-cell-a-worldwide-campaign-against-telecommunications-providers)

|  [G0047](https://attack.mitre.org/groups/G0047)   |  [Gamaredon Group](https://attack.mitre.org/groups/G0047)   |

[Gamaredon Group](https://attack.mitre.org/groups/G0047) has collected files from infected systems and uploaded them to a C2 server.[[101]](https://www.welivesecurity.com/2020/06/11/gamaredon-group-grows-its-game/)[[102]](https://web-assets.esetstatic.com/wls/en/papers/white-papers/cyberespionage-gamaredon-way.pdf)

|  [S0666](https://attack.mitre.org/software/S0666)   |  [Gelsemium](https://attack.mitre.org/software/S0666)   |

[Gelsemium](https://attack.mitre.org/software/S0666) can collect data from a compromised host.[[55]](https://www.welivesecurity.com/wp-content/uploads/2021/06/eset_gelsemium.pdf)

|  [S9010](https://attack.mitre.org/software/S9010)   |  [GlassWorm](https://attack.mitre.org/software/S9010)   |

[GlassWorm](https://attack.mitre.org/software/S9010) has collected local data from a compromised host to include desktop cryptocurrency wallet data, and documents from within Desktop, Documents, and Downloads.[[103]](https://socket.dev/blog/glassworm-loader-hits-open-vsx-via-suspected-developer-account-compromise)

|  [S0477](https://attack.mitre.org/software/S0477)   |  [Goopy](https://attack.mitre.org/software/S0477)   |

[Goopy](https://attack.mitre.org/software/S0477) has the ability to exfiltrate documents from infected systems.[[104]](https://cdn2.hubspot.net/hubfs/3354902/Cybereason%20Labs%20Analysis%20Operation%20Cobalt%20Kitty.pdf)

|  [S0237](https://attack.mitre.org/software/S0237)   |  [GravityRAT](https://attack.mitre.org/software/S0237)   |

[GravityRAT](https://attack.mitre.org/software/S0237) steals files with the following extensions: .docx, .doc, .pptx, .ppt, .xlsx, .xls, .rtf, and .pdf.[[105]](https://blog.talosintelligence.com/2018/04/gravityrat-two-year-evolution-of-apt.html)

|  [S0690](https://attack.mitre.org/software/S0690)   |  [Green Lambert](https://attack.mitre.org/software/S0690)   |

[Green Lambert](https://attack.mitre.org/software/S0690) can collect data from a compromised host.[[106]](https://objective-see.com/blog/blog_0x68.html)

|  [S0632](https://attack.mitre.org/software/S0632)   |  [GrimAgent](https://attack.mitre.org/software/S0632)   |

[GrimAgent](https://attack.mitre.org/software/S0632) can collect data and files from a compromised host.[[107]](https://www.group-ib.com/blog/grimagent/)

|  [G0125](https://attack.mitre.org/groups/G0125)   |  [HAFNIUM](https://attack.mitre.org/groups/G0125)   |

[HAFNIUM](https://attack.mitre.org/groups/G0125) has collected data and files from a compromised machine.[[54]](https://www.rapid7.com/blog/post/2021/03/23/defending-against-the-zero-day-analyzing-attacker-behavior-post-exploitation-of-microsoft-exchange/)[[108]](https://www.microsoft.com/en-us/security/blog/2025/03/05/silk-typhoon-targeting-it-supply-chain/)

|  [S1229](https://attack.mitre.org/software/S1229)   |  [Havoc](https://attack.mitre.org/software/S1229)   |

[Havoc](https://attack.mitre.org/software/S1229) can download files from the victim's computer.[[109]](https://havocframework.com/docs/welcome)[[110]](https://www.immersivelabs.com/resources/blog/havoc-c2-framework-a-defensive-operators-guide)

|  [S9023](https://attack.mitre.org/software/S9023)   |  [HiddenFace](https://attack.mitre.org/software/S9023)   |

[HiddenFace](https://attack.mitre.org/software/S9023) can upload files from the victim machine to C2 nodes.[[111]](https://www.trendmicro.com/en_us/research/24/k/lodeinfo-campaign-of-earth-kasha.html)[[112]](https://blogs.jpcert.or.jp/en/2024/07/mirrorface-attack-against-japanese-organisations.html)

|  [S0009](https://attack.mitre.org/software/S0009)   |  [Hikit](https://attack.mitre.org/software/S0009)   |

[Hikit](https://attack.mitre.org/software/S0009) can upload files from compromised machines.[[22]](https://web.archive.org/web/20230115144216/http://www.novetta.com/wp-content/uploads/2014/11/Executive_Summary-Final_1.pdf)

|  [S0203](https://attack.mitre.org/software/S0203)   |  [Hydraq](https://attack.mitre.org/software/S0203)   |

[Hydraq](https://attack.mitre.org/software/S0203) creates a backdoor through which remote attackers can read data from files.[[113]](https://www.symantec.com/connect/blogs/trojanhydraq-incident)[[114]](https://www.symantec.com/security_response/writeup.jsp?docid=2010-011114-1830-99)

|  [S1022](https://attack.mitre.org/software/S1022)   |  [IceApple](https://attack.mitre.org/software/S1022)   |

[IceApple](https://attack.mitre.org/software/S1022) can collect files, passwords, and other data from a compromised host.[[115]](https://www.crowdstrike.com/wp-content/uploads/2022/05/crowdstrike-iceapple-a-novel-internet-information-services-post-exploitation-framework.pdf)

|  [G0100](https://attack.mitre.org/groups/G0100)   |  [Inception](https://attack.mitre.org/groups/G0100)   |

[Inception](https://attack.mitre.org/groups/G0100) used a file hunting plugin to collect .txt, .pdf, .xls or .doc files from the infected host.[[116]](https://securelist.com/recent-cloud-atlas-activity/92016/)

|  [S1245](https://attack.mitre.org/software/S1245)   |  [InvisibleFerret](https://attack.mitre.org/software/S1245)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has collected data utilizing a script that contained a list of excluded files and directory names and naming patterns of interest such as environment and configuration files, documents, spreadsheets, and other files that contained the words secret, wallet, private, and password.[[31]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)

|  [S0260](https://attack.mitre.org/software/S0260)   |  [InvisiMole](https://attack.mitre.org/software/S0260)   |

[InvisiMole](https://attack.mitre.org/software/S0260) can collect data from the system, and can monitor changes in specified directories.[[117]](https://www.welivesecurity.com/2018/06/07/invisimole-equipped-spyware-undercover/)

|  [S1132](https://attack.mitre.org/software/S1132)   |  [IPsec Helper](https://attack.mitre.org/software/S1132)   |

[IPsec Helper](https://attack.mitre.org/software/S1132) can identify specific files and folders for follow-on exfiltration.[[118]](https://assets.sentinelone.com/sentinellabs/evol-agrius)

|  [S0015](https://attack.mitre.org/software/S0015)   |  [Ixeshe](https://attack.mitre.org/software/S0015)   |

[Ixeshe](https://attack.mitre.org/software/S0015) can collect data from a local system.[[119]](https://www.trendmicro.de/cloud-content/us/pdfs/security-intelligence/white-papers/wp_ixeshe.pdf)

|  [S0265](https://attack.mitre.org/software/S0265)   |  [Kazuar](https://attack.mitre.org/software/S0265)   |

[Kazuar](https://attack.mitre.org/software/S0265) uploads files from a specified directory to the C2 server.[[120]](https://researchcenter.paloaltonetworks.com/2017/05/unit42-kazuar-multiplatform-espionage-backdoor-api-access/)

|  [G0004](https://attack.mitre.org/groups/G0004)   |  [Ke3chang](https://attack.mitre.org/groups/G0004)   |

[Ke3chang](https://attack.mitre.org/groups/G0004) gathered information and files from local directories for exfiltration.[[121]](https://www.mandiant.com/resources/operation-ke3chang-targeted-attacks-against-ministries-of-foreign-affairs)[[122]](https://www.microsoft.com/security/blog/2021/12/06/nickel-targeting-government-organizations-across-latin-america-and-europe)

|  [S1020](https://attack.mitre.org/software/S1020)   |  [Kevin](https://attack.mitre.org/software/S1020)   |

[Kevin](https://attack.mitre.org/software/S1020) can upload logs and other data from a compromised host.[[123]](https://vblocalhost.com/uploads/VB2021-Kayal-etal.pdf)

|  [S0526](https://attack.mitre.org/software/S0526)   |  [KGH_SPY](https://attack.mitre.org/software/S0526)   |

[KGH_SPY](https://attack.mitre.org/software/S0526) can send a file containing victim system information to C2.[[124]](https://www.cybereason.com/blog/back-to-the-future-inside-the-kimsuky-kgh-spyware-suite)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has collected Office, PDF, and HWP documents from its victims.[[125]](https://securelist.com/the-kimsuky-operation-a-north-korean-apt/57915/)[[126]](https://blog.talosintelligence.com/2021/11/kimsuky-abuses-blogs-delivers-malware.html) [Kimsuky](https://attack.mitre.org/groups/G0094) has also harvested victim files through the use of the RecentFiles() function that collects paths of recently accessed files by parsing .lnk shortcuts from %APPDATA%\Microsoft\Windows\Recent.[[127]](https://www.aryaka.com/docs/reports/aryaka-kimsuky-apt-operational-blueprint.pdf)

|  [S0250](https://attack.mitre.org/software/S0250)   |  [Koadic](https://attack.mitre.org/software/S0250)   |

[Koadic](https://attack.mitre.org/software/S0250) can download files off the target system to send back to the server.[[128]](https://github.com/offsecginger/koadic)[[129]](https://web.archive.org/web/20211003035156/https://www.malwarebytes.com/resources/files/2021/02/lazyscripter.pdf)

|  [S0356](https://attack.mitre.org/software/S0356)   |  [KONNI](https://attack.mitre.org/software/S0356)   |

[KONNI](https://attack.mitre.org/software/S0356) has stored collected information and discovered processes in a tmp file.[[130]](https://blog.malwarebytes.com/threat-intelligence/2021/08/new-variant-of-konni-malware-used-in-campaign-targetting-russia/)

|  [S1075](https://attack.mitre.org/software/S1075)   |  [KOPILUWAK](https://attack.mitre.org/software/S1075)   |

[KOPILUWAK](https://attack.mitre.org/software/S1075) can gather information from compromised hosts.[[44]](https://www.mandiant.com/resources/blog/turla-galaxy-opportunity)

|  [S9035](https://attack.mitre.org/software/S9035)   |  [LAMEHUG](https://attack.mitre.org/software/S9035)   |

[LAMEHUG](https://attack.mitre.org/software/S9035) has the ability to collect system information and files of interest from compromised systems.[[131]](https://www.splunk.com/en_us/blog/security/lamehug-ai-driven-malware-llm-cyber-intrusion-analysis.html)[[132]](https://cloud.google.com/blog/topics/threat-intelligence/threat-actor-usage-of-ai-tools)

|  [G1004](https://attack.mitre.org/groups/G1004)   |  [LAPSUS$](https://attack.mitre.org/groups/G1004)   |

[LAPSUS$](https://attack.mitre.org/groups/G1004) uploaded sensitive files, information, and credentials from a targeted organization for extortion or public release.[[133]](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)

|  [S1160](https://attack.mitre.org/software/S1160)   |  [Latrodectus](https://attack.mitre.org/software/S1160)   |

[Latrodectus](https://attack.mitre.org/software/S1160) can collect data from a compromised host using a stealer module.[[134]](https://www.bitsight.com/blog/latrodectus-are-you-coming-back)

|  [G0032](https://attack.mitre.org/groups/G0032)   |  [Lazarus Group](https://attack.mitre.org/groups/G0032)   |

[Lazarus Group](https://attack.mitre.org/groups/G0032) has collected data and files from compromised networks.[[135]](https://web.archive.org/web/20160226161828/https://www.operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Report.pdf)[[136]](https://web.archive.org/web/20190508165631/https://operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Loaders-Installers-and-Uninstallers-Report.pdf)[[137]](https://web.archive.org/web/20220608001455/https://www.operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-RAT-and-Staging-Report.pdf)[[138]](https://securelist.com/lazarus-threatneedle/100803/)

|  [S0395](https://attack.mitre.org/software/S0395)   |  [LightNeuron](https://attack.mitre.org/software/S0395)   |

[LightNeuron](https://attack.mitre.org/software/S0395) can collect files from a local system.[[139]](https://www.welivesecurity.com/wp-content/uploads/2019/05/ESET-LightNeuron.pdf)

|  [S0211](https://attack.mitre.org/software/S0211)   |  [Linfo](https://attack.mitre.org/software/S0211)   |

[Linfo](https://attack.mitre.org/software/S0211) creates a backdoor through which remote attackers can obtain data from local systems.[[140]](https://www.symantec.com/security_response/writeup.jsp?docid=2012-051605-2535-99)

|  [S9020](https://attack.mitre.org/software/S9020)   |  [LODEINFO](https://attack.mitre.org/software/S9020)   |

[LODEINFO](https://attack.mitre.org/software/S9020) can upload files from infected hosts to the C2.[[141]](https://securelist.com/apt10-tracking-down-lodeinfo-2022-part-ii/107745/)[[142]](https://www.welivesecurity.com/2022/12/14/unmasking-mirrorface-operation-liberalface-targeting-japanese-political-entities/)[[143]](https://blog-en.itochuci.co.jp/entry/2024/01/24/134100)

|  [S1101](https://attack.mitre.org/software/S1101)   |  [LoFiSe](https://attack.mitre.org/software/S1101)   |

[LoFiSe](https://attack.mitre.org/software/S1101) can collect files of interest from targeted systems.[[144]](https://securelist.com/toddycat-keep-calm-and-check-logs/110696/)

|  [G1014](https://attack.mitre.org/groups/G1014)   |  [LuminousMoth](https://attack.mitre.org/groups/G1014)   |

[LuminousMoth](https://attack.mitre.org/groups/G1014) has collected files and data from compromised machines.[[145]](https://securelist.com/apt-luminousmoth/103332/)[[146]](https://www.bitdefender.com/blog/labs/luminousmoth-plugx-file-exfiltration-and-persistence-revisited)

|  [S0409](https://attack.mitre.org/software/S0409)   |  [Machete](https://attack.mitre.org/software/S0409)   |

[Machete](https://attack.mitre.org/software/S0409) searches the File system for files of interest.[[147]](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)

|  [S1016](https://attack.mitre.org/software/S1016)   |  [MacMa](https://attack.mitre.org/software/S1016)   |

[MacMa](https://attack.mitre.org/software/S1016) can collect then exfiltrate files from the compromised system.[[148]](https://www.welivesecurity.com/2022/01/25/watering-hole-deploys-new-macos-malware-dazzlespy-asia/)

|  [S1060](https://attack.mitre.org/software/S1060)   |  [Mafalda](https://attack.mitre.org/software/S1060)   |

[Mafalda](https://attack.mitre.org/software/S1060) can collect files and information from a compromised host.[[149]](https://assets.sentinelone.com/sentinellabs22/metador#page=1)

|  [G0059](https://attack.mitre.org/groups/G0059)   |  [Magic Hound](https://attack.mitre.org/groups/G0059)   |

[Magic Hound](https://attack.mitre.org/groups/G0059) has used a web shell to exfiltrate a ZIP file containing a dump of LSASS memory on a compromised machine.[[150]](https://thedfirreport.com/2022/03/21/apt35-automates-initial-access-using-proxyshell)[[151]](https://thedfirreport.com/2021/11/15/exchange-exploit-leads-to-domain-wide-ransomware/)

|  [S0652](https://attack.mitre.org/software/S0652)   |  [MarkiRAT](https://attack.mitre.org/software/S0652)   |

[MarkiRAT](https://attack.mitre.org/software/S0652) can upload data from the victim's machine to the C2 server.[[152]](https://securelist.com/ferocious-kitten-6-years-of-covert-surveillance-in-iran/102806/)

|  [S0500](https://attack.mitre.org/software/S0500)   |  [MCMD](https://attack.mitre.org/software/S0500)   |

[MCMD](https://attack.mitre.org/software/S0500) has the ability to upload files from an infected device.[[153]](https://www.secureworks.com/research/mcmd-malware-analysis)

|  [G0045](https://attack.mitre.org/groups/G0045)   |  [menuPass](https://attack.mitre.org/groups/G0045)   |

[menuPass](https://attack.mitre.org/groups/G0045) has collected various files from the compromised computers.[[154]](https://www.justice.gov/opa/pr/two-chinese-hackers-associated-ministry-state-security-charged-global-computer-intrusion)[[155]](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/cicada-apt10-japan-espionage)

|  [S1059](https://attack.mitre.org/software/S1059)   |  [metaMain](https://attack.mitre.org/software/S1059)   |

[metaMain](https://attack.mitre.org/software/S1059) can collect files and system information from a compromised host.[[149]](https://assets.sentinelone.com/sentinellabs22/metador#page=1)[[156]](https://docs.google.com/document/d/1e9ZTW9b71YwFWS_18ZwDAxa-cYbV8q1wUefmKZLYVsA/edit#heading=h.lmnbtht1ikzm)

|  [S1146](https://attack.mitre.org/software/S1146)   |  [MgBot](https://attack.mitre.org/software/S1146)   |

[MgBot](https://attack.mitre.org/software/S1146) includes modules for collecting files from local systems based on a given set of properties and filenames.[[157]](https://www.welivesecurity.com/2023/04/26/evasive-panda-apt-group-malware-updates-popular-chinese-software/)

|  [S1015](https://attack.mitre.org/software/S1015)   |  [Milan](https://attack.mitre.org/software/S1015)   |

[Milan](https://attack.mitre.org/software/S1015) can upload files from a compromised host.[[158]](https://www.clearskysec.com/siamesekitten/)

|  [S9059](https://attack.mitre.org/software/S9059)   |  [MiniBike](https://attack.mitre.org/software/S9059)   |

[MiniBike](https://attack.mitre.org/software/S9059) has collected files from the local victim machines.[[159]](https://cloud.google.com/blog/topics/threat-intelligence/suspected-iranian-unc1549-targets-israel-middle-east)

|  [S9052](https://attack.mitre.org/software/S9052)   |  [MiniFast](https://attack.mitre.org/software/S9052)   |

[MiniFast](https://attack.mitre.org/software/S9052) has collected data from local files and directories on the victim device.[[160]](https://research.checkpoint.com/2026/fast-and-furious-nimbus-manticore-operations-during-the-iranian-conflict/)

|  [G1054](https://attack.mitre.org/groups/G1054)   |  [MirrorFace](https://attack.mitre.org/groups/G1054)   |

[MirrorFace](https://attack.mitre.org/groups/G1054) gathered data and files of interest from victim's systems.[[111]](https://www.trendmicro.com/en_us/research/24/k/lodeinfo-campaign-of-earth-kasha.html)

|  [S0084](https://attack.mitre.org/software/S0084)   |  [Mis-Type](https://attack.mitre.org/software/S0084)   |

[Mis-Type](https://attack.mitre.org/software/S0084) has collected files and data from a compromised host.[[161]](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)

|  [S0083](https://attack.mitre.org/software/S0083)   |  [Misdat](https://attack.mitre.org/software/S0083)   |

[Misdat](https://attack.mitre.org/software/S0083) has collected files and data from a compromised host.[[161]](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)

|  [S0079](https://attack.mitre.org/software/S0079)   |  [MobileOrder](https://attack.mitre.org/software/S0079)   |

[MobileOrder](https://attack.mitre.org/software/S0079) exfiltrates data collected from the victim mobile device.[[162]](http://researchcenter.paloaltonetworks.com/2016/01/scarlet-mimic-years-long-espionage-targets-minority-activists/)

|  [S1026](https://attack.mitre.org/software/S1026)   |  [Mongall](https://attack.mitre.org/software/S1026)   |

[Mongall](https://attack.mitre.org/software/S1026) has the ability to upload files from victim's machines.[[163]](https://www.sentinelone.com/labs/aoqin-dragon-newly-discovered-chinese-linked-apt-has-been-quietly-spying-on-organizations-for-10-years/)

|  [S0630](https://attack.mitre.org/software/S0630)   |  [Nebulae](https://attack.mitre.org/software/S0630)   |

[Nebulae](https://attack.mitre.org/software/S0630) has the capability to upload collected files to C2.[[164]](https://www.bitdefender.com/files/News/CaseStudies/study/396/Bitdefender-PR-Whitepaper-NAIKON-creat5397-en-EN.pdf)

|  [S0691](https://attack.mitre.org/software/S0691)   |  [Neoichor](https://attack.mitre.org/software/S0691)   |

[Neoichor](https://attack.mitre.org/software/S0691) can upload files from a victim's machine.[[122]](https://www.microsoft.com/security/blog/2021/12/06/nickel-targeting-government-organizations-across-latin-america-and-europe)

|  [C0002](https://attack.mitre.org/campaigns/C0002)   |  [Night Dragon](https://attack.mitre.org/campaigns/C0002)   |

During [Night Dragon](https://attack.mitre.org/campaigns/C0002), the threat actors collected files and other data from compromised systems.[[165]](https://scadahacker.com/library/Documents/Cyber_Events/McAfee%20-%20Night%20Dragon%20-%20Global%20Energy%20Cyberattacks.pdf)

|  [S1090](https://attack.mitre.org/software/S1090)   |  [NightClub](https://attack.mitre.org/software/S1090)   |

[NightClub](https://attack.mitre.org/software/S1090) can use a file monitor to steal specific files from targeted systems.[[166]](https://www.welivesecurity.com/en/eset-research/moustachedbouncer-espionage-against-foreign-diplomats-in-belarus/)

|  [S0385](https://attack.mitre.org/software/S0385)   |  [njRAT](https://attack.mitre.org/software/S0385)   |

[njRAT](https://attack.mitre.org/software/S0385) can collect data from a local system.[[167]](https://www.threatminer.org/_reports/2013/fta-1009---njrat-uncovered-1.pdf)

|  [S1131](https://attack.mitre.org/software/S1131)   |  [NPPSPY](https://attack.mitre.org/software/S1131)   |

[NPPSPY](https://attack.mitre.org/software/S1131) records data entered from the local system logon at Winlogon to capture credentials in cleartext.[[168]](https://www.huntress.com/blog/cleartext-shenanigans-gifting-user-passwords-to-adversaries-with-nppspy)

|  [S0340](https://attack.mitre.org/software/S0340)   |  [Octopus](https://attack.mitre.org/software/S0340)   |

[Octopus](https://attack.mitre.org/software/S0340) can exfiltrate files from the system using a documents collector tool.[[169]](https://www.virusbulletin.com/uploads/pdf/conference_slides/2018/Cherepanov-VB2018-Octopus.pdf)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) has used PowerShell to upload files from compromised systems.[[170]](https://www.trendmicro.com/en_us/research/24/j/earth-simnavaz-cyberattacks.html)

|  [C0012](https://attack.mitre.org/campaigns/C0012)   |  [Operation CuckooBees](https://attack.mitre.org/campaigns/C0012)   |

During [Operation CuckooBees](https://attack.mitre.org/campaigns/C0012), the threat actors collected data, files, and other information from compromised networks.[[171]](https://www.cybereason.com/blog/operation-cuckoobees-deep-dive-into-stealthy-winnti-techniques)

|  [C0022](https://attack.mitre.org/campaigns/C0022)   |  [Operation Dream Job](https://attack.mitre.org/campaigns/C0022)   |

During [Operation Dream Job](https://attack.mitre.org/campaigns/C0022), [Lazarus Group](https://attack.mitre.org/groups/G0032) used malicious Trojans and DLL files to exfiltrate data from an infected host.[[77]](https://www.clearskysec.com/wp-content/uploads/2020/08/Dream-Job-Campaign.pdf)[[172]](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/operation-north-star-a-job-offer-thats-too-good-to-be-true/?hilite=%27Operation%27%2C%27North%27%2C%27Star%27)

|  [C0006](https://attack.mitre.org/campaigns/C0006)   |  [Operation Honeybee](https://attack.mitre.org/campaigns/C0006)   |

During [Operation Honeybee](https://attack.mitre.org/campaigns/C0006), the threat actors collected data from compromised hosts.[[173]](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/mcafee-uncovers-operation-honeybee-malicious-document-campaign-targeting-humanitarian-aid-groups/)

|  [C0048](https://attack.mitre.org/campaigns/C0048)   |  [Operation MidnightEclipse](https://attack.mitre.org/campaigns/C0048)   |

During [Operation MidnightEclipse](https://attack.mitre.org/campaigns/C0048), threat actors stole saved cookies and login data from targeted systems.[[174]](https://www.volexity.com/blog/2024/04/12/zero-day-exploitation-of-unauthenticated-remote-code-execution-vulnerability-in-globalprotect-cve-2024-3400/)

|  [C0014](https://attack.mitre.org/campaigns/C0014)   |  [Operation Wocao](https://attack.mitre.org/campaigns/C0014)   |

During [Operation Wocao](https://attack.mitre.org/campaigns/C0014), threat actors exfiltrated files and directories of interest from the targeted system.[[175]](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)

|  [S0352](https://attack.mitre.org/software/S0352)   |  [OSX_OCEANLOTUS.D](https://attack.mitre.org/software/S0352)   |

[OSX_OCEANLOTUS.D](https://attack.mitre.org/software/S0352) has the ability to upload files from a compromised host.[[176]](https://www.trendmicro.com/en_us/research/20/k/new-macos-backdoor-connected-to-oceanlotus-surfaces.html)

|  [S0594](https://attack.mitre.org/software/S0594)   |  [Out1](https://attack.mitre.org/software/S0594)   |

[Out1](https://attack.mitre.org/software/S0594) can copy files and Registry data from compromised hosts.[[177]](https://www.trendmicro.com/en_us/research/21/c/earth-vetala---muddywater-continues-to-target-organizations-in-t.html)

|  [S1017](https://attack.mitre.org/software/S1017)   |  [OutSteel](https://attack.mitre.org/software/S1017)   |

[OutSteel](https://attack.mitre.org/software/S1017) can collect information from a compromised host.[[178]](https://unit42.paloaltonetworks.com/ukraine-targeted-outsteel-saintbot/)

|  [S0598](https://attack.mitre.org/software/S0598)   |  [P.A.S. Webshell](https://attack.mitre.org/software/S0598)   |

[P.A.S. Webshell](https://attack.mitre.org/software/S0598) has the ability to copy files on a compromised host.[[179]](https://www.cert.ssi.gouv.fr/uploads/CERTFR-2021-CTI-005.pdf)

|  [S0208](https://attack.mitre.org/software/S0208)   |  [Pasam](https://attack.mitre.org/software/S0208)   |

[Pasam](https://attack.mitre.org/software/S0208) creates a backdoor through which remote attackers can retrieve files.[[180]](https://www.symantec.com/security_response/writeup.jsp?docid=2012-050412-4128-99)

|  [G0040](https://attack.mitre.org/groups/G0040)   |  [Patchwork](https://attack.mitre.org/groups/G0040)   |

[Patchwork](https://attack.mitre.org/groups/G0040) collected and exfiltrated files from the infected system.[[181]](https://web.archive.org/web/20180825085952/https:/s3-us-west-2.amazonaws.com/cymmetria-blog/public/Unveiling_Patchwork.pdf)

|  [S1102](https://attack.mitre.org/software/S1102)   |  [Pcexter](https://attack.mitre.org/software/S1102)   |

[Pcexter](https://attack.mitre.org/software/S1102) can upload files from targeted systems.[[144]](https://securelist.com/toddycat-keep-calm-and-check-logs/110696/)

|  [S1050](https://attack.mitre.org/software/S1050)   |  [PcShare](https://attack.mitre.org/software/S1050)   |

[PcShare](https://attack.mitre.org/software/S1050) can collect files and information from a compromised host.[[48]](https://www.bitdefender.com/files/News/CaseStudies/study/379/Bitdefender-Whitepaper-Chinese-APT.pdf)

|  [S0517](https://attack.mitre.org/software/S0517)   |  [Pillowmint](https://attack.mitre.org/software/S0517)   |

[Pillowmint](https://attack.mitre.org/software/S0517) has collected credit card data using native API functions.[[182]](https://www.trustwave.com/en-us/resources/blogs/spiderlabs-blog/pillowmint-fin7s-monkey-thief/)

|  [S0048](https://attack.mitre.org/software/S0048)   |  [PinchDuke](https://attack.mitre.org/software/S0048)   |

[PinchDuke](https://attack.mitre.org/software/S0048) collects user files from the compromised host based on predefined file extensions.[[183]](https://www.f-secure.com/documents/996508/1030745/dukes_whitepaper.pdf)

|  [S1031](https://attack.mitre.org/software/S1031)   |  [PingPull](https://attack.mitre.org/software/S1031)   |

[PingPull](https://attack.mitre.org/software/S1031) can collect data from a compromised host.[[184]](https://unit42.paloaltonetworks.com/pingpull-gallium/)

|  [S0012](https://attack.mitre.org/software/S0012)   |  [PoisonIvy](https://attack.mitre.org/software/S0012)   |

[PoisonIvy](https://attack.mitre.org/software/S0012) creates a backdoor through which remote attackers can steal system information.[[185]](https://www.symantec.com/security_response/writeup.jsp?docid=2005-081910-3934-99)

|  [S1012](https://attack.mitre.org/software/S1012)   |  [PowerLess](https://attack.mitre.org/software/S1012)   |

[PowerLess](https://attack.mitre.org/software/S1012) has the ability to exfiltrate data, including Chrome and Edge browser database files, from compromised machines.[[186]](https://www.cybereason.com/blog/research/powerless-trojan-iranian-apt-phosphorus-adds-new-powershell-backdoor-for-espionage)

|  [S0194](https://attack.mitre.org/software/S0194)   |  [PowerSploit](https://attack.mitre.org/software/S0194)   |

[PowerSploit](https://attack.mitre.org/software/S0194) contains a collection of Exfiltration modules that can access data from local files, volumes, and processes.[[187]](https://github.com/PowerShellMafia/PowerSploit)[[188]](http://powersploit.readthedocs.io)

|  [S0223](https://attack.mitre.org/software/S0223)   |  [POWERSTATS](https://attack.mitre.org/software/S0223)   |

[POWERSTATS](https://attack.mitre.org/software/S0223) can upload files from compromised hosts.[[189]](https://www.fireeye.com/blog/threat-research/2018/03/iranian-threat-group-updates-ttps-in-spear-phishing-campaign.html)

|  [S0238](https://attack.mitre.org/software/S0238)   |  [Proxysvc](https://attack.mitre.org/software/S0238)   |

[Proxysvc](https://attack.mitre.org/software/S0238) searches the local system and gathers data.[[190]](https://securingtomorrow.mcafee.com/mcafee-labs/analyzing-operation-ghostsecret-attack-seeks-to-steal-data-worldwide/)

|  [S0197](https://attack.mitre.org/software/S0197)   |  [PUNCHTRACK](https://attack.mitre.org/software/S0197)   |

[PUNCHTRACK](https://attack.mitre.org/software/S0197) scrapes memory for properly formatted payment card data.[[191]](https://www.fireeye.com/blog/threat-research/2016/05/windows-zero-day-payment-cards.html)[[192]](https://www2.fireeye.com/WBNR-Know-Your-Enemy-UNC622-Spear-Phishing.html)

|  [S0650](https://attack.mitre.org/software/S0650)   |  [QakBot](https://attack.mitre.org/software/S0650)   |

[QakBot](https://attack.mitre.org/software/S0650) can use a variety of commands, including esentutl.exe to steal sensitive data from Internet Explorer and Microsoft Edge, to acquire information that is subsequently exfiltrated.[[193]](https://redcanary.com/threat-detection-report/threats/qbot/)[[194]](https://securelist.com/qakbot-technical-analysis/103931/)

|  [S0262](https://attack.mitre.org/software/S0262)   |  [QuasarRAT](https://attack.mitre.org/software/S0262)   |

[QuasarRAT](https://attack.mitre.org/software/S0262) can retrieve files from compromised client machines.[[195]](https://www.cisa.gov/uscert/ncas/analysis-reports/AR18-352A)

|  [S0686](https://attack.mitre.org/software/S0686)   |  [QuietSieve](https://attack.mitre.org/software/S0686)   |

[QuietSieve](https://attack.mitre.org/software/S0686) can collect files from a compromised host.[[196]](https://www.microsoft.com/security/blog/2022/02/04/actinium-targets-ukrainian-organizations/)

|  [S1148](https://attack.mitre.org/software/S1148)   |  [Raccoon Stealer](https://attack.mitre.org/software/S1148)   |

[Raccoon Stealer](https://attack.mitre.org/software/S1148) collects data from victim machines based on configuration information received from command and control nodes.[[197]](https://medium.com/s2wblog/raccoon-stealer-is-back-with-a-new-version-5f436e04b20d)[[198]](https://blog.sekoia.io/raccoon-stealer-v2-part-2-in-depth-analysis/)

|  [S0629](https://attack.mitre.org/software/S0629)   |  [RainyDay](https://attack.mitre.org/software/S0629)   |

[RainyDay](https://attack.mitre.org/software/S0629) can use a file exfiltration tool to collect recently changed files on a compromised host.[[164]](https://www.bitdefender.com/files/News/CaseStudies/study/396/Bitdefender-PR-Whitepaper-NAIKON-creat5397-en-EN.pdf)

|  [S0458](https://attack.mitre.org/software/S0458)   |  [Ramsay](https://attack.mitre.org/software/S0458)   |

[Ramsay](https://attack.mitre.org/software/S0458) can collect Microsoft Word documents from the target's file system, as well as .txt, .doc, and .xls files from the Internet Explorer cache.[[199]](https://www.welivesecurity.com/2020/05/13/ramsay-cyberespionage-toolkit-airgapped-networks/)[[200]](https://www.programmersought.com/article/62493896999/)

|  [S1113](https://attack.mitre.org/software/S1113)   |  [RAPIDPULSE](https://attack.mitre.org/software/S1113)   |

[RAPIDPULSE](https://attack.mitre.org/software/S1113) retrieves files from the victim system via encrypted commands sent to the web shell.[[201]](https://www.mandiant.com/resources/blog/updates-on-chinese-apt-compromising-pulse-secure-vpn-devices)

|  [S0169](https://attack.mitre.org/software/S0169)   |  [RawPOS](https://attack.mitre.org/software/S0169)   |

[RawPOS](https://attack.mitre.org/software/S0169) dumps memory from specific processes on a victim system, parses the dumped files, and scrapes them for credit card data.[[202]](https://www.kroll.com/en/insights/publications/malware-analysis-report-rawpos-malware)[[203]](http://sjc1-te-ftp.trendmicro.com/images/tex/pdf/RawPOS%20Technical%20Brief.pdf)[[204]](https://www.youtube.com/watch?v=fevGZs0EQu8)

|  [S0662](https://attack.mitre.org/software/S0662)   |  [RCSession](https://attack.mitre.org/software/S0662)   |

[RCSession](https://attack.mitre.org/software/S0662) can collect data from a compromised host.[[205]](https://web.archive.org/web/20210104144857/https://shared-public-reports.s3-eu-west-1.amazonaws.com/APT27+turns+to+ransomware.pdf)[[56]](https://documents.trendmicro.com/assets/white_papers/wp-uncovering-DRBcontrol.pdf)

|  [G1039](https://attack.mitre.org/groups/G1039)   |  [RedCurl](https://attack.mitre.org/groups/G1039)   |

[RedCurl](https://attack.mitre.org/groups/G1039) has collected data from the local disk of compromised hosts.[[206]](https://www.group-ib.com/resources/research-hub/red-curl/)[[207]](https://www.group-ib.com/resources/research-hub/red-curl-2/)

|  [S1240](https://attack.mitre.org/software/S1240)   |  [RedLine Stealer](https://attack.mitre.org/software/S1240)   |

[RedLine Stealer](https://attack.mitre.org/software/S1240) has collected data stored locally including chat logs and files associated with chat services such as Steam, Discord, and Telegram.[[208]](https://www.welivesecurity.com/en/eset-research/life-crooked-redline-analyzing-infamous-infostealers-backend/)

|  [S0448](https://attack.mitre.org/software/S0448)   |  [Rising Sun](https://attack.mitre.org/software/S0448)   |

[Rising Sun](https://attack.mitre.org/software/S0448) has collected data and files from a compromised host.[[209]](https://www.mcafee.com/enterprise/en-us/assets/reports/rp-operation-sharpshooter.pdf)

|  [S0240](https://attack.mitre.org/software/S0240)   |  [ROKRAT](https://attack.mitre.org/software/S0240)   |

[ROKRAT](https://attack.mitre.org/software/S0240) can collect host data and specific file types.[[210]](https://research.nccgroup.com/2018/11/08/rokrat-analysis/)[[211]](https://www.volexity.com/blog/2021/08/24/north-korean-bluelight-special-inkysquid-deploys-rokrat/)[[212]](https://blog.malwarebytes.com/threat-analysis/2021/01/retrohunting-apt37-north-korean-apt-used-vba-self-decode-technique-to-inject-rokrat/)

|  [S0090](https://attack.mitre.org/software/S0090)   |  [Rover](https://attack.mitre.org/software/S0090)   |

[Rover](https://attack.mitre.org/software/S0090) searches for files on local drives based on a predefined list of file extensions.[[213]](http://researchcenter.paloaltonetworks.com/2016/02/new-malware-rover-targets-indian-ambassador-to-afghanistan/)

|  [S1018](https://attack.mitre.org/software/S1018)   |  [Saint Bot](https://attack.mitre.org/software/S1018)   |

[Saint Bot](https://attack.mitre.org/software/S1018) can collect files and information from a compromised host.[[214]](https://blog.malwarebytes.com/threat-intelligence/2021/04/a-deep-dive-into-saint-bot-downloader/)

|  [G1045](https://attack.mitre.org/groups/G1045)   |  [Salt Typhoon](https://attack.mitre.org/groups/G1045)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has gathered data from victim devices.[[215]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  [S1099](https://attack.mitre.org/software/S1099)   |  [Samurai](https://attack.mitre.org/software/S1099)   |

[Samurai](https://attack.mitre.org/software/S1099) can leverage an exfiltration module to download arbitrary files from compromised machines.[[216]](https://securelist.com/toddycat/106799/)

|  [G0034](https://attack.mitre.org/groups/G0034)   |  [Sandworm Team](https://attack.mitre.org/groups/G0034)   |

[Sandworm Team](https://attack.mitre.org/groups/G0034) has exfiltrated internal documents, files, and other data from compromised hosts.[[217]](https://www.justice.gov/opa/press-release/file/1328521/download)

|  [S1085](https://attack.mitre.org/software/S1085)   |  [Sardonic](https://attack.mitre.org/software/S1085)   |

[Sardonic](https://attack.mitre.org/software/S1085) has the ability to collect data from a compromised machine to deliver to the attacker.[[218]](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/syssphinx-fin8-backdoor)

|  [S0461](https://attack.mitre.org/software/S0461)   |  [SDBbot](https://attack.mitre.org/software/S0461)   |

[SDBbot](https://attack.mitre.org/software/S0461) has the ability to access the file system on a compromised host.[[219]](https://www.proofpoint.com/us/threat-insight/post/ta505-distributes-new-sdbbot-remote-access-trojan-get2-downloader)

|  [C0058](https://attack.mitre.org/campaigns/C0058)   |  [SharePoint ToolShell Exploitation](https://attack.mitre.org/campaigns/C0058)   |

During [SharePoint ToolShell Exploitation](https://attack.mitre.org/campaigns/C0058), threat actors extracted information from the compromised systems.[[220]](https://www.microsoft.com/en-us/security/blog/2025/07/22/disrupting-active-exploitation-of-on-premises-sharepoint-vulnerabilities/#storm-2603)[[221]](https://www.welivesecurity.com/en/eset-research/toolshell-an-all-you-can-eat-buffet-for-threat-actors/)[[222]](https://www.sentinelone.com/blog/sharepoint-toolshell-zero-day-exploited-in-the-wild-targets-enterprise-servers/)[[223]](https://unit42.paloaltonetworks.com/microsoft-sharepoint-cve-2025-49704-cve-2025-49706-cve-2025-53770/)

|  [S1019](https://attack.mitre.org/software/S1019)   |  [Shark](https://attack.mitre.org/software/S1019)   |

[Shark](https://attack.mitre.org/software/S1019) can upload files to its C2.[[158]](https://www.clearskysec.com/siamesekitten/)[[224]](https://www.accenture.com/us-en/blogs/cyber-defense/iran-based-lyceum-campaigns)

|  [S1089](https://attack.mitre.org/software/S1089)   |  [SharpDisco](https://attack.mitre.org/software/S1089)   |

[SharpDisco](https://attack.mitre.org/software/S1089) has dropped a recent-files stealer plugin to C:\Users\Public\WinSrcNT\It11.exe.[[166]](https://www.welivesecurity.com/en/eset-research/moustachedbouncer-espionage-against-foreign-diplomats-in-belarus/)

|  [S0444](https://attack.mitre.org/software/S0444)   |  [ShimRat](https://attack.mitre.org/software/S0444)   |

[ShimRat](https://attack.mitre.org/software/S0444) has the capability to upload collected files to a C2.[[225]](https://foxitsecurity.files.wordpress.com/2016/06/fox-it_mofang_threatreport_tlp-white.pdf)

|  [S0610](https://attack.mitre.org/software/S0610)   |  [SideTwist](https://attack.mitre.org/software/S0610)   |

[SideTwist](https://attack.mitre.org/software/S0610) has the ability to upload files from a compromised host.[[226]](https://research.checkpoint.com/2021/irans-apt34-returns-with-an-updated-arsenal/)

|  [S9058](https://attack.mitre.org/software/S9058)   |  [SIGHTGRAB](https://attack.mitre.org/software/S9058)   |

[SIGHTGRAB](https://attack.mitre.org/software/S9058) has captured and staged stolen data in folders named using a captured timestamp in the format of YYYY-MM-DD-HH-MM.[[74]](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)

|  [S1110](https://attack.mitre.org/software/S1110)   |  [SLIGHTPULSE](https://attack.mitre.org/software/S1110)   |

[SLIGHTPULSE](https://attack.mitre.org/software/S1110) can read files specified on the local system.[[227]](https://www.mandiant.com/resources/blog/suspected-apt-actors-leverage-bypass-techniques-pulse-secure-zero-day)

|  [S0533](https://attack.mitre.org/software/S0533)   |  [SLOTHFULMEDIA](https://attack.mitre.org/software/S0533)   |

[SLOTHFULMEDIA](https://attack.mitre.org/software/S0533) has uploaded files and information from victim machines.[[228]](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-275a)

|  [C0024](https://attack.mitre.org/campaigns/C0024)   |  [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024)   |

During the [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024), [APT29](https://attack.mitre.org/groups/G0016) extracted files from compromised networks.[[229]](https://www.volexity.com/blog/2020/12/14/dark-halo-leverages-solarwinds-compromise-to-breach-organizations/)

|  [S0615](https://attack.mitre.org/software/S0615)   |  [SombRAT](https://attack.mitre.org/software/S0615)   |

[SombRAT](https://attack.mitre.org/software/S0615) has collected data and files from a compromised host.[[61]](https://blogs.blackberry.com/en/2020/11/the-costaricto-campaign-cyber-espionage-outsourced)[[230]](https://us-cert.cisa.gov/ncas/analysis-reports/ar21-126a)

|  [S9024](https://attack.mitre.org/software/S9024)   |  [SPAWNCHIMERA](https://attack.mitre.org/software/S9024)   |

[SPAWNCHIMERA](https://attack.mitre.org/software/S9024) has extracted the device’s Linux kernel image (vmlinux).[[231]](https://www.cisa.gov/news-events/analysis-reports/ar25-087a)[[232]](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-exploiting-critical-ivanti-vulnerability)[[233]](https://www.picussecurity.com/resource/blog/unc5221-cve-2025-22457-ivanti-connect-secure)

|  [S0646](https://attack.mitre.org/software/S0646)   |  [SpicyOmelette](https://attack.mitre.org/software/S0646)   |

[SpicyOmelette](https://attack.mitre.org/software/S0646) has collected data and other information from a compromised host.[[234]](https://www.secureworks.com/blog/cybercriminals-increasingly-trying-to-ensnare-the-big-financial-fish)

|  [S1037](https://attack.mitre.org/software/S1037)   |  [STARWHALE](https://attack.mitre.org/software/S1037)   |

[STARWHALE](https://attack.mitre.org/software/S1037) can collect data from an infected local host.[[235]](https://www.cisa.gov/uscert/ncas/alerts/aa22-055a)

|  [S1200](https://attack.mitre.org/software/S1200)   |  [StealBit](https://attack.mitre.org/software/S1200)   |

[StealBit](https://attack.mitre.org/software/S1200) can upload data and files to the LockBit victim-shaming site.[[236]](https://www.ic3.gov/CSA/2022/220204.pdf)[[237]](https://www.cybereason.com/blog/research/threat-analysis-report-inside-the-lockbit-arsenal-the-stealbit-exfiltration-tool)

|  [G0038](https://attack.mitre.org/groups/G0038)   |  [Stealth Falcon](https://attack.mitre.org/groups/G0038)   |

[Stealth Falcon](https://attack.mitre.org/groups/G0038) malware gathers data from the local victim system.[[238]](https://citizenlab.org/2016/05/stealth-falcon/)

|  [S1034](https://attack.mitre.org/software/S1034)   |  [StrifeWater](https://attack.mitre.org/software/S1034)   |

[StrifeWater](https://attack.mitre.org/software/S1034) can collect data from a compromised host.[[239]](https://www.cybereason.com/blog/research/strifewater-rat-iranian-apt-moses-staff-adds-new-trojan-to-ransomware-operations)

|  [S0559](https://attack.mitre.org/software/S0559)   |  [SUNBURST](https://attack.mitre.org/software/S0559)   |

[SUNBURST](https://attack.mitre.org/software/S0559) collected information from a compromised host.[[240]](https://www.microsoft.com/security/blog/2020/12/18/analyzing-solorigate-the-compromised-dll-file-that-started-a-sophisticated-cyberattack-and-how-microsoft-defender-helps-protect/)[[241]](https://www.fireeye.com/blog/threat-research/2020/12/evasive-attacker-leverages-solarwinds-supply-chain-compromises-with-sunburst-backdoor.html)

|  [S1064](https://attack.mitre.org/software/S1064)   |  [SVCReady](https://attack.mitre.org/software/S1064)   |

[SVCReady](https://attack.mitre.org/software/S1064) can collect data from an infected host.[[242]](https://threatresearch.ext.hp.com/svcready-a-new-loader-reveals-itself/)

|  [S0663](https://attack.mitre.org/software/S0663)   |  [SysUpdate](https://attack.mitre.org/software/S0663)   |

[SysUpdate](https://attack.mitre.org/software/S0663) can collect information and files from a compromised host.[[243]](https://www.trendmicro.com/en_us/research/23/c/iron-tiger-sysupdate-adds-linux-targeting.html)

|  [S0011](https://attack.mitre.org/software/S0011)   |  [Taidoor](https://attack.mitre.org/software/S0011)   |

[Taidoor](https://attack.mitre.org/software/S0011) can upload data and files from a victim's machine.[[244]](http://www.trendmicro.com/cloud-content/us/pdfs/security-intelligence/white-papers/wp_the_taidoor_campaign.pdf)

|  [S0467](https://attack.mitre.org/software/S0467)   |  [TajMahal](https://attack.mitre.org/software/S0467)   |

[TajMahal](https://attack.mitre.org/software/S0467) has the ability to steal documents from the local system including the print spooler queue.[[245]](https://securelist.com/project-tajmahal/90240/)

|  [G1056](https://attack.mitre.org/groups/G1056)   |  [TeamPCP](https://attack.mitre.org/groups/G1056)   |

[TeamPCP](https://attack.mitre.org/groups/G1056) has stolen source code from victim environments including Mistral AI.[[246]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  [G0027](https://attack.mitre.org/groups/G0027)   |  [Threat Group-3390](https://attack.mitre.org/groups/G0027)   |

[Threat Group-3390](https://attack.mitre.org/groups/G0027) ran a command to compile an archive of file types of interest from the victim user's directories.[[247]](https://www.secureworks.com/research/bronze-union)

|  [S0665](https://attack.mitre.org/software/S0665)   |  [ThreatNeedle](https://attack.mitre.org/software/S0665)   |

[ThreatNeedle](https://attack.mitre.org/software/S0665) can collect data and files from a compromised host.[[138]](https://securelist.com/lazarus-threatneedle/100803/)

|  [S0668](https://attack.mitre.org/software/S0668)   |  [TinyTurla](https://attack.mitre.org/software/S0668)   |

[TinyTurla](https://attack.mitre.org/software/S0668) can upload files from a compromised host.[[248]](https://blog.talosintelligence.com/2021/09/tinyturla.html)

|  [G1022](https://attack.mitre.org/groups/G1022)   |  [ToddyCat](https://attack.mitre.org/groups/G1022)   |

[ToddyCat](https://attack.mitre.org/groups/G1022) has run scripts to collect documents from targeted hosts.[[144]](https://securelist.com/toddycat-keep-calm-and-check-logs/110696/)

|  [S0671](https://attack.mitre.org/software/S0671)   |  [Tomiris](https://attack.mitre.org/software/S0671)   |

[Tomiris](https://attack.mitre.org/software/S0671) has the ability to collect recent files matching a hardcoded list of extensions prior to exfiltration.[[249]](https://securelist.com/darkhalo-after-solarwinds-the-tomiris-connection/104311/)

|  [S0266](https://attack.mitre.org/software/S0266)   |  [TrickBot](https://attack.mitre.org/software/S0266)   |

[TrickBot](https://attack.mitre.org/software/S0266) collects local files and information from the victim’s local machine.[[250]](https://www.securityartwork.es/wp-content/uploads/2017/07/Trickbot-report-S2-Grupo.pdf)

|  [S1196](https://attack.mitre.org/software/S1196)   |  [Troll Stealer](https://attack.mitre.org/software/S1196)   |

[Troll Stealer](https://attack.mitre.org/software/S1196) gathers information from infected systems such as SSH information from the victim's .ssh directory.[[251]](https://www.security.com/threat-intelligence/springtail-kimsuky-backdoor-espionage) [Troll Stealer](https://attack.mitre.org/software/S1196) collects information from local FileZilla installations and Microsoft Sticky Note.[[252]](https://medium.com/s2wblog/kimsuky-disguised-as-a-korean-company-signed-with-a-valid-certificate-to-distribute-troll-stealer-cfa5d54314e2)

|  [S9009](https://attack.mitre.org/software/S9009)   |  [TruffleHog](https://attack.mitre.org/software/S9009)   |

[TruffleHog](https://attack.mitre.org/software/S9009) has gathered data from home directories of the victim environment.[[253]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)

|  [G0010](https://attack.mitre.org/groups/G0010)   |  [Turla](https://attack.mitre.org/groups/G0010)   |

[Turla](https://attack.mitre.org/groups/G0010) RPC backdoors can upload files from victim machines.[[254]](https://www.welivesecurity.com/2019/05/29/turla-powershell-usage/)

|  [S0275](https://attack.mitre.org/software/S0275)   |  [UPPERCUT](https://attack.mitre.org/software/S0275)   |

[UPPERCUT](https://attack.mitre.org/software/S0275) can upload files to the C2 from infected machines.[[255]](https://www.trendmicro.com/en_us/research/25/d/earth-kasha-updates-ttps.html)[[256]](https://www.trendmicro.com/en_us/research/24/k/return-of-anel-in-the-recent-earth-kasha-spearphishing-campaign.html)

|  [S0022](https://attack.mitre.org/software/S0022)   |  [Uroburos](https://attack.mitre.org/software/S0022)   |

[Uroburos](https://attack.mitre.org/software/S0022) can use its Get command to exfiltrate specified files from the compromised system.[[257]](https://www.cisa.gov/sites/default/files/2023-05/aa23-129a_snake_malware_2.pdf)

|  [S0386](https://attack.mitre.org/software/S0386)   |  [Ursnif](https://attack.mitre.org/software/S0386)   |

[Ursnif](https://attack.mitre.org/software/S0386) has collected files from victim machines, including certificates and cookies.[[258]](https://www.trendmicro.com/vinfo/us/threat-encyclopedia/malware/BKDR_URSNIF.SM?_ga=2.129468940.1462021705.1559742358-1202584019.1549394279)

|  [S0452](https://attack.mitre.org/software/S0452)   |  [USBferry](https://attack.mitre.org/software/S0452)   |

[USBferry](https://attack.mitre.org/software/S0452) can collect information from an air-gapped host machine.[[259]](https://documents.trendmicro.com/assets/Tech-Brief-Tropic-Trooper-s-Back-USBferry-Attack-Targets-Air-gapped-Environments.pdf)

|  [S9050](https://attack.mitre.org/software/S9050)   |  [Vidar](https://attack.mitre.org/software/S9050)   |

[Vidar](https://attack.mitre.org/software/S9050) has attempted to collect browser information.[[260]](https://www.quorumcyber.com/wp-content/uploads/2023/01/Malware-Analysis-Vidar.pdf)

|  [G1055](https://attack.mitre.org/groups/G1055)   |  [VOID MANTICORE](https://attack.mitre.org/groups/G1055)   |

[VOID MANTICORE](https://attack.mitre.org/groups/G1055) has collected cached data and files from within the victim environment.[[261]](https://specopssoft.com/blog/stryker-cyber-attack-what-we-know-remote-wipe/)[[262]](https://www.justice.gov/opa/media/1431956/dl?inline)[[263]](https://www.ic3.gov/CSA/2026/260320.pdf)

|  [G1017](https://attack.mitre.org/groups/G1017)   |  [Volt Typhoon](https://attack.mitre.org/groups/G1017)   |

[Volt Typhoon](https://attack.mitre.org/groups/G1017) has stolen files from a sensitive file server and the Active Directory database from targeted environments, and used [Wevtutil](https://attack.mitre.org/software/S0645) to extract event log information.[[264]](https://media.defense.gov/2023/May/24/2003229517/-1/-1/0/CSA_Living_off_the_Land.PDF)[[265]](https://web.archive.org/web/20230601025540/https://www.secureworks.com/blog/chinese-cyberespionage-group-bronze-silhouette-targets-us-government-and-defense-organizations)[[266]](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)

|  [S0670](https://attack.mitre.org/software/S0670)   |  [WarzoneRAT](https://attack.mitre.org/software/S0670)   |

[WarzoneRAT](https://attack.mitre.org/software/S0670) can collect data from a compromised host.[[267]](https://research.checkpoint.com/2020/warzone-behind-the-enemy-lines/)

|  [S0515](https://attack.mitre.org/software/S0515)   |  [WellMail](https://attack.mitre.org/software/S0515)   |

[WellMail](https://attack.mitre.org/software/S0515) can exfiltrate files from the victim machine.[[268]](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-198c)

|  [S0514](https://attack.mitre.org/software/S0514)   |  [WellMess](https://attack.mitre.org/software/S0514)   |

[WellMess](https://attack.mitre.org/software/S0514) can send files from the victim machine to C2.[[269]](https://www.pwc.co.uk/issues/cyber-security-services/insights/cleaning-up-after-wellmess.html)[[270]](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-198b)

|  [S0645](https://attack.mitre.org/software/S0645)   |  [Wevtutil](https://attack.mitre.org/software/S0645)   |

[Wevtutil](https://attack.mitre.org/software/S0645) can be used to export events from a specific log.[[271]](https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/wevtutil)[[272]](https://web.archive.org/web/20200901113617/https://labs.f-secure.com/assets/BlogFiles/f-secureLABS-tlp-white-lazarus-threat-intel-report2.pdf)

|  [G0124](https://attack.mitre.org/groups/G0124)   |  [Windigo](https://attack.mitre.org/groups/G0124)   |

[Windigo](https://attack.mitre.org/groups/G0124) has used a script to gather credentials in files left on disk by OpenSSH backdoors.[[273]](https://www.welivesecurity.com/wp-content/uploads/2018/12/ESET-The_Dark_Side_of_the_ForSSHe.pdf)

|  [G0102](https://attack.mitre.org/groups/G0102)   |  [Wizard Spider](https://attack.mitre.org/groups/G0102)   |

[Wizard Spider](https://attack.mitre.org/groups/G0102) has collected data from a compromised host prior to exfiltration.[[274]](https://web.archive.org/web/20220313061955/https://www.mandiant.com/sites/default/files/2021-10/fin12-group-profile.pdf)

|  [S1065](https://attack.mitre.org/software/S1065)   |  [Woody RAT](https://attack.mitre.org/software/S1065)   |

[Woody RAT](https://attack.mitre.org/software/S1065) can collect information from a compromised host.[[275]](https://www.malwarebytes.com/blog/threat-intelligence/2022/08/woody-rat-a-new-feature-rich-malware-spotted-in-the-wild)

|  [S0653](https://attack.mitre.org/software/S0653)   |  [xCaon](https://attack.mitre.org/software/S0653)   |

[xCaon](https://attack.mitre.org/software/S0653) has uploaded files from victims' machines.[[37]](https://research.checkpoint.com/2021/indigozebra-apt-continues-to-attack-central-asia-with-evolving-tools/)

|  [S0658](https://attack.mitre.org/software/S0658)   |  [XCSSET](https://attack.mitre.org/software/S0658)   |

[XCSSET](https://attack.mitre.org/software/S0658) collects contacts and application data from files in Desktop, Documents, Downloads, Dropbox, and WeChat folders.[[276]](https://documents.trendmicro.com/assets/pdf/XCSSET_Technical_Brief.pdf)

|  [S0248](https://attack.mitre.org/software/S0248)   |  [yty](https://attack.mitre.org/software/S0248)   |

[yty](https://attack.mitre.org/software/S0248) collects files with the following extensions: .ppt, .pptx, .pdf, .doc, .docx, .xls, .xlsx, .docm, .rtf, .inp, .xlsm, .csv, .odt, .pps, .vcf and sends them back to the C2 server.[[277]](https://www.arbornetworks.com/blog/asert/donot-team-leverages-new-modular-malware-framework-south-asia/)

|  [S0672](https://attack.mitre.org/software/S0672)   |  [Zox](https://attack.mitre.org/software/S0672)   |

[Zox](https://attack.mitre.org/software/S0672) has the ability to upload files from a targeted system.[[22]](https://web.archive.org/web/20230115144216/http://www.novetta.com/wp-content/uploads/2014/11/Executive_Summary-Final_1.pdf)

|  [S0412](https://attack.mitre.org/software/S0412)   |  [ZxShell](https://attack.mitre.org/software/S0412)   |

[ZxShell](https://attack.mitre.org/software/S0412) can transfer files from a compromised host.[[278]](https://blogs.cisco.com/security/talos/opening-zxshell)

|  [S1013](https://attack.mitre.org/software/S1013)   |  [ZxxZ](https://attack.mitre.org/software/S1013)   |

[ZxxZ](https://attack.mitre.org/software/S1013) can collect data from a compromised host.[[279]](https://blog.talosintelligence.com/2022/05/bitter-apt-adds-bangladesh-to-their.html)

## Mitigations

| ID  | Mitigation  | Description

|  [M1057](https://attack.mitre.org/mitigations/M1057)   |  [Data Loss Prevention](https://attack.mitre.org/mitigations/M1057)   |

Data loss prevention can restrict access to sensitive data and detect sensitive data that is unencrypted.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0380](https://attack.mitre.org/detectionstrategies/DET0380)   |  [Detection of Local Data Collection Prior to Exfiltration](https://attack.mitre.org/detectionstrategies/DET0380)   |  [AN1070](https://attack.mitre.org/detectionstrategies/DET0380#AN1070)   |

Adversaries collecting local files via PowerShell, WMI, or direct file API calls often include recursive file listings, targeted file reads, and temporary file staging.

|  [AN1071](https://attack.mitre.org/detectionstrategies/DET0380#AN1071)   |

Adversaries using bash scripts or tools to recursively enumerate user home directories, config files, or SSH keys.

|  [AN1072](https://attack.mitre.org/detectionstrategies/DET0380#AN1072)   |

Adversary use of bash/zsh or AppleScript to locate files and exfil targets like user keychains or documents.

|  [AN1073](https://attack.mitre.org/detectionstrategies/DET0380#AN1073)   |

Collection of device configuration via CLI commands (e.g., show running-config, copy flash, more), often followed by TFTP/SCP transfers.

|  [AN1074](https://attack.mitre.org/detectionstrategies/DET0380#AN1074)   |

Adversaries accessing datastore or configuration files via vim-cmd, esxcli, or SCP to extract logs, VMs, or host configurations.

## References

-    [Cisco. (2022, August 16). show running-config - Cisco IOS Configuration Fundamentals Command Reference . Retrieved July 13, 2022.](https://www.cisco.com/c/en/us/td/docs/ios-xml/ios/fundamentals/command/cf_command_ref/show_protocols_through_showmon.html#wp2760878733)
-    [Threat Intelligence Team. (2021, December 2). SideCopy APT: Connecting lures victims, payloads to infrastructure. Retrieved June 13, 2022.](https://www.malwarebytes.com/blog/news/2021/12/sidecopy-apt-connecting-lures-to-victims-payloads-to-infrastructure)
-    [Or Chechik, Tom Fakterman, Daniel Frank & Assaf Dahan. (2023, November 6). Agonizing Serpens (Aka Agrius) Targeting the Israeli Higher Education and Tech Sectors. Retrieved May 22, 2024.](https://unit42.paloaltonetworks.com/agonizing-serpens-targets-israeli-tech-higher-ed-sectors/)
-    [Kasuya, M. (2020, January 8). Threat Spotlight: Amadey Bot Targets Non-Russian Users. Retrieved July 14, 2022.](https://blogs.blackberry.com/en/2020/01/threat-spotlight-amadey-bot)
-    [FSI. (2017, July 27). Campaign Rifle - Andariel, the Maiden of Anguish. Retrieved September 12, 2024.](https://fsiceat.tistory.com/2)
-    [Anthropic. (2025, November). Disrupting the first reported AI-orchestrated cyber espionage campaign. Retrieved April 20, 2026.](https://assets.anthropic.com/m/ec212e6566a0d47/original/Disrupting-the-first-reported-AI-orchestrated-cyber-espionage-campaign.pdf)
-    [Jazi, H. (2021, June 1). Kimsuky APT continues to target South Korean government using AppleSeed backdoor. Retrieved June 10, 2021.](https://blog.malwarebytes.com/threat-analysis/2021/06/kimsuky-apt-continues-to-target-south-korean-government-using-appleseed-backdoor/)
-    [KISA. (2021). Phishing Target Reconnaissance and Attack Resource Analysis Operation Muzabi. Retrieved March 8, 2024.](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)
-    [Mandiant. (n.d.). APT1 Exposing One of China’s Cyber Espionage Units. Retrieved July 18, 2016.](https://www.fireeye.com/content/dam/fireeye-www/services/pdfs/mandiant-apt1-report.pdf)
-    [Guarnieri, C. (2015, June 19). Digital Attack on German Parliament: Investigative Report on the Hack of the Left Party Infrastructure in Bundestag. Retrieved January 22, 2018.](https://netzpolitik.org/2015/digital-attack-on-german-parliament-investigative-report-on-the-hack-of-the-left-party-infrastructure-in-bundestag/)
-    [Mueller, R. (2018, July 13). Indictment - United States of America vs. VIKTOR BORISOVICH NETYKSHO, et al. Retrieved November 17, 2024.](https://cdn.cnn.com/cnn/2018/images/07/13/gru.indictment.pdf)
-    [Hacquebord, F. (n.d.). Pawn Storm in 2019 A Year of Scanning and Credential Phishing on High-Profile Targets. Retrieved December 29, 2020.](https://documents.trendmicro.com/assets/white_papers/wp-pawn-storm-in-2019.pdf)
-    [NSA, CISA, FBI, NCSC. (2021, July). Russian GRU Conducting Global Brute Force Campaign to Compromise Enterprise and Cloud Environments. Retrieved July 26, 2021.](https://media.defense.gov/2021/Jul/01/2002753896/-1/-1/1/CSA_GRU_GLOBAL_BRUTE_FORCE_CAMPAIGN_UOO158036-21.PDF)
-    [Mandiant. (2022, May 2). UNC3524: Eye Spy on Your Email. Retrieved August 17, 2023.](https://www.mandiant.com/resources/blog/unc3524-eye-spy-email)
-    [valsmith. (2012, September 21). More on APTSim. Retrieved September 28, 2017.](http://carnal0wnage.attackresearch.com/2012/09/more-on-aptsim.html)
-    [FireEye. (2018, February 20). APT37 (Reaper): The Overlooked North Korean Actor. Retrieved November 17, 2024.](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)
-    [DHS/CISA. (2020, August 26). FASTCash 2.0: North Korea's BeagleBoyz Robbing Banks. Retrieved September 29, 2021.](https://us-cert.cisa.gov/ncas/alerts/aa20-239a)
-    [Symantec. (2018, February 28). Chafer: Latest Attacks Reveal Heightened Ambitions. Retrieved May 22, 2020.](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/chafer-latest-attacks-reveal-heightened-ambitions)
-    [FBI. (2020, September 17). Indicators of Compromise Associated with Rana Intelligence Computing, also known as Advanced Persistent Threat 39, Chafer, Cadelspy, Remexi, and ITG07. Retrieved December 10, 2020.](https://www.iranwatch.org/sites/default/files/public-intelligence-alert.pdf)
-    [Rostovcev, N. (2021, June 10). Big airline heist APT41 likely behind a third-party attack on Air India. Retrieved August 26, 2021.](https://www.group-ib.com/blog/colunmtk-apt41/)
-    [CrowdStrike. (2023). 2022 Falcon OverWatch Threat Hunting Report. Retrieved May 20, 2024.](https://go.crowdstrike.com/rs/281-OBQ-266/images/2022OverWatchThreatHuntingReport.pdf)
-    [Novetta. (n.d.). Operation SMN: Axiom Threat Actor Group Report. Retrieved November 12, 2014.](https://web.archive.org/web/20230115144216/http://www.novetta.com/wp-content/uploads/2014/11/Executive_Summary-Final_1.pdf)
-    [Accenture iDefense Unit. (2019, March 5). Mudcarp's Focus on Submarine Technologies. Retrieved August 24, 2021.](https://web.archive.org/web/20210722005437/https://www.accenture.com/_acnmedia/PDF-96/Accenture-Security-MUDCARP.pdf)
-    [Settle, A., et al. (2016, August 8). MONSOON - Analysis Of An APT Campaign. Retrieved September 22, 2016.](https://www.forcepoint.com/sites/default/files/resources/files/forcepoint-security-labs-monsoon-analysis-report.pdf)
-    [Levene, B. et al.. (2018, March 7). Patchwork Continues to Deliver BADNEWS to the Indian Subcontinent. Retrieved March 31, 2018.](https://researchcenter.paloaltonetworks.com/2018/03/unit42-patchwork-continues-deliver-badnews-indian-subcontinent/)
-    [Bar, T., Conant, S. (2017, October 20). BadPatch. Retrieved November 13, 2018.](https://researchcenter.paloaltonetworks.com/2017/10/unit42-badpatch/)
-    [Check Point. (2020, November 26). Bandook: Signed & Delivered. Retrieved May 31, 2021.](https://research.checkpoint.com/2020/bandook-signed-delivered/)
-    [Sherstobitoff, R. (2018, March 08). Hidden Cobra Targets Turkish Financial Sector With New Bankshot Implant. Retrieved May 18, 2018.](https://securingtomorrow.mcafee.com/mcafee-labs/hidden-cobra-targets-turkish-financial-sector-new-bankshot-implant/)
-    [Cybereason Nocturnus. (2020, July 16). A BAZAR OF TRICKS: FOLLOWING TEAM9’S DEVELOPMENT CYCLES. Retrieved November 18, 2020.](https://www.cybereason.com/blog/a-bazar-of-tricks-following-team9s-development-cycles)
-    [Kirill Boychenko. (2025, July 14). Contagious Interview Campaign Escalates With 67 Malicious npm Packages and New Malware Loader. Retrieved October 19, 2025.](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)
-    [Matej Havranek. (2025, February 20). DeceptiveDevelopment targets freelance developers. Retrieved October 17, 2025.](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)
-    [Unit 42. (2023, November 21). Hacking Employers and Seeking Employment: Two Job-Related Campaigns Bear Hallmarks of North Korean Threat Actors. Retrieved October 17, 2025.](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)
-    [Unit42. (2024, October 9). Contagious Interview: DPRK Threat Actors Lure Tech Industry Job Seekers to Install New Variants of BeaverTail and InvisibleFerret Malware. Retrieved October 17, 2025.](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)
-    [Mercer, W., et al. (2020, March 5). Bisonal: 10 years of play. Retrieved January 26, 2022.](https://blog.talosintelligence.com/2020/03/bisonal-10-years-of-play.html)
-    [MSTIC. (2019, December 12). GALLIUM: Targeting global telecom. Retrieved January 13, 2021.](https://www.microsoft.com/security/blog/2019/12/12/gallium-targeting-global-telecom/)
-    [US-CERT. (2020, August 19). MAR-10295134-1.v1 – North Korean Remote Access Trojan: BLINDINGCAN. Retrieved August 19, 2020.](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-232a)
-    [CheckPoint Research. (2021, July 1). IndigoZebra APT continues to attack Central Asia with evolving tools. Retrieved September 24, 2021.](https://research.checkpoint.com/2021/indigozebra-apt-continues-to-attack-central-asia-with-evolving-tools/)
-    [DHS/CISA. (2026, February 11). AR25-338A: BRICKSTORM Backdoor. Retrieved April 16, 2026.](https://www.cisa.gov/news-events/analysis-reports/ar25-338a)
-    [Counter Threat Unit Research Team. (2017, October 12). BRONZE BUTLER Targets Japanese Enterprises. Retrieved January 4, 2018.](https://www.secureworks.com/research/bronze-butler-targets-japanese-businesses)
-    [Harbison, M. and Renals, P. (2022, July 5). When Pentest Tools Go Brutal: Red-Teaming Tool Being Abused by Malicious Actors. Retrieved February 1, 2023.](https://unit42.paloaltonetworks.com/brute-ratel-c4-tool/)
-    [Cybereason. (2022, August 17). Bumblebee Loader – The High Road to Enterprise Domain Control. Retrieved August 29, 2022.](https://www.cybereason.com/blog/threat-analysis-report-bumblebee-loader-the-high-road-to-enterprise-domain-control)
-    [DFIR Report. (2021, November 29). CONTInuing the Bazar Ransomware Story. Retrieved September 29, 2022.](https://thedfirreport.com/2021/11/29/continuing-the-bazar-ransomware-story/)
-    [Rufus Brown, Van Ta, Douglas Bienstock, Geoff Ackerman, John Wolfram. (2022, March 8). Does This Look Infected? A Summary of APT41 Targeting U.S. State Governments. Retrieved July 8, 2022.](https://www.mandiant.com/resources/apt41-us-state-governments)
-    [Hawley, S. et al. (2023, February 2). Turla: A Galaxy of Opportunity. Retrieved May 15, 2023.](https://www.mandiant.com/resources/blog/turla-galaxy-opportunity)
-    [Kuzin, M., Zelensky S. (2018, July 20). Calisto Trojan for macOS. Retrieved September 7, 2018.](https://securelist.com/calisto-trojan-for-macos/86543/)
-    [Marvi, A. et al.. (2023, March 16). Fortinet Zero-Day and Custom Malware Used by Suspected Chinese Actor in Espionage Operation. Retrieved March 22, 2023.](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)
-    [ClearSky Cyber Security. (2021, January). “Lebanese Cedar” APT Global Lebanese Espionage Campaign Leveraging Web Servers. Retrieved February 10, 2021.](https://www.clearskysec.com/wp-content/uploads/2021/01/Lebanese-Cedar-APT.pdf)
-    [Vrabie, V. (2020, November). Dissecting a Chinese APT Targeting South Eastern Asian Government Institutions. Retrieved September 19, 2022.](https://www.bitdefender.com/files/News/CaseStudies/study/379/Bitdefender-Whitepaper-Chinese-APT.pdf)
-    [Check Point. (2022, January 11). APT35 exploits Log4j vulnerability to distribute new modular PowerShell toolkit. Retrieved January 24, 2022.](https://research.checkpoint.com/2022/apt35-exploits-log4j-vulnerability-to-distribute-new-modular-powershell-toolkit/)
-    [Jenkins, L. at al. (2022, August 4). ROADSWEEP Ransomware - Likely Iranian Threat Actor Conducts Politically Motivated Disruptive Activity Against Albanian Government Organizations. Retrieved August 6, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/likely-iranian-threat-actor-conducts-politically-motivated-disruptive-activity-against/)
-    [FireEye. (2018, March 16). Suspected Chinese Cyber Espionage Group (TEMP.Periscope) Targeting U.S. Engineering and Maritime Industries. Retrieved April 11, 2018.](https://www.fireeye.com/blog/threat-research/2018/03/suspected-chinese-espionage-group-targeting-maritime-and-engineering-industries.html)
-    [Lee, T., Hanzlik, D., Ahl, I. (2013, August 7). Breaking Down the China Chopper Web Shell - Part I. Retrieved March 27, 2015.](https://www.fireeye.com/blog/threat-research/2013/08/breaking-down-the-china-chopper-web-shell-part-i.html)
-    [The Australian Cyber Security Centre (ACSC), the Canadian Centre for Cyber Security (CCCS), the New Zealand National Cyber Security Centre (NZ NCSC), CERT New Zealand, the UK National Cyber Security Centre (UK NCSC) and the US National Cybersecurity and Communications Integration Center (NCCIC). (2018, October 11). Joint report on publicly available hacking tools. Retrieved March 11, 2019.](https://www.ncsc.gov.uk/report/joint-report-on-publicly-available-hacking-tools)
-    [Eoin Miller. (2021, March 23). Defending Against the Zero Day: Analyzing Attacker Behavior Post-Exploitation of Microsoft Exchange. Retrieved October 27, 2022.](https://www.rapid7.com/blog/post/2021/03/23/defending-against-the-zero-day-analyzing-attacker-behavior-post-exploitation-of-microsoft-exchange/)
-    [Dupuy, T. and Faou, M. (2021, June). Gelsemium. Retrieved November 30, 2021.](https://www.welivesecurity.com/wp-content/uploads/2021/06/eset_gelsemium.pdf)
-    [Lunghi, D. et al. (2020, February). Uncovering DRBControl. Retrieved November 12, 2021.](https://documents.trendmicro.com/assets/white_papers/wp-uncovering-DRBcontrol.pdf)
-    [Cobalt Strike. (2017, December 8). Tactics, Techniques, and Procedures. Retrieved November 17, 2024.](https://web.archive.org/web/20210924171429/https://www.cobaltstrike.com/downloads/reports/tacticstechniquesandprocedures.pdf)
-    [Strategic Cyber LLC. (2020, November 5). Cobalt Strike: Advanced Threat Tactics for Penetration Testers. Retrieved April 13, 2021.](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)
-    [Chen, y., et al. (2019, January 31). Mac Malware Steals Cryptocurrency Exchanges’ Cookies. Retrieved July 22, 2020.](https://unit42.paloaltonetworks.com/mac-malware-steals-cryptocurrency-exchanges-cookies/)
-    [F-Secure Labs. (2014, July). COSMICDUKE Cosmu with a twist of MiniDuke. Retrieved July 3, 2014.](https://blog.f-secure.com/wp-content/uploads/2019/10/CosmicDuke.pdf)
-    [The BlackBerry Research and Intelligence Team. (2020, November 12). The CostaRicto Campaign: Cyber-Espionage Outsourced. Retrieved May 24, 2021.](https://blogs.blackberry.com/en/2020/11/the-costaricto-campaign-cyber-espionage-outsourced)
-    [Microsoft. (2022, June 2). Exposing POLONIUM activity and infrastructure targeting Israeli organizations. Retrieved July 1, 2022.](https://www.microsoft.com/security/blog/2022/06/02/exposing-polonium-activity-and-infrastructure-targeting-israeli-organizations/)
-    [N. Baisini. (2022, July 13). Transparent Tribe begins targeting education sector in latest campaign. Retrieved September 22, 2022.](https://blog.talosintelligence.com/2022/07/transparent-tribe-targets-education.html)
-    [Faou, M. (2020, December 2). Turla Crutch: Keeping the “back door” open. Retrieved December 4, 2020.](https://www.welivesecurity.com/2020/12/02/turla-crutch-keeping-back-door-open/)
-    [Stokes, P. (2020, July 27). Four Distinct Families of Lazarus Malware Target Apple’s macOS Platform. Retrieved August 7, 2020.](https://www.sentinelone.com/blog/four-distinct-families-of-lazarus-malware-target-apples-macos-platform/)
-    [MSTIC. (2021, November 16). Evolving trends in Iranian threat actor activity – MSTIC presentation at CyberWarCon 2021. Retrieved January 12, 2023.](https://www.microsoft.com/en-us/security/blog/2021/11/16/evolving-trends-in-iranian-threat-actor-activity-mstic-presentation-at-cyberwarcon-2021)
-    [Meltzer, M. et al. (2024, January 10). Active Exploitation of Two Zero-Day Vulnerabilities in Ivanti Connect Secure VPN. Retrieved February 27, 2024.](https://www.volexity.com/blog/2024/01/10/active-exploitation-of-two-zero-day-vulnerabilities-in-ivanti-connect-secure-vpn/)
-    [Lin, M. et al. (2024, January 31). Cutting Edge, Part 2: Investigating Ivanti Connect Secure VPN Zero-Day Exploitation. Retrieved February 27, 2024.](https://www.mandiant.com/resources/blog/investigating-ivanti-zero-day-exploitation)
-    [NCSC. (2022, February 23). Cyclops Blink Malware Analysis Report. Retrieved March 3, 2022.](https://www.ncsc.gov.uk/files/Cyclops-Blink-Malware-Analysis-Report.pdf)
-    [SecureWorks 2019, August 27 LYCEUM Takes Center Stage in Middle East Campaign Retrieved. 2019/11/19](https://www.secureworks.com/blog/lyceum-takes-center-stage-in-middle-east-campaign)
-    [Blaich, A., et al. (2018, January 18). Dark Caracal: Cyber-espionage at a Global Scale. Retrieved April 11, 2018.](https://info.lookout.com/rs/051-ESQ-475/images/Lookout_Dark-Caracal_srr_20180118_us_v.1.0.pdf)
-    [McGraw, T. (2024, December 4). Black Basta Ransomware Campaign Drops Zbot, DarkGate, and Custom Malware. Retrieved December 9, 2024.](https://www.rapid7.com/blog/post/2024/12/04/black-basta-ransomware-campaign-drops-zbot-darkgate-and-custom-malware/)
-    [Smith, S., Stafford, M. (2021, December 14). DarkWatchman: A new evolution in fileless techniques. Retrieved January 10, 2022.](https://web.archive.org/web/20220629230035/https://www.prevailion.com/darkwatchman-new-fileless-techniques/)
-    [El-Banna, M., et al. (2025, November 17). Frontline Intelligence: Analysis of UNC1549 TTPs, Custom Tools, and Malware Targeting the Aerospace and Defense Ecosystem. Retrieved September 28, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)
-    [Shivtarkar, N. and Kumar, A. (2022, June 9). Lyceum .NET DNS Backdoor. Retrieved June 23, 2022.](https://www.zscaler.com/blogs/security-research/lyceum-net-dns-backdoor)
-    [US-CERT. (2018, March 16). Alert (TA18-074A): Russian Government Cyber Activity Targeting Energy and Other Critical Infrastructure Sectors. Retrieved June 6, 2018.](https://www.us-cert.gov/ncas/alerts/TA18-074A)
-    [ClearSky Research Team. (2020, August 13). Operation 'Dream Job' Widespread North Korean Espionage Campaign. Retrieved December 20, 2021.](https://www.clearskysec.com/wp-content/uploads/2020/08/Dream-Job-Campaign.pdf)
-    [NSA/FBI. (2020, August). Russian GRU 85th GTsSS Deploys Previously Undisclosed Drovorub Malware. Retrieved August 25, 2020.](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)
-    [Hod Gavriel. (2019, November 21). Dtrack: In-depth analysis of APT on a nuclear power plant. Retrieved January 20, 2021.](https://www.cyberbit.com/blog/endpoint-security/dtrack-apt-malware-found-in-nuclear-power-plant/)
-    [Mike Stokkel et al. (2024, July 18). APT41 Has Arisen From the DUST. Retrieved September 16, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/apt41-arisen-from-dust)
-    [Microsoft Threat Intelligence. (2023, June 14). Cadet Blizzard emerges as a novel and distinct Russian threat actor. Retrieved July 10, 2023.](https://www.microsoft.com/en-us/security/blog/2023/06/14/cadet-blizzard-emerges-as-a-novel-and-distinct-russian-threat-actor/)
-    [US Cybersecurity & Infrastructure Security Agency et al. (2024, September 5). Russian Military Cyber Actors Target U.S. and Global Critical Infrastructure. Retrieved September 6, 2024.](https://www.cisa.gov/sites/default/files/2024-09/aa24-249a-russian-military-cyber-actors-target-us-and-global-critical-infrastructure.pdf)
-    [MSTIC. (2021, May 28). Breaking down NOBELIUM’s latest early-stage toolset. Retrieved August 4, 2021.](https://www.microsoft.com/security/blog/2021/05/28/breaking-down-nobeliums-latest-early-stage-toolset/)
-    [Red Canary. (2021, March 31). 2021 Threat Detection Report. Retrieved August 31, 2021.](https://resource.redcanary.com/rs/003-YRU-314/images/2021-Threat-Detection-Report.pdf?mkt_tok=MDAzLVlSVS0zMTQAAAF_PIlmhNTaG2McG4X_foM-cIr20UfyB12MIQ10W0HbtMRwxGOJaD0Xj6CRTNg_S-8KniRxtf9xzhz_ACvm_TpbJAIgWCV8yIsFgbhb8cuaZA)
-    [Faou, M., Tartare, M., Dupuy, T. (2019, October). OPERATION GHOST. Retrieved September 23, 2020.](https://www.welivesecurity.com/wp-content/uploads/2019/10/ESET_Operation_Ghost_Dukes.pdf)
-    [Ta, V., et al. (2022, August 8). FIN13: A Cybercriminal Threat Actor Focused on Mexico. Retrieved February 9, 2023.](https://www.mandiant.com/resources/blog/fin13-cybercriminal-mexico)
-    [Sygnia Incident Response Team. (2022, January 5). TG2003: ELEPHANT BEETLE UNCOVERING AN ORGANIZED FINANCIAL-THEFT OPERATION. Retrieved February 9, 2023.](https://web.archive.org/web/20220105132433/https://f.hubspotusercontent30.net/hubfs/8776530/Sygnia-%20Elephant%20Beetle_Jan2022.pdf)
-    [Chen, J. (2019, October 10). Magecart Card Skimmers Injected Into Online Shops. Retrieved September 9, 2020.](https://www.trendmicro.com/en_us/research/19/j/fin6-compromised-e-commerce-platform-via-magecart-to-inject-credit-card-skimmers-into-thousands-of-online-shops.html)
-    [Klijnsma, Y. (2018, September 11). Inside the Magecart Breach of British Airways: How 22 Lines of Code Claimed 380,000 Victims. Retrieved September 9, 2020.](https://web.archive.org/web/20181231220607/https://riskiq.com/blog/labs/magecart-british-airways-breach/)
-    [Klijnsma, Y. (2018, September 19). Another Victim of the Magecart Assault Emerges: Newegg. Retrieved September 9, 2020.](https://web.archive.org/web/20181209083100/https://www.riskiq.com/blog/labs/magecart-newegg/)
-    [Loui, E. and Reynolds, J. (2021, August 30). CARBON SPIDER Embraces Big Game Hunting, Part 1. Retrieved September 20, 2021.](https://www.crowdstrike.com/blog/carbon-spider-embraces-big-game-hunting-part-1/)
-    [Hada, H. (2021, December 28). Flagpro The new malware used by BlackTech. Retrieved March 25, 2022.](https://insight-jp.nttsecurity.com/post/102hf3q/flagpro-the-new-malware-used-by-blacktech)
-    [FireEye Labs. (2015, April). APT30 AND THE MECHANICS OF A LONG-RUNNING CYBER ESPIONAGE OPERATION. Retrieved November 17, 2024.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2015/05/20081935/rpt-apt30.pdf)
-    [Financial Security Institute. (2020, February 28). Profiling of TA505 Threat Group That Continues to Attack the Financial Sector. Retrieved July 14, 2022.](https://www.fsec.or.kr/user/bbs/fsec/163/344/bbsDataView/1382.do?page=1&column=&search=&searchSDate=&searchEDate=&bbsDataCategory=)
-    [Ramin Nafisi. (2021, September 27). FoggyWeb: Targeted NOBELIUM malware leads to persistent backdoor. Retrieved October 4, 2021.](https://www.microsoft.com/security/blog/2021/09/27/foggyweb-targeted-nobelium-malware-leads-to-persistent-backdoor/)
-    [CISA. (2020, September 15). Iran-Based Threat Actor Exploits VPN Vulnerabilities. Retrieved December 21, 2020.](https://us-cert.cisa.gov/ncas/alerts/aa20-259a)
-    [Kremez, V. (2019, September 19). FIN6 “FrameworkPOS”: Point-of-Sale Malware Analysis & Internals. Retrieved September 8, 2020.](https://labs.sentinelone.com/fin6-frameworkpos-point-of-sale-malware-analysis-internals-2/)
-    [Adamitis, D. et al. (2019, June 4). It's alive: Threat actors cobble together open-source pieces into monstrous Frankenstein campaign. Retrieved May 11, 2020.](https://blog.talosintelligence.com/2019/06/frankenstein-campaign.html)
-    [Global Research and Analysis Team. (2020, April 30). APT trends report Q1 2020. Retrieved September 19, 2022.](https://securelist.com/apt-trends-report-q1-2020/96826/)
-    [Cybereason Nocturnus. (2019, June 25). Operation Soft Cell: A Worldwide Campaign Against Telecommunications Providers. Retrieved July 18, 2019.](https://www.cybereason.com/blog/operation-soft-cell-a-worldwide-campaign-against-telecommunications-providers)
-    [Boutin, J. (2020, June 11). Gamaredon group grows its game. Retrieved June 16, 2020.](https://www.welivesecurity.com/2020/06/11/gamaredon-group-grows-its-game/)
-    [Rusnák, Z. (2024, September 26). Cyberespionage the Gamaredon way: Analysis of toolset used to spy on Ukraine in 2022 and 2023. Retrieved October 30, 2024.](https://web-assets.esetstatic.com/wls/en/papers/white-papers/cyberespionage-gamaredon-way.pdf)
-    [Kirill Boychenko. (2026, January 31). GlassWorm Loader Hits Open VSX via Developer Account Compromise. Retrieved April 10, 2026.](https://socket.dev/blog/glassworm-loader-hits-open-vsx-via-suspected-developer-account-compromise)
-    [Dahan, A. (2017). Operation Cobalt Kitty. Retrieved December 27, 2018.](https://cdn2.hubspot.net/hubfs/3354902/Cybereason%20Labs%20Analysis%20Operation%20Cobalt%20Kitty.pdf)
-    [Mercer, W., Rascagneres, P. (2018, April 26). GravityRAT - The Two-Year Evolution Of An APT Targeting India. Retrieved May 16, 2018.](https://blog.talosintelligence.com/2018/04/gravityrat-two-year-evolution-of-apt.html)
-    [Sandvik, Runa. (2021, October 1). Made In America: Green Lambert for OS X. Retrieved March 21, 2022.](https://objective-see.com/blog/blog_0x68.html)
-    [Priego, A. (2021, July). THE BROTHERS GRIM: THE REVERSING TALE OF GRIMAGENT MALWARE USED BY RYUK. Retrieved September 19, 2024.](https://www.group-ib.com/blog/grimagent/)
-    [Microsoft Threat Intelligence . (2025, March 5). Silk Typhoon targeting IT supply chain. Retrieved March 20, 2025.](https://www.microsoft.com/en-us/security/blog/2025/03/05/silk-typhoon-targeting-it-supply-chain/)
-    [Ungur, P. (n.d.). HAVOC. Retrieved August 4, 2025.](https://havocframework.com/docs/welcome)
-    [Immersive Content Team. (2024, April 9). Havoc C2 Framework – A Defensive Operator’s Guide. Retrieved August 13, 2025.](https://www.immersivelabs.com/resources/blog/havoc-c2-framework-a-defensive-operators-guide)
-    [Trend Micro. (2024, November 19). Spot the Difference: Earth Kasha's New LODEINFO Campaign And The Correlation Analysis With The APT10 Umbrella. Retrieved April 17, 2026.](https://www.trendmicro.com/en_us/research/24/k/lodeinfo-campaign-of-earth-kasha.html)
-    [Tomonaga, S. (2024, July 16). MirrorFace Attack against Japanese Organisations. Retrieved April 17, 2026.](https://blogs.jpcert.or.jp/en/2024/07/mirrorface-attack-against-japanese-organisations.html)
-    [Symantec Security Response. (2010, January 18). The Trojan.Hydraq Incident. Retrieved February 20, 2018.](https://www.symantec.com/connect/blogs/trojanhydraq-incident)
-    [Lelli, A. (2010, January 11). Trojan.Hydraq. Retrieved February 20, 2018.](https://www.symantec.com/security_response/writeup.jsp?docid=2010-011114-1830-99)
-    [CrowdStrike. (2022, May). ICEAPPLE: A NOVEL INTERNET INFORMATION SERVICES (IIS) POST-EXPLOITATION FRAMEWORK. Retrieved June 27, 2022.](https://www.crowdstrike.com/wp-content/uploads/2022/05/crowdstrike-iceapple-a-novel-internet-information-services-post-exploitation-framework.pdf)
-    [GReAT. (2019, August 12). Recent Cloud Atlas activity. Retrieved May 8, 2020.](https://securelist.com/recent-cloud-atlas-activity/92016/)
-    [Hromcová, Z. (2018, June 07). InvisiMole: Surprisingly equipped spyware, undercover since 2013. Retrieved July 10, 2018.](https://www.welivesecurity.com/2018/06/07/invisimole-equipped-spyware-undercover/)
-    [Amitai Ben & Shushan Ehrlich. (2021, May). From Wiper to Ransomware: The Evolution of Agrius. Retrieved May 21, 2024.](https://assets.sentinelone.com/sentinellabs/evol-agrius)
-    [Sancho, D., et al. (2012, May 22). IXESHE An APT Campaign. Retrieved June 7, 2019.](https://www.trendmicro.de/cloud-content/us/pdfs/security-intelligence/white-papers/wp_ixeshe.pdf)
-    [Levene, B, et al. (2017, May 03). Kazuar: Multiplatform Espionage Backdoor with API Access. Retrieved July 17, 2018.](https://researchcenter.paloaltonetworks.com/2017/05/unit42-kazuar-multiplatform-espionage-backdoor-api-access/)
-    [Villeneuve, N., Bennett, J. T., Moran, N., Haq, T., Scott, M., & Geers, K. (2014). OPERATION “KE3CHANG”: Targeted Attacks Against Ministries of Foreign Affairs. Retrieved November 12, 2014.](https://www.mandiant.com/resources/operation-ke3chang-targeted-attacks-against-ministries-of-foreign-affairs)
-    [MSTIC. (2021, December 6). NICKEL targeting government organizations across Latin America and Europe. Retrieved March 18, 2022.](https://www.microsoft.com/security/blog/2021/12/06/nickel-targeting-government-organizations-across-latin-america-and-europe)
-    [Kayal, A. et al. (2021, October). LYCEUM REBORN: COUNTERINTELLIGENCE IN THE MIDDLE EAST. Retrieved June 14, 2022.](https://vblocalhost.com/uploads/VB2021-Kayal-etal.pdf)
-    [Dahan, A. et al. (2020, November 2). Back to the Future: Inside the Kimsuky KGH Spyware Suite. Retrieved November 6, 2020.](https://www.cybereason.com/blog/back-to-the-future-inside-the-kimsuky-kgh-spyware-suite)
-    [Tarakanov , D.. (2013, September 11). The “Kimsuky” Operation: A North Korean APT?. Retrieved August 13, 2019.](https://securelist.com/the-kimsuky-operation-a-north-korean-apt/57915/)
-    [An, J and Malhotra, A. (2021, November 10). North Korean attackers use malicious blogs to deliver malware to high-profile South Korean targets. Retrieved December 29, 2021.](https://blog.talosintelligence.com/2021/11/kimsuky-abuses-blogs-delivers-malware.html)
-    [Varadharajan Krishnasamy, Aditya K Sood. (2025, July 29). From Reconnaissance to Control: The Operational Blueprint of Kimsuky APT for Cyber Espionage. Retrieved April 18, 2026.](https://www.aryaka.com/docs/reports/aryaka-kimsuky-apt-operational-blueprint.pdf)
-    [Magius, J., et al. (2017, July 19). Koadic. Retrieved September 27, 2024.](https://github.com/offsecginger/koadic)
-    [Jazi, H. (2021, February). LazyScripter: From Empire to double RAT. Retrieved November 17, 2024.](https://web.archive.org/web/20211003035156/https://www.malwarebytes.com/resources/files/2021/02/lazyscripter.pdf)
-    [Threat Intelligence Team. (2021, August 23). New variant of Konni malware used in campaign targetting Russia. Retrieved January 5, 2022.](https://blog.malwarebytes.com/threat-intelligence/2021/08/new-variant-of-konni-malware-used-in-campaign-targetting-russia/)
-    [Conteras, T., Splunk Research Team. (2025, September 25). From Prompt to Payload: LAMEHUG’s LLM-Driven Cyber Intrusion. Retrieved April 21, 2026.](https://www.splunk.com/en_us/blog/security/lamehug-ai-driven-malware-llm-cyber-intrusion-analysis.html)
-    [Google Threat Intelligence Group. (2025, November 5). GTIG AI Threat Tracker: Advances in Threat Actor Usage of AI Tools. Retrieved March 31, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/threat-actor-usage-of-ai-tools)
-    [MSTIC, DART, M365 Defender. (2022, March 24). DEV-0537 Criminal Actor Targeting Organizations for Data Exfiltration and Destruction. Retrieved May 17, 2022.](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)
-    [Batista, J. (2024, June 17). Latrodectus, are you coming back?. Retrieved September 13, 2024.](https://www.bitsight.com/blog/latrodectus-are-you-coming-back)
-    [Novetta Threat Research Group. (2016, February 24). Operation Blockbuster: Unraveling the Long Thread of the Sony Attack. Retrieved February 25, 2016.](https://web.archive.org/web/20160226161828/https://www.operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Report.pdf)
-    [Novetta Threat Research Group. (2016, February 24). Operation Blockbuster: Loaders, Installers and Uninstallers Report. Retrieved November 17, 2024.](https://web.archive.org/web/20190508165631/https://operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Loaders-Installers-and-Uninstallers-Report.pdf)
-    [Novetta Threat Research Group. (2016, February 24). Operation Blockbuster: Remote Administration Tools & Content Staging Malware Report. Retrieved March 16, 2016.](https://web.archive.org/web/20220608001455/https://www.operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-RAT-and-Staging-Report.pdf)
-    [Vyacheslav Kopeytsev and Seongsu Park. (2021, February 25). Lazarus targets defense industry with ThreatNeedle. Retrieved October 27, 2021.](https://securelist.com/lazarus-threatneedle/100803/)
-    [Faou, M. (2019, May). Turla LightNeuron: One email away from remote code execution. Retrieved June 24, 2019.](https://www.welivesecurity.com/wp-content/uploads/2019/05/ESET-LightNeuron.pdf)
-    [Zhou, R. (2012, May 15). Backdoor.Linfo. Retrieved February 23, 2018.](https://www.symantec.com/security_response/writeup.jsp?docid=2012-051605-2535-99)

-    [Ishimaru, S. (2022, October 31). APT10: Tracking down LODEINFO 2022, part II. Retrieved April 17, 2026.](https://securelist.com/apt10-tracking-down-lodeinfo-2022-part-ii/107745/)
-    [Breitenbacher, D. (2022, December 14). Unmasking MirrorFace: Operation LiberalFace targeting Japanese political entities. Retrieved April 17, 2026.](https://www.welivesecurity.com/2022/12/14/unmasking-mirrorface-operation-liberalface-targeting-japanese-political-entities/)
-    [ITOCHU. (2024, January 24). The Endless Struggle Against APT10: Insights from LODEINFO v0.6.6 - v0.7.3 Analysis. Retrieved April 17, 2026.](https://blog-en.itochuci.co.jp/entry/2024/01/24/134100)
-    [Dedola, G. et al. (2023, October 12). ToddyCat: Keep calm and check logs. Retrieved January 3, 2024.](https://securelist.com/toddycat-keep-calm-and-check-logs/110696/)
-    [Lechtik, M, and etl. (2021, July 14). LuminousMoth APT: Sweeping attacks for the chosen few. Retrieved October 20, 2022.](https://securelist.com/apt-luminousmoth/103332/)
-    [Botezatu, B and etl. (2021, July 21). LuminousMoth - PlugX, File Exfiltration and Persistence Revisited. Retrieved October 20, 2022.](https://www.bitdefender.com/blog/labs/luminousmoth-plugx-file-exfiltration-and-persistence-revisited)
-    [ESET. (2019, July). MACHETE JUST GOT SHARPER Venezuelan government institutions under attack. Retrieved September 13, 2019.](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)
-    [M.Léveillé, M., Cherepanov, A.. (2022, January 25). Watering hole deploys new macOS malware, DazzleSpy, in Asia. Retrieved May 6, 2022.](https://www.welivesecurity.com/2022/01/25/watering-hole-deploys-new-macos-malware-dazzlespy-asia/)
-    [Ehrlich, A., et al. (2022, September). THE MYSTERY OF METADOR | AN UNATTRIBUTED THREAT HIDING IN TELCOS, ISPS, AND UNIVERSITIES. Retrieved January 23, 2023.](https://assets.sentinelone.com/sentinellabs22/metador#page=1)
-    [DFIR Report. (2022, March 21). APT35 Automates Initial Access Using ProxyShell. Retrieved May 25, 2022.](https://thedfirreport.com/2022/03/21/apt35-automates-initial-access-using-proxyshell)
-    [DFIR Report. (2021, November 15). Exchange Exploit Leads to Domain Wide Ransomware. Retrieved January 5, 2023.](https://thedfirreport.com/2021/11/15/exchange-exploit-leads-to-domain-wide-ransomware/)
-    [GReAT. (2021, June 16). Ferocious Kitten: 6 Years of Covert Surveillance in Iran. Retrieved September 22, 2021.](https://securelist.com/ferocious-kitten-6-years-of-covert-surveillance-in-iran/102806/)
-    [Secureworks. (2019, July 24). MCMD Malware Analysis. Retrieved August 13, 2020.](https://www.secureworks.com/research/mcmd-malware-analysis)
-    [United States District Court Southern District of New York (USDC SDNY) . (2018, December 17). United States of America v. Zhu Hua and Zhang Shilong. Retrieved April 17, 2019.](https://www.justice.gov/opa/pr/two-chinese-hackers-associated-ministry-state-security-charged-global-computer-intrusion)
-    [Symantec. (2020, November 17). Japan-Linked Organizations Targeted in Long-Running and Sophisticated Attack Campaign. Retrieved December 17, 2020.](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/cicada-apt10-japan-espionage)
-    [SentinelLabs. (2022, September 22). Metador Technical Appendix. Retrieved April 4, 2023.](https://docs.google.com/document/d/1e9ZTW9b71YwFWS_18ZwDAxa-cYbV8q1wUefmKZLYVsA/edit#heading=h.lmnbtht1ikzm)
-    [Facundo Muñoz. (2023, April 26). Evasive Panda APT group delivers malware via updates for popular Chinese software. Retrieved July 25, 2024.](https://www.welivesecurity.com/2023/04/26/evasive-panda-apt-group-malware-updates-popular-chinese-software/)
-    [ClearSky Cyber Security . (2021, August). New Iranian Espionage Campaign By “Siamesekitten” - Lyceum. Retrieved June 6, 2022.](https://www.clearskysec.com/siamesekitten/)
-    [Rozmann, O., et al. (2024, February 27). When Cats Fly: Suspected Iranian Threat Actor UNC1549 Targets Israeli and Middle East Aerospace and Defense Sectors. Retrieved September 28, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/suspected-iranian-unc1549-targets-israel-middle-east)
-    [Check Point. (2026, May 22). Fast and Furious – Nimbus Manticore Operations During the Iranian Conflict. Retrieved September 23, 2026.](https://research.checkpoint.com/2026/fast-and-furious-nimbus-manticore-operations-during-the-iranian-conflict/)
-    [Gross, J. (2016, February 23). Operation Dust Storm. Retrieved December 22, 2021.](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)
-    [Falcone, R. and Miller-Osborn, J.. (2016, January 24). Scarlet Mimic: Years-Long Espionage Campaign Targets Minority Activists. Retrieved February 10, 2016.](http://researchcenter.paloaltonetworks.com/2016/01/scarlet-mimic-years-long-espionage-targets-minority-activists/)
-    [Chen, Joey. (2022, June 9). Aoqin Dragon | Newly-Discovered Chinese-linked APT Has Been Quietly Spying On Organizations For 10 Years. Retrieved July 14, 2022.](https://www.sentinelone.com/labs/aoqin-dragon-newly-discovered-chinese-linked-apt-has-been-quietly-spying-on-organizations-for-10-years/)
-    [Vrabie, V. (2021, April 23). NAIKON – Traces from a Military Cyber-Espionage Operation. Retrieved June 29, 2021.](https://www.bitdefender.com/files/News/CaseStudies/study/396/Bitdefender-PR-Whitepaper-NAIKON-creat5397-en-EN.pdf)
-    [McAfee® Foundstone® Professional Services and McAfee Labs™. (2011, February 10). Global Energy Cyberattacks: “Night Dragon”. Retrieved February 19, 2018.](https://scadahacker.com/library/Documents/Cyber_Events/McAfee%20-%20Night%20Dragon%20-%20Global%20Energy%20Cyberattacks.pdf)
-    [Faou, M. (2023, August 10). MoustachedBouncer: Espionage against foreign diplomats in Belarus. Retrieved September 25, 2023.](https://www.welivesecurity.com/en/eset-research/moustachedbouncer-espionage-against-foreign-diplomats-in-belarus/)
-    [Fidelis Cybersecurity. (2013, June 28). Fidelis Threat Advisory #1009: "njRAT" Uncovered. Retrieved June 4, 2019.](https://www.threatminer.org/_reports/2013/fta-1009---njrat-uncovered-1.pdf)
-    [Dray Agha. (2022, August 16). Cleartext Shenanigans: Gifting User Passwords to Adversaries With NPPSPY. Retrieved May 17, 2024.](https://www.huntress.com/blog/cleartext-shenanigans-gifting-user-passwords-to-adversaries-with-nppspy)
-    [Cherepanov, A. (2018, October 4). Nomadic Octopus Cyber espionage in Central Asia. Retrieved October 13, 2021.](https://www.virusbulletin.com/uploads/pdf/conference_slides/2018/Cherepanov-VB2018-Octopus.pdf)
-    [Fahmy, M. et al. (2024, October 11). Earth Simnavaz (aka APT34) Levies Advanced Cyberattacks Against Middle East. Retrieved November 27, 2024.](https://www.trendmicro.com/en_us/research/24/j/earth-simnavaz-cyberattacks.html)
-    [Cybereason Nocturnus. (2022, May 4). Operation CuckooBees: Deep-Dive into Stealthy Winnti Techniques. Retrieved September 22, 2022.](https://www.cybereason.com/blog/operation-cuckoobees-deep-dive-into-stealthy-winnti-techniques)
-    [Cashman, M. (2020, July 29). Operation North Star Campaign. Retrieved December 20, 2021.](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/operation-north-star-a-job-offer-thats-too-good-to-be-true/?hilite=%27Operation%27%2C%27North%27%2C%27Star%27)
-    [Sherstobitoff, R. (2018, March 02). McAfee Uncovers Operation Honeybee, a Malicious Document Campaign Targeting Humanitarian Aid Groups. Retrieved May 16, 2018.](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/mcafee-uncovers-operation-honeybee-malicious-document-campaign-targeting-humanitarian-aid-groups/)
-    [Volexity Threat Research. (2024, April 12). Zero-Day Exploitation of Unauthenticated Remote Code Execution Vulnerability in GlobalProtect (CVE-2024-3400). Retrieved November 20, 2024.](https://www.volexity.com/blog/2024/04/12/zero-day-exploitation-of-unauthenticated-remote-code-execution-vulnerability-in-globalprotect-cve-2024-3400/)
-    [Dantzig, M. v., Schamper, E. (2019, December 19). Operation Wocao: Shining a light on one of China’s hidden hacking groups. Retrieved October 8, 2020.](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)
-    [Magisa, L. (2020, November 27). New MacOS Backdoor Connected to OceanLotus Surfaces. Retrieved December 2, 2020.](https://www.trendmicro.com/en_us/research/20/k/new-macos-backdoor-connected-to-oceanlotus-surfaces.html)
-    [Peretz, A. and Theck, E. (2021, March 5). Earth Vetala – MuddyWater Continues to Target Organizations in the Middle East. Retrieved March 18, 2021.](https://www.trendmicro.com/en_us/research/21/c/earth-vetala---muddywater-continues-to-target-organizations-in-t.html)
-    [Unit 42. (2022, February 25). Spear Phishing Attacks Target Organizations in Ukraine, Payloads Include the Document Stealer OutSteel and the Downloader SaintBot. Retrieved June 9, 2022.](https://unit42.paloaltonetworks.com/ukraine-targeted-outsteel-saintbot/)
-    [ANSSI. (2021, January 27). SANDWORM INTRUSION SET CAMPAIGN TARGETING CENTREON SYSTEMS. Retrieved March 30, 2021.](https://www.cert.ssi.gouv.fr/uploads/CERTFR-2021-CTI-005.pdf)
-    [Mullaney, C. & Honda, H. (2012, May 4). Trojan.Pasam. Retrieved February 22, 2018.](https://www.symantec.com/security_response/writeup.jsp?docid=2012-050412-4128-99)
-    [Cymmetria. (2016). Unveiling Patchwork - The Copy-Paste APT. Retrieved November 17, 2024.](https://web.archive.org/web/20180825085952/https:/s3-us-west-2.amazonaws.com/cymmetria-blog/public/Unveiling_Patchwork.pdf)
-    [Trustwave SpiderLabs. (2020, June 22). Pillowmint: FIN7’s Monkey Thief . Retrieved July 27, 2020.](https://www.trustwave.com/en-us/resources/blogs/spiderlabs-blog/pillowmint-fin7s-monkey-thief/)
-    [F-Secure Labs. (2015, September 17). The Dukes: 7 years of Russian cyberespionage. Retrieved December 10, 2015.](https://www.f-secure.com/documents/996508/1030745/dukes_whitepaper.pdf)
-    [Unit 42. (2022, June 13). GALLIUM Expands Targeting Across Telecommunications, Government and Finance Sectors With New PingPull Tool. Retrieved August 7, 2022.](https://unit42.paloaltonetworks.com/pingpull-gallium/)
-    [Hayashi, K. (2005, August 18). Backdoor.Darkmoon. Retrieved February 23, 2018.](https://www.symantec.com/security_response/writeup.jsp?docid=2005-081910-3934-99)
-    [Cybereason Nocturnus. (2022, February 1). PowerLess Trojan: Iranian APT Phosphorus Adds New PowerShell Backdoor for Espionage. Retrieved June 1, 2022.](https://www.cybereason.com/blog/research/powerless-trojan-iranian-apt-phosphorus-adds-new-powershell-backdoor-for-espionage)
-    [PowerShellMafia. (2012, May 26). PowerSploit - A PowerShell Post-Exploitation Framework. Retrieved February 6, 2018.](https://github.com/PowerShellMafia/PowerSploit)
-    [PowerSploit. (n.d.). PowerSploit. Retrieved February 6, 2018.](http://powersploit.readthedocs.io)
-    [Singh, S. et al.. (2018, March 13). Iranian Threat Group Updates Tactics, Techniques and Procedures in Spear Phishing Campaign. Retrieved April 11, 2018.](https://www.fireeye.com/blog/threat-research/2018/03/iranian-threat-group-updates-ttps-in-spear-phishing-campaign.html)
-    [Sherstobitoff, R., Malhotra, A. (2018, April 24). Analyzing Operation GhostSecret: Attack Seeks to Steal Data Worldwide. Retrieved May 16, 2018.](https://securingtomorrow.mcafee.com/mcafee-labs/analyzing-operation-ghostsecret-attack-seeks-to-steal-data-worldwide/)
-    [Kizhakkinan, D., et al. (2016, May 11). Threat Actor Leverages Windows Zero-day Exploit in Payment Card Data Attacks. Retrieved February 12, 2018.](https://www.fireeye.com/blog/threat-research/2016/05/windows-zero-day-payment-cards.html)
-    [Elovitz, S. & Ahl, I. (2016, August 18). Know Your Enemy: New Financially-Motivated & Spear-Phishing Group. Retrieved February 26, 2018.](https://www2.fireeye.com/WBNR-Know-Your-Enemy-UNC622-Spear-Phishing.html)
-    [Rainey, K. (n.d.). Qbot. Retrieved September 27, 2021.](https://redcanary.com/threat-detection-report/threats/qbot/)
-    [Kuzmenko, A. et al. (2021, September 2). QakBot technical analysis. Retrieved September 27, 2021.](https://securelist.com/qakbot-technical-analysis/103931/)
-    [CISA. (2018, December 18). Analysis Report (AR18-352A) Quasar Open-Source Remote Administration Tool. Retrieved August 1, 2022.](https://www.cisa.gov/uscert/ncas/analysis-reports/AR18-352A)
-    [Microsoft Threat Intelligence Center. (2022, February 4). ACTINIUM targets Ukrainian organizations. Retrieved February 18, 2022.](https://www.microsoft.com/security/blog/2022/02/04/actinium-targets-ukrainian-organizations/)
-    [S2W TALON. (2022, June 16). Raccoon Stealer is Back with a New Version. Retrieved August 1, 2024.](https://medium.com/s2wblog/raccoon-stealer-is-back-with-a-new-version-5f436e04b20d)
-    [Pierre Le Bourhis, Quentin Bourgue, & Sekoia TDR. (2022, June 29). Raccoon Stealer v2 - Part 2: In-depth analysis. Retrieved August 1, 2024.](https://blog.sekoia.io/raccoon-stealer-v2-part-2-in-depth-analysis/)
-    [Sanmillan, I.. (2020, May 13). Ramsay: A cyber‑espionage toolkit tailored for air‑gapped networks. Retrieved May 27, 2020.](https://www.welivesecurity.com/2020/05/13/ramsay-cyberespionage-toolkit-airgapped-networks/)
-    [Antiy CERT. (2020, April 20). Analysis of Ramsay components of Darkhotel's infiltration and isolation network. Retrieved March 24, 2021.](https://www.programmersought.com/article/62493896999/)
-    [Perez, D. et al. (2021, May 27). Re-Checking Your Pulse: Updates on Chinese APT Actors Compromising Pulse Secure VPN Devices. Retrieved February 5, 2024.](https://www.mandiant.com/resources/blog/updates-on-chinese-apt-compromising-pulse-secure-vpn-devices)
-    [Nesbit, B. and Ackerman, D. (2017, January). Malware Analysis Report - RawPOS Malware: Deconstructing an Intruder’s Toolkit. Retrieved October 4, 2017.](https://www.kroll.com/en/insights/publications/malware-analysis-report-rawpos-malware)
-    [TrendLabs Security Intelligence Blog. (2015, April). RawPOS Technical Brief. Retrieved October 4, 2017.](http://sjc1-te-ftp.trendmicro.com/images/tex/pdf/RawPOS%20Technical%20Brief.pdf)
-    [Bromiley, M. and Lewis, P. (2016, October 7). Attacking the Hospitality and Gaming Industries: Tracking an Attacker Around the World in 7 Years. Retrieved October 6, 2017.](https://www.youtube.com/watch?v=fevGZs0EQu8)
-    [Global Threat Center, Intelligence Team. (2020, December). APT27 Turns to Ransomware. Retrieved November 12, 2021.](https://web.archive.org/web/20210104144857/https://shared-public-reports.s3-eu-west-1.amazonaws.com/APT27+turns+to+ransomware.pdf)
-    [Group-IB. (2020, August). RedCurl: The Pentest You Didn’t Know About. Retrieved August 9, 2024.](https://www.group-ib.com/resources/research-hub/red-curl/)
-    [Group-IB. (2021, November). RedCurl: The Awakening. Retrieved August 14, 2024.](https://www.group-ib.com/resources/research-hub/red-curl-2/)
-    [Alexandre Cote Cyr. (2024, November 8). Life on a crooked RedLine: Analyzing the infamous infostealer’s backend. Retrieved September 17, 2025.](https://www.welivesecurity.com/en/eset-research/life-crooked-redline-analyzing-infamous-infostealers-backend/)
-    [Sherstobitoff, R., Malhotra, A., et. al.. (2018, December 18). Operation Sharpshooter Campaign Targets Global Defense, Critical Infrastructure. Retrieved May 14, 2020.](https://www.mcafee.com/enterprise/en-us/assets/reports/rp-operation-sharpshooter.pdf)
-    [Pantazopoulos, N.. (2018, November 8). RokRat Analysis. Retrieved May 21, 2020.](https://research.nccgroup.com/2018/11/08/rokrat-analysis/)
-    [Cash, D., Grunzweig, J., Adair, S., Lancaster, T. (2021, August 25). North Korean BLUELIGHT Special: InkySquid Deploys RokRAT. Retrieved October 1, 2021.](https://www.volexity.com/blog/2021/08/24/north-korean-bluelight-special-inkysquid-deploys-rokrat/)
-    [Jazi, Hossein. (2021, January 6). Retrohunting APT37: North Korean APT used VBA self decode technique to inject RokRat. Retrieved March 22, 2022.](https://blog.malwarebytes.com/threat-analysis/2021/01/retrohunting-apt37-north-korean-apt-used-vba-self-decode-technique-to-inject-rokrat/)
-    [Ray, V., Hayashi, K. (2016, February 29). New Malware ‘Rover’ Targets Indian Ambassador to Afghanistan. Retrieved February 29, 2016.](http://researchcenter.paloaltonetworks.com/2016/02/new-malware-rover-targets-indian-ambassador-to-afghanistan/)
-    [Hasherezade. (2021, April 6). A deep dive into Saint Bot, a new downloader. Retrieved June 9, 2022.](https://blog.malwarebytes.com/threat-intelligence/2021/04/a-deep-dive-into-saint-bot-downloader/)
-    [CISA. (2025, September 3). Countering Chinese State-Sponsored Actors Compromise of Networks Worldwide to Feed Global Espionage System. Retrieved September 22, 2026.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)
-    [Dedola, G. (2022, June 21). APT ToddyCat. Retrieved January 3, 2024.](https://securelist.com/toddycat/106799/)
-    [Scott W. Brady. (2020, October 15). United States vs. Yuriy Sergeyevich Andrienko et al.. Retrieved November 25, 2020.](https://www.justice.gov/opa/press-release/file/1328521/download)
-    [Symantec Threat Hunter Team. (2023, July 18). FIN8 Uses Revamped Sardonic Backdoor to Deliver Noberus Ransomware. Retrieved August 9, 2023.](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/syssphinx-fin8-backdoor)
-    [Schwarz, D. et al. (2019, October 16). TA505 Distributes New SDBbot Remote Access Trojan with Get2 Downloader. Retrieved May 29, 2020.](https://www.proofpoint.com/us/threat-insight/post/ta505-distributes-new-sdbbot-remote-access-trojan-get2-downloader)
-    [Microsoft Threat Intelligence. (2025, July 22). Disrupting active exploitation of on-premises SharePoint vulnerabilities. Retrieved October 15, 2025.](https://www.microsoft.com/en-us/security/blog/2025/07/22/disrupting-active-exploitation-of-on-premises-sharepoint-vulnerabilities/#storm-2603)
-    [ESET Research. (2025, July 24). ToolShell: An all-you-can-eat buffet for threat actors. Retrieved October 15, 2025.](https://www.welivesecurity.com/en/eset-research/toolshell-an-all-you-can-eat-buffet-for-threat-actors/)
-    [Kenin, S. et al. (2025, July 21). SharePoint ToolShell | Zero-Day Exploited in-the-Wild Targets Enterprise Servers. Retrieved October 15, 2025.](https://www.sentinelone.com/blog/sharepoint-toolshell-zero-day-exploited-in-the-wild-targets-enterprise-servers/)
-    [Unit 42. (2025, July 31). Active Exploitation of Microsoft SharePoint Vulnerabilities: Threat Brief (Updated). Retrieved October 15, 2025.](https://unit42.paloaltonetworks.com/microsoft-sharepoint-cve-2025-49704-cve-2025-49706-cve-2025-53770/)
-    [Accenture. (2021, November 9). Who are latest targets of cyber group Lyceum?. Retrieved June 16, 2022.](https://www.accenture.com/us-en/blogs/cyber-defense/iran-based-lyceum-campaigns)
-    [Yonathan Klijnsma. (2016, May 17). Mofang: A politically motivated information stealing adversary. Retrieved May 12, 2020.](https://foxitsecurity.files.wordpress.com/2016/06/fox-it_mofang_threatreport_tlp-white.pdf)
-    [Check Point. (2021, April 8). Iran’s APT34 Returns with an Updated Arsenal. Retrieved May 5, 2021.](https://research.checkpoint.com/2021/irans-apt34-returns-with-an-updated-arsenal/)
-    [Perez, D. et al. (2021, April 20). Check Your Pulse: Suspected APT Actors Leverage Authentication Bypass Techniques and Pulse Secure Zero-Day. Retrieved February 5, 2024.](https://www.mandiant.com/resources/blog/suspected-apt-actors-leverage-bypass-techniques-pulse-secure-zero-day)
-    [DHS/CISA, Cyber National Mission Force. (2020, October 1). Malware Analysis Report (MAR) MAR-10303705-1.v1 – Remote Access Trojan: SLOTHFULMEDIA. Retrieved October 2, 2020.](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-275a)
-    [Cash, D. et al. (2020, December 14). Dark Halo Leverages SolarWinds Compromise to Breach Organizations. Retrieved December 29, 2020.](https://www.volexity.com/blog/2020/12/14/dark-halo-leverages-solarwinds-compromise-to-breach-organizations/)
-    [CISA. (2021, May 6). Analysis Report (AR21-126A) FiveHands Ransomware. Retrieved June 7, 2021.](https://us-cert.cisa.gov/ncas/analysis-reports/ar21-126a)
-    [DHS/CISA. (2026, February 26). MAR-25993211-r1.v2 Ivanti Connect Secure (RESURGE): AR25-087A. Retrieved April 17, 2026.](https://www.cisa.gov/news-events/analysis-reports/ar25-087a)
-    [John Wolfram, Michael Edie, Jacob Thompson, Matt Lin, Josh Murchie. (2025, April 3). Suspected China-Nexus Threat Actor Actively Exploiting Critical Ivanti Connect Secure Vulnerability (CVE-2025-22457). Retrieved April 13, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-exploiting-critical-ivanti-vulnerability)
-    [Sila Ozeren Hacioglu. (2025, May 5). UNC5221’s Latest Exploit: Weaponizing CVE-2025-22457 in Ivanti Connect Secure. Retrieved April 13, 2026.](https://www.picussecurity.com/resource/blog/unc5221-cve-2025-22457-ivanti-connect-secure)
-    [CTU. (2018, September 27). Cybercriminals Increasingly Trying to Ensnare the Big Financial Fish. Retrieved September 20, 2021.](https://www.secureworks.com/blog/cybercriminals-increasingly-trying-to-ensnare-the-big-financial-fish)
-    [FBI, CISA, CNMF, NCSC-UK. (2022, February 24). Iranian Government-Sponsored Actors Conduct Cyber Operations Against Global Government and Commercial Networks. Retrieved September 27, 2022.](https://www.cisa.gov/uscert/ncas/alerts/aa22-055a)
-    [FBI. (2022, February 4). Indicators of Compromise Associated with LockBit 2.0 Ransomware. Retrieved January 24, 2025.](https://www.ic3.gov/CSA/2022/220204.pdf)
-    [Cybereason Global SOC Team. (n.d.). THREAT ANALYSIS REPORT: Inside the LockBit Arsenal - The StealBit Exfiltration Tool. Retrieved January 29, 2025.](https://www.cybereason.com/blog/research/threat-analysis-report-inside-the-lockbit-arsenal-the-stealbit-exfiltration-tool)
-    [Marczak, B. and Scott-Railton, J.. (2016, May 29). Keep Calm and (Don’t) Enable Macros: A New Threat Actor Targets UAE Dissidents. Retrieved June 8, 2016.](https://citizenlab.org/2016/05/stealth-falcon/)
-    [Cybereason Nocturnus. (2022, February 1). StrifeWater RAT: Iranian APT Moses Staff Adds New Trojan to Ransomware Operations. Retrieved August 15, 2022.](https://www.cybereason.com/blog/research/strifewater-rat-iranian-apt-moses-staff-adds-new-trojan-to-ransomware-operations)
-    [MSTIC. (2020, December 18). Analyzing Solorigate, the compromised DLL file that started a sophisticated cyberattack, and how Microsoft Defender helps protect customers . Retrieved January 5, 2021.](https://www.microsoft.com/security/blog/2020/12/18/analyzing-solorigate-the-compromised-dll-file-that-started-a-sophisticated-cyberattack-and-how-microsoft-defender-helps-protect/)
-    [FireEye. (2020, December 13). Highly Evasive Attacker Leverages SolarWinds Supply Chain to Compromise Multiple Global Victims With SUNBURST Backdoor. Retrieved January 4, 2021.](https://www.fireeye.com/blog/threat-research/2020/12/evasive-attacker-leverages-solarwinds-supply-chain-compromises-with-sunburst-backdoor.html)
-    [Schlapfer, Patrick. (2022, June 6). A New Loader Gets Ready. Retrieved December 13, 2022.](https://threatresearch.ext.hp.com/svcready-a-new-loader-reveals-itself/)
-    [Daniel Lunghi. (2023, March 1). Iron Tiger’s SysUpdate Reappears, Adds Linux Targeting. Retrieved March 20, 2023.](https://www.trendmicro.com/en_us/research/23/c/iron-tiger-sysupdate-adds-linux-targeting.html)
-    [Trend Micro. (2012). The Taidoor Campaign. Retrieved November 12, 2014.](http://www.trendmicro.com/cloud-content/us/pdfs/security-intelligence/white-papers/wp_the_taidoor_campaign.pdf)
-    [GReAT. (2019, April 10). Project TajMahal – a sophisticated new APT framework. Retrieved October 14, 2019.](https://securelist.com/project-tajmahal/90240/)
-    [Flashpoint. (2026, May 28). The Mini Shai-Hulud Worm and the New Era of CI/CD Exploitation. Retrieved July 16, 2026.](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)
-    [Counter Threat Unit Research Team. (2017, June 27). BRONZE UNION Cyberespionage Persists Despite Disclosures. Retrieved July 13, 2017.](https://www.secureworks.com/research/bronze-union)
-    [Cisco Talos. (2021, September 21). TinyTurla - Turla deploys new malware to keep a secret backdoor on victim machines. Retrieved December 2, 2021.](https://blog.talosintelligence.com/2021/09/tinyturla.html)
-    [Kwiatkoswki, I. and Delcher, P. (2021, September 29). DarkHalo After SolarWinds: the Tomiris connection. Retrieved December 27, 2021.](https://securelist.com/darkhalo-after-solarwinds-the-tomiris-connection/104311/)
-    [Salinas, M., Holguin, J. (2017, June). Evolution of Trickbot. Retrieved July 31, 2018.](https://www.securityartwork.es/wp-content/uploads/2017/07/Trickbot-report-S2-Grupo.pdf)
-    [Symantec Threat Hunter Team. (2024, May 16). Springtail: New Linux Backdoor Added to Toolkit. Retrieved January 17, 2025.](https://www.security.com/threat-intelligence/springtail-kimsuky-backdoor-espionage)
-    [Jiho Kim & Sebin Lee, S2W. (2024, February 7). Kimsuky disguised as a Korean company signed with a valid certificate to distribute Troll Stealer (English ver.). Retrieved January 17, 2025.](https://medium.com/s2wblog/kimsuky-disguised-as-a-korean-company-signed-with-a-valid-certificate-to-distribute-troll-stealer-cfa5d54314e2)
-    [Gianpietro Cutolo. (2025, November 26). Shai-Hulud 2.0: Aggressive, Automated, and Fast Spreading. Retrieved April 9, 2026.](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)
-    [Faou, M. and Dumont R.. (2019, May 29). A dive into Turla PowerShell usage. Retrieved June 14, 2019.](https://www.welivesecurity.com/2019/05/29/turla-powershell-usage/)
-    [Hiroaki, H. (2025, April 30). Earth Kasha Updates TTPs in Latest Campaign Targeting Taiwan and Japan. Retrieved April 17, 2026.](https://www.trendmicro.com/en_us/research/25/d/earth-kasha-updates-ttps.html)
-    [Hiroaki, H. (2024, November 26). Guess Who’s Back - The Return of ANEL in the Recent Earth Kasha Spear-phishing Campaign in 2024. Retrieved April 17, 2026.](https://www.trendmicro.com/en_us/research/24/k/return-of-anel-in-the-recent-earth-kasha-spearphishing-campaign.html)
-    [FBI et al. (2023, May 9). Hunting Russian Intelligence “Snake” Malware. Retrieved June 8, 2023.](https://www.cisa.gov/sites/default/files/2023-05/aa23-129a_snake_malware_2.pdf)
-    [Sioting, S. (2013, June 15). BKDR_URSNIF.SM. Retrieved June 5, 2019.](https://www.trendmicro.com/vinfo/us/threat-encyclopedia/malware/BKDR_URSNIF.SM?_ga=2.129468940.1462021705.1559742358-1202584019.1549394279)
-    [Chen, J.. (2020, May 12). Tropic Trooper’s Back: USBferry Attack Targets Air gapped Environments. Retrieved May 20, 2020.](https://documents.trendmicro.com/assets/Tech-Brief-Tropic-Trooper-s-Back-USBferry-Attack-Targets-Air-gapped-Environments.pdf)
-    [Quorum Cyber. (n.d.). Malware Analysis Report Vidar - Stealerware. Retrieved August 28, 2026.](https://www.quorumcyber.com/wp-content/uploads/2023/01/Malware-Analysis-Vidar.pdf)
-    [David Ketler. (2026, March 30). Stryker Cyber-Attack: What we Know so Far About the Remote Wipe Attack. Retrieved April 20, 2026.](https://specopssoft.com/blog/stryker-cyber-attack-what-we-know-remote-wipe/)
-    [DOJ/FBI. (2026, March 19). Case 1:26-mj-00683-CDA: Affidavit in Support of Seizure Warrant: In the Matter of the Seizure of Domain Names Justicehomeland[.]org; karmabelow80[.]org; handala-hack[.]to; and handala-redwatned[.]to. Retrieved April 20, 2026.](https://www.justice.gov/opa/media/1431956/dl?inline)
-    [FBI. (2026, March 20). FBI Flash: FLASH-20260320-001:Government of Iran Cyber Actors Deploy Telegram C2 to Push Malware to Identified Targets. Retrieved April 20, 2026.](https://www.ic3.gov/CSA/2026/260320.pdf)
-    [NSA et al. (2023, May 24). People's Republic of China State-Sponsored Cyber Actor Living off the Land to Evade Detection. Retrieved July 27, 2023.](https://media.defense.gov/2023/May/24/2003229517/-1/-1/0/CSA_Living_off_the_Land.PDF)
-    [Counter Threat Unit Research Team. (2023, May 24). Chinese Cyberespionage Group BRONZE SILHOUETTE Targets U.S. Government and Defense Organizations. Retrieved July 27, 2023.](https://web.archive.org/web/20230601025540/https://www.secureworks.com/blog/chinese-cyberespionage-group-bronze-silhouette-targets-us-government-and-defense-organizations)
-    [CISA et al.. (2024, February 7). PRC State-Sponsored Actors Compromise and Maintain Persistent Access to U.S. Critical Infrastructure. Retrieved May 15, 2024.](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)
-    [Harakhavik, Y. (2020, February 3). Warzone: Behind the enemy lines. Retrieved December 17, 2021.](https://research.checkpoint.com/2020/warzone-behind-the-enemy-lines/)
-    [CISA. (2020, July 16). MAR-10296782-3.v1 – WELLMAIL. Retrieved September 29, 2020.](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-198c)
-    [PWC. (2020, July 16). How WellMess malware has been used to target COVID-19 vaccines. Retrieved September 24, 2020.](https://www.pwc.co.uk/issues/cyber-security-services/insights/cleaning-up-after-wellmess.html)
-    [CISA. (2020, July 16). MAR-10296782-2.v1 – WELLMESS. Retrieved September 24, 2020.](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-198b)
-    [Microsoft. (n.d.). wevtutil. Retrieved September 14, 2021.](https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/wevtutil)
-    [F-Secure Labs. (2020, August 18). Lazarus Group Campaign Targeting the Cryptocurrency Vertical. Retrieved September 1, 2020.](https://web.archive.org/web/20200901113617/https://labs.f-secure.com/assets/BlogFiles/f-secureLABS-tlp-white-lazarus-threat-intel-report2.pdf)
-    [Dumont, R., M.Léveillé, M., Porcher, H. (2018, December 1). THE DARK SIDE OF THE FORSSHE A landscape of OpenSSH backdoors. Retrieved July 16, 2020.](https://www.welivesecurity.com/wp-content/uploads/2018/12/ESET-The_Dark_Side_of_the_ForSSHe.pdf)
-    [Shilko, J., et al. (2021, October 7). FIN12: The Prolific Ransomware Intrusion Threat Actor That Has Aggressively Pursued Healthcare Targets. Retrieved June 15, 2023.](https://web.archive.org/web/20220313061955/https://www.mandiant.com/sites/default/files/2021-10/fin12-group-profile.pdf)
-    [MalwareBytes Threat Intelligence Team. (2022, August 3). Woody RAT: A new feature-rich malware spotted in the wild. Retrieved December 6, 2022.](https://www.malwarebytes.com/blog/threat-intelligence/2022/08/woody-rat-a-new-feature-rich-malware-spotted-in-the-wild)
-    [Mac Threat Response, Mobile Research Team. (2020, August 13). The XCSSET Malware: Inserts Malicious Code Into Xcode Projects, Performs UXSS Backdoor Planting in Safari, and Leverages Two Zero-day Exploits. Retrieved October 5, 2021.](https://documents.trendmicro.com/assets/pdf/XCSSET_Technical_Brief.pdf)
-    [Schwarz, D., Sopko J. (2018, March 08). Donot Team Leverages New Modular Malware Framework in South Asia. Retrieved June 11, 2018.](https://www.arbornetworks.com/blog/asert/donot-team-leverages-new-modular-malware-framework-south-asia/)
-    [Allievi, A., et al. (2014, October 28). Threat Spotlight: Group 72, Opening the ZxShell. Retrieved September 24, 2019.](https://blogs.cisco.com/security/talos/opening-zxshell)
-    [Raghuprasad, C . (2022, May 11). Bitter APT adds Bangladesh to their targets. Retrieved June 1, 2022.](https://blog.talosintelligence.com/2022/05/bitter-apt-adds-bangladesh-to-their.html)

×
