const LinkDetector = require('../link-detector');
const defaults = require('../config/brand-domains.json');

describe('deterministic destination rules', () => {
    const d = new LinkDetector();
    test.each([
        ['Visit https://paypal.com', 'https://evil.example', 'domain.mismatch'],
        ['https://paypal.com', 'https://evil.example', 'domain.mismatch'],
        ['', 'https://microsft.com', 'domain.typo'],
        ['Open', 'https://paypal.com.attacker.example', 'domain.brand'],
        ['Open', 'https://paypal.com@evil.example/', 'url.userinfo'],
        ['Open', 'javascript:alert(1)', 'url.active'],
        ['support@example.org', 'mailto:wrong@example.org', 'email.mismatch'],
        ['555-123-4567', 'tel:555-999-9999', 'phone.mismatch'],
    ])('flags %s → %s', (text, url, rule) => {
        const r = d.analyzeLink(text, url);
        expect(r.isSuspicious).toBe(true); expect(r.rule).toBe(rule);
    });
    test.each([
        ['Python docs', 'https://docs.python.org'],
        ['Open', 'https://example.org/café'],
        ['Open', 'https://bücher.de'],
        ['Open', 'http://example.org'],
        ['Open', 'https://example.org/download.zip'],
        ['github.com', 'https://docs.github.com'],
        ['Open', 'https://shop.example'],
        ['Open', 'https://alice.github.io'],
        ['Open', 'https://alice.gitlab.io'],
    ])('does not call benign/unproven properties phishing: %s → %s', (text, url) => {
        expect(d.analyzeLink(text, url).isSuspicious).toBe(false);
    });
    test('IDN information handles ASCII browser-normalized hostnames', () => {
        const r = d.analyzeLink('', new URL('https://bücher.de').href);
        expect(r.rule).toBe('domain.idn'); expect(r.details.decoded).toBe('bücher.de');
    });
    test('confusable protected domain is reviewed', () => {
        const r = d.analyzeLink('', new URL('https://раураl.com').href);
        expect(r.rule).toBe('domain.confusable');
    });
    test('uses private suffixes, not a hand-maintained two-label shortcut', () => {
        expect(d.registrable('alice.github.io')).toBe('alice.github.io');
        expect(d.analyzeLink('alice.github.io', 'https://bob.github.io').rule).toBe('domain.mismatch');
        expect(d.registrable('www.example.co.uk')).toBe('example.co.uk');
    });
    test('blocklist matches are on label boundaries and override exceptions', () => {
        const d = new LinkDetector(defaults, { blockedDomains: ['bad.example'], exceptions: { 'bad.example': Date.now() + 60000 } });
        expect(d.analyzeLink('', 'https://bad.example').level).toBe('block');
        expect(d.analyzeLink('', 'https://sub.bad.example').level).toBe('block');
        expect(d.analyzeLink('', 'https://notbad.example').isSuspicious).toBe(false);
    });
    test('exact URLs do not block neighboring paths', () => {
        const d = new LinkDetector(defaults, { blockedUrls: ['https://example.org/bad'] });
        expect(d.analyzeLink('', 'https://example.org/bad').level).toBe('block');
        expect(d.analyzeLink('', 'https://example.org/bad#bypass').level).toBe('block');
        expect(d.analyzeLink('', 'https://example.org/good').level).toBe('none');
    });
    test('exceptions expire', () => {
        const d = new LinkDetector(defaults, { exceptions: { 'microsft.com': Date.now() - 100 } });
        expect(d.analyzeLink('', 'https://microsft.com').isSuspicious).toBe(true);
    });
    test('relative links use document base URI', () => {
        expect(d.analyzeLink('Open', '/docs', 'https://example.org').details.actualDomain).toBe('example.org');
    });
    test('invalid URL is unavailable, never cleared as safe', () => {
        expect(d.analyzeLink('', 'https://').level).toBe('unavailable');
    });
    test('ClickFix rule requires verification, terminal and paste context together', () => {
        expect(d.analyzePage('Verify you are human. Press Win+R, then Ctrl+V to paste code.').rule).toBe('page.clickfix');
        expect(d.analyzePage('Open your terminal to install this developer tool.')).toBeNull();
        expect(d.analyzePage('Please complete the CAPTCHA.')).toBeNull();
    });
    test('analysis makes no requests', () => {
        global.fetch = jest.fn(() => { throw new Error('Network forbidden'); });
        d.analyzeLink('Open', 'https://example.org');
        expect(global.fetch).not.toHaveBeenCalled(); delete global.fetch;
    });
});
