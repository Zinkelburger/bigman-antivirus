import { Circle, CircleCheck, Dot } from "lucide-react";
import { useState, type ReactNode } from "react";
import {
  ActionButton,
  ActionDialog,
  applyTheme,
  AutomaticChecks,
  IncompleteNotice,
  PageHeader,
  PreviewDialog,
  Select,
  Skeleton,
  storedTheme,
  nowSeconds,
  signatureAge,
  TextField,
  Toggle,
  ago,
  formatTime,
  SKIP_REASON,
  SourceRemedy,
  type ThemeChoice,
} from "@/design";
import { useQueryClient } from "@tanstack/react-query";
import {
  desktopShell,
  DisconnectedError,
  useHealth,
  useNotifications,
  useOpenTool,
  useSetNotifications,
  useSources,
  useStatus,
  useToolGroups,
  type NotificationSettings,
} from "@/ipc";

/**
 * One line of the checklist. `done` is true or false for steps that can be
 * completed; null for settings and information, which get a neutral marker.
 */
function Item({ title, done, children }: { title: string; done: boolean | null; children: ReactNode }) {
  const Mark = done === null ? Dot : done ? CircleCheck : Circle;
  return (
    <li className="grid grid-cols-[1.25rem_minmax(0,1fr)] gap-x-3 border-b border-rule py-4 last:border-b-0">
      <Mark aria-hidden size={18} className={`mt-0.5 ${done ? "text-ink" : "text-ink-muted"}`} />
      <div className="flex min-w-0 flex-col gap-2">
        <h2 className="m-0 text-body font-semibold text-ink">
          {title}
          {done !== null && <span className="sr-only">{done ? " (done)" : " (not done)"}</span>}
        </h2>
        {children}
      </div>
    </li>
  );
}

const THEMES: readonly { value: ThemeChoice; label: string }[] = [
  { value: "system", label: "Follow the system" },
  { value: "light", label: "Light" },
  { value: "dark", label: "Dark" },
];

function Tools({ disabled }: { disabled: boolean }) {
  const groups = useToolGroups();
  const openTool = useOpenTool();
  if (disabled) return <p className="m-0 text-small text-ink-muted">Shown once background checks are running.</p>;
  if (groups.isPending) return <Skeleton shape="rows" count={3} />;
  if (groups.isError) return <IncompleteNotice title="Tools could not be listed" errors={[groups.error.message]} />;
  return (
    <ul className="m-0 flex list-none flex-col gap-2 p-0">
      {groups.data.map((g) => (
        <li key={g.id} className="flex min-h-8 flex-wrap items-center justify-between gap-x-4 gap-y-1">
          <span className="min-w-0">
            <span className="font-semibold text-ink">{g.label}</span>
            <span className="text-ink-muted"> · {g.purpose}</span>
          </span>
          {g.installed ? (
            g.id === "firewall" ? (
              <ActionButton variant="quiet" disabled={openTool.isPending} onClick={() => openTool.mutate("firewall")}>
                Open OpenSnitch
              </ActionButton>
            ) : (
              <span className="text-small text-ink-muted">Installed</span>
            )
          ) : (
            <ActionDialog action={{ kind: "install_tool", tool: g.id }} label="Install…" variant="quiet" />
          )}
        </li>
      ))}
    </ul>
  );
}

const CATEGORIES: readonly { key: keyof Omit<NotificationSettings, "paused_until">; label: string; description: string }[] = [
  { key: "findings", label: "Findings to review", description: "Anything rated Review or High, from any source." },
  { key: "new_connections", label: "New connections", description: "A program going online for the first time, or reaching a new address range." },
  { key: "new_listeners", label: "New listening programs", description: "A program starting to accept connections." },
  { key: "packages", label: "Package changes", description: "Installs, updates and removals." },
  { key: "denials", label: "Denials", description: "SELinux or AppArmor stopping a program." },
];

/** Which live source feeds each notification category. */
const SOURCE_OF: Partial<Record<string, string>> = {
  denials: "denials",
  packages: "packages",
  new_connections: "connections",
  new_listeners: "listeners",
};

function Notifications({ disabled }: { disabled: boolean }) {
  const settings = useNotifications();
  const sources = useSources();
  // A category whose source cannot read would never notify; say so beside its switch.
  const blind = new Map((sources.data ?? []).filter((s) => !s.ok).map((s) => [s.name, s.label]));
  const save = useSetNotifications();
  const now = nowSeconds();
  if (disabled) return <p className="m-0 text-small text-ink-muted">Shown once background checks are running.</p>;
  if (settings.isPending) return <Skeleton shape="rows" count={3} />;
  if (settings.isError) return <IncompleteNotice title="Notification settings could not be loaded" errors={[settings.error.message]} />;
  const current = settings.data;
  const paused = current.paused_until !== null && current.paused_until > now;
  return (
    <div className="flex flex-col gap-3">
      {paused && current.paused_until !== null && (
        <div className="flex flex-wrap items-center gap-3">
          <p className="m-0 text-ink">Paused until {formatTime(current.paused_until)}.</p>
          <ActionButton variant="quiet" onClick={() => save.mutate({ ...current, paused_until: null })}>
            Resume now
          </ActionButton>
        </div>
      )}
      {CATEGORIES.map((c) => (
        <Toggle
          key={c.key}
          label={c.label}
          description={blind.has(SOURCE_OF[c.key] ?? "") ? `${c.description} Not collected right now; see What is collected.` : c.description}
          checked={current[c.key]}
          disabled={save.isPending}
          onChange={(checked) => save.mutate({ ...current, [c.key]: checked })}
        />
      ))}
      {save.isError && <IncompleteNotice title="The change was not saved" errors={[save.error.message]} />}
    </div>
  );
}

