// Number and time formatting shared by every view. Figures are always tabular
// (set globally); these functions decide units and wording.

const UNITS = ["B", "KiB", "MiB", "GiB", "TiB"] as const;

/** Bytes in the unit that keeps the number under four digits. */
export function formatBytes(bytes: number): string {
  let value = bytes;
  let unit = 0;
  while (value >= 1000 && unit < UNITS.length - 1) {
    value /= 1024;
    unit += 1;
  }
  const digits = unit === 0 || value >= 100 ? 0 : value >= 10 ? 1 : 2;
  return `${value.toFixed(digits)} ${UNITS[unit] ?? "B"}`;
}

export function formatCount(n: number): string {
  return new Intl.NumberFormat("en").format(n);
}

/** "3 min", "5 h", "2 days": how long ago, for status lines and tiles. */
export function formatAge(seconds: number, now: number): string {
  const age = Math.max(0, now - seconds);
  if (age < 60) return "under a minute";
  if (age < 3_600) return plural(Math.floor(age / 60), "minute");
  if (age < 86_400) return plural(Math.floor(age / 3_600), "hour");
  return plural(Math.floor(age / 86_400), "day");
}

/** "just now", "5 minutes ago": for lists and tiles. */
export function ago(seconds: number, now: number): string {
  return now - seconds < 60 ? "just now" : `${formatAge(seconds, now)} ago`;
}

export function plural(n: number, word: string): string {
  if (n === 1) return `${formatCount(n)} ${word}`;
  // "entry" → "entries", "day" stays "days".
  return `${formatCount(n)} ${word.replace(/([^aeiou])y$/, "$1ie")}s`;
}

// Fixed locale until the app is translated: 24-hour times, "Oct 3" dates.
const time = new Intl.DateTimeFormat("en-US", { hour: "2-digit", minute: "2-digit", hourCycle: "h23" });
const day = new Intl.DateTimeFormat("en-US", { weekday: "short", month: "short", day: "numeric" });
const date = new Intl.DateTimeFormat("en-US", { month: "short", day: "numeric" });

/** 14:02 */
export function formatTime(seconds: number): string {
  return time.format(seconds * 1000);
}

const clockSeconds = new Intl.DateTimeFormat("en-US", {
  hour: "2-digit",
  minute: "2-digit",
  second: "2-digit",
  hourCycle: "h23",
});

/** 14:02, or 14:02:31 for chart axes at second resolution. */
export function formatClock(seconds: number, withSeconds: boolean): string {
  return (withSeconds ? clockSeconds : time).format(seconds * 1000);
}

/** Oct 3 style short date, used in "first on Oct 3". */
export function formatDate(seconds: number): string {
  return date.format(seconds * 1000);
}

/** Day group heading: "Today", "Yesterday" or "Tue, Oct 27". */
export function formatDay(seconds: number, now: number): string {
  const start = (s: number) => {
    const d = new Date(s * 1000);
    d.setHours(0, 0, 0, 0);
    return d.getTime();
  };
  const diff = Math.round((start(now) - start(seconds)) / 86_400_000);
  if (diff === 0) return "Today";
  if (diff === 1) return "Yesterday";
  return day.format(seconds * 1000);
}

/** "today", "yesterday" or "5 days old" for signature freshness wording. */
export function signatureAge(updated: number, now: number): string {
  const days = Math.floor((now - updated) / 86_400);
  if (days <= 0) return "from today";
  if (days === 1) return "from yesterday";
  return `${plural(days, "day")} old`;
}

export function nowSeconds(): number {
  return Math.floor(Date.now() / 1000);
}
