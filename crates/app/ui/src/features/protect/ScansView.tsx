import type { ColumnDef } from "@tanstack/react-table";
import { useState } from "react";
import {
  ActionButton,
  ago,
  CheckStatus,
  DataTable,
  EmptyState,
  heldIn,
  nowSeconds,
  PathPicker,
  plural,
  ReportById,
  reportSeverity,
  reportStatus,
  SeverityBadge,
  signatureAge,
  Skeleton,
  useDrilldown,
} from "@/design";
import { desktopShell, useCheck, useHistory, useQuarantined, useScanItems, useStatus, type ProtectionStatus, type Report } from "@/ipc";

const columns = (now: number): ColumnDef<Report>[] => [
  { id: "when", header: "When", accessorFn: (r) => r.created, cell: ({ row }) => ago(row.original.created, now), meta: { width: "9rem" } },
  { accessorKey: "subject", header: "Scanned", meta: { mono: true } },
  {
    id: "result",
    header: "Result",
    accessorFn: (r) => reportStatus(r),
    cell: ({ row }) => {
      const severity = reportSeverity(row.original);
      return severity && severity !== "info" ? <SeverityBadge severity={severity} /> : reportStatus(row.original);
    },
    meta: { width: "15rem" },
  },
];

/** The hero: what the last scan found and how fresh the signatures are. */
function ScanStatus({ last, status, onUpdate, busy, now }: { last: Report | undefined; status: ProtectionStatus | undefined; onUpdate: () => void; busy: boolean; now: number }) {
  const open = useDrilldown((s) => s.open);
  const signatures = status?.signatures;
  const stale = !signatures || now - signatures.updated > 3 * 86_400;
  // Files already moved to quarantine no longer need review here.
  const items = useScanItems(last?.id ?? null);
  const quarantine = useQuarantined();
  const flagged = (items.data ?? []).filter((item) => item.detected && !heldIn(quarantine.data, item)).length;
  const severity = last ? reportSeverity(last) : null;
  return (
    <section aria-label="Scan status" className="flex flex-wrap items-center gap-x-6 gap-y-3 rounded-panel border border-rule bg-surface p-4">
      <div className="flex min-w-0 flex-1 flex-col gap-1">
        <p className="m-0 flex flex-wrap items-center gap-2 text-body font-semibold text-ink">
          {last
            ? reportStatus(last) === "Findings to review" && flagged > 0
              ? `${plural(flagged, "file")} to review in ${last.subject}`
              : reportStatus(last) === "Findings to review"
                ? `Flagged files from the last scan of ${last.subject} are in quarantine`
              : reportStatus(last) === "Incomplete"
                ? `The last scan of ${last.subject} is incomplete`
                : `No findings from the last scan of ${last.subject}`
            : "No scans yet"}
          {flagged > 0 && severity && severity !== "info" && <SeverityBadge severity={severity} />}
        </p>
        <p className="m-0 text-small text-ink-muted">
          {last ? `Ran ${ago(last.created, now)}` : "Scan Downloads to see what ClamAV signatures match."} ·{" "}
          {signatures ? `Signatures ${signatureAge(signatures.updated, now)}` : "No signatures yet"}
        </p>
      </div>
      <div className="flex gap-2">
        {last && reportStatus(last) !== "No findings from these checks" && (
          <ActionButton onClick={() => open({ title: "File scan", content: <ReportById id={last.id} /> })}>Open</ActionButton>
        )}
        {stale && (
          <ActionButton disabled={busy} onClick={onUpdate}>
            Update signatures
          </ActionButton>
        )}
      </div>
    </section>
  );
}

export function ScansView() {
  const status = useStatus();
  const history = useHistory();
  const scan = useCheck();
  const update = useCheck();
  const open = useDrilldown((s) => s.open);
  const [path, setPath] = useState<string | null>(null);
  const now = nowSeconds();
  const target = path ?? status.data?.downloads ?? "";
  const scans = (history.data ?? []).filter((r) => r.kind === "File scan");
  const busy = scan.isPending || update.isPending;
  const runScan = () => scan.mutate({ type: "scan", path: target });
  const browse = async () => {
    const chosen = await desktopShell().choosePath("directory", "Choose a folder to scan");
    if (chosen) setPath(chosen);
  };

  if (history.isPending || status.isPending) return <Skeleton shape="rows" count={5} />;
  return (
    <div className="flex flex-col gap-5">
      <ScanStatus last={scans[0]} status={status.data} onUpdate={() => update.mutate({ type: "update_signatures" })} busy={busy} now={now} />
      <CheckStatus check={update} running="Updating signatures" detail="Downloads ClamAV definitions into Bigman's private store." />
      <section aria-label="Scan files" className="flex flex-col gap-3">
        <PathPicker label="Folder or file to scan" value={target} onChange={setPath} onBrowse={() => void browse()} />
        <div className="flex flex-wrap items-center gap-2">
          <ActionButton variant="primary" disabled={busy || !target} onClick={runScan}>
            Scan with ClamAV
          </ActionButton>
          <span className="text-small text-ink-muted">Private copies are scanned, so files are never changed. Nothing is removed without your say.</span>
        </div>
      </section>
      <CheckStatus check={scan} running={`Scanning ${target}`} detail="At most 2,000 files, 64 MiB each, 512 MiB in total." />
      <section aria-labelledby="recent-scans" className="flex flex-col gap-2">
        <h2 id="recent-scans" className="m-0 text-body font-semibold text-ink">
          Recent scans
        </h2>
        {scans.length ? (
          <DataTable
            label="Recent scans"
            columns={columns(now)}
            data={scans}
            getRowId={(r) => r.id}
            initialSorting={[{ id: "when", desc: true }]}
            onOpen={(r) => open({ title: "File scan", content: <ReportById id={r.id} /> })}
            height={300}
          />
        ) : (
          <EmptyState>Your scans are listed here with what each one found.</EmptyState>
        )}
      </section>
    </div>
  );
}
