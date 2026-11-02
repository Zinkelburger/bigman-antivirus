---
url: "https://docs.kernel.org/admin-guide/tainted-kernels.html"
title: "Tainted kernels — The Linux Kernel documentation"
captured: "2026-11-02T18:28:55Z"
via: "live"
sha256: "4b2740262d57f08ea5bdb9c22c59092f7366863f3c0f3646ac82727a2ab668da"
wayback: "https://web.archive.org/web/20261026101559/https://docs.kernel.org/admin-guide/tainted-kernels.html"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
  - "docs/threats/05-malware-families-and-detection.md"
---

# Tainted kernels — The Linux Kernel documentation

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://docs.kernel.org/admin-guide/tainted-kernels.html>
> Snapshot: <https://web.archive.org/web/20261026101559/https://docs.kernel.org/admin-guide/tainted-kernels.html>

https://docs.kernel.org/index.html

# [The Linux Kernel](https://docs.kernel.org/index.html)

7.4.0-rc1

### Quick search

### Contents

Working with the development community

- [Development process](https://docs.kernel.org/process/development-process.html)
- [Submitting patches](https://docs.kernel.org/process/submitting-patches.html)
- [Code of conduct](https://docs.kernel.org/process/code-of-conduct.html)
- [Maintainer handbook](https://docs.kernel.org/maintainer/index.html)
- [All development-process docs](https://docs.kernel.org/process/index.html)

Internal API manuals

- [Core API](https://docs.kernel.org/core-api/index.html)
- [Driver APIs](https://docs.kernel.org/driver-api/index.html)
- [Subsystems](https://docs.kernel.org/subsystem-apis.html)
- [Locking](https://docs.kernel.org/locking/index.html)

Development tools and processes

- [Licensing rules](https://docs.kernel.org/process/license-rules.html)
- [Writing documentation](https://docs.kernel.org/doc-guide/index.html)
- [Development tools](https://docs.kernel.org/dev-tools/index.html)
- [Testing guide](https://docs.kernel.org/dev-tools/testing-overview.html)
- [Hacking guide](https://docs.kernel.org/kernel-hacking/index.html)
- [Tracing](https://docs.kernel.org/trace/index.html)
- [Fault injection](https://docs.kernel.org/fault-injection/index.html)
- [Livepatching](https://docs.kernel.org/livepatch/index.html)
- [Rust](https://docs.kernel.org/rust/index.html)

User-oriented documentation

- [Administration](https://docs.kernel.org/admin-guide/index.html)

- [General guides to kernel administration](https://docs.kernel.org/admin-guide/index.html#general-guides-to-kernel-administration)
- [Booting the kernel](https://docs.kernel.org/admin-guide/index.html#booting-the-kernel)
- [Tracking down and identifying problems](https://docs.kernel.org/admin-guide/index.html#tracking-down-and-identifying-problems)

- [Reporting issues](https://docs.kernel.org/admin-guide/reporting-issues.html)
- [Reporting regressions](https://docs.kernel.org/admin-guide/reporting-regressions.html)
- [How to quickly build a trimmed Linux kernel](https://docs.kernel.org/admin-guide/quickly-build-trimmed-linux.html)
- [How to verify bugs and bisect regressions](https://docs.kernel.org/admin-guide/verify-bugs-and-bisect-regressions.html)
- [Bug hunting](https://docs.kernel.org/admin-guide/bug-hunting.html)
- [Bisecting a regression](https://docs.kernel.org/admin-guide/bug-bisect.html)
- [Tainted kernels](https://docs.kernel.org/admin-guide/tainted-kernels.html#)
- [Ramoops oops/panic logger](https://docs.kernel.org/admin-guide/ramoops.html)
- [Dynamic debug](https://docs.kernel.org/admin-guide/dynamic-debug-howto.html)
- [Explaining the “No working init found.” boot hang message](https://docs.kernel.org/admin-guide/init.html)
- [Documentation for Kdump - The kexec-based Crash Dumping Solution](https://docs.kernel.org/admin-guide/kdump/index.html)
- [Performance monitor support](https://docs.kernel.org/admin-guide/perf/index.html)
- [pstore block oops/panic logger](https://docs.kernel.org/admin-guide/pstore-blk.html)
- [Clearing WARN_ONCE](https://docs.kernel.org/admin-guide/clearing-warn-once.html)
- [Reducing OS jitter due to per-cpu kthreads](https://docs.kernel.org/admin-guide/kernel-per-CPU-kthreads.html)
- [Softlockup detector and hardlockup detector (aka nmi_watchdog)](https://docs.kernel.org/admin-guide/lockup-watchdogs.html)
- [Reliability, Availability and Serviceability (RAS)](https://docs.kernel.org/admin-guide/RAS/main.html)
- [Error decoding](https://docs.kernel.org/admin-guide/RAS/error-decoding.html)
- [Address translation](https://docs.kernel.org/admin-guide/RAS/address-translation.html)
- [Linux Magic System Request Key Hacks](https://docs.kernel.org/admin-guide/sysrq.html)

- [Core-kernel subsystems](https://docs.kernel.org/admin-guide/index.html#core-kernel-subsystems)
- [Block-layer and filesystem administration](https://docs.kernel.org/admin-guide/index.html#block-layer-and-filesystem-administration)
- [Device-specific guides](https://docs.kernel.org/admin-guide/index.html#device-specific-guides)
- [Workload analysis](https://docs.kernel.org/admin-guide/index.html#workload-analysis)
- [Everything else](https://docs.kernel.org/admin-guide/index.html#everything-else)

- [Build system](https://docs.kernel.org/kbuild/index.html)
- [Reporting issues](https://docs.kernel.org/admin-guide/reporting-issues.html)
- [Userspace tools](https://docs.kernel.org/tools/index.html)
- [Userspace API](https://docs.kernel.org/userspace-api/index.html)

Firmware-related documentation

- [Firmware](https://docs.kernel.org/firmware-guide/index.html)
- [Firmware and Devicetree](https://docs.kernel.org/devicetree/index.html)

Architecture-specific documentation

- [CPU architectures](https://docs.kernel.org/arch/index.html)

Other documentation

- [Unsorted documentation](https://docs.kernel.org/staging/index.html)

Translations

- [Translations](https://docs.kernel.org/translations/index.html)

### This Page

- [Show Source](https://docs.kernel.org/_sources/admin-guide/tainted-kernels.rst.txt)

English

- [Chinese (Simplified)](https://docs.kernel.org/translations/zh_CN/admin-guide/tainted-kernels.html)
- [Chinese (Traditional)](https://docs.kernel.org/translations/zh_TW/admin-guide/tainted-kernels.html)

# Tainted kernels[¶](https://docs.kernel.org/admin-guide/tainted-kernels.html#tainted-kernels)

The kernel will mark itself as ‘tainted’ when something occurs that might be relevant later when investigating problems. Don’t worry too much about this, most of the time it’s not a problem to run a tainted kernel; the information is mainly of interest once someone wants to investigate some problem, as its real cause might be the event that got the kernel tainted. That’s why bug reports from tainted kernels will often be ignored by developers, hence try to reproduce problems with an untainted kernel.

Note the kernel will remain tainted even after you undo what caused the taint (i.e. unload a proprietary kernel module), to indicate the kernel remains not trustworthy. That’s also why the kernel will print the tainted state when it notices an internal problem (a ‘kernel bug’), a recoverable error (‘kernel oops’) or a non-recoverable error (‘kernel panic’) and writes debug information about this to the logs dmesg outputs. It’s also possible to check the tainted state at runtime through a file in /proc/.

## Tainted flag in bugs, oops or panics messages[¶](https://docs.kernel.org/admin-guide/tainted-kernels.html#tainted-flag-in-bugs-oops-or-panics-messages)

You find the tainted state near the top in a line starting with ‘CPU:’; if or why the kernel was tainted is shown after the Process ID (‘PID:’) and a shortened name of the command (‘Comm:’) that triggered the event:

```
BUG: unable to handle kernel NULL pointer dereference at 0000000000000000
Oops: 0002 [#1] SMP PTI
CPU: 0 PID: 4424 Comm: insmod Tainted: P        W  O      4.20.0-0.rc6.fc30 #1
Hardware name: Red Hat KVM, BIOS 0.5.1 01/01/2011
RIP: 0010:my_oops_init+0x13/0x1000 [kpanic]
[...]

```

You’ll find a ‘Not tainted: ‘ there if the kernel was not tainted at the time of the event; if it was, then it will print ‘Tainted: ‘ and characters either letters or blanks. In the example above it looks like this:

```
Tainted: P        W  O

```

The meaning of those characters is explained in the table below. In this case the kernel got tainted earlier because a proprietary Module (P) was loaded, a warning occurred (W), and an externally-built module was loaded (O). To decode other letters use the table below.

## Decoding tainted state at runtime[¶](https://docs.kernel.org/admin-guide/tainted-kernels.html#decoding-tainted-state-at-runtime)

At runtime, you can query the tainted state by reading cat /proc/sys/kernel/tainted. If that returns 0, the kernel is not tainted; any other number indicates the reasons why it is. The easiest way to decode that number is the script tools/debugging/kernel-chktaint, which your distribution might ship as part of a package called linux-tools or kernel-tools; if it doesn’t, you can download the script from [git.kernel.org](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/tools/debugging/kernel-chktaint) and execute it with sh kernel-chktaint, which would print something like this on the machine that had the statements in the logs that were quoted earlier:

```
Kernel is Tainted for following reasons:
 * Proprietary module was loaded (#0)
 * Kernel issued warning (#9)
 * Externally-built ('out-of-tree') module was loaded  (#12)
See Documentation/admin-guide/tainted-kernels.rst in the Linux kernel or
 https://www.kernel.org/doc/html/latest/admin-guide/tainted-kernels.html for
 a more details explanation of the various taint flags.
Raw taint value as int/string: 4609/'P        W  O     '

```

You can try to decode the number yourself. That’s easy if there was only one reason that got your kernel tainted, as in this case you can find the number with the table below. If there were multiple reasons you need to decode the number, as it is a bitfield, where each bit indicates the absence or presence of a particular type of taint. It’s best to leave that to the aforementioned script, but if you need something quick you can use this shell command to check which bits are set:

```
$ for i in $(seq 0 19); do echo $i $(($(cat /proc/sys/kernel/tainted)>>$i&1));done

```

## Table for decoding tainted state[¶](https://docs.kernel.org/admin-guide/tainted-kernels.html#table-for-decoding-tainted-state)

|

Bit

|

Log

|

Number

|

Reason that got the kernel tainted

|

0

|

G/P

|

1

|

proprietary module was loaded

|

1

|

_/F

|

2

|

module was force loaded

|

2

|

_/S

|

4

|

kernel running on an out of specification system

|

3

|

_/R

|

8

|

module was force unloaded

|

4

|

_/M

|

16

|

processor reported a Machine Check Exception (MCE)

|

5

|

_/B

|

32

|

bad page referenced or some unexpected page flags

|

6

|

_/U

|

64

|

taint requested by userspace application

|

7

|

_/D

|

128

|

kernel died recently, i.e. there was an OOPS or BUG

|

8

|

_/A

|

256

|

ACPI table overridden by user

|

9

|

_/W

|

512

|

kernel issued warning

|

10

|

_/C

|

1024

|

staging driver was loaded

|

11

|

_/I

|

2048

|

workaround for bug in platform firmware applied

|

12

|

_/O

|

4096

|

externally-built (“out-of-tree”) module was loaded

|

13

|

_/E

|

8192

|

unsigned module was loaded

|

14

|

_/L

|

16384

|

soft lockup occurred

|

15

|

_/K

|

32768

|

kernel has been live patched

|

16

|

_/X

|

65536

|

auxiliary taint, defined for and used by distros

|

17

|

_/T

|

131072

|

kernel was built with the struct randomization plugin

|

18

|

_/N

|

262144

|

an in-kernel test has been run

|

19

|

_/J

|

524288

|

userspace used a mutating debug operation in fwctl

Note: The character _ is representing a blank in this table to make reading easier.

## More detailed explanation for tainting[¶](https://docs.kernel.org/admin-guide/tainted-kernels.html#more-detailed-explanation-for-tainting)

-

G if all modules loaded have a GPL or compatible license, P if any proprietary module has been loaded. Modules without a MODULE_LICENSE or with a MODULE_LICENSE that is not recognised by insmod as GPL compatible are assumed to be proprietary.

-

F if any module was force loaded by insmod -f, ' ' if all modules were loaded normally.

-

S if the kernel is running on a processor or system that is out of specification: hardware has been put into an unsupported configuration, therefore proper execution cannot be guaranteed. Kernel will be tainted if, for example:

-

on x86: PAE is forced through forcepae on intel CPUs (such as Pentium M) which do not report PAE but may have a functional implementation, an SMP kernel is running on non officially capable SMP Athlon CPUs, MSRs are being poked at from userspace.

-

on arm: kernel running on certain CPUs (such as Keystone 2) without having certain kernel features enabled.

-

on arm64: there are mismatched hardware features between CPUs, the bootloader has booted CPUs in different modes.

-

certain drivers are being used on non supported architectures (such as scsi/snic on something else than x86_64, scsi/ips on non x86/x86_64/itanium, have broken firmware settings for the irqchip/irq-gic on arm64 ...).

-

x86/x86_64: Microcode late loading is dangerous and will result in tainting the kernel. It requires that all CPUs rendezvous to make sure the update happens when the system is as quiescent as possible. However, a higher priority MCE/SMI/NMI can move control flow away from that rendezvous and interrupt the update, which can be detrimental to the machine.

-

R if a module was force unloaded by rmmod -f, ' ' if all modules were unloaded normally.

-

M if any processor has reported a Machine Check Exception, ' ' if no Machine Check Exceptions have occurred.

-

B If a page-release function has found a bad page reference or some unexpected page flags. This indicates a hardware problem or a kernel bug; there should be other information in the log indicating why this tainting occurred.

-

U if a user or user application specifically requested that the Tainted flag be set, ' ' otherwise.

-

D if the kernel has died recently, i.e. there was an OOPS or BUG.

-

A if an ACPI table has been overridden.

-

W if a warning has previously been issued by the kernel. (Though some warnings may set more specific taint flags.)

-

C if a staging driver has been loaded.

-

I if the kernel is working around a severe bug in the platform firmware (BIOS or similar).

-

O if an externally-built (“out-of-tree”) module has been loaded.

-

E if an unsigned module has been loaded in a kernel supporting module signature.

-

L if a soft lockup has previously occurred on the system.

-

K if the kernel has been live patched.

-

X Auxiliary taint, defined for and used by Linux distributors.

-

T Kernel was build with the randstruct plugin, which can intentionally produce extremely unusual kernel structure layouts (even performance pathological ones), which is important to know when debugging. Set at build time.

-

N if an in-kernel test, such as a KUnit test, has been run.

-

J if userspace opened /dev/fwctl/* and performed a FWTCL_RPC_DEBUG_WRITE to use the devices debugging features. Device debugging features could cause the device to malfunction in undefined ways.

©The kernel development community. | Powered by [Sphinx 5.3.0](https://www.sphinx-doc.org/) & [Alabaster 0.7.16](https://alabaster.readthedocs.io) | [Page source](https://docs.kernel.org/_sources/admin-guide/tainted-kernels.rst.txt)
