// An install receipt in three levels (plan Part F): the summary card, then the
// files and install scripts, then each script with the matched lines marked.
import type { ColumnDef } from "@tanstack/react-table";
import { useRules, useReceipt, type Finding, type Receipt, type ReceiptScript } from "@/ipc";
import { ActionButton } from "./components/ActionButton";
import { CoverageList } from "./components/CoverageList";
import { DataTable } from "./components/DataTable";
import { EvidenceBlock } from "./components/EvidenceBlock";
import { FileTree, type FileTreeEntry, type FileTreeGroup } from "./components/FileTree";
import { IncompleteNotice } from "./components/IncompleteNotice";
import { KeyValue } from "./components/KeyValue";
import { SEVERITY_RANK, SeverityBadge } from "./components/SeverityBadge";
import { Skeleton } from "./components/Skeleton";
import { useDrilldown } from "./drilldown";
import { formatDate, formatTime, plural } from "./format";
import { FindingEvidence } from "./report";

const CATEGORY_ORDER = ["/etc", "systemd", "cron", "PAM", "libraries", "binaries", "other"];
/** Packages named before the list folds into "and N more". */
const PACKAGES_SHOWN = 5;

type Result = "Findings to review" | "No findings from these checks" | "Incomplete";

/** The three honest results, as for reports: anything not read makes it incomplete. */
export function receiptResult(r: Receipt): Result {
  if (r.findings.some((f) => f.severity !== "info")) return "Findings to review";
  return r.coverage.skipped.length > 0 ? "Incomplete" : "No findings from these checks";
}

function fileGroups(r: Receipt, only?: (f: Receipt["files"][number]) => boolean): FileTreeGroup[] {
  const kind = new Map(r.packages.map((p) => [p.name, p.kind]));
  const groups = new Map<string, FileTreeGroup>();
  for (const f of r.files.filter(only ?? (() => true))) {
    const group = groups.get(f.category) ?? { category: f.category, files: [] };
    group.files.push({
      path: f.path,
      change: kind.get(f.package) === "install" ? "added" : "changed",
      detail: f.privilege ? `${f.package}, ${f.privilege}` : f.package,
      severity: f.privilege ? "review" : undefined,
    });
    groups.set(f.category, group);
  }
  return [...groups.values()].sort((a, b) => CATEGORY_ORDER.indexOf(a.category) - CATEGORY_ORDER.indexOf(b.category));
}

function FileEvidence({ receipt, file }: { receipt: Receipt; file: FileTreeEntry }) {
  const f = receipt.files.find((x) => x.path === file.path);
  const pkg = receipt.packages.find((p) => p.name === f?.package);
  return (
    <KeyValue
      items={[
        { label: "Path", value: file.path, mono: true },
        { label: "Package", value: pkg ? `${pkg.name} ${pkg.version}` : (f?.package ?? "") },
        { label: "Kind", value: f?.category ?? "" },
        { label: "Raised privileges", value: f?.privilege ?? "None" },
        ...(receipt.repositories[f?.package ?? ""] ? [{ label: "Repository", value: receipt.repositories[f?.package ?? ""] ?? "" }] : []),
      ]}
    />
  );
}

function Files({ receipt, only }: { receipt: Receipt; only?: (f: Receipt["files"][number]) => boolean }) {
  const push = useDrilldown((s) => s.push);
  return (
    <FileTree
      groups={fileGroups(receipt, only)}
      onOpen={(file) => push({ title: file.path, content: <FileEvidence receipt={receipt} file={file} /> })}
    />
  );
}

