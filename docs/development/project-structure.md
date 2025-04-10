# Project Structure

DotForge is organized into a modular structure following Rust conventions.

## Directory Structure

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

## Module Overview

### CLI Module

Handles command-line interface parsing and dispatching commands to their respective handlers.

### Config Module

Manages application configuration including database connections and user preferences.

### Dotfile Module

Provides core functionality for managing dotfiles, including backup, linking, and unlinking operations.

### Scanner Module

Implements directory scanning to find and process files for symlinking.

### Symlink Module

Contains cross-platform abstractions for symlink creation and management.

### Utils Module

Provides various utility functions used throughout the application.