import { formatTime, nowSeconds, signatureAge } from "@/design";
import { DisconnectedError, useHealth, useHistory, useStatus } from "@/ipc";

/** What is running, how old the signatures are, and when the last entry was made. */
export function StatusStrip() {
  const health = useHealth();
  const status = useStatus();
  const history = useHistory();
  const now = nowSeconds();
  const down = health.error instanceof DisconnectedError;
  const signatures = status.data?.signatures;
  const last = history.data?.[0];
  return (
    <>
      <span className="inline-flex items-center gap-1.5">
        <span aria-hidden className={`size-2 rounded-full ${down ? "border border-ink-muted" : health.data ? "bg-calm" : "bg-rule-strong"}`} />
        {down ? "Background checks are not running" : health.data ? "Background checks running" : "Connecting…"}
      </span>
      {status.data && <span>{status.data.monitor ? "Canary tripwires watched" : "Canary tripwires off"}</span>}
      {status.data && <span>{signatures ? `Signatures ${signatureAge(signatures.updated, now)}` : "No signatures yet"}</span>}
      {last && <span>Last entry {formatTime(last.created)}</span>}
    </>
  );
}
