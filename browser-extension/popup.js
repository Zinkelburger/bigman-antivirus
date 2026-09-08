const el = id => document.getElementById(id);
el('options').onclick = () => chrome.runtime.openOptionsPage();
el('scan').onclick = async () => {
    try {
        const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
        const response = await chrome.tabs.sendMessage(tab.id, { action: 'rescan' }, { frameId: 0 });
        if (!response?.success) throw new Error('Page check unavailable');
        const r = response.results;
        el('status').textContent = `${r.total} links in the main document checked; ${r.suspicious} findings. ${r.incomplete ? 'Coverage incomplete.' : 'No finding is not a safety guarantee.'}`;
        el('details').textContent = r.error || r.details.map(d => `${d.reason}\n${d.url}\n${JSON.stringify(d.details)}`).join('\n\n') || 'No findings from local checks.';
    } catch (error) { el('status').textContent = `Unavailable: ${error.message}. Reload ordinary web pages after installing the extension; restricted browser pages cannot be scanned.`; }
};
async function native(request) {
    el('native').textContent = 'Waiting for desktop…';
    try { const r = await chrome.runtime.sendMessage({ action: 'native', request }); el('native').textContent = `${r.status}: ${r.reason}\n${r.report ? JSON.stringify(r.report, null, 2) : ''}`; }
    catch (error) { el('native').textContent = `Incomplete: ${error.message}`; }
}
el('health').onclick = () => native({ op: 'health' });
el('inspect').onclick = () => native({ op: 'inspect-url', url: el('installer').value });
