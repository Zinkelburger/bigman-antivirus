// Synthetic fixture data for browser dev mode and component tests. Not real
// detections: names, paths and hashes are invented and labeled as such.
import type { ProtectionStatus, Report, ToolState } from "@/ipc";

export const NOW = Math.floor(Date.now() / 1000);
const minutes = (n: number) => NOW - n * 60;
const days = (n: number) => NOW - n * 86_400;

const TOOL_LIST: [string, string][] = [
  ["clamscan", "Known-malware file scanning"],
  ["freshclam", "Signature updates"],
  ["shellcheck", "Shell correctness checks"],
  ["bwrap", "Restricted execution"],
  ["strace", "Execution traces"],
  ["lynis", "System hardening audit"],
  ["osv-scanner", "Dependency vulnerability checks"],
  ["osqueryi", "System inventory"],
  ["yr", "YARA-X rules"],
  ["pdfid.py", "PDF triage"],
  ["pdf-parser.py", "PDF structural inspection"],
  ["pdfinfo", "PDF metadata and links"],
  ["opensnitch-ui", "Application firewall interface"],
];

export function tools(missing: string[]): ToolState[] {
  return TOOL_LIST.map(([name, purpose]) => ({ name, purpose, available: !missing.includes(name) }));
}

export function report(overrides: Partial<Report>): Report {
  return {
    id: "3f6c2a1e-0000-4000-8000-000000000001",
    created: minutes(42),
    kind: "File scan",
    subject: "/home/sam/Downloads",
    sha256: null,
    findings: [],
    coverage: {
      checked: ["ClamAV signature scan"],
      notes: [
        "ClamAV signature/heuristic scan of snapshots. At most 2,000 files, 64 MiB/file, 512 MiB total.",
        "214 file snapshots, 1.75 GiB.",
      ],
      skipped: [],
    },
    errors: [],
    details: "",
    ...overrides,
  };
}

export const scanWithFinding = report({
  findings: [
    {
      rule: "clamav.match",
      severity: "high",
      summary: "invoice-copy.zip in Downloads matches a ClamAV signature",
      evidence:
        "/home/sam/Downloads/invoice-copy.zip\nWin.Test.EICAR_HDB-1\nSHA-256 275a021bbfb6489e54d471899f7db9d1663fc695ec2fe2a2c4538aabf651fd0f",
      action:
        "If you did not expect this file, quarantine it: Bigman keeps a private recovery copy and you can restore it later.",
    },
  ],
});

export const scanIncomplete = report({
  coverage: {
    checked: ["ClamAV signature scan"],
    notes: ["ClamAV signature/heuristic scan of snapshots."],
    skipped: [
      {
        what: "/home/sam/Downloads/backup.tar",
        why: "limit_reached",
        detail: "Expected a regular file no larger than 67108864 bytes",
      },
    ],
  },
});

export const statusScenarios: Record<string, ProtectionStatus> = {
  findings: {
    checked_at: NOW,
    tools: tools(["lynis", "osv-scanner", "osqueryi", "yr", "pdfid.py", "pdf-parser.py"]),
    signatures: { updated: days(5), location: "/var/lib/clamav" },
    monitor: {
      updated: NOW - 2,
      pid: 4121,
      watched: 1,
      configured: 1,
      errors: [],
      watched_ids: ["8d0c7e55-0000-4000-8000-000000000002"],
    },
    last_scan: scanWithFinding,
  },
  calm: {
    checked_at: NOW,
    tools: tools([]),
    signatures: { updated: minutes(180), location: "/home/sam/.local/share/bigman/signatures" },
    monitor: {
      updated: NOW - 1,
      pid: 4121,
      watched: 2,
      configured: 2,
      errors: [],
      watched_ids: [],
    },
    last_scan: report({}),
  },
  incomplete: {
    checked_at: NOW,
    tools: tools(["shellcheck", "strace"]),
    signatures: { updated: minutes(600), location: "/var/lib/clamav" },
    monitor: null,
    last_scan: scanIncomplete,
  },
  empty: {
    checked_at: NOW,
    tools: tools(TOOL_LIST.map(([name]) => name)),
    signatures: null,
    monitor: null,
    last_scan: null,
  },
};

/** A checkup result like the daemon returns for `doctor`. */
export function checkup(): Report {
  return report({
    id: `3f6c2a1e-0000-4000-8000-${String(Date.now()).slice(-12).padStart(12, "0")}`,
    created: Math.floor(Date.now() / 1000),
    kind: "Security checkup",
    subject: "Local Linux workstation",
    findings: [
      {
        rule: "tool.missing",
        severity: "info",
        summary: "lynis is not installed",
        evidence: "System hardening audit",
        action: "Open Setup for an installation plan. Optional tools are not required for unrelated features.",
      },
    ],
    coverage: {
      checked: ["Optional tools", "Signature freshness", "Canary monitor heartbeat", "Git credential storage"],
      notes: ["Read-only local tool and configuration check. Listening sockets are inventory, not findings."],
      skipped: [],
    },
  });
}

/** Report history, newest first, for the Overview's life list. */
export function history(scenario: string): Report[] {
  const at = (minutesAgo: number) => NOW - minutesAgo * 60;
  const scans = Array.from({ length: 6 }, (_, i) =>
    report({ id: `3f6c2a1e-0000-4000-8000-1000000000${i}`, created: at(42 + i * 1_440) }),
  );
  const latestScan = scenario === "findings" ? scanWithFinding : scenario === "incomplete" ? scanIncomplete : scans[0];
  return [
    report({
      id: "3f6c2a1e-0000-4000-8000-200000000001",
      created: at(8),
      kind: "Installer inspection",
      subject: "/home/sam/Downloads/install-tool.sh",
      findings:
        scenario === "calm"
          ? []
          : [
              {
                rule: "shell.execute-download",
                severity: "high",
                summary: "Downloads and executes code",
                evidence: "Line 5: curl -fsSL https://example.org/setup.sh | sh",
                action: "Save and inspect the exact downloaded bytes first.",
              },
            ],
    }),
    { ...(latestScan ?? scans[0]), created: at(42) } as Report,
    ...scans.slice(1),
  ];
}
