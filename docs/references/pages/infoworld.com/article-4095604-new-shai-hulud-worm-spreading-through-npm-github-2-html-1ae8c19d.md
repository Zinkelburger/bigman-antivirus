---
url: "https://www.infoworld.com/article/4095604/new-shai-hulud-worm-spreading-through-npm-github-2.html"
title: "New Shai-Hulud worm spreading through npm, GitHub"
site: "InfoWorld"
captured: "2026-11-02T18:55:09Z"
via: "live"
sha256: "418e41b906d97d83dd067ab147691599d0c31b1ee5cc9afd11578a0a2376abff"
wayback: "https://web.archive.org/web/20251218034639/https://www.infoworld.com/article/4095604/new-shai-hulud-worm-spreading-through-npm-github-2.html"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# New Shai-Hulud worm spreading through npm, GitHub

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.infoworld.com/article/4095604/new-shai-hulud-worm-spreading-through-npm-github-2.html>
> Snapshot: <https://web.archive.org/web/20251218034639/https://www.infoworld.com/article/4095604/new-shai-hulud-worm-spreading-through-npm-github-2.html>

##  The latest version also executes malicious code during the preinstall phase, and is bigger and faster than the first wave, say researchers.

Credit: Inactive design - Shutterstock.com

A new version of the Shai-Hulud credentials-stealing self-propagating worm is expanding through the open npm registry, a threat that developers who download packages from the repository have to deal with immediately.

