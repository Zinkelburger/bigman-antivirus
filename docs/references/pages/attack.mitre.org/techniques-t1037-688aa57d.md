---
url: "https://attack.mitre.org/techniques/T1037/"
title: "Boot or Logon Initialization Scripts, Technique T1037 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:26:39Z"
via: "live"
sha256: "2762976fd0a3877d962059e3c1089548e1029a031e4d5ebaba48f05d82a399e3"
wayback: "https://web.archive.org/web/20261025143740/https://attack.mitre.org/techniques/T1037/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Boot or Logon Initialization Scripts, Technique T1037 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1037/>
> Snapshot: <https://web.archive.org/web/20261025143740/https://attack.mitre.org/techniques/T1037/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Boot or Logon Initialization Scripts

#  Boot or Logon Initialization Scripts

##### Sub-techniques (5)

| ID  | Name

|  [T1037.001](https://attack.mitre.org/techniques/T1037/001/)   |  [Logon Script (Windows)](https://attack.mitre.org/techniques/T1037/001/)

|  [T1037.002](https://attack.mitre.org/techniques/T1037/002/)   |  [Login Hook](https://attack.mitre.org/techniques/T1037/002/)

|  [T1037.003](https://attack.mitre.org/techniques/T1037/003/)   |  [Network Logon Script](https://attack.mitre.org/techniques/T1037/003/)

|  [T1037.004](https://attack.mitre.org/techniques/T1037/004/)   |  [RC Scripts](https://attack.mitre.org/techniques/T1037/004/)

|  [T1037.005](https://attack.mitre.org/techniques/T1037/005/)   |  [Startup Items](https://attack.mitre.org/techniques/T1037/005/)

Adversaries may use scripts automatically executed at boot or logon initialization to establish persistence.[[1]](https://www.mandiant.com/resources/blog/unc3524-eye-spy-email)[[2]](https://www.anomali.com/blog/rocke-evolves-its-arsenal-with-a-new-malware-family-written-in-golang) Initialization scripts can be used to perform administrative functions, which may often execute other programs or send information to an internal logging server. These scripts can vary based on operating system and whether applied locally or remotely.

Adversaries may use these scripts to maintain persistence on a single system. Depending on the access configuration of the logon scripts, either local credentials or an administrator account may be necessary.

An adversary may also be able to escalate their privileges since some boot or logon initialization scripts run with higher privileges.

ID: T1037

Sub-techniques:  [T1037.001](https://attack.mitre.org/techniques/T1037/001), [T1037.002](https://attack.mitre.org/techniques/T1037/002), [T1037.003](https://attack.mitre.org/techniques/T1037/003), [T1037.004](https://attack.mitre.org/techniques/T1037/004), [T1037.005](https://attack.mitre.org/techniques/T1037/005)

ⓘ

Tactics: [Persistence](https://attack.mitre.org/tactics/TA0003), [Privilege Escalation](https://attack.mitre.org/tactics/TA0004)

ⓘ

Platforms: ESXi, Linux, Network Devices, Windows, macOS

Version: 2.4

Created: 31 May 2017

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1037/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1037/)

## Procedure Examples

| ID  | Name  | Description

|  [G0016](https://attack.mitre.org/groups/G0016)   |  [APT29](https://attack.mitre.org/groups/G0016)   |

[APT29](https://attack.mitre.org/groups/G0016) has hijacked legitimate application-specific startup scripts to enable malware to execute on system startup.[[1]](https://www.mandiant.com/resources/blog/unc3524-eye-spy-email)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) used a hidden shell script in /etc/rc.d/init.d to leverage the ADORE.XSECbackdoor and Adore-NG rootkit.[[3]](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)

|  [C0046](https://attack.mitre.org/campaigns/C0046)   |  [ArcaneDoor](https://attack.mitre.org/campaigns/C0046)   |

[ArcaneDoor](https://attack.mitre.org/campaigns/C0046) used malicious boot scripts to install the [Line Runner](https://attack.mitre.org/software/S1188) backdoor on victim devices.[[4]](https://blog.talosintelligence.com/arcanedoor-new-espionage-focused-campaign-found-targeting-perimeter-network-devices/)

|  [G0106](https://attack.mitre.org/groups/G0106)   |  [Rocke](https://attack.mitre.org/groups/G0106)   |

[Rocke](https://attack.mitre.org/groups/G0106) has installed an "init.d" startup script to maintain persistence.[[2]](https://www.anomali.com/blog/rocke-evolves-its-arsenal-with-a-new-malware-family-written-in-golang)

|  [S1078](https://attack.mitre.org/software/S1078)   |  [RotaJakiro](https://attack.mitre.org/software/S1078)   |

Depending on the Linux distribution and when executing with root permissions, [RotaJakiro](https://attack.mitre.org/software/S1078) may install persistence using a .conf file in the /etc/init/ folder.[[5]](https://blog.netlab.360.com/stealth_rotajakiro_backdoor_en/)

|  [S9024](https://attack.mitre.org/software/S9024)   |  [SPAWNCHIMERA](https://attack.mitre.org/software/S9024)   |

[SPAWNCHIMERA](https://attack.mitre.org/software/S9024) has modified the boot process files within /tmp/coreboot_fs/bin/init to establish persistence.[[6]](https://www.cisa.gov/news-events/analysis-reports/ar25-087a)

|  [G1048](https://attack.mitre.org/groups/G1048)   |  [UNC3886](https://attack.mitre.org/groups/G1048)   |

[UNC3886](https://attack.mitre.org/groups/G1048) has attempted to bypass digital signature verification checks at startup by adding a command to the startup config /etc/init.d/localnet within the rootfs.gz archive of both FortiManager and FortiAnalyzer devices.[[7]](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)

|  [S1217](https://attack.mitre.org/software/S1217)   |  [VIRTUALPITA](https://attack.mitre.org/software/S1217)   |

[VIRTUALPITA](https://attack.mitre.org/software/S1217) can persist as an init.d startup service on Linux vCenter systems.[[8]](https://cloud.google.com/blog/topics/threat-intelligence/esxi-hypervisors-malware-persistence)

## Mitigations

| ID  | Mitigation  | Description

|  [M1022](https://attack.mitre.org/mitigations/M1022)   |  [Restrict File and Directory Permissions](https://attack.mitre.org/mitigations/M1022)   |

Restrict write access to logon scripts to specific administrators.

|  [M1024](https://attack.mitre.org/mitigations/M1024)   |  [Restrict Registry Permissions](https://attack.mitre.org/mitigations/M1024)   |

Ensure proper permissions are set for Registry hives to prevent users from modifying keys for logon scripts that may lead to persistence.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0112](https://attack.mitre.org/detectionstrategies/DET0112)   |  [Boot or Logon Initialization Scripts Detection Strategy](https://attack.mitre.org/detectionstrategies/DET0112)   |  [AN0311](https://attack.mitre.org/detectionstrategies/DET0112#AN0311)   |

Monitoring modification and execution of user or system logon scripts such as in registry Run keys or startup folders.

|  [AN0312](https://attack.mitre.org/detectionstrategies/DET0112#AN0312)   |

Detection of changes or execution of shell initialization scripts like .bashrc, .profile, or /etc/profile for persistence.

|  [AN0313](https://attack.mitre.org/detectionstrategies/DET0112#AN0313)   |

Monitoring for modification and execution of login hook scripts or LaunchAgents/LaunchDaemons used for persistence.

|  [AN0314](https://attack.mitre.org/detectionstrategies/DET0112#AN0314)   |

Detection of modification to ESXi rc.local.d or rc scripts that are used to execute on boot.

|  [AN0315](https://attack.mitre.org/detectionstrategies/DET0112#AN0315)   |

Detection of changes to device startup-config files that include boot scripts or scheduled execution routines.

## References

-    [Mandiant. (2022, May 2). UNC3524: Eye Spy on Your Email. Retrieved August 17, 2023.](https://www.mandiant.com/resources/blog/unc3524-eye-spy-email)
-    [Anomali Labs. (2019, March 15). Rocke Evolves Its Arsenal With a New Malware Family Written in Golang. Retrieved April 24, 2019.](https://www.anomali.com/blog/rocke-evolves-its-arsenal-with-a-new-malware-family-written-in-golang)
-    [Mandiant. (n.d.). APT41, A DUAL ESPIONAGE AND CYBER CRIME OPERATION. Retrieved June 11, 2024.](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)
-    [Cisco Talos. (2024, April 24). ArcaneDoor - New espionage-focused campaign found targeting perimeter network devices. Retrieved January 6, 2025.](https://blog.talosintelligence.com/arcanedoor-new-espionage-focused-campaign-found-targeting-perimeter-network-devices/)

-    [Alex Turing, Hui Wang. (2021, April 28). RotaJakiro: A long live secret backdoor with 0 VT detection. Retrieved June 14, 2023.](https://blog.netlab.360.com/stealth_rotajakiro_backdoor_en/)
-    [DHS/CISA. (2026, February 26). MAR-25993211-r1.v2 Ivanti Connect Secure (RESURGE): AR25-087A. Retrieved April 17, 2026.](https://www.cisa.gov/news-events/analysis-reports/ar25-087a)
-    [Marvi, A. et al.. (2023, March 16). Fortinet Zero-Day and Custom Malware Used by Suspected Chinese Actor in Espionage Operation. Retrieved March 22, 2023.](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)
-    [Alexander Marvi, Jeremy Koppen, Tufail Ahmed, and Jonathan Lepore. (2022, September 29). Bad VIB(E)s Part One: Investigating Novel Malware Persistence Within ESXi Hypervisors. Retrieved March 26, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/esxi-hypervisors-malware-persistence)

×
