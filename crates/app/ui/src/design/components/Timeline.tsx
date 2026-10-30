import { useVirtualizer } from "@tanstack/react-virtual";
import { useId, useMemo, useRef, useState, type KeyboardEvent, type ReactNode } from "react";
import { formatDate, formatTime, plural } from "../format";
import { layout, type TimelineEntry } from "../timeline";
import { FirstSeenMarker } from "./FirstSeenMarker";
import { SeverityBadge } from "./SeverityBadge";
import { Tally } from "./Tally";

interface TimelineProps {
  label: string;
  entries: readonly TimelineEntry[];
  now: number;
  onOpen?: ((entry: TimelineEntry) => void) | undefined;
  /** Shown inside the list frame when there are no entries: what would appear, and one action. */
  empty?: ReactNode;
  /** Visible height; the list is virtualized beyond it. */
  height?: number;
}

// Estimates only: rows measure themselves, so wrapped sentences get their full height.
const DAY_HEIGHT = 36;
const ENTRY_HEIGHT = 40;
const FOLDED_HEIGHT = 56;

/** The life list: dated groups, first sightings on top, repeats folded with a tally. */
export function Timeline({ label, entries, now, onOpen, empty, height = 420 }: TimelineProps) {
  const rows = useMemo(() => layout(entries, now), [entries, now]);
  const entryIndexes = useMemo(() => rows.flatMap((r, i) => (r.kind === "entry" ? [i] : [])), [rows]);
  const [active, setActive] = useState(0);
  const [focused, setFocused] = useState(false);
  const scroller = useRef<HTMLDivElement>(null);
  const base = useId();
  // TanStack Virtual returns functions that the React Compiler cannot memoize.
  // eslint-disable-next-line react-hooks/incompatible-library
  const virtual = useVirtualizer({
    count: rows.length,
    getScrollElement: () => scroller.current,
    estimateSize: (i) => {
      const row = rows[i];
      if (row?.kind === "day") return DAY_HEIGHT;
      return row && row.entry.count > 1 ? FOLDED_HEIGHT : ENTRY_HEIGHT;
    },
    overscan: 16,
    initialRect: { width: 800, height },
  });
  const current = Math.min(active, Math.max(0, entryIndexes.length - 1));
  const optionId = (rowIndex: number) => `${base}-${rowIndex}`;

  if (entries.length === 0) {
    return (
      <div className="flex flex-col rounded-panel border border-rule bg-surface">
        <div className="flex flex-col items-start gap-3 px-4 py-5 text-body text-ink-muted">{empty}</div>
      </div>
    );
  }

  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    const move = (to: number) => {
      event.preventDefault();
      const next = Math.max(0, Math.min(entryIndexes.length - 1, to));
      setActive(next);
      virtual.scrollToIndex(entryIndexes[next] ?? 0, { align: "auto" });
    };
    if (event.key === "ArrowDown") move(current + 1);
    else if (event.key === "ArrowUp") move(current - 1);
    else if (event.key === "Home") move(0);
    else if (event.key === "End") move(entryIndexes.length - 1);
    else if (event.key === "Enter" && onOpen) {
      const row = rows[entryIndexes[current] ?? -1];
      if (row?.kind === "entry") {
        event.preventDefault();
        onOpen(row.entry);
      }
    }
  };

  return (
    <div ref={scroller} className="overflow-auto rounded-panel border border-rule bg-surface" style={{ maxHeight: height }}>
      <div
        role="listbox"
        aria-label={label}
        tabIndex={0}
        aria-activedescendant={entryIndexes.length ? optionId(entryIndexes[current] ?? 0) : undefined}
        onKeyDown={onKeyDown}
        onFocus={(e) => setFocused(e.target === e.currentTarget)}
        onBlur={() => setFocused(false)}
        className="relative focus-visible:outline-offset-[-2px]"
        style={{ height: virtual.getTotalSize() }}
      >
        {virtual.getVirtualItems().map((item) => {
          const row = rows[item.index];
          if (!row) return null;
          const style = { transform: `translateY(${item.start}px)` };
          if (row.kind === "day") {
            return (
              <div
                key={row.key}
                ref={virtual.measureElement}
                data-index={item.index}
                role="presentation"
                className="absolute inset-x-0 flex h-9 items-end border-b border-rule-strong bg-surface px-4 pb-1.5 text-small font-semibold text-ink"
                style={style}
              >
                {row.label}
              </div>
            );
          }
          const { entry } = row;
          const Icon = entry.icon;
          // The keyboard cursor shows only while the list has focus.
          const selected = focused && entryIndexes[current] === item.index;
          return (
            <div
              key={row.key}
              ref={virtual.measureElement}
              data-index={item.index}
              id={optionId(item.index)}
              role="option"
              aria-selected={selected}
              onClick={() => {
                setActive(entryIndexes.indexOf(item.index));
                onOpen?.(entry);
              }}
              className={`absolute inset-x-0 grid min-h-10 grid-cols-[3rem_1rem_1rem_minmax(0,1fr)_auto] items-start gap-x-2.5 border-b border-rule px-4 py-2 animate-[bigman-row-in_200ms_ease-out] ${onOpen ? "cursor-pointer" : ""} ${selected ? "bg-sunken" : "hover:bg-ground"}`}
              style={style}
            >
              <span className="sr-only">{row.day}, </span>
              <time dateTime={new Date(entry.at * 1000).toISOString()} className="pt-0.5 text-right text-small text-ink-muted">
                {formatTime(entry.at)}
              </time>
              <span className="flex justify-center pt-1">{entry.firstSeen && <FirstSeenMarker compact />}</span>
              <span className="flex justify-center pt-0.5 text-ink-muted">
                {Icon && <Icon aria-hidden size={15} />}
                <span className="sr-only">{entry.source}: </span>
              </span>
              <span className="flex min-w-0 flex-col">
                <span className="line-clamp-2 break-words text-ink" title={entry.summary}>
                  {entry.summary}
                </span>
                {entry.count > 1 && (
                  <span className="flex items-center gap-2 text-small text-ink-muted">
                    <Tally count={entry.count} />
                    seen {plural(entry.count, "time")}, first on {formatDate(entry.firstAt)}
                  </span>
                )}
              </span>
              <span className="flex w-20 justify-end">
                {entry.severity !== "info" && <SeverityBadge severity={entry.severity} />}
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );
}
