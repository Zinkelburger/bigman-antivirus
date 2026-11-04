---
url: "https://attack.mitre.org/techniques/T1686/"
title: "Disable or Modify System Firewall, Technique T1686 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T19:04:53Z"
via: "live"
sha256: "221d382175a0b34729160941a8d33a9a43738993c73e692b6c6116dac8375178"
wayback: "https://web.archive.org/web/20261014223342/https://attack.mitre.org/techniques/T1686/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Disable or Modify System Firewall, Technique T1686 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1686/>
> Snapshot: <https://web.archive.org/web/20261014223342/https://attack.mitre.org/techniques/T1686/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Disable or Modify System Firewall

#  Disable or Modify System Firewall

##### Sub-techniques (3)

| ID  | Name

|  [T1686.001](https://attack.mitre.org/techniques/T1686/001/)   |  [Cloud Firewall](https://attack.mitre.org/techniques/T1686/001/)

|  [T1686.002](https://attack.mitre.org/techniques/T1686/002/)   |  [Network Device Firewall](https://attack.mitre.org/techniques/T1686/002/)

|  [T1686.003](https://attack.mitre.org/techniques/T1686/003/)   |  [Windows Host Firewall](https://attack.mitre.org/techniques/T1686/003/)

Adversaries may disable or modify host-based or network firewalls to impair defensive mechanisms and enable further action. Once an adversary has gathered sufficient privileges, they can tamper with firewall services, policies, or rule sets to remove restrictions on inbound or outbound traffic. For example, this may include turning off firewall profiles, altering existing rules to permit previously blocked ports or protocols, or adding new rules that create covert communication paths (e.g., adding a new firewall rule for a well-known protocol (such as RDP) using a non-traditional and potentially less securitized port.[[1]](https://x.com/TheDFIRReport/status/1498657772254240768)

Adversaries may disable or modify firewalls using different behaviors, depending on the platform. For example, in ESXi, firewall rules may be modified directly via the esxcli (e.g., via esxcli network firewall set) or via the vCenter user interface.[[2]](https://techdocs.broadcom.com/us/en/vmware-cis/vsphere/vsphere/7-0/add-allowed-ip-addresses-for-an-esxi-host-by-using-the-vmware-host-client.html)[[3]](https://www.trellix.com/en-au/blogs/research/ransomhouse-am-see/)

ID: T1686

Sub-techniques:  [T1686.001](https://attack.mitre.org/techniques/T1686/001), [T1686.002](https://attack.mitre.org/techniques/T1686/002), [T1686.003](https://attack.mitre.org/techniques/T1686/003)

ⓘ

Tactic: [Defense Impairment](https://attack.mitre.org/tactics/TA0112)

ⓘ

Platforms: ESXi, Linux, Network Devices, Windows, macOS

Version: 1.0

Created: 14 April 2026

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1686/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1686/)

## Procedure Examples

| ID  | Name  | Description

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) have created firewall exemptions on specific ports, including ports 443, 6443, 8443, and 9443.[[4]](https://us-cert.cisa.gov/ncas/alerts/aa20-239a)

|  [S0031](https://attack.mitre.org/software/S0031)   |  [BACKSPACE](https://attack.mitre.org/software/S0031)   |

The "ZR" variant of [BACKSPACE](https://attack.mitre.org/software/S0031) will check to see if known host-based firewalls are installed on the infected systems. [BACKSPACE](https://attack.mitre.org/software/S0031) will attempt to establish a C2 channel, then will examine open windows to identify a pop-up from the firewall software and will simulate a mouse-click to allow the connection to proceed.[[5]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2015/05/20081935/rpt-apt30.pdf)

|  [G1043](https://attack.mitre.org/groups/G1043)   |  [BlackByte](https://attack.mitre.org/groups/G1043)   |

[BlackByte](https://attack.mitre.org/groups/G1043) modified firewall rules on victim machines to enable remote system discovery.[[6]](https://www.picussecurity.com/resource/ttps-used-by-blackbyte-ransomware-targeting-critical-infrastructure)[[7]](https://www.security.com/threat-intelligence/blackbyte-exbyte-ransomware)

|  [S1161](https://attack.mitre.org/software/S1161)   |  [BPFDoor](https://attack.mitre.org/software/S1161)   |

[BPFDoor](https://attack.mitre.org/software/S1161) starts a shell on a high TCP port starting at 42391 up to 43391, then changes the local iptables rules to redirect all packets from the attacker to the shell port.[[8]](https://sandflysecurity.com/blog/bpfdoor-an-evasive-linux-backdoor-technical-analysis/)

|  [G0008](https://attack.mitre.org/groups/G0008)   |  [Carbanak](https://attack.mitre.org/groups/G0008)   |

[Carbanak](https://attack.mitre.org/groups/G0008) may use [netsh](https://attack.mitre.org/software/S0108) to add local firewall rule exceptions.[[9]](http://www.group-ib.com/files/Anunak_APT_against_financial_institutions.pdf)

|  [S0492](https://attack.mitre.org/software/S0492)   |  [CookieMiner](https://attack.mitre.org/software/S0492)   |

[CookieMiner](https://attack.mitre.org/software/S0492) has checked for the presence of "Little Snitch", macOS network monitoring and application firewall software, stopping and exiting if it is found.[[10]](https://unit42.paloaltonetworks.com/mac-malware-steals-cryptocurrency-exchanges-cookies/)

|  [G0035](https://attack.mitre.org/groups/G0035)   |  [Dragonfly](https://attack.mitre.org/groups/G0035)   |

[Dragonfly](https://attack.mitre.org/groups/G0035) has disabled host-based firewalls. The group has also globally opened port 3389.[[11]](https://www.us-cert.gov/ncas/alerts/TA18-074A)

|  [G0046](https://attack.mitre.org/groups/G0046)   |  [FIN7](https://attack.mitre.org/groups/G0046)   |

[FIN7](https://attack.mitre.org/groups/G0046) has added a firewall rule to allow TCP port 59999 inbound and a rule to allow sshd.exe on TCP port 9898.[[12]](https://blogs.blackberry.com/en/2024/04/fin7-targets-the-united-states-automotive-industry)

|  [S0531](https://attack.mitre.org/software/S0531)   |  [Grandoreiro](https://attack.mitre.org/software/S0531)   |

[Grandoreiro](https://attack.mitre.org/software/S0531) can block the Deibold Warsaw GAS Tecnologia security tool at the firewall level.[[13]](https://www.welivesecurity.com/2020/04/28/grandoreiro-how-engorged-can-exe-get/)

|  [S1211](https://attack.mitre.org/software/S1211)   |  [Hannotog](https://attack.mitre.org/software/S1211)   |

[Hannotog](https://attack.mitre.org/software/S1211) can modify local firewall settings via netsh commands to open a listening UDP port.[[14]](https://www.security.com/threat-intelligence/espionage-asia-governments-cert-authority)

|  [S0376](https://attack.mitre.org/software/S0376)   |  [HOPLIGHT](https://attack.mitre.org/software/S0376)   |

[HOPLIGHT](https://attack.mitre.org/software/S0376) has modified the firewall using [netsh](https://attack.mitre.org/software/S0108).[[15]](https://www.us-cert.gov/ncas/analysis-reports/AR19-100A)

|  [S0260](https://attack.mitre.org/software/S0260)   |  [InvisiMole](https://attack.mitre.org/software/S0260)   |

[InvisiMole](https://attack.mitre.org/software/S0260) has a command to disable routing and the Firewall on the victim’s machine.[[16]](https://www.welivesecurity.com/2018/06/07/invisimole-equipped-spyware-undercover/)

|  [S0088](https://attack.mitre.org/software/S0088)   |  [Kasidet](https://attack.mitre.org/software/S0088)   |

[Kasidet](https://attack.mitre.org/software/S0088) has the ability to change firewall settings to allow a plug-in to be downloaded.[[17]](http://research.zscaler.com/2016/01/malicious-office-files-dropping-kasidet.html)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has been observed disabling the system firewall.[[18]](https://securelist.com/the-kimsuky-operation-a-north-korean-apt/57915/)

|  [C0049](https://attack.mitre.org/campaigns/C0049)   |  [Leviathan Australian Intrusions](https://attack.mitre.org/campaigns/C0049)   |

[Leviathan](https://attack.mitre.org/groups/G0065) modified system firewalls to add two open listening ports on 9998 and 9999 during [Leviathan Australian Intrusions](https://attack.mitre.org/campaigns/C0049).[[19]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa24-190a)

|  [G1051](https://attack.mitre.org/groups/G1051)   |  [Medusa Group](https://attack.mitre.org/groups/G1051)   |

[Medusa Group](https://attack.mitre.org/groups/G1051) has utilized [PsExec](https://attack.mitre.org/software/S0029) to execute batch scripts that modify firewall settings.[[20]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-071a) [Medusa Group](https://attack.mitre.org/groups/G1051) has also enabled and modified firewall rules to allow for RDP connections for lateral movement and device interactions.[[20]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-071a)

|  [S0336](https://attack.mitre.org/software/S0336)   |  [NanoCore](https://attack.mitre.org/software/S0336)   |

[NanoCore](https://attack.mitre.org/software/S0336) can modify the victim's firewall.[[21]](https://www.digitrustgroup.com/nanocore-not-your-average-rat/)[[22]](https://researchcenter.paloaltonetworks.com/2016/02/nanocorerat-behind-an-increase-in-tax-themed-phishing-e-mails/)

|  [S0108](https://attack.mitre.org/software/S0108)   |  [netsh](https://attack.mitre.org/software/S0108)   |

[netsh](https://attack.mitre.org/software/S0108) can be used to disable local firewall settings.[[23]](https://technet.microsoft.com/library/bb490939.aspx)[[24]](https://technet.microsoft.com/en-us/library/cc771046(v=ws.10).aspx)

|  [S0013](https://attack.mitre.org/software/S0013)   |  [PlugX](https://attack.mitre.org/software/S0013)   |

[PlugX](https://attack.mitre.org/software/S0013) has modified local firewall rules on victim machines to enable a random, high-number listening port for subsequent access and C2 activity.[[25]](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/)

|  [S1032](https://attack.mitre.org/software/S1032)   |  [PyDCrypt](https://attack.mitre.org/software/S1032)   |

[PyDCrypt](https://attack.mitre.org/software/S1032) has modified firewall rules to allow incoming SMB, NetBIOS, and RPC connections using netsh.exe on remote machines.[[26]](https://research.checkpoint.com/2021/mosesstaff-targeting-israeli-companies/)

|  [G0106](https://attack.mitre.org/groups/G0106)   |  [Rocke](https://attack.mitre.org/groups/G0106)   |

[Rocke](https://attack.mitre.org/groups/G0106) used scripts which killed processes and added firewall rules to block traffic related to other cryptominers.[[27]](https://blog.talosintelligence.com/2018/08/rocke-champion-of-monero-miners.html)

|  [G1045](https://attack.mitre.org/groups/G1045)   |  [Salt Typhoon](https://attack.mitre.org/groups/G1045)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has made changes to the Access Control List (ACL) and loopback interface address on compromised devices.[[28]](https://blog.talosintelligence.com/salt-typhoon-analysis/)

|  [S1178](https://attack.mitre.org/software/S1178)   |  [ShrinkLocker](https://attack.mitre.org/software/S1178)   |

[ShrinkLocker](https://attack.mitre.org/software/S1178) turns on the system firewall and deletes all of its rules during execution.[[29]](https://securelist.com/ransomware-abuses-bitlocker/112643/)[[30]](https://www.splunk.com/en_us/blog/security/shrinklocker-malware-abusing-bitlocker-to-lock-your-data.html)

|  [C0024](https://attack.mitre.org/campaigns/C0024)   |  [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024)   |

During the [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024), [APT29](https://attack.mitre.org/groups/G0016) used netsh to configure firewall rules that limited certain UDP outbound packets.[[31]](https://www.microsoft.com/security/blog/2021/01/20/deep-dive-into-the-solorigate-second-stage-activation-from-sunburst-to-teardrop-and-raindrop/)

|  [G0139](https://attack.mitre.org/groups/G0139)   |  [TeamTNT](https://attack.mitre.org/groups/G0139)   |

[TeamTNT](https://attack.mitre.org/groups/G0139) has disabled iptables.[[32]](https://blog.aquasec.com/container-security-tnt-container-attack)

|  [S1223](https://attack.mitre.org/software/S1223)   |  [THINCRUST](https://attack.mitre.org/software/S1223)   |

[THINCRUST](https://attack.mitre.org/software/S1223) can use the Django python module "django.views.decorators.csrf" along with the decorator "csrf_exempt" within victim firewalls to disable cross-site request forgery protections.[[33]](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)

|  [G1022](https://attack.mitre.org/groups/G1022)   |  [ToddyCat](https://attack.mitre.org/groups/G1022)   |

Prior to executing a backdoor [ToddyCat](https://attack.mitre.org/groups/G1022) has run cmd /c start /b netsh advfirewall firewall add rule name="SGAccessInboundRule" dir=in protocol=udp action=allow localport=49683 to allow the targeted system to receive UDP packets on port 49683.[[34]](https://securelist.com/toddycat-keep-calm-and-check-logs/110696/)

|  [G1048](https://attack.mitre.org/groups/G1048)   |  [UNC3886](https://attack.mitre.org/groups/G1048)   |

[UNC3886](https://attack.mitre.org/groups/G1048) has used the TABLEFLIP traffic redirection utility and the esxcli command line to modify firewall rules.[[35]](https://cloud.google.com/blog/topics/threat-intelligence/esxi-hypervisors-malware-persistence)[[36]](https://cloud.google.com/blog/topics/threat-intelligence/vmware-esxi-zero-day-bypass/)[[33]](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)

|  [G1047](https://attack.mitre.org/groups/G1047)   |  [Velvet Ant](https://attack.mitre.org/groups/G1047)   |

[Velvet Ant](https://attack.mitre.org/groups/G1047) modified system firewall settings during [PlugX](https://attack.mitre.org/software/S0013) installation using netsh.exe to open a listening, random high number port on victim devices.[[25]](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/)

|  [S0412](https://attack.mitre.org/software/S0412)   |  [ZxShell](https://attack.mitre.org/software/S0412)   |

[ZxShell](https://attack.mitre.org/software/S0412) can disable the firewall by modifying the registry key HKLM\SYSTEM\CurrentControlSet\Services\SharedAccess\Parameters\FirewallPolicy\StandardProfile.[[37]](https://blogs.cisco.com/security/talos/opening-zxshell)

## Mitigations

| ID  | Mitigation  | Description

|  [M1047](https://attack.mitre.org/mitigations/M1047)   |  [Audit](https://attack.mitre.org/mitigations/M1047)   |

Routinely check account role permissions to ensure only expected users and roles have permission to modify system firewalls.

|  [M1022](https://attack.mitre.org/mitigations/M1022)   |  [Restrict File and Directory Permissions](https://attack.mitre.org/mitigations/M1022)   |

Ensure proper process and file permissions are in place to prevent adversaries from disabling or modifying firewall settings.

|  [M1024](https://attack.mitre.org/mitigations/M1024)   |  [Restrict Registry Permissions](https://attack.mitre.org/mitigations/M1024)   |

Ensure proper Registry permissions are in place to prevent adversaries from disabling or modifying firewall settings.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Ensure proper user permissions are in place to prevent adversaries from disabling or modifying firewall settings.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0145](https://attack.mitre.org/detectionstrategies/DET0145)   |  [Detection of Disabled or Modified System Firewalls across OS Platforms.](https://attack.mitre.org/detectionstrategies/DET0145)   |  [AN0406](https://attack.mitre.org/detectionstrategies/DET0145#AN0406)   |

Detection of firewall tampering by monitoring processes executing netsh, PowerShell Set-NetFirewallProfile, or sc stop mpssvc. Registry modifications under HKLM\SYSTEM\CurrentControlSet\Services\SharedAccess\Parameters\FirewallPolicy also indicate adversarial actions.

|  [AN0407](https://attack.mitre.org/detectionstrategies/DET0145#AN0407)   |

Detection of iptables, nftables, or firewalld rule modifications. Correlation of sudden drops in active firewall rules with suspicious processes suggests adversarial evasion.

|  [AN0408](https://attack.mitre.org/detectionstrategies/DET0145#AN0408)   |

Detection of PF firewall rule modifications via pfctl, socketfilterfw, or defaults write to com.apple.alf. Adversaries often disable firewall profiles entirely or whitelist malicious processes.

|  [AN0409](https://attack.mitre.org/detectionstrategies/DET0145#AN0409)   |

Detection of firewall changes using esxcli network firewall set or vSphere API modifications. Sudden disabling of firewall rules across management interfaces is a strong adversarial signal.

|  [AN0410](https://attack.mitre.org/detectionstrategies/DET0145#AN0410)   |

Detection of firewall ACL or rule base changes through CLI (e.g., no access-list, permit any any). Monitor configuration commits from unusual users or sessions.

## References

-    [The DFIR Report. (2022, March 1). "Change RDP port" #ContiLeaks. Retrieved September 12, 2024.](https://x.com/TheDFIRReport/status/1498657772254240768)
-    [Broadcom. (2025, March 24). Add Allowed IP Addresses for an ESXi Host by Using the VMware Host Client. Retrieved March 26, 2025.](https://techdocs.broadcom.com/us/en/vmware-cis/vsphere/vsphere/7-0/add-allowed-ip-addresses-for-an-esxi-host-by-using-the-vmware-host-client.html)
-    [Pham Duy Phuc, Max Kersten, Noël Keijzer, and Michaël Schrijver. (2024, February 14). RansomHouse am See. Retrieved March 26, 2025.](https://www.trellix.com/en-au/blogs/research/ransomhouse-am-see/)
-    [DHS/CISA. (2020, August 26). FASTCash 2.0: North Korea's BeagleBoyz Robbing Banks. Retrieved September 29, 2021.](https://us-cert.cisa.gov/ncas/alerts/aa20-239a)
-    [FireEye Labs. (2015, April). APT30 AND THE MECHANICS OF A LONG-RUNNING CYBER ESPIONAGE OPERATION. Retrieved November 17, 2024.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2015/05/20081935/rpt-apt30.pdf)
-    [Huseyin Can Yuceel. (2022, February 21). TTPs used by BlackByte Ransomware Targeting Critical Infrastructure. Retrieved December 16, 2024.](https://www.picussecurity.com/resource/ttps-used-by-blackbyte-ransomware-targeting-critical-infrastructure)
-    [Symantec Threat Hunter Team. (2022, October 21). Exbyte: BlackByte Ransomware Attackers Deploy New Exfiltration Tool. Retrieved December 16, 2024.](https://www.security.com/threat-intelligence/blackbyte-exbyte-ransomware)
-    [The Sandfly Security Team. (2022, May 11). BPFDoor - An Evasive Linux Backdoor Technical Analysis. Retrieved September 29, 2023.](https://sandflysecurity.com/blog/bpfdoor-an-evasive-linux-backdoor-technical-analysis/)
-    [Group-IB and Fox-IT. (2014, December). Anunak: APT against financial institutions. Retrieved April 20, 2016.](http://www.group-ib.com/files/Anunak_APT_against_financial_institutions.pdf)
-    [Chen, y., et al. (2019, January 31). Mac Malware Steals Cryptocurrency Exchanges’ Cookies. Retrieved July 22, 2020.](https://unit42.paloaltonetworks.com/mac-malware-steals-cryptocurrency-exchanges-cookies/)
-    [US-CERT. (2018, March 16). Alert (TA18-074A): Russian Government Cyber Activity Targeting Energy and Other Critical Infrastructure Sectors. Retrieved June 6, 2018.](https://www.us-cert.gov/ncas/alerts/TA18-074A)
-    [The BlackBerry Research and Intelligence Team. (2024, April 17). Threat Group FIN7 Targets the U.S. Automotive Industry. Retrieved May 1, 2025.](https://blogs.blackberry.com/en/2024/04/fin7-targets-the-united-states-automotive-industry)
-    [ESET. (2020, April 28). Grandoreiro: How engorged can an EXE get?. Retrieved November 13, 2020.](https://www.welivesecurity.com/2020/04/28/grandoreiro-how-engorged-can-exe-get/)
-    [Symntec Threat Hunter Team. (2022, November 12). Billbug: State-sponsored Actor Targets Cert Authority, Government Agencies in Multiple Asian Countries. Retrieved March 15, 2025.](https://www.security.com/threat-intelligence/espionage-asia-governments-cert-authority)
-    [US-CERT. (2019, April 10). MAR-10135536-8 – North Korean Trojan: HOPLIGHT. Retrieved April 19, 2019.](https://www.us-cert.gov/ncas/analysis-reports/AR19-100A)
-    [Hromcová, Z. (2018, June 07). InvisiMole: Surprisingly equipped spyware, undercover since 2013. Retrieved July 10, 2018.](https://www.welivesecurity.com/2018/06/07/invisimole-equipped-spyware-undercover/)
-    [Yadav, A., et al. (2016, January 29). Malicious Office files dropping Kasidet and Dridex. Retrieved March 24, 2016.](http://research.zscaler.com/2016/01/malicious-office-files-dropping-kasidet.html)
-    [Tarakanov , D.. (2013, September 11). The “Kimsuky” Operation: A North Korean APT?. Retrieved August 13, 2019.](https://securelist.com/the-kimsuky-operation-a-north-korean-apt/57915/)
-    [CISA et al. (2024, July 8). People’s Republic of China (PRC) Ministry of State Security APT40 Tradecraft in Action. Retrieved February 3, 2025.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa24-190a)

-    [Cybersecurity and Infrastructure Security Agency. (2025, March 12). AA25-071A #StopRansomware: Medusa Ransomware. Retrieved October 15, 2025.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-071a)
-    [The DigiTrust Group. (2017, January 01). NanoCore Is Not Your Average RAT. Retrieved November 9, 2018.](https://www.digitrustgroup.com/nanocore-not-your-average-rat/)
-    [Kasza, A., Halfpop, T. (2016, February 09). NanoCoreRAT Behind an Increase in Tax-Themed Phishing E-mails. Retrieved November 9, 2018.](https://researchcenter.paloaltonetworks.com/2016/02/nanocorerat-behind-an-increase-in-tax-themed-phishing-e-mails/)
-    [Microsoft. (n.d.). Using Netsh. Retrieved February 13, 2017.](https://technet.microsoft.com/library/bb490939.aspx)
-    [Microsoft. (2009, June 3). Netsh Commands for Windows Firewall. Retrieved April 20, 2016.](https://technet.microsoft.com/en-us/library/cc771046(v=ws.10).aspx)
-    [Sygnia Team. (2024, June 3). China-Nexus Threat Group ‘Velvet Ant’ Abuses F5 Load Balancers for Persistence. Retrieved March 14, 2025.](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/)
-    [Checkpoint Research. (2021, November 15). Uncovering MosesStaff techniques: Ideology over Money. Retrieved August 11, 2022.](https://research.checkpoint.com/2021/mosesstaff-targeting-israeli-companies/)
-    [Liebenberg, D.. (2018, August 30). Rocke: The Champion of Monero Miners. Retrieved May 26, 2020.](https://blog.talosintelligence.com/2018/08/rocke-champion-of-monero-miners.html)
-    [Cisco Talos. (2025, February 20). Weathering the storm: In the midst of a Typhoon. Retrieved February 24, 2025.](https://blog.talosintelligence.com/salt-typhoon-analysis/)
-    [Cristian Souza, Eduardo Ovalle, Ashley Muñoz, & Christopher Zachor. (2024, May 23). ShrinkLocker: Turning BitLocker into ransomware. Retrieved December 7, 2024.](https://securelist.com/ransomware-abuses-bitlocker/112643/)
-    [Splunk Threat Research Team , Teoderick Contreras. (2024, September 5). ShrinkLocker Malware: Abusing BitLocker to Lock Your Data. Retrieved December 7, 2024.](https://www.splunk.com/en_us/blog/security/shrinklocker-malware-abusing-bitlocker-to-lock-your-data.html)
-    [MSTIC, CDOC, 365 Defender Research Team. (2021, January 20). Deep dive into the Solorigate second-stage activation: From SUNBURST to TEARDROP and Raindrop . Retrieved January 22, 2021.](https://www.microsoft.com/security/blog/2021/01/20/deep-dive-into-the-solorigate-second-stage-activation-from-sunburst-to-teardrop-and-raindrop/)
-    [Kol, Roi. Morag, A. (2020, August 25). Deep Analysis of TeamTNT Techniques Using Container Images to Attack. Retrieved September 22, 2021.](https://blog.aquasec.com/container-security-tnt-container-attack)
-    [Marvi, A. et al.. (2023, March 16). Fortinet Zero-Day and Custom Malware Used by Suspected Chinese Actor in Espionage Operation. Retrieved March 22, 2023.](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)
-    [Dedola, G. et al. (2023, October 12). ToddyCat: Keep calm and check logs. Retrieved January 3, 2024.](https://securelist.com/toddycat-keep-calm-and-check-logs/110696/)
-    [Alexander Marvi, Jeremy Koppen, Tufail Ahmed, and Jonathan Lepore. (2022, September 29). Bad VIB(E)s Part One: Investigating Novel Malware Persistence Within ESXi Hypervisors. Retrieved March 26, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/esxi-hypervisors-malware-persistence)
-    [Alexander Marvi, Brad Slaybaugh, Ron Craft, and Rufus Brown. (2023, June 13). VMware ESXi Zero-Day Used by Chinese Espionage Actor to Perform Privileged Guest Operations on Compromised Hypervisors. Retrieved March 26, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/vmware-esxi-zero-day-bypass/)
-    [Allievi, A., et al. (2014, October 28). Threat Spotlight: Group 72, Opening the ZxShell. Retrieved September 24, 2019.](https://blogs.cisco.com/security/talos/opening-zxshell)

×
