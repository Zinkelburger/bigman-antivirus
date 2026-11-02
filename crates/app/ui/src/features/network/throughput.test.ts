import { describe, expect, it } from "vitest";
import { buildSeries, byTraffic } from "./throughput";

const s = (at: number, rx: number, iface = "eth0") => ({ at, iface, rx, tx: rx / 2 });

describe("buildSeries", () => {
  it("shows a bucket the daemon did not record as a gap", () => {
    const series = buildSeries({ bucket: 10, samples: [s(100, 1), s(110, 2), s(140, 5)] }, [], 100, 160).get("eth0");
    expect(series?.at).toEqual([100, 110, 120, 130, 140, 150, 160]);
    expect(series?.rx).toEqual([1, 2, null, null, 5, null, null]);
  });

  it("appends live samples after the stored range without duplicating", () => {
    const series = buildSeries({ bucket: 1, samples: [s(100, 1), s(101, 2)] }, [s(101, 9), s(102, 3), s(103, 4)], 100, 103).get("eth0");
    expect(series?.rx).toEqual([1, 2, 3, 4]);
  });

  it("never draws a line across a gap between stored and live samples", () => {
    const series = buildSeries({ bucket: 1, samples: [s(100, 1)] }, [s(110, 5)], 100, 110).get("eth0");
    expect(series?.rx).toEqual([1, null, 5]);
  });

  it("orders interfaces busiest first", () => {
    const series = buildSeries({ bucket: 1, samples: [s(1, 1, "tun0"), s(1, 50, "wlp2s0")] }, [], 1, 1);
    expect(byTraffic(series).map(([name]) => name)).toEqual(["wlp2s0", "tun0"]);
  });
});

describe("chart gaps", () => {
  it("names each stretch that was not recorded", async () => {
    const { gaps } = await import("@/design/components/ThroughputChart");
    const at = (h: number, m: number) => new Date(2026, 10, 2, h, m).getTime() / 1000;
    expect(gaps({ at: [at(13, 0), at(13, 5), at(13, 10), at(13, 15)], rx: [1, null, null, 4], tx: [1, null, null, 4] })).toEqual(["13:05–13:15"]);
    expect(gaps({ at: [at(13, 0), at(13, 5)], rx: [1, null], tx: [1, null] })).toEqual(["since 13:05"]);
  });
});
