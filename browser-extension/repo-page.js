// Repository pages on code hosts: which repository a page belongs to, and what
// the desktop's read-before-clone check found, in a few plain lines for the popup.
(function (root) {
        // First path segments that are each host's own pages, not an owner.
    const RESERVED = {
        'github.com': new Set([
            'about', 'account', 'apps', 'collections', 'contact', 'copilot', 'customer-stories', 'dashboard',
            'enterprise', 'enterprises', 'events', 'explore', 'features', 'git-guides', 'home', 'issues', 'join',
            'login', 'logout', 'marketplace', 'new', 'notifications', 'organizations', 'orgs', 'partners', 'pricing',
            'pulls', 'readme', 'resources', 'search', 'security', 'settings', 'signup', 'site', 'solutions', 'sponsors',
            'stars', 'team', 'topics', 'trending', 'users', 'watching'
        ]),
        'gitlab.com': new Set(['dashboard', 'explore', 'groups', 'help', 'projects', 'search', 'users']),
        'codeberg.org': new Set(['admin', 'explore', 'issues', 'notifications', 'org', 'pulls', 'repo', 'user']),
        'bitbucket.org': new Set(['account', 'dashboard', 'product', 'site'])
    };
    const PART = /^[A-Za-z0-9_][A-Za-z0-9_.-]*$/;
    const ORDER = { high: 0, review: 1, info: 2 };
    const LABEL = { high: 'High', review: 'Review', info: 'Info' };

    /** The repository a page belongs to, as https://host/OWNER/REPO, or null. */
    function repoFromPage(href) {
        let url;
        try { url = new URL(href); } catch { return null; }
        if (url.protocol !== 'https:') return null;
        const host = url.hostname.toLowerCase().replace(/^www\./, '');
        if (!RESERVED[host]) return null;
        const parts = url.pathname.split('/').filter(Boolean);
        // GitLab nests groups and marks the end of a project path with "/-/".
        const marker = parts.indexOf('-');
        const path = host === 'gitlab.com' ? (marker === -1 ? parts : parts.slice(0, marker)) : parts.slice(0, 2);
        if (path.length < 2 || RESERVED[host].has(path[0].toLowerCase())) return null;
        path[path.length - 1] = path[path.length - 1].replace(/\.git$/, '');
        if (!path.every(p => PART.test(p))) return null;
        return `https://${host}/${path.join('/')}`;
    }

    /** What the popup shows for a native repo-check response. */
    function summarize(response) {
        const report = response && response.report;
        if (!report) return { status: `${(response && response.status) || 'INCOMPLETE'}: ${(response && response.reason) || 'No answer from the desktop app.'}`, findings: [], more: 0, read: null, next: null };
        const findings = report.findings
            .filter(f => f.severity !== 'info')
            .sort((a, b) => ORDER[a.severity] - ORDER[b.severity])
            .map(f => ({ severity: LABEL[f.severity] || f.severity, summary: f.summary }));
        const notes = [...(report.coverage.notes || []), ...(report.coverage.checked || [])];
        const read = notes.find(n => n.startsWith('Read before cloning')) || null;
        const high = findings.some(f => f.severity === 'High');
        return {
            status: response.reason,
            findings: findings.slice(0, 8),
            more: Math.max(0, findings.length - 8),
            read,
            next: high
                ? 'Do not open or run it on this computer. If you need to see it run, clone it and use bigman try, which runs it in a disposable container.'
                : null
        };
    }

    const api = { repoFromPage, summarize };
    if (typeof module !== 'undefined' && module.exports) module.exports = api;
    else root.BigmanRepoPage = api;
})(typeof globalThis !== 'undefined' ? globalThis : this);
