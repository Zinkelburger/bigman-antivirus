---
url: "https://blog.gitguardian.com/three-supply-chain-campaigns-hit-npm-pypi-and-docker-hub-in-48-hours/"
title: "No Off Season: Three Supply Chain Campaigns Hit npm, PyPI, and Docker Hub in 48 Hours"
site: "GitGuardian Blog - Take Control of Your Secrets Security"
published: "2026-04-23T09:32:50.000Z"
captured: "2026-11-02T18:22:52Z"
via: "live"
sha256: "aa7b521b1bfc0d9fada0858fd6f710d8d80cb2c532cf5c35dfe4c5a5b9d05668"
wayback: "https://web.archive.org/web/20261023181344/https://blog.gitguardian.com/three-supply-chain-campaigns-hit-npm-pypi-and-docker-hub-in-48-hours/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# No Off Season: Three Supply Chain Campaigns Hit npm, PyPI, and Docker Hub in 48 Hours

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://blog.gitguardian.com/three-supply-chain-campaigns-hit-npm-pypi-and-docker-hub-in-48-hours/>
> Snapshot: <https://web.archive.org/web/20261023181344/https://blog.gitguardian.com/three-supply-chain-campaigns-hit-npm-pypi-and-docker-hub-in-48-hours/>

[Breach explained](https://blog.gitguardian.com/tag/breach-explained/)

# No Off Season: Three Supply Chain Campaigns Hit npm, PyPI, and Docker Hub in 48 Hours

Three supply chain attacks hit npm, PyPI, and Docker Hub between April 21–23, 2026. All three targeted secrets: API keys, cloud credentials, SSH keys, and tokens from developer environments and CI/CD pipelines.

-

## Guillaume Valadon

Guillaume is a Cybersecurity Researcher at GitGuardian. He holds a PhD in networking. He likes looking at data and crafting packets. He co-maintains Scapy. And he still remembers what AT+MS=V34 means!

[More posts](https://blog.gitguardian.com/author/guillaumevaladon/) by Guillaume Valadon.

https://blog.gitguardian.com/author/guillaumevaladon/

#### [Guillaume Valadon](https://blog.gitguardian.com/author/guillaumevaladon/)

23 Apr 2026 • 3 min read

Follow us on

https://www.linkedin.com/company/gitguardian/ https://bsky.app/profile/gitguardian.com

After a few quieter weeks, three supply chain attacks put secrets back in the spotlight.

Between April 21 and 23, 2026, three distinct attacks hit npm, PyPI, and Docker Hub simultaneously. Their targets differ and the threat actor groups might, but their objectives don't: in each case, the malware's primary goal was to steal secrets from developer environments and CI/CD pipelines. API keys, cloud credentials, SSH keys, and registry tokens were all targeted.

## Campaign 1 - Checkmarx KICS: Compromised Security Scanner Turns on Its Users

The first attack compromised official Checkmarx KICS Docker images and VS Code extensions. Docker flagged suspicious activity on the checkmarx/kics repository on April 22 and alerted Socket. An obfuscated payload harvested GitHub authentication tokens, AWS credentials, Azure and Google Cloud tokens, npm configuration files, SSH keys, and environment variables, compressing and encrypting everything before exfiltration. The payload swept up any API keys stored in environment variables.

TeamPCP likely orchestrated the attack, based on posts they published on X immediately after disclosure. This would be the group's second Checkmarx attack in two months.

## Campaign 2 - CanisterSprawl: A Worm That Turns Developer Machines into Launchpads

On April 21, malicious versions of pgserve, a PostgreSQL server for Node.js, appeared on npm. The compromised versions inject a credential-harvesting script that runs via a postinstall hook on every npm install. It searches for npm publish tokens, and for each package the victim can publish, it bumps the patch version, injects itself, and publishes them to npm. If a PyPI token is also found, the worm jumps ecosystems entirely.

Socket and StepSecurity track this as CanisterSprawl, named after its use of an Internet Computer Protocol (ICP) canister as a resilient, decentralized C2 channel. Socket's follow-up investigation linked compromised Namastex.ai npm packages to the same core methods: install-time execution, credential theft, off-host exfiltration to canister-backed infrastructure, and self-propagation logic.

## Campaign 3 - xinference: TeamPCP Returns to PyPI

On April 22, three consecutive releases of xinference on PyPI carried a credential-stealing payload. The malware decodes a second-stage collector, harvests SSH keys, cloud credentials, environment variables, and crypto wallets. StepSecurity attributes this to TeamPCP, the same group behind the [litellm](https://blog.gitguardian.com/litellm-supply-chain-attack/) and telnyx PyPI compromises in March.

There is one notable technical difference from prior TeamPCP campaigns: the xinference payload sends a plain tar.gz directly to the C2 server. The lack of encryption is why some researchers have suggested a copycat, though the injection pattern and multi-version cadence remain consistent with TeamPCP's established tradecraft.

## The Common Thread

Three campaigns, three ecosystems, one objective. None of these attacks aimed to disrupt software delivery or corrupting build outputs. Every payload, from the CanisterSprawl worm to the trojanized KICS scanner to the xinference stealer, was engineered to do one thing: extract credentials from the environments where developers and pipelines operate. The question every affected team should be asking right now isn't just "did this package run in my environment?" It's: what secrets were accessible if it did, and have they been rotated?

Answering that requires knowing where your secrets live: across repositories, CI configurations, environment variables, and developer machines. GitGuardian provides continuous detection of exposed secrets across every surface attackers target, repositories, CI configs, environment variables, and developer machines, so when the next compromised package runs in your pipeline, you're not starting from zero.

[The Team PCP Snowball Effect: A Quantitative Analysis

Supply chain attacks cascade through ecosystems in ways traditional metrics hardly capture. GitGuardian evaluates the PCP Team incidents and finds damage spread to thousands of public targets.

GitGuardian Blog - Take Control of Your Secrets SecurityGaetan Ferry](https://blog.gitguardian.com/team-pcp-snowball-analysis/)

@npm

https://www.gitguardian.com/interactive-demo?ref=blog.gitguardian.com

https://blog.gitguardian.com/binance-hack-compromised-laptop/

[Endpoint Protection](https://blog.gitguardian.com/tag/endpoint-protection/) [Breach explained](https://blog.gitguardian.com/tag/breach-explained/)

[# The Binance Hack Raises a Critical Question: What Can a Compromised Laptop Reach?](https://blog.gitguardian.com/binance-hack-compromised-laptop/)

A compromised laptop may have started the Binance hack. What can an attacker reach from a developer’s machine?

https://blog.gitguardian.com/author/katie/

[### Katie DeMatteis](https://blog.gitguardian.com/author/katie/)

2 Nov 2026 – 3 min read

https://blog.gitguardian.com/ghostaction-x-polinrider/

[Breach explained](https://blog.gitguardian.com/tag/breach-explained/)

[# GhostAction Crosses Paths With PolinRider: Inside the October Escalation](https://blog.gitguardian.com/ghostaction-x-polinrider/)

GhostAction pushed 72,026 commits to 22,964 repos in nine days, now targeting CI runners and developer machines with a launcher copied from PolinRider.

https://blog.gitguardian.com/author/gaetan/

[### Gaetan Ferry](https://blog.gitguardian.com/author/gaetan/)

27 Oct 2026 – 10 min read

https://blog.gitguardian.com/php2shell-php-vulnerability-secrets/

[Breach explained](https://blog.gitguardian.com/tag/breach-explained/)

[# Responding to php2shell PHP Vulnerability When You Don’t Know What Secrets Are Deployed](https://blog.gitguardian.com/php2shell-php-vulnerability-secrets/)

On Oct. 21, 2026, PHP fixed a vulnerability named php2shell by Calif. Pre-auth RCE under default configurations, confirmed on common CMS, webmail, e-commerce, file sharing, and database administration applications. A PHP server also stores and uses credentials, so this is a secret theft vector.

https://blog.gitguardian.com/author/cybersecurity/

[### Cybersecurity research team](https://blog.gitguardian.com/author/cybersecurity/)

22 Oct 2026 – 8 min read

https://blog.gitguardian.com/railway-october-2026-incident-environment-variables/

[Breach explained](https://blog.gitguardian.com/tag/breach-explained/)

[# Railway October 2026 Incident: Treat Every Environment Variable as Exposed](https://blog.gitguardian.com/railway-october-2026-incident-environment-variables/)

Railway's security incident may have exposed decrypted customer environment variables. Pull and scan yours with GitGuardian to know what to rotate first.

https://blog.gitguardian.com/author/cybersecurity/

[### Cybersecurity research team](https://blog.gitguardian.com/author/cybersecurity/)

22 Oct 2026 – 2 min read
