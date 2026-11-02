---
url: "https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/"
title: "In-Depth Analysis: Velvet Ant's Prolonged Cyber Attack on a Large Organization"
site: "Sygnia"
published: "2024-06-03T09:28:43+00:00"
captured: "2026-11-02T19:00:14Z"
via: "live"
sha256: "ba94a4729fa38ce86ddab5d68eebcb42334d5f95f69bac964ba72bf581c2355b"
wayback: "https://web.archive.org/web/20260831104046/https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# In-Depth Analysis: Velvet Ant's Prolonged Cyber Attack on a Large Organization

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/>
> Snapshot: <https://web.archive.org/web/20260831104046/https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/>

https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fwww.sygnia.co%2Fblog%2Fchina-nexus-threat-group-velvet-ant%2F

https://twitter.com/intent/tweet?text=China-Nexus%20Threat%20Group%20%E2%80%98Velvet%20Ant%E2%80%99%20Abuses%20F5%20Load%20Balancers%20for%20Persistence&url=https%3A%2F%2Fwww.sygnia.co%2Fblog%2Fchina-nexus-threat-group-velvet-ant%2F

https://www.linkedin.com/shareArticle?mini=true&url=https%3A%2F%2Fwww.sygnia.co%2Fblog%2Fchina-nexus-threat-group-velvet-ant%2F

# China-Nexus Threat Group ‘Velvet Ant’ Abuses F5 Load Balancers for Persistence

Discover the detailed forensic investigation by Sygnia into the sophisticated cyber attack by Velvet Ant on a major organization.

Sygnia Team

3 June 2024

22 min

