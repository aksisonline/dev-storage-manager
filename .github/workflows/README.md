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
- **Build**: Creates release binaries for:
  - macOS x86_64 (Intel)
  - macOS aarch64 (Apple Silicon)
  - Windows x86_64
- **Create Release**: 
  - Downloads all build artifacts
  - Creates a GitHub release with auto-generated release notes
  - Attaches all platform binaries as release assets
- **Verify Release**: Confirms the release was created successfully

**Build Outputs:**
- `dev-storage-cleaner-macos-x86_64` - macOS Intel binary
- `dev-storage-cleaner-macos-aarch64` - macOS Apple Silicon binary
- `dev-storage-cleaner-windows-x86_64.exe` - Windows binary

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
2. Add a new matrix entry in the `build` job:
   ```yaml
   - os: ubuntu-latest
     target: x86_64-unknown-linux-gnu
     artifact_name: dev-storage-cleaner
     asset_name: dev-storage-cleaner-linux-x86_64
   ```

3. If needed, add platform-specific build steps

## Workflow Badges

Add these badges to your main README.md:

```markdown
[![CI](https://github.com/YOUR_USERNAME/dev-storage-cleaner/workflows/CI/badge.svg)](https://github.com/YOUR_USERNAME/dev-storage-cleaner/actions/workflows/ci.yml)
[![Release](https://github.com/YOUR_USERNAME/dev-storage-cleaner/workflows/Build%20and%20Release/badge.svg)](https://github.com/YOUR_USERNAME/dev-storage-cleaner/actions/workflows/release.yml)
```

Replace `YOUR_USERNAME` with your GitHub username.

## Cost Considerations

- GitHub Actions provides **2,000 minutes/month** for free on public repositories
- Private repositories get **2,000 minutes/month** on the free plan
- Different OS multipliers:
  - Linux: 1x
  - macOS: 10x
  - Windows: 2x

Each release build uses approximately:
- macOS (2 builds): ~20-30 minutes (200-300 billed minutes)
- Windows (1 build): ~10-15 minutes (20-30 billed minutes)
- Total per release: ~30-45 actual minutes (~230-330 billed minutes)

## Future Enhancements

Potential improvements to consider:

1. **Code Signing**: Add macOS code signing and Windows code signing
2. **Notarization**: Notarize macOS builds with Apple
3. **Installers**: Create `.dmg` for macOS and `.msi` for Windows
4. **Linux Support**: Add Linux builds (AppImage, .deb, .rpm)
5. **Changelog Automation**: Auto-generate CHANGELOG.md from commits
6. **Pre-release Builds**: Add beta/alpha release support
7. **Performance Benchmarks**: Add benchmark tracking across releases
8. **Security Scanning**: Integrate dependency vulnerability scanning

## Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust CI/CD Best Practices](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)
- [action-gh-release Documentation](https://github.com/softprops/action-gh-release)
- [Semantic Versioning](https://semver.org/)