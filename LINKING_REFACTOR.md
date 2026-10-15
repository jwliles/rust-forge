# Forge Linking Refactor

Status: Design proposal based on the linking-workflow discussion.

## Purpose

Forge consolidates files in a managed directory and makes them available at their
original locations through symlinks. It manages arbitrary files, not only dotfiles.
Its first responsibility is to link and unlink those files reliably.

Profiles extend that workflow by providing independent configurations for
experimentation. Pack-and-Go captures a selected setup for installation on another
machine and optional provenance. Diff provides read-only inspection. All three
rely on correct file selection, stable mappings, and safe filesystem transitions.

This refactor establishes that foundation. It should improve performance and
quality by correcting the underlying state model and consolidating repeated
selection and execution logic. It is not a parser rewrite or an expansion into
content version control.

See [BUGS.md](BUGS.md) for the current audit and
[the previous command audit](docs/src/development/command-symmetry-audit.md) for
additional failure analysis. This document defines proposed behavior; it does not
describe commands already implemented.

## Requirements

- Keep the CLI small, clearly named, and reasonably scoped.
- Separate selecting files from executing a filesystem transition.
- Recognize existing tracking when files and parent folders overlap.
- Support recursive selection with optional depth limits and multiple targets.
- Never lose file contents when linking, unlinking, or recovering from failure.
- Make preparation and execution undoable and redoable.
- Allow unrelated files and independent operations to be reversed independently.
- Keep routine output concise and expose details through verbosity and preview.
- Fix causes of inconsistent state rather than concealing them with successful
  messages, silent skips, or additional command ceremony.

## Command model

The core workflow has four commands:

| Command | Responsibility |
| --- | --- |
| `forge gather <targets>` | Add explicitly selected files and directory selections to the pending selection. |
| `forge release [targets]` | Remove targets from the pending selection; omit targets to release everything gathered. |
| `forge link` | Bring the gathered selection under management and establish its original-location links. |
| `forge unlink` | Release the gathered selection from active management and restore usable ordinary files at original locations. |

`gather` and `release` change selection only. They do not move content, create
staging symlinks, or change active tracking. A gathered file is not yet linked.
The pending selection persists between command invocations.

Repeated gathers are additive. Overlapping selections must not cause duplicate
execution. Releasing files changes the pending selection, not their linked state.
It can therefore release linked or unlinked candidates without modifying either.

There is no separate `stage` and `add` operation in this model. A linked file's
content edits propagate through its symlink naturally; it does not need to be
added again as though Forge maintained Git's content index.

Example preparation and execution:

```sh
forge gather a/a a/b/b
forge link
```

Example cancellation of part or all of a selection:

```sh
forge release a/a
forge release
```

To unlink files, gather the desired targets and execute `forge unlink`. The same
selection rules apply in both directions. Selection lifetime after successful or
failed execution is an explicit decision listed below, not an implicit behavior
of these examples.

## Target syntax and traversal boundaries

A trailing slash expresses directory intent:

| Target | Meaning |
| --- | --- |
| `a/` | Select directory `a` recursively, optionally limited by depth. |
| `a/a` | Select that file only. |
| A directory without its trailing slash | Invalid target; explain the required form. |
| A file supplied with a trailing slash | Invalid target; explain the type mismatch. |

Preserve the user's trailing-slash intent before path normalization removes it.
Do not guess a different target type or silently widen a request.

From one working directory, the user can select `a/a` and `a/b/b` without changing
directories and without adopting the rest of `a`. A request for `a/` instead
expresses directory selection. Explicit file selection and whole-directory
tracking must remain distinguishable.

The CWD at gather time sets the upper boundary for discovering files to adopt.
Forge must not add files above that boundary or widen traversal to unrelated
siblings. Store that boundary with the request; changing CWD between `gather` and
`link` must not reinterpret the selection.

Containment checks must handle parent components and symlink resolution without
losing the identity of the original pathname. The configured managed destination
can be outside this source-selection boundary: it is a known destination, not a
reason to discover additional source files there. Handling of symlinks that point
outside the selection boundary needs an explicit policy before implementation.

### Recursion and targeted depth

Support unlimited recursion and an optional depth limit for directory targets.
Support multiple explicitly named targets with different limits, so users can
select particular branches without traversing every directory in a project.
Depth has no recursive meaning for an explicit file target.

