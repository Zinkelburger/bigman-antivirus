---
url: "https://github.com/systemd/systemd/pull/28263"
title: "Disable LLMNR protocol in default builds by pemensik · Pull Request #28263 · systemd/systemd"
site: "GitHub"
captured: "2026-11-03T02:58:48Z"
via: "live"
sha256: "8eeb4fc78cd64c7d99a8600800855ca7ed51383768910d4734bed5856af32c5f"
wayback: "https://web.archive.org/web/20250830034207/https://github.com/systemd/systemd/pull/28263"
cited_in:
  - "docs/research/hardening.md"
---

# Disable LLMNR protocol in default builds by pemensik · Pull Request #28263 · systemd/systemd

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/systemd/systemd/pull/28263>
> Snapshot: <https://web.archive.org/web/20250830034207/https://github.com/systemd/systemd/pull/28263>

## Repository navigation

# Disable LLMNR protocol in default builds - #28263

Open

[pemensik](https://github.com/pemensik) wants to merge 1 commit into

[systemd:main](https://github.com/systemd/systemd/tree/main)from

[pemensik:llmnr-disable](https://github.com/pemensik/systemd/tree/llmnr-disable)

Open

##

[Disable LLMNR protocol in default builds](https://github.com/systemd/systemd/pull/28263#top)#28263

[pemensik](https://github.com/pemensik) wants to merge 1 commit into

[systemd:main](https://github.com/systemd/systemd/tree/main)from

[pemensik:llmnr-disable](https://github.com/pemensik/systemd/tree/llmnr-disable)

## Conversation

https://github.com/pemensik

###

[pemensik](https://github.com/pemensik)  commented [Jul 5, 2023](https://github.com/systemd/systemd/pull/28263#issue-1790206354)

Copy link

Copy Markdown

Contributor

Even Microsoft has disabled resolution over LLMNR protocol in their Windows 10. I have found that when testing something and wondered, why mDNS works but LLMNR does not on Windows machine in our network. Found mDNS is now preferred and updated systems get it disabled.

Found an article about that is by design, not conincidental: https://techcommunity.microsoft.com/t5/networking-blog/aligning-on-mdns-ramping-down-netbios-name-resolution-and-llmnr/ba-p/3290816

Because the way systemd-resolved is breaking some operations in DNS, I would like it disabled by default. Multicast DNS is preferred solution and has separate .local domain. It makes it easier to decide which queries should be directed to multicast resolution. Legacy users on Windows 7 should be forced to enable it, if they want to use it.

Related: [#23622](https://github.com/systemd/systemd/issues/23622)

https://github.com/pemensik

[Disable LLMNR protocol in default builds](https://github.com/systemd/systemd/pull/28263/commits/1fcb30098f98b11d22338b71894146bb9bb4f242)

[1fcb300](https://github.com/systemd/systemd/pull/28263/commits/1fcb30098f98b11d22338b71894146bb9bb4f242)

```
Even Microsoft has disabled resolution over LLMNR protocol in their
Windows 10. I have found that when testing something and wondered, why
mDNS works but LLMNR does not on Windows machine in our network. Found
mDNS is now preferred and updated systems get it disabled.

Found an article about that is by design, not conincidental:
https://techcommunity.microsoft.com/t5/networking-blog/aligning-on-mdns-ramping-down-netbios-name-resolution-and-llmnr/ba-p/3290816

Because the way systemd-resolved is breaking some operations in DNS, I
would like it disabled by default. Multicast DNS is preferred solution
and has separate .local domain. It makes it easier to decide which
queries should be directed to multicast resolution. Legacy users on
Windows 7 should be forced to enable it, if they want to use it.

Related: [systemd#23622](https://github.com/systemd/systemd/issues/23622)
```

https://github.com/apps/github-actions [github-actions](https://github.com/apps/github-actions) Bot added [meson](https://github.com/systemd/systemd/issues?q=state%3Aopen%20label%3Ameson) [please-review](https://github.com/systemd/systemd/issues?q=state%3Aopen%20label%3Aplease-review) PR is ready for (re-)review by a maintainer labels [Jul 5, 2023](https://github.com/systemd/systemd/pull/28263#event-9735875764)

https://github.com/pemensik [pemensik](https://github.com/pemensik) added [resolve](https://github.com/systemd/systemd/issues?q=state%3Aopen%20label%3Aresolve) and removed [meson](https://github.com/systemd/systemd/issues?q=state%3Aopen%20label%3Ameson) [please-review](https://github.com/systemd/systemd/issues?q=state%3Aopen%20label%3Aplease-review) PR is ready for (re-)review by a maintainer labels [Jul 5, 2023](https://github.com/systemd/systemd/pull/28263#event-9735877843)

https://github.com/poettering

###

[poettering](https://github.com/poettering)  commented [Jul 5, 2023](https://github.com/systemd/systemd/pull/28263#issuecomment-1622514320)

Copy link

Copy Markdown

Member

|

Has it actually been disabled in Windows 10? the way i read that document and the links they haven't done so yet, but are just planning to do so. is there any doc that makes clear they actually turned it off for good?

I am not sure we should hurry any change regarding this into v254. in particular as just switch this around might create problems to people who currently use it.

https://github.com/poettering [poettering](https://github.com/poettering) added the [needs-discussion 🤔](https://github.com/systemd/systemd/issues?q=state%3Aopen%20label%3A%22needs-discussion%20%F0%9F%A4%94%22) label [Jul 5, 2023](https://github.com/systemd/systemd/pull/28263#event-9736567154)

https://github.com/pemensik

###

[pemensik](https://github.com/pemensik)  commented [Jul 5, 2023](https://github.com/systemd/systemd/pull/28263#issuecomment-1622531603)

Copy link

Copy Markdown

Contributor   Author

|

Yes, I think we should hurry. I found that when trying to ping my father's pc running Windows 10, that it responded to <name>.local, but not anymore just <name>. I am quite sure I did not disable it on his PC nor he would know how to do that. That brings me to the only outcome, that it was disabled by some updates.

I have more stations to test it later, but I am quite sure the result will just confirm this.

Anyway, it was causing regressions to me and I am sure other people also. If they want that, let them express that by explicit line in resolved.conf. I am not saying it should not be possible to use it. I say people who never requested that should not get it enabled by default.

mDNS should be preferred for new deployments. It does not cause similar regressions.

https://github.com/pemensik

###

[pemensik](https://github.com/pemensik)  commented [Jul 5, 2023](https://github.com/systemd/systemd/pull/28263#issuecomment-1622538968)  •

edited

Loading

Copy link

Copy Markdown

Contributor   Author

|

I mean, that article was written on Apr 22 2022. They may have just planned it back then, but it seems they have deployed it already. I have not found exact reference, but I do not think that really matters.

https://github.com/bluca

###

[bluca](https://github.com/bluca)  commented [Jul 6, 2023](https://github.com/systemd/systemd/pull/28263#issuecomment-1624043658)

Copy link

Copy Markdown

Member

|

This is just the default of a build option, to do this experiment isn't it better to start from a distribution? IE change the Fedora build to disable it first, to shake the tree and see what falls off

https://github.com/pemensik [pemensik](https://github.com/pemensik) mentioned this pull request [Jul 6, 2023](https://github.com/systemd/systemd/pull/28263#ref-issue-1260409578)

[Handle LLMNR only by nss-resolve #23622](https://github.com/systemd/systemd/issues/23622)

Open

https://github.com/pemensik [pemensik](https://github.com/pemensik) added the [downstream/fedora](https://github.com/systemd/systemd/issues?q=state%3Aopen%20label%3Adownstream%2Ffedora) Tracking bugs for Fedora label [Aug 1, 2023](https://github.com/systemd/systemd/pull/28263#event-9976021475)

https://github.com/tojens-ietf

###

[tojens-ietf](https://github.com/tojens-ietf)  commented [Sep 23, 2023](https://github.com/systemd/systemd/pull/28263#issuecomment-1732157557)  •

edited

Loading

Copy link

Copy Markdown

|

Hey [@pemensik](https://github.com/pemensik) thank you for pinging me over on my blog post and I'm sorry I didn't see it for so long (I am the author of this post: https://techcommunity.microsoft.com/t5/networking-blog/aligning-on-mdns-ramping-down-netbios-name-resolution-and-llmnr/ba-p/3290816). I figured I would come over here to continue the discussion unless anyone has objections to that.

Hey everyone! I'm Tommy Jensen, a technical PM for the Windows DNS and HTTP native code stacks.

There is some unfortunate confusion around Windows 10 and Windows 11 because before Win11 was made public, Insider builds were called Windows 10 even though their features did not roll into Win10 by default. An example of this is another feature by our team: DNS over HTTPS (originally went to Win10 Insiders, but only shipped to Win11 in production).

All that to say that the changes you are describing apply to Win11, not Win10. That's not to say there weren't behavioral changes to other components that affected how name resolution occurred, but us placing NetBIOS NR into learning mode (it will only be called once mDNS times out) did not go into Win10, only Win11.

I cannot overstate how much I support everyone moving to mDNS. Having said that, I do respect the need to slowly ramp changes to avoid breaking in-market customers (hence we haven't entirely disabled LLMNR and NetBIOS NR yet even if we want to). How you time changes to systemd is your business. You can expect in the long run that the only multicast name resolution protocol Windows will use regularly is mDNS. Using multiple protocols for the same task just increases the possible attack surface of the machine, and it's clear that mDNS is the industry standard for this use case.

If you would like to engage with us privately so we can do things like exchange network traces, feel free to reach out to me at tojens@microsoft.com (though the weekend is starting where I live, so I'll see you next week). Side note: I also happen to be involved with encrypted DNS discovery in the IETF, so if you have any discussions about how that can work interoperably with Windows or the ecosystem at large, you can ping me for that too! We're passionate about seeing modern DNS protocols gain widespread adoption.

https://github.com/bluca

###

[bluca](https://github.com/bluca)  commented [Sep 24, 2023](https://github.com/systemd/systemd/pull/28263#issuecomment-1732545896)

Copy link

Copy Markdown

Member

|

[@keszybz](https://github.com/keszybz) could this be done in Fedora first given the next release is upcoming? Then we can get some real-world testing

https://github.com/pemensik

###

[pemensik](https://github.com/pemensik)  commented [Nov 7, 2023](https://github.com/systemd/systemd/pull/28263#issuecomment-1798270288)  •

edited

Loading

Copy link

Copy Markdown

Contributor   Author

|

I have found later Windows desktops differentiate llmnr responder status based on network connection trust setting. If the Win10 box has public setting, which is I believe the default, it does not enable LLMNR responder on such connection. I believe that is how I got confused. You have to switch the network to trusted or home in order to have local resolution working. We lack that security default on Fedora, it just enables it always and even on servers.

At least firewall settings prevent it in public mode.

https://github.com/pemensik [pemensik](https://github.com/pemensik) mentioned this pull request [Mar 6, 2024](https://github.com/systemd/systemd/pull/28263#ref-issue-1776119403)

[resolve queries to the reverse mapping domains (.in-addr.arpa, ip6.arpa) are slow on negative response #28166](https://github.com/systemd/systemd/issues/28166)

Closed

https://github.com/rpigott

###

[rpigott](https://github.com/rpigott)  commented [Mar 19, 2024](https://github.com/systemd/systemd/pull/28263#issuecomment-2008289894)

Copy link

Copy Markdown

Contributor

|

I think it makes sense to disable llmnr by default, but I think we should first allow sending single-label names to mdns as host.local with a config toggle or with local in the search list.

https://github.com/tojens-ietf

###

[tojens-ietf](https://github.com/tojens-ietf)  commented [Mar 20, 2024](https://github.com/systemd/systemd/pull/28263#issuecomment-2008694798)

Copy link

Copy Markdown

|

FWIW we are taking an approach in Windows where we first try mDNS, and then only try other protocols once mDNS has failed. This seemed like a safe way to reduce usage without requiring manual action from users/admins (and the few who rely on it can of course change the behavior back).

https://github.com/pemensik

###

[pemensik](https://github.com/pemensik)  commented [Mar 20, 2024](https://github.com/systemd/systemd/pull/28263#issuecomment-2009201890)

Copy link

Copy Markdown

Contributor   Author

|

FWIW we are taking an approach in Windows where we first try mDNS, and then only try other protocols once mDNS has failed. This seemed like a safe way to reduce usage without requiring manual action from users/admins (and the few who rely on it can of course change the behavior back).

Isn't the problem with this approach that mdns query to non-existent host adds significant visible timeout, before it can be considered failed? It seems to me mdns before others adds possible vector of attack, just like llmnr this way does. Except mDNS has own domain, so it is more clear when mDNS lookup should be attempted at all. I have seen again last week Active Directory example with domain.local, which seems MS legacy.

If single-label mDNS fallback needs to be explicitly enabled, then okay. But please do not give it higher priority than unicast DNS to local servers.

https://github.com/tojens-ietf

###

[tojens-ietf](https://github.com/tojens-ietf)  commented [Mar 23, 2024](https://github.com/systemd/systemd/pull/28263#issuecomment-2016370784)

Copy link

Copy Markdown

|

Sorry, flying back from the IETF and catching up

If single-label mDNS fallback needs to be explicitly enabled, then okay. But please do not give it higher priority than unicast DNS to local servers.

What I was trying to say was only applicable to when we would have done LLMNR+mDNS+NBNR, now we will do mDNS->wait->then-LLMNR-NBNR, not change when we do unicast versus multicast name queries. I believe we did not change the logic for that decision... but I will double check and get back to you once I'm back to work.

https://github.com/rpigott

###

[rpigott](https://github.com/rpigott)  commented [Nov 19, 2024](https://github.com/systemd/systemd/pull/28263#issuecomment-2486626786)

Copy link

Copy Markdown

Contributor

|

[@poettering](https://github.com/poettering) I think we should consider disabling LLMNR by default in v257 or v258.

I am reminded of this by [linux-pam/linux-pam#834](https://github.com/linux-pam/linux-pam/issues/834).

https://github.com/Alphix

###

[Alphix](https://github.com/Alphix)  commented [Jan 17, 2025](https://github.com/systemd/systemd/pull/28263#issuecomment-2598787552)

Copy link

Copy Markdown

Contributor

|

[@poettering](https://github.com/poettering) I think we should consider disabling LLMNR by default in v257 or v258.

I agree. I was recently bit by this because LLMNR seems to race with normal DNS-based PTR resolution, meaning that in a network where there are proper PTR records in the DNS servers, systemd-resolved will randomly return either the LLMNR record (host) or the DNS-based record (host.example.com), meaning that things like pam_access rules (with domain-based matches) or Kerberos (which likes to do reverse lookups) will fail randomly...

https://github.com/alexvojproc [alexvojproc](https://github.com/alexvojproc) mentioned this pull request [Aug 18, 2025](https://github.com/systemd/systemd/pull/28263#ref-pullrequest-3332314468)

[feat(resolved): disable LLMNR resolution by default secureblue/secureblue#1267](https://github.com/secureblue/secureblue/pull/1267)

Merged

https://github.com/sedlund [sedlund](https://github.com/sedlund) mentioned this pull request [Sep 18, 2025](https://github.com/systemd/systemd/pull/28263#ref-pullrequest-3431282152)

[feat(resolved): disable llmnr on server profile nix-community/srvos#697](https://github.com/nix-community/srvos/pull/697)

Merged

https://github.com/agners [agners](https://github.com/agners) mentioned this pull request [Oct 6, 2025](https://github.com/systemd/systemd/pull/28263#ref-issue-2672702358)

[Default mDNS settings results in network flooding in networks with reflectors enabled home-assistant/supervisor#5435](https://github.com/home-assistant/supervisor/issues/5435)

Closed

### This branch has not been deployed

No deployments

[Sign up for free](https://github.com/join?source=comment-repo) to join this conversation on GitHub. Already have an account? [Sign in to comment](https://github.com/login?return_to=https%3A%2F%2Fgithub.com%2Fsystemd%2Fsystemd%2Fpull%2F28263)

###  Labels

[downstream/fedora](https://github.com/systemd/systemd/issues?q=state%3Aopen%20label%3Adownstream%2Ffedora) Tracking bugs for Fedora [needs-discussion 🤔](https://github.com/systemd/systemd/issues?q=state%3Aopen%20label%3A%22needs-discussion%20%F0%9F%A4%94%22) [resolve](https://github.com/systemd/systemd/issues?q=state%3Aopen%20label%3Aresolve)

###  6 participants

https://github.com/pemensik https://github.com/poettering https://github.com/bluca https://github.com/tojens-ietf https://github.com/rpigott https://github.com/Alphix
