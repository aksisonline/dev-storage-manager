# Dev Storage Cleaner

A fast desktop app to find and clean up old `node_modules` folders, freeing up gigabytes of disk space.

![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey)
![License](https://img.shields.io/badge/license-MIT-blue)
[![CI](https://github.com/aksisonline/dev-storage-cleaner/workflows/CI/badge.svg)](https://github.com/aksisonline/dev-storage-cleaner/actions/workflows/ci.yml)
[![Release](https://github.com/aksisonline/dev-storage-cleaner/workflows/Build%20and%20Release/badge.svg)](https://github.com/aksisonline/dev-storage-cleaner/actions/workflows/release.yml)

## What It Does

Scans your projects directory and identifies `node_modules` folders that haven't been modified in 30+ days. Select the ones you want to remove and reclaim disk space with one click.

- **Choose your directory** - Pick where to scan (remembers your choice)
- **Smart scanning** - Recursively finds all old node_modules
- **Safe deletion** - Only removes what you select

**Typical results:** 5-15 GB freed from 20 old projects 🎉

## Features

- 📁 **Directory Selection** - Choose which folder to scan, remembers your choice
- 🔍 **Smart Scanning** - Finds all projects with `node_modules` folders
- ⏰ **Age Detection** - Shows how many days since last modification
- 💾 **Size Display** - See folder sizes in GB before deleting
- ✅ **Selective Deletion** - Choose exactly which projects to clean
- 🎨 **Modern UI** - Fast, GPU-accelerated interface
- 🔒 **Safe** - Only deletes `node_modules`, your code stays intact
- 💾 **Persistent Settings** - Remembers your directory between runs

## Download & Install

### Pre-built Binaries (Recommended)

Download the latest release from the [Releases page](https://github.com/aksisonline/dev-storage-cleaner/releases):

- **macOS**: `dev-storage-cleaner-macos-<version>.zip`
- **Windows**: `dev-storage-cleaner-windows-x86_64-<version>.zip`

### Installation Instructions

#### macOS

1. Download the macOS ZIP file
2. Extract the ZIP file
3. Run the `dev-storage-cleaner` executable
4. On first launch, right-click the app and select "Open" (security requirement)

#### Windows

1. Download the ZIP file
2. Extract to any folder
3. Run `dev-storage-cleaner.exe`
4. If Windows Defender blocks it, click "More info" → "Run anyway"

## How to Use

1. **Launch** the app
2. **Click "📁 Change Directory..."** - Choose your projects folder (e.g., ~/Projects)
3. **Click "Scan"** - Finds old projects (30+ days)
4. **Select projects** - Click on any project to select it (blue border appears)
5. **Click "Delete Selected"** - Removes `node_modules` folders
6. **Done!** - See how much space you freed

The app remembers your directory choice for next time!

### Tips

- **First time setup**: Click "Change Directory" and select your main projects folder
- Select projects by clicking anywhere on the card
- Check the total size before deleting (shown in green)
- Projects are sorted by size (largest first)
- The app remembers your directory, so next time just click "Scan"
- You can always restore with `npm install` if needed

## Building from Source

### macOS

```bash
# Install Rust if you don't already have it: https://rustup.rs/
cargo build --release

# Run the release binary
./target/release/dev-storage-cleaner
```

### Windows

1. Install [Rust](https://rustup.rs/) (MSVC toolchain)
2. Open "Developer Command Prompt for VS" or PowerShell
3. Build and run:

```powershell
cargo build --release
.\target\release\dev-storage-cleaner.exe
```

## Development

### Release Process

This project uses automated CI/CD with GitHub Actions:

1. **Continuous Integration**: Every push and PR is automatically tested on macOS, Windows, and Linux
2. **Automated Releases**: Releases are automatically built and published when you push a version tag

#### Creating a Release

Follow these manual steps:

1. Update version in `Cargo.toml`.
2. Refresh `Cargo.lock`: `cargo check`.
3. Commit the changes: `git add Cargo.toml Cargo.lock && git commit -m "Bump version to 0.2.0"`.
4. Tag the release: `git tag v0.2.0`.
5. Push main and the tag: `git push origin main && git push origin v0.2.0`.

The GitHub Actions workflow will automatically:
- Build a ZIP package for macOS
- Build a ZIP package for Windows x86_64
- Create a GitHub release with auto-generated release notes
- Upload both packages as release assets

See [`.github/workflows/README.md`](.github/workflows/README.md) for detailed documentation.

## Requirements

- **macOS:** 10.15 (Catalina) or later
- **Windows:** Windows 10 or Windows 11

## Safety

✅ **What gets deleted:** Only `node_modules` folders you select  
✅ **What's safe:** All your source code, configs, and other files  
✅ **Restoration:** Run `npm install` in the project to restore dependencies

## Technical Details

Built with:
- **Rust** - Fast, safe systems language
- **GPUI** - GPU-accelerated UI framework by Zed Industries
- Platform-native rendering (Metal on macOS, DirectX on Windows)

Project structure:
- `src/main.rs` - Application entry point
- `src/app.rs` - Main application state and logic
- `src/config.rs` - Configuration persistence
- `src/scanner.rs` - Directory scanning logic
- `src/ui/` - UI rendering components

## Troubleshooting

**macOS: "App can't be opened" or "from an unidentified developer"**
- Right-click the app → Open → Open (first time only)
- Or: System Preferences → Security & Privacy → Click "Open Anyway"

**Windows: "Windows protected your PC"**
- Click "More info" → Run anyway

**No projects found**
- Click "Change Directory" and select your projects folder
- Make sure the selected folder contains projects with `node_modules`

## License

MIT License - Feel free to use and modify.

---

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

The CI workflow will automatically test your changes on all platforms.

---

**Made by AKS**

> **Note:** Replace `aksisonline` in the badge URLs with your actual GitHub username.