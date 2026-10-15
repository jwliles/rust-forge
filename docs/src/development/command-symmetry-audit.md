# Command Symmetry and Safety Audit

This document records a review of Forge's command surface, file operations, state
transitions, and Pack-and-Go workflow. It is intended to guide future fixes and
feature design rather than prescribe that every command must have a literal
opposite.

## Product model

Forge has two related but distinct responsibilities:

1. Manage live files through an explicit `stage` and `link` workflow.
2. Create portable, timestamped snapshots through Pack-and-Go.

The live-file workflow should be reversible wherever doing so is reasonable.
Users should be able to back out of staging, linking, profile selection, and
other non-destructive state changes without losing content.

Sealing is different. A sealed archive is a completed, immutable snapshot, not
an active pack state. `seal` intentionally has no `unseal` inverse. If the user
wants a newer snapshot, they start or populate another active pack and seal it.
Old sealed archives remain independent historical artifacts.

This also keeps Forge out of version-control territory. Forge may report changes
relative to a recorded hash, but it does not need to retain file content merely
to provide arbitrary historical reverts.

## Meanings of inverse, undo, and redo

These terms should remain separate:

- An **inverse command** performs the opposite domain action. Examples are
  `stage`/`unstage`, `link`/`unlink`, and `pack`/`unpack`.
- **Undo** restores the exact state immediately before a completed operation.
- **Redo** safely reapplies an operation that was undone.
- A **compensating action** restores consistency after an operation fails partway
  through. This is internal rollback, not necessarily a user-facing command.
- A **terminal action** deliberately produces a durable artifact or destroys
  data. `seal` is terminal but non-destructive; `delete` is terminal and
  destructive.

Naming two commands as opposites is not sufficient. Their filesystem and
database transitions must actually round-trip.

## Current command matrix

| Command | Domain inverse or successor | Assessment |
| --- | --- | --- |
| `init` | `deinit` or unregister | Missing. Should unregister safely without deleting managed content. |
| `stage` | `unstage` | Present, but selective and bulk unstage behave differently. |
| `link` | `unlink` | Present, but the current implementation is not a safe round trip. |
| `remove` | stage/adopt again | Conceptually recoverable while the original exists, but exact undo is absent. |
| `delete` | none | Intentionally destructive. Confirmation and optional trash are more appropriate than a guaranteed inverse. |
| `purge` | reinitialize/re-adopt | Destructive administrative operation; exact undo would require a retained snapshot or journal. |
| `new` | remove/unregister profile | Missing. Rename and unregister are also missing. |
| `switch` | switch to previous profile | The user can switch again, but Forge does not record or expose the previous selection. |
| `start packing` | cancel/discard active pack | Missing. |
| `pack` | `unpack` | Present for manifest membership. |
| `repack` | repack again from current files | Refresh operation. Exact undo would require retaining the prior manifest entry, not prior file content. |
| `check` | none required | Read-only. |
| `explain` | none required | Read-only. |
| `seal` | seal another snapshot | Deliberately has no inverse. The sealed archive is an immutable result. |
| `install` | uninstall from receipt | Missing. Safe uninstall requires a receipt and backups of overwritten destinations. |
| `restore` | restore another snapshot | A later snapshot can supersede it. Exact undo of overwritten files requires pre-restore backups. |
| `list` | none required | Read-only. |

## Critical correctness findings

### Unlink and remove restore through a live symlink

Both commands copy the managed file to the original path before removing the
symlink at that path. Opening the original path as a copy destination follows the
symlink, so the destination may resolve to the managed file itself. The copy can
fail as a same-file copy or affect the canonical managed file, after which the
symlink is removed without a real original file taking its place.

The safe transition is:

1. Copy the managed file to a temporary sibling of the original path.
2. Flush and validate the temporary copy.
3. Remove the original symlink.
4. Atomically rename the temporary file to the original path.
5. Update the database only after the filesystem transition succeeds.

If any step fails, Forge should retain enough information to restore the prior
linked state.

### Link is not failure-atomic

