---
url: "https://github.com/DataDog/guarddog"
title: "GitHub - DataDog/guarddog: :snake: GuardDog is a CLI tool to Identify malicious PyPI and npm packages"
site: "GitHub"
captured: "2026-11-02T18:36:10Z"
via: "live"
sha256: "f03dbde001806a548c04a9f35042f5f03c833e2bd336a8c4abf43d3154a2da68"
wayback: "https://web.archive.org/web/20261023012629/https://github.com/datadog/guarddog"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# GitHub - DataDog/guarddog: :snake: GuardDog is a CLI tool to Identify malicious PyPI and npm packages

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/DataDog/guarddog>
> Snapshot: <https://web.archive.org/web/20261023012629/https://github.com/datadog/guarddog>

# GuardDog

https://github.com/DataDog/guarddog#guarddog

https://github.com/DataDog/guarddog/actions/workflows/checks.yml https://securityscorecards.dev/viewer/?uri=github.com/DataDog/guarddog https://www.bestpractices.dev/projects/12273

https://github.com/DataDog/guarddog/blob/v3/docs/images/logo.png?raw=true

GuardDog is a CLI tool that identifies malicious PyPI and npm packages, Go modules, Rust crates, RubyGems, GitHub actions, or VSCode extensions. It runs static analysis on package source code (through YARA rules) and analyzes package metadata to detect supply chain attacks.

What makes GuardDog different: Instead of just listing suspicious patterns, GuardDog correlates findings to identify actual risks based on attack chains. A package needs both the capability to perform an action (e.g., network access) and a threat indicator (e.g., suspicious domain) in the same file to be flagged as high risk.

It downloads and scans code from:

