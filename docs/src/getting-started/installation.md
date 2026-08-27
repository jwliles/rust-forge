---
date_created: 2025-10-03T02-46-38
date_updated: 2026-05-30
title: installation
---
# Installation

Install the published crate with Cargo:

```bash
cargo install forge-rs
```

The binary name is `forge`.

Build from a local checkout:

```bash
cargo build
cargo test
```

Print the installed version:

```bash
forge --version
```

## Global Flags

- `-v`, `--verbose`: sets `FORGE_VERBOSE=1` for commands that emit additional diagnostic output.
- `-I`, `--interactive`: starts placeholder interactive mode when no subcommand is provided. The TUI is not implemented yet.
- `-h`, `--help`: prints help.
- `-V`, `--version`: prints version information.

Examples:

```bash
forge --help
forge stage --help
forge -v list
```
