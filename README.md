# Dev Storage Cleaner

A fast desktop app to find and clean up old `node_modules` folders, freeing up gigabytes of disk space.

![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey)
![License](https://img.shields.io/badge/license-MIT-blue)

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

### macOS

1. Download `DevStorageCleaner.app`
2. Open the app (Right-click → Open if you see a security warning)
3. Done!

Optionally, move to Applications folder for easy access.

### Windows

1. Build on Windows (see [Building on Windows](#building-on-windows))
2. Run `dev-storage-cleaner.exe`
3. No installation needed - it's portable!

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
# Install dependencies
./scripts/setup-macos.sh

# Build the app
./scripts/build-macos-app.sh

# Run it
open DevStorageCleaner.app
```

### Windows

1. Install [Rust](https://rustup.rs/)
2. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) with C++ workload
3. Open "Developer Command Prompt for VS"
4. Run:

```batch
cd dev-storage-cleaner
cargo build --release
scripts\package-windows.bat
```

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

**macOS: "App can't be opened"**
- Right-click the app → Open → Open (first time only)

**Windows: "Windows protected your PC"**
- Click "More info" → Run anyway

**No projects found**
- Click "Change Directory" and select your projects folder
- Make sure the selected folder contains projects with `node_modules`

## License

MIT License - Feel free to use and modify.

---

**Made by AKS**