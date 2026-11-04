---
url: "https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html"
title: "Chapter 5. Issues to be aware of for bookworm"
captured: "2026-11-03T03:06:57Z"
via: "live"
sha256: "05ae042676008accccdd39fdb6f91c49e621ebcd9ee95df4737bfe443638a50e"
wayback: "https://web.archive.org/web/20260908062031/https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html"
cited_in:
  - "docs/research/hardening.md"
---

# Chapter 5. Issues to be aware of for bookworm

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html>
> Snapshot: <https://web.archive.org/web/20260908062031/https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html>

| Chapter 5. Issues to be aware of for bookworm

| https://www.debian.org/releases/bookworm/amd64/release-notes/ch-upgrading.en.html  |   |  https://www.debian.org/releases/bookworm/amd64/release-notes/ch-moreinfo.en.html

# Chapter 5. Issues to be aware of for bookworm

Table of Contents

[5.1. Upgrade specific items for bookworm](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#upgrade-specific-issues)

[5.1.1. Non-free firmware moved to its own component in the archive](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#non-free-split)

[5.1.2. Changes to packages that set the system clock](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#changes-to-packages-that-set-the-system-clock)

[5.1.3. Puppet configuration management system upgraded to 7](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#puppetserver)

[5.1.4. youtube-dl replaced with yt-dlp](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#youtube-dl)

[5.1.5. Fcitx versions no longer co-installable](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#fcitx-fcitx5-bookworm-conflict)

[5.1.6. MariaDB package names no longer include version numbers](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#mariadb-versionless-package-names)

[5.1.7. Changes to system logging](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#changes-to-system-logging)

[5.1.8. rsyslog changes affecting log analyzers such as logcheck](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#rsyslog-timestamp-change-affects-logcheck)

[5.1.9. rsyslog creates fewer log files](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#rsyslog-creates-fewer-logfiles)

[5.1.10. slapd upgrade may require manual intervention](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#openldap-2.5)

[5.1.11. GRUB no longer runs os-prober by default](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#grub-os-prober)

[5.1.12. GNOME has reduced accessibility support for screen readers](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#gnome-has-reduced-accessibility-support-for-screen-readers)

[5.1.13. Changes to polkit configuration](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#changes-to-polkit-configuration)

[5.1.14. A “merged-/usr” is now required](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#a-merged-usr-is-now-required)

[5.1.15. Unsupported upgrades from buster fail on libcrypt1](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#libcrypt-upgrade-from-buster)

[5.1.16. Things to do post upgrade before rebooting](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#before-first-reboot)

[5.2. Items not limited to the upgrade process](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#not-upgrade-only)

[5.2.1. Limitations in security support](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#limited-security-support)

[5.2.2. Python Interpreters marked externally-managed](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#python3-pep-668)

[5.2.3. systemd-resolved has been split into a separate package](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#systemd-resolved)

[5.2.4. systemd-boot has been split into a separate package](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#systemd-boot)

[5.2.5. systemd-journal-remote no longer uses GnuTLS](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#systemd-journal-remote)

[5.2.6. Extensive changes in adduser for bookworm](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#adduser-changes)

[5.2.7. Predictable naming for Xen network interfaces](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#xen-network)

[5.2.8. Change in dash handling of circumflex](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#dash-circumflex)

[5.2.9. netcat-openbsd supports abstract sockets](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#netcat-openbsd-now-supports-abstract-sockets)

[5.3. Obsolescence and deprecation](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#obsolescense-and-deprecation)

[5.3.1. Noteworthy obsolete packages](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#noteworthy-obsolete-packages)

[5.3.2. Deprecated components for bookworm](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#deprecated-components)

[5.4. Known severe bugs](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#rc-bugs)

Sometimes, changes introduced in a new release have side-effects we cannot reasonably avoid, or they expose bugs somewhere else. This section documents issues we are aware of. Please also read the errata, the relevant packages' documentation, bug reports, and other information mentioned in [Section 6.1, “Further reading”](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-moreinfo.en.html#morereading).

## 5.1. Upgrade specific items for bookworm

This section covers items related to the upgrade from bullseye to bookworm.

### 5.1.1. Non-free firmware moved to its own component in the archive

As described in [Section 2.2, “Archive areas”](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-whats-new.en.html#archive-areas), non-free firmware packages are now served from a dedicated archive component, called non-free-firmware. To ensure installed non-free firmware packages receive proper upgrades, changes to the APT configuration are required. Assuming the non-free component was only added to the APT sources-list to install firmware, the updated APT source-list entry could look like:

```
deb https://deb.debian.org/debian bookworm main non-free-firmware
```

If you were pointed to this chapter by apt you can prevent it from continuously notifying you about this change by creating an [apt.conf(5)](https://manpages.debian.org//bookworm/apt/apt.conf.5.html) file named /etc/apt/apt.conf.d/no-bookworm-firmware.conf with the following content:

```
APT::Get::Update::SourceListWarnings::NonFreeFirmware "false";
```

### 5.1.2. Changes to packages that set the system clock

The ntp package, which used to be the default way to set the system clock from a Network Time Protocol (NTP) server, has been replaced by ntpsec.

Most users will not need to take any specific action to transition from ntp to ntpsec.

In bookworm there are also several other packages that provide a similar service. The Debian default is now systemd-timesyncd, which may be adequate for users who only need an ntp client to set their clock. bookworm also includes chrony and openntpd which support more advanced features, such as operating your own NTP server.

### 5.1.3. Puppet configuration management system upgraded to 7

Puppet has been upgraded from 5 to 7, skipping the Puppet 6 series altogether. This introduces major changes to the Puppet ecosystem.

The classic Ruby-based Puppet Master 5.5.x application has been deprecated upstream and is no longer available in Debian. It is replaced by Puppet Server 7.x, provided by the puppetserver package. The package is automatically installed as a dependency of the transitional puppet-master package.

In some cases, Puppet Server is a drop-in replacement for Puppet Master, but you should review the configuration files available under /etc/puppet/puppetserver to ensure the new defaults are suitable for your deployment. In particular the legacy format for the auth.conf file is deprecated, see the [auth.conf documentation](https://www.puppet.com/docs/puppet/7/server/config_file_auth.html) for details.

The recommended approach is to upgrade the server before clients. The Puppet 7 Server is [backwards compatible with older clients](https://www.puppet.com/docs/puppet/7/server/compatibility_with_puppet_agent.html); a Puppet 5 Server can still handle upgraded agents but cannot register new Puppet 7 agents. So if you deploy new Puppet 7 agents before upgrading the server, you will not be able to add them to the fleet.

The puppet package has been replaced by the puppet-agent package and is now a transitional package to ensure a smooth upgrade.

Finally, the puppetdb package was removed in bullseye but is reintroduced in bookworm.

### 5.1.4. youtube-dl replaced with yt-dlp

The popular tool youtube-dl, which can download videos from a large variety of websites (including, but not limited to, YouTube) is no longer included in Debian. Instead, it has been replaced with an empty transitional package that pulls in the yt-dlp package instead. yt-dlp is a fork of youtube-dl where new development is currently happening.

There are no compatibility wrappers provided, so you'll need to modify your scripts and personal behavior to call yt-dlp instead of youtube-dl. The functionality should be mostly the same, although some options and behavioral details have changed. Be sure to check yt-dlp's [man page](https://manpages.debian.org//bookworm/yt-dlp/yt-dlp.1.html) for details, and in particular the [Differences in default behavior](https://manpages.debian.org/bookworm/yt-dlp/yt-dlp.1.html#Differences_in_default_behavior) section.

### 5.1.5. Fcitx versions no longer co-installable

The packages fcitx and fcitx5 provide version 4 and version 5 of the popular Fcitx Input Method Framework. Following upstream's recommendation, they can no longer be co-installed on the same operating system. Users should determine which version of Fcitx is to be kept if they had co-installed fcitx and fcitx5 previously.

Before the upgrade, users are strongly encouraged to purge all related packages for the unwanted Fcitx version (fcitx-* for Fcitx 4, and fcitx5-* for Fcitx 5). When the upgrade is finished, consider executing the im-config again to select the desired input method framework to be used in the system.

You can read more background information in [the announcement posted in the mailing list](https://lists.debian.org/debian-chinese-gb/2021/12/msg00000.html) (text written in Simplified Chinese).

### 5.1.6. MariaDB package names no longer include version numbers

Unlike bullseye that had the MariaDB version in package names (e.g. mariadb-server-10.5 and mariadb-client-10.5), in bookworm the equivalent MariaDB 10.11 package names are fully versionless (e.g. mariadb-server or mariadb-client). The MariaDB version is still visible in the package version metadata.

There is at least one known upgrade scenario [(Bug #1035949)](https://bugs.debian.org/1035949) where the transition to versionless package names fails: running

```
apt-get install default-mysql-server
```

may fail when mariadb-client-10.5 and the file /usr/bin/mariadb-admin in it is removed before the MariaDB server SysV init service has issued a shutdown, which uses mariadb-admin. The workaround is to run

```
apt upgrade
```

before running

```
apt full-upgrade
```

.

For more information about the package name changes in MariaDB, see [/usr/share/doc/mariadb-server/NEWS.Debian.gz](https://salsa.debian.org/mariadb-team/mariadb-server/-/blob/280369f034bedae714af2f26111d1d9d1659f142/debian/mariadb-server.NEWS).

### 5.1.7. Changes to system logging

The rsyslog package is no longer needed on most systems and you may be able to remove it.

Many programs produce log messages to inform the user of what they are doing. These messages can be managed by systemd's “journal” or by a “syslog daemon” such as rsyslog.

In bullseye, rsyslog was installed by default and the systemd journal was configured to forward log messages to rsyslog, which writes messages into various text files such as /var/log/syslog.

From bookworm, rsyslog is no longer installed by default. If you do not want to continue using rsyslog, after the upgrade you can mark it as automatically installed with

```
apt-mark auto rsyslog
```

and then an

```
apt autoremove
```

will remove it, if possible. If you have upgraded from older Debian releases, and not accepted the default configuration settings, the journal may not have been configured to save messages to persistent storage: instructions for enabling this are in [journald.conf(5)](https://manpages.debian.org//bookworm/systemd/journald.conf.5.html).

If you decide to switch away from rsyslog you can use the journalctl command to read log messages, which are stored in a binary format under /var/log/journal. For example,

```
journalctl -e
```

shows the most recent log messages in the journal and

```
journalctl -ef
```

shows new messages as they are written (similar to running

```
tail -f /var/log/syslog
```

).

### 5.1.8. rsyslog changes affecting log analyzers such as logcheck

rsyslog now defaults to “high precision timestamps” which may affect other programs that analyze the system logs. There is further information about how to customize this setting in [rsyslog.conf(5)](https://manpages.debian.org//bookworm/rsyslog/rsyslog.conf.5.html).

The change in timestamps may require locally-created logcheck rules to be updated. logcheck checks messages in the system log (produced by systemd-journald or rsyslog) against a customizable database of regular expressions known as rules. Rules that match the time the message was produced will need to be updated to match the new rsyslog format. The default rules, which are provided by the logcheck-database package, have been updated, but other rules, including those created locally, may require updating to recognize the new format. See [/usr/share/doc/logcheck-database/NEWS.Debian.gz](https://salsa.debian.org/debian/logcheck/-/blob/debian/1.4.0/debian/logcheck-database.NEWS) for a script to help update local logcheck rules.

### 5.1.9. rsyslog creates fewer log files

rsyslog has changed which log files it creates, and some files in /var/log can be deleted.

If you are continuing to use rsyslog (see [Section 5.1.7, “Changes to system logging”](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html#changes-to-system-logging)), some log files in /var/log will no longer be created by default. The messages that were written to these files are also in /var/log/syslog but are no longer created by default. Everything that used to be written to these files will still be available in /var/log/syslog.

The files that are no longer created are:

-

/var/log/mail.{info,warn,err}

These files contained messages from the local mail transport agent (MTA), split up by priority.

As /var/log/mail.log contains all mail related messages, these files (and their rotated counterparts) can be deleted safely. If you were using those files to monitor anomalies, a suitable alternative might be something like logcheck.

-

/var/log/lpr.log

This file contained log messages relating to printing. The default print system in debian is cups which does not use this file, so unless you installed a different printing system this file (and its rotated counterparts) can be deleted.

-

/var/log/{messages,debug,daemon.log}

These files (and their rotated counterparts) can be deleted. Everything that used to be written to these files will still be in /var/log/syslog.

### 5.1.10. slapd upgrade may require manual intervention

OpenLDAP 2.5 is a major new release and includes several incompatible changes as described in [the upstream release announcement](https://git.openldap.org/openldap/openldap/-/raw/OPENLDAP_REL_ENG_2_5/ANNOUNCEMENT). Depending on the configuration, the slapd service might remain stopped after the upgrade, until necessary configuration updates are completed.

The following are some of the known incompatible changes:

-

The [slapd-bdb(5)](https://manpages.debian.org//bullseye/slapd/slapd-bdb.5.html) and [slapd-hdb(5)](https://manpages.debian.org//bullseye/slapd/slapd-hdb.5.html) database backends have been removed. If you are using one of these backends under bullseye, it is strongly recommended to migrate to the [slapd-mdb(5)](https://manpages.debian.org//bookworm/slapd/slapd-mdb.5.html) backend before upgrading to bookworm.

-

The [slapd-shell(5)](https://manpages.debian.org//bullseye/slapd/slapd-shell.5.html) database backend has been removed.

-

The [slapo-ppolicy(5)](https://manpages.debian.org//bookworm/slapd/slapo-ppolicy.5.html) overlay now includes its schema compiled into the module. The old external schema, if present, conflicts with the new built-in one.

-

The [pw-argon2](https://manpages.debian.org//bullseye/slapd-contrib/slapd-pw-argon2.5.html) contrib password module has been renamed to [argon2](https://manpages.debian.org//bookworm/slapd/slappw-argon2.5.html).

Instructions for completing the upgrade and resuming the slapd service can be found in [/usr/share/doc/slapd/README.Debian.gz](https://sources.debian.org/src/openldap/bookworm/debian/slapd.README.Debian/). You should also consult [the upstream upgrade notes](https://openldap.org/doc/admin25/appendix-upgrading.html).

### 5.1.11. GRUB no longer runs os-prober by default

For a long time, grub has used the os-prober package to detect other operating systems installed on a computer so that it can add them to the boot menu. Unfortunately, that can be problematic in certain cases (e.g. where guest virtual machines are running), so this has now been disabled by default in the latest upstream release.

If you are using GRUB to boot your system and want to continue to have other operating systems listed on the boot menu, you can change this. Either edit the file /etc/default/grub, ensure you have the setting GRUB_DISABLE_OS_PROBER=false and re-run update-grub, or run

```
dpkg-reconfigure <GRUB_PACKAGE>
```

to change this and other GRUB settings in a more user-friendly way.

### 5.1.12. GNOME has reduced accessibility support for screen readers

Many GNOME apps have switched from the GTK3 graphics toolkit to GTK4. Sadly, this has made many apps much less usable with screen readers such as orca.

If you depend on a screen reader you should consider switching to a different desktop such as [Mate](https://mate-desktop.org), which has better accessibility support. You can do this by installing the mate-desktop-environment package. Information about how to use Orca under Mate is available at [here](https://wiki.debian.org/Accessibility/Orca#MATE).

### 5.1.13. Changes to polkit configuration

For consistency with upstream and other distributions, the polkit (formerly PolicyKit) service, which allows unprivileged programs to access privileged system services, has changed the syntax and location for local policy rules. You should now write local rules for customizing the security policy in [JavaScript](https://wikipedia.org/wiki/JavaScript), and place them at /etc/polkit-1/rules.d/*.rules. Example rules using the new format can be found in /usr/share/doc/polkitd/examples/, and [polkit(8)](https://manpages.debian.org//unstable/polkitd/polkit.8.html#AUTHORIZATION_RULES) has further information.

Previously, rules could be written in pkla format, and placed in subdirectories of /etc/polkit-1/localauthority or /var/lib/polkit-1/localauthority. However, .pkla files should now be considered deprecated, and will only continue to work if the polkitd-pkla package is installed. This package will usually be installed automatically when you upgrade to bookworm, but it is likely not to be included in future Debian releases, so any local policy overrides will need to be migrated to the JavaScript format.

### 5.1.14. A “merged-/usr” is now required

Debian has adopted a filesystem layout, referred to as “merged-/usr”, which no longer includes the legacy directories /bin, /sbin, /lib, or optional variants such as /lib64. In the new layout, the legacy directories are replaced with symlinks to the corresponding locations /usr/bin, /usr/sbin, /usr/lib, and /usr/lib64. This means that, for example, both /bin/bash and /usr/bin/bash will launch bash.

For systems installed as buster or bullseye there will be no change, as the new filesystem layout was already the default in these releases. However, the older layout is no longer supported, and systems using it will be converted to the new layout when they are upgraded to bookworm.

The conversion to the new layout should have no impact on most users. All files are automatically moved to their new locations even if they were installed locally or come from packages not provided by Debian, and hardcoded paths such as /bin/sh continue to work. There are, however, some potential issues:

-

```
dpkg --search
```

will give wrong answers for files moved to the new locations:

```
dpkg --search /usr/bin/bash
```

will not identify that bash came from a package. (But

```
dpkg --search /bin/bash
```

still works as expected.)

-

Local software not provided by Debian may not support the new layout and may, for example, rely on /usr/bin/name and /bin/name being two different files. This is not supported on merged systems (including new installations since buster), so any such software must be fixed or removed before the upgrade.

-

Systems that rely on a “base layer” that is not directly writable (such as [WSL1](https://wiki.debian.org//https://wiki.debian.org/InstallingDebianOn/Microsoft/Windows/SubsystemForLinux) images or container systems using multi-layer [overlayfs](https://en.wikipedia.org/wiki/OverlayFS) filesystems) cannot be safely converted and should either be replaced (e.g., by installing a new WSL1 image from the store) or have each individual layer upgraded (e.g., by upgrading the base Debian layer of the overlayfs independently) rather than dist-upgraded.

For further information, see [The Case for the /usr merge](https://www.freedesktop.org/wiki/Software/systemd/TheCaseForTheUsrMerge/) and [the Debian Technical Committee resolution](https://bugs.debian.org/994388#110).

### 5.1.15. Unsupported upgrades from buster fail on libcrypt1

Debian officially supports upgrades only from one stable release to the next, e.g. from bullseye to bookworm. Upgrades from buster to bookworm are not supported, and will fail due to [Bug #993755](https://bugs.debian.org/993755) with the following error:

```

Setting up libc6:amd64 (2.36-9) ...
/usr/bin/perl: error while loading shared libraries: libcrypt.so.1: cannot open shared object file: No such file or directory
dpkg: error processing package libc6:amd64 (--configure):
installed libc6:amd64 package post-installation script subprocess returned error exit status 127

```

It is however possible to manually recover from this particular situation by forcibly installing the new libcrypt1:

```

# cd $(mktemp -d)
# apt download libcrypt1
# dpkg-deb -x libcrypt1_*.deb .
# cp -ra lib/* /lib/
# apt --fix-broken install

```

### 5.1.16. Things to do post upgrade before rebooting

When apt full-upgrade has finished, the “formal” upgrade is complete. For the upgrade to bookworm, there are no special actions needed before performing a reboot.

## 5.2. Items not limited to the upgrade process

### 5.2.1. Limitations in security support

There are some packages where Debian cannot promise to provide minimal backports for security issues. These are covered in the following subsections.

|  | Note

|

The package debian-security-support helps to track the security support status of installed packages.

#### 5.2.1.1. Security status of web browsers and their rendering engines

Debian 12 includes several browser engines which are affected by a steady stream of security vulnerabilities. The high rate of vulnerabilities and partial lack of upstream support in the form of long term branches make it very difficult to support these browsers and engines with backported security fixes. Additionally, library interdependencies make it extremely difficult to update to newer upstream releases. Applications using the webkit2gtk  source package (e.g. epiphany ) are covered by security support, but applications using qtwebkit (source package qtwebkit-opensource-src ) are not.

For general web browser use we recommend Firefox or Chromium. They will be kept up-to-date by rebuilding the current ESR releases for stable. The same strategy will be applied for Thunderbird.

Once a release becomes oldstable, officially supported browsers may not continue to receive updates for the standard period of coverage. For example, Chromium will only receive 6 months of security support in oldstable rather than the typical 12 months.

#### 5.2.1.2. Go- and Rust-based packages

The Debian infrastructure currently has problems with rebuilding packages of types that systematically use static linking. With the growth of the Go and Rust ecosystems it means that these packages will be covered by limited security support until the infrastructure is improved to deal with them maintainably.

In most cases if updates are warranted for Go or Rust development libraries, they will only be released via regular point releases.

### 5.2.2. Python Interpreters marked externally-managed

The Debian provided python3 interpreter packages (python3.11 and pypy3) are now marked as being externally-managed, following [PEP-668](https://peps.python.org/pep-0668/). The version of python3-pip provided in Debian follows this, and will refuse to manually install packages on Debian's python interpreters, unless the --break-system-packages option is specified.

If you need to install a Python application (or version) that isn't packaged in Debian, we recommend that you install it with pipx (in the pipx Debian package). pipx will set up an environment isolated from other applications and system Python modules, and install the application and its dependencies into that.

If you need to install a Python library module (or version) that isn't packaged in Debian, we recommend installing it into a virtualenv, where possible. You can create virtualenvs with the venv Python stdlib module (in the python3-venv Debian package) or the virtualenv Python 3rd-party tool (in the virtualenv Debian package). For example, instead of running pip install --user foo, run:  mkdir -p ~/.venvs && python3 -m venv ~/.venvs/foo && ~/.venvs/foo/bin/python -m pip install foo  to install it in a dedicated virtualenv.

See /usr/share/doc/python3.11/README.venv for more details.

### 5.2.3. systemd-resolved has been split into a separate package

The new systemd-resolved package will not be installed automatically on upgrades. If you were using the systemd-resolved system service, please install the new package manually after the upgrade, and note that until it has been installed, DNS resolution might no longer work since the service will not be present on the system. Installing this package will automatically give systemd-resolved control of /etc/resolv.conf. For more information about systemd-resolved, consult the official [documentation](https://www.freedesktop.org/software/systemd/man/systemd-resolved.service.html). Note that systemd-resolved was not, and still is not, the default DNS resolver in Debian. If you have not configured your machine to use systemd-resolved as the DNS resolver, no action is required.

### 5.2.4. systemd-boot has been split into a separate package

The new systemd-boot package will not be installed automatically on upgrades. If you were using systemd-boot, please install this new package manually, and note that until you do so, the older version of systemd-boot will be used as the bootloader. Installing this package will automatically configure systemd-boot as the machine's bootloader. The default boot loader in Debian is still GRUB. If you have not configured the machine to use systemd-boot as the bootloader, no action is required.

### 5.2.5. systemd-journal-remote no longer uses GnuTLS

The optional [systemd-journal-gatewayd](https://www.freedesktop.org/software/systemd/man/systemd-journal-remote.service.html#--trust=) and [systemd-journal-remote](https://www.freedesktop.org/software/systemd/man/systemd-journal-gatewayd.service.html#--trust=) services are now built without GnuTLS support, which means the --trust option is no longer provided by either program, and an error will be raised if it is specified.

### 5.2.6. Extensive changes in adduser for bookworm

There have been several changes in adduser. The most prominent change is that --disabled-password and --disabled-login are now functionally identical. For further details, please read the /usr/share/doc/adduser/NEWS.Debian.gz.

### 5.2.7. Predictable naming for Xen network interfaces

The predictable naming logic in systemd for network interfaces has been extended to generate stable names from Xen netfront device information. This means that instead of the former system of names assigned by the kernel, interfaces now have stable names of the form enX#. Please adapt your system before rebooting after the upgrade. Some more information can be found on the [NetworkInterfaceNames wiki page](https://wiki.debian.org/NetworkInterfaceNames#bookworm-xen).

### 5.2.8. Change in dash handling of circumflex

dash, which by default provides the system shell /bin/sh in Debian, has switched to treating the circumflex (^) as a literal character, as was always the intended POSIX-compliant behavior. This means that in bookworm [^0-9] no longer means “not 0 to 9” but “0 to 9 and ^”.

### 5.2.9. netcat-openbsd supports abstract sockets

The netcat utility for reading and writing data across network connections supports [abstract sockets](https://manpages.debian.org//bookworm/manpages/unix.7.html#Abstract_sockets), and uses them by default in some circumstances.

By default, netcat is provided by netcat-traditional. However, if netcat is provided by the netcat-openbsd package and you are using an AF_UNIX socket, then this new default applies. In this case the -U option to nc will now interpret an argument starting with an @ as requesting an abstract socket rather than as a filename beginning with an @ in the current directory. This can have security implications because filesystem permissions can no longer be used to control access to an abstract socket. You can continue to use a filename starting with an @ by prefixing the name with ./ or by specifying an absolute path.

## 5.3. Obsolescence and deprecation

### 5.3.1. Noteworthy obsolete packages

The following is a list of known and noteworthy obsolete packages (see [Section 4.8, “Obsolete packages”](https://www.debian.org/releases/bookworm/amd64/release-notes/ch-upgrading.en.html#obsolete) for a description).

The list of obsolete packages includes:

-

The libnss-ldap package has been removed from bookworm. Its functionalities are now covered by libnss-ldapd and libnss-sss.

-

The libpam-ldap package has been removed from bookworm. Its replacement is libpam-ldapd.

-

The fdflush package has been removed from bookworm. In its stead, please use blockdev --flushbufs from util-linux.

-

The libgdal-perl package has been removed from bookworm, because the Perl binding for GDAL is no longer supported upstream. If you need Perl support for GDAL, you can migrate to the FFI interface provided by the Geo::GDAL::FFI package, available on CPAN. You will have to build your own binaries as documented on the [BookwormGdalPerl Wiki page](https://wiki.debian.org/BookwormGdalPerl).

### 5.3.2. Deprecated components for bookworm

With the next release of Debian 13 (codenamed trixie) some features will be deprecated. Users will need to migrate to other alternatives to prevent trouble when updating to Debian 13.

This includes the following features:

-

Development of the NSS service gw_name stopped in 2015. The associated package libnss-gw-name may be removed in future Debian releases. The upstream developer suggests using libnss-myhostname instead.

-

dmraid has not seen upstream activity since end 2010 and has been on life support in Debian. bookworm will be the last release to ship it, so please plan accordingly if you're using dmraid.

-

request-tracker4 has been superseded by request-tracker5 in this release, and will be removed in future releases. We recommend that you plan to migrate from request-tracker4 to request-tracker5 during the lifetime of this release.

-

The isc-dhcp suite has been [deprecated](https://www.isc.org/blogs/isc-dhcp-eol/) by the [ISC](https://www.isc.org/). The [Debian Wiki](https://wiki.debian.org/) has a list of alternative implementations, see [DHCP Client](https://wiki.debian.org/DHCP_Client) and [DHCP Server](https://wiki.debian.org/DHCP_Server) pages for the latest. If you are using NetworkManager or systemd-networkd, you can safely remove the isc-dhcp-client package as they both ship their own implementation. If you are using the ifupdown package, you can experiment with udhcpc as a replacement. The ISC recommends the Kea package as a replacement for DHCP servers.

The security team will support the isc-dhcp package during the bookworm lifetime, but the package will likely be unsupported in the next stable release, see [bug #1035972 (isc-dhcp EOL'ed)](https://bugs.debian.org/1035972) for more details.

## 5.4. Known severe bugs

Although Debian releases when it's ready, that unfortunately doesn't mean there are no known bugs. As part of the release process all the bugs of severity serious or higher are actively tracked by the Release Team, so an [overview of those bugs](https://bugs.debian.org/cgi-bin/pkgreport.cgi?users=release.debian.org@packages.debian.org;tag=bookworm-can-defer) that were tagged to be ignored in the last part of releasing bookworm can be found in the [Debian Bug Tracking System](https://bugs.debian.org/). The following bugs were affecting bookworm at the time of the release and worth mentioning in this document:

| Bug number | Package (source or binary) | Description

| [1032240](https://bugs.debian.org/1032240) | akonadi-backend-mysql | akonadi server fails to start since it cannot connect to mysql database

| [918984](https://bugs.debian.org/918984) | src:fuse3 | provide upgrade path fuse -> fuse3 for bookworm

| [1016903](https://bugs.debian.org/1016903) | g++-12 | tree-vectorize: Wrong code at O2 level (-fno-tree-vectorize is working)

| [1020284](https://bugs.debian.org/1020284) | git-daemon-run | fails to purge: deluser -f: Unknown option: f

| [919296](https://bugs.debian.org/919296) | git-daemon-run | fails with 'warning: git-daemon: unable to open supervise/ok: file does not exist'

| [1034752](https://bugs.debian.org/1034752) | src:gluegen2 | embeds non-free headers

| [1036256](https://bugs.debian.org/1036256) | src:golang-github-pin-tftp | FTBFS in testing: dh_auto_test: error: cd _build && go test -vet=off -v -p 8 github.com/pin/tftp github.com/pin/tftp/netascii returned exit code 1

| [1036575](https://bugs.debian.org/1036575) | groonga-bin | missing Depends: libjs-jquery-flot, libjs-jquery-ui

| [1036041](https://bugs.debian.org/1036041) | src:grub2 | upgrade-reports: Dell XPS 9550 fails to boot after bullseye to bookworm upgrade - grub/bios interaction bug?

| [558422](https://bugs.debian.org/558422) | grub-pc | upgrade hangs

| [913916](https://bugs.debian.org/913916) | grub-efi-amd64 | UEFI boot option removed after update to grub2 2.02~beta3-5+deb9u1

| [924151](https://bugs.debian.org/924151) | grub2-common | wrong grub.cfg for efi boot and fully encrypted disk

| [925134](https://bugs.debian.org/925134) | grub-efi-amd64 | grub-efi-amd64-signed: doesn't mount cryptodisk

| [945001](https://bugs.debian.org/945001) | grub-efi-amd64 | GRUB-EFI messes up boot variables

| [965026](https://bugs.debian.org/965026) | grub-emu | grub-emu hangs linux console when run as root

| [984760](https://bugs.debian.org/984760) | grub-efi-amd64 | upgrade works, boot fails (error: symbol `grub_is_lockdown` not found)

| [1036263](https://bugs.debian.org/1036263) | src:guestfs-tools | FTBFS in testing: make[6]: *** [Makefile:1716: test-suite.log] Error 1

| [916596](https://bugs.debian.org/916596) | iptables | iptables.postinst failure on link creation

| [919058](https://bugs.debian.org/919058) | itstool | its-tools: crashes when freeing xmlDocs

| [1028416](https://bugs.debian.org/1028416) | kexec-tools | systemctl kexec doesn't shutdown system properly and corrupts mounted filesystems

| [935182](https://bugs.debian.org/935182) | libreoffice-core | Concurrent file open on the same host results file deletion

| [1036755](https://bugs.debian.org/1036755) | src:linux | 6.1.26 <= x < 6.1.30 breaks applications using mmap(MAP_32BIT) [affects ganeti]

| [1036580](https://bugs.debian.org/1036580) | src:llvm-defaults | please add some Breaks for smoother upgrades from bullseye

| [1036359](https://bugs.debian.org/1036359) | elpa-markdown-toc | crashes with (wrong-type-argument consp nil)

| [1032647](https://bugs.debian.org/1032647) | nvidia-driver | Intermittent black screen after updating to 525.89.02-1

| [1029342](https://bugs.debian.org/1029342) | openjdk-17-jre-headless | jexec: can't locate java: No such file or directory

| [1035798](https://bugs.debian.org/1035798) | libphp8.2-embed | does not ship SONAME link /usr/lib/libphp.so -> libphp8.2.so

| [1034993](https://bugs.debian.org/1034993) | software-properties-qt | missing Breaks+Replaces for software-properties-kde when upgrading from bullseye

| [1036388](https://bugs.debian.org/1036388) | sylpheed | account reset when mail is checked

| [1036424](https://bugs.debian.org/1036424) | sylpheed | replying to an email you sent doesn't set account accordingly

| [994274](https://bugs.debian.org/994274) | src:syslinux | FTBFS with gnu-efi 3.0.13

| [1031152](https://bugs.debian.org/1031152) | system-config-printer | unlock button in system-config-printer provides no elevated permissions dialog

| [975490](https://bugs.debian.org/975490) | u-boot-sunxi | A64-Olinuxino-eMMC boot stuck at "Starting kernel ..."

| [1034995](https://bugs.debian.org/1034995) | python-is-python3 | missing Breaks+Replaces for python-dev-is-python2 when upgrading from bullseye

| [1036881](https://bugs.debian.org/1036881) | whitedune | segfaults

| [1036601](https://bugs.debian.org/1036601) | xenstore-utils | missing Depends: xen-utils-common

| [1036578](https://bugs.debian.org/1036578) | python3-yade | does not ship a python module

| https://www.debian.org/releases/bookworm/amd64/release-notes/ch-upgrading.en.html  |   |  https://www.debian.org/releases/bookworm/amd64/release-notes/ch-moreinfo.en.html

| Chapter 4. Upgrades from Debian 11 (bullseye)  | https://www.debian.org/releases/bookworm/amd64/release-notes/index.en.html |  Chapter 6. More information on Debian
