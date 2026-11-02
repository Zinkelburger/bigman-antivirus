---
url: "https://www.aquasec.com/blog/threat-alert-teamtnts-docker-gatling-gun-campaign/"
title: "TeamTNT’s Docker Gatling Gun Campaign"
site: "Aqua"
published: "2024-10-25T05:05:37+00:00"
captured: "2026-11-02T18:52:29Z"
via: "live"
sha256: "478d6bd12beecef23d3ff13ba5d12dda5c49a12e292d688811b2f6bf73b7ed57"
wayback: "https://web.archive.org/web/20260617004012/https://www.aquasec.com/blog/threat-alert-teamtnts-docker-gatling-gun-campaign/"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# TeamTNT’s Docker Gatling Gun Campaign

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.aquasec.com/blog/threat-alert-teamtnts-docker-gatling-gun-campaign/>
> Snapshot: <https://web.archive.org/web/20260617004012/https://www.aquasec.com/blog/threat-alert-teamtnts-docker-gatling-gun-campaign/>

[Aqua Blog](https://www.aquasec.com/blog/)

# TeamTNT’s Docker Gatling Gun Campaign

Security Threat

https://www.aquasec.com/authors/assaf-morag/

[Assaf Morag](https://www.aquasec.com/authors/assaf-morag/)

October 25, 2024

Long time no see, Aqua Nautilus researchers have identified a new campaign in the making by TeamTNT, a notorious hacking group. In this campaign, TeamTNT appears to be returning to its roots while preparing for a large-scale attack on cloud native environments. The group is currently targeting exposed Docker daemons to deploy Sliver malware, a cyber worm, and cryptominers, using compromised servers and Docker Hub as the infrastructure to spread their malware.

In this campaign, TeamTNT is leveraging native capabilities in cloud environments by appending compromised Docker instances to a Docker Swarm and utilizing Docker Hub to store and distribute their malware. They are also renting the victims’ computational power to third parties, effectively earning money indirectly from cryptomining without the hassle of managing it themselves. In addition, they have adopted new hacking tools by replacing their traditional Tsunami backdoor with the stealthier Sliver malware. In this blog, we will explore the campaign and its components.

### The attack flow

We are currently witnessing the preparation of TeamTNT’s cloud based infrastructure for a large-scale attack. The group is using both compromised web servers and Docker Hub registries to disseminate malware, aiming to deploy cryptominers or rent out computational power to third parties.

There are clear indicators of a TeamTNT campaign, including their characteristics, naming conventions, choice of tools, and familiar infrastructure. They even returned using their retired domain teamtnt.red, which was terminated in 2022.

Below you can see the entire attack flow:

https://www.aquasec.com/wp-content/uploads/2024/10/TeamTNT-Gutling-Gun.jpg

Figure 1: The entire attack flow

Throughout the past seven years of observing TeamTNT’s campaigns, we have consistently seen four key elements:

- External and Local Lateral Movement: Aggressive detection and infection methods using tools like Masscan and ZGrab and local searches to propagate the infection across additional servers within the target’s network.
- Resource Hijacking: This campaign focuses on cryptominer deployment and the evolution of selling compromised infrastructure to others, avoiding the overhead of running their own cryptomining operation.
- Command and Control (C2): In this campaign, TeamTNT is using Sliver malware, which may replace the previously used Tsunami malware. In earlier campaigns, researchers had gained insights into TeamTNT’s operations by infiltrating their IRC servers, but Sliver makes this much harder. Having said that the new infrastructure also include a Tsunami malware capability.
- Cloud Tools: TeamTNT has always experimented with cloud native open-source software (OSS) and offensive security tools (OSTs). In this campaign, they are using Docker Hub to store and spread malware, and Sliver for control and exploitation.

Additionally, TeamTNT continues to use their established naming conventions, such as Chimaera,TDGG, and bioset (for C2 operations), which reinforces the idea that this is a classic TeamTNT campaign.

https://www.aquasec.com/wp-content/uploads/2024/10/Screenshot-2024-10-22-at-22.16.06.jpg

Figure 2: TeamTNT’s signature ASCII art

#### Initial access

The campaign gains initial access by exploiting exposed Docker daemons on ports 2375, 2376, 4243, and 4244 (there are also indications to exploit K8s clusters in the future). The attack script, which scans for these ports, is known as the Docker Gatling Gun. It targets a wide range of IP addresses (~16.7 million) and deploys a container from TeamTNT’s compromised Docker Hub account, running an Alpine Linux image with malicious commands. The image executes an initial script called TDGGinit.sh (short for TeamTNT’s Docker Gatling Gun init shell script).

https://www.aquasec.com/wp-content/uploads/2024/10/carbon-2024-10-22T223522.137.jpg

Figure 3: The function TDGG

#### Sliver malware

A new tool used in this campaign is the Sliver malware, which replaces the previously used Tsunami backdoor. Sliver is an open-source, cross-platform,adversary emulation and red-team framework. Its implants support Command and Control (C2) over multiple protocols, including mTLS, WireGuard, HTTP(S), and DNS, and are dynamically compiled with per-binary asymmetric encryption keys. It can be used to execute commands and deliver payloads, including in-memory execution.

https://www.aquasec.com/wp-content/uploads/2024/10/Screenshot-2024-10-23-at-1.08.52.jpg

Figure 4: The C2 generate function in the Sliver binary

TeamTNT also maintains a list of compromised victims on their web servers (as seen in previous campaigns) and has experimented with adding compromised servers to a Docker Swarm. Docker Swarm is a native clustering and orchestration tool that allows Docker nodes to be managed as a single system.

We also found references to TeamTNT’s past campaigns. For example, in the current campaign one container image referenced the domain solscan[.]life, along with the path Chimaera, which was part of a previous TeamTNT campaign in 2021.

https://www.aquasec.com/wp-content/uploads/2024/10/Screenshot-2024-10-24-at-12.10.58.jpg

Figure 5: Chimaera’s campaign Twit taken from X.com

On the website there’s infrastructure for a third wave of Chimaera, targeting more initial access vectors, than solely exposed docker daemons.

In the Chimeara capmaign (https[:]//solscan[.]life/chimaera/sh) it appears that TeamTNT are actively targeting SSH, Jupyter, Docker, Kubernetes misconfigurations, as well as credentials of SSH, AWS, Docker, s3cfg, GitHub, Shodan, gcloud, Ngrok, Pidgin, FileZilla, HexChat, MoneroGuiWallet, CloudFlared, davfs2, PostgreSQL, smbClients.

Furthermore, in this campaign TeamTNT is also using anondns (AnonDNS or Anonymous DNS is a concept or service designed to provide anonymity and privacy when resolving DNS queries), in order to point to their web server. They use devnull.anondns.net to point on IP address 45.154.2.77.

## Additional threat intelligence on the infrastructure

#### Malicious servers and websites

TeamTNT registered several new domains on September 24th, 2024, including solscan.life, solscan.one, solscan.online, and solscan.store. These domains host malicious binaries and scripts that support their campaign.

https://www.aquasec.com/wp-content/uploads/2024/10/Screenshot-2024-10-22-at-22.01.33.jpg

https://www.aquasec.com/wp-content/uploads/2024/10/Screenshot-2024-10-22-at-22.01.56.jpg

Figures 6&7: A couple of TeamTNT’s domains

As seen in the screenshots below, these websites were created recently, with dates showing they went live on October 19th, 2024.

https://www.aquasec.com/wp-content/uploads/2024/10/Screenshot-2024-10-22-at-21.59.47.jpg

Figure 8: A of screenshot from TeamTNT’s download HTTP servers

https://www.aquasec.com/wp-content/uploads/2024/10/Screenshot-2024-10-22-at-14.38.29.jpg

Figure 9: A of screenshot from TeamTNT’s download HTTP servers

https://www.aquasec.com/wp-content/uploads/2024/10/Screenshot-2024-10-24-at-12.16.19.jpg

Figure 10: The Chimaera artifacts on the web server

One interesting observation is that the domain solscan.life (hosted on IP 95.182.101.23) has an open port 6670, typically used for IRC servers. This suggests that TeamTNT may still use Tsunami malware as a C2 server, though the more modern Sliver C2 may take precedence in their current operations. Meanwhile, solscan.one (IP 45.154.2.77) has port 8888 open, used for Sliver C2 communication. Now they are using 188.114.97.7 and 188.114.96.7 to host their old website (teamtnt.red).

#### Docker Hub account compromise

TeamTNT has a history of compromising Docker Hub accounts. In this case, a Docker Hub account (nmlm99) that appeared to be legitimate was breached, and TeamTNT used it to host malware. Over the last month, this account saw a significant spike in activity, uploading around 30 images, divided into two categories:

- Infrastructure Images: These 10 images are used to deploy malware or worms to detect new victims.
- Impact Images: The remaining 20 images focus on running cryptominers or appending victim servers to platforms like Mining Rig Rentals, where computational power is rented in exchange for cryptocurrency. This group of images includes the following cryptomining software: XMRIG, T-Rex miner, CGMiner, BFGMiner, and SGMiner. The latter is utilizing GPU mining to earn more money.

https://www.aquasec.com/wp-content/uploads/2024/10/Screenshot-2024-10-24-at-10.55.32.jpg

Figures 11: The Mining Rig Rentals website, mind the earn more section

Mapping the Campaign to the MITRE ATT&CK Framework

Our investigation showed that the attackers have been using some common techniques throughout the campaign. Here we map each component of the attack to the corresponding techniques of the [MITRE ATT&CK](https://attack.mitre.org/) framework:

https://www.aquasec.com/wp-content/uploads/2024/10/mitre-table.png

#### Initial access

- Exploit Public-Facing Application: TeamTNT gains initial access by exploiting exposed Docker daemons (ports 2375, 2376, 4243, and 4244). This is a well known technique used to compromise systems that have publicly accessible services.

#### Execution

- Command and Scripting Interpreter: The initial script, TDGGinit.sh, is executed on compromised systems to launch subsequent malicious actions.

#### Persistence

- Modify Cloud Compute Infrastructure – Create Cloud Instance: TeamTNT download Docker and Dockerswarm binaries and actively exposed Docker instances to a Docker Swarm, allowing them to persist in the environment as part of a larger cluster, ensuring continued access and control.

#### Defense evasion

- Exploitation for Defense Evasion: TeamTNT uses Sliver malware, which replaces their previous Tsunami malware. Sliver is harder to detect, and it evades traditional detection methods by dynamically compiling with per-binary encryption keys.

- Masquerading: TeamTNT uses names like Chimaera and other familiar naming conventions (such as bioset) to evade detection by masquerading as legitimate processes or infrastructure.

- Rootkit: We found prochider rootkit ready to deploy in TeamTNT’s download server. They are known to use this in the past.

#### Credentials access

- Unsecured Credentials: Credentials in Files: TeamTNT deploy among other a local search of keys and credentials, such as SSH, cloud metadata server calls etc. Once they gain access, they store and disseminate their malware through these accounts.

#### Discovery

- Network Service Scanning: TeamTNT uses tools like masscan to aggressively scan the internet for exposed Docker daemons and other vulnerable systems, identifying additional targets.

- Remote System Discovery: The campaign involves local network scanning to find additional systems that can be compromised.

#### Command and control

- Web Service – Dead Drop Resolver: TeamTNT uses Docker Hub and web servers as part of their infrastructure to store and distribute malware and to manage infected systems.

- Application Layer Protocol – DNS: TeamTNT’s use of Sliver malware supports DNS for Command and Control (C2) communication, in addition to HTTP(S) and mTLS.

- Proxy: Sliver supports WireGuard and other proxy techniques to tunnel C2 communications through legitimate channels, bypassing detection.

#### Impact

- Resource Hijacking: Running a cryptominer as part of the campaign or selling computation power of their victims.

### Conclusion

TeamTNT’s latest campaign shows their ability to adapt and evolve, incorporating modern tools like Sliver malware and leveraging cloud native capabilities to scale their attacks. By exploiting exposed Docker daemons and utilizing Docker Hub, TeamTNT is setting the stage for a large-scale attack, this is exactly how the [Silentbob](https://www.aquasec.com/blog/threat-alert-anatomy-of-silentbobs-cloud-attack/) campaign started. With some exploitation experiments against exposed Docker Daemon and then targeting K8s cluster and other vulnerabilities and misconfigurations. Eventually targeting dozens of environments. Organizations need to ensure proper security configurations for Docker instances and continuously monitor for unusual activity to stay ahead of this persistent threat.

### Indications of Compromise (IOCs)

| Type  | Value  | Comment

| IP Addresses

| IP Address  | 188.114.96.7  | Host download server

| IP Addresses  | 188.114.96.7  | Host download server

| IP Addresses   | 104.21.8.145  | Host download server

| IP Address  | 172.67.130.114  | Host download server

| IP Address  | 45.154.2.77  | Silver C2

| IP Address  | 95.182.101.23  | Tsunami C2

| Domain

| Domains  | solscan.life  |

| Domain  | solscan.one  |

| Domain  | solscan.online  |

| Domain  | solscan.store  |

| Domain  | devnull.anondns.net  |

| Domain  | teamtnt.red  |

| Files

| Binary file  | MD5=b62ce36054a7e024376b98df7911a5a7   | prochider (xmrig.so)

| Binary file  | MD5=64c3ac5a0f4318f64f438e78a6b42d40   | prochider (systemd.so)

| Binary file  | MD5=8b553728900ba2e45b784252a1ff6d17   | Sliver Malware (SPLENDID_ISLAND)

| Binary file  | MD5=9dc2819c176c60e879f28529b1b08da1   | Sliver Malware (bioset)

| Shell script   | MD5=a733160e0603207d8328ddb025c43d42   | TDGGinit, TDGGinit.sh

| Shell script   | MD5=fdf9c2f7221de9f3567fc094d5e759a9   | TDGG, TDGG.sh

| Shell script   | MD5=0bc189bb53c9c92322e7b2fd6ac68bd7   | docker

| Perl script  | MD5=db2fbe4d00b222cab6dd00cdfdd38e31   | scan.pl

| Docker Hub Accounts

| nmlm99   | https://hub.docker.com/u/nmlm99   | Disseminates malware and cryptominers

Published under: [SECURITY RESEARCH](https://www.aquasec.com/category/research/)

Tags: [Security Threats](https://www.aquasec.com/tag/security-threats/)

[Assaf Morag](https://www.aquasec.com/authors/assaf-morag/)

Assaf was the Director of Threat Intelligence at Aqua Nautilus.

https://www.linkedin.com/in/assaf-morag-812a9432?lipi=urn%3Ali%3Apage%3Ad_flagship3_profile_view_base_contact_details%3BjV7HR%2FUHRF%2BsUjhj%2FSgAFA%3D%3D

https://www.facebook.com/sharer/sharer.php?u=https://www.aquasec.com/blog/threat-alert-teamtnts-docker-gatling-gun-campaign/ https://twitter.com/share?url=https://www.aquasec.com/blog/threat-alert-teamtnts-docker-gatling-gun-campaign/&text=TeamTNT%E2%80%99s%20Docker%20Gatling%20Gun%20Campaign https://www.linkedin.com/shareArticle?mini=true&url=https://www.aquasec.com/blog/threat-alert-teamtnts-docker-gatling-gun-campaign/&title=TeamTNT%E2%80%99s%20Docker%20Gatling%20Gun%20Campaign
