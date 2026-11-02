import { ActionButton, CheckStatus, LastResult } from "@/design";
import { useCheck, useHistory } from "@/ipc";

export function StartupView() {
  const compare = useCheck();
  const last = useHistory().data?.find((r) => r.kind === "Persistence changes");
  return (
    <div className="flex flex-col gap-5">
      {!compare.data && !compare.isPending && <LastResult report={last} empty="Startup files have not been compared yet." />}
      <section aria-label="Startup files" className="flex flex-col gap-3">
        <p className="m-0 max-w-[56ch] text-ink-muted">
          Compare the files that start programs for you (systemd units, autostart entries, shell profiles, cron)
          with the baseline you accepted. A baseline records what was there; it does not say those entries are
          trustworthy.
        </p>
        <div>
          <ActionButton variant="primary" disabled={compare.isPending} onClick={() => compare.mutate({ type: "changes" })}>
            Compare startup files
          </ActionButton>
        </div>
      </section>
      <CheckStatus check={compare} running="Comparing startup files" />
    </div>
  );
}
