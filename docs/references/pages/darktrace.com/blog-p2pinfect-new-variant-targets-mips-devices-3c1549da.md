---
url: "https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices"
title: "P2Pinfect - New Variant Targets MIPS Devices"
captured: "2026-11-02T18:52:22Z"
via: "live"
sha256: "3b1253d093101b5d59953d0a61e2ef21b58790815f2a66e195b2558037bd2242"
wayback: "https://web.archive.org/web/20260413180108/https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# P2Pinfect - New Variant Targets MIPS Devices

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices>
> Snapshot: <https://web.archive.org/web/20260413180108/https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices>

[Blog](https://www.darktrace.com/blog)

/

[Cloud](https://www.darktrace.com/primary-topics/cloud)

/

April 12, 2023

# P2Pinfect - New Variant Targets MIPS Devices

A new P2Pinfect variant compiled for the Microprocessor without Interlocked Pipelined Stages (MIPS) architecture has been discovered. This demonstrates increased targeting of routers, Internet of Things (IoT) and other embedded devices by those behind P2Pinfect.

[Written by

The Darktrace Community](https://www.darktrace.com/people/the-darktrace-community)

Inside the SOC

Darktrace cyber analysts are world-class experts in threat intelligence, threat hunting and incident response, and provide 24/7 SOC support to thousands of Darktrace customers around the globe. Inside the SOC is exclusively authored by these experts, providing analysis of cyber incidents and threat trends, based on real-world experience in the field.

Written by

The Darktrace Community

Share

https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices#https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices#https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices#

12

Apr 2023

## Introduction: P2PInfect

Since July 2023, researchers at Cado Security Labs (now part of Darktrace) have been monitoring and reporting on the rapid growth of a cross-platform botnet, named “P2Pinfect”. As the name suggests, the malware - written in Rust - acts as a botnet agent, connecting infected hosts in a peer-to-peer topology. In early samples, the malware exploited Redis for initial access - a relatively common technique in cloud environments.

There are a number of methods for exploiting Redis servers, several of which appear to be utilized by P2Pinfect. These include exploitation of [CVE-2022-0543](https://nvd.nist.gov/vuln/detail/CVE-2022-0543)[1] - a sandbox escape vulnerability in the LUA scripting language (reported by [Unit42](https://unit42.paloaltonetworks.com/peer-to-peer-worm-p2pinfect/) [2]), and, as reported previously by Cado Security Labs, an unauthorized replication attack resulting in the loading of a malicious Redis module.

Researchers have since encountered a new variant of the malware, specifically targeting embedded devices based on 32-bit MIPS processors, and attempting to brute force SSH access to these devices. It’s highly likely that by targeting MIPS, the P2Pinfect developers intend to infect routers and IoT devices with the malware. Use of MIPS processors is common for embedded devices and the architecture has been previously targeted by botnet malware, including high-profile families like [Mirai](https://unit42.paloaltonetworks.com/mirai-variant-iz1h9/)[3], and its variants/derivatives.

Not only is this an interesting development in that it demonstrates a widening of scope for the developers behind P2Pinfect (more supported processor architectures equals more nodes in the botnet itself), but the MIPS32 sample includes some notable defense evasion techniques.

This, combined with the malware’s utilization of Rust (aiding cross-platform development) and rapid growth of the botnet itself, reinforces previous suggestions that this campaign is being conducted by a sophisticated threat actor.

## Initial access

Cado researchers encountered the MIPS variant of P2Pinfect after triaging files uploaded via SFTP and SCP to a SSH honeypot. Although earlier variants had been observed scanning for SSH servers, and attempting to propagate the malware via SSH as part of its worming procedure, researchers had yet to observe successful implantation of a P2Pinfect sample using this method - until now.

In keeping with similar botnet families, P2Pinfect includes a number of common username/password pairs embedded within the MIPS binary itself. The malware will then iterate through these pairs, initiating a SSH connection with servers identified during the scanning phase to conduct a brute force attack.

It was assumed that SSH would be the primary method of propagation for the MIPS variant, due to routers and other embedded devices being more likely to utilize SSH. However, additional research shows that it is in fact possible to run the Redis server on MIPS. This is achievable via an OpenWRT package named [redis-server.](https://openwrt.org/packages/pkgdata/redis-server) [4]

It is unclear what use-case running Redis on an embedded MIPS device solves, or whether it is commonly encountered in the wild. If such a device is compromised by P2Pinfect and has the Redis-server package installed, it is perfectly feasible for that node to then be used to compromise new peers via one of the reported P2Pinfect attack patterns, involving exploitation of Redis or SSH brute-forcing.

## Static analysis

The MIPS variant of P2Pinfect is a 32-bit, statically-linked, ELF binary with stripped debug information. Basic static analysis revealed the presence of an additional ELF executable, along with a 32-bit Windows DLL in the PE32 format - more on this later.

This piqued the interest of Cado analysts, as it is unusual to encounter a compiled ELF with an embedded DLL. Consequently, it was a defining feature of the original P2Pinfect samples.

Figure 1: Embedded Windows PE32 executable

Further analysis of the host executable revealed a structure named “BotnetConf” with members consistent in naming with the original P2Pinfect samples.

Figure 2: Example of a partially populated version of the BotnetConf struct

As the name suggests, this structure defines the configuration of the malware itself, whilst also storing the IP addresses of nodes identified during the SSH and Redis scans. This, in combination with the embedded ELF and DLL, along with the use of the Rust programming language allowed for positive attribution of this sample to the P2Pinfect family.

## Updated evasion - consulting tracerpid

One of the more interesting aspects of the MIPS sample was the inclusion of a new evasion technique. Shortly after execution, the sample calls fork() to spawn a child process.

The child process then proceeds to access /proc using openat(), determines its own Process Identifier (PID) using the Linux getpid() syscall, and then uses this PID to consult the relevant /proc subdirectory and read the status file within that. Note that this is likely achieved in the source code by resolving the symbolic link at /proc/self/status.

‍

Figure 3: Example contents of /proc/pid/status when process not being traced

/proc/<pid>/status contains human-readable metadata and other information about the process itself, including memory usage and the name of the command currently being run. Importantly, the status file also contains a field TracerPID:. This field is assigned a value of 0 if the current process is not being traced by dynamic analysis tools, such as strace and ltrace.

Figure 4: Example MIPS disassembly showing reading of /proc/pid/status file

If this value is non-zero, the MIPS variant of P2Pinfect determines that it is being analyzed and will immediately terminate both the child process and its parent.

```
read(5, "Name:\tmips_embedded_p\nUmask:\t002", 32) = 32
read(5, "2\nState:\tR (running)\nTgid:\t975\nN", 32) = 32
read(5, "gid:\t0\nPid:\t975\nPPid:\t1\nTracerPid:\t971\nUid:\t0\t0\t0\t0\nGid:\t0\t0\t0\t0", 64) = 64
read(5, "\nFDSize:\t32\nGroups:\t0 \nNStgid:\t975\nNSpid:\t975\nNSpgid:\t975\nNSsid:\t975\nVmPeak:\t    3200 kB\nVmSize:\t    3192 kB\nVmLck:\t       0 kB\n", 128) = 128
read(5, "VmPin:\t       0 kB\nVmHWM:\t    1564 kB\nVmRSS:\t    1560 kB\nRssAnon:\t      60 kB\nRssFile:\t    1500 kB\nRssShmem:\t       0 kB\nVmData:\t     108 kB\nVmStk:\t     132 kB\nVmExe:\t    2932 kB\nVmLib:\t       8 kB\nVmPTE:\t      16 kB\nVmSwap:\t       0 kB\nCoreDumping:\t0\nThre", 256) = 256
mmap2(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x77ff1000
read(5, "ads:\t1\nSigQ:\t0/1749\nSigPnd:\t00000000000000000000000000000000\nShdPnd:\t00000000000000000000000000000000\nSigBlk:\t00000000000000000000000000000000\nSigIgn:\t00000000000000000000000000001000\nSigCgt:\t00000000000000000000000000000600\nCapInh:\t0000000000000000\nCapPrm:\t0000003fffffffff\nCapEff:\t0000003fffffffff\nCapBnd:\t0000003fffffffff\nCapAmb:\t0000000000000000\nNoNewPrivs:\t0\nSeccomp:\t0\nSpeculation_Store_Bypass:\tunknown\nCpus_allowed:\t1\nCpus_allowed_list:\t0\nMems_allowed:\t1\nMems_allowed_list:\t0\nvoluntary_ctxt_switches:\t92\nn", 512) = 512
mmap2(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x77fef000
munmap(0x77ff1000, 4096)                = 0
read(5, "onvoluntary_ctxt_switches:\t0\n", 1024) = 29
read(5, "", 995)                        = 0
close(5)                                = 0
munmap(0x77fef000, 8192)                = 0
sigaltstack({ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=8192}, NULL) = 0
munmap(0x77ff4000, 12288)               = 0
exit_group(-101)                        = ?
+++ exited with 155 +++
```

Strace output demonstrating TracerPid evasion technique

## Updated evasion - disabling core dumps

Interestingly, the sample will also attempt to disable Linux core dumps. This is likely used as an anti-forensics procedure as the memory regions written to disk as part of the core dump can often contain internal information about the malware itself. In the case of P2Pinfect, this would likely include information such as IP addresses of connected peers and the populated BotnetConf structure mentioned previously.

It is also possible that the sample prevents core dumps from being created to protect the availability of the MIPS device itself. Low-powered embedded devices are unlikely to have much local storage available and core dumps could quickly fill what little storage they do have, affecting performance of the device itself.

Image 5

This procedure can be observed during dynamic analysis, with the binary utilising the prctl() syscall and passing the parameters PR_SET_DUMPABLE, SUID_DUMP_DISABLE.

```
munmap(0x77ff1000, 4096)                = 0
prctl(PR_SET_DUMPABLE, SUID_DUMP_DISABLE) = 0
prlimit64(0, RLIMIT_CORE, {rlim_cur=0, rlim_max=0}, NULL) = 0
```

Example strace output demonstrating disabling of core dumps

## Embedded DLL

As mentioned in the Static Analysis section, the MIPS variant of P2Pinfect includes an embedded 64-bit Windows DLL. This DLL acts as a malicious loadable module for Redis, implementing the system.exec functionality to allow the running of shell commands on a compromised host.

Figure 6: Disassembly of the Redis module entrypoint, mapping the system.exec command to a handler

This is consistent with the previous examples of P2Pinfect, and demonstrates that the intention is to utilize MIPS devices for the Redis-specific initial access attack patterns mentioned throughout this blog.

Interestingly, this embedded DLL also includes a Virtual Machine (VM) evasion function, demonstrating the lengths that the P2Pinfect developers have taken to hinder the analysis process. In the DLLs main function, a call can be observed to a function helpfully labelled anti_vm by IDAs Lumina feature.

Figure 7: Decompiler output showing call to anti_vm function

Viewing the function itself, it can be seen that researchers Christopher Gardner and Moritz Raabe have identified it as a known VM evasion method in other malware samples.

Figure 8: IDA’s graph view for the anti_vm function showing Lumina annotations

## Conclusion

P2Pinfect’s continued evolution and broadened targeting appear to be the utilization of a variety of evasion techniques demonstrate an above-average level of sophistication when it comes to malware development. This is a botnet that will continue to grow until it’s properly utilized by its operators.

While much of the functionality of the MIPS variant is consistent with the previous variants of this malware, the developer’s efforts in making both the host and embedded executables as evasive as possible show a continued commitment to complicating the analysis procedure. The use of anti-forensics measures such as the disabling of core dumps on Linux systems also supports this.

## Indicators of compromise (IoCs)

### Files SHA256

MIPS ELF 8b704d6334e59475a578d627ae4bcb9c1d6987635089790350c92eafc28f5a6c

Embedded DLL Redis Module d75d2c560126080f138b9c78ac1038ff2e7147d156d1728541501bc801b6662f

### References:

[1] [https://nvd.nist.gov/vuln/detail/CVE-2022-0543](https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices#)

[2] [https://unit42.paloaltonetworks.com/peer-to-peer-worm-p2pinfect/](https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices#)

[3] [https://unit42.paloaltonetworks.com/mirai-variant-iz1h9/](https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices#)

[4] [https://openwrt.org/packages/pkgdata/redis-server](https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices#)

###

https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices#

[Written by

The Darktrace Community](https://www.darktrace.com/people/the-darktrace-community)

Inside the SOC

Darktrace cyber analysts are world-class experts in threat intelligence, threat hunting and incident response, and provide 24/7 SOC support to thousands of Darktrace customers around the globe. Inside the SOC is exclusively authored by these experts, providing analysis of cyber incidents and threat trends, based on real-world experience in the field.

Written by

The Darktrace Community

Share this post

https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices#https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices#https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices#

Latest blogs

[### Beyond the Fridge: Exposed Industrial Control Systems

OT

•

October 21, 2026

Tara Gould

Malware Research Lead](https://www.darktrace.com/blog/beyond-the-fridge-exposed-industrial-control-systems)

[### PHP Vulnerabilities, ARTEX and the Growing Role of AI in Cyber Attacks

AI

•

October 22, 2026

Nathaniel Jones

SVP, Global Threat Intelligence](https://www.darktrace.com/blog/vulnerability-discovery-to-active-attacks-ai-in-cyber-operations)

[Watch the NIS2 Webinar](https://darktrace.com/resources/nis2-directive-implications-on-cyber-security-and-ai)

Newsletter

# Enjoying the blog?

Sign up to receive the latest news and insights from the Darktrace newsletter – delivered directly to your inbox

Thanks for signing up!

Look out for your first newsletter, coming soon.

Oops! Something went wrong while submitting the form.

Trending blogs

[1

Prompt Security in Enterprise AI: Strengths, Weaknesses, and Common Approaches

May 20, 2026](https://www.darktrace.com/blog/prompt-security-in-enterprise-ai-strengths-weaknesses-and-common-approaches)

[2

What the Darktrace Annual Threat Report 2026 Means for Security Leaders

Feb 26, 2026](https://www.darktrace.com/blog/what-the-darktrace-annual-threat-report-2026-means-for-security-leaders)

[3

Journey of a Threat: How Multi-Layered AI Works in Darktrace / EMAIL

May 26, 2026](https://www.darktrace.com/blog/how-multi-layered-ai-works-in-darktrace-email)

[4

How to Evaluate AI Vendors: 5 Key categories for AI Adoption

May 27, 2026](https://www.darktrace.com/blog/how-to-evaluate-ai-vendors-5-key-categories-for-ai-adoption)

[5

Darktrace Unites Human Behavior and Threat Detection Across Email, Slack, Teams, and Zoom

Mar 24, 2026](https://www.darktrace.com/blog/darktrace-unites-human-behavior-and-threat-detection-across-email-slack-teams-and-zoom)

## More in this series

No items found.

Continue reading

[Cloud](https://www.darktrace.com/primary-topics/cloud)

•

July 6, 2026

### Why Hybrid Environments Need Unified Visibility

Despite the growing popularity of hybrid environments, most organizations face challenges in achieving unified visibility between on-premises and cloud networks. AI-powered platform tools can bridge this gap in visibility to reduce detection and response times and simplify operations.

Pallavi Singh

Senior Product Marketing Manager

[Read more](https://www.darktrace.com/blog/breaking-silos-why-unified-security-is-critical-in-hybrid-world)

[Cloud](https://www.darktrace.com/primary-topics/cloud)

•

June 15, 2026

### Forensics or Fauxrensics: Five Core Capabilities for Cloud Forensics and Incident Response

This blog covers the five core capabilities that security teams should consider when evaluating a cloud forensics and incident response solution.

Calum Hall

Technical Content Researcher

[Read more](https://www.darktrace.com/blog/forensics-or-fauxrensics-five-core-capabilities-for-cloud-forensics-and-incident-response)

[Cloud](https://www.darktrace.com/primary-topics/cloud)

•

April 9, 2026

### Bringing Together SOC and IR teams with Automated Threat Investigations for the Hybrid World

Incident response often breaks down after detection due to fragmented tools and limited visibility. Darktrace unifies detection and investigation across cloud and on-premises environments, enabling automated evidence capture and faster, clearer response.

Paul Bottomley

Director of Product Management | Darktrace

[Read more](https://www.darktrace.com/blog/bringing-together-soc-and-ir-teams-with-automated-threat-investigations-for-the-hybrid-world)

## Blog

/

## AI

/

## November 2, 2026

## PHP Vulnerabilities, ARTEX and the Growing Role of AI in Cyber Attacks

The disclosure of CVE-2026-109618 and CVE-2026-109619 placed immediate pressure on organizations running PHP. Both vulnerabilities can allow unauthenticated remote code execution, including in products and appliances where PHP is embedded, and updates depend on third-party vendors [1].

Early guidance focused on patching internet-facing systems and monitoring for behavior associated with successful exploitation, including unexpected command execution, Webroot changes and outbound connections from PHP servers.

Darktrace observed widespread scanning across monitored environments involving hundreds of distinct source IP addresses that public reporting has associated with activity targeting the PHP vulnerabilities. The volume of scanning was significantly greater than the evidence available for successful exploitation. For analysts, that distinction is important. Scanning demonstrates interest in exposed infrastructure; establishing compromise requires evidence of subsequent behavior on the targeted system, including command execution, file modification or unexpected outbound communication.

PHP is not an isolated example. Imperva subsequently demonstrated remote code execution against a vulnerable Node.js build through malicious Brotli data, reproducing execution through compressed HTTP responses and configured request parsers. The same inputs were rejected by the patched runtime. Imperva linked its Node2Shell and PHP2Shell research to the increasing speed of vulnerability discovery and exploit development. [2]

While Darktrace has not observed direct evidence of CVE-2026-109618 or CVE-2026-109619 exploitation, on October 21st we observed a six-month high in detections consistent with zero-day exploitation activity, representing a 64% increase compared to historic data. This increase has not been confirmed as related to the PHP vulnerabilities. Darktrace’s behavioral approach is designed to detect anomalous activity without relying solely on rules written for a specific exploit. Comparable increases were observed around the Log4j disclosure and other globally significant vulnerability disclosures.

### AI-assisted vulnerability research

The PHP disclosure arrives as AI systems have become more capable at vulnerability research [3]. OpenAI launched Daybreak in June 2026, combining GPT-5.5-Cyber, Codex Security and partnerships with security researchers and software maintainers. OpenAI says its models have been applied to discovering and generating patches for vulnerabilities in browsers, network infrastructure, FreeBSD and the Linux kernel. Darktrace joined the OpenAI Daybreak Cyber Partner Program in June to explore how OpenAI's cyber capabilities could be integrated within Darktrace products and services [4][10].

OpenAI reported in August that Codex Security Cloud had analyzed more than 30 million commits across more than 30,000 codebases. According to OpenAI, Cloudflare uses models accessed through Daybreak to investigate customer-authorized code and combines findings with live traffic and security signals to assess whether affected routes are active or receiving suspicious traffic, with proposed mitigations and patches remaining subject to customer review [5][6].

Faster discovery still leaves defenders with the work of locating affected software, testing fixes and completing remediation. The PHP case shows that gap clearly, particularly when vulnerable runtimes are embedded inside other products.

### Enterprise agents as an attack surface

Darktrace Signal Labs has also examined how a trusted enterprise agent could be abused after an attacker has already obtained access to an environment. In a controlled simulation, researchers demonstrated that, in that post-compromise scenario, the Claude platform’s memory stores feature could be abused to establish a live C2 channel between an external operator and internal Claude Code agents. The external operator was able to dynamically assign network discovery, credential access, and data exfiltration tasks to the agents via this C2 channel. Detection of the agents’ activities relied on the correlation of process creation and network traffic activity from agent harness processes, alongside behavioral analysis to identify deviations from expected behavior. Darktrace has not claimed it observed this memory-store command-and-control technique in operational attacks [7].

### AI-assisted attacks in Japan

Evidence from Japan indicates AI being used for vulnerability scanning and attacks outside controlled research. On 21 October, JPCERT/CC published findings from an investigation into recent unauthorized access incidents affecting organizations in Japan. JPCERT/CC states that it confirmed attackers had used AI to scan for known vulnerabilities and conduct attacks through APIs. According to JPCERT/CC, its examination of attack infrastructure found evidence of Android application analysis, API discovery and testing for weaknesses including server-side request forgery, and the infrastructure also targeted services intended for users outside Japan [8].

JPCERT/CC does not identify ARTEX by name in that publication. The named ARTEX attribution comes from separate investigations, including Microsoft's analysis of Storm-0940.

### AI in offensive operations

Microsoft describes ARTEX as an open-source, multi-agent penetration-testing system capable of automating activity with limited human interaction. Microsoft reported that Storm-0940, which it assesses to be a China state-aligned actor active since 2021, used ARTEX against hundreds of organizations, and Microsoft attributed the ConnectWise supply-chain compromise [9].

Microsoft observed ARTEX accelerating scanning, reconnaissance, password spraying and OAuth application abuse. Several ARTEX-driven operations reached privileged cloud access less than three hours after initial targeting. Following the ConnectWise compromise, Storm-0940 attempted to authenticate to more than 2,250 tenants at rates of up to 2,900 attempts per minute [9].

Darktrace Threat Research found activity in its own telemetry involving infrastructure later associated with the campaign. On 13 October, Darktrace recorded failed authentication activity involving 144.172.64[.]128, one of the addresses Microsoft later identified as performing sign-ins and Microsoft Graph activity in ConnectWise customer environments. Darktrace analysis identified 65 failed authentication attempts in around 20 minutes.

Figure 1: Darktrace Advanced Search showing failed authentication events involving 144.172.64[.]128 on 13 October 2026, (customer-identifying information redacted).

The telemetry establishes repeated failed authentication from infrastructure later associated with the campaign. It does not establish ARTEX execution or successful compromise. At the time of observation, Microsoft's later attribution was unavailable. Analysts therefore had the behavior itself: repeated failed authentication from the same source over a short period, together with any related identity, application or network activity inside the environment.

Microsoft documented a wider password-spray campaign against 327 accounts across eight organizations between 10 and 14 October. It recorded more than 151,000 failed logons, including more than 111,000 on 11 October, but no successful authentication, token issuance, Graph call or directory change from that campaign. Microsoft also reported that most Storm-0940 targeting was unsuccessful [9].

Where the actor progressed beyond authentication, Microsoft observed service-principal sign-ins followed by Graph and directory operations. Activity included reconnaissance, application credential changes and attempts to modify privileges. Microsoft recommends correlating service-principal sign-ins with Graph activity, directory changes and resource access [9].

### Behavior before attribution

The PHP and ARTEX cases begin with different information. PHP defenders knew which software required urgent remediation but initially had limited information about exploitation. Process execution, file modification and outbound communication could still provide evidence of compromise.

The PHP and ARTEX observations also illustrate the difference between volume and impact. Darktrace observed broad scanning around the PHP disclosure, while the ARTEX-associated infrastructure produced concentrated authentication activity. Neither observation independently demonstrates compromise and in each case examining the behavior that follows the initial interaction is the investigative clue.

The Darktrace ARTEX-related observation began with authentication behavior. Microsoft publicly attributed the source infrastructure eight days later. That attribution added context, but it did not change what had originally been recorded in the environment.

AI is now present at several points in this cycle. Daybreak applies models to vulnerability discovery and remediation. Darktrace Signal Labs demonstrated how a trusted enterprise agent could be co-opted in a controlled environment. JPCERT/CC confirmed AI-assisted vulnerability scanning and API attacks in recent incidents. Microsoft documented ARTEX being used by Storm-0940 in active operations [4][7][8][9].

Microsoft's findings also show why scale and impact must be assessed separately. One ARTEX-associated campaign produced more than 151,000 failed logons without successful access, while other operations reached privileged cloud access in less than three hours. Security teams have to distinguish high-volume attempts from activity that progresses through authentication, privilege changes and resource access.

The evidence required to assess these cases remains familiar: authentication records, process execution, application and directory changes, file activity and network communication. AI can increase the rate at which those behaviors occur. Defenders still need to establish their sequence and impact.

Credit to Emma Foulger, Sam Lister, Dylan Evans, Moishe Tajtelbaum

### References

[1] NCSC-NL, php2shell vulnerabilities (CVE-2026-109618, CVE-2026-109619), 21 October 2026. https://github.com/NCSC-NL/php2shell

[2] Imperva, https://www.imperva.com/blog/node2shell-from-brotli-data-to-remote-code-execution/

[3] Cyber Security Agency of Singapore, https://www.csa.gov.sg/alerts-and-advisories/alerts/al-2026-151/

[4] OpenAI, Daybreak: Tools for securing every organization in the world, 22 June 2026. https://openai.com/index/daybreak-securing-the-world/

[5] OpenAI Developers, Scaling cyber defenders with Daybreak, 21 August 2026. https://developers.openai.com/blog/scaling-cyber-defenders-with-daybreak

[6] OpenAI, Cloudflare links vulnerability discovery to defense with OpenAI, 21 October 2026. https://openai.com/index/cloudflare-cyber/

[7] Darktrace Signal Labs, Shared Memories as a Covert C2 Channel, draft research supplied for review.

[8] JPCERT/CC, Recent AI-Assisted Cyber Attacks in Japan: Investigating Attack Infrastructure and Techniques, 21 October 2026. https://blogs.jpcert.or.jp/en/2026/10/web_api_attacks.html

[9] Microsoft Security Research and Microsoft Threat Intelligence, AI-driven, large-scale nation-state campaign using ARTEX enables supply chain compromise, 21 October 2026. https://www.microsoft.com/en-us/security/blog/2026/10/21/storm-0940-uses-artex-agentic-ai-connectwise-supply-chain-compromise/

[10] Darktrace, Advancing the Use of Frontier AI in Cybersecurity: Darktrace Joins the OpenAI Daybreak Cyber Partner Program to Explore Defensive AI Integrations, 22 June 2026. https://www.darktrace.com/blog/advancing-the-use-of-frontier-ai-in-cybersecurity-darktrace-joins-the-openai-daybreak-cyber-partner-program-to-explore-defensive-ai-integrations

[Continue reading](https://www.darktrace.com/blog/vulnerability-discovery-to-active-attacks-ai-in-cyber-operations)

About the author

Nathaniel Jones

SVP, Global Threat Intelligence

## Blog

/

## OT

/

## November 2, 2026

## Beyond the Fridge: Exposed Industrial Control Systems

## Introduction

In October 2026, hundreds of supermarkets across France were reportedly affected by a cyberattack targeting their refrigeration systems. According to public reports, the attackers gained access to internet-connected controllers and modified their settings, forcing refrigeration units into high and extremely low temperatures and causing significant disruption. Total losses have been reported to reach several million euros.

The incident drew attention to the security of industrial refrigeration systems, particularly older devices with exposed management interfaces and known vulnerabilities.

The exact details of how the attackers gained access have not been confirmed, however it is speculated that the public-facing web server could be accessed with the username “admin” and password being the current date. Researchers from Claroty’s Team82 identified a vulnerability in Danfoss AK-SM 8xxA Series in 2025, “CVE-2025-41450", allowing for administrative access using a time-based “code-of-the-day" for authentication [1].

Default credentials, outdated firmware and publicly accessible administrative interfaces have been an issue for years. And yet these systems continue to appear in internet scans and require little sophistication to gain admin access. A simple search of internet-connected devices shows a range of exposed devices from Programmable Logic Controllers (PLCs) and remote gateways, to, yes, more fridges.

Darktrace malware researchers identified hundreds of IoT and OT devices publicly exposed on the internet, many of which are configured with no authentication, weak credentials, or default credentials. As shown in Figure 1, an exposed Copeland XWEB web interface, used to manage Dixell equipment, including refrigerators, can provide the ability to remotely control connected devices.

‍

Figure 1: Copeland XWEB interface used to control systems including refrigerators and air conditioning.

However, it’s not only fridges; web management interfaces for industrial controls are also publicly accessible, often protected only by weak credentials. Figure 2 shows an example of a publicly exposed WAGO interface. The WAGO management interface is a configuration and administration panel for controllers. Through it, a logged-in user can view device details and diagnostics, manage user accounts and passwords, update firmware, back up or restore the full configuration, set the clock, and manage SD card storage. While it typically doesn’t manage switch outputs or read live process values, someone with access could still disrupt operation by changing network or Modbus settings, loading bad firmware or configurations, or by resetting passwords and enabling insecure services.

‍

Figure 2: WAGO Management interface.

‍

The issue extends beyond management interfaces. During our research, we identified publicly accessible web interfaces belonging to PLCs, including Rockwell Automation Allen-Bradley devices. In each case, the risk arises from how the devices have been deployed and configured, not from a flaw in the vendors’ products.

‍

Figure 3: Web interface for Allen-Bradley CompactLogix controller.

‍

Figure 3 shows the web interface of an Allen-Bradley CompactLogix controller. Unlike the previous examples, this interface provides a view into the controller's operation, including running tasks, network connections, memory usage and diagnostic information. Rockwell Automation's CompactLogix controllers are commonly used to automate industrial processes, from manufacturing and packaging lines to water treatment and material handling systems. Depending on their configuration, these controllers can operate motors, pumps, valves and other physical equipment.

Although in this example the interface shown does not provide direct control over the PLC's logic, it exposes information about the model, firmware, network layout, and connections, which is useful reconnaissance for a bigger attack.

The concern is not necessarily that every exposed interface allows an attacker to immediately manipulate industrial processes. However, these devices are accessible from the public internet in the first place, providing information about systems that may be responsible for controlling physical equipment.

The attacks on refrigeration systems in France highlight the consequences of poorly secured industrial devices. As demonstrated throughout this research, publicly accessible management interfaces are not limited to refrigeration controllers but extend to PLCs and other systems used to manage industrial equipment.

Finding these devices requires little effort. Internet scanners make it possible to identify publicly exposed interfaces quickly, and in some cases these interfaces rely on publicly documented default credentials or have no authentication at all. As attackers increasingly use AI tools, such attacks may be identified and carried out faster and at a greater scale.

The consequences of this access depend on the device and the functionality exposed. While some interfaces provide little more than diagnostic information, others allow users to modify configurations, disrupt communications, or change settings that affect physical equipment. In industrial environments, this can result in operational downtime, damaged equipment and financial losses.

Weak passwords, default credentials and exposed administrative interfaces are not new security issues. Yet their continued presence on internet-connected industrial systems demonstrates how easily preventable weaknesses can leave physical operations vulnerable to attack.

Exposed devices often come from third-party suppliers and contractors, who frequently install refrigeration controllers, gateways and PLCs without the organization ever being aware they are connected to the internet. Asset visibility remains an important foundation, but organizations should assume that some unknown assets and services will exist. Effective security therefore also depends on continuous monitoring, anomaly detection, network segmentation and rapid response, so that risky exposure and unusual activity can be identified and addressed before it becomes disruptive.

Darktrace helps security teams gain visibility across IT and OT and respond to risks: https://www.darktrace.com/products/operational-technology.

Copeland, Dixell, WAGO, Rockwell Automation, Allen-Bradley and CompactLogix are trademarks of their respective owners. Their use here is for identification only and does not imply any affiliation with or endorsement by those owners.

## References

1. https://claroty.com/team82/research/freeze-the-controller-defrost-the-food-uncovering-vulnerabilities-in-danfoss-refrigeration-controllers

‍

[Continue reading](https://www.darktrace.com/blog/beyond-the-fridge-exposed-industrial-control-systems)

About the author

Tara Gould

Malware Research Lead

Your data. Our AI.

Elevate your network security with Darktrace AI

[Get a demo](https://www.darktrace.com/demo)
