---
url: "https://man.archlinux.org/man/vercmp.8"
title: "vercmp(8) — Arch manual pages"
captured: "2026-11-02T18:35:44Z"
via: "live"
sha256: "f6bff58344557a8bd380c180bb0671b1a668d16d83735eaaf32dddc28a03f081"
wayback: "https://web.archive.org/web/20260918134416/https://man.archlinux.org/man/vercmp.8"
cited_in:
  - "docs/research/vulnerability-intelligence-and-feeds.md"
---

# vercmp(8) — Arch manual pages

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://man.archlinux.org/man/vercmp.8>
> Snapshot: <https://web.archive.org/web/20260918134416/https://man.archlinux.org/man/vercmp.8>

| VERCMP(8)  | Pacman Manual  | VERCMP(8)

# [NAME](https://man.archlinux.org/man/vercmp.8#NAME)

vercmp - version comparison utility

# [SYNOPSIS](https://man.archlinux.org/man/vercmp.8#SYNOPSIS)

vercmp [-h] [--help] <version1> <version2>

# [DESCRIPTION](https://man.archlinux.org/man/vercmp.8#DESCRIPTION)

vercmp is used to determine the relationship between two given version numbers. It outputs values as follows:

•< 0 : if ver1 < ver2

•= 0 : if ver1 == ver2

•> 0 : if ver1 > ver2

Version comparison operates as follows:

```
Alphanumeric:
  1.0a < 1.0b < 1.0beta < 1.0p < 1.0pre < 1.0rc < 1.0 < 1.0.a < 1.0.1
Numeric:
  1 < 1.0 < 1.1 < 1.1.1 < 1.2 < 2.0 < 3.0.0
```

Additionally, version strings can have an epoch value defined that will overrule any version comparison, unless the epoch values are equal. This is specified in an epoch:version-rel format. For example, 2:1.0-1 is always greater than 1:3.6-1.

Keep in mind that the pkgrel is only compared if it is available on both versions given to this tool. For example, comparing 1.5-1 and 1.5 will yield 0; comparing 1.5-1 and 1.5-2 will yield < 0 as expected. This is mainly for supporting versioned dependencies that do not include the pkgrel.

# [OPTIONS](https://man.archlinux.org/man/vercmp.8#OPTIONS)

-h, --help

Display summary of the available return codes. Must be the first option specified.

# [EXAMPLES](https://man.archlinux.org/man/vercmp.8#EXAMPLES)

```
$ vercmp 1 2
-1
```

```
$ vercmp 2 1
1
```

```
$ vercmp 2.0-1 1.7-6
1
```

```
$ vercmp 2.0 2.0-13
0
```

```
$ vercmp 4.34 1:001
-1
```

# [SEE ALSO](https://man.archlinux.org/man/vercmp.8#SEE_ALSO)

[pacman(8)](https://man.archlinux.org/man/pacman.8.en), [makepkg(8)](https://man.archlinux.org/man/makepkg.8.en), [libalpm(3)](https://man.archlinux.org/man/libalpm.3.en)

See the pacman website at https://archlinux.org/pacman/ for current information on pacman and its related tools.

# [BUGS](https://man.archlinux.org/man/vercmp.8#BUGS)

Bugs? You must be kidding; there are no bugs in this software. But if we happen to be wrong, please report them to the issue tracker at https://gitlab.archlinux.org/pacman/pacman/-/issues with specific information such as your command-line, the nature of the bug, and even the package database if it helps.

# [AUTHORS](https://man.archlinux.org/man/vercmp.8#AUTHORS)

Current maintainers:

•Allan McRae <allan@archlinux.org>

•Andrew Gregory <andrew.gregory.8@gmail.com>

•Morgan Adamiec <morganamilo@archlinux.org>

Past major contributors:

•Judd Vinet <jvinet@zeroflux.org>

•Aurelien Foret <aurelien@archlinux.org>

•Aaron Griffin <aaron@archlinux.org>

•Dan McGee <dan@archlinux.org>

•Xavier Chantry <shiningxc@gmail.com>

•Nagy Gabor <ngaba@bibl.u-szeged.hu>

•Dave Reisner <dreisner@archlinux.org>

•Eli Schwartz <eschwartz@archlinux.org>

For additional contributors, use git shortlog -s on the pacman.git repository.

| 2026-10-15  | Pacman 7.1.0
