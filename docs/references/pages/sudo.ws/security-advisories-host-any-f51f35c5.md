---
url: "https://www.sudo.ws/security/advisories/host_any/"
title: "Local Privilege Escalation via host option"
site: "Sudo"
published: "2025-06-30T14:00:00+00:00"
captured: "2026-11-02T19:00:53Z"
via: "live"
sha256: "8cd6df9c8c9ec2a0c0691ec749f406e1ee9116bc421a62ab2afda672970cb5d8"
wayback: "https://web.archive.org/web/20261013082729/https://www.sudo.ws/security/advisories/host_any/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Local Privilege Escalation via host option

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.sudo.ws/security/advisories/host_any/>
> Snapshot: <https://web.archive.org/web/20261013082729/https://www.sudo.ws/security/advisories/host_any/>

# Local Privilege Escalation via host option

Jun 30, 2025

Sudo’s host (-h or --host) option is intended to be used in conjunction with the list option (-l or --list) to list a user’s sudo privileges on a host other than the current one. However, due to a bug it was not restricted to listing privileges and could be used when running a command via sudo or editing a file with sudoedit. Depending on the rules present in the sudoers file this could allow a local privilege escalation attack.

##  Sudo versions affected: https://www.sudo.ws/security/advisories/host_any/#sudo-versions-affected

Sudo versions 1.8.8 to 1.9.17 inclusive are affected.

##  CVE ID: https://www.sudo.ws/security/advisories/host_any/#cve-id

This vulnerability has been assigned [CVE-2025-32462](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2025-32462) in the [Common Vulnerabilities and Exposures](https://cve.mitre.org/) database.

##  Details: https://www.sudo.ws/security/advisories/host_any/#details

The intent of sudo’s -h (--host) option is to make it possible to list a user’s sudo privileges for a host other than the current one. It was only intended be used with in conjunction with the -l (--list) option.

The bug effectively makes the hostname portion of a sudoers rule irrelevant since the user can set the host to be used when evaluating the rules themselves. A user must still be listed in the sudoers file, but they do not needed to have an entry for the current host.

For example, given the sudoers rule:

```
alice cerebus = ALL

```

user alice would be able to run sudo -h cerebus id on any host, not just cerebus. For example:

```
alice@hades$ sudo -l
Sorry, user alice may not run sudo on hades.

alice@hades$ sudo -l -h cerebus
User alice may run the following commands on cerebus:
    (root) ALL

alice@hades$ sudo -h cerebus id
uid=0(root) gid=0(root) groups=0(root)

```

##  Impact: https://www.sudo.ws/security/advisories/host_any/#impact

Sudoers files that include rules where the host field is not the current host or ALL are affected. This primarily affects sites that use a common sudoers file that is distributed to multiple machines. Sites that use LDAP-based sudoers (including SSSD) are similarly impacted.

For example, a sudoers rule such as:

```
bob ALL = ALL

```

is not affected since the host ALL already matches any hosts, but a rule like:

```
alice cerebus = ALL

```

could allow user alice to run any command even if the current host is not cerebus.

##  Fix: https://www.sudo.ws/security/advisories/host_any/#fix

The bug is fixed in sudo 1.9.17p1.

##  Credit: https://www.sudo.ws/security/advisories/host_any/#credit

Thanks to Rich Mirch from Stratascale Cyber Research Unit (CRU) for reporting and analyzing the bug. His advisory may be found at https://www.stratascale.com/vulnerability-alert-CVE-2025-32462-sudo-host.

[Double free with per-command chroot sudoers rules](https://www.sudo.ws/security/advisories/double_free/) [Local Privilege Escalation via chroot option](https://www.sudo.ws/security/advisories/chroot_bug/)
