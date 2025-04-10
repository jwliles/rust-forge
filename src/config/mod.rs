use rusqlite::Connection;
use std::path::PathBuf;

pub struct Config {
    db_path: PathBuf,
    connection: Option<Connection>,
}

impl Config {
    pub fn new() -> Self {
        // Initialize config
        Self {
            db_path: Self::get_default_db_path(),
            connection: None,
        }
    }

    fn get_default_db_path() -> PathBuf {
        // Get the default path for the database
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("dotforge");
        path.push("dotforge.db");
        path
    }

    pub fn connect(&mut self) -> rusqlite::Result<()> {
        // Connect to the database
        self.connection = Some(Connection::open(&self.db_path)?); 
        Ok(())
    }
}
