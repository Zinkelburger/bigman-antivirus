# Desktop environment risks and sandboxing

> Scope: risks that come from the Linux *desktop* rather than from servers or package registries. That covers the display server (X11 compared with Wayland and portals), app sandboxes (Flatpak, Snap, Firejail, Bubblewrap, systemd-run) and how far they can be trusted, browser extensions, desktop launchers and MIME handlers, file-manager thumbnailers and indexers, malicious documents and archives, USB devices, screen locking, clipboard history, shell and theme extension stores, mandatory access control (AppArmor/SELinux), spoofed credential prompts, and notification hygiene. Every entry is written for the defender. It says what to inspect and how Bigman could report it as deterministic, read-only "findings to review". How packages are installed and how Snap or Flatpak repositories are trusted belongs to [distro packaging](./03-distro-packaging-and-install-channels.md). Autostart and udev baselining belong to [host integrity](./04-host-integrity-and-hardening-audit.md). Phishing lures belong to [phishing](./01-phishing-social-engineering.md). Last researched: 2026-10-29.

## Summary table

| ID | Attack | Severity for a Linux desktop/dev user | Bigman today | Proposed |
|---|---|---|---|---|
| DS-01 | X11 lets any client keylog, screen-grab and inject input | Medium | Gap | `bigman desktop session` report: session type, X11 fallback, XWayland clients |
| DS-02 | Residual Wayland exposure: XWayland, portal consent fatigue, compositor plugins | Low | Gap | List XWayland clients and screencast/remote-desktop portal grants |
| DS-03 | Over-permissioned or malicious Flatpak apps | Medium | Gap | Flatpak permission audit with risk explanations per permission |
| DS-04 | Sandbox-runtime escape bugs (Flatpak, Bubblewrap, snap-confine) left unpatched | High | Gap | Version check of flatpak/bwrap/snapd/xdg-desktop-portal against a pinned advisory table |
| DS-05 | Snap classic/devmode/unasserted snaps and Snap Store scam apps | Medium | Gap | Snap confinement and interface audit |
| DS-06 | Hijacked or sold browser extensions pushing malicious updates | High | Gap | Browser-extension inventory with change diffing (version, permissions, owner signals) |
| DS-07 | Extensions force-installed via enterprise policy or sideloaded/unpacked | High | Gap | Policy-file and install-location audit for Chromium-family and Firefox |
| DS-08 | Impersonation extensions in official stores (fake wallets, fake tools) | Medium | Gap | Name/ID mismatch and high-risk-permission flags in the inventory |
| DS-09 | `.desktop` launchers disguised as documents | High | Partial | `desktop.launcher-untrusted` rule in Downloads/scan/inspect, masquerade detection |
| DS-10 | MIME and URL-scheme handler hijacking | Medium | Gap | Baseline `mimeapps.list` and user `.desktop` handlers |
| DS-11 | Thumbnailer and indexer auto-parsing of downloads | High | Partial | Thumbnailer/indexer inventory plus parser-version advisories |
| DS-12 | Malicious documents and PDFs (viewer bugs, macros, Ghostscript) | High | Partial | Office macro/event detection, comic-archive and PostScript flags, safe-open workflow |
| DS-13 | Archive extraction: path traversal, symlinks, absolute paths | Medium | Partial | `bigman archive check`: list-only analysis, then optional sandboxed extract |
| DS-14 | USB HID injection, auto-mount/auto-open, rogue devices | Medium | Gap | USBGuard status, automount settings, udev rule baseline |
| DS-15 | Weak screen lock, autologin, lock-screen bypass bugs | Medium | Partial | Lock/idle/autologin report from gsettings, KDE config and display-manager config |
| DS-16 | Clipboard managers storing secrets; clipboard sync | Medium | Gap | Detect clipboard managers and history files (metadata only) |
| DS-17 | GNOME Shell extensions, Plasma global themes and widgets running arbitrary code | Medium | Gap | Inventory and diff; reuse `shell.*` inspect rules on theme scripts |
| DS-18 | MAC (AppArmor/SELinux) disabled, permissive, or user-namespace restriction off | Medium | Partial | `mac.*` findings from rootless status reads |
| DS-19 | Overtrusting Firejail, Bubblewrap and systemd-run sandboxes | Medium | Partial | Explain limits; check Firejail version; `systemd-analyze security` summary for user units |
| DS-20 | Spoofed polkit/sudo/keyring password prompts | High | Partial | New `shell.credential-prompt` inspect rule plus user guidance |
| DS-21 | Desktop notification spoofing and secrets on the lock screen | Low | Gap | Notification lock-screen setting check; Bigman alerts never request actions via notification |
| DS-22 | Unencrypted disk on a portable or shared computer | Medium (High for laptops) | Partial | Rootless `lsblk` walk from `/` and `/home` to a `crypt` device; swap and homed checks |

## Attacks

### DS-01 X11 lets any client keylog, screen-grab and inject input
- **What it is:** X11 has no meaningful isolation between client applications. Any program that can connect to the X server can read keystrokes typed into other windows, take screenshots of the whole desktop, and synthesize input. It does not need root. Malware or a malicious app running as the user can therefore capture passwords typed into a terminal `sudo` prompt, a password manager or a browser. Wayland compositors deliver input only to the focused client and do not let clients read other windows. GNOME 50 (March 2026) and KDE Plasma 6.8 (October 2026) removed their native X11 sessions, but other desktops and older LTS installs still run X11. Some Xfce, MATE, Cinnamon and i3 setups are X11-only.
- **Real-world examples:**
  - Joanna Rutkowska, "The Linux Security Circus: On GUI isolation" (2011): the classic explanation of why X11 undermines every other sandbox. https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html
  - Martin Gräßlin (KWin), "Looking at the security of Plasma/Wayland" (2015). https://blog.martin-graesslin.com/blog/2015/11/
  - GNOME 50 removed the X11 session (The Register, 2026-03-19). https://www.theregister.com/2026/03/19/gnome_50/ Plasma 6.8 went Wayland-only (Phoronix). https://www.phoronix.com/news/KDE-Plasma-68-Wayland-Exclusive
  - Any Flatpak granted `--socket=x11`, or any Firejail profile that does not use a nested X server, inherits this exposure (see DS-03 and DS-19).
- **What a defender inspects (indicators):**
  - Session type: `echo "$XDG_SESSION_TYPE"`, `loginctl show-session "$XDG_SESSION_ID" -p Type -p Desktop`, plus whether `$WAYLAND_DISPLAY` and/or `$DISPLAY` are set.
  - Display manager defaults: `/etc/gdm/custom.conf` or `/etc/gdm3/custom.conf` (`WaylandEnable=false` forces X11), and SDDM's `/etc/sddm.conf.d/*.conf` (`DisplayServer=`).
  - X server access control: `xhost` output. Any `+` entry, or "access control disabled", is a red flag. Also check for an `~/.Xauthority` readable by other users.
  - Under X11, unexpected long-running GUI-less processes that hold an X connection (`xlsclients -l`).
- **Bigman today:** Gap. `bigman doctor` does not report the session type.
- **Proposed mitigation / detection:**
  - A `bigman desktop session` CLI check (also a `doctor` section) that reads the environment, `loginctl` and display-manager configs, and emits:
    - `session.x11` (Review): "Applications in this session can observe each other's keystrokes and screens."
    - `session.x11-forced` (Review): `WaylandEnable=false` or equivalent is set.
    - `session.xhost-open` (High): `xhost` reports disabled access control or wildcard entries.
  - The guidance text explains how to pick a Wayland session at the login screen, and notes when the desktop has no Wayland session at all.
  - Rootless, deterministic, and no dependencies beyond `loginctl` (optional `xhost`).
- **Test case idea:** Run the classifier with a fake environment map (`XDG_SESSION_TYPE=x11`, `DISPLAY=:0`, no `WAYLAND_DISPLAY`) and a temp `custom.conf` containing `WaylandEnable=false`, and assert both findings. Feed a canned `xhost` stdout of "access control disabled".
- **References:** https://attack.mitre.org/techniques/T1056/001/ (Keylogging), https://attack.mitre.org/techniques/T1113/ (Screen Capture), https://wayland.freedesktop.org/architecture.html, https://www.freedesktop.org/software/systemd/man/latest/loginctl.html

### DS-02 Residual Wayland exposure: XWayland, portal consent fatigue, compositor plugins
- **What it is:** Wayland is not a complete answer. Legacy X11 applications run under XWayland and share one X server with each other, so they can still snoop on each other, though not on native Wayland windows. Screen capture and remote control go through xdg-desktop-portal consent dialogs, and a user who clicks "Share" or "Allow remote interaction" hands over the screen and input. Finally, the compositor itself is fully trusted, so a malicious KWin script or GNOME Shell extension runs with compositor privileges (see DS-17).
- **Real-world examples:**
  - Gräßlin noted in 2015 that a malicious KWin plugin can act as a keylogger. https://blog.martin-graesslin.com/blog/2015/11/
  - The Flatpak docs call `--socket=inherit-wayland-socket` "a very sensitive permission". https://docs.flatpak.org/en/latest/sandbox-permissions.html
  - No specific incident abusing portal consent fatigue on Linux was found (unverified as a real-world campaign). It is included as a category.
- **What a defender inspects (indicators):**
  - XWayland clients present: `xlsclients` run inside a Wayland session lists X11 apps. A password manager or terminal running under XWayland is noteworthy.
  - Portal permission store: `flatpak permission-list` (or `flatpak permissions`) shows tables such as `screenshot`, `background` and `devices`. Persisted "remote desktop"/"screencast" restore tokens live in the permission store (`~/.local/share/flatpak/db/`).
  - Active screen-sharing indicators in the panel. GNOME shows an orange/red indicator.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** Extend `bigman desktop session` to:
  - List XWayland clients by name as inventory.
  - Dump portal permission-store grants for screenshot/screencast/remote-desktop/background as "permissions to review". This reads only the user's own permission DB.
  - Never claim the session is secure.
- **Test case idea:** Parse a captured `flatpak permission-list` text fixture containing a `screenshot` grant for `com.example.App` and assert the inventory entry. Parse a fixture `xlsclients` output.
- **References:** https://flatpak.github.io/xdg-desktop-portal/docs/ , https://attack.mitre.org/techniques/T1113/

