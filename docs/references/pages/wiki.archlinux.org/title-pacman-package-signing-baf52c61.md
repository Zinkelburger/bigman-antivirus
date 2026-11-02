---
url: "https://wiki.archlinux.org/title/Pacman/Package_signing"
title: "pacman/Package signing - ArchWiki"
captured: "2026-11-02T18:50:06Z"
via: "live"
sha256: "e3029eee483ee9ed3dfb70aca27d0922f169c835c02780b32e701809b6f059f2"
wayback: "https://web.archive.org/web/20261018140716/https://wiki.archlinux.org/title/Pacman/Package_signing"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# pacman/Package signing - ArchWiki

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://wiki.archlinux.org/title/Pacman/Package_signing>
> Snapshot: <https://web.archive.org/web/20261018140716/https://wiki.archlinux.org/title/Pacman/Package_signing>

# pacman/Package signing

- [Deutsch](https://wiki.archlinux.de/title/Pacman-key)
- [Español](https://wiki.archlinux.org/title/Pacman_(Espa%C3%B1ol)/Package_signing_(Español))
- [Français](https://wiki.archlinux.org/title/Pacman_(Fran%C3%A7ais)/Package_signing_(Français))
- [Magyar](https://wiki.archlinux.org/title/Pacman_(Magyar)/Package_signing_(Magyar))
- [Italiano](https://wiki.archlinux.org/title/Pacman_(Italiano)/Package_signing_(Italiano))
- [日本語](https://wiki.archlinux.jp/index.php/Pacman/%E3%83%91%E3%83%83%E3%82%B1%E3%83%BC%E3%82%B8%E3%81%AE%E7%BD%B2%E5%90%8D)
- [Português](https://wiki.archlinux.org/title/Pacman_(Portugu%C3%AAs)/Package_signing_(Português))
- [Русский](https://wiki.archlinux.org/title/Pacman_(%D0%A0%D1%83%D1%81%D1%81%D0%BA%D0%B8%D0%B9)/Package_signing_(Русский))
- [中文（简体）](https://wiki.archlinuxcn.org/wiki/Pacman/Package_signing)

From ArchWiki

< [Pacman](https://wiki.archlinux.org/title/Pacman)

Related articles

- [GnuPG](https://wiki.archlinux.org/title/GnuPG)
- [OpenPGP](https://wiki.archlinux.org/title/OpenPGP)

To determine if packages are authentic, pacman uses [OpenPGP](https://wiki.archlinux.org/title/OpenPGP) keys in a [web of trust](https://www.gnupg.org/gph/en/manual.html#AEN385) model. The current Master Signing Keys are found [here](https://archlinux.org/master-keys/). At least three of these Master Signing Keys are used to sign the Developers' and Package Maintainers' own keys. They are then used to sign their packages. Each user also has a unique OpenPGP key, which is generated when you configure [pacman-key(8)](https://man.archlinux.org/man/pacman-key.8). It is this web of trust that links the user's key to the master keys.

Examples of webs of trust:

- Custom packages: Packages made and signed with a local key.
- Unofficial packages: Packages made and signed by a developer. Then, a local key was used to sign the developer's key.
- Official packages: Packages made and signed by a developer. The developer's key was signed by the Arch Linux master keys. You used your key to sign the master keys, and you trust them to vouch for developers.

Note The HKP protocol uses 11371/tcp for communication. In order to get the signed keys from the servers (using pacman-key), this port is required for communication.

## Setup

### Configuring pacman

The SigLevel option in /etc/pacman.conf determines the level of trust required to install a package with pacman -S. For a detailed explanation of SigLevel, see [pacman.conf(5) § PACKAGE AND DATABASE SIGNATURE CHECKING](https://man.archlinux.org/man/pacman.conf.5#PACKAGE_AND_DATABASE_SIGNATURE_CHECKING), and the file comments. One can set signature checking globally, or per repository. If SigLevel is set globally in the [options] section, all packages installed with pacman -S will require signing. With the LocalFileSigLevel setting from the default pacman.conf, any packages you build, and install with pacman -U, will not need to be signed using makepkg.

Note Although all official packages are now signed, as of November 2018 signing of the databases is a [work in progress](https://bbs.archlinux.org/viewtopic.php?id=242258). If Required is set then DatabaseOptional should also be set.

For remote packages, the default configuration will only support the installation of packages signed by trusted keys:

```
/etc/pacman.conf
```

```
SigLevel = Required DatabaseOptional
```

TrustedOnly is a default compiled-in pacman parameter. The default configuration is identical to using the global option of:

```
SigLevel = Required DatabaseOptional TrustedOnly

```

The above can be achieved too on a repository level further below in the configuration, e.g.:

```
[core]
SigLevel = PackageRequired
Include = /etc/pacman.d/mirrorlist

```

explicitly adds signature checking for the packages of the repository, but does not require the database to be signed. Optional here would turn off a global Required for this repository.

Warning The SigLevel TrustAll option exists for debugging purposes and makes it very easy to trust keys that have not been verified. You should use TrustedOnly for all official repositories.

### Initializing the keyring

To initialize the pacman keyring run:

```
# pacman-key --init

```

## Managing the keyring

### Verifying the master keys

The initial setup of keys is achieved using:

```
# pacman-key --populate

```

Take time to verify the [Master Signing Keys](https://archlinux.org/master-keys/) when prompted as these are used to co-sign (and therefore trust) all other packager's keys.

OpenPGP keys are too large (2048 bits or more) for humans to work with, so they are usually hashed to create a 40-hex-digit fingerprint which can be used to check by hand that two keys are the same. The last eight digits of the fingerprint serve as a name for the key known as the '(short) key ID' (the last sixteen digits of the fingerprint would be the 'long key ID').

### Adding developer keys

The official Developers' and [Package Maintainers](https://wiki.archlinux.org/title/Package_Maintainers)' keys are signed by the master keys, so you do not need to use pacman-key to sign them yourself. Whenever pacman encounters a key it does not recognize, it will prompt you to download it from a keyserver configured in /etc/pacman.d/gnupg/gpg.conf (or by using the --keyserver option on the command line). Wikipedia maintains a [list of keyservers](https://en.wikipedia.org/wiki/Key_server_(cryptographic)).

Once you have downloaded a developer key, you will not have to download it again, and it can be used to verify any other packages signed by that developer.

Note The [archlinux-keyring](https://archlinux.org/packages/?name=archlinux-keyring) package, which is a dependency of [base](https://archlinux.org/packages/?name=base), contains the latest keys. However keys can also be updated manually using pacman-key --refresh-keys (as root). While doing --refresh-keys, your local key will also be looked up on the remote keyserver, and you will receive a message about it not being found. This is nothing to be concerned about.

### Adding unofficial keys

This article or section needs expansion.

Reason: Explain how to roll out custom keyring packages following [archlinux-keyring](https://archlinux.org/packages/?name=archlinux-keyring). (Discuss in [Talk:Pacman/Package_signing#Addition_of_guide_to_create_unofficial_keyrings](https://wiki.archlinux.org/title/Talk:Pacman/Package_signing#Addition_of_guide_to_create_unofficial_keyrings))

This method can be utilized to add a key to the pacman keyring, or to enable signed [unofficial user repositories](https://wiki.archlinux.org/title/Unofficial_user_repositories).

First, get the key ID (keyid) from its owner. Then add it to the keyring using one of the two methods:

- If the key is found on a keyserver, import it with:

```
# pacman-key --recv-keys keyid
```

- If otherwise a link to a keyfile is provided, download it and then run:

```
# pacman-key --add /path/to/downloaded/keyfile
```

It is recommended to verify the fingerprint, as with any master key or any other key you are going to sign:

```
$ pacman-key --finger keyid

```

Finally, you must locally sign the imported key:

```
# pacman-key --lsign-key keyid

```

You now trust this key to sign packages.

### Debugging with gpg

For debugging purposes, you can access pacman's keyring directly with gpg, e.g.:

```
# gpg --homedir /etc/pacman.d/gnupg --list-keys

```

## Tips and tricks

### Upgrade system regularly

Upgrading the system regularly via [pacman#Upgrading packages](https://wiki.archlinux.org/title/Pacman#Upgrading_packages) prevents most signing errors. If delay is unavoidable and system upgrade gets delayed for an extended period, manually sync the package database and upgrade the [archlinux-keyring](https://archlinux.org/packages/?name=archlinux-keyring) package before system upgrade:

```
# pacman -Sy --needed archlinux-keyring && pacman -Su

```

This command is not considered a [partial upgrade](https://wiki.archlinux.org/title/Partial_upgrade) since it syncs the package database and upgrades the keyring package first. Both must be processed just before starting system upgrade to ensure signatures of all upgraded packages can be properly verified.

Note Since [2022-07-29](https://gitlab.archlinux.org/archlinux/archlinux-keyring/-/commit/ad8698e96c423dfc68405b547f310f2e1075a95d), the archlinux-keyring-wkd-sync.service and the associated [systemd timer](https://wiki.archlinux.org/title/Systemd_timer) have been created and enabled by default, solving signature from "Some Developer <developer_email@archlinux.org>" is marginal trust issues (e.g. [BBS#278332](https://bbs.archlinux.org/viewtopic.php?id=278332)) without requiring any user intervention, by fetching new signatures of already trusted keys once a week.

### Update system time regularly

When the [system time](https://wiki.archlinux.org/title/System_time) is faulty, signing keys could be considered expired (or invalid) and signature checks on packages will fail. Synchronize the system clock regularly by using the [Network Time Protocol daemon](https://wiki.archlinux.org/title/Network_Time_Protocol_daemon).

## Troubleshooting

### Invalid signature errors

pacman-key depends on [system time](https://wiki.archlinux.org/title/System_time). If your system clock is not synchronized, system installation/upgrade may fail with:

```
error: PackageName: signature from "User <email@archlinux.org>" is invalid
error: failed to commit transaction (invalid or corrupted package (PGP signature))
Errors occurred, no packages were upgraded.

```

If using ntpd, correct the system time (as root) with ntpd -qg followed by hwclock -w.

Other NTP clients can be used. See [time synchronization](https://wiki.archlinux.org/title/Time_synchronization).

If correction of the system clock does not resolve the failure, try one of the following approaches:

#### Removing packages from cache

Some packages could be corrupted or may be unsigned, causing failure. Remove each offending package from the system cache rm /var/cache/pacman/pkg/pkgname so it gets freshly downloaded, or [clear the entire cache](https://wiki.archlinux.org/title/Pacman#Cleaning_the_package_cache).

#### Resetting all the keys

Remove or reset all the keys installed in your system by removing the /etc/pacman.d/gnupg directory (as root) and by rerunning pacman-key --init followed by pacman-key --populate to re-add the default keys.

#### Disabling signature checking

Warning Use with caution. Disabling package signing will allow pacman to install untrusted packages.

If you are not concerned about package signing, you can disable OpenPGP signature checking completely. Edit /etc/pacman.conf to have the following lines under [options]:

```
SigLevel = Never
#LocalFileSigLevel = Optional
#RemoteFileSigLevel = Required

```

You need to comment out any repository-specific SigLevel settings because they override the global settings. This will result in no signature checking, which was the behavior before pacman 4. If you do this, you do not need to set up a keyring with pacman-key. You can change those options later if you decide to enable package verification.

### Cannot import keys

This article or section needs language, wiki syntax or style improvements. See [Help:Style](https://wiki.archlinux.org/title/Help:Style) for reference.

Reason: Instructions could be clearer. Not clear how this section is different from the preceding one. Redundant information. (Discuss in [Talk:Pacman/Package signing](https://wiki.archlinux.org/title/Talk:Pacman/Package_signing))

There are multiple possible sources of this problem:

- An outdated [archlinux-keyring](https://archlinux.org/packages/?name=archlinux-keyring) package.
- The clock being set to an incorrect date.
- Your ISP blocked the port used to import OpenPGP keys.
- Your pacman cache contains copies of unsigned packages from previous attempts.
- dirmngr is not correctly configured.

You might be stuck because of an outdated [archlinux-keyring](https://archlinux.org/packages/?name=archlinux-keyring) package when doing an upgrade synchronization.

Below are a few solutions that could work depending on your case.

#### Upgrade the system

See if [upgrading the system](https://wiki.archlinux.org/title/Pacman#Upgrading_packages) can fix it first.

#### Change keyserver

If you suspect that something is not working right with the keyserver, you could try to switch to the Ubuntu keyserver. To do this, edit /etc/pacman.d/gnupg/gpg.conf and change the keyserver line to:

```
keyserver hkp://keyserver.ubuntu.com

```

#### Clean cached packages

If you suspect that your pacman cache at /var/cache/pacman/pkg/ might contain unsigned packages, try cleaning the cache manually or run:

```
# pacman -Sc

```

which removes all cached packages that have not been installed.

### Signature is unknown trust

Sometimes when running pacman -Syu you might encounter this error:

```
error: package-name: signature from "packager" is unknown trust

```

This occurs because the packager's key used in the package package-name is not present and/or not trusted in the local pacman-key gpg database.

This can occur when:

- a new packager recently got their key signed, pushes a package, but your keyring does not contain their key yet,
- a key has expired since it was added to your keychain,
- a new master key is introduced to replace a previous one (usually when a master key holder resigns).

Pacman has built-in mechanisms to handle the first two cases. For an unkown key, pacman will attempt to import the key using WKD first; if it fails, pacman goes back to the old keyserver infrastructure (which will probably fail too). Since version 7.1, pacman will automatically refresh known keys from WKD and keyservers when it encounters an expired key.[[1]](https://reddit.com/r/archlinux/comments/1rafluf/pacman_and_keyring_issues/)

If you encounter the error, mitigate by:

- [manually upgrading the archlinux-keyring package prior to the system upgrade](https://wiki.archlinux.org/title/Pacman/Package_signing#Upgrade_system_regularly), or
- refreshing your keys with pacman-key --refresh-keys, or
- [resetting all the keys](https://wiki.archlinux.org/title/Pacman/Package_signing#Resetting_all_the_keys).

The last two options at your disposal break the chain of trust, and should not be used:

- [manually signing the untrusted key locally](https://wiki.archlinux.org/title/Pacman/Package_signing#Adding_unofficial_keys), or
- setting temporarily SigLevel to TrustAll.

### Updating keys via proxy

In order to use a proxy when updating keys the honor-http-proxy option must be set in both /etc/gnupg/dirmngr.conf and /etc/pacman.d/gnupg/dirmngr.conf. See [GnuPG#Use a keyserver](https://wiki.archlinux.org/title/GnuPG#Use_a_keyserver) for more information.

Note If pacman-key is used without the honor-http-proxy option and fails, a reboot may solve the issue.

### GPGME error: No data

This article or section is a candidate for merging with [pacman#error: GPGME error: No data](https://wiki.archlinux.org/title/Pacman#error:_GPGME_error:_No_data).

Notes: Same issue. (Discuss in [Talk:Pacman/Package signing](https://wiki.archlinux.org/title/Talk:Pacman/Package_signing))

if you running pacman -Syu and encounter this error:

```
error: GPGME error: No data
error: failed to synchronize all databases (invalid or corrupted database (PGP signature))

```

Or having download issue via HTTPS. This is because your network needs authentication. The pacman's request will be redericted to the login page, causing it downloading the wrong file.

Solution: first switch to a new network, then peform

```
rm /var/lib/pacman/sync/*.sig

```

and try pacman -Syu again.

## See also

- [Pacman Package Signing – 1: Makepkg and Repo-add](http://allanmcrae.com/2011/08/pacman-package-signing-1-makepkg-and-repo-add/)
- [Pacman Package Signing – 2: Pacman-key](http://allanmcrae.com/2011/08/pacman-package-signing-2-pacman-key/)
- [Pacman Package Signing – 3: Pacman](http://allanmcrae.com/2011/08/pacman-package-signing-3-pacman/)
- [Pacman Package Signing – 4: Arch Linux](http://allanmcrae.com/2011/12/pacman-package-signing-4-arch-linux/)

Retrieved from "https://wiki.archlinux.org/index.php?title=Pacman/Package_signing&oldid=866946"

[Categories](https://wiki.archlinux.org/title/Special:Categories):

- [OpenPGP](https://wiki.archlinux.org/title/Category:OpenPGP)
- [Package manager](https://wiki.archlinux.org/title/Category:Package_manager)

Hidden categories:

- [Pages or sections flagged with Template:Expansion](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Expansion)
- [Pages or sections flagged with Template:Style](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Style)
- [Pages or sections flagged with Template:Merge](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Merge)
