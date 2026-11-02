// Synthetic live data for browser dev mode and tests: timeline events, throughput
// with one collection gap, the current sockets and source health. Invented, and
// shaped like what bigmand's sources produce.
import type { Event, Finding, ListenerRow, NetworkSnapshot, Receipt, Report, Sample, SampleSeries, Socket, SourceStatus } from "@/ipc";
import rules from "@/ipc/generated/rules.json";
import { NOW } from "./fixtures";

/** What to do for a rule, from the registry, as core's findings do. */
const remedy = (id: string) => (rules as { id: string; remediation: string }[]).find((r) => r.id === id)?.remediation ?? "";

const minutes = (n: number) => NOW - n * 60;
const days = (n: number) => NOW - n * 86_400;

/** The daemon was not running for these ten minutes; charts must show the gap. */
export const GAP: readonly [number, number] = [minutes(50), minutes(40)];

type Seed = Omit<Event, "id" | "rule" | "link" | "first_at" | "seen_before"> &
  Partial<Pick<Event, "rule" | "link" | "first_at" | "seen_before">>;

const SEEDS: Seed[] = [
  { at: minutes(3), source: "listener", key: "listener:dockerd:0.0.0.0:2375", severity: "high", rule: "net.listener.docker_api", summary: "The container API is reachable on port 2375", evidence: "tcp listen 0.0.0.0:2375\ndockerd (pid 1893) /usr/bin/dockerd" },
  { at: minutes(9), source: "network", key: "net:curl", severity: "info", summary: "curl made its first network connection, to 93.184.215.14:443", evidence: "curl (pid 40211) /usr/bin/curl\n192.168.1.20:51344 → 93.184.215.14:443" },
  { at: minutes(26), source: "receipt", key: "transaction:dnf:412", severity: "review", rule: "receipt.hook.sensitive_path", summary: "dnf installed nix and nix-libs · 1 finding to review", evidence: "dnf install nix\nInstall nix 2.24.9-1.fc44\nInstall nix-libs 2.24.9-1.fc44", link: { kind: "receipt", id: "rc-412" } },
  { at: minutes(31), source: "denial", key: "denial:selinux:firefox:read:dir:ssh_home_t", severity: "info", rule: "denial.selinux", summary: "firefox, confined by the mozilla policy, tried to read .ssh (dir) and was stopped.", evidence: 'avc:  denied  { read } for  pid=4242 comm="firefox" name=".ssh" dev="dm-0" ino=1234 scontext=unconfined_u:unconfined_r:mozilla_t:s0 tcontext=unconfined_u:object_r:ssh_home_t:s0 tclass=dir permissive=0', seen_before: 13, first_at: days(30) },
  { at: minutes(64), source: "network", key: "net:slack:35.82.187.x:443", severity: "info", summary: "slack connected to a new address range, 35.82.187.x port 443", evidence: "slack (pid 3301) /usr/lib/slack/slack\n192.168.1.20:49822 → 35.82.187.142:443" },
  { at: minutes(130), source: "listener", key: "listener:beam.smp:0.0.0.0:4000", severity: "review", rule: "net.listener.wildcard", summary: "beam.smp listens on every network interface, port 4000", evidence: "tcp listen 0.0.0.0:4000\nbeam.smp (pid 2210) /usr/lib64/erlang/erts-15.2/bin/beam.smp" },
  { at: days(1) + 3_600, source: "receipt", key: "transaction:dnf:411", severity: "info", summary: "dnf updated 32 packages", evidence: "dnf upgrade --refresh" },
  { at: days(2), source: "network", key: "net:obsidian", severity: "info", summary: "obsidian made its first network connection, to 104.26.0.147:443", evidence: "obsidian (pid 5120)\n192.168.1.20:40112 → 104.26.0.147:443" },
];

const REPORT_SENTENCE: Record<string, (r: Report) => string> = {
  "Security checkup": () => "Ran a checkup",
  "File scan": (r) => `Scanned ${r.subject}`,
  "PDF triage": (r) => `Read the PDF ${r.subject}`,
  "Installer inspection": (r) => `Inspected ${r.subject}`,
  "Persistence changes": () => "Compared startup files",
};

