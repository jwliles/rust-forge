pub mod path_utils;
pub mod ui;

// General utility functions that don't fit elsewhere
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
