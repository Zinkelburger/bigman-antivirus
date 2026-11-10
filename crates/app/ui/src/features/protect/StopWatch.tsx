// Protect > Running > Stop a program and watch: you choose one of your programs,
// Bigman asks it to stop and watches for a minute in the background for anything
// that starts it again, and records what did. Bigman never stops a program on
// its own; every stop goes through a preview. A stop nothing undid can be
// started again.
import { useQueryClient } from "@tanstack/react-query";
import type { ColumnDef } from "@tanstack/react-table";
import { useEffect, useRef, useState } from "react";
import {
  ActionButton,
  ActionDialog,
  ago,
  DataTable,
  IncompleteNotice,
  KeyValue,
  nowSeconds,
  openActionDialog,
  ReportPanel,
  SearchField,
  SeverityBadge,
  SEVERITY_RANK,
  Skeleton,
  useDrilldown,
} from "@/design";
import { useHistory, usePrograms, useReport, useStops, type Program, type Report, type Startable, type Watching } from "@/ipc";

export const STOP_AND_WATCH = "Stop and watch";

/**
 * Where a program runs, in words: "app-org.kde.konsole-1234.scope" is "konsole",
 * "syncthing.service" is "syncthing service".
 */
export function unitLabel(unit: string | null): string {
  if (!unit) return "";
  const app = /^app-(?:[a-z0-9]+\.)*?([^.@]+?)(?:-\d+)?(?:@[\w-]+)?\.(?:scope|service)$/i.exec(unit);
  if (app?.[1]) return app[1];
  const service = /^(.+?)(?:@[\w-]*)?\.service$/.exec(unit);
  return service?.[1] ? `${service[1]} service` : unit;
}

const ROW_HEIGHT = 64;

const columns = (now: number): ColumnDef<Program>[] => [
  {
    id: "program",
    header: "Program",
    accessorFn: (p) => `${p.name} ${String(p.pid)} ${p.command}`,
    cell: ({ row }) => {
      const p = row.original;
      return (
        <span className="flex min-w-0 flex-col whitespace-normal">
          <span className={`truncate ${p.refused ? "text-ink-muted" : "text-ink"}`}>{p.name}</span>
          <span className="line-clamp-2 text-small text-ink-muted">
            {p.refused ? `Not offered: ${p.refused}` : <span className="font-mono">{`process ${String(p.pid)} · ${p.command}`}</span>}
          </span>
        </span>
      );
    },
  },
  {
    id: "unit",
    header: "Runs in",
    accessorFn: (p) => unitLabel(p.unit),
    cell: ({ row }) => <span className="line-clamp-2 whitespace-normal text-ink-muted">{unitLabel(row.original.unit)}</span>,
    meta: { width: "8rem" },
  },
  {
    id: "started",
    header: "Started",
    accessorFn: (p) => p.started,
    cell: ({ row }) => <span className="whitespace-normal text-ink-muted">{ago(row.original.started, now)}</span>,
    meta: { width: "7rem", align: "right" },
  },
];

/** A program Bigman will not stop: why, and what it is. */
function Refused({ program }: { program: Program }) {
  return (
    <div className="flex flex-col gap-4">
      <p className="m-0 max-w-[56ch] text-pretty text-ink">Bigman does not offer to stop this program. {program.refused}</p>
      <KeyValue
        items={[
          { label: "Program", value: program.exe ?? program.name, mono: true },
          { label: "Process ID", value: String(program.pid), mono: true },
          { label: "Command line", value: program.command, mono: true },
          ...(program.unit ? [{ label: "Runs in", value: program.unit, mono: true }] : []),
        ]}
      />
    </div>
  );
}

