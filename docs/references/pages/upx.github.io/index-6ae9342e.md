---
url: "https://upx.github.io/"
title: "UPX: the Ultimate Packer for eXecutables - Homepage"
captured: "2026-11-02T18:47:52Z"
via: "live"
sha256: "f07678bd16df1e713cac0c04bd2eb0b3d2accfb98c589f6f63911d6f32ad1fa7"
wayback: "https://web.archive.org/web/20261102101642/https://upx.github.io/"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# UPX: the Ultimate Packer for eXecutables - Homepage

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://upx.github.io/>
> Snapshot: <https://web.archive.org/web/20261102101642/https://upx.github.io/>

# [UPX](https://upx.github.io)

## the Ultimate Packer for eXecutables

[View source on GitHub](https://github.com/upx/upx) [Download latest release](https://github.com/upx/upx/releases/latest)

# Welcome

UPX is a free, secure, portable, extendable, high-performance executable packer for several executable formats.

# Introduction

UPX is an advanced executable file compressor. UPX will typically reduce the file size of programs and DLLs by around 50%-70%, thus reducing disk space, network load times, download times and other distribution and storage costs.

Programs and libraries compressed by UPX are completely self-contained and run exactly as before, with no runtime or memory penalty for most of the supported formats.

UPX supports a number of different executable formats, including Windows programs and DLLs, macOS apps and Linux executables.

UPX is free software distributed under the term of the GNU General Public License. Full source code is available.

UPX may be distributed and used freely, even with commercial applications.

# Overview

UPX strengths in a nutshell:

-  secure: as UPX is documented Open Source since many years any relevant Security/Antivirus software is able to peek inside UPX compressed apps to verify them
-  excellent compression ratio: typically compresses better than Zip, use UPX to decrease the size of your distribution!
-  very fast decompression: more than 500 MB/sec on any reasonably modern machine
-  no memory overhead for your compressed executables because of in-place decompression
-  safe: you can list, test and unpack your executables. Also, a checksum of both the compressed and uncompressed file is maintained internally.
-  universal: UPX can pack a number of executable formats, including Windows programs and DLLs, macOS apps and Linux executables
-  portable: UPX is written in portable endian-neutral C++
-  extendable: because of the class layout it's very easy to add new executable formats or new compression algorithms
-  free: UPX is distributed with full source code under the terms of the [GNU General Public License v2+](https://upx.github.io/upx-COPYING.txt); either under the pure GPLv2+, or (at your option) under the GPLv+2 with special exceptions and restrictions granting the free usage for all binaries including commercial programs as stated in the [UPX License Agreement](https://upx.github.io/upx-license.html)

You probably understand now why we call UPX the "Ultimate Packer for eXecutables"

# Copyright

UPX is Copyright © 2024 by [Markus F.X.J. Oberhumer](https://www.oberhumer.com/), László Molnár & [John F. Reiser](https://www.bitwagon.com/).

The term UPX is a shorthand for the Ultimate Packer for eXecutables. All trademarks, brands, and names are the property of their respective owners.
