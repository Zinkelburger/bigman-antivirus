---
url: "https://thehackernews.com/2025/10/over-100-vs-code-extensions-exposed.html"
title: "Over 100 VS Code Extensions Exposed Developers to Hidden Supply Chain Risks"
site: "The Hacker News"
captured: "2026-11-02T18:48:57Z"
via: "live"
sha256: "2d4bee9232e9300929572437f471d097dce2af14dc6bfb91048492101d4f78c4"
wayback: "https://web.archive.org/web/20260921114933/https://thehackernews.com/2025/10/over-100-vs-code-extensions-exposed.html"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Over 100 VS Code Extensions Exposed Developers to Hidden Supply Chain Risks

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://thehackernews.com/2025/10/over-100-vs-code-extensions-exposed.html>
> Snapshot: <https://web.archive.org/web/20260921114933/https://thehackernews.com/2025/10/over-100-vs-code-extensions-exposed.html>

# [Over 100 VS Code Extensions Exposed Developers to Hidden Supply Chain Risks](https://thehackernews.com/2025/10/over-100-vs-code-extensions-exposed.html)

Ravie LakshmananOct 15, 2025Software Supply Chain / Malware

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEh7nxODcfvRiRGSDL38ckIG0H-EQiEDgbQ7L09l-4hmze6f8fJxdAGx1sTPtM7qq4RJKe-0hvOxWJnxKElwWIem-mteqIPckj3p3LsPg-qWkBMHgS-hhQvOZZTwXvun2IRsLnjJt4d3nu0orFjiZYNDj_frr4wU6zTId8Lli8pr_yq1I4KbDMj18Je21op9/s1700-nu-rw-lo-l85-e365/vscode.jpg

New research has uncovered that publishers of over 100 Visual Studio Code (VS Code) extensions leaked access tokens that could be exploited by bad actors to update the extensions, posing a critical software supply chain risk.

