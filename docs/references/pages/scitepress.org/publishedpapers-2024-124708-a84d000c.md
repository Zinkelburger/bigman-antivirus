---
url: "https://scitepress.org/PublishedPapers/2024/124708"
title: "SCITEPRESS - SCIENCE AND TECHNOLOGY PUBLICATIONS"
captured: "2026-11-02T18:42:07Z"
via: "live"
source_url: "https://www.scitepress.org/PublishedPapers/2024/124708/"
sha256: "107c58de72453a83ccbad17a4c9bbca944ff5d0e1702559c44de5e3210ab4e0e"
wayback: "https://web.archive.org/web/20240423024025/https://www.scitepress.org/PublishedPapers/2024/124708/"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# SCITEPRESS - SCIENCE AND TECHNOLOGY PUBLICATIONS

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://scitepress.org/PublishedPapers/2024/124708>
> Snapshot: <https://web.archive.org/web/20240423024025/https://www.scitepress.org/PublishedPapers/2024/124708/>

[# Detecting eBPF Rootkits Using Virtualization and Memory Forensics](https://www.scitepress.org/publishedPapers/2024/124708/pdf/index.html)

###  Nezer Zaidenberg, Michael Kiperberg, Eliav Menachi, Asaf Eitani

#### 2024

#### Abstract

There is a constant increase in the sophistication of cyber threats. Areas considered immune to malicious code, such as eBPF, are shown to be perfectly suitable for malware. Initially, the eBPF mechanism was devised to inject small programs into the kernel, assisting in network routing and filtering. Recently, it was demonstrated that malicious eBPF programs can be used to construct rootkits. The previously proposed countermeasures need to be revised against rootkits that attempt to hide their presence. We propose a novel detection scheme that divides the detection process into two phases. In the first phase, the memory image of the potentially infected system is acquired using a hypervisor. In the second phase, the image is analyzed. The analysis includes extraction and classification of the eBPF programs. The classifier’s decision is based on the set of helper functions used by each eBPF program. Our study revealed a set of helper functions used only by malicious eBPF programs. The proposed scheme achieves optimal precision while suffering only a minor performance penalty for each additional eBPF program.

[Download](http://www.scitepress.org/Papers/2024/124708/124708.pdf)

#### Paper Citation

#### in Harvard Style

Zaidenberg N., Kiperberg M., Menachi E. and Eitani A. (2024). Detecting eBPF Rootkits Using Virtualization and Memory Forensics. In Proceedings of the 10th International Conference on Information Systems Security and Privacy - Volume 1: ICISSP; ISBN 978-989-758-683-5, SciTePress, pages 254-261. DOI: 10.5220/0012470800003648

#### in Bibtex Style

@conference{icissp24,
author={Nezer Zaidenberg and Michael Kiperberg and Eliav Menachi and Asaf Eitani},
title={Detecting eBPF Rootkits Using Virtualization and Memory Forensics},
booktitle={Proceedings of the 10th International Conference on Information Systems Security and Privacy - Volume 1: ICISSP},
year={2024},
pages={254-261},
publisher={SciTePress},
organization={INSTICC},
doi={10.5220/0012470800003648},
isbn={978-989-758-683-5},
}

#### in EndNote Style

TY - CONF

JO - Proceedings of the 10th International Conference on Information Systems Security and Privacy - Volume 1: ICISSP
TI - Detecting eBPF Rootkits Using Virtualization and Memory Forensics
SN - 978-989-758-683-5
AU - Zaidenberg N.
AU - Kiperberg M.
AU - Menachi E.
AU - Eitani A.
PY - 2024
SP - 254
EP - 261
DO - 10.5220/0012470800003648
PB - SciTePress
