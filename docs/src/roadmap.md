---
date_created: 2025-10-03T02-46-38
date_updated: 2026-05-30
title: roadmap
---
# Roadmap

## Current Focus

- Keep command documentation aligned with `src/main.rs`.
- Improve failure handling and exit-code consistency.
- Add transaction safety for multi-step file operations.

## Planned

- Interactive TUI for the reserved `-I`, `--interactive` mode.
- Shell completion scripts.
- More consistent rollback behavior when file operations fail partway through.
- Broader pack install/restore tests for conflict and path-mapping behavior.

## Already Implemented

- Stage/link/unlink/remove/delete lifecycle commands.
- Recursive and depth-limited staging.
- Legacy profile commands plus `new`, `switch`, and `list --profiles`.
- Pack-and-Go creation, checking, sealing, explaining, installing, restoring, repacking, and unpacking.
- BLAKE3 hash checking for packed files.
