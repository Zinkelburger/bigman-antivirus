---
url: "https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/"
title: "TanStack and 160+ npm/PyPI Packages Compromised in Supply Chain Worm Attack"
site: "Orca Security"
published: "2026-05-12T15:29:00+00:00"
captured: "2026-11-02T18:39:24Z"
via: "live"
sha256: "1af116d77a731a0ef54f1b6135736c95a0f26026284569c674a0ce515579f014"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# TanStack and 160+ npm/PyPI Packages Compromised in Supply Chain Worm Attack

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/>

[Blog Home](https://orca.security/resources/blog/)

-  [Research](https://orca.security/resources/category/research/)
-  [Research Pod](https://orca.security/resources/category/research-pod/)

# TanStack and 160+ npm/PyPI Packages Compromised in Supply Chain Worm Attack

https://orca.security/resources/author/roi-nisimi/

Published:  May 12, 2026

-  https://twitter.com/share?text=TanStack%20and%20160%2B%20npm%2FPyPI%20Packages%20Compromised%20in%20Supply%20Chain%20Worm%20Attack&url=https%3A%2F%2Forca.security%2Fresources%2Fblog%2Ftanstack-npm-supply-chain-worm%2F
-  https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Forca.security%2Fresources%2Fblog%2Ftanstack-npm-supply-chain-worm%2F
-  https://www.facebook.com/sharer.php?u=https%3A%2F%2Forca.security%2Fresources%2Fblog%2Ftanstack-npm-supply-chain-worm%2F
-

## Table of contents

- [Key Takeaways](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-key-takeaways)
- [Executive Summary](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-executive-summary)
- [How the Exploit Chain Worked](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-how-the-exploit-chain-worked)
- [How the Worm Propagates](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-how-the-worm-propagates)
- [The gh-token-monitor Wiper](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-the-gh-token-monitor-wiper)
- [Scope of Compromise](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-scope-of-compromise)
- [Immediate Mitigation Steps](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-immediate-mitigation-steps)
- [Why This Attack Matters](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-why-this-attack-matters)
- [How Can Orca Help?](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-how-can-orca-help)
- [UPDATE May 12th 17:00UTC](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-update-may-12th-17-00utc)

## Key Takeaways

- Critical supply chain compromise disclosed May 12, 2026
- TanStack, Mistral AI, UiPath, and 160+ npm/PyPI packages affected
- TeamPCP attacker group
- Named “Mini Shai-Hulud” due to credential theft, self-propagation, and destructive wiper
- Immediate action required

## Executive Summary

A critical [supply chain compromise](https://orca.security/resources/blog/what-is-a-supply-chain-attack/) was disclosed on May 12, 2026, affecting TanStack, Mistral AI, UiPath, and over 160 additional npm and PyPI packages. The attack, attributed to the threat actor group TeamPCP and dubbed “Mini Shai-Hulud,” allows attackers to steal credentials, self-propagate through the npm ecosystem, and potentially wipe developer home directories via a persistent destructive daemon. Due to the active exploitation and worm-like propagation, immediate action is required.

## How the Exploit Chain Worked

The issue originates from a chain of three [GitHub Actions vulnerabilities](https://orca.security/resources/blog/pull-request-nightmare-github-actions-rce/) exploited in sequence. The attacker forked the [TanStack/router repository](https://github.com/TanStack/router), opened a pull request that triggered a pull_request_target workflow, and poisoned the GitHub Actions cache with a malicious pnpm store. When legitimate maintainer merges later triggered the release workflow, the poisoned cache was restored and attacker-controlled binaries extracted OIDC tokens directly from the GitHub Actions runner process memory via /proc/<pid>/mem. These tokens were then used to publish malicious package versions to npm without ever needing to steal long-lived credentials. Because the malicious packages were published from within the legitimate GitHub Actions runner using valid OIDC tokens, they carry valid npm provenance attestations, meaning provenance alone is not a reliable safety signal for this attack.

## How the Worm Propagates

By installing any affected package version, the malicious payload executes during npm lifecycle hooks and steals GitHub tokens, npm tokens, AWS credentials (via IMDSv2), GCP and Azure credentials, [Kubernetes service account tokens](https://orca.security/platform/container-and-kubernetes-security/), HashiCorp Vault tokens, and environment variables. The payload then identifies npm packages the victim has publish access to, modifies those package archives to inject the same malicious dependency, bumps versions, and publishes new compromised releases using the stolen credentials. This worm behavior means each compromised developer or CI runner becomes a new infection vector, amplifying the attack across the ecosystem.

## The gh-token-monitor Wiper

A particularly dangerous component is the persistent wiper daemon called gh-token-monitor. If the payload finds a valid GitHub token with repo write access and org membership, it installs a daemon that polls GitHub every 60 seconds. If the token is revoked, the daemon executes rm -rf ~/, wiping the user’s home directory. On macOS this persists via a LaunchAgent plist, and on Linux via a systemd user service. The daemon automatically exits after 24 hours. Security teams must locate and remove this daemon before revoking any GitHub tokens to avoid triggering the destructive payload.

No CVE has been assigned at the time of writing. Aikido Security classifies this as a 100/100 [Critical malware issue](https://orca.security/solutions/malware-detection/). No authentication is required to exploit this issue since it triggers automatically during package installation.

## Scope of Compromise

The following components are affected across 373 malicious package-version entries spanning 169 npm package names and 2 PyPI packages. The primary affected namespaces include @tanstack (83 entries across router, start, devtools, and adapter packages), @uipath (66 entries), @squawk (87 entries), @mistralai (including mistralai, mistralai-azure, and mistralai-gcp), @tallyui, @beproduct, and numerous unscoped packages such as intercom-client, lightning, and opensearch-project/opensearch. On PyPI, the affected packages are guardrails-ai 0.10.1 and mistralai 2.4.6.

Exfiltration uses a triple-channel [C2 architecture](https://orca.security/platform/cloud-detection-and-response-cdr/) including the typosquat domain git-tanstack[.]com, the decentralized Session messenger network (via getsession.org seed nodes), and GitHub API dead drops where stolen tokens create Dune-themed repositories. The Session network channel is new in this wave and significantly harder to disrupt than domain-based C2.

## Immediate Mitigation Steps

Users should immediately check for the persistence daemon at ~/Library/LaunchAgents/com.user.gh-token-monitor.plist on macOS or ~/.config/systemd/user/gh-token-monitor.service on Linux and remove it before revoking any tokens. Then audit lockfiles and CI logs for any affected package versions, check .claude/ and .vscode/ directories for persisted payload files such as router_runtime.js or setup.mjs which survive npm uninstall, and rotate all credentials from any affected machine or runner including npm tokens, GitHub PATs, AWS/GCP/Azure credentials, Kubernetes service account tokens, and [CI/CD](https://orca.security/glossary/ci-cd-security/) secrets. Block git-tanstack[.]com, *.getsession.org, and 83.142.209[.]194 at the DNS or proxy level. Upgrade to clean versions published by maintainers after May 12, 2026.

## Why This Attack Matters

At the time of writing, this is an actively exploited supply chain attack with no separate proof of concept needed since the malicious packages themselves are the exploit. The npm team has been notified and is actively removing affected versions. This is the third documented wave from TeamPCP, following compromises of SAP npm packages in late April 2026 and PyTorch Lightning on April 30, 2026.

## How Can Orca Help?

Orca enables customers to quickly identify assets running vulnerable versions, understand their exposure in context including internet accessibility, runtime reachability, and asset criticality, and prioritize remediation based on real risk rather than CVSS alone. [Orca’s platform](https://orca.security/platform/) highlights affected assets directly in the newItem view, helping security teams focus on the most critical remediation paths first.

##  UPDATE May 12th 17:00UTC

In a troubling development, TeamPCP briefly published the Shai-Hulud source code on GitHub before the repository was removed. Copies have already been mirrored online, enabling defenders to improve detections but also giving threat actors a ready-made foundation to create new variants, similar to how the Mirai botnet evolved after its source code was released publicly. This significantly increases the likelihood of copycat attacks and derivative strains. At this moment it is too early to know the full impact this will have, but we continue to monitor for any developments.

## Table of contents

- [Key Takeaways](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-key-takeaways)
- [Executive Summary](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-executive-summary)
- [How the Exploit Chain Worked](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-how-the-exploit-chain-worked)
- [How the Worm Propagates](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-how-the-worm-propagates)
- [The gh-token-monitor Wiper](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-the-gh-token-monitor-wiper)
- [Scope of Compromise](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-scope-of-compromise)
- [Immediate Mitigation Steps](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-immediate-mitigation-steps)
- [Why This Attack Matters](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-why-this-attack-matters)
- [How Can Orca Help?](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-how-can-orca-help)
- [UPDATE May 12th 17:00UTC](https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/#h-update-may-12th-17-00utc)

-  https://twitter.com/share?text=TanStack%20and%20160%2B%20npm%2FPyPI%20Packages%20Compromised%20in%20Supply%20Chain%20Worm%20Attack&url=https%3A%2F%2Forca.security%2Fresources%2Fblog%2Ftanstack-npm-supply-chain-worm%2F
-  https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Forca.security%2Fresources%2Fblog%2Ftanstack-npm-supply-chain-worm%2F
-  https://www.facebook.com/sharer.php?u=https%3A%2F%2Forca.security%2Fresources%2Fblog%2Ftanstack-npm-supply-chain-worm%2F
-

## Related articles

https://orca.security/resources/blog/how-to-build-your-actual-network-map-with-custom-agents-in-orca/

##  [How to Build Your Actual Network Map with Custom Agents in Orca](https://orca.security/resources/blog/how-to-build-your-actual-network-map-with-custom-agents-in-orca/)

Oct 16, 2026

https://orca.security/resources/blog/frontier-level-red-teaming-from-a-self-hostable-open-weight-model-testing-nvidia-nemotron-3-5-super/

Research

##  [Frontier-Level Red-Teaming From a Self-Hostable Open-Weight Model: Testing NVIDIA Nemotron 3.5 Super](https://orca.security/resources/blog/frontier-level-red-teaming-from-a-self-hostable-open-weight-model-testing-nvidia-nemotron-3-5-super/)

Oct 15, 2026

https://orca.security/resources/research/critical-gitlab-rce-lets-unauthenticated-attackers-execute-code-on-servers-cve-2026-97229/

Research

##  [Critical GitLab RCE Lets Unauthenticated Attackers Execute Code on Servers (CVE-2026-97229)](https://orca.security/resources/research/critical-gitlab-rce-lets-unauthenticated-attackers-execute-code-on-servers-cve-2026-97229/)

Oct 14, 2026

### Stay in the loop

Keep up to date with everything you need to know about cloud security and our latest research

By submitting my email address I agree to the use of my personal data in accordance with Orca Security [Privacy Policy](https://orca.security/privacy-policy/).
