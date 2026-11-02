---
url: "https://www.yubico.com/blog/github-now-supports-ssh-security-keys/"
title: "GitHub now supports SSH security keys"
site: "Yubico"
published: "2021-05-10T16:54:20+00:00"
captured: "2026-11-02T19:03:01Z"
via: "live"
sha256: "c90ba6a869fdf38d41d749f11293cdc67c6c95fa1ab462df205dc525d3f223ec"
wayback: "https://web.archive.org/web/20260419203843/https://www.yubico.com/blog/github-now-supports-ssh-security-keys/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# GitHub now supports SSH security keys

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.yubico.com/blog/github-now-supports-ssh-security-keys/>
> Snapshot: <https://web.archive.org/web/20260419203843/https://www.yubico.com/blog/github-now-supports-ssh-security-keys/>

# GitHub now supports SSH security keys

Emil Lundberg

Emil Lundberg

May 10, 2021

https://www.facebook.com/share.php?u=https://www.yubico.com/blog/github-now-supports-ssh-security-keyshttps://twitter.com/intent/tweet?text=GitHub now supports SSH security keys&url=https://www.yubico.com/blog/github-now-supports-ssh-security-keyshttps://www.linkedin.com/sharing/share-offsite/?url=https://www.yubico.com/blog/github-now-supports-ssh-security-keys

