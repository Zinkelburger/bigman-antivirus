---
url: "https://attack.mitre.org/techniques/T1485/"
title: "Data Destruction, Technique T1485 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:42:58Z"
via: "live"
sha256: "91bfd21f35e8dfc43df472fabfd36dfea69cf06ca6b4608790c4e067ddb97849"
wayback: "https://web.archive.org/web/20261023090832/https://attack.mitre.org/techniques/T1485/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Data Destruction, Technique T1485 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1485/>
> Snapshot: <https://web.archive.org/web/20261023090832/https://attack.mitre.org/techniques/T1485/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Data Destruction

#  Data Destruction

##### Sub-techniques (1)

| ID  | Name

|  [T1485.001](https://attack.mitre.org/techniques/T1485/001/)   |  [Lifecycle-Triggered Deletion](https://attack.mitre.org/techniques/T1485/001/)

Adversaries may destroy data and files on specific systems or in large numbers on a network to interrupt availability to systems, services, and network resources. Data destruction is likely to render stored data irrecoverable by forensic techniques through overwriting files or data on local and remote drives.[[1]](https://www.symantec.com/connect/blogs/shamoon-attacks)[[2]](https://web.archive.org/web/20210126065851/https://www.fireeye.com/blog/threat-research/2016/11/fireeye_respondsto.html)[[3]](http://researchcenter.paloaltonetworks.com/2016/11/unit42-shamoon-2-return-disttrack-wiper/)[[4]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07180722/Report_Shamoon_StoneDrill_final.pdf)[[5]](https://unit42.paloaltonetworks.com/shamoon-3-targets-oil-gas-organization/)[[6]](https://blog.talosintelligence.com/2018/02/olympic-destroyer.html) Common operating system file deletion commands such as del and rm often only remove pointers to files without wiping the contents of the files themselves, making the files recoverable by proper forensic methodology. This behavior is distinct from [Disk Content Wipe](https://attack.mitre.org/techniques/T1561/001) and [Disk Structure Wipe](https://attack.mitre.org/techniques/T1561/002) because individual files are destroyed rather than sections of a storage disk or the disk's logical structure.

Adversaries may attempt to overwrite files and directories with randomly generated data to make it irrecoverable.[[4]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07180722/Report_Shamoon_StoneDrill_final.pdf)[[5]](https://unit42.paloaltonetworks.com/shamoon-3-targets-oil-gas-organization/) In some cases politically oriented image files have been used to overwrite data.[[2]](https://web.archive.org/web/20210126065851/https://www.fireeye.com/blog/threat-research/2016/11/fireeye_respondsto.html)[[3]](http://researchcenter.paloaltonetworks.com/2016/11/unit42-shamoon-2-return-disttrack-wiper/)[[4]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07180722/Report_Shamoon_StoneDrill_final.pdf)

To maximize impact on the target organization in operations where network-wide availability interruption is the goal, malware designed for destroying data may have worm-like features to propagate across a network by leveraging additional techniques like [Valid Accounts](https://attack.mitre.org/techniques/T1078), [OS Credential Dumping](https://attack.mitre.org/techniques/T1003), and [SMB/Windows Admin Shares](https://attack.mitre.org/techniques/T1021/002).[[1]](https://www.symantec.com/connect/blogs/shamoon-attacks)[[2]](https://web.archive.org/web/20210126065851/https://www.fireeye.com/blog/threat-research/2016/11/fireeye_respondsto.html)[[3]](http://researchcenter.paloaltonetworks.com/2016/11/unit42-shamoon-2-return-disttrack-wiper/)[[4]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07180722/Report_Shamoon_StoneDrill_final.pdf)[[6]](https://blog.talosintelligence.com/2018/02/olympic-destroyer.html).

In cloud environments, adversaries may leverage access to delete cloud storage objects, machine images, database instances, and other infrastructure crucial to operations to damage an organization or their customers.[[7]](https://threatpost.com/hacker-puts-hosting-service-code-spaces-out-of-business/106761/)[[8]](https://www.justice.gov/usao-ndca/pr/san-jose-man-pleads-guilty-damaging-cisco-s-network) Similarly, they may delete virtual machines from on-prem virtualized environments.

ID: T1485

Sub-techniques:  [T1485.001](https://attack.mitre.org/techniques/T1485/001)

ⓘ

Tactic: [Impact](https://attack.mitre.org/tactics/TA0040)

ⓘ

Platforms: Containers, ESXi, IaaS, Linux, Windows, macOS

ⓘ

Impact Type: Availability

Contributors: Brent Murphy, Elastic; David French, Elastic; Joey Lei; Prasad Somasamudram, McAfee; Sekhar Sarukkai, McAfee; Syed Ummar Farooqh, McAfee; Varonis Threat Labs

Version: 1.4

Created: 14 March 2019

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1485/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1485/)

## Procedure Examples

| ID  | Name  | Description

|  [C0034](https://attack.mitre.org/campaigns/C0034)   |  [2022 Ukraine Electric Power Attack](https://attack.mitre.org/campaigns/C0034)   |

During the [2022 Ukraine Electric Power Attack](https://attack.mitre.org/campaigns/C0034), [Sandworm Team](https://attack.mitre.org/groups/G0034) deployed [CaddyWiper](https://attack.mitre.org/software/S0693) on the victim’s IT environment systems to wipe files related to the OT capabilities, along with mapped drives, and physical drive partitions.[[9]](https://www.mandiant.com/resources/blog/sandworm-disrupts-power-ukraine-operational-technology)

|  [C0063](https://attack.mitre.org/campaigns/C0063)   |  [2025 Poland Wiper Attacks](https://attack.mitre.org/campaigns/C0063)   |

During the [2025 Poland Wiper Attacks](https://attack.mitre.org/campaigns/C0063), the adversaries utilized wiper malware to overwrite files using a 16-byte buffer that fully overwrites files 16 bytes or smaller or partially overwrites files greater than 16 bytes to speed up the process.[[10]](https://www.eset.com/us/about/newsroom/research/eset-research-russian-sandwormapt-attacks-energy-company-poland-with-dynowiper/)[[11]](https://www.welivesecurity.com/en/eset-research/dynowiper-update-technical-analysis-attribution/)

|  [S1167](https://attack.mitre.org/software/S1167)   |  [AcidPour](https://attack.mitre.org/software/S1167)   |

[AcidPour](https://attack.mitre.org/software/S1167) can perform an in-depth wipe of victim filesystems and attached storage devices through either data overwrite or calling various IOCTLS to erase them, similar to [AcidRain](https://attack.mitre.org/software/S1125).[[12]](https://www.sentinelone.com/labs/acidpour-new-embedded-wiper-variant-of-acidrain-appears-in-ukraine/)

|  [S1125](https://attack.mitre.org/software/S1125)   |  [AcidRain](https://attack.mitre.org/software/S1125)   |

[AcidRain](https://attack.mitre.org/software/S1125) performs an in-depth wipe of the target filesystem and various attached storage devices through either a data overwrite or calling various IOCTLS to erase it.[[13]](https://www.sentinelone.com/labs/acidrain-a-modem-wiper-rains-down-on-europe/)

|  [S1133](https://attack.mitre.org/software/S1133)   |  [Apostle](https://attack.mitre.org/software/S1133)   |

[Apostle](https://attack.mitre.org/software/S1133) initially masqueraded as ransomware but actual functionality is a data destruction tool, supported by an internal name linked to an early version, wiper-action. [Apostle](https://attack.mitre.org/software/S1133) writes random data to original files after an encrypted copy is created, along with resizing the original file to zero and changing time property metadata before finally deleting the original file.[[14]](https://assets.sentinelone.com/sentinellabs/evol-agrius)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has used a custom secure delete function to make deleted files unrecoverable.[[15]](https://services.google.com/fh/files/misc/apt38-un-usual-suspects.pdf)

|  [S0089](https://attack.mitre.org/software/S0089)   |  [BlackEnergy](https://attack.mitre.org/software/S0089)   |

[BlackEnergy](https://attack.mitre.org/software/S0089) 2 contains a "Destroy" plug-in that destroys data stored on victim hard drives by overwriting file contents.[[16]](https://securelist.com/be2-extraordinary-plugins-siemens-targeting-dev-fails/68838/)[[17]](https://www.welivesecurity.com/2016/01/03/blackenergy-sshbeardoor-details-2015-attacks-ukrainian-news-media-electric-industry/)

|  [S0693](https://attack.mitre.org/software/S0693)   |  [CaddyWiper](https://attack.mitre.org/software/S0693)   |

[CaddyWiper](https://attack.mitre.org/software/S0693) can work alphabetically through drives on a compromised system to take ownership of and overwrite all files.[[18]](https://www.welivesecurity.com/2022/03/15/caddywiper-new-wiper-malware-discovered-ukraine)[[19]](https://blog.talosintelligence.com/2022/03/threat-advisory-caddywiper.html)

|  [S9042](https://attack.mitre.org/software/S9042)   |  [CanisterWorm](https://attack.mitre.org/software/S9042)   |

[CanisterWorm](https://attack.mitre.org/software/S9042) has targeted wiper components that can delete entire clusters and execute recursive file deletions on non-containerized hosts.[[20]](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)[[21]](https://www.aikido.dev/blog/teampcp-stage-payload-canisterworm-iran)

|  [S1134](https://attack.mitre.org/software/S1134)   |  [DEADWOOD](https://attack.mitre.org/software/S1134)   |

[DEADWOOD](https://attack.mitre.org/software/S1134) overwrites files on victim systems with random data to effectively destroy them.[[14]](https://assets.sentinelone.com/sentinellabs/evol-agrius)

|  [S0659](https://attack.mitre.org/software/S0659)   |  [Diavol](https://attack.mitre.org/software/S0659)   |

[Diavol](https://attack.mitre.org/software/S0659) can delete specified files from a targeted system.[[22]](https://www.fortinet.com/blog/threat-research/diavol-new-ransomware-used-by-wizard-spider)

|  [S9038](https://attack.mitre.org/software/S9038)   |  [DynoWiper](https://attack.mitre.org/software/S9038)   |

[DynoWiper](https://attack.mitre.org/software/S9038) has overwritten files with 16-byte sequences of random data generated by the Mersenne Twister algorithm using the Microsoft Windows native CreateFileW() function to open the file and the SetFilePointerEx() and WriteFile() functions to overwrite the file.[[23]](https://cert.pl/uploads/docs/CERT_Polska_Energy_Sector_Incident_Report_2025.pdf) Additionally, versions of [DynoWiper](https://attack.mitre.org/software/S9038) can also delete files using the DeleteFileW API.[[11]](https://www.welivesecurity.com/en/eset-research/dynowiper-update-technical-analysis-attribution/)

|  [S0697](https://attack.mitre.org/software/S0697)   |  [HermeticWiper](https://attack.mitre.org/software/S0697)   |

[HermeticWiper](https://attack.mitre.org/software/S0697) can recursively wipe folders and files in Windows, Program Files, Program Files(x86), PerfLogs, Boot, System, Volume Information, and AppData folders using FSCTL_MOVE_FILE. [HermeticWiper](https://attack.mitre.org/software/S0697) can also overwrite symbolic links and big files in My Documents and on the Desktop with random bytes.[[24]](https://www.welivesecurity.com/2022/03/01/isaacwiper-hermeticwizard-wiper-worm-targeting-ukraine)

|  [S0604](https://attack.mitre.org/software/S0604)   |  [Industroyer](https://attack.mitre.org/software/S0604)   |

[Industroyer](https://attack.mitre.org/software/S0604)’s data wiper module clears registry keys and overwrites both ICS configuration and Windows files.[[25]](https://dragos.com/blog/crashoverride/CrashOverride-01.pdf)

|  [S0265](https://attack.mitre.org/software/S0265)   |  [Kazuar](https://attack.mitre.org/software/S0265)   |

[Kazuar](https://attack.mitre.org/software/S0265) can overwrite files with random data before deleting them.[[26]](https://researchcenter.paloaltonetworks.com/2017/05/unit42-kazuar-multiplatform-espionage-backdoor-api-access/)

|  [S0607](https://attack.mitre.org/software/S0607)   |  [KillDisk](https://attack.mitre.org/software/S0607)   |

[KillDisk](https://attack.mitre.org/software/S0607) deletes system files to make the OS unbootable. [KillDisk](https://attack.mitre.org/software/S0607) also targets and deletes files with 35 different file extensions.[[27]](http://www.welivesecurity.com/2016/01/03/blackenergy-sshbeardoor-details-2015-attacks-ukrainian-news-media-electric-industry/)

|  [G1004](https://attack.mitre.org/groups/G1004)   |  [LAPSUS$](https://attack.mitre.org/groups/G1004)   |

[LAPSUS$](https://attack.mitre.org/groups/G1004) has deleted the target's systems and resources both on-premises and in the cloud.[[28]](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)[[29]](https://www.nccgroup.com/research-blog/lapsus-recent-techniques-tactics-and-procedures/)

|  [G0032](https://attack.mitre.org/groups/G0032)   |  [Lazarus Group](https://attack.mitre.org/groups/G0032)   |

[Lazarus Group](https://attack.mitre.org/groups/G0032) has used a custom secure delete function to overwrite file contents with data from heap memory.[[30]](https://web.archive.org/web/20160226161828/https://www.operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Report.pdf)

|  [S9039](https://attack.mitre.org/software/S9039)   |  [LazyWiper](https://attack.mitre.org/software/S9039)   |

[LazyWiper](https://attack.mitre.org/software/S9039) has overwritten files with pseudorandom 32‑byte sequences written at 16‑byte intervals making the file unrecoverable.[[23]](https://cert.pl/uploads/docs/CERT_Polska_Energy_Sector_Incident_Report_2025.pdf)

|  [S0688](https://attack.mitre.org/software/S0688)   |  [Meteor](https://attack.mitre.org/software/S0688)   |

[Meteor](https://attack.mitre.org/software/S0688) can fill a victim's files and directories with zero-bytes in replacement of real content before deleting them.[[31]](https://research.checkpoint.com/2021/indra-hackers-behind-recent-attacks-on-iran/)

|  [S9043](https://attack.mitre.org/software/S9043)   |  [Mini Shai-Hulud](https://attack.mitre.org/software/S9043)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has wiped data on devices that fall within specified parameters to include those that resolve to specific geolocations including Iran and Israel.[[32]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[33]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/) [Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has also implemented a dead-man’s switch that wipes the victims home directory if the operator revokes a GitHub token created by the adversary. [[34]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  [S1135](https://attack.mitre.org/software/S1135)   |  [MultiLayer Wiper](https://attack.mitre.org/software/S1135)   |

[MultiLayer Wiper](https://attack.mitre.org/software/S1135) deletes files on network drives, but corrupts and overwrites with random data files stored locally.[[35]](https://unit42.paloaltonetworks.com/agonizing-serpens-targets-israeli-tech-higher-ed-sectors/)

|  [S0365](https://attack.mitre.org/software/S0365)   |  [Olympic Destroyer](https://attack.mitre.org/software/S0365)   |

[Olympic Destroyer](https://attack.mitre.org/software/S0365) overwrites files locally and on remote shares.[[6]](https://blog.talosintelligence.com/2018/02/olympic-destroyer.html)[[36]](https://www.justice.gov/opa/press-release/file/1328521/download)

|  [S0139](https://attack.mitre.org/software/S0139)   |  [PowerDuke](https://attack.mitre.org/software/S0139)   |

[PowerDuke](https://attack.mitre.org/software/S0139) has a command to write random data across a file and delete it.[[37]](https://www.volexity.com/blog/2016/11/09/powerduke-post-election-spear-phishing-campaigns-targeting-think-tanks-and-ngos/)

|  [S0238](https://attack.mitre.org/software/S0238)   |  [Proxysvc](https://attack.mitre.org/software/S0238)   |

[Proxysvc](https://attack.mitre.org/software/S0238) can overwrite files indicated by the attacker before deleting them.[[38]](https://securingtomorrow.mcafee.com/mcafee-labs/analyzing-operation-ghostsecret-attack-seeks-to-steal-data-worldwide/)

|  [S0364](https://attack.mitre.org/software/S0364)   |  [RawDisk](https://attack.mitre.org/software/S0364)   |

[RawDisk](https://attack.mitre.org/software/S0364) was used in [Shamoon](https://attack.mitre.org/software/S0140) to write to protected system locations such as the MBR and disk partitions in an effort to destroy data.[[3]](http://researchcenter.paloaltonetworks.com/2016/11/unit42-shamoon-2-return-disttrack-wiper/)[[5]](https://unit42.paloaltonetworks.com/shamoon-3-targets-oil-gas-organization/)

|  [S0496](https://attack.mitre.org/software/S0496)   |  [REvil](https://attack.mitre.org/software/S0496)   |

[REvil](https://attack.mitre.org/software/S0496) has the capability to destroy files and folders.[[39]](https://securelist.com/sodin-ransomware/91473/)[[40]](https://www.secureworks.com/blog/revil-the-gandcrab-connection)[[41]](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/mcafee-atr-analyzes-sodinokibi-aka-revil-ransomware-as-a-service-what-the-code-tells-us/)[[41]](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/mcafee-atr-analyzes-sodinokibi-aka-revil-ransomware-as-a-service-what-the-code-tells-us/)[[42]](https://intel471.com/blog/revil-ransomware-as-a-service-an-analysis-of-a-ransomware-affiliate-operation/)[[43]](https://www.picussecurity.com/blog/a-brief-history-and-further-technical-analysis-of-sodinokibi-ransomware)[[44]](https://www.secureworks.com/research/revil-sodinokibi-ransomware)

|  [S9030](https://attack.mitre.org/software/S9030)   |  [SameCoin](https://attack.mitre.org/software/S9030)   |

[SameCoin](https://attack.mitre.org/software/S9030) can overwrite designated files on targeted systems with random bytes.[[45]](https://research.checkpoint.com/2024/hamas-affiliated-threat-actor-expands-to-disruptive-activity/)

|  [G0034](https://attack.mitre.org/groups/G0034)   |  [Sandworm Team](https://attack.mitre.org/groups/G0034)   |

[Sandworm Team](https://attack.mitre.org/groups/G0034) has used [CaddyWiper](https://attack.mitre.org/software/S0693), [SDelete](https://attack.mitre.org/software/S0195), and the [BlackEnergy](https://attack.mitre.org/software/S0089) KillDisk component to overwrite files on victim systems. [[46]](https://www.us-cert.gov/ics/alerts/IR-ALERT-H-16-056-01)[[47]](https://www.welivesecurity.com/2017/06/30/telebots-back-supply-chain-attacks-against-ukraine/)[[9]](https://www.mandiant.com/resources/blog/sandworm-disrupts-power-ukraine-operational-technology) Additionally, [Sandworm Team](https://attack.mitre.org/groups/G0034) has used the JUNKMAIL tool to overwrite files with null bytes.[[48]](https://services.google.com/fh/files/misc/apt44-unearthing-sandworm.pdf)

|  [S0195](https://attack.mitre.org/software/S0195)   |  [SDelete](https://attack.mitre.org/software/S0195)   |

[SDelete](https://attack.mitre.org/software/S0195) deletes data in a way that makes it unrecoverable.[[49]](https://docs.microsoft.com/en-us/sysinternals/downloads/sdelete)

|  [S9008](https://attack.mitre.org/software/S9008)   |  [Shai-Hulud](https://attack.mitre.org/software/S9008)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has destroyed the victim’s home directory by overwriting and deleting every writable file within the user's home folder.[[50]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[51]](https://socket.dev/blog/shai-hulud-strikes-again-v2) [Shai-Hulud](https://attack.mitre.org/software/S9008) has also utilized the shred command on Linux devices.[[52]](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)

|  [S0140](https://attack.mitre.org/software/S0140)   |  [Shamoon](https://attack.mitre.org/software/S0140)   |

[Shamoon](https://attack.mitre.org/software/S0140) attempts to overwrite operating system files and disk structures with image files.[[1]](https://www.symantec.com/connect/blogs/shamoon-attacks)[[2]](https://web.archive.org/web/20210126065851/https://www.fireeye.com/blog/threat-research/2016/11/fireeye_respondsto.html)[[3]](http://researchcenter.paloaltonetworks.com/2016/11/unit42-shamoon-2-return-disttrack-wiper/) In a later variant, randomly generated data was used for data overwrites.[[5]](https://unit42.paloaltonetworks.com/shamoon-3-targets-oil-gas-organization/)[[53]](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/shamoon-returns-to-wipe-systems-in-middle-east-europe/)

|  [G1057](https://attack.mitre.org/groups/G1057)   |  [ShinyHunters](https://attack.mitre.org/groups/G1057)   |

[ShinyHunters](https://attack.mitre.org/groups/G1057) has executed the DeleteBucket API call to delete buckets.[[54]](https://unit42.paloaltonetworks.com/shinyhunters-ransomware-extortion/)

|  [S1178](https://attack.mitre.org/software/S1178)   |  [ShrinkLocker](https://attack.mitre.org/software/S1178)   |

[ShrinkLocker](https://attack.mitre.org/software/S1178) can initiate a destructive payload depending on the operating system check through resizing and reformatting portions of the victim machine's disk, leading to system instability and potential data corruption.[[55]](https://www.splunk.com/en_us/blog/security/shrinklocker-malware-abusing-bitlocker-to-lock-your-data.html)

|  [S0380](https://attack.mitre.org/software/S0380)   |  [StoneDrill](https://attack.mitre.org/software/S0380)   |

[StoneDrill](https://attack.mitre.org/software/S0380) has a disk wiper module that targets files other than those in the Windows directory.[[4]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07180722/Report_Shamoon_StoneDrill_final.pdf)

|  [G1053](https://attack.mitre.org/groups/G1053)   |  [Storm-0501](https://attack.mitre.org/groups/G1053)   |

[Storm-0501](https://attack.mitre.org/groups/G1053) has destroyed data and backup files.[[56]](https://www.microsoft.com/en-us/security/blog/2025/08/27/storm-0501s-evolving-techniques-lead-to-cloud-based-ransomware/)

|  [G1056](https://attack.mitre.org/groups/G1056)   |  [TeamPCP](https://attack.mitre.org/groups/G1056)   |

[TeamPCP](https://attack.mitre.org/groups/G1056) has deployed privileged DaemonSets to delete files on Kubernetes clusters and has executed recursive file deletions on non-containerized hosts.[[20]](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)

|  [G1055](https://attack.mitre.org/groups/G1055)   |  [VOID MANTICORE](https://attack.mitre.org/groups/G1055)   |

[VOID MANTICORE](https://attack.mitre.org/groups/G1055) has conducted data wiping attacks on compromised systems.[[57]](https://research.checkpoint.com/2026/handala-hack-unveiling-groups-modus-operandi/)[[58]](https://specopssoft.com/blog/stryker-cyber-attack-what-we-know-remote-wipe/)[[59]](https://www.justice.gov/opa/media/1431956/dl?inline)[[60]](https://unit42.paloaltonetworks.com/evolution-of-iran-cyber-threats/) [VOID MANTICORE](https://attack.mitre.org/groups/G1055) has also manually deleted files from compromised hosts, to include selecting all files and then deleting them.[[57]](https://research.checkpoint.com/2026/handala-hack-unveiling-groups-modus-operandi/)[[59]](https://www.justice.gov/opa/media/1431956/dl?inline)

|  [S0689](https://attack.mitre.org/software/S0689)   |  [WhisperGate](https://attack.mitre.org/software/S0689)   |

[WhisperGate](https://attack.mitre.org/software/S0689) can corrupt files by overwriting the first 1 MB with 0xcc and appending random extensions.[[61]](https://www.microsoft.com/security/blog/2022/01/15/destructive-malware-targeting-ukrainian-organizations/)[[62]](https://www.crowdstrike.com/blog/technical-analysis-of-whispergate-malware)[[63]](https://www.cybereason.com/blog/cybereason-vs.-whispergate-wiper)[[64]](https://unit42.paloaltonetworks.com/ukraine-cyber-conflict-cve-2021-32648-whispergate/#whispergate-malware-family)[[65]](https://blog.talosintelligence.com/2022/01/ukraine-campaign-delivers-defacement.html)[[66]](https://medium.com/s2wblog/analysis-of-destructive-malware-whispergate-targeting-ukraine-9d5d158f19f3)

|  [S0341](https://attack.mitre.org/software/S0341)   |  [Xbash](https://attack.mitre.org/software/S0341)   |

[Xbash](https://attack.mitre.org/software/S0341) has destroyed Linux-based databases as part of its ransomware capabilities.[[67]](https://researchcenter.paloaltonetworks.com/2018/09/unit42-xbash-combines-botnet-ransomware-coinmining-worm-targets-linux-windows/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1053](https://attack.mitre.org/mitigations/M1053)   |  [Data Backup](https://attack.mitre.org/mitigations/M1053)   |

Consider implementing IT disaster recovery plans that contain procedures for taking regular data backups that can be used to restore organizational data.[[68]](https://www.ready.gov/business/implementation/IT) Ensure backups are stored off system and protected from common methods adversaries may use to gain access and destroy the backups to prevent recovery.

|  [M1032](https://attack.mitre.org/mitigations/M1032)   |  [Multi-factor Authentication](https://attack.mitre.org/mitigations/M1032)   |

Implement multi-factor authentication (MFA) delete for cloud storage resources, such as AWS S3 buckets, to prevent unauthorized deletion of critical data and infrastructure. MFA delete requires additional authentication steps, making it significantly more difficult for adversaries to destroy data without proper credentials. This additional security layer helps protect against the impact of data destruction in cloud environments by ensuring that only authenticated actions can irreversibly delete storage or machine images.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

In cloud environments, limit permissions to modify cloud bucket lifecycle policies (e.g., PutLifecycleConfiguration in AWS) to only those accounts that require it. In AWS environments, consider using Service Control policies to limit the use of the PutBucketLifecycle API call.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0146](https://attack.mitre.org/detectionstrategies/DET0146)   |  [Detection of Data Destruction Across Platforms via Mass Overwrite and Deletion Patterns](https://attack.mitre.org/detectionstrategies/DET0146)   |  [AN0411](https://attack.mitre.org/detectionstrategies/DET0146#AN0411)   |

Adversary spawns command-line tools (e.g., del, cipher /w, SDelete) or scripts to recursively delete or overwrite user/system files. This may be correlated with abnormal file IO activity, registry writes, or tampering in critical system directories.

|  [AN0412](https://attack.mitre.org/detectionstrategies/DET0146#AN0412)   |

Massive recursive deletions or overwrites via rm -rf, shred, dd, or wiper binaries. May include unlink syscalls, deletion of known config/data paths, or sequential overwrite patterns.

|  [AN0413](https://attack.mitre.org/detectionstrategies/DET0146#AN0413)   |

Destruction via rm -rf, overwrite with dd or srm, often executed by script in /tmp or /private/tmp, may also involve file overwrite to political or decoy image data.

|  [AN0414](https://attack.mitre.org/detectionstrategies/DET0146#AN0414)   |

Adversary deletes critical infrastructure: EC2 instances, S3 buckets, snapshots, or volumes using elevated IAM credentials. Frequently includes batch API calls with Delete* or TerminateInstances.

|  [AN0415](https://attack.mitre.org/detectionstrategies/DET0146#AN0415)   |

Adversary destroys virtual disks (VMDK), images, or VMs by invoking vim-cmd, deleting datastore contents, or purging snapshots.

|  [AN0416](https://attack.mitre.org/detectionstrategies/DET0146#AN0416)   |

Container process executes destructive file operations inside volume mounts or host paths. Includes rm -rf /mnt/volumes/, container breakout followed by host deletion attempts.

## References

-    [Symantec. (2012, August 16). The Shamoon Attacks. Retrieved March 14, 2019.](https://www.symantec.com/connect/blogs/shamoon-attacks)
-    [FireEye. (2016, November 30). FireEye Responds to Wave of Destructive Cyber Attacks in Gulf Region. Retrieved November 17, 2024.](https://web.archive.org/web/20210126065851/https://www.fireeye.com/blog/threat-research/2016/11/fireeye_respondsto.html)
-    [Falcone, R.. (2016, November 30). Shamoon 2: Return of the Disttrack Wiper. Retrieved January 11, 2017.](http://researchcenter.paloaltonetworks.com/2016/11/unit42-shamoon-2-return-disttrack-wiper/)
-    [Kaspersky Lab. (2017, March 7). From Shamoon to StoneDrill: Wipers attacking Saudi organizations and beyond. Retrieved March 14, 2019.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07180722/Report_Shamoon_StoneDrill_final.pdf)
-    [Falcone, R. (2018, December 13). Shamoon 3 Targets Oil and Gas Organization. Retrieved March 14, 2019.](https://unit42.paloaltonetworks.com/shamoon-3-targets-oil-gas-organization/)
-    [Mercer, W. and Rascagneres, P. (2018, February 12). Olympic Destroyer Takes Aim At Winter Olympics. Retrieved March 14, 2019.](https://blog.talosintelligence.com/2018/02/olympic-destroyer.html)
-    [Mimoso, M.. (2014, June 18). Hacker Puts Hosting Service Code Spaces Out of Business. Retrieved December 15, 2020.](https://threatpost.com/hacker-puts-hosting-service-code-spaces-out-of-business/106761/)
-    [DOJ. (2020, August 26). San Jose Man Pleads Guilty To Damaging Cisco’s Network. Retrieved December 15, 2020.](https://www.justice.gov/usao-ndca/pr/san-jose-man-pleads-guilty-damaging-cisco-s-network)
-    [Ken Proska, John Wolfram, Jared Wilson, Dan Black, Keith Lunden, Daniel Kapellmann Zafra, Nathan Brubaker, Tyler Mclellan, Chris Sistrunk. (2023, November 9). Sandworm Disrupts Power in Ukraine Using a Novel Attack Against Operational Technology. Retrieved March 28, 2024.](https://www.mandiant.com/resources/blog/sandworm-disrupts-power-ukraine-operational-technology)
-    [ESET. (2026, January 30). Russian Sandworm group attacks energy company in Poland with DynoWiper, ESET Research discovers. Retrieved April 22, 2026.](https://www.eset.com/us/about/newsroom/research/eset-research-russian-sandwormapt-attacks-energy-company-poland-with-dynowiper/)
-    [ESET. (2026, January 30). DynoWiper update: Technical analysis and attribution. Retrieved April 22, 2026.](https://www.welivesecurity.com/en/eset-research/dynowiper-update-technical-analysis-attribution/)
-    [Juan Andrés Guerrero-Saade & Tom Hegel. (2024, March 21). AcidPour | New Embedded Wiper Variant of AcidRain Appears in Ukraine. Retrieved November 25, 2024.](https://www.sentinelone.com/labs/acidpour-new-embedded-wiper-variant-of-acidrain-appears-in-ukraine/)
-    [Juan Andres Guerrero-Saade and Max van Amerongen, SentinelOne. (2022, March 31). AcidRain | A Modem Wiper Rains Down on Europe. Retrieved March 25, 2024.](https://www.sentinelone.com/labs/acidrain-a-modem-wiper-rains-down-on-europe/)
-    [Amitai Ben & Shushan Ehrlich. (2021, May). From Wiper to Ransomware: The Evolution of Agrius. Retrieved May 21, 2024.](https://assets.sentinelone.com/sentinellabs/evol-agrius)
-    [FireEye. (2018, October 03). APT38: Un-usual Suspects. Retrieved November 17, 2024.](https://services.google.com/fh/files/misc/apt38-un-usual-suspects.pdf)
-    [Baumgartner, K. and Garnaeva, M.. (2015, February 17). BE2 extraordinary plugins, Siemens targeting, dev fails. Retrieved March 24, 2016.](https://securelist.com/be2-extraordinary-plugins-siemens-targeting-dev-fails/68838/)
-    [Cherepanov, A.. (2016, January 3). BlackEnergy by the SSHBearDoor: attacks against Ukrainian news media and electric industry . Retrieved June 10, 2020.](https://www.welivesecurity.com/2016/01/03/blackenergy-sshbeardoor-details-2015-attacks-ukrainian-news-media-electric-industry/)
-    [ESET. (2022, March 15). CaddyWiper: New wiper malware discovered in Ukraine. Retrieved March 23, 2022.](https://www.welivesecurity.com/2022/03/15/caddywiper-new-wiper-malware-discovered-ukraine)
-    [Malhotra, A. (2022, March 15). Threat Advisory: CaddyWiper. Retrieved March 23, 2022.](https://blog.talosintelligence.com/2022/03/threat-advisory-caddywiper.html)
-    [Unit 42. (2026, March 31). Weaponizing the Protectors: TeamPCP’s Multi-Stage Supply Chain Attack on Security Infrastructure. Retrieved July 1, 2026.](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)
-    [Eriksen, C. (2026, March 22). CanisterWorm Gets Teeth: TeamPCP's Kubernetes Wiper Targets Iran. Retrieved July 27, 2026.](https://www.aikido.dev/blog/teampcp-stage-payload-canisterworm-iran)
-    [Neeamni, D., Rubinfeld, A.. (2021, July 1). Diavol - A New Ransomware Used By Wizard Spider?. Retrieved November 12, 2021.](https://www.fortinet.com/blog/threat-research/diavol-new-ransomware-used-by-wizard-spider)
-    [CERT Polska. (2026, January 30). Energy Sector Incident Report – 29 December. Retrieved April 22, 2026.](https://cert.pl/uploads/docs/CERT_Polska_Energy_Sector_Incident_Report_2025.pdf)
-    [ESET. (2022, March 1). IsaacWiper and HermeticWizard: New wiper and worm targetingUkraine. Retrieved April 10, 2022.](https://www.welivesecurity.com/2022/03/01/isaacwiper-hermeticwizard-wiper-worm-targeting-ukraine)
-    [Dragos Inc.. (2017, June 13). CRASHOVERRIDE Analysis of the Threat to Electric Grid Operations. Retrieved December 18, 2020.](https://dragos.com/blog/crashoverride/CrashOverride-01.pdf)
-    [Levene, B, et al. (2017, May 03). Kazuar: Multiplatform Espionage Backdoor with API Access. Retrieved July 17, 2018.](https://researchcenter.paloaltonetworks.com/2017/05/unit42-kazuar-multiplatform-espionage-backdoor-api-access/)
-    [Cherepanov, A.. (2016, January 3). BlackEnergy by the SSHBearDoor: attacks against Ukrainian news media and electric industry. Retrieved May 18, 2016.](http://www.welivesecurity.com/2016/01/03/blackenergy-sshbeardoor-details-2015-attacks-ukrainian-news-media-electric-industry/)
-    [MSTIC, DART, M365 Defender. (2022, March 24). DEV-0537 Criminal Actor Targeting Organizations for Data Exfiltration and Destruction. Retrieved May 17, 2022.](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)
-    [Brown, D., et al. (2022, April 28). LAPSUS$: Recent techniques, tactics and procedures. Retrieved December 22, 2022.](https://www.nccgroup.com/research-blog/lapsus-recent-techniques-tactics-and-procedures/)
-    [Novetta Threat Research Group. (2016, February 24). Operation Blockbuster: Unraveling the Long Thread of the Sony Attack. Retrieved February 25, 2016.](https://web.archive.org/web/20160226161828/https://www.operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Report.pdf)
-    [Check Point Research Team. (2021, August 14). Indra - Hackers Behind Recent Attacks on Iran. Retrieved February 17, 2022.](https://research.checkpoint.com/2021/indra-hackers-behind-recent-attacks-on-iran/)
-    [Hunt.io. (2026, May 14). How TeamPCP's Python Toolkit Survives a C2 Takedown: FIRESCALE, GitHub, and the Victim's Own Account. Retrieved July 16, 2026.](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)
-    [Webb, M. (2026, May 20). TeamPCP Wave Four: GitHub Breach via Poisoned VS Code Extension, durabletask PyPI Worm, and ~4,000 Internal Repositories Exfiltrated. Retrieved July 16, 2026.](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)
-    [Flashpoint. (2026, May 28). The Mini Shai-Hulud Worm and the New Era of CI/CD Exploitation. Retrieved July 16, 2026.](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

-    [Or Chechik, Tom Fakterman, Daniel Frank & Assaf Dahan. (2023, November 6). Agonizing Serpens (Aka Agrius) Targeting the Israeli Higher Education and Tech Sectors. Retrieved May 22, 2024.](https://unit42.paloaltonetworks.com/agonizing-serpens-targets-israeli-tech-higher-ed-sectors/)
-    [Scott W. Brady. (2020, October 15). United States vs. Yuriy Sergeyevich Andrienko et al.. Retrieved November 25, 2020.](https://www.justice.gov/opa/press-release/file/1328521/download)
-    [Adair, S.. (2016, November 9). PowerDuke: Widespread Post-Election Spear Phishing Campaigns Targeting Think Tanks and NGOs. Retrieved January 11, 2017.](https://www.volexity.com/blog/2016/11/09/powerduke-post-election-spear-phishing-campaigns-targeting-think-tanks-and-ngos/)
-    [Sherstobitoff, R., Malhotra, A. (2018, April 24). Analyzing Operation GhostSecret: Attack Seeks to Steal Data Worldwide. Retrieved May 16, 2018.](https://securingtomorrow.mcafee.com/mcafee-labs/analyzing-operation-ghostsecret-attack-seeks-to-steal-data-worldwide/)
-    [Mamedov, O, et al. (2019, July 3). Sodin ransomware exploits Windows vulnerability and processor architecture. Retrieved August 4, 2020.](https://securelist.com/sodin-ransomware/91473/)
-    [Secureworks . (2019, September 24). REvil: The GandCrab Connection. Retrieved August 4, 2020.](https://www.secureworks.com/blog/revil-the-gandcrab-connection)
-    [McAfee. (2019, October 2). McAfee ATR Analyzes Sodinokibi aka REvil Ransomware-as-a-Service – What The Code Tells Us. Retrieved August 4, 2020.](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/mcafee-atr-analyzes-sodinokibi-aka-revil-ransomware-as-a-service-what-the-code-tells-us/)
-    [Intel 471 Malware Intelligence team. (2020, March 31). REvil Ransomware-as-a-Service – An analysis of a ransomware affiliate operation. Retrieved August 4, 2020.](https://intel471.com/blog/revil-ransomware-as-a-service-an-analysis-of-a-ransomware-affiliate-operation/)
-    [Ozarslan, S. (2020, January 15). A Brief History of Sodinokibi. Retrieved August 5, 2020.](https://www.picussecurity.com/blog/a-brief-history-and-further-technical-analysis-of-sodinokibi-ransomware)
-    [Counter Threat Unit Research Team. (2019, September 24). REvil/Sodinokibi Ransomware. Retrieved August 4, 2020.](https://www.secureworks.com/research/revil-sodinokibi-ransomware)
-    [Check Point. (2024, November 12). Hamas-affiliated Threat Actor WIRTE Continues its Middle East Operations and Moves to Disruptive Activity. Retrieved April 20, 2026.](https://research.checkpoint.com/2024/hamas-affiliated-threat-actor-expands-to-disruptive-activity/)
-    [US-CERT. (2016, February 25). ICS Alert (IR-ALERT-H-16-056-01) Cyber-Attack Against Ukrainian Critical Infrastructure. Retrieved June 10, 2020.](https://www.us-cert.gov/ics/alerts/IR-ALERT-H-16-056-01)
-    [Cherepanov, A.. (2017, June 30). TeleBots are back: Supply chain attacks against Ukraine. Retrieved June 11, 2020.](https://www.welivesecurity.com/2017/06/30/telebots-back-supply-chain-attacks-against-ukraine/)
-    [Roncone, G. et al. (n.d.). APT44: Unearthing Sandworm. Retrieved July 11, 2024.](https://services.google.com/fh/files/misc/apt44-unearthing-sandworm.pdf)
-    [Russinovich, M. (2016, July 4). SDelete v2.0. Retrieved February 8, 2018.](https://docs.microsoft.com/en-us/sysinternals/downloads/sdelete)
-    [Justin Moore. (2025, November 25). "Shai-Hulud" Worm Compromises npm Ecosystem in Supply Chain Attack (Updated November 26). Retrieved April 9, 2026.](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)
-    [Socket Research Team. (2025, November 24). Shai Hulud Strikes Again (v2). Retrieved April 9, 2026.](https://socket.dev/blog/shai-hulud-strikes-again-v2)
-    [Microsoft Defender Security Team. (n.d.). Shai-Hulud 2.0: Guidance for detecting, investigating, and defending against the supply chain attack. Retrieved April 9, 2026.](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)
-    [Mundo, A., Roccia, T., Saavedra-Morales, J., Beek, C.. (2018, December 14). Shamoon Returns to Wipe Systems in Middle East, Europe . Retrieved May 29, 2020.](https://www.mcafee.com/blogs/other-blogs/mcafee-labs/shamoon-returns-to-wipe-systems-in-middle-east-europe/)
-    [Kelley, M., Vaya, C. (2024, August 23). Bling Libra’s Tactical Evolution: The Threat Actor Group Behind ShinyHunters Ransomware. Retrieved May 18, 2026.](https://unit42.paloaltonetworks.com/shinyhunters-ransomware-extortion/)
-    [Splunk Threat Research Team , Teoderick Contreras. (2024, September 5). ShrinkLocker Malware: Abusing BitLocker to Lock Your Data. Retrieved December 7, 2024.](https://www.splunk.com/en_us/blog/security/shrinklocker-malware-abusing-bitlocker-to-lock-your-data.html)
-    [Microsoft Threat Intelligence. (2025, August 27). Storm-0501’s evolving techniques lead to cloud-based ransomware. Retrieved October 19, 2025.](https://www.microsoft.com/en-us/security/blog/2025/08/27/storm-0501s-evolving-techniques-lead-to-cloud-based-ransomware/)
-    [Check Point Research. (2026, March 12). “Handala Hack” – Unveiling Group’s Modus Operandi. Retrieved April 20, 2026.](https://research.checkpoint.com/2026/handala-hack-unveiling-groups-modus-operandi/)
-    [David Ketler. (2026, March 30). Stryker Cyber-Attack: What we Know so Far About the Remote Wipe Attack. Retrieved April 20, 2026.](https://specopssoft.com/blog/stryker-cyber-attack-what-we-know-remote-wipe/)
-    [DOJ/FBI. (2026, March 19). Case 1:26-mj-00683-CDA: Affidavit in Support of Seizure Warrant: In the Matter of the Seizure of Domain Names Justicehomeland[.]org; karmabelow80[.]org; handala-hack[.]to; and handala-redwatned[.]to. Retrieved April 20, 2026.](https://www.justice.gov/opa/media/1431956/dl?inline)
-    [Justin Moore. (2026, March 16). Iranian Cyber Threat Evolution: From MBR Wipers to Identity Weaponization. Retrieved April 20, 2026.](https://unit42.paloaltonetworks.com/evolution-of-iran-cyber-threats/)
-    [MSTIC. (2022, January 15). Destructive malware targeting Ukrainian organizations. Retrieved March 10, 2022.](https://www.microsoft.com/security/blog/2022/01/15/destructive-malware-targeting-ukrainian-organizations/)
-    [Crowdstrike. (2022, January 19). Technical Analysis of the WhisperGate Malicious Bootloader. Retrieved March 10, 2022.](https://www.crowdstrike.com/blog/technical-analysis-of-whispergate-malware)
-    [Cybereason Nocturnus. (2022, February 15). Cybereason vs. WhisperGate and HermeticWiper. Retrieved March 10, 2022.](https://www.cybereason.com/blog/cybereason-vs.-whispergate-wiper)
-    [Falcone, R. et al.. (2022, January 20). Threat Brief: Ongoing Russia and Ukraine Cyber Conflict. Retrieved March 10, 2022.](https://unit42.paloaltonetworks.com/ukraine-cyber-conflict-cve-2021-32648-whispergate/#whispergate-malware-family)
-    [Biasini, N. et al.. (2022, January 21). Ukraine Campaign Delivers Defacement and Wipers, in Continued Escalation. Retrieved March 14, 2022.](https://blog.talosintelligence.com/2022/01/ukraine-campaign-delivers-defacement.html)
-    [S2W. (2022, January 18). Analysis of Destructive Malware (WhisperGate) targeting Ukraine. Retrieved March 14, 2022.](https://medium.com/s2wblog/analysis-of-destructive-malware-whispergate-targeting-ukraine-9d5d158f19f3)
-    [Xiao, C. (2018, September 17). Xbash Combines Botnet, Ransomware, Coinmining in Worm that Targets Linux and Windows. Retrieved November 14, 2018.](https://researchcenter.paloaltonetworks.com/2018/09/unit42-xbash-combines-botnet-ransomware-coinmining-worm-targets-linux-windows/)
-    [Ready.gov. (n.d.). IT Disaster Recovery Plan. Retrieved March 15, 2019.](https://www.ready.gov/business/implementation/IT)

×
