---
url: "https://wiki.archlinux.org/title/Arch_User_Repository"
title: "Arch User Repository - ArchWiki"
captured: "2026-11-02T18:48:55Z"
via: "live"
sha256: "404cb1c607a5daf94610a6fe9616476da5d49177c1f84dab7abdc0e39cf73b81"
wayback: "https://web.archive.org/web/20261024103526/https://wiki.archlinux.org/title/Arch_User_Repository"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# Arch User Repository - ArchWiki

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://wiki.archlinux.org/title/Arch_User_Repository>
> Snapshot: <https://web.archive.org/web/20261024103526/https://wiki.archlinux.org/title/Arch_User_Repository>

# Arch User Repository

- [Deutsch](https://wiki.archlinux.de/title/Arch_User_Repository)
- [Español](https://wiki.archlinux.org/title/Arch_User_Repository_(Español))
- [Français](https://wiki.archlinux.org/title/Arch_User_Repository_(Français))
- [Magyar](https://wiki.archlinux.org/title/Arch_User_Repository_(Magyar))
- [Italiano](https://wiki.archlinux.org/title/Arch_User_Repository_(Italiano))
- [日本語](https://wiki.archlinux.jp/index.php/Arch_User_Repository)
- [Português](https://wiki.archlinux.org/title/Arch_User_Repository_(Português))
- [Русский](https://wiki.archlinux.org/title/Arch_User_Repository_(Русский))
- [Türkçe](https://wiki.archlinux.org/title/Arch_User_Repository_(Türkçe))
- [Українська](https://wiki.archlinux.org/title/Arch_User_Repository_(Українська))
- [中文（简体）](https://wiki.archlinuxcn.org/wiki/Arch_%E7%94%A8%E6%88%B7%E8%BD%AF%E4%BB%B6%E4%BB%93%E5%BA%93)

From ArchWiki

Related articles

- [AUR Cleanup Day](https://wiki.archlinux.org/title/AUR_Cleanup_Day)
- [AUR helpers](https://wiki.archlinux.org/title/AUR_helpers)
- [AUR submission guidelines](https://wiki.archlinux.org/title/AUR_submission_guidelines)
- [Aurweb RPC interface](https://wiki.archlinux.org/title/Aurweb_RPC_interface)
- [Creating packages](https://wiki.archlinux.org/title/Creating_packages)
- [Official repositories](https://wiki.archlinux.org/title/Official_repositories)
- [Package Maintainer guidelines](https://wiki.archlinux.org/title/Package_Maintainer_guidelines)
- [Unofficial user repositories](https://wiki.archlinux.org/title/Unofficial_user_repositories)
- [.SRCINFO](https://wiki.archlinux.org/title/.SRCINFO)

The Arch User Repository (AUR) is a community-driven repository for [Arch Linux](https://wiki.archlinux.org/title/Arch_Linux) users. It contains package descriptions ([PKGBUILDs](https://wiki.archlinux.org/title/PKGBUILD)) that allow you to compile a package from source with [makepkg](https://wiki.archlinux.org/title/Makepkg) and then install it via [pacman](https://wiki.archlinux.org/title/Pacman#Additional_commands). The AUR was created to organize and share new packages from the community and to help expedite popular packages' inclusion into the [extra](https://wiki.archlinux.org/title/Extra) repository. This document explains how users can access and utilize the AUR.

A good number of new packages that enter the official repositories start in the AUR. In the AUR, users are able to contribute their own package builds (PKGBUILD and related files). The AUR community has the ability to [vote](https://wiki.archlinux.org/title/Arch_User_Repository#Voting_for_packages) for packages in the AUR. If a package becomes popular enough—provided it has a compatible license and good packaging technique—it may be entered into the extra repository (directly accessible by pacman or from the [Arch build system](https://wiki.archlinux.org/title/Arch_build_system)).

Warning AUR packages are user-produced content. These PKGBUILDs are completely unofficial and have not been thoroughly vetted. Any use of the provided files is at your own risk.

Note If you plan to use AUR, it is highly recommended to follow [aur-general Arch mailing list](https://lists.archlinux.org/mailman3/lists/aur-general.lists.archlinux.org/) which has been used for security warnings in the past. [[1]](https://lists.archlinux.org/archives/list/aur-general@lists.archlinux.org/thread/FFCMZGL4UQODYKZGUY7KTN3UBF3XN66P/) [[2]](https://lists.archlinux.org/archives/list/aur-general@lists.archlinux.org/thread/7EZTJXLIAQLARQNTMEW2HBWZYE626IFJ/) [[3]](https://lists.archlinux.org/archives/list/aur-general@lists.archlinux.org/thread/FGXPCB3ZVCJIV7FX323SBAX2JHYB7ZS4/) [[4]](https://archlinux.org/news/active-aur-malicious-packages-incident/) [[5]](https://lists.archlinux.org/archives/list/aur-general@lists.archlinux.org/thread/BU6RECTA5DTJBL7Q4NQI5T3AKIN2FWSF/) [[6]](https://lists.archlinux.org/archives/list/aur-general@lists.archlinux.org/thread/IYRETAX4N5IGUV7JNNQIC7ZBYEDLHSJJ/) [[7]](https://lists.archlinux.org/archives/list/aur-general@lists.archlinux.org/thread/DRDEU3JUSC72CB265XHXPFA3DFSLXPBP/)

## Getting started

Users can search and download PKGBUILDs from the [AUR Web Interface](https://aur.archlinux.org). These PKGBUILDs can be built into installable packages using makepkg, then installed using pacman.

- Ensure [base-devel](https://archlinux.org/packages/?name=base-devel) is [installed](https://wiki.archlinux.org/title/Install).
- Glance over the [#Frequently asked questions](https://wiki.archlinux.org/title/Arch_User_Repository#Frequently_asked_questions) for answers to the most common questions.
- You may wish to adjust /etc/makepkg.conf to optimize the build process to your system prior to building packages from the AUR. A significant improvement in package build times can be realized on systems with multi-core processors by adjusting the MAKEFLAGS variable, by using multiple cores for compression, or by using different compression algorithm. Users can also enable hardware-specific compiler optimizations via the CFLAGS variable. See [makepkg#Optimization](https://wiki.archlinux.org/title/Makepkg#Optimization) for more information.

if you have set up [AUR SSH authentication](https://wiki.archlinux.org/title/AUR_submission_guidelines#Authentication) then it is also possible to interact with the AUR through SSH: type ssh aur@aur.archlinux.org help for a list of available commands.

## Installing and upgrading packages

Installing packages from the AUR is a relatively simple process. Essentially:

- [Acquire the build files](https://wiki.archlinux.org/title/Arch_User_Repository#Acquire_build_files), including the PKGBUILD and possibly other required files, like [systemd](https://wiki.archlinux.org/title/Systemd) units and patches (often not the actual code).
- [Verify the PKGBUILD](https://wiki.archlinux.org/title/Arch_User_Repository#Verify_the_PKGBUILD); the most important step to verify that the PKGBUILD and accompanying files are not malicious or untrustworthy.
- [Build the package](https://wiki.archlinux.org/title/Arch_User_Repository#Build_the_package), by running makepkg in the directory where the files are saved. This will download the code, compile it, and package it.
- [Install the package](https://wiki.archlinux.org/title/Arch_User_Repository#Install_the_package), by executing pacman -U package_file to install the package onto your system.

First, ensure that the necessary tools are [installed](https://wiki.archlinux.org/title/Install) by installing [base-devel](https://archlinux.org/packages/?name=base-devel); this [meta package](https://wiki.archlinux.org/title/Meta_package) has [make](https://archlinux.org/packages/?name=make) and other tools needed for compiling from source, listed as dependencies.

Note Packages in the AUR assume that [base-devel](https://archlinux.org/packages/?name=base-devel) is installed in the build environment.

Next, choose an appropriate build directory. A build directory is simply a directory where the package will be made or "built" from source, and can be any directory. The examples in the following sections will use ~/builds as the build directory.

### Acquire build files

Locate the package in the AUR. This is done using the search field at the top of the [AUR home page](https://aur.archlinux.org/). Clicking the application's name in the search list brings up an information page on the package. Read through the description to confirm that this is the desired package, note when the package was last updated, and read any comments.

There are several methods for acquiring the build files for a package:

- Clone its [git](https://wiki.archlinux.org/title/Git) repository, labeled "Git Clone URL" in the "Package Details" on its AUR page. This is the preferred method, an advantage of which is that you can easily get updates to the package via git pull.

```
$ git clone https://aur.archlinux.org/package_name.git
```

- Download a snapshot, either by clicking the "Download snapshot" link under "Package Actions" on the right hand side of its AUR page, or in a terminal:

```
$ curl -L -O https://aur.archlinux.org/cgit/aur.git/snapshot/package_name.tar.gz
```

Note The snapshot file is compressed, and must be extracted (preferably in a directory set aside for AUR builds): tar -xvf package_name.tar.gz

- Use the read-only mirror [archlinux/aur on GitHub](https://github.com/archlinux/aur), where every package is located in a branch. It is recommended to clone only a single branch (the whole repository is too big and performance would be low). You can do this with one of the following two methods:

- Use git clone --single-branch:

```
$ git clone --branch package_name --single-branch https://github.com/archlinux/aur.git
```

- Do a [partial clone](https://wiki.archlinux.org/title/Git#Partially_fetching_the_repository) of this repository (git clone --depth=1) and [add branches](https://wiki.archlinux.org/title/Git#Getting_other_branches) selectively:

```
$ git clone --depth=1 https://github.com/archlinux/aur.git; cd aur
$ git remote set-branches --add origin package_name
$ git fetch
$ git switch package_name
```

### Verify the PKGBUILD

Change directories to the directory containing the package PKGBUILD.

```
$ cd package_name

```

Warning Carefully check the PKGBUILD, any .install files, and any other files in the package's git repository for malicious or dangerous commands. If in doubt, do not build the package, and [seek advice](https://wiki.archlinux.org/title/General_troubleshooting#Additional_support) on the forums or mailing list. Malicious code has been found in packages before. [[8]](https://lists.archlinux.org/archives/list/aur-general@lists.archlinux.org/thread/FFCMZGL4UQODYKZGUY7KTN3UBF3XN66P/) [[9]](https://lists.archlinux.org/archives/list/aur-general@lists.archlinux.org/thread/7EZTJXLIAQLARQNTMEW2HBWZYE626IFJ/) [[10]](https://archlinux.org/news/active-aur-malicious-packages-incident/) [[11]](https://lists.archlinux.org/archives/list/aur-general@lists.archlinux.org/thread/FGXPCB3ZVCJIV7FX323SBAX2JHYB7ZS4/) A few tools, such as [traur](https://aur.archlinux.org/packages/traur/)AUR and [ks-aur-scanner](https://aur.archlinux.org/packages/ks-aur-scanner/)AUR, are available to assist users in scanning PKGBUILD content; however, they are not substitutes for careful manual verification.

View the contents of all provided files. For example, to use the pager less to view PKGBUILD, do:

```
$ less PKGBUILD

```

Check if a signature file in the form of .sig or .asc is part of the PKGBUILD source array. If that is the case, then acquire one of the public keys listed in the PKGBUILD [validpgpkeys](https://wiki.archlinux.org/title/PKGBUILD#validpgpkeys) array. Refer to [makepkg#Signature checking](https://wiki.archlinux.org/title/Makepkg#Signature_checking) for more information.

In any case, be aware also malicious code may be signed, therefore you must vet the PKGBUILD, particularly for sections that may pull software code from outside the repositories.

Tip If you are updating a package, you may want to look at the changes since the last commit.

- To view changes since the last git commit, you can use git show.
- To view changes since the last commit using vimdiff, do git difftool @~..@ --tool=vimdiff. The advantage of vimdiff is that you view the entire contents of each file along with indicators on what has changed.

### Build the package

This article or section needs expansion.

Reason: Suggest [building in a clean chroot](https://wiki.archlinux.org/title/Building_in_a_clean_chroot). (Discuss in [Talk:Arch User Repository](https://wiki.archlinux.org/title/Talk:Arch_User_Repository))

Make the package. After manually confirming the contents of the files, run [makepkg](https://wiki.archlinux.org/title/Makepkg) as a normal user. Some helpful flags:

- -s/--syncdeps automatically resolves and installs any dependencies with [pacman](https://wiki.archlinux.org/title/Pacman) before building. If the package depends on other AUR packages, you will need to manually install them first.
- -i/--install installs the package if it is built successfully. This lets you skip the next step that is usually done manually.
- -r/--rmdeps removes build-time dependencies after the build, as they are no longer needed. However, these dependencies may need to be reinstalled the next time the package is updated.
- -c/--clean cleans up temporary build files after the build, as they are no longer needed. These files are usually needed only when debugging the build process.
- --packagelist lists package filenames that would be produced without building.

Tip Use  git clean -dfx to delete all files that are not tracked by git, thus deleting all previously built package files.

### Install the package

The package can now be installed with pacman:

```
# pacman -U package_name-version-architecture.pkg.tar.zst

```

Note

- If you have changed your PKGEXT in makepkg.conf, the name of the package file may be slightly different.
- The above example is only a brief summary of the build process. It is highly recommended to read the [makepkg](https://wiki.archlinux.org/title/Makepkg) and [Arch build system](https://wiki.archlinux.org/title/Arch_build_system) articles for more details.

### Upgrading packages

In the directory containing the package's PKGBUILD, you must first update the files and changes by using the command

```
$ git pull

```

then follow the previous build and install instructions.

### Updating packages

The AUR is unsupported, so any packages you install are your responsibility to update, not pacman's. If packages in the official repositories are updated, you will need to rebuild any AUR packages that depend on those libraries. The checkrebuild tool and rebuild-detector [hook](https://wiki.archlinux.org/title/Pacman#Hooks) from [rebuild-detector](https://archlinux.org/packages/?name=rebuild-detector) can help find packages needing rebuilt.

## Account status

### Suspension

When editing a user as a [Package Maintainer](https://wiki.archlinux.org/title/Package_Maintainer), the Suspended field can be set, which suspends the target user. When a user is suspended, they cannot:

- Login to https://aur.archlinux.org
- Receive notifications
- Interact with the git interface

### Inactivity

When editing your own account or another as a Package Maintainer, the Inactive field can be set. Inactive accounts are used for two reasons:

- Display the date someone was marked inactive on their account page
- Generate a current count of active Package Maintainer based on their inactivity for new proposals

## Feedback

### Commenting on packages

The [AUR Web Interface](https://aur.archlinux.org) has a comments facility that allows users to provide suggestions and feedback on improvements to the PKGBUILD contributor.

Tip Avoid pasting patches or PKGBUILDs into the comments section: they quickly become obsolete and just end up needlessly taking up lots of space. Instead, email those files to the maintainer, or even use a [pastebin](https://wiki.archlinux.org/title/Pastebin).

[Python-Markdown](https://python-markdown.github.io/) provides basic [Markdown](https://en.wikipedia.org/wiki/Markdown) syntax to format comments.

Note

- This implementation has some occasional [differences](https://python-markdown.github.io/#differences) with the official [syntax rules](https://daringfireball.net/projects/markdown/syntax).
- Commit hashes to the [Git](https://wiki.archlinux.org/title/Git) repository of the package and references to [Flyspray](https://wiki.archlinux.org/title/Flyspray) tickets are converted to links automatically.
- Long comments are collapsed and can be expanded on demand.

### Voting for packages

One of the easiest activities for all Arch users is to browse the AUR and vote for their favourite packages using the online interface. All packages are eligible for adoption by a Package Maintainer for inclusion in the [extra repository](https://wiki.archlinux.org/title/Extra_repository), and the vote count is one of the considerations in that process; it is in everyone's interest to vote!

Sign up on the [AUR website](https://aur.archlinux.org/) to get a "Vote for this package" option while browsing packages. After signing up, it is also possible to vote from the commandline with [aur-auto-vote-git](https://aur.archlinux.org/packages/aur-auto-vote-git/)AUR.

Alternatively, if you have set up [AUR SSH authentication](https://wiki.archlinux.org/title/AUR_submission_guidelines#Authentication), you can directly vote from the command line using your ssh key. This means that you will not need to save or type in your AUR password.

```
$ ssh aur@aur.archlinux.org vote package_name

```

### Flagging packages out-of-date

First, you should flag the package out-of-date indicating details on why the package is outdated, preferably including links to the release announcement or the new release [tarball](https://wiki.archlinux.org/title/Archiving_and_compression#Archiving_only).

You should also try to reach out to the maintainer directly by email. If there is no response from the maintainer after two weeks, you can file an orphan request. See [AUR submission guidelines#Requests](https://wiki.archlinux.org/title/AUR_submission_guidelines#Requests) for details.

Note [VCS packages](https://wiki.archlinux.org/title/VCS_package_guidelines) are not considered out of date when the pkgver changes; do not flag them as the maintainer will merely unflag the package and ignore you. AUR maintainers should not commit mere pkgver bumps.

## Debugging the package build process

- Ensure your build environment is up-to-date by [upgrading](https://wiki.archlinux.org/title/Pacman#Upgrading_packages) before building anything.
- Ensure you have [base-devel](https://archlinux.org/packages/?name=base-devel) installed.
- Use the -s option with makepkg to check and install all dependencies needed before starting the build process.
- Try the default [makepkg configuration](https://gitlab.archlinux.org/archlinux/packaging/packages/pacman/-/blob/main/makepkg.conf).
- See [Makepkg#Troubleshooting](https://wiki.archlinux.org/title/Makepkg#Troubleshooting) for common issues.

If you are having trouble building a package, first read its PKGBUILD and the comments on its AUR page.

It is possible that a PKGBUILD is broken for everyone. If you cannot figure it out on your own, report it to the maintainer (e.g. by [posting the errors you are getting in the comments on the AUR page](https://wiki.archlinux.org/title/Arch_User_Repository#Commenting_on_packages)). You may also seek help in the [AUR Issues, Discussion & PKGBUILD Requests forum](https://bbs.archlinux.org/viewforum.php?id=38).

The reason might not be trivial after all. Custom CFLAGS, LDFLAGS and MAKEFLAGS can cause failures. To avoid problems caused by your particular system configuration, build packages in a [clean chroot](https://wiki.archlinux.org/title/Clean_chroot). If the build process still fails in a clean chroot, the issue is probably with the PKGBUILD.

See [Creating packages#Checking package sanity](https://wiki.archlinux.org/title/Creating_packages#Checking_package_sanity) about using [namcap](https://wiki.archlinux.org/title/Namcap). If you would like to have a PKGBUILD reviewed, post it on the [aur-general mailing list](https://lists.archlinux.org/mailman3/lists/aur-general.lists.archlinux.org/) to get feedback from the [Package Maintainers](https://wiki.archlinux.org/title/Package_Maintainers) and fellow AUR members, or the [Creating & Modifying Packages forum](https://bbs.archlinux.org/viewforum.php?id=4). You could also seek help in the [IRC channel](https://wiki.archlinux.org/title/IRC_channel) #archlinux-aur on the [Libera Chat](https://libera.chat) network.

## Submitting packages

Users can share PKGBUILDs using the Arch User Repository. See [AUR submission guidelines](https://wiki.archlinux.org/title/AUR_submission_guidelines) for details.

## Web interface translation

See [i18n.md](https://gitlab.archlinux.org/archlinux/aurweb/-/blob/master/doc/i18n.md) in the AUR source tree for information about creating and maintaining translation of the [AUR Web Interface](https://aur.archlinux.org).

## History

In the beginning, there was ftp://ftp.archlinux.org/incoming, and people contributed by simply uploading the PKGBUILD, the needed supplementary files, and the built package itself to the server. The package and associated files remained there until a "Trusted user" (renamed as [Package Maintainer](https://wiki.archlinux.org/title/Package_Maintainer)) saw the program and adopted it.

Then the Trusted User Repositories were born. Certain individuals in the community were allowed to host their own repositories for anyone to use. The AUR expanded on this basis, with the aim of making it both more flexible and more usable. In fact, the AUR maintainers were referred to as TUs (Trusted Users) until the change in naming to Package Maintainers.

Between 2015-06-08 and 2015-08-08, the AUR transitioned from version 3.5.1 to 4.0.0, introducing the use of Git repositories for publishing the PKGBUILDs. Existing packages were dropped unless manually migrated to the new infrastructure by their maintainers.

### Git repositories for AUR3 packages

The [AUR Archive](https://github.com/aur-archive) on GitHub has a repository for every package that was in AUR 3 at the time of the migration. Alternatively, there is the [aur3-mirror](https://github.com/felixonmars/aur3-mirror/) repository which provides the same.

## Frequently asked questions

### What kind of packages are permitted on the AUR?

The packages on the AUR are merely "build scripts", i.e. recipes to build binaries for [pacman](https://wiki.archlinux.org/title/Pacman). For most cases, everything is permitted, subject to [usefulness and scope guidelines](https://wiki.archlinux.org/title/AUR_submission_guidelines#Rules_of_submission), as long as you are in compliance with the licensing terms of the content. For other cases, where it is mentioned that "you may not link" to downloads, i.e. contents that are not redistributable, you may only use the file name itself as the source. This means and requires that users already have the restricted source in the build directory prior to building the package. When in doubt, ask.

### How can I vote for packages in the AUR?

See [#Voting for packages](https://wiki.archlinux.org/title/Arch_User_Repository#Voting_for_packages).

### What is a package maintainer?

See [Arch terminology#Package maintainer](https://wiki.archlinux.org/title/Arch_terminology#Package_maintainer).

### What is the difference between the Arch User Repository and the extra repository?

The Arch User Repository is where all PKGBUILDs that users submit are stored, and must be built manually with [makepkg](https://wiki.archlinux.org/title/Makepkg). When PKGBUILDs receive enough community interest and the support of a [Package Maintainer](https://wiki.archlinux.org/title/Package_Maintainer), they are moved into the [extra repository](https://wiki.archlinux.org/title/Extra_repository) (maintained by the Package Maintainers), where the binary packages can be installed with [pacman](https://wiki.archlinux.org/title/Pacman).

### Foo in the AUR is outdated; what should I do?

See [#Flagging packages out-of-date](https://wiki.archlinux.org/title/Arch_User_Repository#Flagging_packages_out-of-date).

In the meantime, you can try updating the package yourself by editing the PKGBUILD locally. Sometimes, updates do not require changes to the build or package process, in which case simply updating the pkgver or source array is sufficient.

### Foo in the AUR does not compile when I run makepkg; what should I do?

You are probably missing something trivial; see [#Debugging the package build process](https://wiki.archlinux.org/title/Arch_User_Repository#Debugging_the_package_build_process).

### ERROR: One or more PGP signatures could not be verified!; what should I do?

Most likely, you do not have the required public key(s) in your personal keyring to verify downloaded files. See [Makepkg#Signature checking](https://wiki.archlinux.org/title/Makepkg#Signature_checking) for details.

### How do I create a PKGBUILD?

Consult the [AUR submission guidelines#Rules of submission](https://wiki.archlinux.org/title/AUR_submission_guidelines#Rules_of_submission), then see [creating packages](https://wiki.archlinux.org/title/Creating_packages).

### I have a PKGBUILD I would like to submit; can someone check it to see if there are any errors?

There are several channels available to submit your package for review; see [#Debugging the package build process](https://wiki.archlinux.org/title/Arch_User_Repository#Debugging_the_package_build_process).

### How to get a PKGBUILD into the extra repository?

Usually, at least 10 votes are required for something to move into [extra](https://wiki.archlinux.org/title/Extra_repository). However, if a [Package Maintainer](https://wiki.archlinux.org/title/Package_Maintainer) wants to support a package, it will often be found in the repository.

Reaching the required minimum of votes is not the only requirement; there has to be a package maintainer willing to maintain the package. Package Maintainers are not required to move a package into the extra repository even if it has thousands of votes.

Usually, when a very popular package stays in the AUR, it is because:

- Arch Linux already has another version of a package in the repositories
- Its license prohibits redistribution
- It helps retrieve user-submitted PKGBUILDs. [AUR helpers](https://wiki.archlinux.org/title/AUR_helpers) are [unsupported](https://bbs.archlinux.org/viewtopic.php?pid=828310#p828310) by definition.

See also [Rules for Packages Entering the extra repository](https://wiki.archlinux.org/title/AUR_Package_Maintainer_guidelines#Rules_for_packages_entering_the_extra_repository)

### How can I speed up repeated build processes?

See [Makepkg#Improving build times](https://wiki.archlinux.org/title/Makepkg#Improving_build_times).

### What is the difference between foo and foo-git packages?

Many AUR packages come in "stable" release and "unstable" development versions. Development packages usually have a [suffix](https://wiki.archlinux.org/title/VCS_package_guidelines#Package_naming) denoting their [Version Control System](https://wiki.archlinux.org/title/Version_Control_System) and are not intended for regular use, but may offer new features or bugfixes. Because these packages only download the latest available source when you execute makepkg, their pkgver() in the AUR does not reflect upstream changes. Likewise, these packages cannot perform an authenticity checksum on any [VCS source](https://wiki.archlinux.org/title/VCS_package_guidelines#VCS_sources).

See also [System maintenance#Use proven software packages](https://wiki.archlinux.org/title/System_maintenance#Use_proven_software_packages).

### Why has foo disappeared from the AUR?

It is possible the package has been adopted by a [Package Maintainer](https://wiki.archlinux.org/title/Package_Maintainer) and is now in the [extra repository](https://wiki.archlinux.org/title/Extra_repository).

Packages may be deleted if they did not fulfill the [rules of submission](https://wiki.archlinux.org/title/AUR_submission_guidelines#Rules_of_submission). See the [aur-requests archives](https://lists.archlinux.org/archives/list/aur-requests@lists.archlinux.org/) for the reason for deletion.

Note The git repository for a deleted package typically remains available. See [AUR submission guidelines#Requests](https://wiki.archlinux.org/title/AUR_submission_guidelines#Requests) for details.

### How do I find out if any of my installed packages disappeared from AUR?

Compare the output of pacman -Qm (installed packages not found in a sync database) against the list of all AUR packages:

```
$ comm -23 <(pacman -Qqm | sort) <(curl https://aur.archlinux.org/packages.gz | gzip -cd | sort)

```

### How can I obtain a list of all AUR packages?

- https://aur.archlinux.org/packages.gz
- Use aurpkglist from [python3-aur](https://aur.archlinux.org/packages/python3-aur/)AUR
- [New AUR Metadata Archives](https://lists.archlinux.org/archives/list/aur-general@lists.archlinux.org/thread/D4YC6Y7L4T5VSEONUCLHOX2R4NJKNIDP/)

## See also

- [AUR Web Interface](https://aur.archlinux.org)
- [AUR Mailing List](https://lists.archlinux.org/mailman3/lists/aur-general.lists.archlinux.org/)

Retrieved from "https://wiki.archlinux.org/index.php?title=Arch_User_Repository&oldid=886223"

[Categories](https://wiki.archlinux.org/title/Special:Categories):

- [About Arch](https://wiki.archlinux.org/title/Category:About_Arch)
- [Package development](https://wiki.archlinux.org/title/Category:Package_development)
- [Package management](https://wiki.archlinux.org/title/Category:Package_management)

Hidden category:

- [Pages or sections flagged with Template:Expansion](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Expansion)
