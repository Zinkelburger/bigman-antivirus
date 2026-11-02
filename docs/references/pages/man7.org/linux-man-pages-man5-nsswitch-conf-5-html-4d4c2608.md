---
url: "https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html"
title: "nsswitch.conf(5) - Linux manual page"
captured: "2026-11-02T18:35:54Z"
via: "live"
sha256: "3c9074885a2bc05454c4f63bbd323e23a555c01e52c598d5a887bcd68b863354"
wayback: "https://web.archive.org/web/20261015061949/https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# nsswitch.conf(5) - Linux manual page

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html>
> Snapshot: <https://web.archive.org/web/20261015061949/https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html>

|

[man7.org](https://man7.org/index.html) > Linux > [man-pages](https://man7.org/linux/man-pages/index.html)

|

[Linux/UNIX system programming training](http://man7.org/training/)

# nsswitch.conf(5) — Linux manual page

|

[NAME](https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#NAME) | [DESCRIPTION](https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#DESCRIPTION) | [FILES](https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#FILES) | [NOTES](https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#NOTES) | [SEE ALSO](https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#SEE_ALSO) | [COLOPHON](https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#COLOPHON)

|

|

```

nsswitch.conf(5)           File Formats Manual           nsswitch.conf(5)

```

## https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#NAMENAME [top](https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#top_of_page)

```

       nsswitch.conf - Name Service Switch configuration file

```

## https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#DESCRIPTIONDESCRIPTION [top](https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#top_of_page)

```

       The Name Service Switch (NSS) configuration file,
       /etc/nsswitch.conf, is used by the GNU C Library and certain other
       applications to determine the sources from which to obtain name-
       service information in a range of categories, and in what order.
       Each category of information is identified by a database name.

       The file is plain ASCII text, with columns separated by spaces or
       tab characters.  The first column specifies the database name.
       The remaining columns describe the order of sources to query and a
       limited set of actions that can be performed by lookup result.

       The following databases are understood by the GNU C Library:

       aliases
              Mail aliases, used by [getaliasent(3)](https://man7.org/linux/man-pages/man3/getaliasent.3.html) and related functions.

       ethers Ethernet numbers.

       group  Groups of users, used by [getgrent(3)](https://man7.org/linux/man-pages/man3/getgrent.3.html) and related functions.

       hosts  Host names and numbers, used by [gethostbyname(3)](https://man7.org/linux/man-pages/man3/gethostbyname.3.html) and
              related functions.

       initgroups
              Supplementary group access list, used by [getgrouplist(3)](https://man7.org/linux/man-pages/man3/getgrouplist.3.html)
              function.

       netgroup
              Network-wide list of hosts and users, used for access
              rules.  C libraries before glibc 2.1 supported netgroups
              only over NIS.

       networks
              Network names and numbers, used by [getnetent(3)](https://man7.org/linux/man-pages/man3/getnetent.3.html) and related
              functions.

       passwd User passwords, used by [getpwent(3)](https://man7.org/linux/man-pages/man3/getpwent.3.html) and related functions.

       protocols
              Network protocols, used by [getprotoent(3)](https://man7.org/linux/man-pages/man3/getprotoent.3.html) and related
              functions.

       publickey
              Public and secret keys for Secure_RPC used by NFS and NIS+.

       rpc    Remote procedure call names and numbers, used by
              [getrpcbyname(3)](https://man7.org/linux/man-pages/man3/getrpcbyname.3.html) and related functions.

       services
              Network services, used by [getservent(3)](https://man7.org/linux/man-pages/man3/getservent.3.html) and related
              functions.

       shadow Shadow user passwords, used by [getspnam(3)](https://man7.org/linux/man-pages/man3/getspnam.3.html) and related
              functions.

       The GNU C Library ignores databases with unknown names.  Some
       applications use this to implement special handling for their own
       databases.  For example, [sudo(8)](https://man7.org/linux/man-pages/man8/sudo.8.html) consults the sudoers database.
       Delegation of subordinate user/group IDs can be configured using
       the subid database.  Refer to [subuid(5)](https://man7.org/linux/man-pages/man5/subuid.5.html) and [subgid(5)](https://man7.org/linux/man-pages/man5/subgid.5.html) for more
       details.

       Here is an example /etc/nsswitch.conf file:

           passwd:         compat
           group:          compat
           shadow:         compat

           hosts:          dns [!UNAVAIL=return] files
           networks:       nis [NOTFOUND=return] files
           ethers:         nis [NOTFOUND=return] files
           protocols:      nis [NOTFOUND=return] files
           rpc:            nis [NOTFOUND=return] files
           services:       nis [NOTFOUND=return] files

       The first column is the database name.  The remaining columns
       specify:

       •  One or more service specifications, for example, "files", "db",
          or "nis".  The order of the services on the line determines the
          order in which those services will be queried, in turn, until a
          result is found.

       •  Optional actions to perform if a particular result is obtained
          from the preceding service, for example, "[NOTFOUND=return]".

       The service specifications supported on your system depend on the
       presence of shared libraries, and are therefore extensible.
       Libraries called /lib/libnss_SERVICE.so.X will provide the named
       SERVICE.  On a standard installation, you can use "files", "db",
       "nis", and "nisplus".  For the hosts database, you can
       additionally specify "dns".  For the passwd, group, and shadow
       databases, you can additionally specify "compat" (see
       Compatibility mode below).  The version number X may be 1 for
       glibc 2.0, or 2 for glibc 2.1 and later.  On systems with
       additional libraries installed, you may have access to further
       services such as "hesiod", "ldap", "winbind", and "wins".

       An action may also be specified following a service specification.
       The action modifies the behavior following a result obtained from
       the preceding data source.  Action items take the general form:

           [STATUS=ACTION]
           [!STATUS=ACTION]

       where

           STATUS => success | notfound | unavail | tryagain
           ACTION => return | continue | merge

       The ! negates the test, matching all possible results except the
       one specified.  The case of the keywords is not significant.

       The STATUS value is matched against the result of the lookup
       function called by the preceding service specification, and can be
       one of:

           success
                  No error occurred and the requested entry is returned.
                  The default action for this condition is "return".

           notfound
                  The lookup succeeded, but the requested entry was not
                  found.  The default action for this condition is
                  "continue".

           unavail
                  The service is permanently unavailable.  This can mean
                  either that the required file cannot be read, or, for
                  network services, that the server is not available or
                  does not allow queries.  The default action for this
                  condition is "continue".

           tryagain
                  The service is temporarily unavailable.  This could
                  mean a file is locked or a server currently cannot
                  accept more connections.  The default action for this
                  condition is "continue".

       The ACTION value can be one of:

           return Return a result now.  Do not call any further lookup
                  functions.  However, for compatibility reasons, if this
                  is the selected action for the group database and the
                  notfound status, and the configuration file does not
                  contain the initgroups line, the next lookup function
                  is always called, without affecting the search result.

           continue
                  Call the next lookup function.

           merge  [SUCCESS=merge] is used between two database entries.
                  When a group is located in the first of the two group
                  entries, processing will continue on to the next one.
                  If the group is also found in the next entry (and the
                  group name and GID are an exact match), the member list
                  of the second entry will be added to the group object
                  to be returned.  Available since glibc 2.24.  Note that
                  merging will not be done for [getgrent(3)](https://man7.org/linux/man-pages/man3/getgrent.3.html) nor will
                  duplicate members be pruned when they occur in both
                  entries being merged.

   Compatibility mode (compat)
       The NSS "compat" service is similar to "files" except that it
       additionally permits special entries in corresponding files for
       granting users or members of netgroups access to the system.  The
       following entries are valid in this mode:

           For passwd and shadow databases:

               +user  Include the specified user from the NIS
                      passwd/shadow map.

               +@netgroup
                      Include all users in the given netgroup.

               -user  Exclude the specified user from the NIS
                      passwd/shadow map.

               -@netgroup
                      Exclude all users in the given netgroup.

               +      Include every user, except previously excluded
                      ones, from the NIS passwd/shadow map.

           For group database:

               +group Include the specified group from the NIS group map.

               -group Exclude the specified group from the NIS group map.

               +      Include every group, except previously excluded
                      ones, from the NIS group map.

       By default, the source is "nis", but this may be overridden by
       specifying any NSS service except "compat" itself as the source
       for the pseudo-databases passwd_compat, group_compat, and
       shadow_compat.

```

## https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#FILESFILES [top](https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#top_of_page)

```

       A service named SERVICE is implemented by a shared object library
       named libnss_SERVICE.so.X that resides in /lib.

           /etc/nsswitch.conf
                  NSS configuration file.
           /lib/libnss_compat.so.X
                  implements "compat" source.
           /lib/libnss_db.so.X
                  implements "db" source.
           /lib/libnss_dns.so.X
                  implements "dns" source.
           /lib/libnss_files.so.X
                  implements "files" source.
           /lib/libnss_hesiod.so.X
                  implements "hesiod" source.
           /lib/libnss_nis.so.X
                  implements "nis" source.
           /lib/libnss_nisplus.so.X
                  implements "nisplus" source.

       The following files are read when "files" source is specified for
       respective databases:

           aliases
                  /etc/aliases
           ethers /etc/ethers
           group  /etc/group
           hosts  /etc/hosts
           initgroups
                  /etc/group
           netgroup
                  /etc/netgroup
           networks
                  /etc/networks
           passwd /etc/passwd
           protocols
                  /etc/protocols
           publickey
                  /etc/publickey
           rpc    /etc/rpc
           services
                  /etc/services
           shadow /etc/shadow

```

## https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#NOTESNOTES [top](https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#top_of_page)

```

       Starting with glibc 2.33, nsswitch.conf is automatically reloaded
       if the file is changed.  In earlier versions, the entire file was
       read only once within each process.  If the file was later
       changed, the process would continue using the old configuration.

       Traditionally, there was only a single source for service
       information, often in the form of a single configuration file
       (e.g., /etc/passwd).  However, as other name services, such as the
       Network Information Service (NIS) and the Domain Name Service
       (DNS), became popular, a method was needed that would be more
       flexible than fixed search orders coded into the C library.  The
       Name Service Switch mechanism, which was based on the mechanism
       used by Sun Microsystems in the Solaris 2 C library, introduced a
       cleaner solution to the problem.

```

## https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#SEE_ALSOSEE ALSO [top](https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#top_of_page)

```

       [getent(1)](https://man7.org/linux/man-pages/man1/getent.1.html), [nss(5)](https://man7.org/linux/man-pages/man5/nss.5.html)

```

## https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#COLOPHONCOLOPHON [top](https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html#top_of_page)

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

Linux man-pages 6.19            2026-02-08               nsswitch.conf(5)

```

Pages that refer to this page: [getent(1)](https://man7.org/linux/man-pages/man1/getent.1.html), [gethostbyname(3)](https://man7.org/linux/man-pages/man3/gethostbyname.3.html), [host.conf(5)](https://man7.org/linux/man-pages/man5/host.conf.5.html), [nss(5)](https://man7.org/linux/man-pages/man5/nss.5.html), [resolv.conf(5)](https://man7.org/linux/man-pages/man5/resolv.conf.5.html), [hostname(7)](https://man7.org/linux/man-pages/man7/hostname.7.html), [pcap-filter(7)](https://man7.org/linux/man-pages/man7/pcap-filter.7.html), [nscd(8)](https://man7.org/linux/man-pages/man8/nscd.8.html), [nss-myhostname(8)](https://man7.org/linux/man-pages/man8/nss-myhostname.8.html), [nss-mymachines(8)](https://man7.org/linux/man-pages/man8/nss-mymachines.8.html), [nss-resolve(8)](https://man7.org/linux/man-pages/man8/nss-resolve.8.html), [nss-systemd(8)](https://man7.org/linux/man-pages/man8/nss-systemd.8.html)

|

HTML rendering created 2026-09-09 by [Michael Kerrisk](https://man7.org/mtk/index.html), author of [The Linux Programming Interface](https://man7.org/tlpi/).

For details of in-depth Linux/UNIX system programming training courses that I teach, look [here](https://man7.org/training/).

Hosting by [jambit GmbH](https://www.jambit.com/index_en.html).

|    |  https://man7.org/tlpi/
