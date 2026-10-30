---
version: 1
slug: "crates-app-ui-src"
primary_target: "crates/app/ui/src"
related_targets: []
---

# Bigman desktop app (all seven tabs and the component gallery)

Mode: Operate. The visitor is one person on one Linux workstation checking, in passing, whether anything on the machine is new or worth a look, and inspecting things before running them. Product truth lives in PRODUCT.md; the UI contract (states, copy, components, keyboard) is Part B of docs/plan/implementation-plan.md and is binding.

## Direction contract

THESIS: Bigman keeps a naturalist's life list of your machine. Everything it notices is entered once, on the day it was first seen; repeats fold into a tally beneath that first entry, so what is new stands out without alarm. It refuses the consumer-antivirus shield dashboard and the dark SOC console.

OWN-WORLD: Neutral, untinted ground and panels; a graphite shell rail like a field notebook's cloth cover; one accent, field-guide range-map violet, for interaction and the first-seen mark only. Severity colors reserved (Info muted blue, Review amber, High red); calm green only in the status strip and tray. Atkinson Hyperlegible Next for UI with tabular figures, Atkinson Hyperlegible Mono for evidence. Three sizes per screen. Hairline rules, dated entry groups, 4 px grid.

STORY: The visitor sees at a glance what is new since they last looked and whether collection is healthy, understands each entry in one plain sentence, opens any number to its table and any row to its evidence, and acts only through previewed changes.

FIRST VIEWPORT: Graphite rail at left with the B mark and seven numbered tabs. Page header: "Overview", range picker, primary action "Run checkup" at top right. One row of four status tiles, "Since you last looked" first. Below, today's life list: day heading, first sightings on top with the violet mark, recurring entries folded with an authored tally, each row opening a drill-down panel at right. A thin status strip at the foot: daemon state, signature age, last entry time.

FORM: Life List, ranked 1 of 7 (the user's choice; the roll assigned Timetable), seed 3b379b4c. Signature move: first-sighting entries with tally folding in the Timeline. Raises kept from declined challengers: live rows update in place (split-flap); before/after views share one scale and column positions (botanical folio); figures right-aligned to one margin (j-card); one fixed ramp for volume marks (star atlas); one angle grammar for drawn connectors (transit map).

FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance
