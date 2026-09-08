const el = id => document.getElementById(id);
chrome.storage.local.get(['scanDownloads', 'blockedDomains', 'blockedUrls', 'feedUpdated', 'nativeHealth']).then(s => {
    el('downloads').checked = !!s.scanDownloads;
    el('list').value = [...(s.blockedDomains || []), ...(s.blockedUrls || [])].join('\n');
    el('status').textContent = s.feedUpdated ? `Last list import: ${new Date(s.feedUpdated).toLocaleString()}. Lists do not update automatically.` : 'No reputation list has been imported.';
    el('health').textContent = s.nativeHealth ? `Latest desktop result: ${s.nativeHealth.status}\n${s.nativeHealth.reason}\n${new Date(s.nativeHealth.checkedAt).toLocaleString()}` : 'Desktop connection has not been checked.';
});
el('file').onchange = async () => {
    const file = el('file').files[0]; if (!file) return;
    if (file.size > 2 * 1024 * 1024) { el('status').textContent = 'File exceeds 2 MiB limit.'; return; }
    el('list').value = await file.text();
};
el('save').onclick = async () => {
    try {
        const lines = el('list').value.split(/\r?\n/).map(x => x.trim()).filter(x => x && !x.startsWith('#'));
        if (lines.length > 20000) throw new Error('At most 20,000 entries are supported');
        const blockedDomains = [], blockedUrls = [];
        for (const line of lines) {
            const isUrl = /^https?:\/\//i.test(line);
            const u = new URL(isUrl ? line : `https://${line}`);
            if (!['http:', 'https:'].includes(u.protocol) || u.username || u.password || (!isUrl && (u.pathname !== '/' || u.port || u.search || u.hash))) throw new Error(`Invalid entry: ${line}`);
            u.hash = '';
            (isUrl ? blockedUrls : blockedDomains).push(isUrl ? u.href : u.hostname.toLowerCase().replace(/\.$/, ''));
        }
        await chrome.storage.local.set({ blockedDomains: [...new Set(blockedDomains)], blockedUrls: [...new Set(blockedUrls)], feedUpdated: Date.now(), scanDownloads: el('downloads').checked });
        el('status').textContent = `Saved ${blockedDomains.length} domain entries and ${blockedUrls.length} exact URLs. Existing pages use the new settings on their next scan.`;
    } catch (error) { el('status').textContent = `Not saved: ${error.message}`; }
};
el('clear').onclick = async () => { await chrome.storage.local.set({ exceptions: {} }); el('status').textContent = 'Temporary exceptions cleared.'; };
