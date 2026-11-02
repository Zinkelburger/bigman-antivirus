---
url: "https://www.usenix.org/conference/usenixsecurity20/presentation/zhu"
title: "Measuring and Modeling the Label Dynamics of Online Anti-Malware Engines | USENIX"
captured: "2026-11-02T19:04:55Z"
via: "live"
sha256: "c3062ce7a45470302e953b09402f8276fdf1813936bb79349b8da0544e2a2a7e"
wayback: "https://web.archive.org/web/20260924201724/https://www.usenix.org/conference/usenixsecurity20/presentation/zhu"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# Measuring and Modeling the Label Dynamics of Online Anti-Malware Engines | USENIX

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.usenix.org/conference/usenixsecurity20/presentation/zhu>
> Snapshot: <https://web.archive.org/web/20260924201724/https://www.usenix.org/conference/usenixsecurity20/presentation/zhu>

Shuofei Zhu, The Pennsylvania State University; Jianjun Shi, BIT, The Pennsylvania State University; Limin Yang, University of Illinois at Urbana-Champaign; Boqin Qin, BUPT, The Pennsylvania State University; Ziyi Zhang, USTC, The Pennsylvania State University; Linhai Song, Pennsylvania State University; Gang Wang, University of Illinois at Urbana-Champaign

VirusTotal provides malware labels from a large set of anti-malware engines, and is heavily used by researchers for malware annotation and system evaluation. Since different engines often disagree with each other, researchers have used various methods to aggregate their labels. In this paper, we take a data-driven approach to categorize, reason, and validate common labeling methods used by researchers. We first survey 115 academic papers that use VirusTotal, and identify common methodologies. Then we collect the daily snapshots of VirusTotal labels for more than 14,000 files (including a subset of manually verified ground-truth) from 65 VirusTotal engines over a year. Our analysis validates the benefits of threshold-based label aggregation in stabilizing files’ labels, and also points out the impact of poorly-chosen thresholds. We show that hand-picked “trusted” engines do not always perform well, and certain groups of engines are strongly correlated and should not be treated independently. Finally, we empirically show certain engines fail to perform in-depth analysis on submitted files and can easily produce false positives. Based on our findings, we offer suggestions for future usage of VirusTotal for data annotation.

## Open Access Media

USENIX is committed to Open Access to the research presented at our events. Papers and proceedings are freely available to everyone once the event begins. Any video, audio, and/or slides that are posted after the event are also free and open to everyone. [Support USENIX](https://www.usenix.org/annual-fund) and our commitment to Open Access.

BibTeX

@inproceedings {251586,
author = {Shuofei Zhu and Jianjun Shi and Limin Yang and Boqin Qin and Ziyi Zhang and Linhai Song and Gang Wang},
title = {Measuring and Modeling the Label Dynamics of Online {Anti-Malware} Engines},
booktitle = {29th USENIX Security Symposium (USENIX Security 20)},
year = {2020},
isbn = {978-1-939133-17-5},
pages = {2361--2378},
url = {https://www.usenix.org/conference/usenixsecurity20/presentation/zhu},
publisher = {USENIX Association},
month = aug
}

[Download](https://www.usenix.org/biblio/export/bibtex/251586)

[Zhu PDF](https://www.usenix.org/system/files/sec20-zhu.pdf)

[Zhu Paper (Prepublication) PDF](https://www.usenix.org/system/files/sec20fall_zhu_prepub.pdf)

[View the slides](https://www.usenix.org/system/files/sec20_slides_zhu.pdf)

## Presentation Video
