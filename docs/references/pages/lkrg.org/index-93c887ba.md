---
url: "https://lkrg.org/"
title: "LKRG - Linux Kernel Runtime Guard"
captured: "2026-11-02T18:34:52Z"
via: "live"
sha256: "e457b2d2886d7b862a2d643f1cf98168a0dc7d5369c68cb9e7113e35c8f61fc7"
wayback: "https://web.archive.org/web/20261012072217/https://lkrg.org/"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# LKRG - Linux Kernel Runtime Guard

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://lkrg.org/>
> Snapshot: <https://web.archive.org/web/20261012072217/https://lkrg.org/>

# Linux Kernel Runtime Guard

LKRG performs runtime integrity checking of the Linux kernel and detection of security vulnerability exploits against the kernel.

LKRG is a kernel module (not a kernel patch), so it can be built for and loaded on top of a wide range of mainline and distros' kernels, without needing to patch those. We currently support kernel versions ranging from as far back as RHEL7's (and its many clones/revisions) to latest mainline and distros kernels.

LKRG 1.0.1 has been tested with Linux kernels from RHEL/CentOS 7's 3.10.0-1160 and up to Fedora's build of 7.0.0-62.fc45.x86_64.

LKRG currently supports the x86-64, 32-bit x86, AArch64 (ARM64), and 32-bit ARM CPU architectures.

