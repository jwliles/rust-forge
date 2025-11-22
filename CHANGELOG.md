---
date_created: 2025-10-05T16-06-20
date_updated: 2025-10-05T16-06-20
timestamp: 1759680380888
title: CHANGELOG
id: 3e9632ad-2e24-4171-8bba-798d7150d678
hash: eaa859323727e1ba4c780cb29ea3af90f75115042e551a813eb707dc71778b51
---
# Changelog

All notable changes to Forge will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1] - 2025-10-05

### Fixed
- Pack-and-go exponential growth by excluding .forge directory from recursive operations
- Manifest path normalization bug in pack system

### Changed
- Removed dead code and unused functions for improved maintainability

### Added
- Comprehensive test suite with 15+ test files covering core functionality
- Test database isolation for parallel test execution
- Database isolation via environment variables (FORGE_TEST_DB, FORGE_TEST_CONFIG_DIR)

## [0.5.0] - 2025-08-15

### Fixed
- Critical panics from unwrap() calls in production code
- UI flush operations now handle errors gracefully
- Path handling edge cases with defensive error checking
- File operation logic to prevent data loss during symlink creation

### Changed
- Improved error handling throughout the codebase
- Operation ordering in file management to ensure safety

### Added
- Comprehensive bug fixes documented in BUGS.md
- Robust error handling for edge cases

## [0.4.0] - 2025-06-23

### Added
- Pack-and-go feature for creating portable configuration bundles
- Ability to export and import complete configuration setups
- Bundle compression and metadata tracking
- Configuration migration between systems

### Changed
- **BREAKING**: Crate renamed from `forge` to `forge-rs`
- Install with: `cargo install forge-rs` (binary remains `forge`)
- Updated documentation and references to use consistent `forge` naming

### Migration Notes
- Existing `forge` crate users: install `forge-rs` instead
- **BREAKING**: Configuration directories changed from `.dotforge/` to `.forge/` for naming consistency
- Users with existing repositories will need to manually rename `.dotforge/` to `.forge/` or reinitialize

## [0.3.2] - 2025-04-24

### Changed
- Streamlined command interface for better usability
- Added `switch` command to replace `profile switch`
- Added `new` command to create profiles in specific locations
- Enhanced `list` command to show profiles with `--profiles` flag
- Updated documentation to better explain path handling
- Marked redundant profile commands as legacy

## [0.3.1] - 2025-04-23

### Added
- Man page documentation
- Build script for man page installation
- Updated README with current implementation status

## [0.3.0] - 2025-04-23

### Added
- SQLite database for persistent storage of dotfile information
- Enhanced file operations with clear separation of actions:
  - `init` - Initialize a directory as a forge managed folder
  - `stage` - Temporarily track files for symlinking
  - `link` - Create permanent symlinks for tracked files
  - `unlink` - Remove symlinks but keep files in the forge folder
  - `remove` - Delete files from forge folder but keep originals
  - `delete` - Completely remove files from the system
  - `list` - Show tracked files and their statuses
- File status tracking (staged, linked, unlinked)
- Managed folders configuration for repository organization
- Confirmation prompts for destructive operations
- Database migration from file-based config
- Improved error handling and reporting

### Changed
- Redesigned command structure with intuitive staging workflow
- Removed ambiguous naming conventions
- Added proper safety checks for destructive operations
- Legacy command names are still supported temporarily for backward compatibility

## [0.2.0] - 2025-04-09

### Added
- Modular project structure based on Go version
- Module organization following Rust conventions
- Cross-platform symlink handling
- Directory scanning functionality
- Path utilities for handling home directory expansion

### Changed
- Updated dependencies in Cargo.toml
- Added error handling crates (thiserror, anyhow)
- Added documentation on project structure

## [0.1.0] - 2025-04-09

### Added
- Initial project skeleton
- Basic CLI interface with clap
- Command structure (heat, forge, cool, profile)
- Basic symlink creation tests

[Unreleased]: https://github.com/jwliles/rust-forge/compare/v0.5.1...HEAD
[0.5.1]: https://github.com/jwliles/rust-forge/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/jwliles/rust-forge/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/jwliles/rust-forge/compare/v0.3.2...v0.4.0
[0.3.2]: https://github.com/jwliles/rust-forge/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/jwliles/rust-forge/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/jwliles/rust-forge/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/jwliles/rust-forge/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jwliles/rust-forge/releases/tag/v0.1.0
