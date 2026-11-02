import { screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
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
