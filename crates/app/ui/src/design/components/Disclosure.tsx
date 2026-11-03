import { ChevronRight } from "lucide-react";
import type { ReactNode } from "react";

/**
 * A section that opens on demand, with the same chevron as FileTree instead of
 * the browser's triangle. `panel` draws it as a bordered panel.
 */
export function Disclosure({ summary, children, panel = true }: { summary: ReactNode; children: ReactNode; panel?: boolean }) {
  return (
    <details className={`group ${panel ? "rounded-panel border border-rule bg-surface" : ""}`}>
      <summary
        className={`flex cursor-pointer list-none items-center gap-2 rounded-panel [&::-webkit-details-marker]:hidden ${panel ? "px-4 py-3 font-semibold text-ink hover:bg-ground" : "text-ink"}`}
      >
        <ChevronRight aria-hidden size={14} className="shrink-0 text-ink-muted transition-transform group-open:rotate-90 motion-reduce:transition-none" />
        {summary}
      </summary>
      <div className={panel ? "px-4 pt-1 pb-4" : "mt-2"}>{children}</div>
    </details>
  );
}
