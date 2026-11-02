import type { ColumnDef } from "@tanstack/react-table";
import {
  ActionButton,
  DataTable,
  EventPanel,
  eventOccurrence,
  fold,
  ago,
  IncompleteNotice,
  KeyValue,
  nowSeconds,
  PageHeader,
  plural,
  ProgressRow,
  ReportById,
  ReportPanel,
  reportSeverity,
  reportStatus,
  Select,
  signatureAge,
  Skeleton,
  StatusLine,
  StatusTile,
  Timeline,
  useDrilldown,
  useToasts,
  type TileState,
} from "@/design";
import {
  DisconnectedError,
  useCheck,
  useEvents,
  useStatus,
  useUnread,
  type Event,
  type ProtectionStatus,
  type ToolState,
  type Unread,
} from "@/ipc";
import { useShell, type Range } from "@/shell/store";

const STALE_DAYS = 3;
const RANGES = [
  { value: "24h", label: "Last 24 hours" },
  { value: "7d", label: "Last 7 days" },
] as const satisfies readonly { value: Range; label: string }[];
const RANGE_SECONDS: Partial<Record<Range, number>> = { "24h": 86_400, "7d": 7 * 86_400 };

const toolColumns: ColumnDef<ToolState>[] = [
  { accessorKey: "name", header: "Tool", meta: { mono: true, width: "10rem" } },
  { accessorKey: "purpose", header: "Adds" },
  { id: "available", header: "Installed", accessorFn: (t) => (t.available ? "Yes" : "No"), meta: { width: "6rem" } },
];

/** Readiness of Bigman's own tools. Not a finding about the machine, so never a severity. */
function readiness(status: ProtectionStatus, now: number) {
  const installed = status.tools.filter((t) => t.available).length;
  const signatures = status.signatures;
  const fresh = signatures !== null && now - signatures.updated <= STALE_DAYS * 86_400;
  // Name the one thing to do, not a vague "needs setup".
  const value = !signatures
    ? "No signatures yet"
    : !fresh
      ? "Signatures out of date"
      : !status.monitor
        ? "Tripwires off"
        : "Ready";
  const age = signatures ? `Signatures ${signatureAge(signatures.updated, now)}` : "File scans need ClamAV signatures";
  return { value, detail: `${age} · ${installed} of ${status.tools.length} optional tools`, installed };
}

function ToolsPanel({ status, now }: { status: ProtectionStatus; now: number }) {
  const r = readiness(status, now);
  return (
    <div className="flex flex-col gap-4">
      <p className="m-0 max-w-[56ch] text-ink-muted">
        These are Bigman&apos;s own tools, not findings about your computer. Missing tools only limit the checks that
        need them.
      </p>
      <KeyValue
        items={[
          {
            label: "Canary tripwires",
            value: status.monitor
              ? `Watched: ${status.monitor.watched} of ${plural(status.monitor.configured, "file")}`
              : "Not watched. Reads of decoy files are not noticed.",
          },
          {
            label: "Malware signatures",
            value: status.signatures
              ? `${signatureAge(status.signatures.updated, now)}, in ${status.signatures.location}`
              : "None yet. File scans need ClamAV signatures.",
          },
          { label: "Optional tools", value: `${r.installed} of ${status.tools.length} installed` },
        ]}
      />
      <DataTable
        label="Optional tools"
        columns={toolColumns}
        data={status.tools}
        getRowId={(t) => t.name}
        initialSorting={[{ id: "available", desc: false }]}
        height={420}
      />
    </div>
  );
}

/** The one sentence the eye lands on: what to look at, or the calm status line. */
function ResultLine({ events, status, rangeLabel, now }: { events: Event[]; status: ProtectionStatus; rangeLabel: string; now: number }) {
  const open = useDrilldown((s) => s.open);
  const toReview = events.filter((e) => e.severity !== "info");
  const scan = status.last_scan;
  const incomplete = scan && reportStatus(scan) === "Incomplete" ? scan : null;
  const first = toReview[0];
  return (
    <>
      {first && (
        <div className="flex flex-wrap items-center gap-x-3 gap-y-2">
          <p className="m-0 text-body font-semibold text-ink">
            Findings to review: {plural(toReview.length, "entry")} from the {rangeLabel.toLowerCase()}.
          </p>
          <ActionButton variant="quiet" onClick={() => open({ title: first.summary, content: <EventPanel event={first} /> })}>
            Open the latest
          </ActionButton>
        </div>
      )}
      {incomplete && (
        <IncompleteNotice
          title={`Your last scan of ${incomplete.subject} is incomplete`}
          coverage={incomplete.coverage}
          errors={incomplete.errors}
        />
      )}
      {!first && !incomplete && scan && (
        <StatusLine ranAt={scan.created} signaturesUpdated={status.signatures?.updated} now={now} />
      )}
    </>
  );
}

/** "Since last look": first sightings and findings since Activity was marked as seen. */
function sinceLastLook(unread: Unread | undefined) {
  if (!unread) return { value: undefined, detail: undefined };
  const { first_seen: fresh, to_review: review } = unread;
  if (fresh + review === 0) return { value: "Nothing new", detail: "Everything in Activity has been seen." };
  return {
    value: fresh > 0 ? plural(fresh, "first sighting") : plural(review, "finding"),
    detail: review > 0 ? `${plural(review, "entry")} to review since you last looked` : "New since you last marked Activity as seen.",
  };
}

/** Days are loaded from the start of an hour so the query stays put as time passes. */
const hourFloor = (t: number) => Math.floor(t / 3_600) * 3_600;

