---
name: Bigman
description: A naturalist's life list of one Linux workstation, calm by default, every figure opening to its evidence.
colors:
  ground: "oklch(97.4% 0.002 90)"
  surface: "oklch(99.4% 0.001 90)"
  sunken: "oklch(95.2% 0.003 90)"
  rule: "oklch(89.5% 0.004 90)"
  rule-strong: "oklch(80% 0.006 90)"
  ink: "oklch(23% 0.008 90)"
  ink-muted: "oklch(44% 0.008 90)"
  rail: "oklch(25.5% 0.007 90)"
  rail-ink: "oklch(93% 0.004 90)"
  rail-ink-muted: "oklch(72% 0.006 90)"
  rail-active: "oklch(33% 0.008 90)"
  accent: "oklch(49% 0.17 293)"
  accent-ink: "oklch(99% 0 0)"
  accent-soft: "oklch(95% 0.03 293)"
  focus: "oklch(49% 0.17 293)"
  info: "oklch(50% 0.07 245)"
  info-soft: "oklch(95% 0.015 245)"
  review: "oklch(52% 0.12 65)"
  review-soft: "oklch(95.5% 0.04 80)"
  high: "oklch(51% 0.18 27)"
  high-soft: "oklch(95.5% 0.025 27)"
  calm: "oklch(50% 0.09 155)"
  series-1: "oklch(57.5% 0.163 256)"
  series-2: "oklch(62.2% 0.173 40)"
  series-3: "oklch(58.4% 0.118 165)"
  series-4: "oklch(63% 0.134 73)"
  series-5: "oklch(62.2% 0.171 1)"
  series-rest: "oklch(61% 0.006 90)"
typography:
  title:
    fontFamily: "Atkinson Hyperlegible Next Variable, ui-sans-serif, system-ui, sans-serif"
    fontSize: "1.25rem"
    fontWeight: 600
    lineHeight: "1.75rem"
    fontFeature: "\"tnum\""
  body:
    fontFamily: "Atkinson Hyperlegible Next Variable, ui-sans-serif, system-ui, sans-serif"
    fontSize: "0.9375rem"
    fontWeight: 400
    lineHeight: "1.375rem"
    fontFeature: "\"tnum\""
  body-strong:
    fontFamily: "Atkinson Hyperlegible Next Variable, ui-sans-serif, system-ui, sans-serif"
    fontSize: "0.9375rem"
    fontWeight: 600
    lineHeight: "1.375rem"
    fontFeature: "\"tnum\""
  small:
    fontFamily: "Atkinson Hyperlegible Next Variable, ui-sans-serif, system-ui, sans-serif"
    fontSize: "0.8125rem"
    fontWeight: 400
    lineHeight: "1.125rem"
    fontFeature: "\"tnum\""
  label:
    fontFamily: "Atkinson Hyperlegible Next Variable, ui-sans-serif, system-ui, sans-serif"
    fontSize: "0.8125rem"
    fontWeight: 600
    lineHeight: "1.125rem"
    fontFeature: "\"tnum\""
  mono:
    fontFamily: "Atkinson Hyperlegible Mono Variable, ui-monospace, monospace"
    fontSize: "0.8125rem"
    fontWeight: 400
    lineHeight: "1.25rem"
rounded:
  control: "4px"
  panel: "6px"
  full: "9999px"
spacing:
  "1": "4px"
  "2": "8px"
  "3": "12px"
  "4": "16px"
  "5": "20px"
  "6": "24px"
  control-height: "32px"
  row-height: "36px"
  timeline-row: "40px"
  rail-width: "192px"
