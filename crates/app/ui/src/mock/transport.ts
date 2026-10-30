import { DisconnectedError, type Request, type Response, type Transport } from "@/ipc";
import { checkup, history, NOW, statusScenarios } from "./fixtures";

/**
 * Fixture-backed daemon for `npm run dev:browser` and tests. The scenario comes
 * from `?scenario=` (findings, calm, incomplete, empty, disconnected, loading).
 */
export function mockTransport(scenario = "findings", latencyMs = 250): Transport {
  return {
    async request(request: Request): Promise<Response> {
      if (scenario === "loading") return new Promise(() => undefined);
      await new Promise((resolve) => setTimeout(resolve, latencyMs));
      if (scenario === "disconnected") {
        throw new DisconnectedError("Bigman daemon is not running at /run/user/1000/bigman/bigmand.sock");
      }
      switch (request.type) {
        case "health":
          return {
            type: "health",
            value: { version: "0.2.0", pid: 4100, started: NOW - 3_600, state_dir: "/home/sam/.local/share/bigman" },
          };
        case "status": {
          const value = statusScenarios[scenario] ?? statusScenarios.findings;
          if (!value) throw new Error("missing fixture");
          return { type: "status", value };
        }
        case "doctor":
          await new Promise((resolve) => setTimeout(resolve, 1_200));
          return { type: "report", value: checkup() };
        case "history":
          return { type: "history", value: scenario === "empty" ? [] : history(scenario).slice(0, request.limit) };
      }
    },
  };
}

export function scenarioFromLocation(): string {
  return new URLSearchParams(window.location.search).get("scenario") ?? "findings";
}