export function OverviewPage() {
  const status = useStatus();
  const unread = useUnread();
  const doctor = useCheck();
  const open = useDrilldown((s) => s.open);
  const toast = useToasts((s) => s.push);
  const { range, setRange, openUnread } = useShell();
  const now = nowSeconds();

  const overviewRange: Range = range === "7d" ? "7d" : "24h";
  const rangeLabel = RANGES.find((r) => r.value === overviewRange)?.label ?? "Last 24 hours";
  const since = now - (RANGE_SECONDS[overviewRange] ?? 86_400);
  const events = useEvents(hourFloor(since));
  const disconnected = status.error instanceof DisconnectedError || events.error instanceof DisconnectedError;
  const tileState: TileState = disconnected ? "disconnected" : status.isPending ? "loading" : status.isError ? "incomplete" : "ready";
  const ready = tileState === "ready";
  const data = status.data;
  const tools = data ? readiness(data, now) : null;
  const scan = data?.last_scan ?? null;
  const scanStatus = scan ? reportStatus(scan) : null;
  const scanSeverity = scan ? reportSeverity(scan) : null;
  const inRange = (events.data ?? []).filter((e) => e.at >= since);
  const byId = new Map(inRange.map((e) => [String(e.id), e]));
  // The store's counts carry repeats from before the range, so folding needs only the range.
  const entries = fold(inRange.map(eventOccurrence));
  const look = sinceLastLook(unread.data);

  const runCheckup = () =>
    doctor.mutate({ type: "doctor" }, {
      onSuccess: (report) => {
        toast("Checkup finished.");
        open({ title: "Checkup", content: <ReportPanel report={report} now={nowSeconds()} /> });
      },
    });

  return (
    <>
      <PageHeader
        title="Overview"
        controls={<Select label="Time range" hideLabel value={overviewRange} options={RANGES} onChange={setRange} />}
        action={
          <ActionButton variant="primary" busy={doctor.isPending} disabled={disconnected} onClick={runCheckup}>
            Run checkup
          </ActionButton>
        }
      />
      <div className="flex flex-col gap-5 px-6 py-5">
        {disconnected && (
          <IncompleteNotice
            kind="disconnected"
            errors={[status.error?.message ?? events.error?.message ?? ""]}
            action={
              // The desktop shell starts bigmand when a request cannot connect.
              <ActionButton onClick={() => void Promise.all([status.refetch(), events.refetch()])}>
                Start background checks
              </ActionButton>
            }
          />
        )}
        {status.isError && !disconnected && (
          <IncompleteNotice title="Bigman could not report its status" errors={[status.error.message]} />
        )}
        {doctor.isPending && (
          <ProgressRow
            label="Running the checkup"
            detail="Read-only: tools, signatures, canaries and listening programs."
            progress={null}
            onCancel={doctor.cancel}
          />
        )}
        {doctor.isError && <IncompleteNotice title="The checkup did not finish" errors={[doctor.error.message]} />}
        {data && events.data && <ResultLine events={inRange} status={data} rangeLabel={rangeLabel} now={now} />}

        <section aria-label="Status" className="grid grid-cols-4 gap-3">
          <StatusTile
            label="Since last look"
            state={!ready ? tileState : unread.isPending ? "loading" : unread.isError ? "incomplete" : look.value === "Nothing new" ? "empty" : "ready"}
            value={look.value}
            detail={look.detail}
            onOpen={unread.data && look.value !== "Nothing new" ? openUnread : undefined}
          />
          <StatusTile
            label="Bigman's tools"
            state={tileState}
            value={tools?.value}
            detail={tools?.detail}
            onOpen={data ? () => open({ title: "Bigman's tools", content: <ToolsPanel status={data} now={now} /> }) : undefined}
          />
          <StatusTile
            label="Last scan"
            state={ready && !scan ? "empty" : tileState}
            value={
              !scan
                ? "No scans yet"
                : scanStatus === "Findings to review"
                  ? plural(scan.findings.length, "finding")
                  : scanStatus === "Incomplete"
                    ? "Incomplete"
                    : "No findings"
            }
            detail={
              !ready
                ? undefined
                : !scan
                  ? "Scans of Downloads appear here."
                  : scanStatus === "Incomplete"
                    ? `${plural(scan.coverage.skipped.length + scan.errors.length, "item")} not checked · ${ago(scan.created, now)}`
                    : `${scan.subject} · ${ago(scan.created, now)}`
            }
            severity={scanSeverity === "info" ? null : scanSeverity}
            onOpen={scan ? () => open({ title: "Last scan", content: <ReportById id={scan.id} /> }) : undefined}
          />
          <StatusTile
            label="Updates"
            state={ready ? "empty" : tileState}
            value="Not checked"
            detail={ready ? "Pending security updates and the CVEs they fix will be listed here." : undefined}
          />
        </section>

        <section aria-labelledby="life-list" className="flex flex-col gap-3">
          <h2 id="life-list" className="m-0 text-body font-semibold text-ink">
            {rangeLabel}
          </h2>
          {disconnected ? (
            <p className="m-0 text-body text-ink-muted">Entries appear once background checks are running again.</p>
          ) : events.isPending ? (
            <Skeleton shape="rows" count={4} />
          ) : (
            <Timeline
              label={rangeLabel}
              entries={entries}
              now={now}
              height={560}
              onOpen={(entry) => {
                const event = byId.get(entry.id);
                if (event) open({ title: entry.summary, content: <EventPanel event={event} /> });
              }}
              empty={
                <>
                  <p className="m-0 max-w-[56ch]">
                    New connections, listeners, package installs and every check you run are entered here the first
                    time they are seen; repeats fold under them.
                  </p>
                  <ActionButton disabled={doctor.isPending} onClick={runCheckup}>
                    Run checkup
                  </ActionButton>
                </>
              }
            />
          )}
        </section>
      </div>
    </>
  );
}
