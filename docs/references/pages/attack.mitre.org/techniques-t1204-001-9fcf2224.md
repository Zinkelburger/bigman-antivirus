---
url: "https://attack.mitre.org/techniques/T1204/001/"
title: "User Execution: Malicious Link, Sub-technique T1204.001 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:40:40Z"
via: "live"
sha256: "ce71899f0110a40e72edc84363207266990e80ac226c7b58bba0021cd38bd7ce"
wayback: "https://web.archive.org/web/20261023090735/https://attack.mitre.org/techniques/T1204/001/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# User Execution: Malicious Link, Sub-technique T1204.001 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1204/001/>
> Snapshot: <https://web.archive.org/web/20261023090735/https://attack.mitre.org/techniques/T1204/001/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [User Execution](https://attack.mitre.org/techniques/T1204)
- Malicious Link

#  User Execution: Malicious Link

##### Other sub-techniques of User Execution (5)

| ID  | Name

|  T1204.001   |  Malicious Link

|  [T1204.002](https://attack.mitre.org/techniques/T1204/002/)   |  [Malicious File](https://attack.mitre.org/techniques/T1204/002/)

|  [T1204.003](https://attack.mitre.org/techniques/T1204/003/)   |  [Malicious Image](https://attack.mitre.org/techniques/T1204/003/)

|  [T1204.004](https://attack.mitre.org/techniques/T1204/004/)   |  [Malicious Copy and Paste](https://attack.mitre.org/techniques/T1204/004/)

|  [T1204.005](https://attack.mitre.org/techniques/T1204/005/)   |  [Malicious Library](https://attack.mitre.org/techniques/T1204/005/)

An adversary may rely upon a user clicking a malicious link in order to gain execution. Users may be subjected to social engineering to get them to click on a link that will lead to code execution. This user action will typically be observed as follow-on behavior from [Spearphishing Link](https://attack.mitre.org/techniques/T1566/002). Clicking on a link may also lead to other execution techniques such as exploitation of a browser or application vulnerability via [Exploitation for Client Execution](https://attack.mitre.org/techniques/T1203). Links may also lead users to download files that require execution via [Malicious File](https://attack.mitre.org/techniques/T1204/002).

ID: T1204.001

Sub-technique of:  [T1204](https://attack.mitre.org/techniques/T1204)

ⓘ

Tactic: [Execution](https://attack.mitre.org/tactics/TA0002)

ⓘ

Platforms: Linux, Windows, macOS

Version: 1.2

Created: 11 March 2020

Last Modified: 11 September 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1204/001/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1204/001/)

## Procedure Examples

| ID  | Name  | Description

|  [S0584](https://attack.mitre.org/software/S0584)   |  [AppleJeus](https://attack.mitre.org/software/S0584)   |

[AppleJeus](https://attack.mitre.org/software/S0584)'s spearphishing links required user interaction to navigate to the malicious website.[[1]](https://us-cert.cisa.gov/ncas/alerts/aa21-048a)

|  [G0099](https://attack.mitre.org/groups/G0099)   |  [APT-C-36](https://attack.mitre.org/groups/G0099)   |

[APT-C-36](https://attack.mitre.org/groups/G0099) has used malicious links in emails, often impersonating official notifications and documents, to direct users to execute malicious payloads.[[2]](https://securelist.com/blindeagle-apt/113414/)

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[APT28](https://attack.mitre.org/groups/G0007) has tricked unwitting recipients into clicking on malicious hyperlinks within emails crafted to resemble trustworthy senders.[[3]](https://www.justice.gov/opa/page/file/1098481/download)[[4]](https://www.secureworks.com/research/iron-twilight-supports-active-measures)

|  [G0016](https://attack.mitre.org/groups/G0016)   |  [APT29](https://attack.mitre.org/groups/G0016)   |

[APT29](https://attack.mitre.org/groups/G0016) has used various forms of spearphishing attempting to get a user to click on a malicious link.[[5]](https://www.microsoft.com/security/blog/2021/05/27/new-sophisticated-email-based-attack-from-nobelium/)[[6]](https://www.secureworks.com/blog/usaid-themed-phishing-campaign-leverages-us-elections-lure)

|  [G0022](https://attack.mitre.org/groups/G0022)   |  [APT3](https://attack.mitre.org/groups/G0022)   |

[APT3](https://attack.mitre.org/groups/G0022) has lured victims into clicking malicious links delivered through spearphishing.[[7]](https://www.fireeye.com/blog/threat-research/2015/06/operation-clandestine-wolf-adobe-flash-zero-day.html)

|  [G0050](https://attack.mitre.org/groups/G0050)   |  [APT32](https://attack.mitre.org/groups/G0050)   |

[APT32](https://attack.mitre.org/groups/G0050) has lured targets to download a Cobalt Strike beacon by including a malicious link within spearphishing emails.[[8]](https://cdn2.hubspot.net/hubfs/3354902/Cybereason%20Labs%20Analysis%20Operation%20Cobalt%20Kitty.pdf)[[9]](https://www.volexity.com/blog/2020/11/06/oceanlotus-extending-cyber-espionage-operations-through-fake-websites/)[[10]](https://www.amnestyusa.org/wp-content/uploads/2021/02/Click-and-Bait_Vietnamese-Human-Rights-Defenders-Targeted-with-Spyware-Attacks.pdf)

|  [G0064](https://attack.mitre.org/groups/G0064)   |  [APT33](https://attack.mitre.org/groups/G0064)   |

[APT33](https://attack.mitre.org/groups/G0064) has lured users to click links to malicious HTML applications delivered via spearphishing emails.[[11]](https://www.fireeye.com/blog/threat-research/2017/09/apt33-insights-into-iranian-cyber-espionage.html)[[12]](https://www.symantec.com/blogs/threat-intelligence/elfin-apt33-espionage)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has used links to execute a malicious Visual Basic script.[[13]](https://securelist.com/bluenoroff-methods-bypass-motw/108383/) [APT38](https://attack.mitre.org/groups/G0082) has directed users to navigate to a website under adversary control through the sharing of links in messages.[[14]](https://www.microsoft.com/en-us/security/blog/2024/11/22/microsoft-shares-latest-intelligence-on-north-korean-and-chinese-threat-actors-at-cyberwarcon/)[[15]](https://www.microsoft.com/en-us/security/blog/2026/06/17/postinstall-payload-inside-mastra-npm-supply-chain-compromise/)[[16]](https://www.microsoft.com/en-us/security/blog/2026/04/01/mitigating-the-axios-npm-supply-chain-compromise/)

|  [G0087](https://attack.mitre.org/groups/G0087)   |  [APT39](https://attack.mitre.org/groups/G0087)   |

[APT39](https://attack.mitre.org/groups/G0087) has sent spearphishing emails in an attempt to lure users to click on a malicious link.[[17]](https://www.fireeye.com/blog/threat-research/2019/01/apt39-iranian-cyber-espionage-group-focused-on-personal-information.html)[[18]](https://www.iranwatch.org/sites/default/files/public-intelligence-alert.pdf)

|  [S0475](https://attack.mitre.org/software/S0475)   |  [BackConfig](https://attack.mitre.org/software/S0475)   |

[BackConfig](https://attack.mitre.org/software/S0475) has compromised victims via links to URLs hosting malicious content.[[19]](https://unit42.paloaltonetworks.com/updated-backconfig-malware-targeting-government-and-military-organizations/)

|  [S0534](https://attack.mitre.org/software/S0534)   |  [Bazar](https://attack.mitre.org/software/S0534)   |

[Bazar](https://attack.mitre.org/software/S0534) can gain execution after a user clicks on a malicious link to decoy landing pages hosted on Google Docs.[[20]](https://www.cybereason.com/blog/a-bazar-of-tricks-following-team9s-development-cycles)[[21]](https://www.zscaler.com/blogs/research/spear-phishing-campaign-delivers-buer-and-bazar-malware)[[22]](https://www.crowdstrike.com/blog/wizard-spider-adversary-update/)

|  [G0098](https://attack.mitre.org/groups/G0098)   |  [BlackTech](https://attack.mitre.org/groups/G0098)   |

[BlackTech](https://attack.mitre.org/groups/G0098) has used e-mails with malicious links to lure victims into installing malware.[[23]](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)

|  [S1039](https://attack.mitre.org/software/S1039)   |  [Bumblebee](https://attack.mitre.org/software/S1039)   |

[Bumblebee](https://attack.mitre.org/software/S1039) has relied upon a user downloading a file from a OneDrive link for execution.[[24]](https://www.proofpoint.com/us/blog/threat-insight/bumblebee-is-still-transforming)[[25]](https://www.cybereason.com/blog/threat-analysis-report-bumblebee-loader-the-high-road-to-enterprise-domain-control)

|  [C0011](https://attack.mitre.org/campaigns/C0011)   |  [C0011](https://attack.mitre.org/campaigns/C0011)   |

During [C0011](https://attack.mitre.org/campaigns/C0011), [Transparent Tribe](https://attack.mitre.org/groups/G0134) relied on student targets to click on a malicious link sent via email.[[26]](https://blog.talosintelligence.com/2022/07/transparent-tribe-targets-education.html)

|  [C0021](https://attack.mitre.org/campaigns/C0021)   |  [C0021](https://attack.mitre.org/campaigns/C0021)   |

During [C0021](https://attack.mitre.org/campaigns/C0021), the threat actors lured users into clicking a malicious link which led to the download of a ZIP archive containing a malicious .LNK file.[[27]](https://www.fireeye.com/blog/threat-research/2018/11/not-so-cozy-an-uncomfortable-examination-of-a-suspected-apt29-phishing-campaign.html)

|  [G0080](https://attack.mitre.org/groups/G0080)   |  [Cobalt Group](https://attack.mitre.org/groups/G0080)   |

[Cobalt Group](https://attack.mitre.org/groups/G0080) has sent emails containing malicious links that require users to execute a file or macro to infect the victim machine.[[28]](https://blog.talosintelligence.com/2018/07/multiple-cobalt-personality-disorder.html)[[29]](https://researchcenter.paloaltonetworks.com/2018/10/unit42-new-techniques-uncover-attribute-cobalt-gang-commodity-builders-infrastructure-revealed/)[[30]](https://www.secureworks.com/blog/cybercriminals-increasingly-trying-to-ensnare-the-big-financial-fish)

|  [G0142](https://attack.mitre.org/groups/G0142)   |  [Confucius](https://attack.mitre.org/groups/G0142)   |

[Confucius](https://attack.mitre.org/groups/G0142) has lured victims into clicking on a malicious link sent through spearphishing.[[31]](https://www.trendmicro.com/en_us/research/21/h/confucius-uses-pegasus-spyware-related-lures-to-target-pakistani.html)

|  [G1052](https://attack.mitre.org/groups/G1052)   |  [Contagious Interview](https://attack.mitre.org/groups/G1052)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has lured victims to click on a malicious link that led to download of a malicious payload.[[32]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat) [Contagious Interview](https://attack.mitre.org/groups/G1052) has also leveraged links to malicious payloads on social media and code repositories.[[32]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)

|  [G1034](https://attack.mitre.org/groups/G1034)   |  [Daggerfly](https://attack.mitre.org/groups/G1034)   |

[Daggerfly](https://attack.mitre.org/groups/G1034) has used strategic website compromise to deliver a malicious link requiring user interaction.[[33]](https://www.welivesecurity.com/en/eset-research/evasive-panda-leverages-monlam-festival-target-tibetans/)

|  [G1006](https://attack.mitre.org/groups/G1006)   |  [Earth Lusca](https://attack.mitre.org/groups/G1006)   |

[Earth Lusca](https://attack.mitre.org/groups/G1006) has sent spearphishing emails that required the user to click on a malicious link and subsequently open a decoy document with a malicious loader.[[34]](https://www.trendmicro.com/content/dam/trendmicro/global/en/research/22/a/earth-lusca-employs-sophisticated-infrastructure-varied-tools-and-techniques/technical-brief-delving-deep-an-analysis-of-earth-lusca-operations.pdf)

|  [G0066](https://attack.mitre.org/groups/G0066)   |  [Elderwood](https://attack.mitre.org/groups/G0066)   |

[Elderwood](https://attack.mitre.org/groups/G0066) has leveraged multiple types of spearphishing in order to attempt to get a user to open links.[[35]](https://web.archive.org/web/20190717233006/http:/www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/the-elderwood-project.pdf)[[36]](https://www.csmonitor.com/USA/2012/0914/Stealing-US-business-secrets-Experts-ID-two-huge-cyber-gangs-in-China)

|  [S0367](https://attack.mitre.org/software/S0367)   |  [Emotet](https://attack.mitre.org/software/S0367)   |

[Emotet](https://attack.mitre.org/software/S0367) has relied upon users clicking on a malicious link delivered through spearphishing.[[37]](https://blog.trendmicro.com/trendlabs-security-intelligence/new-banking-malware-uses-network-sniffing-for-data-theft/)[[38]](https://www.carbonblack.com/2019/04/24/cb-tau-threat-intelligence-notification-emotet-utilizing-wmi-to-launch-powershell-encoded-code/)

|  [G0120](https://attack.mitre.org/groups/G0120)   |  [Evilnum](https://attack.mitre.org/groups/G0120)   |

[Evilnum](https://attack.mitre.org/groups/G0120) has sent spearphishing emails designed to trick the recipient into opening malicious shortcut links which downloads a .LNK file.[[39]](https://www.welivesecurity.com/2020/07/09/more-evil-deep-look-evilnum-toolset/)

|  [G1011](https://attack.mitre.org/groups/G1011)   |  [EXOTIC LILY](https://attack.mitre.org/groups/G1011)   |

[EXOTIC LILY](https://attack.mitre.org/groups/G1011) has used malicious links to lure users into executing malicious payloads.[[40]](https://blog.google/threat-analysis-group/exposing-initial-access-broker-ties-conti/)

|  [G0085](https://attack.mitre.org/groups/G0085)   |  [FIN4](https://attack.mitre.org/groups/G0085)   |

[FIN4](https://attack.mitre.org/groups/G0085) has lured victims to click malicious links delivered via spearphishing emails (often sent from compromised accounts).[[41]](https://web.archive.org/web/20220313062515/https://www.mandiant.com/sites/default/files/2021-09/rpt-fin4.pdf)[[42]](https://www2.fireeye.com/WBNR-14Q4NAMFIN4.html)

|  [G0046](https://attack.mitre.org/groups/G0046)   |  [FIN7](https://attack.mitre.org/groups/G0046)   |

[FIN7](https://attack.mitre.org/groups/G0046) has used malicious links to lure victims into downloading malware.[[43]](https://www.crowdstrike.com/blog/carbon-spider-embraces-big-game-hunting-part-1/)

|  [G0061](https://attack.mitre.org/groups/G0061)   |  [FIN8](https://attack.mitre.org/groups/G0061)   |

[FIN8](https://attack.mitre.org/groups/G0061) has used emails with malicious links to lure victims into installing malware.[[44]](https://web.archive.org/web/20170923102302/https://www.fireeye.com/blog/threat-research/2017/06/obfuscation-in-the-wild.html)[[45]](https://www.fireeye.com/blog/threat-research/2016/05/windows-zero-day-payment-cards.html)[[46]](https://www2.fireeye.com/WBNR-Know-Your-Enemy-UNC622-Spear-Phishing.html)

|  [G0047](https://attack.mitre.org/groups/G0047)   |  [Gamaredon Group](https://attack.mitre.org/groups/G0047)   |

[Gamaredon Group](https://attack.mitre.org/groups/G0047) has attempted to get users to click on a link pointing to a malicious HTML file leading to follow-on malicious content.[[47]](https://unit42.paloaltonetworks.com/trident-ursa/)[[48]](https://web-assets.esetstatic.com/wls/en/papers/white-papers/cyberespionage-gamaredon-way.pdf)

|  [S1138](https://attack.mitre.org/software/S1138)   |  [Gootloader](https://attack.mitre.org/software/S1138)   |

[Gootloader](https://attack.mitre.org/software/S1138) has been executed through malicious links presented to users as internet search results.[[49]](https://news.sophos.com/en-us/2021/03/01/gootloader-expands-its-payload-delivery-options/)[[50]](https://www.sentinelone.com/labs/gootloader-initial-access-as-a-service-platform-expands-its-search-for-high-value-targets/)

|  [S0531](https://attack.mitre.org/software/S0531)   |  [Grandoreiro](https://attack.mitre.org/software/S0531)   |

[Grandoreiro](https://attack.mitre.org/software/S0531) has used malicious links to gain execution on victim machines.[[51]](https://securityintelligence.com/posts/grandoreiro-malware-now-targeting-banks-in-spain/)[[52]](https://www.welivesecurity.com/2020/04/28/grandoreiro-how-engorged-can-exe-get/)

|  [S0561](https://attack.mitre.org/software/S0561)   |  [GuLoader](https://attack.mitre.org/software/S0561)   |

[GuLoader](https://attack.mitre.org/software/S0561) has relied upon users clicking on links to malicious documents.[[53]](https://unit42.paloaltonetworks.com/guloader-installing-netwire-rat/)

|  [S0499](https://attack.mitre.org/software/S0499)   |  [Hancitor](https://attack.mitre.org/software/S0499)   |

[Hancitor](https://attack.mitre.org/software/S0499) has relied upon users clicking on a malicious link delivered through phishing.[[54]](https://threatpost.com/spammers-revive-hancitor-downloader-campaigns/123011/)

|  [S0528](https://attack.mitre.org/software/S0528)   |  [Javali](https://attack.mitre.org/software/S0528)   |

[Javali](https://attack.mitre.org/software/S0528) has achieved execution through victims clicking links to malicious websites.[[55]](https://securelist.com/the-tetrade-brazilian-banking-malware/97779/)

|  [S9044](https://attack.mitre.org/software/S9044)   |  [Kali365](https://attack.mitre.org/software/S9044)   |

[Kali365](https://attack.mitre.org/software/S9044) has directed victims to actor-controlled phishing pages through malicious links, initiating device code authorization flows or adversary-in-the-middle session capture. [[56]](https://arcticwolf.com/resources/blog/token-bingo-dont-let-your-code-be-the-winner/)[[57]](https://www.huntress.com/blog/kali365-device-code-phishing-kit)

|  [S0585](https://attack.mitre.org/software/S0585)   |  [Kerrdown](https://attack.mitre.org/software/S0585)   |

[Kerrdown](https://attack.mitre.org/software/S0585) has gained execution through victims opening malicious links.[[10]](https://www.amnestyusa.org/wp-content/uploads/2021/02/Click-and-Bait_Vietnamese-Human-Rights-Defenders-Targeted-with-Spyware-Attacks.pdf)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has lured victims into clicking malicious links.[[58]](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)

|  [S0669](https://attack.mitre.org/software/S0669)   |  [KOCTOPUS](https://attack.mitre.org/software/S0669)   |

[KOCTOPUS](https://attack.mitre.org/software/S0669) has relied on victims clicking on a malicious link delivered via email.[[59]](https://web.archive.org/web/20211003035156/https://www.malwarebytes.com/resources/files/2021/02/lazyscripter.pdf)

|  [S1160](https://attack.mitre.org/software/S1160)   |  [Latrodectus](https://attack.mitre.org/software/S1160)   |

[Latrodectus](https://attack.mitre.org/software/S1160) has been executed through malicious links distributed in email campaigns.[[60]](https://www.proofpoint.com/us/blog/threat-insight/latrodectus-spider-bytes-ice)[[61]](https://www.bleepingcomputer.com/news/security/new-latrodectus-malware-attacks-use-microsoft-cloudflare-themes/)

|  [G0140](https://attack.mitre.org/groups/G0140)   |  [LazyScripter](https://attack.mitre.org/groups/G0140)   |

[LazyScripter](https://attack.mitre.org/groups/G0140) has relied upon users clicking on links to malicious files.[[59]](https://web.archive.org/web/20211003035156/https://www.malwarebytes.com/resources/files/2021/02/lazyscripter.pdf)

|  [G0065](https://attack.mitre.org/groups/G0065)   |  [Leviathan](https://attack.mitre.org/groups/G0065)   |

[Leviathan](https://attack.mitre.org/groups/G0065) has sent spearphishing email links attempting to get a user to click.[[62]](https://www.proofpoint.com/us/threat-insight/post/leviathan-espionage-actor-spearphishes-maritime-and-defense-targets)[[63]](https://us-cert.cisa.gov/ncas/alerts/aa21-200a)

|  [G1014](https://attack.mitre.org/groups/G1014)   |  [LuminousMoth](https://attack.mitre.org/groups/G1014)   |

[LuminousMoth](https://attack.mitre.org/groups/G1014) has lured victims into clicking malicious Dropbox download links delivered through spearphishing.[[64]](https://securelist.com/apt-luminousmoth/103332/)

|  [G0095](https://attack.mitre.org/groups/G0095)   |  [Machete](https://attack.mitre.org/groups/G0095)   |

[Machete](https://attack.mitre.org/groups/G0095) has has relied on users opening malicious links delivered through spearphishing to execute malware.[[65]](https://threatvector.cylance.com/en_us/home/el-machete-malware-attacks-cut-through-latam.html)[[66]](https://securelist.com/el-machete/66108/)[[67]](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)

|  [G0059](https://attack.mitre.org/groups/G0059)   |  [Magic Hound](https://attack.mitre.org/groups/G0059)   |

[Magic Hound](https://attack.mitre.org/groups/G0059) has attempted to lure victims into opening malicious links embedded in emails.[[68]](https://www.clearskysec.com/wp-content/uploads/2020/08/The-Kittens-are-Back-in-Town-3.pdf)[[69]](https://blog.certfa.com/posts/charming-kitten-christmas-gift/)

|  [S0530](https://attack.mitre.org/software/S0530)   |  [Melcoz](https://attack.mitre.org/software/S0530)   |

[Melcoz](https://attack.mitre.org/software/S0530) has gained execution through victims opening malicious links.[[55]](https://securelist.com/the-tetrade-brazilian-banking-malware/97779/)

|  [G0103](https://attack.mitre.org/groups/G0103)   |  [Mofang](https://attack.mitre.org/groups/G0103)   |

[Mofang](https://attack.mitre.org/groups/G0103)'s spearphishing emails required a user to click the link to connect to a compromised website.[[70]](https://foxitsecurity.files.wordpress.com/2016/06/fox-it_mofang_threatreport_tlp-white.pdf)

|  [G0021](https://attack.mitre.org/groups/G0021)   |  [Molerats](https://attack.mitre.org/groups/G0021)   |

[Molerats](https://attack.mitre.org/groups/G0021) has sent malicious links via email trick users into opening a RAR archive and running an executable.[[71]](https://securelist.com/gaza-cybergang-group1-operation-sneakypastes/90068/)[[72]](https://unit42.paloaltonetworks.com/molerats-delivers-spark-backdoor/)

|  [G0069](https://attack.mitre.org/groups/G0069)   |  [MuddyWater](https://attack.mitre.org/groups/G0069)   |

[MuddyWater](https://attack.mitre.org/groups/G0069) has distributed URLs in phishing e-mails that link to lure documents.[[73]](https://www.anomali.com/blog/probable-iranian-cyber-actors-static-kitten-conducting-cyberespionage-campaign-targeting-uae-and-kuwait-government-agencies)[[74]](https://www.trendmicro.com/en_us/research/21/c/earth-vetala---muddywater-continues-to-target-organizations-in-t.html)[[75]](https://www.proofpoint.com/us/blog/threat-insight/security-brief-ta450-uses-embedded-links-pdf-attachments-latest-campaign)

|  [G0129](https://attack.mitre.org/groups/G0129)   |  [Mustang Panda](https://attack.mitre.org/groups/G0129)   |

[Mustang Panda](https://attack.mitre.org/groups/G0129) has sent malicious links including links directing victims to a Google Drive folder.[[76]](https://www.ibm.com/think/x-force/hive0154-mustang-panda-shifts-focus-tibetan-community-deploy-pubload-backdoor)[[77]](https://www.ibm.com/think/x-force/hive0154-targeting-us-philippines-pakistan-taiwan)[[78]](https://www.crowdstrike.com/blog/meet-crowdstrikes-adversary-of-the-month-for-june-mustang-panda/)[[79]](https://www.trendmicro.com/en_us/research/22/k/earth-preta-spear-phishing-governments-worldwide.html)[[80]](https://www.proofpoint.com/us/blog/threat-insight/good-bad-and-web-bug-ta416-increases-operational-tempo-against-european)[[81]](https://www.mcafee.com/enterprise/en-us/assets/reports/rp-operation-dianxun.pdf) [Mustang Panda](https://attack.mitre.org/groups/G0129) has also utilized webpages with Javascript code that downloads malicious payloads to the victim device.[[82]](https://cloud.google.com/blog/topics/threat-intelligence/prc-nexus-espionage-targets-diplomats)

|  [G1020](https://attack.mitre.org/groups/G1020)   |  [Mustard Tempest](https://attack.mitre.org/groups/G1020)   |

[Mustard Tempest](https://attack.mitre.org/groups/G1020) has lured users into downloading malware through malicious links in fake advertisements and spearphishing emails.[[83]](https://www.microsoft.com/en-us/security/blog/2022/05/09/ransomware-as-a-service-understanding-the-cybercrime-gig-economy-and-how-to-protect-yourself/)[[84]](https://www.proofpoint.com/us/blog/threat-insight/part-1-socgholish-very-real-threat-very-fake-update)

|  [S0198](https://attack.mitre.org/software/S0198)   |  [NETWIRE](https://attack.mitre.org/software/S0198)   |

[NETWIRE](https://attack.mitre.org/software/S0198) has been executed through convincing victims into clicking malicious links.[[85]](https://www.mandiant.com/resources/blog/dissecting-netwire-phishing-campaigns-usage-process-hollowing)[[53]](https://unit42.paloaltonetworks.com/guloader-installing-netwire-rat/)

|  [C0002](https://attack.mitre.org/campaigns/C0002)   |  [Night Dragon](https://attack.mitre.org/campaigns/C0002)   |

During [Night Dragon](https://attack.mitre.org/campaigns/C0002), threat actors enticed users to click on links in spearphishing emails to download malware.[[86]](https://scadahacker.com/library/Documents/Cyber_Events/McAfee%20-%20Night%20Dragon%20-%20Global%20Energy%20Cyberattacks.pdf)

|  [G1059](https://attack.mitre.org/groups/G1059)   |  [NIMBUS MANTICORE](https://attack.mitre.org/groups/G1059)   |

[NIMBUS MANTICORE](https://attack.mitre.org/groups/G1059) has enticed victims to click on links delivered in phishing campaigns.[[87]](https://research.checkpoint.com/2026/fast-and-furious-nimbus-manticore-operations-during-the-iranian-conflict/)[[88]](https://research.checkpoint.com/2025/nimbus-manticore-deploys-new-malware-targeting-europe/)[[89]](https://cloud.google.com/blog/topics/threat-intelligence/suspected-iranian-unc1549-targets-israel-middle-east)[[90]](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)

|  [S0644](https://attack.mitre.org/software/S0644)   |  [ObliqueRAT](https://attack.mitre.org/software/S0644)   |

[ObliqueRAT](https://attack.mitre.org/software/S0644) has gained execution on targeted systems through luring users to click on links to malicious URLs.[[91]](https://blog.talosintelligence.com/2021/02/obliquerat-new-campaign.html)[[92]](https://blog.talosintelligence.com/2021/05/transparent-tribe-infra-and-targeting.html)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) has delivered malicious links to achieve execution on the target system.[[93]](https://researchcenter.paloaltonetworks.com/2018/02/unit42-oopsie-oilrig-uses-threedollars-deliver-new-trojan/)[[94]](https://researchcenter.paloaltonetworks.com/2018/07/unit42-oilrig-targets-technology-service-provider-government-agency-quadagent/)[[95]](https://www.crowdstrike.com/blog/meet-crowdstrikes-adversary-of-the-month-for-november-helix-kitten/)[[96]](http://www.clearskysec.com/oilrig/)

|  [C0060](https://attack.mitre.org/campaigns/C0060)   |  [Operation AkaiRyū](https://attack.mitre.org/campaigns/C0060)   |

During [Operation AkaiRyū](https://attack.mitre.org/campaigns/C0060), [MirrorFace](https://attack.mitre.org/groups/G1054) lured users into executing malicious payloads with links to resources hosted on OneDrive.[[97]](https://www.trendmicro.com/en_us/research/24/k/return-of-anel-in-the-recent-earth-kasha-spearphishing-campaign.html)[[98]](https://www.welivesecurity.com/en/eset-research/operation-akairyu-mirrorface-invites-europe-expo-2025-revives-anel-backdoor/)

|  [C0022](https://attack.mitre.org/campaigns/C0022)   |  [Operation Dream Job](https://attack.mitre.org/campaigns/C0022)   |

During [Operation Dream Job](https://attack.mitre.org/campaigns/C0022), [Lazarus Group](https://attack.mitre.org/groups/G0032) lured users into executing a malicious link to disclose private account information or provide initial access.[[99]](https://www.clearskysec.com/wp-content/uploads/2020/08/Dream-Job-Campaign.pdf)[[100]](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_Operation_Interception.pdf)

|  [C0016](https://attack.mitre.org/campaigns/C0016)   |  [Operation Dust Storm](https://attack.mitre.org/campaigns/C0016)   |

During [Operation Dust Storm](https://attack.mitre.org/campaigns/C0016), the threat actors relied on a victim clicking on a malicious link sent via email.[[101]](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)

|  [C0005](https://attack.mitre.org/campaigns/C0005)   |  [Operation Spalax](https://attack.mitre.org/campaigns/C0005)   |

During [Operation Spalax](https://attack.mitre.org/campaigns/C0005), the threat actors relied on a victim to click on a malicious link distributed via phishing emails.[[102]](https://www.welivesecurity.com/2021/01/12/operation-spalax-targeted-malware-attacks-colombia/)

|  [S1017](https://attack.mitre.org/software/S1017)   |  [OutSteel](https://attack.mitre.org/software/S1017)   |

[OutSteel](https://attack.mitre.org/software/S1017) has relied on a user to click a malicious link within a spearphishing email.[[103]](https://unit42.paloaltonetworks.com/ukraine-targeted-outsteel-saintbot/)

|  [G0040](https://attack.mitre.org/groups/G0040)   |  [Patchwork](https://attack.mitre.org/groups/G0040)   |

[Patchwork](https://attack.mitre.org/groups/G0040) has used spearphishing with links to try to get users to click, download and open malicious files.[[104]](http://www.symantec.com/connect/blogs/patchwork-cyberespionage-group-expands-targets-governments-wide-range-industries)[[105]](https://documents.trendmicro.com/assets/tech-brief-untangling-the-patchwork-cyberespionage-group.pdf)[[106]](https://www.volexity.com/blog/2018/06/07/patchwork-apt-group-targets-us-think-tanks/)[[19]](https://unit42.paloaltonetworks.com/updated-backconfig-malware-targeting-government-and-military-organizations/)

|  [S0435](https://attack.mitre.org/software/S0435)   |  [PLEAD](https://attack.mitre.org/software/S0435)   |

[PLEAD](https://attack.mitre.org/software/S0435) has been executed via malicious links in e-mails.[[23]](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)

|  [S0453](https://attack.mitre.org/software/S0453)   |  [Pony](https://attack.mitre.org/software/S0453)   |

[Pony](https://attack.mitre.org/software/S0453) has attempted to lure targets into clicking links in spoofed emails from legitimate banks.[[107]](https://blog.malwarebytes.com/threat-analysis/2015/11/no-money-but-pony-from-a-mail-to-a-trojan-horse/)

|  [S0650](https://attack.mitre.org/software/S0650)   |  [QakBot](https://attack.mitre.org/software/S0650)   |

[QakBot](https://attack.mitre.org/software/S0650) has gained execution through users opening malicious links.[[108]](https://www.trendmicro.com/vinfo/ph/security/news/cybercrime-and-digital-threats/qakbot-resurges-spreads-through-vbs-files)[[109]](https://www.kroll.com/en/insights/publications/cyber/qakbot-malware-exfiltrating-emails-thread-hijacking-attacks)[[110]](https://success.trendmicro.com/en-US/solution/KA-0011282)[[111]](https://cybersecurity.att.com/blogs/labs-research/the-rise-of-qakbot)[[112]](https://securelist.com/qakbot-technical-analysis/103931/)[[113]](https://web.archive.org/web/20220119114433/https://groupib.pathfactory.com/ransomware-reports/prolock_wp)[[114]](https://www.trendmicro.com/en_us/research/22/j/black-basta-infiltrates-networks-via-qakbot-brute-ratel-and-coba.html)

|  [S1242](https://attack.mitre.org/software/S1242)   |  [Qilin](https://attack.mitre.org/software/S1242)   |

[Qilin](https://attack.mitre.org/software/S1242) has been executed by luring victims into clicking links in spearphishing emails.[[115]](https://www.sentinelone.com/anthology/agenda-qilin/)[[116]](https://news.sophos.com/en-us/2025/04/01/sophos-mdr-tracks-ongoing-campaign-by-qilin-affiliates-targeting-screenconnect/)

|  [G1039](https://attack.mitre.org/groups/G1039)   |  [RedCurl](https://attack.mitre.org/groups/G1039)   |

[RedCurl](https://attack.mitre.org/groups/G1039) has used malicious links to infect the victim machines.[[117]](https://www.group-ib.com/resources/research-hub/red-curl/)[[118]](https://www.group-ib.com/resources/research-hub/red-curl-2/)

|  [C0047](https://attack.mitre.org/campaigns/C0047)   |  [RedDelta Modified PlugX Infection Chain Operations](https://attack.mitre.org/campaigns/C0047)   |

[Mustang Panda](https://attack.mitre.org/groups/G0129) distributed hyperlinks that would result in an MSC file running a PowerShell command to download and install a remotely-hosted MSI file during [RedDelta Modified PlugX Infection Chain Operations](https://attack.mitre.org/campaigns/C0047).[[119]](https://go.recordedfuture.com/hubfs/reports/cta-cn-2025-0109.pdf)

|  [S9026](https://attack.mitre.org/software/S9026)   |  [ROAMINGHOUSE](https://attack.mitre.org/software/S9026)   |

[ROAMINGHOUSE](https://attack.mitre.org/software/S9026) has been executed through luring victims into clicking links to download malicious ZIP files.[[120]](https://www.trendmicro.com/en_us/research/25/d/earth-kasha-updates-ttps.html)

|  [G1031](https://attack.mitre.org/groups/G1031)   |  [Saint Bear](https://attack.mitre.org/groups/G1031)   |

[Saint Bear](https://attack.mitre.org/groups/G1031) has, in addition to email-based phishing attachments, used malicious websites masquerading as legitimate entities to host links to malicious files for user execution.[[103]](https://unit42.paloaltonetworks.com/ukraine-targeted-outsteel-saintbot/)[[121]](https://www.microsoft.com/en-us/security/blog/2023/06/14/cadet-blizzard-emerges-as-a-novel-and-distinct-russian-threat-actor/)

|  [S1018](https://attack.mitre.org/software/S1018)   |  [Saint Bot](https://attack.mitre.org/software/S1018)   |

[Saint Bot](https://attack.mitre.org/software/S1018) has relied on users to click on a malicious link delivered via a spearphishing.[[103]](https://unit42.paloaltonetworks.com/ukraine-targeted-outsteel-saintbot/)

|  [G0034](https://attack.mitre.org/groups/G0034)   |  [Sandworm Team](https://attack.mitre.org/groups/G0034)   |

[Sandworm Team](https://attack.mitre.org/groups/G0034) has tricked unwitting recipients into clicking on malicious hyperlinks within emails crafted to resemble trustworthy senders.[[122]](https://www.justice.gov/opa/press-release/file/1328521/download)

|  [G0121](https://attack.mitre.org/groups/G0121)   |  [Sidewinder](https://attack.mitre.org/groups/G0121)   |

[Sidewinder](https://attack.mitre.org/groups/G0121) has lured targets to click on malicious links to gain execution in the target environment.[[123]](https://cdn-cybersecurity.att.com/docs/global-perspective-of-the-sidewinder-apt.pdf)[[124]](https://www.rewterz.com/threats/sidewinder-apt-group-campaign-analysis)[[125]](https://www.rewterz.com/articles/analysis-on-sidewinder-apt-group-covid-19)[[126]](https://cybleinc.com/2020/09/26/sidewinder-apt-targets-with-futuristic-tactics-and-techniques/)

|  [S0649](https://attack.mitre.org/software/S0649)   |  [SMOKEDHAM](https://attack.mitre.org/software/S0649)   |

[SMOKEDHAM](https://attack.mitre.org/software/S0649) has relied upon users clicking on a malicious link delivered through phishing.[[127]](https://www.fireeye.com/blog/threat-research/2021/05/shining-a-light-on-darkside-ransomware-operations.html)

|  [S1086](https://attack.mitre.org/software/S1086)   |  [Snip3](https://attack.mitre.org/software/S1086)   |

[Snip3](https://attack.mitre.org/software/S1086) has been executed through luring victims into clicking malicious links.[[128]](https://telefonicatech.com/blog/snip3-investigacion-malware)

|  [S1124](https://attack.mitre.org/software/S1124)   |  [SocGholish](https://attack.mitre.org/software/S1124)   |

[SocGholish](https://attack.mitre.org/software/S1124) has lured victims into interacting with malicious links on compromised websites for execution.[[84]](https://www.proofpoint.com/us/blog/threat-insight/part-1-socgholish-very-real-threat-very-fake-update)

|  [S0646](https://attack.mitre.org/software/S0646)   |  [SpicyOmelette](https://attack.mitre.org/software/S0646)   |

[SpicyOmelette](https://attack.mitre.org/software/S0646) has been executed through malicious links within spearphishing emails.[[30]](https://www.secureworks.com/blog/cybercriminals-increasingly-trying-to-ensnare-the-big-financial-fish)

|  [S1030](https://attack.mitre.org/software/S1030)   |  [Squirrelwaffle](https://attack.mitre.org/software/S1030)   |

[Squirrelwaffle](https://attack.mitre.org/software/S1030) has relied on victims to click on a malicious link send via phishing campaigns.[[129]](https://www.zscaler.com/blogs/security-research/squirrelwaffle-new-loader-delivering-cobalt-strike)

|  [G1018](https://attack.mitre.org/groups/G1018)   |  [TA2541](https://attack.mitre.org/groups/G1018)   |

[TA2541](https://attack.mitre.org/groups/G1018) has used malicious links to cloud and web services to gain execution on victim machines.[[130]](https://www.proofpoint.com/us/blog/threat-insight/charting-ta2541s-flight)[[85]](https://www.mandiant.com/resources/blog/dissecting-netwire-phishing-campaigns-usage-process-hollowing)

|  [G0092](https://attack.mitre.org/groups/G0092)   |  [TA505](https://attack.mitre.org/groups/G0092)   |

[TA505](https://attack.mitre.org/groups/G0092) has used lures to get users to click links in emails and attachments. For example, [TA505](https://attack.mitre.org/groups/G0092) makes their malware look like legitimate Microsoft Word documents, .pdf and/or .lnk files. [[131]](https://www.proofpoint.com/us/threat-insight/post/threat-actor-profile-ta505-dridex-globeimposter)[[132]](https://www.proofpoint.com/us/threat-insight/post/ta505-shifts-times)[[133]](https://www.proofpoint.com/us/threat-insight/post/servhelper-and-flawedgrace-new-malware-introduced-ta505)[[134]](https://www.cybereason.com/blog/threat-actor-ta505-targets-financial-enterprises-using-lolbins-and-a-new-backdoor-malware)[[135]](https://www.proofpoint.com/us/threat-insight/post/ta505-abusing-settingcontent-ms-within-pdf-files-distribute-flawedammyy-rat)[[136]](https://www.proofpoint.com/us/threat-insight/post/leaked-ammyy-admin-source-code-turned-malware)[[137]](https://blog.trendmicro.com/trendlabs-security-intelligence/shifting-tactics-breaking-down-ta505-groups-use-of-html-rats-and-other-techniques-in-latest-campaigns/)[[138]](https://www.proofpoint.com/us/threat-insight/post/ta505-distributes-new-sdbbot-remote-access-trojan-get2-downloader)

|  [G1037](https://attack.mitre.org/groups/G1037)   |  [TA577](https://attack.mitre.org/groups/G1037)   |

[TA577](https://attack.mitre.org/groups/G1037) has lured users into executing malicious JavaScript files by sending malicious links via email.[[60]](https://www.proofpoint.com/us/blog/threat-insight/latrodectus-spider-bytes-ice)

|  [G1038](https://attack.mitre.org/groups/G1038)   |  [TA578](https://attack.mitre.org/groups/G1038)   |

[TA578](https://attack.mitre.org/groups/G1038) has placed malicious links in contact forms on victim sites, often spoofing a copyright complaint, to redirect users to malicious file downloads.[[60]](https://www.proofpoint.com/us/blog/threat-insight/latrodectus-spider-bytes-ice)

|  [G0134](https://attack.mitre.org/groups/G0134)   |  [Transparent Tribe](https://attack.mitre.org/groups/G0134)   |

[Transparent Tribe](https://attack.mitre.org/groups/G0134) has directed users to open URLs hosting malicious content.[[91]](https://blog.talosintelligence.com/2021/02/obliquerat-new-campaign.html)[[92]](https://blog.talosintelligence.com/2021/05/transparent-tribe-infra-and-targeting.html)

|  [S0436](https://attack.mitre.org/software/S0436)   |  [TSCookie](https://attack.mitre.org/software/S0436)   |

[TSCookie](https://attack.mitre.org/software/S0436) has been executed via malicious links embedded in e-mails spoofing the Ministries of Education, Culture, Sports, Science and Technology of Japan.[[139]](https://blogs.jpcert.or.jp/en/2018/03/malware-tscooki-7aa0.html)

|  [G0010](https://attack.mitre.org/groups/G0010)   |  [Turla](https://attack.mitre.org/groups/G0010)   |

[Turla](https://attack.mitre.org/groups/G0010) has used spearphishing via a link to get users to download and run their malware.[[140]](https://www.welivesecurity.com/wp-content/uploads/2018/01/ESET_Turla_Mosquito.pdf)

|  [S9050](https://attack.mitre.org/software/S9050)   |  [Vidar](https://attack.mitre.org/software/S9050)   |

[Vidar](https://attack.mitre.org/software/S9050) has been distributed via a malicious link that redirects to a filesharing website.[[141]](https://www.intrinsec.com/wp-content/uploads/2026/04/TLP_CLEAR-20260424-New_Vidar.pdf)

|  [C0037](https://attack.mitre.org/campaigns/C0037)   |  [Water Curupira Pikabot Distribution](https://attack.mitre.org/campaigns/C0037)   |

[Water Curupira Pikabot Distribution](https://attack.mitre.org/campaigns/C0037) distributed a PDF attachment containing a malicious link to a [Pikabot](https://attack.mitre.org/software/S1145) installer.[[142]](https://www.trendmicro.com/en_us/research/24/a/a-look-into-pikabot-spam-wave-campaign.html)

|  [G0112](https://attack.mitre.org/groups/G0112)   |  [Windshift](https://attack.mitre.org/groups/G0112)   |

[Windshift](https://attack.mitre.org/groups/G0112) has used links embedded in e-mails to lure victims into executing malicious code.[[143]](https://www.scribd.com/document/661837258/WINDSHIFT-summit-archive-1554718868)

|  [G1035](https://attack.mitre.org/groups/G1035)   |  [Winter Vivern](https://attack.mitre.org/groups/G1035)   |

[Winter Vivern](https://attack.mitre.org/groups/G1035) has mimicked legitimate government-related domains to deliver malicious webpages containing links to documents or other content for user execution.[[144]](https://www.sentinelone.com/labs/winter-vivern-uncovering-a-wave-of-global-espionage/)[[145]](https://cert.gov.ua/article/3761104)

|  [G0090](https://attack.mitre.org/groups/G0090)   |  [WIRTE](https://attack.mitre.org/groups/G0090)   |

[WIRTE](https://attack.mitre.org/groups/G0090) has used links embedded in emails to lure users into downloading malicious files.[[146]](https://research.checkpoint.com/2024/hamas-affiliated-threat-actor-expands-to-disruptive-activity/)

|  [G0102](https://attack.mitre.org/groups/G0102)   |  [Wizard Spider](https://attack.mitre.org/groups/G0102)   |

[Wizard Spider](https://attack.mitre.org/groups/G0102) has lured victims into clicking a malicious link delivered through spearphishing.[[147]](https://us-cert.cisa.gov/ncas/alerts/aa20-302a)

|  [G0128](https://attack.mitre.org/groups/G0128)   |  [ZIRCONIUM](https://attack.mitre.org/groups/G0128)   |

[ZIRCONIUM](https://attack.mitre.org/groups/G0128) has used malicious links in e-mails to lure victims into downloading malware.[[148]](https://blog.google/threat-analysis-group/how-were-tackling-evolving-online-threats/)[[149]](https://www.zscaler.com/blogs/security-research/apt-31-leverages-covid-19-vaccine-theme-and-abuses-legitimate-online)

## Mitigations

| ID  | Mitigation  | Description

|  [M1031](https://attack.mitre.org/mitigations/M1031)   |  [Network Intrusion Prevention](https://attack.mitre.org/mitigations/M1031)   |

If a link is being visited by a user, network intrusion prevention systems and systems designed to scan and remove malicious downloads can be used to block activity.

|  [M1021](https://attack.mitre.org/mitigations/M1021)   |  [Restrict Web-Based Content](https://attack.mitre.org/mitigations/M1021)   |

If a link is being visited by a user, block unknown or unused files in transit by default that should not be downloaded or by policy from suspicious sites as a best practice to prevent some vectors, such as .scr, .exe, .pif, .cpl, etc. Some download scanning devices can open and analyze compressed and encrypted formats, such as zip and rar that may be used to conceal malicious files.

|  [M1017](https://attack.mitre.org/mitigations/M1017)   |  [User Training](https://attack.mitre.org/mitigations/M1017)   |

Use user training as a way to bring awareness to common phishing and spearphishing techniques and how to raise suspicion for potentially malicious events.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0066](https://attack.mitre.org/detectionstrategies/DET0066)   |  [User Execution – Malicious Link (click → suspicious egress → download/write → follow-on activity)](https://attack.mitre.org/detectionstrategies/DET0066)   |  [AN0178](https://attack.mitre.org/detectionstrategies/DET0066#AN0178)   |

Behavioral chain: (1) a user-facing app (browser/Office/email client) launches a URL or handles a link, then (2) the same process lineage makes an outbound connection to an untrusted domain/IP, (3) a file is downloaded or unpacked to a user-writable location shortly after the click. Optional enrichment: subsequent child execution by LOLBINs.

|  [AN0179](https://attack.mitre.org/detectionstrategies/DET0066#AN0179)   |

Behavioral chain: (1) browser/office/GUI mail client opens a URL, (2) outbound connection to untrusted domain, (3) a new file is saved in $HOME/Downloads, /tmp, or cache immediately after.

|  [AN0180](https://attack.mitre.org/detectionstrategies/DET0066#AN0180)   |

Behavioral chain: (1) Safari/Chrome/Firefox/Office handles a URL; unified logs show open/click or LSQuarantine assignment, (2) outbound connection to untrusted domain, (3) a new file appears in ~/Downloads or /private/var/folders/* with quarantine flag.

## References

-    [Cybersecurity and Infrastructure Security Agency. (2021, February 21). AppleJeus: Analysis of North Korea’s Cryptocurrency Malware. Retrieved March 1, 2021.](https://us-cert.cisa.gov/ncas/alerts/aa21-048a)
-    [Global Research & Analysis Team, Kaspersky. (2024, August 19). BlindEagle flying high in Latin America. Retrieved April 16, 2026.](https://securelist.com/blindeagle-apt/113414/)
-    [Brady, S . (2018, October 3). Indictment - United States vs Aleksei Sergeyevich Morenets, et al.. Retrieved October 1, 2020.](https://www.justice.gov/opa/page/file/1098481/download)
-    [Secureworks CTU. (2017, March 30). IRON TWILIGHT Supports Active Measures. Retrieved February 28, 2022.](https://www.secureworks.com/research/iron-twilight-supports-active-measures)
-    [Microsoft Threat Intelligence Center (MSTIC). (2021, May 27). New sophisticated email-based attack from NOBELIUM. Retrieved May 28, 2021.](https://www.microsoft.com/security/blog/2021/05/27/new-sophisticated-email-based-attack-from-nobelium/)
-    [Secureworks CTU. (2021, May 28). USAID-Themed Phishing Campaign Leverages U.S. Elections Lure. Retrieved February 24, 2022.](https://www.secureworks.com/blog/usaid-themed-phishing-campaign-leverages-us-elections-lure)
-    [Eng, E., Caselden, D.. (2015, June 23). Operation Clandestine Wolf – Adobe Flash Zero-Day in APT3 Phishing Campaign. Retrieved January 14, 2016.](https://www.fireeye.com/blog/threat-research/2015/06/operation-clandestine-wolf-adobe-flash-zero-day.html)
-    [Dahan, A. (2017). Operation Cobalt Kitty. Retrieved December 27, 2018.](https://cdn2.hubspot.net/hubfs/3354902/Cybereason%20Labs%20Analysis%20Operation%20Cobalt%20Kitty.pdf)
-    [Adair, S. and Lancaster, T. (2020, November 6). OceanLotus: Extending Cyber Espionage Operations Through Fake Websites. Retrieved November 20, 2020.](https://www.volexity.com/blog/2020/11/06/oceanlotus-extending-cyber-espionage-operations-through-fake-websites/)
-    [Amnesty International. (2021, February 24). Vietnamese activists targeted by notorious hacking group. Retrieved March 1, 2021.](https://www.amnestyusa.org/wp-content/uploads/2021/02/Click-and-Bait_Vietnamese-Human-Rights-Defenders-Targeted-with-Spyware-Attacks.pdf)
-    [O'Leary, J., et al. (2017, September 20). Insights into Iranian Cyber Espionage: APT33 Targets Aerospace and Energy Sectors and has Ties to Destructive Malware. Retrieved February 15, 2018.](https://www.fireeye.com/blog/threat-research/2017/09/apt33-insights-into-iranian-cyber-espionage.html)
-    [Security Response attack Investigation Team. (2019, March 27). Elfin: Relentless Espionage Group Targets Multiple Organizations in Saudi Arabia and U.S.. Retrieved April 10, 2019.](https://www.symantec.com/blogs/threat-intelligence/elfin-apt33-espionage)
-    [SEONGSU PARK. (2022, December 27). BlueNoroff introduces new methods bypassing MoTW. Retrieved February 6, 2024.](https://securelist.com/bluenoroff-methods-bypass-motw/108383/)
-    [Microsoft Threat Intelligence. (2024, November 22). Microsoft shares latest intelligence on North Korean and Chinese threat actors at CYBERWARCON. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2024/11/22/microsoft-shares-latest-intelligence-on-north-korean-and-chinese-threat-actors-at-cyberwarcon/)
-    [Microsoft Threat Intelligence. (2026, June 17). From package to postinstall payload: Inside the Mastra npm supply chain compromise by Sapphire Sleet. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/06/17/postinstall-payload-inside-mastra-npm-supply-chain-compromise/)
-    [Microsoft Threat Intelligence. (2026, April 1). Mitigating the Axios npm supply chain compromise. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/04/01/mitigating-the-axios-npm-supply-chain-compromise/)
-    [Hawley et al. (2019, January 29). APT39: An Iranian Cyber Espionage Group Focused on Personal Information. Retrieved February 19, 2019.](https://www.fireeye.com/blog/threat-research/2019/01/apt39-iranian-cyber-espionage-group-focused-on-personal-information.html)
-    [FBI. (2020, September 17). Indicators of Compromise Associated with Rana Intelligence Computing, also known as Advanced Persistent Threat 39, Chafer, Cadelspy, Remexi, and ITG07. Retrieved December 10, 2020.](https://www.iranwatch.org/sites/default/files/public-intelligence-alert.pdf)
-    [Hinchliffe, A. and Falcone, R. (2020, May 11). Updated BackConfig Malware Targeting Government and Military Organizations in South Asia. Retrieved June 17, 2020.](https://unit42.paloaltonetworks.com/updated-backconfig-malware-targeting-government-and-military-organizations/)
-    [Cybereason Nocturnus. (2020, July 16). A BAZAR OF TRICKS: FOLLOWING TEAM9’S DEVELOPMENT CYCLES. Retrieved November 18, 2020.](https://www.cybereason.com/blog/a-bazar-of-tricks-following-team9s-development-cycles)
-    [Sadique, M. and Singh, A. (2020, September 29). Spear Phishing Campaign Delivers Buer and Bazar Malware. Retrieved November 19, 2020.](https://www.zscaler.com/blogs/research/spear-phishing-campaign-delivers-buer-and-bazar-malware)
-    [Podlosky, A., Hanel, A. et al. (2020, October 16). WIZARD SPIDER Update: Resilient, Reactive and Resolute. Retrieved June 15, 2021.](https://www.crowdstrike.com/blog/wizard-spider-adversary-update/)
-    [Bermejo, L., et al. (2017, June 22). Following the Trail of BlackTech’s Cyber Espionage Campaigns. Retrieved May 5, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)
-    [Merriman, K. and Trouerbach, P. (2022, April 28). This isn't Optimus Prime's Bumblebee but it's Still Transforming. Retrieved August 22, 2022.](https://www.proofpoint.com/us/blog/threat-insight/bumblebee-is-still-transforming)
-    [Cybereason. (2022, August 17). Bumblebee Loader – The High Road to Enterprise Domain Control. Retrieved August 29, 2022.](https://www.cybereason.com/blog/threat-analysis-report-bumblebee-loader-the-high-road-to-enterprise-domain-control)
-    [N. Baisini. (2022, July 13). Transparent Tribe begins targeting education sector in latest campaign. Retrieved September 22, 2022.](https://blog.talosintelligence.com/2022/07/transparent-tribe-targets-education.html)
-    [Dunwoody, M., et al. (2018, November 19). Not So Cozy: An Uncomfortable Examination of a Suspected APT29 Phishing Campaign. Retrieved November 27, 2018.](https://www.fireeye.com/blog/threat-research/2018/11/not-so-cozy-an-uncomfortable-examination-of-a-suspected-apt29-phishing-campaign.html)
-    [Svajcer, V. (2018, July 31). Multiple Cobalt Personality Disorder. Retrieved September 5, 2018.](https://blog.talosintelligence.com/2018/07/multiple-cobalt-personality-disorder.html)
-    [Unit 42. (2018, October 25). New Techniques to Uncover and Attribute Financial actors Commodity Builders and Infrastructure Revealed. Retrieved December 11, 2018.](https://researchcenter.paloaltonetworks.com/2018/10/unit42-new-techniques-uncover-attribute-cobalt-gang-commodity-builders-infrastructure-revealed/)
-    [CTU. (2018, September 27). Cybercriminals Increasingly Trying to Ensnare the Big Financial Fish. Retrieved September 20, 2021.](https://www.secureworks.com/blog/cybercriminals-increasingly-trying-to-ensnare-the-big-financial-fish)
-    [Lunghi, D. (2021, August 17). Confucius Uses Pegasus Spyware-related Lures to Target Pakistani Military. Retrieved December 26, 2021.](https://www.trendmicro.com/en_us/research/21/h/confucius-uses-pegasus-spyware-related-lures-to-target-pakistani.html)
-    [Insikt Group. (2025, February 13). Inside the Scam: North Korea’s IT Worker Threat. Retrieved October 17, 2025.](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)
-    [Ahn Ho, Facundo Muñoz, & Marc-Etienne M.Léveillé. (2024, March 7). Evasive Panda leverages Monlam Festival to target Tibetans. Retrieved July 25, 2024.](https://www.welivesecurity.com/en/eset-research/evasive-panda-leverages-monlam-festival-target-tibetans/)
-    [Chen, J., et al. (2022). Delving Deep: An Analysis of Earth Lusca’s Operations. Retrieved July 1, 2022.](https://www.trendmicro.com/content/dam/trendmicro/global/en/research/22/a/earth-lusca-employs-sophisticated-infrastructure-varied-tools-and-techniques/technical-brief-delving-deep-an-analysis-of-earth-lusca-operations.pdf)
-    [O'Gorman, G., and McDonald, G.. (2012, September 6). The Elderwood Project. Retrieved November 17, 2024.](https://web.archive.org/web/20190717233006/http:/www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/the-elderwood-project.pdf)
-    [Clayton, M.. (2012, September 14). Stealing US business secrets: Experts ID two huge cyber 'gangs' in China. Retrieved February 15, 2018.](https://www.csmonitor.com/USA/2012/0914/Stealing-US-business-secrets-Experts-ID-two-huge-cyber-gangs-in-China)
-    [Salvio, J.. (2014, June 27). New Banking Malware Uses Network Sniffing for Data Theft. Retrieved March 25, 2019.](https://blog.trendmicro.com/trendlabs-security-intelligence/new-banking-malware-uses-network-sniffing-for-data-theft/)
-    [Lee, S.. (2019, April 24). Emotet Using WMI to Launch PowerShell Encoded Code. Retrieved May 24, 2019.](https://www.carbonblack.com/2019/04/24/cb-tau-threat-intelligence-notification-emotet-utilizing-wmi-to-launch-powershell-encoded-code/)
-    [Porolli, M. (2020, July 9). More evil: A deep look at Evilnum and its toolset. Retrieved January 22, 2021.](https://www.welivesecurity.com/2020/07/09/more-evil-deep-look-evilnum-toolset/)
-    [Stolyarov, V. (2022, March 17). Exposing initial access broker with ties to Conti. Retrieved August 18, 2022.](https://blog.google/threat-analysis-group/exposing-initial-access-broker-ties-conti/)
-    [Vengerik, B. et al.. (2014, December 5). Hacking the Street? FIN4 Likely Playing the Market. Retrieved December 17, 2018.](https://web.archive.org/web/20220313062515/https://www.mandiant.com/sites/default/files/2021-09/rpt-fin4.pdf)
-    [Vengerik, B. & Dennesen, K.. (2014, December 5). Hacking the Street? FIN4 Likely Playing the Market. Retrieved January 15, 2019.](https://www2.fireeye.com/WBNR-14Q4NAMFIN4.html)
-    [Loui, E. and Reynolds, J. (2021, August 30). CARBON SPIDER Embraces Big Game Hunting, Part 1. Retrieved September 20, 2021.](https://www.crowdstrike.com/blog/carbon-spider-embraces-big-game-hunting-part-1/)
-    [Bohannon, D. & Carr N. (2017, June 30). Obfuscation in the Wild: Targeted Attackers Lead the Way in Evasion Techniques. Retrieved February 12, 2018.](https://web.archive.org/web/20170923102302/https://www.fireeye.com/blog/threat-research/2017/06/obfuscation-in-the-wild.html)
-    [Kizhakkinan, D., et al. (2016, May 11). Threat Actor Leverages Windows Zero-day Exploit in Payment Card Data Attacks. Retrieved February 12, 2018.](https://www.fireeye.com/blog/threat-research/2016/05/windows-zero-day-payment-cards.html)
-    [Elovitz, S. & Ahl, I. (2016, August 18). Know Your Enemy: New Financially-Motivated & Spear-Phishing Group. Retrieved February 26, 2018.](https://www2.fireeye.com/WBNR-Know-Your-Enemy-UNC622-Spear-Phishing.html)
-    [Unit 42. (2022, December 20). Russia’s Trident Ursa (aka Gamaredon APT) Cyber Conflict Operations Unwavering Since Invasion of Ukraine. Retrieved September 12, 2024.](https://unit42.paloaltonetworks.com/trident-ursa/)
-    [Rusnák, Z. (2024, September 26). Cyberespionage the Gamaredon way: Analysis of toolset used to spy on Ukraine in 2022 and 2023. Retrieved October 30, 2024.](https://web-assets.esetstatic.com/wls/en/papers/white-papers/cyberespionage-gamaredon-way.pdf)
-    [Szappanos, G. & Brandt, A. (2021, March 1). “Gootloader” expands its payload delivery options. Retrieved September 30, 2022.](https://news.sophos.com/en-us/2021/03/01/gootloader-expands-its-payload-delivery-options/)
-    [Pirozzi, A. (2021, June 16). Gootloader: ‘Initial Access as a Service’ Platform Expands Its Search for High Value Targets. Retrieved May 28, 2024.](https://www.sentinelone.com/labs/gootloader-initial-access-as-a-service-platform-expands-its-search-for-high-value-targets/)
-    [Abramov, D. (2020, April 13). Grandoreiro Malware Now Targeting Banks in Spain. Retrieved November 12, 2020.](https://securityintelligence.com/posts/grandoreiro-malware-now-targeting-banks-in-spain/)
-    [ESET. (2020, April 28). Grandoreiro: How engorged can an EXE get?. Retrieved November 13, 2020.](https://www.welivesecurity.com/2020/04/28/grandoreiro-how-engorged-can-exe-get/)
-    [Duncan, B. (2020, April 3). GuLoader: Malspam Campaign Installing NetWire RAT. Retrieved January 7, 2021.](https://unit42.paloaltonetworks.com/guloader-installing-netwire-rat/)
-    [Tom Spring. (2017, January 11). Spammers Revive Hancitor Downloader Campaigns. Retrieved August 13, 2020.](https://threatpost.com/spammers-revive-hancitor-downloader-campaigns/123011/)
-    [GReAT. (2020, July 14). The Tetrade: Brazilian banking malware goes global. Retrieved November 9, 2020.](https://securelist.com/the-tetrade-brazilian-banking-malware/97779/)
-    [Artic Wolf Labs. (2026, April 24). Token Bingo: Don’t Let Your Code be the Winner. Retrieved July 30, 2026.](https://arcticwolf.com/resources/blog/token-bingo-dont-let-your-code-be-the-winner/)
-    [Tanner Flip. (2026, June 11). Hook, Line, and Token: Anatomy of the Kali365 / Octopi365 Phishing-as-a-Service Kit. Retrieved July 30, 2026.](https://www.huntress.com/blog/kali365-device-code-phishing-kit)
-    [KISA. (2021). Phishing Target Reconnaissance and Attack Resource Analysis Operation Muzabi. Retrieved March 8, 2024.](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)
-    [Jazi, H. (2021, February). LazyScripter: From Empire to double RAT. Retrieved November 17, 2024.](https://web.archive.org/web/20211003035156/https://www.malwarebytes.com/resources/files/2021/02/lazyscripter.pdf)
-    [Proofpoint Threat Research and Team Cymru S2 Threat Research. (2024, April 4). Latrodectus: This Spider Bytes Like Ice . Retrieved May 31, 2024.](https://www.proofpoint.com/us/blog/threat-insight/latrodectus-spider-bytes-ice)
-    [Abrams, L. (2024, April 30). New Latrodectus malware attacks use Microsoft, Cloudflare themes. Retrieved September 13, 2024.](https://www.bleepingcomputer.com/news/security/new-latrodectus-malware-attacks-use-microsoft-cloudflare-themes/)
-    [Axel F, Pierre T. (2017, October 16). Leviathan: Espionage actor spearphishes maritime and defense targets. Retrieved February 15, 2018.](https://www.proofpoint.com/us/threat-insight/post/leviathan-espionage-actor-spearphishes-maritime-and-defense-targets)
-    [CISA. (2021, July 19). (AA21-200A) Joint Cybersecurity Advisory – Tactics, Techniques, and Procedures of Indicted APT40 Actors Associated with China’s MSS Hainan State Security Department. Retrieved August 12, 2021.](https://us-cert.cisa.gov/ncas/alerts/aa21-200a)
-    [Lechtik, M, and etl. (2021, July 14). LuminousMoth APT: Sweeping attacks for the chosen few. Retrieved October 20, 2022.](https://securelist.com/apt-luminousmoth/103332/)
-    [The Cylance Threat Research Team. (2017, March 22). El Machete's Malware Attacks Cut Through LATAM. Retrieved September 13, 2019.](https://threatvector.cylance.com/en_us/home/el-machete-malware-attacks-cut-through-latam.html)
-    [Kaspersky Global Research and Analysis Team. (2014, August 20). El Machete. Retrieved September 13, 2019.](https://securelist.com/el-machete/66108/)
-    [ESET. (2019, July). MACHETE JUST GOT SHARPER Venezuelan government institutions under attack. Retrieved September 13, 2019.](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)
-    [ClearSky Research Team. (2020, August 1). The Kittens Are Back in Town 3 - Charming Kitten Campaign Evolved and Deploying Spear-Phishing link by WhatsApp. Retrieved April 21, 2021.](https://www.clearskysec.com/wp-content/uploads/2020/08/The-Kittens-are-Back-in-Town-3.pdf)
-    [Certfa Labs. (2021, January 8). Charming Kitten’s Christmas Gift. Retrieved May 3, 2021.](https://blog.certfa.com/posts/charming-kitten-christmas-gift/)
-    [Yonathan Klijnsma. (2016, May 17). Mofang: A politically motivated information stealing adversary. Retrieved May 12, 2020.](https://foxitsecurity.files.wordpress.com/2016/06/fox-it_mofang_threatreport_tlp-white.pdf)
-    [GReAT. (2019, April 10). Gaza Cybergang Group1, operation SneakyPastes. Retrieved May 13, 2020.](https://securelist.com/gaza-cybergang-group1-operation-sneakypastes/90068/)
-    [Falcone, R., et al. (2020, March 3). Molerats Delivers Spark Backdoor to Government and Telecommunications Organizations. Retrieved December 14, 2020.](https://unit42.paloaltonetworks.com/molerats-delivers-spark-backdoor/)
-    [Mele, G. et al. (2021, February 10). Probable Iranian Cyber Actors, Static Kitten, Conducting Cyberespionage Campaign Targeting UAE and Kuwait Government Agencies. Retrieved March 17, 2021.](https://www.anomali.com/blog/probable-iranian-cyber-actors-static-kitten-conducting-cyberespionage-campaign-targeting-uae-and-kuwait-government-agencies)
-    [Peretz, A. and Theck, E. (2021, March 5). Earth Vetala – MuddyWater Continues to Target Organizations in the Middle East. Retrieved March 18, 2021.](https://www.trendmicro.com/en_us/research/21/c/earth-vetala---muddywater-continues-to-target-organizations-in-t.html)
-    [Miller, J. et al. (2024, March 21). Security Brief: TA450 Uses Embedded Links in PDF Attachments in Latest Campaign. Retrieved March 27, 2024.](https://www.proofpoint.com/us/blog/threat-insight/security-brief-ta450-uses-embedded-links-pdf-attachments-latest-campaign)

-    [Golo Muhr, Joshua Chung. (2025, June 23). Hive0154 aka Mustang Panda shifts focus on Tibetan community to deploy Pubload backdoor. Retrieved August 4, 2025.](https://www.ibm.com/think/x-force/hive0154-mustang-panda-shifts-focus-tibetan-community-deploy-pubload-backdoor)
-    [Golo Muhr, Joshua Chung. (2025, May 15). Hive0154 targeting US, Philippines, Pakistan and Taiwan in suspected espionage campaign. Retrieved August 4, 2025.](https://www.ibm.com/think/x-force/hive0154-targeting-us-philippines-pakistan-taiwan)
-    [Meyers, A. (2018, June 15). Meet CrowdStrike’s Adversary of the Month for June: MUSTANG PANDA. Retrieved April 12, 2021.](https://www.crowdstrike.com/blog/meet-crowdstrikes-adversary-of-the-month-for-june-mustang-panda/)
-    [Nick Dai, Vickie Su, Sunny Lu. (2022, November 18). Earth Preta Spear-Phishing Governments Worldwide. Retrieved August 4, 2025.](https://www.trendmicro.com/en_us/research/22/k/earth-preta-spear-phishing-governments-worldwide.html)
-    [Raggi, M. et al. (2022, March 7). The Good, the Bad, and the Web Bug: TA416 Increases Operational Tempo Against European Governments as Conflict in Ukraine Escalates. Retrieved March 16, 2022.](https://www.proofpoint.com/us/blog/threat-insight/good-bad-and-web-bug-ta416-increases-operational-tempo-against-european)
-    [Roccia, T., Seret, T., Fokker, J. (2021, March 16). Technical Analysis of Operation Dianxun. Retrieved April 13, 2021.](https://www.mcafee.com/enterprise/en-us/assets/reports/rp-operation-dianxun.pdf)
-    [Patrick Whitsell. (2025, August 25). Deception in Depth: PRC-Nexus Espionage Campaign Hijacks Web Traffic to Target Diplomats. Retrieved September 9, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/prc-nexus-espionage-targets-diplomats)
-    [Microsoft. (2022, May 9). Ransomware as a service: Understanding the cybercrime gig economy and how to protect yourself. Retrieved March 10, 2023.](https://www.microsoft.com/en-us/security/blog/2022/05/09/ransomware-as-a-service-understanding-the-cybercrime-gig-economy-and-how-to-protect-yourself/)
-    [Andrew Northern. (2022, November 22). SocGholish, a very real threat from a very fake update. Retrieved February 13, 2024.](https://www.proofpoint.com/us/blog/threat-insight/part-1-socgholish-very-real-threat-very-fake-update)
-    [Maniath, S. and Kadam P. (2019, March 19). Dissecting a NETWIRE Phishing Campaign's Usage of Process Hollowing. Retrieved January 7, 2021.](https://www.mandiant.com/resources/blog/dissecting-netwire-phishing-campaigns-usage-process-hollowing)
-    [McAfee® Foundstone® Professional Services and McAfee Labs™. (2011, February 10). Global Energy Cyberattacks: “Night Dragon”. Retrieved February 19, 2018.](https://scadahacker.com/library/Documents/Cyber_Events/McAfee%20-%20Night%20Dragon%20-%20Global%20Energy%20Cyberattacks.pdf)
-    [Check Point. (2026, May 22). Fast and Furious – Nimbus Manticore Operations During the Iranian Conflict. Retrieved September 23, 2026.](https://research.checkpoint.com/2026/fast-and-furious-nimbus-manticore-operations-during-the-iranian-conflict/)
-    [Check Point. (2025, September 22). Nimbus Manticore Deploys New Malware Targeting Europe. Retrieved September 24, 2026.](https://research.checkpoint.com/2025/nimbus-manticore-deploys-new-malware-targeting-europe/)
-    [Rozmann, O., et al. (2024, February 27). When Cats Fly: Suspected Iranian Threat Actor UNC1549 Targets Israeli and Middle East Aerospace and Defense Sectors. Retrieved September 28, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/suspected-iranian-unc1549-targets-israel-middle-east)
-    [El-Banna, M., et al. (2025, November 17). Frontline Intelligence: Analysis of UNC1549 TTPs, Custom Tools, and Malware Targeting the Aerospace and Defense Ecosystem. Retrieved September 28, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)
-    [Malhotra, A. (2021, March 2). ObliqueRAT returns with new campaign using hijacked websites. Retrieved September 2, 2021.](https://blog.talosintelligence.com/2021/02/obliquerat-new-campaign.html)
-    [Malhotra, A. et al. (2021, May 13). Transparent Tribe APT expands its Windows malware arsenal. Retrieved September 2, 2021.](https://blog.talosintelligence.com/2021/05/transparent-tribe-infra-and-targeting.html)
-    [Lee, B., Falcone, R. (2018, February 23). OopsIE! OilRig Uses ThreeDollars to Deliver New Trojan. Retrieved July 16, 2018.](https://researchcenter.paloaltonetworks.com/2018/02/unit42-oopsie-oilrig-uses-threedollars-deliver-new-trojan/)
-    [Lee, B., Falcone, R. (2018, July 25). OilRig Targets Technology Service Provider and Government Agency with QUADAGENT. Retrieved August 9, 2018.](https://researchcenter.paloaltonetworks.com/2018/07/unit42-oilrig-targets-technology-service-provider-government-agency-quadagent/)
-    [Meyers, A. (2018, November 27). Meet CrowdStrike’s Adversary of the Month for November: HELIX KITTEN. Retrieved December 18, 2018.](https://www.crowdstrike.com/blog/meet-crowdstrikes-adversary-of-the-month-for-november-helix-kitten/)
-    [ClearSky Cybersecurity. (2017, January 5). Iranian Threat Agent OilRig Delivers Digitally Signed Malware, Impersonates University of Oxford. Retrieved May 3, 2017.](http://www.clearskysec.com/oilrig/)
-    [Hiroaki, H. (2024, November 26). Guess Who’s Back - The Return of ANEL in the Recent Earth Kasha Spear-phishing Campaign in 2024. Retrieved April 17, 2026.](https://www.trendmicro.com/en_us/research/24/k/return-of-anel-in-the-recent-earth-kasha-spearphishing-campaign.html)
-    [Dominik Breitenbacher. (2025, March 18). Operation AkaiRyū: MirrorFace invites Europe to Expo 2025 and revives ANEL backdoor. Retrieved May 22, 2025.](https://www.welivesecurity.com/en/eset-research/operation-akairyu-mirrorface-invites-europe-expo-2025-revives-anel-backdoor/)
-    [ClearSky Research Team. (2020, August 13). Operation 'Dream Job' Widespread North Korean Espionage Campaign. Retrieved December 20, 2021.](https://www.clearskysec.com/wp-content/uploads/2020/08/Dream-Job-Campaign.pdf)
-    [Breitenbacher, D and Osis, K. (2020, June 17). OPERATION IN(TER)CEPTION: Targeted Attacks Against European Aerospace and Military Companies. Retrieved December 20, 2021.](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_Operation_Interception.pdf)
-    [Gross, J. (2016, February 23). Operation Dust Storm. Retrieved December 22, 2021.](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)
-    [M. Porolli. (2021, January 21). Operation Spalax: Targeted malware attacks in Colombia. Retrieved September 16, 2022.](https://www.welivesecurity.com/2021/01/12/operation-spalax-targeted-malware-attacks-colombia/)
-    [Unit 42. (2022, February 25). Spear Phishing Attacks Target Organizations in Ukraine, Payloads Include the Document Stealer OutSteel and the Downloader SaintBot. Retrieved June 9, 2022.](https://unit42.paloaltonetworks.com/ukraine-targeted-outsteel-saintbot/)
-    [Hamada, J.. (2016, July 25). Patchwork cyberespionage group expands targets from governments to wide range of industries. Retrieved August 17, 2016.](http://www.symantec.com/connect/blogs/patchwork-cyberespionage-group-expands-targets-governments-wide-range-industries)
-    [Lunghi, D., et al. (2017, December). Untangling the Patchwork Cyberespionage Group. Retrieved July 10, 2018.](https://documents.trendmicro.com/assets/tech-brief-untangling-the-patchwork-cyberespionage-group.pdf)
-    [Meltzer, M, et al. (2018, June 07). Patchwork APT Group Targets US Think Tanks. Retrieved July 16, 2018.](https://www.volexity.com/blog/2018/06/07/patchwork-apt-group-targets-us-think-tanks/)
-    [hasherezade. (2016, April 11). No money, but Pony! From a mail to a trojan horse. Retrieved May 21, 2020.](https://blog.malwarebytes.com/threat-analysis/2015/11/no-money-but-pony-from-a-mail-to-a-trojan-horse/)
-    [Mendoza, E. et al. (2020, May 25). Qakbot Resurges, Spreads through VBS Files. Retrieved September 27, 2021.](https://www.trendmicro.com/vinfo/ph/security/news/cybercrime-and-digital-threats/qakbot-resurges-spreads-through-vbs-files)
-    [Sette, N. et al. (2020, June 4). Qakbot Malware Now Exfiltrating Emails for Sophisticated Thread Hijacking Attacks. Retrieved September 27, 2021.](https://www.kroll.com/en/insights/publications/cyber/qakbot-malware-exfiltrating-emails-thread-hijacking-attacks)
-    [Trend Micro. (2020, December 17). QAKBOT: A decade-old malware still with new tricks. Retrieved November 17, 2024.](https://success.trendmicro.com/en-US/solution/KA-0011282)
-    [Morrow, D. (2021, April 15). The rise of QakBot. Retrieved September 27, 2021.](https://cybersecurity.att.com/blogs/labs-research/the-rise-of-qakbot)
-    [Kuzmenko, A. et al. (2021, September 2). QakBot technical analysis. Retrieved September 27, 2021.](https://securelist.com/qakbot-technical-analysis/103931/)
-    [Group IB. (2020, September). LOCK LIKE A PRO. Retrieved November 17, 2024.](https://web.archive.org/web/20220119114433/https://groupib.pathfactory.com/ransomware-reports/prolock_wp)
-    [Kenefick, I. et al. (2022, October 12). Black Basta Ransomware Gang Infiltrates Networks via QAKBOT, Brute Ratel, and Cobalt Strike. Retrieved February 6, 2023.](https://www.trendmicro.com/en_us/research/22/j/black-basta-infiltrates-networks-via-qakbot-brute-ratel-and-coba.html)
-    [SentinelOne. (2022, November 30). Agenda (Qilin). Retrieved September 26, 2025.](https://www.sentinelone.com/anthology/agenda-qilin/)
-    [Bradshaw, A. et al. (2025, April 1). Qilin affiliates spear-phish MSP ScreenConnect admin, targeting customers downstream. Retrieved September 26, 2025.](https://news.sophos.com/en-us/2025/04/01/sophos-mdr-tracks-ongoing-campaign-by-qilin-affiliates-targeting-screenconnect/)
-    [Group-IB. (2020, August). RedCurl: The Pentest You Didn’t Know About. Retrieved August 9, 2024.](https://www.group-ib.com/resources/research-hub/red-curl/)
-    [Group-IB. (2021, November). RedCurl: The Awakening. Retrieved August 14, 2024.](https://www.group-ib.com/resources/research-hub/red-curl-2/)
-    [Insikt Group. (2025, January 9). Chinese State-Sponsored RedDelta Targeted Taiwan, Mongolia, and Southeast Asia with Adapted PlugX Infection Chain. Retrieved January 14, 2025.](https://go.recordedfuture.com/hubfs/reports/cta-cn-2025-0109.pdf)
-    [Hiroaki, H. (2025, April 30). Earth Kasha Updates TTPs in Latest Campaign Targeting Taiwan and Japan. Retrieved April 17, 2026.](https://www.trendmicro.com/en_us/research/25/d/earth-kasha-updates-ttps.html)
-    [Microsoft Threat Intelligence. (2023, June 14). Cadet Blizzard emerges as a novel and distinct Russian threat actor. Retrieved July 10, 2023.](https://www.microsoft.com/en-us/security/blog/2023/06/14/cadet-blizzard-emerges-as-a-novel-and-distinct-russian-threat-actor/)
-    [Scott W. Brady. (2020, October 15). United States vs. Yuriy Sergeyevich Andrienko et al.. Retrieved November 25, 2020.](https://www.justice.gov/opa/press-release/file/1328521/download)
-    [Hegel, T. (2021, January 13). A Global Perspective of the SideWinder APT. Retrieved January 27, 2021.](https://cdn-cybersecurity.att.com/docs/global-perspective-of-the-sidewinder-apt.pdf)
-    [Rewterz. (2020, April 20). Sidewinder APT Group Campaign Analysis. Retrieved January 29, 2021.](https://www.rewterz.com/threats/sidewinder-apt-group-campaign-analysis)
-    [Rewterz. (2020, June 22). Analysis on Sidewinder APT Group – COVID-19. Retrieved January 29, 2021.](https://www.rewterz.com/articles/analysis-on-sidewinder-apt-group-covid-19)
-    [Cyble. (2020, September 26). SideWinder APT Targets with futuristic Tactics and Techniques. Retrieved January 29, 2021.](https://cybleinc.com/2020/09/26/sidewinder-apt-targets-with-futuristic-tactics-and-techniques/)
-    [FireEye. (2021, May 11). Shining a Light on DARKSIDE Ransomware Operations. Retrieved September 22, 2021.](https://www.fireeye.com/blog/threat-research/2021/05/shining-a-light-on-darkside-ransomware-operations.html)
-    [Jornet, A. (2021, December 23). Snip3, an investigation into malware. Retrieved September 19, 2023.](https://telefonicatech.com/blog/snip3-investigacion-malware)
-    [Kumar, A., Stone-Gross, Brett. (2021, September 28). Squirrelwaffle: New Loader Delivering Cobalt Strike. Retrieved August 9, 2022.](https://www.zscaler.com/blogs/security-research/squirrelwaffle-new-loader-delivering-cobalt-strike)
-    [Larson, S. and Wise, J. (2022, February 15). Charting TA2541's Flight. Retrieved September 12, 2023.](https://www.proofpoint.com/us/blog/threat-insight/charting-ta2541s-flight)
-    [Proofpoint Staff. (2017, September 27). Threat Actor Profile: TA505, From Dridex to GlobeImposter. Retrieved May 28, 2019.](https://www.proofpoint.com/us/threat-insight/post/threat-actor-profile-ta505-dridex-globeimposter)
-    [Proofpoint Staff. (2018, June 8). TA505 shifts with the times. Retrieved May 28, 2019.](https://www.proofpoint.com/us/threat-insight/post/ta505-shifts-times)
-    [Schwarz, D. and Proofpoint Staff. (2019, January 9). ServHelper and FlawedGrace - New malware introduced by TA505. Retrieved May 28, 2019.](https://www.proofpoint.com/us/threat-insight/post/servhelper-and-flawedgrace-new-malware-introduced-ta505)
-    [Salem, E. (2019, April 25). Threat Actor TA505 Targets Financial Enterprises Using LOLBins and a New Backdoor Malware. Retrieved May 28, 2019.](https://www.cybereason.com/blog/threat-actor-ta505-targets-financial-enterprises-using-lolbins-and-a-new-backdoor-malware)
-    [Proofpoint Staff. (2018, July 19). TA505 Abusing SettingContent-ms within PDF files to Distribute FlawedAmmyy RAT. Retrieved April 19, 2019.](https://www.proofpoint.com/us/threat-insight/post/ta505-abusing-settingcontent-ms-within-pdf-files-distribute-flawedammyy-rat)
-    [Proofpoint Staff. (2018, March 7). Leaked Ammyy Admin Source Code Turned into Malware. Retrieved May 28, 2019.](https://www.proofpoint.com/us/threat-insight/post/leaked-ammyy-admin-source-code-turned-malware)
-    [Hiroaki, H. and Lu, L. (2019, June 12). Shifting Tactics: Breaking Down TA505 Group’s Use of HTML, RATs and Other Techniques in Latest Campaigns. Retrieved May 29, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/shifting-tactics-breaking-down-ta505-groups-use-of-html-rats-and-other-techniques-in-latest-campaigns/)
-    [Schwarz, D. et al. (2019, October 16). TA505 Distributes New SDBbot Remote Access Trojan with Get2 Downloader. Retrieved May 29, 2020.](https://www.proofpoint.com/us/threat-insight/post/ta505-distributes-new-sdbbot-remote-access-trojan-get2-downloader)
-    [Tomonaga, S. (2018, March 6). Malware “TSCookie”. Retrieved May 6, 2020.](https://blogs.jpcert.or.jp/en/2018/03/malware-tscooki-7aa0.html)
-    [ESET, et al. (2018, January). Diplomats in Eastern Europe bitten by a Turla mosquito. Retrieved July 3, 2018.](https://www.welivesecurity.com/wp-content/uploads/2018/01/ESET_Turla_Mosquito.pdf)
-    [Intrinsec. (2026, April). "Chaos is a ladder": Vidar's recent rise to the top. Retrieved April 27, 2026.](https://www.intrinsec.com/wp-content/uploads/2026/04/TLP_CLEAR-20260424-New_Vidar.pdf)
-    [Shinji Robert Arasawa, Joshua Aquino, Charles Steven Derion, Juhn Emmanuel Atanque, Francisrey Joshua Castillo, John Carlo Marquez, Henry Salcedo, John Rainier Navato, Arianne Dela Cruz, Raymart Yambot & Ian Kenefick. (2024, January 9). Black Basta-Affiliated Water Curupira’s Pikabot Spam Campaign. Retrieved July 17, 2024.](https://www.trendmicro.com/en_us/research/24/a/a-look-into-pikabot-spam-wave-campaign.html)
-    [Karim, T. (2018, August). TRAILS OF WINDSHIFT. Retrieved November 17, 2024.](https://www.scribd.com/document/661837258/WINDSHIFT-summit-archive-1554718868)
-    [Tom Hegel. (2023, March 16). Winter Vivern | Uncovering a Wave of Global Espionage. Retrieved July 29, 2024.](https://www.sentinelone.com/labs/winter-vivern-uncovering-a-wave-of-global-espionage/)
-    [CERT-UA. (2023, February 1). UAC-0114 aka Winter Vivern to target Ukrainian and Polish GOV entities (CERT-UA#5909). Retrieved July 29, 2024.](https://cert.gov.ua/article/3761104)
-    [Check Point. (2024, November 12). Hamas-affiliated Threat Actor WIRTE Continues its Middle East Operations and Moves to Disruptive Activity. Retrieved April 20, 2026.](https://research.checkpoint.com/2024/hamas-affiliated-threat-actor-expands-to-disruptive-activity/)
-    [DHS/CISA. (2020, October 28). Ransomware Activity Targeting the Healthcare and Public Health Sector. Retrieved October 28, 2020.](https://us-cert.cisa.gov/ncas/alerts/aa20-302a)
-    [Huntley, S. (2020, October 16). How We're Tackling Evolving Online Threats. Retrieved March 24, 2021.](https://blog.google/threat-analysis-group/how-were-tackling-evolving-online-threats/)
-    [Singh, S. and Antil, S. (2020, October 27). APT-31 Leverages COVID-19 Vaccine Theme and Abuses Legitimate Online Services. Retrieved March 24, 2021.](https://www.zscaler.com/blogs/security-research/apt-31-leverages-covid-19-vaccine-theme-and-abuses-legitimate-online)

×
