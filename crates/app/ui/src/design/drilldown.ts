import type { ReactNode } from "react";
import { create } from "zustand";

/** One level of the three-level rule: a table (depth 1) or its evidence (depth 2). */
export interface Panel {
  title: string;
  content: ReactNode;
}

interface DrilldownStore {
  stack: Panel[];
  /** Element that opened the panel, refocused on close. */
  opener: HTMLElement | null;
  /** Opens a table panel, replacing whatever was open. */
  open: (panel: Panel) => void;
  /** Opens evidence beside the current table. There are never more than two levels. */
  push: (panel: Panel) => void;
  back: () => void;
  close: () => void;
}

export const useDrilldown = create<DrilldownStore>((set, get) => ({
  stack: [],
  opener: null,
  open: (panel) =>
    set({
      stack: [panel],
      opener: document.activeElement instanceof HTMLElement ? document.activeElement : null,
    }),
  push: (panel) =>
    set((s) => (s.stack.length === 0 ? { stack: [panel] } : { stack: [s.stack[0] as Panel, panel] })),
  back: () => set((s) => ({ stack: s.stack.slice(0, 1) })),
  close: () => {
    const { opener } = get();
    set({ stack: [], opener: null });
    // The page behind the panel is inert until the next render; focus it after that.
    requestAnimationFrame(() => opener?.focus());
  },
}));
