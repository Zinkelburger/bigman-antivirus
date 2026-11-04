---
url: "https://unit42.paloaltonetworks.com/peer-to-peer-worm-p2pinfect/"
title: "P2PInfect: The Rusty Peer-to-Peer Self-Replicating Worm"
site: "Unit 42"
published: "2023-07-19T17:00:56+00:00"
captured: "2026-11-02T18:48:54Z"
via: "live"
sha256: "614a43a1f3f26082af023950ddddd6e2571deca3fc9d143b8d77dbb6baba0ff7"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# P2PInfect: The Rusty Peer-to-Peer Self-Replicating Worm

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://unit42.paloaltonetworks.com/peer-to-peer-worm-p2pinfect/>

-  [Threat Research Center](https://unit42.paloaltonetworks.com)
- [Threat Research](https://unit42.paloaltonetworks.com/category/threat-research/)
- [Cloud Cybersecurity Research](https://unit42.paloaltonetworks.com/category/cloud-cybersecurity-research/)

[Cloud Cybersecurity Research](https://unit42.paloaltonetworks.com/category/cloud-cybersecurity-research/)

# P2PInfect: The Rusty Peer-to-Peer Self-Replicating Worm

11  min read

Related Products

[Advanced Threat Prevention](https://unit42.paloaltonetworks.com/product-category/advanced-threat-prevention/)[Advanced URL Filtering](https://unit42.paloaltonetworks.com/product-category/advanced-url-filtering/)[Advanced WildFire](https://unit42.paloaltonetworks.com/product-category/advanced-wildfire/)[Cloud-Delivered Security Services](https://unit42.paloaltonetworks.com/product-category/cloud-delivered-security-services/)[Cortex XDR](https://unit42.paloaltonetworks.com/product-category/cortex-xdr/)[Cortex XSIAM](https://unit42.paloaltonetworks.com/product-category/cortex-xsiam/)[Next-Generation Firewall](https://unit42.paloaltonetworks.com/product-category/next-generation-firewall/)[Prisma Cloud](https://unit42.paloaltonetworks.com/product-category/prisma-cloud/)[Unit 42 Incident Response](https://unit42.paloaltonetworks.com/product-category/unit-42-incident-response/)

-

By:

- [William Gamazo](https://unit42.paloaltonetworks.com/author/william-gamazo/)
- [Nathaniel Quist](https://unit42.paloaltonetworks.com/author/nathaniel-quist/)

-

Published:July 19, 2023

-

Categories:

- [Cloud Cybersecurity Research](https://unit42.paloaltonetworks.com/category/cloud-cybersecurity-research/)
- [Threat Research](https://unit42.paloaltonetworks.com/category/threat-research/)

-

Tags:

- [Cloud Security](https://unit42.paloaltonetworks.com/tag/cloud-security/)
- [Container security](https://unit42.paloaltonetworks.com/tag/container-security/)
- [P2p](https://unit42.paloaltonetworks.com/tag/p2p/)
- [Worm](https://unit42.paloaltonetworks.com/tag/worm/)

- https://unit42.paloaltonetworks.com/peer-to-peer-worm-p2pinfect/?pdf=download&lg=en&_wpnonce=0f3101da20
- https://unit42.paloaltonetworks.com/peer-to-peer-worm-p2pinfect/?pdf=print&lg=en&_wpnonce=0f3101da20

[Share](https://unit42.paloaltonetworks.com/peer-to-peer-worm-p2pinfect/#)

-  https://unit42.paloaltonetworks.com/peer-to-peer-worm-p2pinfect/#
-
-  https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Funit42.paloaltonetworks.com%2Fpeer-to-peer-worm-p2pinfect%2F
-  https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Funit42.paloaltonetworks.com%2Fpeer-to-peer-worm-p2pinfect%2F&title=P2PInfect:%20The%20Rusty%20Peer-to-Peer%20Self-Replicating%20Worm
-  https://twitter.com/intent/tweet?url=https%3A%2F%2Funit42.paloaltonetworks.com%2Fpeer-to-peer-worm-p2pinfect%2F&text=P2PInfect:%20The%20Rusty%20Peer-to-Peer%20Self-Replicating%20Worm
-  https://www.reddit.com/submit?url=https%3A%2F%2Funit42.paloaltonetworks.com%2Fpeer-to-peer-worm-p2pinfect%2F
-  https://mastodon.social/share?text=P2PInfect:%20The%20Rusty%20Peer-to-Peer%20Self-Replicating%20Worm%20https%3A%2F%2Funit42.paloaltonetworks.com%2Fpeer-to-peer-worm-p2pinfect%2F

## Executive Summary

On July 11, 2023, Unit 42 cloud researchers discovered a new peer-to-peer (P2P) worm we call P2PInfect. Written in Rust, a highly scalable and cloud-friendly programming language, this worm is capable of cross-platform infections and targets Redis, a popular open-source database application that is heavily used within cloud environments. Redis instances can be run on both Linux and Windows operating systems. Unit 42 researchers have identified over 307,000 unique Redis systems communicating publicly over the last two weeks, of which 934 may be vulnerable to this P2P worm variant. While not all of the 307,000 Redis instances will be vulnerable, the worm will still target these systems and attempt the compromise.

The P2PInfect worm infects vulnerable Redis instances by exploiting the Lua sandbox escape vulnerability, [CVE-2022-0543](https://nvd.nist.gov/vuln/detail/CVE-2022-0543). While the vulnerability was disclosed in 2022, its scope is not fully known at this point. However, it is rated in the NIST National Vulnerability Database with a Critical CVSS score of 10.0. Additionally, the fact that P2PInfect exploits Redis servers running on both Linux and Windows operating systems makes it more scalable and potent than other worms. The P2P worm observed by Unit 42 researchers serves as an example of a serious attack threat actors could conduct using this vulnerability.

P2PInfect exploits CVE-2022-0543 for initial access and then drops an initial payload that establishes P2P communication to a larger P2P network. Once the P2P connection is established, the worm pulls down additional malicious binaries such as OS-specific scripts and scanning software. The infected instance then joins the P2P network to provide access to the other payloads to future compromised Redis instances.

Exploiting CVE-2022-0543 in this way makes the P2PInfect worm more effective at operating and propagating in cloud container environments. This is where Unit 42 researchers discovered the worm by it compromising a Redis container instance within our HoneyCloud environment, which is a set of honeypots that we use to identify and study novel cloud-based attacks across public cloud environments.

Unit 42 believes this P2PInfect campaign is the first stage of a potentially more capable attack that leverages this robust P2P command and control (C2) network. There are instances of the word “miner” within the malicious toolkit of P2PInfect. However, researchers did not find any definitive evidence that cryptomining operations ever occurred. Additionally, the P2P network appears to possess multiple C2 features such as “Auto-updating” that would allow the controllers of the P2P network to push new payloads into the network that could alter and enhance the performance of any of the malicious operations. Unit 42 researchers will continue to monitor for changes and update accordingly.

Palo Alto Networks customers receive protections against the types of threats discussed in this article through products including:

- [Prisma Cloud](https://www.paloaltonetworks.com/prisma/cloud)
- [Advanced WildFire](https://www.paloaltonetworks.com/network-security/wildfire)
- [Cloud-Delivered Security Services](https://www.paloaltonetworks.com/network-security/security-subscriptions) for the [Next-Generation Firewall](https://www.paloaltonetworks.com/network-security/next-generation-firewall), including [Advanced URL Filtering](https://www.paloaltonetworks.com/network-security/advanced-url-filtering) and [Advanced Threat Prevention](https://docs.paloaltonetworks.com/advanced-threat-prevention).

If you believe you have been compromised by P2PInfect, the [Unit 42 Incident Response team](https://start.paloaltonetworks.com/contact-unit42.html) can provide a personalized response.

| Related Unit 42 Topics  | [Cloud](https://unit42.paloaltonetworks.com/category/cloud/), [Worm](https://unit42.paloaltonetworks.com/tag/worm/), [P2P](https://unit42.paloaltonetworks.com/tag/p2p/)

## Self-replicating Peer-to-Peer Worm

Unit 42 discovered the first known instance of P2PInfect on July 11, 2023, using our HoneyCloud environment, which is a set of honeypots that we use to identify and study novel cloud-based attacks across public cloud environments.

The P2PInfect worm uses a P2P network to support and facilitate the transmission of malicious binaries. We chose the name because the term P2PInfect appears in the leaked symbols reflecting the malware author project structure, as shown in Figure 1.

Figure 1. Artifacts of the Windows version, names and Redis module.

All collected samples of the P2P worm are written in Rust, a highly scalable and cloud-friendly programming language. This allows the worm to be capable of cross-platform infections that target Redis instances on both Linux and Windows operating systems (please note that Redis does not officially support the Windows OS).

The worm infects vulnerable Redis instances using the Lua sandbox escape vulnerability, [CVE-2022-0543](https://nvd.nist.gov/vuln/detail/CVE-2022-0543). The first exploit for this particular vulnerability was published in March 2022, which resulted in the connection of the infected Redis instance to the [Muhstik botnet](https://cymulate.com/threats/muhstik-gang-targets-redis-servers-2/). However, the P2PInfect worm appears to be associated with a different malicious network, not known to be related to the Muhstik botnet.

After initial infection through the exploitation of the Lua vulnerability, an initial payload is executed that establishes a P2P communication to the larger C2 botnet, which serves as a P2P network for delivering other payloads to future compromised Redis instances. Once the P2P connection is established, the worm pulls down additional payloads, such as a scanner. The newly infected instance then joins the ranks of the P2P network to provide scanning payloads to future compromised Redis instances.

Exploiting CVE-2022-0543 makes P2PInfect effective in cloud container environments. Containers have a reduced set of functionalities – for example, they do not have “cron” services. Many of the most active worms exploiting Redis use a technique to achieve remote code execution (RCE) using cron services. This technique does not work in containers. P2PInfect incorporates the exploit for [CVE-2022-0543](https://nvd.nist.gov/vuln/detail/CVE-2022-0543) with the intention of covering as many vulnerable scenarios as possible, including cloud container environments.

The following sections will cover details about the [exploitation payloads](https://unit42.paloaltonetworks.com/peer-to-peer-worm-p2pinfect/#post-129197-_ccmuifelh87), the [behavior of P2PInfect](https://unit42.paloaltonetworks.com/peer-to-peer-worm-p2pinfect/#post-129197-_g0sbcvc6zt6w), and some of the details of the [P2P network protocol](https://unit42.paloaltonetworks.com/peer-to-peer-worm-p2pinfect/#post-129197-_5zdttzhc36hk).

Since the P2PInfect worm is newly discovered, we have focused here on providing an overview of its behavior and the P2P architecture it supports, as well as basic sample analysis. However, additional analysis and study is warranted in future research.

## Exploitation of CVE-2022-0543

P2PInfect currently exploits the Lua sandbox escape vulnerability CVE-2022-0543 for initial access. This vulnerability has been used in previous attacks such as [Muhstik](https://blogs.juniper.net/en-us/security/muhstik-gang-targets-redis-servers) and [Redigo](https://cybersecuritynews.com/hackers-drops-bakcdoor-malware-redigo-redis-server/), both of which resulted in the compromised Redis instances participating in denial-of-service (DoS), flooding and brute-forcing attacks against other systems.

This exploit vector follows a similar pattern to what has been seen previously. However, the post-exploit operations of P2PInfect are significantly different from the previous uses of the vulnerability. It is important to note that this vulnerability is not a Redis application vulnerability — it is specifically a Lua sandbox vulnerability.

While this campaign does target vulnerable Redis instances and perform worm-like operations, there are no known links to other threat actor groups known for targeting Redis and deploying worms, such as [Automated Libra (aka PurpleUrchin)](https://unit42.paloaltonetworks.com/atoms/automated-libra), [Adept Libra (aka TeamTNT)](https://unit42.paloaltonetworks.com/atoms/adept-libra), [Thief Libra (aka WatchDog)](https://unit42.paloaltonetworks.com/atoms/thieflibra), [Money Libra (aka Kinsing)](https://unit42.paloaltonetworks.com/atoms/moneylibra), [Aged Libra (aka Rocke)](https://unit42.paloaltonetworks.com/atoms/agedlibra) or [Returned Libra (aka 8220)](https://unit42.paloaltonetworks.com/atoms/returnedlibra).

### How P2PInfect Leverages CVE-2022-0543 to Infect Vulnerable Redis Instances

The P2PInfect worm’s initial infection vector – exploiting Redis through CVE-2022-0543 – is not common among other cryptojacking-focused worms known to target Redis instances, such as those created by Adept Libra (aka TeamTnT), Thief Libra (aka WatchDog) threat actors or the ones delivering Money Libra (aka Kinsing) variants. These groups use alternative Redis vulnerabilities or misconfigurations in order to operate.

[CVE-2022-0543](https://nvd.nist.gov/vuln/detail/CVE-2022-0543) is a vulnerability with the Lua library related to the way Redis is packaged and delivered by Debian Linux package management. As such, it only affects users of Redis who use the Debian or derived (Ubuntu and others) distributions. Due to the focus on the OS and leveraging a subcomponent of Redis to compromise, P2PInfect’s exploitation efforts are therefore complex. Figure 2 shows an example of a captured exploit for CVE-2022-0543.

Figure 2. Example of the P2PInfect exploit on the Debian OS.

Within the above image, one can see how the vulnerability is being weaponized. By using network requests through /dev/tcp, as seen on line four, the threat actor connects to a C2 IP address, written as ip-cnc over port 60100. Port 60100 is one of the P2P communication ports used by P2PInfect to maintain C2 communication. The initial payload, also seen on line four, sets the GET request to the directory /linux, which is the main dropper maintaining the core functionality of the P2PInfect worm. Other binaries are distributed within the P2P network, as we are going to see later in the article.

## Network Communication Behavior

P2PInfect uses its P2P network to distribute follow-up malware to newly infected systems or cloud instances. When a system is first compromised, it will make a network connection to the P2P network and download the samples for the custom protocol to be used. As Figure 3 illustrates, the command: GET /linux, is followed by the image download of the core P2PInfect functionality.

Figure 3. Network communication protocol displaying the download of P2PInfect.

Both Linux and Windows OS P2PInfect samples communicate in the same manner. The following samples were downloaded from the P2P network in plaintext: linux, miner, winminer and windows (see Figure 4).

Figure 4. List of the malware samples pulled from the P2P network.

Once the core P2PInfect sample finishes execution, the payload will start scanning for additional hosts to compromise. The scanning operation focuses on exposed Redis hosts. However, researchers also found that compromised Redis instances also perform scanning attempts over port 22, SSH. While it is not clear why this scanning operation is taking place, as there are no known exploitation attempts by P2PInfect to compromise SSH, it is not altogether uncommon for port 22 to be scanned post-compromise by other known worms. Please see the [Scanning Behavior](https://unit42.paloaltonetworks.com/peer-to-peer-worm-p2pinfect/#post-129197-_ptdm3bngloup) section for additional details.

### Node Communications

The main dropper communicates with any other listening P2P members on the current list of configured nodes using TLS 1.3. The C2 infrastructure is updated when the compromised node sends a json request with all known nodes to the P2P network. Updates to the C2 infrastructure will automatically be downloaded. The following image, Figure 5, shows an example of the nodes update.

Figure 5. P2P nodes update.

The values with x.x.x.x are the current node IP, or the new learned nodes.

## Scanning Behavior

Figure 6 illustrates the network scanning behavior of an infected host scanning for exposed SSH instances. These scanning operations occur across a random netrange selected by the P2PInfect functionality.

Figure 6. Scanning traffic for SSH instances.

Figure 7 illustrates the P2PInfect scanning operations for exposed Redis instances.

Figure 7. Scanning traffic for Redis instances.

## Other Observations of P2PInfect

Some of the initial payload P2PInfect samples delivered to exploited systems were packed with UPX, while the second-stage malware samples, miner and winminer, were not UPX packed.

After the first dropper runs, it starts decrypting the configuration received from a command line, with information about other nodes in the P2P network. We found that the P2P port was variable – a design choice that allows the attack to be resilient to blocking and network firewall mitigation techniques (see Figure 8).

Figure 8. Example of the variable port usage of P2PInfect.

All samples identified by Unit 42 researchers have been written in Rust, and some have “symbols leaked” inside, which gives indicators about the malware authors’ project structure. For example, the windows sample main execution thread leaks the name of the project as well as the file directory usage of the threat actor (see Figure 9).

Figure 9. Analysis pulled from the core Windows P2PInfect sample.

We also identified a PowerShell script designed to establish and maintain communication between the compromised host and the P2P network. The PowerShell script leveraged the encode command to obfuscate the communication initiation (see Figure 10).

Figure 10. Obfuscated PowerShell command to establish P2P network connection.

One of the first operations performed by the PowerShell command is to configure the local system firewall to block legitimate access to or from the compromised Redis application (see line five of Figure 11). Then (starting on line 17 in Figure 11), the script opens a communication port for the threat actor to access the compromised instance. This is a form of persistence, allowing the threat actors to maintain access to the infected host and keep it operable.

Figure 11. Modifying the network traffic rules of a compromised Windows instance.

Of note from the decoded PowerShell, shown in Figure 11, are the following firewall configuration settings:

- Peer-to-peer port is 60102 – this port is variable, as not all nodes use the same port
- Redis port 6379 is only allowed to connect known C2 IPs
- The firewall rule is named Microsoft Sync

### The Monitor Process

Another interesting feature of the initial P2PInfect payload when running in Windows OS is a process called the Monitor. The Monitor process fulfills the role of maintaining the functionality of the P2PInfect running processes on the infected host. The Monitor is dumped to C:\Users\username\AppData\Local\Temp\cmd.exe (see Figure 12 for an example of the Monitor (cmd.exe) enumerating system running processes).

Figure 12. The P2PInfect Monitor sample, cmd.exe process tree.

After launching the Monitor (cmd.exe), the initial P2PInfect payload downloads new versions of itself from the P2P network and persists them with random names into the same original folder and an encrypted configuration is dropped (.conf) (see Figure 13).

Figure 13. Example of the random filenames.

The new P2PInfect download versions are executed, and the scanning operations to locate additional vulnerable Redis instances starts. The initial P2PInfect dropper will attempt to delete itself (see Figure 14).

Figure 14. Deletion of the core P2PInfect payload.

## Conclusion

The P2PInfect worm appears to be well designed with several modern development choices. Key among these is the use of the Rust language, which provides resilient capabilities and the flexibility to allow the worm to rapidly spread across multiple operating systems.

The design and building of a P2P network to perform the auto-propagation of malware is not something commonly seen within the cloud targeting or cryptojacking threat landscape. At the same time, we believe it was purpose-built to compromise and support as many Redis vulnerable instances as possible across multiple platforms.

We have caught several samples within our HoneyCloud platform, across multiple geographic regions, and we strongly believe the number of P2P nodes is growing. This is due to the volume of potential targets – over 307,000 Redis instances communicating publicly over the last two weeks – and since the worm was able to compromise multiple of our Redis honeypots across disparate regions. However, we don't have an estimate yet of how many nodes exist or how fast the malicious network associated with P2PInfect is growing.

We recommend that organizations monitor all Redis applications, both on-premises and within cloud environments, to ensure they do not contain random filenames within the /tmp directory. Additionally, DevOps personnel should continually monitor their Redis instances to ensure they maintain legitimate operations and maintain network access. All Redis instances should also be updated to their latest versions or anything newer than redis/5:6.0.16-1+deb11u2, redis/5:5.0.14-1+deb10u2, redis/5:6.0.16-2 and redis/5:7.0~rc2-2.

Palo Alto Networks customers receive protections against the types of threats in the following ways:

- [Prisma Cloud](https://docs.paloaltonetworks.com/prisma/prisma-cloud/prisma-cloud-admin-compute/technology_overviews/intel_stream) is capable of identifying the runtime environment of any cloud Redis instance to ensure it detects and prevents the unknown and malicious execution of the P2PInfect worm.
- [Cloud-Delivered Security Services](https://www.paloaltonetworks.com/network-security/security-subscriptions) for the [Next-Generation Firewall include a variety of protections.](https://www.paloaltonetworks.com/network-security/next-generation-firewall)
- [Advanced URL Filtering](https://www.paloaltonetworks.com/network-security/advanced-url-filtering) blocks malicious IoCs related to this worm.
- [Advanced Threat Prevention](https://docs.paloaltonetworks.com/advanced-threat-prevention) can block the attacks with Best Practices via Threat Prevention signatures [92349](https://threatvault.paloaltonetworks.com/?query=92349) and [93004](https://threatvault.paloaltonetworks.com/?query=93004).
- The [Advanced WildFire](https://www.paloaltonetworks.com/network-security/advanced-wildfire) cloud-delivered malware analysis service accurately identifies known samples as malicious.

If you think you might have been impacted or have an urgent matter, get in touch with the [Unit 42 Incident Response team](https://start.paloaltonetworks.com/contact-unit42.html?_gl=1*13pmp8e*_ga*NzQyNjM2NzkuMTY2NjY3OTczNw..*_ga_KS2MELEEFC*MTY2OTczNjA2MS4zMS4wLjE2Njk3MzYwNjEuNjAuMC4w) or call:

- North America Toll-Free: 866.486.4842 (866.4.UNIT42)
- EMEA: +31.20.299.3130
- APAC: +65.6983.8730
- Japan: +81.50.1790.0200

Palo Alto Networks has shared these findings, including file samples and indicators of compromise, with our fellow Cyber Threat Alliance (CTA) members. CTA members use this intelligence to rapidly deploy protections to their customers and to systematically disrupt malicious cyber actors. Learn more about the [Cyber Threat Alliance](https://www.cyberthreatalliance.org/).

## Indicators of Compromise

### SHA256 Samples

Linux:

- 88601359222a47671ea6f010a670a35347214d8592bceaf9d2e8d1b303fe26d7

Miner:

- b1fab9d92a29ca7e8c0b0c4c45f759adf69b7387da9aebb1d1e90ea9ab7de76c

Windows:

- 68eaccf15a96fdc9a4961daffec5e42878b5924c3c72d6e7d7a9b143ba2bbfa9

WinMiner:

- 89be7d1d2526c22f127c9351c0b9eafccd811e617939e029b757db66dadc8f93

### IPs

- 35.183.81[.]182
- 66.154.127[.]38
- 66.154.127[.]39
- 8.218.44[.]75
- 97.107.96[.]14

### CNC Requests

- GET /linux
- GET /linux_sign
- GET /miner
- GET /miner_sigg
- GET /winminer
- GET /winminer_sign
- GET /windows_sign
- GET /windows

Updated July 20, 2023, at 1:08 p.m. PT.

### Tags

- [Cloud Security](https://unit42.paloaltonetworks.com/tag/cloud-security/)
- [Container security](https://unit42.paloaltonetworks.com/tag/container-security/)
- [P2p](https://unit42.paloaltonetworks.com/tag/p2p/)
- [Worm](https://unit42.paloaltonetworks.com/tag/worm/)

[Threat Research Center](https://unit42.paloaltonetworks.com) [Next: CVE-2023-36884 - Microsoft Office and Windows HTML Remote Code Execution: Threat Brief (Updated)](https://unit42.paloaltonetworks.com/cve-2023-36884-rce/)

### Table of Contents

-

### Related Articles

-  [Essential Data Sources for Detection Beyond the Endpoint](https://unit42.paloaltonetworks.com/detection-beyond-the-endpoint/)
-  [The Browser Defense Playbook: Stopping the Attacks That Start on Your Screen](https://unit42.paloaltonetworks.com/browser-defense-playbook/)
-  ["Shai-Hulud" Worm Compromises npm Ecosystem in Supply Chain Attack (Updated November 26)](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)

## Related Cloud Cybersecurity Research Resources

[Threat Research](https://unit42.paloaltonetworks.com/category/threat-research/) September 21, 2026 [#### From Exposure to Lockdown: How AWS Neutralizes Compromised IAM Credentials through Managed Policies](https://unit42.paloaltonetworks.com/detecting-exposed-aws-iam-credentials/)

-  [AWS](https://unit42.paloaltonetworks.com/tag/aws/)
-  [AWS CloudTrail](https://unit42.paloaltonetworks.com/tag/aws-cloudtrail/)
-  [Bedrock](https://unit42.paloaltonetworks.com/tag/bedrock/)

[Read now](https://unit42.paloaltonetworks.com/detecting-exposed-aws-iam-credentials/)

[Threat Research](https://unit42.paloaltonetworks.com/category/threat-research/) September 18, 2026 [#### A Vault with a Heap-View: The Uncomfortable Space Between AgentCore Harness and Identity](https://unit42.paloaltonetworks.com/securing-aws-agentcore-harness-credentials/)

-  [Agentic AI](https://unit42.paloaltonetworks.com/tag/agentic-ai/)
-  [AWS](https://unit42.paloaltonetworks.com/tag/aws/)
-  [Cloud](https://unit42.paloaltonetworks.com/tag/cloud/)

[Read now](https://unit42.paloaltonetworks.com/securing-aws-agentcore-harness-credentials/)

[Threat Research](https://unit42.paloaltonetworks.com/category/threat-research/) September 14, 2026 [#### Unmasking Cloud Identities: From Behavioral Clustering to Automated Detection](https://unit42.paloaltonetworks.com/behavioral-clustering-map-to-cloud-identities/)

-  [AWS CloudTrail](https://unit42.paloaltonetworks.com/tag/aws-cloudtrail/)
-  [Cloud detection](https://unit42.paloaltonetworks.com/tag/cloud-detection/)
-  [DevOps](https://unit42.paloaltonetworks.com/tag/devops/)

[Read now](https://unit42.paloaltonetworks.com/behavioral-clustering-map-to-cloud-identities/)

[Threat Research](https://unit42.paloaltonetworks.com/category/threat-research/) June 22, 2026 [#### The Global Namespace Risk: Universal Bucket Hijacking Technique for Cloud Data Exfiltration](https://unit42.paloaltonetworks.com/cloud-bucket-hijacking-risks/)

-  [AWS](https://unit42.paloaltonetworks.com/tag/aws/)
-  [Bucket hijacking](https://unit42.paloaltonetworks.com/tag/bucket-hijacking/)
-  [Cloud data exfiltration](https://unit42.paloaltonetworks.com/tag/cloud-data-exfiltration/)

[Read now](https://unit42.paloaltonetworks.com/cloud-bucket-hijacking-risks/)

[Threat Research](https://unit42.paloaltonetworks.com/category/threat-research/) June 16, 2026 [#### Pickle in the Middle – Hijacking Vertex AI Model Uploads for Cross-Tenant RCE](https://unit42.paloaltonetworks.com/hijacking-vertex-ai-model/)

-  [Bucket squatting](https://unit42.paloaltonetworks.com/tag/bucket-squatting/)
-  [Google Cloud](https://unit42.paloaltonetworks.com/tag/google-cloud/)
-  [Joblib](https://unit42.paloaltonetworks.com/tag/joblib/)

[Read now](https://unit42.paloaltonetworks.com/hijacking-vertex-ai-model/)

[Threat Research](https://unit42.paloaltonetworks.com/category/threat-research/) June 9, 2026 [#### Blinding the Watchmen: Abusing Cloud Logging Services for Defense Evasion and Visibility](https://unit42.paloaltonetworks.com/cloud-logging-defense-evasion/)

-  [AWS CloudTrail](https://unit42.paloaltonetworks.com/tag/aws-cloudtrail/)
-  [Cloud logging](https://unit42.paloaltonetworks.com/tag/cloud-logging/)
-  [Defense evasion](https://unit42.paloaltonetworks.com/tag/defense-evasion/)

[Read now](https://unit42.paloaltonetworks.com/cloud-logging-defense-evasion/)

[Threat Research](https://unit42.paloaltonetworks.com/category/threat-research/) May 22, 2026 [#### Paved With Intent: ROADtools and Nation-State Tactics in the Cloud](https://unit42.paloaltonetworks.com/roadtools-cloud-attacks/)

-  [Curious Serpens](https://unit42.paloaltonetworks.com/tag/curious-serpens/)
-  [Entra ID](https://unit42.paloaltonetworks.com/tag/entra-id/)
-  [Microsoft Azure](https://unit42.paloaltonetworks.com/tag/microsoft-azure/)

[Read now](https://unit42.paloaltonetworks.com/roadtools-cloud-attacks/)

[Threat Research](https://unit42.paloaltonetworks.com/category/threat-research/) April 23, 2026 [#### Can AI Attack the Cloud? Lessons From Building an Autonomous Cloud Offensive Multi-Agent System](https://unit42.paloaltonetworks.com/autonomous-ai-cloud-attacks/)

-  [AI](https://unit42.paloaltonetworks.com/tag/ai/)
-  [Cloud](https://unit42.paloaltonetworks.com/tag/cloud/)
-  [Data exfiltration](https://unit42.paloaltonetworks.com/tag/data-exfiltration/)

[Read now](https://unit42.paloaltonetworks.com/autonomous-ai-cloud-attacks/)

[Threat Research](https://unit42.paloaltonetworks.com/category/threat-research/) March 23, 2026 [#### Google Cloud Authenticator: The Hidden Mechanisms of Passwordless Authentication](https://unit42.paloaltonetworks.com/passwordless-authentication/)

-  [Google](https://unit42.paloaltonetworks.com/tag/google/)
-  [Google authenticator](https://unit42.paloaltonetworks.com/tag/google-authenticator/)
-  [Google Chrome](https://unit42.paloaltonetworks.com/tag/google-chrome/)

[Read now](https://unit42.paloaltonetworks.com/passwordless-authentication/)

[Threat Research](https://unit42.paloaltonetworks.com/category/threat-research/) February 6, 2026 [#### Novel Technique to Detect Cloud Threat Actor Operations](https://unit42.paloaltonetworks.com/tracking-threat-groups-through-cloud-logging/)

-  [API](https://unit42.paloaltonetworks.com/tag/api/)
-  [IAM](https://unit42.paloaltonetworks.com/tag/iam/)
-  [MITRE](https://unit42.paloaltonetworks.com/tag/mitre/)

[Read now](https://unit42.paloaltonetworks.com/tracking-threat-groups-through-cloud-logging/)

-
-
