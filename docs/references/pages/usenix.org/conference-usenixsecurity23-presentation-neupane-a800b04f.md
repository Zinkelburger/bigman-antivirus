---
url: "https://www.usenix.org/conference/usenixsecurity23/presentation/neupane"
title: "Beyond Typosquatting: An In-depth Look at Package Confusion | USENIX"
captured: "2026-11-02T19:05:04Z"
via: "live"
sha256: "5878435eac357dc6c80f3cf2401bb6395595958329853fa0da5b949fe7e9313a"
wayback: "https://web.archive.org/web/20250328052755/https://www.usenix.org/conference/usenixsecurity23/presentation/neupane"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Beyond Typosquatting: An In-depth Look at Package Confusion | USENIX

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.usenix.org/conference/usenixsecurity23/presentation/neupane>
> Snapshot: <https://web.archive.org/web/20250328052755/https://www.usenix.org/conference/usenixsecurity23/presentation/neupane>

Shradha Neupane, Worcester Polytechnic Institute; Grant Holmes, Elizabeth Wyss, and Drew Davidson, University of Kansas; Lorenzo De Carli, University of Calgary

Package confusion incidents - where a developer is misled into importing a package other than the intended one - are one of the most severe issues in supply chain security with significant security implications, especially when the wrong package has malicious functionality. While the prevalence of the issue is generally well-documented, little work has studied the range of mechanisms by which confusion in a package name could arise or be employed by an adversary. In our work, we present the first comprehensive categorization of the mechanisms used to induce confusion, and we show how this understanding can be used for detection.

First, we use qualitative analysis to identify and rigorously define 13 categories of confusion mechanisms based on a dataset of 1200+ documented attacks. Results show that, while package confusion is thought to mostly exploit typing errors, in practice attackers use a variety of mechanisms, many of which work at semantic, rather than syntactic, level. Equipped with our categorization, we then define detectors for the discovered attack categories, and we evaluate them on the entire npm package set.

Evaluation of a sample, performed through an online survey, identifies a subset of highly effective detection rules which (i) return high-quality matches (77% matches marked as potentially or highly confusing, and 18% highly confusing) and (ii) generate low warning overhead (1 warning per 100M+ package pairs). Comparison with state-of-the-art reveals that the large majority of such pairs are not flagged by existing tools. Thus, our work has the potential to concretely improve the identification of confusable package names in the wild.

## Open Access Media

USENIX is committed to Open Access to the research presented at our events. Papers and proceedings are freely available to everyone once the event begins. Any video, audio, and/or slides that are posted after the event are also free and open to everyone. [Support USENIX](https://www.usenix.org/annual-fund) and our commitment to Open Access.

BibTeX

@inproceedings {291044,
author = {Shradha Neupane and Grant Holmes and Elizabeth Wyss and Drew Davidson and Lorenzo De Carli},
title = {Beyond Typosquatting: An In-depth Look at Package Confusion},
booktitle = {32nd USENIX Security Symposium (USENIX Security 23)},
year = {2023},
isbn = {978-1-939133-37-3},
address = {Anaheim, CA},
pages = {3439--3456},
url = {https://www.usenix.org/conference/usenixsecurity23/presentation/neupane},
publisher = {USENIX Association},
month = aug
}

[Download](https://www.usenix.org/biblio/export/bibtex/291044)

[Neupane PDF](https://www.usenix.org/system/files/usenixsecurity23-neupane.pdf)

[View the slides](https://www.usenix.org/system/files/sec23_slides_neupane.pdf)

## Presentation Video
