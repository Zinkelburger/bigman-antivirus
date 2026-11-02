---
url: "https://attack.mitre.org/techniques/T1037/004/"
title: "Boot or Logon Initialization Scripts: RC Scripts, Sub-technique T1037.004 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:27:40Z"
via: "live"
sha256: "e8ac4193764cbab934610bb3194e33129c41221903ea01883e90799c197d804a"
wayback: "https://web.archive.org/web/20261023090759/https://attack.mitre.org/techniques/T1037/004/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
  - "docs/threats/05-malware-families-and-detection.md"
---

# Boot or Logon Initialization Scripts: RC Scripts, Sub-technique T1037.004 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1037/004/>
> Snapshot: <https://web.archive.org/web/20261023090759/https://attack.mitre.org/techniques/T1037/004/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Boot or Logon Initialization Scripts](https://attack.mitre.org/techniques/T1037)
- RC Scripts

#  Boot or Logon Initialization Scripts: RC Scripts

##### Other sub-techniques of Boot or Logon Initialization Scripts (5)

| ID  | Name

|  [T1037.001](https://attack.mitre.org/techniques/T1037/001/)   |  [Logon Script (Windows)](https://attack.mitre.org/techniques/T1037/001/)

|  [T1037.002](https://attack.mitre.org/techniques/T1037/002/)   |  [Login Hook](https://attack.mitre.org/techniques/T1037/002/)

|  [T1037.003](https://attack.mitre.org/techniques/T1037/003/)   |  [Network Logon Script](https://attack.mitre.org/techniques/T1037/003/)

|  T1037.004   |  RC Scripts

|  [T1037.005](https://attack.mitre.org/techniques/T1037/005/)   |  [Startup Items](https://attack.mitre.org/techniques/T1037/005/)

Adversaries may establish persistence by modifying RC scripts, which are executed during a Unix-like system’s startup. These files allow system administrators to map and start custom services at startup for different run levels. RC scripts require root privileges to modify.

Adversaries may establish persistence by adding a malicious binary path or shell commands to rc.local, rc.common, and other RC scripts specific to the Unix-like distribution.[[1]](https://iranthreats.github.io/resources/attribution-flying-rocket-kitten/)[[2]](https://www.intezer.com/blog-hiddenwasp-malware-targeting-linux-systems/) Upon reboot, the system executes the script's contents as root, resulting in persistence.

Adversary abuse of RC scripts is especially effective for lightweight Unix-like distributions using the root user as default, such as ESXi hypervisors, IoT, or embedded systems.[[3]](https://www.intezer.com/blog/research/kaiji-new-chinese-linux-malware-turning-to-golang/) As ESXi servers store most system files in memory and therefore discard changes on shutdown, leveraging /etc/rc.local.d/local.sh is one of the few mechanisms for enabling persistence across reboots.[[4]](https://blogs.juniper.net/en-us/threat-research/a-custom-python-backdoor-for-vmware-esxi-servers)

Several Unix-like systems have moved to Systemd and deprecated the use of RC scripts. This is now a deprecated mechanism in macOS in favor of Launchd.[[5]](https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/CreatingLaunchdJobs.html)[[6]](https://developer.apple.com/library/content/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/StartupItems.html) This technique can be used on Mac OS X Panther v10.3 and earlier versions which still execute the RC scripts.[[7]](https://www.virusbulletin.com/uploads/pdf/conference/vb2014/VB2014-Wardle.pdf) To maintain backwards compatibility some systems, such as Ubuntu, will execute the RC scripts if they exist with the correct file permissions.[[8]](http://manpages.ubuntu.com/manpages/bionic/man8/systemd-rc-local-generator.8.html)

ID: T1037.004

Sub-technique of:  [T1037](https://attack.mitre.org/techniques/T1037)

ⓘ

Tactics: [Persistence](https://attack.mitre.org/tactics/TA0003), [Privilege Escalation](https://attack.mitre.org/tactics/TA0004)

ⓘ

Platforms: ESXi, Linux, Network Devices, macOS

Version: 2.2

Created: 15 January 2020

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1037/004/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1037/004/)

## Procedure Examples

| ID  | Name  | Description

|  [G0016](https://attack.mitre.org/groups/G0016)   |  [APT29](https://attack.mitre.org/groups/G0016)   |

[APT29](https://attack.mitre.org/groups/G0016) has installed a run command on a compromised system to enable malware execution on system startup.[[9]](https://www.mandiant.com/resources/blog/unc3524-eye-spy-email)

|  [S0687](https://attack.mitre.org/software/S0687)   |  [Cyclops Blink](https://attack.mitre.org/software/S0687)   |

[Cyclops Blink](https://attack.mitre.org/software/S0687) has the ability to execute on device startup, using a modified RC script named S51armled.[[10]](https://www.ncsc.gov.uk/files/Cyclops-Blink-Malware-Analysis-Report.pdf)

|  [S0690](https://attack.mitre.org/software/S0690)   |  [Green Lambert](https://attack.mitre.org/software/S0690)   |

[Green Lambert](https://attack.mitre.org/software/S0690) can add init.d and rc.d files in the /etc folder to establish persistence.[[11]](https://objective-see.com/blog/blog_0x68.html)[[12]](https://web.archive.org/web/20211018145402/https://www.glitch-cat.com/blog/green-lambert-and-attack)

|  [S0394](https://attack.mitre.org/software/S0394)   |  [HiddenWasp](https://attack.mitre.org/software/S0394)   |

[HiddenWasp](https://attack.mitre.org/software/S0394) installs reboot persistence by adding itself to /etc/rc.local.[[2]](https://www.intezer.com/blog-hiddenwasp-malware-targeting-linux-systems/)

|  [S0278](https://attack.mitre.org/software/S0278)   |  [iKitten](https://attack.mitre.org/software/S0278)   |

[iKitten](https://attack.mitre.org/software/S0278) adds an entry to the rc.common file for persistence.[[13]](https://objective-see.com/blog/blog_0x25.html)

|  [S9046](https://attack.mitre.org/software/S9046)   |  [IOControl](https://attack.mitre.org/software/S9046)   |

IOCONTROL has used an RC script (/etc/rc3.d/S93InitSystemd.sh) to establish persistence on a system across restarts.[[14]](https://claroty.com/team82/research/inside-a-new-ot-iot-cyber-weapon-iocontrol)[[15]](https://www.armis.com/blog/iocontrol-malware-whats-new-whats-not/)

|  [S9051](https://attack.mitre.org/software/S9051)   |  [The Gentlemen Ransomware](https://attack.mitre.org/software/S9051)   |

[The Gentlemen Ransomware](https://attack.mitre.org/software/S9051) has utilized RC scripts to establish persistence in ESXi service.[[16]](https://research.checkpoint.com/2026/dfir-report-the-gentlemen/)

|  [G1048](https://attack.mitre.org/groups/G1048)   |  [UNC3886](https://attack.mitre.org/groups/G1048)   |

[UNC3886](https://attack.mitre.org/groups/G1048) has placed a bash installation script into /etc/rc.local.d/ to establish persistence.[[17]](https://cloud.google.com/blog/topics/threat-intelligence/esxi-hypervisors-malware-persistence)

|  [G1047](https://attack.mitre.org/groups/G1047)   |  [Velvet Ant](https://attack.mitre.org/groups/G1047)   |

[Velvet Ant](https://attack.mitre.org/groups/G1047) used a modified /etc/rc.local file on compromised F5 BIG-IP devices to maintain persistence.[[18]](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1022](https://attack.mitre.org/mitigations/M1022)   |  [Restrict File and Directory Permissions](https://attack.mitre.org/mitigations/M1022)   |

Limit privileges of user accounts so only authorized users can edit the rc.common file.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0237](https://attack.mitre.org/detectionstrategies/DET0237)   |  [Detection Strategy for Boot or Logon Initialization Scripts: RC Scripts](https://attack.mitre.org/detectionstrategies/DET0237)   |  [AN0658](https://attack.mitre.org/detectionstrategies/DET0237#AN0658)   |

Detection of modified or newly created /etc/rc.local or /etc/init.d scripts followed by suspicious execution during system startup.

|  [AN0659](https://attack.mitre.org/detectionstrategies/DET0237#AN0659)   |

Detection of edits or additions to /etc/rc.common, /Library/StartupItems, or /System/Library/StartupItems and associated script execution during login or reboot.

|  [AN0660](https://attack.mitre.org/detectionstrategies/DET0237#AN0660)   |

Detection of changes to /etc/rc.local.d/local.sh or rc.local during post-boot script execution with abnormal commands or additions.

|  [AN0661](https://attack.mitre.org/detectionstrategies/DET0237#AN0661)   |

Detection of modified boot-time configuration scripts that persist malicious CLI commands across reboots.

## References

-    [Iran Threats . (2017, December 5). Flying Kitten to Rocket Kitten, A Case of Ambiguity and Shared Code. Retrieved May 28, 2020.](https://iranthreats.github.io/resources/attribution-flying-rocket-kitten/)
-    [Sanmillan, I. (2019, May 29). HiddenWasp Malware Stings Targeted Linux Systems. Retrieved June 24, 2019.](https://www.intezer.com/blog-hiddenwasp-malware-targeting-linux-systems/)
-    [Paul Litvak. (2020, May 4). Kaiji: New Chinese Linux malware turning to Golang. Retrieved December 17, 2020.](https://www.intezer.com/blog/research/kaiji-new-chinese-linux-malware-turning-to-golang/)
-    [Asher Langton. (2022, December 9). A Custom Python Backdoor for VMWare ESXi Servers. Retrieved March 26, 2025.](https://blogs.juniper.net/en-us/threat-research/a-custom-python-backdoor-for-vmware-esxi-servers)
-    [Apple. (2016, September 13). Daemons and Services Programming Guide - Creating Launch Daemons and Agents. Retrieved February 24, 2021.](https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/CreatingLaunchdJobs.html)
-    [Apple. (2016, September 13). Startup Items. Retrieved July 11, 2017.](https://developer.apple.com/library/content/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/StartupItems.html)
-    [Patrick Wardle. (2014, September). Methods of Malware Persistence on Mac OS X. Retrieved July 5, 2017.](https://www.virusbulletin.com/uploads/pdf/conference/vb2014/VB2014-Wardle.pdf)
-    [Canonical Ltd.. (n.d.). systemd-rc-local-generator - Compatibility generator for starting /etc/rc.local and /usr/sbin/halt.local during boot and shutdown. Retrieved February 23, 2021.](http://manpages.ubuntu.com/manpages/bionic/man8/systemd-rc-local-generator.8.html)
-    [Mandiant. (2022, May 2). UNC3524: Eye Spy on Your Email. Retrieved August 17, 2023.](https://www.mandiant.com/resources/blog/unc3524-eye-spy-email)

-    [NCSC. (2022, February 23). Cyclops Blink Malware Analysis Report. Retrieved March 3, 2022.](https://www.ncsc.gov.uk/files/Cyclops-Blink-Malware-Analysis-Report.pdf)
-    [Sandvik, Runa. (2021, October 1). Made In America: Green Lambert for OS X. Retrieved March 21, 2022.](https://objective-see.com/blog/blog_0x68.html)
-    [Sandvik, Runa. (2021, October 18). Green Lambert and ATT&CK. Retrieved November 17, 2024.](https://web.archive.org/web/20211018145402/https://www.glitch-cat.com/blog/green-lambert-and-attack)
-    [Patrick Wardle. (n.d.). Mac Malware of 2017. Retrieved September 21, 2018.](https://objective-see.com/blog/blog_0x25.html)
-    [Claroty Team82. (2024, December 10). Inside a New OT/IoT Cyberweapon: IOCONTROL. Retrieved September 2, 2026.](https://claroty.com/team82/research/inside-a-new-ot-iot-cyber-weapon-iocontrol)
-    [Michael Freeman. (2024, December 16). IOControl Malware: What's New, What's Not?. Retrieved September 2, 2026.](https://www.armis.com/blog/iocontrol-malware-whats-new-whats-not/)
-    [Check Point Research. (2026, April 20). DFIR Report – The Gentlemen & SystemBC: A Sneak Peek Behind the Proxy. Retrieved September 18, 2026.](https://research.checkpoint.com/2026/dfir-report-the-gentlemen/)
-    [Alexander Marvi, Jeremy Koppen, Tufail Ahmed, and Jonathan Lepore. (2022, September 29). Bad VIB(E)s Part One: Investigating Novel Malware Persistence Within ESXi Hypervisors. Retrieved March 26, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/esxi-hypervisors-malware-persistence)
-    [Sygnia Team. (2024, June 3). China-Nexus Threat Group ‘Velvet Ant’ Abuses F5 Load Balancers for Persistence. Retrieved March 14, 2025.](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/)

×