### DS-03 Over-permissioned or malicious Flatpak apps
- **What it is:** Flatpak's sandbox is only as strong as the permissions in the app's manifest, and many Flathub apps request broad ones: `--filesystem=home` or `host`, `--socket=x11`, `--device=all`, or the unfiltered session bus. Some permissions are effectively a sandbox exit. `--talk-name=org.freedesktop.Flatpak` lets an app spawn commands on the host. `host-root`, `--socket=session-bus` and `--socket=system-bus` remove the D-Bus filtering. A malicious or compromised app with such permissions is just a normal user process with a nice icon. Users rarely see these permissions at install time from GUI stores.
- **Real-world examples:**
  - The Flatpak documentation itself warns that the session/system bus sockets are "a security risk", that `--device=usb` gives "unfettered access to any USB device", and that `--share=network` also grants access to abstract Unix sockets "with no permission checks". https://docs.flatpak.org/en/latest/sandbox-permissions.html
  - The "flatkill.org" critique (2018) of apps shipping with `filesystem=host` is a classic argument (opinionated; historical). https://flatkill.org/
  - The Register (2024-03-28) noted that Flathub flags apps that make notable permission changes, parallel to the Snap Store crackdown. https://www.theregister.com/2024/03/28/canonical_snap_store_scams/
- **What a defender inspects (indicators):**
  - `flatpak list --app --columns=application,origin,version,installation`
  - `flatpak info --show-permissions <APPID>` (effective permissions including overrides) and `flatpak override --show [--user] <APPID>`
  - Raw keyfiles: `/var/lib/flatpak/app/<id>/current/active/metadata` and `~/.local/share/flatpak/app/<id>/current/active/metadata` (`[Context]` `filesystems=`, `sockets=`, `devices=`, `shared=`, and `[Session Bus Policy]`)
  - Overrides: `/var/lib/flatpak/overrides/` and `~/.local/share/flatpak/overrides/`
  - Remotes: `flatpak remotes -d`. A non-Flathub/non-distro remote is a supply-chain question (see [distro packaging](./03-distro-packaging-and-install-channels.md)).
  - Flatseal shows the same data graphically.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - `bigman flatpak audit` (CLI, plus a GUI table) parses the metadata keyfiles and override files directly, with the `flatpak` CLI as a fallback. It is deterministic, rootless and read-only.
  - It emits one finding per risky permission with a fixed risk explanation:

    | Permission | Rule ID | Severity | Fixed explanation |
    |---|---|---|---|
    | `talk-name=org.freedesktop.Flatpak` | `flatpak.host-spawn` | High | Can run commands outside the sandbox. |
    | `filesystem=host`, `host-root`, `host-os`, `host-etc` | `flatpak.filesystem-host` | High | Can read and write most of the host filesystem. |
    | `filesystem=home` | `flatpak.filesystem-home` | Review | Can read `~/.ssh`, browser profiles, `~/.config/autostart`. |
    | `socket=session-bus`, `socket=system-bus` | `flatpak.dbus-unfiltered` | High | No D-Bus filtering. |
    | `socket=x11` without `fallback-x11` | `flatpak.x11` | Review | See DS-01. |
    | `device=all`, `device=usb` | `flatpak.device-broad` | Review | Broad device access. |
    | `persistent=` | `flatpak.persist` | Info | Ties to CVE-2024-42472 on old Flatpak versions. |
    | Any user/system override that widens the base manifest | `flatpak.override-widened` | Review | An override grants more than the app's own manifest. |

  - Snapshot the permission set and diff it on each run, like `bigman changes`, to report `flatpak.permission-added` after updates.
  - The GUI suggests Flatseal or `flatpak override --user --nofilesystem=home <APPID>` as the user-driven remediation. Bigman never applies it automatically.
- **Test case idea:** Create a temp directory tree mimicking `app/com.example.Bad/current/active/metadata` with `[Context] filesystems=host;` and `[Session Bus Policy] org.freedesktop.Flatpak=talk`, plus an override file adding `home`. Assert `flatpak.filesystem-host`, `flatpak.host-spawn` and `flatpak.override-widened`.
- **References:** https://docs.flatpak.org/en/latest/sandbox-permissions.html , https://docs.flatpak.org/en/latest/flatpak-command-reference.html , https://github.com/tchx84/Flatseal , https://attack.mitre.org/techniques/T1611/ (Escape to Host, conceptually)

### DS-04 Sandbox-runtime escape bugs left unpatched (Flatpak, Bubblewrap, snap-confine)
- **What it is:** The sandbox machinery is itself software with security bugs. Unpatched Flatpak, xdg-desktop-portal, Bubblewrap or snapd can let a sandboxed app reach host files, or let a local user become root. Users treat "it's a Flatpak/Snap" as a safety guarantee, so stale runtimes matter more than usual. These are version-checkable facts that a deterministic tool can report precisely.
- **Real-world examples:**
  - **CVE-2024-32462 (April 2024):** Flatpak sandbox escape via the RequestBackground portal and `--command` argument injection. Fixed in Flatpak 1.10.9/1.12.9/1.14.6/1.15.8; xdg-desktop-portal 1.18.4 also mitigates it. https://lists.freedesktop.org/archives/flatpak/2024-April/002341.html , https://access.redhat.com/security/cve/CVE-2024-32462
  - **CVE-2024-42472 (August 2024):** Flatpak `persistent=` directories that are replaced with a symlink get mounted on the next launch, giving file access outside the sandbox. Fixed in 1.14.10 and 1.15.10. https://seclists.org/oss-sec/2024/q3/197 , https://access.redhat.com/security/cve/CVE-2024-42472
  - **CVE-2017-5226:** Bubblewrap TIOCSTI terminal injection escape. The fix is the opt-in `--new-session` flag or a seccomp filter. https://nvd.nist.gov/vuln/detail/CVE-2017-5226
  - **CVE-2026-3888 (March 2026):** snap-confine combined with systemd-tmpfiles gives local root on default Ubuntu Desktop 24.04+. Fixed in snapd 2.73+ubuntu24.04.2, 2.73+ubuntu25.10.1, 2.74.1+ubuntu26.04.1, and upstream 2.75 (Qualys). https://blog.qualys.com/vulnerabilities-threat-research/2026/03/17/cve-2026-3888-important-snap-flaw-enables-local-privilege-escalation-to-root
  - **CVE-2026-8933 (July 2026):** local privilege escalation in the set-capabilities variant of snap-confine (Qualys). Fixed versions were not verified here; consult the USN. https://blog.qualys.com/vulnerabilities-threat-research/2026/07/21/cve-2026-8933-snap-confine-local-privilege-escalation
- **What a defender inspects (indicators):** `flatpak --version`, `bwrap --version`, `snap version`, and the installed xdg-desktop-portal package version (`rpm -q xdg-desktop-portal` / `dpkg-query -W xdg-desktop-portal`). Also `flatpak list --runtime` for end-of-life runtimes, which `flatpak update` warns about, and pending updates via `flatpak remote-ls --updates`.
- **Bigman today:** Gap. OSV-Scanner integration covers project lockfiles, not host packages.
- **Proposed mitigation / detection:**
  - A small, curated advisory table shipped with Bigman, pinned per release and refreshable from the feed pipeline in [vulnerability intelligence](../research/vulnerability-intelligence-and-feeds.md). Each row holds a component, a version-comparison rule per distro, the CVE and the fixed version.
  - `bigman doctor` emits `sandbox.runtime-outdated` (High) with the CVE ID and the exact package to update.
  - Prefer distro-aware comparisons: rpm `rpmdev-vercmp` semantics and dpkg `--compare-versions` semantics, implemented natively.
  - Never auto-update. Reuse the `bigman setup` polkit path only if the user explicitly asks.
- **Test case idea:** Feed version strings (`1.14.5`, `1.14.10`, `2.73+ubuntu24.04.1`) into the comparator with a fixture advisory table and assert which produce findings. Cover Ubuntu `+ubuntu` suffix ordering.
- **References:** https://attack.mitre.org/techniques/T1611/ , https://attack.mitre.org/techniques/T1068/ , https://github.com/flatpak/flatpak/security/advisories , https://ubuntu.com/security/notices

### DS-05 Snap classic/devmode/unasserted snaps and Snap Store scam apps
- **What it is:** Snaps run under strict confinement by default, but `classic` confinement means no sandbox at all, and `devmode` only logs violations instead of blocking them. Snaps installed with `--dangerous` (unasserted, revision `x1` and so on) bypass store signing. Interface connections such as `home`, `removable-media`, `system-files`, `personal-files` and `x11` widen access. The Snap Store has repeatedly hosted fake cryptocurrency wallet apps that stole seed phrases, which shows that store presence is not vetting.
- **Real-world examples:**
  - September 2023: fake crypto apps in the Snap Store; Canonical added temporary manual review. https://www.phoronix.com/news/Snap-Store-Malicious-Apps
  - March 2024: a fake Exodus wallet snap cost one user 9 BTC; Canonical moved to manual review of new snap name registrations. https://www.theregister.com/2024/03/28/canonical_snap_store_scams/
  - snap-confine LPEs (DS-04).
- **What a defender inspects (indicators):** `snap list --all` (the Notes column shows `classic`, `devmode`, `jailmode` and `disabled`; local unasserted revisions start with `x`), `snap connections` (look for `personal-files`, `system-files`, `home`, `removable-media`, `x11`, `ssh-keys`, `password-manager-service`), and `snap info <name>` for the publisher and its verification status.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** `bigman snap audit` parses `snap list --all` and `snap connections`, or the snapd REST API via `/run/snapd.socket`, which is readable by users for list endpoints. It emits these findings with fixed explanations:
  - `snap.classic` (Review): "not sandboxed".
  - `snap.devmode` (High): "sandbox violations are not enforced".
  - `snap.unasserted` (Review): installed outside the store.
  - `snap.sensitive-interface` (Review): per interface. `ssh-keys` and `personal-files` rank above `home`.
  - `snap.wallet-name` (Info): name matches a wallet brand list. This is a name heuristic, labeled as such, and is optional.
- **Test case idea:** Fixture text for `snap list --all` with a `devmode` row and an `x1` revision, plus a `snap connections` fixture with `ssh-keys` connected. Assert the findings.
- **References:** https://snapcraft.io/docs/snap-confinement , https://snapcraft.io/docs/supported-interfaces , https://attack.mitre.org/techniques/T1204/002/

