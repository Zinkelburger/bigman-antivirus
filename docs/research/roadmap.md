# Bigman roadmap from the threat map

> Prioritised build plan derived from the eight files in [docs/threats](../threats/README.md) and the two research surveys. Last updated: 2026-10-29.

The threat map has 181 entries. Bigman fully covers 1, partially covers 59 and has gaps on 121. Many gaps share one missing building block, so the plan is ordered by building block rather than by threat. Each phase lists the entries it closes or improves.

## Guiding constraints

These come from the current design in the README and were kept by every research file:

- Deterministic rules only, no ML, no telemetry, no automatic upload or deletion.
- Rootless by default. Anything that needs root is an explicit, previewed, user-approved action.
- Results stay "findings to review", "no findings from these checks" or "incomplete". Never "safe".
- Bigman is AGPL-3.0-only, so GPL-2.0-only and Elastic License 2.0 content is reimplemented, not copied.

## Phase 1: extend what already exists

Low risk, mostly rootless, and each item reuses an existing workflow.

| Feature | Builds on | Closes or improves |
|---|---|---|
| **Wider startup baseline.** Add cron.{hourly,daily,weekly,monthly}, user crontab, `/etc/profile.d`, `environment.d`, rc.local and init.d, MOTD scripts, udev rules, `/etc/pam.d`, `ld.so.conf.d`, `modules-load.d`, `~/.ssh/config`, `/etc/hosts`, `resolv.conf`, `mimeapps.list`, `.github/workflows` per project. Record mode, owner and file capabilities, not only content hashes. | `bigman changes` | HI-01 to HI-11, MW-15, MW-18, MW-19, NE-15, CR-09, DS-10, SC-17 |
| **Package attribution of changes.** Look up each changed file in rpm/dpkg/pacman and label it "package-owned and verified", "package-owned but modified" or "unowned". | `bigman changes` | Reduces noise everywhere; HI-01, HI-08, MW-08, MW-15 |
| **Developer brand pack for the extension.** Registries, forges, container hubs, IDE marketplaces, Flathub, Snapcraft, Homebrew and distro sites. Fix the rule that skips brand names under 5 characters, which missed `pypj.org`. Add Linux ClickFix phrases and `blob:` URLs. | link detector | PH-02, PH-03, PH-11, PH-18, PH-20, PH-21 |
| **Credential inventory, metadata only.** Fixed path list for git, npm, PyPI, cargo, Docker, gh, cloud and kube configs. Report presence of a token-like key, mode, owner and age. Never values. Detect passphrase-less SSH and GPG keys from their headers only. | `bigman doctor` | CR-01 to CR-04, CR-08, CR-10, CR-16, CR-20 |
| **Listener, firewall and Docker findings.** Turn the raw `ss` and firewall dumps into findings with owning unit, binary and package. Flag wildcard binds, Docker API on TCP, published ports that bypass the firewall, and local AI or debug servers reachable from the LAN. | `bigman doctor` | NE-01, NE-06, NE-07, NE-09 to NE-12, NE-22, MW-03 |
| **Separate malicious-package findings.** Report OSV `MAL-` records as a High finding, not with the "a vulnerability is not malware" wording. | `bigman audit osv` | SC-05, SC-06, SC-11 |

## Phase 2: "update X because of CVE Y"

The owner's headline request. Full design in [vulnerability-intelligence-and-feeds.md, section 4](./vulnerability-intelligence-and-feeds.md#4-design-bigman-advisories).

- **`bigman advisories`.** Match installed packages against distro data only: dnf5 advisories on Fedora, the Debian tracker or debsecan data on Debian, Ubuntu OVAL, Arch `all.json`, Alpine secdb. Print the package, installed and fixed versions, the CVE and the update command. Never run the update.
- **Exploited-in-the-wild priority.** Raise severity when the CVE is in the CISA Known Exploited Vulnerabilities catalog. Show EPSS only as context.
- **Reboot required.** Flag a running kernel older than the installed one.
- **Honest coverage.** Report packages no tracker covers (AUR, local builds, third-party repos), stale data and end-of-life releases as incomplete coverage.
- **`bigman feeds fetch` and `digest`.** Pull the OPML feeds, de-duplicate by CVE or advisory ID, mark items that affect this machine, and hand the digest to the weekly agent prompt in section 5 of the same document. Agent output becomes a proposal for review, never an automatic rule.

Closes or improves DP-01, DP-04, DP-16, DP-20, HI-23, NE-04, NE-17, DS-04, DS-11, MW-20.

## Phase 3: inspect before install or open

Extends the existing "inspect, never execute" model beyond npm and shell scripts.

