---
url: "https://attack.mitre.org/software/S1219/"
title: "REPTILE, Software S1219 | MITRE ATT&CK®"
captured: "2026-11-02T18:22:55Z"
via: "live"
sha256: "c30ad5f49782f6f594f633c0595dfe052aa9f7e37d017195b625448da4799286"
wayback: "https://web.archive.org/web/20261003211845/https://attack.mitre.org/software/S1219/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# REPTILE, Software S1219 | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/software/S1219/>
> Snapshot: <https://web.archive.org/web/20261003211845/https://attack.mitre.org/software/S1219/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Software](https://attack.mitre.org/software/)
- REPTILE

#  REPTILE

[REPTILE](https://attack.mitre.org/software/S1219) is an open-source Linux rootkit with multiple components that provides backdoor access and functionality.[[1]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

ID: S1219

ⓘ

Type: MALWARE

ⓘ

Platforms: Linux

Version: 1.0

Created: 09 June 2025

Last Modified: 09 June 2025

[Version Permalink](https://attack.mitre.org/versions/v20/software/S1219/)

[Live Version](https://attack.mitre.org/versions/v20/software/S1219/)

###### Enterprise Layer

[download](https://attack.mitre.org/software/S1219/S1219-enterprise-layer.json)  [view](https://attack.mitre.org/software/S1219/#)

## Techniques Used

| Domain  | ID  | Name  | Use

|  Enterprise   |  [T1547](https://attack.mitre.org/techniques/T1547)   |  [.006](https://attack.mitre.org/techniques/T1547/006)   |  [Boot or Logon Autostart Execution](https://attack.mitre.org/techniques/T1547): [Kernel Modules and Extensions](https://attack.mitre.org/techniques/T1547/006)   |

The [REPTILE](https://attack.mitre.org/software/S1219) rootkit is implemented as a loadable kernel module (LKM).[[1]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

|  Enterprise   |  [T1059](https://attack.mitre.org/techniques/T1059)   |  [.004](https://attack.mitre.org/techniques/T1059/004)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [Unix Shell](https://attack.mitre.org/techniques/T1059/004)   |

[REPTILE](https://attack.mitre.org/software/S1219) can deploy components automatically with shell scripts.[[1]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

|  Enterprise   |  [T1543](https://attack.mitre.org/techniques/T1543)   |  [.004](https://attack.mitre.org/techniques/T1543/004)   |  [Create or Modify System Process](https://attack.mitre.org/techniques/T1543): [Launch Daemon](https://attack.mitre.org/techniques/T1543/004)   |

The [REPTILE](https://attack.mitre.org/software/S1219) launcher can daemonize a process.[[1]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

|  Enterprise   |  [T1140](https://attack.mitre.org/techniques/T1140)   |  [Deobfuscate/Decode Files or Information](https://attack.mitre.org/techniques/T1140)   |

The [REPTILE](https://attack.mitre.org/software/S1219) launcher component can decrypt kernel module code from a file and load it into memory.[[1]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

|  Enterprise   |  [T1573](https://attack.mitre.org/techniques/T1573)   |  [.002](https://attack.mitre.org/techniques/T1573/002)   |  [Encrypted Channel](https://attack.mitre.org/techniques/T1573): [Asymmetric Cryptography](https://attack.mitre.org/techniques/T1573/002)   |

[REPTILE](https://attack.mitre.org/software/S1219) can use TLS over raw TCP for secure C2.[[1]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)[[2]](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)

|  Enterprise   |  [T1546](https://attack.mitre.org/techniques/T1546)   |  [.017](https://attack.mitre.org/techniques/T1546/017)   |  [Event Triggered Execution](https://attack.mitre.org/techniques/T1546): [Udev Rules](https://attack.mitre.org/techniques/T1546/017)   |

[REPTILE](https://attack.mitre.org/software/S1219) has used udev for persistence.[[1]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

|  Enterprise   |  [T1564](https://attack.mitre.org/techniques/T1564)   |  [.001](https://attack.mitre.org/techniques/T1564/001)   |  [Hide Artifacts](https://attack.mitre.org/techniques/T1564): [Hidden Files and Directories](https://attack.mitre.org/techniques/T1564/001)   |

[REPTILE](https://attack.mitre.org/software/S1219) has the ability to communicate with the kernel-mode component to hide files.[[1]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

|  Enterprise   |  [T1095](https://attack.mitre.org/techniques/T1095)   |  [Non-Application Layer Protocol](https://attack.mitre.org/techniques/T1095)   |

[REPTILE](https://attack.mitre.org/software/S1219) can communicate using TLS over raw TCP.[[1]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)[[2]](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)

|  Enterprise   |  [T1014](https://attack.mitre.org/techniques/T1014)   |  [Rootkit](https://attack.mitre.org/techniques/T1014)   |

[REPTILE](https://attack.mitre.org/software/S1219) has the ability to hook kernel functions and modify functions data to achieve rootkit functionality such as hiding processes and network connections.[[1]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

|  Enterprise   |  [T1205](https://attack.mitre.org/techniques/T1205)   |  [Traffic Signaling](https://attack.mitre.org/techniques/T1205)   |

The [REPTILE](https://attack.mitre.org/software/S1219) reverse shell component can listen for a specialized packet in TCP, UDP, or ICMP for activation.[[1]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)[[2]](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)

|   |   |  [.001](https://attack.mitre.org/techniques/T1205/001)   |  [Port Knocking](https://attack.mitre.org/techniques/T1205/001)   |

[REPTILE](https://attack.mitre.org/software/S1219) has the ability to control compromised endpoints via port knocking.[[1]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

## Groups That Use This Software

| ID  | Name  | References

|  [G1048](https://attack.mitre.org/groups/G1048)   |  [UNC3886](https://attack.mitre.org/groups/G1048)   |

[[1]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)[[3]](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-targets-juniper-routers)

## Campaigns

| ID  | Name  | Description

|  [C0056](https://attack.mitre.org/campaigns/C0056)   |  [RedPenguin](https://attack.mitre.org/campaigns/C0056)   |

[REPTILE](https://attack.mitre.org/software/S1219) was used for command execution and persistence during [RedPenguin](https://attack.mitre.org/campaigns/C0056).[[3]](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-targets-juniper-routers)

## References

-    [Punsaen Boonyakarn, Shawn Chew, Logeswaran Nadarajan, Mathew Potaczek, Jakub Jozwiak, and Alex Marvi. (2024, June 18). Cloaked and Covert: Uncovering UNC3886 Espionage Operations. Retrieved September 24, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)
-    [Marvi, A. et al.. (2023, March 16). Fortinet Zero-Day and Custom Malware Used by Suspected Chinese Actor in Espionage Operation. Retrieved March 22, 2023.](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)

-    [Lamparski, L. et al. (2025, March 11). Ghost in the Router: China-Nexus Espionage Actor UNC3886 Targets Juniper Routers. Retrieved June 24, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-targets-juniper-routers)

×
