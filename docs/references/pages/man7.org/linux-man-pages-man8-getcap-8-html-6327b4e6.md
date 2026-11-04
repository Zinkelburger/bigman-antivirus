---
url: "https://man7.org/linux/man-pages/man8/getcap.8.html"
title: "getcap(8) - Linux manual page"
captured: "2026-11-02T18:39:30Z"
via: "live"
sha256: "7bc3fa34225a6d312d0896280b3f24729e5d9521c361d7456f2ef38344eb4ce2"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# getcap(8) - Linux manual page

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://man7.org/linux/man-pages/man8/getcap.8.html>

|

[man7.org](https://man7.org/index.html) > Linux > [man-pages](https://man7.org/linux/man-pages/index.html)

|

[Linux/UNIX system programming training](http://man7.org/training/)

# getcap(8) — Linux manual page

|

[NAME](https://man7.org/linux/man-pages/man8/getcap.8.html#NAME) | [SYNOPSIS](https://man7.org/linux/man-pages/man8/getcap.8.html#SYNOPSIS) | [DESCRIPTION](https://man7.org/linux/man-pages/man8/getcap.8.html#DESCRIPTION) | [OPTIONS](https://man7.org/linux/man-pages/man8/getcap.8.html#OPTIONS) | [REPORTING BUGS](https://man7.org/linux/man-pages/man8/getcap.8.html#REPORTING_BUGS) | [SEE ALSO](https://man7.org/linux/man-pages/man8/getcap.8.html#SEE_ALSO) | [COLOPHON](https://man7.org/linux/man-pages/man8/getcap.8.html#COLOPHON)

|

|

```

GETCAP(8)                System Manager's Manual                GETCAP(8)

```

## https://man7.org/linux/man-pages/man8/getcap.8.html#NAMENAME [top](https://man7.org/linux/man-pages/man8/getcap.8.html#top_of_page)

```

       getcap - examine file capabilities

```

## https://man7.org/linux/man-pages/man8/getcap.8.html#SYNOPSISSYNOPSIS [top](https://man7.org/linux/man-pages/man8/getcap.8.html#top_of_page)

```

       getcap [-v] [-n] [-r] [-h] filename [ ... ]

```

## https://man7.org/linux/man-pages/man8/getcap.8.html#DESCRIPTIONDESCRIPTION [top](https://man7.org/linux/man-pages/man8/getcap.8.html#top_of_page)

```

       getcap displays the name and capabilities of each specified file.

```

## https://man7.org/linux/man-pages/man8/getcap.8.html#OPTIONSOPTIONS [top](https://man7.org/linux/man-pages/man8/getcap.8.html#top_of_page)

```

       -h  prints quick usage.

       -n  prints any non-zero user namespace root user ID value found to
           be associated with a file's capabilities.

       -r  enables recursive search.

       -v  display all searched entries, even if the have no file-
           capabilities.

       NOTE: an empty value of '=' is not equivalent to an omitted (or
       removed) capability on a file. This is most significant with
       respect to the Ambient capability vector, since a process with
       Ambient capabilities will lose them when executing a file having
       '=' capabilities, but will retain the Ambient inheritance of
       privilege when executing a file with an omitted file capability.
       This special empty setting can be used to prevent a binary from
       executing with privilege. For some time, the kernel honored this
       suppression for root executing the file, but the kernel developers
       decided after a number of years that this behavior was unexpected
       for the superuser and reverted it just for that user identity.
       Suppression of root privilege, for a process tree, is possible,
       using the [capsh(1)](https://man7.org/linux/man-pages/man1/capsh.1.html) --mode option.

       filename
           One file per line.

```

## https://man7.org/linux/man-pages/man8/getcap.8.html#REPORTING_BUGSREPORTING BUGS [top](https://man7.org/linux/man-pages/man8/getcap.8.html#top_of_page)

```

       Please report bugs via the bugtracker referenced at:

       https://sites.google.com/site/fullycapable

```

## https://man7.org/linux/man-pages/man8/getcap.8.html#SEE_ALSOSEE ALSO [top](https://man7.org/linux/man-pages/man8/getcap.8.html#top_of_page)

```

       [capsh(1)](https://man7.org/linux/man-pages/man1/capsh.1.html), [cap_get_file(3)](https://man7.org/linux/man-pages/man3/cap_get_file.3.html), [cap_to_text(3)](https://man7.org/linux/man-pages/man3/cap_to_text.3.html), [capabilities(7)](https://man7.org/linux/man-pages/man7/capabilities.7.html),
       [user_namespaces(7)](https://man7.org/linux/man-pages/man7/user_namespaces.7.html), [captree(8)](https://man7.org/linux/man-pages/man8/captree.8.html), [getpcaps(8)](https://man7.org/linux/man-pages/man8/getpcaps.8.html) and [setcap(8)](https://man7.org/linux/man-pages/man8/setcap.8.html).

```

## https://man7.org/linux/man-pages/man8/getcap.8.html#COLOPHONCOLOPHON [top](https://man7.org/linux/man-pages/man8/getcap.8.html#top_of_page)

```

       This page is part of the libcap (capabilities commands and
       library) project.  Information about the project can be found at
       ⟨https://sites.google.com/site/fullycapable/⟩.  If you have a bug
       report for this manual page, send it to morgan@kernel.org (please
       put "libcap" in the Subject line).  This page was obtained from
       the project's upstream Git repository
       ⟨https://git.kernel.org/pub/scm/libs/libcap/libcap.git/⟩ on
       2026-08-04.  (At that time, the date of the most recent commit
       that was found in the repository was 2026-07-30.)  If you discover
       any rendering problems in this HTML version of the page, or you
       believe there is a better or more up-to-date source for the page,
       or you have corrections or improvements to the information in this
       COLOPHON (which is not part of the original manual page), send a
       mail to man-pages@man7.org

                                2026-04-07                      GETCAP(8)

```

Pages that refer to this page: [capsh(1)](https://man7.org/linux/man-pages/man1/capsh.1.html), [libcap(3)](https://man7.org/linux/man-pages/man3/libcap.3.html), [capabilities(7)](https://man7.org/linux/man-pages/man7/capabilities.7.html), [cap_text_formats(7)](https://man7.org/linux/man-pages/man7/cap_text_formats.7.html), [getpcaps(8)](https://man7.org/linux/man-pages/man8/getpcaps.8.html), [setcap(8)](https://man7.org/linux/man-pages/man8/setcap.8.html)

|

HTML rendering created 2026-09-09 by [Michael Kerrisk](https://man7.org/mtk/index.html), author of [The Linux Programming Interface](https://man7.org/tlpi/).

For details of in-depth Linux/UNIX system programming training courses that I teach, look [here](https://man7.org/training/).

Hosting by [jambit GmbH](https://www.jambit.com/index_en.html).

|    |  https://man7.org/tlpi/
