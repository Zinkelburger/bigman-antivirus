---
url: "https://github.com/ossf/package-analysis"
title: "GitHub - ossf/package-analysis: Open Source Package Analysis"
site: "GitHub"
captured: "2026-11-02T19:07:03Z"
via: "live"
sha256: "3c2b921f223e6500bfae16e1f7e0ad7897ebed5a7597100e9b2fab2479d20249"
wayback: "https://web.archive.org/web/20261023012158/https://github.com/ossf/package-analysis"
cited_in:
  - "REVIEW.md"
  - "docs/threats/02-supply-chain-packages.md"
---

# GitHub - ossf/package-analysis: Open Source Package Analysis

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/ossf/package-analysis>
> Snapshot: <https://web.archive.org/web/20261023012158/https://github.com/ossf/package-analysis>

https://api.securityscorecards.dev/projects/github.com/ossf/package-analysis

# Package Analysis

https://github.com/ossf/package-analysis#package-analysis

The Package Analysis project analyses the capabilities of packages available on open source repositories. The project looks for behaviors that indicate malicious software:

- What files do they access?
- What addresses do they connect to?
- What commands do they run?

The project also tracks changes in how packages behave over time, to identify when previously safe software begins acting suspiciously.

This effort is meant to improve the security of open source software by detecting malicious behavior, informing consumers selecting packages, and providing researchers with data about the ecosystem.

