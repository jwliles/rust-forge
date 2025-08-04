// CLI command implementations
use crate::config;
use crate::symlink;
use crate::utils::path_utils;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir;

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

    // Create a .forge subfolder in the managed folder
    let forge_dir = init_dir.join(".forge");
    if !forge_dir.exists() {
        match fs::create_dir_all(&forge_dir) {
            Ok(_) => println!("Created .forge directory"),
            Err(e) => {
                eprintln!("Failed to create .forge directory: {}", e);
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
    stage_command(files, false, None)
}

/// Stage files to be tracked for symlinking
pub fn stage_command(files: &[PathBuf], recursive: bool, max_depth: Option<usize>) {
    if recursive {
        println!("Staging files and directories recursively");
    } else if let Some(depth) = max_depth {
        println!("Staging files and directories with max depth: {}", depth);
    } else {
        println!("Staging files/directories: {:?}", files);
    }

    // Get the active managed folder
    let (folder_name, forge_path) = match config::get_active_managed_folder() {
        Ok(Some((name, path))) => (name, path),
        Ok(None) => {
            eprintln!("No managed folders found. Please run 'forge init' first.");
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

    // Process each file or directory
    for file in files {
        // Normalize path
        let abs_source = path_utils::normalize(file);

        if !abs_source.exists() {
            eprintln!("Path does not exist: {}", abs_source.display());
            continue;
        }

        if abs_source.is_dir() {
            // Process directory
            if recursive || max_depth.is_some() {
                let walkdir_depth = match max_depth {
                    Some(depth) => depth,
                    None => usize::MAX, // Unlimited depth for recursive mode
                };

                println!(
                    "Processing directory: {} (max depth: {})",
                    abs_source.display(),
                    if walkdir_depth == usize::MAX {
                        "unlimited".to_string()
                    } else {
                        walkdir_depth.to_string()
                    }
                );

                // Use walkdir to recursively process directory
                for entry in walkdir::WalkDir::new(&abs_source)
                    .min_depth(1) // Skip the root dir itself
                    .max_depth(walkdir_depth)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_file())
                {
                    // Get the directory name to create proper nesting
                    let dir_name = abs_source.file_name().unwrap_or_default();

                    // Calculate relative path from original directory
                    let rel_path = entry
                        .path()
                        .strip_prefix(&abs_source)
                        .unwrap_or_else(|_| Path::new(entry.file_name()));

                    // Calculate target path in forge directory preserving subdirectories
                    // Ensure the top-level directory name is included
                    let target = forge_path.join(dir_name).join(rel_path);

                    // Ensure target parent directory exists
                    if let Some(parent) = target.parent() {
                        if !parent.exists() {
                            if let Err(e) = fs::create_dir_all(parent) {
                                eprintln!("Failed to create directory {}: {}", parent.display(), e);
                                continue;
                            }
                        }
                    }

                    // Skip existing targets
                    if target.exists() {
                        println!(
                            "Target already exists in forge folder: {}",
                            target.display()
                        );
                        continue;
                    }

                    // Create a symlink from forge folder TO original file (reverse of final state)
                    match symlink::create_symlink(entry.path(), &target) {
                        Ok(_) => {
                            println!(
                                "Created staging symlink: {} → {}",
                                target.display(),
                                entry.path().display()
                            );

                            // Add to database as staged
                            match config::stage_dotfile(entry.path(), &target, None) {
                                Ok(_) => println!(
                                    "Staged file: {} (use 'link' to make permanent)",
                                    entry.path().display()
                                ),
                                Err(e) => {
                                    eprintln!("Failed to stage {}: {}", entry.path().display(), e)
                                }
                            }
                        }
                        Err(e) => eprintln!(
                            "Failed to create staging symlink for {}: {}",
                            entry.path().display(),
                            e
                        ),
                    }
                }
            } else {
                println!(
                    "Skipping directory: {} (use --recursive or --depth to include contents)",
                    abs_source.display()
                );

                // Even with no recursion, we still stage the directory itself
                // Extract directory name for the target
                if let Some(dirname) = file.file_name() {
                    let target = forge_path.join(dirname);

                    // Check if target exists
                    if target.exists() {
                        println!(
                            "Target already exists in forge folder: {}",
                            target.display()
                        );
                        continue;
                    }

                    // Create the directory in the forge folder
                    if let Err(e) = fs::create_dir_all(&target) {
                        eprintln!("Failed to create directory {}: {}", target.display(), e);
                        continue;
                    }

                    println!("Created directory in forge folder: {}", target.display());

                    // Add to database as staged directory
                    match config::stage_dotfile(&abs_source, &target, None) {
                        Ok(_) => println!(
                            "Staged directory: {} (use 'link' to make permanent)",
                            abs_source.display()
                        ),
                        Err(e) => {
                            eprintln!("Failed to stage directory {}: {}", abs_source.display(), e)
                        }
                    }
                } else {
                    eprintln!("Invalid directory path: {}", file.display());
                }
            }
        } else {
            // Process regular file
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
            eprintln!("No managed folders found. Please run 'forge init' first.");
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

            // Check if it's a specific file or a directory name
            if abs_path.is_dir() {
                // If it's a directory, find all staged files under that directory
                match config::get_staged_dotfiles(None) {
                    Ok(all_dotfiles) => {
                        for df in all_dotfiles {
                            // Check if this file is within the specified directory
                            if df.is_staged() && df.source.starts_with(&abs_path) {
                                result.push(df);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error fetching staged files: {}", e);
                    }
                }

                // Also check if the directory itself is staged
                match config::find_dotfile_by_source(&abs_path) {
                    Ok(Some(df)) => {
                        if df.is_staged() {
                            result.push(df);
                        }
                    }
                    Ok(None) => {}
                    Err(e) => {
                        eprintln!("Error checking directory {}: {}", abs_path.display(), e);
                    }
                }

                // If using a relative path or just a directory name
                if !abs_path.is_absolute() {
                    let dir_name = file.file_name().unwrap_or_default();
                    let forge_dir_path = forge_path.join(dir_name);

                    // Check if dotfiles are in the forge directory with this name
                    match config::get_staged_dotfiles(None) {
                        Ok(all_dotfiles) => {
                            for df in all_dotfiles {
                                if df.is_staged() && df.target.starts_with(&forge_dir_path) {
                                    result.push(df);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error fetching staged files: {}", e);
                        }
                    }
                }

                if result.is_empty() {
                    eprintln!(
                        "No staged files found for directory: {}",
                        abs_path.display()
                    );
                }
            } else {
                // Original behavior for specific files
                match config::find_dotfile_by_target(&abs_path) {
                    Ok(Some(df)) => {
                        if df.is_staged() {
                            result.push(df);
                        } else {
                            println!("File already linked: {}", abs_path.display());
                        }
                    }
                    Ok(None) => {
                        // Try looking it up by source instead
                        match config::find_dotfile_by_source(&abs_path) {
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
                    Err(e) => {
                        eprintln!("Error checking file {}: {}", abs_path.display(), e);
                    }
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
        println!("---");
        println!("Processing dotfile:");
        println!("  Source: {}", dotfile.source.display());
        println!("  Target: {}", dotfile.target.display());
        if dotfile.source.is_dir() {
            println!("  Type: Directory");
            // For directories, we have a different process
            println!("Processing directory: {}", dotfile.source.display());

            // We can update the database status to linked but don't move the directory itself
            if let Err(e) = config::link_dotfile(&dotfile.source, &dotfile.target) {
                eprintln!("Failed to update directory tracking status: {}", e);
                error_count += 1;
            } else {
                println!("Directory tracking status updated to linked.");
                success_count += 1;
            }

            // Continue to the next item
            continue;
        } else {
            println!("  Type: File");
        }

        // Process files - first check if target is a file or symlink (not a dir)
        if dotfile.target.is_file() || symlink::is_symlink(&dotfile.target) {
            println!(
                "  Target is a file or symlink, removing staging symlink: {}",
                dotfile.target.display()
            );
            if let Err(e) = fs::remove_file(&dotfile.target) {
                eprintln!(
                    "Failed to remove staging symlink {}: {}",
                    dotfile.target.display(),
                    e
                );
                error_count += 1;
                continue;
            } else {
                println!("  Removed staging symlink: {}", dotfile.target.display());
            }
        } else if dotfile.target.is_dir() {
            // For directory targets, just skip removal
            println!(
                "  Target is a directory, skipping removal: {}",
                dotfile.target.display()
            );
        }

        // Ensure target parent directory exists
        if let Some(parent) = dotfile.target.parent() {
            if !parent.exists() {
                println!("  Creating parent directory: {}", parent.display());
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("Failed to create directory {}: {}", parent.display(), e);
                    error_count += 1;
                    continue;
                }
            }
        }

        // Move the original file to the forge directory
        println!(
            "  Copying file from {} to {}",
            dotfile.source.display(),
            dotfile.target.display()
        );
        // If the target file in the forge directory exists, back it up before copying
        if dotfile.target.exists() {
            let backup_path = dotfile.target.with_extension("bak");
            println!(
                "  Target file {} exists, backing up to {}",
                dotfile.target.display(),
                backup_path.display()
            );
            if let Err(e) = fs::rename(&dotfile.target, &backup_path) {
                eprintln!(
                    "Failed to back up existing target file: {}: {}",
                    dotfile.target.display(),
                    e
                );
                error_count += 1;
                continue;
            }
        }
        match fs::copy(&dotfile.source, &dotfile.target) {
            Ok(_) => {
                println!("  File copied successfully.");
                // Step 1: Validate copy succeeded (file exists at target and matches size)
                let src_metadata = fs::metadata(&dotfile.source);
                let tgt_metadata = fs::metadata(&dotfile.target);
                let copy_valid = match (&src_metadata, &tgt_metadata) {
                    (Ok(src), Ok(tgt)) => src.len() == tgt.len(),
                    _ => false,
                };
                if !copy_valid {
                    eprintln!(
                        "  Copy validation failed: source and target file sizes do not match or metadata error."
                    );
                    error_count += 1;
                    continue;
                }
                // Create symlink from original location to forge directory FIRST
                println!(
                    "  Creating symlink from {} to {}",
                    dotfile.source.display(),
                    dotfile.target.display()
                );
                // Remove the original file only after confirming the copy
                if dotfile.source.exists() {
                    println!("  Removing original file: {}", dotfile.source.display());
                    if let Err(e) = fs::remove_file(&dotfile.source) {
                        eprintln!(
                            "Failed to remove original file after copy: {}: {}",
                            dotfile.source.display(),
                            e
                        );
                        error_count += 1;
                        continue;
                    } else {
                        println!("  Original file removed.");
                    }
                }
                // Final check: if the original file still exists, abort before symlinking
                if dotfile.source.exists() {
                    eprintln!(
                        "  ERROR: Original file still exists at symlink location ({}), cannot create symlink. File removal may have failed.",
                        dotfile.source.display()
                    );
                    error_count += 1;
                    continue;
                }
                match symlink::create_symlink(&dotfile.target, &dotfile.source) {
                    Ok(_) => {
                        println!("  Symlink created successfully.");
                        println!(
                            "Created symlink: {} → {}",
                            dotfile.source.display(),
                            dotfile.target.display()
                        );

                        // Update database status to linked
                        if let Err(e) = config::link_dotfile(&dotfile.source, &dotfile.target) {
                            eprintln!("Failed to update database: {}", e);
                        } else {
                            println!("  Database status updated to linked.");
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
            eprintln!("No managed folders found. Please run 'forge init' first.");
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
            eprintln!("No managed folders found. Please run 'forge init' first.");
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
            eprintln!("No managed folders found. Please run 'forge init' first.");
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

pub mod pack;

pub mod profile {
    use crate::config;
    use std::fs;
    use std::path::PathBuf;

    const PROFILES_DIR: &str = ".forge/profiles";

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

/// Unstage (deactivate) staged files by target path

/// Unstage (deactivate) staged files by target path, with optional recursive support
pub fn unstage_command(files: &[PathBuf], recursive: bool, max_depth: Option<usize>) {
    let staged_dotfiles = match config::get_staged_dotfiles(None) {
        Ok(df) => df,
        Err(e) => {
            eprintln!("Failed to fetch staged files: {}", e);
            return;
        }
    };
    if files.is_empty() {
        // Unstage all staged files in a single batch operation
        let targets: Vec<_> = staged_dotfiles.iter().map(|df| df.target.clone()).collect();
        match config::deactivate_dotfiles(&targets) {
            Ok(count) => {
                println!("Unstaged {} files.", count);
                // Remove the staging symlinks in the forge directory
                for dotfile in &staged_dotfiles {
                    if let Err(e) = std::fs::remove_file(&dotfile.target) {
                        if e.kind() != std::io::ErrorKind::NotFound {
                            eprintln!(
                                "Failed to remove staging symlink {}: {}",
                                dotfile.target.display(),
                                e
                            );
                        }
                    } else {
                        println!("Removed staging symlink: {}", dotfile.target.display());
                    }
                }
            }
            Err(e) => eprintln!("Failed to unstage files: {}", e),
        }
        return;
    }
    for file in files {
        let abs_path = path_utils::normalize(file);
        if abs_path.is_dir() && recursive {
            let walkdir_depth = max_depth.unwrap_or(usize::MAX);
            for entry in walkdir::WalkDir::new(&abs_path)
                .min_depth(1)
                .max_depth(walkdir_depth)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
            {
                let entry_path = entry.path();
                for dotfile in &staged_dotfiles {
                    if dotfile.source == entry_path {
                        match config::deactivate_dotfile(&dotfile.target) {
                            Ok(true) => println!("Unstaged: {}", dotfile.target.display()),
                            Ok(false) => println!(
                                "File not found or already inactive: {}",
                                dotfile.target.display()
                            ),
                            Err(e) => {
                                println!("Failed to unstage {}: {}", dotfile.target.display(), e)
                            }
                        }
                    }
                }
            }
        } else {
            // Unstage the file directly
            for dotfile in &staged_dotfiles {
                if dotfile.source == abs_path || dotfile.target == abs_path {
                    match config::deactivate_dotfile(&dotfile.target) {
                        Ok(true) => println!("Unstaged: {}", dotfile.target.display()),
                        Ok(false) => println!(
                            "File not found or already inactive: {}",
                            dotfile.target.display()
                        ),
                        Err(e) => println!("Failed to unstage {}: {}", dotfile.target.display(), e),
                    }
                }
            }
        }
    }
}

/// Purge all dotfile records (staged or managed) and forge files for a specified folder
pub fn purge_command(folder: &Path, recursive: bool) {
    let abs_folder = path_utils::normalize(folder);
    println!(
        "Purging all dotfile records and forge files for folder: {}",
        abs_folder.display()
    );
    // Remove from database
    match crate::config::purge_dotfiles_in_folder(&abs_folder, recursive) {
        Ok(count) => println!("Purged {} database records.", count),
        Err(e) => eprintln!("Failed to purge database records: {}", e),
    }
    // Remove files in forge directory
    // (Optional: implement if desired, or prompt user)
}

/// Purge all dotfile records and managed files for a specified folder, restoring originals to prevent data loss
pub fn purge_command_safe(folder: &Path, recursive: bool) {
    let abs_folder = path_utils::normalize(folder);
    println!(
        "Safely purging all dotfile records and managed files for folder: {}",
        abs_folder.display()
    );
    // Get all dotfiles (staged or linked) under the folder
    let dotfiles = match crate::config::get_dotfiles_in_folder(&abs_folder, recursive) {
        Ok(df) => df,
        Err(e) => {
            eprintln!("Failed to fetch dotfiles: {}", e);
            return;
        }
    };
    for dotfile in &dotfiles {
        // If the original location is a symlink to the managed file, restore the real file
        if dotfile.source.is_symlink() {
            match std::fs::read_link(&dotfile.source) {
                Ok(target_path) if target_path == dotfile.target => {
                    // Remove the symlink at the original location
                    if let Err(e) = std::fs::remove_file(&dotfile.source) {
                        eprintln!(
                            "Failed to remove symlink {}: {}",
                            dotfile.source.display(),
                            e
                        );
                        continue;
                    }
                    // Copy the managed file back to the original location
                    if let Err(e) = std::fs::copy(&dotfile.target, &dotfile.source) {
                        eprintln!(
                            "Failed to restore file from managed folder: {} -> {}: {}",
                            dotfile.target.display(),
                            dotfile.source.display(),
                            e
                        );
                        continue;
                    }
                    println!("Restored and removed symlink: {}", dotfile.source.display());
                }
                _ => {}
            }
        }
        // Remove the managed file
        if let Some(name) = dotfile.target.file_name().and_then(|n| n.to_str()) {
            if name.starts_with('.') || name == ".forge" {
                continue;
            }
        }
        if let Err(e) = std::fs::remove_file(&dotfile.target) {
            if e.kind() != std::io::ErrorKind::NotFound {
                eprintln!(
                    "Failed to remove managed file {}: {}",
                    dotfile.target.display(),
                    e
                );
            }
        } else {
            println!("Removed managed file: {}", dotfile.target.display());
        }
    }
    // Purge from database
    match crate::config::purge_dotfiles_in_folder(&abs_folder, recursive) {
        Ok(count) => println!("Purged {} database records.", count),
        Err(e) => eprintln!("Failed to purge database records: {}", e),
    }
    // Recursively remove all empty directories under the managed folder
    fn remove_empty_dirs(path: &std::path::Path) {
        if path.is_dir() {
            let entries = match std::fs::read_dir(path) {
                Ok(e) => e,
                Err(_) => return,
            };
            for entry in entries {
                if let Ok(entry) = entry {
                    let p = entry.path();
                    remove_empty_dirs(&p);
                }
            }
            // Try to remove the directory (will only succeed if empty)
            let _ = std::fs::remove_dir(path);
        }
    }
    remove_empty_dirs(&abs_folder);
    println!(
        "All files and directories under {} have been purged.",
        abs_folder.display()
    );
}
