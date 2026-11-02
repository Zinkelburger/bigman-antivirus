import { useShell, type ProtectView } from "@/shell/store";

export type { ProtectView };

/** The open sub-view, kept in the shell store so other tabs can open one. */
export function useProtectView() {
  const view = useShell((s) => s.protectView);
  const setView = useShell((s) => s.setProtectView);
  return { view, setView };
}
