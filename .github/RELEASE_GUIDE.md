# Release Guide (Manual Cargo Workflow)

This guide documents the exact steps required to cut a release of **Dev Storage Cleaner** using plain Cargo commands and the simplified GitHub Actions pipeline. No helper scripts are involved.

---

## 1. Prerequisites

- Rust toolchain installed via [rustup](https://rustup.rs/)
- GitHub repository access with permissions to push to `main` and create tags
- Updated local `main` branch (`git checkout main && git pull`)
- All feature work merged and tested

---

## 2. Prepare the Release

1. **Run the local quality gate**

   ```bash
   cargo fmt --all
   cargo clippy -- -D warnings
   cargo test
   cargo build --release
   ```

2. **Update documentation**

   - Refresh `CHANGELOG.md` with the new version entry.
   - Verify `README.md` still reflects the macOS ZIP + Windows `.exe` distribution model.

3. **Commit release notes**

   ```bash
   git add CHANGELOG.md README.md
   git commit -m "Update docs for vX.Y.Z"
   ```

---

## 3. Bump the Version Manually

1. Edit `Cargo.toml` and set `version = "X.Y.Z"`.
2. Regenerate `Cargo.lock` to capture the new version:

   ```bash
   cargo check
   ```

3. Stage and commit the version bump:

   ```bash
   git add Cargo.toml Cargo.lock
   git commit -m "Bump version to X.Y.Z"
   ```

4. Create an annotated tag:

   ```bash
   git tag -a vX.Y.Z -m "Release X.Y.Z"
   ```

---

## 4. Push & Trigger GitHub Actions

```bash
git push origin main
git push origin vX.Y.Z
```

Pushing the tag starts the `Build and Release` workflow, which will:

- `cargo build --release` on macOS (universal host build) and package `dev-storage-cleaner-macos-X.Y.Z.zip`
- `cargo build --release` on Windows and package `dev-storage-cleaner-windows-x86_64-X.Y.Z.zip`
- Create a GitHub Release that attaches both ZIP files and generates concise notes

**Estimated time:** 7–9 minutes total (macOS + Windows jobs).

---

## 5. Monitor & Verify

1. Open the **Actions** tab ➜ watch the latest `Build and Release` run.
2. Ensure both `build-macos` and `build-windows` jobs succeed.
3. Confirm the `Create Release` job publishes the release with two assets.
4. Download each artifact and spot-check:
   - macOS: `unzip dev-storage-cleaner-macos-X.Y.Z.zip && ./dev-storage-cleaner`
   - Windows: `Expand-Archive` or `tar -xf` and run the `.exe`

---

## 6. Manual Dispatch (Optional)

If you need to rebuild an existing commit without creating a new tag:

1. Go to **Actions → Build and Release → Run workflow**.
2. Provide the desired semantic version (e.g., `0.4.1`).
3. The workflow will use that input for artifact names and the release tag.

Use this only for dry-runs or emergency rebuilds; the canonical path is still “tag ➜ push”.

---

## 7. Troubleshooting

| Symptom | Likely Cause | Fix |
|---------|--------------|-----|
| macOS job fails compiling | stale target cache or missing dependency | rerun job; if persistent, `cargo clean` locally, push fix |
| Windows job missing `.exe` | build failure or antivirus interference | inspect logs, address Rust errors, rerun |
| Release missing assets | `Create Release` couldn’t find ZIPs | verify artifacts existed, rerun entire workflow |
| Wrong version in release | tag mismatch | delete release (`gh release delete vX.Y.Z`), delete tag, re-tag, push again |
| Workflow blocked | insufficient permissions | in repo settings, ensure Actions token has “Read and write” permissions |

---

## 8. Rollback Procedure

```bash
gh release delete vX.Y.Z
git push origin :refs/tags/vX.Y.Z
git tag -d vX.Y.Z
git revert <commit-hash-of-version-bump>
git push origin main
```

Re-run the release process with the corrected version.

---

## 9. Final Checklist

### Before Tagging
- [ ] `cargo fmt`, `cargo clippy`, `cargo test`, `cargo build --release`
- [ ] `CHANGELOG.md` + `README.md` updated and committed
- [ ] `Cargo.toml` & `Cargo.lock` bumped and committed
- [ ] On latest `main`

### After Tagging
- [ ] `git push origin main`
- [ ] `git push origin vX.Y.Z`
- [ ] Actions workflow succeeded on macOS + Windows
- [ ] Release assets download and run successfully
- [ ] Release notes reviewed (edit if needed)
- [ ] Users or channels notified (optional)

---

## 10. Reference Commands

```bash
# View releases
gh release list

# View a specific release
gh release view vX.Y.Z

# Download release assets
gh release download vX.Y.Z

# Rerun failed workflow
gh run rerun <run-id>
```

Stay disciplined with semantic versioning (`MAJOR.MINOR.PATCH`), keep documentation current, and the fully manual cargo-based release flow will remain fast, predictable, and transparent.