---
url: "https://www.wiz.io/blog/supply-chain-risk-in-vscode-extension-marketplaces"
title: "Supply Chain Risk in VSCode Extension Marketplaces | Wiz Blog"
site: "wiz.io"
published: "2025-10-15T08:00:01-04:00"
captured: "2026-11-02T19:04:39Z"
via: "live"
sha256: "7a1ce775b56cbc9022b411684fcac8682b370773c1e5e5ff82131005dce21b68"
wayback: "https://web.archive.org/web/20260922182945/https://www.wiz.io/blog/supply-chain-risk-in-vscode-extension-marketplaces"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Supply Chain Risk in VSCode Extension Marketplaces | Wiz Blog

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.wiz.io/blog/supply-chain-risk-in-vscode-extension-marketplaces>
> Snapshot: <https://web.archive.org/web/20260922182945/https://www.wiz.io/blog/supply-chain-risk-in-vscode-extension-marketplaces>

Wiz Research identified a pattern of secret leakage by publishers of VSCode IDE Extensions. This occurred across both the VSCode and Open VSX marketplaces, the latter of which is used by AI-powered VSCode forks like Cursor and Windsurf. Critically, in over a hundred cases this included leakage of access tokens granting the ability to update the extension itself. By default, VS Code will auto-update extensions as new versions become available. A leaked VSCode Marketplace or OpenVSX PAT allows an attacker to directly distribute a malicious extension update across the entire install base. An attacker who discovered this issue would have been able to directly distribute malware to the cumulative 150,000 install base.

Each leaked secret is a result of publisher error. However, after reporting this issue via Microsoft's Security Response Center (MSRC), Wiz has been collaborating with Microsoft on platform level improvements to provide guardrails against future secrets leakage in the VSCode Marketplace. Together, we've also launched a notification campaign to alert impacted publishers and help them address these vulnerabilities.

#

Discovering a massive secrets leak

