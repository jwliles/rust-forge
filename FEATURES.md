# Forge Features and Open Design Work

Status: Product design and implementation priorities.

## Product scope

Forge keeps files together in a managed directory while making them usable at their
original locations through symlinks. It manages arbitrary files, not only dotfiles.
Its extensions address configuration experimentation, machine setup, and visibility
into changes.

| Feature | Purpose | Current position | Priority |
| --- | --- | --- | --- |
| Symlink management | Consolidate files and maintain their original-location links. | Implemented, with correctness defects and a proposed linking refactor. | 1 |
| Profiles | Fork configurations for independent experimentation and switch between them. | Creation and switching exist; the intended independent-fork workflow needs substantial work. | 2 |
| Pack-and-Go | Capture a selected setup and install it on another machine. | Implemented and useful in the author's limited machine-setup use; safety and workflow gaps remain. | 3 |
| Diff | Inspect live-folder divergence and differences against packs. | Proposed; comparison scope and semantics need definition. | 4 |

This is the order for strengthening features, not a reason to defer confirmed
security or data-loss fixes in another feature. [BUGS.md](BUGS.md) records the audit;
its dependency-upgrade follow-up distinguishes addressed findings from outstanding
work. Current code references below identify implementation starting points, not
claims that each workflow has been validated end to end.

## 1. Symlink management

### Purpose and intended behavior

Adopt selected files into a managed directory, establish links at their original
locations, and later release them without losing content. The managed directory
must be a practical place to work, including creating and reorganizing files.

The proposed core commands are:

| Command | Purpose |
| --- | --- |
| `gather <targets>` | Add targets to a pending selection. |
| `release [targets]` | Remove targets from that selection; omit targets to clear it. |
| `link` | Execute linking for gathered items. |
| `unlink` | Execute unlinking for gathered items while preserving their contents. |

Preparation does not move files or change active tracking. Repeated gathers are
additive; overlaps resolve to one effective selection per item. A trailing slash
selects a directory, while a target without one must be a file. Directory selections
recurse with optional depth limits. Users can select `a/a` and `a/b/b` from one CWD
without adopting their siblings or discovering files above that CWD.

Gathering an already linked file's parent must incorporate the existing mapping,
not create another active copy. Unlinking a parent must handle individually linked
children correctly. Preparation and execution both support exact undo/redo, and
independent changes remain independently reversible.

The full design and acceptance criteria are in
[LINKING_REFACTOR.md](LINKING_REFACTOR.md).

### Work needed

- Correct unsafe unlink/remove restoration and link failure handling.
- Replace destination-exists checks with tracking-aware identity and conflict checks.
- Introduce explicit folder intent and member mappings so whole-folder tracking is
  distinguishable from independently selected files.
- Consolidate bounded selection, recursive traversal, depth control, and path lookup.
- Implement one operation planner with durable progress, rollback, and restart recovery.
- Add a SQLite change journal for exact selection and execution undo/redo, dependencies,
  and alternate redo branches.
- Distinguish intentional exclusions from missing files and pending-selection exclusions.
- Support explicit reverse adoption and managed-side reorganization without searching
  the system to rediscover original destinations.
- Keep normal output brief; support status, link checks, empty-item reporting, preview,
  and diagnostic verbosity.
- Migrate existing databases and commands without silently changing destructive behavior.

Implementation starts in [live-file commands](src/cli/commands.rs),
[tracking storage](src/config/mod.rs), and [path handling](src/utils/path_utils.rs).
Audit findings B01, B03, B05–B06, B08, and B11–B17 cover relevant defects.

### Unresolved concepts

- Pending-selection scope and lifetime after success or failure.
- Depth syntax and numbering, per-target limits, and exact name/path matching rules.
- Managed destination layout and relocation when a linked file's parent is adopted.
- Symlink traversal policy, managed-side target aliases, and selection boundaries.
- Handling mixed selections containing untracked, correctly linked, and conflicting items.
- Disposition of managed copies after unlink, recovery-data retention, and cleanup.
- Targeted undo and branch navigation without a large command interface.
- Migration or retirement of stage, unstage, remove, and purge.
- How known additions, removals, and reorganizations are explicitly reconciled.

### Evidence that this feature is solid

Supported file/folder round trips preserve contents, expected paths, and tracking.
Selection order cannot create duplicates. Depth and boundary rules hold under overlap,
missing paths, and symlinks. Injected failures and process interruption recover
predictably. Undo/redo restores actual changes rather than rescanning and guessing.

## 2. Profiles

