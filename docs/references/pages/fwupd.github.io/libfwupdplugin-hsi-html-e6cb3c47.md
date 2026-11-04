---
url: "https://fwupd.github.io/libfwupdplugin/hsi.html"
title: "FwupdPlugin: Host Security ID Specification"
captured: "2026-11-02T18:31:21Z"
via: "live"
sha256: "831a902202246bab9edfc0baeb75fac6f6ebb257a3c27689b90ca41b4da52c82"
wayback: "https://web.archive.org/web/20261005034711/https://fwupd.github.io/libfwupdplugin/hsi.html"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# FwupdPlugin: Host Security ID Specification

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://fwupd.github.io/libfwupdplugin/hsi.html>
> Snapshot: <https://web.archive.org/web/20261005034711/https://fwupd.github.io/libfwupdplugin/hsi.html>

####  Host Security ID Specification https://fwupd.github.io/libfwupdplugin/hsi.html#title

WARNING: This specification is still in active development: it is incomplete, subject to change, and may have errors; use this at your own risk. It is based on publicly available information.

Authors:

- Richard Hughes
- Mario Limonciello
- Alex Bazhaniuk
- Alex Matrosov

## Introductionhttps://fwupd.github.io/libfwupdplugin/hsi.html#introduction

Not all system vendors prioritize building a secure platform. The truth is that security costs money. Vendors have to choose between saving a few cents on a bill-of-materials by sharing a SPI chip, or correctly implementing BootGuard. Discovering security vulnerabilities often takes an external researcher filing a disclosure. These disclosures are often technical in nature and difficult for an average consumer to decipher.

The Linux Vendor Firmware Service (LVFS) could provide some easy-to-understand information to people buying hardware. The service already knows a huge amount of information about machines from signed reports uploaded to the LVFS and from analyzing firmware binaries. However this information alone does not explain firmware security to the user in a way they can actually interpret.

### Other Toolshttps://fwupd.github.io/libfwupdplugin/hsi.html#other-tools

Traditionally, figuring out the true security of your hardware and firmware requires sifting through the marketing documentation provided by the OEM and in many cases just “trusting” they did it right. Tools such as Chipsec can check the hardware configuration, but they do not work out of the box and use technical jargon that an average user cannot interpret. Unfortunately, running a tool like Chipsec requires that you actively turn off some security layers such as UEFI Secure Boot, and allow 3rd party unsigned kernel modules to be loaded.

## [Verifying Host Firmware Security](https://fwupd.github.io/libfwupdplugin/hsi.html#verifying)https://fwupd.github.io/libfwupdplugin/hsi.html#verifying-host-firmware-security

To start out some core protections must be assigned a relative importance. Then an evaluation must be done to determine how each vendor is conforming to the model. For instance, a user might say that for home use any hardware the bare minimum security level (HSI:1) is good enough. For a work laptop the company IT department might restrict the choice of models to anything meeting the criteria of level HSI:2 or above. A journalist or a security researcher would only buy level HSI:3 and above. The reality is that HSI:4 is going to be more expensive than some unbranded hardware that is rated HSI:0.

To be trusted, this rating information should be distributed in a centralized agnostic database such as the LVFS.

Of course, tools need to detect implementation errors, and to verify that the model that is measured does indeed match the HSI level advertised by the LVFS. Some existing compliance solutions place the burden on the OEM to define what firmware security has been implemented, which is easy to get wrong and in some cases impossible to verify.

For this reason HSI will only measure security protections that can be verified by the end user without requiring any extra hardware to be connected, additional software to be installed, or disabling any existing security layers to measure.

The HSI specification is primarily designed for laptop and desktop hardware, although some tests may still make sense on server or embedded hardware. It is not expected that non-consumer hardware will publish an HSI number.

## [Runtime Behavior](https://fwupd.github.io/libfwupdplugin/hsi.html#runtime-behaviour)https://fwupd.github.io/libfwupdplugin/hsi.html#runtime-behavior

Orthogonal to the security features provided by the firmware there are other security considerations related to the firmware which may require internet access to discover or that runtime OS changes directly affect the security of the firmware. It would not make sense to have have updates on the LVFS as a requirement for a specific security level as this would mean offline the platform might be a higher level initially but as soon as it is brought online it is downgraded which would be really confusing to users. The core security level will not change at Operating System runtime, but the suffix may.

