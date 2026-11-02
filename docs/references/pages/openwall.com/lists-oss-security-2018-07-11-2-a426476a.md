---
url: "https://www.openwall.com/lists/oss-security/2018/07/11/2"
title: "oss-security - polkit: CVE-2018-1116: polkitd trusting client-supplied UID allows spoofed authentication dialogs"
captured: "2026-11-02T18:58:19Z"
via: "live"
sha256: "43392cfb46340916cde145cbb191ca31ba55fcbd14be5069958711ba1e7b4015"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# oss-security - polkit: CVE-2018-1116: polkitd trusting client-supplied UID allows spoofed authentication dialogs

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.openwall.com/lists/oss-security/2018/07/11/2>

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

[[<prev]](https://www.openwall.com/lists/oss-security/2018/07/11/1) [[next>]](https://www.openwall.com/lists/oss-security/2018/07/11/3) [[day]](https://www.openwall.com/lists/oss-security/2018/07/11/) [[month]](https://www.openwall.com/lists/oss-security/2018/07/) [[year]](https://www.openwall.com/lists/oss-security/2018/) [[list]](https://www.openwall.com/lists/oss-security/)

```

Message-ID: <20180711082156.GA8145@f195.suse.de>
Date: Wed, 11 Jul 2018 10:21:56 +0200
From: Matthias Gerstner <mgerstner@...e.de>
To: oss-security@...ts.openwall.com
Subject: polkit: CVE-2018-1116: polkitd trusting client-supplied UID allows
 spoofed authentication dialogs

Hello,

during a code reviewing related to polkit
<https://www.freedesktop.org/wiki/Software/polkit/> I found a spoofed
authentication vulnerability in the implementation of the polkitd
daemon. It allows a local attacker to trigger authentication dialogs for
other users' processes. This way the attacker can obtain certain
information about the polkit rules configuration of other users, confuse
other users or DoS other users by infinitely triggering authentication
dialogs.

Basically the issue is that an attacker is able to specify
arbitrary target process UIDs when talking to polkitd via D-Bus like
this:

$ gdbus call --system --dest org.freedesktop.PolicyKit1 \
	--object-path /org/freedesktop/PolicyKit1/Authority \
	--method org.freedesktop.PolicyKit1.Authority.CheckAuthorization \
	'("unix-process", {"pid": <uint32 ${PID}>, "start-time": <uint64 0>, "uid": <${UID}>})' \
	org.freedesktop.timedate1.set-time '[]' 1 ''

Where ${PID} needs to be the process ID of the target process and ${UID}
the user ID of the calling process i.e. `id -u`.

Upstream just released version 0.115 of polkit that addresses this issue
by way of commit bc7ffad53643a9c80231fc41f5582d6a8931c32c. The issue was
introduced with a fix for CVE-2013-4288 in polkit version 0.112.

Further below you can find the upstream commit message with a more
detailed explanation of the issue and its fix. I want to thank the
upstream developers for the constructive communication and quick
handling of the issue.

Best regards

Matthias

Timeline:

2018-06-21: I discovered and analyzed the issue
2018-06-22: I reported the issue privately to upstream via
    dbus-security@...ts.freedesktop.org. In the following days upstream
    devised a patch that was discussed and reviewed on the mailing list.
    Publication has been scheduled for 2018-07-10 together with the
    release of the fixed polkit version.
2018-07-10: The upstream release was published as scheduled.

References:

- Upstream Release Notice: https://lists.freedesktop.org/archives/polkit-devel/2018-July/000583.html
- Upstream Fix: https://cgit.freedesktop.org/polkit/commit/?id=bc7ffad53643a9c80231fc41f5582d6a8931c32c
- SUSE Bug for the issue: https://bugzilla.suse.com/show_bug.cgi?id=1099031

Upstream Commit Message:

     Fix CVE-2018-1116: Trusting client-supplied UID

     As part of CVE-2013-4288, the D-Bus clients were allowed (and
     encouraged) to submit the UID of the subject of authorization checks
     to avoid races against UID changes (notably using executables
     set-UID to root).

     However, that also allowed any client to submit an arbitrary UID, and
     that could be used to bypass "can only ask about / affect the same UID"
     checks in CheckAuthorization / RegisterAuthenticationAgent /
     UnregisterAuthenticationAgent.  This allowed an attacker:

     - With CheckAuthorization, to cause the registered authentication
       agent in victim's session to pop up a dialog, or to determine whether
       the victim currently has a temporary authorization to perform an
       operation.

       (In principle, the attacker can also determine whether JavaScript
       rules allow the victim process to perform an operation; however,
       usually rules base their decisions on information determined from
       the supplied UID, so the attacker usually won't learn anything new.)

     - With RegisterAuthenticationAgent, to prevent the victim's
       authentication agent to work (for a specific victim process),
       or to learn about which operations requiring authorization
       the victim is attempting.

     To fix this, expose internal _polkit_unix_process_get_owner() /
     obsolete polkit_unix_process_get_owner() as a private
     polkit_unix_process_get_racy_uid__() (being more explicit about the
     dangers on relying on it), and use it in
     polkit_backend_session_monitor_get_user_for_subject() to return
     a boolean indicating whether the subject UID may be caller-chosen.

     Then, in the permission checks that require the subject to be
     equal to the caller, fail on caller-chosen UIDs (and continue
     through the pre-existing code paths which allow root, or root-designated
     server processes, to ask about arbitrary subjects.)

     Signed-off-by: Miloslav Trmač <mitr@...hat.com>

--
Matthias Gerstner <matthias.gerstner@...e.de>
Dipl.-Wirtsch.-Inf. (FH), Security Engineer
https://www.suse.com/security
Telefon: +49 911 740 53 290
GPG Key ID: 0x14C405C971923553

SUSE Linux GmbH
GF: Felix Imendörffer, Jane Smithard, Graham Norton
HRB 21284 (AG Nuernberg)

Download attachment "[signature.asc](https://www.openwall.com/lists/oss-security/2018/07/11/2/1)" of type "application/pgp-signature" (834 bytes)

```

[Powered by blists](https://www.openwall.com/blists/) - [more mailing lists](https://lists.openwall.net)

Please check out the [Open Source Software Security Wiki](https://oss-security.openwall.org/wiki/), which is counterpart to this [mailing list](https://oss-security.openwall.org/wiki/mailing-lists/oss-security).

Confused about [mailing lists](https://www.openwall.com/lists/) and their use? [Read about mailing lists on Wikipedia](https://en.wikipedia.org/wiki/Electronic_mailing_list) and check out these [guidelines on proper formatting of your messages](https://www.complang.tuwien.ac.at/anton/mail-news-errors.html).
