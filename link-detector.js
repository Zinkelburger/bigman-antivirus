/**
 * BigMan AntiVirus - Link Mismatch Detector
 * * Core functionality: Detect when visible text doesn't match the actual href URL.
 * Now with content validation for mailto:, tel:, and sms: protocols.
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

        const schemeMatch = hrefUrl.match(/^([a-z][a-z0-9+\-.]*):/i);
        const scheme = schemeMatch ? schemeMatch[1].toLowerCase() : null;

        // Handle telephone and SMS links
        if (scheme === 'tel' || scheme === 'sms') {
            const hrefPhone = this.normalizePhone(hrefUrl);
            const extractedTextPhone = this.extractPhoneNumber(visibleText);
            
            if (extractedTextPhone) {
                const textPhone = this.normalizePhone(extractedTextPhone);
                if (hrefPhone !== textPhone) {
                    return {
                        isSuspicious: true,
                        reason: 'Phone number mismatch',
                        details: {
                            visibleIdentifier: extractedTextPhone.trim(),
                            actualIdentifier: hrefUrl.replace(/^(tel:|sms:)/i, '').trim()
                        }
                    };
                }
            }
            return { isSuspicious: false, reason: 'No phone number mismatch detected.', details: {} };
        }

        // Handle email links
        if (scheme === 'mailto') {
            const hrefEmail = this.extractEmail(hrefUrl);
            const textEmail = this.extractEmail(visibleText);

            if (textEmail && hrefEmail && textEmail !== hrefEmail) {
                 return {
                    isSuspicious: true,
                    reason: 'Email address mismatch',
                    details: {
                        visibleIdentifier: textEmail,
                        actualIdentifier: hrefEmail
                    }
                };
            }
            return { isSuspicious: false, reason: 'No email mismatch detected.', details: {} };
        }

        // Fallback to domain comparison for http, https, ftp, etc.
        const textDomain = this.extractDomain(visibleText);
        const hrefDomain = this.extractDomain(hrefUrl);

        if (!hrefDomain) {
            return { 
                isSuspicious: true,
                reason: 'Link destination is not a valid URL.',
                details: { originalUrl: hrefUrl }
            };
        }
        
        if (!textDomain) {
            return { 
                isSuspicious: false, 
                reason: 'No domain found in visible text to compare.',
                details: { actualDomain: this.normalizeDomain(hrefDomain) }
            };
        }

        const normalizedText = this.normalizeDomain(textDomain);
        const normalizedHref = this.normalizeDomain(hrefDomain);

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
     * NEW: Extracts a reasonably formatted phone number from a string.
     * @param {string} input - The string to search.
     * @returns {string|null} The found phone number string or null.
     */
    extractPhoneNumber(input) {
        if (!input) return null;
        // This regex finds common US-style phone number formats.
        const phoneMatch = input.match(/((?:\+?1\s*?)?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4})/);
        return phoneMatch ? phoneMatch[0] : null;
    }

    /**
     * NEW: Normalizes a phone number string to only digits for comparison.
     * @param {string} phoneStr - The string containing a phone number.
     * @returns {string} A string of digits, with US country code removed.
     */
    normalizePhone(phoneStr) {
        if (!phoneStr) return '';
        const digits = phoneStr.replace(/\D/g, '');
        // Standardize by removing the US country code if present
        if (digits.length === 11 && digits.startsWith('1')) {
            return digits.slice(1);
        }
        return digits;
    }

    /**
     * NEW: Extracts the first valid email address from a string.
     * @param {string} input - The string to search.
     * @returns {string|null} The found email address in lowercase or null.
     */
    extractEmail(input) {
        if (!input) return null;
        // This regex finds a standard email address pattern within a larger string.
        const emailMatch = input.match(/\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b/i);
        return emailMatch ? emailMatch[0].toLowerCase() : null;
    }

    /**
     * Extract domain from text or URL for HTTP/S links.
     * @param {string} input - Text or URL
     * @returns {string|null} Extracted domain
     */
    extractDomain(input) {
        if (!input) return null;
        if (input.startsWith('http')) {
            try {
                // need a dot, e.g. https://invalid shouldn't work
                const hostname = new URL(input).hostname;
                return hostname.includes('.') ? hostname : null;
            } catch (e) {
                return null;
            }
        }
        const domainMatch = input.match(/(?:https?:\/\/)?(?:www\.)?([a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}/);
        return domainMatch ? domainMatch[0] : null;
    }

    /**
     * Normalize domain for comparison for HTTP/S links.
     * @param {string} domain - Domain to normalize
     * @returns {string} Normalized domain
     */
    normalizeDomain(domain) {
        if (!domain) return '';
        return domain.toLowerCase()
            .replace(/^https?:\/\//, '')
            .replace(/^www\./, '')
            .split('/')[0]
            .split('?')[0]
            .split('#')[0];
    }
}

// Export for use in different environments
if (typeof window !== 'undefined') {
    window.LinkDetector = LinkDetector;
} else if (typeof module !== 'undefined' && module.exports) {
    module.exports = LinkDetector;
}