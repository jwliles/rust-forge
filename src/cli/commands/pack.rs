// Pack-and-Go functionality for Forge
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};
use crate::config;
use crate::utils::path_utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackManifest {
    pub version: String,
    pub scope: String,
    pub created: DateTime<Utc>,
    pub files: HashMap<String, PackFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackFile {
    pub target_path: String,
    pub relative_path: String,
    pub hash: Option<String>,
    pub size: u64,
    pub modified: DateTime<Utc>,
}

impl PackManifest {
    pub fn new(scope: &str) -> Self {
        Self {
            version: "1.0".to_string(),
            scope: scope.to_string(),
            created: Utc::now(),
            files: HashMap::new(),
        }
    }
    
    pub fn add_file(&mut self, target_path: &Path, relative_path: &Path, hash: Option<String>) -> Result<()> {
        let metadata = fs::metadata(target_path)?;
        let modified = metadata.modified()?;
        let modified_dt = DateTime::<Utc>::from(modified);
        
        let pack_file = PackFile {
            target_path: target_path.to_string_lossy().to_string(),
            relative_path: relative_path.to_string_lossy().to_string(),
            hash,
            size: metadata.len(),
            modified: modified_dt,
        };
        
        self.files.insert(
            target_path.to_string_lossy().to_string(),
            pack_file
        );
        
        Ok(())
    }
}

/// Get the pack staging directory for a given scope
fn get_pack_staging_dir(scope: &str) -> Result<PathBuf> {
    let (_, forge_path) = config::get_active_managed_folder()?
        .ok_or_else(|| anyhow!("No managed folders found. Please run 'forge init' first."))?;
    
    let staging_dir = forge_path.join(".forge").join("tmp").join("pack").join(scope);
    Ok(staging_dir)
}

/// Get the pack archives directory
fn get_pack_archives_dir() -> Result<PathBuf> {
    let (_, forge_path) = config::get_active_managed_folder()?
        .ok_or_else(|| anyhow!("No managed folders found. Please run 'forge init' first."))?;
    
    let archives_dir = forge_path.join(".forge").join("archives");
    Ok(archives_dir)
}

/// Get default scope name from current working directory
fn get_default_scope() -> Result<String> {
    let cwd = env::current_dir()?;
    let scope = cwd.file_name()
        .ok_or_else(|| anyhow!("Could not determine directory name for default scope"))?
        .to_string_lossy()
        .to_string();
    Ok(scope)
}

/// Start packing files for a given scope
pub fn start_packing(scope: &str) {
    println!("Starting pack creation for scope: {}", scope);
    
    match start_packing_impl(scope) {
        Ok(_) => {
            println!("Pack staging area created successfully for '{}'", scope);
            println!("Use 'forge pack <file>' to add files to this pack.");
        }
        Err(e) => {
            eprintln!("Failed to start packing: {}", e);
        }
    }
}

fn start_packing_impl(scope: &str) -> Result<()> {
    let staging_dir = get_pack_staging_dir(scope)?;
    
    // Check if pack already exists
    if staging_dir.exists() {
        return Err(anyhow!("Pack '{}' already exists. Use 'forge pack' to add files or 'forge seal' to finalize.", scope));
    }
    
    // Create staging directory structure
    let files_dir = staging_dir.join("files");
    fs::create_dir_all(&files_dir)?;
    
    // Create initial manifest
    let manifest = PackManifest::new(scope);
    let manifest_path = staging_dir.join("manifest.toml");
    let manifest_content = toml::to_string_pretty(&manifest)?;
    fs::write(&manifest_path, manifest_content)?;
    
    println!("Created staging directory: {}", staging_dir.display());
    println!("Created manifest: {}", manifest_path.display());
    
    Ok(())
}

/// Add files to an existing pack
pub fn pack_files(files: &[PathBuf], scope: Option<&str>) {
    let scope = match scope {
        Some(s) => s,
        None => match get_default_scope() {
            Ok(s) => Box::leak(s.into_boxed_str()),
            Err(_) => {
                eprintln!("Could not determine scope. Please specify with --scope");
                return;
            }
        }
    };
    
    println!("Adding {} files to pack '{}'", files.len(), scope);
    
    match pack_files_impl(files, scope) {
        Ok(count) => {
            println!("Successfully added {} files to pack '{}'", count, scope);
            println!("Use 'forge seal' to create the final archive.");
        }
        Err(e) => {
            eprintln!("Failed to pack files: {}", e);
        }
    }
}

fn pack_files_impl(files: &[PathBuf], scope: &str) -> Result<usize> {
    let staging_dir = get_pack_staging_dir(scope)?;
    
    if !staging_dir.exists() {
        return Err(anyhow!("Pack '{}' does not exist. Use 'forge start packing {}' first.", scope, scope));
    }
    
    // Load existing manifest
    let manifest_path = staging_dir.join("manifest.toml");
    let mut manifest: PackManifest = if manifest_path.exists() {
        let content = fs::read_to_string(&manifest_path)?;
        toml::from_str(&content)?
    } else {
        PackManifest::new(scope)
    };
    
    let files_dir = staging_dir.join("files");
    let mut added_count = 0;
    
    for file in files {
        let abs_source = path_utils::normalize(file);
        
        if !abs_source.exists() {
            eprintln!("File does not exist: {}", abs_source.display());
            continue;
        }
        
        if abs_source.is_dir() {
            eprintln!("Directories not yet supported: {}", abs_source.display());
            continue;
        }
        
        // Generate relative path for storage
        let filename = abs_source.file_name()
            .ok_or_else(|| anyhow!("Invalid filename: {}", abs_source.display()))?;
        let relative_path = Path::new(filename);
        let target_in_pack = files_dir.join(&relative_path);
        
        // Check if already exists
        if target_in_pack.exists() {
            println!("File already in pack: {}", relative_path.display());
            continue;
        }
        
        // Copy file to pack
        if let Some(parent) = target_in_pack.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&abs_source, &target_in_pack)?;
        
        // Calculate hash
        let hash = calculate_file_hash(&target_in_pack)?;
        
        // Add to manifest
        manifest.add_file(&abs_source, &relative_path, Some(hash))?;
        
        println!("Packed: {} → {}", abs_source.display(), relative_path.display());
        added_count += 1;
    }
    
    // Save updated manifest
    let manifest_content = toml::to_string_pretty(&manifest)?;
    fs::write(&manifest_path, manifest_content)?;
    
    Ok(added_count)
}

