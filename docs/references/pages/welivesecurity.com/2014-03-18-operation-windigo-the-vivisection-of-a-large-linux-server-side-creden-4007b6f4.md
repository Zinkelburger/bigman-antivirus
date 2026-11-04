---
url: "https://www.welivesecurity.com/2014/03/18/operation-windigo-the-vivisection-of-a-large-linux-server-side-credential-stealing-malware-campaign/"
title: "Operation Windigo – the vivisection of a large Linux server-side credential-stealing malware campaign"
captured: "2026-11-02T19:02:24Z"
via: "live"
sha256: "1f7cdddca2cb9adc08f30c6cf26d7f29cbcb46a25ff235d58a7950eacd45a863"
wayback: "https://web.archive.org/web/20260924004614/https://www.welivesecurity.com/2014/03/18/operation-windigo-the-vivisection-of-a-large-linux-server-side-credential-stealing-malware-campaign/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Operation Windigo – the vivisection of a large Linux server-side credential-stealing malware campaign

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.welivesecurity.com/2014/03/18/operation-windigo-the-vivisection-of-a-large-linux-server-side-credential-stealing-malware-campaign/>
> Snapshot: <https://web.archive.org/web/20260924004614/https://www.welivesecurity.com/2014/03/18/operation-windigo-the-vivisection-of-a-large-linux-server-side-credential-stealing-malware-campaign/>

https://www.welivesecurity.com/en/