components:
  button-primary:
    backgroundColor: "{colors.accent}"
    textColor: "{colors.accent-ink}"
    typography: "{typography.body-strong}"
    rounded: "{rounded.control}"
    padding: "0 12px"
    height: "{spacing.control-height}"
  button-secondary:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    typography: "{typography.body-strong}"
    rounded: "{rounded.control}"
    padding: "0 12px"
    height: "{spacing.control-height}"
  button-secondary-hover:
    backgroundColor: "{colors.ground}"
  button-quiet:
    backgroundColor: "transparent"
    textColor: "{colors.ink}"
    typography: "{typography.body-strong}"
    rounded: "{rounded.control}"
    padding: "0 12px"
    height: "{spacing.control-height}"
  button-quiet-hover:
    backgroundColor: "{colors.sunken}"
  input:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    typography: "{typography.body}"
    rounded: "{rounded.control}"
    padding: "0 12px"
    height: "{spacing.control-height}"
  chip:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    typography: "{typography.small}"
    rounded: "{rounded.full}"
    padding: "0 10px"
    height: "28px"
  chip-selected:
    backgroundColor: "{colors.ink}"
    textColor: "{colors.surface}"
  rail-tab:
    backgroundColor: "{colors.rail}"
    textColor: "{colors.rail-ink-muted}"
    typography: "{typography.body}"
    rounded: "{rounded.control}"
    padding: "0 8px"
    height: "{spacing.row-height}"
  rail-tab-active:
    backgroundColor: "{colors.rail-active}"
    textColor: "{colors.rail-ink}"
    typography: "{typography.body-strong}"
  tab-badge:
    backgroundColor: "{colors.accent}"
    textColor: "{colors.accent-ink}"
    typography: "{typography.label}"
    rounded: "{rounded.full}"
    padding: "0 6px"
  severity-info:
    backgroundColor: "{colors.info-soft}"
    textColor: "{colors.info}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "2px 6px"
  severity-review:
    backgroundColor: "{colors.review-soft}"
    textColor: "{colors.review}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "2px 6px"
  severity-high:
    backgroundColor: "{colors.high-soft}"
    textColor: "{colors.high}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "2px 6px"
  status-tile:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    typography: "{typography.title}"
    rounded: "{rounded.panel}"
    padding: "14px 28px 14px 14px"
  timeline-day:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    typography: "{typography.label}"
    padding: "0 16px 6px"
    height: "36px"
  timeline-entry:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    typography: "{typography.body}"
    padding: "8px 16px"
    height: "{spacing.timeline-row}"
  timeline-entry-selected:
    backgroundColor: "{colors.sunken}"
  evidence-block:
    backgroundColor: "{colors.sunken}"
    textColor: "{colors.ink}"
    typography: "{typography.mono}"
    rounded: "{rounded.panel}"
  drilldown-panel:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    padding: "16px"
    width: "min(560px, 62%)"
  status-strip:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink-muted}"
    typography: "{typography.small}"
    padding: "0 24px"
    height: "32px"
---

# Design System: Bigman

## Overview

**Creative North Star: "The Life List"**

Bigman keeps a naturalist's field notebook of one machine. Everything it notices is entered once, on the day it was first seen, and every later repeat folds into a hand-drawn tally beneath that first entry. What is new stands out because it is ringed in the one violet ink, not because anything is loud. The ground is near-white paper with a whisper of warmth; the navigation sits on a graphite rail like a notebook's cloth cover; everything else is hairline rules, dated groups and plain sentences.

Density is that of a good system monitor: 32 px controls, 36 to 40 px rows, three type sizes per screen, figures in tabular numerals right-aligned to one margin. Depth is almost absent. Panels are flat, outlined with a hairline, and only the two surfaces that float over the page (the drill-down panel and the preview dialog, plus toasts) carry a shadow. Motion is limited to things that carry meaning: a row arriving, the panel sliding in, a progress bar filling, the live graph updating in place.

The system refuses the consumer-antivirus shield dashboard and the dark SOC console. Red never decorates; it means High severity and nothing else. Green is never a verdict; it is a small dot beside the one calm sentence the product is allowed to say.

**Key Characteristics:**
- Neutral paper ground, graphite rail, one violet accent for interaction and first sightings.
- Three reserved severity hues (Info blue, Review amber, High red), always with an icon and a word.
- Atkinson Hyperlegible Next for UI, Atkinson Hyperlegible Mono for evidence, tabular figures everywhere.
- Exactly three sizes: small 13 px, body 15 px, title 20 px; two weights, 400 and 600.
- Flat outlined panels at 6 px, controls at 4 px; shadow only on what floats.
- Every number opens to a table, every row opens to evidence, through one side panel.
- Light and dark from the same token names; the system preference decides unless Setup pins one.

## Colors

A neutral, nearly untinted paper-and-graphite palette (hue 90, chroma at most 0.008) carrying one field-guide violet and three reserved severity hues. All tokens are OKLCH and live only in `design/tokens.css`; Tailwind's default palette is removed, so only these names exist as utilities. The values below are light mode; dark mode redefines every token under the same name (see `.impeccable/design.json` `colorMeta.*.dark`).

