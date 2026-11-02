---
url: "https://attack.mitre.org/techniques/T1574/007/"
title: "Hijack Execution Flow: Path Interception by PATH Environment Variable, Sub-technique T1574.007 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T19:01:48Z"
via: "live"
sha256: "562626efb83a5436e6cba9d6e59922a79b1b889675e4849fd6c9d553df3327e2"
wayback: "https://web.archive.org/web/20261023161641/https://attack.mitre.org/techniques/T1574/007/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Hijack Execution Flow: Path Interception by PATH Environment Variable, Sub-technique T1574.007 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1574/007/>
> Snapshot: <https://web.archive.org/web/20261023161641/https://attack.mitre.org/techniques/T1574/007/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Hijack Execution Flow](https://attack.mitre.org/techniques/T1574)
- Path Interception by PATH Environment Variable

#  Hijack Execution Flow: Path Interception by PATH Environment Variable

##### Other sub-techniques of Hijack Execution Flow (12)

| ID  | Name

|  [T1574.001](https://attack.mitre.org/techniques/T1574/001/)   |  [DLL](https://attack.mitre.org/techniques/T1574/001/)

|  [T1574.004](https://attack.mitre.org/techniques/T1574/004/)   |  [Dylib Hijacking](https://attack.mitre.org/techniques/T1574/004/)

|  [T1574.005](https://attack.mitre.org/techniques/T1574/005/)   |  [Executable Installer File Permissions Weakness](https://attack.mitre.org/techniques/T1574/005/)

|  [T1574.006](https://attack.mitre.org/techniques/T1574/006/)   |  [Dynamic Linker Hijacking](https://attack.mitre.org/techniques/T1574/006/)

|  T1574.007   |  Path Interception by PATH Environment Variable

|  [T1574.008](https://attack.mitre.org/techniques/T1574/008/)   |  [Path Interception by Search Order Hijacking](https://attack.mitre.org/techniques/T1574/008/)

|  [T1574.009](https://attack.mitre.org/techniques/T1574/009/)   |  [Path Interception by Unquoted Path](https://attack.mitre.org/techniques/T1574/009/)

|  [T1574.010](https://attack.mitre.org/techniques/T1574/010/)   |  [Services File Permissions Weakness](https://attack.mitre.org/techniques/T1574/010/)

|  [T1574.011](https://attack.mitre.org/techniques/T1574/011/)   |  [Services Registry Permissions Weakness](https://attack.mitre.org/techniques/T1574/011/)

|  [T1574.012](https://attack.mitre.org/techniques/T1574/012/)   |  [COR_PROFILER](https://attack.mitre.org/techniques/T1574/012/)

|  [T1574.013](https://attack.mitre.org/techniques/T1574/013/)   |  [KernelCallbackTable](https://attack.mitre.org/techniques/T1574/013/)

|  [T1574.014](https://attack.mitre.org/techniques/T1574/014/)   |  [AppDomainManager](https://attack.mitre.org/techniques/T1574/014/)

Adversaries may execute their own malicious payloads by hijacking environment variables used to load libraries. The PATH environment variable contains a list of directories (User and System) that the OS searches sequentially through in search of the binary that was called from a script or the command line.

Adversaries can place a malicious program in an earlier entry in the list of directories stored in the PATH environment variable, resulting in the operating system executing the malicious binary rather than the legitimate binary when it searches sequentially through that PATH listing.

For example, on Windows if an adversary places a malicious program named "net.exe" in C:\example path, which by default precedes C:\Windows\system32\net.exe in the PATH environment variable, when "net" is executed from the command-line the C:\example path will be called instead of the system's legitimate executable at C:\Windows\system32\net.exe. Some methods of executing a program rely on the PATH environment variable to determine the locations that are searched when the path for the program is not given, such as executing programs from a [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059).[[1]](https://www.expressvpn.com/blog/cybersecurity-lessons-a-path-vulnerability-in-windows/)

Adversaries may also directly modify the $PATH variable specifying the directories to be searched. An adversary can modify the $PATH variable to point to a directory they have write access. When a program using the $PATH variable is called, the OS searches the specified directory and executes the malicious binary. On macOS, this can also be performed through modifying the $HOME variable. These variables can be modified using the command-line, launchctl, [Unix Shell Configuration Modification](https://attack.mitre.org/techniques/T1546/004), or modifying the /etc/paths.d folder contents.[[2]](https://www.uptycs.com/blog/new-poc-exploit-backdoor-malware)[[3]](https://www.cyberciti.biz/faq/appleosx-bash-unix-change-set-path-environment-variable/)[[4]](https://www.elastic.co/guide/en/security/7.17/prebuilt-rule-7-16-4-modification-of-environment-variable-via-launchctl.html)

ID: T1574.007

Sub-technique of:  [T1574](https://attack.mitre.org/techniques/T1574)

ⓘ

Tactics: [Stealth](https://attack.mitre.org/tactics/TA0005), [Execution](https://attack.mitre.org/tactics/TA0002)

ⓘ

Platforms: Linux, Windows, macOS

Contributors: Stefan Kanthak

Version: 2.0

Created: 13 March 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1574/007/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1574/007/)

## Procedure Examples

| ID  | Name  | Description

|  [S9015](https://attack.mitre.org/software/S9015)   |  [BRICKSTORM](https://attack.mitre.org/software/S9015)   |

[BRICKSTORM](https://attack.mitre.org/software/S9015) has checked hard-coded paths of /etc/sysconfig/ or /etc/sysconfig/network prior to execution and loading file contents from that path.[[5]](https://www.cisa.gov/news-events/analysis-reports/ar25-338a)

|  [S1111](https://attack.mitre.org/software/S1111)   |  [DarkGate](https://attack.mitre.org/software/S1111)   |

[DarkGate](https://attack.mitre.org/software/S1111) overrides the %windir% environment variable by setting a Registry key, HKEY_CURRENT_User\Environment\windir, to an alternate command to execute a malicious AutoIt script. This allows [DarkGate](https://attack.mitre.org/software/S1111) to run every time the scheduled task DiskCleanup is executed as this uses the path value %windir%\system32\cleanmgr.exe for execution.[[6]](https://www.fortinet.com/blog/threat-research/enter-the-darkgate-new-cryptocurrency-mining-and-ransomware-campaign)

|  [S0363](https://attack.mitre.org/software/S0363)   |  [Empire](https://attack.mitre.org/software/S0363)   |

[Empire](https://attack.mitre.org/software/S0363) contains modules that can discover and exploit path interception opportunities in the PATH environment variable.[[7]](https://github.com/PowerShellEmpire/Empire)

|  [S0194](https://attack.mitre.org/software/S0194)   |  [PowerSploit](https://attack.mitre.org/software/S0194)   |

[PowerSploit](https://attack.mitre.org/software/S0194) contains a collection of Privesc-PowerUp modules that can discover and exploit path interception opportunities in the PATH environment variable.[[8]](https://github.com/PowerShellMafia/PowerSploit)[[9]](http://powersploit.readthedocs.io)

## Mitigations

| ID  | Mitigation  | Description

|  [M1047](https://attack.mitre.org/mitigations/M1047)   |  [Audit](https://attack.mitre.org/mitigations/M1047)   |

Find and eliminate path interception weaknesses in program configuration files, scripts, the PATH environment variable, services, and in shortcuts by surrounding PATH variables with quotation marks when functions allow for them. Be aware of the search order Windows uses for executing or loading binaries and use fully qualified paths wherever appropriate.

Clean up old Windows Registry keys when software is uninstalled to avoid keys with no associated legitimate binaries. Periodically search for and correct or report path interception weaknesses on systems that may have been introduced using custom or available tools that report software using insecure path configurations.[[10]](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessa)[[11]](https://docs.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-security?redirectedfrom=MSDN)[[12]](https://skanthak.homepage.t-online.de/sentinel.html)

|  [M1038](https://attack.mitre.org/mitigations/M1038)   |  [Execution Prevention](https://attack.mitre.org/mitigations/M1038)   |

Adversaries will likely need to place new binaries in locations to be executed through this weakness. Identify and block potentially malicious software executed path interception by using application control tools, like Windows Defender Application Control, AppLocker, or Software Restriction Policies where appropriate.[[13]](https://www.sans.org/reading-room/whitepapers/application/application-whitelisting-panacea-propaganda-33599)[[14]](https://docs.microsoft.com/en-us/windows/security/threat-protection/windows-defender-application-control/windows-defender-application-control)[[15]](https://blogs.jpcert.or.jp/en/2016/01/windows-commands-abused-by-attackers.html)[[16]](https://apps.nsa.gov/iaarchive/library/ia-guidance/tech-briefs/application-whitelisting-using-microsoft-applocker.cfm)[[17]](https://docs.microsoft.com/en-us/previous-versions/technet-magazine/cc510322(v=msdn.10)?redirectedfrom=MSDN)[[18]](https://docs.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2012-R2-and-2012/ee791851(v=ws.11)?redirectedfrom=MSDN)

|  [M1022](https://attack.mitre.org/mitigations/M1022)   |  [Restrict File and Directory Permissions](https://attack.mitre.org/mitigations/M1022)   |

Ensure that proper permissions and directory access control are set to deny users the ability to write files to the top-level directory C: and system directories, such as C:\Windows\, to reduce places where malicious files could be placed for execution. Require that all executables be placed in write-protected directories.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0004](https://attack.mitre.org/detectionstrategies/DET0004)   |  [Detection Strategy for Hijack Execution Flow using Path Interception by PATH Environment Variable.](https://attack.mitre.org/detectionstrategies/DET0004)   |  [AN0009](https://attack.mitre.org/detectionstrategies/DET0004#AN0009)   |

Abnormal modification of the PATH environment variable or registry keys controlling system paths, combined with execution of binaries named after legitimate system tools from user-writable directories. Defender correlates registry modifications, file creation of suspicious binaries, and process execution paths inconsistent with baseline system directories.

|  [AN0010](https://attack.mitre.org/detectionstrategies/DET0004#AN0010)   |

User modification of the $PATH environment variable in shell configuration files or direct runtime PATH changes, followed by execution of binaries from user-controlled directories. Defender observes file edits to ~/.bashrc, ~/.profile, or /etc/paths.d and process execution resolving to unexpected binary locations.

|  [AN0011](https://attack.mitre.org/detectionstrategies/DET0004#AN0011)   |

Modification of PATH or HOME environment variables through shell config files, launchctl, or /etc/paths.d entries, combined with process execution from attacker-controlled directories. Defender correlates file changes in /etc/paths.d with process execution resolving to malicious binaries.

## References

-    [ExpressVPN Security Team. (2021, November 16). Cybersecurity lessons: A PATH vulnerability in Windows. Retrieved September 28, 2023.](https://www.expressvpn.com/blog/cybersecurity-lessons-a-path-vulnerability-in-windows/)
-    [Nischay Hegde and Siddartha Malladi. (2023, July 12). PoC Exploit: Fake Proof of Concept with Backdoor Malware. Retrieved September 28, 2023.](https://www.uptycs.com/blog/new-poc-exploit-backdoor-malware)
-    [Vivek Gite. (2023, August 22). MacOS – Set / Change $PATH Variable Command. Retrieved September 28, 2023.](https://www.cyberciti.biz/faq/appleosx-bash-unix-change-set-path-environment-variable/)
-    [Elastic Security 7.17. (2022, February 1). Modification of Environment Variable via Launchctl. Retrieved September 28, 2023.](https://www.elastic.co/guide/en/security/7.17/prebuilt-rule-7-16-4-modification-of-environment-variable-via-launchctl.html)
-    [DHS/CISA. (2026, February 11). AR25-338A: BRICKSTORM Backdoor. Retrieved April 16, 2026.](https://www.cisa.gov/news-events/analysis-reports/ar25-338a)
-    [Adi Zeligson & Rotem Kerner. (2018, November 13). Enter The DarkGate - New Cryptocurrency Mining and Ransomware Campaign. Retrieved February 9, 2024.](https://www.fortinet.com/blog/threat-research/enter-the-darkgate-new-cryptocurrency-mining-and-ransomware-campaign)
-    [Schroeder, W., Warner, J., Nelson, M. (n.d.). Github PowerShellEmpire. Retrieved April 28, 2016.](https://github.com/PowerShellEmpire/Empire)
-    [PowerShellMafia. (2012, May 26). PowerSploit - A PowerShell Post-Exploitation Framework. Retrieved February 6, 2018.](https://github.com/PowerShellMafia/PowerSploit)
-    [PowerSploit. (n.d.). PowerSploit. Retrieved February 6, 2018.](http://powersploit.readthedocs.io)

-    [Microsoft. (n.d.). CreateProcess function. Retrieved September 12, 2024.](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessa)
-    [Microsoft. (n.d.). Dynamic-Link Library Security. Retrieved July 25, 2016.](https://docs.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-security?redirectedfrom=MSDN)
-    [Kanthak, S.. (2016, July 20). Vulnerability and Exploit Detector. Retrieved February 3, 2017.](https://skanthak.homepage.t-online.de/sentinel.html)
-    [Beechey, J.. (2014, November 18). Application Whitelisting: Panacea or Propaganda?. Retrieved November 18, 2014.](https://www.sans.org/reading-room/whitepapers/application/application-whitelisting-panacea-propaganda-33599)
-    [Gorzelany, A., Hall, J., Poggemeyer, L.. (2019, January 7). Windows Defender Application Control. Retrieved July 16, 2019.](https://docs.microsoft.com/en-us/windows/security/threat-protection/windows-defender-application-control/windows-defender-application-control)
-    [Tomonaga, S. (2016, January 26). Windows Commands Abused by Attackers. Retrieved February 2, 2016.](https://blogs.jpcert.or.jp/en/2016/01/windows-commands-abused-by-attackers.html)
-    [NSA Information Assurance Directorate. (2014, August). Application Whitelisting Using Microsoft AppLocker. Retrieved March 31, 2016.](https://apps.nsa.gov/iaarchive/library/ia-guidance/tech-briefs/application-whitelisting-using-microsoft-applocker.cfm)
-    [Corio, C., & Sayana, D. P.. (2008, June). Application Lockdown with Software Restriction Policies. Retrieved November 18, 2014.](https://docs.microsoft.com/en-us/previous-versions/technet-magazine/cc510322(v=msdn.10)?redirectedfrom=MSDN)
-    [Microsoft. (2012, June 27). Using Software Restriction Policies and AppLocker Policies. Retrieved April 7, 2016.](https://docs.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2012-R2-and-2012/ee791851(v=ws.11)?redirectedfrom=MSDN)

×
