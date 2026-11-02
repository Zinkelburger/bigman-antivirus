---
url: "https://www.kernel.org/doc/html/latest/admin-guide/LSM/Yama.html"
title: "Yama — The Linux Kernel documentation"
captured: "2026-11-02T18:55:31Z"
via: "live"
sha256: "156febd9d065eb1e217c403747350e4e05073f9ba032357ba24ddc764196792c"
wayback: "https://web.archive.org/web/20261013093430/https://www.kernel.org/doc/html/latest/admin-guide/LSM/Yama.html"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Yama — The Linux Kernel documentation

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.kernel.org/doc/html/latest/admin-guide/LSM/Yama.html>
> Snapshot: <https://web.archive.org/web/20261013093430/https://www.kernel.org/doc/html/latest/admin-guide/LSM/Yama.html>

https://www.kernel.org/doc/html/latest/index.html

# [The Linux Kernel](https://www.kernel.org/doc/html/latest/index.html)

7.4.0-rc1

### Quick search

### Contents

Working with the development community

- [Development process](https://www.kernel.org/doc/html/latest/process/development-process.html)
- [Submitting patches](https://www.kernel.org/doc/html/latest/process/submitting-patches.html)
- [Code of conduct](https://www.kernel.org/doc/html/latest/process/code-of-conduct.html)
- [Maintainer handbook](https://www.kernel.org/doc/html/latest/maintainer/index.html)
- [All development-process docs](https://www.kernel.org/doc/html/latest/process/index.html)

Internal API manuals

- [Core API](https://www.kernel.org/doc/html/latest/core-api/index.html)
- [Driver APIs](https://www.kernel.org/doc/html/latest/driver-api/index.html)
- [Subsystems](https://www.kernel.org/doc/html/latest/subsystem-apis.html)
- [Locking](https://www.kernel.org/doc/html/latest/locking/index.html)

Development tools and processes

- [Licensing rules](https://www.kernel.org/doc/html/latest/process/license-rules.html)
- [Writing documentation](https://www.kernel.org/doc/html/latest/doc-guide/index.html)
- [Development tools](https://www.kernel.org/doc/html/latest/dev-tools/index.html)
- [Testing guide](https://www.kernel.org/doc/html/latest/dev-tools/testing-overview.html)
- [Hacking guide](https://www.kernel.org/doc/html/latest/kernel-hacking/index.html)
- [Tracing](https://www.kernel.org/doc/html/latest/trace/index.html)
- [Fault injection](https://www.kernel.org/doc/html/latest/fault-injection/index.html)
- [Livepatching](https://www.kernel.org/doc/html/latest/livepatch/index.html)
- [Rust](https://www.kernel.org/doc/html/latest/rust/index.html)

User-oriented documentation

- [Administration](https://www.kernel.org/doc/html/latest/admin-guide/index.html)

- [General guides to kernel administration](https://www.kernel.org/doc/html/latest/admin-guide/index.html#general-guides-to-kernel-administration)

- [Linux kernel release 6.x <http://kernel.org/>](https://www.kernel.org/doc/html/latest/admin-guide/README.html)
- [Linux allocated devices (4.x+ version)](https://www.kernel.org/doc/html/latest/admin-guide/devices.html)
- [Feature status on all architectures](https://www.kernel.org/doc/html/latest/admin-guide/features.html)
- [Rules on how to access information in sysfs](https://www.kernel.org/doc/html/latest/admin-guide/sysfs-rules.html)
- [Documentation for /proc/sys](https://www.kernel.org/doc/html/latest/admin-guide/sysctl/index.html)
- [How CPU topology info is exported via sysfs](https://www.kernel.org/doc/html/latest/admin-guide/cputopology.html)
- [Linux ABI description](https://www.kernel.org/doc/html/latest/admin-guide/abi.html)
- [Hardware vulnerabilities](https://www.kernel.org/doc/html/latest/admin-guide/hw-vuln/index.html)
- [Linux Security Module Usage](https://www.kernel.org/doc/html/latest/admin-guide/LSM/index.html)
- [Perf events and tool security](https://www.kernel.org/doc/html/latest/admin-guide/perf-security.html)

- [Booting the kernel](https://www.kernel.org/doc/html/latest/admin-guide/index.html#booting-the-kernel)
- [Tracking down and identifying problems](https://www.kernel.org/doc/html/latest/admin-guide/index.html#tracking-down-and-identifying-problems)
- [Core-kernel subsystems](https://www.kernel.org/doc/html/latest/admin-guide/index.html#core-kernel-subsystems)
- [Block-layer and filesystem administration](https://www.kernel.org/doc/html/latest/admin-guide/index.html#block-layer-and-filesystem-administration)
- [Device-specific guides](https://www.kernel.org/doc/html/latest/admin-guide/index.html#device-specific-guides)
- [Workload analysis](https://www.kernel.org/doc/html/latest/admin-guide/index.html#workload-analysis)
- [Everything else](https://www.kernel.org/doc/html/latest/admin-guide/index.html#everything-else)

- [Build system](https://www.kernel.org/doc/html/latest/kbuild/index.html)
- [Reporting issues](https://www.kernel.org/doc/html/latest/admin-guide/reporting-issues.html)
- [Userspace tools](https://www.kernel.org/doc/html/latest/tools/index.html)
- [Userspace API](https://www.kernel.org/doc/html/latest/userspace-api/index.html)

Firmware-related documentation

- [Firmware](https://www.kernel.org/doc/html/latest/firmware-guide/index.html)
- [Firmware and Devicetree](https://www.kernel.org/doc/html/latest/devicetree/index.html)

Architecture-specific documentation

- [CPU architectures](https://www.kernel.org/doc/html/latest/arch/index.html)

Other documentation

- [Unsorted documentation](https://www.kernel.org/doc/html/latest/staging/index.html)

Translations

- [Translations](https://www.kernel.org/doc/html/latest/translations/index.html)

### This Page

- [Show Source](https://www.kernel.org/doc/html/latest/_sources/admin-guide/LSM/Yama.rst.txt)

# Yama[¶](https://www.kernel.org/doc/html/latest/admin-guide/LSM/Yama.html#yama)

Yama is a Linux Security Module that collects system-wide DAC security protections that are not handled by the core kernel itself. This is selectable at build-time with CONFIG_SECURITY_YAMA, and can be controlled at run-time through sysctls in /proc/sys/kernel/yama:

## ptrace_scope[¶](https://www.kernel.org/doc/html/latest/admin-guide/LSM/Yama.html#ptrace-scope)

As Linux grows in popularity, it will become a larger target for malware. One particularly troubling weakness of the Linux process interfaces is that a single user is able to examine the memory and running state of any of their processes. For example, if one application (e.g. Pidgin) was compromised, it would be possible for an attacker to attach to other running processes (e.g. Firefox, SSH sessions, GPG agent, etc) to extract additional credentials and continue to expand the scope of their attack without resorting to user-assisted phishing.

This is not a theoretical problem. [SSH session hijacking](https://www.blackhat.com/presentations/bh-usa-05/bh-us-05-boileau.pdf) and [arbitrary code injection](https://c-skills.blogspot.com/2007/05/injectso.html) attacks already exist and remain possible if ptrace is allowed to operate as before. Since ptrace is not commonly used by non-developers and non-admins, system builders should be allowed the option to disable this debugging system.

For a solution, some applications use prctl(PR_SET_DUMPABLE, ...) to specifically disallow such ptrace attachment (e.g. ssh-agent), but many do not. A more general solution is to only allow ptrace directly from a parent to a child process (i.e. direct “gdb EXE” and “strace EXE” still work), or with CAP_SYS_PTRACE (i.e. “gdb --pid=PID”, and “strace -p PID” still work as root).

In mode 1, software that has defined application-specific relationships between a debugging process and its inferior (crash handlers, etc), prctl(PR_SET_PTRACER, pid, ...) can be used. An inferior can declare which other process (and its descendants) are allowed to call PTRACE_ATTACH against it. Only one such declared debugging process can exists for each inferior at a time. For example, this is used by KDE, Chromium, and Firefox’s crash handlers, and by Wine for allowing only Wine processes to ptrace each other. If a process wishes to entirely disable these ptrace restrictions, it can call prctl(PR_SET_PTRACER, PR_SET_PTRACER_ANY, ...) so that any otherwise allowed process (even those in external pid namespaces) may attach.

The sysctl settings (writable only with CAP_SYS_PTRACE) are:

0 - classic ptrace permissions:

a process can PTRACE_ATTACH to any other process running under the same uid, as long as it is dumpable (i.e. did not transition uids, start privileged, or have called prctl(PR_SET_DUMPABLE...) already). Similarly, PTRACE_TRACEME is unchanged.

1 - restricted ptrace:

a process must have a predefined relationship with the inferior it wants to call PTRACE_ATTACH on. By default, this relationship is that of only its descendants when the above classic criteria is also met. To change the relationship, an inferior can call prctl(PR_SET_PTRACER, debugger, ...) to declare an allowed debugger PID to call PTRACE_ATTACH on the inferior. Using PTRACE_TRACEME is unchanged.

2 - admin-only attach:

only processes with CAP_SYS_PTRACE may use ptrace, either with PTRACE_ATTACH or through children calling PTRACE_TRACEME.

3 - no attach:

no processes may use ptrace with PTRACE_ATTACH nor via PTRACE_TRACEME. Once set, this sysctl value cannot be changed.

The original children-only logic was based on the restrictions in grsecurity.

©The kernel development community. | Powered by [Sphinx 5.3.0](https://www.sphinx-doc.org/) & [Alabaster 0.7.16](https://alabaster.readthedocs.io) | [Page source](https://www.kernel.org/doc/html/latest/_sources/admin-guide/LSM/Yama.rst.txt)