const NIX_SCRIPT = [
  "if [ $1 -eq 1 ]; then",
  "  groupadd -r nixbld 2>/dev/null || :",
  "  ln -sf /nix/var/nix/profiles/default/etc/profile.d/nix.sh /etc/profile.d/nix.sh",
  "fi",
].join("\n");

/** The receipt behind "dnf installed nix and nix-libs", as core's builder makes it. */
export const RECEIPTS: Record<string, Receipt> = {
  "rc-412": {
    id: "rc-412",
    at: minutes(26),
    manager: "dnf",
    transaction: "412",
    command: "dnf install nix",
    packages: [
      { name: "nix", version: "2.24.9-1.fc44", kind: "install" },
      { name: "nix-libs", version: "2.24.9-1.fc44", kind: "install" },
    ],
    files: [
      { path: "/usr/bin/nix", package: "nix", category: "binaries", privilege: null },
      { path: "/usr/bin/nix-daemon", package: "nix", category: "binaries", privilege: null },
      { path: "/usr/lib/systemd/system/nix-daemon.service", package: "nix", category: "systemd", privilege: null },
      { path: "/usr/lib/systemd/system/nix-daemon.socket", package: "nix", category: "systemd", privilege: null },
      { path: "/etc/nix/nix.conf", package: "nix", category: "/etc", privilege: null },
      { path: "/usr/lib64/libnixstore.so.2.24.9", package: "nix-libs", category: "libraries", privilege: null },
      { path: "/usr/lib64/libnixutil.so.2.24.9", package: "nix-libs", category: "libraries", privilege: null },
    ],
    scripts: [{ package: "nix", phase: "postinstall", interpreter: "/bin/sh", text: NIX_SCRIPT, changed: false }],
    hits: [{ script: 0, line: 3, rule: "receipt.hook.sensitive_path", severity: "review" }],
    services: ["/usr/lib/systemd/system/nix-daemon.service", "/usr/lib/systemd/system/nix-daemon.socket"],
    repositories: {},
    after: { checked: true, listeners: [] },
    findings: [
      {
        rule: "receipt.hook.sensitive_path",
        severity: "review",
        summary: "An install script writes to a startup or authentication path",
        evidence: "nix postinstall script, line 3: ln -sf /nix/var/nix/profiles/default/etc/profile.d/nix.sh /etc/profile.d/nix.sh",
        action: "Read the line. Packages rarely need to change these paths from a script; if you did not expect it, reinstall from your distribution's repository or remove the package.",
      },
    ],
    coverage: {
      checked: ["Package database (rpm)"],
      notes: ["Built after the transaction from the package database: the files and install scripts of each installed or updated package. Bigman did not see the transaction run, and old package versions are not compared yet."],
      skipped: [],
    },
    score: 1,
  },
};

/** A saved report as the reports source enters it (core sources/reports.rs). */
export function reportEvent(r: Report, id: number, seenBefore: number, firstAt: number): Event {
  const rank = { info: 0, review: 1, high: 2 } as const;
  const worst = r.findings.reduce<Finding | null>((w, f) => (!w || rank[f.severity] > rank[w.severity] ? f : w), null);
  const toReview = r.summary?.to_review ?? r.findings.filter((f) => f.severity !== "info").length;
  const incomplete = r.errors.length > 0 || r.coverage.skipped.length > 0;
  const outcome = incomplete ? "incomplete" : toReview === 0 ? "no findings" : `${toReview} finding${toReview === 1 ? "" : "s"} to review`;
  const what = REPORT_SENTENCE[r.kind]?.(r) ?? `${r.kind}: ${r.subject}`;
  const canary = r.kind === "Canary triggered";
  return {
    id,
    at: r.created,
    source: canary ? "canary" : "scan",
    key: `report:${r.kind}:${r.subject}:${incomplete ? "incomplete" : (worst && worst.severity !== "info" ? worst.severity : "clear")}`,
    rule: worst?.rule ?? null,
    severity: worst?.severity ?? "info",
    summary: canary ? `A canary file was opened: ${r.subject}` : `${what} · ${outcome}`,
    evidence: r.subject,
    seen_before: seenBefore,
    first_at: firstAt,
    link: { kind: "report", id: r.id },
  };
}

