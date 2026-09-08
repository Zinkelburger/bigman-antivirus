# Bigman Security Companion

Bigman makes a set of deterministic Linux security checks easier to use. It has a desktop app, a CLI, and an optional Chromium extension. There is no AI model, automatic file upload, or automatic deletion of detections.

Results say **findings to review**, **no findings from these checks**, or **incomplete**. A scan cannot certify that a file or machine is safe. Capability rules explain behavior without equating every download, permission change, or unfamiliar hostname with malware.

## Start

Requirements: Linux, a current stable Rust toolchain, and a graphical desktop for the GUI. Build checks were run with Rust 1.98. The CLI works without a graphical session. Optional system tools are discovered at runtime.

```sh
cargo build --locked
./target/debug/bigman gui
```

For a user-local binary and application-menu entry:

```sh
./install.sh
```

This builds the locked release and installs under `~/.local`. It does not install antivirus packages, change firewall policy, register browser hosts, start monitors, or enable schedules. Use the app’s **Tools & setup** page to review and apply those supported actions.

Recommended first use:

1. Run **Check my system**.
2. Install the ClamAV group if needed, then **Update definitions** and scan Downloads.
3. Review **Startup changes** before accepting an initial baseline.
4. Create a decoy in **Canary tripwires**, start monitoring, press **Test read**, and verify a new **Canary triggered** report.
5. Enable the per-user monitor, daily scan and definition-update timers if wanted.

Package installation plans support Fedora, Debian and Ubuntu. Packages come from the user's configured repositories and administrator authentication uses polkit. Repository availability can vary; failures remain visible. Bigman does not add package repositories or disable existing security services.

## Implemented workflows

| Workflow | What it does | Important boundary |
|---|---|---|
| System checkup | Tool availability, known signature locations/age, monitor heartbeat, Git plaintext-store detection, listening sockets and selected protection status | Read-only inventory; no claim that the machine is clean |
| File scans | ClamAV checks private snapshots; findings map to original paths and SHA-256 hashes | Limits, missing tools, failures and skipped files are reported |
| Quarantine | Saves a private recovery copy, verifies the current file hash, then removes the original | Explicit per-file action, only for scan findings; no automatic cleanup |
| Restore | Verifies the recovery hash and creates the original path with mode 0600 | Never overwrites an existing file; executable permissions are not restored |
| Installer inspection | Saves exact local/HTTPS script bytes; source-linked capability rules and optional ShellCheck | Never runs scripts while inspecting; dynamic behavior remains unresolved |
| Package inspection | Reads local package manifests, referenced script files, unavailable lockfile hooks, or `.tgz` contents without extracting | npm/pnpm-focused; no lifecycle or build commands invoked |
| Project script policy | Previews and writes `ignore-scripts=true` to project `.npmrc`, retaining a backup | Explicit `npm run` and command-line overrides can still execute code; this is not package trust certification |
| Restricted rehearsal | Runs an explicitly approved saved installer hash in an offline Bubblewrap environment, optionally traced with strace | No host install or promotion; restrictions can prevent completion; not a malware-detonation service |
| PDF triage | Escaped/raw PDF names, PDFiD, pdf-parser statistics, pdfinfo links | External tools require Bubblewrap; compressed/encrypted or unsupported content may remain unexamined |
| Startup changes | Compares selected user/system startup files and symlink targets with a stored baseline | Baseline records prior state, not trust or writer attribution |
| Local canaries | Synthetic token files with inotify read/change/removal alerts, test/disarm, durable events, desktop notifications and health heartbeat | Monitoring only while running; no reliable reader identity or access blocking |
| Hosted canary shortcuts | Saves an HTTPS token URL supplied by the user | Bigman does not provision a provider account or token; the provider triggers only when the URL is activated |
| Optional integrations | Lynis report findings, OSV advisory matching, osquery process JSON, user-supplied YARA-X rules, OpenSnitch launch/setup | External tools must be installed; adapters preserve failures and raw evidence |
| Browser checks | Text/destination mismatch, lookalikes, selected Unicode confusables, ClickFix text combinations, user-imported blocklists, warning pages and expiring exceptions | Limited page/link heuristics; no remote reputation requests or complete navigation interception |
| Background services | Rootless monitor, daily Downloads scan and private ClamAV update timers | Explicit installation/removal; follows user session; no lingering or system services enabled |
| History/export | Private JSON reports, coverage, errors and evidence shared by CLI, GUI and native host | Export is explicit; reports can contain local paths and inspected source |

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
# The desktop provides a Quarantine button for each matched item.
bigman quarantine REPORT_ID ITEM_INDEX
bigman quarantined
bigman restore QUARANTINE_ID
```

## Canaries and monitoring

```sh
bigman canary create ~/Documents/backup-access.env
bigman canary list
bigman watch
# In another terminal, use the ID returned by create/list:
bigman canary test CANARY_ID
bigman history
bigman canary disarm CANARY_ID
```

Creation is exclusive: an existing file, directory or symlink is not overwritten. The local token is a UUID-based decoy with no working privileges. Pick a location that ordinary applications rarely read. Test reads, backups, search indexers, antivirus tools and synchronization software can produce legitimate alerts. Bigman's own scanner skips registered decoys.

The monitor watches parent directories so replacements can be observed. It reports queue overflow, missing paths and watcher failures; writes a heartbeat; and groups repeated events for a canary within three seconds. Files copied or read while the monitor is stopped cannot be reconstructed later. inotify does not provide a reliable reader PID or tell whether data was exfiltrated. Host malware with the same user privileges can interfere with this application or its state.

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
systemctl --user status bigman-monitor.service bigman-scan.timer bigman-update.timer
```

The monitor compares persistence snapshots every five minutes. Review the initial inventory and establish a baseline first. Existing state remains after disabling services. A monitor launched directly from the GUI continues after the window closes; stop that process before switching to the service to avoid two instances competing for the monitor lock.

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

Source modules separate reports/storage, process execution, inspection, scans, canaries, system checks, setup, native protocol and GUI. Rules have stable IDs and local evidence. There is no telemetry from Bigman; explicitly launched upstream tools may have their own network behavior.

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked
python3 tests/cli_integration.py target/debug/bigman
cd browser-extension && npm run check
```

`tests/cli_integration.py` uses synthetic files and a scanner contract fixture. Rust tests exercise real inotify events and filesystem safeguards. Browser tests cover host parsing, Unicode, mismatches, exceptions, ClickFix rules and DOM mutations. `tests/browser_smoke.cjs` additionally exercises the built bundle with real Chrome click events using Playwright (set `BIGMAN_PLAYWRIGHT_MODULE` and optionally `BIGMAN_CHROME`). CI runs Rust/CLI and extension checks and builds an extension artifact.

The historical [review](REVIEW.md) explains the original prototype and proposals. This README describes the implemented interfaces and their limits. Advanced host-wide credential-read attribution, privileged on-access blocking, incident cleanup, Windows/macOS support, provider-side token provisioning and enterprise monitoring servers are not implemented or implied by this release.
