---
url: "https://thehackernews.com/2025/03/clearfake-infects-9300-sites-uses-fake.html"
title: "ClearFake Infects 9,300 Sites, Uses Fake reCAPTCHA and Turnstile to Spread Info-Stealers"
site: "The Hacker News"
captured: "2026-11-02T18:47:43Z"
via: "live"
sha256: "af86e1d785a222a957c91216d9739b3602346f8f9dc43d0799be8e5b82e0ba5f"
wayback: "https://web.archive.org/web/20260930155053/https://thehackernews.com/2025/03/clearfake-infects-9300-sites-uses-fake.html"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# ClearFake Infects 9,300 Sites, Uses Fake reCAPTCHA and Turnstile to Spread Info-Stealers

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://thehackernews.com/2025/03/clearfake-infects-9300-sites-uses-fake.html>
> Snapshot: <https://web.archive.org/web/20260930155053/https://thehackernews.com/2025/03/clearfake-infects-9300-sites-uses-fake.html>

# [ClearFake Infects 9,300 Sites, Uses Fake reCAPTCHA and Turnstile to Spread Info-Stealers](https://thehackernews.com/2025/03/clearfake-infects-9300-sites-uses-fake.html)

Ravie LakshmananMar 19, 2025Cloud Security / Web Security

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEjpa0ebMbexiprgvnLfwSJ58i9syhv-e8luRcQjfDwVLxgaUj2fR0WfQlfiDB6YKk3WKoepfTsdDbVdeR4P1jD77cMhqKkhZiRFGb4fveZayZOwpjbmX9joHxAXHxRkd_kC8i4zwDzgIb2Pj_HOoN4En15w6rpHUXJTgKwrdahzA611lNltUVj4Uh1d1lxd/s1700-nu-rw-lo-l85-e365/malware.png

The threat actors behind the ClearFake campaign are using fake reCAPTCHA or Cloudflare Turnstile verifications as lures to trick users into downloading malware such as Lumma Stealer and Vidar Stealer.

