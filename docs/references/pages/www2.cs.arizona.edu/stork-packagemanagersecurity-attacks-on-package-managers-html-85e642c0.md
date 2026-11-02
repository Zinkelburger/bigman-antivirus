---
url: "https://www2.cs.arizona.edu/stork/packagemanagersecurity/attacks-on-package-managers.html"
title: "Attacks on Package Managers"
captured: "2026-11-02T19:03:31Z"
via: "live"
sha256: "397c6435f50c19b861ec7bd4d7425635d15698164f10f355249d604d2404a476"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# Attacks on Package Managers

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www2.cs.arizona.edu/stork/packagemanagersecurity/attacks-on-package-managers.html>

[Overview](https://www2.cs.arizona.edu/stork/packagemanagersecurity/attacks-on-package-managers.html#overview) | [Attack Anatomy](https://www2.cs.arizona.edu/stork/packagemanagersecurity/attacks-on-package-managers.html#explanation) | [Impact](https://www2.cs.arizona.edu/stork/packagemanagersecurity/attacks-on-package-managers.html#howtolaunchattack) | [Protecting Yourself](https://www2.cs.arizona.edu/stork/packagemanagersecurity/attacks-on-package-managers.html#protectingyourself) | [People](https://www2.cs.arizona.edu/stork/packagemanagersecurity/attacks-on-package-managers.html#people) | [Other Attacks](https://www2.cs.arizona.edu/stork/packagemanagersecurity/otherattacks.html) | [FAQ](https://www2.cs.arizona.edu/stork/packagemanagersecurity/faq.html) | [Papers](https://www2.cs.arizona.edu/stork/packagemanagersecurity/papers.html) | [Acknowledgments](https://www2.cs.arizona.edu/stork/packagemanagersecurity/attacks-on-package-managers.html#acknowledgements)

## Overview

Security vulnerabilities are often the result of software bugs. It is important to keep software up-to-date, as malicious parties often can exploit bugs in outdated software. Package managers were created to automate the process of package update and installation, however, if the package manager is not secure, it may represent another avenue of attack!

Package managers are normally run with unrestricted access in order to allow them to modify critical system software. The package manager's actions, therefore, affect the entire system and make the security of package managers vital.

Given their critical role, the expectation would be for package managers to be extremely secure. We examined ten popular package managers (APT, YUM, YaST, etc.) for Linux and BSD systems and found vulnerabilities in all of them. The attacks discussed here can give attackers the ability to read or erase any file on your computer, capture passwords, set up a backdoor into your system, etc. without needing to compromise any keys or passwords.

## Attack Anatomy: Replay Attacks

To provide an example of the sorts of attacks an attacker can launch on package managers, this page describes an example attack called a [replay attack](http://en.wikipedia.org/wiki/Replay_attack). [Other attacks](https://www2.cs.arizona.edu/stork/packagemanagersecurity/otherattacks.html) are described on a separate page.

### The First Piece: Metadata Replay

Packages are essentially trusted forever once they or repository metadata describing them has been signed. This means that even after a package has a vulnerability discovered in it, clients will continue to be willing to install that insecure package. An attacker can replay the correctly signed packages or metadata from a previous release and your computer will install software with flaws that the attacker can exploit.

The default behavior of package managers is to install the most recent version of a requested package that is available from the distribution. Keeping a client safe would seem to be just a matter of retrieving the latest repository metadata provided by the distribution on a regular basis.

### The Second Piece: Mirror Control

Unfortunately, replay attacks on package managers are still very possible even if a client frequently requests updated repository metadata. The problem is that most distributions use a network of publicly provided servers to host their mirrors. In most cases, the people and organizations running these mirrors are only superficially examined before they are listed as an official public mirror. Once listed as a public mirror, a distribution will begin directing requests to the mirror.

To give an example of how easy it is for a malicious party to obtain a mirror, we ran an experiment where we created a fake administrator and company name and leased a server from a hosting provider. We were able to get our mirror listed on every distribution we tried (Ubuntu, Fedora, OpenSuSE, CentOS, and Debian) and our mirrors were contacted by thousands of clients, even including military and government computers!

Since there is so little verification of who owns and runs a mirror, there is no guarantee that the mirror is benevolent. They may, in fact, be talking to an attacker who has setup a mirror and become listed as an official mirror for the distribution.

Some distributions do simple checking that the mirrors respond with updated and valid content. However, such checks only serve to discover benevolent mirrors that have fallen behind. They are unable to detect a maliciously operated mirror. Attackers can respond to the distribution's queries with the expected answer while at the same time targeting the public at large or individual clients with different responses. These targeted responses can include old signed files or manipulated unsigned files.

## Impact: What Can an Attacker Do With a Replay Attack?

### Vulnerable Package Installation

When a user installs a package, their package manager retrieves from a repository mirror the list of available packages. A malicious mirror can respond to the client with an outdated signed list of packages or an arbitrary list of outdated packages, depending on characteristics of the package manager (described in the [FAQ](https://www2.cs.arizona.edu/stork/packagemanagersecurity/faq.html)). The client only knows that files it receives are properly signed by their distribution. They do not know that what they have received is an old list of packages.

Due to the way existing package managers are currently implemented, it is even possible to provide a client an older list of files than a list they have already seen. That is, if a client already obtained an updated, signed list of packages yesterday but they are talking to a malicious mirror today, the malicious mirror can provide the client a signed list of packages from the year before. The malicious mirror needs only to have a copy of signed files that were made available by the distribution. They do not need to have compromised the signing key.

### Preventing Security Updates

A malicious mirror doesn't have to provide a client outdated packages with vulnerabilities known to the attacker in order to leave the client vulnerable. All that an attacker needs to do is to continuously serve the client the same repository metadata. Over time, even though vulnerabilities become known in various packages and new versions become available, the client will never see them when updating.

For example, it is known that an earlier version of OpenSSL for Debian has a security flaw. The list of files from the repository that previously included this package is still correctly signed. Using this old signed file list, a malicious mirror can keep a client on the insecure version of OpenSSL by responding to the client's package manager with the old list of files.

## Protecting Yourself

### Things You Can Do Today:

- Use repositories you trust. Use only mirrors that belong to reputable organizations. Don't randomly choose mirrors, even from official lists. The official lists of public repositories often contain many superficially verified mirrors.
- Manually update your systems (and local mirror caches). Know when package updates become available and what the versions should be. Manually verify and install the updated packages (or add them to your local mirror cache that your systems update from) rather than relying on automated updates. We have observed mirrors many months out of date for some distributions, so you should check periodically that your mirror is being updated.
- Use signed repository metadata. If your package manager or distribution does not yet support signed metadata but only signed packages, at least require signed packages until signed metadata is supported.
- Use HTTPS for mirror communication. Unfortunately, this is generally only available with paid support services (and only protects you against man-in-the-middle attacks, not malicious mirrors). However, by running a distribution with HTTPS support on their mirrors, a man-in-the-middle attacker cannot easily launch an attack as though it were a mirror.

### In the future:

- Use package managers that sign repository metadata. Unsigned repository metadata gives malicious parties [more leverage in their attacks](https://www2.cs.arizona.edu/stork/packagemanagersecurity/otherattacks.html).
- Use package managers that implement metadata expiration. If there is no way in a package manager for metadata to ever expire, replay attacks will be able to go unnoticed.
- Use distributions that properly make use of the package manager's security features. If a distribution doesn't sign repository metadata or expire these signed files even though the package manager supports doing so, it doesn't help you stay secure.

## People

[Justin Cappos](http://www.cs.arizona.edu/people/justin/), [Justin Samuel](http://www.cs.arizona.edu/people/jsamuel/), [Scott Baker](http://www.smbaker.com/), [John H. Hartman](http://www.cs.arizona.edu/people/jhh/).

Contact us at:  or if you prefer to use GPG, contact  using the GPG key with fingerprint: 6022 7DB9 D963 56AA 0359 CB5C 7EF6 B11F ABFB 2A4A and  using the GPG key with fingerprint: 66EF 84E2 F184 B140 712B 55A7 2B96 AB8F DDF1 F3EE

## Acknowledgments

We would like to thank Ryan Giobbi and the CERT Coordination Center for contacting many of the major package managers and distributions. We appreciate James Hamilton of [Offshoot Inc.](http://www.offshootinc.com) creating the logo at the top of this page. We also appreciate the comments given to us by Ryan Giobbi, Igor Crk, Michael Piatek, Angelos Stavrou, Chris Gniady, Yoshi Kohno, and Wenjun Hu.

######  This web page was made public on 10 July 2008.