function Sources({ disabled }: { disabled: boolean }) {
  const sources = useSources();
  const now = nowSeconds();
  if (disabled) return <p className="m-0 text-small text-ink-muted">Shown once background checks are running.</p>;
  if (sources.isPending) return <Skeleton shape="rows" count={3} />;
  if (sources.isError) return <IncompleteNotice title="Sources could not be listed" errors={[sources.error.message]} />;
  return (
    <ul className="m-0 flex list-none flex-col gap-2 p-0">
      {sources.data.map((source) => (
        <li key={source.name} className="flex flex-col">
          <span>
            <span className="font-semibold text-ink">{source.label}</span>
            <span className="text-ink-muted">
              {" · "}
              {source.off
                ? "off"
                : source.ok
                ? source.last_run === null
                  ? "starting"
                  : `${source.name === "updates" ? "checked" : "read"} ${ago(source.last_run, now)}`
                : `not collected (${SKIP_REASON[source.why ?? "failed"].toLowerCase()})`}
            </span>
          </span>
          {!source.ok && source.detail && <span className="max-w-[56ch] text-small text-ink-muted">{source.detail}</span>}
          {!source.ok && (
            <div className="mt-1">
              <SourceRemedy source={source} />
            </div>
          )}
        </li>
      ))}
    </ul>
  );
}

/** Stops bigmand and quits, since the open app would start it again. */
function StopChecks({ services }: { services: boolean }) {
  const [open, setOpen] = useState(false);
  return (
    <>
      <div>
        <ActionButton onClick={() => setOpen(true)}>
          Stop background checks…
        </ActionButton>
      </div>
      <PreviewDialog
        open={open}
        onOpenChange={setOpen}
        title="Stop background checks"
        confirmLabel="Stop and quit Bigman"
        plan={
          <div className="flex max-w-[56ch] flex-col gap-3 text-ink">
            <p className="m-0">
              bigmand stops and Bigman quits. Until you open Bigman again{services ? " or log in" : ""}, new downloads
              and project folders are not checked, Activity and Network record nothing, and the daily security-update
              check does not run.
            </p>
            {services && (
              <p className="m-0">
                Still running, because background services are installed: the canary and startup monitor and the daily
                ClamAV signature download. Remove the services to stop those too.
              </p>
            )}
            <p className="m-0 text-ink-muted">Reports, Activity history and canary files stay as they are.</p>
          </div>
        }
        onConfirm={async () => {
          await desktopShell().stopAndQuit();
          return "Background checks stopped.";
        }}
      />
    </>
  );
}

