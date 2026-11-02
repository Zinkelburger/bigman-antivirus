---
url: "https://attack.mitre.org/techniques/T1056/001/"
title: "Input Capture: Keylogging, Sub-technique T1056.001 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:28:54Z"
via: "live"
sha256: "4a4ba9bb15e381662b7ac6aaccd29dd8d3ecd9d0e9de7fe6b5d9c6bc60407387"
wayback: "https://web.archive.org/web/20261019131445/https://attack.mitre.org/techniques/T1056/001/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Input Capture: Keylogging, Sub-technique T1056.001 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1056/001/>
> Snapshot: <https://web.archive.org/web/20261019131445/https://attack.mitre.org/techniques/T1056/001/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Input Capture](https://attack.mitre.org/techniques/T1056)
- Keylogging

#  Input Capture: Keylogging

##### Other sub-techniques of Input Capture (4)

| ID  | Name

|  T1056.001   |  Keylogging

|  [T1056.002](https://attack.mitre.org/techniques/T1056/002/)   |  [GUI Input Capture](https://attack.mitre.org/techniques/T1056/002/)

|  [T1056.003](https://attack.mitre.org/techniques/T1056/003/)   |  [Web Portal Capture](https://attack.mitre.org/techniques/T1056/003/)

|  [T1056.004](https://attack.mitre.org/techniques/T1056/004/)   |  [Credential API Hooking](https://attack.mitre.org/techniques/T1056/004/)

Adversaries may log user keystrokes to intercept credentials as the user types them. Keylogging is likely to be used to acquire credentials for new access opportunities when [OS Credential Dumping](https://attack.mitre.org/techniques/T1003) efforts are not effective, and may require an adversary to intercept keystrokes on a system for a substantial period of time before credentials can be successfully captured. In order to increase the likelihood of capturing credentials quickly, an adversary may also perform actions such as clearing browser cookies to force users to reauthenticate to systems.[[1]](https://blog.talosintelligence.com/2021/11/kimsuky-abuses-blogs-delivers-malware.html)

Keylogging is the most prevalent type of input capture, with many different ways of intercepting keystrokes.[[2]](http://opensecuritytraining.info/Keylogging_files/The%20Adventures%20of%20a%20Keystroke.pdf) Some methods include:

- Hooking API callbacks used for processing keystrokes. Unlike [Credential API Hooking](https://attack.mitre.org/techniques/T1056/004), this focuses solely on API functions intended for processing keystroke data.
- Reading raw keystroke data from the hardware buffer.
- Windows Registry modifications.
- Custom drivers.
- [Modify System Image](https://attack.mitre.org/techniques/T1601) may provide adversaries with hooks into the operating system of network devices to read raw keystrokes for login sessions.[[3]](https://community.cisco.com/t5/security-blogs/attackers-continue-to-target-legacy-devices/ba-p/4169954)

ID: T1056.001

Sub-technique of:  [T1056](https://attack.mitre.org/techniques/T1056)

ⓘ

Tactics: [Collection](https://attack.mitre.org/tactics/TA0009), [Credential Access](https://attack.mitre.org/tactics/TA0006)

ⓘ

Platforms: Linux, Network Devices, Windows, macOS

Contributors: TruKno

Version: 1.3

Created: 11 February 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1056/001/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1056/001/)

## Procedure Examples

| ID  | Name  | Description

|  [C0028](https://attack.mitre.org/campaigns/C0028)   |  [2015 Ukraine Electric Power Attack](https://attack.mitre.org/campaigns/C0028)   |

During the [2015 Ukraine Electric Power Attack](https://attack.mitre.org/campaigns/C0028), [Sandworm Team](https://attack.mitre.org/groups/G0034) gathered account credentials via a [BlackEnergy](https://attack.mitre.org/software/S0089) keylogger plugin. [[4]](https://www.boozallen.com/content/dam/boozallen/documents/2016/09/ukraine-report-when-the-lights-went-out.pdf)[[5]](https://nsarchive.gwu.edu/sites/default/files/documents/3891751/SANS-and-Electricity-Information-Sharing-and.pdf)

|  [S0045](https://attack.mitre.org/software/S0045)   |  [ADVSTORESHELL](https://attack.mitre.org/software/S0045)   |

[ADVSTORESHELL](https://attack.mitre.org/software/S0045) can perform keylogging.[[6]](http://www.welivesecurity.com/wp-content/uploads/2016/10/eset-sednit-part-2.pdf)[[7]](https://download.bitdefender.com/resources/media/materials/white-papers/en/Bitdefender_In-depth_analysis_of_APT28%E2%80%93The_Political_Cyber-Espionage.pdf)

|  [S0331](https://attack.mitre.org/software/S0331)   |  [Agent Tesla](https://attack.mitre.org/software/S0331)   |

[Agent Tesla](https://attack.mitre.org/software/S0331) can log keystrokes on the victim’s machine.[[8]](https://blog.talosintelligence.com/2018/10/old-dog-new-tricks-analysing-new-rtf_15.html)[[9]](https://www.digitrustgroup.com/agent-tesla-keylogger/)[[10]](https://www.fortinet.com/blog/threat-research/in-depth-analysis-of-net-malware-javaupdtr.html)[[11]](https://labs.bitdefender.com/2020/04/oil-gas-spearphishing-campaigns-drop-agent-tesla-spyware-in-advance-of-historic-opec-deal/)[[12]](https://labs.sentinelone.com/agent-tesla-old-rat-uses-new-tricks-to-stay-on-top/)

|  [G0130](https://attack.mitre.org/groups/G0130)   |  [Ajax Security Team](https://attack.mitre.org/groups/G0130)   |

[Ajax Security Team](https://attack.mitre.org/groups/G0130) has used CWoolger and MPK, custom-developed malware, which recorded all keystrokes on an infected system.[[13]](https://blog.checkpoint.com/wp-content/uploads/2015/11/rocket-kitten-report.pdf)

|  [S0622](https://attack.mitre.org/software/S0622)   |  [AppleSeed](https://attack.mitre.org/software/S0622)   |

[AppleSeed](https://attack.mitre.org/software/S0622) can use GetKeyState and GetKeyboardState to capture keystrokes on the victim’s machine.[[14]](https://blog.malwarebytes.com/threat-analysis/2021/06/kimsuky-apt-continues-to-target-south-korean-government-using-appleseed-backdoor/)[[15]](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[APT28](https://attack.mitre.org/groups/G0007) has used tools to perform keylogging.[[16]](http://download.microsoft.com/download/4/4/C/44CDEF0E-7924-4787-A56A-16261691ACE3/Microsoft_Security_Intelligence_Report_Volume_19_English.pdf)[[17]](https://cdn.cnn.com/cnn/2018/images/07/13/gru.indictment.pdf)[[18]](https://www.trendmicro.com/en_us/research/20/l/pawn-storm-lack-of-sophistication-as-a-strategy.html)

|  [G0022](https://attack.mitre.org/groups/G0022)   |  [APT3](https://attack.mitre.org/groups/G0022)   |

[APT3](https://attack.mitre.org/groups/G0022) has used a keylogging tool that records keystrokes in encrypted files.[[19]](https://web.archive.org/web/20160910124439/http://www.symantec.com/connect/blogs/buckeye-cyberespionage-group-shifts-gaze-us-hong-kong)

|  [G0050](https://attack.mitre.org/groups/G0050)   |  [APT32](https://attack.mitre.org/groups/G0050)   |

[APT32](https://attack.mitre.org/groups/G0050) has abused the PasswordChangeNotify to monitor for and capture account password changes.[[20]](https://cdn2.hubspot.net/hubfs/3354902/Cybereason%20Labs%20Analysis%20Operation%20Cobalt%20Kitty.pdf)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) used a Trojan called KEYLIME to capture keystrokes from the victim’s machine.[[21]](https://services.google.com/fh/files/misc/apt38-un-usual-suspects.pdf)

|  [G0087](https://attack.mitre.org/groups/G0087)   |  [APT39](https://attack.mitre.org/groups/G0087)   |

[APT39](https://attack.mitre.org/groups/G0087) has used tools for capturing keystrokes.[[22]](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/chafer-latest-attacks-reveal-heightened-ambitions)[[23]](https://www.iranwatch.org/sites/default/files/public-intelligence-alert.pdf)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) used a keylogger called GEARSHIFT on a target system.[[24]](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)

|  [G1044](https://attack.mitre.org/groups/G1044)   |  [APT42](https://attack.mitre.org/groups/G1044)   |

[APT42](https://attack.mitre.org/groups/G1044) has used custom malware to log keystrokes.[[25]](https://services.google.com/fh/files/misc/apt42-crooked-charms-cons-and-compromises.pdf)

|  [G1023](https://attack.mitre.org/groups/G1023)   |  [APT5](https://attack.mitre.org/groups/G1023)   |

[APT5](https://attack.mitre.org/groups/G1023) has used malware with keylogging capabilities to monitor the communications of targeted entities.[[26]](https://web.archive.org/web/20220122121143/https://www.fireeye.com/content/dam/fireeye-www/current-threats/pdfs/rpt-southeast-asia-threat-landscape.pdf)[[27]](https://www.mandiant.com/resources/insights/apt-groups)

|  [S0373](https://attack.mitre.org/software/S0373)   |  [Astaroth](https://attack.mitre.org/software/S0373)   |

[Astaroth](https://attack.mitre.org/software/S0373) logs keystrokes from the victim's machine. [[28]](https://web.archive.org/web/20200302071436/https://cofense.com/seeing-resurgence-demonic-astaroth-wmic-trojan/)

|  [S1087](https://attack.mitre.org/software/S1087)   |  [AsyncRAT](https://attack.mitre.org/software/S1087)   |

[AsyncRAT](https://attack.mitre.org/software/S1087) can capture keystrokes on the victim’s machine.[[29]](https://github.com/NYAN-x-CAT/AsyncRAT-C-Sharp/blob/master/README.md)

|  [S0438](https://attack.mitre.org/software/S0438)   |  [Attor](https://attack.mitre.org/software/S0438)   |

One of [Attor](https://attack.mitre.org/software/S0438)'s plugins can collect user credentials via capturing keystrokes and can capture keystrokes pressed within the window of the injected process.[[30]](https://www.welivesecurity.com/wp-content/uploads/2019/10/ESET_Attor.pdf)

|  [S0414](https://attack.mitre.org/software/S0414)   |  [BabyShark](https://attack.mitre.org/software/S0414)   |

[BabyShark](https://attack.mitre.org/software/S0414) has a [PowerShell](https://attack.mitre.org/techniques/T1059/001)-based remote administration ability that can implement a PowerShell or C# based keylogger.[[31]](https://unit42.paloaltonetworks.com/babyshark-malware-part-two-attacks-continue-using-kimjongrat-and-pcrat/)

|  [S0128](https://attack.mitre.org/software/S0128)   |  [BADNEWS](https://attack.mitre.org/software/S0128)   |

When it first starts, [BADNEWS](https://attack.mitre.org/software/S0128) spawns a new thread to log keystrokes.[[32]](https://www.forcepoint.com/sites/default/files/resources/files/forcepoint-security-labs-monsoon-analysis-report.pdf)[[33]](https://researchcenter.paloaltonetworks.com/2018/03/unit42-patchwork-continues-deliver-badnews-indian-subcontinent/)[[34]](https://documents.trendmicro.com/assets/tech-brief-untangling-the-patchwork-cyberespionage-group.pdf)

|  [S0337](https://attack.mitre.org/software/S0337)   |  [BadPatch](https://attack.mitre.org/software/S0337)   |

[BadPatch](https://attack.mitre.org/software/S0337) has a keylogging capability.[[35]](https://researchcenter.paloaltonetworks.com/2017/10/unit42-badpatch/)

|  [S0234](https://attack.mitre.org/software/S0234)   |  [Bandook](https://attack.mitre.org/software/S0234)   |

[Bandook](https://attack.mitre.org/software/S0234) contains keylogging capabilities.[[36]](https://www.blackhat.com/docs/us-16/materials/us-16-Quintin-When-Governments-Attack-State-Sponsored-Malware-Attacks-Against-Activists-Lawyers-And-Journalists.pdf)

|  [S0017](https://attack.mitre.org/software/S0017)   |  [BISCUIT](https://attack.mitre.org/software/S0017)   |

[BISCUIT](https://attack.mitre.org/software/S0017) can capture keystrokes.[[37]](https://www.mandiant.com/sites/default/files/2021-09/mandiant-apt1-report.pdf)

|  [S0089](https://attack.mitre.org/software/S0089)   |  [BlackEnergy](https://attack.mitre.org/software/S0089)   |

[BlackEnergy](https://attack.mitre.org/software/S0089) has run a keylogger plug-in on a victim.[[38]](https://securelist.com/be2-custom-plugins-router-abuse-and-target-profiles/67353/)

|  [S1226](https://attack.mitre.org/software/S1226)   |  [BOOKWORM](https://attack.mitre.org/software/S1226)   |

[BOOKWORM](https://attack.mitre.org/software/S1226) has used its KBLogger.dll module to capture keystrokes and stored them in a folder. [[39]](https://unit42.paloaltonetworks.com/bookworm-trojan-a-model-of-modular-architecture/)

|  [S0454](https://attack.mitre.org/software/S0454)   |  [Cadelspy](https://attack.mitre.org/software/S0454)   |

[Cadelspy](https://attack.mitre.org/software/S0454) has the ability to log keystrokes on the compromised host.[[40]](https://www.symantec.com/connect/blogs/iran-based-attackers-use-back-door-threats-spy-middle-eastern-targets)

|  [S0030](https://attack.mitre.org/software/S0030)   |  [Carbanak](https://attack.mitre.org/software/S0030)   |

[Carbanak](https://attack.mitre.org/software/S0030) logs key strokes for configured processes and sends them back to the C2 server.[[41]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/08064518/Carbanak_APT_eng.pdf)[[42]](https://www.fireeye.com/blog/threat-research/2017/06/behind-the-carbanak-backdoor.html)

|  [S0348](https://attack.mitre.org/software/S0348)   |  [Cardinal RAT](https://attack.mitre.org/software/S0348)   |

[Cardinal RAT](https://attack.mitre.org/software/S0348) can log keystrokes.[[43]](https://researchcenter.paloaltonetworks.com/2017/04/unit42-cardinal-rat-active-two-years/)

|  [S0261](https://attack.mitre.org/software/S0261)   |  [Catchamas](https://attack.mitre.org/software/S0261)   |

[Catchamas](https://attack.mitre.org/software/S0261) collects keystrokes from the victim’s machine.[[44]](https://web.archive.org/web/20190508165711/https://www-west.symantec.com/content/symantec/english/en/security-center/writeup.html/2018-040209-1742-99)

|  [S1149](https://attack.mitre.org/software/S1149)   |  [CHIMNEYSWEEP](https://attack.mitre.org/software/S1149)   |

[CHIMNEYSWEEP](https://attack.mitre.org/software/S1149) has the ability to support keylogging.[[45]](https://cloud.google.com/blog/topics/threat-intelligence/likely-iranian-threat-actor-conducts-politically-motivated-disruptive-activity-against/)

|  [S0023](https://attack.mitre.org/software/S0023)   |  [CHOPSTICK](https://attack.mitre.org/software/S0023)   |

[CHOPSTICK](https://attack.mitre.org/software/S0023) is capable of performing keylogging.[[46]](https://www.crowdstrike.com/blog/bears-midst-intrusion-democratic-national-committee/)[[6]](http://www.welivesecurity.com/wp-content/uploads/2016/10/eset-sednit-part-2.pdf)[[17]](https://cdn.cnn.com/cnn/2018/images/07/13/gru.indictment.pdf)

|  [S0660](https://attack.mitre.org/software/S0660)   |  [Clambling](https://attack.mitre.org/software/S0660)   |

[Clambling](https://attack.mitre.org/software/S0660) can capture keystrokes on a compromised host.[[47]](https://documents.trendmicro.com/assets/white_papers/wp-uncovering-DRBcontrol.pdf)[[48]](https://www.talent-jump.com/article/2020/02/17/CLAMBLING-A-New-Backdoor-Base-On-Dropbox-en/)

|  [S0154](https://attack.mitre.org/software/S0154)   |  [Cobalt Strike](https://attack.mitre.org/software/S0154)   |

[Cobalt Strike](https://attack.mitre.org/software/S0154) can track key presses with a keylogger module.[[49]](https://web.archive.org/web/20210825130434/https://cobaltstrike.com/downloads/csmanual38.pdf)[[50]](https://www.amnestyusa.org/wp-content/uploads/2021/02/Click-and-Bait_Vietnamese-Human-Rights-Defenders-Targeted-with-Spyware-Attacks.pdf)[[51]](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)

|  [S0338](https://attack.mitre.org/software/S0338)   |  [Cobian RAT](https://attack.mitre.org/software/S0338)   |

[Cobian RAT](https://attack.mitre.org/software/S0338) has a feature to perform keylogging on the victim’s machine.[[52]](https://www.zscaler.com/blogs/research/cobian-rat-backdoored-rat)

|  [S1235](https://attack.mitre.org/software/S1235)   |  [CorKLOG](https://attack.mitre.org/software/S1235)   |

[CorKLOG](https://attack.mitre.org/software/S1235) has captured keystrokes.[[53]](https://www.zscaler.com/blogs/security-research/latest-mustang-panda-arsenal-paklog-corklog-and-splatcloak-p2)

|  [S0050](https://attack.mitre.org/software/S0050)   |  [CosmicDuke](https://attack.mitre.org/software/S0050)   |

[CosmicDuke](https://attack.mitre.org/software/S0050) uses a keylogger.[[54]](https://www.f-secure.com/documents/996508/1030745/dukes_whitepaper.pdf)

|  [S0115](https://attack.mitre.org/software/S0115)   |  [Crimson](https://attack.mitre.org/software/S0115)   |

[Crimson](https://attack.mitre.org/software/S0115) can use a module to perform keylogging on compromised hosts.[[55]](https://www.proofpoint.com/sites/default/files/proofpoint-operation-transparent-tribe-threat-insight-en.pdf)[[56]](https://securelist.com/transparent-tribe-part-1/98127/)[[57]](https://blog.talosintelligence.com/2022/07/transparent-tribe-targets-education.html)

|  [S0625](https://attack.mitre.org/software/S0625)   |  [Cuba](https://attack.mitre.org/software/S0625)   |

[Cuba](https://attack.mitre.org/software/S0625) logs keystrokes via polling by using GetKeyState and VkKeyScan functions.[[58]](https://www.mcafee.com/enterprise/en-us/assets/reports/rp-cuba-ransomware.pdf)

|  [C0029](https://attack.mitre.org/campaigns/C0029)   |  [Cutting Edge](https://attack.mitre.org/campaigns/C0029)   |

During [Cutting Edge](https://attack.mitre.org/campaigns/C0029), threat actors modified a JavaScript file on the Web SSL VPN component of Ivanti Connect Secure devices to keylog credentials.[[59]](https://www.volexity.com/blog/2024/01/10/active-exploitation-of-two-zero-day-vulnerabilities-in-ivanti-connect-secure-vpn/)

|  [S0334](https://attack.mitre.org/software/S0334)   |  [DarkComet](https://attack.mitre.org/software/S0334)   |

[DarkComet](https://attack.mitre.org/software/S0334) has a keylogging capability.[[60]](https://www.trendmicro.com/vinfo/us/threat-encyclopedia/malware/DARKCOMET)

|  [S1111](https://attack.mitre.org/software/S1111)   |  [DarkGate](https://attack.mitre.org/software/S1111)   |

[DarkGate](https://attack.mitre.org/software/S1111) will spawn a thread on execution to capture all keyboard events and write them to a predefined log file.[[61]](https://www.fortinet.com/blog/threat-research/enter-the-darkgate-new-cryptocurrency-mining-and-ransomware-campaign)[[62]](https://www.rapid7.com/blog/post/2024/12/04/black-basta-ransomware-campaign-drops-zbot-darkgate-and-custom-malware/)

|  [G0012](https://attack.mitre.org/groups/G0012)   |  [Darkhotel](https://attack.mitre.org/groups/G0012)   |

[Darkhotel](https://attack.mitre.org/groups/G0012) has used a keylogger.[[63]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/08070903/darkhotel_kl_07.11.pdf)

|  [S1066](https://attack.mitre.org/software/S1066)   |  [DarkTortilla](https://attack.mitre.org/software/S1066)   |

[DarkTortilla](https://attack.mitre.org/software/S1066) can download a keylogging module.[[64]](https://www.secureworks.com/research/darktortilla-malware-analysis)

|  [S0673](https://attack.mitre.org/software/S0673)   |  [DarkWatchman](https://attack.mitre.org/software/S0673)   |

[DarkWatchman](https://attack.mitre.org/software/S0673) can track key presses with a keylogger module.[[65]](https://web.archive.org/web/20220629230035/https://www.prevailion.com/darkwatchman-new-fileless-techniques/)

|  [S0187](https://attack.mitre.org/software/S0187)   |  [Daserf](https://attack.mitre.org/software/S0187)   |

[Daserf](https://attack.mitre.org/software/S0187) can log keystrokes.[[66]](http://blog.trendmicro.com/trendlabs-security-intelligence/redbaldknight-bronze-butler-daserf-backdoor-now-using-steganography/)[[67]](https://www.secureworks.com/research/bronze-butler-targets-japanese-businesses)

|  [S9017](https://attack.mitre.org/software/S9017)   |  [DCRAT](https://attack.mitre.org/software/S9017)   |

[DCRAT](https://attack.mitre.org/software/S9017) can log keystrokes on targeted systems.[[68]](https://www.zscaler.com/blogs/security-research/blindeagle-targets-colombian-government-agency-caminho-and-dcrat)

|  [S0021](https://attack.mitre.org/software/S0021)   |  [Derusbi](https://attack.mitre.org/software/S0021)   |

[Derusbi](https://attack.mitre.org/software/S0021) is capable of logging keystrokes.[[69]](https://www.fireeye.com/blog/threat-research/2018/03/suspected-chinese-espionage-group-targeting-maritime-and-engineering-industries.html)

|  [S0213](https://attack.mitre.org/software/S0213)   |  [DOGCALL](https://attack.mitre.org/software/S0213)   |

[DOGCALL](https://attack.mitre.org/software/S0213) is capable of logging keystrokes.[[70]](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)[[71]](https://researchcenter.paloaltonetworks.com/2018/10/unit42-nokki-almost-ties-the-knot-with-dogcall-reaper-group-uses-new-malware-to-deploy-rat/)

|  [S9013](https://attack.mitre.org/software/S9013)   |  [DRYHOOK](https://attack.mitre.org/software/S9013)   |

[DRYHOOK](https://attack.mitre.org/software/S9013) has captured user credentials and passwords in plaintext and has encrypted them in a stored file on the network device.[[72]](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-connect-secure-vpn-zero-day)[[73]](https://www.picussecurity.com/resource/blog/unc5221-cve-2025-22457-ivanti-connect-secure)

|  [S0567](https://attack.mitre.org/software/S0567)   |  [Dtrack](https://attack.mitre.org/software/S0567)   |

[Dtrack](https://attack.mitre.org/software/S0567)’s dropper contains a keylogging executable.[[74]](https://securelist.com/my-name-is-dtrack/93338/)

|  [S0038](https://attack.mitre.org/software/S0038)   |  [Duqu](https://attack.mitre.org/software/S0038)   |

[Duqu](https://attack.mitre.org/software/S0038) can track key presses with a keylogger module.[[75]](https://web.archive.org/web/20111213083345/https://www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/w32_duqu_the_precursor_to_the_next_stuxnet.pdf)

|  [S1159](https://attack.mitre.org/software/S1159)   |  [DUSTTRAP](https://attack.mitre.org/software/S1159)   |

[DUSTTRAP](https://attack.mitre.org/software/S1159) can perform keylogging operations.[[76]](https://cloud.google.com/blog/topics/threat-intelligence/apt41-arisen-from-dust)

|  [S0062](https://attack.mitre.org/software/S0062)   |  [DustySky](https://attack.mitre.org/software/S0062)   |

[DustySky](https://attack.mitre.org/software/S0062) contains a keylogger.[[77]](https://www.clearskysec.com/wp-content/uploads/2016/01/Operation%20DustySky_TLP_WHITE.pdf)

|  [S0593](https://attack.mitre.org/software/S0593)   |  [ECCENTRICBANDWAGON](https://attack.mitre.org/software/S0593)   |

[ECCENTRICBANDWAGON](https://attack.mitre.org/software/S0593) can capture and store keystrokes.[[78]](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-239a)

|  [S0363](https://attack.mitre.org/software/S0363)   |  [Empire](https://attack.mitre.org/software/S0363)   |

[Empire](https://attack.mitre.org/software/S0363) includes keylogging capabilities for Windows, Linux, and macOS systems.[[79]](https://github.com/PowerShellEmpire/Empire)

|  [S0152](https://attack.mitre.org/software/S0152)   |  [EvilGrab](https://attack.mitre.org/software/S0152)   |

[EvilGrab](https://attack.mitre.org/software/S0152) has the capability to capture keystrokes.[[80]](https://www.pwc.co.uk/cyber-security/pdf/pwc-uk-operation-cloud-hopper-technical-annex-april-2017.pdf)

|  [S0569](https://attack.mitre.org/software/S0569)   |  [Explosive](https://attack.mitre.org/software/S0569)   |

[Explosive](https://attack.mitre.org/software/S0569) has leveraged its keylogging capabilities to gain access to administrator accounts on target servers.[[81]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2015/03/20082004/volatile-cedar-technical-report.pdf)[[82]](https://www.clearskysec.com/wp-content/uploads/2021/01/Lebanese-Cedar-APT.pdf)

|  [S0076](https://attack.mitre.org/software/S0076)   |  [FakeM](https://attack.mitre.org/software/S0076)   |

[FakeM](https://attack.mitre.org/software/S0076) contains a keylogger module.[[83]](http://researchcenter.paloaltonetworks.com/2016/01/scarlet-mimic-years-long-espionage-targets-minority-activists/)

|  [G1016](https://attack.mitre.org/groups/G1016)   |  [FIN13](https://attack.mitre.org/groups/G1016)   |

[FIN13](https://attack.mitre.org/groups/G1016) has logged the keystrokes of victims to escalate privileges.[[84]](https://www.mandiant.com/resources/blog/fin13-cybercriminal-mexico)

|  [G0085](https://attack.mitre.org/groups/G0085)   |  [FIN4](https://attack.mitre.org/groups/G0085)   |

[FIN4](https://attack.mitre.org/groups/G0085) has captured credentials via fake Outlook Web App (OWA) login pages and has also used a .NET based keylogger.[[85]](https://web.archive.org/web/20220313062515/https://www.mandiant.com/sites/default/files/2021-09/rpt-fin4.pdf)[[86]](https://www2.fireeye.com/WBNR-14Q4NAMFIN4.html)

|  [S0381](https://attack.mitre.org/software/S0381)   |  [FlawedAmmyy](https://attack.mitre.org/software/S0381)   |

[FlawedAmmyy](https://attack.mitre.org/software/S0381) can collect keyboard events.[[87]](https://www.fsec.or.kr/user/bbs/fsec/163/344/bbsDataView/1382.do?page=1&column=&search=&searchSDate=&searchEDate=&bbsDataCategory=)

|  [S1044](https://attack.mitre.org/software/S1044)   |  [FunnyDream](https://attack.mitre.org/software/S1044)   |

The [FunnyDream](https://attack.mitre.org/software/S1044) Keyrecord component can capture keystrokes.[[88]](https://www.bitdefender.com/files/News/CaseStudies/study/379/Bitdefender-Whitepaper-Chinese-APT.pdf)

|  [S0410](https://attack.mitre.org/software/S0410)   |  [Fysbis](https://attack.mitre.org/software/S0410)   |

[Fysbis](https://attack.mitre.org/software/S0410) can perform keylogging.[[89]](https://researchcenter.paloaltonetworks.com/2016/02/a-look-into-fysbis-sofacys-linux-backdoor/)

|  [S0032](https://attack.mitre.org/software/S0032)   |  [gh0st RAT](https://attack.mitre.org/software/S0032)   |

[gh0st RAT](https://attack.mitre.org/software/S0032) has a keylogger.[[90]](http://blog.trendmicro.com/trendlabs-security-intelligence/kunming-attack-leads-to-gh0st-rat-variant/)[[91]](https://cybersecurity.att.com/blogs/labs-research/the-odd-case-of-a-gh0strat-variant)

|  [S0531](https://attack.mitre.org/software/S0531)   |  [Grandoreiro](https://attack.mitre.org/software/S0531)   |

[Grandoreiro](https://attack.mitre.org/software/S0531) can log keystrokes on the victim's machine.[[92]](https://www.welivesecurity.com/2020/04/28/grandoreiro-how-engorged-can-exe-get/)

|  [S0342](https://attack.mitre.org/software/S0342)   |  [GreyEnergy](https://attack.mitre.org/software/S0342)   |

[GreyEnergy](https://attack.mitre.org/software/S0342) has a module to harvest pressed keystrokes.[[93]](https://www.welivesecurity.com/wp-content/uploads/2018/10/ESET_GreyEnergy.pdf)

|  [G0043](https://attack.mitre.org/groups/G0043)   |  [Group5](https://attack.mitre.org/groups/G0043)   |

Malware used by [Group5](https://attack.mitre.org/groups/G0043) is capable of capturing keystrokes.[[94]](https://citizenlab.ca/2016/08/group5-syria/)

|  [S0170](https://attack.mitre.org/software/S0170)   |  [Helminth](https://attack.mitre.org/software/S0170)   |

The executable version of [Helminth](https://attack.mitre.org/software/S0170) has a module to log keystrokes.[[95]](http://researchcenter.paloaltonetworks.com/2016/05/the-oilrig-campaign-attacks-on-saudi-arabian-organizations-deliver-helminth-backdoor/)

|  [G1001](https://attack.mitre.org/groups/G1001)   |  [HEXANE](https://attack.mitre.org/groups/G1001)   |

[HEXANE](https://attack.mitre.org/groups/G1001) has used a PowerShell-based keylogger named kl.ps1.[[96]](https://www.secureworks.com/blog/lyceum-takes-center-stage-in-middle-east-campaign)[[97]](https://vblocalhost.com/uploads/VB2021-Kayal-etal.pdf)

|  [S1249](https://attack.mitre.org/software/S1249)   |  [HexEval Loader](https://attack.mitre.org/software/S1249)   |

[HexEval Loader](https://attack.mitre.org/software/S1249) has utilized a cross-platform keylogger that has the capability to capture keystrokes on Windows, macOS and Linux systems.[[98]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)

|  [S0070](https://attack.mitre.org/software/S0070)   |  [HTTPBrowser](https://attack.mitre.org/software/S0070)   |

[HTTPBrowser](https://attack.mitre.org/software/S0070) is capable of capturing keystrokes on victims.[[99]](https://www.secureworks.com/research/threat-group-3390-targets-organizations-for-cyberespionage)

|  [S0434](https://attack.mitre.org/software/S0434)   |  [Imminent Monitor](https://attack.mitre.org/software/S0434)   |

[Imminent Monitor](https://attack.mitre.org/software/S0434) has a keylogging module.[[100]](https://unit42.paloaltonetworks.com/imminent-monitor-a-rat-down-under/)

|  [S1245](https://attack.mitre.org/software/S1245)   |  [InvisibleFerret](https://attack.mitre.org/software/S1245)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has conducted keylogging using the Python project "pyWinHook" and "Pyhook".[[101]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[102]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[103]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also captured keylogging thread checks for changes in an active window and key presses.[[104]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  [S0260](https://attack.mitre.org/software/S0260)   |  [InvisiMole](https://attack.mitre.org/software/S0260)   |

[InvisiMole](https://attack.mitre.org/software/S0260) can capture keystrokes on a compromised host.[[105]](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_InvisiMole.pdf)

|  [S0201](https://attack.mitre.org/software/S0201)   |  [JPIN](https://attack.mitre.org/software/S0201)   |

[JPIN](https://attack.mitre.org/software/S0201) contains a custom keylogger.[[106]](https://download.microsoft.com/download/2/2/5/225BFE3E-E1DE-4F5B-A77B-71200928D209/Platinum%20feature%20article%20-%20Targeted%20attacks%20in%20South%20and%20Southeast%20Asia%20April%202016.pdf)

|  [S0283](https://attack.mitre.org/software/S0283)   |  [jRAT](https://attack.mitre.org/software/S0283)   |

[jRAT](https://attack.mitre.org/software/S0283) has the capability to log keystrokes from the victim’s machine, both offline and online.[[107]](https://www.symantec.com/blogs/threat-intelligence/jrat-new-anti-parsing-techniques)[[108]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07195002/KL_AdwindPublicReport_2016.pdf)

|  [S0088](https://attack.mitre.org/software/S0088)   |  [Kasidet](https://attack.mitre.org/software/S0088)   |

[Kasidet](https://attack.mitre.org/software/S0088) has the ability to initiate keylogging.[[109]](http://research.zscaler.com/2016/01/malicious-office-files-dropping-kasidet.html)

|  [G0004](https://attack.mitre.org/groups/G0004)   |  [Ke3chang](https://attack.mitre.org/groups/G0004)   |

[Ke3chang](https://attack.mitre.org/groups/G0004) has used keyloggers.[[110]](https://research.nccgroup.com/2018/03/10/apt15-is-alive-and-strong-an-analysis-of-royalcli-and-royaldns/)[[111]](https://www.microsoft.com/security/blog/2021/12/06/nickel-targeting-government-organizations-across-latin-america-and-europe)

|  [S0387](https://attack.mitre.org/software/S0387)   |  [KeyBoy](https://attack.mitre.org/software/S0387)   |

[KeyBoy](https://attack.mitre.org/software/S0387) installs a keylogger for intercepting credentials and keystrokes.[[112]](https://blog.rapid7.com/2013/06/07/keyboy-targeted-attacks-against-vietnam-and-india/)

|  [S0526](https://attack.mitre.org/software/S0526)   |  [KGH_SPY](https://attack.mitre.org/software/S0526)   |

[KGH_SPY](https://attack.mitre.org/software/S0526) can perform keylogging by polling the GetAsyncKeyState() function.[[113]](https://www.cybereason.com/blog/back-to-the-future-inside-the-kimsuky-kgh-spyware-suite)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has used a PowerShell-based keylogger as well as a tool called MECHANICAL to log keystrokes.[[114]](https://blog.alyac.co.kr/2234)[[115]](https://securelist.com/the-kimsuky-operation-a-north-korean-apt/57915/)[[116]](https://us-cert.cisa.gov/ncas/alerts/aa20-301a)[[117]](https://asert.arbornetworks.com/stolen-pencil-campaign-targets-academia/)[[1]](https://blog.talosintelligence.com/2021/11/kimsuky-abuses-blogs-delivers-malware.html)[[15]](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf) [Kimsuky](https://attack.mitre.org/groups/G0094) has also leveraged Native Windows API functions such as GetAsyncKeyState() along with others to capture keystrokes every 50 milliseconds and stores data in a file stored in the temp directory.[[118]](https://www.aryaka.com/docs/reports/aryaka-kimsuky-apt-operational-blueprint.pdf)

|  [S0437](https://attack.mitre.org/software/S0437)   |  [Kivars](https://attack.mitre.org/software/S0437)   |

[Kivars](https://attack.mitre.org/software/S0437) has the ability to initiate keylogging on the infected host.[[119]](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)

|  [S0356](https://attack.mitre.org/software/S0356)   |  [KONNI](https://attack.mitre.org/software/S0356)   |

[KONNI](https://attack.mitre.org/software/S0356) has the capability to perform keylogging.[[120]](https://blog.talosintelligence.com/2017/05/konni-malware-under-radar-for-years.html)

|  [G0032](https://attack.mitre.org/groups/G0032)   |  [Lazarus Group](https://attack.mitre.org/groups/G0032)   |

[Lazarus Group](https://attack.mitre.org/groups/G0032) malware KiloAlfa contains keylogging functionality.[[121]](https://web.archive.org/web/20160226161828/https://www.operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Report.pdf)[[122]](https://web.archive.org/web/20220425194457/https://operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Tools-Report.pdf)

|  [S9020](https://attack.mitre.org/software/S9020)   |  [LODEINFO](https://attack.mitre.org/software/S9020)   |

[LODEINFO](https://attack.mitre.org/software/S9020) can capture keystrokes on targeted systems.[[123]](https://www.welivesecurity.com/2022/12/14/unmasking-mirrorface-operation-liberalface-targeting-japanese-political-entities/)[[124]](https://blog-en.itochuci.co.jp/entry/2024/01/24/134100)[[125]](https://www.trendmicro.com/en_us/research/24/k/lodeinfo-campaign-of-earth-kasha.html)

|  [S0447](https://attack.mitre.org/software/S0447)   |  [Lokibot](https://attack.mitre.org/software/S0447)   |

[Lokibot](https://attack.mitre.org/software/S0447) has the ability to capture input on the compromised host via keylogging.[[126]](https://www.f-secure.com/v-descs/trojan_w32_lokibot.shtml)

|  [S0409](https://attack.mitre.org/software/S0409)   |  [Machete](https://attack.mitre.org/software/S0409)   |

[Machete](https://attack.mitre.org/software/S0409) logs keystrokes from the victim’s machine.[[127]](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)[[128]](https://securelist.com/el-machete/66108/)[[129]](https://threatvector.cylance.com/en_us/home/el-machete-malware-attacks-cut-through-latam.html)[[130]](https://blog.360totalsecurity.com/en/apt-c-43-steals-venezuelan-military-secrets-to-provide-intelligence-support-for-the-reactionaries-hpreact-campaign/)

|  [S1016](https://attack.mitre.org/software/S1016)   |  [MacMa](https://attack.mitre.org/software/S1016)   |

[MacMa](https://attack.mitre.org/software/S1016) can use Core Graphics Event Taps to intercept user keystrokes from any text input field and saves them to text files. Text input fields include Spotlight, Finder, Safari, Mail, Messages, and other apps that have text fields for passwords.[[131]](https://objective-see.org/blog/blog_0x69.html)[[132]](https://www.sentinelone.com/labs/infect-if-needed-a-deeper-dive-into-targeted-backdoor-macos-macma/)

|  [S0282](https://attack.mitre.org/software/S0282)   |  [MacSpy](https://attack.mitre.org/software/S0282)   |

[MacSpy](https://attack.mitre.org/software/S0282) captures keystrokes.[[133]](https://objective-see.com/blog/blog_0x25.html)

|  [G0059](https://attack.mitre.org/groups/G0059)   |  [Magic Hound](https://attack.mitre.org/groups/G0059)   |

[Magic Hound](https://attack.mitre.org/groups/G0059) malware is capable of keylogging.[[134]](https://researchcenter.paloaltonetworks.com/2017/02/unit42-magic-hound-campaign-attacks-saudi-targets/)

|  [S0652](https://attack.mitre.org/software/S0652)   |  [MarkiRAT](https://attack.mitre.org/software/S0652)   |

[MarkiRAT](https://attack.mitre.org/software/S0652) can capture all keystrokes on a compromised host.[[135]](https://securelist.com/ferocious-kitten-6-years-of-covert-surveillance-in-iran/102806/)

|  [S0167](https://attack.mitre.org/software/S0167)   |  [Matryoshka](https://attack.mitre.org/software/S0167)   |

[Matryoshka](https://attack.mitre.org/software/S0167) is capable of keylogging.[[136]](http://www.clearskysec.com/wp-content/uploads/2017/07/Operation_Wilted_Tulip.pdf)[[137]](https://cdn2.hubspot.net/hubfs/1903456/Whitepapers/CopyKittens.pdf)

|  [G0045](https://attack.mitre.org/groups/G0045)   |  [menuPass](https://attack.mitre.org/groups/G0045)   |

[menuPass](https://attack.mitre.org/groups/G0045) has used key loggers to steal usernames and passwords.[[138]](https://www.justice.gov/opa/page/file/1122671/download)

|  [S1059](https://attack.mitre.org/software/S1059)   |  [metaMain](https://attack.mitre.org/software/S1059)   |

[metaMain](https://attack.mitre.org/software/S1059) has the ability to log keyboard events.[[139]](https://assets.sentinelone.com/sentinellabs22/metador#page=1)[[140]](https://docs.google.com/document/d/1e9ZTW9b71YwFWS_18ZwDAxa-cYbV8q1wUefmKZLYVsA/edit#heading=h.lmnbtht1ikzm)

|  [S0455](https://attack.mitre.org/software/S0455)   |  [Metamorfo](https://attack.mitre.org/software/S0455)   |

[Metamorfo](https://attack.mitre.org/software/S0455) has a command to launch a keylogger and capture keystrokes on the victim’s machine.[[141]](https://www.fortinet.com/blog/threat-research/another-metamorfo-variant-targeting-customers-of-financial-institutions)[[142]](https://www.welivesecurity.com/2019/10/03/casbaneiro-trojan-dangerous-cooking/)

|  [S1146](https://attack.mitre.org/software/S1146)   |  [MgBot](https://attack.mitre.org/software/S1146)   |

[MgBot](https://attack.mitre.org/software/S1146) includes keylogger payloads focused on the QQ chat application.[[143]](https://www.welivesecurity.com/2023/04/26/evasive-panda-apt-group-malware-updates-popular-chinese-software/)[[144]](https://symantec-enterprise-blogs.security.com/threat-intelligence/apt-attacks-telecoms-africa-mgbot)

|  [S0339](https://attack.mitre.org/software/S0339)   |  [Micropsia](https://attack.mitre.org/software/S0339)   |

[Micropsia](https://attack.mitre.org/software/S0339) has keylogging capabilities.[[145]](https://www.radware.com/blog/security/2018/07/micropsia-malware/)

|  [S1122](https://attack.mitre.org/software/S1122)   |  [Mispadu](https://attack.mitre.org/software/S1122)   |

[Mispadu](https://attack.mitre.org/software/S1122) can log keystrokes on the victim's machine.[[146]](https://www.welivesecurity.com/2019/11/19/mispadu-advertisement-discounted-unhappy-meal/)[[147]](https://www.metabaseq.com/mispadu-banking-trojan/)[[148]](https://blog.scilabs.mx/en/evolution-of-banking-trojan-ursa-mispadu/)

|  [S0149](https://attack.mitre.org/software/S0149)   |  [MoonWind](https://attack.mitre.org/software/S0149)   |

[MoonWind](https://attack.mitre.org/software/S0149) has a keylogger.[[149]](http://researchcenter.paloaltonetworks.com/2017/03/unit42-trochilus-rat-new-moonwind-rat-used-attack-thai-utility-organizations/)

|  [S0336](https://attack.mitre.org/software/S0336)   |  [NanoCore](https://attack.mitre.org/software/S0336)   |

[NanoCore](https://attack.mitre.org/software/S0336) can perform keylogging on the victim’s machine.[[150]](https://researchcenter.paloaltonetworks.com/2016/02/nanocorerat-behind-an-increase-in-tax-themed-phishing-e-mails/)

|  [S0247](https://attack.mitre.org/software/S0247)   |  [NavRAT](https://attack.mitre.org/software/S0247)   |

[NavRAT](https://attack.mitre.org/software/S0247) logs the keystrokes on the targeted system.[[151]](https://blog.talosintelligence.com/2018/05/navrat.html)

|  [S0033](https://attack.mitre.org/software/S0033)   |  [NetTraveler](https://attack.mitre.org/software/S0033)   |

[NetTraveler](https://attack.mitre.org/software/S0033) contains a keylogger.[[152]](https://web.archive.org/web/20160326004042/http://kasperskycontenthub.com/wp-content/uploads/sites/43/vlpdfs/kaspersky-the-net-traveler-part1-final.pdf)

|  [S0198](https://attack.mitre.org/software/S0198)   |  [NETWIRE](https://attack.mitre.org/software/S0198)   |

[NETWIRE](https://attack.mitre.org/software/S0198) can perform keylogging.[[153]](https://securingtomorrow.mcafee.com/mcafee-labs/netwire-rat-behind-recent-targeted-attacks/)[[154]](https://www.brighttalk.com/webcast/10703/275683)[[155]](https://www.mandiant.com/resources/blog/dissecting-netwire-phishing-campaigns-usage-process-hollowing)[[156]](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)[[157]](https://www.proofpoint.com/us/blog/threat-insight/geofenced-netwire-campaigns)

|  [S1090](https://attack.mitre.org/software/S1090)   |  [NightClub](https://attack.mitre.org/software/S1090)   |

[NightClub](https://attack.mitre.org/software/S1090) can use a plugin for keylogging.[[158]](https://www.welivesecurity.com/en/eset-research/moustachedbouncer-espionage-against-foreign-diplomats-in-belarus/)

|  [S0385](https://attack.mitre.org/software/S0385)   |  [njRAT](https://attack.mitre.org/software/S0385)   |

[njRAT](https://attack.mitre.org/software/S0385) is capable of logging keystrokes.[[159]](https://www.threatminer.org/_reports/2013/fta-1009---njrat-uncovered-1.pdf)[[160]](https://blog.trendmicro.com/trendlabs-security-intelligence/autoit-compiled-worm-affecting-removable-media-delivers-fileless-version-of-bladabindi-njrat-backdoor/)[[94]](https://citizenlab.ca/2016/08/group5-syria/)[[161]](https://securelist.com/blindeagle-apt/113414/)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) has employed keyloggers including KEYPUNCH and LONGWATCH.[[162]](https://www.brighttalk.com/webcast/10703/296317/apt34-new-targeted-attack-in-the-middle-east)[[163]](https://www.fireeye.com/blog/threat-research/2019/07/hard-pass-declining-apt34-invite-to-join-their-professional-network.html)[[164]](https://www.security.com/threat-intelligence/crambus-middle-east-government)

|  [S0439](https://attack.mitre.org/software/S0439)   |  [Okrum](https://attack.mitre.org/software/S0439)   |

[Okrum](https://attack.mitre.org/software/S0439) was seen using a keylogger tool to capture keystrokes. [[165]](https://www.welivesecurity.com/wp-content/uploads/2019/07/ESET_Okrum_and_Ketrican.pdf)

|  [C0014](https://attack.mitre.org/campaigns/C0014)   |  [Operation Wocao](https://attack.mitre.org/campaigns/C0014)   |

During [Operation Wocao](https://attack.mitre.org/campaigns/C0014), threat actors obtained the password for the victim's password manager via a custom keylogger.[[166]](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)

|  [S0072](https://attack.mitre.org/software/S0072)   |  [OwaAuth](https://attack.mitre.org/software/S0072)   |

[OwaAuth](https://attack.mitre.org/software/S0072) captures and DES-encrypts credentials before writing the username and password to a log file, C:\log.txt.[[99]](https://www.secureworks.com/research/threat-group-3390-targets-organizations-for-cyberespionage)

|  [S1233](https://attack.mitre.org/software/S1233)   |  [PAKLOG](https://attack.mitre.org/software/S1233)   |

[PAKLOG](https://attack.mitre.org/software/S1233) has captured keystrokes using Windows API.[[53]](https://www.zscaler.com/blogs/security-research/latest-mustang-panda-arsenal-paklog-corklog-and-splatcloak-p2)

|  [S1050](https://attack.mitre.org/software/S1050)   |  [PcShare](https://attack.mitre.org/software/S1050)   |

[PcShare](https://attack.mitre.org/software/S1050) has the ability to capture keystrokes.[[88]](https://www.bitdefender.com/files/News/CaseStudies/study/379/Bitdefender-Whitepaper-Chinese-APT.pdf)

|  [S0643](https://attack.mitre.org/software/S0643)   |  [Peppy](https://attack.mitre.org/software/S0643)   |

[Peppy](https://attack.mitre.org/software/S0643) can log keystrokes on compromised hosts.[[55]](https://www.proofpoint.com/sites/default/files/proofpoint-operation-transparent-tribe-threat-insight-en.pdf)

|  [G0068](https://attack.mitre.org/groups/G0068)   |  [PLATINUM](https://attack.mitre.org/groups/G0068)   |

[PLATINUM](https://attack.mitre.org/groups/G0068) has used several different keyloggers.[[106]](https://download.microsoft.com/download/2/2/5/225BFE3E-E1DE-4F5B-A77B-71200928D209/Platinum%20feature%20article%20-%20Targeted%20attacks%20in%20South%20and%20Southeast%20Asia%20April%202016.pdf)

|  [S0013](https://attack.mitre.org/software/S0013)   |  [PlugX](https://attack.mitre.org/software/S0013)   |

[PlugX](https://attack.mitre.org/software/S0013) has a module for capturing keystrokes per process including window titles.[[167]](http://circl.lu/assets/files/tr-12/tr-12-circl-plugx-analysis-v1.pdf)

|  [S0428](https://attack.mitre.org/software/S0428)   |  [PoetRAT](https://attack.mitre.org/software/S0428)   |

[PoetRAT](https://attack.mitre.org/software/S0428) has used a Python tool named klog.exe for keylogging.[[168]](https://blog.talosintelligence.com/2020/04/poetrat-covid-19-lures.html)

|  [S0012](https://attack.mitre.org/software/S0012)   |  [PoisonIvy](https://attack.mitre.org/software/S0012)   |

[PoisonIvy](https://attack.mitre.org/software/S0012) contains a keylogger.[[169]](https://www.mandiant.com/sites/default/files/2021-09/rpt-poison-ivy.pdf)[[170]](https://www.symantec.com/security_response/writeup.jsp?docid=2005-081910-3934-99)

|  [S0378](https://attack.mitre.org/software/S0378)   |  [PoshC2](https://attack.mitre.org/software/S0378)   |

[PoshC2](https://attack.mitre.org/software/S0378) has modules for keystroke logging and capturing credentials from spoofed Outlook authentication messages.[[171]](https://github.com/nettitude/PoshC2_Python)

|  [S1012](https://attack.mitre.org/software/S1012)   |  [PowerLess](https://attack.mitre.org/software/S1012)   |

[PowerLess](https://attack.mitre.org/software/S1012) can use a module to log keystrokes.[[172]](https://www.cybereason.com/blog/research/powerless-trojan-iranian-apt-phosphorus-adds-new-powershell-backdoor-for-espionage)

|  [S0194](https://attack.mitre.org/software/S0194)   |  [PowerSploit](https://attack.mitre.org/software/S0194)   |

[PowerSploit](https://attack.mitre.org/software/S0194)'s Get-Keystrokes Exfiltration module can log keystrokes.[[173]](https://github.com/PowerShellMafia/PowerSploit)[[174]](http://powersploit.readthedocs.io)

|  [S0113](https://attack.mitre.org/software/S0113)   |  [Prikormka](https://attack.mitre.org/software/S0113)   |

[Prikormka](https://attack.mitre.org/software/S0113) contains a keylogger module that collects keystrokes and the titles of foreground windows.[[175]](http://www.welivesecurity.com/wp-content/uploads/2016/05/Operation-Groundbait.pdf)

|  [S0279](https://attack.mitre.org/software/S0279)   |  [Proton](https://attack.mitre.org/software/S0279)   |

[Proton](https://attack.mitre.org/software/S0279) uses a keylogger to capture keystrokes.[[133]](https://objective-see.com/blog/blog_0x25.html)

|  [S0192](https://attack.mitre.org/software/S0192)   |  [Pupy](https://attack.mitre.org/software/S0192)   |

[Pupy](https://attack.mitre.org/software/S0192) uses a keylogger to capture keystrokes it then sends back to the server after it is stopped.[[176]](https://github.com/n1nj4sec/pupy)

|  [S0650](https://attack.mitre.org/software/S0650)   |  [QakBot](https://attack.mitre.org/software/S0650)   |

[QakBot](https://attack.mitre.org/software/S0650) can capture keystrokes on a compromised host.[[177]](https://www.kroll.com/en/insights/publications/cyber/qakbot-malware-exfiltrating-emails-thread-hijacking-attacks)[[178]](https://success.trendmicro.com/en-US/solution/KA-0011282)[[179]](https://securelist.com/qakbot-technical-analysis/103931/)

|  [S0262](https://attack.mitre.org/software/S0262)   |  [QuasarRAT](https://attack.mitre.org/software/S0262)   |

[QuasarRAT](https://attack.mitre.org/software/S0262) has a built-in keylogger.[[180]](https://github.com/quasar/QuasarRAT)[[181]](https://www.volexity.com/blog/2018/06/07/patchwork-apt-group-targets-us-think-tanks/)[[161]](https://securelist.com/blindeagle-apt/113414/)

|  [S0662](https://attack.mitre.org/software/S0662)   |  [RCSession](https://attack.mitre.org/software/S0662)   |

[RCSession](https://attack.mitre.org/software/S0662) has the ability to capture keystrokes on a compromised host.[[47]](https://documents.trendmicro.com/assets/white_papers/wp-uncovering-DRBcontrol.pdf)[[182]](https://web.archive.org/web/20210104144857/https://shared-public-reports.s3-eu-west-1.amazonaws.com/APT27+turns+to+ransomware.pdf)

|  [S0019](https://attack.mitre.org/software/S0019)   |  [Regin](https://attack.mitre.org/software/S0019)   |

[Regin](https://attack.mitre.org/software/S0019) contains a keylogger.[[183]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/08070305/Kaspersky_Lab_whitepaper_Regin_platform_eng.pdf)

|  [S0332](https://attack.mitre.org/software/S0332)   |  [Remcos](https://attack.mitre.org/software/S0332)   |

[Remcos](https://attack.mitre.org/software/S0332) has a command for keylogging.[[184]](https://www.fortinet.com/blog/threat-research/remcos-a-new-rat-in-the-wild-2.html)[[185]](https://blog.talosintelligence.com/2018/08/picking-apart-remcos.html)

|  [S0375](https://attack.mitre.org/software/S0375)   |  [Remexi](https://attack.mitre.org/software/S0375)   |

[Remexi](https://attack.mitre.org/software/S0375) gathers and exfiltrates keystrokes from the machine.[[186]](https://securelist.com/chafer-used-remexi-malware/89538/)

|  [S0125](https://attack.mitre.org/software/S0125)   |  [Remsec](https://attack.mitre.org/software/S0125)   |

[Remsec](https://attack.mitre.org/software/S0125) contains a keylogger component.[[187]](http://www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/Symantec_Remsec_IOCs.pdf)[[188]](https://securelist.com/files/2016/07/The-ProjectSauron-APT_Technical_Analysis_KL.pdf)

|  [S0379](https://attack.mitre.org/software/S0379)   |  [Revenge RAT](https://attack.mitre.org/software/S0379)   |

[Revenge RAT](https://attack.mitre.org/software/S0379) has a plugin for keylogging.[[189]](https://www.cylance.com/content/dam/cylance-web/en-us/resources/knowledge-center/resource-library/reports/WhiteCompanyOperationShaheenReport.pdf?_ga=2.161661948.1943296560.1555683782-1066572390.1555511517)[[190]](https://web.archive.org/web/20200428173819/https://cofense.com/upgrades-delivery-support-infrastructure-revenge-rat-malware-bigger-threat/)

|  [S0240](https://attack.mitre.org/software/S0240)   |  [ROKRAT](https://attack.mitre.org/software/S0240)   |

[ROKRAT](https://attack.mitre.org/software/S0240) can use SetWindowsHookEx and GetKeyNameText to capture keystrokes.[[191]](https://blog.talosintelligence.com/2017/04/introducing-rokrat.html)[[192]](https://www.volexity.com/blog/2021/08/24/north-korean-bluelight-special-inkysquid-deploys-rokrat/)

|  [S0090](https://attack.mitre.org/software/S0090)   |  [Rover](https://attack.mitre.org/software/S0090)   |

[Rover](https://attack.mitre.org/software/S0090) has keylogging functionality.[[193]](http://researchcenter.paloaltonetworks.com/2016/02/new-malware-rover-targets-indian-ambassador-to-afghanistan/)

|  [S0148](https://attack.mitre.org/software/S0148)   |  [RTM](https://attack.mitre.org/software/S0148)   |

[RTM](https://attack.mitre.org/software/S0148) can record keystrokes from both the keyboard and virtual keyboard.[[194]](https://www.welivesecurity.com/wp-content/uploads/2017/02/Read-The-Manual.pdf)[[195]](https://unit42.paloaltonetworks.com/russian-language-malspam-pushing-redaman-banking-malware/)

|  [S0253](https://attack.mitre.org/software/S0253)   |  [RunningRAT](https://attack.mitre.org/software/S0253)   |

[RunningRAT](https://attack.mitre.org/software/S0253) captures keystrokes and sends them back to the C2 server.[[196]](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/gold-dragon-widens-olympics-malware-attacks-gains-permanent-presence-on-victims-systems/)

|  [G0034](https://attack.mitre.org/groups/G0034)   |  [Sandworm Team](https://attack.mitre.org/groups/G0034)   |

[Sandworm Team](https://attack.mitre.org/groups/G0034) has used a keylogger to capture keystrokes by using the SetWindowsHookEx function.[[197]](https://www.welivesecurity.com/2016/12/13/rise-telebots-analyzing-disruptive-killdisk-attacks/)

|  [S0692](https://attack.mitre.org/software/S0692)   |  [SILENTTRINITY](https://attack.mitre.org/software/S0692)   |

[SILENTTRINITY](https://attack.mitre.org/software/S0692) has a keylogging capability.[[198]](https://github.com/byt3bl33d3r/SILENTTRINITY/tree/master/silenttrinity/core/teamserver/modules/boo)

|  [S0533](https://attack.mitre.org/software/S0533)   |  [SLOTHFULMEDIA](https://attack.mitre.org/software/S0533)   |

[SLOTHFULMEDIA](https://attack.mitre.org/software/S0533) has a keylogging capability.[[199]](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-275a)

|  [S0649](https://attack.mitre.org/software/S0649)   |  [SMOKEDHAM](https://attack.mitre.org/software/S0649)   |

[SMOKEDHAM](https://attack.mitre.org/software/S0649) can continuously capture keystrokes.[[200]](https://www.fireeye.com/blog/threat-research/2021/05/shining-a-light-on-darkside-ransomware-operations.html)[[201]](https://www.fireeye.com/blog/threat-research/2021/06/darkside-affiliate-supply-chain-software-compromise.html)

|  [G0054](https://attack.mitre.org/groups/G0054)   |  [Sowbug](https://attack.mitre.org/groups/G0054)   |

[Sowbug](https://attack.mitre.org/groups/G0054) has used keylogging tools.[[202]](https://www.symantec.com/connect/blogs/sowbug-cyber-espionage-group-targets-south-american-and-southeast-asian-governments)

|  [S0058](https://attack.mitre.org/software/S0058)   |  [SslMM](https://attack.mitre.org/software/S0058)   |

[SslMM](https://attack.mitre.org/software/S0058) creates a new thread implementing a keylogging facility using Windows Keyboard Accelerators.[[203]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07205555/TheNaikonAPT-MsnMM1.pdf)

|  [S0018](https://attack.mitre.org/software/S0018)   |  [Sykipot](https://attack.mitre.org/software/S0018)   |

[Sykipot](https://attack.mitre.org/software/S0018) contains keylogging functionality to steal passwords.[[204]](https://www.alienvault.com/open-threat-exchange/blog/sykipot-variant-hijacks-dod-and-windows-smart-cards)

|  [S0467](https://attack.mitre.org/software/S0467)   |  [TajMahal](https://attack.mitre.org/software/S0467)   |

[TajMahal](https://attack.mitre.org/software/S0467) has the ability to capture keystrokes on an infected host.[[205]](https://securelist.com/project-tajmahal/90240/)

|  [S0595](https://attack.mitre.org/software/S0595)   |  [ThiefQuest](https://attack.mitre.org/software/S0595)   |

[ThiefQuest](https://attack.mitre.org/software/S0595) uses the CGEventTap functions to perform keylogging.[[206]](https://www.trendmicro.com/en_us/research/20/g/updates-on-quickly-evolving-thiefquest-macos-malware.html)

|  [G0027](https://attack.mitre.org/groups/G0027)   |  [Threat Group-3390](https://attack.mitre.org/groups/G0027)   |

[Threat Group-3390](https://attack.mitre.org/groups/G0027) actors installed a credential logger on Microsoft Exchange servers. [Threat Group-3390](https://attack.mitre.org/groups/G0027) also leveraged the reconnaissance framework, ScanBox, to capture keystrokes.[[99]](https://www.secureworks.com/research/threat-group-3390-targets-organizations-for-cyberespionage)[[207]](https://thehackernews.com/2018/06/chinese-watering-hole-attack.html)[[208]](https://securelist.com/luckymouse-hits-national-data-center/86083/)

|  [S0004](https://attack.mitre.org/software/S0004)   |  [TinyZBot](https://attack.mitre.org/software/S0004)   |

[TinyZBot](https://attack.mitre.org/software/S0004) contains keylogger functionality.[[209]](https://web.archive.org/web/20200302085133/https://www.cylance.com/content/dam/cylance/pages/operation-cleaver/Cylance_Operation_Cleaver_Report.pdf)

|  [S1239](https://attack.mitre.org/software/S1239)   |  [TONESHELL](https://attack.mitre.org/software/S1239)   |

[TONESHELL](https://attack.mitre.org/software/S1239) has capabilities to conduct keylogging.[[210]](https://unit42.paloaltonetworks.com/stately-taurus-attacks-se-asian-government/)

|  [G0131](https://attack.mitre.org/groups/G0131)   |  [Tonto Team](https://attack.mitre.org/groups/G0131)   |

[Tonto Team](https://attack.mitre.org/groups/G0131) has used keylogging tools in their operations.[[211]](https://vb2020.vblocalhost.com/uploads/VB2020-06.pdf)

|  [S0094](https://attack.mitre.org/software/S0094)   |  [Trojan.Karagany](https://attack.mitre.org/software/S0094)   |

[Trojan.Karagany](https://attack.mitre.org/software/S0094) can capture keystrokes on a compromised host.[[212]](https://www.secureworks.com/research/updated-karagany-malware-targets-energy-sector)

|  [S0130](https://attack.mitre.org/software/S0130)   |  [Unknown Logger](https://attack.mitre.org/software/S0130)   |

[Unknown Logger](https://attack.mitre.org/software/S0130) is capable of recording keystrokes.[[32]](https://www.forcepoint.com/sites/default/files/resources/files/forcepoint-security-labs-monsoon-analysis-report.pdf)

|  [S0257](https://attack.mitre.org/software/S0257)   |  [VERMIN](https://attack.mitre.org/software/S0257)   |

[VERMIN](https://attack.mitre.org/software/S0257) collects keystrokes from the victim machine.[[213]](https://researchcenter.paloaltonetworks.com/2018/01/unit42-vermin-quasar-rat-custom-malware-used-ukraine/)

|  [G1017](https://attack.mitre.org/groups/G1017)   |  [Volt Typhoon](https://attack.mitre.org/groups/G1017)   |

[Volt Typhoon](https://attack.mitre.org/groups/G1017) has created and accessed a file named rult3uil.log on compromised domain controllers to capture keypresses and command execution.[[214]](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)

|  [S0670](https://attack.mitre.org/software/S0670)   |  [WarzoneRAT](https://attack.mitre.org/software/S0670)   |

[WarzoneRAT](https://attack.mitre.org/software/S0670) has the capability to install a live and offline keylogger, including through the use of the GetAsyncKeyState Windows API.[[215]](https://research.checkpoint.com/2020/warzone-behind-the-enemy-lines/)[[216]](https://www.uptycs.com/blog/warzone-rat-comes-with-uac-bypass-technique)

|  [S0161](https://attack.mitre.org/software/S0161)   |  [XAgentOSX](https://attack.mitre.org/software/S0161)   |

[XAgentOSX](https://attack.mitre.org/software/S0161) contains keylogging functionality that will monitor for active application windows and write them to the log, it can handle special characters, and it will buffer by default 50 characters before sending them out over the C2 infrastructure.[[217]](https://researchcenter.paloaltonetworks.com/2017/02/unit42-xagentosx-sofacys-xagent-macos-tool/)

|  [S1207](https://attack.mitre.org/software/S1207)   |  [XLoader](https://attack.mitre.org/software/S1207)   |

[XLoader](https://attack.mitre.org/software/S1207) can capture keystrokes from the victim machine.[[218]](https://cloud.google.com/blog/topics/threat-intelligence/formbook-malware-distribution-campaigns/)

|  [S0248](https://attack.mitre.org/software/S0248)   |  [yty](https://attack.mitre.org/software/S0248)   |

[yty](https://attack.mitre.org/software/S0248) uses a keylogger plugin to gather keystrokes.[[219]](https://www.arbornetworks.com/blog/asert/donot-team-leverages-new-modular-malware-framework-south-asia/)

|  [S0330](https://attack.mitre.org/software/S0330)   |  [Zeus Panda](https://attack.mitre.org/software/S0330)   |

[Zeus Panda](https://attack.mitre.org/software/S0330) can perform keylogging on the victim’s machine by hooking the functions TranslateMessage and WM_KEYDOWN.[[220]](https://cyberwtf.files.wordpress.com/2017/07/panda-whitepaper.pdf)

|  [S0412](https://attack.mitre.org/software/S0412)   |  [ZxShell](https://attack.mitre.org/software/S0412)   |

[ZxShell](https://attack.mitre.org/software/S0412) has a feature to capture a remote computer's keystrokes using a keylogger.[[24]](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)[[221]](https://blogs.cisco.com/security/talos/opening-zxshell)

## Mitigations

This type of attack technique cannot be easily mitigated with preventive controls since it is based on the abuse of system features.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0089](https://attack.mitre.org/detectionstrategies/DET0089)   |  [Behavioral Detection of Keylogging Activity Across Platforms](https://attack.mitre.org/detectionstrategies/DET0089)   |  [AN0243](https://attack.mitre.org/detectionstrategies/DET0089#AN0243)   |

Monitors suspicious usage of Windows API calls like SetWindowsHookEx, GetKeyState, or polling functions within non-UI service processes, combined with Registry or driver modifications.

|  [AN0244](https://attack.mitre.org/detectionstrategies/DET0089#AN0244)   |

Detects non-system processes accessing /dev/input/* or issuing ptrace/evdev syscalls used for reading keystroke buffers directly.

|  [AN0245](https://attack.mitre.org/detectionstrategies/DET0089#AN0245)   |

Detects unauthorized TCC access or use of Quartz Event Services (CGEventTapCreate) or IOHID for event tap installation within unexpected processes.

|  [AN0246](https://attack.mitre.org/detectionstrategies/DET0089#AN0246)   |

Keylogging on legacy network devices via unauthorized system image modification or remote capture of console keystrokes (telnet, SSH) through altered firmware or man-in-the-middle key sniffing.

## References

-    [An, J and Malhotra, A. (2021, November 10). North Korean attackers use malicious blogs to deliver malware to high-profile South Korean targets. Retrieved December 29, 2021.](https://blog.talosintelligence.com/2021/11/kimsuky-abuses-blogs-delivers-malware.html)
-    [Tinaztepe, E. (n.d.). The Adventures of a Keystroke: An in-depth look into keyloggers on Windows. Retrieved April 27, 2016.](http://opensecuritytraining.info/Keylogging_files/The%20Adventures%20of%20a%20Keystroke.pdf)
-    [Omar Santos. (2020, October 19). Attackers Continue to Target Legacy Devices. Retrieved October 20, 2020.](https://community.cisco.com/t5/security-blogs/attackers-continue-to-target-legacy-devices/ba-p/4169954)
-    [Booz Allen Hamilton. (2016). When The Lights Went Out. Retrieved December 18, 2024.](https://www.boozallen.com/content/dam/boozallen/documents/2016/09/ukraine-report-when-the-lights-went-out.pdf)
-    [Electricity Information Sharing and Analysis Center; SANS Industrial Control Systems. (2016, March 18). Analysis of the Cyber Attack on the Ukranian Power Grid: Defense Use Case. Retrieved March 27, 2018.](https://nsarchive.gwu.edu/sites/default/files/documents/3891751/SANS-and-Electricity-Information-Sharing-and.pdf)
-    [ESET. (2016, October). En Route with Sednit - Part 2: Observing the Comings and Goings. Retrieved November 21, 2016.](http://www.welivesecurity.com/wp-content/uploads/2016/10/eset-sednit-part-2.pdf)
-    [Bitdefender. (2015, December). APT28 Under the Scope. Retrieved February 23, 2017.](https://download.bitdefender.com/resources/media/materials/white-papers/en/Bitdefender_In-depth_analysis_of_APT28%E2%80%93The_Political_Cyber-Espionage.pdf)
-    [Brumaghin, E., et al. (2018, October 15). Old dog, new tricks - Analysing new RTF-based campaign distributing Agent Tesla, Loki with PyREbox. Retrieved November 5, 2018.](https://blog.talosintelligence.com/2018/10/old-dog-new-tricks-analysing-new-rtf_15.html)
-    [The DigiTrust Group. (2017, January 12). The Rise of Agent Tesla. Retrieved November 5, 2018.](https://www.digitrustgroup.com/agent-tesla-keylogger/)
-    [Zhang, X. (2017, June 28). In-Depth Analysis of A New Variant of .NET Malware AgentTesla. Retrieved November 5, 2018.](https://www.fortinet.com/blog/threat-research/in-depth-analysis-of-net-malware-javaupdtr.html)
-    [Arsene, L. (2020, April 21). Oil & Gas Spearphishing Campaigns Drop Agent Tesla Spyware in Advance of Historic OPEC+ Deal. Retrieved May 19, 2020.](https://labs.bitdefender.com/2020/04/oil-gas-spearphishing-campaigns-drop-agent-tesla-spyware-in-advance-of-historic-opec-deal/)
-    [Walter, J. (2020, August 10). Agent Tesla | Old RAT Uses New Tricks to Stay on Top. Retrieved December 11, 2020.](https://labs.sentinelone.com/agent-tesla-old-rat-uses-new-tricks-to-stay-on-top/)
-    [Check Point Software Technologies. (2015). ROCKET KITTEN: A CAMPAIGN WITH 9 LIVES. Retrieved March 16, 2018.](https://blog.checkpoint.com/wp-content/uploads/2015/11/rocket-kitten-report.pdf)
-    [Jazi, H. (2021, June 1). Kimsuky APT continues to target South Korean government using AppleSeed backdoor. Retrieved June 10, 2021.](https://blog.malwarebytes.com/threat-analysis/2021/06/kimsuky-apt-continues-to-target-south-korean-government-using-appleseed-backdoor/)
-    [KISA. (2021). Phishing Target Reconnaissance and Attack Resource Analysis Operation Muzabi. Retrieved March 8, 2024.](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)
-    [Anthe, C. et al. (2015, October 19). Microsoft Security Intelligence Report Volume 19. Retrieved December 23, 2015.](http://download.microsoft.com/download/4/4/C/44CDEF0E-7924-4787-A56A-16261691ACE3/Microsoft_Security_Intelligence_Report_Volume_19_English.pdf)
-    [Mueller, R. (2018, July 13). Indictment - United States of America vs. VIKTOR BORISOVICH NETYKSHO, et al. Retrieved November 17, 2024.](https://cdn.cnn.com/cnn/2018/images/07/13/gru.indictment.pdf)
-    [Hacquebord, F., Remorin, L. (2020, December 17). Pawn Storm’s Lack of Sophistication as a Strategy. Retrieved January 13, 2021.](https://www.trendmicro.com/en_us/research/20/l/pawn-storm-lack-of-sophistication-as-a-strategy.html)
-    [Symantec Security Response. (2016, September 6). Buckeye cyberespionage group shifts gaze from US to Hong Kong. Retrieved September 26, 2016.](https://web.archive.org/web/20160910124439/http://www.symantec.com/connect/blogs/buckeye-cyberespionage-group-shifts-gaze-us-hong-kong)
-    [Dahan, A. (2017). Operation Cobalt Kitty. Retrieved December 27, 2018.](https://cdn2.hubspot.net/hubfs/3354902/Cybereason%20Labs%20Analysis%20Operation%20Cobalt%20Kitty.pdf)
-    [FireEye. (2018, October 03). APT38: Un-usual Suspects. Retrieved November 17, 2024.](https://services.google.com/fh/files/misc/apt38-un-usual-suspects.pdf)
-    [Symantec. (2018, February 28). Chafer: Latest Attacks Reveal Heightened Ambitions. Retrieved May 22, 2020.](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/chafer-latest-attacks-reveal-heightened-ambitions)
-    [FBI. (2020, September 17). Indicators of Compromise Associated with Rana Intelligence Computing, also known as Advanced Persistent Threat 39, Chafer, Cadelspy, Remexi, and ITG07. Retrieved December 10, 2020.](https://www.iranwatch.org/sites/default/files/public-intelligence-alert.pdf)
-    [Fraser, N., et al. (2019, August 7). Double DragonAPT41, a dual espionage and cyber crime operation APT41. Retrieved September 23, 2019.](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)
-    [Mandiant. (n.d.). APT42: Crooked Charms, Cons and Compromises. Retrieved October 9, 2024.](https://services.google.com/fh/files/misc/apt42-crooked-charms-cons-and-compromises.pdf)
-    [FireEye. (2015, March). SOUTHEAST ASIA: AN EVOLVING CYBER THREAT LANDSCAPE. Retrieved February 5, 2024.](https://web.archive.org/web/20220122121143/https://www.fireeye.com/content/dam/fireeye-www/current-threats/pdfs/rpt-southeast-asia-threat-landscape.pdf)
-    [Mandiant. (n.d.). Advanced Persistent Threats (APTs). Retrieved February 14, 2024.](https://www.mandiant.com/resources/insights/apt-groups)
-    [Doaty, J., Garrett, P.. (2018, September 10). We’re Seeing a Resurgence of the Demonic Astaroth WMIC Trojan. Retrieved September 25, 2024.](https://web.archive.org/web/20200302071436/https://cofense.com/seeing-resurgence-demonic-astaroth-wmic-trojan/)
-    [Nyan-x-Cat. (n.d.). NYAN-x-CAT / AsyncRAT-C-Sharp. Retrieved October 3, 2023.](https://github.com/NYAN-x-CAT/AsyncRAT-C-Sharp/blob/master/README.md)
-    [Hromcova, Z. (2019, October). AT COMMANDS, TOR-BASED COMMUNICATIONS: MEET ATTOR, A FANTASY CREATURE AND ALSO A SPY PLATFORM. Retrieved May 6, 2020.](https://www.welivesecurity.com/wp-content/uploads/2019/10/ESET_Attor.pdf)
-    [Lim, M.. (2019, April 26). BabyShark Malware Part Two – Attacks Continue Using KimJongRAT and PCRat . Retrieved October 7, 2019.](https://unit42.paloaltonetworks.com/babyshark-malware-part-two-attacks-continue-using-kimjongrat-and-pcrat/)
-    [Settle, A., et al. (2016, August 8). MONSOON - Analysis Of An APT Campaign. Retrieved September 22, 2016.](https://www.forcepoint.com/sites/default/files/resources/files/forcepoint-security-labs-monsoon-analysis-report.pdf)
-    [Levene, B. et al.. (2018, March 7). Patchwork Continues to Deliver BADNEWS to the Indian Subcontinent. Retrieved March 31, 2018.](https://researchcenter.paloaltonetworks.com/2018/03/unit42-patchwork-continues-deliver-badnews-indian-subcontinent/)
-    [Lunghi, D., et al. (2017, December). Untangling the Patchwork Cyberespionage Group. Retrieved July 10, 2018.](https://documents.trendmicro.com/assets/tech-brief-untangling-the-patchwork-cyberespionage-group.pdf)
-    [Bar, T., Conant, S. (2017, October 20). BadPatch. Retrieved November 13, 2018.](https://researchcenter.paloaltonetworks.com/2017/10/unit42-badpatch/)
-    [Galperin, E., Et al.. (2016, August 4). When Governments Attack: State Sponsored Malware Attacks Against Activists, Lawyers, and Journalists. Retrieved May 23, 2018.](https://www.blackhat.com/docs/us-16/materials/us-16-Quintin-When-Governments-Attack-State-Sponsored-Malware-Attacks-Against-Activists-Lawyers-And-Journalists.pdf)
-    [Mandiant. (n.d.). Appendix C (Digital) - The Malware Arsenal. Retrieved July 18, 2016.](https://www.mandiant.com/sites/default/files/2021-09/mandiant-apt1-report.pdf)
-    [Baumgartner, K. and Garnaeva, M.. (2014, November 3). BE2 custom plugins, router abuse, and target profiles. Retrieved March 24, 2016.](https://securelist.com/be2-custom-plugins-router-abuse-and-target-profiles/67353/)
-    [Robert Falcone, Mike Scott, Juan Cortes. (2015, November 10). Bookworm Trojan: A Model of Modular Architecture. Retrieved July 21, 2025.](https://unit42.paloaltonetworks.com/bookworm-trojan-a-model-of-modular-architecture/)
-    [Symantec Security Response. (2015, December 7). Iran-based attackers use back door threats to spy on Middle Eastern targets. Retrieved April 17, 2019.](https://www.symantec.com/connect/blogs/iran-based-attackers-use-back-door-threats-spy-middle-eastern-targets)
-    [Kaspersky Lab's Global Research and Analysis Team. (2015, February). CARBANAK APT THE GREAT BANK ROBBERY. Retrieved August 23, 2018.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/08064518/Carbanak_APT_eng.pdf)
-    [Bennett, J., Vengerik, B. (2017, June 12). Behind the CARBANAK Backdoor. Retrieved June 11, 2018.](https://www.fireeye.com/blog/threat-research/2017/06/behind-the-carbanak-backdoor.html)
-    [Grunzweig, J.. (2017, April 20). Cardinal RAT Active for Over Two Years. Retrieved December 8, 2018.](https://researchcenter.paloaltonetworks.com/2017/04/unit42-cardinal-rat-active-two-years/)
-    [Balanza, M. (2018, April 02). Infostealer.Catchamas. Retrieved November 17, 2024.](https://web.archive.org/web/20190508165711/https://www-west.symantec.com/content/symantec/english/en/security-center/writeup.html/2018-040209-1742-99)
-    [Jenkins, L. at al. (2022, August 4). ROADSWEEP Ransomware - Likely Iranian Threat Actor Conducts Politically Motivated Disruptive Activity Against Albanian Government Organizations. Retrieved August 6, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/likely-iranian-threat-actor-conducts-politically-motivated-disruptive-activity-against/)
-    [Alperovitch, D.. (2016, June 15). Bears in the Midst: Intrusion into the Democratic National Committee. Retrieved August 3, 2016.](https://www.crowdstrike.com/blog/bears-midst-intrusion-democratic-national-committee/)
-    [Lunghi, D. et al. (2020, February). Uncovering DRBControl. Retrieved November 12, 2021.](https://documents.trendmicro.com/assets/white_papers/wp-uncovering-DRBcontrol.pdf)
-    [Chen, T. and Chen, Z. (2020, February 17). CLAMBLING - A New Backdoor Base On Dropbox. Retrieved November 12, 2021.](https://www.talent-jump.com/article/2020/02/17/CLAMBLING-A-New-Backdoor-Base-On-Dropbox-en/)
-    [Strategic Cyber LLC. (2017, March 14). Cobalt Strike Manual. Retrieved May 24, 2017.](https://web.archive.org/web/20210825130434/https://cobaltstrike.com/downloads/csmanual38.pdf)
-    [Amnesty International. (2021, February 24). Vietnamese activists targeted by notorious hacking group. Retrieved March 1, 2021.](https://www.amnestyusa.org/wp-content/uploads/2021/02/Click-and-Bait_Vietnamese-Human-Rights-Defenders-Targeted-with-Spyware-Attacks.pdf)
-    [Strategic Cyber LLC. (2020, November 5). Cobalt Strike: Advanced Threat Tactics for Penetration Testers. Retrieved April 13, 2021.](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)
-    [Yadav, A., et al. (2017, August 31). Cobian RAT – A backdoored RAT. Retrieved November 13, 2018.](https://www.zscaler.com/blogs/research/cobian-rat-backdoored-rat)
-    [Sudeep Singh. (2025, April 16). Latest Mustang Panda Arsenal: PAKLOG, CorKLOG, and SplatCloak | P2. Retrieved September 12, 2025.](https://www.zscaler.com/blogs/security-research/latest-mustang-panda-arsenal-paklog-corklog-and-splatcloak-p2)
-    [F-Secure Labs. (2015, September 17). The Dukes: 7 years of Russian cyberespionage. Retrieved December 10, 2015.](https://www.f-secure.com/documents/996508/1030745/dukes_whitepaper.pdf)
-    [Huss, D. (2016, March 1). Operation Transparent Tribe. Retrieved June 8, 2016.](https://www.proofpoint.com/sites/default/files/proofpoint-operation-transparent-tribe-threat-insight-en.pdf)
-    [Dedola, G. (2020, August 20). Transparent Tribe: Evolution analysis, part 1. Retrieved September 2, 2021.](https://securelist.com/transparent-tribe-part-1/98127/)
-    [N. Baisini. (2022, July 13). Transparent Tribe begins targeting education sector in latest campaign. Retrieved September 22, 2022.](https://blog.talosintelligence.com/2022/07/transparent-tribe-targets-education.html)
-    [Roccio, T., et al. (2021, April). Technical Analysis of Cuba Ransomware. Retrieved June 18, 2021.](https://www.mcafee.com/enterprise/en-us/assets/reports/rp-cuba-ransomware.pdf)
-    [Meltzer, M. et al. (2024, January 10). Active Exploitation of Two Zero-Day Vulnerabilities in Ivanti Connect Secure VPN. Retrieved February 27, 2024.](https://www.volexity.com/blog/2024/01/10/active-exploitation-of-two-zero-day-vulnerabilities-in-ivanti-connect-secure-vpn/)
-    [TrendMicro. (2014, September 03). DARKCOMET. Retrieved November 6, 2018.](https://www.trendmicro.com/vinfo/us/threat-encyclopedia/malware/DARKCOMET)
-    [Adi Zeligson & Rotem Kerner. (2018, November 13). Enter The DarkGate - New Cryptocurrency Mining and Ransomware Campaign. Retrieved February 9, 2024.](https://www.fortinet.com/blog/threat-research/enter-the-darkgate-new-cryptocurrency-mining-and-ransomware-campaign)
-    [McGraw, T. (2024, December 4). Black Basta Ransomware Campaign Drops Zbot, DarkGate, and Custom Malware. Retrieved December 9, 2024.](https://www.rapid7.com/blog/post/2024/12/04/black-basta-ransomware-campaign-drops-zbot-darkgate-and-custom-malware/)
-    [Kaspersky Lab's Global Research and Analysis Team. (2014, November). The Darkhotel APT A Story of Unusual Hospitality. Retrieved November 12, 2014.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/08070903/darkhotel_kl_07.11.pdf)
-    [Secureworks Counter Threat Unit Research Team. (2022, August 17). DarkTortilla Malware Analysis. Retrieved November 3, 2022.](https://www.secureworks.com/research/darktortilla-malware-analysis)
-    [Smith, S., Stafford, M. (2021, December 14). DarkWatchman: A new evolution in fileless techniques. Retrieved January 10, 2022.](https://web.archive.org/web/20220629230035/https://www.prevailion.com/darkwatchman-new-fileless-techniques/)
-    [Chen, J. and Hsieh, M. (2017, November 7). REDBALDKNIGHT/BRONZE BUTLER’s Daserf Backdoor Now Using Steganography. Retrieved December 27, 2017.](http://blog.trendmicro.com/trendlabs-security-intelligence/redbaldknight-bronze-butler-daserf-backdoor-now-using-steganography/)
-    [Counter Threat Unit Research Team. (2017, October 12). BRONZE BUTLER Targets Japanese Enterprises. Retrieved January 4, 2018.](https://www.secureworks.com/research/bronze-butler-targets-japanese-businesses)
-    [Pellegrino, G. (2025, December 16). BlindEagle Targets Colombian Government Agency with Caminho and DCRAT. Retrieved April 16, 2026.](https://www.zscaler.com/blogs/security-research/blindeagle-targets-colombian-government-agency-caminho-and-dcrat)
-    [FireEye. (2018, March 16). Suspected Chinese Cyber Espionage Group (TEMP.Periscope) Targeting U.S. Engineering and Maritime Industries. Retrieved April 11, 2018.](https://www.fireeye.com/blog/threat-research/2018/03/suspected-chinese-espionage-group-targeting-maritime-and-engineering-industries.html)
-    [FireEye. (2018, February 20). APT37 (Reaper): The Overlooked North Korean Actor. Retrieved November 17, 2024.](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)
-    [Grunzweig, J. (2018, October 01). NOKKI Almost Ties the Knot with DOGCALL: Reaper Group Uses New Malware to Deploy RAT. Retrieved November 5, 2018.](https://researchcenter.paloaltonetworks.com/2018/10/unit42-nokki-almost-ties-the-knot-with-dogcall-reaper-group-uses-new-malware-to-deploy-rat/)
-    [John Wolfram, Josh Murchie, Matt Lin, Daniel Ainsworth, Robert Wallace, Dimiter Andonov, Dhanesh Kizhakkinan, Jacob Thompson. (2025, January 8). Ivanti Connect Secure VPN Targeted in New Zero-Day Exploitation. Retrieved April 14, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-connect-secure-vpn-zero-day)
-    [Sila Ozeren Hacioglu. (2025, May 5). UNC5221’s Latest Exploit: Weaponizing CVE-2025-22457 in Ivanti Connect Secure. Retrieved April 13, 2026.](https://www.picussecurity.com/resource/blog/unc5221-cve-2025-22457-ivanti-connect-secure)
-    [Konstantin Zykov. (2019, September 23). Hello! My name is Dtrack. Retrieved January 20, 2021.](https://securelist.com/my-name-is-dtrack/93338/)
-    [Symantec Security Response. (2011, November). W32.Duqu: The precursor to the next Stuxnet. Retrieved September 17, 2015.](https://web.archive.org/web/20111213083345/https://www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/w32_duqu_the_precursor_to_the_next_stuxnet.pdf)
-    [Mike Stokkel et al. (2024, July 18). APT41 Has Arisen From the DUST. Retrieved September 16, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/apt41-arisen-from-dust)
-    [ClearSky. (2016, January 7). Operation DustySky. Retrieved January 8, 2016.](https://www.clearskysec.com/wp-content/uploads/2016/01/Operation%20DustySky_TLP_WHITE.pdf)
-    [Cybersecurity and Infrastructure Security Agency. (2020, August 26). MAR-10301706-1.v1 - North Korean Remote Access Tool: ECCENTRICBANDWAGON. Retrieved March 18, 2021.](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-239a)
-    [Schroeder, W., Warner, J., Nelson, M. (n.d.). Github PowerShellEmpire. Retrieved April 28, 2016.](https://github.com/PowerShellEmpire/Empire)
-    [PwC and BAE Systems. (2017, April). Operation Cloud Hopper: Technical Annex. Retrieved April 13, 2017.](https://www.pwc.co.uk/cyber-security/pdf/pwc-uk-operation-cloud-hopper-technical-annex-april-2017.pdf)
-    [Threat Intelligence and Research. (2015, March 30). VOLATILE CEDAR. Retrieved February 8, 2021.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2015/03/20082004/volatile-cedar-technical-report.pdf)
-    [ClearSky Cyber Security. (2021, January). “Lebanese Cedar” APT Global Lebanese Espionage Campaign Leveraging Web Servers. Retrieved February 10, 2021.](https://www.clearskysec.com/wp-content/uploads/2021/01/Lebanese-Cedar-APT.pdf)
-    [Falcone, R. and Miller-Osborn, J.. (2016, January 24). Scarlet Mimic: Years-Long Espionage Campaign Targets Minority Activists. Retrieved February 10, 2016.](http://researchcenter.paloaltonetworks.com/2016/01/scarlet-mimic-years-long-espionage-targets-minority-activists/)
-    [Ta, V., et al. (2022, August 8). FIN13: A Cybercriminal Threat Actor Focused on Mexico. Retrieved February 9, 2023.](https://www.mandiant.com/resources/blog/fin13-cybercriminal-mexico)
-    [Vengerik, B. et al.. (2014, December 5). Hacking the Street? FIN4 Likely Playing the Market. Retrieved December 17, 2018.](https://web.archive.org/web/20220313062515/https://www.mandiant.com/sites/default/files/2021-09/rpt-fin4.pdf)
-    [Vengerik, B. & Dennesen, K.. (2014, December 5). Hacking the Street? FIN4 Likely Playing the Market. Retrieved January 15, 2019.](https://www2.fireeye.com/WBNR-14Q4NAMFIN4.html)
-    [Financial Security Institute. (2020, February 28). Profiling of TA505 Threat Group That Continues to Attack the Financial Sector. Retrieved July 14, 2022.](https://www.fsec.or.kr/user/bbs/fsec/163/344/bbsDataView/1382.do?page=1&column=&search=&searchSDate=&searchEDate=&bbsDataCategory=)
-    [Vrabie, V. (2020, November). Dissecting a Chinese APT Targeting South Eastern Asian Government Institutions. Retrieved September 19, 2022.](https://www.bitdefender.com/files/News/CaseStudies/study/379/Bitdefender-Whitepaper-Chinese-APT.pdf)
-    [Bryan Lee and Rob Downs. (2016, February 12). A Look Into Fysbis: Sofacy’s Linux Backdoor. Retrieved September 10, 2017.](https://researchcenter.paloaltonetworks.com/2016/02/a-look-into-fysbis-sofacys-linux-backdoor/)
-    [Alintanahin, K. (2014, March 13). Kunming Attack Leads to Gh0st RAT Variant. Retrieved November 12, 2014.](http://blog.trendmicro.com/trendlabs-security-intelligence/kunming-attack-leads-to-gh0st-rat-variant/)
-    [Quinn, J. (2019, March 25). The odd case of a Gh0stRAT variant. Retrieved July 15, 2020.](https://cybersecurity.att.com/blogs/labs-research/the-odd-case-of-a-gh0strat-variant)
-    [ESET. (2020, April 28). Grandoreiro: How engorged can an EXE get?. Retrieved November 13, 2020.](https://www.welivesecurity.com/2020/04/28/grandoreiro-how-engorged-can-exe-get/)
-    [Cherepanov, A. (2018, October). GREYENERGY A successor to BlackEnergy. Retrieved November 15, 2018.](https://www.welivesecurity.com/wp-content/uploads/2018/10/ESET_GreyEnergy.pdf)
-    [Scott-Railton, J., et al. (2016, August 2). Group5: Syria and the Iranian Connection. Retrieved September 26, 2016.](https://citizenlab.ca/2016/08/group5-syria/)
-    [Falcone, R. and Lee, B.. (2016, May 26). The OilRig Campaign: Attacks on Saudi Arabian Organizations Deliver Helminth Backdoor. Retrieved May 3, 2017.](http://researchcenter.paloaltonetworks.com/2016/05/the-oilrig-campaign-attacks-on-saudi-arabian-organizations-deliver-helminth-backdoor/)
-    [SecureWorks 2019, August 27 LYCEUM Takes Center Stage in Middle East Campaign Retrieved. 2019/11/19](https://www.secureworks.com/blog/lyceum-takes-center-stage-in-middle-east-campaign)
-    [Kayal, A. et al. (2021, October). LYCEUM REBORN: COUNTERINTELLIGENCE IN THE MIDDLE EAST. Retrieved June 14, 2022.](https://vblocalhost.com/uploads/VB2021-Kayal-etal.pdf)
-    [Kirill Boychenko. (2025, June 25). Another Wave: North Korean Contagious Interview Campaign Drops 35 New Malicious npm Packages. Retrieved October 19, 2025.](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)
-    [Dell SecureWorks Counter Threat Unit Threat Intelligence. (2015, August 5). Threat Group-3390 Targets Organizations for Cyberespionage. Retrieved August 18, 2018.](https://www.secureworks.com/research/threat-group-3390-targets-organizations-for-cyberespionage)
-    [Unit 42. (2019, December 2). Imminent Monitor – a RAT Down Under. Retrieved May 5, 2020.](https://unit42.paloaltonetworks.com/imminent-monitor-a-rat-down-under/)
-    [eSentire Threat Response Unit (TRU). (2024, November 14). Bored BeaverTail & InvisibleFerret Yacht Club – A Lazarus Lure Pt.2. Retrieved October 17, 2025.](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)
-    [Matej Havranek. (2025, February 20). DeceptiveDevelopment targets freelance developers. Retrieved October 17, 2025.](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)
-    [Unit 42. (2023, November 21). Hacking Employers and Seeking Employment: Two Job-Related Campaigns Bear Hallmarks of North Korean Threat Actors. Retrieved October 17, 2025.](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)
-    [Seongsu Park. (2024, November 4). From Pyongyang to Your Payroll: The Rise of North Korean Remote Workers in the West. Retrieved October 17, 2025.](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)
-    [Hromcova, Z. and Cherpanov, A. (2020, June). INVISIMOLE: THE HIDDEN PART OF THE STORY. Retrieved July 16, 2020.](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_InvisiMole.pdf)
-    [Windows Defender Advanced Threat Hunting Team. (2016, April 29). PLATINUM: Targeted attacks in South and Southeast Asia. Retrieved February 15, 2018.](https://download.microsoft.com/download/2/2/5/225BFE3E-E1DE-4F5B-A77B-71200928D209/Platinum%20feature%20article%20-%20Targeted%20attacks%20in%20South%20and%20Southeast%20Asia%20April%202016.pdf)
-    [Sharma, R. (2018, August 15). Revamped jRAT Uses New Anti-Parsing Techniques. Retrieved September 21, 2018.](https://www.symantec.com/blogs/threat-intelligence/jrat-new-anti-parsing-techniques)
-    [Kamluk, V. & Gostev, A. (2016, February). Adwind - A Cross-Platform RAT. Retrieved April 23, 2019.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07195002/KL_AdwindPublicReport_2016.pdf)
-    [Yadav, A., et al. (2016, January 29). Malicious Office files dropping Kasidet and Dridex. Retrieved March 24, 2016.](http://research.zscaler.com/2016/01/malicious-office-files-dropping-kasidet.html)
-    [Smallridge, R. (2018, March 10). APT15 is alive and strong: An analysis of RoyalCli and RoyalDNS. Retrieved April 4, 2018.](https://research.nccgroup.com/2018/03/10/apt15-is-alive-and-strong-an-analysis-of-royalcli-and-royaldns/)
-    [MSTIC. (2021, December 6). NICKEL targeting government organizations across Latin America and Europe. Retrieved March 18, 2022.](https://www.microsoft.com/security/blog/2021/12/06/nickel-targeting-government-organizations-across-latin-america-and-europe)

-    [Guarnieri, C., Schloesser M. (2013, June 7). KeyBoy, Targeted Attacks against Vietnam and India. Retrieved June 14, 2019.](https://blog.rapid7.com/2013/06/07/keyboy-targeted-attacks-against-vietnam-and-india/)
-    [Dahan, A. et al. (2020, November 2). Back to the Future: Inside the Kimsuky KGH Spyware Suite. Retrieved November 6, 2020.](https://www.cybereason.com/blog/back-to-the-future-inside-the-kimsuky-kgh-spyware-suite)
-    [Alyac. (2019, April 3). Kimsuky Organization Steals Operation Stealth Power. Retrieved August 13, 2019.](https://blog.alyac.co.kr/2234)
-    [Tarakanov , D.. (2013, September 11). The “Kimsuky” Operation: A North Korean APT?. Retrieved August 13, 2019.](https://securelist.com/the-kimsuky-operation-a-north-korean-apt/57915/)
-    [CISA, FBI, CNMF. (2020, October 27). https://us-cert.cisa.gov/ncas/alerts/aa20-301a. Retrieved November 4, 2020.](https://us-cert.cisa.gov/ncas/alerts/aa20-301a)
-    [ASERT team. (2018, December 5). STOLEN PENCIL Campaign Targets Academia. Retrieved February 5, 2019.](https://asert.arbornetworks.com/stolen-pencil-campaign-targets-academia/)
-    [Varadharajan Krishnasamy, Aditya K Sood. (2025, July 29). From Reconnaissance to Control: The Operational Blueprint of Kimsuky APT for Cyber Espionage. Retrieved April 18, 2026.](https://www.aryaka.com/docs/reports/aryaka-kimsuky-apt-operational-blueprint.pdf)
-    [Bermejo, L., et al. (2017, June 22). Following the Trail of BlackTech’s Cyber Espionage Campaigns. Retrieved May 5, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)
-    [Rascagneres, P. (2017, May 03). KONNI: A Malware Under The Radar For Years. Retrieved November 5, 2018.](https://blog.talosintelligence.com/2017/05/konni-malware-under-radar-for-years.html)
-    [Novetta Threat Research Group. (2016, February 24). Operation Blockbuster: Unraveling the Long Thread of the Sony Attack. Retrieved February 25, 2016.](https://web.archive.org/web/20160226161828/https://www.operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Report.pdf)
-    [Novetta Threat Research Group. (2016, February 24). Operation Blockbuster: Tools Report. Retrieved March 10, 2016.](https://web.archive.org/web/20220425194457/https://operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Tools-Report.pdf)
-    [Breitenbacher, D. (2022, December 14). Unmasking MirrorFace: Operation LiberalFace targeting Japanese political entities. Retrieved April 17, 2026.](https://www.welivesecurity.com/2022/12/14/unmasking-mirrorface-operation-liberalface-targeting-japanese-political-entities/)
-    [ITOCHU. (2024, January 24). The Endless Struggle Against APT10: Insights from LODEINFO v0.6.6 - v0.7.3 Analysis. Retrieved April 17, 2026.](https://blog-en.itochuci.co.jp/entry/2024/01/24/134100)
-    [Trend Micro. (2024, November 19). Spot the Difference: Earth Kasha's New LODEINFO Campaign And The Correlation Analysis With The APT10 Umbrella. Retrieved April 17, 2026.](https://www.trendmicro.com/en_us/research/24/k/lodeinfo-campaign-of-earth-kasha.html)
-    [Kazem, M. (2019, November 25). Trojan:W32/Lokibot. Retrieved May 15, 2020.](https://www.f-secure.com/v-descs/trojan_w32_lokibot.shtml)
-    [ESET. (2019, July). MACHETE JUST GOT SHARPER Venezuelan government institutions under attack. Retrieved September 13, 2019.](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)
-    [Kaspersky Global Research and Analysis Team. (2014, August 20). El Machete. Retrieved September 13, 2019.](https://securelist.com/el-machete/66108/)
-    [The Cylance Threat Research Team. (2017, March 22). El Machete's Malware Attacks Cut Through LATAM. Retrieved September 13, 2019.](https://threatvector.cylance.com/en_us/home/el-machete-malware-attacks-cut-through-latam.html)
-    [kate. (2020, September 25). APT-C-43 steals Venezuelan military secrets to provide intelligence support for the reactionaries — HpReact campaign. Retrieved November 20, 2020.](https://blog.360totalsecurity.com/en/apt-c-43-steals-venezuelan-military-secrets-to-provide-intelligence-support-for-the-reactionaries-hpreact-campaign/)
-    [Wardle, P. (2021, November 11). OSX.CDDS (OSX.MacMa). Retrieved June 30, 2022.](https://objective-see.org/blog/blog_0x69.html)
-    [Stokes, P. (2021, November 15). Infect If Needed | A Deeper Dive Into Targeted Backdoor macOS.Macma. Retrieved June 30, 2022.](https://www.sentinelone.com/labs/infect-if-needed-a-deeper-dive-into-targeted-backdoor-macos-macma/)
-    [Patrick Wardle. (n.d.). Mac Malware of 2017. Retrieved September 21, 2018.](https://objective-see.com/blog/blog_0x25.html)
-    [Lee, B. and Falcone, R. (2017, February 15). Magic Hound Campaign Attacks Saudi Targets. Retrieved December 27, 2017.](https://researchcenter.paloaltonetworks.com/2017/02/unit42-magic-hound-campaign-attacks-saudi-targets/)
-    [GReAT. (2021, June 16). Ferocious Kitten: 6 Years of Covert Surveillance in Iran. Retrieved September 22, 2021.](https://securelist.com/ferocious-kitten-6-years-of-covert-surveillance-in-iran/102806/)
-    [ClearSky Cyber Security and Trend Micro. (2017, July). Operation Wilted Tulip: Exposing a cyber espionage apparatus. Retrieved August 21, 2017.](http://www.clearskysec.com/wp-content/uploads/2017/07/Operation_Wilted_Tulip.pdf)
-    [Minerva Labs LTD and ClearSky Cyber Security. (2015, November 23). CopyKittens Attack Group. Retrieved November 17, 2024.](https://cdn2.hubspot.net/hubfs/1903456/Whitepapers/CopyKittens.pdf)
-    [US District Court Southern District of New York. (2018, December 17). United States v. Zhu Hua Indictment. Retrieved December 17, 2020.](https://www.justice.gov/opa/page/file/1122671/download)
-    [Ehrlich, A., et al. (2022, September). THE MYSTERY OF METADOR | AN UNATTRIBUTED THREAT HIDING IN TELCOS, ISPS, AND UNIVERSITIES. Retrieved January 23, 2023.](https://assets.sentinelone.com/sentinellabs22/metador#page=1)
-    [SentinelLabs. (2022, September 22). Metador Technical Appendix. Retrieved April 4, 2023.](https://docs.google.com/document/d/1e9ZTW9b71YwFWS_18ZwDAxa-cYbV8q1wUefmKZLYVsA/edit#heading=h.lmnbtht1ikzm)
-    [Zhang, X. (2020, February 4). Another Metamorfo Variant Targeting Customers of Financial Institutions in More Countries. Retrieved July 30, 2020.](https://www.fortinet.com/blog/threat-research/another-metamorfo-variant-targeting-customers-of-financial-institutions)
-    [ESET Research. (2019, October 3). Casbaneiro: peculiarities of this banking Trojan that affects Brazil and Mexico. Retrieved September 23, 2021.](https://www.welivesecurity.com/2019/10/03/casbaneiro-trojan-dangerous-cooking/)
-    [Facundo Muñoz. (2023, April 26). Evasive Panda APT group delivers malware via updates for popular Chinese software. Retrieved July 25, 2024.](https://www.welivesecurity.com/2023/04/26/evasive-panda-apt-group-malware-updates-popular-chinese-software/)
-    [Threat Hunter Team. (2023, April 20). Daggerfly: APT Actor Targets Telecoms Company in Africa. Retrieved July 25, 2024.](https://symantec-enterprise-blogs.security.com/threat-intelligence/apt-attacks-telecoms-africa-mgbot)
-    [Tsarfaty, Y. (2018, July 25). Micropsia Malware. Retrieved November 13, 2018.](https://www.radware.com/blog/security/2018/07/micropsia-malware/)
-    [ESET Security. (2019, November 19). Mispadu: Advertisement for a discounted Unhappy Meal. Retrieved March 13, 2024.](https://www.welivesecurity.com/2019/11/19/mispadu-advertisement-discounted-unhappy-meal/)
-    [Garcia, F., Regalado, D. (2023, March 7). Inside Mispadu massive infection campaign in LATAM. Retrieved March 15, 2024.](https://www.metabaseq.com/mispadu-banking-trojan/)
-    [SCILabs. (2023, May 23). Evolution of banking trojan URSA/Mispadu. Retrieved March 13, 2024.](https://blog.scilabs.mx/en/evolution-of-banking-trojan-ursa-mispadu/)
-    [Miller-Osborn, J. and Grunzweig, J.. (2017, March 30). Trochilus and New MoonWind RATs Used In Attack Against Thai Organizations. Retrieved March 30, 2017.](http://researchcenter.paloaltonetworks.com/2017/03/unit42-trochilus-rat-new-moonwind-rat-used-attack-thai-utility-organizations/)
-    [Kasza, A., Halfpop, T. (2016, February 09). NanoCoreRAT Behind an Increase in Tax-Themed Phishing E-mails. Retrieved November 9, 2018.](https://researchcenter.paloaltonetworks.com/2016/02/nanocorerat-behind-an-increase-in-tax-themed-phishing-e-mails/)
-    [Mercer, W., Rascagneres, P. (2018, May 31). NavRAT Uses US-North Korea Summit As Decoy For Attacks In South Korea. Retrieved June 11, 2018.](https://blog.talosintelligence.com/2018/05/navrat.html)
-    [Kaspersky Lab's Global Research and Analysis Team. (n.d.). The NetTraveler (aka ‘Travnet’). Retrieved November 17, 2024.](https://web.archive.org/web/20160326004042/http://kasperskycontenthub.com/wp-content/uploads/sites/43/vlpdfs/kaspersky-the-net-traveler-part1-final.pdf)
-    [McAfee. (2015, March 2). Netwire RAT Behind Recent Targeted Attacks. Retrieved February 15, 2018.](https://securingtomorrow.mcafee.com/mcafee-labs/netwire-rat-behind-recent-targeted-attacks/)
-    [Davis, S. and Carr, N. (2017, September 21). APT33: New Insights into Iranian Cyber Espionage Group. Retrieved February 15, 2018.](https://www.brighttalk.com/webcast/10703/275683)
-    [Maniath, S. and Kadam P. (2019, March 19). Dissecting a NETWIRE Phishing Campaign's Usage of Process Hollowing. Retrieved January 7, 2021.](https://www.mandiant.com/resources/blog/dissecting-netwire-phishing-campaigns-usage-process-hollowing)
-    [Lambert, T. (2020, January 29). Intro to Netwire. Retrieved January 7, 2021.](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)
-    [Proofpoint. (2020, December 2). Geofenced NetWire Campaigns. Retrieved January 7, 2021.](https://www.proofpoint.com/us/blog/threat-insight/geofenced-netwire-campaigns)
-    [Faou, M. (2023, August 10). MoustachedBouncer: Espionage against foreign diplomats in Belarus. Retrieved September 25, 2023.](https://www.welivesecurity.com/en/eset-research/moustachedbouncer-espionage-against-foreign-diplomats-in-belarus/)
-    [Fidelis Cybersecurity. (2013, June 28). Fidelis Threat Advisory #1009: "njRAT" Uncovered. Retrieved June 4, 2019.](https://www.threatminer.org/_reports/2013/fta-1009---njrat-uncovered-1.pdf)
-    [Pascual, C. (2018, November 27). AutoIt-Compiled Worm Affecting Removable Media Delivers Fileless Version of BLADABINDI/njRAT Backdoor. Retrieved June 4, 2019.](https://blog.trendmicro.com/trendlabs-security-intelligence/autoit-compiled-worm-affecting-removable-media-delivers-fileless-version-of-bladabindi-njrat-backdoor/)
-    [Global Research & Analysis Team, Kaspersky. (2024, August 19). BlindEagle flying high in Latin America. Retrieved April 16, 2026.](https://securelist.com/blindeagle-apt/113414/)
-    [Davis, S. and Caban, D. (2017, December 19). APT34 - New Targeted Attack in the Middle East. Retrieved December 20, 2017.](https://www.brighttalk.com/webcast/10703/296317/apt34-new-targeted-attack-in-the-middle-east)
-    [Bromiley, M., et al.. (2019, July 18). Hard Pass: Declining APT34’s Invite to Join Their Professional Network. Retrieved August 26, 2019.](https://www.fireeye.com/blog/threat-research/2019/07/hard-pass-declining-apt34-invite-to-join-their-professional-network.html)
-    [Symantec Threat Hunter Team. (2023, October 19). Crambus: New Campaign Targets Middle Eastern Government. Retrieved November 27, 2024.](https://www.security.com/threat-intelligence/crambus-middle-east-government)
-    [Hromcova, Z. (2019, July). OKRUM AND KETRICAN: AN OVERVIEW OF RECENT KE3CHANG GROUP ACTIVITY. Retrieved May 6, 2020.](https://www.welivesecurity.com/wp-content/uploads/2019/07/ESET_Okrum_and_Ketrican.pdf)
-    [Dantzig, M. v., Schamper, E. (2019, December 19). Operation Wocao: Shining a light on one of China’s hidden hacking groups. Retrieved October 8, 2020.](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)
-    [Computer Incident Response Center Luxembourg. (2013, March 29). Analysis of a PlugX variant. Retrieved November 5, 2018.](http://circl.lu/assets/files/tr-12/tr-12-circl-plugx-analysis-v1.pdf)
-    [Mercer, W, et al. (2020, April 16). PoetRAT: Python RAT uses COVID-19 lures to target Azerbaijan public and private sectors. Retrieved April 27, 2020.](https://blog.talosintelligence.com/2020/04/poetrat-covid-19-lures.html)
-    [FireEye. (2014). POISON IVY: Assessing Damage and Extracting Intelligence. Retrieved September 19, 2024.](https://www.mandiant.com/sites/default/files/2021-09/rpt-poison-ivy.pdf)
-    [Hayashi, K. (2005, August 18). Backdoor.Darkmoon. Retrieved February 23, 2018.](https://www.symantec.com/security_response/writeup.jsp?docid=2005-081910-3934-99)
-    [Nettitude. (2018, July 23). Python Server for PoshC2. Retrieved April 23, 2019.](https://github.com/nettitude/PoshC2_Python)
-    [Cybereason Nocturnus. (2022, February 1). PowerLess Trojan: Iranian APT Phosphorus Adds New PowerShell Backdoor for Espionage. Retrieved June 1, 2022.](https://www.cybereason.com/blog/research/powerless-trojan-iranian-apt-phosphorus-adds-new-powershell-backdoor-for-espionage)
-    [PowerShellMafia. (2012, May 26). PowerSploit - A PowerShell Post-Exploitation Framework. Retrieved February 6, 2018.](https://github.com/PowerShellMafia/PowerSploit)
-    [PowerSploit. (n.d.). PowerSploit. Retrieved February 6, 2018.](http://powersploit.readthedocs.io)
-    [Cherepanov, A.. (2016, May 17). Operation Groundbait: Analysis of a surveillance toolkit. Retrieved May 18, 2016.](http://www.welivesecurity.com/wp-content/uploads/2016/05/Operation-Groundbait.pdf)
-    [Nicolas Verdier. (n.d.). Retrieved January 29, 2018.](https://github.com/n1nj4sec/pupy)
-    [Sette, N. et al. (2020, June 4). Qakbot Malware Now Exfiltrating Emails for Sophisticated Thread Hijacking Attacks. Retrieved September 27, 2021.](https://www.kroll.com/en/insights/publications/cyber/qakbot-malware-exfiltrating-emails-thread-hijacking-attacks)
-    [Trend Micro. (2020, December 17). QAKBOT: A decade-old malware still with new tricks. Retrieved November 17, 2024.](https://success.trendmicro.com/en-US/solution/KA-0011282)
-    [Kuzmenko, A. et al. (2021, September 2). QakBot technical analysis. Retrieved September 27, 2021.](https://securelist.com/qakbot-technical-analysis/103931/)
-    [MaxXor. (n.d.). QuasarRAT. Retrieved July 10, 2018.](https://github.com/quasar/QuasarRAT)
-    [Meltzer, M, et al. (2018, June 07). Patchwork APT Group Targets US Think Tanks. Retrieved July 16, 2018.](https://www.volexity.com/blog/2018/06/07/patchwork-apt-group-targets-us-think-tanks/)
-    [Global Threat Center, Intelligence Team. (2020, December). APT27 Turns to Ransomware. Retrieved November 12, 2021.](https://web.archive.org/web/20210104144857/https://shared-public-reports.s3-eu-west-1.amazonaws.com/APT27+turns+to+ransomware.pdf)
-    [Kaspersky Lab's Global Research and Analysis Team. (2014, November 24). THE REGIN PLATFORM NATION-STATE OWNAGE OF GSM NETWORKS. Retrieved December 1, 2014.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/08070305/Kaspersky_Lab_whitepaper_Regin_platform_eng.pdf)
-    [Bacurio, F., Salvio, J. (2017, February 14). REMCOS: A New RAT In The Wild. Retrieved November 6, 2018.](https://www.fortinet.com/blog/threat-research/remcos-a-new-rat-in-the-wild-2.html)
-    [Brumaghin, E., Unterbrink, H. (2018, August 22). Picking Apart Remcos Botnet-In-A-Box. Retrieved November 6, 2018.](https://blog.talosintelligence.com/2018/08/picking-apart-remcos.html)
-    [Legezo, D. (2019, January 30). Chafer used Remexi malware to spy on Iran-based foreign diplomatic entities. Retrieved April 17, 2019.](https://securelist.com/chafer-used-remexi-malware/89538/)
-    [Symantec Security Response. (2016, August 8). Backdoor.Remsec indicators of compromise. Retrieved August 17, 2016.](http://www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/Symantec_Remsec_IOCs.pdf)
-    [Kaspersky Lab's Global Research & Analysis Team. (2016, August 9). The ProjectSauron APT. Technical Analysis. Retrieved August 17, 2016.](https://securelist.com/files/2016/07/The-ProjectSauron-APT_Technical_Analysis_KL.pdf)
-    [Livelli, K, et al. (2018, November 12). Operation Shaheen. Retrieved May 1, 2019.](https://www.cylance.com/content/dam/cylance-web/en-us/resources/knowledge-center/resource-library/reports/WhiteCompanyOperationShaheenReport.pdf?_ga=2.161661948.1943296560.1555683782-1066572390.1555511517)
-    [Gannon, M. (2019, February 11). With Upgrades in Delivery and Support Infrastructure, Revenge RAT Malware is a Bigger Threat. Retrieved November 17, 2024.](https://web.archive.org/web/20200428173819/https://cofense.com/upgrades-delivery-support-infrastructure-revenge-rat-malware-bigger-threat/)
-    [Mercer, W., Rascagneres, P. (2017, April 03). Introducing ROKRAT. Retrieved May 21, 2018.](https://blog.talosintelligence.com/2017/04/introducing-rokrat.html)
-    [Cash, D., Grunzweig, J., Adair, S., Lancaster, T. (2021, August 25). North Korean BLUELIGHT Special: InkySquid Deploys RokRAT. Retrieved October 1, 2021.](https://www.volexity.com/blog/2021/08/24/north-korean-bluelight-special-inkysquid-deploys-rokrat/)
-    [Ray, V., Hayashi, K. (2016, February 29). New Malware ‘Rover’ Targets Indian Ambassador to Afghanistan. Retrieved February 29, 2016.](http://researchcenter.paloaltonetworks.com/2016/02/new-malware-rover-targets-indian-ambassador-to-afghanistan/)
-    [Faou, M. and Boutin, J. (2017, February). Read The Manual: A Guide to the RTM Banking Trojan. Retrieved March 9, 2017.](https://www.welivesecurity.com/wp-content/uploads/2017/02/Read-The-Manual.pdf)
-    [Duncan, B., Harbison, M. (2019, January 23). Russian Language Malspam Pushing Redaman Banking Malware. Retrieved June 16, 2020.](https://unit42.paloaltonetworks.com/russian-language-malspam-pushing-redaman-banking-malware/)
-    [Sherstobitoff, R., Saavedra-Morales, J. (2018, February 02). Gold Dragon Widens Olympics Malware Attacks, Gains Permanent Presence on Victims’ Systems. Retrieved June 6, 2018.](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/gold-dragon-widens-olympics-malware-attacks-gains-permanent-presence-on-victims-systems/)
-    [Cherepanov, A.. (2016, December 13). The rise of TeleBots: Analyzing disruptive KillDisk attacks. Retrieved June 10, 2020.](https://www.welivesecurity.com/2016/12/13/rise-telebots-analyzing-disruptive-killdisk-attacks/)
-    [Salvati, M. (2019, August 6). SILENTTRINITY Modules. Retrieved March 24, 2022.](https://github.com/byt3bl33d3r/SILENTTRINITY/tree/master/silenttrinity/core/teamserver/modules/boo)
-    [DHS/CISA, Cyber National Mission Force. (2020, October 1). Malware Analysis Report (MAR) MAR-10303705-1.v1 – Remote Access Trojan: SLOTHFULMEDIA. Retrieved October 2, 2020.](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-275a)
-    [FireEye. (2021, May 11). Shining a Light on DARKSIDE Ransomware Operations. Retrieved September 22, 2021.](https://www.fireeye.com/blog/threat-research/2021/05/shining-a-light-on-darkside-ransomware-operations.html)
-    [FireEye. (2021, June 16). Smoking Out a DARKSIDE Affiliate’s Supply Chain Software Compromise. Retrieved September 22, 2021.](https://www.fireeye.com/blog/threat-research/2021/06/darkside-affiliate-supply-chain-software-compromise.html)
-    [Symantec Security Response. (2017, November 7). Sowbug: Cyber espionage group targets South American and Southeast Asian governments. Retrieved November 16, 2017.](https://www.symantec.com/connect/blogs/sowbug-cyber-espionage-group-targets-south-american-and-southeast-asian-governments)
-    [Baumgartner, K., Golovkin, M.. (2015, May). The MsnMM Campaigns: The Earliest Naikon APT Campaigns. Retrieved April 10, 2019.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07205555/TheNaikonAPT-MsnMM1.pdf)
-    [Blasco, J. (2012, January 12). Sykipot variant hijacks DOD and Windows smart cards. Retrieved January 10, 2016.](https://www.alienvault.com/open-threat-exchange/blog/sykipot-variant-hijacks-dod-and-windows-smart-cards)
-    [GReAT. (2019, April 10). Project TajMahal – a sophisticated new APT framework. Retrieved October 14, 2019.](https://securelist.com/project-tajmahal/90240/)
-    [Gabrielle Joyce Mabutas, Luis Magisa, Steven Du. (2020, July 17). Updates on Quickly-Evolving ThiefQuest macOS Malware. Retrieved April 26, 2021.](https://www.trendmicro.com/en_us/research/20/g/updates-on-quickly-evolving-thiefquest-macos-malware.html)
-    [Khandelwal, S. (2018, June 14). Chinese Hackers Carried Out Country-Level Watering Hole Attack. Retrieved August 18, 2018.](https://thehackernews.com/2018/06/chinese-watering-hole-attack.html)
-    [Legezo, D. (2018, June 13). LuckyMouse hits national data center to organize country-level waterholing campaign. Retrieved August 18, 2018.](https://securelist.com/luckymouse-hits-national-data-center/86083/)
-    [Cylance. (2014, December). Operation Cleaver. Retrieved September 14, 2017.](https://web.archive.org/web/20200302085133/https://www.cylance.com/content/dam/cylance/pages/operation-cleaver/Cylance_Operation_Cleaver_Report.pdf)
-    [Lior Rochberger, Tom Fakterman, Robert Falcone. (2023, September 22). Cyberespionage Attacks Against Southeast Asian Government Linked to Stately Taurus, Aka Mustang Panda. Retrieved September 9, 2025.](https://unit42.paloaltonetworks.com/stately-taurus-attacks-se-asian-government/)
-    [Daniel Lughi, Jaromir Horejsi. (2020, October 2). Tonto Team - Exploring the TTPs of an advanced threat actor operating a large infrastructure. Retrieved October 17, 2021.](https://vb2020.vblocalhost.com/uploads/VB2020-06.pdf)
-    [Secureworks. (2019, July 24). Updated Karagany Malware Targets Energy Sector. Retrieved August 12, 2020.](https://www.secureworks.com/research/updated-karagany-malware-targets-energy-sector)
-    [Lancaster, T., Cortes, J. (2018, January 29). VERMIN: Quasar RAT and Custom Malware Used In Ukraine. Retrieved July 5, 2018.](https://researchcenter.paloaltonetworks.com/2018/01/unit42-vermin-quasar-rat-custom-malware-used-ukraine/)
-    [CISA et al.. (2024, February 7). PRC State-Sponsored Actors Compromise and Maintain Persistent Access to U.S. Critical Infrastructure. Retrieved May 15, 2024.](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)
-    [Harakhavik, Y. (2020, February 3). Warzone: Behind the enemy lines. Retrieved December 17, 2021.](https://research.checkpoint.com/2020/warzone-behind-the-enemy-lines/)
-    [Mohanta, A. (2020, November 25). Warzone RAT comes with UAC bypass technique. Retrieved April 7, 2022.](https://www.uptycs.com/blog/warzone-rat-comes-with-uac-bypass-technique)
-    [Robert Falcone. (2017, February 14). XAgentOSX: Sofacy's Xagent macOS Tool. Retrieved July 12, 2017.](https://researchcenter.paloaltonetworks.com/2017/02/unit42-xagentosx-sofacys-xagent-macos-tool/)
-    [Nart Villeneuve, Randi Eitzman, Sandor Nemes & Tyler Dean, Google Cloud. (2017, October 5). Significant FormBook Distribution Campaigns Impacting the U.S. and South Korea. Retrieved March 11, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/formbook-malware-distribution-campaigns/)
-    [Schwarz, D., Sopko J. (2018, March 08). Donot Team Leverages New Modular Malware Framework in South Asia. Retrieved June 11, 2018.](https://www.arbornetworks.com/blog/asert/donot-team-leverages-new-modular-malware-framework-south-asia/)
-    [Ebach, L. (2017, June 22). Analysis Results of Zeus.Variant.Panda. Retrieved November 5, 2018.](https://cyberwtf.files.wordpress.com/2017/07/panda-whitepaper.pdf)
-    [Allievi, A., et al. (2014, October 28). Threat Spotlight: Group 72, Opening the ZxShell. Retrieved September 24, 2019.](https://blogs.cisco.com/security/talos/opening-zxshell)

×
