---
url: "https://firejail.wordpress.com/"
title: "Firejail"
site: "Firejail"
captured: "2026-11-02T18:30:14Z"
via: "live"
sha256: "4e7d645e716067ad24c0325a5a69ef3d649842ce0d008725fbe7e5cc2f833b07"
wayback: "https://web.archive.org/web/20261024180520/https://firejail.wordpress.com/"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# Firejail

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://firejail.wordpress.com/>
> Snapshot: <https://web.archive.org/web/20261024180520/https://firejail.wordpress.com/>

# About

https://odysee.com/@netblue30:9/intro

[Introduction](https://odysee.com/@netblue30:9/intro)

Firejail is a SUID program that reduces the risk of security breaches by restricting the running environment of untrusted applications using [Linux namespaces](https://lwn.net/Articles/531114/) and [seccomp-bpf](https://l3net.wordpress.com/2015/04/13/firejail-seccomp-guide/). It allows a process and all its descendants to have their own private view of the globally shared kernel resources, such as the network stack, process table, mount table.

Written in C with virtually no dependencies, the software runs on any Linux computer with a 3.x kernel version or newer. The sandbox is lightweight, the overhead is low. There are no complicated configuration files to edit, no socket connections open, no daemons running in the background. All security features are implemented directly in Linux kernel and available on any Linux computer. The program is released under [GPL v2](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html) license.

Firejail can sandbox any type of processes: servers, graphical applications, and even user login sessions. The software includes security profiles for a large number of Linux programs: Mozilla Firefox, Chromium, VLC, Transmission etc. To start the sandbox, prefix your command with “firejail”:

```
$ firejail firefox                       # starting Mozilla Firefox
$ firejail transmission-gtk              # starting Transmission BitTorrent
$ firejail vlc                           # starting VideoLAN Client
$ sudo firejail /etc/init.d/nginx start  # starting nginx web server

```

To protect user’s privacy, we deploy a very strict Mandatory Access Control (MAC) on top of the existing file system. Access to passwords, encryption keys, and private data is blocked for more than 1000 different desktop applications. As a general rule, networked apps and games have access only to configuration files and Downloads directory.

User home directory as seen by Mozilla Firefox browser process

We also develop two additional sandbox subprojects:

https://odysee.com/@netblue30:9/firetools

[Firetools](http://firejailtools.wordpress.com/) is the graphical user interface of Firejail. The application is built using Qt5 libraries. It provides a sandbox launcher integrated with the system tray, sandbox editing, management and statistics. ([development page](https://github.com/netblue30/firetools))

https://odysee.com/@netblue30:9/fdns

[FDNS](https://firejaildns.wordpress.com) is a DNS over HTTPS (DoH) proxy server. FDNS protects your computer against some of the most common cyber threats,. We use only DoH services from non-logging providers ([development page](https://firejaildns.wordpress.com))

# About Us

Firejail is a community project. We are not affiliated with any company, and we don’t have any commercial goals. Our focus is the Linux desktop. Home users and Linux beginners are our target market. The software is built by a large international team of volunteers on [GitHub](http://github.com/netblue30/firejail). Expert or regular Linux user, you are welcome to join us!

Security bugs are taken seriously, please email them to netblue30 at protonmail.com

# News

March 2026 – released Firejail version 0.9.80. Small number of new features, security updates, bugfixes, and new application profiles. Major changes: a new seccomp-bpf engine, Xephyr is set as default for –x11 option, by default PID 1 is hidden inside the sandbox, and support for the new Firefox XDG configuration directory. [Release Notes](https://firejail.wordpress.com/download-2/release-notes/)

January 2026 – released Firejail version 0.9.78. This is an emergency release due to GTK library changes:

Applications that use glycin 2.0.0 or later via gdk-pixbuf2 (examples: Firefox, Thunderbird, GIMP) crash

The library glycin provides a set of “safe” image format loaders to gdk-pixbuf2, another library which is widely used in GTK-based applications for loading images.

As of gdk-pixbuf2 2.44.1 the calls to glycin loaders are wrapped in bubblewrap (bwrap).

For more details, see [#6906](https://github.com/netblue30/firejail/issues/6906). [Release Notes](https://firejail.wordpress.com/download-2/release-notes/)

November 2025 – released FDNS 0.9.76. This release brings in support for DNS over QUIC, a much larger server list, a full rebuild of the adblocking filters, and lots of bugfixes. [Release Notes](https://firejaildns.wordpress.com/release-notes/)

August 2025 – released Firejail version 0.9.76. This is an emergency release!  Due to an xorg change in the xkeyboard-config package (https://gitlab.freedesktop.org/xkeyboard-config/xkeyboard-config), many/most xorg programs were rendered completely broken when running under firejail 0.9.74. The issue was brought up by Arch Linux users, currently affected by this problem. In this release there are also a small number of new features, and lots of bugfixes. [Release Notes](https://firejail.wordpress.com/download-2/release-notes)

March 2025 – released Firejail version 0.9.74. In this release we introduce a large number of new features, bugfixes, and new application profiles. We also added experimental support for Landlock, a new stackable Linux Security Module (LSM) still under heavy development in the Linux kernel. [Release Notes](https://firejail.wordpress.com/download-2/release-notes)

October 2024 –released FDNS 0.74.2 bugfixes! [Release Notes](https://firejaildns.wordpress.com/release-notes/)

September 2024 – released FDNS 0.0.74. The software went trough a major cleanup, and several rarely used features have been removed: –debug-ssl, –proxy-addr-any, –cache-ttl, –log-timeout, –blacklist, –qps, –unlist, –disable-local-doh, –disable-syslog, –transport, and –fallback-only. In addition, we implemented several new features, such as keepalive autodetection, streamlined some of the existing functionality, a full rebuild of adblock filters, and lots of bugfixes. The server count is around 280. [Release Notes](https://firejaildns.wordpress.com/release-notes/)

October 2023 – released FDNS 0.9.72. In this release we continue the development of our nxdomain tool, streamlined some of the least-used features, and added --disable-syslog command line option. The adblock filters have been fully rebuilt, we introduced a new phishing filter, and updated the server list. [Release Notes](https://firejaildns.wordpress.com/release-notes/)

January 2023 – released Firetools version 0.9.72. [Release Notes](https://firejailtools.wordpress.com/release-notes/)

January 2023 – released Firejail version 0.9.72. In this release we introduce a new seccomp filter for restricting the creation of Linux namespaces, and continue with the implementation of our network tracing feature. There are also lots of security updates, bug fixes, and new application profiles. [Release Notes](https://firejail.wordpress.com/download-2/release-notes/)

# External Projects

http://www.shutterstock.com

Firejailed!

- [HOWTO: Firejailed Tor Browser](https://github.com/rusty-snake/firejailed-tor-browser)
- [fjp](https://rusty-snake.github.io/fjp) is a handy command line program to work fast and straightforward with firejail profiles.
- [Fireinvoke](https://codeberg.org/rps/fireinvoke) is a program that allows to easily run appimages and programs extracted from archives in the firejail sandbox by running simple command or by just double clicking it in file manager.
- [fire-install](https://codeberg.org/rps/fire-install) installs programs from archives and appimages into ~/.programs and creates launchers in ~/.local/bin. The programs are executed with fireinvoke.
- [Firewarden](https://github.com/pigmonkey/firewarden) is a bash script used to open a program within a private Firejail sandbox.
- [Ansible role](https://github.com/debops-contrib/ansible-firejail) to setup Firejail
- [firejail-extras](https://aur.archlinux.org/packages/firejail-extras/): Arch Linux AUR package containing extra security profiles for Firejail
- https://github.com/chiraag-nataraj/firejail-profiles – This is a collection of tighter security profiles maintained by a member of Firejail development team.
- Firejail package on [SlackBuilds.org](https://slackbuilds.org/repository/14.2/system/firejail/?search=firejail)
- [ansible-firejail](https://bitbucket.org/aaaaaaaaaaaaaaaaaaaaa1/ansible-firejail/overview) – Ansible playbook for Firejail.

### Share this:

- [X](https://firejail.wordpress.com/?share=twitter)
- [Facebook](https://firejail.wordpress.com/?share=facebook)
-

joe425g [July 17, 2022 at 6:46 pm](https://firejail.wordpress.com/#comment-22102)

hello, thank you for your firejail program very useful under linux. i use it all the time to isolate programs. recently i wanted to install the nim language in a private directory. normally the installer is supposed to believe that it is located at the root of home and doesn’t go out of the sandbox but this time it’s not the case, i can find traces of the program in the real user’s directory, here’s the command that i used:

firejail –noprofile –ipc-namespace –no3d –nodbus –nodvd –notv –nou2f –novideo –shell=none –noexec=all –disable-mnt –nogroups –private-dev –private-tmp –noroot –caps.drop=all –seccomp –nonewprivs –private=/private/directory/name curl https://nim-lang.org/choosenim/init.sh -sSf | sh

I took the installation order on the official nim website. please note that i have installed the latest version of firejail (0.9.70) and the result is the same. just for information. thank you for your excellent work.

[Like](https://firejail.wordpress.com/?like_comment=22102&_wpnonce=7aa2fda276)Liked by [1 person](https://firejail.wordpress.com/#)

[Reply](https://firejail.wordpress.com/?replytocom=22102#respond) ↓

[bodqhrohro](http://bodqhrohro.ho.ua) [December 16, 2023 at 8:20 am](https://firejail.wordpress.com/#comment-23039)

That’s because you only ran curl in firejail. The script was executed outside of it.

[Like](https://firejail.wordpress.com/?like_comment=23039&_wpnonce=f1bc5e2032)Liked by [2 people](https://firejail.wordpress.com/#)

[Reply](https://firejail.wordpress.com/?replytocom=23039#respond) ↓

Jimmy [March 8, 2025 at 4:58 pm](https://firejail.wordpress.com/#comment-23058)

You guys rock! Thank You!

[Like](https://firejail.wordpress.com/?like_comment=23058&_wpnonce=fdba056339)Like

[Reply](https://firejail.wordpress.com/?replytocom=23058#respond) ↓

Anderson [November 1, 2025 at 8:41 pm](https://firejail.wordpress.com/#comment-23069)

Interesting project. Can it protect against potential zip-b*mbs?

[Like](https://firejail.wordpress.com/?like_comment=23069&_wpnonce=a45ec5e061)Like

[Reply](https://firejail.wordpress.com/?replytocom=23069#respond) ↓
