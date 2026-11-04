# Distribution packaging and software install channels

> Scope: risks in the channels Linux desktop users and maintainers use to get software. That covers distro package managers (apt/dpkg, dnf/rpm, pacman, zypper), user and third-party repositories (AUR, Ubuntu PPAs, Fedora COPR, openSUSE OBS home projects, vendor apt/yum repos), standalone `.deb`/`.rpm` downloads, app stores and bundle formats (Snap Store, Flatpak/Flathub, AppImage), Homebrew on Linux, Nix, and `curl | bash` installers. It also covers repository signing and key scoping, mirror and metadata attacks, distro infrastructure incidents, root-run maintainer scripts, automatic-update health, installed-file verification, and Secure Boot/MOK at a high level. Language registries (npm, PyPI, crates, CI actions, IDE extensions) belong to [supply-chain](./02-supply-chain-packages.md). Host-wide baselining of persistence locations is in [host integrity](./04-host-integrity-and-hardening-audit.md), and sandbox internals are in [desktop and sandboxing](./08-desktop-and-sandboxing.md). Everything is written from the defender's side: what to audit and how Bigman could detect it read-only. Last researched: 2026-10-29.

## Summary table

| ID | Attack | Severity for a Linux desktop/dev user | Bigman today | Proposed |
|---|---|---|---|---|
| DP-01 | Upstream maintainer takeover and backdoored release (xz-utils, CVE-2024-3094) | High | Gap | Feed-driven "known-bad package version" advisory, plus a tarball-vs-tag diff helper for maintainers |
| DP-02 | Unsigned, HTTP or signature-disabled repositories (`[trusted=yes]`, `gpgcheck=0`, `SigLevel = Never`) | High | Covered | `bigman repos` audit over apt/dnf/zypper/pacman/flatpak config |
| DP-03 | Third-party repo key in the global keyring (no `signed-by` scoping) | Medium | Covered | Flag keys in `trusted.gpg`/`trusted.gpg.d` and sources without `signed-by` |
| DP-04 | Package-manager verification and parser bugs (apt, rpm, pacman, dpkg CVEs) | Medium (historic, patched) | Gap | Version check of apt/dpkg/rpm/pacman against advisories |
| DP-05 | Freeze, rollback, replay and malicious-mirror attacks | Medium | Partial | Flag disabled `Valid-Until` checks, stale metadata, plain-HTTP mirrors |
| DP-06 | Distribution infrastructure or signing-host compromise | High (rare) | Gap | Feed-driven advisories; key fingerprint inventory with change alerts |
| DP-07 | Tampered ISO or installer on an official download site | High | Partial (ClamAV scan of the file) | Checksum plus signature verification helper for ISOs and downloads |
| DP-08 | Fake or trojanized vendor `.deb`/`.rpm` downloads | High | Partial | Static `.deb`/`.rpm` inspection before install |
| DP-09 | Root-run maintainer scripts and RPM scriptlets | High | Partial | Extract and lint `preinst/postinst/%post/.install` without installing |
| DP-10 | Abandoned third-party repos whose domains expire | High | Partial | Repo domain inventory; flag NXDOMAIN, unreachable or newly re-registered hosts |
| DP-11 | Malicious or hijacked AUR packages (PKGBUILD and orphan adoption) | High on Arch | Partial | PKGBUILD/`.install` static inspection; AUR-package inventory with feed match |
| DP-12 | Personal repos: PPA, COPR, OBS home projects | Medium | Covered | Inventory personal repos and warn when they replace distro packages |
| DP-13 | Scam and impersonation apps in app stores (Snap Store wallets, miners) | High for crypto users | Gap | Snap/Flatpak inventory with publisher and verification status |
| DP-14 | Store publisher account takeover via expired publisher domain | Medium | Gap | Highlight long-dormant snaps that suddenly update; feed-driven removal notices |
| DP-15 | Flatpak permission over-breadth and unverified Flathub apps | Medium | Gap | Permission inventory (`filesystem=host`, `home`, session/system bus, `devices=all`) |
| DP-16 | Snap classic/devmode confinement and snapd/Flatpak sandbox-escape CVEs | Medium | Gap | List classic/devmode snaps; version check snapd/flatpak against advisories |
| DP-17 | AppImage: unsigned, unsandboxed, self-updating executables | Medium | Partial | AppImage header and embedded-signature inspection, desktop-integration audit |
| DP-18 | `curl \| bash` and other pipe-to-shell installers | High | Partial (`bigman inspect`, shell.* rules, sandbox rehearsal) | Detect repo/key additions inside installers (`installer.adds-repo`) |
| DP-19 | Homebrew on Linux and Nix: user-writable prefixes, executable formulae, store bugs | Medium | Gap | Inventory brew/nix installs; version check against advisories |
| DP-20 | Stale systems: missing security updates or broken unattended upgrades | High | Partial | Pending-security-update report and auto-update health check |
| DP-21 | Installed files altered after install (package DB mismatch) | High | Gap | Wrap `dpkg --verify`/`debsums`, `rpm -Va`, `pacman -Qkk` with noise filtering |
| DP-22 | Third-party repo overriding distro packages (pinning, priority, epoch) | Medium | Partial | Flag high `Pin-Priority`, dnf `priority`, and installed packages whose origin is not the distro |
| DP-23 | Boot chain and kernel modules: Secure Boot off, MOK enrollment, unsigned modules | Medium | Partial (Secure Boot state) | Read-only Secure Boot/MOK/taint report |

## Attacks

### DP-01 Upstream maintainer takeover and backdoored release (xz-utils, CVE-2024-3094)
- **What it is:** An attacker spends years earning maintainer trust in a small upstream project, then ships a release whose tarball contains build logic that is not in the public git history. Distributions build from that tarball, so the backdoor reaches signed distro packages. Every downstream signature is valid because the distro faithfully packaged what upstream released. Repository signing, mirror security and package verification cannot catch it.
- **Real-world examples:**
  - **2024, xz-utils 5.6.0/5.6.1 (CVE-2024-3094).**
    - *Social engineering:* According to Russ Cox's timeline, "Jia Tan" began contributing in late 2021. Sock-puppet accounts then pressured the burnt-out sole maintainer during 2022 to hand over more control. Jia Tan had commit access by late 2022, and the payload was merged in February 2024, hidden in binary test fixtures plus a modified build script that existed only in the release tarball. Small preparatory changes along the way, such as disabling a sandbox feature check and changes around fuzzing, looked harmless on their own.
    - *Detection:* Andres Freund was investigating slow SSH logins and valgrind errors on Debian sid. He measured failed logins taking about 0.8 s instead of 0.3 s and traced the cause to liblzma. He reported it on oss-security on 2024-03-29. The backdoor targeted x86-64 glibc systems whose sshd links libsystemd (and through it liblzma) via distro patches, and it activated only in deb/rpm package builds.
    - *Exposure:* Mostly pre-release distributions: Fedora Rawhide and 41 pre-release, Debian testing/unstable, openSUSE Tumbleweed, Kali, and Arch's packaged tarball.
    - *What defenders checked afterwards:*
      1. The installed xz/liblzma version (`xz --version`, `dpkg -l xz-utils liblzma5`, `rpm -q xz-libs`, `pacman -Q xz`).
      2. Whether sshd was linked against liblzma (`ldd "$(command -v sshd)"`).
      3. Whether any such host exposed sshd to the internet while running the bad version.
      4. Distro advisories to confirm the downgrade or rebuild had been applied.
    - CISA advised downgrading to an uncompromised version such as 5.4.6, hunting for malicious activity, and reporting findings. Freund's advice was to treat potentially exposed systems as needing investigation, not just patching.
