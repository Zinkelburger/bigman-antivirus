---
url: "https://github.com/praetorian-inc/noseyparker"
title: "GitHub - praetorian-inc/noseyparker: Nosey Parker is a command-line tool that finds secrets and sensitive information in textual data and Git history."
site: "GitHub"
captured: "2026-11-02T19:07:08Z"
via: "live"
sha256: "27fa9d4686c51dbeb2530b624918a4c6e66e28bdde23b091eda9f2e2274d9bf0"
wayback: "https://web.archive.org/web/20260729001541/https://github.com/praetorian-inc/noseyparker"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# GitHub - praetorian-inc/noseyparker: Nosey Parker is a command-line tool that finds secrets and sensitive information in textual data and Git history.

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/praetorian-inc/noseyparker>
> Snapshot: <https://web.archive.org/web/20260729001541/https://github.com/praetorian-inc/noseyparker>

# Nosey Parker is now Replaced by Titus. Nosey Parker is Officially Retired

https://github.com/praetorian-inc/noseyparker#nosey-parker-is-now-replaced-by-titus-nosey-parker-is-officially-retired

See our announcement post https://www.praetorian.com/blog/titus-open-source-secret-scanner/. The Titus Repo can be found at https://github.com/praetorian-inc/titus

## Overview

https://github.com/praetorian-inc/noseyparker#overview

Nosey Parker is a CLI tool that finds secrets and sensitive information in textual data. It is essentially a special-purpose grep-like tool for detection of secrets.

