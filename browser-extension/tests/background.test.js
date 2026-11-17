const fs = require('fs');
const vm = require('vm');
const path = require('path');
const source = fs.readFileSync(path.join(__dirname, '../background.js'), 'utf8');
const tick = () => new Promise(resolve => setImmediate(resolve));
async function until(predicate) {
    for (let i = 0; i < 100; i++) { if (predicate()) return; await tick(); }
    throw new Error('Background operation did not finish');
}
function fixture() {
    const state = { scanDownloads: true, downloadQueue: [{ id: 1, path: '/home/test/Downloads/first' }] };
    const handlers = {}, scans = [], notices = [], session = {}, opened = [];
    let holdWrite = null;
    const chrome = {
        storage: { local: {
            get: async keys => Object.fromEntries(keys.filter(k => k in state).map(k => [k, structuredClone(state[k])])),
            set: async values => { if (holdWrite) await holdWrite(values); Object.assign(state, structuredClone(values)); }
        }, session: {
            get: async keys => Object.fromEntries([].concat(keys).filter(k => k in session).map(k => [k, structuredClone(session[k])])),
            set: async values => { Object.assign(session, structuredClone(values)); },
            remove: async keys => { for (const k of [].concat(keys)) delete session[k]; }
        } },
        tabs: {
            create: async value => opened.push({ how: 'create', ...value }), update: async (id, value) => opened.push({ how: 'update', id, ...value }),
            onCreated: { addListener: fn => handlers.tabCreated = fn }, onRemoved: { addListener: fn => handlers.tabRemoved = fn }
        },
        runtime: {
            id: 'test',
            getURL: path => `chrome-extension://test/${path}`,
            sendNativeMessage: (_host, request, callback) => scans.push({ request, callback }),
            onMessage: { addListener: fn => handlers.message = fn },
            onInstalled: { addListener: () => {} },
            onStartup: { addListener: fn => handlers.startup = fn }
        },
        downloads: { onChanged: { addListener: fn => handlers.download = fn }, search: async ({ id }) => [{ id, filename: '/home/test/Downloads/second' }] },
        alarms: { create: () => {}, onAlarm: { addListener: fn => handlers.alarm = fn } },
        notifications: { create: async value => notices.push(value) }
    };
    vm.runInNewContext(source, { chrome, URL, crypto: require('crypto').webcrypto });
    return { state, session, opened, handlers, scans, notices, chrome, hold: fn => holdWrite = fn };
}

test('overlapping completion and enqueue preserve both downloads without rescanning the first', async () => {
    const f = fixture();
    await until(() => f.scans.length === 1);
    let release, writing = false;
    const gate = new Promise(resolve => release = resolve);
    f.hold(async values => { if (values.downloadQueue?.some(j => j.id === 2)) { writing = true; await gate; } });
    f.handlers.download({ id: 2, state: { current: 'complete' } });
    await until(() => writing);
    f.scans[0].callback({ status: 'NO_FINDINGS', reason: 'Fixture result' });
    for (let i = 0; i < 10; i++) await tick();
    release();
    await until(() => f.scans.length === 2);
    expect(f.scans[1].request.filePath).toBe('/home/test/Downloads/second');
    f.scans[1].callback({ status: 'NO_FINDINGS', reason: 'Fixture result' });
    await until(() => f.state.downloadQueue.length === 0);
    expect(f.scans).toHaveLength(2);
});

test('missing native host leaves visible incomplete status and notification', async () => {
    const f = fixture();
    await until(() => f.scans.length === 1);
    f.chrome.runtime.lastError = { message: 'Native host not installed' };
    f.scans[0].callback(undefined);
    delete f.chrome.runtime.lastError;
    await until(() => f.notices.length === 1);
    expect(f.state.nativeHealth.status).toBe('INCOMPLETE');
    expect(f.state.nativeHealth.reason).toContain('not installed');
    expect(f.state.downloadQueue).toEqual([]);
});

