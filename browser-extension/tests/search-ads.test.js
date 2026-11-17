/** @jest-environment jsdom */
// Real results pages saved in November 2026 and reduced by scripts/trim-serp.js.
const fs = require('fs');
const path = require('path');
const ads = require('../search-ads');

function page(name, url) {
    const doc = new DOMParser().parseFromString(fs.readFileSync(path.join(__dirname, 'fixtures/serp', `${name}.html`), 'utf8'), 'text/html');
    const base = doc.createElement('base');
    base.href = url;
    doc.head.append(base);
    return doc;
}
const text = el => el.textContent.replace(/\s+/g, ' ');
// Every site a unit's links say they end at ('shown'), or pass through ('via').
const destinations = (unit, engine, kind = 'shown') => [...new Set([...unit.querySelectorAll('a[href]')].flatMap(a => ads.destinationsFor(a, unit, engine)[kind]))].sort();

test('recognises results pages and nothing else', () => {
    expect(ads.engineFor('https://www.google.com/search?q=vpn')).toBe('google');
    expect(ads.engineFor('https://www.google.co.uk/search?q=vpn')).toBe('google');
    expect(ads.engineFor('https://www.google.com.au/search?q=vpn')).toBe('google');
    expect(ads.engineFor('https://www.bing.com/search?q=vpn')).toBe('bing');
    expect(ads.engineFor('https://duckduckgo.com/?q=vpn')).toBe('duckduckgo');
    expect(ads.engineFor('https://html.duckduckgo.com/html/?q=vpn')).toBe('duckduckgo');
    for (const other of ['https://www.google.com/maps', 'https://google.evil.example/search?q=x', 'https://mail.google.com/search', 'http://www.google.com/search?q=x', 'https://duckduckgo.com/', 'https://www.bing.com/images', 'not a url'])
        expect(ads.engineFor(other)).toBeNull();
});

test('Google: top, bottom and AI Overview ads, each with the sites it names', () => {
    const doc = page('google-vpn', 'https://www.google.com/search?q=vpn&hl=en&gl=us');
    const found = ads.findAds(doc, 'google');
    expect(found).toHaveLength(7);
    // Six text ads, and the sponsored suggestions inside the AI Overview.
    expect(found.filter(u => u.matches('[data-text-ad]'))).toHaveLength(6);
    const overview = found.find(u => !u.matches('[data-text-ad]'));
    expect(text(overview)).toMatch(/^Sponsored ?You might consider these options/);
    // The displayed domain is where the click must end; the final address and its
    // tracking template (Kenshoo, a known tracker, is dropped) are only steps.
    expect(destinations(found[0], 'google')).toEqual(['microsoft.com']);
    expect(destinations(found[0], 'google', 'via')).toEqual(['explore.microsoft.com']);
    expect(destinations(found[1], 'google')).toEqual(['nordvpn.com']);
    // No organic result is taken for an ad (Google links them through /goto as well).
    expect(text(doc.body)).toContain('en.wikipedia.org');
    for (const unit of found) expect(text(unit)).not.toContain('wikipedia.org');
});

test('Google shopping ads carry their address in data attributes, not the link', () => {
    const found = ads.findAds(page('google-shoes', 'https://www.google.com/search?q=buy+running+shoes'), 'google');
    expect(found).toHaveLength(2);
    expect(destinations(found[0], 'google')).toEqual(['luckyfeetshoes.com']);
    expect(destinations(found[1], 'google')).toEqual(['vivobarefoot.com']);
});

test('Bing: top, sidebar, bottom and shopping ads, with each ad link\'s own advertiser', () => {
    const found = ads.findAds(page('bing-vpn', 'https://www.bing.com/search?q=buy+vpn'), 'bing');
    expect(found).toHaveLength(7);
    const shopping = found.find(u => u.matches('.b_PolePAContainer'));
    // Several advertisers share the carousel; each link names its own, without the click tracker.
    expect(destinations(shopping, 'bing')).toEqual(expect.arrayContaining(['us.norton.com', 'nordvpn.com']));
    expect(destinations(shopping, 'bing')).not.toContain('ad.doubleclick.net');
    const forbes = found.find(u => text(u).startsWith('Forbes'));
    expect(destinations(forbes, 'bing')).toEqual(['forbes.com']);
    // With a printed address, the ad's encoded target is a step, not the end.
    expect(destinations(forbes, 'bing', 'via')).not.toContain('forbes.com');
    for (const unit of found) expect(unit.closest('li.b_algo')).toBeNull();
});

test('DuckDuckGo: text and shopping ads, named by the ad_domain the engine declares', () => {
    const found = ads.findAds(page('duckduckgo-vpn', 'https://duckduckgo.com/?q=buy+vpn&kl=us-en'), 'duckduckgo');
    const items = found.filter(u => u.tagName === 'LI');
    expect(items).toHaveLength(6);
    expect(destinations(items[0], 'duckduckgo')).toEqual(['expressvpn.com']);
    expect(destinations(items[1], 'duckduckgo')).toEqual(['nordvpn.com']);
    // Sitelink menus rendered outside the result list are marked link by link.
    for (const link of found.filter(u => u.tagName === 'A')) expect(link.href).toContain('ad_domain=');
    for (const unit of found) expect(unit.matches('li[data-layout="organic"]')).toBe(false);
});

test('Bing ad addresses decode with and without the a1 prefix', () => {
    const doc = document.implementation.createHTMLDocument('');
    const encoded = btoa(encodeURIComponent('https://brewe.sh/install')).replace(/\+/g, '-').replace(/\//g, '_');
    const link = Object.assign(doc.createElement('a'), { href: `https://www.bing.com/aclk?ld=e8&u=a1${encoded}` });
    expect(ads.destinationsFor(link, null, 'bing')).toEqual({ shown: ['brewe.sh'], via: [] });
    link.href = `https://www.bing.com/aclk?u=${btoa('https://www.example.org/')}`;
    expect(ads.destinationsFor(link, null, 'bing').shown).toEqual(['example.org']);
    link.href = 'https://www.bing.com/aclk?u=not-base64!';
    expect(ads.destinationsFor(link, null, 'bing')).toEqual({ shown: [], via: [] });
});

test('a page with no ads and an unknown engine find nothing', () => {
    const doc = document.implementation.createHTMLDocument('');
    doc.body.innerHTML = '<div id="rso"><div data-hveid="1"><a href="https://www.google.com/goto?url=x">Example https://example.org</a></div></div>';
    expect(ads.findAds(doc, 'google')).toEqual([]);
    expect(ads.findAds(doc, 'yahoo')).toEqual([]);
    expect(ads.isRedirector('www.googleadservices.com')).toBe(true);
    expect(ads.isRedirector('clickserve.dartsearch.net')).toBe(true);
    expect(ads.isRedirector('brewe.sh')).toBe(false);
});

test('Google: a tracking template on the advertiser\'s own host is a step, the displayed domain the end', () => {
    const doc = document.implementation.createHTMLDocument('');
    doc.body.innerHTML = '<div data-text-ad><span data-dtld="brew.sh"></span><a data-pcu="https://brew.sh/,https://trk.evil.example/c" href="https://www.google.com/aclk?sa=l">Homebrew</a></div>';
    const unit = doc.querySelector('[data-text-ad]');
    expect(ads.destinationsFor(doc.querySelector('a'), unit, 'google')).toEqual({ shown: ['brew.sh'], via: ['trk.evil.example'] });
});
