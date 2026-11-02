---
url: "https://cwe.mitre.org/data/definitions/59.html"
title: "CWE - CWE-59: Improper Link Resolution Before File Access (5.0)"
captured: "2026-11-02T18:26:28Z"
via: "live"
sha256: "4fae5c5e7444b5e2f58b126e966640a41d1a055bd60a40b16a5e4ad9b5a7a038"
wayback: "https://web.archive.org/web/20261102194204/https://cwe.mitre.org/data/definitions/59.html"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# CWE - CWE-59: Improper Link Resolution Before File Access (5.0)

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://cwe.mitre.org/data/definitions/59.html>
> Snapshot: <https://web.archive.org/web/20261102194204/https://cwe.mitre.org/data/definitions/59.html>

[# Common Weakness Enumeration

Celebrating 20 years of community-developed cybersecurity weakness knowledge!](https://cwe.mitre.org/index.html)

https://cwe.mitre.org/about/new_to_cwe.html

https://cwe.mitre.org/topHW/index.html

https://cwe.mitre.org/top25/

|  [Home](https://cwe.mitre.org/) > [CWE List](https://cwe.mitre.org/data/index.html) > CWE-59: Improper Link Resolution Before File Access (5.0)   |

- [Home](https://cwe.mitre.org/index.html)
-

[Who We Are](https://cwe.mitre.org/about/index.html) [User Stories](https://cwe.mitre.org/about/user_stories.html) [History](https://cwe.mitre.org/about/history.html) [Documents](https://cwe.mitre.org/about/documents.html) [Videos](https://cwe.mitre.org/about/cwe_videos.html)

-

[Basics](https://cwe.mitre.org/about/new_to_cwe.html)

[Root Cause Mapping ►](https://cwe.mitre.org/documents/cwe_usage/guidance.html)

[Guidance](https://cwe.mitre.org/documents/cwe_usage/guidance.html) [Quick Tips](https://cwe.mitre.org/documents/cwe_usage/quick_tips.html) [Examples](https://cwe.mitre.org/documents/cwe_usage/mapping_examples.html)

[How to Contribute Weakness Content](https://cwe.mitre.org/community/submissions/overview.html) [FAQs](https://cwe.mitre.org/about/faq.html) [Glossary](https://cwe.mitre.org/documents/glossary/index.html)

-

[Top-N Lists ►](https://cwe.mitre.org/scoring/index.html#top_n_lists)

[Top 25 Software](https://cwe.mitre.org/top25/) [Top Hardware](https://cwe.mitre.org/topHW/index.html) [Top 10 KEV Weaknesses](https://cwe.mitre.org/top25/archive/2025/2025_kev_list.html)

[CWE List ►](https://cwe.mitre.org/data/index.html)

[Current Version](https://cwe.mitre.org/data/index.html) [Reports](https://cwe.mitre.org/data/reports.html) [Visualizations](https://cwe.mitre.org/data/pdfs.html) [Releases Archive](https://cwe.mitre.org/data/archive.html)

[Downloads](https://cwe.mitre.org/data/downloads.html) [REST API](https://github.com/CWE-CAPEC/REST-API-wg/blob/main/Quick%20Start.md)

-

[News ►](https://cwe.mitre.org/news/)

[Current News](https://cwe.mitre.org/news/archives/news2026.html) [Blog](https://medium.com/@CWE_CAPEC) [Podcast](https://cwe.mitre.org/news/podcast.html) [News Archive](https://cwe.mitre.org/news/archives/index.html)

[CWE Board](https://cwe.mitre.org/community/board.html) [Working Groups & Special Interest Groups](https://cwe.mitre.org/community/working_groups.html) [Email Lists](https://cwe.mitre.org/community/registration.html)

-

[Search CWE List](https://cwe.mitre.org/) [Search Website](https://cwe.mitre.org/find/index.html)

|       |    |

|

|

CWE Glossary Definition

|

|

## CWE-59: Improper Link Resolution Before File Access

|

Weakness ID: 59

[Vulnerability Mapping](https://cwe.mitre.org/data/definitions/59.html#Vulnerability_Mapping_Notes_59): ALLOWED This CWE ID may be used to map to real-world vulnerabilities
Abstraction: Base Base - a weakness that is still mostly independent of a resource or technology, but with sufficient details to provide specific methods for detection and prevention. Base level weaknesses typically describe issues in terms of 2 or 3 of the following dimensions: behavior, property, technology, language, and resource.

View customized information:

Shows the description and diagram, platforms, mitigations, consequences, relationships, memberships, and content history.    For users who are interested in more notional aspects of a weakness. Example: educators, technical writers, and project/program managers.    For users who are concerned with the practical application and details about the nature of a weakness and how to prevent it from happening. Example: tool developers, security researchers, pen-testers, incident response analysts.    For users who are mapping an issue to CWE/CAPEC IDs, i.e., finding the most appropriate CWE for a specific issue (e.g., a CVE record). Example: tool developers, security researchers.    For users who wish to see all available information for the CWE/CAPEC entry.    For users who want to customize what details are displayed.

×

## Edit Custom Filter

More information is available — Please edit the custom filter or select a different filter.

Description

The product attempts to access a file based on the filename, but it does not properly prevent that filename from identifying a link or shortcut that resolves to an unintended resource.

Alternate Terms

|  insecure temporary file   |

Some people use the phrase "insecure temporary file" when referring to a link following weakness, but other weaknesses can produce insecure temporary files without any symlink involvement at all.

|  Zip Slip   |

Roles: Attack-Focused

"Zip slip" is an attack that uses file archives (e.g., ZIP, tar, rar, etc.) that contain filenames with path traversal sequences that cause the files to be written outside of the directory under which the archive is expected to be extracted [[REF-1282](https://cwe.mitre.org/data/definitions/59.html#REF-1282_59)]. It is most commonly used for relative path traversal ([CWE-23](https://cwe.mitre.org/data/definitions/23.html)) and link following ([CWE-59](https://cwe.mitre.org/data/definitions/59.html)).

|  Link Following   |

Roles: Common-Name

Common Consequences

This table specifies different individual consequences associated with the weakness. The Scope identifies the application security area that is violated, while the Impact describes the negative technical impact that arises if an adversary succeeds in exploiting this weakness. The Likelihood provides information about how likely the specific consequence is expected to be seen relative to the other consequences in the list. For example, there may be high likelihood that a weakness will be exploited to achieve a certain impact, but a low likelihood that it will be exploited to achieve a different impact.

| Impact  | Details

|

Read Files or Directories; Modify Files or Directories; Bypass Protection Mechanism

|  Scope: Confidentiality, Integrity, Access Control

An attacker may be able to traverse the file system to unintended locations and read or overwrite the contents of unexpected files. If the files are used for a security mechanism then an attacker may be able to bypass the mechanism.

|

Execute Unauthorized Code or Commands

|  Scope: Other

Windows simple shortcuts, sometimes referred to as soft links, can be exploited remotely since a ".LNK" file can be uploaded like a normal file. This can enable remote execution.

Potential Mitigations

| Phase(s)  | Mitigation

|

Architecture and Design

|

Strategy: Separation of Privilege

Follow the principle of least privilege when assigning access rights to entities in a software system.

Denying access to a file can prevent an attacker from replacing that file with a link to a sensitive file. Ensure good compartmentalization in the system to provide protected areas that can be trusted.

Relationships

This table shows the weaknesses and high level categories that are related to this weakness. These relationships are defined as ChildOf, ParentOf, MemberOf and give insight to similar items that may exist at higher and lower levels of abstraction. In addition, relationships such as PeerOf, Overlaps, and CanAlsoBe are defined to show similar weaknesses that the user may want to explore.

|   Relevant to the view "Complete Weakness Hierarchy" (View-1000)

| Nature  | Type  | ID  | Name

| ChildOf  |   Class - a weakness that is described in a very abstract fashion, typically independent of any specific language or technology. More specific than a Pillar Weakness, but more general than a Base Weakness. Class level weaknesses typically describe issues in terms of 1 or 2 of the following dimensions: behavior, property, and resource.    |  [706](https://cwe.mitre.org/data/definitions/706.html)   |  Use of Incorrectly-Resolved Name or Reference

| ParentOf  |   Composite - a Compound Element that consists of two or more distinct weaknesses, in which all weaknesses must be present at the same time in order for a potential vulnerability to arise. Removing any of the weaknesses eliminates or sharply reduces the risk. One weakness, X, can be "broken down" into component weaknesses Y and Z. There can be cases in which one weakness might not be essential to a composite, but changes the nature of the composite when it becomes a vulnerability.    |  [61](https://cwe.mitre.org/data/definitions/61.html)   |  UNIX Symbolic Link (Symlink) Following

| ParentOf  |   Variant - a weakness that is linked to a certain type of product, typically involving a specific language or technology. More specific than a Base weakness. Variant level weaknesses typically describe issues in terms of 3 to 5 of the following dimensions: behavior, property, technology, language, and resource.    |  [62](https://cwe.mitre.org/data/definitions/62.html)   |  UNIX Hard Link

| ParentOf  |   Variant - a weakness that is linked to a certain type of product, typically involving a specific language or technology. More specific than a Base weakness. Variant level weaknesses typically describe issues in terms of 3 to 5 of the following dimensions: behavior, property, technology, language, and resource.    |  [64](https://cwe.mitre.org/data/definitions/64.html)   |  Windows Shortcut Following (.LNK)

| ParentOf  |   Variant - a weakness that is linked to a certain type of product, typically involving a specific language or technology. More specific than a Base weakness. Variant level weaknesses typically describe issues in terms of 3 to 5 of the following dimensions: behavior, property, technology, language, and resource.    |  [65](https://cwe.mitre.org/data/definitions/65.html)   |  Windows Hard Link

| ParentOf  |   Base - a weakness that is still mostly independent of a resource or technology, but with sufficient details to provide specific methods for detection and prevention. Base level weaknesses typically describe issues in terms of 2 or 3 of the following dimensions: behavior, property, technology, language, and resource.    |  [1386](https://cwe.mitre.org/data/definitions/1386.html)   |  Insecure Operation on Windows Junction / Mount Point

| CanFollow  |   Base - a weakness that is still mostly independent of a resource or technology, but with sufficient details to provide specific methods for detection and prevention. Base level weaknesses typically describe issues in terms of 2 or 3 of the following dimensions: behavior, property, technology, language, and resource.    |  [73](https://cwe.mitre.org/data/definitions/73.html)   |  External Control of File Name or Path

| CanFollow  |   Base - a weakness that is still mostly independent of a resource or technology, but with sufficient details to provide specific methods for detection and prevention. Base level weaknesses typically describe issues in terms of 2 or 3 of the following dimensions: behavior, property, technology, language, and resource.    |  [363](https://cwe.mitre.org/data/definitions/363.html)   |  Race Condition Enabling Link Following

|   Relevant to the view "Software Development" (View-699)

| Nature  | Type  | ID  | Name

| MemberOf  |   Grouping - a CWE entry that contains a set of other entries that share a common characteristic.    |  [1219](https://cwe.mitre.org/data/definitions/1219.html)   |  File Handling Issues

|   Relevant to the view "Weaknesses for Simplified Mapping of Published Vulnerabilities" (View-1003)

| Nature  | Type  | ID  | Name

| ChildOf  |   Class - a weakness that is described in a very abstract fashion, typically independent of any specific language or technology. More specific than a Pillar Weakness, but more general than a Base Weakness. Class level weaknesses typically describe issues in terms of 1 or 2 of the following dimensions: behavior, property, and resource.    |  [706](https://cwe.mitre.org/data/definitions/706.html)   |  Use of Incorrectly-Resolved Name or Reference

|   Relevant to the view "Architectural Concepts" (View-1008)

| Nature  | Type  | ID  | Name

| MemberOf  |   Grouping - a CWE entry that contains a set of other entries that share a common characteristic.    |  [1019](https://cwe.mitre.org/data/definitions/1019.html)   |  Validate Inputs

Background Details

| Soft links are a UNIX term that is synonymous with simple shortcuts on Windows-based platforms.

Modes Of Introduction

The different Modes of Introduction provide information about how and when this weakness may be introduced. The Phase identifies a point in the life cycle at which introduction may occur, while the Note provides a typical scenario related to introduction during the given phase.

| Phase(s)  | Note

|

Implementation

| REALIZATION: This weakness is caused during implementation of an architectural security tactic.

Applicable Platforms

This listing shows possible areas for which the given weakness could appear. These may be for specific named Languages, Operating Systems, Architectures, Paradigms, Technologies, or a class of such platforms. The platform is listed along with how frequently the given weakness appears for that instance.

| Type  | Applicable Platforms  | Notes

| Languages  |

Class: Not Language-Specific (Undetermined Prevalence)

|

| Operating Systems  |

Class: Windows (Sometimes Prevalent)

Class: Unix (Often Prevalent)

|

| Technologies  |

Class: Not Technology-Specific (Undetermined Prevalence)

|

Likelihood Of Exploit

Medium

Selected Observed Examples

Note: this is a curated list of examples for users to understand the variety of ways in which this weakness can be introduced. It is not a complete list of all CVEs that are related to this CWE entry.

| Reference  | Description

|

[CVE-1999-1386](https://www.cve.org/CVERecord?id=CVE-1999-1386)

|

Some versions of Perl follow symbolic links when running with the -e option, which allows local users to overwrite arbitrary files via a symlink attack.

|

[CVE-2000-1178](https://www.cve.org/CVERecord?id=CVE-2000-1178)

|

Text editor follows symbolic links when creating a rescue copy during an abnormal exit, which allows local users to overwrite the files of other users.

|

[CVE-2004-0217](https://www.cve.org/CVERecord?id=CVE-2004-0217)

|

Antivirus update allows local users to create or append to arbitrary files via a symlink attack on a logfile.

|

[CVE-2003-0517](https://www.cve.org/CVERecord?id=CVE-2003-0517)

|

Symlink attack allows local users to overwrite files.

|

[CVE-2004-0689](https://www.cve.org/CVERecord?id=CVE-2004-0689)

|

Window manager does not properly handle when certain symbolic links point to "stale" locations, which could allow local users to create or truncate arbitrary files.

|

[CVE-2005-1879](https://www.cve.org/CVERecord?id=CVE-2005-1879)

|

Second-order symlink vulnerabilities

|

[CVE-2005-1880](https://www.cve.org/CVERecord?id=CVE-2005-1880)

|

Second-order symlink vulnerabilities

|

[CVE-2005-1916](https://www.cve.org/CVERecord?id=CVE-2005-1916)

|

Symlink in Python program

|

[CVE-2000-0972](https://www.cve.org/CVERecord?id=CVE-2000-0972)

|

Setuid product allows file reading by replacing a file being edited with a symlink to the targeted file, leaking the result in error messages when parsing fails.

|

[CVE-2005-0824](https://www.cve.org/CVERecord?id=CVE-2005-0824)

|

Signal causes a dump that follows symlinks.

|

[CVE-2001-1494](https://www.cve.org/CVERecord?id=CVE-2001-1494)

|

Hard link attack, file overwrite; interesting because program checks against soft links

|

[CVE-2002-0793](https://www.cve.org/CVERecord?id=CVE-2002-0793)

|

Hard link and possibly symbolic link following vulnerabilities in embedded operating system allow local users to overwrite arbitrary files.

|

[CVE-2003-0578](https://www.cve.org/CVERecord?id=CVE-2003-0578)

|

Server creates hard links and unlinks files as root, which allows local users to gain privileges by deleting and overwriting arbitrary files.

|

[CVE-1999-0783](https://www.cve.org/CVERecord?id=CVE-1999-0783)

|

Operating system allows local users to conduct a denial of service by creating a hard link from a device special file to a file on an NFS file system.

|

[CVE-2004-1603](https://www.cve.org/CVERecord?id=CVE-2004-1603)

|

Web hosting manager follows hard links, which allows local users to read or modify arbitrary files.

|

[CVE-2004-1901](https://www.cve.org/CVERecord?id=CVE-2004-1901)

|

Package listing system allows local users to overwrite arbitrary files via a hard link attack on the lockfiles.

|

[CVE-2005-1111](https://www.cve.org/CVERecord?id=CVE-2005-1111)

|

Hard link race condition

|

[CVE-2000-0342](https://www.cve.org/CVERecord?id=CVE-2000-0342)

|

Mail client allows remote attackers to bypass the user warning for executable attachments such as .exe, .com, and .bat by using a .lnk file that refers to the attachment, aka "Stealth Attachment."

|

[CVE-2001-1042](https://www.cve.org/CVERecord?id=CVE-2001-1042)

|

FTP server allows remote attackers to read arbitrary files and directories by uploading a .lnk (link) file that points to the target file.

|

[CVE-2001-1043](https://www.cve.org/CVERecord?id=CVE-2001-1043)

|

FTP server allows remote attackers to read arbitrary files and directories by uploading a .lnk (link) file that points to the target file.

|

[CVE-2005-0587](https://www.cve.org/CVERecord?id=CVE-2005-0587)

|

Browser allows remote malicious web sites to overwrite arbitrary files by tricking the user into downloading a .LNK (link) file twice, which overwrites the file that was referenced in the first .LNK file.

|

[CVE-2001-1386](https://www.cve.org/CVERecord?id=CVE-2001-1386)

|

".LNK." - .LNK with trailing dot

|

[CVE-2003-1233](https://www.cve.org/CVERecord?id=CVE-2003-1233)

|

Rootkits can bypass file access restrictions to Windows kernel directories using NtCreateSymbolicLinkObject function to create symbolic link

|

[CVE-2002-0725](https://www.cve.org/CVERecord?id=CVE-2002-0725)

|

Chain: File system allows local attackers to hide file usage activities via a hard link ([CWE-65](https://cwe.mitre.org/data/definitions/65.html)) to the target file, which causes the link to be recorded in the audit trail instead of the target file ([CWE-221](https://cwe.mitre.org/data/definitions/221.html)).

|

[CVE-2003-0844](https://www.cve.org/CVERecord?id=CVE-2003-0844)

|

Web server plugin allows local users to overwrite arbitrary files via a symlink attack on predictable temporary filenames.

|

[CVE-2015-3629](https://www.cve.org/CVERecord?id=CVE-2015-3629)

|

A Libcontainer used in Docker Engine allows local users to escape containerization and write to an arbitrary file on the host system via a symlink attack in an image when respawning a container.

|

[CVE-2021-21272](https://www.cve.org/CVERecord?id=CVE-2021-21272)

|

"Zip Slip" vulnerability in Go-based Open Container Initiative (OCI) registries product allows writing arbitrary files outside intended directory via symbolic links or hard links in a gzipped tarball.

|

[CVE-2020-27833](https://www.cve.org/CVERecord?id=CVE-2020-27833)

|

"Zip Slip" vulnerability in container management product allows writing arbitrary files outside intended directory via a container image (.tar format) with filenames that are symbolic links that point to other files within the same tar file; however, the files being pointed to can also be symbolic links to destinations outside the intended directory, bypassing the initial check.

Weakness Ordinalities

| Ordinality  | Description

|

Resultant

|

(where the weakness is typically related to the presence of some other weaknesses)

Detection Methods

| Method  | Details

|

Automated Static Analysis - Binary or Bytecode

|

According to SOAR [[REF-1479](https://cwe.mitre.org/data/definitions/59.html#REF-1479_59)], the following detection techniques may be useful:

Cost effective for partial coverage:

- Bytecode Weakness Analysis - including disassembler + source code weakness analysis

Effectiveness: SOAR Partial

|

Manual Static Analysis - Binary or Bytecode

|

According to SOAR [[REF-1479](https://cwe.mitre.org/data/definitions/59.html#REF-1479_59)], the following detection techniques may be useful:

Cost effective for partial coverage:

- Binary / Bytecode disassembler - then use manual analysis for vulnerabilities & anomalies

Effectiveness: SOAR Partial

|

Dynamic Analysis with Automated Results Interpretation

|

According to SOAR [[REF-1479](https://cwe.mitre.org/data/definitions/59.html#REF-1479_59)], the following detection techniques may be useful:

Cost effective for partial coverage:

- Web Application Scanner
- Web Services Scanner
- Database Scanners

Effectiveness: SOAR Partial

|

Dynamic Analysis with Manual Results Interpretation

|

According to SOAR [[REF-1479](https://cwe.mitre.org/data/definitions/59.html#REF-1479_59)], the following detection techniques may be useful:

Cost effective for partial coverage:

- Fuzz Tester
- Framework-based Fuzzer

Effectiveness: SOAR Partial

|

Manual Static Analysis - Source Code

|

According to SOAR [[REF-1479](https://cwe.mitre.org/data/definitions/59.html#REF-1479_59)], the following detection techniques may be useful:

Highly cost effective:

- Focused Manual Spotcheck - Focused manual analysis of source
- Manual Source Code Review (not inspections)

Effectiveness: High

|

Automated Static Analysis - Source Code

|

According to SOAR [[REF-1479](https://cwe.mitre.org/data/definitions/59.html#REF-1479_59)], the following detection techniques may be useful:

Cost effective for partial coverage:

- Source code Weakness Analyzer
- Context-configured Source Code Weakness Analyzer

Effectiveness: SOAR Partial

|

Architecture or Design Review

|

According to SOAR [[REF-1479](https://cwe.mitre.org/data/definitions/59.html#REF-1479_59)], the following detection techniques may be useful:

Highly cost effective:

- Formal Methods / Correct-By-Construction

Cost effective for partial coverage:

- Inspection (IEEE 1028 standard) (can apply to requirements, design, source code, etc.)

Effectiveness: High

Functional Areas

- File Processing

Affected Resources

- File or Directory

Memberships

This MemberOf Relationships table shows additional CWE Categories and Views that reference this weakness as a member. This information is often useful in understanding where a weakness fits within the context of external information sources.

| Nature  | Type  | ID  | Name

| MemberOf  | View - a subset of CWE entries that provides a way of examining CWE content. The two main view structures are Slices (flat lists) and Graphs (containing relationships between entries).  |  [635](https://cwe.mitre.org/data/definitions/635.html)   |  Weaknesses Originally Used by NVD from 2008 to 2016

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [743](https://cwe.mitre.org/data/definitions/743.html)   |  CERT C Secure Coding Standard (2008) Chapter 10 - Input Output (FIO)

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [748](https://cwe.mitre.org/data/definitions/748.html)   |  CERT C Secure Coding Standard (2008) Appendix - POSIX (POS)

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [808](https://cwe.mitre.org/data/definitions/808.html)   |  2010 Top 25 - Weaknesses On the Cusp

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [877](https://cwe.mitre.org/data/definitions/877.html)   |  CERT C++ Secure Coding Section 09 - Input Output (FIO)

| MemberOf  | View - a subset of CWE entries that provides a way of examining CWE content. The two main view structures are Slices (flat lists) and Graphs (containing relationships between entries).  |  [884](https://cwe.mitre.org/data/definitions/884.html)   |  CWE Cross-section

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [980](https://cwe.mitre.org/data/definitions/980.html)   |  SFP Secondary Cluster: Link in Resource Name Resolution

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [1185](https://cwe.mitre.org/data/definitions/1185.html)   |  SEI CERT Perl Coding Standard - Guidelines 07. File Input and Output (FIO)

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [1345](https://cwe.mitre.org/data/definitions/1345.html)   |  OWASP Top Ten 2021 Category A01:2021 - Broken Access Control

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [1404](https://cwe.mitre.org/data/definitions/1404.html)   |  Comprehensive Categorization: File Handling

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [1436](https://cwe.mitre.org/data/definitions/1436.html)   |  OWASP Top Ten 2025 Category A01:2025 - Broken Access Control

Vulnerability Mapping Notes

|  Usage   |   ALLOWED

(this CWE ID may be used to map to real-world vulnerabilities)

| Reason  |  Acceptable-Use

|

Rationale

|  This CWE entry is at the Base level of abstraction, which is a preferred level of abstraction for mapping to the root causes of vulnerabilities.

|

Comments

|  Carefully read both the name and description to ensure that this mapping is an appropriate fit. Do not try to 'force' a mapping to a lower-level Base/Variant simply to comply with this preferred level of abstraction.

Notes

Theoretical

Link following vulnerabilities are Multi-factor Vulnerabilities (MFV). They are the combination of multiple elements: file or directory permissions, filename predictability, race conditions, and in some cases, a design limitation in which there is no mechanism for performing atomic file creation operations.

Some potential factors are race conditions, permissions, and predictability.

Taxonomy Mappings

| Mapped Taxonomy Name  | Node ID  | Fit  | Mapped Node Name

| PLOVER  |   |   | Link Following

| CERT C Secure Coding  | FIO02-C  |   | Canonicalize path names originating from untrusted sources

| CERT C Secure Coding  | POS01-C  |   | Check for the existence of links when dealing with files

| SEI CERT Perl Coding Standard  | FIO01-PL  | CWE More Specific  | Do not operate on files that can be modified by untrusted users

| Software Fault Patterns  | SFP18  |   | Link in resource name resolution

Related Attack Patterns

| CAPEC-ID  | Attack Pattern Name

| [CAPEC-132](http://capec.mitre.org/data/definitions/132.html)  | Symlink Attack

| [CAPEC-17](http://capec.mitre.org/data/definitions/17.html)  | Using Malicious Files

| [CAPEC-35](http://capec.mitre.org/data/definitions/35.html)  | Leverage Executable Code in Non-Executable Files

| [CAPEC-76](http://capec.mitre.org/data/definitions/76.html)  | Manipulating Web Input to File System Calls

References

| [REF-62]  |  Mark Dowd, John McDonald and Justin Schuh. "The Art of Software Security Assessment". Chapter 9, "Symbolic Link Attacks", Page 518. 1st Edition. Addison Wesley. 2006.

| [REF-1282]  |  Snyk. "Zip Slip Vulnerability". 2018-06-05.
<https://security.snyk.io/research/zip-slip-vulnerability>.

| [REF-1479]  |  Gregory Larsen, E. Kenneth Hong Fong, David A. Wheeler and Rama S. Moorthy. "State-of-the-Art Resources (SOAR) for Software Vulnerability Detection, Test, and Evaluation". 2014-07.
<https://www.ida.org/-/media/feature/publications/s/st/stateoftheart-resources-soar-for-software-vulnerability-detection-test-and-evaluation/p-5061.ashx>. (URL validated: 2025-09-05)

Content History

|  Submissions

| Submission Date  | Submitter  | Organization

|  2006-07-19
(CWE Draft 3, 2006-07-19)   | PLOVER  |

|

|  Modifications

| Modification Date  | Modifier  | Organization

|  2026-10-28
(CWE 5.0, 2026-10-28)   | CWE Content Team  | MITRE

|  updated Alternate_Terms, Name, Observed_Examples

|  2025-12-11
(CWE 4.19, 2025-12-11)   | CWE Content Team  | MITRE

|  updated Applicable_Platforms, Relationships

|  2025-09-09
(CWE 4.18, 2025-09-09)   | CWE Content Team  | MITRE

|  updated Detection_Factors, References

|  2023-06-29
(CWE 4.12, 2023-06-29)   | CWE Content Team  | MITRE

|  updated Mapping_Notes

|  2023-04-27
(CWE 4.11, 2023-04-27)   | CWE Content Team  | MITRE

|  updated Relationships

|  2023-01-31
(CWE 4.10, 2023-01-31)   | CWE Content Team  | MITRE

|  updated Description

|  2022-10-13
(CWE 4.9, 2022-10-13)   | CWE Content Team  | MITRE

|  updated Alternate_Terms, Background_Details, Observed_Examples, References, Relationship_Notes, Theoretical_Notes

|  2022-06-28
(CWE 4.8, 2022-06-28)   | CWE Content Team  | MITRE

|  updated Relationships

|  2022-04-28
(CWE 4.7, 2022-04-28)   | CWE Content Team  | MITRE

|  updated Research_Gaps

|  2021-10-28
(CWE 4.6, 2021-10-28)   | CWE Content Team  | MITRE

|  updated Relationships

|  2021-03-15
(CWE 4.4, 2021-03-15)   | CWE Content Team  | MITRE

|  updated Relationships

|  2020-02-24
(CWE 4.0, 2020-02-24)   | CWE Content Team  | MITRE

|  updated Relationships

|  2019-06-20
(CWE 3.3, 2019-06-20)   | CWE Content Team  | MITRE

|  updated Relationships

|  2019-01-03
(CWE 3.2, 2019-01-03)   | CWE Content Team  | MITRE

|  updated Taxonomy_Mappings

|  2017-11-08
(CWE 3.0, 2017-11-08)   | CWE Content Team  | MITRE

|  updated Affected_Resources, Applicable_Platforms, Causal_Nature, Common_Consequences, Functional_Areas, Likelihood_of_Exploit, Modes_of_Introduction, Relationships, Taxonomy_Mappings

|  2015-12-07
(CWE 2.9, 2015-12-07)   | CWE Content Team  | MITRE

|  updated Relationships

|  2014-07-30
(CWE 2.8, 2014-07-31)   | CWE Content Team  | MITRE

|  updated Detection_Factors, Relationships, Taxonomy_Mappings

|  2014-06-23
(CWE 2.7, 2014-06-23)   | CWE Content Team  | MITRE

|  updated Common_Consequences, Other_Notes

|  2012-10-30
(CWE 2.3, 2012-10-30)   | CWE Content Team  | MITRE

|  updated Potential_Mitigations

|  2012-05-11
(CWE 2.2, 2012-05-15)   | CWE Content Team  | MITRE

|  updated Common_Consequences, Observed_Examples, References, Relationships

|  2011-09-13
(CWE 2.1, 2011-09-13)   | CWE Content Team  | MITRE

|  updated Relationships, Taxonomy_Mappings

|  2011-06-01
(CWE 1.13, 2011-06-01)   | CWE Content Team  | MITRE

|  updated Common_Consequences

|  2010-04-05
(CWE 1.8.1, 2010-04-05)   | CWE Content Team  | MITRE

|  updated Related_Attack_Patterns

|  2010-02-16
(CWE 1.8, 2010-02-16)   | CWE Content Team  | MITRE

|  updated Potential_Mitigations, Relationships

|  2009-10-29
(CWE 1.6, 2009-10-29)   | CWE Content Team  | MITRE

|  updated Background_Details, Other_Notes

|  2009-05-27
(CWE 1.4, 2009-05-27)   | CWE Content Team  | MITRE

|  updated Description, Name

|  2009-01-12
(CWE 1.2, 2009-01-12)   | CWE Content Team  | MITRE

|  updated Relationships

|  2008-11-24
(CWE 1.1, 2008-11-25)   | CWE Content Team  | MITRE

|  updated Relationships, Taxonomy_Mappings

|  2008-09-08
(CWE 1.0, 2008-09-09)   | CWE Content Team  | MITRE

|  updated Alternate_Terms, Applicable_Platforms, Relationships, Other_Notes, Relationship_Notes, Taxonomy_Mappings, Weakness_Ordinalities

|  2008-07-01
(CWE 1.0, 2008-09-09)   | Eric Dalci  | Cigital

|  updated Time_of_Introduction

|  Previous Entry Names

| Change Date  | Previous Entry Name

| 2026-10-28   | Improper Link Resolution Before File Access ('Link Following')

| 2009-05-27   | Failure to Resolve Links Before File Access (aka 'Link Following')

| 2008-04-11   | Link Following

More information is available — Please edit the custom filter or select a different filter.

Page Last Updated:  Oct 28, 2026

|

|

http://www.mitre.org

|

[Site Map](https://cwe.mitre.org/sitemap.html) | [Terms of Use](https://cwe.mitre.org/about/termsofuse.html) | [Manage Cookies](https://cwe.mitre.org/data/definitions/59.html#) | [Cookie Notice](https://cwe.mitre.org/about/cookie_notice.html) | [Privacy Policy](https://cwe.mitre.org/about/privacy_policy.html) | Contact Us | https://x.com/CweCapec https://www.linkedin.com/showcase/cve-cwe-capec https://bsky.app/profile/cweprogram.bsky.social https://mastodon.social/@CWE_Program https://www.youtube.com/channel/UCpY9VIpRmFK4ebD6orssifA https://cwe.mitre.org/news/podcast.html https://medium.com/@CWE_CAPEC

Use of the Common Weakness Enumeration (CWE™) and the associated references from this website are subject to the [Terms of Use](https://cwe.mitre.org/about/termsofuse.html). CWE is sponsored by the [U.S. Department of Homeland Security](https://www.dhs.gov/) (DHS) [Cybersecurity and Infrastructure Security Agency](https://www.dhs.gov/cisa/cybersecurity-division) (CISA) and managed by the [Homeland Security Systems Engineering and Development Institute](https://www.dhs.gov/science-and-technology/hssedi) (HSSEDI) which is operated by [The MITRE Corporation](http://www.mitre.org/) (MITRE). Copyright © 2006–2026, The MITRE Corporation. CWE, CWSS, CWRAF, and the CWE logo are trademarks of The MITRE Corporation.

|

https://www.dhs.gov/science-and-technology/hssedi
