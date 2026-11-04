---
url: "https://github.com/andrewgregory/pacutils"
title: "GitHub - andrewgregory/pacutils: Helper library for libalpm based programs."
site: "GitHub"
captured: "2026-11-02T18:45:19Z"
via: "live"
sha256: "def7a57acd0096ec7e0d62a855b4d49f5a4143291d3b63e679c31d607c84753c"
wayback: "https://web.archive.org/web/20260906124213/https://github.com/andrewgregory/pacutils"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# GitHub - andrewgregory/pacutils: Helper library for libalpm based programs.

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/andrewgregory/pacutils>
> Snapshot: <https://web.archive.org/web/20260906124213/https://github.com/andrewgregory/pacutils>

# NAME

https://github.com/andrewgregory/pacutils#name

pacutils - utility library for libalpm front-ends

# SYNOPSIS

https://github.com/andrewgregory/pacutils#synopsis

```
pu_config_t *config = pu_config_new_from_file("/etc/pacman.conf");
alpm_handle_t *handle = pu_initialize_handle_from_config(config);
alpm_list_t *syncdbs = pu_register_syncdbs(config->repos);

alpm_option_set_progresscb(handle, pu_cb_progress, NULL);
alpm_option_set_dlcb(handle, pu_cb_download, NULL);

/* do something with handle... */

alpm_list_free(sync_dbs);
alpm_release(handle);
pu_config_free(config);
```

pacutils encapsulates several of the common tasks that most users of libalpm need to perform, such as parsing pacman.conf and providing standard output functions for callbacks. The included programs can also be used as guides for how to accomplish particular tasks using libalpm.

# DESCRIPTION

https://github.com/andrewgregory/pacutils#description

## Configuration Parsing

https://github.com/andrewgregory/pacutils#configuration-parsing

pu_config_t *pu_config_new_from_file(const char *filename);

Parse a pacman configuration file into a pu_config_t structure. Note: this is the final configuration as parsed by pacman. Include directives will be processed, defaults are set, $repo and $arch will be replaced in Server directives and Architecture will be replaced if set to auto.

void pu_config_free(pu_config_t *config);

alpm_handle_t *pu_initialize_handle_from_config(pu_config_t *config);

## Repositories

https://github.com/andrewgregory/pacutils#repositories

alpm_db_t *pu_register_syncdb(pu_repo_t *repo);

alpm_list_t *pu_register_syncdbs(alpm_list_t *repos);

void pu_repo_free(pu_repo_t *repo);

## Callbacks

https://github.com/andrewgregory/pacutils#callbacks

const char *pu_msg_progress(alpm_progress_t event);

void pu_cb_progress(alpm_progress_t event, const char *pkgname, int percent, size_t total, size_t current);

void pu_cb_download(const char *filename, off_t xfered, off_t total);

## MTREE

https://github.com/andrewgregory/pacutils#mtree

Package mtree reading functions are provided to fill gaps in libarchive's mtree reader, notably md5digest and sha256digest which are currently not read by libarchive. Support is incomplete and libalpm's mtree reader should be used whenever possible.

pu_mtree_t

```
typedef struct pu_mtree_t {
  char *path;
  char type[16];
  uid_t uid;
  gid_t gid;
  mode_t mode;
  off_t size;
  char md5digest[33];
  char sha256digest[65];
} pu_mtree_t;
```

alpm_list_t *pu_mtree_load_pkg_mtree(alpm_handle_t *handle, alpm_pkg_t *pkg);

Returns a list of mtree entries for pkg.

void pu_mtree_free(pu_mtree_t *mtree);

## Pkgspec

https://github.com/andrewgregory/pacutils#pkgspec

A pkgspec is a string consisting of sufficient information to uniquely identify an individual package. Currently recognized formats are:

<dbname>/<pkgname>

```
local/pacman
core/pacman
```

<protocol>://<path>

```
ftp://ftp.archlinux.org/core/os/x86_64/pacman-4.1.2-5-x86_64.pkg.tar.xz
file:///var/cache/pacman/pkg/pacman-4.1.2-5-x86_64.pkg.tar.xz
```

# PROGRAMS

https://github.com/andrewgregory/pacutils#programs

There are several programs located in src/. Refer to each program's documentation for more information.

paccapability

Query compile time libalpm features.

```
paccapability signatures >/dev/null && echo "libalpm built with signature support"
```

pacconf

Query pacman's configuration file. Useful for shell scripts:

```
dbpath=$(pacconf dbpath)
color=$(pacconf color)
[[ -n $color ]] && ... # print something in color
```

Note: this is the final configuration as parsed by pacman. Include directives will be processed, defaults are set, $repo and $arch will be replaced in Server directives and Architecture will be replaced if set to auto.

pacini

Query a pacman-style INI configuration file.

pacsync

Updates repository databases. Equivalent to pacman -Sy.

pactrans, pacinstall, pacremove

Install/remove packages in a single transaction.

NOTE: alpm support for installation and removal in a single transaction is flaky; some options may be ignored.

pacinfo

Display information for packages, similar to pacman -Qi and pacman -Si. Input should be a list of pkgspecs.

pacsift

Package search/filter tool. Combine with pacinfo to display brief package information similar to pacman -Qs and pacman -Ss:

```
pacsift --name pacman | pacinfo --short
```

pacfile

Display information about a file. Includes information from pacman -Qo, pacman -Qkk, and pacman -Qii. Expects input to match the path stored in pacman's database with or without the root prepended. realpath can be used to call with relative paths or paths containing symbolic links:

```
pacfile "$(realpath ./foo)"
```

pacrepairfile

Repair file properties based on pacman's database.

```
find /usr/share | pacrepairfile --uid --gid
```

paclock

Provide script-friendly database locking and unlocking.

```
paclock --run -- paccache --dryrun --verbose
```

paclog

Filter pacman log entries. If multiple filters are provided, the entries returned are the union of all filters. Output can be piped to a second instance to find the intersection of filters instead:

```
paclog --caller ALPM | paclog --after "$(date -I --date '3 days ago')"
```

pacrepairdb

Attempt to repair broken entries in libalpm's database by reinstalling affected packages from the cache:

```
paccheck --files --db-files --list-broken | pacrepairdb
```

pacreport

Generate a report of installed packages including:

unneeded packages installed explicitly

unneeded packages installed as dependencies

installed packages not in a repository

packages missing from specified groups

missing package files

cache directory sizes

packages sizes include dependencies not needed by other packages

paccheck

Verify installed packages against pacman's database.

# BUGS

https://github.com/andrewgregory/pacutils#bugs

Not implemented yet:

error reporting

proper handling of transaction flags with combined transactions

# COPYRIGHT AND LICENSE

https://github.com/andrewgregory/pacutils#copyright-and-license

Copyright (c) 2012-2016 Andrew Gregory

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
