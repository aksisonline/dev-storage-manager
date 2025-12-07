#!/bin/bash

# Dev Storage Cleaner - macOS Setup Script
# This script helps set up the required dependencies for building the app on macOS

set -e

echo "🔧 Dev Storage Cleaner - macOS Setup"
echo "===================================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed."
    echo "📥 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✅ Rust installed successfully!"
else
    echo "✅ Rust is already installed ($(rustc --version))"
fi

# Check if Xcode Command Line Tools are installed
if ! xcode-select -p &> /dev/null; then
    echo "❌ Xcode Command Line Tools not found."
    echo "📥 Installing Xcode Command Line Tools..."
    xcode-select --install
    echo "⏳ Please complete the installation in the dialog and run this script again."
    exit 1
else
    echo "✅ Xcode Command Line Tools are installed"
fi

# Check if Metal toolchain is available
if ! xcrun -f metal &> /dev/null; then
    echo "⚠️  Metal toolchain not found."
    echo ""
    echo "The Metal toolchain is required to build this app."
    echo "Please choose an option:"
    echo ""
    echo "1. Install full Xcode (Recommended - includes everything)"
    echo "   - Open Mac App Store and search for 'Xcode'"
    echo "   - Or visit: https://apps.apple.com/us/app/xcode/id497799835"
    echo ""
    echo "2. Download Metal Toolchain only"
    echo "   - Run: xcodebuild -downloadComponent MetalToolchain"
    echo "   - Or run: xcodebuild -runFirstLaunch"
    echo ""
    read -p "Would you like to try downloading the Metal toolchain now? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "📥 Attempting to download Metal toolchain..."
        xcodebuild -runFirstLaunch || {
            echo "⚠️  Automatic download failed. Please install Xcode from the App Store."
            echo "   Visit: https://apps.apple.com/us/app/xcode/id497799835"
            exit 1
        }
    else
        echo "⏳ Please install Xcode or the Metal toolchain manually and run this script again."
        exit 1
    fi
else
    echo "✅ Metal toolchain is available"
fi

echo ""
echo "🎉 All dependencies are installed!"
echo ""
echo "Next steps:"
echo "1. Build the project: cargo build --release"
echo "2. Run the app: cargo run --release"
echo ""
echo "Or simply run: ./build.sh"
