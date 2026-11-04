---
url: "https://discourse.ubuntu.com/t/private-home-directories-for-ubuntu-21-04-onwards/19533"
title: "Private home directories for Ubuntu 21.04 onwards?"
site: "Ubuntu Discourse"
published: "2020-11-27T01:57:31+00:00"
captured: "2026-11-03T02:57:13Z"
via: "live"
sha256: "834b3527d813fd2982a3b082e0bf5a542731d2cfe57673d1a02c3df7596183d2"
wayback: "https://web.archive.org/web/20260518104847/https://discourse.ubuntu.com/t/private-home-directories-for-ubuntu-21-04-onwards/19533"
cited_in:
  - "docs/research/hardening.md"
---

# Private home directories for Ubuntu 21.04 onwards?

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://discourse.ubuntu.com/t/private-home-directories-for-ubuntu-21-04-onwards/19533>
> Snapshot: <https://web.archive.org/web/20260518104847/https://discourse.ubuntu.com/t/private-home-directories-for-ubuntu-21-04-onwards/19533>

[Ubuntu Discourse](https://discourse.ubuntu.com/)

#  [Private home directories for Ubuntu 21.04 onwards?](https://discourse.ubuntu.com/t/private-home-directories-for-ubuntu-21-04-onwards/19533)

[Project Discussion](https://discourse.ubuntu.com/c/project/307)    [Security](https://discourse.ubuntu.com/c/project/security/33)

[alexmurray](https://discourse.ubuntu.com/u/alexmurray)     November 27, 2020, 1:57am   1

(Cross-posting from the [email to ubuntu-devel-discuss](https://lists.ubuntu.com/archives/ubuntu-devel-discuss/2020-November/018842.html) earlier)

After more than [14 years](https://bugs.launchpad.net/ubuntu/+source/adduser/+bug/48734) of debate, I propose that it is time we moved ahead and stopped creating home directories as world-readable on Ubuntu for hirsute (21.04) onwards. The old arguments from the bug referenced in [the launchpad bug](https://bugs.launchpad.net/ubuntu/+source/adduser/+bug/48734) mainly centered on the convenience of this feature when considered in regards to a shared desktop machine with multiple user accounts wanting to easily share files with one-another. However, a lot of things have changed in the last 14 years, not least of which that Ubuntu has a significant customer and user-base in the public cloud and server space. For these users, there is generally 1 admin account and perhaps a number of less privileged worker accounts, and so world-readable home directories now present more like a footgun than a feature - in this case, if a worker account is compromised, an attacker could now more easily access sensitive data from the other worker accounts or the admin account.

Whilst the Ubuntu Security team does a great job of staying on top of security updates and keeping the distro packages as secure as possible, there will always be [instances](https://ruffell.nz/reverse-engineering/writeups/2020/10/27/analysis-of-the-dovecat-and-hy4-linux-malware.html) where for whatever reason machines are not kept up-to-date or weak passwords are used and so they become compromised. We should therefore be taking an approach to limit access in this unlikely event.

The other part of the past argument for this was that knowledgeable users / sysadmins will be aware of this default and will change it if they deem necessary. Whilst I am sure that we do have many knowledgeable users, we have many more who simply are deploying Ubuntu to solve other tasks - and I think it goes against the usability proposition of Ubuntu in these cases to expect admins to make this change. Instead we can ensure these deployments limit the chance for sensitive information to be compromised by changing this default.

It should be noted too that from a regression point-of-view, changing this default will also not affect any permissions on existing installs (if it was perhaps decided to SRU this change back to older releases) or on upgrades - only if new users are created will they then have these more restrictive permissions. By making this change now, this also gives 3 development releases and 2 interim releases to work through any unforseen issues etc before landing in an LTS release. Without some widespread testing it is not possible to know in advance all of the possible use-cases that have depended on this behaviour which may then have issues so I feel now is the time to make such a change so we can determine any appropriate work-arounds and associated documentation etc as needed.

As such, for a concrete proposal, I suggest changing /etc/adduser.conf to specify DIR_MODE=0750 instead of the current DIR_MODE=0755 - this will ensure any users created via adduser have their home-directory non-readable and non-executable by others.

In the case of [useradd](https://manpages.ubuntu.com/manpages/hirsute/en/man8/useradd.8.html) (the low-level util) this is a bit trickier - whilst the documentation suggests we can set HOME_MODE in /etc/login.defs this does not appear to be respected and only if we set say UMASK=027 in /etc/login.defs and then create a user (useradd -m -d /home/test test) does the home dir have the expected permissions. However, modifying the default umask has other consequences so I am not suggesting we consider that at this point. So this will need some more investigation, for now I would like to focus on adduser as this is the documented approach for adding new Ubuntu users .

If you want to try this yourself, you can simply:

```
# ensure future users homes are safe
sudo sed -i s/DIR_MODE=0755/DIR_MODE=0750/ /etc/adduser.conf

# then get your own house in order
chmod 750 $HOME

```

In regards to things to watch out for, one use-case that I have come across myself is libvirt VMs where the disk image is stored in your home directory - these become unreadable now to the libvirt-qemu user and so you can’t launch these anymore  - however, there is a simple fix for
this  - you can use an ACL entry to re-enable this access for just the libvirt-qemu user:

```
setfacl -m u:libvirt-qemu:rx $HOME

```

(or you could just store your images under /var/lib/libvirt/images)

I suspect this ACL approach may be needed for other common use-cases but like I said above this remains to be seen so any testing others could give of this would be really appreciated in helping to decide whether to try and proceed with this change.

Finally, if there is a strong case for deployments who rely on the existing functionality (say universities etc) where having to manually roll-back the setting on each machine install would be painful, we could look at adding some functionality to the installer/preseed/whatnot to create initial users etc with the old permissions.

Thanks for taking the time to read this - any constructive feedback is welcome.

14 Likes

[Automated Server install quickstart](https://discourse.ubuntu.com/t/automated-server-install-quickstart/16614/34)

[alexmurray](https://discourse.ubuntu.com/u/alexmurray)    January 13, 2021, 6:39am   2

Just to follow up on this thread - since there was no opposition to this proposal, I have uploaded updated adduser and shadow packages to hirsute-proposed to support setting the mode of home directories to 750 by default when they are created via either adduser or useradd.

Let me know if you encounter any significant issues

3 Likes

[olafvdspek](https://discourse.ubuntu.com/u/olafvdspek)    January 13, 2021, 8:53pm   3

So [Bug #675560 “Home dirs shouldn't be world readable” : Bugs : adduser package : Ubuntu](https://bugs.launchpad.net/ubuntu/+source/adduser/+bug/675560) can be marked as fixed rather than as invalid?

[alexmurray](https://discourse.ubuntu.com/u/alexmurray)    January 14, 2021, 7:10am   4

Thanks for raising that - I’ve marked that bug https://bugs.launchpad.net/ubuntu/+source/adduser/+bug/675560 as a duplicate of the original “primary” one mentioned in the first post in this thread and updated that primary with the current status.

1 Like

[gp057788](https://discourse.ubuntu.com/u/gp057788)    January 15, 2021, 8:31am   5

One (rather old) feature impacted could be apache web server userrdir support
(~/user web, mapped to /home/user/public_html).

It was a '90s “feature”. I hope deprecated.

https://superuser.com/questions/962784/apache-userdir-home-folder-permissions

cron, mail, lp taken a different approach: user dirs under /var/spool

[gp057788](https://discourse.ubuntu.com/u/gp057788)    January 15, 2021, 8:46am   6

User namespaces are impacted?

Docker/Podman, Snap/Flatpak, Selinux/AppArmor ?

Mode 750 may be too permissive (if users are in ‘users’ default group),
To share files, ‘002’ is good for umask (and setgid on dirs)

[gp057788](https://discourse.ubuntu.com/u/gp057788)    January 15, 2021, 9:27am   7

Anyway, something must be done

this do not work (ok):

```
find ../myrichclassmate/Dropbox -name '*bank*'

cat ../a.sysadmin/.ssh/id_rsa

```

but this works ! (NOT ok):

```
find ../myexfriend/Videos -name '*my-secret-videos*'

```

under ‘~/snap’ some credential store (git http) ?

[jdstrand](https://discourse.ubuntu.com/u/jdstrand)    January 15, 2021, 7:05pm   8

Snaps themselves should actually be ok because we use “owner” match in the AppArmor policy. This change should only mean that DAC rejects it before AppArmor does if a snap run by one user tries to access files in home from another user.

[gp057788](https://discourse.ubuntu.com/u/gp057788)    January 16, 2021, 1:49am   9

I do not know what happens in hybrid envs:
Snap+selinux (redhat) vs Flatpak+AppArmor (ubuntu).
Probably acces to “…/another/snap/*” is restricted at application level (like “advisory locking”…)

One point is not clear to me:

With “podman” you can have rootless containers, where container root is mapped to running user, with images stored under ~/.local (with restricted access).
But, if inside the container the process owner changes, it will be mapped by namespace user mapping to a different os user, without any special privileges.
If the user home is restricted, containers cannot “see” volume binding inside home.
How can this work?
I need to enable container based development under full user control in untrusted multiuser env (hpc classrooms)

[danilo](https://discourse.ubuntu.com/u/danilo)    February 14, 2021, 12:32am   10

I think you should highlight a very common problem with encrypted home directories: openssh-server can’t see ~/.ssh/authorized_keys files, thus SSH log in does not work by default.

I usually resolve that by moving SSH authorized_keys files to /etc/ssh/users/%u, but that does not work for the general case (because /etc might be mounted read-only).

1 Like

[zippy.zebu](https://discourse.ubuntu.com/u/zippy.zebu)    July 13, 2021, 11:57am   11

[@alexmurray](https://discourse.ubuntu.com/u/alexmurray)

Telegram and many other purple plugins, (and evince) is now do not work unless I modify that apparmor settings or restore it to 755. Please try to fix most common problem before landing something like this.

What are your suggestion regarding this?

[alexmurray](https://discourse.ubuntu.com/u/alexmurray)    July 19, 2021, 12:41am   12

[@zippy.zebu](https://discourse.ubuntu.com/u/zippy.zebu) can you either file bugs for these issues or at least provide more information on the errors etc that you see, then I can try and help?

[alowther](https://discourse.ubuntu.com/u/alowther)    January 10, 2022, 7:50pm   13

A common situation is to have a central set of users (e.g. in LDAP) and use pam_mkhomedir.so to create the home directory when the user first logs in.

These changes do not cover this situation. The default configuration of pam_mkhomedir.so will result in a home directory created with 0755 permissions.

To make pam_mkhomedir.so create a home directory by default with permissions consistent with the other tools then a umask argument can be added to the pam_mkhomedir.so module in the file /usr/share/pam-configs/mkhomedir. I believe this would have to be done before enabling the module. The file is part of the libpam-modules package.

1 Like

[alexmurray](https://discourse.ubuntu.com/u/alexmurray)    January 11, 2022, 1:54am   14

Thanks for the info re pam-mkhomedir - I have filed a bug on launchpad (https://bugs.launchpad.net/ubuntu/+source/pam/+bug/1957024) to track this. If you can provide any info on a test-case for this, could you please do so in a comment on that bug report? Thanks.

1 Like

[pponnuvel](https://discourse.ubuntu.com/u/pponnuvel)    August 23, 2024, 4:10pm   15

Hi [@alexmurray](https://discourse.ubuntu.com/u/alexmurray),

I am looking SRU’ing the proposed patch for [LP#1957024](https://bugs.launchpad.net/ubuntu/+source/pam/+bug/1957024). Do you have feedback on the [patch](https://launchpadlibrarian.net/690083485/private_homedir.patch)? It seems reasonable but I don’t know the full implications of such a change. So I’d appreciate your comment and whether I can proceed with SRU’ing it. Thanks.

[alexmurray](https://discourse.ubuntu.com/u/alexmurray)    August 28, 2024, 11:46pm   16

Whether it is appropriate as an SRU is up to the SRU team - the first step would be landing this change as a [FFe](https://wiki.ubuntu.com/FreezeExceptionProcess) in the current devel release (oracular) and then proposing it as an SRU.

But from my perspective, my main concern with this patch when I first looked at it, was that a sysadmin may have edited /etc/adduser.conf to specify a different DIR_MODE or similar and now with this umask in pam-mkhomedir it would not be honoured - however the adduser configuration is not being honoured at the moment anyway by pam-mkhomedir - so then I think this change to make the default configuration of pam-mkhomedir consistent with the default configuration of adduser seems appropriate - +1 from me FWIW.

2 Likes
