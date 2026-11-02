// Lists hold history summaries, which are shortened. Opening one always loads
// the full report by id, so the same report reads the same everywhere.
import { Download } from "lucide-react";
import { desktopShell, useReport } from "@/ipc";
import { ActionButton } from "./components/ActionButton";
import { IncompleteNotice } from "./components/IncompleteNotice";
import { Skeleton } from "./components/Skeleton";
import { useToasts } from "./components/Toast";
import { nowSeconds } from "./format";
import { ReportPanel } from "./report";

export function ReportById({ id }: { id: string }) {
  const report = useReport(id);
  const toast = useToasts((s) => s.push);
  if (report.isPending) return <Skeleton shape="rows" count={5} />;
  if (report.isError) return <IncompleteNotice title="This report could not be loaded" errors={[report.error.message]} />;
  const save = async () => {
    try {
      const path = await desktopShell().exportReport(id);
      if (path) toast(`Saved ${path}. It may contain local paths and inspected source.`);
    } catch (error) {
      toast(error instanceof Error ? error.message : String(error));
    }
  };
  return (
    <div className="flex flex-col gap-4">
      <ReportPanel report={report.data} now={nowSeconds()} />
      <div>
        <ActionButton icon={Download} onClick={() => void save()}>
          Export report…
        </ActionButton>
      </div>
    </div>
  );
}
