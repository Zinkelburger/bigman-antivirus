---
url: "https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/"
title: "KDE advises extreme caution after theme wipes Linux user's files"
site: "BleepingComputer"
captured: "2026-11-02T18:52:07Z"
via: "live"
sha256: "8f05f8e1c368022eacb98bffba7482359ef6baa293b76c7dba2c45ba8b030134"
wayback: "https://web.archive.org/web/20260926192932/https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# KDE advises extreme caution after theme wipes Linux user's files

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/>
> Snapshot: <https://web.archive.org/web/20260926192932/https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/>

https://www.bleepingcomputer.com/

- https://www.facebook.com/BleepingComputer
- https://twitter.com/BleepinComputer
- https://infosec.exchange/@BleepingComputer
- https://www.bleepingcomputer.com/feed/

https://www.wiz.io/lp/ai-threat-readiness-playbook?utm_source=bleepingcomputer&utm_medium=display&utm_campaign=FY27Q2_INB_FORM_AI-Threat-Readiness-Playbook&sfcid=701Vh00000cnU0lIAE&utm_term=FY27Q4-bleepingcomputer-970x250-Nov&utm_content=AITR-Playbook

- [Home](https://www.bleepingcomputer.com/)
- [News](https://www.bleepingcomputer.com/news/)
- [Linux](https://www.bleepingcomputer.com/news/linux/)
- KDE advises extreme caution after theme wipes Linux user's files

# KDE advises extreme caution after theme wipes Linux user's files

By

###### [Sergiu Gatlan](https://www.bleepingcomputer.com/author/sergiu-gatlan/)

- March 21, 2024
- 03:05 PM
- [10](https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#comments)

On Wednesday, the KDE team warned Linux users to exercise "extreme caution" when installing global themes, even from the official KDE Store, because these themes run arbitrary code on devices to customize the desktop's appearance.

The KDE Store currently allows anyone to upload new themes and various other plugins or add-ons without any checks for malicious behavior.

However, as KDE said, it currently lacks the resources to review the code used by each global theme submitted for inclusion in its official store. If the themes are faulty or malicious, this can result in unexpected consequences.

https://www.wiz.io/lp/ai-threat-readiness-playbook?utm_source=bleepingcomputer&utm_medium=display&utm_campaign=FY27Q2_INB_FORM_AI-Threat-Readiness-Playbook&sfcid=701Vh00000cnU0lIAE&utm_term=FY27Q4-bleepingcomputer-970x250-Nov&utm_content=AITR-Playbook

"Global themes and widgets created by 3rd party developers for Plasma can and will run arbitrary code. You are encouraged to exercise extreme caution when using these products," KDE [cautioned](https://floss.social/@kde/112128243960545659).

"Global themes do not only change the look of Plasma, but also the behavior. To do this they run code, and this code can be faulty, as in the case mentioned above. The same goes for widgets and plasmoids."

Code execution is needed because global themes are designed to change everything on a Plasma desktop, from icons to windows decorations, lock screens, splash screens, wallpapers, color schemes, and so on, using executable bash scripts.

## Global theme wipes user's files using 'rm -rf'

According to a Reddit post quoted by KDE, at least one user had their files wiped after installing one such global Plasma theme.

After it was installed, the theme deleted all personal data from mounted drives using 'rm -rf', a very dangerous UNIX command that forcefully and recursively deletes a directory's contents (including files and other folders) without any warnings and prompting for confirmation.

When this command is used to delete files, they are permanently wiped and can only be recovered using data recovery software or restored from backups.

While the faulty global theme has since been removed from KDE's store, any other global themes available through KDE's official plugin repository could cause data loss if the developers haven't thoroughly tested them before submission.

"It executes rm -rf on your behalf [and] deletes all personal data immediately. No questions asked," the KDE user [warned](https://www.reddit.com/r/kde/comments/1bixmbx/do_not_install_global_themes_some_wipe_out_all/). "I canceled this when it asked for my root password, but it was too late for my personal data. All drives mounted under my user were gone, down to 0 bytes. [G]ames, configurations, browser data, [and the] home folder [are] all gone."

"Then it threw some sort of error, [and] my plasma kind of got stuck. [T]hen I checked and my two hard-drives were fully erased, games, configurations, personal data, all gone. Any drive mounted with user permissions also wiped out," the user [added](https://www.reddit.com/r/openSUSE/comments/1biunsl/hacked_installed_a_global_theme_it_erased_all_my/) in a separate Reddit post.

## KDE promises to start vetting store content

In light of the risks behind installing unvetted Plasma plugins, KDE asked the community to report faulty software already available through the KDE Store.

The team also promised to curate store content and improve the warnings shown to users before installing community-developed themes and plugins on their systems.

"[W]e need to communicate clearly what security expectations Plasma users should have for extensions they download onto their desktops," [said](http://blog.davidedmundson.co.uk/blog/kde-store-content/) David Edmundson, a Software Engineer and Project Lead at KDE. "Then, we can look at providing curation and auditing as part of the store process in combination with slowly improving sandbox support."

"If you install content from the store, I would advise checking it locally or looking for reviews from trusted sources."

"Nevertheless, this will take time and resources. We recommend all users to be careful when installing and running software not provided directly by KDE or your distros," the KDE team [added](https://floss.social/@kde/112128258445603141).

Until then, users will still be warned when installing global themes from the KDE system settings: "The content available here has been uploaded by users like you and has not been reviewed by your distributor for functionality or stability."

Warning before installing a global theme in KDE (David Edmundson)

https://www.adaptivesecurity.com/lp/nb/email-scan?utm_source=display_network&utm_medium=paid_display&utm_campaign=2026_NA_Q4_Bleepingcomputer_Inadunit_October2026_scan&utm_id=701Rd00000sqmWCIAY

##  [Uncover email threats your existing defenses may miss.](https://www.adaptivesecurity.com/lp/nb/email-scan?utm_source=display_network&utm_medium=paid_display&utm_campaign=2026_NA_Q4_Bleepingcomputer_Inadunit_October2026_scan&utm_id=701Rd00000sqmWCIAY)

Adaptive Security's AI agents analyze email intent, context, and behavior to uncover phishing, business email compromise, and invoice fraud.

See what's getting through with a 14-day read-only scan of Microsoft 365 or Google Workspace, with no MX changes.

[Start your free email security scan.](https://www.adaptivesecurity.com/lp/nb/email-scan?utm_source=display_network&utm_medium=paid_display&utm_campaign=2026_NA_Q4_Bleepingcomputer_Inadunit_October2026_scan&utm_id=701Rd00000sqmWCIAY)

### Related Articles:

[Synology warns of C3RB3R ransomware attacks targeting NAS devices](https://www.bleepingcomputer.com/news/security/synology-warns-customers-of-ongoing-c3rb3r-ransomware-attacks/)

[Alpine Linux rebuilds GitLab after attempted RCE attack](https://www.bleepingcomputer.com/news/security/alpine-linux-rebuilds-gitlab-after-attempted-rce-attack/)

[Critical Linux kernel flaw allows remote root attacks on MPTCP servers](https://www.bleepingcomputer.com/news/security/critical-linux-kernel-flaw-allows-remote-root-attacks-on-mptcp-servers/)

[GitLab warns of new max severity code execution vulnerability](https://www.bleepingcomputer.com/news/security/gitlab-warns-of-new-max-severity-code-execution-vulnerability/)

[CISA warns of Fragnesia Linux flaw chained in PHP2Shell attacks](https://www.bleepingcomputer.com/news/security/cisa-warns-of-fragnesia-linux-flaw-chained-in-php2shell-attacks/)

- [KDE](https://www.bleepingcomputer.com/tag/kde/)
- [Linux](https://www.bleepingcomputer.com/tag/linux/)
- [Theme](https://www.bleepingcomputer.com/tag/theme/)
- [Warning](https://www.bleepingcomputer.com/tag/warning/)

-
-
-

-
- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#

https://www.bleepingcomputer.com/author/sergiu-gatlan/

##### [Sergiu Gatlan](https://www.bleepingcomputer.com/author/sergiu-gatlan/)   https://twitter.com/serghei

Sergiu is a news reporter who has covered the latest cybersecurity and technology developments for over a decade. Email or Twitter DMs for tips.

- [Previous Article](https://www.bleepingcomputer.com/offer/deals/this-windows-11-pro-and-microsoft-office-2019-bundle-is-now-under-50/)
- [Next Article](https://www.bleepingcomputer.com/news/microsoft/windows-11-notepad-finally-gets-spellcheck-and-autocorrect/)

### Comments

-

######  [ReaperX7](https://www.bleepingcomputer.com/forums/u/1310052/reaperx7/)  - 2 years ago

- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#
- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#cid31375

They should have done this vetting years ago. This is just gross negligence on the part of KDE.

I'm actually glad Xfce is still one of the more sane desktops to use.

-

######  [GT500](https://www.bleepingcomputer.com/forums/u/377072/gt500/)  - 2 years ago

- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#
- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#cid31377

They shouldn't be allowing themes to execute code of any kind. That's a massive security risk.

-

######  [MobileJAD](https://www.bleepingcomputer.com/forums/u/1310248/mobilejad/)  - 2 years ago

- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#
- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#cid31400

You would think a theme would just be a different set of graphics for the user interface, wouldn't it?

-

######  [GT500](https://www.bleepingcomputer.com/forums/u/377072/gt500/)  - 2 years ago

- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#
- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#cid31402

"You would think a theme would just be a different set of graphics for the user interface, wouldn't it? "

That's what a theme is supposed to be. Style information and nothing else. The very idea of a theme being able to execute any sort of code, even just a script, is absolutely ludicrous.

-

######  [0lds0d](https://www.bleepingcomputer.com/forums/u/1151306/0lds0d/)  - 2 years ago

- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#
- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#cid31382

Very upsetting, to say at the very least.

-

######  [Alan_Peery](https://www.bleepingcomputer.com/forums/u/1310126/alan-peery/)  - 2 years ago

- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#
- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#cid31386

KDE has a gaping security hole, and all theme functionally that can run code *must* be disabled.

Vetting themes is not an adequate solution.

-

######  [Nekocat](https://www.bleepingcomputer.com/forums/u/1310264/nekocat/)  - 2 years ago

- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#
- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#cid31403

Well, the store requires root access so ...

-

######  [greg18](https://www.bleepingcomputer.com/forums/u/1138479/greg18/)  - 2 years ago

- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#
- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#cid31407

"Well, the store requires root access so ..."

You have to grant it that. You can also disable root, alias rm and other functions. It is easier to harden Linux than it is with Windows.

-

######  [Thomas_i90](https://www.bleepingcomputer.com/forums/u/1310435/thomas-i90/)  - 2 years ago

- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#
- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#cid31425

Sounds like the same issue when installing python-packages infected with malware. Very concerning.

-

######  [rhasce](https://www.bleepingcomputer.com/forums/u/1062113/rhasce/)  - 2 years ago

- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#
- https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#cid31438

OMG bless me for running xfce on my Debian daily runner laptop lol and I hate themes so I am good on this front lol

##### Post a Comment [Community Rules](https://www.bleepingcomputer.com/posting-guidelines/)

###### You need to login in order to post a comment

Not a member yet? [Register Now](https://www.bleepingcomputer.com/forums/index.php?app=core&module=global&section=register)

### You may also like:

https://www.kaseya.com/lp/next-gen-rmm?utm_source=701Rp00000y8fGN&utm_medium=Other

Popular Stories

-  [Critical Linux kernel flaw allows remote root attacks on MPTCP servers](https://www.bleepingcomputer.com/news/security/critical-linux-kernel-flaw-allows-remote-root-attacks-on-mptcp-servers/)
-  [New Android SMS of Death exploit can remotely crash unpatched phones](https://www.bleepingcomputer.com/news/security/new-android-sms-of-death-exploit-can-remotely-crash-unpatched-phones/)
-  [CISA says max severity GitLab flaw now exploited in the wild](https://www.bleepingcomputer.com/news/security/hackers-now-exploiting-max-severity-gitlab-flaw-in-attacks/)

Sponsor Posts

-  [Your carrier tracks you. We don’t. Switch to Cape, America’s Privacy-first Mobile Carrier.](https://www.cape.co/?utm_source=BleepingComputer&utm_medium=article_sponsorship&utm_campaign=BleepingComputer_ArticleSponsorship_Nov2)
-  [See every AI tool in use. Control what data reaches it. Try Keep Aware for free.](https://keepaware.com/demo/generative-ai-monitoring?utm_source=bleeping_computer&utm_medium=article&utm_campaign=october26)
-  [Want to break into cyber security? Learn hands-on in your browser with TryHackMe.](https://tryhackme.com/)
-  [Stay one step ahead of new threats in the new year. Join Huntress for the monthly Tradecraft Tuesday.](https://www.huntress.com/community-series/tradecraft-tuesday?utm_source=bleepingcomputer&utm_medium=resource_link&utm_campaign=cy26-09-camp-multi-global-prospect-all-x-x-social_eng-bleeping_computer&hnt=nsnzaa2spign)
-  [Identity-based order to control the chaos of your agentic AI. See Token Security’s AI Agent Security](https://www.token.security/book-a-demo?utm_source=bleepingcomputer&utm_medium=3rd-party&utm_campaign=bleepingcomputer&utm_content=oct-28)
-  [Your carrier tracks you. We don’t. Switch to Cape, America’s Privacy-first Mobile Carrier.](https://www.cape.co/?utm_source=BleepingComputer&utm_medium=article_sponsorship&utm_campaign=BleepingComputer_ArticleSponsorship_Oct30)

https://www.threatlocker.com/stop-judgment-day?utm_source=bleeping_computer&utm_medium=sponsor&utm_campaign=&utm_content=judgment_day_sticky_sidebar-400x500&utm_term=display

https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#

#### Login

#### Reporter

###### Help us understand the problem. What is going on with this comment?

Submitting...

[SUBMIT](https://www.bleepingcomputer.com/news/linux/kde-advises-extreme-caution-after-theme-wipes-linux-users-files/#)
