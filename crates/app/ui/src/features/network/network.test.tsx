import { screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { renderWithDaemon } from "@/test-utils";
import { NetworkPage } from "./NetworkPage";

describe("Network", () => {
  it("lists the most exposed listener first and opens it to its findings", async () => {
    const { user } = renderWithDaemon(<NetworkPage />);
    const table = await screen.findByRole("grid", { name: "Listening programs" });
    const rows = within(table).getAllByRole("row").slice(1);
    expect(rows[0]).toHaveTextContent("dockerd");
    expect(rows[0]).toHaveTextContent("High");
    await user.click(rows[0] as HTMLElement);
    const panel = await screen.findByRole("dialog", { name: "dockerd on port 2375" });
    // Findings sit in the panel with their evidence and what to do, worst first.
    expect(within(panel).getByText("The container API is reachable on port 2375")).toBeInTheDocument();
    expect(within(panel).getByText(/Use the local socket instead/)).toBeInTheDocument();
  });

  it("marks connections first seen in the last day", async () => {
    renderWithDaemon(<NetworkPage />);
    const table = await screen.findByRole("grid", { name: "Your programs' connections" });
    const curl = within(table).getByText("curl").closest("tr");
    if (!(curl instanceof HTMLElement)) throw new Error("curl row missing");
    expect(within(curl).getByText("First seen")).toBeInTheDocument();
    const firefox = within(table).getAllByText("firefox")[0]?.closest("tr");
    if (!(firefox instanceof HTMLElement)) throw new Error("firefox row missing");
    expect(within(firefox).queryByText("First seen")).toBeNull();
  });
});

describe("Network connections", () => {
  it("opens a connection to what Bigman recorded about it", async () => {
    const { user } = renderWithDaemon(<NetworkPage />);
    const table = await screen.findByRole("grid", { name: "Your programs' connections" });
    await user.click(within(table).getByText("curl"));
    const panel = await screen.findByRole("dialog", { name: "curl to 93.184.215.14:443" });
    expect(within(panel).getByText("93.184.215.x")).toBeInTheDocument();
  });
});
