import { screen, waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { useProtectView } from "@/shell/protect-view";
import { useShell } from "@/shell/store";
import { renderWithDaemon } from "@/test-utils";
import { OverviewPage } from "./OverviewPage";

describe("Overview's Hardening tile", () => {
  it("shows on Overview as a tile that opens Protect > Hardening", async () => {
    useShell.setState({ tab: "overview" });
    useProtectView.setState({ view: "scans" });
    const { user } = renderWithDaemon(<OverviewPage />);
    // The tile counts the way the view does: three to review, two optional.
    const tile = await screen.findByRole("button", { name: /Hardening.*3 to review/ });
    expect(tile).toHaveTextContent("Turn off the SSH server, and 2 more · 2 optional");
    await user.click(tile);
    expect(useShell.getState().tab).toBe("protect");
    expect(useProtectView.getState().view).toBe("hardening");
  });

  it("shows incomplete coverage on the Overview tile", async () => {
    useShell.setState({ tab: "overview" });
    renderWithDaemon(<OverviewPage />, "incomplete");
    const tile = await screen.findByRole("button", { name: /Hardening/ });
    await waitFor(() => expect(tile).toHaveTextContent("1 setting not checked"));
  });
  it("moves focus to the new page's heading when a tile switches tabs", async () => {
    useShell.setState({ tab: "overview" });
    const { user } = renderWithDaemon(
      <>
        <main>
          <h1 tabIndex={-1}>Protect</h1>
        </main>
        <OverviewPage />
      </>,
    );
    await user.click(await screen.findByRole("button", { name: /Hardening.*3 to review/ }));
    await waitFor(() => expect(document.activeElement).toBe(screen.getByRole("heading", { level: 1, name: "Protect" })));
  });
});