RPT's targeted-depth idea is a useful reference, not a requirement to copy its
syntax or parser. The final syntax and depth numbering remain undecided. Whatever
syntax is chosen must produce an explicit association between each target and its
limit. Reject malformed values, mismatched target/depth lists, and ambiguous
associations before changing the pending selection.

Use one selection implementation for gather, targeted release, preview, link,
and unlink. Files already represented by tracking records must remain addressable
when their original path is missing or its link is broken; a fresh walk of existing
files alone cannot supply the full selection.

## Selection identity and folder membership

Maintain one active original-to-managed mapping per managed-folder/profile scope.
Use recorded identity and normalized paths to recognize the same tracked item.
Identical hashes or basenames alone do not establish identity.

Record both directory intent and its individual member mappings. Tracking an
entire folder is different from independently tracking some of its files. This
information supports later discovery of new members, reverse adoption from the
managed tree, and meaningful removal from a tracked folder.

Store the actual selected members as well as the request that selected them.
Preview and execution operate on that recorded selection, with current-state
validation. They must not silently adopt files created after gathering. Newly
observed files can be reported for a subsequent gather.

Releasing a file selected through a gathered parent must actually remove it from
the pending selection. Overlapping requests must not make a released file
impossible to deselect. Persist enough selection information to undo that release
exactly. The precedence of a later gather over a prior exclusion must be defined
and tested.

### Overlapping file and folder selections

| Situation | Required result |
| --- | --- |
| Gather a file twice | One effective selection, no second content copy or active mapping. |
| Gather a file, then its parent folder | Incorporate the file into the folder selection. |
| Gather a folder, then one of its files | Recognize existing membership; do not execute twice. |
| Link a file, then gather and link its parent | Incorporate the existing mapping into the intended folder layout safely. |
| Unlink a parent containing individually linked children | Resolve its tracked descendants and unlink each selected item once. |
| Select two distinct files with the same basename | Preserve their identity and structure, or report a destination conflict before execution. |

For example, adding a parent folder must not leave both `managed/file.conf` and
`managed/config/file.conf` as active copies of one original file. If incorporation
requires relocation, plan the managed-file move, original-link update, and database
change together. Existing correct links are recognized; unrelated files or
unexpected symlinks are conflicts rather than generic "already exists" skips.

Managed-path layout and promotion rules need an explicit policy. Results must not
arbitrarily depend on which database row or filesystem entry is processed last.

## Linking, unlinking, and file preservation

Linking establishes the managed content and an original-location symlink to it.
Validate the selected item's current state, both destinations, and any existing
mapping before committing a transition.

Unlinking restores an ordinary file at the original location and ends the active
management relationship for that selection. A file must not disappear, become
empty, or remain accessible only through a broken symlink. This behavior is
separate from releasing a pending selection.

Do not copy through an original-location symlink as though it were an independent
copy destination. Prepare a validated temporary sibling and replace the link
safely. Do not remove the only usable original before recovery information and a
validated managed copy exist.

Prepare a complete operation plan before execution. Preflight conflicts must not
leave half a batch changed. Filesystem transitions and SQLite updates need
coordinated execution with compensating actions; an SQLite transaction alone
cannot reverse filesystem writes.

Interrupted execution must retain durable progress and recovery information.
Rollback failure is a distinct failure with an accurate description of remaining
files and tracking. Commands must return nonzero for failed or incomplete work.

Unlinking must also preserve edits made during normal use. If an undo or redo
would overwrite intervening content, detect and report that conflict rather than
restoring an older copy silently. The disposition and retention of managed copies
and recovery data after unlinking remain policy decisions.

Removing one file from a fully tracked folder must be representable without
causing later reconciliation to adopt it again automatically. Persistent tracking
exclusions and temporary selection exclusions are different kinds of state.

## Undo, redo, and independent changes

Reversibility covers preparation as well as execution:

- Undo gather restores the exact prior selection.
- Undo release restores exactly the selection removed by that operation.
- Undo link or unlink reverses the recorded file, link, and tracking changes.
- Redo reapplies the recorded change, subject to current-state validation.

An inverse command is not a substitute for undo. `release` makes a new selection
change; undoing a release restores the actual prior selection without requiring
the user to reconstruct it. Likewise, `unlink` is a new operation with defined
semantics, not automatically an exact undo of an earlier link.

