// CLI command implementations
use std::path::PathBuf;
use crate::config;
use crate::symlink;
use crate::dotfile;

/// Heat (stage) files for symlinking 
pub fn heat_command(files: &[PathBuf]) {
    println!("Staging files: {:?}", files);
    
    // In the real implementation, this would store the files in a staging area
    // For now, just print the files
    
    for file in files {
        println!("Staged: {:?}", file);
    }
}

/// Create symlinks for all heated files
pub fn forge_command() {
    println!("Creating symlinks");
    
    // Get default target directory from config
    let target_dir = config::read_default_path();
    
    // Create symlinks from current directory to target
    match symlink::create_symlinks(".", &target_dir) {
        Ok(_) => println!("Symlinks created successfully to {}", target_dir),
        Err(e) => println!("Error creating symlinks: {}", e),
    }
}

/// Remove symlinks for specific files
pub fn cool_command(files: &[PathBuf]) {
    println!("Removing symlinks for files: {:?}", files);
    
    // For each file, check if it's linked and remove the symlink
    for file in files {
        // In a real implementation, this would lookup the symlink target in the database
        // For now, just try to handle the file directly
        
        if let Some(filename) = file.file_name() {
            // Get default target directory from config
            let target_dir = config::read_default_path();
            
            // Construct the target path
            let target_path = PathBuf::from(&target_dir).join(filename);
            
            // Check if it's a symlink
            if symlink::is_symlink(&target_path) {
                // Try to remove it
                match std::fs::remove_file(&target_path) {
                    Ok(_) => println!("Removed symlink: {:?}", target_path),
                    Err(e) => println!("Failed to remove symlink {:?}: {}", target_path, e),
                }
            } else {
                println!("Not a symlink or doesn't exist: {:?}", target_path);
            }
        }
    }
}

pub mod profile {
    use std::fs;
    use std::path::PathBuf;
    use crate::config;
    
    const PROFILES_DIR: &str = ".dotforge/profiles";
    
    /// Create a new profile
    pub fn create(name: &str) {
        println!("Creating profile: {}", name);
        
        // Create profile directory
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let profile_dir = home_dir.join(PROFILES_DIR).join(name);
        
        if profile_dir.exists() {
            println!("Profile '{}' already exists", name);
            return;
        }
        
        match fs::create_dir_all(&profile_dir) {
            Ok(_) => println!("Profile '{}' created at {:?}", name, profile_dir),
            Err(e) => println!("Failed to create profile directory: {}", e),
        }
    }
    
    /// List available profiles
    pub fn list() {
        println!("Available profiles:");
        
        // Get the profiles directory
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let profiles_dir = home_dir.join(PROFILES_DIR);
        
        if !profiles_dir.exists() {
            println!("No profiles found");
            return;
        }
        
        // Read the directories in the profiles directory
        match fs::read_dir(&profiles_dir) {
            Ok(entries) => {
                let mut found = false;
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Ok(metadata) = entry.metadata() {
                            if metadata.is_dir() {
                                found = true;
                                if let Some(name) = entry.file_name().to_str() {
                                    println!("  - {}", name);
                                }
                            }
                        }
                    }
                }
                
                if !found {
                    println!("No profiles found");
                }
            },
            Err(e) => println!("Error reading profiles directory: {}", e),
        }
    }
    
    /// Switch to a profile
    pub fn switch(name: &str) {
        println!("Switching to profile: {}", name);
        
        // Check if profile exists
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let profile_dir = home_dir.join(PROFILES_DIR).join(name);
        
        if !profile_dir.exists() {
            println!("Profile '{}' does not exist", name);
            return;
        }
        
        // Get default target directory from config
        let target_dir = config::read_default_path();
        
        // Create symlinks from profile directory to target
        match crate::symlink::create_symlinks(&profile_dir, &target_dir) {
            Ok(_) => println!("Switched to profile '{}' successfully", name),
            Err(e) => println!("Error switching to profile '{}': {}", name, e),
        }
    }
}
