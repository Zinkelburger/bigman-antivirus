# Command line

> Part of the [Bigman README](../README.md).

## Commands

```sh
bigman doctor
bigman scan ~/Downloads
bigman scan ~/Downloads --notify
bigman update
bigman inspect ./install.sh
bigman inspect https://example.org/install.sh
cat ./install.sh | bigman
bigman packages ./project
bigman packages ./downloaded-package.tgz
bigman check ~/Downloads/tool_1.0_amd64.deb   # picks the inspector: archive, launcher, package file, PDF, …
bigman repo-check ./cloned-project
bigman repo-check https://github.com/OWNER/take-home   # read it before you clone it
bigman try ./take-home                      # shell in a disposable container with a copy of it
bigman try --offline --port 3000 ./take-home -- npm start
bigman skills                                   # installed AI skills and plugins
bigman skills --approve REPORT_ID 3             # approve item 3 of that report as you read it
bigman check ~/Downloads/some-skill.zip         # a skill folder, SKILL.md, archive or OpenCode plugin
bigman lockfile ./package-lock.json --against ./old/package-lock.json
bigman package-sources
bigman package-policy ./project                 # preview
bigman package-policy ./project --apply
bigman changes
bigman changes --accept-baseline REVIEWED_REPORT_ID
bigman running                                  # processes, modules and sockets, looked at two ways
bigman running --rows                           # every row with its number, key and pinned details
bigman running --known REPORT_ID 3              # mark row 3 as known, pinned to what it is now
bigman running --forget KEY                     # take the mark back
bigman harden                                   # suggestions, with what each costs
bigman harden harden.sshd                       # preview: the exact commands and the undo
bigman harden harden.sshd --apply
bigman harden harden.sshd --undo
bigman harden harden.sshd --keep                # stop suggesting it; --suggest-again
bigman history
bigman history REPORT_ID
bigman --json scan ~/Downloads
bigman daemon status
bigman daemon stop                      # stop background checks; the app starts them again when it opens
```

Baseline acceptance rechecks the saved report’s inventory hash and refuses changes made since review.

Report-producing commands return **0** when there are no review/high findings and no errors, **1** for review/high findings, and **2** for errors/incomplete coverage. Informational findings alone do not cause exit 1. `--json` formats reports and inventories; administrative status messages remain text. Piped input is inspection, never execution.

Restricted execution requires both the saved report and its exact hash:

```sh
bigman sandbox REPORT_ID --approve-sha256 SHA256_FROM_REPORT
```

The saved file is rehashed before launch. The sandbox exposes system runtime files read-only, uses a disposable home/work directory, clears inherited environment credentials, omits host service/agent sockets, and denies networking through a separate network namespace. It has wall-clock, CPU, address-space, output, descriptor, file-size and UID-relative process limits. The process limit is not a dedicated cgroup quota. If Bubblewrap cannot enforce the profile, execution fails without an unrestricted retry. Missing strace makes behavioral coverage incomplete even if the script finishes.

`bigman try` shows the exact Podman commands and asks before starting (`--yes` skips the question). The folder is copied in with `podman cp`, never mounted, so nothing inside can write to it. `--image` takes a full image name with its registry; `--root` makes the shell root inside the container, for installing system packages, at the cost that an escape would act as your account. The command after `--` runs instead of a shell, and its exit code is returned. In the app, the same preview opens a terminal through `systemd-run --user`, because rootless Podman cannot start under the daemon's no-new-privileges setting.

Inspection fetches use HTTPS, bounded size/time/redirects, public destination checks and DNS pinning through curl. Each redirect is checked. Proxies and curl user configuration are disabled for these fetches. Installers that need private endpoints can be downloaded separately and explicitly inspected as local files.

ClamAV defaults: at most 2,000 regular files, 64 MiB per file, 512 MiB total snapshot data. Archive expansion is limited to 256 MiB, 256 files and nesting depth 16. Symlinks, Bigman state and registered canaries are excluded. Directory walks have a 50,000-entry limit. Limit/encryption detections mean incomplete examination. The app prefers its private definitions after `update`, otherwise ClamAV’s default database configuration applies. Checkup freshness knows the private directory and `/var/lib/clamav`; custom system configurations require separate verification.

Quarantine identifiers and indexed scan items:

```sh
# Item indexes and hashes are in STATE/artifacts/REPORT_ID.scan.json.
# In the app, open a scan in Protect → Scans and use Quarantine… on a flagged file.
bigman quarantine REPORT_ID ITEM_INDEX
bigman quarantined
bigman restore QUARANTINE_ID
```

## Optional deterministic tools

```sh
bigman setup clamav                     # preview repository installation
bigman setup clamav --apply
bigman setup shellcheck --apply
bigman setup sandbox --apply            # bubblewrap + strace
bigman setup pdf --apply                # Poppler utilities
bigman setup container --apply          # Podman, for bigman try
bigman audit lynis
bigman audit osv ./project              # may query advisory services
bigman audit osquery
bigman yara ./reviewed-rules.yar ./Downloads
```

Install PDFiD/pdf-parser, OSV-Scanner, osquery and YARA-X through their upstream or distro-supported distribution methods. The PDF parser isolation profile supports executables under `/usr/bin` or `/usr/local/bin`; user virtualenv-only tools are not automatically given home-directory access. YARA-X requires a reviewed local rules file; no unreviewed rule feed is fetched.

OSV integration targets its `scan source --format=json` interface. OSV identifies known vulnerabilities, not malware; its advisory queries are an explicit network action. Lynis runs as the user, so privileged checks may be unavailable. OpenSnitch provides its own firewall interface. Bigman does not enable arbitrary firewall rules or run a second system audit collector. Socket/LLM classification is deliberately absent to keep decisions non-AI.
