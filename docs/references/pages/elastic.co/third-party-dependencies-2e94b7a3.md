---
url: "https://www.elastic.co/third-party-dependencies"
title: "Product Component Licenses"
captured: "2026-11-02T18:55:04Z"
via: "live"
source_url: "https://www.elastic.co/trust/third-party-dependencies"
sha256: "2eca6e96f69067f42a9b028d91b7b29c8d8dcc3c9dd5b8e3dbabd51baa55dbd8"
wayback: "https://web.archive.org/web/20231202034352/https://www.elastic.co/third-party-dependencies"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# Product Component Licenses

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.elastic.co/third-party-dependencies>
> Snapshot: <https://web.archive.org/web/20231202034352/https://www.elastic.co/third-party-dependencies>

# Product Component Licenses

Many Elastic products may be licensed under one set of terms but incorporate components either developed by Elastic or by third parties, which are governed by their own separate license terms. For your convenience, this page provides information about these components and their applicable licenses. Should you notice any discrepancy or error, please reach out to us at open-source@elastic.co.

## Third Party Components

Compliance with the license terms of any third-party components is something Elastic takes seriously. Each Elastic product that you download will contain a NOTICE file which includes the applicable licenses and required attributions for any third-party components contained within that download.

Below are links to product-specific summaries of third-party components, as well as their applicable licenses.

[Elastic Stack - Current Version](https://artifacts.elastic.co/reports/dependencies/dependencies-current.html)
Including Elasticsearch, Logstash, Kibana, Beats (including the Elastic Agent), Elastic Endpoint (shipped with the Elastic Agent), APM Server (shipped with the Elastic Agent), Fleet Server (shipped with the Elastic Agent), Elasticsearch Hadoop, Machine Learning (shipped with Elasticsearch), Elasticsearch ODBC driver, and Enterprise Search

[Elastic Cloud Enterprise (ECE) - Current Version](https://www.elastic.co/guide/en/cloud-enterprise/current/ece-dependencies.html)

[Elastic Cloud on Kubernetes (ECK) - Current Version](https://www.elastic.co/guide/en/cloud-on-k8s/current/k8s-dependencies.html)

[Elastic Endgame - Current Version](https://static-www.elastic.co/v3/assets/bltefdd0b53724fa2ce/blte7d719eab58be525/5e7bd4dba65b970bc593d9e1/notice.txt)

## First Party Components

Elastic dual licenses eBPF modules in the Elastic Endpoint component that is bundled with Linux releases of the Elastic Agent under the GNU General Public License v2.0 and the BSD 2‑Clause license, as those modules use functions in the Linux kernel that require a GPL‑compatible license. Below are the modules that are bundled with Linux releases of the Elastic Agent.

| name | license

| [Elastic Endpoint Security eBPF modules](https://github.com/elastic/ebpf) | [GPL-2.0](https://spdx.org/licenses/GPL-2.0.html) or [BSD 2‑Clause](https://spdx.org/licenses/BSD-2-Clause.html)
