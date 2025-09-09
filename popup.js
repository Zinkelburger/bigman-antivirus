document.getElementById('scanBtn').addEventListener('click', async () => {
    const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
    
    const results = await chrome.scripting.executeScript({
        target: { tabId: tab.id },
        function: () => {
            const links = document.querySelectorAll('a[href]');
            const detector = new LinkDetector();
            const suspicious = [];
            
            links.forEach(link => {
                const result = detector.analyzeLink(link.textContent, link.href);
                if (result.isSuspicious) {
                    suspicious.push({
                        text: link.textContent,
                        url: link.href,
                        reason: result.reason
                    });
                }
            });
            
            return { total: links.length, suspicious: suspicious.length, details: suspicious };
        }
    });
    
    const result = results[0].result;
    document.getElementById('results').innerHTML = `
        <h3>Scan Results</h3>
        <p>Total links: ${result.total}</p>
        <p>Suspicious links: ${result.suspicious}</p>
        ${result.suspicious > 0 ? '<p style="color: red;">⚠️ Suspicious links detected!</p>' : '<p style="color: green;">✅ No suspicious links found</p>'}
    `;
});