- **What a defender inspects (indicators):**
  - Installed versions of packages named in active advisories (`dpkg-query -W`, `rpm -qa`, `pacman -Q`) matched against a feed of known-bad versions.
  - Whether long-running network daemons link unexpected libraries (`ldd`, `/proc/<pid>/maps` for one's own processes).
  - For maintainers of their own projects: release tarballs that contain files absent from the tagged git tree, especially build scripts (`m4/`, `configure`), and binary test fixtures with no provenance.
  - Unexpected CPU or latency changes in sshd after an update.
- **Bigman today:** Gap. `bigman audit osv DIR` scans project lockfiles, not installed OS packages.
- **Proposed mitigation / detection:**
  - Feed-driven advisory (CLI check plus monitor): ingest distro security trackers and OSV ecosystems for Debian, Ubuntu, Fedora/Red Hat, Arch, Alpine and openSUSE. Match against the installed package list and emit `pkg.known-bad-version` (backdoor, critical) or `pkg.security-update` (ordinary CVE). This is deterministic version comparison using distro version semantics (dpkg `--compare-versions` rules, rpmvercmp, vercmp). Rootless.
  - Maintainer helper `bigman release-diff TARBALL GIT_TAG`: list files present in the tarball but not in the tag, and flag generated build files and binary blobs under `tests/`. Read-only.
  - Link to [vulnerability feeds](../research/vulnerability-intelligence-and-feeds.md) for ingesting oss-security and distro DSA/USN/FEDORA advisories.
- **Test case idea:**
  - Fixture installed-package list (a fake `dpkg status` file in a temp root) containing `xz-utils 5.6.1-1` plus a fixture advisory JSON marking `>=5.6.0 <5.6.2` as malicious. Assert `pkg.known-bad-version`.
  - For release-diff, build a tarball in a temp dir containing `m4/extra.m4` absent from a temp git repo tag, and assert a finding.
- **References:**
  - https://www.openwall.com/lists/oss-security/2024/03/29/4
  - https://research.swtch.com/xz-timeline
  - https://www.cisa.gov/news-events/alerts/2024/03/29/reported-supply-chain-compromise-affecting-xz-utils-data-compression-library-cve-2024-3094
  - https://snyk.io/blog/the-xz-backdoor-cve-2024-3094/
  - MITRE ATT&CK T1195.001 (Compromise Software Dependencies and Development Tools), T1195.002 (Compromise Software Supply Chain): https://attack.mitre.org/techniques/T1195/

### DP-02 Unsigned, HTTP or signature-disabled repositories
- **What it is:** Every major package manager verifies repository metadata or package signatures by default, and every one has a switch to turn that off. Vendor install guides and old blog posts often tell users to flip it "to make the error go away". A repository with signature checking disabled lets anyone who can tamper with the network path, mirror or hosting serve arbitrary root-installed packages.
- **Real-world examples:** Debian's `apt-secure(8)` documents `Acquire::AllowInsecureRepositories`, per-source `allow-insecure=yes` and `trusted=yes`, says insecure repositories are "strongly discouraged", and notes the options will eventually be removed. OpenSCAP/ComplianceAsCode and CIS benchmarks have long carried "ensure gpgcheck is globally activated" rules for RHEL/Fedora. Pacman's CVE-2019-18182 was exploitable only when unsigned databases were in use (see DP-04), which shows how a disabled check turns a parser bug into code execution.
- **What a defender inspects (indicators):**
  - **apt:**
    - `/etc/apt/sources.list` and `/etc/apt/sources.list.d/*.list`: options `[trusted=yes]`, `[allow-insecure=yes]`, `[allow-downgrade-to-insecure=yes]`, `http://` URIs to non-distro hosts.
    - `/etc/apt/sources.list.d/*.sources` (deb822): `Trusted: yes`, `Allow-Insecure: yes`.
    - `/etc/apt/apt.conf.d/*`: `Acquire::AllowInsecureRepositories`, `Acquire::AllowDowngradeToInsecureRepositories`, `APT::Get::AllowUnauthenticated`.
  - **dnf/yum:** `/etc/yum.repos.d/*.repo` and `/etc/dnf/dnf.conf` with `gpgcheck=0`, `repo_gpgcheck=0` on third-party repos, `sslverify=0`, `localpkg_gpgcheck=0`, or `baseurl=http://`.
  - **zypper:** `/etc/zypp/repos.d/*.repo` with `gpgcheck=0`, `pkg_gpgcheck=0`, `repo_gpgcheck=0`. `/etc/zypp/zypp.conf` with `gpgcheck = off`.
  - **pacman:** `/etc/pacman.conf` with `SigLevel = Never`, or `Optional TrustAll`, globally or per repo.
  - **Flatpak:** `flatpak remotes --show-details` and `/var/lib/flatpak/repo/config` / `~/.local/share/flatpak/repo/config` with `gpg-verify=false` or `gpg-verify-summary=false`.
  - Read-only commands: `grep -rnE 'trusted=yes|allow-insecure' /etc/apt/`, `grep -rn 'gpgcheck *= *0' /etc/yum.repos.d /etc/zypp/repos.d`, `grep -n SigLevel /etc/pacman.conf`, `flatpak remotes --show-details`.
- **Bigman today:** Covered. `bigman package-sources` (Protect > Updates) reports `channel.signature-disabled` and `channel.plain-http` for apt, dnf, zypper, pacman, Flatpak and Nix, from configuration only. Package files that would add a source are reported before install (`pkg.adds-source`).
- **Proposed mitigation / detection:** New CLI check `bigman repos`, also called from `bigman doctor`. It is a pure parser of the files above, rootless because they are world-readable. Rules:
  - `repo.signature-disabled` (High)
  - `repo.plain-http` (Medium; lower if signed, since apt and dnf still verify signatures over HTTP, but metadata freshness and privacy suffer)
  - `repo.tls-verify-disabled` (High)

  Each finding names the file, line and the safer setting. The monitor snapshots these files with the existing `bigman changes` machinery, so a new or edited repo shows as a startup-style change.
- **Test case idea:** Temp root with `etc/apt/sources.list.d/test.list` containing `deb [trusted=yes] http://repo.invalid/ stable main`, a `etc/yum.repos.d/x.repo` with `gpgcheck=0`, and a `pacman.conf` with `SigLevel = Never`. Assert three findings with line numbers, and no findings for a clean fixture.
- **References:**
  - https://manpages.debian.org/bookworm/apt/apt-secure.8.en.html
  - https://manpages.debian.org/bookworm/apt/sources.list.5.en.html
  - https://dnf.readthedocs.io/en/latest/conf_ref.html
  - https://wiki.archlinux.org/title/Pacman/Package_signing
  - https://github.com/ComplianceAsCode/content
  - MITRE T1553 (Subvert Trust Controls): https://attack.mitre.org/techniques/T1553/

### DP-03 Third-party repo key in the global keyring (no `signed-by` scoping)
- **What it is:** On apt systems, a key placed in `/etc/apt/trusted.gpg` or `/etc/apt/trusted.gpg.d/` is trusted for every configured repository, not just the vendor that supplied it. A compromised or careless third-party vendor key can therefore sign a fake `Release` file for any other source the attacker can reach, including the distro mirrors. The modern practice is a dedicated keyring file referenced with `signed-by=` (or `Signed-By:` in deb822), which limits each key to its own repo. `apt-key` is deprecated for this reason. RPM has no per-repo key scoping once a key is imported into the rpmdb, so imported third-party keys deserve inventory there too.
- **Real-world examples:** Debian's third-party repository guidance (wiki.debian.org/DebianRepository/UseThirdParty) recommends per-repo keyrings under `/usr/share/keyrings` or `/etc/apt/keyrings` referenced via `signed-by`. Ubuntu 22.04+ and Debian 12 print `apt-key is deprecated` warnings. Many vendor install scripts still pipe keys into `apt-key add` or drop them into `trusted.gpg.d`.
- **What a defender inspects (indicators):**
  - Presence and contents of `/etc/apt/trusted.gpg` (legacy global keyring).
  - Non-distro keys in `/etc/apt/trusted.gpg.d/`.
  - Sources without `signed-by`.
  - `signed-by` pointing at a file writable by a non-root user.
  - Expired keys, and keys using weak algorithms (1024-bit DSA/RSA).
  - On RPM systems, the list of imported keys: `rpm -q gpg-pubkey --qf '%{NAME}-%{VERSION}-%{RELEASE}\t%{SUMMARY}\n'`.
  - Read-only commands: `ls -l /etc/apt/trusted.gpg* /etc/apt/keyrings /usr/share/keyrings`, `gpg --show-keys --with-fingerprint FILE`, `sq inspect FILE`.
- **Bigman today:** Covered. `channel.unscoped-key` (third-party apt source without signed-by) and `channel.global-key` (third-party key in the global keyrings); package files that ship or import a key are reported before install (`pkg.adds-key`).
- **Proposed mitigation / detection:** In `bigman repos`:
  - `repo.global-key` (Medium): a key in a global keyring that does not belong to the distro archive keyring package. Ownership is resolved with `dpkg -S`.
  - `repo.unscoped-source` (Medium): a source with no `signed-by`.
  - `repo.key-weak-or-expired` (Low/Medium). Parse OpenPGP packets in Rust (for example with `sequoia-openpgp`, LGPL-2.0+) instead of shelling out.
  - `repo.key-writable` (High): a keyring that is group- or world-writable.

  Keep a fingerprint inventory in the baseline so that a new or changed key is reported as a change.
- **Test case idea:** Generate a throwaway OpenPGP key in the test, write it into a temp `etc/apt/trusted.gpg.d/vendor.gpg`, add a source line without `signed-by`, and assert both findings. Chmod a keyring 0666 to trigger `repo.key-writable`.
- **References:**
  - https://wiki.debian.org/DebianRepository/UseThirdParty
  - https://manpages.debian.org/bookworm/apt/apt-key.8.en.html
  - https://michael-prokop.at/blog/?p=5545
  - MITRE T1553: https://attack.mitre.org/techniques/T1553/

### DP-04 Package-manager verification and parser bugs
- **What it is:** The code that downloads and verifies packages runs as root and parses attacker-reachable input: HTTP responses, signature files and archive formats. Bugs there can bypass signatures or corrupt state even when configuration is correct. These issues are patched, but they matter for long-lived or unmaintained installs and as a reminder that HTTPS transport is a useful second layer.
- **Real-world examples (each verified against the Debian/NVD tracker or the vendor advisory):**
  - **CVE-2016-1252** (apt, DSA-3733, 2016-12-13, found by Jann Horn): improper error handling when validating InRelease signatures let a network attacker bypass signature checks for HTTP repositories. https://security-tracker.debian.org/tracker/CVE-2016-1252
  - **CVE-2019-3462** (apt ≤1.4.8, DSA-4371, 2019-01-22, found by Max Justicz): bad sanitization of HTTP redirect fields let a man-in-the-middle inject content, potentially leading to root code execution. https://security-tracker.debian.org/tracker/CVE-2019-3462
  - **CVE-2020-27350** (apt, DSA-4808, 2020-12-09, GitHub Security Lab GHSL-2020-168/169): integer overflows in `.deb` ar/tar parsing (arfile.cc, debfile.cc, extracttar.cc), leading to out-of-bounds reads or denial of service. https://security-tracker.debian.org/tracker/CVE-2020-27350
  - **CVE-2021-20271** (rpm, 2021): a package signed by a trusted key could be modified so that it still passed signature checks, and installing it corrupted the rpmdb. https://bugzilla.redhat.com/show_bug.cgi?id=1934125
  - **CVE-2019-18182 / CVE-2019-18183** (pacman <5.2.0, ASA-201910-13): command execution via the non-default `XferCommand` with unsigned databases, and via the non-default delta feature. https://lists.archlinux.org/pipermail/arch-security/2019-October/001469.html
  - **CVE-2019-9686** (pacman <5.1.3): directory traversal in `pacman -U <url>` that bypassed signature checking (per NVD; not independently re-read here).
  - **CVE-2022-1664** (dpkg, DSA-5147, 2022, Max Justicz): directory traversal when unpacking untrusted *source* packages with `dpkg-source`. Relevant to maintainers who unpack `.dsc` files from unknown sources. https://www.debian.org/security/dsa-5147
- **What a defender inspects (indicators):**
  - Installed versions of `apt`, `dpkg`, `rpm`, `dnf`/`libdnf`, `pacman`, `zypper`/`libzypp`, `snapd`, `flatpak` compared with distro advisories.
  - Whether the distro release is still supported: `/etc/os-release` `VERSION_ID` checked against EOL dates.
  - Non-default risky options such as pacman `XferCommand`.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - The DP-01 feed-driven advisory should give extra weight to "package manager itself vulnerable" (`pkg.manager-vulnerable`, High).
  - Add `os.eol` (High) when `/etc/os-release` names a release past end of life. This needs a small curated table or the endoflife.date API, cached.
  - Rootless.
- **Test case idea:** Fixture `os-release` for Ubuntu 18.04 plus a fixture EOL table, then assert `os.eol`. Fixture package list with `apt 1.4.8` plus an advisory entry for CVE-2019-3462, then assert `pkg.manager-vulnerable`.
- **References:** the advisories above, plus https://lists.debian.org/debian-security-announce/2019/msg00010.html (DSA-4371) and https://lwn.net/Articles/709119/ (CVE-2016-1252 coverage). MITRE T1195.002.

### DP-05 Freeze, rollback, replay and malicious-mirror attacks
- **What it is:** Cappos et al. showed that signing alone does not make a package manager safe. An attacker who runs a public mirror, or sits on the network path, can serve old but validly signed metadata. That can freeze a client so it never sees security updates, roll it back to a known-vulnerable version, or serve an endless or huge response. The paper also showed how easy it was to become an official mirror for several distros. TUF (Samuel et al., CCS 2010) generalized the defenses: signed timestamps with expiry, version monotonicity, role separation, threshold signing, and survivable key compromise.
- **Real-world examples:**
  - **2008, "A Look in the Mirror: Attacks on Package Managers"** (Cappos, Samuel, Baker, Hartman, ACM CCS 2008): evaluated ten package managers. The authors obtained mirror status for several distributions with a fake administrator and showed replay and freeze attacks on clients.
  - **2010, TUF** (Samuel, Mathewson, Cappos, Dingledine, CCS 2010).
  - Debian `Release` files now carry a `Valid-Until` field for the security archive, and apt rejects expired metadata unless `Check-Valid-Until` is disabled. Many snapshot or offline-mirror guides tell users to disable that check, which re-opens freeze attacks if left in place.
- **What a defender inspects (indicators):**
  - `Acquire::Check-Valid-Until "false"` in `/etc/apt/apt.conf.d/*` and `[check-valid-until=no]` in source lines.
  - Age of `/var/lib/apt/lists/*Release` files (metadata not refreshed for weeks while the network works).
  - dnf `metadata_expire=-1` or very long values.
  - Unofficial mirrors in `mirrorlist`/`baseurl`/`/etc/pacman.d/mirrorlist` that are not on the distro's mirror list.
  - Plain-HTTP mirrors. They are signed, but a network attacker can still freeze or replay them.
- **Bigman today:** Partial. `channel.expiry-check-off`, `channel.stale-metadata` (distribution metadata older than 14 days) and `channel.plain-http`. A malicious mirror serving fresh-looking but replayed metadata is not detectable from configuration.
- **Proposed mitigation / detection:** In `bigman repos`:
  - `repo.validity-check-disabled` (Medium).
  - `repo.metadata-stale` (Medium): the newest `Date:` in cached `Release`/`InRelease` files, or the dnf/pacman cache timestamp, is more than N days old. This relies only on what is already on disk and needs no network.
  - `repo.unknown-mirror` (Low): optionally compare mirror hosts with a bundled list of official mirror domains per distro.
- **Test case idea:** Temp `var/lib/apt/lists/` with a fixture `InRelease` whose `Date:` is 60 days ago, and an `apt.conf.d` fixture with `Acquire::Check-Valid-Until "false";`. Assert both findings.
- **References:**
  - Cappos et al. 2008: https://www2.cs.arizona.edu/stork/packagemanagersecurity/ccs2008.pdf (project page https://www2.cs.arizona.edu/stork/packagemanagersecurity/attacks-on-package-managers.html)
  - Samuel et al. 2010: https://uptane.org/assets/files/samuel_ccs_2010-2e4e7a69695e936b1ad3b0b7010c550a.pdf
  - TUF: https://theupdateframework.io/security/
  - MITRE T1557 (Adversary-in-the-Middle).

### DP-06 Distribution infrastructure or signing-host compromise
- **What it is:** Distro build servers, git hosting, mailing lists and signing hosts are high-value targets. A compromise can expose signing keys, tamper with source, or sniff developer credentials. Users can't detect this locally. What a desktop tool can do is surface the distro's advisory quickly and tell the user which keys or packages to check.
- **Real-world examples:**
  - **Debian, November 2003:** master, murphy, gluck and klecker were compromised with a sniffed developer password. The archive was not affected. Announcement: https://lists.debian.org/debian-announce/2003/msg00003.html. Investigation report: https://www.debian.org/News/2003/20031202
  - **Fedora and Red Hat, August 2008:** an intruder reached Fedora infrastructure, including the host holding the signing key, though Fedora concluded the passphrase was not captured. Fedora rotated its keys (https://listman.redhat.com/archives/fedora-announce-list/2009-March/002626.html). Separately, Red Hat disclosed that an intruder had signed a small number of tampered OpenSSH packages for RHEL 4/5 and published a detection script and blacklist (RHSA-2008:0855, https://access.redhat.com/errata/RHSA-2008:0855).
  - **kernel.org, August 2011:** several servers were rooted. Git integrity was verified through SHA-1 history and developer clones (LWN, 2011-08-31: https://lwn.net/Articles/457142/). Later ESET research reportedly tied kernel.org to the Ebury SSH backdoor (secondary coverage; not re-verified here).
  - **Gentoo GitHub mirror, 2018-06-28:** an organization admin's password was guessed from a reused pattern, and repos were defaced with destructive ebuild changes. Gentoo's own rsync and signed infrastructure was unaffected. Report: https://wiki.gentoo.org/wiki/Project:Infrastructure/Incident_reports/2018-06-28_Github
  - **Arch AUR, June-August 2026:** see DP-11.
- **What a defender inspects (indicators):**
  - The archive-key fingerprints on the system (DP-03) compared with fingerprints the distro publishes.
  - Packages installed during an advisory's exposure window (`/var/log/dpkg.log`, `/var/log/apt/history.log`, `dnf history`, `/var/log/pacman.log`).
  - Whether the user cloned or pulled from a mirror (for example GitHub) instead of the canonical source.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - Feed-driven advisory component. When a distro infrastructure advisory appears, Bigman cross-references `dpkg.log`/`dnf history`/`pacman.log` for packages installed or updated inside the stated window and lists them (`pkg.installed-in-incident-window`).
  - Key-fingerprint change alerts from DP-03.
- **Test case idea:** Fixture `pacman.log` with an upgrade timestamp inside a fixture advisory window and one outside. Assert only the first is reported.
- **References:** the URLs above, plus MITRE T1195.002 and T1584 (Compromise Infrastructure).

### DP-07 Tampered ISO or installer on an official download site
- **What it is:** If attackers control the project website, they can point download links at a modified ISO and also change the checksums shown next to it. Checksums published on the same compromised page prove nothing. Only a detached signature checked against a key obtained earlier and out of band does. Users rarely verify, and ISOs install with full trust.
- **Real-world examples:** On 2016-02-20 the Linux Mint website was altered to link a backdoored Linux Mint 17.3 Cinnamon ISO. Mint told users to compare MD5 sums and to look for `/var/lib/man.cy` in the live session. Mint later moved to signed SHA256 sums. Blog post: https://blog.linuxmint.com/?p=2994 (2016-02-21).
- **What a defender inspects (indicators):** whether a downloaded ISO has a matching `SHA256SUMS` and a valid `SHA256SUMS.gpg`/`.sign` from the distro's known signing key, the key fingerprint compared with the distro's documentation, and whether the ISO came from an official mirror or a third-party host.
- **Bigman today:** Partial. `bigman scan` computes SHA-256 and runs ClamAV, but does not compare against signed checksums.
- **Proposed mitigation / detection:**
  - `bigman verify-download FILE [--sums SHA256SUMS --sig SHA256SUMS.gpg]`: compute SHA-256, find the matching line, and verify the signature against a bundled set of distro release-key fingerprints (Debian, Ubuntu, Fedora, Mint, Arch, openSUSE), pinned in the source and updated through Bigman releases.
  - Report `download.checksum-only` (Medium: checksum matched but no signature) or `download.signature-mismatch` (High).
  - The browser extension's download scan could trigger it automatically for `.iso`.
- **Test case idea:** Create a 1 KiB fake `.iso`, write a `SHA256SUMS`, sign it with a test key in a temp GNUPGHOME, and assert a pass with the test key pinned and `download.signature-mismatch` after flipping one byte.
- **References:** https://blog.linuxmint.com/?p=2994, https://www.theregister.com/2016/02/21/linux_mint_hacked_malwareinfected_isos_linked_from_official_site/, https://ubuntu.com/tutorials/how-to-verify-ubuntu, MITRE T1195.002.

### DP-08 Fake or trojanized vendor `.deb`/`.rpm` downloads
- **What it is:** Users install proprietary or niche apps by downloading a `.deb`/`.rpm` from a vendor site, a search ad, or a lookalike domain. A tampered package installs with root and runs maintainer scripts (DP-09), so a single click-through installs persistence. It can also add a vendor repo and key (DP-02/DP-03) so that later "updates" come from the attacker.
- **Real-world examples:** **2023, Free Download Manager:** Kaspersky found that FDM's site sometimes redirected Linux users to `deb.fdmpkg[.]org`. The `.deb` there had a postinst script that dropped a Bew-family backdoor (`crond`) and a credential stealer. The campaign ran for about three years (2020-2022) before being reported in September 2023. https://securelist.com/backdoored-free-download-manager-linux-malware/110465/
- **What a defender inspects (indicators):**
  - Package origin: no repo, installed from file. Read `/var/log/apt/history.log` lines with `.deb` paths and `dnf history` entries with `@commandline`.
  - Maintainer scripts that write cron jobs, systemd units or binaries outside the package's own paths.
  - Files under `/etc/cron.d`, `/etc/systemd/system` and `/usr/local` that no package owns (`dpkg -S`, `rpm -qf`).
  - The download domain compared with the vendor's real domain.
- **Bigman today:** Partial. Inspect parses `.deb` and `.rpm` files in Rust without dpkg or rpm: identity (name, version, maintainer, homepage, rpm signature presence), `pkg.install-script`, `pkg.adds-source`, `pkg.adds-key`, `pkg.setuid`, `pkg.startup-path` and `pkg.user-path`; the Downloads watch routes downloaded packages there. ClamAV and the browser link checks remain. Not yet: the `pkg.orphan-file` monitor check and vendor-domain comparison.
- **Proposed mitigation / detection:**
  - Extend `bigman inspect FILE` to recognize `.deb` (ar + control.tar.* + data.tar.*) and `.rpm` (lead + header + cpio payload) and parse them in pure Rust without running dpkg or rpm (see DP-09).
  - Report the declared package name, maintainer, homepage and version.
  - Report files installed outside `/usr` and `/opt`, setuid/setgid bits, files dropped in `/etc/apt/sources.list.d`, `/etc/yum.repos.d`, `/etc/cron*`, `/etc/systemd`, `/etc/profile.d` or `/etc/ld.so.preload`, and keyring drops (`pkg.adds-repo`, `pkg.adds-key`, `pkg.setuid`, `pkg.persistence-path`).
  - The browser extension's download hook routes `.deb`/`.rpm` to this inspection.
  - Add a `pkg.orphan-file` monitor check: files in persistence paths owned by no package (overlaps [host integrity](./04-host-integrity-and-hardening-audit.md)).
- **Test case idea:** In the test, build a minimal `.deb` with Rust (`ar` with `debian-binary`, `control.tar.gz` and `data.tar.gz`). `data.tar` holds `etc/cron.d/fixture` and `etc/apt/sources.list.d/fixture.list`, and `postinst` contains only `echo fixture`. Assert `pkg.persistence-path` and `pkg.adds-repo`. Never install it.
- **References:** Securelist FDM report above, MITRE T1195.002, T1204.002 (User Execution: Malicious File), T1036.005 (Masquerading: Match Legitimate Name or Location).

### DP-09 Root-run maintainer scripts and RPM scriptlets
- **What it is:** Linux packages can carry scripts that run as root during install, upgrade and removal. On dpkg these are `preinst`, `postinst`, `prerm`, `postrm` and `config`, plus triggers. On RPM they are `%pre`, `%post`, `%preun`, `%postun`, `%pretrans`, `%posttrans` and file triggers. On pacman they are `.install` functions and ALPM hooks. Signature checks prove who built the package, not what the scripts do, so for any non-distro package the scripts are the main place to look. All of them can be read without installing.
- **Real-world examples:** FDM 2023 (DP-08) used `postinst`. The AUR 2026 incidents (DP-11) reportedly used post-install scripts. MITRE documents this as T1546.016 (Installer Packages), noting that maintainer scripts run with root privileges.
- **What a defender inspects (indicators), all read-only and without installing:**
  - `.deb`: `dpkg-deb -I pkg.deb` (control fields), `dpkg-deb -c pkg.deb` (file list), and `dpkg-deb -e pkg.deb OUTDIR` to extract control scripts for reading.
  - `.rpm`: `rpm -qp --scripts pkg.rpm`, `rpm -qp --triggers pkg.rpm`, `rpm -qp --filetriggers pkg.rpm`, `rpm -qplv pkg.rpm`, `rpm -Kv pkg.rpm` (signature/digest status).
  - Arch: read `PKGBUILD` and the `.install` file before building. For built packages, `bsdtar -xOf pkg.pkg.tar.zst .INSTALL`, and `pacman -Qip` for metadata.
  - Already installed: `/var/lib/dpkg/info/<pkg>.{preinst,postinst,prerm,postrm}`, `rpm -q --scripts <pkg>`, `/var/lib/pacman/local/<pkg>/install`, `/usr/share/libalpm/hooks/` and `/etc/pacman.d/hooks/`.
  - Suspicious content: network fetch, `systemctl enable` of units not in the package, writing to `~` of real users, adding repos or keys, `chmod +s`, base64 or eval blobs, disabling SELinux/AppArmor, or edits to `/etc/ld.so.preload`.
- **Bigman today:** Partial. Every package transaction (dnf5 history, pacman or dpkg log) gets an install receipt with each installed package's scripts verbatim; `receipt.hook.sensitive_path`, `receipt.hook.download`, `receipt.hook.obfuscation` and `receipt.hook.changed` mark lines and changes. Receipts are built after the transaction; the pre-transaction preview hooks are opt-in from Setup. Before install, Inspect reads maintainer scripts straight from `.deb`, `.rpm` and Arch package files and PKGBUILD `.install` files (`pkg.install-script`) and applies the same `receipt.hook.*` line rules. `bigman audit installed-scripts` is not built yet.
- **Proposed mitigation / detection:**
  - Reuse the `shell.*` rule engine on extracted maintainer scripts, with new rule IDs `pkg.maintainer-script` (Info: present) and `pkg.maintainer-script.<capability>` (Review), mirroring `package.lifecycle`.
  - Add `bigman audit installed-scripts` to scan `/var/lib/dpkg/info/*.{pre,post}*`, `/var/lib/pacman/local/*/install` and RPM scripts via `rpm -qa --scripts`. List installed non-distro packages whose scripts reach network or persistence capabilities. Rootless.
  - Parsing must never invoke the package's own code. ar, tar, cpio, zstd, xz and gzip parsing stays in Rust with the existing size and depth limits.
- **Test case idea:** Fixture `.deb` and fixture `.rpm`, where the RPM is committed as a binary test asset built once with `rpmbuild` and has `%post` reading `echo fixture; systemctl enable fixture.service` as text. Assert `pkg.maintainer-script.persistence` with no execution. Use an Arch fixture `PKGBUILD` plus `fixture.install` with `post_install() { echo fixture; }`.
- **References:**
  - https://attack.mitre.org/techniques/T1546/016/
  - https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html
  - https://rpm-software-management.github.io/rpm/manual/scriptlet_expansion.html
  - https://wiki.archlinux.org/title/PKGBUILD#install
  - https://man.archlinux.org/man/alpm-hooks.5

### DP-10 Abandoned third-party repos whose domains expire
- **What it is:** A user adds a vendor or community apt/yum repo once and forgets it. Years later the project moves, the domain lapses and someone else registers it. Signing protects the user only if the repo was signed and the key was scoped (DP-02/DP-03). An unsigned or `[trusted=yes]` source, an install script that re-fetches the key, or a key that the new owner also controls (when the key was fetched from the same domain at setup time) leaves the machine installing whatever the new owner serves.
- **Real-world examples:**
  - **2013, debian-multimedia.org:** an unofficial and popular repo whose domain lapsed and was registered by a party unknown to Debian. The DPL warned users (https://lists.debian.org/debian-devel-announce/2013/06/msg00000.html).
  - **2026, Helm's community apt mirror `baltocdn.com`:** decommissioned in September 2025 and re-registered by a third party on 2026-05-19. Helm's 2026-05-29 notice says to remove such sources and treat any system that ran binaries from it after that date as potentially compromised (https://blog.helm.sh/blog/security-notice-baltocdn).
  - **2026, Snap Store:** the same pattern used against store accounts (DP-14).
- **What a defender inspects (indicators):** every host in apt/dnf/zypper/pacman/flatpak repo configs, whether it still resolves, whether it serves valid signed metadata (`apt-get update` errors such as `NO_PUBKEY`, `EXPKEYSIG`, 404 or NXDOMAIN in `/var/log/apt/term.log`), the last successful fetch time per source, and advisories naming a repo domain.
- **Bigman today:** Partial. `channel.third-party` (Info) inventories every third-party source so abandoned ones can be removed by hand. Domain lookups (NXDOMAIN, re-registration) need the network and are not done.
- **Proposed mitigation / detection:**
  - `bigman repos --online` (opt-in, because it makes DNS queries) resolves each repo host and reports `repo.host-unresolvable` (Medium) and `repo.host-changed` (Medium/High when the A/AAAA or NS records differ from the baseline).
  - Offline mode reports `repo.never-refreshed` for sources with no `/var/lib/apt/lists` entry.
  - Feed-driven: a curated `known-dead-repos.json` (for example `baltocdn.com`, `debian-multimedia.org`) gives `repo.known-abandoned` (High).
  - Optional RDAP lookup for domain creation date is out of scope by default for privacy.
- **Test case idea:** Fixture source pointing at `deb https://baltocdn.com/helm/stable/debian/ all main` plus a fixture `known-dead-repos.json`. Assert `repo.known-abandoned` without any network access. For online mode, use `*.invalid`, which is guaranteed not to resolve.
- **References:** Helm notice above, https://jfrog.com/help/r/artifactory-remediating-supply-chain-risks-from-deprecated-upstream-helm-apt-mirror-baltocdn-com, the debian-devel-announce post above, Nadji et al. on residual domain trust (BlueHat v17, slides: https://www.slideshare.net/MSbluehat/bluehat-v17-28-registrations-later-measuring-the-exploitation-of-residual-trust-in-domains), MITRE T1584.001 / T1583.001 (Domains).

### DP-11 Malicious or hijacked AUR packages (PKGBUILD and orphan adoption)
- **What it is:** The Arch User Repository hosts user-submitted build recipes (PKGBUILDs) that AUR helpers download, build and install. It is unsupported by Arch and has no pre-publication review. Attackers have uploaded lookalike packages and, at scale in 2026, adopted orphaned packages that existing users already trusted, then pushed malicious updates. AUR helpers that auto-update without showing diffs turn this into silent compromise.
- **Real-world examples:**
  - **July 2025:** `librewolf-fix-bin`, `firefox-patch-bin` and `zen-browser-patched-bin`, and later `google-chrome-stable` lookalikes, fetched a CHAOS RAT payload from an attacker-controlled GitHub repo. Arch removed them within about two days (https://www.bleepingcomputer.com/news/security/arch-linux-pulls-aur-packages-that-installed-chaos-rat-malware/).
  - **June 2026:** Arch news "Active AUR malicious packages incident" (2026-06-12, https://archlinux.org/news/active-aur-malicious-packages-incident/) told users to review all PKGBUILD and install-script changes when updating. More than 1,500 packages were affected through malicious adoptions and updates. New registrations were frozen. The affected-package list is at https://md.archlinux.org/s/SxbqukK6IA. Secondary coverage (It's FOSS, Phoronix) describes post-install scripts pulling an npm package and infostealer/rootkit behavior; payload details are not verified here.
  - **July-August 2026:** a renewed wave led Arch to disable package adoption, and briefly AUR write access (https://fossforce.com/2026/07/new-attack-puts-archs-aur-into-lockdown-again/, LWN https://lwn.net/Articles/1086489/; the LWN page was not fetched due to rate limiting).
  - Older AUR incidents (for example 2018 `acroread`) are unverified here.
- **What a defender inspects (indicators):**
  - List foreign (non-repo) packages with `pacman -Qm`.
  - For each, compare installed version and maintainer with the AUR RPC metadata and the published affected-package lists.
  - Check whether the AUR maintainer changed recently (adoption), whether the PKGBUILD `source=` points to a personal GitHub repo or paste site rather than the upstream project, and whether a `-bin` package's sources differ from the upstream's official release URLs.
  - Check the helper's configuration (for example yay/paru settings that skip diff review).
- **Bigman today:** Partial. Inspect reads a PKGBUILD as text: the installer rules on its code (metadata and `$pkgdir` housekeeping left out), `pkgbuild.skip-checksum` for non-VCS downloads with SKIP, and its install file with `pkg.install-script` and the receipt line rules; built `.pkg.tar.*` files are read like other package files. pacman `.install` scripts of installed packages are in receipts. Not yet: `pacman -Qm` matched against incident lists.
- **Proposed mitigation / detection:**
  - `bigman packages` gains a PKGBUILD mode that statically parses the `source=`, `sha256sums=`, `install=` and `pkgver` fields (no `makepkg`, no sourcing bash) and runs `shell.*` rules on `prepare/build/package` function bodies and the `.install` file.
  - New rules: `aur.source-not-upstream` (Review), `aur.skip-checksum` (`SKIP` for non-VCS sources), `aur.install-script`.
  - Inventory mode: `pacman -Qm` plus a feed of incident lists gives `aur.known-malicious` (High).
  - Optional online check: maintainer changed since install gives `aur.recent-adoption` (Review).
- **Test case idea:** Fixture PKGBUILD with `source=("https://example.invalid/patches.tar.gz")`, `sha256sums=('SKIP')` and `install=fixture.install` containing only echo. Assert the three rules. Fixture `pacman -Qm` output plus a fixture incident list gives `aur.known-malicious`.
- **References:** links above, https://wiki.archlinux.org/title/Arch_User_Repository, MITRE T1195.002, T1036.005.

### DP-12 Personal repos: Ubuntu PPAs, Fedora COPR, openSUSE OBS home projects
- **What it is:** PPAs, COPR and OBS `home:` projects let any account holder publish signed binary repos. The signature proves only that the build service built what that account uploaded. The account owner is the trust root, and once enabled the repo can ship root-run scripts and replace any system package (DP-22). Personal repos are often abandoned, so their packages stop receiving security fixes, and an account takeover reaches every user.
- **Real-world examples:** Fedora's docs say COPR is not supported by Fedora infrastructure and its packages are used "at your own risk" (https://docs.fedoraproject.org/en-US/quick-docs/third-party-software/). No widely reported malicious PPA/COPR/OBS incident was verified in this research pass, so this entry reflects documented risk, not a known campaign.
- **What a defender inspects (indicators):**
  - Ubuntu: `ppa.launchpadcontent.net` / `ppa.launchpad.net` sources in `/etc/apt/sources.list.d/`.
  - Fedora: `/etc/yum.repos.d/_copr:*.repo`.
  - openSUSE: `download.opensuse.org/repositories/home:` in `/etc/zypp/repos.d`.
  - For each: last-modified date of its metadata, which installed packages come from it (`apt-cache policy <pkg>`, `dnf repoquery --installed --qf '%{name} %{from_repo}'`, `zypper se -i -r <alias>`), and whether any of those shadow distro packages.
- **Bigman today:** Covered. `channel.personal-repo` (Info) names PPA, COPR and OBS home sources and `channel.priority-override` (Review) reports those ranked above the distribution.
- **Proposed mitigation / detection:**
  - `bigman repos` classifies sources as distro, vendor, personal or unknown using host patterns.
  - Report `repo.personal` (Info) with the list of installed packages it supplies.
  - Escalate to Review when it supplies a package also present in the distro repos or a core security-sensitive package (openssh, sudo, glibc, systemd, kernel, polkit).
- **Test case idea:** Fixture `_copr:copr.fedorainfracloud.org:user:proj.repo` plus a fixture `dnf repoquery` text output listing `openssh-server` from that repo. Assert `repo.personal` escalated.
- **References:** Fedora third-party docs above, https://fedoraproject.org/wiki/Third_Party_Repository_Policy, https://help.launchpad.net/Packaging/PPA, https://openbuildservice.org/help/manuals/obs-user-guide/, MITRE T1195.002.

### DP-13 Scam and impersonation apps in app stores
- **What it is:** Snap Store and Flathub let third parties publish under familiar app names. Automated review checks packaging, not intent. Users trust the "store" framing, so a fake "Exodus" or "Ledger Live" wallet, or a game with a hidden miner, gets installed by people who would never run a random binary.
- **Real-world examples:**
  - **May 2018:** the snaps `2048buntu` and `Hextris` mined Bytecoin, and Canonical removed all of that publisher's apps (https://lwn.net/Articles/754502/).
  - **2023-2024:** repeated fake crypto-wallet snaps. In February 2024 a user reportedly lost 9 BTC (about $490K) to a fake Exodus wallet snap (https://blog.popey.com/2024/02/exodus-bitcoin-wallet-490k-swindle/). In March 2024 Canonical temporarily required manual review of new snap name registrations (https://www.theregister.com/2024/03/28/canonical_snap_store_scams/).
  - **2026:** the campaign continued through publisher-domain hijacks (DP-14).
- **What a defender inspects (indicators):**
  - `snap list --all` (name, publisher, verified-publisher mark, notes column), `snap info <name>` (publisher, store URL, contact).
  - `flatpak list --app --columns=application,origin,version` and the Flathub page's verification status.
  - Crypto-wallet, password-manager and browser apps whose publisher is not the vendor.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - `bigman apps` inventory (rootless) over snap and flatpak.
  - Rule `store.sensitive-unverified` (Review) when an app in a sensitive category (curated keyword list: wallet, ledger, exodus, metamask, trezor, electrum, bitwarden, keepass, 1password, browser names) has a publisher without the store's verified mark.
  - Feed-driven `store.removed-app` when a store or blog advisory lists an installed snap or flatpak ID.
- **Test case idea:** Fixture `snap list` text with a row `exodus-wallet ... digisafe00000 -` (unverified) and fixture `flatpak list` output. Assert `store.sensitive-unverified` for the snap only.
- **References:** links above, https://snapcraft.io/docs/snap-store, https://docs.flathub.org/docs/for-users/verification, MITRE T1036.005, T1204.002.

### DP-14 Store publisher account takeover via expired publisher domain
- **What it is:** Store accounts are often tied to an email address on the publisher's own domain. If that domain expires, whoever re-registers it can receive password-reset mail and take over the account. That gives them a publisher with years of history and an existing user base that will auto-refresh to their next release. Snap refreshes automatically by default, so the window between malicious upload and installation is short.
- **Real-world examples:**
  - **January 2026:** Alan Pope documented domains of legitimate snap publishers (`storewise.tech`, `vagueentertainment.com`) being re-registered to take over accounts and push malicious updates (https://blog.popey.com/2026/01/malware-purveyors-taking-over-published-snap-email-domains/).
  - **2022, PyPI `ctx`:** the same mechanism on a language registry. See [supply-chain](./02-supply-chain-packages.md).
- **What a defender inspects (indicators):** snaps whose installed revision is recent but whose previous revision is very old (dormant then sudden update), `snap changes` / `snap refresh --time`, publisher contact domain from `snap info`, and refresh hold status.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - In `bigman apps`, `store.dormant-then-updated` (Review) when the gap between consecutive installed revisions exceeds N months for a sensitive-category app. Data comes from the `snap list --all` revision list and `/var/lib/snapd/state.json`, which is root-only. Fall back to `snap changes`.
  - Guidance: for wallets, prefer vendor-signed releases. Consider `snap refresh --hold` for sensitive snaps and review before refresh.
  - Feed-driven: watch the store and maintainer blogs.
- **Test case idea:** Fixture `snap list --all` output with revisions dated 2023-01 and 2026-01 for `fixture-wallet`. Assert the rule.
- **References:** Pope 2026 above, https://snapcraft.io/docs/managing-updates, MITRE T1195.002, T1584.001.

### DP-15 Flatpak permission over-breadth and unverified Flathub apps
- **What it is:** Flatpak's sandbox is only as strong as the permissions in the app's manifest. Many popular apps request `--filesystem=home` or `host`, session-bus talk rights, `--device=all` or X11. With those, a malicious or compromised app can read `~/.ssh`, browser profiles and shell rc files, much like an unsandboxed program. Flathub's "verified" badge confirms publisher identity, not code safety. Unverified apps are community repackages.
- **Real-world examples:**
  - Flatpak's own docs say session and system bus access "must be avoided, unless the application is a development tool" and advise "avoiding full home access" (https://docs.flatpak.org/en/latest/sandbox-permissions.html).
  - Flathub began prominently labeling unverified apps in 2024 (https://www.gamingonlinux.com/2024/04/flathub-now-prominently-notes-unverified-flatpak-apps).
  - Linux Mint's Software Manager hides unverified Flatpaks by default (https://www.omgubuntu.co.uk/2024/06/linux-mint-will-hide-unverified-flatpaks).
- **What a defender inspects (indicators):**
  - `flatpak info --show-permissions <app>`.
  - `/var/lib/flatpak/app/<id>/current/active/metadata` and `~/.local/share/flatpak/app/...`.
  - User overrides in `~/.local/share/flatpak/overrides/` and `/var/lib/flatpak/overrides/`.
  - Remotes and subsets with `flatpak remotes --show-details`.
  - Risky keys: `filesystems=host;`, `host-os`, `home`, `xdg-config`, `~/.ssh`, `sockets=x11`, `session-bus`, `system-bus`, `devices=all`, `talk-name=org.freedesktop.Flatpak` (allows spawning host commands), and `own-name` on sensitive names.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - `bigman apps` parses each installed Flatpak's `metadata` keyfile plus overrides (rootless) and reports `flatpak.host-filesystem` (Review), `flatpak.home-filesystem` (Info/Review), `flatpak.host-spawn` (High: `org.freedesktop.Flatpak` talk), `flatpak.x11` (Info), `flatpak.unverified` (Info, when online check enabled) and `flatpak.override-widened` (Review: a local override grants more than the manifest).
  - Point users to Flatseal or `flatpak override --user --nofilesystem=...` as guidance only. Bigman does not change it.
- **Test case idea:** Temp `share/flatpak/app/org.fixture.App/current/active/metadata` with `[Context] filesystems=host;` and `[Session Bus Policy] org.freedesktop.Flatpak=talk`. Assert two findings.
- **References:** docs above, https://docs.flathub.org/docs/for-users/verification, https://github.com/tchx84/Flatseal, see [desktop and sandboxing](./08-desktop-and-sandboxing.md).

### DP-16 Snap classic/devmode confinement and sandbox-escape CVEs
- **What it is:** Classic-confinement snaps run with no sandbox, effectively as traditional packages. Devmode snaps log violations instead of enforcing them. The privileged helpers that set up the sandboxes (snap-confine, flatpak-portal, bubblewrap) have had local root or escape bugs, so outdated snapd/flatpak versions are themselves a risk.
- **Real-world examples:**
  - Classic confinement is documented as "the most permissive level of snap confinement", and Ubuntu docs describe classic snaps as having no confinement (https://snapcraft.io/docs/classic-confinement).
  - **CVE-2021-44731** (snap-confine race, local root on default Ubuntu Desktop; Qualys "Oh Snap! More Lemmings", 2022-02-17): https://www.qualys.com/2022/02/17/cve-2021-44731/oh-snap-more-lemmings.txt
  - **CVE-2021-21261** (flatpak-portal sandbox escape, January 2021).
  - **CVE-2024-32462** (Flatpak sandbox escape via RequestBackground portal `--command` argument handling, fixed in 1.14.6/1.15.8 and backports; GHSA-phv6-cpc2-2fgj): https://security-tracker.debian.org/tracker/CVE-2024-32462
- **What a defender inspects (indicators):** `snap list` notes column (`classic`, `devmode`, `jailmode`), `snap connections <snap>` for interfaces like `system-files`, `home`, `removable-media` and `ssh-keys`, and versions of `snapd`, `flatpak`, `bubblewrap` and `xdg-desktop-portal` compared with advisories.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** `snap.classic` (Info, or Review for non-dev-tool categories), `snap.devmode` (Review: rarely legitimate on a user desktop), `snap.sensitive-interface` (Review when `ssh-keys`, `password-manager-service`, `system-files` or `kernel-module-control` is connected), and sandbox-runtime versions fed into DP-01's advisory matcher.
- **Test case idea:** Fixture `snap list` output with `notes` = `classic` and `devmode` rows, and fixture `snap connections` output with `ssh-keys` connected. Assert findings.
- **References:** links above, https://snapcraft.io/docs/snap-confinement, https://github.com/flatpak/flatpak/security/advisories, MITRE T1611 (Escape to Host), T1068 (Exploitation for Privilege Escalation).

### DP-17 AppImage: unsigned, unsandboxed, self-updating executables
- **What it is:** An AppImage is a single ELF plus squashfs that the user marks executable and runs. It has no repository, no package database, no default sandbox and usually no signature. AppImage supports an embedded GPG signature, but `--appimage-signature` only prints it, and verifying it needs the separate `validate` tool plus a key obtained out of band. Desktop integrators can add `.desktop` files and autostart entries. Some AppImages carry update information that points to an arbitrary URL.
- **Real-world examples:** The AppImage docs state that printing the signature "does not tell you whether the signature is valid or not" (https://docs.appimage.org/packaging-guide/optional/signatures.html). No specific large AppImage malware campaign was verified in this pass. AppImages are a common vehicle for "download the Linux version" lures in fake-vendor pages (see [phishing](./01-phishing-social-engineering.md)).
- **What a defender inspects (indicators):**
  - Executable files with the AppImage magic (`AI\x02` at offset 8) in `~/Downloads`, `~/Applications`, `~/.local/bin` and `/opt`.
  - Whether each carries an embedded signature section (`.sha256_sig`, `.sig_key` ELF sections) and its update-information section (`.upd_info`).
  - `.desktop` files in `~/.local/share/applications` and `~/.config/autostart` whose `Exec=` points to an AppImage.
  - Whether AppImageLauncher-style integration moved files.
- **Bigman today:** Partial. Inspect recognises AppImages (`AI\x02`) and reads their ELF sections: `appimage.unsigned` (Info) and `appimage.update-url` (Info). The squashfs contents and autostart entries pointing at AppImages are not checked yet.
- **Proposed mitigation / detection:**
  - `bigman inspect` recognizes AppImage type 2 and reports `appimage.unsigned` (Info), `appimage.update-url` (Info, shows host), and `appimage.autostart` (Review when an autostart entry targets it).
  - It lists the squashfs contents' `.desktop` and `AppRun` and runs `shell.*` rules on a shell `AppRun` (squashfs reading in Rust, or via `unsquashfs -l` under existing bwrap).
  - Extend `bigman changes` to include `~/.local/share/applications`.
- **Test case idea:** Synthetic file: minimal ELF header bytes plus `AI\x02` at offset 8 and an ELF section named `.upd_info` containing `zsync|https://example.invalid/x.zsync`. Assert type detection and `appimage.update-url`. Add a fixture autostart `.desktop` pointing at it.
- **References:** https://docs.appimage.org/packaging-guide/optional/signatures.html, https://github.com/AppImage/AppImageSpec/blob/master/draft.md, MITRE T1204.002.

### DP-18 `curl | bash` and other pipe-to-shell installers
- **What it is:** Many dev tools install by piping a remote script into a shell, often with `sudo`. The user never sees the script, and the server can tell from request patterns and timing whether output is being piped straight into a shell, so what a user inspects in a browser can differ from what executes. A connection drop mid-transfer can run a truncated script. These scripts commonly add apt/yum repos and keys (DP-02/DP-03/DP-10), which keeps the installer's trust alive long after the one-off run.
- **Real-world examples:**
  - The 2016 write-up "Detecting the use of 'curl | bash' server side" (idontplaydarts.com; cited conceptually only) concluded that untrusted streams should never be piped into a shell and that scripts should be downloaded, reviewed, then run.
  - The 2021 Codecov bash uploader compromise is a CI-side example; see [supply-chain](./02-supply-chain-packages.md).
- **What a defender inspects (indicators):** shell history for pipe-to-shell patterns (`~/.bash_history`, `~/.zsh_history`, read with consent), new files in `/etc/apt/sources.list.d` or `/etc/yum.repos.d` and new keys with timestamps matching the install, `/usr/local/bin` binaries not owned by any package, and new systemd units.
- **Bigman today:** Partial, and the strongest coverage in this domain.
  - `bigman inspect FILE|URL` and `curl ... | bigman` fetch with DNS pinning and size limits, never execute, and apply `shell.download`, `shell.execute-download`, `shell.privilege`, `shell.persistence`, `shell.dynamic` and the other shell rules.
  - `bigman sandbox` rehearses an approved script offline.
  - Gap: no rule recognizes repo or key installation specifically.
- **Proposed mitigation / detection:**
  - New installer rules: `installer.adds-repo` (writes under `sources.list.d`, `yum.repos.d`, `zypp/repos.d`, `pacman.conf`, or runs `add-apt-repository`, `dnf config-manager`, `zypper ar`, `flatpak remote-add`), `installer.adds-key` (`apt-key`, `gpg --dearmor` into keyrings, `rpm --import`), `installer.disables-verification` (any DP-02 setting), and `installer.unscoped-key` (key added without `signed-by`).
  - Link findings to the post-install `bigman repos` audit so the user can confirm what changed.
  - Add a "history scan" (opt-in) that lists past pipe-to-shell commands for review.
- **Test case idea:** Fixture script that contains, as inert text, a line writing `deb [trusted=yes] https://example.invalid stable main` into `/etc/apt/sources.list.d/fixture.list` and an `rpm --import https://example.invalid/key`. Assert `installer.adds-repo`, `installer.disables-verification` and `installer.adds-key` without executing it.
- **References:** idontplaydarts.com 2016 "Detecting the use of curl | bash server side" (original domain no longer resolves as of 2026-10-29; discussion at https://forums.whonix.org/t/detecting-the-use-of-curl-bash-server-side/2361; cited conceptually), https://sandstorm.io/news/2015-09-24-is-curl-bash-insecure-pgp-verified-install, MITRE T1059.004 (Unix Shell), T1105 (Ingress Tool Transfer).

### DP-19 Homebrew on Linux and Nix
- **What it is:** Homebrew on Linux installs into `/home/linuxbrew/.linuxbrew` or `~/.linuxbrew`, owned by the user. Any process running as that user can replace binaries that end up early on `PATH`. Formulae and casks are executable Ruby, so taps (third-party formula repos) are code-execution grants. Nix's store is content-addressed and read-only, but the daemon runs as root and has had integrity bugs, and users can add arbitrary substituters and trusted public keys.
- **Real-world examples:**
  - Trail of Bits audited Homebrew (published 2024-07-30): 25 findings, including loading executable code from unexpected sources. **CVE-2024-42381**, a Linux-specific code execution flaw via `ldd` on untrusted ELF files, was fixed in brew 4.2.20 (https://blog.trailofbits.com/2024/07/30/our-audit-of-homebrew/, https://brew.sh/2024/07/30/homebrew-security-audit).
  - **Nix CVE-2024-27297:** a fixed-output derivation could pass file descriptors to modify a store path after Nix had registered it as valid. Fixed in 2.3.18/2.18.2/2.19.4/2.20.5 (https://nvd.nist.gov/vuln/detail/cve-2024-27297).
- **What a defender inspects (indicators):**
  - `brew tap` (third-party taps), `brew config`, and ownership and permissions of the brew prefix.
  - `PATH` order: does a brew bin directory precede `/usr/bin` for `sudo`, `ssh` or `git` names?
  - Nix: `/etc/nix/nix.conf` and `~/.config/nix/nix.conf` for `substituters`, `trusted-substituters`, `trusted-public-keys`, `require-sigs = false`, and `trusted-users` (a user in `trusted-users` is effectively root), plus `nix --version`.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** `brew.third-party-tap` (Info), `brew.shadows-system-binary` (Review: brew provides `sudo`, `ssh`, `gpg` or `git` earlier on PATH), `nix.require-sigs-disabled` (High), `nix.extra-substituter` (Review), and `nix.trusted-users-broad` (Review: `@wheel`, `*`, or the desktop user). Versions go to the advisory matcher.
- **Test case idea:** Fixture `nix.conf` with `require-sigs = false` and `trusted-users = *`. Fixture PATH ordering with a temp dir containing a fake `ssh` script that only echoes. Assert findings.
- **References:** links above, https://nix.dev/manual/nix/latest/command-ref/conf-file, https://docs.brew.sh/Homebrew-on-Linux, MITRE T1574.007 (Path Interception by PATH Environment Variable).

### DP-20 Stale systems: missing security updates or broken unattended upgrades
- **What it is:** Most real compromises use known, already-patched bugs. Desktop users often believe automatic updates are on when the timer is disabled, a held package blocks the transaction, third-party repo errors abort `apt-get update`, or the distro release has gone EOL. Long uptimes also mean updated libraries are installed but old copies are still mapped in running processes until a restart.
- **Real-world examples:**
  - Debian documents unattended-upgrades via `/etc/apt/apt.conf.d/50unattended-upgrades`, the `apt-daily.timer`/`apt-daily-upgrade.timer` units and logs in `/var/log/unattended-upgrades/` (https://wiki.debian.org/UnattendedUpgrades).
  - Fedora/RHEL use `dnf-automatic` (`dnf-automatic.timer`, `/etc/dnf/automatic.conf` `apply_updates`, `upgrade_type = security`). Fedora 41+ ships dnf5 with a reworked automatic plugin; check the current docs.
  - The xz incident itself (DP-01) shows the reverse: rolling and pre-release users received a backdoor quickly. Update policy is a trade-off, not "faster is always safer".
- **What a defender inspects (indicators), read-only:**
  - `systemctl is-enabled apt-daily-upgrade.timer unattended-upgrades.service dnf-automatic.timer dnf5-automatic.timer` and `systemctl list-timers`.
  - `/var/log/unattended-upgrades/unattended-upgrades.log` last run and errors.
  - `apt list --upgradable` and `apt-get -s upgrade` (simulation).
  - `dnf updateinfo list --security` / `dnf check-update --security`, `zypper list-patches --category security`, `arch-audit` (Arch).
  - `apt-mark showhold` / `dnf versionlock list`.
  - `needrestart -b` or `dnf needs-restarting -r` (or `/run/reboot-required` on Debian/Ubuntu).
  - Running kernel (`uname -r`) compared with the newest installed kernel.
- **Bigman today:** Partial. `bigman advisories` (Protect > Updates) matches installed packages against the distribution's own data (dnf updateinfo on Fedora, the Debian security tracker, the Arch security tracker), raises updates that fix a CISA KEV entry to High, and flags a pending reboot. Ubuntu, SUSE and Alpine data and broken unattended upgrades are not covered yet.
- **Proposed mitigation / detection:** `bigman doctor` adds:
  - `updates.auto-disabled` (Medium)
  - `updates.last-success-old` (Medium, more than 14 days since the last successful run)
  - `updates.security-pending` (High, with package and CVE list, the "update X because of CVE Y" output)
  - `updates.held-security` (High: a held package has a pending security fix)
  - `updates.reboot-required` (Medium)
  - `os.eol` (DP-04)

  All rootless: simulation commands and log/timer reads work unprivileged on Debian/Fedora defaults. `unattended-upgrades` logs may be root-only on some distros; report "incomplete" then. Bigman never applies updates itself.
- **Test case idea:** Fixture `apt list --upgradable` text output and a fixture USN/DSA mapping. Assert `updates.security-pending` names the CVE. Fixture `unattended-upgrades.log` with the last run 30 days ago gives `updates.last-success-old`. Fixture `/run/reboot-required` in a temp root.
- **References:** Debian wiki above, https://dnf.readthedocs.io/en/latest/automatic.html, https://security-tracker.debian.org/tracker/, https://ubuntu.com/security/notices, https://gitlab.archlinux.org/archlinux/arch-audit (location unverified; formerly on GitHub), MITRE T1190/T1068 (exploitation of unpatched software).

### DP-21 Installed files altered after install (package DB mismatch)
- **What it is:** Package databases record a digest, size, mode and owner for every installed file. Rootkits, trojanized binaries (for example a replaced `sshd` or `sudo`) and botched manual installs show up as mismatches. A root attacker can also update the database, so this is a tripwire for opportunistic tampering, not a guarantee. Configuration files legitimately change, which creates noise that must be filtered.
- **Real-world examples:** Red Hat's 2008 tampered-OpenSSH response relied on package verification and a published blacklist (DP-06). The 2008 Fedora intruder built modified OpenSSH and RPM packages for internal hosts. See [host integrity](./04-host-integrity-and-hardening-audit.md) for the full checklist. This entry is the packaging-specific view.
- **What a defender inspects (indicators), read-only:**
  - `dpkg --verify` (dpkg ≥1.17; md5 only) or `debsums -c` / `debsums -as` (list changed files, including config).
  - `rpm -Va` (flags: `S` size, `5` digest, `M` mode, `U`/`G` owner, `T` mtime; `c` marks config files).
  - `pacman -Qkk` (checks mtime, size, sha256 via mtree), or `paccheck --sha256sum` from pacutils.
  - Missing md5sums files in `/var/lib/dpkg/info/` for some packages (debsums `-l`).
  - Files in `/usr/bin`, `/usr/sbin` and `/usr/lib*` owned by no package.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - `bigman audit packages-verify` runs the distro's verifier and parses its output.
  - It suppresses config-file changes by default, with a separate list.
  - It escalates mismatches in a curated high-value set (sshd, sudo, su, login, passwd, ssh, gpg, systemd, libpam*, libc, ld.so, the package managers themselves) to `pkgdb.critical-mismatch` (High); other mismatches are `pkgdb.mismatch` (Review).
  - Unowned binaries in system bin directories produce `pkgdb.unowned-binary` (Review).
  - Rootless runs report unreadable files as "incomplete", not clean. Prefer a Rust reader of `/var/lib/dpkg/info/*.md5sums` and pacman `mtree` files for speed and determinism; rpm digests need `rpm` itself.
  - Baseline the result so only new mismatches alert in the monitor.
- **Test case idea:** Fake root with `var/lib/dpkg/info/fixture.md5sums` listing `usr/bin/fixture` with a known md5, then write different bytes to `usr/bin/fixture` in the temp root. Assert `pkgdb.mismatch`. Name it `sshd` to assert escalation. Fixture `etc/fixture.conf` marked as a conffile is suppressed.
- **References:** https://manpages.debian.org/bookworm/debsums/debsums.1.en.html, https://manpages.debian.org/bookworm/dpkg/dpkg.1.en.html (`--verify`), https://rpm-software-management.github.io/rpm/man/rpm.8.html (verify options), https://man.archlinux.org/man/pacman.8 (`-Qkk`), MITRE T1554 (Compromise Host Software Binary), T1014 (Rootkit).

### DP-22 Third-party repo overriding distro packages (pinning, priority, epoch)
- **What it is:** Once a third-party repo is enabled, it can publish a package with the same name as a core distro package and a higher version or epoch. The package manager will "upgrade" to it. Some vendor instructions also add apt pinning (`Pin-Priority` ≥1000 allows downgrades) or dnf `priority=` to force their packages to win. The vendor, or whoever controls the vendor repo (DP-10), can then replace `openssl` or `sudo`.
- **Real-world examples:** Debian's "Don't break Debian" guidance warns against mixing third-party repos and core packages (https://wiki.debian.org/DontBreakDebian). Fedora's third-party guidance warns about mixing repositories. No named malicious incident was verified for this exact mechanism; it is the escalation path for DP-10/DP-12.
- **What a defender inspects (indicators):**
  - `/etc/apt/preferences` and `/etc/apt/preferences.d/*` with `Pin-Priority` >990 for non-distro origins.
  - `apt-cache policy` for core packages, showing which origin wins.
  - dnf `priority=`, `exclude=` and `includepkgs=` in `.repo` files.
  - zypper `priority`.
  - Installed packages whose origin is not the distro: `apt list --installed` with origin fields, `dnf repoquery --installed --qf '%{name} %{from_repo}'`, `pacman -Qm`.
- **Bigman today:** Partial. `channel.priority-override` reports apt pins above 500 and dnf or zypper priorities above the distribution's; `advisory.untracked-package` lists installed packages outside the distribution's data. Epochs are not compared.
- **Proposed mitigation / detection:**
  - `repo.high-pin` (Review)
  - `repo.overrides-core` (High when a non-distro origin supplies a package in a curated core list)
  - `pkg.foreign-origin` (Info inventory)

  Rootless.
- **Test case idea:** Fixture `preferences.d/vendor` with `Package: *`, `Pin: origin example.invalid` and `Pin-Priority: 1001`. Fixture `apt-cache policy openssh-server` output showing the winning origin as `example.invalid`. Assert both rules.
- **References:** https://manpages.debian.org/bookworm/apt/apt_preferences.5.en.html, https://wiki.debian.org/DontBreakDebian, https://docs.fedoraproject.org/en-US/quick-docs/third-party-software/, MITRE T1195.002.

### DP-23 Boot chain and kernel modules: Secure Boot off, MOK enrollment, unsigned modules
- **What it is:** UEFI Secure Boot (via shim and GRUB on most distros) plus kernel module signature enforcement keeps unsigned bootloaders and kernel modules from loading. Desktop users often disable Secure Boot to get NVIDIA or VirtualBox drivers working, or enroll a Machine Owner Key (MOK) through DKMS, whose private key then sits on disk and can sign anything. Boot-chain bugs have allowed bypass even with Secure Boot on. This is only high-level packaging context; kernel and module integrity auditing is in [host integrity](./04-host-integrity-and-hardening-audit.md).
- **Real-world examples:**
  - **BootHole CVE-2020-10713** (GRUB2 buffer overflow, Eclypsium, 2020-07-29), which required DBX revocation updates.
  - **CVE-2023-40547** (shim HTTP boot RCE, fixed in shim 15.8, 2024): https://eclypsium.com/blog/the-real-Shim-shady-how-cve-2023-40547-impacts-most-linux-systems/
  - The kernel documents that with enforcement off, unsigned modules load and taint the kernel with `E` (https://www.kernel.org/doc/html/latest/admin-guide/module-signing.html).
- **What a defender inspects (indicators), read-only:**
  - `mokutil --sb-state` (works unprivileged on most systems) and `mokutil --list-enrolled`.
  - `/sys/firmware/efi` presence.
  - `cat /proc/sys/kernel/tainted`, decoded: bit 12 `E` (unsigned module) and bit 13 `O` (out-of-tree).
  - `/sys/module/*/taint`.
  - `cat /sys/module/module/parameters/sig_enforce`.
  - Lockdown mode in `/sys/kernel/security/lockdown`.
  - DKMS MOK private key location (`/var/lib/dkms/mok.key`; check existence and mode only, never read it).
  - fwupd/LVFS DBX update state: `fwupdmgr get-devices`, `fwupdmgr get-updates` (read-only).
- **Bigman today:** Partial. `harden.secure_boot` (Info) reads the `SecureBoot` EFI variable without root and gives the firmware steps, including enrolling the key of out-of-tree modules first ([hardening research](../research/hardening.md)). MOK enrollment and module taint are not read yet.
- **Proposed mitigation / detection:**
  - `bigman doctor` adds `boot.secureboot-off` (Info/Medium: informative, not alarming, since many users need it off), `kernel.unsigned-module-loaded` (Review, listing modules with `E` taint), `kernel.sig-enforce-off` (Info) and `boot.mok-key-readable` (High if the DKMS MOK private key is group- or world-readable).
  - Feed-driven DBX/shim advisories.
  - Rootless except `mokutil --list-enrolled` on some distros (report incomplete).
- **Test case idea:** Fake sysfs root with `proc/sys/kernel/tainted` = `12288` (E+O), `sys/module/fixture/taint` = `OE`, and a temp `var/lib/dkms/mok.key` chmod 0644 (dummy bytes). Assert the three findings and that the key content is never read.
- **References:** links above, https://eclypsium.com/blog/theres-a-hole-in-the-boot/ (2020-07-29), https://wiki.debian.org/SecureBoot, MITRE T1542.003 (Bootkit), T1547.006 (Kernel Modules and Extensions), T1553.006 (Code Signing Policy Modification).

## Tools and scripts worth mining for rules

| Tool | URL | License | What we can borrow | Caveats |
|---|---|---|---|---|
| debsums / `dpkg --verify` | https://manpages.debian.org/bookworm/debsums/debsums.1.en.html | GPL-2.0 | md5sums file format, config-file exclusion logic (DP-21) | MD5 only; root attacker can rewrite |
| `rpm -V` | https://rpm-software-management.github.io/rpm/man/rpm.8.html | GPL-2.0 / LGPL | Verify flag semantics (`S.5....T. c`) to parse (DP-21) | Some files unreadable as non-root |
| pacman `-Qkk`, pacutils `paccheck` | https://man.archlinux.org/man/pacman.8, https://github.com/andrewgregory/pacutils | GPL-2.0 (pacman); pacutils MIT (verify) | mtree-based verification; `paccheck --sha256sum` | Arch only |
| lintian | https://salsa.debian.org/lintian/lintian | GPL-2.0+ | Maintainer-script tags (for example `maintainer-script-*`, `possibly-insecure-handling-of-tmp-files-in-maintainer-script`) as a list of patterns for DP-09 | Perl; checks policy quality, not malice |
| rpmlint | https://github.com/rpm-software-management/rpmlint | GPL-2.0+ | Scriptlet checks, setuid and permission checks, `/etc` writes | Same caveat |
| namcap | https://gitlab.archlinux.org/pacman/namcap | GPL-2.0+ | PKGBUILD/package lint rules for DP-11 | Not a malware detector |
| ComplianceAsCode (OpenSCAP content) | https://github.com/ComplianceAsCode/content | BSD-3-Clause | Rule logic such as `ensure_gpgcheck_globally_activated`, `ensure_gpgcheck_never_disabled`, `ensure_gpgcheck_local_packages`, `ensure_redhat_gpgkey_installed`, and apt equivalents (DP-02) | Server-oriented; verify exact rule IDs before citing them in output |
| Lynis | https://github.com/CISOfy/lynis | GPL-3.0 | Package tests (PKGS-*: GPG checks, pending security updates, unattended upgrades); Bigman already parses Lynis reports | Test IDs should be confirmed against the current Lynis release |
| osquery | https://osquery.io/schema/ | Apache-2.0 / GPL-2.0 | `apt_sources`, `yum_sources`, `deb_packages`, `rpm_packages`, `kernel_modules` tables, a reference for fields to collect | Bigman should parse files itself to stay dependency-light |
| arch-audit | https://gitlab.archlinux.org/archlinux/arch-audit (unverified location) | MIT (verify) | Matching installed packages with security.archlinux.org AVGs (DP-20) | Arch only |
| needrestart | https://github.com/liske/needrestart | GPL-2.0 | Detecting processes using deleted or updated libraries (DP-20) | Root for full view |
| Vegard Nossum's xz `detect.sh` (linked from Freund's post) | https://www.openwall.com/lists/oss-security/2024/03/29/4 | unverified | Example of a one-off, version- and binary-pattern check for a known-bad release (DP-01) | Superseded by version checks; do not ship byte patterns blindly |
| diffoscope | https://diffoscope.org/ | GPL-3.0+ | Deep diff of tarball against git export for the DP-01 maintainer helper | Heavy; optional |
| Reproducible Builds / rebuilderd | https://reproducible-builds.org/, https://github.com/kpcyrd/rebuilderd | various (rebuilderd GPL-3.0, verify) | Status of whether installed packages were independently rebuilt (Arch, Debian) | Coverage varies by distro |
| TUF / in-toto | https://theupdateframework.io/, https://in-toto.io/ | Apache-2.0 / MIT | Design vocabulary (freeze, rollback, threshold) for DP-05 findings text | Not deployed in distro apt/dnf today |
| Flatseal | https://github.com/tchx84/Flatseal | GPL-3.0 | Permission categories and human-readable labels (DP-15) | GUI; read its permission model, don't depend on it |
| AppImage `validate` / AppImageSpec | https://github.com/AppImage/AppImageSpec | MIT | ELF section names (`.sha256_sig`, `.sig_key`, `.upd_info`) for DP-17 | Verification still needs a trusted key |
| mokutil, fwupd | https://github.com/lcp/mokutil, https://github.com/fwupd/fwupd | GPL-3.0 / LGPL-2.1+ | Secure Boot state and DBX/firmware update status (DP-23) | Some queries need root |

## Papers and long-form references

- Cappos, Samuel, Baker, Hartman (2008). "A Look in the Mirror: Attacks on Package Managers." ACM CCS 2008. https://www2.cs.arizona.edu/stork/packagemanagersecurity/ccs2008.pdf. Signing is not enough: malicious mirrors can freeze, replay and roll back clients, and becoming a mirror was easy.
- Cappos et al. (2008). "Package Management Security." University of Arizona TR08-02. https://www.cs.arizona.edu/sites/default/files/TR08-02.pdf. A longer companion survey of ten package managers' security properties.
- Samuel, Mathewson, Cappos, Dingledine (2010). "Survivable Key Compromise in Software Update Systems." ACM CCS 2010. https://uptane.org/assets/files/samuel_ccs_2010-2e4e7a69695e936b1ad3b0b7010c550a.pdf. Introduces TUF: role separation, thresholds, expiry and explicit revocation so a single key theft is survivable.
- Russ Cox (2024). "Timeline of the xz open source attack." https://research.swtch.com/xz-timeline. A day-by-day reconstruction of the social-engineering campaign. The key lesson is that release artifacts must match source.
- Andres Freund (2024). "backdoor in upstream xz/liblzma leading to ssh server compromise." oss-security. https://www.openwall.com/lists/oss-security/2024/03/29/4. A primary detection write-up showing how performance anomalies plus valgrind exposed a supply-chain backdoor.
- Fedora Project / Paul W. Frields (2009). "Update and Report on Fedora August 2008 Intrusion." https://listman.redhat.com/archives/fedora-announce-list/2009-March/002626.html. Signing host exposure, key rotation and the value of a published post-mortem.
- Gentoo Infrastructure (2018). "2018-06-28 Github incident report." https://wiki.gentoo.org/wiki/Project:Infrastructure/Incident_reports/2018-06-28_Github. An honest lessons-learned list: mirror vs canonical source, communications, and credential hygiene.
- Debian Project (2003). "Debian Investigation Report after Server Compromises." https://www.debian.org/News/2003/20031202. Early distro infrastructure breach; the archive survived because of separation.
- Kaspersky GReAT (2023). "Backdoored Free Download Manager for Linux." https://securelist.com/backdoored-free-download-manager-linux-malware/110465/. A vendor-site redirect served a trojanized `.deb` via postinst for about three years.
- Trail of Bits (2024). "Our audit of Homebrew." https://blog.trailofbits.com/2024/07/30/our-audit-of-homebrew/. The package format is executable code, so taps are trust grants; CVE-2024-42381.
- Alan Pope (2024, 2026). Snap Store scam series. https://blog.popey.com/2024/02/exodus-bitcoin-wallet-490k-swindle/ and https://blog.popey.com/2026/01/malware-purveyors-taking-over-published-snap-email-domains/. Store "trust" signals erode, and expired publisher domains enable account takeover.
- Helm Project (2026). "Security Notice: Former Helm APT Mirror Domain baltocdn.com." https://blog.helm.sh/blog/security-notice-baltocdn. A concrete modern case of a lapsed repo domain re-registered by a third party.
- Qualys (2022). "Oh Snap! More Lemmings (CVE-2021-44731)." https://www.qualys.com/2022/02/17/cve-2021-44731/oh-snap-more-lemmings.txt. Sandbox setup helpers are privileged attack surface.

## Open questions for the maintainer

1. **Scope of `bigman repos --online`:** DNS resolution of repo hosts leaks which vendors a user trusts to their resolver. Is opt-in DNS acceptable, or should domain-expiry detection rely only on a shipped `known-dead-repos.json` updated with Bigman releases and the feed pipeline?
2. **Advisory data source for "update X because of CVE Y":** per-distro trackers (Debian security tracker JSON, Ubuntu OVAL/USN JSON, Fedora Bodhi/updateinfo, Arch AVG JSON) or OSV's distro ecosystems as one normalized feed? OSV is simpler but its distro coverage lags for some (verify current coverage of Fedora and Arch).
3. **Package parsing dependencies:** is it acceptable to add pure-Rust ar/tar/cpio/xz/zstd and OpenPGP (sequoia, LGPL) crates, or should Bigman shell out to `dpkg-deb`/`rpm` inside the existing Bubblewrap boundary? Pure Rust keeps "never execute package code" strict. Shelling out means less code.
4. **Severity of disabled Secure Boot:** many NVIDIA and VirtualBox users have it off deliberately. Should `boot.secureboot-off` be Info-only with an explanation, or suppressible via accepted baseline like startup changes?
5. **Root-only data:** `/var/lib/snapd/state.json`, some `rpm -Va` paths and some `unattended-upgrades` logs need root. Should Bigman offer an optional polkit-elevated, read-only helper with a fixed command list (as `bigman setup` does for installs), or keep reporting these as "incomplete"?
6. **Change monitoring:** should `/etc/apt/sources.list.d`, `/etc/apt/trusted.gpg.d`, `/etc/apt/keyrings`, `/usr/share/keyrings`, `/etc/yum.repos.d`, `/etc/pacman.conf`, `/etc/zypp/repos.d`, flatpak remotes and `~/.local/share/applications` join the existing `bigman changes` snapshot set now? They are cheap, world-readable, and the highest-signal place to catch DP-02/03/10/18 at the moment they happen.
7. **AUR coverage:** given the 2026 incidents, is an Arch-specific "review PKGBUILD diff before helper update" workflow (`bigman packages PKGBUILD_DIR` plus diff against the previously installed recipe in the helper's cache) worth the maintenance cost relative to Arch's share of the user base?
