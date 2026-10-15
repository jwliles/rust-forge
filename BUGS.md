# Forge Bugs and Audit

## Current audit — October 14, 2026

Reviewed the working tree based on commit `0fc3e41`, after root documentation
consolidation. This is a focused correctness and workflow audit of live-file
commands, Pack-and-Go, path handling, database transitions, and test coverage;
it is not an exhaustive security review or a verification of every historical fix.
No production code was changed during this audit.

The previous [Command Symmetry and Safety Audit](docs/src/development/command-symmetry-audit.md)
provides the detailed rationale, command matrix, and recommended implementation
order. Its [critical findings](docs/src/development/command-symmetry-audit.md#critical-correctness-findings),
[snapshot findings](docs/src/development/command-symmetry-audit.md#snapshot-specific-findings),
and [state inconsistencies](docs/src/development/command-symmetry-audit.md#other-state-and-symmetry-inconsistencies)
remain useful starting points. The current findings below supersede its status
claims where noted. That document's reference to `fixes.org` now points to the
reconciliation requests preserved in [ROADMAP.md](ROADMAP.md#forge-fixes).

### Dependency upgrade follow-up

The dependency upgrade addresses B02 by rejecting absolute and parent-traversing
ZIP entry paths before writing them. Install and restore also reject unsafe
manifest payload paths from B10. Payload and destination collision detection in
B10 remains outstanding. The table below preserves the original audit findings.

### Confirmed outstanding work

“Reproduced” means observed using the built CLI with isolated SQLite/config paths
and disposable files under `/tmp`. “Code review” means the implementation was
inspected but the failure was not injected or reproduced during this audit.

| ID  | Priority | Finding and effect                                                                                                                                                                                                                                                                      | Evidence                                                                                                                                             | Verification                                                                        |
| --- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| B01 | Critical | `unlink` copies through the original symlink, then removes that path. A normal link/unlink cycle lost the original pathname and emptied the managed content. `remove` uses the same unsafe ordering and subsequently deletes the managed copy.                                          | [unlink](src/cli/commands.rs#L732), [remove](src/cli/commands.rs#L832)                                                                               | Unlink reproduced; remove code review.                                              |
| B02 | Critical | ZIP extraction joins unvalidated entry names to its temporary directory. An absolute entry wrote outside extraction during `install --dry-run`, before manifest validation.                                                                                                             | [extract_zip_archive](src/cli/commands/pack.rs#L940)                                                                                                 | Reproduced with a marker file confined to the audit's temporary directory.          |
| B03 | High     | `link` removes the original before creating its symlink; symlink failure has no restoration despite the message claiming preservation. Existing managed regular files are removed as though they were staging symlinks.                                                                 | [target removal](src/cli/commands.rs#L474), [original removal and symlink creation](src/cli/commands.rs#L558)                                        | Code review.                                                                        |
| B04 | High     | Same-scope seals on the same day select the same archive name, and `File::create` truncates the previous snapshot.                                                                                                                                                                      | [archive naming](src/cli/commands/pack.rs#L489), [archive creation](src/cli/commands/pack.rs#L510)                                                   | Two successful seals left one archive.                                              |
| B05 | High     | Purge deletes all matching database records even after individual restoration or removal failures. It skips managed dotfiles but still removes their records.                                                                                                                           | [purge_command_safe](src/cli/commands.rs#L1225)                                                                                                      | Code review.                                                                        |
| B06 | High     | Purge selection and deletion use different path boundaries. Deletion matches `folder%`, including sibling names such as `managed-other`; SQL `%` and `_` in real paths are also treated as wildcards. Nonrecursive queries match the folder itself rather than immediate child records. | [deletion query](src/config/mod.rs#L897), [selection query](src/config/mod.rs#L944)                                                                  | Sibling record deletion reproduced; wildcard and nonrecursive behavior code review. |
| B07 | High     | `repack` delegates to the add-only pack implementation, which skips existing entries. Modified files keep stale hashes and remain unsealable.                                                                                                                                           | [existing-entry skip](src/cli/commands/pack.rs#L302), [repack](src/cli/commands/pack.rs#L991)                                                        | Reproduced; `check` still reported MODIFIED after repack.                           |
| B08 | High     | Relative paths are cleaned before joining the current directory. Leading `..` is discarded: `../file` resolves inside the current directory instead of its parent. Absolute traversal above root can also pop the root component.                                                       | [normalize and clean_path_components](src/utils/path_utils.rs#L35)                                                                                   | Code review.                                                                        |
| B09 | High     | Seal hashes sources before reading them again for ZIP output. A source change between validation and archive writing can produce contents inconsistent with the embedded hash. Output is written directly to the final archive path without atomic publication.                         | [validation](src/cli/commands/pack.rs#L449), [source reread](src/cli/commands/pack.rs#L521)                                                          | Code review; concurrent modification not injected.                                  |
| B10 | High     | Manifest `relative_path` is joined without containment validation; separate source files can also share the same archive-relative filename. Install/restore need validated, unique payload paths and preflight destination collision checks.                                            | [manifest entry](src/cli/commands/pack.rs#L39), [payload lookup](src/cli/commands/pack.rs#L671)                                                      | Code review.                                                                        |
| B11 | Medium   | Most command handlers print errors and return normally, or skip hash failures/conflicts while reporting successful totals. Seal now exits 1 for implementation failures, so the previous audit's exit-status finding is partially resolved.                                             | [pack error handling](src/cli/commands/pack.rs#L167), [install loop](src/cli/commands/pack.rs#L671), [seal exception](src/cli/commands/pack.rs#L414) | Missing pack reported an error with exit 0; missing seal exited 1.                  |
| B12 | Medium   | Selective/recursive unstage deactivates rows without removing staging symlinks; bulk unstage deactivates first, then attempts filesystem cleanup. Directory staging produces directories that bulk `remove_file` cannot remove.                                                         | [unstage_command](src/cli/commands.rs#L1147)                                                                                                         | Selective symlink leak reproduced; remaining paths code review.                     |
| B13 | Medium   | Directory link marks a record linked without moving the original or creating its final symlink.                                                                                                                                                                                         | [directory branch](src/cli/commands.rs#L454)                                                                                                         | Code review.                                                                        |
| B14 | Medium   | Commands disagree on path resolution; unlink/remove reduce relative paths to basenames and search by managed target, while link/unstage accept source paths differently.                                                                                                                | [unlink resolution](src/cli/commands.rs#L667), [remove resolution](src/cli/commands.rs#L794)                                                         | Code review.                                                                        |
| B15 | Medium   | Filesystem writes and database updates lack compensating rollback. Stage can leave links after database failure; link counts success even if the database update fails. Status and active flags still have overlapping meanings.                                                        | [stage](src/cli/commands.rs#L287), [link database update](src/cli/commands.rs#L591), [deactivation](src/config/mod.rs#L655)                          | Code review.                                                                        |
| B16 | Medium   | Recursive walks discard errors, permitting incomplete operations to look successful.                                                                                                                                                                                                    | [stage walk](src/cli/commands.rs#L156), [pack walk](src/cli/commands/pack.rs#L255), [unstage walk](src/cli/commands.rs#L1188)                        | Code review.                                                                        |
| B17 | Low      | Backup names use a single `.bak` path; hashing loads entire files into memory; hash-map iteration makes manifest/archive ordering unstable.                                                                                                                                             | [backup](src/dotfile/backup.rs#L6), [hashing](src/cli/commands/pack.rs#L347), [archive iteration](src/cli/commands/pack.rs#L522)                     | Code review.                                                                        |

### Validation and coverage gaps

- `cargo test --all-features`: passed, 65 tests. The suite does not establish the
  safety properties above; the isolated probes exposed failures despite this pass.
- `cargo clippy --all-features --all-targets -- -D warnings`: failed on existing
  unused imports and unused test helpers in `tests/core_commands.rs`,
  `tests/pack_and_go.rs`, and `tests/common/mod.rs`.
- Isolated CLI probes confirmed B01, B02, B04, B06, B07, B11, and B12. Probe
  files and databases were temporary; the user's managed files were not used.
- Required regression coverage: contents and symlink destinations after complete
  round trips; SQLite state after each operation; injected filesystem/database
  failures; sibling folder isolation; leading-parent path resolution; archive
  traversal rejection; repack hash refresh; multiple preserved same-day seals;
  payload/destination collisions; and nonzero exits for incomplete operations.

### Priorities and design work

Fix B01–B06 first because they affect file safety, archive containment, and
tracking integrity. Then fix repack and path normalization, publish seals
atomically from validated content, and standardize failure reporting. Shared
path resolution and coordinated filesystem/database transitions should support
these fixes, rather than requiring a broad rewrite before any fix can land.

The previous audit's [missing capabilities](docs/src/development/command-symmetry-audit.md#missing-capabilities-suggested-by-existing-design-documents)
and [command matrix](docs/src/development/command-symmetry-audit.md#current-command-matrix)
remain design backlog: cancel an active pack, unregister managed folders and
profiles safely, record install receipts, and explicitly reconcile reverse
adoption and managed-side reorganization. These are feature gaps, not all
correctness bugs. See [ROADMAP.md](ROADMAP.md) and the draft diff proposal in
[DESIGN.md](DESIGN.md). Pack-baseline diff still needs an explicit baseline and
scope definition; it should not be conflated with live-folder reconciliation.

### Historical claims reassessed

The UI flush helper now handles flush errors without `unwrap()`, as the old
report claimed. However, its blanket statement that all critical bugs were fixed
is contradicted by the current findings. The old symlink example does not
describe the current implementation's failure behavior. The historical memory
leak claim and commit attribution were not independently re-audited.
