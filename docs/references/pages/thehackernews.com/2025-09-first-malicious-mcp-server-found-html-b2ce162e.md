---
url: "https://thehackernews.com/2025/09/first-malicious-mcp-server-found.html"
title: "First Malicious MCP Server Found Stealing Emails in Rogue Postmark-MCP Package"
site: "The Hacker News"
captured: "2026-11-02T18:48:50Z"
via: "live"
sha256: "7eac0fb9180c3dc11c16a769e90c87375a86dd1f76cf22909aeda9390299d914"
wayback: "https://web.archive.org/web/20261025100227/https://thehackernews.com/2025/09/first-malicious-mcp-server-found.html"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# First Malicious MCP Server Found Stealing Emails in Rogue Postmark-MCP Package

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://thehackernews.com/2025/09/first-malicious-mcp-server-found.html>
> Snapshot: <https://web.archive.org/web/20261025100227/https://thehackernews.com/2025/09/first-malicious-mcp-server-found.html>

# [First Malicious MCP Server Found Stealing Emails in Rogue Postmark-MCP Package](https://thehackernews.com/2025/09/first-malicious-mcp-server-found.html)

Ravie LakshmananSep 29, 2025MCP Server / Vulnerability

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEgK1oW8o38Twsv57DnMEOum9PYqM8x1bROlUvaKUge71kyWEZRzNmhQJPsmO-CD1ZGqpwOTn3grXBQ5mk0tDkOy1xdDHRMWjulc2ok-9ftApRyK9lkFNcf-eMNnVm4uyG0eIKsjnjwaujyHuQhkQruaABmUzl77EUvEPfQztJBhHm5sY3UiSV6SChhFqm6X/s1700-nu-rw-lo-l85-e365/mcp-ai-email-hack.jpg

Cybersecurity researchers have discovered what has been described as the first-ever instance of a malicious Model Context Protocol ([MCP](https://thehackernews.com/2025/04/experts-uncover-critical-mcp-and-a2a.html)) server spotted in the wild, raising software supply chain risks.

According to Koi Security, a legitimate-looking developer managed to slip in rogue code within an npm package called "postmark-mcp" that copied an official Postmark Labs library of the same name. The malicious functionality was introduced in version 1.0.16, which was released on September 17, 2025.

The actual "postmark-mcp" library, [available on GitHub](https://github.com/ActiveCampaign/postmark-mcp), [exposes an MCP server](https://postmarkapp.com/lp/mcp) to allow users to send emails, access and use email templates, and track campaigns using artificial intelligence (AI) assistants.

https://thehackernews.uk/ai-governance-platform-d

The npm package in question has since been deleted from npm by the developer "[phanpak](https://www.npmjs.com/~phanpak)," who [uploaded](https://secure.software/npm/packages/postmark-mcp) it to the repository on September 15, 2025, and maintains 31 other packages. The JavaScript library [attracted](https://npm-stat.com/charts.html?package=postmark-mcp) a total of 1,643 downloads.

"Since version 1.0.16, it's been quietly copying every email to the developer's personal server," Koi Security Chief Technology Officer Idan Dardikman [said](https://www.koi.security/blog/postmark-mcp-npm-malicious-backdoor-email-theft). "This is the world’s first sighting of a real-world malicious MCP server. The attack surface for endpoint supply chain attacks is slowly becoming the enterprise's biggest attack surface."

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEjLxgJDBlWCWcrX6NTxbqoCcAnSGfEB3Y_N4B2ay9UIfZPDDXEQ0tcfx6CLIQLA63xCVhfxZwaZaYcuqNOEIZbUwSEqa9q4ojCxiZ0nXA1TTu7eVPohcOdFoJjs2HjpB7W3V0260Jm-HUobCRqqlNrWXzmNVqVsjpRB4IsGDDZkNIlGalU0jGZWwZSKqLlO/s1700-nu-rw-lo-l85-e365/postmark-mcp.jpg

The malicious package is a replica of the original library, save for a one-line change added in version 1.0.16 that essentially forwards every email sent using the MCP server to the email address "phan@giftshop[.]club" by BCC'ing it, potentially exposing sensitive communications.

"The postmark-mcp backdoor isn't sophisticated – it's embarrassingly simple," Dardikman said. "But it perfectly demonstrates how completely broken this whole setup is. One developer. One line of code. Thousands upon thousands of stolen emails."

Developers who have installed the npm package are recommended to immediately remove it from their workflows, rotate any credentials that may have been exposed through email, and review email logs for BCC traffic to the reported domain.

https://thehackernews.uk/network-defense-dd

"MCP servers typically run with high trust and broad permissions inside agent toolchains. As such, any data they handle can be sensitive (password resets, invoices, customer communications, internal memos, etc.)," Snyk [said](https://snyk.io/blog/malicious-mcp-server-on-npm-postmark-mcp-harvests-emails/). "In this case, the backdoor in this MCP Server was built with the intention to harvest and exfiltrate emails for agentic workflows that relied on this MCP Server."

The findings illustrate how threat actors continue to abuse the user trust associated with the open-source ecosystem and the nascent MCP ecosystem to their advantage, especially when they are rolled out in business critical environments without adequate guardrails.

### Update

In a statement, Postmark said the npm package "postmark-mcp" was not official, and that a malicious actor created a fake package on npm impersonating its name to steal email data.

"We didn't develop, authorize, or have any involvement with the 'postmark-mcp' npm package," the email delivery platform [said](https://postmarkapp.com/blog/information-regarding-malicious-postmark-mcp-package). "The legitimate Postmark API and services remain secure and unaffected by this incident."

Found this article interesting? Follow us on [Google News](https://news.google.com/publications/CAAqLQgKIidDQklTRndnTWFoTUtFWFJvWldoaFkydGxjbTVsZDNNdVkyOXRLQUFQAQ), [Twitter](https://twitter.com/thehackersnews) and [LinkedIn](https://www.linkedin.com/company/thehackernews/) to read more exclusive content we post.

SHARE [](https://thehackernews.com/2025/09/first-malicious-mcp-server-found.html#link_share) [](https://thehackernews.com/2025/09/first-malicious-mcp-server-found.html#link_share) [](https://thehackernews.com/2025/09/first-malicious-mcp-server-found.html#link_share) 

[Tweet](https://thehackernews.com/2025/09/first-malicious-mcp-server-found.html#link_share)

[Share](https://thehackernews.com/2025/09/first-malicious-mcp-server-found.html#link_share)

[Share](https://thehackernews.com/2025/09/first-malicious-mcp-server-found.html#link_share)

Share

SHARE 

[cybersecurity](https://thehackernews.com/search/label/cybersecurity), [data exfiltration](https://thehackernews.com/search/label/data%20exfiltration), [email security](https://thehackernews.com/search/label/email%20security), [JavaScript](https://thehackernews.com/search/label/JavaScript), [MCP Server](https://thehackernews.com/search/label/MCP%20Server), [NPM](https://thehackernews.com/search/label/NPM), [Open Source](https://thehackernews.com/search/label/Open%20Source), [supply chain attack](https://thehackernews.com/search/label/supply%20chain%20attack), [Vulnerability](https://thehackernews.com/search/label/Vulnerability)

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
