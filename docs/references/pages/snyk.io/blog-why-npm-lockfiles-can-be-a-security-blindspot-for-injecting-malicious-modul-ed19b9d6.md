---
url: "https://snyk.io/blog/why-npm-lockfiles-can-be-a-security-blindspot-for-injecting-malicious-modules/"
title: "Why npm lockfiles can be a security blindspot for injecting malicious modules | Snyk"
site: "Snyk"
published: "2019-09-24T18:35-05:00"
captured: "2026-11-02T18:46:36Z"
via: "live"
sha256: "52dfc991ddf32ea1ab2f9d18192be0ca49d8f45cf16e604db08e48db15d4e85c"
wayback: "https://web.archive.org/web/20260808213954/https://snyk.io/blog/why-npm-lockfiles-can-be-a-security-blindspot-for-injecting-malicious-modules/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Why npm lockfiles can be a security blindspot for injecting malicious modules | Snyk

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://snyk.io/blog/why-npm-lockfiles-can-be-a-security-blindspot-for-injecting-malicious-modules/>
> Snapshot: <https://web.archive.org/web/20260808213954/https://snyk.io/blog/why-npm-lockfiles-can-be-a-security-blindspot-for-injecting-malicious-modules/>

# Why npm lockfiles can be a security blindspot for injecting malicious modules

Written by

