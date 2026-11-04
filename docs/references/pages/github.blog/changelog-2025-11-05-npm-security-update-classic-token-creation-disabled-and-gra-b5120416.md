---
url: "https://github.blog/changelog/2025-11-05-npm-security-update-classic-token-creation-disabled-and-granular-token-changes/"
title: "npm security update: Classic token creation disabled and granular token changes - GitHub Changelog"
site: "The GitHub Blog"
captured: "2026-11-02T18:33:36Z"
via: "live"
sha256: "0c988816f73fc11366184f60cfe6da7011a2681222ed6daa83711039e92f4e29"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# npm security update: Classic token creation disabled and granular token changes - GitHub Changelog

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.blog/changelog/2025-11-05-npm-security-update-classic-token-creation-disabled-and-granular-token-changes/>

[Back to changelog](https://github.blog/changelog/)

Retired

November 5, 2025 • 2 minute read

# npm security update: Classic token creation disabled and granular token changes

Editor’s note (November 5, 2025): We’ve updated this post to explicitly clarify that the affected tokens are npm tokens.

Today marks another milestone in our ongoing effort to strengthen npm’s security. As [previously announced](https://github.com/orgs/community/discussions/178140), we’re implementing the first set of changes to npm’s token management system.

Important: These changes only affect npm tokens used for the npm registry. GitHub tokens (e.g, personal access tokens, fine-grained tokens, etc.) are not affected by these changes.

## [What’s changing today (November 5, 2025)](https://github.blog/changelog/2025-11-05-npm-security-update-classic-token-creation-disabled-and-granular-token-changes/#whats-changing-today-november-5-2025)

### [npm classic tokens](https://github.blog/changelog/2025-11-05-npm-security-update-classic-token-creation-disabled-and-granular-token-changes/#npm-classic-tokens)

- New npm classic tokens can no longer be created through the npmjs.com website, CLI, or API.
- Existing npm classic tokens continue to work until November 19, 2025.
- The npm token create CLI command no longer generates npm classic tokens.

### [npm granular access tokens](https://github.blog/changelog/2025-11-05-npm-security-update-classic-token-creation-disabled-and-granular-token-changes/#npm-granular-access-tokens)

- New npm tokens with write permissions now enforce 2FA by default.
- A new Bypass 2FA option is available for CI/CD workflows (unchecked by default).
- All npm existing granular tokens with write permissions are now capped at 90-day maximum lifetime.

- Existing tokens set to expire after February 3, 2026 have been adjusted to expire on that date.

## [What you need to do](https://github.blog/changelog/2025-11-05-npm-security-update-classic-token-creation-disabled-and-granular-token-changes/#what-you-need-to-do)

### [If you use npm classic tokens](https://github.blog/changelog/2025-11-05-npm-security-update-classic-token-creation-disabled-and-granular-token-changes/#if-you-use-npm-classic-tokens)

You have until November 19, 2025 to migrate to npm granular access tokens. After this date, all npm classic tokens will be permanently revoked.

#### [Migration steps:](https://github.blog/changelog/2025-11-05-npm-security-update-classic-token-creation-disabled-and-granular-token-changes/#migration-steps)

- Visit [npmjs.com/settings/~/tokens](https://www.npmjs.com/settings/~/tokens).
- Click Generate New Token → Granular Access Token.
- Configure appropriate permissions for your use case.
- For CI/CD workflows requiring write access, consider enabling Bypass 2FA or use established trust publishing with OIDC.
- Replace npm classic tokens in your workflows with the new npm granular tokens.

Note: npm granular tokens must currently be created through the npmjs.com website. We’re actively working on adding full CLI support for granular token management via npm token commands in the coming weeks.

### [If you use npm granular tokens](https://github.blog/changelog/2025-11-05-npm-security-update-classic-token-creation-disabled-and-granular-token-changes/#if-you-use-npm-granular-tokens)

- Check your npm tokens’ expiration dates at [npmjs.com/settings/~/tokens](https://www.npmjs.com/settings/~/tokens).
- Plan for token rotation if any of them expire on February 3, 2026.
- New tokens created for CI/CD may need Bypass 2FA enabled if noninteractive publishing is required.

## [Not affected by these changes](https://github.blog/changelog/2025-11-05-npm-security-update-classic-token-creation-disabled-and-granular-token-changes/#not-affected-by-these-changes)

- GitHub personal access tokens (classic)
- GitHub fine-grained personal access tokens
- GitHub Actions secrets – No impact, though npm tokens stored as secrets should be updated
- GITHUB_TOKEN in actions

## [Looking ahead: November 19, 2025](https://github.blog/changelog/2025-11-05-npm-security-update-classic-token-creation-disabled-and-granular-token-changes/#looking-ahead-november-19-2025)

On November 19, we will permanently revoke all npm classic tokens and replace long-lived local publishing tokens with 2-hour session tokens.

## [Need help?](https://github.blog/changelog/2025-11-05-npm-security-update-classic-token-creation-disabled-and-granular-token-changes/#need-help)

- 📚 [npm token migration guide](https://docs.npmjs.com/creating-and-viewing-access-tokens)
- 💬 [Community discussion](https://github.com/orgs/community/discussions/178140)
- 🎫 [npm support](https://www.npmjs.com/support)

We understand these changes require effort from you. Thank you for partnering with us to make npm more secure for the entire JavaScript ecosystem.

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