test('repository checks go to the desktop only from extension pages', async () => {
    const f = fixture();
    f.chrome.runtime.getURL = path => `chrome-extension://test/${path}`;
    await until(() => f.scans.length === 1);
    const request = { action: 'native', request: { op: 'repo-check', url: 'https://github.com/owner/repo' } };
    // A page's content script cannot reach the native host.
    expect(f.handlers.message(request, { id: 'test', tab: { id: 1 }, url: 'https://github.com/owner/repo' }, () => {})).toBe(false);
    expect(f.scans.length).toBe(1);
    const respond = jest.fn();
    expect(f.handlers.message(request, { id: 'test', url: 'chrome-extension://test/popup.html' }, respond)).toBe(true);
    expect(f.scans[1].request).toEqual({ op: 'repo-check', url: 'https://github.com/owner/repo' });
    f.scans[1].callback({ status: 'FINDINGS', reason: 'Findings to review' });
    await until(() => respond.mock.calls.length === 1);
    expect(respond.mock.calls[0][0].status).toBe('FINDINGS');
    // Other operations stay closed to the popup.
    expect(f.handlers.message({ action: 'native', request: { op: 'scan', filePath: '/etc/passwd' } }, { id: 'test', url: 'chrome-extension://test/popup.html' }, () => {})).toBe(false);
});

// Sends a message as a content script in tab `tab` (frame 0 unless given) and waits for any reply.
async function send(f, request, tab, extra = {}) {
    let reply;
    const respond = value => { reply = value; };
    const async = f.handlers.message(request, { id: 'test', tab, frameId: 0, ...extra }, respond);
    if (async) await until(() => reply !== undefined);
    else for (let i = 0; i < 5; i++) await tick();
    return reply;
}

const search = 'https://www.google.com/search?q=brew';
const settle = async () => { for (let i = 0; i < 20; i++) await tick(); };

test('an ad clicked in the tab is offered to the pages that tab opens, until it is settled', async () => {
    const f = fixture();
    await send(f, { action: 'adClick', shown: ['brew.sh'], via: ['trk.example'], newTab: false }, { id: 4 }, { url: search });
    await settle();
    expect(f.session['ad:4']).toMatchObject({ shown: ['brew.sh'], via: ['trk.example'], from: search });
    // An iframe cannot record one.
    await send(f, { action: 'adClick', shown: ['x.example'] }, { id: 5 }, { frameId: 3, url: search });
    await settle();
    expect(f.session['ad:5']).toBeUndefined();
    // Asking does not use it up: a tracking page on the way leaves it for the next page.
    expect(await send(f, { action: 'adLanding' }, { id: 4 })).toMatchObject({ key: 'ad:4' });
    expect(await send(f, { action: 'adLanding' }, { id: 4 })).toMatchObject({ key: 'ad:4' });
    // Another tab cannot settle this tab's record.
    await send(f, { action: 'adLanded', key: 'ad:4' }, { id: 9 });
    await settle();
    expect(f.session['ad:4']).toBeDefined();
    await send(f, { action: 'adLanded', key: 'ad:4' }, { id: 4 });
    await settle();
    expect(f.session['ad:4']).toBeUndefined();
});

test('ads opened into new tabs go to those tabs in order, whichever loads first', async () => {
    const f = fixture();
    await send(f, { action: 'adClick', shown: ['nordvpn.com'], newTab: true }, { id: 4 }, { url: search });
    await send(f, { action: 'adClick', notAd: true, newTab: true }, { id: 4 }, { url: search });
    await send(f, { action: 'adClick', shown: ['expressvpn.com'], newTab: true }, { id: 4 }, { url: search });
    await settle();
    expect(f.session['ad:4']).toBeUndefined();
    f.handlers.tabCreated({ id: 10, openerTabId: 4 });
    f.handlers.tabCreated({ id: 11, openerTabId: 4 });
    f.handlers.tabCreated({ id: 12, openerTabId: 4 });
    await settle();
    expect(f.session['ad:10'].shown).toEqual(['nordvpn.com']);
    expect(f.session['ad:11']).toBeUndefined();
    expect(f.session['ad:12'].shown).toEqual(['expressvpn.com']);
    expect((await send(f, { action: 'adLanding' }, { id: 12 })).record.shown).toEqual(['expressvpn.com']);
    // A tab opened by something else gets nothing.
    f.handlers.tabCreated({ id: 13, openerTabId: 4 });
    f.handlers.tabCreated({ id: 14 });
    await settle();
    expect(f.session['ad:13']).toBeUndefined();
    expect(f.session['ad:14']).toBeUndefined();
});

