# Forge Project Context - March 23, 2025

## Project Overview

Forge is evolving into a comprehensive symlink management tool designed to replace and extend beyond GNU Stow's functionality. The project is being migrated from Go to Rust for improved safety, performance, and ecosystem benefits.

## Core Concepts

### Symlink Management

Forge will manage symlinks throughout the system using a consistent workflow metaphor:
- **Heat**: Stage files for symlinking
- **Forge**: Create the actual symlinks
- **Cool**: Remove symlinks and tracking

### Profile System

Profiles allow managing multiple versions of configurations:
- Use case: Multiple Neovim configurations for different purposes
- Quick switching between profiles
- Same target location, different source configurations
- Inspired by Git's branch switching mechanism

### Multiple Directories

Different from profiles, multiple directories help organize different types of symlinked content:
- Separate directories for dotfiles, scripts, etc.
- Different target locations with their own organization
- Keeps the system organized at a higher level

### Interactive Mode

A proposed feature accessed via the `-I` flag:
- Split-screen interface showing available profiles and active profile
- Easy navigation and selection for switching profiles
- Visual interface for managing symlink operations

## Use Cases

Forge is designed to handle various symlink management scenarios:

1. **Dotfile Management**: Traditional config file organization
2. **Script Organization**: Managing script symlinks in PATH
3. **AppImage Integration**: Managing AppImages and potentially generating desktop entries
4. **Font Management**: Organizing and switching between font collections

## Architecture Decisions

### Modular Design

- Core symlink management functionality remains focused
- Optional modules (like AppImage integration) available as feature flags
- Clear separation of concerns with plugin-like architecture

### Rust Benefits

- Memory safety without garbage collection
- Robust error handling via Result type
- Strong type system with enums and pattern matching
- Cross-platform considerations handled more cleanly

### Database Backend

- SQLite database for tracking symlinks and metadata
- Replaces file-based tracking from the Go version
- Provides transactional safety and querying capabilities

## Implementation Priorities

1. Core symlink management functionality
2. Profile management and switching
3. Multiple directory support
4. Interactive mode
5. Optional modules (AppImage, etc.)

## Open Questions & Decisions

1. **AppImage Integration**: Decided to implement as an optional module via Cargo features
2. **Font Management**: Confirmed as a valid use case for the tool
3. **Desktop Entry Generation**: Will be part of the optional AppImage module
4. **Interactive Mode**: Design will be similar to a split-screen file manager

## Next Steps

1. Refocus on core symlink management functionality
2. Define the basic data model for tracking symlinks
3. Implement the profile switching mechanism
4. Create the command structure for the CLI
5. Develop the interactive mode interface

## Technical Requirements

1. Cross-platform path handling
2. Safe symlink operations with backups
3. Efficient profile switching
4. SQLite-based state tracking
5. Modular codebase structure with feature flags
