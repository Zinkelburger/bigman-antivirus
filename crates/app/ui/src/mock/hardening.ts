// Synthetic hardening review for browser dev mode and tests, shaped like the
// daemon's HardeningReview. Values are invented, not read from any machine.
import type { Action, ActionOutcome, ActionPlan, AppliedChange, Coverage, HardeningReview, InPlace, Suggestion } from "@/ipc";
import { NOW, report } from "./fixtures";

const SUGGESTIONS: Suggestion[] = [
  {
    id: "harden.sshd",
    rule: "harden.sshd",
    severity: "review",
    title: "Turn off the SSH server",
    summary:
      "The SSH server is on. It lets people log in to this computer from other computers. Other computers can connect through the FedoraWorkstation zone (wlp2s0) because it allows the ssh service. Nobody has logged in over the network in the last 90 days, as far as the logs Bigman can read show.",
    evidence: "sshd.service: enabled, active\ntcp listen 0.0.0.0:22\ntcp listen [::]:22\nLogin history: the system journal and wtmp, the last 90 days",
    why: "Every service that accepts connections is a way in. One you do not use only adds risk: a weak password, a leaked key or an unpatched bug in it is enough.",
    tradeoff:
      "You can no longer log in to this computer from another one with ssh. Your own ssh to other computers, git over SSH and scp to servers keep working. If you need it again, the undo turns it back on.",
    fix: { kind: "bigman", admin: true, verb: "Turn off" },
    kept: false,
  },
  {
    id: "harden.sysctl.ptrace_scope",
    rule: "harden.sysctl.ptrace_scope",
    severity: "review",
    title: "Stop programs from reading each other's memory",
    summary:
      "Any program running as you can attach to your other programs (ptrace_scope is 0) and read their memory: your browser's cookies, an unlocked password manager, ssh-agent keys. The elfutils-default-yama-scope package sets it, in /usr/lib/sysctl.d/10-default-yama-scope.conf.",
    evidence: "kernel.yama.ptrace_scope = 0\nSet to 0 by /usr/lib/sysctl.d/10-default-yama-scope.conf (installed by the elfutils-default-yama-scope package)",
    why: "With ptrace_scope at 1, a program can only debug the programs it started itself. Malware that arrives through a package or an editor extension can no longer read secrets out of programs that are already running.",
    tradeoff:
      "Attaching to a program that is already running (gdb -p, strace -p, reptyr, py-spy --pid) needs sudo. Starting a program under the debugger, which is what IDEs do, still works.",
    fix: { kind: "bigman", admin: true, verb: "Change" },
    kept: false,
  },
  {
    id: "harden.screen_lock",
    rule: "harden.screen_lock",
    severity: "review",
    title: "Lock the screen when you step away",
    summary: "Your screen does not lock by itself when you step away.",
    evidence: "kscreenlockerrc [Daemon] Autolock=false\nkscreenlockerrc [Daemon] Timeout=0\nkscreenlockerrc [Daemon] LockOnResume=(default)",
    why: "An unlocked session lets anyone at the keyboard read your mail, use your logged-in accounts or run a command as you in seconds.",
    tradeoff:
      "You type your password when you come back after five minutes away, or after the computer sleeps. You can pick a longer delay in your desktop's screen-locking settings.",
    fix: { kind: "bigman", admin: false, verb: "Turn on" },
    kept: false,
  },
  {
    id: "harden.name.llmnr",
    rule: "harden.name.llmnr",
    severity: "info",
    title: "Stop asking the local network for computer names",
    summary:
      'When a name is not in DNS, this computer asks everyone on the local network for it (LLMNR is "resolve"). Any device on the same Wi-Fi can answer and send you to itself.',
    evidence: "resolvectl status: LLMNR=resolve",
    why: "On public Wi-Fi, any device on the network can answer these questions, to collect sign-in attempts or redirect connections. Names ending in .local (mDNS) and normal DNS are not affected.",
    tradeoff: 'Short names of computers on a network without a DNS server ("nas", "printer") may stop resolving. Use their .local name or address instead.',
    fix: { kind: "bigman", admin: true, verb: "Turn off" },
    kept: false,
  },
  {
    id: "harden.secure_boot",
    rule: "harden.secure_boot",
    severity: "info",
    title: "Turn on Secure Boot",
    summary: "Secure Boot is off, so the firmware starts any boot loader and kernel, including a modified one.",
    evidence: "SecureBoot EFI variable: 0",
    why: "Secure Boot stops boot-level malware (bootkits) that would load before Linux and hide from everything running after it.",
    tradeoff:
      "Kernel modules you built or installed outside the distribution (NVIDIA from a third-party repository, VirtualBox) stop loading until their signing key is enrolled.",
    fix: {
      kind: "manual",
      steps:
        "If you use out-of-tree kernel modules, enroll their signing key first (for example sudo mokutil --import with the key your distribution documents). Then restart, open the firmware setup (usually F2, F12 or Delete at power-on) and turn Secure Boot on.",
    },
    kept: false,
  },
];

