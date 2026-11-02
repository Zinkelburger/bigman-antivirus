---
url: "https://github.com/netblue30/firejail"
title: "GitHub - netblue30/firejail: Linux namespaces and seccomp-bpf sandbox"
site: "GitHub"
captured: "2026-11-02T19:06:17Z"
via: "live"
sha256: "d3853dfe127d782a9f015867dcfece92029f5e41a7d584b2242c67c03a16eafe"
wayback: "https://web.archive.org/web/20260918105541/https://github.com/netblue30/firejail"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# GitHub - netblue30/firejail: Linux namespaces and seccomp-bpf sandbox

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/netblue30/firejail>
> Snapshot: <https://web.archive.org/web/20260918105541/https://github.com/netblue30/firejail>

# Firejail

https://github.com/netblue30/firejail#firejail

https://gitlab.com/Firejail/firejail_ci/pipelines https://github.com/netblue30/firejail/actions?query=workflow%3ABuild https://github.com/netblue30/firejail/actions?query=workflow%3ABuild-extra https://github.com/netblue30/firejail/actions?query=workflow%3ATest https://github.com/netblue30/firejail/actions?query=workflow%3ACheck-C https://github.com/netblue30/firejail/actions?query=workflow%3ACheck-Profiles https://github.com/netblue30/firejail/actions?query=workflow%3ACheck-Python https://github.com/netblue30/firejail/actions?query=workflow%3ACodespell https://repology.org/project/firejail/versions

Firejail is a lightweight security tool intended to protect a Linux system by setting up a restricted environment for running (potentially untrusted) applications.

More specifically, it is an SUID sandbox program that reduces the risk of security breaches by using Linux namespaces, seccomp-bpf and Linux capabilities. It allows a process and all its descendants to have their own private view of the globally shared kernel resources, such as the network stack, process table and mount table. Firejail can work in an SELinux or AppArmor environment, and it is integrated with Linux Control Groups.

Written in C with virtually no dependencies, the software runs on any Linux computer with a 3.x kernel version or newer. It can sandbox any type of processes: servers, graphical applications, and even user login sessions. The software includes sandbox profiles for a number of more common Linux programs, such as Mozilla Firefox, Chromium, VLC, Transmission etc.

The sandbox is lightweight, the overhead is low. There are no complicated configuration files to edit, no socket connections open, no daemons running in the background. All security features are implemented directly in Linux kernel and available on any Linux computer.

## Videos

https://github.com/netblue30/firejail#videos

