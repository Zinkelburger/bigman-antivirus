---
url: "https://attack.mitre.org/techniques/T1203/"
title: "Exploitation for Client Execution, Technique T1203 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:40:37Z"
via: "live"
sha256: "95981eee7d4809ba14cc4848c226f3e4803564f7d9cb086e1d3f2efa91e9f37e"
wayback: "https://web.archive.org/web/20261023090731/https://attack.mitre.org/techniques/T1203/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Exploitation for Client Execution, Technique T1203 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1203/>
> Snapshot: <https://web.archive.org/web/20261023090731/https://attack.mitre.org/techniques/T1203/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Exploitation for Client Execution

#  Exploitation for Client Execution

Adversaries may exploit software vulnerabilities in client applications to execute code. Vulnerabilities can exist in software due to unsecure coding practices that can lead to unanticipated behavior. Adversaries can take advantage of certain vulnerabilities through targeted exploitation for the purpose of arbitrary code execution. Oftentimes the most valuable exploits to an offensive toolkit are those that can be used to obtain code execution on a remote system because they can be used to gain access to that system. Users will expect to see files related to the applications they commonly used to do work, so they are a useful target for exploit research and development because of their high utility.

Several types exist:

### Browser-based Exploitation

Web browsers are a common target through [Drive-by Compromise](https://attack.mitre.org/techniques/T1189) and [Spearphishing Link](https://attack.mitre.org/techniques/T1566/002). Endpoint systems may be compromised through normal web browsing or from certain users being targeted by links in spearphishing emails to adversary controlled sites used to exploit the web browser. These often do not require an action by the user for the exploit to be executed.

### Office Applications

Common office and productivity applications such as Microsoft Office are also targeted through [Phishing](https://attack.mitre.org/techniques/T1566). Malicious files will be transmitted directly as attachments or through links to download them. These require the user to open the document or file for the exploit to run.

### Common Third-party Applications

Other applications that are commonly seen or are part of the software deployed in a target network may also be used for exploitation. Applications such as Adobe Reader and Flash, which are common in enterprise environments, have been routinely targeted by adversaries attempting to gain access to systems. Depending on the software and nature of the vulnerability, some may be exploited in the browser or require the user to open a file. For instance, some Flash exploits have been delivered as objects within Microsoft Office documents.

ID: T1203

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Execution](https://attack.mitre.org/tactics/TA0002)

ⓘ

Platforms: Linux, Windows, macOS

Version: 1.5

Created: 18 April 2018

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1203/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1203/)

## Procedure Examples

| ID  | Name  | Description

|  [C0057](https://attack.mitre.org/campaigns/C0057)   |  [3CX Supply Chain Attack](https://attack.mitre.org/campaigns/C0057)   |

During the [3CX Supply Chain Attack](https://attack.mitre.org/campaigns/C0057), [AppleJeus](https://attack.mitre.org/groups/G1049) leveraged the Chrome vulnerability, CVE-2022-0609, in combination with a [Drive-by Compromise](https://attack.mitre.org/techniques/T1189) website.[[1]](https://cloud.google.com/blog/topics/threat-intelligence/3cx-software-supply-chain-compromise/)

|  [G0018](https://attack.mitre.org/groups/G0018)   |  [admin@338](https://attack.mitre.org/groups/G0018)   |

[admin@338](https://attack.mitre.org/groups/G0018) has exploited client software vulnerabilities for execution, such as Microsoft Word CVE-2012-0158.[[2]](https://www.fireeye.com/blog/threat-research/2015/11/china-based-threat.html)

|  [S0331](https://attack.mitre.org/software/S0331)   |  [Agent Tesla](https://attack.mitre.org/software/S0331)   |

[Agent Tesla](https://attack.mitre.org/software/S0331) has exploited Office vulnerabilities such as CVE-2017-11882 and CVE-2017-8570 for execution during delivery.[[3]](https://labs.sentinelone.com/agent-tesla-old-rat-uses-new-tricks-to-stay-on-top/)

|  [G0138](https://attack.mitre.org/groups/G0138)   |  [Andariel](https://attack.mitre.org/groups/G0138)   |

[Andariel](https://attack.mitre.org/groups/G0138) has exploited numerous ActiveX vulnerabilities, including zero-days.[[4]](https://fsiceat.tistory.com/2)[[5]](http://www.issuemakerslab.com/research3/)[[6]](https://www.trendmicro.com/en_us/research/18/g/new-andariel-reconnaissance-tactics-hint-at-next-targets.html)

|  [G1007](https://attack.mitre.org/groups/G1007)   |  [Aoqin Dragon](https://attack.mitre.org/groups/G1007)   |

[Aoqin Dragon](https://attack.mitre.org/groups/G1007) has exploited CVE-2012-0158 and CVE-2010-3333 for execution against targeted systems.[[7]](https://www.sentinelone.com/labs/aoqin-dragon-newly-discovered-chinese-linked-apt-has-been-quietly-spying-on-organizations-for-10-years/)

|  [G0005](https://attack.mitre.org/groups/G0005)   |  [APT12](https://attack.mitre.org/groups/G0005)   |

[APT12](https://attack.mitre.org/groups/G0005) has exploited multiple vulnerabilities for execution, including Microsoft Office vulnerabilities (CVE-2009-3129, CVE-2012-0158) and vulnerabilities in Adobe Reader and Flash (CVE-2009-4324, CVE-2009-0927, CVE-2011-0609, CVE-2011-0611).[[8]](https://www.fireeye.com/blog/threat-research/2014/09/darwins-favorite-apt-group-2.html)[[9]](https://www.trendmicro.de/cloud-content/us/pdfs/security-intelligence/white-papers/wp_ixeshe.pdf)

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[APT28](https://attack.mitre.org/groups/G0007) has exploited Microsoft Office vulnerability CVE-2017-0262 for execution.[[10]](https://securelist.com/a-slice-of-2017-sofacy-activity/83930/)

|  [G0016](https://attack.mitre.org/groups/G0016)   |  [APT29](https://attack.mitre.org/groups/G0016)   |

[APT29](https://attack.mitre.org/groups/G0016) has used multiple software exploits for common client software, like Microsoft Word, Exchange, and Adobe Reader, to gain code execution.[[11]](https://www.f-secure.com/documents/996508/1030745/dukes_whitepaper.pdf)[[12]](https://www.ncsc.gov.uk/files/Advisory-further-TTPs-associated-with-SVR-cyber-actors.pdf)[[13]](https://www.microsoft.com/security/blog/2021/05/27/new-sophisticated-email-based-attack-from-nobelium/)

|  [G0022](https://attack.mitre.org/groups/G0022)   |  [APT3](https://attack.mitre.org/groups/G0022)   |

[APT3](https://attack.mitre.org/groups/G0022) has exploited the Adobe Flash Player vulnerability CVE-2015-3113 and Internet Explorer vulnerability CVE-2014-1776.[[14]](https://www.fireeye.com/blog/threat-research/2015/06/operation-clandestine-wolf-adobe-flash-zero-day.html)[[15]](https://www.fireeye.com/blog/threat-research/2014/04/new-zero-day-exploit-targeting-internet-explorer-versions-9-through-11-identified-in-targeted-attacks.html)

|  [G0050](https://attack.mitre.org/groups/G0050)   |  [APT32](https://attack.mitre.org/groups/G0050)   |

[APT32](https://attack.mitre.org/groups/G0050) has used RTF document that includes an exploit to execute malicious code. (CVE-2017-11882)[[16]](https://www.welivesecurity.com/2019/03/20/fake-or-fake-keeping-up-with-oceanlotus-decoys/)

|  [G0064](https://attack.mitre.org/groups/G0064)   |  [APT33](https://attack.mitre.org/groups/G0064)   |

[APT33](https://attack.mitre.org/groups/G0064) has attempted to exploit a known vulnerability in WinRAR (CVE-2018-20250), and attempted to gain remote code execution via a security bypass vulnerability (CVE-2017-11774).[[17]](https://www.symantec.com/blogs/threat-intelligence/elfin-apt33-espionage)[[18]](https://www.microsoft.com/security/blog/2020/06/18/inside-microsoft-threat-protection-mapping-attack-chains-from-cloud-to-endpoint/)

|  [G0067](https://attack.mitre.org/groups/G0067)   |  [APT37](https://attack.mitre.org/groups/G0067)   |

[APT37](https://attack.mitre.org/groups/G0067) has used exploits for Flash Player (CVE-2016-4117, CVE-2018-4878), Word (CVE-2017-0199), Internet Explorer (CVE-2020-1380 and CVE-2020-26411), and Microsoft Edge (CVE-2021-26411) for execution.[[19]](https://securelist.com/operation-daybreak/75100/)[[20]](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)[[21]](https://blog.talosintelligence.com/2018/01/korea-in-crosshairs.html)[[22]](https://www.volexity.com/blog/2021/08/17/north-korean-apt-inkysquid-infects-victims-using-browser-exploits/)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) leveraged the follow exploits in their operations: CVE-2012-0158, CVE-2015-1641, CVE-2017-0199, CVE-2017-11882, and CVE-2019-3396.[[23]](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)

|  [G0001](https://attack.mitre.org/groups/G0001)   |  [Axiom](https://attack.mitre.org/groups/G0001)   |

[Axiom](https://attack.mitre.org/groups/G0001) has used exploits for multiple vulnerabilities including CVE-2014-0322, CVE-2012-4792, CVE-2012-1889, and CVE-2013-3893.[[24]](http://blogs.cisco.com/security/talos/threat-spotlight-group-72)

|  [S0239](https://attack.mitre.org/software/S0239)   |  [Bankshot](https://attack.mitre.org/software/S0239)   |

[Bankshot](https://attack.mitre.org/software/S0239) leverages a known zero-day vulnerability in Adobe Flash to execute the implant into the victims’ machines.[[25]](https://securingtomorrow.mcafee.com/mcafee-labs/hidden-cobra-targets-turkish-financial-sector-new-bankshot-implant/)

|  [G1002](https://attack.mitre.org/groups/G1002)   |  [BITTER](https://attack.mitre.org/groups/G1002)   |

[BITTER](https://attack.mitre.org/groups/G1002) has exploited Microsoft Office vulnerabilities CVE-2012-0158, CVE-2017-11882, CVE-2018-0798, and CVE-2018-0802.[[26]](https://blog.talosintelligence.com/2022/05/bitter-apt-adds-bangladesh-to-their.html)[[27]](https://www.forcepoint.com/blog/x-labs/bitter-targeted-attack-against-pakistan)

|  [G0098](https://attack.mitre.org/groups/G0098)   |  [BlackTech](https://attack.mitre.org/groups/G0098)   |

[BlackTech](https://attack.mitre.org/groups/G0098) has exploited multiple vulnerabilities for execution, including Microsoft Office vulnerabilities CVE-2012-0158, CVE-2014-6352, CVE-2017-0199, and Adobe Flash CVE-2015-5119.[[28]](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)

|  [G0060](https://attack.mitre.org/groups/G0060)   |  [BRONZE BUTLER](https://attack.mitre.org/groups/G0060)   |

[BRONZE BUTLER](https://attack.mitre.org/groups/G0060) has exploited Microsoft Office vulnerabilities CVE-2014-4114, CVE-2018-0802, and CVE-2018-0798 for execution.[[29]](https://www.symantec.com/connect/blogs/tick-cyberespionage-group-zeros-japan)[[30]](https://documents.trendmicro.com/assets/pdf/Operation-ENDTRADE-TICK-s-Multi-Stage-Backdoors-for-Attacking-Industries-and-Stealing-Classified-Data.pdf)

|  [G0080](https://attack.mitre.org/groups/G0080)   |  [Cobalt Group](https://attack.mitre.org/groups/G0080)   |

[Cobalt Group](https://attack.mitre.org/groups/G0080) had exploited multiple vulnerabilities for execution, including Microsoft’s Equation Editor (CVE-2017-11882), an Internet Explorer vulnerability (CVE-2018-8174), CVE-2017-8570, CVE-2017-0199, and CVE-2017-8759.[[31]](https://blog.talosintelligence.com/2018/07/multiple-cobalt-personality-disorder.html)[[32]](https://www.ptsecurity.com/upload/corporate/ww-en/analytics/Cobalt-2017-eng.pdf)[[33]](https://www.ptsecurity.com/upload/corporate/ww-en/analytics/Cobalt-Snatch-eng.pdf)[[34]](https://www.proofpoint.com/us/threat-insight/post/microsoft-word-intruder-integrates-cve-2017-0199-utilized-cobalt-group-target)[[35]](https://web.archive.org/web/20190508170630/https://www.riskiq.com/blog/labs/cobalt-strike/)[[36]](https://web.archive.org/web/20190508170147/https://www.riskiq.com/blog/labs/cobalt-group-spear-phishing-russian-banks/)[[37]](https://crowdstrike.lookbookhq.com/global-threat-report-2018-web/cs-2018-global-threat-report)[[38]](https://blog.trendmicro.com/trendlabs-security-intelligence/cobalt-spam-runs-use-macros-cve-2017-8759-exploit/)

|  [S0154](https://attack.mitre.org/software/S0154)   |  [Cobalt Strike](https://attack.mitre.org/software/S0154)   |

[Cobalt Strike](https://attack.mitre.org/software/S0154) can exploit Oracle Java vulnerabilities for execution, including CVE-2011-3544, CVE-2013-2465, CVE-2012-4681, and CVE-2013-2460.[[39]](https://web.archive.org/web/20210219195905/https://talos-intelligence-site.s3.amazonaws.com/production/document_files/files/000/095/031/original/Talos_Cobalt_Strike.pdf)[[40]](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)

|  [G0142](https://attack.mitre.org/groups/G0142)   |  [Confucius](https://attack.mitre.org/groups/G0142)   |

[Confucius](https://attack.mitre.org/groups/G0142) has exploited Microsoft Office vulnerabilities, including CVE-2015-1641, CVE-2017-11882, and CVE-2018-0802.[[41]](https://www.uptycs.com/blog/confucius-apt-deploys-warzone-rat)[[42]](https://www.trendmicro.com/en_us/research/18/b/deciphering-confucius-cyberespionage-operations.html)

|  [G0012](https://attack.mitre.org/groups/G0012)   |  [Darkhotel](https://attack.mitre.org/groups/G0012)   |

[Darkhotel](https://attack.mitre.org/groups/G0012) has exploited Adobe Flash vulnerability CVE-2015-8651 for execution.[[43]](https://www.microsoft.com/security/blog/2016/06/09/reverse-engineering-dubnium-2/)

|  [S0243](https://attack.mitre.org/software/S0243)   |  [DealersChoice](https://attack.mitre.org/software/S0243)   |

[DealersChoice](https://attack.mitre.org/software/S0243) leverages vulnerable versions of Flash to perform execution.[[44]](https://researchcenter.paloaltonetworks.com/2018/03/unit42-sofacy-uses-dealerschoice-target-european-government-agency/)

|  [G0035](https://attack.mitre.org/groups/G0035)   |  [Dragonfly](https://attack.mitre.org/groups/G0035)   |

[Dragonfly](https://attack.mitre.org/groups/G0035) has exploited CVE-2011-0611 in Adobe Flash Player to gain execution on a targeted system.[[45]](https://vblocalhost.com/uploads/VB2021-Slowik.pdf)

|  [G0066](https://attack.mitre.org/groups/G0066)   |  [Elderwood](https://attack.mitre.org/groups/G0066)   |

[Elderwood](https://attack.mitre.org/groups/G0066) has used exploitation of endpoint software, including Microsoft Internet Explorer Adobe Flash vulnerabilities, to gain execution. They have also used zero-day exploits.[[46]](https://web.archive.org/web/20190717233006/http:/www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/the-elderwood-project.pdf)

|  [G1003](https://attack.mitre.org/groups/G1003)   |  [Ember Bear](https://attack.mitre.org/groups/G1003)   |

[Ember Bear](https://attack.mitre.org/groups/G1003) has used exploits to enable follow-on execution of frameworks such as Meterpreter.[[47]](https://www.cisa.gov/sites/default/files/2024-09/aa24-249a-russian-military-cyber-actors-target-us-and-global-critical-infrastructure.pdf)

|  [S0396](https://attack.mitre.org/software/S0396)   |  [EvilBunny](https://attack.mitre.org/software/S0396)   |

[EvilBunny](https://attack.mitre.org/software/S0396) has exploited CVE-2011-4369, a vulnerability in the PRC component in Adobe Reader.[[48]](https://web.archive.org/web/20150311013500/http://www.cyphort.com/evilbunny-malware-instrumented-lua/)

|  [G1011](https://attack.mitre.org/groups/G1011)   |  [EXOTIC LILY](https://attack.mitre.org/groups/G1011)   |

[EXOTIC LILY](https://attack.mitre.org/groups/G1011) has used malicious documents containing exploits for CVE-2021-40444 affecting Microsoft MSHTML.[[49]](https://blog.google/threat-analysis-group/exposing-initial-access-broker-ties-conti/)

|  [C0001](https://attack.mitre.org/campaigns/C0001)   |  [Frankenstein](https://attack.mitre.org/campaigns/C0001)   |

During [Frankenstein](https://attack.mitre.org/campaigns/C0001), the threat actors exploited CVE-2017-11882 to execute code on the victim's machine.[[50]](https://blog.talosintelligence.com/2019/06/frankenstein-campaign.html)

|  [S0391](https://attack.mitre.org/software/S0391)   |  [HAWKBALL](https://attack.mitre.org/software/S0391)   |

[HAWKBALL](https://attack.mitre.org/software/S0391) has exploited Microsoft Office vulnerabilities CVE-2017-11882 and CVE-2018-0802 to deliver the payload.[[51]](https://www.fireeye.com/blog/threat-research/2019/06/government-in-central-asia-targeted-with-hawkball-backdoor.html)

|  [G0126](https://attack.mitre.org/groups/G0126)   |  [Higaisa](https://attack.mitre.org/groups/G0126)   |

[Higaisa](https://attack.mitre.org/groups/G0126) has exploited CVE-2018-0798 for execution.[[52]](https://www.ptsecurity.com/ww-en/analytics/pt-esc-threat-intelligence/covid-19-and-new-year-greetings-the-higaisa-group/)

|  [G0100](https://attack.mitre.org/groups/G0100)   |  [Inception](https://attack.mitre.org/groups/G0100)   |

[Inception](https://attack.mitre.org/groups/G0100) has exploited CVE-2012-0158, CVE-2014-1761, CVE-2017-11882 and CVE-2018-0802 for execution.[[53]](https://securelist.com/recent-cloud-atlas-activity/92016/)[[54]](https://securelist.com/cloud-atlas-redoctober-apt-is-back-in-style/68083/)[[55]](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/inception-framework-hiding-behind-proxies)[[56]](https://unit42.paloaltonetworks.com/unit42-inception-attackers-target-europe-year-old-office-vulnerability/)

|  [S0260](https://attack.mitre.org/software/S0260)   |  [InvisiMole](https://attack.mitre.org/software/S0260)   |

[InvisiMole](https://attack.mitre.org/software/S0260) has installed legitimate but vulnerable Total Video Player software and wdigest.dll library drivers on compromised hosts to exploit stack overflow and input validation vulnerabilities for code execution.[[57]](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_InvisiMole.pdf)

|  [G0032](https://attack.mitre.org/groups/G0032)   |  [Lazarus Group](https://attack.mitre.org/groups/G0032)   |

[Lazarus Group](https://attack.mitre.org/groups/G0032) has exploited Adobe Flash vulnerability CVE-2018-4878 for execution.[[25]](https://securingtomorrow.mcafee.com/mcafee-labs/hidden-cobra-targets-turkish-financial-sector-new-bankshot-implant/)

|  [G0065](https://attack.mitre.org/groups/G0065)   |  [Leviathan](https://attack.mitre.org/groups/G0065)   |

[Leviathan](https://attack.mitre.org/groups/G0065) has exploited multiple Microsoft Office and .NET vulnerabilities for execution, including CVE-2017-0199, CVE-2017-8759, and CVE-2017-11882.[[58]](https://www.proofpoint.com/us/threat-insight/post/leviathan-espionage-actor-spearphishes-maritime-and-defense-targets)[[59]](https://www.fireeye.com/blog/threat-research/2018/03/suspected-chinese-espionage-group-targeting-maritime-and-engineering-industries.html)[[60]](https://us-cert.cisa.gov/ncas/alerts/aa21-200a)[[61]](https://web.archive.org/web/20210722005437/https://www.accenture.com/_acnmedia/PDF-96/Accenture-Security-MUDCARP.pdf)

|  [G0069](https://attack.mitre.org/groups/G0069)   |  [MuddyWater](https://attack.mitre.org/groups/G0069)   |

[MuddyWater](https://attack.mitre.org/groups/G0069) has exploited the Office vulnerability CVE-2017-0199 for execution.[[62]](https://www.clearskysec.com/wp-content/uploads/2019/06/Clearsky-Iranian-APT-group-%E2%80%98MuddyWater%E2%80%99-Adds-Exploits-to-Their-Arsenal.pdf)

|  [G0129](https://attack.mitre.org/groups/G0129)   |  [Mustang Panda](https://attack.mitre.org/groups/G0129)   |

[Mustang Panda](https://attack.mitre.org/groups/G0129) has exploited CVE-2017-0199 in Microsoft Word to execute code.[[63]](https://www.crowdstrike.com/blog/meet-crowdstrikes-adversary-of-the-month-for-june-mustang-panda/)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) has exploited CVE-2024-30088 to run arbitrary code in the context of SYSTEM.[[64]](https://www.trendmicro.com/en_us/research/24/j/earth-simnavaz-cyberattacks.html)

|  [C0016](https://attack.mitre.org/campaigns/C0016)   |  [Operation Dust Storm](https://attack.mitre.org/campaigns/C0016)   |

During [Operation Dust Storm](https://attack.mitre.org/campaigns/C0016), the threat actors exploited Adobe Flash vulnerability CVE-2011-0611, Microsoft Windows Help vulnerability CVE-2010-1885, and several Internet Explorer vulnerabilities, including CVE-2011-1255, CVE-2012-1889, and CVE-2014-0322.[[65]](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)

|  [G0040](https://attack.mitre.org/groups/G0040)   |  [Patchwork](https://attack.mitre.org/groups/G0040)   |

[Patchwork](https://attack.mitre.org/groups/G0040) uses malicious documents to deliver remote execution exploits as part of. The group has previously exploited CVE-2017-8570, CVE-2012-1856, CVE-2014-4114, CVE-2017-0199, CVE-2017-11882, and CVE-2015-1641.[[66]](https://web.archive.org/web/20180825085952/https:/s3-us-west-2.amazonaws.com/cymmetria-blog/public/Unveiling_Patchwork.pdf)[[67]](https://securelist.com/the-dropping-elephant-actor/75328/)[[68]](http://www.symantec.com/connect/blogs/patchwork-cyberespionage-group-expands-targets-governments-wide-range-industries)[[69]](https://researchcenter.paloaltonetworks.com/2018/03/unit42-patchwork-continues-deliver-badnews-indian-subcontinent/)[[70]](https://documents.trendmicro.com/assets/tech-brief-untangling-the-patchwork-cyberespionage-group.pdf)[[71]](https://www.volexity.com/blog/2018/06/07/patchwork-apt-group-targets-us-think-tanks/)[[72]](https://unit42.paloaltonetworks.com/updated-backconfig-malware-targeting-government-and-military-organizations/)

|  [S0458](https://attack.mitre.org/software/S0458)   |  [Ramsay](https://attack.mitre.org/software/S0458)   |

[Ramsay](https://attack.mitre.org/software/S0458) has been embedded in documents exploiting CVE-2017-0199, CVE-2017-11882, and CVE-2017-8570.[[73]](https://www.welivesecurity.com/2020/05/13/ramsay-cyberespionage-toolkit-airgapped-networks/)[[74]](https://www.programmersought.com/article/62493896999/)

|  [C0047](https://attack.mitre.org/campaigns/C0047)   |  [RedDelta Modified PlugX Infection Chain Operations](https://attack.mitre.org/campaigns/C0047)   |

[Mustang Panda](https://attack.mitre.org/groups/G0129) used the GrimResource exploitation technique via specially crafted MSC files for arbitrary code execution during [RedDelta Modified PlugX Infection Chain Operations](https://attack.mitre.org/campaigns/C0047).[[75]](https://go.recordedfuture.com/hubfs/reports/cta-cn-2025-0109.pdf)

|  [C0056](https://attack.mitre.org/campaigns/C0056)   |  [RedPenguin](https://attack.mitre.org/campaigns/C0056)   |

During [RedPenguin](https://attack.mitre.org/campaigns/C0056), [UNC3886](https://attack.mitre.org/groups/G1048) exploited CVE-2025-21590 to bypass Veriexec protections in Junos OS designed to prevent unauthorized binary execution.[[76]](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-targets-juniper-routers)[[77]](https://supportportal.juniper.net/sfc/servlet.shepherd/document/download/069Dp00000FzdmIIAR?operationContext=S1)

|  [G1031](https://attack.mitre.org/groups/G1031)   |  [Saint Bear](https://attack.mitre.org/groups/G1031)   |

[Saint Bear](https://attack.mitre.org/groups/G1031) has leveraged vulnerabilities in client applications such as CVE-2017-11882 in Microsoft Office to enable code execution in victim environments.[[78]](https://unit42.paloaltonetworks.com/ukraine-targeted-outsteel-saintbot/)

|  [G0034](https://attack.mitre.org/groups/G0034)   |  [Sandworm Team](https://attack.mitre.org/groups/G0034)   |

[Sandworm Team](https://attack.mitre.org/groups/G0034) has exploited vulnerabilities in Microsoft PowerPoint via OLE objects (CVE-2014-4114) and Microsoft Word via crafted TIFF images (CVE-2013-3906).[[79]](https://web.archive.org/web/20160503234007/https:/www.isightpartners.com/2014/10/cve-2014-4114/)[[80]](https://blog.trendmicro.com/trendlabs-security-intelligence/an-analysis-of-windows-zero-day-vulnerability-cve-2014-4114-aka-sandworm/)[[81]](https://www.mcafee.com/blogs/other-blogs/mcafee-labs-detects-zero-day-exploit-targeting-microsoft-office-2)

|  [G1041](https://attack.mitre.org/groups/G1041)   |  [Sea Turtle](https://attack.mitre.org/groups/G1041)   |

[Sea Turtle](https://attack.mitre.org/groups/G1041) has used exploits for vulnerabilities such as CVE-2021-44228, CVE-2021-21974, and CVE-2022-0847 to achieve client code execution.[[82]](https://www.pwc.com/gx/en/issues/cybersecurity/cyber-threat-intelligence/tortoise-and-malwahare.html)

|  [G1057](https://attack.mitre.org/groups/G1057)   |  [ShinyHunters](https://attack.mitre.org/groups/G1057)   |

[ShinyHunters](https://attack.mitre.org/groups/G1057) has exploited vulnerabilities in the target company’s GitHub repository source code to enable more complex follow-on third-party or supply chain attacks.[[83]](https://www.intel471.com/blog/shinyhunters-data-breach-mitre-attack)

|  [G0121](https://attack.mitre.org/groups/G0121)   |  [Sidewinder](https://attack.mitre.org/groups/G0121)   |

[Sidewinder](https://attack.mitre.org/groups/G0121) has exploited vulnerabilities to gain execution including CVE-2017-11882 and CVE-2020-0674.[[84]](https://cdn-cybersecurity.att.com/docs/global-perspective-of-the-sidewinder-apt.pdf)[[85]](https://cybleinc.com/2020/09/26/sidewinder-apt-targets-with-futuristic-tactics-and-techniques/)

|  [S0374](https://attack.mitre.org/software/S0374)   |  [SpeakUp](https://attack.mitre.org/software/S0374)   |

[SpeakUp](https://attack.mitre.org/software/S0374) attempts to exploit the following vulnerabilities in order to execute its malicious script: CVE-2012-0874, CVE-2010-1871, CVE-2017-10271, CVE-2018-2894, CVE-2016-3088, JBoss AS 3/4/5/6, and the Hadoop YARN ResourceManager. [[86]](https://research.checkpoint.com/speakup-a-new-undetected-backdoor-linux-trojan/)

|  [S0578](https://attack.mitre.org/software/S0578)   |  [SUPERNOVA](https://attack.mitre.org/software/S0578)   |

[SUPERNOVA](https://attack.mitre.org/software/S0578) was installed via exploitation of a SolarWinds Orion API authentication bypass vulnerability (CVE-2020-10148).[[87]](https://www.kb.cert.org/vuls/id/843464)[[88]](https://www.splunk.com/en_us/blog/security/detecting-supernova-malware-solarwinds-continued.html)

|  [G0062](https://attack.mitre.org/groups/G0062)   |  [TA459](https://attack.mitre.org/groups/G0062)   |

[TA459](https://attack.mitre.org/groups/G0062) has exploited Microsoft Word vulnerability CVE-2017-0199 for execution.[[89]](https://www.proofpoint.com/us/threat-insight/post/apt-targets-financial-analysts)

|  [G0089](https://attack.mitre.org/groups/G0089)   |  [The White Company](https://attack.mitre.org/groups/G0089)   |

[The White Company](https://attack.mitre.org/groups/G0089) has taken advantage of a known vulnerability in Microsoft Word (CVE 2012-0158) to execute code.[[90]](https://www.cylance.com/content/dam/cylance-web/en-us/resources/knowledge-center/resource-library/reports/WhiteCompanyOperationShaheenReport.pdf?_ga=2.161661948.1943296560.1555683782-1066572390.1555511517)

|  [G0027](https://attack.mitre.org/groups/G0027)   |  [Threat Group-3390](https://attack.mitre.org/groups/G0027)   |

[Threat Group-3390](https://attack.mitre.org/groups/G0027) has exploited CVE-2018-0798 in Equation Editor.[[91]](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)

|  [G0131](https://attack.mitre.org/groups/G0131)   |  [Tonto Team](https://attack.mitre.org/groups/G0131)   |

[Tonto Team](https://attack.mitre.org/groups/G0131) has exploited Microsoft vulnerabilities, including CVE-2018-0798, CVE-2018-8174, CVE-2018-0802, CVE-2017-11882, CVE-2019-9489 CVE-2020-8468, and CVE-2018-0798 to enable execution of their delivered malicious payloads.[[92]](https://securelist.com/cactuspete-apt-groups-updated-bisonal-backdoor/97962/)[[93]](https://vb2020.vblocalhost.com/uploads/VB2020-06.pdf)[[94]](https://blog.talosintelligence.com/2020/03/bisonal-10-years-of-play.html)[[95]](https://blog.talosintelligence.com/2020/03/bisonal-10-years-of-play.html)

|  [G0134](https://attack.mitre.org/groups/G0134)   |  [Transparent Tribe](https://attack.mitre.org/groups/G0134)   |

[Transparent Tribe](https://attack.mitre.org/groups/G0134) has crafted malicious files to exploit CVE-2012-0158 and CVE-2010-3333 for execution.[[96]](https://www.proofpoint.com/sites/default/files/proofpoint-operation-transparent-tribe-threat-insight-en.pdf)

|  [G0081](https://attack.mitre.org/groups/G0081)   |  [Tropic Trooper](https://attack.mitre.org/groups/G0081)   |

[Tropic Trooper](https://attack.mitre.org/groups/G0081) has executed commands through Microsoft security vulnerabilities, including CVE-2017-11882, CVE-2018-0802, and CVE-2012-0158.[[97]](https://blog.trendmicro.com/trendlabs-security-intelligence/tropic-trooper-new-strategy/)[[98]](https://researchcenter.paloaltonetworks.com/2016/11/unit42-tropic-trooper-targets-taiwanese-government-and-fossil-fuel-provider-with-poison-ivy/)

|  [G1048](https://attack.mitre.org/groups/G1048)   |  [UNC3886](https://attack.mitre.org/groups/G1048)   |

[UNC3886](https://attack.mitre.org/groups/G1048) has exoloited CVE-2023-34048 to enable command execution on vCenter servers and CVE-2023-20867 in VMware Tools to execute unauthenticated Guest Operations from ESXi hosts to guest VMs.[[99]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

|  [S1154](https://attack.mitre.org/software/S1154)   |  [VersaMem](https://attack.mitre.org/software/S1154)   |

[VersaMem](https://attack.mitre.org/software/S1154) was installed through exploitation of CVE-2024-39717 in Versa Director servers.[[100]](https://blog.lumen.com/taking-the-crossroads-the-versa-director-zero-day-exploitation/)

|  [S1065](https://attack.mitre.org/software/S1065)   |  [Woody RAT](https://attack.mitre.org/software/S1065)   |

[Woody RAT](https://attack.mitre.org/software/S1065) has relied on CVE-2022-30190 (Follina) for execution during delivery.[[101]](https://www.malwarebytes.com/blog/threat-intelligence/2022/08/woody-rat-a-new-feature-rich-malware-spotted-in-the-wild)

|  [S0341](https://attack.mitre.org/software/S0341)   |  [Xbash](https://attack.mitre.org/software/S0341)   |

[Xbash](https://attack.mitre.org/software/S0341) can attempt to exploit known vulnerabilities in Hadoop, Redis, or ActiveMQ when it finds those services running in order to conduct further execution.[[102]](https://researchcenter.paloaltonetworks.com/2018/09/unit42-xbash-combines-botnet-ransomware-coinmining-worm-targets-linux-windows/)[[103]](https://www.trendmicro.com/vinfo/us/security/news/cybercrime-and-digital-threats/new-multi-platform-xbash-packs-obfuscation-ransomware-coinminer-worm-and-botnet)

|  [S1207](https://attack.mitre.org/software/S1207)   |  [XLoader](https://attack.mitre.org/software/S1207)   |

[XLoader](https://attack.mitre.org/software/S1207) has exploited Office vulnerabilities during local execution such as CVE-2017-11882 and CVE-2018-0798.[[104]](https://www.netskope.com/blog/new-formbook-campaign-delivered-through-phishing-emails)

## Mitigations

| ID  | Mitigation  | Description

|  [M1048](https://attack.mitre.org/mitigations/M1048)   |  [Application Isolation and Sandboxing](https://attack.mitre.org/mitigations/M1048)   |

Browser sandboxes can be used to mitigate some of the impact of exploitation, but sandbox escapes may still exist. [[105]](https://blogs.windows.com/msedgedev/2017/03/23/strengthening-microsoft-edge-sandbox/) [[106]](https://arstechnica.com/information-technology/2017/03/hack-that-escapes-vm-by-exploiting-edge-browser-fetches-105000-at-pwn2own/)

Other types of virtualization and application microsegmentation may also mitigate the impact of client-side exploitation. Risks of additional exploits and weaknesses in those systems may still exist. [[106]](https://arstechnica.com/information-technology/2017/03/hack-that-escapes-vm-by-exploiting-edge-browser-fetches-105000-at-pwn2own/)

|  [M1050](https://attack.mitre.org/mitigations/M1050)   |  [Exploit Protection](https://attack.mitre.org/mitigations/M1050)   |

Security applications that look for behavior used during exploitation such as Windows Defender Exploit Guard (WDEG) and the Enhanced Mitigation Experience Toolkit (EMET) can be used to mitigate some exploitation behavior. [[107]](https://blogs.technet.microsoft.com/srd/2017/08/09/moving-beyond-emet-ii-windows-defender-exploit-guard/) Control flow integrity checking is another way to potentially identify and stop a software exploit from occurring. [[108]](https://en.wikipedia.org/wiki/Control-flow_integrity) Many of these protections depend on the architecture and target application binary for compatibility.

|  [M1051](https://attack.mitre.org/mitigations/M1051)   |  [Update Software](https://attack.mitre.org/mitigations/M1051)   |

Perform regular software updates to mitigate exploitation risk. Keeping software up-to-date with the latest security patches helps prevent adversaries from exploiting known vulnerabilities in client software, reducing the risk of successful attacks.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0287](https://attack.mitre.org/detectionstrategies/DET0287)   |  [Exploitation for Client Execution – cross-platform behavior chain (browser/Office/3rd-party apps)](https://attack.mitre.org/detectionstrategies/DET0287)   |  [AN0797](https://attack.mitre.org/detectionstrategies/DET0287#AN0797)   |

Cause→effect chain: (1) A client app (browser, Office, PDF/Flash/reader) experiences a crash/abnormal exit or loads from an unusual location, then (2) drops or modifies a file in user-writable paths, and/or (3) spawns an unexpected child (e.g., powershell/cmd/mshta/rundll32/wscript/installer), and (4) establishes outbound C2-like connections shortly after. Correlate application logs, file writes, process lineage, and network egress within a short window.

|  [AN0798](https://attack.mitre.org/detectionstrategies/DET0287#AN0798)   |

Cause→effect chain: (1) Browser/Office/reader process logs crash/segfault or abnormal sandbox message, (2) new executable/script/write occurs in $HOME (Downloads, ~/.cache, /tmp), (3) unexpected child like curl/wget/bash/python opens network connections soon after.

|  [AN0799](https://attack.mitre.org/detectionstrategies/DET0287#AN0799)   |

Cause→effect chain: (1) App crash/abnormal termination in unified logs for Safari/Chrome/Office/Preview, (2) new files/scripts in ~/Library, ~/Downloads, /private/var/folders/*, (3) unexpected child (osascript, zsh, bash, curl) spawned by those apps, (4) new outbound connections.

## References

-    [Jeff Johnson, Fred Plan, Adrian Sanchez, Renato Fontana, Jake Nicastro, Dimiter Andonov, Marius Fodoreanu, Daniel Scott. (2023, April 20). 3CX Software Supply Chain Compromise Initiated by a Prior Software Supply Chain Compromise; Suspected North Korean Actor Responsible. Retrieved August 25, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/3cx-software-supply-chain-compromise/)
-    [FireEye Threat Intelligence. (2015, December 1). China-based Cyber Threat Group Uses Dropbox for Malware Communications and Targets Hong Kong Media Outlets. Retrieved December 4, 2015.](https://www.fireeye.com/blog/threat-research/2015/11/china-based-threat.html)
-    [Walter, J. (2020, August 10). Agent Tesla | Old RAT Uses New Tricks to Stay on Top. Retrieved December 11, 2020.](https://labs.sentinelone.com/agent-tesla-old-rat-uses-new-tricks-to-stay-on-top/)
-    [FSI. (2017, July 27). Campaign Rifle - Andariel, the Maiden of Anguish. Retrieved September 12, 2024.](https://fsiceat.tistory.com/2)
-    [IssueMakersLab. (2017, May 1). Operation GoldenAxe. Retrieved September 12, 2024.](http://www.issuemakerslab.com/research3/)
-    [Chen, Joseph. (2018, July 16). New Andariel Reconnaissance Tactics Uncovered. Retrieved September 29, 2021.](https://www.trendmicro.com/en_us/research/18/g/new-andariel-reconnaissance-tactics-hint-at-next-targets.html)
-    [Chen, Joey. (2022, June 9). Aoqin Dragon | Newly-Discovered Chinese-linked APT Has Been Quietly Spying On Organizations For 10 Years. Retrieved July 14, 2022.](https://www.sentinelone.com/labs/aoqin-dragon-newly-discovered-chinese-linked-apt-has-been-quietly-spying-on-organizations-for-10-years/)
-    [Moran, N., Oppenheim, M., Engle, S., & Wartell, R.. (2014, September 3). Darwin’s Favorite APT Group [Blog]. Retrieved November 12, 2014.](https://www.fireeye.com/blog/threat-research/2014/09/darwins-favorite-apt-group-2.html)
-    [Sancho, D., et al. (2012, May 22). IXESHE An APT Campaign. Retrieved June 7, 2019.](https://www.trendmicro.de/cloud-content/us/pdfs/security-intelligence/white-papers/wp_ixeshe.pdf)
-    [Kaspersky Lab's Global Research & Analysis Team. (2018, February 20). A Slice of 2017 Sofacy Activity. Retrieved November 27, 2018.](https://securelist.com/a-slice-of-2017-sofacy-activity/83930/)
-    [F-Secure Labs. (2015, September 17). The Dukes: 7 years of Russian cyberespionage. Retrieved December 10, 2015.](https://www.f-secure.com/documents/996508/1030745/dukes_whitepaper.pdf)
-    [NCSC, CISA, FBI, NSA. (2021, May 7). Further TTPs associated with SVR cyber actors. Retrieved July 29, 2021.](https://www.ncsc.gov.uk/files/Advisory-further-TTPs-associated-with-SVR-cyber-actors.pdf)
-    [Microsoft Threat Intelligence Center (MSTIC). (2021, May 27). New sophisticated email-based attack from NOBELIUM. Retrieved May 28, 2021.](https://www.microsoft.com/security/blog/2021/05/27/new-sophisticated-email-based-attack-from-nobelium/)
-    [Eng, E., Caselden, D.. (2015, June 23). Operation Clandestine Wolf – Adobe Flash Zero-Day in APT3 Phishing Campaign. Retrieved January 14, 2016.](https://www.fireeye.com/blog/threat-research/2015/06/operation-clandestine-wolf-adobe-flash-zero-day.html)
-    [Chen, X., Scott, M., Caselden, D.. (2014, April 26). New Zero-Day Exploit targeting Internet Explorer Versions 9 through 11 Identified in Targeted Attacks. Retrieved January 14, 2016.](https://www.fireeye.com/blog/threat-research/2014/04/new-zero-day-exploit-targeting-internet-explorer-versions-9-through-11-identified-in-targeted-attacks.html)
-    [Dumont, R. (2019, March 20). Fake or Fake: Keeping up with OceanLotus decoys. Retrieved April 1, 2019.](https://www.welivesecurity.com/2019/03/20/fake-or-fake-keeping-up-with-oceanlotus-decoys/)
-    [Security Response attack Investigation Team. (2019, March 27). Elfin: Relentless Espionage Group Targets Multiple Organizations in Saudi Arabia and U.S.. Retrieved April 10, 2019.](https://www.symantec.com/blogs/threat-intelligence/elfin-apt33-espionage)
-    [Microsoft Threat Protection Intelligence Team. (2020, June 18). Inside Microsoft Threat Protection: Mapping attack chains from cloud to endpoint. Retrieved June 22, 2020.](https://www.microsoft.com/security/blog/2020/06/18/inside-microsoft-threat-protection-mapping-attack-chains-from-cloud-to-endpoint/)
-    [Raiu, C., and Ivanov, A. (2016, June 17). Operation Daybreak. Retrieved February 15, 2018.](https://securelist.com/operation-daybreak/75100/)
-    [FireEye. (2018, February 20). APT37 (Reaper): The Overlooked North Korean Actor. Retrieved November 17, 2024.](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)
-    [Mercer, W., Rascagneres, P. (2018, January 16). Korea In The Crosshairs. Retrieved May 21, 2018.](https://blog.talosintelligence.com/2018/01/korea-in-crosshairs.html)
-    [Cash, D., Grunzweig, J., Meltzer, M., Adair, S., Lancaster, T. (2021, August 17). North Korean APT InkySquid Infects Victims Using Browser Exploits. Retrieved September 30, 2021.](https://www.volexity.com/blog/2021/08/17/north-korean-apt-inkysquid-infects-victims-using-browser-exploits/)
-    [Fraser, N., et al. (2019, August 7). Double DragonAPT41, a dual espionage and cyber crime operation APT41. Retrieved September 23, 2019.](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)
-    [Esler, J., Lee, M., and Williams, C. (2014, October 14). Threat Spotlight: Group 72. Retrieved January 14, 2016.](http://blogs.cisco.com/security/talos/threat-spotlight-group-72)
-    [Sherstobitoff, R. (2018, March 08). Hidden Cobra Targets Turkish Financial Sector With New Bankshot Implant. Retrieved May 18, 2018.](https://securingtomorrow.mcafee.com/mcafee-labs/hidden-cobra-targets-turkish-financial-sector-new-bankshot-implant/)
-    [Raghuprasad, C . (2022, May 11). Bitter APT adds Bangladesh to their targets. Retrieved June 1, 2022.](https://blog.talosintelligence.com/2022/05/bitter-apt-adds-bangladesh-to-their.html)
-    [Dela Paz, R. (2016, October 21). BITTER: a targeted attack against Pakistan. Retrieved June 1, 2022.](https://www.forcepoint.com/blog/x-labs/bitter-targeted-attack-against-pakistan)
-    [Bermejo, L., et al. (2017, June 22). Following the Trail of BlackTech’s Cyber Espionage Campaigns. Retrieved May 5, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)
-    [DiMaggio, J. (2016, April 28). Tick cyberespionage group zeros in on Japan. Retrieved July 16, 2018.](https://www.symantec.com/connect/blogs/tick-cyberespionage-group-zeros-japan)
-    [Chen, J. et al. (2019, November). Operation ENDTRADE: TICK’s Multi-Stage Backdoors for Attacking Industries and Stealing Classified Data. Retrieved June 9, 2020.](https://documents.trendmicro.com/assets/pdf/Operation-ENDTRADE-TICK-s-Multi-Stage-Backdoors-for-Attacking-Industries-and-Stealing-Classified-Data.pdf)
-    [Svajcer, V. (2018, July 31). Multiple Cobalt Personality Disorder. Retrieved September 5, 2018.](https://blog.talosintelligence.com/2018/07/multiple-cobalt-personality-disorder.html)
-    [Positive Technologies. (2017, August 16). Cobalt Strikes Back: An Evolving Multinational Threat to Finance. Retrieved September 5, 2018.](https://www.ptsecurity.com/upload/corporate/ww-en/analytics/Cobalt-2017-eng.pdf)
-    [Positive Technologies. (2016, December 16). Cobalt Snatch. Retrieved October 9, 2018.](https://www.ptsecurity.com/upload/corporate/ww-en/analytics/Cobalt-Snatch-eng.pdf)
-    [Mesa, M, et al. (2017, June 1). Microsoft Word Intruder Integrates CVE-2017-0199, Utilized by Cobalt Group to Target Financial Institutions. Retrieved October 10, 2018.](https://www.proofpoint.com/us/threat-insight/post/microsoft-word-intruder-integrates-cve-2017-0199-utilized-cobalt-group-target)
-    [Klijnsma, Y.. (2017, November 28). Gaffe Reveals Full List of Targets in Spear Phishing Attack Using Cobalt Strike Against Financial Institutions. Retrieved October 10, 2018.](https://web.archive.org/web/20190508170630/https://www.riskiq.com/blog/labs/cobalt-strike/)
-    [Klijnsma, Y.. (2018, January 16). First Activities of Cobalt Group in 2018: Spear Phishing Russian Banks. Retrieved October 10, 2018.](https://web.archive.org/web/20190508170147/https://www.riskiq.com/blog/labs/cobalt-group-spear-phishing-russian-banks/)
-    [CrowdStrike. (2018, February 26). CrowdStrike 2018 Global Threat Report. Retrieved October 10, 2018.](https://crowdstrike.lookbookhq.com/global-threat-report-2018-web/cs-2018-global-threat-report)
-    [Giagone, R., Bermejo, L., and Yarochkin, F. (2017, November 20). Cobalt Strikes Again: Spam Runs Use Macros and CVE-2017-8759 Exploit Against Russian Banks. Retrieved March 7, 2019.](https://blog.trendmicro.com/trendlabs-security-intelligence/cobalt-spam-runs-use-macros-cve-2017-8759-exploit/)
-    [Mavis, N. (2020, September 21). The Art and Science of Detecting Cobalt Strike. Retrieved September 12, 2024.](https://web.archive.org/web/20210219195905/https://talos-intelligence-site.s3.amazonaws.com/production/document_files/files/000/095/031/original/Talos_Cobalt_Strike.pdf)
-    [Strategic Cyber LLC. (2020, November 5). Cobalt Strike: Advanced Threat Tactics for Penetration Testers. Retrieved April 13, 2021.](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)
-    [Uptycs Threat Research Team. (2021, January 12). Confucius APT deploys Warzone RAT. Retrieved December 17, 2021.](https://www.uptycs.com/blog/confucius-apt-deploys-warzone-rat)
-    [Lunghi, D and Horejsi, J. (2018, February 13). Deciphering Confucius: A Look at the Group's Cyberespionage Operations. Retrieved December 26, 2021.](https://www.trendmicro.com/en_us/research/18/b/deciphering-confucius-cyberespionage-operations.html)
-    [Microsoft. (2016, June 9). Reverse-engineering DUBNIUM. Retrieved March 31, 2021.](https://www.microsoft.com/security/blog/2016/06/09/reverse-engineering-dubnium-2/)
-    [Falcone, R. (2018, March 15). Sofacy Uses DealersChoice to Target European Government Agency. Retrieved June 4, 2018.](https://researchcenter.paloaltonetworks.com/2018/03/unit42-sofacy-uses-dealerschoice-target-european-government-agency/)
-    [Slowik, J. (2021, October). THE BAFFLING BERSERK BEAR: A DECADE’S ACTIVITY TARGETING CRITICAL INFRASTRUCTURE. Retrieved December 6, 2021.](https://vblocalhost.com/uploads/VB2021-Slowik.pdf)
-    [O'Gorman, G., and McDonald, G.. (2012, September 6). The Elderwood Project. Retrieved November 17, 2024.](https://web.archive.org/web/20190717233006/http:/www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/the-elderwood-project.pdf)
-    [US Cybersecurity & Infrastructure Security Agency et al. (2024, September 5). Russian Military Cyber Actors Target U.S. and Global Critical Infrastructure. Retrieved September 6, 2024.](https://www.cisa.gov/sites/default/files/2024-09/aa24-249a-russian-military-cyber-actors-target-us-and-global-critical-infrastructure.pdf)
-    [Marschalek, M.. (2014, December 16). EvilBunny: Malware Instrumented By Lua. Retrieved June 28, 2019.](https://web.archive.org/web/20150311013500/http://www.cyphort.com/evilbunny-malware-instrumented-lua/)
-    [Stolyarov, V. (2022, March 17). Exposing initial access broker with ties to Conti. Retrieved August 18, 2022.](https://blog.google/threat-analysis-group/exposing-initial-access-broker-ties-conti/)
-    [Adamitis, D. et al. (2019, June 4). It's alive: Threat actors cobble together open-source pieces into monstrous Frankenstein campaign. Retrieved May 11, 2020.](https://blog.talosintelligence.com/2019/06/frankenstein-campaign.html)
-    [Patil, S. and Williams, M.. (2019, June 5). Government Sector in Central Asia Targeted With New HAWKBALL Backdoor Delivered via Microsoft Office Vulnerabilities. Retrieved June 20, 2019.](https://www.fireeye.com/blog/threat-research/2019/06/government-in-central-asia-targeted-with-hawkball-backdoor.html)
-    [PT ESC Threat Intelligence. (2020, June 4). COVID-19 and New Year greetings: an investigation into the tools and methods used by the Higaisa group. Retrieved March 2, 2021.](https://www.ptsecurity.com/ww-en/analytics/pt-esc-threat-intelligence/covid-19-and-new-year-greetings-the-higaisa-group/)
-    [GReAT. (2019, August 12). Recent Cloud Atlas activity. Retrieved May 8, 2020.](https://securelist.com/recent-cloud-atlas-activity/92016/)
-    [GReAT. (2014, December 10). Cloud Atlas: RedOctober APT is back in style. Retrieved May 8, 2020.](https://securelist.com/cloud-atlas-redoctober-apt-is-back-in-style/68083/)

-    [Symantec. (2018, March 14). Inception Framework: Alive and Well, and Hiding Behind Proxies. Retrieved May 8, 2020.](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/inception-framework-hiding-behind-proxies)
-    [Lancaster, T. (2018, November 5). Inception Attackers Target Europe with Year-old Office Vulnerability. Retrieved May 8, 2020.](https://unit42.paloaltonetworks.com/unit42-inception-attackers-target-europe-year-old-office-vulnerability/)
-    [Hromcova, Z. and Cherpanov, A. (2020, June). INVISIMOLE: THE HIDDEN PART OF THE STORY. Retrieved July 16, 2020.](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_InvisiMole.pdf)
-    [Axel F, Pierre T. (2017, October 16). Leviathan: Espionage actor spearphishes maritime and defense targets. Retrieved February 15, 2018.](https://www.proofpoint.com/us/threat-insight/post/leviathan-espionage-actor-spearphishes-maritime-and-defense-targets)
-    [FireEye. (2018, March 16). Suspected Chinese Cyber Espionage Group (TEMP.Periscope) Targeting U.S. Engineering and Maritime Industries. Retrieved April 11, 2018.](https://www.fireeye.com/blog/threat-research/2018/03/suspected-chinese-espionage-group-targeting-maritime-and-engineering-industries.html)
-    [CISA. (2021, July 19). (AA21-200A) Joint Cybersecurity Advisory – Tactics, Techniques, and Procedures of Indicted APT40 Actors Associated with China’s MSS Hainan State Security Department. Retrieved August 12, 2021.](https://us-cert.cisa.gov/ncas/alerts/aa21-200a)
-    [Accenture iDefense Unit. (2019, March 5). Mudcarp's Focus on Submarine Technologies. Retrieved August 24, 2021.](https://web.archive.org/web/20210722005437/https://www.accenture.com/_acnmedia/PDF-96/Accenture-Security-MUDCARP.pdf)
-    [ClearSky. (2019, June). Iranian APT group ‘MuddyWater’ Adds Exploits to Their Arsenal. Retrieved May 14, 2020.](https://www.clearskysec.com/wp-content/uploads/2019/06/Clearsky-Iranian-APT-group-%E2%80%98MuddyWater%E2%80%99-Adds-Exploits-to-Their-Arsenal.pdf)
-    [Meyers, A. (2018, June 15). Meet CrowdStrike’s Adversary of the Month for June: MUSTANG PANDA. Retrieved April 12, 2021.](https://www.crowdstrike.com/blog/meet-crowdstrikes-adversary-of-the-month-for-june-mustang-panda/)
-    [Fahmy, M. et al. (2024, October 11). Earth Simnavaz (aka APT34) Levies Advanced Cyberattacks Against Middle East. Retrieved November 27, 2024.](https://www.trendmicro.com/en_us/research/24/j/earth-simnavaz-cyberattacks.html)
-    [Gross, J. (2016, February 23). Operation Dust Storm. Retrieved December 22, 2021.](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)
-    [Cymmetria. (2016). Unveiling Patchwork - The Copy-Paste APT. Retrieved November 17, 2024.](https://web.archive.org/web/20180825085952/https:/s3-us-west-2.amazonaws.com/cymmetria-blog/public/Unveiling_Patchwork.pdf)
-    [Kaspersky Lab's Global Research & Analysis Team. (2016, July 8). The Dropping Elephant – aggressive cyber-espionage in the Asian region. Retrieved August 3, 2016.](https://securelist.com/the-dropping-elephant-actor/75328/)
-    [Hamada, J.. (2016, July 25). Patchwork cyberespionage group expands targets from governments to wide range of industries. Retrieved August 17, 2016.](http://www.symantec.com/connect/blogs/patchwork-cyberespionage-group-expands-targets-governments-wide-range-industries)
-    [Levene, B. et al.. (2018, March 7). Patchwork Continues to Deliver BADNEWS to the Indian Subcontinent. Retrieved March 31, 2018.](https://researchcenter.paloaltonetworks.com/2018/03/unit42-patchwork-continues-deliver-badnews-indian-subcontinent/)
-    [Lunghi, D., et al. (2017, December). Untangling the Patchwork Cyberespionage Group. Retrieved July 10, 2018.](https://documents.trendmicro.com/assets/tech-brief-untangling-the-patchwork-cyberespionage-group.pdf)
-    [Meltzer, M, et al. (2018, June 07). Patchwork APT Group Targets US Think Tanks. Retrieved July 16, 2018.](https://www.volexity.com/blog/2018/06/07/patchwork-apt-group-targets-us-think-tanks/)
-    [Hinchliffe, A. and Falcone, R. (2020, May 11). Updated BackConfig Malware Targeting Government and Military Organizations in South Asia. Retrieved June 17, 2020.](https://unit42.paloaltonetworks.com/updated-backconfig-malware-targeting-government-and-military-organizations/)
-    [Sanmillan, I.. (2020, May 13). Ramsay: A cyber‑espionage toolkit tailored for air‑gapped networks. Retrieved May 27, 2020.](https://www.welivesecurity.com/2020/05/13/ramsay-cyberespionage-toolkit-airgapped-networks/)
-    [Antiy CERT. (2020, April 20). Analysis of Ramsay components of Darkhotel's infiltration and isolation network. Retrieved March 24, 2021.](https://www.programmersought.com/article/62493896999/)
-    [Insikt Group. (2025, January 9). Chinese State-Sponsored RedDelta Targeted Taiwan, Mongolia, and Southeast Asia with Adapted PlugX Infection Chain. Retrieved January 14, 2025.](https://go.recordedfuture.com/hubfs/reports/cta-cn-2025-0109.pdf)
-    [Lamparski, L. et al. (2025, March 11). Ghost in the Router: China-Nexus Espionage Actor UNC3886 Targets Juniper Routers. Retrieved June 24, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-targets-juniper-routers)
-    [Juniper Networks, Cybersecurity R&D. (2025, March 11). The RedPenguin Malware Incident. Retrieved June 24, 2025.](https://supportportal.juniper.net/sfc/servlet.shepherd/document/download/069Dp00000FzdmIIAR?operationContext=S1)
-    [Unit 42. (2022, February 25). Spear Phishing Attacks Target Organizations in Ukraine, Payloads Include the Document Stealer OutSteel and the Downloader SaintBot. Retrieved June 9, 2022.](https://unit42.paloaltonetworks.com/ukraine-targeted-outsteel-saintbot/)
-    [Ward, S.. (2014, October 14). iSIGHT discovers zero-day vulnerability CVE-2014-4114 used in Russian cyber-espionage campaign. Retrieved November 17, 2024.](https://web.archive.org/web/20160503234007/https:/www.isightpartners.com/2014/10/cve-2014-4114/)
-    [Wu, W. (2014, October 14). An Analysis of Windows Zero-day Vulnerability ‘CVE-2014-4114’ aka “Sandworm”. Retrieved June 18, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/an-analysis-of-windows-zero-day-vulnerability-cve-2014-4114-aka-sandworm/)
-    [Li, H. (2013, November 5). McAfee Labs Detects Zero-Day Exploit Targeting Microsoft Office. Retrieved June 18, 2020.](https://www.mcafee.com/blogs/other-blogs/mcafee-labs-detects-zero-day-exploit-targeting-microsoft-office-2)
-    [PwC Threat Intelligence. (2023, December 5). The Tortoise and The Malware. Retrieved November 20, 2024.](https://www.pwc.com/gx/en/issues/cybersecurity/cyber-threat-intelligence/tortoise-and-malwahare.html)
-    [Intel 471. (2021, August 23). Here’s how to guard your enterprise against ShinyHunters. Retrieved July 29, 2026.](https://www.intel471.com/blog/shinyhunters-data-breach-mitre-attack)
-    [Hegel, T. (2021, January 13). A Global Perspective of the SideWinder APT. Retrieved January 27, 2021.](https://cdn-cybersecurity.att.com/docs/global-perspective-of-the-sidewinder-apt.pdf)
-    [Cyble. (2020, September 26). SideWinder APT Targets with futuristic Tactics and Techniques. Retrieved January 29, 2021.](https://cybleinc.com/2020/09/26/sidewinder-apt-targets-with-futuristic-tactics-and-techniques/)
-    [Check Point Research. (2019, February 4). SpeakUp: A New Undetected Backdoor Linux Trojan. Retrieved April 17, 2019.](https://research.checkpoint.com/speakup-a-new-undetected-backdoor-linux-trojan/)
-    [Carnegie Mellon University. (2020, December 26). SolarWinds Orion API authentication bypass allows remote command execution. Retrieved February 22, 2021.](https://www.kb.cert.org/vuls/id/843464)
-    [Stoner, J. (2021, January 21). Detecting Supernova Malware: SolarWinds Continued. Retrieved February 22, 2021.](https://www.splunk.com/en_us/blog/security/detecting-supernova-malware-solarwinds-continued.html)
-    [Axel F. (2017, April 27). APT Targets Financial Analysts with CVE-2017-0199. Retrieved February 15, 2018.](https://www.proofpoint.com/us/threat-insight/post/apt-targets-financial-analysts)
-    [Livelli, K, et al. (2018, November 12). Operation Shaheen. Retrieved May 1, 2019.](https://www.cylance.com/content/dam/cylance-web/en-us/resources/knowledge-center/resource-library/reports/WhiteCompanyOperationShaheenReport.pdf?_ga=2.161661948.1943296560.1555683782-1066572390.1555511517)
-    [Lunghi, D. and Lu, K. (2021, April 9). Iron Tiger APT Updates Toolkit With Evolved SysUpdate Malware. Retrieved November 12, 2021.](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)
-    [Zykov, K. (2020, August 13). CactusPete APT group’s updated Bisonal backdoor. Retrieved May 5, 2021.](https://securelist.com/cactuspete-apt-groups-updated-bisonal-backdoor/97962/)
-    [Daniel Lughi, Jaromir Horejsi. (2020, October 2). Tonto Team - Exploring the TTPs of an advanced threat actor operating a large infrastructure. Retrieved October 17, 2021.](https://vb2020.vblocalhost.com/uploads/VB2020-06.pdf)
-    [Mercer, W., et al. (2020, March 5). Bisonal: 10 years of play. Retrieved January 26, 2022.](https://blog.talosintelligence.com/2020/03/bisonal-10-years-of-play.html)
-    [Warren Mercer, Paul Rascagneres, Vitor Ventura. (2020, March 6). Bisonal 10 Years of Play. Retrieved October 17, 2021.](https://blog.talosintelligence.com/2020/03/bisonal-10-years-of-play.html)
-    [Huss, D. (2016, March 1). Operation Transparent Tribe. Retrieved June 8, 2016.](https://www.proofpoint.com/sites/default/files/proofpoint-operation-transparent-tribe-threat-insight-en.pdf)
-    [Horejsi, J., et al. (2018, March 14). Tropic Trooper’s New Strategy. Retrieved November 9, 2018.](https://blog.trendmicro.com/trendlabs-security-intelligence/tropic-trooper-new-strategy/)
-    [Ray, V. (2016, November 22). Tropic Trooper Targets Taiwanese Government and Fossil Fuel Provider With Poison Ivy. Retrieved November 9, 2018.](https://researchcenter.paloaltonetworks.com/2016/11/unit42-tropic-trooper-targets-taiwanese-government-and-fossil-fuel-provider-with-poison-ivy/)
-    [Punsaen Boonyakarn, Shawn Chew, Logeswaran Nadarajan, Mathew Potaczek, Jakub Jozwiak, and Alex Marvi. (2024, June 18). Cloaked and Covert: Uncovering UNC3886 Espionage Operations. Retrieved September 24, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)
-    [Black Lotus Labs. (2024, August 27). Taking The Crossroads: The Versa Director Zero-Day Exploitaiton. Retrieved August 27, 2024.](https://blog.lumen.com/taking-the-crossroads-the-versa-director-zero-day-exploitation/)
-    [MalwareBytes Threat Intelligence Team. (2022, August 3). Woody RAT: A new feature-rich malware spotted in the wild. Retrieved December 6, 2022.](https://www.malwarebytes.com/blog/threat-intelligence/2022/08/woody-rat-a-new-feature-rich-malware-spotted-in-the-wild)
-    [Xiao, C. (2018, September 17). Xbash Combines Botnet, Ransomware, Coinmining in Worm that Targets Linux and Windows. Retrieved November 14, 2018.](https://researchcenter.paloaltonetworks.com/2018/09/unit42-xbash-combines-botnet-ransomware-coinmining-worm-targets-linux-windows/)
-    [Trend Micro. (2018, September 19). New Multi-Platform Xbash Packs Obfuscation, Ransomware, Coinminer, Worm and Botnet. Retrieved June 4, 2019.](https://www.trendmicro.com/vinfo/us/security/news/cybercrime-and-digital-threats/new-multi-platform-xbash-packs-obfuscation-ransomware-coinminer-worm-and-botnet)
-    [Gustavo Palazolo, Netskope. (2022, March 11). New Formbook Campaign Delivered Through Phishing Emails. Retrieved March 11, 2025.](https://www.netskope.com/blog/new-formbook-campaign-delivered-through-phishing-emails)
-    [Cowan, C. (2017, March 23). Strengthening the Microsoft Edge Sandbox. Retrieved March 12, 2018.](https://blogs.windows.com/msedgedev/2017/03/23/strengthening-microsoft-edge-sandbox/)
-    [Goodin, D. (2017, March 17). Virtual machine escape fetches $105,000 at Pwn2Own hacking contest - updated. Retrieved March 12, 2018.](https://arstechnica.com/information-technology/2017/03/hack-that-escapes-vm-by-exploiting-edge-browser-fetches-105000-at-pwn2own/)
-    [Nunez, N. (2017, August 9). Moving Beyond EMET II – Windows Defender Exploit Guard. Retrieved March 12, 2018.](https://blogs.technet.microsoft.com/srd/2017/08/09/moving-beyond-emet-ii-windows-defender-exploit-guard/)
-    [Wikipedia. (2018, January 11). Control-flow integrity. Retrieved March 12, 2018.](https://en.wikipedia.org/wiki/Control-flow_integrity)

×
