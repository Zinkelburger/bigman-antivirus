---
url: "https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system"
title: "dm-crypt/Encrypting an entire system - ArchWiki"
captured: "2026-11-03T02:58:34Z"
via: "live"
sha256: "9762abbcc598e0e4492c4a8071110a9f5d89858fc65240fbc55aff7534eabfd7"
wayback: "https://web.archive.org/web/20261030234322/https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# dm-crypt/Encrypting an entire system - ArchWiki

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system>
> Snapshot: <https://web.archive.org/web/20261030234322/https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system>

# dm-crypt/Encrypting an entire system

- [Deutsch](https://wiki.archlinux.de/title/Systemverschl%C3%BCsselung_mit_dm-crypt)
- [Español](https://wiki.archlinux.org/title/Dm-crypt_(Espa%C3%B1ol)/Encrypting_an_entire_system_(Español))
- [Magyar](https://wiki.archlinux.org/title/Dm-crypt_(Magyar)/Encrypting_an_entire_system_(Magyar))
- [日本語](https://wiki.archlinux.jp/index.php/Dm-crypt/%E3%82%B7%E3%82%B9%E3%83%86%E3%83%A0%E5%85%A8%E4%BD%93%E3%81%AE%E6%9A%97%E5%8F%B7%E5%8C%96)
- [Polski](https://wiki.archlinux.org/title/Dm-crypt_(Polski)/Encrypting_an_entire_system_(Polski))
- [Português](https://wiki.archlinux.org/title/Dm-crypt_(Portugu%C3%AAs)/Encrypting_an_entire_system_(Português))
- [Русский](https://wiki.archlinux.org/title/Dm-crypt_(%D0%A0%D1%83%D1%81%D1%81%D0%BA%D0%B8%D0%B9)/Encrypting_an_entire_system_(Русский))
- [中文（简体）](https://wiki.archlinuxcn.org/wiki/dm-crypt/%E5%8A%A0%E5%AF%86%E6%95%B4%E4%B8%AA%E7%B3%BB%E7%BB%9F)

From ArchWiki

< [Dm-crypt](https://wiki.archlinux.org/title/Dm-crypt)

The following are examples of common scenarios of full system encryption with dm-crypt. They explain all the adaptations that need to be done to the normal [installation procedure](https://wiki.archlinux.org/title/Installation_guide). All the necessary tools are on the [installation image](https://archlinux.org/download/).

If you want to encrypt an existing unencrypted file system, see [dm-crypt/Device encryption#Encrypt an existing unencrypted file system](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#Encrypt_an_existing_unencrypted_file_system).

## Overview

Securing a root file system is where dm-crypt excels, feature and performance-wise. Unlike selectively encrypting non-root file systems, an encrypted root file system can conceal information such as which programs are installed, the usernames of all user accounts, and common data-leakage vectors such as [locate](https://wiki.archlinux.org/title/Locate) and /var/log/. Furthermore, an encrypted root file system makes tampering with the system far more difficult, as everything except the [boot loader](https://wiki.archlinux.org/title/Boot_loader) and (usually) the kernel is encrypted.

All scenarios illustrated in the following share these advantages, other pros and cons differentiating them are summarized below:

| Scenarios   | Advantages   | Disadvantages

| [#LUKS on a partition](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#LUKS_on_a_partition)

shows a basic and straightforward set-up for a fully LUKS encrypted root.

|

- Simple partitioning and setup
- On a GPT partitioned disk, [systemd can auto-mount](https://wiki.archlinux.org/title/Systemd#GPT_partition_automounting) the root partition.

|

- Inflexible; disk-space to be encrypted has to be pre-allocated

| [#LUKS on a partition with TPM2 and Secure Boot](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#LUKS_on_a_partition_with_TPM2_and_Secure_Boot)

Similar to the example above, with Secure Boot and TPM2 providing additional layers of security.

|

Same advantages as above, and

- Secure Boot allows protection against [Evil maid attacks](https://en.wikipedia.org/wiki/Evil_maid_attack)
- TPM2 prevents the system from being unlocked if Secure Boot is disabled or modified

|

- Same disadvantages as above.

| [#LVM on LUKS](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#LVM_on_LUKS)

achieves partitioning flexibility by using LVM inside a single LUKS encrypted partition.

|

- Simple partitioning with knowledge of LVM
- Only one key required to unlock all volumes (e.g. easy resume-from-disk setup)
- Volume layout not visible when locked
- Easiest method to allow [suspension to disk](https://wiki.archlinux.org/title/Dm-crypt/Swap_encryption#With_suspend-to-disk_support)

|

- LVM adds an additional mapping layer and hook
- Less useful, if a singular volume should receive a separate key
- If you have several LVM physical volumes (PVs) in a volume group that you want to use inside LUKS, then each physical volume must be encrypted separately using LUKS. In order to use them, all containers must be unlocked individually before the volume group is activated during system boot.

| [#LUKS on LVM](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#LUKS_on_LVM)

uses dm-crypt only after the LVM is setup.

|

- LVM can be used to have encrypted volumes span multiple disks
- Easy mix of un-/encrypted volume groups

|

- Complex; changing volumes requires changing encryption mappers too
- Volumes require individual keys
- LVM layout is visible when locked
- Slower boot time; each encrypted LV must be unlocked seperately

| [#LUKS on software RAID](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#LUKS_on_software_RAID)

uses dm-crypt only after RAID is setup.

|

- Analogous to LUKS on LVM

|

- Analogous to LUKS on LVM and Encrypted boot partition (GRUB)

| [#Plain dm-crypt](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#Plain_dm-crypt)

uses dm-crypt plain mode, i.e. without a LUKS header and its options for multiple keys.

This scenario also employs USB devices for /boot and key storage, which may be applied to the other scenarios.

|

- Data resilience for cases where a LUKS header may be damaged
- Allows [deniable encryption](https://en.wikipedia.org/wiki/Deniable_encryption)
- Helps addressing [problems](https://wiki.archlinux.org/title/Dm-crypt/Specialties#Discard/TRIM_support_for_solid_state_drives_(SSD)) with SSDs

|

- High care to all encryption parameters is required
- Single encryption key and no option to change it
- Very complicated setup for a regular-use system

| [#Encrypted boot partition (GRUB)](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#Encrypted_boot_partition_(GRUB))

shows how to encrypt the boot partition using the GRUB boot loader.

This scenario also employs an EFI system partition, which may be applied to the other scenarios.

|

- Same advantages as the scenario the installation is based on (LVM on LUKS for this particular example)
- Less data is left unencrypted, i.e. the boot loader and the EFI system partition, if present

|

- Same disadvantages as the scenario the installation is based on (LVM on LUKS for this particular example)
- More complicated configuration
- Not supported by other boot loaders
- [GRUB takes a long time to unlock LUKS](https://wiki.archlinux.org/title/GRUB/Tips_and_tricks#Speeding_up_LUKS_decryption_in_GRUB), thus slowing down boot

| [#Root on ZFS](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#Root_on_ZFS)   |

- In the case of a encrypted zpool all datasets are contained inside the same cryptographic environment making it easy to dual-boot and share data across installs.
- Backups can be made to a destination with an unencrypted zfs setup. Snapshots will be [encrypted natively on the destination](https://freebsdfoundation.org/our-work/journal/browser-based-edition/storage-and-filesystems/protecting-data-with-zfs-native-encryption/#:~:text=As%20you%20have%20seen%2C%20ZFS,able%20to%20mount%20the%20dataset.).

|

- [ZFS will not encrypt](https://openzfs.github.io/openzfs-docs/man/v2.2/8/zfs-load-key.8.html) metadata related to the pool structure, including dataset and snapshot names, dataset hierarchy, properties, file size, file holes, and deduplication tables (though the deduplicated data itself is encrypted).
- Pool creation requires the user to have a more in-depth knowledge of disks geometry setting even block size (ashift) for best performance.
- ZFS has some caveats with its own implementation of aes and some encryption algorithms [may not perform well](https://github.com/openzfs/zfs/issues/15276).
- Swap on a zvol or file inside a dataset is a [old and well-known issue](https://github.com/openzfs/zfs/issues/7734) with no workaround other than having your swap in another partition or lv and suspend to disk disabled (see below).

While all above scenarios provide much greater protection from outside threats than encrypted secondary file systems, they also share a common disadvantage: any user in possession of the encryption key is able to decrypt the entire drive, and therefore can access other users' data. If that is of concern, it is possible to use a combination of block device and stacked file system encryption and reap the advantages of both. See [Data-at-rest encryption](https://wiki.archlinux.org/title/Data-at-rest_encryption) to plan ahead.

See [dm-crypt/Drive preparation#Partitioning](https://wiki.archlinux.org/title/Dm-crypt/Drive_preparation#Partitioning) for a general overview of the partitioning strategies used in the scenarios.

Another area to consider is whether to set up an encrypted swap partition and what kind. See [dm-crypt/Swap encryption](https://wiki.archlinux.org/title/Dm-crypt/Swap_encryption) for alternatives.

If you anticipate to protect the system's data not only against physical theft, but also have a requirement of precautions against logical tampering, see [dm-crypt/Specialties#Securing the unencrypted boot partition](https://wiki.archlinux.org/title/Dm-crypt/Specialties#Securing_the_unencrypted_boot_partition) for further possibilities after following one of the scenarios.

For [solid state drives](https://wiki.archlinux.org/title/Solid_state_drive) you might want to consider enabling TRIM support, but be warned, there are potential security implications. See [dm-crypt/Specialties#Discard/TRIM support for solid state drives (SSD)](https://wiki.archlinux.org/title/Dm-crypt/Specialties#Discard/TRIM_support_for_solid_state_drives_(SSD)) for more information.

Warning

- In any scenario, never use file system repair software such as [fsck](https://wiki.archlinux.org/title/Fsck) directly on an encrypted volume, or it will destroy any means to recover the key used to decrypt your files. Such tools must be used on the decrypted (opened) device instead.
- The Argon2 key derivation function has a high RAM usage per design, defaulting to 1 GiB per encrypted mapper. Machines with low RAM and/or multiple LUKS2 partitions unlocked in parallel may error on boot. See the --pbkdf-memory option to control memory usage.[[1]](https://gitlab.com/cryptsetup/cryptsetup/issues/372)
- Waking-up from suspend to disk on a ZFS dataset can corrupt your pool so, be extra careful when setting up hibernation even if swap is placed outside the zvol. [Reference here](https://github.com/openzfs/zfs/issues/260#issuecomment-991912492).

## LUKS on a partition

This example covers a full system encryption with dm-crypt + LUKS in a simple partition layout:

```
+-----------------------+------------------------+-----------------------+
| Boot partition        | LUKS encrypted root    | Optional free space   |
|                       | partition              | for additional        |
|                       |                        | partitions to be set  |
| /boot                 | /                      | up later              |
|                       |                        |                       |
|                       | /dev/mapper/root       |                       |
|                       |------------------------|                       |
| /dev/sda1             | /dev/sda2              |                       |
+-----------------------+------------------------+-----------------------+

```

The first steps can be performed directly after booting the Arch Linux install image.

### Preparing the disk

Prior to creating any partitions, you should inform yourself about the importance and methods to securely erase the disk, described in [dm-crypt/Drive preparation](https://wiki.archlinux.org/title/Dm-crypt/Drive_preparation).

Then create the needed partitions, at least one for / (e.g. /dev/sda2) and /boot (/dev/sda1). See [Partitioning](https://wiki.archlinux.org/title/Partitioning).

### Preparing non-boot partitions

This and the next section replace the instructions of [Installation guide#Format the partitions](https://wiki.archlinux.org/title/Installation_guide#Format_the_partitions).

The following commands create and mount the encrypted root partition. They correspond to the procedure described in detail in [dm-crypt/Device encryption#Encrypting devices with LUKS mode](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#Encrypting_devices_with_LUKS_mode). If you want to use particular non-default encryption options (e.g. cipher, key length, sector size), see the [encryption options](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#Encryption_options_for_LUKS_mode) before executing the first command.

```
# cryptsetup -v luksFormat /dev/sda2
# cryptsetup open /dev/sda2 root

```

[Create a file system](https://wiki.archlinux.org/title/Create_a_file_system) on unlocked LUKS device. For example, to create an [Ext4](https://wiki.archlinux.org/title/Ext4) file system, run:

```
# mkfs.ext4 /dev/mapper/root

```

Mount the root volume to /mnt:

```
# mount /dev/mapper/root /mnt

```

Check the mapping works as intended:

```
# umount /mnt
# cryptsetup close root
# cryptsetup open /dev/sda2 root
# mount /dev/mapper/root /mnt

```

If you created separate partitions (e.g. /home), these steps have to be adapted and repeated for all of them, except for /boot. See [dm-crypt/Encrypting a non-root file system#Automated unlocking and mounting](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_a_non-root_file_system#Automated_unlocking_and_mounting) on how to handle additional partitions at boot.

Note that each block device requires its own passphrase. This may be inconvenient, because it results in a separate passphrase to be input during boot. An alternative is to use a keyfile stored in the root partition to unlock the separate partition via crypttab. See [dm-crypt/Device encryption#Using LUKS to format partitions with a keyfile](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#Using_LUKS_to_format_partitions_with_a_keyfile) for instructions.

### Preparing the boot partition

What you do have to setup is a non-encrypted /boot partition, which is needed for an encrypted root. For an [EFI system partition](https://wiki.archlinux.org/title/EFI_system_partition) on UEFI systems, execute the following command to format the newly created partition:

Warning Only format the EFI system partition if you created it during the partitioning step. If there already was an EFI system partition on disk beforehand, reformatting it can destroy the boot loaders of other installed operating systems.

```
# mkfs.fat -F32 /dev/sda1

```

or for an ordinary boot partition on BIOS systems:

```
# mkfs.ext4 /dev/sda1

```

Afterwards create the directory for the mountpoint and mount the partition:

```
# mount --mkdir /dev/sda1 /mnt/boot

```

### Mounting the devices

At the step [Installation guide#Mount the file systems](https://wiki.archlinux.org/title/Installation_guide#Mount_the_file_systems), you should mount the /dev/mapper/* devices (the contents of LUKS), not the actual partitions. Of course, the partition for /boot, which is not encrypted, should still be mounted directly. During installation, it should be mounted to /mnt/boot (assuming the device for the root file system is mounted to /mnt during installation).

### Configuring mkinitcpio

Before following [Installation guide#Initramfs](https://wiki.archlinux.org/title/Installation_guide#Initramfs) you must do the following to your new system:

If using the default [systemd-based initramfs](https://wiki.archlinux.org/title/Mkinitcpio), add the keyboard and sd-encrypt hooks to [mkinitcpio.conf](https://wiki.archlinux.org/title/Mkinitcpio.conf). If you use a non-US console keymap or a non-default console font, additionally add the sd-vconsole hook.

```
HOOKS=(base systemd autodetect microcode modconf kms keyboard sd-vconsole block sd-encrypt filesystems fsck)

```

If using a busybox-based initramfs, add the keyboard and encrypt hooks. If you use a non-US console keymap or a non-default console font, additionally add the keymap and consolefont hooks, respectively.

```
HOOKS=(base udev autodetect microcode modconf kms keyboard keymap consolefont block encrypt filesystems fsck)

```

[Regenerate the initramfs](https://wiki.archlinux.org/title/Regenerate_the_initramfs) after saving the changes. See [dm-crypt/System configuration#mkinitcpio](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#mkinitcpio) for details and other hooks that you may need.

### Configuring the boot loader

In order to unlock the encrypted root partition at boot, the following [kernel parameters](https://wiki.archlinux.org/title/Kernel_parameters) need to be set by the boot loader:

```
rd.luks.name=device-UUID=root root=/dev/mapper/root

```

If using the encrypt hook, the following need to be set instead:

```
cryptdevice=UUID=device-UUID:root root=/dev/mapper/root

```

The device-UUID refers to the UUID of the LUKS superblock. See [Persistent block device naming](https://wiki.archlinux.org/title/Persistent_block_device_naming) for details.

Also see [dm-crypt/System configuration#Kernel parameters](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#Kernel_parameters) for more details.

Tip

- If the root partition is on the same disk as the /boot partition and your UEFI boot loader supports [GPT partition automounting](https://wiki.archlinux.org/title/Systemd#GPT_partition_automounting), you can configure the [partition type GUID](https://en.wikipedia.org/wiki/GUID_Partition_Table#Partition_type_GUIDs) (type should be "Root partition", not "LUKS partition") and rely on [systemd-gpt-auto-generator(8)](https://man.archlinux.org/man/systemd-gpt-auto-generator.8) instead of using the kernel parameters.
- For systemd-based initramfs, instead of using rd.luks kernel parameters, encrypted devices can be specified in [crypttab](https://wiki.archlinux.org/title/Crypttab) with the x-initrd.attach option. See [dm-crypt/System configuration#Using systemd-cryptsetup-generator](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#Using_systemd-cryptsetup-generator).

## LUKS on a partition with TPM2 and Secure Boot

This example is similar to [#LUKS on a partition](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#LUKS_on_a_partition), but integrates the use of [Secure Boot](https://wiki.archlinux.org/title/Secure_Boot) and a [Trusted Platform Module](https://wiki.archlinux.org/title/Trusted_Platform_Module) (TPM), enhancing the overall security of the boot process.

In this configuration, only the [EFI system partition](https://wiki.archlinux.org/title/EFI_system_partition) remains unencrypted, housing a [unified kernel image](https://wiki.archlinux.org/title/Unified_kernel_image) and [systemd-boot](https://wiki.archlinux.org/title/Systemd-boot) (or alternatively another compatible boot loader of your choice)—both signed for use with Secure Boot. If Secure Boot is disabled or its key databases are tampered with, the TPM will not release the key to unlock the encrypted partition. This approach is akin to BitLocker on Windows or FileVault on macOS. A recovery-key will also be created to make sure the data remains accessible in case of a problem with the TPM unlocking mechanism (unsigned boot loader or kernel update, firmware update, etc.). Optionally, a TPM pin can be set to be required during boot time to prevent fully automatic unlocking.

Make sure to thoroughly read the discussion and warnings in [Trusted Platform Module#LUKS encryption](https://wiki.archlinux.org/title/Trusted_Platform_Module#LUKS_encryption).

In this example, partitions are created respecting [systemd#GPT partition automounting](https://wiki.archlinux.org/title/Systemd#GPT_partition_automounting), there is no need for an fstab or crypttab file.

```
+-----------------------+---------------------------------+
| EFI system partition  | LUKS encrypted root partition   |
|                       |                                 |
|                       |                                 |
| /boot                 | /                               |
|                       |                                 |
|                       | /dev/mapper/root                |
|                       |---------------------------------|
| /dev/sda1             | /dev/sda2                       |
+-----------------------+---------------------------------+

```

Follow the [Installation guide](https://wiki.archlinux.org/title/Installation_guide) up to step [Installation guide#Partition the disks](https://wiki.archlinux.org/title/Installation_guide#Partition_the_disks).

### Preparing the disk

Prior to creating any partitions, you should inform yourself about the importance and methods to securely erase the disk, described in [dm-crypt/Drive preparation](https://wiki.archlinux.org/title/Dm-crypt/Drive_preparation).

[Partition](https://wiki.archlinux.org/title/Partition) the drive with the [GUID Partition Table](https://wiki.archlinux.org/title/GUID_Partition_Table) (GPT).

Create an [EFI system partition](https://wiki.archlinux.org/title/EFI_system_partition#GPT_partitioned_disks) (e.g., /dev/sda1) with an appropriate size. This will be mounted at /boot.

In the remaining space, create a root partition (e.g., /dev/sda2) that will be encrypted and mounted at /. Set the partition type GUID for the root partition using type "Linux root (x86-64)" in [fdisk](https://wiki.archlinux.org/title/Fdisk) or type code 8304 in [gdisk](https://wiki.archlinux.org/title/Gdisk).

Check the output of fdisk -l to make sure partition types are properly set.

### Preparing the root partition

The following commands create and mount the encrypted root partition. They correspond to the procedure described in detail in [dm-crypt/Device encryption#Encrypting devices with LUKS mode](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#Encrypting_devices_with_LUKS_mode).

If you want to use particular non-default encryption options (e.g. cipher, key length), or if you don't want to use TPM based decryption, see the [encryption options](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#Encryption_options_for_LUKS_mode) before executing the first command.

Warning Use a sufficiently secure password. Even though the keyslot will be wiped later, SSD wear-leveling can cause it to persist after removal for an indefinite amount of time.

Create the LUKS volume and mount it:

```
# cryptsetup luksFormat /dev/sda2
# cryptsetup open /dev/sda2 root

```

[Create a file system](https://wiki.archlinux.org/title/Create_a_file_system) on unlocked LUKS device. For example, to create an [Ext4](https://wiki.archlinux.org/title/Ext4) file system, run:

```
# mkfs.ext4 /dev/mapper/root

```

Mount the root volume to /mnt:

```
# mount /dev/mapper/root /mnt

```

### Preparing the EFI system partition

Format the newly created EFI system partition as instructed in [EFI system partition#Format the partition](https://wiki.archlinux.org/title/EFI_system_partition#Format_the_partition) and mount it afterwards.

```
# mount --mkdir /dev/sda1 /mnt/boot

```

Continue the installation process until [Installation guide#Initramfs](https://wiki.archlinux.org/title/Installation_guide#Initramfs). You can skip [Installation guide#Fstab](https://wiki.archlinux.org/title/Installation_guide#Fstab).

### Configuring mkinitcpio

To build a working systemd based initramfs, modify the HOOKS= line in [mkinitcpio.conf](https://wiki.archlinux.org/title/Mkinitcpio.conf) as follows:

```
HOOKS=(base systemd autodetect microcode modconf kms keyboard sd-vconsole block sd-encrypt filesystems fsck)

```

Next, see [Unified kernel image#mkinitcpio](https://wiki.archlinux.org/title/Unified_kernel_image#mkinitcpio) to configure mkinitcpio for [Unified kernel images](https://wiki.archlinux.org/title/Unified_kernel_image).

### Installing the boot loader

You can configure your system to directly boot the UEFI image without any boot loader, see [Unified kernel image#Directly from UEFI](https://wiki.archlinux.org/title/Unified_kernel_image#Directly_from_UEFI).

If a boot loader is desired, the easiest option is to use [systemd-boot](https://wiki.archlinux.org/title/Systemd-boot) which does not require installing additional packages. Install [systemd-boot](https://wiki.archlinux.org/title/Systemd-boot) in your EFI system partition with

```
# bootctl install

```

The [Unified kernel image](https://wiki.archlinux.org/title/Unified_kernel_image) generated by mkinitcpio will be automatically recognized and does not need an entry in /boot/loader/entries/.

See [systemd-boot#Updating the UEFI boot manager](https://wiki.archlinux.org/title/Systemd-boot#Updating_the_UEFI_boot_manager) and [systemd-boot#Loader configuration](https://wiki.archlinux.org/title/Systemd-boot#Loader_configuration) for further configuration. Alternative options to [systemd-boot](https://wiki.archlinux.org/title/Systemd-boot) compatible with [Unified kernel images](https://wiki.archlinux.org/title/Unified_kernel_image) are listed in [Unified_kernel_image#Booting](https://wiki.archlinux.org/title/Unified_kernel_image#Booting).

### Finalizing the installation

First, [Regenerate the initramfs](https://wiki.archlinux.org/title/Regenerate_the_initramfs), and make sure the image generation is successful.

Make sure you did not forget to [set a root password](https://wiki.archlinux.org/title/Installation_guide#Root_password), [reboot](https://wiki.archlinux.org/title/Installation_guide#Reboot) to finish the installation.

### Secure Boot

You can now sign the boot loader executables and the EFI binary, in order to enable [Secure Boot](https://wiki.archlinux.org/title/Secure_Boot). For a quick and easy way, see [Unified Extensible Firmware Interface/Secure Boot#Assisted process with sbctl](https://wiki.archlinux.org/title/Unified_Extensible_Firmware_Interface/Secure_Boot#Assisted_process_with_sbctl).

### Enrolling the TPM

After signing the boot loader executables and enabling Secure Boot, you can now enroll the TPM in order to use it to unlock the LUKS volume. The following commands will remove the empty passphrase created during the LUKS format process, create a key bound to the TPM [PCR 7](https://wiki.archlinux.org/title/Trusted_Platform_Module#Accessing_PCR_registers) ([Secure Boot](https://wiki.archlinux.org/title/Secure_Boot) state and enrolled certificates) and create a recovery key to be used in case of any problems. The TPM will automatically release the key as long as the boot chain is not tampered with. See [systemd-cryptenroll#Trusted Platform Module](https://wiki.archlinux.org/title/Systemd-cryptenroll#Trusted_Platform_Module) and [systemd-cryptenroll(1)](https://man.archlinux.org/man/systemd-cryptenroll.1).

```
# systemd-cryptenroll /dev/sda2 --recovery-key
# systemd-cryptenroll /dev/sda2 --wipe-slot=empty --tpm2-device=auto --tpm2-pcrs=7+15:sha256=0000000000000000000000000000000000000000000000000000000000000000

```

Note

- If a passphrase was set during the LUKS format process, the corresponding keyslot should be wiped (e.g. --wipe-slot=0).
- You can list keyslots using systemd-cryptenroll /dev/sda2. See [systemd-cryptenroll#List keyslots](https://wiki.archlinux.org/title/Systemd-cryptenroll#List_keyslots).

Tip Add --tpm2-with-pin=yes to require an additional PIN to unlock at boot time.

Warning

- Make sure [Secure Boot](https://wiki.archlinux.org/title/Secure_Boot) is active and in user mode when binding to PCR 7, otherwise, unauthorized boot devices could unlock the encrypted volume.
- The state of PCR 7 can change if firmware certificates change, which can risk locking the user out. This can be implicitly done by [fwupd](https://wiki.archlinux.org/title/Fwupd)[[2]](https://raw.githubusercontent.com/systemd/systemd/ed272a9ff59a26beedaab508dd3c9d631de67165/TODO) or explicitly by rotating Secure Boot keys.

## LVM on LUKS

The straightforward method is to set up [LVM](https://wiki.archlinux.org/title/LVM) on top of the encrypted partition instead of the other way round. Technically the LVM is setup inside one big encrypted block device. Hence, the LVM is not visible until the block device is unlocked and the underlying volume structure is scanned and mounted during boot.

The disk layout in this example is:

```
+-----------------------------------------------------------------------+ +----------------+
| Logical volume 1      | Logical volume 2      | Logical volume 3      | | Boot partition |
|                       |                       |                       | |                |
| [SWAP]                | /                     | /home                 | | /boot          |
|                       |                       |                       | |                |
| /dev/MyVolGroup/swap  | /dev/MyVolGroup/root  | /dev/MyVolGroup/home  | |                |
|_ _ _ _ _ _ _ _ _ _ _ _|_ _ _ _ _ _ _ _ _ _ _ _|_ _ _ _ _ _ _ _ _ _ _ _| | (may be on     |
|                                                                       | | other device)  |
|                         LUKS encrypted partition                      | |                |
|                           /dev/sda1                                   | | /dev/sdb1      |
+-----------------------------------------------------------------------+ +----------------+

```

Note While using the encrypt hook this method does not allow you to span the logical volumes over multiple disks; either use the [sd-encrypt](https://wiki.archlinux.org/title/Sd-encrypt) or see [dm-crypt/Specialties#Modifying the encrypt hook for multiple partitions](https://wiki.archlinux.org/title/Dm-crypt/Specialties#Modifying_the_encrypt_hook_for_multiple_partitions).

Tip Two variants of this setup:

- Instructions at [dm-crypt/Specialties#Encrypted system using a detached LUKS header](https://wiki.archlinux.org/title/Dm-crypt/Specialties#Encrypted_system_using_a_detached_LUKS_header) use this setup with a detached LUKS header on a USB device to achieve a two factor authentication with it.
- Instructions at [dm-crypt/Specialties#Encrypted /boot and a detached LUKS header on USB](https://wiki.archlinux.org/title/Dm-crypt/Specialties#Encrypted_/boot_and_a_detached_LUKS_header_on_USB) use this setup with a detached LUKS header, encrypted /boot partition, and encrypted keyfile all on a USB device.

### Preparing the disk

Prior to creating any partitions, you should inform yourself about the importance and methods to securely erase the disk, described in [dm-crypt/Drive preparation](https://wiki.archlinux.org/title/Dm-crypt/Drive_preparation).

Tip When using the [GRUB](https://wiki.archlinux.org/title/GRUB) boot loader for BIOS booting from a [GPT](https://wiki.archlinux.org/title/GPT) disk, create a [BIOS boot partition](https://wiki.archlinux.org/title/BIOS_boot_partition).

[Create a partition](https://wiki.archlinux.org/title/Installation_guide#Partition_the_disks) to be mounted at /boot with a size of 1 GiB or more.

Tip UEFI systems can use the [EFI system partition](https://wiki.archlinux.org/title/EFI_system_partition) for /boot.

Create a partition which will later contain the encrypted container.

Create the LUKS encrypted container at the designated partition. Enter the chosen password twice.

```
# cryptsetup luksFormat /dev/sda1

```

For more information about the available cryptsetup options see the [LUKS encryption options](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#Encryption_options_for_LUKS_mode) prior to above command.

Open the container:

```
# cryptsetup open /dev/sda1 cryptlvm

```

The decrypted container is now available at /dev/mapper/cryptlvm.

### Preparing the logical volumes

Create a physical volume on top of the opened LUKS container:

```
# pvcreate /dev/mapper/cryptlvm

```

Create a volume group (in this example named MyVolGroup, but it can be whatever you want) and add the previously created physical volume to it:

```
# vgcreate MyVolGroup /dev/mapper/cryptlvm

```

Create all your logical volumes on the volume group:

Tip If a logical volume will be formatted with [ext4](https://wiki.archlinux.org/title/Ext4), leave at least 256 MiB free space in the volume group to allow using [e2scrub(8)](https://man.archlinux.org/man/e2scrub.8). After creating the last volume with -l 100%FREE, this can be accomplished by reducing its size with lvreduce -L -256M MyVolGroup/home.

```
# lvcreate -L 4G -n swap MyVolGroup
# lvcreate -L 32G -n root MyVolGroup
# lvcreate -l 100%FREE -n home MyVolGroup

```

[Format](https://wiki.archlinux.org/title/Format) your file systems on each logical volume. For example, using [Ext4](https://wiki.archlinux.org/title/Ext4) for the root and home volumes:

```
# mkfs.ext4 /dev/MyVolGroup/root
# mkfs.ext4 /dev/MyVolGroup/home
# mkswap /dev/MyVolGroup/swap

```

Mount your file systems:

```
# mount /dev/MyVolGroup/root /mnt
# mount --mkdir /dev/MyVolGroup/home /mnt/home
# swapon /dev/MyVolGroup/swap

```

### Preparing the boot partition

The boot loader loads the kernel, [initramfs](https://wiki.archlinux.org/title/Initramfs), and its own configuration files from the /boot directory. Any file system on a disk that can be read by the boot loader is eligible.

Create a [file system](https://wiki.archlinux.org/title/File_system) on the partition intended for /boot. For an [EFI system partition](https://wiki.archlinux.org/title/EFI_system_partition) on UEFI systems, execute the following command to format the newly created partition:

Warning Only format the EFI system partition if you created it during the partitioning step. If there already was an EFI system partition on disk beforehand, reformatting it can destroy the boot loaders of other installed operating systems.

```
# mkfs.fat -F32 /dev/sdb1

```

or for an ordinary boot partition on BIOS systems:

```
# mkfs.ext4 /dev/sdb1

```

Mount the partition to /mnt/boot:

```
# mount --mkdir /dev/sdb1 /mnt/boot

```

At this point resume the common [Installation guide#Installation](https://wiki.archlinux.org/title/Installation_guide#Installation) steps. Return to this page to customize the [Initramfs](https://wiki.archlinux.org/title/Installation_guide#Initramfs) and [Boot loader](https://wiki.archlinux.org/title/Installation_guide#Boot_loader) steps.

### Configuring mkinitcpio

Make sure the [lvm2](https://archlinux.org/packages/?name=lvm2) package is [installed](https://wiki.archlinux.org/title/Install).

If using the default systemd-based initramfs, add the keyboard, sd-encrypt and lvm2 hooks to [mkinitcpio.conf](https://wiki.archlinux.org/title/Mkinitcpio.conf). If you use a non-US console keymap or a non-default console font, additionally add the sd-vconsole hook.

```
HOOKS=(base systemd autodetect microcode modconf kms keyboard sd-vconsole block sd-encrypt lvm2 filesystems fsck)

```

If using a busybox-based initramfs, instead add the keyboard, encrypt and lvm2 hooks. If you use a non-US console keymap or a non-default console font, additionally add the keymap and consolefont hooks, respectively.

```
HOOKS=(base udev autodetect microcode modconf kms keyboard keymap consolefont block encrypt lvm2 filesystems fsck)

```

[Regenerate the initramfs](https://wiki.archlinux.org/title/Regenerate_the_initramfs) after saving the changes. See [dm-crypt/System configuration#mkinitcpio](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#mkinitcpio) for details and other hooks that you may need.

### Configuring the boot loader

In order to unlock the encrypted root partition at boot, the following [kernel parameters](https://wiki.archlinux.org/title/Kernel_parameters) need to be set by the boot loader:

```
rd.luks.name=device-UUID=cryptlvm root=/dev/MyVolGroup/root

```

If using the encrypt hook, the following needs to be set instead:

```
cryptdevice=UUID=device-UUID:cryptlvm root=/dev/MyVolGroup/root

```

The device-UUID refers to the UUID of the LUKS superblock, in this example it is the UUID of /dev/sda1 e.g. a144e931-7580-40bf-ae8c-6beff4c1ac45. See [Persistent block device naming](https://wiki.archlinux.org/title/Persistent_block_device_naming) for details.

If using [dracut](https://wiki.archlinux.org/title/Dracut), these parameters are known to work:

```
rd.luks.uuid=device-UUID root=/dev/MyVolGroup/root

```

you may need a more extensive list of parameters, try:

```
rd.luks.uuid=luks-deviceUUID rd.lvm.lv=MyVolGroup/root  rd.lvm.lv=MyVolGroup/swap  root=/dev/mapper/MyVolGroup-root rootfstype=ext4 rootflags=rw,relatime

```

See [dm-crypt/System configuration#Kernel parameters](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#Kernel_parameters) for details.

Tip For systemd-based initramfs, instead of using rd.luks kernel parameters, encrypted devices can be specified in [crypttab](https://wiki.archlinux.org/title/Crypttab) with the x-initrd.attach option. See [dm-crypt/System configuration#Using systemd-cryptsetup-generator](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#Using_systemd-cryptsetup-generator).

## LUKS on LVM

To use encryption on top of [LVM](https://wiki.archlinux.org/title/LVM), the LVM volumes are set up first and then used as the base for the encrypted partitions. This way, a mixture of encrypted and non-encrypted volumes/partitions is possible as well.

Tip Unlike [#LVM on LUKS](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#LVM_on_LUKS), this method allows normally spanning the logical volumes over multiple disks.

The following short example creates a LUKS on LVM setup and mixes in the use of a key-file for the /home partition and a temporary encrypted volume for swap. This is considered desirable from a security perspective, because no potentially sensitive temporary data survives the reboot, when the encryption is re-initialised. If you are experienced with LVM, you will be able to ignore/replace LVM and other specifics according to your plan.

If you want to span a logical volume over multiple disks that have already been set up, or expand the logical volume for /home (or any other volume), a procedure to do so is described in [dm-crypt/Specialties#Expanding LVM on multiple disks](https://wiki.archlinux.org/title/Dm-crypt/Specialties#Expanding_LVM_on_multiple_disks). It is important to note that the LUKS encrypted container has to be resized as well.

This article or section needs expansion.

Reason: The intro of this scenario needs some adjustment now that a comparison has been added to [#Overview](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#Overview). A suggested structure is to make it similar to the [#LUKS on a partition](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#LUKS_on_a_partition) intro. (Discuss in [Talk:Dm-crypt/Encrypting an entire system](https://wiki.archlinux.org/title/Talk:Dm-crypt/Encrypting_an_entire_system))

### Preparing the disk

Partitioning scheme:

```
+----------------+-------------------------------------------------------------------------------------------------+
| Boot partition | dm-crypt plain encrypted volume | LUKS encrypted volume         | LUKS encrypted volume         |
|                |                                 |                               |                               |
| /boot          | [SWAP]                          | /                             | /home                         |
|                |                                 |                               |                               |
|                | /dev/mapper/swap                | /dev/mapper/root              | /dev/mapper/home              |
|                |_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _|_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _|_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _|
|                | Logical volume 1                | Logical volume 2              | Logical volume 3              |
|                | /dev/MyVolGroup/cryptswap       | /dev/MyVolGroup/cryptroot     | /dev/MyVolGroup/crypthome     |
|                |_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _|_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _|_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _|
|                |                                                                                                 |
|   /dev/sda1    |                                   /dev/sda2                                                     |
+----------------+-------------------------------------------------------------------------------------------------+

```

Randomise /dev/sda2 according to [dm-crypt/Drive preparation#dm-crypt wipe on an empty device or partition](https://wiki.archlinux.org/title/Dm-crypt/Drive_preparation#dm-crypt_wipe_on_an_empty_device_or_partition).

### Preparing the logical volumes

```
# pvcreate /dev/sda2
# vgcreate MyVolGroup /dev/sda2
# lvcreate -L 4G -n cryptswap MyVolGroup
# lvcreate -L 32G -n cryptroot MyVolGroup
# lvcreate -l 100%FREE -n crypthome MyVolGroup

```

```
# cryptsetup luksFormat /dev/MyVolGroup/cryptroot
# cryptsetup open /dev/MyVolGroup/cryptroot root

```

[Create a file system](https://wiki.archlinux.org/title/Create_a_file_system) on unlocked LUKS device and mount it. For example, to create an [Ext4](https://wiki.archlinux.org/title/Ext4) file system, run:

```
# mkfs.ext4 /dev/mapper/root
# mount /dev/mapper/root /mnt

```

More information about the encryption options can be found in [dm-crypt/Device encryption#Encryption options for LUKS mode](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#Encryption_options_for_LUKS_mode). Note that /home will be encrypted in [#Encrypting logical volume /home](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#Encrypting_logical_volume_/home).

Tip If you ever have to access the encrypted root from the Arch-ISO, the above open action will allow you to after the [LVM shows up](https://wiki.archlinux.org/title/LVM#Logical_volumes_do_not_show_up).

### Preparing the boot partition

Create a [file system](https://wiki.archlinux.org/title/File_system) on the partition intended for /boot. For an [EFI system partition](https://wiki.archlinux.org/title/EFI_system_partition) on UEFI systems, execute the following command to format the newly created partition:

Warning Only format the EFI system partition if you created it during the partitioning step. If there already was an EFI system partition on disk beforehand, reformatting it can destroy the boot loaders of other installed operating systems.

```
# mkfs.fat -F32 /dev/sda1

```

or for an ordinary boot partition on BIOS systems:

```
# mkfs.ext4 /dev/sda1

```

Afterwards create the directory for the mountpoint and mount the partition:

```
# mount --mkdir /dev/sda1 /mnt/boot

```

### Configuring mkinitcpio

Make sure the [lvm2](https://archlinux.org/packages/?name=lvm2) package is [installed](https://wiki.archlinux.org/title/Install).

If using the default systemd-based initramfs, add the keyboard, sd-encrypt and lvm2 hooks to [mkinitcpio.conf](https://wiki.archlinux.org/title/Mkinitcpio.conf). If you use a non-US console keymap or a non-default console font, additionally add the sd-vconsole hook.

```
HOOKS=(base systemd autodetect microcode modconf kms keyboard sd-vconsole block sd-encrypt lvm2 filesystems fsck)

```

If using a busybox-based initramfs, instead add the keyboard, encrypt and lvm2 hooks. If you use a non-US console keymap or a non-default console font, additionally add the keymap and consolefont hooks, respectively.

```
HOOKS=(base udev autodetect microcode modconf kms keyboard keymap consolefont block encrypt lvm2 filesystems fsck)

```

[Regenerate the initramfs](https://wiki.archlinux.org/title/Regenerate_the_initramfs) after saving the changes. See [dm-crypt/System configuration#mkinitcpio](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#mkinitcpio) for details and other hooks that you may need.

### Configuring the boot loader

In order to unlock the encrypted root partition at boot, the following [kernel parameters](https://wiki.archlinux.org/title/Kernel_parameters) need to be set by the boot loader:

```
rd.luks.name=device-UUID=root root=/dev/mapper/root

```

If using the encrypt hook, the following need to be set instead:

```
cryptdevice=UUID=device-UUID:root root=/dev/mapper/root

```

The device-UUID refers to the UUID of the LUKS superblock, in this example it is the UUID of /dev/MyVolGroup/cryptroot e.g. a144e931-7580-40bf-ae8c-6beff4c1ac45. See [Persistent block device naming](https://wiki.archlinux.org/title/Persistent_block_device_naming) for details.

See [dm-crypt/System configuration#Kernel parameters](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#Kernel_parameters) for details.

Tip For systemd-based initramfs, instead of using rd.luks kernel parameters, encrypted devices can be specified in [crypttab](https://wiki.archlinux.org/title/Crypttab) with the x-initrd.attach option. See [dm-crypt/System configuration#Using systemd-cryptsetup-generator](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#Using_systemd-cryptsetup-generator).

### Configuring fstab and crypttab

Both [crypttab](https://wiki.archlinux.org/title/Crypttab) and [fstab](https://wiki.archlinux.org/title/Fstab) entries are required to both unlock the device and mount the file systems, respectively. The following lines will re-encrypt the swap volume on each reboot:

```
/etc/crypttab
```

```
swap	/dev/MyVolGroup/cryptswap	/dev/urandom	swap,cipher=aes-xts-plain64,size=256,sector-size=4096
```

```
/etc/fstab
```

```
/dev/mapper/root                          /     ext4   defaults  0 1
UUID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx /boot ext4   defaults  0 2
/dev/mapper/swap                          none  swap   defaults  0 0
```

### Encrypting logical volume /home

Since this scenario uses LVM as the primary and dm-crypt as secondary mapper, each encrypted logical volume requires its own encryption. Yet, unlike the temporary file systems configured with volatile encryption above, the logical volume for /home should of course be persistent. The following assumes you have rebooted into the installed system, otherwise you have to adjust paths. To save on entering a second passphrase at boot, a [keyfile](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#Keyfiles) is created:

```
# dd bs=512 count=4 if=/dev/random iflag=fullblock | install -m 0600 /dev/stdin /etc/cryptsetup-keys.d/home.key

```

The logical volume is encrypted with it:

```
# cryptsetup luksFormat -v /dev/MyVolGroup/crypthome /etc/cryptsetup-keys.d/home.key
# cryptsetup -d /etc/cryptsetup-keys.d/home.key open /dev/MyVolGroup/crypthome home

```

[Create a file system](https://wiki.archlinux.org/title/Create_a_file_system) on unlocked LUKS device and mount it. For example, to create an [Ext4](https://wiki.archlinux.org/title/Ext4) file system, run:

```
# mkfs.ext4 /dev/mapper/home
# mount /dev/mapper/home /home

```

The encrypted mount is configured in both [crypttab](https://wiki.archlinux.org/title/Crypttab) and [fstab](https://wiki.archlinux.org/title/Fstab):

```
/etc/crypttab
```

```
home	/dev/MyVolGroup/crypthome   none

```

```
/etc/fstab
```

```
/dev/mapper/home        /home   ext4        defaults        0       2

```

## LUKS on software RAID

This example is based on a real-world setup for a workstation class laptop equipped with two SSDs of equal size, and an additional HDD for bulk storage. The end result is LUKS based full disk encryption (including /boot) for all drives, with the SSDs in a [RAID0](https://wiki.archlinux.org/title/RAID) array, and keyfiles used to unlock all encryption after [GRUB](https://wiki.archlinux.org/title/GRUB) is given a correct passphrase at boot.

This setup utilizes a very simplistic partitioning scheme, with all the available RAID storage being mounted at / (no separate /boot partition), and the decrypted HDD being mounted at /data.

Please note that regular [backups](https://wiki.archlinux.org/title/System_backup) are very important in this setup. If either of the SSDs fail, the data contained in the RAID array will be practically impossible to recover. You may wish to select a different [RAID level](https://wiki.archlinux.org/title/RAID#Standard_RAID_levels) if fault tolerance is important to you.

The encryption is not deniable in this setup.

For the sake of the instructions below, the following block devices are used:

```
/dev/sda = first SSD
/dev/sdb = second SSD
/dev/sdc = HDD

```

```
+---------------------+---------------------------+---------------------------+ +---------------------+---------------------------+---------------------------+ +---------------------------+
| BIOS boot partition | EFI system partition      | LUKS encrypted volume     | | BIOS boot partition | EFI system partition      | LUKS encrypted volume     | | LUKS encrypted volume     |
|                     |                           |                           | |                     |                           |                           | |                           |
|                     | /efi                      | /                         | |                     | /efi                      | /                         | | /data                     |
|                     |                           |                           | |                     |                           |                           | |                           |
|                     |                           | /dev/mapper/root          | |                     |                           | /dev/mapper/root          | |                           |
|                     +---------------------------+---------------------------+ |                     +---------------------------+---------------------------+ |                           |
|                     | RAID1 array (part 1 of 2) | RAID0 array (part 1 of 2) | |                     | RAID1 array (part 2 of 2) | RAID0 array (part 2 of 2) | |                           |
|                     |                           |                           | |                     |                           |                           | |                           |
|                     | /dev/md/ESP               | /dev/md/root              | |                     | /dev/md/ESP               | /dev/md/root              | | /dev/mapper/data          |
|                     +---------------------------+---------------------------+ |                     +---------------------------+---------------------------+ +---------------------------+
| /dev/sda1           | /dev/sda2                 | /dev/sda3                 | | /dev/sdb1           | /dev/sdb2                 | /dev/sdb3                 | | /dev/sdc1                 |
+---------------------+---------------------------+---------------------------+ +---------------------+---------------------------+---------------------------+ +---------------------------+

```

Be sure to substitute them with the appropriate device designations for your setup, as they may be different.

### Preparing the disks

Prior to creating any partitions, you should inform yourself about the importance and methods to securely erase the disk, described in [dm-crypt/Drive preparation](https://wiki.archlinux.org/title/Dm-crypt/Drive_preparation).

For [BIOS systems](https://wiki.archlinux.org/title/GRUB#BIOS_systems) with GPT, create a [BIOS boot partition](https://wiki.archlinux.org/title/BIOS_boot_partition) with size of 1 MiB for GRUB to store the second stage of BIOS boot loader. Do not mount the partition.

For [UEFI systems](https://wiki.archlinux.org/title/GRUB#UEFI_systems) create an [EFI system partition](https://wiki.archlinux.org/title/EFI_system_partition) with an appropriate size, it will later be mounted at /efi.

In the remaining space on the drive create a partition (/dev/sda3 in this example) for "Linux RAID". Choose partition type ID fd for MBR or partition type GUID A19D880F-05FC-4D3B-A006-743F0F84911E for GPT.

Once partitions have been created on /dev/sda, the following commands can be used to clone them to /dev/sdb.

```
# sfdisk -d /dev/sda > sda.dump
# sfdisk /dev/sdb < sda.dump

```

The HDD is prepared with a single Linux partition covering the whole drive at /dev/sdc1.

### Building the RAID array

Create the RAID array for the SSDs.

Note

- All parts of an EFI system partition RAID array must be individually usable, that means that the ESP can only be placed in a RAID1 array.
- The RAID superblock must be placed at the end of the EFI system partition using --metadata=1.0, otherwise the firmware will not be able to access the partition.
- Be aware the cryptsetup project warns about resizing RAID devices post-setup.[[3]](https://gitlab.com/cryptsetup/cryptsetup/-/commit/c5e500ea0ffebb69a98a01d556246260214623ab)

```
# mdadm --create --verbose --level=1 --metadata=1.0 --raid-devices=2 /dev/md/ESP /dev/sda2 /dev/sdb2

```

This example utilizes RAID0 for root, you may wish to substitute a different level based on your preferences or requirements.

```
# mdadm --create --verbose --level=0 --metadata=1.2 --raid-devices=2 /dev/md/root /dev/sda3 /dev/sdb3

```

### Preparing the block devices

As explained in [dm-crypt/Drive preparation](https://wiki.archlinux.org/title/Dm-crypt/Drive_preparation), the devices are wiped with random data utilizing /dev/zero and a crypt device with a random key. Alternatively, you could use dd with /dev/random or /dev/urandom, though it will be much slower.

```
# cryptsetup open --type plain --sector-size 4096 --key-file /dev/urandom /dev/md/root to_be_wiped
# dd if=/dev/zero of=/dev/mapper/to_be_wiped bs=1M status=progress
# cryptsetup close to_be_wiped

```

And repeat above for the HDD (/dev/sdc1 in this example).

Set up encryption for /dev/md/root:

```
# cryptsetup -v luksFormat /dev/md/root
# cryptsetup open /dev/md/root root

```

[Create a file system](https://wiki.archlinux.org/title/Create_a_file_system) on unlocked LUKS device. For example, to create an [Ext4](https://wiki.archlinux.org/title/Ext4) file system, run:

```
# mkfs.ext4 /dev/mapper/root

```

Mount the root volume to /mnt:

```
# mount /dev/mapper/root /mnt

```

And repeat for the HDD:

```
# cryptsetup -v luksFormat /dev/sdc1
# cryptsetup open /dev/sdc1 data
# mkfs.ext4 /dev/mapper/data
# mount --mkdir /dev/mapper/data /mnt/data

```

For UEFI systems, format the newly created EFI system partition and mount it:

Warning Only format the EFI system partition if you created it during the partitioning step. If there already was an EFI system partition on disk beforehand, reformatting it can destroy the boot loaders of other installed operating systems.

```
# mkfs.fat -F32 /dev/md/ESP
# mount --mkdir /dev/md/ESP /mnt/efi

```

### Configuring GRUB

Configure [GRUB](https://wiki.archlinux.org/title/GRUB) for the LUKS encrypted system by editing /etc/default/grub with the following:

```
GRUB_CMDLINE_LINUX="cryptdevice=/dev/md/root:root"
GRUB_ENABLE_CRYPTODISK=y

```

If you have a USB keyboard on a newer system either enable legacy USB support in firmware or add the following to /etc/default/grub:

```
GRUB_TERMINAL_INPUT="usb_keyboard"
GRUB_PRELOAD_MODULES="usb usb_keyboard ohci uhci ehci"

```

Otherwise you may not be able to use your keyboard at the LUKS prompt.

See [dm-crypt/System configuration#Kernel parameters](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#Kernel_parameters) and [GRUB#Encrypted /boot](https://wiki.archlinux.org/title/GRUB#Encrypted_/boot) for details.

Complete the GRUB install to both SSDs (in reality, installing only to /dev/sda will work).

```
# grub-install --target=i386-pc /dev/sda
# grub-install --target=i386-pc /dev/sdb
# grub-install --target=x86_64-efi --efi-directory=/efi --bootloader-id=GRUB
# grub-mkconfig -o /boot/grub/grub.cfg

```

### Creating the keyfiles

The next steps save you from entering your passphrase twice when you boot the system (once so GRUB can unlock the LUKS device, and second time once the initramfs assumes control of the system). This is done by creating a [keyfile](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#Keyfiles) for the encryption and adding it to the initramfs image to allow the encrypt hook to unlock the root device. See [dm-crypt/Device encryption#With a keyfile embedded in the initramfs](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#With_a_keyfile_embedded_in_the_initramfs) for details.

- Create the [keyfile](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#Keyfiles) and add the key to /dev/md/root.
- Create another keyfile for the HDD (/dev/sdc1) so it can also be unlocked at boot. For convenience, leave the passphrase created above in place as this can make recovery easier if you ever need it. Edit /etc/crypttab to decrypt the HDD at boot. See [dm-crypt/System configuration#Unlocking with a keyfile](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#Unlocking_with_a_keyfile).

### Configuring the system

Edit [fstab](https://wiki.archlinux.org/title/Fstab) to mount the root and data block devices and the ESP:

```
/dev/mapper/root  /	  ext4	rw,noatime 	   0 1
/dev/mapper/data  /data   ext4	defaults           0 2
/dev/md/ESP       /efi     vfat	rw,relatime,codepage=437,iocharset=iso8859-1,shortname=mixed,utf8,tz=UTC,errors=remount-ro  	0 2

```

Save the RAID configuration:

```
# mdadm --detail --scan >> /etc/mdadm.conf

```

Edit [mkinitcpio.conf](https://wiki.archlinux.org/title/Mkinitcpio.conf) to include your keyfile and add the proper hooks:

```
FILES=(/crypto_keyfile.bin)
HOOKS=(base udev autodetect microcode modconf kms keyboard keymap consolefont block mdadm_udev encrypt filesystems fsck)

```

See [dm-crypt/System configuration#mkinitcpio](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#mkinitcpio) for details.

## Plain dm-crypt

Contrary to LUKS, dm-crypt plain mode does not require a header on the encrypted device: this scenario exploits this feature to set up a system on an unpartitioned, encrypted disk that will be indistinguishable from a disk filled with random data, which could allow [deniable encryption](https://en.wikipedia.org/wiki/Deniable_encryption). See also [wikipedia:Disk encryption#Full disk encryption](https://en.wikipedia.org/wiki/Disk_encryption#Full_disk_encryption).

Note that if full disk encryption is not required, the methods using LUKS described in the sections above are better options for both system encryption and encrypted partitions. [LUKS features](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#Cryptsetup_actions_specific_for_LUKS) like key management with multiple passphrases/key-files, master key backups or re-encrypting a device in-place are unavailable with plain mode.

Plain dm-crypt encryption can be more resilient to damage than LUKS, because it does not rely on an encryption master-key which can be a single-point of failure if damaged or forcefully destroyed. However, using plain mode also requires more manual configuration of encryption options to achieve the same cryptographic strength. See also [Data-at-rest encryption#Cryptographic metadata](https://wiki.archlinux.org/title/Data-at-rest_encryption#Cryptographic_metadata). Using plain mode could also be considered if concerned with the problems explained in [dm-crypt/Specialties#Discard/TRIM support for solid state drives (SSD)](https://wiki.archlinux.org/title/Dm-crypt/Specialties#Discard/TRIM_support_for_solid_state_drives_(SSD)).

Tip If headerless encryption is your goal but you are unsure about the lack of key-derivation with plain mode, then two alternatives are:

- [dm-crypt LUKS mode with a detached header](https://wiki.archlinux.org/title/Dm-crypt/Specialties#Encrypted_system_using_a_detached_LUKS_header) by using the cryptsetup --header option. It cannot be used with the standard encrypt hook, but the hook may be modified.
- [tcplay](https://wiki.archlinux.org/title/Tcplay) which offers headerless encryption but with the PBKDF2 function.

The scenario uses two USB sticks:

- one for the boot device, which also allows storing the options required to open/unlock the plain encrypted device in the boot loader configuration, since typing them on each boot would be error prone;
- another for the encryption key file, assuming it stored as raw bits so that to the eyes of an unaware attacker who might get the usbkey the encryption key will appear as random data instead of being visible as a normal file. See also [Wikipedia:Security through obscurity](https://en.wikipedia.org/wiki/Security_through_obscurity), follow [dm-crypt/Device encryption#Keyfiles](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#Keyfiles) to prepare the keyfile.

The disk layout is:

```
+----------------------+----------------------+----------------------+ +----------------+ +----------------+
| Logical volume 1     | Logical volume 2     | Logical volume 3     | | Boot device    | | Encryption key |
|                      |                      |                      | |                | | file storage   |
| /                    | [SWAP]               | /home                | | /boot          | | (unpartitioned |
|                      |                      |                      | |                | | in example)    |
| /dev/MyVolGroup/root | /dev/MyVolGroup/swap | /dev/MyVolGroup/home | | /dev/sdb1      | | /dev/sdc       |
|----------------------+----------------------+----------------------| |----------------| |----------------|
| disk drive /dev/sda encrypted using plain mode and LVM             | | USB stick 1    | | USB stick 2    |
+--------------------------------------------------------------------+ +----------------+ +----------------+

```

Tip

- It is also possible to use a single USB key physical device:

- By putting the key on another partition (/dev/sdb2) of the USB storage device (/dev/sdb).
- By copying the keyfile to the initramfs directly. An example keyfile /etc/cryptsetup-keys.d/root.key gets copied to the initramfs image by setting FILES=(/etc/cryptsetup-keys.d/root.key) in /etc/mkinitcpio.conf. The way to instruct the encrypt hook to read the keyfile in the initramfs image is using rootfs: prefix before the filename, e.g. cryptkey=rootfs:/etc/cryptsetup-keys.d/root.key.

- Another option is using a passphrase with good [entropy](https://wiki.archlinux.org/title/Security#Choosing_secure_passwords).

### Preparing the disk

It is vital that the mapped device is filled with random data. In particular this applies to the scenario use case we apply here.

See [dm-crypt/Drive preparation](https://wiki.archlinux.org/title/Dm-crypt/Drive_preparation) and [dm-crypt/Drive preparation#dm-crypt specific methods](https://wiki.archlinux.org/title/Dm-crypt/Drive_preparation#dm-crypt_specific_methods)

### Preparing the non-boot partitions

See [dm-crypt/Device encryption#Encryption options for plain mode](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#Encryption_options_for_plain_mode) for details.

Using the device /dev/sda, with the aes-xts cipher with a 512 bit key size and using a keyfile we have the following options for this scenario:

```
# cryptsetup open --type plain --cipher=aes-xts-plain64 --offset=0 --key-file=/dev/sdc --key-size=512 --sector-size 4096 /dev/sda cryptlvm

```

Unlike encrypting with LUKS, the above command must be executed in full whenever the mapping needs to be re-established, so it is important to remember the cipher, and key file details.

We can now check a mapping entry has been made for /dev/mapper/cryptlvm:

```
# fdisk -l

```

Tip

- A simpler alternative to using LVM, advocated in the cryptsetup FAQ for cases where LVM is not necessary, is to just create a file system on the entirety of the mapped dm-crypt device.
- If a logical volume will be formatted with [ext4](https://wiki.archlinux.org/title/Ext4), leave at least 256 MiB free space in the volume group to allow using [e2scrub(8)](https://man.archlinux.org/man/e2scrub.8). After creating the last volume with -l 100%FREE, this can be accomplished by reducing its size with lvreduce -L -256M MyVolGroup/home.

Next, we setup [LVM](https://wiki.archlinux.org/title/LVM) logical volumes on the mapped device. See [Install Arch Linux on LVM](https://wiki.archlinux.org/title/Install_Arch_Linux_on_LVM) for further details:

```
# pvcreate /dev/mapper/cryptlvm
# vgcreate MyVolGroup /dev/mapper/cryptlvm
# lvcreate -L 32G MyVolGroup -n root
# lvcreate -L 4G MyVolGroup -n swap
# lvcreate -l 100%FREE MyVolGroup -n home

```

We format and mount them and activate swap. See [File systems#Create a file system](https://wiki.archlinux.org/title/File_systems#Create_a_file_system) for further details:

```
# mkfs.ext4 /dev/MyVolGroup/root
# mkfs.ext4 /dev/MyVolGroup/home
# mount /dev/MyVolGroup/root /mnt
# mount --mkdir /dev/MyVolGroup/home /mnt/home
# mkswap /dev/MyVolGroup/swap
# swapon /dev/MyVolGroup/swap

```

### Preparing the boot partition

The /boot partition can be a typical FAT32 formatted partition on a USB stick, if required. But if manual partitioning is needed, then a small 1 GiB partition is all that is required. Create the partition using a [partitioning tool](https://wiki.archlinux.org/title/Partitioning#Partitioning_tools) of your choice.

Create a [file system](https://wiki.archlinux.org/title/File_system) on the newly created partition intended for /boot:

Warning Only format the EFI system partition if you created it during the partitioning step. If there already was an EFI system partition on disk beforehand, reformatting it can destroy the boot loaders of other installed operating systems.

```
# mkfs.fat -F32 /dev/sdb1
# mount --mkdir /dev/sdb1 /mnt/boot

```

### Configuring mkinitcpio

Make sure the [lvm2](https://archlinux.org/packages/?name=lvm2) package is [installed](https://wiki.archlinux.org/title/Install).

If using the default systemd-based initramfs, first configure the encrypted volume in [crypttab](https://wiki.archlinux.org/title/Crypttab) making sure it has x-initrd.attach in the fourth (options) field:

```
/etc/crypttab
```

```
cryptlvm /dev/disk/by-id/disk-ID-of-sda /dev/disk/by-id/disk-ID-of-sdc x-initrd.attach,plain,cipher=aes-xts-plain64,sector-size=4096,offset=0
```

Add the keyboard, sd-encrypt and lvm2 hooks to [mkinitcpio.conf](https://wiki.archlinux.org/title/Mkinitcpio.conf). If you use a non-US console keymap or a non-default console font, additionally add the sd-vconsole hook.

```
HOOKS=(base systemd autodetect microcode modconf kms keyboard sd-vconsole block sd-encrypt lvm2 filesystems fsck)

```

If using a busybox-based initramfs, add the keyboard, encrypt and lvm2 hooks. If you use a non-US console keymap or a non-default console font, additionally add the keymap and consolefont hooks, respectively.

```
HOOKS=(base udev autodetect microcode modconf kms keyboard keymap consolefont block encrypt lvm2 filesystems fsck)

```

[Regenerate the initramfs](https://wiki.archlinux.org/title/Regenerate_the_initramfs) after saving the changes. See [dm-crypt/System configuration#mkinitcpio](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#mkinitcpio) for details and other hooks that you may need.

### Configuring the boot loader

If using a busybox-based initramfs (instead of the default systemd-based initramfs) the following [kernel parameters](https://wiki.archlinux.org/title/Kernel_parameters) need to be set by the boot loader in order to boot the encrypted root partition:

Note 64 is the number of bytes in 512 bits.

```
cryptdevice=/dev/disk/by-id/disk-ID-of-sda:cryptlvm:sector-size=4096 cryptkey=/dev/disk/by-id/disk-ID-of-sdc:0:64 crypto=:aes-xts-plain64:512:0:

```

The disk-ID-of-disk refers to the id of the referenced disk. See [Persistent block device naming](https://wiki.archlinux.org/title/Persistent_block_device_naming) for details.

See [dm-crypt/System configuration#Kernel parameters](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#Kernel_parameters) for details and other parameters that you may need.

Tip If using [GRUB](https://wiki.archlinux.org/title/GRUB), you can install it on the same USB as the /boot partition.

For BIOS:

```
# grub-install --target=i386-pc --recheck /dev/sdb
```

For UEFI:

```
# grub-install --target=x86_64-efi --efi-directory=/boot --removable
```

### Post-installation

You may wish to remove the USB sticks after booting. Since the /boot partition is not usually needed, the noauto option can be added to the relevant line in /etc/fstab:

```
/etc/fstab
```

```
# /dev/sdb1
UUID=XXXX-XXXX /boot vfat noauto,rw,noatime 0 2
```

However, when an update to anything used in the initramfs, or a kernel, or the boot loader is required; the /boot partition must be present and mounted. As the entry in fstab already exists, it can be mounted simply with:

```
# mount /boot

```

## Encrypted boot partition (GRUB)

This setup utilizes the same partition layout and configuration as the previous [#LVM on LUKS](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#LVM_on_LUKS) section, with the difference that the [GRUB](https://wiki.archlinux.org/title/GRUB) boot loader is used since it is capable of booting from an LVM logical volume and a LUKS-encrypted /boot. See also [GRUB#Encrypted /boot](https://wiki.archlinux.org/title/GRUB#Encrypted_/boot).

The disk layout in this example is:

```
+---------------------+----------------------+----------------------+----------------------+----------------------+
| BIOS boot partition | EFI system partition | Logical volume 1     | Logical volume 2     | Logical volume 3     |
|                     |                      |                      |                      |                      |
|                     | /efi                 | /                    | [SWAP]               | /home                |
|                     |                      |                      |                      |                      |
|                     |                      | /dev/MyVolGroup/root | /dev/MyVolGroup/swap | /dev/MyVolGroup/home |
| /dev/sda1           | /dev/sda2            |----------------------+----------------------+----------------------+
| unencrypted         | unencrypted          | /dev/sda3 encrypted using LVM on LUKS                              |
+---------------------+----------------------+--------------------------------------------------------------------+

```

Tip

- All scenarios are intended as examples. It is, of course, possible to apply both of the two above distinct installation steps with the other scenarios as well. See also the variants linked in [#LVM on LUKS](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#LVM_on_LUKS).
- You can use cryptboot script from [cryptboot](https://aur.archlinux.org/packages/cryptboot/)AUR package for simplified encrypted boot management (mounting, unmounting, upgrading packages) and as a defense against [Evil Maid](https://www.schneier.com/blog/archives/2009/10/evil_maid_attac.html) attacks with [UEFI Secure Boot](https://wiki.archlinux.org/title/Secure_Boot#Using_your_own_keys). For more information and limitations see [cryptboot project](https://github.com/kmille/cryptboot) page.

### Preparing the disk

Prior to creating any partitions, you should inform yourself about the importance and methods to securely erase the disk, described in [dm-crypt/Drive preparation](https://wiki.archlinux.org/title/Dm-crypt/Drive_preparation).

For [UEFI systems](https://wiki.archlinux.org/title/GRUB#UEFI_systems) create an [EFI system partition](https://wiki.archlinux.org/title/EFI_system_partition) with an appropriate size, it will later be mounted at /efi.

For [BIOS/GPT setups](https://wiki.archlinux.org/title/GRUB#GUID_Partition_Table_(GPT)_specific_instructions) create a [BIOS boot partition](https://wiki.archlinux.org/title/BIOS_boot_partition) with size of 1 MiB for GRUB to store the second stage of BIOS boot loader. Do not mount the partition. For BIOS/MBR setups this is not necessary.

Create a partition of type 8309, which will later contain the encrypted container for the LVM.

Create the LUKS encrypted container:

```
# cryptsetup luksFormat /dev/sda3

```

For more information about the available cryptsetup options see the [LUKS encryption options](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#Encryption_options_for_LUKS_mode) prior to above command.

Your partition layout should look similar to this:

```
# gdisk -l /dev/sda
```

```
...
Number  Start (sector)    End (sector)  Size       Code  Name
   1            2048            4095   1024.0 KiB  EF02  BIOS boot partition
   2            4096         2101247   1024.0 MiB  EF00  EFI system partition
   3         2101248        69210111   32.0 GiB    8309  Linux LUKS

```

Open the container:

```
# cryptsetup open /dev/sda3 cryptlvm

```

The decrypted container is now available at /dev/mapper/cryptlvm.

### Preparing the logical volumes

The LVM logical volumes of this example follow the exact layout as the [#LVM on LUKS](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#LVM_on_LUKS) scenario. Therefore, please follow [#Preparing the logical volumes](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#Preparing_the_logical_volumes) above and adjust as required.

For UEFI systems, create a mountpoint for the [EFI system partition](https://wiki.archlinux.org/title/EFI_system_partition) at /efi for compatibility with grub-install and mount it:

```
# mount --mkdir /dev/sda2 /mnt/efi

```

At this point, you should have the following partitions and logical volumes inside of /mnt:

```
$ lsblk
```

```
NAME                  MAJ:MIN RM   SIZE RO TYPE  MOUNTPOINT
sda                   8:0      0   200G  0 disk
├─sda1                8:1      0     1M  0 part
├─sda2                8:2      0   550M  0 part  /mnt/efi
└─sda3                8:3      0   100G  0 part
  └─cryptlvm          254:0    0   100G  0 crypt
    ├─MyVolGroup-swap 254:1    0     4G  0 lvm   [SWAP]
    ├─MyVolGroup-root 254:2    0    32G  0 lvm   /mnt
    └─MyVolGroup-home 254:3    0    60G  0 lvm   /mnt/home

```

Now at this point resume the common [Installation guide#Installation](https://wiki.archlinux.org/title/Installation_guide#Installation) steps. Return to this page to customize the [Initramfs](https://wiki.archlinux.org/title/Installation_guide#Initramfs) and [Boot loader](https://wiki.archlinux.org/title/Installation_guide#Boot_loader) steps.

### Configuring mkinitcpio

Make sure the [lvm2](https://archlinux.org/packages/?name=lvm2) package is [installed](https://wiki.archlinux.org/title/Install).

If using the default systemd-based initramfs, add the keyboard, sd-encrypt and lvm2 hooks to [mkinitcpio.conf](https://wiki.archlinux.org/title/Mkinitcpio.conf). If you use a non-US console keymap or a non-default console font, additionally add the sd-vconsole hook.

```
HOOKS=(base systemd autodetect microcode modconf kms keyboard sd-vconsole block sd-encrypt lvm2 filesystems fsck)

```

If using a busybox-based initramfs, instead add the keyboard, encrypt and lvm2 hooks. If you use a non-US console keymap or a non-default console font, additionally add the keymap and consolefont hooks, respectively.

```
HOOKS=(base udev autodetect microcode modconf kms keyboard keymap consolefont block encrypt lvm2 filesystems fsck)

```

[Regenerate the initramfs](https://wiki.archlinux.org/title/Regenerate_the_initramfs) after saving the changes. See [dm-crypt/System configuration#mkinitcpio](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#mkinitcpio) for details and other hooks that you may need.

### Configuring GRUB

Configure GRUB to allow booting from /boot on a LUKS encrypted partition:

```
/etc/default/grub
```

```
GRUB_ENABLE_CRYPTODISK=y
```

Set the kernel parameters, so that the initramfs can unlock the encrypted root partition. Using the sd-encrypt hook:

```
/etc/default/grub
```

```
GRUB_CMDLINE_LINUX="... rd.luks.name=device-UUID=cryptlvm ..."
```

If using the encrypt hook, the following need to be set instead:

```
/etc/default/grub
```

```
GRUB_CMDLINE_LINUX="... cryptdevice=UUID=device-UUID:cryptlvm ..."
```

See [dm-crypt/System configuration#Kernel parameters](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#Kernel_parameters) and [GRUB#Encrypted /boot](https://wiki.archlinux.org/title/GRUB#Encrypted_/boot) for details. The device-UUID refers to the UUID of the LUKS superblock, in this example it is the UUID of /dev/sda3 (the partition which holds the lvm containing the root file system) e.g. a144e931-7580-40bf-ae8c-6beff4c1ac45. See [Persistent block device naming](https://wiki.archlinux.org/title/Persistent_block_device_naming) for details.

Tip For systemd-based initramfs, instead of using rd.luks kernel parameters, encrypted devices can be specified in [crypttab](https://wiki.archlinux.org/title/Crypttab) with the x-initrd.attach option. See [dm-crypt/System configuration#Using systemd-cryptsetup-generator](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#Using_systemd-cryptsetup-generator).

[install GRUB](https://wiki.archlinux.org/title/GRUB#Installation_2) to the mounted ESP for UEFI booting:

```
# grub-install --target=x86_64-efi --efi-directory=/efi --bootloader-id=GRUB --recheck

```

[install GRUB](https://wiki.archlinux.org/title/GRUB#Installation) to the disk for BIOS booting:

```
# grub-install --target=i386-pc --recheck /dev/sda

```

Generate GRUB's [configuration](https://wiki.archlinux.org/title/GRUB#Generate_the_main_configuration_file) file:

```
# grub-mkconfig -o /boot/grub/grub.cfg

```

If all commands finished without errors, GRUB should prompt for the passphrase to unlock the /dev/sda3 partition after the next reboot.

### Avoiding having to enter the passphrase twice

This article or section is a candidate for merging with [Dm-crypt/Device encryption#With a keyfile embedded in the initramfs](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#With_a_keyfile_embedded_in_the_initramfs).

Notes: Too much duplicated content, too much detail here for this overview page. (Discuss in [Talk:Dm-crypt/Encrypting an entire system#Security Issue with Grub Keyfile](https://wiki.archlinux.org/title/Talk:Dm-crypt/Encrypting_an_entire_system#Security_Issue_with_Grub_Keyfile))

While GRUB asks for a passphrase to unlock the LUKS encrypted partition after above instructions, the partition unlock is not passed on to the initramfs. Hence, you have to enter the passphrase twice at boot: once for GRUB and once for the initramfs.

This section deals with extra configuration to let the system boot by only entering the passphrase once, in GRUB. This is accomplished by [with a keyfile embedded in the initramfs](https://wiki.archlinux.org/title/Dm-crypt/Device_encryption#With_a_keyfile_embedded_in_the_initramfs).

First create a keyfile and add it as LUKS key:

```
# dd bs=512 count=4 if=/dev/random iflag=fullblock | install -m 0600 /dev/stdin /etc/cryptsetup-keys.d/cryptlvm.key
# cryptsetup -v luksAddKey /dev/sda3 /etc/cryptsetup-keys.d/cryptlvm.key

```

Add the keyfile to the initramfs image:

```
/etc/mkinitcpio.conf
```

```
FILES=(/etc/cryptsetup-keys.d/cryptlvm.key)
```

[Regenerate the initramfs](https://wiki.archlinux.org/title/Regenerate_the_initramfs).

When using the default [sd-encrypt](https://wiki.archlinux.org/title/Sd-encrypt) hook, /etc/cryptsetup-keys.d/name.key will be used by default, so no additional kernel parameters need to be set.

When using the encrypt hook, set the following kernel parameters to unlock the LUKS partition with the keyfile:

```
GRUB_CMDLINE_LINUX="... cryptkey=rootfs:/etc/cryptsetup-keys.d/cryptlvm.key"

```

If for some reason the keyfile fails to unlock the boot partition, systemd will fallback to ask for a passphrase to unlock and, in case that is correct, continue booting.

Tip If you want to encrypt the /boot partition to protect against offline tampering threats, the [mkinitcpio-chkcryptoboot](https://wiki.archlinux.org/title/Dm-crypt/Specialties#mkinitcpio-chkcryptoboot) hook has been contributed to help.

### Using a USB drive to unlock /boot

To avoid having to memorise a complicated password, or using a simple one which may be guessed, a keyfile stored on an external USB drive can be used to unlock the LUKS volume. For this to be secure, this USB drive must be stored securely away from the computer when not in use.

First, generate a keyfile in the same way as in [#Avoiding having to enter the passphrase twice](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system#Avoiding_having_to_enter_the_passphrase_twice). Do not use the same keyfile as if the USB drive is lost or compromised you will need to replace the keyfile embedded in initramfs.

Copy this keyfile to your USB drive and create a new GRUB configuration file:

```
/boot/grub/grub-pre.cfg
```

```
set crypto_uuid=UUID-of-the-luks-volume
set key_disk=UUID-of-the-volume-with-the-key
cryptomount -u $crypto_uuid -k ($key_disk)/the-location-of-the-key-on-your-usb
set root=UUID-of-the-unlocked-volume-as-in-grub.cfg
set prefix=($root)/boot/grub
insmod normal
normal
```

Create a GRUB image and install it (not all of these modules will be needed depending on your file system):

```
# grub-mkimage -p /boot/grub -O x86_64-efi -c /boot/grub/grub-pre.cfg -o /tmp/grubx64.efi  part_gpt  part_msdos cryptodisk  luks  gcry_rijndael gcry_sha512 lvm ext2 ntfs fat exfat
# install -v /tmp/grubx64.efi /efi/EFI/GRUB/grubx64.efi

```

## Root on ZFS

This article or section is being considered for removal.

Reason: There is nothing inherently different in the encryption setup between ZFS on LUKS or plain dm-crypt compared to any other file system on LUKS or plain dm-crypt. ZFS native encryption is out of scope of this article. (Discuss in [Talk:Dm-crypt/Encrypting an entire system](https://wiki.archlinux.org/title/Talk:Dm-crypt/Encrypting_an_entire_system))

To use dm-crypt with [ZFS](https://wiki.archlinux.org/title/ZFS), see [ZFS#Encryption in ZFS using dm-crypt](https://wiki.archlinux.org/title/ZFS#Encryption_in_ZFS_using_dm-crypt).

Additionally, ZFS features [native encryption](https://wiki.archlinux.org/title/ZFS#Native_encryption), which may also be utilized to encrypt the system root, excluding the boot loader and file system metadata. See [Install Arch Linux on ZFS](https://wiki.archlinux.org/title/Install_Arch_Linux_on_ZFS).

Retrieved from "https://wiki.archlinux.org/index.php?title=Dm-crypt/Encrypting_an_entire_system&oldid=886861"

[Categories](https://wiki.archlinux.org/title/Special:Categories):

- [Data-at-rest encryption](https://wiki.archlinux.org/title/Category:Data-at-rest_encryption)
- [Installation process](https://wiki.archlinux.org/title/Category:Installation_process)

Hidden categories:

- [Pages or sections flagged with Template:Expansion](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Expansion)
- [Pages or sections flagged with Template:Merge](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Merge)
- [Sections flagged with Template:Remove](https://wiki.archlinux.org/title/Category:Sections_flagged_with_Template:Remove)
