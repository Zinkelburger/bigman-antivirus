import { useEffect, useId, useMemo, useRef, useState, type KeyboardEvent, type PointerEvent } from "react";
import { formatBytes, formatClock, formatDate, formatTime } from "../format";
import { byteTicks, rateTickLabel, tickLabel, timeTicks } from "./chart-axis";

export interface AreaLayer {
  id: string;
  label: string;
  /** Series hue 1 to 5, or null for the neutral remainder. */
  slot: number | null;
  /** Bytes per second in each interval, aligned to the chart's `at`. */
  values: readonly number[];
}

interface StackedAreaChartProps {
  /** What the whole stack is, e.g. "Traffic by program". */
  label: string;
  /** Start of each interval (Unix seconds), ascending. */
  at: readonly number[];
  /** Seconds each interval covers. */
  bucket: number;
  /** Bottom of the stack first. */
  layers: readonly AreaLayer[];
  /** Whether each interval was collected; a missing one is a break, never a zero. */
  recorded?: readonly boolean[] | undefined;
  height?: number;
}

const GUTTER = 76;
const TOP = 8;
const BOTTOM = 24;
const RIGHT = 8;
const FALLBACK_WIDTH = 720;
/** Horizontal pixels each drawn point gets; denser data is averaged down to this. */
const POINT_SPACING = 3;

/** The token behind a layer's mark. */
export function seriesColor(slot: number | null): string {
  return slot === null ? "var(--series-rest)" : `var(--series-${String(slot)})`;
}

const rate = (v: number) => `${formatBytes(v)}/s`;

/** Runs of intervals that were not collected, as "13:05–13:15", for a caption. */
export function unrecorded(at: readonly number[], recorded: readonly boolean[] | undefined, bucket: number): string[] {
  const out: string[] = [];
  let start: number | null = null;
  for (let i = 0; i < at.length; i++) {
    const t = at[i] ?? 0;
    const missing = recorded !== undefined && recorded[i] === false;
    if (missing && start === null) start = t;
    else if (!missing && start !== null) {
      out.push(`${formatTime(start)}–${formatTime(t)}`);
      start = null;
    }
  }
  if (start !== null) out.push(`${formatTime(start)}–${formatTime((at[at.length - 1] ?? start) + bucket)}`);
  return out;
}

interface Points {
  at: number[];
  bucket: number;
  recorded: boolean[];
  layers: { layer: AreaLayer; values: number[] }[];
}

/** Averages neighbouring intervals so no more points are drawn than the width can show. */
function fit(at: readonly number[], bucket: number, layers: readonly AreaLayer[], recorded: readonly boolean[] | undefined, most: number): Points {
  const group = Math.max(1, Math.ceil(at.length / Math.max(2, most)));
  const starts = at.filter((_, i) => i % group === 0);
  const mean = (values: readonly (number | undefined)[], from: number) => {
    const slice = values.slice(from, from + group);
    return slice.reduce<number>((sum, v) => sum + (v ?? 0), 0) / Math.max(1, slice.length);
  };
  return {
    at: starts,
    bucket: bucket * group,
    // A drawn point counts as collected when any interval inside it was.
    recorded: starts.map((_, i) => recorded === undefined || recorded.slice(i * group, i * group + group).some(Boolean)),
    layers: layers.map((layer) => ({ layer, values: starts.map((_, i) => mean(layer.values, i * group)) })),
  };
}

/** Consecutive collected points, as [first, last] index pairs. */
function runs(recorded: readonly boolean[]): [number, number][] {
  const out: [number, number][] = [];
  let start = -1;
  recorded.forEach((ok, i) => {
    if (ok && start < 0) start = i;
    if (start >= 0 && (!ok || i === recorded.length - 1)) {
      out.push([start, ok ? i : i - 1]);
      start = -1;
    }
  });
  return out;
}

/**
 * Parts of a whole over time: one filled band per layer, stacked from the bottom,
 * each parted from the next by a hairline of the page ground. Time that was not
 * collected is a break in the bands. Pointing at a time, or moving with the arrow
 * keys while the chart has focus, reads out every layer at that time.
 */
