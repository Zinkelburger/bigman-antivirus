(() => {
    if (globalThis.__bigmanStarted) return;
    globalThis.__bigmanStarted = true;
    const ads = globalThis.BigmanSearchAds;
    const topFrame = window === window.top;
    let detector;
    let settings = {};
    let results = { total: 0, suspicious: 0, details: [], incomplete: true, lastScanTime: null, ads: null };
    let timer;
    let scanning = false;
    let again = false;
    let showHidden = false;
    async function configure() {
        const reply = await chrome.runtime.sendMessage({ action: 'getSettings' });
        if (!reply?.settings) throw new Error('Extension settings unavailable');
        settings = reply.settings;
        detector = new globalThis.BigmanLinkDetector(undefined, settings);
    }
    function engine() { return topFrame ? ads.engineFor(location.href) : null; }
    // A results page's own links (its redirects, ad clicks) show the result's
    // address, not their own, so the visible-address rule does not apply to them.
    function analyze(link) {
        let own = false;
        if (engine()) try { const host = new URL(link.href, document.baseURI).hostname; own = host === location.hostname || ads.isRedirector(host); } catch { /* invalid; analyzeLink reports it */ }
        return detector.analyzeLink(own ? '' : (link.textContent || ''), link.href, document.baseURI);
    }

    // Sponsored results: marked or hidden in place; the page itself is not changed otherwise.
    let notice = null;
    let noticeCount = 0;
    let noticeClosed = false;
    function markAds() {
        const name = engine();
        const chosen = settings.searchAds || 'highlight';
        const mode = !name ? 'off' : chosen === 'hide' && showHidden ? 'highlight' : chosen;
        const found = mode === 'off' ? [] : ads.findAds(document, name);
        const current = new Set(found);
        for (const el of document.querySelectorAll('[data-bigman-ad]')) if (!current.has(el)) el.removeAttribute('data-bigman-ad');
        for (const el of found) if (el.getAttribute('data-bigman-ad') !== mode) el.setAttribute('data-bigman-ad', mode);
        showNotice(mode === 'hide' ? found.length : 0);
        return name ? { engine: name, mode, count: found.length } : null;
    }
    function showNotice(count) {
        if (count === noticeCount) return;
        noticeCount = count;
        notice?.remove();
        notice = null;
        if (!count || noticeClosed) return;
        // Outside <body> and in a closed shadow root, so the page's scripts and styles leave it alone.
        notice = document.createElement('bigman-ad-notice');
        const shadow = notice.attachShadow({ mode: 'closed' });
        const style = document.createElement('style');
        style.textContent = ':host{all:initial;position:fixed;z-index:2147483647;left:16px;bottom:16px}'
            + 'div{display:flex;align-items:center;gap:10px;max-width:calc(100vw - 32px);box-sizing:border-box;padding:8px 10px 8px 14px;border-radius:8px;background:#1c1917;color:#fafaf9;border:1px solid #d97706;font:14px/1.4 system-ui,sans-serif;box-shadow:0 4px 16px rgba(0,0,0,.3)}'
            + 'button{font:inherit;cursor:pointer;border-radius:6px;border:1px solid #a8a29e;background:transparent;color:inherit;padding:3px 10px}'
            + 'button:focus-visible{outline:2px solid #fbbf24;outline-offset:2px}'
            + '.close{border:0;padding:3px 6px;font-size:18px;line-height:1}';
        const box = document.createElement('div');
        box.setAttribute('role', 'status');
        const text = document.createElement('span');
        text.textContent = `Bigman hid ${count} sponsored ${count === 1 ? 'result' : 'results'}.`;
        const show = document.createElement('button');
        show.type = 'button';
        show.textContent = 'Show them';
        show.onclick = () => { showHidden = true; schedule(); };
        const close = document.createElement('button');
        close.type = 'button';
        close.className = 'close';
        close.setAttribute('aria-label', 'Close');
        close.textContent = '×';
        close.onclick = () => { noticeClosed = true; notice?.remove(); notice = null; };
        box.append(text, show, close);
        shadow.append(style, box);
        document.documentElement.append(notice);
    }

    async function scan() {
        if (scanning) { again = true; return; }
        scanning = true;
        try {
            await configure();
            const adInfo = markAds();
            const all = [...document.querySelectorAll('a[href]')];
            const links = all.slice(0, 3000);
            const next = { total: links.length, suspicious: 0, details: [], incomplete: all.length > links.length, lastScanTime: new Date().toISOString(), ads: adInfo };
            for (const link of links) {
                const result = analyze(link);
                if (result.level === 'unavailable') next.incomplete = true;
                if (result.isSuspicious) {
                    next.suspicious++;
                    next.details.push({ text: link.textContent.slice(0, 500), url: link.href, ...result });
                    link.setAttribute('data-bigman-warning', result.reason);
                } else link.removeAttribute('data-bigman-warning');
            }
            const page = detector.analyzePage(document.body?.innerText || document.body?.textContent || '');
            if (page) { next.suspicious++; next.details.push({ text: 'Page instructions', url: location.href, ...page }); }
            results = next;
            chrome.runtime.sendMessage({ action: 'badge', count: next.suspicious, incomplete: next.incomplete }).catch(() => {});
        } catch (error) {
            results = { ...results, incomplete: true, error: error.message };
        } finally {
            scanning = false;
            if (again) { again = false; schedule(); }
        }
    }
    function schedule() { clearTimeout(timer); timer = setTimeout(scan, 200); }
    const observer = new MutationObserver(schedule);
    observer.observe(document.documentElement, { subtree: true, childList: true, characterData: true, attributes: true, attributeFilter: ['href'] });
    // Re-evaluate the current href at interaction time, including image-only links.
    function intercept(event) {
        if (!event.isTrusted) return;
        const link = event.composedPath().find(n => n?.tagName === 'A' && n.href);
        if (!link) return;
        rememberAdClick(event, link);
        const result = detector && analyze(link);
        if (!result || result.isSuspicious) {
            event.preventDefault(); event.stopImmediatePropagation();
            chrome.runtime.sendMessage({ action: 'warn', url: link.href, reason: result?.reason || 'Link check is not ready', rule: result?.rule || 'check.unavailable' }).catch(() => {});
        }
    }
    // An ad click records where the ad said it leads, so the page it opens can be
    // compared with that. Any other click on a results page forgets the last ad.
    // Ads count whatever the display setting, and before the first scan marks them.
    function rememberAdClick(event, link) {
        const name = engine();
        if (!name || (event.type === 'auxclick' && event.button !== 1)) return;
        const target = link.target || document.querySelector('base[target]')?.target || '';
        const newTab = event.type === 'auxclick' || !!(event.ctrlKey || event.metaKey || event.shiftKey) || target.toLowerCase() === '_blank';
        const unit = link.closest('[data-bigman-ad]') || ads.findAds(document, name).find(u => u === link || u.contains(link));
        // A non-ad link opened in a new tab still takes its place in line, so the next new tab is not given an ad's record.
        if (!unit) { chrome.runtime.sendMessage(newTab ? { action: 'adClick', notAd: true, newTab } : { action: 'adClear' }).catch(() => {}); return; }
        const { shown, via } = ads.destinationsFor(link, unit, name);
        chrome.runtime.sendMessage({ action: 'adClick', shown, via, newTab }).catch(() => {});
    }
    // Each top-level page a tab opens after an ad click: the site the ad showed ends
    // the check, an engine or declared tracking page is a step on the way, and any
    // other site gets the warning. Back, forward and reload are not new arrivals.
    async function checkLanding() {
        if (!topFrame) return;
        if (engine()) { chrome.runtime.sendMessage({ action: 'adClear', all: true }).catch(() => {}); return; }
        const navigation = performance.getEntriesByType?.('navigation')?.[0]?.type;
        if (navigation === 'back_forward' || navigation === 'reload') return;
        const host = location.hostname.toLowerCase();
        if (!host || ads.isRedirector(host)) return;
        const reply = await chrome.runtime.sendMessage({ action: 'adLanding' });
        if (!reply?.key) return;
        const done = () => chrome.runtime.sendMessage({ action: 'adLanded', key: reply.key }).catch(() => {});
        if (!detector) await configure();
        const here = detector.registrable(host);
        const has = list => (list || []).some(h => detector.registrable(h) === here);
        const { shown = [], via = [], from } = reply.record;
        if (!shown.length || has(shown) || detector.exceptions[host] > Date.now()) { done(); return; }
        if (has(via)) return;
        done();
        chrome.runtime.sendMessage({
            action: 'warn', here: true, url: location.href, back: from, rule: 'ad.landing-mismatch',
            reason: `This sponsored search result showed ${shown[0]}, but it brought you to ${host}. Ads can show one address and send you to another. Do not download anything or paste commands from this page unless you are sure it is the real site.`
        }).catch(() => {});
    }
    document.addEventListener('click', intercept, true);
    document.addEventListener('auxclick', intercept, true);
    chrome.runtime.onMessage.addListener((request, _sender, respond) => {
        if (request.action === 'getScanResults') { respond({ success: true, results }); return false; }
        if (request.action === 'rescan') { scan().then(() => respond({ success: true, results })); return true; }
        return false;
    });
    checkLanding().catch(() => {});
    schedule();
})();
