---
url: "https://manpages.debian.org/debsums"
title: "debsums(1) — debsums — Debian trixie — Debian Manpages"
captured: "2026-11-02T18:38:25Z"
via: "live"
source_url: "https://manpages.debian.org/trixie/debsums/debsums.1.en.html"
sha256: "868aada3ab1d93aca16a4c0d2ee02b884ccff9a492e54bc63f87a75702dd718d"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# debsums(1) — debsums — Debian trixie — Debian Manpages

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://manpages.debian.org/debsums>

https://www.debian.org/

[MANPAGES](https://manpages.debian.org/)

[Skip Quicknav](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#content)

- [Index](https://manpages.debian.org/)
- [About Manpages](https://manpages.debian.org/about.html)
- [FAQ](https://manpages.debian.org/faq.html)
- [Service Information](https://wiki.debian.org/manpages.debian.org)

/ [trixie](https://manpages.debian.org/contents-trixie.html) / [debsums](https://manpages.debian.org/trixie/debsums/index.html) / debsums(1)

links

-  [language-indep link](https://manpages.debian.org/trixie/debsums/debsums.1)
-  [package tracker](https://tracker.debian.org/pkg/debsums)
-  [raw man page](https://manpages.debian.org/trixie/debsums/debsums.1.en.gz)

table of contents

-  [NAME](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#NAME)
-  [SYNOPSIS](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#SYNOPSIS)
-  [DESCRIPTION](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#DESCRIPTION)
-  [OPTIONS](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#OPTIONS)
-  [EXIT STATUS](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#EXIT_STATUS)
-  [EXAMPLES](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#EXAMPLES)
-  [RESULTS](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#RESULTS)
-  [SEE ALSO](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#SEE_ALSO)
-  [ENVIRONMENT](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#ENVIRONMENT)
-  [CAVEATS](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#CAVEATS)
-  [AUTHOR](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#AUTHOR)
-  [COPYRIGHT](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#COPYRIGHT)

other versions

-  [trixie](https://manpages.debian.org/trixie/debsums/debsums.1.en.html) 3.0.2.3
-  [testing](https://manpages.debian.org/testing/debsums/debsums.1.en.html) 3.0.2.4
-  [unstable](https://manpages.debian.org/unstable/debsums/debsums.1.en.html) 3.0.2.4

other languages

-  [Deutsch](https://manpages.debian.org/trixie/debsums/debsums.1.de.html)
-  [English](https://manpages.debian.org/trixie/debsums/debsums.1.en.html)
-  [español](https://manpages.debian.org/trixie/debsums/debsums.1.es.html)
-  [français](https://manpages.debian.org/trixie/debsums/debsums.1.fr.html)
-  [português (pt)](https://manpages.debian.org/trixie/debsums/debsums.1.pt.html)
-  [português (pt-BR)](https://manpages.debian.org/trixie/debsums/debsums.1.pt_BR.html)
-  [română](https://manpages.debian.org/trixie/debsums/debsums.1.ro.html)
-  [русский](https://manpages.debian.org/trixie/debsums/debsums.1.ru.html)
-  [svenska](https://manpages.debian.org/trixie/debsums/debsums.1.sv.html)

[Scroll to navigation](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#panels)

| DEBSUMS(1)  | General Commands Manual  | DEBSUMS(1)

# NAME[¶](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#NAME)

debsums - check the MD5 sums of installed Debian packages

# SYNOPSIS[¶](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#SYNOPSIS)

debsums [options] [package|deb] ...

# DESCRIPTION[¶](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#DESCRIPTION)

Verify installed Debian package files against MD5 checksum lists from /var/lib/dpkg/info/*.md5sums.

debsums can generate checksum lists from deb archives for packages that don't include one.

# OPTIONS[¶](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#OPTIONS)

[-a, --all](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#a)

Also check configuration files (normally excluded).

[-e, --config](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#e)

Only check configuration files.

[-c, --changed](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#c)

Report changed file list to stdout (implies -s).

[-l, --list-missing](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#l)

List packages (or debs) which don't have an MD5 sums file.

[-s, --silent](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#s)

Only report errors.

[-m, --md5sums=file](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#m)

Read list of deb checksums from file.

[-r, --root=dir](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#r)

Root directory to check (default /).

[-d, --admindir=dir](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#d)

dpkg admin directory (default /var/lib/dpkg).

[-p, --deb-path=dir[:dir...]](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#p)

Directories in which to look for debs derived from the package name (default is the current directory).

A useful value is /var/cache/apt/archives when using apt-get autoclean or not clearing the cache at all. The command:

apt-get --reinstall -d install `debsums -l`

may be used to populate the cache with any debs not already in the cache.

Note: This doesn't work for CD-ROM and other local sources as packages are not copied to /var/cache. Simple file sources (all debs in a single directory) should be added to the -p list.

[-g, --generate=[missing|all][,nocheck]](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#g)

Generate MD5 sums from deb contents. If the argument is a package name rather than a deb archive, the program will look for a deb named package_version_arch.deb in the directories given by the -p option.

[missing](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#missing)

Generate MD5 sums from the deb for packages which don't provide one.

[all](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#all)

Ignore the on disk sums and use the one supplied in the deb, or generated from it if none exists.

[nocheck](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#nocheck)

The extracted/generated sums are not checked against the installed package.

For backward compatibility, the short option -g is equivalent to --generate=missing.

[--no-locale-purge](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#no)

Report missing locale files even if localepurge is configured.

[--no-prelink](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#no~2)

Report changed ELF files even if prelink is configured.

[--ignore-permissions](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#ignore)

Treat permission errors as warnings when running as non-root.

[--ignore-obsolete](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#ignore~2)

Ignore obsolete conffiles.

[--help](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#help)

[--version](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#version)

Print help and version information.

# EXIT STATUS[¶](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#EXIT_STATUS)

debsums returns 0 on success, or a combination* of the following values on error:

1

A specified package or archive name was not installed, invalid or the installed version did not match the given archive.

2

Changed or missing package files, or checksum mismatch on an archive.

255

Invalid option.

*If both of the first two conditions are true, the exit status will be 3.

# EXAMPLES[¶](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#EXAMPLES)

[debsums foo bar](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#debsums)

Check the sums for installed packages foo and bar.

[debsums foo.deb bar.deb](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#debsums~2)

As above, using checksums from (or generated from) the archives.

[debsums -l](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#debsums~3)

List installed packages with no checksums.

[debsums -ca](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#debsums~4)

List changed package files from all installed packages with checksums.

[debsums -ce](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#debsums~5)

List changed configuration files.

[debsums -cagp /var/cache/apt/archives](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#debsums~6)

As above, using sums from cached debs where available.

[apt-get install --reinstall $(dpkg -S $(debsums -c) | cut -d : -f 1 | sort -u)](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#apt_get)

Reinstalls packages with changed files.

# RESULTS[¶](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#RESULTS)

[OK](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#OK)

The file's MD5 sum is good.

[FAILED](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#FAILED)

The file's MD5 sum does not match.

[REPLACED](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#REPLACED)

The file has been replaced by a file from a different package.

# SEE ALSO[¶](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#SEE_ALSO)

[md5sum(1)](https://manpages.debian.org/trixie/coreutils/md5sum.1.en.html),

# ENVIRONMENT[¶](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#ENVIRONMENT)

[TMPDIR](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#TMPDIR)

Directory for extracting information and contents from package archives (/tmp by default).

# CAVEATS[¶](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#CAVEATS)

While in general the program may be run as a normal user, some packages contain files which are not globally readable so cannot be checked. Privileges are of course also required when generating sums with the keep option set.

Files which have been replaced by another package may be erroneously reported as changed.

debsums is intended primarily as a way of determining what installed files have been locally modified by the administrator or damaged by media errors and is of limited use as a security tool.

If you are looking for an integrity checker that can run from safe media, do integrity checks on checksum databases and can be easily configured to run periodically to warn the admin of changes see other tools such as: aide, integrit, samhain, or tripwire.

# AUTHOR[¶](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#AUTHOR)

Written by Brendan O'Dea <bod@debian.org>.
Based on a program by Christoph Lameter <clameter@debian.org> and Petr Cech <cech@debian.org>.

# COPYRIGHT[¶](https://manpages.debian.org/trixie/debsums/debsums.1.en.html#COPYRIGHT)

Copyright © 2002 Brendan O'Dea <bod@debian.org>
This is free software, licensed under the terms of the GNU General Public License. There is NO warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

| Debian  | User Commands

|  Source file:   |  debsums.1.en.gz (from [debsums 3.0.2.3](http://snapshot.debian.org/package/debsums/3.0.2.3/))

|  Source last updated:   |  2025-05-10T19:35:24Z

|  Converted to HTML:   |  2026-10-13T21:00:57Z

debiman HEAD, see [github.com/Debian/debiman](https://github.com/Debian/debiman/). Found a problem? See the [FAQ](https://manpages.debian.org/faq.html).
