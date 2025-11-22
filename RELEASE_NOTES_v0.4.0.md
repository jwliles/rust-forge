---
date_created: 2025-10-03T02-46-38
date_updated: 2025-10-03T02-46-38
timestamp: 1759459598161
title: RELEASE_NOTES_v0.4.0
id: 94094b8d-131e-409f-b29e-57a4940a6d85
hash: 56500572d39e0ebc7adfd83163c083965bf8cae4e39bc9a5ba7b49ba96b3bc2b
---
# Release Notes - v0.4.0

## Remaining tasks for v0.4.0 release

### High Priority (Required for release)
1. **Merge branches in order:**
   ```bash
   git checkout main
   git merge feature/module-structure
   git merge docs
   git merge feature/pack-and-go
   ```

2. **Test and lint:**
   ```bash
   cargo test --all-features
   cargo clippy --all-features -- -D warnings
   cargo fmt --all -- --check
   ```

3. **Release process:**
   ```bash
   git tag v0.4.0
   git push origin main --tags
   ```
   - CI/CD will auto-publish to crates.io as `forge-rs`

### Medium Priority 
4. **Publish deprecation crate:**
   ```bash
   cd /tmp/forge-deprecation
   cargo publish --token $CRATES_TOKEN
   ```

5. **Branch cleanup:**
   ```bash
   git branch -d feature/module-structure
   git branch -d docs
   git branch -d feature/pack-and-go
   ```
   - Keep: `main`, `dev`

## Status Completed ✅
- Crate renamed to `forge-rs` 
- Version bumped to 0.4.0
- Changelog updated with breaking changes
- Release workflow fixed for correct binary name
- Deprecation crate created at `/tmp/forge-deprecation/`

## Current State
- **Current branch:** `feature/pack-and-go` (needs to switch to `main` first)
- **Files modified:** Cargo.toml, CHANGELOG.md, README.md, docs/, .github/workflows/release.yml

## Notes
- The release workflow is fully automated once tagged
- It will build binaries, create GitHub release, and publish to crates.io
- Users will install with `cargo install forge-rs` but run `forge`
- Old `forge` crate will show deprecation message directing to `forge-rs`
