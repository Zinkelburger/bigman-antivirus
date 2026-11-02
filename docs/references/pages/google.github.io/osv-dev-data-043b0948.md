---
url: "https://google.github.io/osv.dev/data/"
title: "Data sources"
site: "OSV"
captured: "2026-11-02T18:32:24Z"
via: "live"
sha256: "9cffdecfbcc36d7430c537ad9d9a24a374007c64f503ab9380f3c38f0094a852"
wayback: "https://web.archive.org/web/20261023005202/https://google.github.io/osv.dev/data/"
cited_in:
  - "docs/research/vulnerability-intelligence-and-feeds.md"
---

# Data sources

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://google.github.io/osv.dev/data/>
> Snapshot: <https://web.archive.org/web/20261023005202/https://google.github.io/osv.dev/data/>

#  https://google.github.io/osv.dev/data/#data-sources Data sources

Table of contents

- [Current data sources](https://google.github.io/osv.dev/data/#current-data-sources)
- [Converted data](https://google.github.io/osv.dev/data/#converted-data)
- [Covered Ecosystems](https://google.github.io/osv.dev/data/#covered-ecosystems)
- [Data Quality](https://google.github.io/osv.dev/data/#data-quality)
- [Data dumps](https://google.github.io/osv.dev/data/#data-dumps)

- [Full database download](https://google.github.io/osv.dev/data/#full-database-download)
- [Per-ecosystem downloads](https://google.github.io/osv.dev/data/#per-ecosystem-downloads)
- [Downloading recent changes](https://google.github.io/osv.dev/data/#downloading-recent-changes)
- [Ecosystem naming](https://google.github.io/osv.dev/data/#ecosystem-naming)

- [Contributing Data](https://google.github.io/osv.dev/data/#contributing-data)

##  https://google.github.io/osv.dev/data/#current-data-sources Current data sources

This is an ongoing project. We encourage open source ecosystems to adopt the [Open Source Vulnerability format](https://ossf.github.io/osv-schema/) to enable open source users to easily aggregate and consume vulnerabilities across all ecosystems. See our [blog post](https://security.googleblog.com/2021/06/announcing-unified-vulnerability-schema.html) for more details.

The following ecosystems have vulnerabilities encoded in this format:

- [GitHub Advisory Database](https://github.com/github/advisory-database) ([CC-BY 4.0](https://github.com/github/advisory-database/blob/main/LICENSE.md))
- [PyPI Advisory Database](https://github.com/pypa/advisory-database) ([CC-BY 4.0](https://github.com/pypa/advisory-database/blob/main/LICENSE))
- [Go Vulnerability Database](https://github.com/golang/vulndb) ([CC-BY 4.0](https://github.com/golang/vulndb#license))
- [Rust Advisory Database](https://github.com/RustSec/advisory-db) ([CC0 1.0](https://github.com/rustsec/advisory-db/blob/main/LICENSE.txt))
- [Drupal Advisory Database](https://github.com/DrupalSecurityTeam/drupal-advisory-database) ([MIT](https://github.com/DrupalSecurityTeam/drupal-advisory-database/blob/main/LICENSE))
- [Global Security Database](https://github.com/cloudsecurityalliance/gsd-database) ([CC0 1.0](https://github.com/cloudsecurityalliance/gsd-database/blob/main/LICENSE))
- [OSS-Fuzz](https://github.com/google/oss-fuzz-vulns) ([CC-BY 4.0](https://github.com/google/oss-fuzz-vulns/blob/main/LICENSE))
- [Rocky Linux](https://distro-tools.rocky.page/apollo/openapi/#osv) ([BSD](https://rockylinux.org/licensing))
- [AlmaLinux](https://github.com/AlmaLinux/osv-database) ([MIT](https://github.com/AlmaLinux/osv-database/blob/master/LICENSE))
- [Haskell Security Advisories](https://github.com/haskell/security-advisories) ([CC0 1.0](https://github.com/haskell/security-advisories/blob/main/LICENSE.txt))
- [RConsortium Advisory Database](https://github.com/RConsortium/r-advisory-database) ([Apache 2.0](https://github.com/RConsortium/r-advisory-database/blob/main/LICENSE))
- [OpenSSF Malicious Packages](https://github.com/ossf/malicious-packages) ([Apache 2.0](https://github.com/ossf/malicious-packages/blob/main/LICENSE))
- [Python Software Foundation Database](https://github.com/psf/advisory-database) ([CC-BY 4.0](https://github.com/psf/advisory-database/blob/main/LICENSE))
- [Bitnami Vulnerability Database](https://github.com/bitnami/vulndb) ([Apache 2.0](https://github.com/bitnami/vulndb/blob/main/LICENSE.md))
- [Ubuntu](https://github.com/canonical/ubuntu-security-notices) ([CC-BY-SA 4.0](https://github.com/canonical/ubuntu-security-notices/blob/main/LICENSE))
- [opam (OCaml package manager)](https://github.com/ocaml/security-advisories) ([CC0 1.0](https://github.com/ocaml/security-advisories/blob/main/LICENSE.txt))
- [Erlang Ecosystem Foundation CNA](https://cna.erlef.org/) ([CC-BY 4.0](https://cna.erlef.org/data-licensing/))
- [Android](https://source.android.com/docs/security/bulletin) ([CC-BY 4.0](https://source.android.com/license))
- [Azure Linux](https://github.com/microsoft/AzureLinuxVulnerabilityData) ([MIT](https://github.com/microsoft/AzureLinuxVulnerabilityData/blob/main/LICENSE))
- [BellSoft](https://github.com/bell-sw/osv-database) ([MIT](https://github.com/bell-sw/osv-database/blob/master/LICENSE))
- [Chainguard / Wolfi](https://github.com/chainguard-dev/vulnerability-scanner-support/blob/main/docs/osv_v3_feed.md) ([CC-BY-NC-ND 4.0](https://github.com/chainguard-dev/vulnerability-scanner-support/blob/main/docs/foundational_concepts.md#security-feeds))
- [CleanStart](https://github.com/cleanstart-dev/cleanstart-security-advisories) ([Apache 2.0](https://github.com/cleanstart-dev/cleanstart-security-advisories/blob/main/LICENSE))
- [Curl](https://curl.se/docs/security.html) ([MIT](https://github.com/curl/curl-www/blob/master/LICENSE))
- [Julia Security Advisories](https://github.com/JuliaLang/SecurityAdvisories.jl) ([CC-BY 4.0](https://github.com/JuliaLang/SecurityAdvisories.jl/blob/main/LICENSE.md))
- [Red Hat Security Data](https://security.access.redhat.com/data) ([CC-BY 4.0](https://security.access.redhat.com/data))
- [Red Hat Lightwell](https://github.com/project-lightwell/lightwell-osv) ([CC-BY 4.0](https://github.com/project-lightwell/lightwell-osv/blob/main/LICENSE))
- [V8 / Chromium](https://github.com/google/chromium-policy-vulnfeed) ([Apache 2.0](https://github.com/google/chromium-policy-vulnfeed/blob/main/LICENSE))

##  https://google.github.io/osv.dev/data/#converted-data Converted data

Additionally, the OSV.dev team maintains a conversion pipeline for:

- [Debian Security Advisories](https://storage.googleapis.com/debian-osv/index.html), using the conversion tools [here](https://github.com/google/osv.dev/tree/master/vulnfeeds/cmd/converters/debian).
- [Alpine SecDB](https://storage.googleapis.com/cve-osv-conversion/index.html?prefix=osv-output/) ([CC-BY-SA 4.0](https://secdb.alpinelinux.org/license.txt)), using the conversion tools [here](https://github.com/google/osv.dev/tree/master/vulnfeeds/cmd/converters/alpine),
- [NVD CVEs for open source software](https://storage.googleapis.com/cve-osv-conversion/index.html?prefix=osv-output/) using the conversion tools [here](https://github.com/google/osv.dev/tree/master/vulnfeeds/cmd/converters/cve/nvd-cve-osv)

##  https://google.github.io/osv.dev/data/#covered-ecosystems Covered Ecosystems

Between the data served in OSV and the data converted to OSV the following ecosystems are covered.

- AlmaLinux
- Alpine
- Android
- Bitnami
- crates.io
- Curl
- Debian GNU/Linux
- Git ([including C/C++](https://osv.dev/blog/posts/introducing-broad-c-c++-support/))
- GitHub Actions
- Go
- Haskell
- Hex
- Julia
- Linux kernel
- Maven
- npm
- NuGet
- opam (OCaml package manager)
- OSS-Fuzz
- Packagist
- Pub
- PyPI
- Python
- R (CRAN and Bioconductor)
- Rocky Linux
- Root
- RubyGems
- SwiftURL
- Ubuntu OS

##  https://google.github.io/osv.dev/data/#data-quality Data Quality

The quality of the data in OSV.dev [is very important to us](https://google.github.io/osv.dev/faq/#ive-found-something-wrong-with-the-data). The minimum quality bar for OSV records acceptable for import is documented [here](https://google.github.io/osv.dev/data_quality.html)

##  https://google.github.io/osv.dev/data/#data-dumps Data dumps

For convenience, these sources are aggregated and [continuously](https://github.com/google/osv.dev/blob/master/deployment/clouddeploy/gke-workers/base/core/exporter.yaml) exported to a GCS bucket maintained by OSV: [gs://osv-vulnerabilities](https://storage.googleapis.com/osv-vulnerabilities/index.html)

###  https://google.github.io/osv.dev/data/#full-database-download Full database download

This bucket contains a zip file with all vulnerabilities across all ecosystems (including withdrawn records) at gs://osv-vulnerabilities/all.zip. This is the easiest way to download the entire OSV database.

###  https://google.github.io/osv.dev/data/#per-ecosystem-downloads Per-ecosystem downloads

Individual vulnerability records can be found at gs://osv-vulnerabilities/<ECOSYSTEM>/<ID>.json. A zip containing all vulnerabilities for each ecosystem is available at gs://osv-vulnerabilities/<ECOSYSTEM>/all.zip. Vulnerabilities without an ecosystem (typically withdrawn ones) are exported to the gs://osv-vulnerabilities/[EMPTY]/ directory.

E.g. for PyPI vulnerabilities:

```
# Or download over HTTP via https://storage.googleapis.com/osv-vulnerabilities/PyPI/all.zip
gcloud storage cp gs://osv-vulnerabilities/PyPI/all.zip .

```

###  https://google.github.io/osv.dev/data/#downloading-recent-changes Downloading recent changes

To efficiently download only new or updated records, you can use the modified_id.csv files. These files list vulnerabilities by their last modified time.

Two types of CSV files are provided:

- A top-level file: Located at gs://osv-vulnerabilities/modified_id.csv, this file contains a list of all modified vulnerabilities across all ecosystems.
- Per-ecosystem files: Each ecosystem directory (e.g., gs://osv-vulnerabilities/PyPI/) contains its own modified_id.csv file, listing only the vulnerabilities for that specific ecosystem.

Format and Usage

The format of the top-level CSV is <iso modified date>,<ecosystem_dir>/<id>. The per-ecosystem files omit the <ecosystem_dir>/ prefix.

For example (from the top-level file):

```
2024-08-15T00:05:00Z,PyPI/PYSEC-2021-123
2024-08-15T00:01:00Z,Go/GO-2022-0123
2024-08-14T12:00:00Z,npm/1234

```

The CSV files are sorted in reverse chronological order. This allows you to stream the file and stop processing when you encounter a timestamp that you have already seen, avoiding the need to parse the entire file.

###  https://google.github.io/osv.dev/data/#ecosystem-naming Ecosystem naming

Some ecosystems contain a : separator in the name (e.g. Alpine:v3.17). For these ecosystems, the data dump will always contain an ecosystem directory without the :.* suffix (e.g. Alpine). This will contain all the advisories of the ecosystem with the same prefix (e.g. All Alpine:.*).

A list of all current ecosystems is available at [gs://osv-vulnerabilities/ecosystems.txt](https://storage.googleapis.com/osv-vulnerabilities/ecosystems.txt)

Note: OSV.dev has stopped exporting entries for ecosystems with prefixes (e.g. All Alpine:.*). Please refer only to the main ecosystem, the one without the :.* suffix, for all vulnerabilities of that ecosystem.

##  https://google.github.io/osv.dev/data/#contributing-data Contributing Data

If you work with a project such as a Linux distribution and would like to contribute your security advisories, please follow the steps and [data licensing requirements](https://google.github.io/osv.dev/data/new#data-licensing) outlined in the [New Data Source](https://google.github.io/osv.dev/data/new) page.

Data can be supplied either through a public Git repository, to [REST API endpoints](https://google.github.io/osv.dev/data/new/rest-api), or through a public GCS bucket.

## Table of contents

-  [New Data Source](https://google.github.io/osv.dev/data/new)
