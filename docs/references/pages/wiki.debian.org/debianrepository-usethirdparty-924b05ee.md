---
url: "https://wiki.debian.org/DebianRepository/UseThirdParty"
title: "DebianRepository/UseThirdParty - Debian Wiki"
captured: "2026-11-02T18:48:51Z"
via: "wayback"
source_url: "https://web.archive.org/web/20260822105217id_/https://wiki.debian.org/DebianRepository/UseThirdParty"
sha256: "4b7a934d4612f8fb4429d4677f19d68540dc0c0a9107dc9622c34ca659658bb9"
wayback: "https://web.archive.org/web/20260822105217/https://wiki.debian.org/DebianRepository/UseThirdParty"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# DebianRepository/UseThirdParty - Debian Wiki

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://wiki.debian.org/DebianRepository/UseThirdParty>
> Snapshot: <https://web.archive.org/web/20260822105217/https://wiki.debian.org/DebianRepository/UseThirdParty>

https://www.debian.org

[Wiki](https://web.archive.org/FrontPage)

[Login](https://web.archive.org/DebianRepository/UseThirdParty?action=login)

- [FrontPage](https://web.archive.org/FrontPage)
- [RecentChanges](https://web.archive.org/RecentChanges)
- [FindPage](https://web.archive.org/FindPage)
- [HelpContents](https://web.archive.org/HelpContents)
- [UseThirdParty](https://web.archive.org/DebianRepository/UseThirdParty)

https://www.debian.org

[Wiki](https://web.archive.org/FrontPage)/

- [Login](https://web.archive.org/DebianRepository/UseThirdParty?action=login)
- [Comments](https://web.archive.org/web/20260822105217id_/https://wiki.debian.org/DebianRepository/UseThirdParty#)
- [Info](https://web.archive.org/DebianRepository/UseThirdParty?action=info)
- [Attachments](https://web.archive.org/DebianRepository/UseThirdParty?action=AttachFile)
-

#

- [DebianRepository](https://web.archive.org/DebianRepository)
- [UseThirdParty](https://web.archive.org/DebianRepository/UseThirdParty)

# Instructions for using a third-party repository

There are many different ways of configuring an unofficial APT repository. This document explains how to add third-party repositories to a Debian system so that the new repository is only trusted to provide a set of expected packages, and so that those packages will be securely delivered to the system.

Where possible, this document uses RFC-like vocabulary as defined by RFC RFC:2119. These instructions assume Debian 9 "stretch" or later.

If you do not trust the maintainers of a 3rd party repository, you should not enable it.

The procedures documented here aim to prevent a repository being trusted to provide arbitrary packages. For example, a repository that ships a video game emulator and its mods should not be able to override the standard [libc6](https://packages.debian.org/libc6) package.

However, a malicious repository can easily circumvent these protections. For example any package can run a [MaintainerScripts](https://web.archive.org/MaintainerScripts) command as root, and this may override the configured preferences or authorise new OpenPGP certificates. For the purposes of this page, attacks by a package that belongs to a given repository are out of scope. To restrict what an installed package can do, see the larger [UntrustedDebs](https://web.archive.org/UntrustedDebs) problem, and particularly [Teams/Dpkg/Spec/DeclarativePackaging](https://web.archive.org/Teams/Dpkg/Spec/DeclarativePackaging) for a potential solution.

Contents

-  [Instructions for using a third-party repository](https://web.archive.org/web/20260822105217id_/https://wiki.debian.org/DebianRepository/UseThirdParty#Instructions_for_using_a_third-party_repository)

-  [OpenPGP certificate distribution](https://web.archive.org/web/20260822105217id_/https://wiki.debian.org/DebianRepository/UseThirdParty#OpenPGP_certificate_distribution)
-  [Sources.list entry](https://web.archive.org/web/20260822105217id_/https://wiki.debian.org/DebianRepository/UseThirdParty#Sources.list_entry)
-  [Standard pinning](https://web.archive.org/web/20260822105217id_/https://wiki.debian.org/DebianRepository/UseThirdParty#Standard_pinning)
-  [Certificate rollover and updates](https://web.archive.org/web/20260822105217id_/https://wiki.debian.org/DebianRepository/UseThirdParty#Certificate_rollover_and_updates)
-  [Complete example](https://web.archive.org/web/20260822105217id_/https://wiki.debian.org/DebianRepository/UseThirdParty#Complete_example)
-  [Troubleshooting](https://web.archive.org/web/20260822105217id_/https://wiki.debian.org/DebianRepository/UseThirdParty#Troubleshooting)
-  [OpenPGP certificate handling](https://web.archive.org/web/20260822105217id_/https://wiki.debian.org/DebianRepository/UseThirdParty#OpenPGP_certificate_handling)
-  [Credits](https://web.archive.org/web/20260822105217id_/https://wiki.debian.org/DebianRepository/UseThirdParty#Credits)
-  [References](https://web.archive.org/web/20260822105217id_/https://wiki.debian.org/DebianRepository/UseThirdParty#References)

## OpenPGP certificate distribution

Repositories MUST be signed with an OpenPGP certificate. A binary export (gpg --export) of the certificate SHOULD be available at the root of the repository under the filename deriv-archive-keyring.pgp, where deriv is the a short name for the repository. The file SHOULD NOT be ASCII-Armored (gpg --export --armor) although a separate armored version MAY be available under deriv-archive-keyring.asc.

The certificate SHOULD be served over HTTPS if possible. A free X509 certificate MAY be obtained from [Let's Encrypt](https://letsencrypt.org/) and automatically configured using the [certbot](https://packages.debian.org/certbot) package.

The certificate MAY also be made available on key servers. If so, operators SHOULD choose an appropriate keyserver or keyserver pool, such as [keys.openpgp.org](https://keys.openpgp.org/) or [keyserver.ubuntu.com](https://keyserver.ubuntu.com/), or implement a [OpenPGP Web Key Directory](https://datatracker.ietf.org/doc/draft-koch-openpgp-webkey-service/). It should be noted that the previously recommended ad-hoc standard pool, [sks-keyservers.net](https://sks-keyservers.net/) is out of service permanently. This certificate SHOULD be signed by other keys, preferably including some that are close to the strong set, in order to leverage the OpenPGP web of trust. The certificate MUST be downloaded over a secure mechanism like HTTPS to a location only writable by root. The certificate MUST NOT be placed in /etc/apt/trusted.gpg.d or loaded by apt-key add.

If future updates to the certificate will be managed by an apt/dpkg package as recommended below, then it SHOULD be downloaded into /usr/share/keyrings using the same filename that will be provided by the package. If it will be managed locally , it SHOULD be downloaded into /etc/apt/keyrings instead.

In releases older than Debian 12 and Ubuntu 22.04, /etc/apt/keyrings does not exist by default. It SHOULD be created with permissions 0755 if it is needed and does not already exist.

For example, users MAY be told to run a command to download the certificate, but because chances are the certificate being distributed is ASCII-Armored, it is best to unconditionally dearmor them. With Sequoia-PGP (with its stable >= 1.0 CLI API):

```
curl https://deriv.example.net/debian/deriv-archive-keyring.pgp | sq  packet dearmor --output /usr/share/keyrings/deriv-archive-keyring.pgp
```

or with GnuPG:

```
curl https://deriv.example.net/debian/deriv-archive-keyring.pgp | gpg -o /usr/share/keyrings/deriv-archive-keyring.pgp --dearmor
```

The reason why we avoid ASCII-Armored files is that they can only be used by [SecureApt](https://web.archive.org/SecureApt) in version 1.4 or later (which appeared in stretch). We also strongly recommend the use of HTTPS as it bypasses certain MITM attacks that would allow a hostile third party to inject OpenPGP certificate material in the repository setup.

## Sources.list entry

A sources.list entry SHOULD have the signed-by option set. The signed-by entry MUST point to a file, and not a fingerprint.

The suite entry SHOULD correspond to the target Debian release if the binaries are built for a specific suite. In other cases, the suite SHOULD be the string "stable", or it MAY be a repository-specific string describing the suite concisely. If the suite does not correspond to a target Debian release, the suite naming convention MUST be clearly documented.

If the repository has no reason to be split into multiple components, then the component name SHOULD be main. If there is a reason for splitting the repo into multiple components, the reason for the split should be clearly documented (e.g. [Debian's documented split](https://www.debian.org/doc/debian-policy/ch-archive#s-sections)) and the component names should concisely reflect that split.

Entries MUST be added in the /etc/apt/sources.list.d directory using a shortened repository name (e.g. deriv.list). The "Deb822" file format MAY be used instead to improve clarity for complex entries (e.g. deriv.sources). (See [sources.list(5)](https://manpages.debian.org/man/buster/5/sources.list#THE_DEB_AND_DEB-SRC_TYPES:_GENERAL_FORMAT))

For example, this would be the content of the /etc/apt/sources.list.d/deriv.list file:

```
deb [signed-by=/usr/share/keyrings/deriv-archive-keyring.pgp] https://deriv.example.net/debian/ stable main
```

The above is a sources.list line for a fictitious Deriv Debian derivative. The suite is stable and the component is the standard main component.

This is equivalent to the following Deb822 file format, under deriv.sources:

```
Types: deb
URIs: https://deriv.example.net/debian/
Suites: stable
Components: main
Signed-By: /usr/share/keyrings/deriv-archive-keyring.pgp
```

The reason we point to a file instead of a fingerprint is that the latter forces the user to add the certificate to the global [SecureApt](https://web.archive.org/SecureApt) trust anchor in /etc/apt/trusted.gpg.d, which would cause the system to accept signatures from the third-party keyholder on all other repositories configured on the system that don't have a signed-by option (including the official Debian repositories).

Serving the repository under HTTPS is OPTIONAL, as it may make running a round-robin network of untrusted mirrors more difficult, and the trust chain provided by [SecureApt](https://web.archive.org/SecureApt) should suffice.

## Standard pinning

When a repository is added to the sources.list.d, a matching preferences file SHOULD be created to restrict the possible effects of the repository. If such a preferences file is used it MUST pin with a user-controlled label (e.g. the hostname of the URI, or some future local mark, see [858406](https://bugs.debian.org/858406)) and MUST NOT use a field provided by the upstream Release file. The Pin-Priority field MAY be set so that packages are upgraded by default (Pin-Priority: 100) or not (Pin-Priority: 1) but it MUST NOT be set to any higher value that may lead to overwriting packages shipped with the default Debian distribution.

If no preferences file is provided or a different Pin-Priority is used, the user MUST be warned of the security consequences.

For example, this will forbid the deriv.example.net repository from upgrading already installed packages from official repositories, while allowing upgrades to be performed for the deriv repository:

```
Package: *
Pin: origin deriv.example.net
Pin-Priority: 100
```

Alternatively, this configuration will allow the user to install packages from the deriv repository but forbid automated upgrades:

```
Package: *
Pin: origin deriv.example.net
Pin-Priority: 1
```

The above origin configuration has not been audited. It MAY be possible the repository could override that value. Further tests are required to confirm the above configuration is resilient to attack by the repository owner.

Note that if the local system pulls multiple repositories from the same host (e.g. different paths, different suites, or different components), then the proposed Pin: origin is incapable of distinguishing between them. Fixing this appears to require improvements in apt, see [858406](https://bugs.debian.org/858406).

## Certificate rollover and updates

Certificate updates SHOULD be distributed by a Debian package called deriv-archive-keyring. This package MUST distribute the certificate in binary form as /usr/share/keyrings/deriv-archive-keyring.pgp, and MAY also include the /etc/apt/sources.list.d/deriv.sources or /etc/apt/sources.list.d/deriv.list files and the /etc/apt/preferences.d/deriv.pref file.

If such a mechanism is used to distribute certificate updates, the preferences file MUST allow automatic upgrades (Pin-Priority: 100) or include a specific entry for the keyring package that adds an exception for that package:

```
Package: deriv-archive-keyring
Pin: origin deriv.example.net
Pin-Priority: 100
```

## Complete example

This example may serve as a template for instructions provided at the root of the archive to help users configure the APT repository.

This is a Debian repository. To install packages from this repository, you should first download a trust anchor into your system using this command:

```
wget -O /usr/share/keyrings/deriv-archive-keyring.pgp https://deriv.example.net/debian/deriv-archive-keyring.pgp
```

Then you can add the repository to your sources.list by creating a text file in /etc/apt/sources.list.d/deriv.sources containing the following:

```
Types: deb deb-src
URIs: https://deriv.example.net/debian/
Suites: stable
Architectures: i386 amd64
Components: main
Signed-By: /usr/share/keyrings/deriv-archive-keyring.pgp
```

Finally, you should also add the following preferences file to restrict what this repository can install, by creating the following file in /etc/apt/preferences.d/deriv.pref:

```
Package: *
Pin: origin deriv.example.net
Pin-Priority: 100
```

Once this is done, you can run apt-get update for the changes to take effect and use apt-get install deriv-archive-keyring to make sure updates to the keyring are received in a timely manner.

## Troubleshooting

```
W: GPG error: http://deriv.example.net stable InRelease: The following signatures couldn't be verified because the public key is not available: NO_PUBKEY 744ACF4DF3319FFA
```

This error message is commonly seen when the OpenPGP certificate you are using is ASCII-Armored instead of being in binary form.

Sequoia-PGP can handle ASCII-Armored data anywhere where it can handle binary data.

## OpenPGP certificate handling

APT used to be unable to handle ASCII-Armored OpenPGP certificates. And thus as indicated above, you MUST NOT use ASCII-Armored certificates. If you have such a certificate, you should convert it to binary form with something like this with Sequoia-PGP:

```
sq packet dearmor --output $CERT.pgp $CERT.asc
```

Or in a slightly more convoluted way with GnuPG:

```
gpg --import $CERT.asc
gpg --export $CERT_FINGERPRINT >$CERT.pgp
```

Then $CERT.pgp can be distributed everywhere with no backwards compatibility concerns.

## Credits

This document was written by [TheAnarcat](https://web.archive.org/TheAnarcat) with extensive help and review from [DanielKahnGillmor](https://web.archive.org/DanielKahnGillmor).

## References

-

[DebianRepository](https://web.archive.org/DebianRepository) - more documentation on Debian repositories, see in particular [setup info](https://web.archive.org/DebianRepository/Setup) and [format info](https://web.archive.org/DebianRepository/Format) (more information about the different fields in the repository Release file)
-

[SecureApt](https://web.archive.org/SecureApt) - OpenPGP integration in APT
-

[SecureApt/TufDerivedImprovements](https://web.archive.org/SecureApt/TufDerivedImprovements) - possible for the above
-

[SourcesList](https://web.archive.org/SourcesList) - basic documentation on sources.list files
-

[sources.list(5)](https://manpages.debian.org/man/sources.list) - manual page
-

[UntrustedDebs](https://web.archive.org/UntrustedDebs) - the larger effort to make third-party repositories more secure
-

[support for embeddeding OpenPGP certificates directly in the sources.list](https://salsa.debian.org/apt-team/apt/-/merge_requests/176) might improve on this

[CategoryPackageManagement](https://web.archive.org/CategoryPackageManagement)

DebianRepository/UseThirdParty ([last modified 2025-11-02 14:17:40](https://web.archive.org/DebianRepository/UseThirdParty?action=info))

- Changes made after 24 July 2025 00:00 UTC are available under [Creative Commons Attribution-ShareAlike 4.0 International](https://web.archive.org/copyright.html) unless otherwise noted.
- Debian [privacy policy](https://www.debian.org/legal/privacy), Wiki [team](https://web.archive.org/Teams/DebianWiki), [bugs](https://bugs.debian.org/wiki.debian.org) and [config](https://salsa.debian.org/debian/wiki.debian.org).
- Powered by [MoinMoin](https://moinmo.in/) and [Python](https://moinmo.in/Python), with hosting provided by [Metropolitan Area Network Darmstadt](https://www.man-da.de/).