- NPM: Packages hosted in [npmjs.org](https://www.npmjs.com/)
- PyPI: Source files (tar.gz) packages hosted in [PyPI.org](https://pypi.org/)
- Go: GoLang source files of repositories hosted in [GitHub.com](https://github.com)
- Rust: Crates hosted on [crates.io](https://crates.io/)
- RubyGems: Gem packages hosted in [rubygems.org](https://rubygems.org/)
- GitHub Actions: Javascript source files of repositories hosted in [GitHub.com](https://github.com)
- VSCode Extensions: Extensions (.vsix) packages hosted in [marketplace.visualstudio.com](https://marketplace.visualstudio.com/)

https://github.com/DataDog/guarddog/blob/v3/docs/images/demo.png?raw=true

## How GuardDog Works

https://github.com/DataDog/guarddog#how-guarddog-works

GuardDog uses a risk-based detection model that correlates code capabilities with threat indicators:

- Detection: Rules identify either capabilities (what code can do) or threats (suspicious indicators)
- Correlation: Capabilities and threats found in the same file form risks (cross-file matches also form risks, with downgraded severity)
- Scoring: Risks are scored (0-10) based on attack chain completeness and sophistication
- Reporting: Packages receive a severity rating (low/medium/high) with detailed risk breakdown

### Why This Approach?

https://github.com/DataDog/guarddog#why-this-approach

Traditional SAST tools flag every suspicious pattern independently, leading to alert fatigue. GuardDog understands that:

- Capability alone isn't malicious (network libraries should make HTTP requests)
- Threat indicators alone might be false positives (test fixtures, documentation)
- Capability + Threat together indicates actual risk (code that can and will do something malicious)

### Risk Scoring

https://github.com/DataDog/guarddog#risk-scoring

Packages receive a score from 0-10 based on four factors:

| Factor  | Weight  | Description

| Severity  | 30%  | Highest severity finding (low/medium/high)

| Attack Chain  | 20%  | Presence of complete attack stages (early → mid/late)

| Specificity  | 30%  | How specific patterns are to malware vs legitimate code

| Sophistication  | 20%  | Technique advancement level

Score Labels:

- 0: No risks detected
- 0.1-3: Low risk (single-stage threats, low specificity)
- 3.1-7.5: Medium risk (partial attack chain, metadata indicators, or single-stage code findings)
- 7.6-10: High risk (multi-stage attack chain with source code evidence — near-certainty of compromise)

Attack Chain Stages (based on MITRE ATT&CK):

- Early: Initial access, execution capabilities
- Mid: Persistence, defense evasion, credential access
- Late: Command & control, exfiltration, impact

### Check out the new Datadog Agent [integration](https://docs.datadoghq.com/integrations/guarddog/) and Cloud SIEM [content pack](https://app.datadoghq.com/security/siem/content-packs?query=guarddog) for GuardDog.

https://github.com/DataDog/guarddog#check-out-the-new-datadog-agent-integration-and-cloud-siem-content-pack-for-guarddog

## Getting started

https://github.com/DataDog/guarddog#getting-started

### Installation

https://github.com/DataDog/guarddog#installation

The easiest way to run GuardDog is to use [uvx](https://docs.astral.sh/uv/guides/tools/):

```
uvx guarddog pypi scan requests
```

To install it locally:

```
uv tool install guarddog
# or
pip install guarddog
```

Or use the Docker image:

```
docker pull ghcr.io/datadog/guarddog
alias guarddog='docker run --rm ghcr.io/datadog/guarddog'
```

Note: On Windows, the only supported installation method is Docker.

### Sample usage

https://github.com/DataDog/guarddog#sample-usage

```
# Scan the most recent version of the 'requests' package
guarddog pypi scan requests

# Scan a specific version of the 'requests' package
guarddog pypi scan requests --version 2.28.1

# Scan the 'request' package using 2 specific heuristics
guarddog pypi scan requests --rules exec-base64 --rules code-execution

# Scan the 'requests' package using all rules but one
guarddog pypi scan requests --exclude-rules exec-base64

# Scan a local package archive
guarddog pypi scan /tmp/triage.tar.gz

# Scan a local package directory
guarddog pypi scan /tmp/triage/

# Scan a package stored in S3 (a folder/prefix or a single archive object)
guarddog pypi scan s3://my-bucket/path/to/package/
guarddog pypi scan s3://my-bucket/path/to/package.tar.gz

# Scan every package referenced in a requirements.txt file of a local folder
guarddog pypi verify workspace/guarddog/requirements.txt

# Scan every package referenced in a requirements.txt file and output a sarif file - works only for verify
guarddog pypi verify --output-format=sarif workspace/guarddog/requirements.txt

# Output JSON to standard output - works for every command
guarddog pypi scan requests --output-format=json

# All the commands also work on npm, go, crates, rubygems
guarddog npm scan express

guarddog go scan github.com/DataDog/dd-trace-go

guarddog go verify /tmp/repo/go.mod

# Scan Rust crates
guarddog crates scan serde

guarddog crates verify /tmp/repo/Cargo.lock

# Scan RubyGems packages
guarddog rubygems scan rails

guarddog rubygems verify /tmp/repo/Gemfile.lock

# Additionally can support scanning GitHub actions that are implemented in JavaScript
guarddog github_action scan DataDog/synthetics-ci-github-action

guarddog github_action verify /tmp/repo/.github/workflows/main.yml

# Scan VSCode extensions from the marketplace
guarddog extension scan ms-python.python

# Scan a specific version of a VSCode extension
guarddog extension scan ms-python.python --version 2023.20.0

# Scan a local VSCode extension directory or VSIX archive
guarddog extension scan /tmp/my-extension/

# Run in debug mode
guarddog --log-level debug npm scan express
```

## Sandboxed Scanning

https://github.com/DataDog/guarddog#sandboxed-scanning

When scanning packages, GuardDog runs source code analysis inside a kernel-level sandbox (Linux via Landlock, macOS via Seatbelt, using [nono](https://github.com/always-further/nono-py)). The sandbox blocks all network access and restricts filesystem operations to only the paths needed for analysis. This protects against malicious packages that attempt to execute code during archive extraction or scanning.

By default, the sandbox is required: if it's not available on the platform, the scan fails instead of running unprotected. To scan without it, you must explicitly pass --no-sandbox:

```
# Default: require the sandbox, exit with an error if it's unavailable
guarddog pypi scan requests

# Explicitly disable the sandbox
guarddog pypi scan requests --no-sandbox
```

For remote packages, three phases run with different privilege levels:

- Download and metadata analysis run without sandbox (need network access)
- Archive extraction runs in a sandboxed subprocess (network blocked, filesystem restricted)
- Source code analysis (YARA) runs in the main process after a sandbox is applied (network blocked, filesystem restricted to extracted files)

The sandbox was introduced to mitigate path traversal and code execution vulnerabilities during archive extraction (CVE-2022-23530, CVE-2022-23531, CVE-2026-22870, CVE-2026-22871).

## Scanning packages from S3

https://github.com/DataDog/guarddog#scanning-packages-from-s3

GuardDog can scan a package stored in S3, either as a folder/prefix or a single archive object:

```
guarddog npm scan s3://my-bucket/path/to/package/
guarddog npm scan s3://my-bucket/path/to/package.tar.gz
```

This uses your existing AWS credentials (environment variables, ~/.aws, SSO, or an IAM role). GuardDog verifies authentication via STS before doing anything and exits with an error if no valid credentials are found. The objects are synced to a temporary directory, scanned under the sandbox like any other untrusted content, and removed from disk afterward.

## Rules

https://github.com/DataDog/guarddog#rules

GuardDog uses two types of detection rules, both participating in the risk-based scoring engine:

- Source code rules (YARA): Static analysis of package source code detecting capabilities and threats
- Metadata rules (Python detectors): Analysis of package registry metadata detecting supply chain attack indicators

For the full list of rules per ecosystem, see [RULES.md](https://github.com/DataDog/guarddog/blob/v3/RULES.md).

For guidance on writing new rules, see [WRITING_RULES.md](https://github.com/DataDog/guarddog/blob/v3/WRITING_RULES.md).

## Running GuardDog in a GitHub Action

https://github.com/DataDog/guarddog#running-guarddog-in-a-github-action

The easiest way to integrate GuardDog in your CI pipeline is to leverage the SARIF output format, and upload it to GitHub's [code scanning](https://docs.github.com/en/code-security/code-scanning/automatically-scanning-your-code-for-vulnerabilities-and-errors/about-code-scanning) feature.

Using this, you get:

- Automated comments to your pull requests based on the GuardDog scan output
- Built-in false positive management directly in the GitHub UI

Sample GitHub Action using GuardDog:

```
name: GuardDog

on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main

permissions:
  contents: read

jobs:
  guarddog:
    permissions:
      contents: read # for actions/checkout to fetch code
      security-events: write # for github/codeql-action/upload-sarif to upload SARIF results
    name: Scan dependencies
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - uses: astral-sh/setup-uv@v7

      - run: uvx guarddog pypi verify requirements.txt --output-format sarif --exclude-rules repository_integrity_mismatch > guarddog.sarif

      - name: Upload SARIF file to GitHub
        uses: github/codeql-action/upload-sarif@v3
        with:
          category: guarddog-builtin
          sarif_file: guarddog.sarif
```

## Development

https://github.com/DataDog/guarddog#development

### Running a local version of GuardDog

https://github.com/DataDog/guarddog#running-a-local-version-of-guarddog

- Ensure poetry has an env with python >=3.10 poetry env use 3.10.0
- Install dependencies poetry install
- Run guarddog poetry run guarddog or poetry shell then run guarddog

### Unit tests

https://github.com/DataDog/guarddog#unit-tests

Running all unit tests: make test

Running unit tests against package metadata heuristics: make test-metadata-rules (tests are [here](https://github.com/DataDog/guarddog/tree/v3/tests/analyzer/metadata)).

### Benchmarking

https://github.com/DataDog/guarddog#benchmarking

You can run GuardDog on legitimate and malicious packages to determine false positives and false negatives. See [./tests/samples](https://github.com/DataDog/guarddog/blob/v3/tests/samples)

### Code quality checks

https://github.com/DataDog/guarddog#code-quality-checks

Run the type checker with

```
mypy --install-types --non-interactive guarddog
```

and the linter with

```
flake8 guarddog --count --select=E9,F63,F7,F82 --show-source --statistics --exclude tests/analyzer/sourcecode,tests/analyzer/metadata/resources,evaluator/data
flake8 guarddog --count --max-line-length=120 --statistics --exclude tests/analyzer/sourcecode,tests/analyzer/metadata/resources,evaluator/data --ignore=E203,W503
```

### Configuration via Environment Variables

https://github.com/DataDog/guarddog#configuration-via-environment-variables

GuardDog's behavior can be customized using environment variables:

#### General Configuration

https://github.com/DataDog/guarddog#general-configuration

| Environment Variable  | Description  | Default Value

| GUARDDOG_PARALLELISM  | Number of threads to use for parallel processing  | Number of CPUs available

| GUARDDOG_VERIFY_EXHAUSTIVE_DEPENDENCIES  | Analyze all possible versions of dependencies (true/false)  | false

| GUARDDOG_NPM_INCLUDE_DEV_DEPENDENCIES  | Include devDependencies when scanning npm package.json files (true/false); can also be toggled per-invocation with guarddog npm verify --include-dev-dependencies  | false

| GUARDDOG_TOP_PACKAGES_CACHE_LOCATION  | Location of the top packages cache directory  | guarddog/analyzer/metadata/resources

| GUARDDOG_YARA_EXT_EXCLUDE  | Comma-separated list of file extensions to exclude from YARA scanning  | ini,md,rst,txt,lock,json,yaml,yml,toml,xml,html,csv,sql,pdf,doc,docx,ppt,pptx,xls,xlsx,odt,changelog,readme,makefile,dockerfile,pkg-info,d.ts

#### Metadata Rule Configuration

https://github.com/DataDog/guarddog#metadata-rule-configuration

| Environment Variable  | Description  | Default Value

| GUARDDOG_NEW_DEPENDENCY_RISK_THRESHOLD  | Minimum risk score for a newly introduced dependency to flag the parent package in the risky_new_dependency rule  | 5.0

#### Archive Extraction Security Limits

https://github.com/DataDog/guarddog#archive-extraction-security-limits

GuardDog implements multiple security checks when extracting package archives to protect against compression bombs and file descriptor exhaustion attacks:

| Environment Variable  | Description  | Default Value

| GUARDDOG_MAX_UNCOMPRESSED_SIZE  | Maximum allowed uncompressed size in bytes (prevents disk space exhaustion)  | 2147483648 (2 GB)

| GUARDDOG_MAX_COMPRESSION_RATIO  | Maximum allowed compression ratio (detects suspicious compression patterns)  | 100 (100:1)

| GUARDDOG_MAX_FILE_COUNT  | Maximum number of files allowed in an archive (prevents file descriptor/inode exhaustion)  | 100000

## Maintainers

https://github.com/DataDog/guarddog#maintainers

- [Sebastian Obregoso](https://www.linkedin.com/in/sebastianobregoso/)
- [Ian Kretz](https://github.com/ikretz)
- [Tesnim Hamdouni](https://github.com/tesnim5hamdouni)

## Authors

https://github.com/DataDog/guarddog#authors

- [Ellen Wang](https://www.linkedin.com/in/ellen-wang-4bb5961a0/)
- [Christophe Tafani-Dereeper](https://github.com/christophetd)

## Acknowledgments

https://github.com/DataDog/guarddog#acknowledgments

Inspiration:

- [Backstabber’s Knife Collection: A Review of Open Source Software Supply Chain Attacks](https://arxiv.org/pdf/2005.09535)
- [What are Weak Links in the npm Supply Chain?](https://arxiv.org/pdf/2112.10165.pdf)
- [A Survey on Common Threats in npm and PyPi Registries](https://arxiv.org/pdf/2108.09576.pdf)
- [A Benchmark Comparison of Python Malware Detection Approaches](https://arxiv.org/pdf/2209.13288.pdf)
- [Towards Measuring Supply Chain Attacks on Package Managers for Interpreted Languages](https://arxiv.org/pdf/2002.01139)
