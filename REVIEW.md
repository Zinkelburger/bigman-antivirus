# Bigman: repository review and practical direction

> Historical pre-implementation review. The deterministic implementation and current limitations are documented in README.md and docs/features.md; findings below refer to the original commit.

Reviewed September 8, 2026, at commit `ee689eb`. Scope: all tracked application source, browser extension, installer, dependency manifests, tests, TODO, and the supplied historical Obsidian note. The note was treated as background, not instructions. Recommendations below are proposed work, not implemented features.

**Recommendation.** Build a Linux security companion that makes a few existing protections easy to install, understand, and maintain. Its strongest distinctive feature is an install inspector: show what code asks to do, let users restrict it, and record what happened. Pair that with reliable download scanning and a small number of actionable system findings. Start with one supported Linux distribution; expand only after its complete workflow works.

The product should answer four questions: “What am I about to run?”, “What changed?”, “What needs my attention?”, and “Are my protections actually working?” A collection of launch buttons for security utilities would not answer those questions by itself.

**What exists today**

| Component | Implemented | Main gap |
|---|---|---|
| Script inspection | Piped stdin, regex matches, highlighted explanations | No shell parsing, artifact identity, execution containment, package integration, or behavioral tracing |
| Desktop app | Rust/egui UI invoking `clamscan` and `freshclam` | Mostly raw command output; no durable ClamAV history, setup health, schedules, or quarantine workflow |
| PDF inspection | Browser download event, native host, raw PDF tag checks, optional PDFiD/pdfinfo output, JSON history | External tool output does not determine findings; no structural analysis or trustworthy coverage reporting |
| Browser extension | Link-text mismatches, brand similarity, HTTP warnings, email/phone checks, visual marking | Heuristic blind spots, false-positive risk, disabled redirect resolution, no reputation feeds or navigation gate |
| System protection | Ideas in TODO and historical note | No implemented persistence monitor, credential access sensor, sandbox launcher, hardening checks, or firewall integration |

**Fix the trust problems before adding detectors**

