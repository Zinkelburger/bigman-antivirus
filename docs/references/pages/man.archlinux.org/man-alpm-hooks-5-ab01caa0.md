---
url: "https://man.archlinux.org/man/alpm-hooks.5"
title: "alpm-hooks(5) — Arch manual pages"
captured: "2026-11-02T18:35:07Z"
via: "live"
sha256: "748308154e750f64f951b25f108f157f47581d13e7a5583c1a4c395b0394aea5"
wayback: "https://web.archive.org/web/20260913003006/https://man.archlinux.org/man/alpm-hooks.5"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# alpm-hooks(5) — Arch manual pages

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://man.archlinux.org/man/alpm-hooks.5>
> Snapshot: <https://web.archive.org/web/20260913003006/https://man.archlinux.org/man/alpm-hooks.5>

| ALPM-HOOKS(5)  | Pacman Manual  | ALPM-HOOKS(5)

# [NAME](https://man.archlinux.org/man/alpm-hooks.5#NAME)

alpm-hooks - alpm hook file format

# [SYNOPSIS](https://man.archlinux.org/man/alpm-hooks.5#SYNOPSIS)

```
[Trigger] (Required, Repeatable)
Operation = Install|Upgrade|Remove (Required, Repeatable)
Type = Path|Package (Required)
Target = <Path|PkgName> (Required, Repeatable)
[Action] (Required)
Description = ... (Optional)
When = PreTransaction|PostTransaction (Required)
Exec = <Command> (Required)
Depends = <PkgName> (Optional)
AbortOnFail (Optional, PreTransaction only)
NeedsTargets (Optional)
```

# [DESCRIPTION](https://man.archlinux.org/man/alpm-hooks.5#DESCRIPTION)

libalpm provides the ability to specify hooks to run before or after transactions based on the packages and/or files being modified. Hooks consist of a single [Action] section describing the action to be run and one or more [Trigger] section describing which transactions it should be run for.

Hooks are read from files located in the system hook directory /usr/share/libalpm/hooks, and additional custom directories specified in [pacman.conf(5)](https://man.archlinux.org/man/pacman.conf.5.en) (the default is /etc/pacman.d/hooks). The file names are required to have the suffix ".hook". Hooks are run in alphabetical order of their file name, where the ordering ignores the suffix.

# [TRIGGERS](https://man.archlinux.org/man/alpm-hooks.5#TRIGGERS)

Hooks must contain at least one [Trigger] section that determines which transactions will cause the hook to run. If multiple trigger sections are defined the hook will run if the transaction matches any of the triggers.

Operation = Install|Upgrade|Remove

Select the type of operation to match targets against. May be specified multiple times. Installations are considered an upgrade if the package or file is already present on the system regardless of whether the new package version is actually greater than the currently installed version. For Path triggers, this is true even if the file changes ownership from one package to another. Required.

Type = Path|Package

Select whether targets are matched against transaction packages or files. See CAVEATS for special notes regarding Path triggers. File is a deprecated alias for Path and will be removed in a future release. Required.

Target = <path|package>

The path or package name to match against the active transaction. Paths refer to the files in the package archive; the installation root should not be included in the path. Shell-style glob patterns are allowed. It is possible to invert matches by prepending a target with an exclamation mark. May be specified multiple times, with subsequent matches overriding previous ones. Required.

# [ACTIONS](https://man.archlinux.org/man/alpm-hooks.5#ACTIONS)

Description = ...

An optional description that describes the action being taken by the hook for use in front-end output.

Exec = <command>

Command to run. Command arguments are split on whitespace. Values containing whitespace should be enclosed in quotes. Required.

When = PreTransaction|PostTransaction

When to run the hook. Required.

Depends = <package>

Packages that must be installed for the hook to run. May be specified multiple times.

AbortOnFail

Causes the transaction to be aborted if the hook exits non-zero. Only applies to PreTransaction hooks.

NeedsTargets

Causes the list of matched trigger targets to be passed to the running hook on stdin.

# [OVERRIDING HOOKS](https://man.archlinux.org/man/alpm-hooks.5#OVERRIDING_HOOKS)

Hooks may be overridden by placing a file with the same name in a higher priority hook directory. Hooks may be disabled by overriding them with a symlink to /dev/null.

# [EXAMPLES](https://man.archlinux.org/man/alpm-hooks.5#EXAMPLES)

```
# Force disks to sync to reduce the risk of data corruption
[Trigger]
Operation = Install
Operation = Upgrade
Operation = Remove
Type = Package
Target = *
[Action]
Depends = coreutils
When = PostTransaction
Exec = /usr/bin/sync
```

# [CAVEATS](https://man.archlinux.org/man/alpm-hooks.5#CAVEATS)

There are situations when path triggers may act in unexpected ways. Hooks are triggered using the file list of the installed, upgraded, or removed package. When installing or upgrading a file that is extracted with a .pacnew extension, the original file name is used in triggering the hook. When removing a package, all files owned by that package can trigger a hook whether or not they were actually present on the file system before package removal.

PostTransaction hooks will not run if the transaction fails to complete for any reason.

See the pacman website at https://archlinux.org/pacman/ for current information on pacman and its related tools.

# [BUGS](https://man.archlinux.org/man/alpm-hooks.5#BUGS)

Bugs? You must be kidding; there are no bugs in this software. But if we happen to be wrong, please report them to the issue tracker at https://gitlab.archlinux.org/pacman/pacman/-/issues with specific information such as your command-line, the nature of the bug, and even the package database if it helps.

# [AUTHORS](https://man.archlinux.org/man/alpm-hooks.5#AUTHORS)

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