### Purpose and intended behavior

Profiles provide editable configurations that can be forked from an active profile
and switched between. They were motivated by experimenting with Emacs and Vim
configurations without maintaining ad hoc backup folders.

An experimental profile must have independent file contents. Editing it must not
modify its parent through shared symlink targets or hardlinks. Switching selects
which configuration is available at each application's expected locations. A pack
can optionally capture a profile for provenance or transfer; packing is not required
for experimentation.

### Current implementation and work needed

Profile commands exist, but creation currently has two different paths: the legacy
creator makes a directory under `~/.forge/profiles`, while `new --profile` registers
a managed folder. Switching reads the legacy profile directory. These paths need
a coherent profile identity, storage model, and active-selection model.

The existing creator does not implement copying an active configuration into an
independent fork. Switching creates links and adds tracking records but needs the
same conflict handling, uniqueness, and recovery guarantees as the linking core.
Repeated switches must not accumulate duplicate mappings or leave stale links from
the previous profile.

Implementation starts in [profile commands](src/cli/commands.rs#L1016),
[CLI dispatch](src/main.rs), and [managed-folder selection](src/config/mod.rs#L377).

### Unresolved concepts

- What a profile owns: a complete managed tree, a selected configuration group,
  or explicit mappings across multiple roots.
- Whether one profile or several nonconflicting profiles can be active at once.
- Fork command syntax, copy semantics, and persistent parent/provenance information.
- Whether forks are complete independent copies or support inheritance; inheritance
  must not be assumed merely because the workflow resembles Git branches.
- What happens to files present in the old profile but absent from the new one.
- Treatment of unmanaged destination files, broken links, and external edits at switch time.
- Profile rename, release, deletion, and retention of inactive experiments.
- Undo dependencies across switches, forks, subsequent file operations, and profile removal.
- Relationship among profile identity, managed folders, and pack scopes.

### Evidence that this feature is solid

Fork, edit, switch away, and switch back preserve both configurations independently.
Switching failures leave a usable prior state or a recoverable recorded state.
Nested files and profile-specific additions/removals follow explicit rules. Undo/redo
restores mappings and active profile state without overwriting intervening work.
Tests must invoke Forge's real profile commands, not just manually replace symlinks.

## 3. Pack-and-Go

### Purpose and intended behavior

Pack-and-Go is a bounded machine-setup facility: with a current pack and the Forge
binary, the user can quickly install the included configuration on another machine.
Its successful use for this purpose is valuable experience, not proof that all
archive or installation paths are safe.

The existing lifecycle deliberately creates an active pack, records selected source
paths and hashes in a manifest, then validates sources and creates an archive at
seal time. Successful sealing consumes active staging. A finalized pack captures
its included files and manifest; it does not guarantee the complete state of the
machine or successful installation despite unresolved conflicts.

Sealed packs are not edited in place. Forgotten files and desired changes are handled
by producing another complete pack. The user may preserve it through enumeration
or explicitly replace the newest or a named pack. Replacement is intentional artifact
creation, not a grace period for modifying a sealed snapshot.

### Work needed

- Fix repack so it refreshes existing manifest entries instead of skipping them.
- Prevent accidental same-day archive replacement; add collision-safe naming and explicit
  replacement behavior.
- Build and validate a replacement before atomically replacing its predecessor.
- Ensure archived bytes match the manifest even if sources change during sealing.
- Detect duplicate payload paths and install/restore destination collisions before writing.
- Make conflict, missing-payload, hash-failure, and partial-install outcomes explicit in
  diagnostics and exit status.
- Preserve recoverable destinations when replacing files during installation or restoration.
- Keep old packs readable and verify home mapping, original-path restoration, permissions,
  and supported archive formats across relevant environments.
- Bound memory use for large files and make output ordering deterministic.

The dependency upgrade added unsafe ZIP-entry and manifest-payload path rejection;
archive hardening is not wholly outstanding. Collision handling, publication safety,
and other audit findings still need work. See B04, B07, B09–B11, and B17 in
[BUGS.md](BUGS.md).

Implementation starts in [pack commands](src/cli/commands/pack.rs).
[DESIGN.md](DESIGN.md) preserves the earlier Pack-and-Go notes.

### Unresolved concepts

- Concise syntax for enumeration, replacing the newest pack, and replacing a named pack.
- Pack identity versus its filename when an explicitly named artifact is replaced.
- How users create a replacement from the current selection after seal consumes staging.
- Exact supported setup scope: files and their destinations, with an explicit policy for
  permissions and symlinks; no automatic expansion into package or service management.
- Whether profile identity and lineage should be included as optional provenance.
- The exact undo/redo boundary at pack finalization and how it is disclosed beforehand.
  Do not assume that finalizing an artifact clears unrelated live-file undo history.
- Recovery and reversal policy for installation/restore operations, distinct from undoing
  pack finalization itself.

### Evidence that this feature is solid

A pack captures exactly the chosen files with matching metadata and hashes. Existing
packs remain installable. Installation produces the intended included setup or clearly
reports why it cannot. Malicious or malformed input cannot escape intended paths.
Failed creation/replacement preserves the previous usable archive and active staging.
Explicit replacement and enumeration behave distinctly and predictably.

## 4. Diff

### Purpose and boundary

Diff is read-only inspection. It arose from additions and deletions not propagating
between original and managed folders, particularly when a user creates a new file
inside a managed configuration directory and expects a way to link it back.

Comparison against a pack is also useful. Supporting a diff view does not establish
a reason to add editing, merging, patch application, automatic synchronization, or
sealed-pack mutation. Those remain outside Diff's responsibility.

### Proposed initial comparisons

| Comparison | Information |
| --- | --- |
| Original and managed state | Tracked mappings, untracked files, missing files, broken/misdirected links, structural divergence, and conflicting independent files. |
| Current selection against a pack | Added, modified, removed, and unchanged files relative to the chosen manifest and selection. |

File edits through a correct symlink are not two independent copies diverging.
Diff must distinguish that normal shared content from an original path replaced by
an independent file. Missing a symlink is not proof that the user intended to delete
its managed content. A matching hash is evidence of content equality, not proof of
rename intent.

Report paths, statuses, concise reasons, and summaries first. More detailed content
comparison is a separate unresolved display decision; even if supported, it would
remain read-only. Profile-to-profile and pack-to-pack comparisons are possible later
extensions, not requirements for the first release.

### Work needed

- Base comparisons on reliable mapping identity, folder intent, exclusions, and profile scope.
- Compare explicit unions of selected current and recorded paths so deleted files appear.
- Distinguish known absence from unreadable files, traversal failures, and unknown state.
- Exclude Forge's own metadata, recovery data, staging, and archives from ordinary live scans.
- Use recorded hashes and explicit provenance rather than relying on timestamps alone.
- Keep inspection from mutating tracking or silently creating/backfilling a baseline.
- Provide results that explicit file-management operations can act on without making Diff
  itself perform reconciliation.

The [original diff proposal](DESIGN.md#forge-diff-feature-proposal) is a draft focused
on the last pack. Its per-file snapshot writes and comparison pseudocode are not
settled implementation requirements and do not cover the full live-folder use case.
There is no implemented Diff command in the current command surface.

### Unresolved concepts

- Default comparison: live mapping state, an explicitly selected pack, or a contextual default.
- Pack baseline: last sealed pack, active manifest, or explicit archive; these are different states.
- Definition of an added file when the pack or tracked folder intentionally contains a subset.
- Relationship to status and check: avoid three commands repeating the same report.
- Single-file, directory, profile, and managed-folder selection syntax and boundaries.
- Whether line-level display has enough value to include, without introducing edit operations.
- How to describe likely moves/renames and conflicts without presenting guesses as facts.
- Output format, verbosity, machine-readable results, and exit codes for differences versus errors.

### Evidence that this feature is solid

Every reported difference has a defined comparison basis. Single-file inspection does
not report unrelated files as removed. Selected empty folders, deleted paths, exclusions,
partially tracked folders, and missing baselines have explicit outcomes. Comparison is
repeatable on unchanged input and leaves files, selection, tracking, and journal state
unchanged.

## Shared work and delivery order

The four features need a shared core for path identity, scoped mappings, bounded
selection, preflight checks, execution plans, recovery, and structured results.
Implement those components for concrete linking needs first, then reuse them in
profiles and Pack-and-Go. Diff should consume their reliable state without becoming
another mutation engine.

1. Establish safe linking/unlinking and exact preparation/execution undo/redo.
2. Complete independent profile forking and reversible switching over that core.
3. Strengthen pack creation, replacement, and machine-setup validation.
4. Implement the smallest useful read-only Diff comparisons.

Fix urgent safety defects wherever they occur during that sequence. Assess performance
with representative directory trees, file sizes, and mapping counts. Reduce repeated
scans, database connections, and copying while retaining observable failure handling.
Keep parser changes and additional public commands tied to demonstrated workflow needs.
