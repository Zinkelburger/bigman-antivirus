import { screen, waitFor, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { useLive } from "@/ipc";
import { call } from "@/ipc/transport";
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

function Live() {
  useLive();
  return null;
}

describe("Activity timeline", () => {
  it("renders a denial as a plain sentence folded with its repeats", async () => {
    renderWithDaemon(<ActivityPage />);
    const row = await screen.findByRole("option", { name: /firefox, confined by the mozilla policy, tried to read \.ssh/ });
    expect(within(row).getByText(/seen 14 times, first on/)).toBeInTheDocument();
  });

  it("says when a source cannot read instead of showing silence", async () => {
    const { user } = renderWithDaemon(<ActivityPage />);
    expect(await screen.findByText(/^SELinux and AppArmor denials: not collected at \d\d:\d\d \(no permission to read\)\.$/)).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Details" }));
    const panel = await screen.findByRole("dialog", { name: "Not collected" });
    expect(within(panel).getByText(/needs membership of systemd-journal/)).toBeInTheDocument();
    await user.click(within(panel).getByRole("button", { name: "Join systemd-journal…" }));
    expect(await screen.findByText("pkexec usermod -aG systemd-journal sam")).toBeInTheDocument();
  });

  it("opens an event without a report to its evidence and rule", async () => {
    const { user } = renderWithDaemon(<ActivityPage />);
    await user.click(await screen.findByRole("option", { name: /container API is reachable/ }));
    const panel = await screen.findByRole("dialog", { name: "The container API is reachable on port 2375" });
    expect(within(panel).getByText(/dockerd \(pid 1893\)/)).toBeInTheDocument();
    expect(within(panel).getByText("net.listener.docker_api")).toBeInTheDocument();
  });

  it("adds pushed entries at the top and marks everything as seen", async () => {
    const { user } = renderWithDaemon(
      <>
        <Live />
        <ActivityPage />
      </>,
    );
    await screen.findByRole("option", { name: /container API is reachable/ });
    await call({ type: "pdf", path: "/home/sam/Downloads/statement.pdf" }, "report");
    expect(await screen.findByRole("option", { name: /Read the PDF \/home\/sam\/Downloads\/statement\.pdf/ })).toBeInTheDocument();
    const mark = screen.getByRole("button", { name: "Mark all as seen" });
    expect(mark).toBeEnabled();
    await user.click(mark);
    await waitFor(() => expect(mark).toBeDisabled());
  });

  it("filters to what is new since the last look and undoes a mark", async () => {
    const { user } = renderWithDaemon(<ActivityPage />);
    const onlyNew = await screen.findByRole("button", { name: /New since your last look/ });
    await user.click(onlyNew);
    const list = screen.getByRole("listbox", { name: "Activity" });
    const rows = within(list).getAllByRole("option");
    expect(rows.every((r) => r.textContent.includes("New since your last look"))).toBe(true);
    await user.click(screen.getByRole("button", { name: "Mark all as seen" }));
    expect(await screen.findByText(/^Marked \d+ entries as seen\.$/)).toBeInTheDocument();
    expect(onlyNew).toHaveAttribute("aria-pressed", "false");
    await user.click(screen.getByRole("button", { name: "Undo" }));
    await waitFor(() => expect(screen.getByRole("button", { name: "Mark all as seen" })).toBeEnabled());
  });

  it("keeps 10,000 entries scrollable by rendering only what is visible", async () => {
    renderWithDaemon(<ActivityPage />, "many");
    const list = await screen.findByRole("listbox", { name: "Activity" });
    const rendered = within(list).getAllByRole("option").length;
    expect(rendered).toBeGreaterThan(5);
    expect(rendered).toBeLessThan(120);
  });
});

describe("Install receipts", () => {
  it("opens a package transaction to its receipt, scripts and marked line", async () => {
    const { user } = renderWithDaemon(<ActivityPage />);
    await user.click(await screen.findByRole("option", { name: /dnf installed nix and nix-libs/ }));
    const panel = await screen.findByRole("dialog", { name: /dnf installed nix and nix-libs/ });
    expect(await within(panel).findByText("Findings to review: 1 finding.")).toBeInTheDocument();
    expect(within(panel).getByText("1 script, 1 marked line")).toBeInTheDocument();
    await user.click(within(panel).getByRole("button", { name: "Install scripts" }));
    const scripts = await screen.findByRole("dialog", { name: "Install scripts" });
    expect(within(scripts).getByText("receipt.hook.sensitive_path")).toBeInTheDocument();
  });
});
