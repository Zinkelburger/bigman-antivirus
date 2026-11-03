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
    const handlers = {}, scans = [], notices = [];
    let holdWrite = null;
    const chrome = {
        storage: { local: {
            get: async keys => Object.fromEntries(keys.filter(k => k in state).map(k => [k, structuredClone(state[k])])),
            set: async values => { if (holdWrite) await holdWrite(values); Object.assign(state, structuredClone(values)); }
        } },
        runtime: {
            id: 'test',
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
    return { state, handlers, scans, notices, chrome, hold: fn => holdWrite = fn };
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
