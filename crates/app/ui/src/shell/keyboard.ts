import { useEffect } from "react";
import { useDrilldown } from "@/design";
import { TAB_IDS, useShell } from "./store";

function typing(target: EventTarget | null): boolean {
  return (
    target instanceof HTMLElement &&
    (target.isContentEditable || ["INPUT", "TEXTAREA", "SELECT"].includes(target.tagName))
  );
}

/** Digits 1–7 switch tabs, `/` focuses search, Esc closes the drill-down. */
export function useGlobalKeys(): void {
  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (event.altKey || event.ctrlKey || event.metaKey || event.defaultPrevented) return;
      if (event.key === "Escape") {
        if (useDrilldown.getState().stack.length) useDrilldown.getState().close();
        return;
      }
      if (typing(event.target) || document.querySelector("[role=dialog][aria-modal=true]")) return;
      const index = Number(event.key) - 1;
      const tab = TAB_IDS[index];
      if (tab) {
        event.preventDefault();
        useShell.getState().setTab(tab);
        // Move focus to the new page's heading so screen readers announce it.
        requestAnimationFrame(() => document.querySelector<HTMLElement>("main h1")?.focus());
      } else if (event.key === "/") {
        // An open drill-down's own search comes first: it is what is in front.
        const field =
          document.querySelector<HTMLInputElement>("aside[role=dialog] [data-search-field]") ??
          document.querySelector<HTMLInputElement>("main [data-search-field]");
        if (field) {
          event.preventDefault();
          field.focus();
        }
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);
}
