---
url: "https://attack.mitre.org/techniques/T1546/001/"
title: "Event Triggered Execution: Change Default File Association, Sub-technique T1546.001 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:47:36Z"
via: "live"
sha256: "e45d753f9133ba03b055b2e742db1085d020c2051037c7cbf629931f91a19a3e"
wayback: "https://web.archive.org/web/20261023090801/https://attack.mitre.org/techniques/T1546/001/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Event Triggered Execution: Change Default File Association, Sub-technique T1546.001 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1546/001/>
> Snapshot: <https://web.archive.org/web/20261023090801/https://attack.mitre.org/techniques/T1546/001/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Event Triggered Execution](https://attack.mitre.org/techniques/T1546)
- Change Default File Association

#  Event Triggered Execution: Change Default File Association

##### Other sub-techniques of Event Triggered Execution (18)

| ID  | Name

|  T1546.001   |  Change Default File Association

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

Adversaries may establish persistence by executing malicious content triggered by a file type association. When a file is opened, the default program used to open the file (also called the file association or handler) is checked. File association selections are stored in the Windows Registry and can be edited by users, administrators, or programs that have Registry access or by administrators using the built-in assoc utility.[[1]](https://support.microsoft.com/en-us/help/18539/windows-7-change-default-programs)[[2]](https://learn.microsoft.com/en-us/previous-versions/visualstudio/visual-studio-2015/extensibility/specifying-file-handlers-for-file-name-extensions?view=vs-2015)[[3]](https://docs.microsoft.com/windows-server/administration/windows-commands/assoc) Applications can modify the file association for a given file extension to call an arbitrary program when a file with the given extension is opened.

System file associations are listed under HKEY_CLASSES_ROOT.[extension], for example HKEY_CLASSES_ROOT.txt. The entries point to a handler for that extension located at HKEY_CLASSES_ROOT\[handler]. The various commands are then listed as subkeys underneath the shell key at HKEY_CLASSES_ROOT\[handler]\shell\[action]\command. For example:

- HKEY_CLASSES_ROOT\txtfile\shell\open\command
- HKEY_CLASSES_ROOT\txtfile\shell\print\command
- HKEY_CLASSES_ROOT\txtfile\shell\printto\command

The values of the keys listed are commands that are executed when the handler opens the file extension. Adversaries can modify these values to continually execute arbitrary commands.[[4]](https://www.trendmicro.com/vinfo/us/threat-encyclopedia/malware/troj_fakeav.gzd)

ID: T1546.001

Sub-technique of:  [T1546](https://attack.mitre.org/techniques/T1546)

ⓘ

Tactics: [Privilege Escalation](https://attack.mitre.org/tactics/TA0004), [Persistence](https://attack.mitre.org/tactics/TA0003)

ⓘ

Platforms: Windows

Contributors: Stefan Kanthak; Travis Smith, Tripwire

Version: 1.1

Created: 24 January 2020

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1546/001/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1546/001/)

## Procedure Examples

| ID  | Name  | Description

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has a HWP document stealer module which changes the default program association in the registry to open HWP documents.[[5]](https://securelist.com/the-kimsuky-operation-a-north-korean-apt/57915/)

|  [S0692](https://attack.mitre.org/software/S0692)   |  [SILENTTRINITY](https://attack.mitre.org/software/S0692)   |

[SILENTTRINITY](https://attack.mitre.org/software/S0692) can conduct an image hijack of an .msc file extension as part of its UAC bypass process.[[6]](https://github.com/byt3bl33d3r/SILENTTRINITY/tree/master/silenttrinity/core/teamserver/modules/boo)

## Mitigations

This type of attack technique cannot be easily mitigated with preventive controls since it is based on the abuse of system features.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0061](https://attack.mitre.org/detectionstrategies/DET0061)   |  [Detect Default File Association Hijack via Registry & Execution Correlation on Windows](https://attack.mitre.org/detectionstrategies/DET0061)   |  [AN0170](https://attack.mitre.org/detectionstrategies/DET0061#AN0170)   |

Detects modification of registry keys used for default file handlers, followed by anomalous process execution from user-initiated file opens. This includes tracking changes under HKCU and HKCR for file extension mappings, and correlating them with new or suspicious handler paths launching unusual child processes (e.g., PowerShell, cmd, wscript).

## References

-    [Microsoft. (n.d.). Change which programs Windows 7 uses by default. Retrieved July 26, 2016.](https://support.microsoft.com/en-us/help/18539/windows-7-change-default-programs)
-    [Microsoft. (n.d.). Specifying File Handlers for File Name Extensions. Retrieved September 12, 2024.](https://learn.microsoft.com/en-us/previous-versions/visualstudio/visual-studio-2015/extensibility/specifying-file-handlers-for-file-name-extensions?view=vs-2015)
-    [Plett, C. et al.. (2017, October 15). assoc. Retrieved August 7, 2018.](https://docs.microsoft.com/windows-server/administration/windows-commands/assoc)

-    [Sioting, S. (2012, October 8). TROJ_FAKEAV.GZD. Retrieved August 8, 2018.](https://www.trendmicro.com/vinfo/us/threat-encyclopedia/malware/troj_fakeav.gzd)
-    [Tarakanov , D.. (2013, September 11). The “Kimsuky” Operation: A North Korean APT?. Retrieved August 13, 2019.](https://securelist.com/the-kimsuky-operation-a-north-korean-apt/57915/)
-    [Salvati, M. (2019, August 6). SILENTTRINITY Modules. Retrieved March 24, 2022.](https://github.com/byt3bl33d3r/SILENTTRINITY/tree/master/silenttrinity/core/teamserver/modules/boo)

×
