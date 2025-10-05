use crate::symlink;
use std::io;
use std::path::Path;

#[allow(dead_code)]
pub fn link_file<P: AsRef<Path>, Q: AsRef<Path>>(source: P, target: Q) -> io::Result<()> {
    // Implementation of link_file
    let source = source.as_ref();
    let target = target.as_ref();

    if target.exists() {
        if target.is_symlink() {
            // If target is already a symlink, remove it
            std::fs::remove_file(target)?;
        } else {
            // If target exists but is not a symlink, back it up
            crate::dotfile::backup::backup_file(target)?;
            std::fs::remove_file(target)?;
        }
    }

    // Create parent directories if they don't exist
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }

    symlink::create_symlink(source, target)?;
    Ok(())
}
