// Axis arithmetic the charts share: round steps in the units the labels use.
import { formatClock, formatDate } from "../format";

/** Round axis steps in the units the labels use: 1, 2, 5, 10, 20, 50 … B, KiB, MiB, GiB. */
export const BYTE_STEPS = [1, 1024, 1024 ** 2, 1024 ** 3].flatMap((base) => [1, 2, 5, 10, 20, 50, 100, 200, 500].map((n) => n * base));

/** Axis label for a tick: dates once ticks are a day or more apart, else the time. */
export function tickLabel(at: number, step: number): string {
  if (step >= 86_400) return formatDate(at);
  const d = new Date(at * 1000);
  if (step >= 3_600 && d.getHours() === 0 && d.getMinutes() === 0) return formatDate(at);
  return formatClock(at, step < 60);
}

/** Ticks from zero in the round byte step that gives at most `most` of them above zero. */
export function byteTicks(max: number, most = 5): number[] {
  const step = BYTE_STEPS.find((s) => max / s <= most) ?? BYTE_STEPS[BYTE_STEPS.length - 1] ?? 1;
  const ticks = [0];
  for (let v = step; v <= max; v += step) ticks.push(v);
  return ticks;
}

const TIME_STEPS = [5, 15, 30, 60, 120, 300, 600, 900, 1_800, 3_600, 7_200, 10_800, 21_600, 43_200, 86_400, 2 * 86_400];

/** Round clock times in `[from, to]`, at most `most` of them, and the step between them. */
export function timeTicks(from: number, to: number, most: number): { step: number; ticks: number[] } {
  const span = Math.max(1, to - from);
  const step = TIME_STEPS.find((s) => span / s <= Math.max(1, most)) ?? TIME_STEPS[TIME_STEPS.length - 1] ?? 86_400;
  const ticks: number[] = [];
  if (step >= 86_400) {
    // Days break at local midnight, which is not a multiple of a day in Unix time.
    const day = new Date(from * 1000);
    day.setHours(0, 0, 0, 0);
    for (; day.getTime() / 1000 <= to; day.setDate(day.getDate() + step / 86_400)) {
      if (day.getTime() / 1000 >= from) ticks.push(day.getTime() / 1000);
    }
    return { step, ticks };
  }
  // Align to local clock time, so zones offset by half an hour still tick on the hour.
  const offset = new Date(from * 1000).getTimezoneOffset() * 60;
  for (let t = Math.ceil((from - offset) / step) * step + offset; t <= to; t += step) ticks.push(t);
  return { step, ticks };
}

const UNITS = ["B", "KiB", "MiB", "GiB"] as const;

/** A rate tick in the unit of the axis step, so one axis never mixes "500 KiB" with "0.98 MiB". */
export function rateTickLabel(value: number, step: number): string {
  const unit = Math.min(UNITS.length - 1, Math.max(0, Math.floor(Math.log(Math.max(1, step)) / Math.log(1024))));
  return `${String(Math.round(value / 1024 ** unit))} ${UNITS[unit] ?? "B"}/s`;
}
