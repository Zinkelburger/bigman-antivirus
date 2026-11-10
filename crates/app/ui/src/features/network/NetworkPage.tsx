import type { ColumnDef } from "@tanstack/react-table";
import { useMemo } from "react";
import {
  ActionButton,
  ago,
  CoverageList,
  DaemonGate,
  DataTable,
  EmptyState,
  FindingEvidence,
  FirstSeenMarker,
  IncompleteNotice,
  KeyValue,
  maxSeverity,
  nowSeconds,
  PageHeader,
  plural,
  SEVERITY_RANK,
  SeverityBadge,
  Skeleton,
  useDrilldown,
} from "@/design";
import { useNetwork, type ConnectionRow, type ListenerRow } from "@/ipc";

/** Table wording for the daemon's reach sentence; the drill-down keeps the full one. */
const REACH_SHORT: Partial<Record<string, string>> = {
  "this computer only": "This computer",
  "one network interface": "One network",
  "every network interface": "Any network",
};
/** A connection or listener first seen this recently is marked as a first sighting. */
const RECENT = 86_400;

const program = (row: { socket: ListenerRow["socket"] }) =>
  row.socket.process?.name ?? (row.socket.uid === 0 ? "A system program (root)" : "A program run by another account");

function FirstSeen({ at, now }: { at: number | null; now: number }) {
  if (at === null) return <span className="text-ink-muted">Not recorded</span>;
  return (
    <span className="inline-flex items-center gap-2">
      {now - at < RECENT && <FirstSeenMarker compact />}
      {ago(at, now)}
    </span>
  );
}

function connectionColumns(now: number): ColumnDef<ConnectionRow>[] {
  return [
    { id: "program", header: "Program", accessorFn: program, meta: { width: "11rem" } },
    { id: "remote", header: "Connected to", accessorFn: (c) => c.socket.remote ?? "", meta: { mono: true } },
    { accessorKey: "range", header: "Address range", meta: { mono: true, width: "10rem" } },
    {
      id: "first",
      header: "First seen",
      accessorFn: (c) => c.first_at ?? 0,
      cell: ({ row }) => <FirstSeen at={row.original.first_at} now={now} />,
      meta: { width: "10.5rem" },
    },
  ];
}

function listenerColumns(now: number): ColumnDef<ListenerRow>[] {
  return [
    {
      id: "severity",
      header: "Severity",
      accessorFn: (l) => {
        const worst = maxSeverity(l.findings.map((f) => f.severity));
        return worst ? SEVERITY_RANK[worst] : -1;
      },
      cell: ({ row }) => {
        const worst = maxSeverity(row.original.findings.map((f) => f.severity));
        return worst ? <SeverityBadge severity={worst} /> : <span className="text-ink-muted">No findings</span>;
      },
      meta: { width: "7rem" },
    },
    { id: "program", header: "Program", accessorFn: program },
    { id: "local", header: "Listening on", accessorFn: (l) => l.socket.local, meta: { mono: true, width: "11rem" } },
    { id: "reach", header: "Reachable from", accessorFn: (l) => REACH_SHORT[l.reach] ?? l.reach, meta: { width: "8.5rem" } },
    {
      id: "first",
      header: "First seen",
      accessorFn: (l) => l.first_at ?? 0,
      cell: ({ row }) => <FirstSeen at={row.original.first_at} now={now} />,
      meta: { width: "10.5rem" },
    },
  ];
}

function ListenerPanel({ listener, now }: { listener: ListenerRow; now: number }) {
  const p = listener.socket.process;
  return (
    <div className="flex flex-col gap-4">
      <KeyValue
        items={[
          { label: "Program", value: p ? `${p.name} (pid ${String(p.pid)})` : "Run by another user; the optional helper can name it" },
          ...(p?.exe ? [{ label: "Executable", value: p.exe, mono: true }] : []),
          { label: "Listening on", value: listener.socket.local, mono: true },
          { label: "Reachable from", value: listener.reach },
          { label: "First seen", value: listener.first_at === null ? "Not recorded" : ago(listener.first_at, now) },
        ]}
      />
      {listener.findings.length === 0 ? (
        <p className="m-0 max-w-[56ch] text-ink-muted">
          No listener rule applies: it is reachable from {listener.reach} and is not a known risky service.
        </p>
      ) : (
        // Few findings per listener, each with its evidence and what to do, worst first.
        [...listener.findings]
          .sort((a, b) => SEVERITY_RANK[b.severity] - SEVERITY_RANK[a.severity])
          .map((f) => (
            <section key={f.rule} className="border-t border-rule pt-4">
              <FindingEvidence finding={f} />
            </section>
          ))
      )}
    </div>
  );
}