/** Events for a scenario, oldest first with ascending ids, like the store. */
export function seedEvents(scenario: string, reports: Report[]): Event[] {
  if (scenario === "empty") return [];
  const counts = new Map<string, { n: number; first: number }>();
  const seeds: Seed[] = scenario === "calm" ? SEEDS.filter((s) => s.severity === "info") : SEEDS;
  const all: (Seed & { report?: Report })[] = [...seeds, ...reports.map((r) => ({ ...reportEvent(r, 0, 0, 0), report: r }))];
  if (scenario === "many") {
    // Enough to prove the list stays smooth: 10,000 rows over 30 days.
    for (let i = 0; i < 10_000; i++) {
      const program = ["firefox", "slack", "code", "spotify", "curl", "git"][i % 6] ?? "firefox";
      all.push({ at: NOW - Math.floor((i * 30 * 86_400) / 10_000), source: "network", key: `net:${program}:10.${i % 251}.0.x:443`, severity: "info", summary: `${program} connected to 10.${i % 251}.0.x port 443`, evidence: `${program}\n192.168.1.20 → 10.${i % 251}.0.7:443` });
    }
  }
  all.sort((a, b) => a.at - b.at);
  return all.map((s, index) => {
    const seen = counts.get(s.key) ?? { n: 0, first: s.report ? s.report.created : (s.first_at ?? s.at) };
    counts.set(s.key, { n: seen.n + 1, first: seen.first });
    const id = index + 1;
    if (s.report) return reportEvent(s.report, id, seen.n, seen.n === 0 ? s.report.created : seen.first);
    return { rule: null, link: null, ...s, id, seen_before: s.seen_before ?? seen.n, first_at: seen.first };
  });
}

/** A repeatable pseudo-random number in [0, 1) for an integer. */
function noise(n: number): number {
  return ((Math.imul(n, 2654435761) >>> 0) % 10_000) / 10_000;
}

/**
 * Typical desktop traffic, deterministic per second: busy in the day, quiet at
 * night, slow swells over tens of minutes, a download now and then, and jitter.
 */
function rate(iface: string, at: number): { rx: number; tx: number } {
  const hour = new Date(at * 1000).getHours();
  const day = hour >= 8 && hour < 23 ? 1 : 0.15;
  const swell = 0.6 + 0.4 * Math.sin(at / 1_900) * Math.sin(at / 610);
  const download = noise(Math.floor(at / 1_800)) > 0.8 && at % 1_800 < 240 ? 5 : 1;
  const jitter = 0.75 + 0.5 * noise(at);
  const scale = iface === "wlp2s0" ? 160_000 : 8_000;
  const rx = scale * day * swell * download * jitter;
  return { rx: Math.round(rx), tx: Math.round(scale * 0.12 * day * jitter * (0.5 + swell / 2)) };
}

export function sampleAt(iface: string, at: number): Sample {
  return { at, iface, ...rate(iface, at) };
}

/** The average over one bucket, as the store's downsampling computes it. */
function bucketAverage(iface: string, start: number, bucket: number): Sample {
  const step = Math.max(1, Math.floor(bucket / 30));
  let rx = 0;
  let tx = 0;
  let n = 0;
  for (let t = start; t < start + bucket; t += step, n++) {
    const r = rate(iface, t);
    rx += r.rx;
    tx += r.tx;
  }
  return { at: start, iface, rx: Math.round(rx / n), tx: Math.round(tx / n) };
}

export const INTERFACES = ["wlp2s0", "tun0"] as const;

/** What the store returns: bucket averages, nothing where the daemon was not running. */
export function samplesFor(since: number, until: number, points: number): SampleSeries {
  const bucket = Math.max(1, Math.floor((until - since) / Math.max(1, points)));
  const samples: Sample[] = [];
  for (let b = Math.floor(since / bucket) * bucket; b <= until; b += bucket) {
    if (b + bucket > GAP[0] && b < GAP[1]) continue;
    for (const iface of INTERFACES) samples.push(bucket === 1 ? sampleAt(iface, b) : bucketAverage(iface, b, bucket));
  }
  return { bucket, samples };
}

