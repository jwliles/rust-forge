# DotForge

A powerful symlink management tool designed as a modern alternative to GNU Stow.

> **EARLY DEVELOPMENT NOTICE**: This project is in its initial development stage. Features are being actively implemented and the API may change. Early versions are published to reserve the name and share development progress.

## Overview

DotForge provides comprehensive symlink management with clear, intuitive commands:

- **Stage**: Temporarily track files for symlinking
- **Link**: Create permanent symlinks for tracked files
- **Unlink**: Remove symlinks but keep files in the forge folder
- **Remove**: Delete files from the forge folder but keep originals
- **Delete**: Completely remove files from the system

## Features

- **Profile System**: Manage multiple configurations for the same target location
- **Multi-Directory Support**: Organize different types of symlinked content (dotfiles, scripts, etc.)
- **Interactive Mode**: Visual interface for managing symlink operations
- **SQLite Backend**: Reliable state tracking with transactional safety
- **Modular Design**: Core functionality with optional feature modules

## Project Structure

The codebase is organized into modules following Rust conventions:

```
src/
├── main.rs             # Entry point and CLI definition
├── cli/                # CLI handling
│   ├── mod.rs          # CLI module exports
│   └── commands.rs     # Command implementations
├── config/             # Configuration handling
│   └── mod.rs          # Config struct and DB connection
├── dotfile/            # Core dotfile operations
│   ├── mod.rs          # DotFile struct definition
│   ├── backup.rs       # Backup functionality
│   ├── link.rs         # Linking functionality 
│   ├── list.rs         # Listing dotfiles
│   └── unlink.rs       # Unlinking functionality
├── scanner/            # Directory scanning
│   └── mod.rs          # Scanning functionality
├── symlink/            # Symlink operations
│   └── mod.rs          # Cross-platform symlink creation
└── utils/              # Utility functions
    ├── mod.rs          # Utils module exports
    └── path_utils.rs   # Path manipulation utilities
```

## Use Cases

DotForge excels at managing various symlink scenarios:

1. **Dotfile Management**: Organize configuration files
2. **Script Organization**: Manage executable scripts in your PATH
3. **AppImage Integration**: Manage AppImages with desktop entry generation
4. **Font Management**: Organize and switch between font collections

## Installation

```bash
# Install from crates.io
cargo install dotforge
```

## Basic Usage

```bash
# Initialize current directory as a forge managed folder (uses directory name if no name provided)
dotforge init [--name dotfiles]

# Stage a file for symlinking (creates a temporary link)
dotforge stage nvim/init.lua

# Create permanent symlinks for all staged files
dotforge link

# Remove symlinks but keep files in the forge folder
dotforge unlink init.lua

# Remove files from forge folder but keep originals
dotforge remove init.lua

# Completely delete files from the system
dotforge delete init.lua

# List all tracked files
dotforge list

# List all available profiles
dotforge list --profiles

# Switch to a different profile
dotforge switch coding
```

## Profiles

Profiles allow you to maintain multiple configurations that target the same location:

```bash
# Create a new profile in a specific location
dotforge new profile coding ~/dotfiles/coding

# List available profiles
dotforge list --profiles

# Switch to a profile
dotforge switch coding

# Initialize current directory as a new profile
dotforge init --name coding
```

## Interactive Mode

Launch the interactive TUI mode with:

```bash
dotforge -I
```

## License

MIT License

## Development Roadmap

DotForge is under active development with the following milestones:

1. **v0.1.x** - Core CLI structure and basic functionality
   - [x] Command-line interface with subcommands
   - [x] Module structure implementation
   - [x] Basic file operations (add, link, unlink)

2. **v0.2.x** - Basic profile system
   - [x] Profile creation
   - [x] Profile listing
   - [x] Profile switching

3. **v0.3.x** - Enhanced state management
   - [x] SQLite database for persistent storage
   - [x] Managed folders concept
   - [x] Complete file lifecycle (stage, link, unlink, remove, delete)
   - [x] Confirmation prompts for destructive operations
   - [x] File status tracking
   - [ ] Transaction safety with rollbacks
   - [ ] Man page documentation
   - [ ] Shell completion scripts

4. **v0.4.x** - Interactive TUI mode
   - [ ] Visual interface for operations
   - [ ] Real-time status updates
   - [ ] Profile management via TUI

5. **v1.0.0** - Production release
   - [ ] Complete feature set
   - [ ] Comprehensive tests
   - [ ] Comprehensive documentation

## Version Policy and crates.io Releases

This project follows semantic versioning (SemVer) for crates.io releases:

- **Patch updates (0.1.0 → 0.1.1)**: Bug fixes and minor documentation updates
- **Minor updates (0.1.0 → 0.2.0)**: New features that don't break compatibility
- **Major updates (0.x.x → 1.0.0)**: Breaking changes or API redesigns

### Publication Guidelines

- Documentation-only changes don't require a version bump
- The `docs` branch is used for documentation development
- Feature development occurs on dedicated `feature/*` branches
- Version bumps occur on the `main` branch before publication to crates.io

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.