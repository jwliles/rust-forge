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
   cd /tmp/dotforge-deprecation
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
- Deprecation crate created at `/tmp/dotforge-deprecation/`

## Current State
- **Current branch:** `feature/pack-and-go` (needs to switch to `main` first)
- **Files modified:** Cargo.toml, CHANGELOG.md, README.md, docs/, .github/workflows/release.yml

## Notes
- The release workflow is fully automated once tagged
- It will build binaries, create GitHub release, and publish to crates.io
- Users will install with `cargo install forge-rs` but run `forge`
- Old `dotforge` crate will show deprecation message directing to `forge-rs`