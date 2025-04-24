# Changelog

All notable changes to DotForge will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/jwliles/rust-dotforge/compare/v0.3.2...HEAD
[0.3.2]: https://github.com/jwliles/rust-dotforge/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/jwliles/rust-dotforge/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/jwliles/rust-dotforge/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/jwliles/rust-dotforge/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jwliles/rust-dotforge/releases/tag/v0.1.0