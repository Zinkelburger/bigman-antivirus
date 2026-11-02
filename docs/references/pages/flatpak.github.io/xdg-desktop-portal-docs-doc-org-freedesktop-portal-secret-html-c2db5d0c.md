---
url: "https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Secret.html"
title: "Secret"
site: "XDG Desktop Portal"
captured: "2026-11-02T18:31:20Z"
via: "live"
sha256: "3c338f86fa42ea1672178735f18a5c62aeca068d05660e063d50965934b14e28"
wayback: "https://web.archive.org/web/20260831193020/https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Secret.html"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Secret

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Secret.html>
> Snapshot: <https://web.archive.org/web/20260831193020/https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Secret.html>

# Secret[🔗](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Secret.html#secret)

org.freedesktop.portal.Secret

Portal for retrieving application secret

The Secret portal allows sandboxed applications to retrieve a per-application secret. The secret can then be used for encrypting confidential data inside the sandbox.

This documentation describes version 1 of this interface.

## Properties[🔗](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Secret.html#properties)

### version[🔗](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Secret.html#version)

```
version readable u

```

## Methods[🔗](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Secret.html#methods)

### RetrieveSecret()[🔗](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Secret.html#retrievesecret)

```
RetrieveSecret (
  IN fd h,
  IN options a{sv},
  OUT handle o
)

```

Retrieves a master secret for a sandboxed application.

The master secret is unique per application and does not change as long as the application is installed (once it has been created). In a typical backend implementation, it is stored in the user’s keyring, under the application ID as a key.

While the master secret can be used for encrypting any confidential data in the sandbox, the format is opaque to the application. In particular, the length of the secret might not be sufficient for the use with certain encryption algorithm. In that case, the application is supposed to expand it using a KDF algorithm.

The portal may return an additional identifier associated with the secret in the results vardict of [Request::Response](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Request.html#org-freedesktop-portal-request-response) signal. In the next call of this method, the application shall indicate it through a token element in options.

Supported keys in the options vardict include:

-

handle_token (s)

A string that will be used as the last element of the handle. Must be a valid object path element. See the [Request](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Request.html#org-freedesktop-portal-request) documentation for more information about the handle.

-

token (s)

An opaque string returned by a previous [RetrieveSecret()](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Secret.html#org-freedesktop-portal-secret-retrievesecret) call.

fd

Writable file descriptor for transporting the secret

options

Vardict with optional further information

handle

Object path for the [Request](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Request.html#org-freedesktop-portal-request) object representing this call
