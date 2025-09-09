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
        test('should detect domain mismatch', () => {
            const result = detector.analyzeLink('Go to google.com', 'https://evil-site.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Domain mismatch');
            expect(result.details.visibleDomain).toBe('google.com');
            expect(result.details.actualDomain).toBe('evil-site.com');
        });

        test('should allow exact domain match', () => {
            const result = detector.analyzeLink('Go to google.com', 'https://google.com');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No mismatch detected');
        });

        test('should allow www match', () => {
            const result = detector.analyzeLink('Go to google.com', 'https://www.google.com');
            expect(result.isSuspicious).toBe(false);
        });

        test('should allow no domain in text', () => {
            const result = detector.analyzeLink('Google Search', 'https://www.google.com');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No domain found in visible text to compare.');
        });

        test('should handle empty text', () => {
            const result = detector.analyzeLink('', 'https://example.com');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('Missing text or URL');
        });

        test('should handle empty URL', () => {
            const result = detector.analyzeLink('Go to example.com', '');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('Missing text or URL');
        });

        test('should detect invalid URL', () => {
            const result = detector.analyzeLink('Click here', 'not-a-url');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Link destination is not a valid URL.');
        });
    });

    describe('Link analysis - Email links', () => {
        test('should detect email mismatch', () => {
            const result = detector.analyzeLink('Contact test@example.com', 'mailto:different@evil.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Email address mismatch');
            expect(result.details.visibleIdentifier).toBe('test@example.com');
            expect(result.details.actualIdentifier).toBe('different@evil.com');
        });

        test('should allow exact email match', () => {
            const result = detector.analyzeLink('Contact test@example.com', 'mailto:test@example.com');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No email mismatch detected.');
        });

        test('should allow no email in text', () => {
            const result = detector.analyzeLink('Contact us', 'mailto:test@example.com');
            expect(result.isSuspicious).toBe(false);
        });

        test('should handle case insensitive email comparison', () => {
            const result = detector.analyzeLink('Contact TEST@EXAMPLE.COM', 'mailto:test@example.com');
            expect(result.isSuspicious).toBe(false);
        });
    });

    describe('Link analysis - Phone links', () => {
        test('should detect phone number mismatch', () => {
            const result = detector.analyzeLink('Call (555) 123-4567', 'tel:+1-555-999-8888');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Phone number mismatch');
            expect(result.details.visibleIdentifier).toBe('(555) 123-4567');
            expect(result.details.actualIdentifier).toBe('+1-555-999-8888');
        });

        test('should allow exact phone match', () => {
            const result = detector.analyzeLink('Call (555) 123-4567', 'tel:+1-555-123-4567');
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('No phone number mismatch detected.');
        });

        test('should allow no phone in text', () => {
            const result = detector.analyzeLink('Call us', 'tel:+1-555-123-4567');
            expect(result.isSuspicious).toBe(false);
        });

        test('should handle different phone formats', () => {
            const result = detector.analyzeLink('Call 555.123.4567', 'tel:+1-555-123-4567');
            expect(result.isSuspicious).toBe(false);
        });
    });

    describe('Link analysis - SMS links', () => {
        test('should detect SMS phone mismatch', () => {
            const result = detector.analyzeLink('Text (555) 123-4567', 'sms:+1-555-999-8888');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Phone number mismatch');
        });

        test('should allow exact SMS phone match', () => {
            const result = detector.analyzeLink('Text (555) 123-4567', 'sms:+1-555-123-4567');
            expect(result.isSuspicious).toBe(false);
        });
    });

    describe('Edge cases and error handling', () => {
        test('should handle null inputs', () => {
            const result = detector.analyzeLink(null, null);
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('Missing text or URL');
        });

        test('should handle undefined inputs', () => {
            const result = detector.analyzeLink(undefined, undefined);
            expect(result.isSuspicious).toBe(false);
            expect(result.reason).toBe('Missing text or URL');
        });

        test('should handle special characters in domains', () => {
            const result = detector.analyzeLink('Go to test-site.com', 'https://test-site.com');
            expect(result.isSuspicious).toBe(false);
        });
    });

    describe('Real-world phishing scenarios', () => {
        test('should detect common phishing pattern', () => {
            const result = detector.analyzeLink('Login to your PayPal.com account', 'https://paypal-security-alert.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Domain mismatch');
        });

        test('should detect subdomain spoofing', () => {
            const result = detector.analyzeLink('Go to google.com', 'https://google.com.evil-site.com');
            expect(result.isSuspicious).toBe(true);
            expect(result.reason).toBe('Domain mismatch');
        });

        test('should allow legitimate subdomains', () => {
            const result = detector.analyzeLink('Go to mail.google.com', 'https://mail.google.com');
            expect(result.isSuspicious).toBe(false);
        });
    });
});
