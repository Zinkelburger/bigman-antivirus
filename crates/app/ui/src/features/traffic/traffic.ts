// What the Traffic page derives from a TrafficReport: stable colors for the
// programs that moved the most, the chart's layers, and the address and
// traffic-type breakdowns added up from the report's flows.
import type { AreaLayer, ShareRow } from "@/design";
import type { Flow, TrafficReport } from "@/ipc";
import type { Range } from "@/shell/store";

/** Seconds shown, intervals asked of the store, and how often the view reloads. */
export const TRAFFIC_RANGES: Record<Range, { seconds: number; points: number; reload: number }> = {
  "5m": { seconds: 300, points: 60, reload: 5 },
  "1h": { seconds: 3_600, points: 360, reload: 10 },
  "24h": { seconds: 86_400, points: 288, reload: 60 },
  "7d": { seconds: 7 * 86_400, points: 168, reload: 300 },
};

/** The daemon files what it reads into five-second intervals and saves each once it has ended. */
const SAVE_LAG = 8;
/** Programs that get a hue of their own; the rest share the neutral band. */
export const SERIES_SLOTS = 5;
/** Rows a breakdown shows before "Show all". */
export const TOP_ROWS = 5;
/** A program first seen on the network this recently carries the first-seen mark. */
const FIRST_SEEN_RECENT = 86_400;

type Who = { program: string | null; uid: number };

export const programKey = (p: Who) => (p.program === null ? `uid:${String(p.uid)}` : `program:${p.program}`);

/**
 * A program by name, or whose it is when Bigman cannot name it. `ownUid` is the
 * account Bigman runs as: an unnamed program of that account is yours, hidden
 * by a sandbox, not somebody else's.
 */
export function programLabel(p: Who, ownUid: number): string {
  if (p.program !== null) return p.program;
  if (p.uid === ownUid) return "Your programs Bigman cannot name";
  return p.uid === 0 ? "System programs (root)" : `Another account's programs (uid ${String(p.uid)})`;
}

const STORED = "bigman.traffic.hues";
let assigned: Map<string, number> | null = null;

/** The hues handed out so far; they outlive the window so a program looks the same tomorrow. */
function held(): Map<string, number> {
  if (assigned) return assigned;
  assigned = new Map();
  try {
    const saved: unknown = JSON.parse(localStorage.getItem(STORED) ?? "[]");
    if (Array.isArray(saved)) {
      for (const entry of saved as unknown[]) {
        if (Array.isArray(entry) && typeof entry[0] === "string" && Number.isInteger(entry[1]) && entry[1] >= 1 && entry[1] <= SERIES_SLOTS) {
          assigned.set(entry[0], entry[1] as number);
        }
      }
    }
  } catch {
    // Storage can be unavailable or hold something else; hues are then dealt afresh.
  }
  return assigned;
}

/**
 * Hues for the programs that lead `ranking`, by program key. A program keeps its
 * hue for as long as it stays among the five, whatever its rank; a newcomer takes
 * a hue nobody holds. The page ranks over the last seven days whatever range is
 * shown, so changing the range never repaints a program: color follows the
 * program, never its position.
 */
export function slotsFor(ranking: readonly string[]): ReadonlyMap<string, number> {
  const slots = held();
  const keep = new Set(ranking.slice(0, SERIES_SLOTS));
  let changed = false;
  for (const key of [...slots.keys()]) {
    if (!keep.has(key)) changed = slots.delete(key) || changed;
  }
  for (const key of keep) {
    if (slots.has(key)) continue;
    const used = new Set(slots.values());
    for (let slot = 1; slot <= SERIES_SLOTS; slot++) {
      if (!used.has(slot)) {
        slots.set(key, slot);
        changed = true;
        break;
      }
    }
  }
  if (changed) {
    try {
      localStorage.setItem(STORED, JSON.stringify([...slots]));
    } catch {
      // The hues still hold for this window.
    }
  }
  return new Map(slots);
}

/** Forgets every assignment; tests start from a clean palette. */
export function resetSlots(): void {
  assigned = null;
  try {
    localStorage.removeItem(STORED);
  } catch {
    // Nothing was stored.
  }
}

export interface ChartData {
  at: number[];
  bucket: number;
  /** Whether the collector was running in each interval. */
  recorded: boolean[];
  /** Bottom of the stack first: programs with a hue, most data first, then everything else. */
  layers: AreaLayer[];
}

/**
 * The stacked chart's layers in bytes per second (received plus sent). The
 * interval still being collected is left off, so the right edge is the last
 * complete reading rather than a dip to nothing.
 */
export function chartData(report: TrafficReport, slots: ReadonlyMap<string, number>): ChartData {
  const complete = report.at.filter((at) => at + report.bucket <= report.until - SAVE_LAG).length;
  const rate = (rx: readonly number[], tx: readonly number[]) =>
    Array.from({ length: complete }, (_, i) => ((rx[i] ?? 0) + (tx[i] ?? 0)) / report.bucket);
  const layers: AreaLayer[] = [];
  const rest = rate(report.rest_rx, report.rest_tx);
  for (const s of report.series) {
    const slot = slots.get(programKey(s));
    const values = rate(s.rx, s.tx);
    if (slot === undefined) values.forEach((v, i) => (rest[i] = (rest[i] ?? 0) + v));
    else layers.push({ id: programKey(s), label: programLabel(s, report.own_uid), slot, values });
  }
  // The report lists series most data first, which is the order of the Programs list beside the chart.
  if (rest.some((v) => v > 0)) layers.push({ id: "rest", label: "Other programs", slot: null, values: rest });
  return { at: report.at.slice(0, complete), bucket: report.bucket, recorded: report.recorded.slice(0, complete), layers };
}

