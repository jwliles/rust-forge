---
date_created: 2025-10-03T02-46-38
date_updated: 2026-05-30
title: project-structure
---
# Project Structure

Forge is organized as a Rust CLI with reusable modules for configuration, dotfile tracking, symlink operations, and Pack-and-Go archives.

```text
src/
├── main.rs                    # clap CLI definition and command dispatch
├── lib.rs                     # library exports for tests
├── cli/
│   ├── mod.rs
│   ├── commands.rs            # core command implementations
│   └── commands/
│       └── pack.rs            # Pack-and-Go implementation
├── config/
│   └── mod.rs                 # SQLite-backed configuration and state
├── dotfile/
│   ├── mod.rs                 # DotFile model
│   ├── backup.rs
│   ├── link.rs
│   ├── list.rs
│   └── unlink.rs
├── scanner/
│   └── mod.rs
├── symlink/
│   └── mod.rs
└── utils/
    ├── mod.rs
    ├── path_utils.rs
    └── ui.rs
```

## Documentation Files

- `README.md`: primary GitHub/crates.io overview and command reference.
- `MANUAL.md`: Markdown manual with examples and flag explanations.
- `forge.1`: roff man page installed or packaged with the binary.
- `forge.md`: Markdown copy of the primary reference.
- `docs/index.md`: MkDocs-style landing page.
- `docs/src/`: mdBook source.

## Command Surface

The authoritative CLI definitions live in `src/main.rs`. When commands or flags change, update the command reference in `README.md`, `MANUAL.md`, `forge.md`, `forge.1`, and the relevant `docs/src/` pages.
