import { describe, expect, it } from "vitest";
import { fold, layout, type Occurrence } from "./timeline";

const day = 86_400;
const now = Math.floor(Date.UTC(2026, 9, 29, 14, 0) / 1000);
const occurrence = (key: string, at: number, seenBefore: number): Occurrence => ({
  id: `${key}-${at}`,
  key,
  at,
  summary: key,
  source: "Network",
  severity: "info",
  seenBefore,
});

describe("life-list folding", () => {
  it("folds repeats under their first sighting", () => {
    const entries = fold([
      occurrence("syncthing", now - 26 * day, 0),
      ...Array.from({ length: 13 }, (_, i) => occurrence("syncthing", now - (12 - i) * day, i + 1)),
    ]);
    expect(entries).toHaveLength(1);
    expect(entries[0]).toMatchObject({ count: 14, firstAt: now - 26 * day, at: now, firstSeen: false });
  });

  it("marks an entry first-seen only when its latest occurrence is the first ever", () => {
    const [entry] = fold([occurrence("firefox", now, 0)]);
    expect(entry?.firstSeen).toBe(true);
  });

  it("puts first sightings on top within a day, newest day first", () => {
    const rows = layout(
      fold([occurrence("old", now - 60, 3), occurrence("new", now - 600, 0), occurrence("yesterday", now - day, 0)]),
      now,
    );
    const order = rows.map((r) => (r.kind === "day" ? `# ${r.label}` : r.entry.summary));
    expect(order).toEqual(["# Today", "new", "old", "# Yesterday", "yesterday"]);
  });
});

describe("large timelines", () => {
  it("lays out 10,000 entries in well under a frame budget", () => {
    const many = Array.from({ length: 10_000 }, (_, i) => occurrence(`k${i}`, now - i * 60, i % 7));
    const started = performance.now();
    const rows = layout(fold(many), now);
    expect(rows.filter((r) => r.kind === "entry")).toHaveLength(10_000);
    expect(performance.now() - started).toBeLessThan(500);
  });
});
