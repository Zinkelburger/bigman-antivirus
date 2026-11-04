---
url: "https://www.theregister.com/2017/07/20/bug_of_the_week_gnome_files_msi/"
title: "Crazy bug of the week: Gnome Files' .MSI parser runs evil VBScripts"
site: "theregister"
published: "2017-07-20T06:30:04.000Z"
captured: "2026-11-02T19:02:27Z"
via: "live"
source_url: "https://www.theregister.com/security/2017/07/20/crazy-bug-of-the-week-gnome-files-msi-parser-runs-evil-vbscripts/1304942"
sha256: "42aeb59a2154578c7c95f1ec3578718f8d4d8643b3e2946b619d448198304072"
wayback: "https://web.archive.org/web/20260124114619/https://www.theregister.com/2017/07/20/bug_of_the_week_gnome_files_msi/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Crazy bug of the week: Gnome Files' .MSI parser runs evil VBScripts

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.theregister.com/2017/07/20/bug_of_the_week_gnome_files_msi/>
> Snapshot: <https://web.archive.org/web/20260124114619/https://www.theregister.com/2017/07/20/bug_of_the_week_gnome_files_msi/>

[Security](https://www.theregister.com/tag/security)

#  Crazy bug of the week: Gnome Files' .MSI parser runs evil VBScripts

Grab a fix now

Richard Chirgwin [Richard Chirgwin](https://www.theregister.com/author/Richard-Chirgwin)

Published thu 20 Jul 2017 // 07:30 UTC

https://twitter.com/intent/tweet?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2017%2F07%2F20%2Fcrazy-bug-of-the-week-gnome-files-msi-parser-runs-evil-vbscripts%2F1304942 https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2017%2F07%2F20%2Fcrazy-bug-of-the-week-gnome-files-msi-parser-runs-evil-vbscripts%2F1304942  https://bsky.app/intent/compose?text=Crazy%20bug%20of%20the%20week%3A%20Gnome%20Files'%20.MSI%20parser%20runs%20evil%20VBScripts%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2017%2F07%2F20%2Fcrazy-bug-of-the-week-gnome-files-msi-parser-runs-evil-vbscripts%2F1304942 https://www.reddit.com/submit?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2017%2F07%2F20%2Fcrazy-bug-of-the-week-gnome-files-msi-parser-runs-evil-vbscripts%2F1304942&title=Crazy%20bug%20of%20the%20week%3A%20Gnome%20Files'%20.MSI%20parser%20runs%20evil%20VBScripts https://api.whatsapp.com/send?text=Crazy%20bug%20of%20the%20week%3A%20Gnome%20Files'%20.MSI%20parser%20runs%20evil%20VBScripts%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2017%2F07%2F20%2Fcrazy-bug-of-the-week-gnome-files-msi-parser-runs-evil-vbscripts%2F1304942 https://t.me/share/url?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2017%2F07%2F20%2Fcrazy-bug-of-the-week-gnome-files-msi-parser-runs-evil-vbscripts%2F1304942&text=Crazy%20bug%20of%20the%20week%3A%20Gnome%20Files'%20.MSI%20parser%20runs%20evil%20VBScripts [Make us preferred on Google](https://google.com/preferences/source?q=theregister.com)

Gnome developers, take a bow: a bug in your image thumbnailer has opened up a (not too scary, thankfully) hole for script injection.

The security vulnerability was revealed this week by Nils Dagsson Moskopp [here](http://news.dieweltistgarnichtso.net/posts/gnome-thumbnailer-msi-fail.html), and his advice for users is:

##

“Delete all files in /usr/share/thumbnailers. Do not use GNOME Files. Uninstall any other software that facilitates automatically executing parts of filenames as code” (our emphasis added).

Here's the rub: while creating thumbnail images, Gnome Files can, if you have Wine installed, accidentally execute parts of some filenames as if they were in a script – and that's problematic. Specifically, we're talking about .MSI files, aka Windows software installer packages: when you see one in Files, Gnome's software tries to make a thumbnail for it, and if there's any VBScript code in the filename, it may get executed. Damn.

REG AD

Imagine downloading a ZIP archive, and it unpacks a .MSI with a malicious filename. Bam, Gnome Files executes that code before you've clicked on anything else – if you have a viewer open for that directory, of course.

REG AD

Here's where it gets weird: Gnome's thumbnail generator for .MSI files uses [Wine](https://www.winehq.org/) to parse the contents of the installer file. The generator crafts a custom temporary script to run within Wine that looks inside the .MSI and pulls out information and stuff to create the thumbnail. But due to a programming blunder, VBScript in the filename can end up being executed, too.

“Whenever an icon for a Microsoft Windows executable (EXE), installer (MSI), library (DLL), or shortcut (LNK) should be shown, Gnome Files calls /usr/bin/gnome-exe-thumbnailer to either extract an embedded icon from the file in question or deliver a fallback image for the appropriate filetype,” said Dagsson Moskopp.

He picks out this single line of code in /usr/bin/gnome-exe-thumbnailer as the culprit:

He went on to explain:

```

DISPLAY=NONE wine cscript.exe //E:vbs //NoLogo Z:\\tmp\\${TEMPFILE1##*/}.vbs 2>/dev/null \

```

Dagsson Moskopp says developers should not use “ad-hoc parsers” to parse files, should “fully recognise inputs before processing them”, and should use unparsers.

This [issue](https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=868705) has now been [addressed](https://github.com/gnome-exe-thumbnailer/gnome-exe-thumbnailer/commit/1d8e3102dd8fd23431ae6127d14a236da6b4a4a5) by using a proper MSI parser rather than screwing around with scripts and Wine: make sure you fetch and install the latest updates for Gnome Files as soon as they land. If you don't have Wine installed, you're not at risk, of course. ®

##

“Instead of parsing an MSI file to get its version number, this code creates a script containing the filename for which a thumbnail should be shown and executes that using Wine. The script is constructed using a template, which makes it possible to embed VBScript in a filename and trigger its execution.”

[vulnerability](https://www.theregister.com/tag/vulnerability) [security](https://www.theregister.com/tag/security)

https://twitter.com/intent/tweet?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2017%2F07%2F20%2Fcrazy-bug-of-the-week-gnome-files-msi-parser-runs-evil-vbscripts%2F1304942 https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2017%2F07%2F20%2Fcrazy-bug-of-the-week-gnome-files-msi-parser-runs-evil-vbscripts%2F1304942  https://bsky.app/intent/compose?text=Crazy%20bug%20of%20the%20week%3A%20Gnome%20Files'%20.MSI%20parser%20runs%20evil%20VBScripts%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2017%2F07%2F20%2Fcrazy-bug-of-the-week-gnome-files-msi-parser-runs-evil-vbscripts%2F1304942 https://www.reddit.com/submit?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2017%2F07%2F20%2Fcrazy-bug-of-the-week-gnome-files-msi-parser-runs-evil-vbscripts%2F1304942&title=Crazy%20bug%20of%20the%20week%3A%20Gnome%20Files'%20.MSI%20parser%20runs%20evil%20VBScripts https://api.whatsapp.com/send?text=Crazy%20bug%20of%20the%20week%3A%20Gnome%20Files'%20.MSI%20parser%20runs%20evil%20VBScripts%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2017%2F07%2F20%2Fcrazy-bug-of-the-week-gnome-files-msi-parser-runs-evil-vbscripts%2F1304942 https://t.me/share/url?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2017%2F07%2F20%2Fcrazy-bug-of-the-week-gnome-files-msi-parser-runs-evil-vbscripts%2F1304942&text=Crazy%20bug%20of%20the%20week%3A%20Gnome%20Files'%20.MSI%20parser%20runs%20evil%20VBScripts [Make us preferred on Google](https://google.com/preferences/source?q=theregister.com)

[## Marvell wants to be a copper topper, bets $115B on photonics leader to push optical networking

IP house believes optics demand is going to explode as Nvidia and others eye bigger systems](https://www.theregister.com/systems/2026/11/02/marvell-wants-to-be-a-copper-topper-bets-115b-on-photonics-leader-to-push-optical-networking/5307028)

[## Oracle gobbles $20 billion borrowing limit a week after setting it

In the middle of a datacenter capex binge, vendor promises no more borrowing until the end of 2030](https://www.theregister.com/paas-and-iaas/2026/11/02/oracle-gobbles-20-billion-borrowing-limit-a-week-after-setting-it/5307014)

[## From expertise to intelligence: NISCO deploys world's first swarm-intelligence steel model at scale

PARTNER CONTENT: 300+ AI applications delivering surges in core metrics and annual cost reduction of over CNY100m](https://www.theregister.com/ai-and-ml/2026/11/02/partner-content-from-expertise-to-intelligence-nisco-deploys-worlds-first-swarm-intelligence-steel-model-at-scale/5306614)

[## Ericsson says it can detect, track drones using only mobile networks

ISAC technology adds detection ability to existing cell networks, enabling them to track low-flying small drones](https://www.theregister.com/networks/2026/11/02/ericsson-says-it-can-detect-track-drones-using-only-mobile-networks/5306958)

[OPINION

## Microsoft's moolah mountain is more massive than ever. It just doesn't want the people who built it

Flash the cash, flush the cache](https://www.theregister.com/columnists/2026/11/02/microsofts-moolah-mountain-is-more-massive-than-ever-it-just-doesnt-want-the-people-who-built-it/5306576)

[## Thieves pilfer environment variables from Trigger AI workflow tool

So, who's ready to clear the dust out of that server closet and move things back on-prem?](https://www.theregister.com/security/2026/11/02/thieves-pilfer-environment-variables-from-trigger-ai-workflow-tool/5306998)
