---
url: "https://www.kernel.org/doc/html/latest/filesystems/efivarfs.html"
title: "efivarfs - a (U)EFI variable filesystem — The Linux Kernel documentation"
captured: "2026-11-03T02:58:41Z"
via: "live"
sha256: "01a2b66224987dbb71448cd8209bf9d0b340218da238cca83804873ca6d051f8"
wayback: "https://web.archive.org/web/20260731061158/https://www.kernel.org/doc/html/latest/filesystems/efivarfs.html"
cited_in:
  - "docs/research/hardening.md"
---

# efivarfs - a (U)EFI variable filesystem — The Linux Kernel documentation

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.kernel.org/doc/html/latest/filesystems/efivarfs.html>
> Snapshot: <https://web.archive.org/web/20260731061158/https://www.kernel.org/doc/html/latest/filesystems/efivarfs.html>

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

- [Core subsystems](https://www.kernel.org/doc/html/latest/subsystem-apis.html#core-subsystems)
- [Human interfaces](https://www.kernel.org/doc/html/latest/subsystem-apis.html#human-interfaces)
- [Networking interfaces](https://www.kernel.org/doc/html/latest/subsystem-apis.html#networking-interfaces)
- [Storage interfaces](https://www.kernel.org/doc/html/latest/subsystem-apis.html#storage-interfaces)

- [Filesystems in the Linux kernel](https://www.kernel.org/doc/html/latest/filesystems/index.html)
- [Block](https://www.kernel.org/doc/html/latest/block/index.html)
- [CD-ROM](https://www.kernel.org/doc/html/latest/cdrom/index.html)
- [SCSI Subsystem](https://www.kernel.org/doc/html/latest/scsi/index.html)
- [TCM Virtual Device](https://www.kernel.org/doc/html/latest/target/index.html)
- [NVMe Subsystem](https://www.kernel.org/doc/html/latest/nvme/index.html)

- [Other subsystems](https://www.kernel.org/doc/html/latest/subsystem-apis.html#other-subsystems)

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

- [Show Source](https://www.kernel.org/doc/html/latest/_sources/filesystems/efivarfs.rst.txt)

# efivarfs - a (U)EFI variable filesystem[¶](https://www.kernel.org/doc/html/latest/filesystems/efivarfs.html#efivarfs-a-u-efi-variable-filesystem)

The efivarfs filesystem was created to address the shortcomings of using entries in sysfs to maintain EFI variables. The old sysfs EFI variables code only supported variables of up to 1024 bytes. This limitation existed in version 0.99 of the EFI specification, but was removed before any full releases. Since variables can now be larger than a single page, sysfs isn’t the best interface for this.

Variables can be created, deleted and modified with the efivarfs filesystem.

efivarfs is typically mounted like this:

```
mount -t efivarfs none /sys/firmware/efi/efivars

```

Due to the presence of numerous firmware bugs where removing non-standard UEFI variables causes the system firmware to fail to POST, efivarfs files that are not well-known standardized variables are created as immutable files. This doesn’t prevent removal - “chattr -i” will work - but it does prevent this kind of failure from being accomplished accidentally.

Warning

When a content of an UEFI variable in /sys/firmware/efi/efivars is displayed, for example using “hexdump”, pay attention that the first 4 bytes of the output represent the UEFI variable attributes, in little-endian format.

Practically the output of each efivar is composed of:

|

4_bytes_of_attributes + efivar_data

## Mount options[¶](https://www.kernel.org/doc/html/latest/filesystems/efivarfs.html#mount-options)

|

(no)statfs

|

Control whether statfs(2) reports the variable-store used/available. Disabling it skips the QueryVariableInfo() EFI runtime service call, which might block all CPUs for a few milliseconds, and reports 0/0 for used/available instead. Defaults to statfs, except on CONFIG_PREEMPT_RT where it defaults to nostatfs.

See also:

-

[SSDT Overlays](https://www.kernel.org/doc/html/latest/admin-guide/acpi/ssdt-overlays.html)

-

[ABI file removed/sysfs-firmware-efi-vars](https://www.kernel.org/doc/html/latest/admin-guide/abi-removed-files.html#abi-file-removed-sysfs-firmware-efi-vars)

©The kernel development community. | Powered by [Sphinx 5.3.0](https://www.sphinx-doc.org/) & [Alabaster 0.7.16](https://alabaster.readthedocs.io) | [Page source](https://www.kernel.org/doc/html/latest/_sources/filesystems/efivarfs.rst.txt)
