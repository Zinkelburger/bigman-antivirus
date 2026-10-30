import { Eye, Info, TriangleAlert, type LucideIcon } from "lucide-react";
import type { Severity } from "@/ipc";

const STYLE: Record<Severity, { label: string; icon: LucideIcon; className: string }> = {
  info: { label: "Info", icon: Info, className: "bg-info-soft text-info" },
  review: { label: "Review", icon: Eye, className: "bg-review-soft text-review" },
  high: { label: "High", icon: TriangleAlert, className: "bg-high-soft text-high" },
};

/** Severity as icon plus text; color is never the only signal. */
export function SeverityBadge({ severity }: { severity: Severity }) {
  const { label, icon: Icon, className } = STYLE[severity];
  return (
    <span
      className={`inline-flex shrink-0 items-center gap-1 rounded-control px-1.5 py-0.5 text-small font-semibold ${className}`}
    >
      <Icon aria-hidden size={13} strokeWidth={2.25} />
      {label}
    </span>
  );
}

/** Higher is more severe; sort descending to put High first. */
export const SEVERITY_RANK: Record<Severity, number> = { info: 0, review: 1, high: 2 };

/** The most severe of a list, or null for an empty list. */
export function maxSeverity(severities: readonly Severity[]): Severity | null {
  return severities.reduce<Severity | null>((m, s) => (m === null || SEVERITY_RANK[s] > SEVERITY_RANK[m] ? s : m), null);
}
