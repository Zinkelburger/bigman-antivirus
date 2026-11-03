# Bigman implementation plan

> The single plan for building Bigman into a beautiful, calm, always-on security app for Linux desktops. Written 2026-10-29 for an implementing agent working in a fresh session. Read this whole file before writing code. It supersedes the earlier `docs/plan/app-plan.md`.

Companion material in this repository:

- `README.md`: what exists today and the boundaries each workflow promises. Those boundaries stay true.
- `docs/threats/`: 182 catalogued threats with IDs (PH-, SC-, DP-, HI-, MW-, CR-, NE-, DS-). Features below cite them.
- `docs/research/roadmap.md`, `linux-av-landscape-and-papers.md`, `vulnerability-intelligence-and-feeds.md`: the research this plan is built from. Section 4 of the feeds document is the advisories design.
- `src/`: the current Rust code. It becomes the `core` crate almost unchanged.

---

## Part A. What Bigman is

**One sentence.** Bigman is GlassWire for Linux security: a pretty, always-running app that shows what is happening on your machine, inspects things before you run them, explains what the system's own defenses are doing, and tells you what to update, without ever pretending to know the machine is safe.

**Who it is for.** One person on one Linux workstation: desktop users, developers and open-source maintainers. Not fleets, not servers, not security teams. Wazuh, Velociraptor and the commercial EDRs serve those.

**Product principles.** These are hard constraints, inherited from the README and the research, and the implementing agent must not relax them.

1. **Deterministic.** Every finding comes from a rule with a stable ID, an explanation, and a fixture. No ML in the detection path.
2. **Three honest results.** Every check ends in "Findings to review", "No findings from these checks" or "Incomplete". The word "safe" never appears in the UI, docs or code comments as a claim.
3. **Observe, explain, preview.** Enforcement belongs to SELinux, AppArmor, Flatpak, OpenSnitch, firewalld and the package manager. Bigman watches, highlights what is new, explains in plain language, and offers reversible changes through the subsystem's own mechanism, always previewed first.
4. **Rootless by default.** Anything needing privilege is an explicit, previewed, user-approved action with a printed scope. One optional read-only root helper is allowed (Part E).
5. **Nothing leaves the machine** unless the user configures it and sees what will be sent. No telemetry, no uploads, no automatic deletion.
6. **No kernel modules, no own eBPF, no SUID binaries.** Bigman imports other tools' telemetry instead.
7. **AGPL-3.0-only.** GPL-2.0-only code and Elastic License 2.0 rules are called as subprocesses or imported by the user. `cargo deny` enforces licence compatibility in CI.
8. **Clean and maintainable beats feature count.** A feature that cannot be added without touching another feature's code is designed wrong.

---

## Part B. UI and UX contract

This part comes before the architecture on purpose. The app is judged on whether a non-expert opens it, understands the state of their machine in five seconds, and is never frightened or lied to.

### B1. Design process with `/impeccable`

The `impeccable` skill is installed in this environment and is the design authority. Use it at these points, not continuously:

1. **Before any UI code:** `/impeccable init` to write `PRODUCT.md` from this plan and the README. Then `/impeccable shape` for the app shell and the Overview tab, and again for each tab before building it. Shape plans UX before code and chooses the visual world. Mode is **Operate** for every surface in this app.
2. **After each tab is built:** `/impeccable critique` on that tab, fix what it finds, then `/impeccable audit` for accessibility, performance and responsiveness.
3. **Once the shell and three tabs exist:** `/impeccable document` to write `DESIGN.md` from the real code, and `/impeccable extract` to pull anything ad-hoc back into the design system.
4. **Before each milestone demo:** `/impeccable harden` (error, empty and edge states), `/impeccable onboard` (first run), then `/impeccable polish`.
5. **Never** `overdrive`, `bolder` or `delight` on this app unless the owner asks. The brief is calm.

The skill's craft-floor reference applies to every UI edit. The non-negotiables below are the brief it must honor.

### B2. The brief

- **Feel.** Calm, precise, trustworthy. The reference points are GlassWire's live graph and timeline, and the quiet density of a good system monitor. Not a SOC console, not a consumer antivirus with red shields and scare copy.
- **Hero per tab.** Overview: five tiles and a short timeline. Network: the live graph. Activity: the timeline. Inspect: the receipt or report being read. Sandboxes: the confinement table. Protect: the hardening status line (its first sub-view). Setup: a checklist. Each tab has exactly one thing the eye lands on first.
- **Color.** A neutral base with one accent for interactive elements. Severity colors are reserved and used nowhere else: Info (muted blue), Review (amber), High (red), plus a "first seen" marker (the accent). A calm green appears only in the status line and the tray, never as a verdict badge.
- **Type.** One sans for UI, one monospace for evidence, hashes, paths and code. At most three sizes on any screen. Tabular numerals for every count and byte figure.
- **Spacing.** 4 px base grid, 8 px rhythm. Dense tables with breathing room around them.
- **Light and dark** from day one through tokens. Follow the system preference, allow an override.
- **Motion** only where it carries meaning: the graph streaming, a drill-down panel opening, a row appearing in the timeline, a status tile changing. Respect `prefers-reduced-motion`.
- **Window.** Minimum 960 × 600. The layout must still work at that size. No horizontal scrolling anywhere.
- **Keyboard.** Digits 1 to 7 switch tabs, `/` focuses search, `Esc` closes a drill-down, arrow keys move in tables, `Enter` opens the selected row. Every action reachable by keyboard; focus visible.
- **Accessibility.** WCAG AA contrast in both themes, every icon has a label, every table has headers, severity is never conveyed by color alone (icon plus text).

### B3. The three-level rule

Every number on screen opens to a table, and every table row opens to its evidence. This is implemented once, as the `Drilldown` component, and every feature uses it. Level one is a tile or a count. Level two is a table in a side panel. Level three is evidence: a code block with highlighted lines, a file list, a diff, or a report. No feature builds its own version.

### B4. State rule

Every data view implements all six states, using the shared components, before it is considered done:

