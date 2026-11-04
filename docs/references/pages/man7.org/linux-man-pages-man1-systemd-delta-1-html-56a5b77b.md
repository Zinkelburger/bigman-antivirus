---
url: "https://man7.org/linux/man-pages/man1/systemd-delta.1.html"
title: "systemd-delta(1) - Linux manual page"
captured: "2026-11-02T18:35:50Z"
via: "live"
sha256: "1f5aa163ef524dd9722f423f1d7a6364910b4b59672ccad7d6f9c9c7e8193d4f"
wayback: "https://web.archive.org/web/20260511071619/https://www.man7.org/linux/man-pages/man1/systemd-delta.1.html"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# systemd-delta(1) - Linux manual page

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://man7.org/linux/man-pages/man1/systemd-delta.1.html>
> Snapshot: <https://web.archive.org/web/20260511071619/https://www.man7.org/linux/man-pages/man1/systemd-delta.1.html>

|

[man7.org](https://man7.org/index.html) > Linux > [man-pages](https://man7.org/linux/man-pages/index.html)

|

[Linux/UNIX system programming training](http://man7.org/training/)

# systemd-delta(1) — Linux manual page

|

[NAME](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#NAME) | [SYNOPSIS](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#SYNOPSIS) | [DESCRIPTION](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#DESCRIPTION) | [OPTIONS](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#OPTIONS) | [EXAMPLES](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#EXAMPLES) | [EXIT STATUS](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#EXIT_STATUS) | [SEE ALSO](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#SEE_ALSO) | [COLOPHON](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#COLOPHON)

|

|

```

SYSTEMD-DELTA(1)              systemd-delta              SYSTEMD-DELTA(1)

```

## https://man7.org/linux/man-pages/man1/systemd-delta.1.html#NAMENAME [top](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#top_of_page)

```

       systemd-delta - Find overridden configuration files

```

## https://man7.org/linux/man-pages/man1/systemd-delta.1.html#SYNOPSISSYNOPSIS [top](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#top_of_page)

```

       systemd-delta [OPTIONS...] [PREFIX | PREFIX/SUFFIX | SUFFIX...]

```

## https://man7.org/linux/man-pages/man1/systemd-delta.1.html#DESCRIPTIONDESCRIPTION [top](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#top_of_page)

```

       systemd-delta may be used to identify and compare configuration
       files that override other configuration files. Files in /etc/ have
       highest priority, files in /run/ have the second highest priority,
       ..., files in /usr/lib/ have lowest priority. Files in a directory
       with higher priority override files with the same name in
       directories of lower priority. In addition, certain configuration
       files can have ".d" directories which contain "drop-in" files with
       configuration snippets which augment the main configuration file.
       "Drop-in" files can be overridden in the same way by placing files
       with the same name in a directory of higher priority (except that,
       in case of "drop-in" files, both the "drop-in" file name and the
       name of the containing directory, which corresponds to the name of
       the main configuration file, must match). For a fuller
       explanation, see [systemd.unit(5)](https://man7.org/linux/man-pages/man5/systemd.unit.5.html).

       The command line argument will be split into a prefix and a
       suffix. Either is optional. The prefix must be one of the
       directories containing configuration files (/etc/, /run/,
       /usr/lib/, ...). If it is given, only overriding files contained
       in this directory will be shown. Otherwise, all overriding files
       will be shown. The suffix must be a name of a subdirectory
       containing configuration files like tmpfiles.d, sysctl.d or
       systemd/system. If it is given, only configuration files in this
       subdirectory (across all configuration paths) will be analyzed.
       Otherwise, all configuration files will be analyzed. If the
       command line argument is not given at all, all configuration files
       will be analyzed. See below for some examples.

```

## https://man7.org/linux/man-pages/man1/systemd-delta.1.html#OPTIONSOPTIONS [top](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#top_of_page)

```

       The following options are understood:

       -t, --type=
           When listing the differences, only list those that are asked
           for. The list itself is a comma-separated list of desired
           difference types.

           Recognized types are:

           masked
               Show masked files

           equivalent
               Show overridden files that while overridden, do not differ
               in content.

           redirected
               Show files that are redirected to another.

           overridden
               Show overridden, and changed files.

           extended
               Show *.conf files in drop-in directories for units.

               Added in version 205.

           unchanged
               Show unmodified files too.

       --diff=
           When showing modified files, when a file is overridden show a
           diff as well. This option takes a boolean argument. If
           omitted, it defaults to true.

       -h, --help
           Print a short help text and exit.

       --version
           Print a short version string and exit.

       --no-pager
           Do not pipe output into a pager.

```

## https://man7.org/linux/man-pages/man1/systemd-delta.1.html#EXAMPLESEXAMPLES [top](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#top_of_page)

```

       To see all local configuration:

           systemd-delta

       To see all runtime configuration:

           systemd-delta /run

       To see all system unit configuration changes:

           systemd-delta systemd/system

       To see all runtime "drop-in" changes for system units:

           systemd-delta --type=extended /run/systemd/system

```

## https://man7.org/linux/man-pages/man1/systemd-delta.1.html#EXIT_STATUSEXIT STATUS [top](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#top_of_page)

```

       On success, 0 is returned, a non-zero failure code otherwise.

```

## https://man7.org/linux/man-pages/man1/systemd-delta.1.html#SEE_ALSOSEE ALSO [top](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#top_of_page)

```

       [systemd(1)](https://man7.org/linux/man-pages/man1/systemd.1.html), [systemd.unit(5)](https://man7.org/linux/man-pages/man5/systemd.unit.5.html)

```

## https://man7.org/linux/man-pages/man1/systemd-delta.1.html#COLOPHONCOLOPHON [top](https://man7.org/linux/man-pages/man1/systemd-delta.1.html#top_of_page)

```

       This page is part of the systemd (systemd system and service
       manager) project.  Information about the project can be found at
       ⟨http://www.freedesktop.org/wiki/Software/systemd⟩.  If you have a
       bug report for this manual page, see
       ⟨http://www.freedesktop.org/wiki/Software/systemd/#bugreports⟩.
       This page was obtained from the project's upstream Git repository
       ⟨https://github.com/systemd/systemd.git⟩ on 2026-08-04.  (At that
       time, the date of the most recent commit that was found in the
       repository was 2026-08-03.)  If you discover any rendering
       problems in this HTML version of the page, or you believe there is
       a better or more up-to-date source for the page, or you have
       corrections or improvements to the information in this COLOPHON
       (which is not part of the original manual page), send a mail to
       man-pages@man7.org

systemd 262~devel                                        SYSTEMD-DELTA(1)

```

Pages that refer to this page: [binfmt.d(5)](https://man7.org/linux/man-pages/man5/binfmt.d.5.html), [modules-load.d(5)](https://man7.org/linux/man-pages/man5/modules-load.d.5.html), [sysctl.d(5)](https://man7.org/linux/man-pages/man5/sysctl.d.5.html), [systemd.preset(5)](https://man7.org/linux/man-pages/man5/systemd.preset.5.html), [tmpfiles.d(5)](https://man7.org/linux/man-pages/man5/tmpfiles.d.5.html), [systemd.directives(7)](https://man7.org/linux/man-pages/man7/systemd.directives.7.html), [systemd.index(7)](https://man7.org/linux/man-pages/man7/systemd.index.7.html)

|

HTML rendering created 2026-09-09 by [Michael Kerrisk](https://man7.org/mtk/index.html), author of [The Linux Programming Interface](https://man7.org/tlpi/).

For details of in-depth Linux/UNIX system programming training courses that I teach, look [here](https://man7.org/training/).

Hosting by [jambit GmbH](https://www.jambit.com/index_en.html).

|    |  https://man7.org/tlpi/