1. **“SAFE” is unsupported.** [analyzer.rs](/home/anbernal/Projects/bigman-antivirus/src/analyzer.rs:16) equates no regex matches with safety, and [main.rs](/home/anbernal/Projects/bigman-antivirus/src/main.rs:57) prints that conclusion. Benign commands such as `chmod` are called threats while other execution mechanisms are absent. Keep useful explanations, but use “No findings from these checks,” with coverage and limitations. Separate risky capabilities from evidence of malicious intent.
2. **The PDF verdict has very narrow coverage.** [pdf_scanner.rs](/home/anbernal/Projects/bigman-antivirus/src/pdf_scanner.rs:78) obtains PDFiD output but decides using raw `/OpenAction` and `/AA` substring searches. Neither finding alone proves malware, and their absence does not exclude JavaScript, embedded content, exploits, or phishing links. Parser failures and missing tools need explicit incomplete/error states.
3. **Default scanning is misleadingly shallow.** [gui.rs](/home/anbernal/Projects/bigman-antivirus/src/gui.rs:23) defaults all scan options to false, while the initial target is `/home`. Without recursion, users will not get the home-directory scan they likely expect. Prefer an explicit Downloads/recent-files preset and report examined, skipped, unreadable, and oversized files. Remove the prominent delete-on-detection option in favor of reversible quarantine.
4. **Local IPC adds avoidable attack surface.** [ipc.rs](/home/anbernal/Projects/bigman-antivirus/src/ipc.rs:8) accepts unauthenticated TCP connections on a fixed loopback port, then scans the supplied path. It handles one blocking connection at a time and reads only one 1,024-byte chunk. Another local process can submit requests or stall delivery. Use a per-user Unix socket with restricted directory/socket permissions, peer identity checks, framing, timeouts, and bounded work. This improves cross-user isolation; it is not a boundary against arbitrary malware already running as the same user.
5. **Native messages and parser jobs are unbounded.** [main.rs](/home/anbernal/Projects/bigman-antivirus/src/main.rs:75) allocates the declared incoming message size. PDF subprocesses and file reads have no resource bounds. Validate requests, restrict files to the intended scan workflow, reject special files, cap sizes/output/time, and scan an identified snapshot so the report corresponds to the bytes examined. Run parsers in restricted subprocesses.
6. **Native setup contains unsupported origins.** [install-native-messaging.sh](/home/anbernal/Projects/bigman-antivirus/install-native-messaging.sh:39) falls back to wildcard extension origins. Chrome requires explicit origins; its manifest does not allow wildcards. The checked-in host manifest also contains an old absolute binary path. Generate a valid manifest from the actual installation and extension ID and perform a health check. See [Chrome native messaging](https://developer.chrome.com/docs/extensions/develop/concepts/native-messaging).
7. **Browser coverage is narrower than it looks.** [link-detector.js](/home/anbernal/Projects/bigman-antivirus/browser-extension/link-detector.js:61) returns early when visible text is empty, bypassing destination checks on image-only links. Its [punycode check](/home/anbernal/Projects/bigman-antivirus/browser-extension/link-detector.js:261) checks non-ASCII characters across the entire URL, missing ASCII `xn--` hostnames and confusing internationalized paths with hostname deception. [content.js](/home/anbernal/Projects/bigman-antivirus/browser-extension/content.js:124) observes added nodes, not changed `href` attributes. Warnings are styling and console logging; clicks continue.
8. **History and task lifecycle need attention.** PDF history uses a working-directory-relative folder, and scan IDs can collide for the same filename within one second. A native request can cause both the host and GUI to scan the same file. Clearing a running ClamAV result drops its receiver without stopping the process. Use one job owner, stable IDs, per-user application storage, cancellation, and durable completion events.

**The tools worth incorporating**

| Need | Existing tool | Bigman's useful contribution | Priority |
|---|---|---|---|
| Known malware in downloads and files | [ClamAV](https://docs.clamav.net/manual/Usage/Scanning.html) | Install/configuration health, definition age, sensible scan scope, understandable results, quarantine and restore | First |
| Explain installer scripts | [ShellCheck](https://www.shellcheck.net/) and [mvdan/sh](https://github.com/mvdan/sh) parser/formatter | Link statements to privileges, downloads, sensitive paths, and persistence changes; identify unresolved behavior | First |
| Control dependency build scripts | [pnpm approve-builds](https://pnpm.io/cli/approve-builds), [npm script settings](https://docs.npmjs.com/cli/v11/commands/npm-install/) | Inspect effective policy, display requested scripts by exact package version, record approvals | First, npm/pnpm only |
| Package reputation before installation | [Socket Firewall Free](https://docs.socket.dev/docs/socket-firewall-free) | Optional guided setup and visible warnings/status | Early optional integration |
| Known dependency vulnerabilities | [OSV-Scanner](https://google.github.io/osv-scanner/) | Scan lockfiles and explain affected dependencies and fixes separately from malware findings | Early |
| Restricted installer execution | [Bubblewrap](https://github.com/containers/bubblewrap), with [strace](https://github.com/strace/strace) for observation | Tested execution profiles and a readable report of process/file/network activity | After static preview |
| Security configuration | [Lynis](https://cisofy.com/lynis/) | Import report IDs, prioritize a small selection, show concrete fixes and verify outcomes | Early |
| Suspicious persistent changes | [osquery](https://osquery.readthedocs.io/en/stable/deployment/file-integrity-monitoring/) plus OS inventory | Baseline selected startup locations and explain changes over time | Next |
| Unexpected outbound connections | [OpenSnitch](https://github.com/evilsocket/opensnitch) | Setup/health integration and links into its existing interface; correlate events where a supported interface permits | Optional |
| PDF triage | [PDFiD and pdf-parser](https://blog.didierstevens.com/programs/pdf-tools/) | Surface active content, extracted destinations, embedded files, and parser limitations | Next |
| Additional file signatures | [YARA-X](https://github.com/VirusTotal/yara-x) | A small maintained ruleset with source, version, performance limits, and explanatory findings | Later |
| Decoy tripwires | [Canarytokens](https://docs.canarytokens.org/guide/) | Optional placement guidance and test/health workflow | Later |

ShellCheck finds shell bugs; it is not a malware classifier. OSV finds known vulnerabilities; a vulnerability-free package can still contain malicious code. YARA-X needs maintained rules; installing its engine alone adds no detection content. Treat each integration as a specific source of evidence.

Socket is a particularly close fit, but evaluate its actual tradeoffs: the free version supports public registries, blocks confirmed malware, warns on AI-suspected malware, and allows unknown versions. It collects package names/versions and a machine identifier as telemetry, with no free-version telemetry switch. Offer it as an explicit external-service integration. [Socket documentation](https://docs.socket.dev/docs/socket-firewall-free)

**Make the installer preview the distinctive feature**

Start with an inspection workflow, conceptually `bigman inspect <installer-url>`; this command does not exist yet. Save the response once, record its URL/redirect chain and SHA-256, and show those exact bytes. Accept local scripts too. Prefer a verified distro package or signed upstream artifact when available; show the evidence for that preference.

Show a short explanation with expandable source locations: requests administrator privileges; fetches another executable; writes a shell startup file; installs a background service; references browser data or credentials. Mark dynamically constructed commands and second-stage downloads as unresolved rather than pretending to have expanded the complete program. Sanitize terminal control characters in displayed untrusted content. An optional LLM can explain findings, but it must not execute instructions from scripts or PDFs, declare code safe, or upload private content implicitly.

If execution is added, execute the saved artifact, not another download from the same URL. Changes in any fetched second-stage payload require their own tracking. Never evaluate an arbitrary pasted shell command merely to discover its URL. Keep inspect and run as explicit separate actions.

The second stage is a disposable restricted rehearsal with a fake home, sanitized environment, no credentials or agent sockets, no host service sockets, a controlled writable area, resource limits, and network denied unless deliberately enabled. Copy only necessary inputs; do not mount a real home read-only, since that still exposes secrets. Keep tracing outside the target's control where possible. Report allowed, denied, and unobserved operations separately.

Bubblewrap supplies building blocks, not a ready-made security policy. Its protection depends on the wrapper's configuration. A traced run only reveals the behavior exercised in that environment; malware may delay or detect it. Do not call this a guarantee or a safe place to detonate arbitrary malware. Stronger analysis belongs in a disposable VM or isolated service. [Bubblewrap security model](https://github.com/containers/bubblewrap)

An installer that needs host root may be unable to complete in rehearsal. Explain that limit; do not silently rerun it unrestricted. Keep any later actual installation and its privileges explicit. Preserve successful build restrictions when executing the resulting tool: install-time controls do not protect against malicious runtime code.

For npm/pnpm, inspect direct and transitive package manifests and referenced script files without running lifecycle scripts. Use lockfile integrity and package/version identity; approvals should be invalidated when the reviewed artifact changes. Detect the installed package-manager version and effective configuration, including project overrides. pnpm's current docs use `allowBuilds`; older versions use different settings. [pnpm approvals](https://pnpm.io/cli/approve-builds)

Expand to Python only after that works. Source distributions can execute build backends, and a virtual environment is dependency isolation rather than a security sandbox. Even uv's `--no-build` has documented editable/first-party exceptions. Rust build scripts and procedural macros, and distro package maintainer scripts, need separate adapters later. [uv CLI behavior](https://docs.astral.sh/uv/reference/cli/)

**Help people find problems already on the system**

Build a read-only “Check my system” flow with three distinct result groups:

- **Malware evidence:** targeted ClamAV scans of downloads and suspicious executables; optional curated YARA matches; record engine, signatures, exact file hash, and coverage.
- **Suspicious changes:** newly added user services/timers, desktop autostarts, cron entries, SSH authorized keys, shell startup changes, and unexpected listening applications. Show path, command, user, first-seen time, and the previous state. Where supported, correlate with package transactions or the process that made the change.
- **Configuration weaknesses:** stale definitions, failed scan jobs, inactive protections, pending security updates from the distro's own advisory metadata, unnecessary exposed services, and plaintext Git credential storage. Git documents that `credential.helper store` keeps credentials unencrypted; inspect configuration without printing credential contents. [Git credential-store](https://git-scm.com/docs/git-credential-store)

A baseline means “seen before,” not “trusted.” On an existing machine, review initial entries too. Package ownership or a matching local package database is useful context, not proof that a compromised host is clean. For strong compromise evidence, provide an evidence export and recovery guidance; deleting one flagged file does not revoke stolen sessions or credentials.

Monitoring credential reads is more difficult than watching file changes. Ordinary file watchers do not reliably identify the reader or prevent access. Start with protecting credentials inside the controlled install environment. For host-wide attribution, later integrate narrowly scoped audit records through the existing audit service or another supported event source. Do not enable conflicting audit collectors; osquery's audit configuration has coexistence constraints. [osquery process auditing](https://osquery.readthedocs.io/en/stable/deployment/process-auditing/)

Landlock can restrict a launched process and its descendants without making the UI root, subject to available kernel features. It is not a monitor that automatically controls every other process on the desktop. SELinux/AppArmor should retain their existing protections; configuring a policy for unconfined desktop tools is separate work. [Landlock documentation](https://docs.kernel.org/userspace-api/landlock.html)

**Keep the browser extension focused**

Its best role is explaining deception before the user acts: “This says paypal.com but opens another domain,” “This hostname resembles one you trust,” and “This page asks you to paste a command into a terminal.” The last case directly addresses ClickFix-style fake verification and repair prompts. Treat page analysis as heuristic coverage, not interception of every clipboard operation. [Microsoft's ClickFix analysis](https://www.microsoft.com/en-us/security/blog/2025/08/21/think-before-you-clickfix-analyzing-the-clickfix-social-engineering-technique/)

Use the browser URL parser and a maintained [Public Suffix List](https://publicsuffix.org/) implementation. Compare actual hostnames and registrable domains, including private suffixes; keep internationalized-domain handling distinct from Unicode path text. Internationalized names, HTTP, redirects, and unfamiliar domains are not individually proof of phishing. Narrow brand similarity rules to avoid making every unusual legitimate site red.

Analyze destinations even without link text; handle attribute changes and recheck at relevant user interactions. Put consequential warnings in browser-owned UI where practical, since hostile pages can imitate or remove in-page marks. Existing link handlers alone cannot cover typed addresses, all redirects, forms, scripts, inaccessible frames, or browser-internal pages.

Keep browser-native protection enabled and recommend established blockers through their official distribution links, such as [uBlock Origin](https://github.com/gorhill/uBlock) or the appropriate [uBO Lite](https://github.com/uBlockOrigin/uBOL-home) build. Bigman should complement them with explanations and install-preview handoff.

If adding reputation, distinguish phishing feeds from malware-distribution feeds. [OpenPhish Community](https://openphish.com/phishing_feeds.html) has limited coverage and a 12-hour refresh interval; [URLhaus](https://urlhaus.abuse.ch/api/) requires an Auth-Key. Cache locally where permitted, show freshness, and verify redistribution terms before shipping feeds. Google's [Safe Browsing API](https://developers.google.com/safe-browsing) is for noncommercial use; commercial applications are directed to Web Risk.

Do not automatically visit every link to resolve redirects. Such requests can disclose browsing activity and trigger tracking or one-time actions. The repo already disabled that behavior after service problems. Any user-requested remote resolution needs privacy controls, limits, and protection against redirects to local/private addresses.

**Which old-note ideas to keep or change**

- Keep opinionated setup, rootless UI, scan history, selected credential protections, persistence baselines, and optional decoys.
- Use the typo generator for controlled brand-watch candidates and test cases, not an unconditional blocklist of every permutation. The linked [AIL tool](https://github.com/typosquatter/ail-typo-squatting) now documents wrong-TLD generation, addressing the old note's specific concern.
- Treat [Package Hunter](https://gitlab.com/gitlab-org/security-products/package-hunter) as historical inspiration: it is archived. The [OpenSSF Package Analysis](https://github.com/ossf/package-analysis) project is a useful reference for isolated behavioral analysis, but its worker pipeline is too much infrastructure for the initial desktop app.
- Make Firejail profile setup an optional later integration if users need it. Pick one tested sandbox approach for the install inspector first.
- Defer a Wazuh/OSSEC deployment, broad eBPF collection, network IDS, and automated rootkit cleanup. They make the first release much harder to operate and interpret.
- Avoid country blocklists as a default malware policy. Prefer evidence about a destination or application; location alone does not establish intent.
- Do not intercept GitHub HTTPS or install a local root certificate to catch honeytokens. Use dedicated decoys, never replace live credentials. A decoy's trigger depends on its type and may not fire merely because a file was read or copied.
- Make VirusTotal lookup optional and prefer an existing hash report before an upload. Hash queries still disclose an identifier; file/URL submissions may be shared with partners and customers. Do not automatically upload private documents or authenticated URLs. [VirusTotal sharing model](https://docs.virustotal.com/docs/how-it-works)
- Defer LLM-based PDF obfuscation verdicts and cryptocurrency-address protection until the core flows are reliable.

**A small implementation plan**

| Stage | Deliverable | Acceptance evidence |
|---|---|---|
| 1: Trustworthy foundation | Structured findings; explicit incomplete/error states; fix browser bypasses, IPC, native setup, job lifecycle and storage; restore tests | Missing tools, malformed messages, scan failures, link mutations, and history collisions produce correct visible outcomes |
| 2: Useful desktop release | Read-only checkup; ClamAV setup health, definition freshness, Downloads scan, optional schedule, quarantine/restore | Harmless AV test fixture detected; clean files retained; skipped coverage visible; restoration works; failed/stale protection produces a notification |
| 3: Distinctive install inspector | Saved-artifact shell preview; source-linked explanations; npm/pnpm lifecycle inventory and approval policy; optional Socket/OSV | A harmless test package's lifecycle script remains unexecuted during inspection; transitive hooks appear; changes invalidate approval |
| 4: Controlled execution and changes | Tested restricted execution profile, process/file activity report, selected persistence diffs | Test fixtures cannot read fake protected secrets, write outside the allowed area, or access prohibited network; legitimate builds are exercised too |
| 5: Targeted expansion | PDF structural triage, reputation feeds, optional host audit/OpenSnitch/decoys | Each integration has version checks, failure handling, bounded resources, alert-quality tests, and removal/rollback |

The UI can stay in egui. Separate the CLI, native messaging host, scan workers, and GUI roles explicitly rather than selecting native mode for every command-line argument. A shared finding should contain source/version, subject/hash, observation time, severity, confidence, evidence, coverage, and suggested action. Keep adapters small and prefer documented structured output over scraping colored terminal output.

Run the UI and normal scans as the user. Use a narrowly scoped privileged helper only for supported operations that require it, such as installing packages or changing service configuration. Prefer distro packages; never let that helper accept arbitrary shell commands or browser-supplied root paths. Preview actual configuration changes, preserve backups, verify the result, and provide rollback. Detect existing services instead of starting competing daemons or updater instances.

The dashboard should surface “definitions stale,” “scanner unavailable,” “monitor stopped,” and “new startup entry” before decorative scores. Track last successful checks, configured coverage, and dropped events. Deduplicate notifications; allow scoped exceptions with expiry. A failed or incomplete check must never create a green “protected” state.

**Validation performed for this review**

- `cargo check --locked`: passed. Cargo emitted a future-compatibility warning for `ashpd 0.8.1`.
- `npm ci --ignore-scripts --no-audit --no-fund`: installed the locked browser test dependencies without lifecycle scripts.
- `npm test -- --runInBand`: failed with `ReferenceError: chrome is not defined` in configuration loading. The existing Node tests lack the browser API mock/configuration injection required by the current implementation.
- Direct Node probes using the checked-in config confirmed that empty link text bypasses analysis and browser-normalized `xn--` hostnames are missed by the punycode-specific check. These probes made no network requests.
- This was a source and integration review, not a malware scan of the host, a full dependency-vulnerability audit, or a live browser/GUI end-to-end test. No application code, security configuration, or browser registration was changed. `REVIEW.md` is the only tracked-content addition; build/test artifacts are ignored.
