---
url: "https://www.kaspersky.com/blog/critical-supply-chain-attack-trivy-litellm-checkmarx/55510/"
title: "Trojanization of Trivy, Checkmarx, and LiteLLM solutions"
site: "Kaspersky official blog"
captured: "2026-11-02T18:57:09Z"
via: "live"
source_url: "https://www.kaspersky.com/blog/critical-supply-chain-attack-trivy-litellm-checkmarx-teampcp/55510/"
sha256: "191642e1967de06a2cc7eda4b430a2fc4201ce46af9002d36e4da332e59b9402"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Trojanization of Trivy, Checkmarx, and LiteLLM solutions

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.kaspersky.com/blog/critical-supply-chain-attack-trivy-litellm-checkmarx/55510/>

- [supply chain](https://www.kaspersky.com/blog/tag/supply-chain/)

# Supply chain attack via Trivy and LiteLLM

How open-source security solutions became the starting point for a massive attack on other popular applications, and what organizations that use them should do.

-

https://www.kaspersky.com/blog/author/kasperskyteam/

[Kaspersky Team](https://www.kaspersky.com/blog/author/kasperskyteam/)

-

March 25, 2026

Millions of automated software development pipelines rely on security tools — such as Trivy and Checkmarx AST — integrated into the build process. And it was namely these trusted solutions that recently became the entry point for one of the largest and most dangerous supply chain attacks in modern history. In this post we discuss how to audit automated workflows and secure corporate cloud infrastructure.

## Timeline of the attack and known consequences

On March 19, a successful targeted supply chain attack was carried out via Trivy, an open-source vulnerability scanning tool widely used in CI/CD pipelines. The attackers — a group known as TeamPCP — managed to inject malware into official GitHub Actions workflows and Docker images associated with Trivy. As a result, every automated pipeline scan made triggered malware that stole SSH keys, cloud access tokens, cryptocurrency wallets, and other valuable data from compromised systems. Given the critical nature of the incident, it was assigned the identifier [CVE-2026-33634](https://nvd.nist.gov/vuln/detail/CVE-2026-33634), with a near-maximum CVSS4B score of 9.4.

Later that same day, the Trivy team detected the attack and removed malicious artifacts from the distribution channels, halting this phase of the attack. However, the attackers had already gained access to the environments of many Trivy users.

On March 23, a [similar incident](https://www.sysdig.com/blog/teampcp-expands-supply-chain-compromise-spreads-from-trivy-to-checkmarx-github-actions) was discovered in another application security tool: a GitHub Action for Checkmarx KICS, as well as Checkmarx AST. Three hours later, the malicious code was removed from there as well. TeamPCP also managed to compromise [OpenVSX extensions](https://x.com/ReversingLabs/status/2036193573796978729?s=20) supported by Checkmarx: cx-dev-assist 1.7.0 and ast-results. Reports on when this part of the incident was resolved vary.

On March 24, a popular project using Trivy’s code scanning — the LiteLLM AI gateway, a universal library for access to various LLM providers — was attacked. Versions 1.82.7 and 1.82.8, uploaded to PyPI repository, were compromised. These versions were publicly available for about five hours.

But the fact that the attack lasted only a few hours is no reason to dismiss it. Given the popularity of the affected projects, the malicious code could have been executed thousands of times — including within the infrastructure of very large companies.

This allowed attackers to deploy persistent backdoors in Kubernetes clusters, as well as launch the self-replicating [CanisterWorm](https://www.stepsecurity.io/blog/canisterworm-how-a-self-propagating-npm-worm-is-spreading-backdoors-across-the-ecosystem) across the JavaScript npm ecosystem.

The attackers’ code [has destructive capabilities](https://www.aikido.dev/blog/teampcp-stage-payload-canisterworm-iran) that wipe out a Kubernetes cluster and all its nodes if it detects either Tehran’s time zone, or Farsi as the primary language on the compromised system. In other regions, the malware simply steals data using CanisterWorm.

According to experts, more than 20,000 repositories are considered potentially vulnerable. The attackers claim to have stolen hundreds of gigabytes of data and [more than half a million accounts](https://www.bleepingcomputer.com/news/security/popular-litellm-pypi-package-compromised-in-teampcp-supply-chain-attack/).

## How Trivy Was Attacked

To compromise Trivy, the attackers used credentials stolen in a previous incident. The [previous Trivy compromise](https://cybernews.com/security/claude-powered-ai-bot-compromises-five-github-repositories/), which occurred in late February, was likely not fully contained, and the attackers — the same TeamPCP group — returned with a new attack. Trivy’s developers, Aqua Security, [speculate](https://github.com/aquasecurity/trivy/discussions/10425) that because credentials were being phased out gradually following the previous incident, the attackers were able to generate new access tokens for themselves before compromised old ones had been revoked.

As a result, TeamPCP was able to compromise GitHub Actions used in CI/CD pipelines. Using credentials with tag-writing privileges, the attackers forcibly overrode 76 out of 77 version tags in aquasecurity/trivy-action, and all seven tags in aquasecurity/setup-trivy, redirecting existing trusted versions to malicious commits. This resembles tactics observed in the [Shai-Hulud 2.0 campaign](https://securelist.com/shai-hulud-2-0/118214/). As a result, workflows throughout the pipeline began executing the attackers’ code, while the release metadata showed no visible changes.

At the same time, the attackers published an infected Trivy binary (v0.69.4) to official distribution channels, including GitHub Releases and container registries.

## LiteLLM Compromise

The compromise of the popular language-model access tool LiteLLM could itself trigger a major wave of attacks across the chain of projects that use it. The attack took place on March 24, 2026, when TeamPCP directly published malicious versions of the library (1.82.7 and 1.82.8) on PyPI. Between 10:39 UTC and 16:00 UTC, these compromised packages contained malware that stole credentials. It was embedded in the proxy_server.py file, and version 1.82.8 also contained a malicious litellm_init file. The stolen data was exfiltrated to the server models.litellm[.]cloud.

Customers using LiteLLM Cloud or the official LiteLLM Proxy Docker image were not affected due to strict version locking, whereas developers and downstream projects that installed unpinned versions via pip during the specified time window were compromised.

Within three hours, the malicious packages were removed from the PyPI repository, and the LiteLLM team suspended new releases, rotated credentials, and engaged an external incident response process. Teams that use LiteLLM in their projects are advised to immediately check for the litellm_init.pth compromise indicator, and routinely rotate all potentially compromised secrets.

## Features of the TeamPCP Cloud Stealer malware

Attackers added new logic to GitHub Actions and the Trivy executable while preserving the original functionality. Vulnerability scan results via Trivy appeared normal, but at the same time valuable data was being searched for and extracted. Malicious code was doing the following:

- performing reconnaissance (collecting network data and environment variables);
- searching for tokens and credentials to access AWS and GCP cloud environments;
- scanning memory (/proc/*/mem) to extract secrets stored in the memory of Runner.Worker and Runner.Listener processes;
- extracting Kubernetes secrets (/run/secrets/kubernetes.io/serviceaccount);
- collecting data for connecting to database servers (MySQL, PostgreSQL, MongoDB, Redis, Vault);
- collecting any other API keys and secrets from environment files and CI/CD configuration files (.env, .json, .yml);
- searching for webhooks for Slack and Discord channels;
- searching for data related to crypto wallets (variables related to the Solana blockchain, as well as rpcuser and rpcpassword data).

The collected data was encrypted and uploaded to a server with a name similar to the that of the Trivy’s developers (scan.aquasecurtiy[.]org). As a backup mechanism, the attackers provided a method for uploading data to a repository named docs-tpcp.

The attack on CheckMarx and LiteLLM used a similar tactic with other typosquatting domains: models.litellm[.]cloud and checkmarx[.]zone.

A detailed technical analysis of the malware, along with indicators of compromise, can be found in our expert’s article [on the Securelist blog](https://securelist.com/litellm-supply-chain-attack/119257/).

## Response and Defense Strategies for CVE-2026-33634

Existing signature-based checks and dependency scanning in public registries are no longer sufficient, as the malicious code was injected directly into trusted, signed actions, and evaded detection until behavioral monitoring was applied. CI/CD pipelines have become the “new perimeter” of security.

Immediate Actions. Ensure that all workflows use secure versions (Trivy binary 0.69.3, trivy-action 0.35.0, setup-trivy 0.2.6).

CI/CD pipeline administrators and security teams should immediately review their dependances to Checkmarx (kics-github-action, ast-github-action) and Trivy (setup-trivy and trivy-action) solutions. If workflows referenced a version tag rather than a specific SHA hash, carefully review your workflow execution logs for the duration of the active supply chain attack.

You should also check your network logs for traffic to the domains scan.aquasecurtiy[.]org, checkmarx[.]zone, and models.litellm[.]cloud. The presence of such traffic indicates that sensitive data has been successfully exfiltrated.

If a repository named docs-tpcp has appeared on organization’s GitHub, this may also indicate a successful data breach.

In any case, a [proactive threat hunt](https://www.kaspersky.com/enterprise-security/compromise-assessment?icid=gl_kdailyplacehold_acq_ona_smm__onl_b2b_kasperskydaily_wpplaceholder____) should be conducted, assuming that the systems have been successfully compromised and that the attackers have rapidly advanced within the affected systems.

It’s recommended to restore the affected environments from verified backups.

Check hosts and clusters for signs of compromise – the presence of ~/.config/sysmon/sysmon.py files, suspicious pods in Kubernetes.

Clear the cache and conduct an inventory of PyPI modules: check for malicious ones and roll back to clean versions.

Dependency pinning and secret management. Ensure that exact dependency versions are pinned using cryptographic hashes in all pipelines and Dockerfiles. We advise transitioning from long-lived tokens to short-lived credentials by using a secrets manager tool, and implementing OIDC integrations where supported. Minimize the injection of secrets into the runtime environment — do so only when it’s absolutely necessary. Ensure that secrets are not stored on disk or in temporary files, and are not reused across different processes.

Rotate all potentially compromised credentials – API keys, environment variables, SSH keys, Kubernetes service account tokens, and other secrets.

Other security measures. Allow only GitHub Actions from a list approved by the organization; block new and unverified processes. Configure GITHUB_TOKEN and other access keys in accordance with the principle of least privilege. Don’t grant write permissions unless absolutely necessary.

To enhance the security of GitHub Actions, there are several open-source tools available:

- zizmor — a tool for static analysis and detection of configuration errors in GitHub Actions;
- gato and Gato-X — two versions of a tool that helps identify structurally vulnerable pipelines;
- allstar — a GitHub application, developed by OpenSSF, to configure and enforce security policies in GitHub organizations and repositories.

If you want to learn more about supply chain attacks, we invite you to look at our analytical report [Supply chain reaction: securing the global digital ecosystem in an age of interdependence](https://kas.pr/k8rs). It’s based on insights from technical experts, and reveals how often organizations face supply chain and trusted relationship risks, where protection gaps remain, and what strategies to employ to improve resilience against these kinds of threats.

https://www.kaspersky.com/blog/bubble-no-code-phishing/55488/

- [Read next](https://www.kaspersky.com/blog/bubble-no-code-phishing/55488/)

### [Bubble: a new tool for phishing scams](https://www.kaspersky.com/blog/bubble-no-code-phishing/55488/)

Cybercriminals are now deploying web applications generated by Bubble, an AI-powered app builder, to hunt for corporate credentials.

https://www.kaspersky.com/blog/bubble-no-code-phishing/55488/

- [Tips](https://www.kaspersky.com/blog/tag/tips/)

### [Reading between the lines: how to read e-books safely](https://www.kaspersky.com/blog/how-to-download-and-read-ebooks-safely/56625/)

We break down how to use e-books safely, and which rules to follow when searching for books online, so you don’t fall victim to scammers

- [Tips](https://www.kaspersky.com/blog/tag/tips/)

### [The eMudhra certificate incident: how organizations can respond](https://www.kaspersky.com/blog/emudhra-tls-revocation-enterprise-risks-mitigations/56631/)

How a certificate’s private key ended up being equal to 4, and why browser updates alone won’t protect you from vulnerable eMudhra certificates.

- [Tips](https://www.kaspersky.com/blog/tag/tips/)

### [How to improve your resume with AI without exposing your data](https://www.kaspersky.com/blog/creating-resume-with-ai-safely/56563/)

Let’s look at how to use AI as a resume editor without oversharing to build a document that gets past automated screening systems and catches a recruiter’s eye.

- [Tips](https://www.kaspersky.com/blog/tag/tips/)

### [How can students overcome the fear of unfulfilled potential, and where does cybersecurity come into it?](https://www.kaspersky.com/blog/tech-students-future/56475/)

University students sometimes don’t see the practical value of what they’re studying, and aren’t always sure what to do next. Kaspersky Academy’s free “Cybersecurity. Entry Level” course can help them pick up in-demand skills directly from Kaspersky experts.
