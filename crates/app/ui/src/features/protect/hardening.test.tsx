import { screen, waitFor, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { useProtectView } from "@/shell/protect-view";
import { renderWithDaemon } from "@/test-utils";
import { ProtectPage } from "./ProtectPage";

describe("Hardening", () => {
  it("opens first in Protect and leads with what to review", async () => {
    useProtectView.setState({ view: "hardening" });
    renderWithDaemon(<ProtectPage />);
    expect(await screen.findByText("Suggestions to review: 3, plus 2 optional.")).toBeInTheDocument();
    const list = screen.getByRole("list", { name: "To review" });
    const rows = within(list).getAllByRole("listitem");
    expect(rows).toHaveLength(3);
    expect(rows[0]).toHaveTextContent("Turn off the SSH server");
    expect(within(rows[0] as HTMLElement).getByRole("button", { name: "Turn off…" })).toBeInTheDocument();
    expect(within(screen.getByRole("list", { name: "Optional" })).getAllByRole("listitem")).toHaveLength(2);
    expect(screen.getByRole("tab", { name: "Hardening" })).toHaveAttribute("aria-selected", "true");
  });

  it("gives every suggestion a heading under its group, and says which changes ask for a password", async () => {
    useProtectView.setState({ view: "hardening" });
    renderWithDaemon(<ProtectPage />);
    const group = await screen.findByRole("heading", { level: 2, name: "To review" });
    expect(group).toBeInTheDocument();
    expect(screen.getByRole("heading", { level: 3, name: "Turn off the SSH server" })).toBeInTheDocument();
    const rows = within(screen.getByRole("list", { name: "To review" })).getAllByRole("listitem");
    // sshd needs polkit; the screen lock is changed as you.
    expect(rows[0]).toHaveTextContent("Asks for your password");
    expect(rows[2]).toHaveTextContent("Lock the screen");
    expect(rows[2]).not.toHaveTextContent("Asks for your password");
  });

  it("previews the exact commands, makes the change and offers Undo", async () => {
    useProtectView.setState({ view: "hardening" });
    const { user } = renderWithDaemon(<ProtectPage />);
    const ssh = await screen.findByRole("button", { name: "Turn off…", description: "Turn off the SSH server" });
    await user.click(ssh);
    const dialog = await screen.findByRole("dialog", { name: "Turn off the SSH server" });
    expect(await within(dialog).findByText(/systemctl disable --now sshd\.service/)).toBeInTheDocument();
    // Who runs it is prose before the commands, not a line inside them.
    const who = within(dialog).getByText("Polkit asks for your password, then these commands run as administrator.");
    expect(who.tagName).toBe("P");
    await user.click(within(dialog).getByRole("button", { name: "Turn off" }));
    expect(await within(dialog).findByText(/done\. Bigman checked again/)).toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Undo" }));
    expect(await within(dialog).findByText(/Put back as it was before/)).toBeInTheDocument();
  });

  it("lists a change Bigman made with an Undo that previews first", async () => {
    useProtectView.setState({ view: "hardening" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("button", { name: "Change…" }));
    const dialog = await screen.findByRole("dialog", { name: "Stop programs from reading each other's memory" });
    await user.click(await within(dialog).findByRole("button", { name: "Change" }));
    await within(dialog).findByText(/done\./);
    await user.keyboard("{Escape}");
    const changed = await screen.findByRole("list", { name: "Changed by Bigman" });
    await user.click(within(changed).getByRole("button", { name: "Undo…" }));
    const undo = await screen.findByRole("dialog", { name: "Undo: Stop programs from reading each other's memory" });
    expect(await within(undo).findByText(/rm -f -- \/etc\/sysctl\.d\/90-bigman-ptrace-scope\.conf/)).toBeInTheDocument();
  });

  it("returns focus to the next suggestion when the fixed one leaves the list", async () => {
    useProtectView.setState({ view: "hardening" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("button", { name: "Change…", description: "Stop programs from reading each other's memory" }));
    const dialog = await screen.findByRole("dialog", { name: "Stop programs from reading each other's memory" });
    await user.click(await within(dialog).findByRole("button", { name: "Change" }));
    await within(dialog).findByText(/done\./);
    await waitFor(() => expect(screen.queryByRole("heading", { level: 3, name: "Stop programs from reading each other's memory" })).not.toBeInTheDocument());
    await user.click(within(dialog).getByRole("button", { name: "Close" }));
    await waitFor(() => expect(document.activeElement).toBe(screen.getByRole("button", { name: "Turn on…", description: "Lock the screen when you step away" })));
  });

  it("explains a manual step and leaves a setting as it is on request", async () => {
    useProtectView.setState({ view: "hardening" });
    const { user } = renderWithDaemon(<ProtectPage />);
    await user.click(await screen.findByRole("button", { name: "How to do it", description: "Turn on Secure Boot" }));
    const panel = await screen.findByRole("dialog", { name: "Turn on Secure Boot" });
    expect(within(panel).getByText("How to do it")).toBeInTheDocument();
    expect(within(panel).getByText(/mokutil --import/)).toBeInTheDocument();
    await user.click(within(panel).getByRole("button", { name: "Leave as it is…" }));
    // The dialog names the setting and changes nothing on the computer, so it lists no commands.
    const dialog = await screen.findByRole("dialog", { name: "Stop suggesting: Turn on Secure Boot" });
    expect(await within(dialog).findByText(/Nothing on this computer changes/)).toBeInTheDocument();
    expect(within(dialog).queryByText("Exactly what changes")).not.toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Stop suggesting" }));
    expect(await within(dialog).findByText(/no longer counts it/)).toBeInTheDocument();
    await user.keyboard("{Escape}");
    const kept = await screen.findByRole("list", { name: "Left as it is" });
    expect(within(kept).getByText("Turn on Secure Boot")).toBeInTheDocument();
  });

  it("says so calmly when nothing is suggested", async () => {
    useProtectView.setState({ view: "hardening" });
    renderWithDaemon(<ProtectPage />, "calm");
    expect(await screen.findByText(/No findings from these checks/)).toBeInTheDocument();
    expect(screen.queryByRole("list", { name: "To review" })).not.toBeInTheDocument();
  });

  it("names what could not be checked", async () => {
    useProtectView.setState({ view: "hardening" });
    renderWithDaemon(<ProtectPage />, "incomplete");
    expect(await screen.findByText("1 setting could not be checked")).toBeInTheDocument();
    expect(screen.getByText(/nftables\.service manages the firewall/)).toBeInTheDocument();
    // What could not be checked is not also listed as already set.
    expect(screen.getByRole("heading", { level: 2, name: "Already set (6)" })).toBeInTheDocument();
  });
});
