// BigMan AntiVirus - Content Script
// Single source of truth for link scanning - stores results and responds to popup requests

class PhishingDetector {
    constructor() {
        this.detector = new LinkDetector();
        this.scanResults = {
            total: 0,
            suspicious: 0,
            details: [],
            lastScanTime: null
        };
        this.init();
    }

    init() {
        console.log('BigMan AntiVirus: Initialized');
        this.scanPage();
        this.observeChanges();
        this.setupMessageListener();
    }

    async scanPage() {
        const links = document.querySelectorAll('a[href]');
        console.log(`BigMan AntiVirus: Scanning ${links.length} links`);
        
        // Reset results
        this.scanResults = {
            total: links.length,
            suspicious: 0,
            details: [],
            lastScanTime: new Date()
        };
        
        // Process links asynchronously
        const linkPromises = Array.from(links).map(async (link) => {
            try {
                const result = await this.detector.analyzeLink(link.textContent, link.href);
                if (result.isSuspicious) {
                    this.scanResults.suspicious++;
                    this.scanResults.details.push({
                        text: link.textContent,
                        url: link.href,
                        reason: result.reason,
                        details: result.details
                    });
                    this.markSuspiciousLink(link, result);
                }
            } catch (error) {
                console.error('BigMan AntiVirus: Error analyzing link:', error);
                // Mark as suspicious if we can't analyze it
                this.scanResults.suspicious++;
                this.scanResults.details.push({
                    text: link.textContent,
                    url: link.href,
                    reason: 'Error analyzing link',
                    details: { error: error.message }
                });
                this.markSuspiciousLink(link, {
                    isSuspicious: true,
                    reason: 'Error analyzing link',
                    details: { error: error.message }
                });
            }
        });
        
        // Wait for all link analyses to complete
        await Promise.all(linkPromises);
        
        if (this.scanResults.suspicious > 0) {
            console.warn(`BigMan AntiVirus: Found ${this.scanResults.suspicious} suspicious links`);
        } else {
            console.log('BigMan AntiVirus: No suspicious links detected');
        }
    }

    markSuspiciousLink(link, result) {
        // Visual indicators
        link.style.border = '2px solid #ff4444';
        link.style.backgroundColor = 'rgba(255, 68, 68, 0.1)';
        link.style.borderRadius = '3px';
        link.style.padding = '2px 4px';
        
        // Tooltip with details
        let tooltipText = `⚠️ SUSPICIOUS LINK DETECTED ⚠️\nReason: ${result.reason}\n`;
        
        if (result.details.unshortenedUrl) {
            tooltipText += `Real destination: ${result.details.unshortenedUrl}`;
        } else if (result.details.visibleDomain || result.details.visibleIdentifier) {
            tooltipText += `Visible: ${result.details.visibleDomain || result.details.visibleIdentifier}\n`;
            tooltipText += `Actual: ${result.details.actualDomain || result.details.actualIdentifier}`;
        } else {
            tooltipText += `URL: ${link.href}`;
        }
        
        link.title = tooltipText;
        
        // Just log the warning, no annoying popups
        link.addEventListener('click', (e) => {
            console.log('BigMan AntiVirus: User clicked suspicious link:', link.href, 'Reason:', result.reason);
        });
    }

    observeChanges() {
        const observer = new MutationObserver((mutations) => {
            let shouldScan = false;
            mutations.forEach((mutation) => {
                mutation.addedNodes.forEach((node) => {
                    if (node.nodeType === Node.ELEMENT_NODE) {
                        if (node.tagName === 'A' || node.querySelector('a')) {
                            shouldScan = true;
                        }
                    }
                });
            });
            
            if (shouldScan) {
                this.scanPage().catch(error => {
                    console.error('BigMan AntiVirus: Error during page scan:', error);
                });
            }
        });
        
        observer.observe(document.body, { 
            childList: true, 
            subtree: true 
        });
    }

    setupMessageListener() {
        // Listen for messages from popup
        chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
            if (request.action === 'getScanResults') {
                sendResponse({
                    success: true,
                    results: this.scanResults
                });
            } else if (request.action === 'rescan') {
                this.scanPage().then(() => {
                    sendResponse({
                        success: true,
                        results: this.scanResults
                    });
                }).catch(error => {
                    console.error('BigMan AntiVirus: Error during rescan:', error);
                    sendResponse({
                        success: false,
                        error: error.message
                    });
                });
                return true; // Keep message channel open for async response
            }
            return true; // Keep message channel open for async response
        });
    }
}

// Initialize when page loads
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => new PhishingDetector());
} else {
    new PhishingDetector();
}