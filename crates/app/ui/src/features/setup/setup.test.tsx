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
