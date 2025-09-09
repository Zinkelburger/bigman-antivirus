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

    scanPage() {
        const links = document.querySelectorAll('a[href]');
        console.log(`BigMan AntiVirus: Scanning ${links.length} links`);
        
        // Reset results
        this.scanResults = {
            total: links.length,
            suspicious: 0,
            details: [],
            lastScanTime: new Date()
        };
        
        links.forEach(link => {
            const result = this.detector.analyzeLink(link.textContent, link.href);
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
        });
        
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
        link.title = `⚠️ SUSPICIOUS LINK DETECTED ⚠️\n` +
                    `Reason: ${result.reason}\n` +
                    `Visible: ${result.details.visibleDomain || result.details.visibleIdentifier}\n` +
                    `Actual: ${result.details.actualDomain || result.details.actualIdentifier}`;
        
        // Click warning
        link.addEventListener('click', (e) => {
            if (!confirm(`⚠️ WARNING: This link appears suspicious!\n\n` +
                        `You're about to visit: ${link.href}\n` +
                        `Reason: ${result.reason}\n\n` +
                        `Do you want to continue?`)) {
                e.preventDefault();
                e.stopPropagation();
                return false;
            }
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
                this.scanPage();
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
                this.scanPage();
                sendResponse({
                    success: true,
                    results: this.scanResults
                });
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