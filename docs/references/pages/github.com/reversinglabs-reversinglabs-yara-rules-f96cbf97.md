---
url: "https://github.com/reversinglabs/reversinglabs-yara-rules"
title: "GitHub - reversinglabs/reversinglabs-yara-rules: ReversingLabs YARA Rules"
site: "GitHub"
captured: "2026-11-02T19:07:37Z"
via: "live"
sha256: "7f1196764aab6d68fe344505a4632194e0d921268250e3dfff7f28ebc0f35115"
wayback: "https://web.archive.org/web/20261002075212/https://github.com/reversinglabs/reversinglabs-yara-rules"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# GitHub - reversinglabs/reversinglabs-yara-rules: ReversingLabs YARA Rules

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/reversinglabs/reversinglabs-yara-rules>
> Snapshot: <https://web.archive.org/web/20261002075212/https://github.com/reversinglabs/reversinglabs-yara-rules>

## Repository navigation

[reversinglabs-yara-rules](https://github.com/reversinglabs/reversinglabs-yara-rules)

Public

- [Notifications](https://github.com/login?return_to=%2Freversinglabs%2Freversinglabs-yara-rules)
- [Fork (123)](https://github.com/login?return_to=%2Freversinglabs%2Freversinglabs-yara-rules)
- [Star (952)](https://github.com/login?return_to=%2Freversinglabs%2Freversinglabs-yara-rules)

## About

ReversingLabs YARA Rules

[www.reversinglabs.com](http://www.reversinglabs.com)

### Topics

[indicators-of-compromise](https://github.com/topics/indicators-of-compromise)[malware-detection](https://github.com/topics/malware-detection)[ransomware-detection](https://github.com/topics/ransomware-detection)[ransomware-prevention](https://github.com/topics/ransomware-prevention)[reverse-engineering](https://github.com/topics/reverse-engineering)[yara](https://github.com/topics/yara)[yara-rules](https://github.com/topics/yara-rules)[yara-signatures](https://github.com/topics/yara-signatures)

### Resources

[Readme](https://github.com/reversinglabs/reversinglabs-yara-rules#readme-ov-file)

[MIT license](https://github.com/reversinglabs/reversinglabs-yara-rules#MIT-1-ov-file)

[Activity](https://github.com/reversinglabs/reversinglabs-yara-rules/activity)

[Custom properties](https://github.com/reversinglabs/reversinglabs-yara-rules/custom-properties)

### Stars

952 stars

### Watchers

73 watching

### Forks

[123 forks](https://github.com/reversinglabs/reversinglabs-yara-rules/forks)

[Report repository](https://github.com/contact/report-content?content_url=https%3A%2F%2Fgithub.com%2Freversinglabs%2Freversinglabs-yara-rules&report=reversinglabs+%28user%29)

[Branches](https://github.com/reversinglabs/reversinglabs-yara-rules/branches)[Tags](https://github.com/reversinglabs/reversinglabs-yara-rules/tags)

https://github.com/reversinglabs/reversinglabs-yara-rules/brancheshttps://github.com/reversinglabs/reversinglabs-yara-rules/tags

## Latest commit

## History

[110 Commits](https://github.com/reversinglabs/reversinglabs-yara-rules/commits/develop/)

https://github.com/reversinglabs/reversinglabs-yara-rules/commits/develop/

## Folders and files

| Name | Name |

Last commit message

|

Last commit date

|

[yara](https://github.com/reversinglabs/reversinglabs-yara-rules/tree/develop/yara)

|

[yara](https://github.com/reversinglabs/reversinglabs-yara-rules/tree/develop/yara)

|

|

|

[LICENSE](https://github.com/reversinglabs/reversinglabs-yara-rules/blob/develop/LICENSE)

|

[LICENSE](https://github.com/reversinglabs/reversinglabs-yara-rules/blob/develop/LICENSE)

|

|

|

[README.md](https://github.com/reversinglabs/reversinglabs-yara-rules/blob/develop/README.md)

|

[README.md](https://github.com/reversinglabs/reversinglabs-yara-rules/blob/develop/README.md)

|

|

|

## Repository files navigation

# ReversingLabs YARA Rules

https://github.com/reversinglabs/reversinglabs-yara-rules#reversinglabs-yara-rules

Welcome to the official ReversingLabs YARA rules repository! The repository will be updated continuously, as we develop rules for new threats, and after their quality has been proven through testing in our cloud and other environments.

These rules have been written by our threat analysts, for threat hunters, incident responders, security analysts, and other defenders that could benefit from deploying high-quality threat detection YARA rules in their environment.

Our detection rules, as opposed to hunting rules, need to satisfy certain criteria to be eligible for deployment, namely:

- be as precise as possible, without losing detection quality
- aim to provide zero false-positive detections

In order for the rules to be easy to understand and maintain, we adopted the following set of goals:

- clearly named byte patterns
- readable and transparent conditions
- match unique malware functionality
- prefer code byte patterns over strings

To ensure the quality of our rules, we continuously and extensively test them in our cloud, on over 10B (and rising) unique binaries. Rules are evaluated on every layer to detect threats within layered objects, such as packed PE files, documents, and archives, among other things.

# Prerequisites

https://github.com/reversinglabs/reversinglabs-yara-rules#prerequisites

To successfully run the entire YARA rule set, you must have:

- YARA version >= 3.2.0
- PE and ELF modules enabled

(or any other security solution compliant with the requirements).

# Deployment

https://github.com/reversinglabs/reversinglabs-yara-rules#deployment

To start using our rules, just clone this repository, and start experimenting on your data sets. YARA rules found in this repository can be used in various environments, and the simplest setup is to use them through the standalone YARA executable, which can be found in the [official YARA repository](https://github.com/VirusTotal/yara). The rules can also be deployed in a large number of modern security solutions that offer YARA integration, such as YARA-enabled sandboxes, and other file analysis frameworks. However, to get the best results, it is advisable to use the rules through ReversingLabs’ Titanium Platform which offers native integration of these rules into its classification pipeline.

# License

https://github.com/reversinglabs/reversinglabs-yara-rules#license

This project is licensed under the MIT License - see the [LICENSE](https://github.com/reversinglabs/reversinglabs-yara-rules/blob/develop/LICENSE) file for details.

# Acknowledgements

https://github.com/reversinglabs/reversinglabs-yara-rules#acknowledgements

Thanks go to all the people who actively participate in the development of the YARA engine - without you, these rules would not be possible. Also, we’d like to thank everyone who participates in the YARA community, because you evolve the way YARA is used, improve how rules should be written, and are what makes our work worthwhile.

## About

ReversingLabs YARA Rules

[www.reversinglabs.com](http://www.reversinglabs.com)

### Topics

[indicators-of-compromise](https://github.com/topics/indicators-of-compromise)[malware-detection](https://github.com/topics/malware-detection)[ransomware-detection](https://github.com/topics/ransomware-detection)[ransomware-prevention](https://github.com/topics/ransomware-prevention)[reverse-engineering](https://github.com/topics/reverse-engineering)[yara](https://github.com/topics/yara)[yara-rules](https://github.com/topics/yara-rules)[yara-signatures](https://github.com/topics/yara-signatures)

### Resources

[Readme](https://github.com/reversinglabs/reversinglabs-yara-rules#readme-ov-file)

[MIT license](https://github.com/reversinglabs/reversinglabs-yara-rules#MIT-1-ov-file)

[Activity](https://github.com/reversinglabs/reversinglabs-yara-rules/activity)

[Custom properties](https://github.com/reversinglabs/reversinglabs-yara-rules/custom-properties)

### Stars

952 stars

### Watchers

73 watching

### Forks

[123 forks](https://github.com/reversinglabs/reversinglabs-yara-rules/forks)

[Report repository](https://github.com/contact/report-content?content_url=https%3A%2F%2Fgithub.com%2Freversinglabs%2Freversinglabs-yara-rules&report=reversinglabs+%28user%29)

## Releases

## Packages

## Used by

## Contributors

## Languages
