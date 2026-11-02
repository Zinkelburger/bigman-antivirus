---
url: "https://attack.mitre.org/techniques/T1574/006/"
title: "Hijack Execution Flow: Dynamic Linker Hijacking, Sub-technique T1574.006 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T19:00:52Z"
via: "live"
sha256: "8d54d47d7d2c693960bb1fcab94d436ee5826559825a2e26084740e5894b2d40"
wayback: "https://web.archive.org/web/20261023090732/https://attack.mitre.org/techniques/T1574/006/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
  - "docs/threats/05-malware-families-and-detection.md"
---

# Hijack Execution Flow: Dynamic Linker Hijacking, Sub-technique T1574.006 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1574/006/>
> Snapshot: <https://web.archive.org/web/20261023090732/https://attack.mitre.org/techniques/T1574/006/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Hijack Execution Flow](https://attack.mitre.org/techniques/T1574)
- Dynamic Linker Hijacking

#  Hijack Execution Flow: Dynamic Linker Hijacking

##### Other sub-techniques of Hijack Execution Flow (12)

| ID  | Name

|  [T1574.001](https://attack.mitre.org/techniques/T1574/001/)   |  [DLL](https://attack.mitre.org/techniques/T1574/001/)

|  [T1574.004](https://attack.mitre.org/techniques/T1574/004/)   |  [Dylib Hijacking](https://attack.mitre.org/techniques/T1574/004/)

|  [T1574.005](https://attack.mitre.org/techniques/T1574/005/)   |  [Executable Installer File Permissions Weakness](https://attack.mitre.org/techniques/T1574/005/)

|  T1574.006   |  Dynamic Linker Hijacking

|  [T1574.007](https://attack.mitre.org/techniques/T1574/007/)   |  [Path Interception by PATH Environment Variable](https://attack.mitre.org/techniques/T1574/007/)

|  [T1574.008](https://attack.mitre.org/techniques/T1574/008/)   |  [Path Interception by Search Order Hijacking](https://attack.mitre.org/techniques/T1574/008/)

|  [T1574.009](https://attack.mitre.org/techniques/T1574/009/)   |  [Path Interception by Unquoted Path](https://attack.mitre.org/techniques/T1574/009/)

|  [T1574.010](https://attack.mitre.org/techniques/T1574/010/)   |  [Services File Permissions Weakness](https://attack.mitre.org/techniques/T1574/010/)

|  [T1574.011](https://attack.mitre.org/techniques/T1574/011/)   |  [Services Registry Permissions Weakness](https://attack.mitre.org/techniques/T1574/011/)

|  [T1574.012](https://attack.mitre.org/techniques/T1574/012/)   |  [COR_PROFILER](https://attack.mitre.org/techniques/T1574/012/)

|  [T1574.013](https://attack.mitre.org/techniques/T1574/013/)   |  [KernelCallbackTable](https://attack.mitre.org/techniques/T1574/013/)

|  [T1574.014](https://attack.mitre.org/techniques/T1574/014/)   |  [AppDomainManager](https://attack.mitre.org/techniques/T1574/014/)

Adversaries may execute their own malicious payloads by hijacking environment variables the dynamic linker uses to load shared libraries. During the execution preparation phase of a program, the dynamic linker loads specified absolute paths of shared libraries from various environment variables and files, such as LD_PRELOAD on Linux or DYLD_INSERT_LIBRARIES on macOS.[[1]](https://theevilbit.github.io/posts/dyld_insert_libraries_dylib_injection_in_macos_osx_deep_dive/)[[2]](https://blog.timac.org/2012/1218-simple-code-injection-using-dyld_insert_libraries/)[[3]](https://jon-gabilondo-angulo-7635.medium.com/how-to-inject-code-into-mach-o-apps-part-ii-ddb13ebc8191) Libraries specified in environment variables are loaded first, taking precedence over system libraries with the same function name.[[4]](https://www.man7.org/linux/man-pages/man8/ld.so.8.html)[[5]](https://www.tldp.org/HOWTO/Program-Library-HOWTO/shared-libraries.html)[[6]](https://developer.apple.com/library/archive/documentation/DeveloperTools/Conceptual/DynamicLibraries/100-Articles/OverviewOfDynamicLibraries.html) Each platform's linker uses an extensive list of environment variables at different points in execution. These variables are often used by developers to debug binaries without needing to recompile, deconflict mapped symbols, and implement custom functions in the original library.[[7]](https://www.baeldung.com/linux/ld_preload-trick-what-is)

Hijacking dynamic linker variables may grant access to the victim process's memory, system/network resources, and possibly elevated privileges. On Linux, adversaries may set LD_PRELOAD to point to malicious libraries that match the name of legitimate libraries which are requested by a victim program, causing the operating system to load the adversary's malicious code upon execution of the victim program. For example, adversaries have used LD_PRELOAD to inject a malicious library into every descendant process of the sshd daemon, resulting in execution under a legitimate process. When the executing sub-process calls the execve function, for example, the malicious library’s execve function is executed rather than the system function execve contained in the system library on disk. This allows adversaries to [Hide Artifacts](https://attack.mitre.org/techniques/T1564) from detection, as hooking system functions such as execve and readdir enables malware to scrub its own artifacts from the results of commands such as ls, ldd, iptables, and dmesg.[[8]](https://www.welivesecurity.com/2017/10/30/windigo-ebury-update-2/)[[9]](https://intezer.com/blog/research/new-linux-threat-symbiote/)[[10]](https://www.elastic.co/security-labs/declawing-pumakit)

Hijacking dynamic linker variables may grant access to the victim process's memory, system/network resources, and possibly elevated privileges.

ID: T1574.006

Sub-technique of:  [T1574](https://attack.mitre.org/techniques/T1574)

ⓘ

Tactics: [Stealth](https://attack.mitre.org/tactics/TA0005), [Execution](https://attack.mitre.org/tactics/TA0002)

ⓘ

Platforms: Linux, macOS

Version: 3.0

Created: 13 March 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1574/006/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1574/006/)

## Procedure Examples

| ID  | Name  | Description

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) has configured payloads to load via LD_PRELOAD.[[11]](https://go.crowdstrike.com/rs/281-OBQ-266/images/Report2020CrowdStrikeGlobalThreatReport.pdf)

|  [G0143](https://attack.mitre.org/groups/G0143)   |  [Aquatic Panda](https://attack.mitre.org/groups/G0143)   |

[Aquatic Panda](https://attack.mitre.org/groups/G0143) modified the ld.so preload file in Linux environments to enable persistence for Winnti malware.[[12]](https://go.crowdstrike.com/rs/281-OBQ-266/images/2022OverWatchThreatHuntingReport.pdf)

|  [S1105](https://attack.mitre.org/software/S1105)   |  [COATHANGER](https://attack.mitre.org/software/S1105)   |

[COATHANGER](https://attack.mitre.org/software/S1105) copies the malicious file /data2/.bd.key/preload.so to /lib/preload.so, then launches a child process that executes the malicious file /data2/.bd.key/authd as /bin/authd with the arguments /lib/preload.so reboot newreboot 1.[[13]](https://www.ncsc.nl/binaries/ncsc/documenten/publicaties/2024/februari/6/mivd-aivd-advisory-coathanger-tlp-clear/TLP-CLEAR+MIVD+AIVD+Advisory+COATHANGER.pdf) This injects the malicious preload.so file into the process with PID 1, and replaces its reboot function with the malicious newreboot function for persistence.

|  [S0377](https://attack.mitre.org/software/S0377)   |  [Ebury](https://attack.mitre.org/software/S0377)   |

When [Ebury](https://attack.mitre.org/software/S0377) is running as an OpenSSH server, it uses LD_PRELOAD to inject its malicious shared module in to programs launched by SSH sessions. [Ebury](https://attack.mitre.org/software/S0377) hooks the following functions from libc to inject into subprocesses; system, popen, execve, execvpe, execv, execvp, and execl.[[8]](https://www.welivesecurity.com/2017/10/30/windigo-ebury-update-2/)[[14]](https://web-assets.esetstatic.com/wls/en/papers/white-papers/ebury-is-alive-but-unseen.pdf)

|  [S0394](https://attack.mitre.org/software/S0394)   |  [HiddenWasp](https://attack.mitre.org/software/S0394)   |

[HiddenWasp](https://attack.mitre.org/software/S0394) adds itself as a shared object to the LD_PRELOAD environment variable.[[15]](https://www.intezer.com/blog-hiddenwasp-malware-targeting-linux-systems/)

|  [S0601](https://attack.mitre.org/software/S0601)   |  [Hildegard](https://attack.mitre.org/software/S0601)   |

[Hildegard](https://attack.mitre.org/software/S0601) has modified /etc/ld.so.preload to intercept shared library import functions.[[16]](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)

|  [S1220](https://attack.mitre.org/software/S1220)   |  [MEDUSA](https://attack.mitre.org/software/S1220)   |

[MEDUSA](https://attack.mitre.org/software/S1220) can execute code through dynamic linker hijacking of the LD_PRELOAD library.[[17]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

|  [G0106](https://attack.mitre.org/groups/G0106)   |  [Rocke](https://attack.mitre.org/groups/G0106)   |

[Rocke](https://attack.mitre.org/groups/G0106) has modified /etc/ld.so.preload to hook libc functions in order to hide the installed dropper and mining software in process lists.[[18]](https://www.anomali.com/blog/rocke-evolves-its-arsenal-with-a-new-malware-family-written-in-golang)

|  [S9024](https://attack.mitre.org/software/S9024)   |  [SPAWNCHIMERA](https://attack.mitre.org/software/S9024)   |

[SPAWNCHIMERA](https://attack.mitre.org/software/S9024) has been compiled as a Position Independent Executable (PIE) to use a third-party library for injection.[[19]](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-post-exploitation-lateral-movement)

|  [S0658](https://attack.mitre.org/software/S0658)   |  [XCSSET](https://attack.mitre.org/software/S0658)   |

[XCSSET](https://attack.mitre.org/software/S0658) adds malicious file paths to the DYLD_FRAMEWORK_PATH and DYLD_LIBRARY_PATH environment variables to execute malicious code.[[20]](https://documents.trendmicro.com/assets/pdf/XCSSET_Technical_Brief.pdf)

## Mitigations

| ID  | Mitigation  | Description

|  [M1038](https://attack.mitre.org/mitigations/M1038)   |  [Execution Prevention](https://attack.mitre.org/mitigations/M1038)   |

Adversaries may use new payloads to execute this technique. Identify and block potentially malicious software executed through hijacking by using application control solutions also capable of blocking libraries loaded by legitimate software.

|  [M1028](https://attack.mitre.org/mitigations/M1028)   |  [Operating System Configuration](https://attack.mitre.org/mitigations/M1028)   |

When System Integrity Protection (SIP) is enabled in macOS, the aforementioned environment variables are ignored when executing protected binaries. Third-party applications can also leverage Apple’s Hardened Runtime, ensuring these environment variables are subject to imposed restrictions.[[21]](https://developer.apple.com/documentation/security/hardened_runtime) Admins can add restrictions to applications by setting the setuid and/or setgid bits, use entitlements, or have a __RESTRICT segment in the Mach-O binary.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0435](https://attack.mitre.org/detectionstrategies/DET0435)   |  [Detection Strategy for Hijack Execution Flow: Dynamic Linker Hijacking](https://attack.mitre.org/detectionstrategies/DET0435)   |  [AN1209](https://attack.mitre.org/detectionstrategies/DET0435#AN1209)   |

Detection focuses on identifying abuse of LD_PRELOAD and related linker variables. Defender perspective: monitor unexpected setting or modification of LD_PRELOAD in shell initialization scripts or environment exports, file creation of suspicious shared libraries, and correlation of these modifications with anomalous process execution. Key signals include execve events with LD_PRELOAD defined, newly created .so files in user directories, and processes hooking libc functions exhibiting abnormal behavior.

|  [AN1210](https://attack.mitre.org/detectionstrategies/DET0435#AN1210)   |

Detection centers on DYLD_INSERT_LIBRARIES and DYLD_LIBRARY_PATH abuse. Defender perspective: monitor for modification of these environment variables in shell or plist files, file creation of dylibs in user-controlled paths, and correlation of environment variable usage with unexpected module loads by user applications. Suspicious indicators include processes with DYLD_INSERT_LIBRARIES set, execution of applications loading untrusted dylibs, and anomalies in module load history.

## References

-    [Fitzl, C. (2019, July 9). DYLD_INSERT_LIBRARIES DYLIB injection in macOS / OSX. Retrieved March 26, 2020.](https://theevilbit.github.io/posts/dyld_insert_libraries_dylib_injection_in_macos_osx_deep_dive/)
-    [Timac. (2012, December 18). Simple code injection using DYLD_INSERT_LIBRARIES. Retrieved March 26, 2020.](https://blog.timac.org/2012/1218-simple-code-injection-using-dyld_insert_libraries/)
-    [Jon Gabilondo. (2019, September 22). How to Inject Code into Mach-O Apps. Part II.. Retrieved March 24, 2021.](https://jon-gabilondo-angulo-7635.medium.com/how-to-inject-code-into-mach-o-apps-part-ii-ddb13ebc8191)
-    [Kerrisk, M. (2020, June 13). Linux Programmer's Manual. Retrieved June 15, 2020.](https://www.man7.org/linux/man-pages/man8/ld.so.8.html)
-    [The Linux Documentation Project. (n.d.). Shared Libraries. Retrieved January 31, 2020.](https://www.tldp.org/HOWTO/Program-Library-HOWTO/shared-libraries.html)
-    [Apple Inc.. (2012, July 23). Overview of Dynamic Libraries. Retrieved March 24, 2021.](https://developer.apple.com/library/archive/documentation/DeveloperTools/Conceptual/DynamicLibraries/100-Articles/OverviewOfDynamicLibraries.html)
-    [baeldung. (2020, August 9). What Is the LD_PRELOAD Trick?. Retrieved March 24, 2021.](https://www.baeldung.com/linux/ld_preload-trick-what-is)
-    [Vachon, F. (2017, October 30). Windigo Still not Windigone: An Ebury Update . Retrieved February 10, 2021.](https://www.welivesecurity.com/2017/10/30/windigo-ebury-update-2/)
-    [Joakim Kennedy and The BlackBerry Threat Research & Intelligence Team. (2022, June 9). Symbiote Deep-Dive: Analysis of a New, Nearly-Impossible-to-Detect Linux Threat. Retrieved March 24, 2025.](https://intezer.com/blog/research/new-linux-threat-symbiote/)
-    [Remco Sprooten and Ruben Groenewoud. (2024, December 11). Declawing PUMAKIT. Retrieved March 24, 2025.](https://www.elastic.co/security-labs/declawing-pumakit)
-    [Crowdstrike. (2020, March 2). 2020 Global Threat Report. Retrieved December 11, 2020.](https://go.crowdstrike.com/rs/281-OBQ-266/images/Report2020CrowdStrikeGlobalThreatReport.pdf)

-    [CrowdStrike. (2023). 2022 Falcon OverWatch Threat Hunting Report. Retrieved May 20, 2024.](https://go.crowdstrike.com/rs/281-OBQ-266/images/2022OverWatchThreatHuntingReport.pdf)
-    [Dutch Military Intelligence and Security Service (MIVD) & Dutch General Intelligence and Security Service (AIVD). (2024, February 6). Ministry of Defense of the Netherlands uncovers COATHANGER, a stealthy Chinese FortiGate RAT. Retrieved February 7, 2024.](https://www.ncsc.nl/binaries/ncsc/documenten/publicaties/2024/februari/6/mivd-aivd-advisory-coathanger-tlp-clear/TLP-CLEAR+MIVD+AIVD+Advisory+COATHANGER.pdf)
-    [Marc-Etienne M.Léveillé. (2024, May 1). Ebury is alive but unseen. Retrieved May 21, 2024.](https://web-assets.esetstatic.com/wls/en/papers/white-papers/ebury-is-alive-but-unseen.pdf)
-    [Sanmillan, I. (2019, May 29). HiddenWasp Malware Stings Targeted Linux Systems. Retrieved June 24, 2019.](https://www.intezer.com/blog-hiddenwasp-malware-targeting-linux-systems/)
-    [Chen, J. et al. (2021, February 3). Hildegard: New TeamTNT Cryptojacking Malware Targeting Kubernetes. Retrieved April 5, 2021.](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)
-    [Punsaen Boonyakarn, Shawn Chew, Logeswaran Nadarajan, Mathew Potaczek, Jakub Jozwiak, and Alex Marvi. (2024, June 18). Cloaked and Covert: Uncovering UNC3886 Espionage Operations. Retrieved September 24, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)
-    [Anomali Labs. (2019, March 15). Rocke Evolves Its Arsenal With a New Malware Family Written in Golang. Retrieved April 24, 2019.](https://www.anomali.com/blog/rocke-evolves-its-arsenal-with-a-new-malware-family-written-in-golang)
-    [Matt Lin, Austin Larsen, John Wolfram, Ashley Pearson, Josh Murchie, Lukasz Lamparski, Joseph Pisano, Ryan Hall, Ron Craft, Shawn Crew, Billy Wong, Tyler McLellan. (2024, April 4). Cutting Edge, Part 4: Ivanti Connect Secure VPN Post-Exploitation Lateral Movement Case Studies. Retrieved April 16, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-post-exploitation-lateral-movement)
-    [Mac Threat Response, Mobile Research Team. (2020, August 13). The XCSSET Malware: Inserts Malicious Code Into Xcode Projects, Performs UXSS Backdoor Planting in Safari, and Leverages Two Zero-day Exploits. Retrieved October 5, 2021.](https://documents.trendmicro.com/assets/pdf/XCSSET_Technical_Brief.pdf)
-    [Apple Inc.. (2021, January 1). Hardened Runtime: Manage security protections and resource access for your macOS apps.. Retrieved March 24, 2021.](https://developer.apple.com/documentation/security/hardened_runtime)

×
