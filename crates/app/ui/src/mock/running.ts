// Running-system rows for the mock daemon: one disagreement between two views
// (findings scenario only), one program to review, one row that changed since
// it was marked as known, and the informational rows every desktop has.
import type { Finding, Program, Report, RunningItem } from "@/ipc";
import { NOW, report } from "./fixtures";

const row = (over: Partial<RunningItem> & Pick<RunningItem, "section" | "key" | "rule" | "severity" | "title" | "evidence">): RunningItem => ({
  action: "",
  command: null,
  facts: [],
  standing: "new",
  known_at: null,
  changed: [],
  ...over,
});

const BPF = ["sd_devices", "sd_fw_egress", "sd_fw_ingress", "restrict_filesystems"];

export function seedRunning(scenario: string): RunningItem[] {
  const quiet: RunningItem[] = [
    row({
      section: "programs",
      key: "process.exe-memfd:runtime:/home/sam/Applications/Notes-1.13.7.appimage",
      rule: "process.exe-memfd",
      severity: "info",
      title: "Notes-1.13.7.appimage runs from memory, with no file on disk",
      evidence:
        'Process name: memfd:runtime\nExecutable: memory file "runtime"\nStarted from an AppImage, whose launcher copies itself into memory before it runs the application.\nCommand line: /home/sam/Applications/Notes-1.13.7.appimage',
      action: "If you do not recognise it, copy the program out before stopping it and check which program started it; the commands below do both.",
      command: "cp /proc/27831/exe ~/copy-of-process-27831\nps -o ppid=,args= -p 27831",
      facts: [
        { label: "Memory file", value: "runtime", pinned: true },
        { label: "Started from", value: "/home/sam/Applications/Notes-1.13.7.appimage", pinned: true },
        { label: "Process ID", value: "27831", pinned: false },
      ],
    }),
    row({
      section: "programs",
      key: "process.preload:system",
      rule: "process.preload",
      severity: "info",
      title: "2 libraries are loaded first into some of your programs",
      evidence:
        "Libraries named by LD_PRELOAD or LD_AUDIT:\n/app/lib/spotify-preload.so\nlibmozsandbox.so\nPrograms: Web Content (3), spotify",
      action: "For a library in a folder you can write to, find which file sets the variable (shell profile, ~/.config/environment.d, a .desktop launcher) and what put the library there.",
      facts: [
        { label: "Libraries", value: "/app/lib/spotify-preload.so, libmozsandbox.so", pinned: true },
        { label: "Programs", value: "Web Content (3), spotify", pinned: false },
      ],
    }),
    row({
      section: "network",
      key: "net.packet-socket:A program run by the administrator",
      rule: "net.packet-socket",
      severity: "info",
      title: "A program run by the administrator reads network frames directly",
      evidence:
        "Packet sockets for: ARP (0x0806), Wi-Fi and wired sign-in (EAPOL) (0x888e)\nInterfaces: any interface, wlan0\nHeld by: a program run by the administrator, which Bigman cannot name without the administrator's view",
      action: "If no capture or network-usage tool is running, find the holder with the command below and check what started it.",
      command: "sudo ss -0 -p",
      facts: [
        { label: "Held by", value: "A program run by the administrator", pinned: true },
        { label: "Frames", value: "ARP (0x0806), Wi-Fi and wired sign-in (EAPOL) (0x888e)", pinned: true },
        { label: "Interfaces", value: "any interface, wlan0", pinned: false },
      ],
      standing: "known",
      known_at: NOW - 9 * 86_400,
    }),
  ];
  if (scenario === "empty") return [];
  if (scenario === "calm") return quiet;
  return [
    row({
      section: "disagreements",
      key: "hidden.net-socket:tcp:0.0.0.0:4444",
      rule: "hidden.net-socket",
      severity: "high",
      title: "A TCP socket on 0.0.0.0:4444 is in one socket table and not the other",
      evidence:
        "Shown by the kernel's socket diagnostics (what ss reads): local 0.0.0.0:4444, owned by the administrator (root).\nMissing from /proc/net (what netstat reads).\nEach was read twice with the same result.",
      action: "Note the port and the remote address. Find which program holds it with the view that does show it, and check the disk from other trusted media.",
      facts: [
        { label: "Protocol", value: "TCP", pinned: true },
        { label: "Remote", value: "none (listening or unconnected)", pinned: true },
        { label: "Local", value: "0.0.0.0:4444", pinned: false },
        { label: "Shown by", value: "the kernel's socket diagnostics (what ss reads)", pinned: true },
        { label: "Missing from", value: "/proc/net (what netstat reads)", pinned: true },
      ],
    }),
    row({
      section: "programs",
      key: "process.exe-deleted:/tmp/.cache-4f1a/updater",
      rule: "process.exe-deleted",
      severity: "high",
      title: "updater is running and its file was deleted",
      evidence:
        "/tmp/.cache-4f1a/updater started this program and no longer exists. It ran from a temporary folder.\nCommand line: updater --quiet",
      action: "If you do not recognise it, copy the program out before stopping it and check what started it; the commands below do both.",
      command: "cp /proc/88213/exe ~/copy-of-process-88213\nps -o ppid=,args= -p 88213",
      facts: [
        { label: "Program", value: "/tmp/.cache-4f1a/updater", pinned: true },
        { label: "File", value: "device 0:53, inode 4410, 812344 bytes, changed 2026-11-10T12:00:04Z", pinned: true },
        { label: "Command line", value: "updater --quiet", pinned: false },
        { label: "Process ID", value: "88213", pinned: false },
      ],
    }),
    row({
      section: "kernel",
      key: "bpf.programs",
      rule: "bpf.programs",
      severity: "review",
      title: "5 BPF program names are loaded in the kernel",
      evidence: `Names, from the kernel's symbol list (/proc/kallsyms):\n${[...BPF, "trace_openat"].sort().join("\n")}`,
      action: "Mark the list as known once you recognise it; Bigman then reports when a name is added. The command below shows each program's type and what loaded it.",
      command: "sudo bpftool prog list",
      facts: [{ label: "Names", value: [...BPF, "trace_openat"].sort().join(", "), pinned: true }],
      standing: "changed",
      known_at: NOW - 3 * 86_400,
      changed: ["Names: added trace_openat"],
    }),
    row({
      section: "watches",
      key: "watch.credentials:/home/sam/.cache/sync-agent/agent:/home/sam/.ssh",
      rule: "watch.credentials",
      severity: "review",
      title: "agent is told whenever /home/sam/.ssh changes",
      evidence:
        "/home/sam/.cache/sync-agent/agent (process 5120) holds an inotify watch on /home/sam/.ssh.",
      action: "If you do not recognise the program, read where it is and what started it before you add or change a key.",
      facts: [
        { label: "Program", value: "/home/sam/.cache/sync-agent/agent", pinned: true },
        { label: "File", value: "device 0:54, inode 99120, 2204160 bytes, changed 2026-11-01T12:26:40Z", pinned: true },
        { label: "Watches", value: "/home/sam/.ssh", pinned: true },
        { label: "Process ID", value: "5120", pinned: false },
      ],
    }),
    ...quiet,
  ];
}

