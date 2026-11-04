---
url: "https://attack.mitre.org/techniques/T1547/006/"
title: "Boot or Logon Autostart Execution: Kernel Modules and Extensions, Sub-technique T1547.006 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:48:59Z"
via: "live"
sha256: "742ae5c0644fa458f30d1f8c205f22c5afecc53831310cc874a23bbb3c7347b2"
wayback: "https://web.archive.org/web/20261023090746/https://attack.mitre.org/techniques/T1547/006/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
  - "docs/threats/05-malware-families-and-detection.md"
---

# Boot or Logon Autostart Execution: Kernel Modules and Extensions, Sub-technique T1547.006 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1547/006/>
> Snapshot: <https://web.archive.org/web/20261023090746/https://attack.mitre.org/techniques/T1547/006/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Boot or Logon Autostart Execution](https://attack.mitre.org/techniques/T1547)
- Kernel Modules and Extensions

#  Boot or Logon Autostart Execution: Kernel Modules and Extensions

##### Other sub-techniques of Boot or Logon Autostart Execution (14)

| ID  | Name

|  [T1547.001](https://attack.mitre.org/techniques/T1547/001/)   |  [Registry Run Keys / Startup Folder](https://attack.mitre.org/techniques/T1547/001/)

|  [T1547.002](https://attack.mitre.org/techniques/T1547/002/)   |  [Authentication Package](https://attack.mitre.org/techniques/T1547/002/)

|  [T1547.003](https://attack.mitre.org/techniques/T1547/003/)   |  [Time Providers](https://attack.mitre.org/techniques/T1547/003/)

|  [T1547.004](https://attack.mitre.org/techniques/T1547/004/)   |  [Winlogon Helper DLL](https://attack.mitre.org/techniques/T1547/004/)

|  [T1547.005](https://attack.mitre.org/techniques/T1547/005/)   |  [Security Support Provider](https://attack.mitre.org/techniques/T1547/005/)

|  T1547.006   |  Kernel Modules and Extensions

|  [T1547.007](https://attack.mitre.org/techniques/T1547/007/)   |  [Re-opened Applications](https://attack.mitre.org/techniques/T1547/007/)

|  [T1547.008](https://attack.mitre.org/techniques/T1547/008/)   |  [LSASS Driver](https://attack.mitre.org/techniques/T1547/008/)

|  [T1547.009](https://attack.mitre.org/techniques/T1547/009/)   |  [Shortcut Modification](https://attack.mitre.org/techniques/T1547/009/)

|  [T1547.010](https://attack.mitre.org/techniques/T1547/010/)   |  [Port Monitors](https://attack.mitre.org/techniques/T1547/010/)

|  [T1547.012](https://attack.mitre.org/techniques/T1547/012/)   |  [Print Processors](https://attack.mitre.org/techniques/T1547/012/)

|  [T1547.013](https://attack.mitre.org/techniques/T1547/013/)   |  [XDG Autostart Entries](https://attack.mitre.org/techniques/T1547/013/)

|  [T1547.014](https://attack.mitre.org/techniques/T1547/014/)   |  [Active Setup](https://attack.mitre.org/techniques/T1547/014/)

|  [T1547.015](https://attack.mitre.org/techniques/T1547/015/)   |  [Login Items](https://attack.mitre.org/techniques/T1547/015/)

Adversaries may modify the kernel to automatically execute programs on system boot. Loadable Kernel Modules (LKMs) are pieces of code that can be loaded and unloaded into the kernel upon demand. They extend the functionality of the kernel without the need to reboot the system. For example, one type of module is the device driver, which allows the kernel to access hardware connected to the system.[[1]](https://www.tldp.org/LDP/lkmpg/2.4/lkmpg.pdf)

When used maliciously, LKMs can be a type of kernel-mode [Rootkit](https://attack.mitre.org/techniques/T1014) that run with the highest operating system privilege (Ring 0).[[2]](https://tldp.org/LDP/lkmpg/2.4/html/x437.html) Common features of LKM based rootkits include: hiding itself, selective hiding of files, processes and network activity, as well as log tampering, providing authenticated backdoors, and enabling root access to non-privileged users.[[3]](https://www.megasecurity.org/papers/Rootkits.pdf)

Kernel extensions, also called kext, are used in macOS to load functionality onto a system similar to LKMs for Linux. Since the kernel is responsible for enforcing security and the kernel extensions run as apart of the kernel, kexts are not governed by macOS security policies. Kexts are loaded and unloaded through kextload and kextunload commands. Kexts need to be signed with a developer ID that is granted privileges by Apple allowing it to sign Kernel extensions. Developers without these privileges may still sign kexts but they will not load unless SIP is disabled. If SIP is enabled, the kext signature is verified before being added to the AuxKC.[[4]](https://support.apple.com/guide/deployment/system-and-kernel-extensions-in-macos-depa5fb8376f/web)

Since macOS Catalina 10.15, kernel extensions have been deprecated in favor of System Extensions. However, kexts are still allowed as "Legacy System Extensions" since there is no System Extension for Kernel Programming Interfaces.[[5]](https://developer.apple.com/support/kernel-extensions/)

Adversaries can use LKMs and kexts to conduct [Persistence](https://attack.mitre.org/tactics/TA0003) and/or [Privilege Escalation](https://attack.mitre.org/tactics/TA0004) on a system. Examples have been found in the wild, and there are some relevant open source projects as well.[[6]](https://volatility-labs.blogspot.com/2012/10/phalanx-2-revealed-using-volatility-to.html)[[7]](https://www.crowdstrike.com/blog/http-iframe-injecting-linux-rootkit/)[[8]](https://github.com/f0rb1dd3n/Reptile)[[9]](https://github.com/m0nad/Diamorphine)[[10]](https://www.virusbulletin.com/uploads/pdf/conference/vb2014/VB2014-Wardle.pdf)[[11]](https://objective-see.org/blog/blog_0x21.html)[[12]](https://securelist.com/the-ventir-trojan-assemble-your-macos-spy/67267/)[[13]](https://blog.trendmicro.com/trendlabs-security-intelligence/skidmap-linux-malware-uses-rootkit-capabilities-to-hide-cryptocurrency-mining-payload/)

ID: T1547.006

Sub-technique of:  [T1547](https://attack.mitre.org/techniques/T1547)

ⓘ

Tactics: [Persistence](https://attack.mitre.org/tactics/TA0003), [Privilege Escalation](https://attack.mitre.org/tactics/TA0004)

ⓘ

Platforms: Linux, macOS

Contributors: Anastasios Pingios; Eric Kaiser @ideologysec; Jeremy Galloway; Red Canary; Wayne Silva, F-Secure Countercept

Version: 1.4

Created: 24 January 2020

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1547/006/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1547/006/)

## Procedure Examples

| ID  | Name  | Description

|  [S0502](https://attack.mitre.org/software/S0502)   |  [Drovorub](https://attack.mitre.org/software/S0502)   |

[Drovorub](https://attack.mitre.org/software/S0502) can use kernel modules to establish persistence.[[14]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

|  [C0012](https://attack.mitre.org/campaigns/C0012)   |  [Operation CuckooBees](https://attack.mitre.org/campaigns/C0012)   |

During [Operation CuckooBees](https://attack.mitre.org/campaigns/C0012), attackers used a signed kernel rootkit to establish additional persistence.[[15]](https://www.cybereason.com/blog/operation-cuckoobees-deep-dive-into-stealthy-winnti-techniques)

|  [S1219](https://attack.mitre.org/software/S1219)   |  [REPTILE](https://attack.mitre.org/software/S1219)   |

The [REPTILE](https://attack.mitre.org/software/S1219) rootkit is implemented as a loadable kernel module (LKM).[[16]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

|  [S0468](https://attack.mitre.org/software/S0468)   |  [Skidmap](https://attack.mitre.org/software/S0468)   |

[Skidmap](https://attack.mitre.org/software/S0468) has the ability to install several loadable kernel modules (LKMs) on infected machines.[[13]](https://blog.trendmicro.com/trendlabs-security-intelligence/skidmap-linux-malware-uses-rootkit-capabilities-to-hide-cryptocurrency-mining-payload/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1049](https://attack.mitre.org/mitigations/M1049)   |  [Antivirus/Antimalware](https://attack.mitre.org/mitigations/M1049)   |

Common tools for detecting Linux rootkits include: rkhunter [[17]](http://rkhunter.sourceforge.net), chrootkit [[18]](http://www.chkrootkit.org/), although rootkits may be designed to evade certain detection tools.

|  [M1038](https://attack.mitre.org/mitigations/M1038)   |  [Execution Prevention](https://attack.mitre.org/mitigations/M1038)   |

Application control and software restriction tools, such as SELinux, KSPP, grsecurity MODHARDEN, and Linux kernel tuning can aid in restricting kernel module loading.[[19]](https://patchwork.kernel.org/patch/8754821/)[[20]](https://en.wikibooks.org/wiki/Grsecurity/The_RBAC_System)[[21]](https://www.kernel.org/doc/html/latest/security/self-protection.html)[[22]](https://linux-audit.com/increase-kernel-integrity-with-disabled-linux-kernel-modules-loading/)[[23]](https://xorl.wordpress.com/2018/02/17/lkm-loading-kernel-restrictions/)

|  [M1026](https://attack.mitre.org/mitigations/M1026)   |  [Privileged Account Management](https://attack.mitre.org/mitigations/M1026)   |

Limit access to the root account and prevent users from loading kernel modules and extensions through proper privilege separation and limiting Privilege Escalation opportunities.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Use MDM to disable user's ability to install or approve kernel extensions, and ensure all approved kernel extensions are in alignment with policies specified in com.apple.syspolicy.kernel-extension-policy.[[24]](https://developer.apple.com/library/archive/technotes/tn2459/_index.html)[[25]](https://developer.apple.com/business/documentation/Configuration-Profile-Reference.pdf)

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0450](https://attack.mitre.org/detectionstrategies/DET0450)   |  [Detection Strategy for Kernel Modules and Extensions Autostart Execution](https://attack.mitre.org/detectionstrategies/DET0450)   |  [AN1243](https://attack.mitre.org/detectionstrategies/DET0450#AN1243)   |

Monitor kernel module load/unload activity via modprobe, insmod, rmmod, or direct manipulation of /lib/modules. Correlate with installation of kernel headers, compilation commands, or downloads of .ko files. Detect anomalies in unsigned module loading or repeated module load attempts under non-root users.

|  [AN1244](https://attack.mitre.org/detectionstrategies/DET0450#AN1244)   |

Detect user-initiated kextload commands or modifications to /Library/Extensions. Correlate with changes to KextPolicy database or unauthorized developer signing identities. Alert on attempts to disable SIP or load legacy extensions from unsigned sources.

## References

-    [Pomerantz, O., Salzman, P.. (2003, April 4). The Linux Kernel Module Programming Guide. Retrieved April 6, 2018.](https://www.tldp.org/LDP/lkmpg/2.4/lkmpg.pdf)
-    [Pomerantz, O., Salzman, P. (2003, April 4). Modules vs Programs. Retrieved November 17, 2024.](https://tldp.org/LDP/lkmpg/2.4/html/x437.html)
-    [Chuvakin, A. (2003, February). An Overview of Rootkits. Retrieved September 12, 2024.](https://www.megasecurity.org/papers/Rootkits.pdf)
-    [Apple. (n.d.). System and kernel extensions in macOS. Retrieved March 31, 2022.](https://support.apple.com/guide/deployment/system-and-kernel-extensions-in-macos-depa5fb8376f/web)
-    [Apple. (n.d.). Deprecated Kernel Extensions and System Extension Alternatives. Retrieved November 4, 2020.](https://developer.apple.com/support/kernel-extensions/)
-    [Case, A. (2012, October 10). Phalanx 2 Revealed: Using Volatility to Analyze an Advanced Linux Rootkit. Retrieved April 9, 2018.](https://volatility-labs.blogspot.com/2012/10/phalanx-2-revealed-using-volatility-to.html)
-    [Kurtz, G. (2012, November 19). HTTP iframe Injecting Linux Rootkit. Retrieved December 21, 2017.](https://www.crowdstrike.com/blog/http-iframe-injecting-linux-rootkit/)
-    [Augusto, I. (2018, March 8). Reptile - LMK Linux rootkit. Retrieved April 9, 2018.](https://github.com/f0rb1dd3n/Reptile)
-    [Mello, V. (2018, March 8). Diamorphine - LMK rootkit for Linux Kernels 2.6.x/3.x/4.x (x86 and x86_64). Retrieved April 9, 2018.](https://github.com/m0nad/Diamorphine)
-    [Wardle, P. (2015, April). Malware Persistence on OS X Yosemite. Retrieved April 6, 2018.](https://www.virusbulletin.com/uploads/pdf/conference/vb2014/VB2014-Wardle.pdf)
-    [Wardle, P. (2017, September 8). High Sierra’s ‘Secure Kernel Extension Loading’ is Broken. Retrieved November 17, 2024.](https://objective-see.org/blog/blog_0x21.html)
-    [Mikhail, K. (2014, October 16). The Ventir Trojan: assemble your MacOS spy. Retrieved April 6, 2018.](https://securelist.com/the-ventir-trojan-assemble-your-macos-spy/67267/)
-    [Remillano, A., Urbanec, J. (2019, September 19). Skidmap Linux Malware Uses Rootkit Capabilities to Hide Cryptocurrency-Mining Payload. Retrieved June 4, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/skidmap-linux-malware-uses-rootkit-capabilities-to-hide-cryptocurrency-mining-payload/)

-    [NSA/FBI. (2020, August). Russian GRU 85th GTsSS Deploys Previously Undisclosed Drovorub Malware. Retrieved August 25, 2020.](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)
-    [Cybereason Nocturnus. (2022, May 4). Operation CuckooBees: Deep-Dive into Stealthy Winnti Techniques. Retrieved September 22, 2022.](https://www.cybereason.com/blog/operation-cuckoobees-deep-dive-into-stealthy-winnti-techniques)
-    [Punsaen Boonyakarn, Shawn Chew, Logeswaran Nadarajan, Mathew Potaczek, Jakub Jozwiak, and Alex Marvi. (2024, June 18). Cloaked and Covert: Uncovering UNC3886 Espionage Operations. Retrieved September 24, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)
-    [Rootkit Hunter Project. (2018, February 20). The Rootkit Hunter project. Retrieved April 9, 2018.](http://rkhunter.sourceforge.net)
-    [Murilo, N., Steding-Jessen, K. (2017, August 23). Chkrootkit. Retrieved April 9, 2018.](http://www.chkrootkit.org/)
-    [Vander Stoep, J. (2016, April 5). [v3] selinux: restrict kernel module loadinglogin register. Retrieved April 9, 2018.](https://patchwork.kernel.org/patch/8754821/)
-    [Wikibooks. (2018, August 19). Grsecurity/The RBAC System. Retrieved June 4, 2020.](https://en.wikibooks.org/wiki/Grsecurity/The_RBAC_System)
-    [Kernel.org. (2020, February 6). Kernel Self-Protection. Retrieved June 4, 2020.](https://www.kernel.org/doc/html/latest/security/self-protection.html)
-    [Boelen, M. (2015, October 7). Increase kernel integrity with disabled Linux kernel modules loading. Retrieved June 4, 2020.](https://linux-audit.com/increase-kernel-integrity-with-disabled-linux-kernel-modules-loading/)
-    [Pingios, A.. (2018, February 7). LKM loading kernel restrictions. Retrieved June 4, 2020.](https://xorl.wordpress.com/2018/02/17/lkm-loading-kernel-restrictions/)
-    [Apple. (2018, April 19). Technical Note TN2459: User-Approved Kernel Extension Loading. Retrieved June 30, 2020.](https://developer.apple.com/library/archive/technotes/tn2459/_index.html)
-    [Apple. (2019, May 3). Configuration Profile Reference, Developer. Retrieved April 15, 2022.](https://developer.apple.com/business/documentation/Configuration-Profile-Reference.pdf)

×
