---
url: "https://thehackernews.com/2026/09/compromised-memtensor-packages-deliver.html"
title: "Compromised MemTensor Packages Deliver sckit Credential Stealer via npm and PyPI"
site: "The Hacker News"
captured: "2026-11-02T18:51:15Z"
via: "live"
sha256: "179e3b34bb47f4e00ad6862edc9dc5dca3069c4ff3a26ff083c3e2bcc3d6f748"
wayback: "https://web.archive.org/web/20261023042834/https://thehackernews.com/2026/09/compromised-memtensor-packages-deliver.html"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Compromised MemTensor Packages Deliver sckit Credential Stealer via npm and PyPI

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://thehackernews.com/2026/09/compromised-memtensor-packages-deliver.html>
> Snapshot: <https://web.archive.org/web/20261023042834/https://thehackernews.com/2026/09/compromised-memtensor-packages-deliver.html>

# [Compromised MemTensor Packages Deliver sckit Credential Stealer via npm and PyPI](https://thehackernews.com/2026/09/compromised-memtensor-packages-deliver.html)

Ravie LakshmananSep 23, 2026Malware / Cloud Security

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEhFThFSFsti-2SIka75bNuMKpyJHtOW2ZPrZtcSjbjFQ64GCNn0WtdssYuWlVTbhaLB5cAJ0vu8FgyNmNsDa8g0Ijy-D1zP4FW7ihVfAjk9xWMYDMMdfZPICGyVdjeDwH3-jyKLHOUnjfaXBJIMxGn_3ngeXFsbb4CLnOibRd4fbwXHYpBkMQTcBQAf0edq/s1700-nu-rw-lo-l85-e365/npm-pypi.jpg

Unknown threat actors have managed to compromise two legitimate MemTensor packages across the npm and Python Package Index (PyPI) repositories to push a platform-specific Go-based implant dubbed sckit designed for Windows, Linux, and macOS.

According to reports from [Aikido](https://www.aikido.dev/blog/supplychain-local-memtensor-npm-pypi), [SafeDep](https://safedep.io/memtensor-sckit-worm-npm-pypi/), [Socket](https://socket.dev/blog/memtensor-compromise), and [StepSecurity](https://www.stepsecurity.io/blog/sckit-supply-chain-worm-hits-memtensor-npm-pypi-scopes), the libraries in question below -

- [@memtensor/memos-cloud-openclaw-plugin](https://www.npmjs.com/package/@memtensor/memos-cloud-openclaw-plugin) versions 0.1.21, 0.1.23 and 0.1.25 (versions 0.1.22 and 0.1.24 are clean)
- [MemoryOS](https://pypi.org/project/MemoryOS/) version 2.0.34 (project currently quarantined on PyPI)

The malicious npm package versions include a "hidden Go payload into a legitimate AI memory integration. Versions 0.1.21, 0.1.23, and 0.1.25 contain code that launches the payload when the agent gateway starts and whenever the plugin handles a memory-recall event," StepSecurity said.

https://thehackernews.uk/ai-governance-platform-d

"The launcher passes the host process environment and, during recall, the user's prompt text directly to the malicious executable."

The PyPI package, on the other hand, starts the statically-linked Go binary as soon as the "memos" module is imported into an application.

Regardless of the ecosystem targeted, the end goal is to launch a cross-platform credential-stealing payload capable of harvesting sensitive data from cloud services, source-code platforms, package registries, and developer tools and exfiltrating the details to an external server ("skyleen[.]fr").

According to Socket, targets include npm, PyPI, GitHub, GitLab, AWS, Vault and SSH secrets -

- Credential files (.npmrc, .vault-token, id_ecdsa, credentials.db, access_tokens.json and stored_tokens)
- Environment variables that indicate tokens, passwords, API keys, private keys, session cookies and database or message-broker connection strings (e.g., NPM_TOKEN and PYPI_API_TOKEN)
- AWS access keys, GitHub and GitLab tokens, npm and PyPI tokens, Hugging Face, HashiCorp Vault, Slack, Stripe and SendGrid keys, and JWTs

SafeDep, in its analysis of the supply chain attack, said the attacker obtained the publish tokens from MemTensor's own GitHub Actions release pipelines by pushing commits that caused the workflow to hand over the npm or PyPI token.

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEih9Eq3x05ST2UXweOaS-XCZ1cdO7H_JA7mKniiQTEp3M1QHZBFvppYcCVWEe-GzOc6NzP9ApIc7Kn6adsYciQ3J3Okl185Ji1enErQai6l2G2XZ5AXUWkyU5b523LXogsByGTl-1iw6l_UFXFCYTOiIxtEcPIvGuPmGjVqUnU1fOanLLYbnbzHnnzBuPcN/s1700-nu-rw-lo-l85-e365/sckit.jpg

A deeper examination of the implant suggests that it can function like a worm by self-proliferating through GitHub and direct npm and PyPI package publishing. As of writing, it's unclear if there are packages other than MemTensor that are impacted by the compromise.

"It collects credentials from developer machines and from CI jobs," SafeDep [said](https://safedep.io/sckit-go-implant-framework/). "It receives signed tasks from a command-and-control (C2) server. It also contains templates to install itself in npm packages, Python packages, and GitHub Actions workflows."

https://thehackernews.uk/network-defense-dd

Given that the malicious versions of the npm packages are still available for download, it's essential to pin the packages to a safe baseline version (0.1.20 for the npm package, 2.0.33 for the PyPI package), rotate exposed secrets, kill any sckit process, and block "skyleen[.]fr" and all its subdomains.

"The MemOS Cloud plugin connects the OpenClaw agent runtime to a memory service," StepSecurity said. "Its normal work includes recalling relevant memories before an agent processes a prompt and adding memories after a run. The package also declares integration points for the Clawdbot and Moltbot runtimes."

"This places the plugin inside a process that routinely handles user input and may inherit valuable credentials. On a developer workstation, the same user account can have access to cloud configuration, source repositories, package publishing tokens, and application secrets. In automation, the process may receive credentials injected for a particular job."

### Update

The malicious versions have been taken down from both npm and PyPI. The latest clean versions are. -

- @memtensor/memos-cloud-openclaw-plugin - Version 0.1.24
- MemoryOS - Version 2.0.33

Found this article interesting? Follow us on [Google News](https://news.google.com/publications/CAAqLQgKIidDQklTRndnTWFoTUtFWFJvWldoaFkydGxjbTVsZDNNdVkyOXRLQUFQAQ), [Twitter](https://twitter.com/thehackersnews) and [LinkedIn](https://www.linkedin.com/company/thehackernews/) to read more exclusive content we post.

SHARE [](https://thehackernews.com/2026/09/compromised-memtensor-packages-deliver.html#link_share) [](https://thehackernews.com/2026/09/compromised-memtensor-packages-deliver.html#link_share) [](https://thehackernews.com/2026/09/compromised-memtensor-packages-deliver.html#link_share) 

[Tweet](https://thehackernews.com/2026/09/compromised-memtensor-packages-deliver.html#link_share)

[Share](https://thehackernews.com/2026/09/compromised-memtensor-packages-deliver.html#link_share)

[Share](https://thehackernews.com/2026/09/compromised-memtensor-packages-deliver.html#link_share)

Share

SHARE 

[Cloud security](https://thehackernews.com/search/label/Cloud%20security), [Malware](https://thehackernews.com/search/label/Malware), [Open Source Security](https://thehackernews.com/search/label/Open%20Source%20Security), [Supply Chain](https://thehackernews.com/search/label/Supply%20Chain)

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
