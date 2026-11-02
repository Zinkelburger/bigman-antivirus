---
url: "https://thehackernews.com/2025/09/tor-based-cryptojacking-attack-expands.html"
title: "TOR-Based Cryptojacking Attack Expands Through Misconfigured Docker APIs"
site: "The Hacker News"
captured: "2026-11-02T18:48:53Z"
via: "live"
sha256: "4a3fecc5c3b210fd447da2fdfe17404dd5181be5d6e6e7fcf6d59bbe8354f8b9"
wayback: "https://web.archive.org/web/20260921115023/https://thehackernews.com/2025/09/tor-based-cryptojacking-attack-expands.html"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# TOR-Based Cryptojacking Attack Expands Through Misconfigured Docker APIs

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://thehackernews.com/2025/09/tor-based-cryptojacking-attack-expands.html>
> Snapshot: <https://web.archive.org/web/20260921115023/https://thehackernews.com/2025/09/tor-based-cryptojacking-attack-expands.html>

# [TOR-Based Cryptojacking Attack Expands Through Misconfigured Docker APIs](https://thehackernews.com/2025/09/tor-based-cryptojacking-attack-expands.html)

Ravie LakshmananSep 09, 2025Cloud Security / Malware

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEgTyVEL-sVGK04WfS6I9fiQcWLnTLiP7kInJxyq-ngjBUXeaVY-5Qz28Ry2RAYdhfxpYZQj7_7Y6QM5k4MLyWLd7MB8fI8rnJ-9m6AhwT8DymYkZvNWxX3TGJYWWNyYBzf2xpV3nHV6X1fCVgeZy1jjUv4hxA6xWgf1teDdEsImJh15RViyra8BVcBOXvlZ/s1700-nu-rw-lo-l85-e365/docker.jpg

Cybersecurity researchers have discovered a variant of a recently disclosed campaign that abuses the TOR network for cryptojacking attacks targeting exposed Docker APIs.

Akamai, which discovered the latest activity last month, said it's designed to block other actors from accessing the Docker API from the internet.

