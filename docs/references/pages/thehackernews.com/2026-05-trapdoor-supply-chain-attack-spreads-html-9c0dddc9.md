---
url: "https://thehackernews.com/2026/05/trapdoor-supply-chain-attack-spreads.html"
title: "TrapDoor Supply Chain Attack Spreads Credential-Stealing Malware via npm, PyPI, and CratesIO"
site: "The Hacker News"
captured: "2026-11-02T18:51:08Z"
via: "live"
sha256: "c24ad4ebfe98f19d1a12afb0d7deae087a1155ae888349237d01e1fa56ec7667"
wayback: "https://web.archive.org/web/20261006073341/https://thehackernews.com/2026/05/trapdoor-supply-chain-attack-spreads.html"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# TrapDoor Supply Chain Attack Spreads Credential-Stealing Malware via npm, PyPI, and CratesIO

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://thehackernews.com/2026/05/trapdoor-supply-chain-attack-spreads.html>
> Snapshot: <https://web.archive.org/web/20261006073341/https://thehackernews.com/2026/05/trapdoor-supply-chain-attack-spreads.html>

# [TrapDoor Supply Chain Attack Spreads Credential-Stealing Malware via npm, PyPI, and CratesIO](https://thehackernews.com/2026/05/trapdoor-supply-chain-attack-spreads.html)

Ravie LakshmananMay 25, 2026Supply Chain Attack / Malware

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEjOb58wXNPgRDazHcSLTObawPnMVsCDUEAoFclGVn3CC8qe6Pr_0-Gce-SxCO7FJ5HzU23WE_soU5iTc0zvLL0zzbXPcO8MqVgHIIAKXJo4ExcujDPV9yiIdN1X1jV63ACBN66_ktTeQ0FVmYsCTJC3tpyxpUBrvxLh_xqF-mKOZy8uaqz3QKyf6XpJHC4z/s1700-nu-rw-lo-l85-e365/npm-python-rust.jpg

A new coordinated cross-ecosystem software supply chain attack campaign has targeted npm, PyPI, and Crates.io to distribute credential-stealing malware.

