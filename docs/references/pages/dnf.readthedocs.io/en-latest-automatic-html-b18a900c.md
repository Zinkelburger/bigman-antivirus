---
url: "https://dnf.readthedocs.io/en/latest/automatic.html"
title: "DNF Automatic — DNF @DNF_VERSION@-1 documentation"
captured: "2026-11-02T18:27:34Z"
via: "live"
sha256: "23e43f508fa04c0b3d386af762dae3734992da6206a23e9f298ab82a49984f61"
wayback: "https://web.archive.org/web/20261025224714/https://dnf.readthedocs.io/en/latest/automatic.html"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# DNF Automatic — DNF @DNF_VERSION@-1 documentation

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://dnf.readthedocs.io/en/latest/automatic.html>
> Snapshot: <https://web.archive.org/web/20261025224714/https://dnf.readthedocs.io/en/latest/automatic.html>

- https://dnf.readthedocs.io/en/latest/index.html
- DNF Automatic
-  [Edit on GitHub](https://github.com/rpm-software-management/dnf/blob/master/doc/automatic.rst)

# DNF Automatic[](https://dnf.readthedocs.io/en/latest/automatic.html#dnf-automatic)

## Synopsis[](https://dnf.readthedocs.io/en/latest/automatic.html#synopsis)

dnf-automatic [<config file>]

## Description[](https://dnf.readthedocs.io/en/latest/automatic.html#description)

Alternative CLI to dnf upgrade with specific facilities to make it suitable to be executed automatically and regularly from systemd timers, cron jobs and similar.

The operation of the tool is usually controlled by the configuration file or the function-specific timer units (see below). The command only accepts a single optional argument pointing to the config file, and some control arguments intended for use by the services that back the timer units. If no configuration file is passed from the command line, /etc/dnf/automatic.conf is used.

The tool synchronizes package metadata as needed and then checks for updates available for the given system and then either exits, downloads the packages or downloads and applies the packages. The outcome of the operation is then reported by a selected mechanism, for instance via the standard output, email or MOTD messages.

The systemd timer unit dnf-automatic.timer will behave as the configuration file specifies (see below) with regard to whether to download and apply updates. Some other timer units are provided which override the configuration file with some standard behaviours:

-

dnf-automatic-notifyonly

-

dnf-automatic-download

-

dnf-automatic-install

Regardless of the configuration file settings, the first will only notify of available updates. The second will download, but not install them. The third will download and install them.

## Run dnf-automatic[](https://dnf.readthedocs.io/en/latest/automatic.html#run-dnf-automatic)

You can select one that most closely fits your needs, customize /etc/dnf/automatic.conf for any specific behaviors, and enable the timer unit.

For example: systemctl enable --now dnf-automatic-notifyonly.timer

## Configuration File Format[](https://dnf.readthedocs.io/en/latest/automatic.html#configuration-file-format)

The configuration file is separated into topical sections.

### [commands] section[](https://dnf.readthedocs.io/en/latest/automatic.html#commands-section)

Setting the mode of operation of the program.

apply_updates

boolean, default: False

Whether packages comprising the available updates should be applied by dnf-automatic.timer, i.e. installed via RPM. Implies download_updates. Note that if this is set to False, downloaded packages will be left in the cache till the next successful DNF transaction. Note that the other timer units override this setting.

download_updates

boolean, default: False

Whether packages comprising the available updates should be downloaded by dnf-automatic.timer. Note that the other timer units override this setting.

network_online_timeout

time in seconds, default: 60

Maximal time dnf-automatic will wait until the system is online. 0 means that network availability detection will be skipped.

random_sleep

time in seconds, default: 0

Maximal random delay before downloading. Note that, by default, the systemd timers also apply a random delay of up to 1 hour.

upgrade_type

either one of default, security, default: default

What kind of upgrades to look at. default signals looking for all available updates, security only those with an issued security advisory.

systemd_inhibit

boolean, default: True

Whether to block system shutdown while applying package updates. When enabled, dnf-automatic acquires a systemd inhibitor lock during the transaction phase to prevent scheduled reboots or system shutdown from interrupting package installation. The lock is automatically released when the transaction completes. If systemd-logind is unavailable (e.g., in containers), this setting has no effect. Set to False to disable if the inhibitor conflicts with your maintenance workflows.

reboot

either one of never, when-changed, when-needed, default: never

When the system should reboot following upgrades. never does not reboot the system. when-changed triggers a reboot after any upgrade. when-needed triggers a reboot only when rebooting is necessary to apply changes, such as when systemd or the kernel is upgraded.

reboot_command

string, default: shutdown -r +5 'Rebooting after applying package updates'

Specify the command to run to trigger a reboot of the system. For example, to skip the 5-minute delay and wall message, use shutdown -r

### [emitters] section[](https://dnf.readthedocs.io/en/latest/automatic.html#emitters-section)

Choosing how the results should be reported.

emit_via

list, default: email, stdio, motd

List of emitters to report the results through. Available emitters are stdio to print the result to standard output, command to send the result to a custom command, command_email to send an email using a command, and email to send the report via email and motd sends the result to /etc/motd file.

system_name

string, default: hostname of the given system

How the system is called in the reports.

send_error_messages

boolean, default: False

Invokes emitters when an error occurs.

### [command] section[](https://dnf.readthedocs.io/en/latest/automatic.html#command-section)

The command emitter configuration. Variables usable in format string arguments are body with the message body.

command_format

format string, default: cat

The shell command to execute.

stdin_format

format string, default: {body}

The data to pass to the command on stdin.

### [command_email] section[](https://dnf.readthedocs.io/en/latest/automatic.html#command-email-section)

The command email emitter configuration. Variables usable in format string arguments are body with message body, subject with email subject, email_from with the “From:” address and email_to with a space-separated list of recipients.

command_format

format string, default: mail -Ssendwait -s {subject} -r {email_from} {email_to}

The shell command to execute.

email_from

string, default: root

Message’s “From:” address.

email_to

list, default: root

List of recipients of the message.

stdin_format

format string, default: {body}

The data to pass to the command on stdin.

### [email] section[](https://dnf.readthedocs.io/en/latest/automatic.html#email-section)

The email emitter configuration.

email_from

string, default: root

Message’s “From:” address.

email_host

string, default: localhost

Hostname of the SMTP server used to send the message.

email_port

integer, default: 25

Port number to connect to at the SMTP server.

email_tls

either one of no, yes, starttls, default: no

Whether to use TLS, STARTTLS or no encryption to connect to the SMTP server.

email_to

list, default: root

List of recipients of the message.

### [base] section[](https://dnf.readthedocs.io/en/latest/automatic.html#base-section)

Can be used to override settings from DNF’s main configuration file. See [DNF Configuration Reference](https://dnf.readthedocs.io/en/latest/conf_ref.html).
