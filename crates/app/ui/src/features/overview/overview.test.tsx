import { screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
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