The campaign, codenamed [TrapDoor](https://socket.dev/blog/trapdoor-crypto-stealer-npm-pypi-crates), spans more than 34 malicious packages across over 384 versions. The earliest activity was recorded on May 22, 2026, at 8:20 p.m. UTC, with new packages published to the ecosystems in waves from a cluster of accounts in quick succession.

"TrapDoor targets developers in crypto, DeFi, Solana, and AI communities," Socket said. "The malicious packages are designed to steal developer secrets, crypto wallets, SSH keys, cloud credentials, browser data, and environment variables."

"Several npm packages also deploy a shared payload, trap-core.js, that scans for credentials, validates AWS and GitHub tokens, attempts SSH-based lateral movement, and plants persistence through .cursorrules, CLAUDE.md, Git hooks, shell hooks, systemd, cron, and SSH."

It's worth noting that the activity has no connection to [another campaign of the same name](https://thehackernews.com/2026/05/trapdoor-android-ad-fraud-scheme-hit.html) that HUMAN's Satori Threat Intelligence and Research Team detailed last week as engaging in ad fraud by distributing 455 Android apps through the Google Play Store.

https://thehackernews.uk/ai-governance-platform-d

The [list of identified packages](https://socket.dev/supply-chain-attacks/trapdoor-crypto-stealer) is below -

-  Crates.io

- move-analyzer-build
- move-compiler-tools
- move-project-builder
- sui-framework-helpers
- sui-move-build-helper
- sui-sdk-build-utils

-  npm

- async-pipeline-builder
- build-scripts-utils
- chain-key-validator
- crypto-credential-scanner
- defi-env-auditor
- defi-threat-scanner
- deployment-key-auditor
- dev-env-bootstrapper
- eth-wallet-sentinel
- llm-context-compressor
- mnemonic-safety-check
- model-switch-router
- node-setup-helpers
- project-init-tools
- prompt-engineering-toolkit
- solidity-deploy-guard
- token-usage-tracker
- wallet-backup-verifier
- wallet-security-checker
- web3-secrets-detector
- workspace-config-loader

-  PyPI

- cryptowallet-safety
- data-pipeline-check
- defi-risk-scanner
- env-loader-cli
- eth-security-auditor
- git-config-sync
- solidity-build-guard

The operation is notable for its diverse delivery paths, using postinstall hooks, remote JavaScript payloads that are executed during package imports, and malicious build.rs scripts to target Sui and Move developers. The packages masquerade as seemingly harmless tools, giving attackers the ability to reach a broad audience.

The npm packages have been found to run a JavaScript payload ("trap-core.js"), which scans for credentials and developer secrets, validates stolen credentials using AWS and GitHub API calls, and creates persistence on the host using cron jobs, systemd services, Git hooks, and moves across the network via SSH.

The Rust crates, in a similar fashion, search for local keystores, encrypt the data using a hardcoded XOR key, and exfiltrate it to GitHub Gists. The packages are also noteworthy for the use of a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html) ("build.rs") to trigger the execution of the malicious code.

https://thehackernews.uk/network-defense-dd

The Python packages associated with TrapDoor are designed such that they are auto-executed on import. The primary goal of the packages is to download JavaScript from an attacker-controlled GitHub Pages domain ("ddjidd564.github[.]io"), and run it using "node -e."

"This technique allows the Python package to delegate execution to a remote JavaScript payload, giving the attacker more flexibility after publication," Socket explained. "By hosting the payload externally, the attacker can update behavior without publishing a new PyPI release."

An unusual aspect of the campaign is the implanting of .cursorrules and CLAUDE.md containing hidden instructions to trick artificial intelligence (AI) assistants into running a "security scan" that results in secret discovery and exfiltration. This is achieved by opening GitHub pull requests (PRs) across popular AI and developer projects, including "browser-use/browser-use," "langchain-ai/langchain," and "langflow-ai/langflow."

The PR activity indicates that TrapDoor extends beyond pushing malicious packages to open-source ecosystems. Socket said the threat actor is likely testing whether AI-related project files can be introduced through regular open-source contribution workflows, thereby causing AI coding tools to parse those hidden instructions and apply them.

The findings once again demonstrate how threat actors are increasingly targeting developer workflows, aiming to steal a wide range of information that could make it possible to burrow deeper into target environments for follow-on attacks.

"TrapDoor shows how attackers are combining traditional package typosquatting with newer developer-environment attack paths," Socket said. "The package names are tailored to appear relevant to crypto development, AI tooling, local environment setup, and security workflows. The malware then uses ecosystem-specific execution paths: build.rs in Rust, postinstall hooks in npm, and import-time execution in Python."

Found this article interesting? Follow us on [Google News](https://news.google.com/publications/CAAqLQgKIidDQklTRndnTWFoTUtFWFJvWldoaFkydGxjbTVsZDNNdVkyOXRLQUFQAQ), [Twitter](https://twitter.com/thehackersnews) and [LinkedIn](https://www.linkedin.com/company/thehackernews/) to read more exclusive content we post.

SHARE [](https://thehackernews.com/2026/05/trapdoor-supply-chain-attack-spreads.html#link_share) [](https://thehackernews.com/2026/05/trapdoor-supply-chain-attack-spreads.html#link_share) [](https://thehackernews.com/2026/05/trapdoor-supply-chain-attack-spreads.html#link_share) 

[Tweet](https://thehackernews.com/2026/05/trapdoor-supply-chain-attack-spreads.html#link_share)

[Share](https://thehackernews.com/2026/05/trapdoor-supply-chain-attack-spreads.html#link_share)

[Share](https://thehackernews.com/2026/05/trapdoor-supply-chain-attack-spreads.html#link_share)

Share

SHARE 

[artificial intelligence](https://thehackernews.com/search/label/artificial%20intelligence), [Cloud security](https://thehackernews.com/search/label/Cloud%20security), [cybersecurity](https://thehackernews.com/search/label/cybersecurity), [Malware](https://thehackernews.com/search/label/Malware), [NPM](https://thehackernews.com/search/label/NPM), [Open Source](https://thehackernews.com/search/label/Open%20Source), [PyPI](https://thehackernews.com/search/label/PyPI), [Supply Chain Attack](https://thehackernews.com/search/label/Supply%20Chain%20Attack)

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
