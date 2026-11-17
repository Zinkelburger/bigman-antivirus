// Sponsored results on Google, Bing and DuckDuckGo: which page elements are ads,
// and which sites each ad says it leads to. Pure DOM reading; content.js marks them.
(function (root) {
    const GOOGLE = /^(?:www\.)?google\.(?:com|[a-z]{2}|co\.[a-z]{2}|com\.[a-z]{2})$/;

    /** 'google', 'bing' or 'duckduckgo' for a results page address, otherwise null. */
    function engineFor(href) {
        let u;
        try { u = new URL(href); } catch { return null; }
        if (u.protocol !== 'https:') return null;
        const host = u.hostname.toLowerCase();
        if (GOOGLE.test(host) && u.pathname === '/search') return 'google';
        if (['www.bing.com', 'bing.com', 'cn.bing.com'].includes(host) && u.pathname === '/search') return 'bing';
        if (host === 'duckduckgo.com' && u.pathname === '/' && u.searchParams.has('q')) return 'duckduckgo';
        if (['html.duckduckgo.com', 'lite.duckduckgo.com'].includes(host) && /^\/(?:html|lite)\/?$/.test(u.pathname)) return 'duckduckgo';
        return null;
    }

    // Engines and click trackers an ad passes through before it reaches the
    // advertiser. A page on one is not the landing site, so the landing check waits
    // past it, and an ad that names only these says nothing about where it leads.
    const REDIRECTORS = [
        'googleadservices.com', 'doubleclick.net', 'googlesyndication.com', 'bing.com', 'duckduckgo.com',
        'dartsearch.net', 'xg4ken.com', 'everesttech.net', 'adsrvr.org', 'adform.net', 'clickcease.com'
    ];
    function isRedirector(host) {
        host = String(host).toLowerCase();
        return GOOGLE.test(host) || REDIRECTORS.some(d => host === d || host.endsWith('.' + d));
    }

    // How each engine marks an ad. `units` are single ads, `links` are links only ads
    // have, `containers` are what an ad link belongs to, and `stop` bounds the search
    // for a "Sponsored" label so a whole results column is never taken for one ad.
    const ENGINES = {
        google: {
            units: '[data-text-ad]',
            links: 'a[href*="/aclk?"], a[href*="googleadservices.com/pagead/aclk"], a[data-rw*="/aclk?"], a[data-pcu]',
            containers: '[data-text-ad]',
            stop: '#rso, #search, #center_col, #rcnt, #tads, #tadsb, #bottomads, main, body'
        },
        bing: {
            units: 'li.b_ad > ul > li, li.b_PolePAContainer',
            links: 'a[href*="bing.com/aclk?"], a[href^="/aclk?"]',
            containers: 'li.b_ad > ul > li, li.b_PolePAContainer, li.b_ad, .adsMvCarousel',
            stop: '#b_results, #b_topw, #b_context, main, body'
        },
        duckduckgo: {
            units: 'li[data-layout="ad"], .result--ad',
            links: 'a[href*="/y.js?"][href*="ad_domain="]',
            containers: 'li[data-layout], .result, article',
            stop: '.react-results--main, main, body'
        }
    };
    const LABEL = /^(?:ad|ads|sponsored|sponsored results?|sponsored products?|sponsored links?)$/i;
    const MAX_CLIMB = 10;

    function labels(doc) {
        const found = [];
        const walker = doc.createTreeWalker(doc.body || doc.documentElement, 4 /* text */);
        while (walker.nextNode()) {
            const text = walker.currentNode.nodeValue.trim();
            if (text.length <= 20 && LABEL.test(text) && walker.currentNode.parentElement) found.push(walker.currentNode.parentElement);
        }
        return found;
    }

    function labelledAncestor(link, stop, marks) {
        let el = link.parentElement;
        for (let i = 0; el && i < MAX_CLIMB && !el.matches(stop); i++, el = el.parentElement) {
            if (marks.some(m => el.contains(m))) return el;
        }
        return null;
    }

    /** The ad elements on a results page, outermost only, in document order. */
    function findAds(doc, engine) {
        const rules = ENGINES[engine];
        if (!rules || !doc.body) return [];
        const units = new Set(doc.querySelectorAll(rules.units));
        let marks = null;
        for (const link of doc.querySelectorAll(rules.links)) {
            if ([...units].some(u => u.contains(link))) continue;
            const unit = link.closest(rules.containers) || labelledAncestor(link, rules.stop, marks ??= labels(doc));
            units.add(unit || link);
        }
        // An ad nested inside another ad is marked by its outer one.
        return [...units].filter(u => ![...units].some(o => o !== u && o.contains(u)));
    }

    function hostOf(text) {
        const m = String(text || '').match(/(?:https?:\/\/)?((?:[a-z0-9-]+\.)+[a-z]{2,63})(?=[\s/:?›»]|$)/i);
        return m ? m[1].toLowerCase().replace(/^www\./, '') : null;
    }

    // Bing carries the advertiser's address in `u`: base64 of the address, sometimes
    // percent-encoded first, sometimes after an "a1" prefix.
    function bingTarget(value) {
        if (!value) return null;
        // URLSearchParams turns base64's '+' into a space.
        value = value.replace(/ /g, '+');
        for (const raw of [value, value.replace(/^a1/, '')]) {
            try {
                const decoded = atob(raw.replace(/-/g, '+').replace(/_/g, '/'));
                const address = /^https?%3a/i.test(decoded) ? decodeURIComponent(decoded) : decoded;
                if (/^https?:\/\//i.test(address)) return new URL(address).hostname.toLowerCase();
            } catch { /* not this encoding */ }
        }
        return null;
    }

    const clean = host => (host ? String(host).toLowerCase().replace(/^www\./, '').replace(/\.$/, '') : null);
    function hostOfUrl(value) { try { return clean(new URL(String(value).trim()).hostname); } catch { return null; } }

    /**
     * Where an ad link says it leads, as hostnames. `shown` is the site the ad
     * displayed: the click is expected to end there. `via` are hosts the engine
     * declares on the way (the advertiser's tracking template); a page on one is a
     * step, never the end. Both are empty when unknown.
     */
    function destinationsFor(link, unit, engine) {
        const shown = new Set(), via = new Set();
        let href;
        try { href = new URL(link.href); } catch { href = null; }
        if (engine === 'duckduckgo' && href) shown.add(clean(href.searchParams.get('ad_domain')));
        if (engine === 'google') {
            const own = link.closest('[data-pcu]') || link;
            const single = unit && unit.matches('[data-text-ad]') ? unit : null;
            const pcu = (own.getAttribute('data-pcu') || single?.querySelector('[data-pcu]')?.getAttribute('data-pcu') || '').split(',').map(hostOfUrl).filter(Boolean);
            const displayed = clean(single?.querySelector('[data-dtld]')?.getAttribute('data-dtld'));
            // The displayed domain when Google gives one; otherwise the final address, listed first.
            shown.add(displayed || pcu[0]);
            pcu.forEach(h => via.add(h));
        }
        if (engine === 'bing') {
            const target = href && bingTarget(href.searchParams.get('u'));
            // A single ad prints the advertiser's address in a <cite>; a shopping card does not.
            const cites = unit && unit.matches('li.b_ad > ul > li') ? [...unit.querySelectorAll('cite')].map(c => hostOf(c.textContent)).filter(Boolean) : [];
            if (cites.length) { cites.forEach(h => shown.add(h)); via.add(clean(target)); } else shown.add(clean(target));
        }
        const list = set => [...set].filter(h => h && !isRedirector(h));
        const ends = list(shown);
        return { shown: ends, via: list(via).filter(h => !ends.includes(h)) };
    }

    const api = { engineFor, isRedirector, findAds, destinationsFor, ENGINES };
    if (typeof module !== 'undefined' && module.exports) module.exports = api;
    else root.BigmanSearchAds = api;
})(typeof globalThis !== 'undefined' ? globalThis : this);
