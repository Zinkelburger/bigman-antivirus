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

// On a repository page: offer the desktop's read-before-clone check.
function showRepoCheck(r) {
    const s = BigmanRepoPage.summarize(r);
    el('repo-status').textContent = s.status;
    el('repo-findings').replaceChildren(...s.findings.map(f => {
        const item = document.createElement('li');
        const level = document.createElement('strong');
        level.textContent = `${f.severity}: `;
        item.append(level, f.summary);
        return item;
    }));
    if (s.more) {
        const item = document.createElement('li');
        item.textContent = `${s.more} more in Bigman's Activity tab.`;
        el('repo-findings').append(item);
    }
    el('repo-read').textContent = s.read || '';
    el('repo-next').textContent = s.next || '';
}
async function offerRepoCheck() {
    const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
    const repo = tab?.url && BigmanRepoPage.repoFromPage(tab.url);
    if (!repo) return;
    el('repo').hidden = false;
    el('repo-name').textContent = repo.replace(/^https:\/\//, '');
    el('repo-check').onclick = async () => {
        el('repo-check').disabled = true;
        el('repo-status').textContent = 'Reading the repository… This can take a minute for a large one.';
        el('repo-findings').replaceChildren();
        el('repo-read').textContent = el('repo-next').textContent = '';
        try { showRepoCheck(await chrome.runtime.sendMessage({ action: 'native', request: { op: 'repo-check', url: repo } })); }
        catch (error) { showRepoCheck({ status: 'INCOMPLETE', reason: error.message }); }
        finally { el('repo-check').disabled = false; }
    };
}
offerRepoCheck().catch(() => {});
el('inspect').onclick = () => native({ op: 'inspect-url', url: el('installer').value });
