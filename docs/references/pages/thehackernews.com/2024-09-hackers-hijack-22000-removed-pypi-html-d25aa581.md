---
url: "https://thehackernews.com/2024/09/hackers-hijack-22000-removed-pypi.html"
title: "Researchers Find Over 22,000 Removed PyPI Packages at Risk of Revival Hijack"
site: "The Hacker News"
captured: "2026-11-02T18:46:23Z"
via: "live"
sha256: "3b83609c6c5d0a1db425b939d82d55f401ad9878f0edba35aaff1d450639eaf3"
wayback: "https://web.archive.org/web/20261025111108/https://thehackernews.com/2024/09/hackers-hijack-22000-removed-pypi.html"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Researchers Find Over 22,000 Removed PyPI Packages at Risk of Revival Hijack

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://thehackernews.com/2024/09/hackers-hijack-22000-removed-pypi.html>
> Snapshot: <https://web.archive.org/web/20261025111108/https://thehackernews.com/2024/09/hackers-hijack-22000-removed-pypi.html>

# [Researchers Find Over 22,000 Removed PyPI Packages at Risk of Revival Hijack](https://thehackernews.com/2024/09/hackers-hijack-22000-removed-pypi.html)

Ravie LakshmananSep 04, 2024

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEj8Hel-8ANHTua9rNhRLRWwf_lWbIrRqnJICcLADWnyPcH-6Lt7nO_FLlyA3rv3jZJZNS-fwVLlVWBvJLEBursDobXtXZn9_YkCe8Sjy0z8hYWA4vS0Lu0CwHX7UlwSRaPqgjtJxUUshj2FwrbfXfiF7R4w2Q8iAwsHiKSuPwKpjxf3rJFfIqK2R-SPXIJR/s1700-nu-rw-lo-l85-e365/pipy.jpg

A new supply chain attack technique targeting the Python Package Index (PyPI) registry has been exploited in the wild in an attempt to infiltrate downstream organizations.

It has been codenamed Revival Hijack by software supply chain security firm JFrog, which said the attack method could be used to hijack 22,000 existing PyPI packages and result in "hundreds of thousands" of malicious package downloads. These susceptible packages have more than 100,000 downloads or have been active for over six months.

"This attack technique involves hijacking PyPI software packages by manipulating the option to re-register them once they're removed from PyPI's index by the original owner," JFrog security researchers Andrey Polkovnychenko and Brian Moussalli said in a [report](https://jfrog.com/blog/revival-hijack-pypi-hijack-technique-exploited-22k-packages-at-risk/) shared with The Hacker News.

At its core, the attack hinges on the fact that Python packages published in the PyPI repository may get removed, making available the [names of those deleted projects](https://discuss.python.org/t/stop-allowing-deleting-things-from-pypi/17227) for registration to any other user.

https://thehackernews.uk/ai-governance-platform-d

Statistics shared by JFrog show that about 309 packages are removed each month on average. These could happen for any number of reasons: Lack of maintenance (i.e., abandonware), package getting re-published under a different name, or introducing the same functionality into official libraries or built-in APIs.

This also poses a lucrative attack surface that's more effective than typosquatting and which an attacker, using their own accounts, could exploit to publish malicious packages under the same name and a higher version to infect developer environments.

"The technique does not rely on the victim making a mistake when installing the package," the researchers said, pointing out how Revival Hijack can yield better results from the point of view of an adversary. "Updating a 'once safe' package to its latest version is viewed as a safe operation by many users."

While PyPI does have safeguards in place against author impersonation and typosquatting attempts, JFrog's analysis found that running the "[pip list --outdated](https://pip.pypa.io/en/stable/cli/pip_list/)" command lists the counterfeit package as a new version of the original package, wherein the former corresponds to a different package from an entirely different author.

Even more concerning, running the "[pip install –upgrade](https://pip.pypa.io/en/stable/cli/pip_install/)" command replaces the actual package with the phony one without not so much of a warning that the package's author has changed, potentially exposing unwitting developers to a huge software supply chain risk.

JFrog said it took the step of creating a new PyPI user account called "[security_holding](https://pypi.org/user/security_holding/)" that it used to safely hijack the susceptible packages and replace them with empty placeholders so as to prevent malicious actors from capitalizing on the removed packages.

Additionally, each of these packages has been assigned the version number as 0.0.0.1 – the opposite of a [dependency confusion attack](https://thehackernews.com/2024/04/apache-cordova-app-harness-targeted-in.html) scenario – to avoid getting pulled by developers when running a pip upgrade command.

What's more disturbing is that Revival Hijack has already been exploited in the wild, with an unknown threat actor called Jinnis introducing a benign version of a package named "[pingdomv3](https://www.pepy.tech/projects/pingdomv3?versions=0.0.2&versions=1.0.0&versions=1.1.0)" on March 30, 2024, the same day the original owner (cheneyyan) removed the package from PyPI.

On April 12, 2024, the new developer is said to have released an update containing a Base64-encoded payload that checks for the presence of the "[JENKINS_URL](https://www.jenkins.io/doc/book/managing/cli/)" environment variable, and if present, executes an unknown next-stage module retrieved from a remote server.

https://thehackernews.uk/network-defense-dd

"This suggests that the attackers either delayed the delivery of the attack or designed it to be more targeted, possibly limiting it to a specific IP range," JFrog said.

The new attack is a sign that threat actors are eyeing supply chain attacks on a broader scale by targeting deleted PyPI packages in order to expand the reach of the campaigns. Organizations and developers are recommended to inspect their DevOps pipelines to ensure that they are not installing packages that have been already removed from the repository.

"Using a vulnerable behavior in the handling of removed packages allowed attackers to hijack existing packages, making it possible to install it to the target systems without any changes to the user's workflow," said Moussalli, JFrog Security Research Team Lead.

"The PyPI package attack surface is continually growing. Despite proactive intervention here, users should always stay vigilant and take the necessary precautions to protect themselves and the PyPI community from this hijack technique."

Found this article interesting? Follow us on [Google News](https://news.google.com/publications/CAAqLQgKIidDQklTRndnTWFoTUtFWFJvWldoaFkydGxjbTVsZDNNdVkyOXRLQUFQAQ), [Twitter](https://twitter.com/thehackersnews) and [LinkedIn](https://www.linkedin.com/company/thehackernews/) to read more exclusive content we post.

SHARE [](https://thehackernews.com/2024/09/hackers-hijack-22000-removed-pypi.html#link_share) [](https://thehackernews.com/2024/09/hackers-hijack-22000-removed-pypi.html#link_share) [](https://thehackernews.com/2024/09/hackers-hijack-22000-removed-pypi.html#link_share) 

[Tweet](https://thehackernews.com/2024/09/hackers-hijack-22000-removed-pypi.html#link_share)

[Share](https://thehackernews.com/2024/09/hackers-hijack-22000-removed-pypi.html#link_share)

[Share](https://thehackernews.com/2024/09/hackers-hijack-22000-removed-pypi.html#link_share)

Share

SHARE 

[cybersecurity](https://thehackernews.com/search/label/cybersecurity), [DevOps](https://thehackernews.com/search/label/DevOps), [Malware](https://thehackernews.com/search/label/Malware), [Open Source](https://thehackernews.com/search/label/Open%20Source), [PyPI](https://thehackernews.com/search/label/PyPI), [Python](https://thehackernews.com/search/label/Python), [Software Supply Chain](https://thehackernews.com/search/label/Software%20Supply%20Chain), [Threat Intelligence](https://thehackernews.com/search/label/Threat%20Intelligence)

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
