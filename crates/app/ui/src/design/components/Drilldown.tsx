import { ChevronLeft, X } from "lucide-react";
import { useEffect, useRef } from "react";
import { useDrilldown } from "../drilldown";

/**
 * The single drill-down implementation: a side panel with two depths. Every
 * feature opens it through `useDrilldown`; none builds its own.
 */
export function Drilldown() {
  const { stack, back, close } = useDrilldown();
  const heading = useRef<HTMLHeadingElement>(null);
  const panel = stack[stack.length - 1];
  const depth = stack.length;

  useEffect(() => {
    if (panel) heading.current?.focus();
  }, [panel]);

  if (!panel) return null;
  return (
    <aside
      role="dialog"
      aria-modal="false"
      aria-labelledby="drilldown-title"
      onKeyDown={(e) => {
        if (e.key === "Escape") {
          e.stopPropagation();
          if (depth > 1) back();
          else close();
        }
      }}
      className="absolute inset-y-0 right-0 z-30 flex w-[min(560px,62%)] flex-col border-l border-rule bg-surface shadow-panel animate-[bigman-panel-in_180ms_cubic-bezier(0.16,1,0.3,1)]"
    >
      <header className="flex h-12 shrink-0 items-center gap-2 border-b border-rule px-3">
        {depth > 1 && (
          <button
            type="button"
            onClick={back}
            className="inline-flex cursor-pointer items-center gap-1 rounded-control px-1.5 py-1 text-small font-semibold text-ink-muted hover:bg-sunken"
          >
            <ChevronLeft aria-hidden size={14} />
            {stack[0]?.title}
          </button>
        )}
        <h2 id="drilldown-title" ref={heading} tabIndex={-1} className="m-0 min-w-0 flex-1 truncate px-1 text-body font-semibold text-ink outline-none">
          {panel.title}
        </h2>
        <button
          type="button"
          onClick={close}
          aria-label="Close panel"
          className="cursor-pointer rounded-control p-1.5 text-ink-muted hover:bg-sunken"
        >
          <X aria-hidden size={16} />
        </button>
      </header>
      <div className="flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto p-4">{panel.content}</div>
    </aside>
  );
}
