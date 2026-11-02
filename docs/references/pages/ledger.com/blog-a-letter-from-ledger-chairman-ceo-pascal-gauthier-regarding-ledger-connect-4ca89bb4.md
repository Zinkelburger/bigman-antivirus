---
url: "https://www.ledger.com/blog/a-letter-from-ledger-chairman-ceo-pascal-gauthier-regarding-ledger-connect-kit-exploit"
title: "A letter from Ledger Chairman & CEO Pascal Gauthier Regarding Ledger Connect Kit Exploit | Ledger"
site: "Ledger"
published: "2023-12-14T19:02:03+00:00"
captured: "2026-11-02T18:57:04Z"
via: "live"
sha256: "7fcb676f2767202b7019add9d64d123c9638a9a78f77629203b7ad4148e98973"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# A letter from Ledger Chairman & CEO Pascal Gauthier Regarding Ledger Connect Kit Exploit | Ledger

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.ledger.com/blog/a-letter-from-ledger-chairman-ceo-pascal-gauthier-regarding-ledger-connect-kit-exploit>

[Blog](https://www.ledger.com/blog/a-letter-from-ledger-chairman-ceo-pascal-gauthier-regarding-ledger-connect-kit-exploit)  [Blog posts](https://www.ledger.com/category/blog-posts)

[Company](https://www.ledger.com/blog/category/company-news) | 12/14/2023

# A letter from Ledger Chairman & CEO Pascal Gauthier Regarding Ledger Connect Kit Exploit

[Blog](https://www.ledger.com/blog)  [Company](https://www.ledger.com/blog/category/company-news)

|
Things to know:

|
– December 14th, 2023, Ledger experienced an exploit on Ledger Connect Kit, a Javascript library to connect Web sites to wallets.

– The industry collaborated with Ledger to neutralize the exploit and try to freeze stolen funds very quickly – the exploit was effectively running for less than two hours.

– This exploit is currently being investigated, Ledger has filed complaints and will help affected individuals try to recover funds.

– This exploit did not and does not affect the integrity of Ledger hardware or Ledger Live.

– The exploit was limited to third party DApps which use the Ledger Connect Kit.

Hi everyone,

Today we experienced an exploit on the Ledger Connect Kit, a Javascript library that implements a button allowing users to connect their Ledger device to third party DApps (wallet-connected Web sites).

This exploit was the result of a former employee falling victim to a phishing attack, which allowed a bad actor to upload a malicious file to Ledger’s NPMJS (a package manager for Javascript code shared between apps).

We worked swiftly, alongside our partner WalletConnect, to address the exploit, updating the NPMJS to remove and deactivate the malicious code within 40 minutes of discovery. This is a good example of the industry working swiftly together to address security challenges.

Now, I’d like to address why this happened, how we will improve our security practices to mitigate this specific risk in the future, and share our recommendation to the industry, so we can be stronger together.

The standard practice at Ledger is that no single person can deploy code without review by multiple parties. We have strong access controls, internal reviews, and code multi-signatures when it comes to most parts of our development. This is the case in 99% of our internal systems. Any employee who leaves the company has their access revoked from every Ledger system.

This was an unfortunate isolated incident. It is a reminder that security is not static, and Ledger must continuously improve our security systems and processes. In this area, Ledger will implement stronger security controls, connecting our build pipeline that implements strict software supply chain security to the NPM distribution channel.

It is also a reminder that collectively we need to continue to raise the bar for security around DApps where users will engage in browser-based signing. It was Ledger’s service that was exploited this time, but in the future this could happen to another service or library.

At Ledger, we believe clear signing, as opposed to blind signing, will help mitigate these issues. If the user can see what they sign on a trusted display, unintentionally signing rogue transactions can be avoided.

Ledger devices are open platforms. Ethereum has a plugin system that allows DApps to implement clear signing, and DApps who would like to implement this protection for their users can learn how [on developer.ledger.com](https://developers.ledger.com/dev-journey/services-dapps). In the same way we saw the community come together today, we look forward to your help bringing clear signing to all DApps.

Ledger has engaged with authorities and is doing all we can to help as this investigation unfolds. Ledger will support affected users in helping to find this bad actor, bring them to justice, track the funds, and work with law enforcement to help recover stolen assets from the hacker. We deeply regret the events that unfolded today for affected individuals.

The situation is now under control and the threat has passed. We understand the panic this caused for the community and broader ecosystem.

A full timeline is available below so you can see how our teams and partners responded.

Merci,
Pascal Gauthier

Chairman & CEO

—–

Here is the timeline of what we know about the exploit at this moment:

This morning CET, a former Ledger Employee fell victim to a phishing attack that gained access to their NPMJS account. The attacker published a malicious version of the Ledger Connect Kit (affecting versions 1.1.5, 1.1.6, and 1.1.7). The malicious code used a rogue WalletConnect project to reroute funds to a hacker wallet. Ledger’s technology and security teams were alerted and a fix was deployed within 40 minutes of Ledger becoming aware. The malicious file was live for around 5 hours, however we believe the window where funds were drained was limited to a period of less than two hours. Ledger coordinated with WalletConnect who quickly disabled the the rogue project. The genuine and verified Ledger Connect Kit version 1.1.8 is now propagating and is safe to use.

For builders who are developing and interacting with the Ledger Connect Kit code: connect-kit development team on the NPM project are now read-only and can’t directly push the NPM package for safety reasons. We have internally rotated the secrets to publish on Ledger’s GitHub. Developers, please check again that you’re using the latest version, 1.1.8.

Ledger, along with WalletConnect and our partners, have reported the bad actor’s wallet address. The address is now visible on Chainalysis. Tether has frozen the bad actor’s USDT.

We remind users to always Clear Sign with your Ledger. What you see on the Ledger screen is what you actually sign. If you still need to blind sign, use an additional Ledger mint wallet or parse your transaction manually. We are actively talking with customers whose funds might have been affected, and working proactively to help those individuals at this time. We are also filing a complaint and working with law enforcement on the investigation to find the attacker. On top of that, we’re studying the exploit in order to avoid further attacks. We believe the attacker’s address where the funds were drained is here: 0x658729879fca881d9526480b82ae00efc54b5c2d

We would like to thank WalletConnect, Tether, Chainalysis, zachxbt, and the whole community that helped us and continue to help us quickly identify and solve this attack. Security will always prevail with the help of the whole ecosystem.

The Ledger team

Version française:

##### Un Message de Pascal Gauthier, Chairman & CEO de Ledger, sur l’exploitation du Ledger Connect Kit

|
Points clés:

|
– Le 14 décembre 2023, Ledger fait face à une exploitation du Ledger Connect Kit, une bibliothèque Javascript permettant de connecter des sites Web à des portefeuilles crypto.

– L’industrie a collaboré avec Ledger pour neutraliser l’exploitation et tenter de geler rapidement les fonds volés – l’exploitation a eu lieu durant moins de deux heures.

– Cette exploitation est actuellement en cours d’investigation. Ledger a déposé des plaintes et s’efforcera d’aider les personnes affectées à récupérer leurs fonds.

– Cette exploitation n’a pas affecté et n’affecte pas la sécurité des portefeuilles physiques Ledger ni de Ledger Live.

– L’exploitation était limitée aux applications décentralisées tierces qui utilisent le Ledger Connect Kit.

Bonjour à tous,

Aujourd’hui, nous avons été confrontés à une exploitation du Ledger Connect Kit, une bibliothèque Javascript intégrant un bouton permettant aux utilisateurs de connecter leur appareil Ledger à des applications décentralisées tierces.

Cette situation est liée au fait qu’un ancien employé a été victime d’une attaque de phishing, permettant à un acteur malveillant de télécharger un fichier malveillant sur le NPMJS de Ledger (un gestionnaire de packages pour le code Javascript partagé entre les applications).

Notre réaction a été rapide et coordonnée avec notre partenaire WalletConnect pour remédier à la situation. Dans les 40 minutes suivant son identification, nous avons mis à jour le NPMJS pour éliminer et désactiver le code malveillant. Cet épisode témoigne de l’efficacité de l’industrie travaillant de concert pour surmonter d’importants défis de sécurité.

Abordons maintenant les raisons de cet incident et expliquons comment nous renforcerons nos pratiques de sécurité pour atténuer ce risque spécifique à l’avenir. Nous partageons également nos recommandations avec l’industrie pour renforcer notre collaboration.

Chez Ledger, la norme de sécurité stipule qu’aucune personne seule ne peut déployer du code sans qu’il soit examiné par plusieurs parties, une pratique appliquée dans 99% de nos systèmes internes. De plus, tout employé quittant l’entreprise voit son accès révoqué de tous les systèmes Ledger.

Cet incident, malheureux et isolé, souligne que la sécurité est en constante évolution, nécessitant une amélioration continue de nos systèmes. Dans ce contexte, Ledger mettra en place des contrôles de sécurité renforcés, liant notre chaîne de construction qui applique une sécurité stricte de la chaîne d’approvisionnement au canal de distribution NPM.

Il est également rappelé que collectivement, nous devons élever les normes de sécurité autour des applications décentralisées (DApps) où les utilisateurs interagissent avec des signatures basées sur le navigateur. Bien que cette fois-ci ce soit le service de Ledger qui a été exploité, cela pourrait se produire à l’avenir avec un autre service ou bibliothèque.

Chez Ledger, nous croyons en l’efficacité du clear-signing par rapport au blind-signing pour atténuer ces problèmes. Si l’utilisateur peut voir ce qu’il signe sur un écran de confiance, la signature involontaire de transactions frauduleuses pourra toujours être évitée.

Les appareils Ledger sont des plates-formes ouvertes. Ethereum offre un système de plug-ins permettant aux DApps d’implémenter le clear-signing. Les développeurs intéressés peuvent en savoir plus [sur developer.ledger.com](https://developers.ledger.com/dev-journey/services-dapps). Tout comme nous avons vu la communauté se mobiliser aujourd’hui, nous attendons votre aide pour intégrer le clear-signing à toutes les DApps.

Ledger coopère avec les autorités et prend toutes les mesures possibles pour aider dans l’enquête en cours. Nous soutiendrons les utilisateurs affectés en identifiant l’acteur malveillant, en le traduisant en justice, en retrouvant les fonds et en collaborant avec les forces de l’ordre pour récupérer les actifs volés. Nous regrettons profondément les événements d’aujourd’hui pour les personnes touchées.

La situation est maintenant sous contrôle, la menace écartée. Nous comprenons l’inquiétude que cela a pu causer dans la communauté et l’écosystème. Vous trouverez ci-dessous une chronologie complète pour voir comment nos équipes et partenaires ont réagi.

Merci,

Pascal Gauthier

—–

Voici la chronologie de ce que nous savons sur l’exploitation à l’heure actuelle :

Ce matin, (CET), un ancien employé de Ledger a été victime d’une attaque de phishing qui a permis à un hacker d’accéder à son compte NPMJS. Le hacker a publié une version malveillante du Ledger Connect Kit (affectant les versions 1.1.5, 1.1.6 et 1.1.7). Le code malveillant utilisait un projet WalletConnect frauduleux pour rediriger les fonds vers le portefeuille d’un hacker. Les équipes de sécurité de Ledger ont été alertées, et une correction a été déployée dans les 40 minutes suivant la prise de conscience par Ledger. Le fichier malveillant était actif pendant environ 5 heures, mais nous pensons que la fenêtre pendant laquelle les fonds ont été détournés était limitée à moins de deux heures. Ledger a collaboré avec WalletConnect, qui a rapidement désactivé le projet frauduleux. La version authentique et vérifiée du Ledger Connect Kit, la version 1.1.8, est désormais en propagation et peut être utilisée en toute sécurité.

Pour les développeurs qui travaillent sur le code du Ledger Connect Kit : l’équipe de développement du connect-kit sur le projet NPM est désormais en lecture seule et ne peut pas pousser directement le package NPM par mesure de sécurité. Nous avons changé les secrets pour la publication sur GitHub de Ledger. Pour les développeurs: veuillez vérifier à nouveau que vous utilisez la dernière version, la 1.1.8.

Ledger, en collaboration avec WalletConnect et nos partenaires, a signalé l’adresse du portefeuille du malfaiteur. L’adresse est désormais visible sur Chainalysis. Tether a gelé les USDT du malfaiteur.

Nous rappelons aux utilisateurs de toujours “clear sign” vos transactions avec votre Ledger. Ce que vous voyez sur l’écran de Ledger est ce que vous signez réellement. Si vous devez toujours signer de manière aveugle, utilisez un portefeuille mint Ledger supplémentaire ou analysez votre transaction manuellement. Nous sommes en contact actif avec les clients dont les fonds pourraient avoir été affectés et travaillons de manière proactive pour les aider en ce moment. Nous déposons également une plainte et collaborons avec les forces de l’ordre dans le cadre de l’enquête pour retrouver l’attaquant. De plus, nous étudions l’exploitation afin d’éviter de futures attaques. Nous pensons que l’adresse de l’attaquant où les fonds ont été détournés est la suivante : 0x658729879fca881d9526480b82ae00efc54b5c2d

Nous tenons à remercier WalletConnect, Tether, Chainalysis, zachxbt et toute la communauté qui nous ont aidés et continuent de nous aider à identifier rapidement et à résoudre cette attaque. La sécurité prévaudra toujours avec l’aide de l’ensemble de l’écosystème.

L’équipe de Ledger

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
