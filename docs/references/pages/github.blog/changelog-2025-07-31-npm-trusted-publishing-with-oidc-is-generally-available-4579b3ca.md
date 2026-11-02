---
url: "https://github.blog/changelog/2025-07-31-npm-trusted-publishing-with-oidc-is-generally-available/"
title: "npm trusted publishing with OIDC is generally available - GitHub Changelog"
site: "The GitHub Blog"
captured: "2026-11-02T18:32:26Z"
via: "live"
sha256: "bef9ae9f319c1a5fa18c561ee5894648643455480c83a8f3d319ff0bdcf3afa4"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# npm trusted publishing with OIDC is generally available - GitHub Changelog

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.blog/changelog/2025-07-31-npm-trusted-publishing-with-oidc-is-generally-available/>

[Back to changelog](https://github.blog/changelog/)

Release

July 31, 2025 • 2 minute read

# npm trusted publishing with OIDC is generally available

As of today, npm trusted publishing with OpenID Connect (OIDC) is now generally available. This feature enables you to securely publish npm packages directly from CI/CD workflows using OpenID Connect (OIDC) for authentication, reducing the need to manage long-lived tokens.

## [What’s new](https://github.blog/changelog/2025-07-31-npm-trusted-publishing-with-oidc-is-generally-available/#whats-new)

With trusted publishing, you can now:

- Publish packages without npm tokens: Configure your packages to accept publishes from specific [GitHub Actions](https://github.com/features/actions) or [GitLab CI/CD](https://docs.gitlab.com/ci/pipelines/) workflows using [OIDC](https://openid.net/developers/how-connect-works/) authentication.
- Eliminate token security risks: No more storing, rotating, or accidentally exposing npm tokens in your CI/CD environments.
- Establish cryptographic trust: Each publish is authenticated using short-lived, workflow-specific credentials that cannot be exfiltrated or reused.
- Get automatic provenance: When using trusted publishing, npm CLI publishes provenance attestations by default. The --provenance flag is no longer needed.

Trusted publishing brings enterprise-grade security to your npm publishing workflow and is available today for:

- All npm private and public packages, both scoped and unscoped.
- GitHub Actions (GitHub-hosted runners).
- GitLab CI/CD (gitlab.com shared runners).

This feature requires npm CLI v11.5.1 or later. Note that [provenance remains unavailable when publishing from private source repositories](https://github.blog/changelog/2023-07-25-publishing-with-npm-provenance-from-private-source-repositories-is-no-longer-supported/).

## [How it works](https://github.blog/changelog/2025-07-31-npm-trusted-publishing-with-oidc-is-generally-available/#how-it-works)

Trusted publishing creates a [trust relationship](https://docs.github.com/actions/concepts/security/about-security-hardening-with-openid-connect) between npm and your CI/CD provider. When you configure a trusted publisher for your package, npm will only accept publishes from the specific workflow you’ve authorized.

### [Setting up trusted publishing](https://github.blog/changelog/2025-07-31-npm-trusted-publishing-with-oidc-is-generally-available/#setting-up-trusted-publishing)

- Configure on npmjs.com: Navigate to your package settings and add a trusted publisher by specifying:

- For GitHub Actions: organization/user, repository, workflow filename, and environment name.
- For GitLab CI/CD: namespace, project, the top-level CI file path, and environment name.

- Update your workflow: Add the required permissions / ID tokens to your workflow file. No need to generate npm write tokens!
- Publish securely: Your workflow can now publish directly to npm using OIDC authentication.

Make sure you follow the [detailed steps from our documentation](https://docs.npmjs.com/trusted-publishers).

## [Automatic provenance with trusted publishing](https://github.blog/changelog/2025-07-31-npm-trusted-publishing-with-oidc-is-generally-available/#automatic-provenance-with-trusted-publishing)

When you publish using trusted publishing, the npm CLI automatically generates and publishes provenance attestations for your package. This means you don’t need to add the --provenance flag to your publish command.

Every package published via trusted publishing includes cryptographic proof of its source and build environment. Your users can verify where and how your package was built, increasing trust in your supply chain.

If needed, you can explicitly [opt out of provenance](https://docs.npmjs.com/generating-provenance-statements#using-third-party-package-publishing-tools) by setting NPM_CONFIG_PROVENANCE=false or specific parameters in the package.json or .npmrc files.

## [Getting started](https://github.blog/changelog/2025-07-31-npm-trusted-publishing-with-oidc-is-generally-available/#getting-started)

Check out [our documentation](https://docs.npmjs.com/trusted-publishers) for detailed setup instructions and examples for both GitHub Actions and GitLab CI/CD.

For packages that were part of the private preview, your trusted publisher configurations remain active and will continue working.

## [What’s next](https://github.blog/changelog/2025-07-31-npm-trusted-publishing-with-oidc-is-generally-available/#whats-next)

We’re committed to expanding trusted publishing support. Stay tuned for updates on additional CI/CD providers and self-hosted runner support.

We’d like to thank all the participants in our private preview program whose feedback was invaluable in shaping this feature. Trusted publishing represents a significant step forward in npm’s security posture, and we’re excited to see how the community adopts this new capability.

Happy (and secure) publishing! 🚀

Join the discussion in [GitHub Community](https://github.com/orgs/community/discussions/categories/announcements).

###   Oct.28 Release

[GitHub Copilot Fusion now supports selection of local models](https://github.blog/changelog/2026-10-28-github-copilot-fusion-now-supports-selection-of-local-models)

[client apps](https://github.blog/changelog/2026/?label=client-apps)

###   Oct.28 Release

[Prioritize Dependabot alerts with reachability analysis](https://github.blog/changelog/2026-10-28-prioritize-dependabot-alerts-with-reachability-analysis)

[supply chain security](https://github.blog/changelog/2026/?label=supply-chain-security)

###   Oct.23 Improvement

[Turn notifications into code with GitHub Copilot in Teams](https://github.blog/changelog/2026-10-23-turn-notifications-into-code-with-github-copilot-in-teams)

[client apps](https://github.blog/changelog/2026/?label=client-apps)

###   Oct.21 Release

[GitHub CLI: Opt in to expiring, auto-refreshing OAuth tokens](https://github.blog/changelog/2026-10-21-github-cli-opt-in-to-expiring-auto-refreshing-oauth-tokens)

[application security](https://github.blog/changelog/2026/?label=application-security)

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

[Enterprise managed settings enforcement updates](https://github.blog/changelog/2026-10-13-enterprise-managed-settings-enforcement-updates)

[client apps](https://github.blog/changelog/2026/?label=client-apps)
