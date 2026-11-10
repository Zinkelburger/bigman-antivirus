import { ArrowDown, ArrowUp } from "lucide-react";
import { useId, useState, type KeyboardEvent, type ReactNode } from "react";
import { formatBytes, formatCount } from "../format";
import { FirstSeenMarker } from "./FirstSeenMarker";
import { seriesColor } from "./StackedAreaChart";

export interface ShareRow {
  id: string;
  name: string;
  /** The full text when `name` is shortened, e.g. a long address. */
  title?: string | undefined;
  /** Machine text such as an address is set in mono. */
  mono?: boolean;
  /** A short second fact, e.g. the program behind an address. */
  detail?: string | undefined;
  /** Series hue 1 to 5 for the swatch and bar; null or absent draws them neutral. */
  slot?: number | null | undefined;
  /** Whether the row leads with a swatch (programs and addresses do; types do not). */
  swatch?: boolean;
  /** Seen on the network for the first time recently: carries the first-seen ring. */
  firstSeen?: boolean | undefined;
  received: number;
  sent: number;
}

interface ShareListProps {
  /** The column's heading, e.g. "Programs". */
  label: string;
  /** How many rows exist in all; the list shows the largest. */
  count: number;
  rows: readonly ShareRow[];
  onOpen: (row: ShareRow) => void;
  /** Usually the button that opens the full table. */
  footer?: ReactNode;
}

const percent = (part: number, whole: number) => `${String(whole > 0 ? Math.max(part > 0 ? 1.5 : 0, (part / whole) * 100) : 0)}%`;

/** The key to a ShareList bar: what the solid and the outlined part mean. */
export function ShareKey() {
  return (
    <span className="inline-flex flex-wrap items-center gap-x-4 gap-y-1 text-small text-ink-muted">
      <span className="inline-flex items-center gap-1.5">
        <span aria-hidden className="h-1.5 w-5 rounded-full bg-ink-muted" />
        Received
      </span>
      <span className="inline-flex items-center gap-1.5">
        <span aria-hidden className="h-1.5 w-5 rounded-full border border-ink-muted" />
        Sent
      </span>
    </span>
  );
}

/**
 * The largest members of one breakdown. Each row gives its total, what it
 * received and sent as figures, and a bar scaled to the largest row: received
 * solid, sent outlined. The list is one Tab stop; Up, Down, Home and End move
 * within it and Enter opens the row to its evidence.
 */
export function ShareList({ label, count, rows, onOpen, footer }: ShareListProps) {
  const heading = useId();
  const [active, setActive] = useState(0);
  const current = Math.min(active, Math.max(0, rows.length - 1));
  const max = rows.reduce((m, r) => Math.max(m, r.received + r.sent), 0);
  const onKey = (event: KeyboardEvent<HTMLUListElement>) => {
    const to = { ArrowDown: current + 1, ArrowUp: current - 1, Home: 0, End: rows.length - 1 }[event.key];
    if (to === undefined || rows.length === 0) return;
    event.preventDefault();
    const next = Math.min(rows.length - 1, Math.max(0, to));
    setActive(next);
    event.currentTarget.querySelectorAll<HTMLButtonElement>("button")[next]?.focus();
  };
  return (
    <section aria-labelledby={heading} className="@container flex min-w-0 flex-col gap-1">
      <h3 id={heading} className="m-0 flex items-baseline gap-2 px-2 text-small font-semibold text-ink-muted">
        {label}
        <span className="font-normal">{formatCount(count)}</span>
      </h3>
      {/* A list given the whole width (the third of three in a narrow window) runs in two columns. */}
      <ul className="m-0 grid list-none grid-cols-1 gap-x-4 p-0 @[38rem]:grid-cols-2" onKeyDown={onKey}>
        {rows.map((row, index) => {
          const color = seriesColor(row.slot ?? null);
          return (
            <li key={row.id} className="min-w-0">
              <button
                type="button"
                tabIndex={index === current ? 0 : -1}
                onFocus={() => setActive(index)}
                onClick={() => onOpen(row)}
                aria-label={`${row.title ?? row.name}${row.firstSeen ? ", first seen in the last day" : ""}${row.detail ? `, ${row.detail}` : ""}: ${formatBytes(row.received)} received, ${formatBytes(row.sent)} sent`}
                className="flex w-full cursor-pointer flex-col gap-1 rounded-control border-0 bg-transparent px-2 py-1.5 text-left text-body text-ink transition-colors duration-150 hover:bg-sunken"
              >
                <span className="flex h-[1.375rem] w-full items-center gap-2">
                  {row.swatch && <span aria-hidden className="size-2.5 shrink-0 rounded-control" style={{ background: color }} />}
                  <span title={row.title ?? row.name} className={`min-w-0 truncate ${row.mono ? "font-mono text-small" : ""}`}>
                    {row.name}
                  </span>
                  {row.firstSeen && <FirstSeenMarker compact />}
                  <span className="ml-auto shrink-0 pl-2">{formatBytes(row.received + row.sent)}</span>
                </span>
                <span className="flex w-full items-center gap-3 text-small text-ink-muted">
                  <span className="inline-flex shrink-0 items-center gap-0.5">
                    <ArrowDown aria-hidden size={13} />
                    {formatBytes(row.received)}
                  </span>
                  <span className="inline-flex shrink-0 items-center gap-0.5">
                    <ArrowUp aria-hidden size={13} />
                    {formatBytes(row.sent)}
                  </span>
                  {row.detail && (
                    <span title={row.detail} className="ml-auto min-w-0 truncate">
                      {row.detail}
                    </span>
                  )}
                </span>
                <span aria-hidden className="mt-0.5 flex h-1.5 w-full gap-0.5">
                  <span className="h-full rounded-full" style={{ width: percent(row.received, max), background: color }} />
                  <span className="h-full rounded-full border" style={{ width: percent(row.sent, max), borderColor: color }} />
                </span>
              </button>
            </li>
          );
        })}
      </ul>
      {footer}
    </section>
  );
}
