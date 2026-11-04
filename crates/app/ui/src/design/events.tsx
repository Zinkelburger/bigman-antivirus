// Timeline events from the daemon as life-list occurrences, and the drill-down
// for one event. Any SourceId renders; a new source needs no change here.
import {
  BellRing,
  Box,
  Download,
  FolderDown,
  FolderGit2,
  Globe,
  KeyRound,
  ListChecks,
  Package,
  Puzzle,
  Radio,
  Rocket,
  ShieldAlert,
  ShieldBan,
  Sparkles,
  Upload,
  type LucideIcon,
} from "lucide-react";
import { useRules, type Event, type SourceId } from "@/ipc";
import { EvidenceBlock } from "./components/EvidenceBlock";
import { KeyValue } from "./components/KeyValue";
import { SeverityBadge } from "./components/SeverityBadge";
import { formatDate, formatTime, plural } from "./format";
import { AboutRule } from "./report";
import { ReceiptById } from "./receipt";
import { ReportById } from "./report-by-id";
import type { Occurrence } from "./timeline";

/** Chip and drill-down names. A source without one shows its id. */
const SOURCE_LABEL: Partial<Record<SourceId, string>> = {
  network: "Network",
  listener: "Listeners",
  receipt: "Packages",
  startup: "Startup",
  canary: "Canaries",
  denial: "Denials",
  flatpak: "Flatpak",
  container: "Containers",
  scan: "Checks",
  download: "Downloads",
  project: "Projects",
  advisory: "Advisories",
  extension: "Extensions",
  import: "Imported",
  ssh: "SSH",
  skill: "AI skills",
};

const SOURCE_ICON: Partial<Record<SourceId, LucideIcon>> = {
  network: Globe,
  listener: Radio,
  receipt: Package,
  startup: Rocket,
  canary: BellRing,
  denial: ShieldBan,
  flatpak: Box,
  container: Box,
  scan: ListChecks,
  download: FolderDown,
  project: FolderGit2,
  advisory: ShieldAlert,
  extension: Puzzle,
  import: Download,
  ssh: KeyRound,
  skill: Sparkles,
};

export function sourceLabel(source: SourceId): string {
  return SOURCE_LABEL[source] ?? source;
}

export function eventOccurrence(e: Event): Occurrence {
  return {
    id: String(e.id),
    key: e.key,
    at: e.at,
    summary: e.summary,
    source: sourceLabel(e.source),
    icon: SOURCE_ICON[e.source] ?? Upload,
    severity: e.severity,
    seenBefore: e.seen_before,
    firstAt: e.first_at,
  };
}

/** One event's evidence: its report when it has one, otherwise what was seen. */
export function EventPanel({ event }: { event: Event }) {
  const rule = useRules().data?.find((r) => r.id === event.rule);
  if (event.link?.kind === "report") return <ReportById id={event.link.id} />;
  if (event.link?.kind === "receipt") return <ReceiptById id={event.link.id} />;
  return (
    <div className="flex flex-col gap-4">
      {/* The panel title is the entry's sentence; the badge says how it is rated. */}
      {event.severity !== "info" && (
        <div>
          <SeverityBadge severity={event.severity} />
        </div>
      )}
      <KeyValue
        items={[
          { label: "Noticed by", value: sourceLabel(event.source) },
          { label: "When", value: `${formatDate(event.at)}, ${formatTime(event.at)}` },
          {
            label: "First seen",
            value:
              event.seen_before === 0
                ? "This time"
                : `${formatDate(event.first_at)}; seen ${plural(event.seen_before, "time")} before`,
          },
        ]}
      />
      {event.evidence && <EvidenceBlock text={event.evidence} label="Evidence" />}
      {rule && (
        <section>
          <h3 className="m-0 mb-1 text-small font-semibold text-ink-muted">What you can do</h3>
          <p className="m-0 max-w-[56ch] text-ink">{rule.remediation}</p>
        </section>
      )}
      {event.rule && <AboutRule id={event.rule} />}
    </div>
  );
}
