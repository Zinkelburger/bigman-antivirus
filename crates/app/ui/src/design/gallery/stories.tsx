// Every component in every state it supports. The gallery renders these, and
// gallery.test.tsx renders each one, so a component without stories is untested.
import type { ColumnDef } from "@tanstack/react-table";
import { FileText, Network as NetworkIcon, Package, Radio } from "lucide-react";
import { useState, type ReactNode } from "react";
import type { Coverage, Report } from "@/ipc";
import { ActionButton } from "../components/ActionButton";
import { ConfirmDanger } from "../components/ConfirmDanger";
import { CoverageList } from "../components/CoverageList";
import { DataTable } from "../components/DataTable";
import { DiffView } from "../components/DiffView";
import { EmptyState } from "../components/EmptyState";
import { EvidenceBlock } from "../components/EvidenceBlock";
import { FileTree } from "../components/FileTree";
import { FilterChips } from "../components/FilterChips";
import { FirstSeenMarker } from "../components/FirstSeenMarker";
import { IncompleteNotice } from "../components/IncompleteNotice";
import { KeyValue } from "../components/KeyValue";
import { PathText } from "../components/PathText";
import { PageHeader } from "../components/PageHeader";
import { PathPicker } from "../components/PathPicker";
import { PreviewDialog } from "../components/PreviewDialog";
import { ProgressRow } from "../components/ProgressRow";
import { SearchField } from "../components/SearchField";
import { SegmentedTabs } from "../components/SegmentedTabs";
import { Select } from "../components/Select";
import { SeverityBadge } from "../components/SeverityBadge";
import { Skeleton } from "../components/Skeleton";
import { Sparkline } from "../components/Sparkline";
import { StatusLine } from "../components/StatusLine";
import { StatusTile } from "../components/StatusTile";
import { ThroughputChart } from "../components/ThroughputChart";
import { Timeline } from "../components/Timeline";
import { Toast } from "../components/Toast";
import { TextField } from "../components/TextField";
import { Toggle } from "../components/Toggle";
import { ActionDialog } from "../action";
import { CheckStatus } from "../check";
import { formatBytes } from "../format";
import { RECEIPTS } from "@/mock/live";
import { ReceiptPanel } from "../receipt";
import { ReportPanel } from "../report";
import type { TimelineEntry } from "../timeline";
import trayAttention from "../tray/tray-attention.svg";
import trayProblem from "../tray/tray-problem.svg";
import trayQuiet from "../tray/tray-quiet.svg";

export type StoryState = "loading" | "empty" | "findings" | "no findings" | "incomplete" | "disconnected" | "default";

export interface Story {
  group: "Shell" | "Status" | "Data" | "Evidence" | "Charts" | "Actions" | "Feedback" | "Input";
  component: string;
  state: StoryState;
  render: () => ReactNode;
}

// Synthetic sample data; paths and names are invented.
export const NOW = Math.floor(Date.UTC(2026, 9, 29, 14, 2) / 1000);
const min = (n: number) => NOW - n * 60;

const coverage: Coverage = {
  checked: ["ClamAV signature scan", "Archive expansion"],
  notes: ["Signature and heuristic scan of file snapshots. Findings do not prove the host is uncompromised."],
  skipped: [
    { what: "/home/sam/Downloads/backup.tar", why: "limit_reached", detail: "Expected a regular file no larger than 67108864 bytes" },
    { what: "ShellCheck", why: "tool_missing", detail: "shellcheck is not installed" },
  ],
};

