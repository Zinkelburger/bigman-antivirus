---
url: "https://attack.mitre.org/software/S0502/"
title: "Drovorub, Software S0502 | MITRE ATT&CK®"
captured: "2026-11-02T18:22:51Z"
via: "live"
sha256: "36a6ed555923936bc8c4f2337325cd0e97503244eb81e49a44c94074b188a277"
wayback: "https://web.archive.org/web/20261006103404/https://attack.mitre.org/software/S0502/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Drovorub, Software S0502 | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/software/S0502/>
> Snapshot: <https://web.archive.org/web/20261006103404/https://attack.mitre.org/software/S0502/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Software](https://attack.mitre.org/software/)
- Drovorub

#  Drovorub

[Drovorub](https://attack.mitre.org/software/S0502) is a Linux malware toolset comprised of an agent, client, server, and kernel modules, that has been used by [APT28](https://attack.mitre.org/groups/G0007).[[1]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

ID: S0502

ⓘ

Type: MALWARE

ⓘ

Platforms: Linux

Version: 1.0

Created: 25 August 2020

Last Modified: 25 April 2025

[Version Permalink](https://attack.mitre.org/versions/v20/software/S0502/)

[Live Version](https://attack.mitre.org/versions/v20/software/S0502/)

###### Enterprise Layer

[download](https://attack.mitre.org/software/S0502/S0502-enterprise-layer.json)  [view](https://attack.mitre.org/software/S0502/#)

## Techniques Used

| Domain  | ID  | Name  | Use

|  Enterprise   |  [T1071](https://attack.mitre.org/techniques/T1071)   |  [.001](https://attack.mitre.org/techniques/T1071/001)   |  [Application Layer Protocol](https://attack.mitre.org/techniques/T1071): [Web Protocols](https://attack.mitre.org/techniques/T1071/001)   |

[Drovorub](https://attack.mitre.org/software/S0502) can use the WebSocket protocol and has initiated communication with C2 servers with an HTTP Upgrade request.[[1]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

|  Enterprise   |  [T1547](https://attack.mitre.org/techniques/T1547)   |  [.006](https://attack.mitre.org/techniques/T1547/006)   |  [Boot or Logon Autostart Execution](https://attack.mitre.org/techniques/T1547): [Kernel Modules and Extensions](https://attack.mitre.org/techniques/T1547/006)   |

[Drovorub](https://attack.mitre.org/software/S0502) can use kernel modules to establish persistence.[[1]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

|  Enterprise   |  [T1059](https://attack.mitre.org/techniques/T1059)   |  [.004](https://attack.mitre.org/techniques/T1059/004)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [Unix Shell](https://attack.mitre.org/techniques/T1059/004)   |

[Drovorub](https://attack.mitre.org/software/S0502) can execute arbitrary commands as root on a compromised system.[[1]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

|  Enterprise   |  [T1005](https://attack.mitre.org/techniques/T1005)   |  [Data from Local System](https://attack.mitre.org/techniques/T1005)   |

[Drovorub](https://attack.mitre.org/software/S0502) can transfer files from the victim machine.[[1]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

|  Enterprise   |  [T1140](https://attack.mitre.org/techniques/T1140)   |  [Deobfuscate/Decode Files or Information](https://attack.mitre.org/techniques/T1140)   |

[Drovorub](https://attack.mitre.org/software/S0502) has de-obsfuscated XOR encrypted payloads in WebSocket messages.[[1]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

|  Enterprise   |  [T1041](https://attack.mitre.org/techniques/T1041)   |  [Exfiltration Over C2 Channel](https://attack.mitre.org/techniques/T1041)   |

[Drovorub](https://attack.mitre.org/software/S0502) can exfiltrate files over C2 infrastructure.[[1]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

|  Enterprise   |  [T1070](https://attack.mitre.org/techniques/T1070)   |  [.004](https://attack.mitre.org/techniques/T1070/004)   |  [Indicator Removal](https://attack.mitre.org/techniques/T1070): [File Deletion](https://attack.mitre.org/techniques/T1070/004)   |

[Drovorub](https://attack.mitre.org/software/S0502) can delete specific files from a compromised host.[[1]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

|  Enterprise   |  [T1105](https://attack.mitre.org/techniques/T1105)   |  [Ingress Tool Transfer](https://attack.mitre.org/techniques/T1105)   |

[Drovorub](https://attack.mitre.org/software/S0502) can download files to a compromised host.[[1]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

|  Enterprise   |  [T1095](https://attack.mitre.org/techniques/T1095)   |  [Non-Application Layer Protocol](https://attack.mitre.org/techniques/T1095)   |

[Drovorub](https://attack.mitre.org/software/S0502) can use TCP to communicate between its agent and client modules.[[1]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

|  Enterprise   |  [T1027](https://attack.mitre.org/techniques/T1027)   |  [Obfuscated Files or Information](https://attack.mitre.org/techniques/T1027)   |

[Drovorub](https://attack.mitre.org/software/S0502) has used XOR encrypted payloads in WebSocket client to server messages.[[1]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

|  Enterprise   |  [T1090](https://attack.mitre.org/techniques/T1090)   |  [.001](https://attack.mitre.org/techniques/T1090/001)   |  [Proxy](https://attack.mitre.org/techniques/T1090): [Internal Proxy](https://attack.mitre.org/techniques/T1090/001)   |

[Drovorub](https://attack.mitre.org/software/S0502) can use a port forwarding rule on its agent module to relay network traffic through the client module to a remote host on the same network.[[1]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

|  Enterprise   |  [T1014](https://attack.mitre.org/techniques/T1014)   |  [Rootkit](https://attack.mitre.org/techniques/T1014)   |

[Drovorub](https://attack.mitre.org/software/S0502) has used a kernel module rootkit to hide processes, files, executables, and network artifacts from user space view.[[1]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

## Groups That Use This Software

| ID  | Name  | References

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[[1]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

## References

-    [NSA/FBI. (2020, August). Russian GRU 85th GTsSS Deploys Previously Undisclosed Drovorub Malware. Retrieved August 25, 2020.](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

×
