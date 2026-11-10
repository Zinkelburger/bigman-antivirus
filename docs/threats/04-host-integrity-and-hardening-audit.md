# Host integrity and hardening audit (blue-team checklist)

> Scope: what a defender baselines and reviews on a Linux desktop or developer workstation to notice unauthorized persistence, privilege changes and integrity loss, and to confirm the host is patched against well-known local privilege escalation (LPE) bugs. It is written in the style of CIS Benchmarks, Lynis, osquery packs, chkrootkit/rkhunter and AIDE. Each item says what a normal state looks like, which read-only commands inventory it, the MITRE ATT&CK technique it maps to, and the equivalent Lynis / CIS / osquery check. It also says what Bigman's startup baseline covers today. It does not describe how attackers install or hide anything. Distro packaging channels belong to [03-distro-packaging-and-install-channels](./03-distro-packaging-and-install-channels.md), malware families to [05-malware-families-and-detection](./05-malware-families-and-detection.md), credential files to [06-credential-and-secret-protection](./06-credential-and-secret-protection.md), listening services and SSH exposure to [07-network-exposure-and-remote-access](./07-network-exposure-and-remote-access.md), and vulnerability feeds to [vulnerability-intelligence-and-feeds](../research/vulnerability-intelligence-and-feeds.md). Last researched: 2026-10-29.

**Sources for mappings.** ATT&CK IDs were checked against attack.mitre.org on 2026-10-29. ATT&CK has since moved the old *Impair Defenses* sub-techniques: T1562.012 now redirects to **T1685.004** and T1070.002 to **T1685.006**, so this file uses the new IDs. Lynis test IDs come from `db/tests.db` in CISOfy/lynis (master, 449 tests). CIS section numbers are from the **CIS Ubuntu Linux 24.04 LTS Benchmark v1.0.0**, as encoded in ComplianceAsCode `controls/cis_ubuntu2404.yml` (BSD-3-Clause). The CIS PDF was not read directly, and numbering differs between distros and benchmark versions. "SSG" means a ComplianceAsCode/SCAP Security Guide rule ID. osquery names are table names from `osquery/osquery/specs`. CVE facts come from NVD and from the CISA KEV catalog (catalog version 2026.10.28).

**Rootless reality check.** Bigman runs without root. On a default Fedora 44 host (observed 2026-10-29), an unprivileged user **cannot** read `/etc/shadow` (mode 0000), `/etc/sudoers` (0440), `/etc/sudoers.d/` (0750), `/etc/polkit-1/rules.d/` (0750 root:polkitd), `/etc/ssh/sshd_config` and `sshd_config.d/` (0600/0700), `/var/spool/cron/` or `/var/spool/at/` (0700). The same user **can** read `/etc/passwd`, `/etc/group`, `/etc/cron.d`, `/etc/crontab`, `/etc/pam.d`, `/etc/profile.d`, `/etc/udev/rules.d`, `/etc/ld.so.conf.d`, `/usr/share/polkit-1/rules.d`, `/proc/modules`, `/proc/sys/kernel/tainted`, `/sys/kernel/security/lockdown` and the SecureBoot efivar. The checklist marks which items need an opt-in privileged read.

## Summary table

| ID | Attack | Severity for a Linux desktop/dev user | Bigman today | Proposed |
|---|---|---|---|---|
| HI-01 | Unreviewed system systemd units, drop-ins, generators and timers | High | Partial | Baseline all unit search paths plus `systemd-delta`; attribute each file to its package |
| HI-02 | Unreviewed per-user systemd units/timers and lingering | High | Partial | Add `~/.local/share/systemd/user`, `/etc/systemd/user`, the linger list; allowlist Bigman's own units |
| HI-03 | Cron and at jobs outside the baseline | High | Partial | Add `cron.{hourly,daily,weekly,monthly}`, anacrontab, own `crontab -l`, `atq`; privileged spool read |
| HI-04 | XDG autostart and session-start hooks | High | Partial | Add `/etc/xdg/autostart`, `$XDG_CONFIG_DIRS`, X/KDE session scripts; show the `Exec=` line in findings |
| HI-05 | Shell startup and environment files | High | Partial | Cover all bash/zsh/fish/profile/environment.d files; line-level diff with markers for preload/PATH/alias lines |
| HI-06 | rc.local, SysV init scripts, MOTD scripts, network dispatcher hooks | Medium | Partial | Hash these dirs; flag any that are executable and not owned by a package |
| HI-07 | udev rules that run programs | Medium | Partial | Baseline `/etc/udev/rules.d`, `/run/udev/rules.d`, `/usr/lib/udev/rules.d`; flag `RUN`/`PROGRAM` in unowned rules |
| HI-08 | PAM stack and PAM module files | High | Partial | Baseline `/etc/pam.d`, `/etc/security`, module dirs; `rpm -V pam` / `dpkg --verify`; flag unowned `.so` files |
| HI-09 | Dynamic linker configuration (preload, ld.so.conf.d, LD_* env) | High | Partial | Add `ld.so.conf(.d)`, `/etc/environment`; check own processes for LD_PRELOAD; static helper for cross-view reads |
| HI-10 | SSH authorized keys, sshd configuration, per-user SSH hooks | High | Partial | Key-fingerprint diff, `~/.ssh/rc`, `authorized_keys2`, `~/.ssh/config`; privileged `sshd -T` |
| HI-11 | Module autoload, modprobe directives, package-manager and kernel-install hooks | Medium | Partial | Baseline `modules-load.d`, `modprobe.d`, apt/dnf/pacman hook dirs, `kernel/postinst.d`, `install.d` |
| HI-12 | Extra UID 0, duplicate IDs, new or shell-enabled system accounts | High | Gap | Rootless `/etc/passwd` and `/etc/group` parse plus diff (`host.uid0-extra`, `host.account-new`) |
| HI-13 | Empty or unlocked passwords, `nullok` in PAM | Medium | Gap | Rootless `nullok` scan; privileged read of shadow status only (never hashes) |
| HI-14 | sudoers and sudoers.d grants (NOPASSWD, !authenticate, broad ALL) | High | Gap | Opt-in privileged `visudo -c` and a parsed summary; baseline hashes |
| HI-15 | polkit rules and implicit authorizations | Medium | Gap | Baseline `/usr/share/polkit-1/rules.d` (rootless) and `/etc/polkit-1/rules.d` (privileged); `pkaction` defaults diff |
| HI-16 | Root-equivalent group membership (docker, disk, libvirt, lxd, wheel/sudo, input) | High | Partial (docker) | Rootless group audit with plain-language explanation of each group |
| HI-17 | SUID/SGID files not owned by a package or with changed mode | Medium | Partial | Bounded `find -perm` inventory plus package ownership and mode verification |
| HI-18 | File capabilities on unexpected binaries | Medium | Partial | `security.capability` xattr inventory compared with the package DB (`rpm -V` P flag) |
| HI-19 | World-writable files and dirs, writable PATH entries, unowned files | Medium | Partial | `persistence.shadowed-command` watches `~/.local/bin` and `~/bin` for files that hide system commands; still to do: root `secure_path`, world-writable without sticky, nosuid mounts |
| HI-20 | Loaded kernel modules not owned by a package, unsigned or tainting | High | Partial | `/proc/modules` to `modinfo` to package owner; decode taint bits; signer inventory |
| HI-21 | Secure Boot, lockdown, signature enforcement and boot chain state | Medium | Partial (Secure Boot state) | Read SB/lockdown/cmdline; MOK list diff; `/boot` package verify; fwupd HSI if present |
| HI-22 | Package-file verification (modified or unowned system files) | High | Partial | Scheduled `rpm -Va` / `dpkg --verify` / `pacman -Qkk` with the diff noise explained |
| HI-23 | Kernel and userspace patch status; well-known Linux LPE CVEs | High | Partial | Distro advisory query, running vs installed kernel, reboot-needed, KEV-driven checklist |
| HI-24 | Audit, journal and integrity-tool health | Medium | Partial | Report auditd, journald persistence, AIDE DB age, and its own monitor heartbeat in `doctor` |
| HI-25 | Kernel attack-surface sysctls and rarely used module blocklists | Medium | Partial (three sysctls) | Read-only sysctl report with desktop trade-offs (user namespaces are needed by sandboxes) |

## Bigman's startup baseline today (from `src/system.rs`)

`bigman changes` calls `snapshot()`, which walks these ten roots without following symlinks (max depth 8). It records SHA-256 and byte length for each regular file. For a symlink, it hashes the **link target text**, not the target's contents. The monitor (`src/canary.rs`) re-runs the comparison every 300 s and sends a desktop notification on change. Findings are `persistence.initial` (Info), `persistence.change` and `persistence.removed` (Review). Removals are reported only when the snapshot had no read errors. Accepting a baseline is refused if the inventory changed since the reviewed report or if any read error occurred. Limits are 10,000 entries and 64 MiB. Canary decoy files are excluded.

| Location | Scope | In baseline | Rootless readable (Fedora 44) | ATT&CK |
|---|---|---|---|---|
| `~/.config/systemd/user` | user | **Yes** | yes | T1543.002, T1053.006 |
| `~/.config/autostart` | user | **Yes** | yes | T1547.013 |
| `~/.ssh/authorized_keys` | user | **Yes** (whole-file hash) | yes | T1098.004 |
| `~/.bashrc`, `~/.zshrc`, `~/.profile` | user | **Yes** | yes | T1546.004 |
| `/etc/systemd/system` | system | **Yes** (enablement symlinks hashed as link text) | yes | T1543.002, T1053.006 |
| `/etc/cron.d`, `/etc/crontab` | system | **Yes** | yes | T1053.003 |
| `/etc/ld.so.preload` | system | **Yes** (absent is normal; appearance is a "New startup entry") | yes | T1574.006 |
| `~/.local/share/systemd/user`, `/etc/systemd/user`, `/usr/lib/systemd/{system,user}`, `/usr/local/lib/systemd`, `/run/systemd/*`, generator dirs, `/var/lib/systemd/linger` | both | No | yes | T1543.002 |
| `/etc/cron.{hourly,daily,weekly,monthly}`, `/etc/anacrontab` | system | No | yes | T1053.003 |
| `/var/spool/cron/*`, at spool | both | No | **no** (0700) | T1053.003, T1053.002 |
| `/etc/xdg/autostart`, `~/.xprofile`, `~/.xinitrc`, `/etc/X11/xinit/xinitrc.d`, KDE `plasma-workspace/env` | both | No | yes | T1547.013 |
| `~/.bash_profile`, `~/.bash_login`, `~/.bash_logout`, `~/.zshenv`, `~/.zprofile`, `~/.zlogin`, `~/.config/fish`, `~/.config/environment.d`, `~/.pam_environment` | user | No | yes | T1546.004 |
| `/etc/profile`, `/etc/profile.d`, `/etc/bashrc` or `/etc/bash.bashrc`, `/etc/zshrc` or `/etc/zsh/`, `/etc/environment`, `/etc/security/pam_env.conf` | system | No | yes | T1546.004 |
| `/etc/rc.local`, `/etc/rc.d/rc.local`, `/etc/init.d`, `/etc/update-motd.d`, `/etc/NetworkManager/dispatcher.d` | system | No | yes | T1037.004, T1037 |
| `/etc/udev/rules.d`, `/usr/lib/udev/rules.d` | system | No | yes | T1546.017 |
| `/etc/pam.d`, `/etc/security`, PAM module dirs | system | No | yes | T1556.003 |
| `/etc/ld.so.conf`, `/etc/ld.so.conf.d` | system | No | yes | T1574.006 |
| `~/.ssh/rc`, `~/.ssh/authorized_keys2`, `~/.ssh/config`, `~/.ssh/environment` | user | No | yes | T1098.004 |
| `/etc/ssh/sshd_config`, `sshd_config.d`, `/etc/ssh/sshrc` | system | No | **no** (0600/0700) | T1098.004, T1021.004 |
| `/etc/modules-load.d`, `/etc/modprobe.d`, `/etc/modules` | system | No | yes | T1547.006 |
| `/etc/apt/apt.conf.d`, `/etc/dnf/plugins`, `/etc/pacman.d/hooks`, `/etc/kernel/{postinst.d,install.d}` | system | No | yes | T1546.016 |
| `/etc/sudoers`, `/etc/sudoers.d` | system | No | **no** | T1548.003 |
| `/etc/polkit-1/rules.d` / `/usr/share/polkit-1/rules.d` | system | No | **no** / yes | T1548 |
| `/etc/passwd`, `/etc/group` | system | No | yes | T1136.001 |

**Structural gaps in the current design** (from reading the code):
1. **Content only.** File mode, owner, group, inode, mtime and xattrs (including `security.capability` and SELinux labels) are not recorded. A `chmod`, `chown`, setuid-bit or capability change on a covered file is therefore invisible.
2. **Enablement symlinks are hashed as link text.** A changed vendor unit under `/usr/lib/systemd/system` that an `/etc/systemd/system/*.wants/` link points to is not noticed.
3. **No package attribution.** The recommendation text asks the user to "review the owning package", but Bigman does not run `rpm -qf`/`dpkg -S` or verify the file against the package DB. Ordinary package updates and admin edits therefore look the same as unexplained changes.
4. **No parsing.** A changed `authorized_keys` shows as one hash change, with no list of which key fingerprint was added. A changed `.bashrc` does not show which lines changed. Bigman could keep a private copy of small text files (under 64 KiB) at baseline time so it can show a line diff.
5. **An unreadable file makes the whole report incomplete.** That is the correct conservative behaviour, but it means adding root-only paths (sudoers, spool) to the rootless snapshot would permanently block baseline acceptance. Those paths need a separate, optional privileged collector.
6. **Bigman's own units** (`bigman-monitor.service`, `bigman-scan.{service,timer}`, `bigman-update.{service,timer}`) appear as initial inventory in `~/.config/systemd/user`. They should be labelled as Bigman-owned, with their expected hashes.

## Attacks

Each entry adds two fields to the standard template. **Normal state** describes what a healthy desktop usually looks like. **Equivalent checks** lists the matching Lynis test, CIS section, SSG rule, osquery table and auditd watch. All commands are read-only. `sudo` marks the ones that need root.

