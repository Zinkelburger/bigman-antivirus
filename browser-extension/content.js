(() => {
    if (globalThis.__bigmanStarted) return;
    globalThis.__bigmanStarted = true;
    let detector;
    let results = { total: 0, suspicious: 0, details: [], incomplete: true, lastScanTime: null };
    let timer;
    let scanning = false;
    let again = false;
    async function configure() {
        const reply = await chrome.runtime.sendMessage({ action: 'getSettings' });
        if (!reply?.settings) throw new Error('Extension settings unavailable');
        detector = new globalThis.BigmanLinkDetector(undefined, reply.settings);
    }
    async function scan() {
        if (scanning) { again = true; return; }
        scanning = true;
        try {
            await configure();
            const all = [...document.querySelectorAll('a[href]')];
            const links = all.slice(0, 3000);
            const next = { total: links.length, suspicious: 0, details: [], incomplete: all.length > links.length, lastScanTime: new Date().toISOString() };
            for (const link of links) {
                const result = detector.analyzeLink(link.textContent || '', link.href, document.baseURI);
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
        const result = detector?.analyzeLink(link.textContent || '', link.href, document.baseURI);
        if (!result || result.isSuspicious) {
            event.preventDefault(); event.stopImmediatePropagation();
            chrome.runtime.sendMessage({ action: 'warn', url: link.href, reason: result?.reason || 'Link check is not ready', rule: result?.rule || 'check.unavailable' }).catch(() => {});
        }
    }
    document.addEventListener('click', intercept, true);
    document.addEventListener('auxclick', intercept, true);
    chrome.runtime.onMessage.addListener((request, _sender, respond) => {
        if (request.action === 'getScanResults') { respond({ success: true, results }); return false; }
        if (request.action === 'rescan') { scan().then(() => respond({ success: true, results })); return true; }
        return false;
    });
    schedule();
})();
