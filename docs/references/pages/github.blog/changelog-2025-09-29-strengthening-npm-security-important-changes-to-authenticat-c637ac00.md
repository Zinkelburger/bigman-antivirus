---
url: "https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/"
title: "Strengthening npm security: Important changes to authentication and token management - GitHub Changelog"
site: "The GitHub Blog"
captured: "2026-11-02T18:32:39Z"
via: "live"
sha256: "36448afd818fd11e37f272efb91f02c6169e111e41df1986c9dddd78a890d2db"
wayback: "https://web.archive.org/web/20260617041942/https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Strengthening npm security: Important changes to authentication and token management - GitHub Changelog

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/>
> Snapshot: <https://web.archive.org/web/20260617041942/https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/>

[Back to changelog](https://github.blog/changelog/)

Retired

September 29, 2025 • 3 minute read

# Strengthening npm security: Important changes to authentication and token management

As part of our ongoing commitment to securing the npm ecosystem, we’re implementing the first phase of security improvements outlined in our [recent announcement](https://github.blog/security/supply-chain-security/our-plan-for-a-more-secure-npm-supply-chain/).

These changes will roll out over the coming five weeks completing by mid-November 2025 and require action from package maintainers.

We’re taking this phased approach to balance security improvements with giving you time to adapt your workflows. Your patience and partnership in securing our shared ecosystem is deeply appreciated.

## [What’s changing](https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/#whats-changing)

### [Granular npm access token lifetime limits](https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/#granular-npm-access-token-lifetime-limits)

Starting in mid-October, all newly created write-enabled granular access tokens for npm will have:

- A default expiration of seven days, reduced from 30 days.
- A maximum expiration of 90 days, which used to be unlimited.

#### [Why this matters](https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/#why-this-matters)

Long-lived tokens are a primary vector for supply chain attacks. When tokens are compromised, shorter lifetimes limit the window of exposure and reduce potential damage. This change brings npm in line with security best practices already adopted across the industry.

#### [What you need to do](https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/#what-you-need-to-do)

- Review your npm publishing CI/CD workflows that use granular tokens
- Plan for more frequent token rotation
- Consider migrating to [Trusted Publishers (OIDC)](https://docs.npmjs.com/trusted-publishers) for GitHub Actions workflows

Existing granular tokens will continue working with their current expiration dates, giving you time to transition.

### [Classic npm token sunset](https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/#classic-npm-token-sunset)

Over the next five weeks, we will:

- Revoke all existing legacy classic tokens for npm publishers.
- Disable legacy classic token generation on npmjs.com permanently.

#### [Why this matters](https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/#why-this-matters)

Classic tokens lack granular permissions and modern security controls. They grant broad access to your account, making them high-risk if they are compromised.

#### [What you need to do](https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/#what-you-need-to-do)

If you use classic tokens for npm, you must immediately take the following actions:

- Generate new npm granular access tokens with appropriate scoped permissions.
- Update all automation, CI/CD pipelines, and local configurations.
- Check [npmjs.com documentation](https://docs.npmjs.com) for updated guidance.

### [TOTP 2FA configuration changes](https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/#totp-2fa-configuration-changes)

New TOTP (Time-based One-Time Password) setups for npm access will be permanently disabled. Existing TOTP configurations will continue to work for now, but they will be phased out in the coming months.

#### [Why this matters](https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/#why-this-matters)

We’re strengthening our 2FA implementation to prevent potential bypass vulnerabilities. WebAuthn/passkeys provide stronger, phishing-resistant authentication.

#### [What you need to do](https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/#what-you-need-to-do)

- If you already use TOTP, no immediate action is needed, but you should plan to migrate.
- New 2FA setups should use [WebAuthn/passkeys](https://docs.npmjs.com/configuring-two-factor-authentication).
- Consider upgrading existing TOTP to WebAuthn for enhanced security.

## [Timeline and implementation](https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/#timeline-and-implementation)

These changes will roll out progressively:

- Now: Warning messages appear in npm CLI and website.
- Early October: New token lifetime limits and TOTP changes take effect for npm publishers.
- Mid-November: Classic npm tokens revoked and generation disabled.

We’re deliberately spacing out these changes to give you time to adapt while maintaining momentum on security improvements.

## [Looking ahead: Trusted publishers](https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/#looking-ahead-trusted-publishers)

While we’re tightening token security, we recognize that frequent rotation can be challenging for automation workflows. That’s why we strongly encourage adoption of [trusted publishing (OIDC)](https://docs.npmjs.com/trusted-publishers).

Trusted publishing eliminates long-lived tokens entirely by using temporary, job-specific credentials from your CI/CD provider. Currently supporting GitHub Actions and GitLab CI/CD, we’re actively working to expand support for Azure Pipelines, CircleCI, and other platforms.

The benefits of trusted publishing include:

- No tokens to rotate or leak
- Automatic provenance attestation
- Simplified security model
- Better audit trails

We know trusted publishing needs improvements to support all workflows, and we’re committed to expanding capabilities based on your feedback.

## [We need your partnership](https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/#we-need-your-partnership)

We understand these changes require effort from our community. Securing npm is a shared responsibility. These changes may cause temporary friction, but they’re essential for protecting our ecosystem from increasingly sophisticated attacks. The recent compromises of popular packages underscore the urgency of these improvements.

Things you can do to help:

- Start migrating away from classic npm tokens today
- Share feedback in our [Community Discussion thread](https://github.com/orgs/community/discussions/174507)
- Update your token rotation practices
- Help other maintainers in your community understand these changes

## [Getting support](https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/#getting-support)

We’re here to help you through this transition:

- 📚 [npm Documentation](https://docs.npmjs.com): Updated with the latest guidance
- 💬 [Community Discussion](https://github.com/orgs/community/discussions/174507): Share feedback and get help from the community
- 📧 [npm Support](https://www.npmjs.com/support): For additional assistance

## [Thank you](https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/#thank-you)

Your commitment to security makes npm safer for millions of developers worldwide. Together, we’re building a more resilient foundation for the JavaScript ecosystem.

Stay tuned for future updates as we continue rolling out the security improvements outlined in our [security roadmap](https://github.blog/security/supply-chain-security/our-plan-for-a-more-secure-npm-supply-chain/).

This is the first in a series of security updates. Join the conversation in our [Community Discussion](https://github.com/orgs/community/discussions/174507).

###   Oct.28 Release

[Prioritize Dependabot alerts with reachability analysis](https://github.blog/changelog/2026-10-28-prioritize-dependabot-alerts-with-reachability-analysis)

[supply chain security](https://github.blog/changelog/2026/?label=supply-chain-security)

###   Oct.20 Improvement

[Dependabot alerts REST API now includes dependency versions](https://github.blog/changelog/2026-10-20-dependabot-alerts-rest-api-now-includes-dependency-versions)

[supply chain security](https://github.blog/changelog/2026/?label=supply-chain-security)

###   Oct.20 Release

[GitHub package firewall in public preview](https://github.blog/changelog/2026-10-20-github-package-firewall-in-public-preview)

[supply chain security](https://github.blog/changelog/2026/?label=supply-chain-security)

###   Oct.20 Improvement

[Dependabot now supports npm 12, pnpm 11 and 12, and Bun 1.4](https://github.blog/changelog/2026-10-20-dependabot-now-supports-npm-12-pnpm-11-and-12-and-bun-1-4)

[supply chain security](https://github.blog/changelog/2026/?label=supply-chain-security)

###   Oct.16 Release

[Open source license compliance generally available](https://github.blog/changelog/2026-10-16-open-source-license-compliance-generally-available)

[supply chain security](https://github.blog/changelog/2026/?label=supply-chain-security)

###   Oct.13 Improvement

[Improved detection for withdrawn Dependabot alerts](https://github.blog/changelog/2026-10-13-improved-detection-for-withdrawn-dependabot-alerts)

[supply chain security](https://github.blog/changelog/2026/?label=supply-chain-security)

###   Oct.12 Improvement

[Anonymous organization-level registries for Dependabot](https://github.blog/changelog/2026-10-12-anonymous-organization-level-registries-for-dependabot)

[supply chain security](https://github.blog/changelog/2026/?label=supply-chain-security)

###   Oct.02 Retired

[Unvalidated npm trusted publishing configurations now expire](https://github.blog/changelog/2026-10-02-unvalidated-npm-trusted-publishing-configurations-now-expire)

[supply chain security](https://github.blog/changelog/2026/?label=supply-chain-security)

###   Oct.02 Improvement

[npm staged publishing now supports creating new packages](https://github.blog/changelog/2026-10-02-npm-staged-publishing-now-supports-creating-new-packages)

[supply chain security](https://github.blog/changelog/2026/?label=supply-chain-security)