const report: Report = {
  id: "gallery-report",
  created: min(42),
  kind: "Installer inspection",
  subject: "/home/sam/Downloads/install.sh",
  sha256: "4397ea1f9014f4ae085fb4f108db3aaa16f00d5711ff5070fb36d4117b18f87e",
  findings: [
    {
      rule: "shell.execute-download",
      severity: "high",
      summary: "Downloads and executes code",
      evidence: "Line 5: curl -fsSL https://example.org/setup.sh | sh",
      action: "Save and inspect the exact downloaded bytes first.",
    },
    {
      rule: "shell.privilege",
      severity: "review",
      summary: "Requests elevated privileges",
      evidence: "Line 6: sudo install -m 0755 /tmp/tool /usr/local/bin/tool",
      action: "Check why this operation needs administrator access.",
    },
  ],
  coverage: { checked: ["Capability rules"], notes: ["Static capability rules only. No code was executed."], skipped: [] },
  errors: [],
  details: "",
};

const entries: TimelineEntry[] = [
  { id: "e1", at: min(3), summary: "Firefox connected to a new host, telemetry.example.net", source: "Network", icon: NetworkIcon, severity: "info", firstSeen: true, count: 1, firstAt: min(3) },
  { id: "e2", at: min(9), summary: "A new program is listening on all interfaces, port 8080", source: "Listener", icon: Radio, severity: "review", firstSeen: true, count: 1, firstAt: min(9) },
  { id: "e3", at: min(25), summary: "dnf installed 3 packages, 1 install hook ran", source: "Receipt", icon: Package, severity: "info", firstSeen: false, count: 1, firstAt: min(25) },
  { id: "e4", at: min(31), summary: "Syncthing connected to its relay", source: "Network", icon: NetworkIcon, severity: "info", firstSeen: false, count: 14, firstAt: NOW - 26 * 86_400 },
  { id: "e5", at: NOW - 86_400 - 600, summary: "Downloads scan finished with no findings", source: "Scan", icon: FileText, severity: "info", firstSeen: false, count: 6, firstAt: NOW - 6 * 86_400 },
];

interface Row {
  process: string;
  destination: string;
  port: number;
  bytes: number;
}
const rows: Row[] = [
  { process: "firefox", destination: "telemetry.example.net", port: 443, bytes: 48_210 },
  { process: "syncthing", destination: "relay.example.org", port: 22067, bytes: 1_204_993 },
  { process: "flatpak", destination: "dl.example.org", port: 443, bytes: 412_221_440 },
];
const columns: ColumnDef<Row>[] = [
  { accessorKey: "process", header: "Process", meta: { width: "8rem" } },
  { accessorKey: "destination", header: "Destination" },
  { accessorKey: "port", header: "Port", meta: { align: "right", width: "5rem" } },
  { accessorKey: "bytes", header: "Data", cell: ({ getValue }) => formatBytes(getValue<number>()), meta: { align: "right", width: "7rem" } },
];

const samples = (() => {
  const at: number[] = [];
  const rx: (number | null)[] = [];
  const tx: (number | null)[] = [];
  for (let i = 0; i < 300; i++) {
    at.push(NOW - (300 - i));
    const gap = i > 200 && i < 215;
    rx.push(gap ? null : 20_000 + Math.round(Math.abs(Math.sin(i / 17)) * 180_000));
    tx.push(gap ? null : 4_000 + Math.round(Math.abs(Math.cos(i / 23)) * 30_000));
  }
  return { at, rx, tx };
})();

function ChipsStory() {
  const [selected, setSelected] = useState<Set<string>>(new Set(["File scan"]));
  return <FilterChips label="Filter" options={[{ value: "File scan", label: "File scan", count: 6 }, { value: "Installer inspection", label: "Installer inspection", count: 2 }, { value: "Security checkup", label: "Security checkup", count: 1 }]} selected={selected} onChange={setSelected} />;
}

function TabsStory() {
  const [active, setActive] = useState("scans");
  return <SegmentedTabs label="Protect" tabs={[{ id: "scans", label: "Scans" }, { id: "quarantine", label: "Quarantine", count: 1 }, { id: "canaries", label: "Canaries" }]} active={active} onSelect={setActive} />;
}

