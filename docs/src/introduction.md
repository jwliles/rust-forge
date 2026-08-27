---
date_created: 2025-10-03T02-46-38
date_updated: 2026-05-30
title: introduction
---
# Introduction to Forge

Forge is a symlink management tool for dotfiles, scripts, and portable configuration bundles. It uses a deliberate workflow: initialize a managed folder, stage files, then link them.

```bash
forge init --name dotfiles --dir ~/dotfiles
forge stage ~/.vimrc ~/.config/git/config
forge link
```

## What Forge Does

- Keeps track of managed files in SQLite.
- Stages files before changing their original locations.
- Links staged files by moving managed copies into the Forge folder and creating symlinks at the original paths.
- Restores or removes tracked files through explicit commands.
- Creates portable ZIP packs with BLAKE3 hashes for backup and migration.

## Command Groups

- Repository: `init`, `list`.
- File lifecycle: `stage`, `link`, `unstage`, `unlink`, `remove`, `delete`, `purge`.
- Profiles: `new`, `switch`, `profile`.
- Packs: `start packing`, `pack`, `check`, `seal`, `explain`, `install`, `restore`, `repack`, `unpack`.

## Requirements

- Rust 1.65 or newer to build from source.
- GNU/Linux or another free operating system. Proprietary platforms are not officially tested.

## License

Forge is released under GPL-3.0-or-later.