const listener = (name: string | null, pid: number, local: string, exe: string): Socket => ({
  protocol: "tcp",
  local,
  remote: null,
  state: "listen",
  uid: name ? 1000 : 0,
  process: name ? { pid, name, exe } : null,
});

const finding = (rule: string, severity: Finding["severity"], summary: string, socket: Socket): Finding => ({
  rule,
  severity,
  summary,
  evidence: `tcp listen ${socket.local}\n${socket.process ? `${socket.process.name} (pid ${socket.process.pid}) ${socket.process.exe ?? ""}` : `uid ${socket.uid}`}`,
  action: remedy(rule),
});

function listenerRow(socket: Socket, reach: string, findings: Finding[], firstAt: number | null): ListenerRow {
  return { socket, reach, findings, first_at: firstAt };
}

export function networkSnapshot(scenario: string): NetworkSnapshot {
  const docker = listener("dockerd", 1893, "0.0.0.0:2375", "/usr/bin/dockerd");
  const beam = listener("beam.smp", 2210, "0.0.0.0:4000", "/usr/lib64/erlang/erts-15.2/bin/beam.smp");
  const sshd = listener(null, 0, "0.0.0.0:22", "");
  const listeners: ListenerRow[] = [
    ...(scenario === "calm"
      ? []
      : [
          listenerRow(docker, "every network interface", [
            finding("net.listener.docker_api", "high", "The container API is reachable on port 2375", docker),
            finding("net.listener.wildcard", "review", "dockerd listens on every network interface, port 2375", docker),
          ], minutes(3)),
          listenerRow(beam, "every network interface", [finding("net.listener.wildcard", "review", "beam.smp listens on every network interface, port 4000", beam)], minutes(130)),
        ]),
    listenerRow(sshd, "every network interface", [finding("net.listener.wildcard", "review", "A system program (root) listens on every network interface, port 22", sshd)], days(40)),
    listenerRow(listener("cupsd", 1201, "127.0.0.1:631", "/usr/sbin/cupsd"), "this computer only", [], days(40)),
    listenerRow(listener("vite", 40102, "127.0.0.1:5173", "/usr/bin/node"), "this computer only", [], minutes(12)),
  ];
  const connection = (name: string, pid: number, remote: string, range: string, firstAt: number | null) => ({
    socket: { protocol: "tcp" as const, local: `192.168.1.20:${40000 + pid}`, remote, state: "established", uid: 1000, process: { pid, name, exe: `/usr/bin/${name}` } },
    range,
    first_at: firstAt,
  });
  return {
    at: NOW,
    connections: [
      connection("curl", 4021, "93.184.215.14:443", "93.184.215.x", minutes(9)),
      connection("firefox", 4242, "142.250.80.46:443", "142.250.80.x", days(30)),
      connection("firefox", 4242, "151.101.1.69:443", "151.101.1.x", days(12)),
      connection("slack", 3301, "35.82.187.142:443", "35.82.187.x", minutes(64)),
      connection("spotify", 3410, "35.186.224.9:4070", "35.186.224.x", days(5)),
    ],
    listeners,
    coverage: {
      checked: ["Sockets in /proc/net"],
      notes: ["Connections list your own programs. Listeners list every program, but other users' programs show without a name."],
      skipped: [],
    },
  };
}

export function sourceStatuses(scenario: string): SourceStatus[] {
  const ok = (name: string, label: string): SourceStatus => ({ name, label, last_run: NOW - 2, ok: true, why: null, detail: "" });
  return [
    ok("connections", "Your programs' connections"),
    scenario === "calm"
      ? ok("denials", "SELinux and AppArmor denials")
      : { name: "denials", label: "SELinux and AppArmor denials", last_run: NOW - 4, ok: false, why: "permission_denied", detail: "Reading the system journal needs membership of systemd-journal, adm or wheel." },
    ok("listeners", "Listening programs"),
    ok("packages", "Package installs and updates"),
    ok("reports", "Checks you run"),
    ok("throughput", "Network throughput"),
  ];
}

