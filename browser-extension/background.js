const DEFAULTS = { blockedDomains: [], blockedUrls: [], exceptions: {}, feedUpdated: null, scanDownloads: false, downloadQueue: [], searchAds: 'highlight' };
const AD_CLICK_TTL = 60000;
let queueRunning = false;
async function settings() { return { ...DEFAULTS, ...await chrome.storage.local.get(Object.keys(DEFAULTS)) }; }
// Serialize storage read/modify/write across download events and alarm-driven scans.
let queueWrites = Promise.resolve();
function updateQueue(change) {
    const operation = queueWrites.then(async () => {
        const current = await settings();
        await chrome.storage.local.set(await change(current));
    });
    queueWrites = operation.catch(() => {});
    return operation;
}
function native(request) {
    return new Promise(resolve => chrome.runtime.sendNativeMessage('com.bigman.pdf_scanner', request, response => {
        resolve(chrome.runtime.lastError ? { status: 'INCOMPLETE', reason: chrome.runtime.lastError.message } : (response || { status: 'INCOMPLETE', reason: 'Empty host response' }));
    }));
}
async function drain() {
    if (queueRunning) return;
    queueRunning = true;
    try {
        while (true) {
            const current = await settings();
            if (!current.scanDownloads || !current.downloadQueue.length) return;
            const job = current.downloadQueue[0];
            const result = await native({ op: 'scan', filePath: job.path });
            // Keep the job stored until its result exists. A worker restart can retry a scan.
            await updateQueue(latest => ({ downloadQueue: latest.downloadQueue.filter(j => j.id !== job.id), nativeHealth: { ...result, checkedAt: Date.now() } }));
            if (result.status !== 'NO_FINDINGS') await chrome.notifications.create({ type: 'basic', iconUrl: 'icons/128.png', title: 'Bigman download scan', message: `${result.status}: ${result.reason}`.slice(0, 500) });
        }
    } finally { queueRunning = false; }
}
let enqueue = Promise.resolve();
chrome.downloads.onChanged.addListener(delta => {
    if (delta.state?.current !== 'complete') return;
    enqueue = enqueue.then(async () => {
        const [item] = await chrome.downloads.search({ id: delta.id });
        if (!item) return;
        await updateQueue(s => {
            if (!s.scanDownloads) return {};
            if (s.downloadQueue.length >= 100) return { nativeHealth: { status: 'INCOMPLETE', reason: 'Download queue full; a file was not queued.', checkedAt: Date.now() } };
            return { downloadQueue: [...s.downloadQueue.filter(j => j.id !== item.id), { id: item.id, path: item.filename }] };
        });
    }).then(drain).catch(async error => chrome.storage.local.set({ nativeHealth: { status: 'INCOMPLETE', reason: error.message, checkedAt: Date.now() } }));
});
chrome.runtime.onMessage.addListener((request, sender, respond) => {
    if (sender.id !== chrome.runtime.id) return false;
    if (request.action === 'getSettings') { settings().then(s => respond({ settings: s })).catch(e => respond({ error: e.message })); return true; }
    if (request.action === 'badge' && sender.tab && (!sender.frameId || sender.frameId === 0)) {
        chrome.action.setBadgeText({ tabId: sender.tab.id, text: request.incomplete ? '?' : String(Math.min(99, Math.max(0, Number(request.count) || 0)) || '') });
        chrome.action.setBadgeBackgroundColor({ tabId: sender.tab.id, color: '#b45309' }); return false;
    }
    // Ad clicks, kept briefly so the pages they open can be checked against them.
    if (request.action === 'adClick' && sender.tab && !sender.frameId) {
        const hosts = list => (Array.isArray(list) ? list : []).filter(h => typeof h === 'string').slice(0, 20).map(h => h.slice(0, 253));
        let from = null;
        try { if (['https:', 'http:'].includes(new URL(sender.url).protocol)) from = sender.url; } catch { /* no way back */ }
        const record = request.notAd ? { notAd: true, created: Date.now() } : { shown: hosts(request.shown), via: hosts(request.via), from, created: Date.now() };
        adClick(sender.tab.id, record, !!request.newTab).catch(() => {});
        return false;
    }
    if (request.action === 'adClear' && sender.tab && !sender.frameId) {
        const keys = [`ad:${sender.tab.id}`];
        if (request.all) keys.push(`adq:${sender.tab.id}`, `opened:${sender.tab.id}`);
        adOp(() => chrome.storage.session.remove(keys)).catch(() => {}); return false;
    }
    if (request.action === 'adLanding' && sender.tab && !sender.frameId) {
        adLanding(sender.tab.id).then(respond).catch(() => respond({})); return true;
    }
    if (request.action === 'adLanded' && sender.tab && request.key === `ad:${sender.tab.id}`) { adOp(() => chrome.storage.session.remove(request.key)).catch(() => {}); return false; }
    if (request.action === 'warn' && sender.tab) {
        let u; try { u = new URL(request.url); } catch { respond({ error: 'Invalid URL' }); return false; }
        if (!['https:', 'http:'].includes(u.protocol)) { chrome.notifications.create({ type: 'basic', iconUrl: 'icons/128.png', title: 'Bigman blocked an active link', message: 'This link invokes code or opens a local resource. Inspect it manually before proceeding.' }); respond({ error: 'Active/local URL blocked. Inspect it manually.' }); return false; }
        const key = crypto.randomUUID();
        let back = null;
        try { const b = new URL(request.back); if (['https:', 'http:'].includes(b.protocol)) back = b.href; } catch { /* none */ }
        chrome.storage.session.set({ [key]: { url: u.href, reason: String(request.reason).slice(0, 1000), rule: request.rule, back: request.here ? back : null, created: Date.now() } })
            .then(() => {
                const url = chrome.runtime.getURL(`warning.html?id=${key}`);
                // A page that is itself the problem is replaced; a link's warning opens beside the page.
                return request.here && !sender.frameId ? chrome.tabs.update(sender.tab.id, { url }) : chrome.tabs.create({ url });
            }).then(() => respond({ ok: true })).catch(e => respond({ error: e.message })); return true;
    }
    // Native operations are exposed only to extension-owned pages, not page content scripts.
    if (request.action === 'native' && !sender.tab && sender.url?.startsWith(chrome.runtime.getURL(''))) {
        if (!['health', 'inspect-url', 'repo-check'].includes(request.request?.op)) return false;
        native(request.request).then(async r => { await chrome.storage.local.set({ nativeHealth: { ...r, checkedAt: Date.now() } }); respond(r); }); return true;
    }
    return false;
});
// Ad records: `ad:<tab>` is the ad whose pages that tab is opening. An ad opened
// into a new tab waits in `adq:<results tab>` until the browser creates the tab,
// which claims the oldest one; `opened:<results tab>` covers a tab created before
// the click's message arrived. Storage changes run one at a time.
const NEW_TAB_WINDOW = 10000;
const CREATED_FIRST_WINDOW = 2000;
let adWrites = Promise.resolve();
function adOp(change) {
    const operation = adWrites.then(change);
    adWrites = operation.catch(() => {});
    return operation;
}
const fresh = (list, age) => (Array.isArray(list) ? list : []).filter(r => Date.now() - (r.created ?? r.at) <= age);
function adClick(tab, record, newTab) {
    if (!newTab) return adOp(() => chrome.storage.session.set({ [`ad:${tab}`]: record }));
    return adOp(async () => {
        const stored = await chrome.storage.session.get([`adq:${tab}`, `opened:${tab}`]);
        const opened = fresh(stored[`opened:${tab}`], CREATED_FIRST_WINDOW);
        if (opened.length) {
            const [first, ...rest] = opened;
            await chrome.storage.session.set({ [`opened:${tab}`]: rest, ...(record.notAd ? {} : { [`ad:${first.tab}`]: record }) });
        } else await chrome.storage.session.set({ [`adq:${tab}`]: [...fresh(stored[`adq:${tab}`], NEW_TAB_WINDOW), record].slice(-10) });
    });
}
chrome.tabs.onCreated.addListener(tab => {
    if (tab.openerTabId === undefined || tab.openerTabId === tab.id) return;
    const opener = tab.openerTabId;
    adOp(async () => {
        const stored = await chrome.storage.session.get([`adq:${opener}`, `opened:${opener}`]);
        const queue = fresh(stored[`adq:${opener}`], NEW_TAB_WINDOW);
        if (queue.length) {
            const [first, ...rest] = queue;
            await chrome.storage.session.set({ [`adq:${opener}`]: rest, ...(first.notAd ? {} : { [`ad:${tab.id}`]: first }) });
        } else await chrome.storage.session.set({ [`opened:${opener}`]: [...fresh(stored[`opened:${opener}`], CREATED_FIRST_WINDOW), { tab: tab.id, at: Date.now() }].slice(-10) });
    }).catch(() => {});
});
chrome.tabs.onRemoved.addListener(tab => { adOp(() => chrome.storage.session.remove([`ad:${tab}`, `adq:${tab}`, `opened:${tab}`])).catch(() => {}); });
async function adLanding(tab) {
    const key = `ad:${tab}`;
    const record = (await chrome.storage.session.get(key))[key];
    if (!record) return {};
    if (Date.now() - record.created > AD_CLICK_TTL) { await adOp(() => chrome.storage.session.remove(key)); return {}; }
    return { key, record };
}
async function resumeQueue() {
    try { await drain(); }
    catch (error) { await chrome.storage.local.set({ nativeHealth: { status: 'INCOMPLETE', reason: error.message, checkedAt: Date.now() } }); }
}
chrome.alarms.onAlarm.addListener(alarm => { if (alarm.name === 'download-queue') resumeQueue(); });
chrome.runtime.onInstalled.addListener(() => chrome.alarms.create('download-queue', { periodInMinutes: 1 }));
chrome.runtime.onStartup.addListener(resumeQueue);
chrome.alarms.create('download-queue', { periodInMinutes: 1 });
resumeQueue();
