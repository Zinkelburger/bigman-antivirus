import { useId, useRef, type KeyboardEvent } from "react";

interface SegmentedTabsProps<T extends string> {
  label: string;
  tabs: readonly { id: T; label: string; count?: number | undefined }[];
  active: T;
  onSelect: (id: T) => void;
}

/**
 * Sub-views inside one tab (Protect, Inspect). WAI-ARIA tabs: arrow keys move
 * between them, the panel is labelled by the selected tab.
 */
export function SegmentedTabs<T extends string>({ label, tabs, active, onSelect }: SegmentedTabsProps<T>) {
  const base = useId();
  const refs = useRef<(HTMLButtonElement | null)[]>([]);
  const onKeyDown = (event: KeyboardEvent, index: number) => {
    const step = event.key === "ArrowRight" ? 1 : event.key === "ArrowLeft" ? -1 : 0;
    if (!step) return;
    event.preventDefault();
    const next = (index + step + tabs.length) % tabs.length;
    const tab = tabs[next];
    if (tab) {
      onSelect(tab.id);
      refs.current[next]?.focus();
    }
  };
  return (
    <div role="tablist" aria-label={label} className="flex gap-1 border-b border-rule px-6">
      {tabs.map((tab, i) => {
        const selected = tab.id === active;
        return (
          <button
            key={tab.id}
            ref={(el) => {
              refs.current[i] = el;
            }}
            id={`${base}-${tab.id}`}
            type="button"
            role="tab"
            aria-selected={selected}
            tabIndex={selected ? 0 : -1}
            onClick={() => onSelect(tab.id)}
            onKeyDown={(e) => onKeyDown(e, i)}
            className={`-mb-px inline-flex h-10 cursor-pointer items-center gap-1.5 border-b-2 px-3 text-body transition-colors duration-150 ${selected ? "border-ink font-semibold text-ink" : "border-transparent text-ink-muted hover:text-ink"}`}
          >
            {tab.label}
            {tab.count ? <span className="text-small text-ink-muted">{tab.count}</span> : null}
          </button>
        );
      })}
    </div>
  );
}
