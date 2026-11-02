// Desktop-shell services that are not daemon requests: the file chooser and
// saving an exported report. Browser dev mode supplies stand-ins.
export interface DesktopShell {
  choosePath(kind: "file" | "directory", title: string): Promise<string | null>;
  /** Saves the report's full JSON where the user chooses; resolves to that path, or null if cancelled. */
  exportReport(id: string): Promise<string | null>;
}

let current: DesktopShell | null = null;

export function setDesktopShell(shell: DesktopShell): void {
  current = shell;
}

export function desktopShell(): DesktopShell {
  if (!current) throw new Error("No desktop shell configured; call setDesktopShell() at startup");
  return current;
}
