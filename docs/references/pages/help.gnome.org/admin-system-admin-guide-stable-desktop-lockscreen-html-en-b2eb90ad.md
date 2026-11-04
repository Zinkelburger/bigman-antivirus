---
url: "https://help.gnome.org/admin/system-admin-guide/stable/desktop-lockscreen.html.en"
title: "Lock the screen when the user is idle"
captured: "2026-11-02T18:33:27Z"
via: "live"
source_url: "https://help.gnome.org/system-admin-guide/desktop-lockscreen.html"
sha256: "c1d3ee6fdc3f5e55b99e5b4ef4d2c9fe0b93df4a4e3a18973e34a07b8b2fd0b3"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Lock the screen when the user is idle

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://help.gnome.org/admin/system-admin-guide/stable/desktop-lockscreen.html.en>

# Lock the screen when the user is idle

You can make the screen lock automatically whenever the user is idle for some amount of time. This is useful if users may leave their computers unattended in public or unsecure locations.

## Enable automatic screen lock

-

Create the user profile which contains the following lines:

## /etc/dconf/profile/user

```
user-db:user
system-db:local
```

local is the name of a [dconf](https://help.gnome.org/system-admin-guide/dconf.html) database.

-

Create the directory /etc/dconf/db/local.d/ if it does not already exist.

-

Create the key file /etc/dconf/db/local.d/00-screensaver to provide information for the local database.

## /etc/dconf/db/local.d/00-screensaver

```
# Specify the dconf path
[org/gnome/desktop/session]

# Number of seconds of inactivity before the screen goes blank
# Set to 0 seconds if you want to deactivate the screensaver.
idle-delay=uint32 180

# Specify the dconf path
[org/gnome/desktop/screensaver]

# Number of seconds after the screen is blank before locking the screen
lock-delay=uint32 0
```

You must include the uint32 along with the integer key values as shown.

-

To prevent the user from overriding these settings, create the file /etc/dconf/db/local.d/locks/screensaver with the following content:

## /etc/dconf/db/local.d/locks/screensaver

```
# Lock desktop screensaver settings
/org/gnome/desktop/session/idle-delay
/org/gnome/desktop/screensaver/lock-delay
```

-

Update the system databases:

```
# dconf update
```

-

Users must log out and back in again before the system-wide settings take effect.

## More Information

-  [Appearance](https://help.gnome.org/system-admin-guide/appearance.html) — [Change the lock screen shield](https://help.gnome.org/system-admin-guide/desktop-shield.html), [default favorite applications](https://help.gnome.org/system-admin-guide/desktop-favorite-applications.html), [lock the screen when idle](https://help.gnome.org/system-admin-guide/desktop-lockscreen.html)…

## See Also

-  [Lock down specific settings](https://help.gnome.org/system-admin-guide/dconf-lockdown.html) — Use the lockdown mode in dconf to prevent users from changing specific settings.
-  [Profiles](https://help.gnome.org/system-admin-guide/dconf-profiles.html) — Detailed information about profile and profile selection.
