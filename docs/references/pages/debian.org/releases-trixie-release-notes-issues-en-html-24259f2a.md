---
url: "https://www.debian.org/releases/trixie/release-notes/issues.en.html"
title: "5. Issues to be aware of for trixie — release-notes documentation"
captured: "2026-11-03T02:58:38Z"
via: "live"
sha256: "5063b8f052d179bbe621cc042f91a6f80a4cac0ffca83b262a52f2ba8345ccaf"
wayback: "https://web.archive.org/web/20261024103608/https://www.debian.org/releases/trixie/release-notes/issues.en.html"
cited_in:
  - "docs/research/hardening.md"
---

# 5. Issues to be aware of for trixie — release-notes documentation

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.debian.org/releases/trixie/release-notes/issues.en.html>
> Snapshot: <https://web.archive.org/web/20261024103608/https://www.debian.org/releases/trixie/release-notes/issues.en.html>

- https://www.debian.org/releases/trixie/release-notes/index.en.html
- 5. Issues to be aware of for trixie
-  [View page source](https://www.debian.org/releases/trixie/release-notes/_sources/issues.rst.txt)

[Previous](https://www.debian.org/releases/trixie/release-notes/upgrading.en.html) [Next](https://www.debian.org/releases/trixie/release-notes/moreinfo.en.html)

# 5. Issues to be aware of for trixie[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#issues-to-be-aware-of-for-releasename)

Sometimes, changes introduced in a new release have side-effects we cannot reasonably avoid, or they expose bugs somewhere else. This section documents issues we are aware of. Please also read the errata, the relevant packages’ documentation, bug reports, and other information mentioned in [Further reading](https://www.debian.org/releases/trixie/release-notes/moreinfo.en.html#morereading).

## 5.1. Things to be aware of while upgrading to trixie[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#things-to-be-aware-of-while-upgrading-to-releasename)

This section covers items related to the upgrade from bookworm to trixie.

### 5.1.1. Interrupted remote upgrades[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#interrupted-remote-upgrades)

An issue in OpenSSH in bookworm can lead to inaccessible remote systems if an upgrade being supervised over an SSH connection is interrupted. Users may be unable to re-connect to the remote system to resume the upgrade.

Updated packages for bookworm will resolve this issue in Debian 12.12, but this release was still in preparation at the time of releasing trixie. Instead, users planning upgrades to remote systems over an SSH connection are advised to first update OpenSSH to version 1:9.2p1-2+deb12u7 or greater through the [stable-updates](https://wiki.debian.org/StableUpdates) mechanism.

### 5.1.2. Reduced support for i386[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#reduced-support-for-i386)

From trixie, i386 is no longer supported as a regular architecture: there is no official kernel and no Debian installer for i386 systems. Fewer packages are available for i386 because many projects no longer support it. The architecture’s sole remaining purpose is to support running legacy code, for example, by way of [multiarch](https://wiki.debian.org/Multiarch/HOWTO) or a chroot on a 64-bit (amd64) system.

The i386 architecture is now only intended to be used on a 64-bit (amd64) CPU. Its instruction set requirements include SSE2 support, so it will not run successfully on most of the 32-bit CPU types that were supported by Debian 12.

Users running i386 systems should not upgrade to trixie. Instead, Debian recommends either reinstalling them as amd64, where possible, or retiring the hardware. [Cross-grading](https://wiki.debian.org/CrossGrading) without a reinstall is a technically possible, but risky, alternative.

### 5.1.3. Last release for armel[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#last-release-for-armel)

From trixie, armel is no longer supported as a regular architecture: there is no Debian installer for armel systems, and only Raspberry Pi 1, Zero, and Zero W are supported by the kernel packages.

Users running armel systems can upgrade to trixie, provided their hardware is supported by the kernel packages, or they use a third-party kernel.

trixie will be the last release for the armel architecture. Debian recommends, where possible, reinstalling armel systems as armhf or arm64, or retiring the hardware.

### 5.1.4. MIPS architectures removed[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#mips-architectures-removed)

From trixie, the architectures mipsel and mips64el are no longer supported by Debian. Users of these architectures are advised to switch to different hardware.

### 5.1.5. Ensure /boot has enough free space[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#ensure-boot-has-enough-free-space)

The Linux kernel and firmware packages have increased considerably in size in previous Debian releases and in trixie. As a result your /boot partition might be too small, causing the upgrade to fail. If your system was installed with Debian 10 (buster) or earlier, your system is very likely to be affected.

Before starting the upgrade, make sure your /boot partition is at least 768 MB in size, and has about 300 MB free. If your system does not have a separate /boot partition, there should be nothing to do.

If /boot is in LVM and too small, you can use lvextend to [increase the size of an LVM partition](https://wiki.debian.org/LVM#Increase_the_size_of_a_partition_using_LVM). if /boot is a separate partition it is likely easier to reinstall the system.

### 5.1.6. The temporary-files directory /tmp is now stored in a tmpfs[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#the-temporary-files-directory-tmp-is-now-stored-in-a-tmpfs)

From trixie, the default is for the /tmp/ directory to be stored in memory using a [tmpfs(5)](https://manpages.debian.org/trixie/tmpfs(5)) filesystem. This should make applications using temporary files faster, but if you put large files there, you may run out of memory.

For systems upgraded from bookworm, the new behavior only starts after a reboot. Files left in /tmp will be hidden after the new tmpfs is mounted which will lead to warnings in the system journal or syslog. Such files can be accessed using a bind-mount (see [mount(8)](https://manpages.debian.org/trixie/mount(8))): running mount --bind / /mnt will make the underlying directory accessible at /mnt/tmp (run umount /mnt once you have cleaned up the old files).

The default is to allocate up to 50% of memory to /tmp (this is a maximum: memory is only used when files are actually created in /tmp). You can change the size by running systemctl edit tmp.mount as root and setting, for example:

```
[Mount]
Options=mode=1777,nosuid,nodev,size=2G

```

(see [systemd.mount(5)](https://manpages.debian.org/trixie/systemd.mount(5))).

You can return to /tmp being a regular directory by running systemctl mask tmp.mount as root and rebooting.

The new filesystem defaults can also be overridden in /etc/fstab, so systems that already define a separate /tmp partition will be unaffected.

### 5.1.7. openssh-server no longer reads ~/.pam_environment[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#openssh-server-no-longer-reads-pam-environment)

The Secure Shell (SSH) daemon provided in the openssh-server package, which allows logins from remote systems, no longer reads the user’s ~/.pam_environment file by default; this feature has a [history of security problems](https://bugs.debian.org/1030119) and has been deprecated in current versions of the Pluggable Authentication Modules (PAM) library. If you used this feature, you should switch from setting variables in ~/.pam_environment to setting them in your shell initialization files (e.g. ~/.bash_profile or ~/.bashrc) or some other similar mechanism instead.

Existing SSH connections will not be affected, but new connections may behave differently after the upgrade. If you are upgrading remotely, it is normally a good idea to ensure that you have some other way to log into the system before starting the upgrade; see [Prepare for recovery](https://www.debian.org/releases/trixie/release-notes/upgrading.en.html#recovery).

### 5.1.8. OpenSSH no longer supports DSA keys[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#openssh-no-longer-supports-dsa-keys)

Digital Signature Algorithm (DSA) keys, as specified in the Secure Shell (SSH) protocol, are inherently weak: they are limited to 160-bit private keys and the SHA-1 digest. The SSH implementation provided by the openssh-client and openssh-server packages has disabled support for DSA keys by default since OpenSSH 7.0p1 in 2015, released with Debian 9 (“stretch”), although it could still be enabled using the HostKeyAlgorithms and PubkeyAcceptedAlgorithms configuration options for host and user keys respectively.

The only remaining uses of DSA at this point should be connecting to some very old devices. For all other purposes, the other key types supported by OpenSSH (RSA, ECDSA, and Ed25519) are superior.

As of OpenSSH 9.8p1 in trixie, DSA keys are no longer supported even with the above configuration options. If you have a device that you can only connect to using DSA, then you can use the ssh1 command provided by the openssh-client-ssh1 package to do so.

In the unlikely event that you are still using DSA keys to connect to a Debian server (if you are unsure, you can check by adding the -v option to the ssh command line you use to connect to that server and looking for the “Server accepts key:” line), then you must generate replacement keys before upgrading. For example, to generate a new Ed25519 key and enable logins to a server using it, run this on the client, replacing username@server with the appropriate user and host names:

```
$ ssh-keygen -t ed25519
$ ssh-copy-id username@server

```

### 5.1.9. The last, lastb and lastlog commands have been replaced[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#the-last-lastb-and-lastlog-commands-have-been-replaced)

The util-linux package no longer provides the last or lastb commands, and the login package no longer provides lastlog. These commands provided information about previous login attempts using /var/log/wtmp, /var/log/btmp, /var/run/utmp and /var/log/lastlog, but these files will not be usable after 2038 because they do not allocate enough space to store the login time (the [Year 2038 Problem](https://theyear2038problem.com/)), and the upstream developers do not want to change the file formats. Most users will not need to replace these commands with anything, but the util-linux package provides a lslogins command which can tell you when accounts were last used.

There are two direct replacements available: last can be replaced by wtmpdb from the wtmpdb package (the libpam-wtmpdb package also needs to be installed) and lastlog can be replaced by lastlog2 from the lastlog2 package (libpam-lastlog2 also needs to be installed). If you want to use these, you will need to install the new packages after the upgrade, see the [util-linux NEWS.Debian](https://salsa.debian.org/debian/util-linux/-/blob/debian/unstable/debian/NEWS) for further information. The command lslogins --failed provides similar information to lastb.

If you do not install wtmpdb then we recommend you remove old log files /var/log/wtmp*. If you do install wtmpdb it will upgrade /var/log/wtmp and you can read older wtmp files with wtmpdb import -f <dest>. There is no tool to read /var/log/lastlog* or /var/log/btmp* files: they can be deleted after the upgrade.

### 5.1.10. Encrypted filesystems need systemd-cryptsetup package[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#encrypted-filesystems-need-systemd-cryptsetup-package)

Support for automatically discovering and mounting encrypted filesystems has been moved into the new systemd-cryptsetup package. This new package is recommended by systemd so should be installed automatically on upgrades.

Please make sure the systemd-cryptsetup package is installed before rebooting, if you use encrypted filesystems.

### 5.1.11. Default encryption settings for plain-mode dm-crypt devices changed[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#default-encryption-settings-for-plain-mode-dm-crypt-devices-changed)

The default settings for dm-crypt devices created using plain-mode encryption (see [crypttab(5)](https://manpages.debian.org/trixie/crypttab(5))) have changed to improve security. This will cause problems if you did not record the settings used in /etc/crypttab. The recommended way to configure plain-mode devices is to record the options cipher, size, and hash in /etc/crypttab; otherwise cryptsetup will use default values, and the defaults for cipher and hash algorithm have changed in trixie, which will cause such devices to appear as random data until they are properly configured.

This does not apply to LUKS devices because LUKS records the settings in the device itself.

To properly configure your plain-mode devices, assuming they were created with the bookworm defaults, you should add cipher=aes-cbc-essiv:sha256,size=256,hash=ripemd160 to /etc/crypttab.

To access such devices with cryptsetup on the command line you can use --cipher aes-cbc-essiv:sha256 --key-size 256 --hash ripemd160. Debian recommends that you configure permanent devices with LUKS, or if you do use plain mode, that you explicitly record all the required encryption settings in /etc/crypttab. The new defaults are cipher=aes-xts-plain64 and hash=sha256.

### 5.1.12. RabbitMQ no longer supports HA queues[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#rabbitmq-no-longer-supports-ha-queues)

High-availability (HA) queues are no longer supported by rabbitmq-server starting in trixie. To continue with an HA setup, these queues need to be switched to “quorum queues”.

If you have an OpenStack deployment, please switch the queues to quorum before upgrading. Please also note that beginning with OpenStack’s “Caracal” release in trixie, OpenStack supports only quorum queues.

### 5.1.13. RabbitMQ cannot be directly upgraded from bookworm[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#rabbitmq-cannot-be-directly-upgraded-from-bookworm)

There is no direct, easy upgrade path for RabbitMQ from bookworm to trixie. Details about this issue can be found in [bug 1100165](https://bugs.debian.org/1100165).

The recommended upgrade path is to completely wipe the rabbitmq database and restart the service (after the trixie upgrade). This may be done by deleting /var/lib/rabbitmq/mnesia and all of its contents.

### 5.1.14. MariaDB major version upgrades only work reliably after a clean shutdown[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#mariadb-major-version-upgrades-only-work-reliably-after-a-clean-shutdown)

MariaDB does not support error recovery across major versions. For example if a MariaDB 10.11 server experienced an abrupt shutdown due to power loss or software defect, the database needs to be restarted with the same MariaDB 10.11 binaries so it can do successful error recovery and reconcile the data files and log files to roll-forward or revert transactions that got interrupted.

If you attempt to do crash recovery with MariaDB 11.8 using the data directory from a crashed MariaDB 10.11 instance, the newer MariaDB server will refuse to start.

To ensure a MariaDB Server is shut down cleanly before going into major version upgrade, stop the service with

```
# service mariadb stop

```

followed by checking server logs for Shutdown complete to confirm that flushing all data and buffers to disk completed successfully.

If it didn’t shut down cleanly, restart it to trigger crash recovery, wait, stop again and verify that second stop was clean.

For additional information about how to make backups and other relevant information for system administrators, please see [/usr/share/doc/mariadb-server/README.Debian.gz](https://sources.debian.org/src/mariadb/trixie/debian/mariadb-server.README.Debian/).

### 5.1.15. /etc/sysctl.conf is no longer honored[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#etc-sysctl-conf-is-no-longer-honored)

In Debian 13, systemd-sysctl no longer reads /etc/sysctl.conf. The package linux-sysctl-defaults ships /usr/lib/sysctl.d/50-default.conf which is intended to replace the former /etc/sysctl.conf. This package is recommended by systemd, and will thus be installed by default on systems where installation of recommended packages has not been turned off.

Check whether linux-sysctl-defaults is installed on your system and whether the contents of /usr/lib/sysctl.d/50-default.conf conform to your expectations. Consider putting local configuration into file snippets named /etc/sysctl.d/*.conf.

### 5.1.16. Ping no longer runs with elevated privileges[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#ping-no-longer-runs-with-elevated-privileges)

The default version of ping (provided by iputils-ping) is no longer installed with access to the CAP_NET_RAW linux capability, but instead uses ICMP_PROTO datagram sockets for network communication. Access to these sockets is controlled based on the user’s Unix group membership using the net.ipv4.ping_group_range sysctl. In normal installations, the linux-sysctl-defaults package will set this value to a broadly permissive value, allowing unprivileged users to use ping as expected, but some upgrade scenarios may not automatically install this package. See /usr/lib/sysctl.d/50-default.conf and [the kernel documentation](https://docs.kernel.org/networking/ip-sysctl.html#ip-sysctl) for more information on the semantics of this variable.

### 5.1.17. Network interface names may change[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#network-interface-names-may-change)

Users of systems without easy out-of-band managment are advised to proceed with caution as we’re aware of two circumstances where network interface names assigned by trixie systems may be different from bookworm. This can cause broken network connectivity when rebooting to complete the upgrade.

It is difficult to determine if a given system is affected ahead of time without a detailed technical analysis. Configurations known to be problematic are as follows:

-

Systems using the Linux i40e NIC driver, see [bug #1107187](https://bugs.debian.org/1107187).

-

Systems where firmware exposes the _SUN ACPI table object which was previously ignored by default in bookworm ([systemd.net-naming-scheme](https://manpages.debian.org/trixie/systemd/systemd.net-naming-scheme.7.en.html#HISTORY) v252), but is now used by systemd v257 in trixie. See [bug #1092176](https://bugs.debian.org/1092176).

You can use the $ udevadm test-builtin net_setup_link command to see whether the systemd change alone would yield a different name. This needs to be done just before rebooting to finish the upgrade. For example:

```
# After apt full-upgrade, but before reboot
$ udevadm test-builtin net_setup_link /sys/class/net/enp1s0 2>/dev/null
ID_NET_DRIVER=igb
ID_NET_LINK_FILE=/usr/lib/systemd/network/99-default.link
ID_NET_NAME=ens1  #< Notice the final ID_NET_NAME name is not "enp1s0"!

```

Users that need names to stay stable across the upgrade are advised to create [systemd.link](https://manpages.debian.org/trixie/udev/systemd.link.5.en.html) files to “pin” the current name before the upgrade.

### 5.1.18. Dovecot configuration changes[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#dovecot-configuration-changes)

The dovecot email server suite in trixie uses a configuration format that is incompatible with previous versions. Details about the configuration changes are available at [docs.dovecot.org](https://doc.dovecot.org/main/installation/upgrade/2.3-to-2.4.html).

In order to avoid potentially extended downtime, you are strongly encouraged to port your configuration in a staging environment before beginning the upgrade of a production mail system.

Please also note that some features were removed upstream in v2.4. In particular, the replicator is gone. If you depend on that feature, it is advisable not to upgrade to trixie until you have found an alternative.

### 5.1.19. Significant changes to libvirt packaging[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#significant-changes-to-libvirt-packaging)

The libvirt-daemon package, which provides an API and toolkit for managing virtualization platforms, has been overhauled in trixie. Each driver and storage backend now comes in a separate binary package, which enables much greater flexibility.

Care is taken during upgrades from bookworm to retain the existing set of components, but in some cases functionality might end up being temporarily lost. We recommend that you carefully review the list of installed binary packages after upgrading to ensure that all the expected ones are present; this is also a great time to consider uninstalling unwanted components.

In addition, some conffiles might end up marked as “obsolete” after the upgrade. The /usr/share/doc/libvirt-common/NEWS.Debian.gz file contains additional information on how to verify whether your system is affected by this issue and how to address it.

### 5.1.20. Samba: Active Directory Domain Controller packaging changes[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#samba-active-directory-domain-controller-packaging-changes)

The Active Directory Domain Controller (AD-DC) functionality was split out of samba. If you are using this feature, you need to install the samba-ad-dc package.

### 5.1.21. Samba: VFS modules[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#samba-vfs-modules)

The samba-vfs-modules package was reorganized. Most VFS modules are now included in the samba package. However the modules for ceph and glusterfs have been split off into samba-vfs-ceph and samba-vfs-glusterfs.

### 5.1.22. OpenLDAP TLS now provided by OpenSSL[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#openldap-tls-now-provided-by-openssl)

The TLS support in the OpenLDAP client libldap2 and server slapd is now provided by OpenSSL instead of GnuTLS. This affects the available configuration options, as well as the behavior of them.

Details about the changed options can be found in /usr/share/doc/libldap2/NEWS.Debian.gz.

If no TLS CA certificates are specified, the system default trust store will now be loaded automatically. If you do not want the default CAs to be used, you must configure the trusted CAs explicitly.

For more information about LDAP client configuration, see the [ldap.conf.5](https://manpages.debian.org/trixie/ldap.conf.5) man page. For the LDAP server (slapd), see /usr/share/doc/slapd/README.Debian.gz and the [slapd-config.5](https://manpages.debian.org/trixie/slapd-config.5) man page.

### 5.1.23. bacula-director: Database schema update needs large amounts of disk space and time[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#bacula-director-database-schema-update-needs-large-amounts-of-disk-space-and-time)

The Bacula database will undergo a substantial schema change while upgrading to trixie.

Upgrading the database can take many hours or even days, depending on the size of the database and the performance of your database server.

The upgrade temporarily needs around double the currently used disk space on the database server, plus enough space to hold a backup dump of the Bacula database in /var/cache/dbconfig-common/backups.

Running out of disk space during the upgrade might corrupt your database and will prevent your Bacula installation from functioning correctly.

### 5.1.24. dpkg: warning: unable to delete old directory: …[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#dpkg-warning-unable-to-delete-old-directory)

During the upgrade, dpkg will print warnings like the following, for various packages. This is due to the finalization of the usrmerge project, and the warnings can be safely ignored.

```
Unpacking firmware-misc-nonfree (20230625-1) over (20230515-3) ...
dpkg: warning: unable to delete old directory '/lib/firmware/wfx': Directory not empty
dpkg: warning: unable to delete old directory '/lib/firmware/ueagle-atm': Directory not empty

```

### 5.1.25. Skip-upgrades are not supported[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#skip-upgrades-are-not-supported)

As with any other Debian release, upgrades must be performed from the previous release. Also all point release updates should be installed. See [Start from “pure” Debian](https://www.debian.org/releases/trixie/release-notes/upgrading.en.html#system-status).

Skipping releases when upgrading is explicitly not supported.

For trixie, the finalization of the usrmerge project requires the upgrade to bookworm be completed before starting the trixie upgrade.

### 5.1.26. WirePlumber has a new configuration system[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#wireplumber-has-a-new-configuration-system)

WirePlumber has a new configuration system. For the default configuration you don’t have to do anything; for custom setups see /usr/share/doc/wireplumber/NEWS.Debian.gz.

### 5.1.27. strongSwan migration to a new charon daemon[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#strongswan-migration-to-a-new-charon-daemon)

The strongSwan IKE/IPsec suite is migrating from the legacy charon-daemon (using the [ipsec(8)](https://manpages.debian.org/trixie/ipsec(8)) command and configured in /etc/ipsec.conf) to charon-systemd (managed with the [swanctl(8)](https://manpages.debian.org/trixie/swanctl(8)) tools and configured in /etc/swanctl/conf.d). The trixie version of the strongswan metapackage will pull in the new dependencies, but existing installations are unaffected as long as charon-daemon is kept installed. Users are advised to migrate their installation to the new configuration following the [upstream migration page](https://wiki.strongswan.org/projects/strongswan/wiki/Fromipsecconf).

### 5.1.28. udev properties from sg3-utils missing[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#udev-properties-from-sg3-utils-missing)

Due to [bug 1109923](https://bugs.debian.org/1109923) in sg3-utils SCSI devices do not receive all properties in the “udev” database. If your installation relies on properties injected by the sg3-utils-udev package, either migrate away from them or be prepared to debug failures after rebooting into trixie.

### 5.1.29. Timezones split off into tzdata-legacy package[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#timezones-split-off-into-tzdata-legacy-package)

Timezone names not following the current tzdata naming rule of geographical region (continent or ocean) and city name were split out into the tzdata-legacy package. This includes the US/* timezones. If your installation uses such a timezone, it will be upgraded to use an equivalent timezone. However, SQL databases like PostgreSQL and other services might have copied the name into their configuration or data files. If necessary, you can install the tzdata-legacy package.

See [the tzdata-legacy file list](https://packages.debian.org/trixie/all/tzdata-legacy/filelist) for the affected timezones.

### 5.1.30. Manual steps needed for arm64 cloud VMs using UEFI boot[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#manual-steps-needed-for-arm64-cloud-vms-using-uefi-boot)

Arm64 cloud virtual machines based on images from cloud.debian.org may fail to boot in certain environments using UEFI firmware. Refer to [bug 1132330](https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=1132330) for full details.

To ensure that upgraded VMs are still bootable, you should peform the following operations to update the UEFI bootloader after full-upgrade but before rebooting:

```
$ touch /etc/grub.d/enable_cloud
$ apt install grub-cloud-arm64

```

### 5.1.31. Things to do before rebooting[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#things-to-do-before-rebooting)

When apt full-upgrade has finished, the “formal” upgrade is complete. For the upgrade to trixie, there are no special actions needed before performing a reboot.

## 5.2. Items not limited to the upgrade process[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#items-not-limited-to-the-upgrade-process)

### 5.2.1. The directories /tmp and /var/tmp are now regularly cleaned[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#the-directories-tmp-and-var-tmp-are-now-regularly-cleaned)

On new installations, systemd-tmpfiles will now regularly delete old files in /tmp and /var/tmp while the system is running. This change makes Debian consistent with other distributions. Because there is a small risk of data loss, it has been made “opt-in”: the upgrade to trixie will create a file /etc/tmpfiles.d/tmp.conf which reinstates the old behavior. This file can be deleted to adopt the new default, or edited to define custom rules. The rest of this section explains the new default and how to customize it.

The new default behavior is for files in /tmp to be automatically deleted after 10 days from the time they were last used (as well as after a reboot). Files in /var/tmp are deleted after 30 days (but not deleted after a reboot).

Before adopting the new default, you should either adapt any local programs that store data in /tmp or /var/tmp for long periods to use alternative locations, such as ~/tmp/, or tell systemd-tmpfiles to exempt the data file from deletion by creating a file local-tmp-files.conf in /etc/tmpfiles.d containing lines such as:

```
x /var/tmp/my-precious-file.pdf
x /tmp/foo

```

Please see [systemd-tmpfiles(8)](https://manpages.debian.org/trixie/systemd-tmpfiles(8)) and [tmpfiles.d(5)](https://manpages.debian.org/trixie/tmpfiles.d(5)) for more information.

### 5.2.2. systemd message: System is tainted: unmerged-bin[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#systemd-message-system-is-tainted-unmerged-bin)

systemd upstream, since version 256, considers systems having separate /usr/bin and /usr/sbin directories noteworthy. At startup systemd emits a message to record this fact: System is tainted: unmerged-bin.

It is recommended to ignore this message. Merging these directories manually is unsupported and will break future upgrades. Further details can be found in [bug #1085370](https://bugs.debian.org/1085370).

### 5.2.3. Limitations in security support[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#limitations-in-security-support)

There are some packages where Debian cannot promise to provide minimal backports for security issues. These are covered in the following subsections.

Note

The package debian-security-support helps to track the security support status of installed packages.

#### 5.2.3.1. Security status of web browsers and their rendering engines[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#security-status-of-web-browsers-and-their-rendering-engines)

Debian 13 includes several browser engines which are affected by a steady stream of security vulnerabilities. The high rate of vulnerabilities and partial lack of upstream support in the form of long term branches make it very difficult to support these browsers and engines with backported security fixes. Additionally, library interdependencies make it extremely difficult to update to newer upstream releases. Applications using the webkit2gtk source package (e.g. epiphany) are covered by security support, but applications using qtwebengine (source packages qtwebengine-opensource-src and qt6-webengine) are not.

For general web browser use we recommend Firefox or Chromium. They will be kept up-to-date by rebuilding the current ESR releases for stable. The same strategy will be applied for Thunderbird.

Once a release becomes oldstable, officially supported browsers may not continue to receive updates for the standard period of coverage. For example, Chromium will only receive 6 months of security support in oldstable rather than the typical 12 months.

#### 5.2.3.2. Go- and Rust-based packages[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#go-and-rust-based-packages)

The Debian infrastructure currently has problems with rebuilding packages of types that systematically use static linking. With the growth of the Go and Rust ecosystems it means that these packages will be covered by limited security support until the infrastructure is improved to deal with them maintainably.

In most cases if updates are warranted for Go or Rust development libraries, they will only be released via regular point releases.

### 5.2.4. Problems with VMs on 64-bit little-endian PowerPC (ppc64el)[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#problems-with-vms-on-64-bit-little-endian-powerpc-ppc64el)

Currently QEMU always tries to configure PowerPC virtual machines to support 64 kiB memory pages. This does not work for KVM-accelerated virtual machines when using the default kernel package.

-

If the guest OS can use a page size of 4 kiB, you should set the machine property cap-hpt-max-page-size=4096. For example:

```
$ kvm -machine pseries,cap-hpt-max-page-size=4096 -m 4G -hda guest.img

```

-

If the guest OS requires a page size of 64 kiB, you should install the linux-image-powerpc64le-64k package; see [64-bit little-endian PowerPC (ppc64el) page size](https://www.debian.org/releases/trixie/release-notes/upgrading.en.html#ppc64el-kernel-page-size).

## 5.3. Obsolescence and deprecation[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#obsolescence-and-deprecation)

### 5.3.1. Noteworthy obsolete packages[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#noteworthy-obsolete-packages)

The following is a list of known and noteworthy obsolete packages (see [Obsolete packages](https://www.debian.org/releases/trixie/release-notes/upgrading.en.html#obsolete) for a description).

The list of obsolete packages includes:

-

The libnss-gw-name package has been removed from trixie. The upstream developer suggests using libnss-myhostname instead.

-

The pcregrep package has been removed from trixie. It can be replaced with grep -P (--perl-regexp) or pcre2grep (from pcre2-utils).

-

The request-tracker4 package has been removed from trixie. Its replacement is request-tracker5, which includes instructions on how to migrate your data: you can keep the now obsolete request-tracker4 package from bookworm installed while migrating.

-

The git-daemon-run and git-daemon-sysvinit packages have been removed from trixie due to security reasons.

-

The nvidia-graphics-drivers-tesla-470 packages are no longer supported upstream and have been removed from trixie.

-

The deborphan package has been removed from trixie. To remove unnecessary packages, apt autoremove should be used, after apt-mark minimize-manual. debfoster can also be a useful tool.

-

The tldr package has been removed from trixie. It can be replaced with tealdeer or tldr-py packages.

-

The tpp (Text Presentation Program) package has been removed from trixie. It can be replaced with lookatme or patat packages.

### 5.3.2. Deprecated components for trixie[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#deprecated-components-for-releasename)

With the next release of Debian 14 (codenamed forky) some features will be deprecated. Users will need to migrate to other alternatives to prevent trouble when updating to Debian 14.

This includes the following features:

-

The sudo-ldap package will be removed in forky. The Debian sudo team has decided to discontinue it due to maintenance difficulties and limited use. New and existing systems should use libsss-sudo instead.

Upgrading Debian trixie to forky without completing this migration may result in the loss of intended privilege escalation.

For further details, please refer to [bug 1033728](https://bugs.debian.org/1033728) and to the NEWS file in the sudo package.

-

The sudo_logsrvd feature, used for sudo input/output logging, may be removed in Debian forky unless a maintainer steps forward. This component is of limited use within the Debian context, and maintaining it adds unnecessary complexity to the basic sudo package.

For ongoing discussions, see [bug 1101451](https://bugs.debian.org/1101451) and the NEWS file in the sudo package.

-

The libnss-docker package is no longer developed upstream and requires version 1.21 of the Docker API. That deprecated API version is still supported by Docker Engine v26 (shipped by Debian trixie) but will be removed in Docker Engine v27+ (shipped by Debian forky). Unless upstream development resumes, the package will be removed in Debian forky.

-

The openssh-client and openssh-server packages currently support [GSS-API](https://en.wikipedia.org/wiki/Generic_Security_Services_Application_Program_Interface) authentication and key exchange, which is usually used to authenticate to [Kerberos](https://en.wikipedia.org/wiki/Kerberos_(protocol)) services. This has caused some problems, especially on the server side where it adds new pre-authentication attack surface, and Debian’s main OpenSSH packages will therefore stop supporting it starting with forky.

If you are using GSS-API authentication or key exchange (look for options starting with GSSAPI in your OpenSSH configuration files) then you should install the openssh-client-gssapi (on clients) or openssh-server-gssapi (on servers) package now. On trixie, these are empty packages depending on openssh-client and openssh-server respectively; on forky, they will be built separately.

-

sbuild-debian-developer-setup has been deprecated in favor of sbuild+unshare

sbuild, the tool to build Debian packages in a minimal environment, has had a major upgrade and should work out of the box now. As a result the package sbuild-debian-developer-setup is no longer needed and has been deprecated. You can try the new version with:

```
$ sbuild --chroot-mode=unshare --dist=unstable hello

```

-

The fcitx packages have been deprecated in favor of fcitx5

The fcitx input method framework, also known as fcitx4 or fcitx 4.x, is no longer maintained upstream. As a result, all related input method packages are now deprecated. The package fcitx and packages with names beginning with fcitx- will be removed in Debian forky.

Existing fcitx users are encouraged to switch to fcitx5 following the [fcitx upstream migration guide](https://fcitx-im.org/wiki/Upgrade_from_Fcitx_4) and [Debian Wiki page](https://wiki.debian.org/I18n/Fcitx5).

-

The lxd virtual machine management package is no longer being updated and users should move to incus.

After Canonical Ltd changed the license used by LXD and introduced a new copyright assignment requirement, the Incus project was started as a community-maintained fork (see [bug 1058592](https://bugs.debian.org/1058592)). Debian recommends that you switch from LXD to Incus. The incus-extra package includes tools to migrate containers and virtual machines from LXD.

-

The isc-dhcp suite is [deprecated upstream](https://www.isc.org/blogs/isc-dhcp-eol/).

If you are using NetworkManager or systemd-networkd, you can safely remove the isc-dhcp-client package as they both ship their own implementation. If you are using the ifupdown package, dhcpcd-base provides a replacement. The ISC recommends the Kea package as a replacement for DHCP servers.

-

KDE Frameworks 5 development [has stopped](https://community.kde.org/Schedules/Frameworks#KDE_Frameworks_5.x).

The upstream KDE projects have shifted their development efforts to the Qt 6-based KDE Frameworks 6 libraries, and the Qt 5-based KDE Frameworks 5 are not being maintained anymore.

The Debian Qt / KDE team plans to remove KDE Frameworks 5 from Debian during the forky development cycle.

## 5.4. Known severe bugs[](https://www.debian.org/releases/trixie/release-notes/issues.en.html#known-severe-bugs)

Although Debian releases when it’s ready, that unfortunately doesn’t mean there are no known bugs. As part of the release process all the bugs of severity serious or higher are actively tracked by the Release Team, so an [overview of those bugs](https://bugs.debian.org/cgi-bin/pkgreport.cgi?users=release.debian.org@packages.debian.org;tag=trixie-can-defer) that were tagged to be ignored in the last part of releasing trixie can be found in the [Debian Bug Tracking System](https://bugs.debian.org/). The following bugs were affecting trixie at the time of the release and worth mentioning in this document:

|

Bug number

|

Package (source or binary)

|

Description

|

[1032240](https://bugs.debian.org/1032240)

|

akonadi-backend-mysql

|

akonadi server not robust against mysql upgrades

|

[1078608](https://bugs.debian.org/1078608)

|

apt

|

apt update silently leaves old index data

|

[1108467](https://bugs.debian.org/1108467)

|

artha

|

Segmentation fault

|

[1109499](https://bugs.debian.org/1109499)

|

bacula-director-sqlite3

|

bacula-common: preinst intentionally aborts unattended upgrade of bacula-director

|

[1108010](https://bugs.debian.org/1108010)

|

src:e2fsprogs

|

mc: error while loading shared libraries: libcom_err.so.2: cannot open shared object file

|

[1102690](https://bugs.debian.org/1102690)

|

flash-kernel

|

A higher version (…) is still installed, no reflashing required

|

[1109509](https://bugs.debian.org/1109509)

|

gcc-offload-amdgcn

|

fails to dist-upgrade from bookworm to trixie

|

[1110119](https://bugs.debian.org/1110119)

|

git-merge-changelog

|

git-merge-changelog loses or corrupts ChangeLog entries

|

[1036041](https://bugs.debian.org/1036041)

|

src:grub2

|

upgrade-reports: Dell XPS 9550 fails to boot after bullseye to bookworm upgrade - grub/bios interaction bug?

|

[1102160](https://bugs.debian.org/1102160)

|

grub-efi-amd64

|

upgrade-reports: Bookworm to Trixie [amd64][EFI] initramfs unpacking failed invalid magic at start of compressed archive

|

[913916](https://bugs.debian.org/913916)

|

grub-efi-amd64

|

UEFI boot option removed after update to grub2 2.02~beta3-5+deb9u1

|

[984760](https://bugs.debian.org/984760)

|

grub-efi-amd64

|

upgrade works, boot fails (error: symbol grub_is_lockdown not found)

|

[1099655](https://bugs.debian.org/1099655)

|

kmod

|

initramfs-tools 146 generates incorrect initramfs : does not boot, does not find root fs

|

[935182](https://bugs.debian.org/935182)

|

libreoffice-core

|

Concurrent file open on the same host results file deletion

|

[1017906](https://bugs.debian.org/1017906)

|

src:librsvg

|

Contains generated files whose source is not necessarily the same version that’s in main

|

[1109203](https://bugs.debian.org/1109203)

|

src:linux

|

linux-image-6.12.35+deb13-amd64: hangs during boot, before dmcrypt passphrase prompt

|

[1109676](https://bugs.debian.org/1109676)

|

src:linux

|

Breaks PCI (vfio) passthrough for VM guests

|

[1109512](https://bugs.debian.org/1109512)

|

liblldb-dev

|

fails to dist-upgrade from bookworm to trixie

|

[1104231](https://bugs.debian.org/1104231)

|

libmlir-17t64

|

libmlir-17t64 is couninstallable

|

[1084955](https://bugs.debian.org/1084955)

|

src:llvm-toolchain-18

|

llvm-toolchain-*: assembly code seems to depend on build cpu capabilities

|

[1104177](https://bugs.debian.org/1104177)

|

libc++-18-dev, libunwind-18-dev, libc++abi1-18, libc++abi-18-dev, libunwind-18

|

libc++-18-dev fails to coinstall

|

[1104336](https://bugs.debian.org/1104336)

|

libmlir-18

|

libmlir-18 is Multi-Arch: same but fails to coinstall

|

[1084954](https://bugs.debian.org/1084954)

|

src:llvm-toolchain-19

|

llvm-toolchain-*: assembly code seems to depend on build cpu capabilities

|

[1095866](https://bugs.debian.org/1095866)

|

llvm-19

|

llvm-toolchain-19: unsoundness/miscompilations on i386

|

[1100981](https://bugs.debian.org/1100981)

|

libmlir-19

|

libmlir-19 fails to coinstall

|

[1109519](https://bugs.debian.org/1109519)

|

mbox-importer

|

fails to dist-upgrade from bookworm to trixie (removed during dist-upgrade)

|

[1110263](https://bugs.debian.org/1110263)

|

openshot-qt

|

does not start at all – AttributeError: type object ‘GreenSocket’ has no attribute ‘sendmsg’

|

[1108039](https://bugs.debian.org/1108039)

|

python3.13

|

An object referenced only through it’s own __dict__ can get collected too early

|

[1089432](https://bugs.debian.org/1089432)

|

src:shim

|

Supporting rootless builds by default

|

[1101956](https://bugs.debian.org/1101956)

|

snapd

|

core18-based snap apps don’t work with fonts-cantarell containing variable font

|

[1101839](https://bugs.debian.org/1101839)

|

python3-tqdm

|

segmentation fault in destructor method

|

[1017891](https://bugs.debian.org/1017891)

|

src:vala

|

Ships autogenerated files that can’t be renegerated with the code in Debian main

|

[1109833](https://bugs.debian.org/1109833)

|

voctomix-gui

|

cannot import SafeConfigParser

|

[988477](https://bugs.debian.org/988477)

|

src:xen

|

xen-hypervisor-4.14-amd64: xen dmesg shows (XEN) AMD-Vi: IO_PAGE_FAULT on sata pci device
