# Forge Diff Feature Proposal

*Version: Draft 0.1  ·  Date: 2026-01-02*

## 0  Motivation

Forge currently lacks visibility into what has changed in the managed folder since the last pack operation. Users may:
- Edit dotfiles and forget what changed
- Create redundant sealed packs when nothing actually changed
- Lack awareness of which files need to be repacked

A diff view would provide visibility into changes without turning Forge into a full VCS.

## 1  Design Goals

**In scope:**
- Show which files have been modified/added/removed since last pack
- Use database metadata (not filesystem timestamps, which are unreliable)
- Keep database small (hashes only, no content storage)
- Simple command: `forge diff .` or `forge diff <file>`

**Out of scope:**
- Line-by-line content diffs (that's VCS territory)
- Revert functionality (storage complexity)
- Auto-pack automation (unnecessary complexity)
- Change history over time (just current vs. last pack)

## 2  Database Schema

Add a new table to track file snapshots at pack time:

```sql
CREATE TABLE file_snapshots (
    scope TEXT NOT NULL,
    file_path TEXT NOT NULL,        -- Absolute path to file
    hash TEXT NOT NULL,              -- BLAKE3 hash
    size INTEGER NOT NULL,           -- File size in bytes
    snapshot_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (scope, file_path)
);
```

**Properties:**
- One snapshot per file per scope (last pack state)
- Updates on `forge pack`
- No file content stored (just metadata)
- Keyed by (scope, file_path) so each file has one current snapshot

## 3  Command Syntax

### Compare entire managed folder
```bash
forge diff .
```

Compares all files in the active managed folder against the last pack for the default scope.

### Compare specific file
```bash
forge diff ~/.forge/.vimrc
```

Shows whether the specific file has changed.

### Compare against specific scope
```bash
forge diff <file> --scope <name>
```

Compare against a different scope's last pack.

## 4  Output Format

### Example: Changes detected
```
$ forge diff .

Comparing to last pack: dotfiles (packed 3 days ago)

Modified (2):
  .vimrc
    Previous: 1.2 KB (hash: a4f2b8c3...)
    Current:  1.3 KB (hash: d7e9f1a2...)

  .bashrc
    Previous: 890 bytes (hash: 3b8c9d2e...)
    Current:  920 bytes (hash: 8f2a3b1c...)

Added (1):
  .tmux.conf (450 bytes)

Removed (1):
  .oldrc (was 230 bytes)

Unchanged: 5 files

Summary: 4 changes detected since last pack
```

### Example: No changes
```
$ forge diff .

Comparing to last pack: dotfiles (packed 3 days ago)

No changes detected.

All 7 files unchanged since last pack.
```

### Example: No baseline
```
$ forge diff .

No previous pack found for scope: dotfiles

All files in managed folder (7 files) would be new.
Run 'forge pack . -r' to create initial pack.
```

## 5  Implementation Flow

### During `forge pack`

```rust
// After calculating hash for manifest (pack.rs:324)
let hash = calculate_file_hash(&abs_source)?;

// NEW: Store snapshot in database
store_file_snapshot(
    scope,
    &abs_source,
    &hash,
    metadata.len(),
)?;

// Continue with existing manifest logic
manifest.add_file(&abs_source, &relative_path, Some(hash))?;
```

### During `forge diff`

```rust
fn diff_command(path: &Path, scope: Option<&str>) -> Result<()> {
    // 1. Determine scope (from arg or default)
    let scope = scope.unwrap_or_else(|| get_default_scope());

    // 2. Get managed folder
    let (_, managed_folder) = get_active_managed_folder()?;

    // 3. Get files to compare
    let files = if path == Path::new(".") {
        // All files in managed folder (recursive walk)
        walk_managed_folder(&managed_folder)?
    } else {
        vec![normalize(path)]
    };

    // 4. Load snapshots from database
    let snapshots = load_snapshots_for_scope(scope)?;

    // 5. Compare current files to snapshots
    let mut modified = Vec::new();
    let mut added = Vec::new();
    let mut unchanged = Vec::new();

    for file in &files {
        let current_hash = calculate_file_hash(file)?;

        if let Some(snapshot) = snapshots.get(file) {
            if current_hash == snapshot.hash {
                unchanged.push(file);
            } else {
                modified.push((file, snapshot, current_hash));
            }
        } else {
            added.push(file);
        }
    }

    // 6. Find removed files (in snapshot but not in folder)
    let removed: Vec<_> = snapshots.keys()
        .filter(|path| !files.contains(path))
        .collect();

    // 7. Display results
    display_diff(modified, added, removed, unchanged, scope)?;

    Ok(())
}
```

## 6  Database Operations

### Store snapshot (on pack)
```rust
pub fn store_file_snapshot(
    scope: &str,
    file_path: &Path,
    hash: &str,
    size: u64,
) -> rusqlite::Result<()> {
    let config = get_db_connection()?;
    if let Some(conn) = &config.connection {
        conn.execute(
            "INSERT OR REPLACE INTO file_snapshots
             (scope, file_path, hash, size, snapshot_at)
             VALUES (?1, ?2, ?3, ?4, CURRENT_TIMESTAMP)",
            rusqlite::params![
                scope,
                file_path.to_string_lossy().to_string(),
                hash,
                size as i64
            ],
        )?;
    }
    Ok(())
}
```

### Load snapshots (on diff)
```rust
pub fn load_snapshots_for_scope(
    scope: &str
) -> rusqlite::Result<HashMap<PathBuf, FileSnapshot>> {
    let config = get_db_connection()?;
    let mut snapshots = HashMap::new();

    if let Some(conn) = &config.connection {
        let mut stmt = conn.prepare(
            "SELECT file_path, hash, size, snapshot_at
             FROM file_snapshots
             WHERE scope = ?1"
        )?;

        let rows = stmt.query_map([scope], |row| {
            Ok((
                PathBuf::from(row.get::<_, String>(0)?),
                FileSnapshot {
                    hash: row.get(1)?,
                    size: row.get(2)?,
                    snapshot_at: row.get(3)?,
                }
            ))
        })?;

        for row in rows {
            let (path, snapshot) = row?;
            snapshots.insert(path, snapshot);
        }
    }

    Ok(snapshots)
}
```

## 7  Migration Path

### Database migration
Add migration to create the new table in existing databases:

```rust
// In config/mod.rs init_database()
conn.execute(
    "CREATE TABLE IF NOT EXISTS file_snapshots (
        scope TEXT NOT NULL,
        file_path TEXT NOT NULL,
        hash TEXT NOT NULL,
        size INTEGER NOT NULL,
        snapshot_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY (scope, file_path)
    )",
    [],
)?;
```

### Backfill existing packs
For users with existing unsealed packs, run validation on first diff:

```rust
// If no snapshots exist for scope but pack manifest exists
if snapshots.is_empty() && manifest_exists(scope) {
    println!("Backfilling snapshots from existing pack manifest...");
    backfill_snapshots_from_manifest(scope)?;
}
```

## 8  Bonus: Seal Warning

Optional enhancement to prevent redundant packs:

```rust
// In seal_pack_impl(), before creating archive
let snapshots = load_snapshots_for_scope(scope)?;
let manifest = load_current_manifest(scope)?;

if snapshots_match_manifest(&snapshots, &manifest) {
    println!("Warning: No changes detected since last pack.");
    println!("Seal anyway? [y/N]");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if !input.trim().eq_ignore_ascii_case("y") {
        println!("Seal cancelled.");
        return Ok(());
    }
}
```

## 9  Testing Considerations

- Test with empty managed folder (no snapshots)
- Test with files added/modified/removed
- Test with multiple scopes
- Test hash comparison accuracy
- Test with no previous pack (fresh scope)
- Ensure snapshot updates on repack
- Test with symbolic links in managed folder
- Verify performance with large file counts

## 10  Open Questions

1. **Display verbosity**: Should unchanged files be collapsed by default or shown with `--verbose`?
2. **Hash display**: Show full hash, truncated (8 chars), or hide entirely?
3. **Timestamp display**: Show absolute time, relative ("3 days ago"), or both?
4. **Integration with seal**: Make the warning mandatory or opt-in via flag?

## 11  Summary

This feature adds lightweight change detection to Forge without turning it into a VCS. It:
- Uses existing hash calculation infrastructure
- Adds minimal database storage (metadata only)
- Provides visibility users need without feature creep
- Maintains Forge's philosophy of intentional, explicit actions
- Optionally prevents redundant pack creation

The implementation reuses existing validation logic and fits naturally into the current pack workflow.
