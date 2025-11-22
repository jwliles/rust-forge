---
date_created: 2025-10-05T16-06-33
date_updated: 2025-10-05T16-06-33
timestamp: 1759680393826
title: MANUAL
id: c6a156a7-e435-44c9-916f-553012486028
hash: 5fc8be218199d47b6f4ed00cfb364e96c08b939febd7a2c477f009faef46b066
---
# Forge Manual

**FORGE(1)** - User Commands - **forge 0.5.1** - October 2025

## NAME

forge - a powerful symlink management tool with pack-and-go configuration bundles

## SYNOPSIS

**forge** [**-v**|**--verbose**] [**-I**|**--interactive**] [**-h**|**--help**] [**-V**|**--version**] *COMMAND* [*COMMAND_OPTIONS*] [*ARGS*...]

### Global Flags

- **-v, --verbose**: Enable verbose output for debugging and troubleshooting. When set, Forge prints additional details about operations, errors, and internal state to help diagnose issues.
- **-I, --interactive**: Use interactive mode (TUI, under development).
- **-h, --help**: Print help information.
- **-V, --version**: Print version information.

## DESCRIPTION

**forge** is a modern alternative to GNU Stow for managing configuration files through symlinks. It provides advanced features including profile management, recursive directory staging, and portable pack-and-go bundles for configuration migration between systems.

Forge follows a deliberate, safe-by-default philosophy where configuration files are staged before linking, preventing accidental modifications. The pack-and-go system enables users to create portable archives of their configurations with BLAKE3 integrity checking and flexible deployment options.

The tool operates in multiple phases: initialization creates managed repositories, staging prepares files for tracking, linking creates permanent symlinks, and pack-and-go creates portable bundles for migration. All operations maintain detailed SQLite database records for tracking and recovery.

## COMMANDS

### Repository Management

#### init [**-n**|**--name** *NAME*] [**-d**|**--dir** *DIRECTORY*]

Initialize a directory as a forge managed folder. Creates .forge subdirectory and registers the repository in the global database. If *NAME* is not provided, uses the directory name. If *DIRECTORY* is not provided, uses current working directory. Sets up SQLite database for file tracking.

#### list [**--profiles**] [**-p**|**--profile** *NAME*]

List tracked files or available profiles. Without options, shows all tracked files in the current repository with their status (staged, linked, unlinked). With **--profiles**, lists all available profiles. With **--profile** *NAME*, shows files only in the specified profile.

### File Management

#### stage *FILES*... [**-r**|**--recursive**] [**--depth** *N*]

Stage files or directories for tracking. Creates temporary symlinks from the forge directory to original files. Files remain in original locations until **link** is called. With **--recursive**, processes directories recursively to unlimited depth. With **--depth** *N*, limits recursion to N levels (overrides **--recursive**). Preserves directory structure in forge repository. Updates SQLite database with staged status.

#### link [*FILES*...]

Create permanent symlinks for staged files. Moves files from original locations to forge directory and creates symlinks at original locations. If *FILES* are specified, links only those files. If no files specified, links all staged files. For directories, updates tracking status without moving the directory itself. Updates database status from staged to linked.

#### unlink *FILES*... [**-y**|**--yes**]

Remove symlinks and restore original files. Copies files from forge directory back to original locations, removes symlinks, and updates database status to staged. Prompts for confirmation unless **--yes** is specified. Files remain in forge directory for potential re-linking.

#### remove *FILES*... [**-y**|**--yes**]

Remove files from forge tracking completely. Restores original files to their locations, removes files from forge directory, and removes database entries. This completely severs the relationship between forge and the files. Prompts for confirmation unless **--yes** is specified.

#### delete *FILES*... [**-y**|**--yes**]

Delete files completely from the system. Removes files from both forge directory and original locations, and removes database entries. This is a destructive operation that cannot be undone. Requires explicit confirmation unless **--yes** is specified.

### Profile Management

#### switch *NAME*

Switch to a profile and activate all its files. Creates symlinks for all files associated with the specified profile from their profile directory to the default target directory. Updates database with profile associations. Uses walkdir to process profile directory contents.