[Liran Tal](https://snyk.io/contributors/liran-tal/)

September 24, 2019

0 mins read

I recently started playing around with the idea of threat modeling packages on the npm ecosystem. Can an [event-stream incident](https://snyk.io/blog/a-post-mortem-of-the-malicious-event-stream-backdoor/) happen again? How about other supply chain attacks? What will be the next vector of attack that we haven’t seen yet and might it be entirely preventable?

And then, one day I had a eureka! ? Let me show you how easy it is to introduce back doors that are easily missed by project owners… leaving your code insecure.

The following picture shows how I injected a malicious package through a pull request I made on an open source package on npm.

Take a closer look and see if you can spot it:

Now, when I go through my pull request review checklist for this PR, this is how it looks:

-

The packages I introduced are known in the ecosystem and are [vulnerabilities free](https://snyk.io/product/) - check.

-

There are no typosquatting attempts in these package names - check.

-

These are valid versions of those packages and aren’t malicious in and of themselves - check.

Looks good to me. Let’s go ahead and merge!

But wait. Did you carefully review that lockfile? Of course not, and I don’t blame you. It’s not at all obvious, considering these restraints:

-

GitHub’s interface defaults to folding diffs whenever they are larger than a couple hundred lines of code in length.

-

What’s the point in reviewing this file? It’s machine-generated and it’s also not very reader-friendly.

Regardless of these restraints, what could go wrong really?

Let’s expand the diff and take a look at what’s inside:

When a lockfile is present, whether that is Yarn's yarn.lock or npm's package-lock.json, an install will consult the lockfile as the primary source of truth for package versions and their sources for dependencies installation.

See what I meant before? A 500-line change on a lockfile is actually a pretty low number. I’ve seen thousands of line changes applied to an npm package-lock.json or a Yarn yarn.lock file. I mean, who really reviews thousands of lines of character changes? To make matters worse, in this lockfile we’ve seen dependency version changes and metadata updates– nothing really out of the ordinary all in all.

If we scroll a bit further down to review the rest of the changes applied to this file we find this gem:

What becomes clear when you look closer, is that I replaced the original ms npm package to resolve it with my own version, which is stored in my GitHub repository. I should have gotten it from the official npm registry, just as was originally set in the lockfile of the project.

This project uses a few different semvers of ms, while I only changed ms@2.1.1 to use my own customized rendition:

```
yarn list --pattern ms
yarn list v1.17.3
├─ @babel/helper-module-transforms@7.4.4
├─ debug@2.6.9
│  └─ ms@2.0.0
├─ express@4.17.1
│  └─ ms@2.1.1
├─ ms@2.1.2
├─ send@0.16.2
│  └─ ms@2.0.0
├─ serve-static@1.14.1
│  └─ ms@2.1.1
✨  Done in 0.49s.
```

It’s pretty hard to remember to review on such a granular level - and it’s no wonder so few actually review lockfile changes character-by-character!

So, what does my own version of ms@2.1.1 do?

When this pull request gets merged, I inject my malicious version of ms@2.1.1 into the code in order to control its behavior during runtime.

In this way, I could introduce a backdoor, alter the logic of the ms module or I could run some postinstall scripts. Additionally, the hosting URL at https://github.com/lirantal/ms/tarball/master is obviously alarming for this experiment but would it be easy for you to spot it if I were to buy a typosquatting domain to host it under https://registry.npmgs.org/ms/ ?

For the sake of this experiment, my ms-hosted package has a simple postinstall script that outputs some text and writes to a file so that if you were to install the package itself, you could then confirm that the post-install event simply executes upon a yarn install:

```
"scripts": {
   "postinstall": "echo im installed && echo hello > /tmp/world.txt",
}
```

For project-based applications, lockfile security is critical and is susceptible to the type of malicious injection attacks as this article outlines.

Lockfiles being used in packages as libraries however, are solely designed for use within the development workflow of the package itself. For that reason, when the project (package) is installed as a dependency in another project, its lockfile isn’t used. This means that lockfile injection would primarily compromise and risk project maintainers and collaborators as far as libraries are concerned.

## What can we do to increase lockfile security?

Because lockfiles are meant to be read by machines it means that they are easy to process automatically. That makes it the perfect job for a linter and some preset policies!

As a countermeasure against the above attack vector, I built [lockfile-lint](https://github.com/lirantal/lockfile-lint) to help with this task. I added the file as another short step of the static code analysis that you can run on a CI server such as Travis CI or Circle CI.

At the very least, you should validate all resources that are served:

-

over HTTPS

-

from a trusted source

As an example, to run those checks in CI for a Yarn project, it’s as simple as:

```
$ npx lockfile-lint --path yarn.lock --type yarn --validate-https --allowed-hosts yarnpkg.org

```

Here's an example showing common errors detected while linting a project's lockfile:

## Summary

Attacks can manifest in many different forms and types… even from lockfiles!

With the [event-stream incident](https://snyk.io/blog/a-post-mortem-of-the-malicious-event-stream-backdoor/) we’ve seen the first successful launch of a social engineering attack on open source project maintainers. This was perhaps one of the most sophisticated attacks we’ve seen on software supply-chain security in the npm and JavaScript ecosystem to date.

A lockfile injection could attack in a similar fashion, and so you should have proper practices in place to mitigate this issue, or at least reduce the risk. Consider the following:

-

Carefully review changes to lockfiles ([lockfile-lint](https://github.com/lirantal/lockfile-lint) can help with that).

-

Allow lockfile changes from core maintainers and not sporadic contributors.

-

Avoid the use of lockfiles entirely for libraries.

If you're interested to read more about lockfiles and how they are used, I wrote in depth about [making sense of package lockfiles in the npm ecosystem](https://snyk.io/blog/what-is-package-lock-json/).

## Get started in capture the flag

Learn how to solve capture the flag challenges by watching our virtual 101 workshop on demand.

[Watch now](https://go.snyk.io/240418_ctf-101-workshop.html)

Blog

[PHP2Shell Won't Be the Last. Here's Where the Next One Will Show Up First.](https://snyk.io/blog/php2shell-where-next-one-will-show-up-first/)

PHP2Shell showed how quickly an engine-level flaw can put internet-facing applications at risk. Learn why outside-in inventory, rapid testing, and repeat checks matter before the next disclosure.

Blog

[PHP Patches Two Critical Unauthenticated RCEs (CVE-2026-109618, CVE-2026-109619): [Updated for CVE-2026-115560]](https://snyk.io/blog/php-patches-critical-unauthenticated-rces/)

Two critical flaws in PHP’s request lifecycle can enable unauthenticated remote code execution. Upgrade to a fixed release or install a vendor backport for older versions.

Blog

[Turn Your Vulnerability Backlog Into Pull Requests With Agentic Remediation](https://snyk.io/blog/evo-remediation-agent-open-preview/)

The Evo Remediation Agent is now in Open Preview, bringing automated dependency resolution to Evo Agentic AppSec. AppSec teams can now run ongoing remediation
