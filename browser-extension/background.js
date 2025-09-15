// Background script for BigMan AntiVirus
// Loads configuration and provides it to content scripts

let config = null;

// Load configuration on extension startup
async function loadConfig() {
    try {
        const response = await fetch(chrome.runtime.getURL('config/brand-domains.json'));
        config = await response.json();
        console.log('BigMan AntiVirus: Configuration loaded in background');
    } catch (error) {
        console.error('BigMan AntiVirus: Failed to load configuration:', error);
    }
}

// Listen for messages from content scripts
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
    if (request.action === 'getConfig') {
        sendResponse({ config: config });
    }
    return true;
});

// Load config when extension starts
loadConfig();