More information Additional information about specific bugs and debugging steps are available on the [fwupd wiki](https://github.com/fwupd/fwupd/wiki/Host-security-ID-runtime-issues).

### [HSI:0 (Insecure State)](https://fwupd.github.io/libfwupdplugin/hsi.html#hsi-level0)https://fwupd.github.io/libfwupdplugin/hsi.html#hsi0-insecure-state

Limited firmware protection.

The lowest security level with little or no detected firmware protections. This is the default security level if no tests can be run or some tests in the next security level have failed.

### [HSI:1 (Critical State)](https://fwupd.github.io/libfwupdplugin/hsi.html#hsi-level1)https://fwupd.github.io/libfwupdplugin/hsi.html#hsi1-critical-state

Basic protection but any failure would lead to a critical security impact.

This security level corresponds to the most basic of security protections considered essential by security professionals. Any failures at this level would have critical security impact and could likely be used to compromise the system firmware without physical access.

### [HSI:2 (Risky State)](https://fwupd.github.io/libfwupdplugin/hsi.html#hsi-level2)https://fwupd.github.io/libfwupdplugin/hsi.html#hsi2-risky-state

The failure is only happened by the theoretical exploit in the lab.

This security level corresponds to firmware security issues that pose a theoretical concern or where any exploit would be difficult or impractical to use. At this level various technologies may be employed to protect the boot process from modification by an attacker with local access to the machine.

### [HSI:3 (Protected State)](https://fwupd.github.io/libfwupdplugin/hsi.html#hsi-level3)https://fwupd.github.io/libfwupdplugin/hsi.html#hsi3-protected-state

The system firmware only has few minor issues which do not affect the security status.

This security level corresponds to out-of-band protection of the system firmware perhaps including recovery.

### [HSI:4 (Secure State)](https://fwupd.github.io/libfwupdplugin/hsi.html#hsi-level4)https://fwupd.github.io/libfwupdplugin/hsi.html#hsi4-secure-state

The system is in a robust secure state.

The system is corresponding several kind of encryption and execution protection for the system firmware.

### [HSI:5 (Secure Proven State)](https://fwupd.github.io/libfwupdplugin/hsi.html#hsi-level5)https://fwupd.github.io/libfwupdplugin/hsi.html#hsi5-secure-proven-state

This security level corresponds to out-of-band attestation of the system firmware. There are currently no tests implemented for HSI:5 and so this security level cannot yet be obtained.

### [HSI Runtime Suffix !](https://fwupd.github.io/libfwupdplugin/hsi.html#runtime-bang)https://fwupd.github.io/libfwupdplugin/hsi.html#hsi-runtime-suffix

A runtime security issue detected.

-

UEFI [Secure Boot](https://wiki.ubuntu.com/UEFI/SecureBoot) has been turned off. [v1.5.0]

-

The kernel is [tainted](https://www.kernel.org/doc/html/latest/admin-guide/tainted-kernels.html) due to a non-free module or critical firmware issue. [v1.5.0]

-

The kernel is not [locked down](https://mjg59.dreamwidth.org/50577.html). [v1.5.0]

-

Unencrypted [swap partition](https://wiki.archlinux.org/index.php/Dm-crypt/Swap_encryption). [v1.5.0]

-

The installed fwupd is running with [custom or modified plugins](https://github.com/fwupd/fwupd/tree/main/plugins). [v1.5.0]

## [Low Security Level](https://fwupd.github.io/libfwupdplugin/hsi.html#low-security-level)https://fwupd.github.io/libfwupdplugin/hsi.html#low-security-level

A safe baseline for security should be HSI-1. If your system isn’t at least meeting this criteria, you should adjust firmware setup options, contact your manufacturer or replace the hardware.

The command line tool fwupdmgr security included with fwupd 1.8.4 or later will make individual recommendations on what you can do for individual test failures. GUI tools built against libfwupd 1.8.4 or later may also make these recommendation as well.

## [Tests included in fwupd](https://fwupd.github.io/libfwupdplugin/hsi.html#tests)https://fwupd.github.io/libfwupdplugin/hsi.html#tests-included-in-fwupd

The set of tests is currently x86 UEFI-centric, but will be expanded in the future for various ARM or RISC-V firmware protections as required. Where the requirement is architecture or processor specific it has been noted.

### [AMD Microcode signature vulnerability](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Amd.EntrySign)https://fwupd.github.io/libfwupdplugin/hsi.html#amd-microcode-signature-vulnerability

AMD systems from Zen 1 through Zen 4 have a vulnerability that may allow an attacker with system administrative privilege to load malicious CPU microcode patches.

There are software mitigations for this vulnerability, but this check is whether the firmware has been updated to prevent it.

Impact:

The attacker is able to load malicious microcode.

Possible results:

- not-locked: platform is vulnerable to the attack (failure)
- locked: platform is not vulnerable to the attack (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v2.1.2]

References:

- [Zen and the Art of Microcode Hacking](https://bughunters.google.com/blog/5424842357473280/zen-and-the-art-of-microcode-hacking)
- [AMD SB-7033](https://www.amd.com/en/resources/product-security/bulletin/amd-sb-7033.html)

### [AMD Platform Secure Boot](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Amd.PlatformSecureBoot)https://fwupd.github.io/libfwupdplugin/hsi.html#amd-platform-secure-boot

Platform secure boot is a processor feature that prevents the machine from running firmware images not released by the system manufacturer.

It forms a root-of-trust by fusing in cryptographic keys into the processor itself.

Impact:

When Platform Secure boot is not set up then the chain-of-trust between the CPU and the bootloader can not be verified.

Possible results:

- not-enabled: boot is not verified (failure)
- enabled: Platform Secure Boot enabled (success)

A test success result is needed to meet HSI-2 on systems that run this test. [v2.1.1]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [PlatformSecureBoot article](https://www.ioactive.com/exploring-amd-platform-secure-boot/)

Hardware requirements:

This attribute will only be available when using AMD CPUs.

### [AMD Secure Processor Rollback protection](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Amd.RollbackProtection)https://fwupd.github.io/libfwupdplugin/hsi.html#amd-secure-processor-rollback-protection

AMD SOCs include the ability to prevent a rollback attack by a rollback protection feature on the secure processor.

This feature prevents an attacker from loading an older firmware onto the part after a security vulnerability has been fixed.

Impact:

SOCs without this feature may be attacked by an attacker installing an older firmware that takes advantage of a well-known vulnerability.

Possible results:

- not-enabled: rollback protection disabled (failure)
- enabled: rollback protection enabled (success)

A test success result is needed to meet HSI-4 on systems that run this test. [v1.8.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [Rollback protection](https://www.psacertified.org/blog/anti-rollback-explained/)
- [AMD Secure Processor](https://www.amd.com/en/technologies/pro-security)
- [Loading OS Optimized Defaults on Lenovo systems](https://forums.lenovo.com/t5/Fedora/AMD-Rollback-protection-not-detected-by-fwupd-on-T14-G3-AMD/m-p/5182708?page=1#5810366)

Hardware requirements:

This attribute will only be available when using AMD CPUs.

More information:

This particular check is not for the Microsoft Pluton Security processor which is present on some chips.

End users are not able to directly modify rollback protection, this is controlled by the manufacturer.

On Lenovo systems it has been reported that if this is disabled it may potentially be enabled by loading ‘OS Optimized Defaults’ in BIOS setup.

### [SMM locked down](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Amd.SmmLocked)https://fwupd.github.io/libfwupdplugin/hsi.html#smm-locked-down

Disable SMM_BASE from being saved to and restored from the SMM save state area.

Code in the ASeg and TSeg range and the SMM registers are Read-only and SMI interrupts are not intercepted in SVM.

This check also ensures that the system is not affected by the Sinkclose vulnerability.

Impact:

An attacker with an existing unrelated vulnerability can the SMM save state area to exfiltrate data.

Possible results:

- not-locked: SMM is not locked down (failure)
- not-valid: Vulnerable to Sinkclose (failure)
- locked: SMM is locked down (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v2.0.2]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [AMD Sinkclose](https://www.amd.com/en/resources/product-security/bulletin/amd-sb-7014.html)

### [AMD SPI Replay protections](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Amd.SpiReplayProtection)https://fwupd.github.io/libfwupdplugin/hsi.html#amd-spi-replay-protections

SOCs may include support for replay-protected monotonic counters to prevent replay attacks.

Impact:

SOCs without this feature may be attacked by an attacker modifying the SPI.

Possible results:

- not-enabled: SPI protections disabled (failure)
- enabled: SPI protections enabled (success)

A test success result is needed to meet HSI-3 on systems that run this test. [v1.8.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

Hardware requirements:

This attribute will only be available when using AMD CPUs.

### [AMD SPI Write protections](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Amd.SpiWriteProtection)https://fwupd.github.io/libfwupdplugin/hsi.html#amd-spi-write-protections

SOCs may enforce control of the SPI bus to prevent writes other than by verified entities.

Impact:

SOCs without this feature may be attacked by an attacker modifying the SPI.

Possible results:

- not-enabled: SPI protections disabled (failure)
- enabled: SPI protections enabled (success)

A test success result is needed to meet HSI-2 on systems that run this test. [v1.8.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

Hardware requirements:

This attribute will only be available when using AMD CPUs.

### [BIOS Capsule updates](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Bios.CapsuleUpdates)https://fwupd.github.io/libfwupdplugin/hsi.html#bios-capsule-updates

Some OEMs offer a switch in the BIOS to allow capsule updates to be enabled.

Impact:

When this switch is present but disabled, BIOS firmware updates for Linux may not work.

Possible results:

- not-enabled: BIOS capsule updates disabled (failure)
- enabled: BIOS capsule updates enabled (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.9.6]

Resolution: Turn on ‘firmware updates’ in the UEFI firmware setup.

### [BIOS Firmware Rollback protection](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Bios.RollbackProtection)https://fwupd.github.io/libfwupdplugin/hsi.html#bios-firmware-rollback-protection

Some OEMs include an optional firmware protection feature in their BIOS that would prevent installation of older firmware that may have security vulnerabilities.

Impact:

Firmware without this feature enabled may be attacked by an attacker installing an older firmware that takes advantage of a well-known vulnerability.

Possible results:

- not-enabled: rollback protection disabled (failure)
- enabled: rollback protection enabled (success)

A test success result is needed to meet HSI-2 on systems that run this test. [v1.8.8]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [Rollback protection](https://www.psacertified.org/blog/anti-rollback-explained/)

### [CET Utilized by OS](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Cet.Active)https://fwupd.github.io/libfwupdplugin/hsi.html#cet-utilized-by-os

Control enforcement technology prevents exploits from hijacking the control-flow transfer instructions for both forward-edge (indirect call/jmp) and back-edge transfer (ret).

Impact:

A local or physical attacker with an existing unrelated vulnerability can use a ROP gadget to run arbitrary code.

Possible results:

- not-supported: CET not being used by the host (failure)
- supported: CET being used (success)

A test success result is needed to meet HSI-3 on systems that run this test. [v2.0.0]

Resolution: Update your Linux distribution, which may now support CET.

References:

- [Intel CET Technology Preview](https://software.intel.com/sites/default/files/managed/4d/2a/control-flow-enforcement-technology-preview.pdf)

### [CET: Available](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Cet.Enabled)https://fwupd.github.io/libfwupdplugin/hsi.html#cet-available

Control enforcement technology prevents exploits from hijacking the control-flow transfer instructions for both forward-edge (indirect call/jmp) and back-edge transfer (ret).

Impact:

A local or physical attacker with an existing unrelated vulnerability can use a reliable and well-known method to run arbitrary code.

Possible results:

- not-supported: CET not supported (failure)
- enabled: CET feature enabled by the platform (success)
- supported: CET feature support by the platform (success)

A test success result is needed to meet HSI-3 on systems that run this test. [v2.0.0]

Resolution: Check your CPU supports CET, and ensure the feature is enabled in the UEFI setup screen.

References:

- [Intel CET Technology Preview](https://software.intel.com/sites/default/files/managed/4d/2a/control-flow-enforcement-technology-preview.pdf)

### [DRAM memory encryption](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.EncryptedRam)https://fwupd.github.io/libfwupdplugin/hsi.html#dram-memory-encryption

TME (Intel) or SME (AMD) is used by the hardware on supported SOCs to encrypt all data on external memory buses.

It mitigates against an attacker being able to capture memory data while the system is running or to capture memory by removing a DRAM chip.

This encryption may be activated by either transparently via firmware configuration or by code running in the Linux kernel.

Impact:

A local attacker can either extract unencrypted content by attaching debug probes on the DIMM modules, or by removing them and inserting them into a computer with a modified DRAM controller.

Possible results:

- not-encrypted: detected but disabled (failure)
- not-supported: not available (failure)
- encrypted: detected and enabled (success)

A test success result is needed to meet HSI-4 on systems that run this test. [v1.5.0]

Resolution: Ensure that the motherboard and CPU support this feature, and ensure it is enabled in the UEFI setup screen.

References:

- [Intel TME Press Release](https://software.intel.com/content/www/us/en/develop/blogs/intel-releases-new-technology-specification-for-memory-encryption.html)
- [WikiChip SME Overview](https://en.wikichip.org/wiki/x86/sme)

### [HP SureStart](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Hp.Surestart)https://fwupd.github.io/libfwupdplugin/hsi.html#hp-surestart

HP SureStart is an advanced self-healing BIOS that automatically recovers the BIOS code if it becomes corrupted or is compromised by malicious software.

When enabled, SureStart monitors the BIOS for unauthorized changes and can automatically restore the BIOS to a known-good state.

This provides runtime intrusion prevention and detection for the firmware, protecting against both malware and physical attacks on the BIOS chip.

Impact:

Without HP SureStart enabled, the system firmware is vulnerable to persistent malware that could survive operating system reinstallation.

An attacker who compromises the BIOS could maintain persistent access to the system and potentially bypass security features like Secure Boot.

Physical attacks on the BIOS chip could result in permanent system compromise without detection.

Possible results:

- not-enabled: HP SureStart is disabled (failure)
- enabled: HP SureStart is enabled (success)

A test success result is needed to meet HSI-2 on systems that run this test. [v2.1.1]

References:

- [HP SureStart White Paper](https://h20195.www2.hp.com/v2/GetPDF.aspx/4aa6-4194enw.pdf)

### [Intel BootGuard: ACM](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.IntelBootguard.Acm)https://fwupd.github.io/libfwupdplugin/hsi.html#intel-bootguard-acm

BootGuard is a processor feature that prevents the machine from running firmware images not released by the system manufacturer.

It forms a root-of-trust by fusing in cryptographic keys into the processor itself that are used to verify the Authenticated Code Modules found in the SPI flash.

Impact:

When BootGuard is not set up correctly then the chain-of-trust between the CPU and the bootloader can not be verified.

Possible results:

- not-valid: boot is not verified (failure)
- valid: ACM protected (success)

A test success result is needed to meet HSI-2 on systems that run this test. [v1.5.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

Hardware requirements:

This attribute will only be available when using Intel CPUs.

### [Intel BootGuard: Enabled](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.IntelBootguard.Enabled)https://fwupd.github.io/libfwupdplugin/hsi.html#intel-bootguard-enabled

BootGuard is a processor feature that prevents the machine from running firmware images not released by the system manufacturer.

It forms a root-of-trust by fusing in cryptographic keys into the processor itself that are used to verify the Authenticated Code Modules found in the SPI flash.

Impact:

When BootGuard is not set up correctly then the chain-of-trust between the CPU and the bootloader can not be verified.

This would allow subverting the Secure Boot protection which gives the attacker full access to your hardware.

Possible results:

- not-enabled: not detected, or detected but not enabled (failure)
- enabled: detected and enabled (success)

A test success result is needed to meet HSI-2 on systems that run this test. [v1.5.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [Coreboot documentation](https://github.com/coreboot/coreboot/blob/master/src/soc/intel/jasperlake/include/soc/me.h)

Hardware requirements:

This attribute will only be available when using Intel CPUs.

### [Intel BootGuard: OTP](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.IntelBootguard.Otp)https://fwupd.github.io/libfwupdplugin/hsi.html#intel-bootguard-otp

BootGuard is a processor feature that prevents the machine from running firmware images not released by the system manufacturer.

It forms a root-of-trust by fusing in cryptographic keys into the processor itself that are used to verify the Authenticated Code Modules found in the SPI flash.

Impact:

When BootGuard is not set up correctly then the chain-of-trust between the CPU and the bootloader can not be verified.

This would allow subverting the Secure Boot protection which gives the attacker full access to your hardware.

Possible results:

- not-valid: SOC is not locked (failure)
- valid: SOC is locked (success)

A test success result is needed to meet HSI-2 on systems that run this test. [v1.5.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

Hardware requirements:

This attribute will only be available when using Intel CPUs.

### [Intel BootGuard: Policy](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.IntelBootguard.Policy)https://fwupd.github.io/libfwupdplugin/hsi.html#intel-bootguard-policy

BootGuard is a processor feature that prevents the machine from running firmware images not released by the system manufacturer.

It forms a root-of-trust by fusing in cryptographic keys into the processor itself that are used to verify the Authenticated Code Modules found in the SPI flash.

Impact:

The attacker can invalidate the chain of trust (subverting Secure Boot), and the user would get just a console warning and then continue to boot.

Possible results:

- not-valid: policy is invalid (failure)
- valid: error enforce policy is set to shutdown (success)

A test success result is needed to meet HSI-3 on systems that run this test. [v1.5.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

Hardware requirements:

This attribute will only be available when using Intel CPUs.

### [Intel BootGuard: Verified](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.IntelBootguard.Verified)https://fwupd.github.io/libfwupdplugin/hsi.html#intel-bootguard-verified

BootGuard is a processor feature that prevents the machine from running firmware images not released by the system manufacturer.

It forms a root-of-trust by fusing in cryptographic keys into the processor itself that are used to verify the Authenticated Code Modules found in the SPI flash.

Impact:

When BootGuard is not set up correctly then the chain-of-trust between the CPU and the bootloader can not be verified.

This would allow subverting the Secure Boot protection which gives the attacker full access to your hardware.

Possible results:

- not-valid: boot is not verified (failure)
- success: verified boot chain (success)

A test success result is needed to meet HSI-2 on systems that run this test. [v1.5.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

Hardware requirements:

This attribute will only be available when using Intel CPUs.

### [Intel GDS Mitigation](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.IntelGds)https://fwupd.github.io/libfwupdplugin/hsi.html#intel-gds-mitigation

CPU Microcode must be updated to mitigate against an information-disclosure security issue called Gather Data Sampling.

Impact:

Firmware without this feature enabled allow an attacker to exfiltrate secrets by running a malicious program or script.

Possible results:

- not-valid: microcode is not new enough to support required mitigations (failure)
- not-enabled: mitigation is not enabled (failure)
- not-locked: mitigation is not locked (failure)
- enabled: microcode mitigation is supported, enabled and locked (success)

A test success result is needed to meet HSI-2 on systems that run this test. [v1.9.4]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [Gather Data Sampling](https://www.intel.com/content/www/us/en/developer/articles/technical/software-security-guidance/technical-documentation/gather-data-sampling.html)

### [DMA protection](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Iommu)https://fwupd.github.io/libfwupdplugin/hsi.html#dma-protection

The IOMMU on modern systems is used to mitigate against DMA attacks.

All I/O for devices capable of DMA is mapped into a private virtual memory region.

Common implementations are Intel VT-d and AMD-Vi.

Impact:

An attacker with inexpensive PCIe development hardware can write to system RAM from the ThunderBolt or Firewire ports which can lead to privilege escalation.

Possible results:

- not-found: IOMMU hardware was not detected (failure)
- enabled: IOMMU hardware detected and enabled (success)

A test success result is needed to meet HSI-2 on systems that run this test. [v1.5.0]

Resolution: If available, turn on IOMMU in the system BIOS. You may also have to use additional kernel boot parameters, for example iommu=force.

References:

- [IOMMU Wikipedia Page](https://en.wikipedia.org/wiki/Input%E2%80%93output_memory_management_unit)

### [Kernel Lockdown](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Kernel.Lockdown)https://fwupd.github.io/libfwupdplugin/hsi.html#kernel-lockdown

Kernel lockdown is an important mechanism to limit what hardware actions userspace programs can perform.

Turning on this feature means that often-used mechanisms like /dev/mem used to raise privileges or exfiltrate data are no longer available.

Impact:

An unlocked kernel can be easily abused by a malicious userspace program running as root, which can include replacing system firmware.

Possible results:

- not-valid: could not read lockdown status, perhaps from an old kernel (failure)
- not-enabled: lockdown is set to none (failure)
- enabled: lockdown is set to either integrity or confidentiality. (success)

Resolution: Ensure Secure Boot is enabled in the UEFI firmware setup.

### [Kernel Tainted](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Kernel.Tainted)https://fwupd.github.io/libfwupdplugin/hsi.html#kernel-tainted

When calculating the HSI value fwupd has to ask the Linux Kernel for information.

If the kernel has been tainted by overriding a firmware table or by loading an out-of-tree module then we cannot trust the data it reports.

Impact:

Using a tainted kernel means that values obtained from the kernel cannot be trusted.

Possible results:

- not-valid: could not detect kernel taint status (failure)
- tainted: the kernel is untrusted, perhaps because a proprietary module was loaded (failure)
- not-tainted: the kernel is trusted (success)

Resolution: Remove any non-free kernel modules.

### [ME BootGuard Platform Key](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Mei.KeyManifest)https://fwupd.github.io/libfwupdplugin/hsi.html#me-bootguard-platform-key

The BootGuard Platform Key is fused into the CPU PCH during manufacturing by the OEM.

At boot-up, an authenticated code module computes a hash of the Platform Key and compares it with the one stored in field-programmable fuses.

If the key matches the ACM will pass control to the firmware, otherwise the boot process will stop.

In 2022 a number of Platform secret Keys were leaked by Lenovo and confirmed by Intel.

Impact:

A custom system firmware can be signed using the leaked private key to completely disable UEFI Secure Boot and allow complete persistent compromise of the affected machine.

Possible results:

- not-valid: device uses a key that is compromised (failure)
- valid: device uses a BootGuard Platform Key that is not known to be compromised (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.8.7]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [Intel leak checker](https://github.com/phretor/intel-leak-checker/)
- [Tom’s Hardware Article](https://www.tomshardware.com/news/intel-confirms-6gb-alder-lake-bios-source-code-leak-new-details-emerge)

Hardware requirements:

This attribute will only be available when using Intel CPUs.

### [ME not in manufacturing mode](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Mei.ManufacturingMode)https://fwupd.github.io/libfwupdplugin/hsi.html#me-not-in-manufacturing-mode

There have been some unfortunate cases of the ME being distributed in manufacturing mode.

In manufacturing mode many features from the ME can be interacted with that decrease the platform’s security.

Impact:

If the ME is in manufacturing mode then any user with root access can provision the ME engine with new keys.

This gives them full access to the system even when the system is powered off.

Possible results:

- not-locked: device is in manufacturing mode (failure)
- locked: device has had manufacturing mode disabled (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.5.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [ME Manufacturing Mode: obscured dangers](https://malware.news/t/intel-me-manufacturing-mode-obscured-dangers-and-their-relationship-to-apple-macbook-vulnerability-cve-2018-4251/23214)
- [Intel security advisory SA-00086](https://www.intel.com/content/www/us/en/security-center/advisory/intel-sa-00086.html)

Hardware requirements:

This attribute will only be available when using Intel CPUs.

### [ME Flash Descriptor Override](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Mei.OverrideStrap)https://fwupd.github.io/libfwupdplugin/hsi.html#me-flash-descriptor-override

The Flash Descriptor Security Override Strap is not accessible to end users on consumer boards and Intel stresses that this is for debugging only.

Impact:

The system firmware can be written from userspace by changing the protected region.

This gives any attacker with root access a method to write persistent executable code to the firmware, which survives even a full disk wipe and OS reinstall.

Possible results:

- not-locked: device is in debugging mode (failure)
- locked: device in in normal runtime mode (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.5.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [Chromium documentation for Intel ME](https://chromium.googlesource.com/chromiumos/third_party/flashrom/+/master/Documentation/mysteries_intel.txt)

Hardware requirements:

This attribute will only be available when using Intel CPUs.

### [CSME Version](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Mei.Version)https://fwupd.github.io/libfwupdplugin/hsi.html#csme-version

Converged Security and Manageability Engine is a standalone management module that can manage and control some local devices without the host CPU involvement.

The CSME lives in the PCH and can only be updated by the OEM vendor.

The version of the CSME module can be checked to detect the most common and serious vulnerabilities.

Impact:

Using any one of the critical vulnerabilities, a remote attacker can take full control of the system and all connected devices, even when the system is powered off.

Possible results:

- not-valid: affected by one of the critical CVEs (failure)
- valid: is not affected by the most critical CVEs (success)
- not-enabled: has been disabled using HAP (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.5.0]

Resolution: Update your Management Engine firmware

Issues:

- [CVE-2017-5705](https://nvd.nist.gov/vuln/detail/CVE-2017-5705)
- [CVE-2017-5706](https://nvd.nist.gov/vuln/detail/CVE-2017-5706)
- [CVE-2017-5707](https://nvd.nist.gov/vuln/detail/CVE-2017-5707)
- [CVE-2017-5708](https://nvd.nist.gov/vuln/detail/CVE-2017-5708)
- [CVE-2017-5709](https://nvd.nist.gov/vuln/detail/CVE-2017-5709)
- [CVE-2017-5710](https://nvd.nist.gov/vuln/detail/CVE-2017-5710)
- [CVE-2017-5711](https://nvd.nist.gov/vuln/detail/CVE-2017-5711)
- [CVE-2017-5712](https://nvd.nist.gov/vuln/detail/CVE-2017-5712)

References:

- [Intel CSME Security Review Cumulative Update](https://www.intel.com/content/www/us/en/security-center/advisory/intel-sa-00086.html)

Hardware requirements:

This attribute will only be available when using Intel CPUs.

### [Intel DCI](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.PlatformDebugEnabled)https://fwupd.github.io/libfwupdplugin/hsi.html#intel-dci

Newer Intel CPUs support debugging over USB3 via a proprietary Direct Connection Interface (DCI) with the use of off-the-shelf hardware.

Impact:

Using DCI an attacker with physical access to the computer has full access to all registers and memory in the system, and is able to make changes.

This makes privilege escalation from user to root possible, and also modifying SMM makes it possible to write to system firmware for a persistent backdoor.

Possible results:

- enabled: debugging is currently enabled (failure)
- not-enabled: debugging is not currently enabled (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.5.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [Intel Direct Connect Interface](https://www.intel.co.uk/content/www/uk/en/support/articles/000029393/processors.html)
- [Chipsec 4xxlp register definitions](https://github.com/chipsec/chipsec/blob/master/chipsec/cfg/8086/pch_4xxlp.xml#L270)
- [RISC-V EDK PCH register definitions](https://github.com/riscv/riscv-edk2-platforms/blob/85a50de1b459d1d6644a402081120770aa6dd8c7/Silicon/Intel/CoffeelakeSiliconPkg/Pch/Include/Register/PchRegsDci.h)

More information:

This attribute was previously known as org.fwupd.hsi.IntelDci.Enabled in 1.5.0, but was renamed in 1.8.0 to support other vendors.

### [Part is debug locked](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.PlatformDebugLocked)https://fwupd.github.io/libfwupdplugin/hsi.html#part-is-debug-locked

Some devices support a concept of whether a part has been unlocked for debugging using proprietary hardware. Such parts allow access to registers that are typically restricted when parts are fused.

On Intel systems access to this interface is done via a proprietary Direct Connection Interface (DCI).

Impact:

If using a debug unlocked part, the platform’s overall security will be decreased as an attacker may have elevated access to registers and memory within the system and can potentially enable persistent backdoors.

Possible results:

- not-locked: device is not locked (failure)
- locked: device is locked (success)

A test success result is needed to meet HSI-2 on systems that run this test. [v1.8.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [Intel Direct Connect Interface](https://www.intel.co.uk/content/www/uk/en/support/articles/000029393/processors.html)

### [Part is fused](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.PlatformFused)https://fwupd.github.io/libfwupdplugin/hsi.html#part-is-fused

When fuses are blown in parts from some manufacturers the hardware will enforce protections against tampering or accessing of certain registers.

Impact:

If using an unfused part, the platform’s overall security will be decreased.

Possible results:

- not-locked: device is not fused (failure)
- locked: device is fused (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.8.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

### [Pre-boot DMA protection](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.PrebootDma)https://fwupd.github.io/libfwupdplugin/hsi.html#pre-boot-dma-protection

The IOMMU on modern systems is used to mitigate against DMA attacks.

All I/O for devices capable of DMA is mapped into a private virtual memory region.

On Intel systems the ACPI DMAR table indicated the system is configured with pre-boot DMA protection which eliminates some firmware attacks.

On AMD systems the ACPI IVRS table indicates the same.

Impact:

An attacker could connect a malicious peripheral using ThunderBolt and reboot the machine, which would allow the attacker to modify the system memory.

This would allow subverting the Secure Boot protection, and also invalidate any system attestation.

Possible results:

- not-valid: could not determine state (failure)
- not-enabled: was not enabled (failure)
- enabled: detected correctly (success)

A test success result is needed to meet HSI-3 on systems that run this test. [v1.8.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [IOMMU Wikipedia Page](https://en.wikipedia.org/wiki/Input%E2%80%93output_memory_management_unit)
- [AMD I/O Virtualization Technology (IOMMU) Specification](https://www.amd.com/en/support/tech-docs/amd-io-virtualization-technology-iommu-specification)

More information:

This attribute was previously known as org.fwupd.hsi.AcpiDmar in 1.5.0, but was renamed in 1.8.0 to support other vendors.

### [SMAP](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Smap)https://fwupd.github.io/libfwupdplugin/hsi.html#smap

Without Supervisor Mode Access Prevention, the supervisor code usually has full read and write access to user-space memory mappings.

This can make exploits easier to write, as it allows the kernel to access user-space memory when it did not intend to.

Impact:

A local or remote attacker can use a simple exploit to modify the contents of kernel memory which can lead to privilege escalation.

Possible results:

- not-supported: SMAP not enabled (failure)
- enabled: SMAP features are detected and enabled (success)

A test success result is needed to meet HSI-4 on systems that run this test. [v2.0.0]

Resolution: Ensure that the CPU supports this feature, and then contact your OEM, who may be able to issue a firmware update.

References:

- [SMAP Wikipedia Page](https://en.wikipedia.org/wiki/Supervisor_Mode_Access_Prevention)

### [BIOS Write Enable (BWE)](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Spi.Bioswe)https://fwupd.github.io/libfwupdplugin/hsi.html#bios-write-enable-bwe

Intel hardware provides this mechanism to protect the SPI ROM chip located on the motherboard from being overwritten by the operating system.

The BIOSWE bit must be unset otherwise userspace can write to the SPI chip.

Impact:

The system firmware can be written from userspace.

This gives any attacker with root access a method to write persistent executable code to the firmware, which survives even a full disk wipe and OS reinstall.

Possible results:

- not-found: the SPI device was not found (failure)
- enabled: write enable is enabled (failure)
- not-enabled: write enable is disabled (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.5.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [Intel C200 Datasheet](https://www.intel.com/content/dam/www/public/us/en/documents/datasheets/6-chipset-c200-chipset-datasheet.pdf)

Hardware requirements:

This attribute will only be available when using Intel CPUs.

### [BIOS Lock Enable (BLE)](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Spi.Ble)https://fwupd.github.io/libfwupdplugin/hsi.html#bios-lock-enable-ble

If the lock bit is set then System Management Interrupts (SMIs) are raised when setting BIOS Write Enable.

The BLE bit must be enabled in the PCH otherwise BIOSWE can easily be unset.

Impact:

The system firmware can be written from userspace.

This gives any attacker with root access a method to write persistent executable code to the firmware, which survives even a full disk wipe and OS reinstall.

Possible results:

- not-enabled: the register is not locked (failure)
- enabled: the register is locked (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.5.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [Intel C200 Datasheet](https://www.intel.com/content/dam/www/public/us/en/documents/datasheets/6-chipset-c200-chipset-datasheet.pdf)

Hardware requirements:

This attribute will only be available when using Intel CPUs.

### [Read-only SPI Descriptor](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Spi.Descriptor)https://fwupd.github.io/libfwupdplugin/hsi.html#read-only-spi-descriptor

The SPI descriptor must always be read only from all other regions.

Additionally on Intel architectures the FLOCKDN register must be set to prevent configuration registers in the SPI BAR from being changed.

Impact:

The system firmware can be written from userspace by changing the protected region.

This gives any attacker with root access a method to write persistent executable code to the firmware, which survives even a full disk wipe and OS reinstall.

Possible results:

- not-valid: any region can write to the flash descriptor (failure)
- not-locked: the SPI BAR is not locked (failure)
- locked: the SPI BAR is locked and read only from all regions (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.6.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

Hardware requirements:

This attribute will only be available when using Intel CPUs.

### [SMM Bios Write Protect (SMM_BWP)](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Spi.SmmBwp)https://fwupd.github.io/libfwupdplugin/hsi.html#smm-bios-write-protect-smm_bwp

This bit set defines when the BIOS region can be written by the host.

The SMM_BWP bit must be set to make the BIOS region non-writable unless all processors are in system management mode.

Impact:

The system firmware can be written from userspace by exploiting a race condition in checking BLE.

This gives any attacker with root access a method to write persistent executable code to the firmware, which survives even a full disk wipe and OS reinstall.

Possible results:

- not-locked: the region is not locked (failure)
- locked: the region is locked (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.5.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [Intel C200 Datasheet](https://www.intel.com/content/dam/www/public/us/en/documents/datasheets/6-chipset-c200-chipset-datasheet.pdf)

Hardware requirements:

This attribute will only be available when using Intel CPUs.

### [Supported CPU](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.SupportedCpu)https://fwupd.github.io/libfwupdplugin/hsi.html#supported-cpu

Most platform checks are specific to the CPU vendor.

To avoid giving a very high HSI result for a platform we do not know how to verify, we include this attribute to ensure that the result is meaningful.

Impact:

If using an unsupported CPU then fwupd is unable to verify the platform security.

You should contact your platform vendor and ask them to contribute HSI tests for this CPU type.

Possible results:

- unknown: platform security is unknown (failure)
- valid: the CPU platform is supported and has HSI tests (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.8.0]

Resolution: Contact the silicon vendor and ask them to submit new HSI tests for fwupd.

More information:

On AMD APUs or CPUs this information is reported on kernel 5.19 or later via the ccp kernel module.

If the kernel module is enabled but is not being auto-loaded, this is a kernel bug and should be reported to kernel bugzilla.

If the kernel module has loaded but you still don’t have data this is NOT a fwupd bug. You will have to contact

your motherboard or system manufacturer to enable reporting this information.

### [Suspend-to-Idle](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.SuspendToIdle)https://fwupd.github.io/libfwupdplugin/hsi.html#suspend-to-idle

The platform should be set up with Suspend-to-Idle as the default S3 sleep state.

Impact:

A local attacker could overwrite the S3 resume script to modify system RAM which can lead to privilege escalation.

Possible results:

- not-enabled: deep sleep enabled (failure)
- not-valid: could not determine the default (failure)
- enabled: suspend-to-idle being used (success)
- not-supported: not applicable for this platform (success)

A test success result is needed to meet HSI-3 on systems that run this test. [v1.5.0]

Resolution: Change this setting in the UEFI setup screen, if supported.

### [Suspend to RAM disabled](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.SuspendToRam)https://fwupd.github.io/libfwupdplugin/hsi.html#suspend-to-ram-disabled

Suspend to Ram (S3) keeps the raw contents of the DRAM refreshed when the system is asleep.

This means that the memory modules can be physically removed and the contents recovered, or a cold boot attack can be performed with a USB device.

The firmware should be configured to prefer using suspend to idle instead of suspend to ram or to not offer suspend to RAM.

Impact:

An attacker with physical access to a system can obtain the un-encrypted contents of the RAM by suspending the machine, removing the DIMM and inserting it into another machine with modified DRAM controller before the memory contents decay.

Possible results:

- enabled: sleep enabled (failure)
- not-valid: could not determine the default (failure)
- not-enabled: suspend-to-ram being used (success)

A test success result is needed to meet HSI-3 on systems that run this test. [v1.5.0]

Resolution: Enable this setting in the UEFI setup screen, if supported.

References:

- [Cold Boot Attack Wikipedia Page](https://en.wikipedia.org/wiki/Cold_boot_attack)

### [Empty PCR in TPM](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Tpm.EmptyPcr)https://fwupd.github.io/libfwupdplugin/hsi.html#empty-pcr-in-tpm

The system firmware is responsible for measuring values about its boot stage in PCRs 0 through 7.

Some firmwares have bugs that prevent them from measuring some of those values, breaking the fundamental assumption of the Measured Boot chain-of-trust.

Impact:

A local attacker could measure fake values into the empty PCR, corresponding to a firmware and OS that do not match the ones actually loaded.

This allows hiding a compromised boot chain or fooling a remote-attestation server into believing that a different kernel is running.

Possible results:

- not-found: no TPM hardware could be found (failure)
- not-valid: at least one empty checksum has been found (failure)
- valid: all PCRs from 0 to 7 must have non-empty measurements (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.7.2]

Resolution: Contact your OEM, who may be able to issue a firmware update.

Issues:

- [CVE-2021-42299](https://nvd.nist.gov/vuln/detail/CVE-2021-42299)

References:

- [TPM Carte Blanche](https://github.com/google/security-research/blob/master/pocs/bios/tpm-carte-blanche/writeup.md)

### [PCR0 TPM Event Log Reconstruction](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Tpm.ReconstructionPcr0)https://fwupd.github.io/libfwupdplugin/hsi.html#pcr0-tpm-event-log-reconstruction

The TPM event log records which events are registered for the PCR0 hash.

When reconstructed the event log values should always match the TPM PCR0.

If extra events are included in the event log, or some are missing, the reconstitution will fail.

Impact:

This is not a vulnerability per-se, but it shows that the system firmware checksum cannot be verified as the PCR result has been calculated incorrectly.

Possible results:

- not-valid: could not reconstitute the hash value (failure)
- not-found: no TPM hardware could be found (failure)
- valid: all correct (success)

A test success result is needed to meet HSI-2 on systems that run this test. [v1.5.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [Linux Kernel TPM Documentation](https://www.kernel.org/doc/html/latest/security/tpm/tpm_event_log.html)

More information:

Additional information about specific bugs and debugging steps are available here https://github.com/fwupd/fwupd/wiki/TPM-PCR0-differs-from-reconstruction

### [TPM 2.0 Present](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Tpm.Version20)https://fwupd.github.io/libfwupdplugin/hsi.html#tpm-20-present

A TPM securely stores platform specific secrets that can only be divulged to trusted consumers in a secure environment.

Impact:

The PCR registers will not be available for use by the bootloader and kernel.

This means userspace cannot either encrypt disks to the specific machine, and also can’t know if the system firmware was externally modified.

Possible results:

- not-found: no TPM device found (failure)
- not-enabled: TPM not in v2 mode (failure)
- found: TPM device found in v2 mode (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.5.0]

Resolution: Enable this setting in the UEFI setup screen, if supported.

References:

- [TPM Wikipedia Page](https://en.wikipedia.org/wiki/Trusted_Platform_Module)

### [UEFI BootService Variables](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Uefi.BootserviceVars)https://fwupd.github.io/libfwupdplugin/hsi.html#uefi-bootservice-variables

UEFI boot service variables should not be readable from runtime mode.

Impact:

It is possible to read security-sensitive data that should not be readable by the runtime mode.

Possible results:

- not-locked: bootservice-only data is readable in runtime mode (failure)
- locked: bootservice-only data is not visible (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.9.3]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [UEFI Specification](https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html)

### [UEFI db](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Uefi.Db)https://fwupd.github.io/libfwupdplugin/hsi.html#uefi-db

The UEFI db contains the list of valid certificates that is used to authorize what EFI binaries are allowed to run.

Impact:

Distribution-provided signed bootloader binaries may not boot on any system that does not have the new Microsoft UEFI certificate installed in the db.

Possible results:

- not-valid: the certificate store is not up to date (failure)
- not-found: the certificate store does not contain any Microsoft UEFI certificate (success)
- valid: the certificate store has the Microsoft UEFI 2023 certificate installed (success)

Resolution: Apply the LVFS KEK update for your hardware, and then apply the LVFS db update.

References:

- [fwupd Documentation for UEFI Secure Boot Certificates](https://fwupd.github.io/libfwupdplugin/uefi-db.html)
- [Ubuntu SecureBoot Wiki Page](https://wiki.ubuntu.com/UEFI/SecureBoot/Testing)

### [Early-boot UEFI Memory Protections](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Uefi.MemoryProtection)https://fwupd.github.io/libfwupdplugin/hsi.html#early-boot-uefi-memory-protections

The UEFI system can set up memory attributes at boot which prevent common exploits from running. For instance, the stack can be marked as non-executable to prevent running any malicious code that can be written to the stack.

In the same way, writing to sections that are read-only can also be prevented, making the system more secure from attackers.

Distribution vendors should tag the bootloader as NX compatible, and BIOS vendors should provide a ‘memory attribute protocol’ table that tags memory sections with specific properties.

Microsoft Virtualization-based security (VBS) also requires NX protection for UEFI runtime services for DeviceGuard compliance.

Impact:

The attacker is able to run arbitrary executable code from the stack, or write to memory that is supposed to be read-only.

Possible results:

- not-locked: memory is still executable or writable (failure)
- not-enabled: the bootloader is not marked as NX compatible and the firmware may be operating in a compatibility mode (failure)
- locked: memory is not executable or writable (success)

A test success result is needed to meet HSI-3 on systems that run this test. [v2.0.7]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [Microsoft Windows WHCP](https://aka.ms/WHCP)

### [UEFI PK](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Uefi.Pk)https://fwupd.github.io/libfwupdplugin/hsi.html#uefi-pk

UEFI defines a platform key for the system.

This should not be a test key, e.g. DO NOT TRUST - AMI Test PK

Impact:

It is possible to sign an EFI binary with the test platform key, which invalidates the Secure Boot trust chain.

It effectively gives the local attacker full access to your hardware.

Possible results:

- not-valid: an invalid key has been enrolled (failure)
- valid: valid key (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.5.0]

Resolution: Contact your OEM, who may be able to issue a firmware update.

References:

- [Ubuntu SecureBoot Wiki Page](https://wiki.ubuntu.com/UEFI/SecureBoot/Testing)

### [UEFI SecureBoot](https://fwupd.github.io/libfwupdplugin/hsi.html#org.fwupd.hsi.Uefi.SecureBoot)https://fwupd.github.io/libfwupdplugin/hsi.html#uefi-secureboot

UEFI Secure boot is a verification mechanism for ensuring that code launched by firmware is trusted.

Secure Boot requires that each binary loaded at boot is validated against trusted certificates.

Impact:

When Secure Boot is not enabled any EFI binary can be run at startup, which gives the attacker full access to your hardware.

Possible results:

- not-found: support has not been detected (failure)
- not-enabled: detected, but has been turned off (failure)
- enabled: supported and enabled (success)

A test success result is needed to meet HSI-1 on systems that run this test. [v1.5.0]

Resolution: Turn off CSM boot and enable Secure Boot in the BIOS setup.

References:

- [Ubuntu SecureBoot Wiki Page](https://wiki.ubuntu.com/UEFI/SecureBoot)

## [Conclusion](https://fwupd.github.io/libfwupdplugin/hsi.html#conclusions)https://fwupd.github.io/libfwupdplugin/hsi.html#conclusion

Any system with a Host Security ID of 0 can easily be modified from userspace. PCs with confidential documents should have a HSI:3 or higher level of protection. In a graphical tool that would show details about the computer (such as GNOME Control Center’s details tab) the OS could display a field indicating Host Security ID. The ID should be shown with an alert color if the security is not at least HSI:1 or the suffix is !.

On Linux fwupd is used to enumerate and update firmware. It exports a property HostSecurityId and a GetHostSecurityAttrs() method. The attributes are supposed to represent the system as a whole but individual (internal) devices are able to make a claim that they worsened the state of the security of the system. Certain attributes can “obsolete” other attributes. An example is BIOSGuard will set obsoletes to org.intel.prx.

A plugin method gets called on each plugin which adds attributes directly from the hardware or kernel. Several attributes may be dependent upon the kernel performing measurements and it will take time for these to be upstreamed. In some cases security level measurements will only be possible on systems with a newer kernel.

The long term goal is to increase the HSI:x level of systems being sold to consumers. By making some of the HSI:x attributes part of the LVFS uploaded report we can allow users to compare vendors and models before purchasing hardware.

## [Intentional Omissions](https://fwupd.github.io/libfwupdplugin/hsi.html#omissions)https://fwupd.github.io/libfwupdplugin/hsi.html#intentional-omissions

### Intel SGXhttps://fwupd.github.io/libfwupdplugin/hsi.html#intel-sgx

This is not widely used as it has several high severity security issues.

### Intel MPXhttps://fwupd.github.io/libfwupdplugin/hsi.html#intel-mpx

MPX support was removed from GCC and the Linux kernel in 2019 and it is now considered obsolete.

## Further Workhttps://fwupd.github.io/libfwupdplugin/hsi.html#further-work

More internal and external devices should be factored into the security equation. For now the focus for further tests should be around internal device firmware as it is what can be most directly controlled by fwupd and the hardware manufacturer.

Security conscious manufacturers are actively participating in the development of future initiatives in the Trusted Computing Group (TCG). As those become ratified standards that are available in hardware, there are opportunities for synergy with this specification.