/// Calculate BLAKE3 hash of a file
fn calculate_file_hash(path: &Path) -> Result<String> {
    let content = fs::read(path)?;
    let hash = blake3::hash(&content);
    Ok(hash.to_hex().to_string())
}

/// Seal a pack into a portable archive
pub fn seal_pack(scope: Option<&str>) {
    let scope = match scope {
        Some(s) => s,
        None => match get_default_scope() {
            Ok(s) => Box::leak(s.into_boxed_str()),
            Err(_) => {
                eprintln!("Could not determine scope. Please specify with --scope");
                return;
            }
        }
    };
    
    println!("Sealing pack: {}", scope);
    
    match seal_pack_impl(scope) {
        Ok(archive_path) => {
            println!("Pack sealed successfully: {}", archive_path.display());
            println!("Use 'forge install {}' on another system to install this pack.", archive_path.display());
        }
        Err(e) => {
            eprintln!("Failed to seal pack: {}", e);
        }
    }
}

fn seal_pack_impl(scope: &str) -> Result<PathBuf> {
    let staging_dir = get_pack_staging_dir(scope)?;
    
    if !staging_dir.exists() {
        return Err(anyhow!("Pack '{}' does not exist. Use 'forge start packing {}' first.", scope, scope));
    }
    
    // Create archives directory
    let archives_dir = get_pack_archives_dir()?;
    fs::create_dir_all(&archives_dir)?;
    
    // Generate archive filename with timestamp
    let now = Utc::now();
    let timestamp = now.format("%Y-%m-%d");
    let archive_name = format!("{}-{}.zip", scope, timestamp);
    let archive_path = archives_dir.join(&archive_name);
    
    // Create ZIP archive
    create_zip_archive(&staging_dir, &archive_path)?;
    
    // Clean up staging directory
    fs::remove_dir_all(&staging_dir)?;
    println!("Cleaned up staging directory: {}", staging_dir.display());
    
    Ok(archive_path)
}

