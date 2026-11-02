---
url: "https://usbguard.github.io/"
title: "home | USBGuard"
captured: "2026-11-02T18:47:48Z"
via: "live"
sha256: "f84d6b669d3c20996256d87fcd1b90da5d8274753ed864fd4043235e174e11e4"
wayback: "https://web.archive.org/web/20261001044108/https://usbguard.github.io/"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# home | USBGuard

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://usbguard.github.io/>
> Snapshot: <https://web.archive.org/web/20261001044108/https://usbguard.github.io/>

The USBGuard software framework helps to protect your computer against rogue USB devices (a.k.a. [BadUSB](https://srlabs.de/badusb)) by implementing basic whitelisting and blacklisting capabilities based on device attributes.

## Features

- [Rule language](https://usbguard.github.io/documentation/rule-language.html) for writting USB device authorization policies
- Daemon component with an IPC interface for dynamic interaction and policy enforcement
- Command line and GUI interface to interact with a running USBGuard instance
- C++ API for interacting with the daemon component implemented in a shared library

## Supported Operating Systems

Currently, USBGuard works only on Linux. To enforce the user-defined policy, it uses the USB device authorization feature implemented in the Linux kernel since 2007. Read [this document](https://www.kernel.org/doc/Documentation/usb/authorization.txt) if you want to know more.
