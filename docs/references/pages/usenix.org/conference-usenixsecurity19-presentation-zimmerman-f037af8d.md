---
url: "https://www.usenix.org/conference/usenixsecurity19/presentation/zimmerman"
title: "Small World with High Risks: A Study of Security Threats in the npm Ecosystem | USENIX"
captured: "2026-11-02T19:04:43Z"
via: "live"
sha256: "04a93da5a64f1ade859ca20a7a537d4604d33f96b436c52586d48a26b53bbdf7"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
  - "docs/threats/02-supply-chain-packages.md"
---

# Small World with High Risks: A Study of Security Threats in the npm Ecosystem | USENIX

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.usenix.org/conference/usenixsecurity19/presentation/zimmerman>

Markus Zimmermann and Cristian-Alexandru Staicu, TU Darmstadt; Cam Tenny, r2c; Michael Pradel, TU Darmstadt

The popularity of JavaScript has lead to a large ecosystem of third-party packages available via the npm software package registry. The open nature of npm has boosted its growth, providing over 800,000 free and reusable software packages. Unfortunately, this open nature also causes security risks, as evidenced by recent incidents of single packages that broke or attacked software running on millions of computers. This paper studies security risks for users of npm by systematically analyzing dependencies between packages, the maintainers responsible for these packages, and publicly reported security issues. Studying the potential for running vulnerable or malicious code due to third-party dependencies, we find that individual packages could impact large parts of the entire ecosystem. Moreover, a very small number of maintainer accounts could be used to inject malicious code into the majority of all packages, a problem that has been increasing over time. Studying the potential for accidentally using vulnerable code, we find that lack of maintenance causes many packages to depend on vulnerable code, even years after a vulnerability has become public. Our results provide evidence that npm suffers from single points of failure and that unmaintained packages threaten large code bases. We discuss several mitigation techniques, such as trusted maintainers and total first-party security, and analyze their potential effectiveness.

USENIX Security '19 Open Access Videos Sponsored by
King Abdullah University of Science and Technology (KAUST)

## Open Access Media

USENIX is committed to Open Access to the research presented at our events. Papers and proceedings are freely available to everyone once the event begins. Any video, audio, and/or slides that are posted after the event are also free and open to everyone. [Support USENIX](https://www.usenix.org/annual-fund) and our commitment to Open Access.

BibTeX

@inproceedings {236368,
author = {Markus Zimmermann and Cristian-Alexandru Staicu and Cam Tenny and Michael Pradel},
title = {Small World with High Risks: A Study of Security Threats in the npm Ecosystem},
booktitle = {28th USENIX Security Symposium (USENIX Security 19)},
year = {2019},
isbn = {978-1-939133-06-9},
address = {Santa Clara, CA},
pages = {995--1010},
url = {https://www.usenix.org/conference/usenixsecurity19/presentation/zimmerman},
publisher = {USENIX Association},
month = aug
}

[Download](https://www.usenix.org/biblio/export/bibtex/236368)

[Zimmermann PDF](https://www.usenix.org/system/files/sec19-zimmermann.pdf)

[View the slides](https://www.usenix.org/sites/default/files/conference/protected-files/sec19_slides_zimmermann.pdf)

## Presentation Video
