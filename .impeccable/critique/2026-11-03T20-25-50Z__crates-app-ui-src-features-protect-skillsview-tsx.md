---
target: Protect > AI skills view
total_score: 25
max_score: 40
na_heuristics: 
p0_count: 0
p1_count: 2
target_identity: "file:/home/anbernal/Projects/bigman-agent-skills/crates/app/ui/src/features/protect/SkillsView.tsx"
target_fingerprint: "sha256:4370fec72c35d5fea0427dc80ddb6eab47ea99ce43f94e3b6ce9915e02ef2548"
target_path: /home/anbernal/Projects/bigman-agent-skills/crates/app/ui/src/features/protect/SkillsView.tsx
timestamp: 2026-11-03T20-25-50Z
slug: crates-app-ui-src-features-protect-skillsview-tsx
---
Method: dual-agent (A: design review sub-agent · B: detector and browser sub-agent)

## Design Health Score
| # | Heuristic | Score | Key Issue |
|---|---|---|---|
| 1 | Visibility of System Status | 2 | Incomplete coverage hidden in a closed disclosure; focus drops to body after a check; no announcement |
| 2 | Match System / Real World | 3 | "Runs on its own: 1 thing" vague; "Approved" doesn't say by whom |
| 3 | User Control and Freedom | 3 | Undo and Take back exist; no "I don't want this" path |
| 4 | Consistency and Standards | 2 | Hand-rolled select, own ProgressRow/IncompleteNotice instead of CheckStatus; three names for "runs on its own" |
| 5 | Error Prevention | 2 | Same violet primary Approve on a High download-and-run skill as on a clean one |
| 6 | Recognition Rather Than Recall | 2 | Changed files not clickable; findings citing other files don't switch the viewer |
| 7 | Flexibility and Efficiency | 2 | Keyboard grid works; no status filters, no next-to-review |
| 8 | Aesthetic and Minimalist Design | 3 | Calm, on-token; SHA-256 near the top, "1 item" captions |
| 9 | Error Recovery | 3 | Verbatim errors; vanished-item state handled |
| 10 | Help and Documentation | 3 | How this works, coverage notes, AboutRule per finding |
| **Total** | | **25/40** | **Acceptable** |

## Design Specificity Verdict
LLM: words authored for the product (Read it yourself; rule IDs beside the exact lines of the skill's SKILL.md); structure generic (4-column table, plain-text status, no first-seen use).
Detector: CLI `impeccable detect` clean (0 findings). Browser (headless, no human tab): 1 layout-transition on app root (false positive, transaction-preview progress bar), 11 text-occlusion (false positive, inert grid behind the drill-down), 3 cramped-padding on highlighted source lines (false positive), 2 nested-cards evidence figures in the panel (real, mild, app-wide pattern). A11y pass: names, headings, grid keyboard loop all fine; confirmed focus loss after Check.

## Priority Issues
- [P1] Incomplete coverage hidden and check loses focus. Fix: CheckStatus focus/role=status behaviour; IncompleteNotice with CoverageList above the table when Incomplete. (/impeccable harden)
- [P1] Approve is the default ending even for High findings. Fix: secondary Approve plus acknowledgment when High; "If you don't want it" section with the tool's remove command (Bigman does not run it); statuses "You approved it" / "You have not reviewed it". (/impeccable clarify)
- [P2] Native select file picker; changed files not navigable; Bigman's words in mono. Fix: FileTree-style file list with change word, executable, cited badge; default to first changed or cited file; clicking a finding selects its file. (/impeccable layout)
- [P2] List misreports severity ("3 [High]") and truncates Status at 15rem. Fix: top badge + "3 findings"; two-line status cell. (/impeccable polish)
- [P2] Panel doesn't lead with the answer. Fix: lead finding and its action first (or what changed), hash moved to the bottom as "Fingerprint of its files". (/impeccable layout)

## Persona Red Flags
Alex: counts not filterable; no next-to-review; no diff for changes; tools joined by spaces; ${CLAUDE_PLUGIN_ROOT} unresolved.
Sam: focus lost after Check/Check again; "Checking again…" not live; run-together finding button names; "3 High" misleading.
Jordan: caveat-first intro, "skill" not explained; "1 thing"; red High next to violet Approve invites approving; no "what instead".

## Minor Observations
Mock line 15 vs 14 highlight; three names for one concept, Nothing vs None; "safe" banned word in intro and coverage notes; no not-yet-checked line; EmptyState without action; scoped npm names truncate wrong end; Activity follow-up is plain text (design/ may not import shell/).

## Questions to Consider
Is "Approve" the right verb? Keep approved text for a DiffView? First-seen ring for new or changed skills?