| State | Component | Behavior |
|---|---|---|
| Loading | `Skeleton` | Shape of the real content, no spinners in content areas |
| Empty | `EmptyState` | One sentence on what would appear here and one action to make it appear |
| Findings | the view | Severity-sorted, first-seen on top |
| No findings | `StatusLine` | "No findings from these checks. Ran at 14:02 with signatures from today." |
| Incomplete | `IncompleteNotice` + `CoverageList` | What was skipped and why, from the `Coverage` type, never a free-text error |
| Disconnected | `IncompleteNotice` | Background collection is not running; one action to start it |

### B5. Copy rules

- Plain language, second person, present tense. "Firefox connected to a new host" not "New outbound connection detected from process firefox".
- Every finding shows three things in this order: what was found, the evidence, what you can do.
- No alarm words: no "threat", "attack", "infected", "danger", "critical". Severity labels are exactly Info, Review, High.
- No reassurance words: no "safe", "clean", "protected", "secure". The status line format in B4 is the only calm state.
- Every state-changing action shows a preview of exactly what will change, then a result, and has an undo or revert where the subsystem allows it.
- Numbers get units and tabular figures. Bytes are shown in the unit that keeps the number under four digits.
- Error text from tools is shown verbatim inside an `EvidenceBlock`, never paraphrased into prose.

### B6. Component vocabulary

The whole UI is built from these and nothing else. Adding a component requires a design-system entry, a story in the component gallery, and a reason the existing ones cannot do it.

