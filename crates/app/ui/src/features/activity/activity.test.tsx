import { screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { renderWithDaemon } from "@/test-utils";
import { ActivityPage } from "./ActivityPage";

describe("Activity", () => {
  it("folds repeated scans and filters by severity", async () => {
    const { user } = renderWithDaemon(<ActivityPage />, "calm");
    expect(await screen.findByText(/seen 6 times, first on/)).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "High" }));
    expect(await screen.findByText("Nothing matches these filters.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Clear filters" }));
    expect(await screen.findByText(/seen 6 times/)).toBeInTheDocument();
  });

  it("opens the full report with an export action", async () => {
    const { user } = renderWithDaemon(<ActivityPage />);
    await user.click(await screen.findByRole("option", { name: /Inspected/ }));
    expect(await screen.findByRole("button", { name: "Export report…" })).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Export report…" }));
    expect(await screen.findByText(/It may contain local paths/)).toBeInTheDocument();
  });
});
