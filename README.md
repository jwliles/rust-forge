# DotForge

A powerful symlink management tool designed as a modern alternative to GNU Stow.

> **EARLY DEVELOPMENT NOTICE**: This project is in its initial development stage. Features are being actively implemented and the API may change. Early versions are published to reserve the name and share development progress.

## Overview

DotForge provides comprehensive symlink management using an intuitive workflow metaphor inspired by blacksmithing:

- **Heat**: Stage files for symlinking
- **Forge**: Create the actual symlinks
- **Cool**: Remove symlinks and tracking

## Features

- **Profile System**: Manage multiple configurations for the same target location
- **Multi-Directory Support**: Organize different types of symlinked content (dotfiles, scripts, etc.)
- **Interactive Mode**: Visual interface for managing symlink operations
- **SQLite Backend**: Reliable state tracking with transactional safety
- **Modular Design**: Core functionality with optional feature modules

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
# Heat (stage) a file for symlinking
dotforge heat ~/.config/nvim/init.lua

# Create the symlinks for all heated files
dotforge forge

# Remove symlinks for specific files
dotforge cool ~/.config/nvim/init.lua

# Switch to a different profile
dotforge profile switch coding
```

## Profiles

Profiles allow you to maintain multiple configurations that target the same location:

```bash
# Create a new profile
dotforge profile create coding

# List available profiles
dotforge profile list

# Switch to a profile
dotforge profile switch coding
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
   - [ ] Basic file operations (heat, forge, cool)
   - [ ] SQLite state tracking

2. **v0.2.x** - Profile system and reliability
   - [ ] Multiple profile support
   - [ ] Profile switching
   - [ ] Transaction safety

3. **v0.3.x** - Interactive TUI mode
   - [ ] Visual interface for operations
   - [ ] Real-time status updates
   - [ ] Profile management

4. **v1.0.0** - Production release
   - [ ] Complete feature set
   - [ ] Comprehensive tests
   - [ ] Documentation

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.