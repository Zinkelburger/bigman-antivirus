import { screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
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

  it("explains inotify, keeps the responses folded, and holds reader-dependent ones off until identification is on", async () => {
    useShell.setState({ protectView: "canaries" });
    const { user } = renderWithDaemon(<ProtectPage />);
    expect(await screen.findByText(/inotify watches its folder/)).toBeInTheDocument();
    // Decoys exist, so the list comes before the form for another one.
    const headings = screen.getAllByRole("heading", { level: 2 }).map((h) => h.textContent);
    expect(headings.indexOf("Your canaries")).toBeLessThan(headings.indexOf("Add a local canary"));
    const folded = screen.getByText("When it is touched", { selector: "summary span" });
    expect(folded).toHaveTextContent("When it is touched Log and notify only");
    await user.click(folded);
    const choices = screen.getByRole("group", { name: "When it is touched" });
    // Unavailable, yet still reachable, so the reason is read out with the box.
    const kill = within(choices).getByRole("checkbox", { name: "Kill the process that touched it" });
    expect(kill).toHaveAttribute("aria-disabled", "true");
    expect(kill).toHaveAccessibleDescription("Needs reader identification, under Monitor.");
    await user.click(kill);
    expect(kill).not.toBeChecked();
    expect(within(choices).getByRole("checkbox", { name: "Lock your account" })).toHaveAttribute("aria-disabled", "true");
    // Logging out waits for a named reader too: a file manager showing the folder reads the decoy.
    expect(within(choices).getByRole("checkbox", { name: "Log out" })).toHaveAttribute("aria-disabled", "true");
    const locking = within(choices).getByRole("group", { name: "Lock yourself out" });
    await user.click(within(locking).getByRole("checkbox", { name: "Lock the screen" }));
    expect(folded).toHaveTextContent("Log and notify, then Lock the screen");

    await user.click(screen.getByRole("button", { name: "Turn on reader identification…" }));
    const dialog = await screen.findByRole("dialog", { name: "Turn on reader identification" });
    await user.click(within(dialog).getByRole("button", { name: "Turn on" }));
    expect(await within(dialog).findByText("Reader identification is on.")).toBeInTheDocument();
    await user.keyboard("{Escape}");
    await vi.waitFor(() => expect(kill).not.toHaveAttribute("aria-disabled"));
    // Locking the account says what it costs here, and what it adds up to with a locked screen.
    const lock = within(choices).getByRole("checkbox", { name: "Lock your account" });
    expect(lock).toHaveAccessibleDescription(/only administrator/);
    expect(lock).toHaveAccessibleDescription(/asks for your password once/);
    await user.click(lock);
    expect(lock).toHaveAccessibleDescription(/you cannot get back into this session either/);
  });

  it("does not create a decoy while Run a script is ticked without a path", async () => {
    useShell.setState({ protectView: "canaries" });
    const { user } = renderWithDaemon(<ProtectPage />);
    const create = await screen.findByRole("button", { name: "Create, monitor and test…" });
    expect(create).toBeEnabled();
    await user.click(screen.getByText("When it is touched", { selector: "summary span" }));
    const runScript = screen.getByRole("checkbox", { name: "Run a script" });
    await user.click(runScript);
    const script = screen.getByRole("textbox", { name: "Script" });
    expect(script).toHaveAccessibleDescription(/The full path, starting with \//);
    expect(create).toBeDisabled();
    await user.type(script, "bin/on-canary.sh");
    expect(script).toBeInvalid();
    expect(create).toBeDisabled();
    await user.clear(script);
    await user.type(script, "/home/sam/bin/on-canary.sh");
    expect(script).not.toBeInvalid();
    expect(create).toBeEnabled();
    await user.click(runScript);
    expect(screen.queryByRole("textbox", { name: "Script" })).not.toBeInTheDocument();
    expect(create).toBeEnabled();
  });

  it("shows a decoy's access log and changes its responses with Undo", async () => {
    useShell.setState({ protectView: "canaries" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("row", { name: /backup-access\.env/ }));
    const log = await screen.findByRole("region", { name: "Access log" });
    expect(await within(log).findByText("Bigman's own test read.")).toBeInTheDocument();
    expect(within(log).getAllByText("Lock the screen").length).toBeGreaterThan(0);
    expect(within(log).getByText("Started by bash (48102), started by ptyxis-agent (3310)")).toBeInTheDocument();
    const panel = screen.getByRole("dialog", { name: "Canary" });
    await user.click(within(panel).getByRole("button", { name: "Change responses" }));
    // Focus follows the swap of buttons: into the choices, and back out on Cancel.
    expect(within(panel).getByRole("checkbox", { name: "Run a script" })).toHaveFocus();
    await user.click(within(panel).getByRole("button", { name: "Cancel" }));
    expect(within(panel).getByRole("button", { name: "Change responses" })).toHaveFocus();
    await user.click(within(panel).getByRole("button", { name: "Change responses" }));
    const save = within(panel).getByRole("button", { name: "Save…" });
    expect(save).toBeDisabled();
    await user.click(within(panel).getByRole("checkbox", { name: "Turn networking off" }));
    await user.click(save);
    const dialog = await screen.findByRole("dialog", { name: "Change what happens when a decoy is touched" });
    expect(within(dialog).getByText("Turn networking off Lock the screen", { collapseWhitespace: true })).toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Save" }));
    expect(await within(dialog).findByText(/uses them from the next access/)).toBeInTheDocument();
    await user.keyboard("{Escape}");
    const steps = (await within(panel).findByText("Log and notify")).closest("ol") as HTMLElement;
    expect(within(steps).getAllByRole("listitem").map((li) => li.textContent)).toEqual(["Log and notify", "Turn networking off", "Lock the screen"]);
  });

  it("lets a response chosen earlier be cleared after reader identification was turned off", async () => {
    useShell.setState({ protectView: "canaries" });
    const { user } = renderWithDaemon(<ProtectPage />);
    const readerIdentification = async (label: string, verb: string) => {
      await user.click(await screen.findByRole("button", { name: `${label} reader identification…` }));
      const dialog = await screen.findByRole("dialog", { name: `${label} reader identification` });
      await user.click(within(dialog).getByRole("button", { name: verb }));
      await within(dialog).findByText(/Reader identification is/);
      await user.keyboard("{Escape}");
    };
    // Choose "Kill the process" while it can run.
    await readerIdentification("Turn on", "Turn on");
    await user.click(await screen.findByRole("row", { name: /backup-access\.env/ }));
    const panel = await screen.findByRole("dialog", { name: "Canary" });
    await user.click(await within(panel).findByRole("button", { name: "Change responses" }));
    await user.click(within(panel).getByRole("checkbox", { name: "Kill the process that touched it" }));
    await user.click(within(panel).getByRole("button", { name: "Save…" }));
    const saved = await screen.findByRole("dialog", { name: "Change what happens when a decoy is touched" });
    await user.click(within(saved).getByRole("button", { name: "Save" }));
    await within(saved).findByText(/uses them from the next access/);
    await user.keyboard("{Escape}");
    await user.keyboard("{Escape}");
    // Then take reader identification away: the choice stays visible, says it cannot run, and can be cleared.
    await readerIdentification("Turn off", "Turn off");
    expect(await screen.findByRole("row", { name: /Kill the process \(cannot run now\)/ })).toBeInTheDocument();
    await user.click(screen.getByRole("row", { name: /backup-access\.env/ }));
    const again = await screen.findByRole("dialog", { name: "Canary" });
    await user.click(await within(again).findByRole("button", { name: "Change responses" }));
    const kill = within(again).getByRole("checkbox", { name: "Kill the process that touched it" });
    expect(kill).toBeChecked();
    expect(kill).not.toHaveAttribute("aria-disabled");
    expect(kill).toHaveAccessibleDescription(/Chosen, but cannot run: reader identification is off/);
    await user.click(kill);
    expect(kill).not.toBeChecked();
    expect(kill).toHaveAttribute("aria-disabled", "true");
    expect(within(again).getByRole("button", { name: "Save…" })).toBeEnabled();
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

  it("offers no mark on a row where two views disagree, and offers to check again instead", async () => {
    useShell.setState({ protectView: "running" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("button", { name: "Check the running system" }));
    const grid = await screen.findByRole("grid", { name: "Running system" });
    await user.click(within(grid).getByRole("row", { name: /0\.0\.0\.0:4444/ }));
    const panel = await screen.findByRole("dialog", { name: /0\.0\.0\.0:4444/ });
    expect(within(panel).queryByRole("button", { name: "Mark as known…" })).not.toBeInTheDocument();
    expect(within(panel).getByText("Cannot be marked as known")).toBeInTheDocument();
    expect(within(panel).queryByText("A mark holds while these stay the same")).not.toBeInTheDocument();
    const again = within(panel).getByRole("button", { name: "Check again" });
    await user.click(again);
    expect(await within(panel).findByText(/^Checked .*\. Still listed\.$/)).toBeInTheDocument();
    expect(again).toHaveFocus();
  });

  it("stops a program you choose after a preview, watches in the background, and says which program started it again", async () => {
    useShell.setState({ protectView: "running" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("button", { name: "Choose a program…" }));
    const panel = await screen.findByRole("dialog", { name: "Your programs" });
    const grid = await within(panel).findByRole("grid", { name: "Your programs" });
    // A program Bigman will not stop says why in place of its command, and its row explains instead of opening a stop.
    expect(within(grid).getByRole("row", { name: /kwin_wayland/ })).toHaveTextContent("Not offered: Your desktop session needs it.");
    expect(within(panel).getByText("5 of 7 can be stopped")).toBeInTheDocument();
    await user.click(within(grid).getByRole("row", { name: /kwin_wayland/ }));
    expect(await screen.findByText(/Bigman does not offer to stop this program\. Your desktop session needs it\./)).toBeInTheDocument();
    expect(screen.queryByRole("dialog", { name: "Stop and watch" })).not.toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Your programs" }));

    const list = await screen.findByRole("grid", { name: "Your programs" });
    expect(within(list).getByRole("row", { name: /kworker-update/ })).toHaveTextContent("process 31044 · /home/sam/.cache/.kworker-update --quiet");
    await user.click(within(list).getByRole("row", { name: /kworker-update/ }));
    const dialog = await screen.findByRole("dialog", { name: "Stop and watch" });
    expect(await within(dialog).findByText(/Ask kworker-update to stop, then watch for 60 seconds/)).toBeInTheDocument();
    expect(within(dialog).getByText(/^kill -TERM 31044 +# kworker-update, started at clock tick 3104407$/)).toBeInTheDocument();
    expect(within(dialog).getByText(/Unsaved work in it may be lost/)).toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Stop and watch" }));
    // The dialog does not hold you for the minute.
    expect(await within(dialog).findByText(/kworker-update was asked to stop\. Bigman watches for 60 seconds/)).toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Close dialog" }));
    const recent = await screen.findByRole("region", { name: "Stopped recently" }, { timeout: 20_000 });
    expect(await within(recent).findByText(/kworker-update was started again by update-helper/)).toBeInTheDocument();
    // Something else started it again, so there is nothing to start.
    expect(within(recent).queryByRole("button", { name: "Start it again…" })).not.toBeInTheDocument();
  });

  it("starts a stopped program again after a preview, once nothing else did", async () => {
    useShell.setState({ protectView: "running" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("button", { name: "Choose a program…" }));
    const grid = await screen.findByRole("grid", { name: "Your programs" });
    await user.click(within(grid).getByRole("row", { name: /syncthing/ }));
    const dialog = await screen.findByRole("dialog", { name: "Stop and watch" });
    await user.click(await within(dialog).findByRole("button", { name: "Stop and watch" }));
    await within(dialog).findByText(/syncthing was asked to stop/);
    await user.click(within(dialog).getByRole("button", { name: "Close dialog" }));
    const recent = await screen.findByRole("region", { name: "Stopped recently" }, { timeout: 20_000 });
    expect(await within(recent).findByText(/Nothing started syncthing again within 60 seconds/)).toBeInTheDocument();
    await user.click(await within(recent).findByRole("button", { name: "Start it again…" }, { timeout: 20_000 }));
    const again = await screen.findByRole("dialog", { name: "Start it again" });
    expect(await within(again).findByText(/systemd-run --user --collect --quiet/)).toBeInTheDocument();
    await user.click(within(again).getByRole("button", { name: "Start it again" }));
    expect(await within(again).findByText("syncthing is starting again.")).toBeInTheDocument();
  });

  it("offers Stop and watch on a program row, and says when it cannot be started again", async () => {
    useShell.setState({ protectView: "running" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("button", { name: "Check the running system" }));
    const grid = await screen.findByRole("grid", { name: "Running system" });
    await user.click(within(grid).getByRole("row", { name: /updater is running/ }));
    const panel = await screen.findByRole("dialog", { name: "updater is running and its file was deleted" });
    await user.click(await within(panel).findByRole("button", { name: "Stop and watch…" }));
    const dialog = await screen.findByRole("dialog", { name: "Stop and watch" });
    expect(await within(dialog).findByText(/Its program file was deleted, so Bigman cannot start it again afterwards/)).toBeInTheDocument();
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
