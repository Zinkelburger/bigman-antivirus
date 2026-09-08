const { getDomain } = require('tldts');
const punycode = require('punycode/');
const defaults = require('./config/brand-domains.json');

// Stable, locally evaluated rules. Network reputation lookups and AI are not used.
class LinkDetector {
    constructor(config = defaults, settings = {}) {
        this.config = config;
        this.blocked = new Set(settings.blockedDomains || []);
        this.urls = new Set(settings.blockedUrls || []);
        this.exceptions = settings.exceptions || {};
    }
    extractDomain(value) {
        if (!value || typeof value !== 'string') return null;
        try { const u = new URL(value); if (u.hostname) return u.hostname.toLowerCase().replace(/\.$/, ''); } catch {}
        const m = value.match(/(?:^|[\s(])(?:https?:\/\/)?((?:[a-z0-9-]+\.)+[a-z]{2,63})(?=[/:?\s)]|$)/i);
        return m ? m[1].toLowerCase() : null;
    }
    normalizeDomain(value) { return (this.extractDomain(value) || '').replace(/^www\./, ''); }
    registrable(host) { return getDomain(host, { allowPrivateDomains: true }) || host; }
    isCanonical(host, domains) { return domains.some(d => host === d || host.endsWith('.' + d)); }
    distance(a, b) {
        let prev = Array.from({ length: b.length + 1 }, (_, i) => i);
        for (let i = 1; i <= a.length; i++) {
            const next = [i];
            for (let j = 1; j <= b.length; j++) next[j] = Math.min(next[j - 1] + 1, prev[j] + 1, prev[j - 1] + (a[i - 1] !== b[j - 1]));
            prev = next;
        }
        return prev[b.length];
    }
    analyzeLink(text = '', href = '', base = undefined) {
        if (typeof href !== 'string' || href.length > 8192) return this.result('unavailable', 'URL exceeds inspection limits', 'url.limit', {});
        let u;
        try { u = new URL(href, base); } catch { return this.result('unavailable', 'Invalid or unresolved URL', 'url.invalid', {}); }
        if (['javascript:', 'data:', 'file:'].includes(u.protocol)) return this.result('review', 'Link invokes code or opens a local resource', 'url.active', { actualUrl: u.href });
        if (u.protocol === 'mailto:') {
            const visible = text.match(/[\w.+-]+@[\w.-]+\.[a-z]{2,}/i)?.[0]?.toLowerCase();
            let actual; try { actual = decodeURIComponent(u.pathname).toLowerCase(); } catch { return this.result('unavailable', 'Invalid email encoding', 'email.invalid', {}); }
            if (visible && visible !== actual) return this.result('review', 'Email address mismatch', 'email.mismatch', { visible, actual });
            return this.result('none', 'No email mismatch detected', 'email.checked', {});
        }
        if (['tel:', 'sms:'].includes(u.protocol)) {
            const visible = text.match(/\+?[\d() .-]{7,}/)?.[0];
            const normalize = s => s.replace(/\D/g, '').replace(/^1(?=\d{10}$)/, '');
            if (visible && normalize(visible) !== normalize(u.pathname.split('?')[0])) return this.result('review', 'Phone number mismatch', 'phone.mismatch', { visible, actual: u.pathname });
            return this.result('none', 'No phone mismatch detected', 'phone.checked', {});
        }
        if (!['https:', 'http:'].includes(u.protocol)) return this.result('unavailable', 'This link scheme is not inspected', 'url.unsupported', {});
        const host = u.hostname.toLowerCase().replace(/\.$/, '');
        const domain = this.registrable(host);
        const details = { actualDomain: host, registrableDomain: domain, actualUrl: u.href };
        // Explicit blocklist matches take precedence over temporary heuristic exceptions.
        const reputationUrl = new URL(u.href); reputationUrl.hash = '';
        if (this.urls.has(reputationUrl.href) || [...this.blocked].some(d => host === d || host.endsWith('.' + d))) return this.result('block', 'Matches your imported blocklist', 'list.match', details);
        if (this.exceptions[host] > Date.now()) return this.result('none', 'Temporary hostname exception', 'user.exception', details);
        if (u.username || u.password) return this.result('review', 'URL contains a misleading user-info section', 'url.userinfo', details);
        const visible = this.extractDomain(text);
        if (visible && this.registrable(visible) !== domain) return this.result('review', 'Visible domain differs from the destination', 'domain.mismatch', { ...details, visibleDomain: visible });
        let decoded;
        try { decoded = punycode.toUnicode(host); } catch { return this.result('unavailable', 'Hostname could not be decoded', 'domain.invalid-idn', details); }
        const confusables = { 'а': 'a', 'е': 'e', 'о': 'o', 'р': 'p', 'с': 'c', 'х': 'x', 'у': 'y', 'і': 'i', 'ј': 'j', 'ӏ': 'l', 'α': 'a', 'ο': 'o', 'ρ': 'p' };
        const skeleton = [...decoded].map(c => confusables[c] || c).join('');
        for (const [brand, domains] of Object.entries(this.config.brandDomains)) {
            if (this.isCanonical(host, domains)) continue;
            if (skeleton !== host && this.isCanonical(skeleton, domains)) return this.result('review', 'Internationalized hostname resembles a protected domain', 'domain.confusable', { ...details, decoded, brand });
            const label = domain.split('.')[0];
            if (label.length >= 5 && brand.length >= 5 && this.distance(label, brand) === 1) return this.result('review', 'Hostname is one character away from a protected brand', 'domain.typo', { ...details, brand });
            const subdomains = host.slice(0, host.length - domain.length).split('.');
            if ([label, ...subdomains].includes(brand) || label.startsWith(brand + '-')) return this.result('review', 'Brand name appears outside its known domain', 'domain.brand', { ...details, brand });
        }
        if (host.includes('xn--')) return this.result('info', 'Internationalized hostname; shown decoded for inspection', 'domain.idn', { ...details, decoded });
        if (u.protocol === 'http:') return this.result('info', 'Connection is not encrypted', 'url.http', details);
        return this.result('none', 'No findings from local link checks', 'url.checked', details);
    }
    result(level, reason, rule, details) { return { level, reason, rule, details, isSuspicious: ['review', 'block'].includes(level) }; }
    analyzePage(text) {
        const sample = text.slice(0, 200000);
        const verification = /captcha|verify.{0,30}human|verification|fix.{0,20}(error|browser)/i.test(sample);
        const terminal = /powershell|terminal|win(dows)?\s*\+\s*r|command prompt/i.test(sample);
        const paste = /ctrl\s*\+\s*v|paste.{0,60}(command|code)|copy.{0,60}(command|code)/i.test(sample);
        return verification && terminal && paste ? this.result('review', 'Page combines verification/repair instructions with pasting commands into a terminal', 'page.clickfix', {}) : null;
    }
}
module.exports = LinkDetector;
if (typeof globalThis !== 'undefined') globalThis.BigmanLinkDetector = LinkDetector;
