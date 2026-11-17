/** @jest-environment jsdom */
const fs = require('fs');
const path = require('path');
const LinkDetector = require('../link-detector');
const SearchAds = require('../search-ads');
const source = fs.readFileSync(require.resolve('../content.js'), 'utf8');
const wait = () => new Promise(resolve => setTimeout(resolve, 350));

// Shadow roots stay reachable here so the tests can press the notice's buttons.
const shadows = new WeakMap();
const attach = Element.prototype.attachShadow;
Element.prototype.attachShadow = function (init) { const root = attach.call(this, init); shadows.set(this, root); return root; };

// Runs content.js as if the tab were at `href`. Each run is a fresh page.
function load(href, { settings = {}, landing = {} } = {}) {
    delete globalThis.__bigmanStarted;
    const sent = [];
    const listeners = [];
    const handlers = {};
    const add = jest.spyOn(document, 'addEventListener').mockImplementation(function (type, fn, options) {
        handlers[type] = fn;
        return EventTarget.prototype.addEventListener.call(this, type, fn, options);
    });
    global.chrome = {
        runtime: {
            sendMessage: jest.fn(async request => {
                sent.push(request);
                if (request.action === 'getSettings') return { settings };
                if (request.action === 'adLanding') return landing;
                return {};
            }),
            onMessage: { addListener: fn => listeners.push(fn) }
        }
    };
    global.BigmanLinkDetector = LinkDetector;
    global.BigmanSearchAds = SearchAds;
    const url = new URL(href);
    new Function('location', source)({ href: url.href, hostname: url.hostname });
    add.mockRestore();
    // jsdom cannot make a trusted event, so clicks go straight to content.js's handler.
    const press = (link, init = {}) => handlers[init.type || 'click']({ isTrusted: true, type: 'click', button: 0, composedPath: () => [link], preventDefault() {}, stopImmediatePropagation() {}, ...init });
    return { sent, listeners, press, of: action => sent.filter(m => m.action === action) };
}
function fixture(name) {
    const html = fs.readFileSync(path.join(__dirname, 'fixtures/serp', `${name}.html`), 'utf8');
    document.body.innerHTML = new DOMParser().parseFromString(html, 'text/html').body.innerHTML;
}
const marked = mode => document.querySelectorAll(`[data-bigman-ad="${mode}"]`);
const click = (el, init = {}) => el.dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true, ...init }));

// A fresh document per test: the previous page's observer stays on the old root.
afterEach(() => {
    const html = document.createElement('html');
    html.append(document.createElement('head'), document.createElement('body'));
    document.replaceChild(html, document.documentElement);
});

test('marks Google ads, leaves Google\'s own result links alone, and records an ad click', async () => {
    fixture('google-vpn');
    const page = load('https://www.google.com/search?q=vpn');
    await wait();
    expect(marked('highlight')).toHaveLength(7);
    // Google sends every result through its own /goto link; the visible address is the result's.
    expect(document.querySelectorAll('[data-bigman-warning]')).toHaveLength(0);
    const respond = jest.fn();
    page.listeners[0]({ action: 'getScanResults' }, {}, respond);
    expect(respond.mock.calls[0][0].results.ads).toEqual({ engine: 'google', mode: 'highlight', count: 7 });

    const ad = marked('highlight')[1].querySelector('a[href]');
    page.press(ad);
    expect(page.of('adClick')).toEqual([{ action: 'adClick', shown: ['nordvpn.com'], via: [], newTab: false }]);
    page.press(ad, { type: 'auxclick', button: 1 });
    expect(page.of('adClick')[1].newTab).toBe(true);
    page.press(ad, { type: 'auxclick', button: 2 });
    expect(page.of('adClick')).toHaveLength(2);
    expect(page.of('warn')).toEqual([]);

    // Results set to open in a new tab.
    ad.target = '_blank';
    page.press(ad);
    expect(page.of('adClick')[2].newTab).toBe(true);
    ad.removeAttribute('target');

    const organic = [...document.querySelectorAll('a[href]')].find(a => a.textContent.includes('en.wikipedia.org'));
    const before = page.of('adClear').length;
    page.press(organic);
    expect(page.of('adClear')).toHaveLength(before + 1);
    // Opened in a new tab, it holds its place in line instead.
    page.press(organic, { ctrlKey: true });
    expect(page.of('adClick').at(-1)).toEqual({ action: 'adClick', notAd: true, newTab: true });
    expect(page.of('warn')).toEqual([]);
});

test('with ads left as they are, an ad click is still recorded', async () => {
    fixture('duckduckgo-vpn');
    const page = load('https://duckduckgo.com/?q=buy+vpn', { settings: { searchAds: 'off' } });
    await wait();
    expect(document.querySelectorAll('[data-bigman-ad]')).toHaveLength(0);
    page.press(document.querySelector('li[data-layout="ad"] a[href*="ad_domain=nordvpn.com"]'));
    expect(page.of('adClick')).toEqual([{ action: 'adClick', shown: ['nordvpn.com'], via: [], newTab: false }]);
});

