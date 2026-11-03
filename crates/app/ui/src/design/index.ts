// The design system: the B6 component vocabulary and the helpers it shares.
// Adding a component needs a gallery entry and a reason the existing ones cannot do it.
export { ActionButton } from "./components/ActionButton";
export { AppShell, BigmanMark, type ShellTab } from "./components/AppShell";
export { ConfirmDanger } from "./components/ConfirmDanger";
export { CoverageList, SKIP_REASON } from "./components/CoverageList";
export { DataTable } from "./components/DataTable";
export { DiffView, type DiffHunk, type DiffLine } from "./components/DiffView";
export { Drilldown } from "./components/Drilldown";
export { EmptyState } from "./components/EmptyState";
export { EvidenceBlock } from "./components/EvidenceBlock";
export { FilterChips } from "./components/FilterChips";
export { FileTree, type FileTreeEntry, type FileTreeGroup } from "./components/FileTree";
export { FirstSeenMarker } from "./components/FirstSeenMarker";
export { IncompleteNotice } from "./components/IncompleteNotice";
export { KeyValue, type KeyValueItem } from "./components/KeyValue";
export { PageHeader } from "./components/PageHeader";
export { PathPicker } from "./components/PathPicker";
export { PreviewDialog } from "./components/PreviewDialog";
export { ProgressRow } from "./components/ProgressRow";
export { SearchField } from "./components/SearchField";
export { SegmentedTabs } from "./components/SegmentedTabs";
export { Select } from "./components/Select";
export { maxSeverity, SEVERITY_RANK, SeverityBadge } from "./components/SeverityBadge";
export { Skeleton } from "./components/Skeleton";
export { Sparkline } from "./components/Sparkline";
export { StatusLine } from "./components/StatusLine";
export { StatusTile, type TileState } from "./components/StatusTile";
export { Tally } from "./components/Tally";
export { TextField } from "./components/TextField";
export { ThroughputChart, type ThroughputSeries } from "./components/ThroughputChart";
export { Timeline } from "./components/Timeline";
export { Toast, Toaster, useToasts, type ToastAction } from "./components/Toast";
export { Toggle } from "./components/Toggle";
export { useDrilldown, type Panel } from "./drilldown";
export * from "./format";
export { fold, layout, type Occurrence, type TimelineEntry } from "./timeline";
export { applyTheme, storedTheme, type ThemeChoice } from "./theme";
export {
  AboutRule,
  FindingEvidence,
  reportOccurrence,
  reportOccurrences,
  ReportPanel,
  reportSeverity,
  reportStatus,
  reportTitle,
  verifiesDetection,
  type ReportResult,
} from "./report";
export { ActionDialog, ActionDialogHost, closeActionDialog } from "./action";
export { CheckStatus } from "./check";
export { DaemonGate } from "./daemon-gate";
export { ReportById } from "./report-by-id";
export { EventPanel, eventOccurrence, sourceLabel } from "./events";
export { ReceiptById, ReceiptPanel } from "./receipt";
export { heldIn } from "./report-actions";
export { hardeningSummary, type HardeningSummary } from "./hardening";
export { LastResult } from "./last-result";
export { SourceGapLine, SourceRemedy } from "./source-gap";
