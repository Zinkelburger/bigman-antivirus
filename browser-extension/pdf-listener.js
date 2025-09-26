// pdf-listener.js
// Handles all logic for detecting and scanning downloaded PDF files.

/**
 * Handles the response from the native Rust program.
 * @param {object} response - The JSON response from the native host.
 */
function handleNativeResponse(response) {
    if (chrome.runtime.lastError) {
        console.error('BigMan AntiVirus: Native messaging error:', chrome.runtime.lastError.message);
        return;
    }

    console.log('BigMan AntiVirus: Received response from scanner:', response);

    // If the Rust program found something suspicious, notify the user
    if (response && response.status === 'SUSPICIOUS') {
        chrome.notifications.create({
            type: 'basic',
            iconUrl: 'icons/128.png', // Fixed icon path to match manifest
            title: 'Suspicious PDF Detected!',
            message: `A downloaded PDF was flagged. Reason: ${response.reason}`
        });
    }
}

/**
 * Main listener function that triggers when a download's state changes.
 * @param {chrome.downloads.DownloadDelta} downloadDelta - Object describing the change.
 */
async function onDownloadChanged(downloadDelta) {
    // We only care about downloads that have just completed
    if (downloadDelta.state && downloadDelta.state.current === 'complete') {
        try {
            // Since we get a delta, we need to fetch the full download item
            const downloadItems = await chrome.downloads.search({ id: downloadDelta.id });
            const downloadItem = downloadItems[0];
            
            // Check if the downloaded file is a PDF
            if (downloadItem && downloadItem.mime === 'application/pdf') {
                // Use the full file path from the filename property
                const fullPath = downloadItem.filename;
                console.log('BigMan AntiVirus: PDF download complete. Scanning file:', fullPath);

                // Send the file path to your Rust program for scanning
                // 'com.bigman.pdf_scanner' must match the name in your native host manifest file
                chrome.runtime.sendNativeMessage(
                    'com.bigman.pdf_scanner',
                    { filePath: fullPath },
                    handleNativeResponse
                );
            }
        } catch (error) {
            console.error('BigMan AntiVirus: Error fetching download info:', error);
        }
    }
}

// Attach the listener to the downloads API
chrome.downloads.onChanged.addListener(onDownloadChanged);

console.log('BigMan AntiVirus: PDF download listener is active.');