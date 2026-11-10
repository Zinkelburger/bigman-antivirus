import type { ColumnDef } from "@tanstack/react-table";
import { useMemo } from "react";
import {
  ActionButton,
  CoverageList,
  DaemonGate,
  DataTable,
  EmptyState,
  formatBytes,
  formatCount,
  IncompleteNotice,
  KeyValue,
  nowSeconds,
  PageHeader,
  Select,
  seriesColor,
  ShareKey,
  ShareList,
  Skeleton,
  StackedAreaChart,
  ThroughputChart,
  unrecorded,
  useDrilldown,
  type KeyValueItem,
  type ShareRow,
} from "@/design";
import { useLiveSamples, useSamples, useTraffic, type Flow, type TrafficReport } from "@/ipc";
import { useShell, type Range } from "@/shell/store";
import { buildSeries, byTraffic, RANGES } from "./throughput";
import {
  chartData,
  hostRows,
  isLocal,
  programKey,
  programLabel,
  programRows,
  serviceLabel,
  slotsFor,
  TOP_ROWS,
  total,
  TRAFFIC_RANGES,
  typeKey,
  typeRows,
} from "./traffic";

const RANGE_OPTIONS = (Object.keys(RANGES) as Range[]).map((value) => ({ value, label: RANGES[value].label }));

type Kind = "program" | "address" | "type";

const bytes = (id: "rx" | "tx", header: string): ColumnDef<Flow> => ({
  id,
  header,
  accessorFn: (f) => f[id],
  cell: ({ row }) => formatBytes(row.original[id]),
  meta: { align: "right", width: "5.5rem" },
});

const flowColumns = (ownUid: number): Record<Kind, ColumnDef<Flow>> => ({
  program: { id: "program", header: "Program", accessorFn: (f) => programLabel(f, ownUid) },
  // Wide enough for any IPv4 address in full; the address is the evidence.
  address: { id: "address", header: "Address", accessorFn: (f) => f.remote, meta: { mono: true, width: "9.5rem" } },
  type: { id: "type", header: "Traffic type", accessorFn: (f) => serviceLabel(f.port, f.inbound) },
});

/** The flows of one report and the account they were collected as. */
interface Flows {
  flows: readonly Flow[];
  ownUid: number;
}

/** One program, address or traffic type opened to the connections behind its figure. */
function FlowsPanel({ kind, row, flows, ownUid }: { kind: Kind; row: ShareRow; flows: Flow[]; ownUid: number }) {
  const columns = useMemo(() => {
    const all = flowColumns(ownUid);
    return [...(["program", "address", "type"] as const).filter((k) => k !== kind).map((k) => all[k]), bytes("rx", "Received"), bytes("tx", "Sent")];
  }, [kind, ownUid]);
  const facts: KeyValueItem[] = [
    { label: "Received", value: formatBytes(row.received) },
    { label: "Sent", value: formatBytes(row.sent) },
    ...(kind === "address" ? [{ label: "Where", value: isLocal(row.id) ? "Your local network" : "The internet" }] : []),
  ];
  return (
    <div className="flex flex-col gap-4">
      <KeyValue items={facts} />
      <DataTable
        label={`Traffic behind ${row.name}`}
        columns={columns}
        data={flows}
        getRowId={(f) => `${programKey(f)}-${f.remote}-${typeKey(f)}`}
        initialSorting={[{ id: "rx", desc: true }]}
        height={320}
      />
      <p className="m-0 max-w-[56ch] text-ink-muted">
        Bigman counts what moved over TCP; a busy program is not a finding by itself. The first time a program reaches a new
        address range is entered in Activity.
      </p>
    </div>
  );
}

const TITLES: Record<Kind, (row: ShareRow) => string> = {
  program: (row) => row.name,
  address: (row) => `Traffic with ${row.title ?? row.name}`,
  type: (row) => row.name,
};
const ALL_TITLE: Record<Kind, string> = { program: "Programs by traffic", address: "Addresses by traffic", type: "Traffic types by amount" };

const MATCH: Record<Kind, (flow: Flow, id: string) => boolean> = {
  program: (f, id) => programKey(f) === id,
  address: (f, id) => f.remote === id,
  type: (f, id) => typeKey(f) === id,
};

const panel = (kind: Kind, row: ShareRow, { flows, ownUid }: Flows) => ({
  title: TITLES[kind](row),
  content: <FlowsPanel kind={kind} row={row} flows={flows.filter((f) => MATCH[kind](f, row.id))} ownUid={ownUid} />,
});

const NAME: Record<Kind, string> = { program: "Program", address: "Address", type: "Traffic type" };

