// CLI command implementations
use crate::config;
use crate::symlink;
use crate::utils::path_utils;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Initialize a directory as a forge managed folder
pub fn init_command(name: Option<&str>, dir: Option<&Path>) {
    // Determine the directory to initialize
    let init_dir = match dir {
        Some(d) => path_utils::normalize(d),
        None => match env::current_dir() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Failed to get current directory: {}", e);
                return;
            }
        },
    };

    // Determine the name to use
    let folder_name = match name {
        Some(n) => n.to_string(),
        None => {
            // Use the directory name as the default
            match init_dir.file_name() {
                Some(n) => n.to_string_lossy().to_string(),
                None => {
                    eprintln!("Could not determine folder name. Please specify a name with --name");
                    return;
                }
            }
        }
    };

    println!(
        "Initializing forge managed folder '{}' at {}",
        folder_name,
        init_dir.display()
    );

    // Check if directory exists, create if needed
    if !init_dir.exists() {
        match fs::create_dir_all(&init_dir) {
            Ok(_) => println!("Created directory: {}", init_dir.display()),
            Err(e) => {
                eprintln!("Failed to create directory: {}", e);
                return;
            }
        }
    }

    // Create a .dotforge subfolder in the managed folder
    let dotforge_dir = init_dir.join(".dotforge");
    if !dotforge_dir.exists() {
        match fs::create_dir_all(&dotforge_dir) {
            Ok(_) => println!("Created .dotforge directory"),
            Err(e) => {
                eprintln!("Failed to create .dotforge directory: {}", e);
                return;
            }
        }
    }

    // Add to managed folders
    match config::add_managed_folder(&folder_name, &init_dir) {
        Ok(_) => {
            println!("Added '{}' to managed folders", folder_name);
            println!("\nForge repository initialized successfully!");
            println!("You can now use 'forge stage' to stage files for tracking.");
        }
        Err(e) => {
            eprintln!("Failed to add to managed folders: {}", e);
        }
    }
}

/// Add files to be tracked for symlinking (legacy heat command)
pub fn heat_command(files: &[PathBuf]) {
    stage_command(files)
}

/// Stage files to be tracked for symlinking
pub fn stage_command(files: &[PathBuf]) {
    println!("Staging files: {:?}", files);

    // Get the active managed folder
    let (folder_name, forge_path) = match config::get_active_managed_folder() {
        Ok(Some((name, path))) => (name, path),
        Ok(None) => {
            eprintln!("No managed folders found. Please run 'dotforge init' first.");
            return;
        }
        Err(e) => {
            eprintln!("Failed to get managed folder: {}", e);
            return;
        }
    };

    println!(
        "Using managed folder '{}' at {}",
        folder_name,
        forge_path.display()
    );

    if !forge_path.exists() {
        match fs::create_dir_all(&forge_path) {
            Ok(_) => println!("Created forge directory: {}", forge_path.display()),
            Err(e) => {
                eprintln!(
                    "Failed to create forge directory {}: {}",
                    forge_path.display(),
                    e
                );
                return;
            }
        }
    }

    for file in files {
        // Normalize path
        let abs_source = path_utils::normalize(file);

        if !abs_source.exists() {
            eprintln!("File does not exist: {}", abs_source.display());
            continue;
        }

        // Extract filename for the target
        if let Some(filename) = file.file_name() {
            let target = forge_path.join(filename);

            // Create a temporary symlink (or copy) from forge folder TO original file
            if target.exists() {
                println!(
                    "Target already exists in forge folder: {}",
                    target.display()
                );
                continue;
            }

            // Create a symlink from forge folder TO original file (reverse of final state)
            match symlink::create_symlink(&abs_source, &target) {
                Ok(_) => {
                    println!(
                        "Created staging symlink: {} → {}",
                        target.display(),
                        abs_source.display()
                    );

                    // Add to database as staged
                    match config::stage_dotfile(&abs_source, &target, None) {
                        Ok(_) => println!(
                            "Staged file: {} (use 'link' to make permanent)",
                            abs_source.display()
                        ),
                        Err(e) => eprintln!("Failed to stage {}: {}", abs_source.display(), e),
                    }
                }
                Err(e) => eprintln!(
                    "Failed to create staging symlink for {}: {}",
                    abs_source.display(),
                    e
                ),
            }
        } else {
            eprintln!("Invalid file path: {}", file.display());
        }
    }

    println!("\nNOTE: Files are only staged. Use 'forge link' to create permanent symlinks.");
}

