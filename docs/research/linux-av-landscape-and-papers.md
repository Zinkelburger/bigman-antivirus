# Linux antivirus and endpoint security landscape, with research references

> A survey of the tools a Linux desktop user or open-source maintainer can use today, and the academic and long-form literature on Linux malware and defense. For each tool it says where Bigman should **integrate** (call it as an optional external tool), **borrow** (reuse ideas, check lists, data formats or license-compatible content), or **avoid** (deliberately not imitate). It ends with a positioning section on what the market leaves uncovered. This is a landscape document, not an attack catalogue; threat details live in the `threats/` files. Last researched: 2026-10-29.

**How facts were verified.** License, archive state, last push and latest release for every GitHub-hosted project came from the GitHub API on 2026-10-29. Packaged versions came from Fedora 44 `dnf info`. Paper metadata was checked against Crossref DOI records, publisher or author pages (usenix.org, arxiv.org, ndss-symposium.org, eurecom.fr), or the PDF itself. Vendor claims were checked against vendor documentation pages. The shared web-search budget ran out partway through this research, so a few items could only be checked by fetching a known URL, or not at all. Those are marked **(unverified)** where they appear and listed together in [Section 12](#12-unverified-items-and-open-questions).

**Legend used in the tables**

- **Priv.** `U` = useful rootless. `R` = needs root, a kernel module, or eBPF/kernel privileges. `U/R` = runs as a user with reduced visibility, or has a user-side UI with a root daemon.
- **Focus** `Desk` = desktop/workstation, `Srv` = servers, `Fleet` = central console managing many hosts, `Analyst` = analyst/forensics tool.
- **Bigman stance** `Integrate` / `Borrow` / `Avoid`, often more than one.

**A note on licenses.** Bigman is `AGPL-3.0-only` (`Cargo.toml`). Code under MIT, BSD, Apache-2.0, LGPL-2.1-or-later, GPL-2.0-or-later, GPL-3.0 or AGPL-3.0 can be combined into an AGPLv3 work. Code under **GPL-2.0-only** (ClamAV, much of Wazuh and OSSEC, LKRG) cannot be copied into Bigman. Proprietary or source-available content (Elastic License 2.0, vendor signatures) cannot be redistributed by Bigman either. Calling a tool as a separate process and parsing its output, which is what Bigman already does, avoids the combination question entirely. Detection *ideas* (which file paths to watch, for example) are not copyrightable, but rule text is. This is engineering guidance, not legal advice.

---

## 1. Landscape at a glance

| Tool | License | What it does | Priv. | Focus | Bigman stance |
|---|---|---|---|---|---|
| **ClamAV** 1.5.4 (2026-08-07; Fedora 44 ships 1.4.6) | GPL-2.0-only | Signature AV engine, `freshclam` updates, `clamd`, optional on-access via fanotify | U (scan), R (on-access) | Srv/mail, used on desktops | Integrate (already the scan engine) |
| **ClamTk** | GPL-1.0+ or Artistic (Perl terms) | GTK GUI for ClamAV | U | Desk | Avoid as a dependency: **unmaintained since April 2024**. Fill its gap |
| **Linux Malware Detect (maldet)** | GPL-2.0 | Shared-hosting scanner: MD5/hex/YARA/ClamAV stages, inotify monitor | R | Srv (web hosting) | Borrow ideas (web-shell focus); avoid auto-quarantine |
| **Sophos Protection for Linux** | Proprietary | Server agent (on-access off by default, runtime detection plugin), Sophos Central | R | Srv/Fleet | Avoid; note free edition retired |
| **ESET Endpoint Antivirus for Linux** 13.2 / Server Security | Proprietary | Desktop AV for named desktop distros, X11/Wayland GUI | R | **Desk**/Srv | Coexist; detect its presence |
| **Bitdefender GravityZone (BEST for Linux)** | Proprietary | Managed AV/EDR agent | R | Srv/Fleet | Avoid; coexist |
| **Microsoft Defender for Endpoint on Linux** | Proprietary | NGAV + EDR + vuln mgmt; eBPF sensor, passive mode | R | Srv/Fleet ("Linux server workloads") | Borrow the passive-mode and `mdatp health` ideas |
| **CrowdStrike Falcon (Linux)** | Proprietary | EDR; kernel-mode or user-mode (eBPF) backends | R | Srv/Fleet | Avoid kernel coupling (see 2024 RHEL 9.4 panics) |
| **SentinelOne Singularity (Linux / Cloud Workload)** | Proprietary | EDR/CWPP agent | R | Srv/Fleet | Avoid; coexist |
| **Elastic Defend** | eBPF probes GPL-2.0 or BSD-2-Clause; userspace and rules Elastic License 2.0 | EDR integration for Elastic Agent; prebuilt detection and YARA rules | R | Fleet | Borrow ideas only (ELv2 content cannot be bundled) |
| **Wazuh** 4.14.9 | GPL-2.0 (engine AGPL-3.0+, shared libs GPL-2.0 or AGPL-3.0+) | HIDS/XDR: FIM, rootcheck, SCA (CIS-style policy checks), vuln detection, log analysis | R | Fleet | Borrow SCA/rootcheck concepts |
| **OSSEC** | GPL-2.0 | Original HIDS: log analysis, FIM, rootcheck | R | Srv/Fleet | Borrow concepts |
| **osquery** 5.24.0 | Apache-2.0 OR GPL-2.0-only | OS state as SQL tables | U/R | Fleet/Analyst | Integrate (already ingests process JSON); expand |
| **Fleet** | MIT core, commercial `ee/` | osquery fleet manager | Server | Fleet | Avoid (single-user tool) |
| **Velociraptor** 0.77.3 | AGPL-3.0 | DFIR collection and hunting with VQL "artifacts" | R | Fleet/Analyst | Borrow artifact ideas (license-compatible) |
| **Falco** 0.45.1 | Apache-2.0 | eBPF runtime detection with YAML rules; container focus | R | Srv/K8s | Borrow rule taxonomy |
| **Tracee** 0.24.1 | Apache-2.0 | eBPF runtime security and forensics | R | Srv/K8s | Borrow signatures as ideas |
| **Tetragon** 1.7.1 | Apache-2.0 | eBPF observability plus in-kernel enforcement (TracingPolicy) | R | Srv/K8s | Avoid enforcement; borrow policy ideas |
| **auditd / audit-userspace** | GPL-2.0 | Kernel audit framework and rules | R | Srv | Integrate optionally (rule-pack generator, log reader) |
| **Neo23x0/auditd** ruleset | Apache-2.0 | Best-practice audit rules | n/a | Srv | Borrow (license-compatible) |
| **LKRG** | GPL-2.0 | Kernel module: runtime kernel integrity and exploit detection | R | Srv/Desk | Detect status only |
| **Sysmon for Linux** | MIT | eBPF Sysmon-style event logging to syslog | R | Srv | Optional event source |
| **Kunai** 0.6.2 | GPL-3.0 | Rust + eBPF threat-hunting event logger | R | Srv/Desk | Optional event source; borrow event model |
| **AIDE** 0.19.2 | GPL-2.0-or-later | File integrity database | R (system), U (home) | Srv | Borrow approach; Bigman baselines already cover startup files |
| **fapolicyd** 2.0.1 | GPL-3.0-or-later | Application allowlisting using RPM trust DB | R | Srv/Desk (RHEL/Fedora) | Detect status; borrow trust-DB idea |
| **Lynis** 3.1.7 | GPL-3.0-only | Hardening audit, report file | U (partial), R (full) | Srv/Desk | Integrate (already parses reports) |
| **OpenSCAP** 1.4.5 + **ComplianceAsCode** | LGPL-2.1-or-later; content BSD-3-Clause | SCAP/XCCDF compliance scanning; SCAP Security Guide content | R (most rules) | Srv/Fleet | Borrow check content (BSD) |
| **CIS Benchmarks** | Free PDFs; terms **(unverified)** | Consensus hardening guides; Server and Workstation profiles | n/a | Srv/Desk | Borrow topics, not text |
| **rkhunter** 1.4.6 | GPL-2.0-or-later | Rootkit, backdoor and local-exploit checks | R | Srv | Optional adapter; avoid "clean" claims |
| **chkrootkit** 0.58b | BSD-2-Clause AND GPL-2.0-or-later | Rootkit signature checks | R | Srv | Same as rkhunter |
| **unhide** 20240510 | GPL-3.0-or-later AND MIT | Hidden process/port cross-view checks | R | Srv | Borrow cross-view idea |
| **YARA** 4.5.5 / **YARA-X** 1.22.1 | BSD-3-Clause | Pattern-matching rules engine | U | Analyst/any | Integrate (embed `yara-x` crate) |
| **capa** 10.0.0 | Apache-2.0 | Capability detection for PE/ELF/shellcode, mapped to ATT&CK/MBC | U | Analyst | Integrate optionally ("what can this ELF do") |
| **Detect It Easy** | MIT | File type, compiler and packer identification (ELF included) | U | Analyst | Integrate optionally (packer/UPX note) |
| **OpenSnitch** 1.8.0 | GPL-3.0 | Interactive per-application outbound firewall | U/R (root daemon, user UI) | **Desk** | Integrate (already launches/sets up) |
| **Portmaster** 2.2.8 | GPL-3.0 | Per-app network firewall plus DNS filtering | U/R | **Desk** | Detect status; recommend as alternative |
| **Firejail** 0.9.80 | GPL-2.0-or-later | SUID-root namespace sandbox with app profiles | SUID | **Desk** | Avoid recommending by default; borrow profile knowledge |
| **Bubblewrap** | LGPL | Unprivileged sandbox (used by Flatpak) | U | Desk | Already used by Bigman |
| **Flatseal** | GPL-3.0 | GUI to review and override Flatpak permissions | U | **Desk** | Borrow (rootless permission audit) |
| **USBGuard** 1.1.4 | GPL-2.0-or-later | USB device allow/deny policy daemon | R | Desk/Srv | Detect status; suggest |
| **GNOME Settings "Device Security"** / **fwupd HSI** | GPL-2.0 / LGPL-2.1 | Firmware/platform security rating (HSI levels) | U (read) | **Desk** | Integrate (`fwupdmgr security --json` works rootless) |

---

## 2. Signature antivirus: ClamAV and its front ends

### ClamAV

- **License and stewardship:** GPL-2.0-only, maintained by Cisco Talos. GitHub `Cisco-Talos/clamav`: latest release `clamav-1.5.4` (2026-08-07); Fedora 44 packages 1.4.6. https://github.com/Cisco-Talos/clamav
- **What it does:** on-demand scanning (`clamscan`, or `clamdscan` against the `clamd` daemon), signature updates (`freshclam`), archive and document unpacking, and on-access scanning (`clamonacc`).
- **Privilege:** on-demand scanning is rootless for files the user can read. On-access scanning needs elevated privileges and kernel fanotify (kernel ≥ 3.8). It can block access (`OnAccessPrevention yes`) only when the kernel has `CONFIG_FANOTIFY_ACCESS_PERMISSIONS`; otherwise it runs notify-only. It refuses `/` as an include path because blocking system files could lock up the machine. https://docs.clamav.net/manual/OnAccess.html
- **Desktop vs server:** designed for mail gateways and file servers. No GUI is included.
- **Detection quality:** see [Section 9](#9-independent-tests-and-detection-rate-evidence). The evidence varies widely with the sample set: from bottom-tier (AV-TEST 2015) to 97.9% on a VirusShare ELF corpus (Raffa & Sgandurra) to about 60% on mostly-Windows commodity malware (Splunk 2022).
- **Bigman today:** ClamAV is the file-scan engine, used over a private snapshot. `bigman doctor` reports signature age, and a per-user `freshclam` timer keeps signatures current.
- **Integrate:** keep it. Worth adding:
  - Report which signature databases were loaded, with versions, in each scan report, so "no findings" is always tied to a signature date.
  - Optionally use `clamd` for speed when it is already running.
  - Optionally use third-party signature feeds the user opts into, after checking each feed's license **(unverified)**.
- **Avoid:**
  - Enabling `clamonacc` prevention by default. It needs root, can hang a desktop, and conflicts with Bigman's rootless model.
  - Implying signature coverage of Linux-native threats it does not have.

### ClamTk (maintenance status)

- The README now opens with "Note: This program is no longer maintained" and links issue #163. It was "last checked or updated on 20240330". The last push to GitHub was 2024-03-30, and the repository is **not archived**. Press coverage: OMG! Ubuntu, "Defences Down! ClamTk is No Longer Maintained" (April 2024). https://github.com/dave-theunsub/clamtk/issues/163 , https://www.omgubuntu.co.uk/2024/04/clamtk-is-no-longer-maintained
- It is still packaged by distributions and on Flathub, so users keep installing an unmaintained security GUI.
- **Bigman stance:** do not depend on it. The *gap it leaves* — a maintained, honest desktop front end for ClamAV with scheduled scans, signature freshness, and quarantine — is exactly what Bigman's scan, quarantine and timer workflows already provide. This is the clearest single positioning fact in this survey.

### Linux Malware Detect (maldet / LMD)

- GPL-2.0, `rfxn/linux-malware-detect`, last push 2026-05. It describes itself as a "multi-engine Linux malware scanner with five detection stages (MD5, HEX pattern, YARA, ClamAV, statistical), real-time inotify monitoring, quarantine, and multi-channel alerting", aimed at shared web hosting. https://github.com/rfxn/linux-malware-detect
- **Borrow:**
  - The idea of a web-shell/PHP-focused signature stage, relevant to maintainers who run small sites.
  - Its "statistical" stage as a concept (entropy/obfuscation scoring). Keep it deterministic and explainable.
- **Avoid:** automatic quarantine and root-wide monitoring.

---

## 3. Commercial Linux AV and EDR

Common traits: every product runs as a root agent, usually with a kernel module or eBPF programs. Most report to a cloud or on-premises console and rely on cloud reputation and telemetry. Licensing is per endpoint and business-only. All are useful references for *what* to detect. None is a model for *how* Bigman should run.

| Product | Verified facts | Desktop? | Lessons for Bigman |
|---|---|---|---|
| **Sophos** | Sophos Anti-Virus for Linux v9 reached end of life on 20 July 2023. The free edition was retired earlier (community posts; exact last-update date **unverified**). The replacement, **Sophos Protection for Linux** (release notes list 2026.3, September 2026), is a Sophos Central server agent: on-access scanning added in 2023.1 and off by default, plus a runtime-detection plugin. https://docs.sophos.com/releasenotes/output/en-us/esg/linux_protection_rn.html , https://community.sophos.com/on-premise-endpoint/f/sophos-endpoint-software/127094/sophos-anti-virus-for-linux-version-9-eol | Release notes refer only to servers | The best-known *free* Linux AV for home users is gone. That widens the desktop gap |
| **ESET** | **ESET Endpoint Antivirus for Linux** 13.2 lists desktop distributions explicitly: Ubuntu Desktop 26.04 LTS, RHEL 8/9/10 with a desktop, Linux Mint 21/22, Debian 12/13. It supports X11 and Wayland, with Cinnamon, GNOME, KDE, MATE and XFCE. Separate ESET Server Security for Linux. https://help.eset.com/eeau/13.2/en-US/system_requirements.html | **Yes (paid, business)** | The only mainstream vendor verified here with a current desktop Linux product. Bigman should detect it and coexist (avoid double scanning) |
| **Bitdefender GravityZone** | Linux agent "Bitdefender Endpoint Security Tools (BEST) for Linux", managed from GravityZone. https://www.bitdefender.com/business/support/en/77209-79246-endpoint-security-for-linux.html (page exists; module details such as the on-access mechanism **unverified**) | Business endpoints/servers | Coexist |
| **Microsoft Defender for Endpoint on Linux** | Microsoft describes it as protecting "Linux server workloads". It has real-time protection, behavioral monitoring, **passive mode** (coexists with other AV), scheduled/on-demand scans, EDR, live response and vulnerability management. It uses an eBPF-based sensor without kernel modules (falls back to Netlink), and agent health is shown by `mdatp health`. https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-endpoint-linux , https://learn.microsoft.com/en-us/defender-endpoint/linux-support-ebpf | Server-positioned | Borrow: a *passive / report-only* mode as the default posture, and a single health command (Bigman's `doctor`). Monthly platform builds expire after nine months, which shows that tying protection to agent freshness is normal; Bigman should flag stale components the same way |
| **CrowdStrike Falcon (Linux)** | Kernel-mode and user-mode (eBPF) sensor backends. In April–June 2024, the user-mode (BPF) backend was linked to kernel panics on RHEL 9.4 kernels 5.14.0-410+; kernel-mode was not affected. Red Hat advised disabling the sensor as a mitigation. https://www.theregister.com/2024/07/21/crowdstrike_linux_crashes_restoration_tools/ | Servers primarily | **Avoid** shipping kernel code or tightly coupled eBPF in a desktop tool: even "safe" eBPF crashed production kernels. Bigman's rootless design is a stability feature, not just a privacy one |
| **SentinelOne** | Linux coverage through the Singularity platform and Cloud Workload Security. https://www.sentinelone.com/platform/singularity-cloud-workload-security/ (agent architecture details **unverified**) | Servers/cloud | Coexist |
| **Elastic Defend** | Linux endpoint uses eBPF for event collection on newer kernels. The eBPF probes are dual GPL-2.0/BSD-2-Clause; userspace code is Elastic License 2.0 (`elastic/ebpf` repository). Elastic's prebuilt detection rules (`elastic/detection-rules`) and protection artifacts including YARA and behavior rules (`elastic/protections-artifacts`) are both **Elastic License 2.0**. https://github.com/elastic/ebpf , https://github.com/elastic/protections-artifacts , https://www.elastic.co/third-party-dependencies | Fleet | Elastic Security Labs write-ups are among the best public Linux detection-engineering material (https://www.elastic.co/security-labs). Read them for *ideas*; do not copy ELv2 rule text into an AGPL project. Users may point Bigman's YARA-X adapter at the rules themselves |

Other vendors exist (Kaspersky, Trend Micro, Dr.Web, G Data, eScan, Panda; see the AV-Comparatives 2015 product list) but were not surveyed here.

---

## 4. Open-source HIDS, EDR and telemetry

All of these assume root and, except osquery used interactively, a server or fleet back end. Their value to Bigman is as **optional event sources** and as **libraries of detection ideas**.

### Wazuh

- **License:** mostly GPL-2.0. The repository's LICENSE now puts the Wazuh **engine** (`src/engine/`) under AGPL-3.0-or-later, with shared libraries dual GPL-2.0 or AGPL-3.0+. Latest release v4.14.9 (2026-10-15). https://github.com/wazuh/wazuh
- **What it does:** agent/manager HIDS with file integrity monitoring (FIM), rootcheck, Security Configuration Assessment (SCA, YAML policy checks modelled on CIS), vulnerability detection and log analysis.
- **Desktop vs server:** fleet. One user on one laptop must still stand up a manager and indexer.
- **Borrow:**
  - SCA as a *format idea*: declarative checks of a file/command/registry condition with a rationale and remediation. Bigman's planned hardening checks could use a similar declarative, testable rule format.
  - Rootcheck's lists of known-bad paths as ideas.
- **Avoid:** copying GPL-2.0-only ruleset text, and needing a manager.

### OSSEC

- GPL-2.0, `ossec/ossec-hids`, still receiving commits (last push 2026-10-28). https://www.ossec.net/
- The original source of rootcheck's `rootkit_files` / `rootkit_trojans` style lists and of log-based detection.
- **Borrow:** concepts only.

### osquery and Fleet

- **osquery:** `Apache-2.0 OR GPL-2.0-only` (Bigman can take the Apache option), 5.24.0 (2026-10-09). It exposes OS state as SQL tables. `osqueryi` runs as an ordinary user with reduced visibility; some tables need root. https://github.com/osquery/osquery
- **Fleet:** MIT for the core, commercial licence for `ee/`. A server for managing many osquery agents. https://fleetdm.com/
- **Bigman today:** ingests osquery process JSON.
- **Integrate:** ship a small, reviewed set of *read-only* osquery queries (listening ports, startup items, kernel modules, SUID files, browser extensions, apt/dnf repos) as an optional deeper inventory. Report coverage as "incomplete" when a table needs root.
- **Avoid:** Fleet-style central management.

### Velociraptor

- AGPL-3.0 (same license as Bigman), 0.77.3 (2026-10-05). https://docs.velociraptor.app/
- **What it does:** endpoint DFIR collection and hunting with VQL "artifacts" (YAML-wrapped queries). It has a large Linux artifact library, e.g. crontab, authorized_keys and systemd units (exact artifact names **unverified**).
- **Borrow:** being license-compatible, its artifacts are the richest legally reusable source of Linux persistence-collection logic for Bigman's startup baselines. Attribute any reused content.
- **Avoid:** the client/server deployment model.

### Falco, Tracee, Tetragon (eBPF runtime security)

| | Falco | Tracee | Tetragon |
|---|---|---|---|
| License | Apache-2.0 | Apache-2.0 | Apache-2.0 |
| Latest | 0.45.1 (2026-10-19) | v0.24.1 (2025-11-19; repository active) | v1.7.1 (2026-08-25) |
| Model | YAML rules over syscall events | Event tracing plus behavioral signatures | Observability plus in-kernel enforcement via TracingPolicy |
| Focus | Containers/Kubernetes, also hosts | Containers/hosts | Kubernetes, standalone mode |
| Privilege | R (eBPF, CAP_BPF/CAP_SYS_ADMIN class) | R | R |
| URL | https://falco.org/ | https://aquasecurity.github.io/tracee/latest/ | https://tetragon.io/ |

- **Borrow:** the rule *taxonomy* (for example "sensitive file read by an unexpected process", "shell spawned by a non-shell parent", "write below binary directories") as the vocabulary for Bigman findings. Note that the canary already gives Bigman a rootless form of "sensitive file was read".
- **Avoid:**
  - Running eBPF from a desktop companion. It needs root-equivalent privilege and couples Bigman to kernel versions (see the CrowdStrike row above).
  - Tetragon-style enforcement (killing processes), which conflicts with Bigman's "never auto-remediate" rule.
- If the user already runs one of these, a read-only importer of their JSON alerts into Bigman's history is a cheap integration.

### auditd

- `linux-audit/audit-userspace`, GPL-2.0, maintained (last push 2026-10). It is the kernel's own audit framework: watches on files and syscalls, logged to `/var/log/audit/audit.log`, root-only. https://people.redhat.com/sgrubb/audit/
- **Neo23x0/auditd** (Apache-2.0) is a widely used best-practice rule set, with watches on `/etc/ld.so.preload`, cron, sudoers, ssh keys, and kernel module loading. https://github.com/Neo23x0/auditd
- **Integrate (opt-in, root):** a `bigman setup audit-rules` action that *previews* a small reviewed rule file, installs it via polkit only on approval, and then reads audit events through `ausearch` with group-based access or a one-shot polkit read. This is the honest path to "who read or changed this file", which Bigman's canary currently cannot answer (no reader PID).
- **Borrow:** the rule list as a checklist for which paths Bigman's rootless baseline should snapshot (see [host integrity](../threats/04-host-integrity-and-hardening-audit.md)).

### LKRG (Linux Kernel Runtime Guard)

- GPL-2.0, Openwall/`lkrg-org`, loadable kernel module that checks kernel integrity at runtime and detects exploitation attempts (for example, unexpected credential changes). https://lkrg.org/
- **Bigman stance:** report in `doctor` whether LKRG is loaded (`/proc/modules` or `/sys/module/lkrg` is rootless-readable). Never ship or load it. It is a good recommendation for advanced users and is GPL-2.0-only, so it is incompatible for code reuse anyway.

### Sysmon for Linux and Kunai

- **Sysmon for Linux** (MIT, `microsoft/Sysmon-Linux`; the GitHub URL `microsoft/SysmonForLinux` redirects): Sysmon-style process, network and file events via eBPF to syslog, with an XML configuration. https://github.com/microsoft/SysmonForLinux
- **Kunai** (GPL-3.0, Rust, v0.6.2, 2025-11): "Threat-hunting tool for Linux", an eBPF event logger with JSON output. https://github.com/kunai-project/kunai , https://kunai.rocks/
- **Bigman stance:** both are optional, root-installed event sources. Kunai is Rust and GPL-3.0, so its event schema and parsing are the most reusable. An adapter that reads Kunai or Sysmon JSON for the last N hours and maps events to Bigman finding IDs would add process attribution without Bigman itself needing root.

### AIDE and fapolicyd

- **AIDE** (GPL-2.0-or-later, 0.19.2): a classic file integrity database. Bigman's `changes` baseline is a rootless, startup-focused AIDE. Borrow AIDE's idea of attribute groups (hash, mode, owner, xattrs, SELinux label), and extend baselines to permissions and owners, not only content. https://github.com/aide/aide
- **fapolicyd** (GPL-3.0-or-later, 2.0.1): allowlisting that trusts files known to the RPM database. Borrow the *trust-DB idea* for a rootless check: "executables in `$PATH` or autostart entries that no package owns" (`rpm -qf` / `dpkg -S`). https://github.com/linux-application-whitelisting/fapolicyd

---

## 5. Hardening and compliance auditing

| Tool | Notes | Bigman stance |
|---|---|---|
| **Lynis** (GPL-3.0-only, 3.1.7, 2026-06) https://cisofy.com/lynis/ | Shell-based audit. It can run unprivileged with fewer tests; a full audit needs root. Writes `/var/log/lynis-report.dat` (root) or a user-specified report path. | **Integrate** (already parses reports). Add: run in non-privileged mode by default and label skipped tests as coverage gaps; map Lynis test IDs to Bigman rule IDs. |
| **OpenSCAP** (LGPL-2.1-or-later, 1.4.5) + **ComplianceAsCode/content** (SCAP Security Guide, BSD-3-Clause) https://www.open-scap.org/ , https://github.com/ComplianceAsCode/content | XCCDF/OVAL compliance scans against profiles (CIS, STIG, ANSSI, and others). Most rules need root and are server-oriented. | **Borrow:** the content is BSD-licensed and thoroughly reviewed. Mine its rule descriptions and rationales for desktop-relevant checks (sshd settings, sudoers, mount options, kernel sysctls, package GPG checking). Optional adapter: if `oscap` is installed, run a chosen profile and import ARF results. |
| **CIS Benchmarks** https://www.cisecurity.org/cis-benchmarks | Consensus hardening guides for distributions, with Level 1/Level 2 and Server/Workstation profiles. Free PDF download for non-commercial use; exact license terms **(unverified)**. | **Borrow topics**, cite benchmark section numbers, never copy text. The "Workstation" profile is the right lens for Bigman. |

Related: [host integrity and hardening audit](../threats/04-host-integrity-and-hardening-audit.md) holds the concrete checklist.

---

## 6. Rootkit checkers

| Tool | Status | Notes |
|---|---|---|
| **rkhunter** (GPL-2.0-or-later) http://rkhunter.sourceforge.net/ | Latest release 1.4.6 (still what Fedora 44 ships; release year 2018 **unverified**) | Checks known rootkit files, suspicious strings, hidden files, changed binaries (via its own hash DB), and some configuration. Needs root; well known for false positives after updates unless `--propupd` is rerun. |
| **chkrootkit** (BSD-2-Clause AND GPL-2.0-or-later) https://www.chkrootkit.org/ | 0.58b in Fedora 44 | Signature checks for known rootkits and log tampering; root. |
| **unhide** (GPL-3.0-or-later AND MIT) | 20240510 | Cross-view comparison of `/proc`, `ps` and syscalls to find hidden processes and ports. |

- **Defender's caveat:** these tools run *on* the possibly compromised host, so a kernel-level rootkit can lie to them. Their signatures cover old, mostly server-era families. "No rootkit found" is weak evidence.
- **Bigman stance:**
  - **Integrate** as optional, explicitly root-run adapters whose output appears under "findings to review" with the tool's own caveat text.
  - **Borrow** the cross-view idea: rootless comparisons are possible, for example listening sockets from `ss` against `/proc/net/tcp`, or package-owned binaries against `rpm -Va`/`debsums` results.
  - **Avoid** presenting their pass as "no rootkit".
- See [malware families](../threats/05-malware-families-and-detection.md).

---

## 7. Malware analysis helpers

- **YARA** (BSD-3-Clause, 4.5.5) and **YARA-X** (BSD-3-Clause, Rust, 1.22.1, 2026-10-26): https://virustotal.github.io/yara-x/
  - **Bigman today:** user-supplied YARA-X rules.
  - **Integrate more deeply:** embed the `yara-x` Rust crate directly. It is license-compatible and removes an external dependency. Ship a *small, owned* default rule set (EICAR, the test fixtures, and Bigman's own capability rules expressed as YARA where that fits). Let users import third-party sets.
  - Rule sources and their terms:
    - Florian Roth's `signature-base`: Detection Rule License 1.1, which permits reuse with attribution. https://github.com/Neo23x0/signature-base
    - Elastic `protections-artifacts`: ELv2. Import by the user only.
    - `Yara-Rules/rules`: GPL-2.0, stale since 2024-04.
- **capa** (Apache-2.0, 10.0.0, 2026-10-16): identifies capabilities in PE, **ELF** (since capa v3) and shellcode, mapped to ATT&CK and MBC. https://github.com/mandiant/capa
  - It fits Bigman's "capability, not verdict" philosophy closely.
  - **Integrate optionally:** `bigman inspect` on an ELF could run capa when it is installed, and show "this binary can: open a reverse connection, enumerate files, persist via cron" as review findings. It is Python and fairly heavy, so it stays optional.
- **Detect It Easy** (MIT): file type, compiler, linker and packer identification, with ELF signature support and a CLI (`diec`). https://github.com/horsicq/Detect-It-Easy
  - **Integrate optionally** for a "packed with UPX or a UPX variant" note. Cozzi et al. found that nearly all packed Linux malware in their dataset used UPX or modified UPX (see Section 10).
  - A native, deterministic Rust check for the UPX header and high section entropy is cheap and avoids the dependency.

---

## 8. Desktop-focused security projects

| Project | What it gives a desktop user | Priv. | Bigman stance |
|---|---|---|---|
| **OpenSnitch** (GPL-3.0, 1.8.0, 2025-12) https://github.com/evilsocket/opensnitch | Interactive application firewall that prompts per outbound connection. Daemon runs as root, UI as the user. | U/R | **Integrate** (exists). Add: read its rules DB/log (rootless if permissions allow) to show "new outbound destinations by binary" in the checkup. |
| **Portmaster** (GPL-3.0, Safing, 2.2.8, 2026-10) https://safing.io/ | Per-app firewall with DNS-level filtering and filter lists; root service plus UI. | U/R | Detect and report status; offer as an alternative to OpenSnitch. Never run both. |
| **Firejail** (GPL-2.0-or-later, 0.9.80) https://firejail.wordpress.com/ | SUID-root namespace sandbox with a large profile library. CVE-2022-31214: local root via a crafted `--join` target in 0.9.68. https://nvd.nist.gov/vuln/detail/CVE-2022-31214 | SUID | **Avoid** recommending by default: a SUID-root helper adds attack surface. Detect it and report its version against known CVEs. Bubblewrap (unprivileged, used by Flatpak and by Bigman's rehearsal) is the preferred primitive. |
| **Flatseal** (GPL-3.0) https://flathub.org/apps/com.github.tchx84.Flatseal | GUI to review and override Flatpak permissions. | U | **Borrow:** Bigman can rootlessly run `flatpak list --app --columns=application` and `flatpak info --show-permissions APP` and flag broad grants (`filesystem=host`/`home`, `--socket=x11`, `--talk-name=org.freedesktop.Flatpak` (host command execution), `--device=all`) as review findings. Link the user to Flatseal to change them. See [desktop and sandboxing](../threats/08-desktop-and-sandboxing.md). |
| **USBGuard** (GPL-2.0-or-later, 1.1.4) https://usbguard.github.io/ | Allow/deny policy for USB devices (BadUSB defense). Root daemon. | R | Detect whether installed and active. Suggest it for laptops in shared spaces. Do not manage policy. |
| **GNOME "Device Security"** (GNOME Settings; introduced in GNOME 43, extended in GNOME 44 with status text and a full report) https://release.gnome.org/44/ , built on **fwupd HSI** (LGPL-2.1) https://fwupd.github.io/libfwupdplugin/hsi.html | Firmware/platform rating HSI:0 to HSI:5 (Secure Boot, BIOS write protection, TPM, IOMMU/DMA protection, kernel lockdown and taint, Intel CET, etc.). A `!` suffix marks runtime issues. | U (read) | **Integrate:** `fwupdmgr security --json` was confirmed to run as an unprivileged user on Fedora 44 during this research. Import HSI attributes into `bigman doctor` so one report covers firmware posture as well as files and startup. KDE has a comparable firmware-security module **(unverified)**. |

---

## 9. Independent tests and detection-rate evidence

**There is no current, recurring independent test of Linux desktop antivirus.** As of this research, the two main labs' Linux-specific consumer tests are from 2015. The shared search budget ran out before every lab archive could be checked, so a newer one-off test may exist **(unverified)**.

| Source | Date | What was tested | Key results |
|---|---|---|---|
| AV-TEST, "Linux: 16 Security Packages Against Windows and Linux Malware Put to the Test" https://www.av-test.org/en/news/linux-16-security-packages-against-windows-and-linux-malware-put-to-the-test/ | 2015-10-05 | 16 products on Ubuntu 12.04, about 12,000 Windows and 900 Linux samples | Only Kaspersky Endpoint reached 100% on Linux malware. ClamAV detected 15.3% of the Windows samples and was in the bottom group on Linux (23–66.1% range across that group). Only Comodo raised a false positive, on 1 of more than 210,000 clean files. |
| AV-TEST, "Test of McAfee Endpoint Security for Linux" https://www.av-test.org/en/news/test-of-mcafee-endpoint-security-for-linux/ | 2019 | Commissioned single-product test (21,895 Linux and 3,698 Windows PE samples, per search summary) | Vendor-commissioned; not comparative |
| AV-Comparatives, "Linux Security Review 2015" https://www.av-comparatives.org/wp-content/uploads/2015/05/avc_linux_2015_en.pdf | May 2015 | Feature review of 18 products, including ClamAV 0.98, mostly on Ubuntu 14.04 | A review of features and usability, not a detection ranking. It notes that ClamAV has no GUI and points to ClamTk |
| Raffa & Sgandurra, "Testing anti-virus in Linux: How effective are the solutions available for desktop computers?", Royal Holloway ISG MSc thesis series 2021 (MSc 2020) https://royalholloway.ac.uk/media/16777/giusepperaffaisg.pdf | 2021 | ClamAV, Comodo, Dr.Web, ESET over 4 scans in 3 weeks against **43,553 VirusShare ELF samples**; VirusTotal engines; Metasploit-generated payloads | Average detection ranged from 81.8% (Comodo) to **97.9% (ClamAV)**, yet ClamAV still missed about 896 samples. The VirusTotal engine average "barely reached 60%". Heuristic detection of Metasploit-generated files was as low as 8.3%. 13 VirusTotal engines showed regression between scans |
| Splunk (D. Holiday), "How Good is ClamAV at Detecting Commodity Malware?" https://www.splunk.com/en_us/blog/security/how-good-is-clamav-at-detecting-commodity-malware.html | 2022-11-28 | 416,561 MalwareBazaar samples, mostly Windows commodity families | **59.94%** overall (249,696 detected). Weak on info-stealers, RATs and miners. Described as doing well on ELF, without a published ELF percentage |
| Botacin, Ceschin, de Geus, Grégio, "We need to talk about antiviruses: challenges & pitfalls of AV evaluations", *Computers & Security* 95, 2020, doi:10.1016/j.cose.2020.101859 | 2020 | Methodology critique | Why single-snapshot detection percentages mislead: sample selection, timing and regression |
| Zhu et al., "Measuring and Modeling the Label Dynamics of Online Anti-Malware Engines", USENIX Security 2020 https://www.usenix.org/conference/usenixsecurity20/presentation/zhu | 2020 | VirusTotal labels for more than 14,000 files over a year | Labels change over time, engines are correlated, and threshold choice matters. This argues against Bigman ever using "N engines flagged it" as a verdict |

**Takeaways for Bigman**

1. Signature detection rates swing between roughly 15% and 98% depending on corpus, date and platform. Bigman's refusal to say "SAFE" is the evidence-based position, and the UI should keep saying so.
2. Detection of *crafted* or unpacked Linux payloads by signatures is poor (8.3% in the RHUL study). That is why capability rules, startup baselines and canaries matter more than a better signature engine.
3. With no lab testing Linux desktop protection, Bigman can earn trust by publishing a **reproducible self-test corpus**: EICAR, synthetic installer scripts, fake npm lifecycle hooks, a planted autostart entry. Each release then states which checks fired. This is transparency without real malware.

---

## 10. Papers and long-form references

The **Verified** note says how each entry was confirmed; "abstract only" means the full text was not read.

### Linux malware

- **Cozzi, Graziano, Fratantonio, Balzarotti (2018). "Understanding Linux Malware."** IEEE Symposium on Security and Privacy (S&P), pp. 161–175, doi:10.1109/SP.2018.00054. PDF: https://www.s3.eurecom.fr/docs/oakland18_cozzi.pdf
  - *Takeaway:* the first large measurement (10,548 ELF samples) with a Linux-specific analysis pipeline. In the full text:
    - Only **21%** of samples implemented any persistence, but those that did often tried several methods in a row.
    - `rc.local` and SysV init scripts dominated. Only **two** samples used systemd.
    - Cron was second.
    - User-level fallbacks were `~/.bashrc`, `~/.bash_profile` and X desktop autostart `.desktop` files.
    - Some families replaced `ps`/`netstat` in `/bin` or `/usr/bin`.
    - Most packed samples used UPX or modified UPX.
  - These are exactly Bigman's baseline targets. They also show why `rpm -Va`/`debsums` and a UPX check are worthwhile.
  - *Verified:* DBLP/Eurecom/IEEE DOI; PDF read.
- **Cozzi, Vervier, Dell'Amico, Shen, Bilge, Balzarotti (2020). "The Tangled Genealogy of IoT Malware."** ACSAC 2020, doi:10.1145/3427228.3427256.
  - *Takeaway:* Linux/IoT malware lineages share and reuse code heavily (Mirai/Gafgyt descendants). Family-level signatures and YARA rules generalise well.
  - *Verified:* Crossref; abstract-level.
- **Antonakakis et al. (2017). "Understanding the Mirai Botnet."** USENIX Security 2017, pp. 1093–1110. https://www.usenix.org/conference/usenixsecurity17/technical-sessions/presentation/antonakakis
  - *Takeaway:* simple credential-guessing malware compromised about 600k Linux-based devices. Default credentials and exposed services matter more than exotic exploits ([network exposure](../threats/07-network-exposure-and-remote-access.md)).
  - *Verified:* usenix.org.

### Antivirus evaluation

- Raffa & Sgandurra (2021); Splunk (2022); Botacin et al. (2020); Zhu et al. (2020): see [Section 9](#9-independent-tests-and-detection-rate-evidence).

### Package managers and software update security

- **Cappos, Samuel, Baker, Hartman (2008). "A Look in the Mirror: Attacks on Package Managers."** ACM CCS 2008, pp. 565–574, doi:10.1145/1455770.1455841. https://www2.cs.arizona.edu/stork/packagemanagersecurity/ccs2008.pdf
  - *Takeaway:* ten package managers (APT, YUM, YaST and others) were vulnerable to malicious mirrors and man-in-the-middle **replay, freeze and endless-data attacks**. The authors obtained official mirror status at all five distributions they tried, using false credentials. It is the root reference for checking metadata freshness and mirror trust ([distro packaging](../threats/03-distro-packaging-and-install-channels.md)).
  - *Verified:* Arizona page; DOI.
- **Samuel & Cappos (2009). "Package Managers Still Vulnerable."** ;login:
  - *Takeaway:* APT and YaST had improved since the 2008 disclosure, while others remained vulnerable to replay, freeze and endless-data attacks.
  - *Verified:* listed at https://www2.cs.arizona.edu/stork/packagemanagersecurity/papers.html; exact issue **unverified**.
- **Samuel, Mathewson, Cappos, Dingledine (2010). "Survivable Key Compromise in Software Update Systems."** ACM CCS 2010, pp. 61–72, doi:10.1145/1866307.1866315. PDF: https://uptane.org/assets/files/samuel_ccs_2010-2e4e7a69695e936b1ad3b0b7010c550a.pdf
  - *Takeaway:* introduces **TUF**: role separation, thresholds, expiring metadata and key revocation, so that a single key compromise is survivable. https://theupdateframework.io/
  - *Verified:* Crossref.
- **Kuppusamy, Torres-Arias, Diaz, Cappos (2016). "Diplomat: Using Delegations to Protect Community Repositories."** USENIX NSDI 2016. https://www.usenix.org/conference/nsdi16/technical-sessions/presentation/kuppusamy
  - *Takeaway:* applies TUF delegations to community registries such as PyPI.
  - *Verified:* usenix.org title.
- **Torres-Arias, Afzali, Kuppusamy, Curtmola, Cappos (2019). "in-toto: Providing farm-to-table guarantees for bits and bytes."** USENIX Security 2019, pp. 1393–1410. https://www.usenix.org/conference/usenixsecurity19/presentation/torres-arias
  - *Takeaway:* signed, verifiable build-step attestations from source to install.
  - *Verified:* usenix.org.
- **Newman, Meyers, Torres-Arias (2022). "Sigstore: Software Signing for Everybody."** ACM CCS 2022, doi:10.1145/3548606.3560596.
  - *Takeaway:* keyless signing with a transparency log. This is the mechanism behind npm and PyPI provenance that Bigman could check.
  - *Verified:* Crossref.
- **Lamb & Zacchiroli (2022). "Reproducible Builds: Increasing the Integrity of Software Supply Chains."** IEEE Software 39(2), doi:10.1109/MS.2021.3073045; arXiv:2104.06020.
  - *Takeaway:* independent rebuilds let users check that binaries match source. It is directly relevant to the xz lesson that release tarballs differed from git.
  - *Verified:* Crossref and arXiv.

### Open-source supply chain

- **Ohm, Plate, Sykosch, Meier (2020). "Backstabber's Knife Collection: A Review of Open Source Software Supply Chain Attacks."** DIMVA 2020, LNCS, pp. 23–43, doi:10.1007/978-3-030-52683-2_2; arXiv:2005.09535.
  - *Takeaway:* 174 malicious npm, PyPI and RubyGems packages (2015–2019), with attack trees. Most malicious code triggers **at install time** through lifecycle scripts, which is the basis for Bigman's `package.lifecycle` rule and `ignore-scripts` policy.
  - *Verified:* Crossref and arXiv.
- **Ladisa, Plate, Martinez, Barais (2023). "SoK: Taxonomy of Attacks on Open-Source Software Supply Chains."** IEEE S&P 2023, pp. 1509–1526, doi:10.1109/SP46215.2023.10179304; arXiv:2204.04008.
  - *Takeaway:* an attack tree of 107 vectors tied to 94 real incidents, mapped to 33 safeguards. Use it as the coverage checklist for [supply chain](../threats/02-supply-chain-packages.md).
  - *Verified:* Crossref and arXiv.
  - Follow-up: Ladisa et al., "Journey to the Center of Software Supply Chain Attacks", arXiv:2304.05200 (2023), with an extended safeguards list. *Verified:* arXiv.
- **Zimmermann, Staicu, Tenny, Pradel (2019). "Small World with High Risks: A Study of Security Threats in the npm Ecosystem."** USENIX Security 2019. https://www.usenix.org/conference/usenixsecurity19/presentation/zimmerman
  - *Takeaway:* a handful of maintainer accounts could inject code into most of npm, so **maintainer account takeover is a systemic risk**. This supports Bigman's phishing focus on maintainers.
  - *Verified:* usenix.org.
- **Duan, Alrawi, Kasturi, Elder, Saltaformaggio, Lee (2021). "Towards Measuring Supply Chain Attacks on Package Managers for Interpreted Languages."** NDSS 2021. https://www.ndss-symposium.org/ndss-paper/towards-measuring-supply-chain-attacks-on-package-managers-for-interpreted-languages/
  - *Takeaway:* found 339 new malicious packages, 278 confirmed by registries, 3 with more than 100k downloads. Static and dynamic metadata analysis works at registry scale.
  - *Verified:* NDSS page.
- **Zahan, Zimmermann, Godefroid, Murphy, Maddila, Williams (2022). "What are Weak Links in the npm Supply Chain?"** ICSE-SEIP 2022, doi:10.1145/3510457.3513044; arXiv:2112.10165.
  - *Takeaway:* weak-link signals such as maintainer email domains that have expired and can be re-registered, install scripts, and too many maintainers. Several are cheap deterministic checks Bigman's `packages` workflow could add.
  - *Verified:* Crossref.

### xz-utils backdoor (CVE-2024-3094) post-mortems

- **Andres Freund (2024-03-29). "backdoor in upstream xz/liblzma leading to ssh server compromise."** oss-security. https://www.openwall.com/lists/oss-security/2024/03/29/4
  - *Takeaway:* caught through a performance anomaly (slow sshd logins, valgrind errors) on Debian sid. The payload sat in test files plus a build-script change present only in release tarballs.
  - *Verified:* fetched.
- **Lasse Collin. "XZ Utils backdoor."** https://tukaani.org/xz-backdoor/
  - *Takeaway:* the maintainer's own account. 5.6.0 and 5.6.1 tarballs were built and signed by "Jia Tan", and the trigger code was never in git. It covers account and DNS cleanup, the clean releases of 2024-05-29, and review notes.
  - *Verified:* fetched.
- **Russ Cox (2024-04-01, updated 2024-04-03). "Timeline of the xz open source attack."** https://research.swtch.com/xz-timeline
  - *Takeaway:* a multi-year social-engineering campaign with pressure from apparently fake accounts on a lone maintainer, and a payload hidden in binary test files. Distributions that do not patch sshd to link libsystemd were not affected.
  - *Verified:* fetched.
- **Przymus & Durieux (2025). "Wolves in the Repository: A Software Engineering Analysis of the XZ Utils Supply Chain Attack."** MSR 2025; arXiv:2504.17473.
  - *Takeaway:* the attacker exploited *engineering practices* (community management, CI, build-system and test changes, translations) to gain legitimacy. For defenders, the lesson is to compare release tarballs with VCS and to scrutinise build-system and test-fixture changes.
  - *Verified:* arXiv and MSR program; abstract-level.
- Advisories: NVD CVE-2024-3094 https://nvd.nist.gov/vuln/detail/CVE-2024-3094 ; CISA alert (2024-03-29) https://www.cisa.gov/news-events/alerts/2024/03/29/reported-supply-chain-compromise-affecting-xz-utils-data-compression-library-cve-2024-3094 ; Red Hat, "Urgent security alert for Fedora 40 and Fedora Rawhide users" https://www.redhat.com/en/blog/urgent-security-alert-fedora-41-and-rawhide-users (the URL slug says 41 but the title says 40).
- *For Bigman:* the advisory pattern "installed package version X is in a known-compromised range" is the simplest form of the owner's requested "update package X because of CVE Y" feature. See [vulnerability intelligence and feeds](./vulnerability-intelligence-and-feeds.md).

### eBPF and kernel runtime security monitoring

- **Fournier, Afchain, Baubeau (Datadog) (2021). "Runtime Security Monitoring with eBPF."** SSTIC 2021. https://www.sstic.org/media/SSTIC2021/SSTIC-actes/runtime_security_with_ebpf/SSTIC2021-Article-runtime_security_with_ebpf-fournier_afchain_baubeau.pdf
  - *Takeaway:* a practitioner design for container-aware, kernel-event-based detection. Use it as a reference for what root-level telemetry adds that rootless polling cannot provide.
  - *Verified:* PDF resolves; author names from file name and search summary.
- **Findlay, Somayaji, Barrera (2020). "bpfbox: Simple Precise Process Confinement with eBPF."** ACM CCSW 2020, pp. 91–103, doi:10.1145/3411495.3421358. https://people.scs.carleton.ca/~soma/pubs/findlay-ccsw2020.pdf
  - *Takeaway:* eBPF + LSM (KRSI) confinement in under 2,000 lines of kernel code. The project was superseded by BPFContain (Rust).
  - *Verified:* PDF and DOI.
- **LWN: "Kernel runtime security instrumentation"** (2019) https://lwn.net/Articles/798157/ and **"Impedance matching for BPF and LSM"** (2020) https://lwn.net/Articles/813261/
  - *Takeaway:* the history of BPF LSM (KRSI), the kernel hook that Tetragon-class enforcement builds on.
  - *Verified:* titles fetched.
- **He et al. (2023). "Cross Container Attacks: The Bewildered eBPF on Clouds."** USENIX Security 2023, pp. 5971–5988. https://www.usenix.org/conference/usenixsecurity23/presentation/he
  - *Takeaway (defensive reading):* eBPF has a coarse permission model, and access to it is powerful enough that existing IDSs rarely notice its misuse. Defenders should audit who holds `CAP_BPF`/`CAP_SYS_ADMIN` and which BPF programs are loaded.
  - *Verified:* usenix.org.
- **Ghimire, Bhurtel, Sahani, Jha (2025). "eBPF-PATROL: Protective Agent for Threat Recognition and Overreach Limitation using eBPF in Containerized and Virtualized Environments."** arXiv:2511.18155 (preprint, not peer-reviewed).
  - *Takeaway:* a rule-based eBPF syscall agent with reported overhead below 2.5%.
  - *Verified:* arXiv.
- Vendor documentation: Microsoft's eBPF sensor for MDE https://learn.microsoft.com/en-us/defender-endpoint/linux-support-ebpf ; Elastic's GPL/BSD eBPF probes https://github.com/elastic/ebpf . Both show that the industry has moved from kernel modules to eBPF.
- Stability counterpoint: The Register on CrowdStrike's user-mode BPF sensor and RHEL 9.4 panics (2024) https://www.theregister.com/2024/07/21/crowdstrike_linux_crashes_restoration_tools/

### General reference

- MITRE ATT&CK, Enterprise Linux matrix: https://attack.mitre.org/matrices/enterprise/linux/ . Use its technique IDs as stable cross-references in Bigman findings.

---

## 11. Positioning: where Bigman fits

### What the market looks like

1. **Server and fleet tools dominate.** Every commercial product surveyed except ESET's Endpoint Antivirus for Linux is positioned for servers and fleets: Microsoft says "Linux server workloads", and Sophos's release notes discuss only servers. The open-source EDR and HIDS tools (Wazuh, OSSEC, Velociraptor, Falco, Tracee, Tetragon, Fleet) all assume root plus a manager or SIEM.
2. **Free desktop antivirus has been shrinking.** The Sophos free edition was retired, and the only popular ClamAV GUI (ClamTk) has been unmaintained since April 2024 while still being packaged and installed.
3. **Desktop tools are single-purpose.** OpenSnitch and Portmaster cover network egress, Flatseal Flatpak permissions, USBGuard USB devices, and GNOME Device Security firmware. Nothing combines their status into one explained report.
4. **No current independent test covers Linux desktop protection.** Users cannot compare products on evidence.
5. **Developer supply chain tools are separate and CI-oriented.** OSV-Scanner, Socket-style registry scanners and Sigstore verification are built for CI and registries, not for "I am about to run this `curl | bash` or `npm install` on my laptop".

### Gaps a one-stop, deterministic, rootless tool can fill

| # | Gap | Why nobody fills it | What Bigman does or should do | Status |
|---|---|---|---|---|
| G1 | **Maintained desktop ClamAV front end** with honest results | ClamTk unmaintained; vendors went business-only | Scan, quarantine/restore, freshness timers, "no findings from these checks" language | Done; keep stating signature date per report |
| G2 | **Pre-execution inspection** of installers and packages on the user's machine | EDRs act after execution; registry scanners act on the registry side | `inspect`, `packages`, `package-policy`, offline Bubblewrap rehearsal. Extend to PyPI, crates, RubyGems, AUR PKGBUILDs, `.deb`/`.rpm` maintainer scripts | Partial |
| G3 | **Rootless persistence baselining** of the kind a CCDC blue team does by hand | AIDE, rkhunter and auditd need root and are noisy; EDRs need fleets | Extend `changes` to the locations Cozzi et al. and Velociraptor artifacts enumerate (cron dirs, `/etc/profile.d`, user crontab, systemd user units, `.desktop` autostart, git hooks, `~/.ssh/config`), plus "executable not owned by any package" | Partial ([host integrity](../threats/04-host-integrity-and-hardening-audit.md)) |
| G4 | **Repository and install-channel trust audit** | No desktop tool lists third-party repos, keys and `gpgcheck=0`/`[trusted=yes]` | Rootless read of `/etc/apt/sources.list.d`, `/etc/yum.repos.d`, keyrings, Flatpak remotes, Snap channels; flag unsigned or unexpected sources | Gap ([distro packaging](../threats/03-distro-packaging-and-install-channels.md)) |
| G5 | **"Update X because of CVE Y" for one machine** | MDE and Wazuh do it for fleets; distro tools do not explain it | Map installed package versions to OSV/distro advisories (OSV-Scanner adapter exists for project dirs; extend to the host package DB) and feed-driven advisories | Partial ([feeds](./vulnerability-intelligence-and-feeds.md)) |
| G6 | **One posture report across siloed desktop controls** | Each project shows only its own status | `doctor` imports fwupd HSI, firewall (OpenSnitch/Portmaster/firewalld/ufw) state, USBGuard, LKRG, SELinux/AppArmor mode, Flatpak permission review, sshd exposure, unattended-upgrades/dnf-automatic status | Partial |
| G7 | **Maintainer-specific protection** (phishing, tokens, signing keys) | AV and EDR do not model a maintainer's assets | Browser checks (exist), plaintext credential stores, token files, SSH/GPG key hygiene, GitHub/npm 2FA reminders, canary tokens | Partial ([phishing](../threats/01-phishing-social-engineering.md), [credentials](../threats/06-credential-and-secret-protection.md)) |
| G8 | **Explainable, deterministic, privacy-preserving results** | Vendors rely on cloud ML and telemetry; "AI-powered" marketing | Stable rule IDs, evidence, coverage and "incomplete" states, no uploads; ATT&CK IDs on findings | Done in principle; keep as a hard constraint |
| G9 | **Reproducible public self-test** | Labs stopped testing Linux desktops | Ship a synthetic fixture corpus and publish which checks fired per release | Gap |
| G10 | **Optional bridge to root-level telemetry** without becoming an agent | Running eBPF/auditd is outside a rootless tool's remit | Opt-in readers for auditd, Kunai, Sysmon-for-Linux or Falco JSON when the user already runs them; a previewed audit-rule installer via polkit | Gap |

### What Bigman should deliberately avoid

- **Kernel modules, its own eBPF programs, or SUID helpers.** Kernel-coupled EDRs have crashed production kernels (CrowdStrike on RHEL 9.4, 2024), and SUID sandboxes have had local-root bugs (Firejail CVE-2022-31214).
- **On-access blocking by default.** It needs root and fanotify permission support, and can lock up a desktop. Bigman's model is review, not interception.
- **Automatic remediation or process killing** (Tetragon/EDR style). It conflicts with Bigman's evidence-first, explicit-action design.
- **Cloud reputation or sample upload,** and multi-engine "N of M flagged" verdicts. Zhu et al. show these labels are unstable.
- **Claims of cleanliness** from rootkit checkers or signature scans. The detection-rate literature does not support them.
- **Bundling incompatible content:** GPL-2.0-only code (ClamAV, Wazuh/OSSEC rules, LKRG), Elastic License 2.0 rules, vendor signatures. Call these tools as subprocesses or let users import content.
- **Becoming a fleet product.** Wazuh, Velociraptor, Fleet and the commercial EDRs already serve that market well. Bigman's niche is the individual workstation owner and maintainer.

---

## 12. Unverified items and open questions

**Unverified in this pass** (the shared search budget ran out):

- The exact retirement and last-update dates of the Sophos Anti-Virus for Linux free edition. The 20 July 2023 end of life for v9 comes from a Sophos community thread.
- Bitdefender BEST for Linux internals (on-access mechanism, EDR sensor type) and SentinelOne Linux agent architecture.
- CIS Benchmarks licence terms (believed to be free for non-commercial use under a Creative Commons NC licence; not confirmed).
- rkhunter 1.4.6 release year.
- Exact Velociraptor Linux artifact names.
- Whether a ClamTk fork or successor exists.
- Whether ClamAV 1.4 is an LTS series.
- KDE's firmware-security module.
- Whether AV-TEST or AV-Comparatives published any Linux consumer test after 2019.
- The issue and volume of Samuel & Cappos, "Package Managers Still Vulnerable".
- SSTIC 2021 author list (inferred from the PDF file name).
- Full-text claims of the Tangled Genealogy and Wolves in the Repository papers (abstract-level only).

**Open questions for the maintainer**

1. Should Bigman embed `yara-x` as a crate and ship a small owned rule set, or keep YARA strictly user-supplied?
2. Is an opt-in, polkit-installed auditd rule pack acceptable within the "rootless by default" promise, if it is previewed and reversible?
3. Which external event sources (Kunai, Sysmon for Linux, Falco JSON) are worth an importer, given that most target users will not run any of them?
4. Should `doctor` grow into the G6 "single posture report" (fwupd HSI, firewall, USBGuard, LKRG, MAC mode, Flatpak permissions), and how should the UI weight firmware findings that users often cannot fix?
5. Is publishing a synthetic self-test corpus per release (G9) in scope for v0.3?
