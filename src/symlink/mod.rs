use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config;
use crate::utils::path_utils;

/// Create a symlink from source to target
pub fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(source: P, target: Q) -> io::Result<()> {
    #[cfg(unix)]
    return std::os::unix::fs::symlink(source, target);
    
    #[cfg(windows)]
    {
        let source = source.as_ref();
        if source.is_dir() {
            return std::os::windows::fs::symlink_dir(source, target);
        } else {
            return std::os::windows::fs::symlink_file(source, target);
        }
    }
}

/// Check if path is a symlink
pub fn is_symlink<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_symlink()
}

/// Get the target of a symlink
pub fn get_symlink_target<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path = path.as_ref();
    if !path.is_symlink() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Path is not a symlink",
        ));
    }
    
    std::fs::read_link(path)
}

/// Creates symlinks from files in source directory to target directory
/// based on file types and ignored paths from configuration
pub fn create_symlinks<P: AsRef<Path>, Q: AsRef<Path>>(source: P, target: Q) -> io::Result<()> {
    // Resolve and normalize source directory
    let abs_source = path_utils::normalize(source.as_ref());
    
    // Resolve and normalize target directory
    let abs_target = path_utils::normalize(target.as_ref());
    
    // Ensure the source directory exists
    if !abs_source.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Source directory does not exist: {:?}", abs_source)
        ));
    }
    
    // Ensure the target directory exists
    if !abs_target.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Target directory does not exist: {:?}", abs_target)
        ));
    }
    
    // Read registered file types and ignored paths
    let filetypes = config::get_file_types()?;
    let ignored_paths = config::get_ignored_paths()?;
    
    // Create HashMaps for faster lookup
    let filetypes_map: HashMap<String, bool> = filetypes
        .iter()
        .map(|ft| (ft.to_string(), true))
        .collect();
    
    let ignored_paths_map: HashMap<PathBuf, bool> = ignored_paths
        .iter()
        .map(|path| (PathBuf::from(path), true))
        .collect();
    
    // Walk through source directory and create symlinks in target
    for entry in WalkDir::new(&abs_source)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok()) {
            
        let path = entry.path();
        
        // Skip if path is in ignored paths
        if ignored_paths_map.contains_key(&PathBuf::from(path)) {
            if path.is_dir() {
                continue; // Skip entire directory
            }
            continue;
        }
            
        // Only process files, not directories
        if path.is_file() {
            // Check if the file extension is in the approved list
            if let Some(ext) = path.extension() {
                let ext_str = format!(".{}", ext.to_string_lossy());
                if filetypes_map.contains_key(&ext_str) {
                    // Create the symlink in the target directory
                    let file_name = path.file_name().unwrap();
                    let target_path = abs_target.join(file_name);
                    
                    match create_symlink(path, &target_path) {
                        Ok(_) => println!("Created symlink for {:?} -> {:?}", path, target_path),
                        Err(e) => println!("Failed to create symlink for {:?}: {}", path, e),
                    }
                }
            }
        }
    }
    
    Ok(())
}

/// Remove all symlinks in the given directory
pub fn remove_old_symlinks<P: AsRef<Path>>(target_dir: P) -> io::Result<()> {
    for entry in WalkDir::new(target_dir.as_ref())
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok()) {
            
        let path = entry.path();
        if path.is_symlink() {
            match fs::remove_file(path) {
                Ok(_) => println!("Removed symlink: {:?}", path),
                Err(e) => println!("Failed to remove symlink {:?}: {}", path, e),
            }
        }
    }
    
    Ok(())
}
