---
date_created: 2025-10-03T02-46-38
date_updated: 2026-05-30
timestamp: 1759459598162
title: pack_and_go_timeline
id: 71e34789-3659-4b83-b124-7f9f5039b274
hash: 68872dc9c2de74dc6ff7c112c68ee338ef0e48cfee5041bb805601cf15ef4bf0
---
## Forge Pack-and-Go – Development Timeline

### Overview
This timeline records the implemented Pack-and-Go path and remaining follow-up work. The current implementation uses manifest-only staging: `forge pack` records source paths, relative archive paths, metadata, and BLAKE3 hashes; `forge seal` validates source files and creates the ZIP archive.

---

### ✅ Phase 1: Foundation & Staging (Week 1)
**Goal:** Create and populate pack staging areas with intent and precision.

- Implement `forge start packing <scope>`
- Create `.forge/tmp/pack/<scope>/manifest.toml`
- Implement `forge pack <file>` with source-path recording and BLAKE3 hashing
- Populate manifest entries with target path, relative archive path, size, modified time, and hash

**Note:** Intentional command friction (`start packing`) prevents misuse.

---

### ✅ Phase 2: Sealing & Cleanup (Week 2)
**Goal:** Let users convert staged packs into portable archives.

- Implement `forge seal`
  - Validate manifest hashes against source files
  - ZIP source files as `.forge/archives/<scope>-YYYY-MM-DD.zip`
  - Include `manifest.toml` in the archive
- Delete staging folder (`.forge/tmp/pack/<scope>`) after sealing

**Note:** Sealing removes the active staging directory. Repacking applies to active, unsealed pack manifests.

---

### ✅ Phase 3: Installation (Week 3)
**Goal:** Allow users to extract and use packs on other systems.

- Implement `forge install <archive.zip>`
  - Extract archive
  - Read `manifest.toml`
  - Copy files to calculated target paths
- Conflict handling with `--force`, `--skip-existing`, and `--dry-run`

**Note:** First major cross-system usage milestone.

---

### ✅ Phase 4: Hashing & Validation (Week 4)
**Goal:** Ensure integrity and support future reproducibility.

- Add BLAKE3 hashes to `manifest.toml`
- Implement `forge check --scope <scope>`
- Validate hashes during seal, install, and restore
- Detect and report mismatches

**Note:** Inspired by `gen_readme`, `cargo`, and Vim's `:checkhealth`.

---

### ✅ Phase 5: Repack, Unpack, UX (Week 5)
**Goal:** Improve staging workflow and CLI usability.

- Implement `forge repack --scope <scope>` (refresh manifest entries from disk)
- Implement `forge unpack --scope <scope> <file>` (remove file from pack manifest)
- Add explain/install/restore planning output
- Polish messages, logs, and CLI help output

**Note:** This phase completes the loop of pack creation, revision, and finalization.

---

### ✅ Phase 6: Robustness & Extras (Week 6+)
**Goal:** Stretch goals and production hardening.

- More consistent nonzero exit behavior for recoverable command failures
- Rich conflict handling (`git_origin`, file diff, symlink collision)
- Additional `--dry-run` coverage where useful
- GUI integration (Command Assistant, scope switcher)
- Support for `forge sign` (GPG signature generation)

**Note:** Prepare Forge for GUI-first users and broader adoption.

---

### 🚧 Dev Risk "Parking Lot"
These features are valuable but deferred until core implementation is stable:

- Optional hardlinking during archive creation (optimization)
- Deduplication support
- Pack encryption (`age`, GPG symmetric)
- Multi-pack orchestration
- GUI manifest editor

---

### 📦 Summary Table
| Week | Focus                         | Key Deliverables                     |
|------|-------------------------------|--------------------------------------|
| 1    | Start packing & manifest staging | `start packing`, `pack`, manifest |
| 2    | Sealing & cleanup            | `seal`, archive structure, purge tmp |
| 3    | Install/restore               | `install`, `restore`, conflict flags |
| 4    | Integrity                     | `check`, manifest validation         |
| 5    | UX & control                  | `repack`, `unpack`, `explain`, polish |
| 6+   | Extras                        | Signing, GUI, robustness             |
