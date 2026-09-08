/** @jest-environment jsdom */
const fs = require('fs');
const LinkDetector = require('../link-detector');
const wait = () => new Promise(resolve => setTimeout(resolve, 350));
test('rescans href mutations, checks image links and clears stale markers', async () => {
    const listeners = [];
    global.chrome = {
        runtime: {
            sendMessage: jest.fn(async request => request.action === 'getSettings' ? { settings: {} } : {}),
            onMessage: { addListener: fn => listeners.push(fn) },
        },
    };
    global.BigmanLinkDetector = LinkDetector;
    document.body.innerHTML = '<a id="image" href="https://microsft.com"><img alt="Open"></a><a id="changing" href="https://example.org">Open</a>';
    eval(fs.readFileSync(require.resolve('../content.js'), 'utf8'));
    await wait();
    expect(document.getElementById('image').hasAttribute('data-bigman-warning')).toBe(true);
    const changing = document.getElementById('changing');
    changing.href = 'https://microsft.com'; await wait();
    expect(changing.hasAttribute('data-bigman-warning')).toBe(true);
    changing.href = 'https://example.org'; await wait();
    expect(changing.hasAttribute('data-bigman-warning')).toBe(false);
    const respond = jest.fn(); listeners[0]({ action: 'getScanResults' }, {}, respond);
    expect(respond.mock.calls[0][0].results.suspicious).toBe(1);
});
