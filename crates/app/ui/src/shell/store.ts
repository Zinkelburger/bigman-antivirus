import { create } from "zustand";
import { useDrilldown } from "@/design";

export const TAB_IDS = ["overview", "network", "activity", "inspect", "sandboxes", "protect", "setup"] as const;
export type TabId = (typeof TAB_IDS)[number];

export type Range = "5m" | "1h" | "24h" | "7d";

/** UI-only state. Data lives in TanStack Query; nothing here is persisted by the daemon. */
interface ShellStore {
  tab: TabId;
  range: Range;
  setTab: (tab: TabId) => void;
  setRange: (range: Range) => void;
}

export const useShell = create<ShellStore>((set) => ({
  tab: "overview",
  range: "24h",
  setTab: (tab) => {
    // A drill-down belongs to the tab that opened it.
    useDrilldown.getState().close();
    set({ tab });
  },
  setRange: (range) => set({ range }),
}));
