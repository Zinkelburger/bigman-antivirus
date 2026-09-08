(async () => {
    const el = id => document.getElementById(id);
    el('back').onclick = () => window.close();
    const id = new URL(location.href).searchParams.get('id');
    const record = (await chrome.storage.session.get(id || ''))[id];
    if (!record || Date.now() - record.created > 600000) { el('status').textContent = 'This warning expired. Recheck the original link.'; return; }
    const u = new URL(record.url);
    if (!['http:', 'https:'].includes(u.protocol)) return;
    el('reason').textContent = record.reason; el('url').textContent = u.href;
    el('continue').disabled = false;
    el('continue').onclick = async () => { await chrome.storage.session.remove(id); location.replace(u.href); };
    if (record.rule !== 'list.match') {
        el('exception').disabled = false;
        el('exception').onclick = async () => {
            const { exceptions = {} } = await chrome.storage.local.get('exceptions');
            exceptions[u.hostname] = Date.now() + 3600000;
            await chrome.storage.local.set({ exceptions }); await chrome.storage.session.remove(id); location.replace(u.href);
        };
    }
})();
