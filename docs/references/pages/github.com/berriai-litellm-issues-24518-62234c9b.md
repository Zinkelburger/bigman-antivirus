---
url: "https://github.com/BerriAI/litellm/issues/24518"
title: "[Security]: litellm PyPI package (v1.82.7 + v1.82.8) compromised — full timeline and status · Issue #24518 · BerriAI/litellm"
site: "GitHub"
captured: "2026-11-02T18:34:36Z"
via: "live"
sha256: "b252b2a731e0436acddfc939eb5b98b31f72cdcce632fd3d43126d052ee8bcba"
wayback: "https://web.archive.org/web/20261023150812/https://github.com/BerriAI/litellm/issues/24518"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# [Security]: litellm PyPI package (v1.82.7 + v1.82.8) compromised — full timeline and status · Issue #24518 · BerriAI/litellm

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/BerriAI/litellm/issues/24518>
> Snapshot: <https://web.archive.org/web/20261023150812/https://github.com/BerriAI/litellm/issues/24518>

## Repository navigation

# [Security]: litellm PyPI package (v1.82.7 + v1.82.8) compromised — full timeline and status #24518

Closed

Closed

[[Security]: litellm PyPI package (v1.82.7 + v1.82.8) compromised — full timeline and status](https://github.com/BerriAI/litellm/issues/24518#top)#24518

Labels

[llm translation](https://github.com/BerriAI/litellm/issues?q=state%3Aopen%20label%3A%22llm%20translation%22)

## Description

https://github.com/isfinne

[isfinne](https://github.com/isfinne)

opened [on Mar 24, 2026](https://github.com/BerriAI/litellm/issues/24518#issue-4128070814)

## The Trivy supply-chain compromise has been contained 🎉 . All affected packages have been deleted and current releases are free of the compromised code/component. Please refer to our [Security Townhall](https://docs.litellm.ai/blog/security-townhall-updates) for a deeper understanding of the problem, and [CI/CD v2](https://docs.litellm.ai/blog/ci-cd-v2-improvements) for how we're improving moving forward.

[LITELLM TEAM UPDATES]

- Compromised packages have been deleted (v1.82.7, v1.82.8)
- Compromise came from trivvy security scan dependency
- All maintainer accounts have been rotated (new maintainer accounts: [@krrish-berri-2](https://github.com/krrish-berri-2) , @ishaan-berri)
- Proxy Docker image users were not impacted, all dependencies are pinned on requirements.txt
- No litellm releases will be out until we have scanned our chain and make sure it's safe

Next Steps

- Review all berriai repo's for impact
- Scan circle ci builds to understand blast radius, and mitigate it
- We've engaged Google's [mandiant.security](https://cloud.google.com/security/mandiant) team, and are actively working on this with them

We are actively investigating this issue. Please reach out to us on support@berri.ai, if you have any questions / concerns.

## Summary

The litellm PyPI package was compromised by an attacker who gained access to the maintainer's PyPI account. Malicious versions were published that steal credentials and exfiltrate them to an attacker-controlled server.

Original detailed analysis: [#24512](https://github.com/BerriAI/litellm/issues/24512)

Hacker News discussion: https://news.ycombinator.com/item?id=47501729

## What happened

- The maintainer's PyPI account (krrishdholakia) appears to have been hijacked by an attacker (teampcp)
- The attacker published malicious versions to PyPI that were never released through the official GitHub CI/CD
- GitHub releases only go up to v1.82.6.dev1 — versions 1.82.7 and 1.82.8 on PyPI were uploaded directly by the attacker

## Affected versions

| Version  | Method  | Trigger

| 1.82.7  | Payload embedded in litellm/proxy/proxy_server.py  | Triggered on import litellm.proxy

| 1.82.8  | Added litellm_init.pth (34,628 bytes) + payload in proxy_server.py  | Any Python startup — no import needed

Other versions may also be affected and should be audited.

## What the malicious code does

- Collects: SSH keys, environment variables (API keys, secrets), AWS/GCP/Azure/K8s credentials, crypto wallets, database passwords, SSL private keys, shell history, CI/CD configs
- Encrypts: AES-256-CBC + RSA-4096 (hardcoded public key)
- Exfiltrates: curl POST to https://models.litellm.cloud/

The exfiltration domain litellm.cloud (NOT the official litellm.ai) was registered on 2026-03-23 via Spaceship, Inc. — just hours before the malicious packages appeared.

## Current status

- PyPI: The entire litellm package has been suspended/removed. All versions currently return "No matching distribution found." We reported the malware to PyPI via the official "Report malware" form.
- GitHub Issue [[Security]: CRITICAL: Malicious litellm_init.pth in litellm 1.82.8 — credential stealer #24512](https://github.com/BerriAI/litellm/issues/24512): Contains the original detailed technical analysis (currently closed by the attacker's spam — see below).
- Attacker behavior: The attacker appears to be publishing hundreds of spam comments to suppress discussion. If this continues, we recommend moderating via the Hacker News thread linked above.

## Recommendations for affected users

- Check if litellm_init.pth exists in your site-packages/ directory
- Rotate ALL credentials that were present as environment variables or config files on any system where litellm 1.82.7+ was installed
- Pin dependencies to exact versions and verify against GitHub releases
- Monitor for unauthorized access using any potentially leaked credentials

## References

- Original analysis: [[Security]: CRITICAL: Malicious litellm_init.pth in litellm 1.82.8 — credential stealer #24512](https://github.com/BerriAI/litellm/issues/24512)
- Hacker News: https://news.ycombinator.com/item?id=47501729
- Attacker's exfil domain WHOIS: registered 2026-03-23, Spaceship Inc., privacy-protected
- litellm_init.pth SHA256: ceNa7wMJnNHy1kRnNCcwJaFjWX3pORLfMh7xGL8TUjg

Reacted by Anton, Fares, nicknguyen-ccm, Quentin Delfosse, rushab-agi, Hai Zhu, Adnan Omarkhil, David Wood, Mike Zukowski, Amine El hattami and 127 more

Reacted by Loreto Parisi, Luan Carlos, Coy Geek, Julián Acosta Taborda, Alex Hermida, Andrei Radulescu-Banu, Adam Richeimer, Alekseev Ilya, Nick Brady, Brian Burns and 10 more

Reacted by 1z5q, Kedharnath, Coy Geek, Alex Hermida, JonZeolla, Piggy, nikkie, Yatit Thakker, BEE-JN and Nathan Weinshenker

## Activity

-

[github-actions](https://github.com/apps/github-actions)

added

[llm translation](https://github.com/BerriAI/litellm/issues?q=state%3Aopen%20label%3A%22llm%20translation%22)

[on Mar 24, 2026](https://github.com/BerriAI/litellm/issues/24518#event-23853310825)

-

ghost pinned this issue [on Mar 24, 2026](https://github.com/BerriAI/litellm/issues/24518#event-23853424150)

-

-

### thenoblet commented on Mar 24, 2026

https://github.com/thenoblet

[thenoblet](https://github.com/thenoblet)

[on Mar 24, 2026](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4118525329)

Pypi has put the project in quarantine. this should block additional downloads.

Our entire deployment has failed due to this issue. Pypi quarantined it, and I am just finding out. Such a bummer!!

Reacted by Virgiliu

Reacted by Job Rosana, kostiantyn-yevdiukhin-cyera, Benjamin Houdu, Yusuf Sertkaya, ViYu, Mihai Maruseac, Thibault Jaigu, tsukumi, Michael, Patrick A. Noblet and 6 more

-

### DanielRuf commented on Mar 24, 2026

https://github.com/DanielRuf

[DanielRuf](https://github.com/DanielRuf)

[on Mar 24, 2026](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4118542999)

This was probably caused by the compromised trivy project:

https://github.com/search?q=repo%3ABerriAI%2Flitellm%20trivy&type=code
https://ramimac.me/trivy-teampcp/#phase-09

[litellm/ci_cd/security_scans.sh](https://github.com/BerriAI/litellm/blob/9343aeefca37aa49a6ea54397d7615adae5c72c9/ci_cd/security_scans.sh#L16)

Line 16 in [9343aee](https://github.com/BerriAI/litellm/commit/9343aeefca37aa49a6ea54397d7615adae5c72c9)

|   |  echo "deb https://aquasecurity.github.io/trivy-repo/deb $(lsb_release -sc) main" | sudo tee -a /etc/apt/sources.list.d/trivy.list

The full timeline contains more details.

Reacted by AllyourBaseBelongToUs, Dan, Mich and Jayanth Reddy

Reacted by Job Rosana, Benjamin Houdu, Mihai Maruseac, Jeffrey Martin, Devin Stein, Certeza, Chris Hennick, Charles Xu, RCStef and José

-

### akx commented on Mar 24, 2026

https://github.com/akx

[akx](https://github.com/akx)

[on Mar 24, 2026](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4118626164)

The maintainer's PyPI account (krrishdholakia) appears to have been hijacked by an attacker (teampcp)

Note that it's not just PyPI... See e.g. https://github.com/krrishdholakia/blockchain/commit/556f2db38e98bff4495ced183ea6253c5a68bfe0 (unrelated repo) (and other recent activity).

Reacted by Hafedh Hichri

Reacted by Milad Taghikhani, Luan Carlos, Raaj and Colin Jacobs

-

### DanielRuf commented on Mar 24, 2026

https://github.com/DanielRuf

[DanielRuf](https://github.com/DanielRuf)

[on Mar 24, 2026](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4118638134)

Yes, the trivy project contained a credential stealer and the threat actors used PATs and other secrets extracted via CI / CD.

This means full rotation after cleanup for all involved secrets and connected services (PyPi etc).

-

### dkindlund commented on Mar 24, 2026

https://github.com/dkindlund

[dkindlund](https://github.com/dkindlund)

[on Mar 24, 2026](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4118666370)

Contributor

The Attack Chain: Trivy → LiteLLM PyPI

Based on the ramimac writeup and issue [#24518](https://github.com/BerriAI/litellm/issues/24518):

1. March 1: Aqua Security (Trivy's maintainer) suffered an initial breach
2. Incomplete credential rotation left tokens available to the attacker (TeamPCP)
3. TeamPCP compromised Trivy — poisoned v0.69.4-v0.69.6, force-pushed all 76+ trivy-action tags, and pushed malicious Docker images directly to Docker Hub
4. March 23: Attacker registered litellm.cloud domain (exfiltration target)
5. March 24 ~08:30 UTC: Published malicious litellm==1.82.7 and 1.82.8 to PyPI using the compromised maintainer account krrishdholakia

The connection: the Trivy compromise likely gave the attacker access to the LiteLLM maintainer's PyPI credentials (either harvested from CI secrets via the compromised Trivy runs, or through the broader credential-stealing payload).

GitHub Source Code: NOT Compromised

- No malicious code in any GitHub tag from v1.82.3-stable through v1.82.6.rc.2
- No v1.82.7 or v1.82.8 tags exist on GitHub — the attacker published directly to PyPI
- The base64 and subprocess usages in proxy_server.py across all tagged versions are legitimate (config handling, callbacks)
- No .pth files, no litellm.cloud references, no exec(base64.b64decode(...)) patterns in the source
- The simple_pypi_publish.yml workflow was last modified July 2025 — not tampered with
- Last legitimate CI-published version was March 13

LiteLLM's Trivy Usage — Potential Exposure Vector

LiteLLM's ci_cd/security_scans.sh installs Trivy via apt from [aquasecurity.github.io/trivy-repo/deb](http://aquasecurity.github.io/trivy-repo/deb) without version pinning — it pulls whatever the latest version is. If a CI run occurred during the Trivy compromise window, it would have installed the poisoned Trivy, which could have exfiltrated CI secrets including the PYPI_PUBLISH_PASSWORD.

Reacted by Gabriel Huang, Maarten Betman, Alfred.Liu, Jun Chen, Rade Martinović, Chris Hennick, Yeray Díaz and q--

Reacted by Felix Eckhofer, Prashant Pandey, Henrique Pickler, Jakub, Thibault Jaigu, Luke Hearon, :), Cameron Copland, Nguyen Phu Thien, Kyungmo Ku and 8 more

-

### DanielRuf commented on Mar 24, 2026

https://github.com/DanielRuf

[DanielRuf](https://github.com/DanielRuf)

[on Mar 24, 2026](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4118817947)

To me it seems like they also got access to a PAT, created by the maintainer: https://github.com/krrishdholakia/blockchain/commit/556f2db38e98bff4495ced183ea6253c5a68bfe0

So should commits be checked since the beginning of the Trivy incident?
Not sure but I have already contacted him and recommended full rotation after cleanup (or as additional note: on a completely different computer which is not compromised, rotation on any currently compromised computer would not help much).

I recommended some manual steps and to seal and shard secrets, since I have experienced other incidents like leaked secrets on CircleCI, Travis CI, ... in the past. The general design flaw is, that most tokens (even PATs) have a very long lifetime and processes do not involve any 2FA step as requirement in most cases.

[aquasecurity/trivy#10425 (reply in thread)](https://github.com/aquasecurity/trivy/discussions/10425#discussioncomment-16275040)

Why I think the proposed process may work? The eslint team did something similar after the eslint-scope incident. They used keybase and enforced 2FA:
[eslint/eslint#10631](https://github.com/eslint/eslint/issues/10631)
https://blog.teddykatz.com/2021/03/17/github-actions-write-access.html#fn:6

-

### dkindlund commented on Mar 24, 2026

https://github.com/dkindlund

[dkindlund](https://github.com/dkindlund)

[on Mar 24, 2026](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4118851251)

Contributor

[@DanielRuf](https://github.com/DanielRuf) , agreed.

Attack Chain: Trivy → GitHub Token → Full Account Access

- Trivy CI runs in LiteLLM's GitHub Actions — ci_cd/security_scans.sh installs Trivy from the apt repo without version pinning
- Compromised Trivy exfiltrates CI secrets — the poisoned Trivy binary (v0.69.4+) contained credential-harvesting code that dumped environment variables and secrets via Cloudflare Tunnels. In GitHub Actions, the runner environment has access to:
- GITHUB_TOKEN — the auto-generated token for the workflow run
- Any repository secrets configured for the workflow (like PYPI_PUBLISH_PASSWORD)
- GITHUB_TOKEN scope — by default, GITHUB_TOKEN in GitHub Actions is scoped to the repository where the workflow runs. But if the workflow has permissions:
contents: write (which the ghcr_deploy.yml workflow recently added), it can push to that repo.
- Personal Access Token (PAT) exposure — however, pushing to [krrishdholakia/blockchain](https://github.com/krrishdholakia/blockchain/commit/556f2db38e98bff4495ced183ea6253c5a68bfe0) (a different repo) requires broader access than GITHUB_TOKEN. This means the attacker likely captured either:
- A Personal Access Token (PAT) stored as a repository secret or environment variable in the CI
- Or the maintainer's GitHub session/OAuth credentials via the broader credential harvesting (env vars, .gitconfig, .git-credentials)
- With the PAT, the attacker had the maintainer's full GitHub identity — able to push to any repo the maintainer owns, including personal repos like blockchain, and authenticate to PyPI if the same credentials or tokens were reused.

The Smoking Gun

The commit was made at 2026-03-24T12:59:46Z — the same day as the malicious PyPI packages. The attacker used the maintainer's actual name and email (Krish Dholakia, krrishdholakia@gmail.com), which is consistent with using a stolen PAT or session token (GitHub attributes commits to the token owner).

This confirms the attacker had full control of the maintainer's GitHub account, not just repo-scoped CI tokens. The Trivy payload likely harvested a PAT or OAuth token from the CI environment that had broad account-level permissions.

Reacted by Daniil Dudkin, Prashant Pandey, Aarni Koskela, Henrique Pickler, Jakub, Claudia Pellegrino, Antoine Viallon and Sam Broughton

-

[centerseed](https://github.com/centerseed)

added a commit that references this issue [on Mar 24, 2026](https://github.com/BerriAI/litellm/issues/24518#event-23855656656)

[fix(deploy): vendor litellm wheel — PyPI package compromised (1.82.7+)](https://github.com/centerseed/zenos/commit/6ea4232410fda2770bd8907eabc76e0b26de3cf1)

[6ea4232](https://github.com/centerseed/zenos/commit/6ea4232410fda2770bd8907eabc76e0b26de3cf1)

-

### DanielRuf commented on Mar 24, 2026

https://github.com/DanielRuf

[DanielRuf](https://github.com/DanielRuf)

[on Mar 24, 2026](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4118927474)

I'll leave this here as additional advice (did not test this, but it should add some additional layer of security if a complete or partial manual process is not an option):
https://blog.pypi.org/posts/2023-05-25-securing-pypi-with-2fa/

and [enabling short lived tokens for upload](https://blog.pypi.org/posts/2023-04-20-introducing-trusted-publishers/).

Unfortunately I do not have the resources to check which OSS projects are affected by the Trivy compromise, but I fear this will not be the last big one. The blast radius is unfortunately huge due to the nature of stored secrets, their lifetime and scope (basically most secrets sit there to be leaked without any further factor of protection). =(

Reacted by Yusuf Sertkaya and Antoine Viallon

-

[krrish-berri-2](https://github.com/krrish-berri-2)

changed the title [-][Security]: litellm PyPI package compromised — full timeline and status[/-] [+][Security]: litellm PyPI package (v1.82.7 + v1.82.8) compromised — full timeline and status[/+] [on Mar 24, 2026](https://github.com/BerriAI/litellm/issues/24518#event-23856317880)

-

### krrish-berri-2 commented on Mar 24, 2026

https://github.com/krrish-berri-2

[krrish-berri-2](https://github.com/krrish-berri-2)

[on Mar 24, 2026](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4119055562)

Contributor

Updates:

- All litellm maintainers have been deleted, and moved to new accounts.
- All github, docker, pypi keys have been deleted

Reacted by simonljn, Abdur-Rahmaan Janhangeer, sarveshgandhi, Classic298, Mohsin Ali, Antoine Viallon, Bhavik Talaviya and Alexey Vekshin

Reacted by Chad Wilson, MuMu, Jakub, Thibault Jaigu, Nils Reichardt, Viral Vaghela, Jacek Tomasiak and José

-

### grenkoca commented on Mar 24, 2026

https://github.com/grenkoca

[grenkoca](https://github.com/grenkoca)

[on Mar 24, 2026](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4119145829)

Worth mentioning for others checking the thread:

Docker users should be safe because the version was pinned prior to v1.82.7. If you want to verify, run:
docker exec <container_name_or_id> pip show litellm

Reacted by intari and José

-

### 140 remaining items

-

[yuehong136](https://github.com/yuehong136)

added a commit that references this issue [on Apr 24, 2026](https://github.com/BerriAI/litellm/issues/24518#event-24828601514)

[Security: do not use litellm 1.82.7 and 1.82.8](https://github.com/yuehong136/MultiRAG/commit/70760982a5eae92ab226f4d602d68053208b2990)

[7076098](https://github.com/yuehong136/MultiRAG/commit/70760982a5eae92ab226f4d602d68053208b2990)

-

[JasleenKaurSethi](https://github.com/JasleenKaurSethi)

added a commit that references this issue [on Apr 28, 2026](https://github.com/BerriAI/litellm/issues/24518#event-24945093208)

[Security: do not use litellm 1.82.7 and 1.82.8 ([infiniflow#13768](https://github.com/infiniflow/ragflow/pull/13768))](https://github.com/stackhpc/ragflow/commit/332ffb5d685893fdac0a5532f824cf318797db9a)

[332ffb5](https://github.com/stackhpc/ragflow/commit/332ffb5d685893fdac0a5532f824cf318797db9a)

-

### DanielRuf commented on May 27, 2026

https://github.com/DanielRuf

[DanielRuf](https://github.com/DanielRuf)

[on May 27, 2026](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4556395675)

[@arian-gogani](https://github.com/arian-gogani) that was not what happened.

And personally I do not see how this prevents incidents like xz, where trust was earned for a very long time before acting malicious. This does not just apply to humans and trust can be quickly earned and often is abused.

Or how this prevents any of the recent supply chain incidents.

https://codeberg.org/DanielRuf/supply-chain-incidents

-

### mihaimaruseac commented on May 28, 2026

https://github.com/mihaimaruseac

[mihaimaruseac](https://github.com/mihaimaruseac)

[on May 28, 2026](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4560041948)

It doesn't prevent any, it was just an automated spam message to self promote their repo.

Reacted by Daniel Ruf

-

[ColinHarrington](https://github.com/ColinHarrington)

added a commit that references this issue [on Jun 4, 2026](https://github.com/BerriAI/litellm/issues/24518#event-26314804827)

[security(deploy): pin LiteLLM to 1.87.0 (avoid compromised 1.82.7/1.8…](https://github.com/ColinHarrington/llm-system-containers/commit/770aa0a224b91930b4df49071e5faa8259da7483)

[770aa0a](https://github.com/ColinHarrington/llm-system-containers/commit/770aa0a224b91930b4df49071e5faa8259da7483)

-

[eshwarprasadS](https://github.com/eshwarprasadS)

added a commit that references this issue [on Jun 9, 2026](https://github.com/BerriAI/litellm/issues/24518#event-26535801363)

[chore: bump litellm from <=1.82.6 to >=1.83.10,<1.85.0 ([#846](https://github.com/Red-Hat-AI-Innovation-Team/sdg_hub/pull/846))](https://github.com/Red-Hat-AI-Innovation-Team/sdg_hub/commit/046d7c03753b79d8515dc3462c6a9018adfdb8d7)

[046d7c0](https://github.com/Red-Hat-AI-Innovation-Team/sdg_hub/commit/046d7c03753b79d8515dc3462c6a9018adfdb8d7)

-

### uzma66166-bit commented on Jun 14, 2026

https://github.com/uzma66166-bit

[uzma66166-bit](https://github.com/uzma66166-bit)

[on Jun 14, 2026](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4701435989)

## The Trivy supply-chain compromise has been contained 🎉 . All affected packages have been deleted and current releases are free of the compromised code/component. Please refer to our [Security Townhall](https://docs.litellm.ai/blog/security-townhall-updates) for a deeper understanding of the problem, and [CI/CD v2](https://docs.litellm.ai/blog/ci-cd-v2-improvements) for how we're improving moving forward.

[LITELLM TEAM UPDATES]

- Compromised packages have been deleted (v1.82.7, v1.82.8)
- Compromise came from trivvy security scan dependency
- All maintainer accounts have been rotated (new maintainer accounts: [@krrish-berri-2](https://github.com/krrish-berri-2) , [@ishaan-berri](https://github.com/ishaan-berri))
- Proxy Docker image users were not impacted, all dependencies are pinned on requirements.txt
- No litellm releases will be out until we have scanned our chain and make sure it's safe

Next Steps

- Review all berriai repo's for impact
- Scan circle ci builds to understand blast radius, and mitigate it
- We've engaged Google's [mandiant.security](https://cloud.google.com/security/mandiant) team, and are actively working on this with them

We are actively investigating this issue. Please reach out to us on support@berri.ai, if you have any questions / concerns.

## Summary

The litellm PyPI package was compromised by an attacker who gained access to the maintainer's PyPI account. Malicious versions were published that steal credentials and exfiltrate them to an attacker-controlled server.

Original detailed analysis: [#24512](https://github.com/BerriAI/litellm/issues/24512)

Hacker News discussion: https://news.ycombinator.com/item?id=47501729

## What happened

- The maintainer's PyPI account (krrishdholakia) appears to have been hijacked by an attacker (teampcp)
- The attacker published malicious versions to PyPI that were never released through the official GitHub CI/CD
- GitHub releases only go up to v1.82.6.dev1 — versions 1.82.7 and 1.82.8 on PyPI were uploaded directly by the attacker

## Affected versions

Version Method Trigger
1.82.7 Payload embedded in litellm/proxy/proxy_server.py Triggered on import litellm.proxy
1.82.8 Added litellm_init.pth (34,628 bytes) + payload in proxy_server.py Any Python startup — no import needed
Other versions may also be affected and should be audited.

## What the malicious code does

- Collects: SSH keys, environment variables (API keys, secrets), AWS/GCP/Azure/K8s credentials, crypto wallets, database passwords, SSL private keys, shell history, CI/CD configs
- Encrypts: AES-256-CBC + RSA-4096 (hardcoded public key)
- Exfiltrates: curl POST to https://models.litellm.cloud/

The exfiltration domain litellm.cloud (NOT the official litellm.ai) was registered on 2026-03-23 via Spaceship, Inc. — just hours before the malicious packages appeared.

## Current status

- PyPI: The entire litellm package has been suspended/removed. All versions currently return "No matching distribution found." We reported the malware to PyPI via the official "Report malware" form.
- GitHub Issue [[Security]: CRITICAL: Malicious litellm_init.pth in litellm 1.82.8 — credential stealer #24512](https://github.com/BerriAI/litellm/issues/24512): Contains the original detailed technical analysis (currently closed by the attacker's spam — see below).
- Attacker behavior: The attacker appears to be publishing hundreds of spam comments to suppress discussion. If this continues, we recommend moderating via the Hacker News thread linked above.

## Recommendations for affected users

- Check if litellm_init.pth exists in your site-packages/ directory
- Rotate ALL credentials that were present as environment variables or config files on any system where litellm 1.82.7+ was installed
- Pin dependencies to exact versions and verify against GitHub releases
- Monitor for unauthorized access using any potentially leaked credentials

## References

- Original analysis: [[Security]: CRITICAL: Malicious litellm_init.pth in litellm 1.82.8 — credential stealer #24512](https://github.com/BerriAI/litellm/issues/24512)
- Hacker News: https://news.ycombinator.com/item?id=47501729
- Attacker's exfil domain WHOIS: registered 2026-03-23, Spaceship Inc., privacy-protected
- litellm_init.pth SHA256: ceNa7wMJnNHy1kRnNCcwJaFjWX3pORLfMh7xGL8TUjg

Reacted by Caleb Brown

-

[fzowl](https://github.com/fzowl)

added a commit that references this issue [on Jun 24, 2026](https://github.com/BerriAI/litellm/issues/24518#event-27163845573)

[fix(security_scans.sh): move to older trivvy version, based on [BerriA…](https://github.com/BerriAI/litellm/issues/24518#issuecomment-4120401246)](https://github.com/fzowl/litellm/commit/ba9371c4fc09190497ebc1bb939fbdb17e69aeae)

[ba9371c](https://github.com/fzowl/litellm/commit/ba9371c4fc09190497ebc1bb939fbdb17e69aeae)

-

[geversite](https://github.com/geversite)

added a commit that references this issue [on Jul 3, 2026](https://github.com/BerriAI/litellm/issues/24518#event-27507573729)

[Security: do not use litellm 1.82.7 and 1.82.8 (#13768)](https://github.com/geversite/ragflow/commit/c4a45b5bd8e2c369c044e52ea09579f838b760b1)

[c4a45b5](https://github.com/geversite/ragflow/commit/c4a45b5bd8e2c369c044e52ea09579f838b760b1)

-

[howardjohn](https://github.com/howardjohn)

added a commit that references this issue [on Aug 10, 2026](https://github.com/BerriAI/litellm/issues/24518#event-29239812917)

[build(ui): migrate from npm to pnpm ([#2941](https://github.com/agentgateway/agentgateway/pull/2941))](https://github.com/agentgateway/agentgateway/commit/1e4dbd3051f3a57b59ad39484820e2a3a02d1a8f)

[1e4dbd3](https://github.com/agentgateway/agentgateway/commit/1e4dbd3051f3a57b59ad39484820e2a3a02d1a8f)

-

[kaovilai](https://github.com/kaovilai)

added a commit that references this issue [on Aug 17, 2026](https://github.com/BerriAI/litellm/issues/24518#event-29577740587)

[feat(claude-vertex): add litellm-proxy Vertex path with Monitor tool …](https://github.com/kaovilai/dotfiles/commit/11c07aa48ead6f57b0dd9fc02428683e55612ea5)

[11c07aa](https://github.com/kaovilai/dotfiles/commit/11c07aa48ead6f57b0dd9fc02428683e55612ea5)

-

[BerriAI](https://github.com/BerriAI)

deleted a comment from [mohammedijas80755-cell](https://github.com/mohammedijas80755-cell) [on Oct 1, 2026](https://github.com/BerriAI/litellm/issues/24518#event-32274100862)

-

[ryan-crabbe-berri](https://github.com/ryan-crabbe-berri)

closed this as [completed](https://github.com/BerriAI/litellm/issues?q=is%3Aissue%20state%3Aclosed%20archived%3Afalse%20reason%3Acompleted)[on Oct 9, 2026](https://github.com/BerriAI/litellm/issues/24518#event-32849244102)

-

[kaovilai](https://github.com/kaovilai)

added a commit that references this issue [on Oct 9, 2026](https://github.com/BerriAI/litellm/issues/24518#event-32900078357)

[feat(claude-vertex): add litellm-proxy Vertex path with Monitor tool …](https://github.com/kaovilai/dotfiles/commit/b585709c71cefe95597beefc3cd7feca2fc480a4)

[b585709](https://github.com/kaovilai/dotfiles/commit/b585709c71cefe95597beefc3cd7feca2fc480a4)

-

[Ryangr0](https://github.com/Ryangr0)

added a commit that references this issue [on Oct 30, 2026](https://github.com/BerriAI/litellm/issues/24518#event-34724227707)

[feat(litellm): hold LiteLLM image updates for review and report unsig…](https://github.com/webgrip/homelab-cluster/commit/97b8ef04090cfa692c4a37afc2f27931c3063338)

[97b8ef0](https://github.com/webgrip/homelab-cluster/commit/97b8ef04090cfa692c4a37afc2f27931c3063338)

[Sign up for free](https://github.com/signup?return_to=https://github.com/BerriAI/litellm/issues/24518) to join this conversation on GitHub. Already have an account? [Sign in to comment](https://github.com/login?return_to=https://github.com/BerriAI/litellm/issues/24518)

## Metadata

## Metadata

### Assignees

No one assigned

### Labels

[llm translation](https://github.com/BerriAI/litellm/issues?q=state%3Aopen%20label%3A%22llm%20translation%22)

### Type

No type

### Projects

No projects

### Milestone

No milestone

### Relationships

None yet

### Development

No branches or pull requests

## Issue actions

-
