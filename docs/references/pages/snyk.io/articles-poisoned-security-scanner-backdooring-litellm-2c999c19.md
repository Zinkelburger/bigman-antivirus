---
url: "https://snyk.io/articles/poisoned-security-scanner-backdooring-litellm"
title: "How a Poisoned Security Scanner Became the Key to Backdooring LiteLLM | Snyk"
site: "Snyk"
published: "2026-03-24T00:00-04:00"
captured: "2026-11-02T18:44:13Z"
via: "live"
source_url: "https://snyk.io/blog/poisoned-security-scanner-backdooring-litellm/"
sha256: "107b8389c96a7a0f946d9a41ed13a8e2106a6d0fcfa175de2ea49eb6a5549c9b"
wayback: "https://web.archive.org/web/20260331122224/https://snyk.io/articles/poisoned-security-scanner-backdooring-litellm/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# How a Poisoned Security Scanner Became the Key to Backdooring LiteLLM | Snyk

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://snyk.io/articles/poisoned-security-scanner-backdooring-litellm>
> Snapshot: <https://web.archive.org/web/20260331122224/https://snyk.io/articles/poisoned-security-scanner-backdooring-litellm/>

# How a Poisoned Security Scanner Became the Key to Backdooring LiteLLM

Written by

[Stephen Thoemmes](https://snyk.io/contributors/stephen-thoemmes/)

March 24, 2026

0 mins read

On March 24, 2026, two versions of the litellm Python package on PyPI were found to contain malicious code. The packages (versions 1.82.7 and 1.82.8) were published by a threat actor known as TeamPCP after they obtained the maintainer's PyPI credentials through a prior compromise of Trivy, an open source security scanner used in LiteLLM's CI/CD pipeline.

The malicious versions were available for approximately three hours before PyPI quarantined the package. LiteLLM is downloaded roughly 3.4 million times per day.

Snyk has been tracking this incident. If you're a Snyk customer, you may have already seen the in-app banner alert and received an email notification. The vulnerability record is [SNYK-PYTHON-LITELLM-15762713](https://security.snyk.io/vuln/SNYK-PYTHON-LITELLM-15762713), and status updates are on the [Snyk Trust Center](https://trust.snyk.io/updates).

## TL;DR

|

Affected package

|

litellm (PyPI)

|

Affected versions

|

1.82.7, 1.82.8

|

Safe versions

|

≤ 1.82.6

|

Snyk ID

|

[SNYK-PYTHON-LITELLM-15762713](https://security.snyk.io/vuln/SNYK-PYTHON-LITELLM-15762713)

|

First detected

|

10:39 UTC, March 24, 2026 (1.82.7 upload)

|

PyPI quarantine

|

\~13:38 UTC, March 24, 2026

|

Attacker

|

TeamPCP (also: PCPcat, Persy_PCP, ShellForce, DeadCatx3)

|

Attack vector

|

Supply chain: compromised PyPI publisher credentials via poisoned Trivy GitHub Action in LiteLLM CI/CD

|

Payload type

|

Three-stage: credential harvester + encrypted exfiltration + persistent backdoor + Kubernetes worm

|

Exfiltration domain

|

models.litellm.cloud (registered March 23, 2026)

|

MITRE ATT\&CK

|

T1546.018 (Python Startup Hooks), T1003 (Credential Dumping), T1610 (Deploy Container)

## Leading events

|

Time (UTC)

|

Evidence

|

Event

|

Late Feb 2026

|

[GHSA-9p44-j4g5-cfx5](https://github.com/aquasecurity/trivy-action/security/advisories/GHSA-9p44-j4g5-cfx5)

|

MegaGame10418 Pwn Request against Trivy's CI exploits a pull_request_target workflow to exfiltrate the aqua-bot credentials

|

Mar 19, 17:43 UTC

|

[Snyk Trivy coverage](https://snyk.io/articles/trivy-github-actions-supply-chain-compromise/)

|

Trivy v0.69.4 GitHub Action tags rewritten to point to a malicious release

|

Mar 23, 12:58 UTC

|

[Endor Labs](https://www.endorlabs.com/learn/teampcp-isnt-done) (captured pre-deletion PyPI metadata)

|

Checkmarx KICS GitHub Action compromised; checkmarx.zone C2 domain and models.litellm.cloud registered

|

Mar 24, 10:39 UTC

|

[Endor Labs](https://www.endorlabs.com/learn/teampcp-isnt-done) (captured pre-deletion PyPI metadata)

|

Malicious litellm 1.82.7 published to PyPI

|

Mar 24, 10:52 UTC

|

[FutureSearch](https://futuresearch.ai/blog/litellm-pypi-supply-chain-attack/); [Endor Labs](https://www.endorlabs.com/learn/teampcp-isnt-done)

|

Malicious litellm 1.82.8 published to PyPI (13 minutes after 1.82.7, with escalated .pth delivery mechanism)

|

Mar 24, 11:48 UTC

|

[GitHub issue #24512](https://github.com/BerriAI/litellm/issues/24512)

|

FutureSearch (Callum McMahon) opens disclosure issue

|

Mar 24, 12:36 UTC

|

[Hacker News](https://news.ycombinator.com/item?id=47501729)

|

HN thread posted; reaches 324 points

|

Mar 24, \~12:44 UTC

|

[GitHub issue #24512](https://github.com/BerriAI/litellm/issues/24512) (visible in comment timestamps)

|

Bot comments flood issue #24512; issue closed using the compromised maintainer account

|

Mar 24, 13:03 UTC

|

[FutureSearch](https://futuresearch.ai/blog/litellm-pypi-supply-chain-attack/) (timestamped update)

|

FutureSearch confirms issue closure and bot spam

|

Mar 24, 13:48 UTC

|

[GitHub issue #24518](https://github.com/BerriAI/litellm/issues/24518)

|

Clean tracking issue opened

|

Mar 24, 15:09 UTC

|

[GitHub comment](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4119055562)

|

LiteLLM maintainer confirms all GitHub, Docker, and PyPI keys rotated; maintainer accounts moved to new identities

|

Mar 24, 15:27 UTC

|

[GitHub comment](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4119189850)

|

Compromised versions deleted; package unquarantined on PyPI

## How it was discovered

Callum McMahon at FutureSearch was testing a Cursor MCP plugin that pulled in litellm as a transitive dependency. Shortly after Python started, his machine became unresponsive due to RAM exhaustion. He traced it to the newly installed litellm package and found litellm_init.pth, a 34,628-byte file in site-packages/, double base64-encoded.

The RAM exhaustion was a side effect of the payload, not an intentional feature. The .pth mechanism fires on every Python interpreter startup. Because the payload spawns a new Python subprocess, and that new process also triggers .pth execution, the result was an unintended fork bomb. McMahon published his findings on futuresearch.ai, and the disclosure spread to r/LocalLLaMA, r/Python, and Hacker News within the hour.

## The attack chain

The attack on LiteLLM started five days earlier with Trivy.

March 19: The attackers rewrote Git tags in the trivy-action GitHub Action repository to point to a malicious release (v0.69.4) carrying the same credential-harvesting payload and exfiltration infrastructure used in later operations. (For full details on the Trivy compromise, see [Snyk's Trivy GitHub Actions supply chain compromise coverage](https://snyk.io/articles/trivy-github-actions-supply-chain-compromise/).)

March 23: The same infrastructure was used in a separate attack on Checkmarx KICS (Keep Infrastructure as Code Secure). The C2 domain checkmarx.zone, which impersonates the Checkmarx security company, was registered and activated in this operation.

March 24: LiteLLM's CI/CD pipeline ran Trivy as part of its build process, pulling it from apt without a pinned version. The compromised action exfiltrated the PYPI_PUBLISH token from the GitHub Actions runner environment. With that credential, the attackers published litellm 1.82.7 at 10:39 UTC and 1.82.8 at 10:52 UTC, each containing malicious payloads.

Initial LiteLLM maintainer response on Hacker News.

### Two Delivery Mechanisms

The two malicious versions used different injection techniques:

1.82.7 (source injection): The payload was base64-encoded and embedded directly inside litellm/proxy/proxy_server.py. It [executes when anything imports litellm.proxy](https://socket.dev/pypi/package/litellm/files/1.82.7/tar-gz/litellm-1.82.7/litellm/proxy/proxy_server.py), the standard import path for LiteLLM's proxy server mode.

1.82.8 (.pth file): This version added [litellm_init.pth](https://socket.dev/pypi/package/litellm/files/1.82.8/tar-gz/litellm-1.82.8/litellm_init.pth) to site-packages/. The .pth mechanism fires on every Python interpreter startup with no import required, including when pip, python -c, or an IDE's language server starts Python.

The .pth file is correctly declared in the wheel's RECORD file, so it passes pip's hash verification. The package passes all standard integrity checks because the malicious content was published using legitimate credentials, not injected after the fact. Snyk Learn has a detailed walkthrough of how this class of attack works in [Compromise of a Legitimate Package](https://learn.snyk.io/lesson/compromise-of-legitimate-package/).

CPython maintainers have acknowledged the security risk of .pth startup hooks in [issue #113659](https://github.com/python/cpython/issues/113659), but no patch has been applied. This technique maps to MITRE ATT\&CK [T1546.018 (Python Startup Hooks)](https://attack.mitre.org/techniques/T1546/018/).

## The three-stage payload

### Stage 1: Information collection

The script collects:

-

System data: hostname, environment variables, running processes, network routing, recent auth logs from /var/log/auth.log

-

Credentials: SSH private keys, .env files, Git credentials, API keys from shell history, Slack and Discord webhook tokens, CI/CD configs (Jenkins, Travis CI, Terraform), password hashes from /etc/shadow

-

Cloud credentials: AWS credentials and config, GCP and Azure service account files, cloud metadata via direct IMDS queries, AWS Secrets Manager, and SSM Parameter Store (with full IMDSv2 signing implemented in the script)

-

Container and orchestration: Docker config.json (registry credentials), Kubernetes kubeconfig files, service account tokens, secrets, and certificates

-

Cryptocurrency: Bitcoin, Ethereum, Solana, Cardano, Monero wallet files and seed phrases, Ledger hardware wallet configurations

Portion of the decoded malicious code, showing structured iteration and collection of secrets in various directories

### Stage 2: Encryption and exfiltration

Collected data is encrypted before transmission:

-

A 32-byte AES-256 session key is generated via openssl rand

-

Data is encrypted with AES-256-CBC (PBKDF2 key derivation)

-

The session key is encrypted with a hardcoded 4096-bit RSA public key (OAEP padding)

-

Everything is bundled into tpcp.tar.gz

-

The bundle is POST'd to https://models.litellm.cloud/ via curl

models.litellm.cloud is distinct from the legitimate litellm.ai domain and was registered on March 23, one day before the malicious packages were published.

The RSA public key in this payload is identical to the one found in the Trivy and KICS payloads. [Wiz researchers confirmed this match](https://ramimac.me/trivy-teampcp/) across all three operations, making it the strongest single technical attribution link.

Temporary files created during exfiltration: session.key, payload.enc, session.key.enc, tpcp.tar.gz in the system temp directory.

Defanged view of the malicious code. The tpcp.tar.gz archive name is a consistent marker across the Trivy, KICS, and LiteLLM operations.

### Stage 3: Persistence and lateral movement

Local persistence:

-

Writes a backdoor Python script to ~/.config/sysmon/sysmon.py (or /root/.config/sysmon/sysmon.py)

-

Creates a systemd user service at ~/.config/systemd/user/sysmon.service with the description "System Telemetry Service"

-

Registers and enables it: systemctl --user daemon-reload && systemctl --user enable --now sysmon.service

The persistence script polls https://checkmarx.zone/raw every 5 minutes for a URL, downloads the file to /tmp/pglog, makes it executable, and runs it as a background process. State is tracked in /tmp/.pg_state. At the time of analysis, the endpoint returns YouTube URLs; the operator can switch to delivering live payloads at will.

Kubernetes lateral movement: If the script finds a Kubernetes service account token at the standard mount path, it reads all secrets across every namespace. It then attempts to deploy a privileged pod to every node in kube-system using alpine:latest. These pods mount the host filesystem and install the sysmon backdoor on the underlying node.

Malicious pods are named node-setup-{node_name} (node name truncated to 35 characters), with a container named setup.

## About TeamPCP

TeamPCP (also identified as PCPcat, Persy_PCP, ShellForce, and DeadCatx3 per [Wiz Threat Center](https://threats.wiz.io/all-actors/teampcp)) has been active since at least December 2025. The actor maintains Telegram channels at @Persy_PCP and @teampcp and embeds the string "TeamPCP Cloud stealer" in payloads. Wiz has tracked the full campaign ([Wiz Threat Center](https://threats.wiz.io/all-actors/teampcp); [Wiz blog](https://www.wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack); [ramimac.me](https://ramimac.me/trivy-teampcp/)).

The LiteLLM compromise is Phase 09 of an ongoing campaign. Consistent infrastructure across all operations: same RSA key pair, same tpcp.tar.gz bundle naming, and tpcp-docs-prefixed GitHub repositories used as dead-drop C2 staging. All three domains in this operation share the same registrar (Spaceship, Inc.) and hosting provider (DEMENIN B.V.).

The actor has also deployed CanisterWorm, which uses the Internet Computer Protocol (ICP) as a C2 channel. ICP canisters cannot be taken down by domain registrars or hosting providers. [Aikido security researchers](https://www.aikido.dev/blog/catching-a-canisterworm-uncovering-the-first-ever-icp-based-supply-chain-attack) document this as the first observed use of ICP as a C2 mechanism in a supply chain campaign.

A component called hackerbot-claw uses an AI agent (openclaw) for automated attack targeting. Aikido researchers [documented this](https://www.aikido.dev/blog/catching-a-canisterworm-uncovering-the-first-ever-icp-based-supply-chain-attack) as one of the first cases of an AI agent used operationally in a supply chain attack.

### Issue suppression

When community members began reporting the compromise in [GitHub issue #24512](https://github.com/BerriAI/litellm/issues/24512), the attackers posted 88 bot comments from 73 unique accounts in a 102-second window (12:44-12:46 UTC). The accounts used were previously compromised developer accounts, not purpose-created profiles. [Rami McCarthy's analysis](https://ramimac.me/trivy-teampcp/#phase-09) found 76% account overlap with the botnet used during the Trivy disclosure.

Using the compromised krrishdholakia maintainer account, the attackers closed issue #24512 as "not planned" and made commits to unrelated repositories with the message "teampcp update."

The community opened a parallel tracking issue ([#24518](https://github.com/BerriAI/litellm/issues/24518)) and continued discussion on Hacker News, where [the thread](https://news.ycombinator.com/item?id=47501729) reached 324 points.

## Confirmed impact

The affected versions were on PyPI for approximately 3 hours. The following projects filed security PRs or issues on March 24 to pin away from 1.82.7 and 1.82.8:

|

Project

|

Evidence

|

DSPy

|

[PR #9498](https://github.com/stanfordnlp/dspy/pull/9498) merged; [CI failure report](https://github.com/stanfordnlp/dspy/issues/9500)

|

MLflow

|

[PR #21971](https://github.com/mlflow/mlflow/pull/21971) merged

|

OpenHands

|

[PR #13569](https://github.com/OpenHands/OpenHands/pull/13569)

|

CrewAI

|

[PR #5040](https://github.com/crewAIInc/crewAI/pull/5040) (decoupled from litellm); [PR #5039](https://github.com/crewAIInc/crewAI/pull/5039)

|

langwatch

|

[PR #2577](https://github.com/langwatch/langwatch/pull/2577)

|

strands-agents/sdk-python

|

[PR #1961](https://github.com/strands-agents/sdk-python/pull/1961)

|

Arize Phoenix

|

[PR #12342](https://github.com/Arize-ai/phoenix/pull/12342)

|

nanobot

|

[PR #2441](https://github.com/HKUDS/nanobot/pull/2441)

|

dreadnode/rigging

|

[PR #356](https://github.com/dreadnode/rigging/pull/356)

|

CoPaw

|

[Issue #2209](https://github.com/agentscope-ai/CoPaw/issues/2209)

|

Aider

|

Confirmed safe ([pins litellm==1.82.3](https://github.com/Aider-AI/aider/blob/main/requirements.txt))

The .pth mechanism fires when any Python process starts, including pip itself. In CI/CD environments, this means the payload can execute during build steps, not just at application runtime.

## Detection: Are you affected?

Step 1: Check your installed version

```
pip show litellm | grep Version
```

If the output shows 1.82.7 or 1.82.8, treat the system as compromised and follow the remediation below. Do not just upgrade; the payload may have already run.

Step 2: Check for persistence artifacts

```
# Check for the sysmon backdoor
ls -la ~/.config/sysmon/sysmon.py 2>/dev/null && echo "BACKDOOR FOUND"
ls -la /root/.config/sysmon/sysmon.py 2>/dev/null && echo "ROOT BACKDOOR FOUND"

# Check for the systemd persistence service
systemctl --user status sysmon.service 2>/dev/null
ls -la ~/.config/systemd/user/sysmon.service 2>/dev/null && echo "PERSISTENCE SERVICE FOUND"

# Check for exfiltration archive remnants
ls /tmp/tpcp.tar.gz /tmp/session.key /tmp/payload.enc /tmp/session.key.enc 2>/dev/null && echo "EXFIL ARTIFACTS FOUND"
```

Step 3: Check for malicious .pth files

```
# Find .pth files in site-packages with suspicious patterns
find $(python3 -c "import site; print(' '.join(site.getsitepackages()))") \
  -name "*.pth" -exec grep -l "base64\|subprocess\|exec" {} \;
```

Step 4: Verify file hashes

```
# Check proxy_server.py (1.82.7)
find / -path "*/litellm/proxy/proxy_server.py" 2>/dev/null -exec shasum -a 256 {} \;
# Malicious: a0d229be8efcb2f9135e2ad55ba275b76ddcfeb55fa4370e0a522a5bdee0120b

# Check litellm_init.pth (1.82.8)
find / -name "litellm_init.pth" 2>/dev/null -exec shasum -a 256 {} \;
# Malicious: 71e35aef03099cd1f2d6446734273025a163597de93912df321ef118bf135238
```

Step 5: Check for network indicators

```
grep "litellm.cloud\|checkmarx.zone" /etc/hosts
grep "models.litellm.cloud\|checkmarx.zone" /var/log/syslog 2>/dev/null
```

Step 6: Check Kubernetes

```
kubectl get pods -A | grep "node-setup-"
```

Step 7: Scan with Snyk

```
snyk test --package-manager=pip
```

The Snyk in-app banner alert, with affected repositories surfaced in the asset inventory. The Trust Center link navigates to real-time incident status.

Snyk customers can also check the [litellm package advisor](https://snyk.io/advisor/python-package/litellm) and the full vulnerability record at [SNYK-PYTHON-LITELLM-15762713](https://security.snyk.io/vuln/SNYK-PYTHON-LITELLM-15762713).

## Remediation

If you have NOT installed 1.82.7 or 1.82.8:

Pin to <=1.82.6 until a clean release is available:

```
pip install "litellm<=1.82.6"
# requirements.txt:
litellm<=1.82.6
```

If you HAVE installed 1.82.7 or 1.82.8:

The payload executes when Python starts, including during pip install itself. Treat the system as potentially compromised regardless of whether you ran any application code.

-

Remove persistence artifacts:

```
rm -f ~/.config/sysmon/sysmon.py
rm -f ~/.config/systemd/user/sysmon.service
systemctl --user disable sysmon.service 2>/dev/null
systemctl --user daemon-reload
rm -f /tmp/tpcp.tar.gz /tmp/session.key /tmp/payload.enc /tmp/session.key.enc /tmp/.pg_state /tmp/pglog
```

-

Rotate credentials on the affected system:

-

SSH private keys: generate new keys, revoke old keys from authorized_keys, GitHub, GitLab

-

Cloud credentials: AWS access keys, GCP service account keys, Azure service principals

-

API keys: .env files, shell environment variables, CI/CD secrets

-

Docker registry credentials: ~/.docker/config.json

-

Kubernetes: ~/.kube/config, in-cluster service account tokens

-

Database passwords from any config files on the system

-

Git credentials from ~/.gitconfig or the system credential store

-

Cryptocurrency wallet seed phrases

-

Audit AWS Secrets Manager and SSM Parameter Store, as the payload queries these directly if instance metadata is accessible.

-

Audit Kubernetes cluster secrets. If a service account token was present, all secrets across all namespaces may have been read. Check for node-setup-* pods in kube-system.

-

Install a clean version on a fresh environment rather than upgrading in-place:

```
pip install "litellm<=1.82.6"
```

## Why pip hash verification didn't catch this

Hash verification confirms a file matches what PyPI advertised, but does not indicate whether the advertised content is malicious.

The litellm_init.pth file in 1.82.8 is correctly declared in the wheel's RECORD file with a matching hash. pip install --require-hashes would have passed. The package passes all standard integrity checks because the malicious content was published using legitimate credentials; there is no hash mismatch, no suspicious domains, and no misspelled package name.

The only install-time detection path is inspecting whether a package installs .pth files and whether those files contain patterns like subprocess, base64, or exec. No widely deployed pip plugin currently does this automatically.

## The broader pattern

The target selection across this campaign focuses on tools with elevated access to automated pipelines: a container scanner (Trivy), an infrastructure scanning tool (KICS), and an AI model routing library (LiteLLM). Each of these tools requires broad read access to the systems it operates on (credentials, configs, environment variables) by design.

LiteLLM is increasingly being deployed as a centralized LLM gateway that stores API credentials for multiple model providers. In that configuration, the credential set accessible from a single compromised host is broader than for a typical application.

The initial disclosure spread through AI developer communities (r/LocalLLaMA, r/Python, Hacker News) rather than through traditional security channels such as r/netsec or CVE feeds.

For further reading on supply chain risks specific to LLM tooling, Snyk Learn covers the topic in [Supply Chain Vulnerabilities in LLMs](https://learn.snyk.io/lesson/supply-chain-vulnerabilities-llm/). The [Ultralytics AI Pwn Request supply chain attack](https://snyk.io/blog/ultralytics-ai-pwn-request-supply-chain-attack/) from 2024 is also a useful comparison case: another widely used AI Python library compromised via a CI/CD exploit, with a similar attack chain.

## Indicators of compromise

File hashes:

|

File

|

SHA-256

|

litellm_init.pth (1.82.8)

|

71e35aef03099cd1f2d6446734273025a163597de93912df321ef118bf135238

|

proxy_server.py (1.82.7)

|

a0d229be8efcb2f9135e2ad55ba275b76ddcfeb55fa4370e0a522a5bdee0120b

|

sysmon.py

|

6cf223aea68b0e8031ff68251e30b6017a0513fe152e235c26f248ba1e15c92a

Network:

-

Exfiltration: https://models.litellm.cloud/ (POST)

-

C2 polling: https://checkmarx.zone/raw (GET)

Filesystem:

-

~/.config/sysmon/sysmon.py or /root/.config/sysmon/sysmon.py

-

~/.config/systemd/user/sysmon.service (description: "System Telemetry Service")

-

/tmp/tpcp.tar.gz, /tmp/session.key, /tmp/payload.enc, /tmp/session.key.enc

-

/tmp/.pg_state, /tmp/pglog

Kubernetes:

-

Pods: node-setup-{node_name} in kube-system

-

Container name: setup, image: alpine:latest

RSA public key prefix (hardcoded in all three operations' payloads):

```
MIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAvahaZDo8mucujrT15ry+...
```

## What to do right now

-

Check your litellm version: pip show litellm | grep Version

-

Pin to <=1.82.6 in all environments

-

Run snyk test --package-manager=pip

-

If you had 1.82.7 or 1.82.8 installed: rotate credentials and check for persistence artifacts

-

Audit CI/CD pipelines for unpinned tool versions, including GitHub Actions

-

Check Kubernetes: kubectl get pods -A | grep node-setup-

WHITEPAPER

## The AI Security Crisis in Your Python Environment

As development velocity skyrockets, do you actually know what your AI environment can access?

[Learn more](https://snyk.io/lp/ai-security-crisis-python/)

Blog

[PHP2Shell Won't Be the Last. Here's Where the Next One Will Show Up First.](https://snyk.io/blog/php2shell-where-next-one-will-show-up-first/)

PHP2Shell showed how quickly an engine-level flaw can put internet-facing applications at risk. Learn why outside-in inventory, rapid testing, and repeat checks matter before the next disclosure.

Blog

[Snyk UNFILTERED: Inside Our First-Ever Customer Summit](https://snyk.io/blog/snyk-unfiltered-customer-summit/)

At Snyk’s first customer summit, security leaders shared candid perspectives on AI risk, vulnerability remediation, and governing autonomous agents. Five themes reveal how teams are adapting security for AI speed.

Blog

[PHP Patches Two Critical Unauthenticated RCEs (CVE-2026-109618, CVE-2026-109619): [Updated for CVE-2026-115560]](https://snyk.io/blog/php-patches-critical-unauthenticated-rces/)

Two critical flaws in PHP’s request lifecycle can enable unauthenticated remote code execution. Upgrade to a fixed release or install a vendor backport for older versions.