For example, after gathering 22 files through several requests, an accidental
release of eight files should be reversed by one undo. Redo reapplies that exact
release. Neither action should rescan the folder and select a different set.

The order files were adopted must not dictate the order they can leave management.
If 22 files were linked individually, file 7 can be unlinked without removing and
re-adding files 8 through 22. Targeted undo should likewise reverse an independent
change without discarding unrelated later work.

Use a dependency model inspired by Pijul's change commutation: independent
operations can be applied or reversed independently, while dependent operations
require coordinated treatment. Shared parent directories, managed destinations,
profile state, folder promotion, and tracking exclusions can introduce dependencies
even when two filenames differ. Do not automatically reverse dependent work without
making its extent explicit.

Use Vim's undo-tree behavior as a reference for retaining alternate operation
paths after undo followed by new work. A chronological tree alone is insufficient
for independent reversal; dependency relationships and the set of applied changes
must also be represented.

### SQLite operation journal

Persist structured changes rather than shell command strings. A proposed journal
contains:

| Record | Required information |
| --- | --- |
| Operation | Stable ID, context, kind, ordering information, outcome, and progress. |
| Dependency | Relationships that constrain independent application or reversal. |
| Change | Ordered before/after selection, path, symlink, and tracking information. |
| Recovery data | Retained contents or durable references when reversal needs them. |
| Navigation state | Applied operations, undo/redo position, and alternate branch information. |

A batch execution is one user-visible operation with individually recorded changes.
Keep the record before performing destructive transitions, record execution progress,
and mark completion only after filesystem and database state agree. Concurrent
Forge processes must not operate on stale plans or advance the same journal state
without coordination.

This journal records Forge's actions. It does not monitor and version every content
edit made by an editor or application. Content storage should serve recovery of
Forge operations rather than become general-purpose content history.

The user's general Principle of Reversible Actions motivates this behavior but is
not a Forge specification. Branch retention, targeted reversal, and filesystem
recovery are requirements of this design discussion. Pack finalization is the
explicit exception discussed below.

## Inspection and output

| Command or option | Scope |
| --- | --- |
| `status [targets]` | Show tracked state and gathered selection, with concise conflict and missing-link information. |
| `check [targets]` | Validate links and tracking consistency; report empty files and directories. |
| `undo` / `redo` | Reverse and reapply recorded changes. Target and branch-selection syntax is undecided. |
| `-p`, `--preview` | Proposed short preview option for execution; existing `--dry-run` can remain an alias. |
| `--verbose` | Show per-item decisions and diagnostic execution detail. |

Empty files and directories are findings, not inherently invalid state. `check`
should distinguish them from broken links and tracking inconsistencies.

A public `history` command is not part of the initial agreed command set. The
internal journal is necessary for undo/redo regardless of whether history or tree
inspection later warrants a dedicated interface. Shell history cannot substitute
for records of the changes that actually succeeded.

Normal output should report results, not narrate copying, deleting, and SQL updates.
For example:

```text
Gathered 8 files; 2 already selected.
Linked 8 files.
Released 3 files from selection.
```

Failures should identify the affected paths, the reason, and the resulting state.
Preview must not change files, gathered selection, active tracking, or undo/redo
position. Display the planned actions, including relocations and conflicts.

## Boundaries with other features

### Profiles

Profiles support branching an active configuration for experimentation, originally
motivated by Emacs and Vim configurations. A fork must provide independent contents;
edits to the experimental profile must not modify its parent through shared targets.
The linking core must support scoped mappings and reversible switching without
requiring the profile redesign to be completed in this refactor.

### Pack-and-Go

Packs support quick setup on another machine and optional provenance. A finalized
pack captures its included files and manifest. It is not edited through Diff or
through the linking undo journal.

An omission or desired change is handled by creating another complete pack.
Enumeration and explicit replacement of the newest or a named pack are valid
workflows. Replacement must build and validate the new artifact before replacing
the old one; silent filename-collision overwrite is a bug.

Pack finalization is an explicit limit on undo/redo. The precise boundary—pack
artifact operations, an associated workflow checkpoint, or both—must be settled
before implementing it. Do not infer that sealing clears unrelated live-file undo
history. Existing `pack` and `seal` terminology must clearly identify the finalizing
step and disclose its consequences before execution.

