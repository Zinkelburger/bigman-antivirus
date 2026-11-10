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

describe("Protect Running system", () => {
  it("lists what the two views disagree about first, and marks a row as known after a preview", async () => {
    useShell.setState({ protectView: "running" });
    const { user } = renderWithDaemon(<ProtectPage />);
    // Before the first check: what it does, in words that need no glossary.
    expect(await screen.findByText(/asks your system three questions in two different ways/)).toBeInTheDocument();
    await user.click(await screen.findByRole("button", { name: "Check the running system" }));
    const heading = await screen.findByRole("heading", { name: "4 to review · 2 listed for information · 1 marked as known" });
    expect(heading).toHaveFocus();
    const grid = screen.getByRole("grid", { name: "Running system" });
    const rows = within(grid).getAllByRole("row").slice(1);
    // The most severe leads; a row that changed since your mark comes before
    // new ones of its severity; rows you marked as known come last.
    expect(rows[0]).toHaveTextContent("Two views disagree");
    expect(rows[2]).toHaveTextContent("5 BPF program names are loaded in the kernel");
    expect(rows[2]).toHaveTextContent("Changed since you marked it 3 days ago");
    expect(rows[rows.length - 1]).toHaveTextContent("Marked 9 days ago");

    await user.click(within(grid).getByRole("row", { name: /agent is told whenever/ }));
    const panel = await screen.findByRole("dialog", { name: "agent is told whenever /home/sam/.ssh changes" });
    expect(within(panel).getByText("Not marked as known")).toBeInTheDocument();
    const pinned = within(panel).getByRole("region", { name: "A mark holds while these stay the same" });
    expect(within(pinned).getByText("/home/sam/.cache/sync-agent/agent")).toBeInTheDocument();
    expect(within(pinned).queryByText("5120")).not.toBeInTheDocument();

    await user.click(within(panel).getByRole("button", { name: "Mark as known…" }));
    const dialog = await screen.findByRole("dialog", { name: "Mark as known" });
    expect(await within(dialog).findByText(/stops counting as something to review/)).toBeInTheDocument();
    // A row rated Review says what a mark is not, not only High ones.
    expect(within(dialog).getByText(/does not change its rating or make it harmless/)).toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Mark as known" }));
    expect(await within(dialog).findByText("Marked as known: agent is told whenever /home/sam/.ssh changes")).toBeInTheDocument();
    expect(within(dialog).getByRole("button", { name: "Undo" })).toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Close dialog" }));
    // The panel follows the mark without losing its place, and focus stays in it.
    const taken = await within(panel).findByRole("button", { name: "Take back the mark…" });
    expect(await screen.findByRole("heading", { name: "3 to review · 2 listed for information · 2 marked as known" })).toBeInTheDocument();
    expect(taken).toHaveFocus();
  });

  it("gives a command to copy instead of one written into a sentence", async () => {
    useShell.setState({ protectView: "running" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("button", { name: "Check the running system" }));
    const grid = await screen.findByRole("grid", { name: "Running system" });
    await user.click(within(grid).getByRole("row", { name: /updater is running/ }));
    const panel = await screen.findByRole("dialog", { name: "updater is running and its file was deleted" });
    expect(within(panel).getByText(/cp \/proc\/88213\/exe/)).toBeInTheDocument();
  });

  it("shows the one calm result on a quiet computer, and still lists what it saw", async () => {
    useShell.setState({ protectView: "running" });
    const { user } = renderWithDaemon(<ProtectPage />, "calm");
    await user.click(await screen.findByRole("button", { name: "Check the running system" }));
    expect(await screen.findByRole("heading", { name: "2 listed for information · 1 marked as known" })).toBeInTheDocument();
    expect(screen.getByText(/^No findings from these checks\. Ran at/)).toBeInTheDocument();
    expect(within(screen.getByRole("grid", { name: "Running system" })).getAllByRole("row")).toHaveLength(4);
  });

  it("leads with Incomplete when part of the system could not be read", async () => {
    useShell.setState({ protectView: "running" });
    const { user } = renderWithDaemon(<ProtectPage />, "incomplete");
    await user.click(await screen.findByRole("button", { name: "Check the running system" }));
    expect(await screen.findByText("Some of the running system could not be read")).toBeInTheDocument();
    expect(screen.queryByText(/^No findings from these checks/)).not.toBeInTheDocument();
  });

  it("says so plainly when nothing is listed", async () => {
    useShell.setState({ protectView: "running" });
    const { user } = renderWithDaemon(<ProtectPage />, "empty");
    await user.click(await screen.findByRole("button", { name: "Check the running system" }));
    expect(await screen.findByRole("heading", { name: "Nothing is listed from the parts that were read" })).toBeInTheDocument();
    expect(screen.getByText(/^No findings from these checks\. Ran at/)).toBeInTheDocument();
    expect(screen.queryByRole("grid", { name: "Running system" })).not.toBeInTheDocument();
  });
});
