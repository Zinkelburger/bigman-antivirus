---
url: "https://groups.google.com/a/chromium.org/g/blink-dev/c/GbVcuwg_QjM"
title: "Intent to Deprecate and Remove: Top-frame navigations to data URLs"
captured: "2026-11-02T18:32:29Z"
via: "live"
sha256: "bd46335c66dbd9ecb617a8361dc434b52f369900f5a2a9c74bff9f04b22a6159"
wayback: "https://web.archive.org/web/20260513224617/https://groups.google.com/a/chromium.org/g/blink-dev/c/GbVcuwg_QjM"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# Intent to Deprecate and Remove: Top-frame navigations to data URLs

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://groups.google.com/a/chromium.org/g/blink-dev/c/GbVcuwg_QjM>
> Snapshot: <https://web.archive.org/web/20260513224617/https://groups.google.com/a/chromium.org/g/blink-dev/c/GbVcuwg_QjM>

[Groups](https://groups.google.com/a/chromium.org/g/blink-dev/c/my-groups)

Groups

https://www.google.com/intl/en/about/products?tab=gh

[Sign in](https://accounts.google.com/ServiceLogin?hl=en&passive=true&continue=https://groups.google.com&ec=GAZA0AM)

[Groups](https://groups.google.com/a/chromium.org/g/blink-dev/c/my-groups)

Groups

# Intent to Deprecate and Remove: Top-frame navigations to data URLs

29,727 views

Skip to first unread message

### Mustafa Emre Acer

unread,

Feb 1, 2017, 8:47:55 PM2/1/17

to blink-dev, Emily Schechter, elaw...@chromium.org

Primary eng (and PM) emails

Eng: mea...@chromium.org

PM: emilysc...@chromium.org

Summary

We intend to block web pages from loading data: URLs in the top frame using <A> tags, window.open, window.location and similar mechanisms.

Motivation

data: URLs are generally a source of confusion for users. Because of their unfamiliarity and ability to encode arbitrary untrusted content in a URL, they are widely being used in spoofing and phishing attacks. Another problem is that they can be passed along without a backing page that runs JavaScript (e.g. a data URL can be sent via email). For that reason, we intend to block top-frame navigations to data URLs.

We are considering two alternative implementations:

Alternative 1:

Block only content initiated top-frame navigations to data URLs, while still allowing direct navigations to them. Similar measures are already in place for other schemes such as “chrome:”, “chrome-devtools:” and more recently, “[view-source:](https://groups.google.com/a/chromium.org/forum/#!topic/blink-dev/lyuWXZ_1kXo)”.
In practice, these will be blocked:

-

Navigations when the user clicks on links in the form of <A HREF=”data:…”>

-

window.open(“data:…”)

-

window.location = “data:…”

-

Meta redirects

The following will still be allowed:

-

User navigating to the URL by typing or pasting it in the omnibox

-

Downloads from these protocols:

-

Via non-browser-handled MIME types

-

Via <A download>

-

Via “Save link as”

Alternative 2:

Block all top-frame navigations to data URLs. This only differs from (1) in that it will additionally block direct navigations (“User navigating to the URL by typing or pasting it in the omnibox”).

In both cases, subresources with data URLs (e.g. <img src=”data:...”>, <iframe src=”data:...”>) will be allowed.

Pros of Approach 1:

-

Lower risk of breakage

Cons of Approach 2:

-

Might be confusing to some users ("why does it work when I type the address but not when I click the link?")

-

We might end up playing whack-a-mole with edge cases where we don't properly block the URLs

Pros of Approach 2:

-

Straightforward approach, consistent with IE/Edge behavior.

-

Might be simpler to implement.

Cons of Approach 2:

-

Higher risk of breakage

Compatibility Risk

IE and Edge already block all top-frame navigations to data URLs. Firefox and Safari allow them.

If we implement (2), Chrome’s behavior will align with IE/Edge after this change.

The blocking will be similar to how chrome:// URLs are handled:

-

For same page navigations, clicking the link won’t do anything, and a message will be displayed in the console.

-

For navigations in other tabs or popups, the page will navigate to about:blank instead.

-

No event handlers will be triggered.

Alternative implementation suggestion for web developers

The main use case for navigating to data URLs in the top frame is generating files (HTML, PDF, images etc.) on the fly and displaying them to the user.

For that use case, these alternatives exist:

- Generate the file on the backend and send it to the user over http/https.

- Initiate a download instead of displaying the URL.

- If the contents of the URL is trusted, iframe the URL so that the omnibox displays the site's URL.

Usage information

According to latest Stable Channel metrics over the last 28 days in January 2017, data URLs are 0.05% of all top-frame navigations among all platforms, and 0.01% of all navigations on Android.

OWP launch tracking bug

OWP tracking bug: https://crbug.com/684011

Discussion: https://crbug.com/594215

Entry on the [feature dashboard](https://www.chromestatus.com/)

https://www.chromestatus.com/feature/5669602927312896

Requesting approval to remove too?

Yes, requesting approval to block top-frame navigations to data URLs.

### Peter Kasting

unread,

Feb 2, 2017, 1:24:30 AM2/2/17

to Mustafa Emre Acer, blink-dev, Emily Schechter, elaw...@chromium.org

On Wed, Feb 1, 2017 at 5:47 PM, Mustafa Emre Acer <mea...@chromium.org> wrote:

We are considering two alternative implementations:

Alternative 1:

Block only content initiated top-frame navigations to data URLs, while still allowing direct navigations to them. Similar measures are already in place for other schemes such as “chrome:”, “chrome-devtools:” and more recently, “[view-source:](https://groups.google.com/a/chromium.org/forum/#!topic/blink-dev/lyuWXZ_1kXo)”.

Alternative 2:

Block all top-frame navigations to data URLs. This only differs from (1) in that it will additionally block direct navigations (“User navigating to the URL by typing or pasting it in the omnibox”).

Please don't implement approach 2. As the omnibox owner, I actually test data: URLs and the like by typing them in. If we find we really have to, we can mitigate entry using a method like what we do for javascript: URLs, where we strip the scheme on paste -- the code to do that is already there -- but I would vastly prefer leaving these untouched unless we run into further problems.

The omnibox is a pure user-entry surface that should be able to directly load anything the browser can. Blocking navigations from it should be viewed as an extremely serious step that we have not been willing to take for any other type of URL so far (even shellexecute-type stuff, to my knowledge).

PK

### Charles Harrison

unread,

Feb 2, 2017, 1:30:12 AM2/2/17

to Peter Kasting, Mustafa Emre Acer, blink-dev, Emily Schechter, elaw...@chromium.org

+1

There have been a number of very simple renderer crashes that can be triggered by navigating to a data url. This is very convenient for e.g. pasting into bug reports, as opposed to hosting a file somewhere (or forcing another dev to host it locally to test/bisect).

--
You received this message because you are subscribed to the Google Groups "blink-dev" group.
To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+unsubscribe@chromium.org.

### PhistucK

unread,

Feb 2, 2017, 2:28:22 AM2/2/17

to Charles Harrison, Peter Kasting, Mustafa Emre Acer, blink-dev, Emily Schechter, elaw...@chromium.org

While I also use data: URLs constantly (multiple times hourly sometimes!) for test cases and quick-hack-to-see-if-it-works, I can sympathize with the risk it entails and I think the javascript:-removing approach would be appropriate here (I usually type them, not paste them, so this will only marginally affect my use case).

You can block them altogether, if you really think this is the only safe way to go, but I think it is way too harsh.

It is a shame that data: URLs have to be blocked in any way, though and I would even call it an intervention, honestly.

Regarding the alternatives, blobs can be constructed with the same string (minus data:text/html,) and navigation to them will still be allowed, right?

If so, is that fine from a security perspective (because it shows the originating domain, though with a perhaps confusing prefix), or is this going to be deprecated as well (as far as you know now)?

That can be a sort of acceptable alternative (it only adds about three lines of code, if I remember correctly).

I can imagine a tool that quickly generates blob URLs for the use case we had for data: URLs for test cases (though much less convenient, since you have to create a new URL for every single change you make), but it will not be the same. :(

☆PhistucK

### Peter Kasting

unread,

Feb 2, 2017, 2:39:20 AM2/2/17

to PhistucK, Charles Harrison, Mustafa Emre Acer, blink-dev, Emily Schechter, elaw...@chromium.org

On Wed, Feb 1, 2017 at 11:27 PM, PhistucK <phis...@gmail.com> wrote:

I can sympathize with the risk it entails

They entail risk if they're being used in social-engineering attacks involving pasting them. There are reasons people suggest doing that with JS URLs, most of which don't apply to data: URLs, and I think the risks here are much lower.

I am leery of assuming risk and fixing problems unless you know concretely what the risks and problems are.

PK

### Torne (Richard Coles)

unread,

Feb 2, 2017, 7:36:41 AM2/2/17

to Peter Kasting, PhistucK, Charles Harrison, Mustafa Emre Acer, blink-dev, Emily Schechter, elaw...@chromium.org

Android WebView apps use top-frame data URLs very frequently. Alternative 2 would definitely break those. It's likely some of them will also be broken by alternative 1, but I'm not sure how common that usage is, and we don't currently have metrics for WebView.

### Jochen Eisinger

unread,

Feb 2, 2017, 11:17:47 AM2/2/17

to Torne (Richard Coles), Peter Kasting, PhistucK, Charles Harrison, Mustafa Emre Acer, blink-dev, Emily Schechter, elaw...@chromium.org

Do you have some data or estimates on what part of those 0.05% of navigations are phishing attempts?

I understand that from a security perspective, we'd have to block all possible paths, because otherwise attackers will find ways around this, but I don't think we can break 0.05% of all top-level navigations.

Have we explored alternatives, such as the stripping of the protocol on paste that Peter mentioned, or warn the user if there are e.g. forms on a data: URL, or otherwise making it hard to influence the text shown in the omnibox if it's a data url?

### PhistucK

unread,

Feb 2, 2017, 11:46:34 AM2/2/17

to Jochen Eisinger, Torne (Richard Coles), Peter Kasting, Charles Harrison, Mustafa Emre Acer, blink-dev, Emily Schechter, Eric Lawrence

Note that a "Not Secure" indication was recently added to any data: URL (in Chrome 56, as stable patch!).

https://chromium.googlesource.com/chromium/src/+/2fde612bdca4930cd5bf82a2603ad86d9c5cec57

Huh, this is pretty weird. If data: URLs are not considered secure, then how come you can embed them as an iFrame in an HTTPS page? I would expect them to be mixed content by that rule...

(I do not really think they should be considered insecure, really. Just vulnerable to phishing, just like any page when running a javascript: URL on it)

☆PhistucK

--

### Mustafa Emre Acer

unread,

Feb 2, 2017, 3:50:09 PM2/2/17

to PhistucK, Jochen Eisinger, Torne (Richard Coles), Peter Kasting, Charles Harrison, blink-dev, Emily Schechter, Eric Lawrence

Thanks everyone for the feedback. Some answers inline:

> It is a shame that data: URLs have to be blocked in any way, though and I would even call it an intervention, honestly.

It is indeed an intervention, please see [intervention-dev](https://groups.google.com/a/chromium.org/forum/#!topic/intervention-dev/qaoc7DA_x74) for past updates.

> Regarding the alternatives, blobs can be constructed with the same string (minus data:text/html,) and navigation to them will still be allowed, right?

blob: will be allowed for the time being, though there is ongoing discussion about it (see [crbug.com/](https://crbug.com/)[594215](https://crbug.com/594215)). blob URLs require a backing page to be generated and have a security origin, so we might have other options there, e.g. change how they are displayed.

> Do you have some data or estimates on what part of those 0.05% of navigations are phishing attempts?

We have additional Rappor data where [google.com](http://google.com) and [fb.com](http://fb.com) properties occasionally show up as top initiators of data navigations. Upon further research, these look to be open redirectors that end up at data URLs, and I think it's reasonable to assume they are being used for phishing.

> Have we explored alternatives, such as the stripping of the protocol on paste that Peter mentioned, or warn the user if there are e.g. forms on a data: URL, or otherwise making it hard to influence the text shown in the omnibox if it's a data url?

If we implement #1 (and I lean towards it), stripping the scheme will be moot since direct navigations won't be blocked. We are also looking into other improvements ([crbug.com/680810](https://crbug.com/680810) for form warnings, [crbug.com/533705](https://crbug.com/533705) for graying out url).

> Huh, this is pretty weird. If data: URLs are not considered secure, then how come you can embed them as an iFrame in an HTTPS page? I would expect them to be mixed content by that rule...

The short answer is that data URLs themselves are loaded without hitting the network thus they are not mixed content when loaded on an HTTPS page. Also, please see [crbug.com/684231](https://crbug.com/684231).

### Rick Byers

unread,

Feb 2, 2017, 8:37:28 PM2/2/17

to Mustafa Emre Acer, PhistucK, Jochen Eisinger, Torne (Richard Coles), Peter Kasting, Charles Harrison, blink-dev, Emily Schechter, Eric Lawrence

I chatted a bit with Mustafa and Emily about this. There really is a lot of evidence and user complaints about phishing attacks leveraging this, which makes trying something fairly urgent. It's really hard to say what fraction of the 0.05% is phishing, but it could be most of it (especially given that Edge already blocks this).

Mustafa, can you please [check HTTP Archive](https://groups.google.com/a/chromium.org/forum/#!topic/platform-predictability/Gcp4bdcvoKc) to see if you can find any example of a site that triggers our UseCounter for this on page load? That seems like it shouldn't happen, but if it ever does it would be worth digging in to to understand why.

Assuming we can't find evidence of a site relying on this for legitimate usage than LGTM1 to deprecate and remove, and given the urgency I'd suggest merging the deprecation warning back to M57 and land the removal in M58 ASAP. If you get reports of sites relying on this for legitimate usage then please follow-up here so we can discuss the implications.

There's probably a spec issue to be raised on this, right? Can we produce a pull-request for how we'd like to see the spec change and debate with the other implementations? IMHO getting consensus on that doesn't need to block shipping, but we should at least get the process going.

### Jochen Eisinger

unread,

Feb 2, 2017, 8:45:20 PM2/2/17

to Rick Byers, Mustafa Emre Acer, PhistucK, Torne (Richard Coles), Peter Kasting, Charles Harrison, blink-dev, Emily Schechter, Eric Lawrence

lgtm2

☆PhistucK

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+...@chromium.org.

### Philip Jägenstedt

unread,

Feb 2, 2017, 11:08:25 PM2/2/17

to Jochen Eisinger, Rick Byers, Mustafa Emre Acer, PhistucK, Torne (Richard Coles), Peter Kasting, Charles Harrison, blink-dev, Emily Schechter, Eric Lawrence

lgtm3 assuming nothing worrying appears in the httparchive data.

### PhistucK

unread,

Feb 3, 2017, 1:58:40 AM2/3/17

to Rick Byers, Mustafa Emre Acer, Jochen Eisinger, Torne (Richard Coles), Peter Kasting, Charles Harrison, blink-dev, Emily Schechter, Eric Lawrence

I know websites are using this to hide their referrer (though they can simply use referrerpolicy nowadays, right?). Hehe, I really liked that way of thinking at the time, actually. I thought it was a clever use of this feature.

Perhaps there should be a page linked from the deprecation warning (and later, the request-blocked warning) that explains how to use referrerpolicy instead.

☆PhistucK

### Torne (Richard Coles)

unread,

Feb 3, 2017, 5:16:51 AM2/3/17

to PhistucK, Rick Byers, Mustafa Emre Acer, Jochen Eisinger, Peter Kasting, Charles Harrison, blink-dev, Emily Schechter, Eric Lawrence

I'm still concerned about the possible impact on WebView but I'm not really sure what we can do about it other than be ready to revert if we see lots of issues. WebView doesn't have metrics and is often used to view content embedded in the app or dynamically generated by private API endpoints, which won't show up in the HTTP archive. Deprecation warnings also haven't generated a lot of developer feedback in the past; many developers don't actively test their apps on current versions of WebView and won't see the warnings.

☆PhistucK

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+...@chromium.org.

--
You received this message because you are subscribed to the Google Groups "blink-dev" group.

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+...@chromium.org.

### Jochen Eisinger

unread,

Feb 3, 2017, 9:36:00 AM2/3/17

to Torne (Richard Coles), PhistucK, Rick Byers, Mustafa Emre Acer, Peter Kasting, Charles Harrison, blink-dev, Emily Schechter, Eric Lawrence

Maybe the way to go is to not have this restriction for webview?

### Rick Byers

unread,

Feb 3, 2017, 10:08:10 AM2/3/17

to Jochen Eisinger, Torne (Richard Coles), PhistucK, Mustafa Emre Acer, Peter Kasting, Charles Harrison, blink-dev, Emily Schechter, Eric Lawrence

The most common ways an Android app could load a data URL unto a WebView shouldn't be impacted (since they're not "content navigations"), right? But I can imagine how there may still be substantial risk here.

But WebView is also used to build browsers of potentially malicious content, right? So the same phishing protection arguments apply. Perhaps we could use a targetSdk quirk for this so that Android apps are only impacted when the build against a future SDK? Or perhaps we should wait until we have concrete examples of this being a problem in practice before going to the effort?

☆PhistucK

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+unsubscribe@chromium.org.

--
You received this message because you are subscribed to the Google Groups "blink-dev" group.

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+unsubscribe@chromium.org.

### Emily Schechter

unread,

Feb 3, 2017, 12:31:04 PM2/3/17

to Rick Byers, Jochen Eisinger, Torne (Richard Coles), PhistucK, Mustafa Emre Acer, Peter Kasting, Charles Harrison, blink-dev, Emily Schechter, Eric Lawrence

My understanding is that usage on Android is much lower than on Desktop, so the breakage risk is actually less (@meacer, did we have a separate use counter?)

### PhistucK

unread,

Feb 3, 2017, 12:36:39 PM2/3/17

to Emily Schechter, Rick Byers, Jochen Eisinger, Torne (Richard Coles), Mustafa Emre Acer, Peter Kasting, Charles Harrison, blink-dev, Eric Lawrence

Remember that use counters on Android does not count WebViews.

☆PhistucK

### dyl...@gmail.com

unread,

Feb 6, 2017, 9:23:35 AM2/6/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

As others have mentioned, this looks like a fairly breaking change for various approaches to testing. For example, Intern ( http://theintern.github.io/intern/ ) uses top-level data URLs approach within its Leadfoot ( https://github.com/theintern/leadfoot ) library to run a series of feature tests using WebDriver, prior to running tests. This is to prevent user tests from triggering known WebDriver defects. I don't see either proposed alternative working in this scenario, since it's in the context of an automated testing environment.

### PhistucK

unread,

Feb 6, 2017, 9:28:09 AM2/6/17

to dyl...@gmail.com, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer

Is it using window.location.href (and friends) or clicks on <a href> to change the top level URL to a data: URL, or is it using the WebDriver API for navigation?

I think the former is problematic, while the latter should not be.

☆PhistucK

On Mon, Feb 6, 2017 at 4:23 PM, <dyl...@gmail.com> wrote:

As others have mentioned, this looks like a fairly breaking change for various approaches to testing. For example, Intern ( http://theintern.github.io/intern/ ) uses top-level data URLs approach within its Leadfoot ( https://github.com/theintern/leadfoot ) library to run a series of feature tests using WebDriver, prior to running tests. This is to prevent user tests from triggering known WebDriver defects. I don't see either proposed alternative working in this scenario, since it's in the context of an automated testing environment.

--

### dyl...@gmail.com

unread,

Feb 6, 2017, 9:37:06 AM2/6/17

to blink-dev, dyl...@gmail.com, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

It is using the latter (the WebDriver API). I suppose it was the "and similar mechanisms" that gave us pause for concern, as well as alternative 2 which blocks direct navigation. I understand that WebDriver does not follow the same path as direct navigation, but the intent of the WebDriver API is to mimic direct user interactions. I hope that helps.

☆PhistucK

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+...@chromium.org.

### Elliott Sprehn

unread,

Feb 6, 2017, 10:16:16 AM2/6/17

to Rick Byers, Torne (Richard Coles), Emily Schechter, Peter Kasting, Mustafa Emre Acer, PhistucK, Jochen Eisinger, Charles Harrison, Eric Lawrence, blink-dev

I think the discussion of the two strategies is confusing this intent, I don't think we can do #2 at all, it breaks a substantial amount of use cases (as listed here already plus more). For example folks draw to canvases and then use toDataURL() to display them in an img element. From there users can right click the element and choose "Open in a new tab" or "download" and this would break that.

In the future we should probably have design discussions before the intent to deprecate phase. Having multiple ideas, one very extreme and one less so in the same intent makes this discussion hard to follow.

Which thing got an LGTM above?

### Mustafa Emre Acer

unread,

Feb 6, 2017, 1:51:49 PM2/6/17

to Elliott Sprehn, Rick Byers, Torne (Richard Coles), Emily Schechter, Peter Kasting, PhistucK, Jochen Eisinger, Charles Harrison, Eric Lawrence, blink-dev

> In the future we should probably have design discussions before the intent to deprecate phase. Having multiple ideas, one very extreme and one less so in the same intent makes this discussion hard to follow.

To clarify, I'm going to proceed with Approach #1 (only blocking content initiated navigations). The feedback from this thread indicates that full blockage will be too disruptive and may break legitimate use cases.

We had a separate discussion before sending this intent, and listing both approaches was a decision from that discussion.

Rick, Jochen, Philip: Could you please clarify that you are LGTM'ing Approach #1?

> From there users can right click the element and choose "Open in a new tab" or "download" and this would break that.

Downloads shouldn't be blocked in either case. In contrast, "Open in a new tab" will be broken in both cases to be consistent with the blocking of other URLs such as chrome://, view-source:// etc ("Open in a new tab/window" doesn't work with these URLs). That is, unless we make an exception for data URLs.

### Elliott Sprehn

unread,

Feb 6, 2017, 1:57:16 PM2/6/17

to Mustafa Emre Acer, Torne (Richard Coles), Emily Schechter, Peter Kasting, Rick Byers, PhistucK, Jochen Eisinger, Charles Harrison, Eric Lawrence, blink-dev

On Feb 6, 2017 1:51 PM, "Mustafa Emre Acer" <mea...@chromium.org> wrote:

> In the future we should probably have design discussions before the intent to deprecate phase. Having multiple ideas, one very extreme and one less so in the same intent makes this discussion hard to follow.

To clarify, I'm going to proceed with Approach #1 (only blocking content initiated navigations). The feedback from this thread indicates that full blockage will be too disruptive and may break legitimate use cases.

We had a separate discussion before sending this intent, and listing both approaches was a decision from that discussion.

Rick, Jochen, Philip: Could you please clarify that you are LGTM'ing Approach #1?

> From there users can right click the element and choose "Open in a new tab" or "download" and this would break that.

Downloads shouldn't be blocked in either case. In contrast, "Open in a new tab" will be broken in both cases to be consistent with the blocking of other URLs such as chrome://, view-source:// etc ("Open in a new tab/window" doesn't work with these URLs). That is, unless we make an exception for data URLs.

That seems like a pretty serious breakage. A user choosing "Open in tab" on an image with a data URL src is very reasonable for an offline image editor app or photo galleries. For example Google image search even used data urls for thumbnails at one point.

I don't see why choosing an option like that from a context menu (or dragging the image into the tab strip) would be different from a user pasting the URL into the omnibox.

### Chris Harrelson

unread,

Feb 6, 2017, 4:53:36 PM2/6/17

to Elliott Sprehn, Mustafa Emre Acer, Torne (Richard Coles), Emily Schechter, Peter Kasting, Rick Byers, PhistucK, Jochen Eisinger, Charles Harrison, Eric Lawrence, blink-dev

On Mon, Feb 6, 2017 at 10:57 AM, Elliott Sprehn <esp...@chromium.org> wrote:

On Feb 6, 2017 1:51 PM, "Mustafa Emre Acer" <mea...@chromium.org> wrote:

> In the future we should probably have design discussions before the intent to deprecate phase. Having multiple ideas, one very extreme and one less so in the same intent makes this discussion hard to follow.

To clarify, I'm going to proceed with Approach #1 (only blocking content initiated navigations). The feedback from this thread indicates that full blockage will be too disruptive and may break legitimate use cases.

We had a separate discussion before sending this intent, and listing both approaches was a decision from that discussion.

Rick, Jochen, Philip: Could you please clarify that you are LGTM'ing Approach #1?

> From there users can right click the element and choose "Open in a new tab" or "download" and this would break that.

Downloads shouldn't be blocked in either case. In contrast, "Open in a new tab" will be broken in both cases to be consistent with the blocking of other URLs such as chrome://, view-source:// etc ("Open in a new tab/window" doesn't work with these URLs). That is, unless we make an exception for data URLs.

That seems like a pretty serious breakage. A user choosing "Open in tab" on an image with a data URL src is very reasonable for an offline image editor app or photo galleries. For example Google image search even used data urls for thumbnails at one point.

I don't see why choosing an option like that from a context menu (or dragging the image into the tab strip) would be different from a user pasting the URL into the omnibox.

I agree that this seems a serious problem. Can this be special-cased?

### Mustafa Emre Acer

unread,

Feb 6, 2017, 5:00:51 PM2/6/17

to Chris Harrelson, Elliott Sprehn, Torne (Richard Coles), Emily Schechter, Peter Kasting, Rick Byers, PhistucK, Jochen Eisinger, Charles Harrison, Eric Lawrence, blink-dev

Allowing right click actions for data URLs but keeping them blocked for chrome:// et al sounds reasonable to me.

### Philip Jägenstedt

unread,

Feb 8, 2017, 12:25:54 AM2/8/17

to Mustafa Emre Acer, Chris Harrelson, Elliott Sprehn, Torne (Richard Coles), Emily Schechter, Peter Kasting, Rick Byers, PhistucK, Jochen Eisinger, Charles Harrison, Eric Lawrence, blink-dev

Approach #1 is what I assumed since it was your preference and #2 got pushback.

And keeping "Open in new tab" working on data: images seems reasonable, but is that cleanly separated into its own code path and policy, or will that also keep a few other cases still working? Anything that fixes the problem without breaking too much and that the reviewers are happy with is fine, really.

One thing, though, at least in the case of a script trying to navigate a top-level frame, this change should be web observable, right? Would it be possible to write web-platform-tests for this that will pass in Chromium but fail elsewhere? If so, there should be some spec that covers this that should be updated, and it's probably HTML. Can you look into that while making the change?

☆PhistucK

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+...@chromium.org.

--
You received this message because you are subscribed to the Google Groups "blink-dev" group.

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+...@chromium.org.

### Torne (Richard Coles)

unread,

Feb 8, 2017, 11:11:43 AM2/8/17

to Philip Jägenstedt, Mustafa Emre Acer, Chris Harrelson, Elliott Sprehn, Emily Schechter, Peter Kasting, Rick Byers, PhistucK, Jochen Eisinger, Charles Harrison, Eric Lawrence, blink-dev

OK, as long as we're going with approach #1 this probably is okay for WebView, but since we don't have metrics (or a large dev/beta population) we aren't likely to find out if this breaks a lot of WebView apps until after it ships to stable - we'll have to keep an eye on incoming bug reports for any developers/users reporting issues here.

### Chris Harrelson

unread,

Feb 8, 2017, 12:10:46 PM2/8/17

to Torne (Richard Coles), Philip Jägenstedt, Mustafa Emre Acer, Elliott Sprehn, Emily Schechter, Peter Kasting, Rick Byers, PhistucK, Jochen Eisinger, Charles Harrison, Eric Lawrence, blink-dev

On Wed, Feb 8, 2017 at 8:11 AM, Torne (Richard Coles) <to...@chromium.org> wrote:

OK, as long as we're going with approach #1 this probably is okay for WebView, but since we don't have metrics (or a large dev/beta population) we aren't likely to find out if this breaks a lot of WebView apps until after it ships to stable - we'll have to keep an eye on incoming bug reports for any developers/users reporting issues here.

I'm concerned about WebView. Are there a suite of common WebView apps that can be tested manually on an earlier channel? I would hope the Chrome testing team does some of this as part of release testing.

☆PhistucK

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+unsubscribe@chromium.org.

--
You received this message because you are subscribed to the Google Groups "blink-dev" group.

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+unsubscribe@chromium.org.

--
You received this message because you are subscribed to the Google Groups "blink-dev" group.

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+unsubscribe@chromium.org.

### Torne (Richard Coles)

unread,

Feb 8, 2017, 12:41:55 PM2/8/17

to Chris Harrelson, Philip Jägenstedt, Mustafa Emre Acer, Elliott Sprehn, Emily Schechter, Peter Kasting, Rick Byers, PhistucK, Jochen Eisinger, Charles Harrison, Eric Lawrence, blink-dev

We do test popular apps on WebView, but there is a huge long tail of apps (a majority of all android apps use webview) and we can't test them all.

Unfortunately, we often only find out about the extent of breakage caused by a backward-incompatible platform change like this long after a stable release - in several cases we haven't found out about major breakages until after the *subsequent* stable release, because the latency on "enough users have the new version -> enough complaints reach the app developer -> app developer figures out it was a webview change -> bug gets reported to us" can be very long.

This isn't an argument for not shipping this change, but I'd suggest you at least make sure you have the option to reasonably easily revert this for a while :)

☆PhistucK

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+...@chromium.org.

--
You received this message because you are subscribed to the Google Groups "blink-dev" group.

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+...@chromium.org.

--
You received this message because you are subscribed to the Google Groups "blink-dev" group.

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+...@chromium.org.

### alexand...@gmail.com

unread,

Mar 24, 2017, 12:32:39 PM3/24/17

to blink-dev, chri...@chromium.org, foo...@chromium.org, mea...@chromium.org, esp...@chromium.org, emilysc...@chromium.org, pkas...@chromium.org, rby...@chromium.org, phis...@gmail.com, joc...@chromium.org, cshar...@chromium.org, elaw...@chromium.org

How will this impact HTML injected into a WebView using the loadData method, which seems like it would be a very common use case? Currently the following message appears to be logged:

Upcoming versions will block content-initiated top frame navigations to data: URLs. For more information, see https://goo.gl/BaZAea.", source: data:text/html

Thanks!

### Torne (Richard Coles)

unread,

Mar 24, 2017, 1:06:38 PM3/24/17

to alexand...@gmail.com, blink-dev, chri...@chromium.org, foo...@chromium.org, mea...@chromium.org, esp...@chromium.org, emilysc...@chromium.org, pkas...@chromium.org, rby...@chromium.org, phis...@gmail.com, joc...@chromium.org, cshar...@chromium.org, elaw...@chromium.org

We discussed this earlier in the thread; it shouldn't be affecting use of WebView.loadData because that's not a content-initiated navigation, so unless the plan changed since this thread then that's a bug.

### Mustafa Emre Acer

unread,

Mar 24, 2017, 1:43:13 PM3/24/17

to Torne (Richard Coles), alexand...@gmail.com, blink-dev, Chris Harrelson, foo...@chromium.org, Elliott Sprehn, Emily Schechter, Peter Kasting, Rick Byers, PhistucK, Jochen Eisinger, Charles Harrison, Eric Lawrence

Yes, the deprecation warning may be overly broad and showing in cases where it shouldn't. I'll make sure WebView.loadData still works after this change.

### PhistucK

unread,

Apr 2, 2017, 3:33:15 AM4/2/17

to Mustafa Emre Acer, Torne (Richard Coles), alexand...@gmail.com, blink-dev, Chris Harrelson, Philip Jägenstedt, Elliott Sprehn, Emily Schechter, Peter Kasting, Rick Byers, Jochen Eisinger, Charles Harrison, Eric Lawrence

What about extension-initiated navigations (the extension adds a link and adds an event listener that initiates a navigation to a data URL - all of this is done within the world of the content script)?

While the alternative solution is not too complicated (send a message to the background/event page which calls chrome.tabs.update, I guess), I do not think they should be blocked (like they can use XMLHttpRequest in the world of content script to send cross origin requests and if I remember correctly, call APIs that require user gesture as if they had a user gesture)

☆PhistucK

### jack...@gmail.com

unread,

Apr 4, 2017, 3:53:39 PM4/4/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

For what it's worth, there are many common use cases in web apps for loading urls from blobs.

For example, the only workflow I'm aware of for printing a PDF from the web uses the following steps

1) Pull PDF data as blob

2) Open in new window

3) Call window.print() on window

4) Close window

On Wednesday, February 1, 2017 at 8:47:55 PM UTC-5, Mustafa Emre Acer wrote:

Primary eng (and PM) emails

Eng: mea...@chromium.org

PM: emilysc...@chromium.org

Summary

We intend to block web pages from loading data: URLs in the top frame using <A> tags, window.open, window.location and similar mechanisms.

Motivation

data: URLs are generally a source of confusion for users. Because of their unfamiliarity and ability to encode arbitrary untrusted content in a URL, they are widely being used in spoofing and phishing attacks. Another problem is that they can be passed along without a backing page that runs JavaScript (e.g. a data URL can be sent via email). For that reason, we intend to block top-frame navigations to data URLs.

We are considering two alternative implementations:

Alternative 1:

Block only content initiated top-frame navigations to data URLs, while still allowing direct navigations to them. Similar measures are already in place for other schemes such as “chrome:”, “chrome-devtools:” and more recently, “[view-source:](https://groups.google.com/a/chromium.org/forum/#!topic/blink-dev/lyuWXZ_1kXo)”.
In practice, these will be blocked:

-

Navigations when the user clicks on links in the form of <A HREF=”data:…”>

-

window.open(“data:…”)

-

window.location = “data:…”

-

Meta redirects

The following will still be allowed:

-

User navigating to the URL by typing or pasting it in the omnibox

-

Downloads from these protocols:

-

Via non-browser-handled MIME types

-

Via <A download>

-

Via “Save link as”

Alternative 2:

Block all top-frame navigations to data URLs. This only differs from (1) in that it will additionally block direct navigations (“User navigating to the URL by typing or pasting it in the omnibox”).

In both cases, subresources with data URLs (e.g. <img src=”data:...”>, <iframe src=”data:...”>) will be allowed.

Pros of Approach 1:

-

Lower risk of breakage

Cons of Approach 2:

-

Might be confusing to some users ("why does it work when I type the address but not when I click the link?")

-

We might end up playing whack-a-mole with edge cases where we don't properly block the URLs

Pros of Approach 2:

-

Straightforward approach, consistent with IE/Edge behavior.

-

Might be simpler to implement.

Cons of Approach 2:

-

Higher risk of breakage

Compatibility Risk

IE and Edge already block all top-frame navigations to data URLs. Firefox and Safari allow them.

If we implement (2), Chrome’s behavior will align with IE/Edge after this change.

The blocking will be similar to how chrome:// URLs are handled:

-

For same page navigations, clicking the link won’t do anything, and a message will be displayed in the console.

-

For navigations in other tabs or popups, the page will navigate to about:blank instead.

-

No event handlers will be triggered.

Alternative implementation suggestion for web developers

The main use case for navigating to data URLs in the top frame is generating files (HTML, PDF, images etc.) on the fly and displaying them to the user.

For that use case, these alternatives exist:

- Generate the file on the backend and send it to the user over http/https.

- Initiate a download instead of displaying the URL.

- If the contents of the URL is trusted, iframe the URL so that the omnibox displays the site's URL.

Usage information

According to latest Stable Channel metrics over the last 28 days in January 2017, data URLs are 0.05% of all top-frame navigations among all platforms, and 0.01% of all navigations on Android.

OWP launch tracking bug

OWP tracking bug: https://crbug.com/684011

Discussion: https://crbug.com/594215

Entry on the [feature dashboard](https://www.chromestatus.com/)

https://www.chromestatus.com/feature/5669602927312896

Requesting approval to remove too?

Yes, requesting approval to block top-frame navigations to data URLs.

### Mustafa Emre Acer

unread,

Apr 5, 2017, 7:40:28 PM4/5/17

to jack...@gmail.com, blink-dev, Emily Schechter, Eric Lawrence

> What about extension-initiated navigations (the extension adds a link and adds an event listener that initiates a navigation to a data URL - all of this is done within the world of the content script)?

The current implementation will block content script initiated navigations. Allowing content scripts while blocking pages will complicate the implementation, and I think we should strive for simplicity here. As you pointed, there is a fairly straightforward alternative that serves this use case well.

> For example, the only workflow I'm aware of for printing a PDF from the web uses the following steps

Thanks for pointing this out. I don't think this use case necessitates loading the blob URL in the top frame though. A static page that takes a blob ID as a parameter and displays the corresponding blob URL in an iframe should work the same. Nevertheless, this change will not affect blob URLs and I'll send a separate intent if we decide to change them in the future (unless you meant the literal PDF blob and not blob URLs).

### gabrielale...@hotmail.com

unread,

Apr 9, 2017, 9:08:38 AM4/9/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

Please, no!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

### habd...@gmail.com

unread,

Apr 27, 2017, 3:06:46 PM4/27/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

houf hiidid dijejiid jkejjjijkoqijlkd njkjdiojejnojnjd njeiojokjopjeojj jn nkmkmkndjiekndiod jeknfiojeiojdjjaklmjnnm dfknojejfoijcioejcnnjbbhnfjkldjk jkenvkjnjkhdbnkjnvjkiojjkd jj jondsnkmdjdnj fdnc kjndnkljdlkjfkn dn kjndcjshfbinklkjankjhjc en kjcuhjncouakjbh dnhbvjj hk jdkncoi hb ckj hjkbdiulkjfkjsankdjsfbhdhabiahkjnkjfbjn sakfnbojdnklkljnck hjdbhkcikjd h cjdnchj chjjknusnh d

### Mustafa Emre Acer

unread,

Apr 28, 2017, 6:21:13 PM4/28/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

Update: The blockage CL landed last week and is on Canary channel. For SEO purposes, the console message is "Not allowed to navigate top frame to data URL".

### victorpa...@gmail.com

unread,

May 15, 2017, 11:29:28 PM5/15/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

### PhistucK

unread,

May 19, 2017, 9:50:04 AM5/19/17

to victorpa...@gmail.com, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer

Refreshing the page (by pressing F5, Control + F5, Shift + F5 in a tab, in the omnibox or in the Developer Tools feature, by clicking on the refresh button, or by clicking on "reload" in data:text/html,<a href="javascript:window.location.reload()">reload</a> and similar) shows the deprecation warning. Those should still work after this is implemented, I believe. Should I file an issue, or did you address those cases?

☆PhistucK

--
You received this message because you are subscribed to the Google Groups "blink-dev" group.

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+unsubscribe@chromium.org.

To view this discussion on the web visit [https://groups.google.com/a/chromium.org/d/msgid/blink-dev/637edc2c-457f-4f51-9452-4e769585a615%40chromium.org](https://groups.google.com/a/chromium.org/d/msgid/blink-dev/637edc2c-457f-4f51-9452-4e769585a615%40chromium.org?utm_medium=email&utm_source=footer).

### Mustafa Emre Acer

unread,

May 19, 2017, 1:59:24 PM5/19/17

to PhistucK, victorpa...@gmail.com, blink-dev, Emily Schechter, Eric Lawrence

Sorry, it's not clear to me which deprecation warning you are referring to. Is it the pre-Chrome 60 warning that says "Upcoming versions will block content-initiated top frame navigations"? If so, that's removed in Chrome 60 so you shouldn't be seeing it. I don't see any console warning with data:text/html,<a href="javascript:window.location.reload()">reload</a>in Chrome 60.

### PhistucK

unread,

May 19, 2017, 4:49:28 PM5/19/17

to Mustafa Emre Acer, victor palacio, blink-dev, Emily Schechter, Eric Lawrence

Yes, Chrome 58 -

Upcoming versions will block content-initiated top frame navigations to data: URLs. For more information, see https://goo.gl/BaZAea.

Great, just wanted to verify. :)

☆PhistucK

### Mustafa Emre Acer

unread,

May 19, 2017, 5:24:55 PM5/19/17

to PhistucK, victor palacio, blink-dev, Emily Schechter, Eric Lawrence

Yes, it appears the deprecation message was displayed in some cases where it shouldn't have been :) Sorry about the false alarms.

### mackenzie....@gmail.com

unread,

May 20, 2017, 11:07:21 AM5/20/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

On Wednesday, February 1, 2017 at 8:47:55 PM UTC-5, Mustafa Emre Acer wrote:

### jackl...@gmail.com

unread,

May 29, 2017, 7:58:43 AM5/29/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

### jackl...@gmail.com

unread,

May 29, 2017, 7:59:54 AM5/29/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

### Torne (Richard Coles)

unread,

Jun 9, 2017, 2:43:16 PM6/9/17

to jackl...@gmail.com, blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

While helping someone on #chromium-support I've just noticed that when you do window.open("data:text/html,foo"), the console warning about the deprecation appears to go into the console *of the newly created window*, not into the current window's console. The newly created window then immediately closes without the developer being able to see the warning first. This is probably not ideal :)

--
You received this message because you are subscribed to the Google Groups "blink-dev" group.

To view this discussion on the web visit [https://groups.google.com/a/chromium.org/d/msgid/blink-dev/da88e398-c1b5-427d-93c4-9a66ce24f79e%40chromium.org](https://groups.google.com/a/chromium.org/d/msgid/blink-dev/da88e398-c1b5-427d-93c4-9a66ce24f79e%40chromium.org?utm_medium=email&utm_source=footer).

### PhistucK

unread,

Jun 9, 2017, 3:54:07 PM6/9/17

to Torne (Richard Coles), jackl...@gmail.com, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer

On Fri, Jun 9, 2017 at 9:43 PM, Torne (Richard Coles) <to...@chromium.org> wrote:

The newly created window then immediately closes without the developer being able to see the warning first. This is probably not ideal :)

Why would it be closed immediately? Or are you talking about a particular scenario​ that that specific developer had (just making sure)?

☆PhistucK

### Torne (Richard Coles)

unread,

Jun 9, 2017, 4:05:18 PM6/9/17

to PhistucK, jackl...@gmail.com, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer

Sorry; I mean in M60 and later where these toplevel navigations are blocked, the visible behaviour is that the new window opens and then immediately closes again, and the console error telling you why is only printed in the console of that window so the developer can't see it.

### PhistucK

unread,

Jun 9, 2017, 4:33:58 PM6/9/17

to Torne (Richard Coles), Li Chak, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer

Why would the window even open? :O

☆PhistucK

### Torne (Richard Coles)

unread,

Jun 9, 2017, 4:37:16 PM6/9/17

to PhistucK, Li Chak, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer

I don't know, but it does.

### Mustafa Emre Acer

unread,

Jun 9, 2017, 4:56:07 PM6/9/17

to Torne (Richard Coles), PhistucK, Li Chak, blink-dev, Emily Schechter, Eric Lawrence

That was intentional to be consistent with how other URLs blocked. For example, window.open("chrome://settings") will open a window, fall back to an about:blank URL and print the message in the new window.

I think we can change it if it's more developer and user friendly to print the message in the opener's console, but it would be nice to do it for all blocked URLs.

### PhistucK

unread,

Jun 9, 2017, 6:12:34 PM6/9/17

to Mustafa Emre Acer, Torne (Richard Coles), Li Chak, blink-dev, Emily Schechter, Eric Lawrence

It does not seem consistent if it closes immediately while others remain open with about:blank...

I would mind the error not showing up on the console of the opener much less if the window did not close immediately (giving you absolutely no way of knowing what happened).

☆PhistucK

### Mustafa Emre Acer

unread,

Jun 9, 2017, 6:18:48 PM6/9/17

to PhistucK, Torne (Richard Coles), Li Chak, blink-dev, Emily Schechter, Eric Lawrence

There is no data URL specific code that closes the window. Could the popup blocker or some other extension be closing the window instead?

### mokohk...@gmail.com

unread,

Jun 12, 2017, 7:31:48 PM6/12/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

在 2017年2月2日星期四 UTC+8上午9:47:55，Mustafa Emre Acer写道：

### mrgrea...@gmail.com

unread,

Jun 24, 2017, 6:00:57 PM6/24/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

I am getting a message linking me to this in the android monitor while testing my android application,
How i use the Webview is to display test to my users, my app generates Dungeons and Dragons infor/characters/taverns etc and then sends that data to the webview as html text, this allows for easier sharing, saving etc and for making it possible to dynamically add links inside the text (anchours etc) and spoiler buttons. if this is to be blocked it would be the death of my app! and i certainly have not been using it for phishing or deceiving users.

I would urge reconsideration, i am sure i am not the only app to use the superior and versatile webView over a text box in a perfectly legal and honest fashion

### Mustafa Emre Acer

unread,

Jun 24, 2017, 9:29:39 PM6/24/17

to mrgrea...@gmail.com, blink-dev, Emily Schechter, Eric Lawrence

How are you loading the data URL? For what it's worth, there is a currently a known bug with data URLs and Android Webview: https://crbug.com/732976

### goldma...@gmail.com

unread,

Jul 11, 2017, 3:28:26 PM7/11/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

I ran across this message in my console when using chrome 59.0.3071.115

```

Upcoming versions will block content-initiated top frame navigations to data: URLs. For more information, see https://goo.gl/BaZAea.

```

I regularly use

```

data:text/html;charset=UTF-8,<pre contenteditable>

```

As a scratch space. Originally on a chromebook but now all over.

Approach 2 would break that work flow which may be fine

but there is a general case of serverless web apps that fit in a data uri

things like http://blog.printf.net/articles/2013/05/17/webrtc-without-a-signaling-server/

Applications based on p2p DHTs where the app bootstrap is a data uri with some ips, domain names and a STUN server or two hard coded.

The participants in the network would invite other by generating a data uri and sending over any text channel.

We don't know what the future of powerful p2p browser based apps is but the data uri is likely to be the toe hold that lets this class of application ecosystem thrive.

As for Safe Browsing concern a 'you are about two navigate to a data uri' message similar to the you are following a external link warning on some sites seems a reasonable precaution.

### Torne (Richard Coles)

unread,

Jul 11, 2017, 4:16:58 PM7/11/17

to goldma...@gmail.com, blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

User-initiated navigations to data: URIs are not blocked (approach 2 is not what was implemented): you can still type or paste data URIs into the URL bar, or make bookmarks of data URIs and use them. So, all the use cases you mention should still work, as they aren't content-initiated.

--
You received this message because you are subscribed to the Google Groups "blink-dev" group.

To view this discussion on the web visit [https://groups.google.com/a/chromium.org/d/msgid/blink-dev/55904349-84f6-497a-85ef-db23ddce64c1%40chromium.org](https://groups.google.com/a/chromium.org/d/msgid/blink-dev/55904349-84f6-497a-85ef-db23ddce64c1%40chromium.org?utm_medium=email&utm_source=footer).

### bauer.kla...@gmail.com

unread,

Jul 18, 2017, 7:35:33 AM7/18/17

to blink-dev, goldma...@gmail.com, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

In Chrome 60 and Chrome 61 this change also affects bookmarklets that redirect to a data-uri. Is this intended behavior?

Example:

javascript:/* Park Tab */ (function(){ template=` <html> <head> <title>__TITLE__</title> </head> <body> Parked Tab: <a href=__URI__>__TITLE__</a> </body> </html>`; function htmlEscape(text) { var d = document.createElement('div'); d.innerText = text; return d.innerHTML; } var newbody = ( template.replace(/__URI__/g, JSON.stringify(encodeURI(decodeURI(location.href)))).replace(/__TITLE__/g, htmlEscape(document.title))); location.href = 'data:text/html;base64,' + btoa(newbody); })();

### PhistucK

unread,

Jul 18, 2017, 8:11:00 AM7/18/17

to bauer.kla...@gmail.com, blink-dev, goldma...@gmail.com, Emily Schechter, Eric Lawrence, Mustafa Emre Acer

I believe so, because this is considered as a programmatic navigation.

☆PhistucK

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+unsubscribe@chromium.org.

To view this discussion on the web visit [https://groups.google.com/a/chromium.org/d/msgid/blink-dev/98c09840-0a91-4676-90ef-e61123ef983a%40chromium.org](https://groups.google.com/a/chromium.org/d/msgid/blink-dev/98c09840-0a91-4676-90ef-e61123ef983a%40chromium.org?utm_medium=email&utm_source=footer).

Message has been deleted

### Mustafa Emre Acer

unread,

Jul 20, 2017, 4:56:44 PM7/20/17

to blink-dev, Emily Schechter, Eric Lawrence

An update for Chrome 60 Beta:

Because of a stable release blocker bug (https://crbug.com/732976), we had to re-enable data URL navigations on Android Webview until browser side navigations (PlzNavigate) ships. Please see the bug for details.

### jimand...@gmail.com

unread,

Jul 30, 2017, 3:43:39 PM7/30/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

You're making it hard for web developers to create screenshots of the current screen to give to their users. I have that sort of functionality at http://vispo.com/alephTouch/an.html . The "save" button creates a screenshot and puts it in a new tab. Why don't you back off of this unnecessary crippling of the browser. Think of web developers trying to create interesting features.

### PhistucK

unread,

Jul 30, 2017, 4:44:01 PM7/30/17

to jimand...@gmail.com, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer

1. I think you can easily use blob URLs instead and maintain the same functionality.

2. However, your use case makes sense to me - I think image data URLs are safe. If I were the intent author, I would limit the restriction to non-images. Images should be harmless, just not very efficient.

☆PhistucK

--
You received this message because you are subscribed to the Google Groups "blink-dev" group.
To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+unsubscribe@chromium.org.

To view this discussion on the web visit [https://groups.google.com/a/chromium.org/d/msgid/blink-dev/0be5ac54-c3a1-4084-866e-5b112af5a7c9%40chromium.org](https://groups.google.com/a/chromium.org/d/msgid/blink-dev/0be5ac54-c3a1-4084-866e-5b112af5a7c9%40chromium.org?utm_medium=email&utm_source=footer).

### Jim Andrews

unread,

Jul 30, 2017, 6:29:53 PM7/30/17

to PhistucK, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer

Thank you. That is how you should think. Get rid of dangerous functionality, but don't throw the baby out with the bath water. You say image data URLs are safe. I would have thought so. I have no idea how they could be harmful, and it is really extremely frustrating to have good functionality unnecessarily destroyed. Browser developers seem to rarely think of people like me, who try to deliver great net art.

Please permit image data URLs in Chrome!

Are you implying that using blob URLs would generate the image faster? I haven't used blob URLs yet for anything.

ja
http://vispo.com/alephTouch/an.html
click "save" to generate a screenshot in firefox but not chrome

### PhistucK

unread,

Jul 31, 2017, 2:21:29 AM7/31/17

to Jim Andrews, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer

A blob URL would be the elegant solution, I think. It might even be faster and require less memory.

☆PhistucK

### Jim Andrews

unread,

Jul 31, 2017, 2:42:21 PM7/31/17

to PhistucK, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer

Thanks. I have written some code that attempts to create a screenshot of a canvas and open it in a new tab, using blobs. It creates the <img>, but when it attempts to open the <img> in a new tab, I get this message in the console: "Not allowed to load local resource". The browser itself says "Your file was not found. It may have been moved or deleted. ERR_FILE_NOT_FOUND".

Can the file be saved to disk? Can I name it? Can I open a Save As dialog box for it?

Here's the code:

var canvas=document.createElement('canvas');
canvas.toBlob(blobCreatedCallback);

function blobCreatedCallback(blob) {
var newImg = document.createElement('img');
var url = URL.createObjectURL(blob);
newImg.src = url;
newImg.onload = function() {
// no longer need to read the blob so it's revoked
URL.revokeObjectURL(url);
window.open(newImg);
};
}

### Torne (Richard Coles)

unread,

Jul 31, 2017, 3:09:28 PM7/31/17

to Jim Andrews, PhistucK, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer

Shouldn't that be window.open(url); ?

It takes a URL, not an element..

☆PhistucK

☆PhistucK

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+...@chromium.org.

To view this discussion on the web visit [https://groups.google.com/a/chromium.org/d/msgid/blink-dev/0be5ac54-c3a1-4084-866e-5b112af5a7c9%40chromium.org](https://groups.google.com/a/chromium.org/d/msgid/blink-dev/0be5ac54-c3a1-4084-866e-5b112af5a7c9%40chromium.org?utm_medium=email&utm_source=footer).

--
You received this message because you are subscribed to the Google Groups "blink-dev" group.

To view this discussion on the web visit [https://groups.google.com/a/chromium.org/d/msgid/blink-dev/c2a1c87e-090e-5d3c-02a1-a041b42aa498%40gmail.com](https://groups.google.com/a/chromium.org/d/msgid/blink-dev/c2a1c87e-090e-5d3c-02a1-a041b42aa498%40gmail.com?utm_medium=email&utm_source=footer).

### Jim Andrews

unread,

Jul 31, 2017, 3:18:02 PM7/31/17

to Torne (Richard Coles), PhistucK, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer

Thanks. But the below doesn't work either. I'm working from the example in https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob titled "Getting a file representing a canvas".

var canvas=document.createElement('canvas');
canvas.toBlob(blobCreatedCallback);

function blobCreatedCallback(blob) {
var newImg = document.createElement('img');
var url = URL.createObjectURL(blob);
newImg.src = url;
newImg.onload = function() {
// no longer need to read the blob so it's revoked

window.open(url);
URL.revokeObjectURL(url);
};
}

### jack...@gmail.com

unread,

Jul 31, 2017, 3:20:15 PM7/31/17

to blink-dev

Perhaps this conversation could be moved to direct email? This is probably not the right location for implementation conversation.

### Boris Zbarsky

unread,

Jul 31, 2017, 3:43:12 PM7/31/17

to PhistucK, blink-dev

On 7/31/17 2:20 AM, PhistucK wrote:
> A blob URL would be the elegant solution, I think. It might even be
> faster and require less memory.

There are at least two drawbacks to using a blob URL compared to a data:
URL here, off the top of my head:

1) It's really easy to leak. You have to either revoke the object URL
(when? it's really hard to tell) or leak its image data for the lifetime
of the page that calls createObjectURL. data: URLs do not have this
problem.

2) It really can't play nicely with session restoration, unlike data: URLs.

-Boris

### guest...@gmail.com

unread,

Aug 3, 2017, 10:35:45 PM8/3/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

Bad idea.

The user already has the option to block loading pages by creating a Policy.

If you want to make an option more visible provide an option at Settings to block data URL navigation.

Do not "block web pages" of any kind.

### looro...@gmail.com

unread,

Aug 4, 2017, 7:57:26 AM8/4/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org, guest...@gmail.com

A Chrome product forum user reported that this breaks their website with legitimate use case:

window.open(`data:application/pdf;base64, ${response}`, '_self');

Will Blob URL work with window.open? Can Chromium implement a whitelist for allowed mimetypes?

Chrome Product Forum thread:

https://productforums.google.com/forum/#!topic/chrome/wvYYcBwJ9FA;context-place=forum/chrome

### Mustafa Emre Acer

unread,

Aug 4, 2017, 2:20:23 PM8/4/17

to looro...@gmail.com, blink-dev, Emily Schechter, Eric Lawrence, guest...@gmail.com

To respond to the broader question of why we didn't restrict this to certain mime-types: Main reason is consistency. We want to align with MS Edge behavior which blocks all data URLs. Note that Firefox is also going to block top level navigations to data URLs in upcoming versions: https://bugzilla.mozilla.org/show_bug.cgi?id=1331351

As an alternative, data URLs can still be iframed. Please see https://crbug.com/594215#c64 for sample code. Blob URLs have some drawbacks as described earlier in this thread.

### Jim Andrews

unread,

Aug 6, 2017, 5:48:20 PM8/6/17

to blin...@chromium.org

For my use case, ie, creating a screenshot for users of a web page, the only drawback I've seen to using canvas.toBlob (thanks PhistucK) is that it's not supported on the iPad (and probably the iPhone too, though I haven't checked). My code checks if canvas.toBlob is supported and does it that way, if it can. If canvas.toBlob is not supported, I try to do it with window.open(canvas.toDataURL('image/png')).

Was there some other drawback to canvas.toBlob I missed?

Thanks,
Jim Andrews

--
You received this message because you are subscribed to a topic in the Google Groups "blink-dev" group.
To unsubscribe from this topic, visit https://groups.google.com/a/chromium.org/d/topic/blink-dev/GbVcuwg_QjM/unsubscribe.
To unsubscribe from this group and all its topics, send an email to blink-dev+...@chromium.org.
To view this discussion on the web visit [https://groups.google.com/a/chromium.org/d/msgid/blink-dev/CAHafXh3FuHav8uJK0JpeUs0tt2Ogh%2Bas0MDDtLO%2B2LHozdxZrw%40mail.gmail.com](https://groups.google.com/a/chromium.org/d/msgid/blink-dev/CAHafXh3FuHav8uJK0JpeUs0tt2Ogh%2Bas0MDDtLO%2B2LHozdxZrw%40mail.gmail.com?utm_medium=email&utm_source=footer).

### PhistucK

unread,

Aug 7, 2017, 8:18:12 AM8/7/17

to Jim Andrews, blink-dev

I believe you can create a blob from a data URL and that should work on iPad as well.

☆PhistucK

To unsubscribe from this group and all its topics, send an email to blink-dev+unsubscribe@chromium.org.

To view this discussion on the web visit [https://groups.google.com/a/chromium.org/d/msgid/blink-dev/CAHafXh3FuHav8uJK0JpeUs0tt2Ogh%2Bas0MDDtLO%2B2LHozdxZrw%40mail.gmail.com](https://groups.google.com/a/chromium.org/d/msgid/blink-dev/CAHafXh3FuHav8uJK0JpeUs0tt2Ogh%2Bas0MDDtLO%2B2LHozdxZrw%40mail.gmail.com?utm_medium=email&utm_source=footer).

--

You received this message because you are subscribed to the Google Groups "blink-dev" group.

To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+unsubscribe@chromium.org.
To view this discussion on the web visit [https://groups.google.com/a/chromium.org/d/msgid/blink-dev/e2c20f01-5da9-0133-3bd7-51e8c7b6b8d0%40gmail.com](https://groups.google.com/a/chromium.org/d/msgid/blink-dev/e2c20f01-5da9-0133-3bd7-51e8c7b6b8d0%40gmail.com?utm_medium=email&utm_source=footer).

### Boris Zbarsky

unread,

Aug 7, 2017, 12:51:35 PM8/7/17

to blink-dev

On 8/4/17 2:19 PM, Mustafa Emre Acer wrote:
> To respond to the broader question of why we didn't restrict this to
> certain mime-types: Main reason is consistency. We want to align with MS
> Edge behavior which blocks all data URLs. Note that Firefox is also
> going to block top level navigations to data URLs in upcoming
> versions: https://bugzilla.mozilla.org/show_bug.cgi?id=1331351

> <https://bugzilla.mozilla.org/show_bug.cgi?id=1331351>

That's not a given. Firefox has implemented a behavior like this behind
a preference, but whether it gets shipped depends on compatibility
testing and whatnot.

-Boris

### Mustafa Emre Acer

unread,

Aug 7, 2017, 1:38:34 PM8/7/17

to Boris Zbarsky, blink-dev

Thanks for the correction!

--
You received this message because you are subscribed to a topic in the Google Groups "blink-dev" group.
To unsubscribe from this topic, visit https://groups.google.com/a/chromium.org/d/topic/blink-dev/GbVcuwg_QjM/unsubscribe.
To unsubscribe from this group and all its topics, send an email to blink-dev+unsubscribe@chromium.org.

To view this discussion on the web visit https://groups.google.com/a/chromium.org/d/msgid/blink-dev/0a00b0c4-e120-e74e-a17c-dddb95c728d5%40mit.edu.

### Jim Andrews

unread,

Aug 7, 2017, 7:57:14 PM8/7/17

to PhistucK, blink-dev

Thanks. If I understand you correctly, then, the code for screenshots should first test for support of canvas.toBlob and do it that way if supported. Then test for being able to create a blob from a data URL. And finally, if neither of those work, try window.open(canvas.toDataURL('image/png')). Correct?

ja

### PhistucK

unread,

Aug 8, 2017, 1:38:56 AM8/8/17

to Jim Andrews, blink-dev

Yeah, but I am not sure you even need the fallback in relatively modern browsers. If you want to support Internet Explorer 9 or something, then you might need it (but it will not work, because Internet Explorer does not load top level data URLs anyway). You can create a new window with an iFrame in it that displays the data URL (pass it via postMessage). That should pretty much work everywhere.

☆PhistucK

### guest...@gmail.com

unread,

Aug 13, 2017, 9:51:42 AM8/13/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

How do we turn this "feature" off? Surely you have simultaneously created a flag that disables this data URL blocking behaviour to top frame for those not interested in the "feature"?

### looro...@gmail.com

unread,

Aug 13, 2017, 10:22:30 AM8/13/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org, guest...@gmail.com

On Sunday, August 13, 2017 at 9:51:42 PM UTC+8, guest...@gmail.com wrote:

How do we turn this "feature" off? Surely you have simultaneously created a flag that disables this data URL blocking behaviour to top frame for those not interested in the "feature"?

It can't be turned off. Not every changes made in Chromium have a flag, especially when the change is not a feature.

### PhistucK

unread,

Aug 13, 2017, 12:33:22 PM8/13/17

to Rong Jie Loo, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer, guest...@gmail.com

Are you sure it cannot be turned off?

I have not tried, but I think --enable-features="AllowContentInitiatedDataUrlNavigations" would turn it off.

(https://cs.chromium.org/chromium/src/content/public/common/content_features.cc?q=kAllowContentInitiatedDataUrlNavigations&sq=package:chromium&l=16&dr=CSs)

☆PhistucK

--

You received this message because you are subscribed to the Google Groups "blink-dev" group.
To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+unsubscribe@chromium.org.

To view this discussion on the web visit [https://groups.google.com/a/chromium.org/d/msgid/blink-dev/4e1e1123-205a-450c-ab73-b14ead7feadd%40chromium.org](https://groups.google.com/a/chromium.org/d/msgid/blink-dev/4e1e1123-205a-450c-ab73-b14ead7feadd%40chromium.org?utm_medium=email&utm_source=footer).

### Rong Jie Loo

unread,

Aug 14, 2017, 12:34:11 AM8/14/17

to PhistucK, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer, guest...@gmail.com

(Resending with reply-all)

My bad, I shouldn't make the claim without checking the source code. But of course, the flag will be removed in the future, so doesn't really solve Guestis's question.

### oguen...@gmail.com

unread,

Aug 18, 2017, 2:38:36 AM8/18/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

externalfile:drive-564fbfa5cd35c1f80e162431707a99ee0f14c3ec/root/Google Photos/2017/Screenshot 2017-07-04 at 11.55.55.png

### oguen...@gmail.com

unread,

Aug 18, 2017, 2:41:03 AM8/18/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

On Wednesday, February 1, 2017 at 6:47:55 PM UTC-7, Mustafa Emre Acer wrote:

Primary eng (and PM) emailshttps://www.instagram.com/accounts/edit/?hl=en

### X

unread,

Aug 29, 2017, 12:32:03 PM8/29/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org

We intend to block web pages from loading data: URLs in the top frame using <A> tags, window.open, window.location and similar mechanisms.

Do not do this.

data: URLs this good option who open many positive things

that I use a lot of time.

I've seen bad redirects that use data: URLs

but data: URLs blocking does not solve the problem

IE and Edge already block all top-frame navigations to data URLs. Firefox and Safari allow them.

but IE not support more good things or poorly supported

& 29.08.2017

Firefox today not block all top-frame navigations to data URLs.

### peca...@gmail.com

unread,

Sep 7, 2017, 1:29:31 PM9/7/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org, racoo...@gmail.com

Well, thanks for utterly bricking any number of perfectly legitimate webapps, including one our MakerSpace created for filling out our forms. The phishers have obviously already (like as in, before this thread even started) changed all their stuff to use iFrames, etc. or just were using external links or email attachments to begin with, so basically, you've accomplished nothing.

Congrats. No, really. In fact, I actually have a few more changes to suggest while you're at it.

After all, I would want to be accused of complaining while not proposing any solutions of my own, so here's my new proposal!

Summary

We intend to block JavaScript from working because we are a board room full of pretentious aholes who have nothing better to do today.

Motivation

JavaScript is generally a source of confusion for users. Because of it's dynamic nature, it can change the behavior of HTML elements in unexpected ways that may confuse the user. While it can improve the usability of some websites, some developers make poor, unstable code that could make the user think there's something wrong with the browser. We also believe JavaScript may be widely used in spoofing and phishing attacks, things that would definitely stop happening simply because of a stupid change in a browser.

We are aware that this proposal would horrible break many existing web apps, but hey, apparently Apple can just remove the ability to disable zoom in Mobile iOS without asking anyone, because it's "sometimes used inappropriately", so we don't really care anymore.

We are considering two alternative implementations:

Alternative 1:

Block only JavaScript in <script> tags and in .js files. The user will still be able to manually add scripts to the page by pasting them into the console. Developers can simply indicate what to type into the console on their JavaScript-dependent pages.

The following will still be allowed:

HTML

CSS (We're coming for you next, with your offensive, non-standardized styles)

Alternative 2:

Or... We could just stop banning things because stupid people exist (something that will ALWAYS be the case) or because someone, somewhere might be "triggered."

Image that...

### Mustafa Emre Acer

unread,

Sep 7, 2017, 1:41:16 PM9/7/17

to peca...@gmail.com, blink-dev, Emily Schechter, Eric Lawrence, racoo...@gmail.com

Hi, can you please give more details about your use case? Are you generating forms using data URLs? If so, does iframing the generated data URL not work? The sample code at https://crbug.com/594215#c64 should work with little modification.

### Chris Harrelson

unread,

Sep 7, 2017, 1:45:20 PM9/7/17

to peca...@gmail.com, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer, racoo...@gmail.com

We appreciate your feedback, and will respond to it.

However, please be aware of our [code of conduct](https://chromium.googlesource.com/chromium/src/+/master/CODE_OF_CONDUCT.md). Conversations on this list and elsewhere in the Chromium project must remain respectful, kind, and solutions-oriented.

On Thu, Sep 7, 2017 at 10:29 AM, <peca...@gmail.com> wrote:

--

You received this message because you are subscribed to the Google Groups "blink-dev" group.
To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+unsubscribe@chromium.org.

To view this discussion on the web visit [https://groups.google.com/a/chromium.org/d/msgid/blink-dev/757e2092-2599-4927-8853-c09a35847267%40chromium.org](https://groups.google.com/a/chromium.org/d/msgid/blink-dev/757e2092-2599-4927-8853-c09a35847267%40chromium.org?utm_medium=email&utm_source=footer).

### peca...@gmail.com

unread,

Sep 8, 2017, 1:30:36 AM9/8/17

to blink-dev, peca...@gmail.com, emilysc...@chromium.org, elaw...@chromium.org, racoo...@gmail.com, mea...@chromium.org

On Thursday, September 7, 2017 at 1:41:16 PM UTC-4, Mustafa Emre Acer wrote:

Hi, can you please give more details about your use case? Are you generating forms using data URLs? If so, does iframing the generated data URL not work? The sample code at https://crbug.com/594215#c64 should work with little modification.

We're using data URLs to preview locally generated PDFs, images, and other files across multiple apps. Opening the preview in a new tab works much more consistently than data url iFrames across mobile platforms. It feels like a workaround and people don't like it. Why should I have to take time to manually implement CTRL+S to save functionality and a touch-friendly iframe close mechanism when these things are built into the browser already?

Besides, the point of my original (quite salty) post was, why should a long-standing feature that was working perfectly fine, and has been implemented for some time in existing apps, suddenly be bricked for no reason?

### PhistucK

unread,

Sep 8, 2017, 5:35:33 AM9/8/17

to peca...@gmail.com, blink-dev, Emily Schechter, Eric Lawrence, racoo...@gmail.com, Mustafa Emre Acer

Remember that this feature was never fully interoperable anyway. You may disregard Edge and Internet Explorer, but it never worked in them.

While using an iFrame is indeed more convoluted and takes some work, it exposes a readable URL to the user which may help them make a better decision when trying to identify the content.

(Yes, I realize that most of the users do not look at the URL at all, or at the secure state of it, but that is off topic)

☆PhistucK

--

You received this message because you are subscribed to the Google Groups "blink-dev" group.
To unsubscribe from this group and stop receiving emails from it, send an email to blink-dev+unsubscribe@chromium.org.

To view this discussion on the web visit [https://groups.google.com/a/chromium.org/d/msgid/blink-dev/06c21065-67b4-43ee-bfc7-16bb7fdb08c2%40chromium.org](https://groups.google.com/a/chromium.org/d/msgid/blink-dev/06c21065-67b4-43ee-bfc7-16bb7fdb08c2%40chromium.org?utm_medium=email&utm_source=footer).

### ali.c...@gmail.com

unread,

Sep 21, 2017, 2:02:05 AM9/21/17

to blink-dev, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org, racoo...@gmail.com, peca...@gmail.com

Have to agree with this. Being able to generate files on the client, without having to touch a back end was bringing the web one step closer to desktop apps. This has regressed the web in a big way. I'm pretty disappointed.

### Rick Byers

unread,

Sep 21, 2017, 8:10:44 AM9/21/17

to ali.c...@gmail.com, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer, racoo...@gmail.com, peca...@gmail.com

Can you provide any examples of real-world websites broken by this?

Our data suggests that the vast majority of sites relying on this ability were using it to trick users into doing things they didn't want to do. Protecting our users from malicious websites is a [core part of Chrome's mission](https://www.google.com/chrome/browser/features.html#security). To us, preserving user's confidence in the safety of the web is far from "no reason". As such, our [compatibility principles allow](https://docs.google.com/document/d/1RC-pBBvsazYfCNNUSkPqAVpSpNJ96U8trhNkfV0v9fk/edit#heading=h.iklh96dxj81w) for a slightly greater level of breakage than we'd otherwise tolerate. Still, some number of concrete examples of legitimate real-world cases (especially cases which somehow still work OK in Edge) would cause us to reconsider this change.

Rick

--

You received this message because you are subscribed to the Google Groups "blink-dev" group.

To view this discussion on the web visit [https://groups.google.com/a/chromium.org/d/msgid/blink-dev/d4542481-fc82-4939-8e9f-07718a16aaee%40chromium.org](https://groups.google.com/a/chromium.org/d/msgid/blink-dev/d4542481-fc82-4939-8e9f-07718a16aaee%40chromium.org?utm_medium=email&utm_source=footer).

### Alastair Taft

unread,

Sep 21, 2017, 8:43:45 AM9/21/17

to Rick Byers, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer, racoo...@gmail.com, peca...@gmail.com

In my case I don't have anything public facing relying on this, but I use it for an internal site to generate JSON files that load in a new tab. But I can envision it being applicable to generating things like PDFs and spreadsheets on the fly, instant file generation is so much nicer than waiting for a server to do the work.

It may not be the case that it's widely adopted yet (relating to your data), but I would hedge my bets that as the gap between native apps and the web closes, more and more developers will want to make use of this feature.

Rather than outright block the feature, what about the possibility of showing a warning message that said something along the lines of "This website is trying to generate a file in a new tab, do you wish to proceed? This could be malicious if you did not trigger it!" And if they hit "Yes" it may assume it's a safe site and stop warning them every time a file is generated.

### Torne (Richard Coles)

unread,

Sep 21, 2017, 9:50:47 AM9/21/17

to Alastair Taft, Rick Byers, blink-dev, Emily Schechter, Eric Lawrence, Mustafa Emre Acer, racoo...@gmail.com, peca...@gmail.com

If you want to open a new tab containing some generated JSON, you can do this by opening about:blank and writing the JSON into the document; you don't need a data: URI for this.

To view this discussion on the web visit [https://groups.google.com/a/chromium.org/d/msgid/blink-dev/CAB1-osw6cfFhrdcPfLcS0WcB_oL6kRepCkTFBSxYC2uhx82V9g%40mail.gmail.com](https://groups.google.com/a/chromium.org/d/msgid/blink-dev/CAB1-osw6cfFhrdcPfLcS0WcB_oL6kRepCkTFBSxYC2uhx82V9g%40mail.gmail.com?utm_medium=email&utm_source=footer).

### ru...@ethos7.com

unread,

Oct 19, 2017, 11:48:51 PM10/19/17

to blink-dev, rby...@chromium.org, emilysc...@chromium.org, elaw...@chromium.org, mea...@chromium.org, racoo...@gmail.com, peca...@gmail.com, ali.c...@gmail.com

Hi,

I used this function similarly to Alastair Taft on an internally facing administrative app for my clients. It worked great. Especially once they were able to get past the popup blocker and click to allow popups on this particular website. Which is the obvious solution here. Rather than a draconian ban on a great function, you could have simply done exactly the same workflow as the popup blocker. Block the URL with a warning and a big button that says "I love the web app my coder made me so I would like to allow his code to work on this webpage. He made a great choice for security reasons so I could create a pasword protected PDF that has sensitive data without needing to pass that data back to the server or ever have a file anywhere but on my local machine." Alternatively it could just say "Do not block data URL's on this website."

It really should not be your mission in life to decide who I should trust in instances like this. Unless you really do just want a population of drooling idiots. Which you probably do since they are more likely to click a google ad.

Imagine if they just decided to ban email after that crown prince of Uganda sent that first email asking me to proceed with the wire transfer. Where would Google be without gMail?

Certainly hate the way Google and Apple are starting to act like Microsoft now that they have a solid market share.

-R

### Eric Lawrence

unread,

Oct 20, 2017, 9:19:00 AM10/20/17

to ru...@ethos7.com, blink-dev, Rick Byers, Emily Schechter, Mustafa Emre Acer, racoo...@gmail.com, Bryce Peterson, ali.c...@gmail.com

Hi, Rudy-- Do you need help updating your site to use one of the alternative approaches described in the thread above?

thanks,

-Eric

-Eric

It is loading more messages.

0 new messages

Search

Clear search

Close search

Google apps

Main menu