### Primary
- **Range-Map Violet** (`accent`): the only interactive color. Primary button fill, toggle "on" track, checkbox accent, the unread-count badge on a rail tab, and the first-seen ring. Its pale wash (`accent-soft`) is used only for text selection. In dark mode it lightens to a lilac and its ink flips to a deep violet-black (`accent-ink`).
- **Focus Violet** (`focus`): the same value as the accent, held as its own token so the focus ring can diverge later. A 2 px outline offset 2 px on every focusable element; inset (offset -2 px) on the Timeline listbox and DataTable grid.

### Secondary (Severity, reserved)
- **Muted Survey Blue** (`info`, on `info-soft`): the Info severity badge only.
- **Field Amber** (`review`, on `review-soft`): the Review severity badge, a rule-matched line in EvidenceBlock and DiffView, and the rule note beside it.
- **Signal Red** (`high`, on `high-soft`): the High severity badge, a High-matched evidence line and its note.

### Tertiary
- **Lichen Green** (`calm`): an 8 px dot only. It sits beside "Background checks running" in the status strip, beside the one calm StatusLine ("No findings from these checks. Ran at ..."), and beside "Local read detection verified" in a report.

### Data series (program identity, Traffic only)
- **Plate Colors** (`series-1` to `series-5`: blue, orange, aqua, yellow, magenta): the hues of a field guide's color plate, one per program among the five that moved the most data. They fill that program's band in the StackedAreaChart, its swatch and its bar in a ShareList, and nothing else. The order is fixed because it is what keeps neighbors apart for color-blind readers (checked with the dataviz validator: worst adjacent pair ΔE 8.3 light, 8.4 dark), and every step is at least 3:1 against `ground` in both themes; dark mode has its own steps of the same five hues.
- **Plate Gray** (`series-rest`): every program outside the five, as one neutral band, swatch or bar.

### Neutral
- **Paper Ground** (`ground`): the page and window background; also the hover fill for secondary buttons, chips, table rows and timeline rows.
- **Sheet** (`surface`): every panel, tile, table, timeline frame, input, the drill-down panel, the dialog and the status strip.
- **Pressed Paper** (`sunken`): evidence and diff backgrounds, the keyboard-selected row, quiet-button hover, the static loading blocks inside a StatusTile, the toggle "off" track, progress-bar track.
- **Hairline** (`rule`): every divider and panel outline.
- **Strong Hairline** (`rule-strong`): control borders (buttons, inputs, selects, chips), the rule under a day heading and under a table header, actionable tile outlines, dashed empty-state outline, scrollbar thumb.
- **Pencil** (`ink`): body text, headings, selected chip fill, the received line in the throughput chart, a determinate progress fill.
- **Faded Pencil** (`ink-muted`): labels, secondary sentences, timestamps, icons, the sent line (dashed) in the chart, sparklines, tally strokes.
- **Graphite Cover** (`rail`, `rail-active`, `rail-ink`, `rail-ink-muted`): the left navigation rail only. In light mode the rail stays dark; in dark mode it goes darker than the ground.

### Named Rules
**The Reserved Severity Rule.** Info, Review and High hues appear only on severity: the SeverityBadge and severity-matched evidence lines. They are never used for decoration, charts, buttons or status. Severity is always icon plus word (Info, Eye, TriangleAlert icons), never color alone.

**The One Ink Rule.** The violet accent marks interaction and first sightings, nothing else. Diff additions are neutral (`sunken`), not accent; chart lines are ink, not accent.

**The Plate Rule.** A series hue means "this program" and only in traffic marks. It follows the program, not its rank or the range on screen: hues are dealt from the seven-day ranking and remembered between windows, a program keeps its hue while it stays among those five, and a sixth program is gray rather than a sixth hue. A mark wears a hue only when that one program is behind all of it: an address several programs reach is gray. Text never wears a series hue; the name beside the mark is in ink, so identity is never color alone.

**The Quiet Green Rule.** Calm green is an 8 px dot beside a sentence, never a badge, fill or verdict, and never next to the words safe, clean, protected or secure.

## Typography

**Display Font:** none; the title role is the largest size.
**Body Font:** Atkinson Hyperlegible Next Variable (with ui-sans-serif, system-ui, sans-serif)
**Label/Mono Font:** Atkinson Hyperlegible Mono Variable (with ui-monospace, monospace)

