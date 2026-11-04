---
url: "https://docs.flathub.org/docs/for-users/verification"
title: "Verified apps | Flathub Documentation"
captured: "2026-11-02T18:27:44Z"
via: "live"
sha256: "2bed02a72447e94b7b4619a37d7af9d9a492805bf4de0958bb25cf19e3ca6f71"
wayback: "https://web.archive.org/web/20261024180145/https://docs.flathub.org/docs/for-users/verification"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# Verified apps | Flathub Documentation

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://docs.flathub.org/docs/for-users/verification>
> Snapshot: <https://web.archive.org/web/20261024180145/https://docs.flathub.org/docs/for-users/verification>

# Verified apps

## What is a verified app?[​](https://docs.flathub.org/docs/for-users/verification#what-is-a-verified-app)

A verified app on Flathub is one whose developer has confirmed their ownership of the app ID using a uniquely generated token. This usually also may mean that either the app is maintained directly by the developer or a party authorised or approved by them.

[More information for developers →](https://docs.flathub.org/docs/for-app-authors/verification)

## Why are some apps not verified?[​](https://docs.flathub.org/docs/for-users/verification#why-are-some-apps-not-verified)

Some apps are published on Flathub by the community or third parties and not directly maintained by the original developer. This means such apps are not eligible for verification.

## How do I know if an app is verified?[​](https://docs.flathub.org/docs/for-users/verification#how-do-i-know-if-an-app-is-verified)

Apps will have a blue tick on the Flathub app page indicating they are verified.

## I only want to install verified apps. How do I do that?[​](https://docs.flathub.org/docs/for-users/verification#i-only-want-to-install-verified-apps-how-do-i-do-that)

You can use the --subset=verified option when adding the Flathub remote. This will only add the verified apps to your local repository.

```

flatpak remote-add --if-not-exists --subset=verified --title='Flathub Verified' flathub-verified https://dl.flathub.org/repo/flathub.flatpakrepo

```

If you wish to revert to the full repository, please see the section on [removing subsets](https://docs.flathub.org/docs/for-users/installation#remove-subsets).