Download ([release notes](https://www.openwall.com/lists/announce/2026/04/23/1), [previous release notes](https://www.openwall.com/lists/announce/2025/09/02/1)):

- [LKRG 1.0.1](https://lkrg.org/download/lkrg-1.0.1.tar.gz) and its [signature](https://lkrg.org/download/lkrg-1.0.1.tar.gz.sign)

These and older versions of LKRG are also [available from the Openwall file archive](https://download.openwall.net/pub/projects/lkrg/). The source code and revision history of LKRG can be browsed on [GitHub](https://github.com/lkrg-org/lkrg).

Follow [this link](https://www.openwall.com/signatures/) for information on verifying the signatures.

Please check out our [presentation slides on LKRG](https://www.openwall.com/presentations/OSTconf2020-LKRG-In-A-Nutshell/), [on its 1.0 release](https://www.openwall.com/presentations/NullconBerlin2025-LKRG/), and separately [on its remote logging capability](https://www.openwall.com/presentations/BSidesZagreb2024-Linux-remote-logging/).

We tweet LKRG project news via [@lkrg_org](https://twitter.com/lkrg_org). We also announce LKRG releases on Openwall's [announce](https://www.openwall.com/lists/announce/) and [lkrg-users](https://www.openwall.com/lists/lkrg-users/) mailing lists.

The lkrg-users mailing list is also a place where you can share your experience with LKRG and ask questions.  Please be sure to specify an informative message subject whenever you post to the list (that is, something better than "question" or "problem"). To subscribe, enter your e-mail address below or send an empty message to <lkrg-users-subscribe at lists.openwall.com>. You will be required to confirm your subscription by "replying" to the automated confirmation request that will be sent to you. You will be able to unsubscribe at any time and we will not use your e-mail address for any other purpose or share it with a third party. However, if you post to the list, other subscribers and those viewing the archives may see your address(es) as specified on your message.

Recent LKRG packages (of 1.0.1 or at least 0.9.9+) exist in [ALT Linux](https://packages.altlinux.org/en/sisyphus/srpms/lkrg/), [Arch Linux](https://aur.archlinux.org/packages/lkrg-dkms/), [Gentoo](https://packages.gentoo.org/packages/app-antivirus/lkrg), [NixOS](https://search.nixos.org/packages?channel=unstable&from=0&size=50&sort=relevance&type=packages&query=lkrg), and [Rocky Linux](https://sig-security.rocky.page/packages/lkrg/).
Outdated LKRG packages (older than 0.9.9) also exist(ed) in [Astra Linux](https://dl.astralinux.ru/astra/stable/orel/repository/pool/main/l/lkrg/), Guix, [Whonix](https://www.whonix.org/wiki/Linux_Kernel_Runtime_Guard_LKRG), and [Yocto](https://git.yoctoproject.org/meta-security/tree/recipes-kernel/lkrg) (and thus also OpenBMC).
[Rocky Linux SIG/Security](https://sig-security.rocky.page) yum/dnf repository and LKRG packages are also usable on other Enterprise Linux distributions (AlmaLinux 8 and 9, RHEL 8 and 9, etc.)
Whonix's packaging is also usable for Debian and its other derived distributions (including Ubuntu).

In non-free [Rocky Linux from CIQ Pro Hardened](https://ciq.com/products/rocky-linux/pro/hardened/), LKRG is enabled out of the box and is supported by CIQ. Moreover, it is signed to be part of the UEFI Secure Boot chain starting with keys pre-installed in off-the-shelf hardware.

Why or why not LKRG?

As controversial as this concept is, LKRG attempts to post-detect and hopefully promptly respond to unauthorized modifications to the running Linux kernel (integrity checking) or to credentials such as user IDs of the running processes (exploit detection). For process credentials, LKRG attempts to detect the exploit and take action before the kernel would grant access (such as open a file) based on the unauthorized credentials.

Master's Thesis of Juho Junnila, entitled "Effectiveness of Linux Rootkit Detection Tools", [shows](https://www.openwall.com/lists/lkrg-users/2020/06/14/5) LKRG as the most effective kernel rootkit detector (of those tested).

LKRG defeats many pre-existing exploits of Linux kernel vulnerabilities, and will likely defeat many future exploits (including of yet unknown vulnerabilities) that do not specifically attempt to bypass LKRG. While LKRG is bypassable by design, such bypasses tend to require more complicated and/or less reliable exploits.

LKRG also provides security through diversity, much like running an uncommon OS kernel would, yet without the usability drawbacks of actually running an uncommon OS. As free LKRG becomes somewhat popular and possibly starts being deliberately bypassed by some exploits, we might introduce paid LKRG Pro as a means to fund the project and provide further diversity (with Pro's smaller userbase being beneficial), extra and specialized functionality, and maybe distro-specific binary builds.

Like any software, LKRG may contain bugs and some of those might even be new security vulnerabilities. Moreover, usage of any out-of-tree kernel module involves risk of incompatibilities with the specific kernel version/build, and LKRG is no exception. We test LKRG across a wide range of kernel versions, but obviously not with future kernel versions, with which LKRG might or might not work right. You need to weigh the benefits vs. risks of using LKRG, considering that LKRG is most useful on systems that realistically, despite of this being a best practice for security, won't be promptly rebooted into new kernels (nor live-patched) whenever a new kernel vulnerability is discovered.

LKRG is currently in an experimental stage. We expect occasional false positives (integrity violations and/or exploits detected when there aren't ones), especially with Linux kernel versions or configurations other than those we've tested. Please keep this in mind when configuring LKRG's response to detected violations, such as starting with mild enforcement and only enabling stricter enforcement once you've confirmed you are not seeing false positives.

To illustrate LKRG's exploit detection capabilities, in our testing on vulnerable distro kernels LKRG successfully detected certain pre-existing exploits of CVE-2014-9322 (BadIRET), CVE-2017-5123 (waitid(2) missing access_ok), CVE-2017-6074 (use-after-free in DCCP protocol). However, it wouldn't be expected to detect exploits of CVE-2016-5195 (Dirty COW) since those directly target the userspace even if via the kernel. While in case of Dirty COW the LKRG "bypass" happened due to the nature of the bug and this being the way to exploit it, it's also a way for future exploits to bypass LKRG by similarly directly targeting userspace. It remains to be seen whether such exploits become common (unlikely unless LKRG or similar become popular?) and what (negative?) effect on their reliability this will have (for kernel vulnerabilities where directly targeting the userspace isn't essential and possibly not straightforward).

Performance

The overall [performance impact](https://github.com/lkrg-org/lkrg/blob/main/PERFORMANCE) of LKRG 0.8 as seen from the geometric mean of 58 test results from Phoronix Test Suite on a Xeon E-2176G with Ubuntu 18.04 is around 2.5% for LKRG's heavy profile (which is the default) and around 2.0% for the light profile, although the individual test results vary. Phoronix also [published](https://www.phoronix.com/scan.php?page=article&item=lkrg-08-linux&num=1) a different set of benchmarks with as many as 119 tests.
