---
url: "https://www.xudongz.com/blog/2017/idn-phishing/"
title: "Phishing with Unicode Domains - Xudong Zheng"
captured: "2026-11-02T19:02:23Z"
via: "live"
sha256: "46c7fa41655544b236e126f30cc580764186327cdd15d5079494c859d26502ec"
wayback: "https://web.archive.org/web/20261021004320/https://www.xudongz.com/blog/2017/idn-phishing/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# Phishing with Unicode Domains - Xudong Zheng

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.xudongz.com/blog/2017/idn-phishing/>
> Snapshot: <https://web.archive.org/web/20261021004320/https://www.xudongz.com/blog/2017/idn-phishing/>

## Phishing with Unicode Domains

Posted by [Xudong Zheng](https://www.xudongz.com/) on April 14, 2017

The site above is not the real Apple domain but rather a domain that I was able to purchase. You can see for yourself by visiting the [proof-of-concept](https://www.xn--80ak6aa92e.com/). The security issue has been fixed as of Chrome 59 though it still exists in all versions of Firefox.

[Punycode](https://en.wikipedia.org/wiki/Punycode) makes it possible to register domains with foreign characters. It works by converting individual domain label to an alternative format using only ASCII characters. For example, the domain "xn–s7y.co" is equivalent to "短.co".

From a security perspective, Unicode domains can be problematic because many Unicode characters are difficult to distinguish from common ASCII characters. It is possible to register domains such as "xn–pple-43d.com", which is equivalent to "аpple.com". It may not be obvious at first glance, but "аpple.com" uses the Cyrillic "а" (U+0430) rather than the ASCII "a" (U+0061). This is known as a [homograph attack](https://en.wikipedia.org/wiki/IDN_homograph_attack).

Fortunately modern browsers have mechanisms in place to limit IDN homograph attacks. The page [IDN in Google Chrome](https://www.chromium.org/developers/design-documents/idn-in-google-chrome) highlights the conditions under which an IDN is displayed in its native Unicode form. Generally speaking, the Unicode form will be hidden if a domain label contains characters from multiple different languages. The "аpple.com" domain as described above will appear in its Punycode form as "xn–pple-43d.com" to limit confusion with the real "apple.com".

The homograph protection mechanism in Chrome, Firefox, and Opera unfortunately fails if every characters is replaced with a similar character from a single foreign language. The domain "аррӏе.com", registered as "xn–80ak6aa92e.com", bypasses the filter by only using Cyrillic characters. You can check this out yourself in the [proof-of-concept](https://www.xn--80ak6aa92e.com/) using Chrome, Firefox, or Opera.

Visually, the two domains are indistinguishable due to the font used by Chrome and Firefox. As a result, it becomes impossible to identify the site as fraudulent without carefully inspecting the site's URL or SSL certificate. [This Go program](https://play.golang.org/p/BzJVWN78pA) nicely demonstrates the difference between the two sets of characters. Safari, along with several less mainstream browsers are fortunately not vulnerable.

Internet Explorer does not display native characters in domains unless it belongs to one of the computer's system languages. As a result, it suffers from the same vulnerability if the system has Russian (and other Cyrillic languages) enabled. Internet Explorer's [documentation](https://msdn.microsoft.com/en-us/library/dd565654(v=vs.85).aspx) acknowledges that users are "increasing the risk of spoofing attack" when their system supports additional languages.

This bug was [reported to Chrome](https://bugs.chromium.org/p/chromium/issues/detail?id=683314) and Firefox on January 20, 2017 and was fixed in the Chrome trunk on March 24. The fix is included in Chrome 58 which is currently rolling out to users. The existence of the bug in Opera was brought to my attention only after the initial publication of this post. The problem remains in Firefox as they decided that it is a problem for domain registrars to deal with. You can find the detailed discussion in [the Bugzilla issue](https://bugzilla.mozilla.org/show_bug.cgi?id=1332714).

Our IDN threat model specifically excludes whole-script homographs, because they can't be detected programmatically and our "TLD whitelist" approach didn't scale in the face of a large number of new TLDs. If you are buying a domain in a registry which does not have proper anti-spoofing protections (like .com), it is sadly the responsibility of domain owners to check for whole-script homographs and register them.

Firefox users can limit their exposure by going to about:config and setting network.IDN_show_punycode to true. This will force Firefox to always display IDN domains in its Punycode form, making it possible to identify malicious domains. Thanks to /u/MARKZILLA from reddit for this temporary solution. Chrome 58+ users and Firefox users who apply this fix will see the Punycode domain rather than "apple.com".

A simple way to limit the damage from bugs such as this is to always use a password manager. In general, users must be very careful and pay attention to the URL when entering personal information. Until this is fixed, concerned users should manually type the URL or navigate to sites via a search engine when in doubt. This is a serious vulnerability that can even fool those who are extremely mindful of phishing.

Screenshots

https://www.xudongz.com/static/de4a0e040a8586ab588b057db46666f4248a4154aac21e2315e46103b97c422b.png https://www.xudongz.com/static/3cf366bffb5a77655fb30a9e05cfb80dd372b14488d613a9174470633088fc96.png https://www.xudongz.com/static/d7a1d775e249db201f237962ee35413a86785617e4ddf35dd762902cf4fec8c6.png https://www.xudongz.com/static/bb6c78555178beb820b0a42aa1a6ff72c4fcbeff386cfe97e9acd7ca5e97eaed.png https://www.xudongz.com/static/62ee050834aa1e1c1c229fd193aa6e0b0d2973e6115decc530cc04eb4f42a722.png

Covered by: [9to5mac (2017)](https://9to5mac.com/2017/04/20/how-to-spot-a-phishing-attempt-fake-apple-site/), [9to5mac (2022)](https://9to5mac.com/2022/05/25/fake-apple-com-website/), [Arstechnica](https://arstechnica.com/security/2017/04/chrome-firefox-and-opera-users-beware-this-isnt-the-apple-com-you-want/), [Engadget](https://www.engadget.com/2017/04/17/google-chrome-phishing-unicode-flaw/), [Forbes](https://www.forbes.com/sites/leemathews/2017/04/21/this-apple-phishing-site-is-as-sneaky-as-they-come/), [Fortune](http://fortune.com/2017/04/18/google-chrome-phishing-scam/), [The Guardian](https://www.theguardian.com/technology/2017/apr/19/phishing-url-trick-hackers), [IBTimes](http://www.ibtimes.com/browser-security-google-will-fix-phishing-exploit-chrome-already-patched-safari-edge-2526427), [Inc.](https://www.inc.com/minda-zetlin/devilishly-clever-scam-sites-could-fool-even-the-most-cautious-into-revealing-th.html), [The Independent](http://www.independent.co.uk/life-style/gadgets-and-tech/news/google-chrome-scam-websites-url-non-latin-characters-undetectable-a7695886.html), [Mashable](http://mashable.com/2017/04/19/phishing-homograph-attack-identical-letters/), [PC Magazine](http://www.pcmag.com/news/353205/chrome-blocks-crafty-url-phishing-method), [The Register](https://www.theregister.co.uk/2017/04/18/homograph_attack_again/), [Schneier on Security](https://www.schneier.com/blog/archives/2017/04/faking_domain_n.html), [Softpedia](http://news.softpedia.com/news/chrome-firefox-preparing-fixes-for-nasty-phishing-trick-using-punycode-515005.shtml), [TechCrunch](https://techcrunch.com/2017/04/20/this-phishing-technique-is-more-interesting-than-dangerous/), [TroyHunt](https://www.troyhunt.com/the-decreasing-usefulness-of-positive-visual-security-indicators-and-the-importance-of-negative-ones/), [Wired](https://www.wired.com/2017/04/sneaky-exploit-allows-phishing-attacks-sites-look-secure/)

Discussed on: [Hacker News](https://news.ycombinator.com/item?id=14130241), [/r/programming](https://www.reddit.com/r/programming/comments/65cf1u/phishing_with_unicode_domains/), [/r/netsec](https://www.reddit.com/r/netsec/comments/65csdk/phishing_with_unicode_domains/), [/r/android](https://www.reddit.com/r/Android/comments/671mub/this_spoof_apple_site_illustrates_the/), [/r/webdev](https://www.reddit.com/r/webdev/comments/66mksc/chrome_firefox_and_opera_users_beware_this_isnt/)