const IN_PLACE: InPlace[] = [
  { check: "Firewall", detail: "firewalld is running: the FedoraWorkstation zone (wlp2s0)." },
  { check: "Network service: cups-browsed", detail: "Installed, but off." },
  { check: "Kernel setting kernel.dmesg_restrict", detail: "kernel.dmesg_restrict is 1." },
  { check: "Automatic login", detail: "Nobody is logged in automatically at startup." },
  { check: "Disk encryption", detail: "The system and home folders are on an encrypted disk." },
  { check: "Groups that grant administrator access", detail: "You are not in the docker group." },
  { check: "Home folder privacy", detail: "Only you can open /home/sam (mode 700)." },
];

const PLANS: Record<string, { details: string; undo: string }> = {
  "harden.sshd": {
    details: "systemctl disable --now sshd.service",
    undo: "systemctl enable --now sshd.service",
  },
  "harden.sysctl.ptrace_scope": {
    details:
      "write /etc/sysctl.d/90-bigman-ptrace-scope.conf:\n    # Written by Bigman. Remove this file to undo.\n    kernel.yama.ptrace_scope = 1\nsysctl -w kernel.yama.ptrace_scope=1",
    undo: "rm -f -- /etc/sysctl.d/90-bigman-ptrace-scope.conf\nsysctl -w kernel.yama.ptrace_scope=0",
  },
  "harden.screen_lock": {
    details:
      "kwriteconfig6 --file kscreenlockerrc --group Daemon --key Autolock true\nkwriteconfig6 --file kscreenlockerrc --group Daemon --key Timeout 5\ndbus-send --session --type=method_call --dest=org.freedesktop.ScreenSaver /ScreenSaver org.kde.screensaver.configure",
    undo: "kwriteconfig6 --file kscreenlockerrc --group Daemon --key Autolock false\nkwriteconfig6 --file kscreenlockerrc --group Daemon --key Timeout 0\ndbus-send --session --type=method_call --dest=org.freedesktop.ScreenSaver /ScreenSaver org.kde.screensaver.configure",
  },
  "harden.name.llmnr": {
    details:
      "mkdir -p /etc/systemd/resolved.conf.d\nwrite /etc/systemd/resolved.conf.d/90-bigman-llmnr.conf:\n    # Written by Bigman. Remove this file to undo.\n    [Resolve]\n    LLMNR=no\nsystemctl restart systemd-resolved.service",
    undo: "rm -f -- /etc/systemd/resolved.conf.d/90-bigman-llmnr.conf\nsystemctl restart systemd-resolved.service",
  },
};

/** Who runs a change, as the daemon says it (Change::who). */
const who = (admin: boolean) =>
  admin ? "Polkit asks for your password, then these commands run as administrator." : "These commands run as you; no password is needed.";

/** The in-memory computer: which suggestions are open, kept or changed. */
export class MockHardening {
  open: Suggestion[];
  applied: (AppliedChange & { suggestion: Suggestion })[] = [];
  kept = new Set<string>();
  skipped: Coverage["skipped"];

  constructor(scenario: string) {
    this.open = scenario === "calm" || scenario === "empty" ? [] : SUGGESTIONS.map((s) => ({ ...s }));
    this.skipped =
      scenario === "incomplete"
        ? [{ what: "Firewall", why: "permission_denied", detail: "nftables.service manages the firewall, and its rules are readable only by an administrator" }]
        : [];
  }

