#!/bin/bash

echo "BigMan Antivirus - Native Messaging Host Installer"
echo "=================================================="

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY_PATH="$PROJECT_DIR/target/release/bigman"

echo "Project directory: $PROJECT_DIR"

# Build binary if not exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "Building release binary..."
    cargo build --release
    if [ ! -f "$BINARY_PATH" ]; then
        echo "ERROR: Failed to build binary"
        exit 1
    fi
fi

# Make binary executable
chmod +x "$BINARY_PATH"
echo "Binary is executable: $BINARY_PATH"

# Create native messaging directories
CHROME_DIR="$HOME/.config/google-chrome/NativeMessagingHosts"
CHROMIUM_DIR="$HOME/.config/chromium/NativeMessagingHosts"

mkdir -p "$CHROME_DIR"
echo "Created directory: $CHROME_DIR"

if [ -d "$HOME/.config/chromium" ]; then
    mkdir -p "$CHROMIUM_DIR"
    echo "Created directory: $CHROMIUM_DIR"
fi

# Check for extension ID if provided as argument
EXTENSION_ID="${1:-}"

if [ -n "$EXTENSION_ID" ]; then
    ALLOWED_ORIGINS="[\"chrome-extension://$EXTENSION_ID/\"]"
    echo "Using extension ID: $EXTENSION_ID"
else
    ALLOWED_ORIGINS="[\"chrome-extension://*/\"]"
    echo "WARNING: Using wildcard for allowed_origins (may not work reliably)"
fi

# Create manifest
MANIFEST_FILE="$CHROME_DIR/com.bigman.pdf_scanner.json"
cat > "$MANIFEST_FILE" << EOF
{
  "name": "com.bigman.pdf_scanner",
  "description": "BigMan AntiVirus PDF Scanner",
  "path": "$BINARY_PATH",
  "type": "stdio",
  "allowed_origins": $ALLOWED_ORIGINS
}
EOF

echo "Installed: $MANIFEST_FILE"

# Copy to Chromium if it exists
if [ -d "$CHROMIUM_DIR" ]; then
    cp "$MANIFEST_FILE" "$CHROMIUM_DIR/com.bigman.pdf_scanner.json"
    echo "Installed: $CHROMIUM_DIR/com.bigman.pdf_scanner.json"
fi

# Verify and show next steps
if [ -f "$MANIFEST_FILE" ] && [ -x "$BINARY_PATH" ]; then
    echo ""
    echo "SUCCESS: Native messaging host installed"
    echo ""
    if [ -z "$EXTENSION_ID" ]; then
        echo "To complete setup:"
        echo "1. Load extension: chrome://extensions -> Developer mode -> Load unpacked"
        echo "2. Select folder: $PROJECT_DIR/browser-extension"
        echo "3. Copy the extension ID shown in Chrome"
        echo "4. Re-run: ./install-native-messaging.sh YOUR_EXTENSION_ID"
    else
        echo "Extension is configured for ID: $EXTENSION_ID"
        echo "Load the extension from: $PROJECT_DIR/browser-extension"
    fi
    echo ""
    echo "Test with: python3 test-native-messaging.py"
else
    echo "ERROR: Installation failed"
    exit 1
fi