---
url: "https://developer.chrome.com/blog/remote-debugging-port"
title: "Changes to remote debugging switches to improve security | Blog | Chrome for Developers"
site: "Chrome for Developers"
captured: "2026-11-02T18:26:31Z"
via: "live"
sha256: "64a2d1ab10d02c6e9f578296c3ee7eeea53874891191a9a5cfdfb854dacd75a3"
wayback: "https://web.archive.org/web/20261102194327/https://developer.chrome.com/blog/remote-debugging-port"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Changes to remote debugging switches to improve security | Blog | Chrome for Developers

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://developer.chrome.com/blog/remote-debugging-port>
> Snapshot: <https://web.archive.org/web/20261102194327/https://developer.chrome.com/blog/remote-debugging-port>

-  [Chrome for Developers](https://developer.chrome.com/)
-   [Blog](https://developer.chrome.com/blog)

#  Changes to remote debugging switches to improve security

Will Harris

https://twitter.com/parityzero

Published: March 17, 2025

Last year, we announced improved security for cookies and credentials with [App-Bound Encryption for Chrome](https://security.googleblog.com/2024/07/improving-security-of-chrome-cookies-on.html).

While we observed that this new protection resulted in a substantial reduction in this type of account hijacking, attackers remain very motivated to adapt to these new defenses and we continue to monitor the techniques and behaviors used by infostealers.

Since App-Bound Encryption was enabled we've seen an increase in attackers using Chrome Remote Debugging to extract cookies. Using [this method to steal Chrome cookies](https://mango.pdf.zone/stealing-chrome-cookies-without-a-password/) has been discussed since 2018. However, recent [blog posts covering this use of the remote debugging port](https://embracethered.com/blog/posts/2024/cookie-theft-in-2024-and-what-todo/), and [tools to extract cookies](https://github.com/defaultnamehere/cookie_crimes) support our internal data showing its increased popularity. We remain committed to disrupting these techniques, and constraining attackers further—increasing the cost of these attacks and protecting the security of Chrome users.

Therefore, from Chrome 136 we're making changes to the behavior of --remote-debugging-port and --remote-debugging-pipe. These switches will no longer be respected if attempting to debug the default Chrome data directory. These switches must now be accompanied by the --user-data-dir switch to point to a non-standard directory. A non-standard data directory uses a different encryption key meaning Chrome's data is now protected from attackers.

For developers who need to debug Chrome (for example, from VSCode), the instructions [already specify](https://code.visualstudio.com/docs/nodejs/browser-debugging#_attaching-to-browsers) use of a custom user data dir and we continue to recommend this approach to isolate any debugging from any real profiles.

For browser automation scenarios, we recommend using [Chrome for Testing](https://developer.chrome.com/blog/chrome-for-testing) which will continue to respect the existing behavior.

Please file any issues with this new security feature to [crbug.com](https://crbug.com).
