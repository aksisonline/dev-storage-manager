# Release Workflow Diagram (Manual Cargo Process)

## 🎯 Big Picture

```
┌────────────────────────────────────────────────────────────┐
│                    LOCAL DEVELOPMENT                       │
└────────────────────────────────────────────────────────────┘

1. Build quality gate
   └─> cargo fmt --all
   └─> cargo clippy -- -D warnings
   └─> cargo test
   └─> cargo build --release

2. Update docs
   └─> Edit CHANGELOG.md + README.md
   └─> git commit -m "Update docs for vX.Y.Z"

3. Bump version manually
   └─> Edit Cargo.toml (version = "X.Y.Z")
   └─> cargo check              # refreshes Cargo.lock
   └─> git add Cargo.toml Cargo.lock
   └─> git commit -m "Bump version to X.Y.Z"

4. Tag release
   └─> git tag -a vX.Y.Z -m "Release X.Y.Z"

5. Push
   └─> git push origin main
   └─> git push origin vX.Y.Z   # triggers GitHub Actions
```

```
┌────────────────────────────────────────────────────────────┐
│                    GITHUB ACTIONS                          │
└────────────────────────────────────────────────────────────┘

  ┌───────────────────────────────────────────────────────┐
  │  build-macos                                          │
  ├───────────────────────────────────────────────────────┤
  │  1. cargo build --release                              │
  │  2. Copy target/release/dev-storage-cleaner            │
  │  3. Add README + LICENSE into dist/                    │
  │  4. Zip → dev-storage-cleaner-macos-<ver>.zip          │
  │  5. Upload artifact                                    │
  └───────────────────────────────────────────────────────┘

  ┌───────────────────────────────────────────────────────┐
  │  build-windows                                        │
  ├───────────────────────────────────────────────────────┤
  │  1. cargo build --release                              │
  │  2. Copy target\release\dev-storage-cleaner.exe        │
  │  3. Add README + LICENSE into dist\                    │
  │  4. Zip → dev-storage-cleaner-windows-x86_64-<ver>.zip │
  │  5. Upload artifact                                    │
  └───────────────────────────────────────────────────────┘

  ┌───────────────────────────────────────────────────────┐
  │  create-release                                       │
  ├───────────────────────────────────────────────────────┤
  │  1. Download artifacts                                │
  │  2. Generate brief release_notes.md                   │
  │  3. Publish GitHub release vX.Y.Z                     │
  │  4. Attach both ZIP files                             │
  └───────────────────────────────────────────────────────┘
```

```
┌────────────────────────────────────────────────────────────┐
│                    USERS DOWNLOAD                          │
└────────────────────────────────────────────────────────────┘

🍎 macOS
  • Download dev-storage-cleaner-macos-<ver>.zip
  • unzip
  • ./dev-storage-cleaner

🪟 Windows
  • Download dev-storage-cleaner-windows-x86_64-<ver>.zip
  • Extract all
  • Run dev-storage-cleaner.exe
```

---

## 🔄 Manual Release Checklist

| Step | Command | Notes |
|------|---------|-------|
| Run quality gate | `cargo fmt`, `cargo clippy`, `cargo test`, `cargo build --release` | Ensures tag reflects healthy build |
| Update docs | `git commit -m "Update CHANGELOG for vX.Y.Z"` | Keep changelog + README aligned |
| Bump version | Edit `Cargo.toml` → `cargo check` → commit | `cargo check` rewrites `Cargo.lock` |
| Tag | `git tag -a vX.Y.Z -m "Release X.Y.Z"` | Annotated tag recommended |
| Push | `git push origin main && git push origin vX.Y.Z` | Tag push triggers workflow |
| Verify | Watch Actions → Download ZIPs → smoke test | Should finish in ~7–9 minutes |

---

## ⚠️ Common Mistakes & Fixes

| Mistake | Why it hurts | Fix |
|---------|--------------|-----|
| Forgetting `cargo check` after editing `Cargo.toml` | `Cargo.lock` still points to old version | Always run `cargo check` (or `cargo metadata`) before committing |
| Tagging before committing changelog/docs | Release notes disagree with contents | Commit docs **before** version bump commit |
| Wrong tag format (`1.2.3` instead of `v1.2.3`) | Workflow never triggers | Always prefix with `v` |
| Pushing tag but not main | Release uses outdated code | Push `main` first, then the tag |

---

## 🧭 Timeline Example (v0.4.2)

1. 09:00 – Finish feature, run `cargo fmt/clippy/test`
2. 09:10 – Update `CHANGELOG.md`, commit docs
3. 09:15 – Edit `Cargo.toml`, run `cargo check`, commit bump
4. 09:17 – `git tag -a v0.4.2 -m "Release 0.4.2"`
5. 09:18 – `git push origin main && git push origin v0.4.2`
6. 09:20 – Monitor Actions (macOS + Windows)
7. 09:27 – Release published, download ZIPs, quick smoke test
8. 09:30 – Share release link 🎉

---

## 🧰 Optional Future Enhancements

1. Add Linux job mirroring macOS packaging
2. Insert signing/notarization steps before zipping
3. Extend release notes generation with changelog diff
4. Integrate Slack/Discord notification after publish

Stay disciplined with this manual flow and every release remains transparent, reproducible, and entirely Cargo-driven. 🚀