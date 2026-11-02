---
url: "https://attack.mitre.org/techniques/T1553/006/"
title: "Subvert Trust Controls: Code Signing Policy Modification, Sub-technique T1553.006 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:53:12Z"
via: "live"
sha256: "183079886755cbcacfed66e2277f09adbd9654aea16654e45c639f6db8edc535"
wayback: "https://web.archive.org/web/20261015113934/https://attack.mitre.org/techniques/T1553/006/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Subvert Trust Controls: Code Signing Policy Modification, Sub-technique T1553.006 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1553/006/>
> Snapshot: <https://web.archive.org/web/20261015113934/https://attack.mitre.org/techniques/T1553/006/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Subvert Trust Controls](https://attack.mitre.org/techniques/T1553)
- Code Signing Policy Modification

#  Subvert Trust Controls: Code Signing Policy Modification

##### Other sub-techniques of Subvert Trust Controls (6)

| ID  | Name

|  [T1553.001](https://attack.mitre.org/techniques/T1553/001/)   |  [Gatekeeper Bypass](https://attack.mitre.org/techniques/T1553/001/)

|  [T1553.002](https://attack.mitre.org/techniques/T1553/002/)   |  [Code Signing](https://attack.mitre.org/techniques/T1553/002/)

|  [T1553.003](https://attack.mitre.org/techniques/T1553/003/)   |  [SIP and Trust Provider Hijacking](https://attack.mitre.org/techniques/T1553/003/)

|  [T1553.004](https://attack.mitre.org/techniques/T1553/004/)   |  [Install Root Certificate](https://attack.mitre.org/techniques/T1553/004/)

|  [T1553.005](https://attack.mitre.org/techniques/T1553/005/)   |  [Mark-of-the-Web Bypass](https://attack.mitre.org/techniques/T1553/005/)

|  T1553.006   |  Code Signing Policy Modification

Adversaries may modify code signing policies to enable execution of unsigned or self-signed code. Code signing provides a level of authenticity on a program from a developer and a guarantee that the program has not been tampered with. Security controls can include enforcement mechanisms to ensure that only valid, signed code can be run on an operating system.

Some of these security controls may be enabled by default, such as Driver Signature Enforcement (DSE) on Windows or System Integrity Protection (SIP) on macOS.[[1]](https://docs.microsoft.com/en-us/previous-versions/windows/hardware/design/dn653559(v=vs.85)?redirectedfrom=MSDN)[[2]](https://developer.apple.com/documentation/security/disabling_and_enabling_system_integrity_protection) Other such controls may be disabled by default but are configurable through application controls, such as only allowing signed Dynamic-Link Libraries (DLLs) to execute on a system. Since it can be useful for developers to modify default signature enforcement policies during the development and testing of applications, disabling of these features may be possible with elevated permissions.[[3]](https://docs.microsoft.com/en-us/windows-hardware/drivers/install/installing-an-unsigned-driver-during-development-and-test)[[2]](https://developer.apple.com/documentation/security/disabling_and_enabling_system_integrity_protection)

Adversaries may modify code signing policies in a number of ways, including through use of command-line or GUI utilities, [Modify Registry](https://attack.mitre.org/techniques/T1112), rebooting the computer in a debug/recovery mode, or by altering the value of variables in kernel memory.[[4]](https://docs.microsoft.com/en-us/windows-hardware/drivers/install/the-testsigning-boot-configuration-option)[[2]](https://developer.apple.com/documentation/security/disabling_and_enabling_system_integrity_protection)[[5]](https://web.archive.org/web/20210920172620/https://www.fireeye.com/blog/threat-research/2012/08/hikit-rootkit-advanced-persistent-attack-techniques-part-2.html)[[6]](https://github.com/hfiref0x/TDL) Examples of commands that can modify the code signing policy of a system include bcdedit.exe -set TESTSIGNING ON on Windows and csrutil disable on macOS.[[4]](https://docs.microsoft.com/en-us/windows-hardware/drivers/install/the-testsigning-boot-configuration-option)[[2]](https://developer.apple.com/documentation/security/disabling_and_enabling_system_integrity_protection) Depending on the implementation, successful modification of a signing policy may require reboot of the compromised system. Additionally, some implementations can introduce visible artifacts for the user (ex: a watermark in the corner of the screen stating the system is in Test Mode). Adversaries may attempt to remove such artifacts.[[7]](https://blog-assets.f-secure.com/wp-content/uploads/2019/10/15163408/BlackEnergy_Quedagh.pdf)

To gain access to kernel memory to modify variables related to signature checks, such as modifying g_CiOptions to disable Driver Signature Enforcement, adversaries may conduct [Exploitation for Privilege Escalation](https://attack.mitre.org/techniques/T1068) using a signed, but vulnerable driver.[[8]](https://unit42.paloaltonetworks.com/acidbox-rare-malware/)[[6]](https://github.com/hfiref0x/TDL)

ID: T1553.006

Sub-technique of:  [T1553](https://attack.mitre.org/techniques/T1553)

ⓘ

Tactic: [Defense Impairment](https://attack.mitre.org/tactics/TA0112)

ⓘ

Platforms: Windows, macOS

Contributors: Abel Morales, Exabeam

Version: 2.0

Created: 23 April 2021

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1553/006/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1553/006/)

## Procedure Examples

| ID  | Name  | Description

|  [G0087](https://attack.mitre.org/groups/G0087)   |  [APT39](https://attack.mitre.org/groups/G0087)   |

[APT39](https://attack.mitre.org/groups/G0087) has used malware to turn off the RequireSigned feature which ensures only signed DLLs can be run on Windows.[[9]](https://www.iranwatch.org/sites/default/files/public-intelligence-alert.pdf)

|  [S0089](https://attack.mitre.org/software/S0089)   |  [BlackEnergy](https://attack.mitre.org/software/S0089)   |

[BlackEnergy](https://attack.mitre.org/software/S0089) has enabled the TESTSIGNING boot configuration option to facilitate loading of a driver component.[[7]](https://blog-assets.f-secure.com/wp-content/uploads/2019/10/15163408/BlackEnergy_Quedagh.pdf)

|  [S0009](https://attack.mitre.org/software/S0009)   |  [Hikit](https://attack.mitre.org/software/S0009)   |

[Hikit](https://attack.mitre.org/software/S0009) has attempted to disable driver signing verification by tampering with several Registry keys prior to the loading of a rootkit driver component.[[5]](https://web.archive.org/web/20210920172620/https://www.fireeye.com/blog/threat-research/2012/08/hikit-rootkit-advanced-persistent-attack-techniques-part-2.html)

|  [S0664](https://attack.mitre.org/software/S0664)   |  [Pandora](https://attack.mitre.org/software/S0664)   |

[Pandora](https://attack.mitre.org/software/S0664) can use CVE-2017-15303 to disable Windows Driver Signature Enforcement (DSE) protection and load its driver.[[10]](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)

|  [G0010](https://attack.mitre.org/groups/G0010)   |  [Turla](https://attack.mitre.org/groups/G0010)   |

[Turla](https://attack.mitre.org/groups/G0010) has modified variables in kernel memory to turn off Driver Signature Enforcement after exploiting vulnerabilities that obtained kernel mode privileges.[[8]](https://unit42.paloaltonetworks.com/acidbox-rare-malware/)[[6]](https://github.com/hfiref0x/TDL)

## Mitigations

| ID  | Mitigation  | Description

|  [M1046](https://attack.mitre.org/mitigations/M1046)   |  [Boot Integrity](https://attack.mitre.org/mitigations/M1046)   |

Use of Secure Boot may prevent some implementations of modification to code signing policies.[[4]](https://docs.microsoft.com/en-us/windows-hardware/drivers/install/the-testsigning-boot-configuration-option)

|  [M1026](https://attack.mitre.org/mitigations/M1026)   |  [Privileged Account Management](https://attack.mitre.org/mitigations/M1026)   |

Limit the usage of local administrator and domain administrator accounts to be used for day-to-day operations that may expose them to potential adversaries.

|  [M1024](https://attack.mitre.org/mitigations/M1024)   |  [Restrict Registry Permissions](https://attack.mitre.org/mitigations/M1024)   |

Ensure proper permissions are set for the Registry to prevent users from modifying keys related to code signing policies.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0523](https://attack.mitre.org/detectionstrategies/DET0523)   |  [Detect Code Signing Policy Modification (Windows & macOS)](https://attack.mitre.org/detectionstrategies/DET0523)   |  [AN1446](https://attack.mitre.org/detectionstrategies/DET0523#AN1446)   |

Monitors execution of administrative utilities (e.g., bcdedit.exe) or registry modifications that disable Driver Signature Enforcement (DSE) or enable Test Signing. Correlates command-line activity, registry changes, and subsequent process executions that bypass signing enforcement.

|  [AN1447](https://attack.mitre.org/detectionstrategies/DET0523#AN1447)   |

Detects modification of System Integrity Protection (SIP) or code signing enforcement policies through csrutil or kernel variable tampering. Correlates execution of csrutil disable commands with subsequent policy state changes and anomalous unsigned process executions.

## References

-    [Microsoft. (2017, June 1). Digital Signatures for Kernel Modules on Windows. Retrieved April 22, 2021.](https://docs.microsoft.com/en-us/previous-versions/windows/hardware/design/dn653559(v=vs.85)?redirectedfrom=MSDN)
-    [Apple. (n.d.). Disabling and Enabling System Integrity Protection. Retrieved April 22, 2021.](https://developer.apple.com/documentation/security/disabling_and_enabling_system_integrity_protection)
-    [Microsoft. (2017, April 20). Installing an Unsigned Driver during Development and Test. Retrieved April 22, 2021.](https://docs.microsoft.com/en-us/windows-hardware/drivers/install/installing-an-unsigned-driver-during-development-and-test)
-    [Microsoft. (2021, February 15). Enable Loading of Test Signed Drivers. Retrieved April 22, 2021.](https://docs.microsoft.com/en-us/windows-hardware/drivers/install/the-testsigning-boot-configuration-option)
-    [Glyer, C., Kazanciyan, R. (2012, August 22). The “Hikit” Rootkit: Advanced and Persistent Attack Techniques (Part 2). Retrieved November 17, 2024.](https://web.archive.org/web/20210920172620/https://www.fireeye.com/blog/threat-research/2012/08/hikit-rootkit-advanced-persistent-attack-techniques-part-2.html)

-    [TDL Project. (2016, February 4). TDL (Turla Driver Loader). Retrieved April 22, 2021.](https://github.com/hfiref0x/TDL)
-    [F-Secure Labs. (2014). BlackEnergy & Quedagh: The convergence of crimeware and APT attacks. Retrieved March 24, 2016.](https://blog-assets.f-secure.com/wp-content/uploads/2019/10/15163408/BlackEnergy_Quedagh.pdf)
-    [Reichel, D. and Idrizovic, E. (2020, June 17). AcidBox: Rare Malware Repurposing Turla Group Exploit Targeted Russian Organizations. Retrieved March 16, 2021.](https://unit42.paloaltonetworks.com/acidbox-rare-malware/)
-    [FBI. (2020, September 17). Indicators of Compromise Associated with Rana Intelligence Computing, also known as Advanced Persistent Threat 39, Chafer, Cadelspy, Remexi, and ITG07. Retrieved December 10, 2020.](https://www.iranwatch.org/sites/default/files/public-intelligence-alert.pdf)
-    [Lunghi, D. and Lu, K. (2021, April 9). Iron Tiger APT Updates Toolkit With Evolved SysUpdate Malware. Retrieved November 12, 2021.](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)

×
