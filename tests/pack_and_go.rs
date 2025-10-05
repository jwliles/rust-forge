// Integration tests for pack-and-go system: pack, seal, install, restore, explain, repack, unpack

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::fs;

mod common;

#[test]
fn test_start_packing_creates_new_pack() {
    let temp = TempDir::new().unwrap();

    common::forge_cmd()
        .arg("start")
        .arg("packing")
        .arg("test-pack")
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("test-pack"));
}

#[test]
fn test_pack_single_file() {
    let temp = TempDir::new().unwrap();

    // Start a new pack
    common::forge_cmd()
        .arg("start")
        .arg("packing")
        .arg("test-pack")
        .current_dir(temp.path())
        .assert()
        .success();

    // Create a test file
    let test_file = temp.child("config.txt");
    test_file.write_str("test content").unwrap();

    // Pack the file
    common::forge_cmd()
        .arg("pack")
        .arg(test_file.path())
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Packing"));
}

#[test]
fn test_pack_multiple_files() {
    let temp = TempDir::new().unwrap();

    common::forge_cmd()
        .arg("start")
        .arg("packing")
        .arg("multi-pack")
        .current_dir(temp.path())
        .assert()
        .success();

    let file1 = temp.child("file1.txt");
    let file2 = temp.child("file2.txt");
    file1.write_str("content1").unwrap();
    file2.write_str("content2").unwrap();

    common::forge_cmd()
        .arg("pack")
        .arg(file1.path())
        .arg(file2.path())
        .current_dir(temp.path())
        .assert()
        .success();
}

#[test]
fn test_pack_recursive_directory() {
    let temp = TempDir::new().unwrap();

    common::forge_cmd()
        .arg("start")
        .arg("packing")
        .arg("dir-pack")
        .current_dir(temp.path())
        .assert()
        .success();

    // Create directory structure
    let config_dir = temp.child("config");
    config_dir.create_dir_all().unwrap();
    config_dir.child("file1.txt").write_str("content").unwrap();

    let subdir = config_dir.child("subdir");
    subdir.create_dir_all().unwrap();
    subdir.child("file2.txt").write_str("content").unwrap();

    common::forge_cmd()
        .arg("pack")
        .arg("--recursive")
        .arg(config_dir.path())
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("recursively"));
}

#[test]
fn test_pack_with_depth_limit() {
    let temp = TempDir::new().unwrap();

    common::forge_cmd()
        .arg("start")
        .arg("packing")
        .arg("depth-pack")
        .current_dir(temp.path())
        .assert()
        .success();

    let config_dir = temp.child("config");
    config_dir.create_dir_all().unwrap();
    config_dir.child("file1.txt").write_str("content").unwrap();

    common::forge_cmd()
        .arg("pack")
        .arg("--depth")
        .arg("2")
        .arg(config_dir.path())
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("depth"));
}

#[test]
fn test_pack_dry_run() {
    let temp = TempDir::new().unwrap();

    common::forge_cmd()
        .arg("start")
        .arg("packing")
        .arg("dry-pack")
        .current_dir(temp.path())
        .assert()
        .success();

    let test_file = temp.child("test.txt");
    test_file.write_str("content").unwrap();

    common::forge_cmd()
        .arg("pack")
        .arg("--dry-run")
        .arg(test_file.path())
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("dry").or(predicate::str::contains("preview")));
}

#[test]
fn test_seal_creates_archive() {
    let temp = TempDir::new().unwrap();

    common::forge_cmd()
        .arg("start")
        .arg("packing")
        .arg("seal-test")
        .current_dir(temp.path())
        .assert()
        .success();

    let test_file = temp.child("test.txt");
    test_file.write_str("content").unwrap();

    common::forge_cmd()
        .arg("pack")
        .arg(test_file.path())
        .current_dir(temp.path())
        .assert()
        .success();

    // Seal the pack
    common::forge_cmd()
        .arg("seal")
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("seal").or(predicate::str::contains("archive")));
}

#[test]
fn test_explain_pack_contents() {
    let temp = TempDir::new().unwrap();
    let pack_temp = TempDir::new().unwrap();

    // Create and seal a pack
    common::forge_cmd()
        .arg("start")
        .arg("packing")
        .arg("explain-test")
        .current_dir(pack_temp.path())
        .assert()
        .success();

    let test_file = pack_temp.child("test.txt");
    test_file.write_str("content").unwrap();

    common::forge_cmd()
        .arg("pack")
        .arg(test_file.path())
        .current_dir(pack_temp.path())
        .assert()
        .success();

    common::forge_cmd()
        .arg("seal")
        .current_dir(pack_temp.path())
        .assert()
        .success();

    // Find the created zip file
    let zip_files: Vec<_> = fs::read_dir(pack_temp.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("zip"))
        .collect();

    if let Some(zip_file) = zip_files.first() {
        // Explain the pack
        common::forge_cmd()
            .arg("explain")
            .arg(zip_file.path())
            .current_dir(temp.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("test.txt").or(predicate::str::contains("Contents")));
    }
}

