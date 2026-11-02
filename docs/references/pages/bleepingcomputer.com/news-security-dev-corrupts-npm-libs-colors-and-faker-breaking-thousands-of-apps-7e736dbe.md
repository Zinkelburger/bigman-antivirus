---
url: "https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/"
title: "Dev corrupts NPM libs 'colors' and 'faker' breaking thousands of apps"
site: "BleepingComputer"
captured: "2026-11-02T18:53:17Z"
via: "live"
sha256: "7e73fd2ca45945a02abf17b7ca08dcf72c2137c11f23c658ef0891fe4b4aec38"
wayback: "https://web.archive.org/web/20261023011858/https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Dev corrupts NPM libs 'colors' and 'faker' breaking thousands of apps

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/>
> Snapshot: <https://web.archive.org/web/20261023011858/https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/>

https://www.bleepingcomputer.com/

- https://www.facebook.com/BleepingComputer
- https://twitter.com/BleepinComputer
- https://infosec.exchange/@BleepingComputer
- https://www.bleepingcomputer.com/feed/

https://www.wiz.io/lp/securing-ai-agents-101?utm_source=bleepingcomputer&utm_medium=display&utm_campaign=FY26Q3_INB_FORM_Securing-AI-Agents-101&sfcid=701Py00000RTEWMIA5&utm_term=FY27Q4-bleepingcomputer-970x250-Nov&utm_content=AI-Agents-101

