---
url: "https://thehackernews.com/2026/01/north-korea-linked-hackers-target.html"
title: "North Korea-Linked Hackers Target Developers via Malicious VS Code Projects"
site: "The Hacker News"
captured: "2026-11-02T18:50:02Z"
via: "live"
sha256: "9a3faec7f3608744d26a8c79fb449bee0a0730ea8652bfd8dcfa34ec14bf833e"
wayback: "https://web.archive.org/web/20261013052905/https://thehackernews.com/2026/01/north-korea-linked-hackers-target.html"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# North Korea-Linked Hackers Target Developers via Malicious VS Code Projects

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://thehackernews.com/2026/01/north-korea-linked-hackers-target.html>
> Snapshot: <https://web.archive.org/web/20261013052905/https://thehackernews.com/2026/01/north-korea-linked-hackers-target.html>

# [North Korea-Linked Hackers Target Developers via Malicious VS Code Projects](https://thehackernews.com/2026/01/north-korea-linked-hackers-target.html)

Ravie LakshmananJan 20, 2026Cyber Espionage / Malware

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEg0h2bAZNzwVEuQKgQlWm86TvDCUL308-g1xrbV2ZuCKJdiuCxebUL3w04zPuBIafTaSB4SkyqbdIb0Kpqn_jCUQMB8j9s-amDohBq4Lw_-Q9G0ZVIkmKIIP54wUsq7wvtgV03b9x_I-tpCDytn4Y6UHgBiPPUXQ9sVSsQVyjSYMrexTs7ZSNetSjFybpJp/s1700-nu-rw-lo-l85-e365/vscode.png

