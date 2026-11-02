// A sub-view's hero when nothing has run this session: the newest saved result of
// that check, so leaving and coming back never loses it.
import type { Report } from "@/ipc";
import { ActionButton } from "./components/ActionButton";
import { SeverityBadge } from "./components/SeverityBadge";
import { useDrilldown } from "./drilldown";
import { ago, nowSeconds } from "./format";
import { reportFigure, reportSeverity, reportStatus, reportTitle } from "./report";
import { ReportById } from "./report-by-id";

export function LastResult({ report, empty }: { report: Report | undefined; empty: string }) {
  const open = useDrilldown((s) => s.open);
  if (!report) return <p className="m-0 text-body text-ink-muted">{empty}</p>;
  const severity = reportSeverity(report);
  return (
    <section aria-label="Last result" className="flex flex-wrap items-center gap-x-6 gap-y-2 rounded-panel border border-rule bg-surface p-4">
      <p className="m-0 flex min-w-0 flex-1 flex-wrap items-center gap-2 text-body text-ink">
        <span className="font-semibold">{reportFigure(report) ?? reportStatus(report)}</span>
        {severity && severity !== "info" && <SeverityBadge severity={severity} />}
        <span className="text-small text-ink-muted">Last run {ago(report.created, nowSeconds())}</span>
      </p>
      <ActionButton onClick={() => open({ title: reportTitle(report.kind), content: <ReportById id={report.id} /> })}>Open</ActionButton>
    </section>
  );
}
