import { act, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { AppShell } from "./components/AppShell";
import { useDrilldown } from "./drilldown";
import { NotebookTabs } from "lucide-react";

function Page() {
  const open = useDrilldown((s) => s.open);
  return (
    <button type="button" onClick={() => open({ title: "Last scan", content: <p>Evidence</p> })}>
      Open last scan
    </button>
  );
}

describe("drill-down", () => {
  it("returns focus to the element that opened it", async () => {
    const user = userEvent.setup();
    render(
      <AppShell tabs={[{ id: "overview", label: "Overview", icon: NotebookTabs }]} active="overview" onSelect={() => undefined} statusStrip={null}>
        <Page />
      </AppShell>,
    );
    const opener = screen.getByRole("button", { name: "Open last scan" });
    await user.click(opener);
    expect(screen.getByRole("dialog", { name: "Last scan" })).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Close panel" }));
    await act(() => new Promise((r) => requestAnimationFrame(() => r(undefined))));
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    expect(document.activeElement).toBe(opener);
  });
});
