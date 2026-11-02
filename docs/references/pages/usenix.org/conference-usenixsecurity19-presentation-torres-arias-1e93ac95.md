---
url: "https://www.usenix.org/conference/usenixsecurity19/presentation/torres-arias"
title: "in-toto: Providing farm-to-table guarantees for bits and bytes | USENIX"
captured: "2026-11-02T19:03:45Z"
via: "live"
sha256: "9eaa3581d36ac7d961a3e3bc1b624c55b710a783f37fbd5aebb4c842fea6725d"
wayback: "https://web.archive.org/web/20261016043932/https://www.usenix.org/conference/usenixsecurity19/presentation/torres-arias"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# in-toto: Providing farm-to-table guarantees for bits and bytes | USENIX

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.usenix.org/conference/usenixsecurity19/presentation/torres-arias>
> Snapshot: <https://web.archive.org/web/20261016043932/https://www.usenix.org/conference/usenixsecurity19/presentation/torres-arias>

Santiago Torres-Arias, New York University; Hammad Afzali, New Jersey Institute of Technology; Trishank Karthik Kuppusamy, Datadog; Reza Curtmola, New Jersey Institute of Technology; Justin Cappos, New York University

The software development process is quite complex and involves a number of independent actors. Developers check source code into a version control system, the code is compiled into software at a build farm, and CI/CD systems run multiple tests to ensure the software’s quality among a myriad of other operations. Finally, the software is packaged for distribution into a delivered product, to be consumed by end users. An attacker that is able to compromise any single step in the process can maliciously modify the software and harm any of the software’s users.

To address these issues, we designed in-toto, a framework that cryptographically ensures the integrity of the software supply chain. in-toto grants the end user the ability to verify the software’s supply chain from the project’s inception to its deployment. We demonstrate in-toto’s effectiveness on 30 software supply chain compromises that affected hundreds of million of users and showcase in-toto’s usage over cloud-native, hybrid-cloud and cloud-agnostic applications. in-toto is integrated into products and open source projects that are used by millions of people daily.

USENIX Security '19 Open Access Videos Sponsored by
King Abdullah University of Science and Technology (KAUST)

## Open Access Media

USENIX is committed to Open Access to the research presented at our events. Papers and proceedings are freely available to everyone once the event begins. Any video, audio, and/or slides that are posted after the event are also free and open to everyone. [Support USENIX](https://www.usenix.org/annual-fund) and our commitment to Open Access.

BibTeX

@inproceedings {236322,
author = {Santiago Torres-Arias and Hammad Afzali and Trishank Karthik Kuppusamy and Reza Curtmola and Justin Cappos},
title = {in-toto: Providing farm-to-table guarantees for bits and bytes},
booktitle = {28th USENIX Security Symposium (USENIX Security 19)},
year = {2019},
isbn = {978-1-939133-06-9},
address = {Santa Clara, CA},
pages = {1393--1410},
url = {https://www.usenix.org/conference/usenixsecurity19/presentation/torres-arias},
publisher = {USENIX Association},
month = aug
}

[Download](https://www.usenix.org/biblio/export/bibtex/236322)

[Torres-Arias PDF](https://www.usenix.org/system/files/sec19-torres-arias.pdf)

## Presentation Video
