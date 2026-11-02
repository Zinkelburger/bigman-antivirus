---
target: Bigman Activity, Inspect, Protect, Setup
total_score: 25
max_score: 40
na_heuristics: 
p0_count: 0
p1_count: 3
target_identity: "file:/home/anbernal/Projects/bigman-antivirus/crates/app/ui/src"
timestamp: 2026-10-30T20-24-28Z
slug: crates-app-ui-src
---
Method: dual-agent (A: design review · B: detector + browser overlay)

Target: crates/app/ui/src, tabs Activity, Inspect, Protect, Setup (browser mode, mock daemon).

| # | Heuristic | Score | Key issue |
|---|---|---|---|
| 1 | Visibility of system status | 2 | Disconnected/loading missing in Scans, Startup, Inspect |
| 2 | Match system / real world | 3 | "Persistence changes", "lynis integration", "Checked: .", UUIDs in results |
| 3 | User control and freedom | 2 | Undo never wired in results; baseline accept has no undo |
| 4 | Consistency and standards | 2 | Same report differs per tab; careful dialog on reversible actions, not on Remove services |
| 5 | Error prevention | 2 | Baseline accepted without seeing changes; confirm enabled before preview loads |
| 6 | Recognition rather than recall | 3 | Finding-to-highlighted-line relies on memory |
| 7 | Flexibility and efficiency | 3 | Keyboard model good; tally not openable |
| 8 | Aesthetic and minimalist design | 3 | Calm; prose-first sub-views, result stated twice |
| 9 | Error recovery | 2 | Raw socket path without a Start action outside Overview |
| 10 | Help and documentation | 3 | Scope explanations excellent |
| **Total** | | **25/40** | **Acceptable** |

Design specificity: the shell and Activity carry the Life List world; Protect, Inspect and Setup interiors follow a generic explainer-form-report-table pattern, and the same-scale before/after raise is missing where it belongs (startup files).

Deterministic scan: detect exit 0. Overlay: line-length true positive (65ch renders ~92-95 characters with this face); cramped-padding and layout-transition false positives (virtual list rows; stale dev-server class).

Priority issues
1. [P1] Folding hides High findings: a clean rescan replaces the High occurrence and the High filter no longer finds it. /impeccable harden
2. [P1] Disconnected and loading states missing or misleading in Scans, Startup, Inspect, Activity (no start action). /impeccable harden
3. [P1] Accept baseline preview shows only a report id, not the entries being accepted; no undo. /impeccable clarify
4. [P2] Protect and Inspect heroes sit under their forms; inspection evidence split between result and drill-down, lost when reopened. /impeccable layout
5. [P2] Preview-to-result gaps: confirm label from first word of title, confirm enabled before preview loads, duplicate dialog+toast feedback, undo not offered, careful flag misassigned. /impeccable polish

Persona red flags: Alex (tally not openable, Lynis runs when absent, raw kind headings); Sam (digit tab switch leaves focus on body, null checklist items look undone, duplicate announcements); non-expert (jargon, successful canary test reads "No findings", acknowledgment checkbox on reversible quarantine, checklist never complete).

Minor: result stated twice; truncated "under a minute ago"; rule note always Review amber; "not proof of safety" in daemon copy; mock line numbers off by one; strip vs Canaries status disagreement; Activity empty state lacks action; quarantine missing when a scan report is opened from Activity; Setup row heights; OSV sends package names without preview.
