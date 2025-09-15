/**
 * BigMan AntiVirus - Link Mismatch Detector
 * * Core functionality: Detect when visible text doesn't match the actual href URL.
 * Now with content validation for mailto:, tel:, and sms: protocols.
 * Includes URL unshortening and HTTP/HTTPS security checks.
 */


class LinkDetector {
    constructor() {
        // Check if config is already cached globally
        if (LinkDetector.cachedConfig) {
            this.loadFromCache();
            this.configLoaded = Promise.resolve();
        } else {
            // Load configuration from background script and cache it
            this.configLoaded = this.loadConfiguration();
        }
    }

    /**
     * Load configuration from cached data
     */
    loadFromCache() {
        const config = LinkDetector.cachedConfig;
        this.brandDomains = config.brandDomains;
        this.typosquattingThreshold = config.typosquattingThreshold;
        this.multiLevelTlds = new Set(config.multiLevelTlds);
        console.log('BigMan AntiVirus: Configuration loaded from cache');
    }

    /**
     * Load configuration from background script and cache it
     * @returns {Promise<void>} Promise that resolves when config is loaded
     */
    async loadConfiguration() {
        const response = await chrome.runtime.sendMessage({ action: 'getConfig' });
        const config = response.config;
        
        // Cache globally for future instances
        LinkDetector.cachedConfig = config;
        
        this.brandDomains = config.brandDomains;
        this.typosquattingThreshold = config.typosquattingThreshold;
        this.multiLevelTlds = new Set(config.multiLevelTlds);
        
        console.log('BigMan AntiVirus: Configuration loaded from background and cached');
    }

