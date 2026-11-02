import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useRef } from "react";
import type { Action } from "./generated/Action";
import type { Request } from "./generated/Request";
import { call, DisconnectedError, transport } from "./transport";

// While the daemon is unreachable, poll every few seconds so views recover quickly.
const RECONNECT_MS = 3_000;
const poll = (ms: number) => (query: { state: { error: unknown } }) =>
  query.state.error instanceof DisconnectedError ? RECONNECT_MS : ms;

export function useHealth() {
  return useQuery({
    queryKey: ["health"],
    queryFn: () => call({ type: "health" }, "health"),
    refetchInterval: (query) => poll(5_000)(query),
  });
}

export function useStatus() {
  return useQuery({
    queryKey: ["status"],
    queryFn: () => call({ type: "status" }, "status"),
    refetchInterval: (query) => poll(30_000)(query),
  });
}

/** Recent reports, newest first (history summaries; full evidence comes by id). */
export function useHistory(limit = 200) {
  return useQuery({
    queryKey: ["history", limit],
    queryFn: () => call({ type: "history", limit }, "history"),
    refetchInterval: (query) => poll(15_000)(query),
  });
}

/** One report with its full evidence. */
export function useReport(id: string | null) {
  return useQuery({
    queryKey: ["report", id],
    queryFn: () => call({ type: "report", id: id ?? "" }, "report"),
    enabled: id !== null,
    staleTime: Infinity,
  });
}

export function useScanItems(report: string | null) {
  return useQuery({
    queryKey: ["scan-items", report],
    queryFn: () => call({ type: "scan_items", report: report ?? "" }, "scan_items"),
    enabled: report !== null,
  });
}

export function useQuarantined() {
  return useQuery({ queryKey: ["quarantined"], queryFn: () => call({ type: "quarantined" }, "quarantined") });
}

export function useCanaries() {
  return useQuery({
    queryKey: ["canaries"],
    queryFn: () => call({ type: "canaries" }, "canaries"),
    refetchInterval: (query) => poll(10_000)(query),
  });
}

export function useToolGroups() {
  return useQuery({ queryKey: ["tool-groups"], queryFn: () => call({ type: "tool_groups" }, "tool_groups") });
}

/** Everything a check or an action can change on screen. */
function useRefreshAll() {
  const client = useQueryClient();
  return () => client.invalidateQueries({ predicate: (q) => q.queryKey[0] !== "report" });
}

type CheckRequest = Extract<
  Request,
  { type: "doctor" | "scan" | "update_signatures" | "pdf" | "inspect" | "inspect_any" | "packages" | "changes" | "audit" | "yara" | "canary_test" }
>;

/**
 * Runs a check in the daemon and returns its report. `cancel()` stops it; the
 * cancelled report still arrives and says what was not finished.
 */
export function useCheck() {
  const refresh = useRefreshAll();
  const token = useRef<string | null>(null);
  const mutation = useMutation({
    mutationFn: (request: CheckRequest) => {
      token.current = crypto.randomUUID();
      return call(request, "report", token.current);
    },
    onSettled: () => {
      token.current = null;
      void refresh();
    },
  });
  return {
    ...mutation,
    cancel: () => {
      if (token.current) void transport().cancel(token.current);
    },
  };
}

/** What an action would change, fetched while its dialog is open and never cached. */
export function usePreview(action: Action, enabled: boolean) {
  return useQuery({
    queryKey: ["preview", action],
    queryFn: () => call({ type: "preview", action }, "plan"),
    enabled,
    gcTime: 0,
    staleTime: 0,
    retry: false,
  });
}

/** Apply state changes; every view refreshes afterwards. */
export function useAction() {
  const refresh = useRefreshAll();
  return {
    apply: async (action: Action) => {
      try {
        return await call({ type: "apply", action }, "done");
      } finally {
        void refresh();
      }
    },
  };
}

export function useOpenTool() {
  return useMutation({ mutationFn: (tool: string) => call({ type: "open_tool", tool }, "done") });
}
