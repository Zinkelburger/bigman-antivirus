---
url: "https://threats.wiz.io/all-incidents/migo-cryptominer-targeting-redis"
title: "Migo cryptominer targeting Redis"
site: "Migo cryptominer targeting Redis"
captured: "2026-11-02T18:46:40Z"
via: "live"
sha256: "83add5037359259a2ff80d2c0e4f8bedece5e24755b28e0a26f2aa1fd9ec4116"
wayback: "https://web.archive.org/web/20260617155633/https://threats.wiz.io/all-incidents/migo-cryptominer-targeting-redis"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Migo cryptominer targeting Redis

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://threats.wiz.io/all-incidents/migo-cryptominer-targeting-redis>
> Snapshot: <https://web.archive.org/web/20260617155633/https://threats.wiz.io/all-incidents/migo-cryptominer-targeting-redis>

Type

Campaign

Actors

[🏁Migo operator](https://threats.wiz.io/all-actors/migo-operator)

Pub. date

February 20, 2024

Initial access

Software misconfig

Impact

Resource hijacking

Observed techniques

[Misconfigured Redis abuse](https://threats.wiz.io/all-techniques/misconfigured-redis-abuse)

Observed tools

[XMRig](https://threats.wiz.io/all-tools/xmrig)

Targeted technologies

[Redis](https://threats.wiz.io/targeted-technologies/redis)

References

https://www.cadosecurity.com/migo-a-redis-miner-with-novel-system-weakening-techniques/

Status

Finalized

Last edited

Jun 2, 2024 8:02 AM

A new campaign named Migo targeting Redis servers running on Linux hosts to mine cryptocurrency. The campaign was identified following suspicious activities on a Redis honeypot, where a malicious node disabled several Redis configuration options to weaken security and facilitate unauthorized access. The primary payload, a statically-linked, stripped, and UPX-packed ELF binary, is obfuscated to hinder analysis. It includes a variety of functions for system manipulation, HTTP requests, and file operations. The malware sets up a miner by fetching and configuring XMRig, adjusts system parameters to optimize mining performance, and also performs certain benign actions, possibly to evade detection.
