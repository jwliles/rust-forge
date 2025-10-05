use rusqlite::Connection;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

// Constants for configuration file paths
const DEFAULT_CONFIG_DIR: &str = ".forge";
const DEFAULT_PATH_FILE: &str = "default_path";
const FILETYPES_FILE: &str = "filetypes";
const IGNORED_PATHS_FILE: &str = "ignored_paths";
const MANAGED_FOLDERS_FILE: &str = "managed_folders";
const DEFAULT_MANAGED_FOLDER: &str = "default";

pub struct Config {
    db_path: PathBuf,
    connection: Option<Connection>,
    // File-based config paths
    config_dir: PathBuf,
    default_path_file: PathBuf,
    filetypes_file: PathBuf,
    ignored_paths_file: PathBuf,
    managed_folders_file: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        // Check for test environment variable to isolate database
        let db_path = if let Ok(test_db) = std::env::var("FORGE_TEST_DB") {
            PathBuf::from(test_db)
        } else {
            let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
            db_path.push("forge");
            db_path.push("forge.db");
            db_path
        };

        // Check for test config directory override
        let config_dir = if let Ok(test_config) = std::env::var("FORGE_TEST_CONFIG_DIR") {
            PathBuf::from(test_config)
        } else {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(DEFAULT_CONFIG_DIR)
        };

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            if let Err(e) = fs::create_dir_all(&config_dir) {
                eprintln!("Failed to create config directory: {}", e);
            }
        }

        // Generate file paths
        let default_path_file = config_dir.join(DEFAULT_PATH_FILE);
        let filetypes_file = config_dir.join(FILETYPES_FILE);
        let ignored_paths_file = config_dir.join(IGNORED_PATHS_FILE);
        let managed_folders_file = config_dir.join(MANAGED_FOLDERS_FILE);

        // Ensure config directory for db exists
        if let Some(parent) = db_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("Failed to create database directory: {}", e);
                }
            }
        }

        Self {
            db_path,
            connection: None,
            config_dir,
            default_path_file,
            filetypes_file,
            ignored_paths_file,
            managed_folders_file,
        }
    }

    /// Create a Config with custom paths - primarily for testing
    #[cfg(test)]
    pub fn with_custom_paths(db_path: PathBuf, config_dir: PathBuf) -> Self {
        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            if let Err(e) = fs::create_dir_all(&config_dir) {
                eprintln!("Failed to create config directory: {}", e);
            }
        }

        // Generate file paths
        let default_path_file = config_dir.join(DEFAULT_PATH_FILE);
        let filetypes_file = config_dir.join(FILETYPES_FILE);
        let ignored_paths_file = config_dir.join(IGNORED_PATHS_FILE);
        let managed_folders_file = config_dir.join(MANAGED_FOLDERS_FILE);

        // Ensure database directory exists
        if let Some(parent) = db_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("Failed to create database directory: {}", e);
                }
            }
        }

        Self {
            db_path,
            connection: None,
            config_dir,
            default_path_file,
            filetypes_file,
            ignored_paths_file,
            managed_folders_file,
        }
    }

    pub fn connect(&mut self) -> rusqlite::Result<()> {
        // Connect to the database
        self.connection = Some(Connection::open(&self.db_path)?);

        // Initialize database if needed
        self.init_database()?;

        Ok(())
    }

    // Initialize database tables if they don't exist
    fn init_database(&self) -> rusqlite::Result<()> {
        if let Some(conn) = &self.connection {
            // Create dotfiles table
            conn.execute(
                "CREATE TABLE IF NOT EXISTS dotfiles (
                    id INTEGER PRIMARY KEY,
                    source TEXT NOT NULL,
                    target TEXT NOT NULL,
                    profile TEXT,
                    status TEXT NOT NULL DEFAULT 'staged',
                    active BOOLEAN NOT NULL DEFAULT 1,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )",
                [],
            )?;

            // Create settings table
            conn.execute(
                "CREATE TABLE IF NOT EXISTS settings (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                )",
                [],
            )?;

            // Insert default settings if they don't exist
            let default_path = self.read_default_path();
            let count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM settings WHERE key = 'default_path'",
                [],
                |row| row.get(0),
            )?;

            if count == 0 {
                conn.execute(
                    "INSERT INTO settings (key, value) VALUES (?, ?)",
                    ["default_path", &default_path],
                )?;
            }

            // Create filetypes table
            conn.execute(
                "CREATE TABLE IF NOT EXISTS filetypes (
                    extension TEXT PRIMARY KEY,
                    active BOOLEAN NOT NULL DEFAULT 1,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )",
                [],
            )?;

            // Create ignored_paths table
            conn.execute(
                "CREATE TABLE IF NOT EXISTS ignored_paths (
                    path TEXT PRIMARY KEY,
                    active BOOLEAN NOT NULL DEFAULT 1,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )",
                [],
            )?;

            // Initialize with default filetypes if table is empty
            let count: i64 =
                conn.query_row("SELECT COUNT(*) FROM filetypes", [], |row| row.get(0))?;

            if count == 0 {
                let default_filetypes = [".bashrc", ".zshrc", ".vimrc", ".tmux.conf"];
                for ext in default_filetypes.iter() {
                    conn.execute("INSERT INTO filetypes (extension) VALUES (?)", [ext])?;
                }
            }

            // Import existing filetypes from file
            if self.filetypes_file.exists() {
                match self.read_lines(&self.filetypes_file) {
                    Ok(filetypes) => {
                        for ext in filetypes {
                            conn.execute(
                                "INSERT OR IGNORE INTO filetypes (extension) VALUES (?)",
                                [&ext],
                            )?;
                        }
                    }
                    Err(e) => eprintln!("Failed to read filetypes file: {}", e),
                }
            }

            // Import existing ignored paths from file
            if self.ignored_paths_file.exists() {
                match self.read_lines(&self.ignored_paths_file) {
                    Ok(paths) => {
                        for path in paths {
                            conn.execute(
                                "INSERT OR IGNORE INTO ignored_paths (path) VALUES (?)",
                                [&path],
                            )?;
                        }
                    }
                    Err(e) => eprintln!("Failed to read ignored paths file: {}", e),
                }
            }
        }

        Ok(())
    }

    // Get default path file
    pub fn get_default_path_file(&self) -> &PathBuf {
        &self.default_path_file
    }

    // Get filetypes file
    pub fn get_filetypes_file(&self) -> &PathBuf {
        &self.filetypes_file
    }

    // Get ignored paths file
    pub fn get_ignored_paths_file(&self) -> &PathBuf {
        &self.ignored_paths_file
    }

    // Set default path
    pub fn set_default_path(&self, path: &str) -> io::Result<()> {
        fs::write(&self.default_path_file, path)
    }

    // Read default path
    pub fn read_default_path(&self) -> String {
        match fs::read_to_string(&self.default_path_file) {
            Ok(content) => content.trim().to_string(),
            Err(_) => String::from("~/.forge"),
        }
    }

    // Add file types
    pub fn add_filetypes(&self, extensions: &[String]) -> io::Result<()> {
        for ext in extensions {
            let ext = ext.trim();
            if ext.is_empty() {
                continue;
            }

            if self.file_exists(&self.filetypes_file, ext)? {
                println!("File type '{}' is already approved.", ext);
            } else {
                self.append_to_file(&self.filetypes_file, ext)?;
                println!("File type '{}' added to the approved list.", ext);
            }
        }
        Ok(())
    }

    // Remove file types
    pub fn remove_filetypes(&self, extensions: &[String]) -> io::Result<()> {
        for ext in extensions {
            self.remove_item_from_list(&self.filetypes_file, ext)?;
        }
        Ok(())
    }

    // List file types
    pub fn list_filetypes(&self) -> io::Result<()> {
        self.list_items(&self.filetypes_file, "Approved File Types")
    }

    // Add ignored paths
    pub fn add_ignored_paths(&self, paths: &[String]) -> io::Result<()> {
        for path in paths {
            let abs_path = crate::utils::path_utils::normalize(path);
            let abs_path_str = abs_path.to_string_lossy().to_string();

            if self.file_exists(&self.ignored_paths_file, &abs_path_str)? {
                println!("Path '{}' is already blocked.", abs_path_str);
            } else {
                self.append_to_file(&self.ignored_paths_file, &abs_path_str)?;
                println!("Path '{}' added to the blocked list.", abs_path_str);
            }
        }
        Ok(())
    }

    // Remove ignored paths
    pub fn remove_ignored_paths(&self, paths: &[String]) -> io::Result<()> {
        for path in paths {
            self.remove_item_from_list(&self.ignored_paths_file, path)?;
        }
        Ok(())
    }

    // List ignored paths
    pub fn list_ignored_paths(&self) -> io::Result<()> {
        self.list_items(&self.ignored_paths_file, "Blocked Paths")
    }

    // ---- Managed Folders operations ----

    // Get the managed folders file path
    pub fn get_managed_folders_file(&self) -> &PathBuf {
        &self.managed_folders_file
    }

    // Add a managed folder
    pub fn add_managed_folder(&self, name: &str, path: &Path) -> io::Result<()> {
        let entry = format!("{}:{}", name, path.to_string_lossy());

        // Check if entry already exists (by name)
        let managed_folders = self.get_managed_folders()?;
        if managed_folders.iter().any(|(n, _)| n == name) {
            println!("Managed folder '{}' already exists", name);
            return Ok(());
        }

        // Add the entry
        self.append_to_file(&self.managed_folders_file, &entry)
    }

    // Get managed folders (name, path)
    pub fn get_managed_folders(&self) -> io::Result<Vec<(String, PathBuf)>> {
        let lines = self.read_lines(&self.managed_folders_file)?;
        let mut folders = Vec::new();

        for line in lines {
            if let Some((name, path_str)) = line.split_once(':') {
                folders.push((name.to_string(), PathBuf::from(path_str)));
            }
        }

        Ok(folders)
    }

    // Check if a path is a managed folder
    pub fn is_managed_folder(&self, path: &Path) -> io::Result<bool> {
        let managed_folders = self.get_managed_folders()?;
        Ok(managed_folders.iter().any(|(_, p)| p == path))
    }

    // Get managed folder by name
    pub fn get_managed_folder_by_name(&self, name: &str) -> io::Result<Option<PathBuf>> {
        let managed_folders = self.get_managed_folders()?;
        Ok(managed_folders
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, p)| p.clone()))
    }

    // Get the current active managed folder (for now, just get the default or first)
    pub fn get_active_managed_folder(&self) -> io::Result<Option<(String, PathBuf)>> {
        let managed_folders = self.get_managed_folders()?;

        // First look for the default managed folder
        if let Some(default) = managed_folders
            .iter()
            .find(|(name, _)| name == DEFAULT_MANAGED_FOLDER)
        {
            return Ok(Some((default.0.clone(), default.1.clone())));
        }

        // If no default, return the first one
        Ok(managed_folders.first().map(|(n, p)| (n.clone(), p.clone())))
    }

    // Helper functions

    // Check if file exists and contains item
    fn file_exists<P: AsRef<Path>>(&self, file_path: P, item: &str) -> io::Result<bool> {
        let lines = self.read_lines(file_path)?;
        Ok(lines.iter().any(|line| line.trim() == item))
    }

    // Append text to file
    fn append_to_file<P: AsRef<Path>>(&self, file_path: P, text: &str) -> io::Result<()> {
        let file_path = file_path.as_ref();

        // Create parent directories if they don't exist
        if let Some(parent) = file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path)?;

        writeln!(file, "{}", text)
    }

    // Read lines from file
    fn read_lines<P: AsRef<Path>>(&self, file_path: P) -> io::Result<Vec<String>> {
        let file_path = file_path.as_ref();

        if !file_path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();

        for line in reader.lines() {
            lines.push(line?);
        }

        Ok(lines)
    }

    // Remove item from list
    fn remove_item_from_list<P: AsRef<Path>>(&self, file_path: P, item: &str) -> io::Result<()> {
        let item = item.trim();
        if item.is_empty() {
            println!("No item specified to remove.");
            return Ok(());
        }

        let lines = self.read_lines(&file_path)?;
        let mut updated_lines = Vec::new();
        let mut found = false;

        for line in lines {
            if line.trim() != item {
                updated_lines.push(line);
            } else {
                found = true;
            }
        }

        if !found {
            println!("Item '{}' not found in the list.", item);
            return Ok(());
        }

        fs::write(&file_path, updated_lines.join("\n") + "\n")?;
        println!("Item '{}' removed successfully.", item);

        Ok(())
    }

    // List items with header
    fn list_items<P: AsRef<Path>>(&self, file_path: P, header: &str) -> io::Result<()> {
        let lines = self.read_lines(file_path)?;

        println!("\n{}:", header);
        if lines.is_empty() {
            println!("  No items found.");
        } else {
            for line in lines {
                println!("  - {}", line);
            }
        }

        Ok(())
    }

    // ---- Database operations for dotfiles ----

    // Stage a dotfile in the database
    pub fn stage_dotfile(
        &self,
        source: &Path,
        target: &Path,
        profile: Option<&str>,
    ) -> rusqlite::Result<()> {
        if let Some(conn) = &self.connection {
            let source_str = source.to_string_lossy().to_string();
            let target_str = target.to_string_lossy().to_string();

            conn.execute(
                "INSERT INTO dotfiles (source, target, profile, status) VALUES (?, ?, ?, 'staged')",
                rusqlite::params![source_str, target_str, profile],
            )?;

            Ok(())
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    // Update a dotfile status to linked
    pub fn link_dotfile(&self, source: &Path, target: &Path) -> rusqlite::Result<()> {
        if let Some(conn) = &self.connection {
            let source_str = source.to_string_lossy().to_string();
            let target_str = target.to_string_lossy().to_string();

            conn.execute(
                "UPDATE dotfiles SET status = 'linked' WHERE source = ? AND target = ? AND active = 1",
                rusqlite::params![source_str, target_str],
            )?;

            Ok(())
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    // Add a dotfile directly with linked status (for legacy compatibility)
    pub fn add_dotfile(
        &self,
        source: &Path,
        target: &Path,
        profile: Option<&str>,
    ) -> rusqlite::Result<()> {
        if let Some(conn) = &self.connection {
            let source_str = source.to_string_lossy().to_string();
            let target_str = target.to_string_lossy().to_string();

            conn.execute(
                "INSERT INTO dotfiles (source, target, profile, status) VALUES (?, ?, ?, 'linked')",
                rusqlite::params![source_str, target_str, profile],
            )?;

            Ok(())
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    // Get all dotfiles
    pub fn get_dotfiles(
        &self,
        profile: Option<&str>,
    ) -> rusqlite::Result<Vec<crate::dotfile::DotFile>> {
        let mut dotfiles = Vec::new();

        if let Some(conn) = &self.connection {
            let query = match profile {
                Some(_) => {
                    "SELECT source, target, profile, status FROM dotfiles WHERE active = 1 AND profile = ?1"
                }
                None => "SELECT source, target, profile, status FROM dotfiles WHERE active = 1",
            };

            let mut stmt = conn.prepare(query)?;

            // This closure is used to extract the dotfile data from a row
            let map_row = |row: &rusqlite::Row| -> rusqlite::Result<crate::dotfile::DotFile> {
                let source: String = row.get(0)?;
                let target: String = row.get(1)?;
                let profile: Option<String> = row.get(2)?;
                let status_str: String = row.get(3)?;

                let status = match status_str.as_str() {
                    "staged" => crate::dotfile::DotFileStatus::Staged,
                    "linked" => crate::dotfile::DotFileStatus::Linked,
                    "unlinked" => crate::dotfile::DotFileStatus::Unlinked,
                    _ => crate::dotfile::DotFileStatus::Staged,
                };

                Ok(crate::dotfile::DotFile::with_status(
                    PathBuf::from(source),
                    PathBuf::from(target),
                    profile,
                    status,
                ))
            };

            let mut rows = match profile {
                Some(p) => stmt.query_map([p], map_row)?,
                None => stmt.query_map([], map_row)?,
            };

            while let Some(dotfile_result) = rows.next() {
                dotfiles.push(dotfile_result?);
            }
        }

        Ok(dotfiles)
    }

    // Get staged dotfiles
    pub fn get_staged_dotfiles(
        &self,
        profile: Option<&str>,
    ) -> rusqlite::Result<Vec<crate::dotfile::DotFile>> {
        let mut dotfiles = Vec::new();

        if let Some(conn) = &self.connection {
            let query = match profile {
                Some(_) => {
                    "SELECT source, target, profile, status FROM dotfiles WHERE status = 'staged' AND active = 1 AND profile = ?1"
                }
                None => {
                    "SELECT source, target, profile, status FROM dotfiles WHERE status = 'staged' AND active = 1"
                }
            };

            let mut stmt = conn.prepare(query)?;

            // This closure is used to extract the dotfile data from a row
            let map_row = |row: &rusqlite::Row| -> rusqlite::Result<crate::dotfile::DotFile> {
                let source: String = row.get(0)?;
                let target: String = row.get(1)?;
                let profile: Option<String> = row.get(2)?;
                let status_str: String = row.get(3)?;

                let status = match status_str.as_str() {
                    "staged" => crate::dotfile::DotFileStatus::Staged,
                    "linked" => crate::dotfile::DotFileStatus::Linked,
                    "unlinked" => crate::dotfile::DotFileStatus::Unlinked,
                    _ => crate::dotfile::DotFileStatus::Staged,
                };

                Ok(crate::dotfile::DotFile::with_status(
                    PathBuf::from(source),
                    PathBuf::from(target),
                    profile,
                    status,
                ))
            };

            let mut rows = match profile {
                Some(p) => stmt.query_map([p], map_row)?,
                None => stmt.query_map([], map_row)?,
            };

            while let Some(dotfile_result) = rows.next() {
                dotfiles.push(dotfile_result?);
            }
        }

        Ok(dotfiles)
    }

    // Deactivate (mark as inactive) a dotfile by target path
    pub fn deactivate_dotfile(&self, target: &Path) -> rusqlite::Result<bool> {
        if let Some(conn) = &self.connection {
            let target_str = target.to_string_lossy().to_string();

            let affected = conn.execute(
                "UPDATE dotfiles SET active = 0 WHERE target = ?",
                [target_str],
            )?;

            Ok(affected > 0)
        } else {
            Ok(false)
        }
    }

    // Completely remove a dotfile from the database
    pub fn remove_dotfile(&self, target: &Path) -> rusqlite::Result<bool> {
        if let Some(conn) = &self.connection {
            let target_str = target.to_string_lossy().to_string();

            let affected = conn.execute("DELETE FROM dotfiles WHERE target = ?", [target_str])?;

            Ok(affected > 0)
        } else {
            Ok(false)
        }
    }

    // Find a dotfile by target path
    pub fn find_dotfile_by_target(
        &self,
        target: &Path,
    ) -> rusqlite::Result<Option<crate::dotfile::DotFile>> {
        if let Some(conn) = &self.connection {
            let target_str = target.to_string_lossy().to_string();

            let result = conn.query_row(
                "SELECT source, target, profile, status FROM dotfiles WHERE target = ? AND active = 1",
                [target_str],
                |row| {
                    let source: String = row.get(0)?;
                    let target: String = row.get(1)?;
                    let profile: Option<String> = row.get(2)?;
                    let status_str: String = row.get(3)?;

                    let status = match status_str.as_str() {
                        "staged" => crate::dotfile::DotFileStatus::Staged,
                        "linked" => crate::dotfile::DotFileStatus::Linked,
                        "unlinked" => crate::dotfile::DotFileStatus::Unlinked,
                        _ => crate::dotfile::DotFileStatus::Staged,
                    };

                    Ok(crate::dotfile::DotFile::with_status(
                        PathBuf::from(source),
                        PathBuf::from(target),
                        profile,
                        status,
                    ))
                },
            );

            match result {
                Ok(dotfile) => Ok(Some(dotfile)),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(e),
            }
        } else {
            Ok(None)
        }
    }

    // Find a dotfile by source path
    pub fn find_dotfile_by_source(
        &self,
        source: &Path,
    ) -> rusqlite::Result<Option<crate::dotfile::DotFile>> {
        if let Some(conn) = &self.connection {
            let source_str = source.to_string_lossy().to_string();

            let result = conn.query_row(
                "SELECT source, target, profile, status FROM dotfiles WHERE source = ? AND active = 1",
                [source_str],
                |row| {
                    let source: String = row.get(0)?;
                    let target: String = row.get(1)?;
                    let profile: Option<String> = row.get(2)?;
                    let status_str: String = row.get(3)?;

                    let status = match status_str.as_str() {
                        "staged" => crate::dotfile::DotFileStatus::Staged,
                        "linked" => crate::dotfile::DotFileStatus::Linked,
                        "unlinked" => crate::dotfile::DotFileStatus::Unlinked,
                        _ => crate::dotfile::DotFileStatus::Staged,
                    };

                    Ok(crate::dotfile::DotFile::with_status(
                        PathBuf::from(source),
                        PathBuf::from(target),
                        profile,
                        status,
                    ))
                },
            );

            match result {
                Ok(dotfile) => Ok(Some(dotfile)),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(e),
            }
        } else {
            Ok(None)
        }
    }
}

// Static helper functions to use when a Config instance is not available

// Get default configuration instance
fn get_config() -> Config {
    Config::new()
}

// Read default path
pub fn read_default_path() -> String {
    get_config().read_default_path()
}

// Get file types
pub fn get_file_types() -> io::Result<Vec<String>> {
    get_config().read_lines(get_config().get_filetypes_file())
}

// Get ignored paths
pub fn get_ignored_paths() -> io::Result<Vec<String>> {
    get_config().read_lines(get_config().get_ignored_paths_file())
}

// ---- Managed Folders operations ----

// Add a managed folder
pub fn add_managed_folder(name: &str, path: &Path) -> io::Result<()> {
    get_config().add_managed_folder(name, path)
}

// Get the current active managed folder
pub fn get_active_managed_folder() -> io::Result<Option<(String, PathBuf)>> {
    get_config().get_active_managed_folder()
}

// ---- Database operations for dotfiles ----

// Get a database connection
pub fn get_db_connection() -> rusqlite::Result<Config> {
    let mut config = get_config();
    config.connect()?;
    Ok(config)
}

// Stage a dotfile
pub fn stage_dotfile(source: &Path, target: &Path, profile: Option<&str>) -> rusqlite::Result<()> {
    let config = get_db_connection()?;
    config.stage_dotfile(source, target, profile)
}

// Link a dotfile
pub fn link_dotfile(source: &Path, target: &Path) -> rusqlite::Result<()> {
    let config = get_db_connection()?;
    config.link_dotfile(source, target)
}

// Add a dotfile directly (legacy method)
pub fn add_dotfile(source: &Path, target: &Path, profile: Option<&str>) -> rusqlite::Result<()> {
    let config = get_db_connection()?;
    config.add_dotfile(source, target, profile)
}

// Get all dotfiles
pub fn get_dotfiles(profile: Option<&str>) -> rusqlite::Result<Vec<crate::dotfile::DotFile>> {
    let config = get_db_connection()?;
    config.get_dotfiles(profile)
}

// Get staged dotfiles
pub fn get_staged_dotfiles(
    profile: Option<&str>,
) -> rusqlite::Result<Vec<crate::dotfile::DotFile>> {
    let config = get_db_connection()?;
    config.get_staged_dotfiles(profile)
}

// Deactivate a dotfile
pub fn deactivate_dotfile(target: &Path) -> rusqlite::Result<bool> {
    let config = get_db_connection()?;
    config.deactivate_dotfile(target)
}

// Remove a dotfile completely from the database
pub fn remove_dotfile(target: &Path) -> rusqlite::Result<bool> {
    let config = get_db_connection()?;
    config.remove_dotfile(target)
}

// Find a dotfile by target path
pub fn find_dotfile_by_target(target: &Path) -> rusqlite::Result<Option<crate::dotfile::DotFile>> {
    let config = get_db_connection()?;
    config.find_dotfile_by_target(target)
}

// Find a dotfile by source path
pub fn find_dotfile_by_source(source: &Path) -> rusqlite::Result<Option<crate::dotfile::DotFile>> {
    let config = get_db_connection()?;
    config.find_dotfile_by_source(source)
}

/// Batch deactivate (mark as inactive) dotfiles by target paths
pub fn deactivate_dotfiles(targets: &[std::path::PathBuf]) -> rusqlite::Result<usize> {
    if targets.is_empty() {
        return Ok(0);
    }
    let config = get_db_connection()?;
    if let Some(conn) = &config.connection {
        if targets.is_empty() {
            return Ok(0);
        }
        // Build a parameterized query for all targets
        let mut sql =
            String::from("UPDATE dotfiles SET active = 0 WHERE active = 1 AND target IN (");
        let params: Vec<String> = (0..targets.len()).map(|i| format!("?{}", i + 1)).collect();
        sql.push_str(&params.join(","));
        sql.push(')');
        let target_strs: Vec<_> = targets
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();
        let updated = conn.execute(&sql, rusqlite::params_from_iter(target_strs))?;
        Ok(updated)
    } else {
        Ok(0)
    }
}

/// Purge all dotfile records (staged or managed) for a specified folder (and subfolders if recursive)
pub fn purge_dotfiles_in_folder(
    folder: &std::path::Path,
    recursive: bool,
) -> rusqlite::Result<usize> {
    let config = get_db_connection()?;
    if let Some(conn) = &config.connection {
        let folder_str = folder.to_string_lossy();
        let like_pattern = if recursive {
            format!("{}%", folder_str)
        } else {
            format!("{}", folder_str)
        };
        println!(
            "purge: folder_str = '{}', like_pattern = '{}'",
            folder_str, like_pattern
        );
        // Print a few sample source/target paths for debugging
        let mut stmt = conn.prepare("SELECT source, target FROM dotfiles LIMIT 5")?;
        let rows = stmt.query_map([], |row| {
            let source: String = row.get(0)?;
            let target: String = row.get(1)?;
            Ok((source, target))
        })?;
        for (i, row) in rows.enumerate() {
            if let Ok((source, target)) = row {
                println!(
                    "purge: sample {}: source='{}', target='{}'",
                    i + 1,
                    source,
                    target
                );
            }
        }
        // Delete where source or target is under the folder
        let sql = if recursive {
            "DELETE FROM dotfiles WHERE (source LIKE ?1 OR target LIKE ?1)"
        } else {
            "DELETE FROM dotfiles WHERE (source = ?1 OR target = ?1)"
        };
        let deleted = conn.execute(sql, rusqlite::params![like_pattern])?;
        Ok(deleted)
    } else {
        Ok(0)
    }
}

/// Get all dotfile records (staged or managed) for a specified folder (and subfolders if recursive)
pub fn get_dotfiles_in_folder(
    folder: &std::path::Path,
    recursive: bool,
) -> rusqlite::Result<Vec<crate::dotfile::DotFile>> {
    let config = get_db_connection()?;
    let mut dotfiles = Vec::new();
    if let Some(conn) = &config.connection {
        let folder_str = folder.to_string_lossy();
        let like_pattern = if recursive {
            let mut s = folder_str.to_string();
            if !s.ends_with('/') {
                s.push('/');
            }
            format!("{}%", s)
        } else {
            format!("{}", folder_str)
        };
        let sql = if recursive {
            "SELECT source, target, profile, status FROM dotfiles WHERE (source LIKE ?1 OR target LIKE ?1)"
        } else {
            "SELECT source, target, profile, status FROM dotfiles WHERE (source = ?1 OR target = ?1)"
        };
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(rusqlite::params![like_pattern], |row| {
            Ok(crate::dotfile::DotFile {
                source: std::path::PathBuf::from(row.get::<_, String>(0)?),
                target: std::path::PathBuf::from(row.get::<_, String>(1)?),
                profile: row.get::<_, Option<String>>(2)?,
                status: match row.get::<_, String>(3)?.as_str() {
                    "staged" => crate::dotfile::DotFileStatus::Staged,
                    "linked" => crate::dotfile::DotFileStatus::Linked,
                    _ => crate::dotfile::DotFileStatus::Unlinked,
                },
            })
        })?;
        for df in rows {
            dotfiles.push(df?);
        }
    }
    Ok(dotfiles)
}
