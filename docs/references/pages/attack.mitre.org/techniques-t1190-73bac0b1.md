---
url: "https://attack.mitre.org/techniques/T1190/"
title: "Exploit Public-Facing Application, Technique T1190 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:38:18Z"
via: "live"
sha256: "858f29b2ce3629344c5e735913522edf94a3a6c9e22d46123a6af1b468d311d8"
wayback: "https://web.archive.org/web/20261019204619/https://attack.mitre.org/techniques/T1190/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Exploit Public-Facing Application, Technique T1190 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1190/>
> Snapshot: <https://web.archive.org/web/20261019204619/https://attack.mitre.org/techniques/T1190/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Exploit Public-Facing Application

#  Exploit Public-Facing Application

Adversaries may attempt to exploit a weakness in an Internet-facing host or system to initially access a network. The weakness in the system can be a software bug, a temporary glitch, or a misconfiguration.

Exploited applications are often websites/web servers, but can also include databases (like SQL), standard services (like SMB or SSH), network device administration and management protocols (like SNMP and Smart Install), and any other system with Internet-accessible open sockets.[[1]](https://nvd.nist.gov/vuln/detail/CVE-2016-6662)[[2]](https://www.cisecurity.org/advisory/multiple-vulnerabilities-in-microsoft-windows-smb-server-could-allow-for-remote-code-execution/)[[3]](https://us-cert.cisa.gov/ncas/alerts/TA18-106A)[[4]](https://community.cisco.com/t5/security-blogs/attackers-continue-to-target-legacy-devices/ba-p/4169954)[[5]](https://nvd.nist.gov/vuln/detail/CVE-2014-7169) On ESXi infrastructure, adversaries may exploit exposed OpenSLP services; they may alternatively exploit exposed VMware vCenter servers.[[6]](https://www.recordedfuture.com/blog/esxiargs-ransomware-targets-vmware-esxi-openslp-servers)[[7]](https://arstechnica.com/information-technology/2021/02/armed-with-exploits-hackers-on-the-prowl-for-a-critical-vmware-vulnerability/) Depending on the flaw being exploited, this may also involve [Exploitation for Stealth](https://attack.mitre.org/techniques/T1211) or [Exploitation for Client Execution](https://attack.mitre.org/techniques/T1203).

If an application is hosted on cloud-based infrastructure and/or is containerized, then exploiting it may lead to compromise of the underlying instance or container. This can allow an adversary a path to access the cloud or container APIs (e.g., via the [Cloud Instance Metadata API](https://attack.mitre.org/techniques/T1552/005)), exploit container host access via [Escape to Host](https://attack.mitre.org/techniques/T1611), or take advantage of weak identity and access management policies.

Adversaries may also exploit edge network infrastructure and related appliances, specifically targeting devices that do not support robust host-based defenses.[[8]](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)[[9]](https://www.wired.com/story/russia-ukraine-cyberattacks-mandiant/)

For websites and databases, the OWASP top 10 and CWE top 25 highlight the most common web-based vulnerabilities.[[10]](https://www.owasp.org/index.php/Category:OWASP_Top_Ten_Project)[[11]](https://cwe.mitre.org/top25/index.html)

ID: T1190

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Initial Access](https://attack.mitre.org/tactics/TA0001)

ⓘ

Platforms: Containers, ESXi, IaaS, Linux, Network Devices, Windows, macOS

Contributors: Don Le, Stifel Financial; Praetorian; Yossi Weizman, Azure Defender Research Team

Version: 2.8

Created: 18 April 2018

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1190/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1190/)

## Procedure Examples

| ID  | Name  | Description

|  [G1030](https://attack.mitre.org/groups/G1030)   |  [Agrius](https://attack.mitre.org/groups/G1030)   |

[Agrius](https://attack.mitre.org/groups/G1030) exploits public-facing applications for initial access to victim environments. Examples include widespread attempts to exploit CVE-2018-13379 in FortiOS devices and SQL injection activity.[[12]](https://assets.sentinelone.com/sentinellabs/evol-agrius)

|  [C0064](https://attack.mitre.org/campaigns/C0064)   |  [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064)   |

During the [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064), the adversary tasked Claude Code to exploit a vulnerability in an internet-facing server at the Mexico Federal Tax Authority to achieve remote command execution.[[13]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

During the [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064), the adversary tasked Claude Code with planting a webshell on the victims public-facing portal.[[13]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

|  [C0062](https://attack.mitre.org/campaigns/C0062)   |  [Anthropic AI-orchestrated Campaign](https://attack.mitre.org/campaigns/C0062)   |

During the [Anthropic AI-orchestrated Campaign](https://attack.mitre.org/campaigns/C0062), the adversary used Claude Code to deploy a custom exploit payload targeting an identified SSRF vulnerability to gain initial access to a targeted environment.[[14]](https://assets.anthropic.com/m/ec212e6566a0d47/original/Disrupting-the-first-reported-AI-orchestrated-cyber-espionage-campaign.pdf)

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[APT28](https://attack.mitre.org/groups/G0007) has used a variety of public exploits, including CVE 2020-0688 and CVE 2020-17144, to gain execution on vulnerable Microsoft Exchange; they have also conducted SQL injection attacks against external websites.[[15]](https://www.justice.gov/opa/page/file/1098481/download)[[16]](https://media.defense.gov/2021/Jul/01/2002753896/-1/-1/1/CSA_GRU_GLOBAL_BRUTE_FORCE_CAMPAIGN_UOO158036-21.PDF)

|  [G0016](https://attack.mitre.org/groups/G0016)   |  [APT29](https://attack.mitre.org/groups/G0016)   |

[APT29](https://attack.mitre.org/groups/G0016) has exploited CVE-2019-19781 for Citrix, CVE-2019-11510 for Pulse Secure VPNs, CVE-2018-13379 for FortiGate VPNs, and CVE-2019-9670 in Zimbra software to gain access.[[17]](https://www.ncsc.gov.uk/files/Advisory-further-TTPs-associated-with-SVR-cyber-actors.pdf)[[18]](https://www.ncsc.gov.uk/files/Advisory-APT29-targets-COVID-19-vaccine-development-V1-1.pdf)

|  [G0087](https://attack.mitre.org/groups/G0087)   |  [APT39](https://attack.mitre.org/groups/G0087)   |

[APT39](https://attack.mitre.org/groups/G0087) has used SQL injection for initial compromise.[[19]](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/chafer-latest-attacks-reveal-heightened-ambitions)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) exploited CVE-2020-10189 against Zoho ManageEngine Desktop Central through unsafe deserialization, and CVE-2019-19781 to compromise Citrix Application Delivery Controllers (ADC) and gateway devices.[[20]](https://www.fireeye.com/blog/threat-research/2020/03/apt41-initiates-global-intrusion-campaign-using-multiple-exploits.html) [APT41](https://attack.mitre.org/groups/G0096) leveraged vulnerabilities such as ProxyLogon exploitation or SQL injection for initial access.[[21]](https://www.group-ib.com/blog/apt41-world-tour-2021/) [APT41](https://attack.mitre.org/groups/G0096) exploited CVE-2021-26855 against a vulnerable Microsoft Exchange Server to gain initial access to the victim network.[[22]](https://medium.com/@DCSO_CyTec/apt41-the-spy-who-failed-to-encrypt-me-24fc0f49cad1)

|  [G1023](https://attack.mitre.org/groups/G1023)   |  [APT5](https://attack.mitre.org/groups/G1023)   |

[APT5](https://attack.mitre.org/groups/G1023) has exploited vulnerabilities in externally facing software and devices including Pulse Secure VPNs and Citrix Application Delivery Controllers.[[23]](https://www.mandiant.com/resources/blog/suspected-apt-actors-leverage-bypass-techniques-pulse-secure-zero-day)[[24]](https://www.mandiant.com/resources/blog/updates-on-chinese-apt-compromising-pulse-secure-vpn-devices)[[25]](https://media.defense.gov/2022/Dec/13/2003131586/-1/-1/0/CSA-APT5-CITRIXADC-V1.PDF) [[26]](https://query.prod.cms.rt.microsoft.com/cms/api/am/binary/RW1aFyW)

|  [C0046](https://attack.mitre.org/campaigns/C0046)   |  [ArcaneDoor](https://attack.mitre.org/campaigns/C0046)   |

[ArcaneDoor](https://attack.mitre.org/campaigns/C0046) abused WebVPN traffic to targeted devices to achieve unauthorized remote code execution.[[27]](https://www.cyber.gc.ca/en/news-events/cyber-activity-impacting-cisco-asa-vpns)

|  [G0001](https://attack.mitre.org/groups/G0001)   |  [Axiom](https://attack.mitre.org/groups/G0001)   |

[Axiom](https://attack.mitre.org/groups/G0001) has been observed using SQL injection to gain access to systems.[[28]](https://web.archive.org/web/20230115144216/http://www.novetta.com/wp-content/uploads/2014/11/Executive_Summary-Final_1.pdf)[[29]](http://blogs.cisco.com/security/talos/threat-spotlight-group-72)

|  [G0135](https://attack.mitre.org/groups/G0135)   |  [BackdoorDiplomacy](https://attack.mitre.org/groups/G0135)   |

[BackdoorDiplomacy](https://attack.mitre.org/groups/G0135) has exploited CVE-2020-5902, an F5 BIP-IP vulnerability, to drop a Linux backdoor. [BackdoorDiplomacy](https://attack.mitre.org/groups/G0135) has also exploited mis-configured Plesk servers.[[30]](https://www.welivesecurity.com/2021/06/10/backdoordiplomacy-upgrading-quarian-turian/)

|  [G1043](https://attack.mitre.org/groups/G1043)   |  [BlackByte](https://attack.mitre.org/groups/G1043)   |

[BlackByte](https://attack.mitre.org/groups/G1043) exploited vulnerabilities such as ProxyLogon and ProxyShell for initial access to victim environments.[[31]](https://www.ic3.gov/CSA/2022/220211.pdf)[[32]](https://www.picussecurity.com/resource/ttps-used-by-blackbyte-ransomware-targeting-critical-infrastructure)[[33]](https://www.security.com/threat-intelligence/blackbyte-exbyte-ransomware)[[34]](https://www.microsoft.com/en-us/security/blog/2023/07/06/the-five-day-job-a-blackbyte-ransomware-intrusion-case-study/)

|  [G0098](https://attack.mitre.org/groups/G0098)   |  [BlackTech](https://attack.mitre.org/groups/G0098)   |

[BlackTech](https://attack.mitre.org/groups/G0098) has exploited a buffer overflow vulnerability in Microsoft Internet Information Services (IIS) 6.0, CVE-2017-7269, in order to establish a new HTTP or command and control (C2) server.[[35]](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)

|  [G0108](https://attack.mitre.org/groups/G0108)   |  [Blue Mockingbird](https://attack.mitre.org/groups/G0108)   |

[Blue Mockingbird](https://attack.mitre.org/groups/G0108) has gained initial access by exploiting CVE-2019-18935, a vulnerability within Telerik UI for ASP.NET AJAX.[[36]](https://redcanary.com/blog/blue-mockingbird-cryptominer/)

|  [S1184](https://attack.mitre.org/software/S1184)   |  [BOLDMOVE](https://attack.mitre.org/software/S1184)   |

[BOLDMOVE](https://attack.mitre.org/software/S1184) is associated with exploitation of CVE-2022-49475 in FortiOS.[[37]](https://cloud.google.com/blog/topics/threat-intelligence/chinese-actors-exploit-fortios-flaw/)

|  [C0017](https://attack.mitre.org/campaigns/C0017)   |  [C0017](https://attack.mitre.org/campaigns/C0017)   |

During [C0017](https://attack.mitre.org/campaigns/C0017), [APT41](https://attack.mitre.org/groups/G0096) exploited CVE-2021-44207 in the USAHerds application and CVE-2021-44228 in Log4j, as well as other .NET deserialization, SQL injection, and directory traversal vulnerabilities to gain initial access.[[38]](https://www.mandiant.com/resources/apt41-us-state-governments)

|  [C0018](https://attack.mitre.org/campaigns/C0018)   |  [C0018](https://attack.mitre.org/campaigns/C0018)   |

During [C0018](https://attack.mitre.org/campaigns/C0018), the threat actors exploited VMWare Horizon Unified Access Gateways that were vulnerable to several Log4Shell vulnerabilities, including CVE-2021-44228, CVE-2021-45046, CVE-2021-45105, and CVE-2021-44832.[[39]](https://blog.talosintelligence.com/avoslocker-new-arsenal/)

|  [C0027](https://attack.mitre.org/campaigns/C0027)   |  [C0027](https://attack.mitre.org/campaigns/C0027)   |

During [C0027](https://attack.mitre.org/campaigns/C0027), [Scattered Spider](https://attack.mitre.org/groups/G1015) exploited CVE-2021-35464 in the ForgeRock Open Access Management (OpenAM) application server to gain initial access.[[40]](https://www.crowdstrike.com/blog/analysis-of-intrusion-campaign-targeting-telecom-and-bpo-companies/)

|  [G1021](https://attack.mitre.org/groups/G1021)   |  [Cinnamon Tempest](https://attack.mitre.org/groups/G1021)   |

[Cinnamon Tempest](https://attack.mitre.org/groups/G1021) has exploited multiple unpatched vulnerabilities for initial access including vulnerabilities in Microsoft Exchange, Manage Engine AdSelfService Plus, Confluence, and Log4j.[[41]](https://www.microsoft.com/en-us/security/blog/2022/05/09/ransomware-as-a-service-understanding-the-cybercrime-gig-economy-and-how-to-protect-yourself/)[[42]](https://www.microsoft.com/en-us/security/blog/2021/12/11/guidance-for-preventing-detecting-and-hunting-for-cve-2021-44228-log4j-2-exploitation/)[[43]](https://blog.sygnia.co/revealing-emperor-dragonfly-a-chinese-ransomware-group)[[44]](https://www.secureworks.com/research/bronze-starlight-ransomware-operations-use-hui-loader)

|  [S1105](https://attack.mitre.org/software/S1105)   |  [COATHANGER](https://attack.mitre.org/software/S1105)   |

[COATHANGER](https://attack.mitre.org/software/S1105) is installed following exploitation of a vulnerable FortiGate device. [[45]](https://www.ncsc.nl/binaries/ncsc/documenten/publicaties/2024/februari/6/mivd-aivd-advisory-coathanger-tlp-clear/TLP-CLEAR+MIVD+AIVD+Advisory+COATHANGER.pdf)

|  [C0029](https://attack.mitre.org/campaigns/C0029)   |  [Cutting Edge](https://attack.mitre.org/campaigns/C0029)   |

During [Cutting Edge](https://attack.mitre.org/campaigns/C0029), threat actors exploited CVE-2023-46805 and CVE-2024-21887 in Ivanti Connect Secure VPN appliances to enable authentication bypass and command injection. A server-side request forgery (SSRF) vulnerability, CVE-2024-21893, was identified later and used to bypass mitigations for the initial two vulnerabilities by chaining with CVE-2024-21887.[[46]](https://www.mandiant.com/resources/blog/suspected-apt-targets-ivanti-zero-day)[[47]](https://www.volexity.com/blog/2024/01/10/active-exploitation-of-two-zero-day-vulnerabilities-in-ivanti-connect-secure-vpn/)[[48]](https://www.volexity.com/blog/2024/01/15/ivanti-connect-secure-vpn-exploitation-goes-global/)[[49]](https://www.mandiant.com/resources/blog/investigating-ivanti-zero-day-exploitation)[[50]](https://www.mandiant.com/resources/blog/investigating-ivanti-exploitation-persistence)

|  [G0035](https://attack.mitre.org/groups/G0035)   |  [Dragonfly](https://attack.mitre.org/groups/G0035)   |

[Dragonfly](https://attack.mitre.org/groups/G0035) has conducted SQL injection attacks, exploited vulnerabilities CVE-2019-19781 and CVE-2020-0688 for Citrix and MS Exchange, and CVE-2018-13379 for Fortinet VPNs.[[51]](https://www.cisa.gov/uscert/ncas/alerts/aa20-296a#revisions)

|  [G1006](https://attack.mitre.org/groups/G1006)   |  [Earth Lusca](https://attack.mitre.org/groups/G1006)   |

[Earth Lusca](https://attack.mitre.org/groups/G1006) has compromised victims by directly exploiting vulnerabilities of public-facing servers, including those associated with Microsoft Exchange and Oracle GlassFish.[[52]](https://www.trendmicro.com/content/dam/trendmicro/global/en/research/22/a/earth-lusca-employs-sophisticated-infrastructure-varied-tools-and-techniques/technical-brief-delving-deep-an-analysis-of-earth-lusca-operations.pdf)

|  [G1003](https://attack.mitre.org/groups/G1003)   |  [Ember Bear](https://attack.mitre.org/groups/G1003)   |

[Ember Bear](https://attack.mitre.org/groups/G1003) gains initial access to victim environments by exploiting external-facing services. Examples include exploitation of CVE-2021-26084 in Confluence servers; CVE-2022-41040, ProxyShell, and other vulnerabilities in Microsoft Exchange; and multiple vulnerabilities in open-source platforms such as content management systems.[[53]](https://www.microsoft.com/en-us/security/blog/2023/06/14/cadet-blizzard-emerges-as-a-novel-and-distinct-russian-threat-actor/)[[54]](https://www.cisa.gov/sites/default/files/2024-09/aa24-249a-russian-military-cyber-actors-target-us-and-global-critical-infrastructure.pdf)

|  [G1016](https://attack.mitre.org/groups/G1016)   |  [FIN13](https://attack.mitre.org/groups/G1016)   |

[FIN13](https://attack.mitre.org/groups/G1016) has exploited known vulnerabilities such as CVE-2017-1000486 (Primefaces Application Expression Language Injection), CVE-2015-7450 (WebSphere Application Server SOAP Deserialization Exploit), CVE-2010-5326 (SAP NewWeaver Invoker Servlet Exploit), and EDB-ID-24963 (SAP NetWeaver ConfigServlet Remote Code Execution) to gain initial access.[[55]](https://www.mandiant.com/resources/blog/fin13-cybercriminal-mexico)[[56]](https://web.archive.org/web/20220105132433/https://f.hubspotusercontent30.net/hubfs/8776530/Sygnia-%20Elephant%20Beetle_Jan2022.pdf)

|  [G0046](https://attack.mitre.org/groups/G0046)   |  [FIN7](https://attack.mitre.org/groups/G0046)   |

[FIN7](https://attack.mitre.org/groups/G0046) has compromised targeted organizations through exploitation of CVE-2021-31207 in Exchange.[[41]](https://www.microsoft.com/en-us/security/blog/2022/05/09/ransomware-as-a-service-understanding-the-cybercrime-gig-economy-and-how-to-protect-yourself/)

|  [C0053](https://attack.mitre.org/campaigns/C0053)   |  [FLORAHOX Activity](https://attack.mitre.org/campaigns/C0053)   |

[FLORAHOX Activity](https://attack.mitre.org/campaigns/C0053) has exploited and infected vulnerable routers to recruit additional network devices into the ORB.[[57]](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-orb-networks)

|  [G0117](https://attack.mitre.org/groups/G0117)   |  [Fox Kitten](https://attack.mitre.org/groups/G0117)   |

[Fox Kitten](https://attack.mitre.org/groups/G0117) has exploited known vulnerabilities in Fortinet, PulseSecure, and Palo Alto VPN appliances.[[58]](https://www.clearskysec.com/fox-kitten/)[[59]](https://www.dragos.com/threat/parisite/)[[60]](https://www.crowdstrike.com/blog/who-is-pioneer-kitten/)[[61]](https://us-cert.cisa.gov/ncas/alerts/aa20-259a)[[62]](https://www.clearskysec.com/wp-content/uploads/2020/12/Pay2Kitten.pdf)

|  [C0041](https://attack.mitre.org/campaigns/C0041)   |  [FrostyGoop Incident](https://attack.mitre.org/campaigns/C0041)   |

[FrostyGoop Incident](https://attack.mitre.org/campaigns/C0041) was likely enabled by the adversary exploiting an unknown vulnerability in an external-facing router.[[63]](https://hub.dragos.com/hubfs/Reports/Dragos-FrostyGoop-ICS-Malware-Intel-Brief-0724_r2.pdf)

|  [G0093](https://attack.mitre.org/groups/G0093)   |  [GALLIUM](https://attack.mitre.org/groups/G0093)   |

[GALLIUM](https://attack.mitre.org/groups/G0093) exploited a publicly-facing servers including Wildfly/JBoss servers to gain access to the network.[[64]](https://www.cybereason.com/blog/operation-soft-cell-a-worldwide-campaign-against-telecommunications-providers)[[65]](https://www.microsoft.com/security/blog/2019/12/12/gallium-targeting-global-telecom/)

|  [G0115](https://attack.mitre.org/groups/G0115)   |  [GOLD SOUTHFIELD](https://attack.mitre.org/groups/G0115)   |

[GOLD SOUTHFIELD](https://attack.mitre.org/groups/G0115) has exploited Oracle WebLogic vulnerabilities for initial compromise.[[66]](https://www.secureworks.com/research/revil-sodinokibi-ransomware)

|  [G0125](https://attack.mitre.org/groups/G0125)   |  [HAFNIUM](https://attack.mitre.org/groups/G0125)   |

[HAFNIUM](https://attack.mitre.org/groups/G0125) has exploited multiple vulnerabilities to compromise edge devices and on-premises versions of Microsoft Exchange Server.[[67]](https://www.microsoft.com/security/blog/2021/03/02/hafnium-targeting-exchange-servers/)[[68]](https://www.volexity.com/blog/2021/03/02/active-exploitation-of-microsoft-exchange-zero-day-vulnerabilities/)[[69]](https://www.fireeye.com/blog/threat-research/2021/03/detection-response-to-exploitation-of-microsoft-exchange-zero-day-vulnerabilities.html)[[70]](https://www.microsoft.com/security/blog/2022/04/12/tarrask-malware-uses-scheduled-tasks-for-defense-evasion/)[[42]](https://www.microsoft.com/en-us/security/blog/2021/12/11/guidance-for-preventing-detecting-and-hunting-for-cve-2021-44228-log4j-2-exploitation/)[[71]](https://www.microsoft.com/en-us/security/blog/2025/03/05/silk-typhoon-targeting-it-supply-chain/)

|  [S0224](https://attack.mitre.org/software/S0224)   |  [Havij](https://attack.mitre.org/software/S0224)   |

[Havij](https://attack.mitre.org/software/S0224) is used to automate SQL injection.[[72]](https://blog.checkpoint.com/2015/05/14/analysis-havij-sql-injection-tool/)

|  [C0038](https://attack.mitre.org/campaigns/C0038)   |  [HomeLand Justice](https://attack.mitre.org/campaigns/C0038)   |

For [HomeLand Justice](https://attack.mitre.org/campaigns/C0038), threat actors exploited CVE-2019-0604 in Microsoft SharePoint for initial access.[[73]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa22-264a)

|  [G1032](https://attack.mitre.org/groups/G1032)   |  [INC Ransom](https://attack.mitre.org/groups/G1032)   |

[INC Ransom](https://attack.mitre.org/groups/G1032) has exploited known vulnerabilities including CVE-2023-3519 in Citrix NetScaler for initial access.[[74]](https://socradar.io/dark-web-profile-inc-ransom/)[[75]](https://www.sentinelone.com/anthology/inc-ransom/)

|  [G0004](https://attack.mitre.org/groups/G0004)   |  [Ke3chang](https://attack.mitre.org/groups/G0004)   |

[Ke3chang](https://attack.mitre.org/groups/G0004) has compromised networks by exploiting Internet-facing applications, including vulnerable Microsoft Exchange and SharePoint servers.[[76]](https://www.microsoft.com/security/blog/2021/12/06/nickel-targeting-government-organizations-across-latin-america-and-europe)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has exploited various vulnerabilities for initial access, including Microsoft Exchange vulnerability CVE-2020-0688.[[77]](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)

|  [G0065](https://attack.mitre.org/groups/G0065)   |  [Leviathan](https://attack.mitre.org/groups/G0065)   |

[Leviathan](https://attack.mitre.org/groups/G0065) has used exploits against publicly-disclosed vulnerabilities for initial access into victim networks.[[78]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa24-190a)

|  [C0049](https://attack.mitre.org/campaigns/C0049)   |  [Leviathan Australian Intrusions](https://attack.mitre.org/campaigns/C0049)   |

[Leviathan](https://attack.mitre.org/groups/G0065) exploited public-facing web applications and appliances for initial access during [Leviathan Australian Intrusions](https://attack.mitre.org/campaigns/C0049).[[78]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa24-190a)

|  [G0059](https://attack.mitre.org/groups/G0059)   |  [Magic Hound](https://attack.mitre.org/groups/G0059)   |

[Magic Hound](https://attack.mitre.org/groups/G0059) has exploited the Log4j utility (CVE-2021-44228), on-premises MS Exchange servers via "ProxyShell" (CVE-2021-34473, CVE-2021-34523, CVE-2021-31207), and Fortios SSL VPNs (CVE-2018-13379).[[79]](https://research.checkpoint.com/2022/apt35-exploits-log4j-vulnerability-to-distribute-new-modular-powershell-toolkit/)[[80]](https://thedfirreport.com/2022/03/21/apt35-automates-initial-access-using-proxyshell)[[81]](https://www.cybereason.com/blog/research/powerless-trojan-iranian-apt-phosphorus-adds-new-powershell-backdoor-for-espionage)[[82]](https://thedfirreport.com/2021/11/15/exchange-exploit-leads-to-domain-wide-ransomware/)[[83]](https://www.microsoft.com/en-us/security/blog/2021/11/16/evolving-trends-in-iranian-threat-actor-activity-mstic-presentation-at-cyberwarcon-2021)[[42]](https://www.microsoft.com/en-us/security/blog/2021/12/11/guidance-for-preventing-detecting-and-hunting-for-cve-2021-44228-log4j-2-exploitation/)

|  [G1051](https://attack.mitre.org/groups/G1051)   |  [Medusa Group](https://attack.mitre.org/groups/G1051)   |

[Medusa Group](https://attack.mitre.org/groups/G1051) has leveraged public facing vulnerabilities in their campaigns against victim organizations to gain initial access.[[84]](https://unit42.paloaltonetworks.com/medusa-ransomware-escalation-new-leak-site/)[[85]](https://www.security.com/threat-intelligence/medusa-ransomware-attacks) [Medusa Group](https://attack.mitre.org/groups/G1051) has also utilized CVE-2024-1709 in ScreenConnect, and CVE-2023-48788 in Fortinet EMS for initial access to victim environments.[[86]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-071a)

|  [G0045](https://attack.mitre.org/groups/G0045)   |  [menuPass](https://attack.mitre.org/groups/G0045)   |

[menuPass](https://attack.mitre.org/groups/G0045) has leveraged vulnerabilities in Pulse Secure VPNs to hijack sessions.[[87]](https://securelist.com/apt10-sophisticated-multi-layered-loader-ecipekac-discovered-in-a41apt-campaign/101519/)

|  [G1054](https://attack.mitre.org/groups/G1054)   |  [MirrorFace](https://attack.mitre.org/groups/G1054)   |

[MirrorFace](https://attack.mitre.org/groups/G1054) has exploited vulnerabilities in Fortigate and Array AG devices for initial access.[[88]](https://blogs.jpcert.or.jp/en/2024/07/mirrorface-attack-against-japanese-organisations.html)

|  [G1009](https://attack.mitre.org/groups/G1009)   |  [Moses Staff](https://attack.mitre.org/groups/G1009)   |

[Moses Staff](https://attack.mitre.org/groups/G1009) has exploited known vulnerabilities in public-facing infrastructure such as Microsoft Exchange Servers.[[89]](https://research.checkpoint.com/2021/mosesstaff-targeting-israeli-companies/)

|  [G0069](https://attack.mitre.org/groups/G0069)   |  [MuddyWater](https://attack.mitre.org/groups/G0069)   |

[MuddyWater](https://attack.mitre.org/groups/G0069) has exploited the Microsoft Exchange memory corruption vulnerability (CVE-2020-0688).[[90]](https://www.cisa.gov/uscert/ncas/alerts/aa22-055a)

|  [C0002](https://attack.mitre.org/campaigns/C0002)   |  [Night Dragon](https://attack.mitre.org/campaigns/C0002)   |

During [Night Dragon](https://attack.mitre.org/campaigns/C0002), threat actors used SQL injection exploits against extranet web servers to gain access.[[91]](https://scadahacker.com/library/Documents/Cyber_Events/McAfee%20-%20Night%20Dragon%20-%20Global%20Energy%20Cyberattacks.pdf)

|  [C0012](https://attack.mitre.org/campaigns/C0012)   |  [Operation CuckooBees](https://attack.mitre.org/campaigns/C0012)   |

During [Operation CuckooBees](https://attack.mitre.org/campaigns/C0012), the threat actors exploited multiple vulnerabilities in externally facing servers.[[92]](https://www.cybereason.com/blog/operation-cuckoobees-deep-dive-into-stealthy-winnti-techniques)

|  [C0061](https://attack.mitre.org/campaigns/C0061)   |  [Operation Digital Eye](https://attack.mitre.org/campaigns/C0061)   |

During [Operation Digital Eye](https://attack.mitre.org/campaigns/C0061), threat actors used SQL injection to compromise publicly exposed web and database servers.[[93]](https://www.sentinelone.com/labs/operation-digital-eye-chinese-apt-compromises-critical-digital-infrastructure-via-visual-studio-code-tunnels/)

|  [C0048](https://attack.mitre.org/campaigns/C0048)   |  [Operation MidnightEclipse](https://attack.mitre.org/campaigns/C0048)   |

During [Operation MidnightEclipse](https://attack.mitre.org/campaigns/C0048), threat actors exploited CVE-2024-3400 in Palo Alto Networks GlobalProtect.[[94]](https://www.volexity.com/blog/2024/04/12/zero-day-exploitation-of-unauthenticated-remote-code-execution-vulnerability-in-globalprotect-cve-2024-3400/)[[95]](https://unit42.paloaltonetworks.com/cve-2024-3400/)

|  [C0014](https://attack.mitre.org/campaigns/C0014)   |  [Operation Wocao](https://attack.mitre.org/campaigns/C0014)   |

During [Operation Wocao](https://attack.mitre.org/campaigns/C0014), threat actors gained initial access by exploiting vulnerabilities in JBoss webservers.[[96]](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)

|  [G1040](https://attack.mitre.org/groups/G1040)   |  [Play](https://attack.mitre.org/groups/G1040)   |

[Play](https://attack.mitre.org/groups/G1040) has exploited known vulnerabilities for initial access including CVE-2018-13379 and CVE-2020-12812 in FortiOS and CVE-2022-41082 and CVE-2022-41040 ("ProxyNotShell") in Microsoft Exchange.[[97]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-352a)[[98]](https://www.trendmicro.com/vinfo/us/security/news/ransomware-spotlight/ransomware-spotlight-play)

|  [S1242](https://attack.mitre.org/software/S1242)   |  [Qilin](https://attack.mitre.org/software/S1242)   |

[Qilin](https://attack.mitre.org/software/S1242) has been delivered through exploitation of exposed applications and interfaces including Citrix and RDP.[[99]](https://www.sentinelone.com/anthology/agenda-qilin/)

|  [C0055](https://attack.mitre.org/campaigns/C0055)   |  [Quad7 Activity](https://attack.mitre.org/campaigns/C0055)   |

[Quad7 Activity](https://attack.mitre.org/campaigns/C0055) has enabled the exploitation of vulnerabilities for remote code execution capabilities in SOHO routers including CVE-2023-50224 and CVE-2025-9377 in TP-Link devices.[[100]](https://www.microsoft.com/en-us/security/blog/2024/10/31/chinese-threat-actor-storm-0940-uses-credentials-from-password-spray-attacks-from-a-covert-network/)[[101]](https://www.tp-link.com/us/support/faq/4365/)

|  [G0106](https://attack.mitre.org/groups/G0106)   |  [Rocke](https://attack.mitre.org/groups/G0106)   |

[Rocke](https://attack.mitre.org/groups/G0106) exploited Apache Struts, Oracle WebLogic (CVE-2017-10271), and Adobe ColdFusion (CVE-2017-3066) vulnerabilities to deliver malware.[[102]](https://blog.talosintelligence.com/2018/08/rocke-champion-of-monero-miners.html)[[103]](https://unit42.paloaltonetworks.com/malware-used-by-rocke-group-evolves-to-evade-detection-by-cloud-security-products/)

|  [G1045](https://attack.mitre.org/groups/G1045)   |  [Salt Typhoon](https://attack.mitre.org/groups/G1045)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has leveraged vulnerabilities in public facing applications and network edge devices including Cisco IOS and IOS XE through CVE-2018-0171, Ivanti Connect Secure through CVE-2023-46805, and Palo Alto Networks GlobalProtect through CVE-2024-3400.[[104]](https://blog.talosintelligence.com/salt-typhoon-analysis/)[[105]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)[[106]](https://www.sentinelone.com/labs/malicious-apprentice-how-two-hackers-went-from-cisco-academy-to-cisco-cves/)

|  [G0034](https://attack.mitre.org/groups/G0034)   |  [Sandworm Team](https://attack.mitre.org/groups/G0034)   |

[Sandworm Team](https://attack.mitre.org/groups/G0034) exploits public-facing applications for initial access and to acquire infrastructure, such as exploitation of the EXIM mail transfer agent in Linux systems.[[107]](https://media.defense.gov/2020/May/28/2002306626/-1/-1/0/CSA%20Sandworm%20Actors%20Exploiting%20Vulnerability%20in%20Exim%20Transfer%20Agent%2020200528.pdf)[[108]](https://blog.google/threat-analysis-group/ukraine-remains-russias-biggest-cyber-focus-in-2023/)

|  [G1041](https://attack.mitre.org/groups/G1041)   |  [Sea Turtle](https://attack.mitre.org/groups/G1041)   |

[Sea Turtle](https://attack.mitre.org/groups/G1041) gained access to victim environments by exploiting multiple known vulnerabilities over several campaigns.[[109]](https://blog.talosintelligence.com/seaturtle/)[[110]](https://www.pwc.com/gx/en/issues/cybersecurity/cyber-threat-intelligence/tortoise-and-malwahare.html)

|  [C0045](https://attack.mitre.org/campaigns/C0045)   |  [ShadowRay](https://attack.mitre.org/campaigns/C0045)   |

During [ShadowRay](https://attack.mitre.org/campaigns/C0045), threat actors exploited CVE-2023-48022 on publicly exposed Ray servers to steal computing power and to expose sensitive data.[[111]](https://www.oligo.security/blog/shadowray-attack-ai-workloads-actively-exploited-in-the-wild)

|  [C0058](https://attack.mitre.org/campaigns/C0058)   |  [SharePoint ToolShell Exploitation](https://attack.mitre.org/campaigns/C0058)   |

During [SharePoint ToolShell Exploitation](https://attack.mitre.org/campaigns/C0058), threat actors exploited authentication bypass and remote code execution vulnerabilities (CVE-2025-49706 and CVE-2025-49704) against on-premises SharePoint servers. This activity was characterized by crafted POST requests to the ToolPane endpoint /_layouts/15/ToolPane.aspx.[[112]](https://www.microsoft.com/en-us/security/blog/2025/07/22/disrupting-active-exploitation-of-on-premises-sharepoint-vulnerabilities/#storm-2603)[[113]](https://www.welivesecurity.com/en/eset-research/toolshell-an-all-you-can-eat-buffet-for-threat-actors/)[[114]](https://research.eye.security/sharepoint-under-siege/)[[115]](https://www.trendmicro.com/en_us/research/25/g/cve-2025-53770-and-cve-2025-53771-sharepoint-attacks.html)[[116]](https://www.sentinelone.com/blog/sharepoint-toolshell-zero-day-exploited-in-the-wild-targets-enterprise-servers/)[[117]](https://unit42.paloaltonetworks.com/microsoft-sharepoint-cve-2025-49704-cve-2025-49706-cve-2025-53770/)

|  [G1057](https://attack.mitre.org/groups/G1057)   |  [ShinyHunters](https://attack.mitre.org/groups/G1057)   |

[ShinyHunters](https://attack.mitre.org/groups/G1057) has exploited CVE-2026-35273 against Oracle PeopleSoft application infrastructure.[[118]](https://cloud.google.com/blog/topics/threat-intelligence/shinyhunters-targets-education-sector-oracle-exploit) [ShinyHunters](https://attack.mitre.org/groups/G1057) has exploited known vulnerabilities in internet-facing servers.[[119]](https://blog.eclecticiq.com/shinyhunters-calling-financially-motivated-data-extortion-group-targeting-enterprise-cloud-applications)

|  [S0623](https://attack.mitre.org/software/S0623)   |  [Siloscape](https://attack.mitre.org/software/S0623)   |

[Siloscape](https://attack.mitre.org/software/S0623) is executed after the attacker gains initial access to a Windows container using a known vulnerability.[[120]](https://unit42.paloaltonetworks.com/siloscape/)

|  [C0024](https://attack.mitre.org/campaigns/C0024)   |  [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024)   |

During the [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024), [APT29](https://attack.mitre.org/groups/G0016) exploited CVE-2020-0688 against the Microsoft Exchange Control Panel to regain access to a network.[[121]](https://www.volexity.com/blog/2020/12/14/dark-halo-leverages-solarwinds-compromise-to-breach-organizations/)[[17]](https://www.ncsc.gov.uk/files/Advisory-further-TTPs-associated-with-SVR-cyber-actors.pdf)

|  [S0516](https://attack.mitre.org/software/S0516)   |  [SoreFang](https://attack.mitre.org/software/S0516)   |

[SoreFang](https://attack.mitre.org/software/S0516) can gain access by exploiting a Sangfor SSL VPN vulnerability that allows for the placement and delivery of malicious update binaries.[[122]](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-198a)

|  [C0052](https://attack.mitre.org/campaigns/C0052)   |  [SPACEHOP Activity](https://attack.mitre.org/campaigns/C0052)   |

[SPACEHOP Activity](https://attack.mitre.org/campaigns/C0052) has enabled the exploitation of CVE-2022-27518 and CVE-2022-27518 for illegitimate access.[[25]](https://media.defense.gov/2022/Dec/13/2003131586/-1/-1/0/CSA-APT5-CITRIXADC-V1.PDF)[[57]](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-orb-networks)

|  [S0225](https://attack.mitre.org/software/S0225)   |  [sqlmap](https://attack.mitre.org/software/S0225)   |

[sqlmap](https://attack.mitre.org/software/S0225) can be used to automate exploitation of SQL injection vulnerabilities.[[123]](http://sqlmap.org/)

|  [G1053](https://attack.mitre.org/groups/G1053)   |  [Storm-0501](https://attack.mitre.org/groups/G1053)   |

[Storm-0501](https://attack.mitre.org/groups/G1053) has exploited N-day vulnerabilities associated with public facing services to gain initial access to victim environments to include Zoho ManageEngine (CVE-2022-47966), Citrix NetScaler "Citrix Bleed" (CVE-2023-4966), and Adobe ColdFusion 2016 (CVE-2023-29300 or CVE-2023-38203).[[124]](https://www.microsoft.com/en-us/security/blog/2024/09/26/storm-0501-ransomware-attacks-expanding-to-hybrid-cloud-environments/)

|  [G1056](https://attack.mitre.org/groups/G1056)   |  [TeamPCP](https://attack.mitre.org/groups/G1056)   |

[TeamPCP](https://attack.mitre.org/groups/G1056) has exploited misconfigurations in GitHub Actions and vulnerabilities such as CVE-2026-33634 in the Aqua Security Trivy scanner and CVE-2025-55182 (React2Shell) against vulnerable cloud endpoints.[[125]](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)[[126]](https://www.aikido.dev/blog/telnyx-pypi-compromised-teampcp-canisterworm)[[127]](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)[[128]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)

|  [G1058](https://attack.mitre.org/groups/G1058)   |  [The Gentlemen](https://attack.mitre.org/groups/G1058)   |

[The Gentlemen](https://attack.mitre.org/groups/G1058) has exploited vulnerable public-facing devices and services to gain access to victim environments.[[129]](https://www.intel471.com/blog/threat-hunting-case-study-the-gentlemen)[[130]](https://www.welivesecurity.com/en/eset-research/killing-me-gently-inside-gentlemens-edr-killer-framework/)[[131]](https://unit42.paloaltonetworks.com/the-gentlemen-ransomware/)[[132]](https://www.trendaisecurity.com/en-us/resources-insights/trendai-security-blog/unmasking-the-gentlemen-ransomware)

|  [G0027](https://attack.mitre.org/groups/G0027)   |  [Threat Group-3390](https://attack.mitre.org/groups/G0027)   |

[Threat Group-3390](https://attack.mitre.org/groups/G0027) has exploited the Microsoft SharePoint vulnerability CVE-2019-0604 and CVE-2021-26855, CVE-2021-26857, CVE-2021-26858, and CVE-2021-27065 in Exchange Server.[[133]](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)

|  [G1022](https://attack.mitre.org/groups/G1022)   |  [ToddyCat](https://attack.mitre.org/groups/G1022)   |

[ToddyCat](https://attack.mitre.org/groups/G1022) has exploited the ProxyLogon vulnerability (CVE-2021-26855) to compromise Exchange Servers at multiple organizations.[[134]](https://securelist.com/toddycat/106799/)

|  [G1048](https://attack.mitre.org/groups/G1048)   |  [UNC3886](https://attack.mitre.org/groups/G1048)   |

[UNC3886](https://attack.mitre.org/groups/G1048) has exploited CVE-2022-42475 in FortiOS SSL VPNs to obtain access.[[135]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)[[8]](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)

|  [C0039](https://attack.mitre.org/campaigns/C0039)   |  [Versa Director Zero Day Exploitation](https://attack.mitre.org/campaigns/C0039)   |

[Versa Director Zero Day Exploitation](https://attack.mitre.org/campaigns/C0039) involved exploitation of a vulnerability in Versa Director servers, since identified as CVE-2024-39717, for initial access and code execution.[[136]](https://blog.lumen.com/taking-the-crossroads-the-versa-director-zero-day-exploitation/)

|  [G1055](https://attack.mitre.org/groups/G1055)   |  [VOID MANTICORE](https://attack.mitre.org/groups/G1055)   |

[VOID MANTICORE](https://attack.mitre.org/groups/G1055) has exploited public facing vulnerabilities within victim environments to include SharePoint CVE-2019-0604.[[137]](https://www.justice.gov/opa/media/1431956/dl?inline)

|  [G0123](https://attack.mitre.org/groups/G0123)   |  [Volatile Cedar](https://attack.mitre.org/groups/G0123)   |

[Volatile Cedar](https://attack.mitre.org/groups/G0123) has targeted publicly facing web servers, with both automatic and manual vulnerability discovery.[[138]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2015/03/20082004/volatile-cedar-technical-report.pdf) [[139]](https://www.clearskysec.com/wp-content/uploads/2021/01/Lebanese-Cedar-APT.pdf)

|  [G1017](https://attack.mitre.org/groups/G1017)   |  [Volt Typhoon](https://attack.mitre.org/groups/G1017)   |

[Volt Typhoon](https://attack.mitre.org/groups/G1017) has gained initial access through exploitation of multiple vulnerabilities in internet-facing software and appliances such as Fortinet, Ivanti (formerly Pulse Secure), NETGEAR, Citrix, and Cisco.[[140]](https://web.archive.org/web/20230601025540/https://www.secureworks.com/blog/chinese-cyberespionage-group-bronze-silhouette-targets-us-government-and-defense-organizations)[[141]](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)

|  [G1035](https://attack.mitre.org/groups/G1035)   |  [Winter Vivern](https://attack.mitre.org/groups/G1035)   |

[Winter Vivern](https://attack.mitre.org/groups/G1035) has exploited known and zero-day vulnerabilities in software usch as Roundcube Webmail servers and the "Follina" vulnerability.[[142]](https://www.welivesecurity.com/en/eset-research/winter-vivern-exploits-zero-day-vulnerability-roundcube-webmail-servers/)[[143]](https://www.proofpoint.com/us/blog/threat-insight/exploitation-dish-best-served-cold-winter-vivern-uses-known-zimbra-vulnerability)

|  [S0412](https://attack.mitre.org/software/S0412)   |  [ZxShell](https://attack.mitre.org/software/S0412)   |

[ZxShell](https://attack.mitre.org/software/S0412) has been dropped through exploitation of CVE-2011-2462, CVE-2013-3163, and CVE-2014-0322.[[144]](https://blogs.cisco.com/security/talos/opening-zxshell)

## Mitigations

| ID  | Mitigation  | Description

|  [M1048](https://attack.mitre.org/mitigations/M1048)   |  [Application Isolation and Sandboxing](https://attack.mitre.org/mitigations/M1048)   |

Application isolation will limit what other processes and system features the exploited target can access.

|  [M1050](https://attack.mitre.org/mitigations/M1050)   |  [Exploit Protection](https://attack.mitre.org/mitigations/M1050)   |

Web Application Firewalls may be used to limit exposure of applications to prevent exploit traffic from reaching the application.

|  [M1037](https://attack.mitre.org/mitigations/M1037)   |  [Filter Network Traffic](https://attack.mitre.org/mitigations/M1037)   |

Restrict outbound network traffic from public-facing servers to prevent unauthorized connections from initiating communications with attacker-controlled infrastructure. While this may not prevent the initial exploitation, it limits the attacker's ability to verify and control the compromised server post-exploit, reducing the overall impact of the attack.

|  [M1035](https://attack.mitre.org/mitigations/M1035)   |  [Limit Access to Resource Over Network](https://attack.mitre.org/mitigations/M1035)   |

Ensure that all publicly exposed services are actually intended to be so, and restrict access to any that should only be available internally.

|  [M1030](https://attack.mitre.org/mitigations/M1030)   |  [Network Segmentation](https://attack.mitre.org/mitigations/M1030)   |

Segment externally facing servers and services from the rest of the network with a DMZ or on separate hosting infrastructure.

|  [M1026](https://attack.mitre.org/mitigations/M1026)   |  [Privileged Account Management](https://attack.mitre.org/mitigations/M1026)   |

Use least privilege for service accounts will limit what permissions the exploited process gets on the rest of the system.

|  [M1051](https://attack.mitre.org/mitigations/M1051)   |  [Update Software](https://attack.mitre.org/mitigations/M1051)   |

Update software regularly by employing patch management for externally exposed applications.

|  [M1016](https://attack.mitre.org/mitigations/M1016)   |  [Vulnerability Scanning](https://attack.mitre.org/mitigations/M1016)   |

Regularly scan externally facing systems for vulnerabilities and establish procedures to rapidly patch systems when critical vulnerabilities are discovered through scanning and through public disclosure.[[10]](https://www.owasp.org/index.php/Category:OWASP_Top_Ten_Project)

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0080](https://attack.mitre.org/detectionstrategies/DET0080)   |  [Exploit Public-Facing Application – multi-signal correlation (request → error → post-exploit process/egress)](https://attack.mitre.org/detectionstrategies/DET0080)   |  [AN0219](https://attack.mitre.org/detectionstrategies/DET0080#AN0219)   |

Adversary sends crafted HTTP/S (or other service) input to an Internet-facing app (IIS/ASP.NET, API, device portal). Chain: (1) abnormal request patterns to public endpoint → (2) elevated 4xx/5xx or unusual methods/paths → (3) server process (w3wp.exe/other service) spawns shell/LOLbins or loads non-standard modules → (4) optional outbound callback from the host/container.

|  [AN0220](https://attack.mitre.org/detectionstrategies/DET0080#AN0220)   |

Adversary exploits Apache/Nginx/app servers. Chain: (1) suspicious requests in access logs → (2) spike of 5xx or WAF blocks → (3) web server or interpreter (apache2/nginx/php-fpm/node/python) spawns /bin/sh, curl, wget, socat, or writes webshell → (4) outbound callback.

|  [AN0221](https://attack.mitre.org/detectionstrategies/DET0080#AN0221)   |

Adversary targets macOS-hosted public services (e.g., nginx, node). Chain: suspicious inbound request → service crash/5xx → service spawns shell or writes file → new outbound connection.

|  [AN0222](https://attack.mitre.org/detectionstrategies/DET0080#AN0222)   |

Adversary exploits containerized app via ingress or service. Chain: (1) suspicious request in ingress/app logs → (2) container process spawns a shell/exec/sidecar (kubectl exec/docker exec) → (3) egress to Internet or metadata service (169.254.169.254).

|  [AN0223](https://attack.mitre.org/detectionstrategies/DET0080#AN0223)   |

Adversary targets cloud-hosted public endpoints. Chain: (1) ALB/ELB/Cloud LB logs show exploit-like inputs or error spikes → (2) workload spawns shell or reaches metadata API → (3) egress to new external hosts.

|  [AN0224](https://attack.mitre.org/detectionstrategies/DET0080#AN0224)   |

Adversary exploits exposed OpenSLP on ESXi or vCenter public endpoints. Chain: inbound request pattern to mgmt service → hostd/vpxd error/crash/restart → unexpected process behavior or datastore access → outbound callback.

|  [AN0225](https://attack.mitre.org/detectionstrategies/DET0080#AN0225)   |

Adversary exploits public admin services on routers/firewalls/switches. Chain: anomalous HTTP/SNMP/SmartInstall inputs → device syslog errors/restarts → config changes/CLI spawn → egress to attacker C2.

## References

-    [National Vulnerability Database. (2017, February 2). CVE-2016-6662 Detail. Retrieved April 3, 2018.](https://nvd.nist.gov/vuln/detail/CVE-2016-6662)
-    [CIS. (2017, May 15). Multiple Vulnerabilities in Microsoft Windows SMB Server Could Allow for Remote Code Execution. Retrieved April 3, 2018.](https://www.cisecurity.org/advisory/multiple-vulnerabilities-in-microsoft-windows-smb-server-could-allow-for-remote-code-execution/)
-    [US-CERT. (2018, April 20). Russian State-Sponsored Cyber Actors Targeting Network Infrastructure Devices. Retrieved October 19, 2020.](https://us-cert.cisa.gov/ncas/alerts/TA18-106A)
-    [Omar Santos. (2020, October 19). Attackers Continue to Target Legacy Devices. Retrieved October 20, 2020.](https://community.cisco.com/t5/security-blogs/attackers-continue-to-target-legacy-devices/ba-p/4169954)
-    [National Vulnerability Database. (2017, September 24). CVE-2014-7169 Detail. Retrieved April 3, 2018.](https://nvd.nist.gov/vuln/detail/CVE-2014-7169)
-    [German Hoeffner, Aaron Soehnen and Gianni Perez. (2023, February 7). ESXiArgs Ransomware Targets Publicly-Exposed ESXi OpenSLP Servers. Retrieved March 26, 2025.](https://www.recordedfuture.com/blog/esxiargs-ransomware-targets-vmware-esxi-openslp-servers)
-    [Dan Goodin . (2021, February 25). Code-execution flaw in VMware has a severity rating of 9.8 out of 10. Retrieved April 8, 2025.](https://arstechnica.com/information-technology/2021/02/armed-with-exploits-hackers-on-the-prowl-for-a-critical-vmware-vulnerability/)
-    [Marvi, A. et al.. (2023, March 16). Fortinet Zero-Day and Custom Malware Used by Suspected Chinese Actor in Espionage Operation. Retrieved March 22, 2023.](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)
-    [Greenberg, A. (2022, November 10). Russia’s New Cyberwarfare in Ukraine Is Fast, Dirty, and Relentless. Retrieved March 22, 2023.](https://www.wired.com/story/russia-ukraine-cyberattacks-mandiant/)
-    [OWASP. (2018, February 23). OWASP Top Ten Project. Retrieved April 3, 2018.](https://www.owasp.org/index.php/Category:OWASP_Top_Ten_Project)
-    [Christey, S., Brown, M., Kirby, D., Martin, B., Paller, A.. (2011, September 13). 2011 CWE/SANS Top 25 Most Dangerous Software Errors. Retrieved April 10, 2019.](https://cwe.mitre.org/top25/index.html)
-    [Amitai Ben & Shushan Ehrlich. (2021, May). From Wiper to Ransomware: The Evolution of Agrius. Retrieved May 21, 2024.](https://assets.sentinelone.com/sentinellabs/evol-agrius)
-    [Eyal Sela. (2026, April). The AI-Assisted Breach of Mexico's Government Infrastructure. Retrieved September 2, 2026.](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)
-    [Anthropic. (2025, November). Disrupting the first reported AI-orchestrated cyber espionage campaign. Retrieved April 20, 2026.](https://assets.anthropic.com/m/ec212e6566a0d47/original/Disrupting-the-first-reported-AI-orchestrated-cyber-espionage-campaign.pdf)
-    [Brady, S . (2018, October 3). Indictment - United States vs Aleksei Sergeyevich Morenets, et al.. Retrieved October 1, 2020.](https://www.justice.gov/opa/page/file/1098481/download)
-    [NSA, CISA, FBI, NCSC. (2021, July). Russian GRU Conducting Global Brute Force Campaign to Compromise Enterprise and Cloud Environments. Retrieved July 26, 2021.](https://media.defense.gov/2021/Jul/01/2002753896/-1/-1/1/CSA_GRU_GLOBAL_BRUTE_FORCE_CAMPAIGN_UOO158036-21.PDF)
-    [NCSC, CISA, FBI, NSA. (2021, May 7). Further TTPs associated with SVR cyber actors. Retrieved July 29, 2021.](https://www.ncsc.gov.uk/files/Advisory-further-TTPs-associated-with-SVR-cyber-actors.pdf)
-    [National Cyber Security Centre. (2020, July 16). Advisory: APT29 targets COVID-19 vaccine development. Retrieved September 29, 2020.](https://www.ncsc.gov.uk/files/Advisory-APT29-targets-COVID-19-vaccine-development-V1-1.pdf)
-    [Symantec. (2018, February 28). Chafer: Latest Attacks Reveal Heightened Ambitions. Retrieved May 22, 2020.](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/chafer-latest-attacks-reveal-heightened-ambitions)
-    [Glyer, C, et al. (2020, March). This Is Not a Test: APT41 Initiates Global Intrusion Campaign Using Multiple Exploits. Retrieved April 28, 2020.](https://www.fireeye.com/blog/threat-research/2020/03/apt41-initiates-global-intrusion-campaign-using-multiple-exploits.html)
-    [Nikita Rostovcev. (2022, August 18). APT41 World Tour 2021 on a tight schedule. Retrieved February 22, 2024.](https://www.group-ib.com/blog/apt41-world-tour-2021/)
-    [DCSO CyTec Blog. (2022, December 24). APT41 — The spy who failed to encrypt me. Retrieved June 13, 2024.](https://medium.com/@DCSO_CyTec/apt41-the-spy-who-failed-to-encrypt-me-24fc0f49cad1)
-    [Perez, D. et al. (2021, April 20). Check Your Pulse: Suspected APT Actors Leverage Authentication Bypass Techniques and Pulse Secure Zero-Day. Retrieved February 5, 2024.](https://www.mandiant.com/resources/blog/suspected-apt-actors-leverage-bypass-techniques-pulse-secure-zero-day)
-    [Perez, D. et al. (2021, May 27). Re-Checking Your Pulse: Updates on Chinese APT Actors Compromising Pulse Secure VPN Devices. Retrieved February 5, 2024.](https://www.mandiant.com/resources/blog/updates-on-chinese-apt-compromising-pulse-secure-vpn-devices)
-    [National Security Agency. (2022, December). APT5: Citrix ADC Threat Hunting Guidance. Retrieved February 5, 2024.](https://media.defense.gov/2022/Dec/13/2003131586/-1/-1/0/CSA-APT5-CITRIXADC-V1.PDF)
-    [Microsoft Threat Intelligence. (2023, September). Digital threats from East Asia increase in breadth and effectiveness. Retrieved February 5, 2024.](https://query.prod.cms.rt.microsoft.com/cms/api/am/binary/RW1aFyW)
-    [Canadian Centre for Cyber Security. (2024, April 24). Cyber Activity Impacting CISCO ASA VPNs. Retrieved January 6, 2025.](https://www.cyber.gc.ca/en/news-events/cyber-activity-impacting-cisco-asa-vpns)
-    [Novetta. (n.d.). Operation SMN: Axiom Threat Actor Group Report. Retrieved November 12, 2014.](https://web.archive.org/web/20230115144216/http://www.novetta.com/wp-content/uploads/2014/11/Executive_Summary-Final_1.pdf)
-    [Esler, J., Lee, M., and Williams, C. (2014, October 14). Threat Spotlight: Group 72. Retrieved January 14, 2016.](http://blogs.cisco.com/security/talos/threat-spotlight-group-72)
-    [Adam Burgher. (2021, June 10). BackdoorDiplomacy: Upgrading from Quarian to Turian. Retrieved September 1, 2021](https://www.welivesecurity.com/2021/06/10/backdoordiplomacy-upgrading-quarian-turian/)
-    [US Federal Bureau of Investigation & US Secret Service. (2022, February 11). Indicators of Compromise Associated with BlackByte Ransomware. Retrieved December 16, 2024.](https://www.ic3.gov/CSA/2022/220211.pdf)
-    [Huseyin Can Yuceel. (2022, February 21). TTPs used by BlackByte Ransomware Targeting Critical Infrastructure. Retrieved December 16, 2024.](https://www.picussecurity.com/resource/ttps-used-by-blackbyte-ransomware-targeting-critical-infrastructure)
-    [Symantec Threat Hunter Team. (2022, October 21). Exbyte: BlackByte Ransomware Attackers Deploy New Exfiltration Tool. Retrieved December 16, 2024.](https://www.security.com/threat-intelligence/blackbyte-exbyte-ransomware)
-    [Microsoft Incident Response. (2023, July 6). The five-day job: A BlackByte ransomware intrusion case study. Retrieved December 16, 2024.](https://www.microsoft.com/en-us/security/blog/2023/07/06/the-five-day-job-a-blackbyte-ransomware-intrusion-case-study/)
-    [Bermejo, L., et al. (2017, June 22). Following the Trail of BlackTech’s Cyber Espionage Campaigns. Retrieved May 5, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)
-    [Lambert, T. (2020, May 7). Introducing Blue Mockingbird. Retrieved May 26, 2020.](https://redcanary.com/blog/blue-mockingbird-cryptominer/)
-    [Scott Henderson, Cristiana Kittner, Sarah Hawley & Mark Lechtik, Google Cloud. (2023, January 19). Suspected Chinese Threat Actors Exploiting FortiOS Vulnerability (CVE-2022-42475). Retrieved December 31, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/chinese-actors-exploit-fortios-flaw/)
-    [Rufus Brown, Van Ta, Douglas Bienstock, Geoff Ackerman, John Wolfram. (2022, March 8). Does This Look Infected? A Summary of APT41 Targeting U.S. State Governments. Retrieved July 8, 2022.](https://www.mandiant.com/resources/apt41-us-state-governments)
-    [Venere, G. Neal, C. (2022, June 21). Avos ransomware group expands with new attack arsenal. Retrieved January 11, 2023.](https://blog.talosintelligence.com/avoslocker-new-arsenal/)
-    [Parisi, T. (2022, December 2). Not a SIMulation: CrowdStrike Investigations Reveal Intrusion Campaign Targeting Telco and BPO Companies. Retrieved June 30, 2023.](https://www.crowdstrike.com/blog/analysis-of-intrusion-campaign-targeting-telecom-and-bpo-companies/)
-    [Microsoft. (2022, May 9). Ransomware as a service: Understanding the cybercrime gig economy and how to protect yourself. Retrieved March 10, 2023.](https://www.microsoft.com/en-us/security/blog/2022/05/09/ransomware-as-a-service-understanding-the-cybercrime-gig-economy-and-how-to-protect-yourself/)
-    [Microsoft Threat Intelligence. (2021, December 11). Guidance for preventing, detecting, and hunting for exploitation of the Log4j 2 vulnerability. Retrieved December 7, 2023.](https://www.microsoft.com/en-us/security/blog/2021/12/11/guidance-for-preventing-detecting-and-hunting-for-cve-2021-44228-log4j-2-exploitation/)
-    [Biderman, O. et al. (2022, October 3). REVEALING EMPEROR DRAGONFLY: NIGHT SKY AND CHEERSCRYPT - A SINGLE RANSOMWARE GROUP. Retrieved December 6, 2023.](https://blog.sygnia.co/revealing-emperor-dragonfly-a-chinese-ransomware-group)
-    [Counter Threat Unit Research Team . (2022, June 23). BRONZE STARLIGHT RANSOMWARE OPERATIONS USE HUI LOADER. Retrieved December 7, 2023.](https://www.secureworks.com/research/bronze-starlight-ransomware-operations-use-hui-loader)
-    [Dutch Military Intelligence and Security Service (MIVD) & Dutch General Intelligence and Security Service (AIVD). (2024, February 6). Ministry of Defense of the Netherlands uncovers COATHANGER, a stealthy Chinese FortiGate RAT. Retrieved February 7, 2024.](https://www.ncsc.nl/binaries/ncsc/documenten/publicaties/2024/februari/6/mivd-aivd-advisory-coathanger-tlp-clear/TLP-CLEAR+MIVD+AIVD+Advisory+COATHANGER.pdf)
-    [McLellan, T. et al. (2024, January 12). Cutting Edge: Suspected APT Targets Ivanti Connect Secure VPN in New Zero-Day Exploitation. Retrieved February 27, 2024.](https://www.mandiant.com/resources/blog/suspected-apt-targets-ivanti-zero-day)
-    [Meltzer, M. et al. (2024, January 10). Active Exploitation of Two Zero-Day Vulnerabilities in Ivanti Connect Secure VPN. Retrieved February 27, 2024.](https://www.volexity.com/blog/2024/01/10/active-exploitation-of-two-zero-day-vulnerabilities-in-ivanti-connect-secure-vpn/)
-    [Gurkok, C. et al. (2024, January 15). Ivanti Connect Secure VPN Exploitation Goes Global. Retrieved February 27, 2024.](https://www.volexity.com/blog/2024/01/15/ivanti-connect-secure-vpn-exploitation-goes-global/)
-    [Lin, M. et al. (2024, January 31). Cutting Edge, Part 2: Investigating Ivanti Connect Secure VPN Zero-Day Exploitation. Retrieved February 27, 2024.](https://www.mandiant.com/resources/blog/investigating-ivanti-zero-day-exploitation)
-    [Lin, M. et al. (2024, February 27). Cutting Edge, Part 3: Investigating Ivanti Connect Secure VPN Exploitation and Persistence Attempts. Retrieved March 1, 2024.](https://www.mandiant.com/resources/blog/investigating-ivanti-exploitation-persistence)
-    [CISA. (2020, December 1). Russian State-Sponsored Advanced Persistent Threat Actor Compromises U.S. Government Targets. Retrieved December 9, 2021.](https://www.cisa.gov/uscert/ncas/alerts/aa20-296a#revisions)
-    [Chen, J., et al. (2022). Delving Deep: An Analysis of Earth Lusca’s Operations. Retrieved July 1, 2022.](https://www.trendmicro.com/content/dam/trendmicro/global/en/research/22/a/earth-lusca-employs-sophisticated-infrastructure-varied-tools-and-techniques/technical-brief-delving-deep-an-analysis-of-earth-lusca-operations.pdf)
-    [Microsoft Threat Intelligence. (2023, June 14). Cadet Blizzard emerges as a novel and distinct Russian threat actor. Retrieved July 10, 2023.](https://www.microsoft.com/en-us/security/blog/2023/06/14/cadet-blizzard-emerges-as-a-novel-and-distinct-russian-threat-actor/)
-    [US Cybersecurity & Infrastructure Security Agency et al. (2024, September 5). Russian Military Cyber Actors Target U.S. and Global Critical Infrastructure. Retrieved September 6, 2024.](https://www.cisa.gov/sites/default/files/2024-09/aa24-249a-russian-military-cyber-actors-target-us-and-global-critical-infrastructure.pdf)
-    [Ta, V., et al. (2022, August 8). FIN13: A Cybercriminal Threat Actor Focused on Mexico. Retrieved February 9, 2023.](https://www.mandiant.com/resources/blog/fin13-cybercriminal-mexico)
-    [Sygnia Incident Response Team. (2022, January 5). TG2003: ELEPHANT BEETLE UNCOVERING AN ORGANIZED FINANCIAL-THEFT OPERATION. Retrieved February 9, 2023.](https://web.archive.org/web/20220105132433/https://f.hubspotusercontent30.net/hubfs/8776530/Sygnia-%20Elephant%20Beetle_Jan2022.pdf)
-    [Raggi, Michael. (2024, May 22). IOC Extinction? China-Nexus Cyber Espionage Actors Use ORB Networks to Raise Cost on Defenders. Retrieved July 8, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-orb-networks)
-    [ClearSky. (2020, February 16). Fox Kitten – Widespread Iranian Espionage-Offensive Campaign. Retrieved December 21, 2020.](https://www.clearskysec.com/fox-kitten/)
-    [Dragos. (n.d.). PARISITE. Retrieved December 21, 2020.](https://www.dragos.com/threat/parisite/)
-    [Orleans, A. (2020, August 31). Who Is PIONEER KITTEN?. Retrieved December 21, 2020.](https://www.crowdstrike.com/blog/who-is-pioneer-kitten/)
-    [CISA. (2020, September 15). Iran-Based Threat Actor Exploits VPN Vulnerabilities. Retrieved December 21, 2020.](https://us-cert.cisa.gov/ncas/alerts/aa20-259a)
-    [ClearSky. (2020, December 17). Pay2Key Ransomware – A New Campaign by Fox Kitten. Retrieved December 21, 2020.](https://www.clearskysec.com/wp-content/uploads/2020/12/Pay2Kitten.pdf)
-    [Mark Graham, Carolyn Ahlers, Kyle O'Meara; Dragos. (2024, July). Impact of FrostyGoop ICS Malware on Connected OT Systems. Retrieved November 20, 2024.](https://hub.dragos.com/hubfs/Reports/Dragos-FrostyGoop-ICS-Malware-Intel-Brief-0724_r2.pdf)
-    [Cybereason Nocturnus. (2019, June 25). Operation Soft Cell: A Worldwide Campaign Against Telecommunications Providers. Retrieved July 18, 2019.](https://www.cybereason.com/blog/operation-soft-cell-a-worldwide-campaign-against-telecommunications-providers)
-    [MSTIC. (2019, December 12). GALLIUM: Targeting global telecom. Retrieved January 13, 2021.](https://www.microsoft.com/security/blog/2019/12/12/gallium-targeting-global-telecom/)
-    [Counter Threat Unit Research Team. (2019, September 24). REvil/Sodinokibi Ransomware. Retrieved August 4, 2020.](https://www.secureworks.com/research/revil-sodinokibi-ransomware)
-    [MSTIC. (2021, March 2). HAFNIUM targeting Exchange Servers with 0-day exploits. Retrieved March 3, 2021.](https://www.microsoft.com/security/blog/2021/03/02/hafnium-targeting-exchange-servers/)
-    [Gruzweig, J. et al. (2021, March 2). Operation Exchange Marauder: Active Exploitation of Multiple Zero-Day Microsoft Exchange Vulnerabilities. Retrieved March 3, 2021.](https://www.volexity.com/blog/2021/03/02/active-exploitation-of-microsoft-exchange-zero-day-vulnerabilities/)
-    [Bromiley, M. et al. (2021, March 4). Detection and Response to Exploitation of Microsoft Exchange Zero-Day Vulnerabilities. Retrieved March 9, 2021.](https://www.fireeye.com/blog/threat-research/2021/03/detection-response-to-exploitation-of-microsoft-exchange-zero-day-vulnerabilities.html)
-    [Microsoft Threat Intelligence Team & Detection and Response Team . (2022, April 12). Tarrask malware uses scheduled tasks for defense evasion. Retrieved June 1, 2022.](https://www.microsoft.com/security/blog/2022/04/12/tarrask-malware-uses-scheduled-tasks-for-defense-evasion/)
-    [Microsoft Threat Intelligence . (2025, March 5). Silk Typhoon targeting IT supply chain. Retrieved March 20, 2025.](https://www.microsoft.com/en-us/security/blog/2025/03/05/silk-typhoon-targeting-it-supply-chain/)
-    [Ganani, M. (2015, May 14). Analysis of the Havij SQL Injection tool. Retrieved March 19, 2018.](https://blog.checkpoint.com/2015/05/14/analysis-havij-sql-injection-tool/)

-    [CISA. (2022, September 23). AA22-264A Iranian State Actors Conduct Cyber Operations Against the Government of Albania. Retrieved August 6, 2024.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa22-264a)
-    [SOCRadar. (2024, January 24). Dark Web Profile: INC Ransom. Retrieved June 5, 2024.](https://socradar.io/dark-web-profile-inc-ransom/)
-    [SentinelOne. (n.d.). What Is Inc. Ransomware?. Retrieved June 5, 2024.](https://www.sentinelone.com/anthology/inc-ransom/)
-    [MSTIC. (2021, December 6). NICKEL targeting government organizations across Latin America and Europe. Retrieved March 18, 2022.](https://www.microsoft.com/security/blog/2021/12/06/nickel-targeting-government-organizations-across-latin-america-and-europe)
-    [KISA. (2021). Phishing Target Reconnaissance and Attack Resource Analysis Operation Muzabi. Retrieved March 8, 2024.](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)
-    [CISA et al. (2024, July 8). People’s Republic of China (PRC) Ministry of State Security APT40 Tradecraft in Action. Retrieved February 3, 2025.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa24-190a)
-    [Check Point. (2022, January 11). APT35 exploits Log4j vulnerability to distribute new modular PowerShell toolkit. Retrieved January 24, 2022.](https://research.checkpoint.com/2022/apt35-exploits-log4j-vulnerability-to-distribute-new-modular-powershell-toolkit/)
-    [DFIR Report. (2022, March 21). APT35 Automates Initial Access Using ProxyShell. Retrieved May 25, 2022.](https://thedfirreport.com/2022/03/21/apt35-automates-initial-access-using-proxyshell)
-    [Cybereason Nocturnus. (2022, February 1). PowerLess Trojan: Iranian APT Phosphorus Adds New PowerShell Backdoor for Espionage. Retrieved June 1, 2022.](https://www.cybereason.com/blog/research/powerless-trojan-iranian-apt-phosphorus-adds-new-powershell-backdoor-for-espionage)
-    [DFIR Report. (2021, November 15). Exchange Exploit Leads to Domain Wide Ransomware. Retrieved January 5, 2023.](https://thedfirreport.com/2021/11/15/exchange-exploit-leads-to-domain-wide-ransomware/)
-    [MSTIC. (2021, November 16). Evolving trends in Iranian threat actor activity – MSTIC presentation at CyberWarCon 2021. Retrieved January 12, 2023.](https://www.microsoft.com/en-us/security/blog/2021/11/16/evolving-trends-in-iranian-threat-actor-activity-mstic-presentation-at-cyberwarcon-2021)
-    [Anthony Galiette, Doel Santos. (2024, January 11). Medusa Ransomware Turning Your Files into Stone. Retrieved October 15, 2025.](https://unit42.paloaltonetworks.com/medusa-ransomware-escalation-new-leak-site/)
-    [Threat Hunter Team Symantec and Carbon Black. (2025, March 6). Medusa Ransomware Activity Continues to Increase. Retrieved October 15, 2025.](https://www.security.com/threat-intelligence/medusa-ransomware-attacks)
-    [Cybersecurity and Infrastructure Security Agency. (2025, March 12). AA25-071A #StopRansomware: Medusa Ransomware. Retrieved October 15, 2025.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-071a)
-    [GREAT. (2021, March 30). APT10: sophisticated multi-layered loader Ecipekac discovered in A41APT campaign. Retrieved June 17, 2021.](https://securelist.com/apt10-sophisticated-multi-layered-loader-ecipekac-discovered-in-a41apt-campaign/101519/)
-    [Tomonaga, S. (2024, July 16). MirrorFace Attack against Japanese Organisations. Retrieved April 17, 2026.](https://blogs.jpcert.or.jp/en/2024/07/mirrorface-attack-against-japanese-organisations.html)
-    [Checkpoint Research. (2021, November 15). Uncovering MosesStaff techniques: Ideology over Money. Retrieved August 11, 2022.](https://research.checkpoint.com/2021/mosesstaff-targeting-israeli-companies/)
-    [FBI, CISA, CNMF, NCSC-UK. (2022, February 24). Iranian Government-Sponsored Actors Conduct Cyber Operations Against Global Government and Commercial Networks. Retrieved September 27, 2022.](https://www.cisa.gov/uscert/ncas/alerts/aa22-055a)
-    [McAfee® Foundstone® Professional Services and McAfee Labs™. (2011, February 10). Global Energy Cyberattacks: “Night Dragon”. Retrieved February 19, 2018.](https://scadahacker.com/library/Documents/Cyber_Events/McAfee%20-%20Night%20Dragon%20-%20Global%20Energy%20Cyberattacks.pdf)
-    [Cybereason Nocturnus. (2022, May 4). Operation CuckooBees: Deep-Dive into Stealthy Winnti Techniques. Retrieved September 22, 2022.](https://www.cybereason.com/blog/operation-cuckoobees-deep-dive-into-stealthy-winnti-techniques)
-    [Aleksandar Milenkoski, Luigi Martire. (2024, December 10). Operation Digital Eye | Chinese APT Compromises Critical Digital Infrastructure via Visual Studio Code Tunnels. Retrieved February 27, 2025.](https://www.sentinelone.com/labs/operation-digital-eye-chinese-apt-compromises-critical-digital-infrastructure-via-visual-studio-code-tunnels/)
-    [Volexity Threat Research. (2024, April 12). Zero-Day Exploitation of Unauthenticated Remote Code Execution Vulnerability in GlobalProtect (CVE-2024-3400). Retrieved November 20, 2024.](https://www.volexity.com/blog/2024/04/12/zero-day-exploitation-of-unauthenticated-remote-code-execution-vulnerability-in-globalprotect-cve-2024-3400/)
-    [Unit 42. (2024, April 12). Threat Brief: Operation MidnightEclipse, Post-Exploitation Activity Related to CVE-2024-3400 . Retrieved January 15, 2025.](https://unit42.paloaltonetworks.com/cve-2024-3400/)
-    [Dantzig, M. v., Schamper, E. (2019, December 19). Operation Wocao: Shining a light on one of China’s hidden hacking groups. Retrieved October 8, 2020.](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)
-    [CISA. (2023, December 18). #StopRansomware: Play Ransomware AA23-352A. Retrieved September 24, 2024.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-352a)
-    [Trend Micro Research. (2023, July 21). Ransomware Spotlight: Play. Retrieved September 24, 2024.](https://www.trendmicro.com/vinfo/us/security/news/ransomware-spotlight/ransomware-spotlight-play)
-    [SentinelOne. (2022, November 30). Agenda (Qilin). Retrieved September 26, 2025.](https://www.sentinelone.com/anthology/agenda-qilin/)
-    [Microsoft Threat Intelligence. (2024, October 31). Chinese threat actor Storm-0940 uses credentials from password spray attacks from a covert network. Retrieved June 4, 2025.](https://www.microsoft.com/en-us/security/blog/2024/10/31/chinese-threat-actor-storm-0940-uses-credentials-from-password-spray-attacks-from-a-covert-network/)
-    [TP-Link . (2025, August 29). Technical News and Reports about Quad 7 (7777) Botnet aka CovertNetwork-1658. Retrieved October 10, 2025.](https://www.tp-link.com/us/support/faq/4365/)
-    [Liebenberg, D.. (2018, August 30). Rocke: The Champion of Monero Miners. Retrieved May 26, 2020.](https://blog.talosintelligence.com/2018/08/rocke-champion-of-monero-miners.html)
-    [Xingyu, J.. (2019, January 17). Malware Used by Rocke Group Evolves to Evade Detection by Cloud Security Products. Retrieved May 26, 2020.](https://unit42.paloaltonetworks.com/malware-used-by-rocke-group-evolves-to-evade-detection-by-cloud-security-products/)
-    [Cisco Talos. (2025, February 20). Weathering the storm: In the midst of a Typhoon. Retrieved February 24, 2025.](https://blog.talosintelligence.com/salt-typhoon-analysis/)
-    [CISA. (2025, September 3). Countering Chinese State-Sponsored Actors Compromise of Networks Worldwide to Feed Global Espionage System. Retrieved September 22, 2026.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)
-    [Cary, D. (2025, December 10). Malicious Apprentice | How Two Hackers Went From Cisco Academy to Cisco CVEs. Retrieved September 22, 2026.](https://www.sentinelone.com/labs/malicious-apprentice-how-two-hackers-went-from-cisco-academy-to-cisco-cves/)
-    [National Security Agency. (2020, March 28). Sandworm Actors Exploiting Vulnerability In EXIM Mail Transfer Agent. Retrieved March 1, 2024.](https://media.defense.gov/2020/May/28/2002306626/-1/-1/0/CSA%20Sandworm%20Actors%20Exploiting%20Vulnerability%20in%20Exim%20Transfer%20Agent%2020200528.pdf)
-    [Billy Leonard. (2023, April 19). Ukraine remains Russia’s biggest cyber focus in 2023. Retrieved March 1, 2024.](https://blog.google/threat-analysis-group/ukraine-remains-russias-biggest-cyber-focus-in-2023/)
-    [Cisco Talos. (2019, April 17). Sea Turtle: DNS Hijacking Abuses Trust In Core Internet Service. Retrieved November 20, 2024.](https://blog.talosintelligence.com/seaturtle/)
-    [PwC Threat Intelligence. (2023, December 5). The Tortoise and The Malware. Retrieved November 20, 2024.](https://www.pwc.com/gx/en/issues/cybersecurity/cyber-threat-intelligence/tortoise-and-malwahare.html)
-    [Lumelsly, A. et al. (2024, March 26). ShadowRay: First Known Attack Campaign Targeting AI Workloads Actively Exploited In The Wild. Retrieved December 2, 2024.](https://www.oligo.security/blog/shadowray-attack-ai-workloads-actively-exploited-in-the-wild)
-    [Microsoft Threat Intelligence. (2025, July 22). Disrupting active exploitation of on-premises SharePoint vulnerabilities. Retrieved October 15, 2025.](https://www.microsoft.com/en-us/security/blog/2025/07/22/disrupting-active-exploitation-of-on-premises-sharepoint-vulnerabilities/#storm-2603)
-    [ESET Research. (2025, July 24). ToolShell: An all-you-can-eat buffet for threat actors. Retrieved October 15, 2025.](https://www.welivesecurity.com/en/eset-research/toolshell-an-all-you-can-eat-buffet-for-threat-actors/)
-    [Eye Security. (2025, July 19). SharePoint Under Siege: ToolShell Exploit (CVE-2025-49706 & CVE-2025-49704). Retrieved October 15, 2025.](https://research.eye.security/sharepoint-under-siege/)
-    [Trend Micro Research. (2022, July 22). Proactive Security Insights for SharePoint Attacks (CVE-2025-53770 and CVE-2025-53771). Retrieved October 15, 2025.](https://www.trendmicro.com/en_us/research/25/g/cve-2025-53770-and-cve-2025-53771-sharepoint-attacks.html)
-    [Kenin, S. et al. (2025, July 21). SharePoint ToolShell | Zero-Day Exploited in-the-Wild Targets Enterprise Servers. Retrieved October 15, 2025.](https://www.sentinelone.com/blog/sharepoint-toolshell-zero-day-exploited-in-the-wild-targets-enterprise-servers/)
-    [Unit 42. (2025, July 31). Active Exploitation of Microsoft SharePoint Vulnerabilities: Threat Brief (Updated). Retrieved October 15, 2025.](https://unit42.paloaltonetworks.com/microsoft-sharepoint-cve-2025-49704-cve-2025-49706-cve-2025-53770/)
-    [Mandiant, Google Threat Intelligence Group. (2026, June 11). ShinyHunters Targets Education Sector with Oracle PeopleSoft Exploit. Retrieved June 11, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/shinyhunters-targets-education-sector-oracle-exploit)
-    [Büyükkaya, A. (2025, September 22). ShinyHunters Calling: Financially Motivated Data Extortion Group Targeting Enterprise Cloud Applications. Retrieved May 18, 2026.](https://blog.eclecticiq.com/shinyhunters-calling-financially-motivated-data-extortion-group-targeting-enterprise-cloud-applications)
-    [Prizmant, D. (2021, June 7). Siloscape: First Known Malware Targeting Windows Containers to Compromise Cloud Environments. Retrieved June 9, 2021.](https://unit42.paloaltonetworks.com/siloscape/)
-    [Cash, D. et al. (2020, December 14). Dark Halo Leverages SolarWinds Compromise to Breach Organizations. Retrieved December 29, 2020.](https://www.volexity.com/blog/2020/12/14/dark-halo-leverages-solarwinds-compromise-to-breach-organizations/)
-    [CISA. (2020, July 16). MAR-10296782-1.v1 – SOREFANG. Retrieved September 29, 2020.](https://us-cert.cisa.gov/ncas/analysis-reports/ar20-198a)
-    [Damele, B., Stampar, M. (n.d.). sqlmap. Retrieved March 19, 2018.](http://sqlmap.org/)
-    [Microsoft Threat Intelligence. (2024, September 26). Storm-0501: Ransomware attacks expanding to hybrid cloud environments. Retrieved October 19, 2025.](https://www.microsoft.com/en-us/security/blog/2024/09/26/storm-0501-ransomware-attacks-expanding-to-hybrid-cloud-environments/)
-    [Aqua Team. (2026, April 1). Update: Ongoing Investigation and Continued Remediation. Retrieved July 1, 2026.](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)
-    [Eriksen, C. (2026, March 27). Popular telnyx package compromised on PyPI by TeamPCP. Retrieved July 16, 2026.](https://www.aikido.dev/blog/telnyx-pypi-compromised-teampcp-canisterworm)
-    [Unit 42. (2026, March 31). Weaponizing the Protectors: TeamPCP’s Multi-Stage Supply Chain Attack on Security Infrastructure. Retrieved July 1, 2026.](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)
-    [McCarthy, R., Cohen, A., and Read, B. (2026, May 12). Mini Shai-Hulud Strikes Again: TanStack + more npm Packages Compromised. Retrieved July 16, 2026.](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)
-    [Intel 471. (2026, August 11). Threat Hunting Case Study: The Gentlemen. Retrieved September 18, 2026.](https://www.intel471.com/blog/threat-hunting-case-study-the-gentlemen)
-    [Souček, J. (2026, June 18). Killing me gently: Inside Gentlemen’s EDR killer framework. Retrieved September 18, 2026.](https://www.welivesecurity.com/en/eset-research/killing-me-gently-inside-gentlemens-edr-killer-framework/)
-    [Brady, M. (2026, July 10). No Manners Here: The Ruthless Rise of The Gentlemen Ransomware. Retrieved September 18, 2026.](https://unit42.paloaltonetworks.com/the-gentlemen-ransomware/)
-    [Santos, J., et al. (2025, September 9). Unmasking The Gentlemen Ransomware: Tactics, Techniques, and Procedures Revealed . Retrieved September 18, 2026.](https://www.trendaisecurity.com/en-us/resources-insights/trendai-security-blog/unmasking-the-gentlemen-ransomware)
-    [Lunghi, D. and Lu, K. (2021, April 9). Iron Tiger APT Updates Toolkit With Evolved SysUpdate Malware. Retrieved November 12, 2021.](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)
-    [Dedola, G. (2022, June 21). APT ToddyCat. Retrieved January 3, 2024.](https://securelist.com/toddycat/106799/)
-    [Punsaen Boonyakarn, Shawn Chew, Logeswaran Nadarajan, Mathew Potaczek, Jakub Jozwiak, and Alex Marvi. (2024, June 18). Cloaked and Covert: Uncovering UNC3886 Espionage Operations. Retrieved September 24, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)
-    [Black Lotus Labs. (2024, August 27). Taking The Crossroads: The Versa Director Zero-Day Exploitaiton. Retrieved August 27, 2024.](https://blog.lumen.com/taking-the-crossroads-the-versa-director-zero-day-exploitation/)
-    [DOJ/FBI. (2026, March 19). Case 1:26-mj-00683-CDA: Affidavit in Support of Seizure Warrant: In the Matter of the Seizure of Domain Names Justicehomeland[.]org; karmabelow80[.]org; handala-hack[.]to; and handala-redwatned[.]to. Retrieved April 20, 2026.](https://www.justice.gov/opa/media/1431956/dl?inline)
-    [Threat Intelligence and Research. (2015, March 30). VOLATILE CEDAR. Retrieved February 8, 2021.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2015/03/20082004/volatile-cedar-technical-report.pdf)
-    [ClearSky Cyber Security. (2021, January). “Lebanese Cedar” APT Global Lebanese Espionage Campaign Leveraging Web Servers. Retrieved February 10, 2021.](https://www.clearskysec.com/wp-content/uploads/2021/01/Lebanese-Cedar-APT.pdf)
-    [Counter Threat Unit Research Team. (2023, May 24). Chinese Cyberespionage Group BRONZE SILHOUETTE Targets U.S. Government and Defense Organizations. Retrieved July 27, 2023.](https://web.archive.org/web/20230601025540/https://www.secureworks.com/blog/chinese-cyberespionage-group-bronze-silhouette-targets-us-government-and-defense-organizations)
-    [CISA et al.. (2024, February 7). PRC State-Sponsored Actors Compromise and Maintain Persistent Access to U.S. Critical Infrastructure. Retrieved May 15, 2024.](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)
-    [Matthieu Faou. (2023, October 25). Winter Vivern exploits zero-day vulnerability in Roundcube Webmail servers. Retrieved July 29, 2024.](https://www.welivesecurity.com/en/eset-research/winter-vivern-exploits-zero-day-vulnerability-roundcube-webmail-servers/)
-    [Michael Raggi & The Proofpoint Threat Research Team. (2023, March 30). Exploitation is a Dish Best Served Cold: Winter Vivern Uses Known Zimbra Vulnerability to Target Webmail Portals of NATO-Aligned Governments in Europe. Retrieved July 29, 2024.](https://www.proofpoint.com/us/blog/threat-insight/exploitation-dish-best-served-cold-winter-vivern-uses-known-zimbra-vulnerability)
-    [Allievi, A., et al. (2014, October 28). Threat Spotlight: Group 72, Opening the ZxShell. Retrieved September 24, 2019.](https://blogs.cisco.com/security/talos/opening-zxshell)

×