|  [Quick Start](https://odysee.com/@netblue30:9/install)   |  [Advanced Browser Security](https://odysee.com/@netblue30:9/firefox)   |  [Tor Browser Security](https://odysee.com/@netblue30:9/tor)

## Links

https://github.com/netblue30/firejail#links

- Project webpage: https://firejail.wordpress.com/
- IRC: https://web.libera.chat/#firejail
- Download and Installation: https://firejail.wordpress.com/download-2/
- Features: https://firejail.wordpress.com/features-3/
- Documentation: https://firejail.wordpress.com/documentation-2/
- FAQ: https://github.com/netblue30/firejail/wiki/Frequently-Asked-Questions
- Wiki: https://github.com/netblue30/firejail/wiki
- GitHub Actions: https://github.com/netblue30/firejail/actions
- GitLab CI: https://gitlab.com/Firejail/firejail_ci/pipelines
- Debian CI: https://salsa.debian.org/reiner/firejail
- Debian Package Tracker: https://tracker.debian.org/pkg/firejail
- Ubuntu PPA: https://launchpad.net/~deki/+archive/ubuntu/firejail
- Video Channel: https://odysee.com/@netblue30:9?order=new
- Backup Video Channel: https://www.bitchute.com/profile/JSBsA1aoQVfW/

## Security vulnerabilities

https://github.com/netblue30/firejail#security-vulnerabilities

See [SECURITY.md](https://github.com/netblue30/firejail/blob/master/SECURITY.md).

## Installing

https://github.com/netblue30/firejail#installing

For the supported versions, see [SECURITY.md](https://github.com/netblue30/firejail/blob/master/SECURITY.md).

### Debian

https://github.com/netblue30/firejail#debian

Note: The versions from Debian stable and backports are likely to be outdated, so currently we recommend either downloading and installing the .deb package from the latest release:

- https://github.com/netblue30/firejail/releases/latest

Or [building from source](https://github.com/netblue30/firejail#building).

Old instructions

Debian stable (bullseye): We recommend to use the [backports](https://packages.debian.org/bullseye-backports/firejail) package.

### Ubuntu

https://github.com/netblue30/firejail#ubuntu

Note: The versions from the distribution and PPA are likely to be outdated, so currently we recommend either downloading and installing the .deb package from the latest release:

- https://github.com/netblue30/firejail/releases/latest

Or [building from source](https://github.com/netblue30/firejail#building).

Old instructions

Note: The PPA recommendation is mainly for firejail itself; it should be fine to install firetools and firejail-related tools directly from the distribution if they are not in the PPA as they tend to be updated less frequently.

For Ubuntu 18.04+ and derivatives (such as Linux Mint), users are strongly advised to use the [PPA](https://launchpad.net/~deki/+archive/ubuntu/firejail).

How to add and install from the PPA:

```
sudo add-apt-repository ppa:deki/firejail
sudo apt-get update
sudo apt-get install firejail firejail-profiles
```

Reason: The firejail package for Ubuntu 20.04 has been left vulnerable to CVE-2021-26910 for months after a patch for it was posted on Launchpad:

- [CVE-2021-26910](https://github.com/advisories/GHSA-2q4h-h5jp-942w)
- [firejail version in Ubuntu 20.04 LTS is vulnerable to CVE-2021-26910](https://bugs.launchpad.net/ubuntu/+source/firejail/+bug/1916767)

See also https://wiki.ubuntu.com/SecurityTeam/FAQ:

What software is supported by the Ubuntu Security team?

Ubuntu is currently divided into four components: main, restricted, universe and multiverse. All binary packages in main and restricted are supported by the Ubuntu Security team for the life of an Ubuntu release, while binary packages in universe and multiverse are supported by the Ubuntu community.

Additionally, the PPA version is likely to be more recent and to contain more profile fixes.

See the following discussions for details:

- [Should I keep using the version of firejail available in my distro repos?](https://github.com/netblue30/firejail/discussions/4666)
- [How to install the latest version on Ubuntu and derivatives](https://github.com/netblue30/firejail/discussions/4663)

### Other

https://github.com/netblue30/firejail#other

Firejail is available in multiple Linux distributions:

Repology

https://repology.org/project/firejail/versions

Other than the [aforementioned exceptions](https://github.com/netblue30/firejail#installing), as long as your distribution provides a [supported version](https://github.com/netblue30/firejail/blob/master/SECURITY.md) of firejail, it's generally a good idea to install it from the distribution.

The version can be checked with firejail --version after installing.

You can also install one of the [released packages](https://github.com/netblue30/firejail/releases).

## Building

https://github.com/netblue30/firejail#building

Instructions for building and installing the current development version from source.

It is recommended to use the distribution-specific instructions below when applicable, as they generate native packages, which should make integration with the rest of the system and package management easier.

For other distributions, see the [generic instructions](https://github.com/netblue30/firejail#generic).

See also [.gitlab-ci.yml](https://github.com/netblue30/firejail/blob/master/.gitlab-ci.yml) for how distribution-specific builds are done in CI.

### Arch

https://github.com/netblue30/firejail#arch

Note: The AUR package is not maintained by us.

Install [firejail-git](https://aur.archlinux.org/packages/firejail-git) from the AUR.

Example:

```
yay -S firejail-git
```

### Debian

https://github.com/netblue30/firejail#debian-1

For Debian, Ubuntu and derivatives:

Note: Any ./configure flags should be passed to [mkdeb.sh](https://github.com/netblue30/firejail/blob/master/mkdeb.sh) (such as ./mkdeb.sh --enable-foo).

```
sudo apt-get update -qy
sudo apt-get install -qy \
  git build-essential fakeroot lintian libapparmor-dev pkg-config gawk
git clone 'https://github.com/netblue30/firejail.git' &&
cd firejail &&
./configure &&
make dist &&
./mkdeb.sh &&
sudo dpkg -i ./*.deb &&
make installcheck
```

### Fedora

https://github.com/netblue30/firejail#fedora

For Fedora and derivatives:

Note: Any ./configure flags should be passed to [mkrpm.sh](https://github.com/netblue30/firejail/blob/master/platform/rpm/mkrpm.sh) (such as ./platform/rpm/mkrpm.sh --enable-foo).

```
sudo dnf update -y
sudo dnf install -y rpm-build gcc make
git clone 'https://github.com/netblue30/firejail.git' &&
cd firejail &&
./configure &&
make dist &&
./platform/rpm/mkrpm.sh &&
sudo rpm -i ./*.rpm &&
make installcheck
```

### Generic

https://github.com/netblue30/firejail#generic

Generic build/install instructions:

```
git clone 'https://github.com/netblue30/firejail.git' &&
cd firejail &&
./configure &&
make -j "$(nproc)" &&
sudo make install-strip &&
make installcheck
```

On Debian/Ubuntu you will need to install git and gcc.

To build with AppArmor support (which is usually used on Debian, Ubuntu, openSUSE and derivatives), install the AppArmor development libraries and pkg-config and use the --enable-apparmor ./configure option:

```
sudo apt-get install git build-essential libapparmor-dev pkg-config gawk
```

To build with SELinux support (which is usually used on Fedora, RHEL and derivatives), install libselinux1-dev (libselinux-devel on Fedora) and use the --enable-selinux ./configure option.

Detailed information on using firejail from git is available on the [wiki](https://github.com/netblue30/firejail/wiki/Using-firejail-from-git).

## Running the sandbox

https://github.com/netblue30/firejail#running-the-sandbox

Note: Make sure to use the full path to the program (for example, firejail /usr/bin/firefox instead of firejail firefox) to avoid potential firejail-in-firejail issues (see [#2877](https://github.com/netblue30/firejail/issues/2877)).

To start the sandbox, prefix your command with firejail:

```
firejail /usr/bin/firefox            # starting Mozilla Firefox
firejail /usr/bin/transmission-gtk   # starting Transmission BitTorrent
firejail /usr/bin/vlc                # starting VideoLAN Client
sudo firejail /etc/init.d/nginx start
```

Run firejail --list in a terminal to list all active sandboxes. Example:

```
$ firejail --list
1617:netblue:/usr/bin/firejail /usr/bin/firefox-esr
7719:netblue:/usr/bin/firejail /usr/bin/transmission-qt
7779:netblue:/usr/bin/firejail /usr/bin/galculator
7874:netblue:/usr/bin/firejail /usr/bin/vlc --started-from-file file:///home/netblue/firejail-whitelist.mp4
7916:netblue:firejail --list
```

## Desktop integration

https://github.com/netblue30/firejail#desktop-integration

Integrate your sandbox into your desktop by running the following two commands:

```
firecfg --fix-sound
sudo firecfg
```

The first command solves some shared memory/PID namespace bugs in PulseAudio software prior to version 9. The second command integrates Firejail into your desktop. You would need to logout and login back to apply PulseAudio changes.

Start your programs the way you are used to: desktop manager menus, file manager, desktop launchers.

The integration applies to any program supported by default by Firejail. There are over 900 default applications in the current Firejail version, and the number goes up with every new release.

We keep the application list in [src/firecfg/firecfg.config](https://github.com/netblue30/firejail/blob/master/src/firecfg/firecfg.config) (/etc/firejail/firecfg.config when installed).

## Security profiles

https://github.com/netblue30/firejail#security-profiles

Most Firejail command line options can be passed to the sandbox using profile files.

You can find the profiles for all supported applications in [etc/](https://github.com/netblue30/firejail/blob/master/etc) (/etc/firejail/ when installed).

We also keep a list of profile fixes for previous released versions in [etc-fixes/](https://github.com/netblue30/firejail/blob/master/etc-fixes).

If you keep additional Firejail security profiles in a public repository, please give us a link:

- https://github.com/chiraag-nataraj/firejail-profiles
- https://github.com/triceratops1/fe

Use this issue to request new profiles:

- [Profile requests](https://github.com/netblue30/firejail/issues/1139)

You can also use this tool to get a list of syscalls needed by a program:

- [contrib/syscalls.sh](https://github.com/netblue30/firejail/blob/master/contrib/syscalls.sh)

## Uninstalling

https://github.com/netblue30/firejail#uninstalling

firecfg creates symlinks in /usr/local/bin, so to fully remove firejail, run the following before uninstalling:

```
sudo firecfg --clean
```

See man firecfg for details.

Note: Broken symlinks are ignored when searching for an executable in $PATH, so uninstalling without doing the above should not cause issues.

## Latest released version: 0.9.80

https://github.com/netblue30/firejail#latest-released-version-0980

## Current development version: 0.9.81

https://github.com/netblue30/firejail#current-development-version-0981

### X11 support restructuring

https://github.com/netblue30/firejail#x11-support-restructuring

With the transition from X11 to Wayland for what feels like forever, some restructuring is in order:

- --x11=xpra support was removed.
- --x11=xorg support (Xorg security extension) is disabled in the default software configuration, the user will need to add --enable-x11-legacy flag for ./configure (./configure --enable-x11-legacy)
- --x11=xvfb support (xvfb display server) is disabled in the default software configuration, the user will need to add --enable-x11-legacy flag for ./configure (./configure --enable-x11-legacy)
- --x11=xephyr command is removed, however, Xephyr display server is used by default by --x11 command.

### Landlock support - ongoing/experimental

https://github.com/netblue30/firejail#landlock-support---ongoingexperimental

- Added on #6078, which is based on #5315 from ChrysoliteAzalea/landlock
- Compile-time detection based on linux/landlock.h - if the header is found, the feature is compiled in
- Runtime detection based on whether Landlock is supported by the kernel and is enabled on the system

```
LANDLOCK
       Landlock is a Linux security module first introduced in version 5.13 of
       the  Linux  kernel.  It allows unprivileged processes to restrict their
       access to the filesystem.  Once imposed, these restrictions  can  never
       be  removed,  and  all child processes created by a Landlock-restricted
       processes inherit these restrictions.  Firejail supports Landlock as an
       additional  sandboxing  feature.  It can be used to ensure that a sand‐
       boxed application can only access files and directories that it was ex‐
       plicitly  allowed  to access.  Firejail supports populating the ruleset
       with both a basic set of rules (see --landlock) and with a  custom  set
       of rules.

       Important notes:

              - A process can install a Landlock ruleset only if it has either
              CAP_SYS_ADMIN in its effective capability set, or  the  "No  New
              Privileges"  restriction enabled.  Because of this, enabling the
              Landlock feature will also cause Firejail to enable the "No  New
              Privileges"  restriction,  regardless  of  the  profile  or  the
              --nonewprivs command line option.

              - Access to the /proc directory is managed through  the  --land‐
              lock.proc command line option.

              -  Access  to  the  /etc directory is automatically allowed.  To
              override this, use the --writable-etc command line option.   You
              can  also use the --private-etc option to restrict access to the
              /etc directory.

       To enable Landlock self-restriction on top of your current Firejail se‐
       curity  features,  pass  --landlock flag to Firejail command line.  You
       can also use --landlock.read, --landlock.write, --landlock.special  and
       --landlock.execute  options  together with --landlock or instead of it.
       Example:

       $ firejail --landlock --landlock.read=/media --landlock.proc=ro /usr/bin/mc

```

### Profile Statistics

https://github.com/netblue30/firejail#profile-statistics

A small tool to print profile statistics. Compile and install as usual. The tool is installed in the /usr/lib/firejail directory.

Run it over the profiles in /etc/profiles:

```
$ /usr/lib/firejail/profstats /etc/firejail/*.profile
No include .local found in /etc/firejail/noprofile.profile
Warning: multiple caps in /etc/firejail/tidal-hifi.profile
Warning: multiple caps in /etc/firejail/tqemu.profile
Warning: multiple caps in /etc/firejail/transmission-daemon.profile
Warning: multiple caps in /etc/firejail/trivalent.profile

Stats:
    profiles			1350
    include local profile	1349   (include profile-name.local)
    include globals		1314   (include globals.local)
    blacklist ~/.ssh		1209   (include disable-common.inc)
    seccomp			1221
    capabilities		1343
    noexec			1217   (include disable-exec.inc)
    noroot			1113
    memory-deny-write-execute	320
    restrict-namespaces		1055
    apparmor			873
    private-bin			816
    private-dev			1179
    private-etc			850
    private-cache		937
    private-tmp			1044
    whitelist home directory	663
    whitelist var		982   (include whitelist-var-common.inc)
    whitelist run/user		1313   (include whitelist-runuser-common.inc
					or blacklist ${RUNUSER})
    whitelist usr/share		762   (include whitelist-usr-share-common.inc
    net none			454
    dbus-user none 		770
    dbus-user filter 		210
    dbus-system none 		978
    dbus-system filter 		13

```
