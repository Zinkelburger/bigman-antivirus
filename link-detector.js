/**
 * BigMan AntiVirus - Link Mismatch Detector
 * 
 * Core functionality: Detect when visible text doesn't match the actual href URL
 */

class LinkDetector {
    constructor() {
        // Configuration for future extensions
        this.config = {
            // Add more config options here as needed
        };
    }

    /**
     * Main function: Analyze a link for mismatches
     * @param {string} visibleText - Text user sees
     * @param {string} hrefUrl - Actual URL destination
     * @returns {Object} Detection result
     */
    analyzeLink(visibleText, hrefUrl) {
        if (!visibleText || !hrefUrl) {
            return { 
                isSuspicious: false, 
                reason: 'Missing text or URL',
                details: {}
            };
        }

        
        const textDomain = this.extractDomain(visibleText);
        const hrefDomain = this.extractDomain(hrefUrl);

        // If the href is invalid, we can't proceed.
        if (!hrefDomain) {
            return { 
                isSuspicious: true, // An invalid href is suspicious
                reason: 'Link destination is not a valid URL.',
                details: { originalUrl: hrefUrl }
            };
        }
        
        // If no domain is in the visible text, there's no mismatch to detect.
        if (!textDomain) {
            return { 
                isSuspicious: false, 
                reason: 'No domain found in visible text to compare.',
                details: { actualDomain: this.normalizeDomain(hrefDomain) }
            };
        }

        const normalizedText = this.normalizeDomain(textDomain);
        const normalizedHref = this.normalizeDomain(hrefDomain);

        // Check for exact mismatch
        if (normalizedText !== normalizedHref) {
            return {
                isSuspicious: true,
                reason: 'Domain mismatch',
                details: {
                    visibleDomain: normalizedText,
                    actualDomain: normalizedHref,
                    originalText: visibleText.trim(),
                    originalUrl: hrefUrl
                }
            };
        }

        return { 
            isSuspicious: false, 
            reason: 'No mismatch detected',
            details: {
                visibleDomain: normalizedText,
                actualDomain: normalizedHref
            }
        };
    }

    /**
     * Extract domain from text or URL
     * @param {string} input - Text or URL
     * @returns {string|null} Extracted domain
     */
    extractDomain(input) {
        if (!input) return null;

        // For URLs, use URL constructor
        if (input.startsWith('http')) {
            try {
                return new URL(input).hostname;
            } catch (e) {
                return null;
            }
        }

        // For text, use regex to find domain-like patterns
        const domainMatch = input.match(/(?:https?:\/\/)?(?:www\.)?([a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}/);
        return domainMatch ? domainMatch[0] : null;
    }

    /**
     * Normalize domain for comparison
     * @param {string} domain - Domain to normalize
     * @returns {string} Normalized domain
     */
    normalizeDomain(domain) {
        if (!domain) return '';
        
        return domain.toLowerCase()
            .replace(/^https?:\/\//, '')  // Remove protocol
            .replace(/^www\./, '')        // Remove www
            .split('/')[0]                // Remove path
            .split('?')[0]                // Remove query params
            .split('#')[0];               // Remove fragment
    }

    /**
     * Extension point: Add more detection methods here
     * Example: detectHomographs, detectTyposquatting, etc.
     */
    
    // Future extension methods can be added here:
    // detectHomographs(textDomain, hrefDomain) { ... }
    // detectTyposquatting(textDomain, hrefDomain) { ... }
    // detectSuspiciousPatterns(url) { ... }
}

// Export for use in different environments
if (typeof window !== 'undefined') {
    window.LinkDetector = LinkDetector;
} else if (typeof module !== 'undefined' && module.exports) {
    module.exports = LinkDetector;
}
