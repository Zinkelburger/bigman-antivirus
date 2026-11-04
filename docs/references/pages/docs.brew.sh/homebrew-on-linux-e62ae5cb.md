---
url: "https://docs.brew.sh/Homebrew-on-Linux"
title: "Homebrew on Linux"
site: "Homebrew Documentation"
captured: "2026-11-02T18:27:38Z"
via: "live"
sha256: "b8816aebf5e2117342f9d07b81d509ba3fba50e2d94d1763dbd33bfee7682b02"
wayback: "https://web.archive.org/web/20261003062223/https://docs.brew.sh/Homebrew-on-Linux"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# Homebrew on Linux

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://docs.brew.sh/Homebrew-on-Linux>
> Snapshot: <https://web.archive.org/web/20261003062223/https://docs.brew.sh/Homebrew-on-Linux>

# Homebrew on Linux

The Homebrew package manager may be used on Linux and [Windows Subsystem for Linux (WSL)](https://docs.microsoft.com/en-us/windows/wsl/about) 2. Homebrew was formerly referred to as Linuxbrew when running on Linux or WSL. Homebrew does not use any libraries provided by your host system, except glibc and gcc if they are new enough. Homebrew can install its own current versions of glibc and gcc for older distributions of Linux.

[Features](https://docs.brew.sh/Homebrew-on-Linux#features), [installation instructions](https://docs.brew.sh/Homebrew-on-Linux#install) and [requirements](https://docs.brew.sh/Homebrew-on-Linux#requirements) are described below. Terminology (e.g. the difference between the Cellar, a tap and a cask) is [explained in the documentation](https://docs.brew.sh/Formula-Cookbook#homebrew-terminology).

## Features

- Install software not packaged by your host distribution
- Install up-to-date versions of software when your host distribution is old
- Use the same package manager to manage your macOS, Linux and Windows systems

## Install

Instructions for the best, supported install of Homebrew on Linux are on the [homepage](https://brew.sh/).

The installation script installs Homebrew to /home/linuxbrew/.linuxbrew, which allows the use of most binary packages (bottles). The managing account needs write access to the installation; see [running without sudo](https://docs.brew.sh/Installation#running-without-sudo) and [using a dedicated owner](https://docs.brew.sh/Installation#running-as-the-homebrew-owner).

Follow the installer’s Next steps instructions to add Homebrew to your PATH and shell configuration. For a supported installation using Bash, run:

```
eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"
echo 'eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"' >> ~/.bashrc

```

Use ~/.zshrc instead of ~/.bashrc when zsh is your login shell.

You’re done! Try installing a package:

```
brew install hello

```

If you’re using an older distribution of Linux, installing your first package will also install a recent version of glibc and gcc. Use brew doctor to troubleshoot common issues.

## Requirements

See [Support Tiers](https://docs.brew.sh/Support-Tiers#linux) for the full list of Linux requirements.

Homebrew expects a working system C compiler and the standard Linux development tools. Homebrew can install its own current version of gcc when needed, but this does not replace the system compiler required for bootstrap and post-install steps.

To install build tools, paste at a terminal prompt:

-

Debian or Ubuntu

```
sudo apt-get install build-essential procps curl file git

```

-

Fedora

```
sudo dnf group install development-tools
sudo dnf install procps-ng curl file

```

-

CentOS Stream or RHEL

```
sudo dnf group install 'Development Tools'
sudo dnf install procps-ng curl file

```

-

Arch Linux

```
sudo pacman -S base-devel procps-ng curl file git

```

### ARM32 (Tier 3 Support)

Homebrew can run on 32-bit ARM systems such as Raspberry Pi devices, but because they lack bottles they are a [Tier 3 platform](https://docs.brew.sh/Support-Tiers#tier-3).

You may need to install your own Ruby using your system package manager, a PPA or rbenv/ruby-build because Homebrew does not distribute Portable Ruby for ARM32.

### 32-bit x86 (unsupported)

Homebrew does not run at all on 32-bit x86 platforms.

### Windows Subsystem for Linux 1 (Tier 3 Support)

Due to [known issues](https://github.com/microsoft/WSL/issues/8219) with WSL 1, you may experience issues running various executables installed by Homebrew. We recommend you switch to WSL 2 instead.

## Homebrew on Linux community

- [Homebrew/discussions (forum)](https://github.com/orgs/Homebrew/discussions/categories/linux)
