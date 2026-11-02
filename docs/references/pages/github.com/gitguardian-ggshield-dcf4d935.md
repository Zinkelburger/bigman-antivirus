---
url: "https://github.com/GitGuardian/ggshield"
title: "GitHub - GitGuardian/ggshield: Detect and validate 500+ types of hardcoded secrets with advanced checks. Use it as a pre-commit hook, GitHub Action, or CLI for proactive secret detection and security."
site: "GitHub"
captured: "2026-11-02T18:37:10Z"
via: "live"
sha256: "49a47061b130579293249998c0cfcae5a75679447a76efaf4a960ce91a7d112c"
wayback: "https://web.archive.org/web/20261001051036/https://github.com/GitGuardian/ggshield"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# GitHub - GitGuardian/ggshield: Detect and validate 500+ types of hardcoded secrets with advanced checks. Use it as a pre-commit hook, GitHub Action, or CLI for proactive secret detection and security.

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/GitGuardian/ggshield>
> Snapshot: <https://web.archive.org/web/20261001051036/https://github.com/GitGuardian/ggshield>

# [ggshield](https://github.com/GitGuardian/ggshield): protect your code with GitGuardian

https://github.com/GitGuardian/ggshield#ggshield-protect-your-code-with-gitguardian

https://pypi.org/project/ggshield/ https://hub.docker.com/r/gitguardian/ggshield https://github.com/GitGuardian/ggshield/blob/main/LICENSE https://github.com/GitGuardian/ggshield/stargazers https://github.com/GitGuardian/ggshield/actions https://codecov.io/gh/GitGuardian/ggshield/

Important

macOS: upgrading to 1.54.0 triggers a Keychain prompt for ggshield-py, a second binary that now reads your saved token as part of our performance improvements. Click Always Allow; it won't ask again.

ggshield is a CLI application that runs in your local environment or in a CI environment to help you detect more than 500+ types of secrets.