`link` copies the original into the managed folder, removes the original, and
then creates the symlink. If symlink creation fails, the original pathname has
already been removed even though the diagnostic says the original was preserved.

Forge should prepare both sides before committing the transition. A practical
approach is to copy to a temporary managed path, validate it, atomically rename
it into place, replace the original with a symlink, and roll back from the
managed copy if symlink creation fails. The database status change belongs at
the final commit point.

### ZIP extraction permits paths outside the extraction directory

Pack installation and restoration join the ZIP entry name directly onto a
temporary extraction directory. A malicious entry containing `..` or an
absolute path can escape that directory.

Every archive entry must be converted through ZIP's enclosed-path validation.
Forge should reject entries that are absolute, contain parent traversal, or
otherwise escape the extraction root. This validation must happen before any
directory or file is created.

### Operational failures often return success

Command handlers generally print an error and return `()`. Consequently, scripts
can receive exit status zero when requested work failed or only partially
succeeded. This is already acknowledged in the manual and roadmap.

Handlers should return a structured result containing successes, skips, and
failures. `main` should produce a nonzero exit status when the requested
transition does not complete, while still allowing explicitly requested partial
or best-effort modes.

## Snapshot-specific findings

### Same-day seals are not uniquely timestamped

The archive name currently uses `<scope>-YYYY-MM-DD.zip`. Sealing the same scope
more than once on the same day selects the same path, and archive creation can
replace the earlier snapshot.

This conflicts directly with the timestamped-snapshot model. Archive names
should include at least seconds and preferably UTC plus a collision-safe suffix,
for example:

```text
my_dotfiles-2026-07-30T19-42-18Z.zip
```

Archive creation should also use create-new semantics so an existing snapshot is
never silently truncated. If a collision occurs, Forge should choose a unique
suffix or fail clearly.

### Sealing correctly consumes active staging

The current design deletes the active staging directory after a successful
seal. That is consistent with sealing as finalization. It should remain clear in
the CLI and documentation that `repack` and `unpack` apply only to active,
unsealed manifests.

Failure handling should preserve the active manifest unless the archive has been
fully written, flushed, validated, and committed to its final unique name.

### Sealed snapshots should be immutable by policy

Forge should never modify an existing sealed archive in place. A new seal creates
a new archive. Archive removal, if later provided, should be an explicit cleanup
operation rather than an inverse of sealing.

## Other state and symmetry inconsistencies

### Selective unstage differs from bulk unstage

With no file arguments, `unstage` deactivates database records and removes
staging symlinks. With selected files, it deactivates records but leaves their
staging symlinks behind. Both forms should execute the same per-item transition.

### Directory link is only a database transition

Staged directories can be marked linked without an equivalent filesystem
transition. This makes the meaning of `linked` depend on whether the tracked item
is a file or directory. Directory staging should either expand into tracked file
entries or have an explicit directory state model.

### Status and active state overlap

The model contains `staged`, `linked`, and `unlinked` statuses alongside an
`active` database flag. Some reverse operations deactivate rows rather than
setting `unlinked`, making the declared state difficult to observe or reuse.

A documented state machine should define legal transitions and whether inactive
records are history, soft deletion, or reusable state.

### Path lookup is inconsistent

Some commands interpret relative input as a basename inside the managed folder,
while others normalize it relative to the current directory and search by source
or target. Nested paths and duplicate basenames can therefore select the wrong
record or no record.

All mutating commands should share one resolver that can identify a record by
exact source, exact managed target, or an unambiguous repository-relative path.
Ambiguous basenames should be rejected.

### Recursive traversal hides errors

Several directory walks discard traversal errors with `filter_map(|e| e.ok())`.
A command can report success while permissions or filesystem failures caused
files to be omitted. Traversal errors should be collected and reflected in the
result and exit status.

### Database and filesystem changes are not coordinated

Filesystem changes and SQLite changes occur independently. A failure between
them can leave stale records or untracked files. SQLite transactions alone are
not enough because SQLite cannot roll back filesystem writes; Forge needs both a
database transaction and compensating filesystem actions.

### Backups have a single `.bak` name

