// A live source that cannot read: one line where the gap shows, the details and
// the fix in the drill-down, so the gap is stated without taking over the page.
import { CircleSlash } from "lucide-react";
import { useSources, type SourceStatus } from "@/ipc";
import { ActionDialog } from "./action";
import { ActionButton } from "./components/ActionButton";
import { CoverageList, SKIP_REASON } from "./components/CoverageList";
import { useDrilldown } from "./drilldown";
import { formatTime } from "./format";

/** The fix Bigman can offer for a gap, when it has one. */
export function SourceRemedy({ source }: { source: SourceStatus }) {
  if (source.name === "denials" && source.why === "permission_denied") {
    return <ActionDialog action={{ kind: "journal_access", remove: false }} label="Join systemd-journal…" />;
  }
  return null;
}

function GapDetails({ gaps }: { gaps: SourceStatus[] }) {
  return (
    <div className="flex flex-col gap-4">
      <p className="m-0 max-w-[56ch] text-ink">
        Entries from {gaps.length === 1 ? "this source" : "these sources"} are missing until it can read again; nothing is
        assumed in the meantime.
      </p>
      <CoverageList coverage={{ checked: [], notes: [], skipped: gaps.map((s) => ({ what: s.label, why: s.why ?? "failed", detail: s.detail })) }} />
      {gaps.map((s) => (
        <div key={s.name}>
          <SourceRemedy source={s} />
        </div>
      ))}
    </div>
  );
}

/** "SELinux and AppArmor denials: not collected since 13:46 (no permission to read). Details" */
export function SourceGapLine() {
  const sources = useSources();
  const open = useDrilldown((s) => s.open);
  const gaps = (sources.data ?? []).filter((s) => !s.ok);
  const first = gaps[0];
  if (!first) return null;
  const since = first.last_run === null ? "" : ` at ${formatTime(first.last_run)}`;
  const text =
    gaps.length === 1
      ? `${first.label}: not collected${since} (${SKIP_REASON[first.why ?? "failed"].toLowerCase()}).`
      : `${String(gaps.length)} sources are not being collected.`;
  return (
    <div className="flex flex-wrap items-center gap-x-3 gap-y-1 text-body text-ink-muted">
      <CircleSlash aria-hidden size={16} className="shrink-0" />
      <span>{text}</span>
      <ActionButton variant="quiet" onClick={() => open({ title: "Not collected", content: <GapDetails gaps={gaps} /> })}>
        Details
      </ActionButton>
    </div>
  );
}
