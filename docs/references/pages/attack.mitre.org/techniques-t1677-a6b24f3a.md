---
url: "https://attack.mitre.org/techniques/T1677/"
title: "Poisoned Pipeline Execution, Technique T1677 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T19:04:39Z"
via: "live"
sha256: "c030b030b0f79ac29045cb6f44a16b8a12780c11151fa99f9cac1fa8d012aaa7"
wayback: "https://web.archive.org/web/20261023090733/https://attack.mitre.org/techniques/T1677/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Poisoned Pipeline Execution, Technique T1677 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1677/>
> Snapshot: <https://web.archive.org/web/20261023090733/https://attack.mitre.org/techniques/T1677/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Poisoned Pipeline Execution

#  Poisoned Pipeline Execution

Adversaries may manipulate continuous integration / continuous development (CI/CD) processes by injecting malicious code into the build process. There are several mechanisms for poisoning pipelines:

- In a Direct Pipeline Execution scenario, the threat actor directly modifies the CI configuration file (e.g., gitlab-ci.yml in GitLab). They may include a command to exfiltrate credentials leveraged in the build process to a remote server, or to export them as a workflow artifact.[[1]](https://unit42.paloaltonetworks.com/github-actions-supply-chain-attack)[[2]](https://owasp.org/www-project-top-10-ci-cd-security-risks/CICD-SEC-04-Poisoned-Pipeline-Execution)
- In an Indirect Pipeline Execution scenario, the threat actor injects malicious code into files referenced by the CI configuration file. These may include makefiles, scripts, unit tests, and linters.[[2]](https://owasp.org/www-project-top-10-ci-cd-security-risks/CICD-SEC-04-Poisoned-Pipeline-Execution)
- In a Public Pipeline Execution scenario, the threat actor does not have direct access to the repository but instead creates a malicious pull request from a fork that triggers a part of the CI/CD pipeline. For example, in GitHub Actions, the pull_request_target trigger allows workflows running from forked repositories to access secrets. If this trigger is combined with an explicit pull request checkout and a location for a threat actor to insert malicious code (e.g., an npm build command), a threat actor may be able to leak pipeline credentials.[[1]](https://unit42.paloaltonetworks.com/github-actions-supply-chain-attack)[[3]](https://securitylab.github.com/resources/github-actions-preventing-pwn-requests/) Similarly, threat actors may craft pull requests with malicious inputs (such as branch names) if the build pipeline treats those inputs as trusted.[[4]](https://www.wiz.io/blog/ultralytics-ai-library-hacked-via-github-for-cryptomining)[[5]](https://www.synacktiv.com/en/publications/hijacking-github-runners-to-compromise-the-organization)[[6]](https://securitylab.github.com/resources/github-actions-untrusted-input/) Finally, if a pipeline leverages a self-hosted runner, a threat actor may be able to execute arbitrary code on a host inside the organization’s network.[[7]](https://johnstawinski.com/2024/01/11/playing-with-fire-how-we-executed-a-critical-supply-chain-attack-on-pytorch/)

By poisoning CI/CD pipelines, threat actors may be able to gain access to credentials, laterally move to additional hosts, or input malicious components to be shipped further down the pipeline (i.e., [Supply Chain Compromise](https://attack.mitre.org/techniques/T1195)).

ID: T1677

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Execution](https://attack.mitre.org/tactics/TA0002)

ⓘ

Platforms: SaaS

Contributors: Arun Seelagan, CISA

Version: 1.0

Created: 22 May 2025

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1677/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1677/)

## Procedure Examples

| ID  | Name  | Description

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has poisoned npm packages with fake runtime dependencies to trigger execution of malicious payloads on developer endpoints or continuous integration and continuous delivery (CI/CD) systems during npm installation or update.[[8]](https://www.microsoft.com/en-us/security/blog/2026/04/01/mitigating-the-axios-npm-supply-chain-compromise/)[[9]](https://www.microsoft.com/en-us/security/blog/2026/06/17/postinstall-payload-inside-mastra-npm-supply-chain-compromise/)

|  [S9042](https://attack.mitre.org/software/S9042)   |  [CanisterWorm](https://attack.mitre.org/software/S9042)   |

[CanisterWorm](https://attack.mitre.org/software/S9042) has leveraged stolen tokens from Trivy users to publish itself across over 46 npm packages.[[10]](https://www.aikido.dev/blog/telnyx-pypi-compromised-teampcp-canisterworm)[[11]](https://www.aikido.dev/blog/teampcp-deploys-worm-npm-trivy-compromise)

|  [S9043](https://attack.mitre.org/software/S9043)   |  [Mini Shai-Hulud](https://attack.mitre.org/software/S9043)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has utilized Github Actions to propagate through the use of triggered workflows.[[12]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[13]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)[[14]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  [S9008](https://attack.mitre.org/software/S9008)   |  [Shai-Hulud](https://attack.mitre.org/software/S9008)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has also leveraged GitHub actions from stolen accounts in order to create a malicious Github workflow within .github/workflows/discussion.yaml.[[15]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[16]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[17]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[18]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  [G1056](https://attack.mitre.org/groups/G1056)   |  [TeamPCP](https://attack.mitre.org/groups/G1056)   |

[TeamPCP](https://attack.mitre.org/groups/G1056) has compromised trusted CI/CD pipelines by injecting credential-stealing payloads into legitimate workflows and software packages including open-source security tools Trivy and KICS, and AI gateway LiteLLM.[[19]](https://www.wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack)[[20]](https://github.com/aquasecurity/trivy/security/advisories/GHSA-69fq-xp46-6x23)[[20]](https://github.com/aquasecurity/trivy/security/advisories/GHSA-69fq-xp46-6x23)[[21]](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)[[22]](https://www.sysdig.com/blog/teampcp-expands-supply-chain-compromise-spreads-from-trivy-to-checkmarx-github-actions)[[23]](https://www.wiz.io/blog/teampcp-attack-kics-github-action)[[10]](https://www.aikido.dev/blog/telnyx-pypi-compromised-teampcp-canisterworm)[[24]](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)[[24]](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)[[12]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[25]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[26]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[13]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)[[14]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)[[27]](https://www.ic3.gov/CSA/2026/260702.pdf)[[28]](https://cloud.google.com/blog/topics/threat-intelligence/ai-vulnerability-exploitation-initial-access)

## Mitigations

| ID  | Mitigation  | Description

|  [M1054](https://attack.mitre.org/mitigations/M1054)   |  [Software Configuration](https://attack.mitre.org/mitigations/M1054)   |

Where possible, avoid allowing pipelines to run unreviewed code. Where this is necessary, ensure that these pipelines are executed on isolated nodes without access to secrets. In GitHub, avoid using the pull_request_target trigger if possible, do not treat user-controlled inputs (such as branch names) as trusted, and do not use self-hosted runners on public repositories.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Ensure that CI/CD pipelines only have permissions they require to complete their operations. Additionally, limit the number of users who have write access to internal repositories to only those necessary.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0533](https://attack.mitre.org/detectionstrategies/DET0533)   |  [Detection Strategy for Poisoned Pipeline Execution via SaaS CI/CD Workflows](https://attack.mitre.org/detectionstrategies/DET0533)   |  [AN1473](https://attack.mitre.org/detectionstrategies/DET0533#AN1473)   |

Detects anomalous CI/CD workflow execution originating from forked repositories, with pull request (PR) metadata or commit messages containing suspicious patterns (e.g., encoded payloads), coupled with the use of insecure pipeline triggers like pull_request_target or excessive API usage of CI/CD secrets. Correlation with unusual artifact generation or secret exfiltration via encoded or external network destination URLs confirms suspicious behavior.

## References

-    [Omer Gilm Aviad Hahami, Asi Greenholts, and Yaron Avital. (2025, March 20). GitHub Actions Supply Chain Attack: A Targeted Attack on Coinbase Expanded to the Widespread tj-actions/changed-files Incident: Threat Assessment . Retrieved May 22, 2025.](https://unit42.paloaltonetworks.com/github-actions-supply-chain-attack)
-    [OWASP. (n.d.). CICD-SEC-4: Poisoned Pipeline Execution (PPE). Retrieved May 22, 2025.](https://owasp.org/www-project-top-10-ci-cd-security-risks/CICD-SEC-04-Poisoned-Pipeline-Execution)
-    [Jaroslav Lobačevski. (2021, August 3). Keeping your GitHub Actions and workflows secure Part 1: Preventing pwn requests. Retrieved May 22, 2025.](https://securitylab.github.com/resources/github-actions-preventing-pwn-requests/)
-    [Wiz Threat Research. (2024, December 9). Ultralytics AI Library Hacked via GitHub for Cryptomining. Retrieved May 22, 2025.](https://www.wiz.io/blog/ultralytics-ai-library-hacked-via-github-for-cryptomining)
-    [Hugo Vincent. (2024, May 22). Hijacking GitHub runners to compromise the organization. Retrieved May 22, 2025.](https://www.synacktiv.com/en/publications/hijacking-github-runners-to-compromise-the-organization)
-    [Jaroslav Lobačevski. (2021, August 4). Keeping your GitHub Actions and workflows secure Part 2: Untrusted input. Retrieved May 22, 2025.](https://securitylab.github.com/resources/github-actions-untrusted-input/)
-    [John Stawinski IV. (2024, January 11). Playing with Fire – How We Executed a Critical Supply Chain Attack on PyTorch. Retrieved May 22, 2025.](https://johnstawinski.com/2024/01/11/playing-with-fire-how-we-executed-a-critical-supply-chain-attack-on-pytorch/)
-    [Microsoft Threat Intelligence. (2026, April 1). Mitigating the Axios npm supply chain compromise. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/04/01/mitigating-the-axios-npm-supply-chain-compromise/)
-    [Microsoft Threat Intelligence. (2026, June 17). From package to postinstall payload: Inside the Mastra npm supply chain compromise by Sapphire Sleet. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/06/17/postinstall-payload-inside-mastra-npm-supply-chain-compromise/)
-    [Eriksen, C. (2026, March 27). Popular telnyx package compromised on PyPI by TeamPCP. Retrieved July 16, 2026.](https://www.aikido.dev/blog/telnyx-pypi-compromised-teampcp-canisterworm)
-    [Eriksen, C. (2026, March 20). TeamPCP deploys CanisterWorm on NPM following Trivy compromise. Retrieved July 27, 2026.](https://www.aikido.dev/blog/teampcp-deploys-worm-npm-trivy-compromise)
-    [McCarthy, R., Cohen, A., and Read, B. (2026, May 12). Mini Shai-Hulud Strikes Again: TanStack + more npm Packages Compromised. Retrieved July 16, 2026.](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)
-    [Webb, M. (2026, May 20). TeamPCP Wave Four: GitHub Breach via Poisoned VS Code Extension, durabletask PyPI Worm, and ~4,000 Internal Repositories Exfiltrated. Retrieved July 16, 2026.](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)
-    [Flashpoint. (2026, May 28). The Mini Shai-Hulud Worm and the New Era of CI/CD Exploitation. Retrieved July 16, 2026.](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

-    [Charlie Eriksen. (2025, September 16). S1ngularity/nx attackers strike again. Retrieved April 9, 2026.](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)
-    [Gianpietro Cutolo. (2025, November 26). Shai-Hulud 2.0: Aggressive, Automated, and Fast Spreading. Retrieved April 9, 2026.](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)
-    [Justin Moore. (2025, November 25). "Shai-Hulud" Worm Compromises npm Ecosystem in Supply Chain Attack (Updated November 26). Retrieved April 9, 2026.](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)
-    [Socket Research Team. (2025, November 24). Shai Hulud Strikes Again (v2). Retrieved April 9, 2026.](https://socket.dev/blog/shai-hulud-strikes-again-v2)
-    [McCarthy, R. (2026, March 20). Trivy Compromised: Everything You Need to Know about the Latest Supply Chain Attack. Retrieved July 1, 2026.](https://www.wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack)
-    [Aqua Security . (2026, March 21). Trivy ecosystem supply chain temporarily compromised. Retrieved July 1, 2026.](https://github.com/aquasecurity/trivy/security/advisories/GHSA-69fq-xp46-6x23)
-    [Aqua Team. (2026, April 1). Update: Ongoing Investigation and Continued Remediation. Retrieved July 1, 2026.](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)
-    [Sysdig Threat Research Team. (2026, March 23). TeamPCP expands: Supply chain compromise spreads from Trivy to Checkmarx GitHub Actions. Retrieved July 1, 2026.](https://www.sysdig.com/blog/teampcp-expands-supply-chain-compromise-spreads-from-trivy-to-checkmarx-github-actions)
-    [McCarthy, R., Haughom, J., Read, B. (2026, March 23). KICS GitHub Action Compromised: TeamPCP Strikes Again in Supply Chain Attack. Retrieved July 1, 2026.](https://www.wiz.io/blog/teampcp-attack-kics-github-action)
-    [Unit 42. (2026, March 31). Weaponizing the Protectors: TeamPCP’s Multi-Stage Supply Chain Attack on Security Infrastructure. Retrieved July 1, 2026.](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)
-    [Santos, J. and Navato, J.R. (2026, May 13). Analyzing TeamPCP’s Supply Chain Attacks: Checkmarx KICS and elementary-data in CI/CD Credential Theft. Retrieved July 16, 2026.](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)
-    [Hunt.io. (2026, May 14). How TeamPCP's Python Toolkit Survives a C2 Takedown: FIRESCALE, GitHub, and the Victim's Own Account. Retrieved July 16, 2026.](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)
-    [FBI. (2026, July 2). Cyber Criminal Group TeamPCP. Retrieved July 7, 2026.](https://www.ic3.gov/CSA/2026/260702.pdf)
-    [Google Threat Intelligence Group. (2026, May 11). GTIG AI Threat Tracker: Adversaries Leverage AI for Vulnerability Exploitation, Augmented Operations, and Initial Access. Retrieved July 7, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/ai-vulnerability-exploitation-initial-access)

×
