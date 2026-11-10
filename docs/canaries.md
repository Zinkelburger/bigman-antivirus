# Canaries and monitoring

> Part of the [Bigman README](../README.md).

The default is entirely local: no Canarytokens account, API key, paid service or internet connection is needed. In the app, choose **Protect → Canaries → Create, monitor and test…**. The CLI equivalent is:

```sh
bigman canary arm
```

This creates `~/Documents/backup-access.env` (or `~/backup-access.env` if Documents does not exist), starts monitoring, enables only the monitor at future user logins, performs a read, and waits for a fresh kernel-event receipt. Existing registered local decoys at that path are reused; unrelated existing files are refused. Install Bigman at a stable path first with `./install.sh` so the login service can still find it later.

The setup result reports local detection and startup coverage separately. If the user service manager is unavailable or has conflicting settings, Bigman runs a detached session monitor and reports **incomplete** startup coverage. It does not overwrite a differently configured monitor unit. The monitor continues after closing the app/CLI; logout/reboot gaps are not covered. Enabling at login while a session monitor is already running preserves that process until logout.

```sh
bigman canary arm ~/Documents/another-decoy.env
bigman canary arm --session-only        # no login service changes
bigman canary list
bigman canary start                    # restart existing tokens; create no new files
bigman canary test CANARY_ID            # verifies a new real read receipt
bigman history
bigman canary stop                      # stop monitor, remove its login service
bigman canary stop --session-only       # stop current process; leave service configuration
bigman canary disarm CANARY_ID          # remove this registration; retain its file
```

Verification fails if monitoring is stopped, the path isn't watched, or no new receipt arrives. An old receipt does not count. Receipts prove local filesystem detection and durable evidence; they do not prove desktop notifications were displayed. Alerts remain available in History if notification utilities or the desktop notification service are unavailable. Deliberate verification reads can also appear as ordinary canary alerts.

For manual creation without starting anything, use `bigman canary create PATH`, followed by `bigman watch`. `stop` also stops the startup-change monitor, but preserves tokens, registrations, reports and the signature update timer. `disarm` only removes the selected registration.

Creation is exclusive: an existing file, directory or symlink is not overwritten. The local token is a UUID-based decoy with no working privileges. Pick a location that ordinary applications rarely read. Test reads, backups, search indexers, antivirus tools and synchronization software can produce legitimate alerts. Bigman's own scanner skips registered decoys.

The monitor watches parent directories so replacements can be observed. Only events that name the decoy itself count, plus the folder being moved or deleted: listing the folder with `ls` reports a read of the folder, and that is not a read of the decoy. A file manager or a file chooser showing the folder is different. To choose an icon, GTK and KDE open files whose name does not say what they are (`backup-access.env` is one), and that is a real read of the decoy, so opening the decoy's folder in Files is logged as an access. Writing a new decoy into a folder the monitor already watches is not: events for a decoy are counted from the pass after the monitor first sees it registered. It reports queue overflow, missing paths and watcher failures; writes a heartbeat; and groups repeated events for a canary within three seconds. Files copied or read while the monitor is stopped cannot be reconstructed later. inotify does not provide a reader PID (see reader identification below) or tell whether data was exfiltrated. Host malware with the same user privileges can interfere with this application or its state.

## When a decoy is touched: the access log and responses

Every access goes into the decoy's access log (the last 500, shown in the decoy's panel and by `bigman canary log CANARY_ID`) and History, with a desktop notification. Each decoy can also run responses, chosen when it is created or later with **Change responses**:

| Response | What runs | Needs |
|---|---|---|
| Run a script | Your executable, for up to 30 seconds, with `BIGMAN_CANARY_ID`, `BIGMAN_CANARY_PATH`, `BIGMAN_CANARY_EVENTS`, `BIGMAN_CANARY_TIME` and, when the reader is known, `BIGMAN_READER_PID`, `_UID`, `_USER`, `_NAME`, `_CMDLINE` and `_EXE` | An absolute path to an executable that only you (or root) can change |
| Kill the process | `SIGKILL` to each identified reader that runs as you, after checking its start time against the one the helper recorded. Never Bigman's own programs, the monitor, or anything the monitor runs under (the service manager, the session) | Reader identification |
| Turn networking off | `nmcli networking off` (undo: `nmcli networking on` or the network menu) | NetworkManager |
| Lock your account | The helper runs `usermod --lock --expiredate 1` on your account, so passwords and SSH keys stop working; existing sessions continue unless you also log out. An administrator undoes it with `usermod --unlock --expiredate '' NAME` | Reader identification, a reader that ran as you, and a local account in `/etc/passwd` (not SSSD, LDAP or systemd-homed); never root or a system account |
| Lock the screen | `loginctl lock-session` for each of your sessions | systemd-logind |
| Log out | `loginctl terminate-user` for your account: every session and program of yours ends, the monitor included, until you log back in. A decoy that logged you out does not do so again for ten minutes; the guard is read from the access log, so it holds across the log-out and a program that reads the decoy at every login cannot keep you out | Reader identification and a named reader; systemd-logind |

