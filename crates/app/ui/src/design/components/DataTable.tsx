import {
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getSortedRowModel,
  useReactTable,
  type ColumnDef,
  type RowData,
  type SortingState,
} from "@tanstack/react-table";
import { useVirtualizer } from "@tanstack/react-virtual";
import { ArrowDown, ArrowUp } from "lucide-react";
import { useId, useRef, useState, type KeyboardEvent, type ReactNode } from "react";

declare module "@tanstack/react-table" {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars -- signature fixed by the library
  interface ColumnMeta<TData extends RowData, TValue> {
    /** Figures align right so a column reads down one margin. */
    align?: "right";
    mono?: boolean;
    /** Fixed width such as "6rem"; other columns share the rest. */
    width?: string;
  }
}

const DEFAULT_ROW_HEIGHT = 36;

interface DataTableProps<T> {
  label: string;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any -- column value types vary per column
  columns: ColumnDef<T, any>[];
  data: T[];
  getRowId: (row: T) => string;
  onOpen?: ((row: T) => void) | undefined;
  initialSorting?: SortingState;
  /** Text filter applied across all columns. */
  filter?: string | undefined;
  /** Shown when the filter leaves nothing; empty data uses EmptyState in the view. */
  noMatches?: ReactNode;
  /** Visible height in pixels; rows beyond it are virtualized. */
  height?: number;
  /** Fixed row height; use a taller row for two-line cells. */
  rowHeight?: number;
}

/** Sortable, filterable, virtualized and keyboard navigable. Rows open to evidence. */
export function DataTable<T>({
  label,
  columns,
  data,
  getRowId,
  onOpen,
  initialSorting = [],
  filter = "",
  noMatches = "Nothing matches this search.",
  height = 360,
  rowHeight = DEFAULT_ROW_HEIGHT,
}: DataTableProps<T>) {
  const [sorting, setSorting] = useState<SortingState>(initialSorting);
  const [active, setActive] = useState(0);
  const [focused, setFocused] = useState(false);
  const scroller = useRef<HTMLDivElement>(null);
  const base = useId();
  // TanStack Table returns functions that the React Compiler cannot memoize.
  // eslint-disable-next-line react-hooks/incompatible-library
  const table = useReactTable({
    data,
    columns,
    getRowId: (row) => getRowId(row),
    state: { sorting, globalFilter: filter },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
  });
  const rows = table.getRowModel().rows;
  const virtual = useVirtualizer({
    count: rows.length,
    getScrollElement: () => scroller.current,
    estimateSize: () => rowHeight,
    overscan: 12,
    initialRect: { width: 800, height },
  });
  const items = virtual.getVirtualItems();
  const before = items[0]?.start ?? 0;
  const after = virtual.getTotalSize() - (items[items.length - 1]?.end ?? 0);
  const current = Math.min(active, Math.max(0, rows.length - 1));
  const rowId = (index: number) => `${base}-row-${index}`;

  const onKeyDown = (event: KeyboardEvent<HTMLTableElement>) => {
    // Keys pressed on a header's sort button belong to that button.
    if (event.target !== event.currentTarget) return;
    const move = (to: number) => {
      event.preventDefault();
      const next = Math.max(0, Math.min(rows.length - 1, to));
      setActive(next);
      virtual.scrollToIndex(next, { align: "auto" });
    };
    if (event.key === "ArrowDown") move(current + 1);
    else if (event.key === "ArrowUp") move(current - 1);
    else if (event.key === "Home") move(0);
    else if (event.key === "End") move(rows.length - 1);
    else if (event.key === "PageDown") move(current + Math.floor(height / rowHeight));
    else if (event.key === "PageUp") move(current - Math.floor(height / rowHeight));
    else if (event.key === "Enter" && onOpen) {
      const row = rows[current];
      if (row) {
        event.preventDefault();
        onOpen(row.original);
      }
    }
  };

  return (
    <div
      ref={scroller}
      className="overflow-auto rounded-panel border border-rule bg-surface"
      style={{ maxHeight: height }}
    >
      <table
        role="grid"
        aria-label={label}
        aria-rowcount={rows.length + 1}
        tabIndex={0}
        aria-activedescendant={rows.length ? rowId(current) : undefined}
        onKeyDown={onKeyDown}
        onFocus={(e) => setFocused(e.target === e.currentTarget)}
        onBlur={() => setFocused(false)}
        className="w-full table-fixed border-collapse text-body focus-visible:outline-offset-[-2px]"
      >
        <colgroup>
          {table.getVisibleLeafColumns().map((c) => (
            <col key={c.id} style={c.columnDef.meta?.width ? { width: c.columnDef.meta.width } : undefined} />
          ))}
        </colgroup>
        <thead className="sticky top-0 z-10 bg-surface">
          {table.getHeaderGroups().map((group) => (
            <tr key={group.id} className="border-b border-rule-strong">
              {group.headers.map((header) => {
                const sorted = header.column.getIsSorted();
                const right = header.column.columnDef.meta?.align === "right";
                return (
                  <th
                    key={header.id}
                    scope="col"
                    aria-sort={sorted === "asc" ? "ascending" : sorted === "desc" ? "descending" : undefined}
                    className={`h-8 px-3 text-small font-semibold text-ink-muted ${right ? "text-right" : "text-left"}`}
                  >
                    {header.column.getCanSort() ? (
                      <button
                        type="button"
                        onClick={header.column.getToggleSortingHandler()}
                        className={`inline-flex cursor-pointer items-center gap-1 hover:text-ink ${right ? "flex-row-reverse" : ""}`}
                      >
                        {flexRender(header.column.columnDef.header, header.getContext())}
                        {sorted === "asc" && <ArrowUp aria-hidden size={12} />}
                        {sorted === "desc" && <ArrowDown aria-hidden size={12} />}
                      </button>
                    ) : (
                      flexRender(header.column.columnDef.header, header.getContext())
                    )}
                  </th>
                );
              })}
            </tr>
          ))}
        </thead>
        <tbody>
          {before > 0 && (
            <tr aria-hidden>
              <td colSpan={columns.length} style={{ height: before, padding: 0 }} />
            </tr>
          )}
          {items.map((item) => {
            const row = rows[item.index];
            if (!row) return null;
            // The keyboard cursor shows only while the table has focus.
            const selected = focused && item.index === current;
            return (
              <tr
                key={row.id}
                id={rowId(item.index)}
                aria-rowindex={item.index + 2}
                aria-selected={selected}
                onClick={() => {
                  setActive(item.index);
                  onOpen?.(row.original);
                }}
                className={`border-b border-rule last:border-b-0 ${onOpen ? "cursor-pointer" : ""} ${selected ? "bg-sunken" : "hover:bg-ground"}`}
                style={{ height: rowHeight }}
              >
                {row.getVisibleCells().map((cell) => {
                  const meta = cell.column.columnDef.meta;
                  return (
                    <td
                      key={cell.id}
                      title={typeof cell.getValue() === "string" ? cell.getValue<string>() : undefined}
                      className={`truncate px-3 ${meta?.align === "right" ? "text-right" : ""} ${meta?.mono ? "font-mono text-small" : ""}`}
                    >
                      {flexRender(cell.column.columnDef.cell, cell.getContext())}
                    </td>
                  );
                })}
              </tr>
            );
          })}
          {after > 0 && (
            <tr aria-hidden>
              <td colSpan={columns.length} style={{ height: after, padding: 0 }} />
            </tr>
          )}
          {rows.length === 0 && (
            <tr>
              <td colSpan={columns.length} className="px-3 py-6 text-ink-muted">
                {noMatches}
              </td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
}
