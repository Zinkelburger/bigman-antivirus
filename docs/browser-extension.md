# Browser extension

> Part of the [Bigman README](../README.md).

One extension works in Chrome, Chromium and Firefox (140 or newer). Build it first:

```sh
cd browser-extension
npm ci --ignore-scripts --no-audit --no-fund
npm run build
```

The generated `dist/link-detector.js` bundles the maintained Public Suffix List library; the unbuilt source alone cannot be loaded.

## Chrome and Chromium

Load the `browser-extension` directory as an unpacked extension in the Extensions page (developer mode on). Copy its extension ID, then register the installed native host:

```sh
./install-native-messaging.sh YOUR_32_CHARACTER_EXTENSION_ID
```

Registration uses an explicit Chrome extension origin and the actual binary location. Chrome and Chromium user host manifests are generated; wildcard origins and the old checked-in machine-specific manifest were removed.

## Firefox

Firefox installs only signed add-ons permanently. For now:

- **To try it:** open `about:debugging#/runtime/this-firefox`, choose **Load Temporary Add-on…**, and pick `browser-extension/manifest.json`. It stays until Firefox restarts.
- **To keep it:** Firefox Developer Edition, Nightly and ESR can turn off `xpinstall.signatures.required` in `about:config` and install the release zip. Release Firefox needs the zip signed by Mozilla (an unlisted, self-distributed signing on addons.mozilla.org); that is not set up yet.

Firefox asks for access to your data on all websites. If that access is off (Add-ons and themes → Bigman → Permissions), the extension cannot mark links or sponsored results.

The add-on ID is fixed (`link-detector@bigman-antivirus`), so the native host needs no ID:

```sh
./install-native-messaging.sh                   # Firefox only
./install-native-messaging.sh CHROME_ID         # Firefox, and Chrome and Chromium
```

The app's Setup tab does the same, with the Chrome ID optional. It writes `~/.mozilla/native-messaging-hosts/com.bigman.pdf_scanner.json`, which allows only that add-on ID. Firefox 159 reads only that location, even when its profiles are under `~/.config/mozilla`; a copy goes to `~/.config/mozilla/native-messaging-hosts/` for when Firefox reads that too. Firefox from Snap or Flatpak runs in a sandbox that needs its own setup and is not covered.

## Sponsored search results

On Google, Bing and DuckDuckGo results pages the extension finds the ads and, by default, gives each an amber outline and the label "Ad · not a search result. Check the web address before you download anything." Fake download sites buy these ads because an ad sits above the real result, and the address an ad shows can differ from where it leads.

In Settings, **Sponsored search results** can be:

- **Highlight them** (default).
- **Hide them.** A small note at the bottom left says how many were hidden, with **Show them** for that page.
- **Leave them as they are.** Only their look changes; the check below still runs.

It finds ads by what the engines use to run them, not by their styling: Google's ad attributes (`data-text-ad`, `data-pcu`, and `/aclk` click links), Bing's `/aclk` links and ad lists, and DuckDuckGo's `y.js?ad_domain=` links. A "Sponsored" or "Ad" label near an ad link marks the ad around it, which covers ads inside Google's AI Overview. The rules are tested against real results pages saved in `browser-extension/tests/fixtures/serp/` (`scripts/trim-serp.js` reduces a saved page to the markup the rules read). Engines change their pages; when ads stop being marked, save a fresh page and update `search-ads.js`.

**Where an ad leads.** Clicking an ad remembers the site it displayed (Google's displayed domain, the address Bing prints, DuckDuckGo's `ad_domain`) and the hosts the engine declares on the way, such as the advertiser's tracking template. For a minute, each page the click opens is checked. Reaching the displayed site ends the check. A page on an engine, on a known click tracker (`doubleclick.net`, `dartsearch.net` and a few others) or on a declared host is a step on the way. Any other site is replaced by a warning that names both sites, with **Back to search results**, **Continue once** and **Allow this hostname for one hour**. So a tracking page that forwards to a lookalike still ends in the warning. An ad that names no site is not checked.

Ads opened in a new tab (middle-click, Ctrl-click, or results set to open in new tabs) are checked in the tab they open, each with its own ad. "Open link in new tab" from the right-click menu is not seen, so that tab is not checked. Back, forward and reload are not arrivals from an ad. Loading a results page, or clicking anything else on one, forgets the ad. The check runs whatever the display setting, and after the page has loaded, so it cannot stop a download the page starts at once.

On results pages, the engine's own links (its redirects through `google.com/goto`, `bing.com/aclk` and so on) are not checked for a visible address that differs from the destination: the address they show is the result's, not their own.

## Downloads, installers and repositories

In the popup, check the desktop connection. Download scanning is off until explicitly enabled in Settings. It scans completed downloads, queues jobs in local extension storage, and reports missing hosts/failures. It does not prevent auto-opening a download. The native host accepts only regular download files under `~/Downloads`; arbitrary folders require explicit desktop selection. Installer preview requests from the browser accept HTTPS URLs only. Repository checks accept only a repository address on those four hosts; the popup reduces the page's address to the repository (a `/tree/main/src` page becomes `OWNER/REPO`) and sends nothing else about the page. Native messages are capped at 64 KiB and report frames stay below Chrome's response limit. There is no TCP listener.

## Blocklists and limits

Blocklists are imported locally as domains or exact HTTP(S) URLs. Domain entries match on label boundaries and include subdomains. Imports replace the old list and display their timestamp; they do not refresh automatically. Hostname exceptions last one hour and cannot suppress blocklist findings. “Continue once” is a deliberate bypass. Internationalized domains and HTTP alone receive informational results, not phishing verdicts. A `javascript:void(0)` link does nothing by itself and is not flagged; other `javascript:` links are.

The main-page popup summarizes the main document. Content scripts also check accessible frames and recheck links at click time. The extension cannot intercept all scripted navigation, redirects, typed addresses, forms, inaccessible frames or privileged browser pages. In-page markings are advisory and can be removed by a hostile page. Browser-native protections should stay enabled.