export function runningReport(items: RunningItem[], id: string, created: number, scenario: string): Report {
  const findings: Finding[] = items
    .filter((i) => i.standing !== "known")
    .map((i) => ({
      rule: i.rule,
      severity: i.severity,
      summary: i.standing === "changed" ? `${i.title} (changed since you marked it as known)` : i.title,
      evidence: i.standing === "changed" ? `${i.evidence}\nChanged: ${i.changed.join("; ")}` : i.evidence,
      action: i.action,
      ...(i.command ? { command: i.command } : {}),
    }));
  const known = items.filter((i) => i.standing === "known").length;
  return report({
    id,
    created,
    kind: "Running system",
    subject: "Programs, kernel and network on this computer",
    findings,
    coverage: {
      checked: [
        "Process list, module list, socket tables and /etc/ld.so.preload: the C library's reading against the kernel's",
        "Process list: /proc against a signal probe of every process ID",
        "Kernel modules: /proc/modules against /sys/module and the kernel's symbol list",
        "Sockets: /proc/net against the kernel's socket diagnostics",
        "Your programs: executable file, standard input and output, preloaded libraries",
        "Programs that read packets below TCP and UDP (packet and raw sockets)",
      ],
      notes: [
        ...(known ? [`${String(known)} ${known === 1 ? "row you marked as known is" : "rows you marked as known are"} unchanged and not counted.`] : []),
        "Program details (files, sockets, watches, environment) are read for your own programs only; other accounts' programs are checked by name.",
        "A kernel module that filters every view at once defeats these comparisons. They find libraries injected into programs and modules that hide from one list; checking the disk from other trusted media is the reliable test.",
      ],
      skipped:
        scenario === "incomplete"
          ? [{ what: "Which programs own the password-store and portal bus names", why: "tool_missing", detail: "busctl is not installed." }]
          : [],
    },
  });
}

