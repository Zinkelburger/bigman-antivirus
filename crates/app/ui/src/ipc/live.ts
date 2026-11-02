import { useQueryClient, type QueryClient } from "@tanstack/react-query";
import { useEffect } from "react";
import { create } from "zustand";
import type { Event } from "./generated/Event";
import type { Live } from "./generated/Live";
import type { Sample } from "./generated/Sample";
import { transport } from "./transport";

/** How much live throughput the UI keeps between stored-history refreshes. */
const LIVE_WINDOW = 10 * 60;
const RESUBSCRIBE_MS = 3_000;

interface LiveSamples {
  samples: Sample[];
  add(samples: Sample[]): void;
}

/** One-second samples pushed by the daemon, newest last, for the last ten minutes. */
export const useLiveSamples = create<LiveSamples>((set) => ({
  samples: [],
  add: (incoming) =>
    set((state) => {
      const latest = incoming.reduce((m, s) => Math.max(m, s.at), 0);
      return { samples: [...state.samples, ...incoming].filter((s) => s.at > latest - LIVE_WINDOW) };
    }),
}));

/** Puts a pushed entry at the top of every loaded timeline range it belongs to. */
function prepend(client: QueryClient, event: Event) {
  for (const [key, events] of client.getQueriesData<Event[]>({ queryKey: ["events"] })) {
    const [, since, sources] = key as [string, number, string[]];
    if (!events || event.at < since || (sources.length > 0 && !sources.includes(event.source))) continue;
    if (events.some((e) => e.id === event.id)) continue;
    client.setQueryData<Event[]>(key, [event, ...events]);
  }
}

function apply(client: QueryClient, live: Live) {
  switch (live.type) {
    case "event":
      prepend(client, live.value);
      break;
    case "unread":
      client.setQueryData(["unread"], live.value);
      break;
    case "samples":
      useLiveSamples.getState().add(live.value);
      break;
  }
}

/**
 * Keeps one subscription open for the whole app. After the daemon comes back it
 * resubscribes and reloads the timeline, so nothing pushed meanwhile is missed.
 */
export function useLive() {
  const client = useQueryClient();
  useEffect(() => {
    let stopped = false;
    let current: { stop(): void } | null = null;
    let retry: ReturnType<typeof setTimeout> | undefined;
    const start = (reload: boolean) => {
      if (stopped) return;
      const subscription = transport().subscribe({ events: true, samples: true }, (live) => apply(client, live));
      current = subscription;
      if (reload) void client.invalidateQueries({ queryKey: ["events"] });
      subscription.ended
        .catch(() => undefined)
        .finally(() => {
          if (!stopped) retry = setTimeout(() => start(true), RESUBSCRIBE_MS);
        });
    };
    start(false);
    return () => {
      stopped = true;
      clearTimeout(retry);
      current?.stop();
    };
  }, [client]);
}
