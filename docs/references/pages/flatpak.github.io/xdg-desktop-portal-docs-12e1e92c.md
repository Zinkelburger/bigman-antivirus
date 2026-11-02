---
url: "https://flatpak.github.io/xdg-desktop-portal/docs/"
title: "XDG Desktop Portal"
site: "XDG Desktop Portal"
captured: "2026-11-02T18:31:17Z"
via: "live"
sha256: "1798672826f4785214eb9daf3448cbd4de4c845f82ac9199e17d85706412f097"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# XDG Desktop Portal

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://flatpak.github.io/xdg-desktop-portal/docs/>

# XDG Desktop Portal[🔗](https://flatpak.github.io/xdg-desktop-portal/docs/#xdg-desktop-portal)

Portals allow [Flatpak](https://flatpak.org) apps, and other desktop containment frameworks, to interact with the system in a secure and well defined way.

XDG Desktop Portal works by exposing a series of D-Bus interfaces known as portals to apps. Portals are designed to be usable by all apps, including ones which are confined by sandboxes, such as Flatpak.

The portal interfaces include APIs for file access, opening URIs, printing and others.

XDG Desktop Portal works together with desktop environment specific backends to mediate access to resources and functionality in an integrated manner.

## Documentation[🔗](https://flatpak.github.io/xdg-desktop-portal/docs/#documentation)

This documentation covers everything you need to know to build apps that use portals, write portal backends for your desktop environment, configure and distribute portals as part of a distribution, as well as basic concepts and common conventions:

-

[Common conventions](https://flatpak.github.io/xdg-desktop-portal/docs/common-conventions.html): coding patterns and principles common when app and desktop developers are working with portal APIs.

-

[App Development](https://flatpak.github.io/xdg-desktop-portal/docs/for-app-developers.html): portal APIs that apps can use to interact with the host system.

-

[Desktop Integration](https://flatpak.github.io/xdg-desktop-portal/docs/for-desktop-developers.html): interfaces and configuration files that desktop developers can implement and install in order to write a portal backend.

-

[Contributing](https://flatpak.github.io/xdg-desktop-portal/docs/for-contributors.html): how contributors can contribute to the project.

## Using Portals[🔗](https://flatpak.github.io/xdg-desktop-portal/docs/#using-portals)

Many toolkits and frameworks already integrate with some portals, and you might already be using them without realizing it. For all other cases, there are [Convenience Libraries](https://flatpak.github.io/xdg-desktop-portal/docs/convenience-libraries.html) and direct [D-Bus access](https://flatpak.github.io/xdg-desktop-portal/docs/api-reference.html).

## Backends[🔗](https://flatpak.github.io/xdg-desktop-portal/docs/#backends)

To implement most portals, xdg-desktop-portal relies on a desktop environment specific backend to provide a part of the functionality.

Here are some examples of available backends:

-

GTK [xdg-desktop-portal-gtk](http://github.com/flatpak/xdg-desktop-portal-gtk)

-

GNOME [xdg-desktop-portal-gnome](https://gitlab.gnome.org/GNOME/xdg-desktop-portal-gnome/)

-

KDE [xdg-desktop-portal-kde](https://invent.kde.org/plasma/xdg-desktop-portal-kde)

-

LXQt [xdg-desktop-portal-lxqt](https://github.com/lxqt/xdg-desktop-portal-lxqt)

-

Pantheon (elementary OS) [xdg-desktop-portal-pantheon](https://github.com/elementary/portals)

-

wlroots [xdg-desktop-portal-wlr](https://github.com/emersion/xdg-desktop-portal-wlr)

-

Deepin [xdg-desktop-portal-dde](https://github.com/linuxdeepin/xdg-desktop-portal-dde)

-

Xapp (Cinnamon, MATE, Xfce) [xdg-desktop-portal-xapp](https://github.com/linuxmint/xdg-desktop-portal-xapp)

-

COSMIC [xdg-desktop-portal-cosmic](https://github.com/pop-os/xdg-desktop-portal-cosmic)

## Contributing[🔗](https://flatpak.github.io/xdg-desktop-portal/docs/#contributing)

XDG Desktop Portal is [Free Software](https://www.gnu.org/philosophy/free-sw.html). Contributions [welcome](https://flatpak.github.io/xdg-desktop-portal/docs/for-contributors.html).

Are you an app developer and want a new portal or feature? Contribute to or open a new [discussion](https://github.com/flatpak/xdg-desktop-portal/discussions).

Is there an issue with the portal frontend? Report it to our [issue tracker](https://github.com/flatpak/xdg-desktop-portal/issues).

Just want to talk or need help? Join our [matrix](https://matrix.to/#/#xdg-desktop-portals:matrix.org).