ggshield uses our [public API](https://api.gitguardian.com/docs) through [py-gitguardian](https://github.com/GitGuardian/py-gitguardian) to scan and detect potential vulnerabilities in files and other text content.

Only metadata such as call time, request size and scan mode is stored from scans using ggshield, therefore secrets will not be displayed on your dashboard and your files and secrets won't be stored.

# Table of Contents

https://github.com/GitGuardian/ggshield#table-of-contents

- [Installation](https://github.com/GitGuardian/ggshield#installation)

- [Install script (Recommended)](https://github.com/GitGuardian/ggshield#install-script-recommended)
- [macOS](https://github.com/GitGuardian/ggshield#macos)

- [Homebrew](https://github.com/GitGuardian/ggshield#homebrew)
- [Standalone .pkg package](https://github.com/GitGuardian/ggshield#standalone-pkg-package)

- [Linux](https://github.com/GitGuardian/ggshield#linux)

- [Deb and RPM packages](https://github.com/GitGuardian/ggshield#deb-and-rpm-packages)

- [Windows](https://github.com/GitGuardian/ggshield#windows)

- [Chocolatey](https://github.com/GitGuardian/ggshield#chocolatey)
- [MSI installer](https://github.com/GitGuardian/ggshield#msi-installer)
- [Standalone .zip archive](https://github.com/GitGuardian/ggshield#standalone-zip-archive)

- [All operating systems](https://github.com/GitGuardian/ggshield#all-operating-systems)

- [Using pipx](https://github.com/GitGuardian/ggshield#using-pipx)
- [Using pip](https://github.com/GitGuardian/ggshield#using-pip)

- [Initial setup](https://github.com/GitGuardian/ggshield#initial-setup)

- [Using ggshield auth login](https://github.com/GitGuardian/ggshield#using-ggshield-auth-login)
- [Manual setup](https://github.com/GitGuardian/ggshield#manual-setup)

- [Getting started](https://github.com/GitGuardian/ggshield#getting-started)

- [Secrets](https://github.com/GitGuardian/ggshield#secrets)
- [Migrating a legacy configuration file](https://github.com/GitGuardian/ggshield#migrating-a-legacy-configuration-file)

- [Integrations](https://github.com/GitGuardian/ggshield#integrations)

- [AI coding assistants](https://github.com/GitGuardian/ggshield#ai-coding-assistants)

- [Learn more](https://github.com/GitGuardian/ggshield#learn-more)
- [Output](https://github.com/GitGuardian/ggshield#output)
- [Related open source projects](https://github.com/GitGuardian/ggshield#related-open-source-projects)
- [License](https://github.com/GitGuardian/ggshield#license)

# Installation

https://github.com/GitGuardian/ggshield#installation

## Install script (Recommended)

https://github.com/GitGuardian/ggshield#install-script-recommended

The quickest way to install ggshield.

Linux / macOS:

```
curl -sSfL \
  https://raw.githubusercontent.com/GitGuardian/ggshield/main/scripts/install/install.sh | bash
```

Windows (PowerShell):

```
irm https://raw.githubusercontent.com/GitGuardian/ggshield/main/scripts/install/install.ps1 | iex
```

Or, if you prefer curl (bundled with Windows 10+):

```
curl.exe -sSL https://raw.githubusercontent.com/GitGuardian/ggshield/main/scripts/install/install.ps1 | powershell -NoProfile -ExecutionPolicy Bypass -Command -
```

The script accepts options such as --instance and --plugin (install a plugin). For the EU workspace or a self-hosted instance, set the GITGUARDIAN_INSTANCE environment variable (or pass --instance <URL>) before running.

See [scripts/install/README.md](https://github.com/GitGuardian/ggshield/blob/main/scripts/install/README.md) for the full list of options, the other install methods, and how to uninstall.

The methods below install the CLI manually instead.

## macOS

https://github.com/GitGuardian/ggshield#macos

### Homebrew

https://github.com/GitGuardian/ggshield#homebrew

You can install ggshield using Homebrew:

```
brew install ggshield
```

Upgrading is handled by Homebrew.

### Standalone .pkg package

https://github.com/GitGuardian/ggshield#standalone-pkg-package

Alternatively, you can download and install a standalone .pkg package from [ggshield release page](https://github.com/GitGuardian/ggshield/releases).

This package does not require installing Python, but you have to manually download new versions.

## Linux

https://github.com/GitGuardian/ggshield#linux

### Deb and RPM packages

https://github.com/GitGuardian/ggshield#deb-and-rpm-packages

Deb and RPM packages are available on [Cloudsmith](https://cloudsmith.io/~gitguardian/repos/ggshield/packages/).

Setup instructions:

- [Deb packages](https://cloudsmith.io/~gitguardian/repos/ggshield/setup/#formats-deb)
- [RPM packages](https://cloudsmith.io/~gitguardian/repos/ggshield/setup/#formats-rpm)

Upgrading is handled by the package manager.

## Windows

https://github.com/GitGuardian/ggshield#windows

### Chocolatey

https://github.com/GitGuardian/ggshield#chocolatey

ggshield is available via the [Chocolatey package manager](https://chocolatey.org/packages/ggshield):

```
choco install ggshield
```

### MSI installer

https://github.com/GitGuardian/ggshield#msi-installer

Download the MSI installer from the [ggshield release page](https://github.com/GitGuardian/ggshield/releases) and install it:

```
msiexec /i ggshield-VERSION-x86_64-pc-windows-msvc.msi
```

### Standalone .zip archive

https://github.com/GitGuardian/ggshield#standalone-zip-archive

We provide a standalone .zip archive on [ggshield release page](https://github.com/GitGuardian/ggshield/releases).

Unpack the archive on your disk, then add the directory containing the ggshield.exe file to %PATH%.

This archive does not require installing Python, but you have to manually download new versions.

## All operating systems

https://github.com/GitGuardian/ggshield#all-operating-systems

ggshield can be installed on all supported operating systems via its [PyPI package](https://pypi.org/project/ggshield).

It requires a supported version of Python (not EOL) (except for standalone packages) and git.

If you don't use our packaged versions of ggshield, please be aware that we follow the [Python release cycle](https://devguide.python.org/versions/) and do not support versions that have reached EOL.

### Using pipx

https://github.com/GitGuardian/ggshield#using-pipx

The recommended way to install ggshield from PyPI is to use [pipx](https://pypa.github.io/pipx/), which will install it in an isolated environment:

```
pipx install ggshield
```

To upgrade your installation, run:

```
pipx upgrade ggshield
```

### Using pip

https://github.com/GitGuardian/ggshield#using-pip

You can also install ggshield from PyPI using pip, but this is not recommended because the installation is not isolated, so other applications or packages installed this way may affect your ggshield installation. This method will also not work if your Python installation is declared as externally managed (for example when using the system Python on operating systems like Debian 12):

```
pip install --user ggshield
```

To upgrade your installation, run:

```
pip install --user --upgrade ggshield
```

# Initial setup

https://github.com/GitGuardian/ggshield#initial-setup

## Using ggshield auth login

https://github.com/GitGuardian/ggshield#using-ggshield-auth-login

To use ggshield you need to authenticate against GitGuardian servers. To do so, use the ggshield auth login command. This command automates the provisioning of a personal access token and its configuration on the local workstation.

You can learn more about it from [ggshield auth login documentation](https://docs.gitguardian.com/internal-repositories-monitoring/ggshield/reference/auth/login).

After authenticating, run ggshield machine setup to configure AI and git hooks, plant honeytokens, and register the machine on the Endpoints page before its first scan. Registration includes the current user, or all honeytoken target users when running as root. It is best effort, takes at most five seconds, and does not affect the command's exit code.

The scans of ggshield secret scan pre-commit, pre-push and ai-hook also send the machine hostname, from which GitGuardian registers the users of these hooks. Other scan commands do not send it, nor does the native AI hook that standalone and platform-specific pip installs run.

On CI, ggshield neither registers the machine nor sends the hostname. To turn both off elsewhere, set endpoint_registration: false in .gitguardian.yaml, or set GGSHIELD_NO_ENDPOINT_REGISTRATION to a non-empty value.

## Manual setup

https://github.com/GitGuardian/ggshield#manual-setup

You can also create your personal access token manually and store it in the GITGUARDIAN_API_KEY environment variable to complete the setup.

# Getting started

https://github.com/GitGuardian/ggshield#getting-started

## Secrets

https://github.com/GitGuardian/ggshield#secrets

You can now use ggshield to search for secrets:

- in files: ggshield secret scan path -r .
- in repositories: ggshield secret scan repo .
- in Docker images (docker command must be available): ggshield secret scan docker ubuntu:22.04
- in Pypi packages (pip command must be available): ggshield secret scan pypi flask
- and more, have a look at ggshield secret scan --help output for details.

## Migrating a legacy configuration file

https://github.com/GitGuardian/ggshield#migrating-a-legacy-configuration-file

If ggshield reports that your .gitguardian.yaml (or .gitguardian.yml) config file uses a deprecated format, migrate it to the latest version with:

```
ggshield config migrate
```

By default, this looks for the configuration file in the current directory, so run it from the directory containing the file. To run it from anywhere, point ggshield to the file explicitly:

```
ggshield --config-path path/to/.gitguardian.yaml config migrate
```

The previous version of the file is kept as a .old backup next to it.

# Integrations

https://github.com/GitGuardian/ggshield#integrations

You can integrate ggshield in your [CI/CD workflow](https://docs.gitguardian.com/ggshield-docs/integrations/overview#cicd-integrations-secrets-detection-in-your-cicd-workflow).

To catch errors earlier, use ggshield as a [pre-commit, pre-push or pre-receive Git hook](https://docs.gitguardian.com/ggshield-docs/integrations/overview#git-hooks-prevent-secrets-from-reaching-your-vcs).

## AI coding assistants

https://github.com/GitGuardian/ggshield#ai-coding-assistants

ggshield can [scan interactions](https://docs.gitguardian.com/ggshield-docs/integrations/ai-coding-tools/secret-scanning-for-ai-coding-tools) between you and your AI coding assistant in real time, blocking actions that contain secrets before they are executed.

You can install the hooks with the ggshield install command.

Supported tools: Cursor, Claude Code, Copilot Chat, Codex, and Mistral Vibe 2.21+.

# Learn more

https://github.com/GitGuardian/ggshield#learn-more

For more information, have a look at [the documentation](https://docs.gitguardian.com/ggshield-docs/getting-started)

# Output

https://github.com/GitGuardian/ggshield#output

If no secrets have been found, the exit code will be 0:

```
ggshield secret scan pre-commit
```

If a secret is found in your staged code or in your CI, you will have an alert giving you the filename where the secret has been found and a patch giving you the position of the secret in the file:

```
ggshield secret scan pre-commit
```

```
2 incidents have been found in file production.rb

11 | config.paperclip_defaults = {
12 |     :s3_credentials => {
13 |     :bucket => "XXX",
14 |     :access_key_id => "XXXXXXXXXXXXXXXXXXXX",
                            |_____AWS Keys_____|

15 |     :secret_access_key => "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
                                |_______________AWS Keys_______________|

16 |     }
17 | }

```

Lines that are too long are truncated to match the size of the terminal, unless the verbose mode is used (-v or --verbose).

# Related open source projects

https://github.com/GitGuardian/ggshield#related-open-source-projects

- [truffleHog](https://github.com/dxa4481/truffleHog)
- [gitleaks](https://github.com/zricethezav/gitleaks)
- [gitrob](https://github.com/michenriksen/gitrob)
- [git-hound](https://github.com/tillson/git-hound)
- [AWS git-secrets](https://github.com/awslabs/git-secrets)
- [detect-secrets](https://github.com/Yelp/detect-secrets)

# License

https://github.com/GitGuardian/ggshield#license

ggshield is MIT licensed.
