import { beforeEach, describe, expect, it } from "vitest";
import type { Flow, TrafficReport } from "@/ipc";
import { chartData, hostRows, isLocal, programKey, programLabel, programRows, resetSlots, serviceLabel, shortAddress, slotsFor, typeRows } from "./traffic";

const flow = (program: string | null, remote: string, port: number, rx: number, tx = 0, inbound = false): Flow => ({
  program,
  uid: program === null ? 0 : 1000,
  remote,
  port,
  inbound,
  rx,
  tx,
});

function report(overrides: Partial<TrafficReport> = {}): TrafficReport {
  return {
    since: 1_000,
    until: 1_040,
    own_uid: 1000,
    bucket: 10,
    at: [1_000, 1_010, 1_020, 1_030, 1_040],
    recorded: [true, false, true, true, true],
    series: [
      { program: "firefox", uid: 1000, rx: [100, 200, 300, 50, 9], tx: [0, 0, 100, 0, 0] },
      { program: "curl", uid: 1000, rx: [0, 50, 0, 0, 0], tx: [0, 0, 0, 0, 0] },
    ],
    rest_rx: [10, 0, 0, 0, 0],
    rest_tx: [0, 0, 0, 0, 0],
    programs: [
      { program: "firefox", uid: 1000, rx: 659, tx: 100, first_at: null },
      { program: "curl", uid: 1000, rx: 50, tx: 0, first_at: 1_020 },
      { program: null, uid: 0, rx: 10, tx: 0, first_at: null },
    ],
    flows: [],
    flows_omitted: 0,
    coverage: { checked: [], notes: [], skipped: [] },
    ...overrides,
  };
}

beforeEach(resetSlots);

describe("program colors", () => {
  it("keeps a program's hue when its rank changes and reuses the hue of one that left", () => {
    const first = slotsFor(["a", "b", "c"]);
    expect([first.get("a"), first.get("b"), first.get("c")]).toEqual([1, 2, 3]);
    // b drops out, c overtakes a, d arrives: a and c keep theirs, d takes b's.
    const second = slotsFor(["c", "a", "d"]);
    expect([second.get("a"), second.get("c"), second.get("d")]).toEqual([1, 3, 2]);
    expect(second.has("b")).toBe(false);
  });

  it("remembers hues between windows", () => {
    slotsFor(["a", "b"]);
    // What a fresh window finds: nothing in memory, the saved hues in storage.
    const saved = localStorage.getItem("bigman.traffic.hues");
    resetSlots();
    localStorage.setItem("bigman.traffic.hues", saved ?? "");
    expect(slotsFor(["b", "c"]).get("b")).toBe(2);
  });

  it("colors only the five that lead", () => {
    const slots = slotsFor(["a", "b", "c", "d", "e", "f", "g"]);
    expect(slots.size).toBe(5);
    expect(slots.has("f")).toBe(false);
  });
});

describe("chart layers", () => {
  it("draws rates, leaves off the interval still being collected and folds uncolored programs into the rest", () => {
    const r = report();
    const chart = chartData(r, slotsFor(["program:firefox"]));
    // 1,040 has not ended and 1,030 ended under eight seconds before the report: neither is drawn yet.
    expect(chart.at).toEqual([1_000, 1_010, 1_020]);
    expect(chart.layers.map((l) => l.label)).toEqual(["firefox", "Other programs"]);
    expect(chart.layers[0]?.values).toEqual([10, 20, 40]);
    expect(chart.layers[1]?.values).toEqual([1, 5, 0]);
    expect(chart.layers[1]?.slot).toBeNull();
    // What was not collected travels with the chart, so a gap is drawn as a break.
    expect(chart.recorded).toEqual([true, false, true]);
  });

  it("stacks in the order of the Programs list, whatever hue each holds", () => {
    const r = report({ rest_rx: [0, 0, 0, 0, 0] });
    // curl led an earlier ranking and holds the first hue; firefox leads this range.
    slotsFor(["program:curl"]);
    const chart = chartData(r, slotsFor(["program:curl", "program:firefox"]));
    expect(chart.layers.map((l) => [l.label, l.slot])).toEqual([["firefox", 2], ["curl", 1]]);
  });

  it("has no neutral band when every program has its own", () => {
    const r = report({ rest_rx: [0, 0, 0, 0, 0] });
    const chart = chartData(r, slotsFor(r.programs.map(programKey)));
    expect(chart.layers.map((l) => l.label)).toEqual(["firefox", "curl"]);
  });
});

describe("breakdowns", () => {
  const flows = [
    flow("firefox", "142.250.80.46", 443, 900, 20),
    flow("curl", "142.250.80.46", 443, 100),
    flow("firefox", "93.184.215.14", 80, 50),
    flow(null, "192.168.1.30", 22, 5, 40, true),
  ];

  it("names programs Bigman cannot name by whose they are", () => {
    const rows = programRows(report(), slotsFor(["program:firefox"]));
    expect(rows.map((r) => r.name)).toEqual(["firefox", "curl", "System programs (root)"]);
    expect(rows.map((r) => r.slot)).toEqual([1, null, null]);
    // curl first reached the network twenty seconds before the report; nothing is recorded for the others.
    expect(rows.map((r) => r.firstSeen)).toEqual([false, true, false]);
    // A program of your own account that a sandbox hides is yours, not another account's.
    expect(programLabel({ program: null, uid: 1000 }, 1000)).toBe("Your programs Bigman cannot name");
    expect(programLabel({ program: null, uid: 980 }, 1000)).toBe("Another account's programs (uid 980)");
  });

  it("adds addresses up across programs and names the one behind most of each", () => {
    const rows = hostRows(flows, slotsFor(["program:firefox"]), 1000);
    // Two programs share this address, so its bar wears neither's hue.
    expect(rows[0]).toMatchObject({ name: "142.250.80.46", received: 1_000, sent: 20, detail: "firefox and 1 more", slot: null });
    expect(rows[1]).toMatchObject({ name: "93.184.215.14", detail: "firefox", slot: 1 });
    expect(rows[2]).toMatchObject({ name: "192.168.1.30", detail: "System programs (root)", slot: null });
  });

  it("groups by service port and says which connections came in", () => {
    const rows = typeRows(flows);
    expect(rows.map((r) => r.name)).toEqual(["443 Encrypted web", "80 Web, not encrypted", "Incoming 22 SSH"]);
    expect(serviceLabel(5228, false)).toBe("Port 5228");
  });

  it("tells local addresses from the internet", () => {
    expect(["192.168.1.9", "10.2.35.27", "172.20.0.4", "100.90.43.58", "fd7a:115c::1", "fe80::1"].every(isLocal)).toBe(true);
    expect(["142.250.80.46", "172.253.122.138", "2606:4700::6810:84e5"].some(isLocal)).toBe(false);
    expect(shortAddress("93.184.215.14")).toBe("93.184.215.14");
    expect(shortAddress("2001:0db8:85a3:0000:0000:8a2e:0370:7334")).toBe("2001:0db8:85a3…0370:7334");
  });
});