function DialogStory({ danger = false }: { danger?: boolean }) {
  const [open, setOpen] = useState(false);
  const Dialog = danger ? ConfirmDanger : PreviewDialog;
  return (
    <>
      <ActionButton onClick={() => setOpen(true)}>{danger ? "Quarantine file…" : "Install ShellCheck…"}</ActionButton>
      <Dialog
        open={open}
        onOpenChange={setOpen}
        title={danger ? "Quarantine invoice-copy.zip" : "Install ShellCheck"}
        confirmLabel={danger ? "Quarantine" : "Install"}
        plan={<EvidenceBlock label="Command" text={danger ? "Save a private recovery copy, then remove\n/home/sam/Downloads/invoice-copy.zip" : "pkexec dnf install -y ShellCheck"} />}
        onConfirm={() => new Promise((resolve) => setTimeout(() => resolve(<p className="m-0">Installed ShellCheck 0.10.0.</p>), 600))}
      />
    </>
  );
}

const story = (group: Story["group"], component: string, state: StoryState, render: () => ReactNode): Story => ({ group, component, state, render });

export const STORIES: Story[] = [
  story("Shell", "PageHeader", "default", () => (
    <PageHeader title="Network" controls={<Select label="Range" hideLabel value="1h" onChange={() => undefined} options={[{ value: "1h", label: "Last hour" }, { value: "24h", label: "Last 24 hours" }]} />} action={<ActionButton variant="primary">Run checkup</ActionButton>} />
  )),
  story("Shell", "TrayMenu", "default", () => (
    <div className="flex flex-wrap items-center gap-3 text-small text-ink-muted">
      {[["Quiet", trayQuiet], ["New activity", trayAttention], ["Needs attention", trayProblem]].map(([label, src]) => (
        <span key={label} className="inline-flex shrink-0 items-center gap-2 rounded-panel bg-rail px-3 py-2 whitespace-nowrap text-rail-ink">
          <img src={src} alt="" width={22} height={22} />
          {label}
        </span>
      ))}
      <span>Menu: Open Bigman · Pause notifications for 1 hour · Quit</span>
    </div>
  )),
  story("Status", "StatusTile", "loading", () => <StatusTile label="Bigman's tools" state="loading" />),
  story("Status", "StatusTile", "empty", () => <StatusTile label="Last scan" state="empty" value="No scans yet" detail="Scan Downloads from Protect." />),
  story("Status", "StatusTile", "findings", () => <StatusTile label="Last scan" state="ready" value="1 finding" detail="Downloads · 42 minutes ago" severity="high" onOpen={() => undefined} />),
  story("Status", "StatusTile", "no findings", () => <StatusTile label="Last scan" state="ready" value="No findings" detail="Downloads · 3 hours ago" onOpen={() => undefined} />),
  story("Status", "StatusTile", "incomplete", () => <StatusTile label="Last scan" state="incomplete" detail="2 items were not checked." onOpen={() => undefined} />),
  story("Status", "StatusTile", "disconnected", () => <StatusTile label="Last scan" state="disconnected" />),
  story("Status", "StatusLine", "no findings", () => <StatusLine ranAt={min(0)} signaturesUpdated={min(120)} now={NOW} />),
  story("Status", "SeverityBadge", "default", () => (
    <div className="flex gap-2">
      <SeverityBadge severity="info" />
      <SeverityBadge severity="review" />
      <SeverityBadge severity="high" />
    </div>
  )),
  story("Status", "FirstSeenMarker", "default", () => (
    <div className="flex items-center gap-4">
      <FirstSeenMarker />
      <FirstSeenMarker compact />
    </div>
  )),
  story("Status", "CoverageList", "incomplete", () => <CoverageList coverage={coverage} />),
  story("Status", "IncompleteNotice", "incomplete", () => <IncompleteNotice coverage={coverage} />),
  story("Status", "IncompleteNotice", "disconnected", () => (
    <IncompleteNotice kind="disconnected" errors={["Bigman daemon is not running at /run/user/1000/bigman/bigmand.sock"]} action={<ActionButton>Start background checks</ActionButton>} />
  )),
  story("Data", "DataTable", "findings", () => <DataTable label="Connections" columns={columns} data={rows} getRowId={(r) => r.destination} onOpen={() => undefined} initialSorting={[{ id: "bytes", desc: true }]} height={200} />),
  story("Data", "DataTable", "empty", () => <DataTable label="Connections" columns={columns} data={rows} getRowId={(r) => r.destination} filter="no such host" height={200} />),
  story("Data", "DataTable", "loading", () => <Skeleton shape="rows" count={3} />),
  story("Data", "Timeline", "findings", () => <Timeline label="Activity" entries={entries} now={NOW} onOpen={() => undefined} height={320} />),
  story("Data", "Timeline", "empty", () => (
    <Timeline
      label="Activity"
      entries={[]}
      now={NOW}
      empty={
        <>
          <p className="m-0">Each check you run is entered here the first time; later runs of the same check fold under it.</p>
          <ActionButton>Run checkup</ActionButton>
        </>
      }
    />
  )),
  story("Data", "Timeline", "loading", () => <Skeleton shape="rows" count={4} />),
  story("Data", "KeyValue", "default", () => (
    <KeyValue items={[{ label: "Package", value: "openssh-server 9.9p1-3" }, { label: "Path", value: "/etc/ssh/sshd_config", mono: true }, { label: "Attribution", value: "Owned but modified" }]} />
  )),
  story("Data", "Drilldown", "findings", () => (
    <div className="max-w-[560px] rounded-panel border border-rule bg-surface p-4">
      <ReportPanel report={report} now={NOW} />
    </div>
  )),
  story("Data", "Drilldown", "no findings", () => (
    <div className="max-w-[560px] rounded-panel border border-rule bg-surface p-4">
      <ReportPanel report={{ ...report, findings: [] }} now={NOW} />
    </div>
  )),
  story("Data", "Drilldown", "incomplete", () => (
    <div className="max-w-[560px] rounded-panel border border-rule bg-surface p-4">
      <ReportPanel report={{ ...report, findings: [], coverage, errors: ["clamscan exited with status 2: can't open file"] }} now={NOW} />
    </div>
  )),
  story("Data", "Receipt", "findings", () => (
    <div className="max-w-[560px] rounded-panel border border-rule bg-surface p-4">
      {RECEIPTS["rc-412"] && <ReceiptPanel receipt={RECEIPTS["rc-412"]} />}
    </div>
  )),
  story("Data", "Receipt", "no findings", () => (
    <div className="max-w-[560px] rounded-panel border border-rule bg-surface p-4">
      {RECEIPTS["rc-412"] && <ReceiptPanel receipt={{ ...RECEIPTS["rc-412"], findings: [], hits: [], after: { checked: false, listeners: [] } }} />}
    </div>
  )),
  story("Data", "PathText", "default", () => (
    <div className="w-64 text-small">
      <PathText path="/home/sam/Downloads/quarterly-reports/2026/Meeting_Notice.pdf.desktop" />
    </div>
  )),
  story("Evidence", "EvidenceBlock", "findings", () => (
    <EvidenceBlock label="install.sh" text={'#!/bin/sh\necho "Installing example tool"\ncurl -fsSL https://example.org/setup.sh | sh\nsudo install -m 0755 /tmp/tool /usr/local/bin/tool'} highlight={[3]} notes={{ 3: { text: "shell.execute-download", tone: "high" } }} />
  )),
  story("Evidence", "DiffView", "findings", () => (
    <DiffView
      label="example.install (1.2 → 1.3)"
      hunks={[{ header: "@@ post_install @@", oldStart: 4, newStart: 4, lines: [
        { kind: "context", text: "post_install() {" },
        { kind: "remove", text: "  systemctl daemon-reload" },
        { kind: "add", text: "  systemctl daemon-reload" },
        { kind: "add", text: "  echo 'export PATH=/opt/example:$PATH' > /etc/profile.d/example.sh", rule: "receipt.hook.sensitive_path" },
        { kind: "context", text: "}" },
      ] }]}
    />
  )),
  story("Evidence", "FileTree", "findings", () => (
    <FileTree
      onOpen={() => undefined}
      groups={[
        { category: "/etc", files: [{ path: "/etc/profile.d/example.sh", change: "added", severity: "review" }] },
        { category: "systemd", files: [{ path: "/usr/lib/systemd/system/example.service", change: "added" }] },
        { category: "Binaries", files: [{ path: "/usr/bin/example", change: "changed" }, { path: "/usr/bin/example-helper", change: "added" }] },
      ]}
    />
  )),
  story("Charts", "ThroughputChart", "findings", () => <ThroughputChart label="wlp2s0" series={samples} height={180} />),
  story("Charts", "Sparkline", "default", () => <Sparkline label="Connections per hour, last 24 hours" values={[3, 4, 2, 8, 12, 9, 7, 3, 2, 5, 6, 4]} />),
  story("Actions", "ActionButton", "default", () => (
    <div className="flex flex-wrap gap-2">
      <ActionButton variant="primary">Run checkup</ActionButton>
      <ActionButton>Update definitions</ActionButton>
      <ActionButton variant="quiet">Cancel</ActionButton>
      <ActionButton busy>Scanning</ActionButton>
      <ActionButton disabled>Not available</ActionButton>
    </div>
  )),
  story("Actions", "PreviewDialog", "default", () => <DialogStory />),
  story("Actions", "ConfirmDanger", "default", () => <DialogStory danger />),
  story("Feedback", "EmptyState", "empty", () => <EmptyState action={<ActionButton>Scan Downloads</ActionButton>}>Scan results appear here after your first scan.</EmptyState>),
  story("Feedback", "Skeleton", "loading", () => <Skeleton shape="tiles" count={4} />),
  story("Feedback", "Toast", "default", () => <Toast text="Definitions updated." onDismiss={() => undefined} />),
  story("Feedback", "ProgressRow", "loading", () => <ProgressRow label="Scanning Downloads" detail="1,204 of 2,000 files" progress={0.6} onCancel={() => undefined} />),
  story("Feedback", "ProgressRow", "default", () => <ProgressRow label="Updating definitions" progress={null} onCancel={() => undefined} />),
  story("Input", "PathPicker", "default", () => <PathPicker label="Folder to scan" value="/home/sam/Downloads" onChange={() => undefined} onBrowse={() => undefined} />),
  story("Input", "Toggle", "default", () => <Toggle checked label="New listeners" description="Notify when a program starts listening on the network." onChange={() => undefined} />),
  story("Input", "Select", "default", () => <Select label="Theme" value="system" onChange={() => undefined} options={[{ value: "system", label: "Follow system" }, { value: "light", label: "Light" }, { value: "dark", label: "Dark" }]} />),
  story("Input", "SearchField", "default", () => <SearchField value="" onChange={() => undefined} />),
  story("Input", "TextField", "default", () => <TextField label="Extension ID" value="" onChange={() => undefined} mono hint="32 letters from a to p." />),
  story("Input", "FilterChips", "default", () => <ChipsStory />),
  story("Shell", "SegmentedTabs", "default", () => <TabsStory />),
  story("Actions", "ActionDialog", "default", () => <ActionDialog action={{ kind: "package_policy", path: "/home/sam/Projects/example-app" }} label="Disable automatic install scripts…" />),
  story("Feedback", "CheckStatus", "loading", () => <CheckStatus check={{ isPending: true, error: null, cancel: () => undefined }} running="Scanning /home/sam/Downloads" detail="At most 2,000 files." />),
  story("Feedback", "CheckStatus", "findings", () => <CheckStatus check={{ isPending: false, data: report, error: null, cancel: () => undefined }} running="" />),
  story("Feedback", "CheckStatus", "incomplete", () => <CheckStatus check={{ isPending: false, error: new Error("clamscan exited with status 2: can't open file"), cancel: () => undefined }} running="" />),
];
