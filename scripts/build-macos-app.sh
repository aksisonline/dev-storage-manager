#!/bin/bash

# Build macOS .app bundle for Dev Storage Cleaner

set -e

APP_NAME="Dev Storage Cleaner"
BUNDLE_NAME="DevStorageCleaner.app"
VERSION="0.1.0"
IDENTIFIER="com.devtools.storage-cleaner"

echo "🍎 Building macOS Application Bundle"
echo "===================================="
echo ""

# Build the release binary first
echo "📦 Building release binary..."
~/.cargo/bin/cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Binary built successfully"
echo ""

# Create app bundle structure
echo "📁 Creating app bundle structure..."
rm -rf "$BUNDLE_NAME"
mkdir -p "$BUNDLE_NAME/Contents/MacOS"
mkdir -p "$BUNDLE_NAME/Contents/Resources"

# Copy binary
echo "📋 Copying binary..."
cp target/release/dev-storage-cleaner "$BUNDLE_NAME/Contents/MacOS/dev-storage-cleaner"
chmod +x "$BUNDLE_NAME/Contents/MacOS/dev-storage-cleaner"

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
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2024</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.developer-tools</string>
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
echo "To run the app:"
echo "  open \"$BUNDLE_NAME\""
echo ""
echo "To install to Applications folder:"
echo "  cp -r \"$BUNDLE_NAME\" /Applications/"
echo ""

# Ask if user wants to open the app
read -p "Would you like to open the app now? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    open "$BUNDLE_NAME"
fi
