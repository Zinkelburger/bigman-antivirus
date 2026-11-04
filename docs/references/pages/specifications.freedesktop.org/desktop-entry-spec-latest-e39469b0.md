---
url: "https://specifications.freedesktop.org/desktop-entry-spec/latest/"
title: "Desktop Entry Specification"
captured: "2026-11-02T18:44:19Z"
via: "live"
source_url: "https://specifications.freedesktop.org/desktop-entry/latest/"
sha256: "1b0a0d7c1698092b16a5df4068e3a6a5b3baf6474e927a1205395cc63549ff4f"
wayback: "https://web.archive.org/web/20251021165800/https://specifications.freedesktop.org/desktop-entry-spec/latest/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Desktop Entry Specification

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://specifications.freedesktop.org/desktop-entry-spec/latest/>
> Snapshot: <https://web.archive.org/web/20251021165800/https://specifications.freedesktop.org/desktop-entry-spec/latest/>

[Jump to content](https://specifications.freedesktop.org/desktop-entry/latest/#_content)[Jump to page navigation: previous page [access key p]/next page [access key n]](https://specifications.freedesktop.org/desktop-entry/latest/#_bottom-navigation)

https://specifications.freedesktop.org/

[Desktop Entry Specification](https://specifications.freedesktop.org/desktop-entry/latest/index.html)

[Show Contents: Desktop Entry Specification](https://specifications.freedesktop.org/desktop-entry/latest/index.html)

[Top](https://specifications.freedesktop.org/desktop-entry/latest/#)

- [1 Introduction](https://specifications.freedesktop.org/desktop-entry/latest/index.html#introduction)
- [2 File naming](https://specifications.freedesktop.org/desktop-entry/latest/file-naming.html)
- [3 Basic format of the file](https://specifications.freedesktop.org/desktop-entry/latest/basic-format.html)
- [4 Possible value types](https://specifications.freedesktop.org/desktop-entry/latest/value-types.html)
- [5 Localized values for keys](https://specifications.freedesktop.org/desktop-entry/latest/localized-keys.html)
- [6 Recognized desktop entry keys](https://specifications.freedesktop.org/desktop-entry/latest/recognized-keys.html)
- [7 The Exec key](https://specifications.freedesktop.org/desktop-entry/latest/exec-variables.html)
- [8 D-Bus Activation](https://specifications.freedesktop.org/desktop-entry/latest/dbus.html)
- [9 Interfaces](https://specifications.freedesktop.org/desktop-entry/latest/interfaces.html)
- [10 Registering MIME Types](https://specifications.freedesktop.org/desktop-entry/latest/mime-types.html)
- [11 Additional applications actions](https://specifications.freedesktop.org/desktop-entry/latest/extra-actions.html)
- [12 Extending the format](https://specifications.freedesktop.org/desktop-entry/latest/extending.html)
- [A Example Desktop Entry File](https://specifications.freedesktop.org/desktop-entry/latest/example.html)
- [B Historically Reserved Items](https://specifications.freedesktop.org/desktop-entry/latest/historical-items.html)
- [C Deprecated Items](https://specifications.freedesktop.org/desktop-entry/latest/deprecated-items.html)
- [D The Legacy-Mixed Encoding (Deprecated)](https://specifications.freedesktop.org/desktop-entry/latest/legacy-mixed.html)
- [E Changes to this Specification](https://specifications.freedesktop.org/desktop-entry/latest/ape.html)

##  Desktop Entry Specification [#](https://specifications.freedesktop.org/desktop-entry/latest/index.html)

Authors:

Preston Brown <pbrown@kde.org>

Jonathan Blandford <jrb@redhat.com>

Owen Taylor <otaylor@gtk.org>

Vincent Untz <vuntz@gnome.org>

Waldo Bastian <waldo.bastian@intel.com>

Allison Lortie <desrt@desrt.ca>

David Faure <faure@kde.org>

Will Thompson <wjt@endlessos.org>

Publication Date: 2020-04-27, Version: Version 1.5

[1 Introduction](https://specifications.freedesktop.org/desktop-entry/latest/index.html#introduction)

[2 File naming](https://specifications.freedesktop.org/desktop-entry/latest/file-naming.html)

[3 Basic format of the file](https://specifications.freedesktop.org/desktop-entry/latest/basic-format.html)

[4 Possible value types](https://specifications.freedesktop.org/desktop-entry/latest/value-types.html)

[5 Localized values for keys](https://specifications.freedesktop.org/desktop-entry/latest/localized-keys.html)

[6 Recognized desktop entry keys](https://specifications.freedesktop.org/desktop-entry/latest/recognized-keys.html)

[7 The Exec key](https://specifications.freedesktop.org/desktop-entry/latest/exec-variables.html)

[8 D-Bus Activation](https://specifications.freedesktop.org/desktop-entry/latest/dbus.html)

[9 Interfaces](https://specifications.freedesktop.org/desktop-entry/latest/interfaces.html)

[10 Registering MIME Types](https://specifications.freedesktop.org/desktop-entry/latest/mime-types.html)

[11 Additional applications actions](https://specifications.freedesktop.org/desktop-entry/latest/extra-actions.html)

[12 Extending the format](https://specifications.freedesktop.org/desktop-entry/latest/extending.html)

[A Example Desktop Entry File](https://specifications.freedesktop.org/desktop-entry/latest/example.html)

[B Historically Reserved Items](https://specifications.freedesktop.org/desktop-entry/latest/historical-items.html)

[C Deprecated Items](https://specifications.freedesktop.org/desktop-entry/latest/deprecated-items.html)

[D The Legacy-Mixed Encoding (Deprecated)](https://specifications.freedesktop.org/desktop-entry/latest/legacy-mixed.html)

[E Changes to this Specification](https://specifications.freedesktop.org/desktop-entry/latest/ape.html)

[E.1 Changes from version 1.4 to 1.5](https://specifications.freedesktop.org/desktop-entry/latest/ape.html#id-1.18.2)

[E.2 Changes from version 1.3 to 1.4](https://specifications.freedesktop.org/desktop-entry/latest/apes02.html)

[E.3 Changes from version 1.2 to 1.3](https://specifications.freedesktop.org/desktop-entry/latest/apes03.html)

[E.4 Changes from version 1.1 to 1.2](https://specifications.freedesktop.org/desktop-entry/latest/apes04.html)

[E.5 Changes from version 1.0 to 1.1](https://specifications.freedesktop.org/desktop-entry/latest/apes05.html)

## 1 Introduction [#](https://specifications.freedesktop.org/desktop-entry/latest/index.html#introduction)

Both the KDE and GNOME desktop environments have adopted a similar format for "desktop entries", or configuration files describing how a particular program is to be launched, how it appears in menus, etc. It is to the larger community's benefit that a unified standard be agreed upon by all parties such that interoperation between the two environments, and indeed any additional environments that implement the specification, becomes simpler.

[→File naming](https://specifications.freedesktop.org/desktop-entry/latest/file-naming.html)

© 2026 Freedesktop.org
