---
date_created: 2025-10-03T02-46-38
date_updated: 2025-10-03T02-46-38
timestamp: 1759459598161
title: CONTRIBUTING
id: a438cf88-e72d-4aef-ba9e-71f6fffab035
hash: fbfc6cdc61eb951126e852079d5fc8880abc38d8cb792eea90c7f30658d9edb6
---
# Contributing to Forge

Thank you for your interest in contributing to Forge! This document provides guidelines and workflows for contributing to the project.

## Git Workflow

Forge uses a modified GitFlow workflow:

### Main Branches

- `main`: The production branch. All releases are tagged from this branch.
- `docs`: Documentation-specific development that can be updated independently.

### Development Branches

- `feature/*`: Feature development branches (e.g., `feature/add-profile-system`)
- `bugfix/*`: Bug fix branches (e.g., `bugfix/fix-symlink-creation`)
- `release/*`: Release preparation branches (e.g., `release/0.2.0`)

## Workflow Steps

1. **Create a Branch**:
   ```bash
   # For new features
   git checkout -b feature/my-new-feature main
   
   # For bug fixes
   git checkout -b bugfix/issue-description main
   
   # For documentation changes
   git checkout -b docs/update-readme docs
   ```

2. **Make Changes**: Develop your feature, fix, or documentation update.

3. **Commit Changes**: Use clear, descriptive commit messages.
   ```bash
   git add .
   git commit -m "Add feature X" -m "This implements feature X which..."
   ```

4. **Merge to Main or Docs**:
   ```bash
   # First update your branch with latest changes
   git checkout main
   git pull
   git checkout feature/my-new-feature
   git rebase main
   
   # Then merge (no fast-forward to preserve history)
   git checkout main
   git merge --no-ff feature/my-new-feature
   ```

## Publishing to crates.io

Publishing to crates.io requires a version bump according to SemVer principles:

1. **Patch Version (0.1.0 → 0.1.1)**: 
   - Bug fixes
   - Performance improvements without API changes
   - Minor documentation updates in code

2. **Minor Version (0.1.0 → 0.2.0)**:
   - New features that don't break backward compatibility
   - Deprecation of existing functionality (but still supported)

3. **Major Version (0.x.x → 1.0.0)**:
   - Breaking changes or API redesigns
   - Removal of deprecated functionality

### Publication Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Commit changes: `git commit -m "Bump version to X.Y.Z"`
4. Tag the release: `git tag -a vX.Y.Z -m "Version X.Y.Z"`
5. Push changes: `git push && git push --tags`
6. Publish to crates.io: `cargo publish`

## Documentation-Only Changes

Documentation-only updates (README, docs/, etc.) don't require a version bump and don't need to be published to crates.io. These changes should be made on the `docs` branch and then merged to `main`.

## Code Style

- Follow Rust's official style guide
- Use `cargo fmt` before committing
- Run `cargo clippy` to check for common issues
- Ensure all tests pass with `cargo test`

## Pull Request Process

1. Ensure your code builds without errors and passes all tests
2. Update documentation if needed
3. Submit a pull request from your branch to `main` or `docs`
4. Request review from a maintainer
5. Address any review feedback

Thank you for contributing to Forge!