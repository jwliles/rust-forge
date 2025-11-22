---
date_created: 2025-10-03T02-46-38
date_updated: 2025-10-03T02-46-38
timestamp: 1759459598162
title: pack_and_go_timeline
id: 71e34789-3659-4b83-b124-7f9f5039b274
hash: 68872dc9c2de74dc6ff7c112c68ee338ef0e48cfee5041bb805601cf15ef4bf0
---
## Forge Pack-and-Go – Development Timeline

### Overview
This timeline synthesizes internal discussions and external feedback to propose a realistic 6-week development plan for the Pack-and-Go feature in Forge. The goal is to balance minimal viable functionality, extensibility, and safety, while respecting the tool's deliberate UX philosophy.

---

### ✅ Phase 1: Foundation & Staging (Week 1)
**Goal:** Create and populate pack staging areas with intent and precision.

- Implement `forge start packing <scope>`
- Create `.forge/tmp/pack/<scope>/files/` and `manifest.toml`
- Implement `forge pack <file>` with absolute → relative copying
- Populate manifest entries with target path and metadata

**Note:** Intentional command friction (`start packing`) prevents misuse.

---

### ✅ Phase 2: Sealing & Cleanup (Week 2)
**Goal:** Let users convert staged packs into portable archives.

- Implement `forge seal`
  - ZIP up staged pack as `.forge/archives/<scope>-YYYY-MM-DD.zip`
  - Include `manifest.toml` in the archive
- Delete staging folder (`.forge/tmp/pack/<scope>`) after sealing

**Note:** Sealing is final. Repacking = new staging session.

---

### ✅ Phase 3: Installation (Week 3)
**Goal:** Allow users to extract and use packs on other systems.

- Implement `forge install <archive.zip>`
  - Extract archive
  - Read `manifest.toml`
  - Symlink files to target paths (with absolute path safety)
- Basic conflict detection: file already exists → skip with warning

**Note:** First major cross-system usage milestone.

---

### ✅ Phase 4: Hashing & Validation (Week 4)
**Goal:** Ensure integrity and support future reproducibility.

- Implement `forge hash` (BLAKE3-based) for files in staging
- Add hashes to `manifest.toml`
- Validate hashes during install
- Detect and report mismatches (non-blocking)

**Note:** Inspired by `gen_readme`, `cargo`, and Vim's `:checkhealth`.

---

### ✅ Phase 5: Repack, Unpack, UX (Week 5)
**Goal:** Improve staging workflow and CLI usability.

- Implement `forge repack <scope>` (overwrite staged files from disk)
- Implement `forge unpack <file>` (remove file from pack)
- Add `forge ?` or `forge status` to check CWD scope
- Polish messages, logs, and CLI help output

**Note:** This phase completes the loop of pack creation, revision, and finalization.

---

### ✅ Phase 6: Robustness & Extras (Week 6+)
**Goal:** Stretch goals and production hardening.

- Manifest validation during install
- Rich conflict handling (`git_origin`, file diff, symlink collision)
- `--dry-run` for install and seal
- GUI integration (Command Assistant, scope switcher)
- Support for `forge sign` (GPG signature generation)

**Note:** Prepare Forge for GUI-first users and broader adoption.

---

### 🚧 Dev Risk "Parking Lot"
These features are valuable but deferred until core implementation is stable:

- Hardlinking instead of copying (optimization)
- Deduplication support
- Pack encryption (`age`, GPG symmetric)
- Multi-pack orchestration
- GUI manifest editor

---

### 📦 Summary Table
| Week | Focus                         | Key Deliverables                     |
|------|-------------------------------|--------------------------------------|
| 1    | Start packing & file staging | `start packing`, `pack`, manifest    |
| 2    | Sealing & cleanup            | `seal`, archive structure, purge tmp |
| 3    | Install                       | `install`, symlink handling          |
| 4    | Integrity                     | `hash`, manifest validation          |
| 5    | UX & control                  | `repack`, `unpack`, `?`, polish      |
| 6+   | Extras                        | Signing, GUI, robustness             |

