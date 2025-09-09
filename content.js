// BigMan AntiVirus - Content Script
// Scans for link mismatches between visible text and href

class PhishingDetector {
    constructor() {
        this.detector = new LinkDetector();
        this.init();
    }

    init() {
        console.log('BigMan AntiVirus: Initialized');
        this.scanPage();
        this.observeChanges();
    }

    scanPage() {
        const links = document.querySelectorAll('a[href]');
        console.log(`BigMan AntiVirus: Scanning ${links.length} links`);
        
        let suspiciousCount = 0;
        links.forEach(link => {
            const result = this.detector.analyzeLink(link.textContent, link.href);
            if (result.isSuspicious) {
                suspiciousCount++;
                this.markSuspiciousLink(link, result);
            }
        });
        
        if (suspiciousCount > 0) {
            console.warn(`BigMan AntiVirus: Found ${suspiciousCount} suspicious links`);
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
                    `Visible: ${result.details.visibleDomain}\n` +
                    `Actual: ${result.details.actualDomain}`;
        
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
}

// Initialize when page loads
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => new PhishingDetector());
} else {
    new PhishingDetector();
}
