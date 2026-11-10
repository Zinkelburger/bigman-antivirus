import type { ColumnDef } from "@tanstack/react-table";
import { KeyRound } from "lucide-react";
import { useEffect, useId, useMemo, useRef, useState, type ReactNode, type Ref } from "react";
import {
  ActionButton,
  ActionDialog,
  Checkbox,
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
import {
  desktopShell,
  useCanaries,
  useCanaryLog,
  useCheck,
  type Canary,
  type CanaryAccess,
  type CanaryResponses,
  type CanaryState,
} from "@/ipc";

const KIND = { "local-decoy": "Local decoy", "hosted-url": "Hosted shortcut" } as Record<string, string>;

const NO_RESPONSES: CanaryResponses = {
  lock_screen: false,
  log_out: false,
  network_off: false,
  kill_reader: false,
  lock_account: false,
  script: null,
};

/**
 * Why a response that depends on knowing the reader cannot run on this computer
 * as it is set up now. Logging out is one of them: a file manager showing the
 * decoy's folder reads the decoy, and only a named reader tells that apart.
 */
function blocked(state: CanaryState): { reader: string | null; lock: string | null } {
  const reader = state.helper_installed ? null : "reader identification is off";
  return { reader, lock: state.lock_unavailable ?? reader };
}

const NEEDS_READER = "Needs reader identification, under Monitor.";
const CHOSEN_WITHOUT_READER = "Chosen, but cannot run: reader identification is off. Clear it, or turn reader identification on under Monitor.";

/**
 * The chosen responses in the order the monitor runs them. One that cannot run
 * now says so; for a saved decoy that includes account locking an administrator
 * has not allowed yet (saving the choice again asks for that).
 */
function describe(r: CanaryResponses, state: CanaryState, saved = true): string[] {
  const off = blocked(state);
  const lockOff = off.lock !== null || (saved && !state.lock_allowed);
  return [
    r.script && `Run ${r.script}`,
    r.kill_reader && `Kill the process${off.reader ? " (cannot run now)" : ""}`,
    r.network_off && "Turn networking off",
    r.lock_account && `Lock your account${lockOff ? " (cannot run now)" : ""}`,
    r.lock_screen && "Lock the screen",
    r.log_out && `Log out${off.reader ? " (cannot run now)" : ""}`,
  ].filter((x): x is string => Boolean(x));
}

/** The same sentence wherever one line has to say it: the table and the form's summary. */
function inWords(r: CanaryResponses, state: CanaryState, saved = true): string {
  const chosen = describe(r, state, saved);
  return chosen.length ? ["Log and notify", ...chosen].join(", then ") : "Log and notify only";
}

const same = (a: CanaryResponses, b: CanaryResponses) => JSON.stringify(a) === JSON.stringify(b);

/** A ticked script box with no usable path cannot be saved: it would mean "no script" without saying so. */
const scriptReady = (r: CanaryResponses) => r.script === null || r.script.startsWith("/");

/**
 * A decoy as the table lists it. What happens when it is touched depends on
 * the monitor's state as well as the decoy, so it is part of the row: the table
 * caches a cell's value for as long as its row object stays the same.
 */
type Row = Canary & { touched: string };

const columns = (now: number): ColumnDef<Row>[] => [
  { accessorKey: "path", header: "File", meta: { mono: true } },
  { accessorKey: "touched", header: "When touched", meta: { width: "16rem" } },
  { id: "created", header: "Created", accessorFn: (c) => c.created, cell: ({ row }) => ago(row.original.created, now), meta: { width: "8rem" } },
];

/** Responses that belong together by what they cost, under one caption. */
function Group({ caption, note, children }: { caption: string; note?: string; children: ReactNode }) {
  const id = useId();
  return (
    <div role="group" aria-labelledby={id} aria-describedby={note ? `${id}-note` : undefined} className="flex flex-col gap-3">
      <div className="flex flex-col">
        <span id={id} className="text-small font-semibold text-ink-muted">
          {caption}
        </span>
        {note && (
          <span id={`${id}-note`} className="text-small text-ink-muted">
            {note}
          </span>
        )}
      </div>
      {children}
    </div>
  );
}

const Variable = ({ children }: { children: string }) => <code className="font-mono">{children}</code>;

/** What "Lock your account" says: why it cannot run, or exactly what it costs here. */
function lockAccountText(value: CanaryResponses, state: CanaryState): string {
  const off = blocked(state).lock;
  if (off) {
    if (!value.lock_account) return state.lock_unavailable ?? NEEDS_READER;
    return state.lock_unavailable ? `Chosen, but cannot run. ${state.lock_unavailable} Clear it.` : CHOSEN_WITHOUT_READER;
  }
  return [
    "Only when the process ran as you. Your password, sudo and SSH logins stop working until an administrator unlocks the account.",
    state.lock_warning,
    value.lock_account && (value.lock_screen || value.log_out) && "With Lock the screen or Log out also chosen, you cannot get back into this session either.",
    !state.lock_allowed && "Saving it asks for your password once, to allow the helper to lock this account.",
  ]
    .filter(Boolean)
    .join(" ");
}

interface ResponseChoicesProps {
  value: CanaryResponses;
  onChange: (r: CanaryResponses) => void;
  state: CanaryState;
  /** Hidden when a heading right above already says "When it is touched". */
  legend?: "shown" | "hidden";
  /** The first checkbox, for the caller to move focus to. */
  firstRef?: Ref<HTMLInputElement> | undefined;
}

/**
 * The responses a decoy runs when touched, as checkboxes in run order and
 * grouped by what they cost. A choice that needs reader identification cannot
 * be ticked while it is off; one ticked earlier can still be cleared.
 */
function ResponseChoices({ value, onChange, state, legend = "shown", firstRef }: ResponseChoicesProps) {
  const set = (patch: Partial<CanaryResponses>) => onChange({ ...value, ...patch });
  const off = blocked(state);
  const scriptOn = value.script !== null;
  const browse = async () => {
    const chosen = await desktopShell().choosePath("file", "Choose the script to run");
    if (chosen) set({ script: chosen });
  };
  return (
    <fieldset className="m-0 flex max-w-[60ch] min-w-0 flex-col gap-5 border-0 p-0">
      <legend className={legend === "hidden" ? "sr-only" : "mb-1 p-0 text-small font-semibold text-ink-muted"}>When it is touched</legend>
      <p className="m-0 text-small text-ink-muted">
        Always: the access goes in the decoy's log with a notification. Then, in this order, whatever you choose
        here. Bigman's own test read never runs them, and neither does a file manager, file chooser or search
        indexer that reader identification names.
      </p>
      <div className="flex flex-col gap-3">
        <Checkbox
          ref={firstRef}
          label="Run a script"
          checked={scriptOn}
          // An empty path while the box is ticked keeps Create and Save off until a script is named.
          onChange={(on) => set({ script: on ? "" : null })}
          description={
            <>
              Runs first, for up to 30 seconds. It is given <Variable>BIGMAN_CANARY_PATH</Variable>,{" "}
              <Variable>BIGMAN_CANARY_EVENTS</Variable> and, when known, <Variable>BIGMAN_READER_PID</Variable> and{" "}
              <Variable>BIGMAN_READER_USER</Variable>.
            </>
          }
        />
        {scriptOn && (
          <div className="pl-6">
            <PathPicker
              label="Script"
              value={value.script ?? ""}
              onChange={(script) => set({ script })}
              onBrowse={() => void browse()}
              placeholder="/home/you/bin/on-canary.sh"
              hint="The full path, starting with /. It must be executable, and changeable only by you."
              invalid={value.script !== "" && !scriptReady(value)}
            />
          </div>
        )}
      </div>
      <Group caption="Cut it off">
        <Checkbox
          label="Kill the process that touched it"
          checked={value.kill_reader}
          disabled={!value.kill_reader && off.reader !== null}
          onChange={(kill_reader) => set({ kill_reader })}
          description={
            off.reader === null
              ? "Only processes running as you. A backup tool would be killed too."
              : value.kill_reader
                ? CHOSEN_WITHOUT_READER
                : NEEDS_READER
          }
        />
        <Checkbox
          label="Turn networking off"
          checked={value.network_off}
          onChange={(network_off) => set({ network_off })}
          description="Disconnects the whole computer through NetworkManager, so nothing more leaves. Turn it back on from the network menu."
        />
      </Group>
      <Group caption="Lock yourself out" note="These act on your own login. A backup tool, or anything else that reads the decoy, sets them off too.">
        <Checkbox
          label="Lock your account"
          checked={value.lock_account}
          disabled={!value.lock_account && off.lock !== null}
          onChange={(lock_account) => set({ lock_account })}
          description={lockAccountText(value, state)}
        />
        <Checkbox
          label="Lock the screen"
          checked={value.lock_screen}
          onChange={(lock_screen) => set({ lock_screen })}
          description="Locks every session of yours."
        />
        <Checkbox
          label="Log out"
          checked={value.log_out}
          disabled={!value.log_out && off.reader !== null}
          onChange={(log_out) => set({ log_out })}
          description={
            off.reader === null
              ? "Runs last. Ends every session and program of yours, Bigman's monitor included, until you log back in. A decoy that logged you out waits ten minutes before it can do so again."
              : value.log_out
                ? CHOSEN_WITHOUT_READER
                : NEEDS_READER
          }
        />
      </Group>
    </fieldset>
  );
}

/** The steps a decoy runs, one per line in run order, with the script's path set as a path. */
function Steps({ responses, state }: { responses: CanaryResponses; state: CanaryState }) {
  return (
    <ol className="m-0 list-decimal pl-5">
      <li>Log and notify</li>
      {responses.script && (
        <li>
          Run <code className="font-mono text-small">{responses.script}</code>
        </li>
      )}
      {describe({ ...responses, script: null }, state).map((step) => (
        <li key={step}>{step}</li>
      ))}
    </ol>
  );
}

/** How many accesses the panel lists; older ones stay in the daemon's log. */
const LOG_SHOWN = 50;

function AccessLog({ id }: { id: string }) {
  const log = useCanaryLog(id);
  const now = nowSeconds();
  if (log.isPending) return <Skeleton shape="rows" count={2} />;
  if (log.isError) return <IncompleteNotice title="The access log could not be read" errors={[log.error.message]} />;
  if (!log.data.length) return <EmptyState>Nothing has touched this decoy since it was set up.</EmptyState>;
  return (
    <>
      <ol className="m-0 flex list-none flex-col gap-3 p-0">
        {log.data.slice(0, LOG_SHOWN).map((a) => (
          <AccessRow key={a.id} access={a} now={now} />
        ))}
      </ol>
      {log.data.length > LOG_SHOWN && (
        <p className="m-0 text-small text-ink-muted">
          Showing the latest {LOG_SHOWN} of {log.data.length} accesses.
        </p>
      )}
    </>
  );
}

function AccessRow({ access: a, now }: { access: CanaryAccess; now: number }) {
  return (
    <li className="flex flex-col gap-1 border-b border-rule pb-3 last:border-b-0">
      <span className="text-body text-ink">
        <time dateTime={new Date(a.time * 1000).toISOString()} title={new Date(a.time * 1000).toLocaleString()}>
          {ago(a.time, now)}
        </time>
        {": "}
        {a.kinds.join(", ")}
      </span>
      {a.readers.length ? (
        a.readers.map((r) => (
          <span key={r.pid} className="text-small text-ink-muted">
            By {r.name || "a process"} (PID {r.pid}
            {r.user ? `, user ${r.user}` : ""})
            {r.cmdline && <code className="ml-1 font-mono text-ink">{r.cmdline}</code>}
            {r.parents.length > 0 && <span className="block">Started by {r.parents.join(", started by ")}</span>}
          </span>
        ))
      ) : (
        <span className="text-small text-ink-muted">{a.self_test ? "Bigman's own test read." : "Process not identified."}</span>
      )}
      {a.skipped && !a.self_test && <span className="text-small text-ink-muted">Responses not run: {a.skipped}.</span>}
      {a.outcomes.map((o) => (
        <span key={o.action} className="text-small text-ink">
          <span className="font-semibold">{o.ok ? "Done" : "Failed"}:</span> {o.action}
          {o.detail && <span className="block whitespace-pre-wrap text-ink-muted">{o.detail}</span>}
        </span>
      ))}
    </li>
  );
}

/** Reads the canary live, so the panel reflects a disarm or a stopped monitor. */
function CanaryDetail({ id }: { id: string }) {
  const test = useCheck();
  const state = useCanaries().data;
  const canary = state?.canaries.find((c) => c.id === id);
  const [draft, setDraft] = useState<CanaryResponses | null>(null);
  // Entering and leaving the edit swaps the buttons that had focus, so focus is
  // carried: to the first choice going in, back to "Change responses" coming out.
  const first = useRef<HTMLInputElement>(null);
  const change = useRef<HTMLButtonElement>(null);
  const editing = draft !== null;
  const wasEditing = useRef(false);
  useEffect(() => {
    if (editing) first.current?.focus();
    else if (wasEditing.current) change.current?.focus();
    wasEditing.current = editing;
  }, [editing]);
  if (!state) return <Skeleton shape="rows" count={3} />;
  if (!canary) return <p className="m-0 text-ink-muted">This decoy is no longer watched. Its file was kept.</p>;
  const watched = state.monitor?.watched_ids.includes(id) ?? false;
  const identified = state.monitor?.helper.watched_ids.includes(id) ?? false;
  const local = canary.kind === "local-decoy";
  return (
    <div className="flex flex-col gap-4">
      <KeyValue
        items={[
          { label: "File", value: canary.path, mono: true },
          { label: "Kind", value: KIND[canary.kind] ?? canary.kind },
          { label: "Watched", value: watched ? "Yes" : "No; start monitoring to notice reads" },
          // A hosted shortcut is noticed by its provider, not by the monitor: it has no reader and no responses.
          ...(local
            ? [
                { label: "Reader named", value: identified ? "Yes" : "No; accesses are logged without the process" },
                { label: "When touched", value: <Steps responses={canary.responses} state={state} /> },
              ]
            : []),
          { label: "ID", value: canary.id, mono: true },
        ]}
      />
      {draft && <ResponseChoices value={draft} onChange={setDraft} state={state} firstRef={first} />}
      {/* One row for both sets of buttons, so focus after a Save has somewhere near to land. */}
      <div className="flex flex-wrap gap-2">
        {draft ? (
          <>
            <ActionDialog
              action={{ kind: "canary_responses", id, responses: draft }}
              label="Save…"
              variant="primary"
              disabled={same(draft, canary.responses) || !scriptReady(draft)}
              onDone={() => setDraft(null)}
            />
            <ActionButton variant="quiet" onClick={() => setDraft(null)}>
              Cancel
            </ActionButton>
          </>
        ) : (
          <>
            {local && (
              <ActionButton disabled={!watched || test.isPending} onClick={() => test.mutate({ type: "canary_test", id: canary.id })}>
                Test detection
              </ActionButton>
            )}
            {local && (
              <ActionButton ref={change} onClick={() => setDraft(canary.responses)}>
                Change responses
              </ActionButton>
            )}
            <ActionDialog action={{ kind: "disarm_canary", id: canary.id }} label="Stop watching…" variant="quiet" />
          </>
        )}
      </div>
      <CheckStatus check={test} running="Reading the decoy and waiting for the monitor's receipt" />
      <section aria-labelledby={`log-${id}`} className="flex flex-col gap-2">
        <h3 id={`log-${id}`} className="m-0 text-body font-semibold text-ink">
          Access log
        </h3>
        <AccessLog id={id} />
      </section>
    </div>
  );
}

export function CanariesView() {
  const state = useCanaries();
  const open = useDrilldown((s) => s.open);
  const [path, setPath] = useState<string | null>(null);
  const [responses, setResponses] = useState<CanaryResponses>(NO_RESPONSES);
  const [hostedPath, setHostedPath] = useState("");
  const [hostedUrl, setHostedUrl] = useState("");
  const now = nowSeconds();
  const data = state.data;
  const rows = useMemo<Row[]>(() => (data ? data.canaries.map((c) => ({ ...c, touched: inWords(c.responses, data) })) : []), [data]);
  if (state.isPending) return <Skeleton shape="rows" />;
  if (state.isError) return <IncompleteNotice title="Canaries could not be listed" errors={[state.error.message]} />;
  const { canaries, monitor, default_path, helper_installed } = state.data;
  const helper = monitor?.helper;
  const target = path ?? default_path;
  const browse = async () => {
    const chosen = await desktopShell().choosePath("file", "Choose where the decoy file goes");
    if (chosen) setPath(chosen);
  };
  const reader = !helper_installed
    ? "Off; accesses are logged without the process behind them"
    : helper?.connected
      ? "On; each access names the process and user"
      : monitor
        ? "Installed; the monitor has not connected yet"
        : "Installed; starts naming processes when monitoring runs";

  const add = (
    <section aria-labelledby="new-canary" className="flex flex-col gap-3">
      <h2 id="new-canary" className="m-0 text-body font-semibold text-ink">
        Add a local canary
      </h2>
      <p className="m-0 max-w-[60ch] text-ink-muted">
        How it works: Bigman writes the decoy, then asks the Linux kernel to report every read, change, move or
        removal of it (inotify watches its folder). Nothing is blocked; you find out right after. The kernel says
        that the file was touched, not what touched it. Reader identification, under Monitor, adds the process and
        user.
      </p>
      <p className="m-0 max-w-[60ch] text-ink-muted">
        No account or network needed. Bigman starts monitoring at login, then reads the decoy once to prove detection
        works. Existing files are never overwritten.
      </p>
      <PathPicker label="Decoy file" value={target} onChange={setPath} onBrowse={() => void browse()} />
      <Disclosure
        summary={
          <span className="min-w-0">
            When it is touched <span className="font-normal text-ink-muted">{inWords(responses, state.data, false)}</span>
          </span>
        }
      >
        <ResponseChoices value={responses} onChange={setResponses} state={state.data} legend="hidden" />
      </Disclosure>
      <div>
        <ActionDialog
          action={{ kind: "arm_canary", path: target, session_only: false, responses }}
          label="Create, monitor and test…"
          variant={canaries.length ? "secondary" : "primary"}
          disabled={!target || !scriptReady(responses)}
        />
      </div>
    </section>
  );

  const list = (
    <section aria-labelledby="canary-list" className="flex flex-col gap-2">
      <h2 id="canary-list" className="m-0 text-body font-semibold text-ink">
        Your canaries
      </h2>
      {canaries.length ? (
        <DataTable
          label="Canaries"
          columns={columns(now)}
          data={rows}
          getRowId={(c) => c.id}
          onOpen={(c) => open({ title: "Canary", content: <CanaryDetail id={c.id} /> })}
          height={260}
        />
      ) : (
        <EmptyState>Decoy files you add are listed here, with what happens when each is touched.</EmptyState>
      )}
    </section>
  );

  return (
    <div className="flex flex-col gap-6">
      <p className="m-0 max-w-[56ch] text-ink-muted">
        A canary is a decoy file with a fake token. While the monitor runs, Bigman notices when anything reads,
        changes or removes it. Backups, indexers, a file manager showing its folder and your own tests can trigger
        it too.
      </p>

      <section aria-labelledby="canary-monitor" className="flex flex-col gap-3">
        <h2 id="canary-monitor" className="m-0 text-body font-semibold text-ink">
          Monitor
        </h2>
        <KeyValue
          items={[
            {
              label: "Monitoring",
              value: monitor ? `Running, watching ${monitor.watched} of ${plural(monitor.configured, "file")}` : "Stopped; reads are not noticed",
            },
            { label: "Reader identification", value: reader },
          ]}
        />
        {[...(monitor?.errors ?? []), ...(helper?.errors ?? [])].map((e) => (
          <code key={e} className="font-mono text-small text-ink-muted">
            {e}
          </code>
        ))}
        <div className="flex flex-wrap items-center gap-2">
          {monitor ? (
            <ActionDialog action={{ kind: "stop_monitor", session_only: false }} label="Stop monitoring…" />
          ) : (
            canaries.length > 0 && <ActionDialog action={{ kind: "start_monitor", session_only: false }} label="Start monitoring…" variant="primary" />
          )}
          {helper_installed ? (
            <ActionDialog action={{ kind: "canary_helper", remove: true }} label="Turn off reader identification…" variant="quiet" describedBy="reader-password" />
          ) : (
            <ActionDialog action={{ kind: "canary_helper", remove: false }} label="Turn on reader identification…" describedBy="reader-password" />
          )}
          <span id="reader-password" className="inline-flex items-center gap-1 text-small text-ink-muted">
            <KeyRound aria-hidden size={14} />
            Asks for your password
          </span>
        </div>
      </section>

      {/* Once decoys exist, the page is for looking at them; adding another moves below. */}
      {canaries.length ? (
        <>
          {list}
          {add}
        </>
      ) : (
        <>
          {add}
          {list}
        </>
      )}

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
