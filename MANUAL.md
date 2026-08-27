# Forge Manual

**FORGE(1)** - User Commands - **forge 0.5.1**

## NAME

forge - manage symlinked files and portable configuration packs

## SYNOPSIS

**forge** [**-v**|**--verbose**] [**-I**|**--interactive**] [**-h**|**--help**] [**-V**|**--version**] *COMMAND* [*OPTIONS*] [*ARGS*...]

## DESCRIPTION

Forge manages files through an explicit stage/link workflow. A managed folder stores the canonical copies of linked files, while original locations become symlinks after `forge link`.

Forge also includes a Pack-and-Go system for portable ZIP archives. Active packs store source paths and BLAKE3 hashes in a manifest; source files are read and archived when `forge seal` runs.

## GLOBAL OPTIONS

- **-v, --verbose**: sets `FORGE_VERBOSE=1` for commands that emit diagnostic output.
- **-I, --interactive**: starts placeholder interactive mode when no command is provided. The TUI is not implemented yet.
- **-h, --help**: prints help.
- **-V, --version**: prints version information.

Examples:

```bash
forge --help
forge --version
forge -v list
forge -I
```

## COMMANDS

### Repository Management

#### init [**-n**|**--name** *NAME*] [**-d**|**--dir** *DIRECTORY*]

Initialize a managed folder. Creates the directory if needed, creates `.forge`, and registers the folder.

Flags:

- **-n, --name** *NAME*: managed folder name. Defaults to the initialized directory name.
- **-d, --dir** *DIRECTORY*: directory to initialize. Defaults to the current directory.

Examples:

```bash
forge init
forge init --name dotfiles --dir ~/dotfiles
```

#### list [**--profiles**] [**-p**|**--profile** *NAME*]

List tracked files or legacy profiles.

Flags:

- **--profiles**: list legacy profiles from `~/.forge/profiles`.
- **-p, --profile** *NAME*: filter tracked files by profile name.

Examples:

```bash
forge list
forge list --profile work
forge list --profiles
```

### File Management

#### stage *FILES*... [**-r**|**--recursive**] [**--depth** *N*]

Stage files or directories. Files remain at their original paths until `forge link`.

Flags:

- **-r, --recursive**: include all files under directory arguments.
- **--depth** *N*: include files under directory arguments up to *N* levels deep.

Examples:

```bash
forge stage ~/.vimrc ~/.bashrc
forge stage --recursive ~/.config/nvim
forge stage --depth 2 ~/.config
```

#### link [*FILES*...]

Link staged files. If no files are specified, links all staged files.

Examples:

```bash
forge link
forge link ~/.vimrc
forge link nvim
```

#### unstage [*FILES*...] [**-r**|**--recursive**] [**--depth** *N*]

Deactivate staged records without deleting original files. If no files are specified, unstages all staged files.

Flags:

- **-r, --recursive**: when a directory is supplied, match staged files below it.
- **--depth** *N*: limit recursive matching depth.

Examples:

```bash
forge unstage ~/.vimrc
forge unstage --recursive ~/.config/nvim
forge unstage
```

#### unlink *FILES*... [**-y**|**--yes**]

Remove symlinks and restore files from the managed folder while keeping the managed copies.

Flags:

- **-y, --yes**: skip confirmation.

Examples:

```bash
forge unlink ~/.vimrc
forge unlink --yes ~/.bashrc
```

#### remove *FILES*... [**-y**|**--yes**]

Remove files from Forge tracking and delete managed copies while keeping or restoring originals.

Flags:

- **-y, --yes**: skip confirmation.

Examples:

```bash
forge remove ~/.vimrc
forge remove --yes ~/.old_config
```

#### delete *FILES*... [**-y**|**--yes**]

Delete files from original locations, managed locations, and the database. This is destructive.

Flags:

- **-y, --yes**: skip the typed destructive confirmation.

Examples:

```bash
forge delete ~/.obsolete_config
forge delete --yes ~/.obsolete_config
```

#### purge [*FOLDER*] [**-r**|**--recursive**]

Purge Forge records and managed files for a folder, restoring linked originals first when possible.

Flags:

- **-r, --recursive**: process subfolders recursively. The current CLI defaults this flag to true.

Examples:

```bash
forge purge
forge purge ~/dotfiles
forge purge --recursive ~/dotfiles
```

### Profile Management

#### new **--profile** *NAME* *PATH*

Create a managed folder for a profile name.

Flags:

- **-p, --profile** *NAME*: profile name to register.

Example:

```bash
forge new --profile work ~/dotfiles/work
```

#### switch *NAME*

Switch to a legacy profile by symlinking from `~/.forge/profiles/<NAME>`.

Example:

```bash
forge switch work
```

#### profile create *NAME*
#### profile list
#### profile switch *NAME*

Legacy profile commands. Prefer `forge new`, `forge list --profiles`, and `forge switch`.

Examples:

```bash
forge profile create work
forge profile list
forge profile switch work
```

### Pack-and-Go

#### start packing *SCOPE*

Create an active pack manifest at `.forge/tmp/pack/<SCOPE>/manifest.toml`.

Example:

```bash
forge start packing my_dotfiles
```

#### pack *FILES*... [**-s**|**--scope** *SCOPE*] [**-r**|**--recursive**] [**--depth** *N*] [**--dry-run**]

