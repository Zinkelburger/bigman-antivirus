// Every daemon-backed tab shows the same Disconnected state: what is wrong, the
// daemon's message verbatim, and one action that starts it (the desktop shell
// starts bigmand when a request cannot connect).
import { useQueryClient } from "@tanstack/react-query";
import type { ReactNode } from "react";
import { DisconnectedError, useHealth } from "@/ipc";
import { ActionButton } from "./components/ActionButton";
import { IncompleteNotice } from "./components/IncompleteNotice";
import { Skeleton } from "./components/Skeleton";

export function DaemonGate({ children }: { children: ReactNode }) {
  const health = useHealth();
  const client = useQueryClient();
  if (health.error instanceof DisconnectedError) {
    return (
      <IncompleteNotice
        kind="disconnected"
        errors={[health.error.message]}
        action={<ActionButton onClick={() => void client.invalidateQueries()}>Start background checks</ActionButton>}
      />
    );
  }
  if (health.isPending) return <Skeleton shape="rows" count={4} />;
  return <>{children}</>;
}