### Diff

Diff remains read-only. It may inspect divergence between original and managed
locations, current files against a pack, and other comparisons justified by the
workflow. Comparison does not authorize editing, merging, patch application, or
sealed-pack mutation. Reconciliation changes belong to explicit management actions.

### CLIP and RPT

RPT demonstrates selective traversal and different depths per target. Its partial
CLIP implementation is not yet a reusable library. Forge can implement the agreed
selection model with its current parser; adopting CLIP requires an independent,
concrete justification. Do not make a parser extraction a prerequisite for safe
linking.

## Implementation sequence

1. Specify the state transitions, selection identity, mapping rules, and unresolved
   policies below. Capture the failing overlap and file-preservation cases in tests.
2. Introduce a shared bounded selector and pending-selection storage behind gather
   and release. Keep preparation free of filesystem staging artifacts.
3. Implement the operation journal and dependency representation alongside safe
   link/unlink execution, rollback, and restart recovery.
4. Implement exact selection undo/redo and execution undo/redo, including independent
   reversal and intervening-change checks.
5. Add status, check, preview, and concise output over the same selection and planning
   logic. Complete compatibility migration before making the new workflow the default.
6. Measure traversal, hashing, copying, and database access. Reuse operation-scoped
   connections and prepared statements, avoid repeated full-table scans, and eliminate
   redundant copies without weakening validation or recovery.

No phase should advertise supported mutation paths before their preservation and
recovery guarantees are tested. Urgent existing data-loss defects can be fixed
before the full command migration; they must not wait for a broad rewrite.

## Acceptance tests

Verify contents, path types, exact symlink destinations, active mappings, pending
selection, journal position, and recovery objects—not just exit status or output.

- Gather a file, then its parent directory; repeat in the opposite order.
- Link a file, then incorporate its parent; unlink the parent and verify every member.
- Repeat gather/link without duplicate records, copies, or unnecessary transitions.
- Select `a/a` and `a/b/b` without selecting siblings.
- Exercise full recursion, every supported depth boundary, per-target depths,
  exclusions, malformed input, and trailing-slash type mismatches.
- Reject adoption outside the gather-time CWD boundary, including path escapes;
  changing CWD afterward must not change selection meaning.
- Select records with missing originals, broken links, duplicate basenames, and
  managed-side aliases without confusing their identities.
- Detect conflicting files, destinations, overlapping roots, and unexpected symlinks
  before a batch changes files.
- Release a parent-selected child, release everything, and undo/redo each operation
  exactly; newly created files must not appear during replay.
- Undo a gather that overlapped an earlier gather without removing the earlier
  selection; undo a release without reconstructing requests manually.
- Link 22 files individually and independently unlink or reverse file 7.
- Preserve alternate redo branches and enforce actual dependencies during targeted undo.
- Preserve edits made after linking; reject undo/redo that would silently overwrite them.
- Inject filesystem and database failures at transition boundaries and restart after
  interruption. Verify rollback and incomplete-operation reporting.
- Handle concurrent operations without duplicate mappings or inconsistent journal heads.
- Remove a tracked-folder member without later automatic re-adoption.
- Ensure preview is read-only and ordinary output remains concise.
- Open existing databases and migrate mappings without discarding files or tracking.

## Decisions still required

1. Whether successful execution clears the gathered selection, and what remains
   selected after a failed operation or an operation containing inapplicable items.
2. Depth syntax, depth numbering, per-target association, and name/path matching rules.
3. Mapping layout when an individual file is incorporated into a tracked parent.
4. How selection boundaries apply when gathering from the managed folder using known
   original mappings, and how external, directory, or pre-existing symlinks are handled.
5. Whether gathered selections are per managed folder, profile, working context,
   or session; incompatible contexts must not be combined implicitly.
6. Managed-copy disposition after unlink, recovery retention limits, and cleanup rules.
7. Selection-exclusion precedence and persistent exclusions from tracked folders.
8. Targeted undo and branch-navigation syntax, dependency handling, and treatment of
   selection history versus executed-file history.
9. The exact pack finalization boundary for reversibility.
10. Migration of `stage`, `unstage`, `link`, `unlink`, `remove`, and `purge`; any aliases
    must preserve or explicitly reject old semantics rather than silently change them.
11. How live-state `check` and the current pack-manifest `check` are distinguished.
