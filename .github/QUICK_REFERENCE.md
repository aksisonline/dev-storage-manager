# Quick Reference Card

## 🚀 Common Commands

### Create a Release

```bash
# Automatic (recommended)
# The script automatically commits and tags for you!
./scripts/bump-version.sh patch    # 0.1.0 → 0.1.1
./scripts/bump-version.sh minor    # 0.1.0 → 0.2.0
./scripts/bump-version.sh major    # 0.1.0 → 1.0.0

# The script already committed and tagged, just push:
git push origin main
git push origin v0.x.x

# Windows
.\scripts\bump-version.ps1 patch
git push origin main
git push origin v0.x.x
```

### Manual Release (Without Script)

```bash
# 1. Update Cargo.toml version
# Edit Cargo.toml: version = "0.2.0"

# 2. Update Cargo.lock
cargo check

# 3. Commit and tag
git add Cargo.toml Cargo.lock
git commit -m "Bump version to 0.2.0"
git tag v0.2.0

# 4. Push
git push origin main
git push origin v0.2.0
```

### Before Committing

```bash
# Format code
cargo fmt --all

# Check lints
cargo clippy -- -D warnings

# Run tests
cargo test

# Build release
cargo build --release
```

## 📋 Workflow Triggers

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| CI | Push to `main`/`develop`, PRs | Testing |
| Release | Push tag `v*.*.*` | Build & Release |
| Release | Manual dispatch | Build & Release |

## 🏷️ Version Tags

```bash
# View tags
git tag -l

# Delete local tag
git tag -d v0.1.0

# Delete remote tag
git push origin :refs/tags/v0.1.0

# Push tag
git push origin v0.1.0

# Push all tags
git push origin --tags
```

## 🔍 Monitor Builds

```bash
# Using GitHub CLI
gh run list                    # List recent runs
gh run view                    # View latest run
gh run watch                   # Watch current run
gh run view --log             # View logs

# Browser
open "https://github.com/aksisonline/dev-storage-cleaner/actions"
```

## 📦 Release Management

```bash
# Using GitHub CLI
gh release list                          # List releases
gh release view v0.1.0                   # View release
gh release download v0.1.0               # Download assets
gh release delete v0.1.0                 # Delete release
gh release create v0.1.0 --generate-notes # Create release
```

## 🐛 Troubleshooting

### CI Fails

```bash
# Run locally
cargo test
cargo clippy -- -D warnings
cargo fmt --all -- --check
cargo build --release
```

### Wrong Tag Pushed

```bash
# Delete tag locally
git tag -d v0.1.0

# Delete tag remotely
git push origin :refs/tags/v0.1.0

# Create correct tag
git tag v0.2.0
git push origin v0.2.0
```

### Release Not Created

```bash
# Check permissions
# Settings → Actions → General → Workflow permissions
# ✅ "Read and write permissions"

# Re-run workflow
gh run rerun <run-id>
```

## 📁 Important Files

```
.github/workflows/ci.yml          # CI workflow
.github/workflows/release.yml     # Release workflow
.github/RELEASE_GUIDE.md          # Detailed guide
.github/SETUP.md                  # Setup documentation
scripts/bump-version.sh           # Version bump (Unix)
scripts/bump-version.ps1          # Version bump (Windows)
CHANGELOG.md                      # Version history
```

## 🎯 Release Checklist

Before running bump script:
- [ ] All tests pass locally (`cargo test`)
- [ ] CHANGELOG.md updated
- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] All changes committed
- [ ] On `main` branch
- [ ] Pulled latest changes

After running bump script:
- [ ] Review the commit: `git show HEAD`
- [ ] Push to trigger release

## 📊 Build Times

| OS | Build Time | Billed Minutes |
|----|------------|----------------|
| macOS (2 DMG builds) | ~8-10 min | 80-100 min |
| Windows (1 ZIP) | ~3-4 min | 6-8 min |
| **Total per release** | **~11-14 min** | **~86-108 min** |

**Free tier:** 2,000 minutes/month → ~18-23 releases/month

## 🔗 Quick Links

### Release Files

- macOS Intel: `DevStorageCleaner-macos-x86_64.dmg`
- macOS Apple Silicon: `DevStorageCleaner-macos-aarch64.dmg`
- Windows: `DevStorageCleaner-windows-x86_64.zip`

### Repository Links

- [Actions](https://github.com/aksisonline/dev-storage-cleaner/actions)
- [Releases](https://github.com/aksisonline/dev-storage-cleaner/releases)
- [Settings](https://github.com/aksisonline/dev-storage-cleaner/settings)
- [Workflows](.github/workflows/)

## 💡 Tips

1. **The bump script commits and tags automatically** - no need to commit first
2. **Always test locally before running bump script**
3. **Update CHANGELOG.md before bumping version**
4. **Use semantic versioning** (patch/minor/major)
5. **Check Actions tab after pushing tags**
6. **Review the commit before pushing** (`git show HEAD`)

## 🆘 Emergency Commands

```bash
# Cancel all running workflows
gh run list --status in_progress | awk '{print $7}' | xargs -n1 gh run cancel

# Rollback release
gh release delete v0.2.0
git push origin :refs/tags/v0.2.0
git tag -d v0.2.0
git revert HEAD
git push origin main

# Clear all caches (if builds fail mysteriously)
# Go to: Settings → Actions → Caches → Delete all
```

## 📞 Help

- Detailed docs: [.github/RELEASE_GUIDE.md](.github/RELEASE_GUIDE.md)
- Workflow docs: [.github/workflows/README.md](.github/workflows/README.md)
- Setup guide: [.github/SETUP.md](.github/SETUP.md)
- GitHub Actions: https://docs.github.com/en/actions

---

**Remember:** Replace `aksisonline` with your GitHub username in all files!