export function StackedAreaChart({ label, at, bucket, layers, recorded, height = 220 }: StackedAreaChartProps) {
  const host = useRef<HTMLDivElement>(null);
  const [width, setWidth] = useState(FALLBACK_WIDTH);
  const [cursor, setCursor] = useState<number | null>(null);
  const readoutId = useId();

  useEffect(() => {
    const element = host.current;
    if (!element || typeof ResizeObserver === "undefined") return;
    const measure = () => setWidth(Math.max(320, element.clientWidth));
    measure();
    const observer = new ResizeObserver(measure);
    observer.observe(element);
    return () => observer.disconnect();
  }, []);

  const plotWidth = width - GUTTER - RIGHT;
  const plotHeight = height - TOP - BOTTOM;
  const points = useMemo(() => fit(at, bucket, layers, recorded, Math.floor(plotWidth / POINT_SPACING)), [at, bucket, layers, recorded, plotWidth]);
  const n = points.at.length;
  const first = points.at[0] ?? 0;
  const last = points.at[n - 1] ?? first;
  const totals = points.at.map((_, i) => points.layers.reduce((sum, l) => sum + (l.values[i] ?? 0), 0));
  const peak = totals.reduce((m, v) => Math.max(m, v), 0);
  const ticks = byteTicks(Math.max(peak, 1_024));
  const tickStep = ticks[1] ?? 1;
  const top = Math.max(peak * 1.04, ticks[ticks.length - 1] ?? 1);
  const x = (i: number) => GUTTER + (n > 1 ? (i / (n - 1)) * plotWidth : plotWidth / 2);
  const y = (v: number) => TOP + plotHeight - (v / top) * plotHeight;
  const time = timeTicks(first, last, Math.floor(plotWidth / 110));
  const xAt = (t: number) => GUTTER + (last > first ? ((t - first) / (last - first)) * plotWidth : plotWidth / 2);
  // Once a range crosses midnight a clock time alone is ambiguous.
  const dated = new Date(first * 1000).toDateString() !== new Date(last * 1000).toDateString();
  const moment = (t: number) => (dated ? `${formatDate(t)}, ${formatClock(t, false)}` : formatClock(t, points.bucket < 60));

  // Cumulative tops: edges[k][i] is the top of layer k at point i; edges[-1] is the baseline.
  const edges: number[][] = [];
  points.layers.forEach((l, k) => edges.push(l.values.map((v, i) => (edges[k - 1]?.[i] ?? 0) + v)));
  const line = (values: readonly number[], from: number, to: number, reverse = false) => {
    const pairs = [];
    for (let i = from; i <= to; i++) pairs.push(`${x(i).toFixed(1)},${y(values[i] ?? 0).toFixed(1)}`);
    return (reverse ? pairs.reverse() : pairs).join("L");
  };
  const zero = points.at.map(() => 0);
  const spans = runs(points.recorded);

  const nearest = (clientX: number, element: Element) => {
    const box = element.getBoundingClientRect();
    const ratio = (clientX - box.left - GUTTER) / Math.max(1, plotWidth);
    return Math.min(n - 1, Math.max(0, Math.round(ratio * (n - 1))));
  };
  const onMove = (event: PointerEvent<HTMLDivElement>) => {
    if (n > 0) setCursor(nearest(event.clientX, event.currentTarget));
  };
  const onKey = (event: KeyboardEvent<HTMLDivElement>) => {
    if (n === 0) return;
    if (event.key === "Escape" && cursor !== null) {
      // Only the readout closes; whatever else Escape means on the page is left alone.
      event.stopPropagation();
      setCursor(null);
      return;
    }
    const stride = Math.max(1, Math.round(n / 10));
    const move: Partial<Record<string, (c: number) => number>> = {
      ArrowLeft: (c) => c - 1,
      ArrowRight: (c) => c + 1,
      PageDown: (c) => c - stride,
      PageUp: (c) => c + stride,
      Home: () => 0,
      End: () => n - 1,
    };
    const to = move[event.key];
    if (!to) return;
    event.preventDefault();
    setCursor((c) => Math.min(n - 1, Math.max(0, to(c ?? n - 1))));
  };

  const shown = cursor !== null && cursor < n ? cursor : null;
  const collected = shown !== null && points.recorded[shown] !== false;
  const readout =
    shown === null
      ? ""
      : !collected
        ? `${moment(points.at[shown] ?? 0)}: not recorded`
        : `${moment(points.at[shown] ?? 0)}: ${rate(totals[shown] ?? 0)} in all. ${points.layers
            .filter((l) => (l.values[shown] ?? 0) > 0)
            .map((l) => `${l.layer.label} ${rate(l.values[shown] ?? 0)}`)
            .join(", ")}`;
  const flip = shown !== null && x(shown) > GUTTER + plotWidth * 0.6;

  return (
    <div
      ref={host}
      role="slider"
      tabIndex={0}
      aria-label={`${label}, peak ${rate(peak)}. Arrow keys read each time.`}
      aria-orientation="horizontal"
      aria-valuemin={0}
      aria-valuemax={Math.max(0, n - 1)}
      aria-valuenow={shown ?? Math.max(0, n - 1)}
      aria-valuetext={readout || `${label}, peak ${rate(peak)}`}
      className="relative rounded-control"
      style={{ height }}
      onPointerMove={onMove}
      onPointerLeave={() => setCursor(null)}
      onFocus={() => setCursor((c) => c ?? Math.max(0, n - 1))}
      onBlur={() => setCursor(null)}
      onKeyDown={onKey}
    >
      <svg aria-hidden width={width} height={height} viewBox={`0 0 ${String(width)} ${String(height)}`} className="block max-w-full text-small">
        {ticks.map((t) => (
          <g key={t}>
            <line x1={GUTTER} x2={width - RIGHT} y1={y(t)} y2={y(t)} style={{ stroke: "var(--rule)" }} strokeWidth={1} />
            <text x={GUTTER - 8} y={y(t)} textAnchor="end" dominantBaseline="middle" style={{ fill: "var(--ink-muted)" }}>
              {rateTickLabel(t, tickStep)}
            </text>
          </g>
        ))}
        {spans.map(([from, to]) =>
          points.layers.map(({ layer }, k) => (
            <path
              key={`${layer.id}-${String(from)}`}
              d={`M${line(edges[k] ?? zero, from, to)}L${line(edges[k - 1] ?? zero, from, to, true)}Z`}
              style={{ fill: seriesColor(layer.slot) }}
            />
          )),
        )}
        {/* The part between two bands is a line of the ground along their shared edge; the stack's own top has none. */}
        {spans.map(([from, to]) =>
          points.layers.slice(0, -1).map(({ layer }, k) => (
            <path key={`edge-${layer.id}-${String(from)}`} d={`M${line(edges[k] ?? zero, from, to)}`} fill="none" style={{ stroke: "var(--ground)" }} strokeWidth={1} strokeLinejoin="round" />
          )),
        )}
        <line x1={GUTTER} x2={width - RIGHT} y1={y(0)} y2={y(0)} style={{ stroke: "var(--rule-strong)" }} strokeWidth={1} />
        {time.ticks.map((t) => {
          // A label at either end is anchored inward so it is never cut off.
          const anchor = xAt(t) > width - RIGHT - 32 ? "end" : xAt(t) < GUTTER + 32 ? "start" : "middle";
          return (
            <text key={t} x={anchor === "end" ? width - RIGHT : anchor === "start" ? GUTTER : xAt(t)} y={height - 6} textAnchor={anchor} style={{ fill: "var(--ink-muted)" }}>
              {tickLabel(t, time.step)}
            </text>
          );
        })}
        {shown !== null && <line x1={x(shown)} x2={x(shown)} y1={TOP} y2={y(0)} style={{ stroke: "var(--ink)" }} strokeWidth={1} />}
      </svg>
      {shown !== null && (
        <div
          className="pointer-events-none absolute top-2 z-10 max-w-64 min-w-44 rounded-panel border border-rule bg-surface px-3 py-2 text-small shadow-panel"
          style={flip ? { right: width - x(shown) + 10 } : { left: x(shown) + 10 }}
        >
          <p className="m-0 flex justify-between gap-4 text-ink-muted">
            <span>{moment(points.at[shown] ?? 0)}</span>
            <span className="font-semibold text-ink">{collected ? rate(totals[shown] ?? 0) : "Not recorded"}</span>
          </p>
          {collected && (
            <ul className="m-0 mt-1 flex list-none flex-col gap-0.5 p-0">
              {[...points.layers].reverse().map((l) => (
                <li key={l.layer.id} className="flex items-center gap-2">
                  <span aria-hidden className="h-0.5 w-3 shrink-0 rounded-full" style={{ background: seriesColor(l.layer.slot) }} />
                  <span className="shrink-0 font-semibold text-ink">{rate(l.values[shown] ?? 0)}</span>
                  <span className="truncate text-ink-muted">{l.layer.label}</span>
                </li>
              ))}
            </ul>
          )}
        </div>
      )}
      <p id={readoutId} aria-live="polite" className="sr-only">
        {readout}
      </p>
    </div>
  );
}
