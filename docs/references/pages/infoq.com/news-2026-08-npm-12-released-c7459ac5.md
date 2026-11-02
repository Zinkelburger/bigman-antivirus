---
url: "https://www.infoq.com/news/2026/08/npm-12-released/"
title: "npm 12 Released: Install Scripts off by Default as Registry Moves to Explicit Trust"
site: "InfoQ"
captured: "2026-11-02T18:54:59Z"
via: "live"
sha256: "f176f8e048978783b616fdcd51b5809d93cb4f8df2e276d3d161ca6a8df2fe16"
wayback: "https://web.archive.org/web/20260831142330/https://www.infoq.com/news/2026/08/npm-12-released/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# npm 12 Released: Install Scripts off by Default as Registry Moves to Explicit Trust

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.infoq.com/news/2026/08/npm-12-released/>
> Snapshot: <https://web.archive.org/web/20260831142330/https://www.infoq.com/news/2026/08/npm-12-released/>

[InfoQ Homepage](https://www.infoq.com/) [News](https://www.infoq.com/news) npm 12 Released: Install Scripts off by Default as Registry Moves to Explicit Trust

[Web Development](https://www.infoq.com/Web-Development/)

#  npm 12 Released: Install Scripts off by Default as Registry Moves to Explicit Trust

Aug 14, 2026 3 min read

by

-

https://www.infoq.com/profile/Daniel-Curtis/  [Daniel Curtis](https://www.infoq.com/profile/Daniel-Curtis/)

#### Follow us on

[Youtube232K Followers](https://bit.ly/4bg6QM8) [Linkedin26K Followers](https://bit.ly/44IzAtf) [InstagramNew](https://bit.ly/4eYXrtM) [RSS19K Readers](https://bit.ly/3RaJalC) [X57.1k Followers](https://bit.ly/4pfxivv) [Facebook21K Likes](https://bit.ly/3QrGMH2) [BlueskyNew](https://bit.ly/4eS8FjG)

Listen to this article - 0:00

Audio ready to play

0:00

0:00

- [Reading list](https://www.infoq.com/showbookmarks.action)

npm, the package manager maintained by GitHub and shipped with Node.js, has [released npm 12](https://github.blog/changelog/2026-07-08-npm-install-time-security-and-gat-bypass2fa-deprecation/), shipping with a set of security related default changes to npm install alongside the deprecation of granular access tokens that bypass two factor authentication.

npm 12 turns three install behaviours that previously ran automatically into ones developers explicitly opt into. The changes were [first announced in June](https://github.blog/changelog/2026-06-09-upcoming-breaking-changes-for-npm-v12/) and have been available behind warnings since npm 11.16.0, so teams could prepare before upgrading.

The headline change is that allowScripts now defaults to off. Dependency preinstall, install and postinstall scripts no longer run unless they are explicitly allowed in a project, and that includes implicit node-gyp builds for any package containing a binding.gyp file, even where no install script is declared. prepare scripts from git, file and link dependencies are blocked in the same way. Developers review what is pending, approve what they trust, and commit the resulting allowlist in package.json.

The other two defaults target non registry sources. --allow-git now defaults to none, closing a code execution path where a Git dependency's own .npmrc could override the Git executable even with --ignore-scripts set, and --allow-remote defaults to none, blocking https tarball dependencies. The related --allow-file and --allow-directory flags are unchanged.

GitHub's [migration discussion](https://github.com/orgs/community/discussions/198547) recommends allowing what is already in the tree first, then tightening, and documents recipes for native modules, Cypress, Playwright, Puppeteer, Electron and Husky. Global installs and npx cannot use approve-scripts and need the config instead, for example npm config set allow-scripts=canvas,sharp --location=user. Full details are in the docs for [npm approve-scripts](https://docs.npmjs.com/cli/v11/commands/npm-approve-scripts), [npm deny-scripts](https://docs.npmjs.com/cli/v11/commands/npm-deny-scripts) and [allow-scripts config](https://docs.npmjs.com/cli/v11/using-npm/config). Commenters on the thread flagged that an existing ignore-scripts=true takes precedence and silently defeats the allowlist, and that newly installed packages create a chicken and egg problem, since npm approve-scripts reads from node_modules and errors with ENOMATCH if the package is not yet installed.

Community reaction has been broadly supportive. The [Hacker News thread](https://news.ycombinator.com/item?id=48467705) drew 484 points and more than 200 comments, with atraac writing that:

postinstall scripts should've been removed long time ago, it's the cancer of NPM packages. There's so many deeply nested, uncontrolled postinstalls that run randomly when you pull something it's insane, I don't know how someone at some point ever though that was a good idea.

Others were less convinced. gear54rus pointed to legitimate use cases such as [patch-package](https://www.npmjs.com/package/patch-package), while cookiengineer argued the allowlist ‘has no scope’ and leaves it ‘unpredictable if any of your dependencies of dependencies needs a script’.

Security researchers have raised approval fatigue as the bigger risk. Writing on [OpenSourceMalware](https://opensourcemalware.com/blog/npm-v12), one analyst noted that esbuild, sharp, core-js, puppeteer and bcrypt all rely on lifecycle scripts, and warned that repeated broken builds will turn deny by default into ‘a click-through prompt’, while pushing attacker activity onto surfaces with less visibility. [JFrog](https://jfrog.com/blog/npm-v12-from-implicit-to-explicit-trust/) reports the three vectors were involved in roughly 53% of malicious npm attacks observed in the past year.

npm is also the last major package manager to adopt these controls. pnpm has offered install script allowlisting for years, and on cooldowns pnpm shipped minimumReleaseAge in 10.16, yarn added npmMinimalAgeGate in 4.10.0 and bun followed in 1.3, ahead of npm's own min-release-age in 11.10.0.

npm is the default package manager for Node.js and is maintained by GitHub. It hosts the largest software registry in the world, giving developers a command line client and registry to install, share and publish reusable JavaScript and TypeScript packages. As a foundational part of the wider Node.js ecosystem, npm is used across frontend and backend projects to manage dependencies, run scripts and distribute open-source code at scale.

## About the Author

https://www.infoq.com/profile/Daniel-Curtis/

#### Daniel Curtis

Show moreShow less

####  This content is in the [Web Development](https://www.infoq.com/Web-Development/) topic

##### Related Topics:

-  [Development](https://www.infoq.com/development/)
-  [Web Development](https://www.infoq.com/Web-Development/)
-  [JavaScript](https://www.infoq.com/javascript/)
-  [TypeScript](https://www.infoq.com/TypeScript/)
-  [Package Managers](https://www.infoq.com/PackageManagers/)
-  [Node.js](https://www.infoq.com/nodejs/)

-

#### Related Editorial

-

#### Related Sponsors

-

#### Popular across InfoQ

-

#####  [AI, ML, and Data Engineering InfoQ Trends Report 2026](https://www.infoq.com/podcasts/ai-ml-data-engineering-trends-2026/)

-

#####  [Anthropic Says Nobody Needs a Mental Model of Claude Code. Practitioners Are Testing That](https://www.infoq.com/news/2026/10/anthropic-code-review/)

-

#####  [How Anthropic Redesigned Source Control for AI-Driven Development](https://www.infoq.com/news/2026/10/anthropic-source-control/)

-

#####  [File Mover Express: AWS Releases Open-Source Tool for S3 File Transfers](https://www.infoq.com/news/2026/11/aws-s3-file-transfer/)

-

#####  [AGNTCon + MCPCon 2026: Agentic AI Foundation Adds Four Projects as MCP Turns to Security](https://www.infoq.com/news/2026/10/agntcon-mcpcon-2026/)

-

#####  [Cloudflare Opens Quicksilver to Customers as Workers KV Instant, inside a 1 MB Envelope](https://www.infoq.com/news/2026/10/workers-kv-instant/)

### The InfoQ Newsletter

A round-up of last week’s content on InfoQ sent out every Tuesday. Join a community of over 250,000 senior developers. [View an example](https://assets.infoq.com/newsletter/regular/en/newsletter_sample/newsletter_sample.html)

[We protect your privacy.](https://www.infoq.com/privacy-notice/)
