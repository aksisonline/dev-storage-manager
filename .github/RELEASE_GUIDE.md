# Release Guide

Quick reference for creating releases of Dev Storage Cleaner.

## Quick Release

```bash
# 1. Bump version (automatically commits and tags for you!)
./scripts/bump-version.sh [patch|minor|major]

# 2. Push to GitHub (the script already committed and tagged)
git push origin main
git push origin v0.x.x

# 3. Wait for GitHub Actions to build and create the release
# Check: https://github.com/aksisonline/dev-storage-cleaner/actions
```

That's it! The bump script does the commit and tag automatically.

## What Happens Automatically

When you push a tag (e.g., `v0.2.0`), GitHub Actions will:

1. ✅ Build binaries for:
   - macOS Intel (x86_64)
   - macOS Apple Silicon (aarch64)
   - Windows (x86_64)

2. ✅ Create a GitHub Release with:
   - Auto-generated release notes
   - All three binaries attached
   - Version number from the tag

3. ✅ Verify the release was created successfully

**Build time:** ~5-10 minutes

## Step-by-Step Release Process

### 1. Prepare the Release

**Update CHANGELOG.md:**
```markdown
## [0.2.0] - 2024-01-15

### Added
- New feature X
- New feature Y

### Fixed
- Bug fix Z
```

**Commit the CHANGELOG:**
```bash
git add CHANGELOG.md
git commit -m "Update CHANGELOG for v0.2.0"
```

### 2. Bump Version

**Using the script (recommended):**

```bash
# macOS/Linux
./scripts/bump-version.sh patch  # 0.1.0 -> 0.1.1
./scripts/bump-version.sh minor  # 0.1.0 -> 0.2.0
./scripts/bump-version.sh major  # 0.1.0 -> 1.0.0
```

```powershell
# Windows
.\scripts\bump-version.ps1 patch
```

**The script automatically:**
- ✅ Updates `Cargo.toml` version
- ✅ Updates `Cargo.lock`
- ✅ Creates a git commit
- ✅ Creates a git tag

**You don't need to commit first** - the script does it for you!

**Manual version bump:**

```bash
# 1. Edit Cargo.toml
version = "0.2.0"

# 2. Update lock file
cargo check

# 3. Commit
git add Cargo.toml Cargo.lock
git commit -m "Bump version to 0.2.0"

# 4. Tag
git tag -a v0.2.0 -m "Release 0.2.0"
```

### 3. Push to GitHub

```bash
# Push commits
git push origin main

# Push tag (this triggers the release workflow)
git push origin v0.2.0
```

### 4. Monitor the Build

1. Go to: https://github.com/aksisonline/dev-storage-cleaner/actions
2. Watch the "Build and Release" workflow
3. Check for any failures

### 5. Edit Release Notes (Optional)

After the automated release is created:

1. Go to: https://github.com/aksisonline/dev-storage-cleaner/releases
2. Click "Edit" on the new release
3. Enhance the auto-generated notes with:
   - Highlights from CHANGELOG.md
   - Installation instructions
   - Breaking changes (if any)
   - Screenshots (if applicable)

**Use `.github/RELEASE_TEMPLATE.md` as a guide.**

## Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features (backward compatible)
- **PATCH** (0.0.1): Bug fixes (backward compatible)

### Examples

```
0.1.0 -> 0.1.1  (bug fix)
0.1.0 -> 0.2.0  (new feature)
0.1.0 -> 1.0.0  (breaking change)
1.2.3 -> 1.2.4  (bug fix)
1.2.3 -> 1.3.0  (new feature)
1.2.3 -> 2.0.0  (breaking change)
```

## Manual Release (Without Tag)

You can also trigger a release manually:

1. Go to: https://github.com/aksisonline/dev-storage-cleaner/actions
2. Select "Build and Release" workflow
3. Click "Run workflow"
4. Enter version number (e.g., `0.2.0`)
5. Click "Run workflow"

This is useful for:
- Testing the release process
- Creating releases without pushing tags
- Emergency releases

## Pre-release / Beta Releases

To create a pre-release:

1. Use a pre-release version number:
   ```bash
   git tag v0.2.0-beta.1
   git push origin v0.2.0-beta.1
   ```

2. After the release is created, edit it and check "This is a pre-release"

## Troubleshooting

### Build Fails

**Problem:** Build fails on a specific platform

**Solution:**
1. Check the GitHub Actions logs
2. Fix the issue locally
3. Create a new patch version and try again

### Release Not Created

**Problem:** Builds succeed but no release created

**Solution:**
1. Check workflow permissions: Settings → Actions → General → Workflow permissions
2. Ensure "Read and write permissions" is enabled
3. Re-run the workflow

### Wrong Version in Release

**Problem:** Release has the wrong version number

**Solution:**
1. Delete the incorrect release: `gh release delete v0.x.x`
2. Delete the tag: `git push origin :refs/tags/v0.x.x`
3. Create a new tag with correct version
4. Push again

### Missing Binaries

**Problem:** Some platform binaries are missing

**Solution:**
1. Re-run the failed job in GitHub Actions
2. If that doesn't work, manually trigger the workflow again

## Best Practices

1. ✅ **Test before releasing**
   ```bash
   cargo test
   cargo build --release
   # Test the binary manually
   ```

2. ✅ **Update CHANGELOG.md before running bump script**
   - Commit the CHANGELOG first
   - Then run the bump script

3. ✅ **The bump script commits automatically**
   - Don't manually commit version changes
   - Let the script handle Cargo.toml and Cargo.lock
   - Review the commit after: `git show HEAD`

4. ✅ **Use descriptive commit messages for features**
   ```bash
   git commit -m "Add feature X that does Y"
   ```

5. ✅ **Check the build status before pushing tags**

6. ✅ **Review auto-generated release notes and enhance them**

7. ✅ **Announce the release**
   - Update project README if needed
   - Share on social media
   - Notify users

## GitHub CLI Shortcuts

Install [GitHub CLI](https://cli.github.com/) for easier release management:

```bash
# View latest release
gh release view

# List all releases
gh release list

# Download release assets
gh release download v0.2.0

# Delete a release
gh release delete v0.2.0

# Create a release manually
gh release create v0.2.0 --generate-notes
```

## Rollback a Release

If you need to rollback:

```bash
# 1. Delete the release
gh release delete v0.2.0

# 2. Delete the tag remotely
git push origin :refs/tags/v0.2.0

# 3. Delete the tag locally
git tag -d v0.2.0

# 4. Revert the version bump commit
git revert HEAD
git push origin main
```

## Resources

- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [GitHub Releases Documentation](https://docs.github.com/en/repositories/releasing-projects-on-github)

## Checklist

Before running bump script:

- [ ] All tests pass locally (`cargo test`)
- [ ] CHANGELOG.md is updated and committed
- [ ] README.md is up to date
- [ ] All feature changes are committed
- [ ] You're on the main branch
- [ ] You've pulled the latest changes
- [ ] Version number follows semantic versioning

After running bump script:

- [ ] Review the commit: `git show HEAD`
- [ ] Check version and tag are correct
- [ ] Push changes and tag

After release completes:

- [ ] Build succeeded on all platforms
- [ ] All installers are attached to the release
- [ ] Release notes are clear and helpful
- [ ] Download and test installers
- [ ] Users are notified (if applicable)

---

**Need help?** Check the [workflows README](.github/workflows/README.md) for more details.