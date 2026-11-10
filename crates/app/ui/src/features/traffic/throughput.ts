import type { ThroughputSeries } from "@/design";
import type { Sample, SampleSeries } from "@/ipc";
import type { Range } from "@/shell/store";

/** Seconds shown, bucket count asked of the store, and how often stored history reloads. */
export const RANGES: Record<Range, { seconds: number; points: number; reload: number; label: string }> = {
  "5m": { seconds: 300, points: 300, reload: 300, label: "Last 5 minutes" },
  "1h": { seconds: 3_600, points: 360, reload: 30, label: "Last hour" },
  "24h": { seconds: 86_400, points: 288, reload: 300, label: "Last 24 hours" },
  "7d": { seconds: 7 * 86_400, points: 336, reload: 1_800, label: "Last 7 days" },
};

/**
 * One series per interface over `[since, until]`. Stored buckets are laid on a
 * fixed grid so a bucket the daemon did not record becomes a gap (null), never a
 * line drawn across the time it was not running. Live one-second samples after
 * the stored range are appended as they are.
 */
export function buildSeries(stored: SampleSeries | undefined, live: readonly Sample[], since: number, until: number): Map<string, ThroughputSeries> {
  const byIface = new Map<string, Map<number, Sample>>();
  const add = (s: Sample) => {
    const points = byIface.get(s.iface) ?? new Map<number, Sample>();
    points.set(s.at, s);
    byIface.set(s.iface, points);
  };
  stored?.samples.forEach(add);
  const storedUntil = stored?.samples.reduce((m, s) => Math.max(m, s.at), 0) ?? 0;
  live.filter((s) => s.at > storedUntil && s.at >= since && s.at <= until).forEach(add);

  const bucket = Math.max(1, stored?.bucket ?? 1);
  const out = new Map<string, ThroughputSeries>();
  for (const [iface, points] of byIface) {
    const series: ThroughputSeries = { at: [], rx: [], tx: [] };
    const push = (at: number, s: Sample | undefined) => {
      series.at.push(at);
      series.rx.push(s?.rx ?? null);
      series.tx.push(s?.tx ?? null);
    };
    const tail = [...points.values()].filter((s) => s.at > storedUntil && s.at >= since).sort((a, b) => a.at - b.at);
    // Without live samples the grid runs to `until`, so a daemon that stopped shows as a gap up to now.
    const gridEnd = tail.length ? storedUntil : until;
    for (let at = Math.floor(since / bucket) * bucket; at <= gridEnd; at += bucket) push(at, points.get(at));
    // A live sample that starts more than a bucket after stored history leaves a gap, not a bridge.
    let previous = gridEnd;
    for (const s of tail) {
      if (s.at - previous > bucket) push(previous + bucket, undefined);
      push(s.at, s);
      previous = s.at;
    }
    out.set(iface, series);
  }
  return out;
}

/** Busiest interface first, so the one in use leads. */
export function byTraffic(series: Map<string, ThroughputSeries>): [string, ThroughputSeries][] {
  const total = (s: ThroughputSeries) => s.rx.reduce<number>((sum, v) => sum + (v ?? 0), 0);
  return [...series].sort((a, b) => total(b[1]) - total(a[1]));
}
