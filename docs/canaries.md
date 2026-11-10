# Canaries and monitoring

> Part of the [Bigman README](../README.md).

The default is entirely local: no Canarytokens account, API key, paid service or internet connection is needed. In the app, choose **Protect → Canaries → Create, monitor and test…**. The CLI equivalent is:

```sh
bigman canary arm
```

This creates `~/Documents/backup-access.env` (or `~/backup-access.env` if Documents does not exist), starts monitoring, starts `bigmand` (which runs the monitor) at future user logins, performs a read, and waits for a fresh kernel-event receipt. Existing registered local decoys at that path are reused; unrelated existing files are refused. Install Bigman at a stable path first with `./install.sh` so the login service can still find it later.

The monitor runs inside `bigmand`, so it has no process or login service of its own. If `bigmand` stops or is killed, the monitor ends with it, and it comes back when `bigmand` does, unless you stopped it.

The setup result reports local detection and startup coverage separately. If the user service manager is unavailable or has conflicting settings, the monitor still runs for as long as `bigmand` does, and Bigman reports **incomplete** startup coverage. It does not overwrite a differently configured `bigmand` unit. The monitor continues after closing the app or CLI if `bigmand` keeps running; logout/reboot gaps are not covered.

```sh
bigman canary arm ~/Documents/another-decoy.env
bigman canary arm --session-only        # no login service changes; on until you restart
bigman canary list
bigman canary start                    # restart existing tokens; create no new files
bigman canary test CANARY_ID            # verifies a new real read receipt
bigman history
bigman canary stop                      # stop monitor; stays off until started again
bigman canary stop --session-only       # stop monitor until you restart
bigman canary disarm CANARY_ID          # remove this registration; retain its file
```

Verification fails if monitoring is stopped, the path isn't watched, or no new receipt arrives. An old receipt does not count. Receipts prove local filesystem detection and durable evidence; they do not prove desktop notifications were displayed. Alerts remain available in History if notification utilities or the desktop notification service are unavailable. Deliberate verification reads can also appear as ordinary canary alerts.

For manual creation without starting anything, use `bigman canary create PATH`, followed by `bigman watch`, which runs a monitor in the foreground outside `bigmand`. `stop` also stops the startup-change monitor, but preserves tokens, registrations, reports and the signature update timer. `disarm` only removes the selected registration.

Creation is exclusive: an existing file, directory or symlink is not overwritten. The local token is a UUID-based decoy with no working privileges. Pick a location that ordinary applications rarely read. Test reads, backups, search indexers, antivirus tools and synchronization software can produce legitimate alerts. Bigman's own scanner skips registered decoys.

The monitor watches parent directories so replacements can be observed. Only events that name the decoy itself count, plus the folder being moved or deleted: listing the folder (`ls`, a file manager, a search indexer) is not a read of the decoy. A deleted decoy alerts once and is removed from the list; if its whole folder is missing, such as an unmounted drive, it stays registered. The monitor reports queue overflow, missing paths and watcher failures; writes a heartbeat; and groups repeated events for a canary within three seconds. Files copied or read while the monitor is stopped cannot be reconstructed later. inotify does not provide a reliable reader PID or tell whether data was exfiltrated. Host malware with the same user privileges can interfere with this application or its state.

Local monitoring cannot detect later use of a stolen token on a different computer. That needs a reachable callback service, either hosted by you or a provider; Bigman does not expose a network listener or provision one. This is optional and separate from account-free local detection.

To use an existing hosted token:

```sh
bigman canary create ~/Documents/backup-access.url --hosted-url https://YOUR-PROVIDER/YOUR-TOKEN
```

Use the HTTPS URL the provider gave you. This writes a shortcut and does not visit the URL. Opening the file as plain text does not necessarily activate the provider; following the URL does. Bigman does not embed tokens in live credentials, create cloud credentials, intercept HTTPS, or change certificate trust.

Enable persistent per-user services only after installing Bigman at a stable path:

```sh
bigman services                         # show exact unit contents
bigman services --apply                 # enable bigmand (with the monitor) and the daily signature update
bigman services --remove                # removal preview
bigman services --remove --apply
systemctl --user status bigmand.service bigman-update.timer
```

The monitor compares persistence snapshots every five minutes. Review the initial inventory and establish a baseline first. Existing state remains after disabling services.
