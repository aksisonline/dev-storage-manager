# Release Workflow Diagram

## 🎯 Quick Visual Guide: How Releases Work

### 📊 The Full Process

```
┌─────────────────────────────────────────────────────────────────┐
│                     LOCAL DEVELOPMENT                            │
└─────────────────────────────────────────────────────────────────┘

  1. Make changes to your code
     └─> cargo test
     └─> cargo clippy
     └─> cargo fmt --all
     └─> git add . && git commit -m "Add feature X"

  2. Update CHANGELOG.md
     └─> Document your changes
     └─> git add CHANGELOG.md
     └─> git commit -m "Update CHANGELOG for v0.2.0"

  3. Run bump script
     └─> ./scripts/bump-version.sh patch
         │
         ├─> Updates Cargo.toml
         ├─> Updates Cargo.lock
         ├─> Creates commit: "Bump version to 0.2.0"
         └─> Creates tag: v0.2.0
         
     ⚠️  THE SCRIPT COMMITS & TAGS AUTOMATICALLY!
         You DON'T need to commit manually!

  4. Review the commit
     └─> git show HEAD
     └─> Check version numbers are correct

  5. Push to GitHub
     └─> git push origin main
     └─> git push origin v0.2.0

         ⬇️  Tag push triggers GitHub Actions

┌─────────────────────────────────────────────────────────────────┐
│                     GITHUB ACTIONS                               │
└─────────────────────────────────────────────────────────────────┘

  ┌──────────────────────────────────────────────────────────────┐
  │  BUILD MACOS (Parallel)                                       │
  ├──────────────────────────────────────────────────────────────┤
  │                                                               │
  │  ┌─────────────────────┐    ┌─────────────────────┐         │
  │  │  Intel (x86_64)     │    │  Apple Silicon      │         │
  │  │                     │    │  (aarch64)          │         │
  │  ├─────────────────────┤    ├─────────────────────┤         │
  │  │ 1. cargo build      │    │ 1. cargo build      │         │
  │  │ 2. create-macos-app │    │ 2. create-macos-app │         │
  │  │ 3. create-dmg       │    │ 3. create-dmg       │         │
  │  │ 4. Upload artifact  │    │ 4. Upload artifact  │         │
  │  └─────────────────────┘    └─────────────────────┘         │
  │         │                           │                        │
  │         └───────────┬───────────────┘                        │
  │                     ▼                                        │
  │   DevStorageCleaner-macos-x86_64.dmg                        │
  │   DevStorageCleaner-macos-aarch64.dmg                       │
  └──────────────────────────────────────────────────────────────┘

  ┌──────────────────────────────────────────────────────────────┐
  │  BUILD WINDOWS                                                │
  ├──────────────────────────────────────────────────────────────┤
  │                                                               │
  │  ┌─────────────────────┐                                     │
  │  │  x86_64             │                                     │
  │  │                     │                                     │
  │  ├─────────────────────┤                                     │
  │  │ 1. cargo build      │                                     │
  │  │ 2. Create dist/     │                                     │
  │  │ 3. Compress to ZIP  │                                     │
  │  │ 4. Upload artifact  │                                     │
  │  └─────────────────────┘                                     │
  │         │                                                    │
  │         ▼                                                    │
  │   DevStorageCleaner-windows-x86_64.zip                      │
  └──────────────────────────────────────────────────────────────┘

         │                    │
         └────────┬───────────┘
                  ▼
  ┌──────────────────────────────────────────────────────────────┐
  │  CREATE RELEASE                                               │
  ├──────────────────────────────────────────────────────────────┤
  │  1. Download all artifacts                                   │
  │  2. Generate release notes                                   │
  │  3. Create GitHub Release v0.2.0                            │
  │  4. Attach all DMG and ZIP files                            │
  │  5. Publish release                                         │
  └──────────────────────────────────────────────────────────────┘
                  │
                  ▼
  ┌──────────────────────────────────────────────────────────────┐
  │  VERIFY RELEASE                                               │
  ├──────────────────────────────────────────────────────────────┤
  │  ✅ Check release exists                                     │
  │  ✅ List all assets                                          │
  │  ✅ Display release URL                                      │
  └──────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                     USERS DOWNLOAD                               │
└─────────────────────────────────────────────────────────────────┘

  🍎 macOS Users:
     1. Download appropriate DMG
     2. Open DMG file
     3. Drag app to Applications folder
     4. Done!

  🪟 Windows Users:
     1. Download ZIP file
     2. Extract to any folder
     3. Run dev-storage-cleaner.exe
     4. Done!
```

