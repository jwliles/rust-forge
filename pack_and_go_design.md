---
date_created: 2025-10-03T02-46-38
date_updated: 2025-10-03T02-46-38
timestamp: 1759459598162
title: pack_and_go_design
id: 551a2d41-9158-4ffe-a595-863123c7ca7e
hash: eea92861a01b03938a24ebc22e8ed28a17bfdcaed9306e7b8c3156fc843527f0
---
# Forge "Pack‑and‑Go" Profile Feature Expansion – Detailed Design Document

*Version: Draft 0.4  ·  Last updated: 2025‑06‑21*

> **Preface**  
> This document reflects extensive design iterations on Forge’s Pack‑and‑Go system — not a speculative idea but a near-final spec for a deliberate, user‑protective workflow. Many choices, such as requiring `start packing` and duplicating files during staging, may feel overly careful at first glance. However, each is the result of testing, edge‑case exploration, and learning from real-world mistakes.  
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
- **Reproducibility & auditability** → manifests, hashes, optional GPG.
- **User learning** → GUI/CLI parity via Command Assistant.
- **Transparent tradeoffs** → File duplication for safety, not efficiency.

---

## 1  Glossary (authoritative)

| Term                  | Meaning                                                                           | Notes                                    |
| --------------------- | --------------------------------------------------------------------------------- | ---------------------------------------- |
| **Pack**              | A named container of files the user wants to move/export.                         | Think "project" of configs.              |
| **Scope**             | The identifier of a pack (e.g. `vim_minimal`).                                    | Used in temp paths, manifests, archives. |
| **Packing**           | *Verb*: Starting a new pack. Must be done with `forge start packing`.             | Prevents accidental creation.            |
| **pack**              | *Verb*: Add one or more files to an *existing* pack.                              | `forge pack <file>`                      |
| **Repack**            | Update the staged copy of files (e.g. file modified).                             | Does **not** re‑seal.                    |
| **Seal**              | Freeze the current pack into a `.zip`. Overwrites previous archive of same scope. |                                          |
| **Install**           | Unzip + link a sealed pack on another system.                                     |                                          |
| **Unpack**            | Remove a file from a pack *or* skip it during install.                            |                                          |
| **Inventory**         | Set of all files Forge currently manages (linked or staged).                      |                                          |
| **Command Assistant** | GUI panel echoing the equivalent CLI for the last or next action.                 | Teaches the CLI passively.               |
| **Sign**              | GPG-sign a sealed pack archive to ensure authenticity.                            | Creates `.asc` signature file.           |
| **Hash**              | Generate/check per-file hashes for integrity validation.                          | Used for changes and comparisons.        |

...

