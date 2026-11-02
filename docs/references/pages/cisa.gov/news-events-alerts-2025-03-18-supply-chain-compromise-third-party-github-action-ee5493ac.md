---
url: "https://www.cisa.gov/news-events/alerts/2025/03/18/supply-chain-compromise-third-party-github-action-cve-2025-30066"
title: "Supply Chain Compromise of Third-Party tj-actions/changed-files (CVE-2025-30066) and reviewdog/action-setup@v1 (CVE-2025-30154) | CISA"
site: "Cybersecurity and Infrastructure Security Agency CISA"
captured: "2026-11-02T18:52:44Z"
via: "live"
source_url: "https://www.cisa.gov/news-events/alerts/2025/03/18/supply-chain-compromise-third-party-tj-actionschanged-files-cve-2025-30066-and-reviewdogaction"
sha256: "c20e713135820d8f77f1dd4141bee6538d5b721e4f705e964e2f9da2f29147b4"
wayback: "https://web.archive.org/web/20250326030528/https://www.cisa.gov/news-events/alerts/2025/03/18/supply-chain-compromise-third-party-github-action-cve-2025-30066"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Supply Chain Compromise of Third-Party tj-actions/changed-files (CVE-2025-30066) and reviewdog/action-setup@v1 (CVE-2025-30154) | CISA

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.cisa.gov/news-events/alerts/2025/03/18/supply-chain-compromise-third-party-github-action-cve-2025-30066>
> Snapshot: <https://web.archive.org/web/20250326030528/https://www.cisa.gov/news-events/alerts/2025/03/18/supply-chain-compromise-third-party-github-action-cve-2025-30066>

Alert

#  Supply Chain Compromise of Third-Party tj-actions/changed-files (CVE-2025-30066) and reviewdog/action-setup@v1 (CVE-2025-30154)

Last Revised

March 26, 2025

A popular third-party GitHub Action, tj-actions/changed-files (tracked as [CVE-2025-30066](https://www.cve.org/CVERecord?id=CVE-2025-30066)), was compromised. tj-actions/changed-files is designed to detect which files have changed in a pull request or commit. The supply chain compromise allows for information disclosure of secrets including, but not limited to, valid access keys, GitHub Personal Access Tokens (PATs), npm tokens, and private RSA keys. This has been patched in v46.0.1.

(Updated March 19, 2025) The compromise of tj-actions/changed-files was potentially enabled by a compromise of another GitHub Action, reviewdog/action-setup@v1 (tracked as [CVE-2025-30154](https://www.cve.org/CVERecord?id=CVE-2025-30154)), which occurred around the same time. The following Actions may also be affected:

- reviewdog/action-shellcheck

- reviewdog/action-composite-template

- reviewdog/action-staticcheck

- reviewdog/action-ast-grep

- reviewdog/action-typos

(Updated March 26, 2025) CISA added CVE-2025-30066 and CVE-2025-30154 to its [Known Exploited Vulnerabilities Catalog](https://www.cisa.gov/known-exploited-vulnerabilities-catalog).

CISA strongly urges users to implement the following recommendations to mitigate this compromise. If your organization is impacted:

(Updated March 26, 2025)

- Identify affected repositories. Conduct an audit to locate all projects using all versions of tj-actions/changed-files between 2025-03-12 00:00 UTC to 2025-03-15 12:00 UTC in your organization and/or the reviewdog/action between March 11, 2025, between 18:42 and 20:31 UTC.

- Identify exposed secrets. For public repositories with workflows that ran the malicious commit, check for exposed access keys, GitHub Personal Access Tokens (PATs), npm tokens, and private RSA keys. Note: Secrets may be obfuscated as a double-encoded base64 payload.

- Rotate all identified secrets immediately as they should be considered compromised.

- Update to latest version of [reviewdog/action-setup@3f401fe](https://github.com/reviewdog/action-setup/commit/3f401fe) and/or follow [these instructions](https://github.com/tj-actions/changed-files/blob/45fb12d7a8bedb4da42342e52fe054c6c2c3fd73/README.md?plain=1#L20-L28) provided by GitHub.

Organizations should investigate and report incidents and malicious activity to CISA’s 24/7 Operations Center at Report@cisa.gov or (888) 282-0870.

See the following resources for more guidance to reduce risk when using third-party GitHub Actions:

- GitHub: [tj-actions changed-files through 45.0.7 allows remote attackers to discover secrets by reading actions logs](https://github.com/advisories/GHSA-mrrh-fwg8-r2c3)

- GitHub: [Security hardening for GitHub Actions - GitHub Docs](https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions#using-third-party-actions)

- GitHub: [tj-actions/changed-files: :octocat: Github action to retrieve all (added, copied, modified, deleted, renamed, type changed, unmerged, unknown) files and directories](https://github.com/tj-actions/changed-files?tab=readme-ov-file#changed-files)

- StepSecurity: [Harden-Runner detection: tj-actions/changed-files action is compromised](https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised#recovery-steps)

- Wiz: [GitHub Action tj-actions/changed-files supply chain attack](https://www.wiz.io/blog/github-action-tj-actions-changed-files-supply-chain-attack-cve-2025-30066)

- (Updated March 19, 2025) GitHub: [Multiple Reviewdog actions were compromised during a specific time period · CVE-2025-30154 · GitHub Advisory Database](https://github.com/advisories/GHSA-qmg3-hpqr-gqvc)

- (Updated March 26, 2025) Semgrep: [Popular GitHub Action tj-actions/changed-files is compromised](https://semgrep.dev/blog/2025/popular-github-action-tj-actionschanged-files-is-compromised/)

This alert is provided “as is” for informational purposes only. CISA does not provide any warranties of any kind regarding any information within. CISA does not endorse any commercial product, entity, or service referenced in this alert or otherwise.

This product is provided subject to this [Notification](https://www.cisa.gov/notification) and this [Privacy & Use](https://www.cisa.gov/privacy-policy) policy.

## Please share your thoughts

We recently updated our anonymous [product survey](https://cisasurvey.gov1.qualtrics.com/jfe/form/SV_9n4TtB8uttUPaM6?product=https://www.cisa.gov/news-events/alerts/2025/03/18/supply-chain-compromise-third-party-tj-actionschanged-files-cve-2025-30066-and-reviewdogaction); we welcome your feedback.
