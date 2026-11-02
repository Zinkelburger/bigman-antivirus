---
url: "https://www.kaspersky.com/about/press-releases/kaspersky-uncovers-dero-crypto-miner-spreading-via-exposed-container-environments"
title: "Kaspersky uncovers Dero crypto miner spreading via exposed container environments"
site: "/"
published: "2025-05-21T13:00:00Z"
captured: "2026-11-02T18:56:03Z"
via: "live"
sha256: "6a508d5689d9865543e456cde4cdf600c5b22501c99ba641980ec54655b4dfba"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Kaspersky uncovers Dero crypto miner spreading via exposed container environments

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.kaspersky.com/about/press-releases/kaspersky-uncovers-dero-crypto-miner-spreading-via-exposed-container-environments>

# Kaspersky uncovers Dero crypto miner spreading via exposed container environments

May 21, 2025

Kaspersky Security Services experts have identified a sophisticated cyberattack campaign targeting containerized environments to deploy a miner for the Dero cryptocurrency. The attackers abuse exposed Docker APIs — parts of Docker, an open-source container development platform. In 2025, there are a significant number of Docker API default ports that are insecurely published, accounting for almost 500 occurrences worldwide on average each month. In the discovered campaign, cybercriminals inject two types of malwares into the compromised systems: one is the miner itself and the other is a propagation malware that can spread the campaign to other insecure container networks.

Kaspersky experts discovered this malicious campaign as part of a compromise assessment project. According to expert estimates, any organization that operates containerized infrastructure — while exposing Docker APIs without robust security controls — can be a potential target. These may include technology companies, software development firms, hosting providers, cloud service providers and more enterprises.

According to Shodan, in 2025, there are 485 published Docker API default ports[[1]](https://www.kaspersky.com/about/press-releases/kaspersky-uncovers-dero-crypto-miner-spreading-via-exposed-container-environments#_ftn1) worldwide each month on average. This figure illustrates the campaign’s potential attack surface by tallying the “entry points” — or insecurely exposed ports that attackers might target. China accounted for the largest average number monthly — nearly 138 occurrences — followed by Germany (97), the U.S. (58), Brazil (16), and Singapore (13).

Once attackers identify an insecurely published Docker API, they either compromise existing containers or create new malicious ones based on a legitimate standard Ubuntu image. They then inject two malware types into the compromised containers: “nginx” and “cloud”. The latter is a Dero cryptocurrency miner, while “nginx” is a malicious software that maintains persistence, ensures execution of the miner and scans for other exposed environments. This malware allows attackers to operate without traditional Command-and-Control (C2) servers; instead, each infected container independently scans the internet and can spread the miner to new targets.

An infection chain scheme

“This demonstrates that the campaign has the potential for exponential growth of infections, with each compromised container acting as a new source of attack, if security measures are not immediately put in place in the potentially targeted networks,” explains Amged Wageh, an incident response and a compromise assessment expert at Kaspersky Security Services. “Сontainers are foundational to software development, deployment, and scalability. Their widespread use across cloud-native environments, DevOps, and microservices architectures makes them an attractive target for cyber attackers. This growing reliance demands organizations adopt a 360-degree approach to security — combining robust security solutions with proactive threat hunting and regular compromise assessments”.

The attackers embedded the names “nginx” and “cloud” directly in the binary — an inflexible executable file composed of instructions and data for the processor, not for humans. This is a classic masquerading tactic that lets the payload pose as a legitimate tool, trying to deceive both analysts and automated defenses.

The full technical analysis is available [on Securelist](https://securelist.com/dero-miner-infects-containers-through-docker-api/116546/). Kaspersky products detect these malicious implants with the following verdicts: Trojan.Linux.Agent.gen and RiskTool.Linux.Miner.gen.
To mitigate against container-related threats, Kaspersky recommends:

● Companies that use Docker APIs should immediately review the security of any potentially exposed infrastructure — specifically, refrain from publishing the Docker APIs unless there is an operational need and consider securing the published Docker APIs via [TLS](https://docs.docker.com/engine/security/protect-access/).

● Uncover active cyberattacks and previous unknown attacks that flew under the radar with [Kaspersky Compromise Assessment](https://www.kaspersky.com/enterprise-security/compromise-assessment).

● Containerization is the most popular application development method at the moment. But risks can emerge in each component of a container’s infrastructure and may heavily impact business processes. The protection of containerized environments is crucial and requires specialized security solutions.[Kaspersky Container Security](https://www.kaspersky.com/enterprise-security/container-security) provides security for all stages of containerized application development. Besides the development process, the solution protects runtime, for example, it controls the launch of only trusted containers, the operation of the applications and services inside the containers and monitors the traffic.

● Adopt managed security services by Kaspersky such as[Compromise Assessment](https://www.kaspersky.com/enterprise-security/compromise-assessment),[Managed Detection and Response](https://www.kaspersky.com/enterprise-security/managed-detection-and-response) (MDR) and / or [Incident Response](https://www.kaspersky.com/enterprise-security/incident-response), covering the entire incident management cycle – from threat identification to continuous protection and remediation. They help to protect against evasive cyberattacks, investigate incidents and get additional expertise even if a company lacks cybersecurity workers.

The Security Services​

Delivering hundreds of information security projects every year for Fortune 500 organizations worldwide: incident response, managed detection, SOC consulting, red teaming, penetration testing, application security, digital risks protection.

[[1]](https://www.kaspersky.com/about/press-releases/kaspersky-uncovers-dero-crypto-miner-spreading-via-exposed-container-environments#_ftnref1) Analysis includes insecurely published Docker’s default API port 2375
