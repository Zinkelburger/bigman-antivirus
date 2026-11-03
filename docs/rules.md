# Bigman rules

> Generated from `crates/core/src/rules/mod.rs`. Do not edit by hand: run `BIGMAN_UPDATE_RULES_DOC=1 cargo test -p bigman-core --test fixtures`.

Every finding Bigman shows comes from one of these rules. Each has a synthetic fixture in `fixtures/<rule-id>/` that must fire it.

| Rule | Severity | Title | Threats |
|---|---|---|---|
| `shell.download` | Review | Downloads remote content | DP-18 |
| `shell.execute-download` | High | Downloads and runs code | DP-18, PH-17 |
| `shell.privilege` | Review | Asks for administrator rights | DP-18 |
| `shell.credentials` | High | Touches credentials or browser data | CR-01, CR-02, CR-03, CR-18, SC-23 |
| `shell.persistence` | Review | Sets something up to run later | HI-02, HI-03, HI-04, HI-05, HI-10 |
| `shell.dynamic` | Review | Runs code that is built or decoded at run time | DP-18 |
| `shell.permissions` | High | Sets broad or privileged file permissions | HI-17 |
| `shell.delete` | Review | Deletes files recursively or by wildcard | DP-18 |
| `shell.network-listener` | Review | Uses a tool that can open or forward network connections | NE-14 |
| `shellcheck.warning` | Review | ShellCheck warning | DP-18 |
| `package.lifecycle` | Review | Package runs a script during install | SC-01 |
| `package.script` | Info | Package defines a script | SC-01 |
| `package.unresolved-hook` | Review | Lockfile lists an install script that is not on disk | SC-01, SC-02 |
| `trace.sensitive-path` | Review | The rehearsal touched a sensitive path | CR-18, CR-22 |
| `trace.network` | Review | The rehearsal tried to connect to the network | NE-14 |
| `trace.process` | Info | The rehearsal started a program | DP-18 |
| `pdf.structure` | Info | The PDF contains an active or external element | DS-12 |
| `clamav.match` | High | A file matches a ClamAV signature | DP-08, MW-01 |
| `clamav.no-definitions` | High | No malware signatures found | DP-20 |
| `clamav.stale` | Review | Malware signatures are out of date | DP-20 |
| `tool.missing` | Info | An optional tool is not installed |  |
| `monitor.stopped` | Review | Canary monitoring is not reporting | CR-22, HI-24 |
| `monitor.gap` | Review | Monitoring was off while the computer was awake | HI-24, CR-22 |
| `ssh.key.added` | High | A new SSH key can log in to your account | NE-05, HI-10, SC-19 |
| `ssh.tunnel.reverse` | Review | SSH is forwarding connections back to this computer | NE-20, NE-14, SC-19 |
| `ssh.login.remote` | Review | Someone logged in over the network | NE-03, NE-05, NE-20 |
| `ssh.key.stray` | Review | An SSH private key is stored outside ~/.ssh | CR-03, NE-20 |
| `git.plaintext-credentials` | Review | Git stores credentials in plain text | CR-01, CR-08 |
| `persistence.change` | Review | A startup file changed | HI-01, HI-02, HI-03, HI-04, HI-05, HI-10 |
| `persistence.owned_modified` | Review | A package's file was modified | HI-01, HI-22, DP-21, MW-08 |
| `persistence.sensitive_modified` | High | An authentication or loader file was modified | HI-08, HI-10, HI-22, MW-15 |
| `persistence.package_update` | Info | A package updated a startup file | HI-01, HI-22 |
| `persistence.privilege` | High | A startup file gained elevated privileges | HI-17, HI-18 |
| `persistence.metadata` | Review | A startup file changed permissions or owner | HI-01, HI-05, HI-19 |
| `persistence.removed` | Review | A startup file was removed | HI-01, HI-02, HI-03, HI-04, HI-05 |
| `persistence.initial` | Info | Startup file recorded for the first baseline | HI-01, HI-02, HI-03, HI-04, HI-05 |
| `receipt.hook.sensitive_path` | Review | An install script writes to a startup or authentication path | DP-09, DP-11, HI-05 |
| `receipt.hook.download` | High | An install script downloads something | DP-09, DP-11, DP-18 |
| `receipt.hook.obfuscation` | Review | An install script contains encoded or hidden code | DP-01, DP-09, DP-11 |
| `receipt.hook.changed` | Review | An install script changed in an update | DP-01, DP-09 |
| `receipt.file.privileged` | Review | A package installed a program with raised privileges | HI-17, HI-18 |
| `receipt.sensitive_package` | Info | A package that controls authentication, networking or boot changed | DP-01, DP-06 |
| `receipt.after.listener` | Review | A program started listening right after a package change | NE-01, DP-09 |
| `canary.event` | High | A decoy file was opened or changed | CR-22 |
| `canary.verified` | Info | Local read detection verified | CR-22 |
| `canary.notifications-unavailable` | Info | Desktop pop-ups are not available |  |
| `net.listener.wildcard` | Review | Listens on every network interface | NE-01, NE-22 |
| `net.listener.docker_api` | High | The container API is open on the network | NE-06, MW-03 |
| `net.listener.debugger` | High | A remote debugger is reachable from the network | NE-11 |
| `net.listener.lan_service` | Review | A local AI or notebook server is reachable from the network | NE-09, NE-10 |
| `denial.selinux` | Info | SELinux stopped or logged an access | DS-18 |
| `denial.apparmor` | Info | AppArmor stopped or logged an access | DS-18 |
| `harden.sshd` | Review | The SSH server is on | NE-01, NE-02, NE-03, NE-05, NE-20 |
| `harden.service.cups_browsed` | Review | cups-browsed adds printers from the network | NE-17 |
| `harden.service.network` | Review | A network service a desktop rarely needs is on | NE-01, NE-22 |
| `harden.firewall.off` | Review | No firewall is running | NE-12, NE-01 |
| `harden.firewall.open_range` | Review | The firewall opens every high port | NE-12, NE-01, NE-11 |
| `harden.sysctl.ptrace_scope` | Review | Programs can read each other's memory | CR-07, HI-25 |
| `harden.sysctl.kptr_restrict` | Info | Kernel addresses are visible to programs | HI-25 |
| `harden.sysctl.dmesg_restrict` | Info | Every program can read the kernel log | HI-25 |
| `harden.name.llmnr` | Info | This computer asks the local network for names | NE-16 |
| `harden.screen_lock` | Review | The screen does not lock by itself | DS-15 |
| `harden.autologin` | Review | Someone is logged in automatically at startup | DS-15 |
| `harden.disk_encryption` | Info | The disk is not encrypted | DS-22 |
| `harden.secure_boot` | Info | Secure Boot is off | HI-21, DP-23 |
| `harden.group.docker` | Review | Your account is in the docker group | HI-16 |
| `harden.home.readable` | Review | Other accounts can open your home folder | CR-10 |
| `harden.npm.ignore_scripts` | Info | npm runs dependency install scripts | SC-01 |
| `lynis.finding` | Review | Lynis recommendation | HI-24 |
| `osv.advisory` | Review | A dependency has a published advisory | SC-05, SC-10 |
| `yara.output` | Review | YARA-X rules matched | MW-01 |

