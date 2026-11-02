---
url: "https://www.ledger.com/blog/security-incident-report"
title: "Security Incident Report | Ledger"
site: "Ledger"
published: "2023-12-20T12:46:36+00:00"
captured: "2026-11-02T18:57:06Z"
via: "live"
sha256: "61b77bcefb6977ed71b292f360bec410ade0c844147265aebc70d49fb21e4559"
wayback: "https://web.archive.org/web/20261026014038/https://www.ledger.com/blog/security-incident-report"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Security Incident Report | Ledger

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.ledger.com/blog/security-incident-report>
> Snapshot: <https://web.archive.org/web/20261026014038/https://www.ledger.com/blog/security-incident-report>

[Blog](https://www.ledger.com/blog/security-incident-report)  [Blog posts](https://www.ledger.com/category/blog-posts)

[Tech](https://www.ledger.com/blog/category/tech) | 12/20/2023

# Security Incident Report

[Blog](https://www.ledger.com/blog)  [Tech](https://www.ledger.com/blog/category/tech)

Date of the report: 2023-12-20

Date of the incident: 2023-12-14

Type of Incident Detected: Unauthorized Access & Malicious Code

##### Executive Summary

Ledger detected an exploit using Ledger Connect Kit on Thursday the 14th of December 2023. This exploit injected malicious code inside DApps that were using Ledger Connect Kit, tricking EVM DApp users into signing transactions that drain their signers. The exploit was quickly spotted and a resolution was implemented briefly after. In the meantime, a low volume of users fell into the attack and signed transactions draining their signer.

##### Timeline

Timeline hours are detailed using the time zone Central European Time (CET):

2023-12-14: Morning: A former Ledger Employee fell victim to a sophisticated phishing attack that gained access to their NPMJS account, bypassing 2FA, using the individual’s session token.

2023-12-14 – 09:49AM / 10:44AM / 11:37AM: The attacker published on NPMJS (a package manager for Javascript code shared between apps), a malicious version of the Ledger Connect Kit (affecting versions 1.1.5, 1.1.6, and 1.1.7). The malicious code used a rogue WalletConnect project to reroute assets to hackers’ wallets.

2023-12-14: 1.45PM: Ledger was made aware of the ongoing attack thanks to the prompt reaction of different actors in the ecosystem, including Blockaid who reached out to the Ledger team and shared updates on X.

2023-12-14: 2.18PM: Ledger’s technology and security teams were alerted to the attack and a genuine version of Ledger Connect Kit fix was deployed by Ledger teams within 40 minutes of Ledger becoming aware. Due to the nature of CDN (Content Delivery Network) and caching mechanisms on the Internet, the malicious file remained accessible for a little longer. From the compromission of NPMJS to the complete resolution, approximately 5 hours have passed. This extended availability of the malicious code was a result of the time taken for the CDN to propagate and update its caches globally with the latest, genuine version of the file. Despite the file’s five hour presence, we estimate from our investigation that the window during which user assets were actively drained was confined to less than two hours in total.

Ledger coordinated swiftly with our partner WalletConnect, who disabled the rogue WalletConnect instance used to drain assets from the users.

2023-12-14: 2.55 PM Upon our coordination, Tether froze the USDT of the attacker(s) (cf. [TX](https://etherscan.io/tx/0xf2b76233561ded9382612352eb4cd511e081b715b577c2e2236704af9e8050f4)).

##### Root Cause Analysis, Findings and Preventing Measures

###### Context

Ledger Connect-Kit is a Java Script open source library allowing developers to connect their DApps to Ledger hardware. It can be integrated using the Connect-Kit-loader component that allows a DApp to load the Connect-Kit at runtime from a CDN. This allows DApp developers to always have the most recent version of the Connect-Kit without the need to manually update package versions and release new builds. The CDN used by Ledger for the distribution is NPMJS. Most DApps have integrated the Connect-Kit using the mentioned Connect-Kit-loader.

In the Ledger Connect Kit exploit, the attacker did not at any time have access to any Ledger infrastructure, Ledger code repository, or to DApps themselves. The attacker was able to push a malicious code package within the CDN in place of the Connect-Kit itself. This malicious Connect-Kit code was then dynamically loaded by DApps who already integrate the Connect-Kit-loader.

The Ledger Connect Kit exploit highlights risks Ledger and the industry collectively face to protect users, and it is also a reminder that collectively we need to continue to raise the bar for security around DApps where users will engage in browser-based signing. It was Ledger’s service that was exploited this time, but in the future this could happen to another service or library.

##### Root cause

In order to be able to push the malicious code package on NPMJS, the attacker phished a former employee to leverage the individual’s access on NPMJS. The access of the former employee to Ledger’s systems (including Github, SSO based services, all internal Ledger tools, and external tools) were properly revoked, but unfortunately the former employees’ access to NPMJS was not properly revoked.

We can confirm this was an unfortunate isolated incident. Accesses to Ledger infrastructure by Ledger employees are automatically revoked during employee offboarding, however, due to how current technology services and tools currently operate globally, we cannot automatically revoke access to certain external tools (NPMJS included), and these must be handled manually with an employee offboarding checklist for each individual. Ledger has an existing and regularly updated offboarding procedure where we remove departing employees from all external tools. In this individual case, the access was not manually revoked on the NPMJS, which we regret, and are auditing with an external third party partner.

This was a sophisticated attack conducted by the attacker. Despite having enforced two-factor authentication (2FA) on the NPMJS targeted account, which would normally deter many attempts, the attacker circumvented this security measure by exploiting an API key associated with the former employee’s account.

This specific attack enabled the attacker to upload a new malicious version of the Ledger Connect Kit which contained what is referred to as the Angel Drainer malware. Angel Drainer is a malware as a service that is specifically designed to craft malicious transactions that are draining signers, also known as hardware wallets, when signed. It’s a complete infrastructure specialized on EVM chains that deploys smart contracts on demand and crafts tailored transactions in order to maximize damage.

Unfortunately, [NPMJS.com](http://npmjs.com/) does not allow multi-authorization or signature verification for automatic publishing. We’re working on adding ad hoc mechanisms enforcing further controls at the deployment stage.

##### Findings

This was a well prepared attack executed by experienced attacker(s). The phishing technique implemented did not focus on credentials, which is what we see in most Front-End attacks affecting the ecosystem, but instead the attacker worked directly on the session token.

The malware used was Angel Drainer, and the Ledger security team has seen in the past three months an increase of criminal activities using this malware (please refer to this published [Blockaid report](https://www.blockaid.io/post/attack-report-ledger-connect-kit)). We can also see on-chain that the funds stolen are being split: 85% to the exploiter and 15% to Angel Drainer, which could be seen as a malware as a service.

This Angel Drainer tricks users into signing different types of transactions depending on the type of asset it is currently targeting. For ERC20 and NFT tokens, it requests users to sign approval and permit messages. For native tokens, the drainer asks the user to sign either a fake “claim” transaction where the claim method simply sweeps the funds, or simple token transfers that can be later sweeped by deploying a smart contract at the corresponding address.

This is why we continue to encourage Clear Signing as an industry, so users can verify what they see on a trusted display on their Ledger signer™ .

##### Remedial actions

The Ledger security and technology teams, including the Ledger executive team, are currently reviewing and auditing all our access controls on Ledger internal and external tools and systems that we use.

Ledger will reinforce its policies when it comes to code review, deployment, distribution, and access controls, including adding all external tools to our maintenance and off-boarding checks. We’ll continue to generalize code signing when relevant. Additionally we are conducting recurrent internal audits to make sure this is properly implemented.

Ledger already organizes security training sessions, including phishing training. The internal security training program will also be reinforced at the start of 2024 for all employees in their respective departments. Ledger already organizes regular third party security assessments and will continue to prioritize these assessments.

In early 2024, a specific third party audit will be conducted focused on access control, code promotion and distribution.

In addition, we’ll reinforce our infrastructure monitoring and alerting systems to be able to detect and react even faster to future incidents..

Finally, we’ll double down on preventing Blind Signing, removing it as an option for Ledger users to ensure utmost security practices, and educate users on the potential impact of signing transactions without either a secure display or understanding what they are signing by not using Clear Signing.

We thank again our partners in the ecosystem for swiftly working with the Ledger teams on identifying and resolving the exploit.

https://www.ledger.com/blog-ledger-n3xt-research-competition

[Company](https://www.ledger.com/blog/category/company-news) | 10/29/2026

The N3XT Generation is Building the Future of Digital Ownership...

[Read more](https://www.ledger.com/blog-ledger-n3xt-research-competition)

https://www.ledger.com/blog-ledger-atom-technical-overview

[Product](https://www.ledger.com/blog/category/product-news) | 10/23/2026

A Technical Overview of Ledger Atom™

[Read more](https://www.ledger.com/blog-ledger-atom-technical-overview)

https://www.ledger.com/blog-ledger-spurs-microsite

[Company](https://www.ledger.com/blog/category/company-news) | 10/21/2026

Pound the Rock: The Ledger x Spurs Microsite Is Live...

[Read more](https://www.ledger.com/blog-ledger-spurs-microsite)