/** Your programs for Stop and watch: one that keeps coming back, ordinary apps, and ones Bigman will not stop. */
export function seedPrograms(): Program[] {
  const p = (pid: number, name: string, command: string, unit: string | null, minutes: number, refused: string | null = null, exe: string | null = null): Program => ({
    pid,
    start: pid * 100 + 7,
    name,
    exe: exe ?? command.split(" ")[0] ?? null,
    command,
    parent_pid: 1820,
    parent: "systemd",
    unit,
    started: NOW - minutes * 60,
    refused,
  });
  return [
    p(31044, "kworker-update", "/home/sam/.cache/.kworker-update --quiet", "app-update-helper-31002.scope", 42),
    p(28110, "firefox", "/usr/lib64/firefox/firefox", "app-org.mozilla.firefox-28110.scope", 180),
    p(27831, "Notes-1.13.7.appimage", "/home/sam/Applications/Notes-1.13.7.appimage", "app-notes-27831.scope", 95),
    p(5120, "syncthing", "/usr/bin/syncthing serve --no-browser", "syncthing.service", 600),
    p(88213, "updater", "/home/sam/.local/share/updater/updater --daemon", "app-updater-88200.scope", 30, null, "/home/sam/.local/share/updater/updater (deleted)"),
    p(2404, "kwin_wayland", "/usr/bin/kwin_wayland --wayland-fd 7", "plasma-kwin_wayland.service", 600, "Your desktop session needs it."),
    p(4221, "bigmand", "/home/sam/.local/bin/bigmand", "app-bigman-4210.scope", 600, "Part of Bigman. Stop Bigman from Setup."),
  ];
}

/** The report a Stop and watch of `program` leaves: the one that keeps coming back is restarted by its helper. */
export function stopWatchReport(program: Program, id: string, created: number): { report: Report; restarted: boolean } {
  const base = { id, created, kind: "Stop and watch", subject: `${program.name} (process ${String(program.pid)})`, sha256: null, errors: [], details: "" };
  const coverage = {
    checked: ["The process list, ten times a second for 60 seconds"],
    notes: [
      "Bigman sent the stop request (SIGTERM) because you asked it to; it never stops a program on its own and never forces one.",
      "A program that is started and stops again within a tenth of a second can be missed. Programs that start it later, at the next login or on a timer, are not seen in this minute.",
    ],
    skipped: [],
  };
  const origin = `Stopped: ${program.name} (process ${String(program.pid)})\nProgram: ${program.exe ?? program.command}\nCommand line: ${program.command}\nStarted by: ${program.parent} (process ${String(program.parent_pid)})${program.unit ? `\nRan in: ${program.unit}` : ""}`;
  if (program.name === "kworker-update") {
    return {
      restarted: true,
      report: {
        ...base,
        findings: [
          {
            rule: "respawn.program-restart",
            severity: "review",
            summary: "kworker-update was started again by update-helper, 1.4 seconds later",
            evidence: `${origin}\n\nStarted again 1.4 seconds after the stop request: process 31190\nCommand line: ${program.command}\nRuns in: ${program.unit ?? ""}\nStarted by update-helper (process 31002: /home/sam/.config/autostart-scripts/update-helper, in ${program.unit ?? ""})\n  started by systemd (process 1820, the service manager, in init.scope)\nStill running when the watch ended.`,
            action:
              "A program that starts another again as soon as it stops is how some unwanted software stays running, often as two copies that watch each other. Launchers, updaters and development tools do it too. If you recognise update-helper, nothing needs doing. If not, check its program file from Inspect and look at Protect › Startup files for what starts it.",
          },
        ],
        coverage,
      },
    };
  }
  return {
    restarted: false,
    report: {
      ...base,
      findings: [
        {
          rule: "respawn.not-restarted",
          severity: "info",
          summary: `Nothing started ${program.name} again within 60 seconds`,
          evidence: origin,
          action: "Something that starts it later is not seen here: at the next login, from a timer or a scheduled job. Protect › Startup files lists those.",
        },
      ],
      coverage,
    },
  };
}
