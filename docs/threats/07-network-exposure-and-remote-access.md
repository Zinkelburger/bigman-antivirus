# Network exposure and remote access

> Scope: what a Linux desktop, developer workstation or small self-hosted server exposes to the network, and how remote access to it can be abused. This covers listening services and wildcard binds, SSH configuration, patch state and brute-force noise, developer and AI servers left reachable, Docker's interaction with host firewalls, firewall state, DNS and name-resolution tampering, LAN discovery protocols (LLMNR, mDNS, cups-browsed, UPnP), VPN/WireGuard hygiene, tunnels, and outbound or reverse-shell-like connections as a detection category. It also distils blue-team lessons from the Collegiate Cyber Defense Competition (CCDC). Red-team write-ups were read only to work out what a defender should audit first, and nothing offensive is reproduced here. Package and repository attacks are in [supply-chain](./02-supply-chain-packages.md) and [distro packaging](./03-distro-packaging-and-install-channels.md), persistence and account auditing in [host integrity](./04-host-integrity-and-hardening-audit.md), and key and token theft in [credentials](./06-credential-and-secret-protection.md). Last researched: 2026-10-29.

## Summary table

| ID | Attack | Severity for a Linux desktop/dev user | Bigman today | Proposed |
|---|---|---|---|---|
| NE-01 | Unexpected listening services and wildcard (0.0.0.0 / [::]) binds | High | Partial | Parse `ss`/`/proc/net/*` into findings with owner UID, unit, binary path and owning package; flag wildcard binds |
| NE-02 | Weak SSH server configuration (password auth, root login, forwarding) | High | Gap | `ssh.config.*` audit of effective `sshd -T` output via an explicit, one-shot polkit read |
| NE-03 | SSH brute force, password spraying and credential stuffing | Medium (High if password auth on) | Gap | Rootless journald summary of failed and accepted logins, grouped by source and user |
| NE-04 | Vulnerable SSH daemons (regreSSHion, 2025 OpenSSH CVEs, Terrapin, Erlang/OTP SSH) | High | Partial | Feed-driven advisory keyed on the installed package version and the distro's fixed version |
| NE-05 | Unrecognised SSH keys and risky forwarding and agent settings | High | Partial | Already baselines `~/.ssh/authorized_keys`; add `AuthorizedKeysFile` and `sshd_config.d` drift and a forwarding audit |
| NE-06 | Docker Engine API exposed on TCP (2375/2376) | Critical | Partial | `docker.api.tcp` from listeners, daemon flags, `daemon.json` and unit drop-ins |
| NE-07 | Docker/Podman published ports bypassing ufw/firewalld | High | Gap | `docker.publish.wildcard` from `docker-proxy` or `rootlessport` command lines plus a firewall-backend explanation |
| NE-08 | Unauthenticated data stores (Redis, Elasticsearch, MongoDB, PostgreSQL trust, MySQL) | High | Gap | Port and owner-package match plus a read-only config lint (`bind`, `protected-mode`, `requirepass`, `xpack.security`) |
| NE-09 | Exposed notebooks and ML frameworks (Jupyter, Ray dashboard) | High | Partial | Flag the known ports when bound beyond loopback; read Jupyter config for empty token and password |
| NE-10 | Local AI model servers bound publicly (Ollama, LM Studio, vLLM, ComfyUI) | High | Partial | Flag 11434 and similar ports on non-loopback; check the `OLLAMA_HOST` unit environment |
| NE-11 | Debug ports and dev servers reachable from the LAN (Node inspector, Chrome DevTools, Vite/webpack, Flask debug) | High | Partial | Port and process-argument rules for `--inspect=0.0.0.0`, `--remote-debugging-port`, `--host 0.0.0.0` |
| NE-12 | Host firewall off, permissive or tampered with | High | Partial | Turn the raw `systemctl is-active` dump into `firewall.*` findings: backend, default zone, open ranges, and changes since baseline |
| NE-13 | Outbound connections to unexpected hosts (C2, mining pools, blocklisted ranges) | High | Partial | Rootless established-connection snapshot matched against user-imported DROP or FireHOL level 1 lists and a mining-pool port list |
| NE-14 | Reverse-shell-like process behaviour (a shell whose stdio is a socket) | Critical | Partial (`shell.network-listener` static rule, `trace.network` in the sandbox) | `net.shell-socket-stdio` for processes the user can inspect, with an optional root or auditd mode |
| NE-15 | DNS tampering (`/etc/hosts`, `resolv.conf`, NetworkManager and resolved DNS) | High | Gap | Add these files to the `bigman changes` baseline; diff the effective DNS servers from `resolvectl` |
| NE-16 | LLMNR, mDNS and NetBIOS name poisoning on untrusted LANs | Medium | Gap | Report LLMNR and mDNS state from resolved and NetworkManager; flag UDP 5355 and 5353 listeners |
| NE-17 | cups-browsed IPP chain (CVE-2024-47176 and related) and CUPS DDoS reflection | High (if active) | Gap | `cups.browsed-active` plus `BrowseRemoteProtocols` lint, cross-checked with UDP 631 |
| NE-18 | Router UPnP/NAT-PMP port mappings and router-level exposure | Medium | Gap | Guidance view, plus an optional read-only UPnP IGD port-mapping listing behind an explicit user action |
| NE-19 | VPN/WireGuard configuration hygiene (keys, hooks, AllowedIPs, leaks) | Medium | Gap | Metadata-only audit: file modes, presence of `PostUp`/`PreUp` hooks, `SaveConfig`, full-tunnel and DNS settings |
| NE-20 | Unexpected tunnels and remote-access agents (ngrok, cloudflared, Tailscale Funnel, VNC/RDP) | High | Gap | Known-binary and known-unit inventory plus outbound tunnel endpoints, as "review" findings |
| NE-21 | Default, reused or unchanged service credentials (CCDC's top lesson) | High | Gap | Guidance-first checklist; detect the presence of known default-credential configs without reading secrets |
| NE-22 | Legacy RPC, file-sharing and remote-desktop daemons (rpcbind, NFS, Samba, Telnet, VNC) on a desktop | Medium | Partial | Named-service rules in the listener review: "not normally needed on a workstation" |

## Attacks

### NE-01 Unexpected listening services and wildcard binds
- **What it is:** Any socket bound to `0.0.0.0`, `[::]` or `*` accepts connections on every interface: Wi-Fi at a café, a VPN, Docker bridges. Developer tools, language runtimes and desktop apps often bind to all interfaces for convenience, and internet-wide scanners find a new listener within minutes or hours. Every CCDC write-up puts "know what is listening and why" first, and an unexplained port is the most common early sign of compromise or misconfiguration. On Fedora Workstation the default firewalld zone allows inbound TCP and UDP 1025-65535, so a wildcard listener on a high port is reachable from the LAN unless the user changed the zone. The zone file `FedoraWorkstation.xml` shipped in `firewalld-2.4.4-1.fc44` contains `<port protocol="tcp" port="1025-65535"/>`. Ubuntu ships `ufw` disabled by default.
- **Real-world examples:** The CCDC 2026 red-team guidance for blue teams says to "review listening ports regularly" and to question any open port that is not a scored service (WinterKnight, 2026-03-31). Shodan-based studies of Ollama (2025), Ray (2025) and Docker (2025) in NE-06 to NE-10 all start from a wildcard bind.
- **What a defender inspects (indicators):**
  - `ss -H -tulpn` (process names appear only for your own sockets unless run as root), `ss -H -tuln`, and `/proc/net/tcp`, `/proc/net/tcp6`, `/proc/net/udp` and `/proc/net/udp6`. These files are world-readable and include the socket owner UID and inode.
  - `systemctl list-sockets --all` maps socket-activated listeners to units without root.
  - `/proc/PID/cgroup` (unit name), `/proc/PID/cmdline` and `/proc/PID/exe` (exe is readable only for your own processes).
  - Owning package: `rpm -qf PATH` or `dpkg -S PATH`. A binary owned by no package, or living in `/tmp`, `/dev/shm`, `~/.cache` or `~/.local/share`, is a review item.
  - Suspicious states: a wildcard bind by a process with no package owner, a listener that appeared since the last baseline, or a well-known dev or database port (see NE-06 to NE-11) on a non-loopback address.
- **Bigman today:** Partial. The listeners source reads `/proc/net/tcp*` every few seconds; each listening socket appears in Network > Listening programs with its program (your own processes), reach and first-seen time, a new listener enters Activity, and `net.listener.wildcard` (Review) fires on a wildcard bind. Owning unit and package attribution, and the firewall zone, are not added yet.
- **Proposed mitigation / detection:** Add a CLI check plus monitor diff, `bigman network listeners`, with these steps:
  1. Parse `/proc/net/{tcp,tcp6,udp,udp6}` directly, so the check is deterministic and doesn't depend on `ss` output format. Keep LISTEN state for TCP, and unconnected sockets for UDP.
  2. Classify each bind address as loopback, link-local, private (RFC 1918 or ULA), wildcard or public.
  3. Attribute the owner. For your own UID, use the `/proc/*/fd` inode match, then the exe path, then the package. For other UIDs, use the UID name and the systemd socket unit (`systemctl list-sockets`), and the `docker-proxy`/`rootlessport` command lines, which are world-readable unless `/proc` is mounted with `hidepid`. Offer an optional "Show owners (admin)" button that runs a fixed `ss -H -tulpn` through polkit once, never persistently.
  4. Look up the port in a shipped, versioned table of well-known risky ports (2375, 2376, 6379, 9200, 27017, 5432, 3306, 8888, 8265, 11434, 9229, 9222, 5900, 3389, 111, 139, 445, 631/udp, 5355/udp, 1900/udp).
  5. Emit stable rule IDs: `net.listener.wildcard`, `net.listener.new` (diff against an accepted baseline, like `bigman changes`), `net.listener.unowned-binary`, `net.listener.risky-port`.
  6. Explain reachability using the active firewall zone (NE-12). For example, say "reachable from LAN under FedoraWorkstation zone", which is a statement about policy, not a probe.
- **Test case idea:** Write a synthetic `/proc/net/tcp` fixture with three rows (`0100007F:1F90` for 127.0.0.1:8080, `00000000:2CAB` for 0.0.0.0:11435, and a UID that doesn't exist) and assert the classifications. For an integration test, bind a `TcpListener` on `0.0.0.0:0` in-process and check the finding names the test binary. In a temp `HOME`, accept the baseline and then open a second listener to check that `net.listener.new` fires.
- **References:**
  - https://www.winterknight.net/how-to-win-ccdc-red-team/
  - https://man7.org/linux/man-pages/man8/ss.8.html
  - https://man7.org/linux/man-pages/man5/proc.5.html
  - https://ubuntu.com/server/docs/how-to/security/firewalls/
  - MITRE ATT&CK T1046 Network Service Discovery (attacker-side scanning that finds these listeners): https://attack.mitre.org/techniques/T1046/
  - T1133 External Remote Services: https://attack.mitre.org/techniques/T1133/

### NE-02 Weak SSH server configuration
- **What it is:** An `sshd` that accepts passwords, permits root login, or allows broad forwarding turns any leaked or guessed password into a remote shell. On desktops sshd is often enabled "temporarily" and forgotten. Fedora's Workstation zone allows the `ssh` service. Hardening means key-only authentication, `PermitRootLogin no` (or `prohibit-password` at most), restricting `AllowUsers`/`AllowGroups`, disabling unneeded forwarding, and leaving `PerSourcePenalties` on. That setting is on by default since OpenSSH 9.8 and slows down abusive sources.
- **Real-world examples:** The sshell.co write-up of National CCDC 2025 (2025-07-05) says blue teams should "prefer key-based SSH and watch for password-based logins where keys are expected". The CCDC blue-team manual mirror puts SSH keys and services in the first sweep. The ASEC quarterly honeypot statistics (2025-2026) show password-guessing bots as the dominant way into Linux SSH servers.
- **What a defender inspects (indicators):**
  - Effective configuration from `sshd -T` (needs root, because host keys and `/etc/ssh/sshd_config` are mode 0600 on Fedora) for these keys: `passwordauthentication`, `kbdinteractiveauthentication`, `permitrootlogin`, `permitemptypasswords`, `pubkeyauthentication`, `authorizedkeysfile`, `allowusers`, `allowgroups`, `x11forwarding`, `allowtcpforwarding`, `gatewayports`, `permittunnel`, `allowagentforwarding`, `maxauthtries`, `logingracetime`, `persourcepenalties`, `listenaddress`, `port`, `usepam`.
  - The drop-in directory `/etc/ssh/sshd_config.d/*.conf`. The first value wins, so a drop-in can silently override the main file.
  - `systemctl is-enabled sshd` (Fedora) or `ssh` (Debian/Ubuntu); `systemctl is-active sshd.socket`.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - A CLI check, `bigman ssh audit`. If sshd isn't installed or enabled, report "no SSH server enabled" as an info finding and stop.
  - Otherwise, after the user clicks "Read SSH server settings (admin)", run a fixed `pkexec sshd -T` argument list once and parse it.
  - Findings: `ssh.config.password-auth`, `ssh.config.root-login`, `ssh.config.empty-passwords`, `ssh.config.forwarding`, `ssh.config.gatewayports`, `ssh.config.wildcard-listen`, `ssh.config.penalties-off`, `ssh.config.dropin-override` (a drop-in value that differs from the main file).
  - Give remediation text only. Never edit `sshd_config`, because a bad edit can lock the user out.
  - If the user declines elevation, fall back to `ssh-audit` (MIT) against `127.0.0.1` when it is installed, and say the result covers algorithms only.
- **Test case idea:** Feed fixture `sshd -T` outputs (plain text, one `key value` per line) to the parser: a hardened profile, `passwordauthentication yes` with `permitrootlogin yes`, and a drop-in override case. Assert the rule IDs. No real sshd is needed.
- **References:**
  - https://man.openbsd.org/sshd_config
  - https://www.openssh.org/releasenotes.html (9.8: PerSourcePenalties, the sshd-session split)
  - https://www.sshell.co/red-teaming-at-national-ccdc-2025/
  - https://github.com/C0nd4/CCDC-Blueteam-Manual
  - https://github.com/dev-sec/ssh-baseline
  - https://github.com/jtesta/ssh-audit
  - T1021.004 Remote Services: SSH: https://attack.mitre.org/techniques/T1021/004/

### NE-03 SSH brute force, password spraying and credential stuffing
- **What it is:** Bots try common or leaked username and password pairs against any reachable SSH port, around the clock. Most attempts fail, but the volume tells the user that sshd is internet-reachable. A single "Accepted password" from an unfamiliar address after a run of failures is the high-value signal. Credential stuffing reuses passwords leaked elsewhere, and spraying tries one password across many accounts.
- **Real-world examples:**
  - AhnLab ASEC's quarterly "Malware Targeting Linux SSH Servers" statistics (for example, Q4 2025, published 2026-01-08) report P2PInfect, Prometei and XMRig as top payloads after weak-password SSH access.
  - Darktrace's PumaBot report (2025-05-28) describes SSH brute force followed by `authorized_keys` and systemd persistence.
  - The CCDC 2025 red-team write-up notes that cracked and reused passwords gave early access across hosts.
- **What a defender inspects (indicators):**
  - `journalctl -u sshd -u ssh --since -7d` plus `_COMM=sshd-session` (since OpenSSH 9.8 per-session messages carry the `sshd-session` tag, and since 10.0 there is also `sshd-auth`).
  - `/var/log/secure` (Fedora/RHEL) or `/var/log/auth.log` (Debian/Ubuntu), both root-only.
  - `lastb` (root) and `last`.
  - Patterns: "Failed password", "Invalid user", "Connection closed by authenticating user", "Accepted password/publickey for USER from IP", and "maximum authentication attempts exceeded".
  - Suspicious states: many sources against one user; one source against many users; any `Accepted password` when password auth was meant to be off; or an accepted login from a source that previously only failed.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - A CLI check plus monitor, `bigman network auth-summary`. Read the journal rootless when the user belongs to `wheel`, `adm` or `systemd-journal` (journalctl(1) says these groups can read all journal files). Otherwise say "incomplete: no permission to read system journal".
  - Parse with fixed regexes, never by shelling out to grep.
  - Output: counts per source IP and per username, the first and last time seen, and successful logins.
  - Findings: `auth.ssh.bruteforce` (more than N failures in a window), `auth.ssh.success-after-failures` (High), `auth.ssh.password-login` (an accepted password when the policy is key-only), `auth.ssh.root-login`.
  - Recommend installing fail2ban (GPL-2.0) or sshguard (BSD-style), or ensuring `PerSourcePenalties`. Bigman should not ban IPs itself, because that would need a root firewall change.
- **Test case idea:** A fixture journal export (`journalctl -o json` lines) containing 50 synthetic "Failed password for invalid user test from 192.0.2.10" entries and one "Accepted password for alice from 192.0.2.10". Use RFC 5737 documentation addresses. Assert `auth.ssh.success-after-failures`.
- **References:**
  - https://asec.ahnlab.com/en/92004/
  - https://asec.ahnlab.com/en/87357/
  - https://www.darktrace.com/blog/pumabot-novel-botnet-targeting-iot-surveillance-devices
  - https://www.sshguard.net/
  - https://github.com/fail2ban/fail2ban
  - T1110.001 Password Guessing: https://attack.mitre.org/techniques/T1110/001/
  - T1110.003 Password Spraying: https://attack.mitre.org/techniques/T1110/003/
  - T1110.004 Credential Stuffing: https://attack.mitre.org/techniques/T1110/004/

### NE-04 Vulnerable SSH daemons
- **What it is:** Pre-authentication bugs in the SSH server let an attacker run code or deny service before any password check, so key-only configuration does not help. The defence is patching (or not exposing sshd), and knowing whether the installed package carries the fix. Distros backport fixes without changing the upstream version string, so a naive "version < X" check gives false alarms.
- **Real-world examples:**
  - **CVE-2024-6387 "regreSSHion"** (Qualys, 2024-07-01): a signal-handler race in sshd on glibc Linux. It affects 8.5p1 up to (not including) 9.8p1, and also versions before 4.4p1 that lack the old fixes. Setting `LoginGraceTime 0` removes the code-execution risk at the cost of a denial-of-service exposure.
  - **CVE-2025-26465** (client MitM with `VerifyHostKeyDNS`, 6.8p1-9.9p1) and **CVE-2025-26466** (pre-auth CPU and memory DoS, 9.5p1-9.9p1), both fixed in 9.9p2 (2025-02-18).
  - **OpenSSH 10.0** (2025-04-09) fixed `DisableForwarding` not disabling X11 and agent forwarding.
  - **CVE-2023-48795 "Terrapin"** (2023-12-18): a prefix-truncation downgrade affecting chacha20-poly1305 and CBC-EtM. Both ends must be patched.
  - **CVE-2025-32433** (April 2025): unauthenticated RCE in the Erlang/OTP SSH server (not OpenSSH), fixed in OTP-27.3.3, 26.2.5.11 and 25.3.2.20. It was added to the CISA KEV catalogue in June 2025, and matters for developers who run Erlang/Elixir services.
  - OpenSSH 10.3-10.6 (2026) shipped further security fixes, and the 10.6 notes (2026-10-06) say more frequent releases are coming. The OpenSSH `security.html` page lists advisories only up to April 2025, so do not rely on it alone.
  - The 2024 xz-utils backdoor targeted sshd via liblzma; see [distro packaging](./03-distro-packaging-and-install-channels.md).
- **What a defender inspects (indicators):** the installed package version (`rpm -q openssh-server`, `dpkg-query -W openssh-server`); the distro security tracker entry for each CVE; `ssh -V`; whether sshd is enabled or listening (NE-01); and any Erlang or other embedded SSH servers listening on unusual ports.
- **Bigman today:** Partial. `bigman advisories` (Protect > Updates) matches installed packages against the distribution's own data (dnf updateinfo on Fedora, the Debian security tracker, the Arch security tracker), raises updates that fix a CISA KEV entry to High, and flags a pending reboot. OpenSSH and Erlang SSH fixes appear there like any other package; embedded SSH servers outside packages are not covered.
- **Proposed mitigation / detection:** A feed-driven advisory, `pkg.cve.ssh-server`. Compare the installed NEVRA or Debian version against the distro's own fixed-version data: Fedora Bodhi or updateinfo, Debian Security Tracker JSON, Ubuntu OVAL or USN. See [vulnerability feeds](../research/vulnerability-intelligence-and-feeds.md). Raise severity only when sshd is also enabled and listening beyond loopback. Show the vendor's interim mitigation text (for example `LoginGraceTime 0` for regreSSHion) with its trade-off.
- **Test case idea:** A fixture feed entry ("openssh-server fixed in 9.6p1-1.fc40.x") and fixture installed versions above and below it, with a fake listener table that has 22/tcp on 0.0.0.0 or only on 127.0.0.1. Assert severity changes with exposure.
- **References:**
  - https://www.qualys.com/2024/07/01/cve-2024-6387/regresshion.txt
  - https://nvd.nist.gov/vuln/detail/CVE-2024-6387
  - https://www.openssh.org/security.html
  - https://www.openssh.org/releasenotes.html
  - https://blog.qualys.com/vulnerabilities-threat-research/2025/02/18/qualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466
  - https://terrapin-attack.com/
  - https://seclists.org/oss-sec/2025/q2/52
  - https://www.tenable.com/blog/cve-2025-32433-erlangotp-ssh-unauthenticated-remote-code-execution-vulnerability
  - https://www.cisa.gov/known-exploited-vulnerabilities-catalog
  - T1190 Exploit Public-Facing Application: https://attack.mitre.org/techniques/T1190/

### NE-05 Unrecognised SSH keys and risky forwarding and agent settings
- **What it is:** An extra key in `authorized_keys` (for the user or for root) gives silent, password-free re-entry, and is the first thing many Linux bots and CCDC red teams add. Agent forwarding to an untrusted host lets that host use your keys while you are connected. A changed `AuthorizedKeysFile` or `AuthorizedKeysCommand` can move key lookup somewhere nobody watches.
- **Real-world examples:** PumaBot (Darktrace, 2025) and Qubitstrike (Cado/Darktrace, 2023) both add attacker SSH keys. The CCDC blue-team manual lists sweeping for `authorized_keys` and `id_rsa` files across the filesystem as a first action.
- **What a defender inspects (indicators):**
  - `~/.ssh/authorized_keys` for every user with a login shell (root needed for other users).
  - Effective `authorizedkeysfile`, `authorizedkeyscommand` and `trustedusercakeys` (via `sshd -T`).
  - Key options (`command=`, `from=`, `restrict`).
  - `~/.ssh/config` entries with `ForwardAgent yes` on `Host *`.
- **Bigman today:** Partial. `bigman changes` baselines the current user's `~/.ssh/authorized_keys`. It does not cover other users, root, a non-default `AuthorizedKeysFile`, or `~/.ssh/config`. Account-wide coverage belongs to [host integrity](./04-host-integrity-and-hardening-audit.md), and key protection to [credentials](./06-credential-and-secret-protection.md).
- **Proposed mitigation / detection:**
  - Extend the baseline to `~/.ssh/config` and `/etc/ssh/sshd_config.d/` (names and hashes only, readable via the admin read in NE-02).
  - Add `ssh.client.forwardagent-wildcard` for `ForwardAgent yes` under `Host *`.
  - Show each key's fingerprint and comment, never the key body.
- **Test case idea:** In a temp HOME, write `.ssh/config` containing `Host *\n ForwardAgent yes` and assert the finding. Then append a dummy `ssh-ed25519 AAAA... test@example` line to a baselined `authorized_keys` and assert a diff.
- **References:**
  - https://man.openbsd.org/ssh_config
  - https://man.openbsd.org/sshd#AUTHORIZED_KEYS_FILE_FORMAT
  - T1098.004 SSH Authorized Keys: https://attack.mitre.org/techniques/T1098/004/

### NE-06 Docker Engine API exposed on TCP
- **What it is:** The Docker daemon API is root-equivalent. Docker's docs say "anyone with the keys can give any instructions to your Docker daemon" and "guard these keys as you would a root password". Enabling `-H tcp://0.0.0.0:2375` (no TLS) to make a remote IDE or CI runner work gives the whole network root on the host. Mass scanners look for 2375 specifically.
- **Real-world examples:**
  - Datadog Security Labs (2024) documented a campaign scanning 2375 and deploying miners.
  - Kaspersky (2025) described a Dero miner spreading via exposed container APIs, and estimated about 485 Docker API default ports exposed per month in 2025 from Shodan data.
  - Trend Micro and Akamai (June-September 2025, reported by The Hacker News) described a Tor-based miner that spread via 2375 and then added firewall rules to keep other actors out. That also makes the change a firewall-tampering indicator (NE-12).
- **What a defender inspects (indicators):**
  - A listener on 2375 or 2376 (NE-01).
  - `ps -o args= -C dockerd` for `-H tcp://` or `--host=tcp://`.
  - `/etc/docker/daemon.json` `"hosts"`.
  - `systemctl cat docker.service` drop-ins under `/etc/systemd/system/docker.service.d/`.
  - `DOCKER_HOST` in user environment files.
  - A `tcp://` host without `tlsverify` is critical; a `tcp://` host bound to a non-loopback address is high.
- **Bigman today:** Partial. `net.listener.docker_api` (High) fires on a listener on 2375 or 2376 beyond loopback, in Network and as a new-listener entry in Activity. Daemon flags, `daemon.json` and unit drop-ins are not read yet.
- **Proposed mitigation / detection:**
  - `docker.api.tcp` (Critical without TLS, High with TLS on a wildcard bind). All sources are world-readable or rootless.
  - Guidance: use the Unix socket or SSH (`DOCKER_HOST=ssh://`), or TLS with client certificates on 2376.
  - Same check for Podman's `podman system service tcp:`.
- **Test case idea:** Fixture `daemon.json` with `{"hosts":["unix:///var/run/docker.sock","tcp://0.0.0.0:2375"]}` and a fixture unit drop-in with `ExecStart=/usr/bin/dockerd -H tcp://0.0.0.0:2375`. Assert Critical. A variant with `"tlsverify": true` and port 2376 should be High.
- **References:**
  - https://docs.docker.com/engine/security/protect-access/
  - https://securitylabs.datadoghq.com/articles/attackers-deploying-new-tactics-in-campaign-targeting-exposed-docker-apis/
  - https://www.kaspersky.com/about/press-releases/kaspersky-uncovers-dero-crypto-miner-spreading-via-exposed-container-environments
  - https://thehackernews.com/2025/09/tor-based-cryptojacking-attack-expands.html
  - https://github.com/docker/docker-bench-security
  - T1610 Deploy Container: https://attack.mitre.org/techniques/T1610/
  - T1496 Resource Hijacking: https://attack.mitre.org/techniques/T1496/

### NE-07 Docker/Podman published ports bypassing ufw/firewalld
- **What it is:** `docker run -p 6379:6379` publishes on all host addresses (IPv4 and IPv6) by default, and Docker's docs say "publishing container ports is insecure by default". Docker inserts its own NAT and forward rules, so published-port traffic is routed before ufw's INPUT rules apply, and `ufw deny 6379` has no effect. With firewalld, Docker creates a `docker` zone with ACCEPT and a `docker-forwarding` policy. Users who believe "ufw is on" are often wrong about what is reachable. Before Engine 28.0.0, hosts on the same L2 segment could reach ports published only to 127.0.0.1.
- **Real-world examples:**
  - Docker's own "Packet filtering and firewalls" page states the ufw bypass.
  - The community `ufw-docker` project (GPL-3.0) exists solely to work around it.
  - Exposed Redis and Elasticsearch incidents (NE-08) frequently come from containers started with `-p` on a host the owner believed was firewalled.
- **What a defender inspects (indicators):**
  - `docker ps --format '{{.Names}} {{.Ports}}'` (needs docker group) showing `0.0.0.0:` or `[::]:`.
  - Rootless and world-readable: `/proc/*/cmdline` of `docker-proxy` processes, showing `-host-ip 0.0.0.0 -host-port N`. Unavailable with `userland-proxy: false`, in which case use the listener table plus `nft list ruleset` (root).
  - `rootlessport` processes for rootless Podman/Docker.
  - The `DOCKER-USER` chain contents (root).
  - `daemon.json` `"ip"` (default bind address) and `"iptables"`/`"firewall-backend"`.
  - Whether `ufw` is the active firewall.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - `docker.publish.wildcard`: one finding per published port bound to a wildcard, naming the container (via the docker CLI when allowed) or the `docker-proxy` arguments.
  - `docker.firewall-bypass`: raise severity when ufw is the active firewall and Docker publishes wildcard ports, with the explanation and links to the docs.
  - Guidance: bind with `-p 127.0.0.1:PORT:PORT`, set `"ip": "127.0.0.1"` in `daemon.json`, or add `DOCKER-USER` rules.
  - Never edit firewall rules automatically.
- **Test case idea:** Fixture process table entries (`/usr/bin/docker-proxy -proto tcp -host-ip 0.0.0.0 -host-port 6379 -container-ip 172.17.0.2 -container-port 6379`) plus a fixture "ufw active" state. Assert both findings. An `-host-ip 127.0.0.1` variant should produce none.
- **References:**
  - https://docs.docker.com/engine/network/packet-filtering-firewalls/
  - https://docs.docker.com/engine/network/port-publishing/
  - https://docs.docker.com/engine/network/firewall-iptables/
  - https://github.com/chaifeng/ufw-docker
  - T1686 Disable or Modify System Firewall, formerly T1562.004, the closest technique (the mechanism here is accidental): https://attack.mitre.org/techniques/T1686/

### NE-08 Unauthenticated data stores
- **What it is:** Redis, older Elasticsearch (before 8.0 security was off unless enabled), MongoDB without `--auth`, PostgreSQL with `trust` in `pg_hba.conf`, and MySQL or MariaDB with empty root passwords are frequently started for development and left reachable. Redis's own docs say it "is designed to be accessed by trusted clients inside trusted environments". Attackers abuse Redis configuration and replication features to write files or run code as the Redis user. CCDC reports that "no team had fully restricted database access or rotated all default database credentials" by the end of day one.
- **Real-world examples:**
  - P2Pinfect, a Rust worm abusing Redis replication (Unit 42 and Cado, July 2023). Later variants (2024) added ransomware and miners, and ASEC lists it as 80% of Q4 2025 Linux SSH honeypot payloads.
  - Migo (Cado, 2024-02-20) disabled Redis protected mode and replica-read-only, then changed host settings including `/etc/hosts` (see NE-15).
  - RedisRaider (Datadog, May 2025) is unverified detail; it was seen only in secondary coverage.
- **What a defender inspects (indicators):**
  - Listeners on 6379, 9200/9300, 27017, 5432 and 3306 beyond loopback (NE-01).
  - `redis.conf`: `bind`, `protected-mode no`, absence of `requirepass` or ACL users.
  - `elasticsearch.yml`: `xpack.security.enabled: false`, `network.host: 0.0.0.0`.
  - `mongod.conf`: `security.authorization` and `net.bindIp`.
  - `pg_hba.conf` lines with `trust` for non-local hosts, and `postgresql.conf` `listen_addresses = '*'`.
  - Container equivalents via `docker inspect` env (for example `ALLOW_EMPTY_PASSWORD=yes`).
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - `service.datastore.exposed` (port and owning-package match, or a container image name match). `service.redis.protected-mode-off` and `service.elasticsearch.security-off` are config lints that read only non-secret keys and never print secret values.
  - Guidance: bind to 127.0.0.1, enable auth, and firewall the port.
- **Test case idea:** Fixture `redis.conf` with `bind 0.0.0.0` and `protected-mode no`; fixture `elasticsearch.yml` with `xpack.security.enabled: false`; fixture `pg_hba.conf` with `host all all 0.0.0.0/0 trust`. Assert findings, and that output never contains a `requirepass` value.
- **References:**
  - https://redis.io/docs/latest/operate/oss_and_stack/management/security/
  - https://www.elastic.co/guide/en/elasticsearch/reference/current/security-minimal-setup.html
  - https://unit42.paloaltonetworks.com/peer-to-peer-worm-p2pinfect/
  - https://www.darktrace.com/blog/p2pinfect-new-variant-targets-mips-devices
  - https://www.infosecurity-magazine.com/news/linux-malware-migo-targets-redis/
  - https://threats.wiz.io/all-incidents/migo-cryptominer-targeting-redis
  - https://www.sshell.co/red-teaming-at-national-ccdc-2025/
  - T1190: https://attack.mitre.org/techniques/T1190/

### NE-09 Exposed notebooks and ML frameworks
- **What it is:** Jupyter gives a browser-based terminal and code execution. It is token-protected by default, but people disable the token and password or bind to `0.0.0.0` on GPU boxes. Ray's dashboard and Jobs API (8265) have no authentication, and the vendor considers that a deployment concern. CVE-2023-48022 is disputed and unpatched, so exposure is the vulnerability.
- **Real-world examples:**
  - Qubitstrike (Cado, October 2023) used internet-exposed Jupyter to deploy miners, a rootkit and SSH-key persistence.
  - ShadowRay (Oligo, 2024) and ShadowRay 2.0 (Oligo, 2025-11-18) found more than 200,000 internet-exposed Ray servers and a self-propagating botnet.
- **What a defender inspects (indicators):**
  - Listeners on 8888-8890 (Jupyter), 8265 and 10001 (Ray), 6006 (TensorBoard) and 7860 (Gradio) beyond loopback.
  - `~/.jupyter/jupyter_server_config.py` or `.json` (`ServerApp.ip`, `IdentityProvider.token = ''`, `ServerApp.password`).
  - Process args such as `--ip=0.0.0.0` or `--NotebookApp.token=` (empty).
- **Bigman today:** Partial. `net.listener.lan_service` (Review) flags Jupyter (8888), TensorBoard (6006), Streamlit (8501) and Gradio (7860) bound beyond loopback. Ray and Jupyter token or password settings are not read yet.
- **Proposed mitigation / detection:**
  - `devserver.jupyter.exposed` and `devserver.jupyter.no-auth` (config and argument parsing, user-owned so rootless).
  - `devserver.ray.exposed` (port match and process name `ray`/`dashboard`).
  - Guidance: bind to 127.0.0.1 and use `ssh -L` for remote access.
- **Test case idea:** Fixture `jupyter_server_config.json` `{"ServerApp":{"ip":"0.0.0.0"},"IdentityProvider":{"token":""}}` and a fixture process argv. Assert both findings.
- **References:**
  - https://jupyter-server.readthedocs.io/en/latest/operators/security.html
  - https://www.darktrace.com/blog/qubitstrike-an-emerging-malware-campaign-targeting-jupyter-notebooks
  - https://www.oligo.security/blog/shadowray-2-0-attackers-turn-ai-against-itself-in-global-campaign-that-hijacks-ai-into-self-propagating-botnet
  - https://thehackernews.com/2025/11/shadowray-20-exploits-unpatched-ray.html
  - https://nvd.nist.gov/vuln/detail/CVE-2023-48022
  - https://docs.ray.io/en/latest/ray-security/index.html
  - T1190: https://attack.mitre.org/techniques/T1190/

### NE-10 Local AI model servers bound publicly
- **What it is:** Ollama binds `127.0.0.1:11434` by default, but its FAQ shows `OLLAMA_HOST=0.0.0.0:11434` for network use, and the API has no built-in authentication. Official Docker images publish it more widely. An exposed instance lets anyone use the GPU, pull or overwrite models, and reach any API bugs. Similar local servers (LM Studio's server, vLLM, text-generation-webui, ComfyUI, Open WebUI) follow the same pattern. This is a new and fast-growing desktop exposure.
- **Real-world examples:**
  - Wiz "Probllama" CVE-2024-37032 (June 2024): path traversal leading to RCE in Ollama before 0.1.34. Wiz found more than 1,000 vulnerable instances exposed on 2024-06-10.
  - Cisco Talos, "Detecting Exposed LLM Servers: A Shodan Case Study on Ollama" (2025-09-01): 1,139 exposed instances, 214 serving models.
  - LeakIX (2026-02) reported about 12,000 exposed instances. Higher figures in some vendor blogs are unverified.
- **What a defender inspects (indicators):**
  - Listeners on 11434 (Ollama), 1234 (LM Studio), 8000 (vLLM, generic), 8188 (ComfyUI), 3000 or 8080 (Open WebUI, generic) beyond loopback.
  - `systemctl cat ollama.service` (world-readable) for `Environment="OLLAMA_HOST=0.0.0.0..."`.
  - The user's `~/.config/environment.d/*.conf`.
  - Docker `-p 11434:11434` (NE-07).
  - The installed Ollama version against advisories.
- **Bigman today:** Partial. `net.listener.lan_service` (Review) flags Ollama (11434), LM Studio (1234) and ComfyUI (8188) bound beyond loopback. The `OLLAMA_HOST` unit environment and vLLM are not checked yet.
- **Proposed mitigation / detection:**
  - `ai.server.exposed` (port, process name and unit env).
  - `ai.server.outdated` (feed-driven, via a GitHub release or OSV advisory for `ollama`).
  - Guidance: keep the loopback default and use a reverse proxy with authentication, or an SSH tunnel or VPN.
- **Test case idea:** Fixture unit file containing `Environment="OLLAMA_HOST=0.0.0.0:11434"` and a `/proc/net/tcp` row for 0.0.0.0:11434. Assert a High finding. With `127.0.0.1:11434` there should be no finding.
- **References:**
  - https://docs.ollama.com/faq
  - https://www.wiz.io/blog/probllama-ollama-vulnerability-cve-2024-37032
  - https://nvd.nist.gov/vuln/detail/CVE-2024-37032
  - https://blogs.cisco.com/security/detecting-exposed-llm-servers-shodan-case-study-on-ollama
  - https://www.theregister.com/2025/09/02/exposed_ollama_servers_insecure_research/
  - https://blog.leakix.net/2026/02/ollama-exposed/
  - https://www.upguard.com/blog/understanding-and-securing-exposed-ollama-instances
  - T1496: https://attack.mitre.org/techniques/T1496/

### NE-11 Debug ports and dev servers reachable from the LAN
- **What it is:** Debuggers are code-execution interfaces by design. Node.js docs say a debugger bound to `0.0.0.0` lets "any clients that can reach your IP address ... run arbitrary code". Chrome's DevTools port gives full control of the browser and its cookies. Since Chrome 136 (announced 2025-03-17), `--remote-debugging-port` is ignored on the default profile because infostealers abused it. Frontend dev servers started with `--host` (Vite, webpack-dev-server, Next.js), `python -m http.server`, and Flask or Werkzeug with `debug=True` expose source, `.env` files or an interactive console to the LAN.
- **Real-world examples:**
  - The Chrome change in 2025 was driven by cookie theft via remote debugging.
  - The 2025 Tor-based Docker miner contained (unreachable) logic for port 9222, according to Akamai's analysis reported by The Hacker News. That shows attackers already catalogue DevTools ports.
  - CCDC write-ups repeatedly cite directory listings and exposed config files as early point losses.
- **What a defender inspects (indicators):**
  - Listeners on 9229 (Node inspector), 9222 (Chrome DevTools), 5173, 3000, 4200, 8000 and 8080 (dev servers), and 5678 (debugpy), beyond loopback.
  - Process args containing `--inspect=0.0.0.0`, `--inspect-brk=0.0.0.0`, `--remote-debugging-port`, `--remote-debugging-address=0.0.0.0`, `--host 0.0.0.0` or `--host` with no value, `http.server` with no `--bind`, or `FLASK_DEBUG=1` in the environment (own processes only).
- **Bigman today:** Partial. `net.listener.debugger` (High) flags the Node.js inspector (9229), Chromium DevTools (9222) and debugpy (5678) reachable beyond loopback. Process arguments and dev-server ports are not inspected yet.
- **Proposed mitigation / detection:**
  - `devserver.debug-port.exposed` (High) and `devserver.http.exposed` (Review).
  - Process argv is user-readable, so this is rootless.
  - The monitor re-checks every 5 minutes and notifies only on new exposures, to avoid nagging during normal development on loopback.
- **Test case idea:** Spawn `sleep 30` via a symlink named `node` with argv `node --inspect=0.0.0.0:9229 app.js` (the argv is fake and sleep ignores it). Assert that the argv rule fires. Also test the port-only path with a fixture `/proc/net/tcp`.
- **References:**
  - https://nodejs.org/en/learn/getting-started/debugging
  - https://developer.chrome.com/blog/remote-debugging-port
  - https://thehackernews.com/2025/09/tor-based-cryptojacking-attack-expands.html
  - https://www.sshell.co/red-teaming-at-national-ccdc-2025/
  - T1190: https://attack.mitre.org/techniques/T1190/

### NE-12 Host firewall off, permissive or tampered with
- **What it is:** A host firewall is the safety net for every mistake above. Common failures:
  - The firewall is not running (Ubuntu's `ufw` is disabled by default).
  - The active zone is permissive (Fedora Workstation allows 1025-65535).
  - Docker's rules make the front-end tool's view misleading (NE-07).
  - Malware flushes rules or adds its own. MITRE now tracks this as T1686 Disable or Modify System Firewall, formerly T1562.004.
- **Real-world examples:**
  - The 2025 Tor-based Docker miner installed a cron job that rewrote firewall rules for port 2375 using iptables, nftables or ufw.
  - SigmaHQ carries Linux rules for "disable ufw" and "iptables flush" process events.
  - WinterKnight's CCDC guide calls strict network and host firewalls one of the three most important early steps.
- **What a defender inspects (indicators):**
  - `systemctl is-active firewalld ufw nftables`.
  - `firewall-cmd --state`, `--get-default-zone`, `--get-active-zones` and `--list-all` (polkit may prompt for some queries).
  - `ufw status verbose` (root).
  - `nft list ruleset` (root). `/etc/firewalld/firewalld.conf` is mode 0600 on Fedora 44, so reading it is root-only there.
  - Shipped zone definitions under `/usr/lib/firewalld/zones/` (world-readable).
  - Suspicious states: no firewall active; default zone with wide port ranges; ruleset hash changed since baseline; a cron or systemd unit that invokes `iptables`, `nft` or `ufw` (cross-check with the `shell.persistence` and `shell.privilege` analyzer rules).
- **Bigman today:** Partial. `bigman doctor` records `systemctl is-active firewalld ufw opensnitch` exit and output in details only. There are no findings. OpenSnitch setup and launch exist (`bigman setup firewall`).
- **Proposed mitigation / detection:**
  - `firewall.none-active` (High).
  - `firewall.permissive-zone`: parse the zone XML for `port` ranges wider than 1,000 ports or services beyond a small allow-list, and explain it as "any app listening on 1025-65535 is reachable from this network".
  - `firewall.ruleset-changed`: a hash of `nft list ruleset` captured during an explicit admin read, compared later.
  - `firewall.docker-bypass` (links NE-07).
  - Report-only; Bigman never changes firewall state.
- **Test case idea:** A copy of `FedoraWorkstation.xml` and a restrictive zone XML as fixtures. Assert `firewall.permissive-zone` only for the first. Use a fixture `systemctl is-active` output of `inactive\ninactive\ninactive` to trigger `firewall.none-active`.
- **References:**
  - https://ubuntu.com/server/docs/how-to/security/firewalls/
  - https://firewalld.org/documentation/
  - https://wiki.nftables.org/
  - https://thehackernews.com/2025/09/tor-based-cryptojacking-attack-expands.html
  - https://github.com/SigmaHQ/sigma/tree/master/rules/linux/process_creation (`proc_creation_lnx_disable_ufw.yml`, `proc_creation_lnx_iptables_flush_ufw.yml`)
  - https://www.winterknight.net/how-to-win-ccdc-red-team/
  - T1686: https://attack.mitre.org/techniques/T1686/

### NE-13 Outbound connections to unexpected hosts
- **What it is:** Most Linux compromises "phone home" to C2, a mining pool or a payload host. Defenders can't enumerate all bad IPs, but they can notice unknown binaries holding long-lived outbound connections, connections to well-known mining-pool ports, and destinations on curated blocklists of hijacked or criminal netblocks. CCDC 2025 notes that large volumes of unusual outbound, DNS and NTP traffic went unnoticed for long periods.
- **Real-world examples:**
  - Oligo reports ShadowRay 2.0 miners talking to pool port 3333.
  - Qubitstrike used Discord for C2.
  - Spamhaus DROP lists netblocks "tied to spam and cybercrime operations" (EDROP was merged into DROP on 2024-04-10).
  - abuse.ch Feodo Tracker's datasets are currently empty following law-enforcement takedowns (Emotet 2021, Operation Endgame 2024), a reminder that feeds go stale.
- **What a defender inspects (indicators):**
  - `ss -H -tunp state established` (own processes rootless), `/proc/net/tcp*` rows in ESTABLISHED state with the remote address and UID.
  - Process exe path and package owner.
  - Remote ports 3333, 4444, 5555, 7777, 14444 and 45700 (common pool ports; a heuristic, so Review severity).
  - Matches against Spamhaus DROP, the FireHOL level1 aggregate, and other user-imported lists.
  - Long-lived connections from processes with no package owner.
- **Bigman today:** Partial. The connections source records, rootless, each of your programs' first network connection and each new address range a known program reaches (first-seen Info entries in Activity, optional notifications). No blocklist or mining-pool matching yet, and other users' processes are not attributed.
- **Proposed mitigation / detection:**
  - `net.outbound.blocklist` (CIDR match, user-imported, with the list name, date and licence recorded in the report).
  - `net.outbound.unowned-binary` (an established connection from a binary not owned by any package and outside an allow-list of known user app paths such as Flatpak and Steam).
  - `net.outbound.pool-port` (Review).
  - Blocklist import is explicit and offline. Ship no list by default; offer download links with the licence shown. Respect Spamhaus's "no more than hourly, daily is enough" guidance.
  - Recommend OpenSnitch or Portmaster for interactive per-app egress rules, since Bigman is not a firewall.
- **Test case idea:** Fixture `/proc/net/tcp` with an ESTABLISHED row to `198.51.100.7:3333` (documentation range). Import a fixture DROP JSON containing `198.51.100.0/24`. Assert both findings, and that the report records the list's `sblid`/date fields.
- **References:**
  - https://www.spamhaus.org/blocklists/do-not-route-or-peer/
  - https://github.com/firehol/blocklist-ipsets
  - https://iplists.firehol.org/
  - https://feodotracker.abuse.ch/blocklist/
  - https://abuse.ch/terms-of-use/
  - https://www.sshell.co/red-teaming-at-national-ccdc-2025/
  - T1071 Application Layer Protocol: https://attack.mitre.org/techniques/T1071/
  - T1571 Non-Standard Port: https://attack.mitre.org/techniques/T1571/
  - T1496: https://attack.mitre.org/techniques/T1496/

### NE-14 Reverse-shell-like process behaviour
- **What it is:** A classic sign of interactive compromise is a shell or script interpreter whose standard input and output are a network socket instead of a terminal or pipe. Benign software almost never does this. It's a strong, deterministic, behaviour-based indicator that doesn't depend on signatures, and blue teams hunt for it after any suspected intrusion. (This catalogue describes only how to *detect* the state.)
- **Real-world examples:**
  - SigmaHQ maintains Linux process-creation rules for reverse-shell command lines in several interpreters (DRL 1.1).
  - Falco's default ruleset has "Redirect STDOUT/STDIN to Network Connection in Container" (Apache-2.0).
  - CCDC blue-team manuals list removing `nc`, `socat` and similar tools, and checking `lsof` for odd connections, among the first actions.
- **What a defender inspects (indicators):**
  - For each process whose `comm` is a shell or interpreter (`sh`, `bash`, `dash`, `zsh`, `python*`, `perl`, `ruby`, `php`, `node`, `lua`), check `readlink /proc/PID/fd/0`, `fd/1` and `fd/2`. A `socket:[inode]` whose inode maps to a TCP or UDP row with a non-loopback remote is suspicious.
  - Parent process lineage: a web server, database or `cron` parent is especially notable.
  - Rootless visibility is limited to the user's own processes, which covers desktop-user compromise but not root or service accounts.
- **Bigman today:** Partial. The `shell.network-listener` static rule flags `nc`/`socat` in inspected installers, and the `trace.network` sandbox finding flags network syscalls during rehearsal. There's no live-process check.
- **Proposed mitigation / detection:**
  - `net.shell-socket-stdio` (Critical) in the monitor's 5-minute pass and in `bigman doctor`. Report the PID, the exe, the parent chain and the remote endpoint. Never kill the process; offer "copy details" and guidance.
  - An optional root mode (a one-shot polkit helper with a fixed argument list) widens coverage to all UIDs.
  - Continuous coverage would need auditd (`execve` plus `connect`) or eBPF (out of scope for rootless).
- **Test case idea:** The test opens a loopback `TcpListener` and connects to it, then spawns `/bin/sh -c 'sleep 30'` with stdin and stdout set to the connected socket's file descriptor. The shell runs only `sleep`; nothing is read from the socket. Assert detection. Then assert no detection when stdio is a pipe, and Review rather than Critical when the remote is loopback.
- **References:**
  - https://attack.mitre.org/techniques/T1059/004/
  - https://attack.mitre.org/techniques/T1095/
  - https://github.com/SigmaHQ/sigma/blob/master/rules/linux/process_creation/proc_creation_lnx_netcat_reverse_shell.yml
  - https://github.com/falcosecurity/rules/blob/main/rules/falco_rules.yaml
  - https://github.com/C0nd4/CCDC-Blueteam-Manual

### NE-15 DNS tampering
- **What it is:** Changing name resolution silently redirects the user (phishing, fake package mirrors) or blinds security tooling, for example by pointing update or cloud-agent domains at `0.0.0.0`.
  - `/etc/hosts` is consulted before DNS on most systems (`nsswitch.conf` `hosts: files ...`).
  - `/etc/resolv.conf` may be a symlink to systemd-resolved's stub (`/run/systemd/resolve/stub-resolv.conf`), a NetworkManager-generated file, or a static file. Replacing the symlink with a static file pointing elsewhere is a quiet persistent change.
  - DNS settings also arrive via DHCP from a rogue or compromised router.
- **Real-world examples:**
  - Migo (Cado, 2024-02) edited `/etc/hosts` to block cloud-provider domains used by security and monitoring agents (reported by Infosecurity Magazine).
  - Router DNS hijacking campaigns are a long-standing pattern. Specific campaigns were not re-verified for this catalogue.
- **What a defender inspects (indicators):**
  - The hash and contents of `/etc/hosts` (world-readable). Look for entries for security vendors, package mirrors, `github.com`, `pypi.org`, `registry.npmjs.org`, or banks.
  - `ls -l /etc/resolv.conf` (is it a symlink, and to what?).
  - `resolvectl status` (current per-link DNS servers, DNSSEC and DNSOverTLS).
  - `/etc/systemd/resolved.conf` and `/etc/systemd/resolved.conf.d/`.
  - `nmcli -t -f NAME,IP4.DNS,IP6.DNS dev show`.
  - `/etc/nsswitch.conf` `hosts:` line order.
  - `/etc/NetworkManager/conf.d/` `dns=` settings.
- **Bigman today:** Gap. `bigman changes` does not watch these files.
- **Proposed mitigation / detection:**
  - Add `/etc/hosts`, `/etc/resolv.conf` (target and content), `/etc/nsswitch.conf` and `/etc/systemd/resolved.conf{,.d}` to the startup-changes baseline (they are world-readable).
  - New rules: `dns.hosts.changed`, `dns.hosts.sensitive-domain` (an entry for a domain on a shipped list of package registries and security vendors), `dns.resolv.symlink-replaced`, and `dns.servers.changed` (diff of `resolvectl` servers against the accepted baseline per connection name).
  - The monitor watches with inotify, as canaries already do.
- **Test case idea:** Point the checker at a temp root with a fixture `etc/hosts` that gains `0.0.0.0 registry.npmjs.org`. Assert `dns.hosts.sensitive-domain`. Replace a fixture symlink `etc/resolv.conf -> ../run/systemd/resolve/stub-resolv.conf` with a regular file and assert `dns.resolv.symlink-replaced`.
- **References:**
  - https://www.infosecurity-magazine.com/news/linux-malware-migo-targets-redis/
  - https://man7.org/linux/man-pages/man5/resolved.conf.5.html
  - https://man7.org/linux/man-pages/man5/hosts.5.html
  - https://man7.org/linux/man-pages/man5/nsswitch.conf.5.html
  - T1565.001 Stored Data Manipulation (closest technique): https://attack.mitre.org/techniques/T1565/001/
  - T1557 Adversary-in-the-Middle: https://attack.mitre.org/techniques/T1557/

### NE-16 LLMNR, mDNS and NetBIOS name poisoning
- **What it is:** When normal DNS fails, multicast name protocols ask the whole LAN "who is `fileserver`?", and any host can answer. On untrusted Wi-Fi, a poisoner can redirect connections and capture authentication attempts. systemd-resolved enables LLMNR by default on many distros and sends single-label names to LLMNR. A Fedora request to disable it (Red Hat bug 2228188) was closed EOL on 2026-06-08 without a change. mDNS (Avahi, resolved) and NetBIOS (Samba `nmbd`) have similar trust models.
- **Real-world examples:** MITRE ATT&CK T1557.001, now titled "Name Resolution Poisoning and SMB Relay". Fedora bug 2228188 and the linked systemd PR "Disable LLMNR protocol in default builds" (open at the time of capture).
- **What a defender inspects (indicators):**
  - UDP 5355 (LLMNR), 5353 (mDNS) and 137/138 (NetBIOS) listeners on 0.0.0.0.
  - `resolvectl status` shows `+LLMNR`/`+mDNS` per link.
  - `resolved.conf` `LLMNR=`/`MulticastDNS=`.
  - NetworkManager `connection.llmnr` and `connection.mdns`.
  - `systemctl is-active avahi-daemon nmb`.
- **Bigman today:** Gap. The listener rules treat systemd-resolved's LLMNR listener on 5355 as a distribution default rather than a wildcard finding; `name.llmnr-enabled` is not implemented.
- **Proposed mitigation / detection:** `name.llmnr-enabled` and `name.mdns-enabled` (Review, with "matters on public Wi-Fi" context). Suggest `LLMNR=no` in a resolved drop-in, and per-connection settings for untrusted networks. Report-only.
- **Test case idea:** Fixture `resolvectl status` text containing `Protocols: +DefaultRoute +LLMNR -mDNS -DNSOverTLS` and a fixture resolved.conf. Assert `name.llmnr-enabled`. With `LLMNR=no` in the drop-in fixture, assert no finding.
- **References:**
  - https://attack.mitre.org/techniques/T1557/001/
  - https://man7.org/linux/man-pages/man5/resolved.conf.5.html
  - https://bugzilla.redhat.com/show_bug.cgi?id=2228188

### NE-17 cups-browsed IPP chain and CUPS reflection
- **What it is:** `cups-browsed` auto-adds network printers. Up to 2.0.1 it listened on UDP 631 on all interfaces and trusted any source. Chained with attribute-handling bugs in `libcupsfilters`, `libppd` and `cups-filters`, a remote party could plant a malicious printer definition whose filter ran commands when the user printed. Akamai also showed that exposed cups-browsed instances can be abused as DDoS reflectors. Red Hat noted the chain needs `cups-browsed` to be enabled. Many desktop distros ship it enabled.
- **Real-world examples (verified):** Disclosed 2024-09-26 by Simone Margaritelli:
  - CVE-2024-47176 (cups-browsed, UDP 631 trusts any source)
  - CVE-2024-47076 (libcupsfilters)
  - CVE-2024-47175 (libppd)
  - CVE-2024-47177 (cups-filters foomatic-rip)

  Red Hat rated them Important. Akamai (2024-10-01) found more than 198,000 internet-reachable instances, about 58,000 abusable for amplification. CVE-2024-47850 is a related cups-browsed DDoS identifier, but its relationship to 47176 is described inconsistently across trackers (unverified).
- **What a defender inspects (indicators):**
  - `systemctl is-active cups-browsed` and `is-enabled`.
  - A UDP `0.0.0.0:631` listener.
  - `/etc/cups/cups-browsed.conf` `BrowseRemoteProtocols` containing `cups` (vulnerable configuration; `dnssd` only is the mitigation).
  - Package versions of `cups-browsed`, `libcupsfilters`, `libppd` and `cups-filters` against distro advisories.
  - Unexpected printers in `lpstat -v` (rootless).
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** `cups.browsed-active` (Review if enabled; High if UDP 631 is listening on a wildcard address and `BrowseRemoteProtocols` includes `cups`), `cups.unexpected-printer` (baseline `lpstat -v`), and a feed-driven `pkg.cve.cups-browsed`. Guidance: disable cups-browsed if you don't need auto-discovery.
- **Test case idea:** Fixture `cups-browsed.conf` with `BrowseRemoteProtocols dnssd cups`, a fixture UDP row `00000000:0277` (0.0.0.0:631) and fixture `is-active` output. Assert High. With `BrowseRemoteProtocols dnssd`, assert Review.
- **References:**
  - https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/
  - https://nvd.nist.gov/vuln/detail/CVE-2024-47176
  - https://github.com/OpenPrinting/cups-browsed/security/advisories/GHSA-rj88-6mr5-rcw8
  - https://openprinting.github.io/OpenPrinting-News-Flash-cups-browsed-Remote-Code-Execution-vulnerability/
  - https://www.redhat.com/en/blog/red-hat-response-openprinting-cups-vulnerabilities
  - https://www.akamai.com/blog/security-research/october-cups-ddos-threat
  - T1190: https://attack.mitre.org/techniques/T1190/
  - T1498.002 Reflection Amplification: https://attack.mitre.org/techniques/T1498/002/

### NE-18 Router UPnP/NAT-PMP port mappings and router-level exposure
- **What it is:** UPnP Internet Gateway Device (IGD) lets any LAN application ask the router to open an inbound port, with no authentication. Games, torrent clients and media servers do this, and so can malware. An exposed UPnP interface on the WAN side can also be abused as a proxy or reflector. A desktop tool can't fix the router, but it can show which mappings exist and which local apps hold UPnP sockets.
- **Real-world examples:**
  - CallStranger CVE-2020-12695 (June 2020): a UPnP `SUBSCRIBE` callback flaw enabling data exfiltration and DDoS reflection. The fix is in the OCF spec of 2020-04-17, but device patches lag.
  - Akamai's "UPnProxy" research (2018) found routers whose UPnP was abused as proxies (the URL returned 403 to automated checks; unverified here).
- **What a defender inspects (indicators):**
  - UDP 1900 (SSDP) listeners and the owning apps (rootless for own processes).
  - Router UPnP status in its admin UI.
  - Current IGD port mappings: `upnpc -l` from miniupnpc is read-only listing, and it sends an SSDP discovery on the LAN.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** `upnp.client-app` (Info: which local apps speak SSDP). An optional "List router port mappings" button runs a fixed read-only `upnpc -l` if installed, and only on explicit click, since it sends LAN traffic. `upnp.mapping-to-this-host` flags mappings whose internal client is this machine's address. A guidance card covers disabling UPnP on the router.
- **Test case idea:** Fixture `upnpc -l` output with one mapping `TCP 8080->192.168.1.50:8080 'test'` and a local address of 192.168.1.50. Assert `upnp.mapping-to-this-host`.
- **References:**
  - https://www.helpnetsecurity.com/2020/06/09/cve-2020-12695/
  - https://www.akamai.com/blog/security-research/upnproxy-turns-routers-into-proxies (unverified, 403 to automated fetch)
  - https://attack.mitre.org/techniques/T1090/
  - https://attack.mitre.org/techniques/T1133/

### NE-19 VPN/WireGuard configuration hygiene
- **What it is:** VPN configs hold private keys and can run code: `wg-quick` executes `PreUp`/`PostUp`/`PreDown`/`PostDown` lines with bash as root. A world-readable config leaks the key, and an unexpected hook is a root persistence point. `SaveConfig = true` silently rewrites the file. Overly broad `AllowedIPs` on a peer can route more traffic than intended, and a full tunnel without DNS settings leaks queries. The WireGuard listen port (51820) is a wildcard UDP listener by design.
- **Real-world examples:** No specific incident verified for this catalogue. This is a configuration-hygiene item. The man page documents that hooks are executed by bash.
- **What a defender inspects (indicators):**
  - File mode and owner of `/etc/wireguard/*.conf` (should be 0600 root; the directory listing needs root on most distros).
  - `/etc/NetworkManager/system-connections/*.nmconnection` (0600 root).
  - Presence (not contents) of `PostUp`/`PreUp` keys.
  - `SaveConfig`.
  - Peers with `AllowedIPs = 0.0.0.0/0, ::/0`, with or without `DNS =`.
  - `systemctl list-unit-files 'wg-quick@*'`.
  - Listeners on 51820/udp (NE-01).
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - `vpn.wireguard.world-readable` (High; checkable rootless when the file is in fact readable, which is exactly the problem).
  - `vpn.wireguard.hook-present` (Review, showing hook *names* only, behind admin read).
  - `vpn.wireguard.saveconfig` (Info).
  - Never print key material: the parser must drop `PrivateKey` and `PresharedKey` values before anything is stored.
- **Test case idea:** A temp dir with `wg0.conf` at mode 0644 containing a dummy key and `PostUp = echo test`. Assert the findings, and assert the stored report never contains the dummy key string.
- **References:**
  - https://man7.org/linux/man-pages/man8/wg-quick.8.html
  - https://www.wireguard.com/quickstart/
  - https://www.wireguard.com/papers/wireguard.pdf
  - T1133: https://attack.mitre.org/techniques/T1133/

### NE-20 Unexpected tunnels and remote-access agents
- **What it is:** Reverse tunnels publish a local port to the internet without any inbound firewall change: ngrok, cloudflared quick tunnels, Tailscale Funnel, `ssh -R`, and frp. Remote-desktop agents (VNC, xrdp, GNOME Remote Desktop, RustDesk, AnyDesk, TeamViewer) give interactive control. Developers use all of these legitimately. Attackers install the same tools because outbound traffic to a reputable service blends in. To a defender, an unexplained tunnel is equivalent to an open port.
- **Real-world examples:**
  - MITRE T1219 "Remote Access Tools" and T1572 "Protocol Tunneling" catalogue adversary use of such tools.
  - Ollama's FAQ itself documents exposing the server via ngrok or Cloudflare Tunnel, showing how routinely dev servers get tunnelled.
- **What a defender inspects (indicators):**
  - Processes and units named `ngrok`, `cloudflared`, `tailscaled` with Funnel enabled (`tailscale funnel status`), `frpc`, `ssh` with `-R`, `x11vnc`, `xrdp`, `gnome-remote-desktop`, `rustdesk`, `anydesk` or `teamviewerd`.
  - Their autostart entries (`~/.config/autostart`, user and system units).
  - Long-lived outbound connections to tunnel providers.
- **Bigman today:** Gap. Autostart and systemd directories are baselined by `bigman changes`, so a new unit would show as a change, but nothing names it as remote access.
- **Proposed mitigation / detection:** `remote.tunnel-agent` and `remote.desktop-agent` (Review): a shipped list of binary names and unit names, plus `ssh -R` argv detection for the user's own processes. Show the publishing target when it can be read without secrets (for example `cloudflared` argv `--url http://localhost:8080`).
- **Test case idea:** Fixture process list with argv `cloudflared tunnel --url http://localhost:3000` and `ssh -N -R 8080:localhost:80 host.example`. Assert both findings. A plain `ssh host.example` should produce none.
- **References:**
  - https://attack.mitre.org/techniques/T1219/
  - https://attack.mitre.org/techniques/T1572/
  - https://tailscale.com/kb/1223/funnel
  - https://docs.ollama.com/faq

### NE-21 Default, reused or unchanged service credentials
- **What it is:** Every CCDC red-team report ranks unchanged default credentials and password reuse as the biggest early win, ahead of any exploit. On a desktop or home lab this means database containers started with sample passwords, admin panels (routers, NAS, Grafana, Portainer) on factory defaults, and one password shared across SSH, sudo and services.
- **Real-world examples:**
  - WinterKnight (2026) calls changing default credentials one of the three most important steps.
  - sshell.co (2025) reports that "no team had fully restricted database access or rotated all default database credentials" by the end of day one, and that cookie and session-signing secrets were rarely rotated.
  - GoBruteforcer (2025, secondary coverage) is a botnet that tries default-style usernames across services.
- **What a defender inspects (indicators):**
  - Presence of compose or `.env` files with well-known sample values for variables such as `POSTGRES_PASSWORD`, `MYSQL_ROOT_PASSWORD` or `GF_SECURITY_ADMIN_PASSWORD`. Compare by hash against a short list of known defaults, without displaying them.
  - `ALLOW_EMPTY_PASSWORD=yes`.
  - Accounts with empty password fields (root-only `/etc/shadow`; see [host integrity](./04-host-integrity-and-hardening-audit.md)).
- **Bigman today:** Gap. Git plaintext credential detection exists (`git.plaintext-credentials`), but there's nothing for services.
- **Proposed mitigation / detection:** `service.default-credential` for project directories the user points at (`bigman packages DIR` already walks projects), matching variable names and hashed known-default values. The "first 15 minutes" view (below) puts credential rotation first, as a manual checklist item.
- **Test case idea:** Fixture `docker-compose.yml` with `POSTGRES_PASSWORD: postgres` and `.env` with `ALLOW_EMPTY_PASSWORD=yes`. Assert findings, and that the report shows only the variable name and "matches a known default".
- **References:**
  - https://www.winterknight.net/how-to-win-ccdc-red-team/
  - https://www.sshell.co/red-teaming-at-national-ccdc-2025/
  - https://www.esecurityplanet.com/threats/50000-servers-exposed-as-gobruteforcer-scales-brute-force-attacks/ (secondary coverage)
  - T1078.001 Default Accounts: https://attack.mitre.org/techniques/T1078/001/

### NE-22 Legacy RPC, file-sharing and remote-desktop daemons on a desktop
- **What it is:** Some daemons are rarely needed on a single-user workstation but are often pulled in by package groups and listen on wildcard addresses: `rpcbind` (111), NFS, Samba (139/445), `telnetd`, VNC (5900+) and `xrdp` (3389). Each is an additional pre-authentication attack surface. CCDC guidance is blunt: disable anything that is not needed for the host's role.
- **Real-world examples:** WinterKnight (2026) asks why a host runs a service its role doesn't need. Samba and rpcbind are historic sources of remote CVEs. The specific recent CVEs were not re-verified for this catalogue.
- **What a defender inspects (indicators):** `systemctl list-unit-files --state=enabled` for `rpcbind.socket`, `nfs-server`, `smb`, `nmb`, `xinetd`, `telnet.socket`, `vncserver@`, `xrdp` and `gnome-remote-desktop`; listener ports 111, 2049, 139, 445, 23, 5900-5910 and 3389 on non-loopback addresses.
- **Bigman today:** Partial. When bound beyond loopback they appear in Network > Listening programs with `net.listener.wildcard` (Review), without naming the service or its role.
- **Proposed mitigation / detection:** `service.legacy-network-daemon` (Review), with a one-line role question: "Is this machine a file server?". Guidance: `systemctl disable --now` (shown as text for the user to run, never run by Bigman).
- **Test case idea:** Fixture `systemctl list-sockets` output with a `0.0.0.0:111 rpcbind.socket` row. Assert a finding naming `rpcbind.socket`.
- **References:**
  - https://www.winterknight.net/how-to-win-ccdc-red-team/
  - https://attack.mitre.org/techniques/T1133/
  - https://attack.mitre.org/techniques/T1021/

## Proposed Bigman checks (domain specification)

These are deterministic, rootless by default, read-only and report-only. "Admin read" means one fixed-argument polkit invocation that the user starts explicitly. Its output is parsed and stored in the private report and is never persisted as a privileged helper.

| Check | Command / source | Rule IDs | Root? | Component |
|---|---|---|---|---|
| Listening-socket review | `/proc/net/{tcp,tcp6,udp,udp6}`, `/proc/*/fd` (own UID), `/proc/*/cgroup`, `/proc/*/cmdline`, `systemctl list-sockets --all`, `rpm -qf`/`dpkg -S` on exe; optional admin `ss -H -tulpn` | `net.listener.wildcard`, `net.listener.new`, `net.listener.unowned-binary`, `net.listener.risky-port`, `service.legacy-network-daemon` | No (better with admin read) | `bigman network listeners`, doctor, monitor diff |
| SSH config audit | admin `sshd -T`; fallback `ssh-audit 127.0.0.1` if installed; `~/.ssh/config` | `ssh.config.password-auth`, `ssh.config.root-login`, `ssh.config.empty-passwords`, `ssh.config.forwarding`, `ssh.config.gatewayports`, `ssh.config.dropin-override`, `ssh.config.penalties-off`, `ssh.client.forwardagent-wildcard` | Yes for server config (0600 on Fedora) | `bigman ssh audit` |
| Auth-log summary | `journalctl -o json -u sshd -u ssh _COMM=sshd-session` (rootless for `wheel`/`adm`/`systemd-journal`) | `auth.ssh.bruteforce`, `auth.ssh.success-after-failures`, `auth.ssh.password-login`, `auth.ssh.root-login` | No, if in a journal-reader group | `bigman network auth-summary`, monitor daily |
| Docker exposure | `/etc/docker/daemon.json`, `systemctl cat docker.service`, `dockerd` argv, `docker-proxy`/`rootlessport` argv, optional `docker ps` | `docker.api.tcp`, `docker.publish.wildcard`, `docker.firewall-bypass` | No | `bigman network docker` |
| Hosts/resolv baseline | `/etc/hosts`, `/etc/resolv.conf` (link target and content), `/etc/nsswitch.conf`, `resolved.conf{,.d}`, `resolvectl status`, `nmcli dev show` | `dns.hosts.changed`, `dns.hosts.sensitive-domain`, `dns.resolv.symlink-replaced`, `dns.servers.changed`, `name.llmnr-enabled`, `name.mdns-enabled` | No | `bigman changes` extension and monitor inotify |
| Firewall status | `systemctl is-active firewalld ufw nftables opensnitch`, `/usr/lib/firewalld/zones/*.xml`, admin `firewall-cmd --list-all`, `nft list ruleset` hash, `ufw status verbose` | `firewall.none-active`, `firewall.permissive-zone`, `firewall.ruleset-changed`, `firewall.docker-bypass` | Partly | doctor and `bigman network firewall` |
| Exposed dev/AI services | listener table and argv/config lints (Jupyter, Ray, Ollama, Node inspector, Chrome DevTools, dev servers, Redis/ES/Mongo/PG configs) | `devserver.*`, `ai.server.exposed`, `service.datastore.exposed`, `service.redis.protected-mode-off`, `service.elasticsearch.security-off` | No | listeners check sub-rules |
| Outbound review | `/proc/net/tcp*` ESTABLISHED and own-process attribution; user-imported DROP/FireHOL lists | `net.outbound.blocklist`, `net.outbound.unowned-binary`, `net.outbound.pool-port`, `net.shell-socket-stdio` | No (own UID); root widens | monitor and doctor |
| Print/discovery | `systemctl is-active cups-browsed avahi-daemon`, `cups-browsed.conf`, `lpstat -v`, UDP 631/5353/5355/1900 | `cups.browsed-active`, `cups.unexpected-printer`, `upnp.client-app` | No | doctor |
| VPN/tunnels | `/etc/wireguard` modes (admin), `wg-quick@` units, NetworkManager connection modes, tunnel/remote-desktop binary and unit list | `vpn.wireguard.*`, `remote.tunnel-agent`, `remote.desktop-agent` | Partly | doctor |
| Advisories | installed `openssh-server`, `cups-browsed`, `ollama`, `erlang` versions against distro fixed versions | `pkg.cve.ssh-server`, `pkg.cve.cups-browsed`, `ai.server.outdated` | No | feed-driven advisory ([feeds](../research/vulnerability-intelligence-and-feeds.md)) |

Design notes:
- **Reachability language:** Bigman never probes the network from outside, and it does not "scan the LAN". It says "listening on all interfaces; current firewall zone X permits this port from network Y". That is a policy statement, not a guarantee.
- **Log tags:** OpenSSH 9.8+ logs per-session messages as `sshd-session` (and 10.0+ adds `sshd-auth`). Parsers must match `sshd`, `sshd-session` and `sshd-auth`, or they will silently undercount on current distros.
- **Docker overlap:** Many findings overlap. For example, Ollama in Docker with `-p 11434:11434` under ufw yields `ai.server.exposed`, `docker.publish.wildcard` and `firewall.docker-bypass`. Group them in the UI under one "exposed service" card, keyed by port and owner.

## CCDC-style "first 15 minutes" view (proposed GUI checklist)

CCDC teams get a short window before the red team starts and use a one-page plan: credentials, then attack surface, then persistence, then monitoring. The CCDC blue-team manual mirror and the WinterKnight and sshell red-team debriefs agree on this order. Bigman can present the same order as a checklist on the desktop. Each row shows a status from a deterministic check (*findings to review*, *no findings from these checks*, or *incomplete*) or is marked "manual". No row auto-remediates. Remediation text is copyable, for the user to run.

| # | Step (CCDC order) | Bigman evidence | Status source |
|---|---|---|---|
| 1 | **Change default and shared passwords; rotate keys and tokens you didn't create** | Manual. Links to `git.plaintext-credentials`, `service.default-credential`, [credentials](./06-credential-and-secret-protection.md) | manual + findings |
| 2 | **Inventory what is listening and why** | `net.listener.*` table with owner, package and bind scope; accept baseline | listeners check |
| 3 | **Turn off what you don't need** (sshd on a laptop, cups-browsed, rpcbind, Samba, VNC, dev and AI servers on 0.0.0.0) | `service.legacy-network-daemon`, `cups.browsed-active`, `devserver.*`, `ai.server.exposed` | listeners and doctor |
| 4 | **Lock down SSH** (key-only, no root, no wildcard forwarding, current package) | `ssh.config.*`, `pkg.cve.ssh-server` | ssh audit (admin read) |
| 5 | **Check authorized_keys and remote-access agents** | `bigman changes` for `authorized_keys`, `remote.*` | changes and doctor |
| 6 | **Firewall on, with a sane zone; check Docker isn't bypassing it** | `firewall.*`, `docker.*` | firewall and docker checks |
| 7 | **Check name resolution hasn't been tampered with** | `dns.*`, `name.llmnr-enabled` | baseline |
| 8 | **Look for persistence** (cron, systemd, autostart, rc files, ld.so.preload) | existing `bigman changes`, plus the gaps listed in [host integrity](./04-host-integrity-and-hardening-audit.md) | changes |
| 9 | **Review who has been logging in** | `auth.ssh.*` summary, `last` | auth summary |
| 10 | **Watch outbound and odd processes** | `net.outbound.*`, `net.shell-socket-stdio`; offer OpenSnitch or Portmaster | monitor |
| 11 | **Arm tripwires** | `bigman canary arm` (exists) | canary health |
| 12 | **Write down your baseline** | "Accept current state as baseline" for listeners, DNS and firewall hash | store |

## Tools and scripts worth mining for rules

| Tool | URL | License | What we can borrow | Caveats |
|---|---|---|---|---|
| ssh-audit | https://github.com/jtesta/ssh-audit | MIT | Algorithm, version and CVE tables for SSH servers and clients; policy-file format | Network-probes the target; run only against localhost by default |
| dev-sec ssh-baseline | https://github.com/dev-sec/ssh-baseline | Apache-2.0 | Expected `sshd_config` hardening values as a checklist | InSpec format; last push 2024-08 |
| Lynis | https://github.com/CISOfy/lynis | GPL-3.0 | SSH, firewall, name-service and network test IDs (Bigman already parses its reports) | Root gives fuller results; GPL, so borrow ideas rather than code |
| ComplianceAsCode / OpenSCAP | https://github.com/ComplianceAsCode/content | BSD-3-Clause | Rule rationale text and checks for sshd, firewalld and services (CIS/STIG mappings) | Server-oriented, very large |
| docker-bench-security | https://github.com/docker/docker-bench-security | Apache-2.0 | Daemon config checks (TCP socket, TLS, userland-proxy, iptables) | Needs root and Docker access |
| ufw-docker | https://github.com/chaifeng/ufw-docker | GPL-3.0 | Explanation and canonical `DOCKER-USER` pattern for guidance text | Modifies firewall; reference only |
| fail2ban | https://github.com/fail2ban/fail2ban | GPL-2.0 | sshd filter regexes for auth-log parsing (as a reference for our own regexes) | Active banning is out of Bigman scope |
| SSHGuard | https://www.sshguard.net/ | BSD-style (per upstream; verify) | Attack-pattern list per service, journald backend | Active blocking; recommend, don't embed |
| CrowdSec | https://github.com/crowdsecurity/crowdsec | MIT (engine) | Log parsers and scenarios for SSH brute force | Community blocklist has separate terms; telemetry by default, conflicting with Bigman's no-telemetry stance |
| OpenSnitch | https://github.com/evilsocket/opensnitch | GPL-3.0 | Interactive per-app egress control; Bigman already offers setup and launch | Root daemon; separate install |
| Portmaster | https://github.com/safing/portmaster | GPL-3.0 | Per-app network monitor, DNS filtering; alternative to OpenSnitch | Root service; paid tiers for some features |
| SigmaHQ Linux rules | https://github.com/SigmaHQ/sigma | DRL 1.1 (rules) | Process-creation patterns for reverse shells, ufw disable, iptables flush, sshd anomalies, as detection ideas | Attribution required by DRL; needs process telemetry Bigman lacks rootless |
| Falco rules | https://github.com/falcosecurity/rules | Apache-2.0 | "Redirect STDOUT/STDIN to Network Connection", "Disallowed SSH Connection Non Standard Port" logic | eBPF/kernel module runtime |
| Elastic detection-rules | https://github.com/elastic/detection-rules | Elastic License 2.0 | Linux network and persistence rule ideas | ELv2 is not OSI-approved; read for ideas, don't copy into AGPL code |
| Neo23x0 auditd | https://github.com/Neo23x0/auditd | Apache-2.0 | auditd rules for `/etc/hosts`, `resolv.conf`, sshd config and network tools, if a root mode is ever added | Root and auditd required |
| FireHOL blocklist-ipsets | https://github.com/firehol/blocklist-ipsets | No repo licence; each source keeps its own terms | `firehol_level1` (dshield, feodo, fullbogons, spamhaus_drop) as an importable CIDR set | Bogons must only apply to WAN-side traffic; check each source's licence |
| Spamhaus DROP | https://www.spamhaus.org/blocklists/do-not-route-or-peer/ | Free to use for any organisation; attribution and © notice must be kept; download at most hourly | `drop_v4.json`, `drop_v6.json`, `asndrop.json` | TXT format being deprecated; use JSON |
| abuse.ch Feodo Tracker | https://feodotracker.abuse.ch/blocklist/ | CC0 (per abuse.ch Terms of Use) | Botnet C2 IP list format | Datasets currently empty after takedowns; ToS section 7.3 restricts AI-training use (scope unclear) |
| CCDC-Scripts (bluscreenofjeff) | https://github.com/bluscreenofjeff/CCDC-Scripts | BSD-3-Clause | Nothing to borrow: these are red-team scripts | Cited only as a source of defender lessons |
| CCDC-Blueteam-Manual (mirror) | https://github.com/C0nd4/CCDC-Blueteam-Manual | none stated | First-actions checklist ordering (passwords, keys, services, SUID, logged-in users) | No licence; paraphrase only; 2021 |
| BlueLinuxBastion | https://github.com/fulco/BlueLinuxBastion | MIT | Competition hardening checklist items | Scripts make changes; borrow checks only |
| UNHCSC ccdc2026 | https://github.com/UNHCSC/ccdc2026 | none stated | Firewall-enable and cron-review script ideas from a 2026 regional team | No licence; read-only inspiration |
| trimstray linux-hardening-checklist | https://github.com/trimstray/linux-hardening-checklist | MIT | Network and SSH checklist items with rationale | Server-oriented |

## Papers and long-form references

- **Qualys Threat Research Unit (2024).** "regreSSHion: RCE in OpenSSH's server, on glibc-based Linux systems (CVE-2024-6387)", advisory, https://www.qualys.com/2024/07/01/cve-2024-6387/regresshion.txt. Pre-auth bugs make sshd exposure and patch status matter more than auth settings.
- **Bäumer, Brinkmann, Schwenk (2024).** "Terrapin Attack: Breaking SSH Channel Integrity By Sequence Number Manipulation", USENIX Security 2024, https://terrapin-attack.com/TerrapinAttack.pdf. Both client and server need patching. The audit should report negotiated algorithms, not just versions.
- **Durumeric, Wustrow, Halderman (2013).** "ZMap: Fast Internet-Wide Scanning and its Security Applications", USENIX Security 2013, https://zmap.io/paper.pdf. A single host can scan the IPv4 space in under an hour, so "nobody will find my port" is not a defence.
- **Tziakouris and Biasiotto, Cisco Talos (2025-09-01).** "Detecting Exposed LLM Servers: A Shodan Case Study on Ollama", https://blogs.cisco.com/security/detecting-exposed-llm-servers-shodan-case-study-on-ollama. Local AI servers are now a measurable exposure class.
- **Oligo Security (2025-11-18).** "ShadowRay 2.0", https://www.oligo.security/blog/shadowray-2-0-attackers-turn-ai-against-itself-in-global-campaign-that-hijacks-ai-into-self-propagating-botnet. Unauthenticated-by-design ML dashboards must be treated as exposure findings, since there is no patch.
- **Akamai SIRT (2024-10-01).** "When CUPS Runneth Over: The Threat of DDoS", https://www.akamai.com/blog/security-research/october-cups-ddos-threat. A desktop printing daemon was internet-reachable on about 198,000 hosts.
- **Margaritelli (2024-09-26).** "Attacking UNIX Systems via CUPS, Part I", https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/. Primary disclosure of the cups-browsed chain. Used here only for affected components and mitigations.
- **WinterKnight (2026-03-31).** "How to Win CCDC: Dealing With Red Team", https://www.winterknight.net/how-to-win-ccdc-red-team/. Red-team view of what blue teams should do first: credentials, firewalls, removing unneeded services, reviewing listening ports and persistence.
- **sshell.co (2025-07-05).** "Red Teaming at National CCDC 2025", https://www.sshell.co/red-teaming-at-national-ccdc-2025/. Blue teams lost most to unrotated and reused credentials, open database access, directory listings and unmonitored outbound traffic.
- **AhnLab ASEC (quarterly, 2025-2026).** "Statistics Report on Malware Targeting Linux SSH Servers", e.g. https://asec.ahnlab.com/en/92004/. Honeypot data on what follows weak-password SSH. Useful as a feed for `auth.ssh.*` severity tuning.
- **Docker Inc. (current docs).** "Packet filtering and firewalls", https://docs.docker.com/engine/network/packet-filtering-firewalls/. The authoritative statement that published ports bypass ufw, and how Docker integrates with firewalld.
- **Donenfeld (2017).** "WireGuard: Next Generation Kernel Network Tunnel", NDSS 2017, https://www.wireguard.com/papers/wireguard.pdf. Background for the key and `AllowedIPs` (cryptokey routing) semantics used in NE-19.

## Open questions for the maintainer

1. **Admin reads:** Is a one-shot `pkexec` read acceptable for `sshd -T`, `ss -tulpn`, `nft list ruleset` and `/etc/wireguard` metadata? On Fedora 44, `sshd_config`, `sshd_config.d/`, `/var/log/secure` and `firewalld.conf` are all mode 0600 root, so without it the SSH and firewall audits are "incomplete" for most users. The alternative is a minimal, fixed-command polkit action shipped with the package.
2. **Probing:** Should Bigman ever send network traffic for these checks? Candidates are `upnpc -l` (SSDP on the LAN), `ssh-audit 127.0.0.1`, and an opt-in "check my public IP's open ports" that would need a third-party service. The current proposal is: never by default, and only on explicit click.
3. **Blocklists:** Should Bigman ship blocklist *URLs* in the UI (with licences shown), or require users to paste them? Spamhaus requires attribution and rate limits, and Feodo is currently empty. A bundled snapshot would go stale and raise redistribution questions.
4. **Baseline noise:** Developers open and close listeners constantly. Should `net.listener.new` only alert for wildcard binds (recommended), or also for new loopback listeners?
5. **Severity for distro defaults:** Should Fedora Workstation's 1025-65535 zone be a finding at all, or an Info explanation attached to other findings? Flagging it on every Fedora install may feel like noise. Explaining it beside a wildcard dev server is clearly useful.
6. **Root/eBPF tier:** Is there appetite for an optional root or eBPF tier (auditd rules for `/etc/hosts` and `sshd_config` writes, all-UID reverse-shell detection), or should the product stay strictly rootless and say "incomplete" for other users' processes?
7. **Podman:** Podman is the Fedora default container engine. Should Podman-first support (`rootlessport`, `podman ps` as the user, Quadlet units under `~/.config/containers/systemd/`) come before Docker?
