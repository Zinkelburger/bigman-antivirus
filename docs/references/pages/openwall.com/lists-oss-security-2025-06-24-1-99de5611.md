---
url: "https://www.openwall.com/lists/oss-security/2025/06/24/1"
title: "oss-security - CPython: Multiple CVEs (1 CRITICAL, 3 HIGH, 1 MODERATE) affecting the tarfile module"
captured: "2026-11-02T18:58:26Z"
via: "live"
sha256: "22fb1bee38b7092a159481345496359be5e05af6ea9569708a25e2cd5066da6f"
wayback: "https://web.archive.org/web/20251214012502/https://www.openwall.com/lists/oss-security/2025/06/24/1"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# oss-security - CPython: Multiple CVEs (1 CRITICAL, 3 HIGH, 1 MODERATE) affecting the tarfile module

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.openwall.com/lists/oss-security/2025/06/24/1>
> Snapshot: <https://web.archive.org/web/20251214012502/https://www.openwall.com/lists/oss-security/2025/06/24/1>

|  https://www.openwall.com/  |

- [Products](https://www.openwall.com/)

- [Openwall GNU/*/Linux server OS](https://www.openwall.com/Owl/)
- [Linux Kernel Runtime Guard](https://www.openwall.com/lkrg/)
- [John the Ripper password cracker](https://www.openwall.com/john/)

- [Free & Open Source for any platform](https://www.openwall.com/john/)
- [in the cloud](https://www.openwall.com/john/cloud/)
- [Pro for Linux](https://www.openwall.com/john/pro/linux/)
- [Pro for macOS](https://www.openwall.com/john/pro/macosx/)

- [Wordlists for password cracking](https://www.openwall.com/wordlists/)
- [passwdqc policy enforcement](https://www.openwall.com/passwdqc/)

- [Free & Open Source for Unix](https://www.openwall.com/passwdqc/)
- [Pro for Windows (Active Directory)](https://www.openwall.com/passwdqc/windows/)

- [yescrypt KDF & password hashing](https://www.openwall.com/yescrypt/)
- [yespower Proof-of-Work (PoW)](https://www.openwall.com/yespower/)
- [crypt_blowfish password hashing](https://www.openwall.com/crypt/)
- [phpass ditto in PHP](https://www.openwall.com/phpass/)
- [tcb better password shadowing](https://www.openwall.com/tcb/)
- [Pluggable Authentication Modules](https://www.openwall.com/pam/)
- [scanlogd port scan detector](https://www.openwall.com/scanlogd/)
- [popa3d tiny POP3 daemon](https://www.openwall.com/popa3d/)
- [blists web interface to mailing lists](https://www.openwall.com/blists/)
- [msulogin single user mode login](https://www.openwall.com/msulogin/)
- [php_mt_seed mt_rand() cracker](https://www.openwall.com/php_mt_seed/)

- [Services](https://www.openwall.com/services/)
- Publications

- [Articles](https://www.openwall.com/articles/)
- [Presentations](https://www.openwall.com/presentations/)

- Resources

- [Mailing lists](https://www.openwall.com/lists/)
- [Community wiki](https://openwall.info/wiki/)
- [Source code repositories (GitHub)](https://github.com/openwall)
- [File archive & mirrors](https://www.openwall.com/mirrors/)
- [How to verify digital signatures](https://www.openwall.com/signatures/)
- [OVE IDs](https://www.openwall.com/ove/)

- [What's new](https://www.openwall.com/news)

|

|  [Follow @Openwall on Twitter for new release announcements and other news](https://twitter.com/openwall)

[[<prev]](https://www.openwall.com/lists/oss-security/2025/06/23/2) [[next>]](https://www.openwall.com/lists/oss-security/2025/06/24/2) [[day]](https://www.openwall.com/lists/oss-security/2025/06/24/) [[month]](https://www.openwall.com/lists/oss-security/2025/06/) [[year]](https://www.openwall.com/lists/oss-security/2025/) [[list]](https://www.openwall.com/lists/oss-security/)

```

Message-ID: <d823ee80-b9cb-4d95-ac9f-6ca50523dbc2@oracle.com>
Date: Mon, 23 Jun 2025 18:30:03 -0700
From: Alan Coopersmith <alan.coopersmith@...cle.com>
To: oss-security@...ts.openwall.com
Subject: CPython: Multiple CVEs (1 CRITICAL, 3 HIGH, 1
 MODERATE) affecting the tarfile module

It looks like the following notice never got forwarded to this list.

A followup post added:

> There is a correction for the mitigation. The mitigation code CVE-2025-4517,
> CVE-2025-4330, CVE-2025-4138, and CVE-2024-12718 is now provided in this Gist:
> https://gist.github.com/sethmlarson/52398e33eff261329a0180ac1d54f42f

and a second followup post said:

> Correction for affectedness, an earlier description of the vulnerabilities
> mentioned that only Python 3.12 and later are affected, however the tarfile
> extraction filtering feature had been backported to earlier Python versions.
> If using extraction filtering in versions earlier than 3.12 then that usage
> may be affected. As always, please look to the CVE records for up-to-date
> information about which Python versions are affected.

Google's security research team has also published a Proof of Concept for
CVE-2025-4517 at:

https://github.com/google/security-research/security/advisories/GHSA-hgqp-3mmf-7h8f

	-alan-

-------- Forwarded Message --------
Subject: 	[Security-announce]Multiple CVEs (1 CRITICAL, 3 HIGH, 1 MODERATE) affecting the tarfile module
Date: 	Tue, 3 Jun 2025 08:01:26 -0500
From: 	Seth Larson <seth@...hon.org>
Reply-To: 	security-sig@...hon.org
To: 	security-announce@...hon.org

There are multiple advisories (1 CRITICAL, 3 HIGH, 1 MODERATE) affecting the CPython tarfile module.

## Bypasses in tarfile extraction filtering

These three vulnerabilities are all different methods of bypassing tar extraction filtering which is a feature in Python 3.12 and later.

You are affected by this vulnerability if using the tarfile module to extract untrusted tar archives using TarFile.extractall() or TarFile.extract() using the filter= parameter with a value of "data" or "tar". See the tarfile extraction filters documentation for more information. Only Python versions 3.12 or later are affected by these vulnerabilities, earlier versions don't include the extraction filter feature.

Note that for Python 3.14 or later the default value of filter= changed from "no filtering" to `"data", so if you are relying on this new default behavior then your usage is also affected.

Note that none of these vulnerabilities significantly affect the installation of source distributions which are tar archives as source distributions already allow arbitrary code execution during the build process. However when evaluating source distributions it's important to avoid installing source distributions with suspicious links.

Note that when extracting an untrusted tar archive without extraction filtering enabled already allows for arbitrary write access outside of the extraction directory.

* CVE-2025-4517 <https://www.cve.org/CVERecord?id=CVE-2025-4517> (CRITICAL) allows arbitrary filesystem writes outside the extraction directory during extraction with filter="data".

* CVE-2025-4330 <https://www.cve.org/CVERecord?id=CVE-2025-4330> (HIGH) allows the extraction filter to be ignored, allowing symlink targets to point outside the destination directory, and the modification of some file metadata.

* CVE-2025-4138 <https://www.cve.org/CVERecord?id=CVE-2025-4138> (HIGH) allows creating arbitrary symlinks outside the extraction directory during extraction with filter="data".

* CVE-2024-12718 <https://www.cve.org/CVERecord?id=CVE-2024-12718> (MODERATE) allows modifying some file metadata (e.g. last modified) with `filter="data"` or file permissions (chmod) with `filter="tar"` of files outside the extraction directory.

## Filtered members not skipped with TarFile.errorlevel = 0

When using a `TarFile.errorlevel = 0` and extracting with a filter the documented behavior is that any filtered members would be skipped and not extracted. However the actual behavior of `TarFile.errorlevel = 0` in affected versions is that the member would still be extracted and not skipped.

* CVE-2025-4435 <https://www.cve.org/CVERecord?id=CVE-2025-4435> (HIGH)

## Mitigation

The recommended mitigation is to upgrade your Python version to a fixed version or apply the patch(es) linked within the CVEs. If you cannot patch or upgrade, rejecting all links with the parent directory segment (`".."`) prior to calling extract will mitigate the vulnerabilities below:

```
# Avoid insecure segments in link names.
for member in tar.getmembers():
if not member.islnk():
continue
if os.pardir in os.path.split(member.linkname):
raise OSError("Tarfile with insecure segment ('..') in linkname")

# Now safe to extract members with the data filter.
tar.extractall(filter="data")
```

Please see the linked CVE IDs for the latest information on affected versions.

View attachment "[Attached Message Part](https://www.openwall.com/lists/oss-security/2025/06/24/1/1)" of type "text/plain" (292 bytes)

```

[Powered by blists](https://www.openwall.com/blists/) - [more mailing lists](https://lists.openwall.net)

Please check out the [Open Source Software Security Wiki](https://oss-security.openwall.org/wiki/), which is counterpart to this [mailing list](https://oss-security.openwall.org/wiki/mailing-lists/oss-security).

Confused about [mailing lists](https://www.openwall.com/lists/) and their use? [Read about mailing lists on Wikipedia](https://en.wikipedia.org/wiki/Electronic_mailing_list) and check out these [guidelines on proper formatting of your messages](https://www.complang.tuwien.ac.at/anton/mail-news-errors.html).