[ClearFake](https://thehackernews.com/2023/11/clearfake-campaign-expands-to-deliver.html), first highlighted in July 2023, is the name given to a threat activity cluster that employs [fake web browser update baits](https://www.proofpoint.com/us/blog/threat-insight/are-you-sure-your-browser-date-current-landscape-fake-browser-updates) on compromised WordPress as a malware distribution vector.

The campaign is also known for relying on another technique known as [EtherHiding](https://thehackernews.com/2023/10/binances-smart-chain-exploited-in-new.html) to fetch the next-stage payload by utilizing Binance's Smart Chain (BSC) contracts as a way to make the attack chain more resilient. The end goal of these infection chains is to deliver information-stealing malware capable of targeting both Windows and macOS systems.

As of May 2024, ClearFake attacks have adopted what has by now come to be known as [ClickFix](https://thehackernews.com/2025/03/microsoft-warns-of-clickfix-phishing.html), a [social engineering ploy](https://www.seqrite.com/blog/fake-captcha-lures-victims-lumma-stealer-abuses-clipboard-and-powershell/) that involves deceiving users into running malicious PowerShell code under the guise of addressing a non-existent technical issue.

https://thehackernews.uk/ai-governance-platform-d

"Although this new ClearFake variant continues to rely on the EtherHiding technique and the ClickFix tactic, it has introduced additional interactions with the Binance Smart Chain," Sekoia [said](https://blog.sekoia.io/clearfakes-new-widespread-variant-increased-web3-exploitation-for-malware-delivery/) in a new analysis.

"By using smart contract's Application Binary Interfaces, these interactions involve loading multiple JavaScript codes and additional resources that fingerprint the victim's system, as well as downloading, decrypting and displaying the ClickFix lure."

The latest iteration of the ClearFake framework marks a significant evolution, adopting Web3 capabilities to resist analysis and encrypting the ClickFix-related HTML code.

The net result is an updated multi-stage attack sequence that's initiated when a victim visits a compromised site, which then leads to the retrieval of an intermediate JavaScript code from BSC. The loaded JavaScript is subsequently responsible for fingerprinting the system and fetching the encrypted ClickFix code hosted on Cloudflare Pages.

Should the victim follow through and execute the malicious PowerShell command, it leads to the deployment of [Emmenhtal Loader](https://thehackernews.com/2024/08/new-peaklight-dropper-deployed-in.html) (aka PEAKLIGHT) that subsequently drops Lumma Stealer.

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEiMtHuMR_iPIuK6bZ6460OsLUP5wTEvWcx8nc8Y8_b-Ky6pcisz3h8ovW32mpzHs-DZUWCRyEjk_E5hyphenhyphenvVu1-TK1TTomCSSCA-91N7AjvJ0nEmz1d1aTe6fpsIrNgJOQgToexi9JtubZ0zqRgCSOm8J-vYB3dHB_Uc_484V1zuzSg_1rtvkfCSfg1QBnrnQ/s1700-nu-rw-lo-l85-e365/chain.png

Sekoia said it observed an alternate ClearFake attack chain in late January 2025 that served a PowerShell loader responsible for installing Vidar Stealer. As of last month, at least 9,300 websites have been infected with ClearFake.

"The operator has consistently updated the framework code, lures, and distributed payloads on a daily basis," it added. "ClearFake execution now relies on multiple pieces of data stored in the Binance Smart Chain, including JavaScript code, AES key, URLs hosting lure HTML files, and ClickFix PowerShell commands."

"The number of websites compromised by ClearFake suggest that this threat remains widespread and affects many users worldwide. In July 2024, [...] approximately 200,000 unique users were potentially exposed to ClearFake lures encouraging them to download malware."

The development comes as over 100 auto dealership sites have been discovered compromised with [ClickFix lures](https://www.hhs.gov/sites/default/files/clickfix-attacks-sector-alert-tlpclear.pdf) that lead to the deployment of [SectopRAT](https://thehackernews.com/2024/04/cybercriminals-targeting-latin-america.html) malware.

"Where this infection on the auto dealerships happened was not on the dealership's own website, but a third-party video service," [said](https://rmceoin.github.io/malware-analysis/2025/03/13/supply-chain.html) security researcher Randy McEoin, who detailed some of the earliest [ClearFake campaigns](https://rmceoin.github.io/malware-analysis/2023/08/06/clearfake.html) in 2023, describing the incident as an instance of a supply chain attack.

The video service in question is LES Automotive ("idostream[.]com"), which has since removed the malicious JavaScript injection from the site.

https://thehackernews.uk/network-defense-dd

The findings also coincide with the discovery of several phishing campaigns that are engineered to push various malware families and conduct credential harvesting -

- Using [virtual hard disk (VHD) files](https://www.forcepoint.com/blog/x-labs/venomrat-malware-uses-virtual-hard-drives) embedded within archive file attachments in email messages to distribute [Venom RAT](https://thehackernews.com/2024/04/massive-phishing-campaign-strikes-latin.html) by means of a Windows batch script
- Using [Microsoft Excel file attachments](https://www.seqrite.com/blog/steganographic-campaign-distributing-malware/) that exploit a known security flaw ([CVE-2017-0199](https://thehackernews.com/2024/11/cybercriminals-use-excel-exploit-to.html)) to download an HTML Application (HTA) that then uses Visual Basic Script (VBS) to fetch an image, which contains another payload responsible for decoding and launching AsyncRAT and Remcos RAT
- Exploiting [misconfigurations in Microsoft 365 infrastructure](https://guardz.com/blog/sophisticated-phishing-campaign-exploiting-microsoft-365-infrastructure/) to take control of tenants, create new administrative accounts, and deliver phishing content that bypasses email security protections and ultimately facilitates credential harvesting and account takeover (ATO)

As social engineering campaigns continue to become more sophisticated, it's essential that organizations and businesses stay ahead of the curve and implement robust authentication and access-control mechanisms against Adversary-in-the-Middle (AitM) and Browser-in-the-Middle (BitM) techniques that allow attackers to hijack accounts.

"A pivotal benefit of employing a BitM framework lies in its rapid targeting capability, allowing it to reach any website on the web in a matter of seconds and with minimal configuration," Google-owned Mandiant [said](https://cloud.google.com/blog/topics/threat-intelligence/session-stealing-browser-in-the-middle) in a report published this week.

"Once an application is targeted through a BitM tool or framework, the legitimate site is served through an attacker-controlled browser. This makes the distinction between a legitimate and a fake site exceptionally challenging for a victim. From the perspective of an adversary, BitM allows for a simple yet effective means of stealing sessions protected by MFA."

Found this article interesting? Follow us on [Google News](https://news.google.com/publications/CAAqLQgKIidDQklTRndnTWFoTUtFWFJvWldoaFkydGxjbTVsZDNNdVkyOXRLQUFQAQ), [Twitter](https://twitter.com/thehackersnews) and [LinkedIn](https://www.linkedin.com/company/thehackernews/) to read more exclusive content we post.

SHARE [](https://thehackernews.com/2025/03/clearfake-infects-9300-sites-uses-fake.html#link_share) [](https://thehackernews.com/2025/03/clearfake-infects-9300-sites-uses-fake.html#link_share) [](https://thehackernews.com/2025/03/clearfake-infects-9300-sites-uses-fake.html#link_share) 

[Tweet](https://thehackernews.com/2025/03/clearfake-infects-9300-sites-uses-fake.html#link_share)

[Share](https://thehackernews.com/2025/03/clearfake-infects-9300-sites-uses-fake.html#link_share)

[Share](https://thehackernews.com/2025/03/clearfake-infects-9300-sites-uses-fake.html#link_share)

Share

SHARE 

[Cloud security](https://thehackernews.com/search/label/Cloud%20security), [cybersecurity](https://thehackernews.com/search/label/cybersecurity), [identity theft](https://thehackernews.com/search/label/identity%20theft), [Malware](https://thehackernews.com/search/label/Malware), [Microsoft 365](https://thehackernews.com/search/label/Microsoft%20365), [Phishing](https://thehackernews.com/search/label/Phishing), [ransomware](https://thehackernews.com/search/label/ransomware), [social engineering](https://thehackernews.com/search/label/social%20engineering), [Threat Intelligence](https://thehackernews.com/search/label/Threat%20Intelligence), [web security](https://thehackernews.com/search/label/web%20security)

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
