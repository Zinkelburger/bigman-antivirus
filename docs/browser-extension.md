# Browser extension

> Part of the [Bigman README](../README.md).

```sh
cd browser-extension
npm ci --ignore-scripts --no-audit --no-fund
npm run build
```

Load the `browser-extension` directory as an unpacked extension in Chromium's Extensions page. The generated `dist/link-detector.js` bundles the maintained Public Suffix List library; the unbuilt source alone cannot be loaded. Copy its extension ID, then register the installed native host:

```sh
./install-native-messaging.sh YOUR_32_CHARACTER_EXTENSION_ID
```

Registration uses an explicit Chrome extension origin and the actual binary location. Chrome and Chromium user host manifests are generated; wildcard origins and the old checked-in machine-specific manifest were removed. Firefox, Flatpak browser bridges and other browsers need separate packaging and are not claimed supported here.

In the popup, check the desktop connection. Download scanning is off until explicitly enabled in Settings. It scans completed downloads, queues jobs in local extension storage, and reports missing hosts/failures. It does not prevent auto-opening a download. The native host accepts only regular download files under `~/Downloads`; arbitrary folders require explicit desktop selection. Installer preview requests from the browser accept HTTPS URLs only. Repository checks accept only a repository address on those four hosts; the popup reduces the page's address to the repository (a `/tree/main/src` page becomes `OWNER/REPO`) and sends nothing else about the page. Native messages are capped at 64 KiB and report frames stay below Chrome's response limit. There is no TCP listener.

Blocklists are imported locally as domains or exact HTTP(S) URLs. Domain entries match on label boundaries and include subdomains. Imports replace the old list and display their timestamp; they do not refresh automatically. Hostname exceptions last one hour and cannot suppress blocklist findings. “Continue once” is a deliberate bypass. Internationalized domains and HTTP alone receive informational results, not phishing verdicts.

The main-page popup summarizes the main document. Content scripts also check accessible frames and recheck links at click time. The extension cannot intercept all scripted navigation, redirects, typed addresses, forms, inaccessible frames or privileged browser pages. In-page markings are advisory and can be removed by a hostile page. Browser-native protections should stay enabled.