export function programRows(report: TrafficReport, slots: ReadonlyMap<string, number>): ShareRow[] {
  return report.programs.map((p) => ({
    id: programKey(p),
    name: programLabel(p, report.own_uid),
    slot: slots.get(programKey(p)) ?? null,
    swatch: true,
    firstSeen: p.first_at !== null && report.until - p.first_at < FIRST_SEEN_RECENT,
    received: p.rx,
    sent: p.tx,
  }));
}

/** Private, link-local and carrier-shared ranges: addresses that are not on the internet. */
export function isLocal(address: string): boolean {
  if (address.includes(":")) return /^f[cd]/i.test(address) || /^fe[89ab]/i.test(address);
  const [a = 0, b = 0] = address.split(".").map(Number);
  return a === 10 || (a === 172 && b >= 16 && b <= 31) || (a === 192 && b === 168) || (a === 169 && b === 254) || (a === 100 && b >= 64 && b <= 127);
}

interface Sum {
  received: number;
  sent: number;
  /** Bytes by program, to name the one behind most of it. */
  by: Map<string, { who: Who; bytes: number }>;
}

function sumBy(flows: readonly Flow[], key: (f: Flow) => string): Map<string, Sum> {
  const out = new Map<string, Sum>();
  for (const f of flows) {
    const sum = out.get(key(f)) ?? { received: 0, sent: 0, by: new Map<string, { who: Who; bytes: number }>() };
    sum.received += f.rx;
    sum.sent += f.tx;
    const who = sum.by.get(programKey(f)) ?? { who: f, bytes: 0 };
    who.bytes += f.rx + f.tx;
    sum.by.set(programKey(f), who);
    out.set(key(f), sum);
  }
  return out;
}

const largestFirst = (a: ShareRow, b: ShareRow) => b.received + b.sent - (a.received + a.sent) || a.name.localeCompare(b.name);

/** An IPv6 address shortened in the middle, where the least telling groups are. */
export function shortAddress(address: string): string {
  return address.length > 24 ? `${address.slice(0, 14)}…${address.slice(-9)}` : address;
}

/**
 * Addresses, each naming the program behind most of its traffic. An address
 * wears a program's hue only when that program is behind all of it; shared by
 * several, it is neutral, because the bar would otherwise claim their bytes.
 */
export function hostRows(flows: readonly Flow[], slots: ReadonlyMap<string, number>, ownUid: number): ShareRow[] {
  return [...sumBy(flows, (f) => f.remote)]
    .map(([remote, sum]): ShareRow => {
      const lead = [...sum.by.values()].sort((a, b) => b.bytes - a.bytes)[0];
      const others = sum.by.size - 1;
      return {
        id: remote,
        name: shortAddress(remote),
        title: remote,
        mono: true,
        detail: lead ? `${programLabel(lead.who, ownUid)}${others > 0 ? ` and ${String(others)} more` : ""}` : undefined,
        slot: lead && others === 0 ? (slots.get(programKey(lead.who)) ?? null) : null,
        swatch: true,
        received: sum.received,
        sent: sum.sent,
      };
    })
    .sort(largestFirst);
}

const SERVICES: Partial<Record<number, string>> = {
  22: "SSH",
  25: "Mail delivery",
  53: "DNS",
  80: "Web, not encrypted",
  143: "Mail (IMAP)",
  443: "Encrypted web",
  465: "Mail sending",
  587: "Mail sending",
  853: "DNS over TLS",
  993: "Mail (IMAP)",
  995: "Mail (POP)",
  5222: "Chat (XMPP)",
  8080: "Web, not encrypted",
  8443: "Encrypted web",
};

/**
 * A port and its usual service. The port leads because it is the evidence and
 * must survive a narrow column; the name is only the convention.
 */
export function serviceLabel(port: number, inbound: boolean): string {
  const name = SERVICES[port];
  if (name) return `${inbound ? "Incoming " : ""}${String(port)} ${name}`;
  return inbound ? `Incoming port ${String(port)}` : `Port ${String(port)}`;
}

export const typeKey = (f: { port: number; inbound: boolean }) => `${f.inbound ? "in" : "out"}:${String(f.port)}`;

export function typeRows(flows: readonly Flow[]): ShareRow[] {
  const labels = new Map(flows.map((f) => [typeKey(f), serviceLabel(f.port, f.inbound)]));
  return [...sumBy(flows, typeKey)]
    .map(([key, sum]): ShareRow => ({ id: key, name: labels.get(key) ?? key, received: sum.received, sent: sum.sent }))
    .sort(largestFirst);
}

export const total = (rows: readonly { rx: number; tx: number }[]) =>
  rows.reduce((sum, r) => ({ rx: sum.rx + r.rx, tx: sum.tx + r.tx }), { rx: 0, tx: 0 });
