# Bigman Security Companion

Bigman makes a set of deterministic Linux security checks easier to use. It has a desktop app, a CLI, and an optional Chromium extension. There is no AI model, automatic file upload, or automatic deletion of detections.

Results say **findings to review**, **no findings from these checks**, or **incomplete**. A scan cannot certify that a file or machine is safe. Capability rules explain behavior without equating every download, permission change, or unfamiliar hostname with malware.

## Start

Requirements: Linux, a current stable Rust toolchain, Node.js 22 with npm, and WebKitGTK development files for the desktop app (see [Desktop app](#desktop-app)). Build checks were run with Rust 1.98. The CLI works without a graphical session. Optional system tools are discovered at runtime.

```sh
./install.sh
```

This builds the locked release of `bigman` (CLI), `bigmand` (the per-user daemon that runs every check) and `bigman-app` (the desktop app), and installs them under `~/.local` with an application-menu entry. It does not install antivirus packages, change firewall policy, register browser hosts, start monitors, or enable schedules. Use the app's **Setup** tab to review and apply those supported actions; each one shows exactly what will change before it runs.

Recommended first use:

1. On **Overview**, click **Run checkup**.
2. In **Setup**, install the ClamAV group if needed. In **Protect → Scans**, click **Update signatures**, then scan Downloads.
3. In **Protect → Startup files**, compare and review the inventory before accepting an initial baseline.
4. In **Protect → Canaries**, click **Create, monitor and test…**. The result must say **Local read detection verified**; any startup failure is shown separately.
5. In **Setup**, install the background services if you want the monitor and the daily scan and update timers.

Package installation plans support Fedora, Debian and Ubuntu. Packages come from the user's configured repositories and administrator authentication uses polkit. Repository availability can vary; failures remain visible. Bigman does not add package repositories or disable existing security services.

## Implemented workflows

| Workflow | What it does | Important boundary |
|---|---|---|
| System checkup | Tool availability, known signature locations/age, monitor heartbeat, Git plaintext-store detection, listening sockets and selected protection status | Read-only inventory; no claim that the machine is clean |
| File scans | ClamAV checks private snapshots; findings map to original paths and SHA-256 hashes | Limits, missing tools, failures and skipped files are reported |
| Quarantine | Saves a private recovery copy, verifies the current file hash, then removes the original | Explicit per-file action, only for scan findings; no automatic cleanup |
| Restore | Verifies the recovery hash and creates the original path with mode 0600 | Never overwrites an existing file; executable permissions are not restored |
| Installer inspection | Saves exact local/HTTPS script bytes; source-linked capability rules and optional ShellCheck | Never runs scripts while inspecting; dynamic behavior remains unresolved |
| Inspect routing | One field for a file, folder or HTTPS URL (`bigman check`); the inspector is chosen from the URL scheme, file name and first bytes, never from what the file says about itself | A file no inspector recognises goes to the installer rules on the CLI and to ClamAV from the Downloads watch |
| Package inspection | Reads local package manifests, referenced script files, unavailable lockfile hooks, or `.tgz` contents without extracting; npm packages with a `binding.gyp` and no install script are flagged because npm builds them at install anyway | npm/pnpm-focused; no lifecycle or build commands invoked |
| Lockfile linter | `bigman lockfile`: dependencies from git, URLs or local paths, missing integrity hashes and install hooks in npm, Yarn, pnpm, Bun and Cargo lockfiles; `--against OLDER` reports only what a change adds, including a package that changed source without a new version | Reads the lockfile as text; install hooks are known only where the format records them (npm, pnpm) |
| Project folders (`repo-check`) | What a cloned folder can run when opened: VS Code auto-run tasks, `.envrc`, devcontainer commands, git hooks, filters and config commands, Claude Code hooks, MCP servers, AI instruction files with hidden Unicode, unpinned or `pull_request_target` workflows; Rust and Python projects get the build-script rules below | Nothing in the project is run; only the files these tools read on open are checked, not the code they then call |
| Package sources | `bigman package-sources`: apt (one-line and deb822), dnf, zypper, pacman, Flatpak and Nix sources with signatures off, plain HTTP, apt keys without `signed-by`, expiry checks off, stale metadata, priority over the distribution, personal repositories and an inventory of third-party ones; Protect > Updates | Configuration only; no repository is contacted, so an abandoned repository's domain is not checked. A private mirror of the distribution reads as third-party |
| Archives and launchers | Lists zip and tar archives (gzip, and xz, zstd or bzip2 through the isolated system decompressor) for `..` and absolute names, links leaving the folder, devices and setuid entries, startup and login paths and 100-fold expansion; `.desktop` launchers, alone or inside an archive, are reported as programs, High when they borrow a document's name or icon or download on open | Nothing is extracted or run; other file contents are not checked, and 7z and RAR are recognised but not listed |
| System package files | `.deb`, `.rpm`, built Arch packages, PKGBUILDs and AppImages parsed in Rust: maintainer scripts with the receipt line rules, files or script lines that add package sources or keys, setuid files, startup and sign-in paths, files in home or temporary folders, PKGBUILD downloads with `SKIP` checksums, AppImage signature and update address | No dpkg, rpm or makepkg runs; signatures are noted, not verified; an AppImage's own files are not listed |
| Language packages | Wheels, sdists, `.crate` files and `.vsix` extensions: `.pth` and sitecustomize code that runs at every Python start, console scripts named like common commands, `setup.py` that starts processes or fetches, crate build scripts (High when they open connections or read the home folder) and proc-macros, VS Code extensions that start with the editor, pull in other publishers' extensions, hide invisible text or ship native code | Static reading of the files that decide what runs; nothing is built, installed or imported |
| Downloads watch | The daemon checks each new file in Downloads (`XDG_DOWNLOAD_DIR`) once it stops changing, with the inspector above or a ClamAV scan, and posts one first-seen entry per file contents; files that arrived while it was stopped are checked when it starts | Runs while `bigmand` runs; partial browser downloads are skipped; it reads the file after it lands, so a desktop that previews or indexes it first is not prevented from doing so |
| Project script policy | Previews and writes `ignore-scripts=true` to project `.npmrc`, retaining a backup | Explicit `npm run` and command-line overrides can still execute code; this is not package trust certification |
| Restricted rehearsal | Runs an explicitly approved saved installer hash in an offline Bubblewrap environment, optionally traced with strace | No host install or promotion; restrictions can prevent completion; not a malware-detonation service |
| PDF triage | Escaped/raw PDF names, PDFiD, pdf-parser statistics, pdfinfo links | External tools require Bubblewrap; compressed/encrypted or unsupported content may remain unexamined |
| Startup changes | Compares startup, cron, shell, PAM, loader, kernel-module, udev, SSH, name-resolution and desktop-handler files (content, mode, owner, file capabilities) with a reviewed baseline, and attributes each change to its package: an update the package database explains is Info, a package's file that no longer matches is Review, or High for PAM, loader and SSH files | Baseline records prior state, not trust or the writer; root-only files are compared by metadata; sudoers needs the optional helper |
| Local canaries | One-action local generation, monitor startup, verified kernel read receipt, test/disarm, durable alerts and health heartbeat | No account or network required; login startup uses systemd when available; no reader identity or access blocking |
| Hosted canary shortcuts | Saves an HTTPS token URL supplied by the user | Bigman does not provision a provider account or token; the provider triggers only when the URL is activated |
| Optional integrations | Lynis report findings, OSV advisory matching, osquery process JSON, user-supplied YARA-X rules, OpenSnitch launch/setup | External tools must be installed; adapters preserve failures and raw evidence |
| Browser checks | Text/destination mismatch, lookalikes, selected Unicode confusables, ClickFix text combinations, user-imported blocklists, warning pages and expiring exceptions | Limited page/link heuristics; no remote reputation requests or complete navigation interception |
| Activity timeline | `bigmand` records your programs' first connections and new address ranges, new listening programs, package transactions (dnf5 history, pacman or dpkg log), SELinux/AppArmor denials as plain sentences, and every saved report; repeats fold into their first sighting, unread entries feed Overview and the tray | First-seen is a record of what was observed, not a verdict; denials need journal access (systemd-journal, adm or wheel) and say so when it is missing; other users' processes are not named without the optional helper |
| Network | Per-interface throughput kept in SQLite (1 s, minute averages after a day, hour averages after 30 days), current connections and listeners; listener rules for wildcard binds, the container API, debugger ports and local AI or notebook servers | Read from `/proc` as you; no packet capture, blocking or firewall changes |
| Security updates | Installed packages against the distribution's own security data (dnf updateinfo on Fedora, the Debian and Arch security trackers), with updates fixing a CISA KEV entry first, a pending-reboot flag and packages outside the distribution's data; `bigman advisories` or Protect > Updates | Uses the network for public data only (no package list is sent); shows the update command and never runs it; Ubuntu, SUSE and Alpine are not supported yet |
| Install receipts | Every package transaction gets a receipt: packages, files by category, install scripts verbatim with `receipt.*` rules marking suspicious lines, services shipped, raised privileges and what started listening within a minute; changed hooks are noticed against earlier receipts. Optional from Setup: a dnf, apt or pacman hook shows the receipt before the transaction commits, with Continue and Abort | Built after the fact from the package database; old package versions are not compared yet; removed packages are listed by name only. The preview hook needs Bigman installed system-wide and continues on its own after a minute or whenever Bigman cannot answer |
| Notifications | Batched desktop notifications for the categories you choose (findings on by default), at most one a minute, pausable from the tray | Raised by the daemon through `notify-send`; Activity always has every entry |
| Background services | Rootless collector daemon (which runs the Downloads watch), monitor and private ClamAV update timer; installing or removing clears the daily Downloads scan timer earlier versions installed | Explicit installation/removal; follows user session; no lingering or system services enabled |
| History/export | Private JSON reports, coverage, errors and evidence shared by the CLI, the desktop app's Activity tab and the native host | Export is explicit; reports can contain local paths and inspected source |

## CLI

```sh
bigman doctor
bigman scan ~/Downloads
bigman scan ~/Downloads --notify
bigman update
bigman inspect ./install.sh
bigman inspect https://example.org/install.sh
cat ./install.sh | bigman
bigman packages ./project
bigman packages ./downloaded-package.tgz
bigman check ~/Downloads/tool_1.0_amd64.deb   # picks the inspector: archive, launcher, package file, PDF, …
bigman repo-check ./cloned-project
bigman lockfile ./package-lock.json --against ./old/package-lock.json
bigman package-sources
bigman package-policy ./project                 # preview
bigman package-policy ./project --apply
bigman changes
bigman changes --accept-baseline REVIEWED_REPORT_ID
bigman history
bigman history REPORT_ID
bigman --json scan ~/Downloads
```

Baseline acceptance rechecks the saved report’s inventory hash and refuses changes made since review.

Report-producing commands return **0** when there are no review/high findings and no errors, **1** for review/high findings, and **2** for errors/incomplete coverage. Informational findings alone do not cause exit 1. `--json` formats reports and inventories; administrative status messages remain text. Piped input is inspection, never execution.

Restricted execution requires both the saved report and its exact hash:

```sh
bigman sandbox REPORT_ID --approve-sha256 SHA256_FROM_REPORT
```

The saved file is rehashed before launch. The sandbox exposes system runtime files read-only, uses a disposable home/work directory, clears inherited environment credentials, omits host service/agent sockets, and denies networking through a separate network namespace. It has wall-clock, CPU, address-space, output, descriptor, file-size and UID-relative process limits. The process limit is not a dedicated cgroup quota. If Bubblewrap cannot enforce the profile, execution fails without an unrestricted retry. Missing strace makes behavioral coverage incomplete even if the script finishes.

Inspection fetches use HTTPS, bounded size/time/redirects, public destination checks and DNS pinning through curl. Each redirect is checked. Proxies and curl user configuration are disabled for these fetches. Installers that need private endpoints can be downloaded separately and explicitly inspected as local files.

ClamAV defaults: at most 2,000 regular files, 64 MiB per file, 512 MiB total snapshot data. Archive expansion is limited to 256 MiB, 256 files and nesting depth 16. Symlinks, Bigman state and registered canaries are excluded. Directory walks have a 50,000-entry limit. Limit/encryption detections mean incomplete examination. The app prefers its private definitions after `update`, otherwise ClamAV’s default database configuration applies. Checkup freshness knows the private directory and `/var/lib/clamav`; custom system configurations require separate verification.

Quarantine identifiers and indexed scan items:

```sh
# Item indexes and hashes are in STATE/artifacts/REPORT_ID.scan.json.
# In the app, open a scan in Protect → Scans and use Quarantine… on a flagged file.
bigman quarantine REPORT_ID ITEM_INDEX
bigman quarantined
bigman restore QUARANTINE_ID
```

## Canaries and monitoring

The default is entirely local: no Canarytokens account, API key, paid service or internet connection is needed. In the app, choose **Protect → Canaries → Create, monitor and test…**. The CLI equivalent is:

```sh
bigman canary arm
```

This creates `~/Documents/backup-access.env` (or `~/backup-access.env` if Documents does not exist), starts monitoring, enables only the monitor at future user logins, performs a read, and waits for a fresh kernel-event receipt. Existing registered local decoys at that path are reused; unrelated existing files are refused. Install Bigman at a stable path first with `./install.sh` so the login service can still find it later.

The setup result reports local detection and startup coverage separately. If the user service manager is unavailable or has conflicting settings, Bigman runs a detached session monitor and reports **incomplete** startup coverage. It does not overwrite a differently configured monitor unit. The monitor continues after closing the app/CLI; logout/reboot gaps are not covered. Enabling at login while a session monitor is already running preserves that process until logout.

```sh
bigman canary arm ~/Documents/another-decoy.env
bigman canary arm --session-only        # no login service changes
bigman canary list
bigman canary start                    # restart existing tokens; create no new files
bigman canary test CANARY_ID            # verifies a new real read receipt
bigman history
bigman canary stop                      # stop monitor, remove its login service
bigman canary stop --session-only       # stop current process; leave service configuration
bigman canary disarm CANARY_ID          # remove this registration; retain its file
```

Verification fails if monitoring is stopped, the path isn't watched, or no new receipt arrives. An old receipt does not count. Receipts prove local filesystem detection and durable evidence; they do not prove desktop notifications were displayed. Alerts remain available in History if notification utilities or the desktop notification service are unavailable. Deliberate verification reads can also appear as ordinary canary alerts.

For manual creation without starting anything, use `bigman canary create PATH`, followed by `bigman watch`. `stop` also stops the startup-change monitor, but preserves tokens, registrations, reports and scan timers. `disarm` only removes the selected registration.

Creation is exclusive: an existing file, directory or symlink is not overwritten. The local token is a UUID-based decoy with no working privileges. Pick a location that ordinary applications rarely read. Test reads, backups, search indexers, antivirus tools and synchronization software can produce legitimate alerts. Bigman's own scanner skips registered decoys.

The monitor watches parent directories so replacements can be observed. It reports queue overflow, missing paths and watcher failures; writes a heartbeat; and groups repeated events for a canary within three seconds. Files copied or read while the monitor is stopped cannot be reconstructed later. inotify does not provide a reliable reader PID or tell whether data was exfiltrated. Host malware with the same user privileges can interfere with this application or its state.

Local monitoring cannot detect later use of a stolen token on a different computer. That needs a reachable callback service, either hosted by you or a provider; Bigman does not expose a network listener or provision one. This is optional and separate from account-free local detection.

To use an existing hosted token:

```sh
bigman canary create ~/Documents/backup-access.url --hosted-url https://YOUR-PROVIDER/YOUR-TOKEN
```

Use the HTTPS URL the provider gave you. This writes a shortcut and does not visit the URL. Opening the file as plain text does not necessarily activate the provider; following the URL does. Bigman does not embed tokens in live credentials, create cloud credentials, intercept HTTPS, or change certificate trust.

Enable persistent per-user services only after installing Bigman at a stable path:

```sh
bigman services                         # show exact unit contents
bigman services --apply                 # enable monitor and daily scan/update timers
bigman services --remove                # removal preview
bigman services --remove --apply
systemctl --user status bigmand.service bigman-monitor.service bigman-scan.timer bigman-update.timer
```

The monitor compares persistence snapshots every five minutes. Review the initial inventory and establish a baseline first. Existing state remains after disabling services. Use `bigman canary stop` before replacing a session monitor with the complete service/timer setup.

## Optional deterministic tools

```sh
bigman setup clamav                     # preview repository installation
bigman setup clamav --apply
bigman setup shellcheck --apply
bigman setup sandbox --apply            # bubblewrap + strace
bigman setup pdf --apply                # Poppler utilities
bigman audit lynis
bigman audit osv ./project              # may query advisory services
bigman audit osquery
bigman yara ./reviewed-rules.yar ./Downloads
```

Install PDFiD/pdf-parser, OSV-Scanner, osquery and YARA-X through their upstream or distro-supported distribution methods. The PDF parser isolation profile supports executables under `/usr/bin` or `/usr/local/bin`; user virtualenv-only tools are not automatically given home-directory access. YARA-X requires a reviewed local rules file; no unreviewed rule feed is fetched.

OSV integration targets its `scan source --format=json` interface. OSV identifies known vulnerabilities, not malware; its advisory queries are an explicit network action. Lynis runs as the user, so privileged checks may be unavailable. OpenSnitch provides its own firewall interface. Bigman does not enable arbitrary firewall rules or run a second system audit collector. Socket/LLM classification is deliberately absent to keep decisions non-AI.

## Browser extension

```sh
cd browser-extension
npm ci --ignore-scripts --no-audit --no-fund
npm run build
```

Load the `browser-extension` directory as an unpacked extension in Chromium's Extensions page. The generated `dist/link-detector.js` bundles the maintained Public Suffix List library; the unbuilt source alone cannot be loaded. Copy its extension ID, then register the installed native host:

```sh
./install-native-messaging.sh YOUR_32_CHARACTER_EXTENSION_ID
```

Registration uses an explicit Chrome extension origin and the actual binary location. Chrome and Chromium user host manifests are generated; wildcard origins and the old checked-in machine-specific manifest were removed. Firefox, Flatpak browser bridges and other browsers need separate packaging and are not claimed supported here.

In the popup, check the desktop connection. Download scanning is off until explicitly enabled in Settings. It scans completed downloads, queues jobs in local extension storage, and reports missing hosts/failures. It does not prevent auto-opening a download. The native host accepts only regular download files under `~/Downloads`; arbitrary folders require explicit desktop selection. Installer preview requests from the browser accept HTTPS URLs only. Native messages are capped at 64 KiB and report frames stay below Chrome's response limit. There is no TCP listener.

Blocklists are imported locally as domains or exact HTTP(S) URLs. Domain entries match on label boundaries and include subdomains. Imports replace the old list and display their timestamp; they do not refresh automatically. Hostname exceptions last one hour and cannot suppress blocklist findings. “Continue once” is a deliberate bypass. Internationalized domains and HTTP alone receive informational results, not phishing verdicts.

The main-page popup summarizes the main document. Content scripts also check accessible frames and recheck links at click time. The extension cannot intercept all scripted navigation, redirects, typed addresses, forms, inaccessible frames or privileged browser pages. In-page markings are advisory and can be removed by a hostile page. Browser-native protections should stay enabled.

## Data and development

State defaults to `$XDG_DATA_HOME/bigman` or `~/.local/share/bigman`. Use an absolute `--state /path/to/private-directory` for separate profiles/tests. Directories are mode 0700 and report/recovery files mode 0600. Writes use temporary files and atomic replacement. Existing broad-permission or symlinked state directories are refused. This protects against other ordinary users, not compromise of the same account or root.

The Cargo workspace has five crates. `crates/core` holds the report model, private store, bounded process runner, daemon contract (`ipc`) and one module per check under `checks/`; it has no binaries and no UI. `crates/cli` is the `bigman` command; `bigman gui` opens the desktop app. `crates/daemon` is `bigmand`, the per-user daemon that runs every check for the app: it listens on `$XDG_RUNTIME_DIR/bigman/bigmand.sock` (mode 0600, peer uid checked; `BIGMAN_SOCKET` overrides the path), and a client that disconnects cancels its running check. `bigman daemon status` queries it. `crates/app` is the desktop app. `crates/native-host` is the browser native-messaging bridge. Rules have stable IDs and local evidence: `crates/core/src/rules/mod.rs` registers every rule with its explanation, remediation and the threats it covers, [docs/rules.md](docs/rules.md) is generated from it, and `fixtures/<rule-id>/` holds a synthetic input that must fire exactly its listed rules (`cargo test -p bigman-core --test fixtures`). There is no telemetry from Bigman; explicitly launched upstream tools may have their own network behavior.

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked
python3 tests/cli_integration.py target/debug/bigman
python3 tests/canary_onboarding.py target/debug/bigman
python3 tests/cli_golden.py target/debug/bigman
python3 tests/daemon_integration.py target/debug
cd browser-extension && npm run check
```

### Desktop app

The new desktop app lives in `crates/app`: a Tauri 2 shell (`src-tauri`) and a React + TypeScript UI (`ui`). It is a thin client of `bigmand`; it never runs a check itself and starts the daemon if it is not answering. Every state change is a previewed action (`crates/core/src/actions.rs`): the daemon describes exactly what will change, you confirm, then it applies.

```sh
# Fedora build dependencies for the Tauri shell
sudo dnf install webkit2gtk4.1-devel libsoup3-devel javascriptcoregtk4.1-devel libappindicator-gtk3-devel librsvg2-devel openssl-devel

cd crates/app/ui
npm ci --ignore-scripts
npm run dev:browser          # UI in a normal browser against fixture data (?scenario=calm|incomplete|empty|disconnected)
npm run check                # typecheck, lint, architecture rules, vitest
npm run build                # bundle the UI into crates/app/ui/dist
cd ../../..
cargo build                  # bigman, bigmand
cargo run -p bigman-app      # the desktop app, using the built UI
# Hot reload: npm --prefix crates/app/ui run dev, then cargo run -p bigman-app --no-default-features
```

`http://127.0.0.1:1420/#/gallery` shows every design-system component in every state, light and dark. The TypeScript types for the daemon contract are generated from the Rust types by `cargo test` into `crates/app/ui/src/ipc/generated`; never edit them by hand. `tests/app_smoke.mjs` drives the built app through `tauri-driver` (install it with `cargo install tauri-driver`; it needs `WebKitWebDriver`). `cargo deny check` enforces the licence policy in `deny.toml`.

`tests/cli_integration.py` uses synthetic files and a scanner contract fixture. `tests/cli_golden.py` pins the exact text and JSON output of `doctor`, `inspect`, piped inspection and `packages` against `tests/golden/`; rerun it with `--update` only after an intentional output change. `tests/daemon_integration.py` drives the desktop app's workflows (scan, previewed quarantine and restore, inspection, package policy, baseline, canaries, history) against a real `bigmand` over its socket. `tests/canary_onboarding.py` exercises real monitoring, fresh-read verification, repeat setup, failure fallback and stop behavior; its service manager is a fixture so tests do not change the host’s login configuration. Rust tests exercise real inotify events and filesystem safeguards. Browser tests cover host parsing, Unicode, mismatches, exceptions, ClickFix rules and DOM mutations. `tests/browser_smoke.cjs` additionally exercises the built bundle with real Chrome click events using Playwright (set `BIGMAN_PLAYWRIGHT_MODULE` and optionally `BIGMAN_CHROME`). CI runs Rust/CLI and extension checks and builds an extension artifact.

The historical [review](REVIEW.md) explains the original prototype and proposals. This README describes the implemented interfaces and their limits. Advanced host-wide credential-read attribution, privileged on-access blocking, incident cleanup, Windows/macOS support, provider-side token provisioning and enterprise monitoring servers are not implemented or implied by this release.
