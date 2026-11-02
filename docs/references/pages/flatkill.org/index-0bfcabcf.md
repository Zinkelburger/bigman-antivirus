---
url: "https://flatkill.org/"
title: "Flatpak - a security nightmare"
captured: "2026-11-02T18:31:16Z"
via: "live"
sha256: "fb2b9eca7d6a179c0bb74fff2ce4c62cf44e003aef66e9af2f80cd43d22ce611"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Flatpak - a security nightmare

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://flatkill.org/>

# Flatpak - a security nightmare

UPDATE: [Flatkill 2020](https://flatkill.org/2020) - let's have a look what Flatpak developers have done in last 2 years to address these issues (hint: next to nothing).

Red Hat's flatpak has been getting a lot of attention lately, it's the self-proclaimed new way of distributing desktop applications on Linux. It's secure they say ...

## The sandbox is a lie

Almost all popular applications on flathub come with filesystem=host, filesystem=home or device=all permissions, that is, write permissions to the user home directory (and more), this effectively means that all it takes to "escape the sandbox" is echo download_and_execute_evil >> ~/.bashrc. That's it.

This includes Gimp, VSCode, PyCharm, Octave, Inkscape, Steam, Audacity, VLC, ...

To make matters worse, the users are misled to believe the apps run sandboxed. For all these apps flatpak shows a reassuring "sandbox" icon when installing the app (things do not get much better even when installing in the command line - you need to know flatpak internals to understand the warnings).

## You are NOT getting security updates

Official applications and runtimes are vulnerable to known easily-exploitable code execution vulnerabilities, some of the vulnerabilities have been known (and fixed in distributions but not in flatpak) for half a year.

Yes, it's possible your linux box has been compromised if you use flatpak, we are literally talking about several months old public exploits. Ever opened an image in flatpak Gimp? The criticial vulnerability "shell in the ghost" was fixed in flatpak about one month after linux distributions.

Let's go through DSA and look for something trivial to exploit.

[CVE-2018-11235](https://www.debian.org/security/2018/dsa-4212) reported and fixed more than 4 months ago. Flatpak VSCode, Android Studio and Sublime Text still use unpatched git version 2.9.3. Note that flatpak PyCharm comes with git 2.19.0 with this issue fixed but still vulnerable to [CVE-2018-17456](https://www.debian.org/security/2018/dsa-4311).

We can demonstrate this using Sublime with GitSavvy plugin with recursive clone (plugin at https://github.com/divmain/GitSavvy/tree/dev, clone command git: clone recursively, parameter git://flatkill.org/cve-2018-11235).

## Local root exploit? Minor issue!

Up until 0.8.7 all it took to get root on the host was to install a flatpak package that contains a suid binary (flatpaks are installed to /var/lib/flatpak on your host system). Again, could this be any easier? A high severity [CVE-2017-9780](https://www.cvedetails.com/cve/CVE-2017-9780/) (CVSS Score 7.2) has indeed been assigned to this vulnerability. Flatpak developers consider this [a minor security issue](https://github.com/flatpak/flatpak/releases/tag/0.8.7).

## Future of application distribution?

Let's hope not! Sadly, it's obvious Red Hat developers working on flatpak do not care about security, yet the self-proclaimed goal is to replace desktop application distribution - a cornerstone of linux security.

And it's not only about these security problems. Running KDE apps in fakepak? Forget about desktop integration (not even font size). Need to input Chinese/Japanese/Korean characters? Forget about that too - fcitx has been [broken since flatpak 1.0](https://github.com/flatpak/flatpak/issues/2031), never fixed since.

The way we package and distribute desktop applications on Linux surely needs to be rethinked, sadly flatpak is introducing more problems than it is solving.
