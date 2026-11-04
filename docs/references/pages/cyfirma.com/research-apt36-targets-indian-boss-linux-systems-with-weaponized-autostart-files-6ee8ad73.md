---
url: "https://www.cyfirma.com/research/apt36-targets-indian-boss-linux-systems-with-weaponized-autostart-files/"
title: "APT36: Targets Indian BOSS Linux Systems with Weaponized AutoStart Files - CYFIRMA"
site: "CYFIRMA"
captured: "2026-11-02T18:52:38Z"
via: "live"
sha256: "5e02086b70abfc39083a28c7e7bd4fcdc6b80e75d69a6196ad9ddc80d15b4aa6"
wayback: "https://web.archive.org/web/20261014092905/https://www.cyfirma.com/research/apt36-targets-indian-boss-linux-systems-with-weaponized-autostart-files/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# APT36: Targets Indian BOSS Linux Systems with Weaponized AutoStart Files - CYFIRMA

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.cyfirma.com/research/apt36-targets-indian-boss-linux-systems-with-weaponized-autostart-files/>
> Snapshot: <https://web.archive.org/web/20261014092905/https://www.cyfirma.com/research/apt36-targets-indian-boss-linux-systems-with-weaponized-autostart-files/>

https://www.cyfirma.com

-  [Global (EN)](https://www.cyfirma.com/research/apt36-targets-indian-boss-linux-systems-with-weaponized-autostart-files/#)

# APT36: Targets Indian BOSS Linux Systems with Weaponized AutoStart Files

Published On : 2025-08-22

Share :

- https://www.facebook.com/sharer.php?u=https://www.cyfirma.com/research/apt36-targets-indian-boss-linux-systems-with-weaponized-autostart-files/
- https://twitter.com/intent/tweet?url='https://www.cyfirma.com/research/apt36-targets-indian-boss-linux-systems-with-weaponized-autostart-files/'&text=APT36-Targets-Indian-BOSS-Linux-Systems-with-Weaponized-AutoStart-Files
- http://www.linkedin.com/shareArticle?mini=true&url=https://www.cyfirma.com/research/apt36-targets-indian-boss-linux-systems-with-weaponized-autostart-files/&title=APT36-Targets-Indian-BOSS-Linux-Systems-with-Weaponized-AutoStart-Files&source=https://www.cyfirma.com

## Executive Summary

CYFIRMA has identified an ongoing cyber-espionage campaign orchestrated by APT36 (Transparent Tribe), a Pakistan-based threat actor with a sustained focus on Indian Government entities. This operation reflects the group’s increasing sophistication and flexibility; leveraging tailored malware, aimed at Boss operating systems.

Initial access is achieved through spear phishing emails. Linux BOSS environments are targeted via weaponized .desktop shortcut files that, once opened, download and execute malicious payloads.

APT36’s capability to customize its delivery mechanisms according to the victim’s operating environment thereby increases its chances of success while maintaining persistent access to critical government infrastructure and evading traditional security controls.

## Introduction

Nation-state threat actors continue to evolve their tactics to exploit vulnerabilities across diverse operating environments, posing a significant challenge to government defenses. APT36 (also known as Transparent Tribe), a Pakistan-based adversarial group, remains actively engaged in cyber espionage operations against Indian Government institutions. Recent CYFIRMA observations reveal a marked increase in the group’s sophistication, as it deploys customized malware tailored specifically for the Linux BOSS operating system. By adopting a dual-platform delivery strategy, APT36 seeks to maximize its reach and ensure sustained, covert access to critical government infrastructure while circumventing conventional security mechanisms.

| Target Technologies  | Windows and BOSS Operating System

| Threat Type  | Phishing Campaign

| Written In  | GO Language

| File Types  | ELF (Executable and Linkable format)

| Key Malware Identifiers  | Meeting_Ltr_ID1543ops.pdf.desktop and Meeting_Ltr_ID1543ops.pdf-.elf

| Observed First  | 2025-08-01

| Impact  | Data Exfiltration

| MD5 Hashes  | Meeting_Ltr_ID1543ops.pdf.desktop
“10b7139952e3daae8f9d7ee407696ccf”
Meeting_Ltr_ID1543ops.pdf-.elf “5bfeeae3cc9386513dc7c301c61e67a7”

## Malware Campaign Attack Flow

In this campaign, the group distributes an Archive file named “Meeting_Notice_Ltr_ID1543ops.pdf_.zip”.

When opened, it contains a .desktop file named “Meeting_Ltr_ID1543ops.pdf.desktop” (Linux shortcut file).

The .desktop file has been flagged as highly malicious on VirusTotal, with multiple antivirus engines consistently identifying it as associated with malware or unauthorized execution behavior.

### “.desktop” shortcut file working

The “.desktop” file shown is crafted to masquerade as an ordinary PDF shortcut but contains a chain of commands embedded in its Exec= line that are executed automatically and sequentially as soon as the file is launched. This enables the attacker to perform covert actions while keeping the victim unaware.

The Exec field launches a Bash shell using bash -c, allowing several commands to be chained in a single line. The sequence begins by generating a temporary filename inside /tmp, incorporating a timestamp to avoid detection and prevent naming conflicts. Using curl -s, the script silently retrieves a hex-encoded file (Mt_dated_29.txt) from an attacker-controlled server.

Instead of saving it directly, the download is piped through xxd -r -p, which converts the hexadecimal text into raw binary form. This binary is written directly to the temporary file path.

If this download and conversion succeed, the script uses chmod +x to make the temporary file executable and immediately runs it in the background. At this stage, the malicious payload (whose content is unknown to the user) is now active in the system.

To divert suspicion, the script then launches Firefox in the background and opens a Google Drive link containing a harmless PDF. This acts as a visual decoy, leading the victim to believe that they have simply viewed a document, all while the malicious binary continues its execution unseen.

Type=Application tells the system to treat the entry as an executable program rather than a document, ensuring that its payload is run rather than simply opened.

Terminal=false prevents any terminal window from appearing on-screen, letting all actions execute silently.

Icon=application-pdf assigns a familiar PDF icon to the shortcut, enhancing its disguise as a legitimate document. Together, these attributes support a classic social-engineering rule.

Enabling X-GNOME-Autostart-enabled=true ensures that the file automatically runs every time the user logs in, granting the attacker persistence on the system. Combined with stealthy execution and a convincing disguise, this multistage mechanism allows the attacker to deliver and run malware, maintain access, and potentially perform further malicious operations without triggering immediate detection by the user.

Domain Analysis
Domain “securestore[.]cv” has been identified as a part of the malicious infrastructure actively leveraged in cyber espionage campaigns attributed to the APT36 (Transparent Tribe) threat group, which is assessed to be operating out of Pakistan.

The domain securestore[.]cv has been observed communicating with the file Meeting_Ltr_ID1543ops.pdf.desktop.

The domain securestore[.]cv, registered via Namecheap on 27 July 2025 and currently only 23 days old, warrants scrutiny as potential malicious infrastructure, due to its recent creation and associated registrar attributes.

These campaigns resemble previously observed Linux-based attack patterns, in which .desktop files are weaponized to mimic legitimate government or defense-related documents. Upon execution, the files establish connections with newly registered domains linked to APT36 infrastructure, such as “securestore[.]cv” to download and execute additional malicious payloads.

### Technical Analysis of ELF Malicious Files

#### Static Analysis

ELF Header: Starts with the magic bytes 7F 45 4C 46 (representing “.ELF” in ASCII), indicating it is an Executable and Linkable Format file.

The file was identified as an ELF 64-bit LSB executable for x86-64 (version 1, SYSV), statically linked. It exhibits an unusually large section header offset of 9,626,443, which may indicate corruption or malicious tampering.

This ELF executable file has an entry point at 0x477c00. It includes 6 program headers (56 bytes each) starting at byte 64, and 25 section headers (64 bytes each) starting at byte 400. The section name table is at index 22. The ELF header is 64 bytes, with no special flags set.

Three LOAD segments include readable-executable code, read-only data, and a read-write segment with a larger memory than file size, suggesting unpacked or runtime data typical in malware. The GNU_STACK segment’s read-write permission also poses potential security concerns.

The section headers are suspicious due to missing section names, which are uncommon in normal binaries and often used to hide malicious code. Several sections have unusual flags, zero addresses, or irregular sizes and alignments. Large NOBITS sections with no file offset may be used to load hidden data at runtime, indicating possible obfuscation or packing typical of malware.

#### Command and Control

The presence of “http://modgovindia[.]space:4000” as both a string and a hex value in the malware file indicates a hardcoded command-and-control (C2) server address, embedded to ensure persistent communication for receiving commands, delivering payloads, or exfiltrating data.

### Dynamic Analysis

Upon execution, the malware enables the system-update.service for the current user, ensuring it is automatically started during future user-level systemd sessions.

The malware establishes persistence by quietly inserting a cron job that executes the hidden payload ‘.config/systemd/systemd-update’ on every system reboot. This ensures the malicious code automatically restarts after system reboots or process termination.

Subsequently, the malware established command-and-control (C2) communication with the domain “modgovindia[.]space”, which resolves to the IP address 45[.]141[.]58[.]199, over port 4000.

The observed activity involves low-level socket operations used by malware to perform DNS queries for command-and-control (C2) communication. It creates a non-blocking UDP socket, enables broadcast, and connects to the local DNS resolver at 127.0.0.53:53. Using epoll for efficient event handling, the malware sends DNS queries, such as for the suspicious domain modgovindia[.]space, and attempts to read responses non-blockingly. Futex system calls indicate synchronization in a multithreaded context. This behavior points to DNS-based reconnaissance or communication with a malicious server as part of a C2 infrastructure.

The malware attempts to transmit harvested data to the attacker’s infrastructure, indicating the presence of an active bidirectional command-and-control (C2) channel that facilitates both the retrieval of instructions and the exfiltration of information.

Domain Analysis
The domain “modgovindia[.]space” has been flagged as malicious by multiple detection engines and is assessed to be associated with APT36 activity.

The domain modgovindia[.]space, registered on 8 July 2025 via Namecheap and hosted on IP 45[.]141[.]58[.]199 by Seychelles-based IP Connect Inc., is assessed as likely malicious due to its recent registration and infrastructure commonly used in malware or phishing campaigns.

### EXTERNAL THREAT LANDSCAPE MANAGEMENT

APT36, also known as Transparent Tribe or Mythic Leopard, is a Pakistan-linked cyber-espionage group that has been active for over a decade, with campaigns becoming increasingly sophisticated since 2016. The group is widely assessed to operate with state sponsorship, focusing primarily on Indian government, military, and diplomatic institutions, while also expanding into education, defense contractors, and critical infrastructure sectors. Their operations typically rely on spear-phishing emails, malicious documents, spoofed login portals, and custom malware implants to gain and maintain access. APT36’s persistent targeting reflects its strategic objective of collecting sensitive information to support Pakistan’s military and diplomatic priorities.

While Indian government entities remain the primary focus, APT36 has extended operations to adjacent sectors (education, research, and civil society), as well as opportunistic targeting in other geographies. This broad victimology increases the attack surface and introduces risk to partners, suppliers, and diplomatic missions abroad.

The adoption of .desktop payloads targeting Linux BOSS reflects a tactical shift toward exploiting indigenous technologies. Combined with traditional Windows-based malware and mobile implants, this shows the group’s intent to diversify access vectors and ensure persistence even in hardened environments.

## MITRE ATT&CK Framework

| MITRE MAPPING

| Tactic  | ID  | Technique Name

| Execution  | T1064  | Scripting

| T1053  | Scheduled Task/Job

| Defense Evasion  | T1064  | Scripting

| T1222  | File and Directory Permissions Modification

| T1014  | Rootkit

| T1564.001  | Hide Artifacts: Hidden Files and Directories

| Persistence  | T1053  | Scheduled Task/Job

| T1037  | Boot or Logon Initialization Scripts

| T1546.004  | Event Triggered Execution: Unix Shell Configuration Modification

| T1543.002  | Create or Modify System Process: Systemd Service

| Privilege Escalation  | T1053  | Scheduled Task/Job

| T1546.004  | Event Triggered Execution: Unix Shell Configuration Modification

| T1037  | Boot or Logon Initialization Scripts

| T1543.002  | Create or Modify System Process: Systemd Service

| Credential Access  | T1003.001  | OS Credential Dumping: LSASS Memory

| Discovery  | T1082  | System Information Discovery

| T1083  | File and Directory Discovery

| Collection  | T1005  | Data from Local System

| T1025  | Data from Removable Media

| Command and Control  | T1571  | Non-Standard Port

| T1001.001  | Data Obfuscation: Junk Data

| T1105  | Ingress Tool Transfer

| T1095  | Non-Application Layer Protocol

| T1071  | Application Layer Protocol

| T1090  | Proxy

| Exfiltration  | T1048  | Exfiltration Over Alternative Protocol

| T1011.001  | Exfiltration Over Other Network Medium: Exfiltration Over Bluetooth

| Impact  | T1498  | Network Denial of Service

## Conclusion

The analysis indicates a coordinated cyber-espionage campaign attributed to APT36, leveraging weaponized .desktop files to target BOSS Linux environments within Indian Government entities. These files are designed to masquerade as legitimate documents while covertly communicating with recently registered malicious domains such as “securestore[.]cv” and “modgovindia[.]space” to download and execute secondary payloads. Both domains exhibit characteristics typical of disposable threat infrastructure and have been flagged as malicious by multiple threat-intelligence sources. The group’s ability to rapidly deploy tailored payloads and infrastructure demonstrates increasing sophistication and underscores the need for heightened vigilance, particularly around spear-phishing vectors and Linux-based attack surfaces.

## Recommendations and Mitigation

Recommendations and Mitigation Strategies
To mitigate the ongoing APT36 campaign targeting Indian Government entities through weaponized .desktop files and associated malicious domains, the following measures are formally recommended:

1. Email Security Enhancements

- Deploy advanced email security solutions capable of detecting spear-phishing emails that contain .desktop, .sh, .elf, and compressed archive attachments.
- Disable automatic execution of email attachments and apply sandbox-based detonation of suspicious files prior to delivery.
- Enable URL filtering to block connections to newly registered or malicious domains (e.g., securestore[.]cv, modgovindia[.]space).

2. User Awareness and Training

- Conduct regular cybersecurity awareness training, with emphasis on phishing indicators and high-risk Linux-specific file types.
- Promote caution when handling unsolicited attachments, particularly .desktop files disguised as official documents, or unknown Google Drive links.

3. Host and Operating System Hardening

- Restrict execution of files from world-writable directories, such as /tmp, using filesystem mount options (noexec, nodev).
- Disable execution of .desktop files from untrusted sources and enforce strict application authorization controls.
- Apply least privilege principles in BOSS Linux deployments, with stringent controls over the use of utilities, such as curl, xxd, chmod, and nohup.

4. Endpoint and Network Monitoring

- Implement Linux-capable Endpoint Detection and Response (EDR) solutions to monitor for execution of unknown ELF payloads, lateral movement, and C2 beaconing activity.
- Monitor DNS and outbound traffic for attempted connections to suspicious or recently registered APT36-associated domains.
- Use network segmentation to contain any potential compromise and limit lateral movement within critical infrastructure.

5. Threat Intelligence Integration

- Integrate CYFIRMA-provided Indicators of Compromise (IOCs) and YARA signatures into SIEM and IDS/IPS platforms to facilitate early detection.
- Conduct initiative-taking threat hunting based on known APT36 tactics, techniques, and procedures (TTPs) mapped to MITRE ATT&CK (e.g., T1204, T1059, T1105).

6. Patch and Vulnerability Management

- Ensure BOSS Linux systems and commonly leveraged software (e.g., curl, LibreOffice) are patched promptly to mitigate exploitation of known vulnerabilities.

7. Behavior-Based Controls

- Deploy detection rules to identify suspicious command sequences (e.g., curl | xxd | chmod | nohup) executed through .desktop files or shell scripts.
- Block execution of binaries downloaded via scripts unless verified and digitally signed by the organization.

Maintaining ongoing situational awareness through CYFIRMA’s threat intelligence services, coupled with regular updates of IOCs and a defense-in-depth approach, will significantly enhance the ability to detect, prevent, and respond to APT36’s increasingly sophisticated intrusion activity.

## Indicators of Compromise

Kindly refer to the IOCs section, applying relevant security controls.

| S. No  | Indicator  | Remarks

| 1  | 508a2bcaa4c511f7db2d4491bb76effaa7231d66110c28632b95c77be40ea6b1   | Block

| 2  | 8f8da8861c368e74b9b5c1c59e64ef00690c5eff4a95e1b4fcf386973895bef1   | Block

| 3  | e689afee5f7bdbd1613bd9a3915ef2a185a05c72aaae4df3dee988fa7109cb0b   | Block

| 4  | securestore[.]cv   | Block

| 5  | modgovindia[.]space   | Block

| 6  | 45[.]141[.]58[.]199   | Monitor

## YARA Rules

rule APT36_LinuxDesktop_Campaign
{
meta:
description = “Detects artifacts related to APT36 .desktop file campaign”
author = “CYFIRMA Research”
date = “2025-08-18”
version = “1.0”

strings:
// Malware file hashes (sha256)
$hash1 = “508a2bcaa4c511f7db2d4491bb76effaa7231d66110c28632b95c77be40ea6b1”
$hash2 = “8f8da8861c368e74b9b5c1c59e64ef00690c5eff4a95e1b4fcf386973895bef1”
$hash3 = “e689afee5f7bdbd1613bd9a3915ef2a185a05c72aaae4df3dee988fa7109cb0b”

// Malicious domains and IP
$domain1 = “securestore.cv”
$domain2 = “modgovindia.space”
$ip1 = “45.141.58.199”

// Suspicious command patterns used in payload
$cmd1 = /curl.*xxd.*chmod/s nocase
$cmd2 = /\.desktop/ nocase

condition:
any of ($hash*) or
any of ($domain*,$ip*) or
2 of ($cmd*)
}

[Back to Listing](https://www.cyfirma.com/cyber-research/)
