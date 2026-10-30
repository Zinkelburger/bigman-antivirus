import { STORIES, type Story } from "./stories";

const GROUPS: Story["group"][] = ["Shell", "Status", "Data", "Evidence", "Charts", "Actions", "Feedback", "Input"];

/** /#/gallery: every component in every state, light and dark side by side. */
export default function Gallery() {
  return (
    <div className="min-w-[960px] bg-ground px-6 py-6">
      <h1 className="m-0 mb-1 text-title font-semibold text-ink">Bigman components</h1>
      <p className="m-0 mb-6 text-body text-ink-muted">
        {STORIES.length} stories. Sample data is synthetic.
      </p>
      {GROUPS.map((group) => (
        <section key={group} aria-labelledby={`group-${group}`} className="mb-10">
          <h2 id={`group-${group}`} className="m-0 mb-3 border-b border-rule-strong pb-1 text-body font-semibold text-ink">
            {group}
          </h2>
          <div className="flex flex-col gap-6">
            {STORIES.filter((s) => s.group === group).map((s) => (
              <article key={`${s.component}-${s.state}`} aria-label={`${s.component}, ${s.state}`}>
                <h3 className="m-0 mb-2 text-small font-semibold text-ink-muted">
                  {s.component} · {s.state}
                </h3>
                <div className="grid grid-cols-2 gap-4">
                  {(["light", "dark"] as const).map((theme) => (
                    <div key={theme} data-theme={theme} className="min-w-0 rounded-panel bg-ground p-4 text-ink">
                      {s.render()}
                    </div>
                  ))}
                </div>
              </article>
            ))}
          </div>
        </section>
      ))}
    </div>
  );
}