In February, attackers started [attempting to introduce malware](https://github.com/microsoft/vsmarketplace/blob/main/RemovedPackages.md) to the VSCode Marketplace. Our initial goal was to identify additional malicious extensions, investigate them, and report them to the Marketplace for removal. While we did end up identifying several interesting malicious extensions, we stumbled on something much more impactful: a scourge of secrets leaking in extension packages.

VSCode extensions are distributed as .vsix files, which can be unzipped and inspected. However, we found that publishers often failed to consider that everything in the package was publicly available, or failed to successfully sanitize their extensions of hardcoded secrets.

In total, we found over 550 validated secrets, distributed across more than 500 extensions from hundreds of distinct publishers. Across the 67 distinct types of secrets we found, there were a few notable categories:

-

AI provider secrets (OpenAI, Gemini, Anthropic, XAI, DeepSeek, HuggingFace, Perplexity)

-

High risk profession platform secrets (AWS, Github, Stripe, Auth0, GCP)

-

Database secrets (MongoDB, Postgres, Supabase)

#

From themes to threats

The most interesting and globally impactful secrets are the access tokens that grant the ability to update the extension. For the VSCode Marketplace, these are [Azure DevOps Personal Access Tokens](https://code.visualstudio.com/api/working-with-extensions/publishing-extension). The Open VSX Marketplace uses [open-vsx.org Access Tokens](https://open-vsx.org/user-settings/tokens).

Over one hundred valid leaked VSCode Marketplace PATs were identified within VSCode Marketplace extensions. Together, they represent an install base of over 85,000 extension installs.

Over thirty leaked OVSX Access Tokens were identified, within either VSCode Marketplace or OVSX extensions. Together, they represent an install base of over 100,000 extension installs.

Much of this massive vulnerable install base is actually contributed by themes. This is interesting, because themes are generally viewed as safer than other extensions, given they don’t carry any code. However, they still increase your attack surface, as there is no technical control preventing themes from bundling malware.

An additional interesting lens on these leaked tokens involves the public distribution of company internal or vendor specific extensions. If you investigate the marketplace, you’ll notice extensions that have a low install count, but are specifically designed to support a single company’s engineers or customers. Internal extensions should not be distributed publicly, but often are for convenience. In one case, we found a VSCode Marketplace PAT that would allow us to push targeted malware to the workforce of a $30 billion market cap Chinese megacorp. Vendor specific extensions are common, and allow for interesting targeting opportunities if compromised. For example, one at risk extension belonged to a Russian construction technology company.

#

Now how did that get there?

Whenever we discover [a new dataset of leaked secrets](https://www.wiz.io/blog/leaking-ai-secrets-in-public-code), we attempt to identify patterns that might indicate the root cause(s) and potential mitigations. In this case, the largest contributor to secrets leakage was the bundling of hidden files, also known as dotfiles. The quantity of .env files was especially prominent, although hardcoded credentials in extension source code were also prevalent.

Over the course of the year, we saw an increase in secrets leaking via AI related configuration files, including config.json, mcp.json and .cursorrules. Other common sources included build configuration (e.g package.json) and documentation (e.g README.md).

#

Hardening and Remediation

Discovering this critical issue was one thing, getting it fixed is another. We’ve spent the past six months working with Microsoft to help resolve this issue centrally, ensuring we can patch this gap and disclose responsibly.

The response to this issue took multiple forms.

-

Notification: Wiz made targeted notifications of the highest risk disclosed secrets throughout this process. Microsoft has further made several rounds of notification to impacted extension publishers reported by Wiz and asked them to take action. Every leaked Visual Studio Marketplace PAT was revoked. For other secrets, Microsoft communicated with publishers regarding their exposure and provided appropriate guidance.

-

Prevention:

-

Microsoft integrated secrets scanning capabilities prior to publishing and now blocks extensions with verified secrets, notifying extension owners when secrets are detected. See their announcement: [Upcoming Security Enhancement: Secret Detection for Extensions](https://github.com/microsoft/vsmarketplace/discussions/1383), and follow up [Secret Prevention for Extensions: Now in Blocking Mode](https://github.com/microsoft/vsmarketplace/discussions/1442).

-

OpenVSX is adding a prefix ([ovsxp_](https://github.com/EclipseFdn/open-vsx.org/pull/5261/files)) to their tokens. Microsoft supports OpenVSX tokens within their secret scanning of the VSCode Marketplace.

-

Mitigation: Having prevented further introduction of secrets, Microsoft scanned all existing extensions, for embedded secrets, and will be working with extension owners to ensure they are remediated by publishing a new, sanitized version of the affected extension.

In June, Microsoft shared their progress and roadmap for VSCode Marketplace security in [Security and Trust in Visual Studio Marketplace](https://developer.microsoft.com/blog/security-and-trust-in-visual-studio-marketplace).

On the publisher side, VSCode extension publishers should scan for secrets prior to publishing.

###

Guidance for users and administrators

For VSCode users:

-

Limit the number of installed extensions. Each extension introduces extended threat surface, which should be measured against the benefit of their usage.

-

Review extension trust criteria. Consider installation prevalence, reviews, extension history, and publisher reputation, among other metadata, prior to adoption.

-

Consider auto-update tradeoffs. Auto-updating extensions ensures you consume security updates, but introduces the risk of a compromised extension pushing malware to your machine.

For corporate security teams:

-

Develop an IDE extension inventory, in order to respond to reports of malicious extensions.

-

[Consider a centralized allowlist](https://code.visualstudio.com/docs/setup/enterprise#_configure-allowed-extensions) for VSCode extensions.

-

Consider sourcing extensions from the VSCode Marketplace, which has higher review rigor and controls currently, over the OpenVSX Marketplace.

#

Guidance for Platforms on Hardening Secrets

Throughout this process, we observed the diversity in secrets formatting practice, and the downstream impact that can have on security. We want to take this opportunity to highlight the following security practices that platforms can implement in their secrets:

-

Expiration: defaulting to a reasonable secret lifetime decreases the exploitation window for leaked secrets. In this research, for example, we observed a significant volume of VSCode PATs leaked in 2023 that had expired automatically. In several cases, Open VSX PATs were leaked in the same location, and still valid. This demonstrates the benefit of expiration.

-

Identifiable structure: [GitHub](https://github.blog/engineering/platform-security/behind-githubs-new-authentication-token-formats/) and [Microsoft](https://devblogs.microsoft.com/engineering-at-microsoft/common-annotated-security-keys/) have long been advocates of structuring secrets for easier identification and protection. Identifiable prefixes, checksums, or the full [Common Annotated Security Key (CASK) standard](https://github.com/microsoft/cask) all offer an advantage to defenders. Our results will over-represent well-structured secrets, but remaining risks post-disclosure will predominantly be secrets that lack easily detectable structure.

-

GitHub Advanced Secret Scanning: Platforms should strongly consider enrolling in the [Secret Scanning Partner Program](https://docs.github.com/en/code-security/secret-scanning/secret-scanning-partnership-program/secret-scanning-partner-program). As shown in our [past research](https://www.wiz.io/blog/leaking-ai-secrets-in-public-code), GitHub can be home to a large volume of secrets. In this project, we saw that a number of secrets leaked in VSCode extensions were also leaked on GitHub. For secrets supported by Advanced Secret Scanning, that meant publishers had already been notified of the risk automatically.

#

Takeaways & Timeline

We are relieved to have found, responsibly disclosed, and helped comprehensively resolve this risk.

The issue highlights the continued risks of extensions and plugins, and supply chain security in general. It continues to validate the impression that any package repository carries a high risk of mass secrets leakage. It also reflects our findings that [AI secrets are a large part of the modern secrets leakage landscape](https://www.wiz.io/blog/leaking-ai-secrets-in-public-code), and indicates the role [vibe coding](https://www.wiz.io/blog/safer-vibe-coding-rules-files) might play in that problem.

Finally, our work with Microsoft highlights the role that responsible platforms can play in protecting the ecosystem. We are grateful to Microsoft for the partnership and working to protect customers together. Without their willingness to lean in here, it would have been impossible to scale disclosure and remediation.

For more documentation on VSCode Extension security, please visit:

-

[Extension runtime security](https://code.visualstudio.com/docs/configure/extensions/extension-runtime-security)

-

[Publishing Extensions](https://code.visualstudio.com/api/working-with-extensions/publishing-extension)

-

[Walkthrough: Publish a Visual Studio extension](https://learn.microsoft.com/en-us/visualstudio/extensibility/walkthrough-publishing-a-visual-studio-extension)

##

Timeline

March 30th, 2025: Wiz Research reports this issue to MSRC.

April 4th, 2025: Wiz reports initial batch of 250 leaked secrets.

April 25th, 2025: MSRC completes notification of impacted third-parties who had leaked reported secrets.

May 1st, 2025: MSRC marks the report Ineligible for Bounty, and closes the case as Complete.

May 2nd, 2025: Wiz notes potential negative impact of disclosure without additional controls in place, and requests information on platform level improvements.

May 13th, 2025: MSRC re-opens the case, and starts “working on a plan and a timeline for preventative measures”.

July 10th, 2025: MSRC shares plans for remediation, and requests a late-September disclosure timeline.

June 11th, 2025: Microsoft publishes [Security and Trust in Visual Studio Marketplace](https://developer.microsoft.com/blog/security-and-trust-in-visual-studio-marketplace)

Aug 12th, 2025: MSRC and Wiz Research meet, and expand on remediation plans. Wiz identifies and highlights VSCode Marketplace PAT detection gap in secrets scanning. VSCode Marketplace team announces [Secret Detection for Extensions](https://github.com/microsoft/vsmarketplace/discussions/1383).

Aug 27th, 2025: MSRC sets September 25th as the disclosure date.

Sep 18th, 2025: MSRC requests a delay in disclosure due to a performance issue in an implemented hardening measure.

Sep 23rd, 2025: MSRC suggests October 15, 2025 disclosure date.

[See more from Wiz Research](https://www.wiz.io/research)

Tags

[#Research](https://www.wiz.io/blog/tag/research)[#Threat Intel](https://www.wiz.io/blog/tag/threat-intel)

https://www.wiz.io/blog/honeybee-threat-research

[### Introducing HoneyBee: How We Automate Honeypot Deployment for Threat Research](https://www.wiz.io/blog/honeybee-threat-research)

https://www.wiz.io/authors/yaara-shriki

[Yaara Shriki](https://www.wiz.io/authors/yaara-shriki)

October 7, 2025

Turning attacker insights into stronger cloud security protections.

https://www.wiz.io/blog/wiz-research-redis-rce-cve-2025-49844

[### RediShell: Critical Remote Code Execution Vulnerability (CVE-2025-49844) in Redis, 10 CVSS score](https://www.wiz.io/blog/wiz-research-redis-rce-cve-2025-49844)

https://www.wiz.io/authors/benny-isaacshttps://www.wiz.io/authors/nir-brakha

[Benny Isaacs](https://www.wiz.io/authors/benny-isaacs), [Nir Brakha](https://www.wiz.io/authors/nir-brakha)

October 6, 2025

Wiz Research discovers vulnerability stemming from 13-year-old bug present in all Redis versions, used in 75% of cloud environments.

https://www.wiz.io/blog/database-ransomware-research

[### Defending against database ransomware attacks](https://www.wiz.io/blog/database-ransomware-research)

https://www.wiz.io/authors/danielle-aminovhttps://www.wiz.io/authors/shahar-dorfmanhttps://www.wiz.io/authors/hila-ramati

[Danielle Aminov](https://www.wiz.io/authors/danielle-aminov), [Shahar Dorfman](https://www.wiz.io/authors/shahar-dorfman), [Hila Ramati](https://www.wiz.io/authors/hila-ramati)

October 6, 2025

How attackers exploit exposed databases for extortion—and the defenses that work.
