// Shared reading of a hardening review, for Overview's tile and Protect > Hardening,
// so both count the same things the same way.
import type { HardeningReview, Suggestion } from "@/ipc";
import { plural } from "./format";

export interface HardeningSummary {
  /** The one sentence the eye lands on, or null when nothing is open. */
  line: string | null;
  /** Every suggestion not left as it is, review first. */
  open: Suggestion[];
  /** Review and High: worth a look. */
  toReview: Suggestion[];
  /** Info: optional. */
  optional: Suggestion[];
}

export function hardeningSummary(review: HardeningReview): HardeningSummary {
  const open = review.suggestions.filter((s) => !s.kept);
  const toReview = open.filter((s) => s.severity !== "info");
  const optional = open.filter((s) => s.severity === "info");
  const line =
    open.length === 0
      ? null
      : toReview.length === 0
        ? `${plural(optional.length, "optional suggestion")}, nothing to review.`
        : `Suggestions to review: ${toReview.length}${optional.length ? `, plus ${optional.length} optional` : ""}.`;
  return { line, open: [...toReview, ...optional], toReview, optional };
}
