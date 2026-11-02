// Drill-down content for any Report: findings table (level two) and one
// finding's evidence (level three), plus the report's life-list entry.
// Composed only from design components.
import type { ColumnDef } from "@tanstack/react-table";
import { BellRing, FileSearch, FolderSearch, ListChecks, ScanSearch, Stethoscope, type LucideIcon } from "lucide-react";
import { useRules, type Finding, type Report } from "@/ipc";
import { CoverageList } from "./components/CoverageList";
import { DataTable } from "./components/DataTable";
import { EvidenceBlock } from "./components/EvidenceBlock";
import { KeyValue } from "./components/KeyValue";
import { maxSeverity, SEVERITY_RANK, SeverityBadge } from "./components/SeverityBadge";
import { ReportFollowUp } from "./report-actions";
import { useDrilldown } from "./drilldown";
import { ago, formatTime, plural } from "./format";
import type { Occurrence } from "./timeline";

export type ReportResult = "Findings to review" | "No findings from these checks" | "Incomplete";

/** The three honest results, from the report itself. Info items are notes, not findings to review. */
export function reportStatus(report: Report): ReportResult {
  if (report.errors.length > 0 || report.coverage.skipped.length > 0) return "Incomplete";
  return report.findings.some((f) => f.severity !== "info") ? "Findings to review" : "No findings from these checks";
}

const UPDATE_RULES = ["advisory.update-available", "advisory.exploited"];

/**
 * The one figure a report is known by, when its kind has one: "3 security
 * updates", "No updates listed", "Release no longer supported". History keeps a
 * report's 20 most severe findings, so past that the count is a floor.
 */
export function reportFigure(report: Report): string | null {
  if (report.kind !== "Security updates") return null;
  if (report.findings.some((f) => f.rule === "advisory.unsupported-release")) return "Release no longer supported";
  const n = report.findings.filter((f) => UPDATE_RULES.includes(f.rule)).length;
  if (n === 0) return reportStatus(report) === "Incomplete" ? "Not checked completely" : "No updates listed";
  return (report.summary?.findings ?? 0) > report.findings.length ? `${String(n)}+ security updates` : plural(n, "security update");
}

/** Severity of a report as a whole: its most severe finding. */
export function reportSeverity(report: Report) {
  return maxSeverity(report.findings.map((f) => f.severity));
}

/** Plain names for report kinds, for headings and lists. */
const KIND_TITLE: Record<string, string> = {
  "Persistence changes": "Startup files",
  "Security updates": "Security updates",
  "Project inspection": "Project",
  "Lockfile check": "Lockfile",
  "Package sources": "Package sources",
  "Repository check": "Project",
  "Package script inventory": "Package scripts",
  "Restricted installer rehearsal": "Installer rehearsal",
  "Security checkup": "Checkup",
  "lynis integration": "Lynis audit",
  "osquery integration": "osquery process list",
  "osv integration": "Dependency advisories",
  "Canary verification": "Canary test",
  "Local canary setup": "Canary setup",
};

export function reportTitle(kind: string): string {
  return KIND_TITLE[kind] ?? kind;
}

/** A canary report that proved detection works. */
export function verifiesDetection(report: Report): boolean {
  return report.findings.some((f) => f.rule === "canary.verified");
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
  const figure = reportFigure(report);
  if (figure) return figure.charAt(0).toLowerCase() + figure.slice(1);
  const result = reportStatus(report);
  if (result === "Incomplete") return "incomplete";
  if (result === "No findings from these checks") return "no findings";
  const count = report.summary?.to_review ?? report.findings.filter((f) => f.severity !== "info").length;
  return `${plural(count, "finding")} to review`;
}

/**
 * What folding groups by: the same check of the same thing with the same kind of
 * result. A run with findings never folds into later clean runs, so it stays seen.
 */
function outcomeClass(report: Report): string {
  const result = reportStatus(report);
  return result === "Findings to review" ? (reportSeverity(report) ?? "info") : result;
}

export function reportKey(report: Report): string {
  return `${report.kind}\u0000${report.subject}\u0000${outcomeClass(report)}`;
}

/** One report as a life-list occurrence. */
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
          : `${reportTitle(report.kind)}: ${report.subject}`;
  return {
    id: report.id,
    key: reportKey(report),
    at: report.created,
    summary: `${what} · ${outcome(report)}`,
    source: reportTitle(report.kind),
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
    const key = reportKey(r);
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

/** The registry's explanation of a finding's rule, so every finding says what it means. */
export function AboutRule({ id }: { id: string }) {
  const rule = useRules().data?.find((r) => r.id === id);
  if (!rule) return <KeyValue items={[{ label: "Rule", value: id, mono: true }]} />;
  return (
    <section aria-label="About this rule" className="flex flex-col gap-2 border-t border-rule pt-3">
      <h3 className="m-0 text-small font-semibold text-ink-muted">About this rule</h3>
      <p className="m-0 max-w-[56ch] text-ink">{rule.explanation}</p>
      <KeyValue
        items={[
          { label: "Rule", value: rule.id, mono: true },
          // IDs in Bigman's catalogue, docs/threats.
          ...(rule.threats.length ? [{ label: "Catalogue", value: rule.threats.join(", "), mono: true }] : []),
        ]}
      />
    </section>
  );
}

export function FindingEvidence({ finding }: { finding: Finding }) {
  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-start gap-3">
        <SeverityBadge severity={finding.severity} />
        <p className="m-0 font-semibold text-ink">{finding.summary}</p>
      </div>
      <EvidenceBlock text={finding.evidence} label="Evidence" />
      <section className="flex flex-col gap-2">
        <h3 className="m-0 text-small font-semibold text-ink-muted">What you can do</h3>
        <p className="m-0 max-w-[56ch] text-ink">{finding.action}</p>
        {finding.command && <EvidenceBlock text={finding.command} label="Command (Bigman does not run it)" />}
      </section>
      <AboutRule id={finding.rule} />
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
          { label: "Result", value: verifiesDetection(report) && status !== "Incomplete" ? "Detection verified" : status },
          { label: "Looked at", value: report.subject, mono: report.subject.startsWith("/") },
          { label: "Ran at", value: `${formatTime(report.created)}, ${ago(report.created, now)}` },
          ...(report.sha256 ? [{ label: "SHA-256", value: report.sha256, mono: true }] : []),
        ]}
      />
      {verifiesDetection(report) && (
        <p className="m-0 flex items-center gap-2 text-ink">
          <span aria-hidden className="size-2 shrink-0 rounded-full bg-calm" />
          Local read detection verified: a real read produced a fresh receipt from the monitor.
        </p>
      )}
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
      <ReportFollowUp report={report} />
      <CoverageList coverage={report.coverage} errors={report.errors} />
    </div>
  );
}
