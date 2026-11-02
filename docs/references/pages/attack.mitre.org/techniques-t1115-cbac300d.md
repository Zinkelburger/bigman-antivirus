---
url: "https://attack.mitre.org/techniques/T1115/"
title: "Clipboard Data, Technique T1115 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:35:55Z"
via: "live"
sha256: "fd4fe97f139cf1e56bad8b89d194e8cd03dcd871fe118119d7bdbe60b6b31194"
wayback: "https://web.archive.org/web/20261102191610/https://attack.mitre.org/techniques/T1115/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Clipboard Data, Technique T1115 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1115/>
> Snapshot: <https://web.archive.org/web/20261102191610/https://attack.mitre.org/techniques/T1115/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Clipboard Data

#  Clipboard Data

Adversaries may collect data stored in the clipboard from users copying information within or between applications.

For example, on Windows adversaries can access clipboard data by using clip.exe or Get-Clipboard.[[1]](https://msdn.microsoft.com/en-us/library/ms649012)[[2]](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/clip)[[3]](https://www.cisa.gov/uscert/ncas/alerts/aa21-200b) Additionally, adversaries may monitor then replace users’ clipboard with their data (e.g., [Transmitted Data Manipulation](https://attack.mitre.org/techniques/T1565/002)).[[4]](https://blog.reversinglabs.com/blog/mining-for-malicious-ruby-gems)

macOS and Linux also have commands, such as pbpaste, to grab clipboard contents.[[5]](https://medium.com/rvrsh3ll/operating-with-empyre-ea764eda3363)

ID: T1115

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Collection](https://attack.mitre.org/tactics/TA0009)

ⓘ

Platforms: Linux, Windows, macOS

Version: 1.2

Created: 31 May 2017

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1115/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1115/)

## Procedure Examples

| ID  | Name  | Description

|  [S0331](https://attack.mitre.org/software/S0331)   |  [Agent Tesla](https://attack.mitre.org/software/S0331)   |

[Agent Tesla](https://attack.mitre.org/software/S0331) can steal data from the victim’s clipboard.[[6]](https://blog.talosintelligence.com/2018/10/old-dog-new-tricks-analysing-new-rtf_15.html)[[7]](https://www.fortinet.com/blog/threat-research/analysis-of-new-agent-tesla-spyware-variant.html)[[8]](https://www.fortinet.com/blog/threat-research/in-depth-analysis-of-net-malware-javaupdtr.html)[[9]](https://labs.bitdefender.com/2020/04/oil-gas-spearphishing-campaigns-drop-agent-tesla-spyware-in-advance-of-historic-opec-deal/)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) used a Trojan called KEYLIME to collect data from the clipboard.[[10]](https://services.google.com/fh/files/misc/apt38-un-usual-suspects.pdf)

|  [G0087](https://attack.mitre.org/groups/G0087)   |  [APT39](https://attack.mitre.org/groups/G0087)   |

[APT39](https://attack.mitre.org/groups/G0087) has used tools capable of stealing contents of the clipboard.[[11]](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/chafer-latest-attacks-reveal-heightened-ambitions)

|  [S0373](https://attack.mitre.org/software/S0373)   |  [Astaroth](https://attack.mitre.org/software/S0373)   |

[Astaroth](https://attack.mitre.org/software/S0373) collects information from the clipboard by using the OpenClipboard() and GetClipboardData() libraries. [[12]](https://www.cybereason.com/blog/information-stealing-malware-targeting-brazil-full-research)

|  [S0438](https://attack.mitre.org/software/S0438)   |  [Attor](https://attack.mitre.org/software/S0438)   |

[Attor](https://attack.mitre.org/software/S0438) has a plugin that collects data stored in the Windows clipboard by using the OpenClipboard and GetClipboardData APIs.[[13]](https://www.welivesecurity.com/wp-content/uploads/2019/10/ESET_Attor.pdf)

|  [S1226](https://attack.mitre.org/software/S1226)   |  [BOOKWORM](https://attack.mitre.org/software/S1226)   |

[BOOKWORM](https://attack.mitre.org/software/S1226) has used its KBLogger.dll module to steal data saved to the clipboard. [[14]](https://unit42.paloaltonetworks.com/bookworm-trojan-a-model-of-modular-architecture/)

|  [S0454](https://attack.mitre.org/software/S0454)   |  [Cadelspy](https://attack.mitre.org/software/S0454)   |

[Cadelspy](https://attack.mitre.org/software/S0454) has the ability to steal data from the clipboard.[[15]](https://www.symantec.com/connect/blogs/iran-based-attackers-use-back-door-threats-spy-middle-eastern-targets)

|  [S0261](https://attack.mitre.org/software/S0261)   |  [Catchamas](https://attack.mitre.org/software/S0261)   |

[Catchamas](https://attack.mitre.org/software/S0261) steals data stored in the clipboard.[[16]](https://web.archive.org/web/20190508165711/https://www-west.symantec.com/content/symantec/english/en/security-center/writeup.html/2018-040209-1742-99)

|  [S1149](https://attack.mitre.org/software/S1149)   |  [CHIMNEYSWEEP](https://attack.mitre.org/software/S1149)   |

[CHIMNEYSWEEP](https://attack.mitre.org/software/S1149) can capture content from the clipboard.[[17]](https://cloud.google.com/blog/topics/threat-intelligence/likely-iranian-threat-actor-conducts-politically-motivated-disruptive-activity-against/)

|  [S0660](https://attack.mitre.org/software/S0660)   |  [Clambling](https://attack.mitre.org/software/S0660)   |

[Clambling](https://attack.mitre.org/software/S0660) has the ability to capture and store clipboard data.[[18]](https://documents.trendmicro.com/assets/white_papers/wp-uncovering-DRBcontrol.pdf)[[19]](https://www.talent-jump.com/article/2020/02/17/CLAMBLING-A-New-Backdoor-Base-On-Dropbox-en/)

|  [S0050](https://attack.mitre.org/software/S0050)   |  [CosmicDuke](https://attack.mitre.org/software/S0050)   |

[CosmicDuke](https://attack.mitre.org/software/S0050) copies and exfiltrates the clipboard contents every 30 seconds.[[20]](https://blog.f-secure.com/wp-content/uploads/2019/10/CosmicDuke.pdf)

|  [S0334](https://attack.mitre.org/software/S0334)   |  [DarkComet](https://attack.mitre.org/software/S0334)   |

[DarkComet](https://attack.mitre.org/software/S0334) can steal data from the clipboard.[[21]](https://blog.malwarebytes.com/threat-analysis/2012/06/you-dirty-rat-part-1-darkcomet/)

|  [S1111](https://attack.mitre.org/software/S1111)   |  [DarkGate](https://attack.mitre.org/software/S1111)   |

[DarkGate](https://attack.mitre.org/software/S1111) starts a thread on execution that captures clipboard data and logs it to a predefined log file.[[22]](https://www.fortinet.com/blog/threat-research/enter-the-darkgate-new-cryptocurrency-mining-and-ransomware-campaign)[[23]](https://www.rapid7.com/blog/post/2024/12/04/black-basta-ransomware-campaign-drops-zbot-darkgate-and-custom-malware/)

|  [S1066](https://attack.mitre.org/software/S1066)   |  [DarkTortilla](https://attack.mitre.org/software/S1066)   |

[DarkTortilla](https://attack.mitre.org/software/S1066) can download a clipboard information stealer module.[[24]](https://www.secureworks.com/research/darktortilla-malware-analysis)

|  [S0363](https://attack.mitre.org/software/S0363)   |  [Empire](https://attack.mitre.org/software/S0363)   |

[Empire](https://attack.mitre.org/software/S0363) can harvest clipboard data on both Windows and macOS systems.[[25]](https://github.com/PowerShellEmpire/Empire)

|  [S0569](https://attack.mitre.org/software/S0569)   |  [Explosive](https://attack.mitre.org/software/S0569)   |

[Explosive](https://attack.mitre.org/software/S0569) has a function to use the OpenClipboard wrapper.[[26]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2015/03/20082004/volatile-cedar-technical-report.pdf)

|  [S0381](https://attack.mitre.org/software/S0381)   |  [FlawedAmmyy](https://attack.mitre.org/software/S0381)   |

[FlawedAmmyy](https://attack.mitre.org/software/S0381) can collect clipboard data.[[27]](https://www.fsec.or.kr/user/bbs/fsec/163/344/bbsDataView/1382.do?page=1&column=&search=&searchSDate=&searchEDate=&bbsDataCategory=)

|  [S0531](https://attack.mitre.org/software/S0531)   |  [Grandoreiro](https://attack.mitre.org/software/S0531)   |

[Grandoreiro](https://attack.mitre.org/software/S0531) can capture clipboard data from a compromised host.[[28]](https://securityintelligence.com/posts/grandoreiro-malware-now-targeting-banks-in-spain/)

|  [S0170](https://attack.mitre.org/software/S0170)   |  [Helminth](https://attack.mitre.org/software/S0170)   |

The executable version of [Helminth](https://attack.mitre.org/software/S0170) has a module to log clipboard contents.[[29]](http://researchcenter.paloaltonetworks.com/2016/05/the-oilrig-campaign-attacks-on-saudi-arabian-organizations-deliver-helminth-backdoor/)

|  [S1245](https://attack.mitre.org/software/S1245)   |  [InvisibleFerret](https://attack.mitre.org/software/S1245)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has stolen data from the clipboard using the Python project "pyperclip".[[30]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[31]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[32]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also captured clipboard contents during copy and paste operations.[[33]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  [S0044](https://attack.mitre.org/software/S0044)   |  [JHUHUGIT](https://attack.mitre.org/software/S0044)   |

A [JHUHUGIT](https://attack.mitre.org/software/S0044) variant accesses a screenshot saved in the clipboard and converts it to a JPG image.[[34]](https://web.archive.org/web/20230702043312/https://pan-unit42.github.io/playbook_viewer/)

|  [S0283](https://attack.mitre.org/software/S0283)   |  [jRAT](https://attack.mitre.org/software/S0283)   |

[jRAT](https://attack.mitre.org/software/S0283) can capture clipboard data.[[35]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07195002/KL_AdwindPublicReport_2016.pdf)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has the ability to steal data from the clipboard.[[36]](https://www.aryaka.com/docs/reports/aryaka-kimsuky-apt-operational-blueprint.pdf)

|  [S0250](https://attack.mitre.org/software/S0250)   |  [Koadic](https://attack.mitre.org/software/S0250)   |

[Koadic](https://attack.mitre.org/software/S0250) can retrieve the current content of the user clipboard.[[37]](https://github.com/offsecginger/koadic)

|  [S0356](https://attack.mitre.org/software/S0356)   |  [KONNI](https://attack.mitre.org/software/S0356)   |

[KONNI](https://attack.mitre.org/software/S0356) had a feature to steal data from the clipboard.[[38]](https://blog.talosintelligence.com/2017/05/konni-malware-under-radar-for-years.html)

|  [S0409](https://attack.mitre.org/software/S0409)   |  [Machete](https://attack.mitre.org/software/S0409)   |

[Machete](https://attack.mitre.org/software/S0409) hijacks the clipboard data by creating an overlapped window that listens to keyboard events.[[39]](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)[[40]](https://securelist.com/el-machete/66108/)

|  [S0282](https://attack.mitre.org/software/S0282)   |  [MacSpy](https://attack.mitre.org/software/S0282)   |

[MacSpy](https://attack.mitre.org/software/S0282) can steal clipboard contents.[[41]](https://objective-see.com/blog/blog_0x25.html)

|  [S0652](https://attack.mitre.org/software/S0652)   |  [MarkiRAT](https://attack.mitre.org/software/S0652)   |

[MarkiRAT](https://attack.mitre.org/software/S0652) can capture clipboard content.[[42]](https://securelist.com/ferocious-kitten-6-years-of-covert-surveillance-in-iran/102806/)

|  [S0530](https://attack.mitre.org/software/S0530)   |  [Melcoz](https://attack.mitre.org/software/S0530)   |

[Melcoz](https://attack.mitre.org/software/S0530) can monitor content saved to the clipboard.[[43]](https://securelist.com/the-tetrade-brazilian-banking-malware/97779/)

|  [S0455](https://attack.mitre.org/software/S0455)   |  [Metamorfo](https://attack.mitre.org/software/S0455)   |

[Metamorfo](https://attack.mitre.org/software/S0455) has a function to hijack data from the clipboard by monitoring the contents of the clipboard and replacing the cryptocurrency wallet with the attacker's.[[44]](https://www.fortinet.com/blog/threat-research/another-metamorfo-variant-targeting-customers-of-financial-institutions)[[45]](https://www.welivesecurity.com/2019/10/03/casbaneiro-trojan-dangerous-cooking/)

|  [S1146](https://attack.mitre.org/software/S1146)   |  [MgBot](https://attack.mitre.org/software/S1146)   |

[MgBot](https://attack.mitre.org/software/S1146) can capture clipboard data.[[46]](https://www.welivesecurity.com/2023/04/26/evasive-panda-apt-group-malware-updates-popular-chinese-software/)[[47]](https://symantec-enterprise-blogs.security.com/threat-intelligence/apt-attacks-telecoms-africa-mgbot)

|  [S1122](https://attack.mitre.org/software/S1122)   |  [Mispadu](https://attack.mitre.org/software/S1122)   |

[Mispadu](https://attack.mitre.org/software/S1122) has the ability to capture and replace Bitcoin wallet data in the clipboard on a compromised host.[[48]](https://www.welivesecurity.com/2019/11/19/mispadu-advertisement-discounted-unhappy-meal/)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) has used infostealer tools to copy clipboard data.[[49]](https://www.security.com/threat-intelligence/crambus-middle-east-government)

|  [C0014](https://attack.mitre.org/campaigns/C0014)   |  [Operation Wocao](https://attack.mitre.org/campaigns/C0014)   |

During [Operation Wocao](https://attack.mitre.org/campaigns/C0014), threat actors collected clipboard data in plaintext.[[50]](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)

|  [S1233](https://attack.mitre.org/software/S1233)   |  [PAKLOG](https://attack.mitre.org/software/S1233)   |

[PAKLOG](https://attack.mitre.org/software/S1233) has monitored and extracted clipboard contents.[[51]](https://www.zscaler.com/blogs/security-research/latest-mustang-panda-arsenal-paklog-corklog-and-splatcloak-p2)

|  [S0332](https://attack.mitre.org/software/S0332)   |  [Remcos](https://attack.mitre.org/software/S0332)   |

[Remcos](https://attack.mitre.org/software/S0332) steals and modifies data from the clipboard.[[52]](https://web.archive.org/web/20180124082756/https://www.riskiq.com/blog/labs/spear-phishing-turkish-defense-contractors/)[[53]](https://www.fortinet.com/blog/threat-research/new-campaign-uses-remcos-rat-to-exploit-victims)

|  [S0375](https://attack.mitre.org/software/S0375)   |  [Remexi](https://attack.mitre.org/software/S0375)   |

[Remexi](https://attack.mitre.org/software/S0375) collects text from the clipboard.[[54]](https://securelist.com/chafer-used-remexi-malware/89538/)

|  [S0240](https://attack.mitre.org/software/S0240)   |  [ROKRAT](https://attack.mitre.org/software/S0240)   |

[ROKRAT](https://attack.mitre.org/software/S0240) can extract clipboard data from a compromised host.[[55]](https://www.volexity.com/blog/2021/08/24/north-korean-bluelight-special-inkysquid-deploys-rokrat/)

|  [S0148](https://attack.mitre.org/software/S0148)   |  [RTM](https://attack.mitre.org/software/S0148)   |

[RTM](https://attack.mitre.org/software/S0148) collects data from the clipboard.[[56]](https://www.welivesecurity.com/wp-content/uploads/2017/02/Read-The-Manual.pdf)[[57]](https://unit42.paloaltonetworks.com/russian-language-malspam-pushing-redaman-banking-malware/)

|  [S0253](https://attack.mitre.org/software/S0253)   |  [RunningRAT](https://attack.mitre.org/software/S0253)   |

[RunningRAT](https://attack.mitre.org/software/S0253) contains code to open and copy data from the clipboard.[[58]](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/gold-dragon-widens-olympics-malware-attacks-gains-permanent-presence-on-victims-systems/)

|  [S0692](https://attack.mitre.org/software/S0692)   |  [SILENTTRINITY](https://attack.mitre.org/software/S0692)   |

[SILENTTRINITY](https://attack.mitre.org/software/S0692) can monitor Clipboard text and can use System.Windows.Forms.Clipboard.GetText() to collect data from the clipboard.[[59]](https://github.com/byt3bl33d3r/SILENTTRINITY)

|  [S0467](https://attack.mitre.org/software/S0467)   |  [TajMahal](https://attack.mitre.org/software/S0467)   |

[TajMahal](https://attack.mitre.org/software/S0467) has the ability to steal data from the clipboard of an infected host.[[60]](https://securelist.com/project-tajmahal/90240/)

|  [S0004](https://attack.mitre.org/software/S0004)   |  [TinyZBot](https://attack.mitre.org/software/S0004)   |

[TinyZBot](https://attack.mitre.org/software/S0004) contains functionality to collect information from the clipboard.[[61]](https://web.archive.org/web/20200302085133/https://www.cylance.com/content/dam/cylance/pages/operation-cleaver/Cylance_Operation_Cleaver_Report.pdf)

|  [S0257](https://attack.mitre.org/software/S0257)   |  [VERMIN](https://attack.mitre.org/software/S0257)   |

[VERMIN](https://attack.mitre.org/software/S0257) collects data stored in the clipboard.[[62]](https://researchcenter.paloaltonetworks.com/2018/01/unit42-vermin-quasar-rat-custom-malware-used-ukraine/)

|  [S1207](https://attack.mitre.org/software/S1207)   |  [XLoader](https://attack.mitre.org/software/S1207)   |

[XLoader](https://attack.mitre.org/software/S1207) can collect data stored in the victim's clipboard.[[63]](https://cloud.google.com/blog/topics/threat-intelligence/formbook-malware-distribution-campaigns/)[[64]](https://www.netskope.com/blog/new-formbook-campaign-delivered-through-phishing-emails)

|  [S0330](https://attack.mitre.org/software/S0330)   |  [Zeus Panda](https://attack.mitre.org/software/S0330)   |

[Zeus Panda](https://attack.mitre.org/software/S0330) can hook GetClipboardData function to watch for clipboard pastes to collect.[[65]](https://cyberwtf.files.wordpress.com/2017/07/panda-whitepaper.pdf)

## Mitigations

This type of attack technique cannot be easily mitigated with preventive controls since it is based on the abuse of system features.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0341](https://attack.mitre.org/detectionstrategies/DET0341)   |  [Clipboard Data Access with Anomalous Context](https://attack.mitre.org/detectionstrategies/DET0341)   |  [AN0965](https://attack.mitre.org/detectionstrategies/DET0341#AN0965)   |

Detection of clipboard access via OS utilities (e.g., clip.exe, Get-Clipboard) by non-interactive or abnormal parent processes, potentially chained with staging or exfiltration commands.

|  [AN0966](https://attack.mitre.org/detectionstrategies/DET0341#AN0966)   |

Detection of pbpaste/pbcopy clipboard access by processes without terminal sessions or linked to launch agents, potentially staged for collection.

|  [AN0967](https://attack.mitre.org/detectionstrategies/DET0341#AN0967)   |

Detection of xclip or xsel access to clipboard buffers outside of user terminal context, especially when chained to staging (gzip, base64) or network exfiltration (curl, scp).

## References

-    [Microsoft. (n.d.). About the Clipboard. Retrieved March 29, 2016.](https://msdn.microsoft.com/en-us/library/ms649012)
-    [Microsoft, JasonGerend, et al. (2023, February 3). clip. Retrieved June 21, 2022.](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/clip)
-    [CISA. (2021, August 20). Alert (AA21-200B) Chinese State-Sponsored Cyber Operations: Observed TTPs. Retrieved June 21, 2022.](https://www.cisa.gov/uscert/ncas/alerts/aa21-200b)
-    [Maljic, T. (2020, April 16). Mining for malicious Ruby gems. Retrieved October 15, 2022.](https://blog.reversinglabs.com/blog/mining-for-malicious-ruby-gems)
-    [rvrsh3ll. (2016, May 18). Operating with EmPyre. Retrieved July 12, 2017.](https://medium.com/rvrsh3ll/operating-with-empyre-ea764eda3363)
-    [Brumaghin, E., et al. (2018, October 15). Old dog, new tricks - Analysing new RTF-based campaign distributing Agent Tesla, Loki with PyREbox. Retrieved November 5, 2018.](https://blog.talosintelligence.com/2018/10/old-dog-new-tricks-analysing-new-rtf_15.html)
-    [Zhang, X. (2018, April 05). Analysis of New Agent Tesla Spyware Variant. Retrieved November 5, 2018.](https://www.fortinet.com/blog/threat-research/analysis-of-new-agent-tesla-spyware-variant.html)
-    [Zhang, X. (2017, June 28). In-Depth Analysis of A New Variant of .NET Malware AgentTesla. Retrieved November 5, 2018.](https://www.fortinet.com/blog/threat-research/in-depth-analysis-of-net-malware-javaupdtr.html)
-    [Arsene, L. (2020, April 21). Oil & Gas Spearphishing Campaigns Drop Agent Tesla Spyware in Advance of Historic OPEC+ Deal. Retrieved May 19, 2020.](https://labs.bitdefender.com/2020/04/oil-gas-spearphishing-campaigns-drop-agent-tesla-spyware-in-advance-of-historic-opec-deal/)
-    [FireEye. (2018, October 03). APT38: Un-usual Suspects. Retrieved November 17, 2024.](https://services.google.com/fh/files/misc/apt38-un-usual-suspects.pdf)
-    [Symantec. (2018, February 28). Chafer: Latest Attacks Reveal Heightened Ambitions. Retrieved May 22, 2020.](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/chafer-latest-attacks-reveal-heightened-ambitions)
-    [Salem, E. (2019, February 13). ASTAROTH MALWARE USES LEGITIMATE OS AND ANTIVIRUS PROCESSES TO STEAL PASSWORDS AND PERSONAL DATA. Retrieved April 17, 2019.](https://www.cybereason.com/blog/information-stealing-malware-targeting-brazil-full-research)
-    [Hromcova, Z. (2019, October). AT COMMANDS, TOR-BASED COMMUNICATIONS: MEET ATTOR, A FANTASY CREATURE AND ALSO A SPY PLATFORM. Retrieved May 6, 2020.](https://www.welivesecurity.com/wp-content/uploads/2019/10/ESET_Attor.pdf)
-    [Robert Falcone, Mike Scott, Juan Cortes. (2015, November 10). Bookworm Trojan: A Model of Modular Architecture. Retrieved July 21, 2025.](https://unit42.paloaltonetworks.com/bookworm-trojan-a-model-of-modular-architecture/)
-    [Symantec Security Response. (2015, December 7). Iran-based attackers use back door threats to spy on Middle Eastern targets. Retrieved April 17, 2019.](https://www.symantec.com/connect/blogs/iran-based-attackers-use-back-door-threats-spy-middle-eastern-targets)
-    [Balanza, M. (2018, April 02). Infostealer.Catchamas. Retrieved November 17, 2024.](https://web.archive.org/web/20190508165711/https://www-west.symantec.com/content/symantec/english/en/security-center/writeup.html/2018-040209-1742-99)
-    [Jenkins, L. at al. (2022, August 4). ROADSWEEP Ransomware - Likely Iranian Threat Actor Conducts Politically Motivated Disruptive Activity Against Albanian Government Organizations. Retrieved August 6, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/likely-iranian-threat-actor-conducts-politically-motivated-disruptive-activity-against/)
-    [Lunghi, D. et al. (2020, February). Uncovering DRBControl. Retrieved November 12, 2021.](https://documents.trendmicro.com/assets/white_papers/wp-uncovering-DRBcontrol.pdf)
-    [Chen, T. and Chen, Z. (2020, February 17). CLAMBLING - A New Backdoor Base On Dropbox. Retrieved November 12, 2021.](https://www.talent-jump.com/article/2020/02/17/CLAMBLING-A-New-Backdoor-Base-On-Dropbox-en/)
-    [F-Secure Labs. (2014, July). COSMICDUKE Cosmu with a twist of MiniDuke. Retrieved July 3, 2014.](https://blog.f-secure.com/wp-content/uploads/2019/10/CosmicDuke.pdf)
-    [Kujawa, A. (2018, March 27). You dirty RAT! Part 1: DarkComet. Retrieved November 6, 2018.](https://blog.malwarebytes.com/threat-analysis/2012/06/you-dirty-rat-part-1-darkcomet/)
-    [Adi Zeligson & Rotem Kerner. (2018, November 13). Enter The DarkGate - New Cryptocurrency Mining and Ransomware Campaign. Retrieved February 9, 2024.](https://www.fortinet.com/blog/threat-research/enter-the-darkgate-new-cryptocurrency-mining-and-ransomware-campaign)
-    [McGraw, T. (2024, December 4). Black Basta Ransomware Campaign Drops Zbot, DarkGate, and Custom Malware. Retrieved December 9, 2024.](https://www.rapid7.com/blog/post/2024/12/04/black-basta-ransomware-campaign-drops-zbot-darkgate-and-custom-malware/)
-    [Secureworks Counter Threat Unit Research Team. (2022, August 17). DarkTortilla Malware Analysis. Retrieved November 3, 2022.](https://www.secureworks.com/research/darktortilla-malware-analysis)
-    [Schroeder, W., Warner, J., Nelson, M. (n.d.). Github PowerShellEmpire. Retrieved April 28, 2016.](https://github.com/PowerShellEmpire/Empire)
-    [Threat Intelligence and Research. (2015, March 30). VOLATILE CEDAR. Retrieved February 8, 2021.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2015/03/20082004/volatile-cedar-technical-report.pdf)
-    [Financial Security Institute. (2020, February 28). Profiling of TA505 Threat Group That Continues to Attack the Financial Sector. Retrieved July 14, 2022.](https://www.fsec.or.kr/user/bbs/fsec/163/344/bbsDataView/1382.do?page=1&column=&search=&searchSDate=&searchEDate=&bbsDataCategory=)
-    [Abramov, D. (2020, April 13). Grandoreiro Malware Now Targeting Banks in Spain. Retrieved November 12, 2020.](https://securityintelligence.com/posts/grandoreiro-malware-now-targeting-banks-in-spain/)
-    [Falcone, R. and Lee, B.. (2016, May 26). The OilRig Campaign: Attacks on Saudi Arabian Organizations Deliver Helminth Backdoor. Retrieved May 3, 2017.](http://researchcenter.paloaltonetworks.com/2016/05/the-oilrig-campaign-attacks-on-saudi-arabian-organizations-deliver-helminth-backdoor/)
-    [eSentire Threat Response Unit (TRU). (2024, November 14). Bored BeaverTail & InvisibleFerret Yacht Club – A Lazarus Lure Pt.2. Retrieved October 17, 2025.](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)
-    [Matej Havranek. (2025, February 20). DeceptiveDevelopment targets freelance developers. Retrieved October 17, 2025.](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)
-    [Unit 42. (2023, November 21). Hacking Employers and Seeking Employment: Two Job-Related Campaigns Bear Hallmarks of North Korean Threat Actors. Retrieved October 17, 2025.](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)
-    [Seongsu Park. (2024, November 4). From Pyongyang to Your Payroll: The Rise of North Korean Remote Workers in the West. Retrieved October 17, 2025.](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

-    [Unit 42. (2017, December 15). Unit 42 Playbook Viewer. Retrieved December 20, 2017.](https://web.archive.org/web/20230702043312/https://pan-unit42.github.io/playbook_viewer/)
-    [Kamluk, V. & Gostev, A. (2016, February). Adwind - A Cross-Platform RAT. Retrieved April 23, 2019.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07195002/KL_AdwindPublicReport_2016.pdf)
-    [Varadharajan Krishnasamy, Aditya K Sood. (2025, July 29). From Reconnaissance to Control: The Operational Blueprint of Kimsuky APT for Cyber Espionage. Retrieved April 18, 2026.](https://www.aryaka.com/docs/reports/aryaka-kimsuky-apt-operational-blueprint.pdf)
-    [Magius, J., et al. (2017, July 19). Koadic. Retrieved September 27, 2024.](https://github.com/offsecginger/koadic)
-    [Rascagneres, P. (2017, May 03). KONNI: A Malware Under The Radar For Years. Retrieved November 5, 2018.](https://blog.talosintelligence.com/2017/05/konni-malware-under-radar-for-years.html)
-    [ESET. (2019, July). MACHETE JUST GOT SHARPER Venezuelan government institutions under attack. Retrieved September 13, 2019.](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)
-    [Kaspersky Global Research and Analysis Team. (2014, August 20). El Machete. Retrieved September 13, 2019.](https://securelist.com/el-machete/66108/)
-    [Patrick Wardle. (n.d.). Mac Malware of 2017. Retrieved September 21, 2018.](https://objective-see.com/blog/blog_0x25.html)
-    [GReAT. (2021, June 16). Ferocious Kitten: 6 Years of Covert Surveillance in Iran. Retrieved September 22, 2021.](https://securelist.com/ferocious-kitten-6-years-of-covert-surveillance-in-iran/102806/)
-    [GReAT. (2020, July 14). The Tetrade: Brazilian banking malware goes global. Retrieved November 9, 2020.](https://securelist.com/the-tetrade-brazilian-banking-malware/97779/)
-    [Zhang, X. (2020, February 4). Another Metamorfo Variant Targeting Customers of Financial Institutions in More Countries. Retrieved July 30, 2020.](https://www.fortinet.com/blog/threat-research/another-metamorfo-variant-targeting-customers-of-financial-institutions)
-    [ESET Research. (2019, October 3). Casbaneiro: peculiarities of this banking Trojan that affects Brazil and Mexico. Retrieved September 23, 2021.](https://www.welivesecurity.com/2019/10/03/casbaneiro-trojan-dangerous-cooking/)
-    [Facundo Muñoz. (2023, April 26). Evasive Panda APT group delivers malware via updates for popular Chinese software. Retrieved July 25, 2024.](https://www.welivesecurity.com/2023/04/26/evasive-panda-apt-group-malware-updates-popular-chinese-software/)
-    [Threat Hunter Team. (2023, April 20). Daggerfly: APT Actor Targets Telecoms Company in Africa. Retrieved July 25, 2024.](https://symantec-enterprise-blogs.security.com/threat-intelligence/apt-attacks-telecoms-africa-mgbot)
-    [ESET Security. (2019, November 19). Mispadu: Advertisement for a discounted Unhappy Meal. Retrieved March 13, 2024.](https://www.welivesecurity.com/2019/11/19/mispadu-advertisement-discounted-unhappy-meal/)
-    [Symantec Threat Hunter Team. (2023, October 19). Crambus: New Campaign Targets Middle Eastern Government. Retrieved November 27, 2024.](https://www.security.com/threat-intelligence/crambus-middle-east-government)
-    [Dantzig, M. v., Schamper, E. (2019, December 19). Operation Wocao: Shining a light on one of China’s hidden hacking groups. Retrieved October 8, 2020.](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)
-    [Sudeep Singh. (2025, April 16). Latest Mustang Panda Arsenal: PAKLOG, CorKLOG, and SplatCloak | P2. Retrieved September 12, 2025.](https://www.zscaler.com/blogs/security-research/latest-mustang-panda-arsenal-paklog-corklog-and-splatcloak-p2)
-    [Klijnsma, Y. (2018, January 23). Espionage Campaign Leverages Spear Phishing, RATs Against Turkish Defense Contractors. Retrieved November 6, 2018.](https://web.archive.org/web/20180124082756/https://www.riskiq.com/blog/labs/spear-phishing-turkish-defense-contractors/)
-    [Zhang, X. (2024, November 8). New Campaign Uses Remcos RAT to Exploit Victims. Retrieved April 16, 2026.](https://www.fortinet.com/blog/threat-research/new-campaign-uses-remcos-rat-to-exploit-victims)
-    [Legezo, D. (2019, January 30). Chafer used Remexi malware to spy on Iran-based foreign diplomatic entities. Retrieved April 17, 2019.](https://securelist.com/chafer-used-remexi-malware/89538/)
-    [Cash, D., Grunzweig, J., Adair, S., Lancaster, T. (2021, August 25). North Korean BLUELIGHT Special: InkySquid Deploys RokRAT. Retrieved October 1, 2021.](https://www.volexity.com/blog/2021/08/24/north-korean-bluelight-special-inkysquid-deploys-rokrat/)
-    [Faou, M. and Boutin, J. (2017, February). Read The Manual: A Guide to the RTM Banking Trojan. Retrieved March 9, 2017.](https://www.welivesecurity.com/wp-content/uploads/2017/02/Read-The-Manual.pdf)
-    [Duncan, B., Harbison, M. (2019, January 23). Russian Language Malspam Pushing Redaman Banking Malware. Retrieved June 16, 2020.](https://unit42.paloaltonetworks.com/russian-language-malspam-pushing-redaman-banking-malware/)
-    [Sherstobitoff, R., Saavedra-Morales, J. (2018, February 02). Gold Dragon Widens Olympics Malware Attacks, Gains Permanent Presence on Victims’ Systems. Retrieved June 6, 2018.](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/gold-dragon-widens-olympics-malware-attacks-gains-permanent-presence-on-victims-systems/)
-    [byt3bl33d3r. (n.d.). SILENTTRINITY. Retrieved September 12, 2024.](https://github.com/byt3bl33d3r/SILENTTRINITY)
-    [GReAT. (2019, April 10). Project TajMahal – a sophisticated new APT framework. Retrieved October 14, 2019.](https://securelist.com/project-tajmahal/90240/)
-    [Cylance. (2014, December). Operation Cleaver. Retrieved September 14, 2017.](https://web.archive.org/web/20200302085133/https://www.cylance.com/content/dam/cylance/pages/operation-cleaver/Cylance_Operation_Cleaver_Report.pdf)
-    [Lancaster, T., Cortes, J. (2018, January 29). VERMIN: Quasar RAT and Custom Malware Used In Ukraine. Retrieved July 5, 2018.](https://researchcenter.paloaltonetworks.com/2018/01/unit42-vermin-quasar-rat-custom-malware-used-ukraine/)
-    [Nart Villeneuve, Randi Eitzman, Sandor Nemes & Tyler Dean, Google Cloud. (2017, October 5). Significant FormBook Distribution Campaigns Impacting the U.S. and South Korea. Retrieved March 11, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/formbook-malware-distribution-campaigns/)
-    [Gustavo Palazolo, Netskope. (2022, March 11). New Formbook Campaign Delivered Through Phishing Emails. Retrieved March 11, 2025.](https://www.netskope.com/blog/new-formbook-campaign-delivered-through-phishing-emails)
-    [Ebach, L. (2017, June 22). Analysis Results of Zeus.Variant.Panda. Retrieved November 5, 2018.](https://cyberwtf.files.wordpress.com/2017/07/panda-whitepaper.pdf)

×
