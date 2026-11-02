---
url: "https://www.zscaler.com/blogs/security-research/supply-chain-attacks-surge-march-2026"
title: "Axios NPM & LiteLLM PyPI Supply Chain Attacks 2026 | ThreatLabz"
captured: "2026-11-02T19:02:35Z"
via: "live"
sha256: "20a4c807dd359ce83a512ba7a74116401ccbe37114c846d2ac7809b254edf1f5"
wayback: "https://web.archive.org/web/20260901094410/https://www.zscaler.com/blogs/security-research/supply-chain-attacks-surge-march-2026"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Axios NPM & LiteLLM PyPI Supply Chain Attacks 2026 | ThreatLabz

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.zscaler.com/blogs/security-research/supply-chain-attacks-surge-march-2026>
> Snapshot: <https://web.archive.org/web/20260901094410/https://www.zscaler.com/blogs/security-research/supply-chain-attacks-surge-march-2026>

Zscaler Blog

Get the latest Zscaler blog updates in your inbox

[Subscribe](https://www.zscaler.com/blogs/security-research/supply-chain-attacks-surge-march-2026#get-latest-updates)

[Security Research](https://www.zscaler.com/blogs?type=security-research)

# Supply Chain Attacks Surge in March 2026

[THREATLABZ](https://www.zscaler.com/author/threatlabz)

April 03, 2026 - 12 min read

[Home](https://www.zscaler.com/)

>[Blogs](https://www.zscaler.com/blogs)

>Supply Chain Attacks Surge in March 2026

[Threat Research](https://www.zscaler.com/blogs?topic=threat-research)

https://www.zscaler.com/cdn-cgi/l/email-protection#9da2eee8fff7f8fee9a0cee8ededf1e4b8afaddef5fcf4f3b8afaddce9e9fcfef6eeb8afadcee8effaf8b8afadf4f3b8afadd0fceffef5b8afadafadafabbbfcf0eda6fff2f9e4a0f5e9e9edeea7b2b2e9f4f3e4e8eff1b3fef2f0b2afa8fbe8a9e7a8abhttps://www.facebook.com/sharer/sharer.php?u=https://tinyurl.com/25fu4z56https://x.com/intent/tweet?text=https://tinyurl.com/25fu4z56 — Supply%20Chain%20Attacks%20Surge%20in%20March%202026"https://www.linkedin.com/sharing/share-offsite/?url=https://tinyurl.com/25fu4z56https://www.zscaler.com/blogs/feeds/security-research

Contents

- [Introduction](https://www.zscaler.com/blogs/security-research/supply-chain-attacks-surge-march-2026#introduction)
- [Axios NPM Package Compromised to Distribute Cross-Platform RAT](https://www.zscaler.com/blogs/security-research/supply-chain-attacks-surge-march-2026#axios-npm-package-compromised-to-distribute-cross-platform-rat)
- [TeamPCP Supply Chain Attack Targets LiteLLM on PyPI](https://www.zscaler.com/blogs/security-research/supply-chain-attacks-surge-march-2026#teampcp-supply-chain-attack-targets-litellm-on-pypi)
- [Conclusion](https://www.zscaler.com/blogs/security-research/supply-chain-attacks-surge-march-2026#conclusion)
- [Zscaler Coverage](https://www.zscaler.com/blogs/security-research/supply-chain-attacks-surge-march-2026#zscaler-coverage)
- [Indicators Of Compromise (IOCs)](https://www.zscaler.com/blogs/security-research/supply-chain-attacks-surge-march-2026#indicators-of-compromise--iocs-)
- [FAQs](https://www.zscaler.com/blogs/security-research/supply-chain-attacks-surge-march-2026#faqs)
- [More blogs](https://www.zscaler.com/blogs/security-research/supply-chain-attacks-surge-march-2026#more-blogs)

## Introduction

There was a significant increase in software supply chain attacks in March 2026. There were five major software supply chain attacks that occurred including the Axios NPM package compromise, which has been attributed to a North Korean threat actor. In addition, a hacking group known as TeamPCP was able to compromise Trivy (a vulnerability scanner), KICS (a static analysis tool), LiteLLM (an interface for AI models), and Telnyx (a library for real-time communication features).

In this blog, we cover two of these supply chain attacks, which are significant given the scale and popularity of these packages.

## Axios NPM Package Compromised to Distribute Cross-Platform RAT

### Summary

On March 30, 2026, security researchers discovered that the widely-used NPM package Axios was compromised through an account takeover attack targeting a lead maintainer. Threat actors bypassed the project's GitHub Actions CI/CD pipeline by compromising the maintainer's NPM account and changing its associated email. The threat actor manually published two malicious versions via NPM CLI.

These poisoned releases inject a hidden dependency called [[email protected]](https://www.zscaler.com/cdn-cgi/l/email-protection), which executes a postinstall script functioning as a cross-platform Remote Access Trojan (RAT) dropper targeting macOS, Windows, and Linux systems.

During execution, the malware contacts command-and-control (C2) infrastructure at sfrclak[.]com to deliver platform-specific payloads, then deletes itself and replaces its package.json with a clean version to evade detection.

### Recommendations

- Review package.json, package-lock.json, and yarn.lock files for [[email protected]](https://www.zscaler.com/cdn-cgi/l/email-protection), [[email protected]](https://www.zscaler.com/cdn-cgi/l/email-protection), or [[email protected]](https://www.zscaler.com/cdn-cgi/l/email-protection). Remove any compromised packages, clear caches, and reinstall clean ones.
- Downgrade to [[email protected]](https://www.zscaler.com/cdn-cgi/l/email-protection) (for 1.x users) or [[email protected]](https://www.zscaler.com/cdn-cgi/l/email-protection) (for 0.x users) and update lockfiles.
- Search for connections to sfrclak[.]com or 142.11.206[.]73 from developer workstations and CI/CD systems.
- Use private registry proxies and Software Composition Analysis (SCA) tools to filter and monitor third-party packages.
- Restrict open-source package consumption on corporate devices and CI systems to enterprise-open source package managers. Use Zscaler Internet Access controls to block access to internet package managers from corporate devices. Use native controls and Zscaler Private App Connectors to block access to internet package managers from CI systems.
- Apply lockfiles strictly (e.g., package-lock.json, pnpm-lock.yaml) and use npm ci instead of npm install.
- Reduce dependency surface by auditing and removing unused packages.
- Apply least privilege principles using scoped, short-lived keys and tokens.
- Revoke NPM tokens, GitHub PATs, cloud keys, and CI/CD secrets.
- Enable [phishing-resistant multifactor authentication (MFA)](https://www.zscaler.com/zpedia/phishing-resistant-mfa) on NPM, GitHub, and cloud platforms.
- Flag abnormal NPM publishes, unexpected GitHub workflow additions, or secret scanner usage in CI.
- Treat impacted systems as compromised by isolating, scanning, or reimaging them.
- Update response playbooks for supply chain attacks and run practice drills.
- Restrict build environments to internal package managers or trusted mirrors, and limit internet access to reduce exfiltration risk.
- Reinforce the secure handling of tokens and secrets, and train teams on phishing awareness and supply chain security best practices.
- Enforce a release cooldown period to ensure users can’t check out newly released packages, stopping emerging supply chain attacks.

### Affected packages and versions

The following packages are impacted by this compromise.

|

Package

|

Version

|

Axios

|

1.14.1

|

Axios

|

0.30.4

Table 1: Axios package versions impacted by the compromise.

### How it works

All NPM packages include a package.json file that declares dependencies. In the compromised version of Axios, the threat actor added a dependency for a malicious package called plain-crypto-js, which included a postinstall script that ran a setup.js script via node.

When developers or CI pipelines run npm install [[email protected]](https://www.zscaler.com/cdn-cgi/l/email-protection), NPM resolves the dependency tree, downloads [[email protected]](https://www.zscaler.com/cdn-cgi/l/email-protection), and runs the postinstall script. Running node setup.js triggers the compromise sequence.

#### Attack chain

The figure below shows the attack chain.

Figure 1: Attack chain for the compromised Axios package.

## TeamPCP Supply Chain Attack Targets LiteLLM on PyPI

### Summary

On March 26, 2026, a supply chain attack was uncovered targeting LiteLLM, a popular AI infrastructure library hosted on PyPI with roughly [3.4 million downloads per day](https://pypistats.org/packages/litellm). Two LiteLLM package versions were found to include malicious code published by a threat group called TeamPCP. TeamPCP has been associated with multiple recent supply chain attacks such as [KICS](https://www.wiz.io/blog/teampcp-attack-kics-github-action), [Telnyx](https://www.trendmicro.com/en_us/research/26/c/teampcp-telnyx-attack-marks-a-shift-in-tactics.html), and an [attack](https://www.wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack) on Aqua Security’s Trivy. The impacted package versions of LiteLLM were only available in PyPI for about three hours before they were quarantined.

The poisoned LiteLLM packages appear to be part of an attack designed to harvest high-value secrets such as AWS, GCP, and Azure tokens, SSH keys, and Kubernetes credentials, enabling lateral movement and long-term persistence across compromised CI/CD systems and production environments.

### Recommendations

- Rotate or revoke all potentially exposed secrets such as PyPI tokens, API keys, SSH keys, and cloud credentials. Remove unused secrets, and restrict access to sensitive stores and configuration files (for example, .env files, SSH keys, and cloud CLI configs) using least-privilege controls and strict filesystem or secret-store permissions.
- Closely monitor PyPI publishing activity and recent release history, limit and regularly review maintainer access, and enforce MFA for all maintainers. Strengthen dependency integrity by prioritizing review of Git diffs for dependency version changes to spot suspicious modifications, and implement alerting for any unexpected direct or transitive dependency updates. Verify hashes and signatures where supported.
- Restrict who or what can run builds and publish artifacts, eliminate plaintext secrets in pipelines, and move to secret managers plus short-lived and ephemeral tokens. Add protected branches and tags, mandatory reviews for release workflows, and limit runner and network permissions.
- Apply least-privilege Identity and Access Management (IAM), tighten Kubernetes Role-Based Access Control (RBAC), and reduce credential exposure paths. Ensure container and runtime policies prevent credential harvesting and restrict workload identity access to only the required resources.

### Affected versions and delivery

The following versions of LiteLLM were impacted. Users should upgrade to version 1.82.6 (the last known clean version).

|

Version

|

Delivery

|

1.82.8

|

This version introduced a .pth file (litellm_init.pth) added to site-packages/. Python automatically executes code within .pth files during startup, meaning the malicious payload triggers on any Python invocation on the host, even if LiteLLM itself is not imported.

The .pth file is correctly recorded in the wheel’s RECORD, so pip’s hash verification and other integrity checks still pass because the malicious content was published with legitimate credentials rather than injected afterward.

|

1.82.7

|

This version introduced an obfuscated Base64-encoded payload within proxy_server.py. This payload is designed to execute immediately upon the library being imported.

Table 2: LiteLLM package versions affected and their corresponding delivery mechanism.

### How it works

LiteLLM is a wrapper or proxy for AI models that lets developers call different LLMs using an OpenAI-style API. Since it’s published on PyPI, a developer might download it by installing it for a project with the standard Python package installer, either directly or as part of an automated dependency install.

#### Attack chain

The attack chain for the compromised packages is shown below.

Figure 2: Attack chain for compromised LiteLLM packages.

## Conclusion

These supply chain threats highlight the fragility of the global software supply chain, especially with respect to open source software. ThreatLabz encourages readers to review the recommendations in this blog to help protect against these kinds of threats and minimize their impacts.

## Zscaler Coverage

Zscaler has added coverage for the threats associated with these campaigns, ensuring that any attempts to download a compromised package will be detected with the following threat names.

### For Axios

[Advanced Threat Protection](https://www.zscaler.com/products-and-solutions/advanced-threat-protection)

- [JS.Malicious.npmpackage](https://threatlibrary.zscaler.com/threats/8bc11290-252a-4df3-aad2-e9b516d02d6d)
- [PS.RAT.npmpackage](https://threatlibrary.zscaler.com/threats/5898a6dc-d990-470d-a942-9d081e417662)
- [Python.RAT.npmpackage](https://threatlibrary.zscaler.com/threats/1aca2017-3d57-47b7-975c-74d6eeec3e01)
- [OSX.RAT.npmpackage](https://threatlibrary.zscaler.com/threats/9c3fa10f-37cb-4ff1-b642-dd5211418a66)

### For LiteLLM

Advanced Threat Protection

- LiteLLM-Z
- ABTrojan.SKMG
- [Python.Trojan.LiteLLM](https://threatlibrary.zscaler.com/threats/0ce921bf-add3-457b-a9f8-a4665430126f)

## Indicators Of Compromise (IOCs)

### For Axios

|

Package

|

Version

|

Hash

|

axios

|

0.30.4

|

e56bafda15a624b60ac967111d227bf8

|

axios

|

1.14.1

|

21d2470cae072cf2d027d473d168158c

|

plain-crypto-js

|

4.2.0

|

52f3311ceb5495796e9bed22302d79bc

|

plain-crypto-js

|

4.2.1

|

db7f4c82c732e8b107492cae419740ab

|

@shadanai/openclaw

|

2026.3.31-1

|

1b8615b9732833b4dd0a3e82326982fa

|

@qqbrowser/openclaw-qbot

|

0.0.130

|

759e597c3cc23c04cd39301bd93fc79f

|

setup.js

|

-

|

7658962ae060a222c0058cd4e979bfa1

|

osx script

|

-

|

7a9ddef00f69477b96252ca234fcbeeb

|

python script

|

-

|

9663665850cdd8fe12e30a671e5c4e6f

|

powershell script

|

-

|

04e3073b3cd5c5bfcde6f575ecf6e8c1

|

system.bat

|

-

|

089e2872016f75a5223b5e02c184dfec

## For LiteLLM

### File hashes

|

MD5 Hash

|

Name

|

cde4951bee7e28ac8a29d33d34a41ae5

|

litellm_init.pth

|

f5560871f6002982a6a2cc0b3ee739f7

|

proxy_server.py

|

7cac57b2d328bd814009772dd1eda429

|

p.py

|

85ed77a21b88cae721f369fa6b7bbba3

|

LiteLLM v1.82.7 Package

|

2e3a4412a7a487b32c5715167c755d08

|

LiteLLM v1.82.8 Package

###
Network indicators

|

Indicator

|

Type

|

checkmarx[.]zone

|

C2 polling

|

models[.]litellm[.]cloud

|

Exfiltration URL

## FAQs

The compromised versions are [[email protected]](https://www.zscaler.com/cdn-cgi/l/email-protection) and [[email protected]](https://www.zscaler.com/cdn-cgi/l/email-protection). Both inject a malicious dependency, [[email protected]](https://www.zscaler.com/cdn-cgi/l/email-protection), that executes a RAT dropper via a postinstall script. Immediate steps: check your package.json, package-lock.json, and yarn.lock for these versions; downgrade to [[email protected]](https://www.zscaler.com/cdn-cgi/l/email-protection) (for 1.x users) or [[email protected]](https://www.zscaler.com/cdn-cgi/l/email-protection) (for 0.x users); clear your npm cache and reinstall; search for connections to sfrclak[.]com or 142.11.206[.]73 from developer workstations and CI/CD systems; and treat any impacted systems as potentially compromised.

TeamPCP poisoned LiteLLM versions 1.82.7 and 1.82.8 on PyPI with malicious code designed to harvest high-value developer and infrastructure secrets: AWS, GCP, and Azure tokens, SSH keys, and Kubernetes credentials. Version 1.82.7 embedded an obfuscated Base64-encoded payload in proxy_server.py that executes on import. Version 1.82.8 added a .pth file (litellm_init.pth) that Python automatically executes at startup — even without importing LiteLLM. Both versions passed pip's hash verification because the malicious code was published with legitimate maintainer credentials. Upgrade to v1.82.6 immediately.

The threat actor bypassed Axios's GitHub Actions CI/CD pipeline entirely by targeting the NPM account directly — not the GitHub repository. By compromising the lead maintainer's NPM account and changing its associated email address, the attacker gained direct publishing rights to the NPM registry. This allowed them to publish the malicious versions via NPM CLI without triggering GitHub Actions workflows or repository-level protections. This attack demonstrates that even projects with robust CI/CD pipeline security remain vulnerable if NPM account credentials are compromised independently.

The .pth file technique in v1.82.8 is especially dangerous for three reasons: (1) Python automatically executes code in .pth files during interpreter startup — meaning the malicious payload triggers on any Python invocation on the host, even if LiteLLM is never imported or used; (2) the .pth file is correctly recorded in the wheel's RECORD file, so pip's hash verification passes — standard integrity checks do not catch it; and (3) the malicious code activates across the entire Python environment, not just processes that use LiteLLM, maximizing the credential harvesting scope across all Python-based workloads on the host.

TeamPCP is a threat group associated with multiple software supply chain attacks in early 2026 targeting open-source security and developer infrastructure tools. In addition to the LiteLLM PyPI attack, TeamPCP has been attributed to attacks on Trivy (Aqua Security's open-source vulnerability scanner), KICS (a static analysis tool for infrastructure-as-code), and Telnyx (a real-time communication library). The group's targeting pattern suggests a deliberate focus on tools used in DevSecOps and AI infrastructure pipelines — maximizing access to CI/CD systems, cloud credentials, and AI model infrastructure.

Thank you for reading

## Was this post useful?

Yes, very!Not really

Disclaimer: This blog post has been created by Zscaler for informational purposes only and is provided "as is" without any guarantees of accuracy, completeness or reliability. Zscaler assumes no responsibility for any errors or omissions or for any actions taken based on the information provided. Any third-party websites or resources linked in this blog post are provided for convenience only, and Zscaler is not responsible for their content or practices. All content is subject to change without notice. By accessing this blog, you agree to these terms and acknowledge your sole responsibility to verify and use the information as appropriate for your needs.

## Explore more Zscaler blogs

Anthropic Claude Code Leak

[Read post](https://www.zscaler.com/blogs/security-research/anthropic-claude-code-leak)

Malicious NPM Packages Deliver NodeCordRAT

[Read post](https://www.zscaler.com/blogs/security-research/malicious-npm-packages-deliver-nodecordrat)

Mitigating Risks from the Shai-Hulud NPM Worm

[Read post](https://www.zscaler.com/blogs/security-research/mitigating-risks-shai-hulud-npm-worm)

Shai-Hulud V2 Poses Risk To NPM Supply Chain

[Read post](https://www.zscaler.com/blogs/security-research/shai-hulud-v2-poses-risk-npm-supply-chain)

Malicious PyPI Packages Deliver SilentSync RAT

[Read post](https://www.zscaler.com/blogs/security-research/malicious-pypi-packages-deliver-silentsync-rat)

01 / 03

## Get the latest Zscaler blog updates in your inbox

By submitting the form, you are agreeing to our [privacy policy](https://www.zscaler.com/privacy-compliance/company-privacy-policy).
