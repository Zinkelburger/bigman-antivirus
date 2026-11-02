---
url: "https://thehackernews.com/2024/10/notorious-hacker-group-teamtnt-launches.html"
title: "Notorious Hacker Group TeamTNT Launches New Cloud Attacks for Crypto Mining"
site: "The Hacker News"
captured: "2026-11-02T18:47:22Z"
via: "live"
sha256: "cf3150346bc4a701ab540e31c00546f11c731d55ab4392c2daf89f01230f9a59"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# Notorious Hacker Group TeamTNT Launches New Cloud Attacks for Crypto Mining

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://thehackernews.com/2024/10/notorious-hacker-group-teamtnt-launches.html>

# [Notorious Hacker Group TeamTNT Launches New Cloud Attacks for Crypto Mining](https://thehackernews.com/2024/10/notorious-hacker-group-teamtnt-launches.html)

Ravie LakshmananOct 26, 2024Cloud Security / Cryptocurrency

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEitrezLabESR5-VGZ06cIjmOmAWM5mNKzHgZheuyRWaFj2b0h-Vf3_bOpTfzigRO8jnoB1pi7aSWvi-IBc6WO42g4VzjYeZ-gTaFKTLTSt83cold7gnkU-cTLp7_cl9dGS7zrAgSztSn2IDNt4HAX65E4ifTaC5cqMYUgz8E3N080keDN5iF1X_GmjZlg-K/s1700-nu-rw-lo-l85-e365/coker.png

