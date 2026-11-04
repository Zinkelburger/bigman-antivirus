---
url: "https://attack.mitre.org/techniques/T1027/002/"
title: "Obfuscated Files or Information: Software Packing, Sub-technique T1027.002 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:25:51Z"
via: "live"
sha256: "2a8cf7523024f88c1a680169c2d3e7266eeeeacb9e276bc178980d23a141ee2f"
wayback: "https://web.archive.org/web/20261023090817/https://attack.mitre.org/techniques/T1027/002/"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# Obfuscated Files or Information: Software Packing, Sub-technique T1027.002 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1027/002/>
> Snapshot: <https://web.archive.org/web/20261023090817/https://attack.mitre.org/techniques/T1027/002/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Obfuscated Files or Information](https://attack.mitre.org/techniques/T1027)
- Software Packing

#  Obfuscated Files or Information: Software Packing

##### Other sub-techniques of Obfuscated Files or Information (18)

| ID  | Name

|  [T1027.001](https://attack.mitre.org/techniques/T1027/001/)   |  [Binary Padding](https://attack.mitre.org/techniques/T1027/001/)

|  T1027.002   |  Software Packing

|  [T1027.003](https://attack.mitre.org/techniques/T1027/003/)   |  [Steganography](https://attack.mitre.org/techniques/T1027/003/)

|  [T1027.004](https://attack.mitre.org/techniques/T1027/004/)   |  [Compile After Delivery](https://attack.mitre.org/techniques/T1027/004/)

|  [T1027.005](https://attack.mitre.org/techniques/T1027/005/)   |  [Indicator Removal from Tools](https://attack.mitre.org/techniques/T1027/005/)

|  [T1027.006](https://attack.mitre.org/techniques/T1027/006/)   |  [HTML Smuggling](https://attack.mitre.org/techniques/T1027/006/)

|  [T1027.007](https://attack.mitre.org/techniques/T1027/007/)   |  [Dynamic API Resolution](https://attack.mitre.org/techniques/T1027/007/)

|  [T1027.008](https://attack.mitre.org/techniques/T1027/008/)   |  [Stripped Payloads](https://attack.mitre.org/techniques/T1027/008/)

|  [T1027.009](https://attack.mitre.org/techniques/T1027/009/)   |  [Embedded Payloads](https://attack.mitre.org/techniques/T1027/009/)

|  [T1027.010](https://attack.mitre.org/techniques/T1027/010/)   |  [Command Obfuscation](https://attack.mitre.org/techniques/T1027/010/)

|  [T1027.011](https://attack.mitre.org/techniques/T1027/011/)   |  [Fileless Storage](https://attack.mitre.org/techniques/T1027/011/)

|  [T1027.012](https://attack.mitre.org/techniques/T1027/012/)   |  [LNK Icon Smuggling](https://attack.mitre.org/techniques/T1027/012/)

|  [T1027.013](https://attack.mitre.org/techniques/T1027/013/)   |  [Encrypted/Encoded File](https://attack.mitre.org/techniques/T1027/013/)

|  [T1027.014](https://attack.mitre.org/techniques/T1027/014/)   |  [Polymorphic Code](https://attack.mitre.org/techniques/T1027/014/)

|  [T1027.015](https://attack.mitre.org/techniques/T1027/015/)   |  [Compression](https://attack.mitre.org/techniques/T1027/015/)

|  [T1027.016](https://attack.mitre.org/techniques/T1027/016/)   |  [Junk Code Insertion](https://attack.mitre.org/techniques/T1027/016/)

|  [T1027.017](https://attack.mitre.org/techniques/T1027/017/)   |  [SVG Smuggling](https://attack.mitre.org/techniques/T1027/017/)

|  [T1027.018](https://attack.mitre.org/techniques/T1027/018/)   |  [Invisible Unicode](https://attack.mitre.org/techniques/T1027/018/)

Adversaries may perform software packing or virtual machine software protection to conceal their code. Software packing is a method of compressing or encrypting an executable. Packing an executable changes the file signature in an attempt to avoid signature-based detection. Most decompression techniques decompress the executable code in memory. Virtual machine software protection translates an executable's original code into a special format that only a special virtual machine can run. A virtual machine is then called to run this code.[[1]](https://www.welivesecurity.com/wp-content/uploads/2018/01/WP-FinFisher.pdf)

Utilities used to perform software packing are called packers. Example packers are MPRESS and UPX. A more comprehensive list of known packers is available, but adversaries may create their own packing techniques that do not leave the same artifacts as well-known packers to evade defenses.[[2]](https://github.com/dhondta/awesome-executable-packing)

ID: T1027.002

Sub-technique of:  [T1027](https://attack.mitre.org/techniques/T1027)

ⓘ

Tactic: [Stealth](https://attack.mitre.org/tactics/TA0005)

ⓘ

Platforms: Linux, Windows, macOS

Contributors: Filip Kafka, ESET

Version: 2.0

Created: 05 February 2020

Last Modified: 11 September 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1027/002/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1027/002/)

## Procedure Examples

| ID  | Name  | Description

|  [C0025](https://attack.mitre.org/campaigns/C0025)   |  [2016 Ukraine Electric Power Attack](https://attack.mitre.org/campaigns/C0025)   |

During the [2016 Ukraine Electric Power Attack](https://attack.mitre.org/campaigns/C0025), [Sandworm Team](https://attack.mitre.org/groups/G0034) used UPX to pack a copy of [Mimikatz](https://attack.mitre.org/software/S0002).[[3]](https://www.dragos.com/wp-content/uploads/CRASHOVERRIDE2018.pdf)

|  [S0504](https://attack.mitre.org/software/S0504)   |  [Anchor](https://attack.mitre.org/software/S0504)   |

[Anchor](https://attack.mitre.org/software/S0504) has come with a packed payload.[[4]](https://www.cybereason.com/blog/dropping-anchor-from-a-trickbot-infection-to-the-discovery-of-the-anchor-malware)

|  [G1007](https://attack.mitre.org/groups/G1007)   |  [Aoqin Dragon](https://attack.mitre.org/groups/G1007)   |

[Aoqin Dragon](https://attack.mitre.org/groups/G1007) has used the Themida packer to obfuscate malicious payloads.[[5]](https://www.sentinelone.com/labs/aoqin-dragon-newly-discovered-chinese-linked-apt-has-been-quietly-spying-on-organizations-for-10-years/)

|  [S0622](https://attack.mitre.org/software/S0622)   |  [AppleSeed](https://attack.mitre.org/software/S0622)   |

[AppleSeed](https://attack.mitre.org/software/S0622) has used UPX packers for its payload DLL.[[6]](https://blog.malwarebytes.com/threat-analysis/2021/06/kimsuky-apt-continues-to-target-south-korean-government-using-appleseed-backdoor/)

|  [G0016](https://attack.mitre.org/groups/G0016)   |  [APT29](https://attack.mitre.org/groups/G0016)   |

[APT29](https://attack.mitre.org/groups/G0016) used UPX to pack files.[[7]](https://www.slideshare.net/slideshow/no-easy-breach-derby-con-2016/66447908)

|  [G0022](https://attack.mitre.org/groups/G0022)   |  [APT3](https://attack.mitre.org/groups/G0022)   |

[APT3](https://attack.mitre.org/groups/G0022) has been known to pack their tools.[[8]](https://attack.mitre.org/docs/APT3_Adversary_Emulation_Plan.pdf)[[9]](https://www.fireeye.com/blog/threat-research/2015/06/operation-clandestine-wolf-adobe-flash-zero-day.html)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has used several code packing methods such as Themida, Enigma, VMProtect, and Obsidium, to pack their implants.[[10]](https://services.google.com/fh/files/misc/apt38-un-usual-suspects.pdf)

|  [G0087](https://attack.mitre.org/groups/G0087)   |  [APT39](https://attack.mitre.org/groups/G0087)   |

[APT39](https://attack.mitre.org/groups/G0087) has packed tools with UPX, and has repacked a modified version of [Mimikatz](https://attack.mitre.org/software/S0002) to thwart anti-virus detection.[[11]](https://www.fireeye.com/blog/threat-research/2019/01/apt39-iranian-cyber-espionage-group-focused-on-personal-information.html)[[12]](https://www.bitdefender.com/blog/labs/iranian-chafer-apt-targeted-air-transportation-and-government-in-kuwait-and-saudi-arabia/)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) uses packers such as Themida to obfuscate malicious files.[[13]](https://www.group-ib.com/blog/apt41-world-tour-2021/)

|  [S0373](https://attack.mitre.org/software/S0373)   |  [Astaroth](https://attack.mitre.org/software/S0373)   |

[Astaroth](https://attack.mitre.org/software/S0373) uses a software packer called Pe123\RPolyCryptor.[[14]](https://www.cybereason.com/blog/information-stealing-malware-targeting-brazil-full-research)

|  [S0638](https://attack.mitre.org/software/S0638)   |  [Babuk](https://attack.mitre.org/software/S0638)   |

Versions of [Babuk](https://attack.mitre.org/software/S0638) have been packed.[[15]](https://www.sogeti.com/globalassets/reports/cybersecchronicles_-_babuk.pdf)[[16]](https://www.mcafee.com/enterprise/en-us/assets/reports/rp-babuk-ransomware.pdf)[[17]](https://sebdraven.medium.com/babuk-is-distributed-packed-78e2f5dd2e62)

|  [S0534](https://attack.mitre.org/software/S0534)   |  [Bazar](https://attack.mitre.org/software/S0534)   |

[Bazar](https://attack.mitre.org/software/S0534) has a variant with a packed payload.[[18]](https://www.cybereason.com/blog/a-bazar-of-tricks-following-team9s-development-cycles)[[19]](https://www.zscaler.com/blogs/research/spear-phishing-campaign-delivers-buer-and-bazar-malware)

|  [S0268](https://attack.mitre.org/software/S0268)   |  [Bisonal](https://attack.mitre.org/software/S0268)   |

[Bisonal](https://attack.mitre.org/software/S0268) has used the MPRESS packer and similar tools for obfuscation.[[20]](https://blog.talosintelligence.com/2020/03/bisonal-10-years-of-play.html)

|  [S0520](https://attack.mitre.org/software/S0520)   |  [BLINDINGCAN](https://attack.mitre.org/software/S0520)   |

[BLINDINGCAN](https://attack.mitre.org/software/S0520) has been packed with the UPX packer.[[21]](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-232a)

|  [C0017](https://attack.mitre.org/campaigns/C0017)   |  [C0017](https://attack.mitre.org/campaigns/C0017)   |

During [C0017](https://attack.mitre.org/campaigns/C0017), [APT41](https://attack.mitre.org/groups/G0096) used VMProtect to slow the reverse engineering of malicious binaries.[[22]](https://www.mandiant.com/resources/apt41-us-state-governments)

|  [S0020](https://attack.mitre.org/software/S0020)   |  [China Chopper](https://attack.mitre.org/software/S0020)   |

[China Chopper](https://attack.mitre.org/software/S0020)'s client component is packed with UPX.[[23]](https://www.fireeye.com/blog/threat-research/2013/08/breaking-down-the-china-chopper-web-shell-part-i.html)

|  [S0611](https://attack.mitre.org/software/S0611)   |  [Clop](https://attack.mitre.org/software/S0611)   |

[Clop](https://attack.mitre.org/software/S0611) has been packed to help avoid detection.[[24]](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/clop-ransomware/)[[25]](https://www.cybereason.com/blog/cybereason-vs.-clop-ransomware)

|  [S1105](https://attack.mitre.org/software/S1105)   |  [COATHANGER](https://attack.mitre.org/software/S1105)   |

The first stage of [COATHANGER](https://attack.mitre.org/software/S1105) is delivered as a packed file.[[26]](https://www.ncsc.nl/binaries/ncsc/documenten/publicaties/2024/februari/6/mivd-aivd-advisory-coathanger-tlp-clear/TLP-CLEAR+MIVD+AIVD+Advisory+COATHANGER.pdf)

|  [S0614](https://attack.mitre.org/software/S0614)   |  [CostaBricks](https://attack.mitre.org/software/S0614)   |

[CostaBricks](https://attack.mitre.org/software/S0614) can implement a custom-built virtual machine mechanism to obfuscate its code.[[27]](https://blogs.blackberry.com/en/2020/11/the-costaricto-campaign-cyber-espionage-outsourced)

|  [S0527](https://attack.mitre.org/software/S0527)   |  [CSPY Downloader](https://attack.mitre.org/software/S0527)   |

[CSPY Downloader](https://attack.mitre.org/software/S0527) has been packed with UPX.[[28]](https://www.cybereason.com/blog/back-to-the-future-inside-the-kimsuky-kgh-spyware-suite)

|  [S0625](https://attack.mitre.org/software/S0625)   |  [Cuba](https://attack.mitre.org/software/S0625)   |

[Cuba](https://attack.mitre.org/software/S0625) has a packed payload when delivered.[[29]](https://www.mcafee.com/enterprise/en-us/assets/reports/rp-cuba-ransomware.pdf)

|  [G0070](https://attack.mitre.org/groups/G0070)   |  [Dark Caracal](https://attack.mitre.org/groups/G0070)   |

[Dark Caracal](https://attack.mitre.org/groups/G0070) has used UPX to pack [Bandook](https://attack.mitre.org/software/S0234).[[30]](https://info.lookout.com/rs/051-ESQ-475/images/Lookout_Dark-Caracal_srr_20180118_us_v.1.0.pdf)

|  [S0334](https://attack.mitre.org/software/S0334)   |  [DarkComet](https://attack.mitre.org/software/S0334)   |

[DarkComet](https://attack.mitre.org/software/S0334) has the option to compress its payload using UPX or MPRESS.[[31]](https://blog.malwarebytes.com/threat-analysis/2012/06/you-dirty-rat-part-1-darkcomet/)

|  [S0187](https://attack.mitre.org/software/S0187)   |  [Daserf](https://attack.mitre.org/software/S0187)   |

A version of [Daserf](https://attack.mitre.org/software/S0187) uses the MPRESS packer.[[32]](http://blog.trendmicro.com/trendlabs-security-intelligence/redbaldknight-bronze-butler-daserf-backdoor-now-using-steganography/)

|  [S0281](https://attack.mitre.org/software/S0281)   |  [Dok](https://attack.mitre.org/software/S0281)   |

[Dok](https://attack.mitre.org/software/S0281) is packed with an UPX executable packer.[[33]](https://web.archive.org/web/20221007144948/http://www.hexed.in/2019/07/osxdok-analysis.html)

|  [S0695](https://attack.mitre.org/software/S0695)   |  [Donut](https://attack.mitre.org/software/S0695)   |

[Donut](https://attack.mitre.org/software/S0695) can generate packed code modules.[[34]](https://github.com/TheWover/donut)

|  [S0694](https://attack.mitre.org/software/S0694)   |  [DRATzarus](https://attack.mitre.org/software/S0694)   |

[DRATzarus](https://attack.mitre.org/software/S0694)'s dropper can be packed with UPX.[[35]](https://www.clearskysec.com/wp-content/uploads/2020/08/Dream-Job-Campaign.pdf)

|  [S0024](https://attack.mitre.org/software/S0024)   |  [Dyre](https://attack.mitre.org/software/S0024)   |

[Dyre](https://attack.mitre.org/software/S0024) has been delivered with encrypted resources and must be unpacked for execution.[[36]](https://blog.malwarebytes.com/threat-analysis/2015/11/a-technical-look-at-dyreza/)

|  [S0554](https://attack.mitre.org/software/S0554)   |  [Egregor](https://attack.mitre.org/software/S0554)   |

[Egregor](https://attack.mitre.org/software/S0554)'s payloads are custom-packed, archived and encrypted to prevent analysis.[[37]](https://digital.nhs.uk/cyber-alerts/2020/cc-3681#summary)[[38]](https://cybleinc.com/2020/10/31/egregor-ransomware-a-deep-dive-into-its-activities-and-techniques/)

|  [G0066](https://attack.mitre.org/groups/G0066)   |  [Elderwood](https://attack.mitre.org/groups/G0066)   |

[Elderwood](https://attack.mitre.org/groups/G0066) has packed malware payloads before delivery to victims.[[39]](https://web.archive.org/web/20190717233006/http:/www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/the-elderwood-project.pdf)

|  [S0367](https://attack.mitre.org/software/S0367)   |  [Emotet](https://attack.mitre.org/software/S0367)   |

[Emotet](https://attack.mitre.org/software/S0367) has used custom packers to protect its payloads.[[40]](https://documents.trendmicro.com/assets/white_papers/ExploringEmotetsActivities_Final.pdf)

|  [S0512](https://attack.mitre.org/software/S0512)   |  [FatDuke](https://attack.mitre.org/software/S0512)   |

[FatDuke](https://attack.mitre.org/software/S0512) has been regularly repacked by its operators to create large binaries and evade detection.[[41]](https://www.welivesecurity.com/wp-content/uploads/2019/10/ESET_Operation_Ghost_Dukes.pdf)

|  [S0182](https://attack.mitre.org/software/S0182)   |  [FinFisher](https://attack.mitre.org/software/S0182)   |

A [FinFisher](https://attack.mitre.org/software/S0182) variant uses a custom packer.[[42]](https://web.archive.org/web/20171222050934/http://www.finfisher.com/FinFisher/index.html)[[43]](https://securelist.com/blackoasis-apt-and-new-targeted-attacks-leveraging-zero-day-exploit/82732/)

|  [S0628](https://attack.mitre.org/software/S0628)   |  [FYAnti](https://attack.mitre.org/software/S0628)   |

[FYAnti](https://attack.mitre.org/software/S0628) has used ConfuserEx to pack its .NET module.[[44]](https://securelist.com/apt10-sophisticated-multi-layered-loader-ecipekac-discovered-in-a41apt-campaign/101519/)

|  [G0093](https://attack.mitre.org/groups/G0093)   |  [GALLIUM](https://attack.mitre.org/groups/G0093)   |

[GALLIUM](https://attack.mitre.org/groups/G0093) packed some payloads using different types of packers, both known and custom.[[45]](https://www.cybereason.com/blog/operation-soft-cell-a-worldwide-campaign-against-telecommunications-providers)

|  [S0588](https://attack.mitre.org/software/S0588)   |  [GoldMax](https://attack.mitre.org/software/S0588)   |

[GoldMax](https://attack.mitre.org/software/S0588) has been packed for obfuscation.[[46]](https://www.fireeye.com/blog/threat-research/2021/03/sunshuttle-second-stage-backdoor-targeting-us-based-entity.html)

|  [S0342](https://attack.mitre.org/software/S0342)   |  [GreyEnergy](https://attack.mitre.org/software/S0342)   |

[GreyEnergy](https://attack.mitre.org/software/S0342) is packed for obfuscation.[[47]](https://www.welivesecurity.com/wp-content/uploads/2018/10/ESET_GreyEnergy.pdf)

|  [S0132](https://attack.mitre.org/software/S0132)   |  [H1N1](https://attack.mitre.org/software/S0132)   |

[H1N1](https://attack.mitre.org/software/S0132) uses a custom packing algorithm.[[48]](http://blogs.cisco.com/security/h1n1-technical-analysis-reveals-new-capabilities)

|  [S9018](https://attack.mitre.org/software/S9018)   |  [HeartCrypt](https://attack.mitre.org/software/S9018)   |

[HeartCrypt](https://attack.mitre.org/software/S9018) can pack malicious Windows x86 and .NET payloads in order to evade detection.[[49]](https://unit42.paloaltonetworks.com/packer-as-a-service-heartcrypt-malware/)[[50]](https://research.checkpoint.com/2025/blind-eagle-and-justice-for-all/)

|  [S0601](https://attack.mitre.org/software/S0601)   |  [Hildegard](https://attack.mitre.org/software/S0601)   |

[Hildegard](https://attack.mitre.org/software/S0601) has packed ELF files into other binaries.[[51]](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)

|  [S0431](https://attack.mitre.org/software/S0431)   |  [HotCroissant](https://attack.mitre.org/software/S0431)   |

[HotCroissant](https://attack.mitre.org/software/S0431) has used the open source UPX executable packer.[[52]](https://www.carbonblack.com/2020/04/16/vmware-carbon-black-tau-threat-analysis-the-evolution-of-lazarus/)

|  [S0398](https://attack.mitre.org/software/S0398)   |  [HyperBro](https://attack.mitre.org/software/S0398)   |

[HyperBro](https://attack.mitre.org/software/S0398) has the ability to pack its payload.[[53]](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)

|  [S0483](https://attack.mitre.org/software/S0483)   |  [IcedID](https://attack.mitre.org/software/S0483)   |

[IcedID](https://attack.mitre.org/software/S0483) has packed and encrypted its loader module.[[54]](https://blogs.juniper.net/en-us/threat-research/covid-19-and-fmla-campaigns-used-to-install-new-icedid-banking-malware)

|  [S9046](https://attack.mitre.org/software/S9046)   |  [IOControl](https://attack.mitre.org/software/S9046)   |

IOCONTROL has used the Ultimate Packer for eXecutables (UPX) to pack the malware.[[55]](https://claroty.com/team82/research/inside-a-new-ot-iot-cyber-weapon-iocontrol) [[56]](https://5943619.hs-sites.com/hubfs/312-Year-in-Review/2026/Dragos-2026-OT-Cybersecurity-Report-A-Year-in-Review.pdf)

|  [S0283](https://attack.mitre.org/software/S0283)   |  [jRAT](https://attack.mitre.org/software/S0283)   |

[jRAT](https://attack.mitre.org/software/S0283) payloads have been packed.[[57]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07195002/KL_AdwindPublicReport_2016.pdf)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has packed malware with UPX.[[6]](https://blog.malwarebytes.com/threat-analysis/2021/06/kimsuky-apt-continues-to-target-south-korean-government-using-appleseed-backdoor/)

|  [S0356](https://attack.mitre.org/software/S0356)   |  [KONNI](https://attack.mitre.org/software/S0356)   |

[KONNI](https://attack.mitre.org/software/S0356) has been packed for obfuscation.[[58]](https://blog.malwarebytes.com/threat-intelligence/2022/01/konni-evolves-into-stealthier-rat/)

|  [S1160](https://attack.mitre.org/software/S1160)   |  [Latrodectus](https://attack.mitre.org/software/S1160)   |

The [Latrodectus](https://attack.mitre.org/software/S1160) payload has been packed for obfuscation.[[59]](https://www.elastic.co/security-labs/spring-cleaning-with-latrodectus)

|  [S0513](https://attack.mitre.org/software/S0513)   |  [LiteDuke](https://attack.mitre.org/software/S0513)   |

[LiteDuke](https://attack.mitre.org/software/S0513) has been packed with multiple layers of encryption.[[41]](https://www.welivesecurity.com/wp-content/uploads/2019/10/ESET_Operation_Ghost_Dukes.pdf)

|  [S1202](https://attack.mitre.org/software/S1202)   |  [LockBit 3.0](https://attack.mitre.org/software/S1202)   |

[LockBit 3.0](https://attack.mitre.org/software/S1202) can use code packing to hinder analysis.[[60]](https://www.sentinelone.com/labs/lockbit-3-0-update-unpicking-the-ransomwares-latest-anti-analysis-and-evasion-techniques)[[61]](https://www.incibe.es/en/incibe-cert/blog/lockbit-response-and-recovery-actions)

|  [S0447](https://attack.mitre.org/software/S0447)   |  [Lokibot](https://attack.mitre.org/software/S0447)   |

[Lokibot](https://attack.mitre.org/software/S0447) has used several packing methods for obfuscation.[[62]](https://insights.infoblox.com/threat-intelligence-reports/threat-intelligence--22)

|  [S0532](https://attack.mitre.org/software/S0532)   |  [Lucifer](https://attack.mitre.org/software/S0532)   |

[Lucifer](https://attack.mitre.org/software/S0532) has used UPX packed binaries.[[63]](https://unit42.paloaltonetworks.com/lucifer-new-cryptojacking-and-ddos-hybrid-malware/)

|  [S0409](https://attack.mitre.org/software/S0409)   |  [Machete](https://attack.mitre.org/software/S0409)   |

[Machete](https://attack.mitre.org/software/S0409) has been packed with NSIS.[[64]](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)

|  [G1051](https://attack.mitre.org/groups/G1051)   |  [Medusa Group](https://attack.mitre.org/groups/G1051)   |

[Medusa Group](https://attack.mitre.org/groups/G1051) has packed the code of dropped kernel drivers using the packer ASM Guard.[[65]](https://unit42.paloaltonetworks.com/medusa-ransomware-escalation-new-leak-site/)

|  [S0530](https://attack.mitre.org/software/S0530)   |  [Melcoz](https://attack.mitre.org/software/S0530)   |

[Melcoz](https://attack.mitre.org/software/S0530) has been packed with VMProtect and Themida.[[66]](https://securelist.com/the-tetrade-brazilian-banking-malware/97779/)

|  [S0455](https://attack.mitre.org/software/S0455)   |  [Metamorfo](https://attack.mitre.org/software/S0455)   |

[Metamorfo](https://attack.mitre.org/software/S0455) has used VMProtect to pack and protect files.[[67]](https://www.fortinet.com/blog/threat-research/another-metamorfo-variant-targeting-customers-of-financial-institutions)

|  [S0083](https://attack.mitre.org/software/S0083)   |  [Misdat](https://attack.mitre.org/software/S0083)   |

[Misdat](https://attack.mitre.org/software/S0083) was typically packed using UPX.[[68]](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)

|  [S1026](https://attack.mitre.org/software/S1026)   |  [Mongall](https://attack.mitre.org/software/S1026)   |

[Mongall](https://attack.mitre.org/software/S1026) has been packed with Themida.[[5]](https://www.sentinelone.com/labs/aoqin-dragon-newly-discovered-chinese-linked-apt-has-been-quietly-spying-on-organizations-for-10-years/)

|  [G1019](https://attack.mitre.org/groups/G1019)   |  [MoustachedBouncer](https://attack.mitre.org/groups/G1019)   |

[MoustachedBouncer](https://attack.mitre.org/groups/G1019) has used malware plugins packed with Themida.[[69]](https://www.welivesecurity.com/en/eset-research/moustachedbouncer-espionage-against-foreign-diplomats-in-belarus/)

|  [S0198](https://attack.mitre.org/software/S0198)   |  [NETWIRE](https://attack.mitre.org/software/S0198)   |

[NETWIRE](https://attack.mitre.org/software/S0198) has used .NET packer tools to evade detection.[[70]](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)

|  [C0002](https://attack.mitre.org/campaigns/C0002)   |  [Night Dragon](https://attack.mitre.org/campaigns/C0002)   |

During [Night Dragon](https://attack.mitre.org/campaigns/C0002), threat actors used software packing in its tools.[[71]](https://scadahacker.com/library/Documents/Cyber_Events/McAfee%20-%20Night%20Dragon%20-%20Global%20Energy%20Cyberattacks.pdf)

|  [S0264](https://attack.mitre.org/software/S0264)   |  [OopsIE](https://attack.mitre.org/software/S0264)   |

[OopsIE](https://attack.mitre.org/software/S0264) uses the SmartAssembly obfuscator to pack an embedded .Net Framework assembly used for C2.[[72]](https://researchcenter.paloaltonetworks.com/2018/02/unit42-oopsie-oilrig-uses-threedollars-deliver-new-trojan/)

|  [C0022](https://attack.mitre.org/campaigns/C0022)   |  [Operation Dream Job](https://attack.mitre.org/campaigns/C0022)   |

During [Operation Dream Job](https://attack.mitre.org/campaigns/C0022), [Lazarus Group](https://attack.mitre.org/groups/G0032) packed malicious .db files with Themida to evade detection.[[35]](https://www.clearskysec.com/wp-content/uploads/2020/08/Dream-Job-Campaign.pdf)[[73]](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/operation-north-star-a-job-offer-thats-too-good-to-be-true/?hilite=%27Operation%27%2C%27North%27%2C%27Star%27)[[74]](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/operation-north-star-behind-the-scenes/)

|  [C0016](https://attack.mitre.org/campaigns/C0016)   |  [Operation Dust Storm](https://attack.mitre.org/campaigns/C0016)   |

For [Operation Dust Storm](https://attack.mitre.org/campaigns/C0016), the threat actors used UPX to pack some payloads.[[68]](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)

|  [C0005](https://attack.mitre.org/campaigns/C0005)   |  [Operation Spalax](https://attack.mitre.org/campaigns/C0005)   |

For [Operation Spalax](https://attack.mitre.org/campaigns/C0005), the threat actors used a variety of packers, including CyaX, to obfuscate malicious executables.[[75]](https://www.welivesecurity.com/2021/01/12/operation-spalax-targeted-malware-attacks-colombia/)

|  [S0352](https://attack.mitre.org/software/S0352)   |  [OSX_OCEANLOTUS.D](https://attack.mitre.org/software/S0352)   |

[OSX_OCEANLOTUS.D](https://attack.mitre.org/software/S0352) has a variant that is packed with UPX.[[76]](https://www.welivesecurity.com/2019/04/09/oceanlotus-macos-malware-update/)

|  [G0040](https://attack.mitre.org/groups/G0040)   |  [Patchwork](https://attack.mitre.org/groups/G0040)   |

A [Patchwork](https://attack.mitre.org/groups/G0040) payload was packed with UPX.[[77]](https://securelist.com/the-dropping-elephant-actor/75328/)

|  [S0650](https://attack.mitre.org/software/S0650)   |  [QakBot](https://attack.mitre.org/software/S0650)   |

[QakBot](https://attack.mitre.org/software/S0650) can encrypt and pack malicious payloads.[[78]](https://blog.cyberint.com/qakbot-banking-trojan)

|  [S0565](https://attack.mitre.org/software/S0565)   |  [Raindrop](https://attack.mitre.org/software/S0565)   |

[Raindrop](https://attack.mitre.org/software/S0565) used a custom packer for its [Cobalt Strike](https://attack.mitre.org/software/S0154) payload, which was compressed using the LZMA algorithm.[[79]](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/solarwinds-raindrop-malware)[[80]](https://www.microsoft.com/security/blog/2021/01/20/deep-dive-into-the-solorigate-second-stage-activation-from-sunburst-to-teardrop-and-raindrop/)

|  [S1130](https://attack.mitre.org/software/S1130)   |  [Raspberry Robin](https://attack.mitre.org/software/S1130)   |

[Raspberry Robin](https://attack.mitre.org/software/S1130) contains multiple payloads that are packed for defense evasion purposes and unpacked on runtime.[[81]](https://www.trendmicro.com/en_us/research/22/l/raspberry-robin-malware-targets-telecom-governments.html)

|  [S1240](https://attack.mitre.org/software/S1240)   |  [RedLine Stealer](https://attack.mitre.org/software/S1240)   |

[RedLine Stealer](https://attack.mitre.org/software/S1240) has used obfuscation tools such as DNGuard and Boxed App to pack their code.[[82]](https://www.welivesecurity.com/en/eset-research/life-crooked-redline-analyzing-infamous-infostealers-backend/)

|  [G0106](https://attack.mitre.org/groups/G0106)   |  [Rocke](https://attack.mitre.org/groups/G0106)   |

[Rocke](https://attack.mitre.org/groups/G0106)'s miner has created UPX-packed files in the Windows Start Menu Folder.[[83]](https://blog.talosintelligence.com/2018/08/rocke-champion-of-monero-miners.html)[[84]](https://unit42.paloaltonetworks.com/malware-used-by-rocke-group-evolves-to-evade-detection-by-cloud-security-products/)[[85]](https://www.anomali.com/blog/rocke-evolves-its-arsenal-with-a-new-malware-family-written-in-golang)

|  [S0085](https://attack.mitre.org/software/S0085)   |  [S-Type](https://attack.mitre.org/software/S0085)   |

Some [S-Type](https://attack.mitre.org/software/S0085) samples have been packed with UPX.[[68]](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)

|  [S1210](https://attack.mitre.org/software/S1210)   |  [Sagerunex](https://attack.mitre.org/software/S1210)   |

[Sagerunex](https://attack.mitre.org/software/S1210) has used VMProtect to pack and obscure itself.[[86]](https://blog.talosintelligence.com/lotus-blossom-espionage-group/)

|  [G1031](https://attack.mitre.org/groups/G1031)   |  [Saint Bear](https://attack.mitre.org/groups/G1031)   |

[Saint Bear](https://attack.mitre.org/groups/G1031) clones .NET assemblies from other .NET binaries as well as cloning code signing certificates from other software to obfuscate the initial loader payload.[[87]](https://unit42.paloaltonetworks.com/ukraine-targeted-outsteel-saintbot/)

|  [S1018](https://attack.mitre.org/software/S1018)   |  [Saint Bot](https://attack.mitre.org/software/S1018)   |

[Saint Bot](https://attack.mitre.org/software/S1018) has been packed using a dark market crypter.[[88]](https://blog.malwarebytes.com/threat-intelligence/2021/04/a-deep-dive-into-saint-bot-downloader/)

|  [S0461](https://attack.mitre.org/software/S0461)   |  [SDBbot](https://attack.mitre.org/software/S0461)   |

[SDBbot](https://attack.mitre.org/software/S0461) has used a packed installer file.[[89]](https://web.archive.org/web/20200420201624/https://securityintelligence.com/posts/ta505-continues-to-infect-networks-with-sdbbot-rat/)

|  [S0053](https://attack.mitre.org/software/S0053)   |  [SeaDuke](https://attack.mitre.org/software/S0053)   |

[SeaDuke](https://attack.mitre.org/software/S0053) has been packed with the UPX packer.[[90]](http://researchcenter.paloaltonetworks.com/2015/07/unit-42-technical-analysis-seaduke/)

|  [C0058](https://attack.mitre.org/campaigns/C0058)   |  [SharePoint ToolShell Exploitation](https://attack.mitre.org/campaigns/C0058)   |

During [SharePoint ToolShell Exploitation](https://attack.mitre.org/campaigns/C0058), threat actors UPX-packed malicous payloads including 4L4MD4R ransomware.[[91]](https://unit42.paloaltonetworks.com/microsoft-sharepoint-cve-2025-49704-cve-2025-49706-cve-2025-53770/)

|  [S0444](https://attack.mitre.org/software/S0444)   |  [ShimRat](https://attack.mitre.org/software/S0444)   |

[ShimRat](https://attack.mitre.org/software/S0444)'s loader has been packed with the compressed [ShimRat](https://attack.mitre.org/software/S0444) core DLL and the legitimate DLL for it to hijack.[[92]](https://foxitsecurity.files.wordpress.com/2016/06/fox-it_mofang_threatreport_tlp-white.pdf)

|  [S0543](https://attack.mitre.org/software/S0543)   |  [Spark](https://attack.mitre.org/software/S0543)   |

[Spark](https://attack.mitre.org/software/S0543) has been packed with Enigma Protector to obfuscate its contents.[[93]](https://unit42.paloaltonetworks.com/molerats-delivers-spark-backdoor/)

|  [S1030](https://attack.mitre.org/software/S1030)   |  [Squirrelwaffle](https://attack.mitre.org/software/S1030)   |

[Squirrelwaffle](https://attack.mitre.org/software/S1030) has been packed with a custom packer to hide payloads.[[94]](https://www.zscaler.com/blogs/security-research/squirrelwaffle-new-loader-delivering-cobalt-strike)[[95]](https://www.netskope.com/blog/squirrelwaffle-new-malware-loader-delivering-cobalt-strike-and-qakbot)

|  [G1053](https://attack.mitre.org/groups/G1053)   |  [Storm-0501](https://attack.mitre.org/groups/G1053)   |

[Storm-0501](https://attack.mitre.org/groups/G1053) has used Themida to pack [Cobalt Strike](https://attack.mitre.org/software/S0154) payloads.[[96]](https://cloud.google.com/blog/topics/threat-intelligence/sabbath-ransomware-affiliate/)

|  [S1183](https://attack.mitre.org/software/S1183)   |  [StrelaStealer](https://attack.mitre.org/software/S1183)   |

[StrelaStealer](https://attack.mitre.org/software/S1183) variants have used packers to obfuscate payloads and make analysis more difficult.[[97]](https://unit42.paloaltonetworks.com/strelastealer-campaign/)

|  [S0663](https://attack.mitre.org/software/S0663)   |  [SysUpdate](https://attack.mitre.org/software/S0663)   |

[SysUpdate](https://attack.mitre.org/software/S0663) has been packed with VMProtect.[[53]](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)[[98]](https://www.trendmicro.com/en_us/research/23/c/iron-tiger-sysupdate-adds-linux-targeting.html)

|  [G1018](https://attack.mitre.org/groups/G1018)   |  [TA2541](https://attack.mitre.org/groups/G1018)   |

[TA2541](https://attack.mitre.org/groups/G1018) has used a .NET packer to obfuscate malicious files.[[99]](https://blog.talosintelligence.com/operation-layover-how-we-tracked-attack/)

|  [G0092](https://attack.mitre.org/groups/G0092)   |  [TA505](https://attack.mitre.org/groups/G0092)   |

[TA505](https://attack.mitre.org/groups/G0092) has used UPX to obscure malicious code.[[89]](https://web.archive.org/web/20200420201624/https://securityintelligence.com/posts/ta505-continues-to-infect-networks-with-sdbbot-rat/)

|  [G0139](https://attack.mitre.org/groups/G0139)   |  [TeamTNT](https://attack.mitre.org/groups/G0139)   |

[TeamTNT](https://attack.mitre.org/groups/G0139) has used UPX and Ezuri packer to pack its binaries.[[100]](https://documents.trendmicro.com/assets/white_papers/wp-tracking-the-activities-of-teamTNT.pdf)

|  [G1058](https://attack.mitre.org/groups/G1058)   |  [The Gentlemen](https://attack.mitre.org/groups/G1058)   |

[The Gentlemen](https://attack.mitre.org/groups/G1058) has packed [The Gentlemen Ransomware](https://attack.mitre.org/software/S9051) binaries utilizing either Themida or Enigma to obfuscate the malicious payloads.[[101]](https://www.welivesecurity.com/en/eset-research/killing-me-gently-inside-gentlemens-edr-killer-framework/)

|  [G0089](https://attack.mitre.org/groups/G0089)   |  [The White Company](https://attack.mitre.org/groups/G0089)   |

[The White Company](https://attack.mitre.org/groups/G0089) has obfuscated their payloads through packing.[[102]](https://www.cylance.com/content/dam/cylance-web/en-us/resources/knowledge-center/resource-library/reports/WhiteCompanyOperationShaheenReport.pdf?_ga=2.161661948.1943296560.1555683782-1066572390.1555511517)

|  [G0027](https://attack.mitre.org/groups/G0027)   |  [Threat Group-3390](https://attack.mitre.org/groups/G0027)   |

[Threat Group-3390](https://attack.mitre.org/groups/G0027) has packed malware and tools, including using VMProtect.[[103]](https://documents.trendmicro.com/assets/white_papers/wp-uncovering-DRBcontrol.pdf)[[53]](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)

|  [S0671](https://attack.mitre.org/software/S0671)   |  [Tomiris](https://attack.mitre.org/software/S0671)   |

[Tomiris](https://attack.mitre.org/software/S0671) has been packed with UPX.[[104]](https://securelist.com/darkhalo-after-solarwinds-the-tomiris-connection/104311/)

|  [S0678](https://attack.mitre.org/software/S0678)   |  [Torisma](https://attack.mitre.org/software/S0678)   |

[Torisma](https://attack.mitre.org/software/S0678) has been packed with Iz4 compression.[[74]](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/operation-north-star-behind-the-scenes/)

|  [S0266](https://attack.mitre.org/software/S0266)   |  [TrickBot](https://attack.mitre.org/software/S0266)   |

[TrickBot](https://attack.mitre.org/software/S0266) leverages a custom packer to obfuscate its functionality.[[105]](https://www.securityartwork.es/wp-content/uploads/2017/07/Trickbot-report-S2-Grupo.pdf)

|  [S0094](https://attack.mitre.org/software/S0094)   |  [Trojan.Karagany](https://attack.mitre.org/software/S0094)   |

[Trojan.Karagany](https://attack.mitre.org/software/S0094) samples sometimes use common binary packers such as UPX and Aspack on top of a custom Delphi binary packer.[[106]](https://community.broadcom.com/symantecenterprise/communities/community-home/librarydocuments/viewdocument?DocumentKey=7382dce7-0260-4782-84cc-890971ed3f17&CommunityKey=1ecf5f55-9545-44d6-b0f4-4e4a7f5f5e68&tab=librarydocuments)[[107]](https://www.secureworks.com/research/updated-karagany-malware-targets-energy-sector)

|  [S1196](https://attack.mitre.org/software/S1196)   |  [Troll Stealer](https://attack.mitre.org/software/S1196)   |

[Troll Stealer](https://attack.mitre.org/software/S1196) has been delivered as a VMProtect-packed binary.[[108]](https://medium.com/s2wblog/kimsuky-disguised-as-a-korean-company-signed-with-a-valid-certificate-to-distribute-troll-stealer-cfa5d54314e2)[[109]](https://asec.ahnlab.com/en/61934/)

|  [S0022](https://attack.mitre.org/software/S0022)   |  [Uroburos](https://attack.mitre.org/software/S0022)   |

[Uroburos](https://attack.mitre.org/software/S0022) uses a custom packer.[[110]](https://www.threatminer.org/report.php?q=waterbug-attack-group.pdf&y=2015#gsc.tab=0&gsc.q=waterbug-attack-group.pdf&gsc.page=1)[[111]](https://www.cisa.gov/sites/default/files/2023-05/aa23-129a_snake_malware_2.pdf)

|  [S0476](https://attack.mitre.org/software/S0476)   |  [Valak](https://attack.mitre.org/software/S0476)   |

[Valak](https://attack.mitre.org/software/S0476) has used packed DLL payloads.[[112]](https://assets.sentinelone.com/labs/sentinel-one-valak-i)

|  [S0257](https://attack.mitre.org/software/S0257)   |  [VERMIN](https://attack.mitre.org/software/S0257)   |

[VERMIN](https://attack.mitre.org/software/S0257) is initially packed.[[113]](https://researchcenter.paloaltonetworks.com/2018/01/unit42-vermin-quasar-rat-custom-malware-used-ukraine/)

|  [G1017](https://attack.mitre.org/groups/G1017)   |  [Volt Typhoon](https://attack.mitre.org/groups/G1017)   |

[Volt Typhoon](https://attack.mitre.org/groups/G1017) has used the Ultimate Packer for Executables (UPX) to obfuscate the FRP client files BrightmetricAgent.exe and SMSvcService.ex) and the port scanning utility ScanLine.[[114]](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)

|  [S1207](https://attack.mitre.org/software/S1207)   |  [XLoader](https://attack.mitre.org/software/S1207)   |

[XLoader](https://attack.mitre.org/software/S1207) uses various packers, including CyaX, to obfuscate malicious executables.[[115]](https://www.netskope.com/blog/new-formbook-campaign-delivered-through-phishing-emails)

|  [S0248](https://attack.mitre.org/software/S0248)   |  [yty](https://attack.mitre.org/software/S0248)   |

[yty](https://attack.mitre.org/software/S0248) packs a plugin with UPX.[[116]](https://www.arbornetworks.com/blog/asert/donot-team-leverages-new-modular-malware-framework-south-asia/)

|  [S0251](https://attack.mitre.org/software/S0251)   |  [Zebrocy](https://attack.mitre.org/software/S0251)   |

[Zebrocy](https://attack.mitre.org/software/S0251)'s Delphi variant was packed with UPX.[[117]](https://unit42.paloaltonetworks.com/dear-joohn-sofacy-groups-global-campaign/)[[118]](https://www.accenture.com/t20181129T203820Z__w__/us-en/_acnmedia/PDF-90/Accenture-snakemackerel-delivers-zekapab-malware.pdf#zoom=50)

|  [S0230](https://attack.mitre.org/software/S0230)   |  [ZeroT](https://attack.mitre.org/software/S0230)   |

Some [ZeroT](https://attack.mitre.org/software/S0230) DLL files have been packed with UPX.[[119]](https://www.proofpoint.com/us/threat-insight/post/APT-targets-russia-belarus-zerot-plugx)

|  [G0128](https://attack.mitre.org/groups/G0128)   |  [ZIRCONIUM](https://attack.mitre.org/groups/G0128)   |

[ZIRCONIUM](https://attack.mitre.org/groups/G0128) has used multi-stage packers for exploit code.[[120]](https://research.checkpoint.com/2021/the-story-of-jian/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1049](https://attack.mitre.org/mitigations/M1049)   |  [Antivirus/Antimalware](https://attack.mitre.org/mitigations/M1049)   |

Employ heuristic-based malware detection. Ensure updated virus definitions and create custom signatures for observed malware.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0023](https://attack.mitre.org/detectionstrategies/DET0023)   |  [Obfuscated Binary Unpacking Detection via Behavioral Patterns](https://attack.mitre.org/detectionstrategies/DET0023)   |  [AN0066](https://attack.mitre.org/detectionstrategies/DET0023#AN0066)   |

Detection of unpacking behavior through abnormal memory allocation, followed by executable code injection and execution from non-image sections.

|  [AN0067](https://attack.mitre.org/detectionstrategies/DET0023#AN0067)   |

Correlates ELF file execution with high-entropy writable memory segments and self-modifying code patterns.

|  [AN0068](https://attack.mitre.org/detectionstrategies/DET0023#AN0068)   |

Detection of packed Mach-O binaries unpacking into memory and transferring control to dynamically modified code segments.

## References

-    [Kafka, F. (2018, January). ESET's Guide to Deobfuscating and Devirtualizing FinFisher. Retrieved August 12, 2019.](https://www.welivesecurity.com/wp-content/uploads/2018/01/WP-FinFisher.pdf)
-    [Alexandre D'Hondt. (n.d.). Awesome Executable Packing. Retrieved March 11, 2022.](https://github.com/dhondta/awesome-executable-packing)
-    [Joe Slowik. (2018, October 12). Anatomy of an Attack: Detecting and Defeating CRASHOVERRIDE. Retrieved December 18, 2020.](https://www.dragos.com/wp-content/uploads/CRASHOVERRIDE2018.pdf)
-    [Dahan, A. et al. (2019, December 11). DROPPING ANCHOR: FROM A TRICKBOT INFECTION TO THE DISCOVERY OF THE ANCHOR MALWARE. Retrieved September 10, 2020.](https://www.cybereason.com/blog/dropping-anchor-from-a-trickbot-infection-to-the-discovery-of-the-anchor-malware)
-    [Chen, Joey. (2022, June 9). Aoqin Dragon | Newly-Discovered Chinese-linked APT Has Been Quietly Spying On Organizations For 10 Years. Retrieved July 14, 2022.](https://www.sentinelone.com/labs/aoqin-dragon-newly-discovered-chinese-linked-apt-has-been-quietly-spying-on-organizations-for-10-years/)
-    [Jazi, H. (2021, June 1). Kimsuky APT continues to target South Korean government using AppleSeed backdoor. Retrieved June 10, 2021.](https://blog.malwarebytes.com/threat-analysis/2021/06/kimsuky-apt-continues-to-target-south-korean-government-using-appleseed-backdoor/)
-    [Dunwoody, M. and Carr, N.. (2016, September 27). No Easy Breach DerbyCon 2016. Retrieved September 12, 2024.](https://www.slideshare.net/slideshow/no-easy-breach-derby-con-2016/66447908)
-    [Korban, C, et al. (2017, September). APT3 Adversary Emulation Plan. Retrieved January 16, 2018.](https://attack.mitre.org/docs/APT3_Adversary_Emulation_Plan.pdf)
-    [Eng, E., Caselden, D.. (2015, June 23). Operation Clandestine Wolf – Adobe Flash Zero-Day in APT3 Phishing Campaign. Retrieved January 14, 2016.](https://www.fireeye.com/blog/threat-research/2015/06/operation-clandestine-wolf-adobe-flash-zero-day.html)
-    [FireEye. (2018, October 03). APT38: Un-usual Suspects. Retrieved November 17, 2024.](https://services.google.com/fh/files/misc/apt38-un-usual-suspects.pdf)
-    [Hawley et al. (2019, January 29). APT39: An Iranian Cyber Espionage Group Focused on Personal Information. Retrieved February 19, 2019.](https://www.fireeye.com/blog/threat-research/2019/01/apt39-iranian-cyber-espionage-group-focused-on-personal-information.html)
-    [Rusu, B. (2020, May 21). Iranian Chafer APT Targeted Air Transportation and Government in Kuwait and Saudi Arabia. Retrieved May 22, 2020.](https://www.bitdefender.com/blog/labs/iranian-chafer-apt-targeted-air-transportation-and-government-in-kuwait-and-saudi-arabia/)
-    [Nikita Rostovcev. (2022, August 18). APT41 World Tour 2021 on a tight schedule. Retrieved February 22, 2024.](https://www.group-ib.com/blog/apt41-world-tour-2021/)
-    [Salem, E. (2019, February 13). ASTAROTH MALWARE USES LEGITIMATE OS AND ANTIVIRUS PROCESSES TO STEAL PASSWORDS AND PERSONAL DATA. Retrieved April 17, 2019.](https://www.cybereason.com/blog/information-stealing-malware-targeting-brazil-full-research)
-    [Sogeti. (2021, March). Babuk Ransomware. Retrieved August 11, 2021.](https://www.sogeti.com/globalassets/reports/cybersecchronicles_-_babuk.pdf)
-    [Mundo, A. et al. (2021, February). Technical Analysis of Babuk Ransomware. Retrieved August 11, 2021.](https://www.mcafee.com/enterprise/en-us/assets/reports/rp-babuk-ransomware.pdf)
-    [Sebdraven. (2021, February 8). Babuk is distributed packed. Retrieved August 11, 2021.](https://sebdraven.medium.com/babuk-is-distributed-packed-78e2f5dd2e62)
-    [Cybereason Nocturnus. (2020, July 16). A BAZAR OF TRICKS: FOLLOWING TEAM9’S DEVELOPMENT CYCLES. Retrieved November 18, 2020.](https://www.cybereason.com/blog/a-bazar-of-tricks-following-team9s-development-cycles)
-    [Sadique, M. and Singh, A. (2020, September 29). Spear Phishing Campaign Delivers Buer and Bazar Malware. Retrieved November 19, 2020.](https://www.zscaler.com/blogs/research/spear-phishing-campaign-delivers-buer-and-bazar-malware)
-    [Mercer, W., et al. (2020, March 5). Bisonal: 10 years of play. Retrieved January 26, 2022.](https://blog.talosintelligence.com/2020/03/bisonal-10-years-of-play.html)
-    [US-CERT. (2020, August 19). MAR-10295134-1.v1 – North Korean Remote Access Trojan: BLINDINGCAN. Retrieved August 19, 2020.](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-232a)
-    [Rufus Brown, Van Ta, Douglas Bienstock, Geoff Ackerman, John Wolfram. (2022, March 8). Does This Look Infected? A Summary of APT41 Targeting U.S. State Governments. Retrieved July 8, 2022.](https://www.mandiant.com/resources/apt41-us-state-governments)
-    [Lee, T., Hanzlik, D., Ahl, I. (2013, August 7). Breaking Down the China Chopper Web Shell - Part I. Retrieved March 27, 2015.](https://www.fireeye.com/blog/threat-research/2013/08/breaking-down-the-china-chopper-web-shell-part-i.html)
-    [Mundo, A. (2019, August 1). Clop Ransomware. Retrieved May 10, 2021.](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/clop-ransomware/)
-    [Cybereason Nocturnus. (2020, December 23). Cybereason vs. Clop Ransomware. Retrieved May 11, 2021.](https://www.cybereason.com/blog/cybereason-vs.-clop-ransomware)
-    [Dutch Military Intelligence and Security Service (MIVD) & Dutch General Intelligence and Security Service (AIVD). (2024, February 6). Ministry of Defense of the Netherlands uncovers COATHANGER, a stealthy Chinese FortiGate RAT. Retrieved February 7, 2024.](https://www.ncsc.nl/binaries/ncsc/documenten/publicaties/2024/februari/6/mivd-aivd-advisory-coathanger-tlp-clear/TLP-CLEAR+MIVD+AIVD+Advisory+COATHANGER.pdf)
-    [The BlackBerry Research and Intelligence Team. (2020, November 12). The CostaRicto Campaign: Cyber-Espionage Outsourced. Retrieved May 24, 2021.](https://blogs.blackberry.com/en/2020/11/the-costaricto-campaign-cyber-espionage-outsourced)
-    [Dahan, A. et al. (2020, November 2). Back to the Future: Inside the Kimsuky KGH Spyware Suite. Retrieved November 6, 2020.](https://www.cybereason.com/blog/back-to-the-future-inside-the-kimsuky-kgh-spyware-suite)
-    [Roccio, T., et al. (2021, April). Technical Analysis of Cuba Ransomware. Retrieved June 18, 2021.](https://www.mcafee.com/enterprise/en-us/assets/reports/rp-cuba-ransomware.pdf)
-    [Blaich, A., et al. (2018, January 18). Dark Caracal: Cyber-espionage at a Global Scale. Retrieved April 11, 2018.](https://info.lookout.com/rs/051-ESQ-475/images/Lookout_Dark-Caracal_srr_20180118_us_v.1.0.pdf)
-    [Kujawa, A. (2018, March 27). You dirty RAT! Part 1: DarkComet. Retrieved November 6, 2018.](https://blog.malwarebytes.com/threat-analysis/2012/06/you-dirty-rat-part-1-darkcomet/)
-    [Chen, J. and Hsieh, M. (2017, November 7). REDBALDKNIGHT/BRONZE BUTLER’s Daserf Backdoor Now Using Steganography. Retrieved December 27, 2017.](http://blog.trendmicro.com/trendlabs-security-intelligence/redbaldknight-bronze-butler-daserf-backdoor-now-using-steganography/)
-    [fluffybunny. (2019, July 9). OSX.Dok Analysis. Retrieved November 17, 2024.](https://web.archive.org/web/20221007144948/http://www.hexed.in/2019/07/osxdok-analysis.html)
-    [TheWover. (2019, May 9). donut. Retrieved March 25, 2022.](https://github.com/TheWover/donut)
-    [ClearSky Research Team. (2020, August 13). Operation 'Dream Job' Widespread North Korean Espionage Campaign. Retrieved December 20, 2021.](https://www.clearskysec.com/wp-content/uploads/2020/08/Dream-Job-Campaign.pdf)
-    [hasherezade. (2015, November 4). A Technical Look At Dyreza. Retrieved June 15, 2020.](https://blog.malwarebytes.com/threat-analysis/2015/11/a-technical-look-at-dyreza/)
-    [NHS Digital. (2020, November 26). Egregor Ransomware The RaaS successor to Maze. Retrieved December 29, 2020.](https://digital.nhs.uk/cyber-alerts/2020/cc-3681#summary)
-    [Cybleinc. (2020, October 31). Egregor Ransomware – A Deep Dive Into Its Activities and Techniques. Retrieved December 29, 2020.](https://cybleinc.com/2020/10/31/egregor-ransomware-a-deep-dive-into-its-activities-and-techniques/)
-    [O'Gorman, G., and McDonald, G.. (2012, September 6). The Elderwood Project. Retrieved November 17, 2024.](https://web.archive.org/web/20190717233006/http:/www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/the-elderwood-project.pdf)
-    [Trend Micro. (2019, January 16). Exploring Emotet's Activities . Retrieved March 25, 2019.](https://documents.trendmicro.com/assets/white_papers/ExploringEmotetsActivities_Final.pdf)
-    [Faou, M., Tartare, M., Dupuy, T. (2019, October). OPERATION GHOST. Retrieved September 23, 2020.](https://www.welivesecurity.com/wp-content/uploads/2019/10/ESET_Operation_Ghost_Dukes.pdf)
-    [FinFisher. (n.d.). Retrieved September 12, 2024.](https://web.archive.org/web/20171222050934/http://www.finfisher.com/FinFisher/index.html)
-    [Kaspersky Lab's Global Research & Analysis Team. (2017, October 16). BlackOasis APT and new targeted attacks leveraging zero-day exploit. Retrieved February 15, 2018.](https://securelist.com/blackoasis-apt-and-new-targeted-attacks-leveraging-zero-day-exploit/82732/)
-    [GREAT. (2021, March 30). APT10: sophisticated multi-layered loader Ecipekac discovered in A41APT campaign. Retrieved June 17, 2021.](https://securelist.com/apt10-sophisticated-multi-layered-loader-ecipekac-discovered-in-a41apt-campaign/101519/)
-    [Cybereason Nocturnus. (2019, June 25). Operation Soft Cell: A Worldwide Campaign Against Telecommunications Providers. Retrieved July 18, 2019.](https://www.cybereason.com/blog/operation-soft-cell-a-worldwide-campaign-against-telecommunications-providers)
-    [Smith, L., Leathery, J., Read, B. (2021, March 4). New SUNSHUTTLE Second-Stage Backdoor Uncovered Targeting U.S.-Based Entity; Possible Connection to UNC2452. Retrieved March 12, 2021.](https://www.fireeye.com/blog/threat-research/2021/03/sunshuttle-second-stage-backdoor-targeting-us-based-entity.html)
-    [Cherepanov, A. (2018, October). GREYENERGY A successor to BlackEnergy. Retrieved November 15, 2018.](https://www.welivesecurity.com/wp-content/uploads/2018/10/ESET_GreyEnergy.pdf)
-    [Reynolds, J.. (2016, September 13). H1N1: Technical analysis reveals new capabilities. Retrieved September 26, 2016.](http://blogs.cisco.com/security/h1n1-technical-analysis-reveals-new-capabilities)
-    [Tujague, J., Bunce, D. (n.d.). Crypted Hearts: Exposing the HeartCrypt Packer-as-a-Service Operation. Retrieved April 16, 2026.](https://unit42.paloaltonetworks.com/packer-as-a-service-heartcrypt-malware/)
-    [Check Point Research. (2025, March 10). Blind Eagle: …And Justice for All. Retrieved April 16, 2026.](https://research.checkpoint.com/2025/blind-eagle-and-justice-for-all/)
-    [Chen, J. et al. (2021, February 3). Hildegard: New TeamTNT Cryptojacking Malware Targeting Kubernetes. Retrieved April 5, 2021.](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)
-    [Knight, S.. (2020, April 16). VMware Carbon Black TAU Threat Analysis: The Evolution of Lazarus. Retrieved May 1, 2020.](https://www.carbonblack.com/2020/04/16/vmware-carbon-black-tau-threat-analysis-the-evolution-of-lazarus/)
-    [Lunghi, D. and Lu, K. (2021, April 9). Iron Tiger APT Updates Toolkit With Evolved SysUpdate Malware. Retrieved November 12, 2021.](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)
-    [Kimayong, P. (2020, June 18). COVID-19 and FMLA Campaigns used to install new IcedID banking malware. Retrieved July 14, 2020.](https://blogs.juniper.net/en-us/threat-research/covid-19-and-fmla-campaigns-used-to-install-new-icedid-banking-malware)
-    [Claroty Team82. (2024, December 10). Inside a New OT/IoT Cyberweapon: IOCONTROL. Retrieved September 2, 2026.](https://claroty.com/team82/research/inside-a-new-ot-iot-cyber-weapon-iocontrol)
-    [Dragos. (2026, February 17). Dragos 2026 OT/ICS Cybersecurity Report and Year in Review. Retrieved September 2, 2026.](https://5943619.hs-sites.com/hubfs/312-Year-in-Review/2026/Dragos-2026-OT-Cybersecurity-Report-A-Year-in-Review.pdf)
-    [Kamluk, V. & Gostev, A. (2016, February). Adwind - A Cross-Platform RAT. Retrieved April 23, 2019.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07195002/KL_AdwindPublicReport_2016.pdf)
-    [Santos, R. (2022, January 26). KONNI evolves into stealthier RAT. Retrieved April 13, 2022.](https://blog.malwarebytes.com/threat-intelligence/2022/01/konni-evolves-into-stealthier-rat/)
-    [Stepanic, D. and Bousseaden, S. (2024, May 15). Spring Cleaning with LATRODECTUS: A Potential Replacement for ICEDID. Retrieved September 13, 2024.](https://www.elastic.co/security-labs/spring-cleaning-with-latrodectus)
-    [Walter, J. (2022, July 21). LockBit 3.0 Update | Unpicking the Ransomware’s Latest Anti-Analysis and Evasion Techniques. Retrieved February 5, 2025.](https://www.sentinelone.com/labs/lockbit-3-0-update-unpicking-the-ransomwares-latest-anti-analysis-and-evasion-techniques)

-    [INCIBE-CERT. (2024, March 14). LockBit: response and recovery actions. Retrieved February 5, 2025.](https://www.incibe.es/en/incibe-cert/blog/lockbit-response-and-recovery-actions)
-    [Hoang, M. (2019, January 31). Malicious Activity Report: Elements of Lokibot Infostealer. Retrieved May 15, 2020.](https://insights.infoblox.com/threat-intelligence-reports/threat-intelligence--22)
-    [Hsu, K. et al. (2020, June 24). Lucifer: New Cryptojacking and DDoS Hybrid Malware Exploiting High and Critical Vulnerabilities to Infect Windows Devices. Retrieved November 16, 2020.](https://unit42.paloaltonetworks.com/lucifer-new-cryptojacking-and-ddos-hybrid-malware/)
-    [ESET. (2019, July). MACHETE JUST GOT SHARPER Venezuelan government institutions under attack. Retrieved September 13, 2019.](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)
-    [Anthony Galiette, Doel Santos. (2024, January 11). Medusa Ransomware Turning Your Files into Stone. Retrieved October 15, 2025.](https://unit42.paloaltonetworks.com/medusa-ransomware-escalation-new-leak-site/)
-    [GReAT. (2020, July 14). The Tetrade: Brazilian banking malware goes global. Retrieved November 9, 2020.](https://securelist.com/the-tetrade-brazilian-banking-malware/97779/)
-    [Zhang, X. (2020, February 4). Another Metamorfo Variant Targeting Customers of Financial Institutions in More Countries. Retrieved July 30, 2020.](https://www.fortinet.com/blog/threat-research/another-metamorfo-variant-targeting-customers-of-financial-institutions)
-    [Gross, J. (2016, February 23). Operation Dust Storm. Retrieved December 22, 2021.](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)
-    [Faou, M. (2023, August 10). MoustachedBouncer: Espionage against foreign diplomats in Belarus. Retrieved September 25, 2023.](https://www.welivesecurity.com/en/eset-research/moustachedbouncer-espionage-against-foreign-diplomats-in-belarus/)
-    [Lambert, T. (2020, January 29). Intro to Netwire. Retrieved January 7, 2021.](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)
-    [McAfee® Foundstone® Professional Services and McAfee Labs™. (2011, February 10). Global Energy Cyberattacks: “Night Dragon”. Retrieved February 19, 2018.](https://scadahacker.com/library/Documents/Cyber_Events/McAfee%20-%20Night%20Dragon%20-%20Global%20Energy%20Cyberattacks.pdf)
-    [Lee, B., Falcone, R. (2018, February 23). OopsIE! OilRig Uses ThreeDollars to Deliver New Trojan. Retrieved July 16, 2018.](https://researchcenter.paloaltonetworks.com/2018/02/unit42-oopsie-oilrig-uses-threedollars-deliver-new-trojan/)
-    [Cashman, M. (2020, July 29). Operation North Star Campaign. Retrieved December 20, 2021.](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/operation-north-star-a-job-offer-thats-too-good-to-be-true/?hilite=%27Operation%27%2C%27North%27%2C%27Star%27)
-    [Beek, C. (2020, November 5). Operation North Star: Behind The Scenes. Retrieved December 20, 2021.](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/operation-north-star-behind-the-scenes/)
-    [M. Porolli. (2021, January 21). Operation Spalax: Targeted malware attacks in Colombia. Retrieved September 16, 2022.](https://www.welivesecurity.com/2021/01/12/operation-spalax-targeted-malware-attacks-colombia/)
-    [Dumont, R.. (2019, April 9). OceanLotus: macOS malware update. Retrieved April 15, 2019.](https://www.welivesecurity.com/2019/04/09/oceanlotus-macos-malware-update/)
-    [Kaspersky Lab's Global Research & Analysis Team. (2016, July 8). The Dropping Elephant – aggressive cyber-espionage in the Asian region. Retrieved August 3, 2016.](https://securelist.com/the-dropping-elephant-actor/75328/)
-    [Cyberint. (2021, May 25). Qakbot Banking Trojan. Retrieved September 27, 2021.](https://blog.cyberint.com/qakbot-banking-trojan)
-    [Symantec Threat Hunter Team. (2021, January 18). Raindrop: New Malware Discovered in SolarWinds Investigation. Retrieved January 19, 2021.](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/solarwinds-raindrop-malware)
-    [MSTIC, CDOC, 365 Defender Research Team. (2021, January 20). Deep dive into the Solorigate second-stage activation: From SUNBURST to TEARDROP and Raindrop . Retrieved January 22, 2021.](https://www.microsoft.com/security/blog/2021/01/20/deep-dive-into-the-solorigate-second-stage-activation-from-sunburst-to-teardrop-and-raindrop/)
-    [Christopher So. (2022, December 20). Raspberry Robin Malware Targets Telecom, Governments. Retrieved May 17, 2024.](https://www.trendmicro.com/en_us/research/22/l/raspberry-robin-malware-targets-telecom-governments.html)
-    [Alexandre Cote Cyr. (2024, November 8). Life on a crooked RedLine: Analyzing the infamous infostealer’s backend. Retrieved September 17, 2025.](https://www.welivesecurity.com/en/eset-research/life-crooked-redline-analyzing-infamous-infostealers-backend/)
-    [Liebenberg, D.. (2018, August 30). Rocke: The Champion of Monero Miners. Retrieved May 26, 2020.](https://blog.talosintelligence.com/2018/08/rocke-champion-of-monero-miners.html)
-    [Xingyu, J.. (2019, January 17). Malware Used by Rocke Group Evolves to Evade Detection by Cloud Security Products. Retrieved May 26, 2020.](https://unit42.paloaltonetworks.com/malware-used-by-rocke-group-evolves-to-evade-detection-by-cloud-security-products/)
-    [Anomali Labs. (2019, March 15). Rocke Evolves Its Arsenal With a New Malware Family Written in Golang. Retrieved April 24, 2019.](https://www.anomali.com/blog/rocke-evolves-its-arsenal-with-a-new-malware-family-written-in-golang)
-    [Joey Chen, Cisco Talos. (2025, February 27). Lotus Blossom espionage group targets multiple industries with different versions of Sagerunex and hacking tools. Retrieved March 15, 2025.](https://blog.talosintelligence.com/lotus-blossom-espionage-group/)
-    [Unit 42. (2022, February 25). Spear Phishing Attacks Target Organizations in Ukraine, Payloads Include the Document Stealer OutSteel and the Downloader SaintBot. Retrieved June 9, 2022.](https://unit42.paloaltonetworks.com/ukraine-targeted-outsteel-saintbot/)
-    [Hasherezade. (2021, April 6). A deep dive into Saint Bot, a new downloader. Retrieved June 9, 2022.](https://blog.malwarebytes.com/threat-intelligence/2021/04/a-deep-dive-into-saint-bot-downloader/)
-    [Frydrych, M. (2020, April 14). TA505 Continues to Infect Networks With SDBbot RAT. Retrieved May 29, 2020.](https://web.archive.org/web/20200420201624/https://securityintelligence.com/posts/ta505-continues-to-infect-networks-with-sdbbot-rat/)
-    [Grunzweig, J.. (2015, July 14). Unit 42 Technical Analysis: Seaduke. Retrieved August 3, 2016.](http://researchcenter.paloaltonetworks.com/2015/07/unit-42-technical-analysis-seaduke/)
-    [Unit 42. (2025, July 31). Active Exploitation of Microsoft SharePoint Vulnerabilities: Threat Brief (Updated). Retrieved October 15, 2025.](https://unit42.paloaltonetworks.com/microsoft-sharepoint-cve-2025-49704-cve-2025-49706-cve-2025-53770/)
-    [Yonathan Klijnsma. (2016, May 17). Mofang: A politically motivated information stealing adversary. Retrieved May 12, 2020.](https://foxitsecurity.files.wordpress.com/2016/06/fox-it_mofang_threatreport_tlp-white.pdf)
-    [Falcone, R., et al. (2020, March 3). Molerats Delivers Spark Backdoor to Government and Telecommunications Organizations. Retrieved December 14, 2020.](https://unit42.paloaltonetworks.com/molerats-delivers-spark-backdoor/)
-    [Kumar, A., Stone-Gross, Brett. (2021, September 28). Squirrelwaffle: New Loader Delivering Cobalt Strike. Retrieved August 9, 2022.](https://www.zscaler.com/blogs/security-research/squirrelwaffle-new-loader-delivering-cobalt-strike)
-    [Palazolo, G. (2021, October 7). SquirrelWaffle: New Malware Loader Delivering Cobalt Strike and QakBot. Retrieved August 9, 2022.](https://www.netskope.com/blog/squirrelwaffle-new-malware-loader-delivering-cobalt-strike-and-qakbot)
-    [Tyler McLellan, Brandan Schondorfer. (2021, November 29). Kitten.gif: Meet the Sabbath Ransomware Affiliate Program, Again. Retrieved October 19, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/sabbath-ransomware-affiliate/)
-    [Benjamin Chang, Goutam Tripathy, Pranay Kumar Chhaparwal, Anmol Maurya & Vishwa Thothathri, Palo Alto Networks. (2024, March 22). Large-Scale StrelaStealer Campaign in Early 2024. Retrieved December 31, 2024.](https://unit42.paloaltonetworks.com/strelastealer-campaign/)
-    [Daniel Lunghi. (2023, March 1). Iron Tiger’s SysUpdate Reappears, Adds Linux Targeting. Retrieved March 20, 2023.](https://www.trendmicro.com/en_us/research/23/c/iron-tiger-sysupdate-adds-linux-targeting.html)
-    [Ventura, V. (2021, September 16). Operation Layover: How we tracked an attack on the aviation industry to five years of compromise. Retrieved September 15, 2023.](https://blog.talosintelligence.com/operation-layover-how-we-tracked-attack/)
-    [Fiser, D. Oliveira, A. (n.d.). Tracking the Activities of TeamTNT A Closer Look at a Cloud-Focused Malicious Actor Group. Retrieved September 22, 2021.](https://documents.trendmicro.com/assets/white_papers/wp-tracking-the-activities-of-teamTNT.pdf)
-    [Souček, J. (2026, June 18). Killing me gently: Inside Gentlemen’s EDR killer framework. Retrieved September 18, 2026.](https://www.welivesecurity.com/en/eset-research/killing-me-gently-inside-gentlemens-edr-killer-framework/)
-    [Livelli, K, et al. (2018, November 12). Operation Shaheen. Retrieved May 1, 2019.](https://www.cylance.com/content/dam/cylance-web/en-us/resources/knowledge-center/resource-library/reports/WhiteCompanyOperationShaheenReport.pdf?_ga=2.161661948.1943296560.1555683782-1066572390.1555511517)
-    [Lunghi, D. et al. (2020, February). Uncovering DRBControl. Retrieved November 12, 2021.](https://documents.trendmicro.com/assets/white_papers/wp-uncovering-DRBcontrol.pdf)
-    [Kwiatkoswki, I. and Delcher, P. (2021, September 29). DarkHalo After SolarWinds: the Tomiris connection. Retrieved December 27, 2021.](https://securelist.com/darkhalo-after-solarwinds-the-tomiris-connection/104311/)
-    [Salinas, M., Holguin, J. (2017, June). Evolution of Trickbot. Retrieved July 31, 2018.](https://www.securityartwork.es/wp-content/uploads/2017/07/Trickbot-report-S2-Grupo.pdf)
-    [Symantec Security Response. (2014, June 30). Dragonfly: Cyberespionage Attacks Against Energy Suppliers. Retrieved April 8, 2016.](https://community.broadcom.com/symantecenterprise/communities/community-home/librarydocuments/viewdocument?DocumentKey=7382dce7-0260-4782-84cc-890971ed3f17&CommunityKey=1ecf5f55-9545-44d6-b0f4-4e4a7f5f5e68&tab=librarydocuments)
-    [Secureworks. (2019, July 24). Updated Karagany Malware Targets Energy Sector. Retrieved August 12, 2020.](https://www.secureworks.com/research/updated-karagany-malware-targets-energy-sector)
-    [Jiho Kim & Sebin Lee, S2W. (2024, February 7). Kimsuky disguised as a Korean company signed with a valid certificate to distribute Troll Stealer (English ver.). Retrieved January 17, 2025.](https://medium.com/s2wblog/kimsuky-disguised-as-a-korean-company-signed-with-a-valid-certificate-to-distribute-troll-stealer-cfa5d54314e2)
-    [AhnLab ASEC. (2024, February 16). TrollAgent That Infects Systems Upon Security Program Installation Process (Kimsuky Group). Retrieved January 17, 2025.](https://asec.ahnlab.com/en/61934/)
-    [Symantec. (2015, January 26). The Waterbug attack group. Retrieved April 10, 2015.](https://www.threatminer.org/report.php?q=waterbug-attack-group.pdf&y=2015#gsc.tab=0&gsc.q=waterbug-attack-group.pdf&gsc.page=1)
-    [FBI et al. (2023, May 9). Hunting Russian Intelligence “Snake” Malware. Retrieved June 8, 2023.](https://www.cisa.gov/sites/default/files/2023-05/aa23-129a_snake_malware_2.pdf)
-    [Reaves, J. and Platt, J. (2020, June). Valak Malware and the Connection to Gozi Loader ConfCrew. Retrieved August 31, 2020.](https://assets.sentinelone.com/labs/sentinel-one-valak-i)
-    [Lancaster, T., Cortes, J. (2018, January 29). VERMIN: Quasar RAT and Custom Malware Used In Ukraine. Retrieved July 5, 2018.](https://researchcenter.paloaltonetworks.com/2018/01/unit42-vermin-quasar-rat-custom-malware-used-ukraine/)
-    [CISA et al.. (2024, February 7). PRC State-Sponsored Actors Compromise and Maintain Persistent Access to U.S. Critical Infrastructure. Retrieved May 15, 2024.](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)
-    [Gustavo Palazolo, Netskope. (2022, March 11). New Formbook Campaign Delivered Through Phishing Emails. Retrieved March 11, 2025.](https://www.netskope.com/blog/new-formbook-campaign-delivered-through-phishing-emails)
-    [Schwarz, D., Sopko J. (2018, March 08). Donot Team Leverages New Modular Malware Framework in South Asia. Retrieved June 11, 2018.](https://www.arbornetworks.com/blog/asert/donot-team-leverages-new-modular-malware-framework-south-asia/)
-    [Lee, B., Falcone, R. (2018, December 12). Dear Joohn: The Sofacy Group’s Global Campaign. Retrieved April 19, 2019.](https://unit42.paloaltonetworks.com/dear-joohn-sofacy-groups-global-campaign/)
-    [Accenture Security. (2018, November 29). SNAKEMACKEREL. Retrieved April 15, 2019.](https://www.accenture.com/t20181129T203820Z__w__/us-en/_acnmedia/PDF-90/Accenture-snakemackerel-delivers-zekapab-malware.pdf#zoom=50)
-    [Huss, D., et al. (2017, February 2). Oops, they did it again: APT Targets Russia and Belarus with ZeroT and PlugX. Retrieved April 5, 2018.](https://www.proofpoint.com/us/threat-insight/post/APT-targets-russia-belarus-zerot-plugx)
-    [Itkin, E. and Cohen, I. (2021, February 22). The Story of Jian – How APT31 Stole and Used an Unknown Equation Group 0-Day. Retrieved March 24, 2021.](https://research.checkpoint.com/2021/the-story-of-jian/)

×
