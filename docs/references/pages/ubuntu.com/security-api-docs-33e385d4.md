---
url: "https://ubuntu.com/security/api/docs"
title: "Security API Docs"
captured: "2026-11-02T18:49:27Z"
via: "live"
sha256: "5797afb5e916c4095427df29367848d6a93e7e9e6217090346c6d588b4aa6c37"
cited_in:
  - "docs/research/vulnerability-intelligence-and-feeds.md"
---

# Security API Docs

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://ubuntu.com/security/api/docs>

# Warning

We have reduced the nesting of notices endpoints due to an increase in the size of the payloads that compromised the availability of the service. For each notice, you will still find IDs for linked CVEs, but if you need further detail on the linked CVEs, an additional requests will be required.

We preserved the previous endpoints behaviour on different paths, but the performance will likely not see many improvments, if you wish to use them they are:

- /security/compat/notices.json
- /security/compat/notices/{notice_id}.json