## `shell.download`

**Downloads remote content** (Review)

The script fetches something from the network with curl or wget. What it fetches is not part of this inspection.

What you can do: Inspect each downloaded payload and confirm where it comes from before running the script.

## `shell.execute-download`

**Downloads and runs code** (High)

The script pipes downloaded content straight into a shell, so whatever the server sends at that moment runs on your machine.

What you can do: Download the payload separately, inspect the exact bytes, and run only what you reviewed.

## `shell.privilege`

**Asks for administrator rights** (Review)

The script runs commands with sudo, doas or su. Those commands can change anything on the system.

What you can do: Check why each privileged command is needed and whether it could run without administrator rights.

## `shell.credentials`

**Touches credentials or browser data** (High)

The script refers to SSH keys, cloud or registry tokens, or browser profile data. Installers rarely need these.

What you can do: Run it without your real credentials present, and find out why it reads these paths.

## `shell.persistence`

**Sets something up to run later** (Review)

The script writes cron jobs, systemd units, autostart entries, shell profiles or SSH authorized keys, so its effect outlasts the install.

What you can do: Review exactly what will run in future sessions and compare the startup files afterwards.

## `shell.dynamic`

**Runs code that is built or decoded at run time** (Review)

The script uses eval, source, base64 or an interpreter on generated input. Static rules cannot see what that code will be.

What you can do: Decode or expand the dynamic part by hand and inspect it before running the script.

## `shell.permissions`

**Sets broad or privileged file permissions** (High)

