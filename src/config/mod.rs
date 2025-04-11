use anyhow::{anyhow, Context, Result};
use rusqlite::Connection;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

// Constants for configuration file paths
const DEFAULT_CONFIG_DIR: &str = ".dotforge";
const DEFAULT_PATH_FILE: &str = "default_path";
const FILETYPES_FILE: &str = "filetypes";
const IGNORED_PATHS_FILE: &str = "ignored_paths";

pub struct Config {
    db_path: PathBuf,
    connection: Option<Connection>,
    // File-based config paths
    config_dir: PathBuf,
    default_path_file: PathBuf,
    filetypes_file: PathBuf,
    ignored_paths_file: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let config_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(DEFAULT_CONFIG_DIR);
        
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
        
        // Initialize db_path 
        let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        db_path.push("dotforge");
        db_path.push("dotforge.db");
        
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
        }
    }

    pub fn connect(&mut self) -> rusqlite::Result<()> {
        // Connect to the database
        self.connection = Some(Connection::open(&self.db_path)?); 
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
            Err(_) => String::from("~/dotforge"),
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

// Set default path
pub fn set_default_path(path: &str) -> io::Result<()> {
    get_config().set_default_path(path)
}

// Get file types
pub fn get_file_types() -> io::Result<Vec<String>> {
    get_config().read_lines(get_config().get_filetypes_file())
}

// Get ignored paths
pub fn get_ignored_paths() -> io::Result<Vec<String>> {
    get_config().read_lines(get_config().get_ignored_paths_file())
}

// Add file types
pub fn add_filetypes(extensions: &[String]) -> io::Result<()> {
    get_config().add_filetypes(extensions)
}

// Remove file types
pub fn remove_filetypes(extensions: &[String]) -> io::Result<()> {
    get_config().remove_filetypes(extensions)
}

// List file types
pub fn list_filetypes() -> io::Result<()> {
    get_config().list_filetypes()
}

// Add ignored paths
pub fn add_ignored_paths(paths: &[String]) -> io::Result<()> {
    get_config().add_ignored_paths(paths)
}

// Remove ignored paths
pub fn remove_ignored_paths(paths: &[String]) -> io::Result<()> {
    get_config().remove_ignored_paths(paths)
}

// List ignored paths
pub fn list_ignored_paths() -> io::Result<()> {
    get_config().list_ignored_paths()
}
