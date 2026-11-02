---
url: "https://thehackernews.com/2026/03/the-state-of-secrets-sprawl-2026-9.html"
title: "The State of Secrets Sprawl 2026: 9 Takeaways for CISOs"
site: "The Hacker News"
captured: "2026-11-02T18:50:09Z"
via: "live"
sha256: "3a0c27a5b45c5e66398065232e6dd09e6099185b2a71c4263122d5240c349dcc"
wayback: "https://web.archive.org/web/20260924130838/https://thehackernews.com/2026/03/the-state-of-secrets-sprawl-2026-9.html"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# The State of Secrets Sprawl 2026: 9 Takeaways for CISOs

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://thehackernews.com/2026/03/the-state-of-secrets-sprawl-2026-9.html>
> Snapshot: <https://web.archive.org/web/20260924130838/https://thehackernews.com/2026/03/the-state-of-secrets-sprawl-2026-9.html>

# [The State of Secrets Sprawl 2026: 9 Takeaways for CISOs](https://thehackernews.com/2026/03/the-state-of-secrets-sprawl-2026-9.html)

The Hacker NewsMar 30, 2026DevOps / Artificial Intelligence

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEg4tVpFBDvdU-vz8vM1T6SJZwd3AiySFqEyonUutxGPNimXEqTVOKN-pCI_lF5Ti2GngFUPtEphVI4Qep03CBciF7NhaWEIKYdDfkVY-VleTTcqYJvcMTKrd-EJ4kh2zLk3cY7pqqxI7bfEghuxGAV7lwUPjTD6nTa5dKj5e8_RJD9UjHn12015puG8P4o/s1700-nu-rw-lo-l85-e365/key.gif