#### new **--profile** *NAME* *PATH*

Create a new profile at the specified location. Initializes the path as a forge managed folder with the given profile name. Equivalent to running **init** with **--name** *NAME* and **--dir** *PATH*.

#### profile create *NAME*

(Legacy) Create a new profile in the default profiles directory (~/.forge/profiles/*NAME*). Creates directory structure if it does not exist. Deprecated in favor of **new** command.

#### profile list

(Legacy) List all available profiles by scanning the profiles directory. Shows profile names found in ~/.forge/profiles/. Deprecated in favor of **list --profiles**.

#### profile switch *NAME*

(Legacy) Switch to the specified profile. Deprecated in favor of **switch** command.

### Pack-and-Go System

#### start packing *SCOPE*

Initialize a new pack with the given scope identifier. Creates staging directory at .forge/tmp/pack/*SCOPE*/files/ and initializes manifest.toml with pack metadata. Prevents accidental pack creation by requiring explicit "start packing" command. Scope must be unique within the repository.

#### pack *FILES*... [**-s**|**--scope** *SCOPE*] [**-r**|**--recursive**] [**--depth** *N*] [**--dry-run**]

Add files to an existing pack staging area. Copies files to pack directory with relative paths preserved, calculates BLAKE3 hashes, and updates manifest with file metadata (target path, size, modification time, hash). If **--scope** is not specified, uses current directory name as scope. Files are copied, not moved, preserving originals. With **--recursive**, processes directories recursively to unlimited depth, preserving directory structure within the pack. With **--depth** *N*, limits recursion to N levels (overrides **--recursive**). With **--dry-run**, shows what would be packed without actually copying files.

#### seal [**-s**|**--scope** *SCOPE*]

Finalize pack into timestamped ZIP archive. Creates archive at .forge/archives/*SCOPE*-YYYY-MM-DD.zip with Deflate compression. Includes manifest.toml and all files from staging area. Removes staging directory after successful archive creation. Archive naming prevents overwrites by including date.

#### install *ARCHIVE* [**-f**|**--force**] [**--skip-existing**] [**-t**|**--target** *DIRECTORY*] [**--map-home**] [**--dry-run**]

Install a sealed pack on a new system. Extracts archive to temporary directory, validates manifest.toml, and installs files. By default, installs to current working directory using filenames only. With **--target**, installs relative to specified directory. With **--map-home**, maps home directory paths to current user. With **--force**, overwrites existing files. With **--skip-existing**, skips files that already exist. With **--dry-run**, shows installation plan without making changes. Validates BLAKE3 hashes during installation.

#### restore *ARCHIVE* [**-f**|**--force**] [**--skip-existing**] [**--test**] [**--dry-run**]

Restore a sealed pack to original absolute paths on current system. Used for configuration recovery and backup restoration. By default, restores to original paths from manifest. With **--test**, restores to current directory using filenames only for safe testing. With **--force**, overwrites existing files. With **--skip-existing**, skips files that already exist. With **--dry-run**, shows restoration plan without making changes. Validates BLAKE3 hashes during restoration.

#### explain *ARCHIVE* [**--install**] [**--restore**] [**-t**|**--target** *DIRECTORY*]

Analyze pack contents and show detailed installation/restoration plans. Extracts manifest without installing files. Shows pack metadata (scope, creation date, file count, total size), complete file listing with paths and hash previews, and installation plans with conflict detection. By default shows both install and restore plans. With **--install**, shows only installation plan. With **--restore**, shows only restoration plan. With **--target**, previews installation to specific directory.

#### repack [**-s**|**--scope** *SCOPE*] [*FILES*...]

Update files in an existing pack staging area. If *FILES* are specified, repacks only those files with updated content and metadata. If no files specified, repacks all files listed in manifest with current content from disk. Updates BLAKE3 hashes and modification times. Does not re-seal the pack.

#### unpack *FILES*... [**-s**|**--scope** *SCOPE*]

Remove files from pack staging area. Removes specified files from pack directory and removes entries from manifest.toml. Does not affect files in their original locations. Pack remains active for additional files or sealing.

## OPTIONS

#### **-v**, **--verbose**
Enable verbose output for debugging and troubleshooting. Prints additional details about operations, errors, and internal state.

#### **-I**, **--interactive**
Use interactive mode (under development)

#### **-h**, **--help**
Print help information.

#### **-V**, **--version**
Print version information.

## PACK-AND-GO WORKFLOW

The pack-and-go system follows a deliberate workflow for creating portable configuration bundles:

**1. Create Pack**
```bash
forge start packing vim_config
```

**2. Add Files**
```bash
forge pack ~/.vimrc ~/.vim/
```

**3. Seal Archive**
```bash
forge seal
```

**4. Deploy on New System**
```bash
# Test installation
forge explain vim_config-2025-06-23.zip
forge install vim_config-2025-06-23.zip --dry-run
# Production installation
forge install vim_config-2025-06-23.zip --target /home/user
```

**5. Restore on Current System**
```bash
# Recover from backup
forge restore vim_config-2025-06-23.zip --force
```

## EXAMPLES

### Repository Management

Initialize a forge repository in current directory:
```bash
forge init
```

Initialize a forge repository with custom name and location:
```bash
forge init --name my_dotfiles --dir /path/to/config
```

List all managed repositories and their status:
```bash
forge list
```

List all available profiles:
```bash
forge list --profiles
```

### File Staging and Linking

Stage individual configuration files:
```bash
forge stage ~/.vimrc ~/.bashrc ~/.gitconfig
```

Stage a directory with limited recursion depth:
```bash
forge stage --depth 2 ~/.config/nvim
```

Stage a directory recursively (unlimited depth):
```bash
forge stage --recursive ~/.config
```

Link all staged files:
```bash
forge link
```

Link specific files only:
```bash
forge link ~/.vimrc ~/.bashrc
```

### Pack-and-Go Examples

Create a complete dotfiles backup:
```bash
forge start packing complete_backup
forge pack ~/.vimrc ~/.bashrc ~/.gitconfig ~/.ssh/config
forge pack --scope complete_backup --recursive ~/.config/nvim/
forge seal --scope complete_backup
```

Install to specific directory:
```bash
forge install vim_minimal-2025-06-23.zip --target /home/user/configs
```

Dry run installation (preview only):
```bash
forge install vim_minimal-2025-06-23.zip --dry-run --target /tmp/test
```

## FILES

### Global Configuration
- **~/.forge/** - Global forge configuration directory
- **~/.forge/config.db** - Global SQLite database tracking repositories
- **~/.forge/profiles/** - Default location for legacy profile directories

### Local Repository Structure
- **.forge/** - Local repository metadata directory
- **.forge/database.db** - Local SQLite database for current repository
- **.forge/tmp/pack/** - Pack staging areas
- **.forge/archives/** - Sealed pack archives

## EXIT STATUS

**forge** exits with status 0 on success, and >0 if an error occurs.

## SECURITY

Forge employs BLAKE3 cryptographic hashing for comprehensive file integrity verification in pack-and-go archives. Pack manifests contain comprehensive metadata including original absolute file paths, file sizes, modification timestamps, and BLAKE3 hash values.

Users should always verify pack sources and contents before installation using **forge explain**.

## ENVIRONMENT

- **HOME** - Used to determine global configuration directory location
- **TMPDIR**, **TMP**, **TEMP** - Used for temporary directories during pack operations

## AUTHOR

Written by jwl.

## REPORTING BUGS

Report bugs to the GitHub issue tracker: <https://github.com/jwliles/rust-forge/issues>

## COPYRIGHT

Copyright © 2025 jwl. License GPL-3.0-or-later.

This is free software: you are free to change and redistribute it under the terms of the GNU General Public License.

## SEE ALSO

**stow(8)**, **ln(1)**, **rsync(1)**, **git(1)**, **symlink(2)**, **blake3sum(1)**, **zip(1)**, **sqlite3(1)**

Online documentation: <https://github.com/jwliles/rust-forge/wiki>
