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
| `repo.mcp-server` | Review | The project configures an MCP server | SC-21, SC-22 |
| `repo.hidden-instructions` | High | An AI instruction file contains invisible characters | SC-21, CR-19 |
| `repo.workflow-unpinned` | Review | A workflow uses an action by tag | SC-15 |
| `repo.workflow-pull-request-target` | Review | A workflow uses pull_request_target | SC-16 |
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

A VS Code task in the project is set to run when the folder opens (runOn: folderOpen).

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

Claude Code settings shipped with the project define hooks, which run shell commands while the tool works in the folder.

What you can do: Read the hooks before trusting the project.

## `repo.mcp-server`

**The project configures an MCP server** (Review)

A project MCP configuration starts a program that AI tools run with your permissions.

What you can do: Check the package and pin its version before allowing it.

## `repo.hidden-instructions`

**An AI instruction file contains invisible characters** (High)

An instruction file for AI coding tools (CLAUDE.md, AGENTS.md, .cursorrules and similar) contains zero-width, bidirectional or tag characters, which can carry instructions a reader never sees.

What you can do: Show and remove the characters before an AI tool reads the file.

## `repo.workflow-unpinned`

**A workflow uses an action by tag** (Review)

A GitHub Actions workflow refers to a third-party action by tag or branch, which its owner (or someone who takes over the account) can move to different code.

What you can do: Pin third-party actions to a full commit SHA.

## `repo.workflow-pull-request-target`

**A workflow uses pull_request_target** (Review)

pull_request_target runs with the repository's secrets and write access. Checking out the pull request's own code in it (its head ref or SHA, refs/pull, or gh pr checkout) is rated High.

What you can do: Never check out or run the pull request's code in a pull_request_target workflow.

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
