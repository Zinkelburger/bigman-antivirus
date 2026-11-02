---
url: "https://thehackernews.com/2025/08/transparent-tribe-targets-indian-govt.html"
title: "Transparent Tribe Targets Indian Govt With Weaponized Desktop Shortcuts via Phishing"
site: "The Hacker News"
captured: "2026-11-02T18:48:48Z"
via: "live"
sha256: "abd4b8d305f39a773cf0cf9e694018e93ad5796b6cb6077cc16eebf8be9d2a80"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Transparent Tribe Targets Indian Govt With Weaponized Desktop Shortcuts via Phishing

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://thehackernews.com/2025/08/transparent-tribe-targets-indian-govt.html>

# [Transparent Tribe Targets Indian Govt With Weaponized Desktop Shortcuts via Phishing](https://thehackernews.com/2025/08/transparent-tribe-targets-indian-govt.html)

Ravie LakshmananAug 25, 2025Malware / Cyber Attack

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEid4XIqtUx4dvIytbJUeM5krSLJ26aFucK4uEgLFOnOEx08v0hbzkeZ2pQWeRq2upA7ndK78jnIV0Tmv_LpV7JLOJsR2FEl5W5y57mbfqrSn3bUgg-uk1SZtLven_5sstd7jitvS3MhYQroW-5SpPm7yLvGIDsafUW_g6eEXDjHs9ZA7vThyphenhyphen8nDdM6kmxc7/s1700-nu-rw-lo-l85-e365/icon.jpg

The advanced persistent threat (APT) actor known as Transparent Tribe has been observed targeting both Windows and BOSS (Bharat Operating System Solutions) Linux systems with malicious Desktop shortcut files in attacks targeting Indian Government entities.

