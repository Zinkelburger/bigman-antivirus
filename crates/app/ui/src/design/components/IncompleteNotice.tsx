import { CircleSlash, Unplug } from "lucide-react";
import type { ReactNode } from "react";
import type { Coverage } from "@/ipc";
import { CoverageList } from "./CoverageList";

interface IncompleteNoticeProps {
  /** "incomplete" lists what was skipped; "disconnected" means collection is not running. */
  kind?: "incomplete" | "disconnected";
  title?: string;
  coverage?: Coverage | undefined;
  errors?: string[] | undefined;
  /** One action that resolves the gap, such as starting the daemon. */
  action?: ReactNode;
}

export function IncompleteNotice({
  kind = "incomplete",
  title,
  coverage,
  errors = [],
  action,
}: IncompleteNoticeProps) {
  const Icon = kind === "disconnected" ? Unplug : CircleSlash;
  const heading =
    title ?? (kind === "disconnected" ? "Background checks are not running" : "Incomplete");
  return (
    <div
      role="status"
      className="flex flex-col gap-3 rounded-panel border border-rule bg-surface p-4"
    >
      <div className="flex items-start justify-between gap-4">
        <h2 className="m-0 flex items-center gap-2 text-body font-semibold text-ink">
          <Icon aria-hidden size={16} className="text-ink-muted" />
          {heading}
        </h2>
        {action}
      </div>
      {coverage ? (
        <CoverageList coverage={coverage} errors={errors} />
      ) : (
        errors.map((e, i) => (
          <code key={i} className="break-words font-mono text-small text-ink-muted">
            {e}
          </code>
        ))
      )}
    </div>
  );
}
