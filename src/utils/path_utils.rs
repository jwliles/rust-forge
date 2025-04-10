use std::path::{Path, PathBuf};

pub fn expand_tilde<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    if !path.starts_with("~") {
        return path.to_path_buf();
    }
    
    let path_str = path.to_string_lossy();
    let path_str = path_str.strip_prefix("~").unwrap();
    
    match dirs::home_dir() {
        Some(mut home) => {
            if path_str.starts_with('/') || path_str.starts_with('\\') {
                home.push(&path_str[1..]);
            } else if !path_str.is_empty() {
                home.push(path_str);
            }
            home
        }
        None => path.to_path_buf(),
    }
}

pub fn is_absolute<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_absolute()
}

pub fn normalize<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    let path = expand_tilde(path);
    
    // Make the path absolute if it's not already
    if !path.is_absolute() {
        if let Ok(current_dir) = std::env::current_dir() {
            return current_dir.join(path);
        }
    }
    
    path
}