/** Every row of one breakdown as a sortable table; a row opens the connections behind it. */
function AllRows({ kind, rows, flows }: { kind: Kind; rows: ShareRow[]; flows: Flows }) {
  const push = useDrilldown((s) => s.push);
  const columns = useMemo<ColumnDef<ShareRow>[]>(() => {
    const amount = (id: string, header: string, of: (r: ShareRow) => number): ColumnDef<ShareRow> => ({
      id,
      header,
      accessorFn: of,
      cell: ({ row }) => formatBytes(of(row.original)),
      meta: { align: "right", width: "5.5rem" },
    });
    return [
      { id: "name", header: NAME[kind], accessorFn: (r) => r.title ?? r.name, meta: { mono: kind === "address" } },
      ...(kind === "address" ? [{ id: "detail", header: "Mostly", accessorFn: (r: ShareRow) => r.detail ?? "" }] : []),
      amount("total", "Total", (r) => r.received + r.sent),
      amount("received", "Received", (r) => r.received),
      amount("sent", "Sent", (r) => r.sent),
    ];
  }, [kind]);
  return (
    <DataTable
      label={ALL_TITLE[kind]}
      columns={columns}
      data={rows}
      getRowId={(r) => r.id}
      // The page lists by total; the table starts in the same order.
      initialSorting={[{ id: "total", desc: true }]}
      onOpen={(row) => push(panel(kind, row, flows))}
      height={480}
    />
  );
}

function Breakdown({ kind, label, plural, rows, flows }: { kind: Kind; label: string; plural: string; rows: ShareRow[]; flows: Flows }) {
  const open = useDrilldown((s) => s.open);
  return (
    <ShareList
      label={label}
      count={rows.length}
      rows={rows.slice(0, TOP_ROWS)}
      onOpen={(row) => open(panel(kind, row, flows))}
      footer={
        rows.length > TOP_ROWS && (
          // The button's own padding is pulled back so its words start where the rows' do.
          <div className="-ml-1">
            <ActionButton variant="quiet" onClick={() => open({ title: ALL_TITLE[kind], content: <AllRows kind={kind} rows={rows} flows={flows} /> })}>
              Show all {rows.length} {plural}
            </ActionButton>
          </div>
        )
      }
    />
  );
}

/** "Not recorded 13:40–13:55": the times the chart has a break instead of a reading. */
function gapSentence(gaps: string[]): string {
  if (gaps.length === 0) return "";
  return `Not recorded ${gaps.slice(0, 3).join(", ")}${gaps.length > 3 ? ` and ${String(gaps.length - 3)} more times` : ""}`;
}

function ByProgram({ report, ranking, rangeLabel }: { report: TrafficReport; ranking: TrafficReport; rangeLabel: string }) {
  // Hues are dealt from the seven-day ranking, so changing the range never repaints a program.
  const slots = useMemo(() => slotsFor(ranking.programs.map(programKey)), [ranking]);
  const chart = useMemo(() => chartData(report, slots), [report, slots]);
  const programs = useMemo(() => programRows(report, slots), [report, slots]);
  const hosts = useMemo(() => hostRows(report.flows, slots, report.own_uid), [report, slots]);
  const types = useMemo(() => typeRows(report.flows), [report]);
  const flows = useMemo<Flows>(() => ({ flows: report.flows, ownUid: report.own_uid }), [report]);
  const all = total(report.programs);
  const lead = programs[0];
  const gaps = gapSentence(unrecorded(chart.at, chart.recorded, chart.bucket));
  return (
    <>
      {lead && (
        <p className="m-0 text-body text-ink">
          <span className="font-semibold">{lead.name}</span> moved the most data over TCP in the {rangeLabel.toLowerCase()}:{" "}
          {formatBytes(lead.received)} received, {formatBytes(lead.sent)} sent.
        </p>
      )}
      <section aria-labelledby="by-program" className="flex flex-col gap-4">
        <div className="flex flex-col gap-1">
          <h2 id="by-program" className="m-0 text-body font-semibold text-ink">
            By program
          </h2>
          <p className="m-0 text-small text-ink-muted">
            TCP only: {formatBytes(all.rx)} received, {formatBytes(all.tx)} sent. Video calls, streaming over QUIC, VPN tunnels and DNS
            use UDP and are counted only in the interface charts below.
          </p>
        </div>
        <figure className="m-0 flex flex-col gap-2">
          <figcaption className="flex flex-wrap items-center gap-x-4 gap-y-1 text-small text-ink-muted">
            <span>Received and sent together</span>
            {gaps && <span>{gaps}</span>}
            <ul aria-label="Chart colors" className="m-0 ml-auto flex list-none flex-wrap gap-x-3 gap-y-1 p-0">
              {chart.layers.map((layer) => (
                <li key={layer.id} className="inline-flex items-center gap-1.5">
                  <span aria-hidden className="size-2.5 rounded-control" style={{ background: seriesColor(layer.slot) }} />
                  {layer.label}
                </li>
              ))}
            </ul>
          </figcaption>
          <StackedAreaChart
            label="Traffic by program, received and sent together"
            at={chart.at}
            bucket={chart.bucket}
            layers={chart.layers}
            recorded={chart.recorded}
          />
        </figure>
        <div className="flex flex-wrap items-center gap-x-4 gap-y-1 text-small text-ink-muted">
          <ShareKey />
          <span>Bars are drawn against the largest row of their list.</span>
        </div>
        {/* Rows are inset for their hover fill; the pull keeps their text on the page's margin.
            Three lists need about 60rem; a narrower page puts the third beneath the first two. */}
        <div className="@container">
          <div className="-mx-2 grid grid-cols-2 gap-x-4 gap-y-5 @[60rem]:grid-cols-3">
            <Breakdown kind="program" label="Programs" plural="programs" rows={programs} flows={flows} />
            <Breakdown kind="address" label="Connected to" plural="addresses" rows={hosts} flows={flows} />
            <div className="col-span-2 @[60rem]:col-span-1">
              <Breakdown kind="type" label="Traffic type" plural="types" rows={types} flows={flows} />
            </div>
          </div>
        </div>
        {report.flows_omitted > 0 && (
          <p className="m-0 text-small text-ink-muted">
            {formatCount(report.flows_omitted)} smaller connections are not in the address and traffic-type lists.
          </p>
        )}
        <CoverageList coverage={report.coverage} />
      </section>
    </>
  );
}

