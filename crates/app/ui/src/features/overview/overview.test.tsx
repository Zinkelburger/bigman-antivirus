import { screen } from "@testing-library/react";
import { beforeEach, describe, expect, it } from "vitest";
import { useShell } from "@/shell/store";
import { renderWithDaemon } from "@/test-utils";
import { OverviewPage } from "./OverviewPage";

describe("Overview updates tile", () => {
  it("opens Protect > Updates before the first check", async () => {
    useShell.setState({ tab: "overview", protectView: "scans" });
    const { user } = renderWithDaemon(<OverviewPage />);
    await user.click(await screen.findByRole("button", { name: /^Updates.*Not checked/ }));
    expect(useShell.getState()).toMatchObject({ tab: "protect", protectView: "updates" });
  });

  it("says nothing is listed, not that the system is safe", async () => {
    renderWithDaemon(<OverviewPage />, "calm");
    expect(await screen.findByRole("button", { name: /^Updates.*No updates listed/ })).toBeInTheDocument();
  });
});

describe("Overview first-run note", () => {
  beforeEach(() => localStorage.clear());

  it("says what runs on its own, with its switches, until it is dismissed", async () => {
    const { user, unmount } = renderWithDaemon(<OverviewPage />, "calm");
    expect(await screen.findByRole("heading", { name: "What Bigman does on its own" })).toBeInTheDocument();
    expect(screen.getByText(/keep running after you close this window/)).toBeInTheDocument();
    const downloads = await screen.findByRole("switch", { name: "Check new downloads" });
    await user.click(downloads);
    expect(await screen.findByRole("switch", { name: "Check new downloads" })).toHaveAttribute("aria-checked", "false");
    await user.click(screen.getByRole("button", { name: "Got it" }));
    expect(screen.queryByRole("heading", { name: "What Bigman does on its own" })).not.toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Last 24 hours" })).toHaveFocus();
    unmount();
    renderWithDaemon(<OverviewPage />, "calm");
    expect(await screen.findByRole("button", { name: /^Updates/ })).toBeInTheDocument();
    expect(screen.queryByRole("heading", { name: "What Bigman does on its own" })).not.toBeInTheDocument();
  });

  it("is not shown while background checks are down, so it never claims they run", async () => {
    renderWithDaemon(<OverviewPage />, "disconnected");
    expect(await screen.findByText("Background checks are not running")).toBeInTheDocument();
    expect(screen.queryByRole("heading", { name: "What Bigman does on its own" })).not.toBeInTheDocument();
  });
});
