---
url: "https://brew.sh/2024/07/30/homebrew-security-audit"
title: "2023 Security Audit"
site: "Homebrew"
published: "2024-07-30T00:00:00+00:00"
captured: "2026-11-02T18:24:10Z"
via: "live"
source_url: "https://brew.sh/2024/07/30/homebrew-security-audit/"
sha256: "ee47c5e1db876dc547cb1f6fea8d5471525fb56093578578a590c81293ec534b"
wayback: "https://web.archive.org/web/20261017040920/https://brew.sh/2024/07/30/homebrew-security-audit/"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# 2023 Security Audit

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://brew.sh/2024/07/30/homebrew-security-audit>
> Snapshot: <https://web.archive.org/web/20261017040920/https://brew.sh/2024/07/30/homebrew-security-audit/>

## 2023 Security Audit

### 30 July 2024

### [p-linnane](https://github.com/p-linnane)

Homebrew had a security audit performed in 2023. This audit was funded by the [Open Technology Fund](https://www.usagm.gov/networks/otf/) and conducted by [Trail of Bits](https://www.trailofbits.com). Trail of Bits’ report contained 25 items, of which 16 were fixed, 3 are in progress, and 6 are acknowledged by Homebrew’s maintainers. Below is the scope of testing, findings by severity, and mitigation and acknowledgements.

You can read Trail of Bits’ blog post on the audit [here](https://blog.trailofbits.com/2024/07/30/our-audit-of-homebrew/) and find the full public report [here](https://github.com/trailofbits/publications/blob/eb9344f2261031a4be1be2f223e9b5bc535be6b9/reviews/2023-08-28-homebrew-securityreview.pdf).

Homebrew’s maintainers and Project Leadership Committee would like to thank Open Technology Fund and Trail of Bits for sponsoring and running this engagement. Our partnership directly improves the security of Homebrew and open source software in general.

Scope: [Homebrew/brew](https://github.com/Homebrew/brew), [Homebrew/actions](https://github.com/Homebrew/actions), [Homebrew/formulae.brew.sh](https://github.com/Homebrew/formulae.brew.sh), [Homebrew/homebrew-test-bot](https://github.com/Homebrew/homebrew-test-bot).

Findings by severity:

- High: 0
- Medium: 14
- Low: 2
- Informational: 7
- Undetermined: 2

Mitigation and acknowledgements:

- Path traversal during file caching

- Status: [Fixed](https://github.com/Homebrew/brew/commit/5aebde3ffdb38a7087ac992af144c957dd802788)

- Sandbox escape via string injection

- Status: [Fixed](https://github.com/Homebrew/brew/pull/17700/commits/f4e5e0c716585b072b5a401602f7847272ae5d4a)

- Allow default rule in sandbox configuration is overly permissive

- Status: [Fixed](https://github.com/Homebrew/brew/commit/fea65f760332a26c116de4b675e07f679bf95b1c)

- Special characters are allowed in package names and versions

- Status: Acknowledged

- Use of weak cryptographic digest in Formulary namespaces

- Status: [Fixed](https://github.com/Homebrew/brew/commit/310b5ecbe7c578c850f35d88b3ba9adbb291fc83)

- Extraction is not sandboxed

- Status: Acknowledged

- Use of ldd on untrusted inputs

- Status: [Fixed](https://github.com/Homebrew/brew/commit/0fc9c9f81e2a740d733062e1b47df2b49a8363c6)

- Formulas allow for external resources to be downloaded during the install step

- Status: [In Progress](https://github.com/Homebrew/brew/pull/17703)

- Use of Marshal

- Status: [Fixed](https://github.com/Homebrew/brew/commit/e466417cdf90c7dc8aab717cc5a591c1f31f9415)

- Lack of sandboxing on Linux

- Status: Acknowledged

- Sandbox escape through domain socket pivot on macOS

- Status: [In Progress](https://github.com/Homebrew/brew/pull/17734)

- Formula privilege escalation through sudo

- Status: [Fixed](https://github.com/Homebrew/brew/pull/17694/commits/2adf25dcaf8d8c66124c5b76b8a41ae228a7bb02)

- Formula loading through SFTP, SCP and other protocols

- Status: [Fixed](https://github.com/Homebrew/brew/pull/17697/commits/8b0a4a98bf10b3441008906632648977b4a4f23e)

- Sandbox allows changing permissions for important directories

- Status: [Fixed](https://github.com/Homebrew/brew/commit/92ef6ef7639b08b182fc0d9154cf51561365f749)

- Homebrew supports only end-of-life versions of Ruby

- Status: [Fixed](https://github.com/Homebrew/brew/pull/16294/commits/987ebe3d6d0aaa27ab9ca2e95fe807e0718d4378)

- Path traversal during bottling

- Status: [Fixed](https://github.com/Homebrew/brew/commit/5aebde3ffdb38a7087ac992af144c957dd802788)

- FileUtils.rm_rf does not check if files are deleted

- Status: [In Progress](https://github.com/Homebrew/brew/pull/17705)

- Use of pull_request_target in GitHub Actions workflows

- Status: Fixed: [1](https://github.com/Homebrew/brew/commit/b9da669ef25c7da0757f5ae1d418985b19efb321), [2](https://github.com/Homebrew/actions/commit/323fc6399f7ddf2cb6ee7eb9d900dcd4deee76ec).

- Use of unpinned third-party workflow

- Status: Fixed across the codebase via multiple PRs.

- Unpinned dependencies in formulae.brew.sh

- Status: [Fixed](https://github.com/Homebrew/formulae.brew.sh/commit/83ae27233abbf7b59db9aeb5bae311ecabf1add8)

- Use of RSA for JSON API signing

- Status: Acknowledged. Ed25519 was not an option when this was introduced. The next key reroll will use Ed25519.

- Bottles beginning “-“ can lead to unintended options getting passed to rm

- Status: [Fixed](https://github.com/Homebrew/homebrew-test-bot/pull/1062/commits/5565ba609cb6456d22741751465f088b44209813)

- Code injection through inputs in multiple actions

- Status: Fixed across the codebase via multiple PRs.

- Use of PGP for commit signing

- Status: Acknowledged. Plans to remove the bot account using PGP have been established.

- Unnecessary domain separation between signing key and key ID

- Status: Acknowledged. Will be resolved with the next key reroll.

### Latest Posts

-  [7.0.0](https://brew.sh/2026/09/13/homebrew-7.0.0/) 13 Sep 2026

Today, I’m proud to announce Homebrew 7.0.0. The most significant changes since 6.0.0 are faster installations and upgrades, stronger sandboxing, a native macOS app, b...

-  [6.0.0](https://brew.sh/2026/06/11/homebrew-6.0.0/) 11 Jun 2026

Today, I’m proud to announce Homebrew 6.0.0. The most significant changes since 5.1.0 are a new tap trust security mechanism, the new faster, smaller, default internal...

-  [5.1.0](https://brew.sh/2026/03/10/homebrew-5.1.0/) 10 Mar 2026

Homebrew 5.1.0 has been released. Homebrew’s most significant changes since 5.0.0 are expanded brew bundle support, brew version-install, new -full formula handling an...

-  [5.0.0](https://brew.sh/2025/11/12/homebrew-5.0.0/) 12 Nov 2025

Today, I’d like to announce Homebrew 5.0.0. The most significant changes since 4.6.0 are download concurrency by default, official support for Linux ARM64/AArch64, tim...

-  [4.6.0](https://brew.sh/2025/08/05/homebrew-4.6.0/) 05 Aug 2025

Today, I’d like to announce Homebrew 4.6.0. The most significant changes since 4.5.0 are opt-in concurrent downloads with HOMEBREW_DOWNLOAD_CONCURRENCY, preliminary ma...
