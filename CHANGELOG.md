# Changelog

All notable changes to DotForge will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/jwliles/rust-dotforge/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/jwliles/rust-dotforge/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jwliles/rust-dotforge/releases/tag/v0.1.0