---
url: "https://thehackernews.com/2026/02/researchers-find-341-malicious-clawhub.html"
title: "Researchers Find 341 Malicious ClawHub Skills Stealing Data from OpenClaw Users"
site: "The Hacker News"
captured: "2026-11-04T17:36:31Z"
via: "live"
sha256: "cde6bdf93588d41f85a1020d304b9a0ab3daee6c4586af13409579e0ef4e2f7b"
cited_in:
  - "docs/research/agent-skill-security.md"
  - "docs/threats/02-supply-chain-packages.md"
---

# Researchers Find 341 Malicious ClawHub Skills Stealing Data from OpenClaw Users

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://thehackernews.com/2026/02/researchers-find-341-malicious-clawhub.html>

# [Researchers Find 341 Malicious ClawHub Skills Stealing Data from OpenClaw Users](https://thehackernews.com/2026/02/researchers-find-341-malicious-clawhub.html)

Ravie LakshmananFeb 02, 2026Malware / Artificial Intelligence

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEhrHsgrh_iIHVwJdfZ-6tSVag1fs71bd15QISN82bzqvxchcyJGuWdbtZRqQ25tJC_HoeaGAMI3phpFy_TAy2NYbSpJTwuTJ5jIz_QKmNPTv_zqA2GNvuozLGpKCqTa-OI1Hp-zOy_B8wjkvoYcBn56g09PpoQOMd568LfGiH7J0Ee0hg65eBaHSWxDe92m/s1700-nu-rw-lo-l85-e365/OpenClaw-skills.jpg