/// Create symlinks for all staged/tracked files (legacy forge command)
pub fn forge_command() {
    link_command(&[])
}

/// Create symlinks for all staged/tracked files
pub fn link_command(files: &[PathBuf]) {
    println!("Creating symlinks");

    // Get the active managed folder
    let (folder_name, forge_path) = match config::get_active_managed_folder() {
        Ok(Some((name, path))) => (name, path),
        Ok(None) => {
            eprintln!("No managed folders found. Please run 'dotforge init' first.");
            return;
        }
        Err(e) => {
            eprintln!("Failed to get managed folder: {}", e);
            return;
        }
    };

    println!(
        "Using managed folder '{}' at {}",
        folder_name,
        forge_path.display()
    );

    // Get all staged dotfiles from the database
    let dotfiles = if files.is_empty() {
        match config::get_staged_dotfiles(None) {
            Ok(df) => df,
            Err(e) => {
                eprintln!("Error fetching staged files: {}", e);
                return;
            }
        }
    } else {
        // If specific files requested, check if they exist and are staged
        let mut result = Vec::new();

        for file in files {
            let abs_path = path_utils::normalize(file);

            match config::find_dotfile_by_target(&abs_path) {
                Ok(Some(df)) => {
                    if df.is_staged() {
                        result.push(df);
                    } else {
                        println!("File already linked: {}", abs_path.display());
                    }
                }
                Ok(None) => {
                    eprintln!("File not found in staging: {}", abs_path.display());
                }
                Err(e) => {
                    eprintln!("Error checking file {}: {}", abs_path.display(), e);
                }
            }
        }

        result
    };

    if dotfiles.is_empty() {
        println!("No files to link. Use 'stage' command to stage files first.");
        return;
    }

    let mut success_count = 0;
    let mut error_count = 0;

    // Link each dotfile
    for dotfile in dotfiles {
        // First remove the staging symlink
        if let Err(e) = fs::remove_file(&dotfile.target) {
            eprintln!(
                "Failed to remove staging symlink {}: {}",
                dotfile.target.display(),
                e
            );
            error_count += 1;
            continue;
        }

        // Move the original file to the forge directory
        match fs::copy(&dotfile.source, &dotfile.target) {
            Ok(_) => {
                // Create symlink from original location to forge directory FIRST
                match symlink::create_symlink(&dotfile.target, &dotfile.source) {
                    Ok(_) => {
                        // Only remove the original file AFTER symlink is successfully created
                        if let Err(e) = fs::remove_file(&dotfile.source) {
                            eprintln!(
                                "Failed to remove original file {}: {}",
                                dotfile.source.display(),
                                e
                            );
                            // Try to remove the symlink we just created to maintain consistency
                            if let Err(cleanup_err) = fs::remove_file(&dotfile.source) {
                                eprintln!(
                                    "Failed to cleanup symlink during error recovery: {}",
                                    cleanup_err
                                );
                            }
                            error_count += 1;
                            continue;
                        }

                        println!(
                            "Created symlink: {} → {}",
                            dotfile.source.display(),
                            dotfile.target.display()
                        );

                        // Update database status to linked
                        if let Err(e) = config::link_dotfile(&dotfile.source, &dotfile.target) {
                            eprintln!("Failed to update database: {}", e);
                        }

                        success_count += 1;
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to create symlink {} → {}: {}",
                            dotfile.source.display(),
                            dotfile.target.display(),
                            e
                        );

                        // Original file is still intact, no need to restore
                        println!("Original file preserved at: {}", dotfile.source.display());

                        error_count += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to copy file {} to {}: {}",
                    dotfile.source.display(),
                    dotfile.target.display(),
                    e
                );
                error_count += 1;
            }
        }
    }

    println!(
        "\nSymlink creation completed: {} succeeded, {} failed",
        success_count, error_count
    );
}

