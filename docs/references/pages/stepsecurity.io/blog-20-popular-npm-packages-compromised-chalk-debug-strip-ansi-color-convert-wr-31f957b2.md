---
url: "https://www.stepsecurity.io/blog/20-popular-npm-packages-compromised-chalk-debug-strip-ansi-color-convert-wrap-ansi"
title: "20+ Popular NPM Packages Compromised (Chalk, Debug, Strip-ANSI, Color-Convert, Wrap-ANSI...) - StepSecurity"
captured: "2026-11-02T19:00:40Z"
via: "live"
sha256: "4eafebc360cd7209216d09255d82c37081d2b77d6e950219d48ce57eee72a473"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# 20+ Popular NPM Packages Compromised (Chalk, Debug, Strip-ANSI, Color-Convert, Wrap-ANSI...) - StepSecurity

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.stepsecurity.io/blog/20-popular-npm-packages-compromised-chalk-debug-strip-ansi-color-convert-wrap-ansi>

[Back to Blog](https://www.stepsecurity.io/blog)

[Threat Intel](https://www.stepsecurity.io/blog?category=Threat Intel)

# 20+ Popular NPM Packages Compromised (Chalk, Debug, Strip-ANSI, Color-Convert, Wrap-ANSI...)

Massive NPM supply chain attack targets cryptocurrency users through compromised maintainer account - affecting packages downloaded billions of times weekly including debug, chalk, ansi-styles, color-convert, strip-ansi and 15+ other critical JavaScript packages. Malicious code injected to steal cryptocurrency wallets and redirect blockchain transactions.

Varun Sharma

[View LinkedIn](https://www.linkedin.com/in/varunsharma07)

September 8, 2025

[Share on X](https://www.stepsecurity.io/blog/20-popular-npm-packages-compromised-chalk-debug-strip-ansi-color-convert-wrap-ansi#)[Share on X](https://www.stepsecurity.io/blog/20-popular-npm-packages-compromised-chalk-debug-strip-ansi-color-convert-wrap-ansi#)[Share on LinkedIn](https://www.stepsecurity.io/blog/20-popular-npm-packages-compromised-chalk-debug-strip-ansi-color-convert-wrap-ansi#)[Share on Facebook](https://www.stepsecurity.io/blog/20-popular-npm-packages-compromised-chalk-debug-strip-ansi-color-convert-wrap-ansi#)[Follow our RSS feed](https://www.stepsecurity.io/blog/rss.xml)

Table of Contents

[Loading nav...](https://www.stepsecurity.io/blog/20-popular-npm-packages-compromised-chalk-debug-strip-ansi-color-convert-wrap-ansi#)

-

- https://www.stepsecurity.io/blog/20-popular-npm-packages-compromised-chalk-debug-strip-ansi-color-convert-wrap-ansi#

## Executive Summary

On September 8, 2025, a critical supply chain attack affected over 20 widely-used NPM packages after maintainer Josh Junon (Qix-) fell victim to a sophisticated phishing attack. Malicious versions of packages including chalk, debug, strip-ansi, ansi-regex, color-convert, wrap-ansi, and many others - collectively downloaded billions of times weekly - were published with cryptocurrency-stealing malware.

This incident highlights the ongoing vulnerability of the open source ecosystem to social engineering attacks targeting maintainers.

## Attack Vector and Methodology

The attack began with a highly sophisticated phishing email that appeared to be a legitimate 2FA reset notification from NPM support.

The email, sent from support@npmjs.help, successfully bypassed the maintainer's initial scrutiny. As Josh Junon [stated in his disclosure](https://github.com/debug-js/debug/issues/1005#issuecomment-3266868187): "It was a 2FA reset email that looked shockingly authentic. I should have paid better attention, but it slipped past me."

‍

‍

This attack demonstrates a targeted approach, focusing specifically on packages with high download counts while leaving lesser-used packages on the same account untouched. This selective targeting suggests the attackers had specific objectives and understanding of the NPM ecosystem's most impactful packages.

## Technical Analysis of the Malicious Code

Based on Aikido's analysis, the injected malicious code, found in the compromised packages, exhibits sophisticated obfuscation techniques and data exfiltration capabilities. The malware includes:

### Key Malicious Behaviors:

- Ethereum Wallet Detection: Checks for the presence of crypto wallets through window.ethereum
- Cryptocurrency Address Replacement: Implements a complex system to detect and replace various cryptocurrency addresses including:

- Bitcoin (Legacy and SegWit)
- Ethereum
- Tron
- Litecoin
- Bitcoin Cash
- Solana

- Network Interception: Hijacks fetch and XMLHttpRequest to modify cryptocurrency transactions in real-time
- Transaction Manipulation: Specifically targets DeFi platforms including Uniswap, PancakeSwap, SushiSwap, and 1inch

The malware uses extensive obfuscation with over 40 hardcoded cryptocurrency addresses across different blockchains, suggesting a well-organized operation designed to redirect cryptocurrency transactions to attacker-controlled wallets.

## Complete List of Affected Packages

The following packages were confirmed compromised with malicious versions:

- ansi-styles @ 6.2.2
- strip-ansi @ 7.1.1
- ansi-regex @ 6.2.1
- debug @ 4.4.2
- color-convert @ 3.1.1
- color-name @ 2.0.1
- supports-color @ 10.2.1
- chalk @ 5.6.1
- wrap-ansi @ 9.0.1
- slice-ansi @ 7.1.1
- color @ 5.0.1
- color-string @ 2.1.1
- is-arrayish @ 0.3.3
- simple-swizzle @ 0.2.3
- supports-hyperlinks @ 4.1.1
- has-ansi @ 6.0.1
- chalk-template @ 1.1.1
- backslash @ 0.2.1

## Recovery Steps

### Identify Exposure

You can use GitHub search queries to look for compromised versions in your environment. Here is a [sample GitHub search query](https://github.com/search?q=%22%5C%22%40+debug%5C%22%3A+%5C%224.4.2%5C%22%22).

To evaluate whether a specific project is vulnerable, use the following commands.

```
# Check if your project uses any affected packages
npm list chalk debug ansi-styles strip-ansi ansi-regex supports-color wrap-ansi color-convert slice-ansi color-name color color-string is-arrayish simple-swizzle

# Check your package-lock.json for specific versions
grep -E "(chalk|debug|ansi-styles|strip-ansi|ansi-regex|color-convert|wrap-ansi|color-name).*version" package-lock.json
```

### Remove Compromised Packages

```
# Clear your npm cache
npm cache clean --force

# Remove node_modules and package-lock.json
rm -rf node_modules package-lock.json

# For yarn users
rm -rf node_modules yarn.lock
```

### Rotate secrets and Monitor Crypto Wallets

If a compromised package executed in your environment, take the following steps:

#### Rotate All Secrets

Rotate all secrets that were exposed such as:

- API keys
- Database credentials
- Authentication tokens
- Cryptocurrency wallet keys

#### Monitor Cryptocurrency Wallets

If your application handles cryptocurrency transactions, monitor all wallets for unauthorized activity.

### For StepSecurity Enterprise Customers

The following steps are applicable only for StepSecurity enterprise customers. If you are not an existing enterprise customer, you can start our 14 day free trial by installing [the StepSecurity GitHub App](https://docs.stepsecurity.io/artifact-monitor) to complete the following recovery step.

#### ‍
Use NPM Package Cooldown Check

[The NPM Cooldown check](https://www.stepsecurity.io/blog/introducing-the-npm-package-cooldown-check) automatically fails a pull request if it introduces an npm package version that was released within the organization’s configured cooldown period (default: 2 days). Once the cooldown period has passed, the check will clear automatically with no action required.

The rationale is simple - most supply chain attacks are detected within the first 24 hours of a malicious package release, and the projects that get compromised are often the ones that rushed to adopt the version immediately. By introducing a short waiting period before allowing new dependencies, teams can reduce their exposure to fresh attacks while still keeping their dependencies up to date.

Here is an example showing how this check protected a project from using the compromised versions of packages involved in this incident:

[GitHub Check](https://github.com/step-security/test-reporting/pull/16/checks?check_run_id=49850926488)

#### Discover Pull Requests upgrading to compromised npm packages

We have added a new control specifically to detect pull requests that upgraded to these compromised packages. You can find the new control on the StepSecurity dashboard.

‍

#### Use StepSecurity Harden-Runner to detect compromised dependencies in CI/CD

StepSecurity Harden-Runner adds runtime security monitoring to your GitHub Actions workflows, providing visibility into network calls, file system changes, and process executions during CI/CD runs. Harden-Runner detects the compromised nx packages when they are used in CI/CD. Here is a [sample Harden-Runner insights page](https://app.stepsecurity.io/github/actions-security-demo/compromised-packages/actions/runs/17259145119) demonstrating this detection:

If you're already using Harden-Runner, we strongly recommend you review recent anomaly detections in your Harden-Runner dashboard. You can get started with Harden-Runner by following this [guide](https://docs.stepsecurity.io/harden-runner)

#### Use StepSecurity Artifact Monitor to detect software releases outside of authorized pipelines

StepSecurity Artifact Monitor provides real-time detection of unauthorized package releases by continuously monitoring your artifacts across package registries.

This tool would have flagged this incident by detecting that the compromised versions were published outside of the project's authorized CI/CD pipeline. The monitor tracks release patterns, verifies provenance, and alerts teams when packages are published through unusual channels or from unexpected locations.

By implementing Artifact Monitor, organizations can catch supply chain compromises within minutes rather than hours or days, significantly reducing the window of exposure to malicious packages.

Learn more about implementing Artifact Monitor in your security workflow in our [documentation](https://docs.stepsecurity.io/artifact-monitor)

## Industry Response and Mitigation

The open source community responded rapidly to this incident. Sindre Sorhus, maintainer of the Chalk organization, quickly published clean versions and removed the compromised maintainer's access. NPM support initiated incident response procedures, though the maintainer reported initial delays in account recovery.

Credit goes to the security team at [Aikido](https://www.aikido.dev/blog/npm-debug-and-chalk-packages-compromised) for being the first to detect and publicly report this compromise, enabling rapid community response.

We also want to acknowledge [Josh Junon (Qix-)](https://bsky.app/profile/bad-at-computer.bsky.social), the affected maintainer, for his immediate transparency and responsible disclosure upon discovering the compromise. Josh promptly alerted the community, provided a comprehensive list of affected packages, and worked diligently to remediate the issue.

## Lessons Learned and Future Prevention

This incident underscores several critical security considerations:

- Social Engineering Remains Effective: Even experienced developers can fall victim to sophisticated phishing attacks

- 2FA Is Not Foolproof: Account takeover can occur despite 2FA when reset mechanisms are compromised‍

- Automated Defenses Are Essential: Tools like dependency scanning and package cooldown periods provide critical protection

## Conclusion

The compromise of over 20 popular NPM packages through maintainer account takeover represents one of the most significant supply chain security incidents of 2025, affecting the broader JavaScript ecosystem. While the immediate threat has been contained, this attack serves as a critical reminder of the importance of comprehensive supply chain security measures.

Organizations must implement multiple layers of defense, including automated security checks, dependency management policies, and incident response procedures. The rapid detection and response to this incident demonstrates the importance of community vigilance and automated security tooling in protecting the modern software supply chain.