A security audit of 2,857 skills on ClawHub has found 341 malicious skills across multiple campaigns, according to [new findings](https://www.koi.ai/blog/clawhavoc-341-malicious-clawedbot-skills-found-by-the-bot-they-were-targeting) from Koi Security, exposing users to new supply chain risks.

[ClawHub](https://www.clawhub.ai/skills) is a marketplace designed to make it easy for [OpenClaw](https://thehackernews.com/2026/02/openclaw-bug-enables-one-click-remote.html) users to find and install third-party skills. It's an extension to the OpenClaw project, a self-hosted artificial intelligence (AI) assistant formerly known as both Clawdbot and Moltbot.

The analysis, which Koi conducted with the help of an OpenClaw bot named Alex, found that 335 skills use fake pre-requisites to install an Apple macOS stealer named [Atomic Stealer](https://thehackernews.com/2025/06/new-atomic-macos-stealer-campaign.html) (AMOS). This activity set has been codenamed ClawHavoc.

"You install what looks like a legitimate skill – maybe solana-wallet-tracker or youtube-summarize-pro," Koi researcher Oren Yomtov said. "The skill's documentation looks professional. But there's a 'Prerequisites' section that says you need to install something first."

This step involves instructions for both Windows and macOS systems: On Windows, users are asked to download a file called "openclaw-agent.zip" from a GitHub repository. On macOS, the documentation tells them to copy an installation script hosted at glot[.]io and paste it into the Terminal app. The targeting of macOS is no coincidence, as reports have [emerged](https://www.businessinsider.com/clawdbot-ai-mac-mini-2026-1) of people [buying Mac Minis](https://mashable.com/article/jan-31-apple-mac-mini-deal) to run the AI assistant 24x7.

Present within the password-protected archive is a trojan with keylogging functionality to capture API keys, credentials, and other sensitive data on the machine, including those that the bot already has access to. On the other hand, the glot[.]io script contains obfuscated shell commands to fetch next-stage payloads from an attacker-controlled infrastructure.

This, in turn, entails reaching out to another IP address ("91.92.242[.]30") to retrieve another shell script, which is configured to contact the same server to obtain a universal Mach-O binary that exhibits traits consistent with Atomic Stealer, a commodity stealer available for $500-1000/month that can harvest data from macOS hosts.

https://thehackernews.uk/ai-governance-platform-d

According to Koi, the malicious skills masquerade as

- ClawHub typosquats (e.g., clawhub, clawhub1, clawhubb, clawhubcli, clawwhub, cllawhub)
- Cryptocurrency tools like Solana wallets and wallet trackers
- Polymarket bots (e.g., polymarket-trader, polymarket-pro, polytrading)
- YouTube utilities (e.g., youtube-summarize, youtube-thumbnail-grabber, youtube-video-downloader)
- Auto-updaters (e.g., auto-updater-agent, update, updater)
- Finance and social media tools (e.g., yahoo-finance-pro, x-trends-tracker)
- Google Workspace tools claiming integrations with Gmail, Calendar, Sheets, and Drive
- Ethereum gas trackers
- Lost Bitcoin finders

In addition, the cybersecurity company said it identified skills that hide reverse shell backdoors inside functional code (e.g., better-polymarket and polymarket-all-in-one), or exfiltrate bot credentials present in "~/.clawdbot/.env" to a webhook[.]site (e.g., rankaj).

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEhdUIlSL9qpCVbfy1TNUn-HizuE9n6MdFG5YHsHFM9xPa-VGEUo9aaIOAYFIngxa3eySnljrSlNEswxcdWoqbVIa1r2QMMl6PKzfprbnsb-8Gr4wIa-CElQxwA-lufP6EutwkSLX5Zh-C-3-KiLa61nzkZF1nIw3_CVdEQ290WxL7gAXniTyHy2c5irJ_be/s1700-nu-rw-lo-l85-e365/skill-malware-2.png

The development coincides with a report from OpenSourceMalware, which also flagged the same ClawHavoc campaign targeting OpenClaw users.

"The skills masquerade as cryptocurrency trading automation tools and deliver information-stealing malware to macOS and Windows systems," security researcher Paul McCarty, who goes by the online alias 6mile, [said](https://opensourcemalware.com/blog/clawdbot-skills-ganked-your-crypto).

"All these skills share the same command-and-control infrastructure (91.92.242[.]30) and use sophisticated social engineering to convince users to execute malicious commands, which then steal crypto assets like exchange API keys, wallet private keys, SSH credentials, and browser passwords."

## OpenClaw Adds a Reporting Option

The problem stems from the fact that ClawHub is open by default and allows anyone to upload skills. The only restriction at this stage is that a publisher must have a GitHub account that's at least one week old.

The issue with malicious skills [hasn't gone unnoticed](https://x.com/steipete/status/2018297794952651111) by OpenClaw's creator Peter Steinberger, who has since rolled out a reporting feature that allows signed-in users to flag a skill. "Each user can have up to 20 active reports at a time," the documentation [states](https://docs.openclaw.ai/tools/clawhub#security-and-moderation). "Skills with more than 3 unique reports are auto-hidden by default."

https://thehackernews.uk/network-defense-dd

The findings underscore how open-source ecosystems continue to be abused by threat actors, who are now piggybacking on OpenClaw's sudden popularity to orchestrate malicious campaigns and distribute malware at scale.

In a report last week, Palo Alto Networks warned that OpenClaw represents what British programmer Simon Willison, who coined the term prompt injection, describes as a "[lethal trifecta](https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/)" that renders AI agents vulnerable by design due to their access to private data, exposure to untrusted content, and the ability to communicate externally.

The intersection of these three capabilities, combined with OpenClaw's persistent memory, "acts as an accelerant" and amplifies the risks, the cybersecurity company added.

"With persistent memory, attacks are no longer just point-in-time exploits. They become stateful, delayed-execution attacks," researchers Sailesh Mishra and Sean P. Morgan [said](https://www.paloaltonetworks.com/blog/network-security/why-moltbot-may-signal-ai-crisis/). "Malicious payloads no longer need to trigger immediate execution on delivery. Instead, they can be fragmented, untrusted inputs that appear benign in isolation, are written into long-term agent memory, and later assembled into an executable set of instructions."

"This enables time-shifted prompt injection, memory poisoning, and logic bomb–style activation, where the exploit is created at ingestion but detonates only when the agent’s internal state, goals, or tool availability align."

Found this article interesting? Follow us on [Google News](https://news.google.com/publications/CAAqLQgKIidDQklTRndnTWFoTUtFWFJvWldoaFkydGxjbTVsZDNNdVkyOXRLQUFQAQ), [Twitter](https://twitter.com/thehackersnews) and [LinkedIn](https://www.linkedin.com/company/thehackernews/) to read more exclusive content we post.

SHARE [](https://thehackernews.com/2026/02/researchers-find-341-malicious-clawhub.html#link_share) [](https://thehackernews.com/2026/02/researchers-find-341-malicious-clawhub.html#link_share) [](https://thehackernews.com/2026/02/researchers-find-341-malicious-clawhub.html#link_share) 

[Tweet](https://thehackernews.com/2026/02/researchers-find-341-malicious-clawhub.html#link_share)

[Share](https://thehackernews.com/2026/02/researchers-find-341-malicious-clawhub.html#link_share)

[Share](https://thehackernews.com/2026/02/researchers-find-341-malicious-clawhub.html#link_share)

Share

SHARE 

[artificial intelligence](https://thehackernews.com/search/label/artificial%20intelligence), [cryptocurrency](https://thehackernews.com/search/label/cryptocurrency), [cybersecurity](https://thehackernews.com/search/label/cybersecurity), [MacOS](https://thehackernews.com/search/label/MacOS), [Malware](https://thehackernews.com/search/label/Malware), [Open Source](https://thehackernews.com/search/label/Open%20Source), [Supply Chain Security](https://thehackernews.com/search/label/Supply%20Chain%20Security), [Threat Research](https://thehackernews.com/search/label/Threat%20Research), [windows security](https://thehackernews.com/search/label/windows%20security)

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
