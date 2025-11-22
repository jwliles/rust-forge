---
date_created: 2025-10-03T02-46-38
date_updated: 2025-10-03T02-46-38
timestamp: 1759459598162
title: index
id: 38d961c9-4442-4be5-8051-fb723038bc50
hash: 234ed458b114ac803044cfa616605e03fccae186574850cf64c5c29048622e9a
---
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