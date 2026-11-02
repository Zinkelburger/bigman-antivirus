import { ChevronRight, File } from "lucide-react";
import type { Severity } from "@/ipc";
import { formatCount } from "../format";
import { SeverityBadge } from "./SeverityBadge";

export interface FileTreeEntry {
  path: string;
  change: "added" | "changed" | "removed";
  detail?: string | undefined;
  severity?: Severity | undefined;
}

export interface FileTreeGroup {
  category: string;
  files: FileTreeEntry[];
}

const CHANGE = { added: "Added", changed: "Changed", removed: "Removed" } as const;

/** Changed files grouped by category (/etc, systemd, cron, PAM, libraries, binaries, other). */
export function FileTree({ groups, onOpen }: { groups: FileTreeGroup[]; onOpen?: (file: FileTreeEntry) => void }) {
  return (
    <div className="flex flex-col rounded-panel border border-rule bg-surface">
      {groups.map((group) => (
        <details key={group.category} open={group.files.some((f) => f.severity && f.severity !== "info")} className="group border-b border-rule last:border-b-0">
          <summary className="flex cursor-pointer list-none items-center gap-2 px-3 py-2 hover:bg-ground [&::-webkit-details-marker]:hidden">
            <ChevronRight aria-hidden size={14} className="text-ink-muted transition-transform group-open:rotate-90" />
            <span className="font-semibold text-ink">{group.category}</span>
            <span className="ml-auto text-small text-ink-muted">{formatCount(group.files.length)}</span>
          </summary>
          <ul className="pb-1">
            {group.files.map((file) => (
              <li key={file.path}>
                {(() => {
                  const row = (
                    <>
                      <File aria-hidden size={13} className="text-ink-muted" />
                      <span className="truncate font-mono text-small text-ink" title={file.path}>{file.path}</span>
                      <span className="text-small text-ink-muted">
                        {CHANGE[file.change]}
                        {file.detail ? ` · ${file.detail}` : ""}
                      </span>
                      {file.severity && file.severity !== "info" ? <SeverityBadge severity={file.severity} /> : <span />}
                    </>
                  );
                  const layout = "grid w-full grid-cols-[auto_minmax(0,1fr)_auto_auto] items-center gap-2 py-1 pr-3 pl-9 text-left";
                  // Rows that open are buttons; the rest are plain rows a screen reader does not skip as dimmed.
                  return onOpen ? (
                    <button type="button" onClick={() => onOpen(file)} className={`${layout} cursor-pointer hover:bg-ground`}>
                      {row}
                    </button>
                  ) : (
                    <div className={layout}>{row}</div>
                  );
                })()}
              </li>
            ))}
          </ul>
        </details>
      ))}
    </div>
  );
}