**Character:** A legibility face built for low-vision readers, used plainly: no tracking changes, no uppercase, no italics. The mono twin shares its skeleton, so a path inside a sentence does not jolt.

### Hierarchy
- **Title** (600, 1.25rem / 20 px, 1.75rem): page h1 in the PageHeader, the "Bigman" wordmark in the rail, a StatusTile's value, a dialog title.
- **Body** (400 or 600, 0.9375rem / 15 px, 1.375rem): sentences, table cells, timeline summaries, buttons (600), section h2 (600), rail tabs (600 when current). Prose is held to 56ch.
- **Small** (400 or 600, 0.8125rem / 13 px, 1.125rem): labels and headers (600, `ink-muted`): tile labels, field labels, table headers, KeyValue keys, day headings (600, `ink`); and secondary detail (400): timestamps, tallies' sentence, status strip, hints.
- **Mono** (400, 13 px, 20 px in evidence blocks): evidence, diffs, paths, hashes, tool errors, path-like table cells.

Tabular numerals are set on `html` (`font-variant-numeric: tabular-nums`), so every figure in the app is tabular.

### Named Rules
**The Three Sizes Rule.** Only `text-small`, `text-body` and `text-title` exist; Tailwind's type scale is removed. Hierarchy within a size comes from weight (600) and ink (`ink` vs `ink-muted`), not from new sizes.

**The Evidence Is Mono Rule.** Anything copied from the machine (paths, hashes, file contents, tool error text) is set in the mono face; anything Bigman says in its own voice is sans. A rule note beside an evidence line switches back to sans 600.

## Layout

A fixed two-column desktop shell: a 12rem (192 px) graphite rail and a fluid content column, with a 32 px status strip at the foot of the content column. Minimum window 960 x 600; nothing scrolls horizontally, the main region scrolls vertically.

- **Rail:** 12 px side padding, 16 px top, B mark and "Bigman" wordmark, then eight tabs (36 px tall, 2 px apart) in a fixed order, each with a 16 px icon, label, an optional accent count badge and its digit (1 to 8) right-aligned as the keyboard hint.
- **Page:** PageHeader (min 56 px, 24 px gutters, hairline below) holds the h1 left, then page controls (the range Select), then the one primary action at the far right. Content below sits in a 24 px gutter with 20 px between sections.
- **Overview:** one lead sentence, a row of five tiles (12 px gaps; three columns below 1280 px, five from there), then the life list (Timeline, 420 px max visible, virtualized).
- **Drill-down:** an absolutely positioned right panel `min(560px, 62%)` over the content; the page behind becomes inert and clicking it closes the panel.
- **Dialogs:** centered, `min(640px, 100vw - 48px)`, max 85vh, over the `scrim` token (35% near-black in light, 55% black in dark).
- **Spacing rhythm:** Tailwind's 4 px unit. Working steps are 4, 8, 12, 16, 20, 24 px, with frequent 2 px and 6 px half-steps for badges, icon gaps and chip spacing, and a few 10 and 14 px steps (tile padding, timeline column gap).
- **Columns:** figures right-align (DataTable `align: "right"`, timeline times in a 3rem right-aligned column); before/after views (DiffView) use fixed 2.5rem gutters so both versions line up column for column.

## Elevation & Depth

Flat and tonal. Resting surfaces never cast shadows: depth reads from `ground` vs `surface` vs `sunken` and from hairlines. A single shadow token exists and is used only by surfaces that float over the page.

### Shadow Vocabulary
- **Panel float** (`--shadow-panel`; light `0 8px 28px oklch(20% 0.01 90 / 0.14), 0 1px 3px oklch(20% 0.01 90 / 0.08)`, dark `0 10px 32px oklch(0% 0 0 / 0.5), 0 1px 3px oklch(0% 0 0 / 0.4)`): the Drilldown panel, the PreviewDialog, Toasts.

### Named Rules
**The Float-Only Shadow Rule.** A shadow means "this sits above the page and will go away." Tiles, tables, timelines and evidence are outlined, never lifted.

## Shapes

Two radii and a pill. Controls (buttons, inputs, selects, badges, focus rings, skeleton bars) take 4 px; containers (tiles, tables, the timeline frame, evidence, dialogs, toasts, empty states) take 6 px; chips, the toggle, count badges, status dots and progress bars are full pills. Borders are 1 px hairlines; the only 2 px line is the selected SegmentedTab underline in `ink`. Empty states use a dashed `rule-strong` outline, the one dashed border in the system. Icons are Lucide outlines at 13 to 16 px, stroke 1.75 to 2.25; the B mark is a monochrome outline drawn from the product logo, in `rail-ink`.