### DS-06 Hijacked or sold browser extensions pushing malicious updates
- **What it is:** Browser extensions auto-update silently. An attacker who phishes a developer's Web Store account, or buys the extension outright, can push a malicious version to every existing user. That version can steal cookies and session tokens, inject scripts into every page, or show fake "update" lures (ClickFix). On a maintainer's Linux workstation that means GitHub, registry and cloud sessions. Store review happens mostly at submission, not continuously.
- **Real-world examples:**
  - **Cyberhaven, 2024-12-25:** a phished admin Chrome Web Store account published malicious v24.10.4, which targeted session tokens and cookies. It was part of a campaign reported to cover at least 16 extensions. https://techcrunch.com/2024/12/27/cyberhaven-says-it-was-hacked-to-publish-a-malicious-update-to-its-chrome-extension , https://securityaffairs.com/172491/hacking/chrome-browser-extensions-compromise.html
  - **RedDirection, July 2025 (Koi Security):** 18 Chrome/Edge extensions, some Verified/Featured, about 2.3M users, turned malicious via later updates. https://www.theregister.com/2025/07/08/browser_hijacking_campaign/
  - **ShadyPanda, December 2025 (Koi Security):** extensions that behaved legitimately for years, then poisoned updates in mid-2024; about 4.3M installs overall. https://www.theregister.com/2025/12/01/chrome_edge_malicious_browser_extensions/
  - **QuickLens, February 2026:** sold via ExtensionHub on 2026-02-01, malicious v5.8 on 2026-02-17 with fake Google Update ClickFix lures and crypto theft; about 7,000 users. https://www.bleepingcomputer.com/news/security/quicklens-chrome-extension-steals-crypto-shows-clickfix-attack/
  - Phishing of extension developers with fake DMCA notices continued in 2026 (secondary report; unverified against the primary Malwarebytes post). https://cipherssecurity.com/fake-copyright-dmca-chrome-developer-phishing/
- **What a defender inspects (indicators):**
  - **Chromium family:** `~/.config/google-chrome/<Profile>/Extensions/<id>/<version>/manifest.json`, `~/.config/chromium/...`, `~/.config/BraveSoftware/Brave-Browser/...`, `~/.config/microsoft-edge/...`, plus Flatpak/Snap variants under `~/.var/app/<id>/config/...` and `~/snap/<name>/common/...`. Each profile's `Preferences` and `Secure Preferences` JSON (`extensions.settings.<id>`) records the install location, the granted permissions, `from_webstore`, and the install time.
  - **Firefox:** `<profile>/extensions.json` (per add-on `id`, `version`, `location`, `userPermissions`, `optionalPermissions`, `signedState`, `sourceURI`, `installDate`, `updateDate`) and `<profile>/extensions/*.xpi`. Field names are from current Firefox builds; verify per version.
  - **Red flags:** a version or permission set that changed recently; host permissions `<all_urls>`, `*://*/*`, `cookies`, `webRequest`, `declarativeNetRequest` with header modification, `scripting`, `debugger`, `nativeMessaging`, `management` or `proxy`; and CSP or header stripping.
- **Bigman today:** Gap. Bigman's own Chromium extension warns on deceptive links and scans downloads, but it does not inventory other extensions.
- **Proposed mitigation / detection:**
  - `bigman browser extensions` (CLI and GUI): a rootless filesystem inventory across Chromium-family and Firefox profiles.
    - Records ID, name (resolving `__MSG_` locale keys), version, permissions and host permissions, `update_url`, install source and dates.
    - Hashes each manifest.
  - Baseline and diff, like `bigman changes`:
    - `browser.extension-added` (Review)
    - `browser.extension-permissions-expanded` (High): new host/cookie/scripting permissions in an update.
    - `browser.extension-updated` (Info)
  - Fixed risk text per permission, taken from Chrome's permission-warning table and MDN.
  - An optional feed-driven blocklist of known-bad extension IDs (from public IOC lists such as the Cyberhaven campaign IDs) produces `browser.extension-known-bad`. The list is user-imported, which matches the existing blocklist model.
  - The Bigman Chromium extension *could* also use `chrome.management`, but that needs an extra, alarming permission. The filesystem approach is preferred and also covers Firefox.
- **Test case idea:** A temp `Extensions/abcdef.../1.0/manifest.json` with `permissions: ["storage"]` is baselined. Then add a `1.1/` with `host_permissions: ["<all_urls>"]` and `cookies`, and assert `browser.extension-permissions-expanded`. Add a Firefox `extensions.json` fixture with one add-on.
- **References:** https://attack.mitre.org/techniques/T1176/001/ , https://developer.chrome.com/docs/extensions/develop/concepts/permission-warnings , https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions , see also [supply chain](./02-supply-chain-packages.md) for IDE extensions

### DS-07 Extensions force-installed via enterprise policy, sideloaded, or unpacked
- **What it is:** On Linux, any process with root (for example a malicious package `postinst` script) can drop a managed-policy JSON file. That file force-installs an extension, which the user cannot remove from the browser UI, and can disable safety settings. User-level malware can load unpacked extensions through profile preferences or command-line flags in a modified `.desktop` launcher. These are durable persistence and credential-theft paths that survive browser "reset".
- **Real-world examples:**
  - Chrome and Chromium read managed policies from `/etc/opt/chrome/policies/managed/` and `/etc/chromium/policies/managed/` (Chromium Linux admin docs). https://www.chromium.org/administrators/linux-quick-start/
  - Firefox reads `policies.json` from `/etc/firefox/policies/` or the install's `distribution/` directory, including `ExtensionSettings` force-installs. https://mozilla.github.io/policy-templates/
  - Policy-based extension force-install is a well-known adware/infostealer technique on Windows and macOS (for example ChromeLoader). No Linux-specific campaign was verified during this research, so it is included as a category.
- **What a defender inspects (indicators):**
  - Any file in `/etc/opt/chrome/policies/{managed,recommended}/`, `/etc/chromium/policies/`, `/etc/opt/edge/policies/`, `/etc/brave/policies/`, `/etc/firefox/policies/policies.json`, or `/usr/lib*/firefox/distribution/policies.json`. Look for keys `ExtensionInstallForcelist`, `ExtensionSettings`, `ExtensionInstallSources`, `DeveloperToolsAvailability`, `SafeBrowsingProtectionLevel` and `DnsOverHttpsMode`.
  - Note which package owns the file: `rpm -qf FILE`, `dpkg -S FILE`.
  - Unpacked extensions: Chromium location "unpacked" in `Preferences`. Developer mode: `extensions.ui.developer_mode`.
  - Browser launchers in `~/.local/share/applications/*.desktop` whose `Exec=` adds `--load-extension=` or `--disable-extensions-except=`.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - Part of `bigman browser extensions`:
    - `browser.policy-forced-extension` (High unless the file is owned by a known distro package)
    - `browser.policy-weakened` (Review): Safe Browsing off, DevTools blocked.
    - `browser.extension-unpacked` (Review)
    - `browser.launcher-flags` (High): launcher adds `--load-extension`.
  - Add the policy directories to the `bigman changes` snapshot set, as world-readable system paths that need no root.
- **Test case idea:** A temp root with `etc/opt/chrome/policies/managed/x.json` containing `{"ExtensionInstallForcelist":["aaaa...;https://example.invalid/update.xml"]}`, plus a user `.desktop` with `Exec=chromium --load-extension=/tmp/x`. Assert both findings.
- **References:** https://attack.mitre.org/techniques/T1176/001/ , https://chromeenterprise.google/policies/ , https://mozilla.github.io/policy-templates/

### DS-08 Impersonation extensions in official stores
- **What it is:** Stores host extensions that impersonate wallets, password managers, AI assistants or PDF tools. They pass review because the malicious logic is small, delayed or server-driven. Crypto users and maintainers who hold signing keys or tokens are prime targets.
- **Real-world examples:**
  - "FoxyWallet", July 2025 (Koi Security): over 40 fake Firefox wallet add-ons impersonating MetaMask, Coinbase, Phantom and others. https://thehackernews.com/2025/07/over-40-malicious-firefox-extensions.html
  - GreedyBear, August 2025: a follow-up reported as about 150 Firefox extensions using "extension hollowing" (secondary reporting).
- **What a defender inspects (indicators):** The inventory from DS-06. Check whether the extension's name matches a well-known brand while its ID differs from the brand's official store ID, whether it was installed recently, and whether it has `<all_urls>` together with clipboard or storage permissions.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** Ship a small, curated, user-updatable allowlist of official IDs for high-value brands (MetaMask, Bitwarden, 1Password, and so on). A name that matches a brand with a non-matching ID produces `browser.extension-brand-mismatch` (Review). The allowlist must be versioned and cited; the finding text says "name resembles X but is not X's published ID".
- **Test case idea:** Fixture manifest named "MetaMask" with a random ID. Assert the finding. The official ID from the fixture allowlist produces no finding.
- **References:** https://attack.mitre.org/techniques/T1176/001/ , https://attack.mitre.org/techniques/T1036/

### DS-09 `.desktop` launchers disguised as documents
- **What it is:** A freedesktop `.desktop` file is a launcher whose `Exec=` line runs any command. Delivered in a ZIP or tarball with a name like `Meeting_Notice.pdf.desktop` and a PDF `Icon=`, it can look like a document in some file managers. Opening it runs a downloader, often while it shows a decoy PDF. The same campaigns then add XDG autostart entries, user systemd units and cron jobs for persistence. GNOME and KDE require an executable bit or "Allow Launching"/trust metadata for launchers outside system paths, but archive extraction and social engineering ("right-click, Allow Launching") get around that.
- **Real-world examples:**
  - APT36 / Transparent Tribe against Indian BOSS Linux, August 2025: a ZIP with `Meeting_Ltr_ID1543ops.pdf.desktop`, a decoy document opened in the browser, and persistence through autostart, a user systemd `system-update.service` and cron (CYFIRMA, 2025-08-22). https://www.cyfirma.com/research/apt36-targets-indian-boss-linux-systems-with-weaponized-autostart-files/ , https://securityaffairs.com/181513/apt/pakistan-linked-apt36-abuses-linux-desktop-files-to-drop-custom-malware-in-new-campaign.html
  - The Hacker News coverage (2025-08). https://thehackernews.com/2025/08/transparent-tribe-targets-indian-govt.html
- **What a defender inspects (indicators):**
  - `*.desktop` files in `~/Downloads`, `/tmp` and extracted archive folders.
  - Double extensions (`.pdf.desktop`, `.docx.desktop`) and `Icon=` values naming document icons (`application-pdf`, `x-office-document`).
  - `Exec=` containing shell, download or decode tooling (`sh -c`, `bash -c`, `curl`, `wget`, `base64`, `xxd`, `/tmp/`), `Terminal=false`, and `NoDisplay=true`.
  - GIO trust metadata: `gio info -a metadata::trusted FILE`.
  - Newly added `~/.config/autostart/*.desktop` (already in `bigman changes`) and `~/.config/systemd/user/*.service`.
