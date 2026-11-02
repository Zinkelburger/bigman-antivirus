---
url: "https://thehackernews.com/2026/08/rust-supply-chain-attack-puts-build.html"
title: "Rust Supply Chain Attack Puts Build-Time Malware in Crates with 245 Million Downloads"
site: "The Hacker News"
captured: "2026-11-02T18:51:10Z"
via: "live"
sha256: "fcbced43e033569b43326b65188db6ed7eb3d7f331ecb45169991a23aa6a9049"
wayback: "https://web.archive.org/web/20261013012001/https://thehackernews.com/2026/08/rust-supply-chain-attack-puts-build.html"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Rust Supply Chain Attack Puts Build-Time Malware in Crates with 245 Million Downloads

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://thehackernews.com/2026/08/rust-supply-chain-attack-puts-build.html>
> Snapshot: <https://web.archive.org/web/20261013012001/https://thehackernews.com/2026/08/rust-supply-chain-attack-puts-build.html>

# [Rust Supply Chain Attack Puts Build-Time Malware in Crates with 245 Million Downloads](https://thehackernews.com/2026/08/rust-supply-chain-attack-puts-build.html)

Swati KhandelwalAug 20, 2026Software Security / Supply Chain Attack

https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEgshVe5DUMbByJkLQGRgak37eOf3HddCtrF6KCJfISPdm7l2ZwLFlnjQM3pN2mxPDpc9lWwSoH2fYfucYKBKk3vpuGAKdR5U685IZh8EFbMxWcyVenuRMyqIgvbFUa0_5Sx9pVl9Imu5omju6i7eRE0sKKxiCSKYEV1Say1LLhQHEufY10dgovzX0Nt7as/s1700-nu-rw-lo-l85-e365/rust.jpg

The Rust Project has deleted malicious versions of three widely used Rust crates from crates.io after a compromised maintainer account published releases that added a typosquatted dependency whose build script downloaded and executed a remote payload during compilation.

The affected releases are arrayref 0.3.10, internment 0.8.7, and append-only-vec 0.1.9, all published from the same owner account on August 20, 2026, and all removed within 86 to 107 minutes.

Because the malicious code sat in the build script of the injected dependency, building a project that resolved it was sufficient to run the payload, and nothing from the crates themselves had to be called.

Developers are advised to search ~/.cargo/registry/cache for the deleted crate files and to pin arrayref at 0.3.9 or earlier, after the Rust Security Response Team unyanked the maliciously-yanked versions during the response.

There is no patched version, no CVE identifier has been assigned, and the RustSec advisories for all three crates initially recorded no evidence that any malicious version was used.

