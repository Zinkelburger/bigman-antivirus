import { ActionButton, CheckStatus, LastResult } from "@/design";
import { useCheck, useHistory } from "@/ipc";

const HOW = (
  <>
    Installed packages are compared with your distribution&apos;s own security data (Fedora, Debian or Arch Linux), and
    updates that fix a vulnerability known to be exploited come first. On Fedora, dnf refreshes its repository lists;
    on Debian and Arch, Bigman downloads the public security data, and on every system CISA&apos;s catalog of
    exploited vulnerabilities. No list of your packages is sent anywhere, and Bigman never installs anything.
  </>
);

export function UpdatesView() {
  const check = useCheck();
  const last = useHistory().data?.find((r) => r.kind === "Security updates");
  const run = (refresh: boolean) => check.mutate({ type: "advisories", refresh, offline: false });
  return (
    <div className="flex flex-col gap-5">
      {!check.data && !check.isPending && <LastResult report={last} empty="Security updates have not been checked yet." />}
      <section aria-label="Security updates" className="flex flex-col gap-3">
        {last || check.data ? (
          <details className="max-w-[56ch] text-small text-ink-muted">
            <summary className="cursor-pointer text-ink">How this works</summary>
            <p className="m-0 mt-2">{HOW}</p>
          </details>
        ) : (
          <p className="m-0 max-w-[56ch] text-ink-muted">{HOW}</p>
        )}
        <div className="flex flex-wrap items-center gap-2">
          <ActionButton variant="primary" disabled={check.isPending} onClick={() => run(false)}>
            Check for updates
          </ActionButton>
          <ActionButton variant="quiet" disabled={check.isPending} onClick={() => run(true)}>
            Re-download security data
          </ActionButton>
          <span className="text-small text-ink-muted">Bigman also checks daily and after package changes.</span>
        </div>
      </section>
      <CheckStatus check={check} running="Checking for security updates" detail="Reading installed packages and the distribution's security data." />
    </div>
  );
}