Add files to an active pack manifest. Files are hashed and recorded; they are archived later by `forge seal`.

Flags:

- **-s, --scope** *SCOPE*: pack scope. Defaults to the current directory name.
- **-r, --recursive**: include all files under directory arguments.
- **--depth** *N*: include files under directory arguments up to *N* levels deep.
- **--dry-run**: show what would be packed without updating the manifest.

Examples:

```bash
forge pack --scope my_dotfiles ~/.vimrc ~/.gitconfig
forge pack --scope my_dotfiles --recursive ~/.config/nvim
forge pack --scope my_dotfiles --depth 2 ~/.config
forge pack --scope my_dotfiles --dry-run --recursive ~/.local/bin
```

#### check [**-s**|**--scope** *SCOPE*]

Verify that files in an active pack manifest still exist and match stored BLAKE3 hashes.

Flags:

- **-s, --scope** *SCOPE*: pack scope. Defaults to the current directory name.

Example:

```bash
forge check --scope my_dotfiles
```

#### seal [**-s**|**--scope** *SCOPE*]

Validate an active pack and create `.forge/archives/<SCOPE>-YYYY-MM-DD.zip`.

Flags:

- **-s, --scope** *SCOPE*: pack scope. Defaults to the current directory name.

Example:

```bash
forge seal --scope my_dotfiles
```

#### explain *ARCHIVE* [**--install**] [**--restore**] [**-t**|**--target** *DIRECTORY*]

Inspect a sealed pack and preview install or restore plans.

Flags:

- **--install**: show only the install plan.
- **--restore**: show only the restore plan.
- **-t, --target** *DIRECTORY*: target directory for the install preview.

Examples:

```bash
forge explain my_dotfiles-2026-05-30.zip
forge explain my_dotfiles-2026-05-30.zip --install --target /tmp/test-home
forge explain my_dotfiles-2026-05-30.zip --restore
```

#### install *ARCHIVE* [**-f**|**--force**] [**--skip-existing**] [**-t**|**--target** *DIRECTORY*] [**--map-home**] [**--dry-run**]

Install a sealed pack. By default, files are installed by filename into the current directory.

Flags:

- **-f, --force**: overwrite existing destination files.
- **--skip-existing**: skip destination files that already exist. Mutually exclusive with `--force`.
- **-t, --target** *DIRECTORY*: install into this directory.
- **--map-home**: map source home-directory paths to the current user's home, or under `--target` when both are provided.
- **--dry-run**: show the install plan without writing files.

Examples:

```bash
forge install my_dotfiles-2026-05-30.zip
forge install my_dotfiles-2026-05-30.zip --target ~/restored-config
forge install my_dotfiles-2026-05-30.zip --map-home
forge install my_dotfiles-2026-05-30.zip --dry-run --skip-existing
forge install my_dotfiles-2026-05-30.zip --force
```

#### restore *ARCHIVE* [**-f**|**--force**] [**--skip-existing**] [**--test**] [**--dry-run**]

Restore a sealed pack to original absolute paths, or test restore into the current directory.

Flags:

- **-f, --force**: overwrite existing destination files.
- **--skip-existing**: skip destination files that already exist. Mutually exclusive with `--force`.
- **--test**: restore by filename into the current directory.
- **--dry-run**: show the restore plan without writing files.

Examples:

```bash
forge restore my_dotfiles-2026-05-30.zip --dry-run
forge restore my_dotfiles-2026-05-30.zip --test
forge restore my_dotfiles-2026-05-30.zip --force
forge restore my_dotfiles-2026-05-30.zip --skip-existing
```

#### repack [**-s**|**--scope** *SCOPE*] [*FILES*...]

Refresh files in an active pack manifest.

Flags:

- **-s, --scope** *SCOPE*: pack scope. Defaults to the current directory name.

Examples:

```bash
forge repack --scope my_dotfiles ~/.vimrc
forge repack --scope my_dotfiles
```

#### unpack *FILES*... [**-s**|**--scope** *SCOPE*]

Remove source paths from an active pack manifest.

Flags:

- **-s, --scope** *SCOPE*: pack scope. Defaults to the current directory name.

Example:

```bash
forge unpack --scope my_dotfiles ~/.old_config
```

## FILES

- **~/.forge/**: global Forge configuration directory.
- **~/.forge/config.db**: global SQLite database.
- **~/.forge/profiles/**: legacy profile directory.
- **.forge/**: local managed-folder metadata.
- **.forge/tmp/pack/**: active pack staging directories.
- **.forge/tmp/pack/SCOPE/manifest.toml**: active pack manifest.
- **.forge/archives/**: sealed pack archives.

## EXIT STATUS

Forge generally exits with status 0 on success and nonzero on fatal errors. Some command handlers currently report recoverable operation failures on stderr without terminating the process.

## SECURITY

Pack manifests include original paths, relative archive paths, file sizes, timestamps, and BLAKE3 hashes. Use `forge explain` and `--dry-run` before installing or restoring packs from untrusted sources.

## REPORTING BUGS

Report bugs at <https://github.com/jwliles/rust-forge/issues>.

## COPYRIGHT

Copyright © 2025 jwl. License GPL-3.0-or-later.
