---
date_created: 2026-05-30
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

The authoritative CLI definitions live in `src/main.rs`. The main command implementations live in `src/cli/commands.rs`, with Pack-and-Go code in `src/cli/commands/pack.rs`.