  review(): HardeningReview {
    const suggestions = [...this.open.filter((s) => !this.kept.has(s.id)), ...this.open.filter((s) => this.kept.has(s.id))].map((s) => ({ ...s, kept: this.kept.has(s.id) }));
    const counted = suggestions.filter((s) => !s.kept);
    const notes = ["Read-only: settings, service states and logs were read without administrator rights. Nothing changes until you confirm a previewed change."];
    if (this.kept.size) notes.push(`You chose to leave ${this.kept.size} setting${this.kept.size === 1 ? "" : "s"} as ${this.kept.size === 1 ? "it is" : "they are"}.`);
    return {
      report: report({
        id: "hardening-review",
        created: NOW,
        kind: "Hardening review",
        subject: "This computer's settings",
        findings: counted.map((s) => ({ rule: s.rule, severity: s.severity, summary: s.summary, evidence: s.evidence, action: s.title })),
        coverage: { checked: ["SSH server", "Firewall", "Kernel settings", "Screen lock", "Automatic login", "Disk encryption", "Secure Boot"], notes, skipped: this.skipped },
      }),
      suggestions,
      // A check that could not run is never also listed as already set.
      in_place: IN_PLACE.filter((c) => !this.skipped.some((s) => s.what === c.check)),
      applied: this.applied.map((a) => ({ id: a.id, title: a.title, at: a.at, undo: a.undo })),
    };
  }

  plan(action: Extract<Action, { kind: "harden" | "undo_hardening" | "keep_setting" }>): ActionPlan {
    const s = this.open.find((x) => x.id === action.id) ?? this.applied.find((a) => a.id === action.id)?.suggestion;
    if (!s) throw new Error("This suggestion no longer applies: the setting may have changed since you last looked");
    const plan = PLANS[s.id];
    switch (action.kind) {
      case "harden":
        if (!plan || s.fix.kind !== "bigman") throw new Error("Bigman does not make this change itself; follow the steps shown with it");
        return { title: s.title, verb: s.fix.verb, summary: `${s.tradeoff}\n\n${who(s.fix.admin)}`, details: plan.details, undo: `Offered right after the change, and later under Changed by Bigman (bigman harden ${s.id} --undo), each with its own preview.`, careful: false, facts: [], note: null, details_label: null };
      case "undo_hardening":
        return { title: `Undo: ${s.title}`, verb: "Undo", summary: `Put the setting back the way it was before Bigman changed it.\n\n${who(s.fix.kind === "bigman" && s.fix.admin)}`, details: plan?.undo ?? "", undo: null, careful: false, facts: [], note: null, details_label: null };
      case "keep_setting":
        return action.keep
          ? { title: `Stop suggesting: ${s.title}`, verb: "Stop suggesting", summary: "Bigman stops counting this suggestion and lists it under Left as it is. Nothing on this computer changes.", details: "", undo: `Suggest again… under Left as it is (bigman harden ${s.id} --suggest-again).`, careful: false, facts: [], note: null, details_label: null }
          : { title: `Suggest again: ${s.title}`, verb: "Suggest again", summary: "Bigman counts this suggestion again. Nothing on this computer changes.", details: "", undo: `Leave as it is… on the suggestion (bigman harden ${s.id} --keep).`, careful: false, facts: [], note: null, details_label: null };
    }
  }

  apply(action: Extract<Action, { kind: "harden" | "undo_hardening" | "keep_setting" }>): ActionOutcome {
    switch (action.kind) {
      case "harden": {
        const s = this.open.find((x) => x.id === action.id);
        if (!s) throw new Error("This suggestion no longer applies");
        this.open = this.open.filter((x) => x.id !== s.id);
        this.applied = [...this.applied, { id: s.id, title: s.title, at: Math.floor(Date.now() / 1000), undo: PLANS[s.id]?.undo ?? "", suggestion: s }];
        return { message: `${s.title}: done. Bigman checked again and the new setting is in place.`, report: null, undo: { kind: "undo_hardening", id: s.id } };
      }
      case "undo_hardening": {
        const a = this.applied.find((x) => x.id === action.id);
        if (!a) throw new Error("Bigman has no record of making this change");
        this.applied = this.applied.filter((x) => x.id !== a.id);
        this.open = SUGGESTIONS.filter((s) => s.id === a.id || this.open.some((o) => o.id === s.id));
        return { message: `Put back as it was before "${a.title}".`, report: null, undo: null };
      }
      case "keep_setting":
        if (action.keep) this.kept.add(action.id);
        else this.kept.delete(action.id);
        return {
          message: action.keep ? "Left as it is. Bigman no longer counts it." : "Bigman suggests it again.",
          report: null,
          undo: { kind: "keep_setting", id: action.id, keep: !action.keep },
        };
    }
  }
}