| Group | Components |
|---|---|
| Shell | `AppShell` (sidebar, content, status strip), `PageHeader` (title, time-range picker, primary action), `TrayMenu` |
| Status | `StatusTile`, `StatusLine`, `SeverityBadge`, `FirstSeenMarker`, `CoverageList`, `IncompleteNotice` |
| Data | `DataTable` (virtualized, sortable, filterable, keyboard navigable), `Timeline` (rows grouped by day, first-seen folding), `KeyValue`, `Drilldown` (side panel, two depths) |
| Evidence | `EvidenceBlock` (monospace, line highlights, copy button, wraps control characters as today's `model::clean` output), `DiffView` (hunks with rule highlights), `FileTree` (grouped by category) |
| Charts | `ThroughputChart` (uPlot, streaming, time-range aware), `Sparkline` |
| Actions | `ActionButton`, `PreviewDialog` (plan, confirm, result), `ConfirmDanger` |
| Feedback | `EmptyState`, `Skeleton`, `Toast`, `ProgressRow` (for long scans, with cancel) |
| Input | `PathPicker`, `Toggle`, `Select`, `SearchField` |

Twenty-six components, plus `Tally` (the life-list repeat mark inside `Timeline`) and the native `TrayMenu`. If the count grows past thirty-five, stop and run `/impeccable distill`.

### B7. Navigation

Seven tabs in a left sidebar, in this order: Overview, Network, Activity, Inspect, Sandboxes, Protect, Setup. A thin status strip at the bottom shows daemon state, signature age and the last event time. The tray icon has three states: quiet, attention (unread first-seen events), problem (daemon down or signatures older than seven days). Its menu: Open, Pause notifications for 1 hour, Quit.

Notifications: desktop notifications only for first-seen events in categories the user enabled, batched within a 30 second window, never more than one per minute, with a Setup toggle per category.

---

## Part C. Architecture

### C1. Processes

```
┌──────────────┐   unix socket, JSON    ┌───────────────────┐
│ bigman-app   │◄──────────────────────►│ bigmand           │  user service (systemd --user)
│ Tauri 2 + TS │                        │ collector daemon  │  all Sources, scheduler,
└──────────────┘                        │                   │  socket server, store owner
┌──────────────┐                        │                   │
│ bigman (CLI) │◄──────────────────────►│                   │
└──────────────┘                        └─────────┬─────────┘
┌──────────────┐                                  │
│ native host  │ (browser extension, stdio)       ▼
└──────────────┘                        ┌───────────────────┐
                                        │ store             │  SQLite: events, samples, receipts
                                        │                   │  JSON files: reports, evidence
                                        └───────────────────┘
                                                  ▲
                      ┌───────────────────┐       │       ┌───────────────────┐
                      │ bigman-helper     │───────┴───────│ opensnitch daemon │
                      │ optional, root    │               │ external, if      │
                      │ read-only, polkit │               │ installed         │
                      └───────────────────┘               └───────────────────┘
```

Rules:

- **The daemon owns the store.** Clients read through the daemon, never the SQLite file directly. The CLI may read JSON reports directly for the `history` command, as today, because that path already exists and is tested.
- **The app never runs a check.** Every scan, inspection and collection runs in the daemon. If the daemon is unreachable the app starts it as a child process and shows the Disconnected state until it answers. There is one code path for collection, not two.
- **Thin clients.** The CLI, app and native host contain no check logic. If a feature cannot be exercised from the CLI it is not in core.
- **The helper is separate** (Part E). It has its own socket and its own crate and only the daemon talks to it.

### C2. Workspace

```
bigman/
  Cargo.toml                       workspace
  crates/
    core/                          library: no binaries, no UI, no process spawning outside runner
      src/model/                   Report, Finding, Event, Sample, Receipt, Coverage, Severity
      src/store/                   SQLite (rusqlite, bundled) + JSON report files; schema version; migrations
      src/rules/                   registry: Rule { id, severity, title, explanation, remediation, threat_ids }
      src/checks/                  one module per check (scan, inspect, packages, pdf, changes, doctor,
                                   canary, repos, advisories, receipts, sandboxes, network, credentials, posture)
      src/sources/                 Source trait + one impl per timeline feed
      src/platform/                traits + linux/ implementation (C4)
      src/ipc/                     Request, Response, Stream types; client; server framing
      src/runner.rs                bounded subprocess runner (exists today; keep)
    daemon/                        bigmand: scheduler, Source loop, socket server, helper client, OpenSnitch UI server
    cli/                           bigman: clap commands, thin client; keeps today's exit codes and --json
    native-host/                   stdio bridge for the browser extension; thin client
    helper/                        bigman-helper: root, read-only, allowlisted queries
    app/
      src-tauri/                   Tauri 2 shell: commands forward to daemon client; tray; notifications
      ui/                          React + TypeScript (C3)
  fixtures/                        one directory per rule id; synthetic, benign
  browser-extension/               unchanged
  docs/
```

Mapping from today's `src/`: `model.rs` and `store.rs` become `model/` and `store/`; `scanner.rs`, `inspect.rs`, `pdf_scanner.rs`, `system.rs`, `canary.rs`, `analyzer.rs` become modules under `checks/`; `runner.rs` stays; `setup.rs` splits into `checks/setup` (plans) and daemon install code; `ipc.rs` becomes `native-host`; `gui.rs` is deleted once the Tauri app reaches parity.

### C3. Frontend

Stack: React 19, TypeScript strict, Vite, Tailwind with tokens defined once in `ui/src/design/tokens.css`, TanStack Query for all IPC reads, TanStack Table with virtualization, uPlot for the throughput chart, Zustand for UI-only state (selected tab, time range, open drill-down), Radix primitives under the action and dialog components. Tauri plugins: tray, notification, window-state, opener. Not used: shell, fs, http.

Folder rules:

```
ui/src/
  design/        tokens.css, theme.ts, primitives (the B6 components), gallery route (/gallery renders every component in every state)
  ipc/           the only folder that imports @tauri-apps/api; typed hooks: useReport, useEvents, useSamples, useReceipt, useAction
  features/
    overview/    one folder per tab; pages, feature components, queries, tests
    network/
    activity/
    inspect/
    sandboxes/
    protect/
    setup/
  shell/         AppShell, navigation, status strip, keyboard handling
  mock/          fixture-backed implementation of the ipc hooks for browser dev mode
```

- A feature folder never imports another feature folder. Shared code goes to `design/` or `ipc/` and must carry no feature knowledge.
- Feature code never imports `@tauri-apps/api`. This keeps browser dev mode and the Electron escape hatch real.
- `npm run dev:browser` runs the UI in a normal browser against `mock/` with fixture data. Most UI work happens there. (The plan originally said pnpm; the repository already uses npm with a lockfile for the browser extension, so the app uses npm too.)
- TypeScript types for the contract are generated from the Rust types with `ts-rs` (chosen over `tauri-specta`, whose Tauri 2 line was still pre-release) by `cargo test` into `ui/src/ipc/generated`; CI fails if they are stale. `scripts/check-architecture.mjs` fails the build on hand-written contract types and on the folder rules below.
- Component gallery at `/gallery` is the design-system source of truth and the input for `/impeccable critique`.

### C4. Platform traits

Everything OS-specific sits behind traits in `core/src/platform/` with only a Linux implementation. Rules consume parsed data and never touch paths or commands.

| Trait | Linux implementation | Left open for later |
|---|---|---|
| `PackageDb` | rpm, dpkg, pacman queries; package caches; transaction logs (milestone 4 built the history half as `PackageHistory`: dnf5's `transaction_history.sqlite` read-only, else the pacman or dpkg log) | winget, Homebrew |
| `StartupLocations` | the baseline path list | Run keys, LaunchAgents |
| `NetworkCounters` | `/sys/class/net/*/statistics` | OS APIs |
| `ProcessSockets` | `/proc/net` + `/proc/<pid>/fd` inodes | OS APIs |
| `ServiceManager` | systemd user units and timers | Task Scheduler, launchd |
| `Notifier` | D-Bus desktop notifications (through `notify-send`, raised by the daemon so they arrive with the window closed) | native toasts |
| `PrivilegeBroker` | polkit | UAC, authorization services |
| `Confinement` | SELinux, AppArmor, Flatpak, Snap metadata, `systemd-analyze security` | app sandboxes |
| `Containers` | Docker and Podman inspect JSON | same APIs |
| `Journal` | journald (audit and kernel transports, read with `journalctl -o json` and a saved cursor when the user is in systemd-journal, adm or wheel) and `/var/log` through the helper | event logs |

A rule that applies to one OS declares it in the registry and reports "not applicable" elsewhere.

### C5. The contract

Rust types in `core/src/model/`, exported to TypeScript. Sketches, not final code:

```rust
pub enum Severity { Info, Review, High }

pub struct Finding {
    pub rule: String,            // registry id, e.g. "receipt.hook.new"
    pub severity: Severity,
    pub summary: String,
    pub evidence: String,        // cleaned (model::clean) before storage
    pub action: String,
    pub evidence_ref: Option<EvidenceRef>,   // file, line range, receipt hunk
}

pub struct Coverage {
    pub checked: Vec<String>,
    pub skipped: Vec<Skipped>,   // { what, why: SkipReason }  reasons are an enum, not prose
}

pub struct Report {              // exists today; gains Coverage
    pub id: String, pub created: u64, pub kind: String, pub subject: String,
    pub sha256: Option<String>, pub findings: Vec<Finding>,
    pub coverage: Coverage, pub errors: Vec<String>, pub details: String,
}

pub struct Event {               // one timeline row
    pub id: i64, pub at: u64,
    pub source: SourceId,        // enum: Network, Receipt, Startup, Listener, Canary, Denial, Flatpak, Container, Scan, Advisory, Extension, Import
    pub key: String,             // first-seen key, see Source trait
    pub severity: Severity,
    pub summary: String,
    pub seen_before: u32,        // 0 = first seen
    pub link: Option<Link>,      // Report(id) | Receipt(id) | Evidence(ref)
}

pub struct Sample { pub at: u64, pub iface: String, pub rx: u64, pub tx: u64, pub pid: Option<u32> }

pub struct Receipt {
    pub id: String, pub at: u64, pub manager: PackageManager,
    pub packages: Vec<PackageChange>,        // name, old, new, kind: Install|Upgrade|Remove
    pub files: Vec<FileChange>,              // path, category, kind, package, mode, owner
    pub scripts: Vec<Script>,                // package, phase, text, findings
    pub metadata: Vec<MetadataDelta>,        // new deps, new services, setuid, capabilities, keys
    pub after: AfterEffects,                 // new listeners, new units, new autostarts observed within 60 s
    pub findings: Vec<Finding>,
    pub not_reviewed: Vec<Skipped>,
    pub reviewer: Option<ReviewerOutput>,    // Part G
}
```

Requests are one enum (`Scan { path }`, `Inspect { target }`, `Changes`, `Receipt { id }`, `Events { range, filter }`, `Subscribe { events: bool, samples: bool }`, `Action { kind, preview: bool }` ...). Responses carry either a value or a structured error with a `Coverage`. Streams are pushed as `Event` and `Sample` frames and forwarded into the webview with Tauri channels.

### C6. Sources

```rust
pub trait Source: Send {
    fn id(&self) -> SourceId;
    fn interval(&self) -> Duration;                       // or Watch for inotify/journal-driven sources
    fn collect(&mut self, ctx: &Ctx) -> Result<Vec<Observation>>;
    fn first_seen_key(&self, o: &Observation) -> String;  // what makes two observations "the same thing"
    fn explain(&self, o: &Observation) -> Explanation;    // summary, evidence, action, severity
    fn remediation_preview(&self, o: &Observation) -> Option<Plan>;
}
```

The daemon runs every Source on its interval, turns each observation into an `Event` with `seen_before` computed from the store, and emits it. Adding a source means one module and its fixtures. It must not require UI changes: the Activity tab renders any `Event` and the `SourceId` enum drives its filter chips automatically.

As built in milestone 4 (`core/src/sources/`), the trait is leaner: `name()`, `interval()` and `collect(&mut self, &Ctx) -> Result<Collected, Gap>`. An `Observation` already carries its first-seen key, summary, evidence, severity and rule, because each source knows those when it sees the thing; separate `first_seen_key` and `explain` methods only split one decision across three places. `Gap` is a source that cannot see right now (an unreadable journal), reported as coverage in Setup and Activity rather than as silence. Remediation previews stay with `actions::Action`. Sources that resume across restarts keep a cursor in `settings`; the first run of the connections and listeners sources records what is already there without announcing it. Every source runs on its own daemon thread, and pushes reach clients through a `Subscribe` request on a dedicated connection, forwarded into the webview with a Tauri channel. `ProcessSockets` reads `/proc/net/{tcp,tcp6,udp,udp6}` and the fds of your own processes rather than parsing `ss` output. `Sample` carries bytes per second as `f64` per interface; per-process samples wait for the helper (milestone 10).

### C7. Store

- SQLite via `rusqlite` with the bundled feature. Tables: `schema_version`, `events`, `samples` (downsampled after 24 h to 1 min, after 30 d to 1 h), `receipts`, `first_seen` (key, source, first_at, count), `settings`.
- JSON report files and evidence stay as today under the private store directory with mode 0600, since the CLI integration tests and quarantine code depend on them.
- Forward-only migrations in `store/migrations/`, each tested against a fixture database committed from the previous release. (Milestone 4 shipped migration 0001; the first fixture database is committed with the first release that adds a migration. `Db::open` refuses a database newer than the binary.)

---

## Part D. Features by tab

Each feature lists threat IDs and an acceptance test. **have** = exists today and is ported, **new** = this plan, **later** = behind a Setup action or a decision.

### D1. Overview

- **have** Bigman's tools tile (signatures age, canary tripwires, optional tools; Bigman's own readiness, so neutral, never a severity badge) and last-scan tile. Renamed from "Protection status" by the owner on 2026-10-30 after the first critique: a missing signature set showed a red High on first launch.
- **new** "Since you last looked" tile: unread first-seen events, opens Activity filtered.
- **have** Hardening tile: how many suggestions are open and the most important one, opens Protect > Hardening.
- **new** Updates tile: pending updates with CVE count and KEV count, opens Protect > Advisories.
- **new** 24-hour compact timeline below the tiles, same rows as Activity. Until activity collection exists (milestone 4) it lists the user's own checks from report history; "first seen" there means the first report of that check on that subject within the loaded history.
- **new** Calm state per B4.

Acceptance: on a clean VM with the daemon running, Overview renders five tiles, the status line, and an empty timeline with an `EmptyState` explaining what will appear.

### D2. Network

- **new** `ThroughputChart` per interface from `NetworkCounters`, 1 s samples, history from the store, time ranges 5 min, 1 h, 24 h, 7 d.
- **new** Connections table for the user's own processes from `ProcessSockets`: process, destination, port, state, first seen. First-seen rows marked. Rootless.
- **new** First-seen events into Activity: process connects for the first time, known process reaches a new destination, new listening socket. NE-13.
- **new** Listener findings with owning unit, binary and package: wildcard binds, Docker API on TCP, published container ports bypassing the firewall, local AI or debug servers reachable from the LAN. NE-01, NE-06, NE-07, NE-09 to NE-12, MW-03.
- **new** Firewall view: firewalld or ufw zones, open services, with previewed close actions through polkit. NE-05.
- **new (needs OpenSnitch daemon)** Bigman as the OpenSnitch UI. The daemon connects to a UI server over gRPC; `bigmand` implements that server, the stock Qt UI is not installed, and prompts, connections for all users, and rules render in Bigman's design. Setup installs the daemon and configures its UI socket address. The adapter pins supported daemon versions and fails visibly on mismatch. OpenSnitch is GPL-3.0 and is only spoken to over its protocol.
- **later (needs helper)** Per-process bandwidth samples.

Acceptance: close the window for ten minutes, reopen, the 1 h graph shows the gap-free history; a new process connecting appears as a first-seen row within 5 s; a fixture `/proc/net/tcp` dump with a wildcard bind on 0.0.0.0:8080 produces rule `net.listener.wildcard`.

### D3. Activity

- **new** The timeline. Rows grouped by day, newest first, first-seen on top within a day, recurring events folded into their first occurrence with a count. Filter chips per `SourceId` and per severity. Search.
- **new** Every row drills down (B3) to its report, receipt or evidence.
- **have** Scan history and quarantine appear as `Scan` events; the Quarantine list is a Protect sub-view.
- **new** Mark all as seen; unread count feeds the Overview tile and the tray.

Acceptance: 10,000 fixture events scroll at 60 fps in the virtualized list; a folded row shows "seen 14 times, first on Oct 3".

### D4. Inspect

- **have** Installer inspection from path, URL or stdin; exact bytes saved; capability rules; ShellCheck when present.
- **have** npm package and project inspection; `ignore-scripts` policy with backup; offline Bubblewrap rehearsal of an approved saved hash.
- **have** PDF triage.
- **new** Install receipts (Part F).
- **new** System package files without installing: `.deb`, `.rpm`, PKGBUILD, AppImage; maintainer scripts through the shell rules; files dropped into repo, key, cron, systemd or preload paths; setuid bits. DP-08, DP-09, DP-11, DP-17, DP-18.
- **new** Language packages beyond npm: wheels and sdists including `.pth` files, crates with build scripts and proc-macros, `.vsix`, pnpm, yarn and bun lockfiles, `binding.gyp`. SC-01, SC-03, SC-04, SC-19.
- **new** Lockfile linter with diff mode: non-registry dependencies, missing integrity hashes, new packages with install hooks. SC-02, SC-09, SC-10, SC-13.
- **new** `repo-check DIR`: IDE auto-run tasks, `.envrc`, devcontainers, git hooks and filters, AI-tool configs (`.claude/`, `.mcp.json`, `.cursor/`, instruction files with hidden Unicode), workflow risks (unpinned actions, `pull_request_target`). SC-15, SC-16, SC-20 to SC-22, PH-10, CR-19.
- **new** Repository and channel audit: apt including deb822, yum.repos.d, zypp, `pacman.conf`, Flatpak remotes, Nix. Disabled signatures, plain HTTP, keys without `signed-by`, stale metadata, third-party repos overriding distro packages. DP-02, DP-03, DP-05, DP-10, DP-12, DP-22.
- **new** Archive and launcher checks: list-only path traversal, symlink escapes, startup-path entries, `.desktop` launchers disguised as documents. DS-09, DS-13.
- **new** Downloads watch: inotify on the Downloads directory routes new files to the right inspector or to a ClamAV scan automatically and posts a first-seen event. Replaces the daily timer as the default.
- **later** AI reviewer on any receipt or inspection (Part G).

Acceptance: one fixture per rule id fires its rule and no other; `bigman inspect` output for the existing fixtures is byte-identical to today's.

### D5. Sandboxes

What is confined, how, and what it tried to do.

- **new** Confinement table: every running process of the user with its sandbox: Flatpak, Snap, container, systemd unit with its `systemd-analyze security` score, Bubblewrap, or none. The hero of the tab.
- **new** Flatpak permission audit from app metadata and user and system overrides. Each permission explained in one line with a rank: `filesystem=host` or `home`, `--talk-name=org.freedesktop.Flatpak` (full escape), `--socket=x11` (input of other windows visible), `--device=all`, session or system bus, network combined with home access. Unverified Flathub publishers, end-of-life runtimes. DS-03, DP-13.
- **new** Flatpak permission changes on update as first-seen events, fed by the receipt pipeline. DS-05.
- **new** Flatpak override editor, rootless, writing the same override files Flatseal does, with preview and revert.
- **new** Snap audit: classic and devmode confinement, connected interfaces, unverified publishers. DP-15.
- **new** Container audit for Docker and Podman: privileged, host PID, network or IPC namespaces, Docker socket or sensitive host paths mounted, added capabilities (SYS_ADMIN, SYS_PTRACE, NET_ADMIN), seccomp or AppArmor unconfined, root in the container without user namespace remapping, mutable image tags, unverified registries. Each finding names the escape path it opens. Rootless Podman read directly; Docker through socket access the user already has.
- **new** Runtime posture: rootful versus rootless, docker group membership (root-equivalent, HI-16), user namespaces enabled, default seccomp profile present.
- **new** Denials: SELinux AVC and AppArmor DENIED records from journald (readable by wheel, adm and systemd-journal members) or from the audit log through the helper. Grouped by process and target, first-seen marked, each translated to plain language ("Firefox tried to read your SSH directory and was stopped"), recurring harmless ones marked as known noise from a curated list, correlated with the timeline ("began right after the update at 14:02"). Suggested booleans or local policy modules as a previewed action, never applied automatically. HI-24.
- **new** Confinement status: SELinux or AppArmor mode, permissive domains, unconfined processes that normally have a profile, profiles in complain mode.
- **later** Runtime escape detection stays with Falco, Tracee or Tetragon; their JSON is an `Import` source.

Acceptance: a fixture container config with host PID and the Docker socket mounted yields `container.ns.host_pid` and `container.mount.docker_socket`; a fixture AVC record renders as a plain-language denial row; a Flatpak fixture with `filesystem=host` ranks High in the audit.

### D6. Protect

- **have** ClamAV scan of private snapshots, definitions update, quarantine with recovery copy and hash-verified restore.
- **have** Startup baseline with reviewed acceptance that rechecks the inventory hash.
- **new** Wider baseline: cron directories and user crontab, `profile.d`, `environment.d`, rc.local, init.d, MOTD scripts, udev rules, `pam.d`, `ld.so.conf.d`, `modules-load.d`, `~/.ssh/config`, `/etc/hosts`, `resolv.conf`, `mimeapps.list`, `.github/workflows` per known project. Records mode, owner and file capabilities. HI-01 to HI-11, MW-15, MW-18, MW-19, NE-15, CR-09, DS-10, SC-17.
- **new** Package attribution of every changed file: owned and verified, owned but modified, unowned. HI-01, HI-08, HI-22, DP-21, MW-08, MW-15. This is the noise reducer every other check depends on; build it before widening the baseline.
- **new** Advisories per `docs/research/vulnerability-intelligence-and-feeds.md` section 4: dnf5 on Fedora, Debian tracker or debsecan, Ubuntu OVAL, Arch `all.json`, Alpine secdb. KEV raises severity, EPSS shown as context, reboot-required flag, honest coverage for AUR and third-party packages, stale data and end-of-life releases. Shows the update command, never runs it. DP-01, DP-04, DP-16, DP-20, HI-23, NE-04, NE-17, DS-04, DS-11, MW-20.
- **have** Local canaries with verified read receipts; hosted canary URLs.
- **have** Hardening suggestions (`bigman harden`, Protect > Hardening): 16 `harden.*` rules for network services, the firewall, three kernel sysctls, LLMNR, screen lock, autologin, disk encryption, Secure Boot, the docker group, the home folder mode and npm install scripts. Each gives the evidence, what it protects against and what it costs; exposure takes the firewall zones into account. Fixes are daemon-built from the suggestion ID, previewed, run as one polkit prompt when they need root, and undoable; "keep as it is" stops a suggestion from counting. Reasoning and sources in `docs/research/hardening.md`. NE-01, NE-12, NE-16, NE-17, NE-22, CR-07, CR-10, HI-16, HI-21, HI-25, DP-23, DS-15, DS-22, SC-01.
- **new** More canary types: SSH key, kubeconfig, `.git-credentials`, `.npmrc` in a decoy project, cloud profile. CR-18, CR-22.
- **new** Credential inventory, metadata only: presence, mode, owner and age of token-like entries in git, npm, PyPI, cargo, Docker, gh, cloud and kube configs; passphrase-less SSH and GPG keys detected from headers only. Never values. CR-01 to CR-04, CR-08, CR-10, CR-16, CR-20.
- **new** Posture (Secure Boot state, screen lock, autologin and three sysctls are done in Hardening): `fwupdmgr security` rating, Secure Boot and lockdown, kernel taint, module signing, session type, USBGuard, screen lock and autologin, attack-surface sysctls. HI-20, HI-21, HI-25, DP-23, DS-01, DS-14, DS-15, DS-18, MW-16, MW-17.
- **new** Installed-file verification wrapping `rpm -Va`, `dpkg --verify`, `pacman -Qkk`, config-file noise explained, higher severity for sshd, sudo, PAM and the dynamic linker.
- **new** SUID, SGID and file-capability inventory against the package database. HI-17 to HI-19.
- **new** Account audit: extra UID 0, new accounts, root-equivalent groups. HI-12, HI-16.
- **new** Process checks for the user's own processes: deleted-but-running or memory-only executables, name not matching binary path, shells wired to a socket, PATH entries shadowing system tools. MW-02, MW-22, MW-23, NE-14.
- **new** Browser extension inventory for Chromium-family and Firefox profiles plus managed-policy folders: permission increases, policy-forced and unpacked extensions, user-imported bad-ID lists. DS-06 to DS-08, CR-13.
- **later (helper)** Sudoers, shadow lock status (never hashes), root cron spools, polkit `/etc` rules, `sshd -T`. HI-13 to HI-15, NE-02.
- **later** YARA-X with a small Bigman-owned rule set and user-downloaded packs that keep their licences.

Acceptance: `bigman doctor --json` on a clean VM reports no findings with every check listed in `coverage.checked`; a fixture modified `/etc/pam.d/sudo` with package attribution "owned but modified" ranks High.

### D7. Setup

- **have** Tool installation plans for Fedora, Debian and Ubuntu through polkit; service and timer install and removal; native host registration; OpenSnitch launch.
- **new** Daemon status and install as a user service.
- **new** Pre-transaction hook install for pacman, apt and dnf (Part F).
- **new** Helper install card with its full scope printed (Part E) and one-click removal.
- **new** OpenSnitch daemon install and hand-over to Bigman's UI server.
- **new** Reviewer backend configuration (Part G).
- **new** Notification categories, theme override, Downloads directory.
- **new** Feed fetch and digest for the weekly threat-intel loop from `docs/research/feeds.opml`.
- **have** History export and the privacy statement of what a report contains.

---

## Part E. The helper

`bigman-helper` is Bigman's only root component. It exists for per-process bandwidth, the audit log, and the handful of files only root can read.

- **Off by default.** Every tab works without it and shows which rows it would add.
- **Read-only by construction.** No write, kill, block, install or exec query exists in its code. Remediation stays with the polkit-gated one-off actions the app already uses.
- **Small and separate.** Its own crate, a few hundred lines, depending only on the contract types. Auditable in one sitting.
- **Allowlisted queries, no free arguments:** `net_samples`, `audit_tail { since }`, `sudoers`, `shadow_lock_status` (locked or not per account, never hashes), `root_cron`, `polkit_rules`, `sshd_effective_config`. A query name is the whole request.
- **Minimal privilege.** System service with `CapabilityBoundingSet=CAP_NET_RAW CAP_NET_ADMIN CAP_DAC_READ_SEARCH`, `ProtectSystem=strict`, `ProtectHome=read-only`, `PrivateTmp=yes`, `NoNewPrivileges=yes`, `IPAddressDeny=any`, socket in `/run/bigman/` owned by the installing user with mode 0600, peer uid checked.
- **Bandwidth method:** packet capture on the interfaces with socket-inode-to-process mapping from `/proc`, the nethogs approach. No eBPF.
- **Install card** lists every query, every path it can read, the unit file text, and the polkit action. Removal deletes the unit, binary, socket and polkit action and shows that it did.
- **Precedent to cite in the card:** OpenSnitch, fwupd, USBGuard and every firewall front end run a root service. Bigman prints its scope before you agree.

---

## Part F. Install receipts

The answer to "the package manager just ran and I have no idea what happened".

**Capture.**

- Passive, rootless, default: the daemon tails `/var/log/pacman.log`, `/var/log/dpkg.log` and the dnf history database, and reads old and new package files from `/var/cache/pacman/pkg` and `/var/cache/apt/archives`. Works for every install however it was started.
- Active, opt-in from Setup: a one-time alpm hook, apt `DPkg::Pre-Install-Pkgs` hook or dnf plugin that calls `bigman receipt --preview` before commit, shows the receipt in the app with continue or abort, and times out to continue after 60 s so an unattended upgrade is never blocked.

**Three levels.**

1. Summary card: packages, files changed, hooks run, suspicious lines, new services or timers, new listeners afterwards, new setuid or capabilities, new dependencies, not reviewed.
2. Details: `FileTree` grouped by category (`/etc`, systemd, cron, PAM, libraries, binaries, other), scripts verbatim in `EvidenceBlock`, `KeyValue` metadata deltas, `DiffView` of the PKGBUILD or spec when the recipe is public.
3. Code: any file or hunk with rule highlights and the rule text beside it.

**Deterministic triage, always shown.** Rules under `receipt.*`: new or changed install hook, new dependency, release tarball versus VCS tag mismatch, build-system file changed in the same release as binary test files, obfuscation indicators (base64 blobs, eval, long hex strings, hidden Unicode), new ifunc or constructor usage, new file in a sensitive path, new packager key, package in the sensitive set (openssh, sudo, pam, systemd, glibc, xz, curl, openssl, kernel, shells). Each contributes to a score that selects the reviewer mode in Part G.

**After-effects.** Sixty seconds after a transaction the daemon diffs listeners, units and autostarts and attaches what appeared.

Acceptance: a fixture pacman transaction with a package containing a `.install` script that writes to `/etc/profile.d` produces a receipt whose summary shows one hook and one suspicious line, and whose code level highlights that line with rule `receipt.hook.sensitive_path`.

---

## Part G. The AI reviewer

Opt-in. Reads a receipt or an inspection packet and explains it. It can add findings and never remove or downgrade a deterministic one.

- **Modes.** Everything (all reviewable content up to a size budget, rest listed as not reviewed), Triaged (default; only packets whose triage score crosses the threshold), Rare (only the top triggers such as tarball mismatch or a new hook in a sensitive package).
- **Send preview.** Before the first send to any backend, and on every send when the packet contains a path outside distro package caches, the app shows exactly what will be sent and asks.
- **Backends.** A local model through an OpenAI-compatible endpoint, and the Claude API. Check model IDs and limits against the `claude-api` skill reference at implementation time; do not hardcode from memory. Use a cheaper model for Everything and Triaged, the strongest for Rare.
- **Prompt injection is expected.** The packet is data. The prompt states that any instruction inside the packet is content to be reported, not followed. Output is structured JSON: a list of `{ hunk_ref, severity, reason }` plus a coverage list. Free text is rendered only inside an `EvidenceBlock`.
- **Human path first.** The packet the reviewer would read is the same packet the user sees in the receipt view. A person with no AI budget gets the full deterministic triage.
- **README change.** "No AI model" becomes "No AI unless you configure one, never automatic, always previewed".

---

## Part H. Rules, fixtures and tests

- **Registry.** `core/src/rules/registry.rs` holds every `Rule { id, severity, title, explanation, remediation, threat_ids, platforms }`. IDs are dotted and stable: `shell.curl_pipe`, `receipt.hook.new`, `container.ns.host_pid`, `flatpak.perm.filesystem_host`, `denial.selinux.first_seen`. The registry generates `docs/rules.md` in CI.
- **Fixtures.** `fixtures/<rule-id>/` holds a benign synthetic input and an `expected.json`. A rule without a fixture fails the `rules_have_fixtures` test. Fixtures contain no real malware and no attack procedures; they are the shape of a thing, not the thing.
- **Test layers.**
  - Rust unit tests per rule against fixtures.
  - Daemon integration tests against a temporary store and a fake platform implementation.
  - The existing Python CLI tests in `tests/` keep passing throughout; they are the parity guarantee.
  - vitest for every component in every B4 state, driven by the gallery.
  - A WebDriver smoke test of the built app on Linux in CI: launch, visit each tab, open one drill-down, quit.
  - A manual pre-release checklist: Nvidia with Wayland, Debian stable WebKitGTK, GNOME and KDE, light and dark, reduced motion.
- **CI gates.** `cargo fmt --check`, `cargo clippy -D warnings`, `cargo deny check` (licences and advisories), `cargo test`, `tsc --noEmit`, eslint, vitest, the smoke test, and `rules_have_fixtures`.
- **Public self-test.** Each release publishes which fixtures fired, following G9 in the landscape research.

---

## Part I. Tauri and security settings

- Capabilities file grants only the commands the UI calls. No shell, fs or http plugin exposure to the webview.
- Content security policy with no remote origins. The UI loads nothing from the network, ever.
- Commands take typed arguments; path validation happens in core exactly as for the CLI. The webview never passes a shell string.
- The daemon socket lives in `$XDG_RUNTIME_DIR/bigman/`, mode 0600, peer uid checked.
- Evidence is cleaned in Rust (today's `model::clean`) before it reaches the UI, and `EvidenceBlock` renders it as text, never as HTML.
- Launch sets the documented WebKitGTK renderer fallback environment variable when an Nvidia driver is detected, and the About page shows the WebKitGTK version for bug reports.

---

## Part J. Milestones

Work one milestone per branch and pull request. Each ends with a demo the owner can run with `cargo run -p bigman-app` (the default `bundled-ui` feature serves `crates/app/ui/dist`; build it with `npm --prefix crates/app/ui run build`) and the exit test green. Stop and ask at a milestone boundary if a decision in Part K is needed to continue; otherwise do not stop.

| # | Milestone | Scope | Exit test |
|---|---|---|---|
| 0 | Read and inventory | Read README, this plan, `docs/threats/README.md`, the roadmap. Run the existing tests. Run `/impeccable init`. Write `PRODUCT.md`. | Existing Python tests pass; `PRODUCT.md` committed. |
| 1 | Workspace split | Create `crates/core`, `cli`, `daemon` (socket server with `Health` only), `native-host`. Move code. CLI behaves exactly as today. egui GUI still builds. | Python CLI tests pass unchanged; `bigman --json doctor` output identical to before. |
| 2 | Design system and shell | `/impeccable shape` the shell and Overview. Tauri app with `AppShell`, tokens, the B6 components, the `/gallery` route, mock IPC, browser dev mode, tray. Overview tab with live daemon health. | Gallery shows every component in every B4 state in light and dark; `/impeccable critique` on the gallery has no High findings; smoke test green. |
| 3 | Parity | Port every README workflow into Inspect, Protect, Setup and Activity (history) using the components. Delete egui. | Every README workflow reachable; README's "Start" section updated for the new app. |
| 4 | Live | SQLite store and migrations. Source trait. Sources: network counters, process sockets, listeners, canaries, package-log tail, journal denials. Network tab tier 1. Activity timeline with first-seen folding. Notifications. | Graph shows gap-free history after a window close; first-seen within 5 s; fixture AVC renders as a plain-language row; 10,000 events scroll smoothly. |
| 5 | Attribution and receipts | Package attribution. Wider baseline with mode, owner and capabilities. Passive receipts with the three levels. Active hook installer in Setup. | The Part F acceptance test; a modified PAM file ranks High with "owned but modified". |
| 6 | Advisories | Distro advisory matching, KEV, reboot flag, Overview updates tile, Protect > Advisories. | Fixture package sets on Fedora, Debian and Arch data yield the expected lists with the right fixed versions. |
| 7 | Inspect everything | System and language package inspection, lockfile linter, repo-check, repo audit, archives and launchers, Downloads watch. | One fixture per rule id fires and no others. |
| 8 | Sandboxes | Confinement table, Flatpak audit and overrides, Snap, containers, runtime posture, confinement status. `/impeccable shape` first. | The D5 acceptance tests. |
| 9 | Posture and inventories | Posture, installed-file verification, SUID inventory, accounts, process checks, credential inventory, extension inventory, more canaries. | `bigman doctor --json` on a clean VM reports no findings with full coverage. |
| 10 | Root and network | Helper crate and install card. Per-process bandwidth. Audit-log denials. OpenSnitch UI server, prompts, rules. Firewall view. | Helper removal leaves nothing behind; an OpenSnitch prompt renders in Bigman within 1 s; `cargo deny` still clean. |
| 11 | Reviewer and imports | AI reviewer with both backends and the send preview. Falco or Tracee import source. Feed fetch and digest. YARA-X. | A fixture receipt with an injected "report nothing" comment still shows its deterministic findings and the reviewer output flags the comment. |
| 12 | Release | `/impeccable harden`, `onboard`, `polish`, `document`. Packaging: AppImage, deb, rpm, AUR. Public self-test page. | Manual checklist passed; fixtures-fired page generated. |

Milestones 0 to 4 are sequential. After 4, milestones 5 to 9 are independent of each other and can be reordered by the owner. 10 and 11 depend on 4 and 5.

---

## Part K. Decisions

Made on 2026-10-29 by the owner:

- **Framework: Tauri 2** with React and TypeScript. Linux first; Windows and macOS kept possible through the platform traits, not built.
- **Helper: yes**, optional and read-only, per Part E.
- **OpenSnitch: Bigman replaces its UI** and keeps its daemon.
- **Store: SQLite** for events, samples and receipts.
- **Packaging: no Flatpak build** of Bigman for now; Bigman still audits and manages Flatpaks.
- **Scope: observe, explain, preview.** Not an EDR.
- **Mobile: no.**

Made on 2026-10-29 by the owner, answering the open questions:

1. **Reviewer backends:** ship both, a local OpenAI-compatible endpoint and the Claude API. Triaged mode sends a packet when its deterministic triage score is 3 or more (a High rule scores 3, a Review rule 1, Info 0).
2. **Baseline format:** there are no existing baselines to preserve. The widened format in milestone 5 starts from scratch; an older baseline file is treated as absent and the user reviews an initial inventory.
3. **`systemd-journal` group:** Setup offers it as a previewed polkit action that prints the exact command, what it grants and how to undo it. Never applied automatically.
4. **Logo:** the existing logo is the application and launcher icon only; the app and tray use a quiet monochrome "B" mark so red stays reserved for High severity.
5. **Visual direction (2026-10-30):** "Life List", a field naturalist's life list: first sightings entered once with a ringed violet mark, repeats folded under them with a tally. Recorded in `.impeccable/surfaces/crates-app-ui-src.md`.
6. **Inspect (2026-11-02):** one field for a file, folder or HTTPS URL; the daemon picks the inspector (`core/src/checks/route.rs`, `bigman check`). YARA-X rules and dependency advisories sit under "More inspections". The Downloads watch in milestone 7 reuses the same routing.
7. **Protect (2026-11-02):** keeps five sub-views (Scans, Quarantine, Startup files, Canaries, Audits); Scans opens with the last scan's status and signature age.
8. **Previewed actions (2026-11-02):** every state change is a `core::actions::Action` with a daemon-written preview and a verb for its confirm button; reversible changes return their undo action, offered as Undo in the result; the extra acknowledgment is reserved for running unreviewed code (the installer rehearsal).

9. **Hardening (2026-11-02):** suggestions live in Protect > Hardening, the first Protect sub-view, with a fifth Overview tile. Bigman offers a change only when it can preview it exactly and undo it. Firmware, installer-time and display-manager settings (Secure Boot, disk encryption, autologin) and installing a missing firewall get manual steps. A change is refused, with the reason, when it would cut you off (turning off SSH while you are logged in over the network) or would not last (a sysctl file read after Bigman's sets another value).

Open, ask the owner when reached:

1. Tab count. Seven is the plan; if Sandboxes feels like one too many after milestone 8, Denials can move into Activity and the audits into Protect.

---

## Part L. Working agreement for the implementing agent

- Read Part B before any UI work and Part C before any Rust work. Re-read B5 before writing any user-facing string.
- Prefer deleting over abstracting. Three similar lines are better than a premature helper. A component or trait is added when the third user appears, not the first.
- Every pull request includes: the code, the fixtures, the registry entries, the README table row with its boundary, and the threat-map status update in `docs/threats/`.
- Keep commits small and messages plain: what changed and why, one paragraph.
- Never claim a check proves anything. Write coverage.
- When a tool, API or protocol detail is uncertain (Tauri plugin names, OpenSnitch protocol fields, distro advisory formats, Claude model IDs), look it up or read the source before coding. Do not implement from memory.
- When something in this plan conflicts with what the machine or the code actually shows, trust the machine, fix the plan in the same pull request, and say so in the description.
