---
url: "https://attack.mitre.org/techniques/T1113/"
title: "Screen Capture, Technique T1113 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:35:01Z"
via: "live"
sha256: "324e0fc3a251aa6be0d3656b2e2067d50debad538331d1b85c17aaebe2e9e194"
wayback: "https://web.archive.org/web/20261019131351/https://attack.mitre.org/techniques/T1113/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Screen Capture, Technique T1113 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1113/>
> Snapshot: <https://web.archive.org/web/20261019131351/https://attack.mitre.org/techniques/T1113/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Screen Capture

#  Screen Capture

Adversaries may attempt to take screen captures of the desktop to gather information over the course of an operation. Screen capturing functionality may be included as a feature of a remote access tool used in post-compromise operations. Taking a screenshot is also typically possible through native utilities or API calls, such as CopyFromScreen, xwd, or screencapture.[[1]](https://docs.microsoft.com/en-us/dotnet/api/system.drawing.graphics.copyfromscreen?view=netframework-4.8)[[2]](https://blog.malwarebytes.com/threat-analysis/2017/01/new-mac-backdoor-using-antiquated-code/)

ID: T1113

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Collection](https://attack.mitre.org/tactics/TA0009)

ⓘ

Platforms: Linux, Windows, macOS

Version: 1.1

Created: 31 May 2017

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1113/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1113/)

## Procedure Examples

| ID  | Name  | Description