The script makes files world-writable or setuid. Either can let other programs or users gain access they should not have.

What you can do: Prefer the narrowest permissions the program needs, and never setuid a file you did not build.

## `shell.delete`

**Deletes files recursively or by wildcard** (Review)

The script removes files with rm -r, rm -f or a wildcard. A wrong variable can expand to the wrong path.

What you can do: Check what each deleted path expands to before running the script.

## `shell.network-listener`

**Uses a tool that can open or forward network connections** (Review)

The script runs nc, ncat, netcat or socat, which can listen for connections or relay a shell.

What you can do: Check which address and port it uses, and whether it listens to the whole network.

## `shellcheck.warning`

**ShellCheck warning** (Review)

ShellCheck found a shell correctness problem. It is about how the script is written, not whether it is malicious.

What you can do: Read the ShellCheck code linked in the message to see what can go wrong.

## `package.lifecycle`

**Package runs a script during install** (Review)

A preinstall, install, postinstall or prepare script runs automatically when the package is installed.

What you can do: Read the script and the files it calls. Turning off automatic install scripts for the project stops them running.

## `package.script`

**Package defines a script** (Info)

A script that runs only when you ask for it, for example npm test.

What you can do: Read it before you run it.

## `package.unresolved-hook`

**Lockfile lists an install script that is not on disk** (Review)

The lockfile says a dependency has an install script, but the package is not installed here, so its script could not be read.

What you can do: Inspect that package's archive without running its scripts.

## `trace.sensitive-path`

**The rehearsal touched a sensitive path** (Review)

While running in the offline sandbox, the installer opened or looked for credential or browser paths. The sandbox has no real ones.

What you can do: Check the system call result; the access may have failed. Find out why the installer wanted that path.

## `trace.network`

**The rehearsal tried to connect to the network** (Review)

While running in the offline sandbox, the installer attempted a network connection. The sandbox has no network.

What you can do: Find out what it was trying to reach and why.

## `trace.process`

**The rehearsal started a program** (Info)

A program started while the installer ran in the offline sandbox.

What you can do: Review the program and its arguments.

## `pdf.structure`

**The PDF contains an active or external element** (Info)

The document has an open action, JavaScript, a launch action, an embedded file, a link or encryption. JavaScript and launch actions are Review; the rest are Info.

What you can do: Check the parser output and the link destinations. A tag alone is not evidence of malware.

## `clamav.match`

**A file matches a ClamAV signature** (High)

ClamAV recognised the file as matching a known-malware or heuristic signature. Limit or encryption signatures mean the content was not fully examined.

What you can do: If you did not expect the file, quarantine it; Bigman keeps a recovery copy you can restore.

## `clamav.no-definitions`

**No malware signatures found** (High)

Neither Bigman's private signature folder nor /var/lib/clamav has ClamAV definitions, so file scans cannot run.

What you can do: Update signatures from Protect > Scans, or point ClamAV at your own database.

## `clamav.stale`

**Malware signatures are out of date** (Review)

The newest ClamAV definitions file is more than three days old. File age is a freshness signal, not a verified version.

What you can do: Update signatures from Protect > Scans.

## `tool.missing`

**An optional tool is not installed** (Info)

One of the tools Bigman can use is not installed. Only the checks that need it are affected.

What you can do: Install it from Setup if you want those checks.

## `monitor.stopped`

**Canary monitoring is not reporting** (Review)

The canary monitor has not written a heartbeat in the last ten seconds, so reads of decoy files are not being noticed. The background service also reports this live, once, when the heartbeat has been missing for a minute while the computer was awake.

What you can do: Start monitoring from Protect > Canaries, or install the background services from Setup.

## `monitor.gap`

**Monitoring was off while the computer was awake** (Review)

The canary monitor or Bigman's background service was not running for a while, so nothing was watching in that window. Suspended time does not count, and a reboot starts over. A background-service gap after a normal stop (logout, systemctl --user stop) is Info; one after a crash or kill is Review. Malware running as you can stop these processes, which is why the gap itself is shown.

What you can do: If you did not stop monitoring, review Activity and the startup files for changes made in that window.

## `ssh.key.added`

**A new SSH key can log in to your account** (High)

A key appeared in ~/.ssh/authorized_keys that was not there before. Whoever holds the matching private key can log in as you wherever sshd accepts it, without your password. The key's fingerprint and comment are shown, never the key itself; fingerprints published as indicators of compromise are named.

