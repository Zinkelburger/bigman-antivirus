---
url: "https://microos.opensuse.org/blog/2023-11-06-utmp-and-wtmp-are-gone"
title: "Y2038: /run/utmp, /var/log/wtmp and /var/log/lastlog are gone"
site: "openSUSE MicroOS"
captured: "2026-11-03T02:57:34Z"
via: "live"
source_url: "https://microos.opensuse.org/blog/2023-11-06-utmp-and-wtmp-are-gone/"
sha256: "f16da014cc4f1f707118769b134df6c41f02652fd3570ab8c1c7e1b0b91f6eb8"
wayback: "https://web.archive.org/web/20260514163029/https://microos.opensuse.org/blog/2023-11-06-utmp-and-wtmp-are-gone/"
cited_in:
  - "docs/research/hardening.md"
---

# Y2038: /run/utmp, /var/log/wtmp and /var/log/lastlog are gone

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://microos.opensuse.org/blog/2023-11-06-utmp-and-wtmp-are-gone>
> Snapshot: <https://web.archive.org/web/20260514163029/https://microos.opensuse.org/blog/2023-11-06-utmp-and-wtmp-are-gone/>

-  [MicroOS](https://microos.opensuse.org/)
-  [2023](https://microos.opensuse.org/2023)
-  [November](https://microos.opensuse.org/2023/11)
-  [06](https://microos.opensuse.org/2023/11/06)
-  Y2038: /run/utmp, /var/log/wtmp and /var/log/lastlog are gone

# Y2038: /run/utmp, /var/log/wtmp and /var/log/lastlog are gone

6. Nov 2023 | Thorsten Kukuk | No License

## Intro

With the last updates openSUSE MicroOS and Tumbleweed don’t use /run/utmp, /var/log/wtmp and /var/log/lastlog anymore. This files will no longer be created. /run/utmp will vanish with the next reboot, /var/log/wtmp and /var/log/lastlog will not be created with a fresh installation. On existing systems, the files will stay, but we will not read it anymore. It can be safely moved away by the administrator.

## Background

As already explained in the last two blogs [Y2038: Replace utmp with logind](https://microos.opensuse.org/blog/2023-09-28-replace-utmp-with-logind/) and [Switch from wtmp to Y2038 safe wtmpdb](https://microos.opensuse.org/blog/2023-06-28-switch-to-wtmpdb/), this files are not Y2038 safe. /var/log/wtmp got replaced with wtmpdb, /run/utmp with systemd-logind and /var/log/lastlog with lastlog2.

## What does this mean for the users?

Hopefully: nothing

All big projects accepted our patches or wrote their own support for this, and the majority of them did already release a new version. This are e.g. coreutils, procps, shadow, util-linux and systemd itself. Using who, w, last, lastlog or similar tools should give you a similar output as before, but there is one big difference: you will no longer see xterm, konsole, screen or similar sessions in the output.

## Presentations

There are two presentations explaining this in detail:

- [Recording of All Systems Go!](https://media.ccc.de/v/all-systems-go-2023-183-y2038-replace-utmp-with-logind)
- [OSS Europe 2023](https://osseu2023.sched.com/event/1OGgG/y2038-and-utmpwtmp-on-64bit-systems-thorsten-kukuk-suse)

Categories: [blog](https://microos.opensuse.org/category/blog)

Tags:

##### Share this post:

https://twitter.com/intent/tweet?counturl=https://microos.opensuse.org/blog/2023-11-06-utmp-and-wtmp-are-gone/&url=https://microos.opensuse.org/blog/2023-11-06-utmp-and-wtmp-are-gone/&via=openSUSE https://www.facebook.com/sharer.php?u=https://microos.opensuse.org/blog/2023-11-06-utmp-and-wtmp-are-gone/ https://telegram.me/share/url?url=https://microos.opensuse.org/blog/2023-11-06-utmp-and-wtmp-are-gone/ http://reddit.com/submit?url=https://microos.opensuse.org/blog/2023-11-06-utmp-and-wtmp-are-gone/&title=Y2038:%20/run/utmp,%20/var/log/wtmp%20and%20/var/log/lastlog%20are%20gone
