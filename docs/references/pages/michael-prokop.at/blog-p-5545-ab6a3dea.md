---
url: "https://michael-prokop.at/blog/?p=5545"
title: "mikas blog » Blog Archive » How to properly use 3rd party Debian repository signing keys with apt"
captured: "2026-11-02T18:36:11Z"
via: "live"
source_url: "https://michael-prokop.at/blog/2021/02/16/how-to-properly-use-3rd-party-debian-repository-signing-keys-with-apt/"
sha256: "74b5618e67df6830cfe45301328874dfca320098f1f09cb9b993f9f8e549754a"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# mikas blog » Blog Archive » How to properly use 3rd party Debian repository signing keys with apt

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://michael-prokop.at/blog/?p=5545>

# [mikas blog](https://michael-prokop.at/blog)

… and even if no one reads it

Don't understand german? [Read](https://michael-prokop.at/blog/category/english/) or subscribe to my [english-only feed](https://michael-prokop.at/blog/category/english/feed/).

« [Revisiting 2020](https://michael-prokop.at/blog/2021/01/16/revisiting-2020/)

[Bookdump 01/2021](https://michael-prokop.at/blog/2021/04/02/bookdump-01-2021/) »

## [How to properly use 3rd party Debian repository signing keys with apt](https://michael-prokop.at/blog/2021/02/16/how-to-properly-use-3rd-party-debian-repository-signing-keys-with-apt/)

(Blogging this, since this is a recurring anti-pattern I noticed at several customers and often comes up during deployments of 3rd party repositories.)

Update on 2021-02-19: clarified, that Signed-By requires apt >= 1.1, thanks Vincent Bernat

Many upstream projects provide Debian repository instructions like this:

```

curl -fsSL https://example.com/stable/debian.gpg | sudo apt-key add -

```

Do not follow this, for different reasons, including:

- You do not see what you get before adding the GPG key to your global apt trust store
- You can’t easily script this via your preferred configuration management (the [apt-key manpage](https://manpages.debian.org/stretch/apt/apt-key.8.en.html) clearly discourages programmatic usage)
- The signing key is considered valid for all your enabled Debian repositories (instead of only a specific one)
- You need GnuPG (either gnupg2 or gnupg1) on your system for usage with apt-key

There’s a much better approach to this: download the GPG key, make sure it’s in the appropriate format, then use it via `deb [signed-by=/usr/share/keyrings/ ]` in your apt’s sources list configuration. Note and FTR: the Signed-By feature is available starting with apt 1.1 (so apt in Debian jessie/8 and older does not support it).

TL;DR:

- Install GPG keys in ascii-armored / old public key block format as /usr/share/keyrings/example.asc and use `deb [signed-by=/usr/share/keyrings/example.asc] https://example.com/ ` in apt’s sources.list configuration
- Install GPG keys in binary OpenPGP format as /usr/share/keyrings/example.gpg and use `deb [signed-by=/usr/share/keyrings/example.gpg] https://example.com/…` in apt’s sources.list configuration

As an example, let’s demonstrate this with the [Tailscale Debian repository for buster](https://www.tailscale.com/download/linux/debian-buster).
Downloading the GPG file will give you an ascii-armored GPG file:

```

% curl -fsSL -o buster.gpg https://pkgs.tailscale.com/stable/debian/buster.gpg
% gpg --keyid-format long buster.gpg
gpg: WARNING: no command supplied.  Trying to guess what you mean ...
pub   rsa4096/458CA832957F5868 2020-02-25 [SC]
      2596A99EAAB33821893C0A79458CA832957F5868
uid                           Tailscale Inc. (Package repository signing key) <info@tailscale.com>
sub   rsa4096/B1547A3DDAAF03C6 2020-02-25 [E]
% file buster.gpg
buster.gpg: PGP public key block Public-Key (old)

```

If you have apt version >= 1.4 available (Debian >=stretch/9 and Ubuntu >=bionic/18.04), you can use this file directly as follows:

```

% sudo mv buster.gpg /usr/share/keyrings/tailscale.asc
% cat /etc/apt/sources.list.d/tailscale.list
deb [signed-by=/usr/share/keyrings/tailscale.asc] https://pkgs.tailscale.com/stable/debian buster main
% sudo apt update
[...]

```

And you’re done!

Iff your apt version really is older than 1.4, you need to convert the ascii-armored GPG file into a GPG key public ring file (AKA binary OpenPGP format), either by just dearmor-ing it (if you don’t care about checking ID + fingerprint):

```

% gpg --dearmor < buster.gpg > tailscale.gpg

```

… or if you prefer to go via GPG, you can also use a temporary GPG home directory (if you don’t care about going through your personal GPG setup):

```

% mkdir --mode=700 /tmp/gpg-tmpdir
% gpg --homedir /tmp/gpg-tmpdir --import ./buster.gpg
gpg: keybox '/tmp/gpg-tmpdir/pubring.kbx' created
gpg: /tmp/gpg-tmpdir/trustdb.gpg: trustdb created
gpg: key 458CA832957F5868: public key "Tailscale Inc. (Package repository signing key) <info@tailscale.com>" imported
gpg: Total number processed: 1
gpg:               imported: 1
% gpg --homedir /tmp/gpg-tmpdir --output tailscale.gpg  --export-options=export-minimal --export 0x458CA832957F5868
% rm -rf /tmp/gpg-tmpdir

```

The resulting GPG key public ring file should look like that:

```

% file tailscale.gpg
tailscale.gpg: PGP/GPG key public ring (v4) created Tue Feb 25 04:51:20 2020 RSA (Encrypt or Sign) 4096 bits MPI=0xc00399b10bc12858...
% gpg tailscale.gpg
gpg: WARNING: no command supplied.  Trying to guess what you mean ...
pub   rsa4096/458CA832957F5868 2020-02-25 [SC]
      2596A99EAAB33821893C0A79458CA832957F5868
uid                           Tailscale Inc. (Package repository signing key) <info@tailscale.com>
sub   rsa4096/B1547A3DDAAF03C6 2020-02-25 [E]

```

Then you can use this GPG file on your system as follows:

```

% sudo mv tailscale.gpg /usr/share/keyrings/tailscale.gpg
% cat /etc/apt/sources.list.d/tailscale.list
deb [signed-by=/usr/share/keyrings/tailscale.gpg] https://pkgs.tailscale.com/stable/debian buster main
% sudo apt update
[...]

```

Such a setup ensures:

- You can verify the GPG key file (ID + fingerprint)
- You can easily ship files via /usr/share/keyrings/ and refer to it in your deployment scripts, configuration management, (and can also easily update or get rid of them again!)
- The GPG key is valid only for the repositories with the corresponding `[signed-by=/usr/share/keyrings/ ]` entry
- You don’t need to install GnuPG (neither gnupg2 nor gnupg1) on the system which is using the 3rd party Debian repository

Thanks: Guillem Jover for reviewing an early draft of this blog article.

This entry was posted by [mika](https://michael-prokop.at/) on Tuesday, February 16th, 2021 at 11:19 and is filed under [Computer](https://michael-prokop.at/blog/category/computer/), [Debian](https://michael-prokop.at/blog/category/debian/), [English](https://michael-prokop.at/blog/category/english/), [Open Source](https://michael-prokop.at/blog/category/open-source/). You can follow any responses to this entry through the [RSS 2.0](https://michael-prokop.at/blog/2021/02/16/how-to-properly-use-3rd-party-debian-repository-signing-keys-with-apt/feed/) feed. Both comments and pings are currently closed.

### 4 Responses to “How to properly use 3rd party Debian repository signing keys with apt”

-  [anarcat](https://anarc.at/) Says:
[February 16th, 2021 at 17:33](https://michael-prokop.at/blog/2021/02/16/how-to-properly-use-3rd-party-debian-repository-signing-keys-with-apt/#comment-17107)

I wrote a “best practices” tutorial for this here a while back:

https://wiki.debian.org/DebianRepository/UseThirdParty

-  [mika](http://www.michael-prokop.at/) Says:
[February 16th, 2021 at 17:36](https://michael-prokop.at/blog/2021/02/16/how-to-properly-use-3rd-party-debian-repository-signing-keys-with-apt/#comment-17108)

@anarcat: yeah right, and you also drove our keyring setup in Grml project, thanks for that! We should update the Wiki though, e.g. the “The reason why we avoid ASCII-armored files is that they cannot be used directly by SecureApt.” is no longer true

-  [Vincent Bernat](https://vincent.bernat.im/en) Says:
[February 18th, 2021 at 09:12](https://michael-prokop.at/blog/2021/02/16/how-to-properly-use-3rd-party-debian-repository-signing-keys-with-apt/#comment-17119)

I am still holding back on doing this since apt in Jessie eLTS does not support Signed-By. You may mention the minimum version to get this feature in apt is 1.1.

-  [mika](http://www.michael-prokop.at/) Says:
[February 19th, 2021 at 08:47](https://michael-prokop.at/blog/2021/02/16/how-to-properly-use-3rd-party-debian-repository-signing-keys-with-apt/#comment-17126)

@Vincent: I even checked it in apt source when working on the blog post but somehow decided to not mention it to keep it as short as poossible, but thanks to your comment thought about it again and it’s definitely worth mentioning this. :) Thanks, updated!

[Impressum](https://michael-prokop.at/impressum/) / [Entries (RSS)](https://michael-prokop.at/blog/feed/) / [Comments (RSS)](https://michael-prokop.at/blog/comments/feed/).
