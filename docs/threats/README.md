# Bigman threat map

> Defensive catalogue of threats to Linux desktop users, developers and open-source maintainers, with how Bigman covers each one today and what it should add. Last researched: 2026-10-29.

Each file lists threats with a stable ID, a plain-language description, what a defender inspects, Bigman's current coverage, a proposed deterministic check, a benign test-fixture idea, and references. The files describe what to audit and how to detect or mitigate. They intentionally contain no attack procedures.

The prioritised plan built from these files is in [research/roadmap.md](../research/roadmap.md).

## Files

| File | Prefix | Entries | Covered | Partial | Gap |
|---|---|---|---|---|---|
| [01 Phishing and social engineering](./01-phishing-social-engineering.md) | PH- | 21 | 1 | 11 | 9 |
| [02 Package registry supply chain](./02-supply-chain-packages.md) | SC- | 23 | 0 | 6 | 17 |
| [03 Distro packaging and install channels](./03-distro-packaging-and-install-channels.md) | DP- | 23 | 0 | 3 | 20 |
| [04 Host integrity and hardening audit](./04-host-integrity-and-hardening-audit.md) | HI- | 25 | 0 | 9 | 16 |
| [05 Malware families and detection](./05-malware-families-and-detection.md) | MW- | 24 | 0 | 14 | 10 |
| [06 Credential and secret protection](./06-credential-and-secret-protection.md) | CR- | 22 | 0 | 4 | 18 |
| [07 Network exposure and remote access](./07-network-exposure-and-remote-access.md) | NE- | 22 | 0 | 5 | 17 |
| [08 Desktop and sandboxing](./08-desktop-and-sandboxing.md) | DS- | 21 | 0 | 7 | 14 |
| **Total** | | **181** | **1** | **59** | **121** |

"Partial" usually means an existing workflow (ClamAV scans, `bigman inspect`, the startup baseline, the canary or the browser extension) catches some indicators but not the threat as a whole.

## Supporting research

- [Linux antivirus landscape and papers](../research/linux-av-landscape-and-papers.md): 41 tools and products with licences and integrate/borrow/avoid advice, 28 papers, and Bigman's market position.
- [Vulnerability intelligence and feeds](../research/vulnerability-intelligence-and-feeds.md): distro advisory data sources, version-comparison rules, the `bigman advisories` design, and the weekly threat-intel review prompt.
- [feeds.opml](../research/feeds.opml): 45 RSS/Atom feeds, each fetched and confirmed working on 2026-10-29. Import it into any feed reader.

## Themes that recur across files

- **Package attribution.** Most host checks become far less noisy once each changed file is checked against the rpm/dpkg/pacman database. See HI-01, HI-22, DP-21, MW-08 and MW-15.
- **Distro-correct advisories.** "Update X because of CVE Y" must use distro trackers, because distros backport fixes without changing upstream version numbers. See DP-20, HI-23, NE-04, DS-04 and MW-20.
- **Rootless limits.** Sudoers, `/etc/shadow` status, root cron spools, polkit `/etc` rules and `sshd -T` are unreadable without privilege. Several files propose an optional, user-approved, read-only privileged helper.
- **Developer-ecosystem focus.** Maintainers are targeted through registry phishing, worms that steal publish tokens, repo-open triggers and AI-tool configs. See PH-02, PH-05, SC-06, SC-20, SC-21 and CR-02.
- **Licensing.** Bigman is AGPL-3.0-only. GPL-2.0-only sources (ClamAV, OSSEC, Wazuh, AIDE, Tripwire, linux-audit) and Elastic License 2.0 rules cannot be copied in and must be reimplemented or downloaded at the user's request.

## Verification status

Agents verified most URLs, CVEs and dates against primary sources. Each file ends with the items they could not verify, mostly 2026 incidents known only from press coverage, some tool licences and some paper links. The agents' shared web-search budget ran out partway through, so those items deserve a second pass before they drive product decisions.