test('a new tab created just before its click message arrives still gets the ad', async () => {
    const f = fixture();
    f.handlers.tabCreated({ id: 20, openerTabId: 4 });
    await settle();
    await send(f, { action: 'adClick', shown: ['brew.sh'], newTab: true }, { id: 4 }, { url: search });
    await settle();
    expect(f.session['ad:20'].shown).toEqual(['brew.sh']);
    // Only briefly: a tab opened earlier is not given a later click.
    f.handlers.tabCreated({ id: 21, openerTabId: 4 });
    await settle();
    f.session['opened:4'][0].at -= 3000;
    await send(f, { action: 'adClick', shown: ['late.example'], newTab: true }, { id: 4 }, { url: search });
    await settle();
    expect(f.session['ad:21']).toBeUndefined();
});

test('an old ad click is forgotten, results pages clear the tab, and odd hosts are dropped', async () => {
    const f = fixture();
    await send(f, { action: 'adClick', shown: ['ok.example', 7, null, 'x'.repeat(400)], via: 'nope', newTab: false }, { id: 2 }, { url: 'chrome://newtab/' });
    await settle();
    expect(f.session['ad:2']).toMatchObject({ shown: ['ok.example', 'x'.repeat(253)], via: [], from: null });
    f.session['ad:2'].created -= 61000;
    expect(await send(f, { action: 'adLanding' }, { id: 2 })).toEqual({});
    await settle();
    expect(f.session['ad:2']).toBeUndefined();
    f.session['adq:3'] = [{ shown: ['a.example'], created: Date.now() }];
    f.session['ad:3'] = { shown: ['b.example'], created: Date.now() };
    await send(f, { action: 'adClear', all: true }, { id: 3 });
    await settle();
    expect(f.session['ad:3']).toBeUndefined();
    expect(f.session['adq:3']).toBeUndefined();
    f.session['ad:7'] = { shown: ['c.example'], created: Date.now() };
    f.handlers.tabRemoved(7);
    await settle();
    expect(f.session['ad:7']).toBeUndefined();
});

test('a landing warning replaces the page in its tab; a link warning opens beside it', async () => {
    const f = fixture();
    const warn = { action: 'warn', url: 'https://brewe.sh/', reason: 'Ad mismatch', rule: 'ad.landing-mismatch' };
    await send(f, { ...warn, here: true, back: search }, { id: 6 });
    expect(f.opened[0]).toMatchObject({ how: 'update', id: 6 });
    const saved = Object.values(f.session).find(v => v.rule === 'ad.landing-mismatch');
    expect(saved.back).toBe(search);
    expect(f.opened[0].url).toMatch(/^chrome-extension:\/\/test\/warning\.html\?id=/);
    await send(f, { ...warn, back: 'javascript:alert(1)' }, { id: 6 });
    expect(f.opened[1]).toMatchObject({ how: 'create' });
    expect(Object.values(f.session).filter(v => v.rule === 'ad.landing-mismatch').map(v => v.back)).toEqual([search, null]);
    // A frame cannot replace the page it sits in.
    await send(f, { ...warn, here: true }, { id: 6 }, { frameId: 2 });
    expect(f.opened[2]).toMatchObject({ how: 'create' });
});
