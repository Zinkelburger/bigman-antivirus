---
url: "https://www.gitguardian.com/state-of-secrets-sprawl-report-2025"
title: "State of Secrets Sprawl Report 2025"
captured: "2026-11-02T18:54:46Z"
via: "live"
sha256: "6ca885c377c19c346908bfe5f77598a67e3363576b1d5457aee16f6e38b4bddb"
wayback: "https://web.archive.org/web/20261005011649/https://www.gitguardian.com/state-of-secrets-sprawl-report-2025"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# State of Secrets Sprawl Report 2025

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.gitguardian.com/state-of-secrets-sprawl-report-2025>
> Snapshot: <https://web.archive.org/web/20261005011649/https://www.gitguardian.com/state-of-secrets-sprawl-report-2025>

https://www.gitguardian.com/

Download the State of Secrets Sprawl 2025 full report

[Download the Report](https://www.gitguardian.com/files/the-state-of-secrets-sprawl-report-2025)

the state of

# Secrets Sprawl 2025

[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)[Download the Report](https://www.gitguardian.com/files/the-state-of-secrets-sprawl-report-2025)

Data analysis by GitGuardian

## Leaked Today, Vulnerable Tomorrow

23,770,171

+25%

New secrets detected in public GitHub commits in 2024

Data analysis by GitGuardian

in 2024 GitGuardian scanned 69.6M public repositories of which at least 4.61% contained a secret.

15%

commit authors leaked a secret

1.9 Million

pro-bono alert emails sent

70%

of the secrets leaked in 2022 are still valid

[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)

## Secret Leaks: Slow Fix, Big Risk

Over the past 10 years, the use of stolen credentials has appeared in 31% of all breaches.

[Verizon's 2024 Data Breach Investigations Report](https://www.verizon.com/business/resources/Te3/reports/2024-dbir-data-breach-investigations-report.pdf)

Breaches involving stolen or compromised credentials take an average of 292 days to identify and remediate.

[IBM’s Cost of a Data Breach](https://www.ibm.com/reports/data-breach)

### Secrets Sprawl Surges

#### New secrets detected on GitHub

Data analysis by GitGuardian

25%

Increase in leaked credentials from the previous year, indicating that secrets sprawl is worsening over time.

In 2024, we found 23,770,171 new hardcoded secrets added to public GitHub repositories. Secrets sprawl is steadily worsening over time.

[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)

### Generic Secrets: A Growing Blind Spot

#### Secrets by detector nature

Generic ML-certified

Generic rule-based certified

Specific

Data analysis by GitGuardian

58%

of all detected leaks are [Generic credentials](https://blog.gitguardian.com/why-detecting-generic-credentials-is-a-game-changer/).
A sharp increase from 2023.

GitGuardian's latest AI features now uncover significantly more generic secrets, providing the most comprehensive and accurate view of secrets sprawl.

[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)

### GitHub Push Protection: Not a Silver Bullet

strong

GitHub’s Push Protection has its limitations. While it reduces leaks for some keys, it struggles with generic secrets—the fastest-growing category. These are harder to detect because they lack standardized patterns. Also many other keys, such as MySQL and MongoDB credentials, lack a standardized prefix and still evade GitHub's filter.

#### MySQL credentials were not impacted by GitHub Push Protection

Data analysis by GitGuardian

#### MongoDB credentials were not impacted by GitHub Push Protection

Data analysis by GitGuardian

[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)

### Private Repositories: A False Sense of Security

#### Private vs Public repositories

Contains secret

No secret

Data analysis by GitGuardian

35%

of all customers’ private repositories scanned contained at least one plaintext secret.

Developers treat secrets in private repositories less cautiously than in public ones, assuming privacy equals security. Private repos are 9 times more likely to contain secrets, and become public due to misconfigurations or breaches. Organizations must enforce robust secrets management across all environments—not just public code.

[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)

### Secrets Are Leaking Beyond Source Code

#### Incidents severity: SCM systems vs collaboration tools

Critical

High

Data analysis by GitGuardian

38%

of incidents in collaboration tools are classified as highly critical or urgent—a higher proportion than in code repositories.

Secrets are found leaking across various enterprise platforms. Slack, Jira, and Confluence leaks are widespread, yet often overlooked, as these collaboration tools lack built-in security safeguards.

[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)

### Public Docker Images Are a Massive Attack Surface

#### Breakdown of valid secrets found in public Docker images

Data analysis by GitGuardian

7000+

Valid AWS keys remain exposed on Docker Hub.

GitGuardian’s largest-scale analysis of 15 million public Docker images uncovered 100,000 valid secrets, including AWS keys and GitHub tokens from Fortune 500 companies.

[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)

### Even Secrets Managers Can’t Stop Secrets Sprawl

#### Public repos with and without secrets manager

Public repos without secrets managers

Public repos with secrets managers

Data analysis by GitGuardian

5.1%

of repositories using secrets managers still leaked secrets in 2024.

Top 3 Leaked Secrets:

AWS IAM

Slack webhooks

Azure AD API keys

Misconfigurations, improper access control, and insecure authentication practices continue to expose credentials. Secrets sprawl remains a problem even in organizations with dedicated secrets managers.

[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)

### Leaked secrets stay active for years

When a secret leaks, remediation should be immediate. Yet, the data tells a different story.

70%

of secrets leaked in 2022 are still valid today.

#### % of secrets still valid since exposure

Stil valid

Revoked

Data analysis by GitGuardian

Remediation is slow due to enforcement gaps & complex workflows. Secrets stay valid because organizations lack proper monitoring.

The Non-Human Identity Crisis

Machine-to-machine communications dominate modern software architecture, but security policies haven’t kept pace. Non-human identities (NHIs) outnumber human users in corporate environments, they are mostly long-lived credentials with little to no expiration. Without lifecycle management, these long-lived credentials create enduring security vulnerabilities.

[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)

### AI Coding Assistants Are Exposing Your Secrets

#### Copilot enabled public repos leaking secrets

Contains Secret

No Secret

Data analysis by GitGuardian

40%

higher exposure rate than the average across all public repositories.

GitHub Copilot usage increased by 27% between 2023 and 2024.

GitGuardian found that public repositories using Copilot had a 6.4% secret leakage rate.

This suggests that while AI enhances productivity, it may also introduce security risks, reinforcing the need for strong secret detection controls.

[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)[Share](https://www.gitguardian.com/state-of-secrets-sprawl-report-2025#)

## Examples of Exposed Secrets in the Wild

Artifactory Token Exposures

Our case study reveals that Artifactory token leaks, while less common, pose a major supply chain risk. 60% of leaked tokens were in build configs, impacting production environments in critical sectors like pharma and energy.

AWS S3 Ransomware Attack

Attackers used valid AWS credentials to encrypt S3 buckets and demanded ransom for decryption. This shows how legitimate cloud features can be weaponized when secrets leak.

## The Path Forward >>

## How to Fight Secrets Sprawl

- Enforce semi-automated secret expiration & rotation
- Transition from static to dynamic secrets
- Implement least privilege access controls
- Enable cross-vault analytics for enhanced secrets management
- Prioritize both early detection and swift remediation

### Download the full Report!

Download the State of Secrets Sprawl 2025 report and learn to implement best practices for secrets management and security.

[Download the Report](https://www.gitguardian.com/files/the-state-of-secrets-sprawl-report-2025)

### Subscribe to our newsletter

Receive the latest content and updates from GitGuardian.

Thank you! Your subscription has been registered!

Oops! Something went wrong while submitting the form.

#### Platform

[Internal Secrets Monitoring](https://www.gitguardian.com/monitor-internal-repositories-for-secrets)[Public Secrets Monitoring](https://www.gitguardian.com/monitor-public-github-for-secrets)[NHI Governance](https://www.gitguardian.com/nhi-governance)[Integrations](https://www.gitguardian.com/integrations)[Compare GitGuardian](https://www.gitguardian.com/comparisons)[Pricing](https://www.gitguardian.com/pricing)[Value Calculator](https://www.gitguardian.com/value-calculator)

#### Explore

[Good Samaritan](https://www.gitguardian.com/enterprise-good-samaritan)[Labs](https://labs.gitguardian.com/)[Events](https://www.gitguardian.com/events)[Security Champions](https://www.gitguardian.com/security-champions)[GitHub Security Audit](https://www.gitguardian.com/github-security-audit)[GitGuardian CLI](https://www.gitguardian.com/ggshield)[GitGuardian Scout](https://www.gitguardian.com/ggscout)

#### Resources

[Blog](https://blog.gitguardian.com/)[Learning Center](https://www.gitguardian.com/learning-center)[State of Secrets Sprawl](https://www.gitguardian.com/state-of-secrets-sprawl-report-2026)[Roadmap](https://roadmap.gitguardian.com/)[Documentation](https://docs.gitguardian.com/)[API](https://api.gitguardian.com/docs)[Status](https://status.gitguardian.com/)

#### Company

[About us](https://www.gitguardian.com/about-us)[Community](https://www.gitguardian.com/wall-of-love)[Careers](https://www.gitguardian.com/careers)[Newsroom](https://www.gitguardian.com/newsroom)[Partners](https://www.gitguardian.com/partners)[Legal](https://www.gitguardian.com/legal)[Press Kit](https://cdn.prod.website-files.com/5ee25cbe47310017adf964da/6a83c0defca09c4225e10dc8_GitGuardian_Press_Kit.zip)

#### Connect

[Contact us](https://www.gitguardian.com/contact-us)[LinkedIn](https://www.linkedin.com/company/gitguardian/)[Youtube](https://www.youtube.com/channel/UCHGxYyzf4eXzpMz5kyqVZSQ)[GitHub](https://github.com/GitGuardian)[Facebook](https://www.facebook.com/GitGuardian/)[X (Twitter)](https://twitter.com/gitguardian?lang=en)
