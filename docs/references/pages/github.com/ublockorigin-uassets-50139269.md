---
url: "https://github.com/uBlockOrigin/uAssets"
title: "GitHub - uBlockOrigin/uAssets: Filter lists for uBlock Origin & uBlock Origin Lite"
site: "GitHub"
captured: "2026-11-02T19:08:51Z"
via: "live"
sha256: "62656bee62310cacd8e711e8137929cea6192ddd69fb1fdae9194b52cd66a370"
wayback: "https://web.archive.org/web/20261023144206/https://github.com/uBlockOrigin/uAssets"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# GitHub - uBlockOrigin/uAssets: Filter lists for uBlock Origin & uBlock Origin Lite

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/uBlockOrigin/uAssets>
> Snapshot: <https://web.archive.org/web/20261023144206/https://github.com/uBlockOrigin/uAssets>

## Repository navigation

[uAssets](https://github.com/uBlockOrigin/uAssets)

Public

- [Notifications](https://github.com/login?return_to=%2FuBlockOrigin%2FuAssets)
- [Fork (1.1k)](https://github.com/login?return_to=%2FuBlockOrigin%2FuAssets)
- [Star (6.1k)](https://github.com/login?return_to=%2FuBlockOrigin%2FuAssets)

## About

Filter lists for uBlock Origin & uBlock Origin Lite

### Resources

[Readme](https://github.com/uBlockOrigin/uAssets#readme-ov-file)

[GPL-3.0 license](https://github.com/uBlockOrigin/uAssets#GPL-3.0-1-ov-file)

### Contributing

[Contributing](https://github.com/uBlockOrigin/uAssets#contributing-ov-file)

[Activity](https://github.com/uBlockOrigin/uAssets/activity)

[Custom properties](https://github.com/uBlockOrigin/uAssets/custom-properties)

### Stars

6.1k stars

### Watchers

134 watching

### Forks

[1.1k forks](https://github.com/uBlockOrigin/uAssets/forks)

[Report repository](https://github.com/contact/report-content?content_url=https%3A%2F%2Fgithub.com%2FuBlockOrigin%2FuAssets&report=uBlockOrigin+%28user%29)

[Branches](https://github.com/uBlockOrigin/uAssets/branches)[Tags](https://github.com/uBlockOrigin/uAssets/tags)

https://github.com/uBlockOrigin/uAssets/brancheshttps://github.com/uBlockOrigin/uAssets/tags

## Latest commit

## History

[96,452 Commits](https://github.com/uBlockOrigin/uAssets/commits/master/)

https://github.com/uBlockOrigin/uAssets/commits/master/

## Folders and files

| Name | Name |

Last commit message

|

Last commit date

|

[.github](https://github.com/uBlockOrigin/uAssets/tree/master/.github)

|

[.github](https://github.com/uBlockOrigin/uAssets/tree/master/.github)

|

|

|

[dnr](https://github.com/uBlockOrigin/uAssets/tree/master/dnr)

|

[dnr](https://github.com/uBlockOrigin/uAssets/tree/master/dnr)

|

|

|

[filters](https://github.com/uBlockOrigin/uAssets/tree/master/filters)

|

[filters](https://github.com/uBlockOrigin/uAssets/tree/master/filters)

|

|

|

[templates](https://github.com/uBlockOrigin/uAssets/tree/master/templates)

|

[templates](https://github.com/uBlockOrigin/uAssets/tree/master/templates)

|

|

|

[thirdparties](https://github.com/uBlockOrigin/uAssets/tree/master/thirdparties)

|

[thirdparties](https://github.com/uBlockOrigin/uAssets/tree/master/thirdparties)

|

|

|

[tools](https://github.com/uBlockOrigin/uAssets/tree/master/tools)

|

[tools](https://github.com/uBlockOrigin/uAssets/tree/master/tools)

|

|

|

[.gitattributes](https://github.com/uBlockOrigin/uAssets/blob/master/.gitattributes)

|

[.gitattributes](https://github.com/uBlockOrigin/uAssets/blob/master/.gitattributes)

|

|

|

[.gitignore](https://github.com/uBlockOrigin/uAssets/blob/master/.gitignore)

|

[.gitignore](https://github.com/uBlockOrigin/uAssets/blob/master/.gitignore)

|

|

|

[CONTRIBUTING.md](https://github.com/uBlockOrigin/uAssets/blob/master/CONTRIBUTING.md)

|

[CONTRIBUTING.md](https://github.com/uBlockOrigin/uAssets/blob/master/CONTRIBUTING.md)

|

|

|

[LICENSE](https://github.com/uBlockOrigin/uAssets/blob/master/LICENSE)

|

[LICENSE](https://github.com/uBlockOrigin/uAssets/blob/master/LICENSE)

|

|

|

[Makefile](https://github.com/uBlockOrigin/uAssets/blob/master/Makefile)

|

[Makefile](https://github.com/uBlockOrigin/uAssets/blob/master/Makefile)

|

|

|

[README.md](https://github.com/uBlockOrigin/uAssets/blob/master/README.md)

|

[README.md](https://github.com/uBlockOrigin/uAssets/blob/master/README.md)

|

|

|

## Repository files navigation

# uAssets

https://github.com/uBlockOrigin/uAssets#uassets

This repository is for the resources of [uBlock Origin (uBO)](https://github.com/gorhill/uBlock). It receives all reports for new filters or existing filters that cause web page breakage. Any contributors are welcome. Contributors who are proven valuable will get write permissions to the repository.

The rationale for including a specific filter in uBO's filter lists is the same as the [EasyList/EasyPrivacy policies](https://easylist.to/pages/policy.html) and also takes into account whether a filter requires uBO's extended filter syntax.

It is preferred to fix filter issues in EasyList. Any filters included in uBO's filter lists must use the [extended syntax](https://github.com/gorhill/uBlock/wiki/Static-filter-syntax#extended-syntax).

The EasyList-compatible fixes for high-traffic websites are added to uBO filters until they become added to EasyList.

#### uAssets will fix the following exceptions even if they do not require using the extended syntax:

https://github.com/uBlockOrigin/uAssets#uassets-will-fix-the-following-exceptions-even-if-they-do-not-require-using-the-extended-syntax

- Ad-Reinsertion
- Anti-Blocker
- Context Menu Blockage
- Cut/Copy/Paste Blockage
- Popups/Popunders
- Website Breakage
- Video Ads

#### uAssets will not address the following:

https://github.com/uBlockOrigin/uAssets#uassets-will-not-address-the-following

- Paywalls
- Porn Farms
- Annoyances (widgets, social media buttons, newsletter subscriptions, donation requests, etc)
- Self-promotion elements
- Requests for custom filters to modify or restyle site content (for help with these, visit [/r/uBlockOrigin](https://www.reddit.com/r/uBlockOrigin/)).

#### How to correctly report an issue:

https://github.com/uBlockOrigin/uAssets#how-to-correctly-report-an-issue

-

Disable all other browser extensions and see if the issue still occurs

-

If the issue still occurs when all other browser extensions are disabled, then provide the troubleshooting information:

-

Steps for uBO:

- Open a new browser tab
- Go to the specific page where the issue occurs
- Click the 🛡️ uBO icon
- Click the 💬 chat icon
- Click Troubleshooting Information
- Click Select all and copy the contents
- Paste the contents into the appropriate GitHub thread

-

Steps for uBO Lite:

- Open a new browser tab
- Go to the specific page where the issue occurs
- Click the 🛡️ uBO Lite icon
- Click the menu item for 💬 Report an issue
- Click Troubleshooting Information
- Copy the contents
- Paste the contents into the appropriate GitHub thread

- Always provide a clear description of the issue and the steps necessary to reproduce it (including if an account/login is required)

#### Filter List Requests

https://github.com/uBlockOrigin/uAssets#filter-list-requests

New filter list requests in the issue tracker are not permitted and will be declined and closed.

#### Support Forum

https://github.com/uBlockOrigin/uAssets#support-forum

For support, questions, or help, visit [/r/uBlockOrigin](https://www.reddit.com/r/uBlockOrigin/).

#### uBO Issues

https://github.com/uBlockOrigin/uAssets#ubo-issues

Report issues with uBO in the [uBO issue tracker](https://github.com/uBlockOrigin/uBlock-issues/issues).

#### uBO Lite (uBOL) Issues

https://github.com/uBlockOrigin/uAssets#ubo-lite-ubol-issues

Report issues specific to the Manifest Version 3 (MV3) variant in the [uBOL issue tracker](https://github.com/uBlockOrigin/uBOL-home/issues).

#### Similarly-Purposed Blockers

https://github.com/uBlockOrigin/uAssets#similarly-purposed-blockers

Do NOT use any other [similarly-purposed blockers](https://x.com/gorhill/status/1033706103782170625) concurrently with uBO. It can result in website breakage or undefined results.

## About

Filter lists for uBlock Origin & uBlock Origin Lite

### Resources

[Readme](https://github.com/uBlockOrigin/uAssets#readme-ov-file)

[GPL-3.0 license](https://github.com/uBlockOrigin/uAssets#GPL-3.0-1-ov-file)

### Contributing

[Contributing](https://github.com/uBlockOrigin/uAssets#contributing-ov-file)

[Activity](https://github.com/uBlockOrigin/uAssets/activity)

[Custom properties](https://github.com/uBlockOrigin/uAssets/custom-properties)

### Stars

6.1k stars

### Watchers

134 watching

### Forks

[1.1k forks](https://github.com/uBlockOrigin/uAssets/forks)

[Report repository](https://github.com/contact/report-content?content_url=https%3A%2F%2Fgithub.com%2FuBlockOrigin%2FuAssets&report=uBlockOrigin+%28user%29)

## Releases

## Packages

## Used by

## Contributors

## Languages
