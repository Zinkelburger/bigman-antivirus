---
url: "https://developers.google.com/safe-browsing/v4/usage-limits"
title: "Usage Restrictions | Safe Browsing APIs (v4) | Google for Developers"
site: "Google for Developers"
captured: "2026-11-02T18:27:42Z"
via: "live"
sha256: "5c45fd91d02927ce8eeee3366983f7f324a537c7147a17425ca70004a2becbb0"
wayback: "https://web.archive.org/web/20261102194434/https://developers.google.com/safe-browsing/v4/usage-limits"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# Usage Restrictions | Safe Browsing APIs (v4) | Google for Developers

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://developers.google.com/safe-browsing/v4/usage-limits>
> Snapshot: <https://web.archive.org/web/20261102194434/https://developers.google.com/safe-browsing/v4/usage-limits>

-  [Home](https://developers.google.com/)
-   [Products](https://developers.google.com/products)
-   [Google Safe Browsing](https://developers.google.com/safe-browsing)
-   [Safe Browsing APIs (v4)](https://developers.google.com/safe-browsing/v4)

#  Usage Restrictions

## Page Summary

-

The Safe Browsing API is for non-commercial use only; commercial users should refer to Web Risk.

-

Developers receive a default usage quota, which can be increased upon request through the Google Developer Console.

-

Warnings to users about potentially risky websites must use qualifying language and include attribution to Google with a link to the Safe Browsing Advisory.

-

Warnings should follow the suggested language provided or be slightly modified to fit the product's context.

-

User protection notices are required to inform users that the protection offered is not perfect and may have false positives or negatives.

## Permitted use

The Safe Browsing API is for non-commercial use only (meaning “not for sale or revenue generating purposes”). If you need a solution for commercial purposes, please refer to [Web Risk](https://cloud.google.com/web-risk). For further guidance, please visit the [Google Safe Browsing API](https://groups.google.com/forum/#!forum/google-safe-browsing-api) community forum for answers to common questions.

## Usage Quota

Developers are allocated a default usage quota upon enabling the Safe Browsing API. Current allocation and usage can be viewed in the [Google Developer Console](https://console.developers.google.com/apis/api/safebrowsing.googleapis.com/quotas). If you expect to use more than your currently allocated quota, you may request additional quota from the [Developer Console’s Quota interface](https://console.developers.google.com/apis/api/safebrowsing.googleapis.com/quotas). We review these requests and require a contact when applying for an increased quota to ensure that our service availability meets the needs of all API users. There is no cost for use of this API. For more information about large non-commercial deployments, please apply for additional quota using the link above and describe your application needs or find answers to common questions in the [Google Safe Browsing API](https://groups.google.com/forum/#!forum/google-safe-browsing-api) community forum.

## User warnings

If you use the Safe Browsing APIs (v4) to warn users about risks from particular webpages, the following guidelines apply.

These guidelines help protect both you and Google from misunderstandings by making clear that the page is not known with 100% certainty to be an unsafe web resource, and that the warnings merely identify possible risk.

-  In your user visible warning, you must not lead users to believe that the page in question is, without a doubt, an unsafe web resource. When you refer to the page being identified or the potential risks it may pose to users, you must qualify the warning using terms such as: suspected, potentially, possible, likely, may be.
-  Your warning must enable the user to learn more by reviewing information at [www.antiphishing.org](https://www.google.com/url?q=http://www.antiphishing.org&usg=AFrqEzdc43cbWnGpYdvxqO6R0Xvs3hrtSQ) for social engineering (phishing and deceptive sites) warnings, [Google Search Central](https://www.google.com/url?q=https://developers.google.com/search/docs/advanced/security/malware) for malware warnings, and [Unwanted Software Policy](http://www.google.com/about/company/unwanted-software-policy.html) for unwanted software warnings.
-  When you show warnings for pages identified as risky by the Safe Browsing Service, you must give attribution to Google by including the line "Advisory provided by Google," with a link to the [Safe Browsing Advisory](https://developers.google.com/safe-browsing/v4/advisory). If your product also shows warnings based on other sources, you must not include the Google attribution in warnings derived from non-Google data.

### Suggested warning language

We encourage you to just copy this warning language in your product, or to modify it slightly to fit your product (see [Metadata](https://developers.google.com/safe-browsing/v4/metadata) for additional information that may apply).

For social engineering (phishing and deceptive sites) warnings:

Warning—Deceptive site ahead. Attackers on [site URL] may trick you into doing something dangerous like installing software or revealing your personal information (for example, passwords, phone numbers, or credit cards). You can find out more about social engineering (phishing) at [Social Engineering (Phishing and Deceptive Sites)](https://support.google.com/webmasters/answer/6350487) or from [www.antiphishing.org](https://www.google.com/url?q=http://www.antiphishing.org&usg=AFrqEzdc43cbWnGpYdvxqO6R0Xvs3hrtSQ).

For malware warnings:

Warning—Visiting this web site may harm your computer. This page appears to contain malicious code that could be downloaded to your computer without your consent. You can learn more about harmful web content including viruses and other malicious code and how to protect your computer at [Google Search Central](https://www.google.com/url?q=https://developers.google.com/search/docs/advanced/security/malware).

For unwanted software warnings:

Warning—The site ahead may contain harmful programs. Attackers might attempt to trick you into installing programs that harm your browsing experience (for example, by changing your homepage or showing extra ads on sites you visit). You can learn more about unwanted software at [Unwanted Software Policy](http://www.google.com/about/company/unwanted-software-policy.html).

For Android potentially harmful application (PHA) warnings:

Warning—The site ahead may contain malware. Attackers might attempt to install dangerous apps on your device that steal or delete your information (for example, photos, passwords, messages, and credit cards).

### Age of data and usage

When retrieving data using the Safe Browsing APIs (v4) clients must never use data older than what is specified by the service. Specifically, a warning can only be shown if the following is true:

- Lookup API (v4): In the [threatMatches.find HTTP POST response](https://developers.google.com/safe-browsing/v4/lookup-api#http-post-response) a URL matches a threat entry and the cached response is still valid at the time a warning is shown.
- Update API (v4): In the [fullHashes.find HTTP POST response](https://developers.google.com/safe-browsing/v4/update-api#http-post-response_2) a URL matches a full-length hash and the cached response is still valid at the time a warning is shown.

Important: Under no other circumstances may a warning be shown.

## User protection notice

Our Terms of Service require that if you indicate to users that your service provides protection against unsafe web resources then you must also let them know that the protection is not perfect. This notice must be visible to them before they enable the protection, and it must let them know that there is a chance of both false positives (safe sites flagged as risky) and false negatives (risky sites not flagged). We suggest using the following language:

Google works to provide the most accurate and up-to-date information about unsafe web resources. However, Google cannot guarantee that its information is comprehensive and error-free: some risky sites may not be identified, and some safe sites may be identified in error.
