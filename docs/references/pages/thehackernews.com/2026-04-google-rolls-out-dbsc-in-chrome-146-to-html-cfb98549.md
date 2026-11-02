---
url: "https://thehackernews.com/2026/04/google-rolls-out-dbsc-in-chrome-146-to.html"
title: "Google Rolls Out DBSC in Chrome 146 to Block Session Theft on Windows"
site: "The Hacker News"
captured: "2026-11-02T18:50:56Z"
via: "live"
sha256: "d97fecaa23c7de961ce04d21cc8875e47cc7140617ab5effbc93412f5cda5341"
wayback: "https://web.archive.org/web/20261022095525/https://thehackernews.com/2026/04/google-rolls-out-dbsc-in-chrome-146-to.html"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Google Rolls Out DBSC in Chrome 146 to Block Session Theft on Windows

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://thehackernews.com/2026/04/google-rolls-out-dbsc-in-chrome-146-to.html>
> Snapshot: <https://web.archive.org/web/20261022095525/https://thehackernews.com/2026/04/google-rolls-out-dbsc-in-chrome-146-to.html>

# [Google Rolls Out DBSC in Chrome 146 to Block Session Theft on Windows](https://thehackernews.com/2026/04/google-rolls-out-dbsc-in-chrome-146-to.html)

Ravie LakshmananApr 10, 2026Malware / Browser Security

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEiC-kFnk6uDzN76983rxMJBgJzi5ByxqZ0SM5RAfG1171e3I_lRUCBHIZ0kmMRkxERMiWEO9WRX3D6mkadUuRhw69KYHi4VzPrIa4s4IVilNmFANa2EMbuk1blKF_4ChwqIBuTb4FLj_dqhTDUDsivEnw8OmDL85giaaJTiqATwZArXUq6_3_X7tfd_RLbV/s1700-nu-rw-lo-l85-e365/chrome-cookies.jpg

Google has made Device Bound Session Credentials ([DBSC](https://thehackernews.com/2025/07/google-launches-dbsc-open-beta-in.html)) generally available to all Windows users of its Chrome web browser, months after it [began testing](https://thehackernews.com/2025/07/google-launches-dbsc-open-beta-in.html) the security feature in open beta.

The public availability is currently limited to Windows users on Chrome 146, with macOS expansion planned in an upcoming Chrome release.

"This project represents a significant step forward in our ongoing efforts to combat session theft, which remains a prevalent threat in the modern security landscape," Google's Chrome and Account Security teams [said](https://security.googleblog.com/2026/04/protecting-cookies-with-device-bound.html) in a Thursday post.

Session theft involves the covert exfiltration of session cookies from the web browser, either by gathering existing ones or waiting for a victim to log in to an account, to an attacker-controlled server.

https://thehackernews.uk/ai-governance-platform-d

Typically, this happens when users inadvertently download information-stealing malware into their systems. These stealer malware families – of which there are many, such as Atomic, Lumma, and Vidar Stealer – come with capabilities to harvest a wide range of information from compromised systems, including cookies.

Because session cookies often have extended lifespans, attackers can leverage them to gain unauthorized access to victims' online accounts without having to know their passwords. Once collected, these tokens are packaged and sold to other threat actors for financial gain. Cybercriminals who acquire them can follow up with their attacks of their own.

DBSC, first [announced](https://thehackernews.com/2024/04/google-chrome-beta-tests-new-dbsc.html) by Google in April 2024, aims to counter this abuse by cryptographically tying the authentication session to a specific device. In doing so, the idea is to render cookies worthless even if they get stolen by malware.

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEhxT7sVyKGgXDSeg-GcHvKWbpHvrvgjS8DhwcRhvZgtRD1s2dlh-40Q09IIcm7a0OkglGBZ0LVTsuWchqlL81h-rsLnyuEZHrqgx0LLa-ML1GD1P9-bjH17I6qqVddpw7X5CClHgglbcqtOP59HdaUkwOfBqJqhM0xjwlkwizGrFoG_tUUlvClxGnOK-Bbw/s1700-nu-rw-lo-l85-e365/cookie.png

"It does this using hardware-backed security modules, such as the Trusted Platform Module (TPM) on Windows and the Secure Enclave on macOS, to generate a unique public/private key pair that cannot be exported from the machine," Google explained.

"The issuance of new short-lived session cookies is contingent upon Chrome proving possession of the corresponding private key to the server. Because attackers cannot steal this key, any exfiltrated cookies quickly expire and become useless to those attackers."

In the event a user's device does not support secure key storage, DBSC gracefully falls back to standard behavior without breaking the authentication flow, Google [said](https://developer.chrome.com/docs/web-platform/device-bound-session-credentials) in its developer documentation.

https://thehackernews.uk/network-defense-dd

The tech giant said it has observed a significant reduction in session theft since its launch, an early indication of the success of the countermeasure. The official launch is just the start, as the company plans to bring DBSC to a broader range of devices and introduce advanced capabilities to better integrate with enterprise environments.

Google, which worked with Microsoft to design the standard with an aim to make it an open web standard, also emphasized that the DBSC architecture is private by design and that the distinct key approach ensures that websites cannot use the session credentials to correlate a user's activity across different sessions or sites on the same device.

"Furthermore, the protocol is designed to be lean: it does not leak device identifiers or attestation data to the server beyond the per-session public key required to certify proof of possession," it added. "This minimal information exchange ensures DBSC helps secure sessions without enabling cross-site tracking or acting as a device fingerprinting mechanism."

Found this article interesting? Follow us on [Google News](https://news.google.com/publications/CAAqLQgKIidDQklTRndnTWFoTUtFWFJvWldoaFkydGxjbTVsZDNNdVkyOXRLQUFQAQ), [Twitter](https://twitter.com/thehackersnews) and [LinkedIn](https://www.linkedin.com/company/thehackernews/) to read more exclusive content we post.

SHARE [](https://thehackernews.com/2026/04/google-rolls-out-dbsc-in-chrome-146-to.html#link_share) [](https://thehackernews.com/2026/04/google-rolls-out-dbsc-in-chrome-146-to.html#link_share) [](https://thehackernews.com/2026/04/google-rolls-out-dbsc-in-chrome-146-to.html#link_share) 

[Tweet](https://thehackernews.com/2026/04/google-rolls-out-dbsc-in-chrome-146-to.html#link_share)

[Share](https://thehackernews.com/2026/04/google-rolls-out-dbsc-in-chrome-146-to.html#link_share)

[Share](https://thehackernews.com/2026/04/google-rolls-out-dbsc-in-chrome-146-to.html#link_share)

Share

SHARE 

[browser security](https://thehackernews.com/search/label/browser%20security), [Chrome](https://thehackernews.com/search/label/Chrome), [cybersecurity](https://thehackernews.com/search/label/cybersecurity), [Google](https://thehackernews.com/search/label/Google), [Infostealer](https://thehackernews.com/search/label/Infostealer), [Malware](https://thehackernews.com/search/label/Malware), [Privacy](https://thehackernews.com/search/label/Privacy), [Windows](https://thehackernews.com/search/label/Windows)

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
