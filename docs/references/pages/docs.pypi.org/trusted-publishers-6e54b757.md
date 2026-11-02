---
url: "https://docs.pypi.org/trusted-publishers/"
title: "Getting Started - PyPI User Documentation"
captured: "2026-11-02T18:28:59Z"
via: "live"
sha256: "3bb9ea16722110bfd3ada35958af3721f600334b38ded4b2b572ee9275bed78f"
wayback: "https://web.archive.org/web/20261023004918/https://docs.pypi.org/trusted-publishers/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Getting Started - PyPI User Documentation

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://docs.pypi.org/trusted-publishers/>
> Snapshot: <https://web.archive.org/web/20261023004918/https://docs.pypi.org/trusted-publishers/>

https://github.com/pypi/warehouse/blob/main/docs/user/trusted-publishers/index.md

# Publishing to PyPI with a Trusted Publisher[¶](https://docs.pypi.org/trusted-publishers/#publishing-to-pypi-with-a-trusted-publisher)

"Trusted Publishing" is our term for using the [OpenID Connect (OIDC)](https://openid.net/connect/) standard to exchange short-lived identity tokens between a trusted third-party service and PyPI. This method can be used in automated environments and eliminates the need to use manually generated API tokens to authenticate with PyPI when publishing.

For a quickstart, see:

- [Adding a trusted publisher to an existing PyPI project](https://docs.pypi.org/trusted-publishers/adding-a-publisher/)
- [Creating a PyPI project with a trusted publisher](https://docs.pypi.org/trusted-publishers/creating-a-project-through-oidc/)

## Quick background: Publishing with OpenID Connect[¶](https://docs.pypi.org/trusted-publishers/#quick-background-publishing-with-openid-connect)

OpenID Connect (OIDC) publishing is a mechanism for uploading packages to PyPI, complementing API tokens.

You don't need to understand OIDC to use OIDC publishing with PyPI, but here's the TL;DR:

- Certain CI services (like GitHub Actions) are OIDC identity providers, meaning that they can issue short-lived credentials ("OIDC tokens") that a third party can strongly verify came from the CI service (as well as which user, repository, etc. actually executed);
- Projects on PyPI can be configured to trust a particular configuration on a particular CI service, making that configuration an OIDC publisher for that project;
- Release automation (like GitHub Actions) can submit an OIDC token to PyPI. The token will be matched against configurations trusted by different projects; if any projects trust the token's configuration, then PyPI will mint a short-lived API token for those projects and return it;
- The short-lived API token behaves exactly like a normal project-scoped API token, except that it's only valid for 15 minutes from time of creation (enough time for the CI to use it to upload packages).

This confers significant usability and security advantages when compared to PyPI's traditional authentication methods:

- Usability: with Trusted Publishing, users no longer need to manually create API tokens on PyPI and copy-paste them into their CI provider. The only manual step is configuring the publisher on PyPI.
- Security: PyPI's normal API tokens are long-lived, meaning that an attacker who compromises a package's release token can use it until its legitimate user notices and manually revokes it. Trusted Publishing avoids this problem because the tokens minted expire automatically.
