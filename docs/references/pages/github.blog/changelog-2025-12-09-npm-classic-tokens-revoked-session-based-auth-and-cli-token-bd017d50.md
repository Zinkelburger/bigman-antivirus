---
url: "https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/"
title: "npm classic tokens revoked, session-based auth and CLI token management now available - GitHub Changelog"
site: "The GitHub Blog"
captured: "2026-11-02T18:33:39Z"
via: "live"
sha256: "d9725d62ab01f5fdcdf35da6c2cd44a7c069f23b9e1fa0c29d956445b00672b1"
wayback: "https://web.archive.org/web/20261023011538/https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
  - "docs/threats/06-credential-and-secret-protection.md"
---

# npm classic tokens revoked, session-based auth and CLI token management now available - GitHub Changelog

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/>
> Snapshot: <https://web.archive.org/web/20261023011538/https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/>

[Back to changelog](https://github.blog/changelog/)

Retired

December 9, 2025 • 2 minute read

# npm classic tokens revoked, session-based auth and CLI token management now available

As [previously announced](https://github.blog/changelog/2025-11-05-npm-security-update-classic-token-creation-disabled-and-granular-token-changes/), we’re completing the npm classic token deprecation today. This marks a major milestone in the [security hardening initiative](https://github.com/orgs/community/discussions/178140) to strengthen npm’s authentication system.

## [What’s changing today (December 9, 2025)](https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/#whats-changing-today-december-9-2025)

### [npm classic tokens permanently revoked](https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/#npm-classic-tokens-permanently-revoked)

We’ve permanently revoked all existing npm classic tokens. They can no longer authenticate, be recreated, or be recovered.

### [New session-based authentication](https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/#new-session-based-authentication)

Starting today, when you use npm login you’ll receive a two-hour session token instead of long-lived tokens. These sessions automatically expire after two hours, requiring reauthentication to continue publishing. Unlike traditional tokens, session tokens won’t appear in your UI or CLI token lists—they work behind the scenes to maintain your authenticated session. During these sessions, 2FA is enforced for publishing operations, adding an extra layer of security.

### [New CLI token management tool for granular tokens](https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/#new-cli-token-management-tool-for-granular-tokens)

npm is introducing a new command-line tool specifically designed for managing granular access tokens. This new CLI tool provides the ability to create, list, and revoke granular tokens directly from your terminal, eliminating the need to visit the npm website for token management.

This provides the same convenient command-line experience previously available only for classic tokens.

Full documentation for the new token management commands will be available in the [npm CLI documentation](https://docs.npmjs.com/cli/).

### [Secure-by-default 2FA for new packages (new)](https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/#secure-by-default-2fa-for-new-packages-new)

Starting this week, 2FA enforcement is shifting to become the default option for all new package publications. When you create a new package, 2FA will be enabled by default in the package settings. This ensures packages are protected from the moment they’re created without requiring manual security configuration. Existing packages retain their current 2FA settings.

### [Legacy API endpoint support temporarily restored](https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/#legacy-api-endpoint-support-temporarily-restored)

We’ve temporarily restored support for the legacy API endpoint (/user/org.couchdb.user:) that was causing compatibility issues in November. This endpoint now generates the same two-hour session tokens used by npm login, requiring re-authentication every two hours when publishing.

This is a temporary measure to provide a transition period. The removal of this legacy endpoint is imminent and planned for the coming months. We strongly recommend migrating to modern authentication methods now to avoid disruption.

Note: This endpoint primarily affects users of Yarn v1 and v2. If you’re using these versions, we recommend updating to modern package managers or authentication methods that support current npm security standards.

## [What you need to do](https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/#what-you-need-to-do)

### [If you were still using npm classic tokens](https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/#if-you-were-still-using-npm-classic-tokens)

Your npm classic tokens stopped working today. Here’s how to restore access:

For local development, run npm login to create a new two-hour session. You’ll need to reauthenticate periodically, but this ensures better security.

For CI/CD workflows, create granular access tokens using the new CLI commands (npm token create) or at [npmjs.com/settings/~/tokens](https://www.npmjs.com/settings/~/tokens). When setting up these tokens, you may enable Bypass 2FA for noninteractive automated workflows and set appropriate expiration (write tokens limited to 90 days maximum). For the most secure deployment, consider adopting [OIDC trusted publishing](https://docs.npmjs.com/trusted-publishers), which eliminates the need to generate and manage tokens for publishing packages.

## [What’s next?](https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/#whats-next)

Thank you for your patience as we strengthen npm’s security. These changes protect the entire JavaScript ecosystem while maintaining the workflows you depend on. For more details on our security roadmap and ongoing improvements, see our [community discussion](https://github.com/orgs/community/discussions/179562).

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
