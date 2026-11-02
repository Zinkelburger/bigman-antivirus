---
url: "https://docs.npmjs.com/about-access-tokens"
title: "About access tokens | npm Docs"
captured: "2026-11-02T18:28:04Z"
via: "live"
source_url: "https://docs.npmjs.com/about-access-tokens/"
sha256: "8ec8dc691e7ed01d3b10ab43ffa23b9455a1283ec2485bb29536c74f45aec9eb"
wayback: "https://web.archive.org/web/20261014063914/https://docs.npmjs.com/about-access-tokens/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# About access tokens | npm Docs

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://docs.npmjs.com/about-access-tokens>
> Snapshot: <https://web.archive.org/web/20261014063914/https://docs.npmjs.com/about-access-tokens/>

[Skip to search](https://docs.npmjs.com/about-access-tokens/#search-box-input)[Skip to content](https://docs.npmjs.com/about-access-tokens/#skip-to-content)

[npm Docs](https://docs.npmjs.com/)

[npmjs.com](https://npmjs.com/)[Status](https://status.npmjs.com/)[Support](https://npmjs.com/support/)

# About access tokens

Table of contents

Note: You must be using npm version 5.5.1 or greater to use access tokens.

An access token is an alternative to using your username and password for authenticating to npm when using the API or the npm command-line interface (CLI). An access token is a hexadecimal string that you can use to authenticate, and which gives you the right to install and/or publish your modules.

As of November 2025, only [Granular access tokens](https://docs.npmjs.com/about-access-tokens/#about-granular-access-tokens) are supported. Legacy access tokens have been removed.

You can create access tokens to give other tools (such as continuous integration testing environments) access to your npm packages. For example, GitHub Actions provides the ability to store [secrets](https://docs.github.com/en/actions/configuring-and-managing-workflows/creating-and-storing-encrypted-secrets), such as access tokens, that you can then use to authenticate. When your workflow runs, it will be able to complete npm tasks as you, including installing private packages you can access.

You can work with tokens from the web or the CLI, whichever is easiest. What you do in each environment will be reflected in the other environment.

npm token commands let you:

- View tokens for easier tracking and management
- Limit access according to IP address ranges (CIDR)
- Delete/revoke tokens

For more information on creating and viewing access tokens on the web and CLI, see "[Creating and viewing access tokens](https://docs.npmjs.com/creating-and-viewing-access-tokens)".

## [About granular access tokens](https://docs.npmjs.com/about-access-tokens/#about-granular-access-tokens)

Granular access tokens allow you to restrict access provided to the token based on what you want to use the token for. With granular access tokens, you can:

- Restrict which packages and scopes a token has access to
- Grant tokens access to specific organizations
- Set a token expiration date
- Limit token access based on IP address ranges
- Select between Read-only or Read and write access. For packages and scopes, Read and write access can either publish and stage new versions or be stage only (see "[About stage-only tokens](https://docs.npmjs.com/about-access-tokens/#about-stage-only-tokens)").
- Configure a token to Bypass 2FA requirements

You can create up to 1000 granular access tokens on your npm account. You can set how long your token is valid for, at least one day in the future. Each token can access up to 50 organizations, and up to either 50 packages, 50 scopes, or a combination of 50 packages and scopes. Access tokens are tied to users’ permission; hence it cannot have more permission than the user at any point in time. If a user has their access revoked from a package or an org., their granular access token will also have its access revoked from those packages or org.

When you give a token access to an organization, the token can only be used for managing organization settings and teams or users associated with the organization. It does not give the token the right to publish packages managed by the organization.

The Bypass 2FA capability applies to tokens with write access and is set to false by default at token creation. When the Bypass 2FA option is set to true, this setting takes precedence over account-level and package-level 2FA settings for package and automation actions such as publishing. This means that even if account-level 2FA is enabled and/or package-level 2FA is required, 2FA will still be bypassed when using the token to publish. Do not set Bypass 2FA to true if a package or organization requires fully enforced 2FA.

## [About stage-only tokens](https://docs.npmjs.com/about-access-tokens/#about-stage-only-tokens)

When a granular access token has Read and write access to packages and scopes, you can choose how it is allowed to publish:

- Read and write (publish and stage): the token can publish new package versions directly to the registry.
- Read and write (stage only): the token cannot publish new package versions directly. Instead, it stages a version for a package maintainer to review and promote.

Stage-only tokens are a safer on-ramp for automation such as continuous integration and deployment (CI/CD). Because a stage-only token can't publish a new version directly — each staged version must be reviewed and promoted by a maintainer with two-factor authentication (2FA) — a leaked or misused token can't get a new version onto the registry without that maintainer approval. This narrows the risk of direct publishing; it does not otherwise restrict the token's write capabilities.

A stage-only token can:

- Stage a new package version for review, using npm stage publish
- Deprecate package versions
- Move dist-tags

A stage-only token cannot:

- Publish a new package version directly. Attempting to publish a new version fails with the error E_STAGE_REQUIRED.

Note: Stage-only restricts direct publishing of new versions only. It is not a general-purpose security boundary — a stage-only token retains the other write capabilities listed above, so treat it with the same care as any other token.

### [Publishing with a stage-only token](https://docs.npmjs.com/about-access-tokens/#publishing-with-a-stage-only-token)

When you publish a new version with a stage-only token, the registry does not publish it immediately. Instead:

- Stage the version. In your automation or CLI, run npm stage publish instead of npm publish. This uploads the version and creates a stage that is not yet live. If you run npm publish with a stage-only token, the publish is rejected with an E_STAGE_REQUIRED error telling you to run npm stage publish and have the version approved.
- Review the stage. A maintainer can list pending stages with npm stage list and inspect one with npm stage view <stage-id> or npm stage download <stage-id>.
- Promote the version. A maintainer with 2FA enabled promotes the staged version to the registry with npm stage approve <stage-id> --otp <code>, or discards it with npm stage reject <stage-id>.

### [Direct publishing is being deprecated](https://docs.npmjs.com/about-access-tokens/#direct-publishing-is-being-deprecated)

Bypass-2FA tokens with direct-publish access are being deprecated. The ability to publish new package versions directly with a granular access token will be removed in January 2027. To prepare, switch automation that publishes new versions to a Read and write (stage only) token together with the staging workflow described above. See also "[Account-identity actions require an interactive 2FA challenge](https://docs.npmjs.com/about-access-tokens/#account-identity-actions-require-an-interactive-2fa-challenge)" for related changes to bypass-2FA tokens.

## [Account-identity actions require an interactive 2FA challenge](https://docs.npmjs.com/about-access-tokens/#account-identity-actions-require-an-interactive-2fa-challenge)

Starting August 2026, tokens with Bypass 2FA enabled can no longer be used to perform account-identity or account-governance actions. These actions always require an interactive 2FA challenge:

- Change email or password
- Modify or disable 2FA configuration
- Create, escalate, or manage access tokens
- Add or remove package maintainers
- Organization and team governance actions

At the moment, bypass-2FA tokens can still be used for direct publishing. For CI/CD publishing, consider adopting [trusted publishing](https://docs.npmjs.com/trusted-publishers) instead.

[Edit this page on GitHub](https://github.com/npm/documentation/edit/main/content/integrations/integrating-npm-with-external-services/about-access-tokens.mdx)

8 contributorshttps://github.com/Tayvonhttps://github.com/shmamhttps://github.com/karenjlihttps://github.com/alex-juhttps://github.com/lukekarryshttps://github.com/feelepxyzhttps://github.com/monishcmhttps://github.com/ethomson

Last edited by [Tayvon](https://github.com/Tayvon) on [September 10, 2026](https://github.com/npm/documentation/commit/9cab4838fbfb2267c6363cf22823cffa9c26a3b9)

## Table of contents