The infamous cryptojacking group known as [TeamTNT](https://thehackernews.com/2024/09/new-teamtnt-cryptojacking-campaign.html) appears to be readying for a new large-scale campaign targeting cloud-native environments for mining cryptocurrencies and renting out breached servers to third-parties.

"The group is currently targeting exposed Docker daemons to deploy Sliver malware, a cyber worm, and cryptominers, using compromised servers and Docker Hub as the infrastructure to spread their malware," Assaf Morag, director of threat intelligence at cloud security firm Aqua, [said](https://www.aquasec.com/blog/threat-alert-teamtnts-docker-gatling-gun-campaign/) in a report published Friday.

The attack activity is once again a testament to the threat actor's persistence and its ability to evolve its tactics and mounting multi-stage assaults with the goal of compromising Docker environments and enlisting them into a Docker Swarm.

https://thehackernews.uk/ai-governance-platform-d

Besides using Docker Hub to host and distribute their malicious payloads, TeamTNT has been observed offering the victims' computational power to other parties for illicit cryptocurrency mining, thus diversifying its monetization strategy.

Rumblings of the attack campaign emerged earlier this month when Datadog [disclosed](https://thehackernews.com/2024/10/new-cryptojacking-attack-targets-docker.html) malicious attempts to corral infected Docker instances into a Docker Swarm, alluding it could be the work of TeamTNT, while also stopping short of making a formal attribution. But the full extent of the operation hasn't been clear, until now.

Morag told The Hacker News that Datadog "found the infrastructure in a very early stage" and that their discovery "forced the threat actor to change the campaign a bit."

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEgZI7bb_YkDCrHxahMDIV_7Qe-8U4d0yetgxgXru9fgD2lbsSZqo2MSqa4nwUH0g_s2H3j5GbQhDd4a68d6sKPO-wCZJlHJgIcG5GWLwztwiQXo4xbqKsZBr4OkdzzybEPZTmPtuReZ6B3dEjaISivF58GRqVKSw4Ida6-Q3Gu1eO9RZgpUQdiHiJY0FM-L/s1700-nu-rw-lo-l85-e365/docker.png

The attacks entail identifying unauthenticated and exposed Docker API endpoints using masscan and ZGrab and using them for cryptominer deployment and selling the compromised infrastructure to others on a mining rental platform called Mining Rig Rentals, effectively offloading the job of having to manage them themselves, a sign of the maturation of the illicit business model.

Specifically, this is carried out by means of an attack script that scans for Docker daemons on ports 2375, 2376, 4243, and 4244 across nearly 16.7 million IP addresses. It subsequently deploys a container running an Alpine Linux image with malicious commands.

The image, retrieved from a compromised Docker Hub account ("nmlm99") under their control, also executes an initial shell script named the Docker Gatling Gun ("TDGGinit.sh") to launch post-exploitation activities.

One notable change observed by Aqua is the shift away from the Tsunami backdoor to the open-source [Sliver](https://thehackernews.com/2023/01/threat-actors-turn-to-sliver-as-open.html) command-and-control (C2) framework for remotely commandeering the infected servers.

"Additionally, TeamTNT continues to use their established naming conventions, such as Chimaera, TDGG, and bioset (for C2 operations), which reinforces the idea that this is a classic TeamTNT campaign," Morag said.

https://thehackernews.uk/network-defense-dd

"In this campaign TeamTNT is also using anondns (AnonDNS or Anonymous DNS is a concept or service designed to provide anonymity and privacy when resolving DNS queries), in order to point to their web server."

The findings come as Trend Micro shed light on a new campaign that involved a targeted brute-force attack against an unnamed customer to deliver the [Prometei](https://thehackernews.com/2023/03/new-version-of-prometei-botnet-infects.html) crypto mining botnet.

"Prometei spreads in the system by exploiting vulnerabilities in Remote Desktop Protocol (RDP) and Server Message Block (SMB)," the company [said](https://www.trendmicro.com/en_us/research/24/j/unmasking-prometei-a-deep-dive-into-our-mxdr-findings.html), highlighting the threat actor's efforts on setting up persistence, evading security tools, and gaining deeper access to an organization's network through credential dumping and lateral movement.

"The affected machines connect to a mining pool server which can be used to mine cryptocurrencies (Monero) on compromised machines without the victim's knowledge."

Found this article interesting? Follow us on [Google News](https://news.google.com/publications/CAAqLQgKIidDQklTRndnTWFoTUtFWFJvWldoaFkydGxjbTVsZDNNdVkyOXRLQUFQAQ), [Twitter](https://twitter.com/thehackersnews) and [LinkedIn](https://www.linkedin.com/company/thehackernews/) to read more exclusive content we post.

SHARE [](https://thehackernews.com/2024/10/notorious-hacker-group-teamtnt-launches.html#link_share) [](https://thehackernews.com/2024/10/notorious-hacker-group-teamtnt-launches.html#link_share) [](https://thehackernews.com/2024/10/notorious-hacker-group-teamtnt-launches.html#link_share) 

[Tweet](https://thehackernews.com/2024/10/notorious-hacker-group-teamtnt-launches.html#link_share)

[Share](https://thehackernews.com/2024/10/notorious-hacker-group-teamtnt-launches.html#link_share)

[Share](https://thehackernews.com/2024/10/notorious-hacker-group-teamtnt-launches.html#link_share)

Share

SHARE 

[Cloud security](https://thehackernews.com/search/label/Cloud%20security), [cryptocurrency](https://thehackernews.com/search/label/cryptocurrency), [cryptojacking](https://thehackernews.com/search/label/cryptojacking), [Cyber Crime](https://thehackernews.com/search/label/Cyber%20Crime), [Cyber Threat](https://thehackernews.com/search/label/Cyber%20Threat), [cybersecurity](https://thehackernews.com/search/label/cybersecurity), [Docker Security](https://thehackernews.com/search/label/Docker%20Security), [Malware](https://thehackernews.com/search/label/Malware), [Threat Intelligence](https://thehackernews.com/search/label/Threat%20Intelligence)

⚡ Top Stories This Week

[Leaked LiteLLM Master Key Let Attackers Steal GitHub Tokens and Hijack Release Pipeline](https://thehackernews.com/2026/10/leaked-litellm-master-key-let-attackers.html)

[Chinese Hackers Use AI-Powered ARTEX to Target Over 200 Organizations Worldwide](https://thehackernews.com/2026/10/chinese-hackers-use-ai-powered-artex-to.html)

[TeamViewer Flaw Lets Remote Computers Attack Users Who Connect to Control Them](https://thehackernews.com/2026/10/teamviewer-flaw-lets-remote-computers.html)

[Official Claude Code Extension on Cursor Marketplace Trojanized with Persistent SSH Backdoor](https://thehackernews.com/2026/10/official-claude-code-vs-code-extension.html)

[Binance Suffers Security Breach Affecting $6.3 Billion in Assets, Nearly $3 Billion Frozen](https://thehackernews.com/2026/10/binance-suffers-security-breach.html)

[ThreatsDay: Critical Firewall Flaws, Banking Trojans Rented, Weaponized PDF, Token Theft + 26 More Stories](https://thehackernews.com/2026/10/threatsday-critical-firewall-flaws.html)

[Linux Kernel TCP Flaw With 9.8 CVSS Score Affects Ubuntu, Red Hat and Other Distributions](https://thehackernews.com/2026/10/linux-kernel-tcp-flaw-with-98-cvss.html)

[Cisco Patches Actively Exploited ASA and FTD VPN Flaw That Can Crash Firewalls](https://thehackernews.com/2026/10/cisco-patches-actively-exploited-asa.html)

[China-Linked Hackers Exploited VMware ESX Zero-Day to Reduce Audit Trail Visibility](https://thehackernews.com/2026/10/china-linked-hackers-exploited-vmware.html)

[Attackers Exploit PHP2Shell Flaws in Internet-Exposed Palo Alto Networks Management Interfaces](https://thehackernews.com/2026/10/attackers-exploit-php2shell-flaws-in.html)

[PHP2Shell Attack Roots 25 Servers, Breaches U.S. Government-Affiliated Network](https://thehackernews.com/2026/10/php2shell-attack-roots-25-servers.html)

[Critical 16-Year-Old UEFI Flaw Can Run Code Before Secure Boot on Network-Booting Servers](https://thehackernews.com/2026/10/critical-16-year-old-uefi-flaw-can-run.html)

[Anthropic's Unreleased AI Model Chains Flaws Into Working Chrome, Safari, Windows and Linux Exploits](https://thehackernews.com/2026/10/anthropics-unreleased-ai-model-chains.html)

[CyberStrikeAI Uses 44 AI Agents to Automate Hacking Operations Against Japan Targets](https://thehackernews.com/2026/10/cyberstrikeai-uses-44-ai-agents-to.html)

[GoAhead TLS Flaws Allow Revoked Client Certificates and Enable Session Decryption](https://thehackernews.com/2026/10/goahead-tls-flaws-allow-revoked-client.html)

[Microsoft Authenticator Flaw Let Permissionless Android Apps Steal Work Account Tokens](https://thehackernews.com/2026/10/microsoft-authenticator-flaw-let.html)

[PHP Issues Second Windows Security Update in Five Days to Fix Brotli Code Execution Flaw](https://thehackernews.com/2026/10/php-issues-second-windows-security.html)

[Microsoft 365 Token-Replay Attacks Compromise 97 Accounts at 79 Organizations](https://thehackernews.com/2026/10/microsoft-365-token-replay-attacks.html)

[Unpatched Tailscale SSH Flaws Could Let a Hacked Device Reach Root on Other Servers](https://thehackernews.com/2026/10/unpatched-tailscale-ssh-flaws-could-let.html)

[Brotli Decoder Flaw Behind Node.js RCE Also Affects Bun, Python and Other Widely Used Software](https://thehackernews.com/2026/10/brotli-decoder-flaw-behind-nodejs-rce.html)

[Why Urgent Security Fixes Aren't Fixed. Observe, Prove, Act, Confirm Gets Them Moving](https://thehackernews.com/2026/10/why-urgent-security-fixes-arent-fixed.html)

[The Rise of the Threat Operator: How AI Will Redefine Security Operations and Careers](https://thehackernews.com/2026/10/the-rise-of-threat-operator-how-ai-will.html)

[Wazuh and AI for Enhanced SOC Workflows](https://thehackernews.com/2026/10/wazuh-and-ai-for-enhanced-soc-workflows.html)

[American Organizations Under Attack: Skyw4lker Device Code Phishing Campaign Exposed](https://thehackernews.com/2026/10/american-organizations-under-attack.html)

[AI-Generated Fake Login Pages Are Making Account Takeovers Easier](https://thehackernews.com/2026/10/ai-generated-fake-login-pages-are.html)

⭐ Featured Resources

[Detect Lateral Movement and Evasive Attacks With Corelight's Open NDR Platform](https://thehackernews.uk/corelight-defense)

[Assess Your Organization's AI Code Sprawl Exposure With This Practical Guide](https://thehackernews.uk/tame-wild-code)

[See How Bitdefender GravityZone Identifies Shadow AI and Prioritizes Endpoint Risks](https://thehackernews.uk/control-visibility)

[Beyond CVE Scanning: Book a Demo of Continuous AI-Powered Web Pentesting](https://thehackernews.uk/continuous-security)