[Researchers at Wiz Inc. said Monday](https://www.wiz.io/blog/shai-hulud-2-0-ongoing-supply-chain-attack) that in the early stages of the campaign late last week, a thousand new GitHub repositories containing harvested victim data were being added every 30 minutes. [And researchers at JFrog identified 181 compromised packages.](https://research.jfrog.com/post/shai-hulud-the-second-coming/)

The current campaign introduces a new variant, which Wiz researchers dub Shai-Hulud 2.0, that executes malicious code during the preinstall phase, “significantly increasing potential exposure in build and runtime environments.”

The threat leverages compromised package maintainer accounts to publish trojanized versions of legitimate npm packages. Once installed, the malware exfiltrates developer and CI/CD secrets to GitHub repositories, and also inserts the malicious payload into all of the users’ available npm packages. Threat actors could also use the exfiltrated secrets to break into and install more malware in victims’ IT systems.

JFrog said this new variant generates randomized repository names for exfiltration, making it harder for security teams to hunt down and scrub the leaked secrets. JFrog also said the new payload contains new functionality, including privilege escalation, DNS hijacking, and the ability to delete data from the victim’s machine.

Multiple popular packages used by developers, including those from Zapier, ENS Domains, PostHog, and Postman, have been compromised.

[Researchers at ReversingLabs also noted](https://www.reversinglabs.com/blog/another-shai-hulud-npm-worm-is-spreading-heres-what-you-need-to-know)the list of compromised packages includes AsyncAPI related packages, including [@asyncapi/specs](https://secure.software/npm/packages/@asyncapi/specs), which has had more than 100 million lifetime downloads and an average of 1.4 million weekly downloads. This package in particular is also believed to be the ‘patient-zero’, or the first known infected package, for this wave of attack, the researchers added.

## Second wave is bigger and faster

Developers and security teams looking for indicators of compromise should note that the new variant adds two new payload files: setup_bun.js and bun_environment.js.

“The re-emergence of the worm indicates that this remains a current and serious threat to the npm ecosystem,” said [Johannes Ullrich](https://www.sans.org/profiles/dr-johannes-ullrich), dean of research at the SANS Institute. “CSOs must address this threat by monitoring the components used in their software and hardening their CI/CD pipelines to increase resilience in the event that malicious code is executed.”

Shai-Hulud [first emerged in September](https://www.csoonline.com/article/4058059/warning-hackers-have-inserted-credential-stealing-code-into-some-npm-libraries.html), revealed by the discovery that dozens of npm libraries, including a color library with over 2 million downloads a week, had been replaced with malicious versions.

The initial Shai-Hulud wave was already one of the most severe JavaScript supply-chain attacks Wiz has seen, [Merav Bar](https://www.linkedin.com/in/merav-bar-608351232/?originalSubdomain=il), a company threat researcher and co-author of the report told CSO. “This new wave is bigger and faster: more than 25,000 attacker-created repos across roughly 350 GitHub users, growing by about 1,000 repos every 30 minutes, with malware that steals developer and cloud credentials and runs in the preinstall phase, touching dev machines and CI/CD pipelines alike. That combination of scale, speed, and access makes it a high-impact campaign.”

## Assume compromise

If an individual had pulled any of the affected packages during the November 21–23 window, she said, they should assume their environment is exposed. Remedies include clearing the npm cache on their workstation, removing node_modules, reinstalling from clean versions, or pinning to versions published before the malicious releases, and rotating any tokens or secrets that were present (GitHub PATs, npm tokens, SSH keys, cloud credentials).

Enabling strong MFA on GitHub/npm and watching for unexpected new repos or workflow files in the developer’s personal account is also critical, she added.

Shai-Hulud’s second wave isn’t surprising, said [Brad LaPorte](https://www.linkedin.com/in/brad-laporte/), cybersecurity advisor at Morphisec. “The first attack showed how easily preinstall scripts could be weaponized. This wave proves what happens when those warnings aren’t acted on: larger scale, more destructive payloads, and automation that infects thousands of repositories in hours.”

## Recommendations for repositories

To stop attackers from easily uploading malicious packages, npm needs to tighten its publishing process, LaPorte said, including ensuring the identity of new accounts is verified before they are allowed to publish packages; implementing rate limits to prevent attackers from uploading multiple malicious packages in a short period and monitoring package maintainers for suspicious activity such as sudden spikes in publishing or packages with significant unexplained changes.

“From what we’ve observed in the Shai-Hulud incidents,” said Bar of Wiz, “the core issue isn’t npm specifically. It’s that open-source registries now function as high-impact distribution hubs, and attackers are taking advantage of how much trust developers place in them. In that context, the most important thing is continuing to strengthen the guardrails around how packages are published and updated. That includes making it harder for compromised maintainer accounts to push malicious versions, increasing visibility into unusual publishing behavior, and helping downstream users quickly understand when a package version may be unsafe. Those are ecosystem-level defenses rather than criticisms of any single registry, but they reflect the direction the entire open-source community will need to move as attacks like Shai-Hulud become more automated and far-reaching.”

Ensar Seker, CISO at SOCRadar, cautioned that Shai‑Hulud isn’t what he called typical package compromise. “It’s a worm embedded into the dev supply chain. It signals that attackers are shifting from targeting compiled binaries and runtime environments toward the very processes developers use to build and ship software. No organization should assume, ‘We don’t use npm, so we’re safe’, because even downstream dependencies or dev toolchains can become the launch pad.”

So far npm has focused on ensuring that package authors are properly authenticated and that packages are not altered after being published, said Ullrich of the SANS Institute. But, he added, this does not prevent a malicious actor from publishing malicious packages. Recently, npm further restricted the default access token lifetimes and started to revoke legacy ‘classic tokens,’ he acknowledged. “npm may need to implement some form of automated scanning for obvious malicious content, but it will be difficult to implement a meaningful solution.”

## Recommendations for security teams, developers

Wiz says security teams in organizations with application developer teams that use npm – and individual developers using npm and GitHub – should:

- clear each developer’s npm cache;
- pin dependencies to known clean versions or roll back to pre-November 21 builds;
- revoke and regenerate npm tokens, GitHub PATs, SSH keys and cloud provider credentials;
- enforce phishing-resistant multifactor authentication for developer and CI/CD (continuous integration/continuous delivery) accounts.

Within GitHub and CI/CD environments, they should search for newly-created repositories with ‘Shai-Hulud’ in the description, review unauthorized workflows or suspicious commits referencing hulud, and monitor for new npm publishes under their organization.

For long-term protection CSOs and IT leaders are urged to restrict or disable lifecycle scripts (postinstall, preinstall) in CI/CD environments, limit outbound network access from built systems to trusted domains only, and make sure developers use only short-lived, scoped automation login tokens.

[Version Control Systems](https://www.infoworld.com/version-control-systems/)[Software Development](https://www.infoworld.com/software-development/)[Vulnerabilities](https://www.infoworld.com/vulnerabilities/)[Security](https://www.infoworld.com/security/)
