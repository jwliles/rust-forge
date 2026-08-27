# Forge

Forge is a symlink management tool for dotfiles, scripts, and portable configuration bundles. It uses an explicit workflow: initialize a managed folder, stage files, then link them into place.

## Installation

```bash
cargo install forge-rs
```

The installed binary is `forge`.

## Global Flags

- `-v`, `--verbose`: sets `FORGE_VERBOSE=1` for commands that emit additional diagnostic output.
- `-I`, `--interactive`: starts the placeholder interactive mode when no subcommand is provided. The TUI is not implemented yet.
- `-h`, `--help`: prints help for `forge` or a subcommand.
- `-V`, `--version`: prints the Forge version.

## Core Workflow

```bash
# Create a managed folder in the current directory.
forge init --name dotfiles

# Stage files. This records them and creates temporary links in the managed folder.
forge stage ~/.vimrc ~/.bashrc

# Move staged files into the managed folder and replace originals with symlinks.
forge link

# Show tracked files.
forge list
```

## Commands and Flags

### `forge init`

Initialize a managed folder and register it in Forge's configuration.

```bash
forge init
forge init --name dotfiles --dir ~/dotfiles
```

- `-n`, `--name <NAME>`: names the managed folder. Defaults to the initialized directory name.
- `-d`, `--dir <DIRECTORY>`: directory to initialize. Defaults to the current directory and is created if missing.

### `forge stage`

Stage files or directories before linking.

```bash
forge stage ~/.vimrc ~/.config/git/config
forge stage --recursive ~/.config/nvim
forge stage --depth 2 ~/.config
```

- `<FILES>...`: files or directories to stage.
- `-r`, `--recursive`: stages files below directory arguments with no depth limit.
- `--depth <N>`: stages files below directory arguments up to `N` levels deep. When present, this depth limit is used instead of unlimited recursion.

Without `--recursive` or `--depth`, a directory argument is staged as a directory entry but its contents are not staged.

### `forge link`

Link staged files.

```bash
forge link
forge link ~/.vimrc
forge link nvim
```

- `<FILES>...`: optional files or directories to link. If omitted, Forge links all staged files.

For regular files, Forge copies the original into the managed folder, removes the original, creates a symlink at the original path, and marks the record as linked.

### `forge unstage`

Deactivate staged records without deleting original files.

```bash
forge unstage ~/.vimrc
forge unstage --recursive ~/.config/nvim
forge unstage
```

- `<FILES>...`: files or directories to unstage. If omitted, all staged files are unstaged.
- `-r`, `--recursive`: when a directory is provided, unstages matching staged files below it.
- `--depth <N>`: limits recursive directory matching to `N` levels. It is only applied when recursive directory processing is active.

### `forge unlink`

Remove symlinks and restore files from the managed folder while keeping the managed copies.

```bash
forge unlink ~/.vimrc
forge unlink --yes ~/.bashrc
```

- `<FILES>...`: tracked files to unlink. If omitted, Forge prints the tracked file list.
- `-y`, `--yes`: skips the confirmation prompt.

### `forge remove`

Remove files from Forge tracking and delete the managed copy while keeping or restoring the original file.

```bash
forge remove ~/.vimrc
forge remove --yes ~/.old_config
```

- `<FILES>...`: tracked files to remove. If omitted, Forge prints the tracked file list.
- `-y`, `--yes`: skips the confirmation prompt.

### `forge delete`

Delete tracked files from both their original locations and the managed folder.

```bash
forge delete ~/.obsolete_config
forge delete --yes ~/.obsolete_config
```

- `<FILES>...`: files to delete. If a file is not tracked, Forge can still delete the path you provide.
- `-y`, `--yes`: skips the typed destructive confirmation. Use with care.

### `forge purge`

Remove Forge records and managed files for a folder, restoring linked originals first when possible.

```bash
forge purge
forge purge ~/dotfiles
forge purge --recursive ~/dotfiles
```

- `<FOLDER>`: folder to purge. Defaults to the active managed folder.
- `-r`, `--recursive`: process subfolders recursively. This flag defaults to true in the current CLI.

### `forge list`

List tracked files or legacy profiles.

```bash
forge list
forge list --profile work
forge list --profiles
```

- `--profiles`: lists legacy profiles from `~/.forge/profiles` instead of tracked files.
- `-p`, `--profile <NAME>`: filters tracked files by profile name.

### `forge new`

Create a managed folder for a named profile.

```bash
forge new --profile work ~/dotfiles/work
```

- `-p`, `--profile <NAME>`: profile name to register.
- `<PATH>`: location for the new profile's managed folder.

### `forge switch`

Switch to a legacy profile by creating symlinks from `~/.forge/profiles/<NAME>` into the default target path.

```bash
forge switch work
```

- `<NAME>`: profile name.

### `forge profile`

Legacy profile commands. Prefer `forge new`, `forge list --profiles`, and `forge switch`.

```bash
forge profile create work
forge profile list
forge profile switch work
```

## Pack-and-Go

Packs are portable ZIP archives with a manifest and BLAKE3 hashes. Current pack staging stores source paths and hashes in `.forge/tmp/pack/<SCOPE>/manifest.toml`; files are read from their source locations when the pack is sealed.

