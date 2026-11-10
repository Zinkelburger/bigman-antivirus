import { create } from "zustand";
import { useDrilldown } from "@/design";

export type ProtectView = "hardening" | "scans" | "updates" | "quarantine" | "startup" | "skills" | "canaries" | "audits";

export const TAB_IDS = ["overview", "network", "activity", "inspect", "protect", "setup"] as const;
export type TabId = (typeof TAB_IDS)[number];

export type Range = "5m" | "1h" | "24h" | "7d";

/** UI-only state. Data lives in TanStack Query; nothing here is persisted by the daemon. */
interface ShellStore {
  tab: TabId;
  range: Range;
  /** Activity opens filtered to entries since the last look (from Overview's tile). */
  unreadOnly: boolean;
  /** The open Protect sub-view; it survives switching tabs. */
  protectView: ProtectView;
  setProtectView: (view: ProtectView) => void;
  /** Overview's tiles open a Protect sub-view directly. */
  openProtect: (view: ProtectView) => void;
  setTab: (tab: TabId) => void;
  openUnread: () => void;
  setUnreadOnly: (on: boolean) => void;
  setRange: (range: Range) => void;
}

export const useShell = create<ShellStore>((set) => ({
  tab: "overview",
  range: "24h",
  unreadOnly: false,
  protectView: "hardening",
  setProtectView: (protectView) => set({ protectView }),
  openProtect: (protectView) => {
    useDrilldown.getState().close();
    set({ tab: "protect", protectView });
  },
  openUnread: () => {
    useDrilldown.getState().close();
    set({ tab: "activity", unreadOnly: true });
  },
  setUnreadOnly: (unreadOnly) => set({ unreadOnly }),
  setTab: (tab) => {
    // A drill-down belongs to the tab that opened it.
    useDrilldown.getState().close();
    set({ tab });
  },
  setRange: (range) => set({ range }),
}));