It has been designed for offensive security (e.g., enabling lateral movement on red teams), but it can also be useful for defensive security testing. It has found secrets in hundreds of offensive security engagements at [Praetorian](https://praetorian.com).

Key features:

- Flexiblity: It natively scans files, directories, GitHub, and Git history, and has an extensible input enumeration mechanism
- Field-tested rules: It uses regular expressions with [188 rules](https://github.com/praetorian-inc/noseyparker/blob/main/crates/noseyparker/data/default/builtin/rules) chosen for high precision based on feedback from security engineers
- Signal-to-noise: It deduplicates matches that share the same secret, reducing review burden by 10-1000x or more
- Speed & scalability: it can scan at GB/s on a multicore system, and has scanned inputs as large as 20TB during security engagements

The typical workflow is three phases:

- Scan inputs of interest using the scan command
- Report details of scan results using the report command
- Review and triage findings

## Installation

https://github.com/praetorian-inc/noseyparker#installation

### [Homebrew](https://brew.sh) formula

https://github.com/praetorian-inc/noseyparker#homebrew-formula

```
brew install noseyparker
```

### Prebuilt binaries

https://github.com/praetorian-inc/noseyparker#prebuilt-binaries

The [latest release page](https://github.com/praetorian-inc/noseyparker/releases/latest) contains prebuilt binaries for x86_64/aarch64 Linux and macOS.

### Docker: x86_64/aarch64

https://github.com/praetorian-inc/noseyparker#docker-x86_64aarch64

```
docker pull ghcr.io/praetorian-inc/noseyparker:latest
```

The most recent commit is also available via the main tag.

### Docker: x86_64/aarch64, Alpine base:

https://github.com/praetorian-inc/noseyparker#docker-x86_64aarch64-alpine-base

```
docker pull ghcr.io/praetorian-inc/noseyparker-alpine:latest
```

The most recent commit is also available via the main tag.

### Arch Linux package

https://github.com/praetorian-inc/noseyparker#arch-linux-package

https://aur.archlinux.org/packages/noseyparker

### Windows

https://github.com/praetorian-inc/noseyparker#windows

Nosey Parker does not build natively on Windows ([#121](https://github.com/praetorian-inc/noseyparker/issues/121)). It is possible to run on Windows using [WSL1](https://en.wikipedia.org/wiki/Windows_Subsystem_for_Linux) and the native Linux release.

### Building from source

https://github.com/praetorian-inc/noseyparker#building-from-source

Details

#### 1. Install prerequisites

https://github.com/praetorian-inc/noseyparker#1-install-prerequisites

This has been tested with several versions of Ubuntu Linux and macOS on both x86_64 and aarch64.

Required dependencies:

- cargo: recommended approach: install from https://rustup.rs
- cmake: needed for building the vectorscan-sys crate and some other dependencies
- boost: needed for building the vectorscan-sys crate (supported version >=1.57)
- git: needed for embedding version information into the noseyparker CLI
- patch: needed for building the vectorscan-sys crate
- pkg-config: needed for building the vectorscan-sys crate
- sha256sum: needed for computing digests (often provided by the coreutils package)
- zsh: needed for build scripts

#### 2. Build using the [create-release.zsh](https://github.com/praetorian-inc/noseyparker/blob/main/scripts/create-release.zsh) script

https://github.com/praetorian-inc/noseyparker#2-build-using-the-create-releasezsh-script

```
$ rm -rf release && ./scripts/create-release.zsh
```

If successful, this will produce a directory structure at release populated with release artifacts. The command-line program will be at release/bin/noseyparker.

## Getting help

https://github.com/praetorian-inc/noseyparker#getting-help

Running the noseyparker binary without arguments prints top-level help and exits. You can get abbreviated help for a particular command by running noseyparker COMMAND -h. More detailed help is available with the help command or long-form --help option.

The prebuilt releases also include manpages that collect the command-line help in one place. These manpages converted into Markdown format are also included in the repository [here](https://github.com/praetorian-inc/noseyparker/blob/main/docs/v0.17.0/man/man1).

If you have a question that's not answered by this documentation, please [start a discussion](https://github.com/praetorian-inc/noseyparker/discussions/new/choose).

## Terminology and data model

https://github.com/praetorian-inc/noseyparker#terminology-and-data-model

### The datastore

https://github.com/praetorian-inc/noseyparker#the-datastore

The datastore is a special directory that Nosey Parker uses to record its findings and maintain its internal state. A datastore will be implicitly created by the scan command if needed.

### Blobs

https://github.com/praetorian-inc/noseyparker#blobs

Each scanned input is called a blob. Each blob has a unique blob ID, which is a SHA-1 digest computed the same way git does.

### Provenance

https://github.com/praetorian-inc/noseyparker#provenance

Each blob has one or more provenance entries associated with it. A provenance entry is metadata that describes how the input was discovered, such as a file on the filesystem or a file in Git repository history.

### Rules

https://github.com/praetorian-inc/noseyparker#rules

Nosey Parker is a rule-based system that uses regular expressions. Each rule has a single pattern with at least one capture group that isolates the match content from the surrounding context. You can list available rules with noseyparker rules list.

### Rulesets

https://github.com/praetorian-inc/noseyparker#rulesets

A collection of rules is organized into a ruleset. Nosey Parker's default ruleset includes rules that detect things that appear to be secrets. Other rulesets are available; you can list them with noseyparker rules list.

### Matches

https://github.com/praetorian-inc/noseyparker#matches

When a rule's pattern matches an input, it produces a match. A match is uniquely defined by a rule, blob ID, start byte offset, and end byte offset; these fields are used to compute a unique match identifier.

### Findings

https://github.com/praetorian-inc/noseyparker#findings

Matches that share a rule and capture groups are combined into a finding. In other words, a finding is a group of matches. This is Nosey Parker's top-level unit of reporting.

## Usage examples

https://github.com/praetorian-inc/noseyparker#usage-examples

### NOTE: When using Docker...

https://github.com/praetorian-inc/noseyparker#note-when-using-docker

When using the Docker image, replace noseyparker in the following commands with a Docker invocation that uses a mounted volume:

```
docker run -v "$PWD":/scan ghcr.io/praetorian-inc/noseyparker:latest <ARGS>
```

The Docker container runs with /scan as its working directory, so mounting $PWD at /scan in the container will make tab completion and relative paths in your command-line invocation work.

### Scan filesystem content, including local Git repos

https://github.com/praetorian-inc/noseyparker#scan-filesystem-content-including-local-git-repos

https://github.com/praetorian-inc/noseyparker/blob/main/docs/usage-examples/gifs/02-scan-git-history.gif

Nosey Parker has native support for scanning files, directories, and the entire history of Git repositories.

For example, if you have a Git clone of [CPython](https://github.com/python/cpython) locally at cpython.git, you can scan it with the scan command. Nosey Parker will create a new datastore at cpython.np and saves its findings there. (The name cpython.np is innessential, and can be whatever you want.)

```
$ noseyparker scan -d cpython.np cpython.git
Scanned 19.19 GiB from 335,849 blobs in 17 seconds (1.11 GiB/s); 2,178/2,178 new matches

 Rule                            Findings   Matches   Accepted   Rejected   Mixed   Unlabeled
──────────────────────────────────────────────────────────────────────────────────────────────
 Generic API Key                        1         8          0          0       0           1
 Generic Password                       8     1,283          0          0       0           8
 Generic Username and Password          2        40          0          0       0           2
 HTTP Bearer Token                      1       108          0          0       0           1
 PEM-Encoded Private Key               61       151          0          0       0          61
 netrc Credentials                     27       588          0          0       0          27

Run the `report` command next to show finding details.

```

See noseyparker help scan for more details.

### Scan a Git repo from an HTTPS URL

https://github.com/praetorian-inc/noseyparker#scan-a-git-repo-from-an-https-url

For example, to scan the Nosey Parker repo itself:

```
noseyparker scan --datastore np.noseyparker --git-url https://github.com/praetorian-inc/noseyparker

```

See noseyparker help scan for more details.

### Scan Git repos of a GitHub user or organization

https://github.com/praetorian-inc/noseyparker#scan-git-repos-of-a-github-user-or-organization

Use --github-user=USER or --github-org=ORG. For example, to scan accessible repositories belonging to the [octocat](https://github.com/octocat) user:

```
noseyparker scan --datastore np.noseyparker --github-user octocat

```

These input specifiers will use an optional GitHub token if available in the NP_GITHUB_TOKEN environment variable. Providing an access token gives a higher API rate limit and may make additional repositories accessible to you.

See noseyparker help scan for more details.

### Interactively review and annotate findings

https://github.com/praetorian-inc/noseyparker#interactively-review-and-annotate-findings

See the companion project, [Nosey Parker Explorer](https://github.com/praetorian-inc/noseyparkerexplorer): https://github.com/praetorian-inc/noseyparkerexplorer/blob/32e9133600c79eee53cd9000e37b71792e555fdd/docs/img/main-screen.png?raw=true

### Report findings in human-readable text format

https://github.com/praetorian-inc/noseyparker#report-findings-in-human-readable-text-format

https://github.com/praetorian-inc/noseyparker/blob/main/docs/usage-examples/gifs/03-report-human.gif

### Report findings in JSON format

https://github.com/praetorian-inc/noseyparker#report-findings-in-json-format

https://github.com/praetorian-inc/noseyparker/blob/main/docs/usage-examples/gifs/04-report-json.gif

### Summarize findings

https://github.com/praetorian-inc/noseyparker#summarize-findings

Nosey Parker prints out a summary of its findings when it finishes scanning. You can also run this step separately after scanning:

```
$ noseyparker summarize --datastore np.cpython

 Rule                      Distinct Groups   Total Matches
───────────────────────────────────────────────────────────
 PEM-Encoded Private Key             1,076           1,192
 Generic Secret                        331             478
 netrc Credentials                      42           3,201
 Generic API Key                         2              31
 md5crypt Hash                           1               2

```

Additional output formats are supported, including JSON and JSON lines, via the --format=FORMAT option.

See noseyparker help summarize for more details.

### Enumerate repositories from GitHub

https://github.com/praetorian-inc/noseyparker#enumerate-repositories-from-github

Use github repos list command to list URLs for repositories belonging to GitHub users or organizations. This command uses the GitHub REST API to enumerate repositories belonging to users or organizations. For example:

```
$ noseyparker github repos list --user octocat
https://github.com/octocat/Hello-World.git
https://github.com/octocat/Spoon-Knife.git
https://github.com/octocat/boysenberry-repo-1.git
https://github.com/octocat/git-consortium.git
https://github.com/octocat/hello-worId.git
https://github.com/octocat/linguist.git
https://github.com/octocat/octocat.github.io.git
https://github.com/octocat/test-repo1.git

```

This command will use an optional GitHub token if available in the NP_GITHUB_TOKEN environment variable. Providing an access token gives a higher API rate limit and may make additional repositories accessible to you.

Additional output formats are supported, including JSON and JSON lines, via the --format=FORMAT option.

See noseyparker help github for more details.

## Integrations

https://github.com/praetorian-inc/noseyparker#integrations

Nosey Parker has a few third-party integrations:

- Nosey Parker is packaged in [Homebrew](https://formulae.brew.sh/formula/noseyparker)
- Nosey Parker is packaged in [Arch Linux](https://aur.archlinux.org/packages/noseyparker)
- A [GitHub Action](https://github.com/bpsizemore/noseyparker-action) that runs Nosey Parker is available
- [DefectDojo](https://defectdojo.org) includes a [parser for Nosey Parker v0.16 JSON](https://github.com/DefectDojo/django-DefectDojo/blob/c182e9ca9d8f981c15de2018f948fe69c4d1a800/docs/content/en/integrations/parsers/file/noseyparker.md)
- [Nemesis](https://github.com/SpecterOps/Nemesis) includes support for Nosey Parker
- [noseyparker-compact](https://github.com/metaory/noseyparker-compact) provides compact report postprocessors
- [segfault.net disposable root servers](https://www.thc.org/segfault/) come with Nosey Parker preinstalled

If you have an integration you'd like to share that's not listed here, please create a PR.

## Contributing

https://github.com/praetorian-inc/noseyparker#contributing

Ask questions or share ideas in the [Discussions](https://github.com/praetorian-inc/noseyparker/discussions) area.

Contributions are welcome, especially new regex rules. Developing new regex rules is detailed in a [separate document](https://github.com/praetorian-inc/noseyparker/blob/main/docs/RULES.md).

If you are considering making significant code changes, please [open an issue](https://github.com/praetorian-inc/noseyparker/issues/new) or [start a discussion](https://github.com/praetorian-inc/noseyparker/discussions/new/choose) first.

This project has a number of [pre-commit](https://pre-commit.com/) hooks enabled that you are encouraged to use. To install them in your local repo, make sure you have pre-commit installed and run:

```
$ pre-commit install

```

These checks will help to quickly detect simple errors.

## License

https://github.com/praetorian-inc/noseyparker#license

Nosey Parker is licensed under the [Apache License, Version 2.0](https://github.com/praetorian-inc/noseyparker/blob/main/LICENSE).
