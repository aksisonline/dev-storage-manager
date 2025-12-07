#!/bin/bash

# Create macOS .app bundle from a built binary
# Usage: ./create-macos-app.sh <binary-path> <output-name> [version]

set -e

BINARY_PATH="$1"
OUTPUT_NAME="${2:-DevStorageCleaner}"
VERSION="${3:-0.1.0}"

if [ -z "$BINARY_PATH" ]; then
    echo "Usage: $0 <binary-path> <output-name> [version]"
    echo "Example: $0 target/release/dev-storage-cleaner DevStorageCleaner 0.1.0"
    exit 1
fi

if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ Binary not found: $BINARY_PATH"
    exit 1
fi

APP_NAME="Dev Storage Cleaner"
BUNDLE_NAME="${OUTPUT_NAME}.app"
IDENTIFIER="com.devtools.storage-cleaner"

echo "🍎 Creating macOS Application Bundle"
echo "===================================="
echo "Binary: $BINARY_PATH"
echo "Output: $BUNDLE_NAME"
echo "Version: $VERSION"
echo ""

# Create app bundle structure
echo "📁 Creating app bundle structure..."
rm -rf "$BUNDLE_NAME"
mkdir -p "$BUNDLE_NAME/Contents/MacOS"
mkdir -p "$BUNDLE_NAME/Contents/Resources"

# Copy binary
echo "📋 Copying binary..."
cp "$BINARY_PATH" "$BUNDLE_NAME/Contents/MacOS/dev-storage-cleaner"
chmod +x "$BUNDLE_NAME/Contents/MacOS/dev-storage-cleaner"

# Strip binary to reduce size
echo "🔪 Stripping binary..."
strip "$BUNDLE_NAME/Contents/MacOS/dev-storage-cleaner" || true

# Create Info.plist
echo "📝 Creating Info.plist..."
cat > "$BUNDLE_NAME/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>dev-storage-cleaner</string>
    <key>CFBundleIdentifier</key>
    <string>$IDENTIFIER</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundleDisplayName</key>
    <string>$APP_NAME</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2024</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.developer-tools</string>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
</dict>
</plist>
EOF

# Create PkgInfo
echo "📝 Creating PkgInfo..."
echo -n "APPL????" > "$BUNDLE_NAME/Contents/PkgInfo"

echo ""
echo "✅ macOS Application Bundle created successfully!"
echo ""
echo "📍 Location: $PWD/$BUNDLE_NAME"
echo "📦 Size: $(du -sh "$BUNDLE_NAME" | cut -f1)"
echo ""
