// How every check looks while it runs and when it ends: progress with cancel,
// then the report, or the daemon's error verbatim.
import { useEffect, useRef } from "react";
import type { Report } from "@/ipc";
import { IncompleteNotice } from "./components/IncompleteNotice";
import { ProgressRow } from "./components/ProgressRow";
import { nowSeconds } from "./format";
import { ReportPanel, reportTitle } from "./report";

interface CheckLike {
  isPending: boolean;
  data?: Report | undefined;
  error: Error | null;
  cancel: () => void;
}

export function CheckStatus({ check, running, detail }: { check: CheckLike; running: string; detail?: string }) {
  const heading = useRef<HTMLHeadingElement>(null);
  const resultId = check.data?.id;
  // The button that started the check may be gone; put focus on what it produced.
  useEffect(() => {
    if (resultId) heading.current?.focus();
  }, [resultId]);
  if (check.isPending) return <ProgressRow label={running} detail={detail} progress={null} onCancel={check.cancel} />;
  if (check.error) return <IncompleteNotice title="The check did not finish" errors={[check.error.message]} />;
  if (check.data) {
    return (
      <section aria-label="Result" className="rounded-panel border border-rule bg-surface p-4">
        <h2 ref={heading} tabIndex={-1} className="m-0 mb-3 text-body font-semibold text-ink outline-none">
          {reportTitle(check.data.kind)}
        </h2>
        <ReportPanel report={check.data} now={nowSeconds()} />
      </section>
    );
  }
  return null;
}