The North Korean threat actors associated with the long-running [Contagious Interview](https://thehackernews.com/2025/11/north-korean-hackers-deploy-197-npm.html) campaign have been observed using malicious Microsoft Visual Studio Code (VS Code) projects as lures to deliver a backdoor on compromised endpoints.

The latest finding demonstrates continued evolution of the new tactic that was first discovered in December 2025, Jamf Threat Labs said.

"This activity involved the deployment of a backdoor implant that provides remote code execution capabilities on the victim system," security researcher Thijs Xhaflaire [said](https://www.jamf.com/blog/threat-actors-expand-abuse-of-visual-studio-code/) in a report shared with The Hacker News.

First [disclosed](https://thehackernews.com/2025/12/north-korea-linked-actors-exploit.html) by OpenSourceMalware last month, the attack essentially involves instructing prospective targets to clone a repository on GitHub, GitLab, or Bitbucket, and launch the project in VS Code as part of a supposed job assessment.

The end goal of these efforts is to abuse [VS Code task configuration files](https://code.visualstudio.com/docs/debugtest/tasks) to execute malicious payloads staged on Vercel domains, depending on the operating system on the infected host. Notably, the task is configured with the "runOn: folderOpen" option so that it runs every time that file or any other file in the project folder is opened in VS Code. This ultimately leads to the deployment of BeaverTail and InvisibleFerret.

| https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEh8FFUdFZHFEWIQX3H2Efgi1dfWwGXto8YrnkikLxb7Q6l0tzYvXLu4aI8ZQ2t6penHUGrEGn6byAJGhVOGwRPtuZ3FRWUsxeOtGrpG1hNubRlUf_XfS7PQSzglQlL1mYXGWemwTFs2ITESMZnR-0qQ-KCZt3qEFMRBi6QDN_qZ2DgCRyC_UrHWJ3RTK0FN/s1700-nu-rw-lo-l85-e365/clones.jpg

| Contagious Interview Using VS Code Tasks

Subsequent iterations of the campaign have been [found](https://opensourcemalware.com/blog/contagious-interview-malicious-dictionary) to conceal sophisticated multi-stage droppers in task configuration files by disguising the malware as harmless spell-check dictionaries as a fallback mechanism in the event the task is unable to retrieve the payload from the Vercel domain.

Like before, the obfuscated JavaScript embedded with these files is executed as soon as the victim opens the project in the integrated development environment (IDE). It establishes communication with a remote server ("ip-regions-check.vercel[.]app") and executes any JavaScript code received from it. The final stage delivered as part of the attack is another heavily obfuscated JavaScript.

https://thehackernews.uk/ai-governance-platform-d

Jamf said it discovered yet another change in this campaign, with the threat actors using a previously undocumented infection method to deliver a backdoor that offers remote code execution capabilities on the compromised host. The starting point of the attack chain is no different in that it's activated when the victim clones and opens a malicious Git repository using VS Code.

"When the project is opened, Visual Studio Code prompts the user to trust the repository author," Xhaflaire explained. "If that trust is granted, the application automatically processes the repository’s tasks.json configuration file, which can result in embedded arbitrary commands being executed on the system."

| https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEjhexwC-0vjQtkW9-dOHa2m1XPg2Eku3iIMwMFK0x_fb2a-0KZ5hWThMhSB97g3Uq3eeUG0MUlzfhjm9mW3KTn8blTwnJ46zSmHs54KcPJE_v0ErZXAnO4_mAcHYh9kgjK8W-TGkRYGh2KahyphenhyphenccQIF7q8xssmKZjM2sjH-wJ6cbWMJL8FI_Fk8lH8ZCty55/s1700-nu-rw-lo-l85-e365/git.png

| Using a seemingly harmless dictionary file as a backup option (Source: OpenSourceMalware)

"On macOS systems, this results in the execution of a background shell command that uses nohup bash -c in combination with curl -s to retrieve a JavaScript payload remotely and pipe it directly into the Node.js runtime. This allows execution to continue independently if the Visual Studio Code process is terminated, while suppressing all command output."

The JavaScript payload, hosted on Vercel, contains the main backdoor logic to establish a persistent execution loop that harvests basic host information and communicates with a remote server to facilitate remote code execution, system fingerprinting, and continuous communication.

In one case, the Apple device management firm said it observed more JavaScript instructions being executed roughly eight minutes after the initial infection. The newly downloaded JavaScript is designed to beacon to the server every five seconds, run additional JavaScript, and erase traces of its activity upon receiving a signal from the operator. It's suspected that the script may have been generated using an artificial intelligence (AI) tool owing to the presence of inline comments and phrasing in the source code.

Threat actors with ties to the Democratic People's Republic of Korea (DPRK) are known to specifically go after software engineers, particular those working in cryptocurrency, blockchain, and fintech sectors, as they often tend to have privileged access to financial assets, digital wallets, and technical infrastructure.

Compromising their accounts and systems could allow the attackers unauthorized access to source code, intellectual property, internal systems, and siphon digital assets. These consistent changes to their tactics are seen as an effort to achieve more success in their cyber espionage and financial goals to support the heavily-sanctioned regime.

The development comes as Red Asgard [detailed](https://redasgard.com/blog/hunting-lazarus-contagious-interview-c2-infrastructure) its investigation into a malicious repository that has been found to use a VS Code task configuration to fetch obfuscated JavaScript designed to drop a full-featured backdoor named [Tsunami](https://thehackernews.com/2025/11/north-korean-hackers-turn-json-services.html) (aka TsunamiKit) along with an XMRig cryptocurrency miner.

https://thehackernews.uk/network-defense-dd

Another analysis from Security Alliance last week has also [laid out](https://radar.securityalliance.org/vs-code-tasks-abuse-by-contagious-interview-dprk/) the campaign's abuse of VS Code tasks in an attack where an unspecified victim was approached on LinkedIn, with the threat actors claiming to be the chief technology officer of a project called Meta2140 and sharing a Notion[.]so link that contains a technical assessment and a URL to a Bitbucket repository hosting the malicious code.

Interestingly, the attack chain is engineered to fallback to two other methods: installing a [malicious npm dependency](https://thehackernews.com/2025/11/north-korean-hackers-deploy-197-npm.html) named "[grayavatar](https://www.npmjs.com/package/grayavatar?activeTab=versions)" or running JavaScript code that's responsible for retrieving a sophisticated Node.js controller, which, in turn, runs five distinct modules to log keystrokes, take screenshots, scans the system's home directory for sensitive files, [substitute wallet addresses](https://thehackernews.com/2025/03/new-massjacker-malware-targets-piracy.html) copied to the clipboard, credentials from web browsers, and establish a persistent connection to a remote server.

The malware then proceeds to set up a parallel Python environment using a stager script that enables data collection, cryptocurrency mining using XMRig, keylogging, and the deployment of AnyDesk for remote access. It's worth noting that the Node.js and Python layers are referred to as BeaverTail and InvisibleFerret, respectively.

These findings indicate that the state-sponsored actors are experimenting with multiple delivery methods in tandem to increase the likelihood of success of their attacks.

"While monitoring, we've seen the malware that is being delivered change very quickly over a short amount of time," Jaron Bradley, director of Jamf Threat Labs, told The Hacker News. "It's worth noting that the payload we observed for macOS was written purely in JavaScript and had many signs of being AI assisted. It's difficult to know exactly how quickly attackers are changing their workflows, but this particular threat actor has a reputation for adapting quickly."

To counter the threat, developers are advised to exercise caution when interacting with third-party repositories, mainly those originating from unfamiliar sources or shared directly during coding tests; review source code contents before opening them in VS Code; install only vetted npm packages.

"This activity highlights the continued evolution of DPRK-linked threat actors, who consistently adapt their tooling and delivery mechanisms to integrate with legitimate developer workflows," Jamf said. "The abuse of Visual Studio Code task configuration files and Node.js execution demonstrates how these techniques continue to evolve alongside commonly used development tools."

Found this article interesting? Follow us on [Google News](https://news.google.com/publications/CAAqLQgKIidDQklTRndnTWFoTUtFWFJvWldoaFkydGxjbTVsZDNNdVkyOXRLQUFQAQ), [Twitter](https://twitter.com/thehackersnews) and [LinkedIn](https://www.linkedin.com/company/thehackernews/) to read more exclusive content we post.

SHARE [](https://thehackernews.com/2026/01/north-korea-linked-hackers-target.html#link_share) [](https://thehackernews.com/2026/01/north-korea-linked-hackers-target.html#link_share) [](https://thehackernews.com/2026/01/north-korea-linked-hackers-target.html#link_share) 

[Tweet](https://thehackernews.com/2026/01/north-korea-linked-hackers-target.html#link_share)

[Share](https://thehackernews.com/2026/01/north-korea-linked-hackers-target.html#link_share)

[Share](https://thehackernews.com/2026/01/north-korea-linked-hackers-target.html#link_share)

Share

SHARE 

[cryptocurrency](https://thehackernews.com/search/label/cryptocurrency), [cyber espionage](https://thehackernews.com/search/label/cyber%20espionage), [cybersecurity](https://thehackernews.com/search/label/cybersecurity), [Malware](https://thehackernews.com/search/label/Malware), [North Korea](https://thehackernews.com/search/label/North%20Korea), [remote code execution](https://thehackernews.com/search/label/remote%20code%20execution), [supply chain attack](https://thehackernews.com/search/label/supply%20chain%20attack), [Visual Studio](https://thehackernews.com/search/label/Visual%20Studio)

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
