---
target: Inspect tab and its report views (M7 kinds)
total_score: 26
max_score: 40
na_heuristics: 
p0_count: 0
p1_count: 3
target_identity: "file:/home/anbernal/Projects/bigman-antivirus/crates/app/ui/src/features/inspect/InspectPage.tsx"
target_fingerprint: "sha256:ad43815ffbd1a5f9bcd11b8275711987c7aac41b0b5dffb2e47cb5fd0f5308ff"
target_path: /home/anbernal/Projects/bigman-antivirus/crates/app/ui/src/features/inspect/InspectPage.tsx
timestamp: 2026-11-03T18-19-00Z
slug: crates-app-ui-src-features-inspect-inspectpage-tsx
---
Method: dual-agent (A: design review · B: detector and browser evidence)

## Design Health Score
| # | Heuristic | Score | Key Issue |
|---|---|---|---|
| 1 | Visibility of System Status | 3 | Indeterminate progress; result status mounted pre-filled, so it may not be announced |
| 2 | Match System / Real World | 2 | "Launcher", "Package" vs "Package file" don't match what the user thinks they have |
| 3 | User Control and Freedom | 3 | Can't edit the path in place after a result |
| 4 | Consistency and Standards | 2 | Only installer reports highlight matched lines; kind names inconsistent; URL subject in sans |
| 5 | Error Prevention | 2 | OSV check sends data with no preview; long path overflows at 960 |
| 6 | Recognition Rather Than Recall | 2 | "What you can do" only at level three; drill-down title lacks file name; history paths cut the file name |
| 7 | Flexibility and Efficiency | 3 | No drag-and-drop or paste-to-inspect |
| 8 | Aesthetic and Minimalist Design | 3 | Path shown 3 times, result phrase twice, hash above findings; triple-nested borders |
| 9 | Error Recovery | 3 | Incomplete state is good; errors verbatim |
| 10 | Help and Documentation | 3 | About-this-rule and scope notes strong but small and muted |
| **Total** | | **26/40** | **Acceptable** |

## Design Specificity Verdict
About 70% Bigman: type, rail, severity discipline, installer script with rule notes, honest scope text. The result card is one generic template for every kind, and Recent inspections is a plain table instead of the Life List. Detector: CLI clean; browser: nested-cards (result panel > findings table > evidence figure), skipped-heading inside the closed disclosure, minor cramped padding; layout-transition and text-occlusion are false positives.

## Priority Issues
- [P1] The result doesn't answer first: the decision (what it is, what opening it does, what to do) is only in the finding drill-down; the card leads with Result/Looked at/Ran at/SHA-256. Fix: lead sentence from the top finding's summary + action under the h2; move metadata after coverage.
- [P1] A long path widens main to ~1458px at 960 and pushes "Inspect something else" off-screen (AppShell grid auto column + nowrap truncate). Fix: minmax(0,1fr)/min-w-0; truncate paths from the start.
- [P1] "Check dependency advisories" sends a project's lockfile data to osv.dev on one click, no preview (Principle 5). Fix: PreviewDialog with destination and payload.
- [P2] Kind titles: "Package" vs "Package file", "Launcher", "Archive" read poorly; title by detected format (Debian package, Python wheel, Desktop launcher…), file name in drill-down title; package identity as KeyValue at top, for wheels/crates/vsix too.
- [P2] Evidence: captions numbered as line 1 (line numbers disagree with findings); matched lines not highlighted outside installer reports; 60-line scripts bury "What you can do". Fix: caption into the label, highlight "line N" matches, collapse long blocks around matches.
- [P3] No-findings result lacks the StatusLine; Info table headed "Found" under "No findings".

## Persona Red Flags
- First-timer: file manager said "Meeting_Notice.pdf", Bigman says "Launcher"; hash is the first mono text; Browse can't pick a folder.
- Keyboard user: no shortcut to start a new inspection; disclosure reachable only by tabbing.
- Low-vision: long-path overflow; key coverage text at 13px muted; summaries clamped.

## Minor Observations
Empty trailing evidence line in launcher block; mixed badge/text Result column; each inspection bumps Activity's badge; intro "it" before its referent; py.startup-code explanation says "compromise"; disabled Inspect button contrast; h1→h3 in collapsed disclosure.

## Questions to Consider
- Should a High result lead with the action, ahead of B5's per-finding order?
- Should Recent inspections be a Life List (same file folded with a tally)?
- Should the heading say what the daemon detected ("Debian package") rather than the inspector's name?
