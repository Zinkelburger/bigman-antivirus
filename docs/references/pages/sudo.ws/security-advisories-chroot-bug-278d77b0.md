---
url: "https://www.sudo.ws/security/advisories/chroot_bug/"
title: "Local Privilege Escalation via chroot option"
site: "Sudo"
published: "2025-06-30T14:00:00+00:00"
captured: "2026-11-02T19:00:47Z"
via: "live"
sha256: "cf4f2af0b91094aad57aef11cdcb4eeb15b39945c8326bcefadd989dc2149c54"
wayback: "https://web.archive.org/web/20261026041215/https://www.sudo.ws/security/advisories/chroot_bug/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Local Privilege Escalation via chroot option

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.sudo.ws/security/advisories/chroot_bug/>
> Snapshot: <https://web.archive.org/web/20261026041215/https://www.sudo.ws/security/advisories/chroot_bug/>

# Local Privilege Escalation via chroot option

Jun 30, 2025

An attacker can leverage sudo’s -R (--chroot) option to run arbitrary commands as root, even if they are not listed in the sudoers file.

##  Sudo versions affected: https://www.sudo.ws/security/advisories/chroot_bug/#sudo-versions-affected

Sudo versions 1.9.14 to 1.9.17 inclusive are affected.

##  CVE ID: https://www.sudo.ws/security/advisories/chroot_bug/#cve-id

This vulnerability has been assigned [CVE-2025-32463](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2025-32463) in the [Common Vulnerabilities and Exposures](https://cve.mitre.org/) database.

##  Details: https://www.sudo.ws/security/advisories/chroot_bug/#details

Sudo’s -R (--chroot) option is intended to allow the user to run a command with a user-selected root directory if the sudoers file allows it. A change was made in sudo 1.9.14 to resolve paths via chroot() using the user-specified root directory while the sudoers file was still being evaluated. It is possible for an attacker to trick sudo into loading an arbitrary shared library by creating an /etc/nsswitch.conf file under the user-specified root directory.

The change from sudo 1.9.14 has been reverted in sudo 1.9.17p1 and the chroot feature has been marked as deprecated. It will be removed entirely in a future sudo release. Because of the way sudo resolves commands, supporting a user-specified chroot directory is error-prone and this feature does not appear to be widely used.

A more detailed description of the bug and its effects can be found in the [Stratascale advisory](https://www.stratascale.com/vulnerability-alert-CVE-2025-32463-sudo-chroot).

##  Impact: https://www.sudo.ws/security/advisories/chroot_bug/#impact

On systems that support /etc/nsswitch.conf a user may be able to run arbitrary commands as root.

##  Fix: https://www.sudo.ws/security/advisories/chroot_bug/#fix

The bug is fixed in sudo 1.9.17p1.

##  Credit: https://www.sudo.ws/security/advisories/chroot_bug/#credit

Thanks to Rich Mirch from Stratascale Cyber Research Unit (CRU) for reporting and analyzing the bug. His advisory may be found at https://www.stratascale.com/vulnerability-alert-CVE-2025-32463-sudo-chroot.

[Local Privilege Escalation via host option](https://www.sudo.ws/security/advisories/host_any/)