/** Opens the stop preview for a program, or says why it is not offered. */
export function useStopProgram() {
  const push = useDrilldown((s) => s.push);
  const client = useQueryClient();
  return (p: Program) => {
    if (p.refused) {
      push({ title: p.name, content: <Refused program={p} /> });
      return;
    }
    openActionDialog({ kind: "stop_and_watch", pid: p.pid, start: p.start }, "Stop and watch…", () => {
      void client.invalidateQueries({ queryKey: ["programs"] });
      void client.invalidateQueries({ queryKey: ["stops"] });
    });
  };
}

/** Your programs, read when the panel opens; choosing one opens its preview. */
function Picker() {
  const programs = usePrograms();
  const stopProgram = useStopProgram();
  const [query, setQuery] = useState("");
  if (programs.isPending) return <Skeleton shape="rows" count={6} />;
  if (programs.isError) return <IncompleteNotice title="Your programs could not be listed" errors={[programs.error.message]} />;
  const list = programs.data;
  const offered = list.filter((p) => !p.refused).length;
  return (
    <div className="flex flex-col gap-3">
      <p className="m-0 max-w-[56ch] text-pretty text-ink">Choose a program to see what stopping it would do. Nothing stops until you confirm.</p>
      <div className="flex flex-wrap items-center justify-between gap-3">
        <span className="text-small text-ink-muted">
          {offered} of {list.length} can be stopped
        </span>
        <div className="flex items-center gap-2">
          <SearchField label="Search your programs" value={query} onChange={setQuery} />
          <ActionButton variant="quiet" disabled={programs.isFetching} onClick={() => void programs.refetch()}>
            Refresh
          </ActionButton>
        </div>
      </div>
      <DataTable
        label="Your programs"
        columns={columns(nowSeconds())}
        data={list}
        getRowId={(p) => `${String(p.pid)}:${String(p.start)}`}
        onOpen={stopProgram}
        filter={query}
        rowHeight={ROW_HEIGHT}
        height={Math.min(560, 34 + list.length * ROW_HEIGHT)}
      />
    </div>
  );
}

/** Seconds left of a watch, from its end time. */
function useSecondsLeft(ends: number): number {
  const [now, setNow] = useState(nowSeconds);
  useEffect(() => {
    const timer = setInterval(() => setNow(nowSeconds()), 1000);
    return () => clearInterval(timer);
  }, []);
  return Math.max(0, ends - now);
}

function WatchingRow({ watch }: { watch: Watching }) {
  const left = useSecondsLeft(watch.ends);
  return (
    <li className="flex flex-wrap items-center gap-x-4 gap-y-1 border-b border-rule py-3 last:border-b-0">
      <span className="text-ink">
        {watch.name} (process {watch.pid})
      </span>
      {/* The count is for the eye; the live region below says when it starts and ends. */}
      <span aria-hidden className="text-small text-ink-muted tabular-nums">
        {left > 0 ? `Watching, ${String(left)} seconds left` : "Finishing…"}
      </span>
    </li>
  );
}

/** Starting a stopped program again, only when it can be done. */
function StartAgain({ startable, describedBy }: { startable: Startable | undefined; describedBy: string }) {
  if (!startable) return null;
  if (startable.running.length > 0) {
    return <span className="text-small text-ink-muted">Running again as process {startable.running.join(", ")}</span>;
  }
  return <ActionDialog action={{ kind: "start_again", report: startable.report }} label="Start it again…" variant="secondary" describedBy={describedBy} />;
}