export function SetupPage() {
  const health = useHealth();
  const client = useQueryClient();
  const groups = useToolGroups();
  const status = useStatus();
  const [extension, setExtension] = useState("");
  const [theme, setTheme] = useState<ThemeChoice>(storedTheme);
  const validExtension = /^[a-p]{32}$/.test(extension);
  const down = health.error instanceof DisconnectedError;
  const clamav = groups.data?.find((g) => g.id === "clamav")?.installed;
  const signatures = status.data?.signatures;
  const scanning = clamav === undefined || !status.data ? null : clamav && signatures !== null && nowSeconds() - (signatures?.updated ?? 0) <= 3 * 86_400;

  return (
    <>
      <PageHeader title="Setup" />
      <div className="px-6 py-2">
        <ul className="m-0 list-none p-0">
          <Item title="Background checks" done={health.data ? true : down ? false : null}>
            <p className="m-0 max-w-[56ch] text-ink-muted">
              {health.data
                ? `bigmand ${health.data.version} is running as you and keeps its data in ${health.data.state_dir}. It keeps running after you close this window, until you log out${status.data?.services ? "; the background services also start it when you log in" : ""}.`
                : down
                  ? "bigmand is not running, so the steps below cannot be checked or changed yet."
                  : "Checking…"}
            </p>
            {health.data && (
              <>
                <StopChecks services={status.data?.services ?? false} />
                <p className="m-0 max-w-[56ch] text-small text-ink-muted">
                  The tray menu&apos;s Quit and stop background checks does the same, and so does{" "}
                  <span className="font-mono">bigman daemon stop</span> in a terminal.
                </p>
              </>
            )}
            {down && (
              <div>
                <ActionButton variant="primary" onClick={() => void client.invalidateQueries()}>
                  Start background checks
                </ActionButton>
              </div>
            )}
          </Item>

          <Item title="Checks that run on their own" done={null}>
            <p className="m-0 max-w-[56ch] text-ink-muted">
              While background checks run, these happen without you asking. Findings appear in Activity, and as a
              desktop notification if you allow it under Notifications.
            </p>
            <AutomaticChecks disabled={down} />
          </Item>

          <Item title="Malware scanning" done={down ? null : scanning}>
            <p className="m-0 max-w-[56ch] text-ink-muted">
              {down || clamav === undefined
                ? "Checked once background checks are running."
                : !clamav
                ? "Install ClamAV below to scan files against known-malware signatures."
                : signatures
                  ? `ClamAV is installed. Signatures are ${signatureAge(signatures.updated, nowSeconds())}${scanning ? "." : "; update them from Protect > Scans."}`
                  : "ClamAV is installed but has no signatures yet; update them from Protect > Scans."}
            </p>
          </Item>

          <Item title="Optional tools" done={null}>
            <p className="m-0 max-w-[56ch] text-ink-muted">
              Installed from your distribution&apos;s repositories with administrator approval through polkit.
              Each one only adds the checks that need it.
            </p>
            <Tools disabled={down} />
          </Item>

          <Item title="Background services" done={null}>
            <p className="m-0 max-w-[56ch] text-ink-muted">
              {status.data?.services
                ? "Installed: background checks start when you log in, along with the canary and startup monitor and a daily ClamAV signature download."
                : "Not installed: background checks run only once you open Bigman. Installing makes them start when you log in, along with the canary and startup monitor and a daily ClamAV signature download."}{" "}
              They follow your login session; nothing system-wide is enabled.
            </p>
            <div className="flex gap-2">
              {status.data?.services ? (
                <>
                  <ActionDialog action={{ kind: "services", remove: true }} label="Remove services…" disabled={down} />
                  <ActionDialog action={{ kind: "services", remove: false }} label="Install again…" variant="quiet" disabled={down} />
                </>
              ) : (
                <ActionDialog action={{ kind: "services", remove: false }} label="Install services…" disabled={down} />
              )}
            </div>
          </Item>

          <Item title="Preview package changes" done={null}>
            <p className="m-0 max-w-[56ch] text-ink-muted">
              {status.data?.receipt_hook
                ? `On: ${status.data.receipt_hook} makes your package manager show each transaction's receipt here first, with Continue and Abort. It goes ahead on its own after a minute.`
                : "Every transaction already gets a receipt afterwards. With this hook your package manager also shows the receipt here before anything is installed, with Continue and Abort, and goes ahead on its own after a minute so unattended upgrades never wait. Needs Bigman installed system-wide, because the package manager runs the hook as root."}
            </p>
            <div>
              {status.data?.receipt_hook ? (
                <ActionDialog action={{ kind: "receipt_hook", remove: true }} label="Turn off…" variant="quiet" disabled={down} />
              ) : (
                <ActionDialog action={{ kind: "receipt_hook", remove: false }} label="Turn on…" disabled={down} />
              )}
            </div>
          </Item>

          <Item title="What is collected" done={null}>
            <p className="m-0 max-w-[56ch] text-ink-muted">
              Each source reads as you, without administrator access, and sends nothing. Only security updates use the
              network, to download public data. A source that cannot read says so here and in Activity rather than
              reporting nothing.
            </p>
            <Sources disabled={down} />
          </Item>

          <Item title="Notifications" done={null}>
            <p className="m-0 max-w-[56ch] text-ink-muted">
              Entries gather for 30 seconds into one desktop notification, at most one a minute. Activity always has
              everything; these only choose what interrupts you.
            </p>
            <Notifications disabled={down} />
          </Item>

          <Item title="Browser extension" done={null}>
            <p className="m-0 max-w-[56ch] text-ink-muted">
              Lets the Chromium extension send downloads and installer links to Bigman. Load the extension unpacked,
              then paste its ID from the Extensions page.
            </p>
            <div className="flex max-w-md flex-col gap-2">
              <TextField label="Extension ID" value={extension} onChange={(v) => setExtension(v.trim())} mono hint="32 letters from a to p." />
              <div>
                <ActionDialog action={{ kind: "register_native_host", extension_id: extension }} label="Connect…" disabled={down || !validExtension} />
              </div>
            </div>
          </Item>

          <Item title="Appearance" done={null}>
            <Select
              label="Theme"
              value={theme}
              options={THEMES}
              onChange={(value) => {
                setTheme(value);
                applyTheme(value);
              }}
            />
          </Item>

          <Item title="Your data" done={null}>
            <p className="m-0 max-w-[56ch] text-ink-muted">
              Reports stay on this computer in a private folder (mode 0700) and leave it only when you export one.
              A report can contain local paths, file hashes and the source of scripts you inspected. Bigman sends no
              telemetry. It downloads public data for the daily security-update check (turn it off under Checks that
              run on their own) and ClamAV signatures (when you update them, and daily with background services). Checks
              you start use the network only when they say so, such as fetching an installer from an HTTPS address.
            </p>
          </Item>
        </ul>
      </div>
    </>
  );
}
