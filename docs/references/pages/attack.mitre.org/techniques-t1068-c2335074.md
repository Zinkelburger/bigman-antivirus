---
url: "https://attack.mitre.org/techniques/T1068/"
title: "Exploitation for Privilege Escalation, Technique T1068 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:30:11Z"
via: "live"
sha256: "8e9ac61d4fc3474a63424a9ab995736cf3ea7b79f4e9feee8a4338301b94175b"
wayback: "https://web.archive.org/web/20261023090815/https://attack.mitre.org/techniques/T1068/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Exploitation for Privilege Escalation, Technique T1068 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1068/>
> Snapshot: <https://web.archive.org/web/20261023090815/https://attack.mitre.org/techniques/T1068/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Exploitation for Privilege Escalation

#  Exploitation for Privilege Escalation

Adversaries may exploit software vulnerabilities in an attempt to elevate privileges. Exploitation of a software vulnerability occurs when an adversary takes advantage of a programming error in a program, service, or within the operating system software or kernel itself to execute adversary-controlled code. Security constructs such as permission levels will often hinder access to information and use of certain techniques, so adversaries will likely need to perform privilege escalation to include use of software exploitation to circumvent those restrictions.

When initially gaining access to a system, an adversary may be operating within a lower privileged process which will prevent them from accessing certain resources on the system. Vulnerabilities may exist, usually in operating system components and software commonly running at higher permissions, that can be exploited to gain higher levels of access on the system. This could enable someone to move from unprivileged or user level permissions to SYSTEM or root permissions depending on the component that is vulnerable. This could also enable an adversary to move from a virtualized environment, such as within a virtual machine or container, onto the underlying host. This may be a necessary step for an adversary compromising an endpoint system that has been properly configured and limits other privilege escalation methods.

Adversaries may bring a signed vulnerable driver onto a compromised machine so that they can exploit the vulnerability to execute code in kernel mode. This process is sometimes referred to as Bring Your Own Vulnerable Driver (BYOVD).[[1]](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_InvisiMole.pdf)[[2]](https://unit42.paloaltonetworks.com/acidbox-rare-malware/) Adversaries may include the vulnerable driver with files delivered during Initial Access or download it to a compromised system via [Ingress Tool Transfer](https://attack.mitre.org/techniques/T1105) or [Lateral Tool Transfer](https://attack.mitre.org/techniques/T1570).

ID: T1068

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Privilege Escalation](https://attack.mitre.org/tactics/TA0004)

ⓘ

Platforms: Containers, Linux, Windows, macOS

Contributors: David Tayouri; Idan Revivo, @idanr86, Team Nautilus Aqua Security; Joas Antonio dos Santos, @C0d3Cr4zy, Inmetrics; Yaniv Agman, @AgmanYaniv, Team Nautilus Aqua Security

Version: 1.6

Created: 31 May 2017

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1068/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1068/)

## Procedure Examples

