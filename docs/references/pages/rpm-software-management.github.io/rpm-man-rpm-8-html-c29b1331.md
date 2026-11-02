---
url: "https://rpm-software-management.github.io/rpm/man/rpm.8.html"
title: "rpm(8)"
captured: "2026-11-02T18:41:43Z"
via: "live"
sha256: "fa1f2c1438514b38e4d9fc5e297eca02f957fe1d5418d8addf49cc6e3109de06"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# rpm(8)

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://rpm-software-management.github.io/rpm/man/rpm.8.html>

https://rpm.org/

- [Home](https://rpm.org/)
- [Releases](https://rpm.org/releases/)
- [Documentation](https://rpm.org/docs/)
- [Community](https://rpm.org/community.html)
- [Contribute](https://rpm.org/contribute.html)
- [Roadmap](https://rpm.org/roadmap.html)
- [About](https://rpm.org/about.html)

# RPM(8)

2026-10-19

## NAME

rpm - RPM Package Manager

## SYNOPSIS

### Querying and verifying packages

rpm {-q|--query} [select-options] [query-options] [PACKAGE_FILE ...] [PACKAGE_NAME ...]

rpm {-V|--verify} [select-options] [verify-options] [PACKAGE_NAME ...]

### Installing, upgrading and removing packages

rpm {-i|--install} [transaction-options] [install-options] PACKAGE_FILE ...

rpm {-U|--upgrade} [transaction-options] [install-options] PACKAGE_FILE ...

rpm {-F|--freshen} [transaction-options] [install-options] PACKAGE_FILE ...

rpm --reinstall [transaction-options] [install-options] PACKAGE_FILE ...

rpm --restore [transaction-options] [select-options] PACKAGE_NAME ...

rpm {-e|--erase} [transaction-options] [erase-options] PACKAGE_NAME ...

### Misc operations

rpm --querytags

See [rpm-common(8)](https://rpm-software-management.github.io/rpm/man/rpm-common.8) for operations common to all rpm executables.

## DESCRIPTION

rpm is a powerful Package Manager, which can be used to build, install, query, verify, update, and erase individual software packages. A package consists of an archive of files and meta-data used to install and erase the archive files. The meta-data includes helper scripts, file attributes, and descriptive information about the package. Packages come in two varieties: binary packages, used to encapsulate software to be installed, and source packages, containing the source code and recipe necessary to produce binary packages.

## OPERATIONS

### Querying and verifying packages

-V, --verify

Verify package(s), comparing information about the installed files in the package with information about the files taken from the package metadata stored in the rpm database. Among other things, verifying compares the size, digest, permissions, type, owner and group of each file. By default, only files with discrepancies are displayed. Use --verbose to display all verified files.

-q, --query

Query package files or installed package(s).

### Installing, upgrading and removing packages

-i, --install

Install new package(s) without considering upgrade scenarios. This is considered special usage, normally installations should be done with --upgrade.

-U, --upgrade

Install or upgrade package(s) to a newer version. Old and/or obsoleted package(s) are removed after the new package is installed.

-F, --freshen

Upgrade package(s), but only ones for which an earlier version is installed.

--reinstall

Reinstall previously installed package(s).

--restore

Restore file metadata such as timestamp, owner, group, permissions and capabilities of files of installed package(s).

-e, --erase

Erase installed packages.

### Misc operations

--querytags

Dump known querytags. Useful with the --queryformat option.

### Obsolete compatibility aliases

These are obsolete popt(3) aliases for backwards compatibility only, and their use is discouraged.

--initdb, --rebuilddb, --verifydb

See [rpmdb(8)](https://rpm-software-management.github.io/rpm/man/rpmdb.8).

--addsign, --delsign, --resign

See [rpmsign(1)](https://rpm-software-management.github.io/rpm/man/rpmsign.1).

-K, --checksig, --import

See [rpmkeys(8)](https://rpm-software-management.github.io/rpm/man/rpmkeys.8).

--specfile

See [rpmspec(1)](https://rpm-software-management.github.io/rpm/man/rpmspec.1).

See [rpm-common(8)](https://rpm-software-management.github.io/rpm/man/rpm-common.8) for the operations common to all rpm executables.

## ARGUMENTS

PACKAGE_FILE

Either an rpm package or an [rpm-manifest(5)](https://rpm-software-management.github.io/rpm/man/rpm-manifest.5) file. May also be specified as an ftp or http URL, in which case the package will be downloaded before being installed. See FTP/HTTP OPTIONS for information on rpm's ftp and http client support.

PACKAGE_NAME

Installed package named PACKAGE_NAME. To specify the package more precisely the package name may be followed by the version or version and release both separated by a dash or an architecture name separated by a dot. See the output of rpm -qa or rpm -qp PACKAGE_FILE as an example.

## OPTIONS

See [rpm-common(8)](https://rpm-software-management.github.io/rpm/man/rpm-common.8) for the options common to all operations.

## INSTALL AND UPGRADE OPTIONS

--allfiles

Installs or upgrades all the missingok files in the package, regardless if they exist.

--badreloc

Used with --relocate, permit relocations on all file paths, not just those OLDPATH's included in the binary package relocation hint(s).

--excludepath OLDPATH

Don't install files whose name begins with OLDPATH.

--excludeartifacts

Don't install any files which are marked as artifacts, such as build-id links.

--excludeconfigs, --noconfigs

Do not install configuration files.

--excludedocs

Don't install any files which are marked as documentation (which includes man pages and texinfo documents).

--force

Same as using --replacepkgs, --replacefiles, and --oldpackage.

--ignoresize

Don't check mount filesystems for sufficient disk space before installing this package.

--ignorearch

Allow installation or upgrading even if the architectures of the binary package and host don't match.

--ignoreos

Allow installation or upgrading even if the operating systems of the binary package and host don't match.

--includedocs

Install documentation files. This is the default behavior.

--nocontexts

Don't set SELinux contexts for files and scriptlets. Only effective if [rpm-plugin-selinux(8)](https://rpm-software-management.github.io/rpm/man/rpm-plugin-selinux.8) is installed and active.

--nocaps

Don't set file capabilities.

--noverify

Don't perform verify package files prior to installation.

--nosysusers

Don't create [rpm-sysusers(7)](https://rpm-software-management.github.io/rpm/man/rpm-sysusers.7) from packages

--oldpackage

Allow an upgrade to replace a newer package with an older one.

--prefix NEWPATH

For relocatable binary packages, translate all file paths that start with the installation prefix in the package relocation hint(s) to NEWPATH.

--relocate OLDPATH=NEWPATH

For relocatable binary packages, translate all file paths that start with OLDPATH in the package relocation hint(s) to NEWPATH. This option can be used repeatedly if several OLDPATH's in the package are to be relocated.

--replacefiles

Install the packages even if they replace files from other, already installed, packages.

--replacepkgs

Install the packages even if some of them are already installed on this system.

## ERASE OPTIONS

--allmatches

Remove all versions of the package which match PACKAGE_NAME. Normally an error is issued if PACKAGE_NAME matches multiple packages.

## TRANSACTION OPTIONS

The transaction options are common to --install, --upgrade, --reinstall and --erase operations.

--deploops

Print dependency loops as warnings.

-h, --hash

Print 50 hash marks as the package archive is unpacked. Use with -v|--verbose for a nicer display.

--justdb

Update only the database, not the filesystem.

--nodb

Update only the filesystem, not the database.

--nodeps

Don't check dependencies before uninstalling the packages.

--noorder

Don't reorder the packages for an install. The list of packages would normally be reordered to satisfy dependencies.

--noplugins

Do not load and execute plugins.

--noscripts

Disables execution of package scriptlets. Equivalent to

--nopre --nopost --nopreun --nopostun --nopretrans --noposttrans --nopreuntrans --nopostuntrans

--nopre --nopost --nopreun --nopostun --nopretrans --noposttrans --nopreuntrans --nopostuntrans

Don't execute the scriptlets of the named type.

--notriggers

Disable execution of package triggers. Equivalent to

--notriggerprein --notriggerin --notriggerun --notriggerpostun

--notriggerin --notriggerun --notriggerprein --notriggerpostun

Disable execution of the trigger scriptlets of the named type.

--percent

Print percentages as files are unpacked from the package archive. This is intended to make rpm easy to run from other tools.

--test

Don't really execute anything, just go through the motions. Useful in conjunction with the -vv option for debugging.

## QUERY OPTIONS

--qf QUERYFMT, --queryformat QUERYFMT

Output format of each queried package, as described by [rpm-queryformat(7)](https://rpm-software-management.github.io/rpm/man/rpm-queryformat.7). If omitted, uses the format defined by %_query_all_fmt in [rpm-config(5)](https://rpm-software-management.github.io/rpm/man/rpm-config.5).

There are three subsets of options for querying: package selection, file selection and information selection.

## PACKAGE SELECTION OPTIONS

-a, --all [SELECTOR ...]

Query all installed packages.

Optional SELECTOR's in the form of tag=pattern can be provided to narrow the selection, for example name="b*" to query packages whose name starts with "b".

--dupes

List duplicated packages.

-f, --file FILE

Query package owning installed FILE.

--filecaps

List filenames with POSIX1.e capabilities.

--fileclass

List filenames with their classes (libmagic classification).

--filecolor

List filenames with their colors (0 for noarch, 1 for 32bit, 2 for 64 bit).

--fileprovide

List filenames with their provides.

--filerequire

List filenames with their requires.

-g, --group GROUP

Query packages with the group of GROUP.

--noglob

Do not glob arguments when installing package files.

--nomanifest

Don't process non-package files as [rpm-manifest(5)](https://rpm-software-management.github.io/rpm/man/rpm-manifest.5) files.

-p, --package PACKAGE_FILE

Query an (uninstalled) package PACKAGE_FILE.

--path PATH

Query package(s) owning PATH, whether the file is installed or not. Multiple packages may own a PATH, but the file is only owned by the package installed last.

--querybynumber HDRNUM

Query the HDRNUM database entry directly; this is useful only for debugging.

--specfile SPECFILE

Obsolete, use [rpmspec(1)](https://rpm-software-management.github.io/rpm/man/rpmspec.1) instead.

--tid TID

Query package(s) that have a given TID transaction identifier. A UNIX timestamp is currently used as a transaction identifier. All package(s) installed or erased within a single transaction have a common identifier.

--triggeredby PACKAGE_NAME

Query packages that are triggered by package(s) PACKAGE_NAME.

--whatobsoletes CAPABILITY

Query all packages that obsolete CAPABILITY for proper functioning.

--whatprovides CAPABILITY

Query all packages that provide the CAPABILITY capability.

--whatrequires CAPABILITY

Query all packages that require CAPABILITY for proper functioning.

--whatconflicts CAPABILITY

Query all packages that conflict with CAPABILITY.

--whatrecommends CAPABILITY

Query all packages that recommend CAPABILITY.

--whatsuggests CAPABILITY

Query all packages that suggest CAPABILITY.

--whatsupplements CAPABILITY

Query all packages that supplement CAPABILITY.

--whatenhances CAPABILITY

Query all packages that enhance CAPABILITY.

## PACKAGE QUERY OPTIONS

--changelog

Display change information for the package.

--changes

Display change information for the package with full timestamps.

--conflicts

List capabilities this package conflicts with.

--dump

Dump file information as follows (implies -l):

path size mtime digest mode owner group isconfig isdoc rdev symlink

--enhances

List capabilities enhanced by package(s).

--filesbypkg

List all the files in each selected package.

--filetriggers

List filetrigger scriptlets from package(s).

-i, --info

Display package information, including name, version, and description. This uses the --queryformat if one was specified.

--last

Orders the package listing by install time such that the latest packages are at the top.

-l, --list

List files in package.

--obsoletes

List packages this package obsoletes.

--provides

List capabilities this package provides.

--recommends

List capabilities recommended by package(s).

-R, --requires

List capabilities on which this package depends.

--suggests

List capabilities suggested by package(s).

--supplements

List capabilities supplemented by package(s).

--scripts

List the package specific scriptlet(s) that are used as part of the installation and uninstallation processes.

-s, --state

Display the states of files in the package (implies -l). The state of each file is one of normal, not installed, or replaced.

--triggers, --triggerscripts

Display the trigger scripts, if any, which are contained in the package.

--xml

Format package headers as XML.

## FILE SELECTION OPTIONS

-A, --artifactfiles

Only include artifact files (implies -l).

-c, --configfiles

Only include configuration files (implies -l).

-d, --docfiles

Only include documentation files (implies -l).

-L, --licensefiles

Only include license files (implies -l).

--noartifact

Exclude artifact files.

--noconfig

Exclude config files.

--noghost

Exclude ghost files.

## VERIFY OPTIONS

The package and file selection options are the same as for package querying (including [rpm-manifest(5)](https://rpm-software-management.github.io/rpm/man/rpm-manifest.5) files as arguments). Other options unique to verify mode are:

--nodeps

Don't verify dependencies of packages.

--nodigest

Don't verify package or header digests when reading.

--nofiles

Don't verify any attributes of package files.

--noscripts

Don't execute the %verifyscript scriptlet (if any).

--nosignature

Don't verify package or header signatures when reading.

--nolinkto --nofiledigest (formerly --nomd5) --nosize --nomtime --nomode --nordev

Don't verify the corresponding file attribute.

--nouser --nogroup

Don't verify file user/group ownership. Note that only local passwd(5) and group(5) databases are consulted.

--nocaps

Don't verify file capabilities.

## OUTPUT

--verify

The format of the output is a string of 9 result characters a possible attribute from the package, followed by the filename.

Each of the 9 characters denotes the result of a comparison of attribute(s) of the file to the value of those attribute(s) recorded in the database.

A single "." (period) means the test passed, while a single "?" (question mark) indicates the test could not be performed (e.g. file permissions prevent reading). Otherwise, the (mnemonically emBoldened) character denotes failure of the corresponding --verify test:

|  Result  |  Description

|  S  |  file Size differs

|  M  |  Mode differs (includes permissions and file type)

|  5  |  digest (formerly MD5 sum) differs

|  D  |  Device major/minor number mismatch

|  L  |  readLink(2) path mismatch

|  U  |  User ownership differs

|  G  |  Group ownership differs

|  T  |  mTime differs

|  P  |  caPabilities differ

|  Attribute  |  Description

|  a  |  %artifact - an implicit side-effect file (eg. build-id links)

|  c  |  %configuration file

|  d  |  %documentation file

|  g  |  %ghost file

|  l  |  %license file

|  m  |  %missingok - file missing is not a verify failure

|  n  |  %config(noreplace) - do not replace (a %config file)

|  r  |  %readme file

|  s  |  [rpm-spec(5)](https://rpm-software-management.github.io/rpm/man/rpm-spec.5) file in a source package

## EXIT STATUS

On success, 0 is returned, a nonzero failure code otherwise.

## EXAMPLES

rpm -Uvh hello-2.0-1.noarch.rpm

Install or upgrade hello-2.0-1.noarch.rpm package with verbose output and progress meters. -Uvh is probably the most common of all the rpm operations.

rpm -Uvh --excludedocs --root /srv/test mydist-23.mft

Install packages described by the mydist-23.mft [rpm-manifest(5)](https://rpm-software-management.github.io/rpm/man/rpm-manifest.5) file into the alternative system root at /srv/test, excluding all documentation files.

rpm -Fvh *.rpm

Freshen installed packages from a local directory, with verbose output and progress meters.

rpm -i kernel-6.15.4-200.x86_64.rpm

Install kernel-6.15.4-200.x86_64.rpm package quietly. On some Linux distributions, different kernel versions are intended to be installed in parallel like this, without erasing older ones to ensure there's a bootable kernel in case of problems. It's one of the few cases where -i is the right thing to use.

rpm -evh --allmatches libhello

Erase all versions of package libhello on the system, with verbose output and progress meters. Typically there is only one version of a package installed at a time, but in some cases it's possible to have different versions or different architectures of a package installed in parallel, in which case --allmatches can be handy.

rpm -vh --reinstall hello-2.0-1.noarch.rpm

Reinstall previously installed hello-2.0-1.noarch.rpm package, with verbose output and progress labels. Useful if eg. the files of a package get corrupted or erased accidentally.

rpm -v --restore --all

Restore permissions of files in all installed packages to their packaged specifications, with verbose output.

rpm --verify --noconfig openssh-server

Verify the integrity of the installed openssh-server package, ignoring changes in configuration.

rpm --verify --configfiles --verbose logrotate

List all config files of the installed logrotate package, showing both changed and unchanged files.

rpm -qa

List all installed packages, using the default formatting.

rpm -qlv --noartifact glibc

List the files of the glibc package in style similar to ls(1) command output with ls -l, omitting "artifact" files such as build-id's which are not usually the main interest.

rpm -q --qf "[%{filenames} %{filedigests}\n]" openssh-server

List filenames and their corresponding digests of installed openssh-server package.

rpm -qp --scripts --triggers --filetriggers myserver-1.0-1.x86_64.rpm

Show all scriptlets and triggers from the myserver-1.0-1.x86_64.rpm package file. Before installing unknown packages, at least do this!

rpm --target ppc64le --eval "%optflags"

Print the expansion of the %optflags macro for the ppc64le architecture.

## FILES

See [rpm-common(8)](https://rpm-software-management.github.io/rpm/man/rpm-common.8), [rpm-config(5)](https://rpm-software-management.github.io/rpm/man/rpm-config.5) and [rpm-rpmrc(5)](https://rpm-software-management.github.io/rpm/man/rpm-rpmrc.5).

## SEE ALSO

[rpm-common(8)](https://rpm-software-management.github.io/rpm/man/rpm-common.8), popt(3), [rpm2cpio(1)](https://rpm-software-management.github.io/rpm/man/rpm2cpio.1), [rpmbuild(1)](https://rpm-software-management.github.io/rpm/man/rpmbuild.1), [rpmdb(8)](https://rpm-software-management.github.io/rpm/man/rpmdb.8), [rpmkeys(8)](https://rpm-software-management.github.io/rpm/man/rpmkeys.8), [rpmsign(1)](https://rpm-software-management.github.io/rpm/man/rpmsign.1), [rpmspec(1)](https://rpm-software-management.github.io/rpm/man/rpmspec.1), [rpm-queryformat(7)](https://rpm-software-management.github.io/rpm/man/rpm-queryformat.7) [rpm-manifest(5)](https://rpm-software-management.github.io/rpm/man/rpm-manifest.5) [rpm-version(7)](https://rpm-software-management.github.io/rpm/man/rpm-version.7) [rpm-plugins(8)](https://rpm-software-management.github.io/rpm/man/rpm-plugins.8)

rpm --help - as rpm supports customizing the options via popt aliases it's impossible to guarantee that what's described in the manual matches what's available.

http://www.rpm.org/

rpm.org
