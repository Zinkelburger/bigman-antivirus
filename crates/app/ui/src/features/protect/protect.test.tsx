import { screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { renderWithDaemon } from "@/test-utils";
import { ProtectPage } from "./ProtectPage";
import { useProtectView } from "./store";

describe("Protect", () => {
  it("scans the Downloads folder and shows the result", async () => {
    useProtectView.setState({ view: "scans" });
    const { user } = renderWithDaemon(<ProtectPage />, "calm");
    await user.click(await screen.findByRole("button", { name: "Scan with ClamAV" }));
    const result = await screen.findByRole("region", { name: "Result" });
    expect(within(result).getByText("No findings from these checks")).toBeInTheDocument();
    expect(within(result).getByText("/home/sam/Downloads")).toBeInTheDocument();
  });

  it("quarantines a flagged file after a preview, with Undo in the result", async () => {
    useProtectView.setState({ view: "scans" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("button", { name: "Open" }));
    await user.click(await screen.findByRole("button", { name: "Quarantine…" }));
    const dialog = await screen.findByRole("dialog", { name: "Quarantine a file" });
    expect(await within(dialog).findByText(/invoice-copy\.zip/)).toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Quarantine" }));
    expect(await within(dialog).findByText(/recovery copy is saved/)).toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Undo" }));
    expect(await within(dialog).findByText(/Restored without executable permissions/)).toBeInTheDocument();
  });

  it("restores from quarantine", async () => {
    useProtectView.setState({ view: "quarantine" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("row", { name: /old-installer\.run/ }));
    await user.click(await screen.findByRole("button", { name: "Restore…" }));
    const dialog = await screen.findByRole("dialog", { name: "Restore a quarantined file" });
    await user.click(await within(dialog).findByRole("button", { name: "Restore" }));
    expect(await within(dialog).findByText(/Restored without executable permissions/)).toBeInTheDocument();
  });

  it("tests a canary's detection", async () => {
    useProtectView.setState({ view: "canaries" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("row", { name: /backup-access\.env/ }));
    await user.click(await screen.findByRole("button", { name: "Test detection" }));
    expect(await screen.findByText("Local read detection verified")).toBeInTheDocument();
  });
});
