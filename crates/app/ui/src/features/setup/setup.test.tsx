import { screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { mockShell } from "@/mock/transport";
import { renderWithDaemon } from "@/test-utils";
import { SetupPage } from "./SetupPage";

describe("Setup", () => {
  it("previews a tool installation before anything runs", async () => {
    const { user } = renderWithDaemon(<SetupPage />);
    const lynis = (await screen.findByText("Lynis")).closest("li");
    if (!lynis) throw new Error("Lynis row missing");
    await user.click(within(lynis).getByRole("button", { name: "Install…" }));
    const dialog = await screen.findByRole("dialog", { name: "Install optional tools" });
    expect(await within(dialog).findByText(/pkexec dnf install -y lynis/)).toBeInTheDocument();
  });

  it("accepts only a valid extension ID", async () => {
    const { user } = renderWithDaemon(<SetupPage />);
    const connect = await screen.findByRole("button", { name: "Connect…" });
    expect(connect).toBeDisabled();
    await user.type(screen.getByLabelText("Extension ID"), "abcdefghijklmnopabcdefghijklmnop");
    expect(connect).toBeEnabled();
  });
});

describe("Setup notifications", () => {
  it("keeps findings on by default and saves a category change", async () => {
    const { user } = renderWithDaemon(<SetupPage />);
    const findings = await screen.findByRole("switch", { name: /Findings to review/ });
    expect(findings).toHaveAttribute("aria-checked", "true");
    const connections = screen.getByRole("switch", { name: /New connections/ });
    expect(connections).toHaveAttribute("aria-checked", "false");
    await user.click(connections);
    expect(await screen.findByRole("switch", { name: /New connections/ })).toHaveAttribute("aria-checked", "true");
  });

  it("shows which sources cannot read and why", async () => {
    renderWithDaemon(<SetupPage />);
    expect(await screen.findByText(/not collected \(no permission to read\)/)).toBeInTheDocument();
  });
});

describe("Setup checks that run on their own", () => {
  it("turns the Downloads watch off and shows it as off, not as a gap", async () => {
    const { user } = renderWithDaemon(<SetupPage />);
    const row = (await screen.findByText("New files in Downloads")).closest("li");
    if (!row) throw new Error("Downloads source missing");
    expect(within(row).getByText(/read /)).toBeInTheDocument();
    await user.click(screen.getByRole("switch", { name: /Check new downloads/ }));
    expect(await within(row).findByText(/· off/)).toBeInTheDocument();
  });

  it("stops background checks from Setup, after saying what stops and what does not", async () => {
    const stop = vi.spyOn(mockShell, "stopAndQuit");
    const { user } = renderWithDaemon(<SetupPage />);
    await user.click(await screen.findByRole("button", { name: "Stop background checks…" }));
    const dialog = await screen.findByRole("dialog", { name: "Stop background checks" });
    expect(within(dialog).getByText(/new downloads\s+and project folders are not checked/)).toBeInTheDocument();
    expect(stop).not.toHaveBeenCalled();
    await user.click(within(dialog).getByRole("button", { name: "Stop and quit Bigman" }));
    expect(stop).toHaveBeenCalledOnce();
    stop.mockRestore();
  });

  it("does not claim the network is used only when you ask", async () => {
    renderWithDaemon(<SetupPage />);
    expect(await screen.findByText(/daily security-update check/)).toBeInTheDocument();
    expect(screen.queryByText(/only checks you start/)).not.toBeInTheDocument();
  });
});

describe("Setup receipt preview", () => {
  it("previews the hook file before installing it, and offers to turn it off after", async () => {
    const { user } = renderWithDaemon(<SetupPage />);
    await user.click(await screen.findByRole("button", { name: "Turn on…" }));
    const dialog = await screen.findByRole("dialog", { name: "Preview package changes before they happen" });
    expect(await within(dialog).findByText(/pre_transaction:::raise_error=1:\/usr\/bin\/bigman receipt --preview --manager dnf/)).toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Install the hook" }));
    expect(await within(dialog).findByText(/Installed the hook/)).toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Close" }));
    expect(await screen.findByRole("button", { name: "Turn off…" })).toBeInTheDocument();
  });
});
