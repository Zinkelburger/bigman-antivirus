---
url: "https://google.github.io/osv-scanner/"
title: "OSV-Scanner"
site: "OSV-Scanner"
captured: "2026-11-02T18:32:20Z"
via: "live"
sha256: "3e6ca1a3013b3e816a86299c89509bb642e7635d89c9932e6e73b2190a47dfe9"
wayback: "https://web.archive.org/web/20261004223024/https://google.github.io/osv-scanner/"
cited_in:
  - "REVIEW.md"
---

# OSV-Scanner

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://google.github.io/osv-scanner/>
> Snapshot: <https://web.archive.org/web/20261004223024/https://google.github.io/osv-scanner/>

#  https://google.github.io/osv-scanner/#osv-scanner OSV-Scanner

https://api.securityscorecards.dev/projects/github.com/google/osv-scanner https://goreportcard.com/report/github.com/google/osv-scanner https://slsa.dev https://github.com/google/osv-scanner/releases

Use OSV-Scanner to find existing vulnerabilities affecting your project’s dependencies.

OSV-Scanner provides an officially supported frontend to the [OSV database](https://osv.dev/) that connects a project’s list of dependencies with the vulnerabilities that affect them.

You can use OSV-scanner in two ways:

- As a CLI tool: Run directly in a terminal or CI/CD pipeline to scan projects. See the [Installation Guide](https://google.github.io/osv-scanner/installation/) to get started.
- As a Go library: Import the [Go package](https://pkg.go.dev/github.com/google/osv-scanner/v2/pkg/osvscanner) to integrate vulnerability scanning logic into Go applications.

Since the OSV.dev database is open source and distributed, it has several benefits in comparison with closed source advisory databases and scanners:

- Each advisory comes from an open and authoritative source (e.g. the [RustSec Advisory Database](https://github.com/rustsec/advisory-db))
- Anyone can suggest improvements to advisories, resulting in a very high quality database
- The OSV format unambiguously stores information about affected versions in a machine-readable format that precisely maps onto a developer’s list of packages

The above all results in fewer, more actionable vulnerability notifications, which reduces the time needed to resolve them. Check out our [announcement blog post](https://security.googleblog.com/2022/12/announcing-osv-scanner-vulnerability.html) for more details!
