# GitHub Actions Workflows

This directory contains automated workflows for building, testing, and releasing the Dev Storage Cleaner application.

## Workflows

### 1. CI Workflow (`ci.yml`)

**Triggers:**
- Push to `main` or `develop` branches
- Pull requests targeting `main` or `develop` branches

**Jobs:**
- **Test**: Runs tests on Ubuntu, macOS, and Windows
  - Executes `cargo test`
  - Runs `cargo clippy` with warnings as errors
- **Build**: Builds debug and release versions on all platforms
- **Format**: Checks code formatting with `rustfmt`

**Purpose:** Ensures code quality and cross-platform compatibility on every commit and PR.

### 2. Release Workflow (`release.yml`)

**Triggers:**
- **Automatic**: Push of tags matching `v*.*.*` pattern (e.g., `v0.1.0`, `v1.2.3`)
- **Manual**: Workflow dispatch with version input

**Jobs:**
- **Build macOS Job:** Creates DMG installers for:
  - macOS x86_64 (Intel) - Full .app bundle in DMG
  - macOS aarch64 (Apple Silicon) - Full .app bundle in DMG
- **Build Windows Job:** Creates portable package:
  - Windows x86_64 - ZIP with executable and docs
- **Create Release**: 
  - Downloads all build artifacts
  - Creates a GitHub release with auto-generated release notes
  - Attaches all platform binaries as release assets
- **Verify Release**: Confirms the release was created successfully

**Build Outputs:**
- `DevStorageCleaner-macos-x86_64.dmg` - macOS Intel installer (DMG)
- `DevStorageCleaner-macos-aarch64.dmg` - macOS Apple Silicon installer (DMG)
- `DevStorageCleaner-windows-x86_64.zip` - Windows portable package (ZIP)

## Creating a Release

### Method 1: Tag-based (Recommended)

1. Update version in `Cargo.toml`:
   ```toml
   [package]
   version = "0.2.0"
   ```

2. Commit the version change:
   ```bash
   git add Cargo.toml
   git commit -m "Bump version to 0.2.0"
   ```

3. Create and push a tag:
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

4. The workflow will automatically:
   - Build binaries for all platforms
   - Create a GitHub release
   - Upload all binaries as release assets

### Method 2: Manual Dispatch

1. Go to the Actions tab in GitHub
2. Select "Build and Release" workflow
3. Click "Run workflow"
4. Enter the version number (e.g., `0.2.0`)
5. Click "Run workflow"

## Version Numbering

This project follows [Semantic Versioning](https://semver.org/):
- **MAJOR.MINOR.PATCH** (e.g., `1.2.3`)
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

## Caching Strategy

All workflows use cargo caching to speed up builds:
- Cargo registry cache
- Cargo git dependencies cache
- Target directory cache (build artifacts)

This significantly reduces build times for subsequent runs.

## Required Permissions

The release workflow requires:
- `contents: write` - To create releases and upload assets

These are automatically provided via `GITHUB_TOKEN` in GitHub Actions.

## Troubleshooting

### Release Workflow Fails

**Issue**: Build fails on a specific platform
- Check the build logs for that specific matrix job
- Common issues:
  - Missing system dependencies (rare for Rust)
  - Platform-specific code issues
  - Dependency compilation failures
  - DMG creation issues (macOS): Check if scripts have execute permissions
  - ZIP creation issues (Windows): PowerShell errors

**Issue**: Release creation fails
- Ensure the tag follows the `v*.*.*` pattern
- Check that `GITHUB_TOKEN` has sufficient permissions
- Verify no release already exists for that tag

### CI Workflow Fails

**Issue**: Tests fail
- Run tests locally: `cargo test`
- Check for platform-specific test issues

**Issue**: Clippy fails
- Run locally: `cargo clippy -- -D warnings`
- Fix all warnings before pushing

**Issue**: Format check fails
- Run locally: `cargo fmt --all`
- Commit the formatting changes

## Local Testing

Before pushing, test your changes locally:

```bash
# Run tests
cargo test

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy -- -D warnings

# Build release
cargo build --release
```

## Adding New Platforms

To add support for additional platforms (e.g., Linux):

1. Edit `.github/workflows/release.yml`
2. Create a new `build-linux` job similar to `build-macos` or `build-windows`
3. Add appropriate packaging steps (e.g., AppImage, .deb, .rpm)
4. Update the `needs` array in `create-release` to include the new job

**Example for Linux:**
```yaml
build-linux:
  name: Build Linux x86_64
  runs-on: ubuntu-latest
  steps:
    - name: Build release binary
      run: cargo build --release --target x86_64-unknown-linux-gnu
    - name: Create tar.gz
      run: |
        mkdir -p dist
        cp target/x86_64-unknown-linux-gnu/release/dev-storage-cleaner dist/
        tar -czf DevStorageCleaner-linux-x86_64.tar.gz -C dist .
```

## Workflow Badges

Add these badges to your main README.md:

```markdown
[![CI](https://github.com/aksisonline/dev-storage-cleaner/workflows/CI/badge.svg)](https://github.com/aksisonline/dev-storage-cleaner/actions/workflows/ci.yml)
[![Release](https://github.com/aksisonline/dev-storage-cleaner/workflows/Build%20and%20Release/badge.svg)](https://github.com/aksisonline/dev-storage-cleaner/actions/workflows/release.yml)
```

Replace `aksisonline` with your GitHub username.

## Cost Considerations

- GitHub Actions provides **2,000 minutes/month** for free on public repositories
- Private repositories get **2,000 minutes/month** on the free plan
- Different OS multipliers:
  - Linux: 1x
  - macOS: 10x
  - Windows: 2x

Each release build uses approximately:
- macOS (2 DMG builds): ~15-20 minutes (150-200 billed minutes)
  - Includes: cargo build + .app bundling + DMG creation
- Windows (1 ZIP build): ~10-15 minutes (20-30 billed minutes)
  - Includes: cargo build + ZIP packaging
- Total per release: ~25-35 actual minutes (~170-230 billed minutes)

## Future Enhancements

Potential improvements to consider:

1. **Code Signing**: Add macOS code signing and Windows code signing
2. **Notarization**: Notarize macOS DMG builds with Apple
3. **Windows Installer**: Create `.msi` installer (currently using `.zip`)
4. **Linux Support**: Add Linux builds (AppImage, .deb, .rpm)
5. **Changelog Automation**: Auto-generate CHANGELOG.md from commits
6. **Pre-release Builds**: Add beta/alpha release support
7. **Performance Benchmarks**: Add benchmark tracking across releases
8. **Security Scanning**: Integrate dependency vulnerability scanning
9. **Custom DMG Background**: Add custom background image to DMG installers
10. **Auto-update**: Implement in-app update checking

## Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust CI/CD Best Practices](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)
- [action-gh-release Documentation](https://github.com/softprops/action-gh-release)
- [Semantic Versioning](https://semver.org/)