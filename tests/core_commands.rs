// Integration tests for core forge commands: init, stage, link, unlink, remove, delete

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::fs;

mod common;

#[test]
fn test_init_command_creates_forge_directory() {
    let temp = TempDir::new().unwrap();

    common::forge_cmd()
        .arg("init")
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Forge repository initialized successfully"));

    // Verify .forge directory was created
    assert!(temp.path().join(".forge").exists());
}

#[test]
fn test_init_with_custom_name() {
    let temp = TempDir::new().unwrap();

    common::forge_cmd()
        .arg("init")
        .arg("--name")
        .arg("my-dotfiles")
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("my-dotfiles"));
}

#[test]
fn test_stage_single_file() {
    let temp = TempDir::new().unwrap();
    common::init_forge_repo(&temp).unwrap();

    let test_file = temp.child("test.conf");
    test_file.write_str("test content").unwrap();

    common::forge_cmd()
        .arg("stage")
        .arg(test_file.path())
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Staging"));
}

#[test]
fn test_stage_multiple_files() {
    let temp = TempDir::new().unwrap();
    common::init_forge_repo(&temp).unwrap();

    let file1 = temp.child("file1.conf");
    let file2 = temp.child("file2.conf");
    file1.write_str("content1").unwrap();
    file2.write_str("content2").unwrap();

    common::forge_cmd()
        .arg("stage")
        .arg(file1.path())
        .arg(file2.path())
        .current_dir(temp.path())
        .assert()
        .success();
}

#[test]
fn test_stage_recursive() {
    let temp = TempDir::new().unwrap();
    common::init_forge_repo(&temp).unwrap();

    let dir = temp.child("config");
    dir.create_dir_all().unwrap();
    dir.child("file1.conf").write_str("content").unwrap();
    dir.child("subdir").create_dir_all().unwrap();
    dir.child("subdir/file2.conf").write_str("content").unwrap();

    common::forge_cmd()
        .arg("stage")
        .arg("--recursive")
        .arg(dir.path())
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("recursively"));
}

#[test]
fn test_stage_with_depth_limit() {
    let temp = TempDir::new().unwrap();
    common::init_forge_repo(&temp).unwrap();

    let dir = temp.child("config");
    dir.create_dir_all().unwrap();
    dir.child("file1.conf").write_str("content").unwrap();

    common::forge_cmd()
        .arg("stage")
        .arg("--depth")
        .arg("2")
        .arg(dir.path())
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("max depth: 2"));
}

#[test]
fn test_list_command() {
    let temp = TempDir::new().unwrap();
    common::init_forge_repo(&temp).unwrap();

    let test_file = temp.child("test.conf");
    test_file.write_str("test content").unwrap();

    // Stage a file first
    common::forge_cmd()
        .arg("stage")
        .arg(test_file.path())
        .current_dir(temp.path())
        .assert()
        .success();

    // List should show the staged file
    common::forge_cmd()
        .arg("list")
        .current_dir(temp.path())
        .assert()
        .success();
}

#[test]
fn test_link_command_creates_symlinks() {
    let temp = TempDir::new().unwrap();
    common::init_forge_repo(&temp).unwrap();

    let test_file = temp.child("test.conf");
    test_file.write_str("test content").unwrap();

    // Stage the file
    common::forge_cmd()
        .arg("stage")
        .arg(test_file.path())
        .current_dir(temp.path())
        .assert()
        .success();

    // Link should create symlinks
    common::forge_cmd()
        .arg("link")
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Creating symlinks"));
}

#[test]
fn test_unlink_command() {
    let temp = TempDir::new().unwrap();
    common::init_forge_repo(&temp).unwrap();

    let test_file = temp.child("test.conf");
    test_file.write_str("test content").unwrap();

    // Stage and link
    common::forge_cmd()
        .arg("stage")
        .arg(test_file.path())
        .current_dir(temp.path())
        .assert()
        .success();

    common::forge_cmd()
        .arg("link")
        .current_dir(temp.path())
        .assert()
        .success();

    // Unlink with --yes to skip confirmation
    common::forge_cmd()
        .arg("unlink")
        .arg("--yes")
        .arg(test_file.path())
        .current_dir(temp.path())
        .assert()
        .success();
}

#[test]
fn test_unstage_command() {
    let temp = TempDir::new().unwrap();
    common::init_forge_repo(&temp).unwrap();

    let test_file = temp.child("test.conf");
    test_file.write_str("test content").unwrap();

    // Stage a file
    common::forge_cmd()
        .arg("stage")
        .arg(test_file.path())
        .current_dir(temp.path())
        .assert()
        .success();

    // Unstage it
    common::forge_cmd()
        .arg("unstage")
        .arg(test_file.path())
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Unstaging"));
}

#[test]
fn test_unstage_recursive() {
    let temp = TempDir::new().unwrap();
    common::init_forge_repo(&temp).unwrap();

    let dir = temp.child("config");
    dir.create_dir_all().unwrap();
    dir.child("file1.conf").write_str("content").unwrap();

    // Stage recursively
    common::forge_cmd()
        .arg("stage")
        .arg("--recursive")
        .arg(dir.path())
        .current_dir(temp.path())
        .assert()
        .success();

    // Unstage recursively
    common::forge_cmd()
        .arg("unstage")
        .arg("--recursive")
        .arg(dir.path())
        .current_dir(temp.path())
        .assert()
        .success();
}

#[test]
fn test_purge_command() {
    let temp = TempDir::new().unwrap();
    common::init_forge_repo(&temp).unwrap();

    let test_file = temp.child("test.conf");
    test_file.write_str("test content").unwrap();

    // Stage a file
    common::forge_cmd()
        .arg("stage")
        .arg(test_file.path())
        .current_dir(temp.path())
        .assert()
        .success();

    // Purge should clean up
    common::forge_cmd()
        .arg("purge")
        .arg("--yes")
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("purging"));
}

#[test]
fn test_stage_nonexistent_file_fails() {
    let temp = TempDir::new().unwrap();
    common::init_forge_repo(&temp).unwrap();

    common::forge_cmd()
        .arg("stage")
        .arg(temp.path().join("nonexistent.conf"))
        .current_dir(temp.path())
        .assert()
        .failure();
}

#[test]
fn test_init_without_managed_folder_name_uses_dir_name() {
    let temp = TempDir::new().unwrap();

    common::forge_cmd()
        .arg("init")
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(
            temp.path().file_name().unwrap().to_str().unwrap()
        ));
}

#[test]
fn test_help_flag() {
    common::forge_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_version_flag() {
    common::forge_cmd()
        .arg("--version")
        .assert()
        .success();
}

#[test]
fn test_verbose_flag() {
    let temp = TempDir::new().unwrap();

    common::forge_cmd()
        .arg("-v")
        .arg("init")
        .current_dir(temp.path())
        .assert()
        .success();
}
