---
url: "https://specifications.freedesktop.org/notification-spec/latest/"
title: "Desktop Notifications Specification"
captured: "2026-11-02T18:45:17Z"
via: "live"
source_url: "https://specifications.freedesktop.org/notification/latest/"
sha256: "e0be6e3e4fee5dfeedb7d79a203e295c14b34a9d975cf10fca960952028d034d"
wayback: "https://web.archive.org/web/20251021171841/https://specifications.freedesktop.org/notification-spec/latest/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Desktop Notifications Specification

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://specifications.freedesktop.org/notification-spec/latest/>
> Snapshot: <https://web.archive.org/web/20251021171841/https://specifications.freedesktop.org/notification-spec/latest/>

[Jump to content](https://specifications.freedesktop.org/notification/latest/#_content)[Jump to page navigation: previous page [access key p]/next page [access key n]](https://specifications.freedesktop.org/notification/latest/#_bottom-navigation)

https://specifications.freedesktop.org/

[Desktop Notifications Specification](https://specifications.freedesktop.org/notification/latest/index.html)

[Show Contents: Desktop Notifications Specification](https://specifications.freedesktop.org/notification/latest/index.html)

[Top](https://specifications.freedesktop.org/notification/latest/#)

- [1 Introduction](https://specifications.freedesktop.org/notification/latest/index.html#introduction)
- [2 Basic Design](https://specifications.freedesktop.org/notification/latest/basic-design.html)
- [3 Backwards Compatibility](https://specifications.freedesktop.org/notification/latest/backwards-compat.html)
- [4 Markup](https://specifications.freedesktop.org/notification/latest/markup.html)
- [5 Icons and Images](https://specifications.freedesktop.org/notification/latest/icons-and-images.html)
- [6 Categories](https://specifications.freedesktop.org/notification/latest/categories.html)
- [7 Urgency Levels](https://specifications.freedesktop.org/notification/latest/urgency-levels.html)
- [8 Hints](https://specifications.freedesktop.org/notification/latest/hints.html)
- [9 D-Bus Protocol](https://specifications.freedesktop.org/notification/latest/protocol.html)

##  Desktop Notifications Specification [#](https://specifications.freedesktop.org/notification/latest/index.html)

Authors:

Mike Hearn <mike@navi.cx>

Christian Hammond <chipx86@chipx86.com>

William Jon McCann <jmccann@redhat.com>

Publication Date: 18 August 2024, Version: Version 1.3

[1 Introduction](https://specifications.freedesktop.org/notification/latest/index.html#introduction)

[2 Basic Design](https://specifications.freedesktop.org/notification/latest/basic-design.html)

[3 Backwards Compatibility](https://specifications.freedesktop.org/notification/latest/backwards-compat.html)

[4 Markup](https://specifications.freedesktop.org/notification/latest/markup.html)

[5 Icons and Images](https://specifications.freedesktop.org/notification/latest/icons-and-images.html)

[6 Categories](https://specifications.freedesktop.org/notification/latest/categories.html)

[7 Urgency Levels](https://specifications.freedesktop.org/notification/latest/urgency-levels.html)

[8 Hints](https://specifications.freedesktop.org/notification/latest/hints.html)

[9 D-Bus Protocol](https://specifications.freedesktop.org/notification/latest/protocol.html)

## 1 Introduction [#](https://specifications.freedesktop.org/notification/latest/index.html#introduction)

This is a standard for a desktop notifications service, through which applications can generate passive popups to notify the user in an asynchronous manner of events.

This specification explicitly does not include other types of notification presentation such as modal message boxes, window manager decorations or window list annotations.

Example use cases include:

-

Messages from chat programs

-

Scheduled alarm

-

Completed file transfer

-

New mail notification

-

Low disk space/battery warnings

[→Basic Design](https://specifications.freedesktop.org/notification/latest/basic-design.html)

© 2026 Freedesktop.org
