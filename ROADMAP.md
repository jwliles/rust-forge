# Forge Roadmap

## Correctness and reliability

Address the findings in [BUGS.md](BUGS.md) in this order:

1. Prevent data loss in unlink/remove and restore originals when linking fails.
2. Reject ZIP entries and manifest payload paths that escape their intended directories.
3. Give each sealed archive a unique name and prevent overwriting existing snapshots.
4. Make purge preserve tracking after failures and restrict selection to the requested folder.
5. Make repack refresh existing manifest entries and hashes.
6. Correct parent-directory path normalization and unify tracked-file lookup.
7. Create archives atomically from content that matches the manifest hashes.
8. Return nonzero exit statuses for failed or incomplete commands.
9. Make selective and bulk unstage perform the same filesystem and database transitions.
10. Coordinate filesystem and database changes with rollback, report traversal failures, and define directory and record states consistently.
11. Add regression tests for round trips, injected failures, archive containment, path selection, and snapshot preservation; resolve test lint failures.

## Pack-and-Go

### Implemented foundation

- Deliberate pack creation with `forge start packing <scope>`.
- Manifest staging at `.forge/tmp/pack/<scope>/manifest.toml`.
- Source paths, archive-relative paths, sizes, modified times, and BLAKE3 hashes recorded by `forge pack`.
- Manifest checking with `forge check --scope <scope>`.
- Source validation and ZIP creation with `forge seal`, including the manifest.
- Active staging cleanup after a successful seal.
- Install and restore with `--force`, `--skip-existing`, and `--dry-run` options.
- Manifest membership removal with `forge unpack`.
- Archive inspection and install/restore planning with `forge explain`.

### Planned improvements

- Complete the documented repack workflow by refreshing hashes and metadata from disk.
- Detect payload and destination collisions before installation or restoration.
- Expand conflict handling for Git origins, file differences, and symlink collisions.
- Extend dry-run coverage and improve command messages and help.
- Add a command to cancel an active pack without affecting sealed archives.
- Record install receipts to support safe removal of installed files.
- Stream file hashing and produce deterministic manifest and archive ordering.

## Forge fixes

### Reverse adoption from the Forge-managed folder

Add an explicit command to create missing original-location symlinks for files
created inside the managed folder. Users should be able to add new Espanso
configuration files in the managed tree and link them back without first moving
them into the original tree and repeating `stage` and `link`.

Keep reverse adoption manual by default.

### Propagating reorganization

Support explicit propagation of managed-folder renames, moves, and directory
restructuring to original-location links and tracking records. Users should not
have to repeat the same reorganization in both trees.

Use manual reconciliation by default. An optional setting may enable automatic
propagation.

### Reconciliation between original and managed folders

Provide a reconciliation command that previews divergence and applies requested
changes between original and managed locations:

- Identify files missing from either tree and missing original-location links.
- Identify differences in directory structure and tracked mappings.
- Report conflicting states before applying changes.
- Create missing links and update mappings for approved reorganizations.
- Preserve enough state to recover from a failed apply operation.

## Change detection

Add a read-only pack diff that reports added, modified, removed, and unchanged
files using recorded hashes. Define the comparison baseline, scope membership,
and managed-folder identity before implementation.

Keep file contents, line-by-line diffs, historical reverts, and automatic packing
outside the feature's scope. See the [draft design](DESIGN.md).

## Profile and managed-folder lifecycle

- Add safe unregister operations for managed folders and profiles.
- Support profile renaming and make profile selection state explicit.
- Define reversible transitions for staging, linking, and profile changes.

## User interface and signing

- Add GUI integration with a Command Assistant and scope switcher.
- Add `forge sign` for GPG signature generation.

## Deferred

- Optional hardlinking during archive creation.
- Deduplication.
- Pack encryption with age or symmetric GPG.
- Multi-pack orchestration.
- GUI manifest editor.
