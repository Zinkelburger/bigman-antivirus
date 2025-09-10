document.getElementById('scanBtn').addEventListener('click', async () => {
    const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
    const consoleOutput = document.getElementById('consoleOutput');
    
    consoleOutput.value = "Requesting scan results from content script...\n";
    
    try {
        // Send message to content script to get current scan results
        const response = await chrome.tabs.sendMessage(tab.id, { action: 'getScanResults' });
        
        if (response && response.success && response.results) {
            const result = response.results;
            
            // Ensure result has required properties
            if (typeof result.total === 'undefined' || typeof result.suspicious === 'undefined') {
                throw new Error('Invalid scan results format');
            }
            
            // Update results section
            document.getElementById('results').innerHTML = `
                <h3>Scan Results</h3>
                <p>Total links: ${result.total}</p>
                <p>Suspicious links: ${result.suspicious}</p>
                ${result.suspicious > 0 ? '<p style="color: red;">⚠️ Suspicious links detected!</p>' : '<p style="color: green;">✅ No suspicious links found</p>'}
                <p><small>Last scanned: ${result.lastScanTime ? new Date(result.lastScanTime).toLocaleTimeString() : 'Unknown'}</small></p>
            `;
            
            // Update console output
            let consoleText = `Results retrieved at ${new Date().toLocaleTimeString()}\n`;
            consoleText += `Total links scanned: ${result.total}\n`;
            consoleText += `Suspicious links found: ${result.suspicious}\n`;
            consoleText += `Last scan time: ${result.lastScanTime ? new Date(result.lastScanTime).toLocaleTimeString() : 'Unknown'}\n\n`;
            
            if (result.suspicious > 0) {
                consoleText += "=== SUSPICIOUS LINKS DETECTED ===\n\n";
                result.details.forEach((link, index) => {
                    consoleText += `[${index + 1}] SUSPICIOUS LINK:\n`;
                    consoleText += `Visible Text: "${link.text.trim()}"\n`;
                    consoleText += `Actual URL: ${link.url}\n`;
                    consoleText += `Reason: ${link.reason}\n`;
                    
                    // Add additional details if available
                    if (link.details) {
                        if (link.details.visibleDomain && link.details.actualDomain) {
                            consoleText += `Visible Domain: ${link.details.visibleDomain}\n`;
                            consoleText += `Actual Domain: ${link.details.actualDomain}\n`;
                        } else if (link.details.visibleIdentifier && link.details.actualIdentifier) {
                            consoleText += `Visible Identifier: ${link.details.visibleIdentifier}\n`;
                            consoleText += `Actual Identifier: ${link.details.actualIdentifier}\n`;
                        }
                        if (link.details.originalText) {
                            consoleText += `Original Text: "${link.details.originalText}"\n`;
                        }
                    }
                    
                    consoleText += `\n---\n\n`;
                });
            } else {
                consoleText += "✅ No suspicious links detected. All links appear safe.\n";
            }
            
            consoleOutput.value = consoleText;
        } else {
            throw new Error('Failed to get scan results from content script');
        }
    } catch (error) {
        console.error('Error communicating with content script:', error);
        
        // Fallback: try to inject and scan if content script is not available
        consoleOutput.value = "Content script not available, performing fallback scan...\n";
        
        try {
            // Inject the dependency file into the tab first
            await chrome.scripting.executeScript({
                target: { tabId: tab.id },
                files: ['link-detector.js']
            });

            // Execute the scan function
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
                                reason: result.reason,
                                details: result.details
                            });
                        }
                    });
                    
                    return { 
                        total: links.length, 
                        suspicious: suspicious.length, 
                        details: suspicious,
                        lastScanTime: new Date()
                    };
                }
            });

            const result = results[0].result;
            
            // Ensure result has required properties
            if (!result || typeof result.total === 'undefined' || typeof result.suspicious === 'undefined') {
                throw new Error('Invalid fallback scan results format');
            }
            
            // Update results section
            document.getElementById('results').innerHTML = `
                <h3>Scan Results (Fallback)</h3>
                <p>Total links: ${result.total}</p>
                <p>Suspicious links: ${result.suspicious}</p>
                ${result.suspicious > 0 ? '<p style="color: red;">⚠️ Suspicious links detected!</p>' : '<p style="color: green;">✅ No suspicious links found</p>'}
            `;
            
            // Update console output
            let consoleText = `Fallback scan completed at ${new Date().toLocaleTimeString()}\n`;
            consoleText += `Total links scanned: ${result.total}\n`;
            consoleText += `Suspicious links found: ${result.suspicious}\n\n`;
            
            if (result.suspicious > 0) {
                consoleText += "=== SUSPICIOUS LINKS DETECTED ===\n\n";
                result.details.forEach((link, index) => {
                    consoleText += `[${index + 1}] SUSPICIOUS LINK:\n`;
                    consoleText += `Visible Text: "${link.text.trim()}"\n`;
                    consoleText += `Actual URL: ${link.url}\n`;
                    consoleText += `Reason: ${link.reason}\n`;
                    
                    // Add additional details if available
                    if (link.details) {
                        if (link.details.visibleDomain && link.details.actualDomain) {
                            consoleText += `Visible Domain: ${link.details.visibleDomain}\n`;
                            consoleText += `Actual Domain: ${link.details.actualDomain}\n`;
                        } else if (link.details.visibleIdentifier && link.details.actualIdentifier) {
                            consoleText += `Visible Identifier: ${link.details.visibleIdentifier}\n`;
                            consoleText += `Actual Identifier: ${link.details.actualIdentifier}\n`;
                        }
                        if (link.details.originalText) {
                            consoleText += `Original Text: "${link.details.originalText}"\n`;
                        }
                    }
                    
                    consoleText += `\n---\n\n`;
                });
            } else {
                consoleText += "✅ No suspicious links detected. All links appear safe.\n";
            }
            
            consoleOutput.value = consoleText;
        } catch (fallbackError) {
            console.error('Fallback scan also failed:', fallbackError);
            consoleOutput.value = `Error: Unable to scan page. ${fallbackError.message}`;
        }
    }
    
    // Auto-scroll to top
    consoleOutput.scrollTop = 0;
});