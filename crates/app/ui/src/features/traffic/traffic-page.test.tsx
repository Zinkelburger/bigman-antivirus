import { screen, within } from "@testing-library/react";
import { beforeEach, describe, expect, it } from "vitest";
import { renderWithDaemon } from "@/test-utils";
import { resetSlots } from "./traffic";
import { TrafficPage } from "./TrafficPage";

beforeEach(resetSlots);

describe("Traffic", () => {
  it("says which program moved the most and breaks the traffic down three ways", async () => {
    renderWithDaemon(<TrafficPage />);
    const lead = await screen.findByText(/moved the most data over TCP in the last 24 hours/);
    expect(lead).toHaveTextContent(/^(firefox|curl) moved the most data over TCP in the last 24 hours: .+ received, .+ sent\.$/);
    // What the figures leave out is said beside them, not only at the foot of the page.
    expect(screen.getByText(/^TCP only: .+ use UDP and are counted only in the interface charts below\.$/)).toBeInTheDocument();
    for (const heading of ["By program", "Programs", "Connected to", "Traffic type"]) {
      expect(screen.getByRole("heading", { name: new RegExp(`^${heading}`) })).toBeInTheDocument();
    }
    // The chart's colors are named beside it, never left to color alone.
    const legend = screen.getByRole("list", { name: "Chart colors" });
    expect(within(legend).getByText("firefox")).toBeInTheDocument();
    expect(within(legend).getByText("Other programs")).toBeInTheDocument();
    expect(screen.getByRole("slider", { name: /^Traffic by program.*peak/ })).toBeInTheDocument();
    // The ten minutes the daemon was down are named, and drawn as a break rather than as zero.
    const byProgram = screen.getByRole("region", { name: "By program" });
    expect(within(byProgram).getByText(/^Not recorded \d\d:\d\d–\d\d:\d\d$/)).toBeInTheDocument();
  });

  it("opens a program to the addresses and traffic types behind its figure", async () => {
    const { user } = renderWithDaemon(<TrafficPage />);
    await user.click(await screen.findByRole("button", { name: /^spotify: .+ received, .+ sent$/ }));
    const panel = await screen.findByRole("dialog", { name: "spotify" });
    const table = within(panel).getByRole("grid", { name: "Traffic behind spotify" });
    expect(within(table).getByText("35.186.224.24")).toBeInTheDocument();
    expect(within(table).getByText("Port 4070")).toBeInTheDocument();
    expect(within(table).getByText("443 Encrypted web")).toBeInTheDocument();
    expect(within(panel).getByText(/not a finding by itself/)).toBeInTheDocument();
  });

  it("names the program behind an address and whether the address is local", async () => {
    const { user } = renderWithDaemon(<TrafficPage />);
    await user.click(await screen.findByRole("button", { name: /^192\.168\.1\.42, syncthing: / }));
    const panel = await screen.findByRole("dialog", { name: "Traffic with 192.168.1.42" });
    expect(within(panel).getByText("Your local network")).toBeInTheDocument();
    expect(within(panel).getByText("Incoming port 22000")).toBeInTheDocument();
  });

  it("rings a program first seen on the network in the last day", async () => {
    renderWithDaemon(<TrafficPage />);
    expect(await screen.findByRole("button", { name: /^curl, first seen in the last day: / })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /^firefox: / })).toBeInTheDocument();
  });

  it("keeps a program's color when the range changes", async () => {
    const { user } = renderWithDaemon(<TrafficPage />);
    const swatch = (name: RegExp) => screen.getByRole("button", { name }).querySelector<HTMLElement>("span[aria-hidden]")?.style.background;
    await screen.findByRole("button", { name: /^firefox: / });
    const before = { firefox: swatch(/^firefox: /), spotify: swatch(/^spotify: /) };
    await user.selectOptions(screen.getByLabelText("Time range"), "5m");
    await screen.findByText(/in the last 5 minutes/);
    expect({ firefox: swatch(/^firefox: /), spotify: swatch(/^spotify: /) }).toEqual(before);
    expect(before.firefox).toMatch(/^var\(--series-\d\)$/);
  });

  it("moves through a list with the arrow keys from a single tab stop", async () => {
    const { user } = renderWithDaemon(<TrafficPage />);
    const first = await screen.findByRole("button", { name: /^firefox: / });
    const list = first.closest("ul");
    if (!list) throw new Error("list missing");
    const rows = within(list).getAllByRole("button");
    expect(rows.map((r) => r.tabIndex)).toEqual([0, -1, -1, -1, -1]);
    first.focus();
    await user.keyboard("{ArrowDown}{ArrowDown}");
    expect(rows[2]).toHaveFocus();
    await user.keyboard("{Home}");
    expect(rows[0]).toHaveFocus();
  });

  it("lists every program in a table when there are more than the page shows", async () => {
    const { user } = renderWithDaemon(<TrafficPage />);
    await user.click(await screen.findByRole("button", { name: /^Show all \d+ programs$/ }));
    const panel = await screen.findByRole("dialog", { name: "Programs by traffic" });
    const table = within(panel).getByRole("grid", { name: "Programs by traffic" });
    expect(within(table).getByText("System programs (root)")).toBeInTheDocument();
  });

  it("states what the counters cannot see", async () => {
    renderWithDaemon(<TrafficPage />);
    expect(await screen.findByText(/UDP is not counted per program/)).toBeInTheDocument();
  });

  it("still charts each interface and says what a gap means", async () => {
    renderWithDaemon(<TrafficPage />);
    expect(await screen.findByRole("img", { name: /^wlp2s0: received .*peak/ })).toBeInTheDocument();
    expect(screen.getByRole("img", { name: /^tun0:/ })).toBeInTheDocument();
    expect(screen.getByText("A break in a line is time Bigman was not running.")).toBeInTheDocument();
  });

  it("says so when nothing was recorded", async () => {
    renderWithDaemon(<TrafficPage />, "empty");
    expect(await screen.findByText(/No TCP traffic was recorded in this range/)).toBeInTheDocument();
    expect(screen.queryByText(/moved the most/)).toBeNull();
  });

  it("reports a collector that cannot read the counters as incomplete, not as quiet", async () => {
    renderWithDaemon(<TrafficPage />, "incomplete");
    expect(await screen.findByText("Traffic by program is not being counted")).toBeInTheDocument();
    expect(screen.getByText(/Operation not permitted/)).toBeInTheDocument();
  });
});