/** The one sentence the eye lands on: which listener to look at, or that none needs it. */
function ExposureLine({ listeners, onOpen }: { listeners: ListenerRow[]; onOpen: (l: ListenerRow) => void }) {
  const flagged = listeners.filter((l) => l.findings.some((f) => f.severity !== "info"));
  const first = flagged[0];
  if (!first) {
    return (
      <p className="m-0 flex items-center gap-2 text-body text-ink">
        <span aria-hidden className="size-2 shrink-0 rounded-full bg-calm" />
        No findings from the listener rules. {plural(listeners.length, "program")} listening right now.
      </p>
    );
  }
  return (
    <div className="flex flex-wrap items-center gap-x-3 gap-y-2">
      <p className="m-0 text-body font-semibold text-ink">
        Findings to review: {plural(flagged.length, "listening program")}.
      </p>
      <ActionButton variant="quiet" onClick={() => onOpen(first)}>
        Open {program(first)} on port {first.socket.local.split(":").at(-1)}
      </ActionButton>
    </div>
  );
}

function ConnectionPanel({ connection, now }: { connection: ConnectionRow; now: number }) {
  const p = connection.socket.process;
  return (
    <div className="flex flex-col gap-4">
      <KeyValue
        items={[
          { label: "Program", value: p ? `${p.name} (pid ${String(p.pid)})` : "Unknown" },
          ...(p?.exe ? [{ label: "Executable", value: p.exe, mono: true }] : []),
          { label: "Connected to", value: connection.socket.remote ?? "", mono: true },
          { label: "From", value: connection.socket.local, mono: true },
          { label: "Address range", value: connection.range, mono: true },
          {
            label: "First seen",
            value: connection.first_at === null ? "Not recorded" : `${ago(connection.first_at, now)}, for this program and range`,
          },
        ]}
      />
      <p className="m-0 max-w-[56ch] text-ink-muted">
        Bigman notes the first time each program reaches an address range; that entry is in Activity. A connection is not a
        finding by itself.
      </p>
    </div>
  );
}

export function NetworkPage() {
  const network = useNetwork();
  const open = useDrilldown((s) => s.open);
  const now = nowSeconds();
  const connections = useMemo(() => connectionColumns(now), [now]);
  const listeners = useMemo(() => listenerColumns(now), [now]);
  const data = network.data;

  return (
    <>
      <PageHeader title="Network" />
      <div className="flex flex-col gap-6 px-6 py-5">
        <DaemonGate>
          {data && <ExposureLine listeners={data.listeners} onOpen={(l) => open({ title: `${program(l)} on port ${l.socket.local.split(":").at(-1) ?? ""}`, content: <ListenerPanel listener={l} now={now} /> })} />}
          {network.isError ? (
            <IncompleteNotice title="Connections could not be listed" errors={[network.error.message]} />
          ) : !data ? (
            <Skeleton shape="rows" count={6} />
          ) : (
            <>
              <section aria-labelledby="listeners" className="flex flex-col gap-3">
                <h2 id="listeners" className="m-0 text-body font-semibold text-ink">
                  Listening programs
                </h2>
                {data.listeners.length === 0 ? (
                  <EmptyState>No program is listening for connections.</EmptyState>
                ) : (
                  <DataTable
                    label="Listening programs"
                    columns={listeners}
                    data={data.listeners}
                    getRowId={(l) => `${l.socket.local}-${String(l.socket.process?.pid ?? l.socket.uid)}`}
                    initialSorting={[{ id: "severity", desc: true }]}
                    onOpen={(l) => open({ title: `${program(l)} on port ${l.socket.local.split(":").at(-1) ?? ""}`, content: <ListenerPanel listener={l} now={now} /> })}
                    height={300}
                  />
                )}
              </section>

              <section aria-labelledby="connections" className="flex flex-col gap-3">
                <h2 id="connections" className="m-0 text-body font-semibold text-ink">
                  Your programs&apos; connections
                </h2>
                {data.connections.length === 0 ? (
                  <EmptyState>None of your programs has a connection to another computer open right now.</EmptyState>
                ) : (
                  <DataTable
                    label="Your programs' connections"
                    columns={connections}
                    data={data.connections}
                    getRowId={(c) => `${String(c.socket.process?.pid ?? 0)}-${c.socket.local}-${c.socket.remote ?? ""}`}
                    initialSorting={[{ id: "first", desc: true }]}
                    onOpen={(c) => open({ title: `${program(c)} to ${c.socket.remote ?? ""}`, content: <ConnectionPanel connection={c} now={now} /> })}
                    height={320}
                  />
                )}
              </section>
              <CoverageList coverage={data.coverage} />
            </>
          )}
        </DaemonGate>
      </div>
    </>
  );
}
