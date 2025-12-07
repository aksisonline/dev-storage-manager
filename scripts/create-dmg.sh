#!/bin/bash

# Create a DMG installer from a macOS .app bundle
# Usage: ./create-dmg.sh <app-bundle> <output-dmg-name> [version]

set -e

APP_BUNDLE="$1"
OUTPUT_NAME="${2:-DevStorageCleaner}"
VERSION="${3:-0.1.0}"

if [ -z "$APP_BUNDLE" ]; then
    echo "Usage: $0 <app-bundle> <output-dmg-name> [version]"
    echo "Example: $0 DevStorageCleaner.app DevStorageCleaner 0.1.0"
    exit 1
fi

if [ ! -d "$APP_BUNDLE" ]; then
    echo "❌ App bundle not found: $APP_BUNDLE"
    exit 1
fi

DMG_NAME="${OUTPUT_NAME}-${VERSION}.dmg"
VOLUME_NAME="Dev Storage Cleaner ${VERSION}"
STAGING_DIR="dmg-staging"

echo "💿 Creating DMG Installer"
echo "========================="
echo "App Bundle: $APP_BUNDLE"
echo "Output DMG: $DMG_NAME"
echo "Volume Name: $VOLUME_NAME"
echo ""

# Clean up any existing staging directory
echo "🧹 Cleaning up..."
rm -rf "$STAGING_DIR"
rm -f "$DMG_NAME"

# Create staging directory
echo "📁 Creating staging directory..."
mkdir -p "$STAGING_DIR"

# Copy app bundle to staging
echo "📋 Copying app bundle..."
cp -R "$APP_BUNDLE" "$STAGING_DIR/"

# Create Applications symlink
echo "🔗 Creating Applications symlink..."
ln -s /Applications "$STAGING_DIR/Applications"

# Create temporary DMG
echo "💿 Creating temporary DMG..."
TEMP_DMG="${OUTPUT_NAME}-temp.dmg"
hdiutil create -srcfolder "$STAGING_DIR" \
    -volname "$VOLUME_NAME" \
    -fs HFS+ \
    -fsargs "-c c=64,a=16,e=16" \
    -format UDRW \
    -size 200m \
    "$TEMP_DMG"

# Mount the temporary DMG
echo "📂 Mounting DMG..."
MOUNT_DIR=$(hdiutil attach -readwrite -noverify -noautoopen "$TEMP_DMG" | grep -E '/Volumes/' | sed 's/^.*\/Volumes/\/Volumes/')

if [ -z "$MOUNT_DIR" ]; then
    echo "❌ Failed to mount DMG"
    rm -f "$TEMP_DMG"
    exit 1
fi

echo "Mounted at: $MOUNT_DIR"

# Set custom DMG window properties (optional)
echo "🎨 Setting DMG properties..."
# Wait a moment for the volume to be fully mounted
sleep 2

# Use AppleScript to set window properties
osascript <<EOF || true
tell application "Finder"
    tell disk "$VOLUME_NAME"
        open
        set current view of container window to icon view
        set toolbar visible of container window to false
        set statusbar visible of container window to false
        set the bounds of container window to {100, 100, 600, 400}
        set viewOptions to the icon view options of container window
        set arrangement of viewOptions to not arranged
        set icon size of viewOptions to 100
        set position of item "$(basename "$APP_BUNDLE")" of container window to {150, 150}
        set position of item "Applications" of container window to {350, 150}
        close
        open
        update without registering applications
        delay 2
    end tell
end tell
EOF

# Sync to ensure all changes are written
sync

# Unmount the DMG
echo "⏏️  Unmounting DMG..."
hdiutil detach "$MOUNT_DIR" -force || true
sleep 1

# Convert to compressed read-only DMG
echo "🗜️  Compressing DMG..."
hdiutil convert "$TEMP_DMG" \
    -format UDZO \
    -imagekey zlib-level=9 \
    -o "$DMG_NAME"

# Clean up
echo "🧹 Cleaning up temporary files..."
rm -f "$TEMP_DMG"
rm -rf "$STAGING_DIR"

# Get final size
FINAL_SIZE=$(du -sh "$DMG_NAME" | cut -f1)

echo ""
echo "✅ DMG created successfully!"
echo ""
echo "📍 Location: $PWD/$DMG_NAME"
echo "📦 Size: $FINAL_SIZE"
echo ""
echo "To test the DMG:"
echo "  open \"$DMG_NAME\""
echo ""