The helper backup path can overwrite an earlier backup and can conflict with a
real user file. Backups used for rollback should live in Forge-controlled
metadata with unique operation IDs and should be removed only after commit.

## Missing capabilities suggested by existing design documents

The root `fixes.org` identifies a broader symmetry problem: users can adopt files
from their original locations into Forge, but cannot explicitly adopt new files
created in the managed folder back into the original tree. It also identifies
renames and reorganization that occur on the managed side but are not propagated
to original locations.

A read-only `reconcile` plan followed by an explicit apply operation would cover
these cases without enabling automatic two-way synchronization:

- show managed files that lack original symlinks;
- show original files that lack managed entries;
- detect moved or renamed managed paths using stored identity or hashes;
- show conflicting changes on both sides;
- create missing symlinks only after confirmation;
- record enough of the applied plan to reverse it safely.

This aligns with Forge's preference for deliberate actions and avoids silently
choosing a side when the trees diverge.

The proposed `diff` feature also fits this model as a read-only comparison. Its
documented scope intentionally excludes line-level diffs, retained content, and
historical revert. Those exclusions should remain unless Forge's product scope
changes.

## Consolidation opportunities

### Shared operation planner

Stage, unstage, link, unlink, remove, purge, install, and restore repeat path
selection, preflight checks, copy/remove operations, confirmation, and database
updates. They should share an operation planner that produces a sequence of
typed actions before changing state.

The same plan can drive:

- validation;
- confirmation output;
- dry-run output;
- execution;
- rollback after failure;
- focused round-trip tests.

### Atomic file replacement helper

One implementation should own temporary sibling creation, copying, permission
preservation, flush, validation, atomic rename, symlink replacement, and cleanup.
Using it everywhere reduces the chance that nominally inverse commands implement
different safety rules.

### Unified path resolver

One resolver should normalize user input and return an exact tracked record or a
clear ambiguity error. Recursive selection should be implemented as a query over
tracked records rather than repeatedly rescanning all records inside directory
walks.

### Streaming hashing

Pack hashing currently reads the entire file into memory. Feeding a buffered file
stream into BLAKE3 would bound memory use for large files without changing the
manifest format.

### Deterministic manifests and archives

Pack entries are stored in a hash map, so iteration order is not stable. Sorting
entries before serialization and ZIP creation would improve reproducibility and
make snapshot comparison easier.

## Recommended implementation order

1. Fix ZIP path traversal before treating external packs as safe input.
2. Correct unlink/remove restoration order and link rollback behavior.
3. Make seal filenames unique and prevent overwriting existing snapshots.
4. Return structured command errors and meaningful exit statuses.
5. Make selective and bulk unstage use one transition implementation.
6. Define the dotfile state machine and consolidate path resolution.
7. Introduce an operation planner with compensating rollback actions.
8. Add cancel-pack, unregister/deinit, profile lifecycle, and install receipts.
9. Design explicit reconciliation for reverse adoption and managed-side moves.
10. Stream hashes and make archive ordering deterministic.

## Testing requirements

The current test suite passes, but many command tests assert only successful exit
or a fragment of output. Safety work needs assertions over the complete state:

- file contents at original and managed paths;
- symlink existence and exact destination;
- database status and active state;
- absence of temporary and backup files after commit;
- rollback after injected failure at every transition step;
- `stage -> unstage -> stage` round trips;
- `stage -> link -> unlink -> link` round trips;
- `pack -> unpack -> pack` manifest round trips;
- two seals of one scope on the same day producing two preserved archives;
- rejection of absolute and parent-traversing ZIP entries;
- nonzero exit status for partial and complete failures;
- duplicate basenames and nested directory selections.

Clippy with warnings denied currently fails on unused imports and helpers in the
test targets. These are minor but should be cleaned up so linting can serve as a
reliable CI gate.

## Decision summary

Forge should pursue symmetry for reversible live-state transitions, internal
rollback for multi-step failures, and explicit warnings for destructive terminal
actions. It should not invent an `unseal` command. Sealed packs are immutable,
timestamped snapshots; creating another snapshot is the correct successor to a
seal.