### HI-01 Unreviewed system systemd units, drop-ins, generators and timers
- **What it is:** systemd starts whatever unit files sit in its search path. Units in `/etc/systemd/system` override vendor units in `/usr/lib/systemd/system`, drop-in `*.d/*.conf` files can change `ExecStart=` or `Environment=` of an existing unit, and generators run at every boot and daemon-reload. A root-level unit or timer that nobody reviewed runs as root on every boot, so this is the most common Linux persistence location in ATT&CK procedure examples.
- **Normal state:** `/etc/systemd/system` contains mostly `*.wants/` symlinks into `/usr/lib/systemd/system`, plus a few admin drop-ins. Every file under `/usr/lib/systemd` is owned by a package. Generator directories under `/etc` and `/usr/local` are empty. `systemd-delta` lists only overrides the admin knows about.
- **Real-world examples:**
  - Sandworm configured systemd for persistence during the 2022 Ukraine electric-power attack, and Rocke, Fysbis, Gomir and RotaJakiro install systemd services. All are listed on [ATT&CK T1543.002](https://attack.mitre.org/techniques/T1543/002/).
  - CanisterWorm registered a systemd service on Kubernetes nodes ([ATT&CK T1053.006](https://attack.mitre.org/techniques/T1053/006/)).
  - Pierre Berba's threat-hunting write-up on systemd generators (2022): https://pberba.github.io/security/2022/02/07/linux-threat-hunting-for-persistence-systemd-generators/
- **What a defender inspects (indicators):**
  - `systemctl list-unit-files --state=enabled --no-pager`, `systemctl list-timers --all --no-pager`
  - `systemd-delta --no-pager` lists overridden, extended, masked and redirected units.
  - `systemd-analyze unit-paths` gives the search path. `systemctl cat UNIT` shows the effective unit with its drop-ins.
  - `find /etc/systemd /usr/local/lib/systemd /run/systemd/system -xdev -type f -printf '%TY-%Tm-%Td %u %m %p\n' | sort`
  - Ownership: `rpm -qf FILE` or `dpkg -S FILE`. Integrity: `rpm -V systemd` or `dpkg --verify systemd`.
  - Suspicious in general terms: a unit or generator not owned by any package, an `ExecStart=` pointing into `/tmp`, `/dev/shm`, `/var/tmp` or a home directory, a unit name imitating a system daemon, a drop-in that adds `Environment=LD_PRELOAD`, or a timer with a very short interval.
- **Equivalent checks:**
  - ATT&CK: T1543.002, T1053.006
  - Lynis: BOOT-5177 (boot and running services), BOOT-5264 (`systemd-analyze security`)
  - osquery: `systemd_units`, `startup_items`
  - auditd (Neo23x0): `-w /etc/systemd/ -k systemd`, `-w /usr/lib/systemd -k systemd`
  - CIS/SSG: no control for unknown units; CIS only covers disabling services that are not needed.
- **Bigman today:** Partial. `bigman changes` baselines `/etc/systemd/system` and `/etc/systemd/user` with content, mode, owner and file capabilities. Each new or changed file is attributed to its package (owned and matching, owned but modified, unowned). Vendor unit dirs, `/run`, generators and target contents are not covered.
- **Proposed mitigation / detection:** In the CLI and monitor, extend `snapshot()` to every `systemd-analyze unit-paths` entry plus the `system-generators` and `user-generators` dirs. Resolve enablement symlinks and hash the target's contents. Store `systemd-delta` output in the report. For each new or changed file, run `rpm -qf`/`dpkg -S`, then `rpm -Vf`/`dpkg --verify` on the owner. A file that is package-owned and verifies clean is labelled "changed by package update". A file that is unowned, or package-owned but failing verification, gets a `persistence.unowned` / `persistence.modified-package-file` Review finding. All of this is rootless.
- **Test case idea:** Point the snapshot at a tempdir prefix (refactor `snapshot()` to take a root prefix). Create `etc/systemd/system/benign-test.service` with `ExecStart=/bin/true`, then assert `persistence.change`. Add a fake package-owner resolver that returns "not owned" and assert `persistence.unowned`.
- **References:** https://attack.mitre.org/techniques/T1543/002/ · https://attack.mitre.org/techniques/T1053/006/ · https://man7.org/linux/man-pages/man1/systemd-delta.1.html · https://www.freedesktop.org/software/systemd/man/latest/systemd.generator.html · https://www.freedesktop.org/software/systemd/man/latest/systemd-analyze.html

### HI-02 Unreviewed per-user systemd units, timers and lingering
- **What it is:** Every user has a systemd instance that loads units from `~/.config/systemd/user`, `~/.local/share/systemd/user`, `/etc/systemd/user` and `/usr/lib/systemd/user`. No privileges are needed to add one. With lingering enabled, user services also start at boot without a login. Malicious npm packages on developer machines now use this location.
- **Normal state:** User units come from packages (PipeWire, portals and similar) under `/usr/lib/systemd/user`. Home-directory units are few and recognisable, such as Bigman's own monitor, syncthing or podman. Linger is off unless the user runs headless services.
- **Real-world examples:**
  - Mini Shai-Hulud (2026, TeamPCP) writes a user-level daemon under `~/.config/systemd` for persistence ([ATT&CK S9043](https://attack.mitre.org/software/S9043/); [T1053.006](https://attack.mitre.org/techniques/T1053/006/)).
  - APT38's multiplatform implant writes under `~/.config/systemd/` (same ATT&CK page).
  - Cross-reference: [supply-chain](./02-supply-chain-packages.md).
- **What a defender inspects (indicators):**
  - `systemctl --user list-unit-files --state=enabled --no-pager`, `systemctl --user list-timers --all --no-pager`
  - `systemd-analyze --user unit-paths`
  - `loginctl show-user "$USER" -p Linger` and `ls /var/lib/systemd/linger/`
  - `ls -la ~/.config/systemd/user ~/.local/share/systemd/user`
  - Suspicious: a unit whose `ExecStart=` runs an interpreter on a script in a dot-directory or cache path, a name imitating a desktop component, or linger turned on with no known reason.
- **Equivalent checks:**
  - ATT&CK: T1543.002, T1053.006
  - Lynis: none (system services only)
  - osquery: `systemd_units` (system scope only); `startup_items`
  - auditd: per-user path watches need explicit rules.
- **Bigman today:** Partial. `~/.config/systemd/user` is baselined with metadata and attribution. `~/.local/share/systemd/user` and linger state are not.
- **Proposed mitigation / detection:** In the monitor, add the missing user unit paths and the linger file list. Ship a manifest of Bigman's own units with their expected content hashes, so the initial inventory says "Bigman-owned" instead of asking the user to judge them. Add a `persistence.user-timer` finding that shows `OnCalendar`/`OnUnitActiveSec` and `ExecStart` in plain language.
- **Test case idea:** Use a tempdir HOME containing `.local/share/systemd/user/echo.timer` and `echo.service` with `ExecStart=/bin/echo hi`. Assert a new entry is found. Add a fixture identical to `bigman-monitor.service` and assert it is labelled self-owned.
- **References:** https://attack.mitre.org/software/S9043/ · https://attack.mitre.org/techniques/T1053/006/ · https://www.freedesktop.org/software/systemd/man/latest/systemd-analyze.html

### HI-03 Cron and at jobs outside the baseline
- **What it is:** cron runs commands on a schedule from system tables and per-user spools, and `at` runs one-off deferred jobs. Both are long-standing, widely abused persistence and re-infection mechanisms, especially by cryptominers.
- **Normal state:** `/etc/crontab` and `/etc/cron.*` hold package-owned jobs such as logrotate, man-db and 0anacron. On a desktop, user crontabs are usually empty. `atq` is empty, and atd is often not installed.
- **Real-world examples:**
  - Kinsing uses crontab to re-download scripts every minute. Exaramel for Linux, GoldMax (`@reboot`) and NKAbuse also use cron ([ATT&CK T1053.003](https://attack.mitre.org/techniques/T1053/003/)).
  - Skidmap (2019) cron-based miner: https://www.trendmicro.com/en_us/research/19/i/skidmap-linux-malware-uses-rootkit-capabilities-to-hide-cryptocurrency-mining-payload.html
  - 2026-10: a tampered Cursor extension persisted on Linux with a user crontab `@reboot` entry that relaunched scripts in `~/.config/.smt` and `~/.config/.smtk`. The user spool is unreadable without root, so only `crontab -l` shows it rootlessly. https://www.microsoft.com/en-us/security/blog/2026/10/30/inside-malicious-claude-code-branded-cursor-extension-persistent-ssh-backdoor/
- **What a defender inspects (indicators):**
  - Own table: `crontab -l`. Other users: `sudo ls -la /var/spool/cron/ /var/spool/cron/crontabs/` (location varies by distro).
  - `ls -la /etc/cron.d /etc/cron.hourly /etc/cron.daily /etc/cron.weekly /etc/cron.monthly`, `cat /etc/crontab /etc/anacrontab`
  - `atq` (own jobs) or `sudo atq`; `systemctl is-active crond cron atd`
  - Package owner of each file (`rpm -qf`/`dpkg -S`); `cron.allow`/`cron.deny` and `at.allow`/`at.deny`.
  - Suspicious: an unowned file in `cron.*`, an entry that fetches from the network, an entry running from a world-writable path, or an every-minute schedule.
- **Equivalent checks:**
  - ATT&CK: T1053.003, T1053.002
  - Lynis: SCHD-7702, SCHD-7704 (crontab/cronjobs), SCHD-7718/7720 (at users), SCHD-7724 (at jobs)
  - CIS Ubuntu 24.04: 2.4.1.2–2.4.1.7 (permissions on crontab and cron dirs), 2.4.1.8 (crontab restricted), 2.4.2.1 (at restricted)
  - SSG: `file_permissions_crontab`, `file_permissions_cron_d`
  - osquery: `crontab` table and the incident-response pack `crontab` query
  - auditd (Neo23x0): `-k cron` watches on all cron and at paths
- **Bigman today:** Partial. `/etc/crontab`, `/etc/anacrontab`, `/etc/cron.d` and `cron.{hourly,daily,weekly,monthly}` are baselined, and so is your own crontab through `crontab -l`. Other users' spools and `at` jobs need the helper.
- **Proposed mitigation / detection:**
  - Rootless: add `/etc/cron.{hourly,daily,weekly,monthly}` and `/etc/anacrontab`. Capture `crontab -l` and `atq` output for the current user as virtual baseline entries (hash of the output).
  - Optional privileged collector (see "Privileged read-only collector" under Open questions): list the spool directories as metadata plus hashes.
- **Test case idea:** In a tempdir prefix, add `etc/cron.daily/zz-benign` containing `#!/bin/sh\necho ok` and assert a finding. Mock `crontab -l` output through an injectable runner.
- **References:** https://attack.mitre.org/techniques/T1053/003/ · https://attack.mitre.org/techniques/T1053/002/ · https://github.com/Neo23x0/auditd

### HI-04 XDG autostart and desktop session hooks
- **What it is:** Desktop environments launch every `.desktop` file in `~/.config/autostart` and in the system `$XDG_CONFIG_DIRS/autostart` (`/etc/xdg/autostart`) at login. X session scripts and KDE `plasma-workspace/env` scripts run at session start too. This is the desktop counterpart of the Windows Run key, and it needs no privileges.
- **Normal state:** `/etc/xdg/autostart` is all package-owned (keyring, IME, update notifier and so on). `~/.config/autostart` holds a few apps the user knowingly enabled, or override copies with `Hidden=true`.
- **Real-world examples:**
  - InvisibleFerret, used by the North Korea-linked Contagious Interview campaign against developers, creates `.desktop` autostart entries ([ATT&CK T1547.013](https://attack.mitre.org/techniques/T1547/013/); [S1245](https://attack.mitre.org/software/S1245/)).
  - Fysbis, RotaJakiro, CrossRAT, NETWIRE and Pupy use the same location (same page).
- **What a defender inspects (indicators):**
  - `ls -la ~/.config/autostart /etc/xdg/autostart`
  - `grep -H -E '^(Exec|TryExec|Hidden|X-GNOME-Autostart-enabled)=' ~/.config/autostart/*.desktop`
  - `rpm -qf /etc/xdg/autostart/*.desktop` (or `dpkg -S`)
  - `ls -la ~/.xprofile ~/.xinitrc ~/.xsessionrc /etc/X11/xinit/xinitrc.d ~/.config/plasma-workspace/env 2>/dev/null`
  - Suspicious: an `Exec=` that runs a shell or interpreter with an inline command or a script in a hidden directory, a `.desktop` named after a system component but absent from `/usr/share/applications`, or an unowned file in `/etc/xdg/autostart`.
- **Equivalent checks:**
  - ATT&CK: T1547.013
  - Lynis: none
  - CIS: none
  - osquery: `startup_items` (coverage of XDG on Linux varies by version; verify)
- **Bigman today:** Partial. `~/.config/autostart` and `/etc/xdg/autostart` are baselined. Other `$XDG_CONFIG_DIRS` and X/KDE session scripts are not.
- **Proposed mitigation / detection:** In the monitor, add these paths. Parse each `.desktop` file and put its `Exec=` and `Hidden=` values in the finding evidence, so the user sees what will run rather than only a hash. Add a rule `autostart.exec-interpreter` (Review) for `Exec=` lines starting `sh -c`, `bash -c`, `python -c` or `node -e`, reusing the `shell.dynamic` patterns from the analyzer.
- **Test case idea:** Write `~/.config/autostart/test.desktop` in a temp HOME with `Exec=sh -c "echo benign"`. Assert both `persistence.change` and `autostart.exec-interpreter`.
- **References:** https://attack.mitre.org/techniques/T1547/013/ · https://specifications.freedesktop.org/autostart-spec/latest/

### HI-05 Shell startup and environment files
- **What it is:** Interactive and login shells source many files: per-user bash, zsh and fish rc files plus system `/etc/profile`, `/etc/profile.d/*` and `/etc/bashrc`. `environment.d`, `/etc/environment` and `pam_env` set variables for whole sessions. A line added there runs every time a terminal opens. It can also change `PATH` so that a copy in a user-writable directory shadows `sudo`, `ssh` or `git`.
- **Normal state:** System files are package-owned and verify clean. User rc files contain blocks added by known tool installers (nvm, conda, rustup, sdkman, pyenv, Flutter). These are noisy but recognisable. `/etc/profile.d` is entirely package-owned.
- **Real-world examples:**
  - RotaJakiro adds a command to the user's shell rc when running unprivileged.
  - Linux Rabbit uses `.bashrc`, and Green Lambert modifies profile and rc files ([ATT&CK T1546.004](https://attack.mitre.org/techniques/T1546/004/)).
  - Pierre Berba's threat-hunting write-up on initialization scripts and shell configuration (2022): https://pberba.github.io/security/2022/02/06/linux-threat-hunting-for-persistence-initialization-scripts-and-shell-configuration/
- **What a defender inspects (indicators):**
  - `ls -la ~/.bashrc ~/.bash_profile ~/.bash_login ~/.bash_logout ~/.profile ~/.zshrc ~/.zshenv ~/.zprofile ~/.zlogin ~/.config/fish/ ~/.config/environment.d/ ~/.pam_environment 2>/dev/null`
  - `rpm -Vf /etc/profile /etc/bashrc` (Debian: `dpkg --verify bash base-files`); `rpm -qf /etc/profile.d/*` to find unowned files.
  - `cat /etc/environment`
  - `printf '%s\n' "$PATH" | tr : '\n'`, then check each directory's owner and mode with `stat -c '%A %U %n'`.
  - Suspicious in general terms: lines setting `LD_PRELOAD`, prepending a writable or hidden directory to `PATH`, aliases or functions that redefine `sudo`/`su`/`ssh`/`git`, `PROMPT_COMMAND` or `DEBUG` traps calling out to a script, or download-and-execute one-liners.
- **Equivalent checks:**
  - ATT&CK: T1546.004, T1574.007
  - Lynis: SHLL-6230 (umask in shell configs), HOME-9310 (suspicious shell history files)
  - CIS Ubuntu 24.04: 7.2.10 (local interactive user dot files access), 5.4.2.5 (root path integrity)
  - SSG: `accounts_root_path_dirs_no_write`
  - osquery: none (use `file` and `hash`)
  - auditd (Neo23x0): `-k shell_profiles` on `/etc/profile*`, `/etc/bashrc` and similar
- **Bigman today:** Partial. User and system shell profiles (`~/.bashrc`, `~/.zshrc`, `~/.profile` and friends, `/etc/profile`, `/etc/profile.d`, `/etc/bashrc`, `/etc/environment`, `environment.d`) are baselined with attribution. fish and per-shell extras are not.
- **Proposed mitigation / detection:**
  - In the CLI and monitor, cover the full list above.
  - Keep a private copy of each small rc file so changes can be shown as a line diff.
  - Add deterministic line rules, each with the matched line as evidence: `shell-rc.preload`, `shell-rc.path-prepend-writable`, `shell-rc.alias-privileged-cmd` and `shell-rc.fetch-exec`.
  - Ship an optional allowlist of exact installer blocks (nvm, conda, rustup) to cut noise. Match on exact text, not regex.
- **Test case idea:** Use a temp HOME `.zshenv` containing `export PATH="$HOME/.cache/x:$PATH"` (benign, since the directory is empty). Assert `shell-rc.path-prepend-writable`. Add a fixture containing the standard rustup line and assert it is labelled known-installer.
- **References:** https://attack.mitre.org/techniques/T1546/004/ · https://attack.mitre.org/techniques/T1574/007/ · https://pberba.github.io/security/2022/02/06/linux-threat-hunting-for-persistence-initialization-scripts-and-shell-configuration/

### HI-06 rc.local, SysV init scripts, MOTD scripts and network dispatcher hooks
- **What it is:** Several legacy and less-watched hooks still run as root:
  - `/etc/rc.local`, run by systemd's `rc-local.service` only if the file is executable;
  - `/etc/init.d` scripts, wrapped by `systemd-sysv-generator`;
  - Debian/Ubuntu `/etc/update-motd.d/*`, run by `pam_motd` at each login;
  - NetworkManager dispatcher scripts, run on interface events.
- **Normal state:** `rc.local` is absent or a non-executable stub. `/etc/init.d` holds few, package-owned files. `update-motd.d` files are package-owned (Ubuntu). `dispatcher.d` holds only package-owned scripts.
- **Real-world examples:**
  - HiddenWasp adds itself to `/etc/rc.local`.
  - Velvet Ant modified `/etc/rc.local` on F5 BIG-IP appliances (2024).
  - Cyclops Blink and Green Lambert use RC scripts ([ATT&CK T1037.004](https://attack.mitre.org/techniques/T1037/004/)).
  - Velvet Ant report (Sygnia, 2024): https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/
- **What a defender inspects (indicators):**
  - `ls -la /etc/rc.local /etc/rc.d/rc.local /etc/init.d/ 2>/dev/null`, `systemctl status rc-local.service`
  - `ls -la /etc/update-motd.d/` and `dpkg -S /etc/update-motd.d/*`
  - `grep -rn pam_motd /etc/pam.d/`
  - `ls -la /etc/NetworkManager/dispatcher.d/ /etc/network/if-up.d/ 2>/dev/null`
  - Suspicious: an executable `rc.local` with content, or an unowned script in any of these directories.
- **Equivalent checks:**
  - ATT&CK: T1037.004. MOTD has no dedicated technique; the nearest parent is T1037.
  - Lynis: BOOT-5184 / BOOT-5263 (permissions of boot files and scripts)
  - CIS Ubuntu 24.04: 1.6.4 (access to `/etc/motd`), which covers file permissions only, not the scripts.
  - auditd (Neo23x0): `-w /etc/rc.local -k init`
- **Bigman today:** Partial. `/etc/rc.local`, `/etc/rc.d/rc.local`, `/etc/init.d` and `/etc/update-motd.d` are baselined. NetworkManager dispatcher hooks are not.
- **Proposed mitigation / detection:** In the monitor, add the paths. Record the executable bit for `rc.local`, since whether it runs depends on that bit. Raise `persistence.unowned` on any of these files that has no package owner.
- **Test case idea:** Create `etc/rc.local` in a tempdir prefix (mode 0755, contents `#!/bin/sh\nexit 0`). Assert a finding whose evidence says it is executable.
- **References:** https://attack.mitre.org/techniques/T1037/004/ · https://attack.mitre.org/techniques/T1037/ · https://man7.org/linux/man-pages/man8/pam_motd.8.html

### HI-07 udev rules that run programs
- **What it is:** udev rules match device events and can run programs (`RUN+=`, `PROGRAM=`) as root whenever a matching device appears. Device events happen constantly at boot, so a rule can serve as stealthy root persistence. Rules in `/etc/udev/rules.d` override vendor rules of the same name.
- **Normal state:** `/usr/lib/udev/rules.d` is fully package-owned. `/etc/udev/rules.d` is empty or contains a few admin or hardware rules (adb, YubiKey, dev boards) that set permissions rather than run programs.
- **Real-world examples:**
  - sedexp (Stroz Friedberg/LevelBlue, published 2024-08-19, active since at least 2022) persists through udev rules: https://levelblue.com/blogs/strozfriedberg/unveiling-sedexp
  - The REPTILE rootkit uses udev ([ATT&CK T1546.017](https://attack.mitre.org/techniques/T1546/017/)).
- **What a defender inspects (indicators):**
  - `ls -la /etc/udev/rules.d /run/udev/rules.d /usr/lib/udev/rules.d`
  - `rpm -qf /etc/udev/rules.d/*` (or `dpkg -S`)
  - `grep -nE 'RUN\+?=|PROGRAM=|IMPORT\{program\}' /etc/udev/rules.d/*.rules`
  - `udevadm verify` (systemd 255 and later) for syntax.
  - Suspicious: an unowned rule that runs a program, especially one outside `/usr/lib/udev`, or a rule matching very common devices (`/dev/random`, any block device).
- **Equivalent checks:**
  - ATT&CK: T1546.017
  - Lynis: none (USB-1000/2000/3000 cover USB storage and USBGuard, not rules)
  - CIS: none
  - osquery: none (use `file`)
  - auditd (Neo23x0): `-w /etc/udev/rules.d/ -k udev`
- **Bigman today:** Partial. `/etc/udev/rules.d` is baselined with attribution. `RUN+=` programs are not inspected and `/run/udev/rules.d` is not covered.
- **Proposed mitigation / detection:** In the monitor, add the three rule directories. Rule `udev.run-unowned` (Review) fires when a rule file is not package-owned and contains `RUN`/`PROGRAM`, with the matching line as evidence.
- **Test case idea:** Fixture `etc/udev/rules.d/99-benign.rules` containing `ACTION=="add", SUBSYSTEM=="test", RUN+="/bin/true"`. Assert the finding.
- **References:** https://attack.mitre.org/techniques/T1546/017/ · https://man7.org/linux/man-pages/man8/udevadm.8.html · https://levelblue.com/blogs/strozfriedberg/unveiling-sedexp

### HI-08 PAM stack and PAM module files
- **What it is:** Pluggable Authentication Modules decide how `login`, `sudo`, `sshd`, `gdm` and `polkit` authenticate users. A changed `/etc/pam.d` stack, or a replaced or added `pam_*.so`, can silently accept any password or record credentials. PAM changes are high-value, rare on a desktop, and easy to verify against the package DB.
- **Normal state:** `/etc/pam.d` is managed by `authselect` (Fedora/RHEL) or `pam-auth-update` (Debian/Ubuntu), and `authselect check` passes. Every module in the PAM module directory is package-owned, and `rpm -V pam` reports nothing.
- **Real-world examples:**
  - Plague (Nextron Systems, 2025-08-01) is a malicious PAM module that masqueraded as a system library and had zero VirusTotal detections when reported: https://www.nextron-systems.com/2025/08/01/plague-a-newly-discovered-pam-based-backdoor-for-linux/
  - Skidmap replaced `pam_unix.so` (2019).
  - Ebury tampers with PAM and sshd ([ATT&CK T1556.003](https://attack.mitre.org/techniques/T1556/003/)).
- **What a defender inspects (indicators):**
  - `authselect current` and `authselect check` (Fedora/RHEL); Debian: `ls /usr/share/pam-configs/`
  - `rpm -V pam` (Debian: `dpkg --verify libpam-modules libpam-runtime` or `debsums -s libpam-modules`)
  - `for f in /usr/lib64/security/*.so /usr/lib/x86_64-linux-gnu/security/*.so; do [ -e "$f" ] && { rpm -qf "$f" 2>/dev/null || dpkg -S "$f"; }; done | grep -i 'not owned\|no path found'`
  - `grep -rnE 'pam_exec|pam_permit|nullok|^auth.*sufficient' /etc/pam.d/`
  - Suspicious: an unowned `.so` in the module directory, a module referenced by absolute path from an unusual directory, `pam_permit.so` in an `auth` stack of a login service, a new `pam_exec.so` line, or `authselect check` reporting manual edits.
- **Equivalent checks:**
  - ATT&CK: T1556.003
  - Lynis: AUTH-9262 (PAM password strength), AUTH-9264/9266/9268 (pam.conf and pam.d presence)
  - CIS Ubuntu 24.04: 5.3.1.1–5.3.1.3 (PAM packages current), 5.3.2.1–5.3.2.4 (expected modules enabled), 5.3.3.4.1 (`pam_unix` without `nullok`)
  - osquery: none dedicated
  - auditd (Neo23x0): `-w /etc/pam.d/ -k pam`, `-w /etc/security/ -k pam`
  - Wazuh/OSSEC: rootcheck and FIM defaults include PAM paths.
- **Bigman today:** Partial. `/etc/pam.d` is baselined; a file a package owns that no longer matches it is `persistence.sensitive_modified` (High). `/etc/security` and PAM module directories are not covered.
- **Proposed mitigation / detection:**
  - Rootless CLI check `bigman audit host`: hash `/etc/pam.d`, `/etc/security` and the module dirs into the baseline.
  - For every module run `rpm -qf` and `rpm -Vf`. Rule `pam.module-unowned` is High, because unowned code inside the auth path is never normal. `pam.config-modified` is Review. Also run `authselect check` when it is available.
- **Test case idea:** Point the PAM module dir at a tempdir and copy `/bin/true` there as `pam_test.so`. Use a fake package resolver returning "not owned" and assert `pam.module-unowned`. Add a fixture `etc/pam.d/test` containing `auth sufficient pam_permit.so` and assert a Review finding.
- **References:** https://attack.mitre.org/techniques/T1556/003/ · https://www.nextron-systems.com/2025/08/01/plague-a-newly-discovered-pam-based-backdoor-for-linux/ · https://www.welivesecurity.com/2014/03/18/operation-windigo-the-vivisection-of-a-large-linux-server-side-credential-stealing-malware-campaign/

### HI-09 Dynamic linker configuration (preload, ld.so.conf.d, LD_* environment)
- **What it is:** The dynamic loader reads `/etc/ld.so.preload` and injects listed libraries into **every** dynamically linked process. It also searches directories from `/etc/ld.so.conf` and `ld.so.conf.d`, and honours `LD_PRELOAD`/`LD_LIBRARY_PATH` from the environment. This is the classic Linux userland-rootkit mechanism. A preloaded library can also change what ordinary tools (including `ls`, `cat` and Bigman, which use libc) report about the filesystem, so cross-view checks matter.
- **Normal state:** `/etc/ld.so.preload` does not exist, or exists but is empty. Every file in `ld.so.conf.d` is package-owned and points under `/usr`. No session-wide `LD_PRELOAD` is set.
- **Real-world examples:**
  - Symbiote (Intezer/BlackBerry, 2022) is a preloaded shared object that injects into all processes: https://www.intezer.com/blog/research/new-linux-threat-symbiote/
  - Hildegard and Rocke modified `/etc/ld.so.preload`, and Aquatic Panda used it for Winnti persistence ([ATT&CK T1574.006](https://attack.mitre.org/techniques/T1574/006/)).
  - Wiz explainer (2023): https://www.wiz.io/blog/linux-rootkits-explained-part-1-dynamic-linker-hijacking
- **What a defender inspects (indicators):**
  - `ls -la /etc/ld.so.preload`
  - `cat /etc/ld.so.conf /etc/ld.so.conf.d/*.conf` and `rpm -qf /etc/ld.so.conf.d/*`
  - `grep -l LD_PRELOAD /proc/[0-9]*/environ 2>/dev/null` (own processes rootless; all processes need root)
  - `grep -rn 'LD_PRELOAD\|LD_LIBRARY_PATH' /etc/environment /etc/profile.d ~/.config/environment.d 2>/dev/null`
  - `systemctl show -p Environment SERVICE`
  - Suspicious: any non-empty `ld.so.preload`, an `ld.so.conf.d` entry pointing into `/tmp`, `/dev/shm`, `/var/tmp`, `/opt/<unknown>` or a home directory, or `LD_PRELOAD` set for long-running processes.
  - Cross-view: when a statically linked reader and a libc-based reader disagree about whether `/etc/ld.so.preload` exists or what it contains, treat that as a strong indicator.
- **Equivalent checks:**
  - ATT&CK: T1574.006
  - Lynis: none dedicated
  - CIS: none
  - osquery: `process_envs`, `file`
  - auditd (Neo23x0): `-w /etc/ld.so.preload -k systemwide_preloads`, `-w /etc/ld.so.conf(.d) -k libpath`
  - Tracee detector: `ld_preload`
  - rkhunter and chkrootkit check for known preload rootkit artifacts.
- **Bigman today:** Partial. `/etc/ld.so.preload` is hashed, and `bigman running` reports it whenever it names a library, independent of the baseline (`linker.preload-present`). The same check compares what the C library returns for the preload file, the process list, `/proc/modules` and `/proc/net` with direct system calls (`hidden.libc-view`), without a separate static helper, and lists `LD_PRELOAD`/`LD_AUDIT` in your own programs' environments (`process.preload`). `ld.so.conf.d` and environment files are not checked.
- **Proposed mitigation / detection:**
  - Add `/etc/ld.so.conf` and `ld.so.conf.d`.
  - Rule `linker.preload-present` (High) when `/etc/ld.so.preload` is non-empty, independent of baseline state, because the normal state is "absent".
  - Rule `linker.libpath-writable` when a configured library dir is writable by a non-root user.
  - Build a tiny **statically linked (musl) helper** that only `stat`s and reads a fixed list of paths (`/etc/ld.so.preload`, `/proc/modules`) by direct syscalls. Report disagreement with the libc view as `integrity.cross-view-mismatch`. This is deterministic, rootless and cheap.
- **Test case idea:** Use the tempdir prefix with `etc/ld.so.preload` containing `/nonexistent/libbenign.so` (the file is never created or loaded). Assert `linker.preload-present`. For the cross-view helper, unit-test the comparator with two synthetic readings.
- **References:** https://attack.mitre.org/techniques/T1574/006/ · https://man7.org/linux/man-pages/man8/ld.so.8.html · https://www.intezer.com/blog/research/new-linux-threat-symbiote/ · https://www.wiz.io/blog/linux-rootkits-explained-part-1-dynamic-linker-hijacking

### HI-10 SSH authorized keys, sshd configuration and per-user SSH hooks
- **What it is:** Any public key in a user's `authorized_keys` grants login as that user wherever sshd is reachable. Key options (`command=`, `from=`, `environment=`) and sshd settings (`AuthorizedKeysFile`, `AuthorizedKeysCommand`, `PermitRootLogin`, `PermitUserEnvironment`) change who can get in. `~/.ssh/rc` and `/etc/ssh/sshrc` run at every SSH login.
- **Normal state:** On a desktop, sshd is often not installed or is disabled. `authorized_keys` is absent or holds only keys whose fingerprints the user recognises. sshd effective config has `PermitRootLogin no` or `prohibit-password`, and no `AuthorizedKeysCommand` unless the host is centrally managed.
- **Real-world examples:**
  - Salt Typhoon added authorized keys under root and other users on compromised infrastructure.
  - TeamTNT, Skidmap and Earth Lusca also plant keys ([ATT&CK T1098.004](https://attack.mitre.org/techniques/T1098/004/)).
  - 2026-10: a tampered Cursor extension appended a `svc-telemetry` key to the user's `authorized_keys` and tried to enable sshd. https://www.microsoft.com/en-us/security/blog/2026/10/30/inside-malicious-claude-code-branded-cursor-extension-persistent-ssh-backdoor/
  - Cross-reference: [network exposure](./07-network-exposure-and-remote-access.md) for sshd exposure and CVE-2024-6387.
- **What a defender inspects (indicators):**
  - `ssh-keygen -lf ~/.ssh/authorized_keys` gives one fingerprint per key. Compare against known keys.
  - `ls -la ~/.ssh/` (look for `rc`, `authorized_keys2`, `environment`, permissions)
  - `sudo sshd -T | grep -Ei 'permitrootlogin|passwordauthentication|authorizedkeys|permituserenvironment|permitemptypasswords'`
  - `systemctl is-enabled sshd ssh`; `ss -tlnp 'sport = :22'`
  - `rpm -V openssh-server` (Debian: `dpkg --verify openssh-server`)
  - Suspicious: a key with an unfamiliar comment or fingerprint, keys in root's file on a single-user desktop, `AuthorizedKeysFile` pointing at an odd path, `PermitUserEnvironment yes`, or a new `~/.ssh/rc`.
- **Equivalent checks:**
  - ATT&CK: T1098.004, T1021.004
  - Lynis: SSH-7402, SSH-7404, SSH-7406, SSH-7408, SSH-7440
  - CIS Ubuntu 24.04: 5.1.1 (`sshd_config` permissions), 5.1.19 (`PermitEmptyPasswords`), 5.1.20 (`PermitRootLogin`), 5.1.21 (`PermitUserEnvironment`)
  - SSG: `sshd_disable_root_login`, `sshd_disable_empty_passwords`, `file_permissions_sshd_config`
  - osquery: `authorized_keys`, `user_ssh_keys`, `sshd_config`, `ssh_configs`, and the `linux-ssh-investigation` pack
  - Falco incubating rule: "Adding ssh keys to authorized_keys"
- **Bigman today:** Partial. `authorized_keys`, `~/.ssh/config`, `~/.ssh/rc`, `sshd_config`, `sshd_config.d` and `ssh_config.d` are baselined; a modified package-owned SSH file is High. There is no key-fingerprint diff yet.
- **Proposed mitigation / detection:**
  - Parse `authorized_keys` line by line into (type, fingerprint, comment, options) and baseline the fingerprints. Rule `ssh.key-added` shows the new fingerprint, comment and any `command=`/`from=` options. Never show or store the full key blob in history.
  - Add `~/.ssh/rc`, `authorized_keys2` and `environment`.
  - In the optional privileged collector, add `sshd -T` and root's `authorized_keys` metadata.
- **Test case idea:** Generate a throwaway ed25519 key in the test, append its public key to a temp `authorized_keys`, and assert `ssh.key-added` carries the matching fingerprint.
- **References:** https://attack.mitre.org/techniques/T1098/004/ · https://attack.mitre.org/techniques/T1021/004/ · https://man7.org/linux/man-pages/man5/sshd_config.5.html · https://attack.mitre.org/groups/G1045/

### HI-11 Module autoload, modprobe directives, package-manager and kernel-install hooks
- **What it is:** Several root-executed hook directories are rarely reviewed:
  - `modules-load.d` and `/etc/modules`, which load kernel modules at boot;
  - `modprobe.d`, whose `install` and `softdep` directives can run commands when a module loads;
  - apt `DPkg::Pre-Invoke`/`Post-Invoke` and `APT::Update::Pre-Invoke` snippets, dnf plugins and pacman hooks, which run on every package operation;
  - `/etc/kernel/postinst.d` and `/etc/kernel/install.d`, which run on kernel installs.
- **Normal state:** All of these are package-owned. `modules-load.d` lists known drivers (for example `v4l2loopback` or VirtualBox if installed). There are no `install` directives other than distro-shipped blacklists (`install X /bin/false`).
- **Real-world examples:** ATT&CK documents the general technique ([T1546.016 Installer Packages](https://attack.mitre.org/techniques/T1546/016/), [T1547.006 Kernel Modules and Extensions](https://attack.mitre.org/techniques/T1547/006/)). No dated Linux desktop incident was verified for apt/dnf hook persistence specifically. Cross-reference: [distro packaging](./03-distro-packaging-and-install-channels.md).
- **What a defender inspects (indicators):**
  - `ls -la /etc/modules-load.d /usr/lib/modules-load.d /etc/modprobe.d; cat /etc/modules 2>/dev/null`
  - `grep -rnE '^\s*(install|softdep)\s' /etc/modprobe.d /usr/lib/modprobe.d`
  - `grep -rnE 'Pre-Invoke|Post-Invoke' /etc/apt/apt.conf.d/ 2>/dev/null`; `ls /etc/dnf/plugins /etc/pacman.d/hooks /etc/kernel/postinst.d /etc/kernel/install.d 2>/dev/null`
  - Package ownership of each file.
- **Equivalent checks:**
  - ATT&CK: T1547.006, T1546.016
  - Lynis: KRNL-5726 (loaded modules)
  - CIS Ubuntu 24.04: 1.1.1.x and 3.2.x ("module not available", which uses modprobe.d)
  - osquery: none dedicated
  - auditd (Neo23x0): `-k modprobe`, `-k software_mgmt`
- **Bigman today:** Partial. `/etc/modules-load.d`, `/etc/modprobe.d` and `/etc/modules` are baselined with attribution. Package-manager and kernel-install hooks are not.
- **Proposed mitigation / detection:** In the monitor, add these directories. Rule `hook.unowned` (Review) for any unowned file. Rule `modprobe.install-command` lists `install` directives that run something other than `/bin/false` or `/bin/true`.
- **Test case idea:** Fixture `etc/modprobe.d/zz-test.conf` containing `install dummy_test /bin/echo benign`. Assert `modprobe.install-command`.
- **References:** https://attack.mitre.org/techniques/T1547/006/ · https://attack.mitre.org/techniques/T1546/016/ · https://github.com/Neo23x0/auditd

### HI-12 Extra UID 0, duplicate IDs, new or shell-enabled system accounts
- **What it is:** Any account with UID 0 is root, whatever its name. Attackers who obtain root often add a local account, or give a system account a login shell, so they can return later. On a single-user desktop the account list should almost never change, which makes a diff very effective.
- **Normal state:** Only `root` has UID 0. Accounts below `UID_MIN` (from `/etc/login.defs`, usually 1000) have `nologin` or `false` as their shell, except `root` and `sync`/`shutdown`/`halt`, depending on distro. There are no duplicate UIDs, GIDs or names, and `pwck -r` is clean.
- **Real-world examples:**
  - Dozens of groups and tools create local accounts ([ATT&CK T1136.001](https://attack.mitre.org/techniques/T1136/001/)).
  - The same page also lists recent campaigns in which an AI coding agent was tasked with creating a local backdoor account.
  - Red Hat libuser race CVE-2015-3246 (an `/etc/passwd` corruption LPE) was added to CISA KEV on 2026-08-26.
- **What a defender inspects (indicators):**
  - `awk -F: '$3==0{print $1}' /etc/passwd` and `getent passwd | awk -F: '$3==0'` (the second includes NSS/SSSD sources)
  - `pwck -r` and `grpck -r` (read-only mode)
  - `awk -F: -v min="$(awk '/^UID_MIN/{print $2}' /etc/login.defs)" '$3<min && $7!~/(nologin|false)$/{print $1,$7}' /etc/passwd`
  - `lastlog` (or `lastlog2` on newer util-linux), `last -n 20`
  - Suspicious: a second UID 0 account, a new account since baseline, or a system account with `/bin/bash`.
- **Equivalent checks:**
  - ATT&CK: T1136.001, T1078.003
  - Lynis: AUTH-9204 (UID 0 users), AUTH-9208 (non-unique accounts), AUTH-9228 (`pwck`), AUTH-9218 (harmful login shells), AUTH-9234 (user accounts)
  - CIS Ubuntu 24.04: 5.4.2.1 (root only UID 0), 5.4.2.2/5.4.2.3 (GID 0), 7.2.5–7.2.8 (no duplicate UIDs/GIDs/names), 7.2.3 (groups exist)
  - SSG: `accounts_no_uid_except_zero`, `accounts_root_gid_zero`
  - osquery: `users` (`SELECT * FROM users WHERE uid = 0`)
  - auditd (Neo23x0): `-k etcpasswd`, `-k etcgroup`
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** Add a rootless CLI and monitor check that parses `/etc/passwd`/`/etc/group` into a structured baseline (names, UIDs, GIDs, shells, home directories, but never password fields). Rules:
  - `host.uid0-extra` (High)
  - `host.account-new` (Review)
  - `host.system-account-shell` (Review)
  - `host.duplicate-id` (Review)
- **Test case idea:** Parser unit test with a synthetic passwd string containing `toor:x:0:0::/root:/bin/bash`. Assert `host.uid0-extra`. Diff two synthetic snapshots to assert `host.account-new`.
- **References:** https://attack.mitre.org/techniques/T1136/001/ · https://attack.mitre.org/techniques/T1078/003/ · https://github.com/CISOfy/lynis/blob/master/db/tests.db

### HI-13 Empty or unlocked passwords and `nullok` in PAM
- **What it is:** An account with an empty password field, or a PAM stack whose `pam_unix` has `nullok`, can be logged into with no password on any service that uses that stack. This is a classic CIS item. On desktops it shows up after a manual `passwd -d` or kiosk-style setups.
- **Normal state:** Every account in `/etc/shadow` has a hash, `!` or `*`. System accounts are locked. `nullok` is absent, or present only where the distro ships it by default and no account has an empty password. Some distros ship `nullok` in default stacks, so the check that matters is the combination.
- **Real-world examples:** Configuration weakness rather than a named incident. Covered by every hardening baseline: CIS 7.2.2 and 5.3.3.4.1, Lynis AUTH-9283.
- **What a defender inspects (indicators):**
  - `sudo awk -F: '($2==""){print $1}' /etc/shadow` (prints names only, never hashes)
  - `sudo passwd -S USER` (output differs by distro)
  - `grep -rn nullok /etc/pam.d/` (rootless)
  - `sudo pwck -r`
- **Equivalent checks:**
  - ATT&CK: T1078.003
  - Lynis: AUTH-9283 (accounts without password), AUTH-9282/9286/9288 (aging and expiry)
  - CIS Ubuntu 24.04: 7.2.1 (shadowed passwords), 7.2.2 (shadow fields not empty), 5.3.3.4.1 (no `nullok`)
  - SSG: `no_empty_passwords`, `no_empty_passwords_etc_shadow`, `accounts_password_all_shadowed`
  - osquery: `shadow` (root)
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - Rootless rule `pam.nullok` (Info, with an explanation).
  - The optional privileged collector returns only per-account status flags (empty / locked / hashed), never the hash. Rule `host.empty-password` is High.
- **Test case idea:** A collector parser fed a synthetic shadow string `test::19000:0:99999:7:::` returns `empty` for `test`. A fixture `pam.d/test` containing `auth sufficient pam_unix.so nullok` triggers `pam.nullok`.
- **References:** https://attack.mitre.org/techniques/T1078/003/ · https://github.com/ComplianceAsCode/content

### HI-14 sudoers and sudoers.d grants
- **What it is:** sudo policy decides who can become root and whether a password is needed. A dropped file in `/etc/sudoers.d` granting `NOPASSWD: ALL` (or `!authenticate`, or `env_keep` of loader variables) turns any later compromise of that user into instant root. sudo itself has had several LPE bugs (see HI-23).
- **Normal state:** `%wheel` or `%sudo` `ALL=(ALL) ALL`, with a password required. `sudoers.d` is empty or holds only package-owned files, with mode 0440 root:root. `visudo -c` passes. `Defaults secure_path` contains only root-owned directories.
- **Real-world examples:**
  - Shai-Hulud (npm worm, first reported September 2025) attempted root through sudo and `/etc/sudoers.d` ([ATT&CK S9008](https://attack.mitre.org/software/S9008/); Unit 42: https://unit42.paloaltonetworks.com/npm-supply-chain-attack/).
  - Salt Typhoon edited `/etc/sudoers` or added `sudoers.d` files, and Dok added a NOPASSWD rule ([ATT&CK T1548.003](https://attack.mitre.org/techniques/T1548/003/)).
- **What a defender inspects (indicators):**
  - `sudo visudo -c` (syntax, ownership and mode of all included files)
  - `sudo -l` (own grants); `sudo -l -U USER`
  - `sudo ls -la /etc/sudoers.d/`; `rpm -V sudo` (Debian: `dpkg --verify sudo`)
  - Suspicious: `NOPASSWD` for broad command sets, `!authenticate`, `timestamp_timeout=-1`, `env_keep` including `LD_*` or `PYTHONPATH`, grants for editors, interpreters or pagers that can spawn shells, or unowned `sudoers.d` files.
  - Defenders can use GTFOBins (https://gtfobins.github.io/) as a reference list of binaries whose sudo grants amount to full root. Cite it; do not reproduce it.
- **Equivalent checks:**
  - ATT&CK: T1548.003
  - Lynis: AUTH-9250, AUTH-9252
  - CIS Ubuntu 24.04: 5.2.2 (use_pty), 5.2.3 (sudo log), 5.2.4 (password required), 5.2.5 (re-auth not disabled), 5.2.6 (timeout), 6.2.3.1 (audit sudoers changes)
  - SSG: `sudo_remove_nopasswd`, `sudo_remove_no_authenticate`, `sudo_require_reauthentication`
  - osquery: `sudoers`
  - auditd (Neo23x0): `-w /etc/sudoers(.d) -k actions`
  - Tracee detector: `sudoers_modification`
- **Bigman today:** Gap. The `shell.privilege` rule flags sudo use inside installer scripts but does not audit the policy itself. On Fedora, `/etc/sudoers` and `sudoers.d` are unreadable rootless.
- **Proposed mitigation / detection:** The optional privileged collector runs `visudo -c` and returns the policy files' hashes plus a parsed summary of user specs and Defaults. It does not include comments, aliases or hostnames beyond what is needed. Rules:
  - `sudo.nopasswd-all` (High)
  - `sudo.no-authenticate` (High)
  - `sudo.env-keep-loader` (High)
  - `sudo.dropin-unowned` (Review)
- **Test case idea:** Run a parser unit test on a synthetic sudoers text containing `alice ALL=(ALL) NOPASSWD: ALL` and assert `sudo.nopasswd-all`. In CI only, validate fixtures with `visudo -c -f FILE`, which reads but does not install.
- **References:** https://attack.mitre.org/techniques/T1548/003/ · https://www.sudo.ws/docs/man/sudoers.man/ · https://attack.mitre.org/software/S9008/ · https://gtfobins.github.io/

### HI-15 polkit rules and implicit authorizations
- **What it is:** polkit authorizes privileged desktop actions such as mounting disks, managing units, installing packages and changing network settings. JavaScript rules in `/etc/polkit-1/rules.d` and `/usr/share/polkit-1/rules.d` can return `YES` for whole groups. Action defaults (`allow_any`/`allow_inactive`/`allow_active`) decide what a local or remote session may do without authentication. Bigman's own setup actions go through polkit too.
- **Normal state:** `/usr/share/polkit-1/rules.d` is package-owned. `/etc/polkit-1/rules.d` is empty or holds admin rules the user can explain. `pkaction` defaults are unchanged from the package versions.
- **Real-world examples:**
  - PwnKit CVE-2021-4034 (pkexec) and CVE-2021-3560 (polkit D-Bus auth bypass): GitHub Security Lab, 2021 — https://github.blog/security/vulnerability-research/privilege-escalation-polkit-root-on-linux-with-bug/
  - CVE-2025-6019 (libblockdev/udisks) relied on polkit `allow_active` (Qualys, 2025-06-17): https://www.qualys.com/2025/06/17/suse15-pam-udisks-lpe.txt
- **What a defender inspects (indicators):**
  - `pkaction --verbose`, rootless. Diff the implicit authorizations against the previous run.
  - `ls -la /usr/share/polkit-1/rules.d/` and `rpm -qf /usr/share/polkit-1/rules.d/*`
  - `sudo ls -la /etc/polkit-1/rules.d/`
  - `rpm -V polkit`
  - Suspicious: an unowned rule file, a rule returning `polkit.Result.YES` without a narrow action-ID and group check, or `allow_any=yes` on a sensitive action.
- **Equivalent checks:**
  - ATT&CK: T1548 (no Linux-specific sub-technique)
  - Lynis: none
  - CIS: none
  - SSG: `audit_rules_privileged_commands_polkit_helper` (audit only)
  - osquery: none
  - auditd (Neo23x0): `-w /etc/polkit-1/ -k pam`
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - Rootless: baseline `/usr/share/polkit-1/rules.d` and `/usr/share/polkit-1/actions` plus the `pkaction --verbose` output.
  - Privileged collector: hash `/etc/polkit-1/rules.d`.
  - Rule `polkit.rule-unowned` (Review). Rule `polkit.implicit-changed` (Review) shows the before and after defaults.
- **Test case idea:** Feed a synthetic two-action `pkaction --verbose` text to the parser and diff it against a version where `implicit any` changed from `auth_admin` to `yes`. Assert the finding.
- **References:** https://www.freedesktop.org/software/polkit/docs/latest/polkit.8.html · https://www.qualys.com/2022/01/25/cve-2021-4034/pwnkit.txt · https://attack.mitre.org/techniques/T1548/

### HI-16 Root-equivalent group membership
- **What it is:** Several Unix groups are root in disguise:
  - `docker` (and the rootful `podman` socket if exposed), because the daemon socket can start a privileged container that mounts the host;
  - `lxd`/`incus-admin` and `libvirt`, which control VMs and containers;
  - `disk`, which gives raw block-device access;
  - `wheel`/`sudo`/`admin`, which grant sudo.
  
  Others leak secrets or input: `adm` and `systemd-journal` can read all logs, `shadow` can read hashes, and `input` can read every keyboard. Developers often add themselves to `docker` without realising it equals passwordless root for anything running as their user.
- **Normal state:** The admin user is in `wheel`/`sudo`. `docker` membership is a deliberate decision, or rootless Docker/Podman is used instead. `disk`, `shadow` and `input` contain no human users.
- **Real-world examples:** Docker's own security documentation states that only trusted users should control the Docker daemon, because it gives root-level access: https://docs.docker.com/engine/security/. Escape-to-host techniques are listed at [ATT&CK T1611](https://attack.mitre.org/techniques/T1611/).
- **What a defender inspects (indicators):**
  - `id` and `getent group wheel sudo admin adm systemd-journal docker lxd incus-admin libvirt disk shadow input kvm`
  - `ls -l /var/run/docker.sock /run/podman/podman.sock 2>/dev/null`
  - `systemctl is-active docker`
- **Equivalent checks:**
  - ATT&CK: T1078.003, T1611
  - Lynis: AUTH-9234 (inventory only)
  - CIS Ubuntu 24.04: 7.2.4 (shadow group empty), 5.2.7 (su restricted)
  - osquery: `user_groups`, `groups`
- **Bigman today:** Partial. `harden.group.docker` (Review) reports membership of the docker group and offers a previewed `gpasswd -d` with an undo ([hardening research](../research/hardening.md)). The other root-equivalent groups are not reported yet.
- **Proposed mitigation / detection:** Rootless `doctor` check `host.root-equivalent-group` (Review). Each group gets a one-line plain-language explanation and a link to the rootless alternative (rootless Docker, Podman). Baseline group membership and report `host.group-member-added`.
- **Test case idea:** Parser test using a synthetic `/etc/group` with `docker:x:970:alice` and a current-user name of `alice`. Assert the finding and its explanation text.
- **References:** https://docs.docker.com/engine/security/ · https://attack.mitre.org/techniques/T1611/ · https://attack.mitre.org/techniques/T1078/003/

### HI-17 SUID/SGID files not owned by a package or with changed mode
- **What it is:** Set-user-ID and set-group-ID executables run with their owner's or group's privileges. Each is attack surface (PwnKit and sudo bugs were all SUID binaries). An extra SUID root file anywhere is a ready-made way back to root. Defenders compare the inventory against what the package database says should be SUID.
- **Normal state:** About 15–40 SUID/SGID files, all package-owned (for example `su`, `sudo`, `passwd`, `chage`, `mount`, `umount`, `pkexec`, `fusermount3`, `newgrp`, `unix_chkpwd`, `ssh-keysign` and `Xorg.wrap`), and `rpm -V`/`dpkg --verify` shows no mode change. `/tmp`, `/var/tmp`, `/dev/shm` and `/home` are mounted `nosuid` where possible.
- **Real-world examples:**
  - Keydnap adds the setuid flag to a binary, and Exaramel for Linux uses a setuid helper ([ATT&CK T1548.001](https://attack.mitre.org/techniques/T1548/001/)).
  - OverlayFS CVE-2023-0386 (KEV 2025-06-17) involved setuid files with capabilities copied between mounts.
- **What a defender inspects (indicators):**
  - `find / -xdev \( -perm -4000 -o -perm -2000 \) -type f -printf '%m %u %g %p\n' 2>/dev/null | sort -k4`
  - Repeat for each local filesystem: `findmnt -rno TARGET -t ext4,xfs,btrfs`
  - For each file: `rpm -qf FILE` / `dpkg -S FILE`; `rpm -Va | grep '^.M'` (mode changes)
  - `findmnt -no OPTIONS /tmp /dev/shm /home /var/tmp`
  - Suspicious: an unowned SUID file, one under `/tmp`, `/home`, `/dev/shm`, `/opt` or `/var`, a SUID shell, interpreter or editor, or a package-owned file whose mode differs from the package.
- **Equivalent checks:**
  - ATT&CK: T1548.001
  - Lynis: no dedicated SUID inventory diff test in `tests.db` (FILE-7524 checks only profile-listed permissions)
  - CIS Ubuntu 24.04: 7.1.13 (SUID and SGID files reviewed, Manual), 1.1.2.x.3 (`nosuid` on `/tmp`, `/dev/shm`, `/home`, `/var`, `/var/tmp`)
  - SSG: `file_permissions_unauthorized_suid`, `file_permissions_unauthorized_sgid`
  - osquery: `suid_bin` table and incident-response pack query `suid_bin`
  - Falco incubating rules: "Set Setuid or Setgid bit", "Non sudo setuid"
  - AIDE tracks mode.
- **Bigman today:** Partial. A baselined startup or configuration file that gains the setuid or setgid bit is `persistence.privilege` (High). There is no system-wide SUID inventory yet.
- **Proposed mitigation / detection:** Rootless CLI `bigman audit host --suid`. Run a bounded walk of local filesystems (same entry and time limits style as `snapshot()`), resolve each file's package owner in a batch (`rpm -qf` accepts many paths), and verify mode against the DB. Rules:
  - `suid.unowned` (High)
  - `suid.mode-mismatch` (High)
  - `suid.new-since-baseline` (Review)
  - `mount.missing-nosuid` (Info)
  
  Unreadable directories are listed as coverage gaps, not passes.
- **Test case idea:** Create a temp file with mode 04755 owned by the test user. SUID bits on user-owned files in a tempdir are allowed on most test filesystems; skip the test if the mount is `nosuid`. Assert `suid.unowned`. A fake owner resolver drives the package path.
- **References:** https://attack.mitre.org/techniques/T1548/001/ · https://github.com/osquery/osquery/tree/master/packs · https://github.com/ComplianceAsCode/content

### HI-18 File capabilities on unexpected binaries
- **What it is:** File capabilities (the `security.capability` xattr) give a binary a subset of root powers without SUID. Examples are `cap_net_raw` for ping, or `cap_setuid`, `cap_sys_admin`, `cap_dac_read_search` and `cap_sys_ptrace`. A powerful capability on an interpreter or shell is equivalent to SUID root, and ordinary SUID-only audits miss it.
- **Normal state:** A short, package-owned list. On Fedora this typically includes `newuidmap`/`newgidmap` (`cap_setuid`/`cap_setgid`), `clockdiff`/`arping` (`cap_net_raw`) and a few desktop helpers. `rpm -V` tracks capabilities (verify flag `P`).
- **Real-world examples:** No named 2023–2026 incident verified. Capability-related setuid confusion appears in CVE-2023-0386 (OverlayFS; KEV). Defenders use the capabilities man page and GTFOBins' capabilities list as audit references.
- **What a defender inspects (indicators):**
  - `getcap -r / 2>/dev/null` (crosses filesystems; on large disks prefer per-mount `find -xdev -type f -exec getcap {} +`)
  - `rpm -qf` on each result
  - `rpm -Va | grep '^........P'`
  - Suspicious: any capability on Python, Perl, Node, Ruby, shells, `tar`, `vim` or similar general tools, or on unowned files. Also `cap_sys_admin`, `cap_sys_module`, `cap_dac_override` or `cap_setuid` outside the known short list.
- **Equivalent checks:**
  - ATT&CK: T1548 (no dedicated sub-technique)
  - Lynis: none found in `tests.db`
  - CIS: none
  - osquery: none dedicated (use `file` + extended attributes where supported; unverified)
  - AIDE and Samhain can track xattrs.
- **Bigman today:** Partial. File capabilities (`security.capability`) are recorded for baselined files, and newly present capabilities are High. Binaries outside the baseline are not inventoried.
- **Proposed mitigation / detection:** Read the `security.capability` xattr directly in Rust during the same walk as HI-17, so `getcap` is not a dependency. Decode the capability bitmask. Rules:
  - `caps.unowned` (High)
  - `caps.dangerous-on-interpreter` (High)
  - `caps.mismatch-package` (High, from the `rpm -V` P flag)
  - `caps.new-since-baseline` (Review)
  
  Also record xattr presence in the startup baseline entries (structural gap 1).
- **Test case idea:** Use a pure parser test with synthetic `security.capability` byte arrays (v2/v3 formats) to assert decoding. An optional root-only CI job can `setcap` a temp copy of `/bin/true`.
- **References:** https://man7.org/linux/man-pages/man7/capabilities.7.html · https://man7.org/linux/man-pages/man8/getcap.8.html · https://man7.org/linux/man-pages/man8/rpm.8.html

### HI-19 World-writable files and directories, writable PATH entries, unowned files
- **What it is:** A world-writable file in a system location lets any local process change what something else executes. A world-writable directory without the sticky bit lets anyone replace other users' files. A writable directory early in `PATH`, or in root's `secure_path`, lets a lower-privileged process shadow common commands. Files with no owning user or group are often left over from removed accounts, or extracted from archives with foreign UIDs.
- **Normal state:** `/tmp`, `/var/tmp` and `/dev/shm` are world-writable with the sticky bit. No other world-writable directories exist under `/usr`, `/etc`, `/opt` or `/var/lib`. Root's `PATH` and sudo's `secure_path` contain only root-owned, non-group/world-writable directories. User `PATH` often includes `~/.local/bin` and `~/bin`. That is normal on desktops, but it means anything running as the user can shadow commands for that user.
- **Real-world examples:** General technique ([ATT&CK T1574.007](https://attack.mitre.org/techniques/T1574/007/), [T1222.002](https://attack.mitre.org/techniques/T1222/002/)). No single dated desktop incident verified.
- **What a defender inspects (indicators):**
  - `find / -xdev -type d -perm -0002 ! -perm -1000 -printf '%m %u %p\n' 2>/dev/null`
  - `find / -xdev -type f -perm -0002 ! -path '/proc/*' ! -path '/sys/*' -printf '%m %u %p\n' 2>/dev/null`
  - `find / -xdev \( -nouser -o -nogroup \) -printf '%u %g %p\n' 2>/dev/null`
  - `printf '%s\n' "$PATH" | tr : '\n' | while read -r d; do stat -c '%A %U %n' "$d"; done` (flag empty entries and `.`)
  - `sudo -l | grep secure_path`
- **Equivalent checks:**
  - ATT&CK: T1574.007, T1222.002
  - Lynis: FILE-6362/6363 (sticky bit on `/tmp` and `/var/tmp`), FILE-6374 (mount options), FILE-7524
  - CIS Ubuntu 24.04: 7.1.11 (world-writable secured), 7.1.12 (no unowned files), 5.4.2.5 (root path integrity)
  - SSG: `file_permissions_unauthorized_world_writable`, `dir_perms_world_writable_sticky_bits`, `dir_perms_world_writable_root_owned`, `no_files_unowned_by_user`, `file_permissions_ungroupowned`, `accounts_root_path_dirs_no_write`
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** Rootless checks in the same bounded walk:
  - `perm.world-writable-system` (Review)
  - `perm.world-writable-dir-no-sticky` (Review)
  - `path.writable-before-system` (Info for `~/.local/bin`, with an explanation; Review for other writable entries before `/usr/bin`)
  - `perm.unowned` (Info)
- **Test case idea:** Tempdir with a 0777 directory without the sticky bit. Assert the finding. A synthetic `PATH` string `/tmp/x:/usr/bin`, with `/tmp/x` created 0777, asserts `path.writable-before-system`.
- **References:** https://attack.mitre.org/techniques/T1574/007/ · https://attack.mitre.org/techniques/T1222/002/ · https://github.com/ComplianceAsCode/content

### HI-20 Loaded kernel modules not owned by a package, unsigned or tainting
- **What it is:** Kernel-mode rootkits are usually loadable kernel modules (LKMs). Once loaded, a module can hide processes, files and network activity from every userland tool. A defender cannot fully trust a compromised kernel's answers. Still, three cheap cross-checks catch common cases and misconfigurations:
  - the loaded-module inventory compared with package-owned `.ko` files;
  - the kernel taint mask;
  - module signer information.
- **Normal state:** Every module in `/proc/modules` resolves through `modinfo -F filename` to a file under `/lib/modules/$(uname -r)/` owned by the kernel package. Exceptions are DKMS/akmods modules (NVIDIA, VirtualBox, v4l2loopback, ZFS), which the user installed knowingly. `/proc/sys/kernel/tainted` is `0`, or its bits are explained by those modules: P=1 proprietary, O=4096 out-of-tree, E=8192 unsigned, and K=32768 live patch.
- **Real-world examples:**
  - Pumakit (Elastic Security Labs, December 2024), an LKM rootkit with a userland component: https://www.elastic.co/security-labs/declawing-pumakit
  - Drovorub (NSA/FBI advisory, 2020; [ATT&CK S0502](https://attack.mitre.org/software/S0502/)), REPTILE ([S1219](https://attack.mitre.org/software/S1219/)) and Skidmap.
  - Operation CuckooBees used a **signed** kernel rootkit, so signature alone is not enough ([ATT&CK T1547.006](https://attack.mitre.org/techniques/T1547/006/)).
- **What a defender inspects (indicators):**
  - `cat /proc/modules` (or `lsmod`); for each module: `modinfo -F filename MOD`, `modinfo -F signer MOD`, `modinfo -F sig_key MOD`
  - `rpm -qf "$(modinfo -F filename MOD)"` / `dpkg -S ...`
  - `dkms status`
  - `cat /proc/sys/kernel/tainted`; per module: `cat /sys/module/MOD/taint`
  - `journalctl -k | grep -iE 'taint|module verification failed'`
  - Cross-view: compare module names in `/proc/modules` with directories in `/sys/module/` (built-ins appear only in `/sys/module`, so compare loadable ones). Treat unexplained differences as an indicator worth escalating to offline analysis.
  - Suspicious: a module whose file is not package- or DKMS-owned, a module with no backing file, taint bits E or F appearing with no known cause, or a signer that is neither the distro key nor the user's own MOK.
- **Equivalent checks:**
  - ATT&CK: T1547.006, T1014
  - Lynis: KRNL-5726 / KRNL-5770 (loaded modules), MALW-3275/3276 (chkrootkit/rkhunter present)
  - CIS Ubuntu 24.04: 6.2.3.19 (module load/unload audited), 1.1.1.x and 3.2.x (unneeded modules unavailable)
  - SSG: `audit_rules_kernel_module_loading`
  - osquery: `kernel_modules`, `kernel_info`
  - auditd (Neo23x0): `-S init_module,finit_module,delete_module -k modules`
  - Tracee detectors: `kernel_module_loading`, `hidden_kernel_module`, `syscall_table_hooking`, `hooked_syscall`
  - Falco: "Linux Kernel Module Injection Detected"
  - chkrootkit: `lkm`, `chkproc`/`chkdirs`; rkhunter: LKM checks
- **Bigman today:** Partial. `bigman running` compares `/proc/modules` with `/sys/module` and the kernel's symbol list (`hidden.module`), decodes the taint mask and the per-module marks (`module.taint`), and reports loaded modules with no file in the running kernel's module index (`module.no-file`). Coverage says a kernel that filters every view defeats them. Not done: package owner and signer of each module file, a module baseline (`kernel.module-new`).
- **Proposed mitigation / detection:** Rootless CLI and monitor check `kernel.modules`:
  - Baseline module names, file path, package owner, signer and taint mask, and decode the taint bits into words using the kernel's table.
  - Rules: `kernel.module-unowned` (High), `kernel.module-new` (Review), `kernel.taint-changed` (Review, with decoded bits), `kernel.module-crossview-mismatch` (High).
  - Coverage text must say that a compromised kernel can lie to all of these checks.
- **Test case idea:** Run a taint decoder unit test (`4096|8192` → "O out-of-tree, E unsigned"). Feed `/proc/modules`- and `/sys/module`-shaped fixture directories to the cross-view comparator.
- **References:** https://docs.kernel.org/admin-guide/tainted-kernels.html · https://docs.kernel.org/admin-guide/module-signing.html · https://attack.mitre.org/techniques/T1547/006/ · https://attack.mitre.org/techniques/T1014/ · https://www.elastic.co/security-labs/declawing-pumakit

### HI-21 Secure Boot, kernel lockdown, signature enforcement and boot chain state
- **What it is:** Secure Boot, together with shim and MOK, limits which bootloaders and kernels run. Kernel lockdown (integrity or confidentiality mode) blocks root from modifying the running kernel. `module.sig_enforce` refuses unsigned modules. These controls raise the cost of bootkits and kernel rootkits. Kernel command-line flags or enrolled keys can quietly weaken them.
- **Normal state:**
  - `mokutil --sb-state` reports `SecureBoot enabled` on UEFI hardware.
  - `/sys/kernel/security/lockdown` shows `[integrity]`, or `[none]` where the distro does not tie lockdown to Secure Boot. Fedora 44 showed `[none] integrity confidentiality` on the test host.
  - Enrolled MOKs are the user's own DKMS/akmods key or none.
  - `/proc/cmdline` has no `module.sig_enforce=0`, `lockdown=none`, `selinux=0`, `enforcing=0` or `apparmor=0` the user did not set.
  - `/boot` kernels are package-owned, and `bootctl status` and the fwupd HSI report raise no surprises.
- **Real-world examples:**
  - Bootkitty (ESET, November 2024), the first UEFI bootkit aimed at Linux. It relied on Secure Boot being off or an attacker certificate being enrolled: https://www.welivesecurity.com/en/eset-research/bootkitty-analyzing-first-uefi-bootkit-linux/
  - Binarly later tied Bootkitty to LogoFAIL: https://www.binarly.io/blog/logofail-exploited-to-deploy-bootkitty-the-first-uefi-bootkit-for-linux
  - [ATT&CK T1542.003](https://attack.mitre.org/techniques/T1542/003/), [T1553.006](https://attack.mitre.org/techniques/T1553/006/).
- **What a defender inspects (indicators):**
  - `mokutil --sb-state`; `mokutil --list-enrolled | grep -E 'Subject:|Issuer:'`
  - `bootctl status` (systemd-boot/UEFI info, rootless on most systems)
  - `cat /sys/kernel/security/lockdown`; `cat /sys/module/module/parameters/sig_enforce 2>/dev/null`; `cat /proc/cmdline`
  - `rpm -qf /boot/vmlinuz-*` and `rpm -V shim-x64 grub2-efi-x64` (Debian: `dpkg --verify shim-signed grub-efi-amd64-signed`)
  - `fwupdmgr security` (Host Security ID report) when fwupd is installed
  - Initramfs is generated locally, not package-owned. Baseline its hash after each kernel update (`lsinitrd` / `lsinitramfs` list its contents).
- **Equivalent checks:**
  - ATT&CK: T1542.003, T1553.006, T1014
  - Lynis: BOOT-5116 (UEFI mode), BOOT-5122 (GRUB password), BOOT-5184 (boot file permissions), FINT-4339 (IMA/EVM), FINT-4340/4341 (dm-integrity/dm-verity)
  - CIS Ubuntu 24.04: 1.4.1 (bootloader password), 1.4.2 (bootloader config access)
  - SSG: `grub2_password`
  - osquery: `secureboot`, `secureboot_certificates`
- **Bigman today:** Partial. `harden.secure_boot` (Info) reads the `SecureBoot` EFI variable without root and explains how to turn it on ([hardening research](../research/hardening.md)). Lockdown, the kernel command line and `/boot` verification are not read yet. `doctor` runs `getenforce`.
- **Proposed mitigation / detection:** Rootless `doctor` section "Boot and kernel protections", recording all of the read-only values above in the baseline. Rules:
  - `boot.secureboot-off` (Info on non-UEFI systems, Review on UEFI)
  - `boot.mok-added` (Review)
  - `boot.cmdline-weakened` (Review; fixed list of flags)
  - `boot.kernel-unowned` (High)
  - `boot.initramfs-changed-without-kernel-update` (Review)
- **Test case idea:** Parser tests on fixture strings for `mokutil --sb-state`, the lockdown file and `/proc/cmdline` (`... lockdown=none`).
- **References:** https://man7.org/linux/man-pages/man7/kernel_lockdown.7.html · https://docs.kernel.org/admin-guide/module-signing.html · https://www.kernel.org/doc/html/latest/admin-guide/kernel-parameters.html · https://fwupd.github.io/libfwupdplugin/hsi.html · https://wiki.debian.org/SecureBoot

### HI-22 Package-file verification (modified or unowned system files)
- **What it is:** Package managers record a digest, mode, owner and (on RPM) file capabilities for every file they install. Re-checking those values finds replaced binaries and libraries (trojanized `sshd`, `libkeyutils` or `ps`), mode changes and unexpected unowned files. It is the cheapest strong integrity check available on a desktop, with limits: a root attacker can also edit the package database, Debian's md5sums are weak and optional, and config files are expected to differ.
- **Normal state:**
  - `rpm -Va` lists only `c` (config) files the user edited, plus a few known-noisy entries such as mtime-only `.T` changes.
  - `dpkg --verify` and `debsums -s` are quiet apart from edited conffiles.
  - `pacman -Qkk` reports only backup files.
- **Real-world examples:**
  - Ebury replaced or modified OpenSSH and `libkeyutils` (Operation Windigo, ESET 2014): https://www.welivesecurity.com/2014/03/18/operation-windigo-the-vivisection-of-a-large-linux-server-side-credential-stealing-malware-campaign/
  - Bonadan altered the OpenSSH binary ([ATT&CK T1554](https://attack.mitre.org/techniques/T1554/)).
  - Plague masqueraded as a system library (HI-08).
- **What a defender inspects (indicators):**
  - Fedora/RHEL/openSUSE: `rpm -Va 2>/dev/null` (flags: S size, 5 digest, M mode, U user, G group, T mtime, P capabilities; `c` marks config). Per package: `rpm -V PKG`.
  - Debian/Ubuntu: `dpkg --verify` (digest only), `debsums -s` (changed files), `debsums -l` (packages shipping no md5sums), `debsums -c`.
  - Arch: `pacman -Qkk`; `paccheck --sha256sum` (pacutils).
  - Unowned files in system paths: `find /usr/bin /usr/sbin /usr/lib /usr/lib64 -xdev -type f -newer /etc/os-release -print0 | xargs -0 rpm -qf 2>&1 | grep 'not owned'`.
  - Suspicious: digest mismatch (`5`) on a non-config binary or library, or an unowned executable in `/usr/bin` or a library directory.
  - Strongest form: verify from trusted media, or compare against freshly downloaded, signature-checked packages (`rpm -K PKG.rpm`), which does not trust the local DB.
- **Equivalent checks:**
  - ATT&CK: T1554, T1014
  - Lynis: PKGS-7354 (package DB integrity), PKGS-7370 (debsums present), PKGS-7390 (Ubuntu DB consistency), FINT-4350 (FIM installed), FINT-4402 (checksums)
  - SSG: `rpm_verify_hashes`, `rpm_verify_permissions`, `rpm_verify_ownership`
  - osquery: `rpm_package_files` / `deb_package_files` combined with `hash`
  - rkhunter: file properties check
  - AIDE, Tripwire OSS and Samhain: full FIM
- **Bigman today:** Partial. Package attribution reads the rpm database (digest, mode, owner, link target; `rpm -V` for non-SHA-256 packages), `dpkg --verify` or `pacman -Qkk` for the files `bigman changes` covers. A scheduled whole-system verification is not implemented.
- **Proposed mitigation / detection:**
  - A scheduled user timer, similar to the existing daily scan timer, running `rpm -Va` / `dpkg --verify` / `pacman -Qkk` with a time bound, as the `runner` already enforces.
  - Normalise the output into a stable list, filter `c`/conffiles into a separate Info bucket, and diff against the previous run.
  - Rules: `pkg.file-digest-mismatch` (High for ELF binaries and shared objects; Review otherwise), `pkg.file-mode-mismatch` (Review; High for SUID/caps), `pkg.unowned-executable` (Review).
  - Rootless runs report "permission denied" lines as coverage gaps.
  - Correlate each mismatch with package transaction logs (`dnf history`, `/var/log/dpkg.log`, `/var/log/pacman.log`), so a mismatch that appears straight after an interrupted update is explained.
- **Test case idea:** Inject canned `rpm -Va` output into the parser (`S.5....T.    /usr/bin/fakebin`, `S.5....T.  c /etc/fake.conf`) and assert the classification. In a Fedora CI container, an optional integration test modifies a non-critical file of a dummy RPM built in the test and runs `rpm -V` on it.
- **References:** https://man7.org/linux/man-pages/man8/rpm.8.html · https://man7.org/linux/man-pages/man1/dpkg.1.html · https://manpages.debian.org/debsums · https://man.archlinux.org/man/pacman.8 · https://attack.mitre.org/techniques/T1554/

### HI-23 Kernel and userspace patch status; well-known Linux LPE CVEs
- **What it is:** A common path to root after initial access is a public local privilege escalation (LPE) in the kernel, glibc, sudo or polkit. CISA's KEV catalog lists 32 Linux kernel entries, and 16 of them were added in 2025–2026. Two caveats apply:
  - Distros backport fixes without changing the upstream version, so "is my sudo older than 1.9.17p1?" is the wrong question. The right question is "does my distro's security tracker say the installed package is fixed?".
  - A patched kernel or glibc only protects after a reboot or restart.
- **Normal state:** No pending security advisories. The running kernel equals the newest installed kernel (or a live patch covers the gap). No processes are still mapping deleted, upgraded libraries.
- **Real-world examples:** See the LPE checklist below. CVE-2024-1086 and CVE-2021-4034 are marked by CISA as known to be used in ransomware campaigns.
- **What a defender inspects (indicators):**
  - Fedora (dnf5): `dnf advisory list --security`, `dnf advisory list --cves=CVE-2024-1086`, `dnf needs-restarting` (dnf4: `dnf updateinfo list --security`, `dnf updateinfo info --cve CVE-…`, `dnf needs-restarting -r`)
  - Debian: `apt list --upgradable`, `debsecan --suite "$(. /etc/os-release; echo $VERSION_CODENAME)" --only-fixed`; Debian tracker: https://security-tracker.debian.org/tracker/
  - Ubuntu: `pro fix CVE-… --dry-run`, `ubuntu-security-status`, `/var/run/reboot-required`; CVE pages under https://ubuntu.com/security/
  - Arch: `arch-audit`
  - Running vs installed kernel: `uname -r` vs `rpm -q --last kernel-core | head -1` (Debian: `dpkg -l 'linux-image-*'`)
  - Live patch: `kpatch list`, `canonical-livepatch status`
  - Stale libraries: `needrestart -b` (Debian/Ubuntu), `dnf needs-restarting`
- **Equivalent checks:**
  - ATT&CK: T1068
  - Lynis: PKGS-7392 (Debian/Ubuntu security updates), PKGS-7352 (DNF security), KRNL-5830 (running latest installed kernel), KRNL-5788 (new kernel available), PKGS-7420 (auto-update tooling)
  - CIS Ubuntu 24.04: 1.2.2.1 (updates and patches installed)
  - SSG: `security_patches_up_to_date`
  - osquery: `vuln-management` pack, `rpm_packages`/`deb_packages`
- **Bigman today:** Partial. `bigman advisories` (Protect > Updates) matches installed packages against the distribution's own data (dnf updateinfo on Fedora, the Debian security tracker, the Arch security tracker), raises updates that fix a CISA KEV entry to High, and flags a pending reboot. A newer installed kernel than the running one is `advisory.reboot-required`. Well-known LPE checks beyond the distribution data are not implemented.
- **Proposed mitigation / detection:**
  - Rootless `doctor` check `patch.status`: run the distro-native advisory query read-only, with dnf/apt cache-only flags so `doctor` never refreshes metadata without consent.
  - Rules: `patch.security-pending` (Review, listing advisory IDs and CVEs) and `patch.reboot-pending` (Review; running kernel older than installed, or `reboot-required` present).
  - Feed-driven: ship a small, signed, versioned data file of KEV-listed Linux CVEs (ID, component, KEV date) and match against distro tracker answers. Never guess from upstream version numbers.
  - The "update package X because of CVE Y" message must quote the distro advisory.
  - Cross-reference [vulnerability-intelligence-and-feeds](../research/vulnerability-intelligence-and-feeds.md).
- **Test case idea:** Parser tests on canned `dnf advisory list --security` and `debsecan` outputs. A fixture `/proc/sys/kernel/osrelease` string paired with a fake installed-kernel list asserts `patch.reboot-pending`.
- **References:** https://www.cisa.gov/known-exploited-vulnerabilities-catalog · https://access.redhat.com/security/security-updates/cve · https://ubuntu.com/security/notices · https://manpages.debian.org/debsecan · https://attack.mitre.org/techniques/T1068/

#### LPE and privileged-service CVE checklist (2021–2026)

Confirm each item is fixed **according to your distro's tracker**, not by comparing upstream versions. "KEV" is the date CISA added it to the Known Exploited Vulnerabilities catalog (catalog version 2026.10.28). Affected and fixed ranges are from NVD or the original advisory, as noted. "—" means not in KEV as of 2026-10-28.

| CVE | Name / component | Public | Upstream affected → fixed (source) | KEV | Defender confirmation |
|---|---|---|---|---|---|
| CVE-2021-3156 | "Baron Samedit", sudo heap overflow | 2021-01-26 | sudo before 1.9.5p2 (Qualys advisory) | 2022-04-06 | Distro sudo advisory; `rpm -q sudo` / `dpkg -l sudo` vs tracker |
| CVE-2021-3560 | polkit D-Bus authentication bypass | 2021-06 | polkit before 0.119 (NVD) | 2023-05-12 | polkit package vs tracker |
| CVE-2021-33909 | "Sequoia", kernel seq_file size overflow | 2021-07-20 | Linux 3.16 to before 5.13.4 (NVD) | — | Running kernel vs tracker |
| CVE-2021-22555 | netfilter x_tables heap OOB write (reachable via user namespaces per KEV) | 2021-07 | fix commits listed in the KEV entry | 2025-10-06 | Running kernel vs tracker |
| CVE-2021-3493 | Ubuntu overlayfs file-capability LPE | 2021-04 | Ubuntu kernels (KEV) | 2022-10-20 | Ubuntu USN for the kernel |
| CVE-2021-22600 | kernel packet socket double free | 2022-01 | see NVD | 2022-04-11 | Running kernel vs tracker |
| CVE-2021-4034 | "PwnKit", polkit pkexec memory corruption | 2022-01-25 | polkit before 121 (NVD) | 2022-06-27 (ransomware: Known) | polkit package vs tracker; pkexec SUID inventory (HI-17) |
| CVE-2022-0185 | kernel fs_context heap overflow | 2022-01 | see NVD | 2024-08-21 | Running kernel vs tracker |
| CVE-2022-0492 | cgroups v1 `release_agent` authentication flaw | 2022-02 | fix commit 24f600856418 (KEV) | 2026-06-02 | Running kernel vs tracker |
| CVE-2022-0847 | "Dirty Pipe", pipe buffer flag initialisation | 2022-03-07 | 5.8 to before 5.10.102 / 5.15.25 / 5.16.11 (NVD) | 2022-04-25 | Running kernel vs tracker |
| CVE-2022-0995 | kernel watch_queue OOB write | 2022-03 | fix commit 93ce93587d36 (KEV) | 2026-08-26 | Running kernel vs tracker |
| CVE-2022-2586 | nf_tables use-after-free | 2022-08 | see NVD | 2024-06-26 | Running kernel vs tracker |
| CVE-2022-2588 | cls_route use-after-free | 2022-08 | see NVD (NVD published 2024-01-08) | — | Running kernel vs tracker |
| CVE-2023-22809 | sudoedit extra-argument handling | 2023-01-18 | sudo before 1.9.12p2 (NVD) | — | sudo vs tracker |
| CVE-2023-0386 | OverlayFS setuid/capability copy-up ownership | 2023-03 | 5.11 to before 5.15.91, 5.16 to before 6.1.9 (NVD) | 2025-06-17 | Running kernel vs tracker |
| CVE-2023-32233 | nf_tables batch use-after-free | 2023-05-08 | Linux through 6.3.1 (NVD) | — | Running kernel vs tracker |
| CVE-2023-3269 | "StackRot", VMA lock handling use-after-free | 2023-07 | see NVD | — | Running kernel vs tracker |
| CVE-2023-2640 / CVE-2023-32629 | "GameOver(lay)", Ubuntu-specific overlayfs | 2023-07-26 | Ubuntu kernels with specific SAUCE patches (NVD; Wiz) | — | Ubuntu USN |
| CVE-2023-4911 | "Looney Tunables", glibc ld.so `GLIBC_TUNABLES` overflow | 2023-10-03 | see Qualys advisory and distro trackers | 2023-11-21 | glibc package vs tracker, then reboot |
| CVE-2024-1086 | nf_tables use-after-free | 2024-01 | 3.15 to before 5.15.149, 6.1 to before 6.1.76, 6.2 to before 6.6.15, 6.7 to before 6.7.3 (NVD) | 2024-05-30 (ransomware: Known) | Running kernel vs tracker |
| CVE-2024-6387 | "regreSSHion", sshd signal-handler race (**remote**, unauthenticated) | 2024-07-01 | OpenSSH 8.5p1–9.7p1 → 9.8p1 (Qualys / OpenSSH 9.8 release notes) | — | openssh-server vs tracker; is sshd even enabled (HI-10)? |
| CVE-2024-48990 | needrestart `PYTHONPATH` handling (and sibling CVEs in the same advisory) | 2024-11-19 | needrestart before 3.8 (NVD) | — | needrestart vs tracker (Ubuntu server installs it by default) |
| CVE-2025-6018 | PAM configuration on openSUSE/SLE 15 (`pam_env`) granting `allow_active` | 2025-06-17 | see Qualys advisory | — | SUSE advisory |
| CVE-2025-6019 | libblockdev/udisks via polkit `allow_active` | 2025-06-17 | see Qualys advisory | — | libblockdev/udisks2 vs tracker; review HI-15 |
| CVE-2025-32463 | sudo `--chroot` option LPE | 2025-06-30 | sudo 1.9.14–1.9.17 → 1.9.17p1 (sudo.ws advisory) | 2025-09-29 | sudo vs tracker |
| CVE-2025-32462 | sudo `--host` option with host-restricted rules | 2025-06-30 | sudo before 1.9.17p1 (NVD) | — | sudo vs tracker |
| CVE-2025-38352 | kernel POSIX CPU timers TOCTOU race | 2025 | fix commit 2c72fe18cc5f (KEV; Android bulletin 2025-09) | 2025-09-04 | Running kernel vs tracker |
| CVE-2025-39682 | kernel TLS receive-path record handling | 2025 | fix commits in KEV entry | 2026-09-18 | Running kernel vs tracker |
| CVE-2025-39964 | AF_ALG socket concurrent-write race | 2025 | fix commits in KEV entry | 2026-09-18 | Running kernel vs tracker |
| CVE-2026-31431 | "Copy Fail", AF_ALG `algif_aead` in-place operation (page-cache modification) | 2026-04-29 (NVD 2026-04-22) | fixed upstream 2026-04-01 (Xint write-up) | 2026-05-01 | Running kernel vs tracker. Interim vendor-suggested mitigation: make `algif_aead` unavailable via modprobe.d (HI-25) |
| CVE-2026-53362 | kernel IPv6 subsystem LPE (details unspecified in KEV) | 2026 | fix commits in KEV entry | 2026-08-27 | Running kernel vs tracker |
| CVE-2026-53266 | ebtables SNAT target OOB write | 2026 | fix commits in KEV entry | 2026-09-18 | Running kernel vs tracker |
| CVE-2026-46300 | skb coalescing / ESP in-place decrypt affecting page-cache data | 2026 | fix commits in KEV entry | 2026-10-28 | Running kernel vs tracker; added the day before this research, so expect distro advisories to be catching up |

Older bugs that KEV has added recently and that still matter for unpatched or long-term-support hosts:
- CVE-2016-5195 "Dirty COW" (KEV 2022-03-03)
- CVE-2019-13272 ptrace (KEV 2021-12-10)
- CVE-2018-14634 (KEV 2026-01-26)
- CVE-2015-3246 libuser (KEV 2026-08-26)
- CVE-2015-5287 ABRT (KEV 2026-08-26)

KEV also lists Linux USB/HID driver bugs with physical-access exploitation, cited to Android bulletins: CVE-2024-53104 UVC, CVE-2024-53150/53197 USB audio, and CVE-2024-50302 HID info leak. On desktops these are a reason to keep kernels current and to consider USBGuard (Lynis USB-3000).

A note on the 2026 "page-cache write" bugs (Copy Fail, CVE-2026-46300, CVE-2026-53266) and the 2022 Dirty Pipe: they modify the cached in-memory copy of a file rather than the file on disk. Integrity tools that hash files through the page cache might see modified content until eviction or reboot, while offline verification sees the clean disk copy. For this class, patch status (HI-23) is the control that counts, and FIM results should be read with that limit in mind.

### HI-24 Audit, journal and integrity-tool health
- **What it is:** Detection depends on its sources working. auditd must be running with rules loaded, journald must keep logs across reboots, an AIDE database must exist and be recent, and Bigman's own monitor must report a heartbeat. Attackers disable or clear these. An unnoticed gap is as bad as no tool at all.
- **Normal state:**
  - auditd is active with a non-empty rule set (on Fedora and RHEL it is installed by default; Ubuntu desktops usually lack it).
  - `/var/log/journal` exists, giving persistent storage, and `journalctl --list-boots` shows continuous history.
  - If AIDE is installed, `/var/lib/aide/aide.db.gz` is present and recently refreshed.
  - The Bigman monitor heartbeat is under 10 s old.
- **Real-world examples:** Log clearing and audit tampering are documented at [ATT&CK T1685.004](https://attack.mitre.org/techniques/T1685/004/) (formerly T1562.012) and [T1685.006](https://attack.mitre.org/techniques/T1685/006/) (formerly T1070.002).
- **What a defender inspects (indicators):**
  - `systemctl is-active auditd`; `sudo auditctl -s`; `sudo auditctl -l | wc -l`
  - `journalctl --disk-usage`; `ls -ld /var/log/journal`; `journalctl --list-boots | tail`
  - `ls -l /var/lib/aide/aide.db*` (often root-only); `systemctl list-timers | grep -i aide`
  - `ls -l /var/log/wtmp /var/log/btmp /var/log/lastlog 2>/dev/null`
  - Suspicious: auditd stopped or with zero rules, a missing or zero-length wtmp, boot-history gaps, or an AIDE DB older than the last package update.
- **Equivalent checks:**
  - ATT&CK: T1685, T1685.004, T1685.006
  - Lynis: ACCT-9628/9630/9632/9634 (auditd), LOGG-2136 (journald), FINT-4314/4315/4316 (AIDE), FINT-4350 (FIM present), TOOL-5126/5128 (OSSEC/Wazuh)
  - CIS Ubuntu 24.04: 6.1.1.1 (journald active), 6.1.2.4 (journald Storage), 6.2.1.3 (audit early-boot processes), 6.3.1/6.3.2 (AIDE installed and run regularly)
  - osquery: `osquery-monitoring` pack
- **Bigman today:** Partial. `doctor` reports tool availability (including lynis and osqueryi), ClamAV freshness and the canary monitor heartbeat. bigmand also records gaps: `monitor.stopped` when the heartbeat has been missing for a minute while the computer was awake, and `monitor.gap` when the monitor or bigmand itself comes back after being off (suspend excluded; a crash or kill of bigmand is Review, a normal stop Info). It does not check auditd, journald persistence or AIDE.
- **Proposed mitigation / detection:**
  - Extend `doctor` with rootless reads: auditd unit state, journald storage directory and boot list, and AIDE timer and file presence.
  - Rules: `logging.audit-inactive` (Info on distros without default auditd; Review where it was previously active), `logging.journal-volatile` (Info) and `logging.boot-gap` (Review).
  - Offer (do not apply) the Neo23x0 ruleset as an optional setup action.
- **Test case idea:** Parser test on canned `journalctl --list-boots` output with a missing index, asserting `logging.boot-gap`. Mock `systemctl is-active auditd` returning `inactive`.
- **References:** https://attack.mitre.org/techniques/T1685/004/ · https://github.com/linux-audit/audit-userspace · https://github.com/Neo23x0/auditd · https://aide.github.io/

### HI-25 Kernel attack-surface sysctls and rarely used module blocklists
- **What it is:** Many kernel LPEs are reachable by unprivileged users only through features that a given host may not need: unprivileged user namespaces, unprivileged BPF, rarely used network protocols and AF_ALG crypto sockets. Some sysctls also reduce information leaks or block live kernel replacement. Reporting these settings helps users judge exposure. Desktop trade-offs are real, though: Flatpak, bubblewrap, Chromium's sandbox and Bigman's own `sandbox` and PDF parsers rely on unprivileged user namespaces.
- **Normal state (typical modern desktop):**
  - `kernel.yama.ptrace_scope` ≥ 1 is the kernel's default and Ubuntu's and Arch's, but Fedora and Debian ship 0 (a sysctl file from `elfutils-default-yama-scope` on Fedora, a kernel patch on Debian)
  - `kernel.kptr_restrict` ≥ 1 on Ubuntu; Fedora, Debian and Arch leave it at 0
  - `kernel.dmesg_restrict` = 1 (built in with `CONFIG_SECURITY_DMESG_RESTRICT` on Fedora, Debian, Ubuntu and Arch)
  - `kernel.unprivileged_bpf_disabled` = 1 or 2
  - `fs.protected_symlinks`, `fs.protected_hardlinks` = 1; `fs.protected_fifos` and `fs.protected_regular` ≥ 1
  - `fs.suid_dumpable` = 0
  - `kernel.randomize_va_space` = 2
  - User namespaces enabled but, on Ubuntu 24.04, mediated by AppArmor (`kernel.apparmor_restrict_unprivileged_userns`)
  - Rare protocols (dccp, sctp, rds, tipc) not loaded
- **Real-world examples:** The KEV entry for CVE-2021-22555 notes exploitation through user namespaces. Vendor guidance for CVE-2026-31431 "Copy Fail" suggests blocking `algif_aead` until patched (Xint write-up). CIS Ubuntu 24.04 3.2.1–3.2.4 make dccp, tipc, rds and sctp unavailable.
- **What a defender inspects (indicators):**
  - `sysctl kernel.yama.ptrace_scope kernel.kptr_restrict kernel.dmesg_restrict kernel.unprivileged_bpf_disabled kernel.kexec_load_disabled kernel.randomize_va_space fs.protected_symlinks fs.protected_hardlinks fs.protected_fifos fs.protected_regular fs.suid_dumpable user.max_user_namespaces 2>/dev/null`
  - `sysctl kernel.apparmor_restrict_unprivileged_userns 2>/dev/null` (Ubuntu)
  - `grep -E '^(dccp|sctp|rds|tipc|algif_aead|af_alg) ' /proc/modules`
  - `grep -rnE '^\s*(install|blacklist)\s+(dccp|sctp|rds|tipc|algif_aead)' /etc/modprobe.d /usr/lib/modprobe.d`
- **Equivalent checks:**
  - ATT&CK: T1068
  - Lynis: KRNL-6000 (sysctl key pairs), KRNL-5820 (core dumps)
  - CIS Ubuntu 24.04: 1.5.1 (ASLR), 1.5.2 (ptrace_scope), 1.5.3 (core dumps), 3.2.1–3.2.4 (rare protocols unavailable)
  - SSG: `sysctl_kernel_yama_ptrace_scope`, `sysctl_kernel_unprivileged_bpf_disabled`, `sysctl_kernel_kexec_load_disabled`, `sysctl_user_max_user_namespaces`, `coredump_disable_storage`
  - Neo23x0 auditd keys `af_alg`, `io_uring`, `userfaultfd`, `socket_bpf_filter_attached` record use of these surfaces.
- **Bigman today:** Partial. `bigman harden` reads `ptrace_scope`, `kptr_restrict` and `dmesg_restrict` without root, names the sysctl file and package that set each one, and offers a previewed drop-in under `/etc/sysctl.d` with an undo (`harden.sysctl.*`; reasoning in [hardening research](../research/hardening.md)). The other sysctls and module blocklists are not read yet.
- **Proposed mitigation / detection:**
  - Rootless `doctor` section "Kernel exposure" that reads each value and explains it, with an explicit note where the hardened value would break sandboxes. Rules are Info only; no auto-change.
  - Feed-driven: when a KEV entry names a module-scoped mitigation (as with Copy Fail), the advisory text can say "this module is loaded on your system / not loaded" without changing anything.
- **Test case idea:** Run the sysctl parser over a fixture directory shaped like `/proc/sys`, asserting text and severity. For the module-loaded check, use a fixture `/proc/modules` containing `algif_aead`.
- **References:** https://docs.kernel.org/admin-guide/sysctl/kernel.html · https://www.kernel.org/doc/html/latest/admin-guide/LSM/Yama.html · https://xint.io/blog/copy-fail-linux-distributions · https://attack.mitre.org/techniques/T1068/

## Tools and scripts worth mining for rules

Bigman is AGPL-3.0. GPL-3.0, Apache-2.0 and BSD code can be combined with AGPL-3.0. **GPL-2.0-only** code cannot be copied into an AGPL-3.0 work. Paths and check ideas (facts) can be reimplemented freely, but rule text and code from GPL-2.0-only projects should not be copied. Licenses below were read from each project's GitHub license API or `LICENSE` file, or from Debian copyright files, on 2026-10-29.

| Tool | URL | License | What we can borrow (specific checks / lists / regexes) | Caveats |
|---|---|---|---|---|
| Lynis | https://github.com/CISOfy/lynis | GPL-3.0 (release 3.1.7, 2026-06-25) | Test-ID catalogue `db/tests.db` (449 tests) as a cross-reference for Bigman rule IDs; AUTH-92xx account checks; SCHD-77xx cron/at; KRNL-5726/5830; PKGS-7392; FINT-43xx FIM-presence logic. Bigman already parses `warning[]`/`suggestion[]`. | Lacks dedicated SUID-diff, capability, polkit, udev or lockdown/taint tests. Many tests need root. |
| AIDE | https://github.com/aide/aide | GPL-2.0 (v0.19.5, 2026-10-22) | Rule-group concepts (`p+i+n+u+g+s+m+c+acl+selinux+xattrs+sha256`), and the idea of separate attribute sets for logs, configs and binaries. Distro default `aide.conf` path lists are a good starting list of what to watch. | Needs root for the full tree. DB on the same disk can be tampered. GPL-2.0: reimplement, do not copy. |
| Tripwire Open Source | https://github.com/Tripwire/tripwire-open-source | GPL-2.0 (2.4.3.7, 2018) | Signed policy and DB concepts (site/local keys) as a model for signing Bigman's baseline. | Effectively unmaintained since 2018. |
| Samhain | https://www.la-samhna.de/samhain/ | GPL-2.0-or-later (Debian `samhain` 4.1.4) | Kernel-level checks, SUID inventory, login/logout monitoring, and client/server signed-DB design. | Complex. Mainly server-oriented. |
| OSSEC (rootcheck, syscheck) | https://github.com/ossec/ossec-hids/tree/master/src/rootcheck/db | GPL-2.0 | `rootkit_files.txt` and `rootkit_trojans.txt` data lists, and system-audit policy files as a list of known-bad paths. | Lists are old (mostly pre-2015 rootkits). GPL-2.0. |
| Wazuh (FIM, rootcheck, SCA) | https://github.com/wazuh/wazuh/tree/master/ruleset/rootcheck · https://github.com/wazuh/wazuh/tree/main/ruleset/sca | GPL-2.0 for most of the tree; engine AGPL-3.0-or-later; some shared modules GPL-2.0-only OR AGPL-3.0-or-later (per LICENSE, 2026) | SCA YAML policies (CIS-derived checks expressed as file/command/regex rules), a good model for a declarative Bigman check format. Rootcheck hidden-process/port checks as cross-view ideas. | Mixed licenses. Check each file. SCA content is CIS-derived. |
| rkhunter | https://rkhunter.sourceforge.net/ | GPL-2.0-or-later (Debian copyright) | File-properties check (hash, perm, owner of system binaries vs package manager), hidden-file and known-rootkit path lists, sshd/syslog config checks. | Last upstream release 1.4.6 (2018). High false-positive rate. |
| chkrootkit | https://www.chkrootkit.org/ | BSD-2-Clause (most files; Debian copyright) | Test list (`lkm`, `chkproc`, `chkdirs`, `sniffer`, `wted`, `chkutmp`). Version 0.59 (2026-01-01) added an "executed from memory" process check and XZ-backdoor and Bootkitty checks. Permissive license, so logic can be reused with attribution. | Shell plus small C helpers. Many tests target ancient rootkits. |
| osquery | https://github.com/osquery/osquery (packs: https://github.com/osquery/osquery/tree/master/packs) | Apache-2.0 OR GPL-2.0-only | Packs `incident-response` (crontab, `suid_bin`, `kernel_modules`, `shell_history`, `listening_ports`), `ossec-rootkit`, `linux-ssh-investigation`, `vuln-management`. Tables: `authorized_keys`, `sudoers`, `sshd_config`, `systemd_units`, `secureboot`, `rpm_package_files`, `deb_package_files`. Bigman already runs `osqueryi` for processes and could add these as optional queries. | Optional dependency. Some tables need root. |
| Neo23x0 auditd ruleset | https://github.com/Neo23x0/auditd | Apache-2.0 | Complete watched-path list with keys (`cron`, `pam`, `systemd`, `udev`, `libpath`, `systemwide_preloads`, `shell_profiles`, `modules`, `sshd`, `actions`). This is the best ready-made list of persistence paths for Bigman's baseline registry, and the license is compatible. | Rules need root and auditd. Bigman would only *offer* them. |
| linux-audit (audit-userspace) | https://github.com/linux-audit/audit-userspace/tree/master/rules | GPL-2.0 | Distro-shipped STIG/OSPP rule fragments for comparison. | GPL-2.0. |
| ComplianceAsCode (SCAP Security Guide) | https://github.com/ComplianceAsCode/content | BSD-3-Clause | Rule IDs and OVAL/bash check logic for every CIS/STIG item cited here, plus `controls/cis_*.yml` mapping CIS section numbers per distro (Ubuntu 22.04/24.04, Debian 12/13, Fedora draft, AlmaLinux 9). The cleanest source for CIS mapping without copying CIS text. | Large. Some checks assume servers. |
| CIS Benchmarks | https://www.cisecurity.org/benchmark/ubuntu_linux | CIS terms of use (non-commercial, share-alike; verify before quoting) | Section numbers and titles for user-facing references. | Do not copy benchmark prose into an AGPL tool. Reference IDs only, via ComplianceAsCode. |
| Falco + rules | https://github.com/falcosecurity/falco · https://github.com/falcosecurity/rules | Apache-2.0 | Rule names and conditions as detection ideas: "Modify Shell Configuration File", "Schedule Cron Jobs", "Adding ssh keys to authorized_keys", "Set Setuid or Setgid bit", "Linux Kernel Module Injection Detected", "Execution from /dev/shm", "Fileless execution via memfd_create". | Runtime eBPF needs root or capabilities. Container-centric. |
| Tracee | https://github.com/aquasecurity/tracee | Apache-2.0 (eBPF parts GPL) | Detector catalogue under `detectors/`: `ld_preload`, `sudoers_modification`, `scheduled_task_modification`, `kernel_module_loading`, `syscall_table_hooking`, `hooked_syscall`, `hidden_kernel_module`. Defines what an optional privileged runtime companion would watch. | Root/eBPF required. |
| Tetragon | https://github.com/cilium/tetragon | Apache-2.0 (BPF GPL) | TracingPolicy examples for file-integrity (`/etc/passwd`, `sudoers`) and module-load enforcement. | Root/eBPF. Kubernetes-oriented docs. |
| SigmaHQ Linux rules | https://github.com/SigmaHQ/sigma/tree/master/rules/linux | Detection Rule License 1.1 | auditd- and process-creation-based Linux persistence rules (cron, systemd, ld.so.preload, authorized_keys). Useful to check that Bigman's rule set is complete. | DRL requires attribution. Log-based, not state-based. |
| Elastic detection-rules | https://github.com/elastic/detection-rules/tree/main/rules/linux | Elastic License 2.0 | Very complete Linux persistence coverage (udev, MOTD, PAM, dracut, git hooks, NetworkManager dispatcher, polkit) as an **idea list** only. | ELv2 is not OSI open source. Do not copy rule text into AGPL Bigman. |
| fwupd HSI | https://fwupd.github.io/libfwupdplugin/hsi.html | LGPL-2.1-or-later (fwupd; unverified here) | `fwupdmgr security` gives a firmware and platform security level for HI-21 without root. | Hardware-dependent. Optional. |

## Papers and long-form references

- Gene H. Kim and Eugene H. Spafford, 1994, "The Design and Implementation of Tripwire: A File System Integrity Checker", ACM CCS '94, https://dl.acm.org/doi/10.1145/191177.191183 (publisher page blocks automated fetch; DOI from memory, verify). The founding FIM paper. Its recommendation to keep the baseline database off-host and signed still applies to Bigman's `baseline.json`.
- Reiner Sailer, Xiaolan Zhang, Trent Jaeger and Leendert van Doorn, 2004, "Design and Implementation of a TCG-based Integrity Measurement Architecture", USENIX Security 2004, https://www.usenix.org/conference/13th-usenix-security-symposium/design-and-implementation-tcg-based-integrity-measurement. The origin of Linux IMA. Measuring at load time beats rescanning disk, a reason to read IMA state (Lynis FINT-4339) rather than reinvent it.
- Yi-Min Wang et al., 2005, "Strider GhostBuster: Why It's a Bad Idea for Stealth Software to Hide Files", Microsoft Research / DSN 2005, https://www.microsoft.com/en-us/research/publication/strider-ghostbuster-a-tool-for-detecting-hidden-files-registry-entries-and-processes/ (page blocks automated fetch; verify). The "cross-view diff" principle behind the proposed static helper and module cross-view checks.
- Pierre Berba, 2021–2022, "Linux Threat Hunting for Persistence" blog series, https://pberba.github.io/security/2021/11/22/linux-threat-hunting-for-persistence-sysmon-auditd-webshell/ (plus initialization-scripts and systemd-generators parts linked above). A defender-oriented map of persistence locations with auditd and osquery hunts. It is close to what Bigman's baseline registry should cover.
- Elastic Security Labs, "Linux Detection Engineering – A primer on persistence mechanisms", https://www.elastic.co/security-labs/primer-on-persistence-mechanisms. A broad catalogue of Linux persistence locations with detection notes. Use it as a coverage checklist.
- Wiz, 2023, "Linux rootkits explained – Part 1: Dynamic linker hijacking", https://www.wiz.io/blog/linux-rootkits-explained-part-1-dynamic-linker-hijacking. Explains why preload rootkits defeat libc-based tools and what defenders check.
- Max Kellermann, 2022, "The Dirty Pipe Vulnerability", https://dirtypipe.cm4all.com/. The discoverer's account of a page-cache write bug found through file corruption. Relevant to HI-23's note on page-cache bugs versus FIM.
- Qualys Threat Research advisories (2021–2025), the most readable primary sources for the userspace LPEs in the checklist:
  - PwnKit: https://www.qualys.com/2022/01/25/cve-2021-4034/pwnkit.txt
  - Baron Samedit: https://www.qualys.com/2021/01/26/cve-2021-3156/baron-samedit-heap-based-overflow-sudo.txt
  - Sequoia: https://www.qualys.com/2021/07/20/cve-2021-33909/sequoia-local-privilege-escalation-linux.txt
  - Looney Tunables: https://www.qualys.com/2023/10/03/cve-2023-4911/looney-tunables-local-privilege-escalation-glibc-ld-so.txt
  - regreSSHion: https://www.qualys.com/2024/07/01/cve-2024-6387/regresshion.txt
  - needrestart: https://www.qualys.com/2024/11/19/needrestart/needrestart.txt
  - PAM/udisks: https://www.qualys.com/2025/06/17/suse15-pam-udisks-lpe.txt
- sudo project, 2025, "Local Privilege Escalation via chroot option", https://www.sudo.ws/security/advisories/chroot_bug/, and "host option", https://www.sudo.ws/security/advisories/host_any/. Vendor advisories with exact affected ranges. Discoverer write-up (Stratascale): https://www.stratascale.com/resource/cve-2025-32463-sudo-chroot-elevation-of-privilege/
- Xint, 2026, "Copy Fail" (CVE-2026-31431), https://xint.io/blog/copy-fail-linux-distributions. Affects distributions shipped since 2017. Interim mitigation is to disable `algif_aead`. A 2026 example of a module-scoped mitigation that Bigman can check read-only.
- ESET, 2024, "Bootkitty: Analyzing the first UEFI bootkit for Linux", https://www.welivesecurity.com/en/eset-research/bootkitty-analyzing-first-uefi-bootkit-linux/. Secure Boot state and enrolled keys are the deciding factors.
- Stroz Friedberg / LevelBlue, 2024-08-19, "Unveiling sedexp", https://levelblue.com/blogs/strozfriedberg/unveiling-sedexp. udev-rule persistence went unnoticed for about two years. Supports adding udev rule dirs to the baseline.
- Nextron Systems, 2025-08-01, "Plague: A Newly Discovered PAM-Based Backdoor for Linux", https://www.nextron-systems.com/2025/08/01/plague-a-newly-discovered-pam-based-backdoor-for-linux/. The sample had zero AV detections. Package-ownership checks on PAM modules catch this class without signatures.
- Elastic Security Labs, 2024, "Declawing PUMAKIT", https://www.elastic.co/security-labs/declawing-pumakit. A modern LKM rootkit. Taint, signer and cross-view module checks are the realistic rootless signals.
- CISA, Known Exploited Vulnerabilities catalog (JSON feed), https://www.cisa.gov/known-exploited-vulnerabilities-catalog. Machine-readable source for the LPE checklist and for feed-driven "patch this" advisories.

## Open questions for the maintainer

1. **Privileged read-only collector.** Sudoers, shadow status, polkit `/etc` rules, sshd effective config, cron/at spools and root's `authorized_keys` are unreadable without root. Should Bigman add an opt-in, polkit-authorized, fixed-argument helper? It would follow the pattern of `bigman setup … --apply`. It would return only hashes, metadata and parsed summaries, never secrets or password hashes, and its results would be kept in a separate report so rootless baselines are never blocked by unreadable files. Or should these items stay "Gap, run Lynis/osquery as root"?
2. **Baseline schema change.** Adding mode, uid, gid, xattr presence, symlink-target content hash and package owner to `Entry` changes `baseline.json`. Migrate silently (re-baseline with a one-time "schema upgraded" Info report), or require user review?
3. **Noise from package updates.** Should a change be shown at lower severity, or auto-accepted, when the file is package-owned and `rpm -V`/`dpkg --verify` says it matches the package after an update? Auto-acceptance contradicts "baseline is not trust". Showing it at Info keeps the principle but adds volume.
4. **Distro scope.** Fedora, Debian and Ubuntu are supported today. Is Arch (`pacman -Qkk`, `arch-audit`) or openSUSE (`zypper lp -g security`) in scope for v0.3?
5. **Static helper.** Is a small statically linked (musl) helper for cross-view reads acceptable in the install footprint and the reproducible-build story?
6. **Feed data for HI-23.** Should the KEV-derived Linux CVE list ship inside Bigman (signed, versioned and updated with releases) or be fetched by the feeds component? This decision is shared with [vulnerability-intelligence-and-feeds](../research/vulnerability-intelligence-and-feeds.md).
7. **Allowlists of known installer lines** (nvm, conda, rustup) in shell rc files: ship them, or leave them entirely to the user's review?
8. **License hygiene.** Confirm the policy "reimplement GPL-2.0-only checks, don't copy code or rule text", and that path lists taken from the Apache-2.0 Neo23x0 ruleset may be imported with attribution.
