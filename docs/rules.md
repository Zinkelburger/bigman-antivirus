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
| `persistence.shadowed-command` | High | A program in your own bin folder hides a system command | HI-19, SC-24 |
| `persistence.extension-new` | Review | A new editor extension | SC-19, SC-24 |
| `persistence.extension-updated` | Info | An editor extension was updated | SC-19 |
| `persistence.extension-modified` | High | An editor extension's code changed without an update | SC-19, SC-24 |
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
| `advisory.update-available` | Review | A security update is available | DP-20, HI-23, NE-04 |
| `advisory.exploited` | High | A security update fixes a vulnerability known to be exploited | DP-20, HI-23, MW-20 |
| `advisory.no-fix-yet` | Info | An installed package is affected and has no fix yet | DP-20, HI-23 |
| `advisory.reboot-required` | Review | A restart finishes installing updates | HI-23 |
| `advisory.untracked-package` | Info | Packages outside the distribution's security data | DP-10, DP-11, DP-12 |
| `advisory.unsupported-release` | Review | This release no longer receives security updates | DP-20 |
| `repo.autorun-task` | Review | Opening the folder runs a task | SC-20, PH-10 |
| `repo.envrc` | Review | direnv runs a script when you enter the folder | SC-20 |
| `repo.devcontainer-command` | Review | The devcontainer runs commands | SC-20, PH-10 |
| `repo.git-filter` | Review | The project names a git filter | SC-20 |
| `repo.git-hooks` | Review | Git hooks point into the project | SC-20, PH-10 |
| `repo.git-config-command` | Review | This copy's git settings run a program | SC-20, PH-10 |
| `repo.ai-hooks` | Review | AI tool settings in the project run commands | SC-21, CR-19 |
| `repo.ai-settings` | Review | The project loosens the AI tool's safeguards | SC-21, CR-19, PH-10 |
| `repo.instructions-run-download` | Review | An AI instruction file says to download and run code | SC-21, CR-19, PH-10 |
| `repo.code-loader` | High | The project runs text it fetches or decodes as code | SC-24, PH-10 |
| `repo.encoded-url` | Review | A web address is stored encoded | SC-24, PH-10 |
| `repo.payload-host` | Review | The project uses a JSON storage service lures load code from | SC-24, PH-10 |
| `repo.disguised-asset` | High | A font, image or media file is really text | SC-24, SC-20, PH-10 |
| `repo.mcp-server` | Review | The project configures an MCP server | SC-21, SC-22 |
| `repo.hidden-instructions` | High | An AI instruction file contains invisible characters | SC-21, CR-19 |
| `repo.workflow-unpinned` | Review | A workflow uses an action by tag | SC-15 |
| `repo.workflow-pull-request-target` | Review | A workflow uses pull_request_target | SC-16 |
| `skill.runs-on-load` | Review | A skill runs shell commands as soon as it is used | SC-25, SC-21 |
| `skill.hooks` | Review | A skill or plugin registers hooks | SC-25, SC-21 |
| `skill.allowed-tools` | Review | A skill approves broad tool use in advance | SC-25, SC-21 |
| `skill.mcp-server` | Review | A plugin starts or connects to MCP servers | SC-25, SC-22 |
| `skill.background-command` | Review | A plugin starts programs in the background | SC-25 |
| `skill.bin-on-path` | Review | A plugin adds programs to the agent's PATH | SC-25, HI-19, MW-23 |
| `skill.binary` | Review | A skill ships a compiled program or an archive | SC-25, MW-24 |
| `skill.code-plugin` | Review | OpenCode loads plugin code at every start | SC-25, SC-22 |
| `skill.npm-plugin` | Review | OpenCode installs and loads an npm plugin | SC-25, SC-01, SC-22 |
| `skill.download-and-run` | High | A skill's instructions download and run a program | SC-25, DP-18 |
| `skill.credential-paths` | Review | A skill refers to credential files | SC-25, SC-23, CR-18 |
| `skill.exfil-endpoint` | High | A skill names a data-collection address | SC-25, SC-23 |
| `skill.encoded-blob` | Review | A skill contains encoded text | SC-25, DP-18 |
| `skill.secrecy` | Review | A skill asks the agent to keep things from you | SC-25, SC-21 |
| `skill.memory-write` | Review | A skill changes agent memory or settings files | SC-25, SC-21, HI-05 |
| `skill.html-comment` | Review | A skill has text hidden in an HTML comment | SC-25, SC-21 |
| `skill.script-exfil` | High | A skill's code reads credentials and connects out | SC-25, SC-23 |
| `skill.changed` | Review | An approved skill or plugin changed | SC-25, SC-05, DS-06 |
| `skill.unreviewed` | Info | Skills and plugins not reviewed yet | SC-25 |
| `lockfile.non-registry` | Review | A dependency comes from outside the registry | SC-02, SC-13 |
| `lockfile.missing-integrity` | Review | A dependency has no integrity hash | SC-13 |
| `lockfile.install-hook` | Info | A dependency runs an install script | SC-01 |
| `lockfile.new-install-hook` | Review | A new dependency runs an install script | SC-01, SC-06, SC-09 |
| `lockfile.source-changed` | High | A dependency changed source without a new version | SC-02, SC-13 |
| `channel.signature-disabled` | High | A package source skips signature checks | DP-02 |
| `channel.plain-http` | Info | A package source uses plain HTTP | DP-02, DP-05 |
| `channel.unscoped-key` | Review | A third-party apt source has no signed-by key | DP-03 |
| `channel.global-key` | Review | A third-party key is trusted for every apt source | DP-03 |
| `channel.expiry-check-off` | Review | An apt source accepts expired metadata | DP-05 |
| `channel.stale-metadata` | Review | Package lists are more than two weeks old | DP-05 |
| `channel.priority-override` | Review | A third-party source outranks the distribution | DP-22, DP-12 |
| `channel.personal-repo` | Info | Personal repositories are configured | DP-12 |
| `channel.third-party` | Info | Third-party package sources are configured | DP-10, DP-22 |
| `archive.absolute-path` | High | Archive entries are named with an absolute path | DS-13 |
| `archive.path-traversal` | High | Archive entries climb out of the extraction folder | DS-13 |
| `archive.link-escape` | High | Links in the archive point outside it | DS-13 |
| `archive.special-entry` | Review | The archive holds devices, pipes or setuid programs | DS-13, HI-17 |
| `archive.startup-path` | Review | Archive entries would land in startup or login files | DS-13, HI-02, HI-03, HI-04, HI-05, HI-09, HI-10 |
| `archive.bomb-ratio` | Review | The archive expands to far more than its size | DS-13 |
| `package.gyp-build` | Review | An npm package builds native code at install | SC-01 |
| `package.bin-swapped` | High | A command in node_modules/.bin was not put there by the package manager | SC-24, HI-19 |
| `py.startup-code` | Review | A Python package runs code at every start | SC-03 |
| `py.entrypoint-shadow` | Review | A Python package installs a command named like a common one | SC-03 |
| `py.setup-exec` | Review | Installing a Python source package runs code that reaches out | SC-03 |
| `rust.build-script` | Review | A Rust crate runs a build script | SC-04 |
| `rust.proc-macro` | Info | A Rust crate is a procedural macro | SC-04 |
| `vsix.startup-activation` | Review | A VS Code extension starts with the editor | SC-19 |
| `vsix.dependency-chain` | Review | A VS Code extension installs other publishers' extensions | SC-19 |
| `vsix.invisible-code` | High | A VS Code extension hides text in invisible characters | SC-19 |
| `vsix.native-binary` | Info | A VS Code extension ships native programs | SC-19 |
| `pkg.install-script` | Review | A package runs a script as root when installed | DP-08, DP-09, DP-11 |
| `pkg.adds-source` | Review | Installing the package adds a package source | DP-02, DP-08, DP-22 |
| `pkg.adds-key` | Review | Installing the package trusts a signing key | DP-03, DP-08 |
| `pkg.setuid` | Review | The package installs a program with raised privileges | DP-08, HI-17 |
| `pkg.startup-path` | Review | The package installs files that run at boot or login | DP-08, HI-01, HI-03, HI-04, HI-05, HI-08, HI-09 |
| `pkg.user-path` | High | The package puts files in home or temporary folders | DP-08 |
| `pkgbuild.skip-checksum` | Review | A PKGBUILD download is not checked against a checksum | DP-11 |
| `appimage.unsigned` | Info | The AppImage is not signed | DP-17 |
| `appimage.update-url` | Info | The AppImage can update itself | DP-17 |
| `desktop.launcher` | Review | This is a program launcher, not a document | DS-09 |
| `desktop.masquerade` | High | A program launcher is made to look like a file | DS-09 |
| `desktop.exec-download` | High | A program launcher downloads something when opened | DS-09, DP-18 |
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
| `hidden.process` | High | A running program is missing from the process list | MW-16, MW-02, HI-20 |
| `hidden.module` | High | A kernel module is in one list and not another | MW-16, HI-20 |
| `hidden.net-socket` | High | A socket is in one kernel table and not the other | MW-16, MW-15, NE-01, NE-13 |
| `hidden.libc-view` | High | The C library gives Bigman a different answer than the kernel | MW-15, HI-09, MW-02 |
| `linker.preload-present` | High | A library is loaded into every program | HI-09, MW-15, MW-02 |
| `module.taint` | Info | The kernel carries taint marks | HI-20, MW-16 |
| `module.no-file` | Review | A kernel module was loaded from outside the kernel's module folder | HI-20, MW-16, HI-11 |
| `bpf.programs` | Info | BPF programs are loaded in the kernel | MW-17, MW-10 |
| `process.fake-kernel-thread` | High | A program is named like a kernel thread | MW-23, MW-19 |
| `process.exe-deleted` | Review | A program is running and its file was deleted | MW-22, MW-02 |
| `process.exe-memfd` | Review | A program runs from memory with no file on disk | MW-22, MW-16 |
| `process.exe-shm` | Review | A program runs from shared memory | MW-22, MW-01 |
| `process.daemon-name` | Review | A program has a system service's name and runs from somewhere else | MW-23, MW-09 |
| `process.shell-socket` | High | A shell or script interpreter is wired to a network connection | NE-14, NE-20 |
| `process.preload` | Info | Programs run with a library loaded first | HI-09, MW-15 |
| `net.packet-socket` | Info | A program reads network frames directly | MW-17, MW-21 |
| `net.raw-socket` | Info | A program sends and receives raw IP packets | MW-17, NE-13 |
| `watch.credentials` | Review | A program is told when your credential files change | CR-22, CR-18, CR-03 |
| `watch.own-program` | Review | A program watches its own file | MW-06, MW-02 |
| `respawn.program-restart` | Review | A program you stopped was started again by another program | MW-02, MW-01, MW-06 |
| `respawn.orphan-restart` | Review | A program you stopped was started again by something that let go of it | MW-02, MW-01 |
| `respawn.service-restart` | Info | The service manager started a program you stopped again |  |
| `respawn.not-restarted` | Info | Nothing started a program you stopped again |  |
| `respawn.did-not-stop` | Info | A program you asked to stop kept running |  |
| `bus.name-owner` | Review | An unexpected program answers for your password store or portal | CR-17, DS-20 |

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

