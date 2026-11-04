---
url: "https://github.com/trimstray/linux-hardening-checklist"
title: "GitHub - trimstray/linux-hardening-checklist: Simple checklist to help you deploying the most important areas of the GNU/Linux production systems - work in progress."
site: "GitHub"
captured: "2026-11-02T19:08:39Z"
via: "live"
sha256: "db66912ce6c191b0a80fe3c3e21869f00d95b39ca017090e598ed7d3feb42a52"
wayback: "https://web.archive.org/web/20260221010416/https://github.com/trimstray/linux-hardening-checklist"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# GitHub - trimstray/linux-hardening-checklist: Simple checklist to help you deploying the most important areas of the GNU/Linux production systems - work in progress.

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/trimstray/linux-hardening-checklist>
> Snapshot: <https://web.archive.org/web/20260221010416/https://github.com/trimstray/linux-hardening-checklist>

https://github.com/trimstray/linux-hardening-checklist

https://github.com/trimstray/linux-hardening-checklist/pulls https://github.com/trimstray/linux-hardening-checklist/blob/master/LICENSE.md

# Table of Contents

https://github.com/trimstray/linux-hardening-checklist#table-of-contents

- [Introduction](https://github.com/trimstray/linux-hardening-checklist#introduction)

- [Status](https://github.com/trimstray/linux-hardening-checklist#status)
- [Todo](https://github.com/trimstray/linux-hardening-checklist#todo)
- [Prologue](https://github.com/trimstray/linux-hardening-checklist#prologue)
- [Levels of priority](https://github.com/trimstray/linux-hardening-checklist#levels-of-priority)
- [OpenSCAP](https://github.com/trimstray/linux-hardening-checklist#openscap)

- [Partitioning](https://github.com/trimstray/linux-hardening-checklist#partitioning)

- [Separate partitions](https://github.com/trimstray/linux-hardening-checklist#separate-partitions)
- [Restrict mount options](https://github.com/trimstray/linux-hardening-checklist#restrict-mount-options)
- [Polyinstantiated directories](https://github.com/trimstray/linux-hardening-checklist#polyinstantiated-directories)
- [Shared memory](https://github.com/trimstray/linux-hardening-checklist#shared-memory)
- [Encrypt partitions](https://github.com/trimstray/linux-hardening-checklist#encrypt-partitions)
- [Summary checklist](https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist)

- [Physical Access](https://github.com/trimstray/linux-hardening-checklist#physical-access)

- [Password for Single User Mode](https://github.com/trimstray/linux-hardening-checklist#password-for-single-user-mode)
- [Summary checklist](https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist-1)

- [Bootloader](https://github.com/trimstray/linux-hardening-checklist#bootloader)

- [Protect bootloader config files](https://github.com/trimstray/linux-hardening-checklist#protect-bootloader-config-files)
- [Summary checklist](https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist-2)

- [Linux Kernel](https://github.com/trimstray/linux-hardening-checklist#linux-kernel)

- [Kernel logs](https://github.com/trimstray/linux-hardening-checklist#kernel-logs)
- [Kernel pointers](https://github.com/trimstray/linux-hardening-checklist#kernel-pointers)
- [ExecShield](https://github.com/trimstray/linux-hardening-checklist#execshield)
- [Memory protection](https://github.com/trimstray/linux-hardening-checklist#memory-protection)
- [Summary checklist](https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist-3)

- [Logging](https://github.com/trimstray/linux-hardening-checklist#logging)

- [Syslog](https://github.com/trimstray/linux-hardening-checklist#syslog)

- [Users and Groups](https://github.com/trimstray/linux-hardening-checklist#users-and-groups)

- [Passwords](https://github.com/trimstray/linux-hardening-checklist#passwords)
- [Logon Access](https://github.com/trimstray/linux-hardening-checklist#logon-access)
- [Summary checklist](https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist-4)

- [Filesystem](https://github.com/trimstray/linux-hardening-checklist#filesystem)

- [Hardlinks & Symlinks](https://github.com/trimstray/linux-hardening-checklist#hardlinks--symlinks)
- [Dynamic Mounting and Unmounting](https://github.com/trimstray/linux-hardening-checklist#dynamic-mounting-and-unmounting)
- [Summary checklist](https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist-5)

- [Permissions](https://github.com/trimstray/linux-hardening-checklist#permissions)
- [SELinux & Auditd](https://github.com/trimstray/linux-hardening-checklist#selinux--auditd)

- [SELinux Enforcing](https://github.com/trimstray/linux-hardening-checklist#selinux-enforcing)
- [Summary checklist](https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist-6)

- [System Updates](https://github.com/trimstray/linux-hardening-checklist#system-updates)
- [Network](https://github.com/trimstray/linux-hardening-checklist#network)

- [TCP/SYN](https://github.com/trimstray/linux-hardening-checklist#tcp-syn)
- [Routing](https://github.com/trimstray/linux-hardening-checklist#routing)
- [ICMP Protocol](https://github.com/trimstray/linux-hardening-checklist#icmp-protocol)
- [Broadcast](https://github.com/trimstray/linux-hardening-checklist#broadcast)
- [Summary checklist](https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist-7)

- [Services](https://github.com/trimstray/linux-hardening-checklist#services)
- [Tools](https://github.com/trimstray/linux-hardening-checklist#tools)

# Introduction

https://github.com/trimstray/linux-hardening-checklist#introduction

In computing, hardening is usually the process of securing a system by reducing its surface of vulnerability, which is larger when a system performs more functions; in principle a single-function system is more secure than a multipurpose one. The main goal of systems hardening is to reduce security risk by eliminating potential attack vectors and condensing the system’s attack surface.

This list contains the most important hardening rules for GNU/Linux systems.

## Status

https://github.com/trimstray/linux-hardening-checklist#status

Still work in progress... 👷

I also created another repository (in a more detailed way): [the-practical-linux-hardening-guide](https://github.com/trimstray/the-practical-linux-hardening-guide).

## Todo

https://github.com/trimstray/linux-hardening-checklist#todo

-  Add rationale (e.g. url's, external resources)
-  Review levels of priority

## Prologue

https://github.com/trimstray/linux-hardening-checklist#prologue

I'm not advocating throwing your existing hardening and deployment best practices out the door but I recommend is to always turn a feature from this checklist on in pre-production environments instead of jumping directly into production.

## Levels of priority

https://github.com/trimstray/linux-hardening-checklist#levels-of-priority

All items in this checklist contains three levels of priority:

- https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png means that the item has a low priority.
- https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png means that the item has a medium priority. You shouldn't avoid tackling that item.
- https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/high.png means that the item has a high priority. You can't avoid following that rule and implement the corrections recommended.

## OpenSCAP

https://github.com/trimstray/linux-hardening-checklist#openscap

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/openscap_logo.png

SCAP (Security Content Automation Protocol) provides a mechanism to check configurations, vulnerability management and evaluate policy compliance for a variety of systems. One of the most popular implementations of SCAP is OpenSCAP and it is very helpful for vulnerability assessment and also as hardening helper.

Some of the external audit tools use this standard. For example Nessus has functionality for authenticated SCAP scans.

I tried to make this list compatible with OpenSCAP standard and rules. However, there may be differences.

# Partitioning

https://github.com/trimstray/linux-hardening-checklist#partitioning

## Separate partitions

https://github.com/trimstray/linux-hardening-checklist#separate-partitions

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Ensure /boot located on separate partition.

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Ensure /home located on separate partition.

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Ensure /usr located on separate partition.

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Ensure /var located on separate partition.

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/high.png Ensure /var/log and /var/log/audit located on separate partitions.

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/high.png Ensure /tmp and /var/tmp located on separate partitions.

## Restrict mount options

https://github.com/trimstray/linux-hardening-checklist#restrict-mount-options

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Restrict /usr partition mount options.

Example:

```
UUID=<...>  /usr  ext4  defaults,nodev,ro  0 2
```

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Restrict /var partition mount options.

Example:

```
UUID=<...>  /var  ext4  defaults,nosuid  0 2
```

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Restrict /var/log and /var/log/audit partitions mount options.

Example:

```
UUID=<...>  /var/log        ext4  defaults,nosuid,noexec,nodev  0 2
UUID=<...>  /var/log/audit  ext4  defaults,nosuid,noexec,nodev  0 2
```

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Restrict /proc partition mount options.

Example:

```
proc  /proc  proc  defaults,hidepid=2  0 0
```

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Restrict /boot partition mount options.

Example:

```
LABEL=/boot  /boot  ext2  defaults,nodev,nosuid,noexec,ro  1 2
```

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Restrict /home partition mount options.

Example:

```
UUID=<...>  /home  ext4  defaults,nodev,nosuid  0 2
```

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Restrict /var and /var/tmp partitions mount options.

Example:

```
mv /var/tmp /var/tmp.old
ln -s /tmp /var/tmp
cp -prf /var/tmp.old/* /tmp && rm -fr /var/tmp.old

UUID=<...>  /tmp  ext4  defaults,nodev,nosuid,noexec  0 2
```

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Restrict /dev/shm partition mount options.

Example:

```
tmpfs  /dev/shm  tmpfs  rw,nodev,nosuid,noexec,size=1024M,mode=1777 0 0
```

## Polyinstantiated directories

https://github.com/trimstray/linux-hardening-checklist#polyinstantiated-directories

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Setting up polyinstantiated /var and /var/tmp directories.

Example:

```
# Create new directories:
mkdir --mode 000 /tmp-inst
mkdir --mode 000 /var/tmp/tmp-inst

# Edit /etc/security/namespace.conf:
/tmp      /tmp-inst/          level  root,adm
/var/tmp  /var/tmp/tmp-inst/  level  root,adm

# Set correct SELinux context:
setsebool polyinstantiation_enabled=1
chcon --reference=/tmp /tmp-inst
chcon --reference=/var/tmp/ /var/tmp/tmp-inst
```

## Shared memory

https://github.com/trimstray/linux-hardening-checklist#shared-memory

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Set group for /dev/shm.

Example:

```
tmpfs  /dev/shm  tmpfs  rw,nodev,nosuid,noexec,size=1024M,mode=1770,uid=root,gid=shm 0 0
```

## Encrypt partitions

https://github.com/trimstray/linux-hardening-checklist#encrypt-partitions

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Encrypt swap partition.

Example:

```
# Edit /etc/crypttab:
sdb1_crypt /dev/sdb1 /dev/urandom cipher=aes-xts-plain64,size=256,swap,discard

# Edit /etc/fstab:
/dev/mapper/sdb1_crypt none swap sw 0 0
```

## ☑️ Summary checklist

https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist

| Rule  | Priority  | Checkbox

| Separate /boot  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

| Separate /home  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

| Separate /usr  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

| Separate /var  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

| Separate /var/log and /var/log/audit  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/high.png  | 🔲

| Separate /tmp and /var/tmp  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/high.png  | 🔲

|   |   |

| Restrict /usr mount options  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

| Restrict /var mount options  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

| Restrict /var/log and /var/log/audit mount options  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

| Restrict /proc mount options  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

| Restrict /boot mount options  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

| Restrict /home mount options  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

| Restrict /tmp/ and /var/tmp mount options  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

| Restrict /dev/shm mount options  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

|   |   |

| Polyinstantiated /tmp and /var/tmp  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

|   |   |

| Set group for /dev/shm  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

|   |   |

| Encrypt swap  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

# Physical Access

https://github.com/trimstray/linux-hardening-checklist#physical-access

## Password for Single User Mode

https://github.com/trimstray/linux-hardening-checklist#password-for-single-user-mode

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Protect Single User Mode with root password.

Example:

```
# Edit /etc/sysconfig/init.
SINGLE=/sbin/sulogin
```

## ☑️ Summary checklist

https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist-1

| Rule  | Priority  | Checkbox

| Protect Single User Mode.  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

# Bootloader

https://github.com/trimstray/linux-hardening-checklist#bootloader

## Protect bootloader config files

https://github.com/trimstray/linux-hardening-checklist#protect-bootloader-config-files

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Ensure bootloader config files are set properly permissions.

Example:

```
# Set the owner and group of /etc/grub.conf to the root user:
chown root:root /etc/grub.conf
chown -R root:root /etc/grub.d

# Set permissions on the /etc/grub.conf or /etc/grub.d file to read and write for root only:
chmod og-rwx /etc/grub.conf
chmod -R og-rwx /etc/grub.d
```

## ☑️ Summary checklist

https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist-2

| Rule  | Priority  | Checkbox

| Protect bootloader config files  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

# Linux Kernel

https://github.com/trimstray/linux-hardening-checklist#linux-kernel

## Kernel logs

https://github.com/trimstray/linux-hardening-checklist#kernel-logs

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Restricting access to kernel logs.

Example:

```
echo "kernel.dmesg_restrict = 1" > /etc/sysctl.d/50-dmesg-restrict.conf
```

## Kernel pointers

https://github.com/trimstray/linux-hardening-checklist#kernel-pointers

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Restricting access to kernel pointers.

Example:

```
echo "kernel.kptr_restrict = 1" > /etc/sysctl.d/50-kptr-restrict.conf
```

## ExecShield

https://github.com/trimstray/linux-hardening-checklist#execshield

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png ExecShield protection.

Example:

```
echo "kernel.exec-shield = 2" > /etc/sysctl.d/50-exec-shield.conf
```

## Memory protections

https://github.com/trimstray/linux-hardening-checklist#memory-protections

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Randomise memory space.

```
echo "kernel.randomize_va_space=2" > /etc/sysctl.d/50-rand-va-space.conf
```

## ☑️ Summary checklist

https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist-3

| Rule  | Priority  | Checkbox

| Restricting access to kernel logs  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

| Restricting access to kernel pointers  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

| ExecShield protection  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

| Randomise memory space.  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

# Logging

https://github.com/trimstray/linux-hardening-checklist#logging

## Syslog

https://github.com/trimstray/linux-hardening-checklist#syslog

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Ensure syslog service is enabled and running.

Example:

```
systemctl enable rsyslog
systemctl start rsyslog
```

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Send syslog data to external server.

Example:

```
# ELK
# Logstash
# Splunk
# ...
```

## ☑️ Summary checklist

https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist-4

| Rule  | Priority  | Checkbox

| Ensure syslog service is enabled and running.  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

| Ensure syslog service is enabled and running.  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

# Users and Groups

https://github.com/trimstray/linux-hardening-checklist#users-and-groups

## Passwords

https://github.com/trimstray/linux-hardening-checklist#passwords

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Update password policy (PAM).

Example:

```
authconfig --passalgo=sha512 \
--passminlen=14 \
--passminclass=4 \
--passmaxrepeat=2 \
--passmaxclassrepeat=2 \
--enablereqlower \
--enablerequpper \
--enablereqdigit \
--enablereqother \
--update
```

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Limit password reuse (PAM).

Example:

```
# Edit /etc/pam.d/system-auth

# For the pam_unix.so case:
password sufficient pam_unix.so ... remember=5

# For the pam_pwhistory.so case:
password requisite pam_pwhistory.so ... remember=5
```

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Secure /etc/login.defs password policy.

Example:

```
# Edit /etc/login.defs
PASS_MIN_LEN 14
PASS_MIN_DAYS 1
PASS_MAX_DAYS 60
PASS_WARN_AGE 14
```

## Logon Access

https://github.com/trimstray/linux-hardening-checklist#logon-access

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Set auto logout inactive users.

Example:

```
echo "readonly TMOUT=900" >> /etc/profile.d/idle-users.sh
echo "readonly HISTFILE" >> /etc/profile.d/idle-users.sh
chmod +x /etc/profile.d/idle-users.sh
```

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Set last logon/access notification.

Example:

```
# Edit /etc/pam.d/system-auth
session required pam_lastlog.so showfailed
```

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Lock out accounts after a number of incorrect login (PAM).

Example:

```
# Edit /etc/pam.d/system-auth and /etc/pam.d/password-auth

# Add the following line immediately before the pam_unix.so statement in the AUTH section:
auth required pam_faillock.so preauth silent deny=3 unlock_time=never fail_interval=900

# Add the following line immediately after the pam_unix.so statement in the AUTH section:
auth [default=die] pam_faillock.so authfail deny=3 unlock_time=never fail_interval=900

# Add the following line immediately before the pam_unix.so statement in the ACCOUNT section:
account required pam_faillock.so
```

## ☑️ Summary checklist

https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist-5

| Rule  | Priority  | Checkbox

| Update password policy  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

| Limit password reuse  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

| Secure /etc/login.defs password policy  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

|   |   |

| Set auto logout inactive users.  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

| Set last logon/access notification  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

| Lock out accounts after a number of incorrect login  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

# Filesystem

https://github.com/trimstray/linux-hardening-checklist#filesystem

## Hardlinks & Symlinks

https://github.com/trimstray/linux-hardening-checklist#hardlinks--symlinks

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png Enable hard/soft link protection.

Example:

```
echo "fs.protected_hardlinks = 1" > /etc/sysctl.d/50-fs-hardening.conf
echo "fs.protected_symlinks = 1" >> /etc/sysctl.d/50-fs-hardening.conf
```

## Dynamic Mounting and Unmounting

https://github.com/trimstray/linux-hardening-checklist#dynamic-mounting-and-unmounting

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Disable uncommon filesystems.

Example:

```
echo "install cramfs /bin/false" > /etc/modprobe.d/uncommon-fs.conf
echo "install freevxfs /bin/false" > /etc/modprobe.d/uncommon-fs.conf
echo "install jffs2 /bin/false" > /etc/modprobe.d/uncommon-fs.conf
echo "install hfs /bin/false" > /etc/modprobe.d/uncommon-fs.conf
echo "install hfsplus /bin/false" > /etc/modprobe.d/uncommon-fs.conf
echo "install squashfs /bin/false" > /etc/modprobe.d/uncommon-fs.conf
echo "install udf /bin/false" > /etc/modprobe.d/uncommon-fs.conf
echo "install fat /bin/false" > /etc/modprobe.d/uncommon-fs.conf
echo "install vfat /bin/false" > /etc/modprobe.d/uncommon-fs.conf
echo "install nfs /bin/false" > /etc/modprobe.d/uncommon-fs.conf
echo "install nfsv3 /bin/false" > /etc/modprobe.d/uncommon-fs.conf
echo "install gfs2 /bin/false" > /etc/modprobe.d/uncommon-fs.conf
```

## ☑️ Summary checklist

https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist-6

| Rule  | Priority  | Checkbox

| Enable hard/soft link protection.  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/low.png  | 🔲

| Disable uncommon filesystems.  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

# Permissions

https://github.com/trimstray/linux-hardening-checklist#permissions

# SELinux & Auditd

https://github.com/trimstray/linux-hardening-checklist#selinux--auditd

## SELinux Enforcing

https://github.com/trimstray/linux-hardening-checklist#selinux-enforcing

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/high.png Set SELinux Enforcing mode.

Example:

```
# Edit /etc/selinux/config.
SELINUXTYPE=enforcing
```

## ☑️ Summary checklist

https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist-7

| Rule  | Priority  | Checkbox

| Set SELinux Enforcing mode.  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/high.png  | 🔲

# System Updates

https://github.com/trimstray/linux-hardening-checklist#system-updates

# Network

https://github.com/trimstray/linux-hardening-checklist#network

## TCP/SYN

https://github.com/trimstray/linux-hardening-checklist#tcpsyn

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Enable TCP SYN Cookie protection.

Example:

```
echo "net.ipv4.tcp_syncookies = 1" > /etc/sysctl.d/50-net-stack.conf
```

## Routing

https://github.com/trimstray/linux-hardening-checklist#routing

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Disable IP source routing.

Example:

```
echo "net.ipv4.conf.all.accept_source_route = 0" > /etc/sysctl.d/50-net-stack.conf
```

## ICMP Protocol

https://github.com/trimstray/linux-hardening-checklist#icmp-protocol

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Disable ICMP redirect acceptance.

Example:

```
echo "net.ipv4.conf.all.accept_redirects = 0" > /etc/sysctl.d/50-net-stack.conf
```

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Enable ignoring to ICMP requests.

Example:

```
echo "net.ipv4.icmp_echo_ignore_all = 1" > /etc/sysctl.d/50-net-stack.conf
```

## Broadcast

https://github.com/trimstray/linux-hardening-checklist#broadcast

-

https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png Enable ignoring broadcasts request.

Example:

```
echo "net.ipv4.icmp_echo_ignore_broadcasts = 1" > /etc/sysctl.d/50-net-stack.conf
```

## ☑️ Summary checklist

https://github.com/trimstray/linux-hardening-checklist#ballot_box_with_check-summary-checklist-8

| Rule  | Priority  | Checkbox

| Enable TCP SYN Cookie protection.  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

|   |   |

| Disable IP source routing.  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

|   |   |

| Disable ICMP redirect acceptance.  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

| Enable ignoring to ICMP requests.  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

|   |   |

| Enable ignoring broadcasts request.  | https://github.com/trimstray/linux-hardening-checklist/blob/master/static/img/medium.png  | 🔲

# Services

https://github.com/trimstray/linux-hardening-checklist#services

# Tools

https://github.com/trimstray/linux-hardening-checklist#tools
