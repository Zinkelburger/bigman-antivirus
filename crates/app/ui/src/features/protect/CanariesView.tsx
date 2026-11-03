import type { ColumnDef } from "@tanstack/react-table";
import { useState } from "react";
import {
  ActionButton,
  ActionDialog,
  CheckStatus,
  DataTable,
  Disclosure,
  EmptyState,
  ago,
  IncompleteNotice,
  KeyValue,
  nowSeconds,
  PathPicker,
  plural,
  Skeleton,
  TextField,
  useDrilldown,
} from "@/design";
import { desktopShell, useCanaries, useCheck, type Canary } from "@/ipc";

const KIND = { "local-decoy": "Local decoy", "hosted-url": "Hosted shortcut" } as Record<string, string>;

const columns = (now: number): ColumnDef<Canary>[] => [
  { accessorKey: "path", header: "File", meta: { mono: true } },
  { id: "kind", header: "Kind", accessorFn: (c) => KIND[c.kind] ?? c.kind, meta: { width: "9rem" } },
  { id: "created", header: "Created", accessorFn: (c) => c.created, cell: ({ row }) => ago(row.original.created, now), meta: { width: "9rem" } },
];

/** Reads the canary live, so the panel reflects a disarm or a stopped monitor. */
function CanaryDetail({ id }: { id: string }) {
  const test = useCheck();
  const state = useCanaries().data;
  const canary = state?.canaries.find((c) => c.id === id);
  if (!state) return <Skeleton shape="rows" count={3} />;
  if (!canary) return <p className="m-0 text-ink-muted">This decoy is no longer watched. Its file was kept.</p>;
  const watched = state.monitor?.watched_ids.includes(id) ?? false;
  return (
    <div className="flex flex-col gap-4">
      <KeyValue
        items={[
          { label: "File", value: canary.path, mono: true },
          { label: "Kind", value: KIND[canary.kind] ?? canary.kind },
          { label: "Watched", value: watched ? "Yes" : "No; start monitoring to notice reads" },
          { label: "ID", value: canary.id, mono: true },
        ]}
      />
      <div className="flex gap-2">
        {canary.kind === "local-decoy" && (
          <ActionButton disabled={!watched || test.isPending} onClick={() => test.mutate({ type: "canary_test", id: canary.id })}>
            Test detection
          </ActionButton>
        )}
        <ActionDialog action={{ kind: "disarm_canary", id: canary.id }} label="Stop watching…" variant="quiet" />
      </div>
      <CheckStatus check={test} running="Reading the decoy and waiting for the monitor's receipt" />
    </div>
  );
}

export function CanariesView() {
  const state = useCanaries();
  const open = useDrilldown((s) => s.open);
  const [path, setPath] = useState<string | null>(null);
  const [hostedPath, setHostedPath] = useState("");
  const [hostedUrl, setHostedUrl] = useState("");
  const now = nowSeconds();
  if (state.isPending) return <Skeleton shape="rows" />;
  if (state.isError) return <IncompleteNotice title="Canaries could not be listed" errors={[state.error.message]} />;
  const { canaries, monitor, default_path } = state.data;
  const target = path ?? default_path;
  const browse = async () => {
    const chosen = await desktopShell().choosePath("file", "Choose where the decoy file goes");
    if (chosen) setPath(chosen);
  };
  return (
    <div className="flex flex-col gap-6">
      <section aria-label="Monitor" className="flex flex-col gap-3">
        <p className="m-0 max-w-[56ch] text-ink-muted">
          A canary is a decoy file with a fake token. While the monitor runs, Bigman notices when anything reads,
          changes or removes it. Backups, indexers and your own tests can trigger it too.
        </p>
        <KeyValue
          items={[
            {
              label: "Monitor",
              value: monitor ? `Running, watching ${monitor.watched} of ${plural(monitor.configured, "file")}` : "Stopped; reads are not noticed",
            },
          ]}
        />
        {monitor?.errors.map((e) => (
          <code key={e} className="font-mono text-small text-ink-muted">
            {e}
          </code>
        ))}
        <div>
          {monitor ? (
            <ActionDialog action={{ kind: "stop_monitor", session_only: false }} label="Stop monitoring…" />
          ) : (
            canaries.length > 0 && <ActionDialog action={{ kind: "start_monitor", session_only: false }} label="Start monitoring…" variant="primary" />
          )}
        </div>
      </section>

      <section aria-labelledby="new-canary" className="flex flex-col gap-3">
        <h2 id="new-canary" className="m-0 text-body font-semibold text-ink">
          Add a local canary
        </h2>
        <p className="m-0 max-w-[56ch] text-ink-muted">
          No account or network needed. Bigman creates the decoy, starts monitoring at login, then reads it once to
          prove detection works. Existing files are never overwritten.
        </p>
        <PathPicker label="Decoy file" value={target} onChange={setPath} onBrowse={() => void browse()} />
        <div>
          <ActionDialog action={{ kind: "arm_canary", path: target, session_only: false }} label="Create, monitor and test…" variant={canaries.length ? "secondary" : "primary"} disabled={!target} />
        </div>
      </section>

      <section aria-labelledby="canary-list" className="flex flex-col gap-2">
        <h2 id="canary-list" className="m-0 text-body font-semibold text-ink">
          Your canaries
        </h2>
        {canaries.length ? (
          <DataTable
            label="Canaries"
            columns={columns(now)}
            data={canaries}
            getRowId={(c) => c.id}
            onOpen={(c) => open({ title: "Canary", content: <CanaryDetail id={c.id} /> })}
            height={260}
          />
        ) : (
          <EmptyState>Decoy files you add are listed here with whether the monitor is watching them.</EmptyState>
        )}
      </section>

      <Disclosure summary="A hosted token from a provider">
        <div className="flex flex-col gap-3">
          <p className="m-0 max-w-[56ch] text-ink-muted">
            Only needed to notice a stolen token being used on another computer. Paste the HTTPS URL your provider
            gave you. Bigman writes a shortcut and does not visit the URL.
          </p>
          <PathPicker label="Shortcut file" value={hostedPath} onChange={setHostedPath} placeholder="/home/you/Documents/backup-access.url" />
          <TextField label="Token URL" value={hostedUrl} onChange={setHostedUrl} mono placeholder="https://" hint="HTTPS only, without embedded credentials." />
          <div>
            <ActionDialog action={{ kind: "create_hosted_canary", path: hostedPath, url: hostedUrl }} label="Save shortcut…" disabled={!hostedPath || !hostedUrl.startsWith("https://")} />
          </div>
        </div>
      </Disclosure>
    </div>
  );
}
