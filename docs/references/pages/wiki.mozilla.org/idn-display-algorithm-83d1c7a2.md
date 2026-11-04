---
url: "https://wiki.mozilla.org/IDN_Display_Algorithm"
title: "IDN Display Algorithm - MozillaWiki"
captured: "2026-11-02T18:49:24Z"
via: "live"
sha256: "7f7a490683ff0b601822bf3bda5de9dae0831bc7966b57cb6e7791f1c6705c3a"
wayback: "https://web.archive.org/web/20261005155402/https://wiki.mozilla.org/IDN_Display_Algorithm"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# IDN Display Algorithm - MozillaWiki

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://wiki.mozilla.org/IDN_Display_Algorithm>
> Snapshot: <https://web.archive.org/web/20261005155402/https://wiki.mozilla.org/IDN_Display_Algorithm>

# IDN Display Algorithm

From MozillaWiki

[Jump to navigation](https://wiki.mozilla.org/IDN_Display_Algorithm#mw-head) [Jump to search](https://wiki.mozilla.org/IDN_Display_Algorithm#searchInput)

This page explains how Firefox decides whether to display a given IDN label (a domain name is made up of one or more labels, separated by dots) in its Unicode (i.e. normal) or Punycode (i.e. gobbledigook) form.

Implementing this plan was covered by [bug 722299](https://bugzilla.mozilla.org/show_bug.cgi?id=722299).

## Contents

- [1 Background](https://wiki.mozilla.org/IDN_Display_Algorithm#Background)

- [1.1 The Problem](https://wiki.mozilla.org/IDN_Display_Algorithm#The_Problem)
- [1.2 Previous Algorithm](https://wiki.mozilla.org/IDN_Display_Algorithm#Previous_Algorithm)
- [1.3 Why We Changed](https://wiki.mozilla.org/IDN_Display_Algorithm#Why_We_Changed)

- [2 The New Idea](https://wiki.mozilla.org/IDN_Display_Algorithm#The_New_Idea)
- [3 Algorithm](https://wiki.mozilla.org/IDN_Display_Algorithm#Algorithm)

- [3.1 Possible Issues and Open Questions](https://wiki.mozilla.org/IDN_Display_Algorithm#Possible_Issues_and_Open_Questions)
- [3.2 Downsides](https://wiki.mozilla.org/IDN_Display_Algorithm#Downsides)

## Background

### The Problem

If we just display any possible IDN label, we open ourselves up to [IDN homograph attacks](http://en.wikipedia.org/wiki/IDN_homograph_attack), where one identical-looking domain can spoof another. So we have to have some mechanism to decide which ones to display and which ones to not display, which does not involve comparing the domain in question against every other single domain which exists (which is impossible).

### Previous Algorithm

We updated how Firefox dealt with this in 2012. Our previous algorithm was to display as Unicode all IDN labels within TLDs on our [whitelist](http://www.mozilla.org/projects/security/tld-idn-policy-list.html), and display as Punycode otherwise. We checked the anti-spoofing policies of a registry before adding their TLD to the whitelist. The TLD operator had to apply directly (they cannot be nominated by another person), and on several occasions we required policy updates or implementation as a condition of getting in.

We also had a character blacklist - characters we will never display under any circumstances. This includes those which could be used to spoof the separators "/" and ".", and invisible characters. This still exists.

### Why We Changed

The old strategy provided pretty good user protection, and it provided consistency - every Firefox everywhere works the same. However, it meant that IDNs did not work at all in many TLDs, because the registry (for whatever reason) had not applied for inclusion, or because we didn't think they had sufficiently strong protections in place. In addition, ICANN was about to open a [large number of new TLDs](http://newgtlds.icann.org/en/program-status/application-results/strings-1200utc-13jun12-en). So either maintaining a whitelist was going to become burdensome, or the list was going to become wildly out of date and we would not be serving our users.

## The New Idea

Instead, we now augment our whitelist with something based on ascertaining whether all the characters in a label all come from the same script, or are from one of a limited and defined number of allowable combinations. The hope is that any intra-script near-homographs will be recognisable to people who understand that script.

We retain the whitelist as well, because a) removing it might break some domains which worked previously, and b) if a registry submits a good policy, we have the ability to give them more freedom than the default restrictions do. So an IDN is shown as Unicode if the TLD was on the whitelist or, if not, if it met the criteria above.

## Algorithm

If a TLD is in the whitelist, we unconditionally display Unicode. If it is not, the following algorithm applies.

[Unicode Technical Report 39](http://www.unicode.org/reports/tr39/#Restriction_Level_Detection) defines a "Moderately Restrictive" profile. It says the following (with edits for clarity, after discussion with the author):

No characters in the label can be outside of the [Identifier Profile](http://www.unicode.org/reports/tr39/#Identifier_Characters) (defined for us by the IDNA2008 standard, [RFC 5892](http://tools.ietf.org/html/rfc5892)).

All characters in each label must be from Common + Inherited + a single script, or from one of the following combinations:

- Common + Inherited + Latin + Han + Hiragana + Katakana; or
- Common + Inherited + Latin + Han + Bopomofo; or
- Common + Inherited + Latin + Han + Hangul; or
- Common + Inherited + Latin + any single other "Recommended" or "Aspirational" script except Cyrillic or Greek

[Unicode Technical Report 39](http://www.unicode.org/reports/tr39/#Mixed_Script_Detection) gives a definition for how we detect whether a string is "single script".

Some Common or Inherited characters are only used in a small number (but more than one) script. Mark Davis writes: "The Unicode Consortium in U6.1 (due out soon) is adding the property [Script_Extensions](http://unicode.org/Public/6.1.0/ucd/ScriptExtensions.txt), to provide data about characters which are only used in a few (but more than one) script. The sample code in #39 should be updated to include that, so handling such cases." This data is now available, but not yet in the Firefox platform ([bug 844106](https://bugzilla.mozilla.org/show_bug.cgi?id=844106)). In the mean time, Common and Inherited characters are permitted without restriction.

We also implement additional checks, as suggested by TR #39 sections 5.3 and 5.4:

- Display as Punycode labels which use more than one numbering system
- Display as Punycode labels which contain both simplified-only and traditional-only Chinese characters -- [bug 857481](https://bugzilla.mozilla.org/show_bug.cgi?id=857481)
- Display as Punycode labels which have sequences of the same nonspacing mark

### Possible Issues and Open Questions

The following issues are still open, but were not considered important enough to block initial implementation.

Further suggestions from TR #39:

- Check to see that all the characters are in the sets of exemplar characters for at least one language in the Unicode Common Locale Data Repository. [XXX What does this mean? -- Gerv]

Also:

- Should we document our character hard-blacklist as part of this exercise? It's already visible in the prefs. Are any characters in it legal in IDNA2008 anyway? We should review it to make sure we aren't disallowing characters now allowed by IDNA 2008.
- Do we want to allow the user to choose between multiple "restriction levels", or have a hidden pref? There are significant downsides to allowing this...
- Do we ever want to display errors other than just by using Punycode? I suggest not...
- Should we add Armenian to the list of scripts which cannot mix with Latin?

### Downsides

This system permits whole-script confusables (All-Latin "scope.tld" vs all-Cyrillic "ѕсоре.tld"). However, so do the solutions of the other browsers, and it has not proved to be a significant problem so far. If there is a problem, every browser is equally affected.

Our response to this issue is that in the end, it is up to registries to make sure that their customers cannot rip each other off. Browsers can put some technical restrictions in place, but we are not in a position to do this job for them while still maintaining a level playing field for non-Latin scripts on the web. The registries are the only people in a position to implement the proper checking here. For our part, we want to make sure we don't treat non-Latin scripts as second-class citizens.

Retrieved from "https://wiki.mozilla.org/index.php?title=IDN_Display_Algorithm&oldid=1168502"

## Navigation menu

###  Search

https://wiki.mozilla.org/Main_Page
