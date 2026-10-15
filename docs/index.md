---
date_created: 2025-10-03T02-46-38
date_updated: 2026-05-30
title: index
---
# Forge

Forge manages symlinked files with an explicit workflow:

```bash
forge init --name dotfiles
forge stage ~/.vimrc ~/.bashrc
forge link
forge list
```

It also creates portable configuration packs:

```bash
forge start packing my_dotfiles
forge pack --scope my_dotfiles ~/.vimrc ~/.bashrc
forge seal --scope my_dotfiles
forge explain .forge/archives/my_dotfiles-2026-05-30.zip
```

## Current Features

- Stage, link, unlink, remove, delete, unstage, and purge tracked files.
- Recursive and depth-limited directory staging.
- Legacy profile listing and switching.
- Pack-and-Go archives with BLAKE3 integrity checks.
- Install, restore, explain, repack, unpack, and check pack workflows.
- SQLite-backed tracking.

Interactive mode is reserved by `-I` but the TUI is not implemented yet. Transaction safety is planned, not currently guaranteed.

Start with [Installation](getting-started/installation.md). The full mdBook source also includes quick-start and basic-usage guides under `docs/src/getting-started/`.
