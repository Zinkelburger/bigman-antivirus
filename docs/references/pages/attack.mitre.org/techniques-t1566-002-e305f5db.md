---
url: "https://attack.mitre.org/techniques/T1566/002/"
title: "Phishing: Spearphishing Link, Sub-technique T1566.002 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:59:32Z"
via: "live"
sha256: "a5febacedeea00571ff0764285712a6d4b694d9d94c18d25d522b011bf18aaa0"
wayback: "https://web.archive.org/web/20261023090728/https://attack.mitre.org/techniques/T1566/002/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# Phishing: Spearphishing Link, Sub-technique T1566.002 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1566/002/>
> Snapshot: <https://web.archive.org/web/20261023090728/https://attack.mitre.org/techniques/T1566/002/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Phishing](https://attack.mitre.org/techniques/T1566)
- Spearphishing Link

#  Phishing: Spearphishing Link

##### Other sub-techniques of Phishing (4)

| ID  | Name

|  [T1566.001](https://attack.mitre.org/techniques/T1566/001/)   |  [Spearphishing Attachment](https://attack.mitre.org/techniques/T1566/001/)

|  T1566.002   |  Spearphishing Link

|  [T1566.003](https://attack.mitre.org/techniques/T1566/003/)   |  [Spearphishing via Service](https://attack.mitre.org/techniques/T1566/003/)

|  [T1566.004](https://attack.mitre.org/techniques/T1566/004/)   |  [Spearphishing Voice](https://attack.mitre.org/techniques/T1566/004/)

Adversaries may send spearphishing emails with a malicious link in an attempt to gain access to victim systems. Spearphishing with a link is a specific variant of spearphishing. It is different from other forms of spearphishing in that it employs the use of links to download malware contained in email, instead of attaching malicious files to the email itself, to avoid defenses that may inspect email attachments. Spearphishing may also involve social engineering techniques, such as posing as a trusted source.

All forms of spearphishing are electronically delivered social engineering targeted at a specific individual, company, or industry. In this case, the malicious emails contain links. Generally, the links will be accompanied by social engineering text and require the user to actively click or copy and paste a URL into a browser, leveraging [User Execution](https://attack.mitre.org/techniques/T1204). The visited website may compromise the web browser using an exploit, or the user will be prompted to download applications, documents, zip files, or even executables depending on the pretext for the email in the first place.

Adversaries may also include links that are intended to interact directly with an email reader, including embedded images intended to exploit the end system directly. Additionally, adversaries may use seemingly benign links that abuse special characters to mimic legitimate websites (known as an "IDN homograph attack").[[1]](https://us-cert.cisa.gov/ncas/tips/ST05-016) URLs may also be obfuscated by taking advantage of quirks in the URL schema, such as the acceptance of integer- or hexadecimal-based hostname formats and the automatic discarding of text before an "@" symbol: for example, hxxp://google.com@1157586937.[[2]](https://www.mandiant.com/resources/blog/url-obfuscation-schema-abuse)

Adversaries may also utilize links to perform consent phishing/spearphishing campaigns to [Steal Application Access Token](https://attack.mitre.org/techniques/T1528)s that grant immediate access to the victim environment. For example, a user may be lured into granting adversaries permissions/access via a malicious OAuth 2.0 request URL that when accepted by the user provide permissions/access for malicious applications.[[3]](https://blog.trendmicro.com/trendlabs-security-intelligence/pawn-storm-abuses-open-authentication-advanced-social-engineering-attacks)[[4]](https://www.microsoft.com/security/blog/2021/07/14/microsoft-delivers-comprehensive-solution-to-battle-rise-in-consent-phishing-emails/) These stolen access tokens allow the adversary to perform various actions on behalf of the user via API calls.[[4]](https://www.microsoft.com/security/blog/2021/07/14/microsoft-delivers-comprehensive-solution-to-battle-rise-in-consent-phishing-emails/)

Similarly, malicious links may also target device-based authorization, such as OAuth 2.0 device authorization grant flow which is typically used to authenticate devices without UIs/browsers. Known as "device code phishing," an adversary may send a link that directs the victim to a malicious authorization page where the user is tricked into entering a code/credentials that produces a device token.[[5]](https://www.secureworks.com/blog/oauths-device-code-flow-abused-in-phishing-attacks)[[6]](https://www.netskope.com/blog/new-phishing-attacks-exploiting-oauth-authorization-flows-part-1)[[7]](https://www.optiv.com/insights/source-zero/blog/microsoft-365-oauth-device-code-flow-and-phishing)

ID: T1566.002

Sub-technique of:  [T1566](https://attack.mitre.org/techniques/T1566)

ⓘ

Tactic: [Initial Access](https://attack.mitre.org/tactics/TA0001)

ⓘ

Platforms: Identity Provider, Linux, Office Suite, SaaS, Windows, macOS

Contributors: Jeff Sakowicz, Microsoft Identity Developer Platform Services (IDPM Services); Kobi Haimovich, CardinalOps; Mark Wee; Menachem Goldstein; Philip Winther; Saisha Agrawal, Microsoft Threat Intelligent Center (MSTIC); Shailesh Tiwary (Indian Army)

Version: 2.8

Created: 02 March 2020

Last Modified: 11 September 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1566/002/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1566/002/)

## Procedure Examples

| ID  | Name  | Description

|  [S0677](https://attack.mitre.org/software/S0677)   |  [AADInternals](https://attack.mitre.org/software/S0677)   |

[AADInternals](https://attack.mitre.org/software/S0677) can send "consent phishing" emails containing malicious links designed to steal users’ access tokens.[[8]](https://o365blog.com/aadinternals)

|  [S0584](https://attack.mitre.org/software/S0584)   |  [AppleJeus](https://attack.mitre.org/software/S0584)   |

[AppleJeus](https://attack.mitre.org/software/S0584) has been distributed via spearphishing link.[[9]](https://us-cert.cisa.gov/ncas/alerts/aa21-048a)

|  [G0099](https://attack.mitre.org/groups/G0099)   |  [APT-C-36](https://attack.mitre.org/groups/G0099)   |

[APT-C-36](https://attack.mitre.org/groups/G0099) has sent emails containing a link that appear to lead to an urgent notification from a government institution, at times using URL shorteners like cort[.]as, acortaurl[.]com, and gtly[.]to.[[10]](https://securelist.com/blindeagle-apt/113414/)[[11]](https://research.checkpoint.com/2025/blind-eagle-and-justice-for-all/)[[11]](https://research.checkpoint.com/2025/blind-eagle-and-justice-for-all/)[[12]](https://assets.recordedfuture.com/insikt-report-pdfs/2025/cta-2025-0826.pdf)

|  [G0006](https://attack.mitre.org/groups/G0006)   |  [APT1](https://attack.mitre.org/groups/G0006)   |

[APT1](https://attack.mitre.org/groups/G0006) has sent spearphishing emails containing hyperlinks to malicious files.[[13]](https://www.fireeye.com/content/dam/fireeye-www/services/pdfs/mandiant-apt1-report.pdf)

|  [G0016](https://attack.mitre.org/groups/G0016)   |  [APT29](https://attack.mitre.org/groups/G0016)   |

[APT29](https://attack.mitre.org/groups/G0016) has used spearphishing with a link to trick victims into clicking on a link to a zip file containing malicious files.[[14]](https://www.slideshare.net/slideshow/no-easy-breach-derby-con-2016/66447908)[[15]](https://www.microsoft.com/security/blog/2021/05/27/new-sophisticated-email-based-attack-from-nobelium/)[[16]](https://www.secureworks.com/blog/usaid-themed-phishing-campaign-leverages-us-elections-lure)

|  [G0022](https://attack.mitre.org/groups/G0022)   |  [APT3](https://attack.mitre.org/groups/G0022)   |

[APT3](https://attack.mitre.org/groups/G0022) has sent spearphishing emails containing malicious links.[[17]](https://www.fireeye.com/blog/threat-research/2015/06/operation-clandestine-wolf-adobe-flash-zero-day.html)

|  [G0050](https://attack.mitre.org/groups/G0050)   |  [APT32](https://attack.mitre.org/groups/G0050)   |

[APT32](https://attack.mitre.org/groups/G0050) has sent spearphishing emails containing malicious links.[[18]](https://www.welivesecurity.com/2018/03/13/oceanlotus-ships-new-backdoor/)[[19]](https://www.cybereason.com/blog/operation-cobalt-kitty-apt)[[20]](https://www.fireeye.com/blog/threat-research/2020/04/apt32-targeting-chinese-government-in-covid-19-related-espionage.html)[[21]](https://www.volexity.com/blog/2020/11/06/oceanlotus-extending-cyber-espionage-operations-through-fake-websites/)[[22]](https://www.amnestyusa.org/wp-content/uploads/2021/02/Click-and-Bait_Vietnamese-Human-Rights-Defenders-Targeted-with-Spyware-Attacks.pdf)

|  [G0064](https://attack.mitre.org/groups/G0064)   |  [APT33](https://attack.mitre.org/groups/G0064)   |

[APT33](https://attack.mitre.org/groups/G0064) has sent spearphishing emails containing links to .hta files.[[23]](https://www.fireeye.com/blog/threat-research/2017/09/apt33-insights-into-iranian-cyber-espionage.html)[[24]](https://www.symantec.com/blogs/threat-intelligence/elfin-apt33-espionage)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has sent phishing emails with invite links to fictitious meetings.[[25]](https://www.microsoft.com/en-us/security/blog/2026/04/16/dissecting-sapphire-sleets-macos-intrusion-from-lure-to-compromise/)

|  [G0087](https://attack.mitre.org/groups/G0087)   |  [APT39](https://attack.mitre.org/groups/G0087)   |

[APT39](https://attack.mitre.org/groups/G0087) leveraged spearphishing emails with malicious links to initially compromise victims.[[26]](https://www.fireeye.com/blog/threat-research/2019/01/apt39-iranian-cyber-espionage-group-focused-on-personal-information.html)[[27]](https://www.iranwatch.org/sites/default/files/public-intelligence-alert.pdf)

|  [G1044](https://attack.mitre.org/groups/G1044)   |  [APT42](https://attack.mitre.org/groups/G1044)   |

[APT42](https://attack.mitre.org/groups/G1044) has sent spearphishing emails containing malicious links.[[28]](https://services.google.com/fh/files/misc/apt42-crooked-charms-cons-and-compromises.pdf)[[29]](https://cloud.google.com/blog/topics/threat-intelligence/untangling-iran-apt42-operations)[[30]](https://blog.google/threat-analysis-group/iranian-backed-group-steps-up-phishing-campaigns-against-israel-us/)

|  [S0534](https://attack.mitre.org/software/S0534)   |  [Bazar](https://attack.mitre.org/software/S0534)   |

[Bazar](https://attack.mitre.org/software/S0534) has been spread via emails with embedded malicious links.[[31]](https://www.cybereason.com/blog/a-bazar-of-tricks-following-team9s-development-cycles)[[32]](https://www.zscaler.com/blogs/research/spear-phishing-campaign-delivers-buer-and-bazar-malware)[[33]](https://www.crowdstrike.com/blog/wizard-spider-adversary-update/)

|  [G0098](https://attack.mitre.org/groups/G0098)   |  [BlackTech](https://attack.mitre.org/groups/G0098)   |

[BlackTech](https://attack.mitre.org/groups/G0098) has used spearphishing e-mails with links to cloud services to deliver malware.[[34]](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)

|  [S1039](https://attack.mitre.org/software/S1039)   |  [Bumblebee](https://attack.mitre.org/software/S1039)   |

[Bumblebee](https://attack.mitre.org/software/S1039) has been spread through e-mail campaigns with malicious links.[[35]](https://www.proofpoint.com/us/blog/threat-insight/bumblebee-is-still-transforming)[[36]](https://www.cybereason.com/blog/threat-analysis-report-bumblebee-loader-the-high-road-to-enterprise-domain-control)

|  [C0011](https://attack.mitre.org/campaigns/C0011)   |  [C0011](https://attack.mitre.org/campaigns/C0011)   |

During [C0011](https://attack.mitre.org/campaigns/C0011), [Transparent Tribe](https://attack.mitre.org/groups/G0134) sent emails containing a malicious link to student targets in India.[[37]](https://blog.talosintelligence.com/2022/07/transparent-tribe-targets-education.html)

|  [C0021](https://attack.mitre.org/campaigns/C0021)   |  [C0021](https://attack.mitre.org/campaigns/C0021)   |

During [C0021](https://attack.mitre.org/campaigns/C0021), the threat actors sent phishing emails with unique malicious links, likely for tracking victim clicks.[[38]](https://www.fireeye.com/blog/threat-research/2018/11/not-so-cozy-an-uncomfortable-examination-of-a-suspected-apt29-phishing-campaign.html)[[39]](https://www.microsoft.com/security/blog/2018/12/03/analysis-of-cyberattack-on-u-s-think-tanks-non-profits-public-sector-by-unidentified-attackers/)

|  [G0080](https://attack.mitre.org/groups/G0080)   |  [Cobalt Group](https://attack.mitre.org/groups/G0080)   |

[Cobalt Group](https://attack.mitre.org/groups/G0080) has sent emails with URLs pointing to malicious documents.[[40]](https://blog.talosintelligence.com/2018/07/multiple-cobalt-personality-disorder.html)[[41]](https://www.secureworks.com/blog/cybercriminals-increasingly-trying-to-ensnare-the-big-financial-fish)

|  [G0142](https://attack.mitre.org/groups/G0142)   |  [Confucius](https://attack.mitre.org/groups/G0142)   |

[Confucius](https://attack.mitre.org/groups/G0142) has sent malicious links to victims through email campaigns.[[42]](https://www.trendmicro.com/en_us/research/21/h/confucius-uses-pegasus-spyware-related-lures-to-target-pakistani.html)

|  [S1111](https://attack.mitre.org/software/S1111)   |  [DarkGate](https://attack.mitre.org/software/S1111)   |

[DarkGate](https://attack.mitre.org/software/S1111) is distributed in phishing emails containing links to distribute malicious VBS or MSI files.[[43]](https://www.trellix.com/blogs/research/the-continued-evolution-of-the-darkgate-malware-as-a-service/) [DarkGate](https://attack.mitre.org/software/S1111) uses applications such as Microsoft Teams for distributing links to payloads.[[43]](https://www.trellix.com/blogs/research/the-continued-evolution-of-the-darkgate-malware-as-a-service/)

|  [G1006](https://attack.mitre.org/groups/G1006)   |  [Earth Lusca](https://attack.mitre.org/groups/G1006)   |

[Earth Lusca](https://attack.mitre.org/groups/G1006) has sent spearphishing emails to potential targets that contained a malicious link.[[44]](https://www.trendmicro.com/content/dam/trendmicro/global/en/research/22/a/earth-lusca-employs-sophisticated-infrastructure-varied-tools-and-techniques/technical-brief-delving-deep-an-analysis-of-earth-lusca-operations.pdf)

|  [G0066](https://attack.mitre.org/groups/G0066)   |  [Elderwood](https://attack.mitre.org/groups/G0066)   |

[Elderwood](https://attack.mitre.org/groups/G0066) has delivered zero-day exploits and malware to victims via targeted emails containing a link to malicious content hosted on an uncommon Web server.[[45]](https://web.archive.org/web/20190717233006/http:/www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/the-elderwood-project.pdf)[[46]](https://www.csmonitor.com/USA/2012/0914/Stealing-US-business-secrets-Experts-ID-two-huge-cyber-gangs-in-China)

|  [S0367](https://attack.mitre.org/software/S0367)   |  [Emotet](https://attack.mitre.org/software/S0367)   |

[Emotet](https://attack.mitre.org/software/S0367) has been delivered by phishing emails containing links. [[47]](https://blog.trendmicro.com/trendlabs-security-intelligence/new-banking-malware-uses-network-sniffing-for-data-theft/)[[48]](https://securelist.com/the-banking-trojan-emotet-detailed-analysis/69560/)[[49]](https://www.cisecurity.org/blog/emotet-changes-ttp-and-arrives-in-united-states/)[[50]](https://support.malwarebytes.com/docs/DOC-2295)[[51]](https://www.symantec.com/blogs/threat-intelligence/evolution-emotet-trojan-distributor)[[52]](https://www.us-cert.gov/ncas/alerts/TA18-201A)[[53]](https://blog.talosintelligence.com/2019/01/return-of-emotet.html)[[53]](https://blog.talosintelligence.com/2019/01/return-of-emotet.html)[[54]](https://www.picussecurity.com/blog/the-christmas-card-you-never-wanted-a-new-wave-of-emotet-is-back-to-wreak-havoc.html)

|  [G0120](https://attack.mitre.org/groups/G0120)   |  [Evilnum](https://attack.mitre.org/groups/G0120)   |

[Evilnum](https://attack.mitre.org/groups/G0120) has sent spearphishing emails containing a link to a zip file hosted on Google Drive.[[55]](https://www.welivesecurity.com/2020/07/09/more-evil-deep-look-evilnum-toolset/)

|  [G1011](https://attack.mitre.org/groups/G1011)   |  [EXOTIC LILY](https://attack.mitre.org/groups/G1011)   |

[EXOTIC LILY](https://attack.mitre.org/groups/G1011) has relied on victims to open malicious links in e-mails for execution.[[56]](https://blog.google/threat-analysis-group/exposing-initial-access-broker-ties-conti/)

|  [G0085](https://attack.mitre.org/groups/G0085)   |  [FIN4](https://attack.mitre.org/groups/G0085)   |

[FIN4](https://attack.mitre.org/groups/G0085) has used spearphishing emails (often sent from compromised accounts) containing malicious links.[[57]](https://web.archive.org/web/20220313062515/https://www.mandiant.com/sites/default/files/2021-09/rpt-fin4.pdf)[[58]](https://www2.fireeye.com/WBNR-14Q4NAMFIN4.html)

|  [G0046](https://attack.mitre.org/groups/G0046)   |  [FIN7](https://attack.mitre.org/groups/G0046)   |

[FIN7](https://attack.mitre.org/groups/G0046) has conducted broad phishing campaigns using malicious links.[[59]](https://www.crowdstrike.com/blog/carbon-spider-embraces-big-game-hunting-part-1/) Additionally, [FIN7](https://attack.mitre.org/groups/G0046) has sent spearphishing emails containing a typosquatted link to "ip-sccanner[.]com."[[60]](https://blogs.blackberry.com/en/2024/04/fin7-targets-the-united-states-automotive-industry)

|  [G0061](https://attack.mitre.org/groups/G0061)   |  [FIN8](https://attack.mitre.org/groups/G0061)   |

[FIN8](https://attack.mitre.org/groups/G0061) has distributed targeted emails containing links to malicious documents with embedded macros.[[61]](https://www2.fireeye.com/WBNR-Know-Your-Enemy-UNC622-Spear-Phishing.html)

|  [S0531](https://attack.mitre.org/software/S0531)   |  [Grandoreiro](https://attack.mitre.org/software/S0531)   |

[Grandoreiro](https://attack.mitre.org/software/S0531) has been spread via malicious links embedded in e-mails.[[62]](https://securityintelligence.com/posts/grandoreiro-malware-now-targeting-banks-in-spain/)[[63]](https://www.welivesecurity.com/2020/04/28/grandoreiro-how-engorged-can-exe-get/)

|  [S0561](https://attack.mitre.org/software/S0561)   |  [GuLoader](https://attack.mitre.org/software/S0561)   |

[GuLoader](https://attack.mitre.org/software/S0561) has been spread in phishing campaigns using malicious web links.[[64]](https://unit42.paloaltonetworks.com/guloader-installing-netwire-rat/)

|  [S0499](https://attack.mitre.org/software/S0499)   |  [Hancitor](https://attack.mitre.org/software/S0499)   |

[Hancitor](https://attack.mitre.org/software/S0499) has been delivered via phishing emails which contained malicious links.[[65]](https://threatpost.com/spammers-revive-hancitor-downloader-campaigns/123011/)

|  [S1229](https://attack.mitre.org/software/S1229)   |  [Havoc](https://attack.mitre.org/software/S1229)   |

[Havoc](https://attack.mitre.org/software/S1229) has been distributed through ClickFix phishing campaigns.[[66]](https://www.fortinet.com/blog/threat-research/havoc-sharepoint-with-microsoft-graph-api-turns-into-fud-c2)

|  [S0528](https://attack.mitre.org/software/S0528)   |  [Javali](https://attack.mitre.org/software/S0528)   |

[Javali](https://attack.mitre.org/software/S0528) has been delivered via malicious links embedded in e-mails.[[67]](https://securelist.com/the-tetrade-brazilian-banking-malware/97779/)

|  [S9044](https://attack.mitre.org/software/S9044)   |  [Kali365](https://attack.mitre.org/software/S9044)   |

[Kali365](https://attack.mitre.org/software/S9044) has sent bulk phishing emails containing malicious hyperlinks that direct victims to actor-controlled landing pages impersonating services including SharePoint, OneDrive, Teams, DocuSign, and Adobe Acrobat Sign. [[68]](https://arcticwolf.com/resources/blog/token-bingo-dont-let-your-code-be-the-winner/)

|  [S0585](https://attack.mitre.org/software/S0585)   |  [Kerrdown](https://attack.mitre.org/software/S0585)   |

[Kerrdown](https://attack.mitre.org/software/S0585) has been distributed via e-mails containing a malicious link.[[22]](https://www.amnestyusa.org/wp-content/uploads/2021/02/Click-and-Bait_Vietnamese-Human-Rights-Defenders-Targeted-with-Spyware-Attacks.pdf)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has sent spearphishing emails containing a link to a document that contained malicious macros or took the victim to an actor-controlled domain.[[69]](https://blog.alyac.co.kr/2234)[[70]](https://asert.arbornetworks.com/stolen-pencil-campaign-targets-academia/)[[71]](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)

|  [S0669](https://attack.mitre.org/software/S0669)   |  [KOCTOPUS](https://attack.mitre.org/software/S0669)   |

[KOCTOPUS](https://attack.mitre.org/software/S0669) has been distributed as a malicious link within an email.[[72]](https://web.archive.org/web/20211003035156/https://www.malwarebytes.com/resources/files/2021/02/lazyscripter.pdf)

|  [S1160](https://attack.mitre.org/software/S1160)   |  [Latrodectus](https://attack.mitre.org/software/S1160)   |

[Latrodectus](https://attack.mitre.org/software/S1160) has been distributed to victims through emails containing malicious links.[[73]](https://www.proofpoint.com/us/blog/threat-insight/latrodectus-spider-bytes-ice)[[74]](https://www.bleepingcomputer.com/news/security/new-latrodectus-malware-attacks-use-microsoft-cloudflare-themes/)

|  [G0032](https://attack.mitre.org/groups/G0032)   |  [Lazarus Group](https://attack.mitre.org/groups/G0032)   |

[Lazarus Group](https://attack.mitre.org/groups/G0032) has sent malicious links to victims via email.[[75]](https://securelist.com/lazarus-threatneedle/100803/)

|  [G0140](https://attack.mitre.org/groups/G0140)   |  [LazyScripter](https://attack.mitre.org/groups/G0140)   |

[LazyScripter](https://attack.mitre.org/groups/G0140) has used spam emails that contain a link that redirects the victim to download a malicious document.[[72]](https://web.archive.org/web/20211003035156/https://www.malwarebytes.com/resources/files/2021/02/lazyscripter.pdf)

|  [G0065](https://attack.mitre.org/groups/G0065)   |  [Leviathan](https://attack.mitre.org/groups/G0065)   |

[Leviathan](https://attack.mitre.org/groups/G0065) has sent spearphishing emails with links, often using a fraudulent lookalike domain and stolen branding.[[76]](https://www.proofpoint.com/us/threat-insight/post/leviathan-espionage-actor-spearphishes-maritime-and-defense-targets)[[77]](https://us-cert.cisa.gov/ncas/alerts/aa21-200a)

|  [G1014](https://attack.mitre.org/groups/G1014)   |  [LuminousMoth](https://attack.mitre.org/groups/G1014)   |

[LuminousMoth](https://attack.mitre.org/groups/G1014) has sent spearphishing emails containing a malicious Dropbox download link.[[78]](https://securelist.com/apt-luminousmoth/103332/)

|  [S1213](https://attack.mitre.org/software/S1213)   |  [Lumma Stealer](https://attack.mitre.org/software/S1213)   |

[Lumma Stealer](https://attack.mitre.org/software/S1213) has been delivered through phishing emails containing malicious links.[[79]](https://www.cybereason.com/blog/threat-analysis-rise-of-lummastealer)

|  [G0095](https://attack.mitre.org/groups/G0095)   |  [Machete](https://attack.mitre.org/groups/G0095)   |

[Machete](https://attack.mitre.org/groups/G0095) has sent phishing emails that contain a link to an external server with ZIP and RAR archives.[[80]](https://threatvector.cylance.com/en_us/home/el-machete-malware-attacks-cut-through-latam.html)[[81]](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)

|  [G0059](https://attack.mitre.org/groups/G0059)   |  [Magic Hound](https://attack.mitre.org/groups/G0059)   |

[Magic Hound](https://attack.mitre.org/groups/G0059) has sent malicious URL links through email to victims. In some cases the URLs were shortened or linked to Word documents with malicious macros that executed PowerShells scripts to download [Pupy](https://attack.mitre.org/software/S0192).[[82]](https://www.secureworks.com/blog/iranian-pupyrat-bites-middle-eastern-organizations)[[83]](https://www.clearskysec.com/wp-content/uploads/2020/08/The-Kittens-are-Back-in-Town-3.pdf)[[84]](https://blog.certfa.com/posts/charming-kitten-christmas-gift/)[[85]](https://www.microsoft.com/en-us/security/blog/2021/11/16/evolving-trends-in-iranian-threat-actor-activity-mstic-presentation-at-cyberwarcon-2021)

|  [S0530](https://attack.mitre.org/software/S0530)   |  [Melcoz](https://attack.mitre.org/software/S0530)   |

[Melcoz](https://attack.mitre.org/software/S0530) has been spread through malicious links embedded in e-mails.[[67]](https://securelist.com/the-tetrade-brazilian-banking-malware/97779/)

|  [G1054](https://attack.mitre.org/groups/G1054)   |  [MirrorFace](https://attack.mitre.org/groups/G1054)   |

[MirrorFace](https://attack.mitre.org/groups/G1054) has embedded OneDrive URLs in emails leading to malicious file installation.[[86]](https://www.trendmicro.com/en_us/research/25/d/earth-kasha-updates-ttps.html)

|  [S1122](https://attack.mitre.org/software/S1122)   |  [Mispadu](https://attack.mitre.org/software/S1122)   |

[Mispadu](https://attack.mitre.org/software/S1122) has been spread via malicious links embedded in emails.[[87]](https://blog.scilabs.mx/en/cyber-threat-profile-malteiro/)

|  [G0103](https://attack.mitre.org/groups/G0103)   |  [Mofang](https://attack.mitre.org/groups/G0103)   |

[Mofang](https://attack.mitre.org/groups/G0103) delivered spearphishing emails with malicious links included.[[88]](https://foxitsecurity.files.wordpress.com/2016/06/fox-it_mofang_threatreport_tlp-white.pdf)

|  [G0021](https://attack.mitre.org/groups/G0021)   |  [Molerats](https://attack.mitre.org/groups/G0021)   |

[Molerats](https://attack.mitre.org/groups/G0021) has sent phishing emails with malicious links included.[[89]](https://securelist.com/gaza-cybergang-group1-operation-sneakypastes/90068/)

|  [G0069](https://attack.mitre.org/groups/G0069)   |  [MuddyWater](https://attack.mitre.org/groups/G0069)   |

[MuddyWater](https://attack.mitre.org/groups/G0069) has sent targeted spearphishing e-mails with malicious links.[[90]](https://www.anomali.com/blog/probable-iranian-cyber-actors-static-kitten-conducting-cyberespionage-campaign-targeting-uae-and-kuwait-government-agencies)[[91]](https://www.trendmicro.com/en_us/research/21/c/earth-vetala---muddywater-continues-to-target-organizations-in-t.html)[[92]](https://www.proofpoint.com/us/blog/threat-insight/security-brief-ta450-uses-embedded-links-pdf-attachments-latest-campaign)

|  [G0129](https://attack.mitre.org/groups/G0129)   |  [Mustang Panda](https://attack.mitre.org/groups/G0129)   |

[Mustang Panda](https://attack.mitre.org/groups/G0129) has delivered malicious links to their intended targets.[[93]](https://www.ibm.com/think/x-force/hive0154-mustang-panda-shifts-focus-tibetan-community-deploy-pubload-backdoor)[[94]](https://www.ibm.com/think/x-force/hive0154-targeting-us-philippines-pakistan-taiwan)[[95]](https://www.mcafee.com/enterprise/en-us/assets/reports/rp-operation-dianxun.pdf) [Mustang Panda](https://attack.mitre.org/groups/G0129) has distributed spear-phishing emails with embedded links that direct the victim to a malicious archive hosted on Google or Dropbox.[[96]](https://www.trendmicro.com/en_us/research/22/k/earth-preta-spear-phishing-governments-worldwide.html)

|  [G1020](https://attack.mitre.org/groups/G1020)   |  [Mustard Tempest](https://attack.mitre.org/groups/G1020)   |

[Mustard Tempest](https://attack.mitre.org/groups/G1020) has sent victims emails containing links to compromised websites.[[97]](https://www.proofpoint.com/us/blog/threat-insight/part-1-socgholish-very-real-threat-very-fake-update)

|  [S0198](https://attack.mitre.org/software/S0198)   |  [NETWIRE](https://attack.mitre.org/software/S0198)   |

[NETWIRE](https://attack.mitre.org/software/S0198) has been spread via e-mail campaigns utilizing malicious links.[[64]](https://unit42.paloaltonetworks.com/guloader-installing-netwire-rat/)

|  [C0002](https://attack.mitre.org/campaigns/C0002)   |  [Night Dragon](https://attack.mitre.org/campaigns/C0002)   |

During [Night Dragon](https://attack.mitre.org/campaigns/C0002), threat actors sent spearphishing emails containing links to compromised websites where malware was downloaded.[[98]](https://scadahacker.com/library/Documents/Cyber_Events/McAfee%20-%20Night%20Dragon%20-%20Global%20Energy%20Cyberattacks.pdf)

|  [G1059](https://attack.mitre.org/groups/G1059)   |  [NIMBUS MANTICORE](https://attack.mitre.org/groups/G1059)   |

[NIMBUS MANTICORE](https://attack.mitre.org/groups/G1059) has utilized malicious links in phishing emails that direct users to download a malicious payload.[[99]](https://research.checkpoint.com/2026/fast-and-furious-nimbus-manticore-operations-during-the-iranian-conflict/)[[100]](https://research.checkpoint.com/2025/nimbus-manticore-deploys-new-malware-targeting-europe/)[[101]](https://cloud.google.com/blog/topics/threat-intelligence/suspected-iranian-unc1549-targets-israel-middle-east)[[102]](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) has sent spearphising emails with malicious links to potential victims.[[103]](https://researchcenter.paloaltonetworks.com/2018/02/unit42-oopsie-oilrig-uses-threedollars-deliver-new-trojan/)[[104]](http://www.clearskysec.com/oilrig/)

|  [C0060](https://attack.mitre.org/campaigns/C0060)   |  [Operation AkaiRyū](https://attack.mitre.org/campaigns/C0060)   |

During [Operation AkaiRyū](https://attack.mitre.org/campaigns/C0060), [MirrorFace](https://attack.mitre.org/groups/G1054) sent spearphishing emails with malicious OneDrive links.[[105]](https://www.trendmicro.com/en_us/research/24/k/return-of-anel-in-the-recent-earth-kasha-spearphishing-campaign.html)

|  [C0022](https://attack.mitre.org/campaigns/C0022)   |  [Operation Dream Job](https://attack.mitre.org/campaigns/C0022)   |

During [Operation Dream Job](https://attack.mitre.org/campaigns/C0022), [Lazarus Group](https://attack.mitre.org/groups/G0032) sent malicious OneDrive links with fictitious job offer advertisements via email.[[106]](https://www.clearskysec.com/wp-content/uploads/2020/08/Dream-Job-Campaign.pdf)[[107]](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_Operation_Interception.pdf)

|  [C0016](https://attack.mitre.org/campaigns/C0016)   |  [Operation Dust Storm](https://attack.mitre.org/campaigns/C0016)   |

During [Operation Dust Storm](https://attack.mitre.org/campaigns/C0016), the threat actors sent spearphishing emails containing a malicious link.[[108]](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)

|  [C0005](https://attack.mitre.org/campaigns/C0005)   |  [Operation Spalax](https://attack.mitre.org/campaigns/C0005)   |

During [Operation Spalax](https://attack.mitre.org/campaigns/C0005), the threat actors sent phishing emails to victims that contained a malicious link.[[109]](https://www.welivesecurity.com/2021/01/12/operation-spalax-targeted-malware-attacks-colombia/)

|  [S1017](https://attack.mitre.org/software/S1017)   |  [OutSteel](https://attack.mitre.org/software/S1017)   |

[OutSteel](https://attack.mitre.org/software/S1017) has been distributed through malicious links contained within spearphishing emails.[[110]](https://unit42.paloaltonetworks.com/ukraine-targeted-outsteel-saintbot/)

|  [G0040](https://attack.mitre.org/groups/G0040)   |  [Patchwork](https://attack.mitre.org/groups/G0040)   |

[Patchwork](https://attack.mitre.org/groups/G0040) has used spearphishing with links to deliver files with exploits to initial victims.[[111]](http://www.symantec.com/connect/blogs/patchwork-cyberespionage-group-expands-targets-governments-wide-range-industries)[[112]](https://documents.trendmicro.com/assets/tech-brief-untangling-the-patchwork-cyberespionage-group.pdf)[[113]](https://unit42.paloaltonetworks.com/updated-backconfig-malware-targeting-government-and-military-organizations/)

|  [C0036](https://attack.mitre.org/campaigns/C0036)   |  [Pikabot Distribution February 2024](https://attack.mitre.org/campaigns/C0036)   |

[Pikabot Distribution February 2024](https://attack.mitre.org/campaigns/C0036) utilized emails with hyperlinks leading to malicious ZIP archive files containing scripts to download and install [Pikabot](https://attack.mitre.org/software/S1145).[[114]](https://www.elastic.co/security-labs/pikabot-i-choose-you)

|  [S0453](https://attack.mitre.org/software/S0453)   |  [Pony](https://attack.mitre.org/software/S0453)   |

[Pony](https://attack.mitre.org/software/S0453) has been delivered via spearphishing emails which contained malicious links.[[115]](https://blog.malwarebytes.com/threat-analysis/2015/11/no-money-but-pony-from-a-mail-to-a-trojan-horse/)

|  [S0650](https://attack.mitre.org/software/S0650)   |  [QakBot](https://attack.mitre.org/software/S0650)   |

[QakBot](https://attack.mitre.org/software/S0650) has spread through emails with malicious links.[[116]](https://www.trendmicro.com/vinfo/ph/security/news/cybercrime-and-digital-threats/qakbot-resurges-spreads-through-vbs-files)[[117]](https://www.kroll.com/en/insights/publications/cyber/qakbot-malware-exfiltrating-emails-thread-hijacking-attacks)[[118]](https://success.trendmicro.com/en-US/solution/KA-0011282)[[119]](https://cybersecurity.att.com/blogs/labs-research/the-rise-of-qakbot)[[120]](https://securelist.com/qakbot-technical-analysis/103931/)[[121]](https://web.archive.org/web/20220119114433/https://groupib.pathfactory.com/ransomware-reports/prolock_wp)[[122]](https://www.trendmicro.com/en_us/research/22/j/black-basta-infiltrates-networks-via-qakbot-brute-ratel-and-coba.html)

|  [S1242](https://attack.mitre.org/software/S1242)   |  [Qilin](https://attack.mitre.org/software/S1242)   |

[Qilin](https://attack.mitre.org/software/S1242) has been delivered via malicious links in spearphishing emails.[[123]](https://www.sentinelone.com/anthology/agenda-qilin/)[[124]](https://news.sophos.com/en-us/2025/04/01/sophos-mdr-tracks-ongoing-campaign-by-qilin-affiliates-targeting-screenconnect/)

|  [G1039](https://attack.mitre.org/groups/G1039)   |  [RedCurl](https://attack.mitre.org/groups/G1039)   |

[RedCurl](https://attack.mitre.org/groups/G1039) has used phishing emails with malicious links to gain initial access.[[125]](https://www.group-ib.com/resources/research-hub/red-curl/)[[126]](https://www.group-ib.com/resources/research-hub/red-curl-2/)

|  [C0047](https://attack.mitre.org/campaigns/C0047)   |  [RedDelta Modified PlugX Infection Chain Operations](https://attack.mitre.org/campaigns/C0047)   |

[Mustang Panda](https://attack.mitre.org/groups/G0129) distributed malicious links in phishing emails leading to HTML files that would direct the victim to malicious MSC files if running Windows based on User Agent fingerprinting during [RedDelta Modified PlugX Infection Chain Operations](https://attack.mitre.org/campaigns/C0047).[[127]](https://go.recordedfuture.com/hubfs/reports/cta-cn-2025-0109.pdf)

|  [S9026](https://attack.mitre.org/software/S9026)   |  [ROAMINGHOUSE](https://attack.mitre.org/software/S9026)   |

[ROAMINGHOUSE](https://attack.mitre.org/software/S9026) has been distributed through phishing emails containing malicious OneDrive links.[[86]](https://www.trendmicro.com/en_us/research/25/d/earth-kasha-updates-ttps.html)

|  [S1018](https://attack.mitre.org/software/S1018)   |  [Saint Bot](https://attack.mitre.org/software/S1018)   |

[Saint Bot](https://attack.mitre.org/software/S1018) has been distributed through malicious links contained within spearphishing emails.[[110]](https://unit42.paloaltonetworks.com/ukraine-targeted-outsteel-saintbot/)

|  [G0034](https://attack.mitre.org/groups/G0034)   |  [Sandworm Team](https://attack.mitre.org/groups/G0034)   |

[Sandworm Team](https://attack.mitre.org/groups/G0034) has crafted phishing emails containing malicious hyperlinks.[[128]](https://www.justice.gov/opa/press-release/file/1328521/download)

|  [G0121](https://attack.mitre.org/groups/G0121)   |  [Sidewinder](https://attack.mitre.org/groups/G0121)   |

[Sidewinder](https://attack.mitre.org/groups/G0121) has sent e-mails with malicious links often crafted for specific targets.[[129]](https://cdn-cybersecurity.att.com/docs/global-perspective-of-the-sidewinder-apt.pdf)[[130]](https://cybleinc.com/2020/09/26/sidewinder-apt-targets-with-futuristic-tactics-and-techniques/)

|  [S1086](https://attack.mitre.org/software/S1086)   |  [Snip3](https://attack.mitre.org/software/S1086)   |

[Snip3](https://attack.mitre.org/software/S1086) has been delivered to victims through e-mail links to malicious files.[[131]](https://telefonicatech.com/blog/snip3-investigacion-malware)

|  [S1124](https://attack.mitre.org/software/S1124)   |  [SocGholish](https://attack.mitre.org/software/S1124)   |

[SocGholish](https://attack.mitre.org/software/S1124) has been spread via emails containing malicious links.[[97]](https://www.proofpoint.com/us/blog/threat-insight/part-1-socgholish-very-real-threat-very-fake-update)

|  [S0646](https://attack.mitre.org/software/S0646)   |  [SpicyOmelette](https://attack.mitre.org/software/S0646)   |

[SpicyOmelette](https://attack.mitre.org/software/S0646) has been distributed via emails containing a malicious link that appears to be a PDF document.[[41]](https://www.secureworks.com/blog/cybercriminals-increasingly-trying-to-ensnare-the-big-financial-fish)

|  [S1030](https://attack.mitre.org/software/S1030)   |  [Squirrelwaffle](https://attack.mitre.org/software/S1030)   |

[Squirrelwaffle](https://attack.mitre.org/software/S1030) has been distributed through phishing emails containing a malicious URL.[[132]](https://www.zscaler.com/blogs/security-research/squirrelwaffle-new-loader-delivering-cobalt-strike)

|  [G1046](https://attack.mitre.org/groups/G1046)   |  [Storm-1811](https://attack.mitre.org/groups/G1046)   |

[Storm-1811](https://attack.mitre.org/groups/G1046) has distributed malicious links to victims that redirect to EvilProxy-based phishing sites to harvest credentials.[[133]](https://www.microsoft.com/en-us/security/blog/2024/05/15/threat-actors-misusing-quick-assist-in-social-engineering-attacks-leading-to-ransomware/)

|  [G1018](https://attack.mitre.org/groups/G1018)   |  [TA2541](https://attack.mitre.org/groups/G1018)   |

[TA2541](https://attack.mitre.org/groups/G1018) has used spearphishing e-mails with malicious links to deliver malware. [[134]](https://www.proofpoint.com/us/blog/threat-insight/charting-ta2541s-flight)[[131]](https://telefonicatech.com/blog/snip3-investigacion-malware)

|  [G0092](https://attack.mitre.org/groups/G0092)   |  [TA505](https://attack.mitre.org/groups/G0092)   |

[TA505](https://attack.mitre.org/groups/G0092) has sent spearphishing emails containing malicious links.[[135]](https://www.proofpoint.com/us/threat-insight/post/threat-actor-profile-ta505-dridex-globeimposter)[[136]](https://www.proofpoint.com/us/threat-insight/post/servhelper-and-flawedgrace-new-malware-introduced-ta505)[[137]](https://blog.trendmicro.com/trendlabs-security-intelligence/shifting-tactics-breaking-down-ta505-groups-use-of-html-rats-and-other-techniques-in-latest-campaigns/)[[138]](https://www.proofpoint.com/us/threat-insight/post/ta505-distributes-new-sdbbot-remote-access-trojan-get2-downloader)

|  [G1037](https://attack.mitre.org/groups/G1037)   |  [TA577](https://attack.mitre.org/groups/G1037)   |

[TA577](https://attack.mitre.org/groups/G1037) has sent emails containing links to malicious JavaScript files.[[73]](https://www.proofpoint.com/us/blog/threat-insight/latrodectus-spider-bytes-ice)

|  [G0134](https://attack.mitre.org/groups/G0134)   |  [Transparent Tribe](https://attack.mitre.org/groups/G0134)   |

[Transparent Tribe](https://attack.mitre.org/groups/G0134) has embedded links to malicious downloads in e-mails.[[139]](https://blog.talosintelligence.com/2021/02/obliquerat-new-campaign.html)[[140]](https://blog.talosintelligence.com/2021/05/transparent-tribe-infra-and-targeting.html)

|  [S0266](https://attack.mitre.org/software/S0266)   |  [TrickBot](https://attack.mitre.org/software/S0266)   |

[TrickBot](https://attack.mitre.org/software/S0266) has been delivered via malicious links in phishing e-mails.[[141]](https://www.cybereason.com/blog/dropping-anchor-from-a-trickbot-infection-to-the-discovery-of-the-anchor-malware)

|  [G0010](https://attack.mitre.org/groups/G0010)   |  [Turla](https://attack.mitre.org/groups/G0010)   |

[Turla](https://attack.mitre.org/groups/G0010) attempted to trick targets into clicking on a link featuring a seemingly legitimate domain from Adobe.com to download their malware and gain initial access.[[142]](https://www.welivesecurity.com/wp-content/uploads/2018/01/ESET_Turla_Mosquito.pdf)

|  [S0476](https://attack.mitre.org/software/S0476)   |  [Valak](https://attack.mitre.org/software/S0476)   |

[Valak](https://attack.mitre.org/software/S0476) has been delivered via malicious links in e-mail.[[143]](https://assets.sentinelone.com/labs/sentinel-one-valak-i)

|  [S9050](https://attack.mitre.org/software/S9050)   |  [Vidar](https://attack.mitre.org/software/S9050)   |

[Vidar](https://attack.mitre.org/software/S9050) has been distributed through emails that include malicious links that redirect to fake download pages for Adobe Illustrator.[[144]](https://www.quorumcyber.com/wp-content/uploads/2023/01/Malware-Analysis-Vidar.pdf)

|  [G0112](https://attack.mitre.org/groups/G0112)   |  [Windshift](https://attack.mitre.org/groups/G0112)   |

[Windshift](https://attack.mitre.org/groups/G0112) has sent spearphishing emails with links to harvest credentials and deliver malware.[[145]](https://www.scribd.com/document/661837258/WINDSHIFT-summit-archive-1554718868)

|  [G0090](https://attack.mitre.org/groups/G0090)   |  [WIRTE](https://attack.mitre.org/groups/G0090)   |

[WIRTE](https://attack.mitre.org/groups/G0090) has sent targeted spearphishing emails with malicious links directing victims to malware downloads.[[146]](https://research.checkpoint.com/2024/hamas-affiliated-threat-actor-expands-to-disruptive-activity/)

|  [G0102](https://attack.mitre.org/groups/G0102)   |  [Wizard Spider](https://attack.mitre.org/groups/G0102)   |

[Wizard Spider](https://attack.mitre.org/groups/G0102) has sent phishing emails containing a link to an actor-controlled Google Drive document or other free online file hosting services.[[147]](https://us-cert.cisa.gov/ncas/alerts/aa20-302a)[[148]](https://thedfirreport.com/2020/11/05/ryuk-speed-run-2-hours-to-ransom/)

|  [G0128](https://attack.mitre.org/groups/G0128)   |  [ZIRCONIUM](https://attack.mitre.org/groups/G0128)   |

[ZIRCONIUM](https://attack.mitre.org/groups/G0128) has used malicious links in e-mails to deliver malware.[[149]](https://blogs.microsoft.com/on-the-issues/2020/09/10/cyberattacks-us-elections-trump-biden/)[[150]](https://blog.google/threat-analysis-group/how-were-tackling-evolving-online-threats/)[[151]](https://www.zscaler.com/blogs/security-research/apt-31-leverages-covid-19-vaccine-theme-and-abuses-legitimate-online)

## Mitigations

| ID  | Mitigation  | Description

|  [M1047](https://attack.mitre.org/mitigations/M1047)   |  [Audit](https://attack.mitre.org/mitigations/M1047)   |

Audit applications and their permissions to ensure access to data and resources are limited based upon necessity and principle of least privilege.

|  [M1021](https://attack.mitre.org/mitigations/M1021)   |  [Restrict Web-Based Content](https://attack.mitre.org/mitigations/M1021)   |

Determine if certain websites that can be used for spearphishing are necessary for business operations and consider blocking access if activity cannot be monitored well or if it poses a significant risk.

|  [M1054](https://attack.mitre.org/mitigations/M1054)   |  [Software Configuration](https://attack.mitre.org/mitigations/M1054)   |

Use anti-spoofing and email authentication mechanisms to filter messages based on validity checks of the sender domain (using SPF) and integrity of messages (using DKIM). Enabling these mechanisms within an organization (through policies such as DMARC) may enable recipients (intra-org and cross domain) to perform similar message filtering and validation.[[152]](https://docs.microsoft.com/en-us/microsoft-365/security/office-365-security/anti-spoofing-protection?view=o365-worldwide)[[153]](https://web.archive.org/web/20210708014107/https://www.cyber.gov.au/sites/default/files/2019-03/spoof_email_sender_policy_framework.pdf).

Furthermore, policies may enforce / install browser extensions that protect against IDN and homograph attacks.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Azure AD Administrators apply limitations upon the ability for users to grant consent to unfamiliar or unverified third-party applications.

|  [M1017](https://attack.mitre.org/mitigations/M1017)   |  [User Training](https://attack.mitre.org/mitigations/M1017)   |

Users can be trained to identify social engineering techniques and spearphishing emails with malicious links which includes phishing for consent with OAuth 2.0. Additionally, users may perform visual checks of the domains they visit; however, homographs in ASCII and in IDN domains and URL schema obfuscation may render manual checks difficult. Use email warning banners to alert users when emails contain links from external senders, prompting them to exercise caution and reducing the likelihood of falling victim to spearphishing attacks. Phishing training and other cybersecurity training may raise awareness to check URLs before visiting the sites.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0107](https://attack.mitre.org/detectionstrategies/DET0107)   |  [Detection Strategy for Spearphishing Links](https://attack.mitre.org/detectionstrategies/DET0107)   |  [AN0298](https://attack.mitre.org/detectionstrategies/DET0107#AN0298)   |

Correlation of inbound emails with embedded links followed by user-driven browser navigation to suspicious or obfuscated domains. Detection chain includes malicious URL in email → user click recorded in Office logs → browser process spawning unusual child processes (e.g., PowerShell, cmd) or download activity.

|  [AN0299](https://attack.mitre.org/detectionstrategies/DET0107#AN0299)   |

Detection of spearphishing links through mail logs and browser activity. Behavior includes email with suspicious URLs → user click recorded in mail/web proxy logs → shell or interpreter launched from browser process.

|  [AN0300](https://attack.mitre.org/detectionstrategies/DET0107#AN0300)   |

Correlation of Mail.app logs with Safari/Chrome activity. Suspicious behavior includes email links → Safari/Chrome accessing newly registered or lookalike domains → osascript or Terminal spawned unexpectedly.

|  [AN0301](https://attack.mitre.org/detectionstrategies/DET0107#AN0301)   |

Detection of OAuth consent phishing or malicious login attempts initiated through spearphishing links. Behavior chain includes inbound email with OAuth URL → consent page visited → unusual token grants logged in IdP logs.

## References

-    [CISA. (2019, September 27). Security Tip (ST05-016): Understanding Internationalized Domain Names. Retrieved October 20, 2020.](https://us-cert.cisa.gov/ncas/tips/ST05-016)
-    [Nick Simonian. (2023, May 22). Don't @ Me: URL Obfuscation Through Schema Abuse. Retrieved August 4, 2023.](https://www.mandiant.com/resources/blog/url-obfuscation-schema-abuse)
-    [Hacquebord, F.. (2017, April 25). Pawn Storm Abuses Open Authentication in Advanced Social Engineering Attacks. Retrieved October 4, 2019.](https://blog.trendmicro.com/trendlabs-security-intelligence/pawn-storm-abuses-open-authentication-advanced-social-engineering-attacks)
-    [Microsoft 365 Defender Threat Intelligence Team. (2021, June 14). Microsoft delivers comprehensive solution to battle rise in consent phishing emails. Retrieved December 13, 2021.](https://www.microsoft.com/security/blog/2021/07/14/microsoft-delivers-comprehensive-solution-to-battle-rise-in-consent-phishing-emails/)
-    [SecureWorks Counter Threat Unit Research Team. (2021, June 3). OAuth’S Device Code Flow Abused in Phishing Attacks. Retrieved March 19, 2024.](https://www.secureworks.com/blog/oauths-device-code-flow-abused-in-phishing-attacks)
-    [Jenko Hwong. (2021, August 10). New Phishing Attacks Exploiting OAuth Authorization Flows (Part 1). Retrieved March 19, 2024.](https://www.netskope.com/blog/new-phishing-attacks-exploiting-oauth-authorization-flows-part-1)
-    [Optiv. (2021, August 17). Microsoft 365 OAuth Device Code Flow and Phishing. Retrieved March 19, 2024.](https://www.optiv.com/insights/source-zero/blog/microsoft-365-oauth-device-code-flow-and-phishing)
-    [Dr. Nestori Syynimaa. (2018, October 25). AADInternals. Retrieved February 18, 2022.](https://o365blog.com/aadinternals)
-    [Cybersecurity and Infrastructure Security Agency. (2021, February 21). AppleJeus: Analysis of North Korea’s Cryptocurrency Malware. Retrieved March 1, 2021.](https://us-cert.cisa.gov/ncas/alerts/aa21-048a)
-    [Global Research & Analysis Team, Kaspersky. (2024, August 19). BlindEagle flying high in Latin America. Retrieved April 16, 2026.](https://securelist.com/blindeagle-apt/113414/)
-    [Check Point Research. (2025, March 10). Blind Eagle: …And Justice for All. Retrieved April 16, 2026.](https://research.checkpoint.com/2025/blind-eagle-and-justice-for-all/)
-    [Insikt Group. (2025, August 26). TAG-144’s Persistent Grip on South American Organizations. Retrieved April 16, 2026.](https://assets.recordedfuture.com/insikt-report-pdfs/2025/cta-2025-0826.pdf)
-    [Mandiant. (n.d.). APT1 Exposing One of China’s Cyber Espionage Units. Retrieved July 18, 2016.](https://www.fireeye.com/content/dam/fireeye-www/services/pdfs/mandiant-apt1-report.pdf)
-    [Dunwoody, M. and Carr, N.. (2016, September 27). No Easy Breach DerbyCon 2016. Retrieved September 12, 2024.](https://www.slideshare.net/slideshow/no-easy-breach-derby-con-2016/66447908)
-    [Microsoft Threat Intelligence Center (MSTIC). (2021, May 27). New sophisticated email-based attack from NOBELIUM. Retrieved May 28, 2021.](https://www.microsoft.com/security/blog/2021/05/27/new-sophisticated-email-based-attack-from-nobelium/)
-    [Secureworks CTU. (2021, May 28). USAID-Themed Phishing Campaign Leverages U.S. Elections Lure. Retrieved February 24, 2022.](https://www.secureworks.com/blog/usaid-themed-phishing-campaign-leverages-us-elections-lure)
-    [Eng, E., Caselden, D.. (2015, June 23). Operation Clandestine Wolf – Adobe Flash Zero-Day in APT3 Phishing Campaign. Retrieved January 14, 2016.](https://www.fireeye.com/blog/threat-research/2015/06/operation-clandestine-wolf-adobe-flash-zero-day.html)
-    [Foltýn, T. (2018, March 13). OceanLotus ships new backdoor using old tricks. Retrieved May 22, 2018.](https://www.welivesecurity.com/2018/03/13/oceanlotus-ships-new-backdoor/)
-    [Dahan, A. (2017, May 24). OPERATION COBALT KITTY: A LARGE-SCALE APT IN ASIA CARRIED OUT BY THE OCEANLOTUS GROUP. Retrieved November 5, 2018.](https://www.cybereason.com/blog/operation-cobalt-kitty-apt)
-    [Henderson, S., et al. (2020, April 22). Vietnamese Threat Actors APT32 Targeting Wuhan Government and Chinese Ministry of Emergency Management in Latest Example of COVID-19 Related Espionage. Retrieved April 28, 2020.](https://www.fireeye.com/blog/threat-research/2020/04/apt32-targeting-chinese-government-in-covid-19-related-espionage.html)
-    [Adair, S. and Lancaster, T. (2020, November 6). OceanLotus: Extending Cyber Espionage Operations Through Fake Websites. Retrieved November 20, 2020.](https://www.volexity.com/blog/2020/11/06/oceanlotus-extending-cyber-espionage-operations-through-fake-websites/)
-    [Amnesty International. (2021, February 24). Vietnamese activists targeted by notorious hacking group. Retrieved March 1, 2021.](https://www.amnestyusa.org/wp-content/uploads/2021/02/Click-and-Bait_Vietnamese-Human-Rights-Defenders-Targeted-with-Spyware-Attacks.pdf)
-    [O'Leary, J., et al. (2017, September 20). Insights into Iranian Cyber Espionage: APT33 Targets Aerospace and Energy Sectors and has Ties to Destructive Malware. Retrieved February 15, 2018.](https://www.fireeye.com/blog/threat-research/2017/09/apt33-insights-into-iranian-cyber-espionage.html)
-    [Security Response attack Investigation Team. (2019, March 27). Elfin: Relentless Espionage Group Targets Multiple Organizations in Saudi Arabia and U.S.. Retrieved April 10, 2019.](https://www.symantec.com/blogs/threat-intelligence/elfin-apt33-espionage)
-    [Microsoft Threat Intelligence. (2026, April 16). Dissecting Sapphire Sleet’s macOS intrusion from lure to compromise. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/04/16/dissecting-sapphire-sleets-macos-intrusion-from-lure-to-compromise/)
-    [Hawley et al. (2019, January 29). APT39: An Iranian Cyber Espionage Group Focused on Personal Information. Retrieved February 19, 2019.](https://www.fireeye.com/blog/threat-research/2019/01/apt39-iranian-cyber-espionage-group-focused-on-personal-information.html)
-    [FBI. (2020, September 17). Indicators of Compromise Associated with Rana Intelligence Computing, also known as Advanced Persistent Threat 39, Chafer, Cadelspy, Remexi, and ITG07. Retrieved December 10, 2020.](https://www.iranwatch.org/sites/default/files/public-intelligence-alert.pdf)
-    [Mandiant. (n.d.). APT42: Crooked Charms, Cons and Compromises. Retrieved October 9, 2024.](https://services.google.com/fh/files/misc/apt42-crooked-charms-cons-and-compromises.pdf)
-    [Rozmann, O., et al. (2024, May 1). Uncharmed: Untangling Iran's APT42 Operations. Retrieved October 9, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/untangling-iran-apt42-operations)
-    [Google Threat Analysis Group. (2024, August 14). Iranian backed group steps up phishing campaigns against Israel, U.S.. Retrieved October 9, 2024.](https://blog.google/threat-analysis-group/iranian-backed-group-steps-up-phishing-campaigns-against-israel-us/)
-    [Cybereason Nocturnus. (2020, July 16). A BAZAR OF TRICKS: FOLLOWING TEAM9’S DEVELOPMENT CYCLES. Retrieved November 18, 2020.](https://www.cybereason.com/blog/a-bazar-of-tricks-following-team9s-development-cycles)
-    [Sadique, M. and Singh, A. (2020, September 29). Spear Phishing Campaign Delivers Buer and Bazar Malware. Retrieved November 19, 2020.](https://www.zscaler.com/blogs/research/spear-phishing-campaign-delivers-buer-and-bazar-malware)
-    [Podlosky, A., Hanel, A. et al. (2020, October 16). WIZARD SPIDER Update: Resilient, Reactive and Resolute. Retrieved June 15, 2021.](https://www.crowdstrike.com/blog/wizard-spider-adversary-update/)
-    [Bermejo, L., et al. (2017, June 22). Following the Trail of BlackTech’s Cyber Espionage Campaigns. Retrieved May 5, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)
-    [Merriman, K. and Trouerbach, P. (2022, April 28). This isn't Optimus Prime's Bumblebee but it's Still Transforming. Retrieved August 22, 2022.](https://www.proofpoint.com/us/blog/threat-insight/bumblebee-is-still-transforming)
-    [Cybereason. (2022, August 17). Bumblebee Loader – The High Road to Enterprise Domain Control. Retrieved August 29, 2022.](https://www.cybereason.com/blog/threat-analysis-report-bumblebee-loader-the-high-road-to-enterprise-domain-control)
-    [N. Baisini. (2022, July 13). Transparent Tribe begins targeting education sector in latest campaign. Retrieved September 22, 2022.](https://blog.talosintelligence.com/2022/07/transparent-tribe-targets-education.html)
-    [Dunwoody, M., et al. (2018, November 19). Not So Cozy: An Uncomfortable Examination of a Suspected APT29 Phishing Campaign. Retrieved November 27, 2018.](https://www.fireeye.com/blog/threat-research/2018/11/not-so-cozy-an-uncomfortable-examination-of-a-suspected-apt29-phishing-campaign.html)
-    [Microsoft Defender Research Team. (2018, December 3). Analysis of cyberattack on U.S. think tanks, non-profits, public sector by unidentified attackers. Retrieved April 15, 2019.](https://www.microsoft.com/security/blog/2018/12/03/analysis-of-cyberattack-on-u-s-think-tanks-non-profits-public-sector-by-unidentified-attackers/)
-    [Svajcer, V. (2018, July 31). Multiple Cobalt Personality Disorder. Retrieved September 5, 2018.](https://blog.talosintelligence.com/2018/07/multiple-cobalt-personality-disorder.html)
-    [CTU. (2018, September 27). Cybercriminals Increasingly Trying to Ensnare the Big Financial Fish. Retrieved September 20, 2021.](https://www.secureworks.com/blog/cybercriminals-increasingly-trying-to-ensnare-the-big-financial-fish)
-    [Lunghi, D. (2021, August 17). Confucius Uses Pegasus Spyware-related Lures to Target Pakistani Military. Retrieved December 26, 2021.](https://www.trendmicro.com/en_us/research/21/h/confucius-uses-pegasus-spyware-related-lures-to-target-pakistani.html)
-    [Ernesto Fernández Provecho, Pham Duy Phuc, Ciana Driscoll & Vinoo Thomas. (2023, November 21). The Continued Evolution of the DarkGate Malware-as-a-Service. Retrieved February 9, 2024.](https://www.trellix.com/blogs/research/the-continued-evolution-of-the-darkgate-malware-as-a-service/)
-    [Chen, J., et al. (2022). Delving Deep: An Analysis of Earth Lusca’s Operations. Retrieved July 1, 2022.](https://www.trendmicro.com/content/dam/trendmicro/global/en/research/22/a/earth-lusca-employs-sophisticated-infrastructure-varied-tools-and-techniques/technical-brief-delving-deep-an-analysis-of-earth-lusca-operations.pdf)
-    [O'Gorman, G., and McDonald, G.. (2012, September 6). The Elderwood Project. Retrieved November 17, 2024.](https://web.archive.org/web/20190717233006/http:/www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/the-elderwood-project.pdf)
-    [Clayton, M.. (2012, September 14). Stealing US business secrets: Experts ID two huge cyber 'gangs' in China. Retrieved February 15, 2018.](https://www.csmonitor.com/USA/2012/0914/Stealing-US-business-secrets-Experts-ID-two-huge-cyber-gangs-in-China)
-    [Salvio, J.. (2014, June 27). New Banking Malware Uses Network Sniffing for Data Theft. Retrieved March 25, 2019.](https://blog.trendmicro.com/trendlabs-security-intelligence/new-banking-malware-uses-network-sniffing-for-data-theft/)
-    [Shulmin, A. . (2015, April 9). The Banking Trojan Emotet: Detailed Analysis. Retrieved March 25, 2019.](https://securelist.com/the-banking-trojan-emotet-detailed-analysis/69560/)
-    [CIS. (2017, April 28). Emotet Changes TTPs and Arrives in United States. Retrieved January 17, 2019.](https://www.cisecurity.org/blog/emotet-changes-ttp-and-arrives-in-united-states/)
-    [Smith, A.. (2017, December 22). Protect your network from Emotet Trojan with Malwarebytes Endpoint Security. Retrieved January 17, 2019.](https://support.malwarebytes.com/docs/DOC-2295)
-    [Symantec. (2018, July 18). The Evolution of Emotet: From Banking Trojan to Threat Distributor. Retrieved March 25, 2019.](https://www.symantec.com/blogs/threat-intelligence/evolution-emotet-trojan-distributor)
-    [US-CERT. (2018, July 20). Alert (TA18-201A) Emotet Malware. Retrieved March 25, 2019.](https://www.us-cert.gov/ncas/alerts/TA18-201A)
-    [Brumaghin, E.. (2019, January 15). Emotet re-emerges after the holidays. Retrieved March 25, 2019.](https://blog.talosintelligence.com/2019/01/return-of-emotet.html)
-    [Özarslan, S. (2018, December 21). The Christmas Card you never wanted - A new wave of Emotet is back to wreak havoc. Retrieved March 25, 2019.](https://www.picussecurity.com/blog/the-christmas-card-you-never-wanted-a-new-wave-of-emotet-is-back-to-wreak-havoc.html)
-    [Porolli, M. (2020, July 9). More evil: A deep look at Evilnum and its toolset. Retrieved January 22, 2021.](https://www.welivesecurity.com/2020/07/09/more-evil-deep-look-evilnum-toolset/)
-    [Stolyarov, V. (2022, March 17). Exposing initial access broker with ties to Conti. Retrieved August 18, 2022.](https://blog.google/threat-analysis-group/exposing-initial-access-broker-ties-conti/)
-    [Vengerik, B. et al.. (2014, December 5). Hacking the Street? FIN4 Likely Playing the Market. Retrieved December 17, 2018.](https://web.archive.org/web/20220313062515/https://www.mandiant.com/sites/default/files/2021-09/rpt-fin4.pdf)
-    [Vengerik, B. & Dennesen, K.. (2014, December 5). Hacking the Street? FIN4 Likely Playing the Market. Retrieved January 15, 2019.](https://www2.fireeye.com/WBNR-14Q4NAMFIN4.html)
-    [Loui, E. and Reynolds, J. (2021, August 30). CARBON SPIDER Embraces Big Game Hunting, Part 1. Retrieved September 20, 2021.](https://www.crowdstrike.com/blog/carbon-spider-embraces-big-game-hunting-part-1/)
-    [The BlackBerry Research and Intelligence Team. (2024, April 17). Threat Group FIN7 Targets the U.S. Automotive Industry. Retrieved May 1, 2025.](https://blogs.blackberry.com/en/2024/04/fin7-targets-the-united-states-automotive-industry)
-    [Elovitz, S. & Ahl, I. (2016, August 18). Know Your Enemy: New Financially-Motivated & Spear-Phishing Group. Retrieved February 26, 2018.](https://www2.fireeye.com/WBNR-Know-Your-Enemy-UNC622-Spear-Phishing.html)
-    [Abramov, D. (2020, April 13). Grandoreiro Malware Now Targeting Banks in Spain. Retrieved November 12, 2020.](https://securityintelligence.com/posts/grandoreiro-malware-now-targeting-banks-in-spain/)
-    [ESET. (2020, April 28). Grandoreiro: How engorged can an EXE get?. Retrieved November 13, 2020.](https://www.welivesecurity.com/2020/04/28/grandoreiro-how-engorged-can-exe-get/)
-    [Duncan, B. (2020, April 3). GuLoader: Malspam Campaign Installing NetWire RAT. Retrieved January 7, 2021.](https://unit42.paloaltonetworks.com/guloader-installing-netwire-rat/)
-    [Tom Spring. (2017, January 11). Spammers Revive Hancitor Downloader Campaigns. Retrieved August 13, 2020.](https://threatpost.com/spammers-revive-hancitor-downloader-campaigns/123011/)
-    [Wan, Y. (2025, March 3). Havoc: SharePoint with Microsoft Graph API turns into FUD C2. Retrieved August 4, 2025.](https://www.fortinet.com/blog/threat-research/havoc-sharepoint-with-microsoft-graph-api-turns-into-fud-c2)
-    [GReAT. (2020, July 14). The Tetrade: Brazilian banking malware goes global. Retrieved November 9, 2020.](https://securelist.com/the-tetrade-brazilian-banking-malware/97779/)
-    [Artic Wolf Labs. (2026, April 24). Token Bingo: Don’t Let Your Code be the Winner. Retrieved July 30, 2026.](https://arcticwolf.com/resources/blog/token-bingo-dont-let-your-code-be-the-winner/)
-    [Alyac. (2019, April 3). Kimsuky Organization Steals Operation Stealth Power. Retrieved August 13, 2019.](https://blog.alyac.co.kr/2234)
-    [ASERT team. (2018, December 5). STOLEN PENCIL Campaign Targets Academia. Retrieved February 5, 2019.](https://asert.arbornetworks.com/stolen-pencil-campaign-targets-academia/)
-    [KISA. (2021). Phishing Target Reconnaissance and Attack Resource Analysis Operation Muzabi. Retrieved March 8, 2024.](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)
-    [Jazi, H. (2021, February). LazyScripter: From Empire to double RAT. Retrieved November 17, 2024.](https://web.archive.org/web/20211003035156/https://www.malwarebytes.com/resources/files/2021/02/lazyscripter.pdf)
-    [Proofpoint Threat Research and Team Cymru S2 Threat Research. (2024, April 4). Latrodectus: This Spider Bytes Like Ice . Retrieved May 31, 2024.](https://www.proofpoint.com/us/blog/threat-insight/latrodectus-spider-bytes-ice)
-    [Abrams, L. (2024, April 30). New Latrodectus malware attacks use Microsoft, Cloudflare themes. Retrieved September 13, 2024.](https://www.bleepingcomputer.com/news/security/new-latrodectus-malware-attacks-use-microsoft-cloudflare-themes/)
-    [Vyacheslav Kopeytsev and Seongsu Park. (2021, February 25). Lazarus targets defense industry with ThreatNeedle. Retrieved October 27, 2021.](https://securelist.com/lazarus-threatneedle/100803/)
-    [Axel F, Pierre T. (2017, October 16). Leviathan: Espionage actor spearphishes maritime and defense targets. Retrieved February 15, 2018.](https://www.proofpoint.com/us/threat-insight/post/leviathan-espionage-actor-spearphishes-maritime-and-defense-targets)
-    [CISA. (2021, July 19). (AA21-200A) Joint Cybersecurity Advisory – Tactics, Techniques, and Procedures of Indicted APT40 Actors Associated with China’s MSS Hainan State Security Department. Retrieved August 12, 2021.](https://us-cert.cisa.gov/ncas/alerts/aa21-200a)

-    [Lechtik, M, and etl. (2021, July 14). LuminousMoth APT: Sweeping attacks for the chosen few. Retrieved October 20, 2022.](https://securelist.com/apt-luminousmoth/103332/)
-    [Cybereaon Security Services Team. (n.d.). Your Data Is Under New Lummanagement: The Rise of LummaStealer. Retrieved March 22, 2025.](https://www.cybereason.com/blog/threat-analysis-rise-of-lummastealer)
-    [The Cylance Threat Research Team. (2017, March 22). El Machete's Malware Attacks Cut Through LATAM. Retrieved September 13, 2019.](https://threatvector.cylance.com/en_us/home/el-machete-malware-attacks-cut-through-latam.html)
-    [ESET. (2019, July). MACHETE JUST GOT SHARPER Venezuelan government institutions under attack. Retrieved September 13, 2019.](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)
-    [Counter Threat Unit Research Team. (2017, February 15). Iranian PupyRAT Bites Middle Eastern Organizations. Retrieved December 27, 2017.](https://www.secureworks.com/blog/iranian-pupyrat-bites-middle-eastern-organizations)
-    [ClearSky Research Team. (2020, August 1). The Kittens Are Back in Town 3 - Charming Kitten Campaign Evolved and Deploying Spear-Phishing link by WhatsApp. Retrieved April 21, 2021.](https://www.clearskysec.com/wp-content/uploads/2020/08/The-Kittens-are-Back-in-Town-3.pdf)
-    [Certfa Labs. (2021, January 8). Charming Kitten’s Christmas Gift. Retrieved May 3, 2021.](https://blog.certfa.com/posts/charming-kitten-christmas-gift/)
-    [MSTIC. (2021, November 16). Evolving trends in Iranian threat actor activity – MSTIC presentation at CyberWarCon 2021. Retrieved January 12, 2023.](https://www.microsoft.com/en-us/security/blog/2021/11/16/evolving-trends-in-iranian-threat-actor-activity-mstic-presentation-at-cyberwarcon-2021)
-    [Hiroaki, H. (2025, April 30). Earth Kasha Updates TTPs in Latest Campaign Targeting Taiwan and Japan. Retrieved April 17, 2026.](https://www.trendmicro.com/en_us/research/25/d/earth-kasha-updates-ttps.html)
-    [SCILabs. (2021, December 23). Cyber Threat Profile Malteiro. Retrieved March 13, 2024.](https://blog.scilabs.mx/en/cyber-threat-profile-malteiro/)
-    [Yonathan Klijnsma. (2016, May 17). Mofang: A politically motivated information stealing adversary. Retrieved May 12, 2020.](https://foxitsecurity.files.wordpress.com/2016/06/fox-it_mofang_threatreport_tlp-white.pdf)
-    [GReAT. (2019, April 10). Gaza Cybergang Group1, operation SneakyPastes. Retrieved May 13, 2020.](https://securelist.com/gaza-cybergang-group1-operation-sneakypastes/90068/)
-    [Mele, G. et al. (2021, February 10). Probable Iranian Cyber Actors, Static Kitten, Conducting Cyberespionage Campaign Targeting UAE and Kuwait Government Agencies. Retrieved March 17, 2021.](https://www.anomali.com/blog/probable-iranian-cyber-actors-static-kitten-conducting-cyberespionage-campaign-targeting-uae-and-kuwait-government-agencies)
-    [Peretz, A. and Theck, E. (2021, March 5). Earth Vetala – MuddyWater Continues to Target Organizations in the Middle East. Retrieved March 18, 2021.](https://www.trendmicro.com/en_us/research/21/c/earth-vetala---muddywater-continues-to-target-organizations-in-t.html)
-    [Miller, J. et al. (2024, March 21). Security Brief: TA450 Uses Embedded Links in PDF Attachments in Latest Campaign. Retrieved March 27, 2024.](https://www.proofpoint.com/us/blog/threat-insight/security-brief-ta450-uses-embedded-links-pdf-attachments-latest-campaign)
-    [Golo Muhr, Joshua Chung. (2025, June 23). Hive0154 aka Mustang Panda shifts focus on Tibetan community to deploy Pubload backdoor. Retrieved August 4, 2025.](https://www.ibm.com/think/x-force/hive0154-mustang-panda-shifts-focus-tibetan-community-deploy-pubload-backdoor)
-    [Golo Muhr, Joshua Chung. (2025, May 15). Hive0154 targeting US, Philippines, Pakistan and Taiwan in suspected espionage campaign. Retrieved August 4, 2025.](https://www.ibm.com/think/x-force/hive0154-targeting-us-philippines-pakistan-taiwan)
-    [Roccia, T., Seret, T., Fokker, J. (2021, March 16). Technical Analysis of Operation Dianxun. Retrieved April 13, 2021.](https://www.mcafee.com/enterprise/en-us/assets/reports/rp-operation-dianxun.pdf)
-    [Nick Dai, Vickie Su, Sunny Lu. (2022, November 18). Earth Preta Spear-Phishing Governments Worldwide. Retrieved August 4, 2025.](https://www.trendmicro.com/en_us/research/22/k/earth-preta-spear-phishing-governments-worldwide.html)
-    [Andrew Northern. (2022, November 22). SocGholish, a very real threat from a very fake update. Retrieved February 13, 2024.](https://www.proofpoint.com/us/blog/threat-insight/part-1-socgholish-very-real-threat-very-fake-update)
-    [McAfee® Foundstone® Professional Services and McAfee Labs™. (2011, February 10). Global Energy Cyberattacks: “Night Dragon”. Retrieved February 19, 2018.](https://scadahacker.com/library/Documents/Cyber_Events/McAfee%20-%20Night%20Dragon%20-%20Global%20Energy%20Cyberattacks.pdf)
-    [Check Point. (2026, May 22). Fast and Furious – Nimbus Manticore Operations During the Iranian Conflict. Retrieved September 23, 2026.](https://research.checkpoint.com/2026/fast-and-furious-nimbus-manticore-operations-during-the-iranian-conflict/)
-    [Check Point. (2025, September 22). Nimbus Manticore Deploys New Malware Targeting Europe. Retrieved September 24, 2026.](https://research.checkpoint.com/2025/nimbus-manticore-deploys-new-malware-targeting-europe/)
-    [Rozmann, O., et al. (2024, February 27). When Cats Fly: Suspected Iranian Threat Actor UNC1549 Targets Israeli and Middle East Aerospace and Defense Sectors. Retrieved September 28, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/suspected-iranian-unc1549-targets-israel-middle-east)
-    [El-Banna, M., et al. (2025, November 17). Frontline Intelligence: Analysis of UNC1549 TTPs, Custom Tools, and Malware Targeting the Aerospace and Defense Ecosystem. Retrieved September 28, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)
-    [Lee, B., Falcone, R. (2018, February 23). OopsIE! OilRig Uses ThreeDollars to Deliver New Trojan. Retrieved July 16, 2018.](https://researchcenter.paloaltonetworks.com/2018/02/unit42-oopsie-oilrig-uses-threedollars-deliver-new-trojan/)
-    [ClearSky Cybersecurity. (2017, January 5). Iranian Threat Agent OilRig Delivers Digitally Signed Malware, Impersonates University of Oxford. Retrieved May 3, 2017.](http://www.clearskysec.com/oilrig/)
-    [Hiroaki, H. (2024, November 26). Guess Who’s Back - The Return of ANEL in the Recent Earth Kasha Spear-phishing Campaign in 2024. Retrieved April 17, 2026.](https://www.trendmicro.com/en_us/research/24/k/return-of-anel-in-the-recent-earth-kasha-spearphishing-campaign.html)
-    [ClearSky Research Team. (2020, August 13). Operation 'Dream Job' Widespread North Korean Espionage Campaign. Retrieved December 20, 2021.](https://www.clearskysec.com/wp-content/uploads/2020/08/Dream-Job-Campaign.pdf)
-    [Breitenbacher, D and Osis, K. (2020, June 17). OPERATION IN(TER)CEPTION: Targeted Attacks Against European Aerospace and Military Companies. Retrieved December 20, 2021.](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_Operation_Interception.pdf)
-    [Gross, J. (2016, February 23). Operation Dust Storm. Retrieved December 22, 2021.](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)
-    [M. Porolli. (2021, January 21). Operation Spalax: Targeted malware attacks in Colombia. Retrieved September 16, 2022.](https://www.welivesecurity.com/2021/01/12/operation-spalax-targeted-malware-attacks-colombia/)
-    [Unit 42. (2022, February 25). Spear Phishing Attacks Target Organizations in Ukraine, Payloads Include the Document Stealer OutSteel and the Downloader SaintBot. Retrieved June 9, 2022.](https://unit42.paloaltonetworks.com/ukraine-targeted-outsteel-saintbot/)
-    [Hamada, J.. (2016, July 25). Patchwork cyberespionage group expands targets from governments to wide range of industries. Retrieved August 17, 2016.](http://www.symantec.com/connect/blogs/patchwork-cyberespionage-group-expands-targets-governments-wide-range-industries)
-    [Lunghi, D., et al. (2017, December). Untangling the Patchwork Cyberespionage Group. Retrieved July 10, 2018.](https://documents.trendmicro.com/assets/tech-brief-untangling-the-patchwork-cyberespionage-group.pdf)
-    [Hinchliffe, A. and Falcone, R. (2020, May 11). Updated BackConfig Malware Targeting Government and Military Organizations in South Asia. Retrieved June 17, 2020.](https://unit42.paloaltonetworks.com/updated-backconfig-malware-targeting-government-and-military-organizations/)
-    [Daniel Stepanic & Salim Bitam. (2024, February 23). PIKABOT, I choose you!. Retrieved July 12, 2024.](https://www.elastic.co/security-labs/pikabot-i-choose-you)
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
-    [Scott W. Brady. (2020, October 15). United States vs. Yuriy Sergeyevich Andrienko et al.. Retrieved November 25, 2020.](https://www.justice.gov/opa/press-release/file/1328521/download)
-    [Hegel, T. (2021, January 13). A Global Perspective of the SideWinder APT. Retrieved January 27, 2021.](https://cdn-cybersecurity.att.com/docs/global-perspective-of-the-sidewinder-apt.pdf)
-    [Cyble. (2020, September 26). SideWinder APT Targets with futuristic Tactics and Techniques. Retrieved January 29, 2021.](https://cybleinc.com/2020/09/26/sidewinder-apt-targets-with-futuristic-tactics-and-techniques/)
-    [Jornet, A. (2021, December 23). Snip3, an investigation into malware. Retrieved September 19, 2023.](https://telefonicatech.com/blog/snip3-investigacion-malware)
-    [Kumar, A., Stone-Gross, Brett. (2021, September 28). Squirrelwaffle: New Loader Delivering Cobalt Strike. Retrieved August 9, 2022.](https://www.zscaler.com/blogs/security-research/squirrelwaffle-new-loader-delivering-cobalt-strike)
-    [Microsoft Threat Intelligence. (2024, May 15). Threat actors misusing Quick Assist in social engineering attacks leading to ransomware. Retrieved March 14, 2025.](https://www.microsoft.com/en-us/security/blog/2024/05/15/threat-actors-misusing-quick-assist-in-social-engineering-attacks-leading-to-ransomware/)
-    [Larson, S. and Wise, J. (2022, February 15). Charting TA2541's Flight. Retrieved September 12, 2023.](https://www.proofpoint.com/us/blog/threat-insight/charting-ta2541s-flight)
-    [Proofpoint Staff. (2017, September 27). Threat Actor Profile: TA505, From Dridex to GlobeImposter. Retrieved May 28, 2019.](https://www.proofpoint.com/us/threat-insight/post/threat-actor-profile-ta505-dridex-globeimposter)
-    [Schwarz, D. and Proofpoint Staff. (2019, January 9). ServHelper and FlawedGrace - New malware introduced by TA505. Retrieved May 28, 2019.](https://www.proofpoint.com/us/threat-insight/post/servhelper-and-flawedgrace-new-malware-introduced-ta505)
-    [Hiroaki, H. and Lu, L. (2019, June 12). Shifting Tactics: Breaking Down TA505 Group’s Use of HTML, RATs and Other Techniques in Latest Campaigns. Retrieved May 29, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/shifting-tactics-breaking-down-ta505-groups-use-of-html-rats-and-other-techniques-in-latest-campaigns/)
-    [Schwarz, D. et al. (2019, October 16). TA505 Distributes New SDBbot Remote Access Trojan with Get2 Downloader. Retrieved May 29, 2020.](https://www.proofpoint.com/us/threat-insight/post/ta505-distributes-new-sdbbot-remote-access-trojan-get2-downloader)
-    [Malhotra, A. (2021, March 2). ObliqueRAT returns with new campaign using hijacked websites. Retrieved September 2, 2021.](https://blog.talosintelligence.com/2021/02/obliquerat-new-campaign.html)
-    [Malhotra, A. et al. (2021, May 13). Transparent Tribe APT expands its Windows malware arsenal. Retrieved September 2, 2021.](https://blog.talosintelligence.com/2021/05/transparent-tribe-infra-and-targeting.html)
-    [Dahan, A. et al. (2019, December 11). DROPPING ANCHOR: FROM A TRICKBOT INFECTION TO THE DISCOVERY OF THE ANCHOR MALWARE. Retrieved September 10, 2020.](https://www.cybereason.com/blog/dropping-anchor-from-a-trickbot-infection-to-the-discovery-of-the-anchor-malware)
-    [ESET, et al. (2018, January). Diplomats in Eastern Europe bitten by a Turla mosquito. Retrieved July 3, 2018.](https://www.welivesecurity.com/wp-content/uploads/2018/01/ESET_Turla_Mosquito.pdf)
-    [Reaves, J. and Platt, J. (2020, June). Valak Malware and the Connection to Gozi Loader ConfCrew. Retrieved August 31, 2020.](https://assets.sentinelone.com/labs/sentinel-one-valak-i)
-    [Quorum Cyber. (n.d.). Malware Analysis Report Vidar - Stealerware. Retrieved August 28, 2026.](https://www.quorumcyber.com/wp-content/uploads/2023/01/Malware-Analysis-Vidar.pdf)
-    [Karim, T. (2018, August). TRAILS OF WINDSHIFT. Retrieved November 17, 2024.](https://www.scribd.com/document/661837258/WINDSHIFT-summit-archive-1554718868)
-    [Check Point. (2024, November 12). Hamas-affiliated Threat Actor WIRTE Continues its Middle East Operations and Moves to Disruptive Activity. Retrieved April 20, 2026.](https://research.checkpoint.com/2024/hamas-affiliated-threat-actor-expands-to-disruptive-activity/)
-    [DHS/CISA. (2020, October 28). Ransomware Activity Targeting the Healthcare and Public Health Sector. Retrieved October 28, 2020.](https://us-cert.cisa.gov/ncas/alerts/aa20-302a)
-    [The DFIR Report. (2020, November 5). Ryuk Speed Run, 2 Hours to Ransom. Retrieved November 6, 2020.](https://thedfirreport.com/2020/11/05/ryuk-speed-run-2-hours-to-ransom/)
-    [Burt, T. (2020, September 10). New cyberattacks targeting U.S. elections. Retrieved March 24, 2021.](https://blogs.microsoft.com/on-the-issues/2020/09/10/cyberattacks-us-elections-trump-biden/)
-    [Huntley, S. (2020, October 16). How We're Tackling Evolving Online Threats. Retrieved March 24, 2021.](https://blog.google/threat-analysis-group/how-were-tackling-evolving-online-threats/)
-    [Singh, S. and Antil, S. (2020, October 27). APT-31 Leverages COVID-19 Vaccine Theme and Abuses Legitimate Online Services. Retrieved March 24, 2021.](https://www.zscaler.com/blogs/security-research/apt-31-leverages-covid-19-vaccine-theme-and-abuses-legitimate-online)
-    [Microsoft. (2020, October 13). Anti-spoofing protection in EOP. Retrieved October 19, 2020.](https://docs.microsoft.com/en-us/microsoft-365/security/office-365-security/anti-spoofing-protection?view=o365-worldwide)
-    [Australian Cyber Security Centre. (2012, December). Mitigating Spoofed Emails Using Sender Policy Framework. Retrieved November 17, 2024.](https://web.archive.org/web/20210708014107/https://www.cyber.gov.au/sites/default/files/2019-03/spoof_email_sender_policy_framework.pdf)

×