/** One script with its matched lines highlighted and each rule named beside them. */
function ScriptCode({ receipt, index, script }: { receipt: Receipt; index: number; script: ReceiptScript }) {
  const rules = useRules().data;
  const hits = receipt.hits.filter((h) => h.script === index);
  const notes = Object.fromEntries(
    hits.map((h) => [
      h.line,
      { text: rules?.find((r) => r.id === h.rule)?.title ?? h.rule, tone: h.severity === "high" ? ("high" as const) : ("review" as const) },
    ]),
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

/** Scripts with marked lines first; in `marked` mode the rest fold into one line. */
function Scripts({ receipt, marked = false }: { receipt: Receipt; marked?: boolean }) {
  if (receipt.scripts.length === 0) return <p className="m-0 text-ink-muted">These packages run no install scripts.</p>;
  const indexed = receipt.scripts.map((script, index) => ({ script, index, hit: receipt.hits.some((h) => h.script === index) }));
  const withHits = indexed.filter((s) => s.hit);
  const rest = indexed.filter((s) => !s.hit);
  const render = ({ script, index }: (typeof indexed)[number]) => (
    <ScriptCode key={`${script.package}-${script.phase}`} receipt={receipt} index={index} script={script} />
  );
  return (
    <div className="flex flex-col gap-4">
      <p className="m-0 max-w-[56ch] text-ink-muted">
        Install scripts run as root while the package manager works. Marked lines matched a receipt rule.
      </p>
      {withHits.map(render)}
      {marked && rest.length > 0 ? (
        <details className="rounded-panel border border-rule bg-surface">
          <summary className="cursor-pointer px-3 py-2 text-body text-ink">
            {plural(rest.length, "other script")} with no marked lines
          </summary>
          <div className="flex flex-col gap-4 p-3">{rest.map(render)}</div>
        </details>
      ) : (
        rest.map(render)
      )}
    </div>
  );
}

/** A summary figure that opens its evidence, or plain text when there is none. */
function Figure({ text, onOpen }: { text: string; onOpen?: (() => void) | undefined }) {
  if (!onOpen) return <>{text}</>;
  return (
    <button type="button" onClick={onOpen} className="cursor-pointer text-left text-ink underline decoration-rule-strong underline-offset-4 hover:decoration-ink">
      {text}
    </button>
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

/** In a preview the real choice is the dialog's: stopping the transaction. */
const PREVIEW_ACTION = "If you did not expect this, stop the transaction.";

/**
 * `expanded` is the preview dialog's layout, where the side drill-down cannot open:
 * findings first, then only the scripts with marked lines, files as counts.
 */
export function ReceiptPanel({ receipt, expanded = false }: { receipt: Receipt; expanded?: boolean }) {
  const push = useDrilldown((s) => s.push);
  const result = receiptResult(receipt);
  const privileged = receipt.files.filter((f) => f.privilege);
  const toReview = receipt.findings.filter((f) => f.severity !== "info");
  const findings = receipt.findings.map((f, index) => ({ ...f, index }));
  const open = (title: string, content: React.JSX.Element) => (expanded ? undefined : () => push({ title, content }));
  const repositories = [...new Set(Object.values(receipt.repositories))];
  const shown = receipt.packages.slice(0, PACKAGES_SHOWN);
  const after = receipt.after.checked
    ? receipt.after.listeners.length
      ? plural(receipt.after.listeners.length, "new listening program")
      : "No new listening programs"
    : receipt.transaction.startsWith("preview-")
      ? "Checked after it runs"
      : "Checked a minute after the transaction";
  const categories = [...new Set(receipt.files.map((f) => f.category))]
    .sort((a, b) => CATEGORY_ORDER.indexOf(a) - CATEGORY_ORDER.indexOf(b))
    .map((c) => `${c} ${String(receipt.files.filter((f) => f.category === c).length)}`)
    .join(", ");
  return (
    <div className="flex flex-col gap-4">
      {result === "Incomplete" ? (
        <IncompleteNotice title="Incomplete: some of these packages could not be read" coverage={receipt.coverage} />
      ) : (
        <p className="m-0 font-semibold text-ink">
          {result === "Findings to review" ? `Findings to review: ${plural(toReview.length, "finding")}.` : `${result}.`}
        </p>
      )}
      {expanded &&
        receipt.findings.map((f) => (
          <section key={`${f.rule}-${f.evidence}`} className="border-t border-rule pt-4">
            <FindingEvidence finding={{ ...f, action: f.severity === "info" ? f.action : PREVIEW_ACTION }} />
          </section>
        ))}
      <KeyValue
        items={[
          { label: "When", value: `${formatDate(receipt.at)}, ${formatTime(receipt.at)}` },
          ...(receipt.command ? [{ label: "Command", value: receipt.command, mono: true }] : []),
          ...(repositories.length ? [{ label: "From", value: repositories.join(", ") }] : []),
          {
            label: "Packages",
            value: (
              <ul className="m-0 list-none p-0">
                {shown.map((p) => (
                  <li key={p.name}>
                    {p.name} {p.version} <span className="text-ink-muted">({p.kind})</span>
                  </li>
                ))}
                {receipt.packages.length > PACKAGES_SHOWN && (
                  <li className="text-ink-muted">and {plural(receipt.packages.length - PACKAGES_SHOWN, "more package")}</li>
                )}
              </ul>
            ),
          },
          {
            label: "Files",
            value: expanded ? (
              categories ? `${plural(receipt.files.length, "file")}: ${categories}` : "None listed"
            ) : (
              <Figure text={plural(receipt.files.length, "file")} onOpen={receipt.files.length ? open("Files", <Files receipt={receipt} />) : undefined} />
            ),
          },
          {
            label: "Install scripts",
            value: (
              <Figure
                text={`${plural(receipt.scripts.length, "script")}, ${plural(receipt.hits.length, "marked line")}`}
                onOpen={receipt.scripts.length ? open("Install scripts", <Scripts receipt={receipt} />) : undefined}
              />
            ),
          },
          {
            label: "Services and autostart",
            value: receipt.services.length ? (
              <Figure
                text={plural(receipt.services.length, "entry")}
                onOpen={open("Services and autostart", <EvidenceBlock label="Shipped by these packages" text={receipt.services.join("\n")} />)}
              />
            ) : (
              "None"
            ),
          },
          {
            label: "Raised privileges",
            value: privileged.length ? (
              <Figure text={plural(privileged.length, "file")} onOpen={open("Raised privileges", <Files receipt={receipt} only={(f) => f.privilege !== null} />)} />
            ) : (
              "None"
            ),
          },
          { label: "Started afterwards", value: after },
        ]}
      />
      {expanded ? (
        <section className="flex flex-col gap-2 border-t border-rule pt-4">
          <h3 className="m-0 text-small font-semibold text-ink-muted">Install scripts</h3>
          <Scripts receipt={receipt} marked />
        </section>
      ) : (
        <div className="flex flex-wrap gap-2">
          <ActionButton disabled={receipt.scripts.length === 0} onClick={() => push({ title: "Install scripts", content: <Scripts receipt={receipt} /> })}>
            Install scripts
          </ActionButton>
          <ActionButton disabled={receipt.files.length === 0} onClick={() => push({ title: "Files", content: <Files receipt={receipt} /> })}>
            Files
          </ActionButton>
        </div>
      )}
      {!expanded && findings.length > 0 && (
        <DataTable
          label="Findings"
          columns={findingColumns}
          data={findings}
          getRowId={(f) => String(f.index)}
          initialSorting={[{ id: "severity", desc: true }]}
          onOpen={(f) => push({ title: f.summary, content: <FindingEvidence finding={f} /> })}
          rowHeight={60}
          height={Math.min(300, 36 + findings.length * 60)}
        />
      )}
      {result !== "Incomplete" && <CoverageList coverage={receipt.coverage} />}
    </div>
  );
}

export function ReceiptById({ id }: { id: string }) {
  const receipt = useReceipt(id);
  if (receipt.isPending) return <Skeleton shape="rows" count={5} />;
  if (receipt.isError) return <IncompleteNotice title="This receipt could not be loaded" errors={[receipt.error.message]} />;
  return <ReceiptPanel receipt={receipt.data} />;
}