What you can do: If you did not add it, remove its line, then check ~/.ssh, your crontab and startup files for anything added at the same time. Turn sshd off if you do not need remote logins.

## `ssh.tunnel.reverse`

**SSH is forwarding connections back to this computer** (Review)

One of your ssh or autossh processes uses -R or RemoteForward, so connections to a port on another computer arrive here. Developers do this on purpose. It is High when it forwards to this computer's SSH server, turns off host-key checking, or was started from a code editor, as the October 2026 tampered Cursor extension did.

What you can do: If you did not start it, stop the process, then look for what restarts it (crontab -l, ~/.config/systemd/user, autostart) and check ~/.ssh/authorized_keys.

## `ssh.login.remote`

**Someone logged in over the network** (Review)

The login manager reports a session that came over the network, usually through sshd. It is High when it came from this computer's own address, which is how logins through a reverse tunnel arrive (or a deliberate ssh to localhost).

What you can do: If this was not you, end the session with loginctl terminate-session, remove keys you do not recognise from ~/.ssh/authorized_keys, and change your password.

## `ssh.key.stray`

**An SSH private key is stored outside ~/.ssh** (Review)

A file directly inside a temporary folder, ~/.config, ~/.cache or ~/.local/share starts like an SSH private key. Tools rarely keep keys there; droppers do, to authenticate a tunnel. Only the first line is read.

What you can do: If no tool you use put it there, keep a copy for investigation, look for an ssh or autossh process using it, then delete it.

## `git.plaintext-credentials`

**Git stores credentials in plain text** (Review)

Git's global credential helper is 'store', which writes tokens unencrypted to ~/.git-credentials. The file itself was not read.

What you can do: Switch to your desktop keyring's credential helper and revoke tokens you no longer use.

## `persistence.change`

**A startup file changed** (Review)

A file that starts programs for you (a systemd unit, autostart entry, shell profile, cron file) is new or different from the baseline you accepted.

What you can do: Review the file and the package that owns it before accepting it into the baseline.

## `persistence.owned_modified`

**A package's file was modified** (Review)

A startup or configuration file that a package installed no longer matches the package database. Configuration files are often edited on purpose; other files are not expected to change.

What you can do: Check the change is one you or your administrator made; otherwise reinstall the package.

## `persistence.sensitive_modified`

**An authentication or loader file was modified** (High)

A PAM, dynamic-loader or SSH file installed by a package no longer matches the package database. These files decide who can log in and what code every program loads.

What you can do: Compare it with the package's version (rpm -V, dpkg --verify or pacman -Qkk) and restore it if you did not make the change.

## `persistence.package_update`

**A package updated a startup file** (Info)

A startup or configuration file changed, and the package database shows it is exactly what the owning package installed.

What you can do: Nothing to do; the update explains the change.

## `persistence.privilege`

**A startup file gained elevated privileges** (High)

A startup or configuration file now has the setuid or setgid bit or file capabilities, which lets it run with more rights than the user who starts it.

What you can do: Remove the setuid bit or capability unless you know why it is there.

## `persistence.metadata`

**A startup file changed permissions or owner** (Review)

A startup or configuration file has the same content as before but a different mode or owner.

What you can do: Check why before accepting the new baseline; a writable startup file lets other users change what runs for you.

## `persistence.removed`

**A startup file was removed** (Review)

A startup file in your baseline is gone.

What you can do: Check whether an update or your own change explains it.

## `persistence.initial`

**Startup file recorded for the first baseline** (Info)

There is no baseline yet, so every startup file is listed for review.

What you can do: Review the list, then accept it as the baseline to compare future changes against.

## `receipt.hook.sensitive_path`

**An install script writes to a startup or authentication path** (Review)

A package's install script writes to a path that decides what runs at login or boot, or how you authenticate (profile.d, PAM, sudoers, cron, systemd, udev, the loader, SSH, home directories).

What you can do: Read the line. Packages rarely need to change these paths from a script; if you did not expect it, reinstall from your distribution's repository or remove the package.

## `receipt.hook.download`

**An install script downloads something** (High)

A package's install script calls curl, wget or a similar tool. Distribution packages ship their files, so a download at install time runs code nobody reviewed.

What you can do: Find out what it downloads before trusting the package.

## `receipt.hook.obfuscation`

**An install script contains encoded or hidden code** (Review)

A package's install script uses base64 decoding, eval, a long encoded blob, escaped bytes or invisible Unicode, which hide what it does.

