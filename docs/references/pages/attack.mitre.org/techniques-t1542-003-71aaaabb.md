---
url: "https://attack.mitre.org/techniques/T1542/003/"
title: "Pre-OS Boot: Bootkit, Sub-technique T1542.003 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:45:40Z"
via: "live"
sha256: "9af01cde0b9fb0ec827f80b04396b40ce0de0652a1ff26132c26fe41be8878d7"
wayback: "https://web.archive.org/web/20261023090809/https://attack.mitre.org/techniques/T1542/003/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Pre-OS Boot: Bootkit, Sub-technique T1542.003 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1542/003/>
> Snapshot: <https://web.archive.org/web/20261023090809/https://attack.mitre.org/techniques/T1542/003/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Pre-OS Boot](https://attack.mitre.org/techniques/T1542)
- Bootkit

#  Pre-OS Boot: Bootkit

##### Other sub-techniques of Pre-OS Boot (5)

| ID  | Name

|  [T1542.001](https://attack.mitre.org/techniques/T1542/001/)   |  [System Firmware](https://attack.mitre.org/techniques/T1542/001/)

|  [T1542.002](https://attack.mitre.org/techniques/T1542/002/)   |  [Component Firmware](https://attack.mitre.org/techniques/T1542/002/)

|  T1542.003   |  Bootkit

|  [T1542.004](https://attack.mitre.org/techniques/T1542/004/)   |  [ROMMONkit](https://attack.mitre.org/techniques/T1542/004/)

|  [T1542.005](https://attack.mitre.org/techniques/T1542/005/)   |  [TFTP Boot](https://attack.mitre.org/techniques/T1542/005/)

Adversaries may use bootkits to persist on systems. A bootkit is a malware variant that modifies the boot sectors of a hard drive, allowing malicious code to execute before a computer's operating system has loaded. Bootkits reside at a layer below the operating system and may make it difficult to perform full remediation unless an organization suspects one was used and can act accordingly.

In BIOS systems, a bootkit may modify the Master Boot Record (MBR) and/or Volume Boot Record (VBR).[[1]](https://web.archive.org/web/20211024160454/https://www.fireeye.com/content/dam/fireeye-www/current-threats/pdfs/rpt-mtrends-2016.pdf) The MBR is the section of disk that is first loaded after completing hardware initialization by the BIOS. It is the location of the boot loader. An adversary who has raw access to the boot drive may overwrite this area, diverting execution during startup from the normal boot loader to adversary code.[[2]](http://www.symantec.com/connect/blogs/are-mbr-infections-back-fashion)

The MBR passes control of the boot process to the VBR. Similar to the case of MBR, an adversary who has raw access to the boot drive may overwrite the VBR to divert execution during startup to adversary code.

In UEFI (Unified Extensible Firmware Interface) systems, a bootkit may instead create or modify files in the EFI system partition (ESP). The ESP is a partition on data storage used by devices containing UEFI that allows the system to boot the OS and other utilities used by the system. An adversary can use the newly created or patched files in the ESP to run malicious kernel code.[[3]](https://www.microsoft.com/en-us/security/blog/2023/04/11/guidance-for-investigating-attacks-using-cve-2022-21894-the-blacklotus-campaign/)[[4]](https://www.welivesecurity.com/2023/03/01/blacklotus-uefi-bootkit-myth-confirmed/)

ID: T1542.003

Sub-technique of:  [T1542](https://attack.mitre.org/techniques/T1542)

ⓘ

Tactics: [Stealth](https://attack.mitre.org/tactics/TA0005), [Persistence](https://attack.mitre.org/tactics/TA0003)

ⓘ

Platforms: Linux, Windows

Version: 2.0

Created: 19 December 2019

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1542/003/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1542/003/)

## Procedure Examples

| ID  | Name  | Description

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[APT28](https://attack.mitre.org/groups/G0007) has deployed a bootkit along with [Downdelph](https://attack.mitre.org/software/S0134) to ensure its persistence on the victim. The bootkit shares code with some variants of [BlackEnergy](https://attack.mitre.org/software/S0089).[[5]](http://www.welivesecurity.com/wp-content/uploads/2016/10/eset-sednit-part3.pdf)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) deployed Master Boot Record bootkits on Windows systems to hide their malware and maintain persistence on victim systems.[[6]](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)

|  [S0114](https://attack.mitre.org/software/S0114)   |  [BOOTRASH](https://attack.mitre.org/software/S0114)   |

[BOOTRASH](https://attack.mitre.org/software/S0114) is a Volume Boot Record (VBR) bootkit that uses the VBR to maintain persistence.[[1]](https://web.archive.org/web/20211024160454/https://www.fireeye.com/content/dam/fireeye-www/current-threats/pdfs/rpt-mtrends-2016.pdf)[[7]](https://www.fireeye.com/blog/threat-research/2015/12/fin1-targets-boot-record.html)[[8]](https://web.archive.org/web/20190926040727/https://www.sans.org/cyber-security-summit/archives/file/summit-archive-1498163766.pdf)

|  [S0484](https://attack.mitre.org/software/S0484)   |  [Carberp](https://attack.mitre.org/software/S0484)   |

[Carberp](https://attack.mitre.org/software/S0484) has installed a bootkit on the system to maintain persistence.[[9]](https://www.eset.com/fileadmin/eset/US/resources/docs/white-papers/white-papers-win-32-carberp.pdf)

|  [S0182](https://attack.mitre.org/software/S0182)   |  [FinFisher](https://attack.mitre.org/software/S0182)   |

Some [FinFisher](https://attack.mitre.org/software/S0182) variants incorporate an MBR rootkit.[[10]](https://web.archive.org/web/20171222050934/http://www.finfisher.com/FinFisher/index.html)[[11]](https://cloudblogs.microsoft.com/microsoftsecure/2018/03/01/finfisher-exposed-a-researchers-tale-of-defeating-traps-tricks-and-complex-virtual-machines/)

|  [G0032](https://attack.mitre.org/groups/G0032)   |  [Lazarus Group](https://attack.mitre.org/groups/G0032)   |

[Lazarus Group](https://attack.mitre.org/groups/G0032) malware WhiskeyAlfa-Three modifies sector 0 of the Master Boot Record (MBR) to ensure that the malware will persist even if a victim machine shuts down.[[12]](https://web.archive.org/web/20160226161828/https://www.operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Report.pdf)[[13]](https://web.archive.org/web/20160303200515/https:/operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Destructive-Malware-Report.pdf)

|  [S0112](https://attack.mitre.org/software/S0112)   |  [ROCKBOOT](https://attack.mitre.org/software/S0112)   |

[ROCKBOOT](https://attack.mitre.org/software/S0112) is a Master Boot Record (MBR) bootkit that uses the MBR to establish persistence.[[7]](https://www.fireeye.com/blog/threat-research/2015/12/fin1-targets-boot-record.html)

|  [S0266](https://attack.mitre.org/software/S0266)   |  [TrickBot](https://attack.mitre.org/software/S0266)   |

[TrickBot](https://attack.mitre.org/software/S0266) can implant malicious code into a compromised device's firmware.[[14]](https://eclypsium.com/wp-content/uploads/2020/12/TrickBot-Now-Offers-TrickBoot-Persist-Brick-Profit.pdf)

|  [S0689](https://attack.mitre.org/software/S0689)   |  [WhisperGate](https://attack.mitre.org/software/S0689)   |

[WhisperGate](https://attack.mitre.org/software/S0689) overwrites the MBR with a bootloader component that performs destructive wiping operations on hard drives and displays a fake ransom note when the host boots.[[15]](https://www.crowdstrike.com/blog/technical-analysis-of-whispergate-malware)[[16]](https://www.cybereason.com/blog/cybereason-vs.-whispergate-wiper)[[17]](https://www.microsoft.com/security/blog/2022/01/15/destructive-malware-targeting-ukrainian-organizations/)[[18]](https://blog.talosintelligence.com/2022/01/ukraine-campaign-delivers-defacement.html)[[19]](https://medium.com/s2wblog/analysis-of-destructive-malware-whispergate-targeting-ukraine-9d5d158f19f3)

## Mitigations

| ID  | Mitigation  | Description

|  [M1046](https://attack.mitre.org/mitigations/M1046)   |  [Boot Integrity](https://attack.mitre.org/mitigations/M1046)   |

Use Trusted Platform Module technology and a secure or trusted boot process to prevent system integrity from being compromised.[[20]](http://www.trustedcomputinggroup.org/wp-content/uploads/Trusted-Platform-Module-Summary_04292008.pdf)[[21]](https://docs.microsoft.com/en-us/windows/security/information-protection/secure-the-windows-10-boot-process)

|  [M1026](https://attack.mitre.org/mitigations/M1026)   |  [Privileged Account Management](https://attack.mitre.org/mitigations/M1026)   |

Ensure proper permissions are in place to help prevent adversary access to privileged accounts necessary to install a bootkit.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0150](https://attack.mitre.org/detectionstrategies/DET0150)   |  [Detection Strategy for File Creation or Modification of Boot Files](https://attack.mitre.org/detectionstrategies/DET0150)   |  [AN0428](https://attack.mitre.org/detectionstrategies/DET0150#AN0428)   |

Detection of raw access to physical drives, modification of boot records (MBR/VBR), and suspicious file creation or alteration within the EFI System Partition (ESP). Correlates privileged process execution with low-level disk modification and unexpected driver or firmware interactions.

|  [AN0429](https://attack.mitre.org/detectionstrategies/DET0150#AN0429)   |

Detection of suspicious write operations to block devices, modifications of bootloader files (GRUB, initrd, vmlinuz), and unexpected changes within the EFI System Partition. Monitors privileged execution of utilities like dd, grub-install, or efibootmgr that modify boot sectors or loader entries.

## References

-    [Mandiant. (2016, February 25). Mandiant M-Trends 2016. Retrieved November 17, 2024.](https://web.archive.org/web/20211024160454/https://www.fireeye.com/content/dam/fireeye-www/current-threats/pdfs/rpt-mtrends-2016.pdf)
-    [Lau, H. (2011, August 8). Are MBR Infections Back in Fashion? (Infographic). Retrieved November 13, 2014.](http://www.symantec.com/connect/blogs/are-mbr-infections-back-fashion)
-    [Microsoft Incident Response. (2023, April 11). Guidance for investigating attacks using CVE-2022-21894: The BlackLotus campaign. Retrieved February 12, 2025.](https://www.microsoft.com/en-us/security/blog/2023/04/11/guidance-for-investigating-attacks-using-cve-2022-21894-the-blacklotus-campaign/)
-    [Martin Smolár. (2023, March 1). BlackLotus UEFI bootkit: Myth confirmed. Retrieved February 11, 2025.](https://www.welivesecurity.com/2023/03/01/blacklotus-uefi-bootkit-myth-confirmed/)
-    [ESET. (2016, October). En Route with Sednit - Part 3: A Mysterious Downloader. Retrieved November 21, 2016.](http://www.welivesecurity.com/wp-content/uploads/2016/10/eset-sednit-part3.pdf)
-    [Fraser, N., et al. (2019, August 7). Double DragonAPT41, a dual espionage and cyber crime operation APT41. Retrieved September 23, 2019.](https://www.mandiant.com/sites/default/files/2022-02/rt-apt41-dual-operation.pdf)
-    [Andonov, D., et al. (2015, December 7). Thriving Beyond The Operating System: Financial Threat Group Targets Volume Boot Record. Retrieved May 13, 2016.](https://www.fireeye.com/blog/threat-research/2015/12/fin1-targets-boot-record.html)
-    [Glyer, C.. (2017, June 22). Boot What?. Retrieved November 17, 2024.](https://web.archive.org/web/20190926040727/https://www.sans.org/cyber-security-summit/archives/file/summit-archive-1498163766.pdf)
-    [Matrosov, A., Rodionov, E., Volkov, D., Harley, D. (2012, March 2). Win32/Carberp When You’re in a Black Hole, Stop Digging. Retrieved July 15, 2020.](https://www.eset.com/fileadmin/eset/US/resources/docs/white-papers/white-papers-win-32-carberp.pdf)
-    [FinFisher. (n.d.). Retrieved September 12, 2024.](https://web.archive.org/web/20171222050934/http://www.finfisher.com/FinFisher/index.html)
-    [Allievi, A.,Flori, E. (2018, March 01). FinFisher exposed: A researcher’s tale of defeating traps, tricks, and complex virtual machines. Retrieved July 9, 2018.](https://cloudblogs.microsoft.com/microsoftsecure/2018/03/01/finfisher-exposed-a-researchers-tale-of-defeating-traps-tricks-and-complex-virtual-machines/)

-    [Novetta Threat Research Group. (2016, February 24). Operation Blockbuster: Unraveling the Long Thread of the Sony Attack. Retrieved February 25, 2016.](https://web.archive.org/web/20160226161828/https://www.operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Report.pdf)
-    [Novetta Threat Research Group. (2016, February 24). Operation Blockbuster: Destructive Malware Report. Retrieved November 17, 2024.](https://web.archive.org/web/20160303200515/https:/operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Destructive-Malware-Report.pdf)
-    [Eclypsium, Advanced Intelligence. (2020, December 1). TRICKBOT NOW OFFERS ‘TRICKBOOT’: PERSIST, BRICK, PROFIT. Retrieved March 15, 2021.](https://eclypsium.com/wp-content/uploads/2020/12/TrickBot-Now-Offers-TrickBoot-Persist-Brick-Profit.pdf)
-    [Crowdstrike. (2022, January 19). Technical Analysis of the WhisperGate Malicious Bootloader. Retrieved March 10, 2022.](https://www.crowdstrike.com/blog/technical-analysis-of-whispergate-malware)
-    [Cybereason Nocturnus. (2022, February 15). Cybereason vs. WhisperGate and HermeticWiper. Retrieved March 10, 2022.](https://www.cybereason.com/blog/cybereason-vs.-whispergate-wiper)
-    [MSTIC. (2022, January 15). Destructive malware targeting Ukrainian organizations. Retrieved March 10, 2022.](https://www.microsoft.com/security/blog/2022/01/15/destructive-malware-targeting-ukrainian-organizations/)
-    [Biasini, N. et al.. (2022, January 21). Ukraine Campaign Delivers Defacement and Wipers, in Continued Escalation. Retrieved March 14, 2022.](https://blog.talosintelligence.com/2022/01/ukraine-campaign-delivers-defacement.html)
-    [S2W. (2022, January 18). Analysis of Destructive Malware (WhisperGate) targeting Ukraine. Retrieved March 14, 2022.](https://medium.com/s2wblog/analysis-of-destructive-malware-whispergate-targeting-ukraine-9d5d158f19f3)
-    [Trusted Computing Group. (2008, April 29). Trusted Platform Module (TPM) Summary. Retrieved June 8, 2016.](http://www.trustedcomputinggroup.org/wp-content/uploads/Trusted-Platform-Module-Summary_04292008.pdf)
-    [Microsoft. (n.d.). Secure the Windows 10 boot process. Retrieved April 23, 2020.](https://docs.microsoft.com/en-us/windows/security/information-protection/secure-the-windows-10-boot-process)

×