This code is designed to work with the [Package Feeds](https://github.com/ossf/package-feeds) project, and originally started there.

For examples of what this project has detected, check out the [case studies](https://github.com/ossf/package-analysis/blob/main/docs/case_studies.md).

## How it works

https://github.com/ossf/package-analysis#how-it-works

The project's components are:

- A [scheduler](https://github.com/ossf/package-analysis/blob/main/cmd/scheduler) - creates jobs for the analysis worker from Package Feeds.
- Analysis (one-shot [analyze](https://github.com/ossf/package-analysis/blob/main/cmd/analyze) and [worker](https://github.com/ossf/package-analysis/blob/main/cmd/worker)) - collects package behavior data through static and dynamic analysis of each package.
- A [loader](https://github.com/ossf/package-analysis/blob/main/function/loader) - pushes the analysis results into BigQuery.

The goal is for all of these components to work together and provide extensible, community-run infrastructure to study behavior of open source packages and to look for malicious software. We also hope that the components can be used independently, to provide package feeds or runtime behavior data for anyone interested.

The Package Analysis project currently consists of the following pipeline:

https://github.com/ossf/package-analysis/blob/main/docs/images/Pipeline%20diagram.png

- Package repositories are monitored for new packages.
- Each new package is scheduled to be analyzed by a pool of workers.
- A worker performs dynamic analysis of the package inside a sandbox.
- Results are stored and imported into BigQuery for inspection.

Sandboxing via [gVisor](https://gvisor.dev/) containers ensures the packages are isolated. Detonating a package inside the sandbox allows us to capture strace and packet data that can indicate malicious interactions with the system as well as network connections that can be used to leak sensitive data or allow remote access.

## Public Data

https://github.com/ossf/package-analysis#public-data

This data is available in the public [BigQuery dataset](https://console.cloud.google.com/bigquery?d=packages&p=ossf-malware-analysis&t=analysis&page=table).

## Configuration

https://github.com/ossf/package-analysis#configuration

Configuration for these subprojects consist of a collection of environment variables for the various endpoints. These endpoints are configured using goclouddev compatible URL strings. In these cases, documentation will be linked to and DRIVER-Constructor sections should be ignored in favour of DRIVER sections as these are appropriate to the configurations in place throughout these subprojects. Note that not all drivers will be supported but they can be added quite simply with a minor patch to the repository. See the addition of kafka for scheduler in [one line](https://github.com/ossf/package-analysis/commit/985ab76a67d29d2fc8582b3920643e7eb963da8a#diff-8565ef29cfb886db7902792675eddce1e7a0ccfe33428a59e7f2e365b354af88R12).

An example of these variables can be found in the [e2e example docker-compose](https://github.com/ossf/package-analysis/blob/main/configs/e2e/docker-compose.yml).

### Analysis

https://github.com/ossf/package-analysis#analysis

OSSMALWARE_WORKER_SUBSCRIPTION - Can be used to set the subscription URL for the data coming out of scheduler. Values should follow [goclouddev subscriptions](https://gocloud.dev/howto/pubsub/subscribe/).

OSSF_MALWARE_ANALYSIS_RESULTS - OPTIONAL: Can be used to set the bucket URL to publish results to. Values should follow [goclouddev buckets](https://gocloud.dev/howto/blob/).

OSSF_MALWARE_ANALYSIS_PACKAGES - OPTIONAL: Can be used to set the bucket URL to get custom uploaded packages from. Values should follow [goclouddev buckets](https://gocloud.dev/howto/blob/).

OSSF_MALWARE_NOTIFICATION_TOPIC - OPTIONAL: Can be used to set the topic URL to publish messages for consumption after a new package analysis is complete. Values should follow [goclouddev publishing](https://gocloud.dev/howto/pubsub/publish/).

### Scheduler

https://github.com/ossf/package-analysis#scheduler

OSSMALWARE_WORKER_TOPIC - Can be used to set the topic URL to publish data for consumption by Analysis workers. Values should follow [goclouddev publishing](https://gocloud.dev/howto/pubsub/publish/).

OSSMALWARE_SUBSCRIPTION_URL - Can be used to set the subscription URL for the data coming out of [package-feeds](https://github.com/ossf/package-feeds). Values should follow [goclouddev subscriptions](https://gocloud.dev/howto/pubsub/subscribe/).

## Local Analysis

https://github.com/ossf/package-analysis#local-analysis

To run the analysis code locally, the easiest way is to use the Docker image gcr.io/ossf-malware-analysis/analysis. This can be built with make build/image/analysis, or the public images can be used instead.

This container uses podman to run a nested, sandboxed ([gVisor](https://gvisor.dev/)) container for analysis.

The commands below will dump the JSON results to /tmp/results and full logs to /tmp/dockertmp.

### Live package

https://github.com/ossf/package-analysis#live-package

To run this on a live package (e.g. the latest version of the "Django" package on [pypi.org](https://pypi.org))

```
$ scripts/run_analysis.sh -ecosystem pypi -package Django
```

Or with a specific version

```
$ scripts/run_analysis.sh -ecosystem pypi -package Django -version 4.1.3
```

### Local package

https://github.com/ossf/package-analysis#local-package

To run analysis on a local PyPi package named 'test', located in local archive /path/to/test.whl

```
$ scripts/run_analysis.sh -ecosystem pypi -package test -local /path/to/test.whl
```

### Docker notes

https://github.com/ossf/package-analysis#docker-notes

(Note: these options are handled by the scripts/run_analysis.sh script).

--privileged and a compatible filesystem are required to properly run nested containers. -v /var/lib/containers:/var/lib/containers is also used as it allows caching the sandbox images and supports local developement.

## Development

https://github.com/ossf/package-analysis#development

### Testing

https://github.com/ossf/package-analysis#testing

See sample_packages/README.md for how to use a sample package that simulates malicious activity for testing purposes.

### Required Dependencies

https://github.com/ossf/package-analysis#required-dependencies

- Go v1.26.3
- Docker

# Contributing

https://github.com/ossf/package-analysis#contributing

If you want to get involved or have ideas you'd like to chat about, we discuss this project in the [OSSF Securing Critical Projects Working Group](https://github.com/ossf/wg-securing-critical-projects) meetings.

See the [Community Calendar](https://calendar.google.com/calendar?cid=czYzdm9lZmhwNWk5cGZsdGI1cTY3bmdwZXNAZ3JvdXAuY2FsZW5kYXIuZ29vZ2xlLmNvbQ) for the schedule and meeting invitations.
