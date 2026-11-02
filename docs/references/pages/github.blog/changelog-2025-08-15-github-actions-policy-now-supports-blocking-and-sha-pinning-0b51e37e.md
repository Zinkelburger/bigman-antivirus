---
url: "https://github.blog/changelog/2025-08-15-github-actions-policy-now-supports-blocking-and-sha-pinning-actions/"
title: "GitHub Actions policy now supports blocking and SHA pinning actions - GitHub Changelog"
site: "The GitHub Blog"
captured: "2026-11-02T18:32:33Z"
via: "live"
sha256: "36b5861e02c3efe0b58fcdc8869670a51f0eb491529a10dd2a964b4ea56f823a"
wayback: "https://web.archive.org/web/20260830131130/https://github.blog/changelog/2025-08-15-github-actions-policy-now-supports-blocking-and-sha-pinning-actions/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# GitHub Actions policy now supports blocking and SHA pinning actions - GitHub Changelog

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.blog/changelog/2025-08-15-github-actions-policy-now-supports-blocking-and-sha-pinning-actions/>
> Snapshot: <https://web.archive.org/web/20260830131130/https://github.blog/changelog/2025-08-15-github-actions-policy-now-supports-blocking-and-sha-pinning-actions/>

[Back to changelog](https://github.blog/changelog/)

Improvement

August 15, 2025 • 2 minute read

# GitHub Actions policy now supports blocking and SHA pinning actions

GitHub Actions is powered by a diverse ecosystem of first-party and community contributed actions. If one of these actions has a vulnerability or is compromised by a malicious actor, it can impact all of its dependents in the supply chain.

Tools like [Dependabot](https://docs.github.com/code-security/dependabot/working-with-dependabot/keeping-your-actions-up-to-date-with-dependabot) can identify and upgrade actions versions with known vulnerabilities to help mitigate exposure. However, when an action is compromised to include malicious code, such as exfiltrating secrets or utilizing the permissions granted to the workflow to modify code or releases, users must act quickly and decisively to limit their exposure to these malicious changes. This release provides new features that allow developers and administrators to better respond and proactively limit the impact of compromised dependencies.

### [Block specific actions or versions](https://github.blog/changelog/2025-08-15-github-actions-policy-now-supports-blocking-and-sha-pinning-actions/#block-specific-actions-or-versions)

To provide better governance and respond to third-party workflows that are identified as malicious, the allowed actions and reusable workflows policy now supports explicit blocking. Prefix an entry with ! to block a specific action or version. The blocklist is evaluated last, overriding any other policy that would otherwise allow the action.

### [Enforce SHA pinning](https://github.blog/changelog/2025-08-15-github-actions-policy-now-supports-blocking-and-sha-pinning-actions/#enforce-sha-pinning)

To proactively limit the impact of a compromised dependency, GitHub recommends that workflows [pin dependency versions to a specific commit SHA](https://docs.github.com/actions/reference/security/secure-use#using-third-party-actions). This will prevent malicious code added to a new or updated branch or tag from being automatically used. Administrators can now enforce the use of SHA pinning through the allowed actions policy. A new checkbox appears under each radio selection, except when actions are disabled at the enterprise, organization, or repository level. The policy will check for a full commit SHA, and any workflow that attempts to use an action that isn’t pinned will fail.

### [Documentation](https://github.blog/changelog/2025-08-15-github-actions-policy-now-supports-blocking-and-sha-pinning-actions/#documentation)

To enforce these new policies, refer to our updated documentation on [allowed actions and reusable workflows](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#managing-github-actions-permissions-for-your-repository) and [SHA pinning enforcement](https://docs.github.com/actions/reference/security/secure-use#using-third-party-actions). The [GitHub Well-Architected Framework](https://wellarchitected.github.com/library/application-security/recommendations/actions-security) has been updated to reflect these policies and recommended practices for GitHub Actions security.

### [Roadmap](https://github.blog/changelog/2025-08-15-github-actions-policy-now-supports-blocking-and-sha-pinning-actions/#roadmap)

Hardening the open source and GitHub Actions ecosystem is an ongoing focus and GitHub is introducing [immutable releases](https://github.com/github/roadmap/issues/1137) towards this goal of strengthening supply chain security. Once a release is marked immutable, its assets and Git tag cannot be changed or deleted. The feature also adds release attestations for verifiable artifact integrity and integrates with protection rulesets, supporting both UI and API workflows.

For the GitHub Actions ecosystem, the benefits include the ability to pin tags with immutable references, preventing potential malicious updates to existing tags and enabling dependable updates via Dependabot. Workflows can now consume trusted, unalterable assets, and deployment pipelines gain improved stability and reproducibility through the use of immutable artifacts.

###   Nov.02 Retired

[pull_request_target blocked by default December 2](https://github.blog/changelog/2026-11-02-pull_request_target-blocked-by-default-december-2)

[actions](https://github.blog/changelog/2026/?label=actions)

###   Oct.28 Release

[GitHub Agentic Workflows generally available](https://github.blog/changelog/2026-10-28-github-agentic-workflows-generally-available)

[actions](https://github.blog/changelog/2026/?label=actions)

###   Oct.28 Improvement

[GitHub Actions adds multi-cloud compute to improve reliability](https://github.blog/changelog/2026-10-28-github-actions-adds-multi-cloud-compute-to-improve-reliability)

[actions](https://github.blog/changelog/2026/?label=actions)

###   Oct.28 Release

[GitHub Actions workflow lockfiles are generally available](https://github.blog/changelog/2026-10-28-github-actions-workflow-lockfiles-are-generally-available)

[actions](https://github.blog/changelog/2026/?label=actions)

###   Oct.22 Improvement

[Meta API endpoints updated for Actions blob storage](https://github.blog/changelog/2026-10-22-meta-api-endpoints-updated-for-actions-blob-storage)

[actions](https://github.blog/changelog/2026/?label=actions)

###   Oct.15 Release

[Xcode 27 runner image generally available](https://github.blog/changelog/2026-10-15-xcode-27-runner-image-generally-available)

[actions](https://github.blog/changelog/2026/?label=actions)

###   Oct.14 Release

[GitHub Actions workflow lockfiles in public preview](https://github.blog/changelog/2026-10-14-github-actions-workflow-lockfiles-in-public-preview)

[actions](https://github.blog/changelog/2026/?label=actions)

###   Oct.02 Improvement

[Stateless GitHub App installation tokens rolled out](https://github.blog/changelog/2026-10-02-stateless-github-app-installation-tokens-rolled-out)

[actions](https://github.blog/changelog/2026/?label=actions)

###   Oct.01 Retired

[GitHub Actions: macOS 14 runner image retirement](https://github.blog/changelog/2026-10-01-github-actions-macos-14-runner-image-retirement)

[actions](https://github.blog/changelog/2026/?label=actions)
