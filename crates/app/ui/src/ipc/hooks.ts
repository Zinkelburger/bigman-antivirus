import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useRef } from "react";
import type { Action } from "./generated/Action";
import type { NotificationSettings } from "./generated/NotificationSettings";
import type { Preferences } from "./generated/Preferences";
import type { Request } from "./generated/Request";
import type { SourceId } from "./generated/SourceId";
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

/** The skills and plugins an inventory report listed, with their files. */
export function useSkillItems(report: string | null) {
  return useQuery({
    queryKey: ["skill-items", report],
    queryFn: () => call({ type: "skill_items", report: report ?? "" }, "skill_items"),
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

/**
 * Settings that could harden this computer. The daemon reads them fresh each
 * time (about two seconds), so they refresh on focus and after any change only.
 */
export function useHardening() {
  return useQuery({
    queryKey: ["hardening"],
    queryFn: () => call({ type: "hardening" }, "hardening"),
    staleTime: 60_000,
    refetchInterval: (query) => poll(10 * 60_000)(query),
  });
}

/** The rule registry; it only changes with Bigman itself. */
export function useRules() {
  return useQuery({ queryKey: ["rules"], queryFn: () => call({ type: "rules" }, "rules"), staleTime: Infinity });
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
  { type: "doctor" | "scan" | "update_signatures" | "pdf" | "inspect" | "inspect_any" | "packages" | "lockfile" | "repo_check" | "package_sources" | "changes" | "skills" | "audit" | "yara" | "canary_test" | "advisories" }
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

/**
 * Timeline entries since `since`, newest first. Live pushes prepend to every
 * loaded range (see live.ts), so this refetches only as a fallback.
 */
export function useEvents(since: number, sources: SourceId[] = [], limit = 20_000) {
  return useQuery({
    queryKey: ["events", since, sources, limit],
    queryFn: () => call({ type: "events", since, until: null, sources, limit }, "events"),
    refetchInterval: (query) => poll(60_000)(query),
  });
}

/** One install receipt; it changes once, when its after-effects are attached. */
export function useReceipt(id: string) {
  return useQuery({ queryKey: ["receipt", id], queryFn: () => call({ type: "receipt", id }, "receipt"), staleTime: 60_000 });
}

/** What arrived since Activity was last marked as seen; kept current by pushes. */
export function useUnread() {
  return useQuery({
    queryKey: ["unread"],
    queryFn: () => call({ type: "unread" }, "unread"),
    refetchInterval: (query) => poll(60_000)(query),
  });
}

export function useMarkSeen() {
  const client = useQueryClient();
  return useMutation({
    mutationFn: (through: number) => call({ type: "mark_seen", through }, "unread"),
    onSuccess: (unread) => client.setQueryData(["unread"], unread),
  });
}

/**
 * Stored throughput over the last `seconds`, about `points` buckets per interface,
 * read up to the moment of each fetch so nothing the daemon recorded is missing.
 */
export function useSamples(seconds: number, points: number, refetchMs: number) {
  return useQuery({
    queryKey: ["samples", seconds, points],
    queryFn: () => {
      const until = Math.floor(Date.now() / 1000);
      return call({ type: "samples", since: until - seconds, until, points }, "samples");
    },
    refetchInterval: (query) => poll(refetchMs)(query),
    placeholderData: (previous) => previous,
  });
}

/** Connections and listeners right now. */
export function useNetwork() {
  return useQuery({
    queryKey: ["network"],
    queryFn: () => call({ type: "network" }, "network"),
    refetchInterval: (query) => poll(3_000)(query),
  });
}

/** Each live source's health. */
export function useSources() {
  return useQuery({
    queryKey: ["sources"],
    queryFn: () => call({ type: "sources" }, "sources"),
    refetchInterval: (query) => poll(10_000)(query),
  });
}

export function usePreferences() {
  return useQuery({ queryKey: ["preferences"], queryFn: () => call({ type: "preferences" }, "preferences") });
}

export function useSetPreferences() {
  const client = useQueryClient();
  return useMutation({
    mutationFn: (preferences: Preferences) => call({ type: "set_preferences", preferences }, "preferences"),
    onSuccess: (preferences) => client.setQueryData(["preferences"], preferences),
  });
}

export function useNotifications() {
  return useQuery({ queryKey: ["notifications"], queryFn: () => call({ type: "notifications" }, "notifications") });
}

export function useSetNotifications() {
  const client = useQueryClient();
  return useMutation({
    mutationFn: (settings: NotificationSettings) => call({ type: "set_notifications", settings }, "notifications"),
    onSuccess: (settings) => client.setQueryData(["notifications"], settings),
  });
}

/** Answers a previewed package transaction. */
export function decide(id: string, proceed: boolean) {
  return call({ type: "decide", id, proceed }, "done");
}

export function useOpenTool() {
  return useMutation({ mutationFn: (tool: string) => call({ type: "open_tool", tool }, "done") });
}