| Feature | Closes or improves |
|---|---|
| **`bigman repos` audit.** Read apt (including deb822), yum.repos.d, zypp, `pacman.conf`, Flatpak remotes and Nix config. Flag disabled signatures, plain HTTP, keys without `signed-by`, stale metadata, expired or re-registered repo domains, and third-party repos that override distro packages. | DP-02, DP-03, DP-05, DP-10, DP-12, DP-22 |
| **System package inspection.** Parse `.deb`, `.rpm`, PKGBUILD and AppImage files without installing. Run the existing `shell.*` rules on maintainer scripts. Flag files dropped into repo, key, cron, systemd or preload paths, and setuid bits. Route browser downloads here. | DP-08, DP-09, DP-11, DP-17, DP-18, MW-09, MW-10 |
| **Language package inspection beyond npm.** PyPI wheels and sdists, including executable `.pth` files; Rust crates with build scripts and proc-macros; VS Code `.vsix`; pnpm, yarn and bun lockfiles; missing npm hooks and `binding.gyp` builds. | SC-01, SC-03, SC-04, SC-19 |
| **`bigman repo-check DIR`.** Run before trusting a cloned repo: IDE auto-run tasks, `.envrc`, devcontainers, git hooks and filters, AI-tool configs (`.claude/`, `.mcp.json`, `.cursor/`, instruction files with hidden Unicode), and GitHub workflow risks such as unpinned actions and `pull_request_target`. | SC-15, SC-16, SC-20, SC-21, SC-22, PH-10, CR-19 |
| **Lockfile linter with diff mode.** Non-registry dependencies, missing integrity hashes, new packages with install hooks, and declared dependencies the code never imports. | SC-02, SC-09, SC-10, SC-13 |
| **Archive and launcher checks.** List-only archive analysis for path traversal, symlink escapes and startup-path entries. Flag `.desktop` launchers disguised as documents. | DS-09, DS-13 |

## Phase 4: system posture

| Feature | Closes or improves |
|---|---|
| **Posture section in `bigman doctor`.** Firmware security rating via `fwupdmgr security`, Secure Boot and lockdown, kernel taint, module signing, SELinux or AppArmor mode, session type (X11 or Wayland), USBGuard, screen lock and autologin, attack-surface sysctls. | HI-20, HI-21, HI-24, HI-25, DP-23, DS-01, DS-14, DS-15, DS-18, MW-16, MW-17 |
| **Installed-file verification.** Wrap `rpm -Va`, `dpkg --verify` and `pacman -Qkk`, explain normal config-file noise, and alert only on new mismatches, with higher severity for sshd, sudo, PAM and the dynamic linker. | DP-21, HI-22, MW-08 |
| **SUID, SGID and file-capability inventory** compared with the package database. | HI-17, HI-18, HI-19 |
| **Account audit.** Extra UID 0 accounts, new accounts, root-equivalent groups such as docker, disk and libvirt, with plain explanations. | HI-12, HI-16 |
| **Process checks for the user's own processes.** Deleted-but-running and memory-only executables, names that don't match the binary path, shells whose input and output are a network socket, and PATH entries that shadow system tools. | MW-02, MW-22, MW-23, NE-14 |
| **Flatpak and Snap audit.** Permissions with one-line risk explanations, classic and devmode snaps, unverified publishers, permission increases on update. | DS-03, DS-05, DP-13, DP-15 |
| **Browser extension inventory.** Chromium-family and Firefox profiles plus managed-policy folders. Report permission increases on update, policy-forced and unpacked extensions, and matches against user-imported bad-ID lists. | DS-06, DS-07, DS-08, CR-13 |

## Phase 5: deeper, opt-in protections

These need new trust decisions, host permissions or privilege, so each needs an explicit owner decision first.

- **Optional privileged read-only helper**, following the existing polkit setup pattern, for sudoers, `/etc/shadow` lock status (never hashes), root cron spools, polkit `/etc` rules and `sshd -T`. Closes HI-13 to HI-15 and NE-02.
- **Browser protections that act on the page.** A credential-entry guard that warns before a password is typed on a lookalike origin, a copy guard for clipboard hijacking, and plain-language explainers on OAuth-consent and device-code pages. Closes PH-05, PH-07, PH-08, PH-12.
- **Full navigation coverage** with `declarativeNetRequest`, so imported blocklists also stop typed and script navigations. This needs host permissions. Closes PH-04, PH-19.
- **More canary types.** SSH key, kubeconfig, `.git-credentials`, `.npmrc` in a decoy project, AWS profile and wallet decoys, with optional open-event watching. Improves CR-18, CR-22.
- **Reader attribution** through an optional, previewed auditd rule installer or importers for Falco, Tracee or Kunai output. Tells the user which process read a canary.
- **Embedded YARA-X** with a small rule set Bigman owns, plus download-on-request rule packs that keep their licences and attribution.

## The weekly threat-intel loop

1. `bigman feeds fetch` pulls the 45 verified feeds in [feeds.opml](./feeds.opml).
2. `bigman feeds digest` groups duplicate stories and marks items that match installed packages.
3. An agent runs the prompt template from section 5.2 of [vulnerability-intelligence-and-feeds.md](./vulnerability-intelligence-and-feeds.md) and drafts new entries in the `docs/threats` format.
4. The maintainer reviews each draft and turns accepted ones into rules with synthetic test fixtures.

## Decisions needed from the maintainer

- **Privileged helper.** Whether to add an optional, read-only privileged component, or stay strictly rootless and report those areas as incomplete.
- **Baseline format.** Phase 1 adds mode, owner, capabilities and package ownership, which needs a migration of stored baselines.
- **Extension permissions.** Full navigation coverage and page-level guards need broader host permissions than the extension has now.
- **Firefox support.** Several browser proposals are Chromium-only today.
- **Feed and rule licences.** Google Safe Browsing is non-commercial with a 30-minute freshness rule, OpenPhish is personal use only, MalwareBazaar needs a per-user key and forbids redistribution, and Elastic rules are not AGPL-compatible. The safest pattern is "user imports or downloads with their own key", never bundled.
- **Unverified facts.** Each threat file ends with what its agent could not verify. Re-check the 2026 incidents before citing them publicly.