    /**
     * Main function: Analyze a link for mismatches
     * @param {string} visibleText - Text user sees
     * @param {string} hrefUrl - Actual URL destination
     * @returns {Promise<Object>} Detection result
     */
    async analyzeLink(visibleText, hrefUrl) {
        // Ensure configuration is loaded before proceeding
        await this.configLoaded;
        
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

        // Check for brand name typosquatting
        const brandResult = this.detectBrandTyposquatting(hrefUrl);
        if (brandResult.isSuspicious) {
            return brandResult;
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
        // DISABLED: HEAD requests were causing issues with Google and other services
        // Commenting out to prevent potential IP banning
        
        // try {
        //     // Make a HEAD request with no redirect following
        //     const response = await fetch(shortUrl, {
        //         method: 'HEAD',
        //         redirect: 'manual', // Don't follow redirects automatically
        //         signal: AbortSignal.timeout(5000) // 5 second timeout
        //     });

        //     // Check if the response is a redirect (3xx status codes)
        //     if (response.status >= 300 && response.status < 400) {
        //         const location = response.headers.get('location');
        //         if (location) {
        //             return location;
        //         }
        //     }
            
        //     // The URL was not a redirect, it's the final destination.
        //     return shortUrl;
        // } catch (error) {
        //     // Don't throw errors for network issues - just return original URL
        //     console.warn('BigMan AntiVirus: Could not unshorten URL:', shortUrl, error.message);
        //     throw error;
        // }

        // For now, just return the original URL without attempting to unshorten
        return shortUrl;
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

    /**
     * Calculate Levenshtein distance between two strings
     * @param {string} str1 - First string
     * @param {string} str2 - Second string
     * @returns {number} Levenshtein distance
     */
    levenshteinDistance(str1, str2) {
        const matrix = [];
        
        for (let i = 0; i <= str2.length; i++) {
            matrix[i] = [i];
        }
        
        for (let j = 0; j <= str1.length; j++) {
            matrix[0][j] = j;
        }
        
        for (let i = 1; i <= str2.length; i++) {
            for (let j = 1; j <= str1.length; j++) {
                if (str2.charAt(i - 1) === str1.charAt(j - 1)) {
                    matrix[i][j] = matrix[i - 1][j - 1];
                } else {
                    matrix[i][j] = Math.min(
                        matrix[i - 1][j - 1] + 1, // substitution
                        matrix[i][j - 1] + 1,     // insertion
                        matrix[i - 1][j] + 1      // deletion
                    );
                }
            }
        }
        
        return matrix[str2.length][str1.length];
    }

    /**
     * Extract the registrable domain part (main brand part) from a domain
     * Uses a lightweight Public Suffix List approach
     * @param {string} domain - Domain to analyze
     * @returns {string|null} Registrable domain or null
     */
    getRegistrableDomain(domain) {
        if (!domain) return null;
        
        // Remove www prefix and normalize
        const cleanDomain = domain.toLowerCase().replace(/^www\./, '');
        const parts = cleanDomain.split('.');
        
        if (parts.length < 2) return null;
        
        // Check for multi-level TLDs
        for (let i = parts.length - 2; i >= 0; i--) {
            const possibleTld = parts.slice(i).join('.');
            if (this.multiLevelTlds.has(possibleTld)) {
                // Found a multi-level TLD, the registrable domain is the part before it
                if (i > 0) {
                    return parts[i - 1];
                }
                return null; // Edge case: domain is just the TLD
            }
        }
        
        // No multi-level TLD found, assume single-level TLD
        // Return the second-to-last part as the registrable domain
        return parts[parts.length - 2];
    }

    /**
     * Extract potential brand name from domain
     * @param {string} domain - Domain to analyze
     * @returns {string|null} Potential brand name or null
     */
    extractBrandFromDomain(domain) {
        if (!domain) return null;
        
        // Remove www prefix and normalize
        const cleanDomain = domain.toLowerCase().replace(/^www\./, '');
        const parts = cleanDomain.split('.');
        
        // Check each part of the domain for brand names
        for (const part of parts) {
            for (const brandName of Object.keys(this.brandDomains)) {
                // Check for exact match
                if (part === brandName) {
                    return brandName;
                }
                
                // Check if brand name is contained in the domain part
                if (part.includes(brandName)) {
                    return brandName;
                }
                
                // Check if any canonical domain matches this part
                for (const canonicalDomain of this.brandDomains[brandName]) {
                    const canonicalPart = this.getRegistrableDomain(canonicalDomain);
                    if (canonicalPart && part === canonicalPart) {
                        return brandName;
                    }
                }
            }
        }
        
        return null;
    }

    /**
     * Check if domain is a valid canonical domain for a brand
     * @param {string} domain - Domain to check
     * @param {string} brandName - Brand name
     * @returns {boolean} True if domain is canonical for brand
     */
    isCanonicalDomain(domain, brandName) {
        if (!domain || !brandName || !this.brandDomains[brandName]) {
            return false;
        }
        
        const normalizedDomain = this.normalizeDomain(domain);
        
        // Check if it's an exact match with canonical domains
        if (this.brandDomains[brandName].includes(normalizedDomain)) {
            return true;
        }
        
        // Check if it's a subdomain of a canonical domain
        for (const canonicalDomain of this.brandDomains[brandName]) {
            if (normalizedDomain.endsWith('.' + canonicalDomain)) {
                return true;
            }
        }
        
        return false;
    }

    /**
     * Detect brand name typosquatting in URLs
     * @param {string} url - URL to check
     * @returns {Object} Detection result
     */
    detectBrandTyposquatting(url) {
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

        const domain = this.extractDomain(url);
        if (!domain) {
            return { 
                isSuspicious: false, 
                reason: 'No valid domain found',
                details: {}
            };
        }

        const normalizedDomain = this.normalizeDomain(domain);
        
        // Extract brand from domain using improved logic
        const detectedBrand = this.extractBrandFromDomain(domain);
        
        if (detectedBrand) {
            // Found a brand name in the domain, check if it's canonical
            if (this.isCanonicalDomain(domain, detectedBrand)) {
                return { 
                    isSuspicious: false, 
                    reason: 'Valid canonical domain for brand',
                    details: { brand: detectedBrand, domain: normalizedDomain }
                };
            } else {
                // Brand name found but not canonical - suspicious!
                return {
                    isSuspicious: true,
                    reason: 'Brand name found but domain is not canonical',
                    details: {
                        brand: detectedBrand,
                        suspiciousDomain: normalizedDomain,
                        canonicalDomains: this.brandDomains[detectedBrand],
                        explanation: `Domain "${normalizedDomain}" contains brand "${detectedBrand}" but is not a known canonical domain. This could be typosquatting.`
                    }
                };
            }
        }

        // Check for typosquatting using Levenshtein distance
        // Get the main registrable domain part for comparison
        const mainDomainPart = this.getRegistrableDomain(domain);
        if (!mainDomainPart) {
            return { 
                isSuspicious: false, 
                reason: 'Could not extract registrable domain',
                details: {}
            };
        }
        
        for (const brandName of Object.keys(this.brandDomains)) {
            // Skip if we already detected this brand exactly
            if (detectedBrand === brandName) {
                continue;
            }
            
            // Check against both the brand name and the canonical domain names
            let minDistance = Infinity;
            let closestMatch = brandName;
            
            // Check distance to brand name
            const brandDistance = this.levenshteinDistance(mainDomainPart, brandName);
            if (brandDistance < minDistance) {
                minDistance = brandDistance;
                closestMatch = brandName;
            }
            
            // Check distance to canonical domain names (registrable parts)
            for (const canonicalDomain of this.brandDomains[brandName]) {
                const canonicalMainPart = this.getRegistrableDomain(canonicalDomain);
                if (canonicalMainPart) {
                    const canonicalDistance = this.levenshteinDistance(mainDomainPart, canonicalMainPart);
                    if (canonicalDistance < minDistance) {
                        minDistance = canonicalDistance;
                        closestMatch = canonicalMainPart;
                    }
                }
            }
            
            // If distance is small and domain is not canonical, it's suspicious
            if (minDistance <= this.typosquattingThreshold && minDistance > 0) {
                if (!this.isCanonicalDomain(domain, brandName)) {
                    return {
                        isSuspicious: true,
                        reason: 'Potential typosquatting detected',
                        details: {
                            brand: brandName,
                            suspiciousDomain: normalizedDomain,
                            registrablePart: mainDomainPart,
                            levenshteinDistance: minDistance,
                            closestMatch: closestMatch,
                            canonicalDomains: this.brandDomains[brandName],
                            explanation: `Domain "${normalizedDomain}" has registrable part "${mainDomainPart}" which is very similar to brand "${brandName}" (distance: ${minDistance} from "${closestMatch}") but is not a canonical domain. This could be typosquatting.`
                        }
                    };
                }
            }
        }

        return { 
            isSuspicious: false, 
            reason: 'No brand typosquatting detected',
            details: {}
        };
    }
}

// Export for use in different environments
if (typeof window !== 'undefined') {
    window.LinkDetector = LinkDetector;
} else if (typeof module !== 'undefined' && module.exports) {
    module.exports = LinkDetector;
}