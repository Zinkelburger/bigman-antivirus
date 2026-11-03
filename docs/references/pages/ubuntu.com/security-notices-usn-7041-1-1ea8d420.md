---
url: "https://ubuntu.com/security/notices/USN-7041-1"
title: "USN-7041-1: CUPS vulnerability | Ubuntu security notices | Ubuntu"
site: "Ubuntu"
captured: "2026-11-03T02:58:32Z"
via: "live"
sha256: "5580412661f58b3f069a3aece35c283cb0257ba48cfb2187dc45c5ff2a221841"
wayback: "https://web.archive.org/web/20260606113647/https://ubuntu.com/security/notices/USN-7041-1"
cited_in:
  - "docs/research/hardening.md"
---

# USN-7041-1: CUPS vulnerability | Ubuntu security notices | Ubuntu

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://ubuntu.com/security/notices/USN-7041-1>
> Snapshot: <https://web.archive.org/web/20260606113647/https://ubuntu.com/security/notices/USN-7041-1>

-  [Ubuntu Security Notices](https://ubuntu.com/security/notices)
-  USN-7041-1

# USN-7041-1: CUPS vulnerability

Publication date

26 September 2024

Overview

CUPS could be made to crash or run programs if it received specially crafted network traffic.

Releases

[24.04 LTS](https://ubuntu.com/security/notices?release=noble) [22.04 LTS](https://ubuntu.com/security/notices?release=jammy) [20.04 LTS](https://ubuntu.com/security/notices?release=focal)

## Packages

-  [cups](https://ubuntu.com/security/cves?package=cups) - Common UNIX Printing System(tm)

## Details

Simone Margaritelli discovered that CUPS incorrectly sanitized IPP
data when creating PPD files. A remote attacker could possibly use this
issue to manipulate PPD files and execute arbitrary code when a printer is
used.

Simone Margaritelli discovered that CUPS incorrectly sanitized IPP
data when creating PPD files. A remote attacker could possibly use this
issue to manipulate PPD files and execute arbitrary code when a printer is
used.

## Update instructions

In general, a standard system update will make all the necessary changes.

[Learn more about how to get the fixes.](https://ubuntu.com/security/cves/about#security)

The problem can be corrected by updating your system to the following package versions:

| Ubuntu Release  | Package Version

|  24.04 LTS noble   |  [cups](https://launchpad.net/ubuntu/+source/cups) – [2.4.7-1.2ubuntu7.3](https://launchpad.net/ubuntu/+source/cups/2.4.7-1.2ubuntu7.3)

|  22.04 LTS jammy   |  [cups](https://launchpad.net/ubuntu/+source/cups) – [2.4.1op1-1ubuntu4.11](https://launchpad.net/ubuntu/+source/cups/2.4.1op1-1ubuntu4.11)

|  20.04 LTS focal   |  [cups](https://launchpad.net/ubuntu/+source/cups) – [2.3.1-9ubuntu1.9](https://launchpad.net/ubuntu/+source/cups/2.3.1-9ubuntu1.9)

### Reduce your security exposure

Ubuntu Pro provides ten-year security coverage to 25,000+ packages in Main and Universe repositories, and it is free for up to five machines.

[Get Ubuntu Pro](https://ubuntu.com/pro)

## References

-  [CVE-2024-47175](https://ubuntu.com/security/CVE-2024-47175)

-  [CVE-2024-47175](https://ubuntu.com/security/CVE-2024-47175)

## Related notices

-  [USN-7045-1](https://ubuntu.com/security/notices/USN-7045-1)
-  [USN-7041-2](https://ubuntu.com/security/notices/USN-7041-2)
-  [USN-7041-3](https://ubuntu.com/security/notices/USN-7041-3)

-  [USN-7045-1](https://ubuntu.com/security/notices/USN-7045-1)
-  [USN-7041-2](https://ubuntu.com/security/notices/USN-7041-2)
-  [USN-7041-3](https://ubuntu.com/security/notices/USN-7041-3)

Have additional questions?

[Talk to a member of the team ›](https://wiki.ubuntu.com/SecurityTeam/FAQ?_ga=2.242131138.1744859116.1742138161-1500419473.1726851136&_gl=1*161muja*_gcl_au*MzUyMTIzNjI1LjE3MzYyNDQ4NTE.#Contact)