- **Bigman today:** Partial. `bigman changes` watches `~/.config/autostart` and `~/.config/systemd/user` (`persistence.change`), and the daily Downloads scan runs ClamAV, which is signature-only. There is no launcher-specific rule.
- **Proposed mitigation / detection:**
  - A new deterministic analyzer for `.desktop` files, applied in `bigman scan`, the Downloads timer, `bigman inspect`, and the browser-extension download hook. It parses the keyfile and runs the existing `shell.*` capability rules on the `Exec=` value, adding:
    - `desktop.launcher-in-downloads` (Review)
    - `desktop.masquerade` (High): double extension or document icon combined with an executable `Exec`.
    - `desktop.exec-download` (High): reuses `shell.download`/`shell.execute-download`.
    - `desktop.hidden` (Review): `NoDisplay=true` outside `/usr/share/applications`.
  - Archives get the same check on entry names without extracting (see DS-13).
  - GUI guidance: "This is a program launcher, not a document."
- **Test case idea:** A synthetic `invoice.pdf.desktop` with `Icon=application-pdf` and `Exec=sh -c "echo test"`. Assert `desktop.masquerade`. A variant with an `Exec` that names `curl` only as an inert string (never executed) triggers `desktop.exec-download`. Pack it into a test ZIP for the archive path.
- **References:** https://attack.mitre.org/techniques/T1204/002/ , https://attack.mitre.org/techniques/T1036/008/ , https://attack.mitre.org/techniques/T1036/007/ , https://attack.mitre.org/techniques/T1547/013/ , https://specifications.freedesktop.org/desktop-entry-spec/latest/ , cross-ref [host integrity](./04-host-integrity-and-hardening-audit.md)

### DS-10 MIME and URL-scheme handler hijacking
- **What it is:** The desktop decides which program opens a file type or URL scheme through `mimeapps.list` and `.desktop` files that declare `MimeType=`. User-level malware can register itself as the default handler for `x-scheme-handler/https`, `application/pdf` or `x-scheme-handler/ssh`. Every link or document then flows through it, which gives quiet persistence and interception without root. Newly registered custom schemes (`x-scheme-handler/foo`) can also make web pages launch local programs with attacker-controlled arguments, which is a recurring bug class in apps that register URL handlers.
- **Real-world examples:**
  - No dated Linux in-the-wild campaign was verified during this research (marked unverified). The mechanism is documented in the freedesktop MIME Applications spec, and the Windows analogue is ATT&CK T1546.001. URL-handler argument-injection bugs in desktop apps are a long-running class, but specific Linux CVEs were not re-verified here.
- **What a defender inspects (indicators):**
  - `~/.config/mimeapps.list`, `~/.local/share/applications/mimeapps.list`, `/etc/xdg/mimeapps.list` and `[Default Applications]` entries pointing to `.desktop` files under `~/.local/share/applications/`.
  - Read-only queries: `xdg-mime query default x-scheme-handler/https` and `gio mime x-scheme-handler/https`.
  - `.desktop` files whose `Exec=` points into the home directory, `/tmp` or a hidden directory.
- **Bigman today:** Gap. Not in the `bigman changes` snapshot set.
- **Proposed mitigation / detection:**
  - Add `mimeapps.list` files and `~/.local/share/applications/` to the `bigman changes` baseline.
  - Rule `desktop.handler-changed` (Review) when the default for http/https/pdf/ssh/mailto/office types changes.
  - Rule `desktop.handler-user-exec` (High) when a default handler's `Exec` resolves to a binary under `$HOME`, `/tmp` or `/dev/shm`.
- **Test case idea:** Baseline a temp XDG config with `x-scheme-handler/https=firefox.desktop`. Change it to `evil.desktop` with `Exec=/home/u/.cache/x %u` (a non-existent path) and assert both rules.
- **References:** https://specifications.freedesktop.org/mime-apps-spec/latest/ , https://attack.mitre.org/techniques/T1546/001/ (Windows analogue), https://attack.mitre.org/techniques/T1546/

### DS-11 Thumbnailer and indexer auto-parsing of downloads
- **What it is:** File managers generate thumbnails, and search indexers (GNOME LocalSearch/Tracker, KDE Baloo) extract metadata, automatically when a file lands in a watched folder. A parser bug can therefore turn a "download only" into code execution, with no file ever opened by the user. Thumbnailers are registered through `.thumbnailer` files, so a user-level thumbnailer entry is also a persistence point.
- **Real-world examples:**
  - **CVE-2023-43641 (October 2023):** out-of-bounds write in libcue reached through tracker-miners auto-indexing `~/Downloads`. GitHub Security Lab described it as "1-click RCE on GNOME". Fixed in libcue 2.3.0, with distro backports. https://github.blog/security/vulnerability-research/coordinated-disclosure-1-click-rce-on-gnome-cve-2023-43641/ , https://bugzilla.redhat.com/show_bug.cgi?id=2242943
  - **CVE-2017-11421 "Bad Taste" (2017):** gnome-exe-thumbnailer ran VBScript embedded in MSI filenames via Wine when thumbnailing. https://nvd.nist.gov/vuln/detail/CVE-2017-11421 , https://www.theregister.com/2017/07/20/bug_of_the_week_gnome_files_msi/
  - **CVE-2017-1000083 (2017):** Evince comic-book (`.cbt`) command injection, also reachable via the evince thumbnailer. https://nvd.nist.gov/vuln/detail/CVE-2017-1000083
  - GNOME's gnome-desktop sandboxes thumbnailers with Bubblewrap (introduced around GNOME 3.26; exact version unverified), which reduces impact. Other desktops vary.
- **What a defender inspects (indicators):**
  - Registered thumbnailers: `/usr/share/thumbnailers/*.thumbnailer` and `~/.local/share/thumbnailers/*.thumbnailer`. A user-level file is unusual.
  - Indexer state: `localsearch status` / `tracker3 status` and `balooctl6 status` (or `balooctl status`).
  - Versions of libcue, libgsf, poppler, ghostscript, libraw, ffmpeg and similar parser libraries against advisories.
  - Whether Downloads is in the indexed set (`gsettings get org.freedesktop.Tracker3.Miner.Files index-recursive-directories`).
- **Bigman today:** Partial. The daily Downloads ClamAV scan catches only signature-known files, and it runs after the indexer has already parsed them.
- **Proposed mitigation / detection:**
  - `bigman desktop audit` lists thumbnailers and flags `desktop.user-thumbnailer` (High) for any entry under `~/.local/share/thumbnailers` or with an `Exec=` outside `/usr`.
  - Report whether the indexer covers Downloads, as information with guidance: users who download untrusted files often can exclude Downloads from indexing.
  - Add parser libraries to the DS-04 version-advisory table (`desktop.parser-outdated`).
  - A monitor option: the existing inotify machinery could trigger an immediate ClamAV scan on Downloads `close_write`. That is still racy against the indexer, and should be documented as such.
- **Test case idea:** A temp `XDG_DATA_HOME/thumbnailers/x.thumbnailer` with `Exec=/home/u/bin/thumb %i %o`. Assert the finding. Use fixture `gsettings` output for the indexer list.
- **References:** https://attack.mitre.org/techniques/T1203/ , https://attack.mitre.org/techniques/T1189/ , https://specifications.freedesktop.org/thumbnail-spec/latest/

### DS-12 Malicious documents and PDFs (viewer bugs, macros, Ghostscript)
- **What it is:** Documents are code-adjacent on Linux too.
  - Office suites run macros and event-bound scripts.
  - Document viewers shell out to archive tools for comic-book formats.
  - PostScript and EPS go through Ghostscript, which is a full interpreter whose `-dSAFER` sandbox has been bypassed repeatedly.
  - Image and PDF converters (ImageMagick) invoke Ghostscript under the hood.
- **Real-world examples:**
  - **CVE-2023-51698 (January 2024):** Atril comic-book archive command injection, CVSS 9.6. Fixed in Atril 1.6.2; the advisory notes the same class hit Evince in 2017. https://github.com/mate-desktop/atril/security/advisories/GHSA-34rr-j8v9-v4p2
  - **CVE-2024-3044 (May 2024):** LibreOffice scripts bound to graphic on-click events ran without the macro security check. Fixed in 7.6.7 and 24.2.3. https://www.libreoffice.org/about-us/security/advisories/CVE-2024-3044
  - **CVE-2024-29510 (2024):** Ghostscript uniprint format string bug giving a `-dSAFER` bypass. Fixed in 10.03.1 and reported as exploited in the wild (secondary sources). https://securityaffairs.com/165449/hacking/ghostscript-vulnerability-cve-2024-29510.html , https://nvd.nist.gov/vuln/detail/CVE-2024-29510
  - Bigman's existing PDF triage covers the structural PDF side (`pdf.structure`).
- **What a defender inspects (indicators):**
  - Office files:
    - ODF: `Basic/` or `Scripts/` directories inside the ZIP, and `script:event-listener` elements in `content.xml`.
    - OOXML: `vbaProject.bin`.
    - OLE: VBA streams. `olevba` from oletools reports these.
  - Comic archives (`.cbt/.cbr/.cbz/.cb7`) whose member names contain shell metacharacters or leading dashes.
  - PostScript/EPS files, including when disguised with a `.jpg` or `.pdf` extension (magic bytes `%!PS`).
  - Viewer and Ghostscript versions against advisories.
