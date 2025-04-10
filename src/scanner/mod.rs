use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn scan_directory<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
    // Implementation of scan_directory
    let dir = dir.as_ref();
    let mut result = Vec::new();
    
    if !dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path is not a directory",
        ));
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            result.push(path);
        } else if path.is_dir() {
            // Recursively scan subdirectories
            let mut subdir_files = scan_directory(path)?;
            result.append(&mut subdir_files);
        }
    }
    
    Ok(result)
}
