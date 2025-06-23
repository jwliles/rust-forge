# Introduction to DotForge

DotForge is a powerful symlink management tool designed as a modern alternative to GNU Stow. It provides comprehensive management of dotfiles, scripts, and other configuration files with clear, intuitive commands.

## Why DotForge?

DotForge was created to solve common issues with dotfile management:

- Managing dotfiles across multiple machines
- Keeping track of which files are symlinked where
- Switching between different configurations for the same applications
- Ensuring reliable tracking of dotfiles with a robust backend

## Key Features

- **Comprehensive Symlink Management**: Clear, intuitive commands for managing symlinks
- **Profile System**: Manage multiple configurations for the same target location
- **Multi-Directory Support**: Organize different types of symlinked content
- **Interactive Mode**: Visual interface for managing symlink operations
- **SQLite Backend**: Reliable state tracking with transactional safety
- **Modular Design**: Core functionality with optional feature modules

## Commands and Operations

DotForge provides a clear workflow for managing your dotfiles:

- **Stage**: Temporarily track files for symlinking
- **Link**: Create permanent symlinks for tracked files
- **Unlink**: Remove symlinks but keep files in the forge folder
- **Remove**: Delete files from the forge folder but keep originals
- **Delete**: Completely remove files from the system

## Use Cases

DotForge excels at managing various symlink scenarios:

1. **Dotfile Management**: Organize configuration files
2. **Script Organization**: Manage executable scripts in your PATH
3. **AppImage Integration**: Manage AppImages with desktop entry generation
4. **Font Management**: Organize and switch between font collections

## System Requirements

- Rust (Minimum supported version: 1.65.0)
- GNU/Linux or other free operating system
- Standard system libraries

## License

DotForge is released under the MIT License. See the [LICENSE](https://github.com/jwliles/dotforge/blob/main/LICENSE) file for details.