|  [C0063](https://attack.mitre.org/campaigns/C0063)   |  [2025 Poland Wiper Attacks](https://attack.mitre.org/campaigns/C0063)   |

During the [2025 Poland Wiper Attacks](https://attack.mitre.org/campaigns/C0063), the adversaries captured screenshots of devices using nircmd console through the command nircmd.exe "savescreenshot C:\Windows\Temp\imagetmp.png.[[3]](https://cert.pl/uploads/docs/CERT_Polska_Energy_Sector_Incident_Report_2025.pdf)

|  [S0331](https://attack.mitre.org/software/S0331)   |  [Agent Tesla](https://attack.mitre.org/software/S0331)   |

[Agent Tesla](https://attack.mitre.org/software/S0331) can capture screenshots of the victim’s desktop.[[4]](https://blog.talosintelligence.com/2018/10/old-dog-new-tricks-analysing-new-rtf_15.html)[[5]](https://www.digitrustgroup.com/agent-tesla-keylogger/)[[6]](https://www.fortinet.com/blog/threat-research/analysis-of-new-agent-tesla-spyware-variant.html)[[7]](https://www.fortinet.com/blog/threat-research/in-depth-analysis-of-net-malware-javaupdtr.html)[[8]](https://labs.bitdefender.com/2020/04/oil-gas-spearphishing-campaigns-drop-agent-tesla-spyware-in-advance-of-historic-opec-deal/)

|  [S0622](https://attack.mitre.org/software/S0622)   |  [AppleSeed](https://attack.mitre.org/software/S0622)   |

[AppleSeed](https://attack.mitre.org/software/S0622) can take screenshots on a compromised host by calling a series of APIs.[[9]](https://blog.malwarebytes.com/threat-analysis/2021/06/kimsuky-apt-continues-to-target-south-korean-government-using-appleseed-backdoor/)[[10]](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[APT28](https://attack.mitre.org/groups/G0007) has used tools to take screenshots from victims.[[11]](http://www.welivesecurity.com/wp-content/uploads/2016/10/eset-sednit-part-2.pdf)[[12]](https://researchcenter.paloaltonetworks.com/2017/02/unit42-xagentosx-sofacys-xagent-macos-tool/)[[13]](https://cdn.cnn.com/cnn/2018/images/07/13/gru.indictment.pdf)[[14]](https://www.secureworks.com/research/iron-twilight-supports-active-measures)

|  [G0087](https://attack.mitre.org/groups/G0087)   |  [APT39](https://attack.mitre.org/groups/G0087)   |

[APT39](https://attack.mitre.org/groups/G0087) has used a screen capture utility to take screenshots on a compromised host.[[15]](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/chafer-latest-attacks-reveal-heightened-ambitions)[[16]](https://www.iranwatch.org/sites/default/files/public-intelligence-alert.pdf)

|  [G1044](https://attack.mitre.org/groups/G1044)   |  [APT42](https://attack.mitre.org/groups/G1044)   |

[APT42](https://attack.mitre.org/groups/G1044) has used malware, such as GHAMBAR and POWERPOST, to take screenshots.[[17]](https://services.google.com/fh/files/misc/apt42-crooked-charms-cons-and-compromises.pdf)

|  [S0456](https://attack.mitre.org/software/S0456)   |  [Aria-body](https://attack.mitre.org/software/S0456)   |

[Aria-body](https://attack.mitre.org/software/S0456) has the ability to capture screenshots on compromised hosts.[[18]](https://research.checkpoint.com/2020/naikon-apt-cyber-espionage-reloaded/)

|  [S9031](https://attack.mitre.org/software/S9031)   |  [AshTag](https://attack.mitre.org/software/S9031)   |

The [AshTag](https://attack.mitre.org/software/S9031) AshenOrchestrator component has the ability to take screenshots.[[19]](https://unit42.paloaltonetworks.com/hamas-affiliate-ashen-lepus-uses-new-malware-suite-ashtag/)

|  [S1087](https://attack.mitre.org/software/S1087)   |  [AsyncRAT](https://attack.mitre.org/software/S1087)   |

[AsyncRAT](https://attack.mitre.org/software/S1087) has the ability to view the screen on compromised hosts.[[20]](https://github.com/NYAN-x-CAT/AsyncRAT-C-Sharp/blob/master/README.md)

|  [S0438](https://attack.mitre.org/software/S0438)   |  [Attor](https://attack.mitre.org/software/S0438)   |

[Attor](https://attack.mitre.org/software/S0438)'s has a plugin that captures screenshots of the target applications.[[21]](https://www.welivesecurity.com/wp-content/uploads/2019/10/ESET_Attor.pdf)

|  [S0344](https://attack.mitre.org/software/S0344)   |  [Azorult](https://attack.mitre.org/software/S0344)   |

[Azorult](https://attack.mitre.org/software/S0344) can capture screenshots of the victim’s machines.[[22]](https://researchcenter.paloaltonetworks.com/2018/11/unit42-new-wine-old-bottle-new-azorult-variant-found-findmyname-campaign-using-fallout-exploit-kit/)

|  [S1081](https://attack.mitre.org/software/S1081)   |  [BADHATCH](https://attack.mitre.org/software/S1081)   |

[BADHATCH](https://attack.mitre.org/software/S1081) can take screenshots and send them to an actor-controlled C2 server.[[23]](https://www.bitdefender.com/files/News/CaseStudies/study/394/Bitdefender-PR-Whitepaper-BADHATCH-creat5237-en-EN.pdf)

|  [S0128](https://attack.mitre.org/software/S0128)   |  [BADNEWS](https://attack.mitre.org/software/S0128)   |

[BADNEWS](https://attack.mitre.org/software/S0128) has a command to take a screenshot and send it to the C2 server.[[24]](https://www.forcepoint.com/sites/default/files/resources/files/forcepoint-security-labs-monsoon-analysis-report.pdf)[[25]](https://researchcenter.paloaltonetworks.com/2018/03/unit42-patchwork-continues-deliver-badnews-indian-subcontinent/)

|  [S0337](https://attack.mitre.org/software/S0337)   |  [BadPatch](https://attack.mitre.org/software/S0337)   |

[BadPatch](https://attack.mitre.org/software/S0337) captures screenshots in .jpg format and then exfiltrates them.[[26]](https://researchcenter.paloaltonetworks.com/2017/10/unit42-badpatch/)

|  [S0234](https://attack.mitre.org/software/S0234)   |  [Bandook](https://attack.mitre.org/software/S0234)   |

[Bandook](https://attack.mitre.org/software/S0234) is capable of taking an image of and uploading the current desktop.[[27]](https://info.lookout.com/rs/051-ESQ-475/images/Lookout_Dark-Caracal_srr_20180118_us_v.1.0.pdf)[[28]](https://research.checkpoint.com/2020/bandook-signed-delivered/)

|  [S0017](https://attack.mitre.org/software/S0017)   |  [BISCUIT](https://attack.mitre.org/software/S0017)   |

[BISCUIT](https://attack.mitre.org/software/S0017) has a command to periodically take screenshots of the system.[[29]](https://www.mandiant.com/sites/default/files/2021-09/mandiant-apt1-report.pdf)

|  [S0089](https://attack.mitre.org/software/S0089)   |  [BlackEnergy](https://attack.mitre.org/software/S0089)   |

[BlackEnergy](https://attack.mitre.org/software/S0089) is capable of taking screenshots.[[30]](https://securelist.com/be2-custom-plugins-router-abuse-and-target-profiles/67353/)

|  [S0657](https://attack.mitre.org/software/S0657)   |  [BLUELIGHT](https://attack.mitre.org/software/S0657)   |

[BLUELIGHT](https://attack.mitre.org/software/S0657) has captured a screenshot of the display every 30 seconds for the first 5 minutes after initiating a C2 loop, and then once every five minutes thereafter.[[31]](https://www.volexity.com/blog/2021/08/17/north-korean-apt-inkysquid-infects-victims-using-browser-exploits/)

|  [G0060](https://attack.mitre.org/groups/G0060)   |  [BRONZE BUTLER](https://attack.mitre.org/groups/G0060)   |

[BRONZE BUTLER](https://attack.mitre.org/groups/G0060) has used a tool to capture screenshots.[[32]](https://www.secureworks.com/research/bronze-butler-targets-japanese-businesses)[[33]](https://documents.trendmicro.com/assets/pdf/Operation-ENDTRADE-TICK-s-Multi-Stage-Backdoors-for-Attacking-Industries-and-Stealing-Classified-Data.pdf)

|  [S1063](https://attack.mitre.org/software/S1063)   |  [Brute Ratel C4](https://attack.mitre.org/software/S1063)   |

[Brute Ratel C4](https://attack.mitre.org/software/S1063) can take screenshots on compromised hosts.[[34]](https://unit42.paloaltonetworks.com/brute-ratel-c4-tool/)

|  [S0454](https://attack.mitre.org/software/S0454)   |  [Cadelspy](https://attack.mitre.org/software/S0454)   |

[Cadelspy](https://attack.mitre.org/software/S0454) has the ability to capture screenshots and webcam photos.[[35]](https://www.symantec.com/connect/blogs/iran-based-attackers-use-back-door-threats-spy-middle-eastern-targets)

|  [S0351](https://attack.mitre.org/software/S0351)   |  [Cannon](https://attack.mitre.org/software/S0351)   |

[Cannon](https://attack.mitre.org/software/S0351) can take a screenshot of the desktop.[[36]](https://researchcenter.paloaltonetworks.com/2018/11/unit42-sofacy-continues-global-attacks-wheels-new-cannon-trojan/)

|  [S0030](https://attack.mitre.org/software/S0030)   |  [Carbanak](https://attack.mitre.org/software/S0030)   |

[Carbanak](https://attack.mitre.org/software/S0030) performs desktop video recording and captures screenshots of the desktop and sends it to the C2 server.[[37]](https://www.fireeye.com/blog/threat-research/2017/06/behind-the-carbanak-backdoor.html)

|  [S0484](https://attack.mitre.org/software/S0484)   |  [Carberp](https://attack.mitre.org/software/S0484)   |

[Carberp](https://attack.mitre.org/software/S0484) can capture display screenshots with the screens_dll.dll plugin.[[38]](https://web.archive.org/web/20231227000328/http://pxnow.prevx.com/content/blog/carberp-a_modular_information_stealing_trojan.pdf)

|  [S0348](https://attack.mitre.org/software/S0348)   |  [Cardinal RAT](https://attack.mitre.org/software/S0348)   |

[Cardinal RAT](https://attack.mitre.org/software/S0348) can capture screenshots.[[39]](https://researchcenter.paloaltonetworks.com/2017/04/unit42-cardinal-rat-active-two-years/)

|  [S0261](https://attack.mitre.org/software/S0261)   |  [Catchamas](https://attack.mitre.org/software/S0261)   |

[Catchamas](https://attack.mitre.org/software/S0261) captures screenshots based on specific keywords in the window’s title.[[40]](https://web.archive.org/web/20190508165711/https://www-west.symantec.com/content/symantec/english/en/security-center/writeup.html/2018-040209-1742-99)

|  [S0631](https://attack.mitre.org/software/S0631)   |  [Chaes](https://attack.mitre.org/software/S0631)   |

[Chaes](https://attack.mitre.org/software/S0631) can capture screenshots of the infected machine.[[41]](https://www.cybereason.com/hubfs/dam/collateral/reports/11-2020-Chaes-e-commerce-malware-research.pdf)

|  [S0674](https://attack.mitre.org/software/S0674)   |  [CharmPower](https://attack.mitre.org/software/S0674)   |

[CharmPower](https://attack.mitre.org/software/S0674) has the ability to capture screenshots.[[42]](https://research.checkpoint.com/2022/apt35-exploits-log4j-vulnerability-to-distribute-new-modular-powershell-toolkit/)

|  [S1149](https://attack.mitre.org/software/S1149)   |  [CHIMNEYSWEEP](https://attack.mitre.org/software/S1149)   |

[CHIMNEYSWEEP](https://attack.mitre.org/software/S1149) can capture screenshots on targeted systems using a timer and either upload them or store them to disk.[[43]](https://cloud.google.com/blog/topics/threat-intelligence/likely-iranian-threat-actor-conducts-politically-motivated-disruptive-activity-against/)

|  [S0023](https://attack.mitre.org/software/S0023)   |  [CHOPSTICK](https://attack.mitre.org/software/S0023)   |

[CHOPSTICK](https://attack.mitre.org/software/S0023) has the capability to capture screenshots.[[13]](https://cdn.cnn.com/cnn/2018/images/07/13/gru.indictment.pdf)

|  [S0667](https://attack.mitre.org/software/S0667)   |  [Chrommme](https://attack.mitre.org/software/S0667)   |

[Chrommme](https://attack.mitre.org/software/S0667) has the ability to capture screenshots.[[44]](https://www.welivesecurity.com/wp-content/uploads/2021/06/eset_gelsemium.pdf)

|  [S0660](https://attack.mitre.org/software/S0660)   |  [Clambling](https://attack.mitre.org/software/S0660)   |

[Clambling](https://attack.mitre.org/software/S0660) has the ability to capture screenshots.[[45]](https://documents.trendmicro.com/assets/white_papers/wp-uncovering-DRBcontrol.pdf)

|  [S0154](https://attack.mitre.org/software/S0154)   |  [Cobalt Strike](https://attack.mitre.org/software/S0154)   |

[Cobalt Strike](https://attack.mitre.org/software/S0154)'s Beacon payload is capable of capturing screenshots.[[46]](https://web.archive.org/web/20210825130434/https://cobaltstrike.com/downloads/csmanual38.pdf)[[47]](https://www.amnestyusa.org/wp-content/uploads/2021/02/Click-and-Bait_Vietnamese-Human-Rights-Defenders-Targeted-with-Spyware-Attacks.pdf)[[48]](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)

|  [S0338](https://attack.mitre.org/software/S0338)   |  [Cobian RAT](https://attack.mitre.org/software/S0338)   |

[Cobian RAT](https://attack.mitre.org/software/S0338) has a feature to perform screen capture.[[49]](https://www.zscaler.com/blogs/research/cobian-rat-backdoored-rat)

|  [S0591](https://attack.mitre.org/software/S0591)   |  [ConnectWise](https://attack.mitre.org/software/S0591)   |

[ConnectWise](https://attack.mitre.org/software/S0591) can take screenshots on remote hosts.[[50]](https://www.anomali.com/blog/probable-iranian-cyber-actors-static-kitten-conducting-cyberespionage-campaign-targeting-uae-and-kuwait-government-agencies)

|  [S0050](https://attack.mitre.org/software/S0050)   |  [CosmicDuke](https://attack.mitre.org/software/S0050)   |

[CosmicDuke](https://attack.mitre.org/software/S0050) takes periodic screenshots and exfiltrates them.[[51]](https://blog.f-secure.com/wp-content/uploads/2019/10/CosmicDuke.pdf)

|  [S0115](https://attack.mitre.org/software/S0115)   |  [Crimson](https://attack.mitre.org/software/S0115)   |

[Crimson](https://attack.mitre.org/software/S0115) contains a command to perform screen captures.[[52]](https://www.proofpoint.com/sites/default/files/proofpoint-operation-transparent-tribe-threat-insight-en.pdf)[[53]](https://securelist.com/transparent-tribe-part-1/98127/)[[54]](https://blog.talosintelligence.com/2022/07/transparent-tribe-targets-education.html)

|  [S0235](https://attack.mitre.org/software/S0235)   |  [CrossRAT](https://attack.mitre.org/software/S0235)   |

[CrossRAT](https://attack.mitre.org/software/S0235) is capable of taking screen captures.[[27]](https://info.lookout.com/rs/051-ESQ-475/images/Lookout_Dark-Caracal_srr_20180118_us_v.1.0.pdf)

|  [S1153](https://attack.mitre.org/software/S1153)   |  [Cuckoo Stealer](https://attack.mitre.org/software/S1153)   |

[Cuckoo Stealer](https://attack.mitre.org/software/S1153) can run screencapture to collect screenshots from compromised hosts. [[55]](https://www.kandji.io/blog/malware-cuckoo-infostealer-spyware)

|  [G0070](https://attack.mitre.org/groups/G0070)   |  [Dark Caracal](https://attack.mitre.org/groups/G0070)   |

[Dark Caracal](https://attack.mitre.org/groups/G0070) took screenshots using their Windows malware.[[27]](https://info.lookout.com/rs/051-ESQ-475/images/Lookout_Dark-Caracal_srr_20180118_us_v.1.0.pdf)

|  [S0187](https://attack.mitre.org/software/S0187)   |  [Daserf](https://attack.mitre.org/software/S0187)   |

[Daserf](https://attack.mitre.org/software/S0187) can take screenshots.[[56]](http://blog.trendmicro.com/trendlabs-security-intelligence/redbaldknight-bronze-butler-daserf-backdoor-now-using-steganography/)[[32]](https://www.secureworks.com/research/bronze-butler-targets-japanese-businesses)

|  [S0021](https://attack.mitre.org/software/S0021)   |  [Derusbi](https://attack.mitre.org/software/S0021)   |

[Derusbi](https://attack.mitre.org/software/S0021) is capable of performing screen captures.[[57]](https://www.fireeye.com/blog/threat-research/2018/03/suspected-chinese-espionage-group-targeting-maritime-and-engineering-industries.html)

|  [S0213](https://attack.mitre.org/software/S0213)   |  [DOGCALL](https://attack.mitre.org/software/S0213)   |

[DOGCALL](https://attack.mitre.org/software/S0213) is capable of capturing screenshots of the victim's machine.[[58]](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)[[59]](https://researchcenter.paloaltonetworks.com/2018/10/unit42-nokki-almost-ties-the-knot-with-dogcall-reaper-group-uses-new-malware-to-deploy-rat/)

|  [G0035](https://attack.mitre.org/groups/G0035)   |  [Dragonfly](https://attack.mitre.org/groups/G0035)   |

[Dragonfly](https://attack.mitre.org/groups/G0035) has performed screen captures of victims, including by using a tool, scr.exe (which matched the hash of ScreenUtil).[[60]](https://www.us-cert.gov/ncas/alerts/TA18-074A)[[61]](https://docs.broadcom.com/doc/dragonfly_threat_against_western_energy_suppliers)[[62]](https://vblocalhost.com/uploads/VB2021-Slowik.pdf)

|  [S1159](https://attack.mitre.org/software/S1159)   |  [DUSTTRAP](https://attack.mitre.org/software/S1159)   |

[DUSTTRAP](https://attack.mitre.org/software/S1159) can capture screenshots.[[63]](https://cloud.google.com/blog/topics/threat-intelligence/apt41-arisen-from-dust)

|  [S0062](https://attack.mitre.org/software/S0062)   |  [DustySky](https://attack.mitre.org/software/S0062)   |

[DustySky](https://attack.mitre.org/software/S0062) captures PNG screenshots of the main screen.[[64]](https://securelist.com/gaza-cybergang-group1-operation-sneakypastes/90068/)

|  [S0593](https://attack.mitre.org/software/S0593)   |  [ECCENTRICBANDWAGON](https://attack.mitre.org/software/S0593)   |

[ECCENTRICBANDWAGON](https://attack.mitre.org/software/S0593) can capture screenshots and store them locally.[[65]](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-239a)

|  [S0363](https://attack.mitre.org/software/S0363)   |  [Empire](https://attack.mitre.org/software/S0363)   |

[Empire](https://attack.mitre.org/software/S0363) is capable of capturing screenshots on Windows and macOS systems.[[66]](https://github.com/PowerShellEmpire/Empire)

|  [S0152](https://attack.mitre.org/software/S0152)   |  [EvilGrab](https://attack.mitre.org/software/S0152)   |

[EvilGrab](https://attack.mitre.org/software/S0152) has the capability to capture screenshots.[[67]](https://www.pwc.co.uk/cyber-security/pdf/pwc-uk-operation-cloud-hopper-technical-annex-april-2017.pdf)

|  [G0046](https://attack.mitre.org/groups/G0046)   |  [FIN7](https://attack.mitre.org/groups/G0046)   |

[FIN7](https://attack.mitre.org/groups/G0046) captured screenshots and desktop video recordings.[[68]](https://www.justice.gov/opa/press-release/file/1084361/download)

|  [S0182](https://attack.mitre.org/software/S0182)   |  [FinFisher](https://attack.mitre.org/software/S0182)   |

[FinFisher](https://attack.mitre.org/software/S0182) takes a screenshot of the screen and displays it on top of all other windows for few seconds in an apparent attempt to hide some messages showed by the system during the setup process.[[69]](https://web.archive.org/web/20171222050934/http://www.finfisher.com/FinFisher/index.html)[[70]](https://cloudblogs.microsoft.com/microsoftsecure/2018/03/01/finfisher-exposed-a-researchers-tale-of-defeating-traps-tricks-and-complex-virtual-machines/)

|  [S0143](https://attack.mitre.org/software/S0143)   |  [Flame](https://attack.mitre.org/software/S0143)   |

[Flame](https://attack.mitre.org/software/S0143) can take regular screenshots when certain applications are open that are sent to the command and control server.[[71]](https://securelist.com/the-flame-questions-and-answers-51/34344/)

|  [S0381](https://attack.mitre.org/software/S0381)   |  [FlawedAmmyy](https://attack.mitre.org/software/S0381)   |

[FlawedAmmyy](https://attack.mitre.org/software/S0381) can capture screenshots.[[72]](https://www.fsec.or.kr/user/bbs/fsec/163/344/bbsDataView/1382.do?page=1&column=&search=&searchSDate=&searchEDate=&bbsDataCategory=)

|  [S0277](https://attack.mitre.org/software/S0277)   |  [FruitFly](https://attack.mitre.org/software/S0277)   |

[FruitFly](https://attack.mitre.org/software/S0277) takes screenshots of the user's desktop.[[73]](https://objective-see.com/blog/blog_0x25.html)

|  [S1044](https://attack.mitre.org/software/S1044)   |  [FunnyDream](https://attack.mitre.org/software/S1044)   |

The [FunnyDream](https://attack.mitre.org/software/S1044) ScreenCap component can take screenshots on a compromised host.[[74]](https://www.bitdefender.com/files/News/CaseStudies/study/379/Bitdefender-Whitepaper-Chinese-APT.pdf)

|  [G0047](https://attack.mitre.org/groups/G0047)   |  [Gamaredon Group](https://attack.mitre.org/groups/G0047)   |

[Gamaredon Group](https://attack.mitre.org/groups/G0047)'s malware can take screenshots of the compromised computer every minute.[[75]](https://www.welivesecurity.com/2020/06/11/gamaredon-group-grows-its-game/)[[76]](https://www.security.com/threat-intelligence/shuckworm-ukraine-gammasteel)[[77]](https://web-assets.esetstatic.com/wls/en/papers/white-papers/cyberespionage-gamaredon-way.pdf)

|  [S0032](https://attack.mitre.org/software/S0032)   |  [gh0st RAT](https://attack.mitre.org/software/S0032)   |

[gh0st RAT](https://attack.mitre.org/software/S0032) can capture the victim’s screen remotely.[[78]](https://research.nccgroup.com/2018/04/17/decoding-network-data-from-a-gh0st-rat-variant/)

|  [G0115](https://attack.mitre.org/groups/G0115)   |  [GOLD SOUTHFIELD](https://attack.mitre.org/groups/G0115)   |

[GOLD SOUTHFIELD](https://attack.mitre.org/groups/G0115) has used the remote monitoring and management tool ConnectWise to obtain screen captures from victim's machines.[[79]](https://web.archive.org/web/20210414101816/https://tetradefense.com/incident-response-services/cause-and-effect-sodinokibi-ransomware-analysis/)

|  [S0417](https://attack.mitre.org/software/S0417)   |  [GRIFFON](https://attack.mitre.org/software/S0417)   |

[GRIFFON](https://attack.mitre.org/software/S0417) has used a screenshot module that can be used to take a screenshot of the remote system.[[80]](https://securelist.com/fin7-5-the-infamous-cybercrime-rig-fin7-continues-its-activities/90703/)

|  [G0043](https://attack.mitre.org/groups/G0043)   |  [Group5](https://attack.mitre.org/groups/G0043)   |

Malware used by [Group5](https://attack.mitre.org/groups/G0043) is capable of watching the victim's screen.[[81]](https://citizenlab.ca/2016/08/group5-syria/)

|  [S0151](https://attack.mitre.org/software/S0151)   |  [HALFBAKED](https://attack.mitre.org/software/S0151)   |

[HALFBAKED](https://attack.mitre.org/software/S0151) can obtain screenshots from the victim.[[82]](https://www.fireeye.com/blog/threat-research/2017/04/fin7-phishing-lnk.html)

|  [S1229](https://attack.mitre.org/software/S1229)   |  [Havoc](https://attack.mitre.org/software/S1229)   |

[Havoc](https://attack.mitre.org/software/S1229) can capture screenshots.[[83]](https://havocframework.com/docs/welcome)[[84]](https://www.zscaler.com/blogs/security-research/havoc-across-cyberspace)[[85]](https://www.immersivelabs.com/resources/blog/havoc-c2-framework-a-defensive-operators-guide)

|  [S0431](https://attack.mitre.org/software/S0431)   |  [HotCroissant](https://attack.mitre.org/software/S0431)   |

[HotCroissant](https://attack.mitre.org/software/S0431) has the ability to do real time screen viewing on an infected host.[[86]](https://www.carbonblack.com/2020/04/16/vmware-carbon-black-tau-threat-analysis-the-evolution-of-lazarus/)

|  [S9007](https://attack.mitre.org/software/S9007)   |  [HTTPTroy](https://attack.mitre.org/software/S9007)   |

[HTTPTroy](https://attack.mitre.org/software/S9007) has obtained screen captures leveraging the screen command which captures, encrypts and uploads the stolen image to the adversary controlled C2 server.[[87]](https://www.gendigital.com/blog/insights/research/dprk-kimsuky-lazarus-analysis)

|  [S0203](https://attack.mitre.org/software/S0203)   |  [Hydraq](https://attack.mitre.org/software/S0203)   |

[Hydraq](https://attack.mitre.org/software/S0203) includes a component based on the code of VNC that can stream a live feed of the desktop of an infected host.[[88]](https://www.symantec.com/security_response/writeup.jsp?docid=2010-011114-1830-99)

|  [S0398](https://attack.mitre.org/software/S0398)   |  [HyperBro](https://attack.mitre.org/software/S0398)   |

[HyperBro](https://attack.mitre.org/software/S0398) has the ability to take screenshots.[[89]](https://unit42.paloaltonetworks.com/emissary-panda-attacks-middle-east-government-sharepoint-servers/)

|  [S0260](https://attack.mitre.org/software/S0260)   |  [InvisiMole](https://attack.mitre.org/software/S0260)   |

[InvisiMole](https://attack.mitre.org/software/S0260) can capture screenshots of not only the entire screen, but of each separate window open, in case they are overlapping.[[90]](https://www.welivesecurity.com/2018/06/07/invisimole-equipped-spyware-undercover/)[[91]](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_InvisiMole.pdf)

|  [S0163](https://attack.mitre.org/software/S0163)   |  [Janicab](https://attack.mitre.org/software/S0163)   |

[Janicab](https://attack.mitre.org/software/S0163) captured screenshots and sent them out to a C2 server.[[92]](https://www.f-secure.com/weblog/archives/00002576.html)[[93]](https://web.archive.org/web/20230331162455/https://www.thesafemac.com/new-signed-malware-called-janicab/)

|  [S0044](https://attack.mitre.org/software/S0044)   |  [JHUHUGIT](https://attack.mitre.org/software/S0044)   |

A [JHUHUGIT](https://attack.mitre.org/software/S0044) variant takes screenshots by simulating the user pressing the "Take Screenshot" key (VK_SCREENSHOT), accessing the screenshot saved in the clipboard, and converting it to a JPG image.[[94]](https://web.archive.org/web/20230702043312/https://pan-unit42.github.io/playbook_viewer/)[[95]](https://blog.talosintelligence.com/2017/10/cyber-conflict-decoy-document.html)

|  [S0283](https://attack.mitre.org/software/S0283)   |  [jRAT](https://attack.mitre.org/software/S0283)   |

[jRAT](https://attack.mitre.org/software/S0283) has the capability to take screenshots of the victim’s machine.[[96]](https://www.symantec.com/blogs/threat-intelligence/jrat-new-anti-parsing-techniques)[[97]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07195002/KL_AdwindPublicReport_2016.pdf)

|  [S0088](https://attack.mitre.org/software/S0088)   |  [Kasidet](https://attack.mitre.org/software/S0088)   |

[Kasidet](https://attack.mitre.org/software/S0088) has the ability to initiate keylogging and screen captures.[[98]](http://research.zscaler.com/2016/01/malicious-office-files-dropping-kasidet.html)

|  [S0265](https://attack.mitre.org/software/S0265)   |  [Kazuar](https://attack.mitre.org/software/S0265)   |

[Kazuar](https://attack.mitre.org/software/S0265) captures screenshots of the victim’s screen.[[99]](https://researchcenter.paloaltonetworks.com/2017/05/unit42-kazuar-multiplatform-espionage-backdoor-api-access/)

|  [S0387](https://attack.mitre.org/software/S0387)   |  [KeyBoy](https://attack.mitre.org/software/S0387)   |

[KeyBoy](https://attack.mitre.org/software/S0387) has a command to perform screen grabbing.[[100]](https://web.archive.org/web/20211129064701/https://www.pwc.co.uk/issues/cyber-security-services/research/the-keyboys-are-back-in-town.html)

|  [S0271](https://attack.mitre.org/software/S0271)   |  [KEYMARBLE](https://attack.mitre.org/software/S0271)   |

[KEYMARBLE](https://attack.mitre.org/software/S0271) can capture screenshots of the victim’s machine.[[101]](https://www.us-cert.gov/ncas/analysis-reports/AR18-221A)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has captured browser screenshots using [TRANSLATEXT](https://attack.mitre.org/software/S1201).[[102]](https://www.zscaler.com/blogs/security-research/kimsuky-deploys-translatext-target-south-korean-academia#technical-analysis) [Kimsuky](https://attack.mitre.org/groups/G0094) has also obtained screen captures with custom malware.[[87]](https://www.gendigital.com/blog/insights/research/dprk-kimsuky-lazarus-analysis)

|  [S0437](https://attack.mitre.org/software/S0437)   |  [Kivars](https://attack.mitre.org/software/S0437)   |

[Kivars](https://attack.mitre.org/software/S0437) has the ability to capture screenshots on the infected host.[[103]](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)

|  [S0356](https://attack.mitre.org/software/S0356)   |  [KONNI](https://attack.mitre.org/software/S0356)   |

[KONNI](https://attack.mitre.org/software/S0356) can take screenshots of the victim’s machine.[[104]](https://blog.talosintelligence.com/2017/05/konni-malware-under-radar-for-years.html)

|  [S1185](https://attack.mitre.org/software/S1185)   |  [LightSpy](https://attack.mitre.org/software/S1185)   |

[LightSpy](https://attack.mitre.org/software/S1185) uses Apple's built-in AVFoundation Framework library to access the user's camera and screen. It uses the AVCaptureStillImage to take a picture using the user's camera and the AVCaptureScreen to take a screenshot or record the user's screen for a specified period of time.[[105]](https://www.huntress.com/blog/lightspy-malware-variant-targeting-macos)

|  [S0680](https://attack.mitre.org/software/S0680)   |  [LitePower](https://attack.mitre.org/software/S0680)   |

[LitePower](https://attack.mitre.org/software/S0680) can take system screenshots and save them to %AppData%.[[106]](https://securelist.com/wirtes-campaign-in-the-middle-east-living-off-the-land-since-at-least-2019/105044)

|  [S0681](https://attack.mitre.org/software/S0681)   |  [Lizar](https://attack.mitre.org/software/S0681)   |

[Lizar](https://attack.mitre.org/software/S0681) can take JPEG screenshots of an infected system.[[107]](https://threatpost.com/fin7-backdoor-ethical-hacking-tool/166194/)[[108]](https://bi-zone.medium.com/from-pentest-to-apt-attack-cybercriminal-group-fin7-disguises-its-malware-as-an-ethical-hackers-c23c9a75e319) [Lizar](https://attack.mitre.org/software/S0681) has also used a plugin to take a screenshot of the infected system.[[108]](https://bi-zone.medium.com/from-pentest-to-apt-attack-cybercriminal-group-fin7-disguises-its-malware-as-an-ethical-hackers-c23c9a75e319)

|  [S9020](https://attack.mitre.org/software/S9020)   |  [LODEINFO](https://attack.mitre.org/software/S9020)   |

[LODEINFO](https://attack.mitre.org/software/S9020) has the ability to take screenshots.[[109]](https://securelist.com/apt10-tracking-down-lodeinfo-2022-part-ii/107745/)[[110]](https://www.welivesecurity.com/2022/12/14/unmasking-mirrorface-operation-liberalface-targeting-japanese-political-entities/)[[111]](https://blog-en.itochuci.co.jp/entry/2024/01/24/134100)

|  [S0582](https://attack.mitre.org/software/S0582)   |  [LookBack](https://attack.mitre.org/software/S0582)   |

[LookBack](https://attack.mitre.org/software/S0582) can take desktop screenshots.[[112]](https://www.proofpoint.com/us/threat-insight/post/lookback-malware-targets-united-states-utilities-sector-phishing-attacks)

|  [S1213](https://attack.mitre.org/software/S1213)   |  [Lumma Stealer](https://attack.mitre.org/software/S1213)   |

[Lumma Stealer](https://attack.mitre.org/software/S1213) has taken screenshots of victim machines.[[113]](https://www.cybereason.com/blog/threat-analysis-rise-of-lummastealer)

|  [S1142](https://attack.mitre.org/software/S1142)   |  [LunarMail](https://attack.mitre.org/software/S1142)   |

[LunarMail](https://attack.mitre.org/software/S1142) can capture screenshots from compromised hosts.[[114]](https://www.welivesecurity.com/en/eset-research/moon-backdoors-lunar-landing-diplomatic-missions/)

|  [S0409](https://attack.mitre.org/software/S0409)   |  [Machete](https://attack.mitre.org/software/S0409)   |

[Machete](https://attack.mitre.org/software/S0409) captures screenshots.[[115]](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)[[116]](https://securelist.com/el-machete/66108/)[[117]](https://threatvector.cylance.com/en_us/home/el-machete-malware-attacks-cut-through-latam.html)[[118]](https://blog.360totalsecurity.com/en/apt-c-43-steals-venezuelan-military-secrets-to-provide-intelligence-support-for-the-reactionaries-hpreact-campaign/)

|  [S1016](https://attack.mitre.org/software/S1016)   |  [MacMa](https://attack.mitre.org/software/S1016)   |

[MacMa](https://attack.mitre.org/software/S1016) has used Apple’s Core Graphic APIs, such as CGWindowListCreateImageFromArray, to capture the user's screen and open windows.[[119]](https://www.welivesecurity.com/2022/01/25/watering-hole-deploys-new-macos-malware-dazzlespy-asia/)[[120]](https://objective-see.org/blog/blog_0x69.html)

|  [S0282](https://attack.mitre.org/software/S0282)   |  [MacSpy](https://attack.mitre.org/software/S0282)   |

[MacSpy](https://attack.mitre.org/software/S0282) can capture screenshots of the desktop over multiple monitors.[[73]](https://objective-see.com/blog/blog_0x25.html)

|  [S1060](https://attack.mitre.org/software/S1060)   |  [Mafalda](https://attack.mitre.org/software/S1060)   |

[Mafalda](https://attack.mitre.org/software/S1060) can take a screenshot of the target machine and save it to a file.[[121]](https://assets.sentinelone.com/sentinellabs22/metador#page=1)

|  [G0059](https://attack.mitre.org/groups/G0059)   |  [Magic Hound](https://attack.mitre.org/groups/G0059)   |

[Magic Hound](https://attack.mitre.org/groups/G0059) malware can take a screenshot and upload the file to its C2 server.[[122]](https://researchcenter.paloaltonetworks.com/2017/02/unit42-magic-hound-campaign-attacks-saudi-targets/)

|  [S1156](https://attack.mitre.org/software/S1156)   |  [Manjusaka](https://attack.mitre.org/software/S1156)   |

[Manjusaka](https://attack.mitre.org/software/S1156) can take screenshots of the victim desktop.[[123]](https://blog.talosintelligence.com/manjusaka-offensive-framework/)

|  [S0652](https://attack.mitre.org/software/S0652)   |  [MarkiRAT](https://attack.mitre.org/software/S0652)   |

[MarkiRAT](https://attack.mitre.org/software/S0652) can capture screenshots that are initially saved as ‘scr.jpg’.[[124]](https://securelist.com/ferocious-kitten-6-years-of-covert-surveillance-in-iran/102806/)

|  [S0167](https://attack.mitre.org/software/S0167)   |  [Matryoshka](https://attack.mitre.org/software/S0167)   |

[Matryoshka](https://attack.mitre.org/software/S0167) is capable of performing screen captures.[[125]](http://www.clearskysec.com/wp-content/uploads/2017/07/Operation_Wilted_Tulip.pdf)[[126]](https://cdn2.hubspot.net/hubfs/1903456/Whitepapers/CopyKittens.pdf)

|  [S1059](https://attack.mitre.org/software/S1059)   |  [metaMain](https://attack.mitre.org/software/S1059)   |

[metaMain](https://attack.mitre.org/software/S1059) can take and save screenshots.[[121]](https://assets.sentinelone.com/sentinellabs22/metador#page=1)[[127]](https://docs.google.com/document/d/1e9ZTW9b71YwFWS_18ZwDAxa-cYbV8q1wUefmKZLYVsA/edit#heading=h.lmnbtht1ikzm)

|  [S0455](https://attack.mitre.org/software/S0455)   |  [Metamorfo](https://attack.mitre.org/software/S0455)   |

[Metamorfo](https://attack.mitre.org/software/S0455) can collect screenshots of the victim’s machine.[[128]](https://www.fireeye.com/blog/threat-research/2018/04/metamorfo-campaign-targeting-brazilian-users.html)[[129]](https://www.welivesecurity.com/2019/10/03/casbaneiro-trojan-dangerous-cooking/)

|  [S0339](https://attack.mitre.org/software/S0339)   |  [Micropsia](https://attack.mitre.org/software/S0339)   |

[Micropsia](https://attack.mitre.org/software/S0339) takes screenshots every 90 seconds by calling the Gdi32.BitBlt API.[[130]](https://www.radware.com/blog/security/2018/07/micropsia-malware/)

|  [S1122](https://attack.mitre.org/software/S1122)   |  [Mispadu](https://attack.mitre.org/software/S1122)   |

[Mispadu](https://attack.mitre.org/software/S1122) has the ability to capture screenshots on compromised hosts.[[131]](https://blog.scilabs.mx/en/cyber-threat-profile-malteiro/)[[132]](https://blog.scilabs.mx/en/evolution-of-banking-trojan-ursa-mispadu/)[[133]](https://www.welivesecurity.com/2019/11/19/mispadu-advertisement-discounted-unhappy-meal/)[[134]](https://www.metabaseq.com/mispadu-banking-trojan/)

|  [G1019](https://attack.mitre.org/groups/G1019)   |  [MoustachedBouncer](https://attack.mitre.org/groups/G1019)   |

[MoustachedBouncer](https://attack.mitre.org/groups/G1019) has used plugins to take screenshots on targeted systems.[[135]](https://www.welivesecurity.com/en/eset-research/moustachedbouncer-espionage-against-foreign-diplomats-in-belarus/)

|  [G0069](https://attack.mitre.org/groups/G0069)   |  [MuddyWater](https://attack.mitre.org/groups/G0069)   |

[MuddyWater](https://attack.mitre.org/groups/G0069) has used malware that can capture screenshots of the victim’s machine.[[136]](https://securelist.com/muddywater/88059/)

|  [S0198](https://attack.mitre.org/software/S0198)   |  [NETWIRE](https://attack.mitre.org/software/S0198)   |

[NETWIRE](https://attack.mitre.org/software/S0198) can capture the victim's screen.[[137]](https://securingtomorrow.mcafee.com/mcafee-labs/netwire-rat-behind-recent-targeted-attacks/)[[138]](https://www.mandiant.com/resources/blog/dissecting-netwire-phishing-campaigns-usage-process-hollowing)[[139]](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)[[140]](https://www.proofpoint.com/us/blog/threat-insight/geofenced-netwire-campaigns)

|  [S1090](https://attack.mitre.org/software/S1090)   |  [NightClub](https://attack.mitre.org/software/S1090)   |

[NightClub](https://attack.mitre.org/software/S1090) can load a module to call CreateCompatibleDC and GdipSaveImageToStream for screen capture.[[135]](https://www.welivesecurity.com/en/eset-research/moustachedbouncer-espionage-against-foreign-diplomats-in-belarus/)

|  [S0385](https://attack.mitre.org/software/S0385)   |  [njRAT](https://attack.mitre.org/software/S0385)   |

[njRAT](https://attack.mitre.org/software/S0385) can capture screenshots of the victim’s machines.[[141]](https://blog.trendmicro.com/trendlabs-security-intelligence/autoit-compiled-worm-affecting-removable-media-delivers-fileless-version-of-bladabindi-njrat-backdoor/)[[142]](https://securelist.com/blindeagle-apt/113414/)

|  [S1107](https://attack.mitre.org/software/S1107)   |  [NKAbuse](https://attack.mitre.org/software/S1107)   |

[NKAbuse](https://attack.mitre.org/software/S1107) can take screenshots of the victim machine.[[143]](https://securelist.com/unveiling-nkabuse/111512/)

|  [S0644](https://attack.mitre.org/software/S0644)   |  [ObliqueRAT](https://attack.mitre.org/software/S0644)   |

[ObliqueRAT](https://attack.mitre.org/software/S0644) can capture a screenshot of the current screen.[[144]](https://blog.talosintelligence.com/2021/02/obliquerat-new-campaign.html)

|  [S0340](https://attack.mitre.org/software/S0340)   |  [Octopus](https://attack.mitre.org/software/S0340)   |

[Octopus](https://attack.mitre.org/software/S0340) can capture screenshots of the victims’ machine.[[145]](https://securelist.com/octopus-infested-seas-of-central-asia/88200/)[[146]](https://securityaffairs.co/wordpress/77165/apt/russia-linked-apt-dustsquad.html)[[147]](https://www.virusbulletin.com/uploads/pdf/conference_slides/2018/Cherepanov-VB2018-Octopus.pdf)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) has a tool called CANDYKING to capture a screenshot of user's desktop.[[148]](https://www.brighttalk.com/webcast/10703/296317/apt34-new-targeted-attack-in-the-middle-east)

|  [S1050](https://attack.mitre.org/software/S1050)   |  [PcShare](https://attack.mitre.org/software/S1050)   |

[PcShare](https://attack.mitre.org/software/S1050) can take screen shots of a compromised machine.[[74]](https://www.bitdefender.com/files/News/CaseStudies/study/379/Bitdefender-Whitepaper-Chinese-APT.pdf)

|  [S0643](https://attack.mitre.org/software/S0643)   |  [Peppy](https://attack.mitre.org/software/S0643)   |

[Peppy](https://attack.mitre.org/software/S0643) can take screenshots on targeted systems.[[52]](https://www.proofpoint.com/sites/default/files/proofpoint-operation-transparent-tribe-threat-insight-en.pdf)

|  [S0013](https://attack.mitre.org/software/S0013)   |  [PlugX](https://attack.mitre.org/software/S0013)   |

[PlugX](https://attack.mitre.org/software/S0013) allows the operator to capture screenshots.[[149]](http://circl.lu/assets/files/tr-12/tr-12-circl-plugx-analysis-v1.pdf)

|  [S0428](https://attack.mitre.org/software/S0428)   |  [PoetRAT](https://attack.mitre.org/software/S0428)   |

[PoetRAT](https://attack.mitre.org/software/S0428) has the ability to take screen captures.[[150]](https://blog.talosintelligence.com/2020/04/poetrat-covid-19-lures.html)[[151]](https://hub.dragos.com/hubfs/Year-in-Review/Dragos_2020_ICS_Cybersecurity_Year_In_Review.pdf?hsCtaTracking=159c0fc3-92d8-425d-aeb8-12824f2297e8%7Cf163726d-579b-4996-9a04-44e5a124d770)

|  [S0216](https://attack.mitre.org/software/S0216)   |  [POORAIM](https://attack.mitre.org/software/S0216)   |

[POORAIM](https://attack.mitre.org/software/S0216) can perform screen capturing.[[58]](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)

|  [S0194](https://attack.mitre.org/software/S0194)   |  [PowerSploit](https://attack.mitre.org/software/S0194)   |

[PowerSploit](https://attack.mitre.org/software/S0194)'s Get-TimedScreenshot Exfiltration module can take screenshots at regular intervals.[[152]](https://github.com/PowerShellMafia/PowerSploit)[[153]](http://powersploit.readthedocs.io)

|  [S0223](https://attack.mitre.org/software/S0223)   |  [POWERSTATS](https://attack.mitre.org/software/S0223)   |

[POWERSTATS](https://attack.mitre.org/software/S0223) can retrieve screenshots from compromised hosts.[[154]](https://www.fireeye.com/blog/threat-research/2018/03/iranian-threat-group-updates-ttps-in-spear-phishing-campaign.html)[[155]](https://blog.trendmicro.com/trendlabs-security-intelligence/muddywater-resurfaces-uses-multi-stage-backdoor-powerstats-v3-and-new-post-exploitation-tools/)

|  [S0184](https://attack.mitre.org/software/S0184)   |  [POWRUNER](https://attack.mitre.org/software/S0184)   |

[POWRUNER](https://attack.mitre.org/software/S0184) can capture a screenshot from a victim.[[156]](https://www.fireeye.com/blog/threat-research/2017/12/targeted-attack-in-middle-east-by-apt34.html)

|  [S0113](https://attack.mitre.org/software/S0113)   |  [Prikormka](https://attack.mitre.org/software/S0113)   |

[Prikormka](https://attack.mitre.org/software/S0113) contains a module that captures screenshots of the victim's desktop.[[157]](http://www.welivesecurity.com/wp-content/uploads/2016/05/Operation-Groundbait.pdf)

|  [S0279](https://attack.mitre.org/software/S0279)   |  [Proton](https://attack.mitre.org/software/S0279)   |

[Proton](https://attack.mitre.org/software/S0279) captures the content of the desktop with the screencapture binary.[[73]](https://objective-see.com/blog/blog_0x25.html)

|  [S0147](https://attack.mitre.org/software/S0147)   |  [Pteranodon](https://attack.mitre.org/software/S0147)   |

[Pteranodon](https://attack.mitre.org/software/S0147) can capture screenshots at a configurable interval.[[158]](https://researchcenter.paloaltonetworks.com/2017/02/unit-42-title-gamaredon-group-toolset-evolution/)[[159]](https://unit42.paloaltonetworks.com/gamaredon-primitive-bear-ukraine-update-2021/)

|  [S0192](https://attack.mitre.org/software/S0192)   |  [Pupy](https://attack.mitre.org/software/S0192)   |

[Pupy](https://attack.mitre.org/software/S0192) can drop a mouse-logger that will take small screenshots around at each click and then send back to the server.[[160]](https://github.com/n1nj4sec/pupy)

|  [S1209](https://attack.mitre.org/software/S1209)   |  [Quick Assist](https://attack.mitre.org/software/S1209)   |

[Quick Assist](https://attack.mitre.org/software/S1209) allows for the remote administrator to take screenshots of the running system.[[161]](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist)

|  [S0686](https://attack.mitre.org/software/S0686)   |  [QuietSieve](https://attack.mitre.org/software/S0686)   |

[QuietSieve](https://attack.mitre.org/software/S0686) has taken screenshots every five minutes and saved them to the user's local Application Data folder under Temp\SymbolSourceSymbols\icons or Temp\ModeAuto\icons.[[162]](https://www.microsoft.com/security/blog/2022/02/04/actinium-targets-ukrainian-organizations/)

|  [S1148](https://attack.mitre.org/software/S1148)   |  [Raccoon Stealer](https://attack.mitre.org/software/S1148)   |

[Raccoon Stealer](https://attack.mitre.org/software/S1148) can capture screenshots from victim systems.[[163]](https://medium.com/s2wblog/raccoon-stealer-is-back-with-a-new-version-5f436e04b20d)[[164]](https://blog.sekoia.io/raccoon-stealer-v2-part-2-in-depth-analysis/)

|  [S0629](https://attack.mitre.org/software/S0629)   |  [RainyDay](https://attack.mitre.org/software/S0629)   |

[RainyDay](https://attack.mitre.org/software/S0629) has the ability to capture screenshots.[[165]](https://www.bitdefender.com/files/News/CaseStudies/study/396/Bitdefender-PR-Whitepaper-NAIKON-creat5397-en-EN.pdf)

|  [S0458](https://attack.mitre.org/software/S0458)   |  [Ramsay](https://attack.mitre.org/software/S0458)   |

[Ramsay](https://attack.mitre.org/software/S0458) can take screenshots every 30 seconds as well as when an external removable storage device is connected.[[166]](https://www.programmersought.com/article/62493896999/)

|  [S0662](https://attack.mitre.org/software/S0662)   |  [RCSession](https://attack.mitre.org/software/S0662)   |

[RCSession](https://attack.mitre.org/software/S0662) can capture screenshots from a compromised host.[[167]](https://web.archive.org/web/20210104144857/https://shared-public-reports.s3-eu-west-1.amazonaws.com/APT27+turns+to+ransomware.pdf)

|  [S0495](https://attack.mitre.org/software/S0495)   |  [RDAT](https://attack.mitre.org/software/S0495)   |

[RDAT](https://attack.mitre.org/software/S0495) can take a screenshot on the infected system.[[168]](https://unit42.paloaltonetworks.com/oilrig-novel-c2-channel-steganography/)

|  [S0153](https://attack.mitre.org/software/S0153)   |  [RedLeaves](https://attack.mitre.org/software/S0153)   |

[RedLeaves](https://attack.mitre.org/software/S0153) can capture screenshots.[[169]](https://www.fireeye.com/blog/threat-research/2017/04/apt10_menupass_grou.html)[[170]](http://web.archive.org/web/20220810112638/https:/www.accenture.com/t20180423T055005Z_w_/se-en/_acnmedia/PDF-76/Accenture-Hogfish-Threat-Analysis.pdf)

|  [S1240](https://attack.mitre.org/software/S1240)   |  [RedLine Stealer](https://attack.mitre.org/software/S1240)   |

[RedLine Stealer](https://attack.mitre.org/software/S1240) can capture screenshots on a compromised host.[[171]](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/redline-stealer-a-novel-approach/)[[172]](https://www.splunk.com/en_us/blog/security/do-not-cross-the-redline-stealer-detections-and-analysis.html)

|  [S0332](https://attack.mitre.org/software/S0332)   |  [Remcos](https://attack.mitre.org/software/S0332)   |

[Remcos](https://attack.mitre.org/software/S0332) takes automated screenshots of the infected machine.[[173]](https://web.archive.org/web/20180124082756/https://www.riskiq.com/blog/labs/spear-phishing-turkish-defense-contractors/)[[174]](https://www.fortinet.com/blog/threat-research/new-campaign-uses-remcos-rat-to-exploit-victims)

|  [S0375](https://attack.mitre.org/software/S0375)   |  [Remexi](https://attack.mitre.org/software/S0375)   |

[Remexi](https://attack.mitre.org/software/S0375) takes screenshots of windows of interest.[[175]](https://securelist.com/chafer-used-remexi-malware/89538/)

|  [S0592](https://attack.mitre.org/software/S0592)   |  [RemoteUtilities](https://attack.mitre.org/software/S0592)   |

[RemoteUtilities](https://attack.mitre.org/software/S0592) can take screenshots on a compromised host.[[176]](https://www.trendmicro.com/en_us/research/21/c/earth-vetala---muddywater-continues-to-target-organizations-in-t.html)

|  [S0379](https://attack.mitre.org/software/S0379)   |  [Revenge RAT](https://attack.mitre.org/software/S0379)   |

[Revenge RAT](https://attack.mitre.org/software/S0379) has a plugin for screen capture.[[177]](https://www.cylance.com/content/dam/cylance-web/en-us/resources/knowledge-center/resource-library/reports/WhiteCompanyOperationShaheenReport.pdf?_ga=2.161661948.1943296560.1555683782-1066572390.1555511517)

|  [S0270](https://attack.mitre.org/software/S0270)   |  [RogueRobin](https://attack.mitre.org/software/S0270)   |

[RogueRobin](https://attack.mitre.org/software/S0270) has a command named $screenshot that may be responsible for taking screenshots of the victim machine.[[178]](https://researchcenter.paloaltonetworks.com/2018/07/unit42-new-threat-actor-group-darkhydrus-targets-middle-east-government/)

|  [S0240](https://attack.mitre.org/software/S0240)   |  [ROKRAT](https://attack.mitre.org/software/S0240)   |

[ROKRAT](https://attack.mitre.org/software/S0240) can capture screenshots of the infected system using the gdi32 library.[[179]](https://blog.talosintelligence.com/2017/04/introducing-rokrat.html)[[180]](https://blog.talosintelligence.com/2017/11/ROKRAT-Reloaded.html)[[181]](https://securelist.com/scarcruft-continues-to-evolve-introduces-bluetooth-harvester/90729/)[[182]](https://research.nccgroup.com/2018/11/08/rokrat-analysis/)[[183]](https://blog.malwarebytes.com/threat-analysis/2021/01/retrohunting-apt37-north-korean-apt-used-vba-self-decode-technique-to-inject-rokrat/)

|  [S0090](https://attack.mitre.org/software/S0090)   |  [Rover](https://attack.mitre.org/software/S0090)   |

[Rover](https://attack.mitre.org/software/S0090) takes screenshots of the compromised system's desktop and saves them to C:\system\screenshot.bmp for exfiltration every 60 minutes.[[184]](http://researchcenter.paloaltonetworks.com/2016/02/new-malware-rover-targets-indian-ambassador-to-afghanistan/)

|  [S0148](https://attack.mitre.org/software/S0148)   |  [RTM](https://attack.mitre.org/software/S0148)   |

[RTM](https://attack.mitre.org/software/S0148) can capture screenshots.[[185]](https://www.welivesecurity.com/wp-content/uploads/2017/02/Read-The-Manual.pdf)[[186]](https://unit42.paloaltonetworks.com/russian-language-malspam-pushing-redaman-banking-malware/)

|  [S0546](https://attack.mitre.org/software/S0546)   |  [SharpStage](https://attack.mitre.org/software/S0546)   |

[SharpStage](https://attack.mitre.org/software/S0546) has the ability to capture the victim's screen.[[187]](https://www.cybereason.com/hubfs/dam/collateral/reports/Molerats-in-the-Cloud-New-Malware-Arsenal-Abuses-Cloud-Platforms-in-Middle-East-Espionage-Campaign.pdf)[[188]](https://www.bleepingcomputer.com/news/security/hacking-group-s-new-malware-abuses-google-and-facebook-services/)

|  [S0217](https://attack.mitre.org/software/S0217)   |  [SHUTTERSPEED](https://attack.mitre.org/software/S0217)   |

[SHUTTERSPEED](https://attack.mitre.org/software/S0217) can capture screenshots.[[58]](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)

|  [S9058](https://attack.mitre.org/software/S9058)   |  [SIGHTGRAB](https://attack.mitre.org/software/S9058)   |

[SIGHTGRAB](https://attack.mitre.org/software/S9058) has captured screenshots of victim devices and has saved them incrementally within created folders.[[189]](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)

|  [G0091](https://attack.mitre.org/groups/G0091)   |  [Silence](https://attack.mitre.org/groups/G0091)   |

[Silence](https://attack.mitre.org/groups/G0091) can capture victim screen activity.[[190]](https://securelist.com/the-silence/83009/)[[191]](https://go.group-ib.com/report-silence-en?_gl=1*d1bh3a*_ga*MTIwMzM5Mzc5MS4xNjk4OTI5NzY4*_ga_QMES53K3Y2*MTcwNDcyMjU2OS40LjEuMTcwNDcyMzU1Mi41My4wLjA.)

|  [S0692](https://attack.mitre.org/software/S0692)   |  [SILENTTRINITY](https://attack.mitre.org/software/S0692)   |

[SILENTTRINITY](https://attack.mitre.org/software/S0692) can take a screenshot of the current desktop.[[192]](https://github.com/byt3bl33d3r/SILENTTRINITY/tree/master/silenttrinity/core/teamserver/modules/boo)

|  [S0633](https://attack.mitre.org/software/S0633)   |  [Sliver](https://attack.mitre.org/software/S0633)   |

[Sliver](https://attack.mitre.org/software/S0633) can take screenshots of the victim’s active display.[[193]](https://github.com/BishopFox/sliver/blob/master/implant/sliver/screen/screenshot_windows.go)

|  [S0533](https://attack.mitre.org/software/S0533)   |  [SLOTHFULMEDIA](https://attack.mitre.org/software/S0533)   |

[SLOTHFULMEDIA](https://attack.mitre.org/software/S0533) has taken a screenshot of a victim's desktop, named it "Filter3.jpg", and stored it in the local directory.[[194]](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-275a)

|  [S0649](https://attack.mitre.org/software/S0649)   |  [SMOKEDHAM](https://attack.mitre.org/software/S0649)   |

[SMOKEDHAM](https://attack.mitre.org/software/S0649) can capture screenshots of the victim’s desktop.[[195]](https://www.fireeye.com/blog/threat-research/2021/05/shining-a-light-on-darkside-ransomware-operations.html)[[196]](https://www.fireeye.com/blog/threat-research/2021/06/darkside-affiliate-supply-chain-software-compromise.html)

|  [S0273](https://attack.mitre.org/software/S0273)   |  [Socksbot](https://attack.mitre.org/software/S0273)   |

[Socksbot](https://attack.mitre.org/software/S0273) can take screenshots.[[197]](https://documents.trendmicro.com/assets/tech-brief-untangling-the-patchwork-cyberespionage-group.pdf)

|  [S0380](https://attack.mitre.org/software/S0380)   |  [StoneDrill](https://attack.mitre.org/software/S0380)   |

[StoneDrill](https://attack.mitre.org/software/S0380) can take screenshots.[[198]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07180722/Report_Shamoon_StoneDrill_final.pdf)

|  [S1034](https://attack.mitre.org/software/S1034)   |  [StrifeWater](https://attack.mitre.org/software/S1034)   |

[StrifeWater](https://attack.mitre.org/software/S1034) has the ability to take screen captures.[[199]](https://www.cybereason.com/blog/research/strifewater-rat-iranian-apt-moses-staff-adds-new-trojan-to-ransomware-operations)

|  [S1064](https://attack.mitre.org/software/S1064)   |  [SVCReady](https://attack.mitre.org/software/S1064)   |

[SVCReady](https://attack.mitre.org/software/S1064) can take a screenshot from an infected host.[[200]](https://threatresearch.ext.hp.com/svcready-a-new-loader-reveals-itself/)

|  [S0663](https://attack.mitre.org/software/S0663)   |  [SysUpdate](https://attack.mitre.org/software/S0663)   |

[SysUpdate](https://attack.mitre.org/software/S0663) has the ability to capture screenshots.[[201]](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)

|  [S0098](https://attack.mitre.org/software/S0098)   |  [T9000](https://attack.mitre.org/software/S0098)   |

[T9000](https://attack.mitre.org/software/S0098) can take screenshots of the desktop and target application windows, saving them to user directories as one byte XOR encrypted .dat files.[[202]](http://researchcenter.paloaltonetworks.com/2016/02/t9000-advanced-modular-backdoor-uses-complex-anti-analysis-techniques/)

|  [S0467](https://attack.mitre.org/software/S0467)   |  [TajMahal](https://attack.mitre.org/software/S0467)   |

[TajMahal](https://attack.mitre.org/software/S0467) has the ability to take screenshots on an infected host including capturing content from windows of instant messaging applications.[[203]](https://securelist.com/project-tajmahal/90240/)

|  [S0004](https://attack.mitre.org/software/S0004)   |  [TinyZBot](https://attack.mitre.org/software/S0004)   |

[TinyZBot](https://attack.mitre.org/software/S0004) contains screen capture functionality.[[204]](https://web.archive.org/web/20200302085133/https://www.cylance.com/content/dam/cylance/pages/operation-cleaver/Cylance_Operation_Cleaver_Report.pdf)

|  [S1239](https://attack.mitre.org/software/S1239)   |  [TONESHELL](https://attack.mitre.org/software/S1239)   |

[TONESHELL](https://attack.mitre.org/software/S1239) has conducted screen capturing.[[205]](https://unit42.paloaltonetworks.com/stately-taurus-attacks-se-asian-government/)

|  [S1201](https://attack.mitre.org/software/S1201)   |  [TRANSLATEXT](https://attack.mitre.org/software/S1201)   |

[TRANSLATEXT](https://attack.mitre.org/software/S1201) has the ability to capture screenshots of new browser tabs, based on the presence of the Capture flag.[[102]](https://www.zscaler.com/blogs/security-research/kimsuky-deploys-translatext-target-south-korean-academia#technical-analysis)

|  [S0094](https://attack.mitre.org/software/S0094)   |  [Trojan.Karagany](https://attack.mitre.org/software/S0094)   |

[Trojan.Karagany](https://attack.mitre.org/software/S0094) can take a desktop screenshot and save the file into \ProgramData\Mail\MailAg\shot.png.[[206]](https://community.broadcom.com/symantecenterprise/communities/community-home/librarydocuments/viewdocument?DocumentKey=7382dce7-0260-4782-84cc-890971ed3f17&CommunityKey=1ecf5f55-9545-44d6-b0f4-4e4a7f5f5e68&tab=librarydocuments)[[207]](https://www.secureworks.com/research/updated-karagany-malware-targets-energy-sector)

|  [S1196](https://attack.mitre.org/software/S1196)   |  [Troll Stealer](https://attack.mitre.org/software/S1196)   |

[Troll Stealer](https://attack.mitre.org/software/S1196) can capture screenshots from victim machines.[[208]](https://medium.com/s2wblog/kimsuky-disguised-as-a-korean-company-signed-with-a-valid-certificate-to-distribute-troll-stealer-cfa5d54314e2)[[209]](https://www.security.com/threat-intelligence/springtail-kimsuky-backdoor-espionage)

|  [S0647](https://attack.mitre.org/software/S0647)   |  [Turian](https://attack.mitre.org/software/S0647)   |

[Turian](https://attack.mitre.org/software/S0647) has the ability to take screenshots.[[210]](https://www.welivesecurity.com/2021/06/10/backdoordiplomacy-upgrading-quarian-turian/)

|  [S0199](https://attack.mitre.org/software/S0199)   |  [TURNEDUP](https://attack.mitre.org/software/S0199)   |

[TURNEDUP](https://attack.mitre.org/software/S0199) is capable of taking screenshots.[[211]](https://www.fireeye.com/blog/threat-research/2017/09/apt33-insights-into-iranian-cyber-espionage.html)

|  [S0275](https://attack.mitre.org/software/S0275)   |  [UPPERCUT](https://attack.mitre.org/software/S0275)   |

[UPPERCUT](https://attack.mitre.org/software/S0275) can capture desktop screenshots in the PNG format and send them to the C2 server.[[212]](https://www.fireeye.com/blog/threat-research/2018/09/apt10-targeting-japanese-corporations-using-updated-ttps.html)[[213]](https://www.trendmicro.com/en_us/research/25/d/earth-kasha-updates-ttps.html)[[214]](https://www.trendmicro.com/en_us/research/24/k/return-of-anel-in-the-recent-earth-kasha-spearphishing-campaign.html)[[215]](https://www.welivesecurity.com/en/eset-research/operation-akairyu-mirrorface-invites-europe-expo-2025-revives-anel-backdoor/)

|  [S0386](https://attack.mitre.org/software/S0386)   |  [Ursnif](https://attack.mitre.org/software/S0386)   |

[Ursnif](https://attack.mitre.org/software/S0386) has used hooked APIs to take screenshots.[[216]](https://web.archive.org/web/20210719165945/https://www.trendmicro.com/en_us/research/15/c/ursnif-the-multifaceted-malware.html?_ga=2.165628854.808042651.1508120821-744063452.1505819992)[[217]](https://www.trendmicro.com/vinfo/us/threat-encyclopedia/malware/BKDR_URSNIF.SM?_ga=2.129468940.1462021705.1559742358-1202584019.1549394279)

|  [S0476](https://attack.mitre.org/software/S0476)   |  [Valak](https://attack.mitre.org/software/S0476)   |

[Valak](https://attack.mitre.org/software/S0476) has the ability to take screenshots on a compromised host.[[218]](https://www.cybereason.com/blog/valak-more-than-meets-the-eye)

|  [S0257](https://attack.mitre.org/software/S0257)   |  [VERMIN](https://attack.mitre.org/software/S0257)   |

[VERMIN](https://attack.mitre.org/software/S0257) can perform screen captures of the victim’s machine.[[219]](https://researchcenter.paloaltonetworks.com/2018/01/unit42-vermin-quasar-rat-custom-malware-used-ukraine/)

|  [G1055](https://attack.mitre.org/groups/G1055)   |  [VOID MANTICORE](https://attack.mitre.org/groups/G1055)   |

[VOID MANTICORE](https://attack.mitre.org/groups/G1055) has captured screen content during an active Zoom session.[[220]](https://www.ic3.gov/CSA/2026/260320.pdf)

|  [G1017](https://attack.mitre.org/groups/G1017)   |  [Volt Typhoon](https://attack.mitre.org/groups/G1017)   |

[Volt Typhoon](https://attack.mitre.org/groups/G1017) has obtained a screenshot of the victim's system using the gdi32.dll and gdiplus.dll libraries.[[221]](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)

|  [G1035](https://attack.mitre.org/groups/G1035)   |  [Winter Vivern](https://attack.mitre.org/groups/G1035)   |

[Winter Vivern](https://attack.mitre.org/groups/G1035) delivered PowerShell scripts capable of taking screenshots of victim machines.[[222]](https://cert.gov.ua/article/3761104)

|  [S1065](https://attack.mitre.org/software/S1065)   |  [Woody RAT](https://attack.mitre.org/software/S1065)   |

[Woody RAT](https://attack.mitre.org/software/S1065) has the ability to take a screenshot of the infected host desktop using Windows GDI+.[[223]](https://www.malwarebytes.com/blog/threat-intelligence/2022/08/woody-rat-a-new-feature-rich-malware-spotted-in-the-wild)

|  [S0161](https://attack.mitre.org/software/S0161)   |  [XAgentOSX](https://attack.mitre.org/software/S0161)   |

[XAgentOSX](https://attack.mitre.org/software/S0161) contains the takeScreenShot (along with startTakeScreenShot and stopTakeScreenShot) functions to take screenshots using the CGGetActiveDisplayList, CGDisplayCreateImage, and NSImage:initWithCGImage methods.[[12]](https://researchcenter.paloaltonetworks.com/2017/02/unit42-xagentosx-sofacys-xagent-macos-tool/)

|  [S0658](https://attack.mitre.org/software/S0658)   |  [XCSSET](https://attack.mitre.org/software/S0658)   |

[XCSSET](https://attack.mitre.org/software/S0658) saves a screen capture of the victim's system with a numbered filename and .jpg extension. Screen captures are taken at specified intervals based on the system. [[224]](https://documents.trendmicro.com/assets/pdf/XCSSET_Technical_Brief.pdf)

|  [S1207](https://attack.mitre.org/software/S1207)   |  [XLoader](https://attack.mitre.org/software/S1207)   |

[XLoader](https://attack.mitre.org/software/S1207) can capture screenshots on compromised hosts.[[225]](https://cloud.google.com/blog/topics/threat-intelligence/formbook-malware-distribution-campaigns/)[[226]](https://www.netskope.com/blog/new-formbook-campaign-delivered-through-phishing-emails)

|  [S0248](https://attack.mitre.org/software/S0248)   |  [yty](https://attack.mitre.org/software/S0248)   |

[yty](https://attack.mitre.org/software/S0248) collects screenshots of the victim machine.[[227]](https://www.arbornetworks.com/blog/asert/donot-team-leverages-new-modular-malware-framework-south-asia/)

|  [S0251](https://attack.mitre.org/software/S0251)   |  [Zebrocy](https://attack.mitre.org/software/S0251)   |

A variant of [Zebrocy](https://attack.mitre.org/software/S0251) captures screenshots of the victim’s machine in JPEG and BMP format.[[36]](https://researchcenter.paloaltonetworks.com/2018/11/unit42-sofacy-continues-global-attacks-wheels-new-cannon-trojan/)[[228]](https://www.welivesecurity.com/2018/11/20/sednit-whats-going-zebrocy/)[[229]](https://unit42.paloaltonetworks.com/dear-joohn-sofacy-groups-global-campaign/)[[230]](https://www.welivesecurity.com/2019/05/22/journey-zebrocy-land/)[[231]](https://www.accenture.com/t20181129T203820Z__w__/us-en/_acnmedia/PDF-90/Accenture-snakemackerel-delivers-zekapab-malware.pdf#zoom=50)[[232]](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-303b)

|  [S0330](https://attack.mitre.org/software/S0330)   |  [Zeus Panda](https://attack.mitre.org/software/S0330)   |

[Zeus Panda](https://attack.mitre.org/software/S0330) can take screenshots of the victim’s machine.[[233]](https://cyberwtf.files.wordpress.com/2017/07/panda-whitepaper.pdf)

|  [S0086](https://attack.mitre.org/software/S0086)   |  [ZLib](https://attack.mitre.org/software/S0086)   |

[ZLib](https://attack.mitre.org/software/S0086) has the ability to obtain screenshots of the compromised system.[[234]](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)

|  [S0412](https://attack.mitre.org/software/S0412)   |  [ZxShell](https://attack.mitre.org/software/S0412)   |

[ZxShell](https://attack.mitre.org/software/S0412) can capture screenshots.[[235]](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)

## Mitigations

This type of attack technique cannot be easily mitigated with preventive controls since it is based on the abuse of system features.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0346](https://attack.mitre.org/detectionstrategies/DET0346)   |  [Detect Screen Capture via Commands and API Calls](https://attack.mitre.org/detectionstrategies/DET0346)   |  [AN0980](https://attack.mitre.org/detectionstrategies/DET0346#AN0980)   |

Unusual use of screen capture APIs (e.g., CopyFromScreen) or command-line tools to write image files to disk.

|  [AN0981](https://attack.mitre.org/detectionstrategies/DET0346#AN0981)   |

Invocation of built-in commands like screencapture or use of undocumented APIs from suspicious parent processes.

|  [AN0982](https://attack.mitre.org/detectionstrategies/DET0346#AN0982)   |

Use of tools like xwd or import to generate screenshots, especially under non-GUI parent processes.

## References

-    [Microsoft. (n.d.). Graphics.CopyFromScreen Method. Retrieved March 24, 2020.](https://docs.microsoft.com/en-us/dotnet/api/system.drawing.graphics.copyfromscreen?view=netframework-4.8)
-    [Thomas Reed. (2017, January 18). New Mac backdoor using antiquated code. Retrieved July 5, 2017.](https://blog.malwarebytes.com/threat-analysis/2017/01/new-mac-backdoor-using-antiquated-code/)
-    [CERT Polska. (2026, January 30). Energy Sector Incident Report – 29 December. Retrieved April 22, 2026.](https://cert.pl/uploads/docs/CERT_Polska_Energy_Sector_Incident_Report_2025.pdf)
-    [Brumaghin, E., et al. (2018, October 15). Old dog, new tricks - Analysing new RTF-based campaign distributing Agent Tesla, Loki with PyREbox. Retrieved November 5, 2018.](https://blog.talosintelligence.com/2018/10/old-dog-new-tricks-analysing-new-rtf_15.html)
-    [The DigiTrust Group. (2017, January 12). The Rise of Agent Tesla. Retrieved November 5, 2018.](https://www.digitrustgroup.com/agent-tesla-keylogger/)
-    [Zhang, X. (2018, April 05). Analysis of New Agent Tesla Spyware Variant. Retrieved November 5, 2018.](https://www.fortinet.com/blog/threat-research/analysis-of-new-agent-tesla-spyware-variant.html)
-    [Zhang, X. (2017, June 28). In-Depth Analysis of A New Variant of .NET Malware AgentTesla. Retrieved November 5, 2018.](https://www.fortinet.com/blog/threat-research/in-depth-analysis-of-net-malware-javaupdtr.html)
-    [Arsene, L. (2020, April 21). Oil & Gas Spearphishing Campaigns Drop Agent Tesla Spyware in Advance of Historic OPEC+ Deal. Retrieved May 19, 2020.](https://labs.bitdefender.com/2020/04/oil-gas-spearphishing-campaigns-drop-agent-tesla-spyware-in-advance-of-historic-opec-deal/)
-    [Jazi, H. (2021, June 1). Kimsuky APT continues to target South Korean government using AppleSeed backdoor. Retrieved June 10, 2021.](https://blog.malwarebytes.com/threat-analysis/2021/06/kimsuky-apt-continues-to-target-south-korean-government-using-appleseed-backdoor/)
-    [KISA. (2021). Phishing Target Reconnaissance and Attack Resource Analysis Operation Muzabi. Retrieved March 8, 2024.](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)
-    [ESET. (2016, October). En Route with Sednit - Part 2: Observing the Comings and Goings. Retrieved November 21, 2016.](http://www.welivesecurity.com/wp-content/uploads/2016/10/eset-sednit-part-2.pdf)
-    [Robert Falcone. (2017, February 14). XAgentOSX: Sofacy's Xagent macOS Tool. Retrieved July 12, 2017.](https://researchcenter.paloaltonetworks.com/2017/02/unit42-xagentosx-sofacys-xagent-macos-tool/)
-    [Mueller, R. (2018, July 13). Indictment - United States of America vs. VIKTOR BORISOVICH NETYKSHO, et al. Retrieved November 17, 2024.](https://cdn.cnn.com/cnn/2018/images/07/13/gru.indictment.pdf)
-    [Secureworks CTU. (2017, March 30). IRON TWILIGHT Supports Active Measures. Retrieved February 28, 2022.](https://www.secureworks.com/research/iron-twilight-supports-active-measures)
-    [Symantec. (2018, February 28). Chafer: Latest Attacks Reveal Heightened Ambitions. Retrieved May 22, 2020.](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/chafer-latest-attacks-reveal-heightened-ambitions)
-    [FBI. (2020, September 17). Indicators of Compromise Associated with Rana Intelligence Computing, also known as Advanced Persistent Threat 39, Chafer, Cadelspy, Remexi, and ITG07. Retrieved December 10, 2020.](https://www.iranwatch.org/sites/default/files/public-intelligence-alert.pdf)
-    [Mandiant. (n.d.). APT42: Crooked Charms, Cons and Compromises. Retrieved October 9, 2024.](https://services.google.com/fh/files/misc/apt42-crooked-charms-cons-and-compromises.pdf)
-    [CheckPoint. (2020, May 7). Naikon APT: Cyber Espionage Reloaded. Retrieved May 26, 2020.](https://research.checkpoint.com/2020/naikon-apt-cyber-espionage-reloaded/)
-    [Unit 42. (2025, December 11). Hamas-Affiliated Ashen Lepus Targets Middle Eastern Diplomatic Entities With New AshTag Malware Suite. Retrieved April 20, 2026.](https://unit42.paloaltonetworks.com/hamas-affiliate-ashen-lepus-uses-new-malware-suite-ashtag/)
-    [Nyan-x-Cat. (n.d.). NYAN-x-CAT / AsyncRAT-C-Sharp. Retrieved October 3, 2023.](https://github.com/NYAN-x-CAT/AsyncRAT-C-Sharp/blob/master/README.md)
-    [Hromcova, Z. (2019, October). AT COMMANDS, TOR-BASED COMMUNICATIONS: MEET ATTOR, A FANTASY CREATURE AND ALSO A SPY PLATFORM. Retrieved May 6, 2020.](https://www.welivesecurity.com/wp-content/uploads/2019/10/ESET_Attor.pdf)
-    [Yan, T., et al. (2018, November 21). New Wine in Old Bottle: New Azorult Variant Found in FindMyName Campaign using Fallout Exploit Kit. Retrieved November 29, 2018.](https://researchcenter.paloaltonetworks.com/2018/11/unit42-new-wine-old-bottle-new-azorult-variant-found-findmyname-campaign-using-fallout-exploit-kit/)
-    [Vrabie, V., et al. (2021, March 10). FIN8 Returns with Improved BADHATCH Toolkit. Retrieved September 8, 2021.](https://www.bitdefender.com/files/News/CaseStudies/study/394/Bitdefender-PR-Whitepaper-BADHATCH-creat5237-en-EN.pdf)
-    [Settle, A., et al. (2016, August 8). MONSOON - Analysis Of An APT Campaign. Retrieved September 22, 2016.](https://www.forcepoint.com/sites/default/files/resources/files/forcepoint-security-labs-monsoon-analysis-report.pdf)
-    [Levene, B. et al.. (2018, March 7). Patchwork Continues to Deliver BADNEWS to the Indian Subcontinent. Retrieved March 31, 2018.](https://researchcenter.paloaltonetworks.com/2018/03/unit42-patchwork-continues-deliver-badnews-indian-subcontinent/)
-    [Bar, T., Conant, S. (2017, October 20). BadPatch. Retrieved November 13, 2018.](https://researchcenter.paloaltonetworks.com/2017/10/unit42-badpatch/)
-    [Blaich, A., et al. (2018, January 18). Dark Caracal: Cyber-espionage at a Global Scale. Retrieved April 11, 2018.](https://info.lookout.com/rs/051-ESQ-475/images/Lookout_Dark-Caracal_srr_20180118_us_v.1.0.pdf)
-    [Check Point. (2020, November 26). Bandook: Signed & Delivered. Retrieved May 31, 2021.](https://research.checkpoint.com/2020/bandook-signed-delivered/)
-    [Mandiant. (n.d.). Appendix C (Digital) - The Malware Arsenal. Retrieved July 18, 2016.](https://www.mandiant.com/sites/default/files/2021-09/mandiant-apt1-report.pdf)
-    [Baumgartner, K. and Garnaeva, M.. (2014, November 3). BE2 custom plugins, router abuse, and target profiles. Retrieved March 24, 2016.](https://securelist.com/be2-custom-plugins-router-abuse-and-target-profiles/67353/)
-    [Cash, D., Grunzweig, J., Meltzer, M., Adair, S., Lancaster, T. (2021, August 17). North Korean APT InkySquid Infects Victims Using Browser Exploits. Retrieved September 30, 2021.](https://www.volexity.com/blog/2021/08/17/north-korean-apt-inkysquid-infects-victims-using-browser-exploits/)
-    [Counter Threat Unit Research Team. (2017, October 12). BRONZE BUTLER Targets Japanese Enterprises. Retrieved January 4, 2018.](https://www.secureworks.com/research/bronze-butler-targets-japanese-businesses)
-    [Chen, J. et al. (2019, November). Operation ENDTRADE: TICK’s Multi-Stage Backdoors for Attacking Industries and Stealing Classified Data. Retrieved June 9, 2020.](https://documents.trendmicro.com/assets/pdf/Operation-ENDTRADE-TICK-s-Multi-Stage-Backdoors-for-Attacking-Industries-and-Stealing-Classified-Data.pdf)
-    [Harbison, M. and Renals, P. (2022, July 5). When Pentest Tools Go Brutal: Red-Teaming Tool Being Abused by Malicious Actors. Retrieved February 1, 2023.](https://unit42.paloaltonetworks.com/brute-ratel-c4-tool/)
-    [Symantec Security Response. (2015, December 7). Iran-based attackers use back door threats to spy on Middle Eastern targets. Retrieved April 17, 2019.](https://www.symantec.com/connect/blogs/iran-based-attackers-use-back-door-threats-spy-middle-eastern-targets)
-    [Falcone, R., Lee, B. (2018, November 20). Sofacy Continues Global Attacks and Wheels Out New ‘Cannon’ Trojan. Retrieved November 26, 2018.](https://researchcenter.paloaltonetworks.com/2018/11/unit42-sofacy-continues-global-attacks-wheels-new-cannon-trojan/)
-    [Bennett, J., Vengerik, B. (2017, June 12). Behind the CARBANAK Backdoor. Retrieved June 11, 2018.](https://www.fireeye.com/blog/threat-research/2017/06/behind-the-carbanak-backdoor.html)
-    [Giuliani, M., Allievi, A. (2011, February 28). Carberp - a modular information stealing trojan. Retrieved September 12, 2024.](https://web.archive.org/web/20231227000328/http://pxnow.prevx.com/content/blog/carberp-a_modular_information_stealing_trojan.pdf)
-    [Grunzweig, J.. (2017, April 20). Cardinal RAT Active for Over Two Years. Retrieved December 8, 2018.](https://researchcenter.paloaltonetworks.com/2017/04/unit42-cardinal-rat-active-two-years/)
-    [Balanza, M. (2018, April 02). Infostealer.Catchamas. Retrieved November 17, 2024.](https://web.archive.org/web/20190508165711/https://www-west.symantec.com/content/symantec/english/en/security-center/writeup.html/2018-040209-1742-99)
-    [Salem, E. (2020, November 17). CHAES: Novel Malware Targeting Latin American E-Commerce. Retrieved June 30, 2021.](https://www.cybereason.com/hubfs/dam/collateral/reports/11-2020-Chaes-e-commerce-malware-research.pdf)
-    [Check Point. (2022, January 11). APT35 exploits Log4j vulnerability to distribute new modular PowerShell toolkit. Retrieved January 24, 2022.](https://research.checkpoint.com/2022/apt35-exploits-log4j-vulnerability-to-distribute-new-modular-powershell-toolkit/)
-    [Jenkins, L. at al. (2022, August 4). ROADSWEEP Ransomware - Likely Iranian Threat Actor Conducts Politically Motivated Disruptive Activity Against Albanian Government Organizations. Retrieved August 6, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/likely-iranian-threat-actor-conducts-politically-motivated-disruptive-activity-against/)
-    [Dupuy, T. and Faou, M. (2021, June). Gelsemium. Retrieved November 30, 2021.](https://www.welivesecurity.com/wp-content/uploads/2021/06/eset_gelsemium.pdf)
-    [Lunghi, D. et al. (2020, February). Uncovering DRBControl. Retrieved November 12, 2021.](https://documents.trendmicro.com/assets/white_papers/wp-uncovering-DRBcontrol.pdf)
-    [Strategic Cyber LLC. (2017, March 14). Cobalt Strike Manual. Retrieved May 24, 2017.](https://web.archive.org/web/20210825130434/https://cobaltstrike.com/downloads/csmanual38.pdf)
-    [Amnesty International. (2021, February 24). Vietnamese activists targeted by notorious hacking group. Retrieved March 1, 2021.](https://www.amnestyusa.org/wp-content/uploads/2021/02/Click-and-Bait_Vietnamese-Human-Rights-Defenders-Targeted-with-Spyware-Attacks.pdf)
-    [Strategic Cyber LLC. (2020, November 5). Cobalt Strike: Advanced Threat Tactics for Penetration Testers. Retrieved April 13, 2021.](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)
-    [Yadav, A., et al. (2017, August 31). Cobian RAT – A backdoored RAT. Retrieved November 13, 2018.](https://www.zscaler.com/blogs/research/cobian-rat-backdoored-rat)
-    [Mele, G. et al. (2021, February 10). Probable Iranian Cyber Actors, Static Kitten, Conducting Cyberespionage Campaign Targeting UAE and Kuwait Government Agencies. Retrieved March 17, 2021.](https://www.anomali.com/blog/probable-iranian-cyber-actors-static-kitten-conducting-cyberespionage-campaign-targeting-uae-and-kuwait-government-agencies)
-    [F-Secure Labs. (2014, July). COSMICDUKE Cosmu with a twist of MiniDuke. Retrieved July 3, 2014.](https://blog.f-secure.com/wp-content/uploads/2019/10/CosmicDuke.pdf)
-    [Huss, D. (2016, March 1). Operation Transparent Tribe. Retrieved June 8, 2016.](https://www.proofpoint.com/sites/default/files/proofpoint-operation-transparent-tribe-threat-insight-en.pdf)
-    [Dedola, G. (2020, August 20). Transparent Tribe: Evolution analysis, part 1. Retrieved September 2, 2021.](https://securelist.com/transparent-tribe-part-1/98127/)
-    [N. Baisini. (2022, July 13). Transparent Tribe begins targeting education sector in latest campaign. Retrieved September 22, 2022.](https://blog.talosintelligence.com/2022/07/transparent-tribe-targets-education.html)
-    [Kohler, A. and Lopez, C. (2024, April 30). Malware: Cuckoo Behaves Like Cross Between Infostealer and Spyware. Retrieved August 20, 2024.](https://www.kandji.io/blog/malware-cuckoo-infostealer-spyware)
-    [Chen, J. and Hsieh, M. (2017, November 7). REDBALDKNIGHT/BRONZE BUTLER’s Daserf Backdoor Now Using Steganography. Retrieved December 27, 2017.](http://blog.trendmicro.com/trendlabs-security-intelligence/redbaldknight-bronze-butler-daserf-backdoor-now-using-steganography/)
-    [FireEye. (2018, March 16). Suspected Chinese Cyber Espionage Group (TEMP.Periscope) Targeting U.S. Engineering and Maritime Industries. Retrieved April 11, 2018.](https://www.fireeye.com/blog/threat-research/2018/03/suspected-chinese-espionage-group-targeting-maritime-and-engineering-industries.html)
-    [FireEye. (2018, February 20). APT37 (Reaper): The Overlooked North Korean Actor. Retrieved November 17, 2024.](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)
-    [Grunzweig, J. (2018, October 01). NOKKI Almost Ties the Knot with DOGCALL: Reaper Group Uses New Malware to Deploy RAT. Retrieved November 5, 2018.](https://researchcenter.paloaltonetworks.com/2018/10/unit42-nokki-almost-ties-the-knot-with-dogcall-reaper-group-uses-new-malware-to-deploy-rat/)
-    [US-CERT. (2018, March 16). Alert (TA18-074A): Russian Government Cyber Activity Targeting Energy and Other Critical Infrastructure Sectors. Retrieved June 6, 2018.](https://www.us-cert.gov/ncas/alerts/TA18-074A)
-    [Symantec Security Response. (2014, July 7). Dragonfly: Western energy sector targeted by sophisticated attack group. Retrieved September 9, 2017.](https://docs.broadcom.com/doc/dragonfly_threat_against_western_energy_suppliers)
-    [Slowik, J. (2021, October). THE BAFFLING BERSERK BEAR: A DECADE’S ACTIVITY TARGETING CRITICAL INFRASTRUCTURE. Retrieved December 6, 2021.](https://vblocalhost.com/uploads/VB2021-Slowik.pdf)
-    [Mike Stokkel et al. (2024, July 18). APT41 Has Arisen From the DUST. Retrieved September 16, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/apt41-arisen-from-dust)
-    [GReAT. (2019, April 10). Gaza Cybergang Group1, operation SneakyPastes. Retrieved May 13, 2020.](https://securelist.com/gaza-cybergang-group1-operation-sneakypastes/90068/)
-    [Cybersecurity and Infrastructure Security Agency. (2020, August 26). MAR-10301706-1.v1 - North Korean Remote Access Tool: ECCENTRICBANDWAGON. Retrieved March 18, 2021.](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-239a)
-    [Schroeder, W., Warner, J., Nelson, M. (n.d.). Github PowerShellEmpire. Retrieved April 28, 2016.](https://github.com/PowerShellEmpire/Empire)
-    [PwC and BAE Systems. (2017, April). Operation Cloud Hopper: Technical Annex. Retrieved April 13, 2017.](https://www.pwc.co.uk/cyber-security/pdf/pwc-uk-operation-cloud-hopper-technical-annex-april-2017.pdf)
-    [Department of Justice. (2018, August 01). HOW FIN7 ATTACKED AND STOLE DATA. Retrieved August 24, 2018.](https://www.justice.gov/opa/press-release/file/1084361/download)
-    [FinFisher. (n.d.). Retrieved September 12, 2024.](https://web.archive.org/web/20171222050934/http://www.finfisher.com/FinFisher/index.html)
-    [Allievi, A.,Flori, E. (2018, March 01). FinFisher exposed: A researcher’s tale of defeating traps, tricks, and complex virtual machines. Retrieved July 9, 2018.](https://cloudblogs.microsoft.com/microsoftsecure/2018/03/01/finfisher-exposed-a-researchers-tale-of-defeating-traps-tricks-and-complex-virtual-machines/)
-    [Gostev, A. (2012, May 28). The Flame: Questions and Answers. Retrieved March 1, 2017.](https://securelist.com/the-flame-questions-and-answers-51/34344/)
-    [Financial Security Institute. (2020, February 28). Profiling of TA505 Threat Group That Continues to Attack the Financial Sector. Retrieved July 14, 2022.](https://www.fsec.or.kr/user/bbs/fsec/163/344/bbsDataView/1382.do?page=1&column=&search=&searchSDate=&searchEDate=&bbsDataCategory=)
-    [Patrick Wardle. (n.d.). Mac Malware of 2017. Retrieved September 21, 2018.](https://objective-see.com/blog/blog_0x25.html)
-    [Vrabie, V. (2020, November). Dissecting a Chinese APT Targeting South Eastern Asian Government Institutions. Retrieved September 19, 2022.](https://www.bitdefender.com/files/News/CaseStudies/study/379/Bitdefender-Whitepaper-Chinese-APT.pdf)
-    [Boutin, J. (2020, June 11). Gamaredon group grows its game. Retrieved June 16, 2020.](https://www.welivesecurity.com/2020/06/11/gamaredon-group-grows-its-game/)
-    [Threat Hunter Team, Symantec and Carbon Black. (2025, April 10). Shuckworm Targets Foreign Military Mission Based in Ukraine. Retrieved July 23, 2025.](https://www.security.com/threat-intelligence/shuckworm-ukraine-gammasteel)
-    [Rusnák, Z. (2024, September 26). Cyberespionage the Gamaredon way: Analysis of toolset used to spy on Ukraine in 2022 and 2023. Retrieved October 30, 2024.](https://web-assets.esetstatic.com/wls/en/papers/white-papers/cyberespionage-gamaredon-way.pdf)
-    [Pantazopoulos, N. (2018, April 17). Decoding network data from a Gh0st RAT variant. Retrieved November 2, 2018.](https://research.nccgroup.com/2018/04/17/decoding-network-data-from-a-gh0st-rat-variant/)
-    [Tetra Defense. (2020, March). CAUSE AND EFFECT: SODINOKIBI RANSOMWARE ANALYSIS. Retrieved November 17, 2024.](https://web.archive.org/web/20210414101816/https://tetradefense.com/incident-response-services/cause-and-effect-sodinokibi-ransomware-analysis/)
-    [Namestnikov, Y. and Aime, F. (2019, May 8). FIN7.5: the infamous cybercrime rig “FIN7” continues its activities. Retrieved October 11, 2019.](https://securelist.com/fin7-5-the-infamous-cybercrime-rig-fin7-continues-its-activities/90703/)
-    [Scott-Railton, J., et al. (2016, August 2). Group5: Syria and the Iranian Connection. Retrieved September 26, 2016.](https://citizenlab.ca/2016/08/group5-syria/)
-    [Carr, N., et al. (2017, April 24). FIN7 Evolution and the Phishing LNK. Retrieved April 24, 2017.](https://www.fireeye.com/blog/threat-research/2017/04/fin7-phishing-lnk.html)
-    [Ungur, P. (n.d.). HAVOC. Retrieved August 4, 2025.](https://havocframework.com/docs/welcome)
-    [Shivtarkar, N. and Jain, S. (2023, February 14). Havoc Across the Cyberspace. Retrieved August 4, 2025.](https://www.zscaler.com/blogs/security-research/havoc-across-cyberspace)
-    [Immersive Content Team. (2024, April 9). Havoc C2 Framework – A Defensive Operator’s Guide. Retrieved August 13, 2025.](https://www.immersivelabs.com/resources/blog/havoc-c2-framework-a-defensive-operators-guide)
-    [Knight, S.. (2020, April 16). VMware Carbon Black TAU Threat Analysis: The Evolution of Lazarus. Retrieved May 1, 2020.](https://www.carbonblack.com/2020/04/16/vmware-carbon-black-tau-threat-analysis-the-evolution-of-lazarus/)
-    [Alexndru-Cristian Bardas. (2025, October 30). DPRK’s Playbook: Kimsuky’s HttpTroy and Lazarus’s New BLINDINGCAN Variant. Retrieved April 8, 2026.](https://www.gendigital.com/blog/insights/research/dprk-kimsuky-lazarus-analysis)
-    [Lelli, A. (2010, January 11). Trojan.Hydraq. Retrieved February 20, 2018.](https://www.symantec.com/security_response/writeup.jsp?docid=2010-011114-1830-99)
-    [Falcone, R. and Lancaster, T. (2019, May 28). Emissary Panda Attacks Middle East Government Sharepoint Servers. Retrieved July 9, 2019.](https://unit42.paloaltonetworks.com/emissary-panda-attacks-middle-east-government-sharepoint-servers/)
-    [Hromcová, Z. (2018, June 07). InvisiMole: Surprisingly equipped spyware, undercover since 2013. Retrieved July 10, 2018.](https://www.welivesecurity.com/2018/06/07/invisimole-equipped-spyware-undercover/)
-    [Hromcova, Z. and Cherpanov, A. (2020, June). INVISIMOLE: THE HIDDEN PART OF THE STORY. Retrieved July 16, 2020.](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_InvisiMole.pdf)
-    [Brod. (2013, July 15). Signed Mac Malware Using Right-to-Left Override Trick. Retrieved July 17, 2017.](https://www.f-secure.com/weblog/archives/00002576.html)
-    [Thomas. (2013, July 15). New signed malware called Janicab. Retrieved July 17, 2017.](https://web.archive.org/web/20230331162455/https://www.thesafemac.com/new-signed-malware-called-janicab/)
-    [Unit 42. (2017, December 15). Unit 42 Playbook Viewer. Retrieved December 20, 2017.](https://web.archive.org/web/20230702043312/https://pan-unit42.github.io/playbook_viewer/)
-    [Mercer, W., et al. (2017, October 22). "Cyber Conflict" Decoy Document Used in Real Cyber Conflict. Retrieved November 2, 2018.](https://blog.talosintelligence.com/2017/10/cyber-conflict-decoy-document.html)
-    [Sharma, R. (2018, August 15). Revamped jRAT Uses New Anti-Parsing Techniques. Retrieved September 21, 2018.](https://www.symantec.com/blogs/threat-intelligence/jrat-new-anti-parsing-techniques)
-    [Kamluk, V. & Gostev, A. (2016, February). Adwind - A Cross-Platform RAT. Retrieved April 23, 2019.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07195002/KL_AdwindPublicReport_2016.pdf)
-    [Yadav, A., et al. (2016, January 29). Malicious Office files dropping Kasidet and Dridex. Retrieved March 24, 2016.](http://research.zscaler.com/2016/01/malicious-office-files-dropping-kasidet.html)
-    [Levene, B, et al. (2017, May 03). Kazuar: Multiplatform Espionage Backdoor with API Access. Retrieved July 17, 2018.](https://researchcenter.paloaltonetworks.com/2017/05/unit42-kazuar-multiplatform-espionage-backdoor-api-access/)
-    [Parys, B. (2017, February 11). The KeyBoys are back in town. Retrieved June 13, 2019.](https://web.archive.org/web/20211129064701/https://www.pwc.co.uk/issues/cyber-security-services/research/the-keyboys-are-back-in-town.html)
-    [US-CERT. (2018, August 09). MAR-10135536-17 – North Korean Trojan: KEYMARBLE. Retrieved August 16, 2018.](https://www.us-cert.gov/ncas/analysis-reports/AR18-221A)
-    [Park, S. (2024, June 27). Kimsuky deploys TRANSLATEXT to target South Korean academia. Retrieved October 14, 2024.](https://www.zscaler.com/blogs/security-research/kimsuky-deploys-translatext-target-south-korean-academia#technical-analysis)
-    [Bermejo, L., et al. (2017, June 22). Following the Trail of BlackTech’s Cyber Espionage Campaigns. Retrieved May 5, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)
-    [Rascagneres, P. (2017, May 03). KONNI: A Malware Under The Radar For Years. Retrieved November 5, 2018.](https://blog.talosintelligence.com/2017/05/konni-malware-under-radar-for-years.html)
-    [Stuart Ashenbrenner, Alden Schmidt. (2024, April 25). LightSpy Malware Variant Targeting macOS. Retrieved January 3, 2025.](https://www.huntress.com/blog/lightspy-malware-variant-targeting-macos)
-    [Yamout, M. (2021, November 29). WIRTE’s campaign in the Middle East ‘living off the land’ since at least 2019. Retrieved February 1, 2022.](https://securelist.com/wirtes-campaign-in-the-middle-east-living-off-the-land-since-at-least-2019/105044)
-    [Seals, T. (2021, May 14). FIN7 Backdoor Masquerades as Ethical Hacking Tool. Retrieved February 2, 2022.](https://threatpost.com/fin7-backdoor-ethical-hacking-tool/166194/)
-    [BI.ZONE Cyber Threats Research Team. (2021, May 13). From pentest to APT attack: cybercriminal group FIN7 disguises its malware as an ethical hacker’s toolkit. Retrieved February 2, 2022.](https://bi-zone.medium.com/from-pentest-to-apt-attack-cybercriminal-group-fin7-disguises-its-malware-as-an-ethical-hackers-c23c9a75e319)
-    [Ishimaru, S. (2022, October 31). APT10: Tracking down LODEINFO 2022, part II. Retrieved April 17, 2026.](https://securelist.com/apt10-tracking-down-lodeinfo-2022-part-ii/107745/)
-    [Breitenbacher, D. (2022, December 14). Unmasking MirrorFace: Operation LiberalFace targeting Japanese political entities. Retrieved April 17, 2026.](https://www.welivesecurity.com/2022/12/14/unmasking-mirrorface-operation-liberalface-targeting-japanese-political-entities/)
-    [ITOCHU. (2024, January 24). The Endless Struggle Against APT10: Insights from LODEINFO v0.6.6 - v0.7.3 Analysis. Retrieved April 17, 2026.](https://blog-en.itochuci.co.jp/entry/2024/01/24/134100)
-    [Raggi, M. Schwarz, D.. (2019, August 1). LookBack Malware Targets the United States Utilities Sector with Phishing Attacks Impersonating Engineering Licensing Boards. Retrieved February 25, 2021.](https://www.proofpoint.com/us/threat-insight/post/lookback-malware-targets-united-states-utilities-sector-phishing-attacks)
-    [Cybereaon Security Services Team. (n.d.). Your Data Is Under New Lummanagement: The Rise of LummaStealer. Retrieved March 22, 2025.](https://www.cybereason.com/blog/threat-analysis-rise-of-lummastealer)
-    [Jurčacko, F. (2024, May 15). To the Moon and back(doors): Lunar landing in diplomatic missions. Retrieved June 26, 2024.](https://www.welivesecurity.com/en/eset-research/moon-backdoors-lunar-landing-diplomatic-missions/)
-    [ESET. (2019, July). MACHETE JUST GOT SHARPER Venezuelan government institutions under attack. Retrieved September 13, 2019.](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)
-    [Kaspersky Global Research and Analysis Team. (2014, August 20). El Machete. Retrieved September 13, 2019.](https://securelist.com/el-machete/66108/)
-    [The Cylance Threat Research Team. (2017, March 22). El Machete's Malware Attacks Cut Through LATAM. Retrieved September 13, 2019.](https://threatvector.cylance.com/en_us/home/el-machete-malware-attacks-cut-through-latam.html)
-    [kate. (2020, September 25). APT-C-43 steals Venezuelan military secrets to provide intelligence support for the reactionaries — HpReact campaign. Retrieved November 20, 2020.](https://blog.360totalsecurity.com/en/apt-c-43-steals-venezuelan-military-secrets-to-provide-intelligence-support-for-the-reactionaries-hpreact-campaign/)

-    [M.Léveillé, M., Cherepanov, A.. (2022, January 25). Watering hole deploys new macOS malware, DazzleSpy, in Asia. Retrieved May 6, 2022.](https://www.welivesecurity.com/2022/01/25/watering-hole-deploys-new-macos-malware-dazzlespy-asia/)
-    [Wardle, P. (2021, November 11). OSX.CDDS (OSX.MacMa). Retrieved June 30, 2022.](https://objective-see.org/blog/blog_0x69.html)
-    [Ehrlich, A., et al. (2022, September). THE MYSTERY OF METADOR | AN UNATTRIBUTED THREAT HIDING IN TELCOS, ISPS, AND UNIVERSITIES. Retrieved January 23, 2023.](https://assets.sentinelone.com/sentinellabs22/metador#page=1)
-    [Lee, B. and Falcone, R. (2017, February 15). Magic Hound Campaign Attacks Saudi Targets. Retrieved December 27, 2017.](https://researchcenter.paloaltonetworks.com/2017/02/unit42-magic-hound-campaign-attacks-saudi-targets/)
-    [Asheer Malhotra & Vitor Ventura. (2022, August 2). Manjusaka: A Chinese sibling of Sliver and Cobalt Strike. Retrieved September 4, 2024.](https://blog.talosintelligence.com/manjusaka-offensive-framework/)
-    [GReAT. (2021, June 16). Ferocious Kitten: 6 Years of Covert Surveillance in Iran. Retrieved September 22, 2021.](https://securelist.com/ferocious-kitten-6-years-of-covert-surveillance-in-iran/102806/)
-    [ClearSky Cyber Security and Trend Micro. (2017, July). Operation Wilted Tulip: Exposing a cyber espionage apparatus. Retrieved August 21, 2017.](http://www.clearskysec.com/wp-content/uploads/2017/07/Operation_Wilted_Tulip.pdf)
-    [Minerva Labs LTD and ClearSky Cyber Security. (2015, November 23). CopyKittens Attack Group. Retrieved November 17, 2024.](https://cdn2.hubspot.net/hubfs/1903456/Whitepapers/CopyKittens.pdf)
-    [SentinelLabs. (2022, September 22). Metador Technical Appendix. Retrieved April 4, 2023.](https://docs.google.com/document/d/1e9ZTW9b71YwFWS_18ZwDAxa-cYbV8q1wUefmKZLYVsA/edit#heading=h.lmnbtht1ikzm)
-    [Sierra, E., Iglesias, G.. (2018, April 24). Metamorfo Campaigns Targeting Brazilian Users. Retrieved July 30, 2020.](https://www.fireeye.com/blog/threat-research/2018/04/metamorfo-campaign-targeting-brazilian-users.html)
-    [ESET Research. (2019, October 3). Casbaneiro: peculiarities of this banking Trojan that affects Brazil and Mexico. Retrieved September 23, 2021.](https://www.welivesecurity.com/2019/10/03/casbaneiro-trojan-dangerous-cooking/)
-    [Tsarfaty, Y. (2018, July 25). Micropsia Malware. Retrieved November 13, 2018.](https://www.radware.com/blog/security/2018/07/micropsia-malware/)
-    [SCILabs. (2021, December 23). Cyber Threat Profile Malteiro. Retrieved March 13, 2024.](https://blog.scilabs.mx/en/cyber-threat-profile-malteiro/)
-    [SCILabs. (2023, May 23). Evolution of banking trojan URSA/Mispadu. Retrieved March 13, 2024.](https://blog.scilabs.mx/en/evolution-of-banking-trojan-ursa-mispadu/)
-    [ESET Security. (2019, November 19). Mispadu: Advertisement for a discounted Unhappy Meal. Retrieved March 13, 2024.](https://www.welivesecurity.com/2019/11/19/mispadu-advertisement-discounted-unhappy-meal/)
-    [Garcia, F., Regalado, D. (2023, March 7). Inside Mispadu massive infection campaign in LATAM. Retrieved March 15, 2024.](https://www.metabaseq.com/mispadu-banking-trojan/)
-    [Faou, M. (2023, August 10). MoustachedBouncer: Espionage against foreign diplomats in Belarus. Retrieved September 25, 2023.](https://www.welivesecurity.com/en/eset-research/moustachedbouncer-espionage-against-foreign-diplomats-in-belarus/)
-    [Kaspersky Lab's Global Research & Analysis Team. (2018, October 10). MuddyWater expands operations. Retrieved November 2, 2018.](https://securelist.com/muddywater/88059/)
-    [McAfee. (2015, March 2). Netwire RAT Behind Recent Targeted Attacks. Retrieved February 15, 2018.](https://securingtomorrow.mcafee.com/mcafee-labs/netwire-rat-behind-recent-targeted-attacks/)
-    [Maniath, S. and Kadam P. (2019, March 19). Dissecting a NETWIRE Phishing Campaign's Usage of Process Hollowing. Retrieved January 7, 2021.](https://www.mandiant.com/resources/blog/dissecting-netwire-phishing-campaigns-usage-process-hollowing)
-    [Lambert, T. (2020, January 29). Intro to Netwire. Retrieved January 7, 2021.](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)
-    [Proofpoint. (2020, December 2). Geofenced NetWire Campaigns. Retrieved January 7, 2021.](https://www.proofpoint.com/us/blog/threat-insight/geofenced-netwire-campaigns)
-    [Pascual, C. (2018, November 27). AutoIt-Compiled Worm Affecting Removable Media Delivers Fileless Version of BLADABINDI/njRAT Backdoor. Retrieved June 4, 2019.](https://blog.trendmicro.com/trendlabs-security-intelligence/autoit-compiled-worm-affecting-removable-media-delivers-fileless-version-of-bladabindi-njrat-backdoor/)
-    [Global Research & Analysis Team, Kaspersky. (2024, August 19). BlindEagle flying high in Latin America. Retrieved April 16, 2026.](https://securelist.com/blindeagle-apt/113414/)
-    [KASPERSKY GERT. (2023, December 14). Unveiling NKAbuse: a new multiplatform threat abusing the NKN protocol. Retrieved February 8, 2024.](https://securelist.com/unveiling-nkabuse/111512/)
-    [Malhotra, A. (2021, March 2). ObliqueRAT returns with new campaign using hijacked websites. Retrieved September 2, 2021.](https://blog.talosintelligence.com/2021/02/obliquerat-new-campaign.html)
-    [Kaspersky Lab's Global Research & Analysis Team. (2018, October 15). Octopus-infested seas of Central Asia. Retrieved November 14, 2018.](https://securelist.com/octopus-infested-seas-of-central-asia/88200/)
-    [Paganini, P. (2018, October 16). Russia-linked APT group DustSquad targets diplomatic entities in Central Asia. Retrieved August 24, 2021.](https://securityaffairs.co/wordpress/77165/apt/russia-linked-apt-dustsquad.html)
-    [Cherepanov, A. (2018, October 4). Nomadic Octopus Cyber espionage in Central Asia. Retrieved October 13, 2021.](https://www.virusbulletin.com/uploads/pdf/conference_slides/2018/Cherepanov-VB2018-Octopus.pdf)
-    [Davis, S. and Caban, D. (2017, December 19). APT34 - New Targeted Attack in the Middle East. Retrieved December 20, 2017.](https://www.brighttalk.com/webcast/10703/296317/apt34-new-targeted-attack-in-the-middle-east)
-    [Computer Incident Response Center Luxembourg. (2013, March 29). Analysis of a PlugX variant. Retrieved November 5, 2018.](http://circl.lu/assets/files/tr-12/tr-12-circl-plugx-analysis-v1.pdf)
-    [Mercer, W, et al. (2020, April 16). PoetRAT: Python RAT uses COVID-19 lures to target Azerbaijan public and private sectors. Retrieved April 27, 2020.](https://blog.talosintelligence.com/2020/04/poetrat-covid-19-lures.html)
-    [Dragos. (n.d.). ICS Cybersecurity Year in Review 2020. Retrieved February 25, 2021.](https://hub.dragos.com/hubfs/Year-in-Review/Dragos_2020_ICS_Cybersecurity_Year_In_Review.pdf?hsCtaTracking=159c0fc3-92d8-425d-aeb8-12824f2297e8%7Cf163726d-579b-4996-9a04-44e5a124d770)
-    [PowerShellMafia. (2012, May 26). PowerSploit - A PowerShell Post-Exploitation Framework. Retrieved February 6, 2018.](https://github.com/PowerShellMafia/PowerSploit)
-    [PowerSploit. (n.d.). PowerSploit. Retrieved February 6, 2018.](http://powersploit.readthedocs.io)
-    [Singh, S. et al.. (2018, March 13). Iranian Threat Group Updates Tactics, Techniques and Procedures in Spear Phishing Campaign. Retrieved April 11, 2018.](https://www.fireeye.com/blog/threat-research/2018/03/iranian-threat-group-updates-ttps-in-spear-phishing-campaign.html)
-    [Lunghi, D. and Horejsi, J.. (2019, June 10). MuddyWater Resurfaces, Uses Multi-Stage Backdoor POWERSTATS V3 and New Post-Exploitation Tools. Retrieved May 14, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/muddywater-resurfaces-uses-multi-stage-backdoor-powerstats-v3-and-new-post-exploitation-tools/)
-    [Sardiwal, M, et al. (2017, December 7). New Targeted Attack in the Middle East by APT34, a Suspected Iranian Threat Group, Using CVE-2017-11882 Exploit. Retrieved December 20, 2017.](https://www.fireeye.com/blog/threat-research/2017/12/targeted-attack-in-middle-east-by-apt34.html)
-    [Cherepanov, A.. (2016, May 17). Operation Groundbait: Analysis of a surveillance toolkit. Retrieved May 18, 2016.](http://www.welivesecurity.com/wp-content/uploads/2016/05/Operation-Groundbait.pdf)
-    [Kasza, A. and Reichel, D. (2017, February 27). The Gamaredon Group Toolset Evolution. Retrieved March 1, 2017.](https://researchcenter.paloaltonetworks.com/2017/02/unit-42-title-gamaredon-group-toolset-evolution/)
-    [Unit 42. (2022, February 3). Russia’s Gamaredon aka Primitive Bear APT Group Actively Targeting Ukraine. Retrieved February 21, 2022.](https://unit42.paloaltonetworks.com/gamaredon-primitive-bear-ukraine-update-2021/)
-    [Nicolas Verdier. (n.d.). Retrieved January 29, 2018.](https://github.com/n1nj4sec/pupy)
-    [Microsoft. (2024, September 4). Use Quick Assist to help users. Retrieved March 14, 2025.](https://learn.microsoft.com/en-us/windows/client-management/client-tools/quick-assist)
-    [Microsoft Threat Intelligence Center. (2022, February 4). ACTINIUM targets Ukrainian organizations. Retrieved February 18, 2022.](https://www.microsoft.com/security/blog/2022/02/04/actinium-targets-ukrainian-organizations/)
-    [S2W TALON. (2022, June 16). Raccoon Stealer is Back with a New Version. Retrieved August 1, 2024.](https://medium.com/s2wblog/raccoon-stealer-is-back-with-a-new-version-5f436e04b20d)
-    [Pierre Le Bourhis, Quentin Bourgue, & Sekoia TDR. (2022, June 29). Raccoon Stealer v2 - Part 2: In-depth analysis. Retrieved August 1, 2024.](https://blog.sekoia.io/raccoon-stealer-v2-part-2-in-depth-analysis/)
-    [Vrabie, V. (2021, April 23). NAIKON – Traces from a Military Cyber-Espionage Operation. Retrieved June 29, 2021.](https://www.bitdefender.com/files/News/CaseStudies/study/396/Bitdefender-PR-Whitepaper-NAIKON-creat5397-en-EN.pdf)
-    [Antiy CERT. (2020, April 20). Analysis of Ramsay components of Darkhotel's infiltration and isolation network. Retrieved March 24, 2021.](https://www.programmersought.com/article/62493896999/)
-    [Global Threat Center, Intelligence Team. (2020, December). APT27 Turns to Ransomware. Retrieved November 12, 2021.](https://web.archive.org/web/20210104144857/https://shared-public-reports.s3-eu-west-1.amazonaws.com/APT27+turns+to+ransomware.pdf)
-    [Falcone, R. (2020, July 22). OilRig Targets Middle Eastern Telecommunications Organization and Adds Novel C2 Channel with Steganography to Its Inventory. Retrieved July 28, 2020.](https://unit42.paloaltonetworks.com/oilrig-novel-c2-channel-steganography/)
-    [FireEye iSIGHT Intelligence. (2017, April 6). APT10 (MenuPass Group): New Tools, Global Campaign Latest Manifestation of Longstanding Threat. Retrieved June 29, 2017.](https://www.fireeye.com/blog/threat-research/2017/04/apt10_menupass_grou.html)
-    [Accenture Security. (2018, April 23). Hogfish Redleaves Campaign. Retrieved July 2, 2018.](http://web.archive.org/web/20220810112638/https:/www.accenture.com/t20180423T055005Z_w_/se-en/_acnmedia/PDF-76/Accenture-Hogfish-Threat-Analysis.pdf)
-    [Mohansundaram M, Neil Tyagi. (2024, April 17). Redline Stealer: A Novel Approach. Retrieved September 17, 2025.](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/redline-stealer-a-novel-approach/)
-    [Splunk Threat Research Team. (2023, June 1). Do Not Cross The 'RedLine' Stealer: Detections and Analysis. Retrieved September 17, 2025.](https://www.splunk.com/en_us/blog/security/do-not-cross-the-redline-stealer-detections-and-analysis.html)
-    [Klijnsma, Y. (2018, January 23). Espionage Campaign Leverages Spear Phishing, RATs Against Turkish Defense Contractors. Retrieved November 6, 2018.](https://web.archive.org/web/20180124082756/https://www.riskiq.com/blog/labs/spear-phishing-turkish-defense-contractors/)
-    [Zhang, X. (2024, November 8). New Campaign Uses Remcos RAT to Exploit Victims. Retrieved April 16, 2026.](https://www.fortinet.com/blog/threat-research/new-campaign-uses-remcos-rat-to-exploit-victims)
-    [Legezo, D. (2019, January 30). Chafer used Remexi malware to spy on Iran-based foreign diplomatic entities. Retrieved April 17, 2019.](https://securelist.com/chafer-used-remexi-malware/89538/)
-    [Peretz, A. and Theck, E. (2021, March 5). Earth Vetala – MuddyWater Continues to Target Organizations in the Middle East. Retrieved March 18, 2021.](https://www.trendmicro.com/en_us/research/21/c/earth-vetala---muddywater-continues-to-target-organizations-in-t.html)
-    [Livelli, K, et al. (2018, November 12). Operation Shaheen. Retrieved May 1, 2019.](https://www.cylance.com/content/dam/cylance-web/en-us/resources/knowledge-center/resource-library/reports/WhiteCompanyOperationShaheenReport.pdf?_ga=2.161661948.1943296560.1555683782-1066572390.1555511517)
-    [Falcone, R., et al. (2018, July 27). New Threat Actor Group DarkHydrus Targets Middle East Government. Retrieved August 2, 2018.](https://researchcenter.paloaltonetworks.com/2018/07/unit42-new-threat-actor-group-darkhydrus-targets-middle-east-government/)
-    [Mercer, W., Rascagneres, P. (2017, April 03). Introducing ROKRAT. Retrieved May 21, 2018.](https://blog.talosintelligence.com/2017/04/introducing-rokrat.html)
-    [Mercer, W., Rascagneres, P. (2017, November 28). ROKRAT Reloaded. Retrieved May 21, 2018.](https://blog.talosintelligence.com/2017/11/ROKRAT-Reloaded.html)
-    [GReAT. (2019, May 13). ScarCruft continues to evolve, introduces Bluetooth harvester. Retrieved June 4, 2019.](https://securelist.com/scarcruft-continues-to-evolve-introduces-bluetooth-harvester/90729/)
-    [Pantazopoulos, N.. (2018, November 8). RokRat Analysis. Retrieved May 21, 2020.](https://research.nccgroup.com/2018/11/08/rokrat-analysis/)
-    [Jazi, Hossein. (2021, January 6). Retrohunting APT37: North Korean APT used VBA self decode technique to inject RokRat. Retrieved March 22, 2022.](https://blog.malwarebytes.com/threat-analysis/2021/01/retrohunting-apt37-north-korean-apt-used-vba-self-decode-technique-to-inject-rokrat/)
-    [Ray, V., Hayashi, K. (2016, February 29). New Malware ‘Rover’ Targets Indian Ambassador to Afghanistan. Retrieved February 29, 2016.](http://researchcenter.paloaltonetworks.com/2016/02/new-malware-rover-targets-indian-ambassador-to-afghanistan/)
-    [Faou, M. and Boutin, J. (2017, February). Read The Manual: A Guide to the RTM Banking Trojan. Retrieved March 9, 2017.](https://www.welivesecurity.com/wp-content/uploads/2017/02/Read-The-Manual.pdf)
-    [Duncan, B., Harbison, M. (2019, January 23). Russian Language Malspam Pushing Redaman Banking Malware. Retrieved June 16, 2020.](https://unit42.paloaltonetworks.com/russian-language-malspam-pushing-redaman-banking-malware/)
-    [Cybereason Nocturnus Team. (2020, December 9). MOLERATS IN THE CLOUD: New Malware Arsenal Abuses Cloud Platforms in Middle East Espionage Campaign. Retrieved December 22, 2020.](https://www.cybereason.com/hubfs/dam/collateral/reports/Molerats-in-the-Cloud-New-Malware-Arsenal-Abuses-Cloud-Platforms-in-Middle-East-Espionage-Campaign.pdf)
-    [Ilascu, I. (2020, December 14). Hacking group’s new malware abuses Google and Facebook services. Retrieved December 28, 2020.](https://www.bleepingcomputer.com/news/security/hacking-group-s-new-malware-abuses-google-and-facebook-services/)
-    [El-Banna, M., et al. (2025, November 17). Frontline Intelligence: Analysis of UNC1549 TTPs, Custom Tools, and Malware Targeting the Aerospace and Defense Ecosystem. Retrieved September 28, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)
-    [GReAT. (2017, November 1). Silence – a new Trojan attacking financial organizations. Retrieved May 24, 2019.](https://securelist.com/the-silence/83009/)
-    [Group-IB. (2018, September). Silence: Moving Into the Darkside. Retrieved May 5, 2020.](https://go.group-ib.com/report-silence-en?_gl=1*d1bh3a*_ga*MTIwMzM5Mzc5MS4xNjk4OTI5NzY4*_ga_QMES53K3Y2*MTcwNDcyMjU2OS40LjEuMTcwNDcyMzU1Mi41My4wLjA.)
-    [Salvati, M. (2019, August 6). SILENTTRINITY Modules. Retrieved March 24, 2022.](https://github.com/byt3bl33d3r/SILENTTRINITY/tree/master/silenttrinity/core/teamserver/modules/boo)
-    [BishopFox. (n.d.). Sliver Screenshot. Retrieved September 16, 2021.](https://github.com/BishopFox/sliver/blob/master/implant/sliver/screen/screenshot_windows.go)
-    [DHS/CISA, Cyber National Mission Force. (2020, October 1). Malware Analysis Report (MAR) MAR-10303705-1.v1 – Remote Access Trojan: SLOTHFULMEDIA. Retrieved October 2, 2020.](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-275a)
-    [FireEye. (2021, May 11). Shining a Light on DARKSIDE Ransomware Operations. Retrieved September 22, 2021.](https://www.fireeye.com/blog/threat-research/2021/05/shining-a-light-on-darkside-ransomware-operations.html)
-    [FireEye. (2021, June 16). Smoking Out a DARKSIDE Affiliate’s Supply Chain Software Compromise. Retrieved September 22, 2021.](https://www.fireeye.com/blog/threat-research/2021/06/darkside-affiliate-supply-chain-software-compromise.html)
-    [Lunghi, D., et al. (2017, December). Untangling the Patchwork Cyberespionage Group. Retrieved July 10, 2018.](https://documents.trendmicro.com/assets/tech-brief-untangling-the-patchwork-cyberespionage-group.pdf)
-    [Kaspersky Lab. (2017, March 7). From Shamoon to StoneDrill: Wipers attacking Saudi organizations and beyond. Retrieved March 14, 2019.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07180722/Report_Shamoon_StoneDrill_final.pdf)
-    [Cybereason Nocturnus. (2022, February 1). StrifeWater RAT: Iranian APT Moses Staff Adds New Trojan to Ransomware Operations. Retrieved August 15, 2022.](https://www.cybereason.com/blog/research/strifewater-rat-iranian-apt-moses-staff-adds-new-trojan-to-ransomware-operations)
-    [Schlapfer, Patrick. (2022, June 6). A New Loader Gets Ready. Retrieved December 13, 2022.](https://threatresearch.ext.hp.com/svcready-a-new-loader-reveals-itself/)
-    [Lunghi, D. and Lu, K. (2021, April 9). Iron Tiger APT Updates Toolkit With Evolved SysUpdate Malware. Retrieved November 12, 2021.](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)
-    [Grunzweig, J. and Miller-Osborn, J.. (2016, February 4). T9000: Advanced Modular Backdoor Uses Complex Anti-Analysis Techniques. Retrieved April 15, 2016.](http://researchcenter.paloaltonetworks.com/2016/02/t9000-advanced-modular-backdoor-uses-complex-anti-analysis-techniques/)
-    [GReAT. (2019, April 10). Project TajMahal – a sophisticated new APT framework. Retrieved October 14, 2019.](https://securelist.com/project-tajmahal/90240/)
-    [Cylance. (2014, December). Operation Cleaver. Retrieved September 14, 2017.](https://web.archive.org/web/20200302085133/https://www.cylance.com/content/dam/cylance/pages/operation-cleaver/Cylance_Operation_Cleaver_Report.pdf)
-    [Lior Rochberger, Tom Fakterman, Robert Falcone. (2023, September 22). Cyberespionage Attacks Against Southeast Asian Government Linked to Stately Taurus, Aka Mustang Panda. Retrieved September 9, 2025.](https://unit42.paloaltonetworks.com/stately-taurus-attacks-se-asian-government/)
-    [Symantec Security Response. (2014, June 30). Dragonfly: Cyberespionage Attacks Against Energy Suppliers. Retrieved April 8, 2016.](https://community.broadcom.com/symantecenterprise/communities/community-home/librarydocuments/viewdocument?DocumentKey=7382dce7-0260-4782-84cc-890971ed3f17&CommunityKey=1ecf5f55-9545-44d6-b0f4-4e4a7f5f5e68&tab=librarydocuments)
-    [Secureworks. (2019, July 24). Updated Karagany Malware Targets Energy Sector. Retrieved August 12, 2020.](https://www.secureworks.com/research/updated-karagany-malware-targets-energy-sector)
-    [Jiho Kim & Sebin Lee, S2W. (2024, February 7). Kimsuky disguised as a Korean company signed with a valid certificate to distribute Troll Stealer (English ver.). Retrieved January 17, 2025.](https://medium.com/s2wblog/kimsuky-disguised-as-a-korean-company-signed-with-a-valid-certificate-to-distribute-troll-stealer-cfa5d54314e2)
-    [Symantec Threat Hunter Team. (2024, May 16). Springtail: New Linux Backdoor Added to Toolkit. Retrieved January 17, 2025.](https://www.security.com/threat-intelligence/springtail-kimsuky-backdoor-espionage)
-    [Adam Burgher. (2021, June 10). BackdoorDiplomacy: Upgrading from Quarian to Turian. Retrieved September 1, 2021](https://www.welivesecurity.com/2021/06/10/backdoordiplomacy-upgrading-quarian-turian/)
-    [O'Leary, J., et al. (2017, September 20). Insights into Iranian Cyber Espionage: APT33 Targets Aerospace and Energy Sectors and has Ties to Destructive Malware. Retrieved February 15, 2018.](https://www.fireeye.com/blog/threat-research/2017/09/apt33-insights-into-iranian-cyber-espionage.html)
-    [Matsuda, A., Muhammad I. (2018, September 13). APT10 Targeting Japanese Corporations Using Updated TTPs. Retrieved September 17, 2018.](https://www.fireeye.com/blog/threat-research/2018/09/apt10-targeting-japanese-corporations-using-updated-ttps.html)
-    [Hiroaki, H. (2025, April 30). Earth Kasha Updates TTPs in Latest Campaign Targeting Taiwan and Japan. Retrieved April 17, 2026.](https://www.trendmicro.com/en_us/research/25/d/earth-kasha-updates-ttps.html)
-    [Hiroaki, H. (2024, November 26). Guess Who’s Back - The Return of ANEL in the Recent Earth Kasha Spear-phishing Campaign in 2024. Retrieved April 17, 2026.](https://www.trendmicro.com/en_us/research/24/k/return-of-anel-in-the-recent-earth-kasha-spearphishing-campaign.html)
-    [Dominik Breitenbacher. (2025, March 18). Operation AkaiRyū: MirrorFace invites Europe to Expo 2025 and revives ANEL backdoor. Retrieved May 22, 2025.](https://www.welivesecurity.com/en/eset-research/operation-akairyu-mirrorface-invites-europe-expo-2025-revives-anel-backdoor/)
-    [Caragay, R. (2015, March 26). URSNIF: The Multifaceted Malware. Retrieved June 5, 2019.](https://web.archive.org/web/20210719165945/https://www.trendmicro.com/en_us/research/15/c/ursnif-the-multifaceted-malware.html?_ga=2.165628854.808042651.1508120821-744063452.1505819992)
-    [Sioting, S. (2013, June 15). BKDR_URSNIF.SM. Retrieved June 5, 2019.](https://www.trendmicro.com/vinfo/us/threat-encyclopedia/malware/BKDR_URSNIF.SM?_ga=2.129468940.1462021705.1559742358-1202584019.1549394279)
-    [Salem, E. et al. (2020, May 28). VALAK: MORE THAN MEETS THE EYE . Retrieved June 19, 2020.](https://www.cybereason.com/blog/valak-more-than-meets-the-eye)
-    [Lancaster, T., Cortes, J. (2018, January 29). VERMIN: Quasar RAT and Custom Malware Used In Ukraine. Retrieved July 5, 2018.](https://researchcenter.paloaltonetworks.com/2018/01/unit42-vermin-quasar-rat-custom-malware-used-ukraine/)
-    [FBI. (2026, March 20). FBI Flash: FLASH-20260320-001:Government of Iran Cyber Actors Deploy Telegram C2 to Push Malware to Identified Targets. Retrieved April 20, 2026.](https://www.ic3.gov/CSA/2026/260320.pdf)
-    [CISA et al.. (2024, February 7). PRC State-Sponsored Actors Compromise and Maintain Persistent Access to U.S. Critical Infrastructure. Retrieved May 15, 2024.](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)
-    [CERT-UA. (2023, February 1). UAC-0114 aka Winter Vivern to target Ukrainian and Polish GOV entities (CERT-UA#5909). Retrieved July 29, 2024.](https://cert.gov.ua/article/3761104)
-    [MalwareBytes Threat Intelligence Team. (2022, August 3). Woody RAT: A new feature-rich malware spotted in the wild. Retrieved December 6, 2022.](https://www.malwarebytes.com/blog/threat-intelligence/2022/08/woody-rat-a-new-feature-rich-malware-spotted-in-the-wild)
-    [Mac Threat Response, Mobile Research Team. (2020, August 13). The XCSSET Malware: Inserts Malicious Code Into Xcode Projects, Performs UXSS Backdoor Planting in Safari, and Leverages Two Zero-day Exploits. Retrieved October 5, 2021.](https://documents.trendmicro.com/assets/pdf/XCSSET_Technical_Brief.pdf)
-    [Nart Villeneuve, Randi Eitzman, Sandor Nemes & Tyler Dean, Google Cloud. (2017, October 5). Significant FormBook Distribution Campaigns Impacting the U.S. and South Korea. Retrieved March 11, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/formbook-malware-distribution-campaigns/)
-    [Gustavo Palazolo, Netskope. (2022, March 11). New Formbook Campaign Delivered Through Phishing Emails. Retrieved March 11, 2025.](https://www.netskope.com/blog/new-formbook-campaign-delivered-through-phishing-emails)
-    [Schwarz, D., Sopko J. (2018, March 08). Donot Team Leverages New Modular Malware Framework in South Asia. Retrieved June 11, 2018.](https://www.arbornetworks.com/blog/asert/donot-team-leverages-new-modular-malware-framework-south-asia/)
-    [ESET. (2018, November 20). Sednit: What’s going on with Zebrocy?. Retrieved February 12, 2019.](https://www.welivesecurity.com/2018/11/20/sednit-whats-going-zebrocy/)
-    [Lee, B., Falcone, R. (2018, December 12). Dear Joohn: The Sofacy Group’s Global Campaign. Retrieved April 19, 2019.](https://unit42.paloaltonetworks.com/dear-joohn-sofacy-groups-global-campaign/)
-    [ESET Research. (2019, May 22). A journey to Zebrocy land. Retrieved June 20, 2019.](https://www.welivesecurity.com/2019/05/22/journey-zebrocy-land/)
-    [Accenture Security. (2018, November 29). SNAKEMACKEREL. Retrieved April 15, 2019.](https://www.accenture.com/t20181129T203820Z__w__/us-en/_acnmedia/PDF-90/Accenture-snakemackerel-delivers-zekapab-malware.pdf#zoom=50)
-    [CISA. (2020, October 29). Malware Analysis Report (AR20-303B). Retrieved December 9, 2020.](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-303b)
-    [Ebach, L. (2017, June 22). Analysis Results of Zeus.Variant.Panda. Retrieved November 5, 2018.](https://cyberwtf.files.wordpress.com/2017/07/panda-whitepaper.pdf)
-    [Gross, J. (2016, February 23). Operation Dust Storm. Retrieved December 22, 2021.](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)
-    [Fraser, N., et al. (2019, August 7). Double DragonAPT41, a dual espionage and cyber crime operation APT41. Retrieved September 23, 2019.](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)

×
