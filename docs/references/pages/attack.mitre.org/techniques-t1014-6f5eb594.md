---
url: "https://attack.mitre.org/techniques/T1014/"
title: "Rootkit, Technique T1014 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:24:13Z"
via: "live"
sha256: "8567657e197aaff140686765241c25d381cd542de7fb8aa189c0d0ff4d5a396e"
wayback: "https://web.archive.org/web/20261024180533/https://attack.mitre.org/techniques/T1014/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
  - "docs/threats/05-malware-families-and-detection.md"
---

# Rootkit, Technique T1014 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1014/>
> Snapshot: <https://web.archive.org/web/20261024180533/https://attack.mitre.org/techniques/T1014/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Rootkit

#  Rootkit

Adversaries may use rootkits to hide the presence of programs, files, network connections, services, drivers, and other system components. Rootkits are programs that hide the existence of malware by intercepting/hooking and modifying operating system API calls that supply system information. [[1]](https://www.symantec.com/avcenter/reference/windows.rootkit.overview.pdf)

Rootkits or rootkit enabling functionality may reside at the user or kernel level in the operating system or lower, to include a hypervisor or [System Firmware](https://attack.mitre.org/techniques/T1542/001). [[2]](https://en.wikipedia.org/wiki/Rootkit) Rootkits have been seen for Windows, Linux, and Mac OS X systems. [[3]](https://www.crowdstrike.com/blog/http-iframe-injecting-linux-rootkit/) [[4]](http://www.blackhat.com/docs/asia-14/materials/Tsai/WP-Asia-14-Tsai-You-Cant-See-Me-A-Mac-OS-X-Rootkit-Uses-The-Tricks-You-Havent-Known-Yet.pdf)

Rootkits that reside or modify boot sectors are known as [Bootkit](https://attack.mitre.org/techniques/T1542/003)s and specifically target the boot process of the operating system.

ID: T1014

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Stealth](https://attack.mitre.org/tactics/TA0005)

ⓘ

Platforms: Linux, Windows, macOS

Contributors: Menachem Goldstein

Version: 2.0

Created: 31 May 2017

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1014/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1014/)

## Procedure Examples

| ID  | Name  | Description

|  [C0064](https://attack.mitre.org/campaigns/C0064)   |  [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064)   |

During the [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064), the adversary tasked Claude Code with generating and executing rootkits.[[5]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[APT28](https://attack.mitre.org/groups/G0007) has used a UEFI (Unified Extensible Firmware Interface) rootkit known as [LoJax](https://attack.mitre.org/software/S0397).[[6]](https://www.symantec.com/blogs/election-security/apt28-espionage-military-government)[[7]](https://www.welivesecurity.com/wp-content/uploads/2018/09/ESET-LoJax.pdf)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) deployed rootkits on Linux systems.[[8]](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)[[9]](https://go.crowdstrike.com/rs/281-OBQ-266/images/Report2020CrowdStrikeGlobalThreatReport.pdf)

|  [C0046](https://attack.mitre.org/campaigns/C0046)   |  [ArcaneDoor](https://attack.mitre.org/campaigns/C0046)   |

[ArcaneDoor](https://attack.mitre.org/campaigns/C0046) included hooking the processHostScanReply() function on victim Cisco ASA devices.[[10]](https://blog.talosintelligence.com/arcanedoor-new-espionage-focused-campaign-found-targeting-perimeter-network-devices/)

|  [S0484](https://attack.mitre.org/software/S0484)   |  [Carberp](https://attack.mitre.org/software/S0484)   |

[Carberp](https://attack.mitre.org/software/S0484) has used user mode rootkit techniques to remain hidden on the system.[[11]](https://web.archive.org/web/20231227000328/http://pxnow.prevx.com/content/blog/carberp-a_modular_information_stealing_trojan.pdf)

|  [S0572](https://attack.mitre.org/software/S0572)   |  [Caterpillar WebShell](https://attack.mitre.org/software/S0572)   |

[Caterpillar WebShell](https://attack.mitre.org/software/S0572) has a module to use a rootkit on a system.[[12]](https://www.clearskysec.com/wp-content/uploads/2021/01/Lebanese-Cedar-APT.pdf)

|  [S1105](https://attack.mitre.org/software/S1105)   |  [COATHANGER](https://attack.mitre.org/software/S1105)   |

[COATHANGER](https://attack.mitre.org/software/S1105) hooks or replaces multiple legitimate processes and other functions on victim devices.[[13]](https://www.ncsc.nl/binaries/ncsc/documenten/publicaties/2024/februari/6/mivd-aivd-advisory-coathanger-tlp-clear/TLP-CLEAR+MIVD+AIVD+Advisory+COATHANGER.pdf)

|  [S0502](https://attack.mitre.org/software/S0502)   |  [Drovorub](https://attack.mitre.org/software/S0502)   |

[Drovorub](https://attack.mitre.org/software/S0502) has used a kernel module rootkit to hide processes, files, executables, and network artifacts from user space view.[[14]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

|  [S0377](https://attack.mitre.org/software/S0377)   |  [Ebury](https://attack.mitre.org/software/S0377)   |

[Ebury](https://attack.mitre.org/software/S0377) acts as a user land rootkit using the SSH service.[[15]](https://www.welivesecurity.com/2017/10/30/windigo-ebury-update-2/)[[16]](https://web-assets.esetstatic.com/wls/en/papers/white-papers/ebury-is-alive-but-unseen.pdf)

|  [S0047](https://attack.mitre.org/software/S0047)   |  [Hacking Team UEFI Rootkit](https://attack.mitre.org/software/S0047)   |

[Hacking Team UEFI Rootkit](https://attack.mitre.org/software/S0047) is a UEFI BIOS rootkit developed by the company Hacking Team to persist remote access software on some targeted systems.[[17]](http://blog.trendmicro.com/trendlabs-security-intelligence/hacking-team-uses-uefi-bios-rootkit-to-keep-rcs-9-agent-in-target-systems/)

|  [S0394](https://attack.mitre.org/software/S0394)   |  [HiddenWasp](https://attack.mitre.org/software/S0394)   |

[HiddenWasp](https://attack.mitre.org/software/S0394) uses a rootkit to hook and implement functions on the system.[[18]](https://www.intezer.com/blog-hiddenwasp-malware-targeting-linux-systems/)

|  [S0135](https://attack.mitre.org/software/S0135)   |  [HIDEDRV](https://attack.mitre.org/software/S0135)   |

[HIDEDRV](https://attack.mitre.org/software/S0135) is a rootkit that hides certain operating system artifacts.[[19]](http://www.welivesecurity.com/wp-content/uploads/2016/10/eset-sednit-part3.pdf)

|  [S0009](https://attack.mitre.org/software/S0009)   |  [Hikit](https://attack.mitre.org/software/S0009)   |

[Hikit](https://attack.mitre.org/software/S0009) is a [Rootkit](https://attack.mitre.org/techniques/T1014) that has been used by [Axiom](https://attack.mitre.org/groups/G0001).[[20]](https://web.archive.org/web/20190216180458/https://www.fireeye.com/blog/threat-research/2012/08/hikit-rootkit-advanced-persistent-attack-techniques-part-1.html) [[21]](https://web.archive.org/web/20210920172620/https://www.fireeye.com/blog/threat-research/2012/08/hikit-rootkit-advanced-persistent-attack-techniques-part-2.html)

|  [S0601](https://attack.mitre.org/software/S0601)   |  [Hildegard](https://attack.mitre.org/software/S0601)   |

[Hildegard](https://attack.mitre.org/software/S0601) has modified /etc/ld.so.preload to overwrite readdir() and readdir64().[[22]](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)

|  [S0040](https://attack.mitre.org/software/S0040)   |  [HTRAN](https://attack.mitre.org/software/S0040)   |

[HTRAN](https://attack.mitre.org/software/S0040) can install a rootkit to hide network connections from the host OS.[[23]](https://www.ncsc.gov.uk/report/joint-report-on-publicly-available-hacking-tools)

|  [S1186](https://attack.mitre.org/software/S1186)   |  [Line Dancer](https://attack.mitre.org/software/S1186)   |

[Line Dancer](https://attack.mitre.org/software/S1186) can hook both the crash dump process and the Autehntication, Authorization, and Accounting (AAA) functions on compromised machines to evade forensic analysis and authentication mechanisms.[[10]](https://blog.talosintelligence.com/arcanedoor-new-espionage-focused-campaign-found-targeting-perimeter-network-devices/)

|  [S0397](https://attack.mitre.org/software/S0397)   |  [LoJax](https://attack.mitre.org/software/S0397)   |

[LoJax](https://attack.mitre.org/software/S0397) is a UEFI BIOS rootkit deployed to persist remote access software on some targeted systems.[[7]](https://www.welivesecurity.com/wp-content/uploads/2018/09/ESET-LoJax.pdf)

|  [S1220](https://attack.mitre.org/software/S1220)   |  [MEDUSA](https://attack.mitre.org/software/S1220)   |

[MEDUSA](https://attack.mitre.org/software/S1220) is a rootkit with command execution and credential logging capabilities.[[24]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

|  [S0012](https://attack.mitre.org/software/S0012)   |  [PoisonIvy](https://attack.mitre.org/software/S0012)   |

[PoisonIvy](https://attack.mitre.org/software/S0012) starts a rootkit from a malicious file dropped to disk.[[25]](https://www.symantec.com/security_response/writeup.jsp?docid=2005-081910-3934-99)

|  [S0458](https://attack.mitre.org/software/S0458)   |  [Ramsay](https://attack.mitre.org/software/S0458)   |

[Ramsay](https://attack.mitre.org/software/S0458) has included a rootkit to evade defenses.[[26]](https://www.welivesecurity.com/2020/05/13/ramsay-cyberespionage-toolkit-airgapped-networks/)

|  [C0056](https://attack.mitre.org/campaigns/C0056)   |  [RedPenguin](https://attack.mitre.org/campaigns/C0056)   |

During [RedPenguin](https://attack.mitre.org/campaigns/C0056), [UNC3886](https://attack.mitre.org/groups/G1048) used rootkits such as [REPTILE](https://attack.mitre.org/software/S1219) and [MEDUSA](https://attack.mitre.org/software/S1220).[[27]](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-targets-juniper-routers)

|  [S1219](https://attack.mitre.org/software/S1219)   |  [REPTILE](https://attack.mitre.org/software/S1219)   |

[REPTILE](https://attack.mitre.org/software/S1219) has the ability to hook kernel functions and modify functions data to achieve rootkit functionality such as hiding processes and network connections.[[24]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

|  [G0106](https://attack.mitre.org/groups/G0106)   |  [Rocke](https://attack.mitre.org/groups/G0106)   |

[Rocke](https://attack.mitre.org/groups/G0106) has modified /etc/ld.so.preload to hook libc functions in order to hide the installed dropper and mining software in process lists.[[28]](https://www.anomali.com/blog/rocke-evolves-its-arsenal-with-a-new-malware-family-written-in-golang)

|  [S0468](https://attack.mitre.org/software/S0468)   |  [Skidmap](https://attack.mitre.org/software/S0468)   |

[Skidmap](https://attack.mitre.org/software/S0468) is a kernel-mode rootkit that has the ability to hook system calls to hide specific files and fake network and CPU-related statistics to make the CPU load of the infected machine always appear low.[[29]](https://blog.trendmicro.com/trendlabs-security-intelligence/skidmap-linux-malware-uses-rootkit-capabilities-to-hide-cryptocurrency-mining-payload/)

|  [S0603](https://attack.mitre.org/software/S0603)   |  [Stuxnet](https://attack.mitre.org/software/S0603)   |

[Stuxnet](https://attack.mitre.org/software/S0603) uses a Windows rootkit to mask its binaries and other relevant files.[[30]](https://docs.broadcom.com/doc/security-response-w32-stuxnet-dossier-11-en)

|  [G0139](https://attack.mitre.org/groups/G0139)   |  [TeamTNT](https://attack.mitre.org/groups/G0139)   |

[TeamTNT](https://attack.mitre.org/groups/G0139) has used rootkits such as the open-source Diamorphine rootkit and their custom bots to hide cryptocurrency mining activities on the machine.[[31]](https://documents.trendmicro.com/assets/white_papers/wp-tracking-the-activities-of-teamTNT.pdf) [[32]](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/)

|  [S0221](https://attack.mitre.org/software/S0221)   |  [Umbreon](https://attack.mitre.org/software/S0221)   |

[Umbreon](https://attack.mitre.org/software/S0221) hides from defenders by hooking libc function calls, hiding artifacts that would reveal its presence, such as the user account it creates to provide access and undermining strace, a tool often used to identify malware.[[33]](https://blog.trendmicro.com/trendlabs-security-intelligence/pokemon-themed-umbreon-linux-rootkit-hits-x86-arm-systems/?_ga=2.180041126.367598458.1505420282-1759340220.1502477046)

|  [G1048](https://attack.mitre.org/groups/G1048)   |  [UNC3886](https://attack.mitre.org/groups/G1048)   |

[UNC3886](https://attack.mitre.org/groups/G1048) has used the publicly available rootkits [REPTILE](https://attack.mitre.org/software/S1219) and [MEDUSA](https://attack.mitre.org/software/S1220) on targeted VMs.[[24]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

|  [S0022](https://attack.mitre.org/software/S0022)   |  [Uroburos](https://attack.mitre.org/software/S0022)   |

[Uroburos](https://attack.mitre.org/software/S0022) can use its kernel module to prevent its host components from being listed by the targeted system's OS and to mediate requests between user mode and concealed components.[[34]](https://securelist.com/the-epic-turla-operation/65545/)[[35]](https://www.cisa.gov/sites/default/files/2023-05/aa23-129a_snake_malware_2.pdf)

|  [S0670](https://attack.mitre.org/software/S0670)   |  [WarzoneRAT](https://attack.mitre.org/software/S0670)   |

[WarzoneRAT](https://attack.mitre.org/software/S0670) can include a rootkit to hide processes, files, and startup.[[36]](https://research.checkpoint.com/2020/warzone-behind-the-enemy-lines/)

|  [S0430](https://attack.mitre.org/software/S0430)   |  [Winnti for Linux](https://attack.mitre.org/software/S0430)   |

[Winnti for Linux](https://attack.mitre.org/software/S0430) has used a modified copy of the open-source userland rootkit Azazel, named libxselinux.so, to hide the malware's operations and network activity.[[37]](https://medium.com/chronicle-blog/winnti-more-than-just-windows-and-gates-e4f03436031a)

|  [G0044](https://attack.mitre.org/groups/G0044)   |  [Winnti Group](https://attack.mitre.org/groups/G0044)   |

[Winnti Group](https://attack.mitre.org/groups/G0044) used a rootkit to modify typical server functionality.[[38]](https://securelist.com/winnti-more-than-just-a-game/37029/)

|  [S0027](https://attack.mitre.org/software/S0027)   |  [Zeroaccess](https://attack.mitre.org/software/S0027)   |

[Zeroaccess](https://attack.mitre.org/software/S0027) is a kernel-mode rootkit.[[39]](https://sophosnews.files.wordpress.com/2012/04/zeroaccess2.pdf)

## Mitigations

This type of attack technique cannot be easily mitigated with preventive controls since it is based on the abuse of system features.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0377](https://attack.mitre.org/detectionstrategies/DET0377)   |  [Detection of Kernel/User-Level Rootkit Behavior Across Platforms](https://attack.mitre.org/detectionstrategies/DET0377)   |  [AN1061](https://attack.mitre.org/detectionstrategies/DET0377#AN1061)   |

Unauthorized or anomalous loading of kernel-mode drivers or DLLs, concealed services, or abnormal modification of boot components indicative of rootkit activity.

|  [AN1062](https://attack.mitre.org/detectionstrategies/DET0377#AN1062)   |

Abnormal loading of kernel modules, direct tampering with /dev, /proc, or LD_PRELOAD behaviors hiding processes or files.

|  [AN1063](https://attack.mitre.org/detectionstrategies/DET0377#AN1063)   |

Execution of unsigned kernel extensions (KEXTs), tampering with LaunchDaemons, or userspace hooks into system libraries.

## References

-    [Symantec. (n.d.). Windows Rootkit Overview. Retrieved December 21, 2017.](https://www.symantec.com/avcenter/reference/windows.rootkit.overview.pdf)
-    [Wikipedia. (2016, June 1). Rootkit. Retrieved June 2, 2016.](https://en.wikipedia.org/wiki/Rootkit)
-    [Kurtz, G. (2012, November 19). HTTP iframe Injecting Linux Rootkit. Retrieved December 21, 2017.](https://www.crowdstrike.com/blog/http-iframe-injecting-linux-rootkit/)
-    [Pan, M., Tsai, S. (2014). You can’t see me: A Mac OS X Rootkit uses the tricks you haven't known yet. Retrieved December 21, 2017.](http://www.blackhat.com/docs/asia-14/materials/Tsai/WP-Asia-14-Tsai-You-Cant-See-Me-A-Mac-OS-X-Rootkit-Uses-The-Tricks-You-Havent-Known-Yet.pdf)
-    [Eyal Sela. (2026, April). The AI-Assisted Breach of Mexico's Government Infrastructure. Retrieved September 2, 2026.](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)
-    [Symantec Security Response. (2018, October 04). APT28: New Espionage Operations Target Military and Government Organizations. Retrieved November 14, 2018.](https://www.symantec.com/blogs/election-security/apt28-espionage-military-government)
-    [ESET. (2018, September). LOJAX First UEFI rootkit found in the wild, courtesy of the Sednit group. Retrieved July 2, 2019.](https://www.welivesecurity.com/wp-content/uploads/2018/09/ESET-LoJax.pdf)
-    [Fraser, N., et al. (2019, August 7). Double DragonAPT41, a dual espionage and cyber crime operation APT41. Retrieved September 23, 2019.](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)
-    [Crowdstrike. (2020, March 2). 2020 Global Threat Report. Retrieved December 11, 2020.](https://go.crowdstrike.com/rs/281-OBQ-266/images/Report2020CrowdStrikeGlobalThreatReport.pdf)
-    [Cisco Talos. (2024, April 24). ArcaneDoor - New espionage-focused campaign found targeting perimeter network devices. Retrieved January 6, 2025.](https://blog.talosintelligence.com/arcanedoor-new-espionage-focused-campaign-found-targeting-perimeter-network-devices/)
-    [Giuliani, M., Allievi, A. (2011, February 28). Carberp - a modular information stealing trojan. Retrieved September 12, 2024.](https://web.archive.org/web/20231227000328/http://pxnow.prevx.com/content/blog/carberp-a_modular_information_stealing_trojan.pdf)
-    [ClearSky Cyber Security. (2021, January). “Lebanese Cedar” APT Global Lebanese Espionage Campaign Leveraging Web Servers. Retrieved February 10, 2021.](https://www.clearskysec.com/wp-content/uploads/2021/01/Lebanese-Cedar-APT.pdf)
-    [Dutch Military Intelligence and Security Service (MIVD) & Dutch General Intelligence and Security Service (AIVD). (2024, February 6). Ministry of Defense of the Netherlands uncovers COATHANGER, a stealthy Chinese FortiGate RAT. Retrieved February 7, 2024.](https://www.ncsc.nl/binaries/ncsc/documenten/publicaties/2024/februari/6/mivd-aivd-advisory-coathanger-tlp-clear/TLP-CLEAR+MIVD+AIVD+Advisory+COATHANGER.pdf)
-    [NSA/FBI. (2020, August). Russian GRU 85th GTsSS Deploys Previously Undisclosed Drovorub Malware. Retrieved August 25, 2020.](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)
-    [Vachon, F. (2017, October 30). Windigo Still not Windigone: An Ebury Update . Retrieved February 10, 2021.](https://www.welivesecurity.com/2017/10/30/windigo-ebury-update-2/)
-    [Marc-Etienne M.Léveillé. (2024, May 1). Ebury is alive but unseen. Retrieved May 21, 2024.](https://web-assets.esetstatic.com/wls/en/papers/white-papers/ebury-is-alive-but-unseen.pdf)
-    [Lin, P. (2015, July 13). Hacking Team Uses UEFI BIOS Rootkit to Keep RCS 9 Agent in Target Systems. Retrieved December 11, 2015.](http://blog.trendmicro.com/trendlabs-security-intelligence/hacking-team-uses-uefi-bios-rootkit-to-keep-rcs-9-agent-in-target-systems/)
-    [Sanmillan, I. (2019, May 29). HiddenWasp Malware Stings Targeted Linux Systems. Retrieved June 24, 2019.](https://www.intezer.com/blog-hiddenwasp-malware-targeting-linux-systems/)
-    [ESET. (2016, October). En Route with Sednit - Part 3: A Mysterious Downloader. Retrieved November 21, 2016.](http://www.welivesecurity.com/wp-content/uploads/2016/10/eset-sednit-part3.pdf)
-    [Glyer, C., Kazanciyan, R. (2012, August 20). The “Hikit” Rootkit: Advanced and Persistent Attack Techniques (Part 1). Retrieved November 17, 2024.](https://web.archive.org/web/20190216180458/https://www.fireeye.com/blog/threat-research/2012/08/hikit-rootkit-advanced-persistent-attack-techniques-part-1.html)

-    [Glyer, C., Kazanciyan, R. (2012, August 22). The “Hikit” Rootkit: Advanced and Persistent Attack Techniques (Part 2). Retrieved November 17, 2024.](https://web.archive.org/web/20210920172620/https://www.fireeye.com/blog/threat-research/2012/08/hikit-rootkit-advanced-persistent-attack-techniques-part-2.html)
-    [Chen, J. et al. (2021, February 3). Hildegard: New TeamTNT Cryptojacking Malware Targeting Kubernetes. Retrieved April 5, 2021.](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)
-    [The Australian Cyber Security Centre (ACSC), the Canadian Centre for Cyber Security (CCCS), the New Zealand National Cyber Security Centre (NZ NCSC), CERT New Zealand, the UK National Cyber Security Centre (UK NCSC) and the US National Cybersecurity and Communications Integration Center (NCCIC). (2018, October 11). Joint report on publicly available hacking tools. Retrieved March 11, 2019.](https://www.ncsc.gov.uk/report/joint-report-on-publicly-available-hacking-tools)
-    [Punsaen Boonyakarn, Shawn Chew, Logeswaran Nadarajan, Mathew Potaczek, Jakub Jozwiak, and Alex Marvi. (2024, June 18). Cloaked and Covert: Uncovering UNC3886 Espionage Operations. Retrieved September 24, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)
-    [Hayashi, K. (2005, August 18). Backdoor.Darkmoon. Retrieved February 23, 2018.](https://www.symantec.com/security_response/writeup.jsp?docid=2005-081910-3934-99)
-    [Sanmillan, I.. (2020, May 13). Ramsay: A cyber‑espionage toolkit tailored for air‑gapped networks. Retrieved May 27, 2020.](https://www.welivesecurity.com/2020/05/13/ramsay-cyberespionage-toolkit-airgapped-networks/)
-    [Lamparski, L. et al. (2025, March 11). Ghost in the Router: China-Nexus Espionage Actor UNC3886 Targets Juniper Routers. Retrieved June 24, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-targets-juniper-routers)
-    [Anomali Labs. (2019, March 15). Rocke Evolves Its Arsenal With a New Malware Family Written in Golang. Retrieved April 24, 2019.](https://www.anomali.com/blog/rocke-evolves-its-arsenal-with-a-new-malware-family-written-in-golang)
-    [Remillano, A., Urbanec, J. (2019, September 19). Skidmap Linux Malware Uses Rootkit Capabilities to Hide Cryptocurrency-Mining Payload. Retrieved June 4, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/skidmap-linux-malware-uses-rootkit-capabilities-to-hide-cryptocurrency-mining-payload/)
-    [Nicolas Falliere, Liam O Murchu, Eric Chien 2011, February W32.Stuxnet Dossier (Version 1.4) Retrieved November 17, 2024.](https://docs.broadcom.com/doc/security-response-w32-stuxnet-dossier-11-en)
-    [Fiser, D. Oliveira, A. (n.d.). Tracking the Activities of TeamTNT A Closer Look at a Cloud-Focused Malicious Actor Group. Retrieved September 22, 2021.](https://documents.trendmicro.com/assets/white_papers/wp-tracking-the-activities-of-teamTNT.pdf)
-    [Darin Smith. (2022, April 21). TeamTNT targeting AWS, Alibaba. Retrieved August 4, 2022.](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/)
-    [Fernando Mercês. (2016, September 5). Pokémon-themed Umbreon Linux Rootkit Hits x86, ARM Systems. Retrieved March 5, 2018.](https://blog.trendmicro.com/trendlabs-security-intelligence/pokemon-themed-umbreon-linux-rootkit-hits-x86-arm-systems/?_ga=2.180041126.367598458.1505420282-1759340220.1502477046)
-    [Kaspersky Lab's Global Research and Analysis Team. (2014, August 7). The Epic Turla Operation: Solving some of the mysteries of Snake/Uroburos. Retrieved December 11, 2014.](https://securelist.com/the-epic-turla-operation/65545/)
-    [FBI et al. (2023, May 9). Hunting Russian Intelligence “Snake” Malware. Retrieved June 8, 2023.](https://www.cisa.gov/sites/default/files/2023-05/aa23-129a_snake_malware_2.pdf)
-    [Harakhavik, Y. (2020, February 3). Warzone: Behind the enemy lines. Retrieved December 17, 2021.](https://research.checkpoint.com/2020/warzone-behind-the-enemy-lines/)
-    [Chronicle Blog. (2019, May 15). Winnti: More than just Windows and Gates. Retrieved April 29, 2020.](https://medium.com/chronicle-blog/winnti-more-than-just-windows-and-gates-e4f03436031a)
-    [Kaspersky Lab's Global Research and Analysis Team. (2013, April 11). Winnti. More than just a game. Retrieved February 8, 2017.](https://securelist.com/winnti-more-than-just-a-game/37029/)
-    [Wyke, J. (2012, April). ZeroAccess. Retrieved July 18, 2016.](https://sophosnews.files.wordpress.com/2012/04/zeroaccess2.pdf)

×
