import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { render, screen } from "@testing-library/react";
import type { ReactNode } from "react";
import { beforeAll, describe, expect, it } from "vitest";
import { setTransport } from "@/ipc";
import { mockTransport } from "@/mock/transport";
import { STORIES } from "./stories";

beforeAll(() => setTransport(mockTransport("findings", 0, 0)));
const withQuery = (node: ReactNode) => <QueryClientProvider client={new QueryClient()}>{node}</QueryClientProvider>;

const COMPONENTS = [
  "PageHeader", "TrayMenu", "StatusTile", "StatusLine", "SeverityBadge", "FirstSeenMarker", "CoverageList",
  "IncompleteNotice", "DataTable", "Timeline", "KeyValue", "Drilldown", "EvidenceBlock", "DiffView", "FileTree",
  "ThroughputChart", "Sparkline", "ActionButton", "PreviewDialog", "ConfirmDanger", "EmptyState", "Skeleton",
  "Toast", "ProgressRow", "PathPicker", "Toggle", "Select", "SearchField", "TextField", "FilterChips",
  "SegmentedTabs", "ActionDialog", "CheckStatus",
];

const ALARM = /\b(threat|attack|infected|danger|critical|safe|clean|protected|secure)\b/i;

describe("component gallery", () => {
  it("has at least one story for every component in the vocabulary", () => {
    const covered = new Set(STORIES.map((s) => s.component));
    expect(COMPONENTS.filter((c) => !covered.has(c))).toEqual([]);
  });

  it.each(STORIES.map((s) => [`${s.component} (${s.state})`, s] as const))("renders %s", (_, story) => {
    const { container } = render(withQuery(story.render()));
    expect(container).not.toBeEmptyDOMElement();
    // Copy rules: no alarm or reassurance words in any rendered state.
    expect(container.textContent).not.toMatch(ALARM);
  });

  it("covers the six data-view states for StatusTile", () => {
    const states = STORIES.filter((s) => s.component === "StatusTile").map((s) => s.state);
    expect(states.sort()).toEqual(["disconnected", "empty", "findings", "incomplete", "loading", "no findings"]);
  });

  it("never conveys severity by color alone", () => {
    const badges = STORIES.find((s) => s.component === "SeverityBadge");
    render(<>{badges?.render()}</>);
    for (const label of ["Info", "Review", "High"]) expect(screen.getByText(label)).toBeInTheDocument();
  });
});