- [Home](https://www.bleepingcomputer.com/)
- [News](https://www.bleepingcomputer.com/news/)
- [Security](https://www.bleepingcomputer.com/news/security/)
- Dev corrupts NPM libs 'colors' and 'faker' breaking thousands of apps

# Dev corrupts NPM libs 'colors' and 'faker' breaking thousands of apps

By

###### [Ax Sharma](https://www.bleepingcomputer.com/author/ax-sharma/)

- January 9, 2022
- 09:17 AM
- [32](https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#comments)

Users of popular open-source libraries 'colors' and 'faker' were left stunned after they saw their applications, using these libraries, printing gibberish data and breaking.

Some surmised if the NPM libraries had been compromised, but it turns out there's much more to the story.

The developer of these libraries intentionally introduced an infinite loop that bricked thousands of projects that depend on 'colors' and 'faker.'

https://www.wiz.io/lp/ai-threat-readiness-playbook?utm_source=bleepingcomputer&utm_medium=display&utm_campaign=FY27Q2_INB_FORM_AI-Threat-Readiness-Playbook&sfcid=701Vh00000cnU0lIAE&utm_term=FY27Q4-bleepingcomputer-970x250-Nov&utm_content=AITR-Playbook

The colors library receives over [20 million weekly downloads](https://www.npmjs.com/package/colors) on npm alone and has almost 19,000 projects relying on it. Whereas, [faker](https://www.npmjs.com/package/faker) receives over 2.8 million weekly downloads on npm, and has over 2,500 dependents.

## Open Source Revolution?

The developer behind popular open-source NPM libraries 'colors' (aka colors.js on [GitHub](https://github.com/Marak/colors.js)) and 'faker' (aka faker.js on [GitHub](https://github.com/marak/Faker.js/)) intentionally introduced mischievous commits in them that are impacting thousands of applications relying on these libraries.

Yesterday, users of popular open-source projects, such as [Amazon's Cloud Development Kit](https://github.com/aws/aws-cdk/issues/18323) (aws-cdk) were left stunned on seeing their applications print gibberish messages on their console.

These messages included the text 'LIBERTY LIBERTY LIBERTY' followed by a sequence of non-ASCII characters:

Users left stunned on seeing garbage data printed by 'faker' and 'colors' projects (GitHub)

Initially, users suspected that the libraries 'colors' and 'faker' used by these projects were compromised [[1](https://github.com/aws/aws-cdk/issues/18323), [2](https://github.com/workshopper/javascripting/issues/327), [3](https://github.com/oclif/oclif/issues/786)], similar to how [coa, rc,](https://www.bleepingcomputer.com/news/security/popular-coa-npm-library-hijacked-to-steal-user-passwords/) and [ua-parser-js](https://www.bleepingcomputer.com/news/security/popular-npm-library-hijacked-to-install-password-stealers-miners/) libraries were hijacked last year by malicious actors.

But, in fact, it was the dev behind these two packages who appears to have intentionally [committed](https://github.com/Marak/colors.js/commit/074a0f8ed0c31c35d13d28632bd8a049ff136fb6?diff=unified) the code responsible for the major blunder, as seen by BleepingComputer.

The developer, named Marak Squires added a "new American flag module" to colors.js library yesterday in version v1.4.44-liberty-2 that he then [pushed to GitHub](https://github.com/Marak/colors.js/commit/6bc50e79eeaa1d87369bb3e7e608ebed18c5cf26) and npm. Tainted versions 1.4.1, and 1.4.2 also followed on npm.

colors.js mischievous commit made by 'Marak' (GitHub)

The [infinite loop](https://github.com/Marak/colors.js/commit/074a0f8ed0c31c35d13d28632bd8a049ff136fb6#diff-92bbac9a308cd5fcf9db165841f2d90ce981baddcb2b1e26cfff170929af3bd1R18) introduced in the code will keep running indefinitely; printing the gibberish non-ASCII character sequence endlessly on the console for any applications that use 'colors.'

Likewise, a sabotaged version 6.6.6 of faker [was published to](https://github.com/Marak/faker.js/blob/master/package.json#L4) GitHub and npm.

"It's come to our attention that there is a zalgo bug in the v1.4.44-liberty-2 release of colors," [mocked](https://github.com/Marak/colors.js/issues/285) the developer.

"Please know we are working right now to fix the situation and will have a resolution shortly."

[Zalgo text](https://www.bleepingcomputer.com/news/technology/why-certain-characters-glitch-gmail-youtube-and-twitter/) refers to certain non-ASCII characters that appear glitchy.

The reason behind this mischief on the developer's part appears to be retaliation—against mega-corporations and commercial consumers of open-source projects who extensively rely on cost-free and community-powered software but do not, according to the developer, give back to the community.

In November 2020, Marak had warned that he will no longer be supporting the big corporations with his "free work" and that commercial entities should consider either forking the projects or compensating the dev with a yearly "six figure" salary.

"Respectfully, I am no longer going to support Fortune 500s ( and other smaller sized companies ) with my free work. There isn't much else to say," the developer previously [wrote](http://web.archive.org/web/20210704022108/https://github.com/Marak/faker.js/issues/1046).

"Take this as an opportunity to send me a six figure yearly contract or fork the project and have someone else work on it.

Interestingly, as of today, BleepingComputer noticed that the developer also modified the [README](https://github.com/marak/Faker.js/) page for faker's GitHub repo to make reference to [Aaron Swartz](https://en.wikipedia.org/wiki/Aaron_Swartz):

"What really happened with Aaron Swartz?"

Swartz was an American programmer, entrepreneur, and renowned hacktivist who, following a legal battle, [died by suicide](https://www.eff.org/deeplinks/2013/01/farewell-aaron-swartz).

To make information freely accessible to all, the hacktivist downloaded millions of journal articles from the JSTOR database present on the MIT campus network, allegedly by rotating his IP and MAC addresses repeatedly to get around the technological blocks put in place by JSTOR and MIT.

While doing this, Swartz may have run afoul of the Computer Fraud and Abuse Act and faced criminal charges, with penalties of up to thirty-five years in prison.

## Uncanny can of worms

Marak's bold move has opened up a can of worms and attracted mixed responses.

Some members of the open-source software community have praised the developer's actions, while others are appalled by it.

"Apparently the author of 'colors.js' is angry for not being payed [sic]... So he decided to print the American flag each time his library is loaded... WTF," [tweeted](https://twitter.com/galtashma/status/1479929747337580546) one user.

Some [dubbed](https://twitter.com/vaultec81/status/1479991084587499521) this an instance of "yet another OSS developer going rogue," whereas InfoSec expert VessOnSecurity called the action "[irresponsible](https://twitter.com/VessOnSecurity/status/1480189534625320960)," stating:

"If you have problems with business using your free code for free, don't publish free code. By sabotaging your own widely used stuff, you hurt not only big business but anyone using it. This trains people not to update, 'coz stuff might break."

GitHub has reportedly suspended the developer's account. And, that too, has caused mixed reactions:

NPM has reverted to a previous version of the faker.js package and Github has suspended my access to all public and private projects. I have 100s of projects. [#AaronSwartz](https://twitter.com/hashtag/AaronSwartz?src=hash&ref_src=twsrc%5Etfw) [pic.twitter.com/zFddwn631S](https://t.co/zFddwn631S)

— marak (@marak) [January 6, 2022](https://twitter.com/marak/status/1479200803948830724?ref_src=twsrc%5Etfw)

"Removing your own code from [GitHub] is a violation of their Terms of Service? WTF? This is a kidnapping. We need to start decentralizing the hosting of free software source code," [responded](https://twitter.com/sgomez/status/1480148595059965956) software engineer Sergio Gómez.

"Never know what happened but I’m hosting all of my projects on GitLab private instance just in cause things like this happening to me. Never trust any internet service provider," [tweeted](https://twitter.com/Lakr233/status/1480178455736033282) another.

"Marak yeeted faker and colors, bricking tons of projects, and expected nothing to happen?" [stated](https://twitter.com/piemadd/status/1480017317278978048) a developer named Piero.

Note, Marak's surprising move follows the recent [Log4j debacle that set the internet on fire](https://www.bleepingcomputer.com/news/security/new-zero-day-exploit-for-log4j-java-library-is-an-enterprise-nightmare/).

Open-source library Log4j is used extensively in a vast range of Java applications, including those developed by corporations and commercial entities.

But, shortly after mass-exploitation of the Log4shell vulnerability, the maintainers of the open-source library worked without compensation over the holidays to patch the project, as [more and more CVEs were being discovered](https://www.bleepingcomputer.com/news/security/all-log4j-logback-bugs-we-know-so-far-and-why-you-must-ditch-215/).

Concerns emerged as to how big businesses were used to "[exploiting](https://www.theregister.com/2021/12/14/log4j_vulnerability_open_source_funding/)" open-source; by consuming it incessantly but not giving back enough to support the unpaid volunteers who sustain these critical projects by giving up their free time.

Some also criticized the netizens and bug bounty hunters hounding the Log4j maintainers who were already "working sleeplessly on mitigation measures; fixes, docs, CVE, replies to inquiries, etc." [[1](https://twitter.com/yazicivo/status/1469349956880408583), [2](https://twitter.com/pentestscraps/status/1473707415635963919), [3](https://twitter.com/GossiTheDog/status/1469848362325270530)].

"The responses to the colors.js/faker.js author sabotaging their own packages are really telling about how many corporate developers think they are morally entitled to open source developers' unpaid labour without contributing anything back," [wrote](https://twitter.com/sadiekatze/status/1480164481112100872) one Twitter user.

Time will tell what the future of open-source software entails, with regards to the [OSS sustainability problem](https://github.blog/2019-01-17-lets-talk-about-open-source-sustainability/).

In the meantime, users of 'colors' and 'faker' NPM projects should ensure they are not using an unsafe version. Downgrading to an earlier version of colors (e.g. 1.4.0) and faker (e.g. 5.5.3) is one solution.

Updates:

10:08 AM ET: Added tweet from @VessOnSecurity after publishing.

11:24 AM ET: Added developer's full name, Marak Squires.

Jan 13, 01:43 AM ET: Sabotaged versions have now been removed from npm.

Jan 18, 01:43 AM ET: The functional versions of the 'faker' project were forked and are now being maintained by a separate team of open source volunteers at fakerjs.dev, who have released a [statement](https://fakerjs.dev/update.html). A new [GitHub](https://github.com/faker-js/faker) repo and the [npm project](https://www.npmjs.com/package/@faker-js/faker) have been created.

https://www.adaptivesecurity.com/lp/nb/email-scan?utm_source=display_network&utm_medium=paid_display&utm_campaign=2026_NA_Q4_Bleepingcomputer_Inadunit_October2026_scan&utm_id=701Rd00000sqmWCIAY

##  [Uncover email threats your existing defenses may miss.](https://www.adaptivesecurity.com/lp/nb/email-scan?utm_source=display_network&utm_medium=paid_display&utm_campaign=2026_NA_Q4_Bleepingcomputer_Inadunit_October2026_scan&utm_id=701Rd00000sqmWCIAY)

Adaptive Security's AI agents analyze email intent, context, and behavior to uncover phishing, business email compromise, and invoice fraud.

See what's getting through with a 14-day read-only scan of Microsoft 365 or Google Workspace, with no MX changes.

[Start your free email security scan.](https://www.adaptivesecurity.com/lp/nb/email-scan?utm_source=display_network&utm_medium=paid_display&utm_campaign=2026_NA_Q4_Bleepingcomputer_Inadunit_October2026_scan&utm_id=701Rd00000sqmWCIAY)

### Related Articles:

[LiteLLM release pipeline hacked to distribute info-stealer malware](https://www.bleepingcomputer.com/news/security/litellm-release-pipeline-hacked-to-distribute-info-stealer-malware/)

[Hugging Face malware campaign returns to hijack GPUs for crypto mining](https://www.bleepingcomputer.com/news/security/hugging-face-malware-campaign-returns-to-hijack-gpus-for-crypto-mining/)

[Hackers hijack Convertr analytics script to steal payment card data](https://www.bleepingcomputer.com/news/security/hackers-hijack-convertr-analytics-script-to-steal-payment-card-data/)

[OVHcloud CLI releases backdoored on GitHub to steal developer secrets](https://www.bleepingcomputer.com/news/security/ovhcloud-cli-releases-backdoored-on-github-to-steal-developer-secrets/)

[WordPress SEO plugin Rank Math compromised to backdoor websites](https://www.bleepingcomputer.com/news/security/wordpress-seo-plugin-rank-math-compromised-to-backdoor-websites/)

- [GitHub](https://www.bleepingcomputer.com/tag/github/)
- [npm](https://www.bleepingcomputer.com/tag/npm/)
- [Open Source](https://www.bleepingcomputer.com/tag/open-source/)
- [Supply Chain](https://www.bleepingcomputer.com/tag/supply-chain/)
- [Supply Chain Attack](https://www.bleepingcomputer.com/tag/supply-chain-attack/)

-
-
-

-
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#

https://www.bleepingcomputer.com/author/ax-sharma/

##### [Ax Sharma](https://www.bleepingcomputer.com/author/ax-sharma/)

Ax Sharma is a security researcher and journalist focused on malware analyses and cybercrime investigations. His expertise includes open source software security, threat intel analysis, and reverse engineering. Frequently featured by leading media outlets like the BBC, Channel 5 (UK), Fortune, WIRED, among others, Ax is an active community member of the OWASP Foundation and the Canadian Association of Journalists (CAJ).

- [Previous Article](https://www.bleepingcomputer.com/news/security/trojanized-dnspy-app-drops-malware-cocktail-on-researchers-devs/)
- [Next Article](https://www.bleepingcomputer.com/offer/deals/dive-into-data-with-this-excel-power-bi-and-alteryx-learning-bundle/)

### Comments

-

######  [Mike_Walsh](https://www.bleepingcomputer.com/forums/u/1003494/mike-walsh/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22108

In all honesty, the guy's absolutely within his rights to sabotage his own work. Especially if he's doing it in his own free time, and not seeing one red nickel in compensation.

Why SHOULD big corporations make a ton of money off the back of somebody else's unpaid labour.....even if they DID do it 'for the love of it'?

Fair's fair. You scratch my back, and I'll return the favour. You stab me in the back.....ditto.

Cuts both ways, y'know.....??

-

######  [Mallissin](https://www.bleepingcomputer.com/forums/u/1170133/mallissin/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22116

In all honesty, the guy's absolutely within his rights to poison his own cupcakes. Especially if he's doing it in his own free time for the bake sale, and not seeing one red nickel in compensation.

Why SHOULD big communities make a ton of money off the back of somebody else's unpaid labour.....even if they DID do it 'for the love of it'?

Fair's fair. You scratch my back, and I'll return the favour. You stab me in the back.....ditto.

Cuts both ways, y'know.....??

(Note: Heavy sarcasm as a form of satire.)

-

######  [FastTurtle](https://www.bleepingcomputer.com/forums/u/1234387/fastturtle/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22120

Not a fair comparison. Nobody had to go to the emergency room because of this.

-

######  [GT500](https://www.bleepingcomputer.com/forums/u/377072/gt500/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22109

From what I read here, the guy sounds like a socialist who got greedy. It also sounds like everyone would be better off not relying on his code, since it obviously can't be considered reliable in the future.

I think he's forgotten the spirit of open source, where things were supposed to be "free as in beer"... If the guy wanted paid for his software, he shouldn't have published it under an open source license.

Advocates for open source spent decades trying to get corporations to use open source software, trying to convince them it was free and they should use it because they'd never have to pay for it. What will stunts like this get us? Will we end up going back to the dark days where corporations only used paid software, regardless of how bad it was, because they felt they couldn't trust open source projects? The days when open source was "fringe" and no one cared about it outside of a small group of overzealous nerds and Linux fanboys?

-

######  [Bonzadog](https://www.bleepingcomputer.com/forums/u/1234384/bonzadog/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22118

"From what I read here, the guy sounds like a socialist who got greedy. It also sounds like everyone would be better off not relying on his code, since it obviously can't be considered reliable in the future.
I think he's forgotten the spirit of open source, where things were supposed to be "free as in beer"... If the guy wanted paid for his software, he shouldn't have published it under an open source license.
Advocates for open source spent decades trying to get corporations to use open source software, trying to convince them it was free and they should use it because they'd never have to pay for it. What will stunts like this get us? Will we end up going back to the dark days where corporations only used paid software, regardless of how bad it was, because they felt they couldn't trust open source projects? The days when open source was "fringe" and no one cared about it outside of a small group of overzealous nerds and Linux fanboys?"

Oh that American horror Word "Socialist"......but I agree with you. One cannot trust this person ever again and what is to stop this from happening again? Why should all software be free....some people put a lot of time and effort into their SW so a payment need not be frowned upon.

-

######  [moduletux](https://www.bleepingcomputer.com/forums/u/1234431/moduletux/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22137

I think there are a lot of misconceptions about Free Software. Free and Open Source Software is not about offering everything for "Free" (no payment), but offering the source code as part of the license, regardless if it costs nothing or hundreds of dollars.

The bigger issue is that one of the reasons for Open Sourcing code is to open it up for additional collaboration/contributors. When users consume that code, but don't give back and get mad if the maintainer stops working on it, that is on the user base as a whole - be it an individual, a company, a gov't, etc - not on the developer. They don't generally open source code just to make it free, but also to get people on board with helping develop/maintain it

-

######  [GT500](https://www.bleepingcomputer.com/forums/u/377072/gt500/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22145

When companies "sell" open source software, they usually aren't actually selling the software itself, but rather they are selling premium support packages for it and/or additional features not available in the open source version.

Red Hat is an excellent example of that. You are not technically buying the open source part of Red Hat Enterprise Linux, but rather the closed source software that comes bundled with it, and the premium support offered by Red Hat. The open source part is free, and gets redistributed by the CentOS project as a fully working enterprise-class Operating System.

Another good example of this is NGINX. The open source part is freely available as an HTTP server that anyone can download and install, however there are paid packages available with premium support and additional features.

So open source software is actually free, even if it doesn't come for free from the company that makes it. The nature of open source license agreements means that anyone can take your source code, make any necessary changes to get it to work on its own, change any names/branding, compile it, and redistribute it for free. Some license agreements are more restrictive of course, however it's usually still possible to reuse someone else's code in part or in full in your own project.

But then we have the incident that this article is about, where someone made a free and open source software, and then without any warning pushed out an update that broke functionality for a great many users for no better reason than "companies should be paying me". As I already said, if he wanted paid for his code, then he shouldn't have published it under an open source license. If he wanted to make changes to existing software to monetize it then he should have announced that in advance, and then he could have made changes to the licensing or rewrote part of the software as a separate closed-source module that required paid licensing to use. He could have also made an "enterprise" version of his software with additional support and features (like the NGINX project did) which required paid licensing and contained closed-source modules with the additional features. But did he do any of this? No, instead he quietly sabotaged his own software, calling the reliability of open source projects into question for every company that was relying on his NPM modules. If open source developers start to think that things like this are OK, then we are very quickly going to end up in a Microsoft-only world again.

-

######  [clawsoon](https://www.bleepingcomputer.com/forums/u/1235081/clawsoon/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22214

I think he's forgotten the spirit of open source, where things were supposed to be "free as in beer"...

Wait, what? Whenever I've heard discussion of open source, they emphasize that they're *not* talking about "free as in beer", they're talking about "free as in speech". "Free as in beer" is literally the example they use to say what they're *not* committed to.

-

######  [GT500](https://www.bleepingcomputer.com/forums/u/377072/gt500/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22246

15+ years ago I heard "free as in beer" repeated over and over to describe open source. Pretty sure I also heard people refute the idea of open source being "free as in speech" back then as well, but it's always possible that those people were mistaken about what these things meant, or were reading articles written by those who were mistaken about them.

-

######  [darkoverlordofdata](https://www.bleepingcomputer.com/forums/u/1222778/darkoverlordofdata/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22110

Thanks, Marak. I write open source software. Looks like I may have to start writing everything myself, if I can't reply on my fellow programmers.

-

######  [Koroush](https://www.bleepingcomputer.com/forums/u/1184380/koroush/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22111

This is entirely understandable.I wrote about this exact phenomenon several years ago in an article entitled The Linux Experiment (mirrored on PCgamingwiki now).

Essentially, while the Free Open Source Software movement and its aims are laudable, its business model and overall approach are more like a religion than anything else. It ultimately results in a lot of very talented people becoming disillusioned - and poor - because they discover that total reliance on karma isn't actually a workable business model.

-

######  [moduletux](https://www.bleepingcomputer.com/forums/u/1234431/moduletux/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22135

"This is entirely understandable.I wrote about this exact phenomenon several years ago in an article entitled The Linux Experiment (mirrored on PCgamingwiki now).
Essentially, while the Free Open Source Software movement and its aims are laudable, its business model and overall approach are more like a religion than anything else. It ultimately results in a lot of very talented people becoming disillusioned - and poor - because they discover that total reliance on karma isn't actually a workable business model."

That's not how it works at all. Open Source is not a business model. Several companies utilize or build open source software as part or all of their business. The difference is in the HOW they want to monetize. Monetizing on support for the software is generally a good route to go for Open Source software. You still get the software for free, but you won't get dedicated support from the vendor/developer without either contributing back yourself or paying for a support fee or via ads.

A developer/company is unlikely to succeed financially in Open Source unless it is either:

1. NOT part of the core product.
2. Make money on the selling of support contracts.

Plenty of companies provide open source and still make plenty of money. A LOT of companies sell convenience or support for products/platforms built on mostly, if not entirely, open source tooling.

-

######  [Koroush](https://www.bleepingcomputer.com/forums/u/1184380/koroush/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22141

Unless a developer is explicitly doing something as a hobby or as a philanthropic venture, and genuinely has no desire or exectations of ever making any real revenue from their project, I disagree.

The problem with FOSS is that it IS effectively a business model, in terms of a model for how developers can present their products to the general public and businesses, but it is largely a nebulous philosophical one based on an idealised notion of karma, with unenforcable fantasy-based copyright provisions that are easily circumvented or just plain ignored in the real world.

Woe betide any FOSS developer who actually gets lucky, creates a very popular piece of software, and then wants to monetize it properly. Both the FOSS community, and all manner of users, will dub them "greedy", and shower them with a litany of shame.

I'm not necessarily supporting how far this particular developer went in sabotaging his own software to get attention. But the problem with FOSS is very real. Everyone needs to receive fair compensation for their work, rather than being essentially duped into believing that free software is the way of the future, and that people will then compensate them accordingly if the software becomes ubiquitous/popular.

It's unsustainable; too many devs have quietly made their excuses and bowed out of highly popular projects, or have had to make radical changes, because they realise that once the novelty of public attention and accolades wears off, they're stuck on an endless cycle of ever-rising "customer" expectations with ever-diminishing revenue.

-

######  [ThePhox1982](https://www.bleepingcomputer.com/forums/u/1174148/thephox1982/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22113

This is precisely why the creator of node.js made Deno. When building apps with Node.js or nw.js you never know if now or anywhere down the road if a dependency you used has a dependency that is malicious or one of whether it's dependencies, dependencies, dependencies, dependency can be trusted. That is the issue with NPM, no matter how security focused you are it is IMPOSSIBLE for you to account for everything else down the line and even if a dependency you choose doesn't have any other dependencies now, doesn't mean they won't update it later, so unless you version lock every single dependency you use (assuming the dependencies created leaves old versions available), you will never know if your program will break or if your program will cause people harm, so therefor your program made with an NPM module will NEVER be secure, EVER. Frankly, with these sorts of things happening weekly, it's amazing anyone uses Node.js anymore and really, it's a security threat for anyone to use it!

-

######  [Bonzadog](https://www.bleepingcomputer.com/forums/u/1234384/bonzadog/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22117

This seems to me to be a very nasty thing to do.....at least without a warning.
Perhaps OSS needs a rethink, since this could happen again.

-

######  [h_b_s](https://www.bleepingcomputer.com/forums/u/1211617/h-b-s/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22146

"This seems to me to be a very nasty thing to do.....at least without a warning.
Perhaps OSS needs a rethink, since this could happen again. "

This isn't an open source problem, it's a problem with the software industry in general. Irregardless of the developer's ethics, it points out a huge hole in how many development houses create software. They're being lazy about gating in dependencies. The proper method for doing so is to check each and every dependency pulled in by your project if it's separate from the original software distribution: example if you're a Python developer and decide your project needs to import Pandas, then you should gate in the specific version of Pandas and only use that one version till the next version is gated in and verified to work both as advertised and with your software project. Yes, it's very time consuming. Yes, it's tedious.
This is no longer a world where we can just assume software is and does what it says it does like in the 90s. In the 2020s+, you can't legitimately make these assumptions any longer. Larger corporations should be paying for support if they want code assurance. There are plenty of companies that offer such support contracts. Smaller ones can contribute collectively as well in both money and code. Individuals can manage on our own time scales.
Generally speaking, this isn't a backlash against individual developers. It's a general revolt against corporations taking and taking and expecting everything while never giving anything back, when they very well can afford paying for assurance support.

-

######  [mynameisgod](https://www.bleepingcomputer.com/forums/u/1201110/mynameisgod/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22119

There was never a contract that said this guy is owed money by corporations. What part of free open source do you not understand? The dev is a certified ASS. This move hurt the open source community. Never in my life did I ever think I would take the side of corporations, until now.

-

######  [HahTse](https://www.bleepingcomputer.com/forums/u/1234403/hahtse/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22131

If only there were some kind of license that would prevent the corpos from getting a free ride on FOSS...perhaps some kind of General, or Public License...

Seriously, it's like no one even remembers Richard Stallman.

-

######  [seary](https://www.bleepingcomputer.com/forums/u/1234531/seary/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22150

"Seriously, it's like no one even remembers Richard Stallman" -
I remember RMS. I don't agree with his religious stance but otherwise, I feel he's an AMAZING guy.

-

######  [TigerNinja](https://www.bleepingcomputer.com/forums/u/1234404/tigerninja/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22132

I use these libraries in my projects that I build from the ground up. I'm not a corporation, thanks.

-

######  [dimayv](https://www.bleepingcomputer.com/forums/u/1234405/dimayv/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22133

This is right action towards reforming or eliminating npm. It is not governed - namesquatting and low quality is all over the place. It is risky and was always risky to subscribe to deps autoupdates.
Devs should get off this hook and be responsible for the code they use in dependencies.
It has plenty of benefits https://twitter.com/DimaYv/status/1417482069257723912?s=20

-

######  [TomTom55](https://www.bleepingcomputer.com/forums/u/1234408/tomtom55/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22134

Maybe he could just change their license to a copyleft one if he want people to contribute.

-

######  [Koroush](https://www.bleepingcomputer.com/forums/u/1184380/koroush/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22142

If only such licenses were practical; as it stands, they're largely just pie-in-the-sky stuff that looks great on paper, but have rarely, if ever, actually been *enforced*. Indeed, the open source community itself encourages people not to pursue litigation. https://opensource.com/article/21/3/test-cases-open-source-licenses

So these licenses are mainly backed up by good will...

-

######  [ranodm489234](https://www.bleepingcomputer.com/forums/u/1234456/ranodm489234/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22143

Open Source projects are typically started out of passion and without any request for, though always appreciated, compensation. It's one thing to stop managing your project because you feel it's become too time-consuming. That happens all the time and is perfectly understandable. At that point, you simply advise people to fork the project if they want to continue maintaining it. However, to intentionally break other peoples' applications because you want to make money off a captive audience is evil.

How does such a move serve the open source community? How does such a move serve the world? Now developers will think twice before they use open source libraries. They will waste time "re-inventing the wheel" and it might not be as secure or bug-free. And that version might end up being used in the software you use to access your sensitive data. If you want to start a project and want compensation for it, I recommend just not offering it for free to begin with. Let your intentions be clear from the start.

-

######  [BeCoolDude](https://www.bleepingcomputer.com/forums/u/1234467/becooldude/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22144

Marak was a rock star. He did something that fewer than 1% of the developers achieve on GitHub--created multiple OSS products that were downloaded millions of times a month. He is an artist and his products are his art portfolio that showed the world his work. Everybody used his products including FAANG. Who wouldn't want to be in his position? And in one fell swoop, he burned his life's work.

His problem and developers alike have a monetization problem, nothing more. Nginx, Elastic, MongoDB, and hundreds of products that started off as free later become hits because they solved the monetization problem. The moral of the story, if you have developed a popular product on GitHub that is free, don't give up. Reach out and get advice from a product marketer. You'd be surprised what a little marketing and creativity can do for a free product. It might make you millions. I know, cause I am one and the idea of working with someone like him on this type of product has me salivating at the potential.

-

######  [rdejournett](https://www.bleepingcomputer.com/forums/u/1234532/rdejournett/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22148

I feel like RedHat etc would be well incentivized to pay for each download. Part of the problem is for most of us we have little idea of what our dependancies are dependant upon. The other thing as a CTO every time you turn around, people are demanding massive amounts of cash for trivial things. Where does it end? If we use your module in our product, do we have to set up licenses for 100 different developers? We have standards organizations (X12) DEMANDING an annual license to use a standard. We have people demanding licenses for libraries. For servers (okay this makes sense), for OSs (also makes sense), and suddenly you realize there is no way in hell you can be profitable because you have 100+ licenses attached to your product. This isn't F500 who's profit is obscene. This is a small business with 20 people and we're barely making ends meet. This could be easily remedied if github etc would require subscription from addresses of major corporations.

-

######  [seary](https://www.bleepingcomputer.com/forums/u/1234531/seary/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22149

Often, local managers or even big corporations like Microsoft will *force* developers into using NPM whether they like it or not. One of my superior team members can't stand .NET Core even though I think it's awesome but I'm pretty much forced to use Node.js and NPM. On the other hand, Microsoft has completely jacked up SharePoint so that it relies heavily on the SPFx framework which is IMO a security nightmare due to it's heavy reliance on NPM. Yes, you can still create a SharePoint Add-in via Visual Studio but that tends to be frowned down upon these days. The old .NET 4 based Windows SharePoint Services (WSS3) was actually pretty decent but this new SharePoint is a total pain to customize. Way too much asynchronous client-side programming with CORS headaches and runtime errors. Classic server side WSS3 customization was never this problematic.

I'm not sure, but I wouldn't be surprised if even Power BI extensions rely heavily on NPM. NPM is awful and Node.js and JavaScript really need to go.

Haskell and WASM or C#/F# and WASM are maybe better options than Node.js. Or maybe this would be a good option: C# .NET Core for backend and React.js for front end? Basically Node.js is is a piece of JUNK. Of course if we go with React.js for frontend, we still don't get away from the NPM nightmare...

-

######  [Labeled](https://www.bleepingcomputer.com/forums/u/1234546/labeled/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22152

This is the same guy who was caught trying to make bombs, and abusing his girlfriend.. he should be permanently banned until he gets the help he needs. https://abc7ny.com/suspicious-package-queens-astoria-fire/6425363/

-

######  [syedrakib](https://www.bleepingcomputer.com/forums/u/1234877/syedrakib/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22192

This is exactly why I avoid auto-updates of package versions. I pin it down to the patch-level. If i ever need to update, i will update it myself when i choose to update it.

-

######  [chadf](https://www.bleepingcomputer.com/forums/u/1091642/chadf/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22244

If someone doesn't like how people [ab]use what they do for free, then that person should simply stop giving it away. If you work for someone and feel you're underpaid, then you quit working for them and find somewhere/something that pays you what you think you're worth.. you don't sabotage your employers systems out of spite. And you don't sabotage the opensource community for feeling "underpaid" out of spite, either.

If you want to develop opensource AND be paid, then either only work on sponsored software, or change to some form of split licensing model (e.g. paid subscribers get early [non-security] updates and priority support -- after 6 months, a year, a major release, whatever.. that code is merged into the free licensed version.

And as some have pointed out.. this is a good reason for not blindly auto-updating, in general. Not just for source, but binary updates. The flip side to numerous IoT devices having unpatched security vulnerabilities is having a rouge "feature" auto-installed on your perfectly fine hardware due to some unchecked library update, a random employee, or even a sketchy company once they have a trusting user base.

-

######  [tomachi](https://www.bleepingcomputer.com/forums/u/1235310/tomachi/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22264

1. Wow that dev has balls of steel. I think if I had done that it would have just pointed people more politely to my bitcoin address for donations. But not the infinite loop.
2. BUT it is very uncool that Github closed his *entire* account! They should allow him back if he gets rid of the CPU DDOS and noisey texts. Plus respect the mans work. Treat it like a deprecation warning.
3. Hopefully he got paid in the end. Interesting way to ask.

-

######  [Bug_Farmer](https://www.bleepingcomputer.com/forums/u/1235680/bug-farmer/)  - 4 years ago

- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#
- https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#cid22342

Don't worry folks! Insurers will get the better of those thieving businesses. They will demand that the code is properly reviewed and that the author/maintainer is suitably remunerated so that they feel inclined to solve bugs quickly. All in the name of business continuity risk. If you don't, your premiums will go up.

##### Post a Comment [Community Rules](https://www.bleepingcomputer.com/posting-guidelines/)

###### You need to login in order to post a comment

Not a member yet? [Register Now](https://www.bleepingcomputer.com/forums/index.php?app=core&module=global&section=register)

### You may also like:

https://www.kaseya.com/lp/next-gen-rmm?utm_source=701Rp00000y8fGN&utm_medium=Other

Popular Stories

-  [Critical Linux kernel flaw allows remote root attacks on MPTCP servers](https://www.bleepingcomputer.com/news/security/critical-linux-kernel-flaw-allows-remote-root-attacks-on-mptcp-servers/)
-  [New Android SMS of Death exploit can remotely crash unpatched phones](https://www.bleepingcomputer.com/news/security/new-android-sms-of-death-exploit-can-remotely-crash-unpatched-phones/)
-  [CISA says max severity GitLab flaw now exploited in the wild](https://www.bleepingcomputer.com/news/security/hackers-now-exploiting-max-severity-gitlab-flaw-in-attacks/)

Sponsor Posts

-  [See every AI tool in use. Control what data reaches it. Try Keep Aware for free.](https://keepaware.com/demo/generative-ai-monitoring?utm_source=bleeping_computer&utm_medium=article&utm_campaign=october26)
-  [Want to break into cyber security? Learn hands-on in your browser with TryHackMe.](https://tryhackme.com/)
-  [Your carrier tracks you. We don’t. Switch to Cape, America’s Privacy-first Mobile Carrier.](https://www.cape.co/?utm_source=BleepingComputer&utm_medium=article_sponsorship&utm_campaign=BleepingComputer_ArticleSponsorship_Nov2)
-  [Your carrier tracks you. We don’t. Switch to Cape, America’s Privacy-first Mobile Carrier.](https://www.cape.co/?utm_source=BleepingComputer&utm_medium=article_sponsorship&utm_campaign=BleepingComputer_ArticleSponsorship_Oct30)
-  [Identity-based order to control the chaos of your agentic AI. See Token Security’s AI Agent Security](https://www.token.security/book-a-demo?utm_source=bleepingcomputer&utm_medium=3rd-party&utm_campaign=bleepingcomputer&utm_content=oct-28)
-  [Stay one step ahead of new threats in the new year. Join Huntress for the monthly Tradecraft Tuesday.](https://www.huntress.com/community-series/tradecraft-tuesday?utm_source=bleepingcomputer&utm_medium=resource_link&utm_campaign=cy26-09-camp-multi-global-prospect-all-x-x-social_eng-bleeping_computer&hnt=nsnzaa2spign)

https://www.threatlocker.com/stop-judgment-day?utm_source=bleeping_computer&utm_medium=sponsor&utm_campaign=&utm_content=judgment_day_sticky_sidebar-400x500&utm_term=display

https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#

#### Login

#### Reporter

###### Help us understand the problem. What is going on with this comment?

Submitting...

[SUBMIT](https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/#)