/// Remove symlinks for specific files (legacy cool command)
pub fn cool_command(files: &[PathBuf], _skip_confirm: bool) {
    unlink_command(files, _skip_confirm)
}

/// List all tracked files
pub fn list_command(profile: Option<&str>) {
    crate::dotfile::list::print_dotfiles(profile);
}

/// Remove symlinks but keep files in forge folder
pub fn unlink_command(files: &[PathBuf], skip_confirm: bool) {
    if files.is_empty() {
        println!("No files specified to unlink. Here are all tracked files:");
        crate::dotfile::list::print_dotfiles(None);
        return;
    }

    // Get the active managed folder
    let (folder_name, forge_path) = match config::get_active_managed_folder() {
        Ok(Some((name, path))) => (name, path),
        Ok(None) => {
            eprintln!("No managed folders found. Please run 'dotforge init' first.");
            return;
        }
        Err(e) => {
            eprintln!("Failed to get managed folder: {}", e);
            return;
        }
    };

    println!(
        "Using managed folder '{}' at {}",
        folder_name,
        forge_path.display()
    );

    // For each file, restore the original and remove the symlink
    for file in files {
        // Determine target path
        let target = if file.is_absolute() {
            // If absolute path, use it directly (might be a target)
            file.clone()
        } else if let Some(filename) = file.file_name() {
            // If relative, assume it's a filename to be combined with forge dir
            forge_path.join(filename)
        } else {
            eprintln!("Invalid file path: {}", file.display());
            continue;
        };

        // Try to find the dotfile in the database
        let dotfile = match config::find_dotfile_by_target(&target) {
            Ok(Some(df)) => df,
            Ok(None) => {
                println!("No tracking record found for {}", target.display());

                // Even if not tracked, check if it's a symlink we can remove
                if file.file_name().is_some() {
                    let orig_path = PathBuf::from(file);
                    if symlink::is_symlink(&orig_path) {
                        // Confirm with user
                        if !skip_confirm {
                            let msg = format!(
                                "This will remove the symlink at {} but keep the file in the forge folder.",
                                orig_path.display()
                            );
                            if !crate::utils::ui::confirm(&msg) {
                                println!("Skipping {}", orig_path.display());
                                continue;
                            }
                        }

                        match fs::remove_file(&orig_path) {
                            Ok(_) => println!("Removed untracked symlink: {}", orig_path.display()),
                            Err(e) => {
                                println!("Failed to remove symlink {}: {}", orig_path.display(), e)
                            }
                        }
                    } else {
                        println!("Not a symlink or doesn't exist: {}", orig_path.display());
                    }
                }
                continue;
            }
            Err(e) => {
                eprintln!("Error looking up dotfile: {}", e);
                continue;
            }
        };

        // Confirm with user
        if !skip_confirm {
            let msg = format!(
                "This will remove the symlink at {} but keep the file in the forge folder.\nThe original file will be restored to {}.",
                dotfile.source.display(),
                dotfile.source.display()
            );
            if !crate::utils::ui::confirm(&msg) {
                println!("Skipping {}", dotfile.source.display());
                continue;
            }
        }

        // Copy from forge to original location
        match fs::copy(&dotfile.target, &dotfile.source) {
            Ok(_) => {
                // Remove the symlink
                match fs::remove_file(&dotfile.source) {
                    Ok(_) => {
                        println!(
                            "Removed symlink and restored file: {}",
                            dotfile.source.display()
                        );

                        // Update status in database
                        match config::deactivate_dotfile(&dotfile.target) {
                            Ok(_) => (),
                            Err(e) => eprintln!("Failed to update database: {}", e),
                        }
                    }
                    Err(e) => println!(
                        "Failed to remove symlink {}: {}",
                        dotfile.source.display(),
                        e
                    ),
                }
            }
            Err(e) => println!(
                "Failed to restore file from {}: {}",
                dotfile.target.display(),
                e
            ),
        }
    }
}

