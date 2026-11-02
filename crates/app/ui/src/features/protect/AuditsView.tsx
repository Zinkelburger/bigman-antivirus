import { ActionButton, CheckStatus, LastResult } from "@/design";
import { useCheck, useHistory, useToolGroups } from "@/ipc";

/** Optional upstream tools; their output is kept verbatim as evidence. */
export function AuditsView() {
  const audit = useCheck();
  const groups = useToolGroups();
  const last = useHistory().data?.find((r) => r.kind === "lynis integration" || r.kind === "osquery integration");
  const lynis = groups.data?.find((g) => g.id === "lynis")?.installed ?? false;
  const run = (tool: string) => audit.mutate({ type: "audit", tool, path: "/" });
  return (
    <div className="flex flex-col gap-5">
      {!audit.data && !audit.isPending && <LastResult report={last} empty="No audits have run yet." />}
      <section aria-label="System audits" className="flex flex-col gap-3">
        <p className="m-0 max-w-[56ch] text-ink-muted">
          These run installed tools as you, so checks that need administrator access are reported as not run.
          {!lynis && " Lynis is not installed; install it from Setup."}
        </p>
        <div className="flex gap-2">
          <ActionButton disabled={audit.isPending || !lynis} onClick={() => run("lynis")}>
            Lynis hardening audit
          </ActionButton>
          <ActionButton disabled={audit.isPending} onClick={() => run("osquery")}>
            osquery process list
          </ActionButton>
        </div>
      </section>
      <CheckStatus check={audit} running="Running the audit" />
    </div>
  );
}
