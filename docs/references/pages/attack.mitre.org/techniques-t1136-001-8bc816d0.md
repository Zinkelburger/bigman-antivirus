---
url: "https://attack.mitre.org/techniques/T1136/001/"
title: "Create Account: Local Account, Sub-technique T1136.001 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:36:06Z"
via: "live"
sha256: "e3a1945b65aabb7c0d74caf8fa2c3960a9b6c2ea3508e369fbd4d2f973d79c88"
wayback: "https://web.archive.org/web/20261023090759/https://attack.mitre.org/techniques/T1136/001/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Create Account: Local Account, Sub-technique T1136.001 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1136/001/>
> Snapshot: <https://web.archive.org/web/20261023090759/https://attack.mitre.org/techniques/T1136/001/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Create Account](https://attack.mitre.org/techniques/T1136)
- Local Account

#  Create Account: Local Account

##### Other sub-techniques of Create Account (3)

| ID  | Name

|  T1136.001   |  Local Account

|  [T1136.002](https://attack.mitre.org/techniques/T1136/002/)   |  [Domain Account](https://attack.mitre.org/techniques/T1136/002/)

|  [T1136.003](https://attack.mitre.org/techniques/T1136/003/)   |  [Cloud Account](https://attack.mitre.org/techniques/T1136/003/)

Adversaries may create a local account to maintain access to victim systems. Local accounts are those configured by an organization for use by users, remote support, services, or for administration on a single system or service.

For example, with a sufficient level of access, the Windows net user /add command can be used to create a local account. In Linux, the useradd command can be used, while on macOS systems, the dscl -create command can be used. Local accounts may also be added to network devices, often via common [Network Device CLI](https://attack.mitre.org/techniques/T1059/008) commands such as username, to ESXi servers via esxcli system account add, or to Kubernetes clusters using the kubectl utility.[[1]](https://www.cisco.com/c/en/us/td/docs/ios-xml/ios/security/s1/sec-s1-cr-book/sec-cr-t2.html#wp1047035630)[[2]](https://kubernetes.io/docs/concepts/security/service-accounts/)

Adversaries may also create new local accounts on network firewall management consoles – for example, by exploiting a vulnerable firewall management system, threat actors may be able to establish super-admin accounts that could be used to modify firewall rules and gain further access to the network.[[3]](https://cybersecuritynews.com/superblack-actors-exploiting-two-fortinet-vulnerabilities/)

Such accounts may be used to establish secondary credentialed access that do not require persistent remote access tools to be deployed on the system.

ID: T1136.001

Sub-technique of:  [T1136](https://attack.mitre.org/techniques/T1136)

ⓘ

Tactic: [Persistence](https://attack.mitre.org/tactics/TA0003)

ⓘ

Platforms: Containers, ESXi, Linux, Network Devices, Windows, macOS

Contributors: Austin Clark, @c2defense

Version: 1.5

Created: 28 January 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1136/001/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1136/001/)

## Procedure Examples

| ID  | Name  | Description

|  [C0062](https://attack.mitre.org/campaigns/C0062)   |  [Anthropic AI-orchestrated Campaign](https://attack.mitre.org/campaigns/C0062)   |

During the [Anthropic AI-orchestrated Campaign](https://attack.mitre.org/campaigns/C0062), the adversary used Claude Code to create a local backdoor account to maintain access.[[4]](https://assets.anthropic.com/m/ec212e6566a0d47/original/Disrupting-the-first-reported-AI-orchestrated-cyber-espionage-campaign.pdf)

|  [G0022](https://attack.mitre.org/groups/G0022)   |  [APT3](https://attack.mitre.org/groups/G0022)   |

[APT3](https://attack.mitre.org/groups/G0022) has been known to create or enable accounts, such as support_388945a0.[[5]](http://carnal0wnage.attackresearch.com/2012/09/more-on-aptsim.html)

|  [G0087](https://attack.mitre.org/groups/G0087)   |  [APT39](https://attack.mitre.org/groups/G0087)   |

[APT39](https://attack.mitre.org/groups/G0087) has created accounts on multiple compromised hosts to perform actions within the network.[[6]](https://www.bitdefender.com/blog/labs/iranian-chafer-apt-targeted-air-transportation-and-government-in-kuwait-and-saudi-arabia/)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) has created user accounts.[[7]](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)

|  [G1023](https://attack.mitre.org/groups/G1023)   |  [APT5](https://attack.mitre.org/groups/G1023)   |

[APT5](https://attack.mitre.org/groups/G1023) has created Local Administrator accounts to maintain access to systems with short-cycle credential rotation.[[8]](https://www.mandiant.com/resources/blog/updates-on-chinese-apt-compromising-pulse-secure-vpn-devices)

|  [S0274](https://attack.mitre.org/software/S0274)   |  [Calisto](https://attack.mitre.org/software/S0274)   |

[Calisto](https://attack.mitre.org/software/S0274) has the capability to add its own account to the victim's machine.[[9]](https://web.archive.org/web/20190111082249/https://www.symantec.com/security-center/writeup/2018-073014-2512-99?om_rssid=sr-latestthreats30days)

|  [S0030](https://attack.mitre.org/software/S0030)   |  [Carbanak](https://attack.mitre.org/software/S0030)   |

[Carbanak](https://attack.mitre.org/software/S0030) can create a Windows account.[[10]](https://www.fireeye.com/blog/threat-research/2017/06/behind-the-carbanak-backdoor.html)

|  [G1034](https://attack.mitre.org/groups/G1034)   |  [Daggerfly](https://attack.mitre.org/groups/G1034)   |

[Daggerfly](https://attack.mitre.org/groups/G1034) created a local account on victim machines to maintain access.[[11]](https://symantec-enterprise-blogs.security.com/threat-intelligence/apt-attacks-telecoms-africa-mgbot)

|  [S1111](https://attack.mitre.org/software/S1111)   |  [DarkGate](https://attack.mitre.org/software/S1111)   |

[DarkGate](https://attack.mitre.org/software/S1111) creates a local user account, SafeMode, via net user commands.[[12]](https://www.fortinet.com/blog/threat-research/enter-the-darkgate-new-cryptocurrency-mining-and-ransomware-campaign)

|  [G0035](https://attack.mitre.org/groups/G0035)   |  [Dragonfly](https://attack.mitre.org/groups/G0035)   |

[Dragonfly](https://attack.mitre.org/groups/G0035) has created accounts on victims, including administrator accounts, some of which appeared to be tailored to each individual staging target.[[13]](https://www.us-cert.gov/ncas/alerts/TA18-074A)

|  [S0363](https://attack.mitre.org/software/S0363)   |  [Empire](https://attack.mitre.org/software/S0363)   |

[Empire](https://attack.mitre.org/software/S0363) has a module for creating a local user if permissions allow.[[14]](https://github.com/PowerShellEmpire/Empire)

|  [G1016](https://attack.mitre.org/groups/G1016)   |  [FIN13](https://attack.mitre.org/groups/G1016)   |

[FIN13](https://attack.mitre.org/groups/G1016) has created MS-SQL local accounts in a compromised network.[[15]](https://web.archive.org/web/20220105132433/https://f.hubspotusercontent30.net/hubfs/8776530/Sygnia-%20Elephant%20Beetle_Jan2022.pdf)

|  [S0143](https://attack.mitre.org/software/S0143)   |  [Flame](https://attack.mitre.org/software/S0143)   |

[Flame](https://attack.mitre.org/software/S0143) can create backdoor accounts with login "HelpAssistant" on domain connected systems if appropriate rights are available.[[16]](https://securelist.com/the-flame-questions-and-answers-51/34344/)[[17]](https://securelist.com/flame-bunny-frog-munch-and-beetlejuice-2/32855/)

|  [G0117](https://attack.mitre.org/groups/G0117)   |  [Fox Kitten](https://attack.mitre.org/groups/G0117)   |

[Fox Kitten](https://attack.mitre.org/groups/G0117) has created a local user account with administrator privileges.[[18]](https://www.clearskysec.com/wp-content/uploads/2020/12/Pay2Kitten.pdf)

|  [S0493](https://attack.mitre.org/software/S0493)   |  [GoldenSpy](https://attack.mitre.org/software/S0493)   |

[GoldenSpy](https://attack.mitre.org/software/S0493) can create new users on an infected system.[[19]](https://www.trustwave.com/en-us/resources/library/documents/the-golden-tax-department-and-the-emergence-of-goldenspy-malware/)

|  [S0394](https://attack.mitre.org/software/S0394)   |  [HiddenWasp](https://attack.mitre.org/software/S0394)   |

[HiddenWasp](https://attack.mitre.org/software/S0394) creates a user account as a means to provide initial persistence to the compromised machine.[[20]](https://www.intezer.com/blog-hiddenwasp-malware-targeting-linux-systems/)

|  [S0601](https://attack.mitre.org/software/S0601)   |  [Hildegard](https://attack.mitre.org/software/S0601)   |

[Hildegard](https://attack.mitre.org/software/S0601) has created a user named "monerodaemon".[[21]](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)

|  [G0119](https://attack.mitre.org/groups/G0119)   |  [Indrik Spider](https://attack.mitre.org/groups/G0119)   |

[Indrik Spider](https://attack.mitre.org/groups/G0119) has created local system accounts and has added the accounts to privileged groups.[[22]](https://cloud.google.com/blog/topics/threat-intelligence/unc2165-shifts-to-evade-sanctions/)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has created accounts with net user.[[23]](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)

|  [G0077](https://attack.mitre.org/groups/G0077)   |  [Leafminer](https://attack.mitre.org/groups/G0077)   |

[Leafminer](https://attack.mitre.org/groups/G0077) used a tool called Imecab to set up a persistent remote access account on the victim machine.[[24]](https://www.symantec.com/blogs/threat-intelligence/leafminer-espionage-middle-east)

|  [G0059](https://attack.mitre.org/groups/G0059)   |  [Magic Hound](https://attack.mitre.org/groups/G0059)   |

[Magic Hound](https://attack.mitre.org/groups/G0059) has created local accounts named help and DefaultAccount on compromised machines.[[25]](https://thedfirreport.com/2022/03/21/apt35-automates-initial-access-using-proxyshell)[[26]](https://www.microsoft.com/en-us/security/blog/2021/11/16/evolving-trends-in-iranian-threat-actor-activity-mstic-presentation-at-cyberwarcon-2021)

|  [S0084](https://attack.mitre.org/software/S0084)   |  [Mis-Type](https://attack.mitre.org/software/S0084)   |

[Mis-Type](https://attack.mitre.org/software/S0084) may create a temporary user on the system named Lost_{Unique Identifier}.[[27]](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)

|  [S0039](https://attack.mitre.org/software/S0039)   |  [Net](https://attack.mitre.org/software/S0039)   |

The net user username \password commands in [Net](https://attack.mitre.org/software/S0039) can be used to create a local account.[[28]](https://web.archive.org/web/20150511162820/http://windowsitpro.com/windows/netexe-reference)

|  [G1059](https://attack.mitre.org/groups/G1059)   |  [NIMBUS MANTICORE](https://attack.mitre.org/groups/G1059)   |

[NIMBUS MANTICORE](https://attack.mitre.org/groups/G1059) has created rogue accounts within the compromised environments.[[29]](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)

|  [S0192](https://attack.mitre.org/software/S0192)   |  [Pupy](https://attack.mitre.org/software/S0192)   |

[Pupy](https://attack.mitre.org/software/S0192) can user PowerView to execute "net user" commands and create local system accounts.[[30]](https://github.com/n1nj4sec/pupy)

|  [S0085](https://attack.mitre.org/software/S0085)   |  [S-Type](https://attack.mitre.org/software/S0085)   |

[S-Type](https://attack.mitre.org/software/S0085) may create a temporary user on the system named Lost_{Unique Identifier} with the password pond~!@6"{Unique Identifier}.[[27]](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)

|  [G1045](https://attack.mitre.org/groups/G1045)   |  [Salt Typhoon](https://attack.mitre.org/groups/G1045)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has created local user accounts to utilize in victim environments.[[31]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has also created administrative accounts within compromised environments.[[31]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  [S0382](https://attack.mitre.org/software/S0382)   |  [ServHelper](https://attack.mitre.org/software/S0382)   |

[ServHelper](https://attack.mitre.org/software/S0382) has created a new user named "supportaccount".[[32]](https://www.proofpoint.com/us/threat-insight/post/servhelper-and-flawedgrace-new-malware-introduced-ta505)

|  [S0649](https://attack.mitre.org/software/S0649)   |  [SMOKEDHAM](https://attack.mitre.org/software/S0649)   |

[SMOKEDHAM](https://attack.mitre.org/software/S0649) has created user accounts.[[33]](https://www.fireeye.com/blog/threat-research/2021/06/darkside-affiliate-supply-chain-software-compromise.html)

|  [G0139](https://attack.mitre.org/groups/G0139)   |  [TeamTNT](https://attack.mitre.org/groups/G0139)   |

[TeamTNT](https://attack.mitre.org/groups/G0139) has created local privileged users on victim machines.[[34]](https://www.intezer.com/blog/cloud-security/attackers-abusing-legitimate-cloud-monitoring-tools-to-conduct-cyber-attacks/)

|  [G0102](https://attack.mitre.org/groups/G0102)   |  [Wizard Spider](https://attack.mitre.org/groups/G0102)   |

[Wizard Spider](https://attack.mitre.org/groups/G0102) has created local administrator accounts to maintain persistence in compromised networks.[[35]](https://web.archive.org/web/20220313061955/https://www.mandiant.com/sites/default/files/2021-10/fin12-group-profile.pdf)

|  [S0412](https://attack.mitre.org/software/S0412)   |  [ZxShell](https://attack.mitre.org/software/S0412)   |

[ZxShell](https://attack.mitre.org/software/S0412) has a feature to create local user accounts.[[36]](https://blogs.cisco.com/security/talos/opening-zxshell)

## Mitigations

| ID  | Mitigation  | Description

|  [M1032](https://attack.mitre.org/mitigations/M1032)   |  [Multi-factor Authentication](https://attack.mitre.org/mitigations/M1032)   |

Use multi-factor authentication for user and privileged accounts.

|  [M1026](https://attack.mitre.org/mitigations/M1026)   |  [Privileged Account Management](https://attack.mitre.org/mitigations/M1026)   |

Limit the number of accounts permitted to create other accounts. Limit the usage of local administrator accounts to be used for day-to-day operations that may expose them to potential adversaries.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0447](https://attack.mitre.org/detectionstrategies/DET0447)   |  [T1136.001 Detection Strategy - Local Account Creation Across Platforms](https://attack.mitre.org/detectionstrategies/DET0447)   |  [AN1235](https://attack.mitre.org/detectionstrategies/DET0447#AN1235)   |

Adversary uses built-in tools like 'net user /add', PowerShell, or WMI to create a local user. Sequence: Account creation event (4720) follows process creation of a suspicious executable (e.g., powershell.exe or net.exe).

|  [AN1236](https://attack.mitre.org/detectionstrategies/DET0447#AN1236)   |

Local user accounts are created via binaries like 'useradd', 'adduser', or by editing passwd/shadow. Behavior chain includes execution of user management binaries or modification of user database files.

|  [AN1237](https://attack.mitre.org/detectionstrategies/DET0447#AN1237)   |

Account creation using 'dscl -create' or via GUI tools. Detection involves command execution and file changes to the local directory services database.

|  [AN1238](https://attack.mitre.org/detectionstrategies/DET0447#AN1238)   |

Account created using esxcli commands. Sequence includes esxcli execution and successful modification to account DB.

|  [AN1239](https://attack.mitre.org/detectionstrategies/DET0447#AN1239)   |

Account created in a running container (e.g., via 'useradd' or by modifying /etc/passwd directly). Detectable via runtime telemetry (e.g., Falco or eBPF hooks).

|  [AN1240](https://attack.mitre.org/detectionstrategies/DET0447#AN1240)   |

Account created via CLI using 'username' command or REST API. Detectable through AAA logging or CLI history telemetry.

## References

-    [Cisco. (2023, March 6). username - Cisco IOS Security Command Reference: Commands S to Z. Retrieved July 13, 2022.](https://www.cisco.com/c/en/us/td/docs/ios-xml/ios/security/s1/sec-s1-cr-book/sec-cr-t2.html#wp1047035630)
-    [Kubernetes. (n.d.). Service Accounts. Retrieved July 14, 2023.](https://kubernetes.io/docs/concepts/security/service-accounts/)
-    [Kaaviya. (n.d.). SuperBlack Actors Exploiting Two Fortinet Vulnerabilities to Deploy Ransomware. Retrieved September 22, 2025.](https://cybersecuritynews.com/superblack-actors-exploiting-two-fortinet-vulnerabilities/)
-    [Anthropic. (2025, November). Disrupting the first reported AI-orchestrated cyber espionage campaign. Retrieved April 20, 2026.](https://assets.anthropic.com/m/ec212e6566a0d47/original/Disrupting-the-first-reported-AI-orchestrated-cyber-espionage-campaign.pdf)
-    [valsmith. (2012, September 21). More on APTSim. Retrieved September 28, 2017.](http://carnal0wnage.attackresearch.com/2012/09/more-on-aptsim.html)
-    [Rusu, B. (2020, May 21). Iranian Chafer APT Targeted Air Transportation and Government in Kuwait and Saudi Arabia. Retrieved May 22, 2020.](https://www.bitdefender.com/blog/labs/iranian-chafer-apt-targeted-air-transportation-and-government-in-kuwait-and-saudi-arabia/)
-    [Fraser, N., et al. (2019, August 7). Double DragonAPT41, a dual espionage and cyber crime operation APT41. Retrieved September 23, 2019.](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)
-    [Perez, D. et al. (2021, May 27). Re-Checking Your Pulse: Updates on Chinese APT Actors Compromising Pulse Secure VPN Devices. Retrieved February 5, 2024.](https://www.mandiant.com/resources/blog/updates-on-chinese-apt-compromising-pulse-secure-vpn-devices)
-    [Pantig, J. (2018, July 30). OSX.Calisto. Retrieved September 7, 2018.](https://web.archive.org/web/20190111082249/https://www.symantec.com/security-center/writeup/2018-073014-2512-99?om_rssid=sr-latestthreats30days)
-    [Bennett, J., Vengerik, B. (2017, June 12). Behind the CARBANAK Backdoor. Retrieved June 11, 2018.](https://www.fireeye.com/blog/threat-research/2017/06/behind-the-carbanak-backdoor.html)
-    [Threat Hunter Team. (2023, April 20). Daggerfly: APT Actor Targets Telecoms Company in Africa. Retrieved July 25, 2024.](https://symantec-enterprise-blogs.security.com/threat-intelligence/apt-attacks-telecoms-africa-mgbot)
-    [Adi Zeligson & Rotem Kerner. (2018, November 13). Enter The DarkGate - New Cryptocurrency Mining and Ransomware Campaign. Retrieved February 9, 2024.](https://www.fortinet.com/blog/threat-research/enter-the-darkgate-new-cryptocurrency-mining-and-ransomware-campaign)
-    [US-CERT. (2018, March 16). Alert (TA18-074A): Russian Government Cyber Activity Targeting Energy and Other Critical Infrastructure Sectors. Retrieved June 6, 2018.](https://www.us-cert.gov/ncas/alerts/TA18-074A)
-    [Schroeder, W., Warner, J., Nelson, M. (n.d.). Github PowerShellEmpire. Retrieved April 28, 2016.](https://github.com/PowerShellEmpire/Empire)
-    [Sygnia Incident Response Team. (2022, January 5). TG2003: ELEPHANT BEETLE UNCOVERING AN ORGANIZED FINANCIAL-THEFT OPERATION. Retrieved February 9, 2023.](https://web.archive.org/web/20220105132433/https://f.hubspotusercontent30.net/hubfs/8776530/Sygnia-%20Elephant%20Beetle_Jan2022.pdf)
-    [Gostev, A. (2012, May 28). The Flame: Questions and Answers. Retrieved March 1, 2017.](https://securelist.com/the-flame-questions-and-answers-51/34344/)
-    [Gostev, A. (2012, May 30). Flame: Bunny, Frog, Munch and BeetleJuice…. Retrieved March 1, 2017.](https://securelist.com/flame-bunny-frog-munch-and-beetlejuice-2/32855/)
-    [ClearSky. (2020, December 17). Pay2Key Ransomware – A New Campaign by Fox Kitten. Retrieved December 21, 2020.](https://www.clearskysec.com/wp-content/uploads/2020/12/Pay2Kitten.pdf)

-    [Trustwave SpiderLabs. (2020, June 25). The Golden Tax Department and Emergence of GoldenSpy Malware. Retrieved July 23, 2020.](https://www.trustwave.com/en-us/resources/library/documents/the-golden-tax-department-and-the-emergence-of-goldenspy-malware/)
-    [Sanmillan, I. (2019, May 29). HiddenWasp Malware Stings Targeted Linux Systems. Retrieved June 24, 2019.](https://www.intezer.com/blog-hiddenwasp-malware-targeting-linux-systems/)
-    [Chen, J. et al. (2021, February 3). Hildegard: New TeamTNT Cryptojacking Malware Targeting Kubernetes. Retrieved April 5, 2021.](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)
-    [Mandiant Intelligence. (2022, June 2). To HADES and Back: UNC2165 Shifts to LOCKBIT to Evade Sanctions. Retrieved July 29, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/unc2165-shifts-to-evade-sanctions/)
-    [KISA. (2021). Phishing Target Reconnaissance and Attack Resource Analysis Operation Muzabi. Retrieved March 8, 2024.](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)
-    [Symantec Security Response. (2018, July 25). Leafminer: New Espionage Campaigns Targeting Middle Eastern Regions. Retrieved August 28, 2018.](https://www.symantec.com/blogs/threat-intelligence/leafminer-espionage-middle-east)
-    [DFIR Report. (2022, March 21). APT35 Automates Initial Access Using ProxyShell. Retrieved May 25, 2022.](https://thedfirreport.com/2022/03/21/apt35-automates-initial-access-using-proxyshell)
-    [MSTIC. (2021, November 16). Evolving trends in Iranian threat actor activity – MSTIC presentation at CyberWarCon 2021. Retrieved January 12, 2023.](https://www.microsoft.com/en-us/security/blog/2021/11/16/evolving-trends-in-iranian-threat-actor-activity-mstic-presentation-at-cyberwarcon-2021)
-    [Gross, J. (2016, February 23). Operation Dust Storm. Retrieved December 22, 2021.](https://s7d2.scene7.com/is/content/cylance/prod/cylance-web/en-us/resources/knowledge-center/resource-library/reports/Op_Dust_Storm_Report.pdf)
-    [Savill, J. (1999, March 4). Net.exe reference. Retrieved September 22, 2015.](https://web.archive.org/web/20150511162820/http://windowsitpro.com/windows/netexe-reference)
-    [El-Banna, M., et al. (2025, November 17). Frontline Intelligence: Analysis of UNC1549 TTPs, Custom Tools, and Malware Targeting the Aerospace and Defense Ecosystem. Retrieved September 28, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)
-    [Nicolas Verdier. (n.d.). Retrieved January 29, 2018.](https://github.com/n1nj4sec/pupy)
-    [CISA. (2025, September 3). Countering Chinese State-Sponsored Actors Compromise of Networks Worldwide to Feed Global Espionage System. Retrieved September 22, 2026.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)
-    [Schwarz, D. and Proofpoint Staff. (2019, January 9). ServHelper and FlawedGrace - New malware introduced by TA505. Retrieved May 28, 2019.](https://www.proofpoint.com/us/threat-insight/post/servhelper-and-flawedgrace-new-malware-introduced-ta505)
-    [FireEye. (2021, June 16). Smoking Out a DARKSIDE Affiliate’s Supply Chain Software Compromise. Retrieved September 22, 2021.](https://www.fireeye.com/blog/threat-research/2021/06/darkside-affiliate-supply-chain-software-compromise.html)
-    [Fishbein, N. (2020, September 8). Attackers Abusing Legitimate Cloud Monitoring Tools to Conduct Cyber Attacks. Retrieved September 22, 2021.](https://www.intezer.com/blog/cloud-security/attackers-abusing-legitimate-cloud-monitoring-tools-to-conduct-cyber-attacks/)
-    [Shilko, J., et al. (2021, October 7). FIN12: The Prolific Ransomware Intrusion Threat Actor That Has Aggressively Pursued Healthcare Targets. Retrieved June 15, 2023.](https://web.archive.org/web/20220313061955/https://www.mandiant.com/sites/default/files/2021-10/fin12-group-profile.pdf)
-    [Allievi, A., et al. (2014, October 28). Threat Spotlight: Group 72, Opening the ZxShell. Retrieved September 24, 2019.](https://blogs.cisco.com/security/talos/opening-zxshell)

×
