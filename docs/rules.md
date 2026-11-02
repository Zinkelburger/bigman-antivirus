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
| `git.plaintext-credentials` | Review | Git stores credentials in plain text | CR-01, CR-08 |
| `persistence.change` | Review | A startup file changed | HI-01, HI-02, HI-03, HI-04, HI-05, HI-10 |
| `persistence.removed` | Review | A startup file was removed | HI-01, HI-02, HI-03, HI-04, HI-05 |
| `persistence.initial` | Info | Startup file recorded for the first baseline | HI-01, HI-02, HI-03, HI-04, HI-05 |
| `canary.event` | High | A decoy file was opened or changed | CR-22 |
| `canary.verified` | Info | Local read detection verified | CR-22 |
| `canary.notifications-unavailable` | Info | Desktop pop-ups are not available |  |
| `net.listener.wildcard` | Review | Listens on every network interface | NE-01, NE-22 |
| `net.listener.docker_api` | High | The container API is open on the network | NE-06, MW-03 |
| `net.listener.debugger` | High | A remote debugger is reachable from the network | NE-11 |
| `net.listener.lan_service` | Review | A local AI or notebook server is reachable from the network | NE-09, NE-10 |
| `denial.selinux` | Info | SELinux stopped or logged an access | DS-18 |
| `denial.apparmor` | Info | AppArmor stopped or logged an access | DS-18 |
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

The canary monitor has not written a heartbeat in the last ten seconds, so reads of decoy files are not being noticed.

What you can do: Start monitoring from Protect > Canaries, or install the background services from Setup.

## `git.plaintext-credentials`

**Git stores credentials in plain text** (Review)

Git's global credential helper is 'store', which writes tokens unencrypted to ~/.git-credentials. The file itself was not read.

What you can do: Switch to your desktop keyring's credential helper and revoke tokens you no longer use.

## `persistence.change`

**A startup file changed** (Review)

A file that starts programs for you (a systemd unit, autostart entry, shell profile, cron file) is new or different from the baseline you accepted.

What you can do: Review the file and the package that owns it before accepting it into the baseline.

## `persistence.removed`

**A startup file was removed** (Review)

A startup file in your baseline is gone.

What you can do: Check whether an update or your own change explains it.

## `persistence.initial`

**Startup file recorded for the first baseline** (Info)

There is no baseline yet, so every startup file is listed for review.

What you can do: Review the list, then accept it as the baseline to compare future changes against.

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