"A leaked VS Code Marketplace or Open VSX PAT [personal access token] allows an attacker to directly distribute a malicious extension update across the entire install base," Wiz security researcher Rami McCarthy [said](https://www.wiz.io/blog/supply-chain-risk-in-vscode-extension-marketplaces) in a report shared with The Hacker News. "An attacker who discovered this issue would have been able to directly distribute malware to the cumulative 150,000 install base."

The cloud security firm noted in many cases publishers failed to account for the fact that VS Code extensions, while distributed as .vsix files, can be unzipped and inspected, exposing hard-coded secrets embedded into them.

In all, Wiz said it found over 550 validated secrets, distributed across more than 500 extensions from hundreds of distinct publishers. The 550 secrets have been found to fall under 67 distinct types of secrets, including -

- [AI provider secrets](https://www.wiz.io/blog/leaking-ai-secrets-in-public-code), such as those related to OpenAI, Gemini, Anthropic, XAI, DeepSeek, Hugging Face, and Perplexity
- Cloud service provider secrets, such as those related to Amazon Web Services (AWS), Google Cloud, GitHub, Stripe, and Auth0
- Database secrets, such as those related to MongoDB, PostgreSQL, and Supabase

Wiz also noted in its report that more than 100 extensions leaked VS Code Marketplace PATs, which accounted for over 85,000 installs. Another 30 extensions with a cumulative install base of no less than 100,000 have been found to Open VSX Access Tokens. A significant chunk of the flagged extensions are themes.

https://thehackernews.uk/ai-governance-platform-d

With Open VSX also integrated into artificial intelligence (AI)-powered VS Code forks like Cursor and Windsurf, extensions that leak access tokens can significantly expand the attack surface.

In one instance, the company said it identified a VS Code Marketplace PAT that could have allowed for pushing targeted malware to the workforce of a $30 billion market cap Chinese mega corporation, indicating that the problem also extends to internal or vendor-specific extensions used by organizations.

Following responsible disclosure to Microsoft in late March and April 2025, the Windows maker has revoked the leaked PATs and [announced](https://developer.microsoft.com/blog/security-and-trust-in-visual-studio-marketplace) it's adding [secret scanning capabilities](https://github.com/microsoft/vsmarketplace/discussions/1442) to block extensions with verified secrets and notify developers when secrets are detected.

VS Code users are advised to limit the number of installed extensions, scrutinize extensions prior to downloading them, and weigh the pros and cons of enabling auto-updates. Organizations are recommended to develop an extension inventory to better respond to reports of malicious extensions and consider a centralized allowlist for extensions.

"The issue highlights the continued risks of extensions and plugins, and supply chain security in general," Wiz said. "It continues to validate the impression that any package repository carries a high risk of mass secrets leakage."

### TigerJack Targets VS Code Marketplace with Malicious Extensions

The development comes as Koi Security disclosed details of a threat actor codenamed TigerJack that's been attributed to publishing at least 11 legitimate-looking malicious VS Code extensions using various publisher accounts since early 2025 as part of a "coordinated, systematic" campaign.

"Operating under the identities ab-498, 498, and 498-00, Tiger-Jack has deployed a sophisticated arsenal: extensions that steal source code, mine cryptocurrency, and establish remote backdoors for complete system control," security researcher Tuval Admoni [said](https://www.koi.ai/blog/tiger-jack-malicious-vscode-extensions-stealing-code).

Two of the malicious extensions – C++ Playground and HTTP Format – attracted over 17,000 downloads [prior to their takedown](https://github.com/microsoft/vsmarketplace/blob/main/RemovedPackages.md). However, they continue to be available on Open VSX, with the threat actor also republishing the same malicious code on September 17, 2025, under new names on the VS Code Marketplace after removal.

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEjn5tMdlSHWNwJTeydJOmdwH-j67xoXvmL4t2WBjxNV1Fvu-MSs6BNFwrWJmuL-EXCXjS06ZvT7WqhPZkrk2soeWRlbwWJHFpcI4LzvaH4XrhvV4GF1-DevkiB5yZKCg4SOyqUqBXDOUExWAymkDiQmfBIs5xMzEWND7cTMrcqmhkOAvIBqojefaiRnTTPz/s2600/extent.png

What's notable about these extensions is that they deliver the promised functionality, which provides the perfect cover for their malicious activities to go unnoticed by unsuspecting developers who may have installed them.

Specifically, the C++ Playground extension has been found to capture keystrokes in almost real-time through a listener that's triggered after a 500-millisecond delay. The end goal is to steal C++ source code files. On the other hand, the HTTP Format extension harbors nefarious code to run the CoinIMP miner and stealthily mine cryptocurrency by abusing the system resources.

Three other extensions published by TigerJack under the alias "498," namely cppplayground, httpformat, and pythonformat, further escalate the risk by incorporating the ability to act as a backdoor by downloading and running arbitrary JavaScript from an external server ("ab498.pythonanywhere[.]com") every 20 minutes.

https://thehackernews.uk/network-defense-dd

"By checking for new instructions every 20 minutes and using eval() on remotely fetched code, TigerJack can dynamically push any malicious payload without updating the extension—stealing credentials and API keys, deploying ransomware, using compromised developer machines as entry points into corporate networks, injecting backdoors into your projects, or monitoring your activity in real-time," Admoni noted.

Koi Security also pointed out that most of these extensions started off as completely benign tools before the malicious modifications were introduced, a classic case of a Trojan horse approach. This offers several advantages, as it allows the threat actor to establish legitimacy and gain traction among users.

What's more, it can also deceive a developer who may have vetted the extension before installation, as the threat actor could push an update later on to compromise their environment.

In June 2025, Microsoft [said](https://developer.microsoft.com/blog/security-and-trust-in-visual-studio-marketplace) it has a multi-step process in place to keep the VS Code marketplace free of malware. This includes an initial scan of all incoming packages for malicious run-time behavior in a sandbox environment, as well as rescanning and periodic marketplace-wide scans to "make sure everything stays safe."

That said, these security protections only apply to VS Code Marketplace, and not others like the Open VSX registry, meaning even if the malicious extension gets removed from Microsoft's platform, threat actors can easily migrate to less-secure alternatives.

"The fragmented security landscape across all marketplaces creates dangerous blind spots that sophisticated threat actors are already exploiting," the company said. "When security operates in silos, threats simply migrate between platforms while developers remain unknowingly exposed."

Found this article interesting? Follow us on [Google News](https://news.google.com/publications/CAAqLQgKIidDQklTRndnTWFoTUtFWFJvWldoaFkydGxjbTVsZDNNdVkyOXRLQUFQAQ), [Twitter](https://twitter.com/thehackersnews) and [LinkedIn](https://www.linkedin.com/company/thehackernews/) to read more exclusive content we post.

SHARE [](https://thehackernews.com/2025/10/over-100-vs-code-extensions-exposed.html#link_share) [](https://thehackernews.com/2025/10/over-100-vs-code-extensions-exposed.html#link_share) [](https://thehackernews.com/2025/10/over-100-vs-code-extensions-exposed.html#link_share) 

[Tweet](https://thehackernews.com/2025/10/over-100-vs-code-extensions-exposed.html#link_share)

[Share](https://thehackernews.com/2025/10/over-100-vs-code-extensions-exposed.html#link_share)

[Share](https://thehackernews.com/2025/10/over-100-vs-code-extensions-exposed.html#link_share)

Share

SHARE 

[Cloud security](https://thehackernews.com/search/label/Cloud%20security), [cybersecurity](https://thehackernews.com/search/label/cybersecurity), [Data Leak](https://thehackernews.com/search/label/Data%20Leak), [Malware](https://thehackernews.com/search/label/Malware), [Microsoft](https://thehackernews.com/search/label/Microsoft), [Open VSX](https://thehackernews.com/search/label/Open%20VSX), [Software Supply Chain](https://thehackernews.com/search/label/Software%20Supply%20Chain), [Visual Studio Code](https://thehackernews.com/search/label/Visual%20Studio%20Code)

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
