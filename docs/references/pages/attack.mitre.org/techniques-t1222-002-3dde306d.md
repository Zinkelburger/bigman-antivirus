---
url: "https://attack.mitre.org/techniques/T1222/002/"
title: "File and Directory Permissions Modification: Linux and Mac Permissions, Sub-technique T1222.002 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:42:57Z"
via: "live"
sha256: "92de9963305d072ae4b32d6e24291907bc06e73fb8b01ec5e0c757ad4a550620"
wayback: "https://web.archive.org/web/20261023090819/https://attack.mitre.org/techniques/T1222/002/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# File and Directory Permissions Modification: Linux and Mac Permissions, Sub-technique T1222.002 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1222/002/>
> Snapshot: <https://web.archive.org/web/20261023090819/https://attack.mitre.org/techniques/T1222/002/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [File and Directory Permissions Modification](https://attack.mitre.org/techniques/T1222)
- Linux and Mac Permissions

#  File and Directory Permissions Modification: Linux and Mac Permissions

##### Other sub-techniques of File and Directory Permissions Modification (2)

| ID  | Name

|  [T1222.001](https://attack.mitre.org/techniques/T1222/001/)   |  [Windows Permissions](https://attack.mitre.org/techniques/T1222/001/)

|  T1222.002   |  Linux and Mac Permissions

Adversaries may modify file or directory permissions/attributes to evade access control lists (ACLs) and access protected files.[[1]](https://www.hybrid-analysis.com/sample/ef0d2628823e8e0a0de3b08b8eacaf41cf284c086a948bdfd67f4e4373c14e4d?environmentId=100)[[2]](https://www.hybrid-analysis.com/sample/22dab012c3e20e3d9291bce14a2bfc448036d3b966c6e78167f4626f5f9e38d6?environmentId=110) File and directory permissions are commonly managed by ACLs configured by the file or directory owner, or users with the appropriate permissions. File and directory ACL implementations vary by platform, but generally explicitly designate which users or groups can perform which actions (read, write, execute, etc.).

Most Linux and Linux-based platforms provide a standard set of permission groups (user, group, and other) and a standard set of permissions (read, write, and execute) that are applied to each group. While nuances of each platform’s permissions implementation may vary, most of the platforms provide two primary commands used to manipulate file and directory ACLs: chown (short for change owner), and chmod (short for change mode).

Adversarial may use these commands to make themselves the owner of files and directories or change the mode if current permissions allow it. They could subsequently lock others out of the file. Specific file and directory modifications may be a required step for many techniques, such as establishing Persistence via [Unix Shell Configuration Modification](https://attack.mitre.org/techniques/T1546/004) or tainting/hijacking other instrumental binary/configuration files via [Hijack Execution Flow](https://attack.mitre.org/techniques/T1574).[[3]](https://labs.sentinelone.com/20-common-tools-techniques-used-by-macos-threat-actors-malware/)

ID: T1222.002

Sub-technique of:  [T1222](https://attack.mitre.org/techniques/T1222)

ⓘ

Tactic: [Defense Impairment](https://attack.mitre.org/tactics/TA0112)

ⓘ

Platforms: Linux, macOS

Version: 2.0

Created: 04 February 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1222/002/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1222/002/)

## Procedure Examples

| ID  | Name  | Description

|  [G0050](https://attack.mitre.org/groups/G0050)   |  [APT32](https://attack.mitre.org/groups/G0050)   |

[APT32](https://attack.mitre.org/groups/G0050)'s macOS backdoor changes the permission of the file it wants to execute to 755.[[4]](https://www.welivesecurity.com/2019/04/09/oceanlotus-macos-malware-update/)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has changed file and directory permissions using chmod 777.[[5]](https://www.microsoft.com/en-us/security/blog/2026/04/16/dissecting-sapphire-sleets-macos-intrusion-from-lure-to-compromise/)

|  [S1070](https://attack.mitre.org/software/S1070)   |  [Black Basta](https://attack.mitre.org/software/S1070)   |

The [Black Basta](https://attack.mitre.org/software/S1070) binary can use chmod to gain full permissions to targeted files.[[6]](https://www.uptycs.com/blog/black-basta-ransomware-goes-cross-platform-now-targets-esxi-systems)

|  [S0482](https://attack.mitre.org/software/S0482)   |  [Bundlore](https://attack.mitre.org/software/S0482)   |

[Bundlore](https://attack.mitre.org/software/S0482) changes the permissions of a payload using the command chmod -R 755.[[3]](https://labs.sentinelone.com/20-common-tools-techniques-used-by-macos-threat-actors-malware/)

|  [S1105](https://attack.mitre.org/software/S1105)   |  [COATHANGER](https://attack.mitre.org/software/S1105)   |

[COATHANGER](https://attack.mitre.org/software/S1105) will set the GID of httpsd to 90 when infected.[[7]](https://www.ncsc.nl/binaries/ncsc/documenten/publicaties/2024/februari/6/mivd-aivd-advisory-coathanger-tlp-clear/TLP-CLEAR+MIVD+AIVD+Advisory+COATHANGER.pdf)

|  [S0281](https://attack.mitre.org/software/S0281)   |  [Dok](https://attack.mitre.org/software/S0281)   |

[Dok](https://attack.mitre.org/software/S0281) gives all users execute permissions for the application using the command chmod +x /Users/Shared/AppStore.app.[[8]](https://blog.checkpoint.com/2017/04/27/osx-malware-catching-wants-read-https-traffic/)

|  [S9013](https://attack.mitre.org/software/S9013)   |  [DRYHOOK](https://attack.mitre.org/software/S9013)   |

[DRYHOOK](https://attack.mitre.org/software/S9013) has the ability to remount the filesystem as "read-write" to make changes and then restores it to "read-only" prior to killing processes to apply the modifications.[[9]](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-connect-secure-vpn-zero-day)[[10]](https://www.picussecurity.com/resource/blog/unc5221-cve-2025-22457-ivanti-connect-secure)

|  [S0599](https://attack.mitre.org/software/S0599)   |  [Kinsing](https://attack.mitre.org/software/S0599)   |

[Kinsing](https://attack.mitre.org/software/S0599) has used chmod to modify permissions on key files for use.[[11]](https://blog.aquasec.com/threat-alert-kinsing-malware-container-vulnerability)

|  [C0035](https://attack.mitre.org/campaigns/C0035)   |  [KV Botnet Activity](https://attack.mitre.org/campaigns/C0035)   |

[KV Botnet Activity](https://attack.mitre.org/campaigns/C0035) altered permissions on downloaded tools and payloads to enable execution on victim machines.[[12]](https://blog.lumen.com/routers-roasting-on-an-open-firewall-the-kv-botnet-investigation/)

|  [S0402](https://attack.mitre.org/software/S0402)   |  [OSX/Shlayer](https://attack.mitre.org/software/S0402)   |

[OSX/Shlayer](https://attack.mitre.org/software/S0402) can use the chmod utility to set a file as executable, such as chmod 777 or chmod +x.[[3]](https://labs.sentinelone.com/20-common-tools-techniques-used-by-macos-threat-actors-malware/)[[13]](https://blogs.vmware.com/security/2020/02/vmware-carbon-black-tau-threat-analysis-shlayer-macos.html)[[14]](https://www.jamf.com/blog/shlayer-malware-abusing-gatekeeper-bypass-on-macos/)

|  [S0352](https://attack.mitre.org/software/S0352)   |  [OSX_OCEANLOTUS.D](https://attack.mitre.org/software/S0352)   |

[OSX_OCEANLOTUS.D](https://attack.mitre.org/software/S0352) has changed permissions of a second-stage payload to an executable via chmod.[[15]](https://www.sentinelone.com/labs/apt32-multi-stage-macos-trojan-innovates-on-crimeware-scripting-technique/)

|  [S0598](https://attack.mitre.org/software/S0598)   |  [P.A.S. Webshell](https://attack.mitre.org/software/S0598)   |

[P.A.S. Webshell](https://attack.mitre.org/software/S0598) has the ability to modify file permissions.[[16]](https://www.cert.ssi.gouv.fr/uploads/CERTFR-2021-CTI-005.pdf)

|  [S0587](https://attack.mitre.org/software/S0587)   |  [Penquin](https://attack.mitre.org/software/S0587)   |

[Penquin](https://attack.mitre.org/software/S0587) can add the executable flag to a downloaded file.[[17]](https://www.leonardo.com/documents/20142/10868623/Malware+Technical+Insight+_Turla+%E2%80%9CPenquin_x64%E2%80%9D.pdf)

|  [G0106](https://attack.mitre.org/groups/G0106)   |  [Rocke](https://attack.mitre.org/groups/G0106)   |

[Rocke](https://attack.mitre.org/groups/G0106) has changed file permissions of files so they could not be modified.[[18]](https://www.anomali.com/blog/rocke-evolves-its-arsenal-with-a-new-malware-family-written-in-golang)

|  [G0139](https://attack.mitre.org/groups/G0139)   |  [TeamTNT](https://attack.mitre.org/groups/G0139)   |

[TeamTNT](https://attack.mitre.org/groups/G0139) has modified the permissions on binaries with chattr.[[19]](https://documents.trendmicro.com/assets/white_papers/wp-tracking-the-activities-of-teamTNT.pdf)[[20]](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/)

|  [S0658](https://attack.mitre.org/software/S0658)   |  [XCSSET](https://attack.mitre.org/software/S0658)   |

[XCSSET](https://attack.mitre.org/software/S0658) uses the chmod +x command to grant executable permissions to the malicious file.[[3]](https://labs.sentinelone.com/20-common-tools-techniques-used-by-macos-threat-actors-malware/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1026](https://attack.mitre.org/mitigations/M1026)   |  [Privileged Account Management](https://attack.mitre.org/mitigations/M1026)   |

Ensure critical system files as well as those known to be abused by adversaries have restrictive permissions and are owned by an appropriately privileged account, especially if access is not required by users nor will inhibit system functionality.

|  [M1022](https://attack.mitre.org/mitigations/M1022)   |  [Restrict File and Directory Permissions](https://attack.mitre.org/mitigations/M1022)   |

Applying more restrictive permissions to files and directories could prevent adversaries from modifying the access control lists.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0351](https://attack.mitre.org/detectionstrategies/DET0351)   |  [Unix-like File Permission Manipulation Behavioral Chain Detection Strategy](https://attack.mitre.org/detectionstrategies/DET0351)   |  [AN0998](https://attack.mitre.org/detectionstrategies/DET0351#AN0998)   |

Linux permission escalation behavioral chain: (1) Process creation of permission modification utilities (chmod, chown, chgrp, setfacl) with suspicious parameters indicating privilege escalation intent, (2) System call analysis revealing direct file metadata manipulation (chmod, fchmod, chown, fchown syscalls), (3) Extended attribute and ACL modifications targeting critical system paths, (4) Temporal correlation with subsequent file access or process execution from modified locations, (5) Anomalous permission patterns deviating from system baselines

|  [AN0999](https://attack.mitre.org/detectionstrategies/DET0351#AN0999)   |

macOS permission and attribute manipulation behavioral chain: (1) Process execution of permission utilities (chmod, chown, chgrp) or macOS-specific tools (chflags) with suspicious parameters, (2) System Integrity Protection (SIP) bypass attempts through permission modifications, (3) File flags manipulation (uchg, schg, hidden) for evasion or persistence, (4) Extended attribute (xattr) modifications affecting security metadata, (5) Unified log correlation with file system events and subsequent access patterns, (6) Gatekeeper and code signing bypass through permission/attribute manipulation

## References

-    [Hybrid Analysis. (2018, June 12). c9b65b764985dfd7a11d3faf599c56b8.exe. Retrieved August 19, 2018.](https://www.hybrid-analysis.com/sample/ef0d2628823e8e0a0de3b08b8eacaf41cf284c086a948bdfd67f4e4373c14e4d?environmentId=100)
-    [Hybrid Analysis. (2018, May 30). 2a8efbfadd798f6111340f7c1c956bee.dll. Retrieved August 19, 2018.](https://www.hybrid-analysis.com/sample/22dab012c3e20e3d9291bce14a2bfc448036d3b966c6e78167f4626f5f9e38d6?environmentId=110)
-    [Phil Stokes. (2021, February 16). 20 Common Tools & Techniques Used by macOS Threat Actors & Malware. Retrieved August 23, 2021.](https://labs.sentinelone.com/20-common-tools-techniques-used-by-macos-threat-actors-malware/)
-    [Dumont, R.. (2019, April 9). OceanLotus: macOS malware update. Retrieved April 15, 2019.](https://www.welivesecurity.com/2019/04/09/oceanlotus-macos-malware-update/)
-    [Microsoft Threat Intelligence. (2026, April 16). Dissecting Sapphire Sleet’s macOS intrusion from lure to compromise. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/04/16/dissecting-sapphire-sleets-macos-intrusion-from-lure-to-compromise/)
-    [Sharma, S. and Hegde, N. (2022, June 7). Black basta Ransomware Goes Cross-Platform, Now Targets ESXi Systems. Retrieved March 8, 2023.](https://www.uptycs.com/blog/black-basta-ransomware-goes-cross-platform-now-targets-esxi-systems)
-    [Dutch Military Intelligence and Security Service (MIVD) & Dutch General Intelligence and Security Service (AIVD). (2024, February 6). Ministry of Defense of the Netherlands uncovers COATHANGER, a stealthy Chinese FortiGate RAT. Retrieved February 7, 2024.](https://www.ncsc.nl/binaries/ncsc/documenten/publicaties/2024/februari/6/mivd-aivd-advisory-coathanger-tlp-clear/TLP-CLEAR+MIVD+AIVD+Advisory+COATHANGER.pdf)
-    [Ofer Caspi. (2017, May 4). OSX Malware is Catching Up, and it wants to Read Your HTTPS Traffic. Retrieved October 5, 2021.](https://blog.checkpoint.com/2017/04/27/osx-malware-catching-wants-read-https-traffic/)
-    [John Wolfram, Josh Murchie, Matt Lin, Daniel Ainsworth, Robert Wallace, Dimiter Andonov, Dhanesh Kizhakkinan, Jacob Thompson. (2025, January 8). Ivanti Connect Secure VPN Targeted in New Zero-Day Exploitation. Retrieved April 14, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-connect-secure-vpn-zero-day)
-    [Sila Ozeren Hacioglu. (2025, May 5). UNC5221’s Latest Exploit: Weaponizing CVE-2025-22457 in Ivanti Connect Secure. Retrieved April 13, 2026.](https://www.picussecurity.com/resource/blog/unc5221-cve-2025-22457-ivanti-connect-secure)

-    [Singer, G. (2020, April 3). Threat Alert: Kinsing Malware Attacks Targeting Container Environments. Retrieved April 1, 2021.](https://blog.aquasec.com/threat-alert-kinsing-malware-container-vulnerability)
-    [Black Lotus Labs. (2023, December 13). Routers Roasting On An Open Firewall: The KV-Botnet Investigation. Retrieved June 10, 2024.](https://blog.lumen.com/routers-roasting-on-an-open-firewall-the-kv-botnet-investigation/)
-    [Carbon Black Threat Analysis Unit. (2019, February 12). New macOS Malware Variant of Shlayer (OSX) Discovered. Retrieved August 8, 2019.](https://blogs.vmware.com/security/2020/02/vmware-carbon-black-tau-threat-analysis-shlayer-macos.html)
-    [Jaron Bradley. (2021, April 26). Shlayer malware abusing Gatekeeper bypass on macOS. Retrieved September 22, 2021.](https://www.jamf.com/blog/shlayer-malware-abusing-gatekeeper-bypass-on-macos/)
-    [Phil Stokes. (2020, December 2). APT32 Multi-stage macOS Trojan Innovates on Crimeware Scripting Technique. Retrieved September 13, 2021.](https://www.sentinelone.com/labs/apt32-multi-stage-macos-trojan-innovates-on-crimeware-scripting-technique/)
-    [ANSSI. (2021, January 27). SANDWORM INTRUSION SET CAMPAIGN TARGETING CENTREON SYSTEMS. Retrieved March 30, 2021.](https://www.cert.ssi.gouv.fr/uploads/CERTFR-2021-CTI-005.pdf)
-    [Leonardo. (2020, May 29). MALWARE TECHNICAL INSIGHT TURLA “Penquin_x64”. Retrieved March 11, 2021.](https://www.leonardo.com/documents/20142/10868623/Malware+Technical+Insight+_Turla+%E2%80%9CPenquin_x64%E2%80%9D.pdf)
-    [Anomali Labs. (2019, March 15). Rocke Evolves Its Arsenal With a New Malware Family Written in Golang. Retrieved April 24, 2019.](https://www.anomali.com/blog/rocke-evolves-its-arsenal-with-a-new-malware-family-written-in-golang)
-    [Fiser, D. Oliveira, A. (n.d.). Tracking the Activities of TeamTNT A Closer Look at a Cloud-Focused Malicious Actor Group. Retrieved September 22, 2021.](https://documents.trendmicro.com/assets/white_papers/wp-tracking-the-activities-of-teamTNT.pdf)
-    [Darin Smith. (2022, April 21). TeamTNT targeting AWS, Alibaba. Retrieved August 4, 2022.](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/)

×
