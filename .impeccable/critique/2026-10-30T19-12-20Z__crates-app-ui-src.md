---
target: Bigman app gallery and Overview
total_score: 25
max_score: 40
na_heuristics: 
p0_count: 0
p1_count: 3
target_identity: "file:/home/anbernal/Projects/bigman-antivirus/crates/app/ui/src"
timestamp: 2026-10-30T19-12-20Z
slug: crates-app-ui-src
---
Method: dual-agent (A: design review · B: detector + browser overlay)

Target: crates/app/ui/src (component gallery and Overview tab), browser dev mode.

| # | Heuristic | Score | Key issue |
|---|---|---|---|
| 1 | Visibility of system status | 3 | Strip says "Collecting" while tile says "Monitor stopped"; loading shows EmptyState not Skeleton |
| 2 | Match system / real world | 2 | Tool-speak: "Monitoring", "decoy", "heuristic matched", "uncompromised" |
| 3 | User control and freedom | 3 | Drill-down stack is good; evidence ends without an action |
| 4 | Consistency and standards | 2 | Signatures vs definitions; severity badges on health states; accent overloaded |
| 5 | Error prevention | 3 | PreviewDialog/ConfirmDanger sound; "Start background collection" only refetches |
| 6 | Recognition rather than recall | 3 | Digit hints good; clickable tiles barely distinguishable |
| 7 | Flexibility and efficiency | 2 | No Run checkup or range; sort headers unreachable by keyboard; no skip link |
| 8 | Aesthetic and minimalist design | 3 | Calm; disconnected repeats "Not running" four times |
| 9 | Error recovery | 2 | Monitor stopped offers no action; incomplete scan not explained on page |
| 10 | Help and documentation | 2 | Coverage scope text excellent; badges unexplained |
| **Total** | | **25/40** | **Acceptable** |

Design specificity: the component system is authored for this product (Timeline life list, tally folding, StatusLine, CoverageList); the Overview screen is a stock admin template because the contracted first viewport (range, Run checkup, life list) is not built.

Deterministic scan: `impeccable detect` exit 0, no source findings. Browser overlay: 22 hits on the gallery, 1-2 per Overview scenario; real ones are a skipped heading level (IncompleteNotice h3 under page h1) and ProgressRow animating width. Others are false positives (full-bleed table rows, code-line highlights, sheet-not-card, gallery heading order, stylesheet-only transition rule).

Priority issues
1. [P1] Overview does not implement the contracted first viewport (no range, no Run checkup, no Timeline in any state). /impeccable layout
2. [P1] State honesty and B4 gaps: "Nothing new" before collection exists; EmptyState during loading/disconnected; Open Setup leads to a placeholder; "Start background collection" only refetches; incomplete scan unexplained on page; strip contradicts monitor tile; four repeated "Not running". /impeccable harden
3. [P1] Severity colors used for health states (red High on first launch for missing signatures); info-only findings labelled Review; accent overloaded (selection, diff additions, chart, progress). /impeccable colorize
4. [P2] The plain finding sentence truncates (ReportPanel Found column, Timeline folded rows); finding copy not in B5 voice. /impeccable clarify
5. [P2] Keyboard/screen-reader gaps: sort unreachable, tiles lack dialog semantics, content under the panel stays focusable, no skip link, silent status changes, heading focus ring. /impeccable audit

Persona red flags: Alex (no Run checkup, / inert, sort unreachable); Sam (obscured focus targets, silent changes); a non-expert user (red High on first launch beside "Protection").

Minor: byte counts in coverage notes unformatted; Shield icon on Protect; tile label wrap at 960px; sort arrow direction; "Not checked" means two things; chart x-axis crowded; dark Skeleton nearly invisible; low-contrast tile borders.
