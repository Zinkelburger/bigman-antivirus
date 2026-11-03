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
  "Installer inspection": "Installer script",
  "Archive check": "Archive",
  "Launcher check": "Desktop launcher",
  "Package file": "Package file",
  "Package archive": "Package archive",
  "PKGBUILD check": "PKGBUILD",
  "AppImage check": "AppImage",
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

/** What the daemon recognised, from the coverage line its inspector wrote: "3 files and 1 install script (deb)". */
const FORMAT_NAME: readonly [RegExp, string][] = [
  [/\(deb\)$/, "Debian package"],
  [/\(rpm\)$/, "RPM package"],
  [/\(pacman\)$/, "Arch package"],
  [/\(zip\)$/, "Zip archive"],
  [/\(tar(\.\w+)?\)$/, "Tar archive"],
];
const ECOSYSTEMS = ["Python wheel", "Python source package", "Rust crate", "VS Code extension"];

/**
 * A report's name as the reader would call the thing: "Debian package",
 * "Python wheel", "Desktop launcher", falling back to the kind's title.
 */
export function reportName(report: Pick<Report, "kind" | "subject" | "coverage">): string {
  const checked = report.coverage.checked;
  if (report.kind === "Package archive") {
    const eco = checked.find((c) => ECOSYSTEMS.includes(c));
    if (eco) return eco;
  }
  if (report.kind === "Package file" || report.kind === "Archive check") {
    for (const [pattern, name] of FORMAT_NAME) if (checked.some((c) => pattern.test(c))) return name;
  }
  if (report.kind === "Package script inventory" && /\.(tgz|tar\.gz)$/.test(report.subject)) return "npm package";
  return reportTitle(report.kind);
}

/** The last part of a path or URL, for titles. */
export function baseName(subject: string): string {
  return subject.replace(/\/+$/, "").split("/").pop() || subject;
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

function escapeRegExp(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/**
 * For an install-script finding ("fixture runs a postinst script as root"), the
 * lines other findings in the report point at ("postinst script, line 3: …"),
 * so the script shows each one beside its rule.
 */
function scriptNotes(finding: Finding, report: Report | undefined) {
  const phase = / runs an? (.+) script as root$/.exec(finding.summary)?.[1];
  const notes: Record<number, { text: string; tone: "review" | "high" }> = {};
  if (!phase || !report) return notes;
  const at = new RegExp(`(?:^|\\n|\\s)${escapeRegExp(phase)} script, line (\\d+):`, "g");
  for (const other of report.findings) {
    if (other === finding || other.severity === "info" || other.rule === finding.rule) continue;
    for (const match of other.evidence.matchAll(at)) {
      const line = Number(match[1]);
      const existing = notes[line];
      if (existing?.text.split(", ").includes(other.rule)) continue;
      notes[line] = {
        text: existing ? `${existing.text}, ${other.rule}` : other.rule,
        tone: existing?.tone === "high" || other.severity === "high" ? "high" : "review",
      };
    }
  }
  return notes;
}

export function FindingEvidence({ finding, report }: { finding: Finding; report?: Report }) {
  const notes = scriptNotes(finding, report);
  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-start gap-3">
        <SeverityBadge severity={finding.severity} />
        <p className="m-0 font-semibold text-ink">{finding.summary}</p>
      </div>
      <EvidenceBlock text={finding.evidence} label="Evidence" highlight={Object.keys(notes).map(Number)} notes={notes} fold />
      <section className="flex flex-col gap-2">
        <h3 className="m-0 text-small font-semibold text-ink-muted">What you can do</h3>
        <p className="m-0 max-w-[56ch] text-ink">{finding.action}</p>
        {finding.command && <EvidenceBlock text={finding.command} label="Command (Bigman does not run it)" />}
      </section>
      <AboutRule id={finding.rule} />
    </div>
  );
}

/** What a package says about itself, from the "Key: value" lines its inspector writes. */
function packageIdentity(report: Report) {
  if (report.kind !== "Package file" && report.kind !== "Package archive") return [];
  return report.details
    .split("\n")
    .map((line) => /^([A-Z][A-Za-z]+): (.+)$/.exec(line))
    .filter((m): m is RegExpExecArray => m !== null)
    .map(([, label = "", value = ""]) => ({ label, value, mono: label === "Homepage" }));
}

/**
 * The report's answer first: its most severe finding and what to do about it,
 * so the decision does not wait for a click into the table.
 */
function LeadFinding({ finding, onOpen }: { finding: Finding; onOpen: () => void }) {
  return (
    <section aria-label="Most important finding" className="flex flex-col gap-1.5">
      <div className="flex items-start gap-3">
        <SeverityBadge severity={finding.severity} />
        <p className="m-0 font-semibold text-ink">{finding.summary}</p>
      </div>
      <p className="m-0 max-w-[56ch] text-ink">
        {finding.action}{" "}
        <button type="button" onClick={onOpen} className="cursor-pointer rounded-control text-ink underline underline-offset-2 hover:text-ink-muted">
          See the evidence
        </button>
      </p>
    </section>
  );
}

/**
 * A report's levels two and three. `inline` when a heading above already says
 * the result; `subjectShown` when the page also shows what was looked at
 * (Inspect's path bar), so neither is repeated.
 */
export function ReportPanel({
  report,
  now,
  inline = false,
  subjectShown = false,
}: {
  report: Report;
  now: number;
  inline?: boolean;
  subjectShown?: boolean;
}) {
  const push = useDrilldown((s) => s.push);
  const status = reportStatus(report);
  const findings = report.findings.map((f, index) => ({ ...f, index }));
  const lead = findings
    .filter((f) => f.severity !== "info")
    .sort((a, b) => SEVERITY_RANK[b.severity] - SEVERITY_RANK[a.severity])[0];
  const identity = packageIdentity(report);
  const openFinding = (f: Finding) => push({ title: f.summary, content: <FindingEvidence finding={f} report={report} /> });
  return (
    <div className="flex flex-col gap-4">
      {!(inline && subjectShown) && (
        <KeyValue
          items={[
            ...(inline ? [] : [{ label: "Result", value: verifiesDetection(report) && status !== "Incomplete" ? "Detection verified" : status }]),
            ...(subjectShown ? [] : [{ label: "Looked at", value: report.subject, mono: /^(\/|https:\/\/)/.test(report.subject) }]),
          ]}
        />
      )}
      {identity.length > 0 && <KeyValue items={identity} />}
      {lead && <LeadFinding finding={lead} onOpen={() => openFinding(lead)} />}
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
          onOpen={openFinding}
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
      <KeyValue
        items={[
          { label: "Ran at", value: `${formatTime(report.created)}, ${ago(report.created, now)}` },
          ...(report.sha256 ? [{ label: "SHA-256", value: report.sha256, mono: true }] : []),
        ]}
      />
    </div>
  );
}
