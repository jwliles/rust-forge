---
date_created: 2025-10-03T02-46-38
date_updated: 2026-05-30
title: installation
---
# Installation

Install Forge from crates.io:

```bash
cargo install forge-rs
```

The installed binary is `forge`.

Build and install from a local checkout:

```bash
cargo install --path .
```

Verify the installation:

```bash
forge --version
forge --help
```

## Global Flags

- `-v`, `--verbose`: sets `FORGE_VERBOSE=1` for commands that emit additional diagnostic output.
- `-I`, `--interactive`: starts placeholder interactive mode when no subcommand is provided. The TUI is not implemented yet.
- `-h`, `--help`: prints help.
- `-V`, `--version`: prints version information.

Examples:

```bash
forge -v list
forge stage --help
forge install --help
```
