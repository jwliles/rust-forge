use std::fs;
use std::io;
use std::path::Path;

pub fn backup_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    // Implementation of backup_file
    let path = path.as_ref();
    if !path.exists() {
        return Ok(());
    }

    let backup_path = path.with_extension("bak");
    fs::copy(path, backup_path)?;
    Ok(())
}