| ID  | Name  | Description

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[APT28](https://attack.mitre.org/groups/G0007) has exploited CVE-2014-4076, CVE-2015-2387, CVE-2015-1701, CVE-2017-0263, and CVE-2022-38028 to escalate privileges.[[3]](https://download.bitdefender.com/resources/media/materials/white-papers/en/Bitdefender_In-depth_analysis_of_APT28%E2%80%93The_Political_Cyber-Espionage.pdf)[[4]](http://download.microsoft.com/download/4/4/C/44CDEF0E-7924-4787-A56A-16261691ACE3/Microsoft_Security_Intelligence_Report_Volume_19_English.pdf)[[5]](https://securelist.com/a-slice-of-2017-sofacy-activity/83930/)[[6]](https://www.volexity.com/blog/2024/11/22/the-nearest-neighbor-attack-how-a-russian-apt-weaponized-nearby-wi-fi-networks-for-covert-access/)

|  [G0016](https://attack.mitre.org/groups/G0016)   |  [APT29](https://attack.mitre.org/groups/G0016)   |

[APT29](https://attack.mitre.org/groups/G0016) has exploited CVE-2021-36934 to escalate privileges on a compromised host.[[7]](https://www.welivesecurity.com/wp-content/uploads/2022/02/eset_threat_report_t32021.pdf)

|  [G0050](https://attack.mitre.org/groups/G0050)   |  [APT32](https://attack.mitre.org/groups/G0050)   |

[APT32](https://attack.mitre.org/groups/G0050) has used CVE-2016-7255 to escalate privileges.[[8]](https://www.fireeye.com/blog/threat-research/2017/05/cyber-espionage-apt32.html)

|  [G0064](https://attack.mitre.org/groups/G0064)   |  [APT33](https://attack.mitre.org/groups/G0064)   |

[APT33](https://attack.mitre.org/groups/G0064) has used a publicly available exploit for CVE-2017-0213 to escalate privileges on a local system.[[9]](https://www.fireeye.com/blog/threat-research/2018/12/overruled-containing-a-potentially-destructive-adversary.html)

|  [G1002](https://attack.mitre.org/groups/G1002)   |  [BITTER](https://attack.mitre.org/groups/G1002)   |

[BITTER](https://attack.mitre.org/groups/G1002) has exploited CVE-2021-1732 for privilege escalation.[[10]](https://ti.dbappsecurity.com.cn/blog/articles/2021/02/10/windows-kernel-zero-day-exploit-is-used-by-bitter-apt-in-targeted-attack/)[[11]](https://msrc.microsoft.com/update-guide/vulnerability/CVE-2021-1732)

|  [G1043](https://attack.mitre.org/groups/G1043)   |  [BlackByte](https://attack.mitre.org/groups/G1043)   |

[BlackByte](https://attack.mitre.org/groups/G1043) has exploited CVE-2024-37085 in VMWare ESXi software for authentication bypass and subsequent privilege escalation.[[12]](https://blog.talosintelligence.com/blackbyte-blends-tried-and-true-tradecraft-with-newly-disclosed-vulnerabilities-to-support-ongoing-attacks/)

|  [S1181](https://attack.mitre.org/software/S1181)   |  [BlackByte 2.0 Ransomware](https://attack.mitre.org/software/S1181)   |

[BlackByte 2.0 Ransomware](https://attack.mitre.org/software/S1181) exploits a vulnerability in the RTCore64.sys driver (CVE-2019-16098) to enable privilege escalation and defense evasion when run as a service.[[13]](https://www.microsoft.com/en-us/security/blog/2023/07/06/the-five-day-job-a-blackbyte-ransomware-intrusion-case-study/)

|  [S0484](https://attack.mitre.org/software/S0484)   |  [Carberp](https://attack.mitre.org/software/S0484)   |

[Carberp](https://attack.mitre.org/software/S0484) has exploited multiple Windows vulnerabilities (CVE-2010-2743, CVE-2010-3338, CVE-2010-4398, CVE-2008-1084) and a .NET Runtime Optimization vulnerability for privilege escalation.[[14]](https://www.eset.com/fileadmin/eset/US/resources/docs/white-papers/white-papers-win-32-carberp.pdf)[[15]](https://web.archive.org/web/20231227000328/http://pxnow.prevx.com/content/blog/carberp-a_modular_information_stealing_trojan.pdf)

|  [G0080](https://attack.mitre.org/groups/G0080)   |  [Cobalt Group](https://attack.mitre.org/groups/G0080)   |

[Cobalt Group](https://attack.mitre.org/groups/G0080) has used exploits to increase their levels of rights and privileges.[[16]](https://www.group-ib.com/blog/cobalt)

|  [S0154](https://attack.mitre.org/software/S0154)   |  [Cobalt Strike](https://attack.mitre.org/software/S0154)   |

[Cobalt Strike](https://attack.mitre.org/software/S0154) can exploit vulnerabilities such as MS14-058.[[17]](https://web.archive.org/web/20210924171429/https://www.cobaltstrike.com/downloads/reports/tacticstechniquesandprocedures.pdf)[[18]](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)

|  [S0050](https://attack.mitre.org/software/S0050)   |  [CosmicDuke](https://attack.mitre.org/software/S0050)   |

[CosmicDuke](https://attack.mitre.org/software/S0050) attempts to exploit privilege escalation vulnerabilities CVE-2010-0232 or CVE-2010-4398.[[19]](https://www.f-secure.com/documents/996508/1030745/dukes_whitepaper.pdf)

|  [S1247](https://attack.mitre.org/software/S1247)   |  [Embargo](https://attack.mitre.org/software/S1247)   |

[Embargo](https://attack.mitre.org/software/S1247) has leveraged MS4Killer to deliver a vulnerable driver to the victim device, sometimes referred to as Bring Your Own Vulnerable Driver (BYOVD).[[20]](https://www.welivesecurity.com/en/eset-research/embargo-ransomware-rocknrust/) [Embargo](https://attack.mitre.org/software/S1247) has utilized the vulnerable driver probmon.sys version 3.0.0.4 which had a revoked certificated from "ITM System Co.,LTD."[[20]](https://www.welivesecurity.com/en/eset-research/embargo-ransomware-rocknrust/)

|  [S0363](https://attack.mitre.org/software/S0363)   |  [Empire](https://attack.mitre.org/software/S0363)   |

[Empire](https://attack.mitre.org/software/S0363) can exploit vulnerabilities such as MS16-032 and MS16-135.[[21]](https://github.com/PowerShellEmpire/Empire)

|  [G0037](https://attack.mitre.org/groups/G0037)   |  [FIN6](https://attack.mitre.org/groups/G0037)   |

[FIN6](https://attack.mitre.org/groups/G0037) has used tools to exploit Windows vulnerabilities in order to escalate privileges. The tools targeted CVE-2013-3660, CVE-2011-2005, and CVE-2010-4398, all of which could allow local users to access kernel-level privileges.[[22]](https://web.archive.org/web/20190807112824/https://www2.fireeye.com/rs/848-DID-242/images/rpt-fin6.pdf)

|  [G0061](https://attack.mitre.org/groups/G0061)   |  [FIN8](https://attack.mitre.org/groups/G0061)   |

[FIN8](https://attack.mitre.org/groups/G0061) has exploited the CVE-2016-0167 local vulnerability.[[23]](https://www.fireeye.com/blog/threat-research/2016/05/windows-zero-day-payment-cards.html)[[24]](https://www2.fireeye.com/WBNR-Know-Your-Enemy-UNC622-Spear-Phishing.html)

|  [G0125](https://attack.mitre.org/groups/G0125)   |  [HAFNIUM](https://attack.mitre.org/groups/G0125)   |

[HAFNIUM](https://attack.mitre.org/groups/G0125) has targeted unpatched applications to elevate access in targeted organizations.[[25]](https://www.microsoft.com/en-us/security/blog/2025/03/05/silk-typhoon-targeting-it-supply-chain/)

|  [S0601](https://attack.mitre.org/software/S0601)   |  [Hildegard](https://attack.mitre.org/software/S0601)   |

[Hildegard](https://attack.mitre.org/software/S0601) has used the BOtB tool which exploits CVE-2019-5736.[[26]](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)

|  [S0260](https://attack.mitre.org/software/S0260)   |  [InvisiMole](https://attack.mitre.org/software/S0260)   |

[InvisiMole](https://attack.mitre.org/software/S0260) has exploited CVE-2007-5633 vulnerability in the speedfan.sys driver to obtain kernel mode privileges.[[1]](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_InvisiMole.pdf)

|  [S0044](https://attack.mitre.org/software/S0044)   |  [JHUHUGIT](https://attack.mitre.org/software/S0044)   |

[JHUHUGIT](https://attack.mitre.org/software/S0044) has exploited CVE-2015-1701 and CVE-2015-2387 to escalate privileges.[[27]](http://www.welivesecurity.com/wp-content/uploads/2016/10/eset-sednit-part1.pdf)[[28]](http://www.welivesecurity.com/2015/07/10/sednit-apt-group-meets-hacking-team/)

|  [G1004](https://attack.mitre.org/groups/G1004)   |  [LAPSUS$](https://attack.mitre.org/groups/G1004)   |

[LAPSUS$](https://attack.mitre.org/groups/G1004) has exploited unpatched vulnerabilities on internally accessible servers including JIRA, GitLab, and Confluence for privilege escalation.[[29]](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)

|  [C0049](https://attack.mitre.org/campaigns/C0049)   |  [Leviathan Australian Intrusions](https://attack.mitre.org/campaigns/C0049)   |

[Leviathan](https://attack.mitre.org/groups/G0065) exploited software vulnerabilities in victim environments to escalate privileges during [Leviathan Australian Intrusions](https://attack.mitre.org/campaigns/C0049).[[30]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa24-190a)

|  [G1019](https://attack.mitre.org/groups/G1019)   |  [MoustachedBouncer](https://attack.mitre.org/groups/G1019)   |

[MoustachedBouncer](https://attack.mitre.org/groups/G1019) has exploited CVE-2021-1732 to execute malware components with elevated rights.[[31]](https://www.welivesecurity.com/en/eset-research/moustachedbouncer-espionage-against-foreign-diplomats-in-belarus/)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) has exploited the Windows Kernel Elevation of Privilege vulnerability, CVE-2024-30088.[[32]](https://www.trendmicro.com/en_us/research/24/j/earth-simnavaz-cyberattacks.html)

|  [S0664](https://attack.mitre.org/software/S0664)   |  [Pandora](https://attack.mitre.org/software/S0664)   |

[Pandora](https://attack.mitre.org/software/S0664) can use CVE-2017-15303 to bypass Windows Driver Signature Enforcement (DSE) protection and load its driver.[[33]](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)

|  [G0068](https://attack.mitre.org/groups/G0068)   |  [PLATINUM](https://attack.mitre.org/groups/G0068)   |

[PLATINUM](https://attack.mitre.org/groups/G0068) has leveraged a zero-day vulnerability to escalate privileges.[[34]](https://download.microsoft.com/download/2/2/5/225BFE3E-E1DE-4F5B-A77B-71200928D209/Platinum%20feature%20article%20-%20Targeted%20attacks%20in%20South%20and%20Southeast%20Asia%20April%202016.pdf)

|  [S0378](https://attack.mitre.org/software/S0378)   |  [PoshC2](https://attack.mitre.org/software/S0378)   |

[PoshC2](https://attack.mitre.org/software/S0378) contains modules for local privilege escalation exploits such as CVE-2016-9192 and CVE-2016-0099.[[35]](https://github.com/nettitude/PoshC2_Python)

|  [S0654](https://attack.mitre.org/software/S0654)   |  [ProLock](https://attack.mitre.org/software/S0654)   |

[ProLock](https://attack.mitre.org/software/S0654) can use CVE-2019-0859 to escalate privileges on a compromised host.[[36]](https://web.archive.org/web/20220119114433/https://groupib.pathfactory.com/ransomware-reports/prolock_wp)

|  [S0125](https://attack.mitre.org/software/S0125)   |  [Remsec](https://attack.mitre.org/software/S0125)   |

[Remsec](https://attack.mitre.org/software/S0125) has a plugin to drop and execute vulnerable Outpost Sandbox or avast! Virtualization drivers in order to gain kernel mode privileges.[[37]](https://securelist.com/files/2016/07/The-ProjectSauron-APT_Technical_Analysis_KL.pdf)

|  [G1015](https://attack.mitre.org/groups/G1015)   |  [Scattered Spider](https://attack.mitre.org/groups/G1015)   |

[Scattered Spider](https://attack.mitre.org/groups/G1015) has deployed a malicious kernel driver through exploitation of CVE-2015-2291 in the Intel Ethernet diagnostics driver for Windows (iqvw64.sys).[[38]](https://www.crowdstrike.com/blog/scattered-spider-attempts-to-avoid-detection-with-bring-your-own-vulnerable-driver-tactic/)

|  [C0045](https://attack.mitre.org/campaigns/C0045)   |  [ShadowRay](https://attack.mitre.org/campaigns/C0045)   |

During [ShadowRay](https://attack.mitre.org/campaigns/C0045), threat actors downloaded a privilege escalation payload to gain root access.[[39]](https://www.oligo.security/blog/shadowray-attack-ai-workloads-actively-exploited-in-the-wild)

|  [S0623](https://attack.mitre.org/software/S0623)   |  [Siloscape](https://attack.mitre.org/software/S0623)   |

[Siloscape](https://attack.mitre.org/software/S0623) has leveraged a vulnerability in Windows containers to perform an [Escape to Host](https://attack.mitre.org/techniques/T1611).[[40]](https://unit42.paloaltonetworks.com/siloscape/)

|  [S0603](https://attack.mitre.org/software/S0603)   |  [Stuxnet](https://attack.mitre.org/software/S0603)   |

[Stuxnet](https://attack.mitre.org/software/S0603) used MS10-073 and an undisclosed Task Scheduler vulnerability to escalate privileges on local Windows machines.[[41]](https://docs.broadcom.com/doc/security-response-w32-stuxnet-dossier-11-en)

|  [G1058](https://attack.mitre.org/groups/G1058)   |  [The Gentlemen](https://attack.mitre.org/groups/G1058)   |

[The Gentlemen](https://attack.mitre.org/groups/G1058) has assisted affiliates with gaining privilege escalation through the use of BYOVD.[[42]](https://unit42.paloaltonetworks.com/the-gentlemen-ransomware/)

|  [G0027](https://attack.mitre.org/groups/G0027)   |  [Threat Group-3390](https://attack.mitre.org/groups/G0027)   |

[Threat Group-3390](https://attack.mitre.org/groups/G0027) has used CVE-2014-6324 and CVE-2017-0213 to escalate privileges.[[43]](https://www.secureworks.com/research/bronze-union)[[44]](https://web.archive.org/web/20210104144857/https://shared-public-reports.s3-eu-west-1.amazonaws.com/APT27+turns+to+ransomware.pdf)

|  [G0131](https://attack.mitre.org/groups/G0131)   |  [Tonto Team](https://attack.mitre.org/groups/G0131)   |

[Tonto Team](https://attack.mitre.org/groups/G0131) has exploited CVE-2019-0803 and MS16-032 to escalate privileges.[[45]](https://vb2020.vblocalhost.com/uploads/VB2020-06.pdf)

|  [G0010](https://attack.mitre.org/groups/G0010)   |  [Turla](https://attack.mitre.org/groups/G0010)   |

[Turla](https://attack.mitre.org/groups/G0010) has exploited vulnerabilities in the VBoxDrv.sys driver to obtain kernel mode privileges.[[2]](https://unit42.paloaltonetworks.com/acidbox-rare-malware/)

|  [G1048](https://attack.mitre.org/groups/G1048)   |  [UNC3886](https://attack.mitre.org/groups/G1048)   |

[UNC3886](https://attack.mitre.org/groups/G1048) has exploited zero-day vulnerability CVE-2023-20867 to enable execution of privileged commands across Windows, Linux, and PhotonOS (vCenter) guest VMs.[[46]](https://cloud.google.com/blog/topics/threat-intelligence/vmware-esxi-zero-day-bypass/)

|  [G1017](https://attack.mitre.org/groups/G1017)   |  [Volt Typhoon](https://attack.mitre.org/groups/G1017)   |

[Volt Typhoon](https://attack.mitre.org/groups/G1017) has gained initial access by exploiting privilege escalation vulnerabilities in the operating system or network services.[[47]](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)

|  [G0107](https://attack.mitre.org/groups/G0107)   |  [Whitefly](https://attack.mitre.org/groups/G0107)   |

[Whitefly](https://attack.mitre.org/groups/G0107) has used an open-source tool to exploit a known Windows privilege escalation vulnerability (CVE-2016-0051) on unpatched computers.[[48]](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/whitefly-espionage-singapore)

|  [S0176](https://attack.mitre.org/software/S0176)   |  [Wingbird](https://attack.mitre.org/software/S0176)   |

[Wingbird](https://attack.mitre.org/software/S0176) exploits CVE-2016-4117 to allow an executable to gain escalated privileges.[[49]](http://download.microsoft.com/download/E/B/0/EB0F50CC-989C-4B66-B7F6-68CD3DC90DE3/Microsoft_Security_Intelligence_Report_Volume_21_English.pdf)

|  [S0658](https://attack.mitre.org/software/S0658)   |  [XCSSET](https://attack.mitre.org/software/S0658)   |

[XCSSET](https://attack.mitre.org/software/S0658) has used a zero-day exploit in the ssh launchdaemon to elevate privileges and bypass SIP.[[50]](https://documents.trendmicro.com/assets/pdf/XCSSET_Technical_Brief.pdf)

|  [S1151](https://attack.mitre.org/software/S1151)   |  [ZeroCleare](https://attack.mitre.org/software/S1151)   |

[ZeroCleare](https://attack.mitre.org/software/S1151) has used a vulnerable signed VBoxDrv driver to bypass Microsoft Driver Signature Enforcement (DSE) protections and subsequently load the unsigned [RawDisk](https://attack.mitre.org/software/S0364) driver.[[51]](https://web.archive.org/web/20200211083637/https://securityintelligence.com/posts/new-destructive-wiper-zerocleare-targets-energy-sector-in-the-middle-east/)

|  [G0128](https://attack.mitre.org/groups/G0128)   |  [ZIRCONIUM](https://attack.mitre.org/groups/G0128)   |

[ZIRCONIUM](https://attack.mitre.org/groups/G0128) has exploited CVE-2017-0005 for local privilege escalation.[[52]](https://research.checkpoint.com/2021/the-story-of-jian/)

|  [S0672](https://attack.mitre.org/software/S0672)   |  [Zox](https://attack.mitre.org/software/S0672)   |

[Zox](https://attack.mitre.org/software/S0672) has the ability to leverage local and remote exploits to escalate privileges.[[53]](https://web.archive.org/web/20230115144216/http://www.novetta.com/wp-content/uploads/2014/11/Executive_Summary-Final_1.pdf)

## Mitigations

| ID  | Mitigation  | Description

|  [M1048](https://attack.mitre.org/mitigations/M1048)   |  [Application Isolation and Sandboxing](https://attack.mitre.org/mitigations/M1048)   |

Make it difficult for adversaries to advance their operation through exploitation of undiscovered or unpatched vulnerabilities by using sandboxing. Other types of virtualization and application microsegmentation may also mitigate the impact of some types of exploitation. Risks of additional exploits and weaknesses in these systems may still exist. [[54]](https://arstechnica.com/information-technology/2017/03/hack-that-escapes-vm-by-exploiting-edge-browser-fetches-105000-at-pwn2own/)

|  [M1038](https://attack.mitre.org/mitigations/M1038)   |  [Execution Prevention](https://attack.mitre.org/mitigations/M1038)   |

Consider blocking the execution of known vulnerable drivers that adversaries may exploit to execute code in kernel mode. Validate driver block rules in audit mode to ensure stability prior to production deployment.[[55]](https://docs.microsoft.com/en-us/windows/security/threat-protection/windows-defender-application-control/microsoft-recommended-driver-block-rules)

|  [M1050](https://attack.mitre.org/mitigations/M1050)   |  [Exploit Protection](https://attack.mitre.org/mitigations/M1050)   |

Security applications that look for behavior used during exploitation such as Windows Defender Exploit Guard (WDEG) and the Enhanced Mitigation Experience Toolkit (EMET) can be used to mitigate some exploitation behavior. [[56]](https://blogs.technet.microsoft.com/srd/2017/08/09/moving-beyond-emet-ii-windows-defender-exploit-guard/) Control flow integrity checking is another way to potentially identify and stop a software exploit from occurring. [[57]](https://en.wikipedia.org/wiki/Control-flow_integrity) Many of these protections depend on the architecture and target application binary for compatibility and may not work for software components targeted for privilege escalation.

|  [M1019](https://attack.mitre.org/mitigations/M1019)   |  [Threat Intelligence Program](https://attack.mitre.org/mitigations/M1019)   |

Develop a robust cyber threat intelligence capability to determine what types and levels of threat may use software exploits and 0-days against a particular organization.

|  [M1051](https://attack.mitre.org/mitigations/M1051)   |  [Update Software](https://attack.mitre.org/mitigations/M1051)   |

Update software regularly by employing patch management for internal enterprise endpoints and servers.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0514](https://attack.mitre.org/detectionstrategies/DET0514)   |  [Detection Strategy for Exploitation for Privilege Escalation](https://attack.mitre.org/detectionstrategies/DET0514)   |  [AN1419](https://attack.mitre.org/detectionstrategies/DET0514#AN1419)   |

Detects exploitation attempts targeting vulnerable kernel drivers or OS components, often followed by unusual process or token behavior.

|  [AN1420](https://attack.mitre.org/detectionstrategies/DET0514#AN1420)   |

Detects escalation via vulnerable setuid binaries or kernel modules, often chained with unusual access to /proc/kallsyms or /dev/kmem.

|  [AN1421](https://attack.mitre.org/detectionstrategies/DET0514#AN1421)   |

Detects use of vulnerable kernel extensions or entitlements abused via setuid or AppleScript injection chains.

|  [AN1422](https://attack.mitre.org/detectionstrategies/DET0514#AN1422)   |

Detects container breakout behavior via exploitation (e.g., DirtyPipe, CVE-2022-0847), followed by host OS interaction or escalated capability assignment.

## References

-    [Hromcova, Z. and Cherpanov, A. (2020, June). INVISIMOLE: THE HIDDEN PART OF THE STORY. Retrieved July 16, 2020.](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_InvisiMole.pdf)
-    [Reichel, D. and Idrizovic, E. (2020, June 17). AcidBox: Rare Malware Repurposing Turla Group Exploit Targeted Russian Organizations. Retrieved March 16, 2021.](https://unit42.paloaltonetworks.com/acidbox-rare-malware/)
-    [Bitdefender. (2015, December). APT28 Under the Scope. Retrieved February 23, 2017.](https://download.bitdefender.com/resources/media/materials/white-papers/en/Bitdefender_In-depth_analysis_of_APT28%E2%80%93The_Political_Cyber-Espionage.pdf)
-    [Anthe, C. et al. (2015, October 19). Microsoft Security Intelligence Report Volume 19. Retrieved December 23, 2015.](http://download.microsoft.com/download/4/4/C/44CDEF0E-7924-4787-A56A-16261691ACE3/Microsoft_Security_Intelligence_Report_Volume_19_English.pdf)
-    [Kaspersky Lab's Global Research & Analysis Team. (2018, February 20). A Slice of 2017 Sofacy Activity. Retrieved November 27, 2018.](https://securelist.com/a-slice-of-2017-sofacy-activity/83930/)
-    [Koessel, Sean. Adair, Steven. Lancaster, Tom. (2024, November 22). The Nearest Neighbor Attack: How A Russian APT Weaponized Nearby Wi-Fi Networks for Covert Access. Retrieved February 25, 2025.](https://www.volexity.com/blog/2024/11/22/the-nearest-neighbor-attack-how-a-russian-apt-weaponized-nearby-wi-fi-networks-for-covert-access/)
-    [ESET. (2022, February). THREAT REPORT T3 2021. Retrieved February 10, 2022.](https://www.welivesecurity.com/wp-content/uploads/2022/02/eset_threat_report_t32021.pdf)
-    [Carr, N.. (2017, May 14). Cyber Espionage is Alive and Well: APT32 and the Threat to Global Corporations. Retrieved June 18, 2017.](https://www.fireeye.com/blog/threat-research/2017/05/cyber-espionage-apt32.html)
-    [Ackerman, G., et al. (2018, December 21). OVERRULED: Containing a Potentially Destructive Adversary. Retrieved January 17, 2019.](https://www.fireeye.com/blog/threat-research/2018/12/overruled-containing-a-potentially-destructive-adversary.html)
-    [JinQuan, MaDongZe, TuXiaoYi, and LiHao. (2021, February 10). Windows kernel zero-day exploit (CVE-2021-1732) is used by BITTER APT in targeted attack. Retrieved June 1, 2022.](https://ti.dbappsecurity.com.cn/blog/articles/2021/02/10/windows-kernel-zero-day-exploit-is-used-by-bitter-apt-in-targeted-attack/)
-    [Microsoft. (2018, February 9). Windows Win32k Elevation of Privilege Vulnerability CVE-2021-1732. Retrieved June 1, 2022.](https://msrc.microsoft.com/update-guide/vulnerability/CVE-2021-1732)
-    [James Nutland, Craig Jackson, Terryn Valikodath, & Brennan Evans. (2024, August 28). BlackByte blends tried-and-true tradecraft with newly disclosed vulnerabilities to support ongoing attacks. Retrieved December 16, 2024.](https://blog.talosintelligence.com/blackbyte-blends-tried-and-true-tradecraft-with-newly-disclosed-vulnerabilities-to-support-ongoing-attacks/)
-    [Microsoft Incident Response. (2023, July 6). The five-day job: A BlackByte ransomware intrusion case study. Retrieved December 16, 2024.](https://www.microsoft.com/en-us/security/blog/2023/07/06/the-five-day-job-a-blackbyte-ransomware-intrusion-case-study/)
-    [Matrosov, A., Rodionov, E., Volkov, D., Harley, D. (2012, March 2). Win32/Carberp When You’re in a Black Hole, Stop Digging. Retrieved July 15, 2020.](https://www.eset.com/fileadmin/eset/US/resources/docs/white-papers/white-papers-win-32-carberp.pdf)
-    [Giuliani, M., Allievi, A. (2011, February 28). Carberp - a modular information stealing trojan. Retrieved September 12, 2024.](https://web.archive.org/web/20231227000328/http://pxnow.prevx.com/content/blog/carberp-a_modular_information_stealing_trojan.pdf)
-    [Matveeva, V. (2017, August 15). Secrets of Cobalt. Retrieved October 10, 2018.](https://www.group-ib.com/blog/cobalt)
-    [Cobalt Strike. (2017, December 8). Tactics, Techniques, and Procedures. Retrieved November 17, 2024.](https://web.archive.org/web/20210924171429/https://www.cobaltstrike.com/downloads/reports/tacticstechniquesandprocedures.pdf)
-    [Strategic Cyber LLC. (2020, November 5). Cobalt Strike: Advanced Threat Tactics for Penetration Testers. Retrieved April 13, 2021.](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)
-    [F-Secure Labs. (2015, September 17). The Dukes: 7 years of Russian cyberespionage. Retrieved December 10, 2015.](https://www.f-secure.com/documents/996508/1030745/dukes_whitepaper.pdf)
-    [Jan Holman, Tomas Zvara. (2024, October 23). Embargo ransomware: Rock’n’Rust. Retrieved October 19, 2025.](https://www.welivesecurity.com/en/eset-research/embargo-ransomware-rocknrust/)
-    [Schroeder, W., Warner, J., Nelson, M. (n.d.). Github PowerShellEmpire. Retrieved April 28, 2016.](https://github.com/PowerShellEmpire/Empire)
-    [FireEye Threat Intelligence. (2016, April). Follow the Money: Dissecting the Operations of the Cyber Crime Group FIN6. Retrieved November 17, 2024.](https://web.archive.org/web/20190807112824/https://www2.fireeye.com/rs/848-DID-242/images/rpt-fin6.pdf)
-    [Kizhakkinan, D., et al. (2016, May 11). Threat Actor Leverages Windows Zero-day Exploit in Payment Card Data Attacks. Retrieved February 12, 2018.](https://www.fireeye.com/blog/threat-research/2016/05/windows-zero-day-payment-cards.html)
-    [Elovitz, S. & Ahl, I. (2016, August 18). Know Your Enemy: New Financially-Motivated & Spear-Phishing Group. Retrieved February 26, 2018.](https://www2.fireeye.com/WBNR-Know-Your-Enemy-UNC622-Spear-Phishing.html)
-    [Microsoft Threat Intelligence . (2025, March 5). Silk Typhoon targeting IT supply chain. Retrieved March 20, 2025.](https://www.microsoft.com/en-us/security/blog/2025/03/05/silk-typhoon-targeting-it-supply-chain/)
-    [Chen, J. et al. (2021, February 3). Hildegard: New TeamTNT Cryptojacking Malware Targeting Kubernetes. Retrieved April 5, 2021.](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)
-    [ESET. (2016, October). En Route with Sednit - Part 1: Approaching the Target. Retrieved November 8, 2016.](http://www.welivesecurity.com/wp-content/uploads/2016/10/eset-sednit-part1.pdf)
-    [ESET Research. (2015, July 10). Sednit APT Group Meets Hacking Team. Retrieved March 1, 2017.](http://www.welivesecurity.com/2015/07/10/sednit-apt-group-meets-hacking-team/)
-    [MSTIC, DART, M365 Defender. (2022, March 24). DEV-0537 Criminal Actor Targeting Organizations for Data Exfiltration and Destruction. Retrieved May 17, 2022.](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)

-    [CISA et al. (2024, July 8). People’s Republic of China (PRC) Ministry of State Security APT40 Tradecraft in Action. Retrieved February 3, 2025.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa24-190a)
-    [Faou, M. (2023, August 10). MoustachedBouncer: Espionage against foreign diplomats in Belarus. Retrieved September 25, 2023.](https://www.welivesecurity.com/en/eset-research/moustachedbouncer-espionage-against-foreign-diplomats-in-belarus/)
-    [Fahmy, M. et al. (2024, October 11). Earth Simnavaz (aka APT34) Levies Advanced Cyberattacks Against Middle East. Retrieved November 27, 2024.](https://www.trendmicro.com/en_us/research/24/j/earth-simnavaz-cyberattacks.html)
-    [Lunghi, D. and Lu, K. (2021, April 9). Iron Tiger APT Updates Toolkit With Evolved SysUpdate Malware. Retrieved November 12, 2021.](https://www.trendmicro.com/en_us/research/21/d/iron-tiger-apt-updates-toolkit-with-evolved-sysupdate-malware-va.html)
-    [Windows Defender Advanced Threat Hunting Team. (2016, April 29). PLATINUM: Targeted attacks in South and Southeast Asia. Retrieved February 15, 2018.](https://download.microsoft.com/download/2/2/5/225BFE3E-E1DE-4F5B-A77B-71200928D209/Platinum%20feature%20article%20-%20Targeted%20attacks%20in%20South%20and%20Southeast%20Asia%20April%202016.pdf)
-    [Nettitude. (2018, July 23). Python Server for PoshC2. Retrieved April 23, 2019.](https://github.com/nettitude/PoshC2_Python)
-    [Group IB. (2020, September). LOCK LIKE A PRO. Retrieved November 17, 2024.](https://web.archive.org/web/20220119114433/https://groupib.pathfactory.com/ransomware-reports/prolock_wp)
-    [Kaspersky Lab's Global Research & Analysis Team. (2016, August 9). The ProjectSauron APT. Technical Analysis. Retrieved August 17, 2016.](https://securelist.com/files/2016/07/The-ProjectSauron-APT_Technical_Analysis_KL.pdf)
-    [CrowdStrike. (2023, January 10). SCATTERED SPIDER Exploits Windows Security Deficiencies with Bring-Your-Own-Vulnerable-Driver Tactic in Attempt to Bypass Endpoint Security. Retrieved July 5, 2023.](https://www.crowdstrike.com/blog/scattered-spider-attempts-to-avoid-detection-with-bring-your-own-vulnerable-driver-tactic/)
-    [Lumelsly, A. et al. (2024, March 26). ShadowRay: First Known Attack Campaign Targeting AI Workloads Actively Exploited In The Wild. Retrieved December 2, 2024.](https://www.oligo.security/blog/shadowray-attack-ai-workloads-actively-exploited-in-the-wild)
-    [Prizmant, D. (2021, June 7). Siloscape: First Known Malware Targeting Windows Containers to Compromise Cloud Environments. Retrieved June 9, 2021.](https://unit42.paloaltonetworks.com/siloscape/)
-    [Nicolas Falliere, Liam O Murchu, Eric Chien 2011, February W32.Stuxnet Dossier (Version 1.4) Retrieved November 17, 2024.](https://docs.broadcom.com/doc/security-response-w32-stuxnet-dossier-11-en)
-    [Brady, M. (2026, July 10). No Manners Here: The Ruthless Rise of The Gentlemen Ransomware. Retrieved September 18, 2026.](https://unit42.paloaltonetworks.com/the-gentlemen-ransomware/)
-    [Counter Threat Unit Research Team. (2017, June 27). BRONZE UNION Cyberespionage Persists Despite Disclosures. Retrieved July 13, 2017.](https://www.secureworks.com/research/bronze-union)
-    [Global Threat Center, Intelligence Team. (2020, December). APT27 Turns to Ransomware. Retrieved November 12, 2021.](https://web.archive.org/web/20210104144857/https://shared-public-reports.s3-eu-west-1.amazonaws.com/APT27+turns+to+ransomware.pdf)
-    [Daniel Lughi, Jaromir Horejsi. (2020, October 2). Tonto Team - Exploring the TTPs of an advanced threat actor operating a large infrastructure. Retrieved October 17, 2021.](https://vb2020.vblocalhost.com/uploads/VB2020-06.pdf)
-    [Alexander Marvi, Brad Slaybaugh, Ron Craft, and Rufus Brown. (2023, June 13). VMware ESXi Zero-Day Used by Chinese Espionage Actor to Perform Privileged Guest Operations on Compromised Hypervisors. Retrieved March 26, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/vmware-esxi-zero-day-bypass/)
-    [CISA et al.. (2024, February 7). PRC State-Sponsored Actors Compromise and Maintain Persistent Access to U.S. Critical Infrastructure. Retrieved May 15, 2024.](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)
-    [Symantec. (2019, March 6). Whitefly: Espionage Group has Singapore in Its Sights. Retrieved May 26, 2020.](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/whitefly-espionage-singapore)
-    [Anthe, C. et al. (2016, December 14). Microsoft Security Intelligence Report Volume 21. Retrieved November 27, 2017.](http://download.microsoft.com/download/E/B/0/EB0F50CC-989C-4B66-B7F6-68CD3DC90DE3/Microsoft_Security_Intelligence_Report_Volume_21_English.pdf)
-    [Mac Threat Response, Mobile Research Team. (2020, August 13). The XCSSET Malware: Inserts Malicious Code Into Xcode Projects, Performs UXSS Backdoor Planting in Safari, and Leverages Two Zero-day Exploits. Retrieved October 5, 2021.](https://documents.trendmicro.com/assets/pdf/XCSSET_Technical_Brief.pdf)
-    [Kessem, L. (2019, December 4). New Destructive Wiper ZeroCleare Targets Energy Sector in the Middle East. Retrieved September 4, 2024.](https://web.archive.org/web/20200211083637/https://securityintelligence.com/posts/new-destructive-wiper-zerocleare-targets-energy-sector-in-the-middle-east/)
-    [Itkin, E. and Cohen, I. (2021, February 22). The Story of Jian – How APT31 Stole and Used an Unknown Equation Group 0-Day. Retrieved March 24, 2021.](https://research.checkpoint.com/2021/the-story-of-jian/)
-    [Novetta. (n.d.). Operation SMN: Axiom Threat Actor Group Report. Retrieved November 12, 2014.](https://web.archive.org/web/20230115144216/http://www.novetta.com/wp-content/uploads/2014/11/Executive_Summary-Final_1.pdf)
-    [Goodin, D. (2017, March 17). Virtual machine escape fetches $105,000 at Pwn2Own hacking contest - updated. Retrieved March 12, 2018.](https://arstechnica.com/information-technology/2017/03/hack-that-escapes-vm-by-exploiting-edge-browser-fetches-105000-at-pwn2own/)
-    [Microsoft. (2020, October 15). Microsoft recommended driver block rules. Retrieved March 16, 2021.](https://docs.microsoft.com/en-us/windows/security/threat-protection/windows-defender-application-control/microsoft-recommended-driver-block-rules)
-    [Nunez, N. (2017, August 9). Moving Beyond EMET II – Windows Defender Exploit Guard. Retrieved March 12, 2018.](https://blogs.technet.microsoft.com/srd/2017/08/09/moving-beyond-emet-ii-windows-defender-exploit-guard/)
-    [Wikipedia. (2018, January 11). Control-flow integrity. Retrieved March 12, 2018.](https://en.wikipedia.org/wiki/Control-flow_integrity)

×