"A new version of the arrayref crate was published with a direct dependency on proc-macro1, which would execute a malicious build script. This compromised version was published on 2026-08-20 and removed approximately 86 minutes later, with no evidence of actual usage," an initial version of [RUSTSEC-2026-0260](https://rustsec.org/advisories/RUSTSEC-2026-0260) said.

"This initial assessment was based on the absence of any reports from users or researchers of the payload executing, cross-checked against [crates.io](https://crates.io/) download counts for the malicious versions during the exposure window. We have [updated the wording](https://github.com/rustsec/advisory-db/pull/3166) since the initial advisory," the Rust Security Response Team told The Hacker News.

"The initial draft advisories used a single rounded estimate as a placeholder for all three crates while remediation was still active. Once we pulled exact publish/deletion timestamps from [crates.io](https://crates.io/)'s logs, we updated each advisory with the crate-specific figure. 86, 90 and 107 minutes respectively," the team said.

"Yes, we have been able to reach the maintainer, and the account is still locked," the team added.

In a separate response to The Hacker News, Wiz Research said [public disclosures](https://github.com/rustsec/advisory-db/issues/3161#issuecomment-5356130756) confirm active exploitation and real-world impact, adding that the impacted packages are present in 35% of cloud and code environments and over 75% of environments that used Rust.

https://thehackernews.uk/ai-governance-platform-d

The Rust Security Response Team said it received the report that the proc-macro1 crate was malicious at 07:15 UTC on August 20 and verified that the crate carried a build script downloading a malicious payload, in [an advisory post](https://blog.rust-lang.org/2026/08/20/supply-chain-attack-on-arrayref/) crediting the Research Team at Nextron Systems GmbH with initially discovering and reporting it.

"We do not believe the author of arrayref to be acting maliciously, but their computer or credentials are likely compromised, and we are attempting to contact them," the Rust Security Response Team said.

The Hacker News confirmed via the crates.io API on August 21 that the sole listed owner of arrayref is user 2402, David Roundy, registered in October 2009.

How the account was compromised has not been disclosed.

The Rust Security Response Team listed the malicious versions it deleted, with the time each was online -

- arrayref@0.3.10: published at 2026-08-20T07:15:00Z, deleted at 08:41:40Z. Online for 86 minutes.
- internment@0.8.7: published at 07:34:07Z, deleted at 09:04:11Z. Online for 90 minutes.
- append-only-vec@0.1.9: published at 07:37:49Z, deleted at 09:25:24Z. Online for 107 minutes.
- proc-macro1, proc-macro-en, aovine, arone, aronenao, and tinymember, any versions.

Each compromised release carried a single added line in its manifest, a dependency on proc-macro1, a typosquat of the ubiquitous proc-macro2 crate. The library source of proc-macro1 is a genuine copy of proc-macro2, so builds completed normally.

[The build script](https://docs.rs/crate/proc-macro1/1.0.107/source/build.rs) reassembles its payload host and command-and-control (C2) address from base64 fragments at build time. It then installs a custom certificate verifier whose three verification methods return success unconditionally, disabling TLS validation. It selects one of four payloads by operating system and CPU architecture.

On Unix and macOS it writes the bytes to /tmp/rust-setup, marks the file executable, and spawns it detached with the C2 address as its first argument. On Windows it writes a PowerShell script to %TEMP% and launches it hidden through a VBScript launcher under wscript.exe, then abandons the child process, a step commented in the source as escaping Cargo's job object so the build does not wait on it.

Delivery relied on the owner account yanking arrayref 0.3.5 through 0.3.9 within the same minute as the malicious publish, leaving the compromised release as the only version Cargo would not warn about, according to [the report filed](https://github.com/rustsec/advisory-db/issues/3161) to the RustSec advisory database by the researcher who hit it.

"Delivery: 0.3.5–0.3.9 are all yanked under the owner account, so cargo's consider updating to a version that is not yanked warning is the lure. That is how I hit it," the reporter, GitHub user jhobern, said.

The Hacker News found via the crates.io API on August 21 that arrayref has 245,385,500 downloads all time and 53,905,601 in the 90 days ending August 20, and that 403 distinct crates on crates.io depend on it. We also verified each hop of the dependency chain named in the report against the crates.io index on August 21: winit requires sctk-adwaita ^0.10.1, which requires tiny-skia ^0.11, which requires arrayref ^0.3.6.

Every requirement in that chain is a caret range on 0.3.x, and a caret range on 0.3.x accepts 0.3.10. The same check found that blake3 declared arrayref as a dependency through version 1.8.6 and does not in 1.8.7, published at 09:09 UTC on August 20, and that blake2b_simd and blake2s_simd dropped the same dependency in releases published at 09:25 and 09:26 UTC that morning.

The stage-2 implant beacons over HTTPS POST to the path /49890878, persists through a Registry Run key on Windows, a LaunchAgent on macOS, and a systemd user service on Linux, and supports four commands covering termination, C2 reconfiguration, persistence installation, and downloading and running further scripts, according to [Wiz](https://www.wiz.io/blog/rust-supply-chain-attack-on-arrayref-significant-overlap-with-dprk-campaigns), which said it reads Chrome, Brave, and Edge profiles for saved logins and extension settings by querying the browsers' SQLite credential stores directly.

The [Nextron researcher analysis](https://gist.github.com/marius-benthin/273aa302ac9fb36e1c309a9479c5a8cf) says the analysed Windows stage queries only the origin_url and username_value columns and does not directly extract password_value, but that analysis covered the Windows payload alone, with the Linux and macOS payloads hashed and not analysed. The same analysis notes the crate may be triggered by cargo build, cargo check, and cargo test.

[StepSecurity](https://www.stepsecurity.io/blog/arrayref-rust-crate-supply-chain-attack) shared the following indicators of compromise (IoCs) -

- Network: 23.254.165.112:9089 (payload host), 23.254.165.112:443 (C2), hwsrv-798836.hostwindsdns.com
- Files: /tmp/rust-setup, %TEMP%\rust-setup.ps1, %TEMP%\rust-setup-launch.vbs
- Binaries: rust-crate_0.1.0, _0.2.0, _0.3.0, _0.4.0
- Accounts: dtolney (crates.io id 438608), impersonator; droundy, legitimate owner, presumed compromised
- Email: rchaitm@gmail.com, forged author metadata

Wiz said the infrastructure substantially overlaps with recent North Korean supply chain attacks, naming [the Mastra npm compromise](https://thehackernews.com/2026/06/144-mastra-npm-packages-compromised-via.html) and the axios compromise.

https://thehackernews.uk/network-defense-dd

Microsoft assesses with high confidence that the Mastra activity is attributable to Sapphire Sleet, and Google Threat Intelligence Group (GTIG) [attributed the axios compromise](https://thehackernews.com/2026/04/google-attributes-axios-npm-supply.html) to an actor it now tracks as MIDNIGHT NEPTUNE, formerly known as UNC1069. No vendor has attributed the crates.io incident to a named actor.

"While the malicious versions of axios were removed from the npm registry within three hours of their release, the scope of the compromise is estimated to be broad, as the package has over 100 million weekly downloads," GTIG and Mandiant said in [a July 30 report](https://cloud.google.com/blog/topics/threat-intelligence/mitigation-guidance-for-supply-chain-compromise) recommending cooling windows on newly published third-party assets.

Cargo has no shipped equivalent. A pull request stabilizing a global-min-publish-age setting, which would hold back dependencies younger than a configured age, entered its final comment period on August 18, two days before the attack, and remained [open and unmerged](https://github.com/rust-lang/cargo/pull/17335) as of August 21. GitHub shipped [a similar cooldown default](https://thehackernews.com/2026/07/github-adds-3-day-dependabot-cooldown.html) for Dependabot in July.

In a September 2025 case, two malicious crates impersonating a logging library [executed only at runtime](https://thehackernews.com/2025/09/malicious-rust-crates-steal-solana-and.html), a distinction crates.io drew at the time. Hunt.io. The public primary sources reviewed for this story corroborate the two old authentication bypasses, the p2pwn tool configuration, and the underlying serial-number P2P mechanism, but do not independently confirm those CameraSwarm counts.

Found this article interesting? Follow us on [Google News](https://news.google.com/publications/CAAqLQgKIidDQklTRndnTWFoTUtFWFJvWldoaFkydGxjbTVsZDNNdVkyOXRLQUFQAQ), [Twitter](https://twitter.com/thehackersnews) and [LinkedIn](https://www.linkedin.com/company/thehackernews/) to read more exclusive content we post.

SHARE [](https://thehackernews.com/2026/08/rust-supply-chain-attack-puts-build.html#link_share) [](https://thehackernews.com/2026/08/rust-supply-chain-attack-puts-build.html#link_share) [](https://thehackernews.com/2026/08/rust-supply-chain-attack-puts-build.html#link_share) 

[Tweet](https://thehackernews.com/2026/08/rust-supply-chain-attack-puts-build.html#link_share)

[Share](https://thehackernews.com/2026/08/rust-supply-chain-attack-puts-build.html#link_share)

[Share](https://thehackernews.com/2026/08/rust-supply-chain-attack-puts-build.html#link_share)

Share

SHARE 

[Developer Security](https://thehackernews.com/search/label/Developer%20Security), [Linux security](https://thehackernews.com/search/label/Linux%20security), [MacOS](https://thehackernews.com/search/label/MacOS), [Malware](https://thehackernews.com/search/label/Malware), [Open Source](https://thehackernews.com/search/label/Open%20Source), [Package Security](https://thehackernews.com/search/label/Package%20Security), [Software Security](https://thehackernews.com/search/label/Software%20Security), [Supply Chain](https://thehackernews.com/search/label/Supply%20Chain), [Threat Intelligence](https://thehackernews.com/search/label/Threat%20Intelligence), [Windows Security](https://thehackernews.com/search/label/Windows%20Security)

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
