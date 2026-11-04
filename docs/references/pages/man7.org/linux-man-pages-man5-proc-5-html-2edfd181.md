---
url: "https://man7.org/linux/man-pages/man5/proc.5.html"
title: "proc(5) - Linux manual page"
captured: "2026-11-02T18:36:02Z"
via: "live"
sha256: "b197c5fa64577269f992d50f60acfd5e0cc3df756644ddd8bd19a02591272a97"
wayback: "https://web.archive.org/web/20261023032509/https://man7.org/linux/man-pages/man5/proc.5.html"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# proc(5) - Linux manual page

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://man7.org/linux/man-pages/man5/proc.5.html>
> Snapshot: <https://web.archive.org/web/20261023032509/https://man7.org/linux/man-pages/man5/proc.5.html>

|

[man7.org](https://man7.org/index.html) > Linux > [man-pages](https://man7.org/linux/man-pages/index.html)

|

[Linux/UNIX system programming training](http://man7.org/training/)

# proc(5) — Linux manual page

|

[NAME](https://man7.org/linux/man-pages/man5/proc.5.html#NAME) | [DESCRIPTION](https://man7.org/linux/man-pages/man5/proc.5.html#DESCRIPTION) | [NOTES](https://man7.org/linux/man-pages/man5/proc.5.html#NOTES) | [SEE ALSO](https://man7.org/linux/man-pages/man5/proc.5.html#SEE_ALSO) | [COLOPHON](https://man7.org/linux/man-pages/man5/proc.5.html#COLOPHON)

|

|

```

proc(5)                    File Formats Manual                    proc(5)

```

## https://man7.org/linux/man-pages/man5/proc.5.html#NAMENAME [top](https://man7.org/linux/man-pages/man5/proc.5.html#top_of_page)

```

       proc - process information, system information, and sysctl pseudo-
       filesystem

```

## https://man7.org/linux/man-pages/man5/proc.5.html#DESCRIPTIONDESCRIPTION [top](https://man7.org/linux/man-pages/man5/proc.5.html#top_of_page)

```

       The proc filesystem is a pseudo-filesystem which provides an
       interface to kernel data structures.  It is commonly mounted at
       /proc.  Typically, it is mounted automatically by the system, but
       it can also be mounted manually using a command such as:

           mount -t proc proc /proc

       Most of the files in the proc filesystem are read-only, but some
       files are writable, allowing kernel variables to be changed.

   Mount options
       The proc filesystem supports the following mount options:

       hidepid=n (since Linux 3.3)
              This option controls who can access the information in
              /proc/pid directories.  The argument, n, is one of the
              following values:

              0   Everybody may access all /proc/pid directories.  This
                  is the traditional behavior, and the default if this
                  mount option is not specified.

              1   Users may not access files and subdirectories inside
                  any /proc/pid directories but their own (the /proc/pid
                  directories themselves remain visible).  Sensitive
                  files such as /proc/pid/cmdline and /proc/pid/status
                  are now protected against other users.  This makes it
                  impossible to learn whether any user is running a
                  specific program (so long as the program doesn't
                  otherwise reveal itself by its behavior).

              2   As for mode 1, but in addition the /proc/pid
                  directories belonging to other users become invisible.
                  This means that /proc/pid entries can no longer be used
                  to discover the PIDs on the system.  This doesn't hide
                  the fact that a process with a specific PID value
                  exists (it can be learned by other means, for example,
                  by "kill -0 $PID"), but it hides a process's UID and
                  GID, which could otherwise be learned by employing
                  [stat(2)](https://man7.org/linux/man-pages/man2/stat.2.html) on a /proc/pid directory.  This greatly
                  complicates an attacker's task of gathering information
                  about running processes (e.g., discovering whether some
                  daemon is running with elevated privileges, whether
                  another user is running some sensitive program, whether
                  other users are running any program at all, and so on).

              gid=gid (since Linux 3.3)
                  Specifies the ID of a group whose members are
                  authorized to learn process information otherwise
                  prohibited by hidepid (i.e., users in this group behave
                  as though /proc was mounted with hidepid=0).  This
                  group should be used instead of approaches such as
                  putting nonroot users into the [sudoers(5)](https://man7.org/linux/man-pages/man5/sudoers.5.html) file.

       subset=pid (since Linux 5.8)
              Show only the specified subset of procfs, hiding all top
              level files and directories in the procfs that are not
              related to tasks.

   Overview
       Underneath /proc, there are the following general groups of files
       and subdirectories:

       /proc/pid subdirectories
              See [proc_pid(5)](https://man7.org/linux/man-pages/man5/proc_pid.5.html).

              Each one of these subdirectories contains files and
              subdirectories exposing information about the process with
              the corresponding process ID.

              Underneath each of the /proc/pid directories, a task
              subdirectory contains subdirectories of the form task/tid,
              which contain corresponding information about each of the
              threads in the process, where tid is the kernel thread ID
              of the thread.

              The /proc/pid subdirectories are visible when iterating
              through /proc with [getdents(2)](https://man7.org/linux/man-pages/man2/getdents.2.html) (and thus are visible when
              one uses [ls(1)](https://man7.org/linux/man-pages/man1/ls.1.html) to view the contents of /proc).

       /proc/tid subdirectories
              See [proc_tid(5)](https://man7.org/linux/man-pages/man5/proc_tid.5.html).

              Each one of these subdirectories contains files and
              subdirectories exposing information about the thread with
              the corresponding thread ID.  The contents of these
              directories are the same as the corresponding
              /proc/pid/task/tid directories.

              The /proc/tid subdirectories are not visible when iterating
              through /proc with [getdents(2)](https://man7.org/linux/man-pages/man2/getdents.2.html) (and thus are not visible
              when one uses [ls(1)](https://man7.org/linux/man-pages/man1/ls.1.html) to view the contents of /proc).

       /proc/self
              See [proc_self(5)](https://man7.org/linux/man-pages/man5/proc_self.5.html).

              When a process accesses this magic symbolic link, it
              resolves to the process's own /proc/pid directory.

       /proc/thread-self
              See [proc_thread-self(5)](https://man7.org/linux/man-pages/man5/proc_thread-self.5.html).

              When a thread accesses this magic symbolic link, it
              resolves to the process's own /proc/self/task/tid
              directory.

       /proc/[a-z]*

              Various other files and subdirectories under /proc expose
              system-wide information.

              See also:

              [proc_apm(5)](https://man7.org/linux/man-pages/man5/proc_apm.5.html)
              [proc_buddyinfo(5)](https://man7.org/linux/man-pages/man5/proc_buddyinfo.5.html)
              [proc_bus(5)](https://man7.org/linux/man-pages/man5/proc_bus.5.html)
              [proc_cgroups(5)](https://man7.org/linux/man-pages/man5/proc_cgroups.5.html)
              [proc_cmdline(5)](https://man7.org/linux/man-pages/man5/proc_cmdline.5.html)
              [proc_config.gz(5)](https://man7.org/linux/man-pages/man5/proc_config.gz.5.html)
              [proc_cpuinfo(5)](https://man7.org/linux/man-pages/man5/proc_cpuinfo.5.html)
              [proc_crypto(5)](https://man7.org/linux/man-pages/man5/proc_crypto.5.html)
              [proc_devices(5)](https://man7.org/linux/man-pages/man5/proc_devices.5.html)
              [proc_diskstats(5)](https://man7.org/linux/man-pages/man5/proc_diskstats.5.html)
              [proc_dma(5)](https://man7.org/linux/man-pages/man5/proc_dma.5.html)
              [proc_driver(5)](https://man7.org/linux/man-pages/man5/proc_driver.5.html)
              [proc_execdomains(5)](https://man7.org/linux/man-pages/man5/proc_execdomains.5.html)
              [proc_fb(5)](https://man7.org/linux/man-pages/man5/proc_fb.5.html)
              [proc_filesystems(5)](https://man7.org/linux/man-pages/man5/proc_filesystems.5.html)
              [proc_fs(5)](https://man7.org/linux/man-pages/man5/proc_fs.5.html)
              [proc_ide(5)](https://man7.org/linux/man-pages/man5/proc_ide.5.html)
              [proc_interrupts(5)](https://man7.org/linux/man-pages/man5/proc_interrupts.5.html)
              [proc_iomem(5)](https://man7.org/linux/man-pages/man5/proc_iomem.5.html)
              [proc_ioports(5)](https://man7.org/linux/man-pages/man5/proc_ioports.5.html)
              [proc_kallsyms(5)](https://man7.org/linux/man-pages/man5/proc_kallsyms.5.html)
              [proc_kcore(5)](https://man7.org/linux/man-pages/man5/proc_kcore.5.html)
              [proc_key-users(5)](https://man7.org/linux/man-pages/man5/proc_key-users.5.html)
              [proc_keys(5)](https://man7.org/linux/man-pages/man5/proc_keys.5.html)
              [proc_kmsg(5)](https://man7.org/linux/man-pages/man5/proc_kmsg.5.html)
              [proc_kpagecgroup(5)](https://man7.org/linux/man-pages/man5/proc_kpagecgroup.5.html)
              [proc_kpagecount(5)](https://man7.org/linux/man-pages/man5/proc_kpagecount.5.html)
              [proc_kpageflags(5)](https://man7.org/linux/man-pages/man5/proc_kpageflags.5.html)
              [proc_ksyms(5)](https://man7.org/linux/man-pages/man5/proc_ksyms.5.html)
              [proc_loadavg(5)](https://man7.org/linux/man-pages/man5/proc_loadavg.5.html)
              [proc_locks(5)](https://man7.org/linux/man-pages/man5/proc_locks.5.html)
              [proc_malloc(5)](https://man7.org/linux/man-pages/man5/proc_malloc.5.html)
              [proc_meminfo(5)](https://man7.org/linux/man-pages/man5/proc_meminfo.5.html)
              [proc_modules(5)](https://man7.org/linux/man-pages/man5/proc_modules.5.html)
              [proc_mounts(5)](https://man7.org/linux/man-pages/man5/proc_mounts.5.html)
              [proc_mtrr(5)](https://man7.org/linux/man-pages/man5/proc_mtrr.5.html)
              [proc_net(5)](https://man7.org/linux/man-pages/man5/proc_net.5.html)
              [proc_partitions(5)](https://man7.org/linux/man-pages/man5/proc_partitions.5.html)
              [proc_pci(5)](https://man7.org/linux/man-pages/man5/proc_pci.5.html)
              [proc_profile(5)](https://man7.org/linux/man-pages/man5/proc_profile.5.html)
              [proc_scsi(5)](https://man7.org/linux/man-pages/man5/proc_scsi.5.html)
              [proc_slabinfo(5)](https://man7.org/linux/man-pages/man5/proc_slabinfo.5.html)
              [proc_stat(5)](https://man7.org/linux/man-pages/man5/proc_stat.5.html)
              [proc_swaps(5)](https://man7.org/linux/man-pages/man5/proc_swaps.5.html)
              [proc_sys(5)](https://man7.org/linux/man-pages/man5/proc_sys.5.html)
              [proc_sysrq-trigger(5)](https://man7.org/linux/man-pages/man5/proc_sysrq-trigger.5.html)
              [proc_sysvipc(5)](https://man7.org/linux/man-pages/man5/proc_sysvipc.5.html)
              [proc_timer_list(5)](https://man7.org/linux/man-pages/man5/proc_timer_list.5.html)
              [proc_timer_stats(5)](https://man7.org/linux/man-pages/man5/proc_timer_stats.5.html)
              [proc_tty(5)](https://man7.org/linux/man-pages/man5/proc_tty.5.html)
              [proc_uptime(5)](https://man7.org/linux/man-pages/man5/proc_uptime.5.html)
              [proc_version(5)](https://man7.org/linux/man-pages/man5/proc_version.5.html)
              [proc_vmstat(5)](https://man7.org/linux/man-pages/man5/proc_vmstat.5.html)
              [proc_zoneinfo(5)](https://man7.org/linux/man-pages/man5/proc_zoneinfo.5.html)

```

## https://man7.org/linux/man-pages/man5/proc.5.html#NOTESNOTES [top](https://man7.org/linux/man-pages/man5/proc.5.html#top_of_page)

```

       Many files contain strings (e.g., the environment and command
       line) that are in the internal format, with subfields terminated
       by null bytes ('\0').  When inspecting such files, you may find
       that the results are more readable if you use a command of the
       following form to display them:

           $ cat file | tr '\000' '\n'

```

## https://man7.org/linux/man-pages/man5/proc.5.html#SEE_ALSOSEE ALSO [top](https://man7.org/linux/man-pages/man5/proc.5.html#top_of_page)

```

       [cat(1)](https://man7.org/linux/man-pages/man1/cat.1.html), [dmesg(1)](https://man7.org/linux/man-pages/man1/dmesg.1.html), [find(1)](https://man7.org/linux/man-pages/man1/find.1.html), [free(1)](https://man7.org/linux/man-pages/man1/free.1.html), [htop(1)](https://man7.org/linux/man-pages/man1/htop.1.html), [init(1)](https://man7.org/linux/man-pages/man1/init.1.html), [ps(1)](https://man7.org/linux/man-pages/man1/ps.1.html),
       [pstree(1)](https://man7.org/linux/man-pages/man1/pstree.1.html), [tr(1)](https://man7.org/linux/man-pages/man1/tr.1.html), [uptime(1)](https://man7.org/linux/man-pages/man1/uptime.1.html), [chroot(2)](https://man7.org/linux/man-pages/man2/chroot.2.html), [mmap(2)](https://man7.org/linux/man-pages/man2/mmap.2.html), [readlink(2)](https://man7.org/linux/man-pages/man2/readlink.2.html),
       [syslog(2)](https://man7.org/linux/man-pages/man2/syslog.2.html), [slabinfo(5)](https://man7.org/linux/man-pages/man5/slabinfo.5.html), [sysfs(5)](https://man7.org/linux/man-pages/man5/sysfs.5.html), [hier(7)](https://man7.org/linux/man-pages/man7/hier.7.html), [namespaces(7)](https://man7.org/linux/man-pages/man7/namespaces.7.html), [time(7)](https://man7.org/linux/man-pages/man7/time.7.html),
       [arp(8)](https://man7.org/linux/man-pages/man8/arp.8.html), [hdparm(8)](https://man7.org/linux/man-pages/man8/hdparm.8.html), [ifconfig(8)](https://man7.org/linux/man-pages/man8/ifconfig.8.html), [lsmod(8)](https://man7.org/linux/man-pages/man8/lsmod.8.html), [lspci(8)](https://man7.org/linux/man-pages/man8/lspci.8.html), [mount(8)](https://man7.org/linux/man-pages/man8/mount.8.html),
       [netstat(8)](https://man7.org/linux/man-pages/man8/netstat.8.html), procinfo(8), [route(8)](https://man7.org/linux/man-pages/man8/route.8.html), [sysctl(8)](https://man7.org/linux/man-pages/man8/sysctl.8.html)

       The Linux kernel source files: Documentation/filesystems/proc.rst,
       Documentation/admin-guide/sysctl/fs.rst,
       Documentation/admin-guide/sysctl/kernel.rst,
       Documentation/admin-guide/sysctl/net.rst, and
       Documentation/admin-guide/sysctl/vm.rst.

```

## https://man7.org/linux/man-pages/man5/proc.5.html#COLOPHONCOLOPHON [top](https://man7.org/linux/man-pages/man5/proc.5.html#top_of_page)

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

Linux man-pages 6.19            2026-02-08                        proc(5)

```

Pages that refer to this page: [choom(1)](https://man7.org/linux/man-pages/man1/choom.1.html), [htop(1)](https://man7.org/linux/man-pages/man1/htop.1.html), [kill(1)](https://man7.org/linux/man-pages/man1/kill.1.html), [lsfd(1)](https://man7.org/linux/man-pages/man1/lsfd.1.html), [ps(1)](https://man7.org/linux/man-pages/man1/ps.1.html), [pstree(1)](https://man7.org/linux/man-pages/man1/pstree.1.html), [strace(1)](https://man7.org/linux/man-pages/man1/strace.1.html), [systemd-nspawn(1)](https://man7.org/linux/man-pages/man1/systemd-nspawn.1.html), [unshare(1)](https://man7.org/linux/man-pages/man1/unshare.1.html), [chroot(2)](https://man7.org/linux/man-pages/man2/chroot.2.html), [close_range(2)](https://man7.org/linux/man-pages/man2/close_range.2.html), [delete_module(2)](https://man7.org/linux/man-pages/man2/delete_module.2.html), [eventfd(2)](https://man7.org/linux/man-pages/man2/eventfd.2.html), [execve(2)](https://man7.org/linux/man-pages/man2/execve.2.html), [fork(2)](https://man7.org/linux/man-pages/man2/fork.2.html), [getrlimit(2)](https://man7.org/linux/man-pages/man2/getrlimit.2.html), [init_module(2)](https://man7.org/linux/man-pages/man2/init_module.2.html), [ioctl_nsfs(2)](https://man7.org/linux/man-pages/man2/ioctl_nsfs.2.html), [kcmp(2)](https://man7.org/linux/man-pages/man2/kcmp.2.html), [mlock(2)](https://man7.org/linux/man-pages/man2/mlock.2.html), [mmap(2)](https://man7.org/linux/man-pages/man2/mmap.2.html), [mount(2)](https://man7.org/linux/man-pages/man2/mount.2.html), [mount_setattr(2)](https://man7.org/linux/man-pages/man2/mount_setattr.2.html), [msgctl(2)](https://man7.org/linux/man-pages/man2/msgctl.2.html), [open(2)](https://man7.org/linux/man-pages/man2/open.2.html), [openat2(2)](https://man7.org/linux/man-pages/man2/openat2.2.html), [pidfd_open(2)](https://man7.org/linux/man-pages/man2/pidfd_open.2.html), [posix_fadvise(2)](https://man7.org/linux/man-pages/man2/posix_fadvise.2.html), [seccomp(2)](https://man7.org/linux/man-pages/man2/seccomp.2.html), [seccomp_unotify(2)](https://man7.org/linux/man-pages/man2/seccomp_unotify.2.html), [semctl(2)](https://man7.org/linux/man-pages/man2/semctl.2.html), [shmctl(2)](https://man7.org/linux/man-pages/man2/shmctl.2.html), [signalfd(2)](https://man7.org/linux/man-pages/man2/signalfd.2.html), [statx(2)](https://man7.org/linux/man-pages/man2/statx.2.html), [symlink(2)](https://man7.org/linux/man-pages/man2/symlink.2.html), [sysctl(2)](https://man7.org/linux/man-pages/man2/sysctl.2.html), [sysfs(2)](https://man7.org/linux/man-pages/man2/sysfs.2.html), [sysinfo(2)](https://man7.org/linux/man-pages/man2/sysinfo.2.html), [timer_create(2)](https://man7.org/linux/man-pages/man2/timer_create.2.html), [vfork(2)](https://man7.org/linux/man-pages/man2/vfork.2.html), [errno(3)](https://man7.org/linux/man-pages/man3/errno.3.html), [fexecve(3)](https://man7.org/linux/man-pages/man3/fexecve.3.html), [getauxval(3)](https://man7.org/linux/man-pages/man3/getauxval.3.html), [getloadavg(3)](https://man7.org/linux/man-pages/man3/getloadavg.3.html), [malloc(3)](https://man7.org/linux/man-pages/man3/malloc.3.html), [mallopt(3)](https://man7.org/linux/man-pages/man3/mallopt.3.html), [procps(3)](https://man7.org/linux/man-pages/man3/procps.3.html), [procps_misc(3)](https://man7.org/linux/man-pages/man3/procps_misc.3.html), [procps_pids(3)](https://man7.org/linux/man-pages/man3/procps_pids.3.html), [program_invocation_name(3)](https://man7.org/linux/man-pages/man3/program_invocation_name.3.html), [pthread_create(3)](https://man7.org/linux/man-pages/man3/pthread_create.3.html), [sd_bus_creds_get_pid(3)](https://man7.org/linux/man-pages/man3/sd_bus_creds_get_pid.3.html), [acct(5)](https://man7.org/linux/man-pages/man5/acct.5.html), [core(5)](https://man7.org/linux/man-pages/man5/core.5.html), [filesystems(5)](https://man7.org/linux/man-pages/man5/filesystems.5.html), [proc_apm(5)](https://man7.org/linux/man-pages/man5/proc_apm.5.html), [proc_buddyinfo(5)](https://man7.org/linux/man-pages/man5/proc_buddyinfo.5.html), [proc_bus(5)](https://man7.org/linux/man-pages/man5/proc_bus.5.html), [proc_cgroups(5)](https://man7.org/linux/man-pages/man5/proc_cgroups.5.html), [proc_cmdline(5)](https://man7.org/linux/man-pages/man5/proc_cmdline.5.html), [proc_config.gz(5)](https://man7.org/linux/man-pages/man5/proc_config.gz.5.html), [proc_cpuinfo(5)](https://man7.org/linux/man-pages/man5/proc_cpuinfo.5.html), [proc_crypto(5)](https://man7.org/linux/man-pages/man5/proc_crypto.5.html), [proc_devices(5)](https://man7.org/linux/man-pages/man5/proc_devices.5.html), [proc_diskstats(5)](https://man7.org/linux/man-pages/man5/proc_diskstats.5.html), [proc_dma(5)](https://man7.org/linux/man-pages/man5/proc_dma.5.html), [proc_driver(5)](https://man7.org/linux/man-pages/man5/proc_driver.5.html), [proc_execdomains(5)](https://man7.org/linux/man-pages/man5/proc_execdomains.5.html), [proc_fb(5)](https://man7.org/linux/man-pages/man5/proc_fb.5.html), [proc_filesystems(5)](https://man7.org/linux/man-pages/man5/proc_filesystems.5.html), [proc_fs(5)](https://man7.org/linux/man-pages/man5/proc_fs.5.html), [proc_ide(5)](https://man7.org/linux/man-pages/man5/proc_ide.5.html), [proc_interrupts(5)](https://man7.org/linux/man-pages/man5/proc_interrupts.5.html), [proc_iomem(5)](https://man7.org/linux/man-pages/man5/proc_iomem.5.html), [proc_ioports(5)](https://man7.org/linux/man-pages/man5/proc_ioports.5.html), [proc_kallsyms(5)](https://man7.org/linux/man-pages/man5/proc_kallsyms.5.html), [proc_kcore(5)](https://man7.org/linux/man-pages/man5/proc_kcore.5.html), [proc_keys(5)](https://man7.org/linux/man-pages/man5/proc_keys.5.html), [proc_kmsg(5)](https://man7.org/linux/man-pages/man5/proc_kmsg.5.html), [proc_kpagecgroup(5)](https://man7.org/linux/man-pages/man5/proc_kpagecgroup.5.html), [proc_kpagecount(5)](https://man7.org/linux/man-pages/man5/proc_kpagecount.5.html), [proc_kpageflags(5)](https://man7.org/linux/man-pages/man5/proc_kpageflags.5.html), [proc_loadavg(5)](https://man7.org/linux/man-pages/man5/proc_loadavg.5.html), [proc_locks(5)](https://man7.org/linux/man-pages/man5/proc_locks.5.html), [proc_malloc(5)](https://man7.org/linux/man-pages/man5/proc_malloc.5.html), [proc_meminfo(5)](https://man7.org/linux/man-pages/man5/proc_meminfo.5.html), [proc_modules(5)](https://man7.org/linux/man-pages/man5/proc_modules.5.html), [proc_mtrr(5)](https://man7.org/linux/man-pages/man5/proc_mtrr.5.html), [proc_partitions(5)](https://man7.org/linux/man-pages/man5/proc_partitions.5.html), [proc_pci(5)](https://man7.org/linux/man-pages/man5/proc_pci.5.html), [proc_pid(5)](https://man7.org/linux/man-pages/man5/proc_pid.5.html), [proc_pid_attr(5)](https://man7.org/linux/man-pages/man5/proc_pid_attr.5.html), [proc_pid_autogroup(5)](https://man7.org/linux/man-pages/man5/proc_pid_autogroup.5.html), [proc_pid_auxv(5)](https://man7.org/linux/man-pages/man5/proc_pid_auxv.5.html), [proc_pid_cgroup(5)](https://man7.org/linux/man-pages/man5/proc_pid_cgroup.5.html), [proc_pid_clear_refs(5)](https://man7.org/linux/man-pages/man5/proc_pid_clear_refs.5.html), [proc_pid_cmdline(5)](https://man7.org/linux/man-pages/man5/proc_pid_cmdline.5.html), [proc_pid_comm(5)](https://man7.org/linux/man-pages/man5/proc_pid_comm.5.html), [proc_pid_coredump_filter(5)](https://man7.org/linux/man-pages/man5/proc_pid_coredump_filter.5.html), [proc_pid_cpuset(5)](https://man7.org/linux/man-pages/man5/proc_pid_cpuset.5.html), [proc_pid_cwd(5)](https://man7.org/linux/man-pages/man5/proc_pid_cwd.5.html), [proc_pid_environ(5)](https://man7.org/linux/man-pages/man5/proc_pid_environ.5.html), [proc_pid_exe(5)](https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html), [proc_pid_fd(5)](https://man7.org/linux/man-pages/man5/proc_pid_fd.5.html), [proc_pid_fdinfo(5)](https://man7.org/linux/man-pages/man5/proc_pid_fdinfo.5.html), [proc_pid_io(5)](https://man7.org/linux/man-pages/man5/proc_pid_io.5.html), [proc_pid_limits(5)](https://man7.org/linux/man-pages/man5/proc_pid_limits.5.html), [proc_pid_map_files(5)](https://man7.org/linux/man-pages/man5/proc_pid_map_files.5.html), [proc_pid_maps(5)](https://man7.org/linux/man-pages/man5/proc_pid_maps.5.html), [proc_pid_mem(5)](https://man7.org/linux/man-pages/man5/proc_pid_mem.5.html), [proc_pid_mountinfo(5)](https://man7.org/linux/man-pages/man5/proc_pid_mountinfo.5.html), [proc_pid_mounts(5)](https://man7.org/linux/man-pages/man5/proc_pid_mounts.5.html), [proc_pid_mountstats(5)](https://man7.org/linux/man-pages/man5/proc_pid_mountstats.5.html), [proc_pid_net(5)](https://man7.org/linux/man-pages/man5/proc_pid_net.5.html), [proc_pid_ns(5)](https://man7.org/linux/man-pages/man5/proc_pid_ns.5.html), [proc_pid_numa_maps(5)](https://man7.org/linux/man-pages/man5/proc_pid_numa_maps.5.html), [proc_pid_oom_score(5)](https://man7.org/linux/man-pages/man5/proc_pid_oom_score.5.html), [proc_pid_oom_score_adj(5)](https://man7.org/linux/man-pages/man5/proc_pid_oom_score_adj.5.html), [proc_pid_pagemap(5)](https://man7.org/linux/man-pages/man5/proc_pid_pagemap.5.html), [proc_pid_personality(5)](https://man7.org/linux/man-pages/man5/proc_pid_personality.5.html), [proc_pid_projid_map(5)](https://man7.org/linux/man-pages/man5/proc_pid_projid_map.5.html), [proc_pid_root(5)](https://man7.org/linux/man-pages/man5/proc_pid_root.5.html), [proc_pid_seccomp(5)](https://man7.org/linux/man-pages/man5/proc_pid_seccomp.5.html), [proc_pid_setgroups(5)](https://man7.org/linux/man-pages/man5/proc_pid_setgroups.5.html), [proc_pid_smaps(5)](https://man7.org/linux/man-pages/man5/proc_pid_smaps.5.html), [proc_pid_stack(5)](https://man7.org/linux/man-pages/man5/proc_pid_stack.5.html), [proc_pid_stat(5)](https://man7.org/linux/man-pages/man5/proc_pid_stat.5.html), [proc_pid_statm(5)](https://man7.org/linux/man-pages/man5/proc_pid_statm.5.html), [proc_pid_status(5)](https://man7.org/linux/man-pages/man5/proc_pid_status.5.html), [proc_pid_syscall(5)](https://man7.org/linux/man-pages/man5/proc_pid_syscall.5.html), [proc_pid_task(5)](https://man7.org/linux/man-pages/man5/proc_pid_task.5.html), [proc_pid_timers(5)](https://man7.org/linux/man-pages/man5/proc_pid_timers.5.html), [proc_pid_timerslack_ns(5)](https://man7.org/linux/man-pages/man5/proc_pid_timerslack_ns.5.html), [proc_pid_uid_map(5)](https://man7.org/linux/man-pages/man5/proc_pid_uid_map.5.html), [proc_pid_wchan(5)](https://man7.org/linux/man-pages/man5/proc_pid_wchan.5.html), [proc_profile(5)](https://man7.org/linux/man-pages/man5/proc_profile.5.html), [proc_scsi(5)](https://man7.org/linux/man-pages/man5/proc_scsi.5.html), [proc_slabinfo(5)](https://man7.org/linux/man-pages/man5/proc_slabinfo.5.html), [proc_stat(5)](https://man7.org/linux/man-pages/man5/proc_stat.5.html), [proc_swaps(5)](https://man7.org/linux/man-pages/man5/proc_swaps.5.html), [proc_sys(5)](https://man7.org/linux/man-pages/man5/proc_sys.5.html), [proc_sys_abi(5)](https://man7.org/linux/man-pages/man5/proc_sys_abi.5.html), [proc_sys_debug(5)](https://man7.org/linux/man-pages/man5/proc_sys_debug.5.html), [proc_sys_dev(5)](https://man7.org/linux/man-pages/man5/proc_sys_dev.5.html), [proc_sys_fs(5)](https://man7.org/linux/man-pages/man5/proc_sys_fs.5.html), [proc_sys_kernel(5)](https://man7.org/linux/man-pages/man5/proc_sys_kernel.5.html), [proc_sys_net(5)](https://man7.org/linux/man-pages/man5/proc_sys_net.5.html), [proc_sys_net_ipv4(5)](https://man7.org/linux/man-pages/man5/proc_sys_net_ipv4.5.html), [proc_sys_proc(5)](https://man7.org/linux/man-pages/man5/proc_sys_proc.5.html), [proc_sysrq-trigger(5)](https://man7.org/linux/man-pages/man5/proc_sysrq-trigger.5.html), [proc_sys_sunrpc(5)](https://man7.org/linux/man-pages/man5/proc_sys_sunrpc.5.html), [proc_sys_user(5)](https://man7.org/linux/man-pages/man5/proc_sys_user.5.html), [proc_sysvipc(5)](https://man7.org/linux/man-pages/man5/proc_sysvipc.5.html), [proc_sys_vm(5)](https://man7.org/linux/man-pages/man5/proc_sys_vm.5.html), [proc_tid_children(5)](https://man7.org/linux/man-pages/man5/proc_tid_children.5.html), [proc_timer_list(5)](https://man7.org/linux/man-pages/man5/proc_timer_list.5.html), [proc_timer_stats(5)](https://man7.org/linux/man-pages/man5/proc_timer_stats.5.html), [proc_tty(5)](https://man7.org/linux/man-pages/man5/proc_tty.5.html), [proc_uptime(5)](https://man7.org/linux/man-pages/man5/proc_uptime.5.html), [proc_version(5)](https://man7.org/linux/man-pages/man5/proc_version.5.html), [proc_vmstat(5)](https://man7.org/linux/man-pages/man5/proc_vmstat.5.html), [proc_zoneinfo(5)](https://man7.org/linux/man-pages/man5/proc_zoneinfo.5.html), [sysfs(5)](https://man7.org/linux/man-pages/man5/sysfs.5.html), [systemd.mount(5)](https://man7.org/linux/man-pages/man5/systemd.mount.5.html), [capabilities(7)](https://man7.org/linux/man-pages/man7/capabilities.7.html), [cgroup_namespaces(7)](https://man7.org/linux/man-pages/man7/cgroup_namespaces.7.html), [cpuset(7)](https://man7.org/linux/man-pages/man7/cpuset.7.html), [credentials(7)](https://man7.org/linux/man-pages/man7/credentials.7.html), [epoll(7)](https://man7.org/linux/man-pages/man7/epoll.7.html), [fanotify(7)](https://man7.org/linux/man-pages/man7/fanotify.7.html), [hier(7)](https://man7.org/linux/man-pages/man7/hier.7.html), [inotify(7)](https://man7.org/linux/man-pages/man7/inotify.7.html), [libc(7)](https://man7.org/linux/man-pages/man7/libc.7.html), [mount_namespaces(7)](https://man7.org/linux/man-pages/man7/mount_namespaces.7.html), [namespaces(7)](https://man7.org/linux/man-pages/man7/namespaces.7.html), [netdevice(7)](https://man7.org/linux/man-pages/man7/netdevice.7.html), [network_namespaces(7)](https://man7.org/linux/man-pages/man7/network_namespaces.7.html), [pid_namespaces(7)](https://man7.org/linux/man-pages/man7/pid_namespaces.7.html), [pkeys(7)](https://man7.org/linux/man-pages/man7/pkeys.7.html), [pthreads(7)](https://man7.org/linux/man-pages/man7/pthreads.7.html), [pty(7)](https://man7.org/linux/man-pages/man7/pty.7.html), [signal(7)](https://man7.org/linux/man-pages/man7/signal.7.html), [symlink(7)](https://man7.org/linux/man-pages/man7/symlink.7.html), [user_namespaces(7)](https://man7.org/linux/man-pages/man7/user_namespaces.7.html), [vdso(7)](https://man7.org/linux/man-pages/man7/vdso.7.html), [migratepages(8)](https://man7.org/linux/man-pages/man8/migratepages.8.html), [netstat(8)](https://man7.org/linux/man-pages/man8/netstat.8.html), [numactl(8)](https://man7.org/linux/man-pages/man8/numactl.8.html), [sysctl(8)](https://man7.org/linux/man-pages/man8/sysctl.8.html), [systemd-coredump(8)](https://man7.org/linux/man-pages/man8/systemd-coredump.8.html)

|

HTML rendering created 2026-09-09 by [Michael Kerrisk](https://man7.org/mtk/index.html), author of [The Linux Programming Interface](https://man7.org/tlpi/).

For details of in-depth Linux/UNIX system programming training courses that I teach, look [here](https://man7.org/training/).

Hosting by [jambit GmbH](https://www.jambit.com/index_en.html).

|    |  https://man7.org/tlpi/
