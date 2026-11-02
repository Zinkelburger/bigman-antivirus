---
url: "https://forums.whonix.org/t/detecting-the-use-of-curl-bash-server-side/2361"
title: "Detecting the use of \"curl | bash\" server side"
site: "Whonix Forum"
published: "2016-04-22T15:22:26+00:00"
captured: "2026-11-02T18:31:25Z"
via: "live"
sha256: "57143f77e6cd2d91bf383c983ead0dccc49602be31969b675650fe338e64ef00"
wayback: "https://web.archive.org/web/20260219124226/https://forums.whonix.org/t/detecting-the-use-of-curl-bash-server-side/2361"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# Detecting the use of "curl | bash" server side

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://forums.whonix.org/t/detecting-the-use-of-curl-bash-server-side/2361>
> Snapshot: <https://web.archive.org/web/20260219124226/https://forums.whonix.org/t/detecting-the-use-of-curl-bash-server-side/2361>

[Whonix Wiki](https://www.whonix.org) [Download](https://www.whonix.org/wiki/Download) [Docs](https://www.whonix.org/wiki/Documentation) [News](https://forums.whonix.org/c/news) [Support](https://www.whonix.org/wiki/Support) [Tips](https://www.whonix.org/wiki/Forum_Best_Practices) [Issues](https://www.whonix.org/wiki/Reporting_Bugs#Issue_Tracker) [Contribute](https://www.whonix.org/wiki/Contribute) [DONATE](https://www.whonix.org/wiki/Donate)

[Whonix Forum](https://forums.whonix.org/)

#  [Detecting the use of "curl | bash" server side](https://forums.whonix.org/t/detecting-the-use-of-curl-bash-server-side/2361)

[Development](https://forums.whonix.org/c/development/8)

[HulaHoop](https://forums.whonix.org/u/HulaHoop)      April 22, 2016, 3:22pm   1

Remote servers can change their payload depending on whether they detect curl used standalone or if its outputs are piped into bash.

https://www.idontplaydarts.com/2016/04/detecting-curl-pipe-bash-server-side/

The final advice:

The better solution is never to pipe untrusted data streams into bash. If you still want to run untrusted bash scripts a better approach is to pipe the contents of URL into a file, review the contents on disk and only then execute it.

This doesn’t apply to anything Whonix does as files downloaded are always verified before being read/executed.

[Patrick](https://forums.whonix.org/u/Patrick)     April 22, 2016, 6:15pm   2

Good to advertise indeed.

HulaHoop:

Remote servers can change their payload depending on whether they detect curl used standalone or if its outputs are piped into bash.

Not very hard. From my varnish knowledge I’d know how to set up
something like this.

The final advice:

The better solution is never to pipe untrusted data streams into bash. If you still want to run untrusted bash scripts a better approach is to pipe the contents of URL into a file, review the contents on disk and only then execute it.

This doesn’t apply to anything Whonix does as files downloaded are always verified before being read/executed.

Yes.

[Patrick](https://forums.whonix.org/u/Patrick)     November 6, 2022, 10:48am   3

related:

By using this website, you acknowledge you have read, understood, and agree to be bound by these these agreements: [Terms of Service](https://www.whonix.org/wiki/Terms_of_Service), and [Privacy Policy](https://www.whonix.org/wiki/Privacy_Policy), [Cookie Policy](https://www.whonix.org/wiki/Cookie_Policy), [E-Sign Consent](https://www.whonix.org/wiki/E-Sign_Consent) [DMCA](https://www.whonix.org/wiki/DMCA) [Imprint](https://www.whonix.org/wiki/Imprint)
