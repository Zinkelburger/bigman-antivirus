import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { call, DisconnectedError } from "./transport";

// While the daemon is unreachable, poll every few seconds so views recover quickly.
const RECONNECT_MS = 3_000;

export function useHealth() {
  return useQuery({
    queryKey: ["health"],
    queryFn: () => call({ type: "health" }, "health"),
    refetchInterval: (query) => (query.state.error instanceof DisconnectedError ? RECONNECT_MS : 5_000),
  });
}

export function useStatus() {
  return useQuery({
    queryKey: ["status"],
    queryFn: () => call({ type: "status" }, "status"),
    refetchInterval: (query) => (query.state.error instanceof DisconnectedError ? RECONNECT_MS : 30_000),
  });
}

/** Recent reports, newest first (history summaries; full evidence comes by id). */
export function useHistory(limit = 200) {
  return useQuery({
    queryKey: ["history", limit],
    queryFn: () => call({ type: "history", limit }, "history"),
    refetchInterval: (query) => (query.state.error instanceof DisconnectedError ? RECONNECT_MS : 15_000),
  });
}

/** Runs the read-only checkup in the daemon; history and status refresh afterwards. */
export function useDoctor() {
  const client = useQueryClient();
  return useMutation({
    mutationFn: () => call({ type: "doctor" }, "report"),
    onSettled: () => client.invalidateQueries({ predicate: (q) => ["history", "status"].includes(String(q.queryKey[0])) }),
  });
}
