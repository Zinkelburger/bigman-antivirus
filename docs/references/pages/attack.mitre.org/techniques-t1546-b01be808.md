---
url: "https://attack.mitre.org/techniques/T1546/"
title: "Event Triggered Execution, Technique T1546 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:46:37Z"
via: "live"
sha256: "3becbb547c860cf6371841cc1978becfa323f7038867a46d45890878fb66d8c6"
wayback: "https://web.archive.org/web/20261023090801/https://attack.mitre.org/techniques/T1546/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Event Triggered Execution, Technique T1546 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1546/>
> Snapshot: <https://web.archive.org/web/20261023090801/https://attack.mitre.org/techniques/T1546/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Event Triggered Execution

#  Event Triggered Execution

##### Sub-techniques (18)

| ID  | Name

|  [T1546.001](https://attack.mitre.org/techniques/T1546/001/)   |  [Change Default File Association](https://attack.mitre.org/techniques/T1546/001/)

|  [T1546.002](https://attack.mitre.org/techniques/T1546/002/)   |  [Screensaver](https://attack.mitre.org/techniques/T1546/002/)

|  [T1546.003](https://attack.mitre.org/techniques/T1546/003/)   |  [Windows Management Instrumentation Event Subscription](https://attack.mitre.org/techniques/T1546/003/)

|  [T1546.004](https://attack.mitre.org/techniques/T1546/004/)   |  [Unix Shell Configuration Modification](https://attack.mitre.org/techniques/T1546/004/)

|  [T1546.005](https://attack.mitre.org/techniques/T1546/005/)   |  [Trap](https://attack.mitre.org/techniques/T1546/005/)

|  [T1546.006](https://attack.mitre.org/techniques/T1546/006/)   |  [LC_LOAD_DYLIB Addition](https://attack.mitre.org/techniques/T1546/006/)

|  [T1546.007](https://attack.mitre.org/techniques/T1546/007/)   |  [Netsh Helper DLL](https://attack.mitre.org/techniques/T1546/007/)

|  [T1546.008](https://attack.mitre.org/techniques/T1546/008/)   |  [Accessibility Features](https://attack.mitre.org/techniques/T1546/008/)

|  [T1546.009](https://attack.mitre.org/techniques/T1546/009/)   |  [AppCert DLLs](https://attack.mitre.org/techniques/T1546/009/)

|  [T1546.010](https://attack.mitre.org/techniques/T1546/010/)   |  [AppInit DLLs](https://attack.mitre.org/techniques/T1546/010/)

|  [T1546.011](https://attack.mitre.org/techniques/T1546/011/)   |  [Application Shimming](https://attack.mitre.org/techniques/T1546/011/)

|  [T1546.012](https://attack.mitre.org/techniques/T1546/012/)   |  [Image File Execution Options Injection](https://attack.mitre.org/techniques/T1546/012/)

|  [T1546.013](https://attack.mitre.org/techniques/T1546/013/)   |  [PowerShell Profile](https://attack.mitre.org/techniques/T1546/013/)

|  [T1546.014](https://attack.mitre.org/techniques/T1546/014/)   |  [Emond](https://attack.mitre.org/techniques/T1546/014/)

|  [T1546.015](https://attack.mitre.org/techniques/T1546/015/)   |  [Component Object Model Hijacking](https://attack.mitre.org/techniques/T1546/015/)

|  [T1546.016](https://attack.mitre.org/techniques/T1546/016/)   |  [Installer Packages](https://attack.mitre.org/techniques/T1546/016/)

|  [T1546.017](https://attack.mitre.org/techniques/T1546/017/)   |  [Udev Rules](https://attack.mitre.org/techniques/T1546/017/)

|  [T1546.018](https://attack.mitre.org/techniques/T1546/018/)   |  [Python Startup Hooks](https://attack.mitre.org/techniques/T1546/018/)

Adversaries may establish persistence and/or elevate privileges using system mechanisms that trigger execution based on specific events. Various operating systems have means to monitor and subscribe to events such as logons or other user activity such as running specific applications/binaries. Cloud environments may also support various functions and services that monitor and can be invoked in response to specific cloud events.[[1]](https://medium.com/daniel-grzelak/backdooring-an-aws-account-da007d36f8f9)[[2]](https://www.varonis.com/blog/power-automate-data-exfiltration)[[3]](https://www.microsoft.com/security/blog/2020/03/09/real-life-cybercrime-stories-dart-microsoft-detection-and-response-team)

Adversaries may abuse these mechanisms as a means of maintaining persistent access to a victim via repeatedly executing malicious code. After gaining access to a victim system, adversaries may create/modify event triggers to point to malicious content that will be executed whenever the event trigger is invoked.[[4]](https://www.fireeye.com/content/dam/fireeye-www/global/en/current-threats/pdfs/wp-windows-management-instrumentation.pdf)[[5]](https://www.virusbulletin.com/uploads/pdf/conference/vb2014/VB2014-Wardle.pdf)[[6]](https://researchcenter.paloaltonetworks.com/2017/04/unit42-new-iotlinux-malware-targets-dvrs-forms-botnet/)

Since the execution can be proxied by an account with higher permissions, such as SYSTEM or service accounts, an adversary may be able to abuse these triggered execution mechanisms to escalate their privileges.

ID: T1546

Sub-techniques:  [T1546.001](https://attack.mitre.org/techniques/T1546/001), [T1546.002](https://attack.mitre.org/techniques/T1546/002), [T1546.003](https://attack.mitre.org/techniques/T1546/003), [T1546.004](https://attack.mitre.org/techniques/T1546/004), [T1546.005](https://attack.mitre.org/techniques/T1546/005), [T1546.006](https://attack.mitre.org/techniques/T1546/006), [T1546.007](https://attack.mitre.org/techniques/T1546/007), [T1546.008](https://attack.mitre.org/techniques/T1546/008), [T1546.009](https://attack.mitre.org/techniques/T1546/009), [T1546.010](https://attack.mitre.org/techniques/T1546/010), [T1546.011](https://attack.mitre.org/techniques/T1546/011), [T1546.012](https://attack.mitre.org/techniques/T1546/012), [T1546.013](https://attack.mitre.org/techniques/T1546/013), [T1546.014](https://attack.mitre.org/techniques/T1546/014), [T1546.015](https://attack.mitre.org/techniques/T1546/015), [T1546.016](https://attack.mitre.org/techniques/T1546/016), [T1546.017](https://attack.mitre.org/techniques/T1546/017), [T1546.018](https://attack.mitre.org/techniques/T1546/018)

ⓘ

Tactics: [Privilege Escalation](https://attack.mitre.org/tactics/TA0004), [Persistence](https://attack.mitre.org/tactics/TA0003)

ⓘ

Platforms: IaaS, Linux, Office Suite, SaaS, Windows, macOS

Version: 1.4

Created: 22 January 2020

Last Modified: 04 August 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1546/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1546/)

## Procedure Examples

| ID  | Name  | Description

|  [C0035](https://attack.mitre.org/campaigns/C0035)   |  [KV Botnet Activity](https://attack.mitre.org/campaigns/C0035)   |

[KV Botnet Activity](https://attack.mitre.org/campaigns/C0035) involves managing events on victim systems via libevent to execute a callback function when any running process contains the following references in their path without also having a reference to bioset: busybox, wget, curl, tftp, telnetd, or lua. If the bioset string is not found, the related process is terminated.[[7]](https://blog.lumen.com/routers-roasting-on-an-open-firewall-the-kv-botnet-investigation/)

|  [S9043](https://attack.mitre.org/software/S9043)   |  [Mini Shai-Hulud](https://attack.mitre.org/software/S9043)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has modified settings and configuration files of AI coding agents and other coding applications in order to create event triggered executions through creating hooks and runOn conditions.[[8]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  [S1091](https://attack.mitre.org/software/S1091)   |  [Pacu](https://attack.mitre.org/software/S1091)   |

[Pacu](https://attack.mitre.org/software/S1091) can set up S3 bucket notifications to trigger a malicious Lambda function when a CloudFormation template is uploaded to the bucket. It can also create Lambda functions that trigger upon the creation of users, roles, and groups.[[9]](https://github.com/RhinoSecurityLabs/pacu)

|  [S1164](https://attack.mitre.org/software/S1164)   |  [UPSTYLE](https://attack.mitre.org/software/S1164)   |

[UPSTYLE](https://attack.mitre.org/software/S1164) creates a .pth file beginning with the text import so that any time another process or script attempts to reference the modified item the malicious code will also run.[[10]](https://www.volexity.com/blog/2024/04/12/zero-day-exploitation-of-unauthenticated-remote-code-execution-vulnerability-in-globalprotect-cve-2024-3400/)

|  [S0658](https://attack.mitre.org/software/S0658)   |  [XCSSET](https://attack.mitre.org/software/S0658)   |

[XCSSET](https://attack.mitre.org/software/S0658)'s dfhsebxzod module searches for .xcodeproj directories within the user’s home folder and subdirectories. For each match, it locates the corresponding project.pbxproj file and embeds an encoded payload into a build rule, target configuration, or project setting. The payload is later executed during the build process.[[11]](https://www.microsoft.com/en-us/security/blog/2025/03/11/new-xcsset-malware-adds-new-obfuscation-persistence-techniques-to-infect-xcode-projects/)[[12]](https://www.trendmicro.com/en_us/research/21/d/xcsset-quickly-adapts-to-macos-11-and-m1-based-macs.html)

## Mitigations

| ID  | Mitigation  | Description

|  [M1026](https://attack.mitre.org/mitigations/M1026)   |  [Privileged Account Management](https://attack.mitre.org/mitigations/M1026)   |

Manage the creation, modification, use, and permissions associated to privileged accounts, including SYSTEM and root.

|  [M1051](https://attack.mitre.org/mitigations/M1051)   |  [Update Software](https://attack.mitre.org/mitigations/M1051)   |

Perform regular software updates to mitigate exploitation risk.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0010](https://attack.mitre.org/detectionstrategies/DET0010)   |  [Behavioral Detection of Event Triggered Execution Across Platforms](https://attack.mitre.org/detectionstrategies/DET0010)   |  [AN0024](https://attack.mitre.org/detectionstrategies/DET0010#AN0024)   |

Correlates unexpected modifications to WMI event filters, scheduled task triggers, or registry autorun keys with subsequent execution of non-standard binaries by SYSTEM-level processes.

|  [AN0025](https://attack.mitre.org/detectionstrategies/DET0010#AN0025)   |

Detects inotify or auditd configuration changes that monitor system files coupled with execution of script interpreters or binaries by cron or systemd timers.

|  [AN0026](https://attack.mitre.org/detectionstrategies/DET0010#AN0026)   |

Correlates launchd plist modifications with subsequent unauthorized script execution or anomalous parent-child process trees involving user agents.

|  [AN0027](https://attack.mitre.org/detectionstrategies/DET0010#AN0027)   |

Monitors cloud function creation triggered by specific audit log events (e.g., IAM changes, object creation), followed by anomalous behavior from new service accounts.

|  [AN0028](https://attack.mitre.org/detectionstrategies/DET0010#AN0028)   |

Correlates Power Automate or similar logic app workflows triggered by SaaS file uploads or email rules with data forwarding or anomalous access patterns.

|  [AN0029](https://attack.mitre.org/detectionstrategies/DET0010#AN0029)   |

Detects macros or VBA triggers set to execute on document open or close events, often correlating with embedded payloads or C2 traffic shortly after execution.

## References

-    [Daniel Grzelak. (2016, July 9). Backdooring an AWS account. Retrieved May 27, 2022.](https://medium.com/daniel-grzelak/backdooring-an-aws-account-da007d36f8f9)
-    [Eric Saraga. (2022, February 2). Using Power Automate for Covert Data Exfiltration in Microsoft 365. Retrieved May 27, 2022.](https://www.varonis.com/blog/power-automate-data-exfiltration)
-    [Berk Veral. (2020, March 9). Real-life cybercrime stories from DART, the Microsoft Detection and Response Team. Retrieved May 27, 2022.](https://www.microsoft.com/security/blog/2020/03/09/real-life-cybercrime-stories-dart-microsoft-detection-and-response-team)
-    [Ballenthin, W., et al. (2015). Windows Management Instrumentation (WMI) Offense, Defense, and Forensics. Retrieved March 30, 2016.](https://www.fireeye.com/content/dam/fireeye-www/global/en/current-threats/pdfs/wp-windows-management-instrumentation.pdf)
-    [Patrick Wardle. (2015). Malware Persistence on OS X Yosemite. Retrieved July 10, 2017.](https://www.virusbulletin.com/uploads/pdf/conference/vb2014/VB2014-Wardle.pdf)
-    [Claud Xiao, Cong Zheng, Yanhui Jia. (2017, April 6). New IoT/Linux Malware Targets DVRs, Forms Botnet. Retrieved February 19, 2018.](https://researchcenter.paloaltonetworks.com/2017/04/unit42-new-iotlinux-malware-targets-dvrs-forms-botnet/)

-    [Black Lotus Labs. (2023, December 13). Routers Roasting On An Open Firewall: The KV-Botnet Investigation. Retrieved June 10, 2024.](https://blog.lumen.com/routers-roasting-on-an-open-firewall-the-kv-botnet-investigation/)
-    [Flashpoint. (2026, May 28). The Mini Shai-Hulud Worm and the New Era of CI/CD Exploitation. Retrieved July 16, 2026.](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)
-    [Rhino Security Labs. (2019, August 22). Pacu. Retrieved October 17, 2019.](https://github.com/RhinoSecurityLabs/pacu)
-    [Volexity Threat Research. (2024, April 12). Zero-Day Exploitation of Unauthenticated Remote Code Execution Vulnerability in GlobalProtect (CVE-2024-3400). Retrieved November 20, 2024.](https://www.volexity.com/blog/2024/04/12/zero-day-exploitation-of-unauthenticated-remote-code-execution-vulnerability-in-globalprotect-cve-2024-3400/)
-    [Microsoft Threat Intelligence. (2025, March 11). New XCSSET malware adds new obfuscation, persistence techniques to infect Xcode projects. Retrieved April 2, 2025.](https://www.microsoft.com/en-us/security/blog/2025/03/11/new-xcsset-malware-adds-new-obfuscation-persistence-techniques-to-infect-xcode-projects/)
-    [Steven Du, Dechao Zhao, Luis Magisa, Ariel Neimond Lazaro. (2021, April 16). XCSSET Quickly Adapts to macOS 11 and M1-based Macs. Retrieved February 18, 2025.](https://www.trendmicro.com/en_us/research/21/d/xcsset-quickly-adapts-to-macos-11-and-m1-based-macs.html)

×
