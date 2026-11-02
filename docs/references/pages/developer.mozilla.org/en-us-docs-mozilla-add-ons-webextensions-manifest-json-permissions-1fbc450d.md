---
url: "https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions"
title: "permissions - Mozilla | MDN"
site: "MDN Web Docs"
captured: "2026-11-02T18:26:32Z"
via: "live"
sha256: "c3c2f6acc4b8c90d11f8c421465c5dc7bb51d8e70a59be07250a05ea4119f10d"
wayback: "https://web.archive.org/web/20261016021127/https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# permissions - Mozilla | MDN

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions>
> Snapshot: <https://web.archive.org/web/20261016021127/https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions>

# permissions

| Type  | Array

| Mandatory  | No

| Manifest version  | 2 or higher

| Example  |

json

```
"permissions": [
  "webRequest"
]
```

Use the permissions key to request special powers for your extension. This key is an array of strings, each of which is a permission request.

If you request permissions using this key, the browser may inform the user at install time that the extension is requesting certain privileges and ask them to confirm they are happy to grant privileges. The browser may also allow the user to inspect an extension's privileges after installation. As requesting privileges may impact users' willingness to install your extension, it's worth careful consideration. For example, avoid requesting unnecessary permissions and explain why you are requesting permissions in your extension's store description. For more information on the issues you should consider, see the article [Request the right permissions](https://extensionworkshop.com/documentation/develop/request-the-right-permissions/).

For information on how to test and preview permission requests, see [Test permission requests](https://extensionworkshop.com/documentation/develop/test-permission-requests/) on the Extension Workshop site.

The key can contain three kinds of permissions:

- host permissions (Manifest V2 only, host permissions are specified in the [host_permissions](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/host_permissions) manifest key for Manifest V3 or higher.)
- API permissions
- the [activeTab permission](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/activeTab_permission)

## [Host permissions](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions#host_permissions)

Note: How you request host permissions depends on whether you want them at install time or runtime and which manifest version your extension is using.

- Manifest V2:

- install time request with this (permissions) manifest key.
- runtime request with the [optional_permissions](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/optional_permissions) manifest key.

- Manifest V3 or higher:

- install time request with the [host_permissions](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/host_permissions) manifest key.
- runtime request with the [optional_host_permissions](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/optional_host_permissions) manifest key.

Host permissions are specified as [match patterns](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Match_patterns), and each pattern identifies a group of URLs for which the extension is requesting extra privileges. For example, a host permission could be "*://developer.mozilla.org/*".

The extra privileges include:

- [XMLHttpRequest](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest) and [fetch](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) access to those origins without cross-origin restrictions.

Note: For Manifest V2 extensions in Firefox only, this includes requests made from content scripts.

- the ability to read tab-specific metadata without the "tabs" permission, such as the url, title, and favIconUrl properties of [tabs.Tab](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs/Tab) objects
- the ability to [inject content scripts](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Content_scripts#loading_content_scripts) and styles programmatically into pages served from those origins.
- the ability to receive events from the [webRequest](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/webRequest) API for these hosts
- the ability to access cookies for that host using the [cookies](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/cookies) API, as long as the "cookies" API permission.
- bypassing tracking protection for extension pages where a host is specified as a full domain or with wildcards. Content scripts, however, can only bypass tracking protection for hosts specified with a full domain.
- the ability to create and retrieve WebAuthn credentials. See [Use Web Authn API in web extensions](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Use_the_web_authn_api) for details.

## [API permissions](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions#api_permissions)

You specify API permissions as keywords, and each keyword names a [WebExtension API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API) that the extension would like to use.

These permissions are available in Manifest V2 and above unless otherwise noted:

- activeTab
- alarms
- background
- bookmarks
- browserSettings
- browsingData
- captivePortal
- clipboardRead
- clipboardWrite
- contentSettings
- contextMenus
- contextualIdentities
- cookies
- debugger
- declarativeNetRequest
- declarativeNetRequestFeedback
- declarativeNetRequestWithHostAccess
- devtools (This permission is granted implicitly when the [devtools_page](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/devtools_page) manifest key is present.)
- dns
- downloads
- downloads.open
- find
- geolocation
- history
- identity
- idle
- management
- menus
- menus.overrideContext
- nativeMessaging
- notifications
- pageCapture
- pkcs11
- privacy
- publicSuffix
- proxy
- scripting
- search
- sessions
- storage
- tabGroups
- tabHide
- tabs
- theme
- topSites
- unlimitedStorage
- userScripts (see [userScripts permission](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/userScripts#permissions))
- webNavigation
- webRequest
- webRequestAuthProvider (Manifest V3 and above)
- webRequestBlocking
- webRequestFilterResponse
- webRequestFilterResponse.serviceWorkerScript

In most cases, the permission grants access to the API only, with these exceptions:

-

tabs gives you access to [privileged parts of the tabs API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs) without the need for [host permissions](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions#host_permissions): Tab.url, Tab.title, and Tab.faviconUrl.

- In Firefox 85 and earlier, you also need tabs if you want to include url in the queryInfo parameter to [tabs.query()](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs/query). The extension can use the rest of the tabs API without requesting any permission.
- As of Firefox 86 and Chrome 50, matching [host permissions](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions#host_permissions) can also be used instead of the "tabs" permission.

-

webRequestBlocking enables you to use the "blocking" argument, so you can [modify and cancel requests](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/webRequest).

-

downloads.open lets you use the [downloads.open()](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/downloads/open) API.

-

tabHide lets you use the [tabs.hide()](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs/hide) API.

## [activeTab permission](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions#activetab_permission)

When a user interacts with an extension that has the activeTab permission, the extension is granted extra privileges for the tab where the interaction occurred. This lets an extension act on the current page when the user asks, without requesting broad [host permissions](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions#host_permissions).

For details of how the permission is granted, what it enables, when the access ends, and how behavior differs among browsers, see the [activeTab permission](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/activeTab_permission) page.

## [Clipboard access](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions#clipboard_access)

Two permissions enable an extension to interact with the clipboard:

[clipboardWrite](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions#clipboardwrite)

Write to the clipboard using [Clipboard.write()](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/write), [Clipboard.writeText()](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/writeText), document.execCommand("copy") or document.execCommand("cut").

[clipboardRead](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions#clipboardread)

Read from the clipboard using [Clipboard.read()](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/read), [Clipboard.readText()](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/readText) or document.execCommand("paste").

See [Interact with the clipboard](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Interact_with_the_clipboard) for more details.

## [Unlimited storage](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions#unlimited_storage)

The unlimitedStorage permission:

- Enables extensions to exceed any quota imposed by the [storage.local](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/storage/local) API
- In Firefox, enables extensions to create a ["persistent" IndexedDB database](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API) without the browser prompting the user for permission at the time the database is created.

## [Examples](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions#examples)

json

```
 "permissions": ["*://developer.mozilla.org/*"]

```

In Manifest V2 only, request privileged access to pages under developer.mozilla.org.

json

```
  "permissions": ["tabs"]

```

Request access to the privileged pieces of the tabs API.

json

```
  "permissions": ["*://developer.mozilla.org/*", "tabs"]

```

In Manifest V2 only, request both of the above permissions.

## [Browser compatibility](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions#browser_compatibility)

##  Help improve MDN

[Learn how to contribute](https://developer.mozilla.org/en-US/docs/MDN/Community/Getting_started)

This page was last modified on Oct 8, 2026 by [MDN contributors](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions/contributors.txt).

[View this page on GitHub](https://github.com/mdn/content/blob/main/files/en-us/mozilla/add-ons/webextensions/manifest.json/permissions/index.md?plain=1) • [Report a problem with this content](https://github.com/mdn/content/issues/new?template=page-report.yml&mdn-url=https%3A%2F%2Fdeveloper.mozilla.org%2Fen-US%2Fdocs%2FMozilla%2FAdd-ons%2FWebExtensions%2Fmanifest.json%2Fpermissions&metadata=%3C%21--+Do+not+make+changes+below+this+line+--%3E%0A%3Cdetails%3E%0A%3Csummary%3EPage+report+details%3C%2Fsummary%3E%0A%0A*+Folder%3A+%60en-us%2Fmozilla%2Fadd-ons%2Fwebextensions%2Fmanifest.json%2Fpermissions%60%0A*+MDN+URL%3A+https%3A%2F%2Fdeveloper.mozilla.org%2Fen-US%2Fdocs%2FMozilla%2FAdd-ons%2FWebExtensions%2Fmanifest.json%2Fpermissions%0A*+GitHub+URL%3A+https%3A%2F%2Fgithub.com%2Fmdn%2Fcontent%2Fblob%2Fmain%2Ffiles%2Fen-us%2Fmozilla%2Fadd-ons%2Fwebextensions%2Fmanifest.json%2Fpermissions%2Findex.md%0A*+Last+commit%3A+https%3A%2F%2Fgithub.com%2Fmdn%2Fcontent%2Fcommit%2F674fbb492c76a45adf433810f0f5737a0405bd9c%0A*+Document+last+modified%3A+2026-10-08T15%3A49%3A35.000Z%0A%0A%3C%2Fdetails%3E)
