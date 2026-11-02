---
url: "https://discourse.gnome.org/t/use-virustotal-to-check-gnome-extensions-for-malicious-code/29281"
title: "Use VirusTotal to check Gnome extensions for malicious code"
site: "GNOME Discourse"
published: "2025-06-06T05:03:12+00:00"
captured: "2026-11-02T18:26:37Z"
via: "live"
sha256: "60ded0733478e3d627917c2a77b4a85e9e8aad16efbdfc233f93b279889cb819"
wayback: "https://web.archive.org/web/20251021122252/https://discourse.gnome.org/t/use-virustotal-to-check-gnome-extensions-for-malicious-code/29281"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Use VirusTotal to check Gnome extensions for malicious code

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://discourse.gnome.org/t/use-virustotal-to-check-gnome-extensions-for-malicious-code/29281>
> Snapshot: <https://web.archive.org/web/20251021122252/https://discourse.gnome.org/t/use-virustotal-to-check-gnome-extensions-for-malicious-code/29281>

[GNOME Discourse](https://discourse.gnome.org/)

#  [Use VirusTotal to check Gnome extensions for malicious code](https://discourse.gnome.org/t/use-virustotal-to-check-gnome-extensions-for-malicious-code/29281)

[Desktop](https://discourse.gnome.org/c/desktop/6)

[extensions](https://discourse.gnome.org/tag/extensions)

[49studebaker](https://discourse.gnome.org/u/49studebaker)  (49studebaker)    June 6, 2025, 5:03am   1

The Gnome security team has limited resources and might not be able to verify that every Gnome extension is safe. Extensions could be automatically submitted to VirusTotal to check the extension for malicious code.

[jrahmatzadeh](https://discourse.gnome.org/u/jrahmatzadeh)  (Javad Rahmatzadeh)    June 6, 2025, 8:37am   2

You mean we should have strict rules for [scripts and binaries in EGO Review Guidelines](https://gjs.guide/extensions/review-guidelines/review-guidelines.html#scripts-and-binaries)?

[skeller](https://discourse.gnome.org/u/skeller)  (Sebastian Keller)    June 6, 2025, 10:06am   3

The virus you linked to isn’t really related to gnome extensions. It itself is not an extension nor was it distributed via the extension system and as far as I know made no use of any other extension mechanism. It only installed itself into that directory to disguise itself.

[argentwolf](https://discourse.gnome.org/u/argentwolf)     June 6, 2025, 10:14am   4

Wait, I’ve heard (ad nauseam) GNU/Linux was immune to “malware”? :-J

[tragivictoria](https://discourse.gnome.org/u/tragivictoria)  (Victoria Niedzielska 🏳️‍⚧️)    June 6, 2025, 2:04pm   5

People who say this don’t know how anything works. There is nothing in Linux nor desktop Linux ecosystem that would be inherently “immune” to malware.

[49studebaker](https://discourse.gnome.org/u/49studebaker)  (49studebaker)    June 6, 2025, 4:08pm   6

Is it safe to download gnome extensions from the following website? Is it just as safe as installing extensions via the package manager?

https://extensions.gnome.org/

[jrahmatzadeh](https://discourse.gnome.org/u/jrahmatzadeh)  (Javad Rahmatzadeh)    June 6, 2025, 4:41pm   7

The best place to download and install GNOME Shell extensions is from [EGO](http://extensions.gnome.org/) since we review all of them.

If you are not comfortable using the EGO website, you can use [Extension Manager](https://flathub.org/apps/com.mattjakeman.ExtensionManager). It’s using the EGO’s API.

Btw, all reviews are public. You can find them by going to the extension page → Versions section → click on one of the links in the Status section.

If you found any security issues you can directly talk to us on [GNOME Extensions Matrix Channel](https://matrix.to/#/#extensions:gnome.org).

I actually, invite all extension users and developers to that channel since that’s the fastest way to get in touch with reviewers, shell developers and extension developers. We have many friendly people there willing to help you and answer all of the questions related to the GNOME Shell extensions.

[tragivictoria](https://discourse.gnome.org/u/tragivictoria)  (Victoria Niedzielska 🏳️‍⚧️)    June 6, 2025, 6:57pm   8

Thank you for all your work BTW!

[49studebaker](https://discourse.gnome.org/u/49studebaker)  (49studebaker)    June 7, 2025, 1:39am   9

Thank you for the information and your hard work.

[49studebaker](https://discourse.gnome.org/u/49studebaker)  (49studebaker)    June 7, 2025, 1:43am   10

Any operating system can get malware. Linux is as secure as you make it. If you install software from the package manager, use official repositories, and install security updates, your system will be very secure. However, if you add third-party repositories, download and execute software or scripts from websites, run commands you do not understand that are posted on the internet, or fail to install security updates, your system will likely be compromised.

[argentwolf](https://discourse.gnome.org/u/argentwolf)     June 25, 2025, 7:31am   12

There is NO “very secure” when any/all giant tech is involved. the public has been purposefully and thoroughly Punk’d! as we discover the interweb a surveillance and collection matrix working exactly as intended with GNU/Linux at its terrestrial center.

[Billions of Passwords Have Been Leaked in Massive Breach, Researchers Say. Here’s What You Should Know](https://time.com/7296254/passwords-leaked-data-breach/)
