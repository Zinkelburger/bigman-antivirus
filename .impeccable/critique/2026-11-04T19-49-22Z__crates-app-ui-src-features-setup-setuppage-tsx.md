---
target: background-disclosure UI (Overview note, Setup)
total_score: 26
max_score: 40
na_heuristics: 
p0_count: 0
p1_count: 3
target_identity: "file:/home/anbernal/Projects/bigman-ux/crates/app/ui/src/features/setup/SetupPage.tsx"
target_fingerprint: "sha256:363ed9a3a17891ce650fc9f9d0b527a75a56a56a9b1485dae03da1610e3ff1ab"
target_path: /home/anbernal/Projects/bigman-ux/crates/app/ui/src/features/setup/SetupPage.tsx
timestamp: 2026-11-04T19-49-22Z
slug: crates-app-ui-src-features-setup-setuppage-tsx
---
Method: dual-agent (A: design review · B: detector + browser)

Target: background-disclosure changes (Overview first-run note, AutomaticChecks, Setup "Checks that run on their own", baseline copy, Canaries naming, Sandboxes tab removed).

## Design Health Score: 26/40 (Acceptable)
1 Status 2 (note claims "running now" while disconnected) · 2 Real world 2 (bigmand, per-user units, canaries unexplained) · 3 Control 2 (only stop is the tray, absent on stock GNOME) · 4 Consistency 3 (services preview/result still say "daily scan") · 5 Error prevention 2 (baseline copy contradicts "not trusted", ignores High finding) · 6 Recognition 3 · 7 Efficiency 3 (digit shortcuts shifted; DESIGN.md says 1-7) · 8 Minimalist 2 (note fills 960x600, pushes tiles below fold) · 9 Recovery 3 · 10 Help 3

## Detector
CLI: 0 findings in all 7 changed files. Browser: line-length on the note paragraph (~97 chars, OverviewPage) and Your data (~86 chars); cramped-padding (DataTable, unchanged) and layout-transition (body, unchanged) are false positives. axe: only a pre-existing aria-allowed-role on the status footer. Contrast AA/AAA in both themes; no horizontal scroll at 960x600.

## Priority issues
- [P1] Note renders while the daemon is down, says "running now", and Got it dismisses it before the switches ever load.
- [P1] Note is incomplete and "To stop all of it" overclaims: with services installed the canary monitor and the daily signature download (network) also run, and tray stop leaves them; the app restarts bigmand on next open.
- [P1] The only stop control is a tray icon, which stock GNOME does not show; add a previewed "Stop background checks…" in Setup.
- [P2] Updates switch says data comes "from your distribution" but CISA KEV is fetched too; Your data omits the signature download; the updates source shows "read" under "nothing leaves this computer".
- [P2] Note takes over the first view; switches sit ~1000 px from their labels; one paragraph carries six facts.
- [P2] Accept-as-baseline copy says "saves as normal" (contradicts "not trusted") and gives generic advice next to a High finding.

## Minor
Services preview/result text still says "daily scan" (setup.rs:356, actions.rs:408); DESIGN.md digits 1-7 and seven tabs; Toggle double-announces its description; "Activity, which only reads" has the wrong subject; notifications sentence ignores the setting; Setup copy says "below" for a section three away.
