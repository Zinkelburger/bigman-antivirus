---
url: "https://attack.mitre.org/techniques/T1090/"
title: "Proxy, Technique T1090 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:31:18Z"
via: "live"
sha256: "20792cad8848f2ea662c8f7ede6c94ebb84b71ab5241c131b5b970776c2e67d5"
wayback: "https://web.archive.org/web/20261015135417/https://attack.mitre.org/techniques/T1090/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Proxy, Technique T1090 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1090/>
> Snapshot: <https://web.archive.org/web/20261015135417/https://attack.mitre.org/techniques/T1090/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Proxy

#  Proxy

##### Sub-techniques (4)

| ID  | Name

|  [T1090.001](https://attack.mitre.org/techniques/T1090/001/)   |  [Internal Proxy](https://attack.mitre.org/techniques/T1090/001/)

|  [T1090.002](https://attack.mitre.org/techniques/T1090/002/)   |  [External Proxy](https://attack.mitre.org/techniques/T1090/002/)

|  [T1090.003](https://attack.mitre.org/techniques/T1090/003/)   |  [Multi-hop Proxy](https://attack.mitre.org/techniques/T1090/003/)

|  [T1090.004](https://attack.mitre.org/techniques/T1090/004/)   |  [Domain Fronting](https://attack.mitre.org/techniques/T1090/004/)

Adversaries may use a connection proxy to direct network traffic between systems or act as an intermediary for network communications to a command and control server to avoid direct connections to their infrastructure. Many tools exist that enable traffic redirection through proxies or port redirection, including [HTRAN](https://attack.mitre.org/software/S0040), ZXProxy, and ZXPortMap.[[1]](http://blog.trendmicro.com/trendlabs-security-intelligence/in-depth-look-apt-attack-tools-of-the-trade/) Adversaries use these types of proxies to manage command and control communications, reduce the number of simultaneous outbound network connections, provide resiliency in the face of connection loss, or to ride over existing trusted communications paths between victims to avoid suspicion. Adversaries may chain together multiple proxies to further disguise the source of malicious traffic.

Adversaries can also take advantage of routing schemes in Content Delivery Networks (CDNs) to proxy command and control traffic.

ID: T1090

Sub-techniques:  [T1090.001](https://attack.mitre.org/techniques/T1090/001), [T1090.002](https://attack.mitre.org/techniques/T1090/002), [T1090.003](https://attack.mitre.org/techniques/T1090/003), [T1090.004](https://attack.mitre.org/techniques/T1090/004)

ⓘ

Tactic: [Command and Control](https://attack.mitre.org/tactics/TA0011)

ⓘ

Platforms: ESXi, Linux, Network Devices, Windows, macOS

Contributors: Heather Linn; Jon Sheedy; Walker Johnson

Version: 3.2

Created: 31 May 2017

Last Modified: 08 September 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1090/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1090/)

## Procedure Examples

| ID  | Name  | Description

|  [C0063](https://attack.mitre.org/campaigns/C0063)   |  [2025 Poland Wiper Attacks](https://attack.mitre.org/campaigns/C0063)   |

During the [2025 Poland Wiper Attacks](https://attack.mitre.org/campaigns/C0063), the adversaries utilized the rsocx tool identified as r.exe and rsocx.exe to tunnel within the internal infrastructure using a Reverse SOCKS Proxy.[[2]](https://cert.pl/uploads/docs/CERT_Polska_Energy_Sector_Incident_Report_2025.pdf)[[3]](https://www.welivesecurity.com/en/eset-research/dynowiper-update-technical-analysis-attribution/)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) used a tool called CLASSFON to covertly proxy network communications.[[4]](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)

|  [S0456](https://attack.mitre.org/software/S0456)   |  [Aria-body](https://attack.mitre.org/software/S0456)   |

[Aria-body](https://attack.mitre.org/software/S0456) has the ability to use a reverse SOCKS proxy module.[[5]](https://research.checkpoint.com/2020/naikon-apt-cyber-espionage-reloaded/)

|  [S0347](https://attack.mitre.org/software/S0347)   |  [AuditCred](https://attack.mitre.org/software/S0347)   |

[AuditCred](https://attack.mitre.org/software/S0347) can utilize proxy for communications.[[6]](https://blog.trendmicro.com/trendlabs-security-intelligence/lazarus-continues-heists-mounts-attacks-on-financial-organizations-in-latin-america/)

|  [S0245](https://attack.mitre.org/software/S0245)   |  [BADCALL](https://attack.mitre.org/software/S0245)   |

[BADCALL](https://attack.mitre.org/software/S0245) functions as a proxy server between the victim and C2 server.[[7]](https://www.us-cert.gov/sites/default/files/publications/MAR-10135536-G.PDF)

|  [S1081](https://attack.mitre.org/software/S1081)   |  [BADHATCH](https://attack.mitre.org/software/S1081)   |

[BADHATCH](https://attack.mitre.org/software/S1081) can use SOCKS4 and SOCKS5 proxies to connect to actor-controlled C2 servers. [BADHATCH](https://attack.mitre.org/software/S1081) can also emulate a reverse proxy on a compromised machine to connect with actor-controlled C2 servers.[[8]](https://www.bitdefender.com/files/News/CaseStudies/study/394/Bitdefender-PR-Whitepaper-BADHATCH-creat5237-en-EN.pdf)

|  [S0268](https://attack.mitre.org/software/S0268)   |  [Bisonal](https://attack.mitre.org/software/S0268)   |

[Bisonal](https://attack.mitre.org/software/S0268) has supported use of a proxy server.[[9]](https://blog.talosintelligence.com/2020/03/bisonal-10-years-of-play.html)

|  [G0108](https://attack.mitre.org/groups/G0108)   |  [Blue Mockingbird](https://attack.mitre.org/groups/G0108)   |

[Blue Mockingbird](https://attack.mitre.org/groups/G0108) has used [FRP](https://attack.mitre.org/software/S1144), ssf, and Venom to establish SOCKS proxy connections.[[10]](https://redcanary.com/blog/blue-mockingbird-cryptominer/)

|  [C0017](https://attack.mitre.org/campaigns/C0017)   |  [C0017](https://attack.mitre.org/campaigns/C0017)   |

During [C0017](https://attack.mitre.org/campaigns/C0017), [APT41](https://attack.mitre.org/groups/G0096) used the Cloudflare CDN to proxy C2 traffic.[[11]](https://www.mandiant.com/resources/apt41-us-state-governments)

|  [C0027](https://attack.mitre.org/campaigns/C0027)   |  [C0027](https://attack.mitre.org/campaigns/C0027)   |

During [C0027](https://attack.mitre.org/campaigns/C0027), [Scattered Spider](https://attack.mitre.org/groups/G1015) installed the open-source rsocx reverse proxy tool on a targeted ESXi appliance.[[12]](https://www.crowdstrike.com/blog/analysis-of-intrusion-campaign-targeting-telecom-and-bpo-companies/)

|  [S0348](https://attack.mitre.org/software/S0348)   |  [Cardinal RAT](https://attack.mitre.org/software/S0348)   |

[Cardinal RAT](https://attack.mitre.org/software/S0348) can act as a reverse proxy.[[13]](https://researchcenter.paloaltonetworks.com/2017/04/unit42-cardinal-rat-active-two-years/)

|  [G1021](https://attack.mitre.org/groups/G1021)   |  [Cinnamon Tempest](https://attack.mitre.org/groups/G1021)   |

[Cinnamon Tempest](https://attack.mitre.org/groups/G1021) has used a customized version of the Iox port-forwarding and proxy tool.[[14]](https://blog.sygnia.co/revealing-emperor-dragonfly-a-chinese-ransomware-group)

|  [G1052](https://attack.mitre.org/groups/G1052)   |  [Contagious Interview](https://attack.mitre.org/groups/G1052)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has leveraged Astrill VPN for C2.[[15]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)

|  [G0052](https://attack.mitre.org/groups/G0052)   |  [CopyKittens](https://attack.mitre.org/groups/G0052)   |

[CopyKittens](https://attack.mitre.org/groups/G0052) has used the AirVPN service for operational activity.[[16]](https://www.microsoft.com/security/blog/2022/06/02/exposing-polonium-activity-and-infrastructure-targeting-israeli-organizations/)

|  [S0384](https://attack.mitre.org/software/S0384)   |  [Dridex](https://attack.mitre.org/software/S0384)   |

[Dridex](https://attack.mitre.org/software/S0384) contains a backconnect module for tunneling network traffic through a victim's computer. Infected computers become part of a P2P botnet that can relay C2 traffic to other infected peers.[[17]](https://www.secureworks.com/research/dridex-bugat-v5-botnet-takeover-operation)[[18]](https://research.checkpoint.com/2021/stopping-serial-killer-catching-the-next-strike/)

|  [G1006](https://attack.mitre.org/groups/G1006)   |  [Earth Lusca](https://attack.mitre.org/groups/G1006)   |

[Earth Lusca](https://attack.mitre.org/groups/G1006) adopted Cloudflare as a proxy for compromised servers.[[19]](https://www.trendmicro.com/content/dam/trendmicro/global/en/research/22/a/earth-lusca-employs-sophisticated-infrastructure-varied-tools-and-techniques/technical-brief-delving-deep-an-analysis-of-earth-lusca-operations.pdf)

|  [G0117](https://attack.mitre.org/groups/G0117)   |  [Fox Kitten](https://attack.mitre.org/groups/G0117)   |

[Fox Kitten](https://attack.mitre.org/groups/G0117) has used the open source reverse proxy tools including FRPC and Go Proxy to establish connections from C2 to local servers.[[20]](https://us-cert.cisa.gov/ncas/alerts/aa20-259a)[[21]](https://www.clearskysec.com/wp-content/uploads/2020/12/Pay2Kitten.pdf)[[22]](https://research.checkpoint.com/2020/ransomware-alert-pay2key/)

|  [S1144](https://attack.mitre.org/software/S1144)   |  [FRP](https://attack.mitre.org/software/S1144)   |

[FRP](https://attack.mitre.org/software/S1144) can proxy communications through a server in public IP space to local servers located behind a NAT or firewall.[[23]](https://github.com/fatedier/frp)

|  [S1044](https://attack.mitre.org/software/S1044)   |  [FunnyDream](https://attack.mitre.org/software/S1044)   |

[FunnyDream](https://attack.mitre.org/software/S1044) can identify and use configured proxies in a compromised network for C2 communication.[[24]](https://www.bitdefender.com/files/News/CaseStudies/study/379/Bitdefender-Whitepaper-Chinese-APT.pdf)

|  [G0047](https://attack.mitre.org/groups/G0047)   |  [Gamaredon Group](https://attack.mitre.org/groups/G0047)   |

[Gamaredon Group](https://attack.mitre.org/groups/G0047) has used the Cloudflare Tunnel client to proxy C2 traffic.[[25]](https://web-assets.esetstatic.com/wls/en/papers/white-papers/cyberespionage-gamaredon-way.pdf)

|  [S1197](https://attack.mitre.org/software/S1197)   |  [GoBear](https://attack.mitre.org/software/S1197)   |

[GoBear](https://attack.mitre.org/software/S1197) implements SOCKS5 proxy functionality.[[26]](https://medium.com/s2wblog/kimsuky-disguised-as-a-korean-company-signed-with-a-valid-certificate-to-distribute-troll-stealer-cfa5d54314e2)

|  [S0690](https://attack.mitre.org/software/S0690)   |  [Green Lambert](https://attack.mitre.org/software/S0690)   |

[Green Lambert](https://attack.mitre.org/software/S0690) can use proxies for C2 traffic.[[27]](https://objective-see.com/blog/blog_0x68.html)[[28]](https://web.archive.org/web/20211018145402/https://www.glitch-cat.com/blog/green-lambert-and-attack)

|  [S0246](https://attack.mitre.org/software/S0246)   |  [HARDRAIN](https://attack.mitre.org/software/S0246)   |

[HARDRAIN](https://attack.mitre.org/software/S0246) uses the command cmd.exe /c netsh firewall add portopening TCP 443 "adp" and makes the victim machine function as a proxy server.[[29]](https://www.us-cert.gov/sites/default/files/publications/MAR-10135536-F.pdf)

|  [S1229](https://attack.mitre.org/software/S1229)   |  [Havoc](https://attack.mitre.org/software/S1229)   |

[Havoc](https://attack.mitre.org/software/S1229) has the ability to route HTTP/S communications through designated proxies.[[30]](https://havocframework.com/docs/welcome)

|  [S0376](https://attack.mitre.org/software/S0376)   |  [HOPLIGHT](https://attack.mitre.org/software/S0376)   |

[HOPLIGHT](https://attack.mitre.org/software/S0376) has multiple proxy options that mask traffic between the malware and the remote operators.[[31]](https://www.us-cert.gov/ncas/analysis-reports/AR19-100A)

|  [S0040](https://attack.mitre.org/software/S0040)   |  [HTRAN](https://attack.mitre.org/software/S0040)   |

[HTRAN](https://attack.mitre.org/software/S0040) can proxy TCP socket connections to obfuscate command and control infrastructure.[[32]](https://web.archive.org/web/20210920193513/https://www.fireeye.com/content/dam/fireeye-www/global/en/current-threats/pdfs/wp-operation-quantum-entanglement.pdf)[[33]](https://www.ncsc.gov.uk/report/joint-report-on-publicly-available-hacking-tools)

|  [S0283](https://attack.mitre.org/software/S0283)   |  [jRAT](https://attack.mitre.org/software/S0283)   |

[jRAT](https://attack.mitre.org/software/S0283) can serve as a SOCKS proxy server.[[34]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07195002/KL_AdwindPublicReport_2016.pdf)

|  [S9044](https://attack.mitre.org/software/S9044)   |  [Kali365](https://attack.mitre.org/software/S9044)   |

[Kali365](https://attack.mitre.org/software/S9044) has leveraged Cloudflare workers as reverse proxy infrastructure.[[35]](https://arcticwolf.com/resources/blog/token-bingo-dont-let-your-code-be-the-winner/)[[36]](https://www.huntress.com/blog/kali365-device-code-phishing-kit)[[37]](https://spycloud.com/blog/kali365-anatomy-of-a-microsoft365-phishing-as-a-service-kit/)

|  [S1190](https://attack.mitre.org/software/S1190)   |  [Kapeka](https://attack.mitre.org/software/S1190)   |

[Kapeka](https://attack.mitre.org/software/S1190) can identify system proxy settings via WinHttpGetIEProxyConfigForCurrentUser() during initialization and utilize these settings for subsequent command and control operations.[[38]](https://labs.withsecure.com/content/dam/labs/docs/WithSecure-Research-Kapeka.pdf)

|  [S0487](https://attack.mitre.org/software/S0487)   |  [Kessel](https://attack.mitre.org/software/S0487)   |

[Kessel](https://attack.mitre.org/software/S0487) can use a proxy during exfiltration if set in the configuration.[[39]](https://www.welivesecurity.com/wp-content/uploads/2018/12/ESET-The_Dark_Side_of_the_ForSSHe.pdf)

|  [S1051](https://attack.mitre.org/software/S1051)   |  [KEYPLUG](https://attack.mitre.org/software/S1051)   |

[KEYPLUG](https://attack.mitre.org/software/S1051) has used Cloudflare CDN associated infrastructure to redirect C2 communications to malicious domains.[[11]](https://www.mandiant.com/resources/apt41-us-state-governments)

|  [S0669](https://attack.mitre.org/software/S0669)   |  [KOCTOPUS](https://attack.mitre.org/software/S0669)   |

[KOCTOPUS](https://attack.mitre.org/software/S0669) has deployed a modified version of Invoke-Ngrok to expose open local ports to the Internet.[[40]](https://web.archive.org/web/20211003035156/https://www.malwarebytes.com/resources/files/2021/02/lazyscripter.pdf)

|  [G1004](https://attack.mitre.org/groups/G1004)   |  [LAPSUS$](https://attack.mitre.org/groups/G1004)   |

[LAPSUS$](https://attack.mitre.org/groups/G1004) has leverage NordVPN for its egress points when targeting intended victims.[[41]](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)

|  [S1121](https://attack.mitre.org/software/S1121)   |  [LITTLELAMB.WOOLTEA](https://attack.mitre.org/software/S1121)   |

[LITTLELAMB.WOOLTEA](https://attack.mitre.org/software/S1121) has the ability to function as a SOCKS proxy.[[42]](https://www.mandiant.com/resources/blog/investigating-ivanti-exploitation-persistence)

|  [S1141](https://attack.mitre.org/software/S1141)   |  [LunarWeb](https://attack.mitre.org/software/S1141)   |

[LunarWeb](https://attack.mitre.org/software/S1141) has the ability to use a HTTP proxy server for C&C communications.[[43]](https://www.welivesecurity.com/en/eset-research/moon-backdoors-lunar-landing-diplomatic-missions/)

|  [G0059](https://attack.mitre.org/groups/G0059)   |  [Magic Hound](https://attack.mitre.org/groups/G0059)   |

[Magic Hound](https://attack.mitre.org/groups/G0059) has used Fast Reverse Proxy (FRP) for RDP traffic.[[44]](https://thedfirreport.com/2021/11/15/exchange-exploit-leads-to-domain-wide-ransomware/)

|  [G1054](https://attack.mitre.org/groups/G1054)   |  [MirrorFace](https://attack.mitre.org/groups/G1054)   |

[MirrorFace](https://attack.mitre.org/groups/G1054) has used the GO Simple Tunnel (GOST) proxy tool.[[45]](https://blogs.jpcert.or.jp/en/2024/07/mirrorface-attack-against-japanese-organisations.html)

|  [G1019](https://attack.mitre.org/groups/G1019)   |  [MoustachedBouncer](https://attack.mitre.org/groups/G1019)   |

[MoustachedBouncer](https://attack.mitre.org/groups/G1019) has used a reverse proxy tool similar to the GitHub repository revsocks.[[46]](https://www.welivesecurity.com/en/eset-research/moustachedbouncer-espionage-against-foreign-diplomats-in-belarus/)

|  [G0069](https://attack.mitre.org/groups/G0069)   |  [MuddyWater](https://attack.mitre.org/groups/G0069)   |

[MuddyWater](https://attack.mitre.org/groups/G0069) has used NordVPN to proxy phishing emails, making them appear to originate from France.[[47]](https://falconfeeds.io/blogs/the-digital-redoubt-irans-national-information-network-cyber-conflict)

|  [S1189](https://attack.mitre.org/software/S1189)   |  [Neo-reGeorg](https://attack.mitre.org/software/S1189)   |

[Neo-reGeorg](https://attack.mitre.org/software/S1189) has the ability to establish a SOCKS5 proxy on a compromised web server.[[48]](https://github.com/L-codes/Neo-reGeorg/blob/master/README-en.md)

|  [S0108](https://attack.mitre.org/software/S0108)   |  [netsh](https://attack.mitre.org/software/S0108)   |

[netsh](https://attack.mitre.org/software/S0108) can be used to set up a proxy tunnel to allow remote host access to an infected host.[[49]](https://securelist.com/fileless-attacks-against-enterprise-networks/77403/)

|  [S0198](https://attack.mitre.org/software/S0198)   |  [NETWIRE](https://attack.mitre.org/software/S0198)   |

[NETWIRE](https://attack.mitre.org/software/S0198) can implement use of proxies to pivot traffic.[[50]](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)

|  [S0508](https://attack.mitre.org/software/S0508)   |  [ngrok](https://attack.mitre.org/software/S0508)   |

[ngrok](https://attack.mitre.org/software/S0508) can be used to proxy connections to machines located behind NAT or firewalls.[[51]](https://blog.malwarebytes.com/threat-analysis/2020/02/fraudsters-cloak-credit-card-skimmer-with-fake-content-delivery-network-ngrok-server/)[[52]](https://www.zdnet.com/article/sly-malware-author-hides-cryptomining-botnet-behind-ever-shifting-proxy-service/)

|  [C0048](https://attack.mitre.org/campaigns/C0048)   |  [Operation MidnightEclipse](https://attack.mitre.org/campaigns/C0048)   |

During [Operation MidnightEclipse](https://attack.mitre.org/campaigns/C0048), threat actors used the GO Simple Tunnel reverse proxy tool.[[53]](https://www.volexity.com/blog/2024/04/12/zero-day-exploitation-of-unauthenticated-remote-code-execution-vulnerability-in-globalprotect-cve-2024-3400/)

|  [C0013](https://attack.mitre.org/campaigns/C0013)   |  [Operation Sharpshooter](https://attack.mitre.org/campaigns/C0013)   |

For [Operation Sharpshooter](https://attack.mitre.org/campaigns/C0013), the threat actors used the ExpressVPN service to hide their location.[[54]](https://www.bleepingcomputer.com/news/security/op-sharpshooter-connected-to-north-koreas-lazarus-group/)

|  [C0014](https://attack.mitre.org/campaigns/C0014)   |  [Operation Wocao](https://attack.mitre.org/campaigns/C0014)   |

During [Operation Wocao](https://attack.mitre.org/campaigns/C0014), threat actors used a custom proxy tool called "Agent" which has support for multiple hops.[[55]](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)

|  [S0435](https://attack.mitre.org/software/S0435)   |  [PLEAD](https://attack.mitre.org/software/S0435)   |

[PLEAD](https://attack.mitre.org/software/S0435) has the ability to proxy network communications.[[56]](https://blogs.jpcert.or.jp/en/2018/03/malware-tscooki-7aa0.html)

|  [G1005](https://attack.mitre.org/groups/G1005)   |  [POLONIUM](https://attack.mitre.org/groups/G1005)   |

[POLONIUM](https://attack.mitre.org/groups/G1005) has used the AirVPN service for operational activity.[[16]](https://www.microsoft.com/security/blog/2022/06/02/exposing-polonium-activity-and-infrastructure-targeting-israeli-organizations/)

|  [S0378](https://attack.mitre.org/software/S0378)   |  [PoshC2](https://attack.mitre.org/software/S0378)   |

[PoshC2](https://attack.mitre.org/software/S0378) contains modules that allow for use of proxies in command and control.[[57]](https://github.com/nettitude/PoshC2_Python)

|  [S0262](https://attack.mitre.org/software/S0262)   |  [QuasarRAT](https://attack.mitre.org/software/S0262)   |

[QuasarRAT](https://attack.mitre.org/software/S0262) can communicate over a reverse proxy using SOCKS5.[[58]](https://github.com/quasar/QuasarRAT)[[59]](https://www.volexity.com/blog/2018/06/07/patchwork-apt-group-targets-us-think-tanks/)

|  [S0629](https://attack.mitre.org/software/S0629)   |  [RainyDay](https://attack.mitre.org/software/S0629)   |

[RainyDay](https://attack.mitre.org/software/S0629) can use proxy tools including boost_proxy_client for reverse proxy functionality.[[60]](https://www.bitdefender.com/files/News/CaseStudies/study/396/Bitdefender-PR-Whitepaper-NAIKON-creat5397-en-EN.pdf)

|  [S1212](https://attack.mitre.org/software/S1212)   |  [RansomHub](https://attack.mitre.org/software/S1212)   |

[RansomHub](https://attack.mitre.org/software/S1212) can use a proxy to connect to remote SFTP servers.[[61]](https://www.group-ib.com/blog/ransomhub-never-sleeps-episode-1/)

|  [C0047](https://attack.mitre.org/campaigns/C0047)   |  [RedDelta Modified PlugX Infection Chain Operations](https://attack.mitre.org/campaigns/C0047)   |

[Mustang Panda](https://attack.mitre.org/groups/G0129) proxied communication through the Cloudflare CDN service during [RedDelta Modified PlugX Infection Chain Operations](https://attack.mitre.org/campaigns/C0047).[[62]](https://go.recordedfuture.com/hubfs/reports/cta-cn-2025-0109.pdf)

|  [C0056](https://attack.mitre.org/campaigns/C0056)   |  [RedPenguin](https://attack.mitre.org/campaigns/C0056)   |

During [RedPenguin](https://attack.mitre.org/campaigns/C0056), [UNC3886](https://attack.mitre.org/groups/G1048) used malware capable of establishing a SOCKS proxy connection to a specified IP and port.[[63]](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-targets-juniper-routers)[[64]](https://supportportal.juniper.net/sfc/servlet.shepherd/document/download/069Dp00000FzdmIIAR?operationContext=S1)

|  [S1187](https://attack.mitre.org/software/S1187)   |  [reGeorg](https://attack.mitre.org/software/S1187)   |

[reGeorg](https://attack.mitre.org/software/S1187) can establish an HTTP or SOCKS proxy to tunnel data in and out of a network.[[65]](https://github.com/xl7dev/WebShell/tree/master/reGeorg-master)[[66]](https://www.fortiguard.com/encyclopedia/ips/47584/regeorg-http-tunnel)[[67]](https://www.mandiant.com/resources/blog/unc3524-eye-spy-email)

|  [S0332](https://attack.mitre.org/software/S0332)   |  [Remcos](https://attack.mitre.org/software/S0332)   |

[Remcos](https://attack.mitre.org/software/S0332) uses the infected hosts as SOCKS5 proxies to allow for tunneling and proxying.[[68]](https://web.archive.org/web/20180124082756/https://www.riskiq.com/blog/labs/spear-phishing-turkish-defense-contractors/)[[69]](https://www.fortinet.com/blog/threat-research/new-campaign-uses-remcos-rat-to-exploit-victims)

|  [S1210](https://attack.mitre.org/software/S1210)   |  [Sagerunex](https://attack.mitre.org/software/S1210)   |

[Sagerunex](https://attack.mitre.org/software/S1210) uses several proxy configuration settings to ensure connectivity.[[70]](https://blog.talosintelligence.com/lotus-blossom-espionage-group/)

|  [C0059](https://attack.mitre.org/campaigns/C0059)   |  [Salesforce Data Exfiltration](https://attack.mitre.org/campaigns/C0059)   |

During [Salesforce Data Exfiltration](https://attack.mitre.org/campaigns/C0059), threat actors used Mullvad VPN IPs to proxy voice phishing calls.[[71]](https://cloud.google.com/blog/topics/threat-intelligence/voice-phishing-data-extortion)

|  [G1045](https://attack.mitre.org/groups/G1045)   |  [Salt Typhoon](https://attack.mitre.org/groups/G1045)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has utilized virtual private servers and compromised routers as proxies for malicious communications.[[72]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  [S1099](https://attack.mitre.org/software/S1099)   |  [Samurai](https://attack.mitre.org/software/S1099)   |

[Samurai](https://attack.mitre.org/software/S1099) has the ability to proxy connections to specified remote IPs and ports through a a proxy module.[[73]](https://securelist.com/toddycat/106799/)

|  [G0034](https://attack.mitre.org/groups/G0034)   |  [Sandworm Team](https://attack.mitre.org/groups/G0034)   |

[Sandworm Team](https://attack.mitre.org/groups/G0034)'s BCS-server tool can create an internal proxy server to redirect traffic from the adversary-controlled C2 to internal servers which may not be connected to the internet, but are interconnected locally.[[74]](https://www.welivesecurity.com/2016/12/13/rise-telebots-analyzing-disruptive-killdisk-attacks/)

|  [G1015](https://attack.mitre.org/groups/G1015)   |  [Scattered Spider](https://attack.mitre.org/groups/G1015)   |

[Scattered Spider](https://attack.mitre.org/groups/G1015) has used proxy networks to hamper detection and has installed legitimate proxy tools on VMware vCenter and adversary-controlled VMs.[[75]](https://www.crowdstrike.com/en-us/blog/crowdstrike-services-observes-scattered-spider-escalate-attacks/)[[76]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-320a)

|  [S0461](https://attack.mitre.org/software/S0461)   |  [SDBbot](https://attack.mitre.org/software/S0461)   |

[SDBbot](https://attack.mitre.org/software/S0461) has the ability to use port forwarding to establish a proxy between a target host and C2.[[77]](https://www.proofpoint.com/us/threat-insight/post/ta505-distributes-new-sdbbot-remote-access-trojan-get2-downloader)

|  [C0058](https://attack.mitre.org/campaigns/C0058)   |  [SharePoint ToolShell Exploitation](https://attack.mitre.org/campaigns/C0058)   |

During [SharePoint ToolShell Exploitation](https://attack.mitre.org/campaigns/C0058), threat actors used Fast Reverse Proxy to communicate with C2.[[78]](https://www.microsoft.com/en-us/security/blog/2025/07/22/disrupting-active-exploitation-of-on-premises-sharepoint-vulnerabilities/#storm-2603)[[79]](https://www.welivesecurity.com/en/eset-research/toolshell-an-all-you-can-eat-buffet-for-threat-actors/)

|  [S0273](https://attack.mitre.org/software/S0273)   |  [Socksbot](https://attack.mitre.org/software/S0273)   |

[Socksbot](https://attack.mitre.org/software/S0273) can start SOCKS proxy threads.[[80]](https://documents.trendmicro.com/assets/tech-brief-untangling-the-patchwork-cyberespionage-group.pdf)

|  [S0615](https://attack.mitre.org/software/S0615)   |  [SombRAT](https://attack.mitre.org/software/S0615)   |

[SombRAT](https://attack.mitre.org/software/S0615) has the ability to use an embedded SOCKS proxy in C2 communications.[[81]](https://us-cert.cisa.gov/ncas/analysis-reports/ar21-126a)

|  [S9051](https://attack.mitre.org/software/S9051)   |  [The Gentlemen Ransomware](https://attack.mitre.org/software/S9051)   |

[The Gentlemen Ransomware](https://attack.mitre.org/software/S9051) has utilized SOCKS proxy connections for C2.[[82]](https://www.huntress.com/blog/the-gentlemen-ransomware-defense-evasion-ttps)

|  [S0436](https://attack.mitre.org/software/S0436)   |  [TSCookie](https://attack.mitre.org/software/S0436)   |

[TSCookie](https://attack.mitre.org/software/S0436) has the ability to proxy communications with command and control (C2) servers.[[83]](https://blogs.jpcert.or.jp/en/2019/09/tscookie-loader.html)

|  [G0010](https://attack.mitre.org/groups/G0010)   |  [Turla](https://attack.mitre.org/groups/G0010)   |

[Turla](https://attack.mitre.org/groups/G0010) RPC backdoors have included local UPnP RPC proxies.[[84]](https://www.welivesecurity.com/2019/05/29/turla-powershell-usage/)

|  [S0263](https://attack.mitre.org/software/S0263)   |  [TYPEFRAME](https://attack.mitre.org/software/S0263)   |

A [TYPEFRAME](https://attack.mitre.org/software/S0263) variant can force the compromised system to function as a proxy server.[[85]](https://www.us-cert.gov/ncas/analysis-reports/AR18-165A)

|  [S0386](https://attack.mitre.org/software/S0386)   |  [Ursnif](https://attack.mitre.org/software/S0386)   |

[Ursnif](https://attack.mitre.org/software/S0386) has used a peer-to-peer (P2P) network for C2.[[86]](https://www.cyber.nj.gov/threat-landscape/malware/trojans/ursnif)[[87]](https://www.proofpoint.com/us/threat-insight/post/ursnif-variant-dreambot-adds-tor-functionality)

|  [S0207](https://attack.mitre.org/software/S0207)   |  [Vasport](https://attack.mitre.org/software/S0207)   |

[Vasport](https://attack.mitre.org/software/S0207) is capable of tunneling though a proxy.[[88]](https://www.symantec.com/security_response/writeup.jsp?docid=2012-051606-5938-99)

|  [G1017](https://attack.mitre.org/groups/G1017)   |  [Volt Typhoon](https://attack.mitre.org/groups/G1017)   |

[Volt Typhoon](https://attack.mitre.org/groups/G1017) has used compromised devices and customized versions of open source tools such as [FRP](https://attack.mitre.org/software/S1144) (Fast Reverse Proxy), Earthworm, and [Impacket](https://attack.mitre.org/software/S0357) to proxy network traffic.[[89]](https://www.microsoft.com/en-us/security/blog/2023/05/24/volt-typhoon-targets-us-critical-infrastructure-with-living-off-the-land-techniques/)[[90]](https://media.defense.gov/2023/May/24/2003229517/-1/-1/0/CSA_Living_off_the_Land.PDF)[[91]](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)

|  [S0670](https://attack.mitre.org/software/S0670)   |  [WarzoneRAT](https://attack.mitre.org/software/S0670)   |

[WarzoneRAT](https://attack.mitre.org/software/S0670) has the capability to act as a reverse proxy.[[92]](https://research.checkpoint.com/2020/warzone-behind-the-enemy-lines/)

|  [G0124](https://attack.mitre.org/groups/G0124)   |  [Windigo](https://attack.mitre.org/groups/G0124)   |

[Windigo](https://attack.mitre.org/groups/G0124) has delivered a generic Windows proxy Win32/Glubteta.M. [Windigo](https://attack.mitre.org/groups/G0124) has also used multiple reverse proxy chains as part of their C2 infrastructure.[[93]](https://www.welivesecurity.com/2014/03/18/operation-windigo-the-vivisection-of-a-large-linux-server-side-credential-stealing-malware-campaign/)

|  [S0117](https://attack.mitre.org/software/S0117)   |  [XTunnel](https://attack.mitre.org/software/S0117)   |

[XTunnel](https://attack.mitre.org/software/S0117) relays traffic between a C2 server and a victim.[[94]](https://www.crowdstrike.com/blog/bears-midst-intrusion-democratic-national-committee/)

|  [S1114](https://attack.mitre.org/software/S1114)   |  [ZIPLINE](https://attack.mitre.org/software/S1114)   |

[ZIPLINE](https://attack.mitre.org/software/S1114) can create a proxy server on compromised hosts.[[95]](https://www.mandiant.com/resources/blog/suspected-apt-targets-ivanti-zero-day)[[96]](https://www.mandiant.com/resources/blog/investigating-ivanti-zero-day-exploitation)

|  [S0412](https://attack.mitre.org/software/S0412)   |  [ZxShell](https://attack.mitre.org/software/S0412)   |

[ZxShell](https://attack.mitre.org/software/S0412) can set up an HTTP or SOCKS proxy.[[4]](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)[[97]](https://blogs.cisco.com/security/talos/opening-zxshell)

## Mitigations

| ID  | Mitigation  | Description

|  [M1037](https://attack.mitre.org/mitigations/M1037)   |  [Filter Network Traffic](https://attack.mitre.org/mitigations/M1037)   |

Traffic to known anonymity networks and C2 infrastructure can be blocked through the use of network allow and block lists. It should be noted that this kind of blocking may be circumvented by other techniques like [Domain Fronting](https://attack.mitre.org/techniques/T1090/004).

|  [M1031](https://attack.mitre.org/mitigations/M1031)   |  [Network Intrusion Prevention](https://attack.mitre.org/mitigations/M1031)   |

Network intrusion detection and prevention systems that use network signatures to identify traffic for specific adversary malware can be used to mitigate activity at the network level. Signatures are often for unique indicators within protocols and may be based on the specific C2 protocol used by a particular adversary or tool, and will likely be different across various malware families and versions. Adversaries will likely change tool C2 signatures over time or construct protocols in such a way as to avoid detection by common defensive tools. [[98]](https://arxiv.org/pdf/1408.1136)

|  [M1020](https://attack.mitre.org/mitigations/M1020)   |  [SSL/TLS Inspection](https://attack.mitre.org/mitigations/M1020)   |

If it is possible to inspect HTTPS traffic, the captures can be analyzed for connections that appear to be domain fronting.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0445](https://attack.mitre.org/detectionstrategies/DET0445)   |  [Detection of Proxy Infrastructure Setup and Traffic Bridging](https://attack.mitre.org/detectionstrategies/DET0445)   |  [AN1229](https://attack.mitre.org/detectionstrategies/DET0445#AN1229)   |

Suspicious process spawning (e.g., rundll32, svchost, powershell, or netsh) followed by network connection creation to internal hosts or uncommon external endpoints on high or non-standard ports.

|  [AN1230](https://attack.mitre.org/detectionstrategies/DET0445#AN1230)   |

User-space tools (e.g., socat, ncat, iptables, ssh) used in non-standard ways to establish reverse shells, port-forwarding, or inter-host connections. Often chained with uncommon outbound destinations or SSH tunnels.

|  [AN1231](https://attack.mitre.org/detectionstrategies/DET0445#AN1231)   |

AppleScript, LaunchAgents, or remote login services (ssh, networksetup) establishing proxy tunnels or dynamic port forwards to external IPs or alternate local hosts.

|  [AN1232](https://attack.mitre.org/detectionstrategies/DET0445#AN1232)   |

Direct use of nc, socat, or reverse tunnel scripts initiated by abnormal user contexts or unauthorized VIBs initiating connections from hypervisor to external systems.

|  [AN1233](https://attack.mitre.org/detectionstrategies/DET0445#AN1233)   |

Dynamic or static port forwarding rules added to route traffic through an internal host, or configuration changes to proxy firewall rules not aligned with baselined policy.

## References

-    [Wilhoit, K. (2013, March 4). In-Depth Look: APT Attack Tools of the Trade. Retrieved December 2, 2015.](http://blog.trendmicro.com/trendlabs-security-intelligence/in-depth-look-apt-attack-tools-of-the-trade/)
-    [CERT Polska. (2026, January 30). Energy Sector Incident Report – 29 December. Retrieved April 22, 2026.](https://cert.pl/uploads/docs/CERT_Polska_Energy_Sector_Incident_Report_2025.pdf)
-    [ESET. (2026, January 30). DynoWiper update: Technical analysis and attribution. Retrieved April 22, 2026.](https://www.welivesecurity.com/en/eset-research/dynowiper-update-technical-analysis-attribution/)
-    [Fraser, N., et al. (2019, August 7). Double DragonAPT41, a dual espionage and cyber crime operation APT41. Retrieved September 23, 2019.](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)
-    [CheckPoint. (2020, May 7). Naikon APT: Cyber Espionage Reloaded. Retrieved May 26, 2020.](https://research.checkpoint.com/2020/naikon-apt-cyber-espionage-reloaded/)
-    [Trend Micro. (2018, November 20). Lazarus Continues Heists, Mounts Attacks on Financial Organizations in Latin America. Retrieved December 3, 2018.](https://blog.trendmicro.com/trendlabs-security-intelligence/lazarus-continues-heists-mounts-attacks-on-financial-organizations-in-latin-america/)
-    [US-CERT. (2018, February 06). Malware Analysis Report (MAR) - 10135536-G. Retrieved June 7, 2018.](https://www.us-cert.gov/sites/default/files/publications/MAR-10135536-G.PDF)
-    [Vrabie, V., et al. (2021, March 10). FIN8 Returns with Improved BADHATCH Toolkit. Retrieved September 8, 2021.](https://www.bitdefender.com/files/News/CaseStudies/study/394/Bitdefender-PR-Whitepaper-BADHATCH-creat5237-en-EN.pdf)
-    [Mercer, W., et al. (2020, March 5). Bisonal: 10 years of play. Retrieved January 26, 2022.](https://blog.talosintelligence.com/2020/03/bisonal-10-years-of-play.html)
-    [Lambert, T. (2020, May 7). Introducing Blue Mockingbird. Retrieved May 26, 2020.](https://redcanary.com/blog/blue-mockingbird-cryptominer/)
-    [Rufus Brown, Van Ta, Douglas Bienstock, Geoff Ackerman, John Wolfram. (2022, March 8). Does This Look Infected? A Summary of APT41 Targeting U.S. State Governments. Retrieved July 8, 2022.](https://www.mandiant.com/resources/apt41-us-state-governments)
-    [Parisi, T. (2022, December 2). Not a SIMulation: CrowdStrike Investigations Reveal Intrusion Campaign Targeting Telco and BPO Companies. Retrieved June 30, 2023.](https://www.crowdstrike.com/blog/analysis-of-intrusion-campaign-targeting-telecom-and-bpo-companies/)
-    [Grunzweig, J.. (2017, April 20). Cardinal RAT Active for Over Two Years. Retrieved December 8, 2018.](https://researchcenter.paloaltonetworks.com/2017/04/unit42-cardinal-rat-active-two-years/)
-    [Biderman, O. et al. (2022, October 3). REVEALING EMPEROR DRAGONFLY: NIGHT SKY AND CHEERSCRYPT - A SINGLE RANSOMWARE GROUP. Retrieved December 6, 2023.](https://blog.sygnia.co/revealing-emperor-dragonfly-a-chinese-ransomware-group)
-    [Insikt Group. (2025, February 13). Inside the Scam: North Korea’s IT Worker Threat. Retrieved October 17, 2025.](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)
-    [Microsoft. (2022, June 2). Exposing POLONIUM activity and infrastructure targeting Israeli organizations. Retrieved July 1, 2022.](https://www.microsoft.com/security/blog/2022/06/02/exposing-polonium-activity-and-infrastructure-targeting-israeli-organizations/)
-    [Dell SecureWorks Counter Threat Unit Threat Intelligence. (2015, October 13). Dridex (Bugat v5) Botnet Takeover Operation. Retrieved May 31, 2019.](https://www.secureworks.com/research/dridex-bugat-v5-botnet-takeover-operation)
-    [Check Point Research. (2021, January 4). Stopping Serial Killer: Catching the Next Strike. Retrieved September 7, 2021.](https://research.checkpoint.com/2021/stopping-serial-killer-catching-the-next-strike/)
-    [Chen, J., et al. (2022). Delving Deep: An Analysis of Earth Lusca’s Operations. Retrieved July 1, 2022.](https://www.trendmicro.com/content/dam/trendmicro/global/en/research/22/a/earth-lusca-employs-sophisticated-infrastructure-varied-tools-and-techniques/technical-brief-delving-deep-an-analysis-of-earth-lusca-operations.pdf)
-    [CISA. (2020, September 15). Iran-Based Threat Actor Exploits VPN Vulnerabilities. Retrieved December 21, 2020.](https://us-cert.cisa.gov/ncas/alerts/aa20-259a)
-    [ClearSky. (2020, December 17). Pay2Key Ransomware – A New Campaign by Fox Kitten. Retrieved December 21, 2020.](https://www.clearskysec.com/wp-content/uploads/2020/12/Pay2Kitten.pdf)
-    [Check Point. (2020, November 6). Ransomware Alert: Pay2Key. Retrieved January 4, 2021.](https://research.checkpoint.com/2020/ransomware-alert-pay2key/)
-    [fatedier. (n.d.). What is frp?. Retrieved July 10, 2024.](https://github.com/fatedier/frp)
-    [Vrabie, V. (2020, November). Dissecting a Chinese APT Targeting South Eastern Asian Government Institutions. Retrieved September 19, 2022.](https://www.bitdefender.com/files/News/CaseStudies/study/379/Bitdefender-Whitepaper-Chinese-APT.pdf)
-    [Rusnák, Z. (2024, September 26). Cyberespionage the Gamaredon way: Analysis of toolset used to spy on Ukraine in 2022 and 2023. Retrieved October 30, 2024.](https://web-assets.esetstatic.com/wls/en/papers/white-papers/cyberespionage-gamaredon-way.pdf)
-    [Jiho Kim & Sebin Lee, S2W. (2024, February 7). Kimsuky disguised as a Korean company signed with a valid certificate to distribute Troll Stealer (English ver.). Retrieved January 17, 2025.](https://medium.com/s2wblog/kimsuky-disguised-as-a-korean-company-signed-with-a-valid-certificate-to-distribute-troll-stealer-cfa5d54314e2)
-    [Sandvik, Runa. (2021, October 1). Made In America: Green Lambert for OS X. Retrieved March 21, 2022.](https://objective-see.com/blog/blog_0x68.html)
-    [Sandvik, Runa. (2021, October 18). Green Lambert and ATT&CK. Retrieved November 17, 2024.](https://web.archive.org/web/20211018145402/https://www.glitch-cat.com/blog/green-lambert-and-attack)
-    [US-CERT. (2018, February 05). Malware Analysis Report (MAR) - 10135536-F. Retrieved June 11, 2018.](https://www.us-cert.gov/sites/default/files/publications/MAR-10135536-F.pdf)
-    [Ungur, P. (n.d.). HAVOC. Retrieved August 4, 2025.](https://havocframework.com/docs/welcome)
-    [US-CERT. (2019, April 10). MAR-10135536-8 – North Korean Trojan: HOPLIGHT. Retrieved April 19, 2019.](https://www.us-cert.gov/ncas/analysis-reports/AR19-100A)
-    [Haq, T., Moran, N., Vashisht, S., Scott, M. (2014, September). OPERATION QUANTUM ENTANGLEMENT. Retrieved November 17, 2024.](https://web.archive.org/web/20210920193513/https://www.fireeye.com/content/dam/fireeye-www/global/en/current-threats/pdfs/wp-operation-quantum-entanglement.pdf)
-    [The Australian Cyber Security Centre (ACSC), the Canadian Centre for Cyber Security (CCCS), the New Zealand National Cyber Security Centre (NZ NCSC), CERT New Zealand, the UK National Cyber Security Centre (UK NCSC) and the US National Cybersecurity and Communications Integration Center (NCCIC). (2018, October 11). Joint report on publicly available hacking tools. Retrieved March 11, 2019.](https://www.ncsc.gov.uk/report/joint-report-on-publicly-available-hacking-tools)
-    [Kamluk, V. & Gostev, A. (2016, February). Adwind - A Cross-Platform RAT. Retrieved April 23, 2019.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07195002/KL_AdwindPublicReport_2016.pdf)
-    [Artic Wolf Labs. (2026, April 24). Token Bingo: Don’t Let Your Code be the Winner. Retrieved July 30, 2026.](https://arcticwolf.com/resources/blog/token-bingo-dont-let-your-code-be-the-winner/)
-    [Tanner Flip. (2026, June 11). Hook, Line, and Token: Anatomy of the Kali365 / Octopi365 Phishing-as-a-Service Kit. Retrieved July 30, 2026.](https://www.huntress.com/blog/kali365-device-code-phishing-kit)
-    [Trevor Hilligoss. (2026, June 11). Kali365: Anatomy of a Microsoft 365 Phishing-as-a-Service Kit – From Telegram Hype to FBI Takedown Theater. Retrieved July 30, 2026.](https://spycloud.com/blog/kali365-anatomy-of-a-microsoft365-phishing-as-a-service-kit/)
-    [Mohammad Kazem Hassan Nejad, WithSecure. (2024, April 17). KAPEKA A novel backdoor spotted in Eastern Europe. Retrieved January 6, 2025.](https://labs.withsecure.com/content/dam/labs/docs/WithSecure-Research-Kapeka.pdf)
-    [Dumont, R., M.Léveillé, M., Porcher, H. (2018, December 1). THE DARK SIDE OF THE FORSSHE A landscape of OpenSSH backdoors. Retrieved July 16, 2020.](https://www.welivesecurity.com/wp-content/uploads/2018/12/ESET-The_Dark_Side_of_the_ForSSHe.pdf)
-    [Jazi, H. (2021, February). LazyScripter: From Empire to double RAT. Retrieved November 17, 2024.](https://web.archive.org/web/20211003035156/https://www.malwarebytes.com/resources/files/2021/02/lazyscripter.pdf)
-    [MSTIC, DART, M365 Defender. (2022, March 24). DEV-0537 Criminal Actor Targeting Organizations for Data Exfiltration and Destruction. Retrieved May 17, 2022.](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)
-    [Lin, M. et al. (2024, February 27). Cutting Edge, Part 3: Investigating Ivanti Connect Secure VPN Exploitation and Persistence Attempts. Retrieved March 1, 2024.](https://www.mandiant.com/resources/blog/investigating-ivanti-exploitation-persistence)
-    [Jurčacko, F. (2024, May 15). To the Moon and back(doors): Lunar landing in diplomatic missions. Retrieved June 26, 2024.](https://www.welivesecurity.com/en/eset-research/moon-backdoors-lunar-landing-diplomatic-missions/)
-    [DFIR Report. (2021, November 15). Exchange Exploit Leads to Domain Wide Ransomware. Retrieved January 5, 2023.](https://thedfirreport.com/2021/11/15/exchange-exploit-leads-to-domain-wide-ransomware/)
-    [Tomonaga, S. (2024, July 16). MirrorFace Attack against Japanese Organisations. Retrieved April 17, 2026.](https://blogs.jpcert.or.jp/en/2024/07/mirrorface-attack-against-japanese-organisations.html)
-    [Faou, M. (2023, August 10). MoustachedBouncer: Espionage against foreign diplomats in Belarus. Retrieved September 25, 2023.](https://www.welivesecurity.com/en/eset-research/moustachedbouncer-espionage-against-foreign-diplomats-in-belarus/)
-    [FalconFeeds.io. (2026, March 5). The Digital Redoubt: Iran’s National Information Network and the Asymmetry of Modern Cyber Conflict. Retrieved March 9, 2026.](https://falconfeeds.io/blogs/the-digital-redoubt-irans-national-information-network-cyber-conflict)
-    [L-Codes. (2019). Neo-reGeorg. Retrieved December 4, 2024.](https://github.com/L-codes/Neo-reGeorg/blob/master/README-en.md)
-    [Kaspersky Lab's Global Research and Analysis Team. (2017, February 8). Fileless attacks against enterprise networks. Retrieved February 8, 2017.](https://securelist.com/fileless-attacks-against-enterprise-networks/77403/)

-    [Lambert, T. (2020, January 29). Intro to Netwire. Retrieved January 7, 2021.](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)
-    [Segura, J. (2020, February 26). Fraudsters cloak credit card skimmer with fake content delivery network, ngrok server. Retrieved September 15, 2020.](https://blog.malwarebytes.com/threat-analysis/2020/02/fraudsters-cloak-credit-card-skimmer-with-fake-content-delivery-network-ngrok-server/)
-    [Cimpanu, C. (2018, September 13). Sly malware author hides cryptomining botnet behind ever-shifting proxy service. Retrieved September 15, 2020.](https://www.zdnet.com/article/sly-malware-author-hides-cryptomining-botnet-behind-ever-shifting-proxy-service/)
-    [Volexity Threat Research. (2024, April 12). Zero-Day Exploitation of Unauthenticated Remote Code Execution Vulnerability in GlobalProtect (CVE-2024-3400). Retrieved November 20, 2024.](https://www.volexity.com/blog/2024/04/12/zero-day-exploitation-of-unauthenticated-remote-code-execution-vulnerability-in-globalprotect-cve-2024-3400/)
-    [I. Ilascu. (2019, March 3). Op 'Sharpshooter' Connected to North Korea's Lazarus Group. Retrieved September 26, 2022.](https://www.bleepingcomputer.com/news/security/op-sharpshooter-connected-to-north-koreas-lazarus-group/)
-    [Dantzig, M. v., Schamper, E. (2019, December 19). Operation Wocao: Shining a light on one of China’s hidden hacking groups. Retrieved October 8, 2020.](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)
-    [Tomonaga, S. (2018, June 8). PLEAD Downloader Used by BlackTech. Retrieved May 6, 2020.](https://blogs.jpcert.or.jp/en/2018/03/malware-tscooki-7aa0.html)
-    [Nettitude. (2018, July 23). Python Server for PoshC2. Retrieved April 23, 2019.](https://github.com/nettitude/PoshC2_Python)
-    [MaxXor. (n.d.). QuasarRAT. Retrieved July 10, 2018.](https://github.com/quasar/QuasarRAT)
-    [Meltzer, M, et al. (2018, June 07). Patchwork APT Group Targets US Think Tanks. Retrieved July 16, 2018.](https://www.volexity.com/blog/2018/06/07/patchwork-apt-group-targets-us-think-tanks/)
-    [Vrabie, V. (2021, April 23). NAIKON – Traces from a Military Cyber-Espionage Operation. Retrieved June 29, 2021.](https://www.bitdefender.com/files/News/CaseStudies/study/396/Bitdefender-PR-Whitepaper-NAIKON-creat5397-en-EN.pdf)
-    [Alfano, V. et al. (2025, February 12). RansomHub Never Sleeps Episode 1: The evolution of modern ransomware. Retrieved March 17, 2025.](https://www.group-ib.com/blog/ransomhub-never-sleeps-episode-1/)
-    [Insikt Group. (2025, January 9). Chinese State-Sponsored RedDelta Targeted Taiwan, Mongolia, and Southeast Asia with Adapted PlugX Infection Chain. Retrieved January 14, 2025.](https://go.recordedfuture.com/hubfs/reports/cta-cn-2025-0109.pdf)
-    [Lamparski, L. et al. (2025, March 11). Ghost in the Router: China-Nexus Espionage Actor UNC3886 Targets Juniper Routers. Retrieved June 24, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-targets-juniper-routers)
-    [Juniper Networks, Cybersecurity R&D. (2025, March 11). The RedPenguin Malware Incident. Retrieved June 24, 2025.](https://supportportal.juniper.net/sfc/servlet.shepherd/document/download/069Dp00000FzdmIIAR?operationContext=S1)
-    [xl7dev. (2016). reGeorg-master. Retrieved December 3, 2024.](https://github.com/xl7dev/WebShell/tree/master/reGeorg-master)
-    [FortiGard Labs. (2019, March 12). ReGeorg.HTTP.Tunnel. Retrieved December 3, 2024.](https://www.fortiguard.com/encyclopedia/ips/47584/regeorg-http-tunnel)
-    [Mandiant. (2022, May 2). UNC3524: Eye Spy on Your Email. Retrieved August 17, 2023.](https://www.mandiant.com/resources/blog/unc3524-eye-spy-email)
-    [Klijnsma, Y. (2018, January 23). Espionage Campaign Leverages Spear Phishing, RATs Against Turkish Defense Contractors. Retrieved November 6, 2018.](https://web.archive.org/web/20180124082756/https://www.riskiq.com/blog/labs/spear-phishing-turkish-defense-contractors/)
-    [Zhang, X. (2024, November 8). New Campaign Uses Remcos RAT to Exploit Victims. Retrieved April 16, 2026.](https://www.fortinet.com/blog/threat-research/new-campaign-uses-remcos-rat-to-exploit-victims)
-    [Joey Chen, Cisco Talos. (2025, February 27). Lotus Blossom espionage group targets multiple industries with different versions of Sagerunex and hacking tools. Retrieved March 15, 2025.](https://blog.talosintelligence.com/lotus-blossom-espionage-group/)
-    [Google Threat Intelligence Group. (2025, June 4). The Cost of a Call: From Voice Phishing to Data Extortion. Retrieved October 22, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/voice-phishing-data-extortion)
-    [CISA. (2025, September 3). Countering Chinese State-Sponsored Actors Compromise of Networks Worldwide to Feed Global Espionage System. Retrieved September 22, 2026.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)
-    [Dedola, G. (2022, June 21). APT ToddyCat. Retrieved January 3, 2024.](https://securelist.com/toddycat/106799/)
-    [Cherepanov, A.. (2016, December 13). The rise of TeleBots: Analyzing disruptive KillDisk attacks. Retrieved June 10, 2020.](https://www.welivesecurity.com/2016/12/13/rise-telebots-analyzing-disruptive-killdisk-attacks/)
-    [Counter Adversary Operations. (2025, July 2). CrowdStrike Services Observes SCATTERED SPIDER Escalate Attacks Across Industries. Retrieved October 13, 2025.](https://www.crowdstrike.com/en-us/blog/crowdstrike-services-observes-scattered-spider-escalate-attacks/)
-    [CISA. (2023, November 16). Cybersecurity Advisory: Scattered Spider (AA23-320A). Retrieved March 18, 2024.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-320a)
-    [Schwarz, D. et al. (2019, October 16). TA505 Distributes New SDBbot Remote Access Trojan with Get2 Downloader. Retrieved May 29, 2020.](https://www.proofpoint.com/us/threat-insight/post/ta505-distributes-new-sdbbot-remote-access-trojan-get2-downloader)
-    [Microsoft Threat Intelligence. (2025, July 22). Disrupting active exploitation of on-premises SharePoint vulnerabilities. Retrieved October 15, 2025.](https://www.microsoft.com/en-us/security/blog/2025/07/22/disrupting-active-exploitation-of-on-premises-sharepoint-vulnerabilities/#storm-2603)
-    [ESET Research. (2025, July 24). ToolShell: An all-you-can-eat buffet for threat actors. Retrieved October 15, 2025.](https://www.welivesecurity.com/en/eset-research/toolshell-an-all-you-can-eat-buffet-for-threat-actors/)
-    [Lunghi, D., et al. (2017, December). Untangling the Patchwork Cyberespionage Group. Retrieved July 10, 2018.](https://documents.trendmicro.com/assets/tech-brief-untangling-the-patchwork-cyberespionage-group.pdf)
-    [CISA. (2021, May 6). Analysis Report (AR21-126A) FiveHands Ransomware. Retrieved June 7, 2021.](https://us-cert.cisa.gov/ncas/analysis-reports/ar21-126a)
-    [Carvey, H., O'Donnell-Welch, L. (2026, May 21). The Gentlemen (Ransomware) in Disguise: Defense Evasion and other TTPs. Retrieved September 18, 2026.](https://www.huntress.com/blog/the-gentlemen-ransomware-defense-evasion-ttps)
-    [Tomonaga, S.. (2019, September 18). Malware Used by BlackTech after Network Intrusion. Retrieved May 6, 2020.](https://blogs.jpcert.or.jp/en/2019/09/tscookie-loader.html)
-    [Faou, M. and Dumont R.. (2019, May 29). A dive into Turla PowerShell usage. Retrieved June 14, 2019.](https://www.welivesecurity.com/2019/05/29/turla-powershell-usage/)
-    [US-CERT. (2018, June 14). MAR-10135536-12 – North Korean Trojan: TYPEFRAME. Retrieved July 13, 2018.](https://www.us-cert.gov/ncas/analysis-reports/AR18-165A)
-    [NJCCIC. (2016, September 27). Ursnif. Retrieved September 12, 2024.](https://www.cyber.nj.gov/threat-landscape/malware/trojans/ursnif)
-    [Proofpoint Staff. (2016, August 25). Nightmare on Tor Street: Ursnif variant Dreambot adds Tor functionality. Retrieved June 5, 2019.](https://www.proofpoint.com/us/threat-insight/post/ursnif-variant-dreambot-adds-tor-functionality)
-    [Zhou, R. (2012, May 15). Backdoor.Vasport. Retrieved February 22, 2018.](https://www.symantec.com/security_response/writeup.jsp?docid=2012-051606-5938-99)
-    [Microsoft Threat Intelligence. (2023, May 24). Volt Typhoon targets US critical infrastructure with living-off-the-land techniques. Retrieved July 27, 2023.](https://www.microsoft.com/en-us/security/blog/2023/05/24/volt-typhoon-targets-us-critical-infrastructure-with-living-off-the-land-techniques/)
-    [NSA et al. (2023, May 24). People's Republic of China State-Sponsored Cyber Actor Living off the Land to Evade Detection. Retrieved July 27, 2023.](https://media.defense.gov/2023/May/24/2003229517/-1/-1/0/CSA_Living_off_the_Land.PDF)
-    [CISA et al.. (2024, February 7). PRC State-Sponsored Actors Compromise and Maintain Persistent Access to U.S. Critical Infrastructure. Retrieved May 15, 2024.](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)
-    [Harakhavik, Y. (2020, February 3). Warzone: Behind the enemy lines. Retrieved December 17, 2021.](https://research.checkpoint.com/2020/warzone-behind-the-enemy-lines/)
-    [Bilodeau, O., Bureau, M., Calvet, J., Dorais-Joncas, A., Léveillé, M., Vanheuverzwijn, B. (2014, March 18). Operation Windigo – the vivisection of a large Linux server‑side credential‑stealing malware campaign. Retrieved February 10, 2021.](https://www.welivesecurity.com/2014/03/18/operation-windigo-the-vivisection-of-a-large-linux-server-side-credential-stealing-malware-campaign/)
-    [Alperovitch, D.. (2016, June 15). Bears in the Midst: Intrusion into the Democratic National Committee. Retrieved August 3, 2016.](https://www.crowdstrike.com/blog/bears-midst-intrusion-democratic-national-committee/)
-    [McLellan, T. et al. (2024, January 12). Cutting Edge: Suspected APT Targets Ivanti Connect Secure VPN in New Zero-Day Exploitation. Retrieved February 27, 2024.](https://www.mandiant.com/resources/blog/suspected-apt-targets-ivanti-zero-day)
-    [Lin, M. et al. (2024, January 31). Cutting Edge, Part 2: Investigating Ivanti Connect Secure VPN Zero-Day Exploitation. Retrieved February 27, 2024.](https://www.mandiant.com/resources/blog/investigating-ivanti-zero-day-exploitation)
-    [Allievi, A., et al. (2014, October 28). Threat Spotlight: Group 72, Opening the ZxShell. Retrieved September 24, 2019.](https://blogs.cisco.com/security/talos/opening-zxshell)
-    [Gardiner, J., Cova, M., Nagaraja, S. (2014, February). Command & Control Understanding, Denying and Detecting. Retrieved September 3, 2026.](https://arxiv.org/pdf/1408.1136)

×
