// Drill-down content for any Report: findings table (level two) and one
// finding's evidence (level three), plus the report's life-list entry.
// Composed only from design components.
import type { ColumnDef } from "@tanstack/react-table";
import { BellRing, FileSearch, FolderSearch, ListChecks, ScanSearch, Stethoscope, type LucideIcon } from "lucide-react";
import type { Finding, Report } from "@/ipc";
import { CoverageList } from "./components/CoverageList";
import { DataTable } from "./components/DataTable";
import { EvidenceBlock } from "./components/EvidenceBlock";
import { KeyValue } from "./components/KeyValue";
import { maxSeverity, SEVERITY_RANK, SeverityBadge } from "./components/SeverityBadge";
import { StatusLine } from "./components/StatusLine";
import { useDrilldown } from "./drilldown";
import { formatTime, plural } from "./format";
import type { Occurrence } from "./timeline";

export type ReportResult = "Findings to review" | "No findings from these checks" | "Incomplete";

/** The three honest results, from the report itself. Info items are notes, not findings to review. */
export function reportStatus(report: Report): ReportResult {
  if (report.errors.length > 0 || report.coverage.skipped.length > 0) return "Incomplete";
  return report.findings.some((f) => f.severity !== "info") ? "Findings to review" : "No findings from these checks";
}

/** Severity of a report as a whole: its most severe finding. */
export function reportSeverity(report: Report) {
  return maxSeverity(report.findings.map((f) => f.severity));
}

const KIND_ICON: Record<string, LucideIcon> = {
  "File scan": FolderSearch,
  "PDF triage": FileSearch,
  "Installer inspection": ScanSearch,
  "Package script inventory": ScanSearch,
  "Security checkup": Stethoscope,
  "Persistence changes": ListChecks,
  "Canary triggered": BellRing,
};

const KIND_VERB: Record<string, string> = {
  "File scan": "Scanned",
  "PDF triage": "Read the PDF",
  "Installer inspection": "Inspected",
  "Package script inventory": "Listed package scripts in",
  "Persistence changes": "Compared startup files",
};

function outcome(report: Report): string {
  const result = reportStatus(report);
  if (result === "Incomplete") return "incomplete";
  if (result === "No findings from these checks") return "no findings";
  return `${plural(report.findings.filter((f) => f.severity !== "info").length, "finding")} to review`;
}

/** One report as a life-list occurrence: the same check of the same thing folds together. */
export function reportOccurrence(report: Report, seenBefore: number): Occurrence {
  const verb = KIND_VERB[report.kind];
  const what =
    report.kind === "Security checkup"
      ? "Ran a checkup"
      : report.kind === "Canary triggered"
        ? `A canary file was opened: ${report.subject}`
        : verb
          ? report.kind === "Persistence changes"
            ? verb
            : `${verb} ${report.subject}`
          : `${report.kind}: ${report.subject}`;
  return {
    id: report.id,
    key: `${report.kind}\u0000${report.subject}`,
    at: report.created,
    summary: `${what} · ${outcome(report)}`,
    source: report.kind,
    icon: KIND_ICON[report.kind] ?? ListChecks,
    severity: reportSeverity(report) ?? "info",
    seenBefore,
  };
}

/**
 * Occurrences for a newest-first report list. "Seen before" counts earlier reports
 * of the same check on the same subject within the list the daemon returned.
 */
export function reportOccurrences(reports: readonly Report[]): Occurrence[] {
  const seen = new Map<string, number>();
  const oldestFirst = [...reports].sort((a, b) => a.created - b.created);
  return oldestFirst.map((r) => {
    const key = `${r.kind}\u0000${r.subject}`;
    const before = seen.get(key) ?? 0;
    seen.set(key, before + 1);
    return reportOccurrence(r, before);
  });
}

interface IndexedFinding extends Finding {
  index: number;
}

const columns: ColumnDef<IndexedFinding>[] = [
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

export function FindingEvidence({ finding }: { finding: Finding }) {
  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-start gap-3">
        <SeverityBadge severity={finding.severity} />
        <p className="m-0 font-semibold text-ink">{finding.summary}</p>
      </div>
      <EvidenceBlock text={finding.evidence} label="Evidence" />
      <section>
        <h3 className="m-0 mb-1 text-small font-semibold text-ink-muted">What you can do</h3>
        <p className="m-0 max-w-[65ch] text-ink">{finding.action}</p>
      </section>
      <KeyValue items={[{ label: "Rule", value: finding.rule, mono: true }]} />
    </div>
  );
}

export function ReportPanel({ report, now }: { report: Report; now: number }) {
  const push = useDrilldown((s) => s.push);
  const status = reportStatus(report);
  const findings = report.findings.map((f, index) => ({ ...f, index }));
  return (
    <div className="flex flex-col gap-4">
      <KeyValue
        items={[
          { label: "Result", value: status },
          { label: "Checked", value: report.subject, mono: true },
          { label: "Ran at", value: formatTime(report.created) },
          ...(report.sha256 ? [{ label: "SHA-256", value: report.sha256, mono: true }] : []),
        ]}
      />
      {status === "No findings from these checks" && <StatusLine ranAt={report.created} now={now} />}
      {findings.length > 0 ? (
        <DataTable
          label={status === "Findings to review" ? "Findings" : "Notes"}
          columns={columns}
          data={findings}
          getRowId={(f) => String(f.index)}
          initialSorting={[{ id: "severity", desc: true }]}
          onOpen={(f) => push({ title: f.summary, content: <FindingEvidence finding={f} /> })}
          rowHeight={60}
          height={360}
        />
      ) : (
        status === "Incomplete" && (
          <p className="m-0 text-body text-ink-muted">No findings were recorded, and some items were not checked.</p>
        )
      )}
      <CoverageList coverage={report.coverage} errors={report.errors} />
    </div>
  );
}