function Programs({ range }: { range: Range }) {
  const r = TRAFFIC_RANGES[range];
  const traffic = useTraffic(r.seconds, r.points, r.reload * 1_000);
  // One interval is enough: only the ranking is used, to deal hues that hold across ranges.
  const week = useTraffic(TRAFFIC_RANGES["7d"].seconds, 1, 300_000);
  if (traffic.isError) return <IncompleteNotice title="Traffic by program could not be loaded" errors={[traffic.error.message]} />;
  if (!traffic.data || week.isPending) return <Skeleton shape="block" />;
  const report = traffic.data;
  if (report.programs.length === 0) {
    return report.coverage.skipped.length > 0 ? (
      <IncompleteNotice title="Traffic by program is not being counted" coverage={report.coverage} />
    ) : (
      <>
        <EmptyState>No TCP traffic was recorded in this range. Bigman counts it while background checks run.</EmptyState>
        <CoverageList coverage={report.coverage} />
      </>
    );
  }
  return <ByProgram report={report} ranking={week.data ?? report} rangeLabel={RANGES[range].label} />;
}

function Interfaces({ range }: { range: Range }) {
  const r = RANGES[range];
  const now = nowSeconds();
  // Stored history reloads on a fixed step; between reloads the 5-minute view adds live samples.
  const stored = useSamples(r.seconds, r.points, r.reload * 1_000);
  const live = useLiveSamples((s) => s.samples);
  const series = useMemo(
    () => byTraffic(buildSeries(stored.data, range === "5m" ? live : [], now - r.seconds, now)),
    // `now` moves every render; recompute with the data that drives it.
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [stored.data, live, range],
  );
  if (stored.isError) return <IncompleteNotice title="Throughput history could not be loaded" errors={[stored.error.message]} />;
  if (stored.isPending) return <Skeleton shape="block" />;
  if (series.length === 0) {
    return <EmptyState>No throughput was recorded in this range. Bigman records it while background checks run.</EmptyState>;
  }
  return (
    <div className="flex flex-col gap-5">
      {series.map(([iface, s]) => (
        <ThroughputChart key={iface} label={iface} series={s} height={160} />
      ))}
      <p className="m-0 text-small text-ink-muted">A break in a line is time Bigman was not running.</p>
    </div>
  );
}

export function TrafficPage() {
  const { range, setRange } = useShell();
  return (
    <>
      <PageHeader title="Traffic" controls={<Select label="Time range" hideLabel value={range} options={RANGE_OPTIONS} onChange={setRange} />} />
      <div className="flex flex-col gap-5 px-6 py-5">
        <DaemonGate>
          <Programs range={range} />
          <section aria-labelledby="interfaces" className="flex flex-col gap-3 border-t border-rule pt-5">
            <h2 id="interfaces" className="m-0 text-body font-semibold text-ink">
              All traffic, by network interface
            </h2>
            <p className="m-0 max-w-[56ch] text-small text-ink-muted">
              Everything that crossed each interface, including UDP and anything Bigman cannot match to a program.
            </p>
            <Interfaces range={range} />
          </section>
        </DaemonGate>
      </div>
    </>
  );
}
