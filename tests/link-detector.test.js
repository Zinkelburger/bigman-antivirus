/**
 * Unit tests for LinkDetector class
 * Run with: npm test
 */

const LinkDetector = require('../link-detector.js');

describe('LinkDetector', () => {
    let detector;

    beforeEach(() => {
        detector = new LinkDetector();
    });

    describe('Domain extraction', () => {
        test('should extract domain from full URL', () => {
            expect(detector.extractDomain('https://example.com/path')).toBe('example.com');
        });

        test('should extract domain from text', () => {
            expect(detector.extractDomain('Go to google.com')).toBe('google.com');
        });

        test('should extract domain with www', () => {
            expect(detector.extractDomain('https://www.example.com')).toBe('www.example.com');
        });

        test('should extract domain from text with www', () => {
            expect(detector.extractDomain('Visit www.example.com')).toBe('www.example.com');
        });

        test('should return null for invalid input', () => {
            expect(detector.extractDomain('')).toBe(null);
            expect(detector.extractDomain(null)).toBe(null);
            expect(detector.extractDomain('not a domain')).toBe(null);
        });

        test('should handle malformed URLs', () => {
            expect(detector.extractDomain('https://')).toBe(null);
            expect(detector.extractDomain('http://invalid')).toBe(null);
        });
    });

    describe('Domain normalization', () => {
        test('should normalize full URL', () => {
            expect(detector.normalizeDomain('https://www.example.com/path?query=value')).toBe('example.com');
        });

        test('should normalize case', () => {
            expect(detector.normalizeDomain('EXAMPLE.COM')).toBe('example.com');
        });

        test('should normalize with fragment', () => {
            expect(detector.normalizeDomain('https://example.com#section')).toBe('example.com');
        });

        test('should handle www prefix', () => {
            expect(detector.normalizeDomain('www.example.com')).toBe('example.com');
        });

        test('should handle empty input', () => {
            expect(detector.normalizeDomain('')).toBe('');
            expect(detector.normalizeDomain(null)).toBe('');
        });
    });

    describe('Phone number extraction and normalization', () => {
        test('should extract phone number from text', () => {
            expect(detector.extractPhoneNumber('Call (555) 123-4567')).toBe('(555) 123-4567');
            expect(detector.extractPhoneNumber('Phone: 555-123-4567')).toBe('555-123-4567');
            expect(detector.extractPhoneNumber('+1 555.123.4567')).toBe('+1 555.123.4567');
        });

        test('should return null when no phone number found', () => {
            expect(detector.extractPhoneNumber('No phone here')).toBe(null);
            expect(detector.extractPhoneNumber('')).toBe(null);
        });

        test('should normalize phone numbers', () => {
            expect(detector.normalizePhone('(555) 123-4567')).toBe('5551234567');
            expect(detector.normalizePhone('+1 555-123-4567')).toBe('5551234567');
            expect(detector.normalizePhone('555.123.4567')).toBe('5551234567');
        });

        test('should handle US country code', () => {
            expect(detector.normalizePhone('+15551234567')).toBe('5551234567');
            expect(detector.normalizePhone('15551234567')).toBe('5551234567');
        });
    });

    describe('Email extraction', () => {
        test('should extract email from text', () => {
            expect(detector.extractEmail('Contact us at test@example.com')).toBe('test@example.com');
            expect(detector.extractEmail('Email: user.name+tag@domain.co.uk')).toBe('user.name+tag@domain.co.uk');
        });

        test('should return lowercase email', () => {
            expect(detector.extractEmail('TEST@EXAMPLE.COM')).toBe('test@example.com');
        });

        test('should return null when no email found', () => {
            expect(detector.extractEmail('No email here')).toBe(null);
            expect(detector.extractEmail('')).toBe(null);
        });
    });

    describe('Link analysis - HTTP/HTTPS links', () => {
        test('should detect domain mismatch', async () => {
            const result = await detector.analyzeLink('Go to google.com', 'https://evil-site.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Domain mismatch');
            expect(result.details.visibleDomain).toBe('google.com');
            expect(result.details.actualDomain).toBe('evil-site.com');
        });

        test('should allow exact domain match', async () => {
            const result = await detector.analyzeLink('Go to google.com', 'https://google.com');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No mismatch detected');
        });

        test('should allow www match', async () => {
            const result = await detector.analyzeLink('Go to google.com', 'https://www.google.com');
            expect(result.isSuspicious).toBe(false);
        });

        test('should allow no domain in text', async () => {
            const result = await detector.analyzeLink('Google Search', 'https://www.google.com');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No domain found in visible text to compare.');
        });

        test('should handle empty text', async () => {
            const result = await detector.analyzeLink('', 'https://example.com');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('Missing text or URL');
        });

        test('should handle empty URL', async () => {
            const result = await detector.analyzeLink('Go to example.com', '');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('Missing text or URL');
        });

        test('should detect invalid URL', async () => {
            const result = await detector.analyzeLink('Click here', 'not-a-url');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Link destination is not a valid URL.');
        });
    });

    describe('Link analysis - Email links', () => {
        test('should detect email mismatch', async () => {
            const result = await detector.analyzeLink('Contact test@example.com', 'mailto:different@evil.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Email address mismatch');
            expect(result.details.visibleIdentifier).toBe('test@example.com');
            expect(result.details.actualIdentifier).toBe('different@evil.com');
        });

        test('should allow exact email match', async () => {
            const result = await detector.analyzeLink('Contact test@example.com', 'mailto:test@example.com');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No email mismatch detected.');
        });

        test('should allow no email in text', async () => {
            const result = await detector.analyzeLink('Contact us', 'mailto:test@example.com');
            expect(result.isSuspicious).toBe(false);
        });

        test('should handle case insensitive email comparison', async () => {
            const result = await detector.analyzeLink('Contact TEST@EXAMPLE.COM', 'mailto:test@example.com');
            expect(result.isSuspicious).toBe(false);
        });
    });

    describe('Link analysis - Phone links', () => {
        test('should detect phone number mismatch', async () => {
            const result = await detector.analyzeLink('Call (555) 123-4567', 'tel:+1-555-999-8888');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Phone number mismatch');
            expect(result.details.visibleIdentifier).toBe('(555) 123-4567');
            expect(result.details.actualIdentifier).toBe('+1-555-999-8888');
        });

        test('should allow exact phone match', async () => {
            const result = await detector.analyzeLink('Call (555) 123-4567', 'tel:+1-555-123-4567');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No phone number mismatch detected.');
        });

        test('should allow no phone in text', async () => {
            const result = await detector.analyzeLink('Call us', 'tel:+1-555-123-4567');
            expect(result.isSuspicious).toBe(false);
        });

        test('should handle different phone formats', async () => {
            const result = await detector.analyzeLink('Call 555.123.4567', 'tel:+1-555-123-4567');
            expect(result.isSuspicious).toBe(false);
        });
    });

    describe('Link analysis - SMS links', () => {
        test('should detect SMS phone mismatch', async () => {
            const result = await detector.analyzeLink('Text (555) 123-4567', 'sms:+1-555-999-8888');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Phone number mismatch');
        });

        test('should allow exact SMS phone match', async () => {
            const result = await detector.analyzeLink('Text (555) 123-4567', 'sms:+1-555-123-4567');
            expect(result.isSuspicious).toBe(false);
        });
    });

    describe('Edge cases and error handling', () => {
        test('should handle null inputs', async () => {
            const result = await detector.analyzeLink(null, null);
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('Missing text or URL');
        });

        test('should handle undefined inputs', async () => {
            const result = await detector.analyzeLink(undefined, undefined);
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('Missing text or URL');
        });

        test('should handle special characters in domains', async () => {
            const result = await detector.analyzeLink('Go to test-site.com', 'https://test-site.com');
            expect(result.isSuspicious).toBe(false);
        });
    });

    describe('Punycode detection', () => {
        test('should detect punycode in URL with Cyrillic characters', async () => {
            const result = await detector.analyzeLink('Go to google.com', 'https://аррӏе.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Punycode detected (non-ASCII characters)');
            expect(result.details.originalUrl).toBe('https://аррӏе.com');
            expect(result.details.explanation).toContain('non-ASCII characters');
        });

        test('should detect punycode in URL with Chinese characters', async () => {
            const result = await detector.analyzeLink('Visit microsoft.com', 'https://微软.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Punycode detected (non-ASCII characters)');
        });

        test('should detect punycode in URL with Arabic characters', async () => {
            const result = await detector.analyzeLink('Go to facebook.com', 'https://فيسبوك.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Punycode detected (non-ASCII characters)');
        });

        test('should allow normal ASCII URLs', async () => {
            const result = await detector.analyzeLink('Go to google.com', 'https://google.com');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No mismatch detected');
        });

        test('should allow URLs with special ASCII characters', async () => {
            const result = await detector.analyzeLink('Go to test-site.com', 'https://test-site.com');
            expect(result.isSuspicious).toBe(false);
        });

        test('should handle empty URL in punycode detection', () => {
            const result = detector.detectPunycode('');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No URL provided');
        });

        test('should handle null URL in punycode detection', () => {
            const result = detector.detectPunycode(null);
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No URL provided');
        });

        test('should detect punycode in email URLs', async () => {
            const result = await detector.analyzeLink('Contact test@example.com', 'mailto:test@аррӏе.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Punycode detected (non-ASCII characters)');
        });

        test('should detect punycode in tel URLs', async () => {
            const result = await detector.analyzeLink('Call us', 'tel:+1-555-аррӏе-4567');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Punycode detected (non-ASCII characters)');
        });
    });

    describe('HTTP security detection', () => {
        test('should detect insecure HTTP URLs', async () => {
            const result = await detector.analyzeLink('Go to example.com', 'http://example.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Insecure HTTP connection detected');
            expect(result.details.originalUrl).toBe('http://example.com');
            expect(result.details.explanation).toContain('not encrypted');
        });

        test('should allow secure HTTPS URLs', async () => {
            const result = await detector.analyzeLink('Go to example.com', 'https://example.com');
            expect(result.isSuspicious).toBe(false);
        });

        test('should handle case insensitive HTTP detection', async () => {
            const result = await detector.analyzeLink('Go to example.com', 'HTTP://example.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Insecure HTTP connection detected');
        });

        test('should handle empty URL in HTTP detection', () => {
            const result = detector.detectInsecureHttp('');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No URL provided');
        });

        test('should handle null URL in HTTP detection', () => {
            const result = detector.detectInsecureHttp(null);
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No URL provided');
        });
    });

    describe('URL unshortening', () => {
        test('should handle non-HTTP URLs', async () => {
            const result = await detector.checkAndUnshortenUrl('mailto:test@example.com');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('Not an HTTP/HTTPS URL');
        });

        test('should handle empty URL in unshortening', async () => {
            const result = await detector.checkAndUnshortenUrl('');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No URL provided');
        });

        test('should handle null URL in unshortening', async () => {
            const result = await detector.checkAndUnshortenUrl(null);
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No URL provided');
        });

        test('should detect file downloads', () => {
            expect(detector.isFileDownload('https://example.com/file.pdf')).toBe(true);
            expect(detector.isFileDownload('https://example.com/document.docx')).toBe(true);
            expect(detector.isFileDownload('https://example.com/archive.zip')).toBe(true);
            expect(detector.isFileDownload('https://example.com/installer.exe')).toBe(true);
            expect(detector.isFileDownload('https://example.com/image.jpg')).toBe(true);
        });

        test('should not detect non-file URLs', () => {
            expect(detector.isFileDownload('https://example.com/page')).toBe(false);
            expect(detector.isFileDownload('https://example.com/')).toBe(false);
            expect(detector.isFileDownload('https://example.com/path/to/page')).toBe(false);
        });

        test('should handle case insensitive file detection', () => {
            expect(detector.isFileDownload('https://example.com/FILE.PDF')).toBe(true);
            expect(detector.isFileDownload('https://example.com/Document.DOCX')).toBe(true);
        });

        // Note: We can't easily test actual URL unshortening in unit tests without mocking axios
        // or making real network requests. In a real implementation, you might want to mock axios
        // for these tests to avoid network dependencies.
    });

    describe('Brand detection and typosquatting', () => {
        test('should detect brand name in non-canonical domain', () => {
            const result = detector.detectBrandTyposquatting('https://npmjs.help');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Brand name found but domain is not canonical');
            expect(result.details.brand).toBe('npm');
            expect(result.details.suspiciousDomain).toBe('npmjs.help');
            expect(result.details.canonicalDomains).toContain('npmjs.com');
        });

        test('should allow canonical brand domains', () => {
            const result = detector.detectBrandTyposquatting('https://npmjs.com');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('Valid canonical domain for brand');
        });

        test('should detect typosquatting with Levenshtein distance', () => {
            const result = detector.detectBrandTyposquatting('https://nompjs.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Potential typosquatting detected');
            expect(result.details.brand).toBe('npm');
            expect(result.details.levenshteinDistance).toBe(2);
            expect(result.details.suspiciousDomain).toBe('nompjs.com');
        });

        test('should detect microsoft typosquatting', () => {
            const result = detector.detectBrandTyposquatting('https://micrasoft.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Potential typosquatting detected');
            expect(result.details.brand).toBe('microsoft');
            expect(result.details.levenshteinDistance).toBe(1);
        });

        test('should detect google typosquatting', () => {
            const result = detector.detectBrandTyposquatting('https://gooogle.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Potential typosquatting detected');
            expect(result.details.brand).toBe('google');
            expect(result.details.levenshteinDistance).toBe(1);
        });

        test('should allow legitimate google domains', () => {
            const result = detector.detectBrandTyposquatting('https://google.com');
            expect(result.isSuspicious).toBe(false);
        });

        test('should allow legitimate google country domains', () => {
            const result = detector.detectBrandTyposquatting('https://google.co.uk');
            expect(result.isSuspicious).toBe(false);
        });

        test('should detect paypal typosquatting', () => {
            const result = detector.detectBrandTyposquatting('https://paypall.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Brand name found but domain is not canonical');
            expect(result.details.brand).toBe('paypal');
        });

        test('should detect apple typosquatting', () => {
            const result = detector.detectBrandTyposquatting('https://appie.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Potential typosquatting detected');
            expect(result.details.brand).toBe('apple');
            expect(result.details.levenshteinDistance).toBe(1);
        });

        test('should detect amazon typosquatting', () => {
            const result = detector.detectBrandTyposquatting('https://amazom.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Potential typosquatting detected');
            expect(result.details.brand).toBe('amazon');
            expect(result.details.levenshteinDistance).toBe(1);
        });

        test('should detect github typosquatting', () => {
            const result = detector.detectBrandTyposquatting('https://githab.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Potential typosquatting detected');
            expect(result.details.brand).toBe('github');
            expect(result.details.levenshteinDistance).toBe(1);
        });

        test('should not flag domains with high Levenshtein distance', () => {
            const result = detector.detectBrandTyposquatting('https://completely-different.com');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No brand typosquatting detected');
        });

        test('should handle non-HTTP URLs in brand detection', () => {
            const result = detector.detectBrandTyposquatting('mailto:test@npmjs.help');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('Not an HTTP/HTTPS URL');
        });

        test('should handle empty URL in brand detection', () => {
            const result = detector.detectBrandTyposquatting('');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No URL provided');
        });

        test('should handle null URL in brand detection', () => {
            const result = detector.detectBrandTyposquatting(null);
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No URL provided');
        });
    });

    describe('Levenshtein distance calculation', () => {
        test('should calculate correct distance for identical strings', () => {
            expect(detector.levenshteinDistance('google', 'google')).toBe(0);
        });

        test('should calculate correct distance for single character difference', () => {
            expect(detector.levenshteinDistance('google', 'gooogle')).toBe(1);
            expect(detector.levenshteinDistance('microsoft', 'micrasoft')).toBe(1);
        });

        test('should calculate correct distance for two character difference', () => {
            expect(detector.levenshteinDistance('google', 'goooglee')).toBe(2);
            expect(detector.levenshteinDistance('microsoft', 'micrasoftt')).toBe(2);
        });

        test('should calculate correct distance for completely different strings', () => {
            expect(detector.levenshteinDistance('google', 'amazon')).toBe(6);
            expect(detector.levenshteinDistance('microsoft', 'github')).toBe(8);
        });

        test('should handle empty strings', () => {
            expect(detector.levenshteinDistance('', '')).toBe(0);
            expect(detector.levenshteinDistance('google', '')).toBe(6);
            expect(detector.levenshteinDistance('', 'google')).toBe(6);
        });
    });

    describe('Brand domain validation', () => {
        test('should identify canonical domains correctly', () => {
            expect(detector.isCanonicalDomain('google.com', 'google')).toBe(true);
            expect(detector.isCanonicalDomain('google.co.uk', 'google')).toBe(true);
            expect(detector.isCanonicalDomain('npmjs.com', 'npm')).toBe(true);
            expect(detector.isCanonicalDomain('github.com', 'github')).toBe(true);
        });

        test('should reject non-canonical domains', () => {
            expect(detector.isCanonicalDomain('google.help', 'google')).toBe(false);
            expect(detector.isCanonicalDomain('npmjs.help', 'npm')).toBe(false);
            expect(detector.isCanonicalDomain('githab.com', 'github')).toBe(false);
        });

        test('should handle case insensitive domain comparison', () => {
            expect(detector.isCanonicalDomain('GOOGLE.COM', 'google')).toBe(true);
            expect(detector.isCanonicalDomain('Google.Co.UK', 'google')).toBe(true);
        });

        test('should handle invalid inputs', () => {
            expect(detector.isCanonicalDomain('', 'google')).toBe(false);
            expect(detector.isCanonicalDomain('google.com', '')).toBe(false);
            expect(detector.isCanonicalDomain('google.com', 'nonexistent')).toBe(false);
            expect(detector.isCanonicalDomain(null, 'google')).toBe(false);
            expect(detector.isCanonicalDomain('google.com', null)).toBe(false);
        });
    });

    describe('Brand extraction from domain', () => {
        test('should extract brand names from domains', () => {
            expect(detector.extractBrandFromDomain('google.com')).toBe('google');
            expect(detector.extractBrandFromDomain('npmjs.com')).toBe('npm');
            expect(detector.extractBrandFromDomain('github.com')).toBe('github');
            expect(detector.extractBrandFromDomain('microsoft.com')).toBe('microsoft');
        });

        test('should extract brand names from domains with www', () => {
            expect(detector.extractBrandFromDomain('www.google.com')).toBe('google');
            expect(detector.extractBrandFromDomain('www.npmjs.com')).toBe('npm');
        });

        test('should extract brand names from country-specific domains', () => {
            expect(detector.extractBrandFromDomain('google.co.uk')).toBe('google');
            expect(detector.extractBrandFromDomain('microsoft.de')).toBe('microsoft');
        });

        test('should extract brand names from subdomains (STLD detection)', () => {
            expect(detector.extractBrandFromDomain('mail.google.com')).toBe('google');
            expect(detector.extractBrandFromDomain('poop.google.com')).toBe('google');
            expect(detector.extractBrandFromDomain('sub1.sub2.google.com')).toBe('google');
            expect(detector.extractBrandFromDomain('www.mail.google.com')).toBe('google');
        });

        test('should extract brand names from subdomains with country TLDs', () => {
            expect(detector.extractBrandFromDomain('mail.google.co.uk')).toBe('google');
            expect(detector.extractBrandFromDomain('poop.microsoft.de')).toBe('microsoft');
        });

        test('should return null for domains without known brands', () => {
            expect(detector.extractBrandFromDomain('example.com')).toBe(null);
            expect(detector.extractBrandFromDomain('unknown-site.org')).toBe(null);
            expect(detector.extractBrandFromDomain('sub.example.com')).toBe(null);
        });

        test('should handle empty or invalid inputs', () => {
            expect(detector.extractBrandFromDomain('')).toBe(null);
            expect(detector.extractBrandFromDomain(null)).toBe(null);
        });
    });

    describe('Real-world phishing scenarios', () => {
        test('should detect common phishing pattern', () => {
            const result = detector.detectBrandTyposquatting('https://paypal-security-alert.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Brand name found but domain is not canonical');
            expect(result.details.brand).toBe('paypal');
        });

        test('should detect subdomain spoofing', () => {
            const result = detector.detectBrandTyposquatting('https://google.com.evil-site.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Brand name found but domain is not canonical');
            expect(result.details.brand).toBe('google');
        });

        test('should allow legitimate subdomains', () => {
            const result = detector.detectBrandTyposquatting('https://mail.google.com');
            expect(result.isSuspicious).toBe(false);
        });

        test('should detect npmjs.help as suspicious (example from user)', () => {
            const result = detector.detectBrandTyposquatting('https://npmjs.help');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Brand name found but domain is not canonical');
            expect(result.details.brand).toBe('npm');
            expect(result.details.suspiciousDomain).toBe('npmjs.help');
            expect(result.details.canonicalDomains).toEqual(['npmjs.com']);
        });

        test('should detect brand in subdomain (STLD detection)', () => {
            const result = detector.detectBrandTyposquatting('https://poop.google.com');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('Valid canonical domain for brand');
            expect(result.details.brand).toBe('google');
        });

        test('should detect brand in subdomain with non-canonical TLD', () => {
            const result = detector.detectBrandTyposquatting('https://poop.google.help');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Brand name found but domain is not canonical');
            expect(result.details.brand).toBe('google');
        });

        test('should detect brand in subdomain with typosquatting', () => {
            const result = detector.detectBrandTyposquatting('https://poop.gooogle.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Potential typosquatting detected');
            expect(result.details.brand).toBe('google');
            expect(result.details.levenshteinDistance).toBe(1);
        });

        test('should handle complex subdomains correctly', () => {
            const result = detector.detectBrandTyposquatting('https://mail.google.com');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('Valid canonical domain for brand');
            expect(result.details.brand).toBe('google');
        });

        test('should handle multiple subdomains', () => {
            const result = detector.detectBrandTyposquatting('https://sub1.sub2.google.com');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('Valid canonical domain for brand');
            expect(result.details.brand).toBe('google');
        });
    });
});