A file that starts programs for you (a systemd unit, autostart entry, shell profile, cron file), a file that starts one on an event (a D-Bus service file, a systemd generator, a network dispatcher script), or a program in a developer tool folder (npm's global prefix, nvm, pnpm, ~/.cargo/bin, ~/go/bin), is new or different from the baseline you accepted.

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

## `persistence.shadowed-command`

**A program in your own bin folder hides a system command** (High)

A file in ~/.local/bin or ~/bin has the same name as a system command (git, ssh, sudo, node), so your shell runs it instead. A new or changed one is rated High: swapping in a poisoned copy of a familiar tool is how a project's setup step can outlast the project. One already there when the inventory was first taken is Review.

What you can do: Read the file. If you did not put it there, delete it and check what ran just before it appeared.

## `persistence.extension-new`

**A new editor extension** (Review)

An extension appeared in VS Code, VSCodium, Cursor or Windsurf since the baseline. Extensions run with your permissions whenever the editor starts, and a project's setup step or a stolen publisher account can install one.

What you can do: If you did not install it, remove it in the editor and check what ran just before it appeared.

## `persistence.extension-updated`

**An editor extension was updated** (Info)

An extension's version changed since the baseline, which is what the editor's own updates do.

What you can do: Nothing to do if the editor updates its extensions.

## `persistence.extension-modified`

**An editor extension's code changed without an update** (High)

The code an installed extension loads (its main file) changed while its version stayed the same. Editors install updates as new versions and never rewrite an installed one, so something else edited it, which is how GlassWorm-style campaigns spread through installed extensions.

What you can do: Quit the editor, reinstall the extension from its marketplace, and check what ran just before the change.

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

## `advisory.update-available`

**A security update is available** (Review)

Your distribution has published a fixed version of an installed package. The distribution's own security data decides this, so backported fixes are counted correctly.

What you can do: Install the update with the command shown; Bigman never runs it for you.

## `advisory.exploited`

**A security update fixes a vulnerability known to be exploited** (High)

A pending update fixes a vulnerability listed in CISA's Known Exploited Vulnerabilities catalog, which records flaws seen exploited in the wild.

What you can do: Install this update before the others.

## `advisory.no-fix-yet`

**An installed package is affected and has no fix yet** (Info)

Your distribution marks an installed package as affected by a vulnerability it has not fixed yet. It becomes Review when the vulnerability is being exploited.

What you can do: Nothing to install yet. Avoid the affected feature where you can.

## `advisory.reboot-required`

**A restart finishes installing updates** (Review)

A newer kernel is installed than the one running, or the distribution flagged that a restart is needed, so some fixes are not in effect yet.

What you can do: Restart when convenient.

## `advisory.untracked-package`

**Packages outside the distribution's security data** (Info)

These installed packages came from outside the distribution's repositories (the AUR, a vendor repository, a local file), so its security data cannot say whether they need updates.

What you can do: Follow each project's own releases for security updates.

## `advisory.unsupported-release`

**This release no longer receives security updates** (Review)

The distribution's security data has nothing for this release, which usually means it has reached its end of life.

What you can do: Upgrade to a supported release.

## `repo.autorun-task`

**Opening the folder runs a task** (Review)

A VS Code task in the project is set to run when the folder opens (runOn: folderOpen). A task that hides its output, or downloads and runs code, is rated High: fake job interviews use exactly that.

What you can do: Read the task before opening the folder in a trusted window.

## `repo.envrc`

**direnv runs a script when you enter the folder** (Review)

The project has an .envrc, which direnv runs as a shell script in every shell that enters the folder once it is allowed.

What you can do: Read it before running `direnv allow`.

## `repo.devcontainer-command`

**The devcontainer runs commands** (Review)

The devcontainer configuration runs commands when the container is created or started. initializeCommand runs on this computer, not in the container, and is rated High.

What you can do: Read the commands before reopening the folder in a container.

## `repo.git-filter`

**The project names a git filter** (Review)

A .gitattributes entry names a filter driver, which runs a program on checkout or commit once your git configuration defines it.

What you can do: Check that no setup step asks you to define a filter you have not read.

## `repo.git-hooks`

**Git hooks point into the project** (Review)

This copy's .git/config sets core.hooksPath to a folder in the project, so its scripts run on every commit, checkout or merge.

What you can do: Read the scripts, or remove core.hooksPath.

## `repo.git-config-command`

**This copy's git settings run a program** (Review)

A .git/config that came with the folder sets core.fsmonitor, core.pager, diff.external, a filter or another setting that makes ordinary git commands run a program. git status runs core.fsmonitor, and many shell prompts run git status. Settings that run on routine commands are rated High.

What you can do: Read the program, or remove the setting before running git in the folder.

## `repo.ai-hooks`

**AI tool settings in the project run commands** (Review)

Claude Code settings shipped with the project define hooks or helper commands (statusLine, apiKeyHelper, fileSuggestion and similar), which run shell commands while the tool works in the folder.

What you can do: Read the commands before trusting the project.

## `repo.ai-settings`

**The project loosens the AI tool's safeguards** (Review)

Settings shipped with the project approve MCP servers, commands or edits on your behalf (Claude Code's enableAllProjectMcpServers, permissions, or Copilot's chat.tools.autoApprove), or point the tool's requests elsewhere. Sending requests and your API key to another server (env.ANTHROPIC_BASE_URL or a proxy), or turning off Copilot's confirmations, is rated High.

What you can do: Remove the settings, or run the AI tool with only your own settings in this folder.

## `repo.instructions-run-download`

**An AI instruction file says to download and run code** (Review)

An instruction file for AI coding tools contains a download piped into a shell. An assistant that can run commands may follow it without asking.

What you can do: Check where the download comes from, or remove the line before an AI tool reads the project.

## `repo.code-loader`

**The project runs text it fetches or decodes as code** (High)

A source file hands text to the Function constructor with require, or to eval, Function or vm right next to where it fetched or decoded that text. Fake take-home projects use this to run a payload when the app starts; nothing needs to be installed.

What you can do: Do not run the project on a computer with keys, wallets or work access. Find where the text comes from first; to watch it run, use bigman try, a disposable container without your files or keys.

## `repo.encoded-url`

**A web address is stored encoded** (Review)

A source or .env file holds a base64 or hex string that decodes to a web address. Interview lures hide the address their loader fetches this way, often as a fake API key.

What you can do: Find the code that decodes it and what it does with the reply.

## `repo.payload-host`

**The project uses a JSON storage service lures load code from** (Review)

A source file refers to jsonkeeper.com, npoint.io, jsonsilo.com or mocki.io. Contagious Interview projects fetch their next stage from these services, whose content the poster can change at any time.

What you can do: Read what the project does with the reply; if it runs it, do not start the project.

## `repo.disguised-asset`

**A font, image or media file is really text** (High)

A file named as a font, image or media file is readable text, such as hex, base64 or JavaScript. Genuine ones are binary. Lures hide their payload this way and decode it from a loader or an editor task.

What you can do: Do not open or run the project on this computer; find what reads the file. To watch it run, use bigman try, a disposable container without your files or keys.

## `repo.mcp-server`

**The project configures an MCP server** (Review)

A project MCP configuration starts a program that AI tools run with your permissions.

What you can do: Check the package and pin its version before allowing it.

## `repo.hidden-instructions`

**An AI instruction file contains invisible characters** (High)

An instruction file for AI coding tools (CLAUDE.md, AGENTS.md, .cursorrules, a skill's Markdown and similar) contains zero-width, bidirectional or tag characters, which can carry instructions a reader never sees.

What you can do: Show and remove the characters before an AI tool reads the file.

## `repo.workflow-unpinned`

**A workflow uses an action by tag** (Review)

A GitHub Actions workflow refers to a third-party action by tag or branch, which its owner (or someone who takes over the account) can move to different code.

What you can do: Pin third-party actions to a full commit SHA.

## `repo.workflow-pull-request-target`

**A workflow uses pull_request_target** (Review)

pull_request_target runs with the repository's secrets and write access. Checking out the pull request's own code in it (its head ref or SHA, refs/pull, or gh pr checkout) is rated High.

What you can do: Never check out or run the pull request's code in a pull_request_target workflow.

## `skill.runs-on-load`

**A skill runs shell commands as soon as it is used** (Review)

The skill or command has !`command` lines or a ```! block. Claude Code runs them before the model reads the skill and puts their output in its place, without a prompt unless your permission rules say otherwise. High when a command downloads and runs code or touches credentials.

What you can do: Read each command. Turn on disableSkillShellExecution in Claude Code's settings to stop skills you install from running them.

## `skill.hooks`

**A skill or plugin registers hooks** (Review)

Hooks in a skill's or subagent's frontmatter, or in a plugin's hooks file or manifest, run commands on their own on events such as every tool call or the start of a session. High when a command downloads and runs code or touches credentials.

What you can do: Read each hook command and the scripts it calls before enabling the skill or plugin.

## `skill.allowed-tools`

**A skill approves broad tool use in advance** (Review)

Its allowed-tools let the agent use Bash without limits, run an interpreter, downloader or remover, or fetch any web page, without asking you while the skill runs. Claude Code applies a project skill's allowed-tools even in a folder you have not trusted.

What you can do: Narrow allowed-tools to the exact commands the skill needs, or remove them and approve each use.

## `skill.mcp-server`

**A plugin starts or connects to MCP servers** (Review)

The plugin declares MCP servers, which the AI tool starts or connects to on its own and hands your requests and data. High when it downloads an MCP bundle when it loads, or a server command downloads and runs code.

What you can do: Check each server's program and publisher, and pin its version.

## `skill.background-command`

**A plugin starts programs in the background** (Review)

The plugin declares monitors, which start with every session and keep running, or language servers, which start when you open matching files.

What you can do: Read what each one runs before enabling the plugin.

## `skill.bin-on-path`

**A plugin adds programs to the agent's PATH** (Review)

Files in a Claude Code plugin's bin folder are on the Bash tool's PATH while the plugin is enabled, so the agent runs them as ordinary commands. High when one has the name of a common command such as git, curl or python and so runs in its place.

What you can do: Read each program; remove the plugin if one stands in for a command you rely on.

## `skill.binary`

**A skill ships a compiled program or an archive** (Review)

A file in the skill or plugin is a Linux, Windows or macOS program, a WebAssembly module or an archive. None can be read like the rest of a skill, and password-protected archives are a known way past scanners.

What you can do: Find out what the file is, where it came from and what runs it before using the skill.

## `skill.code-plugin`

**OpenCode loads plugin code at every start** (Review)

A JavaScript or TypeScript file in an OpenCode plugin folder, or named in OpenCode's configuration, runs inside OpenCode at every start with your permissions and sees every prompt and tool result. The evidence says whether the code starts programs, connects to the network or reads credentials.

What you can do: Read the code, or keep only plugins from sources you trust.

## `skill.npm-plugin`

**OpenCode installs and loads an npm plugin** (Review)

OpenCode's configuration names an npm package that it installs with Bun and runs inside itself at every start. With the cached copy present, its install scripts and code get the other rules too.

What you can do: Pin an exact version and read the package before trusting it.

## `skill.download-and-run`

**A skill's instructions download and run a program** (High)

The instructions pipe a download into a shell, download then mark something executable, use PowerShell to fetch and run, or hand over a password for an archive. Fake setup steps like these installed infostealers from hundreds of ClawHub skills in 2026.

What you can do: Do not run it. If the skill needs a tool, install that tool yourself from its real source.

## `skill.credential-paths`

**A skill refers to credential files** (Review)

Its instructions or code name SSH keys, cloud, registry or GitHub credential files, AI tool tokens, browser login data or the cloud metadata address. Few skills need them.

What you can do: Find out why it needs them before letting it run.

## `skill.exfil-endpoint`

**A skill names a data-collection address** (High)

A file names a request catcher, tunnel, paste or file-drop site, a chat webhook, an out-of-band testing host, or a public IP address in a URL. Those are where stolen data is usually sent.

What you can do: Find out what is sent there. Remove the skill if you cannot tell.

## `skill.encoded-blob`

**A skill contains encoded text** (Review)

Readable files (Markdown, JSON, shell) hold a long base64 run, or code decodes text and runs it. Encoded content hides what it does, which is why malicious skills use it.

What you can do: Decode it by hand and read it before using the skill.

## `skill.secrecy`

**A skill asks the agent to keep things from you** (Review)

Its instructions tell the agent not to tell you about something, to act without asking, to skip your approval, to ignore earlier instructions, or to act silently. Phrases inside quotes are left out, as they are usually examples to refuse. A phrase alone is not proof.

What you can do: Read the passage around it and decide whether the skill should be doing that without you.

## `skill.memory-write`

**A skill changes agent memory or settings files** (Review)

Its instructions write to CLAUDE.md, AGENTS.md, MEMORY.md, SOUL.md, a Claude Code, Codex or OpenCode settings file, a shell startup file, authorized_keys or crontab. Those changes outlast the session.

What you can do: Check what it writes and why.

## `skill.html-comment`

**A skill has text hidden in an HTML comment** (Review)

The agent reads HTML comments; a rendered page does not show them. Reported for comments of eight words or more in SKILL.md, commands and subagents, and for comments anywhere that ask for downloads, sending data, credentials or secrecy.

What you can do: Read the comment.

## `skill.script-exfil`

**A skill's code reads credentials and connects out** (High)

A script or plugin file both reads credential files or collects the whole environment and opens network connections, the pairing stolen tokens leave by. Reading one named token for its own API is not counted.

What you can do: Read what it reads and where it sends it before running it.

## `skill.changed`

**An approved skill or plugin changed** (Review)

A file of a skill or plugin you approved was added, removed or changed since your approval. Updates are usually ordinary; a trusted skill turning malicious in an update looks exactly the same.

What you can do: Read the changed files, then approve the new version in Protect > AI skills.

## `skill.unreviewed`

**Skills and plugins not reviewed yet** (Info)

Installed skills and plugins you have not approved. Approving records the exact files you read so a later change is reported; it is not a verdict on the skill.

What you can do: Read each one in Protect > AI skills and approve the version you read.

## `lockfile.non-registry`

**A dependency comes from outside the registry** (Review)

The lockfile resolves a package from git, a plain URL or a local path, which skips the registry's checks and can change without a new version.

What you can do: Make sure the package is meant to come from there; prefer a registry release.

## `lockfile.missing-integrity`

**A dependency has no integrity hash** (Review)

A registry package in the lockfile has no hash, so the package manager cannot tell if the download changed.

What you can do: Regenerate the lockfile with a current package manager.

## `lockfile.install-hook`

**A dependency runs an install script** (Info)

The lockfile records that this package runs a script during install (npm hasInstallScript, pnpm requiresBuild).

What you can do: Read the package's scripts, or install with --ignore-scripts.

## `lockfile.new-install-hook`

**A new dependency runs an install script** (Review)

Compared with the older lockfile, a package that runs an install script was added, changed version, or started running a script.

What you can do: Read the new package's scripts before installing, or install with --ignore-scripts.

## `lockfile.source-changed`

**A dependency changed source without a new version** (High)

Compared with the older lockfile, a package kept its version but now downloads from a different address or with a different hash. A published version never changes, so this means the lockfile was edited.

What you can do: Find out who changed the lockfile and why before installing.

## `channel.signature-disabled`

**A package source skips signature checks** (High)

The source is configured to install packages without verifying their signatures (apt trusted=yes or allow-insecure, gpgcheck=0, pacman SigLevel Never or Optional, Flatpak no-gpg-verify, Nix require-sigs = false).

What you can do: Turn signature checks back on for this source.

## `channel.plain-http`

**A package source uses plain HTTP** (Info)

Packages and metadata are downloaded unencrypted. Signatures still protect the packages, but the network sees what you install and can replay old metadata.

What you can do: Use the source's https:// address if it has one.

## `channel.unscoped-key`

**A third-party apt source has no signed-by key** (Review)

Without signed-by, apt accepts packages for this source signed by any key it trusts, and the vendor's key is trusted for every other source.

What you can do: Store the vendor's key in /etc/apt/keyrings and add signed-by= to the source.

## `channel.global-key`

**A third-party key is trusted for every apt source** (Review)

A keyring in /etc/apt/trusted.gpg.d or the legacy trusted.gpg that no distribution ships can sign packages for any source, the distribution's own included.

What you can do: Move the key to /etc/apt/keyrings and reference it with signed-by= from its own source.

## `channel.expiry-check-off`

**An apt source accepts expired metadata** (Review)

check-valid-until=no lets a mirror keep serving an old snapshot that hides security updates (a freeze attack).

What you can do: Remove the option unless the source is an archive snapshot pinned on purpose.

## `channel.stale-metadata`

**Package lists are more than two weeks old** (Review)

The newest metadata from the distribution's own repositories was published over 14 days ago. Either the lists are not being refreshed, or a mirror is serving a frozen copy.

What you can do: Refresh the package lists; if they stay old, switch mirrors.

## `channel.priority-override`

**A third-party source outranks the distribution** (Review)

A dnf or zypper priority below 99, a pacman repository listed above [core], or an apt pin above the distribution's 500 (1000 and above also downgrades) lets this source replace the distribution's packages.

What you can do: Keep the override only if you mean it, and limit it to the packages you need from that source.

## `channel.personal-repo`

**Personal repositories are configured** (Info)

PPAs, COPR projects and OBS home projects are run by one person or project and are not reviewed by the distribution.

What you can do: Remove the ones you no longer use.

## `channel.third-party`

**Third-party package sources are configured** (Info)

An inventory of sources not run by the distribution or Flathub. Each can ship updates for anything it packages; an abandoned one whose domain changes hands can ship anything.

What you can do: Remove sources you no longer use.

## `archive.absolute-path`

**Archive entries are named with an absolute path** (High)

An entry is named from the root, like /etc/x or C:\x, so an extractor that keeps names as they are writes it there instead of into the folder you chose. Current GNU tar, bsdtar and unzip strip the leading slash; some other tools have not.

What you can do: Do not extract it with an unfamiliar tool. List it first with bsdtar -tvf.

## `archive.path-traversal`

**Archive entries climb out of the extraction folder** (High)

An entry name uses .. to reach above the folder it is extracted into, for example to overwrite ~/.bashrc. Ordinary archives never need this.

What you can do: Do not extract it.

## `archive.link-escape`

**Links in the archive point outside it** (High)

A symbolic or hard link inside the archive points at an absolute path or above the extraction folder. Later entries written through the link, or the next program that follows it, land outside the folder.

What you can do: Do not extract it.

## `archive.special-entry`

**The archive holds devices, pipes or setuid programs** (Review)

An entry is a device file or named pipe, or a program marked setuid or setgid. User archives rarely need these. Extracting as an ordinary user drops the setuid bits and refuses devices; extracting as root keeps them.

What you can do: Extract it only as yourself, never with sudo.

## `archive.startup-path`

**Archive entries would land in startup or login files** (Review)

An entry's path ends in a place that runs at login or controls access: autostart, user systemd units, shell startup files, SSH authorized_keys, cron or ld.so.preload. Extracted into your home folder or /, it takes effect without being opened.

What you can do: Check each entry before extracting into your home folder or /.

## `archive.bomb-ratio`

**The archive expands to far more than its size** (Review)

The sizes the archive declares add up to at least 100 times the archive itself and at least 64 MiB. Ordinary archives grow 2 to 10 times. Extracting one like this can fill the disk.

What you can do: Extract it only somewhere with a size limit, or not at all.

## `package.gyp-build`

**An npm package builds native code at install** (Review)

The package has a binding.gyp and no install script of its own, so npm runs node-gyp rebuild when installing it. That runs a compiler and any command binding.gyp names (<!(...)), and it does not appear in the package's scripts.

What you can do: Read binding.gyp, or install with --ignore-scripts.

## `package.bin-swapped`

**A command in node_modules/.bin was not put there by the package manager** (High)

npm run and npx put node_modules/.bin first on PATH. npm, Yarn and Bun make each command a link into node_modules, and pnpm a shim that runs node on a file there. This one is a plain file where a link belongs, leads outside the project's node_modules, or (for pnpm) runs something other than node or downloads or decodes text.

What you can do: Delete node_modules and install again from the lockfile, and find out what wrote the file.

## `py.startup-code`

**A Python package runs code at every start** (Review)

The package installs a .pth file with an import line, or a sitecustomize or usercustomize module. Python runs these in every program, pip included, without anything importing the package. setuptools does this to adjust imports; a line that decodes, executes, starts processes or connects is rated High, as in the tampered LiteLLM 1.82.8 release.

What you can do: Read the line. Do not install a package whose startup code you cannot account for.

## `py.entrypoint-shadow`

**A Python package installs a command named like a common one** (Review)

A console script in the package is named python, pip, git, sudo, ssh or another common command. Where its scripts folder comes first on PATH, that command runs the package instead.

What you can do: Check why the package needs that name.

## `py.setup-exec`

**Installing a Python source package runs code that reaches out** (Review)

The source package's setup.py starts processes, fetches, decodes or executes text, or pyproject.toml builds it with its own in-tree backend. pip runs this when it installs the package.

What you can do: Read the lines, or install a wheel instead.

## `rust.build-script`

**A Rust crate runs a build script** (Review)

Cargo compiles and runs a crate's build.rs on cargo build, check or test, and rust-analyzer runs it when an editor opens the project. A build script that opens network connections or reads your home folder or credentials is rated High.

What you can do: Read the build script before building or opening the project.

## `rust.proc-macro`

**A Rust crate is a procedural macro** (Info)

The crate's code runs inside the compiler whenever a crate using it is built or analysed. Expected for macro crates.

What you can do: Check the crate is meant to be a macro.

## `vsix.startup-activation`

**A VS Code extension starts with the editor** (Review)

The extension's activationEvents include * or onStartupFinished, so it runs every time the editor starts rather than for the files or commands it handles.

What you can do: Check why it needs to run all the time.

## `vsix.dependency-chain`

**A VS Code extension installs other publishers' extensions** (Review)

The extension's extensionPack or extensionDependencies name extensions from other publishers, which install and update along with it. GlassWorm spread this way.

What you can do: Check each listed extension and its publisher.

## `vsix.invisible-code`

**A VS Code extension hides text in invisible characters** (High)

A JavaScript file in the extension holds a run of variation selectors, tag characters or zero-width marks, which can encode code an editor never shows, as in the GlassWorm extensions.

What you can do: Do not install it.

## `vsix.native-binary`

**A VS Code extension ships native programs** (Info)

The extension includes compiled programs or libraries (.node, .so, .exe, .dll), whose code is not visible in its JavaScript. Common for language tools.

What you can do: Check the extension is meant to include them.

## `pkg.install-script`

**A package runs a script as root when installed** (Review)

The package file carries a maintainer script (deb preinst or postinst, rpm %pre or %post, a pacman .install) that runs as root during install, upgrade or removal. Its lines are also checked with the install-script rules.

What you can do: Read the script before installing. A signature only says who built the package, not what the script does.

## `pkg.adds-source`

**Installing the package adds a package source** (Review)

The package ships a repository file, or its script writes one, so later updates for it and for other packages can come from that source.

What you can do: Check the source is the vendor's own. After installing, it appears in Protect > Updates > Package sources.

## `pkg.adds-key`

**Installing the package trusts a signing key** (Review)

The package ships a signing key into a keyring folder, or its script imports one, so packages signed with that key install without a warning.

What you can do: Check the key belongs to the vendor, and that the apt source using it is scoped with signed-by.

## `pkg.setuid`

**The package installs a program with raised privileges** (Review)

A file in the package is setuid or setgid, so it runs with its owner's rights, usually root's, whoever starts it. Browsers' sandbox helpers are a common, expected case.

What you can do: Check the package is meant to need it.

## `pkg.startup-path`

**The package installs files that run at boot or login** (Review)

The package ships services, timers, cron jobs, autostart entries, login scripts, loader configuration, PAM, sudo or udev rules. These take effect without anything being opened.

What you can do: Check each one is expected for this package.

## `pkg.user-path`

**The package puts files in home or temporary folders** (High)

The package installs into /home, /root, /tmp, /var/tmp or /dev/shm. Package managers install into system folders; these paths are outside what they track and remove.

What you can do: Do not install it unless the vendor explains why.

## `pkgbuild.skip-checksum`

**A PKGBUILD download is not checked against a checksum** (Review)

A source that is not a version-control checkout or a signature file has SKIP for its checksum, so makepkg builds with whatever the server returns.

What you can do: Ask the maintainer for a real checksum, or check the file yourself before building.

## `appimage.unsigned`

**The AppImage is not signed** (Info)

The AppImage has no embedded signature. Most do not, so this is common; it means only the download's origin vouches for it.

What you can do: Download it from the project's own site and compare its checksum with the one published there.

## `appimage.update-url`

**The AppImage can update itself** (Info)

The AppImage carries update information, so an update tool can replace it with whatever that address serves.

What you can do: Check the address belongs to the project.

## `desktop.launcher`

**This is a program launcher, not a document** (Review)

A .desktop file runs the command on its Exec line when opened. It can sit in a folder or an archive looking like any other file. Commands it runs are also checked with the installer rules.

What you can do: Open it only if you expected a program from where it came from.

## `desktop.masquerade`

**A program launcher is made to look like a file** (High)

A .desktop launcher borrows a document's look: a double extension like report.pdf.desktop, a Name such as Invoice.pdf, or a PDF, office or folder icon. File managers show the Name and icon, so it reads as a document while opening it runs a command.

What you can do: Do not open it. Delete it unless you know who made it and why.

## `desktop.exec-download`

**A program launcher downloads something when opened** (High)

The launcher's command runs curl or wget. What it downloads is not part of this check and can change at any time.

What you can do: Do not open it.

## `canary.event`

**A decoy file was opened or changed** (High)

A canary decoy was read, changed, moved or removed while the monitor was running. With reader identification on, the evidence names the process, its user and what started it; without it, inotify cannot say which program did it. The decoy's access log records each access and what its chosen responses did.

What you can do: Check whether you recognize the process. Backups, indexers and your own tests can also trigger decoys.

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

## `hidden.process`

**A running program is missing from the process list** (High)

A process ID answers a signal probe, and /proc, which ps and top read, does not list it. Both were read twice. A kernel module that hides a process filters the listing and leaves the process running. The check is skipped when /proc is mounted to hide other accounts' processes.

What you can do: Note the process ID and name, then check the disk from other trusted media: a kernel that hides one thing can hide others from every tool run on it.

## `hidden.module`

**A kernel module is in one list and not another** (High)

/proc/modules (what lsmod reads), /sys/module and the kernel's symbol list each name the loaded modules. One of them names a module another leaves out, on two readings. Modules that hide remove themselves from the list tools read.

What you can do: Note the module name and check the disk from other trusted media. If you loaded the module yourself and it is still starting or stopping, check again.

## `hidden.net-socket`

**A socket is in one kernel table and not the other** (High)

/proc/net (what netstat reads) and the kernel's socket diagnostics (what ss reads) both list TCP and UDP sockets. One shows a listening port or a connection the other does not, on two readings. Hiding a port means filtering one of these; filtering both the same way is harder.

What you can do: Note the port and the remote address. Find which program holds it with the view that does show it, and check the disk from other trusted media.

## `hidden.libc-view`

**The C library gives Bigman a different answer than the kernel** (High)

Bigman read the process list, the module list, the socket tables or /etc/ld.so.preload twice: through the C library, as every ordinary tool does, and with direct system calls. The C library's answer leaves something out. A library loaded into every program through /etc/ld.so.preload or LD_PRELOAD filters results this way, so ls, ps and ss do not show it.

What you can do: Read /etc/ld.so.preload and the LD_PRELOAD rows below with a statically linked tool (busybox cat), find which package owns each library named, and remove the ones nothing owns.

## `linker.preload-present`

**A library is loaded into every program** (High)

/etc/ld.so.preload names libraries the dynamic loader puts into every program that starts, before the program's own code. The file does not exist on a standard installation. A few performance and compatibility tools add an entry; so does software that hides files, processes and connections from the tools that would show them.

What you can do: Find which package owns each library (rpm -qf, dpkg -S, pacman -Qo). Remove entries nothing owns, then restart.

## `module.taint`

**The kernel carries taint marks** (Info)

The kernel records, until the next restart, that an out-of-tree, unsigned, proprietary or force-loaded module was loaded, and a few other events. Graphics and virtual-machine drivers leave these marks and are named. It is Review when a module mark is set and no loaded module carries it: the module was unloaded since, or is not in the list.

What you can do: If you did not load and remove a driver since the last restart, read which modules the kernel log names with the command below, and the kernel module rows here.

## `module.no-file`

**A kernel module was loaded from outside the kernel's module folder** (Review)

Every module a distribution or a driver package installs has a file under /lib/modules for the running kernel. This loaded module has none, so it was loaded by hand from a file somewhere else.

What you can do: If you built and loaded it yourself, mark it as known. Otherwise read the kernel log with the command below and check the startup files for module load entries.

## `bpf.programs`

**BPF programs are loaded in the kernel** (Info)

BPF programs run inside the kernel, where they can watch or change system calls and network traffic. systemd, container tools and security agents load them; so do some programs that hide themselves. Their names are read from the kernel's symbol list, which needs no administrator rights; the type of each program does not show there.

What you can do: Mark the list as known once you recognise it; Bigman then reports when a name is added. The command below shows each program's type and what loaded it.

## `process.fake-kernel-thread`

**A program is named like a kernel thread** (High)

A process has the name of a kernel thread (kworker, ksoftirqd, kthreadd), or a title in square brackets the way tools print one, and the kernel does not mark it as a kernel thread. Ordinary programs have no reason to take these names.

What you can do: Find where the program is and what started it with the commands below (as its owner), and keep a copy.

## `process.exe-deleted`

**A program is running and its file was deleted** (Review)

The file a running program started from no longer exists, so a scan of your files cannot read it. It is High when the file was in a temporary folder. Programs whose file was replaced by an update or a rebuild are listed together as Info: restarting them runs the new version.

What you can do: If you do not recognise it, copy the program out before stopping it and check what started it; the commands below do both.

## `process.exe-memfd`

**A program runs from memory with no file on disk** (Review)

The program's code was written into a memory file and run from there, so it never existed as a file a scan could read. AppImage launchers and the container runtime runc do this on purpose and are Info.

What you can do: If you do not recognise it, copy the program out before stopping it and check which program started it; the commands below do both.

## `process.exe-shm`

**A program runs from shared memory** (Review)

The program's file is in /dev/shm, a folder kept in memory and emptied at every restart. Software is not installed there; it is a place to put a program that should leave nothing on disk.

What you can do: If you did not put it there, copy the file for later, stop the program and check what started it; the commands below copy it and show its parent.

## `process.daemon-name`

**A program has a system service's name and runs from somewhere else** (Review)

One of your programs is named like a system service (sshd, systemd, cron, dbus-daemon) and its file is outside the system folders where those are installed. A build of your own does this; so does a program made to pass for the service in a process list.

What you can do: If it is your own build, mark it as known. Otherwise read where it is and what started it.

## `process.shell-socket`

**A shell or script interpreter is wired to a network connection** (High)

The standard input and output of a shell or interpreter are one network socket, so whatever is at the other end types commands and reads the results. It is Review when the other end is on this computer.

What you can do: If you did not set this up, note the remote address, stop the process, and look for what started it (crontab -l, ~/.config/systemd/user, autostart).

## `process.preload`

**Programs run with a library loaded first** (Info)

LD_PRELOAD or LD_AUDIT in a program's environment loads a library into it before its own code. Browsers and Flatpak applications do this for their sandbox helpers. It is Review when the library is in your home folder or a temporary folder, where any program of yours could have put it.

What you can do: For a library in a folder you can write to, find which file sets the variable (shell profile, ~/.config/environment.d, a .desktop launcher) and what put the library there.

## `net.packet-socket`

**A program reads network frames directly** (Info)

A packet socket receives frames below TCP and UDP. No port is open, so nothing shows in a list of listening ports. Wi-Fi sign-in and address discovery hold one for their own frame type. It is Review when the socket receives every packet: packet capture tools do that while they run, and so does a program that waits for one particular packet before acting.

What you can do: If no capture or network-usage tool is running, find the holder with the command below and check what started it.

## `net.raw-socket`

**A program sends and receives raw IP packets** (Info)

A raw socket carries a protocol such as ICMP without a port, so nothing shows in a list of listening ports. Network managers and VPN services run by the administrator hold them. It is Review when the holder is not the administrator, which takes a privilege ordinary programs lack.

What you can do: Find the holder with the command below and check how it got the privilege (getcap on its file).

## `watch.credentials`

**A program is told when your credential files change** (Review)

A program holds an inotify watch on a folder or file that holds keys or tokens (~/.ssh, ~/.gnupg, cloud and registry credentials), so the kernel tells it whenever something there is opened or written. It is Info when the program is installed under a system folder, as key agents and file managers are.

What you can do: If you do not recognise the program, read where it is and what started it before you add or change a key.

## `watch.own-program`

**A program watches its own file** (Review)

A program holds an inotify watch on the file it runs from, so the kernel tells it when that file is deleted or replaced. Programs that put themselves back after being removed do this; some self-updaters do too.

What you can do: If you tried to remove this program and it came back, stop it first, then delete the file and its startup entry.

## `respawn.program-restart`

**A program you stopped was started again by another program** (Review)

You asked Bigman to stop one of your programs and watch. Within the minute, a new process running the same program appeared, and its parent is an ordinary program rather than the service manager. Software that wants to keep running is often two processes that restart each other; launchers, updaters and development tools restart programs too.

What you can do: If you recognise the program that started it, nothing needs doing. If not, check its file from Inspect and look at Protect › Startup files for what starts it at login.

## `respawn.orphan-restart`

**A program you stopped was started again by something that let go of it** (Review)

You asked Bigman to stop one of your programs and watch. A new process running the same program appeared, and its parent is the service manager although it is not a service: the program that started it exited at once, so the kernel handed it on. Starting a program this way hides who started it.

What you can do: If you know what restarts it, nothing needs doing. If not, look at what else runs in the same unit and at Protect › Startup files.

## `respawn.service-restart`

**The service manager started a program you stopped again** (Info)

You asked Bigman to stop one of your programs and watch. The service manager started it again because its unit says to restart it when it stops. This is ordinary for services; the unit file says what it runs.

What you can do: To stop it until the next login, run the command shown; to keep it from starting at all, use disable --now instead.

## `respawn.not-restarted`

**Nothing started a program you stopped again** (Info)

You asked Bigman to stop one of your programs and watch. No new process running the same program appeared within the minute. Something that starts it at the next login, from a timer or from a scheduled job is not seen in that minute.

What you can do: Start it again from the result if you need it. Protect › Startup files lists what starts programs at login.

## `respawn.did-not-stop`

**A program you asked to stop kept running** (Info)

Bigman asked the program to stop with SIGTERM, the request a task manager sends, and it was still running when the watch ended. A program may take time to save its work, or ignore the request.

What you can do: Close it from its own window, or end it without waiting with kill -KILL after checking the process ID is still the same program.

## `bus.name-owner`

**An unexpected program answers for your password store or portal** (Review)

Programs ask the session bus for saved passwords and for permission prompts by name (org.freedesktop.secrets, org.freedesktop.portal.Desktop). Whichever program claims the name first gets the requests. This owner is not installed under a system folder.

What you can do: If it is a password manager you set up to take this role, mark it as known. Otherwise stop it and log in again so the desktop's own service takes the name.
