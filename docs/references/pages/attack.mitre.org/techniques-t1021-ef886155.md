---
url: "https://attack.mitre.org/techniques/T1021/"
title: "Remote Services, Technique T1021 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:25:13Z"
via: "live"
sha256: "c8383e10010c1fc584edc41acd19f75e9c8d6d07113b939b1d32336efaae6e74"
wayback: "https://web.archive.org/web/20261102191248/https://attack.mitre.org/techniques/T1021/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Remote Services, Technique T1021 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1021/>
> Snapshot: <https://web.archive.org/web/20261102191248/https://attack.mitre.org/techniques/T1021/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Remote Services

#  Remote Services

##### Sub-techniques (8)

| ID  | Name

|  [T1021.001](https://attack.mitre.org/techniques/T1021/001/)   |  [Remote Desktop Protocol](https://attack.mitre.org/techniques/T1021/001/)

|  [T1021.002](https://attack.mitre.org/techniques/T1021/002/)   |  [SMB/Windows Admin Shares](https://attack.mitre.org/techniques/T1021/002/)

|  [T1021.003](https://attack.mitre.org/techniques/T1021/003/)   |  [Distributed Component Object Model](https://attack.mitre.org/techniques/T1021/003/)

|  [T1021.004](https://attack.mitre.org/techniques/T1021/004/)   |  [SSH](https://attack.mitre.org/techniques/T1021/004/)

|  [T1021.005](https://attack.mitre.org/techniques/T1021/005/)   |  [VNC](https://attack.mitre.org/techniques/T1021/005/)

|  [T1021.006](https://attack.mitre.org/techniques/T1021/006/)   |  [Windows Remote Management](https://attack.mitre.org/techniques/T1021/006/)

|  [T1021.007](https://attack.mitre.org/techniques/T1021/007/)   |  [Cloud Services](https://attack.mitre.org/techniques/T1021/007/)

|  [T1021.008](https://attack.mitre.org/techniques/T1021/008/)   |  [Direct Cloud VM Connections](https://attack.mitre.org/techniques/T1021/008/)

Adversaries may use [Valid Accounts](https://attack.mitre.org/techniques/T1078) to log into a service that accepts remote connections, such as telnet, SSH, and VNC. The adversary may then perform actions as the logged-on user.

In an enterprise environment, servers and workstations can be organized into domains. Domains provide centralized identity management, allowing users to login using one set of credentials across the entire network. If an adversary is able to obtain a set of valid domain credentials, they could login to many different machines using remote access protocols such as secure shell (SSH) or remote desktop protocol (RDP).[[1]](https://www.ssh.com/ssh)[[2]](https://technet.microsoft.com/en-us/windowsserver/ee236407.aspx) They could also login to accessible SaaS or IaaS services, such as those that federate their identities to the domain, or management platforms for internal virtualization environments such as VMware vCenter.

Legitimate applications (such as [Software Deployment Tools](https://attack.mitre.org/techniques/T1072) and other administrative programs) may utilize [Remote Services](https://attack.mitre.org/techniques/T1021) to access remote hosts. For example, Apple Remote Desktop (ARD) on macOS is native software used for remote management. ARD leverages a blend of protocols, including [VNC](https://attack.mitre.org/techniques/T1021/005) to send the screen and control buffers and [SSH](https://attack.mitre.org/techniques/T1021/004) for secure file transfer.[[3]](https://support.apple.com/en-us/HT209161)[[4]](https://support.apple.com/en-us/HT201710)[[5]](https://images.apple.com/remotedesktop/pdf/ARD_Admin_Guide_v3.3.pdf) Adversaries can abuse applications such as ARD to gain remote code execution and perform lateral movement. In versions of macOS prior to 10.14, an adversary can escalate an SSH session to an ARD session which enables an adversary to accept TCC (Transparency, Consent, and Control) prompts without user interaction and gain access to data.[[6]](https://www.fireeye.com/blog/threat-research/2019/10/leveraging-apple-remote-desktop-for-good-and-evil.html)[[7]](http://lockboxx.blogspot.com/2019/07/macos-red-teaming-206-ard-apple-remote.html)[[4]](https://support.apple.com/en-us/HT201710)

ID: T1021

Sub-techniques:  [T1021.001](https://attack.mitre.org/techniques/T1021/001), [T1021.002](https://attack.mitre.org/techniques/T1021/002), [T1021.003](https://attack.mitre.org/techniques/T1021/003), [T1021.004](https://attack.mitre.org/techniques/T1021/004), [T1021.005](https://attack.mitre.org/techniques/T1021/005), [T1021.006](https://attack.mitre.org/techniques/T1021/006), [T1021.007](https://attack.mitre.org/techniques/T1021/007), [T1021.008](https://attack.mitre.org/techniques/T1021/008)

ⓘ

Tactic: [Lateral Movement](https://attack.mitre.org/tactics/TA0008)

ⓘ

Platforms: ESXi, IaaS, Linux, Windows, macOS

Contributors: Dan Borges, @1njection

Version: 1.6

Created: 31 May 2017

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1021/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1021/)

## Procedure Examples

| ID  | Name  | Description

|  [C0064](https://attack.mitre.org/campaigns/C0064)   |  [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064)   |

During the [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064), the adversary tasked Claude Code with attempting to connect to other systems via FTP.[[8]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

|  [G0143](https://attack.mitre.org/groups/G0143)   |  [Aquatic Panda](https://attack.mitre.org/groups/G0143)   |

[Aquatic Panda](https://attack.mitre.org/groups/G0143) used remote scheduled tasks to install malicious software on victim systems during lateral movement actions.[[9]](https://go.crowdstrike.com/rs/281-OBQ-266/images/2022OverWatchThreatHuntingReport.pdf)

|  [S1063](https://attack.mitre.org/software/S1063)   |  [Brute Ratel C4](https://attack.mitre.org/software/S1063)   |

[Brute Ratel C4](https://attack.mitre.org/software/S1063) has the ability to use RPC for lateral movement.[[10]](https://unit42.paloaltonetworks.com/brute-ratel-c4-tool/)

|  [G1003](https://attack.mitre.org/groups/G1003)   |  [Ember Bear](https://attack.mitre.org/groups/G1003)   |

[Ember Bear](https://attack.mitre.org/groups/G1003) uses valid network credentials gathered through credential harvesting to move laterally within victim networks, often employing the [Impacket](https://attack.mitre.org/software/S0357) framework to do so.[[11]](https://www.microsoft.com/en-us/security/blog/2023/06/14/cadet-blizzard-emerges-as-a-novel-and-distinct-russian-threat-actor/)

|  [S0437](https://attack.mitre.org/software/S0437)   |  [Kivars](https://attack.mitre.org/software/S0437)   |

[Kivars](https://attack.mitre.org/software/S0437) has the ability to remotely trigger keyboard input and mouse clicks. [[12]](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)

|  [S1016](https://attack.mitre.org/software/S1016)   |  [MacMa](https://attack.mitre.org/software/S1016)   |

[MacMa](https://attack.mitre.org/software/S1016) can manage remote screen sessions.[[13]](https://www.welivesecurity.com/2022/01/25/watering-hole-deploys-new-macos-malware-dazzlespy-asia/)

|  [S0603](https://attack.mitre.org/software/S0603)   |  [Stuxnet](https://attack.mitre.org/software/S0603)   |

[Stuxnet](https://attack.mitre.org/software/S0603) can propagate via peer-to-peer communication and updates using RPC.[[14]](https://docs.broadcom.com/doc/security-response-w32-stuxnet-dossier-11-en)

|  [G0102](https://attack.mitre.org/groups/G0102)   |  [Wizard Spider](https://attack.mitre.org/groups/G0102)   |

[Wizard Spider](https://attack.mitre.org/groups/G0102) has used the WebDAV protocol to execute [Ryuk](https://attack.mitre.org/software/S0446) payloads hosted on network file shares.[[15]](https://web.archive.org/web/20220313061955/https://www.mandiant.com/sites/default/files/2021-10/fin12-group-profile.pdf)

## Mitigations

| ID  | Mitigation  | Description

|  [M1047](https://attack.mitre.org/mitigations/M1047)   |  [Audit](https://attack.mitre.org/mitigations/M1047)   |

Perform audits or scans of systems, permissions, insecure software, insecure configurations, etc. to identify potential weaknesses.

|  [M1042](https://attack.mitre.org/mitigations/M1042)   |  [Disable or Remove Feature or Program](https://attack.mitre.org/mitigations/M1042)   |

If remote services, such as the ability to make direct connections to cloud virtual machines, are not required, disable these connection types where feasible. On ESXi servers, consider enabling lockdown mode, which disables direct access to an ESXi host and requires that the host be managed remotely using vCenter.[[16]](https://cloud.google.com/blog/topics/threat-intelligence/vmware-detection-containment-hardening)[[17]](https://knowledge.broadcom.com/external/article/336894/enabling-or-disabling-lockdown-mode-on-a.html)

|  [M1035](https://attack.mitre.org/mitigations/M1035)   |  [Limit Access to Resource Over Network](https://attack.mitre.org/mitigations/M1035)   |

Prevent unnecessary remote access to file shares, hypervisors, sensitive systems, etc. Mechanisms to limit access may include use of network concentrators, RDP gateways, etc.[[18]](https://www.sygnia.co/blog/esxi-ransomware-attacks/)

|  [M1032](https://attack.mitre.org/mitigations/M1032)   |  [Multi-factor Authentication](https://attack.mitre.org/mitigations/M1032)   |

Use multi-factor authentication on remote service logons where possible.

|  [M1027](https://attack.mitre.org/mitigations/M1027)   |  [Password Policies](https://attack.mitre.org/mitigations/M1027)   |

Do not reuse local administrator account passwords across systems. Ensure password complexity and uniqueness such that the passwords cannot be cracked or guessed.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Limit the accounts that may use remote services. Limit the permissions for accounts that are at higher risk of compromise; for example, configure SSH so users can only run specific programs.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0269](https://attack.mitre.org/detectionstrategies/DET0269)   |  [Behavioral Detection Strategy for Remote Service Logins and Post-Access Activity](https://attack.mitre.org/detectionstrategies/DET0269)   |  [AN0750](https://attack.mitre.org/detectionstrategies/DET0269#AN0750)   |

Logon via RDP or WMI by a user account followed by uncommon command execution, file manipulation, or lateral network connections.

|  [AN0751](https://attack.mitre.org/detectionstrategies/DET0269#AN0751)   |

SSH session from new source IP followed by interactive shell or privilege escalation (e.g., sudo, su) and outbound lateral connection.

|  [AN0752](https://attack.mitre.org/detectionstrategies/DET0269#AN0752)   |

Remote login via ARD or SSH followed by screensharingd process activity or modification of TCC-protected files.

|  [AN0753](https://attack.mitre.org/detectionstrategies/DET0269#AN0753)   |

Use of cloud-based bastion or VM console session followed by commands that initiate outbound SSH or RDP sessions from the cloud instance to other environments.

|  [AN0754](https://attack.mitre.org/detectionstrategies/DET0269#AN0754)   |

vSphere API logins (vimService) or SSH to ESXi host followed by unauthorized shell commands or lateral remote logins from the ESXi host.

## References

-    [SSH.COM. (n.d.). SSH (Secure Shell). Retrieved March 23, 2020.](https://www.ssh.com/ssh)
-    [Microsoft. (n.d.). Remote Desktop Services. Retrieved June 1, 2016.](https://technet.microsoft.com/en-us/windowsserver/ee236407.aspx)
-    [Apple. (n.d.). Use MDM to enable Remote Management in macOS. Retrieved September 23, 2021.](https://support.apple.com/en-us/HT209161)
-    [Apple. (n.d.). Use the kickstart command-line utility in Apple Remote Desktop. Retrieved September 23, 2021.](https://support.apple.com/en-us/HT201710)
-    [Apple. (n.d.). Apple Remote Desktop Administrator Guide Version 3.3. Retrieved October 5, 2021.](https://images.apple.com/remotedesktop/pdf/ARD_Admin_Guide_v3.3.pdf)
-    [Jake Nicastro, Willi Ballenthin. (2019, October 9). Living off the Orchard: Leveraging Apple Remote Desktop for Good and Evil. Retrieved August 16, 2021.](https://www.fireeye.com/blog/threat-research/2019/10/leveraging-apple-remote-desktop-for-good-and-evil.html)
-    [Dan Borges. (2019, July 21). MacOS Red Teaming 206: ARD (Apple Remote Desktop Protocol). Retrieved September 10, 2021.](http://lockboxx.blogspot.com/2019/07/macos-red-teaming-206-ard-apple-remote.html)
-    [Eyal Sela. (2026, April). The AI-Assisted Breach of Mexico's Government Infrastructure. Retrieved September 2, 2026.](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)
-    [CrowdStrike. (2023). 2022 Falcon OverWatch Threat Hunting Report. Retrieved May 20, 2024.](https://go.crowdstrike.com/rs/281-OBQ-266/images/2022OverWatchThreatHuntingReport.pdf)

-    [Harbison, M. and Renals, P. (2022, July 5). When Pentest Tools Go Brutal: Red-Teaming Tool Being Abused by Malicious Actors. Retrieved February 1, 2023.](https://unit42.paloaltonetworks.com/brute-ratel-c4-tool/)
-    [Microsoft Threat Intelligence. (2023, June 14). Cadet Blizzard emerges as a novel and distinct Russian threat actor. Retrieved July 10, 2023.](https://www.microsoft.com/en-us/security/blog/2023/06/14/cadet-blizzard-emerges-as-a-novel-and-distinct-russian-threat-actor/)
-    [Bermejo, L., et al. (2017, June 22). Following the Trail of BlackTech’s Cyber Espionage Campaigns. Retrieved May 5, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)
-    [M.Léveillé, M., Cherepanov, A.. (2022, January 25). Watering hole deploys new macOS malware, DazzleSpy, in Asia. Retrieved May 6, 2022.](https://www.welivesecurity.com/2022/01/25/watering-hole-deploys-new-macos-malware-dazzlespy-asia/)
-    [Nicolas Falliere, Liam O Murchu, Eric Chien 2011, February W32.Stuxnet Dossier (Version 1.4) Retrieved November 17, 2024.](https://docs.broadcom.com/doc/security-response-w32-stuxnet-dossier-11-en)
-    [Shilko, J., et al. (2021, October 7). FIN12: The Prolific Ransomware Intrusion Threat Actor That Has Aggressively Pursued Healthcare Targets. Retrieved June 15, 2023.](https://web.archive.org/web/20220313061955/https://www.mandiant.com/sites/default/files/2021-10/fin12-group-profile.pdf)
-    [Alex Marvi, Greg Blaum, and Ron Craft. (2023, June 28). Detection, Containment, and Hardening Opportunities for Privileged Guest Operations, Anomalous Behavior, and VMCI Backdoors on Compromised VMware Hosts. Retrieved March 26, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/vmware-detection-containment-hardening)
-    [Broadcom. (2025, February 12). Enabling or disabling Lockdown mode on an ESXi host. Retrieved March 27, 2025.](https://knowledge.broadcom.com/external/article/336894/enabling-or-disabling-lockdown-mode-on-a.html)
-    [Nital Ruzin and Omer Kidron. (2024, May 15). ESXi Ransomware Attacks: Evolution, Impact, and Defense Strategy. Retrieved April 4, 2025.](https://www.sygnia.co/blog/esxi-ransomware-attacks/)

×
