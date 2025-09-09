/**
 * BigMan AntiVirus - Link Mismatch Detector
 * * Core functionality: Detect when visible text doesn't match the actual href URL.
 * Now with content validation for mailto:, tel:, and sms: protocols.
 * Includes URL unshortening and HTTP/HTTPS security checks.
 */

const axios = require('axios');

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
     * @returns {Promise<Object>} Detection result
     */
    async analyzeLink(visibleText, hrefUrl) {
        if (!visibleText || !hrefUrl) {
            return { 
                isSuspicious: false, 
                reason: 'Missing text or URL',
                details: {}
            };
        }

        // Check for punycode in URL
        const punycodeResult = this.detectPunycode(hrefUrl);
        if (punycodeResult.isSuspicious) {
            return punycodeResult;
        }

        // Check for HTTP (insecure) URLs
        const httpResult = this.detectInsecureHttp(hrefUrl);
        if (httpResult.isSuspicious) {
            return httpResult;
        }

        // Check for shortened URLs and unshorten them (only for HTTP/HTTPS)
        if (hrefUrl.toLowerCase().startsWith('http://') || hrefUrl.toLowerCase().startsWith('https://')) {
            const unshortenResult = await this.checkAndUnshortenUrl(hrefUrl);
            if (unshortenResult.isSuspicious) {
                return unshortenResult;
            }
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

    /**
     * Detect punycode usage in URLs (non-ASCII characters)
     * @param {string} url - URL to check for punycode
     * @returns {Object} Detection result
     */
    detectPunycode(url) {
        if (!url) {
            return { 
                isSuspicious: false, 
                reason: 'No URL provided',
                details: {}
            };
        }

        // Check if URL contains non-ASCII characters
        const hasNonAscii = /[^\x00-\x7F]/.test(url);
        
        if (hasNonAscii) {
            return {
                isSuspicious: true,
                reason: 'Punycode detected (non-ASCII characters)',
                details: {
                    originalUrl: url,
                    explanation: 'This URL contains non-ASCII characters which may indicate punycode usage. This can be used to disguise malicious domains.'
                }
            };
        }

        return { 
            isSuspicious: false, 
            reason: 'No punycode detected',
            details: {}
        };
    }

    /**
     * Detect insecure HTTP URLs
     * @param {string} url - URL to check
     * @returns {Object} Detection result
     */
    detectInsecureHttp(url) {
        if (!url) {
            return { 
                isSuspicious: false, 
                reason: 'No URL provided',
                details: {}
            };
        }

        // Check if URL uses HTTP (insecure)
        if (url.toLowerCase().startsWith('http://')) {
            return {
                isSuspicious: true,
                reason: 'Insecure HTTP connection detected',
                details: {
                    originalUrl: url,
                    explanation: 'This URL uses HTTP instead of HTTPS, which means your connection is not encrypted and could be intercepted.'
                }
            };
        }

        return { 
            isSuspicious: false, 
            reason: 'No insecure HTTP detected',
            details: {}
        };
    }

    /**
     * Check if URL is shortened and unshorten it
     * @param {string} url - URL to check and potentially unshorten
     * @returns {Promise<Object>} Detection result
     */
    async checkAndUnshortenUrl(url) {
        if (!url) {
            return { 
                isSuspicious: false, 
                reason: 'No URL provided',
                details: {}
            };
        }

        // Only check HTTP/HTTPS URLs
        if (!url.toLowerCase().startsWith('http://') && !url.toLowerCase().startsWith('https://')) {
            return { 
                isSuspicious: false, 
                reason: 'Not an HTTP/HTTPS URL',
                details: {}
            };
        }

        try {
            const unshortenedUrl = await this.unshortenUrl(url);
            
            if (unshortenedUrl && unshortenedUrl !== url) {
                const originalDomain = this.extractDomain(url);
                const unshortenedDomain = this.extractDomain(unshortenedUrl);
                
                // Normalize domains for comparison (remove www prefix)
                const normalizedOriginal = originalDomain ? this.normalizeDomain(originalDomain) : '';
                const normalizedUnshortened = unshortenedDomain ? this.normalizeDomain(unshortenedDomain) : '';
                
                // Check if redirects to different domain
                if (normalizedOriginal && normalizedUnshortened && normalizedOriginal !== normalizedUnshortened) {
                    return {
                        isSuspicious: true,
                        reason: 'Redirects to different site',
                        details: {
                            originalUrl: url,
                            unshortenedUrl: unshortenedUrl,
                            explanation: `This URL redirects to: ${unshortenedUrl}`
                        }
                    };
                }
                
                // Check if leads to file download
                if (this.isFileDownload(unshortenedUrl)) {
                    return {
                        isSuspicious: true,
                        reason: 'Leads to file download',
                        details: {
                            originalUrl: url,
                            unshortenedUrl: unshortenedUrl,
                            explanation: `This URL leads to a file download: ${unshortenedUrl}`
                        }
                    };
                }
            }

            return { 
                isSuspicious: false, 
                reason: 'No suspicious redirect detected',
                details: {}
            };
        } catch (error) {
            // Don't mark network errors as suspicious - just log and continue
            console.warn('BigMan AntiVirus: Could not unshorten URL:', url, error.message);
            return { 
                isSuspicious: false, 
                reason: 'Could not verify URL (network error)',
                details: {}
            };
        }
    }

    /**
     * Unshorten a URL by following redirects
     * @param {string} shortUrl - URL to unshorten
     * @returns {Promise<string|null>} Unshortened URL or null if error
     */
    async unshortenUrl(shortUrl) {
        try {
            // Make a HEAD request and stop it from following redirects automatically
            const response = await axios.head(shortUrl, {
                maxRedirects: 0, // This is the crucial part!
                validateStatus: status => status >= 200 && status < 400, // Accept redirects as valid
                timeout: 5000 // 5 second timeout
            });

            // Check if the response has a 'location' header (the un-shortened URL)
            if (response.headers.location) {
                return response.headers.location;
            } else {
                // The URL was not a redirect, it's the final destination.
                return shortUrl;
            }
        } catch (error) {
            if (error.response && error.response.status >= 300 && error.response.status < 400) {
                // This is a redirect response, get the location header
                return error.response.headers.location || shortUrl;
            }
            throw error;
        }
    }

    /**
     * Check if a URL leads to a file download
     * @param {string} url - URL to check
     * @returns {boolean} True if leads to file download
     */
    isFileDownload(url) {
        if (!url) return false;
        
        // Common file extensions that indicate downloads
        const fileExtensions = [
            '.exe', '.msi', '.dmg', '.pkg', '.deb', '.rpm', '.zip', '.rar', '.7z', '.tar', '.gz',
            '.pdf', '.doc', '.docx', '.xls', '.xlsx', '.ppt', '.pptx', '.odt', '.ods', '.odp',
            '.jpg', '.jpeg', '.png', '.gif', '.bmp', '.svg', '.ico', '.webp',
            '.mp3', '.mp4', '.avi', '.mov', '.wmv', '.flv', '.mkv', '.webm',
            '.iso', '.img', '.bin', '.apk', '.ipa', '.app', '.jar', '.war'
        ];
        
        const urlLower = url.toLowerCase();
        
        // Check if URL ends with a file extension
        return fileExtensions.some(ext => urlLower.endsWith(ext));
    }
}

// Export for use in different environments
if (typeof window !== 'undefined') {
    window.LinkDetector = LinkDetector;
} else if (typeof module !== 'undefined' && module.exports) {
    module.exports = LinkDetector;
}