/// Remove files from forge folder but keep original files
pub fn remove_command(files: &[PathBuf], skip_confirm: bool) {
    if files.is_empty() {
        println!("No files specified to remove. Here are all tracked files:");
        crate::dotfile::list::print_dotfiles(None);
        return;
    }

    // Get the active managed folder
    let (folder_name, forge_path) = match config::get_active_managed_folder() {
        Ok(Some((name, path))) => (name, path),
        Ok(None) => {
            eprintln!("No managed folders found. Please run 'dotforge init' first.");
            return;
        }
        Err(e) => {
            eprintln!("Failed to get managed folder: {}", e);
            return;
        }
    };

    println!(
        "Using managed folder '{}' at {}",
        folder_name,
        forge_path.display()
    );

    // For each file, remove tracking and delete from forge folder
    for file in files {
        // Determine target path
        let target = if file.is_absolute() {
            // If absolute path, use it directly (might be a target)
            file.clone()
        } else if let Some(filename) = file.file_name() {
            // If relative, assume it's a filename to be combined with forge dir
            forge_path.join(filename)
        } else {
            eprintln!("Invalid file path: {}", file.display());
            continue;
        };

        // Try to find the dotfile in the database
        let dotfile = match config::find_dotfile_by_target(&target) {
            Ok(Some(df)) => df,
            Ok(None) => {
                println!("No tracking record found for {}", target.display());
                continue;
            }
            Err(e) => {
                eprintln!("Error looking up dotfile: {}", e);
                continue;
            }
        };

        // Confirm with user
        if !skip_confirm {
            let msg = format!(
                "This will:\n- Remove the symlink at {}\n- Delete the file from the forge folder\n- Keep the original file at {}\n- Remove tracking information from the database",
                dotfile.source.display(),
                dotfile.source.display()
            );
            if !crate::utils::ui::confirm(&msg) {
                println!("Skipping {}", dotfile.source.display());
                continue;
            }
        }

        // Unlink first
        if symlink::is_symlink(&dotfile.source) {
            // Copy from forge to original location
            match fs::copy(&dotfile.target, &dotfile.source) {
                Ok(_) => {
                    // Remove the symlink
                    if let Err(e) = fs::remove_file(&dotfile.source) {
                        println!(
                            "Failed to remove symlink {}: {}",
                            dotfile.source.display(),
                            e
                        );
                        continue;
                    }
                }
                Err(e) => {
                    println!(
                        "Failed to restore file from {}: {}",
                        dotfile.target.display(),
                        e
                    );
                    continue;
                }
            }
        }

        // Delete from forge folder
        match fs::remove_file(&dotfile.target) {
            Ok(_) => {
                println!(
                    "Removed file from forge folder: {}",
                    dotfile.target.display()
                );

                // Remove from database
                match config::remove_dotfile(&dotfile.target) {
                    Ok(_) => println!("Removed tracking for {}", dotfile.target.display()),
                    Err(e) => eprintln!("Failed to update database: {}", e),
                }
            }
            Err(e) => println!("Failed to remove file {}: {}", dotfile.target.display(), e),
        }
    }
}

