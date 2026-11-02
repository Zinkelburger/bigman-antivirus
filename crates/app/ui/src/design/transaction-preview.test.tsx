import { screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { useLive } from "@/ipc";
import { renderWithDaemon } from "@/test-utils";
import { TransactionPreviewHost, usePreviews, WaitingIndicator } from "./transaction-preview";

const PREVIEW_ADVICE = "If you did not expect this, stop the transaction.";

function Live() {
  useLive();
  return null;
}

describe("Transaction preview", () => {
  it("shows a waiting transaction, keeps it when hidden and stops it on request", async () => {
    usePreviews.setState({ entries: [] });
    const { user } = renderWithDaemon(
      <>
        <Live />
        <TransactionPreviewHost />
        <WaitingIndicator />
      </>,
      "preview",
    );
    const dialog = await screen.findByRole("dialog", { name: "A package change is waiting" });
    expect(within(dialog).getByRole("heading", { name: "dnf wants to change 2 packages: nix, nix-libs" })).toBeInTheDocument();
    expect(within(dialog).getByText(/Goes ahead in \d+ seconds unless you stop it/)).toBeInTheDocument();
    expect(within(dialog).getByText(PREVIEW_ADVICE)).toBeInTheDocument();
    // Hiding keeps it waiting; the status strip reopens it.
    await user.click(within(dialog).getByRole("button", { name: "Hide" }));
    expect(screen.queryByRole("dialog")).toBeNull();
    await user.click(await screen.findByRole("button", { name: "Open" }));
    const reopened = await screen.findByRole("dialog", { name: "A package change is waiting" });
    expect(within(reopened).getAllByText("receipt.hook.sensitive_path").length).toBeGreaterThan(0);
    expect(within(reopened).getByText("An install script writes to a startup or authentication path", { selector: "span" })).toBeInTheDocument();
    await user.click(within(reopened).getByRole("button", { name: "Stop the transaction" }));
    expect(await within(reopened).findByText(/dnf stops the transaction/)).toBeInTheDocument();
    await user.click(within(reopened).getByRole("button", { name: "Close" }));
    expect(screen.queryByRole("dialog")).toBeNull();
  });
});
