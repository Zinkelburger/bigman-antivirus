import type { LucideIcon } from "lucide-react";
import type { Severity } from "@/ipc";
import { formatDay } from "./format";

/** One life-list entry: the first sighting of something, with its repeats folded in. */
export interface TimelineEntry {
  id: string;
  /** Latest occurrence (Unix seconds). */
  at: number;
  summary: string;
  /** What noticed it, e.g. "Network", "Receipt". Drives the filter chips. */
  source: string;
  icon?: LucideIcon | undefined;
  severity: Severity;
  /** True when this is the first time it was ever seen. */
  firstSeen: boolean;
  /** How many times it has been seen, including this one. */
  count: number;
  /** When it was first seen. */
  firstAt: number;
  /** Arrived since the user last marked Activity as seen. */
  unread?: boolean | undefined;
}

/** Raw occurrences before folding; `key` says what makes two of them the same thing. */
export interface Occurrence {
  id: string;
  key: string;
  at: number;
  summary: string;
  source: string;
  icon?: LucideIcon | undefined;
  severity: Severity;
  /** Times seen before this occurrence, from the store; 0 means first seen. */
  seenBefore: number;
  /** When the store first saw this key, which may be before anything loaded. */
  firstAt?: number | undefined;
}

/**
 * Folds recurring occurrences into one entry per key. The entry shows the latest
 * occurrence; it is marked first-seen only when that occurrence was the first ever.
 */
export function fold(occurrences: readonly Occurrence[]): TimelineEntry[] {
  const byKey = new Map<string, { latest: Occurrence; firstAt: number; count: number }>();
  for (const o of occurrences) {
    const group = byKey.get(o.key);
    if (!group) {
      // A missing or zero first time (never recorded) falls back to this occurrence.
      const known = o.firstAt !== undefined && o.firstAt > 0 ? o.firstAt : o.at;
      byKey.set(o.key, { latest: o, firstAt: Math.min(known, o.at), count: o.seenBefore + 1 });
      continue;
    }
    if (o.at > group.latest.at) group.latest = o;
    group.firstAt = Math.min(group.firstAt, o.firstAt !== undefined && o.firstAt > 0 ? o.firstAt : o.at, o.at);
    group.count = Math.max(group.count, o.seenBefore + 1);
  }
  return [...byKey.values()].map(({ latest, firstAt, count }) => ({
    id: latest.id,
    at: latest.at,
    summary: latest.summary,
    source: latest.source,
    icon: latest.icon,
    severity: latest.severity,
    firstSeen: latest.seenBefore === 0,
    count,
    firstAt,
  }));
}

export type TimelineRow =
  | { kind: "day"; key: string; label: string }
  | { kind: "entry"; key: string; entry: TimelineEntry; day: string };

/** Newest day first; within a day first sightings on top, then newest first. */
export function layout(entries: readonly TimelineEntry[], now: number): TimelineRow[] {
  const days = new Map<string, TimelineEntry[]>();
  for (const e of [...entries].sort((a, b) => b.at - a.at)) {
    const label = formatDay(e.at, now);
    const list = days.get(label) ?? [];
    list.push(e);
    days.set(label, list);
  }
  const rows: TimelineRow[] = [];
  for (const [label, list] of days) {
    rows.push({ kind: "day", key: `day-${label}`, label });
    list.sort((a, b) => Number(b.firstSeen) - Number(a.firstSeen) || b.at - a.at);
    for (const entry of list) rows.push({ kind: "entry", key: entry.id, entry, day: label });
  }
  return rows;
}