Today, GitHub [has announced](https://github.blog/2021-05-10-security-keys-supported-ssh-git-operations/) support for using U2F and FIDO2 security keys for SSH, and we’re honored to have been an early collaborator in working with GitHub on developing this feature. This makes it easier than ever to use YubiKeys to secure all your GitHub access, making your SSH keys much more secure while maintaining a great user experience.

While it has long been possible to [use the YubiKey for SSH](https://developers.yubico.com/PIV/Guides/Securing_SSH_with_OpenPGP_or_PIV.html) via the OpenPGP or PIV features, the direct support in SSH is easier to set up, more portable, and works with any U2F or FIDO2 security key – even older ones like the [FIDO U2F Security Key by Yubico](https://support.yubico.com/hc/en-us/articles/360013656800-FIDO-U2F-Security-Key). Let’s dive in!

## Getting started

To get started you’ll need [OpenSSH](https://www.openssh.com/) version 8.2 or later, and you’ll also need [libfido2](https://github.com/Yubico/libfido2) installed. Windows users may need to use [Cygwin](https://cygwin.com/packages/summary/openssh.html) for this.

First you’ll need to generate a key pair. Plug in your security key and run the command:

$ ssh-keygen -t ecdsa-sk

The option -t ecdsa-sk instructs OpenSSH to create an ECDSA key on a FIDO security key instead of a traditional private key file. You can also use -t ed25519-sk to create an EdDSA key instead, but this is not supported by all security keys.

This will create two files in your SSH directory. The first is id_ecdsa_sk.pub, which is a normal OpenSSH public key file whose contents you’ll need to paste into the [new SSH key form](https://github.com/settings/ssh/new) on GitHub. The second is id_ecdsa_sk which would usually contain the corresponding private key, but in this case it instead contains a “[key handle](https://www.yubico.com/blog/yubicos-u2f-key-wrapping/)” that references the security key. You’ll need to copy the id_ecdsa_sk file to each computer where you want to use this SSH key. Or, if your security key supports it, you can use a FIDO2 resident key.

## Using resident keys

If your security key supports FIDO2 resident keys*, like the [YubiKey 5 Series](https://www.yubico.com/product/yubikey-5-nfc/), [YubiKey 5 FIPS Series](https://www.yubico.com/products/yubikey-fips/), or the [Security Key NFC by Yubico](https://www.yubico.com/product/security-key-nfc-by-yubico-black/), you can enable this when creating your SSH key:

$ ssh-keygen -t ecdsa-sk -O resident

This works the same as before, except a resident key is easier to import to a new computer because it can be loaded directly from the security key. To use the SSH key on a new computer, make sure you have ssh-agent running and simply run:

$ ssh-add -K

This will load a “key handle” into the SSH agent and make the key available for use on the new computer. This works great for short visits, but it won’t last forever – you’ll need to run ssh-add again if you reboot the computer, for example. To import the key permanently, instead run:

$ ssh-keygen -K

This will write two files into the current directory: id_ecdsa_sk_rk and id_ecdsa_sk_rk.pub. Now you just need to rename the private key file to id_ecdsa_sk and move it into your SSH directory:

$ mv id_ecdsa_sk_rk ~/.ssh/id_ecdsa_sk

Finally, there’s one more feature to be excited about…

## Passwordless MFA

Passwordless multi-factor authentication is one of the greatest benefits of FIDO security keys, and it is now available for SSH too! If your security key supports FIDO2 user verification, like the [YubiKey 5 Series](https://www.yubico.com/products/yubikey-5-overview/), [YubiKey 5 FIPS Series](https://www.yubico.com/products/yubikey-fips/), or the [Security Key NFC by Yubico](https://www.yubico.com/product/security-key-nfc-by-yubico-black/), you can enable it when creating your SSH key:

$ ssh-keygen -t ecdsa-sk -O verify-required

This will configure the security key to require a PIN or other user authentication whenever you use this SSH key. Your SSH access is now protected with passwordless multi-factor authentication!

Yubico’s mission is to make the internet safer for everyone, and we are thrilled to have both GitHub and the OpenSSH project showing that you don’t have to choose between great security and ease of use. The technologies are still young, but we hope to see more SSH services offer these capabilities in the future.

For additional details, please read today’s [blog from GitHub](https://github.blog/2021-05-10-security-keys-supported-ssh-git-operations/) on this new functionality!

* “Resident keys” have been renamed to “discoverable credentials” in the WebAuthn and CTAP standards, but OpenSSH still uses the “resident key” terminology.

[GitHub](https://www.yubico.com/blog/tag/github/), [SSH](https://www.yubico.com/blog/tag/ssh/)

[Talk to our team](https://www.yubico.com/contact-us/)

Share this article:

https://www.facebook.com/share.php?u=https://www.yubico.com/blog/github-now-supports-ssh-security-keyshttps://twitter.com/intent/tweet?text=GitHub now supports SSH security keys&url=https://www.yubico.com/blog/github-now-supports-ssh-security-keyshttps://www.linkedin.com/sharing/share-offsite/?url=https://www.yubico.com/blog/github-now-supports-ssh-security-keys

##  Recommended Posts

-  [Secure it Forward Spotlight: Where sustainability meets security   The greenest product may be the one you don’t have to replace. This World Sustainability Day, we’re highlighting how that idea shapes Secure it Forward, Yubico’s social impact initiative. By building YubiKeys to last and giving keys with minor cosmetic imperfections a second purpose, we’re making thoughtful use of the products we create while expanding […]    Read more](https://www.yubico.com/blog/secure-it-forward-sustainability/)
-  [Imagine an internet that protects us, even when we make mistakes   In my 19 years in the cybersecurity industry, I have used Cybersecurity Awareness Month in October to sound the alarm about basic digital hygiene and urge internet users to use stronger authentication methods than passwords, avoid clicking on suspicious links, and keep their software updated. This year, I am taking a different approach. With the […]    Read more](https://www.yubico.com/blog/ten-times-trust-safer-internet/)
-  [Underwriting the AI threat: Why cyber insurance must evolve to phishing-resistant MFA   Agentic AI has fundamentally rewritten the rules of phishing and social engineering attacks. Where cybercriminals once spent upwards of 16 hours crafting a single targeted email, AI tools now allow them to deploy hyper-personalized, grammatically correct phishing campaigns at scale in just five minutes. The impact on the cyber insurance market is already alarming: phishing […]    Read more](https://www.yubico.com/blog/cyber-insurance-phishing-resistant-mfa/)
