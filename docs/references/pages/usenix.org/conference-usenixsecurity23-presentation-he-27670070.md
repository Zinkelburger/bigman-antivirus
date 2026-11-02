---
url: "https://www.usenix.org/conference/usenixsecurity23/presentation/he"
title: "Cross Container Attacks: The Bewildered eBPF on Clouds | USENIX"
captured: "2026-11-02T19:05:02Z"
via: "live"
sha256: "01e1d5b45404ba6db4bb8fe04b8cb9e49245bca1cd36435fec61b4ad4d05dac4"
wayback: "https://web.archive.org/web/20260711110141/https://www.usenix.org/conference/usenixsecurity23/presentation/he"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# Cross Container Attacks: The Bewildered eBPF on Clouds | USENIX

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.usenix.org/conference/usenixsecurity23/presentation/he>
> Snapshot: <https://web.archive.org/web/20260711110141/https://www.usenix.org/conference/usenixsecurity23/presentation/he>

Yi He and Roland Guo, Tsinghua University and BNRist; Yunlong Xing, George Mason University; Xijia Che, Tsinghua University and BNRist; Kun Sun, George Mason University; Zhuotao Liu, Ke Xu, and Qi Li, Tsinghua University

The extended Berkeley Packet Filter (eBPF) provides powerful and flexible kernel interfaces to extend the kernel functions for user space programs via running bytecode directly in the kernel space. It has been widely used by cloud services to enhance container security, network management, and system observability. However, we discover that the offensive eBPF that have been extensively discussed in Linux hosts can bring new attack surfaces to containers. With eBPF tracing features, attackers can break the container's isolation and attack the host, e.g., steal sensitive data, DoS, and even escape the container. In this paper, we study the eBPF-based cross container attacks and reveal their security impacts in real world services. With eBPF attacks, we successfully compromise five online Jupyter/Interactive Shell services and the Cloud Shell of Google Cloud Platform. Furthermore, we find that the Kubernetes services offered by three leading cloud vendors can be exploited to launch cross-node attacks after the attackers escape the container via eBPF. Specifically, in Alibaba's Kubernetes services, attackers can compromise the whole cluster by abusing their over-privileged cloud metrics or management Pods. Unfortunately, the eBPF attacks on containers are seldom known and can hardly be discovered by existing intrusion detection systems. Also, the existing eBPF permission model cannot confine the eBPF and ensure secure usage in shared-kernel container environments. To this end, we propose a new eBPF permission model to counter the eBPF attacks in containers.

## Open Access Media

USENIX is committed to Open Access to the research presented at our events. Papers and proceedings are freely available to everyone once the event begins. Any video, audio, and/or slides that are posted after the event are also free and open to everyone. [Support USENIX](https://www.usenix.org/annual-fund) and our commitment to Open Access.

BibTeX

@inproceedings {291134,
author = {Yi He and Roland Guo and Yunlong Xing and Xijia Che and Kun Sun and Zhuotao Liu and Ke Xu and Qi Li},
title = {Cross Container Attacks: The Bewildered {eBPF} on Clouds},
booktitle = {32nd USENIX Security Symposium (USENIX Security 23)},
year = {2023},
isbn = {978-1-939133-37-3},
address = {Anaheim, CA},
pages = {5971--5988},
url = {https://www.usenix.org/conference/usenixsecurity23/presentation/he},
publisher = {USENIX Association},
month = aug
}

[Download](https://www.usenix.org/biblio/export/bibtex/291134)

[He PDF](https://www.usenix.org/system/files/usenixsecurity23-he.pdf)

[View the slides](https://www.usenix.org/system/files/sec23_slides_he.pdf)

## Presentation Video