/** One earlier stop: what it found, and starting the program again. */
function Stopped({ summary, startable, now }: { summary: Report; startable: Startable | undefined; now: number }) {
  const open = useDrilldown((s) => s.open);
  const full = useReport(summary.id).data;
  const lead = [...summary.findings].sort((a, b) => SEVERITY_RANK[b.severity] - SEVERITY_RANK[a.severity])[0];
  const id = `stopped-${summary.id}`;
  return (
    <li className="flex flex-wrap items-center gap-x-4 gap-y-2 border-b border-rule py-3 last:border-b-0">
      <div className="flex min-w-0 flex-1 basis-[20rem] flex-col gap-1">
        <span id={id} className="text-ink">
          {summary.subject}
        </span>
        <span className="flex flex-wrap items-center gap-2 text-small text-ink-muted">
          {lead && lead.severity !== "info" && <SeverityBadge severity={lead.severity} />}
          <span>
            {ago(summary.created, now)}
            {lead ? ` · ${lead.summary}` : ""}
          </span>
        </span>
      </div>
      <div className="flex items-center gap-2">
        {full && (
          <ActionButton
            variant="quiet"
            aria-describedby={id}
            onClick={() => open({ title: `Stop and watch: ${full.subject}`, content: <ReportPanel report={full} now={nowSeconds()} /> })}
          >
            Details
          </ActionButton>
        )}
        <StartAgain startable={startable} describedBy={id} />
      </div>
    </li>
  );
}

/** What screen readers hear: a watch starting, and the minute ending. */
function useWatchAnnouncement(watching: Watching[]): string {
  const client = useQueryClient();
  const previous = useRef<Watching[]>([]);
  const [said, setSaid] = useState("");
  useEffect(() => {
    const was = previous.current;
    previous.current = watching;
    const started = watching.filter((w) => !was.some((x) => x.pid === w.pid));
    const ended = was.filter((w) => !watching.some((x) => x.pid === w.pid));
    if (ended.length) {
      // The report is in history now.
      void client.invalidateQueries({ queryKey: ["history"] });
      setSaid(`The watch of ${ended.map((w) => w.name).join(", ")} ended; the result is under Stopped recently.`);
    } else if (started.length) {
      setSaid(`Watching ${started.map((w) => w.name).join(", ")} for a minute.`);
    }
  }, [watching, client]);
  return said;
}

const NO_WATCHES: Watching[] = [];

export function StopWatch() {
  const open = useDrilldown((s) => s.open);
  const stops = useStops().data;
  const watching = stops?.watching ?? NO_WATCHES;
  const recent = (useHistory().data ?? []).filter((r) => r.kind === STOP_AND_WATCH).slice(0, 5);
  const announcement = useWatchAnnouncement(watching);
  const now = nowSeconds();
  return (
    <section aria-labelledby="stop-watch" className="flex flex-col gap-3 border-t border-rule pt-5">
      <h2 id="stop-watch" className="m-0 text-body font-semibold text-ink">
        Stop a program and watch
      </h2>
      <p className="m-0 max-w-[56ch] text-pretty text-ink-muted">
        If a program keeps coming back, stop it here. Bigman asks it to stop, watches for a minute and records which program or service started it again.
        Bigman never stops a program on its own.
      </p>
      <div>
        <ActionButton variant="secondary" onClick={() => open({ title: "Your programs", content: <Picker /> })}>
          Choose a program…
        </ActionButton>
      </div>
      <p className="sr-only" role="status">
        {announcement}
      </p>
      {watching.length > 0 && (
        <section aria-labelledby="stop-watch-now" className="flex flex-col gap-1">
          <h3 id="stop-watch-now" className="m-0 text-small font-semibold text-ink-muted">
            Watching now
          </h3>
          <ul className="m-0 flex list-none flex-col p-0">
            {watching.map((w) => (
              <WatchingRow key={w.pid} watch={w} />
            ))}
          </ul>
        </section>
      )}
      {recent.length > 0 && (
        <section aria-labelledby="stop-watch-recent" className="flex flex-col gap-1">
          <h3 id="stop-watch-recent" className="m-0 text-small font-semibold text-ink-muted">
            Stopped recently
          </h3>
          <ul className="m-0 flex list-none flex-col p-0">
            {recent.map((r) => (
              <Stopped key={r.id} summary={r} startable={stops?.startable.find((s) => s.report === r.id)} now={now} />
            ))}
          </ul>
        </section>
      )}
    </section>
  );
}
