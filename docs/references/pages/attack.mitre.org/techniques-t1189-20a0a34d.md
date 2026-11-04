---
url: "https://attack.mitre.org/techniques/T1189/"
title: "Drive-by Compromise, Technique T1189 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:38:14Z"
via: "live"
sha256: "c758934645e2f0c502be238d3aa9423336720504e6d5c7bfae3111695aa17175"
wayback: "https://web.archive.org/web/20261102191652/https://attack.mitre.org/techniques/T1189/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Drive-by Compromise, Technique T1189 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1189/>
> Snapshot: <https://web.archive.org/web/20261102191652/https://attack.mitre.org/techniques/T1189/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Drive-by Compromise

#  Drive-by Compromise

Adversaries may gain access to a system through a user visiting a website over the normal course of browsing. Multiple ways of delivering exploit code to a browser exist (i.e., [Drive-by Target](https://attack.mitre.org/techniques/T1608/004)), including:

- A legitimate website is compromised, allowing adversaries to inject malicious code
- Script files served to a legitimate website from a publicly writeable cloud storage bucket are modified by an adversary
- Malicious ads are paid for and served through legitimate ad providers (i.e., [Malvertising](https://attack.mitre.org/techniques/T1583/008))
- Built-in web application interfaces that allow user-controllable content are leveraged for the insertion of malicious scripts or iFrames (e.g., cross-site scripting)

Browser push notifications may also be abused by adversaries and leveraged for malicious code injection via [User Execution](https://attack.mitre.org/techniques/T1204). By clicking "allow" on browser push notifications, users may be granting a website permission to run JavaScript code on their browser.[[1]](https://viruspositive.com/resources/blogs/the-dark-side-of-web-push-notifications)[[2]](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/scammers-impersonating-windows-defender-to-push-malicious-windows-apps/)[[3]](https://www.malwarebytes.com/blog/news/2019/01/browser-push-notifications-feature-asking-abused)

Often the website used by an adversary is one visited by a specific community, such as government, a particular industry, or a particular region, where the goal is to compromise a specific user or set of users based on a shared interest. This kind of targeted campaign is often referred to a strategic web compromise or watering hole attack. There are several known examples of this occurring.[[4]](http://blog.shadowserver.org/2012/05/15/cyber-espionage-strategic-web-compromises-trusted-websites-serving-dangerous-results/)

Typical drive-by compromise process:

- A user visits a website that is used to host the adversary controlled content.
- Scripts automatically execute, typically searching versions of the browser and plugins for a potentially vulnerable version. The user may be required to assist in this process by enabling scripting, notifications, or active website components and ignoring warning dialog boxes.
- Upon finding a vulnerable version, exploit code is delivered to the browser.
- If exploitation is successful, the adversary will gain code execution on the user's system unless other protections are in place. In some cases, a second visit to the website after the initial scan is required before exploit code is delivered.

Unlike [Exploit Public-Facing Application](https://attack.mitre.org/techniques/T1190), the focus of this technique is to exploit software on a client endpoint upon visiting a website. This will commonly give an adversary access to systems on the internal network instead of external systems that may be in a DMZ.

ID: T1189

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Initial Access](https://attack.mitre.org/tactics/TA0001)

ⓘ

Platforms: Identity Provider, Linux, Windows, macOS

Contributors: Frank Angiolelli; Jeff Sakowicz, Microsoft Identity Developer Platform Services (IDPM Services); Saisha Agrawal, Microsoft Threat Intelligent Center (MSTIC)

Version: 1.7

Created: 18 April 2018

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1189/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1189/)

## Procedure Examples

| ID  | Name  | Description

|  [C0057](https://attack.mitre.org/campaigns/C0057)   |  [3CX Supply Chain Attack](https://attack.mitre.org/campaigns/C0057)   |

During the [3CX Supply Chain Attack](https://attack.mitre.org/campaigns/C0057), [AppleJeus](https://attack.mitre.org/groups/G1049) compromised the www.tradingtechnologies[.]com website hosting a hidden IFRAME to exploit visitors, two months before the site was known to deliver a compromised version of the X_TRADER software package.[[5]](https://cloud.google.com/blog/topics/threat-intelligence/3cx-software-supply-chain-compromise/)

|  [G0138](https://attack.mitre.org/groups/G0138)   |  [Andariel](https://attack.mitre.org/groups/G0138)   |

[Andariel](https://attack.mitre.org/groups/G0138) has used watering hole attacks, often with zero-day exploits, to gain initial access to victims within a specific IP range.[[6]](https://web.archive.org/web/20230213154832/http://download.ahnlab.com/global/brochure/%5BAnalysis%5DAndariel_Group.pdf)[[7]](https://www.trendmicro.com/en_us/research/18/g/new-andariel-reconnaissance-tactics-hint-at-next-targets.html)

|  [G0073](https://attack.mitre.org/groups/G0073)   |  [APT19](https://attack.mitre.org/groups/G0073)   |

[APT19](https://attack.mitre.org/groups/G0073) performed a watering hole attack on forbes.com in 2014 to compromise targets.[[8]](https://researchcenter.paloaltonetworks.com/2016/01/new-attacks-linked-to-c0d0s0-group/)

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[APT28](https://attack.mitre.org/groups/G0007) has compromised targets via strategic web compromise utilizing custom exploit kits.[[9]](https://www.secureworks.com/research/iron-twilight-supports-active-measures) [APT28](https://attack.mitre.org/groups/G0007) used reflected cross-site scripting (XSS) against government websites to redirect users to phishing webpages.[[10]](https://blog.google/threat-analysis-group/ukraine-remains-russias-biggest-cyber-focus-in-2023/)

|  [G0050](https://attack.mitre.org/groups/G0050)   |  [APT32](https://attack.mitre.org/groups/G0050)   |

[APT32](https://attack.mitre.org/groups/G0050) has infected victims by tricking them into visiting compromised watering hole websites.[[11]](https://www.welivesecurity.com/2018/03/13/oceanlotus-ships-new-backdoor/)[[12]](https://www.volexity.com/blog/2020/11/06/oceanlotus-extending-cyber-espionage-operations-through-fake-websites/)

|  [G0067](https://attack.mitre.org/groups/G0067)   |  [APT37](https://attack.mitre.org/groups/G0067)   |

[APT37](https://attack.mitre.org/groups/G0067) has used strategic web compromises, particularly of South Korean websites, to distribute malware. The group has also used torrent file-sharing sites to more indiscriminately disseminate malware to victims. As part of their compromises, the group has used a Javascript based profiler called RICECURRY to profile a victim's web browser and deliver malicious code accordingly.[[13]](https://securelist.com/operation-daybreak/75100/)[[14]](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)[[15]](https://www.volexity.com/blog/2021/08/17/north-korean-apt-inkysquid-infects-victims-using-browser-exploits/)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has conducted watering holes schemes to gain initial access to victims.[[16]](https://services.google.com/fh/files/misc/apt38-un-usual-suspects.pdf)[[17]](https://us-cert.cisa.gov/ncas/alerts/aa20-239a)

|  [G0001](https://attack.mitre.org/groups/G0001)   |  [Axiom](https://attack.mitre.org/groups/G0001)   |

[Axiom](https://attack.mitre.org/groups/G0001) has used watering hole attacks to gain access.[[18]](http://blogs.cisco.com/security/talos/threat-spotlight-group-72)

|  [S0606](https://attack.mitre.org/software/S0606)   |  [Bad Rabbit](https://attack.mitre.org/software/S0606)   |

[Bad Rabbit](https://attack.mitre.org/software/S0606) spread through watering holes on popular sites by injecting JavaScript into the HTML body or a .js file.[[19]](https://www.welivesecurity.com/2017/10/24/bad-rabbit-not-petya-back/)[[20]](https://securelist.com/bad-rabbit-ransomware/82851/)

|  [G0060](https://attack.mitre.org/groups/G0060)   |  [BRONZE BUTLER](https://attack.mitre.org/groups/G0060)   |

[BRONZE BUTLER](https://attack.mitre.org/groups/G0060) compromised three Japanese websites using a Flash exploit to perform watering hole attacks.[[21]](https://www.symantec.com/connect/blogs/tick-cyberespionage-group-zeros-japan)

|  [S0482](https://attack.mitre.org/software/S0482)   |  [Bundlore](https://attack.mitre.org/software/S0482)   |

[Bundlore](https://attack.mitre.org/software/S0482) has been spread through malicious advertisements on websites.[[22]](https://mackeeper.com/blog/post/610-macos-bundlore-adware-analysis/)

|  [C0010](https://attack.mitre.org/campaigns/C0010)   |  [C0010](https://attack.mitre.org/campaigns/C0010)   |

During [C0010](https://attack.mitre.org/campaigns/C0010), UNC3890 actors likely established a watering hole that was hosted on a login page of a legitimate Israeli shipping company that was active until at least November 2021.[[23]](https://www.mandiant.com/resources/blog/suspected-iranian-actor-targeting-israeli-shipping)

|  [G1012](https://attack.mitre.org/groups/G1012)   |  [CURIUM](https://attack.mitre.org/groups/G1012)   |

[CURIUM](https://attack.mitre.org/groups/G1012) has used strategic website compromise to infect victims with malware such as [IMAPLoader](https://attack.mitre.org/software/S1152).[[24]](https://www.pwc.com/gx/en/issues/cybersecurity/cyber-threat-intelligence/yellow-liderc-ships-its-scripts-delivers-imaploader-malware.html)

|  [G1034](https://attack.mitre.org/groups/G1034)   |  [Daggerfly](https://attack.mitre.org/groups/G1034)   |

[Daggerfly](https://attack.mitre.org/groups/G1034) has used strategic website compromise for initial access against victims.[[25]](https://www.welivesecurity.com/en/eset-research/evasive-panda-leverages-monlam-festival-target-tibetans/)

|  [G0070](https://attack.mitre.org/groups/G0070)   |  [Dark Caracal](https://attack.mitre.org/groups/G0070)   |

[Dark Caracal](https://attack.mitre.org/groups/G0070) leveraged a watering hole to serve up malicious code.[[26]](https://info.lookout.com/rs/051-ESQ-475/images/Lookout_Dark-Caracal_srr_20180118_us_v.1.0.pdf)

|  [G0012](https://attack.mitre.org/groups/G0012)   |  [Darkhotel](https://attack.mitre.org/groups/G0012)   |

[Darkhotel](https://attack.mitre.org/groups/G0012) used embedded iframes on hotel login portals to redirect selected victims to download malware.[[27]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/08070903/darkhotel_kl_07.11.pdf)

|  [G0035](https://attack.mitre.org/groups/G0035)   |  [Dragonfly](https://attack.mitre.org/groups/G0035)   |

[Dragonfly](https://attack.mitre.org/groups/G0035) has compromised targets via strategic web compromise (SWC) utilizing a custom exploit kit.[[28]](https://www.secureworks.com/research/resurgent-iron-liberty-targeting-energy-sector)[[29]](https://www.us-cert.gov/ncas/alerts/TA18-074A)[[30]](https://vblocalhost.com/uploads/VB2021-Slowik.pdf)

|  [G1006](https://attack.mitre.org/groups/G1006)   |  [Earth Lusca](https://attack.mitre.org/groups/G1006)   |

[Earth Lusca](https://attack.mitre.org/groups/G1006) has performed watering hole attacks.[[31]](https://www.trendmicro.com/content/dam/trendmicro/global/en/research/22/a/earth-lusca-employs-sophisticated-infrastructure-varied-tools-and-techniques/technical-brief-delving-deep-an-analysis-of-earth-lusca-operations.pdf)

|  [G0066](https://attack.mitre.org/groups/G0066)   |  [Elderwood](https://attack.mitre.org/groups/G0066)   |

[Elderwood](https://attack.mitre.org/groups/G0066) has delivered zero-day exploits and malware to victims by injecting malicious code into specific public Web pages visited by targets within a particular sector.[[32]](https://web.archive.org/web/20190717233006/http:/www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/the-elderwood-project.pdf)[[33]](https://www.csmonitor.com/USA/2012/0914/Stealing-US-business-secrets-Experts-ID-two-huge-cyber-gangs-in-China)[[34]](http://securityaffairs.co/wordpress/8528/hacking/elderwood-project-who-is-behind-op-aurora-and-ongoing-attacks.html)

|  [S0531](https://attack.mitre.org/software/S0531)   |  [Grandoreiro](https://attack.mitre.org/software/S0531)   |

[Grandoreiro](https://attack.mitre.org/software/S0531) has used compromised websites and Google Ads to bait victims into downloading its installer.[[35]](https://securelist.com/the-tetrade-brazilian-banking-malware/97779/)[[36]](https://securityintelligence.com/posts/grandoreiro-malware-now-targeting-banks-in-spain/)

|  [S0483](https://attack.mitre.org/software/S0483)   |  [IcedID](https://attack.mitre.org/software/S0483)   |

[IcedID](https://attack.mitre.org/software/S0483) has cloned legitimate websites/applications to distribute the malware.[[37]](https://www.trendmicro.com/en_us/research/22/l/icedid-botnet-distributors-abuse-google-ppc-to-distribute-malware.html)

|  [S0215](https://attack.mitre.org/software/S0215)   |  [KARAE](https://attack.mitre.org/software/S0215)   |

[KARAE](https://attack.mitre.org/software/S0215) was distributed through torrent file-sharing websites to South Korean victims, using a YouTube video downloader application as a lure.[[14]](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)

|  [G0032](https://attack.mitre.org/groups/G0032)   |  [Lazarus Group](https://attack.mitre.org/groups/G0032)   |

[Lazarus Group](https://attack.mitre.org/groups/G0032) delivered [RATANKBA](https://attack.mitre.org/software/S0241) and other malicious code to victims via a compromised legitimate website.[[38]](https://www.trendmicro.com/en_us/research/17/b/ratankba-watering-holes-against-enterprises.html)[[39]](https://blog.google/threat-analysis-group/new-campaign-targeting-security-researchers/)

|  [G0077](https://attack.mitre.org/groups/G0077)   |  [Leafminer](https://attack.mitre.org/groups/G0077)   |

[Leafminer](https://attack.mitre.org/groups/G0077) has infected victims using watering holes.[[40]](https://www.symantec.com/blogs/threat-intelligence/leafminer-espionage-middle-east)

|  [G0065](https://attack.mitre.org/groups/G0065)   |  [Leviathan](https://attack.mitre.org/groups/G0065)   |

[Leviathan](https://attack.mitre.org/groups/G0065) has infected victims using watering holes.[[41]](https://us-cert.cisa.gov/ncas/alerts/aa21-200a)

|  [S0451](https://attack.mitre.org/software/S0451)   |  [LoudMiner](https://attack.mitre.org/software/S0451)   |

[LoudMiner](https://attack.mitre.org/software/S0451) is typically bundled with pirated copies of Virtual Studio Technology (VST) for Windows and macOS.[[42]](https://www.welivesecurity.com/2019/06/20/loudminer-mining-cracked-vst-software/)

|  [G0095](https://attack.mitre.org/groups/G0095)   |  [Machete](https://attack.mitre.org/groups/G0095)   |

[Machete](https://attack.mitre.org/groups/G0095) has distributed [Machete](https://attack.mitre.org/software/S0409) through a fake blog website.[[43]](https://securelist.com/el-machete/66108/)

|  [G0059](https://attack.mitre.org/groups/G0059)   |  [Magic Hound](https://attack.mitre.org/groups/G0059)   |

[Magic Hound](https://attack.mitre.org/groups/G0059) has conducted watering-hole attacks through media and magazine websites.[[44]](https://www.clearskysec.com/wp-content/uploads/2020/08/The-Kittens-are-Back-in-Town-3.pdf)

|  [G1020](https://attack.mitre.org/groups/G1020)   |  [Mustard Tempest](https://attack.mitre.org/groups/G1020)   |

[Mustard Tempest](https://attack.mitre.org/groups/G1020) has used drive-by downloads for initial infection, often using fake browser updates as a lure.[[45]](https://www.proofpoint.com/us/blog/threat-insight/part-1-socgholish-very-real-threat-very-fake-update)[[46]](https://www.sentinelone.com/labs/socgholish-diversifies-and-expands-its-malware-staging-infrastructure-to-counter-defenders/)[[47]](https://redcanary.com/threat-detection-report/threats/socgholish/)[[48]](https://www.secureworks.com/research/threat-profiles/gold-prelude)

|  [C0016](https://attack.mitre.org/campaigns/C0016)   |  [Operation Dust Storm](https://attack.mitre.org/campaigns/C0016)   |

During [Operation Dust Storm](https://attack.mitre.org/campaigns/C0016), the threat actors used a watering hole attack on a popular software reseller to exploit the then-zero-day Internet Explorer vulnerability CVE-2014-0322.[[49]](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)

|  [G0040](https://attack.mitre.org/groups/G0040)   |  [Patchwork](https://attack.mitre.org/groups/G0040)   |

[Patchwork](https://attack.mitre.org/groups/G0040) has used watering holes to deliver files with exploits to initial victims.[[50]](http://www.symantec.com/connect/blogs/patchwork-cyberespionage-group-expands-targets-governments-wide-range-industries)[[51]](https://www.volexity.com/blog/2018/06/07/patchwork-apt-group-targets-us-think-tanks/)

|  [G0068](https://attack.mitre.org/groups/G0068)   |  [PLATINUM](https://attack.mitre.org/groups/G0068)   |

[PLATINUM](https://attack.mitre.org/groups/G0068) has sometimes used drive-by attacks against vulnerable browser plugins.[[52]](https://download.microsoft.com/download/2/2/5/225BFE3E-E1DE-4F5B-A77B-71200928D209/Platinum%20feature%20article%20-%20Targeted%20attacks%20in%20South%20and%20Southeast%20Asia%20April%202016.pdf)

|  [S0216](https://attack.mitre.org/software/S0216)   |  [POORAIM](https://attack.mitre.org/software/S0216)   |

[POORAIM](https://attack.mitre.org/software/S0216) has been delivered through compromised sites acting as watering holes.[[14]](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)

|  [G0056](https://attack.mitre.org/groups/G0056)   |  [PROMETHIUM](https://attack.mitre.org/groups/G0056)   |

[PROMETHIUM](https://attack.mitre.org/groups/G0056) has used watering hole attacks to deliver malicious versions of legitimate installers.[[53]](https://www.bitdefender.com/files/News/CaseStudies/study/353/Bitdefender-Whitepaper-StrongPity-APT.pdf)

|  [S0496](https://attack.mitre.org/software/S0496)   |  [REvil](https://attack.mitre.org/software/S0496)   |

[REvil](https://attack.mitre.org/software/S0496) has infected victim machines through compromised websites and exploit kits.[[54]](https://www.secureworks.com/research/revil-sodinokibi-ransomware)[[55]](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/mcafee-atr-analyzes-sodinokibi-aka-revil-ransomware-as-a-service-what-the-code-tells-us/)[[56]](https://www.picussecurity.com/blog/a-brief-history-and-further-technical-analysis-of-sodinokibi-ransomware)[[57]](https://www.secureworks.com/blog/revil-the-gandcrab-connection)

|  [G0048](https://attack.mitre.org/groups/G0048)   |  [RTM](https://attack.mitre.org/groups/G0048)   |

[RTM](https://attack.mitre.org/groups/G0048) has distributed its malware via the RIG and SUNDOWN exploit kits, as well as online advertising network Yandex.Direct.[[58]](https://www.welivesecurity.com/wp-content/uploads/2017/02/Read-The-Manual.pdf)[[59]](https://www.welivesecurity.com/2019/04/30/buhtrap-backdoor-ransomware-advertising-platform/)

|  [S1086](https://attack.mitre.org/software/S1086)   |  [Snip3](https://attack.mitre.org/software/S1086)   |

[Snip3](https://attack.mitre.org/software/S1086) has been delivered to targets via downloads from malicious domains.[[60]](https://telefonicatech.com/blog/snip3-investigacion-malware)

|  [S1124](https://attack.mitre.org/software/S1124)   |  [SocGholish](https://attack.mitre.org/software/S1124)   |

[SocGholish](https://attack.mitre.org/software/S1124) has been distributed through compromised websites with malicious content often masquerading as browser updates.[[45]](https://www.proofpoint.com/us/blog/threat-insight/part-1-socgholish-very-real-threat-very-fake-update)

|  [G0027](https://attack.mitre.org/groups/G0027)   |  [Threat Group-3390](https://attack.mitre.org/groups/G0027)   |

[Threat Group-3390](https://attack.mitre.org/groups/G0027) has extensively used strategic web compromises to target victims.[[61]](https://www.secureworks.com/research/threat-group-3390-targets-organizations-for-cyberespionage)[[62]](https://securelist.com/luckymouse-hits-national-data-center/86083/)

|  [G0134](https://attack.mitre.org/groups/G0134)   |  [Transparent Tribe](https://attack.mitre.org/groups/G0134)   |

[Transparent Tribe](https://attack.mitre.org/groups/G0134) has used websites with malicious hyperlinks and iframes to infect targeted victims with [Crimson](https://attack.mitre.org/software/S0115), [njRAT](https://attack.mitre.org/software/S0385), and other malicious tools.[[63]](https://www.proofpoint.com/sites/default/files/proofpoint-operation-transparent-tribe-threat-insight-en.pdf)[[64]](https://unit42.paloaltonetworks.com/unit42-projectm-link-found-between-pakistani-actor-and-operation-transparent-tribe/)[[65]](https://blog.talosintelligence.com/2021/05/transparent-tribe-infra-and-targeting.html)

|  [G0010](https://attack.mitre.org/groups/G0010)   |  [Turla](https://attack.mitre.org/groups/G0010)   |

[Turla](https://attack.mitre.org/groups/G0010) has infected victims using watering holes.[[66]](https://www.welivesecurity.com/wp-content/uploads/2020/05/ESET_Turla_ComRAT.pdf)[[67]](http://www.secureworks.com/research/threat-profiles/iron-hunter)

|  [S9050](https://attack.mitre.org/software/S9050)   |  [Vidar](https://attack.mitre.org/software/S9050)   |

[Vidar](https://attack.mitre.org/software/S9050) has been distributed via malicious advertisements when users searched for "Notepad++."[[68]](https://web.archive.org/web/20240517205011/https://www.logpoint.com/wp-content/uploads/2024/03/logpoint-etpr-a-comprehensive-overview-on-stealer-malware-families-1.pdf)

|  [G0124](https://attack.mitre.org/groups/G0124)   |  [Windigo](https://attack.mitre.org/groups/G0124)   |

[Windigo](https://attack.mitre.org/groups/G0124) has distributed Windows malware via drive-by downloads.[[69]](https://www.welivesecurity.com/2014/03/18/operation-windigo-the-vivisection-of-a-large-linux-server-side-credential-stealing-malware-campaign/)

|  [G0112](https://attack.mitre.org/groups/G0112)   |  [Windshift](https://attack.mitre.org/groups/G0112)   |

[Windshift](https://attack.mitre.org/groups/G0112) has used compromised websites to register custom URL schemes on a remote system.[[70]](https://objective-see.com/blog/blog_0x3B.html)

|  [G1035](https://attack.mitre.org/groups/G1035)   |  [Winter Vivern](https://attack.mitre.org/groups/G1035)   |

[Winter Vivern](https://attack.mitre.org/groups/G1035) created dedicated web pages mimicking legitimate government websites to deliver malicious fake anti-virus software.[[71]](https://cert.gov.ua/article/3761104)

## Mitigations

| ID  | Mitigation  | Description

|  [M1048](https://attack.mitre.org/mitigations/M1048)   |  [Application Isolation and Sandboxing](https://attack.mitre.org/mitigations/M1048)   |

Browser sandboxes can be used to mitigate some of the impact of exploitation, but sandbox escapes may still exist.[[72]](https://blogs.windows.com/msedgedev/2017/03/23/strengthening-microsoft-edge-sandbox/)[[73]](https://arstechnica.com/information-technology/2017/03/hack-that-escapes-vm-by-exploiting-edge-browser-fetches-105000-at-pwn2own/)

Other types of virtualization and application microsegmentation may also mitigate the impact of client-side exploitation. The risks of additional exploits and weaknesses in implementation may still exist for these types of systems.[[73]](https://arstechnica.com/information-technology/2017/03/hack-that-escapes-vm-by-exploiting-edge-browser-fetches-105000-at-pwn2own/)

|  [M1050](https://attack.mitre.org/mitigations/M1050)   |  [Exploit Protection](https://attack.mitre.org/mitigations/M1050)   |

Security applications that look for behavior used during exploitation such as Windows Defender Exploit Guard (WDEG) and the Enhanced Mitigation Experience Toolkit (EMET) can be used to mitigate some exploitation behavior.[[74]](https://blogs.technet.microsoft.com/srd/2017/08/09/moving-beyond-emet-ii-windows-defender-exploit-guard/) Control flow integrity checking is another way to potentially identify and stop a software exploit from occurring.[[75]](https://en.wikipedia.org/wiki/Control-flow_integrity) Many of these protections depend on the architecture and target application binary for compatibility.

|  [M1021](https://attack.mitre.org/mitigations/M1021)   |  [Restrict Web-Based Content](https://attack.mitre.org/mitigations/M1021)   |

Adblockers can help prevent malicious code served through ads from executing in the first place. Script blocking extensions can also help to prevent the execution of JavaScript.

Consider disabling browser push notifications from certain applications and browsers.[[76]](https://macsecurity.net/view/543-remove-guroshied-mac)[[77]](https://www.infosecinstitute.com/resources/security-awareness/malicious-push-notifications-is-that-a-real-or-fake-windows-defender-update/)[[78]](https://krebsonsecurity.com/2020/11/be-very-sparing-in-allowing-site-notifications/)

|  [M1051](https://attack.mitre.org/mitigations/M1051)   |  [Update Software](https://attack.mitre.org/mitigations/M1051)   |

Ensuring that all browsers and plugins are kept updated can help prevent the exploit phase of this technique. Use modern browsers with security features turned on.[[79]](https://www.proofpoint.com/us/blog/threat-insight/are-you-sure-your-browser-date-current-landscape-fake-browser-updates)

|  [M1017](https://attack.mitre.org/mitigations/M1017)   |  [User Training](https://attack.mitre.org/mitigations/M1017)   |

Train users to be aware of access or manipulation attempts by an adversary to reduce the risk of successful spearphishing, social engineering, and other techniques that involve user interaction.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0176](https://attack.mitre.org/detectionstrategies/DET0176)   |  [Drive-by Compromise — Behavior-based, Multi-platform Detection Strategy (T1189)](https://attack.mitre.org/detectionstrategies/DET0176)   |  [AN0498](https://attack.mitre.org/detectionstrategies/DET0176#AN0498)   |

Correlated evidence of anomalous browser/network behavior (suspicious external resource fetches and script injection patterns) followed by atypical child processes, ephemeral execution contexts, memory modification or process injection, and unexpected file drops. Defender sees network requests to previously unseen/suspicious domains or resources + browser process spawning unusual children or loading unsigned modules + file writes or registry changes shortly after those requests.

|  [AN0499](https://attack.mitre.org/detectionstrategies/DET0176#AN0499)   |

Correlated evidence of browser or webview fetches to uncommon domains or mutated JS resources (proxy/NGFW logs + Zeek/HTTP logs) followed by unexpected interpreters or script engines executing (python, ruby, sh) spawned from browser processes or user sessions, rapid on-disk staging in /tmp, and outbound connections that deviate from baseline. Defender sees: uncommon resource fetch → short-lived child process executions from user browser context → file writes in temp directories → anomalous outbound C2-like connections.

|  [AN0500](https://attack.mitre.org/detectionstrategies/DET0176#AN0500)   |

Correlated evidence where Safari/Chrome/WebKit-based processes issue network requests for uncommon or obfuscated JS resources followed by spawning of script interpreters, launchd or ad-hoc binaries, unusual child processes, or dynamic library loads into browser processes. Defender sees: proxy/HTTP logs with suspicious resource content + unifiedlogs/ASL showing browser/plugin crashes or extension loads + process events indicating child process creation and file writes to /var/folders or /tmp shortly after the fetch.

|  [AN0501](https://attack.mitre.org/detectionstrategies/DET0176#AN0501)   |

Post-compromise identity & session anomalies that follow a drive-by compromise: token reuse from new/unfamiliar IPs, anomalous sign-in patterns for previously inactive users, unexpected consent/grant events, or provisioning changes. Defender sees an endpoint/browser compromise (network + endpoint signals) followed by unusual IdP events: new refresh token issuance, consent/consent-grant events, odd MFA bypass patterns, or unusual OAuth client registrations.

## References

-    [Gaurav Sethi. (2021, December 14). The Dark Side of Web Push Notifications. Retrieved March 14, 2025.](https://viruspositive.com/resources/blogs/the-dark-side-of-web-push-notifications)
-    [Craig Schmugar. (2021, May 17). Scammers Impersonating Windows Defender to Push Malicious Windows Apps. Retrieved March 14, 2025.](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/scammers-impersonating-windows-defender-to-push-malicious-windows-apps/)
-    [Pieter Arntz. (2019, January 22). Browser push notifications: a feature asking to be abused. Retrieved March 14, 2025.](https://www.malwarebytes.com/blog/news/2019/01/browser-push-notifications-feature-asking-abused)
-    [Adair, S., Moran, N. (2012, May 15). Cyber Espionage & Strategic Web Compromises – Trusted Websites Serving Dangerous Results. Retrieved March 13, 2018.](http://blog.shadowserver.org/2012/05/15/cyber-espionage-strategic-web-compromises-trusted-websites-serving-dangerous-results/)
-    [Jeff Johnson, Fred Plan, Adrian Sanchez, Renato Fontana, Jake Nicastro, Dimiter Andonov, Marius Fodoreanu, Daniel Scott. (2023, April 20). 3CX Software Supply Chain Compromise Initiated by a Prior Software Supply Chain Compromise; Suspected North Korean Actor Responsible. Retrieved August 25, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/3cx-software-supply-chain-compromise/)
-    [AhnLab. (2018, June 23). Targeted attacks by Andariel Threat Group, a subgroup of the Lazarus. Retrieved September 29, 2021.](https://web.archive.org/web/20230213154832/http://download.ahnlab.com/global/brochure/%5BAnalysis%5DAndariel_Group.pdf)
-    [Chen, Joseph. (2018, July 16). New Andariel Reconnaissance Tactics Uncovered. Retrieved September 29, 2021.](https://www.trendmicro.com/en_us/research/18/g/new-andariel-reconnaissance-tactics-hint-at-next-targets.html)
-    [Grunzweig, J., Lee, B. (2016, January 22). New Attacks Linked to C0d0so0 Group. Retrieved August 2, 2018.](https://researchcenter.paloaltonetworks.com/2016/01/new-attacks-linked-to-c0d0s0-group/)
-    [Secureworks CTU. (2017, March 30). IRON TWILIGHT Supports Active Measures. Retrieved February 28, 2022.](https://www.secureworks.com/research/iron-twilight-supports-active-measures)
-    [Billy Leonard. (2023, April 19). Ukraine remains Russia’s biggest cyber focus in 2023. Retrieved March 1, 2024.](https://blog.google/threat-analysis-group/ukraine-remains-russias-biggest-cyber-focus-in-2023/)
-    [Foltýn, T. (2018, March 13). OceanLotus ships new backdoor using old tricks. Retrieved May 22, 2018.](https://www.welivesecurity.com/2018/03/13/oceanlotus-ships-new-backdoor/)
-    [Adair, S. and Lancaster, T. (2020, November 6). OceanLotus: Extending Cyber Espionage Operations Through Fake Websites. Retrieved November 20, 2020.](https://www.volexity.com/blog/2020/11/06/oceanlotus-extending-cyber-espionage-operations-through-fake-websites/)
-    [Raiu, C., and Ivanov, A. (2016, June 17). Operation Daybreak. Retrieved February 15, 2018.](https://securelist.com/operation-daybreak/75100/)
-    [FireEye. (2018, February 20). APT37 (Reaper): The Overlooked North Korean Actor. Retrieved November 17, 2024.](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)
-    [Cash, D., Grunzweig, J., Meltzer, M., Adair, S., Lancaster, T. (2021, August 17). North Korean APT InkySquid Infects Victims Using Browser Exploits. Retrieved September 30, 2021.](https://www.volexity.com/blog/2021/08/17/north-korean-apt-inkysquid-infects-victims-using-browser-exploits/)
-    [FireEye. (2018, October 03). APT38: Un-usual Suspects. Retrieved November 17, 2024.](https://services.google.com/fh/files/misc/apt38-un-usual-suspects.pdf)
-    [DHS/CISA. (2020, August 26). FASTCash 2.0: North Korea's BeagleBoyz Robbing Banks. Retrieved September 29, 2021.](https://us-cert.cisa.gov/ncas/alerts/aa20-239a)
-    [Esler, J., Lee, M., and Williams, C. (2014, October 14). Threat Spotlight: Group 72. Retrieved January 14, 2016.](http://blogs.cisco.com/security/talos/threat-spotlight-group-72)
-    [M.Léveille, M-E.. (2017, October 24). Bad Rabbit: Not‑Petya is back with improved ransomware. Retrieved January 28, 2021.](https://www.welivesecurity.com/2017/10/24/bad-rabbit-not-petya-back/)
-    [Mamedov, O. Sinitsyn, F. Ivanov, A.. (2017, October 24). Bad Rabbit ransomware. Retrieved January 28, 2021.](https://securelist.com/bad-rabbit-ransomware/82851/)
-    [DiMaggio, J. (2016, April 28). Tick cyberespionage group zeros in on Japan. Retrieved July 16, 2018.](https://www.symantec.com/connect/blogs/tick-cyberespionage-group-zeros-japan)
-    [Sushko, O. (2019, April 17). macOS Bundlore: Mac Virus Bypassing macOS Security Features. Retrieved June 30, 2020.](https://mackeeper.com/blog/post/610-macos-bundlore-adware-analysis/)
-    [Mandiant Israel Research Team. (2022, August 17). Suspected Iranian Actor Targeting Israeli Shipping, Healthcare, Government and Energy Sectors. Retrieved September 21, 2022.](https://www.mandiant.com/resources/blog/suspected-iranian-actor-targeting-israeli-shipping)
-    [PwC Threat Intelligence. (2023, October 25). Yellow Liderc ships its scripts and delivers IMAPLoader malware. Retrieved August 14, 2024.](https://www.pwc.com/gx/en/issues/cybersecurity/cyber-threat-intelligence/yellow-liderc-ships-its-scripts-delivers-imaploader-malware.html)
-    [Ahn Ho, Facundo Muñoz, & Marc-Etienne M.Léveillé. (2024, March 7). Evasive Panda leverages Monlam Festival to target Tibetans. Retrieved July 25, 2024.](https://www.welivesecurity.com/en/eset-research/evasive-panda-leverages-monlam-festival-target-tibetans/)
-    [Blaich, A., et al. (2018, January 18). Dark Caracal: Cyber-espionage at a Global Scale. Retrieved April 11, 2018.](https://info.lookout.com/rs/051-ESQ-475/images/Lookout_Dark-Caracal_srr_20180118_us_v.1.0.pdf)
-    [Kaspersky Lab's Global Research and Analysis Team. (2014, November). The Darkhotel APT A Story of Unusual Hospitality. Retrieved November 12, 2014.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/08070903/darkhotel_kl_07.11.pdf)
-    [Secureworks. (2019, July 24). Resurgent Iron Liberty Targeting Energy Sector. Retrieved August 12, 2020.](https://www.secureworks.com/research/resurgent-iron-liberty-targeting-energy-sector)
-    [US-CERT. (2018, March 16). Alert (TA18-074A): Russian Government Cyber Activity Targeting Energy and Other Critical Infrastructure Sectors. Retrieved June 6, 2018.](https://www.us-cert.gov/ncas/alerts/TA18-074A)
-    [Slowik, J. (2021, October). THE BAFFLING BERSERK BEAR: A DECADE’S ACTIVITY TARGETING CRITICAL INFRASTRUCTURE. Retrieved December 6, 2021.](https://vblocalhost.com/uploads/VB2021-Slowik.pdf)
-    [Chen, J., et al. (2022). Delving Deep: An Analysis of Earth Lusca’s Operations. Retrieved July 1, 2022.](https://www.trendmicro.com/content/dam/trendmicro/global/en/research/22/a/earth-lusca-employs-sophisticated-infrastructure-varied-tools-and-techniques/technical-brief-delving-deep-an-analysis-of-earth-lusca-operations.pdf)
-    [O'Gorman, G., and McDonald, G.. (2012, September 6). The Elderwood Project. Retrieved November 17, 2024.](https://web.archive.org/web/20190717233006/http:/www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/the-elderwood-project.pdf)
-    [Clayton, M.. (2012, September 14). Stealing US business secrets: Experts ID two huge cyber 'gangs' in China. Retrieved February 15, 2018.](https://www.csmonitor.com/USA/2012/0914/Stealing-US-business-secrets-Experts-ID-two-huge-cyber-gangs-in-China)
-    [Paganini, P. (2012, September 9). Elderwood project, who is behind Op. Aurora and ongoing attacks?. Retrieved February 13, 2018.](http://securityaffairs.co/wordpress/8528/hacking/elderwood-project-who-is-behind-op-aurora-and-ongoing-attacks.html)
-    [GReAT. (2020, July 14). The Tetrade: Brazilian banking malware goes global. Retrieved November 9, 2020.](https://securelist.com/the-tetrade-brazilian-banking-malware/97779/)
-    [Abramov, D. (2020, April 13). Grandoreiro Malware Now Targeting Banks in Spain. Retrieved November 12, 2020.](https://securityintelligence.com/posts/grandoreiro-malware-now-targeting-banks-in-spain/)
-    [Kenefick , I. (2022, December 23). IcedID Botnet Distributors Abuse Google PPC to Distribute Malware. Retrieved July 24, 2024.](https://www.trendmicro.com/en_us/research/22/l/icedid-botnet-distributors-abuse-google-ppc-to-distribute-malware.html)
-    [Trend Micro. (2017, February 27). RATANKBA: Delving into Large-scale Watering Holes against Enterprises. Retrieved May 22, 2018.](https://www.trendmicro.com/en_us/research/17/b/ratankba-watering-holes-against-enterprises.html)
-    [Weidemann, A. (2021, January 25). New campaign targeting security researchers. Retrieved December 20, 2021.](https://blog.google/threat-analysis-group/new-campaign-targeting-security-researchers/)
-    [Symantec Security Response. (2018, July 25). Leafminer: New Espionage Campaigns Targeting Middle Eastern Regions. Retrieved August 28, 2018.](https://www.symantec.com/blogs/threat-intelligence/leafminer-espionage-middle-east)

-    [CISA. (2021, July 19). (AA21-200A) Joint Cybersecurity Advisory – Tactics, Techniques, and Procedures of Indicted APT40 Actors Associated with China’s MSS Hainan State Security Department. Retrieved August 12, 2021.](https://us-cert.cisa.gov/ncas/alerts/aa21-200a)
-    [Malik, M. (2019, June 20). LoudMiner: Cross-platform mining in cracked VST software. Retrieved May 18, 2020.](https://www.welivesecurity.com/2019/06/20/loudminer-mining-cracked-vst-software/)
-    [Kaspersky Global Research and Analysis Team. (2014, August 20). El Machete. Retrieved September 13, 2019.](https://securelist.com/el-machete/66108/)
-    [ClearSky Research Team. (2020, August 1). The Kittens Are Back in Town 3 - Charming Kitten Campaign Evolved and Deploying Spear-Phishing link by WhatsApp. Retrieved April 21, 2021.](https://www.clearskysec.com/wp-content/uploads/2020/08/The-Kittens-are-Back-in-Town-3.pdf)
-    [Andrew Northern. (2022, November 22). SocGholish, a very real threat from a very fake update. Retrieved February 13, 2024.](https://www.proofpoint.com/us/blog/threat-insight/part-1-socgholish-very-real-threat-very-fake-update)
-    [Milenkoski, A. (2022, November 7). SocGholish Diversifies and Expands Its Malware Staging Infrastructure to Counter Defenders. Retrieved March 22, 2024.](https://www.sentinelone.com/labs/socgholish-diversifies-and-expands-its-malware-staging-infrastructure-to-counter-defenders/)
-    [Red Canary. (2024, March). Red Canary 2024 Threat Detection Report: SocGholish. Retrieved March 22, 2024.](https://redcanary.com/threat-detection-report/threats/socgholish/)
-    [Secureworks. (n.d.). GOLD PRELUDE . Retrieved March 22, 2024.](https://www.secureworks.com/research/threat-profiles/gold-prelude)
-    [Gross, J. (2016, February 23). Operation Dust Storm. Retrieved December 22, 2021.](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)
-    [Hamada, J.. (2016, July 25). Patchwork cyberespionage group expands targets from governments to wide range of industries. Retrieved August 17, 2016.](http://www.symantec.com/connect/blogs/patchwork-cyberespionage-group-expands-targets-governments-wide-range-industries)
-    [Meltzer, M, et al. (2018, June 07). Patchwork APT Group Targets US Think Tanks. Retrieved July 16, 2018.](https://www.volexity.com/blog/2018/06/07/patchwork-apt-group-targets-us-think-tanks/)
-    [Windows Defender Advanced Threat Hunting Team. (2016, April 29). PLATINUM: Targeted attacks in South and Southeast Asia. Retrieved February 15, 2018.](https://download.microsoft.com/download/2/2/5/225BFE3E-E1DE-4F5B-A77B-71200928D209/Platinum%20feature%20article%20-%20Targeted%20attacks%20in%20South%20and%20Southeast%20Asia%20April%202016.pdf)
-    [Tudorica, R. et al. (2020, June 30). StrongPity APT - Revealing Trojanized Tools, Working Hours and Infrastructure. Retrieved July 20, 2020.](https://www.bitdefender.com/files/News/CaseStudies/study/353/Bitdefender-Whitepaper-StrongPity-APT.pdf)
-    [Counter Threat Unit Research Team. (2019, September 24). REvil/Sodinokibi Ransomware. Retrieved August 4, 2020.](https://www.secureworks.com/research/revil-sodinokibi-ransomware)
-    [McAfee. (2019, October 2). McAfee ATR Analyzes Sodinokibi aka REvil Ransomware-as-a-Service – What The Code Tells Us. Retrieved August 4, 2020.](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/mcafee-atr-analyzes-sodinokibi-aka-revil-ransomware-as-a-service-what-the-code-tells-us/)
-    [Ozarslan, S. (2020, January 15). A Brief History of Sodinokibi. Retrieved August 5, 2020.](https://www.picussecurity.com/blog/a-brief-history-and-further-technical-analysis-of-sodinokibi-ransomware)
-    [Secureworks . (2019, September 24). REvil: The GandCrab Connection. Retrieved August 4, 2020.](https://www.secureworks.com/blog/revil-the-gandcrab-connection)
-    [Faou, M. and Boutin, J. (2017, February). Read The Manual: A Guide to the RTM Banking Trojan. Retrieved March 9, 2017.](https://www.welivesecurity.com/wp-content/uploads/2017/02/Read-The-Manual.pdf)
-    [ESET Research. (2019, April 30). Buhtrap backdoor and Buran ransomware distributed via major advertising platform. Retrieved May 11, 2020.](https://www.welivesecurity.com/2019/04/30/buhtrap-backdoor-ransomware-advertising-platform/)
-    [Jornet, A. (2021, December 23). Snip3, an investigation into malware. Retrieved September 19, 2023.](https://telefonicatech.com/blog/snip3-investigacion-malware)
-    [Dell SecureWorks Counter Threat Unit Threat Intelligence. (2015, August 5). Threat Group-3390 Targets Organizations for Cyberespionage. Retrieved August 18, 2018.](https://www.secureworks.com/research/threat-group-3390-targets-organizations-for-cyberespionage)
-    [Legezo, D. (2018, June 13). LuckyMouse hits national data center to organize country-level waterholing campaign. Retrieved August 18, 2018.](https://securelist.com/luckymouse-hits-national-data-center/86083/)
-    [Huss, D. (2016, March 1). Operation Transparent Tribe. Retrieved June 8, 2016.](https://www.proofpoint.com/sites/default/files/proofpoint-operation-transparent-tribe-threat-insight-en.pdf)
-    [Falcone, R. and Conant S. (2016, March 25). ProjectM: Link Found Between Pakistani Actor and Operation Transparent Tribe. Retrieved September 2, 2021.](https://unit42.paloaltonetworks.com/unit42-projectm-link-found-between-pakistani-actor-and-operation-transparent-tribe/)
-    [Malhotra, A. et al. (2021, May 13). Transparent Tribe APT expands its Windows malware arsenal. Retrieved September 2, 2021.](https://blog.talosintelligence.com/2021/05/transparent-tribe-infra-and-targeting.html)
-    [Faou, M. (2020, May). From Agent.btz to ComRAT v4: A ten-year journey. Retrieved June 15, 2020.](https://www.welivesecurity.com/wp-content/uploads/2020/05/ESET_Turla_ComRAT.pdf)
-    [Secureworks CTU. (n.d.). IRON HUNTER. Retrieved February 22, 2022.](http://www.secureworks.com/research/threat-profiles/iron-hunter)
-    [Khadgi, N. (n.d.). A Comprehensive Overview on Stealer Malware Families. Retrieved August 25, 2026.](https://web.archive.org/web/20240517205011/https://www.logpoint.com/wp-content/uploads/2024/03/logpoint-etpr-a-comprehensive-overview-on-stealer-malware-families-1.pdf)
-    [Bilodeau, O., Bureau, M., Calvet, J., Dorais-Joncas, A., Léveillé, M., Vanheuverzwijn, B. (2014, March 18). Operation Windigo – the vivisection of a large Linux server‑side credential‑stealing malware campaign. Retrieved February 10, 2021.](https://www.welivesecurity.com/2014/03/18/operation-windigo-the-vivisection-of-a-large-linux-server-side-credential-stealing-malware-campaign/)
-    [Wardle, Patrick. (2018, December 20). Middle East Cyber-Espionage analyzing WindShift's implant: OSX.WindTail (part 1). Retrieved October 3, 2019.](https://objective-see.com/blog/blog_0x3B.html)
-    [CERT-UA. (2023, February 1). UAC-0114 aka Winter Vivern to target Ukrainian and Polish GOV entities (CERT-UA#5909). Retrieved July 29, 2024.](https://cert.gov.ua/article/3761104)
-    [Cowan, C. (2017, March 23). Strengthening the Microsoft Edge Sandbox. Retrieved March 12, 2018.](https://blogs.windows.com/msedgedev/2017/03/23/strengthening-microsoft-edge-sandbox/)
-    [Goodin, D. (2017, March 17). Virtual machine escape fetches $105,000 at Pwn2Own hacking contest - updated. Retrieved March 12, 2018.](https://arstechnica.com/information-technology/2017/03/hack-that-escapes-vm-by-exploiting-edge-browser-fetches-105000-at-pwn2own/)
-    [Nunez, N. (2017, August 9). Moving Beyond EMET II – Windows Defender Exploit Guard. Retrieved March 12, 2018.](https://blogs.technet.microsoft.com/srd/2017/08/09/moving-beyond-emet-ii-windows-defender-exploit-guard/)
-    [Wikipedia. (2018, January 11). Control-flow integrity. Retrieved March 12, 2018.](https://en.wikipedia.org/wiki/Control-flow_integrity)
-    [David Balaban. (2022, October 7). Remove Guroshied virus popup from Mac. Retrieved March 14, 2025.](https://macsecurity.net/view/543-remove-guroshied-mac)
-    [Dan Virgillito. (2022, January 27). Malicious push notifications: Is that a real or fake Windows Defender update?. Retrieved March 14, 2025.](https://www.infosecinstitute.com/resources/security-awareness/malicious-push-notifications-is-that-a-real-or-fake-windows-defender-update/)
-    [Frank Angiolelli, Indelible LLC, Malwarebytes, McAfee, Norton, Pieter Arntz, PushWelcome. (2020, November 17). Be Very Sparing in Allowing Site Notifications. Retrieved March 14, 2025.](https://krebsonsecurity.com/2020/11/be-very-sparing-in-allowing-site-notifications/)
-    [Dusty Miller. (2023, October 17). Are You Sure Your Browser is Up to Date? The Current Landscape of Fake Browser Updates . Retrieved February 13, 2024.](https://www.proofpoint.com/us/blog/threat-insight/are-you-sure-your-browser-date-current-landscape-fake-browser-updates)

×
