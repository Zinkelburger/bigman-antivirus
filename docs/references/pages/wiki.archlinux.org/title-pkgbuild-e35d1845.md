---
url: "https://wiki.archlinux.org/title/PKGBUILD#install"
title: "PKGBUILD - ArchWiki"
captured: "2026-11-02T18:50:01Z"
via: "live"
sha256: "e08ddea210353f02efc544ac62c7b5cd4321381b5e511047a0e2884ff62ab661"
wayback: "https://web.archive.org/web/20261027094158/https://wiki.archlinux.org/title/PKGBUILD"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# PKGBUILD - ArchWiki

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://wiki.archlinux.org/title/PKGBUILD#install>
> Snapshot: <https://web.archive.org/web/20261027094158/https://wiki.archlinux.org/title/PKGBUILD>

# PKGBUILD

- [Español](https://wiki.archlinux.org/title/PKGBUILD_(Español))
- [Français](https://wiki.archlinux.org/title/PKGBUILD_(Français))
- [Magyar](https://wiki.archlinux.org/title/PKGBUILD_(Magyar))
- [日本語](https://wiki.archlinux.jp/index.php/PKGBUILD)
- [Português](https://wiki.archlinux.org/title/PKGBUILD_(Português))
- [Русский](https://wiki.archlinux.org/title/PKGBUILD_(Русский))
- [中文（简体）](https://wiki.archlinuxcn.org/wiki/PKGBUILD)

From ArchWiki

Related articles

- [Arch build system](https://wiki.archlinux.org/title/Arch_build_system)
- [Arch package guidelines](https://wiki.archlinux.org/title/Arch_package_guidelines)
- [Arch User Repository](https://wiki.archlinux.org/title/Arch_User_Repository)
- [.SRCINFO](https://wiki.archlinux.org/title/.SRCINFO)

This article discusses variables definable by the maintainer in a PKGBUILD. For information on the PKGBUILD functions and creating packages in general, refer to [Creating packages](https://wiki.archlinux.org/title/Creating_packages). Also read [PKGBUILD(5)](https://man.archlinux.org/man/PKGBUILD.5).

A PKGBUILD is a [Bash](https://wiki.archlinux.org/title/Bash) script containing the build information required by [Arch Linux](https://wiki.archlinux.org/title/Arch_Linux) packages.

Packages in Arch Linux are built using the [makepkg](https://wiki.archlinux.org/title/Makepkg) utility. When makepkg is run, it searches for a PKGBUILD file in the current directory and follows the instructions therein to either compile or otherwise acquire the files to build a package archive—pkgname.pkg.tar.zst. The resulting package contains binary files and installation instructions, readily installable with [pacman](https://wiki.archlinux.org/title/Pacman).

Mandatory variables are pkgname, pkgver, pkgrel, and arch. license is not strictly necessary to build a package, but is recommended for any PKGBUILD shared with others, as makepkg will produce a warning if not present.

It is a common practice to define the variables in the PKGBUILD in the same order as given here. However, it is not mandatory.

Tip

- Use [namcap](https://wiki.archlinux.org/title/Namcap) to check PKGBUILDs for common packaging mistakes.
- Use [shellcheck(1)](https://man.archlinux.org/man/shellcheck.1) to check PKGBUILDs for common scripting mistakes. See also [SC2034](https://www.shellcheck.net/wiki/SC2034), [SC2154](https://www.shellcheck.net/wiki/SC2154) and [SC2164](https://www.shellcheck.net/wiki/SC2164):

shellcheck --shell=bash --exclude=SC2034,SC2154,SC2164 PKGBUILD

- [termux-language-server](https://aur.archlinux.org/packages/termux-language-server/)AUR provides a [language server](https://wiki.archlinux.org/title/Language_Server_Protocol) for PKGBUILD, makepkg.conf, etc.

See the .proto [files](https://gitlab.archlinux.org/pacman/pacman/-/tree/master/proto) in the /usr/share/pacman/ directory as examples.

## Package name

### pkgbase

When building regular packages, this variable should not be explicitly declared in the PKGBUILD: its value defaults to that of [#pkgname](https://wiki.archlinux.org/title/PKGBUILD#pkgname).

When building a [split package](https://man.archlinux.org/man/PKGBUILD.5#PACKAGE_SPLITTING), this variable can be used to explicitly specify the name to be used to refer to the group of packages in the output of makepkg and in the naming of source-only tarballs. The value is not allowed to begin with a hyphen. If not specified, the value will default to the first element in the pkgname array.

All options and directives for split packages default to the global values given in the PKGBUILD. Nevertheless, the following ones can be overridden within each split package’s packaging function: [#pkgdesc](https://wiki.archlinux.org/title/PKGBUILD#pkgdesc), [#arch](https://wiki.archlinux.org/title/PKGBUILD#arch), [#url](https://wiki.archlinux.org/title/PKGBUILD#url), [#license](https://wiki.archlinux.org/title/PKGBUILD#license), [#groups](https://wiki.archlinux.org/title/PKGBUILD#groups), [#depends](https://wiki.archlinux.org/title/PKGBUILD#depends), [#optdepends](https://wiki.archlinux.org/title/PKGBUILD#optdepends), [#provides](https://wiki.archlinux.org/title/PKGBUILD#provides), [#conflicts](https://wiki.archlinux.org/title/PKGBUILD#conflicts), [#replaces](https://wiki.archlinux.org/title/PKGBUILD#replaces), [#backup](https://wiki.archlinux.org/title/PKGBUILD#backup), [#options](https://wiki.archlinux.org/title/PKGBUILD#options), [#install](https://wiki.archlinux.org/title/PKGBUILD#install), and [#changelog](https://wiki.archlinux.org/title/PKGBUILD#changelog).

### pkgname

Either the name of the package, e.g. pkgname=foo, or, for split packages, an array of names, e.g. pkgname=(foo bar). Package names should only consist of lowercase alphanumerics and the following characters: @._+- (at symbol, dot, underscore, plus, hyphen). Names are not allowed to start with hyphens or dots. For the sake of consistency, pkgname should match the name of the source tarball of the software: for instance, if the software is in foobar-2.5.tar.gz, use pkgname=foobar.

## Version

### pkgver

The version of the package.

This should be the same as the version published by the author of the upstream software. It can contain letters, numbers, periods and underscores, but not a hyphen (-). If the author of the software uses one, replace it with an underscore (_). If the pkgver variable is used later in the PKGBUILD, then the underscore can easily be substituted for a hyphen, e.g. source=("${pkgname}-${pkgver//_/-}.tar.gz").

Note If upstream uses a timestamp versioning such as 30102014, ensure to use the reversed date, i.e. 20141030 ([ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format). Otherwise it will not appear as a newer version.

Tip

- The ordering of uncommon values can be tested with [vercmp(8)](https://man.archlinux.org/man/vercmp.8), which is provided by the [pacman](https://wiki.archlinux.org/title/Pacman) package.
- [makepkg](https://wiki.archlinux.org/title/Makepkg) can automatically [update](http://allanmcrae.com/2013/04/pacman-4-1-released/) this variable by defining a pkgver() function in the PKGBUILD. See [VCS package guidelines#The pkgver() function](https://wiki.archlinux.org/title/VCS_package_guidelines#The_pkgver()_function) for details.

See also [Arch package guidelines#Package versioning](https://wiki.archlinux.org/title/Arch_package_guidelines#Package_versioning).

### pkgrel

The release number. This is usually a positive integer number that allows to differentiate between consecutive builds of the same version of a package. As fixes and additional features are added to the PKGBUILD that influence the resulting package, the pkgrel should be incremented by 1. When a new version of the software is released, this value must be reset to 1. In exceptional cases other formats can be found in use, such as major.minor.

See also [Arch package guidelines#Package versioning](https://wiki.archlinux.org/title/Arch_package_guidelines#Package_versioning).

### epoch

Used to force the package to be seen as newer than any previous version with a lower epoch. This value is required to be a non-negative integer; the default is 0. It is used when the version numbering scheme of a package changes (or is alphanumeric), breaking normal version comparison logic. For example:

```
pkgver=5.13
pkgrel=2
epoch=1
```

```
1:5.13-2
```

See [pacman(8)](https://man.archlinux.org/man/pacman.8) for more information on version comparisons.

Warning epoch should only be used when absolutely required to do so.

## Generic

### pkgdesc

The description of the package. This is recommended to be 80 characters or less and should not include the package name in a self-referencing way, unless the application name differs from the package name. For example, use pkgdesc='Text editor for X11' instead of pkgdesc='Nedit is a text editor for X11'.

Also it is important to use keywords wisely to increase the chances of appearing in relevant search queries.

### arch

An array of architectures that the PKGBUILD is intended to build and work on. Arch officially supports only x86_64, but other projects may support other architectures. For example, [Arch Linux 32](https://archlinux32.org/) provides support for i686 and pentium4, and [Arch Linux ARM](https://archlinuxarm.org/) provides support for armv7h (armv7 hardfloat) and aarch64 (armv8 64-bit).

There are two types of values the array can use:

- arch=(any) indicates the package can be built on any architecture, and once built, is architecture-independent in its compiled state (usually shell scripts, fonts, themes, many types of extensions, [Java](https://wiki.archlinux.org/title/Java) programs, etc.).
- arch=(...) with one or more architectures (but not any) indicates the package can be compiled for any of the specified architectures, but is architecture-specific once compiled. For these packages, specify all architectures that the PKGBUILD officially supports. For official repository and [AUR](https://wiki.archlinux.org/title/AUR) packages, this means arch=('x86_64'). Optionally, AUR packages may choose to additionally support other known working architectures.

The target architecture can be accessed with the variable CARCH during a build.

### url

The URL of the official site of the software being packaged.

### license

This article or section is a candidate for merging with [Arch package guidelines#licenses](https://wiki.archlinux.org/title/Arch_package_guidelines#licenses).

Notes: The PKGBUILD format does not enforce a packaging policy. (Discuss in [Talk:PKGBUILD](https://wiki.archlinux.org/title/Talk:PKGBUILD))

This article or section needs expansion.

Reason: Add more details from [[1]](https://lists.archlinux.org/hyperkitty/list/arch-dev-public@lists.archlinux.org/thread/NFSB7734U2VVDULPRY65ECXDE3XGNZXM/). (Discuss in [Talk:PKGBUILD](https://wiki.archlinux.org/title/Talk:PKGBUILD))

The license under which the software is distributed. Arch Linux uses [SPDX license identifiers](https://en.wikipedia.org/wiki/Software_Package_Data_Exchange#License_syntax). Each license must have a corresponding entry in /usr/share/licenses/.

For common licenses (like GPL-3.0-or-later), package [licenses](https://archlinux.org/packages/?name=licenses) delivers all the corresponding files. The package is installed by default, as it is a dependency of [base](https://archlinux.org/packages/?name=base) [meta package](https://wiki.archlinux.org/title/Meta_package), and the files may be found in /usr/share/licenses/spdx/. Simply refer to the license using its SPDX license identifier from the [list of SPDX identifiers](https://spdx.org/licenses/).

License families like BSD or MIT are, strictly speaking, not a single license and each instance requires a separate license file. In license variable refer to them using a common SPDX identifier (e.g. BSD-3-Clause or MIT), but then provide the corresponding file as if it was a custom license.

For custom licenses the identifier should be either LicenseRef-license-name or custom:license-name, if they are not covered by the common families mentioned above. The corresponding license text must be placed in directory /usr/share/licenses/pkgname. To install the file a following code snippet may be used in package() section:

```
install -Dm644 LICENSE -t "${pkgdir}/usr/share/licenses/${pkgname}/"

```

Note pkgdir variable is defined by makepkg, see [PKGBUILD(5) § PACKAGING FUNCTIONS](https://man.archlinux.org/man/PKGBUILD.5#PACKAGING_FUNCTIONS) for more information.

Combining multiple licenses or adding exceptions should follow the [SPDX syntax](https://spdx.github.io/spdx-spec/v3.0/annexes/SPDX-license-expressions/). For example a package released under either GNU/GPL 2.0 or GNU/LGPL 2.1 could use 'GPL-2.0-or-later OR LGPL-2.1-or-later', a package released under Apache 2.0 with LLVM exception would use 'Apache-2.0 WITH LLVM-exception' and a package released with part under the BSD 3 clause, others under GNU/LGPL 2.1 and some under GNU/GPL 2.0 would use 'BSD-3-Clause AND LGPL-2.1-or-later AND GPL-2.0-or-later'[[2]](https://gitlab.archlinux.org/archlinux/packaging/packages/musepack/-/blob/f9642f78a1a7c61e6b8d181b138ee3f7e8717054/PKGBUILD#L16). Note that this must be a single string, so the entire expression has to be enclosed in quotes. As for November 2023 [SPDX list of exceptions](https://spdx.org/licenses/preview/exceptions-index.html) is limited, so usually the custom license route must be used.

If issues are encountered with SPDX identifiers, during the transitional period using old identifiers —names of the directories in /usr/share/licenses/common— is acceptable.

See also [Nonfree applications package guidelines](https://wiki.archlinux.org/title/Nonfree_applications_package_guidelines).

Additional information and perspectives on free and open source software licenses may be found on the following pages:

- [Wikipedia:Free software license](https://en.wikipedia.org/wiki/Free_software_license)
- [Wikipedia:Comparison of free and open-source software licenses](https://en.wikipedia.org/wiki/Comparison_of_free_and_open-source_software_licenses)
- [A Legal Issues Primer for Open Source and Free Software Projects](https://www.softwarefreedom.org/resources/2008/foss-primer.html)
- [GNU Project - Various Licenses and Comments about Them](https://www.gnu.org/licenses/license-list.html)
- [Debian - License information](https://www.debian.org/legal/licenses/)
- [Open Source Initiative - Licenses by Name](https://www.opensource.org/licenses/alphabetical)

### groups

The [group](https://wiki.archlinux.org/title/Package_group) the package belongs in. For instance, when installing [plasma](https://archlinux.org/groups/x86_64/plasma/), it installs all packages belonging in that group.

## Dependencies

Note Additional architecture-specific arrays can be added by appending an underscore and the architecture name, e.g. optdepends_x86_64=().

### depends

An array of packages that must be installed for the software to build and run. Dependencies defined inside the package() function are only required to run the software.

Version restrictions can be specified with comparison operators, e.g. depends=('foobar>=1.8.0'); if multiple restrictions are needed, the dependency can be repeated for each, e.g. depends=('foobar>=1.8.0' 'foobar<2.0.0').

This article or section is a candidate for merging with [Arch package guidelines](https://wiki.archlinux.org/title/Arch_package_guidelines).

Notes: The PKGBUILD format does not enforce a packaging policy. (Discuss in [Talk:PKGBUILD](https://wiki.archlinux.org/title/Talk:PKGBUILD))

The depends array should list all direct first level dependencies even when some are already declared transitively. For instance, if a package foo depends on both bar and baz, and the bar package depends in turn on baz too, it will ultimately lead to undesired behavior if bar stops pulling in baz. Pacman will not enforce the installation of baz on systems which newly install the foo package, or have cleaned up orphans, and foo will crash at runtime or otherwise misbehave.

In some cases this is not necessary and may or may not be listed, for example [glibc](https://archlinux.org/packages/?name=glibc) cannot be uninstalled as every system needs some C library, or [python](https://archlinux.org/packages/?name=python) for a package that already depends on another python- module, as the second module must per definition depend on python and cannot ever stop pulling it in as a dependency.

Dependencies should normally include the requirements for building all optional features of a package. Alternatively, any feature whose dependencies are not included should be explicitly disabled via a configure option. Failure to do this can lead to packages with "[automagic dependencies](https://wiki.gentoo.org/wiki/Project:Quality_Assurance/Automagic_dependencies)" build-time optional features that are unpredictably enabled due to transitive dependencies or unrelated software installed on the build machine, but which are not reflected in the package dependencies.

If the dependency name appears to be a library, e.g. depends=(libfoobar.so), makepkg will try to find a binary that depends on the library in the built package and append the [soname](https://en.wikipedia.org/wiki/soname) version needed by the binary. Manually appending the version disables automatic detection, e.g. depends=('libfoobar.so=2').

### makedepends

An array of packages that are only required to build the package. The minimum dependency version can be specified in the same format as in the depends array. The packages in the depends array are implicitly required to build the package, they should not be duplicated here.

Note

- The package [base-devel](https://archlinux.org/packages/?name=base-devel) is assumed to be already installed when building with makepkg. Dependencies of this package should not be included in makedepends array.
- If using [VCS sources](https://wiki.archlinux.org/title/VCS_package_guidelines#VCS_sources), do not forget to include the appropriate VCS tool ([git](https://archlinux.org/packages/?name=git), [subversion](https://archlinux.org/packages/?name=subversion), [cvs](https://archlinux.org/packages/?name=cvs), ...).

Tip Use pactree -rsud1 package | grep base-devel to check whether a particular package is a direct dependency of [base-devel](https://archlinux.org/packages/?name=base-devel) (requires [pacman-contrib](https://archlinux.org/packages/?name=pacman-contrib)).

### checkdepends

An array of packages that the software depends on to run its test suite, but are not needed at runtime. Packages in this list follow the same format as depends. These dependencies are only considered when the [check()](https://wiki.archlinux.org/title/Creating_packages#check()) function is present and is to be run by makepkg.

Note The package [base-devel](https://archlinux.org/packages/?name=base-devel) is assumed to be already installed when building with makepkg. Dependencies of this package should not be included in checkdepends array.

### optdepends

An array of packages that are not needed for the software to function, but provide additional features. This may imply that not all executables provided by a package will function without the respective optdepends.[[3]](https://lists.archlinux.org/archives/list/arch-general@lists.archlinux.org/message/ZFBCD2NFFBSOZBI22AYZYHH2PDRCLJWF/) If the software works on multiple alternative dependencies, all of them can be listed here, instead of the depends array.

A short description of the extra functionality each optdepend provides should also be noted:

```
optdepends=('cups: printing support'
            'sane: scanners support'
            'libgphoto2: digital cameras support'
            'alsa-lib: sound support'
            'giflib: GIF images support'
            'libjpeg: JPEG images support'
            'libpng: PNG images support')

```

## Package relations

Note Additional architecture-specific arrays can be added by appending an underscore and the architecture name, e.g. conflicts_x86_64=().

### provides

An array of additional packages that the software provides the features of, including [virtual packages](https://wiki.archlinux.org/title/Pacman#Virtual_packages) such as cron or sh and [all external shared libraries](https://wiki.archlinux.org/title/Arch_package_guidelines#Package_relations). Packages providing the same item can be installed side-by-side, unless at least one of them uses a conflicts array.

Note

- The version that the package provides should be mentioned (pkgver and potentially the pkgrel), in case packages referencing the software require one. For instance, a modified qt package version 3.3.8, named qt-foobar, should use provides=('qt=3.3.8'); omitting the version number would cause the dependencies that require a specific version of qt to fail.
- Do not add pkgname to the provides array, as it is done automatically.

### conflicts

An array of packages that conflict with, or cause problems with the package, if installed. All these packages and packages providing this item will need to be removed. The version properties of the conflicting packages can also be specified in the same format as the depends array.

Note that conflicts are checked against pkgname as well as names specified in the provides array. Hence, if your package provides a foo feature, specifying foo in the conflicts array will cause a conflict between your package and all other packages that contain foo in their provides array (i.e., there is no need to specify all those conflicting package names in your conflicts array). Let us take a concrete example:

- [netbeans](https://archlinux.org/packages/?name=netbeans) implicitly provides netbeans as the pkgname itself
- A hypothetical netbeans-cpp package would provide netbeans and conflicts with netbeans
- A hypothetical netbeans-php package would provide netbeans and conflicts with netbeans, but does not need to explicitly conflict with netbeans-cpp since packages providing the same feature are implicitly in conflict.

When packages provide the same feature via the provides array, there is a difference between explicitly adding the alternative package to the conflicts array and not adding it. If the conflicts array is explicitly declared the two packages providing the same feature will be considered as alternative; if the conflicts array is missing the two packages providing the same feature will be considered as possibly cohabiting. Packagers should always ignore the content of the provides variable in deciding whether to declare a conflicts variable or not.

### replaces

An array of obsolete packages that are replaced by the package, e.g. [wireshark-qt](https://archlinux.org/packages/?name=wireshark-qt) uses replaces=('wireshark'). When syncing, pacman will immediately replace an installed package upon encountering another package with the matching replaces in the repositories. If providing an alternate version of an already existing package or uploading to the AUR, use the conflicts and provides arrays, which are only evaluated when actually installing the conflicting package.

## Others

### backup

An array of files that can contain user-made changes and should be preserved during upgrade or removal of a package, primarily intended for configuration files in /etc. If these files are unchanged from how they ship with the package, they will be removed or replaced as normal files during upgrade or removal.

Files in this array should use relative paths without the leading slash (/) (e.g. etc/pacman.conf, instead of /etc/pacman.conf). The backup array [does not support empty directories or wildcards such as "*"](https://bbs.archlinux.org/viewtopic.php?pid=2187778).

When updating, new versions may be saved as file.pacnew to avoid overwriting a file which already exists and was previously modified by the user. Similarly, when the package is removed, user-modified files will be preserved as file.pacsave unless the package was removed with the pacman -Rn command.

See also [Pacnew and Pacsave files](https://wiki.archlinux.org/title/Pacnew_and_Pacsave_files).

### options

This array allows overriding some of the default behavior of makepkg, defined in /etc/makepkg.conf. To set an option, include the name in the array. To disable an option, place an ! before it.

The full list of the available options can be found in [PKGBUILD(5) § OPTIONS AND DIRECTIVES](https://man.archlinux.org/man/PKGBUILD.5#OPTIONS_AND_DIRECTIVES).

### install

The name of the .install script to be included in the package.

pacman has the ability to store and execute a package-specific script when it installs, removes or upgrades a package. The script contains the following functions which run at different times:

- pre_install — The script is run right before files are extracted. One argument is passed: new package version.
- post_install — The script is run right after files are extracted. Any additional notes that should be printed after the package is installed should be located here. One argument is passed: new package version.
- pre_upgrade — The script is run right before files are extracted. Two arguments are passed in the following order: new package version, old package version.
- post_upgrade — The script is run right after files are extracted. Two arguments are passed in the following order: new package version, old package version.
- pre_remove — The script is run right before files are removed. One argument is passed: old package version.
- post_remove — The script is run right after files are removed. One argument is passed: old package version.

Each function is run [chrooted](https://wiki.archlinux.org/title/Chroot) inside the pacman install directory. See [this thread](https://bbs.archlinux.org/viewtopic.php?pid=913891).

Tip

- A prototype .install is provided at [/usr/share/pacman/proto.install](https://gitlab.archlinux.org/pacman/pacman/raw/master/proto/proto.install).
- [pacman#Hooks](https://wiki.archlinux.org/title/Pacman#Hooks) provide similar functionality.

Note Do not end the script with exit. This would prevent the contained functions from executing.

### changelog

The name of the package changelog. To view changelogs for installed packages (that have this file):

```
$ pacman -Qc pkgname

```

## Sources

### source

An array of files needed to build the package. It must contain the location of the software source, which in most cases is a full HTTP or FTP URL. The previously set variables pkgname and pkgver can be used effectively here; e.g. source=("https://example.com/${pkgname}-${pkgver}.tar.gz").

Files can also be supplied in the same directory where the PKGBUILD is located, and their names added to this array. Before the actual build process starts, all the files referenced in this array will be downloaded or checked for existence, and makepkg will not proceed if any is missing.

.install files are recognized automatically by makepkg and should not be included in the source array. Files in the source array with extensions .sig, .sign, or .asc are recognized by makepkg as PGP signatures and will be automatically used to verify the integrity of the corresponding source file.

Warning The downloaded source filename must be unique because the [SRCDEST](https://wiki.archlinux.org/title/Makepkg#Package_output) directory can be the same for all packages. For instance, using the version number of the project as a filename potentially conflicts with other projects with the same version number. In this case, the alternative unique filename to be used is provided with the syntax source=('unique_package_name::file_uri'); e.g. source=("${pkgname}-${pkgver}.tar.gz::https://github.com/coder/program/archive/v${pkgver}.tar.gz").

Tip

- Additional architecture-specific arrays can be added by appending an underscore and the architecture name, e.g. source_x86_64=(). There must be a corresponding integrity array with checksums, e.g. sha256sums_x86_64=().
- Some servers restrict download by filtering the User-Agent string of the client or other types of restrictions, which can be circumvented with [DLAGENTS](https://wiki.archlinux.org/title/Nonfree_applications_package_guidelines#Custom_DLAGENTS).
- Use file:// URL to point to a directory or a file in your computer filesystem. For example, a local Git repository can be specified as "${pkgname}::git+file:///path/to/repository".
- Magnet link support can be added using [transmission-dlagent](https://aur.archlinux.org/packages/transmission-dlagent/)AUR as DLAGENT and using the magnet:// URI prefix instead of the canonical magnet:?.
- See [PKGBUILD(5) § USING VCS SOURCES](https://man.archlinux.org/man/PKGBUILD.5#USING_VCS_SOURCES) and [VCS package guidelines#vcs-sources](https://wiki.archlinux.org/title/VCS_package_guidelines#vcs-sources) for details on VCS specific options, such as targeting a specific Git branch or commit.

### noextract

An array of files listed under source, which should not be extracted from their archive format by [makepkg](https://wiki.archlinux.org/title/Makepkg). This can be used with archives that cannot be handled by /usr/bin/bsdtar or those that need to be installed as-is. If an alternative unarchiving tool is used (e.g. [lrzip](https://archlinux.org/packages/?name=lrzip)), it should be added in the makedepends array and the first line of the [prepare()](https://wiki.archlinux.org/title/Creating_packages#prepare()) function should extract the source archive manually; for example:

```
prepare() {
  lrzip -d source.tar.lrz
}

```

Note that while the source array accepts URLs, noextract is just the file name portion:

```
source=("http://foo.org/bar/foobar.tar.xz")
noextract=('foobar.tar.xz')

```

To extract nothing, consider the following:

- If source contains only plain URLs without custom file names, strip the source array before the last slash:

```
noextract=("${source[@]##*/}")
```

- If source contains only entries with custom file names, strip the source array after the :: separator (taken from a previous version of [firefox-i18n's PKGBUILD](https://gitlab.archlinux.org/archlinux/packaging/packages/firefox-i18n/-/blob/46492498ef720353cbb84de5096102de694faf90/PKGBUILD#L132)):

```
noextract=("${source[@]%%::*}")
```

Note If an archive has many top-level files, there is a risk of unwanted overwriting files extracted from other source archives. If that is the case, consider adding it to noextract and extracting it manually into a subdirectory.

### validpgpkeys

An array of PGP fingerprints. If used, makepkg will only accept signatures from the keys listed here and will ignore the trust values from the keyring. If the source file was signed with a subkey, makepkg will still use the primary key for comparison.

Only full fingerprints are accepted. They must be uppercase and must not contain whitespace characters.

Note Use gpg --list-keys --fingerprint KEYID to find out the fingerprint of the appropriate key.

Tip The [export-pkgbuild-keys(1)](https://man.archlinux.org/man/export-pkgbuild-keys.1) command exports the public keys from the local keyring. This way, they do not have to be downloaded externally which improves supply chain security, reproducibility, and automation.

Please read [makepkg#Signature checking](https://wiki.archlinux.org/title/Makepkg#Signature_checking) for more information.

## Integrity

These variables are arrays whose items are checksum strings that will be used to verify the integrity of the respective files in the [source](https://wiki.archlinux.org/title/PKGBUILD#source) array. Insert SKIP for a particular file, and its checksum will not be tested.

The checksum type and values should always be those provided by upstream, such as in release announcements. When multiple types are provided by upstream, the strongest checksum is to be preferred (in order from most to least preferred): b2, sha512, sha384, sha256, sha224, sha1, md5, ck. This best ensures the integrity of the downloaded files, from upstream announcement to package building.

Note Additionally, when upstream makes [digital signatures](https://en.wikipedia.org/wiki/Digital_signature) available, the signature files should be added to the [source](https://wiki.archlinux.org/title/PKGBUILD#source) array and the PGP key fingerprint to the [validpgpkeys](https://wiki.archlinux.org/title/PKGBUILD#validpgpkeys) array. This allows authentication of the files at build time.

The values for these variables can be auto-generated by [makepkg](https://wiki.archlinux.org/title/Makepkg)'s -g/--geninteg option, then commonly appended with makepkg -g >> PKGBUILD. The [updpkgsums(8)](https://man.archlinux.org/man/updpkgsums.8) command from [pacman-contrib](https://archlinux.org/packages/?name=pacman-contrib) is able to update the variables wherever they are in the PKGBUILD. Both tools will use the variable that is already set in the PKGBUILD, or fall back to sha256sums if none is set.

The file integrity checks to use can be set up with the INTEGRITY_CHECK option in /etc/makepkg.conf. See [makepkg.conf(5)](https://man.archlinux.org/man/makepkg.conf.5).

Note Additional architecture-specific arrays can be added by appending an underscore and the architecture name, e.g. sha256sums_x86_64=().

### b2sums

An array of [BLAKE2b](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE2) checksums with digest size of 512 bits.

### sha512sums, sha384sums, sha256sums, sha224sums

An array of [SHA-2](https://en.wikipedia.org/wiki/SHA-2) checksums with digest sizes 512, 384, 256 and 224 bits, respectively. sha256sums is the most common of them.

### sha1sums

An array of 160-bit [SHA-1](https://en.wikipedia.org/wiki/SHA-1) checksums of the files listed in the source array.

### md5sums

An array of 128-bit [MD5](https://en.wikipedia.org/wiki/MD5) checksums of the files listed in the source array.

### cksums

An array [CRC32](https://en.wikipedia.org/wiki/Cyclic_redundancy_check) checksums (from UNIX-standard [cksum](https://en.wikipedia.org/wiki/cksum)) of the files listed in the source array.

Retrieved from "https://wiki.archlinux.org/index.php?title=PKGBUILD&oldid=882712"

[Category](https://wiki.archlinux.org/title/Special:Categories):

- [Package development](https://wiki.archlinux.org/title/Category:Package_development)

Hidden categories:

- [Pages or sections flagged with Template:Merge](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Merge)
- [Pages or sections flagged with Template:Expansion](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Expansion)
