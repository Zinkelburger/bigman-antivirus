---
target: Protect > Hardening view and Overview tile
total_score: 27
max_score: 40
na_heuristics: 
p0_count: 0
p1_count: 2
target_identity: "file:/home/anbernal/Projects/bigman-hardening/crates/app/ui/src/features/protect/HardeningView.tsx"
target_fingerprint: "sha256:70d29a07e59386734a82974c89e95b8b4604508024f1ac65479893d0150ba159"
target_path: /home/anbernal/Projects/bigman-hardening/crates/app/ui/src/features/protect/HardeningView.tsx
timestamp: 2026-11-03T02-46-44Z
slug: ui-src-features-protect-hardeningview-tsx-280ecc7d
---
Method: dual-agent (A: design review sub-agent · B: detector + browser sub-agent)

## Design Health Score: 27/40 (Acceptable)

| # | Heuristic | Score | Key Issue |
|---|---|---|---|
| 1 | Visibility of System Status | 3 | Summary line lacks "ran at" when there are suggestions |
| 2 | Match System / Real World | 3 | "Change…", "Keep as it is"/"Keep it" vague; "Hardening" is jargon |
| 3 | User Control and Freedom | 3 | Undo everywhere; Changed-by-Bigman record below the fold after a fix |
| 4 | Consistency and Standards | 2 | Tile "5 suggestions" vs view "3 to review, plus 2 optional"; tile never Incomplete |
| 5 | Error Prevention | 3 | No advance notice that a password prompt is coming |
| 6 | Recognition Rather Than Recall | 2 | Keep dialog doesn't name the setting; "optional" absent on rows |
| 7 | Flexibility and Efficiency | 2 | Focus lost after a fix; fix button below fold at 960x600 |
| 8 | Aesthetic and Minimalist Design | 3 | Intro paragraph every visit; bold Details summary |
| 9 | Error Recovery | 3 | Verbatim tool errors; incomplete notice has no action |
| 10 | Help and Documentation | 3 | Why/cost present; manual steps put commands in sans prose |

## Design Specificity Verdict
Mostly authored: cost-first preview, exact commands, Already set, honest coverage. Generic list layout; no first-seen marker.
Detector: CLI 0 findings (4 files). Browser: skipped-heading (real); text-occlusion, nested-cards, wide-shadow, cramped-padding, dark-glow are false positives.

## Priority Issues
- [P1] Focus lost to body after applying a fix from a row and after opening the Overview tile (design/action.tsx:40, OverviewPage.tsx:188).
- [P1] Overview tile counts all open suggestions while the view separates review from optional; tile never shows Incomplete.
- [P2] Keep dialog says nothing changes yet shows an "Exactly what changes" block with the rule ID, and does not name the setting (core actions.rs).
- [P2] Admin/password prompt not announced: polkit sentence is a mono line in the evidence block; rows don't mark admin fixes.
- [P2] At 960x600 the fix button in Details is below a bold five-line summary.
- [P2] Alarm word "attacker" in shipped rule text (hardening/mod.rs LLMNR why).
- [P2] Heading structure: main list has no heading (h1 -> h3 skip); suggestion titles are not headings.

## Persona Red Flags
Alex: no copy-all commands; "Change…" opaque. Sam: focus losses; headings skip suggestions; duplicate aria-label. Jordan: 5 vs 3 mismatch; Info never called optional.

## Minor Observations
Ragged 3+2 tile grid at 960; long measure at 1440; misaligned fix buttons; "done." under "Done"; imperative titles in Changed list; IncompleteNotice without action; placeholder-like calm tile detail.

## Questions to Consider
Is "Hardening" the right name for non-experts? Should a fix end on a before/after evidence diff? Should Info items count on Overview at all?
