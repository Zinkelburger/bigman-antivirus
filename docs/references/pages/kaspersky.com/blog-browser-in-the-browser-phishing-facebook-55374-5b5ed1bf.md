---
url: "https://www.kaspersky.com/blog/browser-in-the-browser-phishing-facebook/55374/"
title: "What a browser-in-the-browser attack is, and how to spot a fake login window"
site: "Kaspersky official blog"
captured: "2026-11-02T18:56:09Z"
via: "live"
sha256: "07f8126856e545e5ffc180c19524cef95106fbdd1ff47d82b488f873c972654f"
wayback: "https://web.archive.org/web/20261004075620/https://www.kaspersky.com/blog/browser-in-the-browser-phishing-facebook/55374/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# What a browser-in-the-browser attack is, and how to spot a fake login window

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.kaspersky.com/blog/browser-in-the-browser-phishing-facebook/55374/>
> Snapshot: <https://web.archive.org/web/20261004075620/https://www.kaspersky.com/blog/browser-in-the-browser-phishing-facebook/55374/>

- [phishing](https://www.kaspersky.com/blog/tag/phishing/)

# Browser-in-the-browser attacks: from theory to reality

A browser-in-the-browser attack, theoretically described in 2022, has been adopted in real-world phishing. We break down how it works, and how to spot a fake authentication window.

-

https://www.kaspersky.com/blog/author/alannatitterington/

[Alanna Titterington](https://www.kaspersky.com/blog/author/alannatitterington/)

-

March 4, 2026

In 2022, we dived deep into an attack method called [browser-in-the-browser](https://www.kaspersky.com/blog/browser-in-the-browser-attack/44163/) — originally developed by the cybersecurity researcher known as mr.d0x. Back then, no actual examples existed of this model being used in the wild. Fast-forward four years, and browser-in-the-browser attacks have graduated from the theoretical to the real: attackers are now using them in the field. In this post, we revisit what exactly a browser-in-the-browser attack is, show how hackers are deploying it, and, most importantly, explain how to keep yourself from becoming its next victim.

## What is a browser-in-the-browser (BitB) attack?

For starters, let’s refresh our memories on what [mr.d0x actually cooked up](https://mrd0x.com/browser-in-the-browser-phishing-attack/). The core of the attack stems from his observation of just how advanced modern web development tools — HTML, CSS, JavaScript, and the like — have become. It’s this realization that inspired the researcher to come up with a particularly elaborate phishing model.

A browser-in-the-browser attack is a sophisticated form of phishing that uses web design to craft fraudulent websites imitating login windows for well-known services like Microsoft, Google, Facebook, or Apple that look just like the real thing. The researcher’s concept involves an attacker building a legitimate-looking site to lure in victims. Once there, users can’t leave comments or make purchases unless they “sign in” first.

Signing in seems easy enough: just click the Sign in with {popular service name} button. And this is where things get interesting: instead of a genuine authentication page provided by the legitimate service, the user gets a fake form rendered inside the malicious site, looking exactly like… a browser pop-up. Furthermore, the address bar in the pop-up, also rendered by the attackers, displays a perfectly legitimate URL. Even a close inspection won’t reveal the trick.

From there, the unsuspecting user enters their credentials for Microsoft, Google, Facebook, or Apple into this rendered window, and those details go straight to the cybercriminals. For a while this scheme remained a theoretical experiment by the security researcher. Now — real-world attackers have added it to their arsenals.

## Facebook credential theft

Attackers have put their own spin on mr.d0x’s original concept: recent browser-in-the-browser hits [have been kicking off with emails](https://www.bleepingcomputer.com/news/security/facebook-login-thieves-now-using-browser-in-browser-trick/) designed to alarm recipients. For instance, one phishing campaign posed as a law firm informing the user they’d committed a copyright violation by posting something on Facebook. The message included a credible-looking link allegedly to the offending post.

https://media.kasperskydaily.com/wp-content/uploads/sites/92/2026/03/04090404/browser-in-the-browser-phishing-facebook-1.jpg

Attackers sent messages on behalf of a fake law firm alleging copyright infringement — complete with a link supposedly to the problematic Facebook post. [Source](https://www.bleepingcomputer.com/news/security/facebook-login-thieves-now-using-browser-in-browser-trick/)

Interestingly, to lower the victim’s guard, clicking the link didn’t immediately open a fake Facebook login page. Instead, they were first greeted by a bogus Meta CAPTCHA. Only after passing it was the victim presented with the fake authentication pop-up.

https://media.kasperskydaily.com/wp-content/uploads/sites/92/2026/03/04090511/browser-in-the-browser-phishing-facebook-2.jpg

This isn’t a real browser pop-up; it’s a website element mimicking a Facebook login page — a ruse that allows attackers to display a perfectly convincing address. [Source](https://www.bleepingcomputer.com/news/security/facebook-login-thieves-now-using-browser-in-browser-trick/)

Naturally, the fake Facebook login page followed mr.d0x’s blueprint: it was built entirely with web design tools to harvest the victim’s credentials. Meanwhile, the URL displayed in the forged address bar pointed to the real Facebook site — www.facebook.com.

## How to avoid becoming a victim

The fact that scammers are now deploying browser-in-the-browser attacks just goes to show that their bag of tricks is constantly evolving. But don’t despair — there’s a way to tell if a login window is legit. A [password manager](https://www.kaspersky.com/password-manager?icid=gl_kdailyplacehold_acq_ona_smm__onl_b2c_kasperskydaily_wpplaceholder____kpm___) is your friend here, which, among other things, acts as a reliable security litmus test for any website.

That’s because when it comes to auto-filling credentials, a password manager looks at the actual URL, not what the address bar appears to show, or what the page itself looks like. Unlike a human user, a [password manager](https://www.kaspersky.com/password-manager?icid=gl_kdailyplacehold_acq_ona_smm__onl_b2c_kasperskydaily_wpplaceholder____kpm___) can’t be fooled with browser-in-the-browser tactics, or any other tricks, like domains having a slightly different address (typosquatting) or phishing forms buried in ads and pop-ups. There’s a simple rule: if your password manager offers to auto-fill your login and password, you’re on a website you’ve previously saved credentials for. If it stays silent, something’s fishy.

Beyond that, following our time-tested advice will help you defend against various phishing methods, or at least minimize the fallout if an attack succeeds:

- Enable two-factor authentication (2FA) for every account that supports it. Ideally, use one-time codes [generated by a dedicated authenticator app](https://www.kaspersky.com/blog/kaspersky-password-manager-authenticator/48841/) as your second factor. This helps you dodge phishing schemes designed to intercept confirmation codes sent via SMS, messaging apps, or email. You can read more about one-time-code 2FA in [our dedicated post](https://www.kaspersky.com/blog/authenticator-apps-and-security/47426/).
- Use passkeys. The option to sign in with this method can also serve as a signal that you’re on a legitimate site. You can learn all about what passkeys are and how to start using them in our [deep dive into the technology](https://www.kaspersky.com/blog/full-guide-to-passkeys-in-2025-part-1/53688/).
- Set unique, complex passwords for all your accounts. Whatever you do, never reuse the same password across different accounts. We recently covered what makes a password truly strong [on our blog](https://www.kaspersky.com/blog/international-password-day-2025/53355/). To generate unique combinations — without needing to remember them — [Kaspersky Password Manager](https://www.kaspersky.com/password-manager?icid=gl_kdailyplacehold_acq_ona_smm__onl_b2c_kasperskydaily_wpplaceholder____kpm___) is your best bet. As an added bonus, it can also generate one-time codes for two-factor authentication, store your passkeys, and synchronize your passwords and files across your various devices.

Finally, this post serves as yet another reminder that theoretical attacks described by cybersecurity researchers often find their way out into the wild. So, keep an eye on our blog, and [subscribe to our Telegram channel](https://t.me/+hfDEDRUTiLJlOGE8) to stay up to speed on the latest threats to your digital security and how to shut them down.

Read about other inventive phishing techniques scammers are using day in day out:

- [Phishing and spam: the wildest campaigns of 2025](https://www.kaspersky.com/blog/spam-and-phishing-2025/55295/)
- [What happens to data stolen using phishing?](https://www.kaspersky.com/blog/what-happens-to-data-after-phishing/54968/)
- [How phishers and scammers use AI](https://www.kaspersky.com/blog/ai-phishing-and-scams/54445/)
- [Turnkey phishing](https://www.kaspersky.com/blog/turnkey-phishing/51614/)
- [Progressive phishing: how PWAs can be used to steal passwords](https://www.kaspersky.com/blog/phishing-with-progressive-web-apps/51496/)

https://www.kaspersky.com/blog/cws-update-2026/55368/

- [Read next](https://www.kaspersky.com/blog/cws-update-2026/55368/)

### [Another look inside the container](https://www.kaspersky.com/blog/cws-update-2026/55368/)

The latest update to Kaspersky Cloud Workload Security adds support for an AI assistant for image analysis.

https://www.kaspersky.com/blog/cws-update-2026/55368/

- [Tips](https://www.kaspersky.com/blog/tag/tips/)

### [Reading between the lines: how to read e-books safely](https://www.kaspersky.com/blog/how-to-download-and-read-ebooks-safely/56625/)

We break down how to use e-books safely, and which rules to follow when searching for books online, so you don’t fall victim to scammers

- [Tips](https://www.kaspersky.com/blog/tag/tips/)

### [The eMudhra certificate incident: how organizations can respond](https://www.kaspersky.com/blog/emudhra-tls-revocation-enterprise-risks-mitigations/56631/)

How a certificate’s private key ended up being equal to 4, and why browser updates alone won’t protect you from vulnerable eMudhra certificates.

- [Tips](https://www.kaspersky.com/blog/tag/tips/)

### [How to improve your resume with AI without exposing your data](https://www.kaspersky.com/blog/creating-resume-with-ai-safely/56563/)

Let’s look at how to use AI as a resume editor without oversharing to build a document that gets past automated screening systems and catches a recruiter’s eye.

- [Tips](https://www.kaspersky.com/blog/tag/tips/)

### [How can students overcome the fear of unfulfilled potential, and where does cybersecurity come into it?](https://www.kaspersky.com/blog/tech-students-future/56475/)

University students sometimes don’t see the practical value of what they’re studying, and aren’t always sure what to do next. Kaspersky Academy’s free “Cybersecurity. Entry Level” course can help them pick up in-demand skills directly from Kaspersky experts.
