---
url: "https://docs.pypi.org/attestations/"
title: "Introduction to Attestations - PyPI User Documentation"
captured: "2026-11-02T18:28:52Z"
via: "live"
sha256: "84d6916fc8a36bfcaa1d18c31bbddb1c6f661341e1930313de7060670e3c8246"
wayback: "https://web.archive.org/web/20261018090904/https://docs.pypi.org/attestations/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Introduction to Attestations - PyPI User Documentation

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://docs.pypi.org/attestations/>
> Snapshot: <https://web.archive.org/web/20261018090904/https://docs.pypi.org/attestations/>

https://github.com/pypi/warehouse/blob/main/docs/user/attestations/index.md

# Introduction to Attestations[¶](https://docs.pypi.org/attestations/#introduction-to-attestations)

These pages document PyPI's implementation of digital attestations ([PEP 740](https://peps.python.org/pep-0740/)), including in-toto attestation predicates specific to PyPI itself.

## Quick background[¶](https://docs.pypi.org/attestations/#quick-background)

Digital attestations enable package maintainers as well as third parties (such as the index itself, external auditors, etc.) to cryptographically sign for uploaded packages.

These signatures bind each release distribution (such as an individual sdist or wheel) to a strong cryptographic digest of its contents, allowing both PyPI and downstream users to verify that a particular package was attested to by a particular identity (such as a GitHub Actions workflow).

These attestations can take multiple forms, including [publish attestations](https://docs.pypi.org/attestations/publish/v1/) for publicly verifiable proof that a package was published via a specific [Trusted Publisher](https://docs.pypi.org/trusted-publishers/), or more general [SLSA Provenance](https://slsa.dev/spec/v1.0/provenance) attesting to a package's original source location.

## Supported attestations[¶](https://docs.pypi.org/attestations/#supported-attestations)

PyPI uses the [in-toto Attestation Framework](https://github.com/in-toto/attestation/blob/main/spec/README.md) for the attestations it accepts.

Currently, PyPI allows the following attestation predicates:

- [SLSA Provenance](https://slsa.dev/spec/v1.0/provenance)
- [PyPI Publish](https://docs.pypi.org/attestations/publish/v1/)

Each file can be uploaded along with its attestations. Currently PyPI supports two attestations per uploaded release file: one for each of the allowed predicates. Uploads with more than two attestations per file, and uploads with attestations with repeated predicates will be rejected.

Currently, PyPI allows for attestations to be signed by the following Trusted Publisher identities:

- [GitHub Actions](https://docs.pypi.org/trusted-publishers/using-a-publisher/#github-actions)
- [GitLab CI/CD](https://docs.pypi.org/trusted-publishers/using-a-publisher/#gitlab-cicd)
- [Google Cloud](https://docs.pypi.org/trusted-publishers/using-a-publisher/#google-cloud)
- [ActiveState](https://docs.pypi.org/trusted-publishers/using-a-publisher/#activestate)
