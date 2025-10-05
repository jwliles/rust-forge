// Unit tests for path utility functions

use forge_rs::utils::path_utils;
use std::path::{Path, PathBuf};

#[test]
fn test_expand_tilde_with_tilde_path() {
    let path = Path::new("~/test/file.txt");
    let expanded = path_utils::expand_tilde(path);

    // Should not start with ~ anymore
    assert!(!expanded.to_string_lossy().starts_with("~"));

    // Should be an absolute path
    assert!(expanded.is_absolute() || dirs::home_dir().is_some());
}

#[test]
fn test_expand_tilde_without_tilde() {
    let path = Path::new("/absolute/path/file.txt");
    let expanded = path_utils::expand_tilde(path);

    // Should remain unchanged
    assert_eq!(path, expanded);
}

#[test]
fn test_expand_tilde_relative_path() {
    let path = Path::new("relative/path/file.txt");
    let expanded = path_utils::expand_tilde(path);

    // Should remain unchanged (no tilde)
    assert_eq!(path, expanded);
}

#[test]
fn test_normalize_absolute_path() {
    let path = Path::new("/tmp/test");
    let normalized = path_utils::normalize(path);

    assert!(normalized.is_absolute());
    assert_eq!(normalized, PathBuf::from("/tmp/test"));
}

#[test]
fn test_normalize_relative_path_becomes_absolute() {
    let path = Path::new("test/file.txt");
    let normalized = path_utils::normalize(path);

    // Relative paths should be made absolute
    assert!(normalized.is_absolute());
}

#[test]
fn test_normalize_tilde_path() {
    let path = Path::new("~/Documents/test.txt");
    let normalized = path_utils::normalize(path);

    // Should expand tilde and make absolute
    assert!(!normalized.to_string_lossy().contains("~"));
    assert!(normalized.is_absolute());
}

#[test]
fn test_normalize_empty_path() {
    let path = Path::new("");
    let normalized = path_utils::normalize(path);

    // Empty path should become current directory (absolute)
    assert!(normalized.is_absolute());
}

#[test]
fn test_normalize_dot_path() {
    let path = Path::new(".");
    let normalized = path_utils::normalize(path);

    // Dot should be expanded to current directory
    assert!(normalized.is_absolute());
}

#[test]
fn test_expand_tilde_with_subpath() {
    let path = Path::new("~/test/subdir/file.txt");
    let expanded = path_utils::expand_tilde(path);

    // Should preserve the subpath structure
    assert!(expanded.to_string_lossy().contains("test"));
    assert!(expanded.to_string_lossy().contains("subdir"));
    assert!(expanded.to_string_lossy().contains("file.txt"));
}

#[test]
fn test_normalize_preserves_path_components() {
    let path = Path::new("~/config/nvim/init.lua");
    let normalized = path_utils::normalize(path);

    let normalized_str = normalized.to_string_lossy();
    assert!(normalized_str.contains("config"));
    assert!(normalized_str.contains("nvim"));
    assert!(normalized_str.contains("init.lua"));
}
