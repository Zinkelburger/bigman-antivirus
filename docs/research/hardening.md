# Hardening suggestions: research and reasoning

> Why each `harden.*` rule exists, when it fires, what its change does and costs, and where the facts come from. Researched on 2026-11-02 from primary sources (kernel documentation, upstream repositories, distribution packaging sources, man pages) and rootless probes on a Fedora 44 Workstation (kernel 7.2.9, systemd 259, firewalld 2.4.4, OpenSSH 10.2p1). The rule texts are in [rules.md](../rules.md); the threats they address are in [docs/threats](../threats/README.md).

## What a suggestion is

`bigman harden` and **Protect → Hardening** list settings on this computer that you could change to close a way in. Each suggestion carries:

1. **Evidence that it applies here**: the unit state, sysctl file, config line or listening socket Bigman read, never a generic checklist item.
2. **Why** it matters, and **what it costs** you: what stops working, in plain words.
3. **A fix**, which is one of:
   - a change Bigman makes: previewed exactly (the commands and file contents), run as you or through one polkit prompt, checked afterwards, and undoable later with its own preview;
   - steps to follow by hand, when Bigman cannot make the change safely (firmware, the installer, display-manager files); or
   - a refusal with the reason, when the change would cut you off or would not last.

You can also mark a suggestion **keep as it is**. It then stops counting towards the status and is listed separately, and `--suggest-again` brings it back. Nothing changes until you confirm a preview.

The daemon rebuilds each change from the suggestion ID at the moment you confirm, from a fresh read of the system. The app and CLI never send commands, unit names or paths. If the setting changed in the meantime, the suggestion no longer exists and the request fails with "This suggestion no longer applies".

### Severity

Severity follows exposure on this computer, not the setting in the abstract:

- **Network services** are Review when another computer can reach the port through the firewall (or no firewall runs), Info when the firewall blocks it. Reach comes from the active firewalld zones, read over D-Bus without root: interfaces and sources, allowed services and ports.
- **The SSH server** is also Info when someone logged in over the network in the logs Bigman can read. That is evidence you use it.
- **Autologin** is Review when the disk is also unencrypted, because then switching the computer on is enough.
- **The home folder** is Review when other people's accounts exist (UID 1000 to 59999 with a login shell), Info otherwise.

Nothing is High. These are settings, not detections.

### Reading without root

Every input is read as you: `systemctl show` for unit state, firewalld's D-Bus interface (its read methods are `allow_any` in Fedora's polkit policy, while `/etc/firewalld` is not readable), `/proc/sys` and the `sysctl.d` directories, `resolvectl status`, gsettings and `kscreenlockerrc`, display-manager config files (mode 0644), `lsblk -J`, the `SecureBoot` EFI variable (mode 0644), `/etc/group` and `/etc/passwd`, `npm config get`, and the journal and `last` for network logins. Anything Bigman could not read is listed under "Not checked", with the reason, and makes the result **incomplete**. It is never reported as fine.

## The rules

| Rule | Default | The change Bigman offers | Undo | Threats |
|---|---|---|---|---|
| `harden.sshd` | Review / Info | `systemctl disable --now` the SSH units that are on (admin) | `enable --now` the same units | NE-01, NE-02, NE-03, NE-05, NE-20 |
| `harden.service.cups_browsed` | Review / Info | `systemctl disable --now cups-browsed.service` (admin) | `enable --now` | NE-17 |
| `harden.service.network` | Review / Info | Turn off rpcbind, Samba, vsftpd, telnet, xrdp or Cockpit (admin) | `enable --now` | NE-01, NE-22 |
| `harden.firewall.off` | Review / Info | `systemctl enable --now firewalld`, or `ufw --force enable` (admin); steps to install one if neither is installed | `disable --now`, or `ufw disable` | NE-12, NE-01 |
| `harden.firewall.open_range` | Review / Info | `firewall-cmd --permanent --zone=Z --remove-port=…` and `--reload` (admin) | `--add-port=` the same ranges | NE-12, NE-01, NE-11 |
| `harden.sysctl.ptrace_scope` | Review | Write `/etc/sysctl.d/90-bigman-ptrace-scope.conf` with `1`, `sysctl -w` (admin) | Remove the file, `sysctl -w` the old value | CR-07, HI-25 |
| `harden.sysctl.kptr_restrict` | Info | The same, `90-bigman-kptr-restrict.conf` with `1` | The same | HI-25 |
| `harden.sysctl.dmesg_restrict` | Info | The same, `90-bigman-dmesg-restrict.conf` with `1` | The same | HI-25 |
| `harden.name.llmnr` | Info | Write `/etc/systemd/resolved.conf.d/90-bigman-llmnr.conf` with `LLMNR=no`, restart systemd-resolved (admin) | Remove the file, restart | NE-16 |
| `harden.screen_lock` | Review | `gsettings set` or `kwriteconfig6` for only the keys that are off (as you) | Put back each old value, or reset it to the default | DS-15 |
| `harden.autologin` | Review / Info | Steps: Settings → Users, the SDDM login-screen settings, or the line to delete | — | DS-15 |
| `harden.disk_encryption` | Info | Steps: back up and reinstall with encryption | — | DS-22 |
| `harden.secure_boot` | Info | Steps: enroll out-of-tree module keys, then turn it on in the firmware | — | HI-21, DP-23 |
| `harden.group.docker` | Review | `gpasswd -d USER docker` (admin; applies at next login) | `usermod -aG docker USER` | HI-16 |
| `harden.home.readable` | Review / Info | `chmod` that removes access for others, keeping the group bits (as you) | `chmod` back to the previous mode | CR-10 |
| `harden.npm.ignore_scripts` | Info | `npm config set ignore-scripts true --location=user` (as you) | `npm config delete ignore-scripts --location=user` | SC-01 |