"Initial access is achieved through spear-phishing emails," CYFIRMA [said](https://www.cyfirma.com/research/apt36-targets-indian-boss-linux-systems-with-weaponized-autostart-files/). "Linux BOSS environments are targeted via weaponized .desktop shortcut files that, once opened, download and execute malicious payloads."

Transparent Tribe, also called APT36, is assessed to be of Pakistani origin, with the group – along with its sub-cluster SideCopy – having a [storied](https://thehackernews.com/2025/03/apt36-spoofs-india-post-website-to.html) [history](https://thehackernews.com/2025/04/pakistan-linked-hackers-expand-targets.html) of breaking into [Indian government institutions](https://thehackernews.com/2025/07/tag-140-deploys-drat-v2-rat-targeting.html) with a variety of remote access trojans (RATs).

The latest dual-platform demonstrates the adversarial collective's continued sophistication, allowing it to broaden its targeting footprint and ensure access to compromised environments.

https://thehackernews.uk/ai-governance-platform-d

The attack chains begin with phishing emails bearing supposed meeting notices, which, in reality, are nothing but booby-trapped Linux desktop shortcut files ("Meeting_Ltr_ID1543ops.pdf.desktop"). These files masquerade as PDF documents to trick recipients into opening them, leading to the execution of a shell script.

The shell script serves as a dropper to fetch a hex-encoded file from an attacker-controlled server ("securestore[.]cv") and save it to disk as an ELF binary, while simultaneously opening a decoy PDF hosted on Google Drive by launching Mozilla Firefox. The Go-based binary, for its part, establishes contact with a hard-coded command-and-control (C2) server, modgovindia[.]space:4000, to receive commands, fetch payloads, and exfiltrate data.

The malware also establishes persistence by means of a cron job that executes the main payload automatically after a system reboot or process termination.

Cybersecurity company CloudSEK, which also [independently reported](https://www.cloudsek.com/blog/investigation-report-apt36-malware-campaign-using-desktop-entry-files-and-google-drive-payload-delivery) the activity, said the malware performs system reconnaissance and is equipped to carry out a series of dummy anti-debugging and anti-sandbox checks in a bid to throw off emulators and static analyzers.

Furthermore, Hunt.io's [analysis](https://hunt.io/blog/apt36-india-infrastructure-attacks) of the campaign has revealed that the attacks are designed to deploy a known Transparent Tribe backdoor called [Poseidon](https://thehackernews.com/2023/04/pakistani-hackers-use-linux-malware.html) that enables data collection, long-term access, credential harvesting, and potentially lateral movement.

"APT36's capability to customize its delivery mechanisms according to the victim's operating environment thereby increases its chances of success while maintaining persistent access to critical government infrastructure and evading traditional security controls," CYFIRMA said.

The disclosure comes weeks after the Transparent Tribe actors were observed targeting Indian defense organizations and related government entities using spoofed domains with the ultimate goal of stealing credentials and two-factor authentication (2FA) codes. It's believed that users are redirected to these URLs through spear-phishing emails.

"Upon entering a valid email ID in the initial phishing page and clicking the 'Next' button, the victim is redirected to a second page that prompts the user to input their email account password and the Kavach authentication code," CYFIRMA [said](https://www.cyfirma.com/research/apt36-a-phishing-campaign-targeting-indian-government-entities/).

It's worth noting that the targeting of Kavach, a 2FA solution used by the Indian government agencies to improve account security, is a [tried-and-tested tactic](https://thehackernews.com/2022/03/new-hacking-campaign-by-transparent.html) adopted by [Transparent Tribe and SideCopy](https://thehackernews.com/2022/12/researchers-warn-of-kavach-2fa-phishing.html) since [early 2022](https://thehackernews.com/2023/04/pakistani-hackers-use-linux-malware.html).

https://thehackernews.uk/network-defense-dd

"The use of typo-squatted domains combined with infrastructure hosted on Pakistan-based servers is consistent with the group's established tactics, techniques, and procedures," the company said.

The findings also follow the discovery of a [separate campaign](https://strikeready.com/blog/apt-android-phishing-microsoft/) undertaken by a South Asian APT to strike Bangladesh, Nepal, Pakistan, Sri Lanka, and Turkey through spear-phishing emails that are engineered for credential theft using lookalike pages hosted on Netlify and Pages.dev.

"These campaigns mimic official communication to trick victims into entering credentials on fake login pages," Hunt.io [said](https://hunt.io/blog/apt-sidewinder-netlify-government-phishing) earlier this month, attributing it to a hacking group called [SideWinder](https://thehackernews.com/2025/05/south-asian-ministries-hit-by.html).

"Spoofed Zimbra and Secure Portal Pages were made to look like official government email, file-sharing, or document upload services, prompting victims to submit credentials through fake login panels."

### Update

Nextron Systems, in a new analysis published on August 29, 2025, said it observed a spear-phishing campaign undertaken by Transparent Tribe targeting organizations in India using weaponized .desktop files to deliver an open-source remote management tool known as [MeshAgent](https://github.com/Ylianst/MeshAgent).

The Linux-focused infection method, according to security researcher Pierre-Henri Pezier, [triggers](https://www.nextron-systems.com/2025/08/29/sindoor-dropper-new-phishing-campaign/) a "heavily obfuscated execution chain built to evade both static and dynamic detection. The chain ultimately delivers a MeshAgent payload, which gives the attacker full remote access to the system."

Once launched, the malicious .desktop file is crafted to display a decoy PDF document, while the MeshAgent payload is stealthily installed in the background by means of a UPX-packed Go dropper dubbed Sindoor Dropper. MeshAgent, upon execution, connects to an actor-controlled server to receive commands, transfer files, and maintain persistence on the infected host.

"By using a .desktop file, the attacker can place executable files on the victim's system without needing elevated privileges," Pezier said. "MeshAgent provides the attacker with full remote access to the compromised system, enabling a wide range of post-exploitation activities such as activity monitoring, lateral movement, data exfiltration, and persistent access."

(The story was updated after publication on August 30, 2025, to include additional insights from Nextron Systems.)

Found this article interesting? Follow us on [Google News](https://news.google.com/publications/CAAqLQgKIidDQklTRndnTWFoTUtFWFJvWldoaFkydGxjbTVsZDNNdVkyOXRLQUFQAQ), [Twitter](https://twitter.com/thehackersnews) and [LinkedIn](https://www.linkedin.com/company/thehackernews/) to read more exclusive content we post.

SHARE [](https://thehackernews.com/2025/08/transparent-tribe-targets-indian-govt.html#link_share) [](https://thehackernews.com/2025/08/transparent-tribe-targets-indian-govt.html#link_share) [](https://thehackernews.com/2025/08/transparent-tribe-targets-indian-govt.html#link_share) 

[Tweet](https://thehackernews.com/2025/08/transparent-tribe-targets-indian-govt.html#link_share)

[Share](https://thehackernews.com/2025/08/transparent-tribe-targets-indian-govt.html#link_share)

[Share](https://thehackernews.com/2025/08/transparent-tribe-targets-indian-govt.html#link_share)

Share

SHARE 

[Advanced Persistent Threat](https://thehackernews.com/search/label/Advanced%20Persistent%20Threat), [Credential Theft](https://thehackernews.com/search/label/Credential%20Theft), [cybersecurity](https://thehackernews.com/search/label/cybersecurity), [linux](https://thehackernews.com/search/label/linux), [Malware](https://thehackernews.com/search/label/Malware), [Phishing](https://thehackernews.com/search/label/Phishing), [Remote Access Trojan](https://thehackernews.com/search/label/Remote%20Access%20Trojan), [Transparent Tribe](https://thehackernews.com/search/label/Transparent%20Tribe), [two-factor authentication](https://thehackernews.com/search/label/two-factor%20authentication)

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