## Components

### Buttons (ActionButton)
Every action control; it names its action and shows progress inline.
- **Shape:** 32 px tall, 12 px horizontal padding, 4 px radius, 1 px border, body 600, optional 15 px leading icon, 6 px gap.
- **Primary:** accent fill and border, `accent-ink` text. One per page header (e.g. "Run checkup"), plus the confirm and close buttons in dialogs.
- **Secondary (default):** `surface` fill, `rule-strong` border, `ink` text.
- **Quiet:** transparent, no visible border, `ink` text; Cancel, "Open the latest", inline actions.
- **Hover / Focus:** primary brightens (`brightness(1.1)`), secondary fills `ground`, quiet fills `sunken`, 150 ms. Focus is the global violet ring.
- **Busy / Disabled:** busy swaps the icon for a spinning loader and sets `aria-busy`; disabled is 50% opacity with a not-allowed cursor.

### Chips (FilterChips)
- **Style:** 28 px pills, small text, `surface` with `rule-strong` border; an optional count in `ink-muted`.
- **State:** selected inverts to an `ink` fill with `surface` text and a 12 px check, so selection is never color alone. An empty selection means "all".

### Cards / Containers (StatusTile and panels)
- **Corner Style:** 6 px.
- **Background:** `surface` on `ground`.
- **Shadow Strategy:** none (see Elevation).
- **Border:** `rule`; actionable tiles step up to `rule-strong`, fill `ground` on hover and carry a 16 px chevron bottom-right.
- **Internal Padding:** tiles 14 px with 28 px on the right for the chevron, min height 112 px; generic panels 16 px.
- **StatusTile states:** loading (two `sunken` blocks), ready (title value in `ink`, optional SeverityBadge top-right), empty (value in `ink-muted`), incomplete (the word "Incomplete"), disconnected (an em dash and the whole tile at 60% opacity).

### Inputs / Fields
- **Style:** TextField, SearchField and Select are 32 px, `surface`, `rule-strong` border, 4 px radius, 12 px padding; labels sit above (small 600, `ink-muted`), hints below (small). SearchField is 16rem wide with a 14 px search glyph, a "/" hint in the placeholder and a clear button; Esc clears it. Select is the native control with a custom chevron. TextField can switch to mono for IDs and URLs.
- **Focus:** the global violet ring.
- **Toggle:** a 36 x 20 px pill switch; on is an accent track with an `accent-ink` knob, off is a `sunken` track with an `ink-muted` knob; 150 ms slide.

### Navigation
- **Rail tabs:** graphite rail, `rail-ink-muted` text and 1.75-stroke icons at rest; hover and current fill `rail-active`, current is `rail-ink` 600 with a 2.25-stroke icon and `aria-current="page"`. An accent pill shows unread first-seen counts.
- **SegmentedTabs** (Protect, Inspect sub-views): 40 px text tabs on a hairline, selected has a 2 px `ink` underline and 600 weight; WAI-ARIA tabs with arrow-key movement.
- **Status strip:** 32 px, `surface`, hairline above, small `ink-muted` text, 20 px between items: daemon state with its dot (calm when running, hollow ring when down, `rule-strong` while connecting), canary state, signature age, last entry time. It is a polite live region.

### Life List (Timeline, FirstSeenMarker, Tally)
The signature component. A virtualized listbox inside a 6 px `surface` frame.
- **Grouping:** newest day first; each day opens with a 36 px heading (small 600, `ink`) bottom-aligned on a `rule-strong` hairline.
- **Ordering:** within a day, first sightings sit on top, then newest first.
- **Entry row:** a five-column grid (3rem right-aligned time, 1rem first-seen column, 1rem source icon, the one-sentence summary clamped to two lines, a right badge column) on a hairline, min 40 px, 8 px x 16 px padding.
- **First-seen mark:** a 12 px ringed dot (ring 1.5 px, centre 2 px radius) in the accent, like a pencil circle around a first sighting; it always carries the text "First seen" (visible in the full form, screen-reader in the compact row form).
- **Folding:** repeats of the same thing fold into one entry showing the latest occurrence; beneath its sentence a tally (four strokes and a slash per five, 1.4 px `ink-muted` strokes, capped at twenty marks) and "seen N times, first on <date>".
- **Severity:** Review and High show a SeverityBadge at the right; Info entries carry no badge.
- **States:** hover `ground`; the keyboard cursor (`sunken`) shows only while the list has focus; arrows, Home, End move, Enter opens the drill-down. New rows fade in (opacity only, 200 ms). Empty shows one sentence and one action inside the same frame.

