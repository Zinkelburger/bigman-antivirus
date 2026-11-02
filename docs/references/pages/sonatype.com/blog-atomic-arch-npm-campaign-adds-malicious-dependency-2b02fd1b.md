---
url: "https://www.sonatype.com/blog/atomic-arch-npm-campaign-adds-malicious-dependency"
title: "Atomic Arch npm Campaign Adds Malicious Dependency"
captured: "2026-11-02T18:59:03Z"
via: "live"
sha256: "a7de7d2ebe41bb88f80bb87bdd07c916124e25dd1934ca77cb6c98e003e52142"
wayback: "https://web.archive.org/web/20261012071651/https://www.sonatype.com/blog/atomic-arch-npm-campaign-adds-malicious-dependency"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# Atomic Arch npm Campaign Adds Malicious Dependency

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.sonatype.com/blog/atomic-arch-npm-campaign-adds-malicious-dependency>
> Snapshot: <https://web.archive.org/web/20261012071651/https://www.sonatype.com/blog/atomic-arch-npm-campaign-adds-malicious-dependency>

[Resources](https://www.sonatype.com/resources)  [Blog](https://www.sonatype.com/blog)  Atomic Arch: Attackers Hijack Trusted AUR Packages to ...

# Atomic Arch: Attackers Hijack Trusted AUR Packages to Deliver Rootkit-Like Malware

June 11, 2026  By [Sonatype Research Team](https://www.sonatype.com/blog/author/sonatype-security-research-team)

5 minute read time

Atomic Arch npm Campaign Adds Malicious Dependency

7:19

## TL;DR

-

On June 11, 2026, Sonatype researchers uncovered Atomic Arch, a new campaign targeting orphaned packages in the Arch User Repository in which attackers take over legitimate, abandoned AUR projects and modify PKGBUILDS to install a malicious npm package during installation.

-

Analysis of atomic-lockfile, the malicious dependency, found a bundled Linux payload with functionality tied to credential harvesting, stealth, anti-debugging, and potential data exfiltration.

-

On June 12, 2026, a second wave emerged, using Bun-based installation paths in some affected packages rather than npm alone. Researchers have now identified multiple packages associated with the campaign, including atomic-lockfile, js-digest, and lockfile-js.

-

Preliminary analysis suggests the campaign may now affect approximately 1,500 packages across multiple waves of activity.

Update, June 12, 2026: Since Sonatype's initial publication, researchers have observed additional activity believed to be related to Atomic Arch, including new malicious packages, alternate installation mechanisms, and a significant increase in the number of affected AUR packages. Analysis remains ongoing and this post will be updated as new information becomes available.

Sonatype researchers have identified a malicious package campaign, dubbed Atomic Arch, that targets orphaned packages in the Arch User Repository (AUR).

Flagged by Sonatype Engineer Eyad Hasan, the campaign is taking control of legitimate AUR projects after maintainers abandon them. Once ownership is obtained, package build instructions are modified to install a malicious npm package called atomic-lockfile, which then deploys an additional payload onto victim systems.

Sonatype Research Labs took a closer look at the malicious atomic-lockfile dependency to understand attacker motives, tracking Atomic Arch as [Sonatype-2026-003775](https://guide.sonatype.com/vulnerability/sonatype-2026-003775/sonatype-research) with a CVSS of 8.7. Analysis of the campaign is ongoing. We are tracking the malicious NPM packages from the second wave as [Sonatype-2026-003808](https://guide.sonatype.com/vulnerability/sonatype-2026-003808). Sonatype will update this blog as more information is available.

Atomic Arch highlights a growing supply chain risk: Attackers no longer need to create trust. Sometimes they can inherit it.

## One Malicious Dependency, Massive Downstream Impact

In this case, attackers did not modify the orphaned packages "adopted" from AUR. They modified the packages' PKGBUILD to introduce a post-install script that executes npm install atomic-lockfile minimist chalk during package installation, causing affected systems to retrieve and install the npm package atomic-lockfile.

Reminiscent of the [axios compromise](https://www.sonatype.com/blog/axios-compromise-on-npm-introduces-hidden-malicious-package), in which hijackers added the malicious plain-crypto-js@4.2.1 dependency, this is an effective way for attackers to get past traditional detection tools because the trusted packages themselves do not contain the malicious code.

The campaign initially appeared to affect a few dozen packages. That assessment changed rapidly.

Within 24 hours of Sonatype's initial discovery, researchers observed additional malicious packages associated with the campaign, including js-digest and lockfile-js. Researchers also identified at least one wave that appears to use Bun rather than npm as part of the installation process.

Early reporting now suggests the campaign may have affected hundreds of additional AUR packages, potentially bringing the total number of impacted packages around 1,500. Sonatype's investigation remains ongoing and package counts may continue to change as maintainers audit affected repositories.

Affected hosts should be treated as compromised. Removing the package alone may not be sufficient if the second-stage payload has already executed.

## How Atomic Arch Abuses Developer Trust

The Atomic Arch campaign leverages trusted AUR packages to install additional software during package installation. Sonatype Researcher Adam Reynolds analyzed the npm package atomic-lockfile, which is installed by the modified PKGBUILDs.

The modified packages add a post-install script that invokes npm and installs atomic-lockfile during package installation. Analysis of the atomic-lockfile package identified a bundled native Linux executable that is executed during installation via a preinstall script defined in the package's package.json file. Analysis identified references to an eBPF program (scales.bpf.c) and to libbpf APIs including:

-

bpf_object__load

-

bpf_program__attach

-

bpf_map__pin

eBPF (extended Berkeley Packet Filter) is a Linux technology that allows programs to run inside the kernel with elevated privileges. Static analysis identified functionality associated with process, file, and network hiding. The eBPF-related functionality references hooks for getdents64(), the system call used to enumerate directory entries, and maintains structures named hidden_pids, hidden_names, and hidden_inodes.

The executable also contains functionality associated with Linux socket diagnostics interfaces, including NETLINK_SOCK_DIAG, and logic related to debugger detection through PTRACE_ATTACH and PTRACE_SEIZE, suggesting efforts to reduce visibility and hinder analysis.

The binary contains references to GitHub credentials, SSH artifacts, HashiCorp Vault tokens, browser cookie databases, Slack, Discord, Microsoft Teams, and Telegram data stores. Taken together, these references strongly indicate credential and token harvesting functionality.

The executable also includes archive support, multipart form-data handling, and HTTP upload functionality, including references to POST /upload, indicating potential data collection and exfiltration capabilities.

### The Second Wave

We have since identified additional malicious packages associated with a second wave of the campaign, involving js-digest and lockfile-js, which Sonatype is tracking as Sonatype-2026-003808. While the malicious binary differs from that observed in atomic-lockfile, static analysis revealed many of the same characteristics, including references to eBPF functionality, browser artifacts, credential stores, and collaboration platforms such as Slack and Discord. The similarities suggest the attackers are evolving their tooling while maintaining the same underlying objectives and tradecraft.

Analysis also uncovered what appears to be an operational mistake by the threat actors. The package lockfile-js contains the same preinstall hook observed in js-digest, `"preinstall": "./lib/install-deps.mjs"`, but the published package does not contain the referenced lib directory. Researchers also observed references to earlier campaign artifacts within package metadata. While the error may have reduced the effectiveness of that specific package, it provides additional evidence linking the second wave to the broader Atomic Arch operation.

## AUR Stewardship Process Offers Unique Attacker Opportunity

The Arch User Repository allows community members to maintain package definitions for software not included in the official Arch repositories. When a maintainer abandons a package, it becomes orphaned and another user can request ownership.

In the Atomic Arch campaign, attackers appear to be exploiting this process to gain stewardship of trusted packages already used by the community. Attackers adopt orphaned AUR packages. The package keeps its existing name, history, and user trust, but control of its build instructions changes hands.

From the user's perspective, they are simply installing or updating a familiar package from a trusted source.

## The Atomic Arch Payload Wasn't in the Package

The most important lesson from Atomic Arch is the trust model.

-

Traditional supply chain attacks attempt to convince developers to install something new.

-

Typosquatting attacks exploit mistakes.

-

Brandjacking attacks exploit familiarity.

-

Atomic Arch appears to exploit ownership.

Attackers are not building trust from scratch. They're acquiring projects that have already earned it. That dramatically reduces the warning signs developers normally rely on when evaluating software.

As software ecosystems continue to rely on volunteer maintainers and community stewardship, ownership transitions may become an increasingly attractive target for attackers looking to compromise trusted distribution channels from the inside.

Written by Sonatype Research Team

Sonatype's Research Team is focused on bringing real-time, in-depth intelligence and actionable information about open source and third party vulnerabilities to Sonatype customers.

[Explore All Posts by Sonatype Research Team](https://www.sonatype.com/blog/author/sonatype-security-research-team)

Tags

npm dependencies malicious code npm Malware Analysis Malware Sonatype Research

## Research Sonatype-2026-003775 and Sonatype-2026-003808

Get real-time intelligence on Sonatype-2026-003775 and Sonatype-2026-003808 and keep your software supply chain protected.

[View on Guide](https://guide.sonatype.com/vulnerability/sonatype-2026-003775/sonatype-research)

### Subscribe to Sonatype Insights

## Related Resources

Webinar

###  [PHP2Shell and eMudhra: What Your Teams Need to Know Now](https://www.sonatype.com/resources/webinars/php2shell-and-emudhra)

Watch On Demand

Blog Post

###  [eMudhra Certificate Incident: A Practical Audit to Inspect Developer Tools](https://www.sonatype.com/blog/a-protected-browser-doesnt-mean-your-enterprise-is-remediated)

Read More

Video

###  [PHP2Shell Explained: Everything You Need to Know](https://www.sonatype.com/resources/videos/php2shell-explained)

Watch Now
