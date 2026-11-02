---
url: "https://www.bleepingcomputer.com/news/security/hackers-now-testing-clickfix-attacks-against-linux-targets/"
title: "Hackers now testing ClickFix attacks against Linux targets"
site: "BleepingComputer"
captured: "2026-11-02T18:54:45Z"
via: "live"
sha256: "cfa498936e189f425d4fba6e734846fd30f61c5cd5946e8091452c1537a1d79e"
wayback: "https://web.archive.org/web/20261021214802/https://www.bleepingcomputer.com/news/security/hackers-now-testing-clickfix-attacks-against-linux-targets/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# Hackers now testing ClickFix attacks against Linux targets

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.bleepingcomputer.com/news/security/hackers-now-testing-clickfix-attacks-against-linux-targets/>
> Snapshot: <https://web.archive.org/web/20261021214802/https://www.bleepingcomputer.com/news/security/hackers-now-testing-clickfix-attacks-against-linux-targets/>

# Hackers now testing ClickFix attacks against Linux targets

By

###### [Bill Toulas](https://www.bleepingcomputer.com/author/bill-toulas/)

- May 12, 2025
- 02:10 PM
- [0](https://www.bleepingcomputer.com/news/security/hackers-now-testing-clickfix-attacks-against-linux-targets/#comment_form)

A new campaign employing ClickFix attacks has been spotted targeting both Windows and Linux systems using instructions that make infections on either operating system possible.

ClickFix is a [social engineering tactic](https://www.bleepingcomputer.com/news/security/iclicker-hack-targeted-students-with-malware-via-fake-captcha/) where fake verification systems or application errors are used to trick website visitors into running console commands that install malware.

These attacks have traditionally targeted Windows systems, prompting targets to execute PowerShell scripts from the Windows Run command, resulting in [info-stealer malware](https://www.bleepingcomputer.com/news/security/over-6-000-wordpress-sites-hacked-to-install-plugins-pushing-infostealers/) infections and [even ransomware](https://www.bleepingcomputer.com/news/security/interlock-ransomware-gang-pushes-fake-it-tools-in-clickfix-attacks/).

https://www.wiz.io/lp/ai-threat-readiness-playbook?utm_source=bleepingcomputer&utm_medium=display&utm_campaign=FY27Q2_INB_FORM_AI-Threat-Readiness-Playbook&sfcid=701Vh00000cnU0lIAE&utm_term=FY27Q4-bleepingcomputer-970x250-Nov&utm_content=AITR-Playbook

However, a 2024 campaign using bogus Google Meet errors also [targeted macOS users](https://www.bleepingcomputer.com/news/security/fake-google-meet-conference-errors-push-infostealing-malware/).

## ClickFix targeting Linux users

A more recent campaign spotted by [Hunt.io researchers](https://hunt.io/blog/apt36-clickfix-campaign-indian-ministry-of-defence) last week is among the first to adapt this social engineering technique for Linux systems.

The attack, which is attributed to the Pakistan-linked threat group APT36 (aka "Transparent Tribe"), utilizes a website that impersonates India's Ministry of Defence with a link to an allegedly official press release.

Malicious website mimicking India's Ministry of Defence
Source: Hunt.io

When visitors click on this website link, they are profiled by the platform to determine their operating system, and then redirected to the correct attack flow.

On Windows, victims are served a full-screen page warning them of limited content usage rights. Clicking on 'Continue' triggers JavaScript that copies a malicious MSHTA command to the victim's clipboard, who is instructed to paste and execute it on the Windows terminal.

This launches a .NET-based loader which connects to the attacker's address, while the user sees a decoy PDF file to make everything appear legitimate and as expected.

On Linux, victims are redirected to a CAPTCHA page that copies a shell command to their clipboard when clicking the "I'm not a robot button."

The victim is then guided to press ALT+F2 to open a Linux run dialog, paste the command into it, and then press Enter to execute it.

Instructions for Linux users
Source: Hunt.io

The command drops the 'mapeal.sh' payload on the target's system, which, according to Hunt.io, does not perform any malicious actions in its current version, limited to fetching a JPEG image from the attacker's server.

Linux ClickFix script
Source: BleepingComputer

"The script downloads a JPEG image from the same trade4wealth[.]in directory and opens it in the background," explains Hunt.io.

"No additional activity, such as persistence mechanisms, lateral movement, or outbound communication, was observed during execution."

However, it is possible that APT36 is currently experimenting to determine the effectiveness of the Linux infection chain, as they would just need to swap out the image for a shell script to install malware or perform other malicious activity.

The adaptation of ClickFix to carry out attacks on Linux is another testament to its effectiveness, as the attack type has now been used against all three major desktop OS platforms.

As a general policy, users should not copy and paste any commands into Run dialogs without knowing exactly what the command does. Doing so only increases the risk of a malware infection and theft of sensitive data.

https://www.adaptivesecurity.com/lp/nb/email-scan?utm_source=display_network&utm_medium=paid_display&utm_campaign=2026_NA_Q4_Bleepingcomputer_Inadunit_October2026_scan&utm_id=701Rd00000sqmWCIAY

##  [Uncover email threats your existing defenses may miss.](https://www.adaptivesecurity.com/lp/nb/email-scan?utm_source=display_network&utm_medium=paid_display&utm_campaign=2026_NA_Q4_Bleepingcomputer_Inadunit_October2026_scan&utm_id=701Rd00000sqmWCIAY)

Adaptive Security's AI agents analyze email intent, context, and behavior to uncover phishing, business email compromise, and invoice fraud.

See what's getting through with a 14-day read-only scan of Microsoft 365 or Google Workspace, with no MX changes.

[Start your free email security scan.](https://www.adaptivesecurity.com/lp/nb/email-scan?utm_source=display_network&utm_medium=paid_display&utm_campaign=2026_NA_Q4_Bleepingcomputer_Inadunit_October2026_scan&utm_id=701Rd00000sqmWCIAY)

### Related Articles:

[Hackers hijack HBO Max Reddit account to push malware in ClickFix ads](https://www.bleepingcomputer.com/news/security/hackers-hijack-hbo-max-reddit-account-to-push-malware-in-clickfix-ads/)

[Alpine Linux rebuilds GitLab after attempted RCE attack](https://www.bleepingcomputer.com/news/security/alpine-linux-rebuilds-gitlab-after-attempted-rce-attack/)

[Critical Linux kernel flaw allows remote root attacks on MPTCP servers](https://www.bleepingcomputer.com/news/security/critical-linux-kernel-flaw-allows-remote-root-attacks-on-mptcp-servers/)

[CISA warns of Fragnesia Linux flaw chained in PHP2Shell attacks](https://www.bleepingcomputer.com/news/security/cisa-warns-of-fragnesia-linux-flaw-chained-in-php2shell-attacks/)

[The Huntress Tragic Quadrant: Ranking Threats Hitting Businesses Hardest](https://www.bleepingcomputer.com/news/security/the-huntress-tragic-quadrant-ranking-threats-hitting-businesses-hardest/)
