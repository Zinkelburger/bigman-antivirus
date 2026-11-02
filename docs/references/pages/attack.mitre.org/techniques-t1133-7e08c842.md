---
url: "https://attack.mitre.org/techniques/T1133/"
title: "External Remote Services, Technique T1133 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:36:00Z"
via: "live"
sha256: "50904380aa3b8a3dd3242bda2ca3a1cad3e4ffdd7fc654af86591eec8d4ba8fe"
wayback: "https://web.archive.org/web/20261023090728/https://attack.mitre.org/techniques/T1133/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# External Remote Services, Technique T1133 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1133/>
> Snapshot: <https://web.archive.org/web/20261023090728/https://attack.mitre.org/techniques/T1133/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- External Remote Services

#  External Remote Services

Adversaries may leverage external-facing remote services to initially access and/or persist within a network. Remote services such as VPNs, Citrix, and other access mechanisms allow users to connect to internal enterprise network resources from external locations. There are often remote service gateways that manage connections and credential authentication for these services. Services such as [Windows Remote Management](https://attack.mitre.org/techniques/T1021/006) and [VNC](https://attack.mitre.org/techniques/T1021/005) can also be used externally.[[1]](https://support.apple.com/guide/remote-desktop/set-up-a-computer-running-vnc-software-apdbed09830/mac)

Access to [Valid Accounts](https://attack.mitre.org/techniques/T1078) to use the service is often a requirement, which could be obtained through credential pharming or by obtaining the credentials from users after compromising the enterprise network.[[2]](https://www.volexity.com/blog/2015/10/07/virtual-private-keylogging-cisco-web-vpns-leveraged-for-access-and-persistence/) Access to remote services may be used as a redundant or persistent access mechanism during an operation.

Access may also be gained through an exposed service that doesn’t require authentication. In containerized environments, this may include an exposed Docker API, Kubernetes API server, kubelet, or web application such as the Kubernetes dashboard.[[3]](https://www.trendmicro.com/en_us/research/20/f/xorddos-kaiji-botnet-malware-variants-target-exposed-docker-servers.html)[[4]](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)

Adversaries may also establish persistence on network by configuring a Tor hidden service on a compromised system. Adversaries may utilize the tool ShadowLink to facilitate the installation and configuration of the Tor hidden service. Tor hidden service is then accessible via the Tor network because ShadowLink sets up a .onion address on the compromised system. ShadowLink may be used to forward any inbound connections to RDP, allowing the adversaries to have remote access.[[5]](https://www.microsoft.com/en-us/security/blog/2025/02/12/the-badpilot-campaign-seashell-blizzard-subgroup-conducts-multiyear-global-access-operation/?ref=thestack.technology) Adversaries may get ShadowLink to persist on a system by masquerading it as an MS Defender application.[[6]](https://www.microsoft.com/en-us/security/security-insider/intelligence-reports/russian-threat-actors-dig-in-prepare-to-seize-on-war-fatigue)

ID: T1133

Sub-techniques:  No sub-techniques

ⓘ

Tactics: [Persistence](https://attack.mitre.org/tactics/TA0003), [Initial Access](https://attack.mitre.org/tactics/TA0001)

ⓘ

Platforms: Containers, Linux, Windows, macOS

Contributors: Alfredo Oliveira, Trend Micro; Ariel Shuper, Cisco; Brad Geesaman, @bradgeesaman; Daniel Oakley; David Fiser, @anu4is, Trend Micro; David Tayouri; ExtraHop; Idan Frimark, Cisco; Jay Chen, Palo Alto Networks; Liran Ravich, CardinalOps; Magno Logan, @magnologan, Trend Micro; Rory McCune, Aqua Security; Travis Smith, Tripwire; Vishwas Manral, McAfee; Yossi Weizman, Azure Defender Research Team; Yuval Avrahami, Palo Alto Networks

Version: 2.5

Created: 31 May 2017

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1133/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1133/)

## Procedure Examples

| ID  | Name  | Description

|  [C0028](https://attack.mitre.org/campaigns/C0028)   |  [2015 Ukraine Electric Power Attack](https://attack.mitre.org/campaigns/C0028)   |

During the [2015 Ukraine Electric Power Attack](https://attack.mitre.org/campaigns/C0028), [Sandworm Team](https://attack.mitre.org/groups/G0034) installed a modified Dropbear SSH client as the backdoor to target systems.[[7]](https://www.boozallen.com/content/dam/boozallen/documents/2016/09/ukraine-report-when-the-lights-went-out.pdf)

|  [C0063](https://attack.mitre.org/campaigns/C0063)   |  [2025 Poland Wiper Attacks](https://attack.mitre.org/campaigns/C0063)   |

During the [2025 Poland Wiper Attacks](https://attack.mitre.org/campaigns/C0063), threat actors leveraged the FortiGate VPN interface that was exposed to the internet to gain access to the victim environment.[[8]](https://cert.pl/uploads/docs/CERT_Polska_Energy_Sector_Incident_Report_2025.pdf)[[9]](https://5943619.hs-sites.com/hubfs/Reports/dragos-2025-poland-attack-report.pdf)

|  [G1024](https://attack.mitre.org/groups/G1024)   |  [Akira](https://attack.mitre.org/groups/G1024)   |

[Akira](https://attack.mitre.org/groups/G1024) uses compromised VPN accounts for initial access to victim networks.[[10]](https://www.secureworks.com/research/threat-profiles/gold-sahara)

|  [G0099](https://attack.mitre.org/groups/G0099)   |  [APT-C-36](https://attack.mitre.org/groups/G0099)   |

[APT-C-36](https://attack.mitre.org/groups/G0099) has used VPNs in their operational infrastructure.[[11]](https://assets.recordedfuture.com/insikt-report-pdfs/2025/cta-2025-0826.pdf)

|  [G0026](https://attack.mitre.org/groups/G0026)   |  [APT18](https://attack.mitre.org/groups/G0026)   |

[APT18](https://attack.mitre.org/groups/G0026) actors leverage legitimate credentials to log into external remote services.[[12]](https://web.archive.org/web/20210803040540/https://published-prd.lanyonevents.com/published/rsaus17/sessionsFiles/5009/HTA-F02-Detecting-and-Responding-to-Advanced-Threats-within-Exchange-Environments.pdf)

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[APT28](https://attack.mitre.org/groups/G0007) has used [Tor](https://attack.mitre.org/software/S0183) and a variety of commercial VPN services to route brute force authentication attempts.[[13]](https://media.defense.gov/2021/Jul/01/2002753896/-1/-1/1/CSA_GRU_GLOBAL_BRUTE_FORCE_CAMPAIGN_UOO158036-21.PDF)

|  [G0016](https://attack.mitre.org/groups/G0016)   |  [APT29](https://attack.mitre.org/groups/G0016)   |

[APT29](https://attack.mitre.org/groups/G0016) has used compromised identities to access networks via VPNs and Citrix.[[14]](https://www.ncsc.gov.uk/files/Advisory-APT29-targets-COVID-19-vaccine-development-V1-1.pdf)[[15]](https://www.mandiant.com/resources/blog/apt29-continues-targeting-microsoft)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) compromised an online billing/payment service using VPN access between a third-party service provider and the targeted payment service.[[16]](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)

|  [C0046](https://attack.mitre.org/campaigns/C0046)   |  [ArcaneDoor](https://attack.mitre.org/campaigns/C0046)   |

[ArcaneDoor](https://attack.mitre.org/campaigns/C0046) used WebVPN sessions commonly associated with Clientless SSLVPN services to communicate to compromised devices.[[17]](https://www.cyber.gc.ca/en/news-events/cyber-activity-impacting-cisco-asa-vpns)

|  [C0027](https://attack.mitre.org/campaigns/C0027)   |  [C0027](https://attack.mitre.org/campaigns/C0027)   |

During [C0027](https://attack.mitre.org/campaigns/C0027), [Scattered Spider](https://attack.mitre.org/groups/G1015) used Citrix and VPNs to persist in compromised environments.[[18]](https://www.crowdstrike.com/blog/analysis-of-intrusion-campaign-targeting-telecom-and-bpo-companies/)

|  [C0032](https://attack.mitre.org/campaigns/C0032)   |  [C0032](https://attack.mitre.org/campaigns/C0032)   |

During the [C0032](https://attack.mitre.org/campaigns/C0032) campaign, [TEMP.Veles](https://attack.mitre.org/groups/G0088) used VPN access to persist in the victim environment.[[19]](https://www.fireeye.com/blog/threat-research/2019/04/triton-actor-ttp-profile-custom-attack-tools-detections.html)

|  [G0114](https://attack.mitre.org/groups/G0114)   |  [Chimera](https://attack.mitre.org/groups/G0114)   |

[Chimera](https://attack.mitre.org/groups/G0114) has used legitimate credentials to login to an external VPN, Citrix, SSH, and other remote services.[[20]](https://web.archive.org/web/20231214094657/https://cycraft.com/download/CyCraft-Whitepaper-Chimera_V4.1.pdf)[[21]](https://web.archive.org/web/20230218064220/https://research.nccgroup.com/2021/01/12/abusing-cloud-services-to-fly-under-the-radar/)

|  [C0004](https://attack.mitre.org/campaigns/C0004)   |  [CostaRicto](https://attack.mitre.org/campaigns/C0004)   |

During [CostaRicto](https://attack.mitre.org/campaigns/C0004), the threat actors set up remote tunneling using an SSH tool to maintain access to a compromised environment.[[22]](https://blogs.blackberry.com/en/2020/11/the-costaricto-campaign-cyber-espionage-outsourced)

|  [S0600](https://attack.mitre.org/software/S0600)   |  [Doki](https://attack.mitre.org/software/S0600)   |

[Doki](https://attack.mitre.org/software/S0600) was executed through an open Docker daemon API port.[[23]](https://www.intezer.com/blog/cloud-security/watch-your-containers-doki-infecting-docker-servers-in-the-cloud/)

|  [G0035](https://attack.mitre.org/groups/G0035)   |  [Dragonfly](https://attack.mitre.org/groups/G0035)   |

[Dragonfly](https://attack.mitre.org/groups/G0035) has used VPNs and Outlook Web Access (OWA) to maintain access to victim networks.[[24]](https://www.us-cert.gov/ncas/alerts/TA18-074A)[[25]](https://www.cisa.gov/uscert/ncas/alerts/aa20-296a#revisions)

|  [G1003](https://attack.mitre.org/groups/G1003)   |  [Ember Bear](https://attack.mitre.org/groups/G1003)   |

[Ember Bear](https://attack.mitre.org/groups/G1003) have used VPNs both for initial access to victim environments and for persistence within them following compromise.[[26]](https://www.cisa.gov/sites/default/files/2024-09/aa24-249a-russian-military-cyber-actors-target-us-and-global-critical-infrastructure.pdf)

|  [G1016](https://attack.mitre.org/groups/G1016)   |  [FIN13](https://attack.mitre.org/groups/G1016)   |

[FIN13](https://attack.mitre.org/groups/G1016) has gained access to compromised environments via remote access services such as the corporate virtual private network (VPN).[[27]](https://www.mandiant.com/resources/blog/fin13-cybercriminal-mexico)

|  [G0053](https://attack.mitre.org/groups/G0053)   |  [FIN5](https://attack.mitre.org/groups/G0053)   |

[FIN5](https://attack.mitre.org/groups/G0053) has used legitimate VPN, Citrix, or VNC credentials to maintain access to a victim environment.[[28]](https://web.archive.org/web/20190304204552/https://www2.fireeye.com/WBNR-Are-you-ready-to-respond.html)[[29]](https://www.darkreading.com/analytics/prolific-cybercrime-gang-favors-legit-login-credentials/d/d-id/1322645?)[[30]](https://www.youtube.com/watch?v=fevGZs0EQu8)

|  [G0093](https://attack.mitre.org/groups/G0093)   |  [GALLIUM](https://attack.mitre.org/groups/G0093)   |

[GALLIUM](https://attack.mitre.org/groups/G0093) has used VPN services, including SoftEther VPN, to access and maintain persistence in victim environments.[[31]](https://www.cybereason.com/blog/operation-soft-cell-a-worldwide-campaign-against-telecommunications-providers)[[32]](https://www.microsoft.com/security/blog/2019/12/12/gallium-targeting-global-telecom/)

|  [G0115](https://attack.mitre.org/groups/G0115)   |  [GOLD SOUTHFIELD](https://attack.mitre.org/groups/G0115)   |

[GOLD SOUTHFIELD](https://attack.mitre.org/groups/G0115) has used publicly-accessible RDP and remote management and monitoring (RMM) servers to gain access to victim machines.[[33]](https://www.secureworks.com/research/revil-sodinokibi-ransomware)

|  [S0601](https://attack.mitre.org/software/S0601)   |  [Hildegard](https://attack.mitre.org/software/S0601)   |

[Hildegard](https://attack.mitre.org/software/S0601) was executed through an unsecure kubelet that allowed anonymous access to the victim environment.[[4]](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)

|  [G0004](https://attack.mitre.org/groups/G0004)   |  [Ke3chang](https://attack.mitre.org/groups/G0004)   |

[Ke3chang](https://attack.mitre.org/groups/G0004) has gained access through VPNs including with compromised accounts and stolen VPN certificates.[[34]](https://research.nccgroup.com/2018/03/10/apt15-is-alive-and-strong-an-analysis-of-royalcli-and-royaldns/)[[35]](https://www.microsoft.com/security/blog/2021/12/06/nickel-targeting-government-organizations-across-latin-america-and-europe)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has used RDP to establish persistence.[[36]](https://us-cert.cisa.gov/ncas/alerts/aa20-301a)

|  [S0599](https://attack.mitre.org/software/S0599)   |  [Kinsing](https://attack.mitre.org/software/S0599)   |

[Kinsing](https://attack.mitre.org/software/S0599) was executed in an Ubuntu container deployed via an open Docker daemon API.[[37]](https://blog.aquasec.com/threat-alert-kinsing-malware-container-vulnerability)

|  [G1004](https://attack.mitre.org/groups/G1004)   |  [LAPSUS$](https://attack.mitre.org/groups/G1004)   |

[LAPSUS$](https://attack.mitre.org/groups/G1004) has gained access to internet-facing systems and applications, including virtual private network (VPN), remote desktop protocol (RDP), and virtual desktop infrastructure (VDI) including Citrix. [[38]](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)[[39]](https://www.nccgroup.com/research-blog/lapsus-recent-techniques-tactics-and-procedures/)

|  [G0065](https://attack.mitre.org/groups/G0065)   |  [Leviathan](https://attack.mitre.org/groups/G0065)   |

[Leviathan](https://attack.mitre.org/groups/G0065) has used external remote services such as virtual private networks (VPN) to gain initial access.[[40]](https://us-cert.cisa.gov/ncas/alerts/aa21-200a)

|  [S0362](https://attack.mitre.org/software/S0362)   |  [Linux Rabbit](https://attack.mitre.org/software/S0362)   |

[Linux Rabbit](https://attack.mitre.org/software/S0362) attempts to gain access to the server via SSH.[[41]](https://www.anomali.com/blog/pulling-linux-rabbit-rabbot-malware-out-of-a-hat)

|  [S1060](https://attack.mitre.org/software/S1060)   |  [Mafalda](https://attack.mitre.org/software/S1060)   |

[Mafalda](https://attack.mitre.org/software/S1060) can establish an SSH connection from a compromised host to a server.[[42]](https://docs.google.com/document/d/1e9ZTW9b71YwFWS_18ZwDAxa-cYbV8q1wUefmKZLYVsA/edit#heading=h.lmnbtht1ikzm)

|  [C0002](https://attack.mitre.org/campaigns/C0002)   |  [Night Dragon](https://attack.mitre.org/campaigns/C0002)   |

During [Night Dragon](https://attack.mitre.org/campaigns/C0002), threat actors used compromised VPN accounts to gain access to victim systems.[[43]](https://scadahacker.com/library/Documents/Cyber_Events/McAfee%20-%20Night%20Dragon%20-%20Global%20Energy%20Cyberattacks.pdf)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) uses remote services such as VPN, Citrix, or OWA to persist in an environment.[[44]](https://www.brighttalk.com/webcast/10703/296317/apt34-new-targeted-attack-in-the-middle-east)

|  [C0012](https://attack.mitre.org/campaigns/C0012)   |  [Operation CuckooBees](https://attack.mitre.org/campaigns/C0012)   |

During [Operation CuckooBees](https://attack.mitre.org/campaigns/C0012), the threat actors enabled WinRM over HTTP/HTTPS as a backup persistence mechanism using the following command: cscript //nologo "C:\Windows\System32\winrm.vbs" set winrm/config/service@{EnableCompatibilityHttpsListener="true"}.[[45]](https://www.cybereason.com/blog/operation-cuckoobees-deep-dive-into-stealthy-winnti-techniques)

|  [C0014](https://attack.mitre.org/campaigns/C0014)   |  [Operation Wocao](https://attack.mitre.org/campaigns/C0014)   |

During [Operation Wocao](https://attack.mitre.org/campaigns/C0014), threat actors used stolen credentials to connect to the victim's network via VPN.[[46]](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)

|  [G1040](https://attack.mitre.org/groups/G1040)   |  [Play](https://attack.mitre.org/groups/G1040)   |

[Play](https://attack.mitre.org/groups/G1040) has used Remote Desktop Protocol (RDP) and Virtual Private Networks (VPN) for initial access.[[47]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-352a)[[48]](https://www.trendmicro.com/vinfo/us/security/news/ransomware-spotlight/ransomware-spotlight-play)

|  [G0034](https://attack.mitre.org/groups/G0034)   |  [Sandworm Team](https://attack.mitre.org/groups/G0034)   |

[Sandworm Team](https://attack.mitre.org/groups/G0034) has used Dropbear SSH with a hardcoded backdoor password to maintain persistence within the target network. [Sandworm Team](https://attack.mitre.org/groups/G0034) has also used VPN tunnels established in legitimate software company infrastructure to gain access to internal networks of that software company's users.[[49]](https://www.welivesecurity.com/2016/01/03/blackenergy-sshbeardoor-details-2015-attacks-ukrainian-news-media-electric-industry/)[[50]](https://www.welivesecurity.com/2017/06/30/telebots-back-supply-chain-attacks-against-ukraine/)[[51]](https://www.cert.ssi.gouv.fr/uploads/CERTFR-2021-CTI-005.pdf)[[52]](https://services.google.com/fh/files/misc/apt44-unearthing-sandworm.pdf)

|  [G1015](https://attack.mitre.org/groups/G1015)   |  [Scattered Spider](https://attack.mitre.org/groups/G1015)   |

[Scattered Spider](https://attack.mitre.org/groups/G1015) has leveraged legitimate remote management tools to maintain persistent access.[[53]](https://www.crowdstrike.com/blog/scattered-spider-attempts-to-avoid-detection-with-bring-your-own-vulnerable-driver-tactic/)

|  [G1041](https://attack.mitre.org/groups/G1041)   |  [Sea Turtle](https://attack.mitre.org/groups/G1041)   |

[Sea Turtle](https://attack.mitre.org/groups/G1041) has used external-facing SSH to achieve initial access to the IT environments of victim organizations.[[54]](https://www.huntandhackett.com/blog/turkish-espionage-campaigns)

|  [C0024](https://attack.mitre.org/campaigns/C0024)   |  [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024)   |

For the [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024), [APT29](https://attack.mitre.org/groups/G0016) used compromised identities to access networks via SSH, VPNs, and other remote access tools.[[55]](https://www.microsoft.com/security/blog/2021/03/04/goldmax-goldfinder-sibot-analyzing-nobelium-malware/)[[56]](https://www.crowdstrike.com/blog/observations-from-the-stellarparticle-campaign/)

|  [G0139](https://attack.mitre.org/groups/G0139)   |  [TeamTNT](https://attack.mitre.org/groups/G0139)   |

[TeamTNT](https://attack.mitre.org/groups/G0139) has used open-source tools such as Weave Scope to target exposed Docker API ports and gain initial access to victim environments.[[57]](https://www.intezer.com/blog/cloud-security/attackers-abusing-legitimate-cloud-monitoring-tools-to-conduct-cyber-attacks/)[[58]](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/) [TeamTNT](https://attack.mitre.org/groups/G0139) has also targeted exposed kubelets for Kubernetes environments.[[4]](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)

|  [G0027](https://attack.mitre.org/groups/G0027)   |  [Threat Group-3390](https://attack.mitre.org/groups/G0027)   |

[Threat Group-3390](https://attack.mitre.org/groups/G0027) actors look for and use VPN profiles during an operation to access the network using external VPN services.[[59]](https://www.secureworks.com/research/threat-group-3390-targets-organizations-for-cyberespionage) [Threat Group-3390](https://attack.mitre.org/groups/G0027) has also obtained OWA account credentials during intrusions that it subsequently used to attempt to regain access when evicted from a victim network.[[60]](https://www.secureworks.com/research/bronze-union)

|  [G1047](https://attack.mitre.org/groups/G1047)   |  [Velvet Ant](https://attack.mitre.org/groups/G1047)   |

[Velvet Ant](https://attack.mitre.org/groups/G1047) has leveraged access to internet-facing remote services to compromise and retain access to victim environments.[[61]](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/)

|  [G1055](https://attack.mitre.org/groups/G1055)   |  [VOID MANTICORE](https://attack.mitre.org/groups/G1055)   |

[VOID MANTICORE](https://attack.mitre.org/groups/G1055) has leveraged public facing VPN infrastructure to gain initial access to victim environments.[[62]](https://research.checkpoint.com/2026/handala-hack-unveiling-groups-modus-operandi/)

|  [G1017](https://attack.mitre.org/groups/G1017)   |  [Volt Typhoon](https://attack.mitre.org/groups/G1017)   |

[Volt Typhoon](https://attack.mitre.org/groups/G1017) has used VPNs to connect to victim environments and enable post-exploitation actions.[[63]](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)

|  [G0102](https://attack.mitre.org/groups/G0102)   |  [Wizard Spider](https://attack.mitre.org/groups/G0102)   |

[Wizard Spider](https://attack.mitre.org/groups/G0102) has accessed victim networks by using stolen credentials to access the corporate VPN infrastructure.[[64]](https://www.fireeye.com/blog/threat-research/2020/10/kegtap-and-singlemalt-with-a-ransomware-chaser.html)

## Mitigations

| ID  | Mitigation  | Description

|  [M1042](https://attack.mitre.org/mitigations/M1042)   |  [Disable or Remove Feature or Program](https://attack.mitre.org/mitigations/M1042)   |

Disable or block remotely available services that may be unnecessary.

|  [M1035](https://attack.mitre.org/mitigations/M1035)   |  [Limit Access to Resource Over Network](https://attack.mitre.org/mitigations/M1035)   |

Limit access to remote services through centrally managed concentrators such as VPNs and other managed remote access systems.

|  [M1032](https://attack.mitre.org/mitigations/M1032)   |  [Multi-factor Authentication](https://attack.mitre.org/mitigations/M1032)   |

Use strong two-factor or multi-factor authentication for remote service accounts to mitigate an adversary's ability to leverage stolen credentials, but be aware of [Multi-Factor Authentication Interception](https://attack.mitre.org/techniques/T1111) techniques for some two-factor authentication implementations.

|  [M1030](https://attack.mitre.org/mitigations/M1030)   |  [Network Segmentation](https://attack.mitre.org/mitigations/M1030)   |

Deny direct remote access to internal systems through the use of network proxies, gateways, and firewalls.

|  [M1021](https://attack.mitre.org/mitigations/M1021)   |  [Restrict Web-Based Content](https://attack.mitre.org/mitigations/M1021)   |

Restrict all traffic to and from public Tor nodes. [[65]](https://www.cisa.gov/sites/default/files/publications/AA20-183A_Defending_Against_Malicious_Cyber_Activity_Originating_from_Tor_S508C.pdf)

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0354](https://attack.mitre.org/detectionstrategies/DET0354)   |  [Behavior-chain detection for T1133 External Remote Services across Windows, Linux, macOS, Containers](https://attack.mitre.org/detectionstrategies/DET0354)   |  [AN1004](https://attack.mitre.org/detectionstrategies/DET0354#AN1004)   |

Unusual or unauthorized external remote access attempts (e.g., RDP, VPN, Citrix) → repeated failed logins followed by a successful session from uncommon geolocations or outside business hours → subsequent internal lateral movement or data exfiltration activities.

|  [AN1005](https://attack.mitre.org/detectionstrategies/DET0354#AN1005)   |

Repeated SSH, VPN, or RDP gateway authentication attempts from external IPs → subsequent successful logon → remote shell or lateral movement activity (e.g., scp/sftp).

|  [AN1006](https://attack.mitre.org/detectionstrategies/DET0354#AN1006)   |

Unexpected inbound or outbound VNC/SSH/Screen Sharing connections from external sources → repeated failed logins followed by success → remote interactive sessions or abnormal file transfers.

|  [AN1007](https://attack.mitre.org/detectionstrategies/DET0354#AN1007)   |

Connections to exposed container services (e.g., Docker API, Kubernetes API server) from unauthorized external IPs → abnormal container creation/start → lateral activity within cluster nodes.

## References

-    [Apple Support. (n.d.). Set up a computer running VNC software for Remote Desktop. Retrieved August 18, 2021.](https://support.apple.com/guide/remote-desktop/set-up-a-computer-running-vnc-software-apdbed09830/mac)
-    [Adair, S. (2015, October 7). Virtual Private Keylogging: Cisco Web VPNs Leveraged for Access and Persistence. Retrieved March 20, 2017.](https://www.volexity.com/blog/2015/10/07/virtual-private-keylogging-cisco-web-vpns-leveraged-for-access-and-persistence/)
-    [Remillano II, A., et al. (2020, June 20). XORDDoS, Kaiji Variants Target Exposed Docker Servers. Retrieved April 5, 2021.](https://www.trendmicro.com/en_us/research/20/f/xorddos-kaiji-botnet-malware-variants-target-exposed-docker-servers.html)
-    [Chen, J. et al. (2021, February 3). Hildegard: New TeamTNT Cryptojacking Malware Targeting Kubernetes. Retrieved April 5, 2021.](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)
-    [Microsoft Threat Intelligence. (2025, February 12). The BadPilot campaign: Seashell Blizzard subgroup conducts multiyear global access operation. Retrieved June 18, 2025.](https://www.microsoft.com/en-us/security/blog/2025/02/12/the-badpilot-campaign-seashell-blizzard-subgroup-conducts-multiyear-global-access-operation/?ref=thestack.technology)
-    [Microsoft Threat Intelligence. (2023, December 7). Russian threat actors dig in, prepare to seize on war fatigue. Retrieved June 18, 2025.](https://www.microsoft.com/en-us/security/security-insider/intelligence-reports/russian-threat-actors-dig-in-prepare-to-seize-on-war-fatigue)
-    [Booz Allen Hamilton. (2016). When The Lights Went Out. Retrieved December 18, 2024.](https://www.boozallen.com/content/dam/boozallen/documents/2016/09/ukraine-report-when-the-lights-went-out.pdf)
-    [CERT Polska. (2026, January 30). Energy Sector Incident Report – 29 December. Retrieved April 22, 2026.](https://cert.pl/uploads/docs/CERT_Polska_Energy_Sector_Incident_Report_2025.pdf)
-    [https://5943619.hs-sites.com/hubfs/Reports/dragos-2025-poland-attack-report.pdf. (2026, January). ELECTRUM: CYBER ATTACK ON POLAND’S ELECTRIC SYSTEM 2025. Retrieved April 22, 2026.](https://5943619.hs-sites.com/hubfs/Reports/dragos-2025-poland-attack-report.pdf)
-    [Secureworks. (n.d.). GOLD SAHARA. Retrieved February 20, 2024.](https://www.secureworks.com/research/threat-profiles/gold-sahara)
-    [Insikt Group. (2025, August 26). TAG-144’s Persistent Grip on South American Organizations. Retrieved April 16, 2026.](https://assets.recordedfuture.com/insikt-report-pdfs/2025/cta-2025-0826.pdf)
-    [Adair, S. (2017, February 17). Detecting and Responding to Advanced Threats within Exchange Environments. Retrieved November 17, 2024.](https://web.archive.org/web/20210803040540/https://published-prd.lanyonevents.com/published/rsaus17/sessionsFiles/5009/HTA-F02-Detecting-and-Responding-to-Advanced-Threats-within-Exchange-Environments.pdf)
-    [NSA, CISA, FBI, NCSC. (2021, July). Russian GRU Conducting Global Brute Force Campaign to Compromise Enterprise and Cloud Environments. Retrieved July 26, 2021.](https://media.defense.gov/2021/Jul/01/2002753896/-1/-1/1/CSA_GRU_GLOBAL_BRUTE_FORCE_CAMPAIGN_UOO158036-21.PDF)
-    [National Cyber Security Centre. (2020, July 16). Advisory: APT29 targets COVID-19 vaccine development. Retrieved September 29, 2020.](https://www.ncsc.gov.uk/files/Advisory-APT29-targets-COVID-19-vaccine-development-V1-1.pdf)
-    [Douglas Bienstock. (2022, August 18). You Can’t Audit Me: APT29 Continues Targeting Microsoft 365. Retrieved February 23, 2023.](https://www.mandiant.com/resources/blog/apt29-continues-targeting-microsoft)
-    [Fraser, N., et al. (2019, August 7). Double DragonAPT41, a dual espionage and cyber crime operation APT41. Retrieved September 23, 2019.](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)
-    [Canadian Centre for Cyber Security. (2024, April 24). Cyber Activity Impacting CISCO ASA VPNs. Retrieved January 6, 2025.](https://www.cyber.gc.ca/en/news-events/cyber-activity-impacting-cisco-asa-vpns)
-    [Parisi, T. (2022, December 2). Not a SIMulation: CrowdStrike Investigations Reveal Intrusion Campaign Targeting Telco and BPO Companies. Retrieved June 30, 2023.](https://www.crowdstrike.com/blog/analysis-of-intrusion-campaign-targeting-telecom-and-bpo-companies/)
-    [Miller, S, et al. (2019, April 10). TRITON Actor TTP Profile, Custom Attack Tools, Detections, and ATT&CK Mapping. Retrieved April 16, 2019.](https://www.fireeye.com/blog/threat-research/2019/04/triton-actor-ttp-profile-custom-attack-tools-detections.html)
-    [Cycraft. (2020, April 15). APT Group Chimera - APT Operation Skeleton key Targets Taiwan Semiconductor Vendors. Retrieved August 24, 2020..](https://web.archive.org/web/20231214094657/https://cycraft.com/download/CyCraft-Whitepaper-Chimera_V4.1.pdf)
-    [Jansen, W . (2021, January 12). Abusing cloud services to fly under the radar. Retrieved September 12, 2024.](https://web.archive.org/web/20230218064220/https://research.nccgroup.com/2021/01/12/abusing-cloud-services-to-fly-under-the-radar/)
-    [The BlackBerry Research and Intelligence Team. (2020, November 12). The CostaRicto Campaign: Cyber-Espionage Outsourced. Retrieved May 24, 2021.](https://blogs.blackberry.com/en/2020/11/the-costaricto-campaign-cyber-espionage-outsourced)
-    [Fishbein, N., Kajiloti, M.. (2020, July 28). Watch Your Containers: Doki Infecting Docker Servers in the Cloud. Retrieved March 30, 2021.](https://www.intezer.com/blog/cloud-security/watch-your-containers-doki-infecting-docker-servers-in-the-cloud/)
-    [US-CERT. (2018, March 16). Alert (TA18-074A): Russian Government Cyber Activity Targeting Energy and Other Critical Infrastructure Sectors. Retrieved June 6, 2018.](https://www.us-cert.gov/ncas/alerts/TA18-074A)
-    [CISA. (2020, December 1). Russian State-Sponsored Advanced Persistent Threat Actor Compromises U.S. Government Targets. Retrieved December 9, 2021.](https://www.cisa.gov/uscert/ncas/alerts/aa20-296a#revisions)
-    [US Cybersecurity & Infrastructure Security Agency et al. (2024, September 5). Russian Military Cyber Actors Target U.S. and Global Critical Infrastructure. Retrieved September 6, 2024.](https://www.cisa.gov/sites/default/files/2024-09/aa24-249a-russian-military-cyber-actors-target-us-and-global-critical-infrastructure.pdf)
-    [Ta, V., et al. (2022, August 8). FIN13: A Cybercriminal Threat Actor Focused on Mexico. Retrieved February 9, 2023.](https://www.mandiant.com/resources/blog/fin13-cybercriminal-mexico)
-    [Scavella, T. and Rifki, A. (2017, July 20). Are you Ready to Respond? (Webinar). Retrieved October 4, 2017.](https://web.archive.org/web/20190304204552/https://www2.fireeye.com/WBNR-Are-you-ready-to-respond.html)
-    [Higgins, K. (2015, October 13). Prolific Cybercrime Gang Favors Legit Login Credentials. Retrieved October 4, 2017.](https://www.darkreading.com/analytics/prolific-cybercrime-gang-favors-legit-login-credentials/d/d-id/1322645?)
-    [Bromiley, M. and Lewis, P. (2016, October 7). Attacking the Hospitality and Gaming Industries: Tracking an Attacker Around the World in 7 Years. Retrieved October 6, 2017.](https://www.youtube.com/watch?v=fevGZs0EQu8)
-    [Cybereason Nocturnus. (2019, June 25). Operation Soft Cell: A Worldwide Campaign Against Telecommunications Providers. Retrieved July 18, 2019.](https://www.cybereason.com/blog/operation-soft-cell-a-worldwide-campaign-against-telecommunications-providers)
-    [MSTIC. (2019, December 12). GALLIUM: Targeting global telecom. Retrieved January 13, 2021.](https://www.microsoft.com/security/blog/2019/12/12/gallium-targeting-global-telecom/)
-    [Counter Threat Unit Research Team. (2019, September 24). REvil/Sodinokibi Ransomware. Retrieved August 4, 2020.](https://www.secureworks.com/research/revil-sodinokibi-ransomware)

-    [Smallridge, R. (2018, March 10). APT15 is alive and strong: An analysis of RoyalCli and RoyalDNS. Retrieved April 4, 2018.](https://research.nccgroup.com/2018/03/10/apt15-is-alive-and-strong-an-analysis-of-royalcli-and-royaldns/)
-    [MSTIC. (2021, December 6). NICKEL targeting government organizations across Latin America and Europe. Retrieved March 18, 2022.](https://www.microsoft.com/security/blog/2021/12/06/nickel-targeting-government-organizations-across-latin-america-and-europe)
-    [CISA, FBI, CNMF. (2020, October 27). https://us-cert.cisa.gov/ncas/alerts/aa20-301a. Retrieved November 4, 2020.](https://us-cert.cisa.gov/ncas/alerts/aa20-301a)
-    [Singer, G. (2020, April 3). Threat Alert: Kinsing Malware Attacks Targeting Container Environments. Retrieved April 1, 2021.](https://blog.aquasec.com/threat-alert-kinsing-malware-container-vulnerability)
-    [MSTIC, DART, M365 Defender. (2022, March 24). DEV-0537 Criminal Actor Targeting Organizations for Data Exfiltration and Destruction. Retrieved May 17, 2022.](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)
-    [Brown, D., et al. (2022, April 28). LAPSUS$: Recent techniques, tactics and procedures. Retrieved December 22, 2022.](https://www.nccgroup.com/research-blog/lapsus-recent-techniques-tactics-and-procedures/)
-    [CISA. (2021, July 19). (AA21-200A) Joint Cybersecurity Advisory – Tactics, Techniques, and Procedures of Indicted APT40 Actors Associated with China’s MSS Hainan State Security Department. Retrieved August 12, 2021.](https://us-cert.cisa.gov/ncas/alerts/aa21-200a)
-    [Anomali Labs. (2018, December 6). Pulling Linux Rabbit/Rabbot Malware Out of a Hat. Retrieved March 4, 2019.](https://www.anomali.com/blog/pulling-linux-rabbit-rabbot-malware-out-of-a-hat)
-    [SentinelLabs. (2022, September 22). Metador Technical Appendix. Retrieved April 4, 2023.](https://docs.google.com/document/d/1e9ZTW9b71YwFWS_18ZwDAxa-cYbV8q1wUefmKZLYVsA/edit#heading=h.lmnbtht1ikzm)
-    [McAfee® Foundstone® Professional Services and McAfee Labs™. (2011, February 10). Global Energy Cyberattacks: “Night Dragon”. Retrieved February 19, 2018.](https://scadahacker.com/library/Documents/Cyber_Events/McAfee%20-%20Night%20Dragon%20-%20Global%20Energy%20Cyberattacks.pdf)
-    [Davis, S. and Caban, D. (2017, December 19). APT34 - New Targeted Attack in the Middle East. Retrieved December 20, 2017.](https://www.brighttalk.com/webcast/10703/296317/apt34-new-targeted-attack-in-the-middle-east)
-    [Cybereason Nocturnus. (2022, May 4). Operation CuckooBees: Deep-Dive into Stealthy Winnti Techniques. Retrieved September 22, 2022.](https://www.cybereason.com/blog/operation-cuckoobees-deep-dive-into-stealthy-winnti-techniques)
-    [Dantzig, M. v., Schamper, E. (2019, December 19). Operation Wocao: Shining a light on one of China’s hidden hacking groups. Retrieved October 8, 2020.](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)
-    [CISA. (2023, December 18). #StopRansomware: Play Ransomware AA23-352A. Retrieved September 24, 2024.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-352a)
-    [Trend Micro Research. (2023, July 21). Ransomware Spotlight: Play. Retrieved September 24, 2024.](https://www.trendmicro.com/vinfo/us/security/news/ransomware-spotlight/ransomware-spotlight-play)
-    [Cherepanov, A.. (2016, January 3). BlackEnergy by the SSHBearDoor: attacks against Ukrainian news media and electric industry . Retrieved June 10, 2020.](https://www.welivesecurity.com/2016/01/03/blackenergy-sshbeardoor-details-2015-attacks-ukrainian-news-media-electric-industry/)
-    [Cherepanov, A.. (2017, June 30). TeleBots are back: Supply chain attacks against Ukraine. Retrieved June 11, 2020.](https://www.welivesecurity.com/2017/06/30/telebots-back-supply-chain-attacks-against-ukraine/)
-    [ANSSI. (2021, January 27). SANDWORM INTRUSION SET CAMPAIGN TARGETING CENTREON SYSTEMS. Retrieved March 30, 2021.](https://www.cert.ssi.gouv.fr/uploads/CERTFR-2021-CTI-005.pdf)
-    [Roncone, G. et al. (n.d.). APT44: Unearthing Sandworm. Retrieved July 11, 2024.](https://services.google.com/fh/files/misc/apt44-unearthing-sandworm.pdf)
-    [CrowdStrike. (2023, January 10). SCATTERED SPIDER Exploits Windows Security Deficiencies with Bring-Your-Own-Vulnerable-Driver Tactic in Attempt to Bypass Endpoint Security. Retrieved July 5, 2023.](https://www.crowdstrike.com/blog/scattered-spider-attempts-to-avoid-detection-with-bring-your-own-vulnerable-driver-tactic/)
-    [Hunt & Hackett Research Team. (2024, January 5). Turkish espionage campaigns in the Netherlands. Retrieved November 20, 2024.](https://www.huntandhackett.com/blog/turkish-espionage-campaigns)
-    [Nafisi, R., Lelli, A. (2021, March 4). GoldMax, GoldFinder, and Sibot: Analyzing NOBELIUM’s layered persistence. Retrieved March 8, 2021.](https://www.microsoft.com/security/blog/2021/03/04/goldmax-goldfinder-sibot-analyzing-nobelium-malware/)
-    [CrowdStrike. (2022, January 27). Early Bird Catches the Wormhole: Observations from the StellarParticle Campaign. Retrieved February 7, 2022.](https://www.crowdstrike.com/blog/observations-from-the-stellarparticle-campaign/)
-    [Fishbein, N. (2020, September 8). Attackers Abusing Legitimate Cloud Monitoring Tools to Conduct Cyber Attacks. Retrieved September 22, 2021.](https://www.intezer.com/blog/cloud-security/attackers-abusing-legitimate-cloud-monitoring-tools-to-conduct-cyber-attacks/)
-    [Darin Smith. (2022, April 21). TeamTNT targeting AWS, Alibaba. Retrieved August 4, 2022.](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/)
-    [Dell SecureWorks Counter Threat Unit Threat Intelligence. (2015, August 5). Threat Group-3390 Targets Organizations for Cyberespionage. Retrieved August 18, 2018.](https://www.secureworks.com/research/threat-group-3390-targets-organizations-for-cyberespionage)
-    [Counter Threat Unit Research Team. (2017, June 27). BRONZE UNION Cyberespionage Persists Despite Disclosures. Retrieved July 13, 2017.](https://www.secureworks.com/research/bronze-union)
-    [Sygnia Team. (2024, June 3). China-Nexus Threat Group ‘Velvet Ant’ Abuses F5 Load Balancers for Persistence. Retrieved March 14, 2025.](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/)
-    [Check Point Research. (2026, March 12). “Handala Hack” – Unveiling Group’s Modus Operandi. Retrieved April 20, 2026.](https://research.checkpoint.com/2026/handala-hack-unveiling-groups-modus-operandi/)
-    [CISA et al.. (2024, February 7). PRC State-Sponsored Actors Compromise and Maintain Persistent Access to U.S. Critical Infrastructure. Retrieved May 15, 2024.](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)
-    [Kimberly Goody, Jeremy Kennelly, Joshua Shilko, Steve Elovitz, Douglas Bienstock. (2020, October 28). Unhappy Hour Special: KEGTAP and SINGLEMALT With a Ransomware Chaser. Retrieved October 28, 2020.](https://www.fireeye.com/blog/threat-research/2020/10/kegtap-and-singlemalt-with-a-ransomware-chaser.html)
-    [CISA, FBI. (2020, July 1). Defending Against Malicious Cyber Activity Originating from Tor . Retrieved June 20, 2025.](https://www.cisa.gov/sites/default/files/publications/AA20-183A_Defending_Against_Malicious_Cyber_Activity_Originating_from_Tor_S508C.pdf)

×
