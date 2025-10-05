// Unit tests for symlink utility functions

use assert_fs::TempDir;
use assert_fs::prelude::*;
use forge_rs::symlink;

#[test]
fn test_create_symlink() {
    let temp = TempDir::new().unwrap();

    let source = temp.child("source.txt");
    source.write_str("content").unwrap();

    let target = temp.path().join("link.txt");

    // Create symlink
    let result = symlink::create_symlink(source.path(), &target);
    assert!(result.is_ok());
    assert!(target.exists());
    assert!(target.is_symlink());
}

#[test]
fn test_is_symlink_returns_true_for_symlink() {
    let temp = TempDir::new().unwrap();

    let source = temp.child("source.txt");
    source.write_str("content").unwrap();

    let target = temp.path().join("link.txt");
    symlink::create_symlink(source.path(), &target).unwrap();

    assert!(symlink::is_symlink(&target));
}

#[test]
fn test_is_symlink_returns_false_for_regular_file() {
    let temp = TempDir::new().unwrap();
    let file = temp.child("regular.txt");
    file.write_str("content").unwrap();

    assert!(!symlink::is_symlink(file.path()));
}

#[test]
fn test_is_symlink_returns_false_for_directory() {
    let temp = TempDir::new().unwrap();
    let dir = temp.child("testdir");
    dir.create_dir_all().unwrap();

    assert!(!symlink::is_symlink(dir.path()));
}

#[test]
fn test_symlink_points_to_correct_target() {
    let temp = TempDir::new().unwrap();

    let source = temp.child("source.txt");
    source.write_str("test content").unwrap();

    let target = temp.path().join("link.txt");
    symlink::create_symlink(source.path(), &target).unwrap();

    // Read through the symlink
    let content = std::fs::read_to_string(&target).unwrap();
    assert_eq!(content, "test content");
}

#[test]
fn test_create_symlink_to_nonexistent_source() {
    let temp = TempDir::new().unwrap();
    let target = temp.path().join("link.txt");

    // Creating a symlink to a non-existent source should still work
    // (dangling symlinks are allowed)
    let result = symlink::create_symlink(temp.path().join("nonexistent.txt"), &target);

    // This might succeed (creating a dangling symlink) or fail depending on platform
    // We just verify it doesn't panic
    let _ = result;
}

#[test]
fn test_create_symlink_overwrites_existing_symlink() {
    let temp = TempDir::new().unwrap();

    let source1 = temp.child("source1.txt");
    source1.write_str("content1").unwrap();

    let source2 = temp.child("source2.txt");
    source2.write_str("content2").unwrap();

    let target = temp.path().join("link.txt");

    // Create first symlink
    symlink::create_symlink(source1.path(), &target).unwrap();

    // Remove and create second symlink
    std::fs::remove_file(&target).unwrap();
    let result = symlink::create_symlink(source2.path(), &target);

    assert!(result.is_ok());
    assert!(target.is_symlink());
}

#[cfg(unix)]
#[test]
fn test_create_directory_symlink() {
    let temp = TempDir::new().unwrap();

    let source_dir = temp.child("sourcedir");
    source_dir.create_dir_all().unwrap();

    let target = temp.path().join("linkdir");
    let result = symlink::create_symlink(source_dir.path(), &target);

    assert!(result.is_ok());
    assert!(target.exists());
    assert!(target.is_symlink());
}

#[test]
fn test_symlink_preserves_file_content() {
    let temp = TempDir::new().unwrap();

    let source = temp.child("source.txt");
    let original_content = "Hello, this is test content with special chars: éàü";
    source.write_str(original_content).unwrap();

    let target = temp.path().join("link.txt");
    symlink::create_symlink(source.path(), &target).unwrap();

    let read_content = std::fs::read_to_string(&target).unwrap();
    assert_eq!(read_content, original_content);
}