Admin changes run as one `pkexec` call per change, so polkit asks once. Changes as you run directly. After a change, Bigman reads the setting again and says whether it took. A change that only applies from the next login, like leaving a group, says so.

## Network services

### The SSH server (`harden.sshd`)

An SSH server is often turned on once and forgotten on a desktop. It is the way in that the tampered editor extension of October 2026 used: it added a key to `authorized_keys` and opened `ssh -R` back to `localhost:22`, so the attacker arrived over loopback, which a host firewall does not filter. Turning sshd off is what breaks that chain. The [SSH access watch](../../README.md#implemented-workflows) catches the other steps.

Defaults by distribution:

- **Fedora Workstation** ships openssh-server but disables `sshd.service` and `sshd.socket` in `81-desktop.preset`; Fedora's other editions enable it through `90-default.preset`. The FedoraWorkstation firewall zone still allows the `ssh` service. https://src.fedoraproject.org/rpms/fedora-release/blob/f44/f/81-desktop.preset
- **Ubuntu Desktop** does not install openssh-server: only `openssh-client` is in the `standard` seed. https://git.launchpad.net/~ubuntu-core-dev/ubuntu-seeds/+git/platform/plain/standard?h=noble . When installed on Ubuntu 22.10 or later, `ssh.socket` listens and starts `ssh.service` on the first connection, so both units must be disabled or the socket starts the service again. https://discourse.ubuntu.com/t/sshd-now-uses-socket-based-activation-ubuntu-22-10-and-later/30189
- **Debian** names the units `ssh.service` and `ssh.socket` and enables the service on install. https://salsa.debian.org/ssh-team/openssh/-/tree/master/debian/systemd
- **Arch** ships `sshd.service` and `sshd@.service` and no socket unit, and enables nothing. https://gitlab.archlinux.org/archlinux/packaging/packages/openssh/-/blob/main/PKGBUILD

So Bigman looks at all four unit names and disables every one that is enabled or active, and the undo enables the same set.

**Does stopping sshd end your current session?** No. All three distributions' units set `KillMode=process`, which kills only the main process (https://www.freedesktop.org/software/systemd/man/latest/systemd.kill.html), and `pam_systemd` moves each login into its own `session-N.scope`, outside the service's cgroup (https://www.freedesktop.org/software/systemd/man/latest/pam_systemd.html). New connections are refused, though, so a user who is logged in over the network could lock themselves out at the next disconnect. Bigman therefore refuses to turn sshd off while `loginctl` shows a remote session, and says to do it from the computer's own keyboard.

**Login history.** To tell "on and unused" from "on and needed", Bigman reads network logins of the last 90 days without root: `Accepted … for USER from ADDR` from sshd in the system journal when you can read it (members of `wheel`, `adm` or `systemd-journal`), and `last` (wtmp). On Fedora 44, `/var/log/wtmp` is mode 0664 and `last` works as a user; Fedora 43 replaced lastlog with lastlog2 (https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2). Debian 13 dropped `last` from util-linux in favour of wtmpdb (https://www.debian.org/releases/trixie/release-notes/issues.en.html), and openSUSE removed utmp and wtmp entirely (https://microos.opensuse.org/blog/2023-11-06-utmp-and-wtmp-are-gone). Bigman says which logs it read and how far back they go, and says so when it could read none.

### cups-browsed (`harden.service.cups_browsed`)

CVE-2024-47176: cups-browsed up to 2.0.1 listened on UDP 631 on every address and trusted any packet. Chained with CVE-2024-47076, -47175 and -47177, a packet from the network could add a printer that runs a command as `lp` when you print to it. https://github.com/OpenPrinting/cups-browsed/security/advisories/GHSA-rj88-6mr5-rcw8 , https://nvd.nist.gov/vuln/detail/CVE-2024-47176 . cups-browsed 2.1.0 removed legacy CUPS browsing (https://github.com/OpenPrinting/cups-browsed/releases), and distributions backported the fixes (Ubuntu: https://ubuntu.com/security/notices/USN-7041-1).

It is still worth turning off when unused, and the rule rates it by reachability. Fedora 44 installs it disabled. On Ubuntu and Debian, `cups-daemon` recommends it and it runs by default (https://packages.ubuntu.com/noble/cups-daemon). Turning it off stops permanent queues being created for network printers. Most print dialogs still find driverless (IPP Everywhere, AirPrint) printers through CUPS's temporary queues; some older programs that list only permanent queues will not. https://github.com/OpenPrinting/cups-browsed

### Other network daemons (`harden.service.network`)

One suggestion per enabled daemon, each with its own ID (`harden.service.network:rpcbind`, `:samba`, `:vsftpd`, `:telnet`, `:xrdp`, `:cockpit`):

- **rpcbind** is needed only for NFS version 3 and NIS. Fedora 44 installs it with nfs-utils but leaves it off.
- **Samba** (`smb`/`nmb` on Fedora, `smbd`/`nmbd` on Debian) shares folders from this computer. Opening shares on other computers does not need it.
- **vsftpd** and **telnet** send passwords unencrypted.
- **xrdp** serves remote desktops on 3389. GNOME's own remote desktop is off until you enable it in Settings and is not flagged.
- **Cockpit** is enabled only on Fedora Server (`80-server.preset`). https://src.fedoraproject.org/rpms/fedora-release/blob/f44/f/80-server.preset

Avahi was considered and left out: it is enabled on Fedora and installed on Ubuntu Desktop, and turning it off breaks `.local` names, printer and scanner discovery and network share browsing. That cost is too high for a default suggestion.

## The firewall

### No firewall (`harden.firewall.off`)

Neither firewalld nor ufw is active. Review when something listens beyond loopback, Info when nothing does yet. When firewalld or ufw is installed, Bigman offers to turn it on. Otherwise it gives the steps to install one.

ufw's status needs root (`ufw status` refuses), and its rule files are installed mode 0640 (https://git.launchpad.net/ufw/tree/setup.py). So for ufw Bigman reports only that it is on, and says its rules are readable only by an administrator. Ubuntu installs ufw but leaves it disabled. https://ubuntu.com/server/docs/how-to/security/firewalls/

### Open port ranges (`harden.firewall.open_range`)

Fedora Workstation's default zone allows `1025-65535/tcp` and `1025-65535/udp`. Its description says unsolicited packets are rejected "from port 1 to 1024, except for select network services". https://github.com/firewalld/firewalld/blob/main/config/zones/FedoraWorkstation.xml . So a development server, a debugger port or a program listening on a high port by mistake is reachable from the whole network.

The rule fires for any active zone (one with interfaces or sources) that opens a range of 1,000 or more ports above 1024. It is Review when a listening program is reachable only because of that range, and Info when nothing uses it. The change removes exactly those ranges from that zone, permanently, and reloads; the undo adds them back. What stops working: KDE Connect and GSConnect (1714-1764), Spotify Connect, Steam Remote Play, LAN game servers and development servers reached from a phone. The suggestion includes the command to allow one port again.

## Kernel settings

All three sysctls are read from `/proc/sys`. Bigman also lists every file in `/usr/lib/sysctl.d`, `/usr/local/lib/sysctl.d`, `/run/sysctl.d` and `/etc/sysctl.d` (plus `/etc/sysctl.conf`) that sets the key, and the package that installed it. The change writes a `90-bigman-*.conf` drop-in and applies it with `sysctl -w`. Because systemd-sysctl applies files in name order and a later name wins, Bigman refuses the change, naming the file, when a file sorting after `90-bigman-…` sets a different value. That change would not survive a reboot.

### ptrace_scope (`harden.sysctl.ptrace_scope`)

At 0, any process can attach to any other process of the same user, and read its memory: browser cookies, an unlocked password manager, keys held by ssh-agent. At 1, a process can attach only to its own descendants, unless the target opts in with `PR_SET_PTRACER`, as the crash handlers of KDE, Chromium and Firefox do. https://www.kernel.org/doc/html/latest/admin-guide/LSM/Yama.html

| Distribution | Default | How |
|---|---|---|
| Fedora 44 | 0 | `elfutils-default-yama-scope` installs `/usr/lib/sysctl.d/10-default-yama-scope.conf`. https://src.fedoraproject.org/rpms/elfutils |
| Debian 12 and 13 | 0 | A kernel patch, `yama-disable-by-default.patch`. A merge request to drop it was open, unmerged, in 2025. https://salsa.debian.org/kernel-team/linux/-/raw/debian/latest/debian/patches/debian/yama-disable-by-default.patch , https://salsa.debian.org/kernel-team/linux/-/merge_requests/1507 |
| Ubuntu | 1 | procps ships `10-ptrace.conf`. https://git.launchpad.net/ubuntu/+source/procps/plain/debian/sysctl.d/10-ptrace.conf?h=ubuntu/noble |
| Arch | 1 | The kernel default. https://wiki.archlinux.org/title/Security#ptrace_scope |

Yama acts only on attach-mode access. Value 1 blocks `ptrace` attach, `/proc/PID/mem` and `process_vm_readv`/`writev`, but not `/proc/PID/environ` or `cmdline`, which any same-user process can still read. Environment-variable secrets need a different fix (CR-07).

What 1 costs: attaching to a program that is already running (`gdb -p`, `strace -p`, `ltrace -p`, `reptyr`, `gcore`, `py-spy --pid`, an IDE's "attach to process") needs sudo. Starting a program under a debugger keeps working. Bigman suggests 1 and never 2 or 3: 2 and 3 also break `gdb ./prog`, and 3 cannot be lowered until reboot. KSPP recommends 3, with 1 as the fallback "if you need ptrace to work". https://kspp.github.io/Recommended_Settings

### kptr_restrict and dmesg_restrict

`kernel.kptr_restrict` at 0 shows hashed `%pK` addresses; at 1 they read as zeros without `CAP_SYSLOG`. https://docs.kernel.org/admin-guide/sysctl/kernel.html . Ubuntu sets 1 (https://git.launchpad.net/ubuntu/+source/procps/plain/debian/sysctl.d/10-kernel-hardening.conf?h=ubuntu/noble). Fedora, Debian and Arch leave 0. The cost: perf and BPF tools resolve kernel symbols only as root. The Arch wiki notes the gain is limited on distribution kernels, whose symbols are public (https://wiki.archlinux.org/title/Security), so the rule is Info. Bigman suggests 1, not KSPP's 2.

`kernel.dmesg_restrict` is already 1 on Fedora, Debian, Ubuntu and Arch through `CONFIG_SECURITY_DMESG_RESTRICT` (Arch: https://gitlab.archlinux.org/archlinux/packaging/packages/linux/-/raw/main/config.x86_64 ; Debian: https://salsa.debian.org/kernel-team/linux/-/raw/debian/latest/debian/config/config), so the rule only fires where someone set it to 0. `journalctl -k` keeps working for members of `wheel` or `adm`.

## Name lookups: LLMNR (`harden.name.llmnr`)

When DNS cannot resolve a single-label name, LLMNR asks the whole local network, and any device can answer. On shared Wi-Fi this is used to capture authentication attempts and redirect connections (MITRE ATT&CK T1557.001).

resolved.conf takes `LLMNR=yes`, `no` or `resolve`, where `resolve` sends queries but does not answer them. https://man7.org/linux/man-pages/man5/resolved.conf.5.html

| Distribution | Build default | Source |
|---|---|---|
| Upstream systemd | `yes` | https://github.com/systemd/systemd/blob/main/meson_options.txt |
| Fedora | `resolve`: queries, never answers | https://src.fedoraproject.org/rpms/systemd/blob/f44/f/systemd.spec |
| Ubuntu | `no` | https://git.launchpad.net/ubuntu/+source/systemd/plain/debian/rules?h=ubuntu/noble |
| Debian | upstream `yes`, but resolved is not installed by default since bookworm | https://salsa.debian.org/systemd-team/systemd/-/raw/debian/master/debian/rules , https://www.debian.org/releases/bookworm/amd64/release-notes/ch-information.en.html |
| Arch | upstream `yes` when resolved is enabled (it is not by default) | https://gitlab.archlinux.org/archlinux/packaging/packages/systemd/-/raw/main/PKGBUILD |

Fedora's `resolve` mode still sends the queries, which is the half a poisoner needs, so the rule fires on any mode other than `no` while systemd-resolved runs. A request to turn it off on Fedora was closed at end of life without a change. https://bugzilla.redhat.com/show_bug.cgi?id=2228188 , https://github.com/systemd/systemd/pull/28263

The change is one drop-in, `[Resolve]` `LLMNR=no`, followed by a restart of systemd-resolved. Since resolved enables LLMNR on a link only when both the global and the per-link setting allow it, the global setting covers NetworkManager's per-connection values too. Reload works only from systemd 256, so Bigman restarts the service, which works everywhere. The cost: short names of devices on a network without a DNS server (`nas`, `printer`) stop resolving. `.local` names (mDNS) and DNS are unaffected.

## The desktop session

### Screen lock (`harden.screen_lock`)

- **GNOME:** `org.gnome.desktop.screensaver lock-enabled` (default true) and `org.gnome.desktop.session idle-delay` (default 300 seconds, 0 = never idle, so never locks). On Ubuntu, `org.gnome.desktop.screensaver ubuntu-lock-on-suspend` is read too; elsewhere a missing key counts as locking on resume. https://help.gnome.org/admin/system-admin-guide/stable/desktop-lockscreen.html.en
- **KDE Plasma 6:** `~/.config/kscreenlockerrc`, group `[Daemon]`: `Autolock` (default true), `Timeout` in minutes (default 5) and `LockOnResume` (default true). A missing key means the default, because KConfig writes only non-default values. https://invent.kde.org/plasma/kscreenlocker/-/blob/master/settings/kscreenlockersettings.kcfg

The rule fires when automatic locking is off, the delay is 0, or the screen does not lock on resume. The change touches only those keys, sets a five-minute delay when the delay was 0, and tells KDE's screen locker to reload. The undo writes back each previous value, or removes the key when it was at its default. Administrators can lock GNOME keys through dconf (`/etc/dconf/db/*.d/locks`). Bigman asks `gsettings writable` first, and when a key it would change is locked it says so instead of offering a change that would fail.

### Autologin (`harden.autologin`)

Read from `/etc/gdm/custom.conf` (Fedora), `/etc/gdm3/custom.conf` (Ubuntu) and `/etc/gdm3/daemon.conf` (Debian) (`AutomaticLoginEnable`, `AutomaticLogin`); from SDDM's `/usr/lib/sddm/sddm.conf.d`, `/etc/sddm.conf.d` and `/etc/sddm.conf` (`[Autologin] User=`) (https://github.com/sddm/sddm/blob/develop/data/man/sddm.conf.rst.in); from Plasma Login's equivalent files; and from LightDM's `autologin-user`. Bigman asks systemd which display manager runs and reads only its files. The files are readable without root. Turning autologin off belongs in the desktop's settings (GNOME Settings → Users, Plasma's login-screen settings), which write the same files through their own polkit actions, so Bigman gives those steps rather than editing display-manager files itself.

### Disk encryption (`harden.disk_encryption`)

`lsblk -J -o NAME,TYPE,FSTYPE,MOUNTPOINTS` works without root. Bigman finds the devices mounted at `/` and `/home` and walks up to their parents: an ancestor of TYPE `crypt` means dm-crypt (LUKS, including LVM on LUKS). https://man7.org/linux/man-pages/man8/lsblk.8.html . Encryption is chosen when installing, so the fix is manual: back up and reinstall with encryption on. Swap partitions, systemd-homed and fscrypt homes are not examined yet (see [DS-22](../threats/08-desktop-and-sandboxing.md#ds-22-unencrypted-disk-on-a-portable-or-shared-computer)).

### Secure Boot (`harden.secure_boot`)

`/sys/firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c` is world-readable. Its first four bytes are the variable's attributes and the fifth is the value: 1 is on, 0 is off. https://www.kernel.org/doc/html/latest/filesystems/efivarfs.html . A computer without `/sys/firmware/efi` booted in legacy BIOS mode, and the rule does not apply. Turning Secure Boot on is a firmware setting, so the fix is manual. It starts with enrolling the signing key of any out-of-tree kernel module (NVIDIA from a third-party repository, VirtualBox), because those stop loading otherwise.

## Accounts and files

### The docker group (`harden.group.docker`)

"The `docker` group grants root-level privileges to the user." https://docs.docker.com/engine/install/linux-postinstall/ . Any program you run can start a container that mounts `/`, without a prompt. The alternatives keep containers without root: rootless Docker (https://docs.docker.com/engine/security/rootless/) and Podman, which is rootless by design (https://github.com/containers/podman/blob/main/docs/tutorials/rootless_tutorial.md). Leaving the group takes effect at the next login, and the result says so.

### Home folder mode (`harden.home.readable`)

| Distribution | New home folders | Source |
|---|---|---|
| Fedora | 0700 | `HOME_MODE 0700` in `/etc/login.defs` |
| Debian 12 and 13 | 0700 (adduser) | https://sources.debian.org/src/adduser/3.152/adduser.conf/ |
| Ubuntu 21.04 and later | 0750 | https://discourse.ubuntu.com/t/private-home-directories-for-ubuntu-21-04-onwards/19533 |
| Arch | 0700 | `HOME_MODE 0700` in Arch's packaged `login.defs`, https://gitlab.archlinux.org/archlinux/packaging/packages/shadow |

Upgrades keep the old mode, so home folders created before 2021 on Ubuntu, or before bookworm on Debian, are often 0755. The rule fires when "others" have any access. The change clears only those bits, keeping the group bits, so 0755 becomes 0750 and 0710 stays as it is. The cost: a `~/public_html` site or another account's access to shared folders stops working.

### npm install scripts (`harden.npm.ignore_scripts`)

`ignore-scripts=true` stops npm from running dependency `preinstall`, `install` and `postinstall` scripts. Scripts you run explicitly with `npm run` still run, without their pre- and post-scripts. https://docs.npmjs.com/cli/v11/using-npm/config#ignore-scripts . The self-spreading npm worms of 2025 ran from these scripts (SC-01).

npm 12.0.0 (2026-07-08) blocks dependency lifecycle scripts by default unless the root package's `allowScripts` allows them; approvals are recorded with `npm install-scripts approve`, followed by `npm rebuild`. https://github.com/npm/cli/releases/tag/v12.0.0 , https://docs.npmjs.com/cli/v12/commands/npm-install-scripts . pnpm 10 made the same change (https://github.com/pnpm/pnpm/releases/tag/v10.0.0), and pnpm 11 replaced its settings with `allowBuilds` (https://pnpm.io/settings/build). So the rule fires only for npm older than 12 and is Info. The cost: packages that build or download something at install time (native add-ons, sharp, Prisma, Puppeteer's browser download, git hooks set up by `prepare`) need `npm rebuild <package>` after you review them.

## Researched and left out

- **ptrace_scope 2 or 3, kptr_restrict 2:** they break ordinary debugging or profiling, for little extra protection on a single-user desktop.
- **Avahi and mDNS:** discovery of printers, scanners, shares and `.local` names depends on them.
- **Automatic updates:** GNOME Software already downloads updates by default on Fedora; Ubuntu enables unattended-upgrades for security updates (https://ubuntu.com/server/docs/how-to/software/automatic-updates/); Debian's installer leaves it to a question whose default is none (https://salsa.debian.org/installer-team/pkgsel/-/blob/master/debian/pkgsel.templates); Arch has none by design. The advisories tab covers what is pending instead.
- **sshd configuration** (password logins, root login, forwarding): `sshd -T` needs root, so it belongs to the optional helper (NE-02).

## Corrections made to the threat docs

The research found three errors, now fixed:

- `ptrace_scope` is 0 by default on Fedora and Debian, not 1 (CR-07, HI-25). CR-07 also now says that Yama does not protect `/proc/PID/environ`.
- Fedora runs LLMNR in `resolve` mode (it sends queries but does not answer), and Ubuntu turns it off. The old text said it was on "on many distros" (NE-16).
- npm 12's approval command is `npm install-scripts approve`, not `npm approve-scripts` (SC-01).
