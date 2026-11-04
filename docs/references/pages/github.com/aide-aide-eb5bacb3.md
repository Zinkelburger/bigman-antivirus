---
url: "https://github.com/aide/aide"
title: "GitHub - aide/aide: aide source code"
site: "GitHub"
captured: "2026-11-02T18:44:19Z"
via: "live"
sha256: "91ea18be23b4f9ec5227881d5a348084213f1b1b441887b2992dda706a9a1507"
wayback: "https://web.archive.org/web/20260818154333/https://github.com/aide/aide"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# GitHub - aide/aide: aide source code

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/aide/aide>
> Snapshot: <https://web.archive.org/web/20260818154333/https://github.com/aide/aide>

## Repository navigation

[aide](https://github.com/aide/aide)

Public

- [Notifications](https://github.com/login?return_to=%2Faide%2Faide)
- [Fork (119)](https://github.com/login?return_to=%2Faide%2Faide)
- [Star (760)](https://github.com/login?return_to=%2Faide%2Faide)

## About

aide source code

### Resources

[Readme](https://github.com/aide/aide#readme-ov-file)

[GPL-2.0 license](https://github.com/aide/aide#GPL-2.0-1-ov-file)

### Contributing

[Contributing](https://github.com/aide/aide#contributing-ov-file)

### Security policy

[Security policy](https://github.com/aide/aide#security-ov-file)

[Activity](https://github.com/aide/aide/activity)

[Custom properties](https://github.com/aide/aide/custom-properties)

### Stars

760 stars

### Watchers

26 watching

### Forks

[119 forks](https://github.com/aide/aide/forks)

[Report repository](https://github.com/contact/report-content?content_url=https%3A%2F%2Fgithub.com%2Faide%2Faide&report=aide+%28user%29)

[Branches](https://github.com/aide/aide/branches)[Tags](https://github.com/aide/aide/tags)

https://github.com/aide/aide/brancheshttps://github.com/aide/aide/tags

## Latest commit

## History

[997 Commits](https://github.com/aide/aide/commits/master/)

https://github.com/aide/aide/commits/master/

## Folders and files

| Name | Name |

Last commit message

|

Last commit date

|

[.github](https://github.com/aide/aide/tree/master/.github)

|

[.github](https://github.com/aide/aide/tree/master/.github)

|

|

|

[doc](https://github.com/aide/aide/tree/master/doc)

|

[doc](https://github.com/aide/aide/tree/master/doc)

|

|

|

[include](https://github.com/aide/aide/tree/master/include)

|

[include](https://github.com/aide/aide/tree/master/include)

|

|

|

[src](https://github.com/aide/aide/tree/master/src)

|

[src](https://github.com/aide/aide/tree/master/src)

|

|

|

[tests](https://github.com/aide/aide/tree/master/tests)

|

[tests](https://github.com/aide/aide/tree/master/tests)

|

|

|

[.gitignore](https://github.com/aide/aide/blob/master/.gitignore)

|

[.gitignore](https://github.com/aide/aide/blob/master/.gitignore)

|

|

|

[AGENTS.md](https://github.com/aide/aide/blob/master/AGENTS.md)

|

[AGENTS.md](https://github.com/aide/aide/blob/master/AGENTS.md)

|

|

|

[AUTHORS](https://github.com/aide/aide/blob/master/AUTHORS)

|

[AUTHORS](https://github.com/aide/aide/blob/master/AUTHORS)

|

|

|

[COPYING](https://github.com/aide/aide/blob/master/COPYING)

|

[COPYING](https://github.com/aide/aide/blob/master/COPYING)

|

|

|

[ChangeLog](https://github.com/aide/aide/blob/master/ChangeLog)

|

[ChangeLog](https://github.com/aide/aide/blob/master/ChangeLog)

|

|

|

[Makefile.am](https://github.com/aide/aide/blob/master/Makefile.am)

|

[Makefile.am](https://github.com/aide/aide/blob/master/Makefile.am)

|

|

|

[NEWS](https://github.com/aide/aide/blob/master/NEWS)

|

[NEWS](https://github.com/aide/aide/blob/master/NEWS)

|

|

|

[README](https://github.com/aide/aide/blob/master/README)

|

[README](https://github.com/aide/aide/blob/master/README)

|

|

|

[SECURITY.md](https://github.com/aide/aide/blob/master/SECURITY.md)

|

[SECURITY.md](https://github.com/aide/aide/blob/master/SECURITY.md)

|

|

|

[acinclude.m4](https://github.com/aide/aide/blob/master/acinclude.m4)

|

[acinclude.m4](https://github.com/aide/aide/blob/master/acinclude.m4)

|

|

|

[autogen.sh](https://github.com/aide/aide/blob/master/autogen.sh)

|

[autogen.sh](https://github.com/aide/aide/blob/master/autogen.sh)

|

|

|

[configure.ac](https://github.com/aide/aide/blob/master/configure.ac)

|

[configure.ac](https://github.com/aide/aide/blob/master/configure.ac)

|

|

|

[version.sh](https://github.com/aide/aide/blob/master/version.sh)

|

[version.sh](https://github.com/aide/aide/blob/master/version.sh)

|

|

|

## Repository files navigation

```

             AIDE - Advanced Intrusion Detection Environment
            -------------------------------------------------
                              Version 0.19

    This file is free software; as a special exception the author gives
    unlimited permission to copy and/or distribute it, with or without
    modifications, as long as this notice is preserved.

    This file is distributed in the hope that it will be useful, but
    WITHOUT ANY WARRANTY, to the extent permitted by law; without even the
    implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

    Introduction
    ------------

    AIDE is a tool for monitoring file system changes. It can be used
    to detect unauthorized monitored files and directories. AIDE was
    written to be a simple and free alternative to Tripwire. Features
    currently included in AIDE are as follows:

        o  File attributes monitored: permissions, inode, user, group
           file size, mtime, atime, ctime, links and growing size.
        o  Multiple checksums and hashes supported
        o  Plain text configuration files and database for simplicity.
        o  Rules, variables and macros that can be customized to local
           site or system policies.
        o  Powerful regular expression support to selectively include or
           exclude files and directories to be monitored.
        o  gzip database compression if zlib support is compiled in.
        o  Free software licensed under the GNU General Public License v2.

    The homepage of AIDE is https://aide.github.io

    Current Version
    ---------------

    AIDE is currently maintained on GitHub.

    Please visit https://github.com/aide/aide/ to get the newest version of
    the source code.

    Documentation
    -------------

    The documentation for AIDE can be found in the doc/ directory.

    Installation
    ------------

    If you are using a git version of the source you need to generate the
    configuration files first:

        $ ./autogen.sh

    For generic installation instructions please see the INSTALL file
    (generated by autogen.sh).

    In short, just type:
        $ ./configure
        $ make
        $ make install

    See './configure --help' for the available configuration options.

    Dynamic versus Static Linking
    -----------------------------

    Formerly aide was linked statically by default to reduce the attack vector
    of compromised shared libraries and to ease client/server monitoring
    configurations. However an attacker could still simply replace the
    statically linked binary, tamper the database file or use dynamically
    loaded kernel modules to change the behaviour of AIDE.

    These days many Linux distributions (eg CentOS/Oracle Linux), operating
    systems (eg Mac OS/OpenSolaris) and libraries have dropped support for
    static linking.

    Hence starting with release v0.18 AIDE is linked dynamically by default.

    To re-enable static linking use '--enable-static' when configuring AIDE.

    Source Code Verification
    ------------------------

    We highly recommend checking that the version of AIDE downloaded and
    installed is an original and unmodified one. You can either verify the
    source tarball or the git tag.

    To check the supplied signature with GnuPG:

      $ gpg --verify aide-<VERSION_NUMBER>.tar.gz.asc

    This checks that the detached signature file is indeed a signature
    of aide-<VERSION_NUMBER>.tar.gz.

    To validate the gpg signature of the git tag:

      $ git verify-tag v<VERSION_NUMBER>

    The current public key needed for signature verification is:

        pub   4096R/68E7B931 2011-06-28 [expires: 2025-06-27]
        uid                  Hannes von Haugwitz <hannes@vonhaugwitz.com>

    If you do not have this key, you can get it from one of the well known PGP
    key servers. You have to make sure that the key you install is not a faked
    one. You can do this with reasonable assurance by comparing the output of:

      $ gpg --fingerprint 2BBBD30FAAB29B3253BCFBA6F6947DAB68E7B931

    with the fingerprint published elsewhere.

    Requirements
    ------------

    AIDE requires the following development tools:

       o  C99 compatible compiler.
       o  GNU Autoconf
       o  GNU Autoconf Macro Archive
       o  GNU Automake
       o  GNU flex.
       o  GNU yacc (bison).
       o  GNU make.
       o  pkg-config
       o  PCRE2 library (libpcre2-8, library with 8-bit code unit support)
       o  libnettle (>= 3.7) or libgcrypt

       o  libcheck (optional, needed for 'make check', license: LGPL-2.1)

    Note:
      flex version 2.5.31 is broken, you might see the following error

       conf_lex.c: In function `conflex':
       conf_lex.c:4728: error: `yy_prev_more_offset' undeclared (first use in
       this function)
       conf_lex.c:4728: error: (Each undeclared identifier is reported only once
       conf_lex.c:4728: error: for each function it appears in.)

      Either downgrade to flex 2.5.4 or get an updated version that fixes
      this bug.

    Large File Support
    -----------------

    To be able to store the size of files larger than 2GB, AIDE needs large
    file support (LFS) to be available in the OS. The configure script
    automatically checks for LFS. To turn off LFS call the configure script
    with the '-disable-largefile' option.

    Feedback and Support
    --------------------

    End user support is available on the AIDE mailing list:

        https://www.ipi.fi/mailman/listinfo/aide

    An archive of the mailing list is available online:

        http://www.ipi.fi/pipermail/aide/

    Please report bugs and feature requests to the aide issue tracker

        https://github.com/aide/aide/issues

    Credits
    -------

    Please see the AUTHORS file.

```

## About

aide source code

### Resources

[Readme](https://github.com/aide/aide#readme-ov-file)

[GPL-2.0 license](https://github.com/aide/aide#GPL-2.0-1-ov-file)

### Contributing

[Contributing](https://github.com/aide/aide#contributing-ov-file)

### Security policy

[Security policy](https://github.com/aide/aide#security-ov-file)

[Activity](https://github.com/aide/aide/activity)

[Custom properties](https://github.com/aide/aide/custom-properties)

### Stars

760 stars

### Watchers

26 watching

### Forks

[119 forks](https://github.com/aide/aide/forks)

[Report repository](https://github.com/contact/report-content?content_url=https%3A%2F%2Fgithub.com%2Faide%2Faide&report=aide+%28user%29)

## Releases

## Used by

## Contributors

## Languages
