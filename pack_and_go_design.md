# Forge "Pack‑and‑Go" Profile Feature Expansion – Detailed Design Document

*Version: Current implementation notes  ·  Last updated: 2026-05-30*

> **Preface**  
> This document records the current Pack-and-Go implementation and the design intent behind it. Forge still requires `start packing` to make pack creation deliberate, but current staging is manifest-based: files are hashed and recorded during `forge pack`, then read from their source locations during `forge seal`.
>
> Forge treats configuration like code: we version, we validate, we audit. The manifest approach is inspired by Cargo. The deliberate actions model owes inspiration to Pijul’s patching system. And like any good system, Forge trades convenience for clarity where safety is concerned.  
>
> This document is intentionally verbose so future contributors understand not just what Forge does, but why. All technical and UX decisions documented herein are up for review — but they’re not arbitrary.

---

## 0  Background & Motivation

For years, advanced users have relied on ad‑hoc **dotfile repos**, fragile **setup scripts**, or heavy tools like **GNU Stow** to migrate personal configurations between machines. Forge already manages *linked* files (live profiles) via symlinks, but lacks a first‑class, portable **bundle** format.

**Pack‑and‑Go** solves this by introducing a deliberate staging + sealing workflow inspired by the **patch theory of Pijul**.  The design embraces:

- **Intentional actions** → you can’t accidentally create a pack.
- **Absolute path fidelity** → no fake directory trees.
- **Reproducibility & auditability** → manifests and BLAKE3 hashes.
- **User learning** → GUI/CLI parity via Command Assistant.
- **Transparent tradeoffs** → Manifest-only staging avoids temporary duplication, while `seal` validates that source files have not changed since packing.

---

## 1  Glossary (authoritative)

| Term                  | Meaning                                                                           | Notes                                    |
| --------------------- | --------------------------------------------------------------------------------- | ---------------------------------------- |
| **Pack**              | A named container of files the user wants to move/export.                         | Think "project" of configs.              |
| **Scope**             | The identifier of a pack (for example, `my_dotfiles`).                            | Used in temp paths, manifests, archives. |
| **Packing**           | *Verb*: Starting a new pack. Must be done with `forge start packing`.             | Prevents accidental creation.            |
| **pack**              | *Verb*: Add one or more files to an *existing* pack.                              | `forge pack <file>`                      |
| **Repack**            | Refresh manifest entries and hashes from current source files.                    | Does **not** re-seal.                    |
| **Seal**              | Validate the manifest and write a timestamped `.zip`.                             | Aborts if source files changed/missing.  |
| **Install**           | Extract a sealed pack to the current directory, a target directory, or mapped home paths. |                                  |
| **Restore**           | Extract a sealed pack to the original absolute paths, or use `--test` for current-directory restore. |                         |
| **Unpack**            | Remove a source path from an active pack manifest.                                |                                          |
| **Inventory**         | Set of all files Forge currently manages (linked or staged).                      |                                          |
| **Dry run**           | Preview a pack, install, or restore operation without writing files.              | `--dry-run` on supported commands.       |
| **Hash**              | BLAKE3 per-file hash stored in the manifest.                                      | Used by `check`, `seal`, install, restore. |

...
