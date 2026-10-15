---
date_created: 2025-10-03T02-46-38
date_updated: 2026-05-30
title: overview
---
# Feature Overview

## Symlink Management

Forge tracks files through `stage` and `link`.

```bash
forge stage ~/.vimrc
forge link ~/.vimrc
```

Use `unlink` to restore the original file while keeping the managed copy, `remove` to remove tracking and the managed copy, and `delete` to delete both original and managed files.

```bash
forge unlink --yes ~/.vimrc
forge remove --yes ~/.vimrc
forge delete --yes ~/.vimrc
```

## Directory Handling

Directory commands can be recursive or depth-limited.

```bash
forge stage --recursive ~/.config/nvim
forge stage --depth 2 ~/.config
forge unstage --recursive ~/.config/nvim
```

## Profiles

The current profile support is legacy-oriented. `new` initializes a named managed folder, while `switch` and `profile` operate on `~/.forge/profiles`.

```bash
forge new --profile work ~/dotfiles/work
forge list --profiles
forge switch work
```

## Pack-and-Go

Packs are ZIP archives with a TOML manifest and BLAKE3 hashes.

```bash
forge start packing backup
forge pack --scope backup ~/.vimrc ~/.bashrc
forge check --scope backup
forge seal --scope backup
```

Use `explain` and `--dry-run` before writing files:

```bash
forge explain backup-2026-05-30.zip
forge install backup-2026-05-30.zip --dry-run
forge restore backup-2026-05-30.zip --dry-run
```

## Not Yet Implemented

- The `-I`, `--interactive` flag is reserved for a future TUI.
- Full transaction safety and rollback support are planned but not currently implemented.
