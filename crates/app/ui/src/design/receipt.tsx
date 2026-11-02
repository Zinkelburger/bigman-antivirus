// An install receipt in three levels (plan Part F): the summary card, then the
// files and install scripts, then each script with the matched lines marked.
import type { ColumnDef } from "@tanstack/react-table";
import { useReceipt, type Finding, type Receipt, type ReceiptScript } from "@/ipc";
import { ActionButton } from "./components/ActionButton";
import { CoverageList } from "./components/CoverageList";
import { DataTable } from "./components/DataTable";
import { EvidenceBlock } from "./components/EvidenceBlock";
import { FileTree, type FileTreeGroup } from "./components/FileTree";
import { IncompleteNotice } from "./components/IncompleteNotice";
import { KeyValue } from "./components/KeyValue";
import { SEVERITY_RANK, SeverityBadge } from "./components/SeverityBadge";
import { Skeleton } from "./components/Skeleton";
import { useDrilldown } from "./drilldown";
import { formatDate, formatTime, plural } from "./format";
import { FindingEvidence } from "./report";

const CATEGORY_ORDER = ["/etc", "systemd", "cron", "PAM", "libraries", "binaries", "other"];

function fileGroups(r: Receipt): FileTreeGroup[] {
  const kind = new Map(r.packages.map((p) => [p.name, p.kind]));
  const groups = new Map<string, FileTreeGroup>();
  for (const f of r.files) {
    const group = groups.get(f.category) ?? { category: f.category, files: [] };
    group.files.push({
      path: f.path,
      change: kind.get(f.package) === "install" ? "added" : "changed",
      detail: f.privilege ?? f.package,
      severity: f.privilege ? "review" : undefined,
    });
    groups.set(f.category, group);
  }
  return [...groups.values()].sort((a, b) => CATEGORY_ORDER.indexOf(a.category) - CATEGORY_ORDER.indexOf(b.category));
}

/** One script with its matched lines highlighted and each rule named beside them. */
function ScriptCode({ receipt, index, script }: { receipt: Receipt; index: number; script: ReceiptScript }) {
  const hits = receipt.hits.filter((h) => h.script === index);
  const notes = Object.fromEntries(
    hits.map((h) => [h.line, { text: h.rule, tone: h.severity === "high" ? ("high" as const) : ("review" as const) }]),
  );
  return (
    <EvidenceBlock
      label={`${script.package} ${script.phase} (${script.interpreter})${script.changed ? ", changed since the last version" : ""}`}
      text={script.text}
      highlight={hits.map((h) => h.line)}
      notes={notes}
    />
  );
}

function Scripts({ receipt }: { receipt: Receipt }) {
  if (receipt.scripts.length === 0) return <p className="m-0 text-ink-muted">These packages run no install scripts.</p>;
  return (
    <div className="flex flex-col gap-4">
      <p className="m-0 max-w-[56ch] text-ink-muted">
        Install scripts run as root while the package manager works. Marked lines matched a receipt rule.
      </p>
      {receipt.scripts.map((s, i) => (
        <ScriptCode key={`${s.package}-${s.phase}`} receipt={receipt} index={i} script={s} />
      ))}
    </div>
  );
}

const findingColumns: ColumnDef<Finding & { index: number }>[] = [
  {
    id: "severity",
    header: "Severity",
    accessorFn: (f) => SEVERITY_RANK[f.severity],
    cell: ({ row }) => <SeverityBadge severity={row.original.severity} />,
    meta: { width: "6.5rem" },
  },
  {
    accessorKey: "summary",
    header: "Found",
    cell: ({ row }) => (
      <span className="flex min-w-0 flex-col">
        <span className="line-clamp-2 whitespace-normal text-ink">{row.original.summary}</span>
        <span className="truncate font-mono text-small text-ink-muted">{row.original.rule}</span>
      </span>
    ),
  },
];

export function ReceiptPanel({ receipt }: { receipt: Receipt }) {
  const push = useDrilldown((s) => s.push);
  const privileged = receipt.files.filter((f) => f.privilege).length;
  const toReview = receipt.findings.filter((f) => f.severity !== "info");
  const findings = receipt.findings.map((f, index) => ({ ...f, index }));
  const after = receipt.after.checked
    ? receipt.after.listeners.length
      ? plural(receipt.after.listeners.length, "new listening program")
      : "None"
    : "Checked a minute after the transaction";
  return (
    <div className="flex flex-col gap-4">
      <p className="m-0 font-semibold text-ink">
        {toReview.length ? `Findings to review: ${plural(toReview.length, "finding")}.` : "No findings from these checks."}
      </p>
      <KeyValue
        items={[
          { label: "When", value: `${formatDate(receipt.at)}, ${formatTime(receipt.at)}` },
          ...(receipt.command ? [{ label: "Command", value: receipt.command, mono: true }] : []),
          {
            label: "Packages",
            value: receipt.packages.map((p) => `${p.name} ${p.version} (${p.kind})`).join(", "),
          },
          { label: "Files", value: plural(receipt.files.length, "file") },
          { label: "Install scripts", value: `${plural(receipt.scripts.length, "script")}, ${plural(receipt.hits.length, "marked line")}` },
          { label: "Services and autostart", value: receipt.services.length ? plural(receipt.services.length, "entry") : "None" },
          { label: "Raised privileges", value: privileged ? plural(privileged, "file") : "None" },
          { label: "Started afterwards", value: after },
        ]}
      />
      <div className="flex flex-wrap gap-2">
        <ActionButton onClick={() => push({ title: "Install scripts", content: <Scripts receipt={receipt} /> })}>
          Install scripts
        </ActionButton>
        <ActionButton
          disabled={receipt.files.length === 0}
          onClick={() => push({ title: "Files", content: <FileTree groups={fileGroups(receipt)} /> })}
        >
          Files
        </ActionButton>
      </div>
      {findings.length > 0 && (
        <DataTable
          label="Findings"
          columns={findingColumns}
          data={findings}
          getRowId={(f) => String(f.index)}
          initialSorting={[{ id: "severity", desc: true }]}
          onOpen={(f) => push({ title: f.summary, content: <FindingEvidence finding={f} /> })}
          rowHeight={60}
          height={300}
        />
      )}
      <CoverageList coverage={receipt.coverage} />
    </div>
  );
}

export function ReceiptById({ id }: { id: string }) {
  const receipt = useReceipt(id);
  if (receipt.isPending) return <Skeleton shape="rows" count={5} />;
  if (receipt.isError) return <IncompleteNotice title="This receipt could not be loaded" errors={[receipt.error.message]} />;
  return <ReceiptPanel receipt={receipt.data} />;
}