/// Create a ZIP archive from the staging directory
fn create_zip_archive(staging_dir: &Path, archive_path: &Path) -> Result<()> {
    use std::io::Write;
    use zip::write::FileOptions;
    
    let file = fs::File::create(archive_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);
    
    // Add manifest
    let manifest_path = staging_dir.join("manifest.toml");
    if manifest_path.exists() {
        zip.start_file("manifest.toml", options)?;
        let manifest_content = fs::read(&manifest_path)?;
        zip.write_all(&manifest_content)?;
    }
    
    // Add all files from files/ directory
    let files_dir = staging_dir.join("files");
    if files_dir.exists() {
        for entry in walkdir::WalkDir::new(&files_dir) {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                let relative_path = path.strip_prefix(&files_dir)?;
                let zip_path = format!("files/{}", relative_path.to_string_lossy());
                
                zip.start_file(&zip_path, options)?;
                let content = fs::read(path)?;
                zip.write_all(&content)?;
            }
        }
    }
    
    zip.finish()?;
    Ok(())
}

/// Install a sealed pack on this system
pub fn install_pack(archive: &Path, force: bool) {
    println!("Installing pack from: {}", archive.display());
    
    match install_pack_impl(archive, force) {
        Ok(count) => {
            println!("Successfully installed {} files from pack", count);
        }
        Err(e) => {
            eprintln!("Failed to install pack: {}", e);
        }
    }
}

fn install_pack_impl(archive: &Path, force: bool) -> Result<usize> {
    if !archive.exists() {
        return Err(anyhow!("Archive does not exist: {}", archive.display()));
    }
    
    // Extract archive to temporary directory
    let temp_dir = tempfile::tempdir()?;
    extract_zip_archive(archive, temp_dir.path())?;
    
    // Read manifest
    let manifest_path = temp_dir.path().join("manifest.toml");
    if !manifest_path.exists() {
        return Err(anyhow!("Invalid pack archive: missing manifest.toml"));
    }
    
    let manifest_content = fs::read_to_string(&manifest_path)?;
    let manifest: PackManifest = toml::from_str(&manifest_content)?;
    
    println!("Installing pack '{}' created on {}", manifest.scope, manifest.created.format("%Y-%m-%d %H:%M:%S UTC"));
    
    let files_dir = temp_dir.path().join("files");
    let mut installed_count = 0;
    
    for (_, pack_file) in &manifest.files {
        let source_in_archive = files_dir.join(&pack_file.relative_path);
        let target_path = Path::new(&pack_file.target_path);
        
        if !source_in_archive.exists() {
            eprintln!("File missing from archive: {}", pack_file.relative_path);
            continue;
        }
        
        // Check for conflicts
        if target_path.exists() && !force {
            println!("Conflict: {} already exists (use --force to overwrite)", target_path.display());
            continue;
        }
        
        // Validate hash if available
        if let Some(expected_hash) = &pack_file.hash {
            let actual_hash = calculate_file_hash(&source_in_archive)?;
            if actual_hash != *expected_hash {
                eprintln!("Hash mismatch for {}: expected {}, got {}", 
                    pack_file.relative_path, expected_hash, actual_hash);
                continue;
            }
        }
        
        // Create target directory if needed
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Copy file and create symlink
        fs::copy(&source_in_archive, target_path)?;
        
        println!("Installed: {}", target_path.display());
        installed_count += 1;
    }
    
    Ok(installed_count)
}

