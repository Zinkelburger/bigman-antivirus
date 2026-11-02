import { screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { renderWithDaemon } from "@/test-utils";
import { ProtectPage } from "./ProtectPage";
import { useShell } from "@/shell/store";

describe("Protect", () => {
  it("scans the Downloads folder and shows the result", async () => {
    useShell.setState({ protectView: "scans" });
    const { user } = renderWithDaemon(<ProtectPage />, "calm");
    await user.click(await screen.findByRole("button", { name: "Scan with ClamAV" }));
    const result = await screen.findByRole("region", { name: "Result" });
    expect(within(result).getByText("No findings from these checks")).toBeInTheDocument();
    expect(within(result).getByText("/home/sam/Downloads")).toBeInTheDocument();
  });

  it("quarantines a flagged file after a preview, with Undo in the result", async () => {
    useShell.setState({ protectView: "scans" });
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
    useShell.setState({ protectView: "quarantine" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("row", { name: /old-installer\.run/ }));
    await user.click(await screen.findByRole("button", { name: "Restore…" }));
    const dialog = await screen.findByRole("dialog", { name: "Restore a quarantined file" });
    await user.click(await within(dialog).findByRole("button", { name: "Restore" }));
    expect(await within(dialog).findByText(/Restored without executable permissions/)).toBeInTheDocument();
  });

  it("tests a canary's detection", async () => {
    useShell.setState({ protectView: "canaries" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("row", { name: /backup-access\.env/ }));
    await user.click(await screen.findByRole("button", { name: "Test detection" }));
    expect(await screen.findByText("Local read detection verified")).toBeInTheDocument();
  });
});

describe("Protect updates", () => {
  it("checks for security updates and lists the exploited one first", async () => {
    useShell.setState({ protectView: "updates" });
    const { user } = renderWithDaemon(<ProtectPage />);
    expect(await screen.findByText("Security updates have not been checked yet.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Check for updates" }));
    const result = await screen.findByRole("region", { name: "Result" });
    const rows = within(within(result).getByRole("grid", { name: "Findings" })).getAllByRole("row").slice(1);
    expect(rows[0]).toHaveTextContent("fixes a vulnerability known to be exploited");
    expect(rows[0]).toHaveTextContent("High");
    expect(within(result).getByRole("heading", { name: "Security updates: 3 security updates" })).toBeInTheDocument();
    expect(within(result).getByText("sudo dnf upgrade --advisories=FEDORA-2026-4a1b2c3d4e,FEDORA-2026-31601794bf,FEDORA-2026-d732fb97a0")).toBeInTheDocument();
  });
});
