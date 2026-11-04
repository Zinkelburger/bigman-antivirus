---
url: "https://github.com/advisories/GHSA-97m3-w2cp-4xx6"
title: "CVE-2022-23812 - GitHub Advisory Database"
site: "GitHub"
captured: "2026-11-02T18:43:02Z"
via: "live"
sha256: "19d4c7bf0134646394a11118d46defebd318b5b3c0f8615e61af865d42fa11ca"
wayback: "https://web.archive.org/web/20261012070718/https://github.com/advisories/GHSA-97m3-w2cp-4xx6"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# CVE-2022-23812 - GitHub Advisory Database

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/advisories/GHSA-97m3-w2cp-4xx6>
> Snapshot: <https://web.archive.org/web/20261012070718/https://github.com/advisories/GHSA-97m3-w2cp-4xx6>

##  Embedded Malicious Code in node-ipc

Critical severity   GitHub Reviewed   Published Mar 16, 2022 to the GitHub Advisory Database • Updated Jan 27, 2023

## Package

node-ipc  ([npm](https://github.com/advisories?query=ecosystem%3Anpm))

## Affected versions

>= 10.1.1, < 10.1.3

## Patched versions

10.1.3

## Description

The package node-ipc versions 10.1.1 and 10.1.2 are vulnerable to embedded malicious code that was introduced by the maintainer. The malicious code was intended to overwrite arbitrary files dependent upon the geo-location of the user IP address. The maintainer removed the malicious code in version 10.1.3.

### References

- https://nvd.nist.gov/vuln/detail/CVE-2022-23812
- https://github.com/RIAEvangelist/node-ipc/issues/233
- https://github.com/RIAEvangelist/node-ipc/issues/236
- [RIAEvangelist/node-ipc@847047c](https://github.com/RIAEvangelist/node-ipc/commit/847047cf7f81ab08352038b2204f0e7633449580)
- https://github.com/RIAEvangelist/node-ipc/blob/847047cf7f81ab08352038b2204f0e7633449580/dao/ssl-geospec.js
- https://snyk.io/vuln/SNYK-JS-NODEIPC-2426370
- https://security.netapp.com/advisory/ntap-20220407-0005/

Published by the [National Vulnerability Database](https://nvd.nist.gov/vuln/detail/CVE-2022-23812) Mar 16, 2022

Published to the GitHub Advisory Database Mar 16, 2022

Reviewed Mar 16, 2022

Last updated Jan 27, 2023

### Severity

Critical

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

Unchanged

Confidentiality

High

Integrity

High

Availability

High

CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H

### EPSS score

(91st percentile)

### Weaknesses

Weakness CWE-94

####   [Improper Control of Generation of Code ('Code Injection')](https://github.com/advisories?query=cwe%3A94)

The product constructs all or part of a code segment using externally-influenced input from an upstream component, but it does not neutralize or incorrectly neutralizes special elements that could modify the syntax or behavior of the intended code segment.  [Learn more on MITRE.](https://cwe.mitre.org/data/definitions/94.html)

Weakness CWE-506

####   [Embedded Malicious Code](https://github.com/advisories?query=cwe%3A506)

The product contains code that appears to be malicious in nature.  [Learn more on MITRE.](https://cwe.mitre.org/data/definitions/506.html)

### CVE ID

CVE-2022-23812

### GHSA ID

GHSA-97m3-w2cp-4xx6

### Source code

[RIAEvangelist/node-ipc](https://github.com/RIAEvangelist/node-ipc)

Loading  Checking history

See something to contribute? [Suggest improvements for this vulnerability](https://github.com/advisories/GHSA-97m3-w2cp-4xx6/improve).
