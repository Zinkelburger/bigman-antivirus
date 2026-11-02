---
url: "https://github.com/google/osv-scanner"
title: "GitHub - google/osv-scanner: Vulnerability scanner written in Go which uses the data provided by https://osv.dev"
site: "GitHub"
captured: "2026-11-02T19:01:54Z"
via: "live"
sha256: "0f35c6dd4408dcd72efc9675dd095db62e6a58067bf63bc1331d23c0c1a1ab7e"
wayback: "https://web.archive.org/web/20261023011402/https://github.com/google/osv-scanner"
cited_in:
  - "docs/research/vulnerability-intelligence-and-feeds.md"
  - "docs/threats/02-supply-chain-packages.md"
---

# GitHub - google/osv-scanner: Vulnerability scanner written in Go which uses the data provided by https://osv.dev

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/google/osv-scanner>
> Snapshot: <https://web.archive.org/web/20261023011402/https://github.com/google/osv-scanner>

https://scorecard.dev/viewer/?uri=github.com/google/osv-scanner https://github.com/google/osv-scanner/actions/workflows/checks.yml https://codecov.io/gh/google/osv-scanner https://slsa.dev https://github.com/google/osv-scanner/releases

Use OSV-Scanner to find existing vulnerabilities affecting your project's dependencies. OSV-Scanner provides an officially supported frontend to the [OSV database](https://osv.dev/) and CLI interface to [OSV-Scalibr](https://github.com/google/osv-scalibr) that connects a project’s list of dependencies with the vulnerabilities that affect them.

OSV-Scanner supports a wide range of project types, package managers and features, including but not limited to:

- Languages: C/C++, Dart, Elixir, Go, Java, Javascript, PHP, Python, R, Ruby, Rust.
- Package Managers: npm, pip, yarn, maven, go modules, cargo, gem, composer, nuget and others.
- Operating Systems: Detects vulnerabilities in OS packages on Linux systems.
- Containers: Scans container images for vulnerabilities in their base images and included packages.
- Guided Remediation: Provides recommendations for package version upgrades based on criteria such as dependency depth, minimum severity, fix strategy, and return on investment.

