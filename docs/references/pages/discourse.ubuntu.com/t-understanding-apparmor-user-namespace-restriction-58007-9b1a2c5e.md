---
url: "https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007"
title: "Understanding AppArmor User Namespace Restriction"
site: "Ubuntu Discourse"
published: "2025-03-27T16:56:56+00:00"
captured: "2026-11-02T18:26:41Z"
via: "live"
sha256: "5a968388962391bd29b643b73a621386d7e4481e770140528efcd70a5da4a424"
wayback: "https://web.archive.org/web/20261024173118/https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Understanding AppArmor User Namespace Restriction

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007>
> Snapshot: <https://web.archive.org/web/20261024173118/https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007>

[Ubuntu Discourse](https://discourse.ubuntu.com/)

#  [Understanding AppArmor User Namespace Restriction](https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007)

[Project Discussion](https://discourse.ubuntu.com/c/project/307)    [Security](https://discourse.ubuntu.com/c/project/security/33)

[mbelair](https://discourse.ubuntu.com/u/mbelair)     March 27, 2025, 4:56pm   1

# https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007#p-148026-understanding-the-apparmor-user-namespace-restriction-feature-1Understanding the AppArmor user namespace restriction feature

Ubuntu 23.10 and 24.04 LTS introduced new AppArmor-based features to reduce the attack surface presented by unprivileged user namespaces in the Linux kernel. Unprivileged user namespaces are a feature in the Linux kernel that was introduced in order to provide additional sandboxing functionality for programs such as container runtimes; it enables unprivileged users to gain administrator (root) permissions within a confined environment, without giving them elevated permissions on the host system.

Unprivileged user namespaces have been repeatedly used to exploit kernel vulnerabilities, so the AppArmor restriction acts as a security hardening measure. Our long-term plan is to implement further tightening rules in AppArmor, with the current set of features acting as a stepping stone towards a stronger security posture.

[The security researchers at Qualys](https://www.qualys.com/three-bypasses-of-Ubuntu-unprivileged-user-namespace-restrictions) have identified three different ways through which this hardening feature can, under certain circumstances, be bypassed. They have responsibly disclosed these to the Ubuntu Security Team in January and our teams have been coordinating the release of updated documentation and mitigation instructions, as well as ways to improve this feature (you can find the original advisory text [here](https://www.qualys.com/2025/three-bypasses-of-Ubuntu-unprivileged-user-namespace-restrictions.txt)). These are not security vulnerabilities, as Ubuntu installations benefit from an extra layer of hardening with the AppArmor protections, despite the limitations identified.

While a superficial observation of the application of user namespaces may indicate privileged (root level) access, this is a fictitious state that is operating as expected, with access control still mapped to the real (root namespace) user’s permissions. As such, these bypasses do not enable more access than what the default Linux kernel unprivileged user namespace feature allows in most Linux distributions. They do, however, demonstrate limitations that we are looking to address in order to strengthen existing protections against as-of-yet-unknown Linux kernel vulnerabilities.

In this post we will explain what the AppArmor user namespace restriction is and how to further harden an Ubuntu installation against possible bypasses, such as the ones that Qualys have discovered.

# https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007#p-148026-why-the-apparmor-user-namespace-restriction-feature-was-developed-2Why the AppArmor user namespace restriction feature was developed

Because of the attack surface exposed by user namespaces in the Linux kernel and the vulnerabilities involving them, Ubuntu, Debian and other distros have carried a big hammer patch to be able to globally disable unprivileged user namespaces. This patch, however, can only ever be enabled at the system level as an emergency mitigation because it breaks too many applications (such as desktop environments, Flatpak, LXD, container runtimes, etc.).

Instead of a big all or nothing switch, AppArmor’s unprivileged user namespace restriction allows selectively enabling/disabling the use of unprivileged user namespaces, as well as the use of capabilities within a user namespace. The degree of enablement or restriction is under the control of a privileged admin and can be dynamically scaled, allowing for selective mitigation of an issue to be applied or updated on a live system.

The AppArmor unprivileged user namespace restriction does not stop kernel exploits directly. Instead, it reduces the attack surface presented by restricting the use of unprivileged user namespaces to applications that have a legitimate use for them. Administrators have control on how much they want to harden the system to improve its security.

# https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007#p-148026-bypassing-apparmor-security-restriction-3Bypassing AppArmor security restriction

It is not possible to completely eliminate the risk introduced by unprivileged user namespaces without disabling/restricting the feature. The protection provided by AppArmor is always a tradeoff between security and usability. For this reason, several methods, as detailed by the Qualys security researchers, could be used to bypass these protections. Administrators can choose to disable or add bypasses to the protection. However, an imperfect protection remains better than no protection at all.

For example, unconfined profiles that allow the use of user namespaces provide a trivial bypass of the unprivileged user namespace restriction from unconfined processes. All the unconfined application or user has to do is enter the unconfined profile. The unconfined user or application can execute the application associated with the unconfined profile. If the application provides the ability to run arbitrary code or shell access, that is all that needs to be done to bypass the restriction. The busybox shell is such an example and it is available by default on standard Ubuntu installations.

# https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007#p-148026-why-the-protections-still-matter-4Why the protections still matter

Even if you can bypass the AppArmor restrictions in a general-purpose shell, this does not make the functionality ineffective in all circumstances. In fact, any application that is either properly confined using AppArmor or which is not affected by vulnerabilities that allow breaking out to a general-purpose shell (such as busybox) will still benefit from the functionality.

The restriction will continue to be developed, further hardening Ubuntu systems and reducing the attack surface of Linux kernel vulnerabilities that can be exploited through unprivileged user namespaces.

# https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007#p-148026-minimizing-the-attack-surface-5Minimizing the attack surface

Several approaches can be used to further strengthen Ubuntu installations and mitigate against the first two of the three bypasses discovered by Qualys. By combining them, the security level of the system can be significantly improved. Please note that these hardening steps are only applicable on releases after Ubuntu 23.10 and will be enabled by default in future releases.

## https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007#p-148026-restrict-unprivileged-unconfined-profile-changes-6Restrict unprivileged unconfined profile changes

By making sure the kernel.apparmor_restrict_unprivileged_unconfined sysctl setting is enabled, a process that is unprivileged and unconfined cannot leverage aa-exec to change to a more favourable profile. This can be achieved with the following:

```
echo "kernel.apparmor_restrict_unprivileged_unconfined=1" | sudo tee /etc/sysctl.d/10-apparmor-hardening.conf
sudo sysctl --load /etc/sysctl.d/10-apparmor-hardening.conf

```

Please note that, if installed, LXD will completely disable the user namespace restriction feature when running, effectively making this sysctl irrelevant.

## https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007#p-148026-remove-broad-apparmor-profiles-7Remove broad AppArmor profiles

Ubuntu ships with default unconfined profiles for several applications that allow the creation of user namespaces. This is done because users may reasonably be expected to use functionality within these applications that rely on user namespace creation.

Busybox is a general-purpose shell that also implements a set of common command line utilities. A broad AppArmor profile is provided because we cannot predict how it would be used. However, if the creation of user namespaces (such as through the unshare command) is not required, the profile can be disabled with the following command:

```
sudo ln -s /etc/apparmor.d/busybox /etc/apparmor.d/disable
sudo apparmor_parser -R /etc/apparmor.d/busybox

```

Nautilus is a file manager for the GNOME desktop environment and it is part of standard Ubuntu Desktop installations. It makes use of unprivileged user namespaces for some operations, which is why a default unconfined AppArmor profile allowing user namespace creation is shipped by Ubuntu. While some functionality, such as thumbnail generation, would no longer be available, its broad profile can be similarly disabled:

```
sudo ln -s /etc/apparmor.d/nautilus /etc/apparmor.d/disable
sudo apparmor_parser -R /etc/apparmor.d/nautilus

```

Please note that any other installed application with an unconfined profile that allows creation of user namespaces can be similarly used to bypass this functionality. These two are simply the only ones that apply to programs installed by default on Ubuntu. You may get the list of unconfined profiles and of processes using them with sudo aa-status --filter.mode=unconfined. It has to be noted that most programs will not be installed on your machine and therefore cannot cause any harm.

## https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007#p-148026-define-bwrap-profile-8Define bwrap profile

The Nautilus file manager uses bwrap for the functionality that relies on user namespace creation. Therefore, using a purpose-built bwrap profile can allow Nautilus to continue to function as intended, with one caveat – executing the file manager with administrative privileges (as root) will not retain the higher privileges for any operations that rely on user namespaces. This is usage that we strongly recommend against.

In addition to removing the broad Nautilus profile, you can add a bwrap profile based on the one from the [AppArmor repository](https://gitlab.com/apparmor/apparmor/-/blob/master/profiles/apparmor/profiles/extras/bwrap-userns-restrict?ref_type=heads), if it is not already present on the system.

Please note that the bwrap profile can be used by other applications that rely on its functionality, such as flatpak, libgnome-desktop or bubblejail. This step should only be followed by system administrators that have a strong grasp of AppArmor configuration and may interfere with future updates that will distribute this file in the apparmor package.

# https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007#p-148026-next-steps-9Next steps

In addition to preparing extensive documentation that explains the nuances behind the AppArmor hardening functionality, we are developing new features that will further decrease the attack surface in the event of unforeseen Linux kernel vulnerabilities. The current issues with the above mitigations will be addressed via new AppArmor features and made available by default on standard installations. These will be selectively backported to supported Ubuntu releases via Stable Release Updates or introduced in new releases.

As this is developed, we will update this post to link to the new documentation and feature release notes.

10 Likes

[Ubuntu Weekly Newsletter Issue 885](https://discourse.ubuntu.com/t/ubuntu-weekly-newsletter-issue-885/57815)

[Plucky Puffin Release Notes](https://discourse.ubuntu.com/t/plucky-puffin-release-notes/48687)

[glc](https://discourse.ubuntu.com/u/glc)    July 9, 2025, 4:29pm   2

Hi
I have just hit the restriction , when trying to switch Evolution from Ubuntu source for flatpak (to get app upgrades), After a long conversation with ChatGPT, I was able to get an understanding of where the issue is, is anyone aware if someone with in-depth AppArmor knowledge has created a tutorial on how go forward, I got hit in bwarp, flatpak and Evolution.
Thanks in advance
GC
