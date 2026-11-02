---
url: "https://www.usenix.org/conference/13th-usenix-security-symposium/design-and-implementation-tcg-based-integrity-measurement"
title: "Design and Implementation of a TCG-based Integrity Measurement Architecture | USENIX"
captured: "2026-11-02T19:01:23Z"
via: "live"
sha256: "82360ad8c3a8099279e5353e6d65bcbe9d39b96e7a1af7f7c0cbe748615a8858"
wayback: "https://web.archive.org/web/20221228183807/https://www.usenix.org/conference/13th-usenix-security-symposium/design-and-implementation-tcg-based-integrity-measurement"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Design and Implementation of a TCG-based Integrity Measurement Architecture | USENIX

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.usenix.org/conference/13th-usenix-security-symposium/design-and-implementation-tcg-based-integrity-measurement>
> Snapshot: <https://web.archive.org/web/20221228183807/https://www.usenix.org/conference/13th-usenix-security-symposium/design-and-implementation-tcg-based-integrity-measurement>

[Skip to main content](https://www.usenix.org/conference/13th-usenix-security-symposium/design-and-implementation-tcg-based-integrity-measurement#main-content)

[Back to USENIX](https://www.usenix.org/)

https://www.usenix.org/conference/13thusenixsecuritysymposium

##  USENIX Conference Policies

- [Event Code of Conduct](https://www.usenix.org/conferences/coc)
- [Conference Network Policy](https://www.usenix.org/conferences/network-policy)
- [Statement on Environmental Responsibility Policy](https://www.usenix.org/conferences/environmental-responsibility)

#  Design and Implementation of a TCG-based Integrity Measurement Architecture

We present the design and implementation of a secure integrity measurement system for Linux. All executable content that is loaded onto the Linux system is measured before execution and these measurements are protected by the Trusted Platform Module (TPM) that is part of the Trusted Computing Group (TCG) standards. Our system is the first to extend the TCG trust measurement concepts to dynamic executable content from the BIOS all the way up into the application layer. In effect, we show that many of the Microsoft NGSCB guarantees can be obtained on today's hardware and today's software and that these guarantees do not require a new CPU mode or operating system but merely depend on the availability of an independent trusted entity, a TPM for example. We apply our trust measurement architecture to a web server application where we show how our system can detect undesirable invocations, such as rootkit programs, and that our measurement architecture is practical in terms of the number of measurements taken and the performance impact of making them.

## [Reiner Sailer, IBM T. J. Watson Research Center](https://www.usenix.org/conference/13th-usenix-security-symposium/speaker-or-organizer/reiner-sailer-ibm-t-j-watson-research)

## [Xiaolan Zhang, IBM T. J. Watson Research Center](https://www.usenix.org/conference/13th-usenix-security-symposium/speaker-or-organizer/xiaolan-zhang-ibm-t-j-watson-research)

## [Trent Jaeger, IBM T.J. Watson Research Center](https://www.usenix.org/conference/13th-usenix-security-symposium/speaker-or-organizer/trent-jaeger-ibm-tj-watson-research)

## [Leendert van Doorn, IBM T. J. Watson Research Center](https://www.usenix.org/conference/13th-usenix-security-symposium/speaker-or-organizer/leendert-van-doorn-ibm-t-j-watson)

BibTeX

@inproceedings {269600,
author = {Reiner Sailer and Xiaolan Zhang and Trent Jaeger and Leendert van Doorn},
title = {Design and Implementation of a {TCG-based} Integrity Measurement Architecture},
booktitle = {13th USENIX Security Symposium (USENIX Security 04)},
year = {2004},
address = {San Diego, CA},
url = {https://www.usenix.org/conference/13th-usenix-security-symposium/design-and-implementation-tcg-based-integrity-measurement},
publisher = {USENIX Association},
month = aug
}

[Download](https://www.usenix.org/biblio/export/bibtex/269600)

### Links

Paper:

http://usenix.org/publications/library/proceedings/sec04/tech/full_papers/sailer/sailer.pdf

Paper (HTML):

http://usenix.org/publications/library/proceedings/sec04/tech/full_papers/sailer/sailer_html/index.html

- [Log in](https://www.usenix.org/user/login?destination=comment/reply/151383%23comment-form) or [register](https://www.usenix.org/user/register?destination=comment/reply/151383%23comment-form) to post comments

© USENIX
EIN 13-3055038

- [Privacy Policy](https://www.usenix.org/privacy-policy)
- [Contact Us](https://www.usenix.org/contact)
