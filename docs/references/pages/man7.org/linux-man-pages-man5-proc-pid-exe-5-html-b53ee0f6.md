---
url: "https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html"
title: "proc_pid_exe(5) - Linux manual page"
captured: "2026-11-02T18:37:00Z"
via: "live"
sha256: "8c9bf02f95a93763ca8bc46a2e165c1c0e173658124e97d4e3d407d0a0b5afdc"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# proc_pid_exe(5) - Linux manual page

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html>

|

[man7.org](https://man7.org/index.html) > Linux > [man-pages](https://man7.org/linux/man-pages/index.html)

|

[Linux/UNIX system programming training](http://man7.org/training/)

# proc_pid_exe(5) — Linux manual page

|

[NAME](https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html#NAME) | [DESCRIPTION](https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html#DESCRIPTION) | [SEE ALSO](https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html#SEE_ALSO) | [COLOPHON](https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html#COLOPHON)

|

|

```

proc_pid_exe(5)            File Formats Manual            proc_pid_exe(5)

```

## https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html#NAMENAME [top](https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html#top_of_page)

```

       /proc/pid/exe - symbolic link to program pathname

```

## https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html#DESCRIPTIONDESCRIPTION [top](https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html#top_of_page)

```

       /proc/pid/exe
              Under Linux 2.2 and later, this file is a symbolic link
              containing the actual pathname of the executed command.
              This symbolic link can be dereferenced normally; attempting
              to open it will open the executable.  You can even type
              /proc/pid/exe to run another copy of the same executable
              that is being run by process pid.  If the pathname has been
              unlinked, the symbolic link will contain the string
              ' (deleted)' appended to the original pathname.  In a
              multithreaded process, the contents of this symbolic link
              are not available if the main thread has already terminated
              (typically by calling [pthread_exit(3)](https://man7.org/linux/man-pages/man3/pthread_exit.3.html)).

              Permission to dereference or read ([readlink(2)](https://man7.org/linux/man-pages/man2/readlink.2.html)) this
              symbolic link is governed by a ptrace access mode
              PTRACE_MODE_READ_FSCREDS check; see [ptrace(2)](https://man7.org/linux/man-pages/man2/ptrace.2.html).

              Under Linux 2.0 and earlier, /proc/pid/exe is a pointer to
              the binary which was executed, and appears as a symbolic
              link.  A [readlink(2)](https://man7.org/linux/man-pages/man2/readlink.2.html) call on this file under Linux 2.0
              returns a string in the format:

                  [device]:inode

              For example, [0301]:1502 would be inode 1502 on device
              major 03 (IDE, MFM, etc. drives) minor 01 (first partition
              on the first drive).

              [find(1)](https://man7.org/linux/man-pages/man1/find.1.html) with the -inum option can be used to locate the
              file.

```

## https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html#SEE_ALSOSEE ALSO [top](https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html#top_of_page)

```

       [proc(5)](https://man7.org/linux/man-pages/man5/proc.5.html)

```

## https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html#COLOPHONCOLOPHON [top](https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html#top_of_page)

```

       This page is part of the man-pages (Linux kernel and C library
       user-space interface documentation) project.  Information about
       the project can be found at
       ⟨https://www.kernel.org/doc/man-pages/⟩.  If you have a bug report
       for this manual page, see
       ⟨https://git.kernel.org/pub/scm/docs/man-pages/man-pages.git/tree/CONTRIBUTING⟩.
       This page was obtained from the tarball man-pages-6.19.tar.gz
       fetched from
       ⟨https://mirrors.edge.kernel.org/pub/linux/docs/man-pages/⟩ on
       2026-09-09.  If you discover any rendering problems in this HTML
       version of the page, or you believe there is a better or more up-
       to-date source for the page, or you have corrections or
       improvements to the information in this COLOPHON (which is not
       part of the original manual page), send a mail to
       man-pages@man7.org

Linux man-pages 6.19            2026-02-08                proc_pid_exe(5)

```

Pages that refer to this page: [PR_SET_MM_EXE_FILE(2const)](https://man7.org/linux/man-pages/man2/PR_SET_MM_EXE_FILE.2const.html), [proc_pid(5)](https://man7.org/linux/man-pages/man5/proc_pid.5.html)

|

HTML rendering created 2026-09-09 by [Michael Kerrisk](https://man7.org/mtk/index.html), author of [The Linux Programming Interface](https://man7.org/tlpi/).

For details of in-depth Linux/UNIX system programming training courses that I teach, look [here](https://man7.org/training/).

Hosting by [jambit GmbH](https://www.jambit.com/index_en.html).

|    |  https://man7.org/tlpi/
