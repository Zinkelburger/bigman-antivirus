---
url: "https://man7.org/linux/man-pages/man8/rpm.8.html"
title: "rpm(8) - Linux manual page"
captured: "2026-11-02T18:40:57Z"
via: "live"
sha256: "4a74d9f93db23d34ee4ce8326d2d18c1743b619de33bf67852abc61f7b754116"
wayback: "https://web.archive.org/web/20260719162531/https://www.man7.org/linux/man-pages/man8/rpm.8.html"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# rpm(8) - Linux manual page

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://man7.org/linux/man-pages/man8/rpm.8.html>
> Snapshot: <https://web.archive.org/web/20260719162531/https://www.man7.org/linux/man-pages/man8/rpm.8.html>

|

[man7.org](https://man7.org/index.html) > Linux > [man-pages](https://man7.org/linux/man-pages/index.html)

|

[Linux/UNIX system programming training](http://man7.org/training/)

# rpm(8) — Linux manual page

|

[NAME](https://man7.org/linux/man-pages/man8/rpm.8.html#NAME) | [SYNOPSIS](https://man7.org/linux/man-pages/man8/rpm.8.html#SYNOPSIS) | [DESCRIPTION](https://man7.org/linux/man-pages/man8/rpm.8.html#DESCRIPTION) | [OPERATIONS](https://man7.org/linux/man-pages/man8/rpm.8.html#OPERATIONS) | [ARGUMENTS](https://man7.org/linux/man-pages/man8/rpm.8.html#ARGUMENTS) | [OPTIONS](https://man7.org/linux/man-pages/man8/rpm.8.html#OPTIONS) | [INSTALL AND UPGRADE OPTIONS](https://man7.org/linux/man-pages/man8/rpm.8.html#INSTALL_AND_UPGRADE_OPTIONS) | [ERASE OPTIONS](https://man7.org/linux/man-pages/man8/rpm.8.html#ERASE_OPTIONS) | [TRANSACTION OPTIONS](https://man7.org/linux/man-pages/man8/rpm.8.html#TRANSACTION_OPTIONS) | [QUERY OPTIONS](https://man7.org/linux/man-pages/man8/rpm.8.html#QUERY_OPTIONS) | [PACKAGE SELECTION OPTIONS](https://man7.org/linux/man-pages/man8/rpm.8.html#PACKAGE_SELECTION_OPTIONS) | [PACKAGE QUERY OPTIONS](https://man7.org/linux/man-pages/man8/rpm.8.html#PACKAGE_QUERY_OPTIONS) | [FILE SELECTION OPTIONS](https://man7.org/linux/man-pages/man8/rpm.8.html#FILE_SELECTION_OPTIONS) | [VERIFY OPTIONS](https://man7.org/linux/man-pages/man8/rpm.8.html#VERIFY_OPTIONS) | [OUTPUT](https://man7.org/linux/man-pages/man8/rpm.8.html#OUTPUT) | [EXIT STATUS](https://man7.org/linux/man-pages/man8/rpm.8.html#EXIT_STATUS) | [EXAMPLES](https://man7.org/linux/man-pages/man8/rpm.8.html#EXAMPLES) | [FILES](https://man7.org/linux/man-pages/man8/rpm.8.html#FILES) | [SEE ALSO](https://man7.org/linux/man-pages/man8/rpm.8.html#SEE_ALSO) | [COLOPHON](https://man7.org/linux/man-pages/man8/rpm.8.html#COLOPHON)

|

|

```

RPM(8)                   System Manager's Manual                   RPM(8)

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#NAMENAME [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       rpm - RPM Package Manager

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#SYNOPSISSYNOPSIS [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

   Querying and verifying packages
       rpm {-q|--query} [select-options] [query-options] [PACKAGE_FILE
       ...] [PACKAGE_NAME ...]

       rpm {-V|--verify} [select-options] [verify-options] [PACKAGE_NAME
       ...]

   Installing, upgrading and removing packages
       rpm {-i|--install} [transaction-options] [install-options]
       PACKAGE_FILE ...

       rpm {-U|--upgrade} [transaction-options] [install-options]
       PACKAGE_FILE ...

       rpm {-F|--freshen} [transaction-options] [install-options]
       PACKAGE_FILE ...

       rpm --reinstall [transaction-options] [install-options]
       PACKAGE_FILE ...

       rpm --restore [transaction-options] [select-options] PACKAGE_NAME
       ...

       rpm {-e|--erase} [transaction-options] [erase-options]
       PACKAGE_NAME ...

   Misc operations
       rpm --querytags

       See [rpm-common(8)](https://man7.org/linux/man-pages/man8/rpm-common.8.html) for operations common to all rpm executables.

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#DESCRIPTIONDESCRIPTION [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       rpm is a powerful Package Manager, which can be used to build,
       install, query, verify, update, and erase individual software
       packages. A package consists of an archive of files and meta-data
       used to install and erase the archive files. The meta-data
       includes helper scripts, file attributes, and descriptive
       information about the package. Packages come in two varieties:
       binary packages, used to encapsulate software to be installed, and
       source packages, containing the source code and recipe necessary
       to produce binary packages.

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#OPERATIONSOPERATIONS [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

   Querying and verifying packages
       -V, --verify
           Verify package(s), comparing information about the installed
           files in the package with information about the files taken
           from the package metadata stored in the rpm database. Among
           other things, verifying compares the size, digest,
           permissions, type, owner and group of each file. By default,
           only files with discrepancies are displayed. Use --verbose to
           display all verified files.

       -q, --query
           Query package files or installed package(s).

   Installing, upgrading and removing packages
       -i, --install
           Install new package(s) without considering upgrade scenarios.
           This is considered special usage, normally installations
           should be done with --upgrade.

       -U, --upgrade
           Install or upgrade package(s) to a newer version. Old and/or
           obsoleted package(s) are removed after the new package is
           installed.

       -F, --freshen
           Upgrade package(s), but only ones for which an earlier version
           is installed.

       --reinstall
           Reinstall previously installed package(s).

       --restore
           Restore file metadata such as timestamp, owner, group,
           permissions and capabilities of files of installed package(s).

       -e, --erase
           Erase installed packages.

   Misc operations
       --querytags
           Dump known querytags. Useful with the --queryformat option.

   Obsolete compatibility aliases
       These are obsolete popt(3) aliases for backwards compatibility
       only, and their use is discouraged.

       --initdb, --rebuilddb, --verifydb
           See [rpmdb(8)](https://man7.org/linux/man-pages/man8/rpmdb.8.html).

       --addsign, --delsign, --resign
           See [rpmsign(1)](https://man7.org/linux/man-pages/man1/rpmsign.1.html).

       -K, --checksig, --import
           See [rpmkeys(8)](https://man7.org/linux/man-pages/man8/rpmkeys.8.html).

       --specfile
           See [rpmspec(1)](https://man7.org/linux/man-pages/man1/rpmspec.1.html).

       See [rpm-common(8)](https://man7.org/linux/man-pages/man8/rpm-common.8.html) for the operations common to all rpm
       executables.

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#ARGUMENTSARGUMENTS [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       PACKAGE_FILE
           Either an rpm package or an [rpm-manifest(5)](https://man7.org/linux/man-pages/man5/rpm-manifest.5.html) file. May also be
           specified as an ftp or http URL, in which case the package
           will be downloaded before being installed. See FTP/HTTP
           OPTIONS for information on rpm's ftp and http client support.

       PACKAGE_NAME
           Installed package named PACKAGE_NAME. To specify the package
           more precisely the package name may be followed by the version
           or version and release both separated by a dash or an
           architecture name separated by a dot. See the output of rpm
           -qa or rpm -qp PACKAGE_FILE as an example.

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#OPTIONSOPTIONS [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       See [rpm-common(8)](https://man7.org/linux/man-pages/man8/rpm-common.8.html) for the options common to all operations.

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#INSTALL_AND_UPGRADE_OPTIONSINSTALL AND UPGRADE OPTIONS [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       --allfiles
           Installs or upgrades all the missingok files in the package,
           regardless if they exist.

       --badreloc
           Used with --relocate, permit relocations on all file paths,
           not just those OLDPATH's included in the binary package
           relocation hint(s).

       --excludepath OLDPATH
           Don't install files whose name begins with OLDPATH.

       --excludeartifacts
           Don't install any files which are marked as artifacts, such as
           build-id links.

       --excludeconfigs, --noconfigs
           Do not install configuration files.

       --excludedocs
           Don't install any files which are marked as documentation
           (which includes man pages and texinfo documents).

       --force
           Same as using --replacepkgs, --replacefiles, and --oldpackage.

       --ignoresize
           Don't check mount filesystems for sufficient disk space before
           installing this package.

       --ignorearch
           Allow installation or upgrading even if the architectures of
           the binary package and host don't match.

       --ignoreos
           Allow installation or upgrading even if the operating systems
           of the binary package and host don't match.

       --includedocs
           Install documentation files. This is the default behavior.

       --nocontexts
           Don't set SELinux contexts for files and scriptlets. Only
           effective if [rpm-plugin-selinux(8)](https://man7.org/linux/man-pages/man8/rpm-plugin-selinux.8.html) is installed and active.

       --nocaps
           Don't set file capabilities.

       --noverify
           Don't perform verify package files prior to installation.

       --nosysusers
           Don't create [rpm-sysusers(7)](https://man7.org/linux/man-pages/man7/rpm-sysusers.7.html) from packages

       --oldpackage
           Allow an upgrade to replace a newer package with an older one.

       --prefix NEWPATH
           For relocatable binary packages, translate all file paths that
           start with the installation prefix in the package relocation
           hint(s) to NEWPATH.

       --relocate OLDPATH=NEWPATH
           For relocatable binary packages, translate all file paths that
           start with OLDPATH in the package relocation hint(s) to
           NEWPATH. This option can be used repeatedly if several
           OLDPATH's in the package are to be relocated.

       --replacefiles
           Install the packages even if they replace files from other,
           already installed, packages.

       --replacepkgs
           Install the packages even if some of them are already
           installed on this system.

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#ERASE_OPTIONSERASE OPTIONS [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       --allmatches
           Remove all versions of the package which match PACKAGE_NAME.
           Normally an error is issued if PACKAGE_NAME matches multiple
           packages.

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#TRANSACTION_OPTIONSTRANSACTION OPTIONS [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       The transaction options are common to --install, --upgrade,
       --reinstall and --erase operations.

       --deploops
           Print dependency loops as warnings.

       -h, --hash
           Print 50 hash marks as the package archive is unpacked. Use
           with -v|--verbose for a nicer display.

       --justdb
           Update only the database, not the filesystem.

       --nodb
           Update only the filesystem, not the database.

       --nodeps
           Don't check dependencies before uninstalling the packages.

       --noorder
           Don't reorder the packages for an install. The list of
           packages would normally be reordered to satisfy dependencies.

       --noplugins
           Do not load and execute plugins.

       --noscripts
           Disables execution of package scriptlets. Equivalent to

           --nopre --nopost --nopreun --nopostun --nopretrans
           --noposttrans --nopreuntrans --nopostuntrans

       --nopre --nopost --nopreun --nopostun --nopretrans --noposttrans
       --nopreuntrans --nopostuntrans
           Don't execute the scriptlets of the named type.

       --notriggers
           Disable execution of package triggers. Equivalent to

           --notriggerprein --notriggerin --notriggerun --notriggerpostun

       --notriggerin --notriggerun --notriggerprein --notriggerpostun
           Disable execution of the trigger scriptlets of the named type.

       --percent
           Print percentages as files are unpacked from the package
           archive. This is intended to make rpm easy to run from other
           tools.

       --test
           Don't really execute anything, just go through the motions.
           Useful in conjunction with the -vv option for debugging.

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#QUERY_OPTIONSQUERY OPTIONS [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       --qf QUERYFMT, --queryformat QUERYFMT
           Output format of each queried package, as described by
           [rpm-queryformat(7)](https://man7.org/linux/man-pages/man7/rpm-queryformat.7.html). If omitted, uses the format defined by
           %_query_all_fmt in [rpm-config(5)](https://man7.org/linux/man-pages/man5/rpm-config.5.html).

       There are three subsets of options for querying: package
       selection, file selection and information selection.

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#PACKAGE_SELECTION_OPTIONSPACKAGE SELECTION OPTIONS [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       -a, --all [SELECTOR ...]
           Query all installed packages.

           Optional SELECTOR's in the form of tag=pattern can be provided
           to narrow the selection, for example name="b*" to query
           packages whose name starts with "b".

       --dupes
           List duplicated packages.

       -f, --file FILE
           Query package owning installed FILE.

       --filecaps
           List filenames with POSIX1.e capabilities.

       --fileclass
           List filenames with their classes (libmagic classification).

       --filecolor
           List filenames with their colors (0 for noarch, 1 for 32bit, 2
           for 64 bit).

       --fileprovide
           List filenames with their provides.

       --filerequire
           List filenames with their requires.

       -g, --group GROUP
           Query packages with the group of GROUP.

       --noglob
           Do not glob arguments when installing package files.

       --nomanifest
           Don't process non-package files as [rpm-manifest(5)](https://man7.org/linux/man-pages/man5/rpm-manifest.5.html) files.

       -p, --package PACKAGE_FILE
           Query an (uninstalled) package PACKAGE_FILE.

       --path PATH
           Query package(s) owning PATH, whether the file is installed or
           not. Multiple packages may own a PATH, but the file is only
           owned by the package installed last.

       --querybynumber HDRNUM
           Query the HDRNUM database entry directly; this is useful only
           for debugging.

       --specfile SPECFILE
           Obsolete, use [rpmspec(1)](https://man7.org/linux/man-pages/man1/rpmspec.1.html) instead.

       --tid TID
           Query package(s) that have a given TID transaction identifier.
           A UNIX timestamp is currently used as a transaction
           identifier. All package(s) installed or erased within a single
           transaction have a common identifier.

       --triggeredby PACKAGE_NAME
           Query packages that are triggered by package(s) PACKAGE_NAME.

       --whatobsoletes CAPABILITY
           Query all packages that obsolete CAPABILITY for proper
           functioning.

       --whatprovides CAPABILITY
           Query all packages that provide the CAPABILITY capability.

       --whatrequires CAPABILITY
           Query all packages that require CAPABILITY for proper
           functioning.

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

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#PACKAGE_QUERY_OPTIONSPACKAGE QUERY OPTIONS [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       --changelog
           Display change information for the package.

       --changes
           Display change information for the package with full
           timestamps.

       --conflicts
           List capabilities this package conflicts with.

       --dump
           Dump file information as follows (implies -l):

           path size mtime digest mode owner group isconfig isdoc rdev
           symlink

       --enhances
           List capabilities enhanced by package(s).

       --filesbypkg
           List all the files in each selected package.

       --filetriggers
           List filetrigger scriptlets from package(s).

       -i, --info
           Display package information, including name, version, and
           description. This uses the --queryformat if one was specified.

       --last
           Orders the package listing by install time such that the
           latest packages are at the top.

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
           List the package specific scriptlet(s) that are used as part
           of the installation and uninstallation processes.

       -s, --state
           Display the states of files in the package (implies -l). The
           state of each file is one of normal, not installed, or
           replaced.

       --triggers, --triggerscripts
           Display the trigger scripts, if any, which are contained in
           the package.

       --xml
           Format package headers as XML.

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#FILE_SELECTION_OPTIONSFILE SELECTION OPTIONS [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

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

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#VERIFY_OPTIONSVERIFY OPTIONS [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       The package and file selection options are the same as for package
       querying (including [rpm-manifest(5)](https://man7.org/linux/man-pages/man5/rpm-manifest.5.html) files as arguments). Other
       options unique to verify mode are:

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

       --nolinkto --nofiledigest (formerly --nomd5) --nosize --nomtime
       --nomode --nordev
           Don't verify the corresponding file attribute.

       --nouser --nogroup
           Don't verify file user/group ownership. Note that only local
           [passwd(5)](https://man7.org/linux/man-pages/man5/passwd.5.html) and [group(5)](https://man7.org/linux/man-pages/man5/group.5.html) databases are consulted.

       --nocaps
           Don't verify file capabilities.

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#OUTPUTOUTPUT [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       --verify
           The format of the output is a string of 9 result characters a
           possible attribute from the package, followed by the filename.

           Each of the 9 characters denotes the result of a comparison of
           attribute(s) of the file to the value of those attribute(s)
           recorded in the database.

           A single "." (period) means the test passed, while a single
           "?" (question mark) indicates the test could not be performed
           (e.g. file permissions prevent reading). Otherwise, the
           (mnemonically emBoldened) character denotes failure of the
           corresponding --verify test:

       Result   Description
       S        file Size differs
       M        Mode differs (includes permissions and file type)
       5        digest (formerly MD5 sum) differs
       D        Device major/minor number mismatch
       L        readLink(2) path mismatch
       U        User ownership differs
       G        Group ownership differs
       T        mTime differs
       P        caPabilities differ

       Attribute   Description
       a           %artifact - an implicit side-effect file (eg. build-id
                   links)
       c           %configuration file
       d           %documentation file
       g           %ghost file
       l           %license file
       m           %missingok - file missing is not a verify failure
       n           %config(noreplace) - do not replace (a %config file)
       r           %readme file
       s           rpm-spec(5) file in a source package

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#EXIT_STATUSEXIT STATUS [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       On success, 0 is returned, a nonzero failure code otherwise.

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#EXAMPLESEXAMPLES [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       rpm -Uvh hello-2.0-1.noarch.rpm
           Install or upgrade hello-2.0-1.noarch.rpm package with verbose
           output and progress meters. -Uvh is probably the most common
           of all the rpm operations.

       rpm -Uvh --excludedocs --root /srv/test mydist-23.mft
           Install packages described by the mydist-23.mft
           [rpm-manifest(5)](https://man7.org/linux/man-pages/man5/rpm-manifest.5.html) file into the alternative system root at
           /srv/test, excluding all documentation files.

       rpm -Fvh *.rpm
           Freshen installed packages from a local directory, with
           verbose output and progress meters.

       rpm -i kernel-6.15.4-200.x86_64.rpm
           Install kernel-6.15.4-200.x86_64.rpm package quietly. On some
           Linux distributions, different kernel versions are intended to
           be installed in parallel like this, without erasing older ones
           to ensure there's a bootable kernel in case of problems. It's
           one of the few cases where -i is the right thing to use.

       rpm -evh --allmatches libhello
           Erase all versions of package libhello on the system, with
           verbose output and progress meters. Typically there is only
           one version of a package installed at a time, but in some
           cases it's possible to have different versions or different
           architectures of a package installed in parallel, in which
           case --allmatches can be handy.

       rpm -vh --reinstall hello-2.0-1.noarch.rpm
           Reinstall previously installed hello-2.0-1.noarch.rpm package,
           with verbose output and progress labels. Useful if eg. the
           files of a package get corrupted or erased accidentally.

       rpm -v --restore --all
           Restore permissions of files in all installed packages to
           their packaged specifications, with verbose output.

       rpm --verify --noconfig openssh-server
           Verify the integrity of the installed openssh-server package,
           ignoring changes in configuration.

       rpm --verify --configfiles --verbose logrotate
           List all config files of the installed logrotate package,
           showing both changed and unchanged files.

       rpm -qa
           List all installed packages, using the default formatting.

       rpm -qlv --noartifact glibc
           List the files of the glibc package in style similar to [ls(1)](https://man7.org/linux/man-pages/man1/ls.1.html)
           command output with ls -l, omitting "artifact" files such as
           build-id's which are not usually the main interest.

       rpm -q --qf "[%{filenames} %{filedigests}\n]" openssh-server
           List filenames and their corresponding digests of installed
           openssh-server package.

       rpm -qp --scripts --triggers --filetriggers
       myserver-1.0-1.x86_64.rpm
           Show all scriptlets and triggers from the
           myserver-1.0-1.x86_64.rpm package file. Before installing
           unknown packages, at least do this!

       rpm --target ppc64le --eval "%optflags"
           Print the expansion of the %optflags macro for the ppc64le
           architecture.

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#FILESFILES [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       See [rpm-common(8)](https://man7.org/linux/man-pages/man8/rpm-common.8.html), [rpm-config(5)](https://man7.org/linux/man-pages/man5/rpm-config.5.html) and [rpm-rpmrc(5)](https://man7.org/linux/man-pages/man5/rpm-rpmrc.5.html).

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#SEE_ALSOSEE ALSO [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       [rpm-common(8)](https://man7.org/linux/man-pages/man8/rpm-common.8.html), popt(3), [rpm2cpio(1)](https://man7.org/linux/man-pages/man1/rpm2cpio.1.html), [rpmbuild(1)](https://man7.org/linux/man-pages/man1/rpmbuild.1.html), [rpmdb(8)](https://man7.org/linux/man-pages/man8/rpmdb.8.html),
       [rpmkeys(8)](https://man7.org/linux/man-pages/man8/rpmkeys.8.html), [rpmsign(1)](https://man7.org/linux/man-pages/man1/rpmsign.1.html), [rpmspec(1)](https://man7.org/linux/man-pages/man1/rpmspec.1.html), [rpm-queryformat(7)](https://man7.org/linux/man-pages/man7/rpm-queryformat.7.html)
       [rpm-manifest(5)](https://man7.org/linux/man-pages/man5/rpm-manifest.5.html) [rpm-version(7)](https://man7.org/linux/man-pages/man7/rpm-version.7.html) [rpm-plugins(8)](https://man7.org/linux/man-pages/man8/rpm-plugins.8.html)

       rpm --help - as rpm supports customizing the options via popt
       aliases it's impossible to guarantee that what's described in the
       manual matches what's available.

       http://www.rpm.org/

```

## https://man7.org/linux/man-pages/man8/rpm.8.html#COLOPHONCOLOPHON [top](https://man7.org/linux/man-pages/man8/rpm.8.html#top_of_page)

```

       This page is part of the rpm (RPM Package Manager) project.
       Information about the project can be found at
       ⟨https://github.com/rpm-software-management/rpm⟩.  It is not known
       how to report bugs for this man page; if you know, please send a
       mail to man-pages@man7.org.  This page was obtained from the
       project's upstream Git repository
       ⟨https://github.com/rpm-software-management/rpm.git⟩ on
       2026-08-04.  (At that time, the date of the most recent commit
       that was found in the repository was 2026-07-27.)  If you discover
       any rendering problems in this HTML version of the page, or you
       believe there is a better or more up-to-date source for the page,
       or you have corrections or improvements to the information in this
       COLOPHON (which is not part of the original manual page), send a
       mail to man-pages@man7.org

RPM 6.1.90                      2026-08-03                         RPM(8)

```

Pages that refer to this page: [rpm2archive(1)](https://man7.org/linux/man-pages/man1/rpm2archive.1.html), [rpm2cpio(1)](https://man7.org/linux/man-pages/man1/rpm2cpio.1.html), [rpmbuild(1)](https://man7.org/linux/man-pages/man1/rpmbuild.1.html), [rpmdeps(1)](https://man7.org/linux/man-pages/man1/rpmdeps.1.html), [rpmgraph(1)](https://man7.org/linux/man-pages/man1/rpmgraph.1.html), [rpmlua(1)](https://man7.org/linux/man-pages/man1/rpmlua.1.html), [rpm-setup-autosign(1)](https://man7.org/linux/man-pages/man1/rpm-setup-autosign.1.html), [rpmsign(1)](https://man7.org/linux/man-pages/man1/rpmsign.1.html), [rpmspec(1)](https://man7.org/linux/man-pages/man1/rpmspec.1.html), [yum-versionlock(1)](https://man7.org/linux/man-pages/man1/yum-versionlock.1.html), [rpm-config(5)](https://man7.org/linux/man-pages/man5/rpm-config.5.html), [rpm-manifest(5)](https://man7.org/linux/man-pages/man5/rpm-manifest.5.html), [rpm-rpmrc(5)](https://man7.org/linux/man-pages/man5/rpm-rpmrc.5.html), [rpm-spec(5)](https://man7.org/linux/man-pages/man5/rpm-spec.5.html), [daemon(7)](https://man7.org/linux/man-pages/man7/daemon.7.html), [rpm-dependency-generators(7)](https://man7.org/linux/man-pages/man7/rpm-dependency-generators.7.html), [rpm-design(7)](https://man7.org/linux/man-pages/man7/rpm-design.7.html), [rpm-macros(7)](https://man7.org/linux/man-pages/man7/rpm-macros.7.html), [rpm-queryformat(7)](https://man7.org/linux/man-pages/man7/rpm-queryformat.7.html), [rpm-scriptlets(7)](https://man7.org/linux/man-pages/man7/rpm-scriptlets.7.html), [rpm-sysusers(7)](https://man7.org/linux/man-pages/man7/rpm-sysusers.7.html), [rpm-version(7)](https://man7.org/linux/man-pages/man7/rpm-version.7.html), [rpm-common(8)](https://man7.org/linux/man-pages/man8/rpm-common.8.html), [rpmdb(8)](https://man7.org/linux/man-pages/man8/rpmdb.8.html), [rpmkeys(8)](https://man7.org/linux/man-pages/man8/rpmkeys.8.html), [rpm-plugin-ima(8)](https://man7.org/linux/man-pages/man8/rpm-plugin-ima.8.html), [rpm-plugin-prioreset(8)](https://man7.org/linux/man-pages/man8/rpm-plugin-prioreset.8.html), [rpm-plugins(8)](https://man7.org/linux/man-pages/man8/rpm-plugins.8.html), [rpm-plugin-selinux(8)](https://man7.org/linux/man-pages/man8/rpm-plugin-selinux.8.html), [rpm-plugin-syslog(8)](https://man7.org/linux/man-pages/man8/rpm-plugin-syslog.8.html), [rpm-plugin-systemd-inhibit(8)](https://man7.org/linux/man-pages/man8/rpm-plugin-systemd-inhibit.8.html), [yumdb(8)](https://man7.org/linux/man-pages/man8/yumdb.8.html)

|

HTML rendering created 2026-09-09 by [Michael Kerrisk](https://man7.org/mtk/index.html), author of [The Linux Programming Interface](https://man7.org/tlpi/).

For details of in-depth Linux/UNIX system programming training courses that I teach, look [here](https://man7.org/training/).

Hosting by [jambit GmbH](https://www.jambit.com/index_en.html).

|    |  https://man7.org/tlpi/