#[test]
fn test_install_pack() {
    let pack_temp = TempDir::new().unwrap();
    let install_temp = TempDir::new().unwrap();

    // Create and seal a pack
    common::forge_cmd()
        .arg("start")
        .arg("packing")
        .arg("install-test")
        .current_dir(pack_temp.path())
        .assert()
        .success();

    let test_file = pack_temp.child("test.txt");
    test_file.write_str("install content").unwrap();

    common::forge_cmd()
        .arg("pack")
        .arg(test_file.path())
        .current_dir(pack_temp.path())
        .assert()
        .success();

    common::forge_cmd()
        .arg("seal")
        .current_dir(pack_temp.path())
        .assert()
        .success();

    // Find the created zip file
    let zip_files: Vec<_> = fs::read_dir(pack_temp.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("zip"))
        .collect();

    if let Some(zip_file) = zip_files.first() {
        // Install the pack
        common::forge_cmd()
            .arg("install")
            .arg(zip_file.path())
            .arg("--target")
            .arg(install_temp.path())
            .current_dir(install_temp.path())
            .assert()
            .success();
    }
}

#[test]
fn test_install_with_force_flag() {
    let pack_temp = TempDir::new().unwrap();
    let install_temp = TempDir::new().unwrap();

    common::forge_cmd()
        .arg("start")
        .arg("packing")
        .arg("force-test")
        .current_dir(pack_temp.path())
        .assert()
        .success();

    let test_file = pack_temp.child("test.txt");
    test_file.write_str("content").unwrap();

    common::forge_cmd()
        .arg("pack")
        .arg(test_file.path())
        .current_dir(pack_temp.path())
        .assert()
        .success();

    common::forge_cmd()
        .arg("seal")
        .current_dir(pack_temp.path())
        .assert()
        .success();

    let zip_files: Vec<_> = fs::read_dir(pack_temp.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("zip"))
        .collect();

    if let Some(zip_file) = zip_files.first() {
        common::forge_cmd()
            .arg("install")
            .arg("--force")
            .arg(zip_file.path())
            .arg("--target")
            .arg(install_temp.path())
            .current_dir(install_temp.path())
            .assert()
            .success();
    }
}

#[test]
fn test_install_skip_existing() {
    let pack_temp = TempDir::new().unwrap();
    let install_temp = TempDir::new().unwrap();

    common::forge_cmd()
        .arg("start")
        .arg("packing")
        .arg("skip-test")
        .current_dir(pack_temp.path())
        .assert()
        .success();

    let test_file = pack_temp.child("test.txt");
    test_file.write_str("content").unwrap();

    common::forge_cmd()
        .arg("pack")
        .arg(test_file.path())
        .current_dir(pack_temp.path())
        .assert()
        .success();

    common::forge_cmd()
        .arg("seal")
        .current_dir(pack_temp.path())
        .assert()
        .success();

    let zip_files: Vec<_> = fs::read_dir(pack_temp.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("zip"))
        .collect();

    if let Some(zip_file) = zip_files.first() {
        common::forge_cmd()
            .arg("install")
            .arg("--skip-existing")
            .arg(zip_file.path())
            .arg("--target")
            .arg(install_temp.path())
            .current_dir(install_temp.path())
            .assert()
            .success();
    }
}

#[test]
fn test_restore_pack() {
    let pack_temp = TempDir::new().unwrap();
    let restore_temp = TempDir::new().unwrap();

    common::forge_cmd()
        .arg("start")
        .arg("packing")
        .arg("restore-test")
        .current_dir(pack_temp.path())
        .assert()
        .success();

    let test_file = pack_temp.child("test.txt");
    test_file.write_str("restore content").unwrap();

    common::forge_cmd()
        .arg("pack")
        .arg(test_file.path())
        .current_dir(pack_temp.path())
        .assert()
        .success();

    common::forge_cmd()
        .arg("seal")
        .current_dir(pack_temp.path())
        .assert()
        .success();

    let zip_files: Vec<_> = fs::read_dir(pack_temp.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("zip"))
        .collect();

    if let Some(zip_file) = zip_files.first() {
        // Restore with --test flag
        common::forge_cmd()
            .arg("restore")
            .arg("--test")
            .arg(zip_file.path())
            .current_dir(restore_temp.path())
            .assert()
            .success();
    }
}

#[test]
fn test_pack_without_starting_fails() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file.write_str("content").unwrap();

    // Try to pack without starting a pack first
    common::forge_cmd()
        .arg("pack")
        .arg(test_file.path())
        .current_dir(temp.path())
        .assert()
        .failure();
}

#[test]
fn test_seal_without_packing_fails() {
    let temp = TempDir::new().unwrap();

    common::forge_cmd()
        .arg("start")
        .arg("packing")
        .arg("empty-pack")
        .current_dir(temp.path())
        .assert()
        .success();

    // Try to seal without packing any files
    common::forge_cmd()
        .arg("seal")
        .current_dir(temp.path())
        .assert()
        .failure();
}
