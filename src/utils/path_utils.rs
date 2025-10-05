use std::path::{Component, Path, PathBuf};

pub fn expand_tilde<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    if !path.starts_with("~") {
        return path.to_path_buf();
    }

    let path_str = path.to_string_lossy();
    let path_str = match path_str.strip_prefix("~") {
        Some(stripped) => stripped,
        None => {
            // This should not happen due to the guard above, but be defensive
            return path.to_path_buf();
        }
    };

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

pub fn normalize<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    let path = expand_tilde(path);

    // Make the path absolute if it's not already
    if !path.is_absolute() {
        if let Ok(current_dir) = std::env::current_dir() {
            // Strip . and .. components before joining
            let clean_path = clean_path_components(&path);
            return current_dir.join(clean_path);
        }
    }

    // Also clean absolute paths in case they have . or ..
    clean_path_components(&path)
}

fn clean_path_components(path: &Path) -> PathBuf {
    let mut components = Vec::new();

    for component in path.components() {
        match component {
            Component::CurDir => continue, // Skip "."
            Component::ParentDir => {
                // Handle ".." by popping previous component
                if !components.is_empty() {
                    components.pop();
                }
            }
            comp => components.push(comp),
        }
    }

    components.iter().collect()
}
