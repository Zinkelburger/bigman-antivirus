---
url: "https://lists.freedesktop.org/archives/flatpak/2024-April/002341.html"
title: "flatpak CVE-2024-32462 : Sandbox escape via RequestBackground portal and CWE-88"
captured: "2026-11-02T18:34:04Z"
via: "live"
sha256: "03d039f70ee7d0f90a5861ae19fef00d3c86631db9f3d2af62bed0e5a4222f83"
wayback: "https://web.archive.org/web/20250821103025/https://lists.freedesktop.org/archives/flatpak/2024-April/002341.html"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# flatpak CVE-2024-32462 : Sandbox escape via RequestBackground portal and CWE-88

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://lists.freedesktop.org/archives/flatpak/2024-April/002341.html>
> Snapshot: <https://web.archive.org/web/20250821103025/https://lists.freedesktop.org/archives/flatpak/2024-April/002341.html>

# flatpak CVE-2024-32462 : Sandbox escape via RequestBackground portal and CWE-88

Simon McVittie smcv at debian.org
Thu Apr 18 16:47:18 UTC 2024

- Next message (by thread): [Announcing Flatpak 1.14.6 (security fix release)](https://lists.freedesktop.org/archives/flatpak/2024-April/002342.html)
-  Messages sorted by: [[ date ]](https://lists.freedesktop.org/archives/flatpak/2024-April/date.html#2341) [[ thread ]](https://lists.freedesktop.org/archives/flatpak/2024-April/thread.html#2341) [[ subject ]](https://lists.freedesktop.org/archives/flatpak/2024-April/subject.html#2341) [[ author ]](https://lists.freedesktop.org/archives/flatpak/2024-April/author.html#2341)

```
Flatpak is a system for building, distributing, and running sandboxed
desktop applications on Linux.

Gergo Koteles discovered a sandbox escape when using Flatpak in
conjunction with xdg-desktop-portal.

For patches and any updated information that becomes available, please see:
https://github.com/flatpak/flatpak/security/advisories/GHSA-phv6-cpc2-2fgj

Impact
======

A malicious or compromised Flatpak app could execute arbitrary code
outside its sandbox.

Fixed versions
==============

* flatpak 1.14.x >= 1.14.6 (stable branch)
* flatpak 1.12.x >= 1.12.9 (old stable branch)
* flatpak 1.10.x >= 1.10.9 (old stable branch)
* flatpak >= 1.15.8 (development branch)

Older flatpak branches such as 1.8.x are EOL and will not receive new
upstream releases.

Vulnerable versions
===================

* flatpak 1.15.x before 1.15.8
* flatpak 1.14.x before 1.14.6
* flatpak 1.12.x before 1.12.9
* all versions before 1.10.9

Description
===========

Normally, the --command argument of flatpak run expects to be given a
command to run in the specified Flatpak app, optionally along with some
arguments. For example:

    flatpak run --command=ls org.gnome.gedit

The original implementation of this was subject to CWE-88: in Flatpak
versions that have this vulnerability, it is possible to pass a long
option name to --command=, such as --bind, and it will be misinterpreted
as a bwrap(1) option. For example, one may do

    flatpak run --command=--bind org.gnome.gedit / /host ls -l /host

Which will run:

    bwrap ...lots of stuff... --bind / /host ls -l /host

It is possible to pass an arbitrary `commandline` to the portal interface
org.freedesktop.portal.Background.RequestBackground from within a Flatpak
app. Normally this is safe, because it can only specify a command that
exists inside the sandbox; but when a crafted `commandline` is converted
into a --command and arguments, the app could achieve the same effect of
passing arguments directly to bwrap, and therefore achieve sandbox escape.

The solution is for Flatpak to use the -- argument to bwrap, which makes
it stop processing options, before appending the attacker-specified
command:

    bwrap ...lots of stuff... -- --bind / /host ls -l /host

which will attempt to run a command named `--bind` from the sandboxed app's
PATH (unlikely to work in practice).

The -- argument has been supported since bubblewrap 0.3.0, and all
supported versions of Flatpak already require at least that version
of bubblewrap.

A mitigation is that xdg-desktop-portal version 1.18.4 will no longer
allow Flatpak apps to create new .desktop files for commands that start
with -.

Test-case
=========

1. Install any Flatpak app, for example org.gnome.Recipes

2. flatpak run --command=--help org.gnome.Recipes
   (replacing the app ID with whatever app you chose)

Good result (not vulnerable):

    bwrap: execvp --help: No such file or directory

Bad result (vulnerable): the same help text as `bwrap --help`.

--
Simon McVittie, Collabora Ltd. / Debian
on behalf of the Flatpak maintainers
-------------- next part --------------
A non-text attachment was scrubbed...
Name: signature.asc
Type: application/pgp-signature
Size: 833 bytes
Desc: not available
URL: <https://lists.freedesktop.org/archives/flatpak/attachments/20240418/3d963827/attachment.sig>

```

- Next message (by thread): [Announcing Flatpak 1.14.6 (security fix release)](https://lists.freedesktop.org/archives/flatpak/2024-April/002342.html)
-  Messages sorted by: [[ date ]](https://lists.freedesktop.org/archives/flatpak/2024-April/date.html#2341) [[ thread ]](https://lists.freedesktop.org/archives/flatpak/2024-April/thread.html#2341) [[ subject ]](https://lists.freedesktop.org/archives/flatpak/2024-April/subject.html#2341) [[ author ]](https://lists.freedesktop.org/archives/flatpak/2024-April/author.html#2341)

[More information about the Flatpak mailing list](https://lists.freedesktop.org/mailman/listinfo/flatpak)
