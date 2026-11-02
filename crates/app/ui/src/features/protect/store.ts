import { create } from "zustand";

export type ProtectView = "scans" | "quarantine" | "startup" | "canaries" | "audits";

/** The open sub-view survives switching tabs. */
export const useProtectView = create<{ view: ProtectView; setView: (view: ProtectView) => void }>((set) => ({
  view: "scans",
  setView: (view) => set({ view }),
}));
