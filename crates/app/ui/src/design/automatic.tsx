// What the daemon does without being asked, each with its switch: the Downloads
// and project watches and the daily updates check. Setup and the first-run note
// on Overview show the same switches.
import { usePreferences, useSetPreferences, type Preferences } from "@/ipc";
import { IncompleteNotice } from "./components/IncompleteNotice";
import { Skeleton } from "./components/Skeleton";
import { Toggle } from "./components/Toggle";

const SWITCHES: readonly { key: keyof Preferences; label: string; description: string }[] = [
  {
    key: "watch_downloads",
    label: "Check new downloads",
    description:
      "Each new file in Downloads is checked once it finishes downloading. It is only read: nothing is moved, deleted or opened. When off, check a file yourself from Inspect.",
  },
  {
    key: "watch_projects",
    label: "Check new project folders",
    description:
      "A folder cloned or unpacked into ~/Projects, ~/src, ~/code or Downloads is checked for what it would run when opened. Nothing in it is run. When off, check a folder yourself from Inspect.",
  },
  {
    key: "check_updates",
    label: "Check for security updates",
    description:
      "Once a day and two minutes after package changes. Downloads your distribution's security data and CISA's list of exploited vulnerabilities; no package list is sent. When off, check from Protect › Updates.",
  },
];

export function AutomaticChecks({ disabled }: { disabled: boolean }) {
  const prefs = usePreferences();
  const save = useSetPreferences();
  if (disabled) return <p className="m-0 text-small text-ink-muted">Shown once background checks are running.</p>;
  if (prefs.isPending) return <Skeleton shape="rows" count={3} />;
  if (prefs.isError) return <IncompleteNotice title="These settings could not be loaded" errors={[prefs.error.message]} />;
  const current = prefs.data;
  return (
    <div className="flex max-w-[40rem] flex-col gap-3">
      {SWITCHES.map((s) => (
        <Toggle
          key={s.key}
          label={s.label}
          description={s.description}
          checked={current[s.key]}
          disabled={save.isPending}
          onChange={(checked) => save.mutate({ ...current, [s.key]: checked })}
        />
      ))}
      {save.isError && <IncompleteNotice title="The change was not saved" errors={[save.error.message]} />}
    </div>
  );
}
