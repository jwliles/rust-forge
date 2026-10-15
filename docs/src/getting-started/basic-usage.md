---
date_created: 2025-10-03T02-46-38
date_updated: 2026-05-30
title: basic-usage
---
# Basic Usage

## Repository Commands

```bash
forge init
forge init --name dotfiles --dir ~/dotfiles
forge list
forge list --profile work
forge list --profiles
```

- `init --name <NAME>` names the managed folder.
- `init --dir <DIRECTORY>` chooses the directory to initialize.
- `list --profile <NAME>` filters tracked files by profile.
- `list --profiles` lists legacy profiles.

## File Lifecycle

```bash
forge stage ~/.vimrc ~/.bashrc
forge stage --recursive ~/.config/nvim
forge stage --depth 2 ~/.config
forge link
forge link ~/.vimrc
```

- `stage --recursive` includes all files under directory arguments.
- `stage --depth <N>` limits directory traversal to `N` levels.
- `link` with no arguments links all staged files.
- `link <FILES>...` links only matching staged files.

Undo or remove tracking:

```bash
forge unstage ~/.vimrc
forge unstage --recursive ~/.config/nvim
forge unlink --yes ~/.vimrc
forge remove --yes ~/.old_config
forge delete --yes ~/.obsolete_config
forge purge ~/dotfiles
```

- `unstage --recursive` recursively matches staged files below a directory.
- `unstage --depth <N>` limits recursive matching depth.
- `unlink --yes`, `remove --yes`, and `delete --yes` skip confirmation prompts.
- `purge --recursive` processes subfolders; this flag defaults to true in the current CLI.

## Profiles

```bash
forge new --profile work ~/dotfiles/work
forge switch work
forge profile list
```

- `new --profile <NAME>` registers a named managed folder.
- `switch <NAME>` activates a legacy profile from `~/.forge/profiles/<NAME>`.
- `profile` subcommands are legacy compatibility commands.

## Packs

```bash
forge start packing my_dotfiles
forge pack --scope my_dotfiles ~/.vimrc
forge pack --scope my_dotfiles --recursive ~/.config/nvim
forge pack --scope my_dotfiles --depth 2 ~/.config
forge pack --scope my_dotfiles --dry-run --recursive ~/.local/bin
forge check --scope my_dotfiles
forge seal --scope my_dotfiles
```

- `pack --scope <SCOPE>`, `check --scope <SCOPE>`, and `seal --scope <SCOPE>` select the active pack. The default scope is the current directory name.
- `pack --recursive` includes all files under directory arguments.
- `pack --depth <N>` limits directory traversal to `N` levels.
- `pack --dry-run` previews manifest updates without writing.

Install or restore sealed packs:

```bash
forge explain my_dotfiles-2026-05-30.zip
forge explain my_dotfiles-2026-05-30.zip --install --target /tmp/test-home
forge install my_dotfiles-2026-05-30.zip --dry-run
forge install my_dotfiles-2026-05-30.zip --target ~/restored-config
forge install my_dotfiles-2026-05-30.zip --map-home
forge install my_dotfiles-2026-05-30.zip --force
forge install my_dotfiles-2026-05-30.zip --skip-existing
forge restore my_dotfiles-2026-05-30.zip --test
forge restore my_dotfiles-2026-05-30.zip --dry-run
```

- `explain --install` shows only the install plan.
- `explain --restore` shows only the restore plan.
- `explain --target <DIRECTORY>` previews an install target.
- `install --target <DIRECTORY>` writes under a target directory.
- `install --map-home` maps home-directory paths to the current user's home.
- `install --force` and `restore --force` overwrite conflicts.
- `install --skip-existing` and `restore --skip-existing` skip conflicts.
- `restore --test` restores by filename into the current directory.
- `--dry-run` previews without writing.

Maintain active packs:

```bash
forge repack --scope my_dotfiles ~/.vimrc
forge repack --scope my_dotfiles
forge unpack --scope my_dotfiles ~/.old_config
```

- `repack --scope <SCOPE>` refreshes manifest entries from disk.
- `unpack --scope <SCOPE>` removes source paths from the manifest.
