import { CircleSlash } from "lucide-react";
import type { Coverage, SkipReason } from "@/ipc";

export const SKIP_REASON: Record<SkipReason, string> = {
  tool_missing: "Tool not installed",
  permission_denied: "No permission to read",
  needs_helper: "Needs the optional helper",
  limit_reached: "Size or count limit reached",
  cancelled: "Cancelled",
  unreadable: "Could not be read",
  failed: "Tool did not finish",
  not_applicable: "Not applicable here",
  daemon_unavailable: "Background collection is not running",
};

/** What a check examined and what it could not, from the Coverage contract type. */
export function CoverageList({ coverage, errors = [] }: { coverage: Coverage; errors?: string[] }) {
  return (
    <div className="flex flex-col gap-3 text-small">
      {(coverage.skipped.length > 0 || errors.length > 0) && (
        <section aria-label="Not checked">
          <h3 className="mb-1 text-small font-semibold text-ink">Not checked</h3>
          <ul className="flex flex-col gap-1.5">
            {coverage.skipped.map((s, i) => (
              <li key={`${s.what}-${i}`} className="grid grid-cols-[minmax(0,1fr)_auto] gap-x-3">
                <span className="break-words text-ink">{s.what}</span>
                <span className="inline-flex items-center gap-1 text-ink-muted">
                  <CircleSlash aria-hidden size={12} />
                  {SKIP_REASON[s.why]}
                </span>
                {s.detail && (
                  <code className="col-span-2 break-words font-mono text-ink-muted">{s.detail}</code>
                )}
              </li>
            ))}
            {errors.map((e, i) => (
              <li key={`error-${i}`}>
                <code className="break-words font-mono text-ink-muted">{e}</code>
              </li>
            ))}
          </ul>
        </section>
      )}
      {coverage.checked.length > 0 && (
        <section aria-label="Checked">
          <h3 className="mb-1 text-small font-semibold text-ink">Checked</h3>
          <ul className="flex flex-wrap gap-x-4 gap-y-1 text-ink-muted">
            {coverage.checked.map((c) => (
              <li key={c}>{c}</li>
            ))}
          </ul>
        </section>
      )}
      {coverage.notes.length > 0 && (
        <section aria-label="Scope">
          <h3 className="mb-1 text-small font-semibold text-ink">What these checks can tell you</h3>
          <ul className="flex flex-col gap-1 text-ink-muted">
            {coverage.notes.map((n, i) => (
              <li key={i} className="max-w-[60ch]">
                {n}
              </li>
            ))}
          </ul>
        </section>
      )}
    </div>
  );
}
