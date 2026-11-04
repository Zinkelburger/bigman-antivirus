const DEFAULTS = { blockedDomains: [], blockedUrls: [], exceptions: {}, feedUpdated: null, scanDownloads: false, downloadQueue: [] };
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
    if (request.action === 'warn' && sender.tab) {
        let u; try { u = new URL(request.url); } catch { respond({ error: 'Invalid URL' }); return false; }
        if (!['https:', 'http:'].includes(u.protocol)) { chrome.notifications.create({ type: 'basic', iconUrl: 'icons/128.png', title: 'Bigman blocked an active link', message: 'This link invokes code or opens a local resource. Inspect it manually before proceeding.' }); respond({ error: 'Active/local URL blocked. Inspect it manually.' }); return false; }
        const key = crypto.randomUUID();
        chrome.storage.session.set({ [key]: { url: u.href, reason: String(request.reason).slice(0, 1000), rule: request.rule, created: Date.now() } })
            .then(() => chrome.tabs.create({ url: chrome.runtime.getURL(`warning.html?id=${key}`) })).then(() => respond({ ok: true })).catch(e => respond({ error: e.message })); return true;
    }
    // Native operations are exposed only to extension-owned pages, not page content scripts.
    if (request.action === 'native' && !sender.tab && sender.url?.startsWith(chrome.runtime.getURL(''))) {
        if (!['health', 'inspect-url', 'repo-check'].includes(request.request?.op)) return false;
        native(request.request).then(async r => { await chrome.storage.local.set({ nativeHealth: { ...r, checkedAt: Date.now() } }); respond(r); }); return true;
    }
    return false;
});
async function resumeQueue() {
    try { await drain(); }
    catch (error) { await chrome.storage.local.set({ nativeHealth: { status: 'INCOMPLETE', reason: error.message, checkedAt: Date.now() } }); }
}
chrome.alarms.onAlarm.addListener(alarm => { if (alarm.name === 'download-queue') resumeQueue(); });
chrome.runtime.onInstalled.addListener(() => chrome.alarms.create('download-queue', { periodInMinutes: 1 }));
chrome.runtime.onStartup.addListener(resumeQueue);
chrome.alarms.create('download-queue', { periodInMinutes: 1 });
resumeQueue();
