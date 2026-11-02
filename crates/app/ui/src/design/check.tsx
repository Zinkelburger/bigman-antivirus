// How every check looks while it runs and when it ends: progress with cancel,
// then the report, or the daemon's error verbatim.
import { useEffect, useRef } from "react";
import type { Report } from "@/ipc";
import { IncompleteNotice } from "./components/IncompleteNotice";
import { ProgressRow } from "./components/ProgressRow";
import { nowSeconds } from "./format";
import { ReportPanel, reportFigure, reportStatus, reportTitle } from "./report";

interface CheckLike {
  isPending: boolean;
  data?: Report | undefined;
  error: Error | null;
  cancel: () => void;
}

export function CheckStatus({ check, running, detail }: { check: CheckLike; running: string; detail?: string }) {
  const heading = useRef<HTMLHeadingElement>(null);
  const progress = useRef<HTMLDivElement>(null);
  const resultId = check.data?.id;
  const pending = check.isPending;
  // The button that started the check is disabled while it runs; focus follows
  // the work: the progress row, then the result.
  useEffect(() => {
    if (pending) progress.current?.focus();
  }, [pending]);
  useEffect(() => {
    if (resultId) heading.current?.focus();
  }, [resultId]);
  if (check.isPending) {
    return (
      <div ref={progress} tabIndex={-1} className="outline-none">
        <ProgressRow label={running} detail={detail} progress={null} onCancel={check.cancel} />
      </div>
    );
  }
  if (check.error) return <IncompleteNotice title="The check did not finish" errors={[check.error.message]} />;
  if (check.data) {
    return (
      <section aria-label="Result" className="rounded-panel border border-rule bg-surface p-4">
        <h2 ref={heading} tabIndex={-1} className="m-0 mb-3 rounded-control text-body font-semibold text-ink">
          {reportTitle(check.data.kind)}: {reportFigure(check.data) ?? reportStatus(check.data)}
        </h2>
        <p className="sr-only" role="status">
          {reportTitle(check.data.kind)} finished: {reportFigure(check.data) ?? reportStatus(check.data)}.
        </p>
        <ReportPanel report={check.data} now={nowSeconds()} />
      </section>
    );
  }
  return null;
}
