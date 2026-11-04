---
url: "https://rustsec.org/advisories/RUSTSEC-2026-0259"
title: "RUSTSEC-2026-0259: arone: `arone` was removed from crates.io due to malicious code › RustSec Advisory Database"
captured: "2026-11-02T18:41:25Z"
via: "live"
sha256: "12bf9b09df39fe4a5484d8e32a38c50179883b9fa71b16b81522388fd97d1075"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# RUSTSEC-2026-0259: arone: `arone` was removed from crates.io due to malicious code › RustSec Advisory Database

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://rustsec.org/advisories/RUSTSEC-2026-0259>

[History](https://github.com/RustSec/advisory-db/commits/main/crates/arone/RUSTSEC-2026-0259.md) ⋅ [Edit](https://github.com/RustSec/advisory-db/edit/main/crates/arone/RUSTSEC-2026-0259.md) ⋅ [JSON (OSV)](https://api.osv.dev/v1/vulns/RUSTSEC-2026-0259)

#  RUSTSEC-2026-0259

arone was removed from crates.io due to malicious code

Reported

August 20, 2026

Issued

August 20, 2026   (last modified: August 21, 2026)

Package

[arone](https://rustsec.org/packages/arone.html)

Type

Vulnerability

References

-  https://blog.rust-lang.org/2026/08/20/supply-chain-attack-on-arrayref

Patched

no patched versions

### Description

We identified that arone contained malicious code executed through a build script.

This crate had 7 versions, with the most recent one being published at 2026-08-18. The crate was removed from crates.io on 2026-08-20 and the user account was locked.

This crate was used as part of a malware campaign targeted at users of arrayref, which was downloaded 2,285 times before being removed; see [the arrayref advisory](https://rustsec.org/advisories/RUSTSEC-2026-0260.html) for more detail.

Advisory available under [CC0-1.0](https://spdx.org/licenses/CC0-1.0.html) license.
