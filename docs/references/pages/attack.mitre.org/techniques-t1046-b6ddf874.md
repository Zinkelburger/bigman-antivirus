---
url: "https://attack.mitre.org/techniques/T1046/"
title: "Network Service Discovery, Technique T1046 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:27:45Z"
via: "live"
sha256: "ede091c817214c5198829e52eaf94160b800b8a9601b0fdff6331c9632385496"
wayback: "https://web.archive.org/web/20261023090826/https://attack.mitre.org/techniques/T1046/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Network Service Discovery, Technique T1046 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1046/>
> Snapshot: <https://web.archive.org/web/20261023090826/https://attack.mitre.org/techniques/T1046/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Network Service Discovery

#  Network Service Discovery

Adversaries may attempt to get a listing of services running on remote hosts and local network infrastructure devices, including those that may be vulnerable to remote software exploitation. Common methods to acquire this information include port, vulnerability, and/or wordlist scans using tools that are brought onto a system.[[1]](https://us-cert.cisa.gov/ncas/analysis-reports/ar21-126a)

Within cloud environments, adversaries may attempt to discover services running on other cloud hosts. Additionally, if the cloud environment is connected to a on-premises environment, adversaries may be able to identify services running on non-cloud systems as well.

Within macOS environments, adversaries may use the native Bonjour application to discover services running on other macOS hosts within a network. The Bonjour mDNSResponder daemon automatically registers and advertises a host’s registered services on the network. For example, adversaries can use a mDNS query (such as dns-sd -B _ssh._tcp .) to find other systems broadcasting the ssh service.[[2]](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/NetServices/Introduction.html)[[3]](https://themittenmac.com/what-does-apt-activity-look-like-on-macos/)

ID: T1046

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Discovery](https://attack.mitre.org/tactics/TA0007)

ⓘ

Platforms: Containers, IaaS, Linux, Network Devices, Windows, macOS

Contributors: Aaron Sullivan aka ZerkerEOD; Praetorian

Version: 3.2

Created: 31 May 2017

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1046/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1046/)

## Procedure Examples

| ID  | Name  | Description

|  [C0063](https://attack.mitre.org/campaigns/C0063)   |  [2025 Poland Wiper Attacks](https://attack.mitre.org/campaigns/C0063)   |

During the [2025 Poland Wiper Attacks](https://attack.mitre.org/campaigns/C0063), the adversaries utilized Ping, the Advanced Port Scanner and Advanced IP Scanner to enumerate network devices.[[4]](https://cert.pl/uploads/docs/CERT_Polska_Energy_Sector_Incident_Report_2025.pdf)

|  [G1030](https://attack.mitre.org/groups/G1030)   |  [Agrius](https://attack.mitre.org/groups/G1030)   |

[Agrius](https://attack.mitre.org/groups/G1030) used the open-source port scanner WinEggDrop to perform detailed scans of hosts of interest in victim networks.[[5]](https://unit42.paloaltonetworks.com/agonizing-serpens-targets-israeli-tech-higher-ed-sectors/)

|  [C0064](https://attack.mitre.org/campaigns/C0064)   |  [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064)   |

During the [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064), the adversary instructed Claude Code to use [BACKUPOSINT v9.0 APEX PREDATOR](https://attack.mitre.org/software/S9045) to enumerate ports on victim devices and fed it into OpenAI’s GPT-4.1 API for analysis along with other system information.[[6]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

|  [C0062](https://attack.mitre.org/campaigns/C0062)   |  [Anthropic AI-orchestrated Campaign](https://attack.mitre.org/campaigns/C0062)   |

During the [Anthropic AI-orchestrated Campaign](https://attack.mitre.org/campaigns/C0062), the adversary used Claude Code to enumerate internal network services and endpoints across targeted environments using browser automation via MCP, including databases, container registries, admin interfaces, and workflow orchestration platforms.[[7]](https://assets.anthropic.com/m/ec212e6566a0d47/original/Disrupting-the-first-reported-AI-orchestrated-cyber-espionage-campaign.pdf)

|  [G0050](https://attack.mitre.org/groups/G0050)   |  [APT32](https://attack.mitre.org/groups/G0050)   |

[APT32](https://attack.mitre.org/groups/G0050) performed network scanning on the network to search for open ports, services, OS finger-printing, and other vulnerabilities.[[8]](https://cdn2.hubspot.net/hubfs/3354902/Cybereason%20Labs%20Analysis%20Operation%20Cobalt%20Kitty.pdf)

|  [G0087](https://attack.mitre.org/groups/G0087)   |  [APT39](https://attack.mitre.org/groups/G0087)   |

[APT39](https://attack.mitre.org/groups/G0087) has used [CrackMapExec](https://attack.mitre.org/software/S0488) and a custom port scanner known as BLUETORCH for network scanning.[[9]](https://www.fireeye.com/blog/threat-research/2019/01/apt39-iranian-cyber-espionage-group-focused-on-personal-information.html)[[10]](https://www.bitdefender.com/blog/labs/iranian-chafer-apt-targeted-air-transportation-and-government-in-kuwait-and-saudi-arabia/)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) used a malware variant called WIDETONE to conduct port scans on specified subnets.[[11]](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)

|  [S0093](https://attack.mitre.org/software/S0093)   |  [Backdoor.Oldrea](https://attack.mitre.org/software/S0093)   |

[Backdoor.Oldrea](https://attack.mitre.org/software/S0093) can use a network scanning module to identify ICS-related ports.[[12]](https://vblocalhost.com/uploads/VB2021-Slowik.pdf)

|  [G0135](https://attack.mitre.org/groups/G0135)   |  [BackdoorDiplomacy](https://attack.mitre.org/groups/G0135)   |

[BackdoorDiplomacy](https://attack.mitre.org/groups/G0135) has used SMBTouch, a vulnerability scanner, to determine whether a target is vulnerable to EternalBlue malware.[[13]](https://www.welivesecurity.com/2021/06/10/backdoordiplomacy-upgrading-quarian-turian/)

|  [S9045](https://attack.mitre.org/software/S9045)   |  [BACKUPOSINT v9.0 APEX PREDATOR](https://attack.mitre.org/software/S9045)   |

[BACKUPOSINT v9.0 APEX PREDATOR](https://attack.mitre.org/software/S9045) has enumerated the ports on victim systems within the victim environment.[[6]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

|  [S1081](https://attack.mitre.org/software/S1081)   |  [BADHATCH](https://attack.mitre.org/software/S1081)   |

[BADHATCH](https://attack.mitre.org/software/S1081) can check for open ports on a computer by establishing a TCP connection.[[14]](https://www.bitdefender.com/files/News/CaseStudies/study/394/Bitdefender-PR-Whitepaper-BADHATCH-creat5237-en-EN.pdf)

|  [G1043](https://attack.mitre.org/groups/G1043)   |  [BlackByte](https://attack.mitre.org/groups/G1043)   |

[BlackByte](https://attack.mitre.org/groups/G1043) has used tools such as NetScan to enumerate network services in victim environments.[[15]](https://www.microsoft.com/en-us/security/blog/2023/07/06/the-five-day-job-a-blackbyte-ransomware-intrusion-case-study/)

|  [S1180](https://attack.mitre.org/software/S1180)   |  [BlackByte Ransomware](https://attack.mitre.org/software/S1180)   |

[BlackByte Ransomware](https://attack.mitre.org/software/S1180) identifies remote systems via active directory queries for hostnames prior to launching remote ransomware payloads.[[16]](https://www.trustwave.com/en-us/resources/blogs/spiderlabs-blog/blackbyte-ransomware-pt-1-in-depth-analysis/)

|  [S0089](https://attack.mitre.org/software/S0089)   |  [BlackEnergy](https://attack.mitre.org/software/S0089)   |

[BlackEnergy](https://attack.mitre.org/software/S0089) has conducted port scans on a host.[[17]](https://securelist.com/be2-custom-plugins-router-abuse-and-target-profiles/67353/)

|  [G0098](https://attack.mitre.org/groups/G0098)   |  [BlackTech](https://attack.mitre.org/groups/G0098)   |

[BlackTech](https://attack.mitre.org/groups/G0098) has used the SNScan tool to find other potential targets on victim networks.[[18]](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/palmerworm-blacktech-espionage-apt)

|  [S1063](https://attack.mitre.org/software/S1063)   |  [Brute Ratel C4](https://attack.mitre.org/software/S1063)   |

[Brute Ratel C4](https://attack.mitre.org/software/S1063) can conduct port scanning against targeted systems.[[19]](https://unit42.paloaltonetworks.com/brute-ratel-c4-tool/)

|  [C0018](https://attack.mitre.org/campaigns/C0018)   |  [C0018](https://attack.mitre.org/campaigns/C0018)   |

During [C0018](https://attack.mitre.org/campaigns/C0018), the threat actors used the SoftPerfect Network Scanner for network scanning.[[20]](https://blog.talosintelligence.com/avoslocker-new-arsenal/)

|  [C0027](https://attack.mitre.org/campaigns/C0027)   |  [C0027](https://attack.mitre.org/campaigns/C0027)   |

During [C0027](https://attack.mitre.org/campaigns/C0027), used RustScan to scan for open ports on targeted ESXi appliances.[[21]](https://www.crowdstrike.com/blog/analysis-of-intrusion-campaign-targeting-telecom-and-bpo-companies/)

|  [S0572](https://attack.mitre.org/software/S0572)   |  [Caterpillar WebShell](https://attack.mitre.org/software/S0572)   |

[Caterpillar WebShell](https://attack.mitre.org/software/S0572) has a module to use a port scanner on a system.[[22]](https://www.clearskysec.com/wp-content/uploads/2021/01/Lebanese-Cedar-APT.pdf)

|  [G0114](https://attack.mitre.org/groups/G0114)   |  [Chimera](https://attack.mitre.org/groups/G0114)   |

[Chimera](https://attack.mitre.org/groups/G0114) has used the get -b  -e  -p command for network scanning as well as a custom Python tool packed into a Windows executable named Get.exe to scan IP ranges for HTTP.[[23]](https://web.archive.org/web/20230218064220/https://research.nccgroup.com/2021/01/12/abusing-cloud-services-to-fly-under-the-radar/)

|  [S0020](https://attack.mitre.org/software/S0020)   |  [China Chopper](https://attack.mitre.org/software/S0020)   |

[China Chopper](https://attack.mitre.org/software/S0020)'s server component can spider authentication portals.[[24]](https://www.fireeye.com/blog/threat-research/2018/03/suspected-chinese-espionage-group-targeting-maritime-and-engineering-industries.html)

|  [G0080](https://attack.mitre.org/groups/G0080)   |  [Cobalt Group](https://attack.mitre.org/groups/G0080)   |

[Cobalt Group](https://attack.mitre.org/groups/G0080) leveraged an open-source tool called SoftPerfect Network Scanner to perform network scanning.[[25]](https://www.ptsecurity.com/upload/corporate/ww-en/analytics/Cobalt-2017-eng.pdf)[[26]](https://www.ptsecurity.com/upload/corporate/ww-en/analytics/Cobalt-Snatch-eng.pdf)[[27]](https://www.group-ib.com/blog/cobalt)

|  [S0154](https://attack.mitre.org/software/S0154)   |  [Cobalt Strike](https://attack.mitre.org/software/S0154)   |

[Cobalt Strike](https://attack.mitre.org/software/S0154) can perform port scans from an infected host.[[28]](https://web.archive.org/web/20210825130434/https://cobaltstrike.com/downloads/csmanual38.pdf)[[29]](https://web.archive.org/web/20210219195905/https://talos-intelligence-site.s3.amazonaws.com/production/document_files/files/000/095/031/original/Talos_Cobalt_Strike.pdf)[[30]](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)

|  [S0608](https://attack.mitre.org/software/S0608)   |  [Conficker](https://attack.mitre.org/software/S0608)   |

[Conficker](https://attack.mitre.org/software/S0608) scans for other machines to infect.[[31]](https://web.archive.org/web/20200125132645/https://www.sans.org/security-resources/malwarefaq/conficker-worm)

|  [C0004](https://attack.mitre.org/campaigns/C0004)   |  [CostaRicto](https://attack.mitre.org/campaigns/C0004)   |

During [CostaRicto](https://attack.mitre.org/campaigns/C0004), the threat actors employed nmap and pscan to scan target environments.[[32]](https://blogs.blackberry.com/en/2020/11/the-costaricto-campaign-cyber-espionage-outsourced)

|  [G0105](https://attack.mitre.org/groups/G0105)   |  [DarkVishnya](https://attack.mitre.org/groups/G0105)   |

[DarkVishnya](https://attack.mitre.org/groups/G0105) performed port scanning to obtain the list of active services.[[33]](https://securelist.com/darkvishnya/89169/)

|  [G1003](https://attack.mitre.org/groups/G1003)   |  [Ember Bear](https://attack.mitre.org/groups/G1003)   |

[Ember Bear](https://attack.mitre.org/groups/G1003) has used tools such as NMAP for remote system discovery and enumeration in victim environments.[[34]](https://www.cisa.gov/sites/default/files/2024-09/aa24-249a-russian-military-cyber-actors-target-us-and-global-critical-infrastructure.pdf)

|  [S0363](https://attack.mitre.org/software/S0363)   |  [Empire](https://attack.mitre.org/software/S0363)   |

[Empire](https://attack.mitre.org/software/S0363) can perform port scans from an infected host.[[35]](https://github.com/PowerShellEmpire/Empire)

|  [G1016](https://attack.mitre.org/groups/G1016)   |  [FIN13](https://attack.mitre.org/groups/G1016)   |

[FIN13](https://attack.mitre.org/groups/G1016) has utilized nmap for reconnaissance efforts. [FIN13](https://attack.mitre.org/groups/G1016) has also scanned for internal MS-SQL servers in a compromised network.[[36]](https://www.mandiant.com/resources/blog/fin13-cybercriminal-mexico)[[37]](https://web.archive.org/web/20220105132433/https://f.hubspotusercontent30.net/hubfs/8776530/Sygnia-%20Elephant%20Beetle_Jan2022.pdf)

|  [G0037](https://attack.mitre.org/groups/G0037)   |  [FIN6](https://attack.mitre.org/groups/G0037)   |

[FIN6](https://attack.mitre.org/groups/G0037) used publicly available tools (including Microsoft's built-in SQL querying tool, osql.exe) to map the internal network and conduct reconnaissance against Active Directory, Structured Query Language (SQL) servers, and NetBIOS.[[38]](https://web.archive.org/web/20190807112824/https://www2.fireeye.com/rs/848-DID-242/images/rpt-fin6.pdf)

|  [G0117](https://attack.mitre.org/groups/G0117)   |  [Fox Kitten](https://attack.mitre.org/groups/G0117)   |

[Fox Kitten](https://attack.mitre.org/groups/G0117) has used tools including NMAP to conduct broad scanning to identify open ports.[[39]](https://us-cert.cisa.gov/ncas/alerts/aa20-259a)[[40]](https://www.clearskysec.com/wp-content/uploads/2020/12/Pay2Kitten.pdf)

|  [S1144](https://attack.mitre.org/software/S1144)   |  [FRP](https://attack.mitre.org/software/S1144)   |

As part of load balancing [FRP](https://attack.mitre.org/software/S1144) can set healthCheck.type = "tcp" or healthCheck.type = "http" to check service status on specific hosts with TCPing or an HTTP request.[[41]](https://github.com/fatedier/frp)

|  [S0061](https://attack.mitre.org/software/S0061)   |  [HDoor](https://attack.mitre.org/software/S0061)   |

[HDoor](https://attack.mitre.org/software/S0061) scans to identify open ports on the victim.[[42]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07205555/TheNaikonAPT-MsnMM1.pdf)

|  [S0698](https://attack.mitre.org/software/S0698)   |  [HermeticWizard](https://attack.mitre.org/software/S0698)   |

[HermeticWizard](https://attack.mitre.org/software/S0698) has the ability to scan ports on a compromised network.[[43]](https://www.welivesecurity.com/2022/03/01/isaacwiper-hermeticwizard-wiper-worm-targeting-ukraine)

|  [S0601](https://attack.mitre.org/software/S0601)   |  [Hildegard](https://attack.mitre.org/software/S0601)   |

[Hildegard](https://attack.mitre.org/software/S0601) has used masscan to look for kubelets in the internal Kubernetes network.[[44]](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)

|  [C0038](https://attack.mitre.org/campaigns/C0038)   |  [HomeLand Justice](https://attack.mitre.org/campaigns/C0038)   |

During [HomeLand Justice](https://attack.mitre.org/campaigns/C0038), threat actors executed the Advanced Port Scanner tool on compromised systems.[[45]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa22-264a)[[46]](https://www.microsoft.com/en-us/security/blog/2022/09/08/microsoft-investigates-iranian-attacks-against-the-albanian-government/)

|  [G1032](https://attack.mitre.org/groups/G1032)   |  [INC Ransom](https://attack.mitre.org/groups/G1032)   |

[INC Ransom](https://attack.mitre.org/groups/G1032) has used NETSCAN.EXE for internal reconnaissance.[[47]](https://socradar.io/dark-web-profile-inc-ransom/)[[48]](https://www.sentinelone.com/anthology/inc-ransom/)

|  [S0604](https://attack.mitre.org/software/S0604)   |  [Industroyer](https://attack.mitre.org/software/S0604)   |

[Industroyer](https://attack.mitre.org/software/S0604) uses a custom port scanner to map out a network.[[49]](https://www.welivesecurity.com/wp-content/uploads/2017/06/Win32_Industroyer.pdf)

|  [S0260](https://attack.mitre.org/software/S0260)   |  [InvisiMole](https://attack.mitre.org/software/S0260)   |

[InvisiMole](https://attack.mitre.org/software/S0260) can scan the network for open ports and vulnerable instances of RDP and SMB protocols.[[50]](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_InvisiMole.pdf)

|  [S0250](https://attack.mitre.org/software/S0250)   |  [Koadic](https://attack.mitre.org/software/S0250)   |

[Koadic](https://attack.mitre.org/software/S0250) can scan for open TCP ports on the target network.[[51]](https://github.com/offsecginger/koadic)

|  [G0032](https://attack.mitre.org/groups/G0032)   |  [Lazarus Group](https://attack.mitre.org/groups/G0032)   |

[Lazarus Group](https://attack.mitre.org/groups/G0032) has used nmap from a router VM to scan ports on systems within the restricted segment of an enterprise network.[[52]](https://securelist.com/lazarus-threatneedle/100803/)

|  [G0077](https://attack.mitre.org/groups/G0077)   |  [Leafminer](https://attack.mitre.org/groups/G0077)   |

[Leafminer](https://attack.mitre.org/groups/G0077) scanned network services to search for vulnerabilities in the victim system.[[53]](https://www.symantec.com/blogs/threat-intelligence/leafminer-espionage-middle-east)

|  [S1185](https://attack.mitre.org/software/S1185)   |  [LightSpy](https://attack.mitre.org/software/S1185)   |

To collect data on the host's Wi-Fi connection history, [LightSpy](https://attack.mitre.org/software/S1185) reads the /Library/Preferences/SystemConfiguration/com.apple.airport.preferences.plist file.It also utilizes Apple's CWWiFiClient API to scan for nearby Wi-Fi networks and obtain data on the SSID, security type, and RSSI (signal strength) values.[[54]](https://www.huntress.com/blog/lightspy-malware-variant-targeting-macos)

|  [G0030](https://attack.mitre.org/groups/G0030)   |  [Lotus Blossom](https://attack.mitre.org/groups/G0030)   |

[Lotus Blossom](https://attack.mitre.org/groups/G0030) has used port scanners to enumerate services on remote hosts.[[55]](https://www.security.com/threat-intelligence/espionage-asia-governments-cert-authority)

|  [S0532](https://attack.mitre.org/software/S0532)   |  [Lucifer](https://attack.mitre.org/software/S0532)   |

[Lucifer](https://attack.mitre.org/software/S0532) can scan for open ports including TCP ports 135 and 1433.[[56]](https://unit42.paloaltonetworks.com/lucifer-new-cryptojacking-and-ddos-hybrid-malware/)

|  [G0059](https://attack.mitre.org/groups/G0059)   |  [Magic Hound](https://attack.mitre.org/groups/G0059)   |

[Magic Hound](https://attack.mitre.org/groups/G0059) has used KPortScan 3.0 to perform SMB, RDP, and LDAP scanning.[[57]](https://thedfirreport.com/2021/11/15/exchange-exploit-leads-to-domain-wide-ransomware/)

|  [G1051](https://attack.mitre.org/groups/G1051)   |  [Medusa Group](https://attack.mitre.org/groups/G1051)   |

[Medusa Group](https://attack.mitre.org/groups/G1051) has the capability to use living off the land (LOTL) binaries to perform network enumeration.[[58]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-071a) [Medusa Group](https://attack.mitre.org/groups/G1051) has also utilized the publicly available scanning tool SoftPerfect Network Scanner (netscan.exe) to discover device hostnames and network services.[[59]](https://www.security.com/threat-intelligence/medusa-ransomware-attacks)

|  [G0045](https://attack.mitre.org/groups/G0045)   |  [menuPass](https://attack.mitre.org/groups/G0045)   |

[menuPass](https://attack.mitre.org/groups/G0045) has used tcping.exe, similar to [Ping](https://attack.mitre.org/software/S0097), to probe port status on systems of interest.[[60]](https://www.pwc.co.uk/cyber-security/pdf/pwc-uk-operation-cloud-hopper-technical-annex-april-2017.pdf)

|  [S1146](https://attack.mitre.org/software/S1146)   |  [MgBot](https://attack.mitre.org/software/S1146)   |

[MgBot](https://attack.mitre.org/software/S1146) includes modules for performing HTTP and server service scans.[[61]](https://symantec-enterprise-blogs.security.com/threat-intelligence/apt-attacks-telecoms-africa-mgbot)

|  [S0233](https://attack.mitre.org/software/S0233)   |  [MURKYTOP](https://attack.mitre.org/software/S0233)   |

[MURKYTOP](https://attack.mitre.org/software/S0233) has the capability to scan for open ports on hosts in a connected network.[[24]](https://www.fireeye.com/blog/threat-research/2018/03/suspected-chinese-espionage-group-targeting-maritime-and-engineering-industries.html)

|  [G0129](https://attack.mitre.org/groups/G0129)   |  [Mustang Panda](https://attack.mitre.org/groups/G0129)   |

[Mustang Panda](https://attack.mitre.org/groups/G0129) has leveraged [NBTscan](https://attack.mitre.org/software/S0590) to scan IP networks.[[62]](https://unit42.paloaltonetworks.com/stately-taurus-attacks-se-asian-government/)

|  [G0019](https://attack.mitre.org/groups/G0019)   |  [Naikon](https://attack.mitre.org/groups/G0019)   |

[Naikon](https://attack.mitre.org/groups/G0019) has used the LadonGo scanner to scan target networks.[[63]](https://www.bitdefender.com/files/News/CaseStudies/study/396/Bitdefender-PR-Whitepaper-NAIKON-creat5397-en-EN.pdf)

|  [S0590](https://attack.mitre.org/software/S0590)   |  [NBTscan](https://attack.mitre.org/software/S0590)   |

[NBTscan](https://attack.mitre.org/software/S0590) can be used to scan IP networks.[[64]](https://manpages.debian.org/testing/nbtscan/nbtscan.1.en.html)[[65]](https://sectools.org/tool/nbtscan/)

|  [G1059](https://attack.mitre.org/groups/G1059)   |  [NIMBUS MANTICORE](https://attack.mitre.org/groups/G1059)   |

[NIMBUS MANTICORE](https://attack.mitre.org/groups/G1059) has utilized PowerShell scripts to conduct ping and port scanning of victim subnets.[[66]](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) has used the publicly available tool SoftPerfect Network Scanner as well as a custom tool called GOLDIRONY to conduct network scanning.[[67]](https://www.brighttalk.com/webcast/10703/296317/apt34-new-targeted-attack-in-the-middle-east)

|  [C0014](https://attack.mitre.org/campaigns/C0014)   |  [Operation Wocao](https://attack.mitre.org/campaigns/C0014)   |

During [Operation Wocao](https://attack.mitre.org/campaigns/C0014), threat actors scanned for open ports and used nbtscan to find NETBIOS nameservers.[[68]](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)

|  [S0598](https://attack.mitre.org/software/S0598)   |  [P.A.S. Webshell](https://attack.mitre.org/software/S0598)   |

[P.A.S. Webshell](https://attack.mitre.org/software/S0598) can scan networks for open ports and listening services.[[69]](https://www.cert.ssi.gouv.fr/uploads/CERTFR-2021-CTI-005.pdf)

|  [S0683](https://attack.mitre.org/software/S0683)   |  [Peirates](https://attack.mitre.org/software/S0683)   |

[Peirates](https://attack.mitre.org/software/S0683) can initiate a port scan against a given IP address.[[70]](https://github.com/inguardians/peirates)

|  [S0378](https://attack.mitre.org/software/S0378)   |  [PoshC2](https://attack.mitre.org/software/S0378)   |

[PoshC2](https://attack.mitre.org/software/S0378) can perform port scans from an infected host.[[71]](https://github.com/nettitude/PoshC2_Python)

|  [S0192](https://attack.mitre.org/software/S0192)   |  [Pupy](https://attack.mitre.org/software/S0192)   |

[Pupy](https://attack.mitre.org/software/S0192) has a built-in module for port scanning.[[72]](https://github.com/n1nj4sec/pupy)

|  [S0583](https://attack.mitre.org/software/S0583)   |  [Pysa](https://attack.mitre.org/software/S0583)   |

[Pysa](https://attack.mitre.org/software/S0583) can perform network reconnaissance using the Advanced Port Scanner tool.[[73]](https://www.cert.ssi.gouv.fr/uploads/CERTFR-2020-CTI-003.pdf)

|  [S0458](https://attack.mitre.org/software/S0458)   |  [Ramsay](https://attack.mitre.org/software/S0458)   |

[Ramsay](https://attack.mitre.org/software/S0458) can scan for systems that are vulnerable to the EternalBlue exploit.[[74]](https://www.welivesecurity.com/2020/05/13/ramsay-cyberespionage-toolkit-airgapped-networks/)[[75]](https://www.programmersought.com/article/62493896999/)

|  [G1039](https://attack.mitre.org/groups/G1039)   |  [RedCurl](https://attack.mitre.org/groups/G1039)   |

[RedCurl](https://attack.mitre.org/groups/G1039) has used netstat to check if port 4119 is open.[[76]](https://www.trendmicro.com/en_us/research/24/c/unveiling-earth-kapre-aka-redcurls-cyberespionage-tactics-with-t.html)

|  [S0125](https://attack.mitre.org/software/S0125)   |  [Remsec](https://attack.mitre.org/software/S0125)   |

[Remsec](https://attack.mitre.org/software/S0125) has a plugin that can perform ARP scanning as well as port scanning.[[77]](https://securelist.com/files/2016/07/The-ProjectSauron-APT_Technical_Analysis_KL.pdf)

|  [G0106](https://attack.mitre.org/groups/G0106)   |  [Rocke](https://attack.mitre.org/groups/G0106)   |

[Rocke](https://attack.mitre.org/groups/G0106) conducted scanning for exposed TCP port 7001 as well as SSH and Redis servers.[[78]](https://blog.talosintelligence.com/2018/08/rocke-champion-of-monero-miners.html)[[79]](https://www.anomali.com/blog/rocke-evolves-its-arsenal-with-a-new-malware-family-written-in-golang)

|  [S1073](https://attack.mitre.org/software/S1073)   |  [Royal](https://attack.mitre.org/software/S1073)   |

[Royal](https://attack.mitre.org/software/S1073) can scan the network interfaces of targeted systems.[[80]](https://www.cybereason.com/blog/royal-ransomware-analysis)

|  [G1045](https://attack.mitre.org/groups/G1045)   |  [Salt Typhoon](https://attack.mitre.org/groups/G1045)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has scanned open ports and services within the victim environment.[[81]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  [S0692](https://attack.mitre.org/software/S0692)   |  [SILENTTRINITY](https://attack.mitre.org/software/S0692)   |

[SILENTTRINITY](https://attack.mitre.org/software/S0692) can scan for open ports on a compromised machine.[[82]](https://github.com/byt3bl33d3r/SILENTTRINITY/tree/master/silenttrinity/core/teamserver/modules/boo)

|  [S0374](https://attack.mitre.org/software/S0374)   |  [SpeakUp](https://attack.mitre.org/software/S0374)   |

[SpeakUp](https://attack.mitre.org/software/S0374) checks for availability of specific ports on servers.[[83]](https://research.checkpoint.com/speakup-a-new-undetected-backdoor-linux-trojan/)

|  [G0039](https://attack.mitre.org/groups/G0039)   |  [Suckfly](https://attack.mitre.org/groups/G0039)   |

[Suckfly](https://attack.mitre.org/groups/G0039) the victim's internal network for hosts with ports 8080, 5900, and 40 open.[[84]](http://www.symantec.com/connect/blogs/indian-organizations-targeted-suckfly-attacks)

|  [G0139](https://attack.mitre.org/groups/G0139)   |  [TeamTNT](https://attack.mitre.org/groups/G0139)   |

[TeamTNT](https://attack.mitre.org/groups/G0139) has used masscan to search for open Docker API ports and Kubernetes clusters.[[85]](https://www.cadosecurity.com/team-tnt-the-first-crypto-mining-worm-to-steal-aws-credentials/)[[44]](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)[[86]](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/) [TeamTNT](https://attack.mitre.org/groups/G0139) has also used malware that utilizes zmap and zgrab to search for vulnerable services in cloud environments.[[87]](https://unit42.paloaltonetworks.com/black-t-cryptojacking-variant/)

|  [G0027](https://attack.mitre.org/groups/G0027)   |  [Threat Group-3390](https://attack.mitre.org/groups/G0027)   |

[Threat Group-3390](https://attack.mitre.org/groups/G0027) actors use the Hunter tool to conduct network service discovery for vulnerable systems.[[88]](https://www.secureworks.com/research/threat-group-3390-targets-organizations-for-cyberespionage)[[89]](https://unit42.paloaltonetworks.com/emissary-panda-attacks-middle-east-government-sharepoint-servers/)

|  [G0081](https://attack.mitre.org/groups/G0081)   |  [Tropic Trooper](https://attack.mitre.org/groups/G0081)   |

[Tropic Trooper](https://attack.mitre.org/groups/G0081) used pr and an openly available tool to scan for open ports on target systems.[[90]](https://documents.trendmicro.com/assets/wp/wp-operation-tropic-trooper.pdf)[[91]](https://documents.trendmicro.com/assets/Tech-Brief-Tropic-Trooper-s-Back-USBferry-Attack-Targets-Air-gapped-Environments.pdf)

|  [G1017](https://attack.mitre.org/groups/G1017)   |  [Volt Typhoon](https://attack.mitre.org/groups/G1017)   |

[Volt Typhoon](https://attack.mitre.org/groups/G1017) has used commercial tools, LOTL utilities, and appliances already present on the system for network service discovery.[[92]](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)

|  [S0341](https://attack.mitre.org/software/S0341)   |  [Xbash](https://attack.mitre.org/software/S0341)   |

[Xbash](https://attack.mitre.org/software/S0341) can perform port scanning of TCP and UDP ports.[[93]](https://researchcenter.paloaltonetworks.com/2018/09/unit42-xbash-combines-botnet-ransomware-coinmining-worm-targets-linux-windows/)

|  [S0117](https://attack.mitre.org/software/S0117)   |  [XTunnel](https://attack.mitre.org/software/S0117)   |

[XTunnel](https://attack.mitre.org/software/S0117) is capable of probing the network for open ports.[[94]](https://www.invincea.com/2016/07/tunnel-of-gov-dnc-hack-and-the-russian-xtunnel/)

|  [S0412](https://attack.mitre.org/software/S0412)   |  [ZxShell](https://attack.mitre.org/software/S0412)   |

[ZxShell](https://attack.mitre.org/software/S0412) can launch port scans.[[11]](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)[[95]](https://blogs.cisco.com/security/talos/opening-zxshell)

## Mitigations

| ID  | Mitigation  | Description

|  [M1042](https://attack.mitre.org/mitigations/M1042)   |  [Disable or Remove Feature or Program](https://attack.mitre.org/mitigations/M1042)   |

Ensure that unnecessary ports and services are closed to prevent risk of discovery and potential exploitation.

|  [M1031](https://attack.mitre.org/mitigations/M1031)   |  [Network Intrusion Prevention](https://attack.mitre.org/mitigations/M1031)   |

Use network intrusion detection/prevention systems to detect and prevent remote service scans.

|  [M1030](https://attack.mitre.org/mitigations/M1030)   |  [Network Segmentation](https://attack.mitre.org/mitigations/M1030)   |

Ensure proper network segmentation is followed to protect critical servers and devices.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0376](https://attack.mitre.org/detectionstrategies/DET0376)   |  [Behavioral Detection Strategy for Network Service Discovery Across Platforms](https://attack.mitre.org/detectionstrategies/DET0376)   |  [AN1057](https://attack.mitre.org/detectionstrategies/DET0376#AN1057)   |

Detects processes performing network enumeration (e.g., port scans, service probing) by correlating process creation, socket connections, and sequential destination IP probing within a time window.

|  [AN1058](https://attack.mitre.org/detectionstrategies/DET0376#AN1058)   |

Detects use of network scanning utilities or scripts performing rapid connections to multiple services or hosts using auditd and netflow/pcap telemetry.

|  [AN1059](https://attack.mitre.org/detectionstrategies/DET0376#AN1059)   |

Detects Bonjour-based mDNS enumeration or use of system tools (e.g., dns-sd, nmap) to find active services via multicast probing or targeted scans.

|  [AN1060](https://attack.mitre.org/detectionstrategies/DET0376#AN1060)   |

Detects lateral discovery or container breakout attempts using netcat, curl, or custom binaries probing other services within the same namespace or VPC subnet.

## References

-    [CISA. (2021, May 6). Analysis Report (AR21-126A) FiveHands Ransomware. Retrieved June 7, 2021.](https://us-cert.cisa.gov/ncas/analysis-reports/ar21-126a)
-    [Apple Inc. (2013, April 23). Bonjour Overview. Retrieved October 11, 2021.](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/NetServices/Introduction.html)
-    [Jaron Bradley. (2021, November 14). What does APT Activity Look Like on macOS?. Retrieved January 19, 2022.](https://themittenmac.com/what-does-apt-activity-look-like-on-macos/)
-    [CERT Polska. (2026, January 30). Energy Sector Incident Report – 29 December. Retrieved April 22, 2026.](https://cert.pl/uploads/docs/CERT_Polska_Energy_Sector_Incident_Report_2025.pdf)
-    [Or Chechik, Tom Fakterman, Daniel Frank & Assaf Dahan. (2023, November 6). Agonizing Serpens (Aka Agrius) Targeting the Israeli Higher Education and Tech Sectors. Retrieved May 22, 2024.](https://unit42.paloaltonetworks.com/agonizing-serpens-targets-israeli-tech-higher-ed-sectors/)
-    [Eyal Sela. (2026, April). The AI-Assisted Breach of Mexico's Government Infrastructure. Retrieved September 2, 2026.](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)
-    [Anthropic. (2025, November). Disrupting the first reported AI-orchestrated cyber espionage campaign. Retrieved April 20, 2026.](https://assets.anthropic.com/m/ec212e6566a0d47/original/Disrupting-the-first-reported-AI-orchestrated-cyber-espionage-campaign.pdf)
-    [Dahan, A. (2017). Operation Cobalt Kitty. Retrieved December 27, 2018.](https://cdn2.hubspot.net/hubfs/3354902/Cybereason%20Labs%20Analysis%20Operation%20Cobalt%20Kitty.pdf)
-    [Hawley et al. (2019, January 29). APT39: An Iranian Cyber Espionage Group Focused on Personal Information. Retrieved February 19, 2019.](https://www.fireeye.com/blog/threat-research/2019/01/apt39-iranian-cyber-espionage-group-focused-on-personal-information.html)
-    [Rusu, B. (2020, May 21). Iranian Chafer APT Targeted Air Transportation and Government in Kuwait and Saudi Arabia. Retrieved May 22, 2020.](https://www.bitdefender.com/blog/labs/iranian-chafer-apt-targeted-air-transportation-and-government-in-kuwait-and-saudi-arabia/)
-    [Fraser, N., et al. (2019, August 7). Double DragonAPT41, a dual espionage and cyber crime operation APT41. Retrieved September 23, 2019.](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)
-    [Slowik, J. (2021, October). THE BAFFLING BERSERK BEAR: A DECADE’S ACTIVITY TARGETING CRITICAL INFRASTRUCTURE. Retrieved December 6, 2021.](https://vblocalhost.com/uploads/VB2021-Slowik.pdf)
-    [Adam Burgher. (2021, June 10). BackdoorDiplomacy: Upgrading from Quarian to Turian. Retrieved September 1, 2021](https://www.welivesecurity.com/2021/06/10/backdoordiplomacy-upgrading-quarian-turian/)
-    [Vrabie, V., et al. (2021, March 10). FIN8 Returns with Improved BADHATCH Toolkit. Retrieved September 8, 2021.](https://www.bitdefender.com/files/News/CaseStudies/study/394/Bitdefender-PR-Whitepaper-BADHATCH-creat5237-en-EN.pdf)
-    [Microsoft Incident Response. (2023, July 6). The five-day job: A BlackByte ransomware intrusion case study. Retrieved December 16, 2024.](https://www.microsoft.com/en-us/security/blog/2023/07/06/the-five-day-job-a-blackbyte-ransomware-intrusion-case-study/)
-    [Rodel Mendrez & Lloyd Macrohon. (2021, October 15). BlackByte Ransomware – Pt. 1 In-depth Analysis. Retrieved December 16, 2024.](https://www.trustwave.com/en-us/resources/blogs/spiderlabs-blog/blackbyte-ransomware-pt-1-in-depth-analysis/)
-    [Baumgartner, K. and Garnaeva, M.. (2014, November 3). BE2 custom plugins, router abuse, and target profiles. Retrieved March 24, 2016.](https://securelist.com/be2-custom-plugins-router-abuse-and-target-profiles/67353/)
-    [Threat Intelligence. (2020, September 29). Palmerworm: Espionage Gang Targets the Media, Finance, and Other Sectors. Retrieved March 25, 2022.](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/palmerworm-blacktech-espionage-apt)
-    [Harbison, M. and Renals, P. (2022, July 5). When Pentest Tools Go Brutal: Red-Teaming Tool Being Abused by Malicious Actors. Retrieved February 1, 2023.](https://unit42.paloaltonetworks.com/brute-ratel-c4-tool/)
-    [Venere, G. Neal, C. (2022, June 21). Avos ransomware group expands with new attack arsenal. Retrieved January 11, 2023.](https://blog.talosintelligence.com/avoslocker-new-arsenal/)
-    [Parisi, T. (2022, December 2). Not a SIMulation: CrowdStrike Investigations Reveal Intrusion Campaign Targeting Telco and BPO Companies. Retrieved June 30, 2023.](https://www.crowdstrike.com/blog/analysis-of-intrusion-campaign-targeting-telecom-and-bpo-companies/)
-    [ClearSky Cyber Security. (2021, January). “Lebanese Cedar” APT Global Lebanese Espionage Campaign Leveraging Web Servers. Retrieved February 10, 2021.](https://www.clearskysec.com/wp-content/uploads/2021/01/Lebanese-Cedar-APT.pdf)
-    [Jansen, W . (2021, January 12). Abusing cloud services to fly under the radar. Retrieved September 12, 2024.](https://web.archive.org/web/20230218064220/https://research.nccgroup.com/2021/01/12/abusing-cloud-services-to-fly-under-the-radar/)
-    [FireEye. (2018, March 16). Suspected Chinese Cyber Espionage Group (TEMP.Periscope) Targeting U.S. Engineering and Maritime Industries. Retrieved April 11, 2018.](https://www.fireeye.com/blog/threat-research/2018/03/suspected-chinese-espionage-group-targeting-maritime-and-engineering-industries.html)
-    [Positive Technologies. (2017, August 16). Cobalt Strikes Back: An Evolving Multinational Threat to Finance. Retrieved September 5, 2018.](https://www.ptsecurity.com/upload/corporate/ww-en/analytics/Cobalt-2017-eng.pdf)
-    [Positive Technologies. (2016, December 16). Cobalt Snatch. Retrieved October 9, 2018.](https://www.ptsecurity.com/upload/corporate/ww-en/analytics/Cobalt-Snatch-eng.pdf)
-    [Matveeva, V. (2017, August 15). Secrets of Cobalt. Retrieved October 10, 2018.](https://www.group-ib.com/blog/cobalt)
-    [Strategic Cyber LLC. (2017, March 14). Cobalt Strike Manual. Retrieved May 24, 2017.](https://web.archive.org/web/20210825130434/https://cobaltstrike.com/downloads/csmanual38.pdf)
-    [Mavis, N. (2020, September 21). The Art and Science of Detecting Cobalt Strike. Retrieved September 12, 2024.](https://web.archive.org/web/20210219195905/https://talos-intelligence-site.s3.amazonaws.com/production/document_files/files/000/095/031/original/Talos_Cobalt_Strike.pdf)
-    [Strategic Cyber LLC. (2020, November 5). Cobalt Strike: Advanced Threat Tactics for Penetration Testers. Retrieved April 13, 2021.](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)
-    [Burton, K. (n.d.). The Conficker Worm. Retrieved February 18, 2021.](https://web.archive.org/web/20200125132645/https://www.sans.org/security-resources/malwarefaq/conficker-worm)
-    [The BlackBerry Research and Intelligence Team. (2020, November 12). The CostaRicto Campaign: Cyber-Espionage Outsourced. Retrieved May 24, 2021.](https://blogs.blackberry.com/en/2020/11/the-costaricto-campaign-cyber-espionage-outsourced)
-    [Golovanov, S. (2018, December 6). DarkVishnya: Banks attacked through direct connection to local network. Retrieved May 15, 2020.](https://securelist.com/darkvishnya/89169/)
-    [US Cybersecurity & Infrastructure Security Agency et al. (2024, September 5). Russian Military Cyber Actors Target U.S. and Global Critical Infrastructure. Retrieved September 6, 2024.](https://www.cisa.gov/sites/default/files/2024-09/aa24-249a-russian-military-cyber-actors-target-us-and-global-critical-infrastructure.pdf)
-    [Schroeder, W., Warner, J., Nelson, M. (n.d.). Github PowerShellEmpire. Retrieved April 28, 2016.](https://github.com/PowerShellEmpire/Empire)
-    [Ta, V., et al. (2022, August 8). FIN13: A Cybercriminal Threat Actor Focused on Mexico. Retrieved February 9, 2023.](https://www.mandiant.com/resources/blog/fin13-cybercriminal-mexico)
-    [Sygnia Incident Response Team. (2022, January 5). TG2003: ELEPHANT BEETLE UNCOVERING AN ORGANIZED FINANCIAL-THEFT OPERATION. Retrieved February 9, 2023.](https://web.archive.org/web/20220105132433/https://f.hubspotusercontent30.net/hubfs/8776530/Sygnia-%20Elephant%20Beetle_Jan2022.pdf)
-    [FireEye Threat Intelligence. (2016, April). Follow the Money: Dissecting the Operations of the Cyber Crime Group FIN6. Retrieved November 17, 2024.](https://web.archive.org/web/20190807112824/https://www2.fireeye.com/rs/848-DID-242/images/rpt-fin6.pdf)
-    [CISA. (2020, September 15). Iran-Based Threat Actor Exploits VPN Vulnerabilities. Retrieved December 21, 2020.](https://us-cert.cisa.gov/ncas/alerts/aa20-259a)
-    [ClearSky. (2020, December 17). Pay2Key Ransomware – A New Campaign by Fox Kitten. Retrieved December 21, 2020.](https://www.clearskysec.com/wp-content/uploads/2020/12/Pay2Kitten.pdf)
-    [fatedier. (n.d.). What is frp?. Retrieved July 10, 2024.](https://github.com/fatedier/frp)
-    [Baumgartner, K., Golovkin, M.. (2015, May). The MsnMM Campaigns: The Earliest Naikon APT Campaigns. Retrieved April 10, 2019.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07205555/TheNaikonAPT-MsnMM1.pdf)
-    [ESET. (2022, March 1). IsaacWiper and HermeticWizard: New wiper and worm targetingUkraine. Retrieved April 10, 2022.](https://www.welivesecurity.com/2022/03/01/isaacwiper-hermeticwizard-wiper-worm-targeting-ukraine)
-    [Chen, J. et al. (2021, February 3). Hildegard: New TeamTNT Cryptojacking Malware Targeting Kubernetes. Retrieved April 5, 2021.](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)
-    [CISA. (2022, September 23). AA22-264A Iranian State Actors Conduct Cyber Operations Against the Government of Albania. Retrieved August 6, 2024.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa22-264a)
-    [MSTIC. (2022, September 8). Microsoft investigates Iranian attacks against the Albanian government. Retrieved August 6, 2024.](https://www.microsoft.com/en-us/security/blog/2022/09/08/microsoft-investigates-iranian-attacks-against-the-albanian-government/)
-    [SOCRadar. (2024, January 24). Dark Web Profile: INC Ransom. Retrieved June 5, 2024.](https://socradar.io/dark-web-profile-inc-ransom/)
-    [SentinelOne. (n.d.). What Is Inc. Ransomware?. Retrieved June 5, 2024.](https://www.sentinelone.com/anthology/inc-ransom/)

-    [Anton Cherepanov. (2017, June 12). Win32/Industroyer: A new threat for industrial controls systems. Retrieved December 18, 2020.](https://www.welivesecurity.com/wp-content/uploads/2017/06/Win32_Industroyer.pdf)
-    [Hromcova, Z. and Cherpanov, A. (2020, June). INVISIMOLE: THE HIDDEN PART OF THE STORY. Retrieved July 16, 2020.](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_InvisiMole.pdf)
-    [Magius, J., et al. (2017, July 19). Koadic. Retrieved September 27, 2024.](https://github.com/offsecginger/koadic)
-    [Vyacheslav Kopeytsev and Seongsu Park. (2021, February 25). Lazarus targets defense industry with ThreatNeedle. Retrieved October 27, 2021.](https://securelist.com/lazarus-threatneedle/100803/)
-    [Symantec Security Response. (2018, July 25). Leafminer: New Espionage Campaigns Targeting Middle Eastern Regions. Retrieved August 28, 2018.](https://www.symantec.com/blogs/threat-intelligence/leafminer-espionage-middle-east)
-    [Stuart Ashenbrenner, Alden Schmidt. (2024, April 25). LightSpy Malware Variant Targeting macOS. Retrieved January 3, 2025.](https://www.huntress.com/blog/lightspy-malware-variant-targeting-macos)
-    [Symntec Threat Hunter Team. (2022, November 12). Billbug: State-sponsored Actor Targets Cert Authority, Government Agencies in Multiple Asian Countries. Retrieved March 15, 2025.](https://www.security.com/threat-intelligence/espionage-asia-governments-cert-authority)
-    [Hsu, K. et al. (2020, June 24). Lucifer: New Cryptojacking and DDoS Hybrid Malware Exploiting High and Critical Vulnerabilities to Infect Windows Devices. Retrieved November 16, 2020.](https://unit42.paloaltonetworks.com/lucifer-new-cryptojacking-and-ddos-hybrid-malware/)
-    [DFIR Report. (2021, November 15). Exchange Exploit Leads to Domain Wide Ransomware. Retrieved January 5, 2023.](https://thedfirreport.com/2021/11/15/exchange-exploit-leads-to-domain-wide-ransomware/)
-    [Cybersecurity and Infrastructure Security Agency. (2025, March 12). AA25-071A #StopRansomware: Medusa Ransomware. Retrieved October 15, 2025.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-071a)
-    [Threat Hunter Team Symantec and Carbon Black. (2025, March 6). Medusa Ransomware Activity Continues to Increase. Retrieved October 15, 2025.](https://www.security.com/threat-intelligence/medusa-ransomware-attacks)
-    [PwC and BAE Systems. (2017, April). Operation Cloud Hopper: Technical Annex. Retrieved April 13, 2017.](https://www.pwc.co.uk/cyber-security/pdf/pwc-uk-operation-cloud-hopper-technical-annex-april-2017.pdf)
-    [Threat Hunter Team. (2023, April 20). Daggerfly: APT Actor Targets Telecoms Company in Africa. Retrieved July 25, 2024.](https://symantec-enterprise-blogs.security.com/threat-intelligence/apt-attacks-telecoms-africa-mgbot)
-    [Lior Rochberger, Tom Fakterman, Robert Falcone. (2023, September 22). Cyberespionage Attacks Against Southeast Asian Government Linked to Stately Taurus, Aka Mustang Panda. Retrieved September 9, 2025.](https://unit42.paloaltonetworks.com/stately-taurus-attacks-se-asian-government/)
-    [Vrabie, V. (2021, April 23). NAIKON – Traces from a Military Cyber-Espionage Operation. Retrieved June 29, 2021.](https://www.bitdefender.com/files/News/CaseStudies/study/396/Bitdefender-PR-Whitepaper-NAIKON-creat5397-en-EN.pdf)
-    [Bezroutchko, A. (2019, November 19). NBTscan man page. Retrieved March 17, 2021.](https://manpages.debian.org/testing/nbtscan/nbtscan.1.en.html)
-    [SecTools. (2003, June 11). NBTscan. Retrieved March 17, 2021.](https://sectools.org/tool/nbtscan/)
-    [El-Banna, M., et al. (2025, November 17). Frontline Intelligence: Analysis of UNC1549 TTPs, Custom Tools, and Malware Targeting the Aerospace and Defense Ecosystem. Retrieved September 28, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)
-    [Davis, S. and Caban, D. (2017, December 19). APT34 - New Targeted Attack in the Middle East. Retrieved December 20, 2017.](https://www.brighttalk.com/webcast/10703/296317/apt34-new-targeted-attack-in-the-middle-east)
-    [Dantzig, M. v., Schamper, E. (2019, December 19). Operation Wocao: Shining a light on one of China’s hidden hacking groups. Retrieved October 8, 2020.](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)
-    [ANSSI. (2021, January 27). SANDWORM INTRUSION SET CAMPAIGN TARGETING CENTREON SYSTEMS. Retrieved March 30, 2021.](https://www.cert.ssi.gouv.fr/uploads/CERTFR-2021-CTI-005.pdf)
-    [InGuardians. (2022, January 5). Peirates GitHub. Retrieved February 8, 2022.](https://github.com/inguardians/peirates)
-    [Nettitude. (2018, July 23). Python Server for PoshC2. Retrieved April 23, 2019.](https://github.com/nettitude/PoshC2_Python)
-    [Nicolas Verdier. (n.d.). Retrieved January 29, 2018.](https://github.com/n1nj4sec/pupy)
-    [CERT-FR. (2020, April 1). ATTACKS INVOLVING THE MESPINOZA/PYSA RANSOMWARE. Retrieved March 1, 2021.](https://www.cert.ssi.gouv.fr/uploads/CERTFR-2020-CTI-003.pdf)
-    [Sanmillan, I.. (2020, May 13). Ramsay: A cyber‑espionage toolkit tailored for air‑gapped networks. Retrieved May 27, 2020.](https://www.welivesecurity.com/2020/05/13/ramsay-cyberespionage-toolkit-airgapped-networks/)
-    [Antiy CERT. (2020, April 20). Analysis of Ramsay components of Darkhotel's infiltration and isolation network. Retrieved March 24, 2021.](https://www.programmersought.com/article/62493896999/)
-    [Tancio et al. (2024, March 6). Unveiling Earth Kapre aka RedCurl’s Cyberespionage Tactics With Trend Micro MDR, Threat Intelligence. Retrieved August 9, 2024.](https://www.trendmicro.com/en_us/research/24/c/unveiling-earth-kapre-aka-redcurls-cyberespionage-tactics-with-t.html)
-    [Kaspersky Lab's Global Research & Analysis Team. (2016, August 9). The ProjectSauron APT. Technical Analysis. Retrieved August 17, 2016.](https://securelist.com/files/2016/07/The-ProjectSauron-APT_Technical_Analysis_KL.pdf)
-    [Liebenberg, D.. (2018, August 30). Rocke: The Champion of Monero Miners. Retrieved May 26, 2020.](https://blog.talosintelligence.com/2018/08/rocke-champion-of-monero-miners.html)
-    [Anomali Labs. (2019, March 15). Rocke Evolves Its Arsenal With a New Malware Family Written in Golang. Retrieved April 24, 2019.](https://www.anomali.com/blog/rocke-evolves-its-arsenal-with-a-new-malware-family-written-in-golang)
-    [Cybereason Global SOC and Cybereason Security Research Teams. (2022, December 14). Royal Rumble: Analysis of Royal Ransomware. Retrieved March 30, 2023.](https://www.cybereason.com/blog/royal-ransomware-analysis)
-    [CISA. (2025, September 3). Countering Chinese State-Sponsored Actors Compromise of Networks Worldwide to Feed Global Espionage System. Retrieved September 22, 2026.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)
-    [Salvati, M. (2019, August 6). SILENTTRINITY Modules. Retrieved March 24, 2022.](https://github.com/byt3bl33d3r/SILENTTRINITY/tree/master/silenttrinity/core/teamserver/modules/boo)
-    [Check Point Research. (2019, February 4). SpeakUp: A New Undetected Backdoor Linux Trojan. Retrieved April 17, 2019.](https://research.checkpoint.com/speakup-a-new-undetected-backdoor-linux-trojan/)
-    [DiMaggio, J. (2016, May 17). Indian organizations targeted in Suckfly attacks. Retrieved August 3, 2016.](http://www.symantec.com/connect/blogs/indian-organizations-targeted-suckfly-attacks)
-    [Cado Security. (2020, August 16). Team TNT – The First Crypto-Mining Worm to Steal AWS Credentials. Retrieved September 22, 2021.](https://www.cadosecurity.com/team-tnt-the-first-crypto-mining-worm-to-steal-aws-credentials/)
-    [Darin Smith. (2022, April 21). TeamTNT targeting AWS, Alibaba. Retrieved August 4, 2022.](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/)
-    [Quist, N. (2020, October 5). Black-T: New Cryptojacking Variant from TeamTNT. Retrieved September 22, 2021.](https://unit42.paloaltonetworks.com/black-t-cryptojacking-variant/)
-    [Dell SecureWorks Counter Threat Unit Threat Intelligence. (2015, August 5). Threat Group-3390 Targets Organizations for Cyberespionage. Retrieved August 18, 2018.](https://www.secureworks.com/research/threat-group-3390-targets-organizations-for-cyberespionage)
-    [Falcone, R. and Lancaster, T. (2019, May 28). Emissary Panda Attacks Middle East Government Sharepoint Servers. Retrieved July 9, 2019.](https://unit42.paloaltonetworks.com/emissary-panda-attacks-middle-east-government-sharepoint-servers/)
-    [Alintanahin, K. (2015). Operation Tropic Trooper: Relying on Tried-and-Tested Flaws to Infiltrate Secret Keepers. Retrieved June 14, 2019.](https://documents.trendmicro.com/assets/wp/wp-operation-tropic-trooper.pdf)
-    [Chen, J.. (2020, May 12). Tropic Trooper’s Back: USBferry Attack Targets Air gapped Environments. Retrieved May 20, 2020.](https://documents.trendmicro.com/assets/Tech-Brief-Tropic-Trooper-s-Back-USBferry-Attack-Targets-Air-gapped-Environments.pdf)
-    [CISA et al.. (2024, February 7). PRC State-Sponsored Actors Compromise and Maintain Persistent Access to U.S. Critical Infrastructure. Retrieved May 15, 2024.](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)
-    [Xiao, C. (2018, September 17). Xbash Combines Botnet, Ransomware, Coinmining in Worm that Targets Linux and Windows. Retrieved November 14, 2018.](https://researchcenter.paloaltonetworks.com/2018/09/unit42-xbash-combines-botnet-ransomware-coinmining-worm-targets-linux-windows/)
-    [Belcher, P.. (2016, July 28). Tunnel of Gov: DNC Hack and the Russian XTunnel. Retrieved August 3, 2016.](https://www.invincea.com/2016/07/tunnel-of-gov-dnc-hack-and-the-russian-xtunnel/)
-    [Allievi, A., et al. (2014, October 28). Threat Spotlight: Group 72, Opening the ZxShell. Retrieved September 24, 2019.](https://blogs.cisco.com/security/talos/opening-zxshell)

×
