# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub Actions CI/CD workflows for automated building and releases
- Automated release creation for macOS (Intel + Apple Silicon) and Windows
- Version bump scripts (`bump-version.sh` and `bump-version.ps1`)
- CI workflow for automated testing across platforms

### Changed

### Fixed

### Removed

## [0.1.0] - 2024-01-XX

### Added
- Initial release
- Directory selection and persistence
- Recursive scanning for `node_modules` folders
- Age-based filtering (30+ days threshold)
- Size calculation and display
- Selective deletion with visual feedback
- Modern GPU-accelerated UI using GPUI
- Cross-platform support (macOS and Windows)
- Cmd+Q quit support on macOS
- Smart threshold snapping to actual project ages
- Configuration persistence

### Features
- 📁 Directory selection with native file picker
- 🔍 Recursive project scanning
- ⏰ Age detection for projects
- 💾 Size display in human-readable format
- ✅ Multi-select deletion
- 🎨 Modern, responsive UI
- 💾 Settings persistence between sessions

---

## Release Notes Format

When creating a new release, add a new section with:

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security improvements
```

Replace `X.Y.Z` with the version number and `YYYY-MM-DD` with the release date.

[Unreleased]: https://github.com/YOUR_USERNAME/dev-storage-cleaner/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/YOUR_USERNAME/dev-storage-cleaner/releases/tag/v0.1.0