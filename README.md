# Forge

A powerful symlink management tool designed as a modern alternative to GNU Stow.

> **EARLY DEVELOPMENT NOTICE**: This project is in its initial development stage. Features are being actively implemented and the API may change. Early versions are published to reserve the name and share development progress.

## Overview

Forge provides comprehensive symlink management with clear, intuitive commands:

- **Stage**: Temporarily track files for symlinking
- **Link**: Create permanent symlinks for tracked files
- **Unlink**: Remove symlinks but keep files in the forge folder
- **Remove**: Delete files from the forge folder but keep originals
- **Delete**: Completely remove files from the system

## Features

- **Profile System**: Manage multiple configurations for the same target location
- **Multi-Directory Support**: Organize different types of symlinked content (dotfiles, scripts, etc.)
- **Pack-and-Go System**: Create portable configuration bundles for easy deployment and backup
- **Recursive Directory Packing**: Pack entire directory trees with depth control and dry-run preview
- **BLAKE3 Integrity Verification**: Cryptographic hash verification for all packed files
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

Forge excels at managing various symlink scenarios:

1. **Dotfile Management**: Organize configuration files with portable backup and deployment
2. **Script Organization**: Manage executable scripts in your PATH
3. **Configuration Migration**: Create portable bundles for moving setups between systems
4. **Development Environment Setup**: Package and deploy complete development configurations
5. **Backup and Recovery**: Create cryptographically verified configuration backups
6. **AppImage Integration**: Manage AppImages with desktop entry generation
7. **Font Management**: Organize and switch between font collections

## Installation

```bash
# Install from crates.io (binary name will be 'forge')
cargo install forge-rs
```

## Basic Usage

```bash
# Initialize current directory as a forge managed folder (uses directory name if no name provided)
forge init [--name dotfiles]

# Stage a file for symlinking (creates a temporary link)
forge stage nvim/init.lua

# Stage an entire directory with all its contents recursively 
forge stage --recursive ~/.config/nvim

# Stage a directory with contents up to a specific depth
forge stage --depth=2 ~/.config/i3

# Create permanent symlinks for all staged files
forge link

# Remove symlinks but keep files in the forge folder
forge unlink init.lua

# Remove files from forge folder but keep originals
forge remove init.lua

# Completely delete files from the system
forge delete init.lua

# List all tracked files
forge list

# List all available profiles
forge list --profiles

# Switch to a different profile
forge switch coding
```

## Pack-and-Go System

Forge includes a powerful pack-and-go system for creating portable configuration bundles that can be easily deployed on new systems or used for backup and recovery.

### Basic Pack-and-Go Workflow

```bash
# 1. Initialize a new pack
forge start packing my_dotfiles

# 2. Add individual files
forge pack ~/.vimrc ~/.bashrc ~/.gitconfig

# 3. Add entire directories recursively
forge pack --recursive ~/.config/nvim/ ~/.ssh/

# 4. Add directories with limited depth
forge pack --depth 2 ~/.config/i3/

# 5. Preview what would be packed (without actually packing)
forge pack --recursive --dry-run ~/.local/bin/

# 6. Seal the pack into a portable archive
forge seal

# The result is a timestamped ZIP file: my_dotfiles-2025-06-23.zip
```

### Advanced Directory Packing

The pack system supports sophisticated directory handling:

```bash
# Pack entire directory trees while preserving structure
forge pack --recursive ~/.dotfiles/

# Limit recursion depth to avoid packing too much
forge pack --depth 3 ~/.config/

# Preview complex packing operations before execution
forge pack --recursive --dry-run ~/.home-configs/ ~/.work-configs/

# Combine different approaches in one pack
forge start packing mixed_environment
forge pack ~/.vimrc ~/.bashrc                    # Individual files
forge pack --recursive ~/.config/nvim/           # Full directory
forge pack --depth 1 ~/.local/bin/               # Limited depth
forge pack --dry-run --recursive ~/.scripts/     # Preview first
forge pack --recursive ~/.scripts/               # Then actually pack
forge seal
```

### Deployment and Installation

```bash
# Preview pack contents and installation plan
forge explain my_dotfiles-2025-06-23.zip

# Preview installation to specific directory
forge explain my_dotfiles-2025-06-23.zip --install --target /home/newuser

# Install to current directory (safe default)
forge install my_dotfiles-2025-06-23.zip

# Install to specific target directory
forge install my_dotfiles-2025-06-23.zip --target /home/user/configs

# Install with automatic home directory mapping
forge install my_dotfiles-2025-06-23.zip --map-home

# Preview installation without making changes
forge install my_dotfiles-2025-06-23.zip --dry-run --target /tmp/test

# Force installation (overwrite existing files)
forge install my_dotfiles-2025-06-23.zip --force

# Skip existing files during installation
forge install my_dotfiles-2025-06-23.zip --skip-existing
```

### Backup and Recovery

```bash
# Restore pack to original absolute paths (for backup recovery)
forge restore my_dotfiles-2025-06-23.zip

# Test restore to current directory (safe testing)
forge restore my_dotfiles-2025-06-23.zip --test

# Preview restore operation
forge restore my_dotfiles-2025-06-23.zip --dry-run

# Force restore (overwrite existing files)
forge restore my_dotfiles-2025-06-23.zip --force
```

### Pack Management

```bash
# Update files in an existing pack
forge repack --scope my_dotfiles ~/.vimrc ~/.bashrc

# Update all files in a pack with current versions
forge repack --scope my_dotfiles

# Remove files from a pack
forge unpack --scope my_dotfiles ~/.old_config

# Remove and reseal
forge unpack ~/.deprecated_files
forge seal
```

### Key Features

- **Directory Structure Preservation**: Recursive packing maintains full directory trees
- **Depth Control**: Limit recursion depth to avoid packing unwanted nested content
- **Dry-Run Mode**: Preview packing operations before execution
- **BLAKE3 Integrity**: All files are cryptographically hashed for integrity verification
- **Flexible Deployment**: Install to any directory with automatic path mapping
- **Safe Testing**: Preview and test operations before making permanent changes
- **Conflict Detection**: Identify file conflicts before installation

## Profiles

Profiles allow you to maintain multiple configurations that target the same location:

```bash
# Create a new profile in a specific location
forge new --profile coding ~/dotfiles/coding

# List available profiles
forge list --profiles

# Switch to a profile
forge switch coding

# Initialize current directory as a new profile
forge init --name coding
```

## Interactive Mode

Interactive TUI mode is under development. Launch with:

```bash
forge -I
```

*Note: Interactive mode is currently a placeholder and will display a message indicating it's not fully implemented.*

## Requirements

- Rust (Minimum supported version: 1.65.0)
- GNU/Linux or other free operating system
- Standard system libraries

**Note**: Forge is developed exclusively for free operating systems. It is not officially tested or supported on proprietary platforms.

## License

MIT License

## Development Roadmap

Forge is under active development with the following milestones:

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
   - [x] Man page documentation
   - [ ] Transaction safety with rollbacks
   - [ ] Shell completion scripts

4. **v0.4.x** - Pack-and-Go System
   - [x] Portable configuration bundles
   - [x] BLAKE3 integrity verification
   - [x] Recursive directory packing with depth control
   - [x] Dry-run preview mode
   - [x] Flexible deployment and restoration
   - [x] Pack management (repack, unpack)
   - [ ] Interactive TUI mode
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