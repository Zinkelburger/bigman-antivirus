---
url: "https://attack.mitre.org/techniques/T1036/007/"
title: "Masquerading: Double File Extension, Sub-technique T1036.007 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:26:29Z"
via: "live"
sha256: "e43231d6d73cb1a3dace44be6ffc73763fb32952883cc607b04a4a3acc766c29"
wayback: "https://web.archive.org/web/20261023090817/https://attack.mitre.org/techniques/T1036/007/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Masquerading: Double File Extension, Sub-technique T1036.007 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1036/007/>
> Snapshot: <https://web.archive.org/web/20261023090817/https://attack.mitre.org/techniques/T1036/007/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Masquerading](https://attack.mitre.org/techniques/T1036)
- Double File Extension

#  Masquerading: Double File Extension

##### Other sub-techniques of Masquerading (12)

| ID  | Name

|  [T1036.001](https://attack.mitre.org/techniques/T1036/001/)   |  [Invalid Code Signature](https://attack.mitre.org/techniques/T1036/001/)

|  [T1036.002](https://attack.mitre.org/techniques/T1036/002/)   |  [Right-to-Left Override](https://attack.mitre.org/techniques/T1036/002/)

|  [T1036.003](https://attack.mitre.org/techniques/T1036/003/)   |  [Rename Legitimate Utilities](https://attack.mitre.org/techniques/T1036/003/)

|  [T1036.004](https://attack.mitre.org/techniques/T1036/004/)   |  [Masquerade Task or Service](https://attack.mitre.org/techniques/T1036/004/)

|  [T1036.005](https://attack.mitre.org/techniques/T1036/005/)   |  [Match Legitimate Resource Name or Location](https://attack.mitre.org/techniques/T1036/005/)

|  [T1036.006](https://attack.mitre.org/techniques/T1036/006/)   |  [Space after Filename](https://attack.mitre.org/techniques/T1036/006/)

|  T1036.007   |  Double File Extension

|  [T1036.008](https://attack.mitre.org/techniques/T1036/008/)   |  [Masquerade File Type](https://attack.mitre.org/techniques/T1036/008/)

|  [T1036.009](https://attack.mitre.org/techniques/T1036/009/)   |  [Break Process Trees](https://attack.mitre.org/techniques/T1036/009/)

|  [T1036.010](https://attack.mitre.org/techniques/T1036/010/)   |  [Masquerade Account Name](https://attack.mitre.org/techniques/T1036/010/)

|  [T1036.011](https://attack.mitre.org/techniques/T1036/011/)   |  [Overwrite Process Arguments](https://attack.mitre.org/techniques/T1036/011/)

|  [T1036.012](https://attack.mitre.org/techniques/T1036/012/)   |  [Browser Fingerprint](https://attack.mitre.org/techniques/T1036/012/)

Adversaries may abuse a double extension in the filename as a means of masquerading the true file type. A file name may include a secondary file type extension that may cause only the first extension to be displayed (ex: File.txt.exe may render in some views as just File.txt). However, the second extension is the true file type that determines how the file is opened and executed. The real file extension may be hidden by the operating system in the file browser (ex: explorer.exe), as well as in any software configured using or similar to the system’s policies.[[1]](https://www.pcmag.com/encyclopedia/term/double-extension)[[2]](https://socprime.com/blog/rule-of-the-week-possible-malicious-file-double-extension/)

Adversaries may abuse double extensions to attempt to conceal dangerous file types of payloads. A very common usage involves tricking a user into opening what they think is a benign file type but is actually executable code. Such files often pose as email attachments and allow an adversary to gain [Initial Access](https://attack.mitre.org/tactics/TA0001) into a user’s system via [Spearphishing Attachment](https://attack.mitre.org/techniques/T1566/001) then [User Execution](https://attack.mitre.org/techniques/T1204). For example, an executable file attachment named Evil.txt.exe may display as Evil.txt to a user. The user may then view it as a benign text file and open it, inadvertently executing the hidden malware.[[2]](https://socprime.com/blog/rule-of-the-week-possible-malicious-file-double-extension/)

Common file types, such as text files (.txt, .doc, etc.) and image files (.jpg, .gif, etc.) are typically used as the first extension to appear benign. Executable extensions commonly regarded as dangerous, such as .exe, .lnk, .hta, and .scr, often appear as the second extension and true file type.

ID: T1036.007

Sub-technique of:  [T1036](https://attack.mitre.org/techniques/T1036)

ⓘ

Tactic: [Stealth](https://attack.mitre.org/tactics/TA0005)

ⓘ

Platforms: Windows

Version: 2.0

Created: 04 August 2021

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1036/007/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1036/007/)

## Procedure Examples

| ID  | Name  | Description

|  [S0534](https://attack.mitre.org/software/S0534)   |  [Bazar](https://attack.mitre.org/software/S0534)   |

The [Bazar](https://attack.mitre.org/software/S0534) loader has used dual-extension executable files such as PreviewReport.DOC.exe.[[3]](https://www.cybereason.com/blog/a-bazar-of-tricks-following-team9s-development-cycles)

|  [S1111](https://attack.mitre.org/software/S1111)   |  [DarkGate](https://attack.mitre.org/software/S1111)   |

[DarkGate](https://attack.mitre.org/software/S1111) masquerades malicious LNK files as PDF objects using the double extension .pdf.lnk.[[4]](https://www.trellix.com/blogs/research/the-continued-evolution-of-the-darkgate-malware-as-a-service/)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has used an additional filename extension to hide the true file type. [Kimsuky](https://attack.mitre.org/groups/G0094) has also masqueraded malicious LNK files as PDF objects using the double extension .pdf.lnk.[[5]](https://www.securonix.com/blog/analyzing-deepdrive-north-korean-threat-actors-observed-exploiting-trusted-platforms-for-targeted-attacks/)

|  [S1015](https://attack.mitre.org/software/S1015)   |  [Milan](https://attack.mitre.org/software/S1015)   |

[Milan](https://attack.mitre.org/software/S1015) has used an executable named companycatalog.exe.config to appear benign.[[6]](https://www.clearskysec.com/siamesekitten/)

|  [G0129](https://attack.mitre.org/groups/G0129)   |  [Mustang Panda](https://attack.mitre.org/groups/G0129)   |

[Mustang Panda](https://attack.mitre.org/groups/G0129) has used an additional filename extension to hide the true file type.[[7]](https://www.crowdstrike.com/blog/meet-crowdstrikes-adversary-of-the-month-for-june-mustang-panda/)[[8]](https://www.anomali.com/blog/china-based-apt-mustang-panda-targets-minority-groups-public-and-private-sector-organizations)

## Mitigations

| ID  | Mitigation  | Description

|  [M1028](https://attack.mitre.org/mitigations/M1028)   |  [Operating System Configuration](https://attack.mitre.org/mitigations/M1028)   |

Disable the default to "hide file extensions for known file types" in Windows OS.[[9]](https://www.seqrite.com/blog/how-to-avoid-dual-attack-and-vulnerable-files-with-double-extension/)[[10]](https://www.howtogeek.com/205086/beginner-how-to-make-windows-show-file-extensions/)

|  [M1017](https://attack.mitre.org/mitigations/M1017)   |  [User Training](https://attack.mitre.org/mitigations/M1017)   |

Train users to look for double extensions in filenames, and in general use training as a way to bring awareness to common phishing and spearphishing techniques and how to raise suspicion for potentially malicious events.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0366](https://attack.mitre.org/detectionstrategies/DET0366)   |  [Detection Strategy for Double File Extension Masquerading](https://attack.mitre.org/detectionstrategies/DET0366)   |  [AN1033](https://attack.mitre.org/detectionstrategies/DET0366#AN1033)   |

Detects adversary behavior where a file with a benign-looking first extension (e.g., .txt, .jpg) ends with a dangerous second extension (e.g., .exe, .scr), and is subsequently executed. The behavior chain includes file creation with misleading naming and user or system-initiated process execution from the disguised file.

## References

-    [PCMag. (n.d.). Encyclopedia: double extension. Retrieved August 4, 2021.](https://www.pcmag.com/encyclopedia/term/double-extension)
-    [Eugene Tkachenko. (2020, May 1). Rule of the Week: Possible Malicious File Double Extension. Retrieved July 27, 2021.](https://socprime.com/blog/rule-of-the-week-possible-malicious-file-double-extension/)
-    [Cybereason Nocturnus. (2020, July 16). A BAZAR OF TRICKS: FOLLOWING TEAM9’S DEVELOPMENT CYCLES. Retrieved November 18, 2020.](https://www.cybereason.com/blog/a-bazar-of-tricks-following-team9s-development-cycles)
-    [Ernesto Fernández Provecho, Pham Duy Phuc, Ciana Driscoll & Vinoo Thomas. (2023, November 21). The Continued Evolution of the DarkGate Malware-as-a-Service. Retrieved February 9, 2024.](https://www.trellix.com/blogs/research/the-continued-evolution-of-the-darkgate-malware-as-a-service/)
-    [Den Iuzvyk, Tim Peck. (2025, February 13). Analyzing DEEP#DRIVE: North Korean Threat Actors Observed Exploiting Trusted Platforms for Targeted Attacks. Retrieved August 19, 2025.](https://www.securonix.com/blog/analyzing-deepdrive-north-korean-threat-actors-observed-exploiting-trusted-platforms-for-targeted-attacks/)

-    [ClearSky Cyber Security . (2021, August). New Iranian Espionage Campaign By “Siamesekitten” - Lyceum. Retrieved June 6, 2022.](https://www.clearskysec.com/siamesekitten/)
-    [Meyers, A. (2018, June 15). Meet CrowdStrike’s Adversary of the Month for June: MUSTANG PANDA. Retrieved April 12, 2021.](https://www.crowdstrike.com/blog/meet-crowdstrikes-adversary-of-the-month-for-june-mustang-panda/)
-    [Anomali Threat Research. (2019, October 7). China-Based APT Mustang Panda Targets Minority Groups, Public and Private Sector Organizations. Retrieved April 12, 2021.](https://www.anomali.com/blog/china-based-apt-mustang-panda-targets-minority-groups-public-and-private-sector-organizations)
-    [Seqrite. (n.d.). How to avoid dual attack and vulnerable files with double extension?. Retrieved July 27, 2021.](https://www.seqrite.com/blog/how-to-avoid-dual-attack-and-vulnerable-files-with-double-extension/)
-    [Chris Hoffman. (2017, March 8). How to Make Windows Show File Extensions. Retrieved August 4, 2021.](https://www.howtogeek.com/205086/beginner-how-to-make-windows-show-file-extensions/)

×
