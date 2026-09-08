// Run with BIGMAN_PLAYWRIGHT_MODULE pointing to a local playwright installation.
const { chromium } = require(process.env.BIGMAN_PLAYWRIGHT_MODULE || 'playwright');
const path = require('path');
const assert = require('assert');
(async () => {
    const browser = await chromium.launch({ executablePath: process.env.BIGMAN_CHROME || '/usr/bin/google-chrome', headless: true, args: ['--no-sandbox'] });
    try {
        const page = await browser.newPage();
        await page.route('**/*', route => route.abort());
        await page.setContent('<h1>Link check fixture</h1><a id="image" href="https://microsft.com"><img alt="Open image link"></a><a id="mutable" href="https://example.org">Open link</a>');
        await page.evaluate(() => {
            window.messages = [];
            window.chrome = { runtime: { sendMessage: async m => { window.messages.push(m); return m.action === 'getSettings' ? { settings: {} } : {}; }, onMessage: { addListener: () => {} } } };
        });
        await page.addScriptTag({ path: path.resolve('browser-extension/dist/link-detector.js') });
        await page.addScriptTag({ path: path.resolve('browser-extension/content.js') });
        await page.waitForSelector('#image[data-bigman-warning]');
        await page.click('#image');
        assert(await page.evaluate(() => messages.some(m => m.action === 'warn' && m.url === 'https://microsft.com/')));
        assert.equal(page.url(), 'about:blank');
        await page.evaluate(() => document.getElementById('mutable').href = 'https://microsft.com');
        await page.waitForSelector('#mutable[data-bigman-warning]');
        await page.evaluate(() => document.getElementById('mutable').href = 'https://example.org');
        await page.waitForFunction(() => !document.getElementById('mutable').hasAttribute('data-bigman-warning'));
        console.log('PASS: built extension bundle, real browser image-link click interception, href mutation and warning removal');
    } finally { await browser.close(); }
})().catch(e => { console.error(e); process.exit(1); });
