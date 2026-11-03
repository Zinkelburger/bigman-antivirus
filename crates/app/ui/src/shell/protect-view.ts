import { create } from "zustand";

export type ProtectView = "hardening" | "scans" | "quarantine" | "startup" | "canaries" | "audits";

/** The open sub-view survives switching tabs. */
export const useProtectView = create<{ view: ProtectView; setView: (view: ProtectView) => void }>((set) => ({
  view: "hardening",
  setView: (view) => set({ view }),
}));
