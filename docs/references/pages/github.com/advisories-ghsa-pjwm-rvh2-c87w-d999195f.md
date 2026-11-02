---
url: "https://github.com/advisories/GHSA-pjwm-rvh2-c87w"
title: "CVE-2021-4229 - GitHub Advisory Database"
site: "GitHub"
captured: "2026-11-02T18:44:04Z"
via: "live"
sha256: "eafc5fa41802f98f2257e79830bb235c5fc74ef6ddd36491f566a0a7480b0665"
wayback: "https://web.archive.org/web/20261005043639/https://github.com/advisories/GHSA-pjwm-rvh2-c87w"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# CVE-2021-4229 - GitHub Advisory Database

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/advisories/GHSA-pjwm-rvh2-c87w>
> Snapshot: <https://web.archive.org/web/20261005043639/https://github.com/advisories/GHSA-pjwm-rvh2-c87w>

##  Embedded malware in ua-parser-js

High severity   GitHub Reviewed   Published Oct 22, 2021 to the GitHub Advisory Database • Updated Jul 15, 2026

## Package

ua-parser-js  ([npm](https://github.com/advisories?query=ecosystem%3Anpm))

## Affected versions

= 0.7.29

= 0.8.0

= 1.0.0

## Patched versions

0.7.30

0.8.1

1.0.1

## Description

The npm package ua-parser-js had three versions published with malicious code. Users of affected versions (0.7.29, 0.8.0, 1.0.0) should upgrade as soon as possible and check their systems for suspicious activity. See [this issue](https://github.com/faisalman/ua-parser-js/issues/536) for details as they unfold.

Any computer that has this package installed or running should be considered fully compromised. All secrets and keys stored on that computer should be rotated immediately from a different computer. The package should be removed, but as full control of the computer may have been given to an outside entity, there is no guarantee that removing the package will remove all malicious software resulting from installing it.

### References

- [faisalman/ua-parser-js#536](https://github.com/faisalman/ua-parser-js/issues/536)
- https://www.npmjs.com/package/ua-parser-js
- [faisalman/ua-parser-js#536 (comment)](https://github.com/faisalman/ua-parser-js/issues/536#issuecomment-949772496)

Reviewed Oct 22, 2021

Published to the GitHub Advisory Database Oct 22, 2021

Last updated Jul 15, 2026

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

Required

Scope

Unchanged

Confidentiality

High

Integrity

High

Availability

High

CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:U/C:H/I:H/A:H

### EPSS score

(75th percentile)

### Weaknesses

Weakness CWE-506

####   [Embedded Malicious Code](https://github.com/advisories?query=cwe%3A506)

The product contains code that appears to be malicious in nature.  [Learn more on MITRE.](https://cwe.mitre.org/data/definitions/506.html)

Weakness CWE-912

####   [Hidden Functionality](https://github.com/advisories?query=cwe%3A912)

The product contains functionality that is not documented, not part of the specification, and not accessible through an interface or command sequence that is obvious to the product's users or administrators.  [Learn more on MITRE.](https://cwe.mitre.org/data/definitions/912.html)

### CVE ID

CVE-2021-4229

### GHSA ID

GHSA-pjwm-rvh2-c87w

### Source code

[faisalman/ua-parser-js](https://github.com/faisalman/ua-parser-js)

### Credits

-   https://github.com/xtqqczze [xtqqczze](https://github.com/xtqqczze)   Analyst
-   https://github.com/faisalman [faisalman](https://github.com/faisalman)   Analyst
-   https://github.com/conorfitch [conorfitch](https://github.com/conorfitch)   Analyst

Loading  Checking history

See something to contribute? [Suggest improvements for this vulnerability](https://github.com/advisories/GHSA-pjwm-rvh2-c87w/improve).
