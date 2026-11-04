---
url: "https://thehackernews.com/2026/05/quasar-linux-rat-steals-developer.html"
title: "Quasar Linux RAT Steals Developer Credentials for Software Supply Chain Compromise"
site: "The Hacker News"
captured: "2026-11-02T18:50:58Z"
via: "live"
sha256: "3c9e501bc3482bbba7c7a3fa93682609882216c1dae1b69ec8acc8b057f055aa"
wayback: "https://web.archive.org/web/20260618013919/https://thehackernews.com/2026/05/quasar-linux-rat-steals-developer.html"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# Quasar Linux RAT Steals Developer Credentials for Software Supply Chain Compromise

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://thehackernews.com/2026/05/quasar-linux-rat-steals-developer.html>
> Snapshot: <https://web.archive.org/web/20260618013919/https://thehackernews.com/2026/05/quasar-linux-rat-steals-developer.html>

# [Quasar Linux RAT Steals Developer Credentials for Software Supply Chain Compromise](https://thehackernews.com/2026/05/quasar-linux-rat-steals-developer.html)

Ravie LakshmananMay 08, 2026Linux / DevOps

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEiholjenZRIykmReErkRiguk5xd9RV4BIEEPM0nT-o3LvMvDkCTLpd3G0NpqDGEFHp-f6QyvGRMip6CBhGlllYVlp9wS3XBVoV6xW47CDka7Ig8S_aotcuNlmAv3SYgS4hJzxjLp2nrV4SzqlTXnQLG_w68Cq0Bf5hiOoV6CaN9QZliRDa-StzsvIkJAdSF/s1700-nu-rw-lo-l85-e365/kube.jpg

A previously undocumented Linux implant codenamed Quasar Linux RAT (QLNX) is targeting developers' systems to establish a silent foothold as well as facilitate a broad range of post-compromise functionality, such as credential harvesting, keylogging, file manipulation, clipboard monitoring, and network tunneling.

"QLNX targets developers and DevOps credentials across the software supply chain," Trend Micro researchers Aliakbar Zahravi and Ahmed Mohamed Ibrahim [said](https://www.trendmicro.com/en_us/research/26/e/quasar-linux-qlnx-a-silent-foothold-in-the-software-supply-chain.html) in a technical analysis of the malware.

"Its credential harvester extracts secrets from high-value files such as .npmrc (npm tokens), .pypirc (PyPI credentials), .git-credentials, .aws/credentials, .kube/config, .docker/config.json, .vault-token, Terraform credentials, GitHub CLI tokens, and .env files. The compromise of these assets could allow the operator to push malicious packages to NPM or PyPI registries, access cloud infrastructure, or pivot through CI/CD pipelines."

https://thehackernews.uk/ai-governance-platform-d

The malware's ability to systematically harvest a wide range of credentials poses a severe risk to developer environments. A threat actor who successfully deploys QLNX against a package maintainer gains unauthorized access to their publishing pipeline, allowing the attacker to push poisoned versions that can lead to cascading downstream impacts.

QLNX executes filelessly from memory, masquerades itself as a kernel thread (e.g., kworker or ksoftirqd), and is capable of profiling the host to detect containerized environments, wiping system logs to cover up the tracks, and setting up persistence using no less than seven different methods, including systemd, crontab, and .bashrc shell injection.

Furthermore, it exfiltrates the collected data to an attacker-controlled infrastructure, and receives commands that make it possible to execute shell commands, manage files, inject code into processes, take screenshots, log keystrokes, establish SOCKS proxies and TCP tunnels, run Beacon Object Files (BOFs), and even manage a peer-to-peer (P2P) mesh network.

Exactly how the malware is delivered is unclear. However, once a foothold is established, it enters a primary operational phase by running a persistent loop that continuously attempts to establish and maintain communication with the command-and-control (C2) server over raw TCP, HTTPS, and HTTP. In total, QLNX supports 58 distinct commands that give the operators complete control of the compromised host.

QLNX also comes with a Pluggable Authentication Module ([PAM](https://thehackernews.com/2026/05/new-linux-pamdoora-backdoor-uses-pam.html)) inline-hook backdoor that intercepts plaintext credentials during authentication events, logs outbound SSH session data, and transmits the data to the C2 server. The malware also supports a second PAM-based credentials logger that's automatically loaded into every dynamically linked process to extract the service name, username, and authentication token.

https://thehackernews.uk/network-defense-dd

It employs a two-tiered rootkit architecture: a userland rootkit deployed through the Linux dynamic linker's LD_PRELOAD mechanism to ensure that the implant's artifacts and processes stay hidden. There also exists a kernel-level eBPF component that uses BPF subsystem to conceal processes, files, and network ports from standard userland tools such as ps, ls, and netstat upon receiving instructions from the C2 server.

"The QLNX implant was built for long-term stealth and credential theft," Trend Micro said. "What makes it particularly dangerous is not any single feature, but how its capabilities chain together into a coherent attack workflow: arrive, erase from disk, persist through six redundant mechanisms, hide at both userspace and kernel level, and then harvest the credentials that matter most."

Found this article interesting? Follow us on [Google News](https://news.google.com/publications/CAAqLQgKIidDQklTRndnTWFoTUtFWFJvWldoaFkydGxjbTVsZDNNdVkyOXRLQUFQAQ), [Twitter](https://twitter.com/thehackersnews) and [LinkedIn](https://www.linkedin.com/company/thehackernews/) to read more exclusive content we post.

SHARE [](https://thehackernews.com/2026/05/quasar-linux-rat-steals-developer.html#link_share) [](https://thehackernews.com/2026/05/quasar-linux-rat-steals-developer.html#link_share) [](https://thehackernews.com/2026/05/quasar-linux-rat-steals-developer.html#link_share) 

[Tweet](https://thehackernews.com/2026/05/quasar-linux-rat-steals-developer.html#link_share)

[Share](https://thehackernews.com/2026/05/quasar-linux-rat-steals-developer.html#link_share)

[Share](https://thehackernews.com/2026/05/quasar-linux-rat-steals-developer.html#link_share)

Share

SHARE 

[Credential Harvesting](https://thehackernews.com/search/label/Credential%20Harvesting), [cybersecurity](https://thehackernews.com/search/label/cybersecurity), [DevOps](https://thehackernews.com/search/label/DevOps), [linux](https://thehackernews.com/search/label/linux), [Remote Access Trojan](https://thehackernews.com/search/label/Remote%20Access%20Trojan), [rootkit](https://thehackernews.com/search/label/rootkit), [Supply Chain](https://thehackernews.com/search/label/Supply%20Chain)

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
