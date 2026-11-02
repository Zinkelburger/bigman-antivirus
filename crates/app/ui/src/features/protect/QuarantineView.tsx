import type { ColumnDef } from "@tanstack/react-table";
import { ActionDialog, DataTable, EmptyState, ago, IncompleteNotice, KeyValue, nowSeconds, Skeleton, useDrilldown } from "@/design";
import { useQuarantined, type Quarantined } from "@/ipc";

const columns = (now: number): ColumnDef<Quarantined>[] => [
  { accessorKey: "original", header: "Original location", meta: { mono: true } },
  { id: "when", header: "Quarantined", accessorFn: (q) => q.created, cell: ({ row }) => ago(row.original.created, now), meta: { width: "9rem" } },
  { id: "state", header: "Status", accessorFn: (q) => (q.restored ? "Restored" : "In quarantine"), meta: { width: "8rem" } },
];

/** Reads the entry live, so the panel updates after a restore. */
function Detail({ id }: { id: string }) {
  const item = useQuarantined().data?.find((q) => q.id === id);
  if (!item) return <Skeleton shape="rows" count={3} />;
  return (
    <div className="flex flex-col gap-4">
      <KeyValue
        items={[
          { label: "Original location", value: item.original, mono: true },
          { label: "SHA-256", value: item.hash, mono: true },
          { label: "Status", value: item.restored ? "Restored; the recovery copy is kept" : "In quarantine" },
        ]}
      />
      {!item.restored && (
        <div>
          <ActionDialog action={{ kind: "restore", id: item.id }} label="Restore…" />
        </div>
      )}
    </div>
  );
}

export function QuarantineView() {
  const items = useQuarantined();
  const open = useDrilldown((s) => s.open);
  const now = nowSeconds();
  if (items.isPending) return <Skeleton shape="rows" />;
  if (items.isError) return <IncompleteNotice title="Quarantine could not be listed" errors={[items.error.message]} />;
  return (
    <div className="flex flex-col gap-3">
      <p className="m-0 max-w-[56ch] text-ink-muted">
        Recovery copies are private and not executable. Restoring writes the file back with permissions 0600 and
        never overwrites an existing file.
      </p>
      {items.data.length ? (
        <DataTable
          label="Quarantined files"
          columns={columns(now)}
          data={items.data}
          getRowId={(q) => q.id}
          initialSorting={[{ id: "when", desc: true }]}
          onOpen={(item) => open({ title: "Quarantined file", content: <Detail id={item.id} /> })}
        />
      ) : (
        <EmptyState>Files you quarantine from a scan are kept here, with a recovery copy you can restore.</EmptyState>
      )}
    </div>
  );
}
