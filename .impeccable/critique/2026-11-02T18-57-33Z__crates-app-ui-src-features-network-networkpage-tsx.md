---
target: Network tab and event-based Activity/Overview (M4)
total_score: 27
max_score: 40
na_heuristics: 
p0_count: 0
p1_count: 3
target_identity: "file:/home/anbernal/Projects/bigman-antivirus/crates/app/ui/src/features/network/NetworkPage.tsx"
target_fingerprint: "sha256:a12a71840991d2636c0c8f81770b1e979aa23b00aa779a2365056b5bc007240b"
target_path: /home/anbernal/Projects/bigman-antivirus/crates/app/ui/src/features/network/NetworkPage.tsx
timestamp: 2026-11-02T18-57-33Z
slug: crates-app-ui-src-features-network-networkpage-tsx
---
Method: dual-agent (A: design review · B: detector and browser evidence)

## Design Health Score
| # | Heuristic | Score | Key Issue |
|---|-----------|-------|-----------|
| 1 | Visibility of System Status | 3 | Mark all as seen changes nothing visible; 1h caption reads "Received –" |
| 2 | Match System / Real World | 3 | Raw interface names; 7-day axis all "00:00"; root sshd shown as "Another user's program" |
| 3 | User Control and Freedom | 3 | Mark all as seen has no undo |
| 4 | Consistency and Standards | 2 | "Rated" vs "Severity"; two remediation sources; drill-down titles differ; connection rows focus but don't open |
| 5 | Error Prevention | 3 | Irreversible mark-all |
| 6 | Recognition Rather Than Recall | 2 | Unread badge maps to no rows; nested finding rows don't look openable |
| 7 | Flexibility and Efficiency | 3 | No unread filter; search covers summary only |
| 8 | Aesthetic and Minimalist Design | 3 | Gap notice and two full charts take the first screen |
| 9 | Error Recovery | 2 | Denial source gap has a reason but no remedy |
| 10 | Help and Documentation | 3 | Rule explanations good; threat IDs not linked |
| **Total** | | **27/40** | **Acceptable** |

## Design Specificity Verdict
Activity and Overview are product-authored (tally, ringed first-seen mark, "seen 14 times, first on Oct 3", coverage). Network is category-interchangeable: two monochrome charts and two tables; the life list stops at a First seen column. Detector: CLI clean on all five targets; browser overlay reported cramped-padding (DataTable/Timeline frames, false positive by DESIGN.md) and layout-transition (stale dev-server CSS, false positive). No contrast or naming failures; every grid and listbox labelled.

## Priority Issues
- [P1] Throughput charts claim things that are not true: 5-minute view bridges stored and live data with a line; captions show "–" when the newest bucket is empty; 7-day ticks all "00:00"; odd tick values. Fix in throughput.ts buildSeries and ThroughputChart (null at the seam or fetch to now; caption last value with its time; gaps as text and in aria-label; date ticks for day steps).
- [P1] Network hides what to look at: charts above the High listener; no result line. Fix: result line first, listeners, connections, then throughput; compact secondary interfaces; plain interface names.
- [P1] "New since last look" has no visible form: badge, tile and lead count different things; mark-all changes nothing on screen. Fix: "Since your last look" divider in Timeline at unread.through; toast with Undo after marking; tile opens Activity filtered to unread; lead sentence states its scope.
- [P2] Source-gap notice dominates Activity and has no remedy; mono used for Bigman's own sentence. Fix: one line with Details; remedy command in Setup; note under the Denials toggle.
- [P2] Drill-downs break their own contract: connection rows don't open; listener findings one click deeper; titles by source; "Threats" label. Fix: inline FindingEvidence; connection panel; summary titles; one remediation source; rename label.

## Persona Red Flags
- Alex: no unread filter; narrow search; chips reorder by count; no protocol column; unreadable 7-day axis; no undo.
- Sam: chip names run counts into labels; Enter does nothing on connection rows; chart labels lack current value and gaps; no announcement after mark-all.
- Non-expert: "Reachable from" truncates at 960; root sshd reads as an intruder; 0.0.0.0 and wlp2s0 unexplained.

## Minor Observations
- First seen column truncates; "new address range … seen 7 times" contradiction; epoch-0 first_at renders Dec 31; empty scenario shows full Network data; Overview list capped at 420px; violet mark means different things on Network and Activity; Setup has two right margins; network skeleton not chart-shaped.

## Questions to Consider
- Should Network be a life list of sightings with throughput as a sparkline?
- Is "first seen" the right mark for "new to me", or is a divider missing?
- Should an unfixable-in-app gap be asked once in Setup and then stay quiet?
