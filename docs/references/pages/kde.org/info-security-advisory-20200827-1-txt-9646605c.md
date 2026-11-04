---
url: "https://kde.org/info/security/advisory-20200827-1.txt"
captured: "2026-11-02T18:33:33Z"
via: "live"
sha256: "1c6280fa2f40de956e6d9dabf85598e4474e07d61907903a0ee221846187bd0d"
wayback: "https://web.archive.org/web/20261023074009/https://kde.org/info/security/advisory-20200827-1.txt"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# https://kde.org/info/security/advisory-20200827-1.txt

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://kde.org/info/security/advisory-20200827-1.txt>
> Snapshot: <https://web.archive.org/web/20261023074009/https://kde.org/info/security/advisory-20200827-1.txt>

KDE Project Security Advisory
=============================

Title:           Ark: maliciously crafted TAR archive with symlinks can install files outside the extraction directory.
Risk Rating:     Important
CVE:             CVE-2020-24654
Versions:        ark <= 20.08.0
Author:          Elvis Angelaccio <elvis.angelaccio@kde.org>
Date:            27 August 2020

Overview
========

A maliciously crafted TAR archive containing symlink entries
would install files anywhere in the user's home directory upon extraction.

Proof of concept
================

For testing, an example of malicious archive can be found at
https://github.com/jwilk/traversal-archives/releases/download/0/dirsymlink.tar

Impact
======

Users can unwillingly install files like a modified .bashrc, or a malicious
script placed in ~/.config/autostart.

Workaround
==========

Before extracting a downloaded archive using the Ark GUI, users should inspect it
to make sure it doesn't contain symlink entries pointing outside the extraction folder.

The 'Extract' context menu from the Dolphin file manager shouldn't be used.

Solution
========

Ark 20.08.1 skips maliciously crafted symlinks when extracting TAR archives.

Alternatively, https://invent.kde.org/utilities/ark/-/commit/8bf8c5ef07b0ac5e914d752681e470dea403a5bd can be applied to previous
releases.


Credits
=======

Thanks to Fabian Vogt for reporting this issue and for fixing it.