The findings build on a [prior report](https://thehackernews.com/2025/06/hackers-exploit-misconfigured-docker.html) from Trend Micro in late June 2025, which uncovered a malicious campaign that targeted exposed Docker instances to stealthily drop an XMRig cryptocurrency miner using a TOR domain for anonymity.

"This new strain seems to use similar tooling to the original, but may have a different end goal – including possibly setting up the foundation of a complex botnet," security researcher Yonatan Gilvarg [said](https://www.akamai.com/blog/security-research/new-malware-targeting-docker-apis-akamai-hunt).

The attack chain essentially involves breaking into misconfigured Docker APIs to execute a new container based on the Alpine Docker image and mount the host file system into it. This is followed by the threat actors running a Base64-encoded payload to download a shell script downloader from a .onion domain.

https://thehackernews.uk/ai-governance-platform-d

The script, besides altering SSH configurations to set up persistence, also installs other tools such as masscan, libpcap, libpcap-dev, zstd, and torsocks to conduct reconnaissance, contact a command-and-control (C2) server, and download a compressed binary from a second .onion domain.

"The first file that is downloaded is a dropper written in Go that includes the content it wants to drop, so it won't communicate out to the internet," Gilvarg explained. "Except for dropping another binary file, it parses the utmp file to find who is currently logged in to the machine."

Interestingly, the binary file's source code includes an emoji to depict users who are signed in to the system. This indicates that the artifact may have been crafted using a large language model (LLM).

The dropper also launches Masscan to scan the internet for open Docker API services at port 2375 and propagate the infection to those machines by repeating the same process of creating a container with the Base64 command.

Furthermore, the binary includes checks for two more ports: 23 ([Telnet](https://en.wikipedia.org/wiki/Telnet)) and 9222 ([remote debugging port](https://developers.google.com/cast/docs/debugging/remote_debugger) for Chromium browsers), although the functionality to spread via those ports is yet to be fully fleshed out.

The Telnet attack method entails using a set of known, default routers and device credentials to brute-force logins and exfiltrate successful sign-in attempts to a webhook[.]site endpoint with details about the destination IP address and victim authentication credentials.

In the case of port 9222, the malware utilizes a Go library named chromedp to interact with the web browser. It has been previously weaponized by [North Korean threat actors](https://thehackernews.com/2023/12/kimsuky-hackers-deploying-appleseed.html) to communicate with C2 servers and even by [stealer malware](https://redcanary.com/blog/threat-intelligence/google-chrome-app-bound-encryption/) to bypass Chrome's [app-bound encryption](https://thehackernews.com/2025/05/eddiestealer-malware-uses-clickfix.html), connect remotely to Chromium sessions, and siphon cookies and other private data.

It then proceeds to attach to an existing session with the open remote port and ultimately send a POST to the same .onion domain used to retrieve the shell script downloader with information about the source IP address on which the malware is and the destination it found access to on port 9222.

The details are transmitted to an endpoint named "httpbot/add," raising the possibility that devices with exposed remote debugging ports for Chrome/Chromium could be enlisted into a botnet for delivering additional payloads that can steal data or be used to conduct distributed denial-of-service (DDoS) attacks.

"As the malware only scans for port 2375, the logic for handling ports 23 and 9222 is currently unreachable and will not be executed," Gilvarg said. "However, the implementation exists, which may indicate future capabilities."

"Attackers can gain significant control over systems affected by abused APIs. The importance of segmenting networks, limiting exposure of services to the internet, and securing default credentials cannot be overstated. By adopting these measures, organizations can significantly reduce their vulnerability to such threats."

### Wiz Flags AWS SES Abuse Campaign

The disclosure comes as cloud security firm Wiz detailed an Amazon Simple Email Service (SES) campaign in May 2025 that leveraged compromised Amazon Web Services (AWS) access keys as a launchpad for a mass phishing attack.

https://thehackernews.uk/network-defense-dd

It's currently not known how the keys were obtained. However, various methods exist by which an attacker can accomplish this: accidental public exposure in code repositories or through misconfigured assets, or theft from a developer workstation using stealer malware.

"The attacker used the compromised key to access the victim's AWS environment, bypass SES's built-in restrictions, verify new 'sender' identities, and methodically prepare and conduct a phishing operation," Wiz researchers Itay Harel and Hila Ramati [said](https://www.wiz.io/blog/wiz-discovers-cloud-email-abuse-campaign).

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEh-cUFWQFI4Smys1sAFHhNHecr7NK-yPBEMkD_IpBlYDYW66Ok1rfKpfjrBIS22wPkHG38K77yqakDNEEQtpT1pUEIvHgGAzIiEpOfaYwW1cuxdIGSOW0lPpJqmvA4iu1DKuk2-uUhylq3SwFn_q8GUx2LbrBZIpppXIzE9Vy2GIlixPCUt2zEfl5rdRF83/s1700-nu-rw-lo-l85-e365/wiz.png

Wiz, which further probed the phishing campaign in partnership with Proofpoint, said the emails targeted several organizations spanning multiple geographies and sectors, and employed tax-themed lures to redirect recipients to credential harvesting pages.

"If SES is configured in your account, attackers can send email from your verified domains," Wiz cautioned. "Beyond brand damage, this enables phishing that looks like it came from you and can be used for spearphishing, fraud, data theft, or masquerading in business processes."

When reached for comment, an AWS spokesperson told The Hacker News that "As always, we encourage all customers to follow recommended [security guidance](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-security.html) to secure their accounts and prevent abuse. If anyone suspects that AWS resources are being used for abusive activity, they can report it using the [report abuse form](https://support.aws.amazon.com/#/contacts/report-abuse)."

Found this article interesting? Follow us on [Google News](https://news.google.com/publications/CAAqLQgKIidDQklTRndnTWFoTUtFWFJvWldoaFkydGxjbTVsZDNNdVkyOXRLQUFQAQ), [Twitter](https://twitter.com/thehackersnews) and [LinkedIn](https://www.linkedin.com/company/thehackernews/) to read more exclusive content we post.

SHARE [](https://thehackernews.com/2025/09/tor-based-cryptojacking-attack-expands.html#link_share) [](https://thehackernews.com/2025/09/tor-based-cryptojacking-attack-expands.html#link_share) [](https://thehackernews.com/2025/09/tor-based-cryptojacking-attack-expands.html#link_share) 

[Tweet](https://thehackernews.com/2025/09/tor-based-cryptojacking-attack-expands.html#link_share)

[Share](https://thehackernews.com/2025/09/tor-based-cryptojacking-attack-expands.html#link_share)

[Share](https://thehackernews.com/2025/09/tor-based-cryptojacking-attack-expands.html#link_share)

Share

SHARE 

[AWS](https://thehackernews.com/search/label/AWS), [botnet](https://thehackernews.com/search/label/botnet), [Cloud security](https://thehackernews.com/search/label/Cloud%20security), [cryptojacking](https://thehackernews.com/search/label/cryptojacking), [cybersecurity](https://thehackernews.com/search/label/cybersecurity), [Docker](https://thehackernews.com/search/label/Docker), [Malware](https://thehackernews.com/search/label/Malware), [Phishing](https://thehackernews.com/search/label/Phishing), [Proofpoint](https://thehackernews.com/search/label/Proofpoint), [Tor network](https://thehackernews.com/search/label/Tor%20network)

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
