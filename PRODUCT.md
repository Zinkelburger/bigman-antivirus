# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Stack

Tauri 2 desktop shell with a React 19 + TypeScript UI (Vite, Tailwind tokens, TanStack Query/Table, uPlot, Zustand, Radix primitives), Linux first. The UI is a thin client of the `bigmand` user daemon; it also runs in a normal browser against fixture data for development. Decided by the owner in `docs/plan/implementation-plan.md` Part K.

## Users

One person on one Linux workstation: desktop users, developers and open-source maintainers. They open Bigman in passing, between other work, to answer "is anything on my machine new or worth a look?" and before they run something they downloaded. They are not security professionals; many are technically fluent, none want a SOC console. Fleets, servers and security teams are explicitly out of scope.

## Product Purpose

Bigman is an always-running security companion for the Linux desktop: it shows what is happening on the machine, inspects things before you run them, explains what the system's own defenses (SELinux, AppArmor, Flatpak, firewalld, OpenSnitch, the package manager) are doing, and tells you what to update. Success is a non-expert opening the app and understanding the state of their machine within five seconds, without ever being frightened or told something untrue.

## Positioning

Deterministic, local and honest. Every finding comes from a rule with a stable ID, a plain explanation and a test fixture; there is no ML in the detection path, no telemetry and no upload. Bigman never claims the machine is safe: every check ends in exactly one of "Findings to review", "No findings from these checks" or "Incomplete", and incomplete coverage is always shown with its reasons. It observes, explains and previews; enforcement stays with the subsystem that owns it.

## Operating Context

- A desktop app with seven tabs (Overview, Network, Activity, Inspect, Sandboxes, Protect, Setup), a tray icon and batched desktop notifications for first-seen events.
- A user-level collector daemon (`bigmand`, systemd `--user`) owns all checks and the store; the app, the `bigman` CLI and the browser native host are thin clients.
- Typical moments: glancing at Overview after an update, reading an install receipt after the package manager ran, inspecting a `curl | sh` installer or an npm package before running it, checking why a Flatpak can see the home directory, reviewing a denial SELinux just logged.
- GNOME and KDE, Wayland and X11, light and dark themes, minimum window 960 × 600.

## Capabilities and Constraints

- Rootless by default. One optional, read-only root helper exists and is off by default; every privileged action is previewed and approved through polkit with its scope printed.
- State-changing actions always show a preview of exactly what will change, then a result, and offer undo or revert where the subsystem allows.
- No kernel modules, no own eBPF, no SUID binaries; other tools' telemetry is imported instead.
- AGPL-3.0-only. GPL-2.0-only code and Elastic License 2.0 rules are only called as subprocesses or imported by the user.
- An AI reviewer is optional and opt-in: never automatic, always previewed, and it can add findings but never remove or downgrade a deterministic one.
- Severity labels are exactly Info, Review and High. A "first seen" marker flags anything new.

## Brand Commitments

- Name: Bigman.
- The existing logo (navy "B" with a flexing arm and a red shield with Tux, `browser-extension/icons/BigManAntivirus.png`) is the application and launcher icon only. Inside the app and in the tray a quiet monochrome "B" mark drawn from it is used, so red stays reserved for High severity.
- Voice: plain language, second person, present tense. Every finding says what was found, the evidence, and what you can do, in that order.
- No alarm words ("threat", "attack", "infected", "danger", "critical") and no reassurance words ("safe", "clean", "protected", "secure"). The calm state is the status line: "No findings from these checks. Ran at 14:02 with signatures from today."
- Error text from tools is shown verbatim as evidence, never paraphrased.

## Evidence on Hand

- `README.md`: every implemented workflow and the boundary each one promises.
- `docs/threats/`: 182 catalogued threats with stable IDs that features cite.
- `docs/research/`: landscape survey, roadmap, vulnerability-feed design, verified feeds.
- `docs/plan/implementation-plan.md`: the build plan, UI contract (Part B) and decisions (Part K).
- No testimonials, user counts, benchmarks or press exist; do not invent them.

## Product Principles

1. Deterministic: a finding is a rule with an ID, an explanation and a fixture, or it is not shown.
2. Three honest results, and incomplete coverage is always explained, never hidden.
3. Observe, explain, preview: the subsystem enforces, Bigman makes it legible.
4. Calm by default: one thing the eye lands on per screen, severity color only for severity.
5. Nothing leaves the machine unless the user configured it and saw what will be sent.

## Accessibility & Inclusion

WCAG AA contrast in both themes. Every action is reachable by keyboard with visible focus (digits switch tabs, `/` searches, `Esc` closes a drill-down, arrows move in tables, `Enter` opens a row). Severity is never conveyed by color alone (icon plus text). Every icon has a label, every table has headers, and `prefers-reduced-motion` is respected.
