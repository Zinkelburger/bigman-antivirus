---
target: disposable container section and dialog
total_score: 22
max_score: 40
na_heuristics: 
p0_count: 0
p1_count: 3
target_identity: "file:/home/anbernal/Projects/bigman-try/crates/app/ui/src/design/report-actions.tsx"
target_fingerprint: "sha256:252d01fafe71b0f45521fab7aba91dad738c2c72f2a5c2b921cd3700fcf44b4b"
target_path: /home/anbernal/Projects/bigman-try/crates/app/ui/src/design/report-actions.tsx
timestamp: 2026-11-03T21-25-08Z
slug: crates-app-ui-src-design-report-actions-tsx
---
# Critique: "Run it in a disposable container" (OpenInContainer + preview dialog)

Method: dual-agent (A: design review · B: detector + browser evidence)

| # | Heuristic | Score | Key issue |
|---|---|---|---|
| 1 | Visibility of system status | 2 | "Done / Opened konsole" before anything pulled or started |
| 2 | Match system / real world | 2 | "No network" negative switch; port label implies Bigman opens a browser |
| 3 | User control | 3 | Cancel + shell exit delete; no in-app view of a running container |
| 4 | Consistency | 2 | three names; "Exactly what changes" when nothing changes; not careful unlike Rehearse |
| 5 | Error prevention | 2 | network on by default under a High download-and-run finding |
| 6 | Recognition | 3 | summary doesn't echo network/port choice |
| 7 | Flexibility | 2 | no bigman try one-liner; placeholder commands |
| 8 | Minimalist | 2 | five-element form always open; mono wall dialog |
| 9 | Error recovery | 2 | Podman-missing as mono error, no route to Setup |
| 10 | Help | 2 | no distinction from the offline rehearsal |
| Total | | 22/40 | Acceptable |

Priority issues:
- [P1] Decisive facts (network reach, kernel caveat) buried in mono among podman flags; split plan into facts + note + commands.
- [P1] Summary overclaims with network on ("sees only that copy", "nothing comes back").
- [P1] Network on by default, even under High findings and beside the offline rehearsal; use "Allow network", default off for High/installer.
- [P2] Options always open; section live when Podman missing; port field lacks aria-invalid.
- [P2] Image download from Docker Hub undisclosed; "Done" too early.

Detector: CLI clean (0). Browser: nested-cards, thin-border-wide-shadow (pre-existing shared components), layout-transition and em-dash-overuse false positives. Input/switch borders 1.84:1 (shared components).
