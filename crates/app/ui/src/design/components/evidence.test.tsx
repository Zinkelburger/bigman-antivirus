import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { EvidenceBlock } from "./EvidenceBlock";

describe("EvidenceBlock", () => {
  it("folds a long script to the matched lines and a few around them, and unfolds on request", async () => {
    const script = Array.from({ length: 60 }, (_, i) => `echo line ${String(i + 1)}`).join("\n");
    render(<EvidenceBlock text={script} highlight={[30]} notes={{ 30: { text: "pkg.adds-source", tone: "review" } }} fold />);
    expect(screen.getByText("echo line 30")).toBeInTheDocument();
    expect(screen.getByText("echo line 27")).toBeInTheDocument();
    expect(screen.queryByText("echo line 26")).not.toBeInTheDocument();
    expect(screen.getByText("26 lines not shown")).toBeInTheDocument();
    expect(screen.getByText("27 more lines not shown")).toBeInTheDocument();
    await userEvent.click(screen.getByRole("button", { name: "Show all 60 lines" }));
    expect(screen.getByText("echo line 1")).toBeInTheDocument();
    expect(screen.getByText("echo line 60")).toBeInTheDocument();
  });

  it("does not fold short blocks or blocks without highlights", () => {
    render(<EvidenceBlock text={"a\nb\nc"} highlight={[2]} fold />);
    expect(screen.queryByRole("button", { name: /Show all/ })).not.toBeInTheDocument();
  });
});
