use std::path::PathBuf;

pub mod path_utils;
pub mod ui;

// General utility functions that don't fit elsewhere
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Get the path to the man page, if it exists
pub fn get_man_page() -> Option<PathBuf> {
    option_env!("FORGE_MAN_PAGE").map(PathBuf::from)
}