What you can do: Decode the line by hand, never by running it, and check what it does.

## `receipt.hook.changed`

**An install script changed in an update** (Review)

A package's install script differs from the one recorded in an earlier receipt for the same package.

What you can do: Read what the script does now; a changed hook in an ordinary update is worth a look.

## `receipt.file.privileged`

**A package installed a program with raised privileges** (Review)

An installed or updated package ships a file with the setuid or setgid bit or file capabilities, so it runs with more rights than the user who starts it.

What you can do: Check this is expected for the package.

## `receipt.sensitive_package`

**A package that controls authentication, networking or boot changed** (Info)

One of the packages that decide how this computer authenticates, connects or boots (openssh, sudo, PAM, systemd, glibc, xz, curl, OpenSSL, the kernel, shells) was installed or updated.

What you can do: Nothing to do when the update came from your distribution; it is listed so the change is easy to find later.

## `receipt.after.listener`

**A program started listening right after a package change** (Review)

Within a minute of a package transaction, a program began accepting network connections.

What you can do: Check whether the package is meant to run a server, and who can reach it (Network > Listening programs).

## `canary.event`

**A decoy file was opened or changed** (High)

A canary decoy was read, changed, moved or removed while the monitor was running. inotify cannot say which program did it.

What you can do: Check what was running at that time. Backups, indexers and your own tests can also trigger decoys.

## `canary.verified`

**Local read detection verified** (Info)

A deliberate read of the decoy produced a fresh receipt from the running monitor, so local detection works.

What you can do: Leave monitoring running.

## `canary.notifications-unavailable`

**Desktop pop-ups are not available** (Info)

notify-send is not installed, so canary alerts appear in Activity but not as pop-ups.

What you can do: Install your distribution's desktop notification utilities if you want pop-ups.

## `net.listener.wildcard`

**Listens on every network interface** (Review)

A program accepts connections on all of this computer's network addresses, so other machines on the same network can reach it.

What you can do: If only this computer uses it, make it listen on 127.0.0.1, or stop it.

## `net.listener.docker_api`

**The container API is open on the network** (High)

Docker's API is listening on TCP. Whoever reaches it can start containers with root access to this machine.

What you can do: Use the local socket instead, or require TLS client certificates.

## `net.listener.debugger`

**A remote debugger is reachable from the network** (High)

A debugger port (Node.js inspector, Chromium DevTools, debugpy) is open beyond this computer. Debuggers let whoever connects run code in the program.

What you can do: Bind the debugger to 127.0.0.1, or stop it when you are done.

## `net.listener.lan_service`

**A local AI or notebook server is reachable from the network** (Review)

A server that usually has no login (Ollama, Jupyter, Gradio and similar) listens beyond this computer.

What you can do: If only you use it, bind it to 127.0.0.1.

## `denial.selinux`

**SELinux stopped or logged an access** (Info)

SELinux refused a program something its policy does not allow, or logged it when the program runs in a permissive domain. This is the system's own defense working.

What you can do: If the program needed this to work, a local policy module can allow it; Bigman shows the record and never changes policy for you.

## `denial.apparmor`

**AppArmor stopped or logged an access** (Info)

AppArmor refused a confined program something its profile does not allow, or logged it in complain mode.

What you can do: If the program needed this to work, its profile can be extended; Bigman shows the record and never changes profiles for you.

## `harden.sshd`

**The SSH server is on** (Review)

sshd accepts logins from other computers. On a desktop it is often turned on once and forgotten, and it is the way in that the October 2026 tampered editor extension used through a reverse tunnel. Review when other computers can reach it or no firewall is running, Info when the firewall blocks it or you logged in over the network recently.

What you can do: If you do not log in to this computer remotely, turn it off; the undo turns it back on. Bigman will not turn it off while you are logged in over the network.

## `harden.service.cups_browsed`

**cups-browsed adds printers from the network** (Review)

cups-browsed listens for printer announcements from the whole network. Its 2024 vulnerabilities (CVE-2024-47176 and related) let a device on the network add a printer that runs commands when you print. Review when other computers can reach UDP 631.

What you can do: Turn it off unless you rely on network printers appearing by themselves; printers already set up keep working.

## `harden.service.network`

**A network service a desktop rarely needs is on** (Review)

rpcbind, Samba, an FTP or telnet server, xrdp or Cockpit is enabled. Each accepts connections from other computers. Review when the firewall lets other computers reach it, Info otherwise.

