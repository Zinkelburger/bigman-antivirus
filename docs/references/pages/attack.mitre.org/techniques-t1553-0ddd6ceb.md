---
url: "https://attack.mitre.org/techniques/T1553/"
title: "Subvert Trust Controls, Technique T1553 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:52:54Z"
via: "live"
sha256: "e476a57976460e124694dcc637816c4e57326d748a9b2e28bb6566878bd489d4"
wayback: "https://web.archive.org/web/20261023090820/https://attack.mitre.org/techniques/T1553/"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# Subvert Trust Controls, Technique T1553 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1553/>
> Snapshot: <https://web.archive.org/web/20261023090820/https://attack.mitre.org/techniques/T1553/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Subvert Trust Controls

#  Subvert Trust Controls

##### Sub-techniques (6)

| ID  | Name

|  [T1553.001](https://attack.mitre.org/techniques/T1553/001/)   |  [Gatekeeper Bypass](https://attack.mitre.org/techniques/T1553/001/)

|  [T1553.002](https://attack.mitre.org/techniques/T1553/002/)   |  [Code Signing](https://attack.mitre.org/techniques/T1553/002/)

|  [T1553.003](https://attack.mitre.org/techniques/T1553/003/)   |  [SIP and Trust Provider Hijacking](https://attack.mitre.org/techniques/T1553/003/)

|  [T1553.004](https://attack.mitre.org/techniques/T1553/004/)   |  [Install Root Certificate](https://attack.mitre.org/techniques/T1553/004/)

|  [T1553.005](https://attack.mitre.org/techniques/T1553/005/)   |  [Mark-of-the-Web Bypass](https://attack.mitre.org/techniques/T1553/005/)

|  [T1553.006](https://attack.mitre.org/techniques/T1553/006/)   |  [Code Signing Policy Modification](https://attack.mitre.org/techniques/T1553/006/)

Adversaries may undermine security controls that will either warn users of untrusted activity or prevent execution of untrusted programs. Operating systems and security products may contain mechanisms to identify programs or websites as possessing some level of trust. Examples of such features would include a program being allowed to run because it is signed by a valid code signing certificate, a program prompting the user with a warning because it has an attribute set from being downloaded from the Internet, or getting an indication that you are about to connect to an untrusted site.

Adversaries may attempt to subvert these trust mechanisms. The method adversaries use will depend on the specific mechanism they seek to subvert. Adversaries may conduct [File and Directory Permissions Modification](https://attack.mitre.org/techniques/T1222) or [Modify Registry](https://attack.mitre.org/techniques/T1112) in support of subverting these controls.[[1]](https://specterops.io/assets/resources/SpecterOps_Subverting_Trust_in_Windows.pdf) Adversaries may also create or steal code signing certificates to acquire trust on target systems.[[2]](https://securelist.com/why-you-shouldnt-completely-trust-files-signed-with-digital-certificates/68593/)[[3]](http://www.symantec.com/connect/blogs/how-attackers-steal-private-keys-digital-certificates)

ID: T1553

Sub-techniques:  [T1553.001](https://attack.mitre.org/techniques/T1553/001), [T1553.002](https://attack.mitre.org/techniques/T1553/002), [T1553.003](https://attack.mitre.org/techniques/T1553/003), [T1553.004](https://attack.mitre.org/techniques/T1553/004), [T1553.005](https://attack.mitre.org/techniques/T1553/005), [T1553.006](https://attack.mitre.org/techniques/T1553/006)

ⓘ

Tactic: [Defense Impairment](https://attack.mitre.org/tactics/TA0112)

ⓘ

Platforms: Linux, Windows, macOS

Version: 2.0

Created: 05 February 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1553/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1553/)

## Procedure Examples

| ID  | Name  | Description

|  [G0001](https://attack.mitre.org/groups/G0001)   |  [Axiom](https://attack.mitre.org/groups/G0001)   |

[Axiom](https://attack.mitre.org/groups/G0001) has used digital certificates to deliver malware.[[4]](https://web.archive.org/web/20230115144216/http://www.novetta.com/wp-content/uploads/2014/11/Executive_Summary-Final_1.pdf)

|  [S9008](https://attack.mitre.org/software/S9008)   |  [Shai-Hulud](https://attack.mitre.org/software/S9008)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has suppressed victim NPM warnings using process["exit’](0x0); which results in having all errors exit with code 0.[[5]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

## Mitigations

| ID  | Mitigation  | Description

|  [M1038](https://attack.mitre.org/mitigations/M1038)   |  [Execution Prevention](https://attack.mitre.org/mitigations/M1038)   |

System settings can prevent applications from running that haven't been downloaded through the Apple Store (or other legitimate repositories) which can help mitigate some of these issues. Also enable application control solutions such as AppLocker and/or Device Guard to block the loading of malicious content.

|  [M1028](https://attack.mitre.org/mitigations/M1028)   |  [Operating System Configuration](https://attack.mitre.org/mitigations/M1028)   |

Windows Group Policy can be used to manage root certificates and the Flags value of HKLM\SOFTWARE\Policies\Microsoft\SystemCertificates\Root\ProtectedRoots can be set to 1 to prevent non-administrator users from making further root installations into their own HKCU certificate store. [[6]](https://posts.specterops.io/code-signing-certificate-cloning-attacks-and-defenses-6f98657fc6ec)

|  [M1026](https://attack.mitre.org/mitigations/M1026)   |  [Privileged Account Management](https://attack.mitre.org/mitigations/M1026)   |

Manage the creation, modification, use, and permissions associated to privileged accounts, including SYSTEM and root.

|  [M1024](https://attack.mitre.org/mitigations/M1024)   |  [Restrict Registry Permissions](https://attack.mitre.org/mitigations/M1024)   |

Ensure proper permissions are set for Registry hives to prevent users from modifying keys related to SIP and trust provider components. Components may still be able to be hijacked to suitable functions already present on disk if malicious modifications to Registry keys are not prevented.

|  [M1054](https://attack.mitre.org/mitigations/M1054)   |  [Software Configuration](https://attack.mitre.org/mitigations/M1054)   |

HTTP Public Key Pinning (HPKP) is one method to mitigate potential [Adversary-in-the-Middle](https://attack.mitre.org/techniques/T1557) situations where and adversary uses a mis-issued or fraudulent certificate to intercept encrypted communications by enforcing use of an expected certificate. [[7]](https://en.wikipedia.org/wiki/HTTP_Public_Key_Pinning)

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0452](https://attack.mitre.org/detectionstrategies/DET0452)   |  [Detect Subversion of Trust Controls via Certificate, Registry, and Attribute Manipulation](https://attack.mitre.org/detectionstrategies/DET0452)   |  [AN1246](https://attack.mitre.org/detectionstrategies/DET0452#AN1246)   |

Detection correlates abnormal installation or modification of root or code-signing certificates, creation/modification of suspicious registry keys for trust providers, and unusual module loads from non-standard locations. Identifies unsigned or improperly signed executables bypassing trust prompts, combined with persistence artifacts.

|  [AN1247](https://attack.mitre.org/detectionstrategies/DET0452#AN1247)   |

Detection monitors extended attribute manipulation (xattr) to strip quarantine or trust metadata, anomalous installation of root certificates in /etc/ssl or /usr/local/share/ca-certificates, and unauthorized modification of system trust stores. Correlates with unexpected process execution involving package managers or custom certificate utilities.

|  [AN1248](https://attack.mitre.org/detectionstrategies/DET0452#AN1248)   |

Detection monitors modification of code signing attributes, Gatekeeper/quarantine flags, and insertion of new trust certificates via security add-trusted-cert. Identifies adversary use of xattr to strip quarantine flags from downloaded binaries. Correlates with abnormal module loads bypassing SIP protections.

## References

-    [Graeber, M. (2017, September). Subverting Trust in Windows. Retrieved January 31, 2018.](https://specterops.io/assets/resources/SpecterOps_Subverting_Trust_in_Windows.pdf)
-    [Ladikov, A. (2015, January 29). Why You Shouldn’t Completely Trust Files Signed with Digital Certificates. Retrieved March 31, 2016.](https://securelist.com/why-you-shouldnt-completely-trust-files-signed-with-digital-certificates/68593/)
-    [Shinotsuka, H. (2013, February 22). How Attackers Steal Private Keys from Digital Certificates. Retrieved March 31, 2016.](http://www.symantec.com/connect/blogs/how-attackers-steal-private-keys-digital-certificates)
-    [Novetta. (n.d.). Operation SMN: Axiom Threat Actor Group Report. Retrieved November 12, 2014.](https://web.archive.org/web/20230115144216/http://www.novetta.com/wp-content/uploads/2014/11/Executive_Summary-Final_1.pdf)

-    [Socket Research Team. (2025, November 24). Shai Hulud Strikes Again (v2). Retrieved April 9, 2026.](https://socket.dev/blog/shai-hulud-strikes-again-v2)
-    [Graeber, M. (2017, December 22). Code Signing Certificate Cloning Attacks and Defenses. Retrieved April 3, 2018.](https://posts.specterops.io/code-signing-certificate-cloning-attacks-and-defenses-6f98657fc6ec)
-    [Wikipedia. (2017, February 28). HTTP Public Key Pinning. Retrieved March 31, 2017.](https://en.wikipedia.org/wiki/HTTP_Public_Key_Pinning)

×