### SeverityBadge
Small 600 text with a 13 px icon on the severity's soft wash, 4 px radius, 2 x 6 px padding: Info (info circle), Review (eye), High (warning triangle).

### DataTable
Sortable, filterable, virtualized grid in a 6 px frame. Sticky 32 px header row (small 600, `ink-muted`, `rule-strong` underline, sort arrows), 36 px rows on hairlines (taller where a cell wraps), cells truncate with a title tooltip, mono cells drop to small. Same focus-only keyboard cursor as the Timeline, plus PageUp/PageDown. No-match text sits in the table; empty data uses EmptyState instead.

### Drilldown
The single implementation of the three-level rule: a right side panel with two depths. 48 px header with a back button naming the parent (depth two), the title (body 600, receives focus) and a close button; 16 px padded scrolling body. Slides in over 180 ms (`translateX(16px)` and fade, `cubic-bezier(0.16, 1, 0.3, 1)`). Esc goes back one depth, then closes.

### Evidence (EvidenceBlock, DiffView, FileTree, KeyValue)
- **EvidenceBlock:** a `sunken` 6 px figure with a caption bar (label small 600, Copy button), mono small at 20 px leading, right-aligned line numbers in `ink-muted`; matched lines tint `review-soft` or `high-soft` with a sans 600 note in the severity ink. Truncates at 2,000 lines with a sentence saying so.
- **DiffView:** `surface` figure, hunk headers on `sunken`, fixed old/new number gutters, additions on `sunken`, removals in `ink-muted`, rule-matched lines `review-soft` with the rule name in Review ink.
- **FileTree:** disclosure groups by category with counts right-aligned; groups containing Review or High open by default; files in mono small with change word and badge.
- **KeyValue:** a two-column definition grid, keys small 600 `ink-muted`, values body (mono small when machine text).

### Charts (ThroughputChart, Sparkline, StackedAreaChart, ShareList)
ThroughputChart and Sparkline are monochrome by rule: received is a solid 1.75 px `ink` line, sent a dashed 1.25 px `ink-muted` line, gridlines `rule`, axis labels `ink-muted` at the small size (13 px). A legend caption above states the current and peak figures as text. Data updates in place without rebuilding; a theme change repaints from tokens. Sparkline is a 96 x 24 px 1.5 px `ink-muted` polyline beside a real figure, never instead of it.

StackedAreaChart shows parts of a whole over time (Traffic: bytes per second by program, received and sent together). One filled band per layer in its series hue, in the order of the Programs list with the neutral remainder on top; bands are parted by a 1 px line of the `ground` along their shared edge, never outlined, and the stack's top edge has no line. Time that was not collected is a break in the bands, named in the caption ("Not recorded 13:40-13:55"), never a zero. Hairline `rule` gridlines at round byte steps in one unit per axis, a `rule-strong` baseline, axis labels `ink-muted` at the small size, end labels anchored inward. More intervals than the width can show are averaged to one point per 3 px. The chart is a slider: pointing at a time, or Left, Right, PageUp, PageDown, Home and End while it has focus, moves an `ink` hairline and a floating readout (the float shadow) listing every layer at that time, value in ink 600 first, name after, with the date once the range crosses midnight; the same sentence is the slider's value text, and Esc hides the readout. A legend naming each hue sits in the caption.

ShareList is one column of a breakdown (programs, addresses, traffic types): a small 600 `ink-muted` h3 with its count, then the five largest rows. Each row is three lines: a 10 px swatch (when the list has identities), the name (mono small for addresses, IPv4 never truncated, IPv6 shortened in the middle), the FirstSeenMarker when the program first reached the network in the last day, and the total right-aligned in ink; then received and sent as figures with down and up arrows in small `ink-muted`, with an optional second fact right-aligned; then a 6 px pill bar scaled to the list's largest row, received solid in the hue, a 2 px gap, sent as a 1 px outline in the same hue. ShareKey states that key once above the lists. Rows fill `sunken` on hover and open the Drilldown. A list is one Tab stop: Up, Down, Home and End move within it. A list wider than 38rem runs in two columns. A quiet "Show all N" button opens the full DataTable in the Drilldown.

