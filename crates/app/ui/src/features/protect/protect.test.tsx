import { screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { renderWithDaemon } from "@/test-utils";
import { ProtectPage } from "./ProtectPage";
import { useShell } from "@/shell/store";

describe("Protect startup files", () => {
  it("says a baseline is a starting point and points at the rated entries before accepting", async () => {
    useShell.setState({ protectView: "startup" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("button", { name: "Compare startup files" }));
    expect(await screen.findByText(/It does not mark anything as trustworthy/)).toBeInTheDocument();
    expect(screen.getByText(/2 entries are rated 1 High and 1 Review/)).toBeInTheDocument();
    expect(screen.queryByText(/saves this list as normal/)).not.toBeInTheDocument();
  });
});

describe("Protect", () => {
  it("scans the Downloads folder and shows the result", async () => {
    useShell.setState({ protectView: "scans" });
    const { user } = renderWithDaemon(<ProtectPage />, "calm");
    await user.click(await screen.findByRole("button", { name: "Scan with ClamAV" }));
    const result = await screen.findByRole("region", { name: "Result" });
    expect(within(result).getByRole("heading", { name: /: No findings from these checks$/ })).toBeInTheDocument();
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

  it("checks package sources from the same view", async () => {
    useShell.setState({ protectView: "updates" });
    const { user } = renderWithDaemon(<ProtectPage />);
    const sources = await screen.findByRole("region", { name: "Package sources" });
    expect(within(sources).getByText("Package sources have not been checked yet.")).toBeInTheDocument();
    await user.click(within(sources).getByRole("button", { name: "Check package sources" }));
    const result = await within(sources).findByRole("region", { name: "Result" });
    expect(within(result).getAllByText(/outranks the distribution/).length).toBeGreaterThan(0);
  });
});

describe("Protect AI skills", () => {
  it("lists skills and plugins, opens one with what it runs, and approves the version read", async () => {
    useShell.setState({ protectView: "skills" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("button", { name: "Check skills and plugins" }));
    expect(await screen.findByRole("heading", { name: "4 skills and plugins · 1 changed since you approved them · 2 not reviewed · 3 with findings" })).toBeInTheDocument();
    const grid = screen.getByRole("grid", { name: "Skills and plugins" });
    const rows = within(grid).getAllByRole("row").slice(1);
    expect(rows[0]).toHaveTextContent("deploy-helper");
    expect(rows[0]).toHaveTextContent("Changedsince you approved it 12 days ago");
    // Focus moves to the result, and the badge never stands for the count.
    expect(screen.getByRole("heading", { name: /^4 skills and plugins/ })).toHaveFocus();
    expect(within(grid).getByRole("row", { name: /repo-setup/ })).toHaveTextContent("High3 findings");

    await user.click(within(grid).getByRole("row", { name: /repo-setup/ }));
    const panel = await screen.findByRole("dialog", { name: "repo-setup" });
    expect(within(panel).getByText("When it is used: git branch --show-current")).toBeInTheDocument();
    // The answer leads: the High finding and what to do, before anything else.
    const first = within(panel).getByRole("region", { name: "What to look at first" });
    expect(within(first).getByText("Its instructions download and run a program")).toBeInTheDocument();
    expect(within(panel).getByText("gio trash '/home/sam/.agents/skills/repo-setup'")).toBeInTheDocument();
    // The SKILL.md is shown with the cited line marked.
    expect(within(panel).getAllByText("skill.download-and-run").length).toBeGreaterThan(0);

    await user.click(within(panel).getByRole("button", { name: "Approve…" }));
    const dialog = await screen.findByRole("dialog", { name: "Approve this version" });
    expect(await within(dialog).findByText(/It has a High finding, "Its instructions download and run a program", and 2 more to review/)).toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Approve" }));
    expect(await within(dialog).findByText("repo-setup is approved as it is now.")).toBeInTheDocument();
    expect(within(dialog).getByRole("button", { name: "Undo" })).toBeInTheDocument();
  });
});
