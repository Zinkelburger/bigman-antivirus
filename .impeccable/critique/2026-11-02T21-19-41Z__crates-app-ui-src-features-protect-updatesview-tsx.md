---
target: Protect > Updates and the Overview Updates tile (M6)
total_score: 26
max_score: 40
na_heuristics: 
p0_count: 0
p1_count: 3
target_identity: "file:/home/anbernal/Projects/bigman-antivirus/crates/app/ui/src/features/protect/UpdatesView.tsx"
target_fingerprint: "sha256:ce58b869d73d9408cb3864039c520586058814c5906fb189d44ac377a64ab20f"
target_path: /home/anbernal/Projects/bigman-antivirus/crates/app/ui/src/features/protect/UpdatesView.tsx
timestamp: 2026-11-02T21-19-41Z
slug: crates-app-ui-src-features-protect-updatesview-tsx
---
Method: dual-agent (A: design review · B: detector and browser evidence)

## Design Health Score
| # | Heuristic | Score | Key Issue |
|---|-----------|-------|-----------|
| 1 | Visibility of System Status | 3 | Tile hides Incomplete when updates exist; no staleness |
| 2 | Match System / Real World | 2 | Distro "Critical" on a Review row; count is advisories not packages |
| 3 | User Control and Freedom | 3 | No offline option in the UI |
| 4 | Consistency and Standards | 2 | Tile 3 updates vs timeline 4 findings; commands in sans; "None pending" |
| 5 | Error Prevention | 3 | Commands wrap mid-ID in prose |
| 6 | Recognition Rather Than Recall | 2 | "Not checked" tile names a path instead of opening it; hero has no figure |
| 7 | Flexibility and Efficiency | 2 | No single install command, no copy |
| 8 | Aesthetic and Minimalist Design | 3 | Repeated never-runs sentence; intro above the result |
| 9 | Error Recovery | 3 | Mock cannot show failure states |
| 10 | Help and Documentation | 3 | CISA, repository metadata, upstream unexplained |
| **Total** | | **26/40** | **Acceptable** |

## Design Specificity Verdict
Clearly Bigman (shared components, colour discipline) but nothing update-specific: no figure, no install-these step, no sense of time. Detector: CLI clean; overlay real hit: CoverageList item at 60ch vs the 56ch rule; table-in-panel double frame (judgment); others false positives. Accessibility from B: focus drops to body while the check runs; result heading has no visible focus; nothing announced; "Not checked" tile not focusable; rail names run digit and count into labels.

## Priority Issues
- [P1] Two severity scales and a near-alarm phrase ("attackers are already using"). Fix: drop the distro rating from the summary (keep in evidence), "known to be exploited".
- [P1] Tile can hide incomplete coverage; "None pending" too reassuring, also for an end-of-life release. Fix: incomplete detail/state, "No updates listed", "Release no longer supported".
- [P1] The figure goes stale and nothing re-checks. Fix: re-check after package transactions and daily; mute after 24 h, Incomplete after 7 days.
- [P2] Commands are machine text in prose; no single install step. Fix: mono copyable command; report-level "install all".
- [P2] Figures disagree between screens; hero has none. Fix: shared figure helper for tile, timeline row, hero.
- [P3] Actionable "Not checked" tile; fold intro; rename re-download.

## Persona Red Flags
- Alex: no combined command; counts advisories; no offline in UI.
- Sam: generic result heading; command read aloud with no copy; tile not focusable.
- Non-expert: "attackers" next to red; "(Critical)" next to Review; jargon; only terminal remedies.

## Minor Observations
"Checked" used twice; distro in mono; drill-down title repeats; CVE ids wrap; 960 wrapping; restart counted as a finding not an update; mock ignores scenario; no CVE count on the tile.

## Questions to Consider
- Why wait for a click when receipts see every transaction?
- One ordered "install these first" list instead of two severity vocabularies?
