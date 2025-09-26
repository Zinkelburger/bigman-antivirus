// background.js
// Main background script for BigMan AntiVirus.
// Loads configuration and other modules.

// Import the PDF scanner logic from its own file.
// This line must be at the top level, not inside a function.
try {
    importScripts('pdf-listener.js');
} catch (e) {
    console.error('BigMan AntiVirus: Failed to import pdf-listener.js', e);
}

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
        // If config isn't loaded yet, load it, then respond.
        if (!config) {
            loadConfig().then(() => {
                sendResponse({ config: config });
            });
            return true; // Indicates an async response
        }
        sendResponse({ config: config });
    }
    return true; // Keep the message channel open for async responses
});

// Load config when the extension first starts
loadConfig();