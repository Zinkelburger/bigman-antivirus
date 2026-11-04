---
url: "https://www.ndss-symposium.org/ndss-paper/how-bad-can-it-git-characterizing-secret-leakage-in-public-github-repositories/"
title: "How Bad Can It Git? Characterizing Secret Leakage in Public GitHub Repositories - NDSS Symposium"
site: "NDSS Symposium"
captured: "2026-11-02T18:56:18Z"
via: "live"
sha256: "c5e1952e7450832192fa3c976f0e7125116cde0105606fb22918cb9b304ce4d0"
wayback: "https://web.archive.org/web/20261021121913/https://www.ndss-symposium.org/ndss-paper/how-bad-can-it-git-characterizing-secret-leakage-in-public-github-repositories/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# How Bad Can It Git? Characterizing Secret Leakage in Public GitHub Repositories - NDSS Symposium

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.ndss-symposium.org/ndss-paper/how-bad-can-it-git-characterizing-secret-leakage-in-public-github-repositories/>
> Snapshot: <https://web.archive.org/web/20261021121913/https://www.ndss-symposium.org/ndss-paper/how-bad-can-it-git-characterizing-secret-leakage-in-public-github-repositories/>

# How Bad Can It Git? Characterizing Secret Leakage in Public GitHub Repositories

Michael Meli (North Carolina State University), Matthew R. McNiece (Cisco Systems and North Carolina State University), Bradley Reaves (North Carolina State University)

GitHub and similar platforms have made public collaborative development of software commonplace. However, a problem arises when this public code must manage authentication secrets, such as API keys or cryptographic secrets. These secrets must be kept private for security, yet common development practices like adding these secrets to code make accidental leakage frequent. In this paper, we present the first large-scale and longitudinal analysis of secret leakage on GitHub. We examine billions of files collected using two complementary approaches: a nearly six-month scan of real-time public GitHub commits and a public snapshot covering 13% of open-source repositories. We focus on private key files and 11 high-impact platforms with distinctive API key formats. This focus allows us to develop conservative detection techniques that we manually and automatically evaluate to ensure accurate results. We find that not only is secret leakage pervasive — affecting over 100,000 repositories— but that thousands of new, unique secrets are leaked every day. We also use our data to explore possible root causes of leakage and to evaluate potential mitigation strategies. This work shows that secret leakage on public repository platforms is rampant and far from a solved problem, placing developers and services at persistent risk of compromise and abuse.

[Paper](https://www.ndss-symposium.org/wp-content/uploads/2019/02/ndss2019_04B-3_Meli_paper.pdf)

[Video](https://www.youtube.com/watch?v=N-pg_47s5Ok&index=4&t=1s&list=PLfUWWM-POgQtjEA_FIN7s0XFWoRdW4lil)

## View More Papers

### [Nearby Threats: Reversing, Analyzing, and Attacking Google’s ‘Nearby Connections’...](https://www.ndss-symposium.org/ndss-paper/nearby-threats-reversing-analyzing-and-attacking-googles-nearby-connections-on-android/)

Daniele Antonioli (Singapore University of Technology and Design (SUTD)), Nils Ole Tippenhauer (CISPA), Kasper Rasmussen (University of Oxford)

[Read More](https://www.ndss-symposium.org/ndss-paper/nearby-threats-reversing-analyzing-and-attacking-googles-nearby-connections-on-android/)

### [YODA: Enabling computationally intensive contracts on blockchains with Byzantine...](https://www.ndss-symposium.org/ndss-paper/yoda-enabling-computationally-intensive-contracts-on-blockchains-with-byzantine-and-selfish-nodes/)

Sourav Das (Department of Computer Science and Engineering, Indian Institute of Technology Delhi), Vinay Joseph Ribeiro (Department of Computer Science and Engineering, Indian Institute of Technology Delhi), Abhijeet Anand (Department of Computer Science and Engineering, Indian Institute of Technology Delhi)

[Read More](https://www.ndss-symposium.org/ndss-paper/yoda-enabling-computationally-intensive-contracts-on-blockchains-with-byzantine-and-selfish-nodes/)

### [DIAT: Data Integrity Attestation for Resilient Collaboration of Autonomous...](https://www.ndss-symposium.org/ndss-paper/diat-data-integrity-attestation-for-resilient-collaboration-of-autonomous-systems/)

Tigist Abera (Technische Universität Darmstadt), Raad Bahmani (Technische Universität Darmstadt), Ferdinand Brasser (Technische Universität Darmstadt), Ahmad Ibrahim (Technische Universität Darmstadt), Ahmad-Reza Sadeghi (Technische Universität Darmstadt), Matthias Schunter (Intel Labs)

[Read More](https://www.ndss-symposium.org/ndss-paper/diat-data-integrity-attestation-for-resilient-collaboration-of-autonomous-systems/)
