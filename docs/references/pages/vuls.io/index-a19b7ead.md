---
url: "https://vuls.io/"
title: "Vuls · Agentless Vulnerability Scanner for Linux/FreeBSD"
captured: "2026-11-02T18:48:26Z"
via: "live"
sha256: "c856d326e1b2bc597d2abbf428291aa8fdc950673231ec265d0f44efe116ee54"
wayback: "https://web.archive.org/web/20260904100129/https://vuls.io/"
cited_in:
  - "docs/research/vulnerability-intelligence-and-feeds.md"
---

# Vuls · Agentless Vulnerability Scanner for Linux/FreeBSD

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://vuls.io/>
> Snapshot: <https://web.archive.org/web/20260904100129/https://vuls.io/>

[## Vuls](https://vuls.io/en)

## Agentless Vulnerability Scanner for Linux/FreeBSD

[Tutorial](https://vuls.io/docs/en/tutorial.html)

[Supported OS](https://vuls.io/docs/en/supported-os.html)

[GitHub](https://github.com/future-architect/vuls)

Vuls is open-source, agent-less vulnerability scanner based on information from NVD, OVAL, etc.

https://vuls.io/img/docs/vuls-abstract.png

https://vuls.io/docs/en/supported-os.html

##

Run Anyware

Cloud, on-premise, Docker and supports major distributions.

##

High Quality Scan

Vuls uses multiple vulnerability databases NVD, JVN, OVAL, RHSA/ALAS/ELSA/FreeBSD-SA and Changelog. Vuls v0.5.0 now possible to detect vulnerabilities that patches have not been published from distributors.

https://vuls.io/docs/en/architecture-remote-local.html

##

Remote and Local Scan

Remote scan mode is required to only setup one machine that is connected to other scan target servers via SSH. If you don't want the central Vuls server to connect to each server by SSH, you can use Vuls in the Local Scan mode.

https://vuls.io/docs/en/architecture-fast-deep.html

##

Fast and Deep Scan

Fast scan mode scans without root privilege, no internet access, almost no load on the scan target server. Deep scan mode scans in more detail.

##

Dynamic Analysis

It is possible to acquire the state of the server executing some commands. Vuls v0.5.0 warns not-restarting-processes which updated before but not restarting yet. And detect processes affecting software update in advance.

https://vuls.io/docs/en/usage-scan-non-os-packages.html

##

Scan Vulnerabilites of non-OS packages

It is possible to detect vulnerabilities in non-OS packages, such as something you compiled by yourself, language libraries and frameworks, that have been registered in the [CPE](https://nvd.nist.gov/cpe.cfm).

##

[VulsRepo](https://vuls.io/docs/en/vulsrepo.html)

VulsRepo is awesome OSS Web UI for Vuls. With VulsRepo you can analyze the scan results like Excel's pivot table. Demo is avaliable https://usiusi360.github.io/vulsrepo/

https://vuls.io/docs/en/vulsrepo.html

https://asciinema.org/a/3y9zrf950agiko7klg8abvyck

##

[Terminal Based Viewer](https://vuls.io/docs/en/usage-tui.html)

Vuls has Terminal Based Viewer. You can view the scan result with vim-like keybindings.

##

[Notification](https://vuls.io/docs/en/usage-tui.html)

Email and Slack notification is possible (supports Japanese language)

## Sponsor

https://www.tines.com/?utm_source=oss&utm_medium=sponsorship&utm_campaign=vulshttps://www.sakura.ad.jp/

## Who's Using This?

This project is used by all these people

https://www.future.co.jp/https://jp.corp-sansan.com/https://www.iti-inc.co.jp/https://prottapp.com/https://www.idcf.jp/https://www.sakura.ad.jp/https://www.velc.co.jp/https://www.cgate.jp/https://dmm-corp.com/https://www.lancers.co.jp/https://mazrica.com/https://en.livesense.co.jp/https://corp.raksul.com/https://ierae.co.jp/https://www.weddingpark.co.jp/https://www.nihon-u.ac.jp/

[More Vuls Users](https://vuls.io/en/users.html)
