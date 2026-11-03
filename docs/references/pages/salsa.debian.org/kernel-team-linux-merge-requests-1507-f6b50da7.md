---
url: "https://salsa.debian.org/kernel-team/linux/-/merge_requests/1507"
title: "Drop \"yama: Disable by default\" and use upstream default to restrict ptrace (!1507) · Merge requests · Debian kernel team / linux · GitLab"
site: "GitLab"
captured: "2026-11-03T02:58:51Z"
via: "wayback"
source_url: "https://web.archive.org/web/20260803220337id_/https://salsa.debian.org/kernel-team/linux/-/merge_requests/1507"
sha256: "10801fbeb5a578b4b1ff00e346cb57fcbe9d77103be6b504637311469738c2fa"
wayback: "https://web.archive.org/web/20260803220337/https://salsa.debian.org/kernel-team/linux/-/merge_requests/1507"
cited_in:
  - "docs/research/hardening.md"
---

# Drop "yama: Disable by default" and use upstream default to restrict ptrace (!1507) · Merge requests · Debian kernel team / linux · GitLab

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://salsa.debian.org/kernel-team/linux/-/merge_requests/1507>
> Snapshot: <https://web.archive.org/web/20260803220337/https://salsa.debian.org/kernel-team/linux/-/merge_requests/1507>

#  Drop "yama: Disable by default" and use upstream default to restrict ptrace

-

Review changes

-
-
-

Download

-  [Patches](https://web.archive.org/kernel-team/linux/-/merge_requests/1507.patch)
-  [Plain diff](https://web.archive.org/kernel-team/linux/-/merge_requests/1507.diff)

[Salvatore Bonaccorso](https://web.archive.org/carnil)requested to merge

[carnil/linux:enable-yama-by-default](https://web.archive.org/carnil/linux/-/tree/enable-yama-by-default)  into [debian/latest](https://web.archive.org/kernel-team/linux/-/tree/debian/latest) May 09, 2025

-  [Overview 4](https://web.archive.org/kernel-team/linux/-/merge_requests/1507)
-  [Commits 1](https://web.archive.org/kernel-team/linux/-/merge_requests/1507/commits)
-  [Pipelines 3](https://web.archive.org/kernel-team/linux/-/merge_requests/1507/pipelines)
-  [Changes 3](https://web.archive.org/kernel-team/linux/-/merge_requests/1507/diffs)

When in 2013 it was requested to enable Yama LSM in #704750, later on in the Debian bug #712740 it was requested to be disabled by default because it prevented some debugging operations.

It was argued, that while having restricted ptrace enabled makes debugging harder, it is a security feature with a different default set upstream.

Switch to what is set upstream as default (kernel.yama.ptrace_scope = 1) and so drop the Debian specific patch.

Closes: #1025071

##  Merge request reports

Loading