/// Delete files completely from the system
pub fn delete_command(files: &[PathBuf], skip_confirm: bool) {
    if files.is_empty() {
        println!("No files specified to delete. Here are all tracked files:");
        crate::dotfile::list::print_dotfiles(None);
        return;
    }

    // Get the active managed folder
    let (folder_name, forge_path) = match config::get_active_managed_folder() {
        Ok(Some((name, path))) => (name, path),
        Ok(None) => {
            eprintln!("No managed folders found. Please run 'dotforge init' first.");
            return;
        }
        Err(e) => {
            eprintln!("Failed to get managed folder: {}", e);
            return;
        }
    };

    println!(
        "Using managed folder '{}' at {}",
        folder_name,
        forge_path.display()
    );

    // For each file, delete it completely
    for file in files {
        // Determine target path
        let target = if file.is_absolute() {
            // If absolute path, use it directly (might be a target)
            file.clone()
        } else if let Some(filename) = file.file_name() {
            // If relative, assume it's a filename to be combined with forge dir
            forge_path.join(filename)
        } else {
            eprintln!("Invalid file path: {}", file.display());
            continue;
        };

        // Try to find the dotfile in the database
        let dotfile = match config::find_dotfile_by_target(&target) {
            Ok(Some(df)) => df,
            Ok(None) => {
                println!("No tracking record found for {}", target.display());

                // Even if not tracked, confirm deletion
                if !skip_confirm {
                    let msg = format!(
                        "WARNING: This will PERMANENTLY DELETE the file {} from your system.\nThis action CANNOT be undone.",
                        file.display()
                    );
                    if !crate::utils::ui::confirm_with_text(&msg, "DELETE") {
                        println!("Deletion cancelled.");
                        continue;
                    }
                }

                match fs::remove_file(file) {
                    Ok(_) => println!("Deleted file: {}", file.display()),
                    Err(e) => println!("Failed to delete file {}: {}", file.display(), e),
                }
                continue;
            }
            Err(e) => {
                eprintln!("Error looking up dotfile: {}", e);
                continue;
            }
        };

        // Confirm with user
        if !skip_confirm {
            let msg = format!(
                "WARNING: This will PERMANENTLY DELETE the file from your system.\n\
                - The symlink at {} will be removed\n\
                - The file will be deleted from {}\n\
                - The file will be deleted from the forge folder\n\
                - All tracking information will be removed from the database\n\
                This action CANNOT be undone.",
                dotfile.source.display(),
                dotfile.source.display()
            );
            if !crate::utils::ui::confirm_with_text(&msg, "DELETE") {
                println!("Deletion cancelled.");
                continue;
            }
        }

        // Remove symlink if it exists
        if symlink::is_symlink(&dotfile.source) {
            if let Err(e) = fs::remove_file(&dotfile.source) {
                println!(
                    "Failed to remove symlink {}: {}",
                    dotfile.source.display(),
                    e
                );
                println!("Continuing with deletion...");
            }
        } else {
            // Remove original file if it's not a symlink
            if let Err(e) = fs::remove_file(&dotfile.source) {
                println!(
                    "Failed to remove original file {}: {}",
                    dotfile.source.display(),
                    e
                );
                println!("Continuing with deletion...");
            }
        }

        // Delete from forge folder
        match fs::remove_file(&dotfile.target) {
            Ok(_) => {
                println!(
                    "Deleted file from forge folder: {}",
                    dotfile.target.display()
                );

                // Remove from database
                match config::remove_dotfile(&dotfile.target) {
                    Ok(_) => println!("Removed tracking for {}", dotfile.target.display()),
                    Err(e) => eprintln!("Failed to update database: {}", e),
                }
            }
            Err(e) => println!("Failed to delete file {}: {}", dotfile.target.display(), e),
        }

        println!(
            "File {} has been completely deleted from the system.",
            dotfile.source.display()
        );
    }
}

pub mod profile {
    use crate::config;
    use std::fs;
    use std::path::PathBuf;

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
            }
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
        let target_path = PathBuf::from(&target_dir);

        // Create symlinks from profile directory to target
        match crate::symlink::create_symlinks(&profile_dir, &target_dir) {
            Ok(_) => {
                println!("Created symlinks from profile '{}' successfully", name);

                // Track the files in the database with the profile
                let mut success_count = 0;
                let mut error_count = 0;

                for entry in walkdir::WalkDir::new(&profile_dir)
                    .min_depth(1)
                    .max_depth(1)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_file())
                {
                    let source = entry.path();
                    let filename = entry.file_name();
                    let target = target_path.join(filename);

                    match config::add_dotfile(source, &target, Some(name)) {
                        Ok(_) => {
                            println!(
                                "Added to profile '{}': {} → {}",
                                name,
                                source.display(),
                                target.display()
                            );
                            success_count += 1;
                        }
                        Err(e) => {
                            eprintln!(
                                "Failed to add to profile '{}': {} → {}: {}",
                                name,
                                source.display(),
                                target.display(),
                                e
                            );
                            error_count += 1;
                        }
                    }
                }

                println!(
                    "\nProfile '{}' activated: {} files tracked, {} failed",
                    name, success_count, error_count
                );
            }
            Err(e) => println!("Error switching to profile '{}': {}", name, e),
        }
    }
}
