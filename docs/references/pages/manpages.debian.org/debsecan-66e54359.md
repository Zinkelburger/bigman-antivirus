---
url: "https://manpages.debian.org/debsecan"
title: "debsecan(1) — debsecan — Debian trixie — Debian Manpages"
captured: "2026-11-02T18:38:18Z"
via: "live"
source_url: "https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html"
sha256: "c159cf331e45266d106853459b52033d0c0ad0bbd5967c25cc0bcaace03e8e8b"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# debsecan(1) — debsecan — Debian trixie — Debian Manpages

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://manpages.debian.org/debsecan>

https://www.debian.org/

[MANPAGES](https://manpages.debian.org/)

[Skip Quicknav](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#content)

- [Index](https://manpages.debian.org/)
- [About Manpages](https://manpages.debian.org/about.html)
- [FAQ](https://manpages.debian.org/faq.html)
- [Service Information](https://wiki.debian.org/manpages.debian.org)

/ [trixie](https://manpages.debian.org/contents-trixie.html) / [debsecan](https://manpages.debian.org/trixie/debsecan/index.html) / debsecan(1)

links

-  [language-indep link](https://manpages.debian.org/trixie/debsecan/debsecan.1)
-  [package tracker](https://tracker.debian.org/pkg/debsecan)
-  [raw man page](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.gz)

table of contents

-  [NAME](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#NAME)
-  [SYNOPSIS](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#SYNOPSIS)
-  [DESCRIPTION](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#DESCRIPTION)
-  [OPTIONS](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#OPTIONS)
-  [CONFIGURATION FILE](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#CONFIGURATION_FILE)
-  [CONFIGURATION FILE MACROS](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#CONFIGURATION_FILE_MACROS)
-  [CHANGING THE WHITELIST](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#CHANGING_THE_WHITELIST)
-  [CAVEATS](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#CAVEATS)
-  [EXAMPLES](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#EXAMPLES)
-  [ENVIRONMENT](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#ENVIRONMENT)
-  [FILES](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#FILES)
-  [AUTHOR](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#AUTHOR)
-  [SEE ALSO](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#SEE_ALSO)

other versions

-  [trixie](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html) 0.4.20.1
-  [testing](https://manpages.debian.org/testing/debsecan/debsecan.1.en.html) 0.4.20.2
-  [unstable](https://manpages.debian.org/unstable/debsecan/debsecan.1.en.html) 0.4.20.2

[Scroll to navigation](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#panels)

| DEBSECAN(1)  |   | DEBSECAN(1)

# NAME[¶](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#NAME)

debsecan - Debian Security Analyzer

# SYNOPSIS[¶](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#SYNOPSIS)

debsecan options...

# DESCRIPTION[¶](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#DESCRIPTION)

debsecan analyzes the list of installed packages on the current host and reports vulnerabilities found on the system.

# OPTIONS[¶](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#OPTIONS)

[--suite count](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#suite)

Choose a specific suite. debsecan produces more informative output (including obsolete packages) if the correct suite is specified. The release code name has to be used ("sid"), not the temporal name ("unstable").

[--whitelist file](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#whitelist)

Change the name of the whitelist file.

[--add-whitelist, --remove-whitelist, --show-whitelist](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#add-whitelist)

Add or remove entries from the whitelist, or print the whitelist to standard output. See the CHANGING THE WHITELIST section below.

[--source url](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#source)

Override the default download URL for vulnerability data.

[--status file](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#status)

Evaluate a different dpkg status file.

[--format format](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#format)

Change the output format. If format is summary (the default), a short summary for each vulnerability is printed. The simple format is like the summary format, except that only the bug packages names are printed. For bugs and packages, debsecan lists the names of vulnerabilities and binary packages, respectively. --format detail requests a verbose output format, showing all available data. The report format is used for email reports.

[--line-length characters](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#line-length)

Specifies the line length in report mode. The default is 72.

[--mailto mailbox](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#mailto)

The --mailto option instructions debsecan to the send the report to the email address mailbox. No report is sent if there where no changes since the last invocation with --update-history. This option requires the --format report output format. The option value may contain macros, see the section CONFIGURATION FILE MACROS below.

[--only-fixed](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#only-fixed)

Only list vulnerabilities for which a fix is available in the archive. Note that it can happen that a fix is listed, although the package has not been built for the system's architecture and is not yet available for download. (If you use this option, you also must specify the correct suite using --suite.)

[--no-obsolete](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#no-obsolete)

Do not list any obsolete packages (see below). Using this option is not recommended because it hides real vulnerabilities on some systems, not just false positives.

[--history file](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#history)

Change the name of the history file used by --format report.

[--disable-https-check](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#disable-https-check)

Turn off certificate validation for HTTPS.

[--update-history](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#update-history)

Update the vulnerability status information after reporting it using --format report.

[--cron](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#cron)

Internal option used for invocations from cron. Checks if the vulnerability data has already been downloaded today. In this case, further processing is skipped. See [debsecan-create-cron(8)](https://manpages.debian.org/trixie/debsecan/debsecan-create-cron.8.en.html) for instructions how to create a suitable cron entry.

[--config file](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#config)

Sets the location of the configuration file.

[--help](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#help)

Display a short help message and exit.

[--version](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#version)

Display version information and exit.

# CONFIGURATION FILE[¶](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#CONFIGURATION_FILE)

The configuration file contains the following variables. It follows name=value shell syntax. If value contains white space, it must be surrounded by double quotes. Some variables may contain macros; see the section CONFIGURATION FILE MACROS below.

[MAILTO](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#MAILTO)

Sets the email address to which reports are sent in --cron mode. May contain macros.

[REPORT](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#REPORT)

Controls whether debsecan does any processing whatsoever in --cron mode. (Permitted values: true and false.)

[SOURCE](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#SOURCE)

Controls the URL from which vulnerability information is fetched. If empty, the built-in default is used.

[SUITE](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#SUITE)

Sets the default value of the --suite option (see there).

[SUBJECT](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#SUBJECT)

Changes the subject line of reports. May contain macros.

[DISABLE_HTTPS_CHECK](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#DISABLE_HTTPS_CHECK)

Disables HTTPS certificate checking, just like the --disable-https-check command line option.

# CONFIGURATION FILE MACROS[¶](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#CONFIGURATION_FILE_MACROS)

Macro processing replaces strings of the form %s(key)s with system-dependent values. Support keys are:

[hostname](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#hostname)

The host name on which debsecan runs, without the domain name part.

[fqdn](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#fqdn)

The fully-qualified domain name of the host on which debsecan runs.

[ip](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#ip)

The IP address of the host on which debsecan runs. This may be inaccurate on multi-homed systems.

# CHANGING THE WHITELIST[¶](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#CHANGING_THE_WHITELIST)

You can use the --add-whitelist and --remove-whitelist options to change the whitelist. Whitelisted vulnerabilities are not included in the reports. For example,

debsecan --add-whitelist CVE-2005-4601

ignores the vulnerability CVE-2005-4601 completely, while

debsecan --add-whitelist CVE-2005-4601 perlmagick

ignores it only as far as the perlmagick is concerned. (This is the same format that is produced by the --format simple option.) To remove all whitelist entries for the CVE-2005-4601 vulnerability, use:

debsecan --remove-whitelist CVE-2005-4601

If you want to remove an entry for a specific vulnerability/package pair, list the package name explicitly, as in:

debsecan --remove-whitelist CVE-2005-4601 imagemagick

You can list multiple vulnerability and packages. For example,

debsecan --add-whitelist CVE-2005-4601 \

CVE-2006-0082 imagemagick perlmagick

whitelists CVE-2005-4601 for all packages, and CVE-2006-0082 for the imagemagick and perlmagick packages only.

# CAVEATS[¶](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#CAVEATS)

Much like the official Debian security advisories, debsecan's vulnerability tracking is mostly based on source packages. This can be confusing because tools like dpkg only display binary package names. Therefore, debsecan displays the more familiar binary package names. This has the unfortunate effect that all binary packages (including packages containing only documentation, for example) are flagged as vulnerable, and not only those packages which actually contain the vulnerable code.

If the correct --suite option is specified, debsecan may mark some packages as obsolete. This means that the binary package in question has been removed from the archive. In this case, you need to update all the packages depending on the obsolete package, and subsequently remove the obsolete package.

For certain architectures, build daemons may lag considerably. In such case, debsecan may incorrectly mark a package as fixed, even if an update is not yet available in the Debian archive.

Note that debsecan version uses the --suite option only to determine the availability of corrected packages and to detect obsolete packages. If you specify the wrong suite, only the information on available security updates and obsolete packages is wrong, but the list of vulnerabilities is correct.

Mixing packages from different Debian releases is supported, as long as the packages still carry their official version numbers. Unknown package versions (from backported packages, for example) are compared to the version in Debian unstable only, which may lead to incorrect reports.

# EXAMPLES[¶](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#EXAMPLES)

This command prints all package names for which security fixes are available:

debsecan --suite suite --format packages --only-fixed

If you pass this output to apt-get, you can download new packages which contain security fixes. For example, if you are running sid:

apt-get install \

$(debsecan --suite sid --format packages --only-fixed)

The following command can be invoked periodically, to get notifications of new security issues:

debsecan --suite suite --format report \

--update-history --mailto root

See [debsecan-create-cron(8)](https://manpages.debian.org/trixie/debsecan/debsecan-create-cron.8.en.html) for a tool which creates a suitable cron entry.

# ENVIRONMENT[¶](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#ENVIRONMENT)

[https_proxy](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#https_proxy)

This environment variable instructs debsecan to use a proxy server to fetch the vulnerability data. It must be of the form http://proxy.example.net:8080/ (mimicking a URL).

# FILES[¶](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#FILES)

/etc/default/debsecan

Built-in location of the configuration file.

/var/lib/dpkg/status

File from which the package information is fetched by default.

# AUTHOR[¶](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#AUTHOR)

debsecan was written by Florian Weimer.

# SEE ALSO[¶](https://manpages.debian.org/trixie/debsecan/debsecan.1.en.html#SEE_ALSO)

[dpkg(1)](https://manpages.debian.org/trixie/dpkg/dpkg.1.en.html), [debsecan-create-cron(8)](https://manpages.debian.org/trixie/debsecan/debsecan-create-cron.8.en.html), [apt-get(8)](https://manpages.debian.org/trixie/apt/apt-get.8.en.html)

| 2005-12-23  |

|  Source file:   |  debsecan.1.en.gz (from [debsecan 0.4.20.1](http://snapshot.debian.org/package/debsecan/0.4.20.1/))

|  Source last updated:   |  2020-01-21T20:13:26Z

|  Converted to HTML:   |  2026-10-13T21:02:00Z

debiman HEAD, see [github.com/Debian/debiman](https://github.com/Debian/debiman/). Found a problem? See the [FAQ](https://manpages.debian.org/faq.html).