OSV-Scanner uses the extensible [OSV-Scalibr](https://github.com/google/osv-scalibr) library under the hood to provide this functionality. If a language or package manager is not supported currently, please file a [feature request.](https://github.com/google/osv-scanner/issues)

#### Underlying database

https://github.com/google/osv-scanner#underlying-database

The underlying database, [OSV.dev](https://osv.dev/) has several benefits in comparison with closed source advisory databases and scanners:

- Covering most open source language and OS ecosystems (including [Git](https://osv.dev/list?q=&ecosystem=GIT)), it’s comprehensive.
- Each advisory comes from an open and authoritative source (e.g. [GitHub Security Advisories](https://github.com/github/advisory-database), [RustSec Advisory Database](https://github.com/rustsec/advisory-db), [Ubuntu security notices](https://github.com/canonical/ubuntu-security-notices/tree/main/osv))
- Anyone can suggest improvements to advisories, resulting in a very high quality database.
- The OSV format unambiguously stores information about affected versions in a machine-readable format that precisely maps onto a developer’s list of packages

The above all results in accurate and actionable vulnerability notifications, which reduces the time needed to resolve them. Check out [OSV.dev](https://osv.dev/) for more details!

## Basic installation

https://github.com/google/osv-scanner#basic-installation

To install OSV-Scanner, please refer to the [installation section](https://google.github.io/osv-scanner/installation) of our documentation. OSV-Scanner releases can be found on the [releases page](https://github.com/google/osv-scanner/releases) of the GitHub repository. The recommended method is to download a prebuilt binary for your platform. Alternatively, you can use go install github.com/google/osv-scanner/v2/cmd/osv-scanner@latest to build it from source.

## Key Features

https://github.com/google/osv-scanner#key-features

For more information, please read our [detailed documentation](https://google.github.io/osv-scanner) to learn how to use OSV-Scanner. For detailed information about each feature, click their titles in this README.

Please note: These are the instructions for the latest OSV-Scanner V2 beta. If you are using V1, checkout the V1 [README](https://github.com/google/osv-scanner-v1) and [documentation](https://google.github.io/osv-scanner-v1/) instead.

### [Scanning a source directory](https://google.github.io/osv-scanner/usage)

https://github.com/google/osv-scanner#scanning-a-source-directory

```
$ osv-scanner scan source -r /path/to/your/dir
```

This command will recursively scan the specified directory for any supported package files, such as package.json, go.mod, pom.xml, etc. and output any discovered vulnerabilities.

OSV-Scanner has the option of using call analysis to determine if a vulnerable function is actually being used in the project, resulting in fewer false positives, and actionable alerts.

OSV-Scanner can also detect vendored C/C++ code for vulnerability scanning. See [here](https://google.github.io/osv-scanner/usage/#cc-scanning) for details.

#### Supported Lockfiles

https://github.com/google/osv-scanner#supported-lockfiles

OSV-Scanner supports 11+ language ecosystems and 19+ lockfile types. To check if your ecosystem is covered, please check out our [detailed documentation](https://google.github.io/osv-scanner/supported-languages-and-lockfiles/#supported-lockfiles).

### [Container Scanning](https://google.github.io/osv-scanner/usage/scan-image)

https://github.com/google/osv-scanner#container-scanning

OSV-Scanner also supports comprehensive, layer-aware scanning for container images to detect vulnerabilities in the following operating system packages and language-specific dependencies.

| Distro Support  | Language Artifacts Support

| Alpine OS  | Go

| Debian  | Java

| Ubuntu  | Node

|   | Python

See the [full documentation](https://google.github.io/osv-scanner/supported-languages-and-lockfiles/#supported-artifacts) for details on support.

Usage:

```
$ osv-scanner scan image my-image-name:tag
```

https://private-user-images.githubusercontent.com/86962800/407595827-8bb95366-27ec-45d1-86ed-e42890f2fb46.gif?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3OTM2NDY0MTQsIm5iZiI6MTc5MzY0NjExNCwicGF0aCI6Ii84Njk2MjgwMC80MDc1OTU4MjctOGJiOTUzNjYtMjdlYy00NWQxLTg2ZWQtZTQyODkwZjJmYjQ2LmdpZj9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjExMDIlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYxMTAyVDE5MDE1NFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTRmYWFiNDQ2Y2Y4MjYxYzFiNzZkODk2MjhmOGQ3MTEyY2VkZjVjMmI0MTUyMDcyNjU3NGQ3NWU2NDczZWY1ZDkmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRmdpZiJ9.ETQsI4dpX9fUVhh5J3Kzayw4aFnHgw1H3pGPSAKmSJk

### [License Scanning](https://google.github.io/osv-scanner/usage/license-scanning/)

https://github.com/google/osv-scanner#license-scanning

Check your dependencies' licenses using deps.dev data. For a summary:

```
osv-scanner --licenses path/to/repository
```

To check against an allowed license list (SPDX format):

```
osv-scanner --licenses="MIT,Apache-2.0" path/to/directory
```

### [Offline Scanning](https://google.github.io/osv-scanner/usage/offline-mode/)

https://github.com/google/osv-scanner#offline-scanning

Scan your project against a local OSV database. No network connection is required after the initial database download. The database can also be manually downloaded.

```
osv-scanner --offline --download-offline-databases ./path/to/your/dir
```

### [Guided Remediation](https://google.github.io/osv-scanner/experimental/guided-remediation/) (Experimental)

https://github.com/google/osv-scanner#guided-remediation-experimental

Warning

Guided remediation (the fix command) can be risky when run on untrusted projects. It may trigger the package manager to execute scripts or follow external registries specified in the project. Please ensure you trust the source code and artifacts before proceeding.

OSV-Scanner provides guided remediation, a feature that suggests package version upgrades based on criteria such as dependency depth, minimum severity, fix strategy, and return on investment. We currently support remediating vulnerabilities in the following files:

| Ecosystem  | File Format (Type)  | Supported Remediation Strategies

| npm  | package-lock.json (lockfile)  | [in-place](https://google.github.io/osv-scanner/experimental/guided-remediation/#in-place-lockfile-changes)

| npm  | package.json (manifest)  | [relock](https://google.github.io/osv-scanner/experimental/guided-remediation/#relock-and-relax-direct-dependencies)

| Maven  | pom.xml (manifest)  | [override](https://google.github.io/osv-scanner/experimental/guided-remediation/#override-dependency-versions)

This is available as a headless CLI command, as well as an interactive mode.

#### Example (for npm)

https://github.com/google/osv-scanner#example-for-npm

```
$ osv-scanner fix \
    --max-depth=3 \
    --min-severity=5 \
    --ignore-dev  \
    --strategy=in-place \
    -L path/to/package-lock.json
```

#### Interactive mode (for npm)

https://github.com/google/osv-scanner#interactive-mode-for-npm

```
$ osv-scanner fix \
    -M path/to/package.json \
    -L path/to/package-lock.json
```

https://camo.githubusercontent.com/c49edf24582243489040e4984675fa781ae3c658f0e16cb2348c428bafa446ec/68747470733a2f2f676f6f676c652e6769746875622e696f2f6f73762d7363616e6e65722f696d616765732f6775696465642d72656d6564696174696f6e2d72656c6f636b2d706174636865732e706e67

## Data Sources and Privacy

https://github.com/google/osv-scanner#data-sources-and-privacy

OSV-Scanner communicates with the following external services during operation:

### [OSV.dev API](https://osv.dev/)

https://github.com/google/osv-scanner#osvdev-api

The primary data source for vulnerability information. OSV-Scanner queries this API to check packages for known vulnerabilities and to identify vendored C/C++ dependencies. Data sent includes package names, versions, ecosystems, and file hashes. Use [--offline mode](https://google.github.io/osv-scanner/usage/offline-mode/) to disable network requests and scan against a local database instead.

### [deps.dev API](https://docs.deps.dev/api/)

https://github.com/google/osv-scanner#depsdev-api

Used for supplementary package information:

- Dependency resolution: Resolves dependency graphs for vulnerability scanning and remediation
- Container image scanning: Queries container image metadata for vulnerability detection
- License scanning (--licenses flag): Retrieves license information for packages
- Package deprecation: Checks if packages are deprecated

Data sent includes package names, versions, and ecosystems. No source code is transmitted.

### Package Registries

https://github.com/google/osv-scanner#package-registries

When using native registry for dependency resolution (instead of deps.dev), OSV-Scanner may query:

| Registry  | URL  | Used For

| Maven Central  | repo.maven.apache.org/maven2  | Maven package metadata and POM files

| npm Registry  | registry.npmjs.org  | npm package metadata

| PyPI  | pypi.org  | Python package metadata

## Contribute

https://github.com/google/osv-scanner#contribute

### Report Problems

https://github.com/google/osv-scanner#report-problems

If you have what looks like a bug, please use the [GitHub issue tracking system](https://github.com/google/osv-scanner/issues). Before you file an issue, please search existing issues to see if your issue is already covered.

### Contributing code to osv-scanner

https://github.com/google/osv-scanner#contributing-code-to-osv-scanner

See [CONTRIBUTING.md](https://github.com/google/osv-scanner/blob/main/CONTRIBUTING.md) for documentation on how to contribute code.

## Star History

https://github.com/google/osv-scanner#star-history

https://www.star-history.com/#google/osv-scanner&Date
