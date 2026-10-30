import type { LucideIcon } from "lucide-react";
import type { ReactNode } from "react";
import { useDrilldown } from "../drilldown";
import { Drilldown } from "./Drilldown";

export interface ShellTab<T extends string> {
  id: T;
  label: string;
  icon: LucideIcon;
  /** Unread count shown beside the tab, such as new first-seen entries. */
  badge?: number | undefined;
}

interface AppShellProps<T extends string> {
  tabs: readonly ShellTab<T>[];
  active: T;
  onSelect: (id: T) => void;
  statusStrip: ReactNode;
  children: ReactNode;
}

/** The B mark, drawn from the logo as a quiet monochrome glyph. */
export function BigmanMark({ size = 22 }: { size?: number }) {
  return (
    <svg aria-hidden width={size} height={size} viewBox="0 0 24 24" fill="none">
      <path
        d="M5 3.5h7.6c3.2 0 5.3 1.8 5.3 4.5 0 1.9-1 3.2-2.6 3.8 2.2.5 3.7 2.2 3.7 4.5 0 3.2-2.4 5.2-6 5.2H5Z"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinejoin="round"
      />
      <path d="M9 7.5v4m0 0v6.5m0-6.5h3.4" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    </svg>
  );
}

/** Sidebar, content and status strip. The drill-down panel opens over the content. */
export function AppShell<T extends string>({ tabs, active, onSelect, statusStrip, children }: AppShellProps<T>) {
  const panelOpen = useDrilldown((s) => s.stack.length > 0);
  const closePanel = useDrilldown((s) => s.close);
  return (
    <div className="grid h-screen min-h-[600px] min-w-[960px] grid-cols-[12rem_minmax(0,1fr)] bg-ground">
      <a
        href="#content"
        className="sr-only z-50 rounded-control bg-surface px-3 py-2 font-semibold text-ink focus:not-sr-only focus:fixed focus:top-2 focus:left-2"
      >
        Skip to content
      </a>
      <nav aria-label="Sections" className="flex flex-col gap-1 bg-rail px-3 py-4 text-rail-ink">
        <div className="mb-4 flex items-center gap-2 px-2">
          <BigmanMark />
          <span className="text-title font-semibold">Bigman</span>
        </div>
        <ul className="m-0 flex list-none flex-col gap-0.5 p-0">
          {tabs.map((tab, i) => {
            const Icon = tab.icon;
            const current = tab.id === active;
            return (
              <li key={tab.id}>
                <button
                  type="button"
                  aria-current={current ? "page" : undefined}
                  onClick={() => onSelect(tab.id)}
                  className={`flex h-9 w-full cursor-pointer items-center gap-2.5 rounded-control px-2 text-left text-body transition-colors duration-150 ${current ? "bg-rail-active font-semibold text-rail-ink" : "text-rail-ink-muted hover:bg-rail-active hover:text-rail-ink"}`}
                >
                  <Icon aria-hidden size={16} strokeWidth={current ? 2.25 : 1.75} />
                  <span className="flex-1">{tab.label}</span>
                  {tab.badge ? (
                    <span className="rounded-full bg-accent px-1.5 text-small font-semibold text-accent-ink">
                      {tab.badge}
                      <span className="sr-only"> new</span>
                    </span>
                  ) : null}
                  <kbd aria-hidden className="font-sans text-small text-rail-ink-muted">
                    {i + 1}
                  </kbd>
                </button>
              </li>
            );
          })}
        </ul>
      </nav>
      <div className="grid min-h-0 grid-rows-[minmax(0,1fr)_auto]">
        <div className="relative min-h-0">
          {/* While the panel is open the page behind it is inert; clicking it closes the panel. */}
          <div className="h-full" onClick={panelOpen ? closePanel : undefined}>
            <main id="content" tabIndex={-1} inert={panelOpen} className="h-full overflow-y-auto outline-none">
              {children}
            </main>
          </div>
          <Drilldown />
        </div>
        <footer
          role="status"
          aria-live="polite"
          className="flex h-8 items-center gap-5 border-t border-rule bg-surface px-6 text-small text-ink-muted"
        >
          {statusStrip}
        </footer>
      </div>
    </div>
  );
}