---

## 🔄 What the Bump Script Does

```
YOU RUN:
  ./scripts/bump-version.sh patch

SCRIPT DOES AUTOMATICALLY:
  ┌──────────────────────────────────────────┐
  │ 1. ✅ Update Cargo.toml                  │
  │    version = "0.1.0" → "0.1.1"          │
  │                                          │
  │ 2. ✅ Update Cargo.lock                  │
  │    cargo check --quiet                   │
  │                                          │
  │ 3. ✅ Create Git Commit                  │
  │    git add Cargo.toml Cargo.lock        │
  │    git commit -m "Bump version to 0.1.1"│
  │                                          │
  │ 4. ✅ Create Git Tag                     │
  │    git tag -a v0.1.1 -m "Release 0.1.1" │
  └──────────────────────────────────────────┘

THEN YOU:
  ┌──────────────────────────────────────────┐
  │ 1. Review the commit (optional)          │
  │    git show HEAD                         │
  │                                          │
  │ 2. Push changes                          │
  │    git push origin main                  │
  │                                          │
  │ 3. Push tag (triggers release)           │
  │    git push origin v0.1.1                │
  └──────────────────────────────────────────┘
```

---

## ⚠️ Common Mistakes

### ❌ WRONG - Committing Before Bump Script

```bash
# DON'T DO THIS:
vim Cargo.toml                         # Edit manually
git add Cargo.toml
git commit -m "Bump version to 0.2.0"  # ❌ Manual commit
./scripts/bump-version.sh patch        # ❌ Script fails or creates duplicate commit
```

### ✅ CORRECT - Let Script Handle Version Commits

```bash
# DO THIS:
# 1. Commit your features first
git add src/
git commit -m "Add awesome feature"

# 2. Update and commit CHANGELOG
vim CHANGELOG.md
git add CHANGELOG.md
git commit -m "Update CHANGELOG for v0.2.0"

# 3. Let the script handle version changes
./scripts/bump-version.sh patch        # ✅ Script commits version automatically

# 4. Push everything
git push origin main
git push origin v0.2.0
```

---

## 📝 Timeline Example

**Real-world example of creating v0.2.0:**

```
10:00 AM - Code new feature
10:30 AM - Run tests: cargo test ✅
10:35 AM - Check lints: cargo clippy ✅
10:40 AM - Format code: cargo fmt --all ✅
10:45 AM - Commit feature: git commit -m "Add dark mode support"

10:50 AM - Update CHANGELOG.md
10:55 AM - Commit CHANGELOG: git commit -m "Update CHANGELOG for v0.2.0"

11:00 AM - Run bump script: ./scripts/bump-version.sh minor
           └─> Script updates Cargo files
           └─> Script creates commit "Bump version to 0.2.0"
           └─> Script creates tag v0.2.0
           └─> Script tells you what to do next

11:01 AM - Review commit: git show HEAD ✅
11:02 AM - Push main: git push origin main
11:03 AM - Push tag: git push origin v0.2.0 🚀

11:03 AM - GitHub Actions triggered
11:05 AM - macOS builds start (parallel)
11:08 AM - Windows build starts
11:12 AM - All builds complete
11:13 AM - Release created with all assets
11:14 AM - Release verified ✅

11:15 AM - Users can download! 🎉
```

**Total time from bump to release: ~12 minutes**

---

## 🎯 Key Takeaways

1. ✅ **The bump script commits and tags automatically**
2. ✅ **Commit your features BEFORE running the bump script**
3. ✅ **Update CHANGELOG.md BEFORE running the bump script**
4. ✅ **Don't manually edit version in Cargo.toml - use the script**
5. ✅ **Review the commit after bump: `git show HEAD`**
6. ✅ **Push tag to trigger release: `git push origin v0.x.x`**

---

## 🆘 Quick Troubleshooting

| Problem | Solution |
|---------|----------|
| Script says "uncommitted changes" | Commit your work first, then run script |
| Wrong version created | Delete tag, fix Cargo.toml, run script again |
| Push failed | Pull latest changes first: `git pull origin main` |
| Workflow didn't trigger | Check tag format: must be `v1.2.3` (with 'v') |
| Build failed | Check Actions tab for logs, fix issue, create new version |

---

**Remember:** The bump script is your friend! It automates the tedious parts so you can focus on building great features. 🚀