/// Extract ZIP archive to a directory
fn extract_zip_archive(archive: &Path, target_dir: &Path) -> Result<()> {
    
    let file = fs::File::open(archive)?;
    let mut zip = zip::ZipArchive::new(file)?;
    
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let outpath = target_dir.join(file.name());
        
        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    
    Ok(())
}

/// Update files in an existing pack (repack)
pub fn repack_files(scope: Option<&str>, files: &[PathBuf]) {
    let scope = match scope {
        Some(s) => s,
        None => match get_default_scope() {
            Ok(s) => Box::leak(s.into_boxed_str()),
            Err(_) => {
                eprintln!("Could not determine scope. Please specify with --scope");
                return;
            }
        }
    };
    
    println!("Repacking files in scope: {}", scope);
    
    match repack_files_impl(scope, files) {
        Ok(count) => {
            println!("Successfully repacked {} files", count);
        }
        Err(e) => {
            eprintln!("Failed to repack files: {}", e);
        }
    }
}

fn repack_files_impl(scope: &str, files: &[PathBuf]) -> Result<usize> {
    let staging_dir = get_pack_staging_dir(scope)?;
    
    if !staging_dir.exists() {
        return Err(anyhow!("Pack '{}' does not exist. Use 'forge start packing {}' first.", scope, scope));
    }
    
    if files.is_empty() {
        // Repack all files in manifest
        let manifest_path = staging_dir.join("manifest.toml");
        let content = fs::read_to_string(&manifest_path)?;
        let manifest: PackManifest = toml::from_str(&content)?;
        
        let files_to_repack: Vec<PathBuf> = manifest.files.keys()
            .map(|s| PathBuf::from(s))
            .collect();
        
        pack_files_impl(&files_to_repack, scope)
    } else {
        pack_files_impl(files, scope)
    }
}

/// Remove files from a pack
pub fn unpack_files(files: &[PathBuf], scope: Option<&str>) {
    let scope = match scope {
        Some(s) => s,
        None => match get_default_scope() {
            Ok(s) => Box::leak(s.into_boxed_str()),
            Err(_) => {
                eprintln!("Could not determine scope. Please specify with --scope");
                return;
            }
        }
    };
    
    println!("Removing {} files from pack '{}'", files.len(), scope);
    
    match unpack_files_impl(files, scope) {
        Ok(count) => {
            println!("Successfully removed {} files from pack", count);
        }
        Err(e) => {
            eprintln!("Failed to unpack files: {}", e);
        }
    }
}

fn unpack_files_impl(files: &[PathBuf], scope: &str) -> Result<usize> {
    let staging_dir = get_pack_staging_dir(scope)?;
    
    if !staging_dir.exists() {
        return Err(anyhow!("Pack '{}' does not exist.", scope));
    }
    
    // Load manifest
    let manifest_path = staging_dir.join("manifest.toml");
    let mut manifest: PackManifest = if manifest_path.exists() {
        let content = fs::read_to_string(&manifest_path)?;
        toml::from_str(&content)?
    } else {
        return Err(anyhow!("Pack manifest not found"));
    };
    
    let files_dir = staging_dir.join("files");
    let mut removed_count = 0;
    
    for file in files {
        let abs_path = path_utils::normalize(file);
        let key = abs_path.to_string_lossy().to_string();
        
        if let Some(pack_file) = manifest.files.remove(&key) {
            let file_in_pack = files_dir.join(&pack_file.relative_path);
            
            if file_in_pack.exists() {
                fs::remove_file(&file_in_pack)?;
                println!("Removed from pack: {}", pack_file.relative_path);
                removed_count += 1;
            }
        } else {
            println!("File not found in pack: {}", abs_path.display());
        }
    }
    
    // Save updated manifest
    let manifest_content = toml::to_string_pretty(&manifest)?;
    fs::write(&manifest_path, manifest_content)?;
    
    Ok(removed_count)
}