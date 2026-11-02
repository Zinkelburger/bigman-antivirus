---
url: "https://attack.mitre.org/techniques/T1572/"
title: "Protocol Tunneling, Technique T1572 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T19:00:46Z"
via: "live"
sha256: "84bda4404748b321963d6bd5ef720dc2466a5a4fdef54dc40d796588a551c85c"
wayback: "https://web.archive.org/web/20261023090831/https://attack.mitre.org/techniques/T1572/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Protocol Tunneling, Technique T1572 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1572/>
> Snapshot: <https://web.archive.org/web/20261023090831/https://attack.mitre.org/techniques/T1572/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Protocol Tunneling

#  Protocol Tunneling

Adversaries may tunnel network communications to and from a victim system within a separate protocol to avoid detection/network filtering and/or enable access to otherwise unreachable systems. Tunneling involves explicitly encapsulating a protocol within another. This behavior may conceal malicious traffic by blending in with existing traffic and/or provide an outer layer of encryption (similar to a VPN). Tunneling could also enable routing of network packets that would otherwise not reach their intended destination, such as SMB, RDP, or other traffic that would be filtered by network appliances or not routed over the Internet.

There are various means to encapsulate a protocol within another protocol. For example, adversaries may perform SSH tunneling (also known as SSH port forwarding), which involves forwarding arbitrary data over an encrypted SSH tunnel.[[1]](https://www.ssh.com/ssh/tunneling)[[2]](https://www.sygnia.co/blog/abyss-locker-ransomware-attack-analysis/)

[Protocol Tunneling](https://attack.mitre.org/techniques/T1572) may also be abused by adversaries during [Dynamic Resolution](https://attack.mitre.org/techniques/T1568). Known as DNS over HTTPS (DoH), queries to resolve C2 infrastructure may be encapsulated within encrypted HTTPS packets.[[3]](https://www.bleepingcomputer.com/news/security/new-godlua-malware-evades-traffic-monitoring-via-dns-over-https/)

Adversaries may also leverage [Protocol Tunneling](https://attack.mitre.org/techniques/T1572) in conjunction with [Proxy](https://attack.mitre.org/techniques/T1090) and/or [Protocol or Service Impersonation](https://attack.mitre.org/techniques/T1001/003) to further conceal C2 communications and infrastructure.

ID: T1572

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Command and Control](https://attack.mitre.org/tactics/TA0011)

ⓘ

Platforms: ESXi, Linux, Windows, macOS

Version: 1.1

Created: 15 March 2020

Last Modified: 03 September 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1572/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1572/)

## Procedure Examples

| ID  | Name  | Description

|  [C0034](https://attack.mitre.org/campaigns/C0034)   |  [2022 Ukraine Electric Power Attack](https://attack.mitre.org/campaigns/C0034)   |

During the [2022 Ukraine Electric Power Attack](https://attack.mitre.org/campaigns/C0034), [Sandworm Team](https://attack.mitre.org/groups/G0034) deployed the GOGETTER tunneler software to establish a "Yamux" TLS-based C2 channel with an external server(s).[[4]](https://www.mandiant.com/resources/blog/sandworm-disrupts-power-ukraine-operational-technology)

|  [C0064](https://attack.mitre.org/campaigns/C0064)   |  [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064)   |

During the [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064), the adversary instructed Claude Code to connect to the victim devices through SSH tunnels.[[5]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

During the [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064), the adversary also utilized Chisel SOCKS proxies.[[5]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

|  [S9015](https://attack.mitre.org/software/S9015)   |  [BRICKSTORM](https://attack.mitre.org/software/S9015)   |

[BRICKSTORM](https://attack.mitre.org/software/S9015) has utilized a SOCKS proxy to tunnel access within the victim network and exfiltrate files from internal shares, code repositories, and other endpoints.[[6]](https://www.crowdstrike.com/en-us/blog/warp-panda-cloud-threats/)[[7]](https://www.cisa.gov/news-events/analysis-reports/ar25-338a)[[8]](https://www.picussecurity.com/resource/blog/brickstorm-malware-unc5221-targets-tech-and-legal-sectors-in-the-united-states)[[9]](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-post-exploitation-lateral-movement)[[10]](https://blog.nviso.eu/wp-content/uploads/2025/04/NVISO-BRICKSTORM-Report.pdf)[[11]](https://www.resecurity.com/blog/article/f5-big-ip-source-code-leak-tied-to-state-linked-campaigns-using-brickstorm-backdoor)[[12]](https://cloud.google.com/blog/topics/threat-intelligence/brickstorm-espionage-campaign) [BRICKSTORM](https://attack.mitre.org/software/S9015) has also leveraged Yamux for combining multiple concurrent logical streams over a single a socket.[[7]](https://www.cisa.gov/news-events/analysis-reports/ar25-338a)[[10]](https://blog.nviso.eu/wp-content/uploads/2025/04/NVISO-BRICKSTORM-Report.pdf)[[11]](https://www.resecurity.com/blog/article/f5-big-ip-source-code-leak-tied-to-state-linked-campaigns-using-brickstorm-backdoor)

|  [S1063](https://attack.mitre.org/software/S1063)   |  [Brute Ratel C4](https://attack.mitre.org/software/S1063)   |

[Brute Ratel C4](https://attack.mitre.org/software/S1063) can use DNS over HTTPS for C2.[[13]](https://unit42.paloaltonetworks.com/brute-ratel-c4-tool/)[[14]](https://www.trendmicro.com/en_us/research/22/j/black-basta-infiltrates-networks-via-qakbot-brute-ratel-and-coba.html)

|  [C0027](https://attack.mitre.org/campaigns/C0027)   |  [C0027](https://attack.mitre.org/campaigns/C0027)   |

During [C0027](https://attack.mitre.org/campaigns/C0027), [Scattered Spider](https://attack.mitre.org/groups/G1015) used SSH tunneling in targeted environments.[[15]](https://www.crowdstrike.com/blog/analysis-of-intrusion-campaign-targeting-telecom-and-bpo-companies/)

|  [C0032](https://attack.mitre.org/campaigns/C0032)   |  [C0032](https://attack.mitre.org/campaigns/C0032)   |

During the [C0032](https://attack.mitre.org/campaigns/C0032) campaign, [TEMP.Veles](https://attack.mitre.org/groups/G0088) used encrypted SSH-based PLINK tunnels to transfer tools and enable RDP connections throughout the environment.[[16]](https://www.fireeye.com/blog/threat-research/2019/04/triton-actor-ttp-profile-custom-attack-tools-detections.html)

|  [G0114](https://attack.mitre.org/groups/G0114)   |  [Chimera](https://attack.mitre.org/groups/G0114)   |

[Chimera](https://attack.mitre.org/groups/G0114) has encapsulated [Cobalt Strike](https://attack.mitre.org/software/S0154)'s C2 protocol in DNS and HTTPS.[[17]](https://web.archive.org/web/20230218064220/https://research.nccgroup.com/2021/01/12/abusing-cloud-services-to-fly-under-the-radar/)

|  [G1021](https://attack.mitre.org/groups/G1021)   |  [Cinnamon Tempest](https://attack.mitre.org/groups/G1021)   |

[Cinnamon Tempest](https://attack.mitre.org/groups/G1021) has used the Iox and NPS proxy and tunneling tools in combination create multiple connections through a single tunnel.[[18]](https://blog.sygnia.co/revealing-emperor-dragonfly-a-chinese-ransomware-group)

|  [G0080](https://attack.mitre.org/groups/G0080)   |  [Cobalt Group](https://attack.mitre.org/groups/G0080)   |

[Cobalt Group](https://attack.mitre.org/groups/G0080) has used the Plink utility to create SSH tunnels.[[19]](https://blog.talosintelligence.com/2018/07/multiple-cobalt-personality-disorder.html)[[20]](https://www.ptsecurity.com/upload/corporate/ww-en/analytics/Cobalt-Snatch-eng.pdf)[[21]](https://www.group-ib.com/blog/cobalt)

|  [S0154](https://attack.mitre.org/software/S0154)   |  [Cobalt Strike](https://attack.mitre.org/software/S0154)   |

[Cobalt Strike](https://attack.mitre.org/software/S0154) uses a custom command and control protocol that is encapsulated in HTTP, HTTPS, or DNS. In addition, it conducts peer-to-peer communication over Windows named pipes encapsulated in the SMB protocol. All protocols use their standard assigned ports.[[22]](https://web.archive.org/web/20210825130434/https://cobaltstrike.com/downloads/csmanual38.pdf)[[23]](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)

|  [C0004](https://attack.mitre.org/campaigns/C0004)   |  [CostaRicto](https://attack.mitre.org/campaigns/C0004)   |

During [CostaRicto](https://attack.mitre.org/campaigns/C0004), the threat actors set up remote SSH tunneling into the victim's environment from a malicious domain.[[24]](https://blogs.blackberry.com/en/2020/11/the-costaricto-campaign-cyber-espionage-outsourced)

|  [C0029](https://attack.mitre.org/campaigns/C0029)   |  [Cutting Edge](https://attack.mitre.org/campaigns/C0029)   |

During [Cutting Edge](https://attack.mitre.org/campaigns/C0029), threat actors used Iodine to tunnel IPv4 traffic over DNS.[[25]](https://www.mandiant.com/resources/blog/investigating-ivanti-zero-day-exploitation)

|  [S0687](https://attack.mitre.org/software/S0687)   |  [Cyclops Blink](https://attack.mitre.org/software/S0687)   |

[Cyclops Blink](https://attack.mitre.org/software/S0687) can use DNS over HTTPS (DoH) to resolve C2 nodes.[[26]](https://www.trendmicro.com/en_us/research/22/c/cyclops-blink-sets-sights-on-asus-routers--.html)

|  [S0038](https://attack.mitre.org/software/S0038)   |  [Duqu](https://attack.mitre.org/software/S0038)   |

[Duqu](https://attack.mitre.org/software/S0038) uses a custom command and control protocol that communicates over commonly used ports, and is frequently encapsulated by application layer protocols.[[27]](https://web.archive.org/web/20111213083345/https://www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/w32_duqu_the_precursor_to_the_next_stuxnet.pdf)

|  [G1003](https://attack.mitre.org/groups/G1003)   |  [Ember Bear](https://attack.mitre.org/groups/G1003)   |

[Ember Bear](https://attack.mitre.org/groups/G1003) has used ProxyChains to tunnel protocols to internal networks.[[28]](https://www.cisa.gov/sites/default/files/2024-09/aa24-249a-russian-military-cyber-actors-target-us-and-global-critical-infrastructure.pdf)

|  [G1016](https://attack.mitre.org/groups/G1016)   |  [FIN13](https://attack.mitre.org/groups/G1016)   |

[FIN13](https://attack.mitre.org/groups/G1016) has utilized web shells and Java tools for tunneling capabilities to and from compromised assets.[[29]](https://web.archive.org/web/20220105132433/https://f.hubspotusercontent30.net/hubfs/8776530/Sygnia-%20Elephant%20Beetle_Jan2022.pdf)

|  [G0037](https://attack.mitre.org/groups/G0037)   |  [FIN6](https://attack.mitre.org/groups/G0037)   |

[FIN6](https://attack.mitre.org/groups/G0037) used the Plink command-line utility to create SSH tunnels to C2 servers.[[30]](https://web.archive.org/web/20190807112824/https://www2.fireeye.com/rs/848-DID-242/images/rpt-fin6.pdf)

|  [G0046](https://attack.mitre.org/groups/G0046)   |  [FIN7](https://attack.mitre.org/groups/G0046)   |

[FIN7](https://attack.mitre.org/groups/G0046) has tunneled C2 traffic via OpenSSH.[[31]](https://blogs.blackberry.com/en/2024/04/fin7-targets-the-united-states-automotive-industry)

|  [S0173](https://attack.mitre.org/software/S0173)   |  [FLIPSIDE](https://attack.mitre.org/software/S0173)   |

[FLIPSIDE](https://attack.mitre.org/software/S0173) uses RDP to tunnel traffic from a victim environment.[[32]](https://www.youtube.com/watch?v=fevGZs0EQu8)

|  [G0117](https://attack.mitre.org/groups/G0117)   |  [Fox Kitten](https://attack.mitre.org/groups/G0117)   |

[Fox Kitten](https://attack.mitre.org/groups/G0117) has used protocol tunneling for communication and RDP activity on compromised hosts through the use of open source tools such as [ngrok](https://attack.mitre.org/software/S0508) and custom tool SSHMinion.[[33]](https://www.crowdstrike.com/blog/who-is-pioneer-kitten/)[[34]](https://us-cert.cisa.gov/ncas/alerts/aa20-259a)[[35]](https://www.clearskysec.com/wp-content/uploads/2020/12/Pay2Kitten.pdf)

|  [S1144](https://attack.mitre.org/software/S1144)   |  [FRP](https://attack.mitre.org/software/S1144)   |

[FRP](https://attack.mitre.org/software/S1144) can tunnel SSH and Unix Domain Socket communications over TCP between external nodes and exposed resources behind firewalls or NAT.[[36]](https://github.com/fatedier/frp)

|  [S1044](https://attack.mitre.org/software/S1044)   |  [FunnyDream](https://attack.mitre.org/software/S1044)   |

[FunnyDream](https://attack.mitre.org/software/S1044) can connect to HTTP proxies via TCP to create a tunnel to C2.[[37]](https://www.bitdefender.com/files/News/CaseStudies/study/379/Bitdefender-Whitepaper-Chinese-APT.pdf)

|  [S1027](https://attack.mitre.org/software/S1027)   |  [Heyoka Backdoor](https://attack.mitre.org/software/S1027)   |

[Heyoka Backdoor](https://attack.mitre.org/software/S1027) can use spoofed DNS requests to create a bidirectional tunnel between a compromised host and its C2 servers.[[38]](https://www.sentinelone.com/labs/aoqin-dragon-newly-discovered-chinese-linked-apt-has-been-quietly-spying-on-organizations-for-10-years/)

|  [S9023](https://attack.mitre.org/software/S9023)   |  [HiddenFace](https://attack.mitre.org/software/S9023)   |

[HiddenFace](https://attack.mitre.org/software/S9023) can hide its IP lookup by using DNS over HTTPS (DoH) for C2.[[39]](https://www.trendmicro.com/en_us/research/25/d/earth-kasha-updates-ttps.html)

|  [S0604](https://attack.mitre.org/software/S0604)   |  [Industroyer](https://attack.mitre.org/software/S0604)   |

[Industroyer](https://attack.mitre.org/software/S0604) attempts to perform an HTTP CONNECT via an internal proxy to establish a tunnel.[[40]](https://dragos.com/blog/crashoverride/CrashOverride-01.pdf)

|  [S1020](https://attack.mitre.org/software/S1020)   |  [Kevin](https://attack.mitre.org/software/S1020)   |

[Kevin](https://attack.mitre.org/software/S1020) can use a custom protocol tunneled through DNS or HTTP.[[41]](https://vblocalhost.com/uploads/VB2021-Kayal-etal.pdf)

|  [G0065](https://attack.mitre.org/groups/G0065)   |  [Leviathan](https://attack.mitre.org/groups/G0065)   |

[Leviathan](https://attack.mitre.org/groups/G0065) has used protocol tunneling to further conceal C2 communications and infrastructure.[[42]](https://us-cert.cisa.gov/ncas/alerts/aa21-200a)

|  [S9061](https://attack.mitre.org/software/S9061)   |  [LIGHTRAIL](https://attack.mitre.org/software/S9061)   |

[LIGHTRAIL](https://attack.mitre.org/software/S9061) has used SOCKS4A proxies to tunnel traffic.[[43]](https://cloud.google.com/blog/topics/threat-intelligence/suspected-iranian-unc1549-targets-israel-middle-east)

|  [S1141](https://attack.mitre.org/software/S1141)   |  [LunarWeb](https://attack.mitre.org/software/S1141)   |

[LunarWeb](https://attack.mitre.org/software/S1141) can run a custom binary protocol under HTTPS for C2.[[44]](https://www.welivesecurity.com/en/eset-research/moon-backdoors-lunar-landing-diplomatic-missions/)

|  [G0059](https://attack.mitre.org/groups/G0059)   |  [Magic Hound](https://attack.mitre.org/groups/G0059)   |

[Magic Hound](https://attack.mitre.org/groups/G0059) has used Plink to tunnel RDP over SSH.[[45]](https://thedfirreport.com/2021/11/15/exchange-exploit-leads-to-domain-wide-ransomware/)

|  [S1015](https://attack.mitre.org/software/S1015)   |  [Milan](https://attack.mitre.org/software/S1015)   |

[Milan](https://attack.mitre.org/software/S1015) can use a custom protocol tunneled through DNS or HTTP.[[41]](https://vblocalhost.com/uploads/VB2021-Kayal-etal.pdf)

|  [G0129](https://attack.mitre.org/groups/G0129)   |  [Mustang Panda](https://attack.mitre.org/groups/G0129)   |

[Mustang Panda](https://attack.mitre.org/groups/G0129) has leveraged OpenSSH (sshd.exe) to execute commands, transfer files and spread across the environment communicating over SMB port 445.[[46]](https://unit42.paloaltonetworks.com/stately-taurus-abuses-vscode-southeast-asian-espionage/)

|  [S0699](https://attack.mitre.org/software/S0699)   |  [Mythic](https://attack.mitre.org/software/S0699)   |

[Mythic](https://attack.mitre.org/software/S0699) can use SOCKS proxies to tunnel traffic through another protocol.[[47]](https://docs.mythic-c2.net/)

|  [S1189](https://attack.mitre.org/software/S1189)   |  [Neo-reGeorg](https://attack.mitre.org/software/S1189)   |

[Neo-reGeorg](https://attack.mitre.org/software/S1189) can tunnel data in and out of targeted networks.[[48]](https://github.com/L-codes/Neo-reGeorg/blob/master/README-en.md)

|  [S0508](https://attack.mitre.org/software/S0508)   |  [ngrok](https://attack.mitre.org/software/S0508)   |

[ngrok](https://attack.mitre.org/software/S0508) can tunnel RDP and other services securely over internet connections.[[49]](https://www.fireeye.com/blog/threat-research/2020/05/tactics-techniques-procedures-associated-with-maze-ransomware-incidents.html)[[50]](https://cyware.com/news/cyber-attackers-leverage-tunneling-service-to-drop-lokibot-onto-victims-systems-6f610e44)[[51]](https://blog.malwarebytes.com/threat-analysis/2020/02/fraudsters-cloak-credit-card-skimmer-with-fake-content-delivery-network-ngrok-server/)[[52]](https://www.trendmicro.com/en_us/research/20/i/analysis-of-a-convoluted-attack-chain-involving-ngrok.html)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) has used the Plink utility and other tools to create tunnels to C2 servers.[[53]](https://web.archive.org/web/20230610050742/https://pan-unit42.github.io/playbook_viewer/?pb=evasive-serpens)[[54]](https://www.brighttalk.com/webcast/10703/296317/apt34-new-targeted-attack-in-the-middle-east)[[55]](https://www.fireeye.com/blog/threat-research/2019/07/hard-pass-declining-apt34-invite-to-join-their-professional-network.html)[[56]](https://www.security.com/threat-intelligence/crambus-middle-east-government)

|  [S0650](https://attack.mitre.org/software/S0650)   |  [QakBot](https://attack.mitre.org/software/S0650)   |

The [QakBot](https://attack.mitre.org/software/S0650) proxy module can encapsulate SOCKS5 protocol within its own proxy protocol.[[57]](https://securelist.com/qakbot-technical-analysis/103931/)

|  [S1187](https://attack.mitre.org/software/S1187)   |  [reGeorg](https://attack.mitre.org/software/S1187)   |

[reGeorg](https://attack.mitre.org/software/S1187) can tunnel TCP sessions including RDP, SSH, and SMB through HTTP.[[58]](https://www.fortiguard.com/encyclopedia/ips/47584/regeorg-http-tunnel)[[59]](https://www.mandiant.com/resources/blog/unc3524-eye-spy-email)[[60]](https://www.microsoft.com/en-us/security/blog/2023/06/14/cadet-blizzard-emerges-as-a-novel-and-distinct-russian-threat-actor/)

|  [G1045](https://attack.mitre.org/groups/G1045)   |  [Salt Typhoon](https://attack.mitre.org/groups/G1045)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has modified device configurations to create and use Generic Routing Encapsulation (GRE) tunnels.[[61]](https://blog.talosintelligence.com/salt-typhoon-analysis/)

|  [G1015](https://attack.mitre.org/groups/G1015)   |  [Scattered Spider](https://attack.mitre.org/groups/G1015)   |

[Scattered Spider](https://attack.mitre.org/groups/G1015) has installed protocol-tunneling tools on VMware vCenter and adversary-controlled VMs, including Teleport.sh, Chisel (configured to communicate with trycloudflare[.]com subdomains), MobaXterm, ngrok, Pinggy, and Teleport.[[62]](https://www.crowdstrike.com/en-us/blog/crowdstrike-services-observes-scattered-spider-escalate-attacks/)[[63]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-320a)

|  [C0058](https://attack.mitre.org/campaigns/C0058)   |  [SharePoint ToolShell Exploitation](https://attack.mitre.org/campaigns/C0058)   |

During [SharePoint ToolShell Exploitation](https://attack.mitre.org/campaigns/C0058), threat actors utilized [ngrok](https://attack.mitre.org/software/S0508) tunnels to deliver PowerShell payloads.[[64]](https://www.microsoft.com/en-us/security/blog/2025/07/22/disrupting-active-exploitation-of-on-premises-sharepoint-vulnerabilities/#storm-2603)

|  [S9024](https://attack.mitre.org/software/S9024)   |  [SPAWNCHIMERA](https://attack.mitre.org/software/S9024)   |

[SPAWNCHIMERA](https://attack.mitre.org/software/S9024) has created SSH tunnels to facilitate C2 communications.[[65]](https://www.cisa.gov/news-events/analysis-reports/ar25-087a)[[66]](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-connect-secure-vpn-zero-day)[[9]](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-post-exploitation-lateral-movement)

|  [S0022](https://attack.mitre.org/software/S0022)   |  [Uroburos](https://attack.mitre.org/software/S0022)   |

[Uroburos](https://attack.mitre.org/software/S0022) has the ability to communicate over custom communications methodologies that ride over common network protocols including raw TCP and UDP sockets, HTTP, SMTP, and DNS.[[67]](https://www.cisa.gov/sites/default/files/2023-05/aa23-129a_snake_malware_2.pdf)

|  [G1055](https://attack.mitre.org/groups/G1055)   |  [VOID MANTICORE](https://attack.mitre.org/groups/G1055)   |

[VOID MANTICORE](https://attack.mitre.org/groups/G1055) has used tunneling tools to facilitate destructive attacks on compromised devices.[[68]](https://research.checkpoint.com/2026/handala-hack-unveiling-groups-modus-operandi/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1037](https://attack.mitre.org/mitigations/M1037)   |  [Filter Network Traffic](https://attack.mitre.org/mitigations/M1037)   |

Consider filtering network traffic to untrusted or known bad domains and resources.

|  [M1031](https://attack.mitre.org/mitigations/M1031)   |  [Network Intrusion Prevention](https://attack.mitre.org/mitigations/M1031)   |

Network intrusion detection and prevention systems that use network signatures to identify traffic for specific adversary malware can be used to mitigate activity at the network level.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0538](https://attack.mitre.org/detectionstrategies/DET0538)   |  [Detection Strategy for Protocol Tunneling accross OS platforms.](https://attack.mitre.org/detectionstrategies/DET0538)   |  [AN1483](https://attack.mitre.org/detectionstrategies/DET0538#AN1483)   |

Processes such as plink.exe, ssh.exe, or netsh.exe establishing outbound network connections where traffic patterns show encapsulated protocols (e.g., RDP over SSH). Defender observations include anomalous process-to-network relationships, large asymmetric data flows, and port usage mismatches.

|  [AN1484](https://attack.mitre.org/detectionstrategies/DET0538#AN1484)   |

sshd, socat, or custom binaries initiating port forwarding or encapsulating traffic (e.g., RDP, SMB) through SSH or HTTP. Defender sees abnormal connect/bind syscalls, encrypted traffic on ports typically used for non-encrypted services, and outlier traffic volume patterns.

|  [AN1485](https://attack.mitre.org/detectionstrategies/DET0538#AN1485)   |

launchd or user-invoked processes (ssh, socat) encapsulating traffic via SSH tunnels, VPN-style tooling, or DNS-over-HTTPS clients. Defender sees outbound TLS traffic with embedded DNS or RDP payloads.

|  [AN1486](https://attack.mitre.org/detectionstrategies/DET0538#AN1486)   |

VMware daemons or user processes encapsulating traffic (e.g., guest VMs tunneling via hostd). Defender sees network services inside ESXi creating flows inconsistent with management plane traffic, such as SSH forwarding or DNS-over-HTTPS from management interfaces.

## References

-    [SSH.COM. (n.d.). SSH tunnel. Retrieved March 15, 2020.](https://www.ssh.com/ssh/tunneling)
-    [Abigail See, Zhongyuan (Aaron) Hau, Ren Jie Yow, Yoav Mazor, Omer Kidron, and Oren Biderman. (2025, February 4). The Anatomy of Abyss Locker Ransomware Attack. Retrieved April 4, 2025.](https://www.sygnia.co/blog/abyss-locker-ransomware-attack-analysis/)
-    [Gatlan, S. (2019, July 3). New Godlua Malware Evades Traffic Monitoring via DNS over HTTPS. Retrieved March 15, 2020.](https://www.bleepingcomputer.com/news/security/new-godlua-malware-evades-traffic-monitoring-via-dns-over-https/)
-    [Ken Proska, John Wolfram, Jared Wilson, Dan Black, Keith Lunden, Daniel Kapellmann Zafra, Nathan Brubaker, Tyler Mclellan, Chris Sistrunk. (2023, November 9). Sandworm Disrupts Power in Ukraine Using a Novel Attack Against Operational Technology. Retrieved March 28, 2024.](https://www.mandiant.com/resources/blog/sandworm-disrupts-power-ukraine-operational-technology)
-    [Eyal Sela. (2026, April). The AI-Assisted Breach of Mexico's Government Infrastructure. Retrieved September 2, 2026.](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)
-    [CrowdStrike. (2025, December 4). Unveiling WARP PANDA: A New Sophisticated China-Nexus Adversary. Retrieved April 16, 2026.](https://www.crowdstrike.com/en-us/blog/warp-panda-cloud-threats/)
-    [DHS/CISA. (2026, February 11). AR25-338A: BRICKSTORM Backdoor. Retrieved April 16, 2026.](https://www.cisa.gov/news-events/analysis-reports/ar25-338a)
-    [Huseyin Can Yuceel. (2025, October 1). BRICKSTORM Malware: UNC5221 Targets Tech and Legal Sectors in the United States. Retrieved April 16, 2026.](https://www.picussecurity.com/resource/blog/brickstorm-malware-unc5221-targets-tech-and-legal-sectors-in-the-united-states)
-    [Matt Lin, Austin Larsen, John Wolfram, Ashley Pearson, Josh Murchie, Lukasz Lamparski, Joseph Pisano, Ryan Hall, Ron Craft, Shawn Crew, Billy Wong, Tyler McLellan. (2024, April 4). Cutting Edge, Part 4: Ivanti Connect Secure VPN Post-Exploitation Lateral Movement Case Studies. Retrieved April 16, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-post-exploitation-lateral-movement)
-    [NVISO Incident Response. (2025, April 1). BRICKSTORM Backdoor Analysis: A Persistent Espionage Threat to European Industries. Retrieved April 16, 2026.](https://blog.nviso.eu/wp-content/uploads/2025/04/NVISO-BRICKSTORM-Report.pdf)
-    [Resecurity Threat Intelligence & Incident Analysis. (2025, October 22). F5 BIG-IP Source Code Leak Tied to State-Linked Campaigns Using BRICKSTORM Backdoor. Retrieved April 16, 2026.](https://www.resecurity.com/blog/article/f5-big-ip-source-code-leak-tied-to-state-linked-campaigns-using-brickstorm-backdoor)
-    [Sarah Yoder, John Wolfram, Ashley Pearson, Doug Bienstock, Josh Madeley, Josh Murchie, Brad Slaybaugh, Matt Lin, Geoff Carstairs, Austin Larsen. (2025, September 24). Another BRICKSTORM: Stealthy Backdoor Enabling Espionage into Tech and Legal Sectors. Retrieved April 16, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/brickstorm-espionage-campaign)
-    [Harbison, M. and Renals, P. (2022, July 5). When Pentest Tools Go Brutal: Red-Teaming Tool Being Abused by Malicious Actors. Retrieved February 1, 2023.](https://unit42.paloaltonetworks.com/brute-ratel-c4-tool/)
-    [Kenefick, I. et al. (2022, October 12). Black Basta Ransomware Gang Infiltrates Networks via QAKBOT, Brute Ratel, and Cobalt Strike. Retrieved February 6, 2023.](https://www.trendmicro.com/en_us/research/22/j/black-basta-infiltrates-networks-via-qakbot-brute-ratel-and-coba.html)
-    [Parisi, T. (2022, December 2). Not a SIMulation: CrowdStrike Investigations Reveal Intrusion Campaign Targeting Telco and BPO Companies. Retrieved June 30, 2023.](https://www.crowdstrike.com/blog/analysis-of-intrusion-campaign-targeting-telecom-and-bpo-companies/)
-    [Miller, S, et al. (2019, April 10). TRITON Actor TTP Profile, Custom Attack Tools, Detections, and ATT&CK Mapping. Retrieved April 16, 2019.](https://www.fireeye.com/blog/threat-research/2019/04/triton-actor-ttp-profile-custom-attack-tools-detections.html)
-    [Jansen, W . (2021, January 12). Abusing cloud services to fly under the radar. Retrieved September 12, 2024.](https://web.archive.org/web/20230218064220/https://research.nccgroup.com/2021/01/12/abusing-cloud-services-to-fly-under-the-radar/)
-    [Biderman, O. et al. (2022, October 3). REVEALING EMPEROR DRAGONFLY: NIGHT SKY AND CHEERSCRYPT - A SINGLE RANSOMWARE GROUP. Retrieved December 6, 2023.](https://blog.sygnia.co/revealing-emperor-dragonfly-a-chinese-ransomware-group)
-    [Svajcer, V. (2018, July 31). Multiple Cobalt Personality Disorder. Retrieved September 5, 2018.](https://blog.talosintelligence.com/2018/07/multiple-cobalt-personality-disorder.html)
-    [Positive Technologies. (2016, December 16). Cobalt Snatch. Retrieved October 9, 2018.](https://www.ptsecurity.com/upload/corporate/ww-en/analytics/Cobalt-Snatch-eng.pdf)
-    [Matveeva, V. (2017, August 15). Secrets of Cobalt. Retrieved October 10, 2018.](https://www.group-ib.com/blog/cobalt)
-    [Strategic Cyber LLC. (2017, March 14). Cobalt Strike Manual. Retrieved May 24, 2017.](https://web.archive.org/web/20210825130434/https://cobaltstrike.com/downloads/csmanual38.pdf)
-    [Strategic Cyber LLC. (2020, November 5). Cobalt Strike: Advanced Threat Tactics for Penetration Testers. Retrieved April 13, 2021.](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)
-    [The BlackBerry Research and Intelligence Team. (2020, November 12). The CostaRicto Campaign: Cyber-Espionage Outsourced. Retrieved May 24, 2021.](https://blogs.blackberry.com/en/2020/11/the-costaricto-campaign-cyber-espionage-outsourced)
-    [Lin, M. et al. (2024, January 31). Cutting Edge, Part 2: Investigating Ivanti Connect Secure VPN Zero-Day Exploitation. Retrieved February 27, 2024.](https://www.mandiant.com/resources/blog/investigating-ivanti-zero-day-exploitation)
-    [Haquebord, F. et al. (2022, March 17). Cyclops Blink Sets Sights on Asus Routers. Retrieved March 17, 2022.](https://www.trendmicro.com/en_us/research/22/c/cyclops-blink-sets-sights-on-asus-routers--.html)
-    [Symantec Security Response. (2011, November). W32.Duqu: The precursor to the next Stuxnet. Retrieved September 17, 2015.](https://web.archive.org/web/20111213083345/https://www.symantec.com/content/en/us/enterprise/media/security_response/whitepapers/w32_duqu_the_precursor_to_the_next_stuxnet.pdf)
-    [US Cybersecurity & Infrastructure Security Agency et al. (2024, September 5). Russian Military Cyber Actors Target U.S. and Global Critical Infrastructure. Retrieved September 6, 2024.](https://www.cisa.gov/sites/default/files/2024-09/aa24-249a-russian-military-cyber-actors-target-us-and-global-critical-infrastructure.pdf)
-    [Sygnia Incident Response Team. (2022, January 5). TG2003: ELEPHANT BEETLE UNCOVERING AN ORGANIZED FINANCIAL-THEFT OPERATION. Retrieved February 9, 2023.](https://web.archive.org/web/20220105132433/https://f.hubspotusercontent30.net/hubfs/8776530/Sygnia-%20Elephant%20Beetle_Jan2022.pdf)
-    [FireEye Threat Intelligence. (2016, April). Follow the Money: Dissecting the Operations of the Cyber Crime Group FIN6. Retrieved November 17, 2024.](https://web.archive.org/web/20190807112824/https://www2.fireeye.com/rs/848-DID-242/images/rpt-fin6.pdf)
-    [The BlackBerry Research and Intelligence Team. (2024, April 17). Threat Group FIN7 Targets the U.S. Automotive Industry. Retrieved May 1, 2025.](https://blogs.blackberry.com/en/2024/04/fin7-targets-the-united-states-automotive-industry)
-    [Bromiley, M. and Lewis, P. (2016, October 7). Attacking the Hospitality and Gaming Industries: Tracking an Attacker Around the World in 7 Years. Retrieved October 6, 2017.](https://www.youtube.com/watch?v=fevGZs0EQu8)
-    [Orleans, A. (2020, August 31). Who Is PIONEER KITTEN?. Retrieved December 21, 2020.](https://www.crowdstrike.com/blog/who-is-pioneer-kitten/)
-    [CISA. (2020, September 15). Iran-Based Threat Actor Exploits VPN Vulnerabilities. Retrieved December 21, 2020.](https://us-cert.cisa.gov/ncas/alerts/aa20-259a)

-    [ClearSky. (2020, December 17). Pay2Key Ransomware – A New Campaign by Fox Kitten. Retrieved December 21, 2020.](https://www.clearskysec.com/wp-content/uploads/2020/12/Pay2Kitten.pdf)
-    [fatedier. (n.d.). What is frp?. Retrieved July 10, 2024.](https://github.com/fatedier/frp)
-    [Vrabie, V. (2020, November). Dissecting a Chinese APT Targeting South Eastern Asian Government Institutions. Retrieved September 19, 2022.](https://www.bitdefender.com/files/News/CaseStudies/study/379/Bitdefender-Whitepaper-Chinese-APT.pdf)
-    [Chen, Joey. (2022, June 9). Aoqin Dragon | Newly-Discovered Chinese-linked APT Has Been Quietly Spying On Organizations For 10 Years. Retrieved July 14, 2022.](https://www.sentinelone.com/labs/aoqin-dragon-newly-discovered-chinese-linked-apt-has-been-quietly-spying-on-organizations-for-10-years/)
-    [Hiroaki, H. (2025, April 30). Earth Kasha Updates TTPs in Latest Campaign Targeting Taiwan and Japan. Retrieved April 17, 2026.](https://www.trendmicro.com/en_us/research/25/d/earth-kasha-updates-ttps.html)
-    [Dragos Inc.. (2017, June 13). CRASHOVERRIDE Analysis of the Threat to Electric Grid Operations. Retrieved December 18, 2020.](https://dragos.com/blog/crashoverride/CrashOverride-01.pdf)
-    [Kayal, A. et al. (2021, October). LYCEUM REBORN: COUNTERINTELLIGENCE IN THE MIDDLE EAST. Retrieved June 14, 2022.](https://vblocalhost.com/uploads/VB2021-Kayal-etal.pdf)
-    [CISA. (2021, July 19). (AA21-200A) Joint Cybersecurity Advisory – Tactics, Techniques, and Procedures of Indicted APT40 Actors Associated with China’s MSS Hainan State Security Department. Retrieved August 12, 2021.](https://us-cert.cisa.gov/ncas/alerts/aa21-200a)
-    [Rozmann, O., et al. (2024, February 27). When Cats Fly: Suspected Iranian Threat Actor UNC1549 Targets Israeli and Middle East Aerospace and Defense Sectors. Retrieved September 28, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/suspected-iranian-unc1549-targets-israel-middle-east)
-    [Jurčacko, F. (2024, May 15). To the Moon and back(doors): Lunar landing in diplomatic missions. Retrieved June 26, 2024.](https://www.welivesecurity.com/en/eset-research/moon-backdoors-lunar-landing-diplomatic-missions/)
-    [DFIR Report. (2021, November 15). Exchange Exploit Leads to Domain Wide Ransomware. Retrieved January 5, 2023.](https://thedfirreport.com/2021/11/15/exchange-exploit-leads-to-domain-wide-ransomware/)
-    [Tom Fakterman. (2024, September 6). Chinese APT Abuses VSCode to Target Government in Asia. Retrieved March 24, 2025.](https://unit42.paloaltonetworks.com/stately-taurus-abuses-vscode-southeast-asian-espionage/)
-    [Thomas, C. (n.d.). Mythc Documentation. Retrieved March 25, 2022.](https://docs.mythic-c2.net/)
-    [L-Codes. (2019). Neo-reGeorg. Retrieved December 4, 2024.](https://github.com/L-codes/Neo-reGeorg/blob/master/README-en.md)
-    [Kennelly, J., Goody, K., Shilko, J. (2020, May 7). Navigating the MAZE: Tactics, Techniques and Procedures Associated With MAZE Ransomware Incidents. Retrieved May 18, 2020.](https://www.fireeye.com/blog/threat-research/2020/05/tactics-techniques-procedures-associated-with-maze-ransomware-incidents.html)
-    [Cyware. (2019, May 29). Cyber attackers leverage tunneling service to drop Lokibot onto victims’ systems. Retrieved September 15, 2020.](https://cyware.com/news/cyber-attackers-leverage-tunneling-service-to-drop-lokibot-onto-victims-systems-6f610e44)
-    [Segura, J. (2020, February 26). Fraudsters cloak credit card skimmer with fake content delivery network, ngrok server. Retrieved September 15, 2020.](https://blog.malwarebytes.com/threat-analysis/2020/02/fraudsters-cloak-credit-card-skimmer-with-fake-content-delivery-network-ngrok-server/)
-    [Borja, A. Camba, A. et al (2020, September 14). Analysis of a Convoluted Attack Chain Involving Ngrok. Retrieved September 15, 2020.](https://www.trendmicro.com/en_us/research/20/i/analysis-of-a-convoluted-attack-chain-involving-ngrok.html)
-    [Unit42. (2016, May 1). Evasive Serpens Unit 42 Playbook Viewer. Retrieved February 6, 2023.](https://web.archive.org/web/20230610050742/https://pan-unit42.github.io/playbook_viewer/?pb=evasive-serpens)
-    [Davis, S. and Caban, D. (2017, December 19). APT34 - New Targeted Attack in the Middle East. Retrieved December 20, 2017.](https://www.brighttalk.com/webcast/10703/296317/apt34-new-targeted-attack-in-the-middle-east)
-    [Bromiley, M., et al.. (2019, July 18). Hard Pass: Declining APT34’s Invite to Join Their Professional Network. Retrieved August 26, 2019.](https://www.fireeye.com/blog/threat-research/2019/07/hard-pass-declining-apt34-invite-to-join-their-professional-network.html)
-    [Symantec Threat Hunter Team. (2023, October 19). Crambus: New Campaign Targets Middle Eastern Government. Retrieved November 27, 2024.](https://www.security.com/threat-intelligence/crambus-middle-east-government)
-    [Kuzmenko, A. et al. (2021, September 2). QakBot technical analysis. Retrieved September 27, 2021.](https://securelist.com/qakbot-technical-analysis/103931/)
-    [FortiGard Labs. (2019, March 12). ReGeorg.HTTP.Tunnel. Retrieved December 3, 2024.](https://www.fortiguard.com/encyclopedia/ips/47584/regeorg-http-tunnel)
-    [Mandiant. (2022, May 2). UNC3524: Eye Spy on Your Email. Retrieved August 17, 2023.](https://www.mandiant.com/resources/blog/unc3524-eye-spy-email)
-    [Microsoft Threat Intelligence. (2023, June 14). Cadet Blizzard emerges as a novel and distinct Russian threat actor. Retrieved July 10, 2023.](https://www.microsoft.com/en-us/security/blog/2023/06/14/cadet-blizzard-emerges-as-a-novel-and-distinct-russian-threat-actor/)
-    [Cisco Talos. (2025, February 20). Weathering the storm: In the midst of a Typhoon. Retrieved February 24, 2025.](https://blog.talosintelligence.com/salt-typhoon-analysis/)
-    [Counter Adversary Operations. (2025, July 2). CrowdStrike Services Observes SCATTERED SPIDER Escalate Attacks Across Industries. Retrieved October 13, 2025.](https://www.crowdstrike.com/en-us/blog/crowdstrike-services-observes-scattered-spider-escalate-attacks/)
-    [CISA. (2023, November 16). Cybersecurity Advisory: Scattered Spider (AA23-320A). Retrieved March 18, 2024.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-320a)
-    [Microsoft Threat Intelligence. (2025, July 22). Disrupting active exploitation of on-premises SharePoint vulnerabilities. Retrieved October 15, 2025.](https://www.microsoft.com/en-us/security/blog/2025/07/22/disrupting-active-exploitation-of-on-premises-sharepoint-vulnerabilities/#storm-2603)
-    [DHS/CISA. (2026, February 26). MAR-25993211-r1.v2 Ivanti Connect Secure (RESURGE): AR25-087A. Retrieved April 17, 2026.](https://www.cisa.gov/news-events/analysis-reports/ar25-087a)
-    [John Wolfram, Josh Murchie, Matt Lin, Daniel Ainsworth, Robert Wallace, Dimiter Andonov, Dhanesh Kizhakkinan, Jacob Thompson. (2025, January 8). Ivanti Connect Secure VPN Targeted in New Zero-Day Exploitation. Retrieved April 14, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-connect-secure-vpn-zero-day)
-    [FBI et al. (2023, May 9). Hunting Russian Intelligence “Snake” Malware. Retrieved June 8, 2023.](https://www.cisa.gov/sites/default/files/2023-05/aa23-129a_snake_malware_2.pdf)
-    [Check Point Research. (2026, March 12). “Handala Hack” – Unveiling Group’s Modus Operandi. Retrieved April 20, 2026.](https://research.checkpoint.com/2026/handala-hack-unveiling-groups-modus-operandi/)

×