Cybersecurity research and insight for the AI era · [Moving soon to the ESET Blog](https://www.eset.com/blog/en/)

[Español](https://www.welivesecurity.com/es/)[Deutsch](https://www.welivesecurity.com/de/)[Português](https://www.welivesecurity.com/pt/)[Français](https://www.welivesecurity.com/fr/)

https://www.welivesecurity.com/en/

Cybersecurity research and insight for the AI era · <a href='https://www.eset.com/blog/en/' target='_blank'>Moving soon to the ESET Blog</a>

ESET Research

# Operation Windigo – the vivisection of a large Linux server-side credential-stealing malware campaign

Our report titled “Operation Windigo – the vivisection of a large Linux server-side credential-stealing malware campaign" details our analysis of a set of malicious programs that infect servers and desktop PCs, and send nearly 500,000 web users to malicious content daily."

https://www.welivesecurity.com/en/our-experts/pierre-marc-bureau/

[Pierre-Marc Bureau](https://www.welivesecurity.com/en/our-experts/pierre-marc-bureau/)

18 Mar 2014  •  ,  2 min. read

A month ago, ESET published [a technical analysis on Linux/Ebury](https://www.welivesecurity.com/2014/02/21/an-in-depth-analysis-of-linuxebury/). This malware is a clever OpenSSH backdoor and credential stealer. Since last year, ESET’s research team has been investigating the operation behind Linux/Ebury. We discovered an infrastructure used for malicious activities that is all hosted on compromised servers. We were also able to find a link between different malicious components such as [Linux/Cdorked](https://www.welivesecurity.com/2013/05/07/linuxcdorked-malware-lighttpd-and-nginx-web-servers-also-affected/), [Perl/Calfbot](http://www.virusradar.com/en/Perl_Calfbot/detail) and [Win32/Glupteba.M](http://www.virusradar.com/en/Win32_Glupteba.M/description) and realized they are all operated by the same group.

Today, we are publishing the results of significant amounts of research effort in a report titled “[Operation Windigo – The vivisection of a large Linux server-side credential stealing malware campaign](https://web-assets.esetstatic.com/wls/2014/03/operation_windigo.pdf)”. This report details our analysis of a set of malicious programs that are used together to infect servers and desktop computers. We chose the name “Windigo” for its North American first nation roots and for its references to a malevolent half-beast.

The gang behind Operation Windigo uses infected systems to steal [credentials](https://www.welivesecurity.com/2014/02/21/an-in-depth-analysis-of-linuxebury/), [redirect web traffic](https://www.welivesecurity.com/2013/04/26/linuxcdorked-new-apache-backdoor-in-the-wild-serves-blackhole/) to malicious content, and send spam messages. According to our analysis, over 25,000 servers have been affected over the last two years. More than 10,000 of them are still infected today. These servers have all been compromised with the [Linux/Ebury](https://www.welivesecurity.com/2014/02/21/an-in-depth-analysis-of-linuxebury/) OpenSSH backdoor. This number is significant if you consider each of these systems have access to significant bandwidth, storage, computing power and memory. Well known organizations such as [cPanel](http://docs.cpanel.net/twiki/bin/view/AllDocumentation/CompSystem) and [kernel.org](http://www.linuxfoundation.org/news-media/blogs/browse/2011/08/cracking-kernelorg) were on the list of victims, although they have now cleaned their systems.

The infected servers are used to redirect half of a million web visitors to malicious content on a daily basis. Our research also shows that the attacker is able to send more than 35,000,000 spam messages per day with his current infrastructure. Operating systems affected by the spam component include Linux, FreeBSD, OpenBSD, OS X, and even Windows (with Perl running under Cygwin).

During the course of our analysis, we have had the opportunity to collaborate with various international organizations, including [CERT ‑Bund](https://www.bsi.bund.de/EN/Topics/IT-Crisis-Management/Cert-Bund/cert-bund_node.html), the [Swedish National Infrastructure for Computing](http://www.snic.vr.se/), the [European Organization for Nuclear Research](http://home.web.cern.ch/about) (CERN) and others forming an international Working Group. With the help of the working group, thousands of victims have been notified that their servers were infected, in an effort to clean as many systems as possible. We are now releasing a complete white paper in hopes of raising awareness around Operation Windigo and motivating administrators to clean up their compromised servers.

We have been working hard to prepare this report. First of all because the threats we have analyzed are complex and stealthy. Secondly, because we have accumulated massive amounts of data, ranging from traffic capture to malicious URLs and binaries. Lastly, because we wanted to provide extensive guidance to help system administrators and network operators determine if servers are compromised and what can be done about it. We hope you enjoy reading our report as much as we enjoyed putting it together.

- [Operation Windigo Whitepaper [PDF]](https://web-assets.esetstatic.com/wls/2014/03/operation_windigo.pdf)
- [Indicators of Compromise (IOCs)](https://github.com/eset/malware-ioc)

##  Let us keep you
up to date

Sign up for our newsletters

####  Related Articles

[ESET Research

ESET APT Activity Report Q2 2026–Q3 2026

ESET Research

ESET APT Activity Report Q2 2026–Q3 2026](https://www.welivesecurity.com/en/eset-research/eset-apt-activity-report-q2-2026-q3-2026/)

[ESET Research

MythicRatatoskr: An Yggdrasil-enabled backdoor deployed in a UAC-0099-Sandworm collaboration

ESET Research

MythicRatatoskr: An Yggdrasil-enabled backdoor deployed in a UAC-0099-Sandworm collaboration](https://www.welivesecurity.com/en/eset-research/mythicratatoskr-yggdrasil-enabled-backdoor-deployed-uac0099-sandworm-collaboration/)

[ESET Research

MATCHBOIL: New tricks, same old evil intentions

ESET Research

MATCHBOIL: New tricks, same old evil intentions](https://www.welivesecurity.com/en/eset-research/matchboil-new-tricks-same-old-evil-intentions/)

https://www.eset.com/us/business/services/threat-intelligence/

### Share Article

https://www.facebook.com/sharer/sharer.php?u=https://www.welivesecurity.com/2014/03/18/operation-windigo-the-vivisection-of-a-large-linux-server-side-credential-stealing-malware-campaign/ https://www.linkedin.com/shareArticle?mini=true&url=https://www.welivesecurity.com/2014/03/18/operation-windigo-the-vivisection-of-a-large-linux-server-side-credential-stealing-malware-campaign/ https://twitter.com/intent/tweet?url=https://www.welivesecurity.com/2014/03/18/operation-windigo-the-vivisection-of-a-large-linux-server-side-credential-stealing-malware-campaign/  https://www.welivesecurity.com/2014/03/18/operation-windigo-the-vivisection-of-a-large-linux-server-side-credential-stealing-malware-campaign/

### Discussion
