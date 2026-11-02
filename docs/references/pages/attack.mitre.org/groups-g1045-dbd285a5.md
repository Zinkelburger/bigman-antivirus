---
url: "https://attack.mitre.org/groups/G1045/"
title: "Salt Typhoon, Group G1045 | MITRE ATT&CK®"
captured: "2026-11-02T18:21:47Z"
via: "live"
sha256: "678dd64e1604d6c8bbb3b11d1d31516ec80796748dfb23898bde01339c4e685c"
wayback: "https://web.archive.org/web/20261019175025/https://attack.mitre.org/groups/G1045/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Salt Typhoon, Group G1045 | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/groups/G1045/>
> Snapshot: <https://web.archive.org/web/20261019175025/https://attack.mitre.org/groups/G1045/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Groups](https://attack.mitre.org/groups/)
- Salt Typhoon

#  Salt Typhoon

[Salt Typhoon](https://attack.mitre.org/groups/G1045) is a People's Republic of China (PRC) state-backed actor that has been active since at least 2019 and responsible for numerous compromises of network infrastructure at major U.S. telecommunication and internet service providers (ISP). [Salt Typhoon](https://attack.mitre.org/groups/G1045) has targeted and leveraged public facing network edge devices to gain access to victim environments. [Salt Typhoon](https://attack.mitre.org/groups/G1045) has been publicly associated with activities resembling other security industry reported activity groups to include OPERATOR PANDA, RedMike, GhostEmperor, and UNC5807.[[1]](https://home.treasury.gov/news/press-releases/jy2792)[[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/)[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)[[4]](https://www.sentinelone.com/labs/malicious-apprentice-how-two-hackers-went-from-cisco-academy-to-cisco-cves/)[[5]](https://www.crowdstrike.com/en-us/adversaries/operator-panda/)[[6]](https://www.recordedfuture.com/research/redmike-salt-typhoon-exploits-vulnerable-devices)[[7]](https://learn.microsoft.com/en-us/unified-secops/microsoft-threat-actor-naming)[[8]](https://cloud.google.com/blog/topics/threat-intelligence/m-trends-2026)

ID: G1045

Version: 2.0

Created: 24 February 2025

Last Modified: 23 September 2026

[Version Permalink](https://attack.mitre.org/versions/v20/groups/G1045/)

[Live Version](https://attack.mitre.org/versions/v20/groups/G1045/)

###### Enterprise Layer

[download](https://attack.mitre.org/groups/G1045/G1045-enterprise-layer.json)  [view](https://attack.mitre.org/groups/G1045/#)

## Techniques Used

| Domain  | ID  | Name  | Use

|  Enterprise   |  [T1548](https://attack.mitre.org/techniques/T1548)   |  [.003](https://attack.mitre.org/techniques/T1548/003)   |  [Abuse Elevation Control Mechanism](https://attack.mitre.org/techniques/T1548): [Sudo and Sudo Caching](https://attack.mitre.org/techniques/T1548/003)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has elevated privileges of local user accounts by editing /etc/sudoers or adding a file under /etc/sudoers.d/.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1098](https://attack.mitre.org/techniques/T1098)   |  [.004](https://attack.mitre.org/techniques/T1098/004)   |  [Account Manipulation](https://attack.mitre.org/techniques/T1098): [SSH Authorized Keys](https://attack.mitre.org/techniques/T1098/004)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has added SSH authorized_keys under root or other users at the Linux level on compromised network devices.[[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/)[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1583](https://attack.mitre.org/techniques/T1583)   |  [.003](https://attack.mitre.org/techniques/T1583/003)   |  [Acquire Infrastructure](https://attack.mitre.org/techniques/T1583): [Virtual Private Server](https://attack.mitre.org/techniques/T1583/003)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has utilized virtual private servers (VPS) for C2 infrastructure.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1595](https://attack.mitre.org/techniques/T1595)   |  [Active Scanning](https://attack.mitre.org/techniques/T1595)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has scanned victim environments for open ports and services.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1071](https://attack.mitre.org/techniques/T1071)   |  [.001](https://attack.mitre.org/techniques/T1071/001)   |  [Application Layer Protocol](https://attack.mitre.org/techniques/T1071): [Web Protocols](https://attack.mitre.org/techniques/T1071/001)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has utilized HTTP and HTTPS in C2 communications.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|   |   |  [.002](https://attack.mitre.org/techniques/T1071/002)   |  [Application Layer Protocol](https://attack.mitre.org/techniques/T1071): [File Transfer Protocols](https://attack.mitre.org/techniques/T1071/002)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has leveraged SFTP for C2 communications.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1020](https://attack.mitre.org/techniques/T1020)   |  [.001](https://attack.mitre.org/techniques/T1020/001)   |  [Automated Exfiltration](https://attack.mitre.org/techniques/T1020): [Traffic Duplication](https://attack.mitre.org/techniques/T1020/001)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has enabled traffic mirroring to exfiltrate data.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1110](https://attack.mitre.org/techniques/T1110)   |  [.001](https://attack.mitre.org/techniques/T1110/001)   |  [Brute Force](https://attack.mitre.org/techniques/T1110): [Password Guessing](https://attack.mitre.org/techniques/T1110/001)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has used weak credentials to gain access to victim devices.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|   |   |  [.002](https://attack.mitre.org/techniques/T1110/002)   |  [Brute Force](https://attack.mitre.org/techniques/T1110): [Password Cracking](https://attack.mitre.org/techniques/T1110/002)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has cracked passwords for accounts with weak encryption obtained from the configuration files of compromised network devices.[[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/)

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has cracked weak hashed passwords stolen from configuration files.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1059](https://attack.mitre.org/techniques/T1059)   |  [.004](https://attack.mitre.org/techniques/T1059/004)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [Unix Shell](https://attack.mitre.org/techniques/T1059/004)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has utilized a bash shell to execute Linux commands.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|   |   |  [.006](https://attack.mitre.org/techniques/T1059/006)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [Python](https://attack.mitre.org/techniques/T1059/006)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has used Python scripts to execute commands on victim hosts.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|   |   |  [.008](https://attack.mitre.org/techniques/T1059/008)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [Network Device CLI](https://attack.mitre.org/techniques/T1059/008)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has executed commands on network devices using the native command line interface.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has leveraged Tcl scripts on Cisco IOS devices to include TCLprox.tcl and map.tcl when tclsh was available.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

[Salt Typhoon](https://attack.mitre.org/groups/G1045) also leveraged TACACS+ and RADIUS to modify victim network devices.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|   |   |  [.013](https://attack.mitre.org/techniques/T1059/013)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [Container CLI/API](https://attack.mitre.org/techniques/T1059/013)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has leveraged Cisco Guest Shell to interact with a Linux Container on network devices.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1584](https://attack.mitre.org/techniques/T1584)   |  [.008](https://attack.mitre.org/techniques/T1584/008)   |  [Compromise Infrastructure](https://attack.mitre.org/techniques/T1584): [Network Devices](https://attack.mitre.org/techniques/T1584/008)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has leveraged compromised routers to facilitate operations.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1136](https://attack.mitre.org/techniques/T1136)   |  [Create Account](https://attack.mitre.org/techniques/T1136)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has created Linux-level users on compromised network devices through modification of /etc/shadow and /etc/passwd.[[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/)

|   |   |  [.001](https://attack.mitre.org/techniques/T1136/001)   |  [Local Account](https://attack.mitre.org/techniques/T1136/001)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has created local user accounts to utilize in victim environments.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has also created administrative accounts within compromised environments.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1543](https://attack.mitre.org/techniques/T1543)   |  [.005](https://attack.mitre.org/techniques/T1543/005)   |  [Create or Modify System Process](https://attack.mitre.org/techniques/T1543): [Container Service](https://attack.mitre.org/techniques/T1543/005)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has modified container service configurations using Guest Shell.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1132](https://attack.mitre.org/techniques/T1132)   |  [.001](https://attack.mitre.org/techniques/T1132/001)   |  [Data Encoding](https://attack.mitre.org/techniques/T1132): [Standard Encoding](https://attack.mitre.org/techniques/T1132/001)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has utilized double encoding with hexadecimals in attempts to obfuscate requested command paths.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1602](https://attack.mitre.org/techniques/T1602)   |  [.001](https://attack.mitre.org/techniques/T1602/001)   |  [Data from Configuration Repository](https://attack.mitre.org/techniques/T1602): [SNMP (MIB Dump)](https://attack.mitre.org/techniques/T1602/001)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has enumerated device configuration using SNMP and gathered data stored in the Managed Information Base on target devices.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|   |   |  [.002](https://attack.mitre.org/techniques/T1602/002)   |  [Data from Configuration Repository](https://attack.mitre.org/techniques/T1602): [Network Device Configuration Dump](https://attack.mitre.org/techniques/T1602/002)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has attempted to acquire credentials by dumping network device configurations.[[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/)[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a) [Salt Typhoon](https://attack.mitre.org/groups/G1045) has also obtained data using RANCID.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1005](https://attack.mitre.org/techniques/T1005)   |  [Data from Local System](https://attack.mitre.org/techniques/T1005)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has gathered data from victim devices.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1610](https://attack.mitre.org/techniques/T1610)   |  [Deploy Container](https://attack.mitre.org/techniques/T1610)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has enabled Linux containers in victim network device environments using guestshell enable.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1587](https://attack.mitre.org/techniques/T1587)   |  [Develop Capabilities](https://attack.mitre.org/techniques/T1587)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has hosted and attended formal training programs for foreign-made IT products to include Cisco Network Academy.[[4]](https://www.sentinelone.com/labs/malicious-apprentice-how-two-hackers-went-from-cisco-academy-to-cisco-cves/)

|   |   |  [.001](https://attack.mitre.org/techniques/T1587/001)   |  [Malware](https://attack.mitre.org/techniques/T1587/001)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has used custom tooling including [JumbledPath](https://attack.mitre.org/software/S1206).[[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/)

|  Enterprise   |  [T1686](https://attack.mitre.org/techniques/T1686)   |  [Disable or Modify System Firewall](https://attack.mitre.org/techniques/T1686)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has made changes to the Access Control List (ACL) and loopback interface address on compromised devices.[[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/)

|   |   |  [.002](https://attack.mitre.org/techniques/T1686/002)   |  [Network Device Firewall](https://attack.mitre.org/techniques/T1686/002)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has modified the network firewalls access control lists to add IP addresses to bypass security policies and maintain access.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1685](https://attack.mitre.org/techniques/T1685)   |  [Disable or Modify Tools](https://attack.mitre.org/techniques/T1685)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has disabled logging and disabled the distribution of logs to log aggregators.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|   |   |  [.006](https://attack.mitre.org/techniques/T1685/006)   |  [Clear Linux or Mac System Logs](https://attack.mitre.org/techniques/T1685/006)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has cleared logs including .bash_history, auth.log, lastlog, wtmp, and btmp.[[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/)

|  Enterprise   |  [T1585](https://attack.mitre.org/techniques/T1585)   |  [.001](https://attack.mitre.org/techniques/T1585/001)   |  [Establish Accounts](https://attack.mitre.org/techniques/T1585): [Social Media Accounts](https://attack.mitre.org/techniques/T1585/001)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) members have created and maintained professional accounts on social media to include LinkedIn.[[4]](https://www.sentinelone.com/labs/malicious-apprentice-how-two-hackers-went-from-cisco-academy-to-cisco-cves/)

|  Enterprise   |  [T1048](https://attack.mitre.org/techniques/T1048)   |  [.003](https://attack.mitre.org/techniques/T1048/003)   |  [Exfiltration Over Alternative Protocol](https://attack.mitre.org/techniques/T1048): [Exfiltration Over Unencrypted Non-C2 Protocol](https://attack.mitre.org/techniques/T1048/003)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has exfiltrated configuration files from exploited network devices over FTP and TFTP.[[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/)[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1190](https://attack.mitre.org/techniques/T1190)   |  [Exploit Public-Facing Application](https://attack.mitre.org/techniques/T1190)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has leveraged vulnerabilities in public facing applications and network edge devices including Cisco IOS and IOS XE through CVE-2018-0171, Ivanti Connect Secure through CVE-2023-46805, and Palo Alto Networks GlobalProtect through CVE-2024-3400.[[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/)[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)[[4]](https://www.sentinelone.com/labs/malicious-apprentice-how-two-hackers-went-from-cisco-academy-to-cisco-cves/)

|  Enterprise   |  [T1590](https://attack.mitre.org/techniques/T1590)   |  [.004](https://attack.mitre.org/techniques/T1590/004)   |  [Gather Victim Network Information](https://attack.mitre.org/techniques/T1590): [Network Topology](https://attack.mitre.org/techniques/T1590/004)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has used configuration files from exploited network devices to help discover upstream and downstream network segments.[[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/)[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1070](https://attack.mitre.org/techniques/T1070)   |  [.003](https://attack.mitre.org/techniques/T1070/003)   |  [Indicator Removal](https://attack.mitre.org/techniques/T1070): [Clear Command History](https://attack.mitre.org/techniques/T1070/003)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has deleted and cleared logs to avoid detection.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|   |   |  [.007](https://attack.mitre.org/techniques/T1070/007)   |  [Indicator Removal](https://attack.mitre.org/techniques/T1070): [Clear Network Connection History and Configurations](https://attack.mitre.org/techniques/T1070/007)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has reverted or modified network configuration files to remove traces of activity.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|   |   |  [.009](https://attack.mitre.org/techniques/T1070/009)   |  [Indicator Removal](https://attack.mitre.org/techniques/T1070): [Clear Persistence](https://attack.mitre.org/techniques/T1070/009)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has removed persistence within victim devices through disabling and uninstalling artifacts.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)
[Salt Typhoon](https://attack.mitre.org/groups/G1045) has leveraged the commands guestshell disable and guestshell destroy to deactivate and uninstall Guest Shell containers and return all resources to the system to remove evidence of persistence.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1105](https://attack.mitre.org/techniques/T1105)   |  [Ingress Tool Transfer](https://attack.mitre.org/techniques/T1105)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has downloaded additional payloads and files to victim devices.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1556](https://attack.mitre.org/techniques/T1556)   |  [Modify Authentication Process](https://attack.mitre.org/techniques/T1556)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has modified the Authentication, Authorization, and Accounting (AAA) configurations forcing devices to use less secure authentication methods and to send information to their infrastructure.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1599](https://attack.mitre.org/techniques/T1599)   |  [Network Boundary Bridging](https://attack.mitre.org/techniques/T1599)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has abused peer connections to exfiltrate data.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1046](https://attack.mitre.org/techniques/T1046)   |  [Network Service Discovery](https://attack.mitre.org/techniques/T1046)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has scanned open ports and services within the victim environment.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1040](https://attack.mitre.org/techniques/T1040)   |  [Network Sniffing](https://attack.mitre.org/techniques/T1040)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has used a variety of tools and techniques to capture packet data between network interfaces.[[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/)[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1095](https://attack.mitre.org/techniques/T1095)   |  [Non-Application Layer Protocol](https://attack.mitre.org/techniques/T1095)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has used Generic Routing Encapsulation (GRE) and IPsec tunnels to connect to C2 over non-application layer protocols.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1571](https://attack.mitre.org/techniques/T1571)   |  [Non-Standard Port](https://attack.mitre.org/techniques/T1571)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has leveraged non-standard ports to evade detection by security monitoring.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1027](https://attack.mitre.org/techniques/T1027)   |  [.010](https://attack.mitre.org/techniques/T1027/010)   |  [Obfuscated Files or Information](https://attack.mitre.org/techniques/T1027): [Command Obfuscation](https://attack.mitre.org/techniques/T1027/010)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has obfuscated target pathways with double encoding to bypass security solutions.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1588](https://attack.mitre.org/techniques/T1588)   |  [.002](https://attack.mitre.org/techniques/T1588/002)   |  [Obtain Capabilities](https://attack.mitre.org/techniques/T1588): [Tool](https://attack.mitre.org/techniques/T1588/002)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has used publicly available tooling to exploit vulnerabilities.[[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/)[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has also obtained and utilized open-source tools for creating backdoors and multi-hop proxies to include STOWAWAY.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|   |   |  [.005](https://attack.mitre.org/techniques/T1588/005)   |  [Obtain Capabilities](https://attack.mitre.org/techniques/T1588): [Exploits](https://attack.mitre.org/techniques/T1588/005)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has obtained and utilized numerous exploits that target network edge devices to include Cisco IOS, Palo Alto Pan-OS Global Protect, and Ivanti Connect Secure.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1572](https://attack.mitre.org/techniques/T1572)   |  [Protocol Tunneling](https://attack.mitre.org/techniques/T1572)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has modified device configurations to create and use Generic Routing Encapsulation (GRE) tunnels.[[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/)

|  Enterprise   |  [T1090](https://attack.mitre.org/techniques/T1090)   |  [Proxy](https://attack.mitre.org/techniques/T1090)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has utilized virtual private servers and compromised routers as proxies for malicious communications.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|   |   |  [.003](https://attack.mitre.org/techniques/T1090/003)   |  [Multi-hop Proxy](https://attack.mitre.org/techniques/T1090/003)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has leveraged multi-hop proxies for command-and-control infrastructure.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1021](https://attack.mitre.org/techniques/T1021)   |  [.001](https://attack.mitre.org/techniques/T1021/001)   |  [Remote Services](https://attack.mitre.org/techniques/T1021): [Remote Desktop Protocol](https://attack.mitre.org/techniques/T1021/001)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has opened and conducted operations utilizing RDP within victim environments.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|   |   |  [.004](https://attack.mitre.org/techniques/T1021/004)   |  [Remote Services](https://attack.mitre.org/techniques/T1021): [SSH](https://attack.mitre.org/techniques/T1021/004)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has modified the loopback address on compromised switches and used them as the source of SSH connections to additional devices within the target environment, allowing them to bypass access control lists (ACLs).[[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/) [Salt Typhoon](https://attack.mitre.org/groups/G1045) has also enabled SSH to allow remote access.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1018](https://attack.mitre.org/techniques/T1018)   |  [Remote System Discovery](https://attack.mitre.org/techniques/T1018)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has used SNMPwalk to enumerate devices.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1072](https://attack.mitre.org/techniques/T1072)   |  [Software Deployment Tools](https://attack.mitre.org/techniques/T1072)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has abused network management tools to include RANCID to facilitate actions in victims’ environments.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1082](https://attack.mitre.org/techniques/T1082)   |  [System Information Discovery](https://attack.mitre.org/techniques/T1082)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has discovered system information details utilizing CLI on network devices.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1016](https://attack.mitre.org/techniques/T1016)   |  [System Network Configuration Discovery](https://attack.mitre.org/techniques/T1016)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has discovered and modified network configurations of victim devices.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1569](https://attack.mitre.org/techniques/T1569)   |  [System Services](https://attack.mitre.org/techniques/T1569)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has utilized SNMP daemon to execute commands to include SNMPwalk.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1199](https://attack.mitre.org/techniques/T1199)   |  [Trusted Relationship](https://attack.mitre.org/techniques/T1199)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has leveraged compromised devices associated with trusted connections or those associated with private interconnections to include provider-to-provider or provider-to-customer relationships in order to pivot to target networks.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1552](https://attack.mitre.org/techniques/T1552)   |  [.001](https://attack.mitre.org/techniques/T1552/001)   |  [Unsecured Credentials](https://attack.mitre.org/techniques/T1552): [Credentials In Files](https://attack.mitre.org/techniques/T1552/001)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has collected credentials stored in configuration files.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  Enterprise   |  [T1078](https://attack.mitre.org/techniques/T1078)   |  [Valid Accounts](https://attack.mitre.org/techniques/T1078)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has used logged into accounts on Cisco devices by cracking weak hashed passwords from configuration files.[[3]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

## Software

| ID  | Name  | References  | Techniques

|  [S1206](https://attack.mitre.org/software/S1206)   |  [JumbledPath](https://attack.mitre.org/software/S1206)   |  [[2]](https://blog.talosintelligence.com/salt-typhoon-analysis/)   |  [Archive Collected Data](https://attack.mitre.org/techniques/T1560), [Disable or Modify Tools](https://attack.mitre.org/techniques/T1685), [Disable or Modify Tools](https://attack.mitre.org/techniques/T1685): [Clear Linux or Mac System Logs](https://attack.mitre.org/techniques/T1685/006), [Hide Infrastructure](https://attack.mitre.org/techniques/T1665), [Multi-Stage Channels](https://attack.mitre.org/techniques/T1104), [Network Sniffing](https://attack.mitre.org/techniques/T1040)

## References

-    [US Department of Treasury. (2025, January 17). Treasury Sanctions Company Associated with Salt Typhoon and Hacker Associated with Treasury Compromise. Retrieved February 24, 2025.](https://home.treasury.gov/news/press-releases/jy2792)
-    [Cisco Talos. (2025, February 20). Weathering the storm: In the midst of a Typhoon. Retrieved February 24, 2025.](https://blog.talosintelligence.com/salt-typhoon-analysis/)
-    [CISA. (2025, September 3). Countering Chinese State-Sponsored Actors Compromise of Networks Worldwide to Feed Global Espionage System. Retrieved September 22, 2026.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)
-    [Cary, D. (2025, December 10). Malicious Apprentice | How Two Hackers Went From Cisco Academy to Cisco CVEs. Retrieved September 22, 2026.](https://www.sentinelone.com/labs/malicious-apprentice-how-two-hackers-went-from-cisco-academy-to-cisco-cves/)

-    [CrowdStrike. (n.d.). Operator Panda. Retrieved September 22, 2026.](https://www.crowdstrike.com/en-us/adversaries/operator-panda/)
-    [Insikt Group. (2025, February 13). RedMike (Salt Typhoon) Exploits Cisco Vulnerabilities for Cyber Espionage. Retrieved September 22, 2026.](https://www.recordedfuture.com/research/redmike-salt-typhoon-exploits-vulnerable-devices)
-    [Microsoft. (n.d.). How Microsoft names threat actors. Retrieved September 22, 2026.](https://learn.microsoft.com/en-us/unified-secops/microsoft-threat-actor-naming)
-    [Kutscher, J. (2026, March 23). M-Trends 2026: Data, Insights, and Strategies From the Frontlines. Retrieved September 22, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/m-trends-2026)

×
