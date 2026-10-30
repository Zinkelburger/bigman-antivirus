import { ChevronRight } from "lucide-react";
import type { ReactNode } from "react";
import type { Severity } from "@/ipc";
import { SeverityBadge } from "./SeverityBadge";

export type TileState = "loading" | "ready" | "empty" | "incomplete" | "disconnected";

interface StatusTileProps {
  label: string;
  state: TileState;
  /** The one figure or word this tile reports. */
  value?: ReactNode;
  /** A plain sentence under the value: what it means, or why it is missing.
   *  Disconnected and incomplete tiles show their own wording instead of `value`. */
  detail?: ReactNode;
  severity?: Severity | null | undefined;
  /** Opening a tile shows its table (level two of the three-level rule). */
  onOpen?: (() => void) | undefined;
}

/** Level one of the three-level rule: a count or word that opens to a table. */
export function StatusTile({ label, state, value, detail, severity, onOpen }: StatusTileProps) {
  const body = (
    <>
      <div className="flex items-center justify-between gap-2">
        <span className="truncate text-small font-semibold text-ink-muted" title={label}>
          {label}
        </span>
        {severity && state === "ready" && <SeverityBadge severity={severity} />}
      </div>
      {state === "loading" ? (
        <span aria-hidden className="mt-2 flex flex-col gap-2">
          <span className="h-7 w-16 rounded-control bg-sunken" />
          <span className="h-4 w-36 rounded-control bg-sunken" />
        </span>
      ) : (
        <>
          <span
            className={`mt-1 break-words text-title font-semibold ${state === "ready" ? "text-ink" : "text-ink-muted"}`}
          >
            {state === "ready" || state === "empty" ? value : state === "disconnected" ? "—" : "Incomplete"}
          </span>
          {detail && <span className="text-small text-ink-muted">{detail}</span>}
        </>
      )}
      {onOpen && state !== "loading" && (
        <ChevronRight aria-hidden size={16} className="absolute right-3 bottom-3 text-ink-muted" />
      )}
    </>
  );
  const className =
    "relative flex min-h-28 w-full min-w-0 flex-col gap-0.5 rounded-panel border border-rule bg-surface py-3.5 pr-7 pl-3.5 text-left";
  if (!onOpen || state === "loading") {
    return (
      <div className={`${className} ${state === "disconnected" ? "opacity-60" : ""}`} aria-busy={state === "loading"}>
        {body}
      </div>
    );
  }
  return (
    <button
      type="button"
      aria-haspopup="dialog"
      onClick={onOpen}
      className={`${className} cursor-pointer border-rule-strong transition-colors hover:bg-ground`}
    >
      {body}
    </button>
  );
}
