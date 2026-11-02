---
url: "https://securelist.com/dinodasrat-linux-implant/112284/"
title: "Analysis of DinodasRAT Linux implant"
site: "Securelist"
captured: "2026-11-02T18:43:03Z"
via: "live"
sha256: "6f499c673b84639f6642c88c7c4c96ef27d0c6efb14f1fdc595f0aa1e68d06f3"
wayback: "https://web.archive.org/web/20261022094914/https://securelist.com/dinodasrat-linux-implant/112284/"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# Analysis of DinodasRAT Linux implant

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://securelist.com/dinodasrat-linux-implant/112284/>
> Snapshot: <https://web.archive.org/web/20261022094914/https://securelist.com/dinodasrat-linux-implant/112284/>

[Malware descriptions](https://securelist.com/category/malware-descriptions/)

# DinodasRAT Linux implant targeting entities worldwide

[Malware descriptions](https://securelist.com/category/malware-descriptions/)

28 Mar 2024

minute read

-  [Anderson Leite](https://securelist.com/author/andersonleite/)
-  [Lisandro Ubiedo](https://securelist.com/author/lisandroubiedo/)

Table of Contents

- [Initial infection overview](https://securelist.com/dinodasrat-linux-implant/112284/#initial-infection-overview)
- [Victim ID generation and persistence](https://securelist.com/dinodasrat-linux-implant/112284/#victim-id-generation-and-persistence)
- [C2 Communication](https://securelist.com/dinodasrat-linux-implant/112284/#c2-communication)
- [Encryption](https://securelist.com/dinodasrat-linux-implant/112284/#encryption)
- [Infrastructure](https://securelist.com/dinodasrat-linux-implant/112284/#infrastructure)
- [Victims](https://securelist.com/dinodasrat-linux-implant/112284/#victims)
- [Conclusion](https://securelist.com/dinodasrat-linux-implant/112284/#conclusion)
- [Indicators of compromise](https://securelist.com/dinodasrat-linux-implant/112284/#indicators-of-compromise)

Authors

-  [Anderson Leite](https://securelist.com/author/andersonleite/)
-  [Lisandro Ubiedo](https://securelist.com/author/lisandroubiedo/)

DinodasRAT, also known as [XDealer](https://jsac.jpcert.or.jp/archive/2022/pdf/JSAC2022_7_leon-niwa-ishimaru_en.pdf), is a multi-platform backdoor written in C++ that offers a range of capabilities. This RAT allows the malicious actor to surveil and harvest sensitive data from a target’s computer. A Windows version of this RAT was used in attacks against government entities in Guyana, and documented by ESET researchers as [Operation Jacana](https://www.welivesecurity.com/en/eset-research/operation-jacana-spying-guyana-entity/).

In early October 2023, after the ESET publication, we discovered a new Linux version of DinodasRAT. Sample artifacts suggest that this version (V10 according to the attackers’ versioning system) may have started operating in 2022, although the first known Linux variant (V7), which has still not been publicly described, dates back to 2021. In this analysis, we’ll discuss technical details of one Linux implant used by the attackers.

## Initial infection overview

The DinodasRAT Linux implant primarily targets Red Hat-based distributions and Ubuntu Linux. When first executed, it creates a hidden file in the same directory as the executable, following the format “.[executable_name].mu”. This file is used as a sort of mutex in order to ensure the implant only runs one instance and only allows it to proceed if it is able to successfully create this file.

The backdoor maintains persistence and is launched as follows:

https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2024/03/27122803/DinodasRAT_Linux_implant_01.png

Backdoor main code

The backdoor establishes persistence and starts with the following steps:

- Direct execution without arguments;

- It first executes without any arguments, which makes it run in the background by calling the “daemon” function from Linux.

- Establishing persistence on the infected system by utilizing SystemV or SystemD startup scripts (detailed in the next section).
- Executing itself again with the parent process ID (PPID) as an argument;

- The newly created process (child) continues the backdoor infection while the parent process waits.
- This technique not only gives Dinodas the ability to verify that it has executed correctly, but also makes it harder to detect with debugging and monitoring tools.

## Victim ID generation and persistence

Before establishing contact with the C2 server, the backdoor gathers information about the infected machine and infection time to create a unique identifier for the victim’s machine. Notably, the attackers do not collect any user-specific data to generate this UID. The UID typically includes:

- Date of infection;
- MD5 hash of the dmidecode command output (a detailed report of the infected system’s hardware);
- Randomly generated number as ID;
- Backdoor version.

The unique identifier has the format: Linux_{DATE}_{HASH}_{RAND_NUM}_{VERSION}.

https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2024/03/27122858/DinodasRAT_Linux_implant_02.png

Machine unique identifier generation

Next, the implant stores all the local information about the victim’s ID, privilege level, and any other relevant details in a hidden file called “/etc/.netc.conf“. This profile file contains the current collected metadata of the backdoor. If the file does not exist, Dinodas will create it, adhering to the Section and Key:Value structure.

https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2024/03/27122926/DinodasRAT_Linux_implant_03.png

DinodasRAT profile configuration

It also ensures that any access to this file or to itself (when reading its own filepath) does not update the “access” time in the stat structure, which contains the access timestamp of a given file in the file system. It does this by using the “touch” command with the “-d” parameter to modify this metadata.

https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2024/03/27122955/DinodasRAT_Linux_implant_04.png

Replacing the original file access time code

https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2024/03/27123026/DinodasRAT_Linux_implant_05.png

Modified access time in the backdoor executable

The DinodasRAT Linux version takes advantage of the two versions of Linux service managers to establish persistence on an affected system: Systemd and SystemV. When the malware is launched, a function is called to determine the type of Linux distribution the victim is running. There are currently two flavors of distros that the implant targets based on its readings of “/proc/version” – RedHat and Ubuntu 16/18. However, the malware could infect any distro that supports either of the above versions of system service managers. Once the system is recognized, it installs a suitable init script that provides persistence for the RAT. This script is executed once the network setup is complete and launches the backdoor.

https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2024/03/27123057/DinodasRAT_Linux_implant_06.png

SystemD service registration

For RedHat, RedHat-based systems and Ubuntu, the service initiation scripts used for persistence check for the presence of the chkconfig binary. This is a way to indicate that the initialization is done with SysV instead of Systemd. If it doesn’t exist, the implant will open or create the script file “/etc/rc.d/rc.local” and append itself to the execution chain that runs the backdoor during system initialization. If it exists, the SysV route is implied and the malware creates the persistence scripts in “/etc/init.d”.

## C2 Communication

The Linux version of DinodasRAT communicates with the C2 in the same way as the Windows version. It communicates over TCP or UDP. The C2 domain is hard-coded into the binary:

https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2024/03/27123151/DinodasRAT_Linux_implant_07.png

C2 server and port are hard-coded into the implant

DinodasRAT has a timed interval for sending the information back to the C2, although it is not a fixed interval for all users or all connections. If the user executing the implant is root (EUID = 0), the implant doesn’t wait to send the information back to the C2. In the case of a non-superuser with the configuration set to checkroot, it will wait two minutes for a “short” wait (default) and 10 hours for a “long” wait. The “long” wait is triggered when there’s a remote connection to the infected server coming from one of the C2-configured IP addresses.

To communicate with the C2 server and send any information, the implant follows a network packet structure with many fields, but here are the relevant fields of the structure:

https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2024/03/27123223/DinodasRAT_Linux_implant_08.png

Simplified version of Dinodas network packet

Here’s a list of C2 commands that DinodasRAT recognizes:

| ID  | Function  | Command

| 0x02  | DirClass  | List the directory content.

| 0x03  | DelDir  | Delete directory.

| 0x05  | UpLoadFile  | Upload a file to the C2.

| 0x06  | StopDownLoadFile  | Stop file upload.

| 0x08  | DownLoadFile  | Download remote file to system.

| 0x09  | StopDownFile  | Stop file download.

| 0x0E  | DealChgIp  | Change C2 remote address.

| 0x0F  | CheckUserLogin  | Check logged-in users.

| 0x11  | EnumProcess  | Enumerate running processes.

| 0x12  | StopProcess  | Kill a running process.

| 0x13  | EnumService  | Use chkconfig and enumerate all available services.

| 0x14  | ControlService  | Control an available service. If 1 is passed as an argument, it will start a service, 0 will stop it, while 2 will stop and delete the service.

| 0x18  | DealExShell  | Execute shell command and send its output to C2.

| 0x19  | ExecuteFile  | Execute a specified file path in a separate thread.

| 0x1A  | DealProxy  | Proxy C2 communication through a remote proxy.

| 0x1B  | StartShell  | Drop a shell for the threat actor to interact with.

| 0x1C  | ReRestartShell  | Restart the previously mentioned shell.

| 0x1D  | StopShell  | Stop the execution of the current shell.

| 0x1E  | WriteShell  | Write commands into the current shell or create one if necessary.

| 0x27  | DealFile  | Download and set up a new version of the implant.

| 0x28  | DealLocalProxy  | Send “ok”.

| 0x2B  | ConnectCtl  | Control connection type.

| 0x2C  | ProxyCtl  | Control proxy type.

| 0x2D  | Trans_mode  | Set or get file transfer mode (TCP/UDP).

| 0x2E  | Uninstall  | Uninstall the implant and delete any artifacts from the system.

https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2024/03/27123304/DinodasRAT_Linux_implant_09.png

Command to uninstall itself from the infected system

## Encryption

The Linux version of DinodasRAT also shares encryption characteristics with the Windows version. For encryption and decryption of communication between the implant and the C2, as well as encryption of data, it uses Pidgin’s [libqq](https://code.google.com/archive/p/libqq-pidgin/source/default/source) qq_crypt library functions. This library uses the Tiny Encryption Algorithm ([TEA](https://en.wikipedia.org/wiki/Tiny_Encryption_Algorithm)) in [CBC](https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation#Cipher_block_chaining_(CBC)) mode to cipher and decipher the data, which makes it fairly easy to port between platforms. The Linux implant also shares two of the keys used in the Windows version:

For C2 encryption: A1 A1 18 AA 10 F0 FA 16 06 71 B3 08 AA AF 31 A1 For name encryption: A0 21 A1 FA 18 E0 C1 30 1F 9F C0 A1 A0 A6 6F B1

|

1

2

|

For C2 encryption: A1 A1 18 AA 10 F0 FA 16 06 71 B3 08 AA AF 31 A1

For name encryption: A0 21 A1 FA 18 E0 C1 30 1F 9F C0 A1 A0 A6 6F B1

## Infrastructure

| Domain  | IP  | First seen  | ASN  | Registrar

| update.centos-yum[.]com  | 199.231.211[.]19  | May 4, 2022  | 18978  | Name.com, Inc.

The infrastructure currently in use by the Linux versions of DinodasRAT appeared to be up and running at the time this implant was being analyzed. We identified one IP address resolving for both the Windows and Linux variants’ C2 domains. The Windows version of DinodasRAT uses the domain update.microsoft-settings[.]com, which resolves to the IP address 199.231.211[.]19. This IP address also resolves to update.centos-yum[.]com, which (interestingly enough) uses the same pattern of operating system update subdomain and domain.

## Victims

In our telemetry data and continuous monitoring of this threat since October 2023, we’ve observed that the most affected countries and territories are China, Taiwan, Turkey and Uzbekistan.

All Kaspersky products detect this Linux variant as HEUR:Backdoor.Linux.Dinodas.a.

## Conclusion

In October 2023, ESET published an article about a campaign dubbed Operation Jacana targeting Windows users. As part of our ongoing monitoring efforts, we discovered that the Jacana operators possess and act on their ability to infect Linux infrastructure with a new and previously unknown and undetected Linux DindoasRAT variant, whose code and networking indicators of compromise overlap with the Windows samples described by ESET. They do not collect user information to manage infections. Instead, hardware-specific information is collected and used to generate a UID, demonstrating that DinodasRAT’s primary use case is to gain and maintain access via Linux servers rather than reconnaissance.

The backdoor is fully functional, granting the operator complete control over the infected machine, enabling data exfiltration and espionage.

A more detailed analysis of the latest DinodasRAT versions is available to customers of our private [Threat Intelligence reports](https://www.kaspersky.com/enterprise-security/threat-intelligence). If you have any questions, please contact crimewareintel@kaspersky.com.

## Indicators of compromise

Host-based:

- [8138f1af1dc51cde924aa2360f12d650](https://opentip.kaspersky.com/8138f1af1dc51cde924aa2360f12d650/?utm_source=SL&utm_medium=SL&utm_campaign=SL)
- [decd6b94792a22119e1b5a1ed99e8961](https://opentip.kaspersky.com/decd6b94792a22119e1b5a1ed99e8961/?utm_source=SL&utm_medium=SL&utm_campaign=SL)

Network-based:

- [update.centos-yum[.]com](https://opentip.kaspersky.com/update.centos-yum.com/?utm_source=SL&utm_medium=SL&utm_campaign=SL) (199.231.211[.]19)

- [Malware Technologies](https://securelist.com/tag/malware-technologies/)
- [Linux](https://securelist.com/tag/linux/)
- [RedHat](https://securelist.com/tag/redhat/)
- [Ubuntu](https://securelist.com/tag/ubuntu/)
- [Malware Descriptions](https://securelist.com/tag/malware-descriptions/)
- [Malware](https://securelist.com/tag/malware/)
- [Trojan](https://securelist.com/tag/trojan/)
- [Backdoor](https://securelist.com/tag/backdoor/)
- [DinodasRAT](https://securelist.com/tag/dinodasrat/)

DinodasRAT Linux implant targeting entities worldwide

###

This site uses Akismet to reduce spam. [Learn how your comment data is processed.](https://akismet.com/privacy/)

Table of Contents

- [Initial infection overview](https://securelist.com/dinodasrat-linux-implant/112284/#initial-infection-overview)
- [Victim ID generation and persistence](https://securelist.com/dinodasrat-linux-implant/112284/#victim-id-generation-and-persistence)
- [C2 Communication](https://securelist.com/dinodasrat-linux-implant/112284/#c2-communication)
- [Encryption](https://securelist.com/dinodasrat-linux-implant/112284/#encryption)
- [Infrastructure](https://securelist.com/dinodasrat-linux-implant/112284/#infrastructure)
- [Victims](https://securelist.com/dinodasrat-linux-implant/112284/#victims)
- [Conclusion](https://securelist.com/dinodasrat-linux-implant/112284/#conclusion)
- [Indicators of compromise](https://securelist.com/dinodasrat-linux-implant/112284/#indicators-of-compromise)

GReAT webinars

13 May 2021, 1:00pm

### https://securelist.com/webinars/great-ideas-balalaika-edition/[GReAT Ideas. Balalaika Edition](https://securelist.com/webinars/great-ideas-balalaika-edition/)

26 Feb 2021, 12:00pm

### https://securelist.com/webinars/great-ideas-green-tea-edition/[GReAT Ideas. Green Tea Edition](https://securelist.com/webinars/great-ideas-green-tea-edition/)

17 Jun 2020, 1:00pm

### https://securelist.com/webinars/great-ideas-powered-by-sas-malware-attribution-and-next-gen-iot-honeypots/[GReAT Ideas. Powered by SAS: malware attribution and next-gen IoT honeypots](https://securelist.com/webinars/great-ideas-powered-by-sas-malware-attribution-and-next-gen-iot-honeypots/)

26 Aug 2020, 2:00pm

### https://securelist.com/webinars/great-ideas-powered-by-sas-threat-actors-advance-on-new-fronts/[GReAT Ideas. Powered by SAS: threat actors advance on new fronts](https://securelist.com/webinars/great-ideas-powered-by-sas-threat-actors-advance-on-new-fronts/)

22 Jul 2020, 2:00pm

### https://securelist.com/webinars/great-ideas-powered-by-sas-threat-hunting-and-new-techniques/[GReAT Ideas. Powered by SAS: threat hunting and new techniques](https://securelist.com/webinars/great-ideas-powered-by-sas-threat-hunting-and-new-techniques/)

From the same authors

https://securelist.com/tsundere-node-js-botnet-uses-ethereum-blockchain/117979/

### [Blockchain and Node.js abused by Tsundere: an emerging botnet](https://securelist.com/tsundere-node-js-botnet-uses-ethereum-blockchain/117979/)

https://securelist.com/revengehotels-attacks-with-ai-and-venomrat-across-latin-america/117493/

### [RevengeHotels: a new wave of attacks leveraging LLMs and VenomRAT](https://securelist.com/revengehotels-attacks-with-ai-and-venomrat-across-latin-america/117493/)

https://securelist.com/browservenom-mimicks-deepseek-to-use-malicious-proxy/115728/

### [Toxic trend: Another malware threat targets DeepSeek](https://securelist.com/browservenom-mimicks-deepseek-to-use-malicious-proxy/115728/)

https://securelist.com/mirai-botnet-variant-targets-dvr-devices-with-cve-2024-3721/116742/

### [Analysis of the latest Mirai wave exploiting TBK DVR devices with CVE-2024-3721](https://securelist.com/mirai-botnet-variant-targets-dvr-devices-with-cve-2024-3721/116742/)

https://securelist.com/xz-backdoor-part-3-hooking-ssh/113007/

### [XZ backdoor: Hook analysis](https://securelist.com/xz-backdoor-part-3-hooking-ssh/113007/)

In the same category

https://securelist.com/horabot-campaign/119033/

### [The SOC Files: Time to “Sapecar”. Unpacking a new Horabot campaign in Mexico](https://securelist.com/horabot-campaign/119033/)

https://securelist.com/lumma-fake-captcha-attacks-analysis/116274/

### [Lumma Stealer – Tracking distribution channels](https://securelist.com/lumma-fake-captcha-attacks-analysis/116274/)

https://securelist.com/mamont-banker-disguised-as-parcel-tracking-app/115006/

### [Download a banker to track your parcel](https://securelist.com/mamont-banker-disguised-as-parcel-tracking-app/115006/)

https://securelist.com/elpaco-ransomware-a-mimic-variant/114635/

### [Analysis of Elpaco: a Mimic variant](https://securelist.com/elpaco-ransomware-a-mimic-variant/114635/)

https://securelist.com/new-ymir-ransomware-found-in-colombia/114493/

### [Ymir: new stealthy ransomware in the wild](https://securelist.com/new-ymir-ransomware-found-in-colombia/114493/)

-

https://lp.kaspersky.com/global/next/?icid=gl_sl_NextHero2026_sm-team_cbff6db20a65e744

https://securelist.com/intelbras-camera-vulnerability/121131/

[GReAT research](https://securelist.com/category/great-research/)

### [Millions of security cameras at risk: DoS vulnerability in Intelbras devices](https://securelist.com/intelbras-camera-vulnerability/121131/)

https://securelist.com/cloud-infrastructure-isolation-bypass/121609/

[SAS](https://securelist.com/category/sas/)

### [An insecure neighbor: how a vulnerability chain led to an isolation bypass in cloud infrastructure](https://securelist.com/cloud-infrastructure-isolation-bypass/121609/)

https://securelist.com/tr/how-to-secure-ai-agents/121444/

### [Deploying AI agents securely: how Kaspersky products mitigate AI threats](https://securelist.com/tr/how-to-secure-ai-agents/121444/)

https://securelist.com/uefi-bootkit-blackx/121539/

[SAS](https://securelist.com/category/sas/)

### [How APT41 revived the BlackLotus bootkit to attack diplomatic organizations in Central Asia](https://securelist.com/uefi-bootkit-blackx/121539/)

https://securelist.com/webinars/overddose-inside-the-ddos-for-hire-market/

[Cyberthreat talks](https://securelist.com/webinar-category/cyberthreat-talks/)

08 Oct 2026, 5:00pm  58 min

### [OverDDoSe: Inside the DDoS-for-hire market](https://securelist.com/webinars/overddose-inside-the-ddos-for-hire-market/)

https://securelist.com/webinars/building-trusted-cybersecurity-for-government/

[Technologies and services](https://securelist.com/webinar-category/technologies-and-services/)

29 Sep 2026, 5:00pm  67 min

### [Building trusted cybersecurity for government](https://securelist.com/webinars/building-trusted-cybersecurity-for-government/)

https://securelist.com/webinars/soc-maturity-model-what-suits-your-business-the-most/

[Trainings and workshops](https://securelist.com/webinar-category/trainings-and-workshops/)

04 Aug 2026, 5:00pm  67 min

### [SOC maturity model. What suits your business the most?](https://securelist.com/webinars/soc-maturity-model-what-suits-your-business-the-most/)

https://securelist.com/webinars/from-easy-target-to-ready-business-a-practical-smb-cybersecurity-guide/

[Cyberthreat talks](https://securelist.com/webinar-category/cyberthreat-talks/)

16 Jul 2026, 5:00pm  45 min

### [From easy target to ready business: A practical SMB cybersecurity guide](https://securelist.com/webinars/from-easy-target-to-ready-business-a-practical-smb-cybersecurity-guide/)

### [Mirage Kitten targeting aviation and FinTech sectors across the Middle East and Africa with a new malware set](https://securelist.com/mirage-kitten-new-backdoors-noderabbit-pollcat/121244/)

Kaspersky researchers have discovered new Mirage Kitten attacks using previously undocumented malware families: NodeRabbit in Node.js and PollCat in JavaScript.

### [APT group HoneyMyte upgrades CoolClient: the backdoor gets a kernel-level Windows rootkit](https://securelist.com/honeymyte-coolclient-driver-rootkit/121028/)

Our experts discovered a new CoolClient backdoor variant with a kernel-mode rootkit driver that hides malicious processes, files, and network connections from security tools and threat analysts.

### [Armored Likho expands its cyber-espionage toolkit](https://securelist.com/armored-likho-still-toolkit/121033/)

Kaspersky experts break down a new Armored Likho campaign that poses as a fundraising efforts and delivers a new Still Toolkit aimed at stealing Telegram data and eavesdropping on victims.

### [Mirage Kitten targets Middle East and Africa region with new malware](https://securelist.com/mirage-kitten-new-tools/120811/)

Kaspersky researchers reveal previously undocumented malware attributed to Mirage Kitten (UNC1549, Smoke Sandstorm, Nimbus Manticore): NightLedger backdoor, ArcBridge, and BridgeHead tunneling tools.