test('hide mode hides ads, says so, and can show them for this page', async () => {
    fixture('bing-vpn');
    load('https://www.bing.com/search?q=buy+vpn', { settings: { searchAds: 'hide' } });
    await wait();
    expect(marked('hide')).toHaveLength(7);
    const notice = document.querySelector('bigman-ad-notice');
    expect(notice.parentElement).toBe(document.documentElement);
    const root = shadows.get(notice);
    expect(root.textContent).toContain('Bigman hid 7 sponsored results.');
    click([...root.querySelectorAll('button')].find(b => b.textContent === 'Show them'));
    await wait();
    expect(marked('hide')).toHaveLength(0);
    expect(marked('highlight')).toHaveLength(7);
    expect(document.querySelector('bigman-ad-notice')).toBeNull();
});

test('a closed notice stays closed when more ads load', async () => {
    fixture('bing-vpn');
    load('https://www.bing.com/search?q=buy+vpn', { settings: { searchAds: 'hide' } });
    await wait();
    const root = shadows.get(document.querySelector('bigman-ad-notice'));
    click(root.querySelector('button.close'));
    const more = document.querySelector('li.b_ad').cloneNode(true);
    document.querySelector('#b_results').append(more);
    await wait();
    expect(marked('hide').length).toBeGreaterThan(7);
    expect(document.querySelector('bigman-ad-notice')).toBeNull();
});

test('off leaves results pages looking as they are', async () => {
    fixture('duckduckgo-vpn');
    load('https://duckduckgo.com/?q=buy+vpn', { settings: { searchAds: 'off' } });
    await wait();
    expect(document.querySelectorAll('[data-bigman-ad]')).toHaveLength(0);
});

test('other sites are never scanned for ads, even with ad-like markup', async () => {
    fixture('duckduckgo-vpn');
    load('https://news.example/?q=buy+vpn');
    await wait();
    expect(document.querySelectorAll('[data-bigman-ad]')).toHaveLength(0);
});

test('a page reached from an ad that named another site is replaced by a warning', async () => {
    const from = 'https://www.google.com/search?q=homebrew';
    const page = load('https://brewe.sh/', { landing: { key: 'ad:4', record: { shown: ['brew.sh'], via: [], from, created: Date.now() } } });
    await wait();
    expect(page.of('adLanded')).toEqual([{ action: 'adLanded', key: 'ad:4' }]);
    const [warn] = page.of('warn');
    expect(warn).toMatchObject({ here: true, rule: 'ad.landing-mismatch', url: 'https://brewe.sh/', back: from });
    expect(warn.reason).toContain('showed brew.sh, but it brought you to brewe.sh');
});

test('the advertiser\'s own site, a click tracker and an allowed host pass without a warning', async () => {
    const record = { shown: ['nordvpn.com'], via: ['trk.nord.example'], from: 'https://www.bing.com/search?q=vpn', created: Date.now() };
    let page = load('https://www.nordvpn.com/offer', { landing: { key: 'ad:1', record } });
    await wait();
    expect(page.of('warn')).toEqual([]);
    expect(page.of('adLanded')).toHaveLength(1);

    // A tracker's page is a step on the way: the record waits for the real landing page.
    page = load('https://clickserve.dartsearch.net/link/click?x=1', { landing: { key: 'ad:1', record } });
    await wait();
    expect(page.of('adLanding')).toEqual([]);

    page = load('https://nord-vpn.example/', { settings: { exceptions: { 'nord-vpn.example': Date.now() + 60000 } }, landing: { key: 'ad:1', record } });
    await wait();
    expect(page.of('warn')).toEqual([]);

    // The ad's declared tracking host is a step too, and keeps the record for the next page.
    page = load('https://trk.nord.example/c?id=1', { landing: { key: 'ad:1', record } });
    await wait();
    expect(page.of('warn')).toEqual([]);
    expect(page.of('adLanded')).toEqual([]);

    // An ad that named no site says nothing about where it leads.
    page = load('https://avg.com/', { landing: { key: 'ad:1', record: { ...record, shown: [] } } });
    await wait();
    expect(page.of('warn')).toEqual([]);
});

test('a tracking page that forwards to a lookalike still ends in a warning', async () => {
    const record = { shown: ['brew.sh'], via: ['trk.evil.example'], from: 'https://www.google.com/search?q=brew', created: Date.now() };
    let page = load('https://trk.evil.example/r', { landing: { key: 'ad:2', record } });
    await wait();
    expect(page.of('warn')).toEqual([]);
    page = load('https://brewe.sh/', { landing: { key: 'ad:2', record } });
    await wait();
    expect(page.of('warn')).toHaveLength(1);
});

test('back, forward and reload are not arrivals from an ad', async () => {
    const original = performance.getEntriesByType;
    performance.getEntriesByType = type => (type === 'navigation' ? [{ type: 'back_forward' }] : []);
    try {
        const page = load('https://brewe.sh/', { landing: { key: 'ad:4', record: { shown: ['brew.sh'], via: [], created: Date.now() } } });
        await wait();
        expect(page.of('adLanding')).toEqual([]);
        expect(page.of('warn')).toEqual([]);
    } finally { performance.getEntriesByType = original; }
});

test('loading a results page forgets the last ad clicked in the tab', async () => {
    const page = load('https://www.google.com/search?q=vpn');
    await wait();
    expect(page.of('adClear')).toHaveLength(1);
    expect(page.of('adLanding')).toEqual([]);
});