- **Bigman today:** Partial. PDF triage (PDFiD, pdf-parser, pdfinfo) and ClamAV. No office-macro or PostScript handling.
- **Proposed mitigation / detection:**
  - Extend file triage with deterministic container parsing:
    - `document.macro` (Review) for ODF Basic/Scripts, OOXML `vbaProject.bin` and OLE VBA.
    - `document.event-script` (High) for ODF event listeners bound to scripts.
    - `document.postscript-masquerade` (High) when PostScript magic appears behind a non-PS extension.
    - `document.comic-archive-names` (High) when member names start with `-` or contain `` ` ``, `$(` or `;`.
  - Optional `olevba` integration, run in Bubblewrap like the PDF tools.
  - Add viewer, Ghostscript and LibreOffice versions to the advisory table.
  - Pair with the safe-open workflow below.
- **Test case idea:** Generate a minimal ODT ZIP with a `Basic/Standard/Module1.xml` that contains only a comment. Generate a `.jpg` file whose content starts with `%!PS-Adobe` and contains nothing else. Generate a `.cbt` tar with a member named `--help.jpg`. Assert the three findings.
- **References:** https://attack.mitre.org/techniques/T1204/002/ , https://attack.mitre.org/techniques/T1203/ , https://attack.mitre.org/techniques/T1566/001/ , https://github.com/decalage2/oletools

### DS-13 Archive extraction: path traversal, symlinks, absolute paths
- **What it is:** An archive can contain entries named `../../.bashrc` or `/home/u/.config/autostart/x.desktop`, or a symlink entry followed by a file "inside" it. Extraction then writes outside the target folder, typically into shell rc files or autostart. GUI archive tools, GNU tar and language libraries have all had such bugs. A related issue is extracting setuid bits, device nodes or huge expansions ("zip bombs").
- **Real-world examples:**
  - **CVE-2020-16116 and CVE-2020-24654 (2020):** KDE Ark `../` traversal and TAR symlink escapes, which could plant `.bashrc` or autostart files. Fixed in 20.08.0 and 20.08.1. KDE advised inspecting archives before extracting. https://kde.org/info/security/advisory-20200827-1.txt
  - A later Ark issue: libarchive plugin absolute-path extraction, fixed in Ark 24.12.0 (secondary aggregator; CVE number unverified).
  - **CVE-2025-45582 (2025):** GNU tar through 1.35, traversal and overwrite through two-stage extraction involving symlinks. https://bugzilla.redhat.com/show_bug.cgi?id=2379592
  - **CVE-2025-4517, CVE-2025-4330, CVE-2025-4138, CVE-2024-12718 (June 2025):** Python `tarfile` extraction-filter bypasses. https://mail.python.org/archives/list/security-announce@python.org/thread/MAXIJJCUUMCL7ATZNDVEGGHUMQMUUKLG/ , https://www.openwall.com/lists/oss-security/2025/06/24/1
  - "Zip Slip" (Snyk, 2018), the cross-ecosystem class name. https://github.com/snyk/zip-slip-vulnerability
- **What a defender inspects (indicators):** List-only views (`bsdtar -tvf FILE`, `unzip -Z -l FILE`, `tar -tvf FILE`). Look for:
  - entries with `..` components or a leading `/`
  - symlinks or hardlinks whose targets leave the root
  - duplicate names where a symlink precedes a file
  - setuid/setgid modes, device or FIFO entries
  - extreme compression ratios
  - `.desktop`, `.bashrc`, `autostart/` or `.ssh/` paths inside the archive
- **Bigman today:** Partial. `bigman scan` passes archives to ClamAV with depth and size limits and never extracts into user directories, but no traversal findings exist.
- **Proposed mitigation / detection:**
  - `bigman archive check FILE`: a pure-Rust, list-only parser for zip and tar (gz/xz/zst) that never writes entries to disk. It emits:
    - `archive.path-traversal` (High)
    - `archive.absolute-path` (High)
    - `archive.symlink-escape` (High)
    - `archive.special-entry` (Review): device, FIFO, setuid.
    - `archive.bomb-ratio` (Review)
    - `archive.persistence-path` (High): entry paths under `.config/autostart`, `.ssh`, `.bashrc` and similar, reusing the `shell.persistence` path list.
    - `archive.launcher` (High): contains a `.desktop` file (DS-09).
  - Optional `--extract` into a fresh directory, inside Bubblewrap with `--unshare-all --new-session`, a read-only home and only the destination writable. Symlinks are refused and names are sanitized. This uses the same runner as `bigman sandbox`.
  - Wire it into the Downloads timer and the browser download hook.
- **Test case idea:** Build fixture archives in-test with the `tar` and `zip` crates: an entry `../escape.txt`, a symlink `link -> /tmp` followed by `link/x`, an absolute `/etc/x`, a `.desktop` entry and a 1 KB file that expands to 100 MB of zeros. Assert each finding, and assert that nothing was written outside the temp dir.
- **References:** https://attack.mitre.org/techniques/T1204/002/ , https://cwe.mitre.org/data/definitions/22.html , https://cwe.mitre.org/data/definitions/59.html

### DS-14 USB HID injection, auto-mount/auto-open, rogue devices
- **What it is:** A USB device can claim to be a keyboard and type commands ("BadUSB" and Rubber-Ducky-style devices), or present storage that the desktop auto-mounts and, with some settings, auto-opens. Auto-mount also exposes filesystem drivers and thumbnailers (DS-11) to attacker-controlled media. USBGuard lets the user allowlist devices so that new devices are blocked until approved.
- **Real-world examples:**
  - Nohl and Lell, "BadUSB – On accessories that turn evil", Black Hat USA 2014 (classic). https://www.blackhat.com/us-14/briefings.html (session listing; slide URL unverified)
  - Tian et al., "SoK: 'Plug & Pray' Today – Understanding USB Insecurity in Versions 1 Through C", IEEE S&P 2018 (URL unverified).
  - USBGuard project. https://usbguard.github.io/
- **What a defender inspects (indicators):**
  - `systemctl is-active usbguard` and `usbguard list-devices` (needs IPC permission configured in `IPCAllowedUsers`/`IPCAllowedGroups`). The policy at `/etc/usbguard/rules.conf` is root-only, so Bigman reports "unreadable" rather than guessing.
  - Kernel default authorization: `/sys/bus/usb/devices/usb*/authorized_default`.
  - GNOME: `gsettings get org.gnome.desktop.media-handling automount`, `automount-open` and `autorun-never`. KDE: `kded` device automounter settings in `~/.config/kded_device_automounterrc` (filename per current Plasma; verify).
  - Custom udev rules in `/etc/udev/rules.d/` and `/run/udev/rules.d/` (baseline, see [host integrity](./04-host-integrity-and-hardening-audit.md)).
  - Device inventory with `lsusb`. Watch for an unexpected second keyboard interface on a storage device.
- **Bigman today:** Gap. udev is explicitly listed as not yet in `bigman changes`.
- **Proposed mitigation / detection:**
  - `bigman doctor` USB section:
    - `usb.no-policy` (Info/Review): USBGuard is not installed or not active.
    - `usb.automount-open` (Review): `automount-open=true` or `autorun-never=false`.
    - `usb.udev-changed`: once `/etc/udev/rules.d` joins the baseline.
  - `bigman setup usbguard --apply` would install the package through the existing fixed-argument polkit path. It generates the initial allow-current-devices policy only after an explicit confirmation that lists every device, because a wrong policy can lock out keyboards.
  - An optional monitor rule: when a device that exposes both HID and mass-storage interfaces appears (from `/sys/bus/usb/devices/*/bInterfaceClass`), show an alert. This is read-only and needs no root.
- **Test case idea:** A fixture sysfs tree under a temp root with a device that has interface classes `03` (HID) and `08` (mass storage). Assert `usb.composite-hid-storage`. Fixture `gsettings` outputs for automount.
- **References:** https://attack.mitre.org/techniques/T1200/ , https://attack.mitre.org/techniques/T1091/ , https://usbguard.github.io/ , https://www.kernel.org/doc/html/latest/usb/authorization.html

### DS-15 Weak screen lock, autologin, lock-screen bypass bugs
- **What it is:** An unlocked or auto-logged-in session gives anyone with physical access the user's browser sessions, SSH agent and keyring. Lock screens have had bypass bugs triggered by monitor hotplug, virtual keyboards or crashes. Timed login and autologin settings in display managers weaken this further.
- **Real-world examples:**
  - CVE-2019-3820: GNOME/gdm keybinding lock bypass. https://nvd.nist.gov/vuln/detail/CVE-2019-3820
  - CVE-2019-3825: GDM timed-login bypass. https://nvd.nist.gov/vuln/detail/CVE-2019-3825
  - CVE-2021-34557: XScreenSaver crash after output disconnects. https://nvd.nist.gov/vuln/detail/CVE-2021-34557
  - CVE-2016-2312: kscreenlocker unlocks after screen power-cycle. https://mailman.archlinux.org/pipermail/arch-security/2016-February/000550.html
  - Cinnamon-screensaver virtual-keyboard bypass (Arch FS#69338, 2021). https://bugs.archlinux.org/task/69338.html
  - No 2024–2026 Linux desktop lock-screen CVE was confirmed in this research.
- **What a defender inspects (indicators):**
  - GNOME: `gsettings get org.gnome.desktop.screensaver lock-enabled`, `lock-delay`, `org.gnome.desktop.session idle-delay`, and `org.gnome.desktop.notifications show-in-lock-screen`.
  - KDE: `~/.config/kscreenlockerrc` (`Autolock`, `Timeout`, `LockOnResume`).
  - Display managers:
    - `/etc/gdm{,3}/custom.conf`: `AutomaticLoginEnable` and `TimedLoginEnable`.
    - `/etc/sddm.conf` and `/etc/sddm.conf.d/*`: `[Autologin] User=`.
    - `/etc/lightdm/lightdm.conf`: `autologin-user`.
  - `loginctl show-session -p LockedHint -p IdleHint`.
- **Bigman today:** Partial. `harden.screen_lock` reads GNOME's `lock-enabled`, `idle-delay` and lock-on-suspend settings and KDE's `kscreenlockerrc`, offers a previewed per-user change with an undo, and says so instead when an administrator locked a key it would change (dconf). `harden.autologin` reads GDM, SDDM and LightDM configuration, rated Review when the disk is also unencrypted ([DS-22](#ds-22-unencrypted-disk-on-a-portable-or-shared-computer)). Lock-screen bypass advisories are not matched yet ([hardening research](../research/hardening.md)).
- **Proposed mitigation / detection:**
  - `bigman desktop audit` (rootless) emits:
    - `session.lock-disabled` (High)
    - `session.lock-delay-long` (Review): idle delay over 15 minutes or lock-delay over 0. The thresholds are documented constants.
    - `session.autologin` (Review)
    - `session.timed-login` (High)
    - `session.lock-on-suspend-off` (Review)
  - The GUI links to the desktop's own setting panel.
  - Feed-driven advisories for gdm, gnome-shell, kscreenlocker, xscreensaver and cinnamon-screensaver versions.
- **Test case idea:** Fixture `custom.conf` with `AutomaticLoginEnable=True` and a fixture `kscreenlockerrc` with `Autolock=false`. Assert the findings.
- **References:** https://attack.mitre.org/techniques/T1078/003/ (Local Accounts), https://help.gnome.org/admin/system-admin-guide/stable/desktop-lockscreen.html.en

### DS-16 Clipboard managers storing secrets; clipboard sync
- **What it is:** Clipboard history tools (Klipper, GPaste, CopyQ, clipman, cliphist) persist everything copied, including passwords, API tokens, recovery codes and seed phrases, often to plain files in the home directory. Any same-user malware, backup job or cloud sync can then read them. Clipboard sharing (KDE Connect and similar) sends secrets to phones. Password managers set a hint MIME type (`x-kde-passwordManagerHint`) so that cooperating managers skip the entry, but not every manager honors it.
- **Real-world examples:**
  - Klipper change to "not insert secret data into history" (honors `x-kde-passwordManagerHint`). https://phabricator.kde.org/R120:637cd0bf62e419b8038cb9b1565089fbd341ee7a
  - KDE bug 458063: KDE Connect synced passwords copied from password managers. https://bugs.kde.org/show_bug.cgi?id=458063
  - xfce4-clipman request to honor the password hint. https://gitlab.xfce.org/panel-plugins/xfce4-clipman-plugin/-/issues/72
  - Clipboard-hijack ("clipper") malware swapping crypto addresses is a cross-platform class; see [malware families](./05-malware-families-and-detection.md).
- **What a defender inspects (indicators):**
  - Running managers (`pgrep -a 'gpaste|copyq|clipman|cliphist|klipper'`; on Plasma 6 Klipper runs inside plasmashell).
  - History file presence, size, permissions and age, without reading content:
    - `~/.local/share/gpaste/`
    - `~/.local/share/klipper/`
    - `~/.config/copyq/` and `~/.local/share/copyq/`
    - `~/.cache/cliphist/`
  - Klipper settings `KeepClipboardContents` and history size in `~/.config/klipperrc`.
  - KDE Connect clipboard plugin enabled.
- **Bigman today:** Gap. Bigman canaries watch a decoy credential file, not clipboard stores.
- **Proposed mitigation / detection:**
  - `bigman desktop audit` emits:
    - `clipboard.history-persistent` (Review): a manager is present and its history file exists. The finding includes the path, size and permission mode; contents are never read.
    - `clipboard.history-world-readable` (High): mode allows group or other access.
    - `clipboard.sync-enabled` (Review)
  - Guidance: exclude password managers, shorten history, or disable persistence.
  - Optional future feature, explicitly opt-in only: run the `shell.credentials` token regexes over the history file to say "N likely secrets present". Report counts only.
- **Test case idea:** A temp `XDG_DATA_HOME/gpaste/history.xml` with mode 0644. Assert both findings, and assert that the reader never opened the file (open counter via a test hook).
- **References:** https://attack.mitre.org/techniques/T1115/ , cross-ref [credentials](./06-credential-and-secret-protection.md)

### DS-17 GNOME Shell extensions, Plasma global themes, widgets and KWin scripts running arbitrary code
- **What it is:** Desktop add-ons are code, not decoration.
  - GNOME Shell extensions run JavaScript inside the compositor with full access to input and screen.
  - Plasma global themes, widgets (plasmoids) and KWin scripts can run arbitrary code and shell scripts.
  - Store review is limited: extensions.gnome.org reviews submissions publicly; the KDE Store said it lacks resources to review every theme.
  - An author's account takeover or a malicious update has the same effect as DS-06.
- **Real-world examples:**
  - March 2024: the KDE Store global theme "Grey Layout" ran a destructive delete that wiped a user's mounted drives. KDE warned that third-party global themes and widgets "can and will run arbitrary code" (BleepingComputer, 2024-03-21). https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/
  - Discussion of adding automated malware checks for extensions.gnome.org. https://discourse.gnome.org/t/use-virustotal-to-check-gnome-extensions-for-malicious-code/29281
  - CVE-2012-4427: the old GNOME Shell browser plugin allowed forced extension installs (historical; plugin removed since GNOME 3.32). https://security-tracker.debian.org/tracker/CVE-2012-4427
  - No confirmed malicious GNOME extension campaign was found (unverified).
- **What a defender inspects (indicators):**
  - GNOME: `gnome-extensions list --enabled --details`; `~/.local/share/gnome-shell/extensions/*/metadata.json` (uuid, version, `shell-version`, url); `gsettings get org.gnome.shell enabled-extensions`.
  - Plasma: `~/.local/share/plasma/plasmoids/`, `~/.local/share/plasma/look-and-feel/`, `~/.local/share/kwin/scripts/` and `metadata.json` / `metadata.desktop` files; shell scripts inside theme packages.
  - Recently changed extension directories.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - `bigman desktop extensions`: inventory plus baseline/diff, giving `desktop.extension-added` and `desktop.extension-updated`.
  - Run the existing `bigman inspect` static rules (`shell.delete`, `shell.download`, `shell.persistence`, `shell.privilege`) over every `*.sh` and every `Exec`/`X-KDE` script reference in Plasma packages. GNOME `.js` files get a small rule set: `GLib.spawn_command_line_*`, `Gio.Subprocess`, and `Soup` network calls to non-local hosts. These findings are "capabilities to review", not detections.
  - Add the extension directories to `bigman changes`.
- **Test case idea:** A fixture look-and-feel package containing `contents/run.sh` that only echoes, plus a never-executed text file holding a string the existing `shell.delete` rule matches. Assert `shell.delete` from the package scan. A GNOME extension fixture with a `GLib.spawn_command_line_async('true')` line produces the spawn finding.
- **References:** https://attack.mitre.org/techniques/T1176/ (Software Extensions), https://gjs.guide/extensions/review-guidelines/review-guidelines.html , https://develop.kde.org/docs/plasma/

### DS-18 MAC (AppArmor/SELinux) disabled, permissive, or user-namespace restriction off
- **What it is:** AppArmor (Ubuntu, Debian, openSUSE) and SELinux (Fedora, RHEL) confine daemons and some desktop apps. Snap and many Flatpak-adjacent protections depend on them. Users often turn them off to fix a problem and never turn them back on. Ubuntu 24.04+ also uses AppArmor to restrict unprivileged user namespaces, a common kernel-exploit prerequisite. Qualys showed three bypasses in March 2025 and recommended extra hardening.
- **Real-world examples:**
  - Qualys TRU, "three bypasses of Ubuntu unprivileged user namespace restrictions" (2025-03-27). https://blog.qualys.com/vulnerabilities-threat-research/2025/03/27/qualys-tru-discovers-three-bypasses-of-ubuntu-unprivileged-user-namespace-restrictions , advisory text https://www.qualys.com/2025/three-bypasses-of-Ubuntu-unprivileged-user-namespace-restrictions.txt
  - Ubuntu's explanation and the `kernel.apparmor_restrict_unprivileged_unconfined` sysctl. https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007
  - Q1 2026: an AppArmor confused-deputy profile-management flaw was reported on oss-security (details not re-verified). https://seclists.org/oss-sec/2026/q1/303
- **What a defender inspects (indicators):** All of these are readable without root:
  - SELinux: `getenforce`, `/sys/fs/selinux/enforce`, `sestatus`, and the `SELINUX=` value in `/etc/selinux/config`.
  - AppArmor: `aa-enabled`, `/sys/module/apparmor/parameters/enabled`, and the process's own label in `/proc/self/attr/current`.
  - `/proc/sys/kernel/apparmor_restrict_unprivileged_userns`, `/proc/sys/kernel/apparmor_restrict_unprivileged_unconfined`, and `/proc/sys/kernel/unprivileged_userns_clone` (Debian).
  - Kernel command line `/proc/cmdline` for `selinux=0`, `apparmor=0`, `enforcing=0` or `security=`.
  - The full `aa-status` profile list needs root; report it as incomplete rather than guessing.
- **Bigman today:** Partial. `bigman doctor` runs `getenforce` and stores raw output in the report details, but emits no finding and has no AppArmor check.
- **Proposed mitigation / detection:**
  - Promote this to structured findings:
    - `mac.none` (Review): neither SELinux nor AppArmor active.
    - `mac.selinux-permissive` (High)
    - `mac.selinux-config-disabled` (High)
    - `mac.disabled-cmdline` (High)
    - `mac.userns-unrestricted` (Info on distros where that is the default, Review on Ubuntu 24.04+ where it was turned off)
    - `mac.apparmor-unconfined-sysctl-off` (Info, with the Qualys reference)
  - Coverage text must say "status only; profile coverage requires root".
- **Test case idea:** A fake `/proc` and `/sys` root via an injectable path prefix: `enforce` containing `0`, cmdline containing `apparmor=0`, and the sysctl file containing `0`. Assert the findings.
- **References:** https://attack.mitre.org/techniques/T1562/001/ , https://attack.mitre.org/techniques/T1068/ , https://docs.fedoraproject.org/en-US/quick-docs/selinux-getting-started/ , https://ubuntu.com/server/docs/apparmor

### DS-19 Overtrusting Firejail, Bubblewrap and systemd-run sandboxes
- **What it is:** Ad-hoc sandboxes are useful, but each has sharp edges.
  - **Firejail** is setuid-root, which adds local privilege-escalation surface. It has had root exploits, and its profiles are allow-by-default in places.
  - **Bubblewrap** is only a primitive. Without `--new-session` or a seccomp TIOCSTI filter, the sandbox can inject keystrokes into the parent terminal. Binding the X11 socket or the session bus voids isolation.
  - **systemd-run** properties (`ProtectHome=`, `PrivateNetwork=`, and so on) are partly unavailable or weaker in `--user` mode without user namespaces, and nothing stops a confined process from asking another unconfined service to act for it over D-Bus.
  - Users read "sandboxed" as "safe".
- **Real-world examples:**
  - CVE-2022-31214: Firejail `--join` local root; fixed in 0.9.70. https://www.debian.org/security/2022/dsa-5167 , https://security-tracker.debian.org/tracker/CVE-2022-31214
  - CVE-2017-5226: Bubblewrap TIOCSTI escape; the `--new-session` flag was added. https://nvd.nist.gov/vuln/detail/CVE-2017-5226
  - systemd `systemd.exec(5)` documents which sandboxing options require privileges or user namespaces in user managers. https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html
- **What a defender inspects (indicators):**
  - `firejail --version` and `ls -l $(command -v firejail)` (setuid bit).
  - Whether `/etc/firejail/firejail.users` restricts who may run it.
  - Firecfg symlinks in `/usr/local/bin`. These silently route apps through Firejail, so report them as information.
  - `bwrap --version`.
  - `systemd-analyze --user security` for exposure scores of user units, including Bigman's own monitor unit.
- **Bigman today:** Partial. Bigman's own `bigman sandbox` and PDF tooling run Bubblewrap with `--unshare-all --die-with-parent --new-session` (verified in `src/runner.rs` and `src/inspect.rs`), and documents that this is "not a detonation VM". No check of other sandbox tools exists.
- **Proposed mitigation / detection:**
  - `bigman doctor`:
    - `sandbox.firejail-outdated` (High): below 0.9.70.
    - `sandbox.firejail-unrestricted` (Info): `firejail.users` absent, so any user can invoke the setuid binary.
    - Report `systemd-analyze --user security` scores for Bigman's own units, to keep them honest.
  - Documentation page: "What Bigman's sandbox does and does not isolate", stating that the X11 socket, session bus and network are never bound in.
  - Add a regression unit test asserting that the Bubblewrap argument list always contains `--new-session` and `--unshare-all` and never binds `/tmp/.X11-unix` or `$XDG_RUNTIME_DIR/bus`.
- **Test case idea:** The argument-list assertion above. A version comparator fixture for Firejail. Fixture `systemd-analyze` JSON output (`--json=short`).
- **References:** https://attack.mitre.org/techniques/T1611/ , https://attack.mitre.org/techniques/T1548/001/ , https://github.com/netblue30/firejail , https://github.com/containers/bubblewrap

### DS-20 Spoofed polkit/sudo/keyring password prompts
- **What it is:** Any program running as the user can draw a window that looks like a polkit "Authentication Required" dialog or a keyring unlock prompt, for example with zenity, kdialog or yad password dialogs, or a fake terminal `sudo` prompt via shell aliases. The user cannot reliably tell a real system prompt from a fake one. Once typed, the password gives the attacker root or decrypts the keyring. This is a common post-compromise step and a social-engineering staple against developers.
- **Real-world examples:**
  - Embrace The Red, "Spoofing credential dialogs on macOS, Linux and Windows" (2021): a defender-useful survey. https://embracethered.com/blog/posts/2021/spoofing-credential-dialogs
  - Ubuntu blueprint acknowledging that polkit/gksu prompts "can easily be spoofed". https://blueprints.launchpad.net/ubuntu/+spec/security-m-authentication-spoofing (URL variant unverified; a staging mirror was seen)
  - CVE-2018-1116: polkitd trusted a client-supplied UID, allowing authentication dialogs to be triggered in other users' sessions. https://www.openwall.com/lists/oss-security/2018/07/11/2
  - Detection content referencing a Linux campaign ("UNK_DeadDrop") that tricks developers into giving their system passwords (unverified secondary source).
- **What a defender inspects (indicators):**
  - Processes `zenity --password`, `kdialog --password`, `yad --entry --hide-text` or `pinentry` launched by unusual parents (browser, archive tool, `/tmp` binaries).
  - Shell rc files defining `sudo` as a function or alias (see [host integrity](./04-host-integrity-and-hardening-audit.md) for rc baselining).
  - Installer scripts that prompt for a password themselves instead of calling `sudo`/`pkexec`.
  - The real agent: `pgrep -a polkit` and the desktop's agent (gnome-shell, polkit-kde-authentication-agent-1).
- **Bigman today:** Partial. `bigman inspect` flags `shell.privilege` (sudo/pkexec) and `shell.persistence` (rc-file edits), but has no rule for password-dialog tooling. `bigman changes` snapshots `~/.bashrc`/`~/.zshrc`.
- **Proposed mitigation / detection:**
  - New static rule `shell.credential-prompt` (High) in `bigman inspect`/`bigman packages`. It matches `zenity --password`, `kdialog --password`, `yad ... --hide-text`, `read -s` combined with "password" text, and piping that into `sudo -S`.
  - Extend the rc-file diff to flag `sudo`/`su`/`pkexec` being redefined as an alias or function (`shell.sudo-shadowed`).
  - User guidance card: "Real polkit prompts name the action and dim the screen. Bigman never asks for your password in a pop-up; it only uses polkit through `bigman setup`."
  - Optional osquery-based process snapshot (existing integration) highlighting password-dialog processes with non-desktop parents.
- **Test case idea:** Fixture installer `install.sh` containing a `zenity --password` line inside a function that is never called. Assert `shell.credential-prompt`. A fixture `.bashrc` adding `alias sudo=...` against the baseline asserts `shell.sudo-shadowed`.
- **References:** https://attack.mitre.org/techniques/T1056/002/ (GUI Input Capture), https://attack.mitre.org/techniques/T1548/003/ , https://www.freedesktop.org/software/polkit/docs/latest/

### DS-21 Desktop notification spoofing and secrets on the lock screen
- **What it is:** Any same-user process can post a desktop notification with an arbitrary app name and icon through the freedesktop Notifications D-Bus interface. That makes notifications a convenient lure ("Security update required – click to install") and lets malware impersonate security tools, including Bigman's own canary alerts. Separately, notifications showing OTP codes, chat messages or password-reset links appear on the lock screen by default on some desktops.
- **Real-world examples:**
  - Fake "update" lures delivered through browser-injected pages are documented in DS-06 (QuickLens ClickFix). Native-notification abuse on Linux is a category; no dated campaign was verified.
  - Freedesktop Desktop Notifications spec: the `app_name` and `app_icon` parameters are caller-supplied. https://specifications.freedesktop.org/notification-spec/latest/
- **What a defender inspects (indicators):**
  - `gsettings get org.gnome.desktop.notifications show-in-lock-screen`, per-app `org.gnome.desktop.notifications.application` settings, and KDE `~/.config/plasmanotifyrc`.
  - Browser site notification permissions (Chromium `Preferences` `profile.content_settings.exceptions.notifications`, Firefox `permissions.sqlite`). Many sites with notification permission is a spam/lure indicator.
- **Bigman today:** Gap. Bigman uses desktop notifications for canary alerts (`canary.event`) and records an `canary.notifications-unavailable` finding.
- **Proposed mitigation / detection:**
  - `notify.lock-screen-content` (Info) when lock-screen notifications are on.
  - `browser.notification-sites` (Review) when the count of allowed notification origins exceeds a documented threshold; list them.
  - Design rule for Bigman itself: notifications only *point to* the Bigman app or report, never carry an action that installs or runs anything, and every alert is verifiable in `bigman history`. Document this so users can recognize fakes.
- **Test case idea:** A fixture Chromium `Preferences` JSON with 12 notification-allowed origins asserts the finding and lists them. A unit test asserts that Bigman's notification payload contains no actions other than "Open Bigman".
- **References:** https://attack.mitre.org/techniques/T1204/001/ , https://specifications.freedesktop.org/notification-spec/latest/

### DS-22 Unencrypted disk on a portable or shared computer
- **What it is:** Without full-disk encryption, anyone who holds the disk can read every file on it by booting another system or moving the disk to another computer: SSH and GPG keys, browser cookies and saved sessions, API tokens in dotfiles, mail. The account password does not protect data at rest. It also changes the weight of other settings: with autologin ([DS-15](#ds-15-weak-screen-lock-autologin-lock-screen-bypass-bugs)) and no encryption, switching the computer on is enough to get the session. Encryption with LUKS is chosen when installing, and adding it later means backing up and reinstalling (or a careful in-place `cryptsetup reencrypt`).
- **Real-world examples:** Physical loss and theft of laptops is the category; no dated Linux-specific incident was verified for this entry. Fedora's, Debian's and Ubuntu's installers offer LUKS as a choice during installation; whether a given computer has it is what matters, and that is readable without root.
- **What a defender inspects (indicators):**
  - `lsblk -J -o NAME,TYPE,FSTYPE,MOUNTPOINTS` (rootless): the devices mounted at `/` and `/home` should have an ancestor of TYPE `crypt` (dm-crypt/LUKS; LVM-on-LUKS shows `lvm` under `crypt`). A partition with FSTYPE `crypto_LUKS` is the container. `MOUNTPOINTS` needs util-linux 2.37 or later; older versions have `MOUNTPOINT`.
  - A swap partition outside an encrypted device leaks memory contents to disk; zram swap stays in RAM. An unencrypted `/boot` is normal.
  - Home encryption that is not whole-disk: systemd-homed (`homectl inspect`, a LUKS loop file under `/home`), fscrypt, eCryptfs.
- **Bigman today:** Partial. `harden.disk_encryption` (Info) walks `lsblk` from `/` and `/home` to a `crypt` ancestor without root and says so when the system or home folder is not encrypted; `harden.autologin` is rated Review when it is not ([hardening research](../research/hardening.md)). Encryption is set up when installing, so Bigman gives steps and never changes disks. Swap, systemd-homed and fscrypt are not checked yet.
- **Proposed mitigation / detection:** Extend the check to swap partitions outside `crypt` (Review on a laptop, since hibernation writes memory to it) and to systemd-homed and fscrypt homes, so a homed user with an unencrypted root is not told their files are exposed.
- **Test case idea:** Fixture `lsblk -J` output with `/` on `luks-<uuid>` under a `crypto_LUKS` partition (no finding) and one with `/` on a plain ext4 partition (finding); a third with LVM on LUKS (no finding).
- **References:** https://man7.org/linux/man-pages/man8/lsblk.8.html , https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system , https://gitlab.com/cryptsetup/cryptsetup

## Proposed Bigman checks (consolidated)

| Command / component | Reads (rootless) | Rule IDs | Notes |
|---|---|---|---|
| `bigman desktop session` (and `doctor` section) | env, `loginctl`, DM configs, `xhost`, `xlsclients`, portal permission store | `session.x11`, `session.x11-forced`, `session.xhost-open`, inventory of XWayland clients and portal grants | DS-01/02 |
| `bigman flatpak audit` | `metadata` keyfiles, overrides, `flatpak` CLI fallback | `flatpak.host-spawn`, `flatpak.filesystem-host`, `flatpak.filesystem-home`, `flatpak.dbus-unfiltered`, `flatpak.x11`, `flatpak.device-broad`, `flatpak.persist`, `flatpak.override-widened`, `flatpak.permission-added` | Per-permission fixed explanations; diff across updates |
| `bigman snap audit` | `snap list --all`, `snap connections` or snapd socket | `snap.classic`, `snap.devmode`, `snap.unasserted`, `snap.sensitive-interface` | DS-05 |
| Sandbox-runtime advisories (`doctor`) | package versions | `sandbox.runtime-outdated`, `sandbox.firejail-outdated`, `desktop.parser-outdated` | Pinned advisory table refreshed by the feed pipeline |
| `bigman browser extensions` | Chromium-family `Extensions/*/manifest.json`, `Preferences`, `Secure Preferences`; Firefox `extensions.json`; policy dirs | `browser.extension-added`, `browser.extension-permissions-expanded`, `browser.extension-known-bad`, `browser.extension-brand-mismatch`, `browser.extension-unpacked`, `browser.policy-forced-extension`, `browser.policy-weakened`, `browser.launcher-flags`, `browser.notification-sites` | Covers Flatpak/Snap browser profile paths too |
| MAC status (`doctor`) | `/sys/fs/selinux`, `/sys/module/apparmor`, `/proc/sys/kernel/apparmor_*`, `/proc/cmdline` | `mac.none`, `mac.selinux-permissive`, `mac.disabled-cmdline`, `mac.userns-unrestricted` | Profile list needs root and is marked incomplete |
| USB (`doctor`, monitor) | `systemctl`, sysfs, gsettings | `usb.no-policy`, `usb.automount-open`, `usb.composite-hid-storage` | `bigman setup usbguard` with an explicit device-list confirmation |
| `bigman harden` (implemented) | gsettings, `kscreenlockerrc`, GDM/SDDM/LightDM configs, `lsblk`, the `SecureBoot` EFI variable | `harden.screen_lock`, `harden.autologin`, `harden.disk_encryption`, `harden.secure_boot` | DS-15, DS-22; each suggestion has a previewed change with an undo, or manual steps |
| `bigman desktop audit` | gsettings, KDE rc files, DM configs, thumbnailer dirs, clipboard history metadata | `session.lock-disabled`, `session.autologin`, `session.timed-login`, `desktop.user-thumbnailer`, `clipboard.history-persistent`, `clipboard.history-world-readable`, `notify.lock-screen-content` | |
| `bigman desktop extensions` | GNOME/Plasma extension dirs | `desktop.extension-added`, `desktop.extension-updated`, plus `shell.*` on theme scripts | |
| `.desktop` analyzer (scan, Downloads timer, inspect, download hook) | the file | `desktop.launcher-in-downloads`, `desktop.masquerade`, `desktop.exec-download`, `desktop.hidden` | DS-09 |
| `bigman changes` additions | `mimeapps.list`, `~/.local/share/applications`, browser policy dirs, extension dirs, `/etc/udev/rules.d` | `desktop.handler-changed`, `desktop.handler-user-exec`, reuse `persistence.change` | |
| `bigman archive check` | archive (list-only) | `archive.path-traversal`, `archive.absolute-path`, `archive.symlink-escape`, `archive.special-entry`, `archive.bomb-ratio`, `archive.persistence-path`, `archive.launcher` | Optional sandboxed extract |
| Inspect rules | scripts | `shell.credential-prompt`, `shell.sudo-shadowed` | DS-20 |
| Document triage | office/PS/comic files | `document.macro`, `document.event-script`, `document.postscript-masquerade`, `document.comic-archive-names` | DS-12 |

### Safe-open workflow for downloaded documents and archives (proposal)

`bigman open FILE` is an explicit user action from the GUI, the CLI or the extension's download panel.

1. **Identify.** Compute SHA-256, sniff the type from magic bytes (not from the extension), and flag an extension/type mismatch.
2. **Static checks.** Run ClamAV (existing), then route by type:
   - `.desktop` analyzer (DS-09)
   - archive list-only check (DS-13)
   - PDF triage (existing)
   - office/PS/comic checks (DS-12)
   - `bigman inspect` for scripts
3. **Present the result honestly.** Show "findings to review", "no findings from these checks", or "incomplete". Never say "safe".
4. **Open, only if the user confirms.** Open a *copy* in a restricted viewer:
   - If a Flatpak build of the viewer is installed and DS-03 shows it without home or host filesystem access, use `flatpak run --file-forwarding` with the document portal.
   - Otherwise use Bubblewrap with `--unshare-all --new-session --die-with-parent`, a read-only system, a tmpfs home, only the copy bound read-only, no network, Wayland socket only, and no X11 or session bus. If the session is X11 (DS-01), warn that the viewer cannot be isolated from other windows.
5. **Archives.** Archives are extracted only by the sandboxed extractor into a fresh directory under `~/Downloads/bigman-extracted/<hash>/`, with symlinks and special files refused. Launchers inside are never made executable.
6. **Record.** The report (hash, type, findings, viewer used) is stored in history. Nothing is deleted or uploaded.

## Tools and scripts worth mining for rules

| Tool | URL | License | What we can borrow | Caveats |
|---|---|---|---|---|
| Flatseal | https://github.com/tchx84/Flatseal | GPL-3.0 (verify) | Permission grouping and human-readable descriptions per Flatpak permission | GUI; we only borrow descriptions with attribution |
| flatpak CLI / docs | https://docs.flatpak.org/en/latest/sandbox-permissions.html | LGPL-2.1+ (code), docs CC (verify) | Authoritative list of risky permissions and warning text | `x11` and `org.freedesktop.Flatpak` risk not stated on that page |
| snapd | https://github.com/canonical/snapd | GPL-3.0 | Interface list and the "super-privileged" interface classification | REST API shape changes between releases |
| USBGuard | https://usbguard.github.io/ | GPL-2.0+ | Rule language; `generate-policy` semantics for the setup action | Daemon policy is root-only |
| Firejail | https://github.com/netblue30/firejail | GPL-2.0 | Profile corpus lists sensitive home paths (`disable-common.inc`, `disable-programs.inc`): good for "what a Flatpak with home access could read" | setuid binary; do not depend on it |
| Bubblewrap | https://github.com/containers/bubblewrap | LGPL-2.0+ | Already used; man page warnings (TIOCSTI) for regression tests | Primitive, not a policy |
| systemd-analyze security | https://www.freedesktop.org/software/systemd/man/latest/systemd-analyze.html | LGPL-2.1+ | Exposure scoring for Bigman's own user units | `--user` mode coverage limited |
| oletools (olevba, mraptor) | https://github.com/decalage2/oletools | BSD-2-Clause | Macro detection keyword sets for OLE/OOXML | Python; run sandboxed like PDF tools |
| Chrome permission warnings | https://developer.chrome.com/docs/extensions/develop/concepts/permission-warnings | CC-BY-4.0 docs | Permission-to-risk text for the extension inventory | Text changes over time; pin a snapshot |
| MDN WebExtensions permissions | https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions | CC-BY-SA-2.5 | Firefox permission semantics | Attribution required |
| Mozilla policy-templates | https://mozilla.github.io/policy-templates/ | MPL-2.0 | List of security-relevant Firefox policy keys | |
| Chromium Linux admin docs | https://www.chromium.org/administrators/linux-quick-start/ | BSD-style / CC (verify) | Policy directory locations | Brave/Edge/Vivaldi paths differ; verify each |
| Lynis | https://github.com/CISOfy/lynis | GPL-3.0 | Existing tests for USB storage, MAC status, screen-lock-related checks; Bigman already parses its report | Shell; we borrow check ideas, not code |
| DoubleX (research) | https://github.com/Aurore54F/DoubleX | AGPL-3.0 (verify) | Data-flow patterns for dangerous extension API use (future static extension analysis) | Research-grade, JS-only |
| libarchive / bsdtar | https://github.com/libarchive/libarchive | BSD-2-Clause | Its secure-extraction flags (`ARCHIVE_EXTRACT_SECURE_*`) document the traversal checks to mirror | We should implement list-only checks natively |

## Papers and long-form references

- Joanna Rutkowska, 2011, "The Linux Security Circus: On GUI isolation", Invisible Things Lab blog. https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html. X11 lets any app snoop or inject into any other app, which undermines per-app sandboxes.
- Martin Gräßlin, 2015, "Looking at the security of Plasma/Wayland", blog. https://blog.martin-graesslin.com/blog/2015/11/. Wayland isolates clients, but the compositor and its plugins remain fully trusted.
- Nikolaos Pantelaios, Nick Nikiforakis, Alexandros Kapravelos, 2020, "You've Changed: Detecting Malicious Browser Extensions through their Update Deltas", ACM CCS 2020. https://dl.acm.org/doi/10.1145/3372297.3423343 (URL returned 403 to automated fetch; DOI not re-verified). Diffing extension updates is an effective, deterministic signal and supports DS-06's diff design.
- Aurore Fass, Dolière Francis Somé, Michael Backes, Ben Stock, 2021, "DoubleX: Statically Detecting Vulnerable Data Flows in Browser Extensions at Scale", ACM CCS 2021. https://github.com/Aurore54F/DoubleX. Static data-flow analysis of extension APIs is feasible at scale.
- Sheryl Hsu, Manda Tran, Aurore Fass, 2024, "What is in the Chrome Web Store? Investigating Security-Noteworthy Browser Extensions", ACM AsiaCCS 2024 (URL unverified). Malicious and vulnerable extensions stay listed for long periods, so local inventory matters.
- Karsten Nohl, Jakob Lell, 2014, "BadUSB – On accessories that turn evil", Black Hat USA 2014 (slide URL unverified). USB device firmware can impersonate keyboards, so device allowlisting is the defense.
- Jing (Dave) Tian, Nolen Scaife, Deepak Kumar, Michael Bailey, Adam Bates, Kevin Butler, 2018, "SoK: 'Plug & Pray' Today – Understanding USB Insecurity in Versions 1 Through C", IEEE S&P 2018 (URL unverified). A systematization of USB attacks and defenses, with context for USBGuard.
- Kevin Backhouse (GitHub Security Lab), 2023, "Coordinated Disclosure: 1-Click RCE on GNOME (CVE-2023-43641)". https://github.blog/security/vulnerability-research/coordinated-disclosure-1-click-rce-on-gnome-cve-2023-43641/. Background indexers turn downloads into an attack surface.
- Qualys TRU, 2025, "Three bypasses of Ubuntu unprivileged user namespace restrictions". https://www.qualys.com/2025/three-bypasses-of-Ubuntu-unprivileged-user-namespace-restrictions.txt. MAC-based userns restriction is defense in depth, not a wall; check the extra sysctl.
- Snyk, 2018, "Zip Slip". https://github.com/snyk/zip-slip-vulnerability. Archive traversal is cross-ecosystem; validate every entry path before writing.
- Koi Security, 2025, ShadyPanda and RedDirection reports (Koi URLs now redirect after a corporate change; use The Register coverage linked in DS-06). Store review is a one-time gate, and long-dormant extensions turn malicious via updates.

## Open questions for the maintainer

1. **Browser inventory scope.** Should `bigman browser extensions` read `Secure Preferences` (the HMAC-protected file) at all, or only manifests? Reading is harmless, but the file also contains other settings, and Bigman's privacy stance may prefer manifest-only plus `Preferences`.
2. **Known-bad extension ID lists.** Ship a curated, cited list in-repo, or keep it strictly user-imported like the URL blocklists? Shipping one improves usefulness but makes Bigman a list maintainer.
3. **USBGuard setup.** Is generating an "allow all currently attached devices" policy acceptable as a `bigman setup usbguard --apply` action, given the lock-out risk on desktops with Bluetooth or USB keyboards? Or should Bigman only report status and link to docs?
4. **Firefox support.** The Chromium extension covers link and download checks. Is a Firefox port planned? DS-06/07/08 inventory works for Firefox from the CLI regardless.
5. **Safe-open viewer choice.** Prefer the Flatpak viewer (portal-based) or Bigman's own Bubblewrap wrapper around host viewers? The latter needs per-viewer bind lists and is harder to keep working across distros.
6. **Advisory table maintenance.** Who curates the version table for flatpak, bwrap, snapd, xdg-desktop-portal, libcue, ghostscript, LibreOffice, Atril/Evince and firejail? Should it be generated from distro OVAL/OSV feeds (see the [feeds research](../research/vulnerability-intelligence-and-feeds.md)) rather than hand-written?
7. **Clipboard secret counting.** Should the opt-in "count likely secrets in clipboard history" feature exist at all, given Bigman's rule of not reading credential contents?
8. **X11 messaging.** With GNOME 50 and Plasma 6.8 Wayland-only, should `session.x11` be Review severity (default) or Info on desktops with no Wayland option (Xfce/MATE), to avoid nagging users who cannot act on it?