### Actions (PreviewDialog, ConfirmDanger, Toast, ProgressRow)
- **PreviewDialog:** every state-changing action. Title bar (title size), "What will change" plan, footer with Cancel (quiet) and the named confirm (primary); then a "Done" result step with optional Undo, or "This change did not complete" with the system's message verbatim in an EvidenceBlock. Cannot be dismissed while running.
- **ConfirmDanger:** the same dialog plus a required acknowledgment checkbox in the accent.
- **Toast:** confirmations of completed actions only (never findings), bottom-right above the strip, at most three, 5 s, `surface` with the float shadow.
- **ProgressRow:** a panel with label, percent at right, a 4 px pill bar (`ink` fill scaling from the left over 300 ms, or a sliding `ink-muted` third when indeterminate) and a quiet Cancel.

### Feedback states (Skeleton, EmptyState, IncompleteNotice, StatusLine, CoverageList)
- **Skeleton:** content-shaped bars (tiles, rows, block) shimmering `rule` to `rule-strong` over 1.4 s; never a spinner in content.
- **EmptyState:** a dashed `rule-strong` 6 px outline, one `ink-muted` sentence (max 56ch) and one action.
- **IncompleteNotice:** a `surface` panel with a 16 px muted icon (slashed circle for incomplete, unplugged for disconnected), body 600 heading, an action at right, and a CoverageList.
- **CoverageList:** "Not checked" (what, plain reason, mono detail), "Checked", and "What these checks can tell you", all in small.
- **StatusLine:** the calm dot and "No findings from these checks. Ran at HH:MM with signatures ...". The only calm state.

### Motion and reduced motion
All transitions are 150 ms color/filter changes, except: row fade-in 200 ms ease-out, panel slide 180 ms expo-out, progress fill 300 ms, shimmer and indeterminate bar 1.4 s loops, the busy spinner. Under `prefers-reduced-motion: reduce` every animation and transition collapses to 0.01 ms.

### Keyboard model
Digits 1 to 8 switch tabs and move focus to the new page's h1 (ignored while typing or while a modal dialog is open); `/` focuses the page's SearchField; Esc closes the drill-down (back one depth first) and clears a non-empty search; arrows, Home, End (and PageUp/PageDown in tables) move the cursor; Enter opens the row. A "Skip to content" link appears on first Tab.

## Do's and Don'ts

### Do:
- **Do** take every color, size and radius from `design/tokens.css` by token name; add a token there before using a new value.
- **Do** mark a first sighting with the ringed violet FirstSeenMarker and fold its repeats under it with a Tally and "seen N times, first on <date>".
- **Do** show severity as SeverityBadge (icon plus Info, Review or High) and sort findings severity-first, first-seen on top.
- **Do** right-align figures to one margin and keep before/after columns on shared fixed gutters.
- **Do** open every figure through the single Drilldown panel (tile, then table, then evidence).
- **Do** route every state-changing action through PreviewDialog (plan, confirm, result, undo where possible).
- **Do** keep each screen to the three sizes (13, 15, 20 px) and two weights (400, 600).
- **Do** set machine text (paths, hashes, file contents, tool errors) in Atkinson Hyperlegible Mono.
- **Do** cover all six states with the shared components: Skeleton, EmptyState, the view, StatusLine, IncompleteNotice with CoverageList, and the disconnected notice.

### Don't:
- **Don't** use the info, review or high hues for anything but severity, and don't use the accent for anything but interaction and first sightings.
- **Don't** use a series hue outside traffic marks, as text, or for a sixth program; don't repaint a program because its rank changed.
- **Don't** use calm green as a badge, fill or verdict; it is an 8 px dot beside a sentence.
- **Don't** put a shadow on a resting tile, table, timeline or evidence block; shadows belong to the drill-down, dialog and toasts.
- **Don't** add a fourth type size, uppercase labels, letter-spacing, or a display face.
- **Don't** use the red-shield logo inside the app; the rail carries the monochrome B mark.
- **Don't** build a feature-local side panel, modal flow or loading spinner when Drilldown, PreviewDialog and Skeleton exist.
- **Don't** animate anything that carries no meaning, and don't animate position on Timeline rows (they are positioned by transform; fade opacity only).
