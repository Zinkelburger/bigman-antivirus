---
url: "https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html"
title: "kernel_lockdown(7) - Linux manual page"
captured: "2026-11-02T18:38:30Z"
via: "live"
sha256: "d07ec15ad499bf13b46de33770335433d6a51335cbb27903afd7708d5e505e0a"
wayback: "https://web.archive.org/web/20261020001610/https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# kernel_lockdown(7) - Linux manual page

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html>
> Snapshot: <https://web.archive.org/web/20261020001610/https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html>

|

[man7.org](https://man7.org/index.html) > Linux > [man-pages](https://man7.org/linux/man-pages/index.html)

|

[Linux/UNIX system programming training](http://man7.org/training/)

# kernel_lockdown(7) — Linux manual page

|

[NAME](https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html#NAME) | [DESCRIPTION](https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html#DESCRIPTION) | [VERSIONS](https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html#VERSIONS) | [NOTES](https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html#NOTES) | [COLOPHON](https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html#COLOPHON)

|

|

```

kernel_lockdown(7)   Miscellaneous Information Manual  kernel_lockdown(7)

```

## https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html#NAMENAME [top](https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html#top_of_page)

```

       kernel_lockdown - kernel image access prevention feature

```

## https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html#DESCRIPTIONDESCRIPTION [top](https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html#top_of_page)

```

       The Kernel Lockdown feature is designed to prevent both direct and
       indirect access to a running kernel image, attempting to protect
       against unauthorized modification of the kernel image and to
       prevent access to security and cryptographic data located in
       kernel memory, whilst still permitting driver modules to be
       loaded.

       If a prohibited or restricted feature is accessed or used, the
       kernel will emit a message that looks like:

           Lockdown: X: Y is restricted, see man kernel_lockdown.7

       where X indicates the process name and Y indicates what is
       restricted.

   Coverage
       When lockdown is in effect, a number of features are disabled or
       have their use restricted.  This includes special device files and
       kernel services that allow direct access of the kernel image:

              /dev/mem
              /dev/kmem
              /dev/kcore
              /dev/ioports
              BPF
              kprobes

       and the ability to directly configure and control devices, so as
       to prevent the use of a device to access or modify a kernel image:

       •  The use of module parameters that directly specify hardware
          parameters to drivers through the kernel command line or when
          loading a module.

       •  The use of direct PCI BAR access.

       •  The use of the ioperm and iopl instructions on x86.

       •  The use of the KD*IO console ioctls.

       •  The use of the TIOCSSERIAL serial ioctl.

       •  The alteration of MSR registers on x86.

       •  The replacement of the PCMCIA CIS.

       •  The overriding of ACPI tables.

       •  The use of ACPI error injection.

       •  The specification of the ACPI RDSP address.

       •  The use of ACPI custom methods.

       Certain facilities are restricted:

       •  Only validly signed modules may be loaded (waived if the module
          file being loaded is vouched for by IMA appraisal).

       •  Only validly signed binaries may be kexec'd (waived if the
          binary image file to be executed is vouched for by IMA
          appraisal).

       •  Unencrypted hibernation/suspend to swap are disallowed as the
          kernel image is saved to a medium that can then be accessed.

       •  Use of debugfs is not permitted as this allows a whole range of
          actions including direct configuration of, access to and
          driving of hardware.

       •  IMA requires the addition of the "secure_boot" rules to the
          policy, whether or not they are specified on the command line,
          for both the built-in and custom policies in secure boot
          lockdown mode.

```

## https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html#VERSIONSVERSIONS [top](https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html#top_of_page)

```

       The Kernel Lockdown feature was added in Linux 5.4.

```

## https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html#NOTESNOTES [top](https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html#top_of_page)

```

       The Kernel Lockdown feature is enabled by
       CONFIG_SECURITY_LOCKDOWN_LSM.  The lsm=lsm1,...,lsmN command line
       parameter controls the sequence of the initialization of Linux
       Security Modules.  It must contain the string lockdown to enable
       the Kernel Lockdown feature.  If the command line parameter is not
       specified, the initialization falls back to the value of the
       deprecated security= command line parameter and further to the
       value of CONFIG_LSM.

```

## https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html#COLOPHONCOLOPHON [top](https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html#top_of_page)

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

Linux man-pages 6.19            2026-02-10             kernel_lockdown(7)

```

|

HTML rendering created 2026-09-09 by [Michael Kerrisk](https://man7.org/mtk/index.html), author of [The Linux Programming Interface](https://man7.org/tlpi/).

For details of in-depth Linux/UNIX system programming training courses that I teach, look [here](https://man7.org/training/).

Hosting by [jambit GmbH](https://www.jambit.com/index_en.html).

|    |  https://man7.org/tlpi/
