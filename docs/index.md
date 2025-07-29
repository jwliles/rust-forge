# Forge

A powerful symlink management tool designed as a modern alternative to GNU Stow.

## Overview

Forge provides comprehensive symlink management using an intuitive workflow metaphor inspired by blacksmithing:

- **Heat**: Stage files for symlinking
- **Forge**: Create the actual symlinks
- **Cool**: Remove symlinks and tracking

## Features

- **Profile System**: Manage multiple configurations for the same target location
- **Multi-Directory Support**: Organize different types of symlinked content (dotfiles, scripts, etc.)
- **Interactive Mode**: Visual interface for managing symlink operations
- **SQLite Backend**: Reliable state tracking with transactional safety
- **Modular Design**: Core functionality with optional feature modules

## Quick Start

```bash
# Install from crates.io
cargo install forge-rs

# Heat (stage) a file for symlinking
forge heat ~/.config/nvim/init.lua

# Create the symlinks for all heated files
forge forge
```

Check out the [Getting Started](getting-started/installation.md) section for more detailed instructions.