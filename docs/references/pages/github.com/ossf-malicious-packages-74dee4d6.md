---
url: "https://github.com/ossf/malicious-packages"
title: "GitHub - ossf/malicious-packages: A repository of reports of malicious packages identified in Open Source package repositories, consumable via the Open Source Vulnerability (OSV) format."
site: "GitHub"
captured: "2026-11-02T19:06:53Z"
via: "live"
sha256: "1250141f2cc5f0523bc091fa3bff7d8e95c387cb29f2c325dc62b4d61c12c678"
wayback: "https://web.archive.org/web/20260809131808/https://github.com/ossf/malicious-packages"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# GitHub - ossf/malicious-packages: A repository of reports of malicious packages identified in Open Source package repositories, consumable via the Open Source Vulnerability (OSV) format.

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/ossf/malicious-packages>
> Snapshot: <https://web.archive.org/web/20260809131808/https://github.com/ossf/malicious-packages>

# OpenSSF Malicious Packages

https://github.com/ossf/malicious-packages#openssf-malicious-packages

https://ossf.github.io/malicious-packages/stats/

This repository is a collection of reports of malicious packages identified in Open Source package repositories, consumable via the [Open Source Vulnerability (OSV)](https://osv.dev) format.

This project is closely related to the [OpenSSF Package Analysis project](https://github.com/ossf/package-analysis).

## About

https://github.com/ossf/malicious-packages#about

### Background

https://github.com/ossf/malicious-packages#background

Attacks against open source ecosystems are gaining popularity. Typosquatting, dependency confusion, account takeovers, etc are happening more frequently each year.

While some protection can be found through various security solutions, there is no comprehensive database of malicious packages published to open source package repositories.

### Objective

https://github.com/ossf/malicious-packages#objective

The aim of this project and repository is to be a comprehensive, high quality, open source database of reports of malicious packages published on open source package repositories.

These public reports help protect the open source community, and provide a data source for the security community to improve their ability to find and detect new open source malware.

## Scope

https://github.com/ossf/malicious-packages#scope

What is in scope?

- any package that belongs to an ecosystem supported by the [OSV Schema](https://ossf.github.io/osv-schema/)
- malicious packages published under typosquatting type attacks
- malicious packages published through account takeover
- malicious prebuilt binaries downloaded or installed with a package
- security researcher activity
- dependency and manifest confusion

Borderline:

- typosquatting, or spam packages that are empty or trivial, while not malicious, are allowed to be present in the dataset

Out-of-scope:

- non-malicious packages
- vulnerability reports
- compromised infrastructure
- offensive security tools, unless they execute malicious payloads on install

## Definition of a Malicious Package

https://github.com/ossf/malicious-packages#definition-of-a-malicious-package

Below is the definition of what this repository considers a malicious package.

- a package publicly available in a package registry
- and either:

- when installed or used, would require some sort of incident response due to the loss of confidentiality, availability and/or integrity; or
- exfiltrates an identifier that can be directly used to launch a subsequent attack against the victim (e.g. username for phishing or password bruteforcing)

- and also either:

- violates the terms of the package registry; or
- would be reasonably considered to require removal from the package registry

### Dependency and manifest confusion

https://github.com/ossf/malicious-packages#dependency-and-manifest-confusion

[Dependency confusion](https://medium.com/@alex.birsan/dependency-confusion-4a5d60fec610) and [manifest confusion](https://blog.vlt.sh/blog/the-massive-hole-in-the-npm-ecosystem) are techniques that exploit quirks in the behavior of package systems and how they are used within organizations. Packages using these attacks are malicious.

Very occasionally someone may unintentionally encounter these quirks, but this is considered infrequent.

Manifest confusion requires someone to bypass the NPM command line tool and deliberately provide an altered manifest.

Dependency confusion are effectively the same as an account takeover where an attacker replaces a package's code with their own. This means even trivial or empty dependency confusion packages would require incident response.

### Spam and typosquating

https://github.com/ossf/malicious-packages#spam-and-typosquating

Spam, typosquatting are not malicious, unless the package itself exhibits malicious behavior as-per the definition above.

These types of packages are often empty (i.e. no functional code), or consist of only useless trivial functionality (e.g. printing a message). While these packages are not malicious, they are a nuisance and generally unwanted.

Typosquatting packages may be hard to distinguish from dependency confusion. As a result, these reports are allowed to be present in the malicious packages repository.

### Reverse engineering protection (e.g. obfuscation)

https://github.com/ossf/malicious-packages#reverse-engineering-protection-eg-obfuscation

Reverse engineering protections are not malicious, unless it exhibits malicious behavior as-per the definition above.

Obfuscation, debugger evasion, and other reverse engineering protection techniques, are used by both developers seeking to protect their source code and attackers seeking to evade detection.

Malicious obfuscated packages may be hard to distinguish from non-malicious obfuscated packages. As a result, reports are allowed in the malicious package repository for obfuscated packages, even if they are not clearly malicious, when they belong to an ecosystem where obfuscation is forbidden in their terms of use (e.g. PyPI, Crates.IO).

### Telemetry

https://github.com/ossf/malicious-packages#telemetry

Telemetry, on its own, is not malicious.

Many open source packages use telemetry to track installs or the behavior and performance of the package.

However, if telemetry is abused to exfiltrate and steal sensitive data, or provide remote access, this can be considered malicious.

### Protestware

https://github.com/ossf/malicious-packages#protestware

Protestware is not malicious if it does not affect the availability, integrity or confidentiality of the systems the package is run on. For example, a message logged to a console may be annoying to a developer, but is not malicious.

However, protestware that destroys files, slows performance, or otherwise affects availability, integrity or confidentiality as part of the protest may be considered malicious.

### Offensive Security Tools

https://github.com/ossf/malicious-packages#offensive-security-tools

Offensive security tools, libraries, hacking tools, etc are not malicious.

While an offensive security tool being discovered in an environment may indicate the presence of compromise, the package itself is not itself malicious.

These packages don't necessarily violate the terms of the registries hosting them, and are often used by security researchers.

However, offensive security tools that execute malicious payloads during installation are considered malicious packages.

## Statistics

https://github.com/ossf/malicious-packages#statistics

Statistics on the number of reports in the repository are automatically updated daily.

View them at [ossf.github.io/malicious-packages/stats](https://ossf.github.io/malicious-packages/stats/).

## Get Involved

https://github.com/ossf/malicious-packages#get-involved

### Contribute Malicious Package Reports

https://github.com/ossf/malicious-packages#contribute-malicious-package-reports

See our [contributing guide](https://github.com/ossf/malicious-packages/blob/main/CONTRIBUTING.md) for complete details.

#### OSV reports via Pull Request

https://github.com/ossf/malicious-packages#osv-reports-via-pull-request

We accept new reports, and updates to existing reports.

We will also accept bulk imports via PR (please create an issue first).

#### Automated Sources

https://github.com/ossf/malicious-packages#automated-sources

If you regularly produce high-quality detections with few false-positives, and have them accumulating in a database, we can automatically consume them as OSV from a cloud storage environment (S3, GCS).

### Comms

https://github.com/ossf/malicious-packages#comms

- Most communication occurs in the [OpenSSF Package Analysis Slack channel](https://openssf.slack.com/archives/package_analysis)
- Official communications occur on the https://lists.openssf.org/g/openssf-wg-securing-crit-prjs mailing list.
[Manage your subscriptions to Open SSF mailing lists](https://lists.openssf.org/g/main/subgroups).

### Meeting Times

https://github.com/ossf/malicious-packages#meeting-times

- Every other Thursday @ either an APAC or EMEA friendly time (See [shared calendar](https://calendar.google.com/calendar/u/2?cid=czYzdm9lZmhwNWk5cGZsdGI1cTY3bmdwZXNAZ3JvdXAuY2FsZW5kYXIuZ29vZ2xlLmNvbQ)).
- [Meeting Minutes](https://docs.google.com/document/d/1MIXxadtWsaROpFcJnBtYnQPoyzTCIDhd0IGV8PIV0mQ/edit).

## False Positives

https://github.com/ossf/malicious-packages#false-positives

While we do our best to ensure false positives are not present, they may be present in our dataset from time-to-time.

If you see a non-malicious package is flagged as malicious [create an issue](https://github.com/ossf/malicious-packages/issues/new). Please include the following:

- The affected ecosystem and package.
- Which versions are false positives, if specific versions are false positives.
- Any relevant links.

We will then either:

- Move the entire report into the ./osv/withdrawn/ directory and add the withdrawn time to the report - if the whole report is a false positive.
- Move the affected versions into a database_specific array indicating that which versions were false positives - if some versions are malicious and some are false positives.

Finally, reports that have been added to the malicious packages repository will not be removed.

Note: support for handling false positives is TBC.

## Prior and Related Work

https://github.com/ossf/malicious-packages#prior-and-related-work

- GitHub's [Advisory Database (filtered by malware)](https://github.com/advisories?query=type%3Amalware), for the NPM ecosystem.
- https://github.com/lxyeternal/pypi_malregistry (PyPI)
- https://dasfreak.github.io/Backstabbers-Knife-Collection/ (PyPI and npm), by Marc Ohm et al.
- https://github.com/datadog/malicious-software-packages-dataset (PyPI), by Datadog

## Governance

https://github.com/ossf/malicious-packages#governance

This work is associated with the [Package Analysis project](https://github.com/ossf/package-analysis).

This project belongs to the [Securing Critical Projects Working Group](https://github.com/ossf/wg-securing-critical-projects) in the [OpenSSF](https://openssf.org/) ([Slack](https://openssf.slack.com/archives/wg_securing_critical_projects)).

The working group's [CHARTER.md](https://github.com/ossf/wg-securing-critical-projects/blob/main/CHARTER.md) outlines the scope and governance of our group activities.
