---
url: "https://github.blog/engineering/platform-security/behind-githubs-new-authentication-token-formats/"
title: "Behind GitHub's new authentication token formats"
site: "The GitHub Blog"
published: "2021-04-05T16:31:34+00:00"
captured: "2026-11-02T18:33:43Z"
via: "live"
sha256: "823134ce962c3f3bcc3e43771c5420b7a18cf8472aeecc57b7e5a9257cb9ac74"
wayback: "https://web.archive.org/web/20261019200510/https://github.blog/engineering/platform-security/behind-githubs-new-authentication-token-formats/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Behind GitHub's new authentication token formats

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.blog/engineering/platform-security/behind-githubs-new-authentication-token-formats/>
> Snapshot: <https://web.archive.org/web/20261019200510/https://github.blog/engineering/platform-security/behind-githubs-new-authentication-token-formats/>

[Home](https://github.blog/) / [Engineering](https://github.blog/engineering/) / [Platform security](https://github.blog/engineering/platform-security/)

# Behind GitHub’s new authentication token formats

We’re excited to share a deep dive into how our new authentication token formats are built and how these improvements are keeping your tokens more secure. As we continue to…

[Indigo K](https://github.blog/author/indigok/)·[@indigok](https://github.com/indigok)

April 5, 2021  |  Updated May 10, 2023

|  4 minutes

-   Share:
-  https://x.com/share?text=Behind%20GitHub%26%238217%3Bs%20new%20authentication%20token%20formats&url=https%3A%2F%2Fgithub.blog%2Fengineering%2Fplatform-security%2Fbehind-githubs-new-authentication-token-formats%2F
-  https://www.facebook.com/sharer/sharer.php?t=Behind%20GitHub%26%238217%3Bs%20new%20authentication%20token%20formats&u=https%3A%2F%2Fgithub.blog%2Fengineering%2Fplatform-security%2Fbehind-githubs-new-authentication-token-formats%2F
-  https://www.linkedin.com/shareArticle?title=Behind%20GitHub%26%238217%3Bs%20new%20authentication%20token%20formats&url=https%3A%2F%2Fgithub.blog%2Fengineering%2Fplatform-security%2Fbehind-githubs-new-authentication-token-formats%2F

We’re excited to share a deep dive into how our [new authentication token formats](https://github.blog/changelog/2021-03-31-authentication-token-format-updates-are-generally-available/) are built and how these improvements are keeping your tokens more secure. As we continue to focus on the security of our platform and services across the web, this update shows how big an impact simple changes can have.

Many of our old authentication token formats are hex-encoded 40 character strings that are indistinguishable from other encoded data like SHA hashes. These have several limitations, such as inefficient or even inaccurate detection of compromised tokens for our [secret scanning](https://docs.github.com/en/github/administering-a-repository/about-secret-scanning) feature. We continually strive for security excellence, so we knew that token detection was something we wanted to improve. How could we make our tokens easier to identify and more secure?

Without further ado, here are the design decisions behind our new authentication token formats that let us meet both goals.

### [Identifiable prefixes](https://github.blog/engineering/platform-security/behind-githubs-new-authentication-token-formats/#identifiable-prefixes)

As we see across the industry from companies like [Slack](https://api.slack.com/authentication/token-types) and [Stripe](https://stripe.com/docs/api/authentication), token prefixes are a clear way to make tokens identifiable. We are including specific 3 letter prefixes to represent each token, starting with a company signifier, gh, and the first letter of the token type. The results are:

- ghp for [GitHub personal access tokens](https://docs.github.com/en/github/authenticating-to-github/creating-a-personal-access-token)
- gho for [OAuth access tokens](https://docs.github.com/en/developers/apps/authorizing-oauth-apps)
- ghu for [GitHub user-to-server tokens](https://docs.github.com/en/developers/apps/identifying-and-authorizing-users-for-github-apps)
- ghs for [GitHub server-to-server tokens](https://docs.github.com/en/developers/apps/authenticating-with-github-apps#authenticating-as-an-installation)
- ghr for [refresh tokens](https://docs.github.com/en/developers/apps/refreshing-user-to-server-access-tokens)

Additionally, we want to make these prefixes clearly distinguishable within the token to improve readability. Thus, we are adding a separator: _. An underscore is not a Base64 character which helps ensure that our tokens cannot be accidentally duplicated by randomly generated strings like SHAs.

One other neat thing about _ is it will reliably select the whole token when you double click on it. Other characters we considered are sometimes included in application word separators and thus will stop highlighting at that character. Try out double clicking this-random-text versus this_random_text!

With this prefix alone, we anticipate the false positive rate for secret scanning will be down to 0.5%.⚡

### [Checksum](https://github.blog/engineering/platform-security/behind-githubs-new-authentication-token-formats/#checksum)

Identifiable prefixes are great, but let’s go one step further. A checksum virtually eliminates false positives for secret scanning offline. We can check the token input matches the checksum and eliminate fake tokens without having to hit our database.

A 32 bit checksum in the last 6 digits of each token strikes the optimal balance between keeping the random token portion at a consistent entropy and enough confidence in the checksum. We start the implementation with a CRC32 algorithm, a standard checksum algorithm. We then encode the result with a Base62 implementation, using leading zeros for padding as needed.

### [Token entropy](https://github.blog/engineering/platform-security/behind-githubs-new-authentication-token-formats/#token-entropy)

We of course can’t forget about token entropy. Entropy is a logarithmic measure of information or uncertainty inherent in the possible token combinations. We use it as a representation of uniqueness for a given pattern and it’s important to maintain for the vast number of tokens we generate everyday. For personal access tokens alone, we create over 10k on a slow day and upwards of 18k on peak days. With our new formats, not only did we maintain our previous levels — we increased them!

Previously, our implementation for OAuth access tokens had an entropy of 160:

```
Math.log(((“a”..“f”).to_a + (0..9).to_a).length)/Math.log(2) * 40 = 160

```

Our implementation for OAuth access tokens are now 178:

```
Math.log(((“a”..“z”).to_a + (“A”..“Z”).to_a + (0..9).to_a).length)/Math.log(2) * 30 = 178

```

As we continue to grow and move forward, we will increase this entropy even more. But for now, we are thrilled our tokens have increased identifiability, security, and entropy — all without changing the token length.

### [What does this mean for you?](https://github.blog/engineering/platform-security/behind-githubs-new-authentication-token-formats/#what-does-this-mean-for-you)

As a GitHub user…

We strongly encourage you to reset any personal access tokens and OAuth tokens you have. These improvements help secret scanning detection and will help you mitigate any risk to compromised tokens. You can reset your personal access tokens by going to [developer settings](https://github.com/settings/tokens) and your OAuth tokens with [our API](https://docs.github.com/en/rest/reference/apps#reset-a-token).

As a service provider…

If you issue tokens as part of your platform and aren’t part of our secret scanning feature, we encourage you to follow the guidelines we outline here for your own tokens and join our [secret scanning program](https://docs.github.com/en/developers/overview/secret-scanning) so we can keep your tokens secure too.

We thank you for helping us make our platform and services the best and most secure they can be.✨

## Tags:

-  [authentication](https://github.blog/tag/authentication/)
-  [token authentication](https://github.blog/tag/token-authentication/)

##  Written by

###  [Indigo K](https://github.blog/author/indigok/)

[@indigok](https://github.com/indigok)

##  Related posts

[Architecture & optimization](https://github.blog/engineering/architecture-optimization/)

###  [The bridge to agent-scale Git infrastructure](https://github.blog/engineering/architecture-optimization/the-bridge-to-agent-scale-git-infrastructure/)

We moved a copy of 99.99999% of GitHub’s repositories to Azure. Here’s how per-reference quorums will bring over the rest, and how object storage lets reads scale independently.

[Engineering](https://github.blog/engineering/)

###  [Staying Evergreen: How we automated secure access and repo hygiene in a shared org](https://github.blog/engineering/platform-security/staying-evergreen-how-we-automated-secure-access-and-repo-hygiene-in-a-shared-org/)

Put native GitHub features to work to review outside collaborator access and archive stale repositories.

[Architecture & optimization](https://github.blog/engineering/architecture-optimization/)

###  [From 2,500 requests to 24: Rebuilding star history without the stargazers](https://github.blog/engineering/architecture-optimization/from-2500-requests-to-24-rebuilding-star-history-without-the-stargazers/)

Our new repo star history API now provides trends without returning individual identities or requiring unbounded database work. These are the engineering tradeoffs we made.

## We do newsletters, too

Discover tips, technical guides, and best practices in our biweekly newsletter just for devs.

Your email address
