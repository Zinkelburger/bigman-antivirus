---
url: "https://github.com/cisagov/kev-data"
title: "GitHub - cisagov/kev-data: Mirror of cisa.gov/kev data files"
site: "GitHub"
captured: "2026-11-02T18:48:41Z"
via: "live"
sha256: "4cd111b40d8e924b149db8b48c013845aa7f06d7fc12e514b96d1a4985e57b84"
wayback: "https://web.archive.org/web/20260709160010/https://github.com/cisagov/kev-data"
cited_in:
  - "docs/research/vulnerability-intelligence-and-feeds.md"
---

# GitHub - cisagov/kev-data: Mirror of cisa.gov/kev data files

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/cisagov/kev-data>
> Snapshot: <https://web.archive.org/web/20260709160010/https://github.com/cisagov/kev-data>

## Repository navigation

[kev-data](https://github.com/cisagov/kev-data)

Public

- [Notifications](https://github.com/login?return_to=%2Fcisagov%2Fkev-data)
- [Fork (31)](https://github.com/login?return_to=%2Fcisagov%2Fkev-data)
- [Star (136)](https://github.com/login?return_to=%2Fcisagov%2Fkev-data)

## About

Mirror of cisa.gov/kev data files

### Resources

[Readme](https://github.com/cisagov/kev-data#readme-ov-file)

[License](https://github.com/cisagov/kev-data#License-1-ov-file)

### Contributing

[Contributing](https://github.com/cisagov/kev-data#contributing-ov-file)

### Security policy

[Security policy](https://github.com/cisagov/kev-data#security-ov-file)

[Activity](https://github.com/cisagov/kev-data/activity)

[Custom properties](https://github.com/cisagov/kev-data/custom-properties)

### Stars

136 stars

### Watchers

18 watching

### Forks

[31 forks](https://github.com/cisagov/kev-data/forks)

[Report repository](https://github.com/contact/report-content?content_url=https%3A%2F%2Fgithub.com%2Fcisagov%2Fkev-data&report=cisagov+%28user%29)

[Branches](https://github.com/cisagov/kev-data/branches)[Tags](https://github.com/cisagov/kev-data/tags)

https://github.com/cisagov/kev-data/brancheshttps://github.com/cisagov/kev-data/tags

## Latest commit

## History

[473 Commits](https://github.com/cisagov/kev-data/commits/develop/)

https://github.com/cisagov/kev-data/commits/develop/

## Folders and files

| Name | Name |

Last commit message

|

Last commit date

|

[.github](https://github.com/cisagov/kev-data/tree/develop/.github)

|

[.github](https://github.com/cisagov/kev-data/tree/develop/.github)

|

|

|

[CONTRIBUTING.md](https://github.com/cisagov/kev-data/blob/develop/CONTRIBUTING.md)

|

[CONTRIBUTING.md](https://github.com/cisagov/kev-data/blob/develop/CONTRIBUTING.md)

|

|

|

[LICENSE](https://github.com/cisagov/kev-data/blob/develop/LICENSE)

|

[LICENSE](https://github.com/cisagov/kev-data/blob/develop/LICENSE)

|

|

|

[README.md](https://github.com/cisagov/kev-data/blob/develop/README.md)

|

[README.md](https://github.com/cisagov/kev-data/blob/develop/README.md)

|

|

|

[index.html](https://github.com/cisagov/kev-data/blob/develop/index.html)

|

[index.html](https://github.com/cisagov/kev-data/blob/develop/index.html)

|

|

|

[known_exploited_vulnerabilities.csv](https://github.com/cisagov/kev-data/blob/develop/known_exploited_vulnerabilities.csv)

|

[known_exploited_vulnerabilities.csv](https://github.com/cisagov/kev-data/blob/develop/known_exploited_vulnerabilities.csv)

|

|

|

[known_exploited_vulnerabilities.json](https://github.com/cisagov/kev-data/blob/develop/known_exploited_vulnerabilities.json)

|

[known_exploited_vulnerabilities.json](https://github.com/cisagov/kev-data/blob/develop/known_exploited_vulnerabilities.json)

|

|

|

[known_exploited_vulnerabilities_schema.json](https://github.com/cisagov/kev-data/blob/develop/known_exploited_vulnerabilities_schema.json)

|

[known_exploited_vulnerabilities_schema.json](https://github.com/cisagov/kev-data/blob/develop/known_exploited_vulnerabilities_schema.json)

|

|

|

## Repository files navigation

# Welcome to kev-data

https://github.com/cisagov/kev-data#welcome-to-kev-data

This repository is home to the data files that make up the Known Exploited Vulnerabilities (KEV) catalog. The data is originally sourced from https://www.cisa.gov/known-exploited-vulnerabilities-catalog, which is short linked at https://cisa.gov/kev.

Interactive dashboard to easily see commit changes: https://cisagov.github.io/kev-data/

## File formats

https://github.com/cisagov/kev-data#file-formats

Currently, KEV data is produced in two formats, [CSV](https://github.com/cisagov/kev-data/blob/develop/known_exploited_vulnerabilities.csv) (Comma-Separated Values) and [JSON](https://github.com/cisagov/kev-data/blob/develop/known_exploited_vulnerabilities.json) (JavaScript Object Notation).

Additionally, this repo also contains the most current [JSON schema](https://github.com/cisagov/kev-data/blob/develop/known_exploited_vulnerabilities_schema.json) for the KEV data.

## Update schedule

https://github.com/cisagov/kev-data#update-schedule

This repository is updated whenever the [KEV](https://cisa.gov/KEV) is updated. Technically, this repo is updated shortly after the canonical source at cisa.gov is updated, which typically happens weekdays during normal US Eastern business hours when there are new or updated KEV entries. Users should expect both sources ([cisa.gov](https://cisa.gov/KEV) and [github.com](https://github.com/cisagov/kev-data/blob/main/README.md)) to be synchronized within minutes of each other.

The JSON schema will also remain in sync, though that file is not expected to be updated frequently (perhaps a few times per year).

## Contributing

https://github.com/cisagov/kev-data#contributing

We welcome contribution suggestions from the public to make this repository better and more accurate. Please see this repository's [CONTRIBUTING.md](https://github.com/cisagov/kev-data/blob/develop/CONTRIBUTING.md) for more detail. In summary, we expect issues and pull requests to address purely technical issues like typos in prose, broken links to remediation guidance, accidental schema violations, and the like.

Notably, this is not a repo for requesting additions or deletions of KEV entries. The KEV is managed by CISA directly, and CISA's requirements for KEV addition are fairly strict, relying on the authority of [BOD 26-04](https://www.cisa.gov/news-events/directives/bod-26-04-prioritizing-security-updates-based-risk).

Tips or suggestions about individual entries (beyond technical bug reports) should be directed via email to KEV@cisa.dhs.gov.

## Usage

https://github.com/cisagov/kev-data#usage

The purpose of this repo of KEV data is to enable easier usage of the KEV JSON and CSV files that CISA produces. GitHub provides a rich API for querying and downloading data sets, so oftentimes, code that is developed and maintained on GitHub (and beyond) has an easier time consuming data sources from GitHub than they might from US government websites.

In addition, the git commit history provided by GitHub can make tracking changes to data sources like the KEV easier and more transparent. The KEV, in particular, has no inline file revision history or log that's easily accessible after the fact. Related vulnerability tracking projects, such as the [CVE List](https://github.com/CVEProject/cvelistV5) and [Vulnrichment](https://github.com/cisagov/vulnrichment) benefit from this kind of public logging and public issue tracking functionality.

## Licensing

https://github.com/cisagov/kev-data#licensing

This data repository is licensed under the CC0 license, which allows for universal public domain use of the information here. This is identical to the licensing found at https://www.cisa.gov/sites/default/files/licenses/kev/license.txt.

## About

Mirror of cisa.gov/kev data files

### Resources

[Readme](https://github.com/cisagov/kev-data#readme-ov-file)

[License](https://github.com/cisagov/kev-data#License-1-ov-file)

### Contributing

[Contributing](https://github.com/cisagov/kev-data#contributing-ov-file)

### Security policy

[Security policy](https://github.com/cisagov/kev-data#security-ov-file)

[Activity](https://github.com/cisagov/kev-data/activity)

[Custom properties](https://github.com/cisagov/kev-data/custom-properties)

### Stars

136 stars

### Watchers

18 watching

### Forks

[31 forks](https://github.com/cisagov/kev-data/forks)

[Report repository](https://github.com/contact/report-content?content_url=https%3A%2F%2Fgithub.com%2Fcisagov%2Fkev-data&report=cisagov+%28user%29)

## Releases

## Packages

## Used by

## Contributors

## Languages
