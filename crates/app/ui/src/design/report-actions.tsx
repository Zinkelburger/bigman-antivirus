// What a reader can do with a report, wherever it opens (the tab that ran it,
// Activity, a recent list): the same follow-ups for the same report.
import type { Quarantined, Report, ScanItem } from "@/ipc";
import { useQuarantined, useScanItems } from "@/ipc";
import { ActionDialog } from "./action";
import { EvidenceBlock } from "./components/EvidenceBlock";
import { KeyValue } from "./components/KeyValue";

/** The quarantine entry holding this scanned file, if it is still in quarantine. */
export function heldIn(quarantine: readonly Quarantined[] | undefined, item: ScanItem): Quarantined | undefined {
  return quarantine?.find((q) => !q.restored && q.original === item.path && q.hash === item.hash);
}

/** Files a scan flagged, each with its own previewed quarantine, or restore once quarantined. */
function FlaggedFiles({ report }: { report: string }) {
  const items = useScanItems(report);
  const quarantine = useQuarantined();
  const flagged = (items.data ?? []).map((item, index) => ({ item, index })).filter(({ item }) => item.detected);
  if (flagged.length === 0) return null;
  return (
    <section aria-label="Flagged files" className="flex flex-col gap-3">
      <h3 className="m-0 text-small font-semibold text-ink-muted">Flagged files</h3>
      {flagged.map(({ item, index }) => (
        <div key={item.hash + item.path} className="flex flex-col gap-3 rounded-panel border border-rule p-3">
          <KeyValue
            items={[
              { label: "File", value: item.path, mono: true },
              { label: "Signature", value: item.signature, mono: true },
              { label: "SHA-256", value: item.hash, mono: true },
            ]}
          />
          <QuarantineControl report={report} index={index} held={heldIn(quarantine.data, item)} />
        </div>
      ))}
    </section>
  );
}

function QuarantineControl({ report, index, held }: { report: string; index: number; held: Quarantined | undefined }) {
  if (held) {
    return (
      <div className="flex flex-wrap items-center gap-3">
        <span className="text-small text-ink-muted">In quarantine; a recovery copy is kept.</span>
        <ActionDialog action={{ kind: "restore", id: held.id }} label="Restore…" variant="quiet" />
      </div>
    );
  }
  return (
    <div>
      <ActionDialog action={{ kind: "quarantine", report, index }} label="Quarantine…" />
    </div>
  );
}

/**
 * Level three for an installer: the exact script, every line a rule matched
 * highlighted in its severity's color with the rule IDs beside it.
 */
function InstallerSource({ report }: { report: Report }) {
  if (!report.details) return null;
  const lines = report.details.split("\n").length;
  const notes: Record<number, { text: string; tone: "review" | "high" }> = {};
  let outside = 0;
  for (const finding of report.findings) {
    const line = Number(/^Line (\d+):/.exec(finding.evidence)?.[1] ?? 0);
    if (!line || finding.severity === "info") continue;
    if (line > lines) {
      outside += 1;
      continue;
    }
    const existing = notes[line];
    notes[line] = {
      text: existing ? `${existing.text}, ${finding.rule}` : finding.rule,
      tone: existing?.tone === "high" || finding.severity === "high" ? "high" : "review",
    };
  }
  return (
    <section aria-label="The script" className="flex flex-col gap-2">
      <EvidenceBlock label="The script, as saved" text={report.details} highlight={Object.keys(notes).map(Number)} notes={notes} />
      {outside > 0 && (
        <p className="m-0 text-small text-ink-muted">
          {outside} finding line numbers fall outside the saved text and are not highlighted.
        </p>
      )}
    </section>
  );
}

export function ReportFollowUp({ report }: { report: Report }) {
  switch (report.kind) {
    case "File scan":
      return <FlaggedFiles report={report.id} />;
    case "Installer inspection":
      return (
        <>
          <InstallerSource report={report} />
          {report.sha256 && (
            <div>
              <ActionDialog action={{ kind: "rehearse", report: report.id, sha256: report.sha256 }} label="Rehearse in the offline sandbox…" />
            </div>
          )}
        </>
      );
    case "Project inspection":
    case "Package script inventory":
      return report.subject.endsWith(".tgz") || report.subject.endsWith(".gz") ? null : (
        <div>
          <ActionDialog action={{ kind: "package_policy", path: report.subject }} label="Disable automatic install scripts…" />
        </div>
      );
    case "Security updates":
      return report.details ? (
        <section className="flex flex-col gap-2">
          <h3 className="m-0 text-small font-semibold text-ink-muted">Install all security updates</h3>
          <EvidenceBlock text={report.details} label="Command (Bigman does not run it)" />
          <p className="m-0 max-w-[56ch] text-small text-ink-muted">
            Your desktop&apos;s software updater (GNOME Software or Discover) installs the same updates.
          </p>
        </section>
      ) : null;
    case "PKGBUILD check":
    case "Launcher check":
      return report.details ? (
        <EvidenceBlock text={report.details} label={report.kind === "PKGBUILD check" ? "The PKGBUILD, as read" : "The launcher, as read"} />
      ) : null;
    case "Persistence changes":
      return report.errors.length === 0 ? (
        <div>
          <ActionDialog action={{ kind: "accept_baseline", report: report.id }} label="Accept as baseline…" />
        </div>
      ) : null;
    default:
      return null;
  }
}
