---
target: Install receipts and transaction preview (M5)
total_score: 25
max_score: 40
na_heuristics: 
p0_count: 1
p1_count: 3
target_identity: "file:/home/anbernal/Projects/bigman-antivirus/crates/app/ui/src/design/transaction-preview.tsx"
target_fingerprint: "sha256:96f5f4841afaa3386179731d1c8254b8679b9e26ce9b1b0f5e91c98b6aa56157"
target_path: /home/anbernal/Projects/bigman-antivirus/crates/app/ui/src/design/transaction-preview.tsx
timestamp: 2026-11-02T20-36-27Z
slug: crates-app-ui-src-design-transaction-preview-tsx
---
Method: dual-agent (A: design review · B: detector and browser evidence)

## Design Health Score
| # | Heuristic | Score | Key Issue |
|---|-----------|-------|-----------|
| 1 | Visibility of System Status | 2 | Countdown is muted body text that scrolls away; nothing shows dnf still waiting after closing |
| 2 | Match System / Real World | 3 | "Decide later" promises a later that does not exist; "Nothing was installed" wrong for upgrades |
| 3 | User Control and Freedom | 2 | Esc, close, backdrop and Decide later all discard the preview irreversibly |
| 4 | Consistency and Standards | 3 | Receipt result line cannot say Incomplete; opt-in hook shown as a to-do |
| 5 | Error Prevention | 2 | Continue is primary even with findings; focus starts on close; backdrop click dismisses |
| 6 | Recognition Rather Than Recall | 2 | "line 3" in evidence not linked to the script; highlight shows only the rule id |
| 7 | Flexibility and Efficiency | 2 | No keys for Continue/Abort; files do not open; no repository or old→new version |
| 8 | Aesthetic and Minimalist Design | 3 | Modal shows all three levels at once |
| 9 | Error Recovery | 3 | Verbatim errors; clear "no longer waiting" |
| 10 | Help and Documentation | 3 | Good explanations; hook plan block mixes path, file and command |
| **Total** | | **25/40** | **Acceptable** |

## Design Specificity Verdict
Parts are system-specific (marked script line with rule, mono evidence, coverage); the screen shapes are generic: receipt is a key-value list plus two buttons, the dialog is the stock PreviewDialog with the whole receipt inside. Detector: CLI clean on five targets; overlay 15 hits, all false positives by DESIGN.md (timeline/table frames, review-soft line band, the documented dialog shadow, EvidenceBlock figures, occlusion under the drill-down, a stale Tailwind dev rule). Real accessibility points from B: drill-down lets focus reach the rail; focus drops to body after Esc on the preview.

## Priority Issues
- [P0] "Decide later" silently means Continue and the preview cannot be found again (close removes it from the store; no timeout toast; backdrop click also dismisses). Fix: keep hidden previews, a waiting indicator in the status strip with Open, block backdrop dismissal, honest labels.
- [P1] Clock and recommendation styled wrong: countdown in muted body text; Continue primary even with findings; focus on close; generic description. Fix: countdown and progress bar in the footer; equal-weight buttons when findings exist; focus the heading; description names packages and time; live announcements.
- [P1] Modal has no triage order and does not scale (all findings, all scripts, unvirtualised files, 32 packages in one paragraph). Fix: findings first linking to marked lines, only marked scripts, file counts, capped package list.
- [P1] Receipt summary cannot say Incomplete when coverage was skipped. Fix: Incomplete with IncompleteNotice.
- [P2] Level 3 and "every figure opens" missing: FileTree has no onOpen and drops detail; figures do not open; note shows rule id only; preview advice wrong. Fix accordingly.

## Persona Red Flags
- Alex: no repository or old→new version; no keys; files and findings do not link; startup summaries omit the path.
- Sam: countdown never announced; decision after seven tab stops; Esc discards silently; disabled file rows.
- Non-expert mid `sudo dnf upgrade`: terminal silent for up to 60 s; preview not replayed to a window opened from the notification; violet Continue reads as the safe choice.

## Minor Observations
Long back label at 960; Bigman's own sentence in mono; "Started afterwards: None" wording; single-row table chrome; Setup to-do circle for an opt-in item; mock preview reuses the after-the-fact note; buttons stay enabled at 0 s; no "1 of 2" for queued previews; hook plan's path on line 1.

## Questions to Consider
- If timing out continues, is Continue an answer at all? One button, "Stop the transaction"?
- Should the receipt lead with repository and biggest change?
- Should the terminal carry the countdown too?