[Blog](https://www.sygnia.co/knowledge-center/?format[]=1) [Attack Techniques](https://www.sygnia.co/knowledge-center/?topic[]=88) [Cyber Crime](https://www.sygnia.co/knowledge-center/?topic[]=94) [Detection](https://www.sygnia.co/knowledge-center/?topic[]=89) [Ransomware](https://www.sygnia.co/knowledge-center/?topic[]=95)

Contents

-  [Key Takeaways](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/#1)
-  [Introduction](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/#2)
-  [It all began with PlugX](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/#3)

-  [Lateral Movement and C&C](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/#4)
-  [PlugX is Dead; Long Live PlugX](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/#5)

-  [F5 BIG-IP - The Perfect Place to Hide](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/#6)

-  [Pivoting From the F5 Appliance](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/#7)
-  [Additional Malware on the F5 Appliances](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/#8)

-  [A Note on Attribution](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/#9)
-  [Defending Against Velvet Ant](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/#10)
-  [Appendix I: Indicators of Compromise](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/#11)

https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fwww.sygnia.co%2Fblog%2Fchina-nexus-threat-group-velvet-ant%2F

https://twitter.com/intent/tweet?text=China-Nexus%20Threat%20Group%20%E2%80%98Velvet%20Ant%E2%80%99%20Abuses%20F5%20Load%20Balancers%20for%20Persistence&url=https%3A%2F%2Fwww.sygnia.co%2Fblog%2Fchina-nexus-threat-group-velvet-ant%2F

https://www.linkedin.com/shareArticle?mini=true&url=https%3A%2F%2Fwww.sygnia.co%2Fblog%2Fchina-nexus-threat-group-velvet-ant%2F

## Key Takeaways

- In late 2023, a large organization was the victim of a serious cyber attack. Sygnia’s forensic investigation into the attack revealed a sophisticated threat actor who exhibited robust capabilities and employed a methodical approach. The evidence gathered suggests the involvement of a China-nexus state-sponsored threat actor.
- Velvet Ant is a sophisticated and innovative threat actor. The investigation confirmed the threat actor maintained a prolonged presence in the organization’s on–premises network for about three years. The overall goal behind this campaign was to maintain access to the target network for espionage.
- The threat actor achieved remarkable persistence by establishing and maintaining multiple footholds within the victim company’s environment. One of the mechanisms utilized for persistence was a legacy F5 BIG-IP appliance, which was exposed to the internet and which the threat actor leveraged as an internal Command and Control (C&C).
- After one foothold was discovered and remediated, the threat actor swiftly pivoted to another, demonstrating agility and adaptability in evading detection.
- The threat actor exploited various entry points across the victim’s network infrastructure, indicating a comprehensive understanding of the target’s environment.
- This incident highlights the importance of establishing resilient defense strategies against sophisticated threats – particularly those posed by state-sponsored groups. A holistic approach to mitigating these threats combines continuous monitoring with proactive response mechanisms – including periodic and systematic threat hunts – alongside stringent traffic controls and system hardening practices for both legacy and public-facing devices. By embracing such an approach, organizations can enhance their ability to detect, deter, and counteract the persistent threat presented by state-sponsored groups.

## Introduction

In late 2023, Sygnia responded to a cyber attack targeting a large organization. During the investigation, Sygnia determined that the threat actor exhibited behaviors like those of a China-nexus threat actor. This threat actor, which Sygnia tracks as Velvet Ant, maintained access to the organization’s network for at least three years, and had succeeded in gaining a strong foothold, and intimate knowledge of the network.

Over the course of the attack, the threat actor adeptly used various tools and techniques to infiltrate critical systems and obtain access to sensitive data. The prolonged period of the attack allowed the threat actor to familiarize itself with the complex network infrastructure, enabling it to conceal persistence mechanisms within overlooked areas. Although Sygnia eventually ousted the threat actor from the network, the eradication process resembled a relentless game of cat-and-mouse. Despite Sygnia’s diligent efforts to remediate compromised systems and enhance visibility into hosts and network devices, the threat actor resurfaced time and again through the use of dormant persistence mechanisms in unmonitored systems.

This blog describes the usage of one of those mechanisms, which included leveraging legacy servers and unpatched network appliances.

## It all began with PlugX

Throughout the duration of the attack, one of Velvet Ant’s attack techniques was to [hijack execution flow](https://attack.mitre.org/techniques/T1574/), by leveraging different methods such as [DLL search order hijacking](https://attack.mitre.org/techniques/T1574/001/), [Phantom DLL loading](https://resources.infosecinstitute.com/topics/hacking/dll-hijacking-attacks-revisited/) and [DLL side loading](https://attack.mitre.org/techniques/T1574/002/).

Soon after starting the forensic investigation, Sygnia identified the various tools employed by the threat actor, determined the scope of compromise, and formulated a remediation plan. The plan combined eradication, remediation, and visibility enhancement, which focused on the threat actor’s Tactics, Techniques and Procedures (TTPs).

The plan was executed successfully. Alerts that were configured to identify re-entry attempts were not triggered, and Sygnia’s experts worked with the IT and security teams of the victim company to bolster overall security. However, the success of the remediation and hardening efforts drove the threat actor to focus on legacy operating systems, targeting Windows Server 2003 systems on which the organization’s Endpoint Detection and Response (EDR) product was not installed, and logging was limited. The threat actor resumed activity by utilizing previously deployed malware that remained dormant for months in the victim environment. The malware was identified as PlugX, a well-known remote access Trojan.

[PlugX](https://mcas-proxyweb.mcas.ms/certificate-checker?login=false&originalUrl=https%3A%2F%2Fattack.mitre.org.mcas.ms%2Fsoftware%2FS0013%2F%3FMcasTsid%3D20892&McasCSRF=https://attack.mitre.org/software/S0013/) has been used by multiple Chinese state sponsored groups since 2008. The tool is still [widely deployed](https://www.recordedfuture.com/2023-adversary-infrastructure-report) – although its successor, ShadowPad [has been in use since 2015](https://www.sentinelone.com/labs/shadowpad-a-masterpiece-of-privately-sold-malware-in-chinese-espionage/). PlugX was primarily designed to provide remote access to infected systems. However, it has a modular plugin system, which provides attackers with a large set of additional capabilities that can be utilized for nefarious purposes.

The PlugX execution chain in this network consisted of three files: ‘iviewers.exe’, ‘iviewers.dll’ and ‘iviewers.dll.ui’.

- ‘iviewers.exe’ is a legitimate application called ’OLE/COM Object Viewer‘, that is part of the Windows SDK.
- ‘iviewers.dll’ is the malicious PlugX DLL loader, that is loaded by ‘iviewers.exe’ via DLL search order hijacking.
- ‘iviewers.dll.ui’ contains the actual malicious payload, which is loaded by ‘iviewers.dll’.

When ‘iviewers.exe’ is executed, these three files are copied to a sub-directory with a non-fixed name under ‘C:\ProgramData’ (or ‘C:\Documents and Settings\All Users\Application Data’, on Windows Server 2003 systems), and ‘iviewers.exe’ is installed as a Windows service. Afterwards, several ‘Svchost’ processes are launched, and code is injected into them.

Figure 1: Snippet from Sygnia’s Velocity XDR system, showing the three files that were created by the malware on an infected system. The creation time of the files was manipulated by the malware and does not reflect the actual creation time.

Figure 1: Snippet from Sygnia’s Velocity XDR system, showing the three files that were created by the malware on an infected system. The creation time of the files was manipulated by the malware and does not reflect the actual creation time.

Figure 2: Snippet from VMRay sandbox, showing a process graph for an execution of ‘iviewers.exe’.

Throughout the investigation, Sygnia acquired memory dumps of those ‘Svchost’ processes, which proved helpful in tracking the threat actor’s activities – especially on legacy servers, where logs were lacking. The memory dumps contained harvested credentials and various commands executed by the threat actor.

Figure 3: Snippet from a memory dump of an injected ‘Svchost’ process, showing credential harvesting.

Figure 4: Snippet from a memory dump of an injected ‘Svchost’ process, showing part of a command that was executed to scan an internal network segment.

### Lateral Movement and C&C

When moving laterally to workstations installed with newer Windows versions, the threat actor consistently tampered with the EDR product prior to installing PlugX. On one occasion, the threat actor attempted to disable the EDR product on a target workstation but failed, and then did not proceed to install PlugX. This demonstrates a high level of operational security (OPSEC) awareness.

[Impacket](https://github.com/fortra/impacket/tree/master), an open-source collection of Python classes, was used for lateral tool transfer, and to execute code remotely on hosts. Specifically, the threat actor employed Impacket’s [wmiexec.py](https://github.com/fortra/impacket/tree/master), a tool that utilizes the native Windows Management Instrumentation (WMI) to execute remote commands.

Figure 5: Snippet from an MFT entry of a file created by WmiExec to store command output. This output belongs to a ‘del’ command, which was executed to delete EDR anti-malware logs.

The PlugX C&C address, 202.61.136[.]158, was also found in these memory dumps. This allowed the creation of network-based detections, and assisted Sygnia in scoping the incident.

Figure 6: Snippet from VirusTotal, showing that the malware’s C&C IP address was identified as a PlugX C&C server.

The ‘Svchost’ process launched by ‘iviewers.exe’ binds to a pre-configured local port and accepts TCP connections. It also creates a new rule in the Windows firewall called ‘Network Discovery (SSDP-In)’, to allow these connections. This behavior did not seem to be related to the C&C mechanism, and its purpose was initially unclear.

Figure 7: Snippet from Sygnia’s XDR system, Velocity, showing a Sysmon process creation event for a ‘netsh’ command which was launched on a legacy file serve by the injected ‘Svchost’ process to create a local firewall rule.

The purpose of the local firewall rule soon became clear, following a kill-switch operation that initiated a dramatic turn of events.

### PlugX is Dead; Long Live PlugX

Equipped with the forensic investigation findings, the Sygnia team again worked with the victim company to eradicate the discovered threats. Dozens of compromised hosts were re-imaged, many of the legacy servers were decommissioned, hundreds of indicators of compromise (IOCs) were blocked across the network, and all the known instances of threat actor access to the network were eradicated.

The victim organization has a large network with complex architecture. The post-breach remediation plan designed by Sygnia included prioritized tasks divided into immediate and mid-term recommendations. As a standard procedure, Sygnia enhanced the organization’s visibility, and established advanced monitoring based on the indicators of attack (IOA).

Several days after eliminating Velvet Ant’s presence, the advanced monitoring capabilities that were deployed in the network proved their value, as alerts were triggered when PlugX was deployed on newly-infected systems. Sygnia’s investigation team quickly analyzed the new infections and noticed a substantial difference in the memory dump acquired from the newly infected hosts – there was no external C&C configured. This was puzzling; PlugX is a remote access tool – how can it be deployed without a C&C server, and still be effective?

Additional forensic analysis revealed that the threat actor reconfigured PlugX to use an internal file server as its C&C and channeled traffic through that server. This defense evasion technique allowed the C&C traffic to blend in with legitimate internal network traffic.

This meant that the threat actor deployed two versions of PlugX within the network. The first version, configured with an external C&C server, was installed on endpoints with direct internet access, facilitating the exfiltration of sensitive information. The second version did not have a C&C and was deployed exclusively on legacy servers.

This left us with unanswered questions. How did the threat actor control the legacy file server? And how did they manage to restore access to the organization despite the comprehensive remediation activities? To answer these questions, Sygnia’s incident response team set up additional network and host-based monitoring tools on the file server and closely monitored for any suspicious activities.

As shown in figure 7, PlugX created a local firewall rule on compromised endpoints and listened on a high, random port number. On the file server, port 13742 was opened. Analyzing network connections to the file server identified an internal IP address communicating with it on the same port. The internal IP address belonged to an F5 load balancer.

Figure 8: Snippet from Sygnia’s XDR system, Velocity, showing the network connection event from the F5 load balancer to the file server on port 13742.

This load balancer was not part of previous remediation efforts, because it was not supposed to be operational in the production network. One team in the organization started deploying the F5 solution in the network a long time ago as part of a disaster recovery plan (DRP), but the project was never completed.

## F5 BIG-IP – The Perfect Place to Hide

F5 BIG-IP appliances occupy a trusted position within the network architecture, often placed at the perimeter or between different network segments. By compromising such a device, attackers can exert significant control over network traffic without arousing suspicion. Moreover, while organizations frequently gather logs from network devices, their attention is predominantly directed towards application-level logs such as traffic records and alerts. Unfortunately, operating system logs are often overlooked, either due to vendor constraints or a lack of recognition regarding their significance. As a result, a backdoor hidden within the F5 appliance can evade detection from traditional log monitoring solutions.

The compromised organization had two F5 BIG-IP appliances which provided services such as firewall, WAF, load balancing and local traffic management. These appliances were directly exposed to the internet, and both of which were compromised.

Both F5 appliances were running an outdated, vulnerable, operating system. The threat actor may have leveraged one of the vulnerabilities to gain remote access to the appliances. However, visibility limitations hinder the ability to identify exactly how the appliances were compromised.​

Forensic investigation of the appliances revealed a reverse SSH tunnel connection to the same C&C IP address that was previously blocked on the corporate firewalls during the eradication efforts. Since the appliances were not located behind the main corporate firewall, the traffic was not blocked.

Figure 9: Snippet showing the output of a ‘ps’ command executed on one of the F5 appliances. The threat actor created a reverse SSH tunnel to the C&C with a user named ‘leo’.

Analyzing the active network connections on the F5, an established connection was observed between the appliance and the file server, on the port PlugX was listening on. This complemented the analysis made on the file server itself (see figure 8).

Figure 10: Snippet from an output of ‘netstat’ command executed on the F5 appliance, showing the connection to the file server.

Figure 11: Snippet extracted from Wireshark, showing that the traffic transmitted from the F5 appliances was sent to port 13742 of the file server. This is the port that PlugX was listening on in this server.

### Pivoting From the F5 Appliance

The figure below depicts the stages of the threat actor’s working flow.

(1) The malware running on the device polls the C&C server once an hour. After connecting to the C&C server (2), the threat actor sends commands to the compromised appliance, including for the creation of a reverse SSH tunnel (3). Once the tunnel with the appliance is established, the threat actor connects to the file server that is infected with PlugX (4).

The PlugX instance on the compromised file server was used by the threat actor as an internal C&C server. From this server, the threat actor conducted reconnaissance activities, deployed additional instances of the PlugX onto legacy servers by leveraging Impacket’s WmiExec (5).

Figure 12: Diagram showing how the F5 appliance was leveraged by the threat actor as a persistent beachhead, utilized to move laterally and execute remote commands on different servers.

Recording the traffic between the F5 appliance and the file server revealed additional details about the threat actor’s TTPs, as the traffic was sent over SMB, and was not encrypted. The following figures show the interactive activities conducted by the threat actor prior to deploying additional PlugX instances.

The threat actor’s first step was to enumerate the active network connections on the targeted server:

Figure 13: Snippet extracted from Wireshark, showing that the threat actor enumerated the active network connections before moving laterally to additional servers.

After selecting a new server for PlugX deployment, the threat actor listed files in different folders, presumably in order to choose where to deploy the malware.

Figure 14: Snippet from Wireshark showing how the threat actor enumerated additional legacy systems within the network by executing commands through WmiExec.

The threat actor then chose a folder, and leveraged WmiExec to transfer PlugX over SMB from the file server to the target server.

Figure 15: Snippet from Wireshark showing the iviewers PlugX variant being sent over SMB and saved into the ‘C:\tools\’ folder.

### Additional Malware on the F5 Appliances

Forensic analysis of the F5 appliances identified four binaries deployed by the threat actor:

- VELVETSTING – a tool that connects to the threat actor’s C&C once an hour, searching commands to execute. The threat actor used the IP address 202.61.136[.]158:8443 as a C&C and the commands were encoded with the passphrase ‘1qaz@WSXedc’. Once the tool received a command, it was executed via ‘csh’ (Unix C shell).

Figure 16: Snippet showing the output of the ‘strace’ command, which monitored the ‘VELVETSTING’ process. The malware tried to connect to the C&C server on port 8443 every 3600 seconds (1 hour). Once the connection was successful, the malware created a child process based on the commands it received from the C&C.

- VELVETTAP – a tool with the ability to capture network packets. The binary was executed on the F5 appliance with the argument ‘mgmt’, which is the name of the internal NIC of the F5 device.

Figure 17: Snippet from an output of ‘ifconfig’ command, showing the ‘mgmt’ network interface on the F5 appliance.

To maintain persistence on the F5 appliance, the threat actor added both VELVETSTING and VELVETTAP to the ‘/etc/rc.local’ file. ‘rc.local’ is executed as part of the system’s startup process, which means that any commands or scripts listed in it will be executed automatically whenever the system boots up.

Figure 18: Snippet showing that the paths for VELVETSTING and VELVETTAP were added to the ‘/etc/rc.local’ file by the threat actor.

- SAMRID – identified as ‘EarthWorm’, an open-source SOCKS proxy tunneller available on [GitHub](https://github.com/rootkiter/EarthWorm). The tool was utilized in the past by different Chinese state-sponsored groups, such as ‘[Volt Typhoon](https://attack.mitre.org/groups/G1017/)’, ‘[APT27](https://securelist.com/luckymouse-ndisproxy-driver/87914/)’ and ‘[Gelsemium](https://unit42.paloaltonetworks.com/rare-possible-gelsemium-attack-targets-se-asia/)’. The tool was not running on the device at the time of investigation.
- ESRDE – a tool with similar capabilities to that of ‘VELVETSTING’, but with minor differences, such as using bash instead of ‘csh’. The tool was not running on the device at the time of investigation.

## A Note on Attribution

The attack described in this blog demonstrates a single attack flow of Velvet Ant, a threat actor that exhibited characteristics like those of a China-nexus state sponsored threat actor. These include but are not limited to target selection (industry and geo-location), clear definition of goals, targeting of network devices, exploitation of vulnerabilities, and a toolset that includes the ShadowPad and PlugX malware families, and utilization of DLL side-loading techniques. The threat actor was extremely persistent and remained active in the compromised network for about three years, despite several eradication attempts throughout that period.

The modus operandi of Chinese-nexus Intrusion Sets includes shared tools, infrastructure, and sometimes manpower – for example via shared contractors. This mode of operation makes it difficult for Sygnia to attribute the attack to any previously publicly-reported group. Moreover, the visibility available in the network prevents Sygnia from ruling out a ‘false-flag’ operation conducted by a different Advanced Persistent Threat (APT) group.

## Defending Against Velvet Ant

The defense strategies presented below are designed to provide organizations with robust measures for countering threats related to C&C and the malicious weaponization of legacy and edge devices strengthening organizational cyber security defenses and strengthening resilience against advanced persistent attacks.

- Limit outbound internet traffic: Outbound internet connections should be restricted across servers, workstations, and edge devices, to prevent attackers from maintaining access via Command and Control (C&C) channels – similar to the tactic of utilizing the PlugX loader. Configure perimeter firewalls to allow only the necessary outbound connections, placing internet-facing devices such as load balancers behind these firewalls to prevent direct public access.
- Limit lateral movement throughout the network: Performing lateral movement within networks is a technique frequently employed by threat actors using tools like WmiExec. To mitigate this risk, it is essential to rigorously control traffic over common management ports. These include SMB on port 445, RPC on port 135, WinRM on ports 5985-5986, RDP on port 3389, and SSH on port 22. Access should be strictly limited to hosts with explicit authorization. Although implementing such controls poses challenges in complex network environments, the security benefits are substantial. Mitigation strategies may include deploying host-based firewalls,establishing effective network segmentation, and adopting cutting-edge microsegmentation technologies.
- Enhance security hardening of legacy servers: Prioritize decommissioning and replacement of legacy systems. Tighten control over legacy servers, which are prime targets for cyberattacks due to their weaker defenses. Enhance security by installing Endpoint protection tools that support older systems such as Windows Server 2003, or using alternatives like Sysmon for comprehensive monitoring. Strengthen protections by limiting access to essential staff, enforcing strict network segmentation, and monitoring traffic to and from these servers. These steps considerably reduce the vulnerabilities inherent in older server infrastructures.
- Mitigate credential harvesting: Implement Endpoint Detection and Response (EDR) systems for continuous monitoring and interception of malicious actions. Ensure that EDR sensors are configured with anti-tampering features, and are consistently operational and current, as attackers might attempt to neutralize or circumvent the sensors. Apply Protected Process Light (PPL) to LSASS, and activate Windows Credential Guard for credential security.
- Protect public-facing devices: To improve the security of public-facing edge devices such as F5 BIG-IP load balancers, it is essential to implement a proactive security strategy that includes external attack surface and asset management, patch management, Intrusion Detection and Prevention Systems, and robust security and integrity monitoring. Additionally, migration to cloud-based solutions such as SASE or cloud-native load balancers can reduce the risk of running vulnerable, exploitable edge devices. For comprehensive guidance on establishing such defenses, refer to Sygnia’s blog “[Defending Your Network Edge Against the Next Zero-Day Exploit](https://www.sygnia.co/threat-reports-and-advisories/defending-your-network-edge-against-the-next-zero-day-exploit/)”, which outlines key measures for enhancing the resilience of network edges against threats demonstrated by this threat actor, and other emerging threats.

## Appendix I: Indicators of Compromise

| Value | Type | Description

| d1e6767900c85535f300e08d76aac9ab  | MD5  | iviewers.exe

| 4a0f328e7672ee7ba83f265d48a6077a0c9068d4  | SHA1  | iviewers.exe

| 91f6547bceddfb2f241570ac82c00de700e311e4a38dea60d8619638f1ed3520  | SHA256  | iviewers.exe

| 0d5abbe83e5eeb2cb79630caba3a33c7  | MD5  | iviewers.exe

| d80427c922db5fcd8cf490a028915485ff833666  | SHA1  | iviewers.exe

| d663b323d132a3c811bb53a48a686ea85c6bf8faeef3b48dfa93528be8f4133b  | SHA256  | iviewers.exe

| 977a7e48f8b05c12249a28dbc4054d78  | MD5  | iviewers.exe.ui

| 291bcceef6e03a9f4f0c524f1dd3a4b77d870cd8  | SHA1  | iviewers.exe.ui

| 9a7a24b1c785b3c7c39f7e33e99897290165693dea1f46ed4f3c7919aef93928  | SHA256  | iviewers.exe.ui

| 4cdeffe8c379e6b702f2d22160c59ccf  | MD5  | iviewers.exe.ui

| f07272762b322cea1d8cc0845718371f1af0bd4a  | SHA1  | iviewers.exe.ui

| 75fa71e65344b61a80f0e598349b735912be39d04a7e2159748423bd860d3454  | SHA256  | iviewers.exe.ui

| dacfc13d17cda55e58fab7d66d5417d1  | MD5  | iviewers.exe.ui

| 37d3665d3b803eeddfad245c0e96172b9c3e8a29  | SHA1  | iviewers.exe.ui

| be852d7a59ba168d93eb975fbed652617046433e6fdc177d0087331f9a095f02  | SHA256  | iviewers.exe.ui

| 74a6c8bb6fb08c68d0ff4a6efad64242  | MD5  | iviewers.exe.ui

| 2c5d678948938de4d10095db35390c064305413c  | SHA1  | iviewers.exe.ui

| 0acc25396ef78c00631c64df538678a323982115bafbfa7487a4370d4b4129ac  | SHA256  | iviewers.exe.ui

| cefbb71fd132df7fb913ec747080da7c  | MD5  | iviewers.exe.ui

| 6003f8042d375ec5c6d56a1d6e363e2d2cc9eb67  | SHA1  | iviewers.exe.ui

| 859c823eb3e7420e0db234ba224764faa62d391bddd25e9ad415b11d853741f9  | SHA256  | iviewers.exe.ui

| ababde83c740651f014d9671d4dab557  | MD5  | iviewers.exe.ui

| 1fc7b986e55f116d92e77e3b2bee86b720ffa155  | SHA1  | iviewers.exe.ui

| 9b9d2da73b510276d38d1698f3b87671958e338b40230e6a004ccaf3dcceb03b  | SHA256  | iviewers.exe.ui

| ad2d2126fe198b35a657804c7cbbf84f  | MD5  | iviewers.exe.ui

| 0b400eb4451c3148fa48bc72cb8a84fdcf4461d3  | SHA1  | iviewers.exe.ui

| b4d71b0ac0bc1495789501f9afce6f950b601a36c0836534294640f2db6b2f40  | SHA256  | iviewers.exe.ui

| 654349edf1ac14dabce9bec435f06f98  | MD5  | iviewers.exe.ui

| 49d2e3dfabd21ed4a11c6fca6236ced7b17fa97e  | SHA1  | iviewers.exe.ui

| 3a6a5b1d76dfcac5920e6e9163c08543304ba013425eb2c2e64071b15d26996e  | SHA256  | iviewers.exe.ui

| 113779c96c005ac50729462ec1b81f96  | MD5  | iviewers.exe.ui

| e6bd682c47f1a9d564f45a54427100b42e19d2e9  | SHA1  | iviewers.exe.ui

| bdf8a8c7f0298484dc95895dbddf367689ca361618453597129343838b94debb  | SHA256  | iviewers.exe.ui

| 0b0b592ec201605503b4a245f024b37c  | MD5  | iviewers.exe.ui

| fc06519154e3a4b28fe16606dec05ec02dd2f647  | SHA1  | iviewers.exe.ui

| 7c9336afd7530576b6a0f2e978b36955e8f264fee429d810309ce157a4918aaa  | SHA256  | iviewers.exe.ui

| 7266fb2e71fc97036ad642fb592d3444  | MD5  | iviewers.exe.ui

| ca7331e0c8dda90054eb941a2fdd0cc943a04fc4  | SHA1  | iviewers.exe.ui

| c456747731141c2ea0f8e69f89193e8bb823da4667527fe90b614b97f1d425ae  | SHA256  | iviewers.exe.ui

| c5af1894f9806fafc6eae449a4021362  | MD5  | iviewers.exe.ui

| 61a382b2139512f8c816ceae93ec823c88bd6eed  | SHA1  | iviewers.exe.ui

| 55d6c4a95b5172ea47381ab66ea9ea37fa0afb53b9bb10a1d752ac4acc8f6cd4  | SHA256  | iviewers.exe.ui

| 52692f03f7c14bc6a6bd35679beb7fab  | MD5  | iviewers.exe.ui

| 8e722b2c6b114b69bd71c37759dc3410a32b7594  | SHA1  | iviewers.exe.ui

| 527df166af23cd0d139ebad9d219f125137b5a7b619fa50e5e245ccaf8c0b7d6  | SHA256  | iviewers.exe.ui

| b9a46c2960ddbece1a5fc4db1a810d92  | MD5  | iviewers.exe.ui

| 35e0cbec56e6ad052c3cf53a052b254490995453  | SHA1  | iviewers.exe.ui

| 4965f809b71ffb71fe8456d88dcd0a80a99fa6aa4ffd6ba96e1a1d810d41bbd0  | SHA256  | iviewers.exe.ui

| 805fa6261f5fb268e56fd9f11fd13e01  | MD5  | iviewers.exe.ui

| 7dc223a47fa35011d9e5ed8ef0bbeaf7bd08500f  | SHA1  | iviewers.exe.ui

| b5aa86fd97624a317945d110541a07fc80b83dd960fbf16642720fc275d8f04f  | SHA256  | iviewers.exe.ui

| f0293d80323383dcb494b12ceb313105  | MD5  | iviewers.exe.ui

| 0667f44b8dc20d0d1b8f1a5c2fe2f8011204664b  | SHA1  | iviewers.exe.ui

| 9092cdd52109531f9f58c28bda25b0c3f82d9bd2d261ce5fcb0137873dbb0868  | SHA256  | iviewers.exe.ui

| 3e32bcdc16db8baf98065b29faeaa18d  | MD5  | iviewers.exe.ui

| 86a219232410f236665c51854425fbe5e37b07b3  | SHA1  | iviewers.exe.ui

| bcbc3184756a6cacfd5ca2b879708cfd015e84050c9b9ede096cfb70282f870c  | SHA256  | iviewers.exe.ui

| 5312ba28ce0105cf4563279508bf83fa  | MD5  | iviewers.exe.ui

| 3faf065a9987ade102f20dd1ac6b857c7c191b97  | SHA1  | iviewers.exe.ui

| febe116a87860e42bbcfd7c6e2c710446f33bdacc56e990f69493837c01f1059  | SHA256  | iviewers.exe.ui

| d8a1805843925a0394d64d1574b15388  | MD5  | iviewers.exe.ui

| 2b3b897dd7ef6a54bc038a9afc9d79d5989b6c5f  | SHA1  | iviewers.exe.ui

| 7e118a6c4d6f162d8c6a53faf972bd3e675da7f9d0a0b67a1988b4e2102ebb53  | SHA256  | iviewers.exe.ui

| f26edb1d8a61d6bee6da5b5214cce77e  | MD5  | iviewers.exe.ui

| 44e2b73f6f5ec010681cb1fa5681ca0903f0a080  | SHA1  | iviewers.exe.ui

| cc48a02f06066a37c90d063b6d28ae17d9503e4ba6df69aef1b55b5fa5a5ff48  | SHA256  | iviewers.exe.ui

| 9003d7c01c4f2b2e2632a86815eb40a2  | MD5  | iviewers.exe.ui

| ddb59cf25b40273ef0f394c6f164923b6872d7cf  | SHA1  | iviewers.exe.ui

| 562974ea1325a88c916a55719fb9263eb6c710ba281fdee4ba7e9a98a3f4a5a8  | SHA256  | iviewers.exe.ui

| ad9267c5c64390d1ed2d6cfa498b5339  | MD5  | iviewers.dll

| 1f2e03650afbbd10b9cff21116b7b8d9b192cee3  | SHA1  | iviewers.dll

| 92b2535373e55b16b6f3b2d134a1d5545e837d3c19fff4cead4e92558e302b6e  | SHA256  | iviewers.dll

| c5a873b83798a7ad21990eff4c90cb98  | MD5  | iviewers.dll

| 3a5ea30f0ff6928a26c4e67352d0adf44dd978da  | SHA1  | iviewers.dll

| a9556cc05422cae960e36f76eeff7168b8e3cfeb16a20855a93d4f2ed4a65a8b  | SHA256  | iviewers.dll

| 9f128f604a3e57a92381457e6552f886  | MD5  | VELVETSTING (PMCD)

| ef22dfed358bf35f702af4a14f7a646375123e05  | SHA1  | VELVETSTING (PMCD)

| 821d0cdc3e8a735976045ecb1afd1c0170bf39701d2da118b9533a45383a9ebe  | SHA256  | VELVETSTING (PMCD)

| d8958e44fa0499a5fbde99b71207184b  | MD5  | VELVETTAP (MCDP)

| 553674972e59e7b37a63d19556152b13bf785d71  | SHA1  | VELVETTAP (MCDP)

| 436f35dc69bbe7cb8cf5430b52c3aedace099730245de57e004dc1f6531ae262  | SHA256  | VELVETTAP (MCDP)

| 2666a1f1f38ba3bd261c908f14d588c7  | MD5  | ESRDE

| 0e7c4f374009ff3e264d299dfc1c279bff5b6b4b  | SHA1  | ESRDE

| 13f3c05cc348ecb47c4e86d1fb522fdf499a6fb23e0cc6370f4618137f055b04  | SHA256  | ESRDE

| d313dd345d5ea37bc1c431a53d1af91d  | MD5  | SAMRID

| baaa29799bdbb6c1f3fc70e25c0aee4b033fefc8  | SHA1  | SAMRID

| 3d9aaac0a8e5c7eadd79d8d5c16119d04f4e9db7107fc44a1e32a8746a1ec375  | SHA256  | SAMRID

| 202.61.136[.]158  | IP Address  | C&C

| 103.138.13[.]31  | IP Address  | C&C

## Appendix II: MITRE ATT&CK Matrix Mapping

- Initial Access

- T1133 – External Remote Services

- Execution

- T1047- Windows Management Instrumentation
- T1059.008 – Command and Scripting Interpreter: Network Device CLI
- T1569.002 – System Services: Service Execution

- Persistence

- T1037.004 – Boot or Logon Initialization Scripts: RC Scripts
- T1133 – External Remote Services
- T1078.002 – Valid Accounts: Domain Accounts
- T1078.003 – Valid Accounts: Local Accounts

- Privilege Escalation

- T1078.002 – Valid Accounts: Domain Accounts

- Defense Evasion

- T1574.001 – Hijack Execution Flow: DLL Search Order Hijacking
- T1562.004 – Impair Defenses: Disable or Modify System Firewall
- T1055 – Process Injection
- T1070.006 – Indicator Removal: Timestomp
- Masquerading: Match Legitimate Name or Location (T1036.005)

- Credential Access

- T1003.001 – OS Credential Dumping: LSASS Memory

- Discovery

- T1087.002 – Account Discovery: Domain Account
- T1083 – File and Directory Discovery
- T1135 – Network Share Discovery
- T1018 – Remote System Discovery
- T1082 – System Information Discovery
- T1016 – System Network Configuration Discovery
- T1040 – Network Sniffing

- Lateral Movement

- T1021.002 – Remote Services: SMB/Windows Admin Shares
- T1021.004 – Remote Services: SSH
- T1570 – Lateral Tool Transfer

- Collection

- T1039 – Data from Network Shared Drive

- Command and Control

- T1572 – Protocol Tunneling
- T1090.001 – Proxy: Internal Proxy
- T1132.001 – Data Encoding: Standard Encoding
- T1071.001 – Application Layer Protocol: Web Protocols

- Exfiltration

- T1048 – Exfiltration Over Alternative Protocol

If you were impacted by this attack or are seeking guidance on how to prevent similar attacks, please contact us at contact@sygnia.co or our 24-hour hotline +1-877-686-8680.

This advisory and any information or recommendation contained here has been prepared for general informational purposes and is not intended to be used as a substitute for professional consultation on facts and circumstances specific to any entity. While we have made attempts to ensure the information contained herein has been obtained from reliable sources and to perform rigorous analysis, this advisory is based on initial rapid study, and needs to be treated accordingly. Sygnia is not responsible for any errors or omissions, or for the results obtained from the use of this Advisory. This advisory is provided on an as-is basis, and without warranties of any kind.

About the Author

Sygnia Team

APA citation (7th Ed.)

Sygnia Team. (2024, June 3). China-Nexus Threat Group ‘Velvet Ant’ Abuses F5 Load Balancers for Persistence, Sygnia.
https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/

## Related Resources

[Blog

### Investigating Data Extortion Involving Trusted Vendor Access

Learn how sensitive data left an organization through legitimate vendor access, and what the investigation revealed about logging and evidence preservation.](https://www.sygnia.co/blog/investigating-data-extortion-involving-trusted-vendor-access/)

[Webinars

### Cyber Insurance in Practice: What to Know Before an Incident

Join Sygnia’s Tim Smit for a practical client briefing on how to prepare your organization to use its cyber insurance effectively.](https://www.sygnia.co/webinars/cyber-insurance-in-practice-what-to-know-before-an-incident/)

[Blog

### Incident Response Best Practices: Strategies, Tools & Plans

Build incident response best practices into your plan, team, tools, and exercises, using NIST SP 800-61 Rev. 3 and lessons from real investigations.](https://www.sygnia.co/blog/best-practices-incident-response/)

[Webinars

### AI Webinar Series Recordings

Join Sygnia experts for a two-part webinar series examining how AI-related threats are becoming an operational reality for many organizations and the practical approaches security teams should take to protect their business from them.](https://www.sygnia.co/webinars/ai-webinar-series-recordings/)

[Press Release

### Sygnia Unveils Security Operations Center Hubs and Expands AI-Powered Cybersecurity Capabilities To Challenge Threat Attack Rise

Sygnia’s next phase of global growth focuses on AI-driven innovation across MDR, incident response, and proactive security services.](https://www.sygnia.co/press-release/sygnia-expands-global-mdr-operations-austin-singapore-soc-hubs/)

[Webinars

### AI Threat Realities Prompt Proactive Security Readiness

Join Sygnia experts for a two-part webinar series examining how AI-related threats are becoming an operational reality for many organizations and the practical approaches security teams should take to protect their business from them.](https://www.sygnia.co/webinars/ai-threat-realities-prompt-proactive-security-readiness/)

[Blog

### When IT Support Is the Attack: How Law Firms Can Defend Against Silent Ransom Group

Strengthen your readiness against Silent Ransom Group. As SRG increasingly uses IT impersonation and legitimate remote-access tools to target law firms, firms should review how they verify support interactions, protect sensitive client data and detect suspicious activity.](https://www.sygnia.co/blog/when-it-support-is-the-attack-how-law-firms-can-defend-against-silent-ransom-group/)

[Webinars

### When IT Support Is the Attack: Defending Law Firms Against Silent Ransom Group

Join us for practical session on how law firms can defend against Silent Ransom Group and similar threats that turn trusted IT support into a path to sensitive client data.](https://www.sygnia.co/webinars/how-law-firms-can-defend-against-silent-ransom-group/)

[Blog

### Shai-Hulud in the Wild: What Security and Incident Response Teams Need to Know

Learn how Shai-Hulud compromises trusted software workflows, exposes credentials across CI/CD environments, and creates attack paths into cloud, production, and downstream systems, and how security teams can contain and recover from the compromise.](https://www.sygnia.co/blog/shai-hulud-in-the-wild-what-security-and-incident-response-teams-need-to-know/)

[Blog

### Exfiltration in Plain Sight: How the SafePay Ransomware Group Abused OneDrive to Steal Data

Expose how ransomware operators abuse trusted cloud services like OneDrive to bypass traditional security controls, exfiltrate sensitive data over legitimate HTTPS traffic, and hide malicious activity within normal business operations.](https://www.sygnia.co/blog/how-the-safepay-ransomware-group-abused-onedrive-to-steal-data/)

[Press Release

### Sygnia Reveals New Activity by China-Nexus Threat Actor Fire Ant Targeting Trusted Infrastructure

Incident Response leader reveals long-running espionage activity abusing routers, authentication systems and Linux management hosts to collect intelligence and explore paths toward connected high-value environments.](https://www.sygnia.co/press-release/sygnia-reveals-new-activity-by-china-nexus-threat-actor-fire-ant-targeting-trusted-infrastructure/)

[Guides and tools

### How to Secure Enterprise AI: From Adoption to Incident Readiness

Learn where AI risk lives across the enterprise and how to secure AI across its full lifecycle with Sygnia’s practical AI security framework.](https://www.sygnia.co/guides-and-tools/secure-enterprise-ai-incident-readiness-guide/)

[Webinars

### Shai-Hulud in the Wild: What Security and IR Teams Need to Know

What happens when software supply-chain attacks compromise not only packages, but the trusted systems and workflows used to build and distribute them?](https://www.sygnia.co/webinars/shai-hulud-in-the-wild/)

[Blog

### Incident Response vs. Crisis Management: Why Treating Them as the Same Thing Is Dangerous

Incident response contains the attack. Crisis management protects the business. Learn why conflating the two is dangerous and how to align them.](https://www.sygnia.co/blog/incident-response-vs-crisis-management/)

[Press Release

### Sygnia Brings Expert-Led Managed Detection and Response to AWS Marketplace

Company joins the AWS ISV Accelerate Program, making it easier for AWS customers to access Sygnia MDR with bundled incident response support](https://www.sygnia.co/press-release/sygnia-brings-expert-led-managed-detection-and-response-to-aws-marketplace/)

[Blog

### Why Most OT Cyber Incidents Never Reach the OT Environment

Learn why modern industrial operations depend on IT, the misconceptions that leave organizations exposed, and how anti-fragile resilience keeps production running when IT is compromised.](https://www.sygnia.co/blog/why-most-ot-cyber-incidents-never-reach-the-ot-environment/)

[Webinars

### Exfiltration in Plain Sight: SafePay’s OneDrive Play

Get an inside look at the attack path, the forensic evidence that exposed the OneDrive exfiltration technique, and the practical detection opportunities defenders can use to uncover similar activity before it becomes a full-scale extortion event.](https://www.sygnia.co/webinars/when-ransomware-hides-in-onedrive-inside-safepay-exfiltration-play/)

[Blog

### When Technical Controls Work, Attackers Change the Rules: How a Failed Intrusion Became a Data Exposure Incident

Discover how a failed intrusion quickly became a data exposure incident after a threat actor pivoted from blocked technical attacks to social engineering, and why layered defenses, rapid response, and operational safeguards are essential when attackers change the rules.](https://www.sygnia.co/blog/when-technical-controls-work-attackers-change-the-rules/)

[Blog

### Ransomware Incident Response in 2026: What Has Changed and What CISOs Are Still Getting Wrong

Learn what has changed in ransomware incident response in 2026, and what CISOs are still getting wrong](https://www.sygnia.co/blog/ransomware-incident-response/)

[Press Release

### Sygnia Investigation Finds AI Accelerated Attack Enabled Lone Threat Actor to Rapidly Compromise Enterprise Cloud Environment

Financially motivated cyberattack reveals agentic AI workflows to accelerate victim reconnaissance, attack tool development, command structuring and environment-specific adaptation.](https://www.sygnia.co/press-release/sygnia-investigation-into-ai-accelerated-attack/)

[Webinars

### On-Demand Webinar: Inside an AI-Assisted Cloud Attack

Watch a practical session based on Sygnia’s investigation of an AI-assisted cloud intrusion that progressed from initial access to broad compromise in just 72 hours.](https://www.sygnia.co/webinars/inside-an-ai-assisted-cloud-attack-familiar-techniques-at-unfamiliar-speed/)

[Blog

### Inside an AI-Assisted Cloud Attack: Familiar Techniques at Unfamiliar Speed

Learn how a threat actor used AI to accelerate a cloud attack from initial access to broad compromise in just 72 hours](https://www.sygnia.co/blog/inside-an-ai-assisted-cloud-attack/)

[Blog

### Why 73% of Organizations Still Aren’t Ready for a Cyberattack

Explore lessons from Sygnia's 2026 CISO Survey on Incident Response Readiness.](https://www.sygnia.co/blog/incident-response-readiness-ciso-survey/)

[Blog

### Crypto Security in 2026: Why Transaction Authority Is the Most Important Security Control

Most crypto breaches occur when attackers reach transaction authority. Learn how to secure identities, infrastructure, and authorization systems.](https://www.sygnia.co/blog/crypto-security-in-2026/)

[Guides and tools

### 2026 Gartner® Hype Cycle™ for CPS Security

Gain valuable insights into CPS security innovations, threats, and technology evolution.](https://www.sygnia.co/guides-and-tools/2026-gartner-hype-cycle-for-cps-security/)

[Blog

### Velvet Ant’s Operation Highland: How a China-Nexus Actor Infiltrated an Internal Network Undetected

Discover the detailed forensic investigation by Sygnia into the sophisticated cyber attack by Velvet Ant on a major organization.](https://www.sygnia.co/blog/operation-highland-velvet-ant/)

[Blog

### Incident Response Metrics That Actually Matter to Boards (And the Ones That Don’t)

Learn which incident response metrics boards actually need to make decisions, and which ones create false confidence.](https://www.sygnia.co/blog/incident-response-metrics-that-matter/)

[Blog

### 8 Incident Response Plan Mistakes That Lead to Failures

Learn the 8 critical incident response mistakes that leave 73% of organizations unprepared, and how to close the execution gaps that turn manageable cyber incidents into full-scale business crises.](https://www.sygnia.co/blog/incident-response-plan-mistakes/)

[Threat Reports and Advisories

### The Mythos Effect: Preparing for AI-Accelerated Exploitation

How Vulnerability Management Programs Must Evolve](https://www.sygnia.co/threat-reports-and-advisories/mythos-effect-ai-accelerated-exploitation-vulnerability-management/)

[Guides and tools

### CISO Survey 2026: The State of Incident Response Readiness

Benchmark your readiness against 600 peers and gain data-driven insights into how ready organizations truly are to manage a significant cyberattack.](https://www.sygnia.co/guides-and-tools/ciso-survey-2026/)

[Blog

### How to Leverage an Incident Response Retainer

Discover how to get the most from an incident response retainer, and what sets apart a provider that can respond effectively when your organization is under real pressure.](https://www.sygnia.co/blog/why-incident-response-retainers-fail/)

[Blog

### Exfiltration in Plain Sight: SafePay’s OneDrive Play

Discover how SafePay’s rapid, high-pressure ransomware campaigns unfold, and what Sygnia learned while investigating a real-world double extortion attack that put a global organization to the test.](https://www.sygnia.co/blog/safepay-onedrive-exfiltration-technique/)

[Blog

### Incident Response in the AI Threat Era: What Boards Must Re-Evaluate in 2026

Explore how AI-driven cyber threats are reshaping board-level risk decisions and what executives must re-evaluate for incident response in 2026.](https://www.sygnia.co/blog/incident-response-ai-threat-era/)

[Blog

### What 2025’s Major Cyber Incidents Taught Executives About Incident Response Readiness

Discover what Threat Detection, Investigation & Response (TDIR) means for modern enterprises, including frameworks, risks, and executive-level decision insights.](https://www.sygnia.co/blog/2025-cyber-incident-incident-response-lessons/)

[Datasheet

### Incident Response and SOC Training Services

Sygnia offers a suite of services that are designed and proven to enhance cloud cyber resilience.](https://bunny-wp-pullzone-5pkz60xvv9.b-cdn.net/wp-content/uploads/2026/07/Sygnia-IR-and-SOC-Training-Services_Datasheet_Final.pdf)

[Datasheet

### Pathfinder

Sygnia offers a suite of services that are designed and proven to enhance cloud cyber resilience.](https://bunny-wp-pullzone-5pkz60xvv9.b-cdn.net/wp-content/uploads/2026/07/Pathfinder-Datasheet-032025.pdf)

[Datasheet

### Incident Response Services

Sygnia offers a suite of services that are designed and proven to enhance cloud cyber resilience.](https://bunny-wp-pullzone-5pkz60xvv9.b-cdn.net/wp-content/uploads/2026/07/Incident-Response-Service-Datasheet.pdf)

[Datasheet

### Incident Response and Retainer Services

Sygnia offers a suite of services that are designed and proven to enhance cloud cyber resilience.](https://bunny-wp-pullzone-5pkz60xvv9.b-cdn.net/wp-content/uploads/2026/07/Sygnia-Incident-Response-and-Retainer-Services_Solution-Brief_Aug-2025.pdf)

[Blog

### Incident Response Strategies for Technology and SaaS Companies

Executive-level incident response strategies for Technology and SaaS companies to strengthen resilience, reduce business impact, and accelerate recovery.](https://www.sygnia.co/blog/saas-incident-response/)

[Blog

### Developing Incident Response Team Communication and Coordination Practices

Master incident response communication with proven frameworks that align teams, accelerate decisions, and minimize business impact during cyber crises.](https://www.sygnia.co/blog/incident-response-communication/)

[Blog

### Creating Incident Response Policies and Procedures: Best Practices

Learn how to create effective incident response policies and procedures with best practices that strengthen resilience, compliance, and recovery.](https://www.sygnia.co/blog/incident-response-policies/)

[Blog

### A Tactical Approach to Incident Response: Navigating the Aftermath of a Cyber Attack

When a cyber attack strikes, every second counts. At Sygnia, we deploy a tactical, multi-stream approach to incident response—prioritizing rapid containment, eradication, and operational resilience while conducting deep investigations. Learn how parallel execution ensures faster recovery and stronger security outcomes.](https://www.sygnia.co/blog/tactical-incident-response-cyber-attack/)

[Blog

### Weaver Ant, the Web Shell Whisperer: Tracking a Live China-nexus Operation

Sygnia details Weaver Ant, a China-nexus threat actor infiltrating a major telecom provider. Using web shells and tunneling, the attackers maintained persistence and facilitated cyber espionage. This blog explores their tactics and provides key defense strategies against state-sponsored threats.](https://www.sygnia.co/threat-reports-and-advisories/weaver-ant-tracking-a-china-nexus-cyber-espionage-operation/)

[Blog

### What is an Incident Response Retainer, Key Features and Benefits, and Why It Matters

Learn about incident response retainers, their key features, benefits, and why they are essential for protecting your organization from cyber threats.](https://www.sygnia.co/blog/what-is-incident-response-retainer/)

[Blog

### The Anatomy of Abyss Locker Ransomware Attack

Abyss Locker ransomware targets critical network devices with swift, disruptive attacks. This blog breaks down its tactics and defense strategies.](https://www.sygnia.co/blog/abyss-locker-ransomware-attack-analysis/)

[Blog

### How to Run Incident Response Tabletop Exercises in 2026

Discover how to prepare your organization for cyber threats in 2026 with incident response tabletop exercises. Learn strategies to strengthen your response plans.](https://www.sygnia.co/blog/incident-response-tabletop-exercise/)

[Blog

### CrowdStrike Fallout: Navigating the Risks of Intrusive Security Tools

Learn how to balance robust cybersecurity with operational stability in the wake of the CrowdStrike outage.](https://www.sygnia.co/blog/crowdstrike-outage-security-vs-operational-stability/)

[Blog

### China-Nexus Threat Group ‘Velvet Ant’ Leverages a Zero-Day to Deploy Malware on Cisco Nexus Switches

Sygnia uncovers the China-Nexus group ‘Velvet Ant’ leveraging a zero-day exploit (CVE-2024-20399) on Cisco Switch appliances, escalating evasion tactics to maintain long-term network persistence.](https://www.sygnia.co/blog/china-threat-group-velvet-ant-cisco-zero-day/)

[Blog

### What is a Rootkit? Exploring the Hidden Threats and Their Impact on System Security

Learn what Rootkits are, how sophisticated threat actors leverage them to gain full control of computers and systems, and how to protect your systems from this unique risk.](https://www.sygnia.co/blog/rootkit-malware/)

[Guides and tools

### Incident Response Readiness: What is it and how to improve it?

The most dangerous cyber attackers have something in common: they are tenacious at their craft and dogged in their determination to find and exploit security weaknesses.](https://www.sygnia.co/guides-and-tools/incident-response-readiness/)

[Blog

### The Return of Ghost Emperor’s Demodex

A Comprehensive Look at the Updated Infection Chain of Ghost Emperor's Demodex Rootkit.](https://www.sygnia.co/blog/ghost-emperor-demodex-rootkit/)

[Blog

### Sygnia Recognized for the Third Consecutive Year in The Gartner 2024 Market Guide for Digital Forensics and Incident Response Retainer Services

In June, Sygnia, was named a Representative Vendor in the 2024 Gartner® Market Guide for Digital Forensics and Incident Response Retainer Services (DFIR).](https://www.sygnia.co/blog/sygnia-recognized-gartner-2024-market-guide-dfir-services/)

[Blog

### China-Nexus Threat Group ‘Velvet Ant’ Exploits Cisco Zero-Day (CVE-2024-20399) to Compromise Nexus Switch Devices – Advisory for Mitigation and Response

Learn about the forensic investigation by Sygnia, the cyber espionage operation by Velvet Ant, and best practices for safeguarding your network against sophisticated threats.](https://www.sygnia.co/threat-reports-and-advisories/china-nexus-threat-group-velvet-ant-exploits-cisco-0-day/)

[Blog

### Navigating the Digital Minefield

Part 1: A Thrilling Guide to Mastering Cyber Crisis Management. Explore real-world cyber crises reimagined as high-stakes dramas, from WannaCry to Target's breach.](https://www.sygnia.co/blog/unveiling-cyber-crisis-strategies/)

[Blog

### Chapter 3 – Reversing DoSvc C++ using Ghidra – Part 1

In this chapter, we’ll finally get started on my favorite part: the reversing phase. Reversing is like looking for a needle in a haystack, so prepare yourselves, because things are about to get technical.](https://www.sygnia.co/blog/reversing-dosvc-c-using-ghidra-part-1/)

[Blog

### Chapter 2 – Black Box Research

The intricacies of WUDO (Windows Update Delivery Optimization).](https://www.sygnia.co/blog/chapter-2-black-box-research/)

[Blog

### The Anatomy of a BlackCat (ALPHV) Attack

Explore the thwarted cyber extortion attempt by the BlackCat ransomware group, unraveled by Sygnia's Incident Response team in 2023.](https://www.sygnia.co/blog/blackcat-ransomware/)

[Blog

### Proof of Concept: Overcoming Google Cloud Incident Response Issues with ‘Mirage’

How the Mirage tool developed by Sygnia assists in evidence collection & investigation, while uncovering key log analysis & incident response strategies for Google Cloud.](https://www.sygnia.co/blog/overcoming-google-cloud-incident-response-issues-with-mirage/)

[Blog

### Announcing ‘Mirage’ – New Opensource Tool to Combat Google Cloud Incident Response Challenges

New open-source tool streamlines incident response in Google Workspace & GCP. Overcome challenges, access data, & gain insights faster. Learn more about Mirage & Google Cloud forensics in our blog series.](https://www.sygnia.co/blog/new-opensource-tool-to-combat-google-cloud-incident-response-challenges/)

[Blog

### The SEC’s SolarWinds investigation in Context: Lessons from 100 Enterprise Security Assessments Governance & Compliance

SEC charges SolarWinds & CISO with misleading statements about security practices. Learn how SolarWinds compares to other companies & what you can do to improve your security posture.](https://www.sygnia.co/blog/the-secs-solarwinds-investigation/)

[Guides and tools

### Building Cyber Readiness: 5 Traits of the World’s Most Cyber-Ready Organizations

The most dangerous cyber attackers have something in common: they are tenacious at their craft and dogged in their determination to find and exploit security weaknesses.](https://www.sygnia.co/guides-and-tools/5-essential-traits-of-cyber-ready-organizations/)

[Blog

### Sygnia Annual Field Report

From relentless adversaries to resilient businesses, the Sygnia Annual Field Report offers unique insights and strategic guidance to strengthen your cybersecurity defenses against the ever-evolving threat landscape.](https://www.sygnia.co/blog/annual-field-report/)

[Threat Reports and Advisories

### Mitigation Advisory: Fortinet and Ivanti Exploitable Vulnerabilities

Protect your network against critical remote code execution vulnerabilities in Fortinet and Ivanti (formerly Pulse Secure) devices, now actively exploited in the wild.](https://www.sygnia.co/threat-reports-and-advisories/fortinet-and-ivanti-pulse-secure-exploitable-vulnerabilities/)

[Blog

### Why monitoring monitors is the key to cyber threat resilience

Protect your organization by monitoring the monitors – a critical defense against cyber threats.](https://www.sygnia.co/blog/why-monitoring-monitors-is-the-key-to-cyber-threat-resilience/)

[Blog

### Diving into the new windows 11 PCA artifact

Discover the new forensic artifact in Windows 11 (22H2) and explore its format and behavior. Learn how to leverage it in investigations with our comprehensive documentation for DFIR practitioners.](https://www.sygnia.co/blog/new-windows-11-pca-artifact/)

[Blog

### Breaking Down the Casbaneiro Infection Chain – Part II

Since 2018, the Casbaneiro banking trojan has targeted Latin American financial sectors. Sygnia monitors it closely as a major threat to multi-regional financial organizations.](https://www.sygnia.co/blog/breaking-down-casbaneiro-infection-chain-part2/)

[Blog

### Case Study: cracking a global Adversary-In-The-Middle campaign using a threat intelligence toolkit

Using a variety of CTI enrichment techniques Sygnia was able to determine that the BEC attack was only a part of a much wider campaign, potentially impacting dozens of world-wide organizations.](https://www.sygnia.co/guides-and-tools/cracking-global-phishing-campaign-using-threat-intelligence-toolkit/)

[Blog

### Threat Actor Spotlight: RagnarLocker Ransomware

Explore the threat of RagnarLocker ransomware. Uncover its tactics and implications in this Sygnia blog spotlight on cyber threats.](https://www.sygnia.co/blog/threat-actor-spotlight-ragnarlocker-ransomware/)

[Blog

### Incident Response in Google Cloud: Forensic Artifacts

Discover effective incident response in Google Cloud. Learn how to analyze forensic artifacts for swift resolution. Expert insights on Sygnia blog.](https://www.sygnia.co/blog/gcp-incident-response/)

[Blog

### Incident Response in Google Cloud: Foundations

Incident response and threat hunting in Google Cloud Platform and Workspace](https://www.sygnia.co/blog/incident-response-in-google-cloud-foundations/)

[Threat Reports and Advisories

### Revealing Emperor Dragonfly: Night Sky and Cheerscrypt – A Single Ransomware Group

The outcome of investigation by Sygnia's incident response teams - Cheerscrypt & Night Sky are rebrands of one ransomware group, named ‘Emperor Dragonfly’.](https://www.sygnia.co/threat-reports-and-advisories/revealing-emperor-dragonfly-a-chinese-ransomware-group/)

[Blog

### Vice Society Ransomware Group: Threats to Health & Education Sector

Professionals across all industries, agree that the Vice Society ransomware group is a major cyber threat, especially in health & education sector](https://www.sygnia.co/blog/vice-society-ransomware-group/)

[Blog

### Vice Society TTPs: Insights from a Real-World Ransomware Investigation

Get technical insights based on forensic investigations of Vice Society (ransomware group carrying out double extortion attacks) conducted by our IR team.](https://www.sygnia.co/blog/the-vice-society-ransomware-investigation/)

[Blog

### Luna Moth Ransomware: The Threat Actors Behind Recent False Subscription Scams

Sygnia’s team identified 'Luna Moth' ransom group. The threat actors resemble false subscription scammers, focusing on corporate data theft.](https://www.sygnia.co/blog/luna-moth-false-subscription-scams/)

[Blog

### Hybrid Phishing Attack Vector – Complementing Phishing Campaigns

Cyber-crime groups are targeting Java-based infrastructures susceptible to the Log4Shell vulnerability and are delivering Qakbot phishing emails](https://www.sygnia.co/blog/phishing-integrity-qakbot/)

[Blog

### The Misconceptions of 2021’s Black Swan Cyber Events

Here are 3 common misconceptions about Black swan events that are considered incidents with high impact and low frequency that are impossible to predict.](https://www.sygnia.co/blog/the-misconceptions-of-2021s-black-swan-cyber-events/)

[Threat Reports and Advisories

### Key Takeaways Leak of Conti crime group information

Shared thoughts and insights following our analysis of the leaked Conti (cyber crime group) information and the groups modus operandi.](https://www.sygnia.co/threat-reports-and-advisories/key-takeaways-conti-group-leak/)

[Blog

### Breaking Down the Casbaneiro Infection Chain

The Casbaneiro banking trojan targets financial organizations to steal user data for financial gain. Get a detailed “attacker fingerprint”.](https://www.sygnia.co/blog/breaking-down-casbaneiro-infection-chain/)

[Threat Reports and Advisories

### Potential Okta Breach

In light of evidence of a successful Okta breach, we recommend taking several steps to mitigate potential risk for organizations leveraging Okta solutions.](https://www.sygnia.co/threat-reports-and-advisories/okta-breach-advisory/)

[Threat Reports and Advisories

### Russia–Ukraine Conflict Escalation

An advisory to keep leadership updated & support global organizations enhance cyber resilience as we monitor the conflict between Russia & Ukraine.](https://www.sygnia.co/threat-reports-and-advisories/russia-ukraine-conflict-escalation-advisory/)

[Blog

### It’s a Threat Actor’s Paradise: Getting Ahead of Attackers in 2022

Achieving cyber resilience in the hostile attack terrain comes down to an organization’s ability to flip the asymmetry between them and the threat actors.](https://www.sygnia.co/blog/a-hackers-paradise-what-an-amazing-time-to-be-a-threat-actor/)

[Threat Reports and Advisories

### Log4Shell Remote Code Execution

Critical remote code execution vulnerability in a software logging package called “Log4Shell” (CVE-2021-44228) is impacting millions of devices globally](https://www.sygnia.co/threat-reports-and-advisories/log4shell-remote-code-execution-advisory/)

[Blog

### Recent Waves of Phishing Attacks Overpowering 2-factor Authentication

Phishing attacks are still a leading attack vector for threat actors globally, & are evolving in prevalence & sophistication given work-from-home models](https://www.sygnia.co/blog/recent-waves-of-phishing-attacks-overpowering-2fa-authentication/)

[Threat Reports and Advisories

### Kaseya Ransomware Supply Chain Attack

Get a deep dive into the Kaseya ransomware attack, and how you can deploy effective defense strategies.](https://www.sygnia.co/threat-reports-and-advisories/kaseya-ransomware-supply-chain-attack/)

[Threat Reports and Advisories

### Detection And Hunting Of Golden SAML Attack

Get actionable insights on how to detect a Golden SAML attack vector and an overview of the compromise at SolarWinds.](https://www.sygnia.co/threat-reports-and-advisories/golden-saml-attack/)

[Threat Reports and Advisories

### Demystifying The PrintNightmare Vulnerability

Learn about the PrintNightmare vulnerability, why it's such a cause for concern and how to mitigate the risk.](https://www.sygnia.co/threat-reports-and-advisories/demystifying-the-print-nightmare-vulnerability/)

Want to get in touch?

[Contact us](https://www.sygnia.co/contact-us/)

Newsletter signup

By clicking Subscribe, I agree to the use of my personal data in accordance with Sygnia [Privacy Policy](https://www.sygnia.co/privacy/). Sygnia will not sell, trade, lease, or rent your personal data to third parties.
