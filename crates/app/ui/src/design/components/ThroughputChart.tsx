import { useEffect, useRef, useState } from "react";
import uPlot from "uplot";
import "uplot/dist/uPlot.min.css";
import { formatBytes, formatClock, formatDate, formatTime } from "../format";
import { subscribeTheme, token } from "../theme";

export interface ThroughputSeries {
  /** Sample times (Unix seconds), ascending. */
  at: number[];
  /** Bytes per second received and sent; null marks a gap in collection. */
  rx: (number | null)[];
  tx: (number | null)[];
}

interface ThroughputChartProps {
  label: string;
  series: ThroughputSeries;
  height?: number;
}

const rate = (v: number | null | undefined) => (v == null ? "–" : `${formatBytes(v)}/s`);
const peak = (values: (number | null)[]) => values.reduce<number>((m, v) => (v != null && v > m ? v : m), 0);

/** Round axis steps in the units the labels use: 1, 2, 5, 10, 20, 50 … B, KiB, MiB, GiB. */
const BYTE_STEPS = [1, 1024, 1024 ** 2, 1024 ** 3].flatMap((base) => [1, 2, 5, 10, 20, 50, 100, 200, 500].map((n) => n * base));

/** Last recorded point, so the caption never reports a gap as a current value. */
function lastRecorded(series: ThroughputSeries): number {
  for (let i = series.at.length - 1; i >= 0; i--) if (series.rx[i] != null) return i;
  return -1;
}

/** Runs of missing samples, as "13:05–13:15", for the caption and screen readers. */
export function gaps(series: ThroughputSeries): string[] {
  const out: string[] = [];
  let start: number | null = null;
  for (let i = 0; i < series.at.length; i++) {
    const at = series.at[i] ?? 0;
    const missing = series.rx[i] == null;
    if (missing && start === null) start = at;
    else if (!missing && start !== null) {
      out.push(`${formatTime(start)}–${formatTime(at)}`);
      start = null;
    }
  }
  if (start !== null) out.push(`since ${formatTime(start)}`);
  return out;
}

function gapSentence(list: string[]): string {
  if (list.length === 0) return "";
  const shown = list.slice(0, 3).join(", ");
  return `Not recorded ${shown}${list.length > 3 ? ` and ${String(list.length - 3)} more times` : ""}`;
}

/** Axis label for a tick: dates once ticks are a day or more apart, else the time. */
function tickLabel(at: number, step: number): string {
  if (step >= 86_400) return formatDate(at);
  const d = new Date(at * 1000);
  if (step >= 3_600 && d.getHours() === 0 && d.getMinutes() === 0) return formatDate(at);
  return formatClock(at, step < 60);
}

function canDraw(): boolean {
  return typeof ResizeObserver !== "undefined" && document.createElement("canvas").getContext("2d") !== null;
}

/**
 * Live throughput for one interface. New samples update the existing chart in
 * place; only a theme change rebuilds it. Received is solid ink, sent is dashed.
 */
export function ThroughputChart({ label, series, height = 220 }: ThroughputChartProps) {
  const host = useRef<HTMLDivElement>(null);
  const chart = useRef<uPlot | null>(null);
  const [themeVersion, setThemeVersion] = useState(0);
  const data: uPlot.AlignedData = [series.at, series.rx, series.tx];
  const latest = lastRecorded(series);
  const stale = latest >= 0 && latest < series.at.length - 1;
  const at = (i: number) => (stale && series.at[i] !== undefined ? ` at ${formatTime(series.at[i])}` : "");
  const missing = gapSentence(gaps(series));

  useEffect(() => subscribeTheme(() => setThemeVersion((v) => v + 1)), []);

  useEffect(() => {
    const element = host.current;
    if (!element || !canDraw()) return;
    // Colors come from the chart's own element so a locally themed container is honored.
    const color = (name: string) => token(name, element);
    const rule = color("rule");
    const axis = {
      stroke: color("ink-muted"),
      grid: { stroke: rule, width: 1 },
      ticks: { stroke: rule, width: 1 },
      font: '13px "Atkinson Hyperlegible Next Variable", sans-serif',
    };
    const u = new uPlot(
      {
        width: element.clientWidth || 600,
        height,
        legend: { show: false },
        cursor: { drag: { x: false, y: false } },
        scales: { x: { time: true } },
        series: [
          {},
          { label: "Received", stroke: color("ink"), width: 1.75, spanGaps: false, points: { show: false } },
          { label: "Sent", stroke: color("ink-muted"), width: 1.25, dash: [4, 3], spanGaps: false, points: { show: false } },
        ],
        axes: [
          {
            ...axis,
            space: 90,
            // Seconds only when ticks are under a minute apart; dates once they span days.
            values: (_u, splits) => {
              const step = (splits[1] ?? 0) - (splits[0] ?? 0);
              return splits.map((s) => tickLabel(s, step));
            },
          },
          // Wide enough for "1023.9 KiB/s" at the small size.
          { ...axis, size: 92, incrs: BYTE_STEPS, values: (_u, splits) => splits.map((s) => `${formatBytes(s)}/s`) },
        ],
      },
      data,
      element,
    );
    chart.current = u;
    const resize = new ResizeObserver(() => u.setSize({ width: element.clientWidth, height }));
    resize.observe(element);
    return () => {
      resize.disconnect();
      u.destroy();
      chart.current = null;
    };
    // Rebuild only for size or theme; data updates go through setData below.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [height, themeVersion]);

  useEffect(() => {
    chart.current?.setData(data);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [series]);

  return (
    <figure className="m-0 flex flex-col gap-2">
      <figcaption className="flex flex-wrap items-center gap-x-5 gap-y-1 text-small text-ink-muted">
        <span className="font-semibold text-ink">{label}</span>
        <span className="inline-flex items-center gap-1.5">
          <span aria-hidden className="h-0.5 w-4 bg-ink" />
          Received {rate(series.rx[latest])}
          {at(latest)}
        </span>
        <span className="inline-flex items-center gap-1.5">
          <span aria-hidden className="h-0 w-4 border-t border-dashed border-ink-muted" />
          Sent {rate(series.tx[latest])}
        </span>
        <span>
          Peak {rate(peak(series.rx))} in, {rate(peak(series.tx))} out
        </span>
        {missing && <span>{missing}</span>}
      </figcaption>
      <div ref={host} role="img" aria-label={`${label}: received ${rate(series.rx[latest])}${at(latest)}, peak ${rate(peak(series.rx))}; sent peak ${rate(peak(series.tx))}${missing ? `. ${missing}` : ""}`} style={{ minHeight: height }} />
    </figure>
  );
}