const UPDATES_NOTE =
  "The distribution's own security data decides what is affected and what fixes it; CISA's catalog of exploited vulnerabilities only raises the severity. Bigman shows the update command and never runs it. No package list leaves this computer.";

/** What `bigman advisories` reports, per scenario: updates, none, or an unsupported system. */
export function securityUpdates(scenario: string, id: string, at: number): Report {
  const base = { id, created: at, kind: "Security updates", sha256: null, errors: [] };
  if (scenario === "empty") {
    return {
      ...base,
      subject: "Ubuntu 24.04 LTS",
      findings: [],
      details: "",
      coverage: { checked: [], notes: [UPDATES_NOTE], skipped: [{ what: "Security data for Ubuntu 24.04 LTS", why: "not_applicable", detail: "Bigman reads Fedora, Debian and Arch Linux security data; other distributions are not supported yet." }] },
    };
  }
  const coverage = { checked: ["3,314 installed rpm packages", "Fedora updateinfo through dnf", "CISA Known Exploited Vulnerabilities"], notes: [UPDATES_NOTE], skipped: [] };
  if (scenario === "calm") return { ...base, subject: "Fedora Linux 44 (Workstation Edition)", findings: [], details: "", coverage };
  const update = (rule: "advisory.update-available" | "advisory.exploited", id_: string, names: string, versions: string, cves: string, rating: string): Finding => ({
    rule,
    severity: rule === "advisory.exploited" ? "high" : "review",
    summary: rule === "advisory.exploited" ? `Security update for ${names} fixes a vulnerability known to be exploited` : `Security update for ${names}`,
    evidence: `${id_} (${rating})\n${versions}\nCVEs: ${cves}${rule === "advisory.exploited" ? `\nIn CISA's Known Exploited Vulnerabilities catalog: ${cves}` : ""}\nFrom dnf updateinfo (Fedora repository metadata)`,
    action: rule === "advisory.exploited" ? "Install this update before the others, with this command or your desktop's software updater." : "Install the update with this command, or from your desktop's software updater.",
    command: `sudo dnf upgrade --advisories=${id_}`,
  });
  return {
    ...base,
    subject: "Fedora Linux 44 (Workstation Edition)",
    details: "sudo dnf upgrade --advisories=FEDORA-2026-4a1b2c3d4e,FEDORA-2026-31601794bf,FEDORA-2026-d732fb97a0",
    coverage,
    findings: [
      update("advisory.exploited", "FEDORA-2026-4a1b2c3d4e", "openssh-server", "openssh-server: installed 0:10.0p1-5.fc44, fixed in 0:10.0p1-6.fc44", "CVE-2026-90001", "Important"),
      update("advisory.update-available", "FEDORA-2026-31601794bf", "firefox and firefox-langpacks", "firefox: installed 0:158.0-2.fc44, fixed in 0:159.0-1.fc44\nfirefox-langpacks: installed 0:158.0-2.fc44, fixed in 0:159.0-1.fc44", "No CVE ids listed", "Critical"),
      update("advisory.update-available", "FEDORA-2026-d732fb97a0", "gcc, cpp and 3 more", "gcc: installed 0:16.2.1-2.fc44, fixed in 0:16.2.1-3.fc44\ncpp: installed 0:16.2.1-2.fc44, fixed in 0:16.2.1-3.fc44", "CVE-2026-102010, CVE-2026-115455", "Moderate"),
      { rule: "advisory.reboot-required", severity: "review", summary: "Restart to finish installing updates", evidence: "Running kernel 7.2.9-200.fc44.x86_64; kernel 7.2.13-200.fc44.x86_64 is installed and takes effect after a restart.", action: "Restart when convenient; until then the fixes in the newer kernel are not running." },
      { rule: "advisory.untracked-package", severity: "info", summary: "2 packages are not covered by your distribution's security data", evidence: "chatgpt 26.1023.51421-1 (from openai-chatgpt)\nappimagelauncher 3.0.0-1 (from @commandline)", action: "These came from outside the distribution's repositories (for example the AUR or a local build). Follow their upstream for security updates." },
    ],
  };
}
