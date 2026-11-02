---
url: "https://github.com/axios/axios/issues/10636"
title: "Post Mortem: axios npm supply chain compromise · Issue #10636 · axios/axios"
site: "GitHub"
captured: "2026-11-02T18:47:40Z"
via: "live"
sha256: "fbe8a2689e7b216677f9d5c0f9ef5db29c043fe8d7c70abb69bf852673a4c892"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# Post Mortem: axios npm supply chain compromise · Issue #10636 · axios/axios

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/axios/axios/issues/10636>

## Repository navigation

# Post Mortem: axios npm supply chain compromise #10636

Closed

Closed

[Post Mortem: axios npm supply chain compromise](https://github.com/axios/axios/issues/10636#top)#10636

Assignees

https://github.com/jasonsaayman

Labels

[type::securityThe PR is a secuirty related changed normally from a CVE](https://github.com/axios/axios/issues?q=state%3Aopen%20label%3A%22type%3A%3Asecurity%22)

## Description

https://github.com/jasonsaayman

[jasonsaayman](https://github.com/jasonsaayman)

opened [on Apr 2, 2026](https://github.com/axios/axios/issues/10636#issue-4195231282)

# Post Mortem: axios npm supply chain compromise

Date: March 31, 2026
Author: Jason Saayman
Status: Remediation in progress

On March 31, 2026, two malicious versions of axios (1.14.1 and 0.30.4) were published to the npm registry through my compromised account. Both versions injected a dependency called plain-crypto-js@4.2.1 that installed a remote access trojan on macOS, Windows, and Linux.

The malicious versions were live for about 3 hours before being removed.

## Are you affected?

Check your lockfile:

```
grep -E "axios@(1\.14\.1|0\.30\.4)|plain-crypto-js" package-lock.json yarn.lock 2>/dev/null
```

If anything comes back, treat that machine as compromised:

- Downgrade to axios@1.14.0 (or 0.30.3 for 0.x users)
- Delete node_modules/plain-crypto-js/
- Rotate every secret, token, and credential on that machine
- Check your network logs for connections to sfrclak[.]com or 142.11.206.73 on port 8000
- If this happened on a CI runner, rotate any secrets that were injected during the affected build

If you were already pinned to a clean version and didn't run a fresh install between 00:21 and 03:15 UTC on March 31, you're fine.

For detailed remediation guidance, including CI/CD-specific steps, see:

- [StepSecurity: full technical analysis and remediation](https://www.stepsecurity.io/blog/axios-compromised-on-npm-malicious-versions-drop-remote-access-trojan)
- [Snyk: advisory and scanning guidance](https://snyk.io/blog/axios-npm-package-compromised-supply-chain-attack-delivers-cross-platform/)
- [Socket: Supply Chain Attack on Axios Pulls Malicious Dependency from npm](https://socket.dev/blog/axios-npm-package-compromised)

## What happened

The attacker gained access to the lead maintainer's PC through a targeted social engineering campaign and RAT malware. This gave them access to the npm account credentials, which they used to publish the malicious versions.

The attack followed patterns documented in similar campaigns targeting open source maintainers.

## Resolution

Complete wipe of all lead maintainer devices as well as resetting of all credentials. This includes all accounts irrespective of platform, both personal and all other capacities.

## What's changing

To prevent this from happening again, these are the changes being made:

| Action  | Type

| Reset of all devices and credentials  | Prevention

| Immutable release setup  | Prevention

| Proper adoption of OIDC flow for publishing  | Prevention

| Improvement of overall security posture  | Prevention

| Updating of all GitHub actions to adopt best practices  | Prevention

This list is not the end. We will continue actively working on improving security beyond these items.

## Attack Timeline

I don't have an exact timeline for when the initial compromise occurred, but this is the sequence of events for the package itself:

- About 2 weeks before March 31: social engineering campaign initiated against the lead maintainer
- March 30, 05:57 UTC: plain-crypto-js@4.2.0 published to npm
- March 31, 00:21 UTC: axios@1.14.1 published with plain-crypto-js@4.2.1 injected
- March 31, around 01:00 UTC: axios@0.30.4 published with the same payload
- March 31, around 01:00 UTC: first external detections
- March 31, around 01:00 UTC: community members file issues reporting the compromise. The attacker deletes them using the compromised account.
- March 31, 01:38 UTC: axios collaborator DigitalBrainJS opens PR [chore(ci): add deprecate action; #10591](https://github.com/axios/axios/pull/10591) to deprecate the compromised versions, flags the deleted issues to the community, and contacts npm directly
- March 31, 03:15 UTC: malicious versions removed from npm
- March 31, 03:29 UTC: plain-crypto-js removed from npm

For detailed technical timelines:

- [StepSecurity: forensic timeline](https://www.stepsecurity.io/blog/axios-compromised-on-npm-malicious-versions-drop-remote-access-trojan)
- [Datadog Security Labs: full attack flow and response analysis](https://securitylabs.datadoghq.com/articles/axios-npm-supply-chain-compromise/)

## Lessons learned

We are actively investigating how unauthorized access was obtained and reviewing all aspects of our security posture and access controls. While we do not have confirmed details to share at this time, this incident reinforces the need for continuous monitoring, strong account protections, and vigilance across both project infrastructure and individual maintainer environments.

We will continue to strengthen our practices in alignment with industry best practices and in collaboration with groups such as the OpenJS Security Working Group.

Security posture should be continuously monitored and improved. Specifically from this incident:

- Publishing directly from a personal account was a risk that could have been avoided. The OIDC flow and immutable release setup we are now adopting should have been in place before this happened.
- There was no automated way to detect an unauthorized publish. Detection depended entirely on the community noticing.
- Open source maintainers with high-impact packages are active targets for sophisticated social engineering. Hyper vigilance is needed both on the registry and in a personal capacity.

## Summary for broarder audience

We can confirm that two compromised versions of Axios were briefly made available through a widely used software download system due to unauthorized access to the lead maintainer's account, not a change to the underlying code.

We are working with the companies that operate this system, along with security experts, to investigate and contain the issue. Organizations that installed these versions should remove them immediately and treat affected systems as potentially compromised, including rotating any sensitive credentials.

This was the result of a compromised lead maintainer's account, and we are actively investigating how access was obtained. We do not have confirmed details to share at this time.

## Acknowledgments

Thanks to [@DigitalBrainJS](https://github.com/DigitalBrainJS) for acting fast when the compromised account had higher permissions than his own and for getting npm to take action. Thanks to Open Source Community and the npm security team for their fast response.

The malicious versions have been removed from npm and are no longer infecting users. The immediate incident is resolved. We are actively working on the security improvements listed above to prevent this from happening again.

I'll update this as the investigation progresses. Questions welcome in this thread.

Reacted by Rossvold, Équipe Octopusmind, chpsueey, Gustavo Adrián Salvini, Jan Werner, Rodrigo Sicarelli, Naor Peled, Vincent Lee, Hani Abdulfattah, Ivan Nikolić and 584 more

Reacted by Victor Garcia, adharshrajeev2, Brian A. Teller, hubcopilottest and Steve

Reacted by Avindra Goolcharan, Rafael Silveira, Michitoshi Yano, Matheus, Francisco Javier Arceo, Waleed Ahmad, Luis Enrique, adharshrajeev2, Kacper Zawojski, Ka Mok and 1 more

Reacted by Suraj Apar, Rafael Silveira, Michitoshi Yano, Rush, OUARHOU Lhoussaine, Matheus, lwjef, Jonathan Kuntz Fornari, Trond Klakken, Luis Enrique and 11 more

Reacted by RoseSecurity, Thomas Broyer, Harutaka Kawamura, Rossvold, Daniel Ruf, Kristopher Johnson, Ulises Gascón, Robin Bender Ginn, gitgotgitgotit, Jan Werner and 221 more

Reacted by João Pedro V. Pansiere, adharshrajeev2, Brian A. Teller, radardosprecos, Daniel Hrynusiw, Kacper Zawojski, Aleksandr Barmin, Mau, hubcopilottest and Viet Nguyen (Jack)

Reacted by 半岛的蒟蒻bddjr, aiboli, Avindra Goolcharan, Patrick Nelson, Rafael Silveira, Richard Cardona, Akira Yokochi, codey-badger, masashi-sutou, Takashi Sato and 25 more

## Activity

-

[jasonsaayman](https://github.com/jasonsaayman)

self-assigned this

[on Apr 2, 2026](https://github.com/axios/axios/issues/10636#event-24151823365)

-

[jasonsaayman](https://github.com/jasonsaayman)

added

[type::securityThe PR is a secuirty related changed normally from a CVE](https://github.com/axios/axios/issues?q=state%3Aopen%20label%3A%22type%3A%3Asecurity%22)

[on Apr 2, 2026](https://github.com/axios/axios/issues/10636#event-24151824144)

-

[jasonsaayman](https://github.com/jasonsaayman)

pinned this issue [on Apr 2, 2026](https://github.com/axios/axios/issues/10636#event-24151833270)

-

### RoseSecurity commented on Apr 2, 2026

https://github.com/RoseSecurity

[RoseSecurity](https://github.com/RoseSecurity)

[on Apr 2, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4178607972)

Thanks for your transparency throughout this. It's not fun, but glad to see the security posture improving as a result and I think these are really important lessons that we should take as an open source community.

Reacted by Thomas Broyer, Équipe Octopusmind, Kristopher Johnson, Gustavo Adrián Salvini, husheng, Hani Abdulfattah, Adityahh19, Diógenes Fernandes, Andrei Saliba, Eduardo Del Aguila and 110 more

-

### corneliusroemer commented on Apr 2, 2026

https://github.com/corneliusroemer

[corneliusroemer](https://github.com/corneliusroemer)

[on Apr 2, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4178882810)

Thanks for the writeup. Most of the stuff you mention here is already public knowledge. Would be great if you could add things that aren't publicly known yet. In particular, could you go into more detail on how they attacked you? This is still quite vague:

About 2 weeks before March 31: social engineering campaign initiated against the lead maintainer

How did they trick you? What were the red flags in retrospect?

This gave them access to the npm account credentials, which they used to publish the malicious versions.

How did they take over npm credentials? Didn't you have mandatory 2FA? Npm claims that all big packages have mandatory 2FA enabled. How did they circumvent that?

Reacted by Haz, Rossvold, Salvatore, Matthew Endicott, Ivan Nikolić, Adityahh19, m4jora, Diógenes Fernandes, Nathan, Avindra Goolcharan and 74 more

Reacted by Indy, mystixa, Emily Xiong and Sinan Postacı

-

### DanielRuf commented on Apr 2, 2026

https://github.com/DanielRuf

[DanielRuf](https://github.com/DanielRuf)

[on Apr 2, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4179080538)

How did they take over npm credentials? Didn't you have mandatory 2FA? Npm claims that all big packages have mandatory 2FA enabled. How did they circumvent that?

As far as I read in the original thread, they had an infostealer on their computer. Some company asked to work together and in the end it was one of the attackers posing as some other company trcking the maintainer into installing software.

Well, even the lastest of my followers seem to be such sock puppet accounts which try to create some fake identity with a huge amount of fake GitHub activity and then probably use that to trick others into beliebing to be some professional developer from some big company.

Currently there are also many job offers, where they ask you to install specific software as part of the job interview.

Reacted by Jay, codey-badger, Rana Jahanzaib, Baptiste Lombard, LanguageNest, adamus1red, Panagiotis Englezos, lavirdev, Dirk Thierbach, Abhishek Pathak and 15 more

Reacted by codey-badger, PCSD-dev, Marcos Eliziario Santos and Bruno Wolff

-

### feross commented on Apr 2, 2026

https://github.com/feross

[feross](https://github.com/feross)

[on Apr 2, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4179129970)

Thanks for the thorough post-mortem, Jason. Writing one of these is never easy, and the transparency here is exactly what the ecosystem needs.

A few things worth highlighting for others reading this:

The response was fast. ~3 hours from publish to removal is a good outcome given the constraints. [@DigitalBrainJS](https://github.com/DigitalBrainJS) in particular deserves recognition for acting decisively.

For maintainers of high-download packages, this incident reinforces a few things we'd recommend:

- OIDC-based publishing (provenance attestation) is the single highest-leverage change you can make. It removes long-lived credentials from the equation entirely. Glad to see this is on the roadmap.
- Publish from CI, not from a local machine. If a maintainer's laptop is compromised, the blast radius shouldn't include the ability to push a release.
- Enable npm's mandatory 2FA for publish, and use a hardware key if possible. TOTP on a compromised machine is not 2FA.

We detected the malicious axios@1.14.1 and axios@0.30.4 versions within minutes of publication and blocked it for users with the [AI Detected Potential Malware](https://socket.dev/alerts/gptMalware) alert enabled. Thanks for sharing our [analysis](https://socket.dev/blog/axios-npm-package-compromised).

This kind of targeted social engineering against individual maintainers is the new normal. It's not a reflection on Jason or the axios team -- these campaigns are sophisticated and persistent. We're seeing them across the ecosystem and they're only accelerating.

Wishing you and the team the best as you work through the remaining security work here. The npm ecosystem is better for having maintainers who respond like this.

Reacted by Berend de Boer, BMI, Shuqian Hon, Takashi Sato, Noah Allen, Yawar Amin, Frog Chen, Christian Kaindl, Abijah Kajabika, Sachin Joseph and 42 more

Reacted by Dmitriy Mozgovoy, Daniel Ruf, Shuqian Hon, Aseem Kishore, Scott Motte, Oliver Weiler, Nico Lynzaad, Frog Chen, Peter Thompson, Christian Kaindl and 42 more

-

### browner12 commented on Apr 2, 2026

https://github.com/browner12

[browner12](https://github.com/browner12)

[on Apr 2, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4179137457)

Remember to include your appropriate lock file in the grep command, as the current list is not exhaustive. For example,

```
grep -E "axios@(1\.14\.1|0\.30\.4)|plain-crypto-js" package-lock.json yarn.lock bun.lock pnpm-lock.yaml 2>/dev/null
```

or maybe even better, just check all lock files

```
grep -E "axios@(1\.14\.1|0\.30\.4)|plain-crypto-js" *.lock 2>/dev/null
```

Reacted by Tudor, Ionut Elian, Gilles Lepretre, Chuck Frey, Mayckon, Will, Nikolai Knopp, Seán O'Grady, Nicholas Ferreira Araujo, David K, isikyus and 3 more

-

### jasonsaayman commented on Apr 2, 2026

https://github.com/jasonsaayman

[jasonsaayman](https://github.com/jasonsaayman)

[on Apr 2, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4179144478)

MemberAuthor

to answer the NPM question. i have been told that once the RAT is on your machine they have full unilateral control of everything on your machine (you can google more about this). i therefore presume they could do anything once they have that. and yes i did have 2fa enabled on my account.

Reacted by Igal Klebanov, PCSD-dev, Nikhil Sharma, Goulven Champenois, Shuqian Hon, Vinicius de Santana, JoungMan Son, orz and Utku İmdat

-

### voxpelli commented on Apr 2, 2026

https://github.com/voxpelli

[voxpelli](https://github.com/voxpelli)

[on Apr 2, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4179392635)

I had a similar incident a few weeks back where someone invited me to a podcast a week later and then invited me into a group with others who would be interviewed in the same podcast episode and then got social media images and preparatory interview questions only to then when we were about to record the podcast they faked a connection issue on a fake version of a streaming website to try to get me to install a non-notarized native app on my Mac (which I later identified as having an info stealer).

Thankfully I was paranoid and didn’t run it, but it’s creepy how they target you, no matter if they are real people or possibly AI. (When it became clear that I wouldn’t run the app and we had chatted back and forth on website and chat app they made one final desperate attempt and tried to get me to run a curl command that would download and run something, then when I refused they went dark and deleted all conversations)

My guess is that they wanted access to Mocha or such.

Reacted by Daniel Ruf, Kristopher Johnson, Éanna Glavin, Keith Avery, Steven20230315, Aaron, Ng Han Seng, CJ, Yukotan, DevFrancois and 37 more

Reacted by Igal Klebanov, Aaron, Abhinav Ramkumar, Dyllon Gunawardhana, utkarsh-ginco, Samuele Piazzesi, Edwin Takahashi, Équipe Octopusmind, Alan Fregtman and Ronny Orbach

Reacted by codey-badger, Tudor Zaharia, Mikan Harada, Luis Marsiglia, Georg, Alexander Kachkaev, Adarsh Hatwar, MilaZhou22, Richard Metzler, Nils P. Ellingsen and 11 more

-

### bcomnes commented on Apr 2, 2026

https://github.com/bcomnes

[bcomnes](https://github.com/bcomnes)

[on Apr 2, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4179657668)

[@jasonsaayman](https://github.com/jasonsaayman) It would be helpful if you are willing, to share more of the social engineering playbook that they hit you with. Often its re-used even if its highly targeted at an individual.

By sharing more of the social engineering details you can help damage any construct infrastructure they used, strategies and tools used, and generally help people in the future with pattern recognition when they are being targeted.

Reacted by m4jora, Avindra Goolcharan, Ahmad Nassri, Igal Klebanov, CJ, Amadej Papež, PCSD-dev, Teodor Potančok, Curt Grimes, Georg and 31 more

-

### ahmadnassri commented on Apr 2, 2026

https://github.com/ahmadnassri

[ahmadnassri](https://github.com/ahmadnassri)

[on Apr 2, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4180081187)

thanks for the write up [@jasonsaayman](https://github.com/jasonsaayman) 👏

beyond the advise listed, worth highlighting for folks:

- npx invocations produce a non-deterministic depedncy resolution at run-time, no package-lock trace. (even if you're not using @latest)
- if your using npx in your CI / MCP with a package that directly or indirectly uses axios with semver range that could have resolved to v1.14.1 / v.0.30.4 during the ~3 hour window, you have likely downloaded plain-crypto-js

- sadly, most CI & MCP tooling setup docs, always advise to simply run: npx <package>@latest

- check the network logs in your CI during those times.
- npx call hardening with pre-install + lock file generation + using --no --offline

- full technical details here: https://socket.dev/blog/hidden-blast-radius-of-the-axios-compromise

Reacted by Carl Bergenhem, Aseem Kishore, miraclebakelaser, omensan, Feross Aboukhadijeh, Frog Chen, Will, Vicki McKay, Matthew, isikyus and 1 more

Reacted by codey-badger, William Oliveira, Stephen Eisenhauer and Paulo Avelar

-

### jasonsaayman commented on Apr 2, 2026

https://github.com/jasonsaayman

[jasonsaayman](https://github.com/jasonsaayman)

[on Apr 2, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4180237789)

MemberAuthor

[@bcomnes](https://github.com/bcomnes) yeah sure. i understand and will share how they do the attack.

so the attack vector mimics what google has documented here: https://cloud.google.com/blog/topics/threat-intelligence/unc1069-targets-cryptocurrency-ai-social-engineering

they tailored this process specifically to me by doing the following:

- they reached out masquerading as the founder of a company they had cloned the companys founders likeness as well as the company itself.
- they then invited me to a real slack workspace. this workspace was branded to the companies ci and named in a plausible manner. the slack was thought out very well, they had channels where they were sharing linked-in posts, the linked in posts i presume just went to the real companys account but it was super convincing etc. they even had what i presume were fake profiles of the team of the company but also number of other oss maintainers.
- they scheduled a meeting with me to connect. the meeting was on ms teams. the meeting had what seemed to be a group of people that were involved.
- the meeting said something on my system was out of date. i installed the missing item as i presumed it was something to do with teams, and this was the RAT.
- everything was extremely well co-ordinated looked legit and was done in a professional manner.

Reacted by Charlie Blundell, 川口洋, omensan, Ryan A. Chapman, Romio Joseph, belowfrog, igk-user, Michael Schroeder, AnthonyHerman, Domi and 138 more

Reacted by Victor Garcia, Alexander Olmedo, psyduck, eyoab-t, Ryo Katsuse, Menelaos Kotoglou, Brian A. Teller, Kuku and zaqes

Reacted by Matheus Souza, Sylv, TheodorV, Sedky Haider, Nahuel Nucera, Syed Ahsan, Rafa Benitez, Wilmer Blanco, A S Gowri Sankar, Billa and 7 more

Reacted by Lukas, Sedky Haider, Paul Thompson, Hunain Hyder and Dries Grobler Jr

Reacted by keiich, YurasovDV, Khaled, Sedky Haider, Simon Willison, Bas Zalmstra, Cuong Vu, Indrajeet, Brian Levine, William Oliveira and 21 more

Reacted by Michael Irwin, Bret Comnes, Andrei Saliba, Patrick Nelson, RoseSecurity, Hasan Sattar, Kyle Adams, Theo Han, Harutaka Kawamura, Keith Avery and 393 more

Reacted by CannibalKush, yyovil , Sedky Haider, Bibin P Binu, Jonathan Leaders, Anjum Rashid, jeffersonsitepassport, Mayon Francis, Brian A. Teller, Nathan Trujillo and 2 more

Reacted by Michael Irwin, andoshin11, Aaron, Tomochika Hara, Shunsuke Mano, anon, appBana, peko, ShutoYamada, Kosei Kitahara and 256 more

-

### bcomnes commented on Apr 2, 2026

https://github.com/bcomnes

[bcomnes](https://github.com/bcomnes)

[on Apr 2, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4180450898)

[@jasonsaayman](https://github.com/jasonsaayman) thank you!

If there are specific personas/accounts/contacts that are still active that were used in the attack, my suggestion would be to archive as much as possible and then publicly identify the personas/constructs, even if its a construct of a real person (clarify the attacker != the person used) otherwise the attacker can continue to use the account as long as they think they can derive value from it.

I don't know what kind of constraints you are under though so if you do not I assume you have good reason not to share.

Reacted by eyoab-t, Samantha Otero, Matthew, Dries Grobler Jr, orz and John-David Dalton

-

### ashishb commented on Apr 3, 2026

https://github.com/ashishb

[ashishb](https://github.com/ashishb)

[on Apr 3, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4181485056)

I would say you should recommend your users to run npm and npx inside a [sandbox](https://github.com/ashishb/amazing-sandbox) as a future defense-in-depth strategy.

Reacted by Luke McQuade, Radosław Grochowski, sensor, RoseSecurity, JeremiTC, Vlad Rusu and Antonio Salvati

Reacted by Hadi Alqattan, Radosław Grochowski, みじっこ, Stephen Eisenhauer and Antonio Salvati

-

### 94 remaining items

-

[RevealUIStudio](https://github.com/RevealUIStudio)

added a commit that references this issue [on Apr 14, 2026](https://github.com/axios/axios/issues/10636#event-24474651114)

[chore(deps): tighten axios/follow-redirects overrides + drop stale lo…](https://github.com/RevealUIStudio/revealui/commit/19efd8f3134907b604b300343845b417a3a2f073)

[19efd8f](https://github.com/RevealUIStudio/revealui/commit/19efd8f3134907b604b300343845b417a3a2f073)

-

[lazka](https://github.com/lazka)

added a commit that references this issue [on Apr 16, 2026](https://github.com/axios/axios/issues/10636#event-24560625701)

[chore: unlock axios again](https://github.com/lazka/typesense-js/commit/753014b6d8d6166ca5db4aa7a204b36d1b9cf911)

[753014b](https://github.com/lazka/typesense-js/commit/753014b6d8d6166ca5db4aa7a204b36d1b9cf911)

-

### jasonsaayman commented on Apr 18, 2026

https://github.com/jasonsaayman

[jasonsaayman](https://github.com/jasonsaayman)

[on Apr 18, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4273727493)

MemberAuthor

Since no comments have been made in about a week, I am going to close this issue. If anyone would like me to reopen this, I am happy to do so. Just reach out to me.

Reacted by mdodge-ecgrow and RoseSecurity

-

[jasonsaayman](https://github.com/jasonsaayman)

closed this as [completed](https://github.com/axios/axios/issues?q=is%3Aissue%20state%3Aclosed%20archived%3Afalse%20reason%3Acompleted)[on Apr 18, 2026](https://github.com/axios/axios/issues/10636#event-24637358879)

-

### MuyembeII commented on Apr 20, 2026

https://github.com/MuyembeII

[MuyembeII](https://github.com/MuyembeII)

[on Apr 20, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4278335815)

Thank you Jason! Keep going 💪🏾

-

### DigitalBrainJS commented on Apr 21, 2026

https://github.com/DigitalBrainJS

[DigitalBrainJS](https://github.com/DigitalBrainJS)

[on Apr 21, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4285202916)

Collaborator

[@asrarmared-ship-it](https://github.com/asrarmared-ship-it) We would greatly appreciate it if you could refrain from posting lengthy messages. If you want to share something related to the topic, use external links.

-

### daiyam commented on May 1, 2026

https://github.com/daiyam

[daiyam](https://github.com/daiyam)

[on May 1, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4360412320)

[@MistyPigeon](https://github.com/MistyPigeon) Man, it isn't by pointing finger at other that our community can learn from their mistake.

Take for example in aviation:

Aviation risk management explicitly rejects blaming individuals for human error, viewing accidents as evidence of system failure rather than individual negligence.

It allows the incidents to be publicly reported and the general security to be improved.

I do think, IT should do the same.
In this case, I did learned and changed lot of my processes to avoid any bubble-effect if I got hack. I'm not a god and I can't claim I won't make a mistake.

Reacted by Daniel Ruf and Steve Dodson

-

### DanielRuf commented on May 1, 2026

https://github.com/DanielRuf

[DanielRuf](https://github.com/DanielRuf)

[on May 1, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4360713970)

I do think, IT should do the same.

It's called Blameless Post Mortem. And yes, that should be done by more.
Most of us are grateful, that we received some details and defenders can work with that information to protect projects and organizations better.

This was a targeted attack by a nationstate threat actor.
Everything we can learn from that, helps protecting all of us.

Mistakes happen and social engineering attacks are unfortunately very efficient.
Even NVIDIA was hacked by some young people with social engineering tricks.

So what can be learned from that:

- add some manual stop-gaps
- check and improve processes (if you did not yet set ignore-scripts=true, now is a good time)
- reduce the amount of dependencies as far as possible to reduce your attack surface
- learn from other cases (https://codeberg.org/DanielRuf/supply-chain-incidents, [lessons learned from the bitwarden/cli case BerriAI/litellm-security-wg#13](https://github.com/BerriAI/litellm-security-wg/issues/13) lists a few learnings around compromised VSCode extensions)
- check and harden your dev setup
- use proper solutions for secrets management
- adopt short-lived tokens where possible
- ...

Reacted by Baptiste Augrain, Stephanie Weiss and sadakura-dot

-

### DanielRuf commented on May 1, 2026

https://github.com/DanielRuf

[DanielRuf](https://github.com/DanielRuf)

[on May 1, 2026](https://github.com/axios/axios/issues/10636#issuecomment-4360761852)

To simply answer [@MistyPigeon](https://github.com/MistyPigeon)'s question: see ClickFix attacks.

-

[jeffs](https://github.com/jeffs)

added a commit that references this issue [on May 4, 2026](https://github.com/axios/axios/issues/10636#event-25134521764)

[Add npmrc with ignore-scripts=true](https://github.com/jeffs/conf/commit/850afc745fa445137d411880551ce231dc02d728)

[850afc7](https://github.com/jeffs/conf/commit/850afc745fa445137d411880551ce231dc02d728)

-

[lsetiawan](https://github.com/lsetiawan)

added a commit that references this issue [on May 14, 2026](https://github.com/axios/axios/issues/10636#event-25506843308)

[Restructure and update Security in the Age of AI deck ([#7](https://github.com/uw-ssec/presentations/pull/7))](https://github.com/uw-ssec/presentations/commit/4d78acc5b027acfd67457d8d41d756568d2c0697)

[4d78acc](https://github.com/uw-ssec/presentations/commit/4d78acc5b027acfd67457d8d41d756568d2c0697)

-

[jeffs](https://github.com/jeffs)

added a commit that references this issue [on Jul 11, 2026](https://github.com/axios/axios/issues/10636#event-27860519402)

[Add npmrc with ignore-scripts=true](https://github.com/jeffs/conf/commit/c0ca4647d8b59bd1a6c886bf312a3021f8a22d00)

[c0ca464](https://github.com/jeffs/conf/commit/c0ca4647d8b59bd1a6c886bf312a3021f8a22d00)

-

### sbedell commented on Jul 31, 2026

https://github.com/sbedell

[sbedell](https://github.com/sbedell)

[on Jul 31, 2026](https://github.com/axios/axios/issues/10636#issuecomment-5146598837)

[@jasonsaayman](https://github.com/jasonsaayman) I don't get why you'd join a random Slack workspace and then jumped on a call with them? What was the compelling reason to do any of that in the first place? Why weren't you skeptical to run the commands like [@voxpelli](https://github.com/voxpelli) was?

Reacted by Steve

Reacted by hackerman and Jordan Harband

Reacted by John-David Dalton

Reacted by Nathan Trujillo

-

[ttraenkler](https://github.com/ttraenkler)

added a commit that references this issue [on Aug 9, 2026](https://github.com/axios/axios/issues/10636#event-29171752883)

[fix(website): point issue links at the markdown issues, not GitHub is…](https://github.com/loopdive/js2/commit/78dbdd4a72db830553052a2749ca52e76b270051)

[78dbdd4](https://github.com/loopdive/js2/commit/78dbdd4a72db830553052a2749ca52e76b270051)

-

[devdacian](https://github.com/devdacian)

added a commit that references this issue [on Oct 22, 2026](https://github.com/axios/axios/issues/10636#event-33940763184)

[feat: accept Yarn Berry lockfile format 10; list the axios compromise](https://github.com/devdacian/coldclone/commit/d36b70213883016bb726e51b18fa7035abc53250)

[d36b702](https://github.com/devdacian/coldclone/commit/d36b70213883016bb726e51b18fa7035abc53250)

[Sign up for free](https://github.com/signup?return_to=https://github.com/axios/axios/issues/10636) to join this conversation on GitHub. Already have an account? [Sign in to comment](https://github.com/login?return_to=https://github.com/axios/axios/issues/10636)

## Metadata

## Metadata

### Assignees

- [jasonsaayman](https://github.com/jasonsaayman)

### Labels

[type::securityThe PR is a secuirty related changed normally from a CVE](https://github.com/axios/axios/issues?q=state%3Aopen%20label%3A%22type%3A%3Asecurity%22)

### Type

No type

### Projects

No projects

### Milestone

No milestone

### Relationships

None yet

### Development

No branches or pull requests

## Issue actions

-