```bash
forge start packing my_dotfiles
forge pack --scope my_dotfiles ~/.vimrc ~/.bashrc
forge pack --scope my_dotfiles --recursive ~/.config/nvim
forge check --scope my_dotfiles
forge seal --scope my_dotfiles
```

### `forge start packing`

Create an empty pack staging area.

```bash
forge start packing my_dotfiles
```

- `<SCOPE>`: unique pack name used for staging and archive naming.

### `forge pack`

Add files to an existing pack manifest.

```bash
forge pack --scope my_dotfiles ~/.vimrc ~/.gitconfig
forge pack --scope my_dotfiles --recursive ~/.config/nvim
forge pack --scope my_dotfiles --depth 2 ~/.config
forge pack --scope my_dotfiles --dry-run --recursive ~/.local/bin
```

- `<FILES>...`: files or directories to add.
- `-s`, `--scope <SCOPE>`: pack scope. Defaults to the current directory name.
- `-r`, `--recursive`: includes files below directory arguments with no depth limit.
- `--depth <N>`: includes files below directory arguments up to `N` levels deep.
- `--dry-run`: prints what would be added without updating the manifest.

Directories are only expanded when `--recursive` or `--depth` is used. `.forge` directories are skipped while packing.

### `forge check`

Verify that files in an active pack manifest still exist and match their stored hashes.

```bash
forge check --scope my_dotfiles
```

- `-s`, `--scope <SCOPE>`: pack scope to check. Defaults to the current directory name.

### `forge seal`

Validate an active pack and write `.forge/archives/<SCOPE>-YYYY-MM-DD.zip`.

```bash
forge seal --scope my_dotfiles
```

- `-s`, `--scope <SCOPE>`: pack scope to seal. Defaults to the current directory name.

Sealing aborts if any manifest file is missing or has changed since it was packed.

### `forge explain`

Inspect a sealed pack and preview install or restore destinations.

```bash
forge explain my_dotfiles-2026-05-30.zip
forge explain my_dotfiles-2026-05-30.zip --install --target /tmp/test-home
forge explain my_dotfiles-2026-05-30.zip --restore
```

- `<ARCHIVE>`: path to a sealed `.zip` pack.
- `--install`: show only the `forge install` plan.
- `--restore`: show only the `forge restore` plan.
- `-t`, `--target <DIRECTORY>`: target directory for the install preview.

If neither `--install` nor `--restore` is set, both plans are shown.

### `forge install`

Install a sealed pack on another system.

```bash
forge install my_dotfiles-2026-05-30.zip
forge install my_dotfiles-2026-05-30.zip --target ~/restored-config
forge install my_dotfiles-2026-05-30.zip --map-home
forge install my_dotfiles-2026-05-30.zip --dry-run --skip-existing
forge install my_dotfiles-2026-05-30.zip --force
```

- `<ARCHIVE>`: path to a sealed `.zip` pack.
- `-f`, `--force`: overwrite existing destination files.
- `--skip-existing`: skip destination files that already exist. Mutually exclusive with `--force`.
- `-t`, `--target <DIRECTORY>`: install into this directory. Without `--map-home`, absolute source paths are installed by filename under the target.
- `--map-home`: map source home-directory paths to the current user's home, or under `--target` when both flags are used.
- `--dry-run`: print the installation plan without writing files.

Without `--target` or `--map-home`, install writes files by filename into the current directory.

### `forge restore`

Restore a sealed pack to original absolute paths, or test restore into the current directory.

```bash
forge restore my_dotfiles-2026-05-30.zip --dry-run
forge restore my_dotfiles-2026-05-30.zip --test
forge restore my_dotfiles-2026-05-30.zip --force
forge restore my_dotfiles-2026-05-30.zip --skip-existing
```

- `<ARCHIVE>`: path to a sealed `.zip` pack.
- `-f`, `--force`: overwrite existing destination files.
- `--skip-existing`: skip destination files that already exist. Mutually exclusive with `--force`.
- `--test`: restore by filename into the current directory instead of original absolute paths.
- `--dry-run`: print the restore plan without writing files.

### `forge repack`

Refresh files in an active pack manifest from disk.

```bash
forge repack --scope my_dotfiles ~/.vimrc
forge repack --scope my_dotfiles
```

- `-s`, `--scope <SCOPE>`: pack scope. Defaults to the current directory name.
- `<FILES>...`: files to refresh. If omitted, Forge attempts to refresh every source path listed in the manifest.

### `forge unpack`

Remove files from an active pack manifest.

```bash
forge unpack --scope my_dotfiles ~/.old_config
```

- `<FILES>...`: source paths to remove from the manifest.
- `-s`, `--scope <SCOPE>`: pack scope. Defaults to the current directory name.

## Files

- `~/.forge/`: global Forge configuration directory.
- `~/.forge/config.db`: global SQLite database.
- `~/.forge/profiles/`: legacy profile directory.
- `.forge/`: local metadata directory inside a managed folder.
- `.forge/tmp/pack/<SCOPE>/manifest.toml`: active pack manifest.
- `.forge/archives/`: sealed pack archive directory.

## Requirements

- Rust 1.65 or newer to build from source.
- GNU/Linux or another free operating system. Proprietary platforms are not officially tested.

## License

GPL-3.0-or-later.
