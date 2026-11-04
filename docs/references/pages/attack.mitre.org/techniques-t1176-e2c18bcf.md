---
url: "https://attack.mitre.org/techniques/T1176/"
title: "Software Extensions, Technique T1176 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:36:19Z"
via: "live"
sha256: "7903dfdc52db0be32eff03c4e78b22d1e515eb9d7598935f5e721f37ef388379"
wayback: "https://web.archive.org/web/20261016170425/https://attack.mitre.org/techniques/T1176/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Software Extensions, Technique T1176 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1176/>
> Snapshot: <https://web.archive.org/web/20261016170425/https://attack.mitre.org/techniques/T1176/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Software Extensions

#  Software Extensions

##### Sub-techniques (2)

| ID  | Name

|  [T1176.001](https://attack.mitre.org/techniques/T1176/001/)   |  [Browser Extensions](https://attack.mitre.org/techniques/T1176/001/)

|  [T1176.002](https://attack.mitre.org/techniques/T1176/002/)   |  [IDE Extensions](https://attack.mitre.org/techniques/T1176/002/)

Adversaries may abuse software extensions to establish persistent access to victim systems. Software extensions are modular components that enhance or customize the functionality of software applications, including web browsers, Integrated Development Environments (IDEs), and other platforms.[[1]](https://web.archive.org/web/20240608001937/https://kjaer.io/extension-malware/)[[2]](https://blog.checkpoint.com/securing-the-cloud/malicious-vscode-extensions-with-more-than-45k-downloads-steal-pii-and-enable-backdoors/) Extensions are typically installed via official marketplaces, app stores, or manually loaded by users, and they often inherit the permissions and access levels of the host application.

Malicious extensions can be introduced through various methods, including social engineering, compromised marketplaces, or direct installation by users or by adversaries who have already gained access to a system. Malicious extensions can be named similarly or identically to benign extensions in marketplaces. Security mechanisms in extension marketplaces may be insufficient to detect malicious components, allowing adversaries to bypass automated scanners or exploit trust established during the installation process. Adversaries may also abuse benign extensions to achieve their objectives, such as using legitimate functionality to tunnel data or bypass security controls.

The modular nature of extensions and their integration with host applications make them an attractive target for adversaries seeking to exploit trusted software ecosystems. Detection can be challenging due to the inherent trust placed in extensions during installation and their ability to blend into normal application workflows.

ID: T1176

Sub-techniques:  [T1176.001](https://attack.mitre.org/techniques/T1176/001), [T1176.002](https://attack.mitre.org/techniques/T1176/002)

ⓘ

Tactic: [Persistence](https://attack.mitre.org/tactics/TA0003)

ⓘ

Platforms: Linux, Windows, macOS

Contributors: Chris Ross @xorrior; Justin Warner, ICEBRG; Manikantan Srinivasan, NEC Corporation India

Version: 2.0

Created: 16 January 2018

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1176/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1176/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1047](https://attack.mitre.org/mitigations/M1047)   |  [Audit](https://attack.mitre.org/mitigations/M1047)   |

Ensure extensions that are installed are the intended ones, as many malicious extensions may masquerade as legitimate ones.

|  [M1038](https://attack.mitre.org/mitigations/M1038)   |  [Execution Prevention](https://attack.mitre.org/mitigations/M1038)   |

Set an extension allow or deny list as appropriate for your security policy.

|  [M1033](https://attack.mitre.org/mitigations/M1033)   |  [Limit Software Installation](https://attack.mitre.org/mitigations/M1033)   |

Only install extensions from trusted sources that can be verified.

|  [M1051](https://attack.mitre.org/mitigations/M1051)   |  [Update Software](https://attack.mitre.org/mitigations/M1051)   |

Ensure operating systems and software are using the most current version.

|  [M1017](https://attack.mitre.org/mitigations/M1017)   |  [User Training](https://attack.mitre.org/mitigations/M1017)   |

Train users to minimize extension use, and to only install trusted extensions.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0092](https://attack.mitre.org/detectionstrategies/DET0092)   |  [Detection of Malicious or Unauthorized Software Extensions](https://attack.mitre.org/detectionstrategies/DET0092)   |  [AN0251](https://attack.mitre.org/detectionstrategies/DET0092#AN0251)   |

Installation or execution of a malicious browser or IDE extension, followed by abnormal registry entries or outbound network connections from the host application

|  [AN0252](https://attack.mitre.org/detectionstrategies/DET0092#AN0252)   |

Installation of configuration profiles or plist entries associated with malicious or unauthorized browser extensions

|  [AN0253](https://attack.mitre.org/detectionstrategies/DET0092#AN0253)   |

Manual or script-based installation of extension-like modules into browser config directories or IDE plugin paths, followed by suspicious network activity

## References

-    [Kjaer, M. (2016, July 18). Malware in the browser: how you might get hacked by a Chrome extension. Retrieved September 12, 2024.](https://web.archive.org/web/20240608001937/https://kjaer.io/extension-malware/)

-    [Abramovsky, O. (2023, May 16). VSCode Security: Malicious Extensions Detected- More Than 45,000 Downloads- PII Exposed, and Backdoors Enabled. Retrieved March 30, 2025.](https://blog.checkpoint.com/securing-the-cloud/malicious-vscode-extensions-with-more-than-45k-downloads-steal-pii-and-enable-backdoors/)

×
