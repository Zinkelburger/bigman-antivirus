---
url: "https://www.usenix.org/conference/nsdi16/technical-sessions/presentation/kuppusamy"
title: "Diplomat: Using Delegations to Protect Community Repositories | USENIX"
captured: "2026-11-02T19:02:18Z"
via: "live"
sha256: "eb31ff936b007dd15e9aaf1139b4a92f1b45f71e92b726cd8668666ad193245c"
wayback: "https://web.archive.org/web/20250920135517/https://www.usenix.org/conference/nsdi16/technical-sessions/presentation/kuppusamy"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# Diplomat: Using Delegations to Protect Community Repositories | USENIX

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.usenix.org/conference/nsdi16/technical-sessions/presentation/kuppusamy>
> Snapshot: <https://web.archive.org/web/20250920135517/https://www.usenix.org/conference/nsdi16/technical-sessions/presentation/kuppusamy>

[Skip to main content](https://www.usenix.org/conference/nsdi16/technical-sessions/presentation/kuppusamy#main-content)

[Back to USENIX](https://www.usenix.org/)

https://www.usenix.org/conference/nsdi16

- [Home](https://www.usenix.org/conference/nsdi16)
- [Attend](https://www.usenix.org/conference/nsdi16/venue-hotel-and-travel)

- [Registration Information](https://www.usenix.org/conference/nsdi16/registration-information)
- [Registration Discounts](https://www.usenix.org/conference/nsdi16/registration-discounts)
- [Venue, Hotel, and Travel](https://www.usenix.org/conference/nsdi16/venue-hotel-and-travel)
- [Students and Grants](https://www.usenix.org/conference/nsdi16/students-and-grants)

- [Activities](https://www.usenix.org/conference/nsdi16/activities)

- [Birds-of-a-Feather Sessions](https://www.usenix.org/conference/nsdi16/birds-feather-sessions)
- [Poster Session and Happy Hour](https://www.usenix.org/conference/nsdi16/poster-session)

- [Program](https://www.usenix.org/conference/nsdi16/technical-sessions)

- [At a Glance](https://www.usenix.org/conference/nsdi16/glance)
- [Technical Sessions](https://www.usenix.org/conference/nsdi16/technical-sessions)

- [Sponsorship](https://www.usenix.org/conference/nsdi16/sponsorship)
- [Participate](https://www.usenix.org/conference/nsdi16/call-for-papers)

- [Instructions for Participants](https://www.usenix.org/conference/nsdi16/instructions-participants)
- [Call for Papers](https://www.usenix.org/conference/nsdi16/call-for-papers)
- [Call for Posters](https://www.usenix.org/conference/nsdi16/call-posters)

- [About](https://www.usenix.org/conference/nsdi16/symposium-organizers)

- [Organizers](https://www.usenix.org/conference/nsdi16/symposium-organizers)
- [Help Promote](https://www.usenix.org/conference/nsdi16/help-promote-nsdi-16)
- [Questions](https://www.usenix.org/conference/nsdi16/questions)
- [Past Symposia](https://www.usenix.org/conferences/byname/178)

- [Home](https://www.usenix.org/conference/nsdi16)
- [Attend](https://www.usenix.org/conference/nsdi16/venue-hotel-and-travel)
- [Activities](https://www.usenix.org/conference/nsdi16/activities)
- [Program](https://www.usenix.org/conference/nsdi16/technical-sessions)
- [Sponsorship](https://www.usenix.org/conference/nsdi16/sponsorship)
- [Participate](https://www.usenix.org/conference/nsdi16/call-for-papers)
- [About](https://www.usenix.org/conference/nsdi16/symposium-organizers)

##  sponsors

Silver Sponsor

https://www.google.com

Silver Sponsor

https://research.facebook.com

Silver Sponsor

http://www.microsoft.com

Silver Sponsor

http://www.vmware.com

Bronze Sponsor

http://research.cisco.com

Media Sponsor

http://queue.acm.org

Media Sponsor

http://www.userfriendly.org

Media Sponsor

http://www.admin-magazine.com

Media Sponsor

http://www.raspberry-pi-geek.com/

Media Sponsor

http://lxer.com

Media Sponsor

http://www.nostarch.com

Media Sponsor

http://www.crcpress.com/

Media Sponsor

http://www.linuxpromagazine.com

Industry Partner

http://www.dmtf.org

##  help promote

Get more
[Help Promote graphics!](https://www.usenix.org/conference/nsdi16/help-promote-nsdi-16)

##  USENIX Conference Policies

- [Event Code of Conduct](https://www.usenix.org/conferences/coc)
- [Conference Network Policy](https://www.usenix.org/conferences/network-policy)
- [Statement on Environmental Responsibility Policy](https://www.usenix.org/conferences/environmental-responsibility)

#  Diplomat: Using Delegations to Protect Community Repositories

Trishank Karthik Kuppusamy, Santiago Torres-Arias, Vladimir Diaz, and Justin Cappos, New York University

Community repositories, such as Docker Hub, PyPI, and RubyGems, are bustling marketplaces that distribute software. Even though these repositories use common software signing techniques (e.g., GPG and TLS), attackers can still publish malicious packages after a server compromise. This is mainly because a community repository must have immediate access to signing keys in order to certify the large number of new projects that are registered each day.

This work demonstrates that community repositories can offer compromise-resilience and real-time project registration by employing mechanisms that disambiguate trust delegations. This is done through two delegation mechanisms that provide flexibility in the amount of trust assigned to different keys. Using this idea we implement Diplomat, a software update framework that supports security models with different security / usability tradeoffs. By leveraging Diplomat, a community repository can achieve near-perfect compromise-resilience while allowing real-time project registration. For example, when Diplomat is deployed and configured to maximize security on Python's community repository, less than 1% of users will be at risk even if an attacker controls the repository and is undetected for a month. Diplomat is being integrated by Ruby, CoreOS, Haskell, OCaml, and Python, and has already been deployed by Flynn, LEAP, and Docker.

## [Trishank Karthik Kuppusamy, New York University](https://www.usenix.org/conference/nsdi16/speaker-or-organizer/trishank-karthik-kuppusamy-new-york-university)

## [Santiago Torres-Arias, New York University](https://www.usenix.org/conference/nsdi16/speaker-or-organizer/santiago-torres-arias-new-york-university)

## [Vladimir Diaz, New York University](https://www.usenix.org/conference/nsdi16/speaker-or-organizer/vladimir-diaz-new-york-university)

## [Justin Cappos, New York University](https://www.usenix.org/conference/nsdi16/speaker-or-organizer/justin-cappos-new-york-university)

## Open Access Media

USENIX is committed to Open Access to the research presented at our events. Papers and proceedings are freely available to everyone once the event begins. Any video, audio, and/or slides that are posted after the event are also free and open to everyone. [Support USENIX](https://www.usenix.org/annual-fund) and our commitment to Open Access.

BibTeX

@inproceedings {194972,
author = {Trishank Karthik Kuppusamy and Santiago Torres-Arias and Vladimir Diaz and Justin Cappos},
title = {Diplomat: Using Delegations to Protect Community Repositories},
booktitle = {13th USENIX Symposium on Networked Systems Design and Implementation (NSDI 16)},
year = {2016},
isbn = {978-1-931971-29-4},
address = {Santa Clara, CA},
pages = {567--581},
url = {https://www.usenix.org/conference/nsdi16/technical-sessions/presentation/kuppusamy},
publisher = {USENIX Association},
month = mar
}

[Download](https://www.usenix.org/biblio/export/bibtex/194972)

[Kuppusamy PDF](https://www.usenix.org/system/files/conference/nsdi16/nsdi16-paper-kuppusamy.pdf)

[View the slides](https://www.usenix.org/sites/default/files/conference/protected-files/nsdi16_slides_kuppusamy.pdf)

#### Presentation Audio

[Download Audio](http://0b4af6cdc2f0c5998459-c0245c5c937c5dedcca3f1764ecc9b2f.r43.cf2.rackcdn.com/nsdi16/kuppusamy.mp3)

- [Log in](https://www.usenix.org/user/login?destination=comment/reply/194973%23comment-form) or [register](https://www.usenix.org/user/register?destination=comment/reply/194973%23comment-form) to post comments

##  Silver Sponsors

https://research.facebook.com

https://www.google.com

http://www.microsoft.com

http://www.vmware.com

##  Bronze Sponsors

http://research.cisco.com http://research.cisco.com

##  Media Sponsors & Industry Partners

http://queue.acm.org http://queue.acm.org

http://www.admin-magazine.com http://www.admin-magazine.com

http://www.crcpress.com/ http://www.crcpress.com/

http://www.dmtf.org http://www.dmtf.org

http://www.linuxpromagazine.com http://www.linuxpromagazine.com

http://lxer.com http://lxer.com

http://www.nostarch.com http://www.nostarch.com

http://www.raspberry-pi-geek.com/ http://www.raspberry-pi-geek.com/

http://www.userfriendly.org http://www.userfriendly.org

##  Open Access Publishing Partners

http://peerj.com/ http://peerj.com/

© USENIX
EIN 13-3055038

- [Privacy Policy](https://www.usenix.org/privacy-policy)
- [Contact Us](https://www.usenix.org/contact)