Secrets sprawl isn't slowing down: in 2025, it accelerated faster than most security teams anticipated. [GitGuardian's State of Secrets Sprawl 2026 report](https://www.gitguardian.com/files/the-state-of-secrets-sprawl-report-2026) analyzed billions of commits across public GitHub and uncovered 29 million new hardcoded secrets in 2025 alone, a 34% increase year over year and the largest single-year jump ever recorded.

This year's findings reveal three core trends: AI has fundamentally reshaped how and where credentials leak, internal systems are far more exposed than most organizations realize, and remediation continues to be the industry's Achilles heel.

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEj7TTJOaUu01nuUn-1nszxtRy-gywqOqWfSYlXnAu8berV_SYtUseBWiiWKSMnbsdjXpISdHYZNiTFw6a-R_B8SMxUJ-bQUNThd40hsiWNwfHFylXFv4JQkIhmKhQN6BDeu0JQxe0w5ZUfKdM7xcm0mlupwveG97fltpFfq3efY_3dWe3fMId62ca9u8cg/s1700-nu-rw-lo-l85-e365/1.png

Here are nine strategic takeaways that matter.

## 1. Secrets are growing faster than the developer population

Since 2021, leaked secrets have grown 152%, while GitHub's public developer base expanded 98%. More developers and more AI-assisted code generation mean more credentials in circulation, and detection alone can't keep pace.

## 2. AI services drove 81% more leaks year over year

GitGuardian detected 1,275,105 leaked secrets tied to AI services in 2025, up 81% from 2024. Eight of the ten fastest-growing categories of leaked secrets were AI-related. This isn't just about OpenAI or Anthropic keys. The real explosion is happening in LLM infrastructure: retrieval APIs like Brave Search (+1,255%), orchestration tools like Firecrawl (+796%), and managed backends like Supabase (+992%). Every new AI integration introduces another machine identity, and each one expands the attack surface. Deploying AI safely requires a proper secrets security strategy.

## 3. Internal repositories are 6x more likely to leak than public ones

While public GitHub gets the attention, internal repositories are where the highest-value credentials live. GitGuardian's research found that 32.2% of internal repos contain at least one hardcoded secret, compared to just 5.6% of public repos. These aren't test keys. They're CI/CD tokens, cloud access credentials, and database passwords, the exact assets attackers target once they gain a foothold. Security through obscurity has failed. Treat internal repos as first-class leak sources.

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEj_OvmKrTLLUqQ5pVs_ovpMia3_NUkJKgy15k4f_AB6ByXoJrGAC252W8_ysTL6UoW9UcdfK-HH8tSG-PgsGNfgKMXeq4e7X9d-6trdDoit5WnaVWIvXXzy6qUcDG6LtssVrfgnF4ldzV_DGdl5lWKbrC5gtJDiku7j9SDyphUh56zf5I2O9TWm157Od1Y/s1700-nu-rw-lo-l85-e365/2.png

## 4. 28% of leaks happen entirely outside code

Secrets don't only live in repositories. GitGuardian found that 28% of incidents in 2025 originated entirely outside source code, in Slack, Jira, Confluence, and similar collaboration tools. These leaks are more dangerous: 56.7% of secrets found only in collaboration tools were rated critical, compared to 43.7% for code-only incidents. Teams share credentials during incident response, troubleshooting, and onboarding. If you're only scanning code, you're missing a quarter of your exposure. And the credentials leaking in collaboration tools are usually more critical and severe.

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEi0lgO08_CQqOlPFjzen2FSjKz5lkjCQ6napoKymbcFUzst92AuzjmqfzILEJsoCjk4jM-o55uxoVqIIib2zGdtjbp9UteR4NMMs9_Lj-AC7j8_N9ZtHaMXyUZAwXUPr_BVnTHSZP7g6HBe3FOkNw_YNJi01_LS_rxXLUADiBn_JLeVJeLoffFcyf0-xAk/s1700-nu-rw-lo-l85-e365/3.png

## 5. Self-hosted GitLab and Docker registries expose secrets at 3-4x the rate of public GitHub

GitGuardian discovered thousands of unintentionally exposed self-hosted GitLab instances and Docker registries in 2025. Scanning these systems revealed 80,000 credentials, with 10,000 still valid. Secrets in Docker images were particularly troubling: 18% of scanned Docker images contained secrets, and 15% of those were valid, compared to 12% of GitLab repositories with a 12% validity rate. Docker secrets are also more production-adjacent. The perimeter between private and public is porous.

## 6. 64% of secrets leaked in 2022 remain valid today

Detection is not remediation. GitGuardian retested secrets confirmed as valid in 2022 and found that 64% are still exploitable four years later. This is not a rounding error. It's proof that rotation and revocation are not routine, owned, or automated in most organizations. Credentials embedded across build systems, CI variables, container images, and vendor integrations are hard to replace without breaking production. For many teams, the safest short-term choice is to do nothing, leaving attackers with durable access paths.

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEhP7n8Bj_V61Ktsts8Km8mGFIxpGjyF5Pe1jhNy6BsZ_R3BtTNOvmjXiiUeC0AOkkej1zjMd5lbeqkYvv-YyCr-OrI1ZYl0c15VyCiMnQG1JEc0EzX63XBjXP7Kq8q_sZu4S-Et4DazMC13OxWGE5mGAKdF9wE6oySm1QvCeIc2kSL6sXlgN1_J3IFC8ps/s1700-nu-rw-lo-l85-e365/4.png

## 7. Developer endpoints are the new credential aggregation layer

The [Shai-Hulud 2 supply chain attack](https://blog.gitguardian.com/shai-hulud-2/) gave researchers rare visibility into what secrets actually look like on [compromised developer machines](https://blog.gitguardian.com/honeytokens-on-the-developer-workstation/). Across 6,943 systems, GitGuardian identified 294,842 secret occurrences corresponding to 33,185 unique secrets. On average, each live secret appeared in eight different locations on the same machine, spread across .env files, shell history, IDE configs, cached tokens, and build artifacts. More striking: 59% of compromised machines were CI/CD runners, not personal laptops. Once secrets start sprawling into build infrastructure, they become an organizational exposure problem, not just an individual hygiene issue.

More recently, the[LiteLLM supply chain attack](https://blog.gitguardian.com/litellm-supply-chain-attack/) demonstrated the same pattern, with compromised packages harvesting SSH keys, cloud credentials, and API tokens from developer machines where AI development tools are increasingly concentrated.

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEhGpW0R-sK0XAst0bi7IhKmZ1-b5kEyC8UDqcxLvgOJ2XAW8tZAuLNO26MTRF-3IXqIEcMhCRHHc3uNu0HJqxLELe4jFka4Lzv6QZe5JWG88U4Tl16GJMOCOmv7rNnvGzwUnBfy2pohHUtujv6WciGm2QX8PvDtqgwfelMCiStTwi_wAoNOGv-Gz6dTTFc/s1700-nu-rw-lo-l85-e365/5.png

## 8. MCP servers exposed 24,000+ secrets in their first year

Model Context Protocol (MCP) made AI systems more useful by connecting them to tools and data sources. It also introduced a new class of credential exposure. In 2025, GitGuardian found 24,008 unique secrets in MCP-related config files on public GitHub, with 2,117 verified as valid. As agentic AI adoption accelerates, MCP and similar frameworks will normalize putting credentials into config files, startup flags, and local JSON. The agent ecosystem is expanding faster than security controls can adapt.

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEgBCmwe_drPAwHHLlZsBkcS0cU-WHrHbMtPxdBkGOJ3Ps5GDarUv7N4IhnkLCo-nMXyVaS0oFP27ROArdjeYaL0mkpfJMqBqTOVPvJN8AfHeLF-2WYBD39q0QYdptFoYS_2pepWwPZXok6XhkjEU2awEu3R5qOHuXA4ujDrO_nKLICrm_bGmbbAcgOA4fY/s1700-nu-rw-lo-l85-e365/6.png

## 9. Shift from secrets detection to non-human identity governance

The industry's limiting factor is answering three questions at scale:

- What non-human identities exist in my environment?

- Who owns them?

- What can they access?

Organizations embracing agentic AI need to move beyond detection and build continuous NHI governance. That means eliminating long-lived static credentials wherever possible, adopting [short-lived identity-driven access](https://blog.gitguardian.com/how-managed-identities-are-transforming-multicloud-security/), implementing secrets vaulting as the default developer workflow, and treating every service account, CI job, and AI agent as a governed identity with lifecycle management.

## The Bottom Line

Secrets sprawl is not slowing down. It's accelerating alongside AI adoption, developer productivity tools, and distributed software delivery. The old model of scanning public repos and hoping for compliance is no longer sufficient. Security teams need visibility across internal systems, collaboration tools, container registries, and developer endpoints. They need remediation workflows that can rotate credentials without breaking production. And most importantly, they need to stop treating secrets as isolated incidents and start managing them as part of a broader non-human identity governance program.

The attack surface has changed. The question is whether security programs will change with it.

About the Research

GitGuardian's yearly State of Secrets Sprawl report was published for the 5th time, analyzing billions of public commits on GitHub, monitoring internal incidents across customer environments, and conducting original research on self-hosted infrastructure exposure and supply chain compromises.

https://www.gitguardian.com/state-of-secrets-sprawl-report-2026

Found this article interesting? This article is a contributed piece from one of our valued partners. Follow us on [Google News](https://news.google.com/publications/CAAqLQgKIidDQklTRndnTWFoTUtFWFJvWldoaFkydGxjbTVsZDNNdVkyOXRLQUFQAQ), [Twitter](https://twitter.com/thehackersnews) and [LinkedIn](https://www.linkedin.com/company/thehackernews/) to read more exclusive content we post.

SHARE [](https://thehackernews.com/2026/03/the-state-of-secrets-sprawl-2026-9.html#link_share) [](https://thehackernews.com/2026/03/the-state-of-secrets-sprawl-2026-9.html#link_share) [](https://thehackernews.com/2026/03/the-state-of-secrets-sprawl-2026-9.html#link_share) 

[Tweet](https://thehackernews.com/2026/03/the-state-of-secrets-sprawl-2026-9.html#link_share)

[Share](https://thehackernews.com/2026/03/the-state-of-secrets-sprawl-2026-9.html#link_share)

[Share](https://thehackernews.com/2026/03/the-state-of-secrets-sprawl-2026-9.html#link_share)

Share

SHARE 

[artificial intelligence](https://thehackernews.com/search/label/artificial%20intelligence), [Cloud security](https://thehackernews.com/search/label/Cloud%20security), [cybersecurity](https://thehackernews.com/search/label/cybersecurity), [data breach](https://thehackernews.com/search/label/data%20breach), [DevOps](https://thehackernews.com/search/label/DevOps), [Docker](https://thehackernews.com/search/label/Docker), [GitHub](https://thehackernews.com/search/label/GitHub), [Gitlab](https://thehackernews.com/search/label/Gitlab)

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