They run in the order above, in the background, so the monitor keeps watching. The log entry is written before any of them and updated after each one, so even a log-out leaves a record. Responses run at most once a minute per decoy. Nothing is blocked: the read has already happened when they run.

Responses do not run, and the access log says why, when:

- The read is Bigman's own test (the setup check and **Test detection**). The test leaves a marker that the monitor uses up with the one read it accounts for, so a monitor that gets to the read late still recognises it, and the next read is real.
- Every process named for the access is a file manager, file chooser or search indexer installed under `/usr` (Files/nautilus, nemo, caja, Thunar, Dolphin, PCManFM, the GTK, GNOME and KDE file-chooser portals, tracker/localsearch and Baloo), or one of Bigman's own programs. These read the files of any folder they show or index. The program is matched by its file under `/usr`, which only root can replace, not by its name. This needs reader identification: without it Bigman cannot tell a file manager from anything else, so opening the decoy's folder runs whatever you chose. That is why **Log out** waits for a named reader.

An application's own built-in file dialog (not the portal) reads the decoy as that application, so it is not on the list.

```sh
bigman canary arm --lock-screen --network-off --script ~/bin/on-canary.sh
bigman canary respond CANARY_ID --log-out   # replace the responses; no flags = log and notify only
bigman canary arm                           # on a decoy that exists: monitor and test it again, keeping its responses
bigman canary log CANARY_ID
```

Choose carefully what locks you out: a backup tool, a file synchronizer or an application's own file dialog that reads the decoy triggers responses too.

## Reader identification (optional, needs root)

inotify reports that a file was read, not who read it, and only root may ask the kernel which process did (fanotify). Reader identification is a small root service, `bigman-canary-helper`, installed from **Protect → Canaries → Turn on reader identification…** or:

```sh
bigman canary helper            # preview the exact root commands and unit
bigman canary helper --apply    # install through polkit
bigman canary helper --remove --apply
```

Installing copies the binary to `/usr/local/libexec/bigman/` owned by root and enables `bigman-canary-helper.service`. The helper program is read once, from next to `bigman` only (never from `PATH`), and those bytes are piped to the root script, which writes them to a new file in the root-owned folder, checks its SHA-256 and only then gives it the program's name: root never installs bytes other than the ones hashed, and never runs a file your account can change. In the app, the hash checked is the one the preview showed, and an install is refused if the program changed since the preview. `bigman canary helper` and `--apply` are separate runs, so there the root script checks the hash that `--apply` reports.

The monitor connects to `/run/bigman-canary/helper.sock` and names its decoys. Each access then records the reader's PID, user, login user (who logged in, even through sudo), command line, and the processes that started it. A decoy that is replaced (an atomic save, a restore, a copy moved over it) is marked again within a second, so reader identification does not quietly end. A process that exits before the helper reads `/proc` leaves only its PID.

The helper decides what a connection may do from the kernel's peer credentials, not from the request:

- It only watches regular files the connecting account owns, and gives one answer for every path it will not watch (missing, not a regular file, someone else's), so it cannot be used to find out what exists in folders closed to you.
- It shows a reader's executable path only for that account's own processes.
- It locks only the connecting account's own local account, and only after an administrator allowed that for the account. Choosing **Lock your account** for a decoy asks for your password once and adds your user ID to `/etc/bigman-canary/lock-uids` (root-owned; the exact commands are in the preview). Until then the helper refuses, whoever asks: an installed helper is not a way for any program running as you to lock you out. It runs `usermod` at most once a minute per account, answers "already locked" without running anything, and refuses root, system accounts and IDs above `UID_MAX`. Turning reader identification off removes the file.
- A connection is bounded: IDs of at most 64 bytes, 256 files per request, events passed on one kernel buffer at a time and dropped (and reported as lost) when the monitor is not reading. The service runs with a capability bounding set, `MemoryMax=64M` and `TasksMax=256`.

Not checked on a real installation yet: the hardened unit and the account-locking path have been tested without root only, with a stand-in for `usermod`. If **Lock your account** fails after an update, the decoy's access log shows `usermod`'s own message.

Local monitoring cannot detect later use of a stolen token on a different computer. That needs a reachable callback service, either hosted by you or a provider; Bigman does not expose a network listener or provision one. This is optional and separate from account-free local detection.

To use an existing hosted token:

```sh
bigman canary create ~/Documents/backup-access.url --hosted-url https://YOUR-PROVIDER/YOUR-TOKEN
```

Use the HTTPS URL the provider gave you. This writes a shortcut and does not visit the URL. Opening the file as plain text does not necessarily activate the provider; following the URL does. Bigman does not embed tokens in live credentials, create cloud credentials, intercept HTTPS, or change certificate trust.

Enable persistent per-user services only after installing Bigman at a stable path:

```sh
bigman services                         # show exact unit contents
bigman services --apply                 # enable bigmand, the monitor and the daily signature update
bigman services --remove                # removal preview
bigman services --remove --apply
systemctl --user status bigmand.service bigman-monitor.service bigman-update.timer
```

The monitor compares persistence snapshots every five minutes. Review the initial inventory and establish a baseline first. Existing state remains after disabling services. Use `bigman canary stop` before replacing a session monitor with the complete service/timer setup.