What you can do: Turn off the ones you do not use; the undo turns each back on.

## `harden.firewall.off`

**No firewall is running** (Review)

Neither firewalld nor ufw is active, so any program that listens on the network is reachable from every network this computer joins. Review when something is listening, Info when nothing is yet.

What you can do: Turn on the firewall your distribution ships; Bigman can do it for firewalld or ufw.

## `harden.firewall.open_range`

**The firewall opens every high port** (Review)

A firewalld zone on an active network allows a wide range of ports, like Fedora Workstation's 1025-65535, so development servers, debuggers and anything else listening on a high port are reachable. Review when a program is reachable only because of the range, Info otherwise.

What you can do: Close the range and allow the few ports you need by name.

## `harden.sysctl.ptrace_scope`

**Programs can read each other's memory** (Review)

kernel.yama.ptrace_scope is 0, so any program running as you can attach to your other programs and read secrets out of their memory.

What you can do: Set it to 1: programs can still debug what they start, and sudo is needed to attach to anything else.

## `harden.sysctl.kptr_restrict`

**Kernel addresses are visible to programs** (Info)

kernel.kptr_restrict is 0, so kernel memory addresses are shown to every program, which helps local kernel exploits.

What you can do: Set it to 1; profilers then need sudo for kernel symbols.

## `harden.sysctl.dmesg_restrict`

**Every program can read the kernel log** (Info)

kernel.dmesg_restrict is 0. The kernel log can reveal addresses and hardware details useful to an exploit.

What you can do: Set it to 1; dmesg then needs sudo.

## `harden.name.llmnr`

**This computer asks the local network for names** (Info)

systemd-resolved uses LLMNR, so names missing from DNS are asked of every device on the local network, and any of them can answer. On public Wi-Fi that is used to capture sign-ins and redirect connections.

What you can do: Turn LLMNR off with a resolved drop-in; .local names and DNS keep working.

## `harden.screen_lock`

**The screen does not lock by itself** (Review)

GNOME or KDE Plasma is set never to lock when you step away, or not to lock after sleep.

What you can do: Turn automatic locking on; Bigman sets a five-minute delay that you can change in your desktop's settings.

## `harden.autologin`

**Someone is logged in automatically at startup** (Review)

The display manager logs an account in without a password when the computer starts. Review when the disk is also unencrypted, Info otherwise.

What you can do: Turn automatic login off in your desktop's user or login-screen settings.

## `harden.disk_encryption`

**The disk is not encrypted** (Info)

The system or home folder is not on an encrypted device, so anyone holding the disk can read your files and keys.

What you can do: Encryption is set up when installing; back up and reinstall with encryption on.

## `harden.secure_boot`

**Secure Boot is off** (Info)

The firmware starts any boot loader and kernel, so boot-level malware can load before Linux.

What you can do: Turn it on in the firmware setup, after enrolling the signing key of any out-of-tree kernel module you use.

## `harden.group.docker`

**Your account is in the docker group** (Review)

Members of the docker group can start a container that mounts the whole disk, which is administrator access without a password, for you and for any program you run.

What you can do: Leave the group and use sudo, rootless Docker or Podman; the undo adds you back.

## `harden.home.readable`

**Other accounts can open your home folder** (Review)

Your home folder lets other accounts on this computer list or open it. Review when other people's accounts exist, Info otherwise.

What you can do: Remove access for others (chmod o-rwx); the undo restores the previous mode.

## `harden.npm.ignore_scripts`

**npm runs dependency install scripts** (Info)

npm runs preinstall, install and postinstall scripts from every dependency during npm install. Self-spreading npm worms in 2025 ran from those scripts.

What you can do: Set ignore-scripts=true in your user .npmrc and rebuild only the packages that need it.

## `lynis.finding`

**Lynis recommendation** (Review)

Lynis reported a warning or suggestion. The line is shown exactly as Lynis wrote it.

What you can do: Look up the Lynis test ID before changing configuration.

## `osv.advisory`

**A dependency has a published advisory** (Review)

OSV lists a known vulnerability for a version this project depends on. A vulnerability is not evidence of malware.

What you can do: Read the advisory's affected and fixed versions and update the dependency.

## `yara.output`

**YARA-X rules matched** (Review)

Rules you supplied matched the target. A match is the result of those rules, not proof of malware.

What you can do: Check where the rules came from and what they look for.
