pub mod path_utils;

// General utility functions that don't fit elsewhere
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
