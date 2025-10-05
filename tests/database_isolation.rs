// Tests to verify database isolation between tests

use assert_fs::TempDir;
use assert_fs::prelude::*;

mod common;

#[test]
fn test_database_isolation_between_tests() {
    // Create two separate test contexts with isolated databases
    let ctx1 = common::TestContext::new();
    let ctx2 = common::TestContext::new();

    // Create two separate managed folders (where .forge directories will be)
    let managed1 = TempDir::new().unwrap();
    let managed2 = TempDir::new().unwrap();

    // Create two separate source directories (where files to be staged live)
    let source1 = TempDir::new().unwrap();
    let source2 = TempDir::new().unwrap();

    // Initialize first forge repo
    ctx1.init_forge_repo(&managed1).unwrap();

    // Create a file in source1 and stage it
    let file1 = source1.child("test1.txt");
    file1.write_str("content1").unwrap();

    ctx1.forge_cmd()
        .arg("stage")
        .arg(file1.path())
        .current_dir(managed1.path())
        .assert()
        .success();

    // Initialize second forge repo
    ctx2.init_forge_repo(&managed2).unwrap();

    // Create a file in source2 and stage it
    let file2 = source2.child("test2.txt");
    file2.write_str("content2").unwrap();

    ctx2.forge_cmd()
        .arg("stage")
        .arg(file2.path())
        .current_dir(managed2.path())
        .assert()
        .success();

    // List files in first repo - should only see test1.txt
    let output1 = ctx1
        .forge_cmd()
        .arg("list")
        .current_dir(managed1.path())
        .output()
        .unwrap();

    let stdout1 = String::from_utf8_lossy(&output1.stdout);

    // First repo should contain test1.txt but not test2.txt
    assert!(
        stdout1.contains("test1.txt"),
        "First repo list should contain test1.txt"
    );
    assert!(
        !stdout1.contains("test2.txt"),
        "First repo should NOT contain test2.txt"
    );

    // List files in second repo - should only see test2.txt
    let output2 = ctx2
        .forge_cmd()
        .arg("list")
        .current_dir(managed2.path())
        .output()
        .unwrap();

    let stdout2 = String::from_utf8_lossy(&output2.stdout);

    // Debug output
    eprintln!("=== First repo output ===\n{}", stdout1);
    eprintln!("=== Second repo output ===\n{}", stdout2);

    // Second repo should contain test2.txt but not test1.txt
    assert!(
        stdout2.contains("test2.txt"),
        "Second repo list should contain test2.txt"
    );
    assert!(
        !stdout2.contains("test1.txt"),
        "Second repo should NOT contain files from first repo - database isolation failed!"
    );
}

#[test]
fn test_multiple_inits_dont_interfere() {
    // Verify that running multiple init commands doesn't pollute global database
    let temp1 = TempDir::new().unwrap();
    let temp2 = TempDir::new().unwrap();
    let temp3 = TempDir::new().unwrap();

    // Init multiple repos
    common::forge_cmd()
        .arg("init")
        .current_dir(temp1.path())
        .assert()
        .success();

    common::forge_cmd()
        .arg("init")
        .current_dir(temp2.path())
        .assert()
        .success();

    common::forge_cmd()
        .arg("init")
        .current_dir(temp3.path())
        .assert()
        .success();

    // Each should have its own .forge directory
    assert!(temp1.path().join(".forge").exists());
    assert!(temp2.path().join(".forge").exists());
    assert!(temp3.path().join(".forge").exists());

    // Verify they can each stage files independently
    let file1 = temp1.child("file1.txt");
    file1.write_str("content1").unwrap();

    common::forge_cmd()
        .arg("stage")
        .arg(file1.path())
        .current_dir(temp1.path())
        .assert()
        .success();

    let file2 = temp2.child("file2.txt");
    file2.write_str("content2").unwrap();

    common::forge_cmd()
        .arg("stage")
        .arg(file2.path())
        .current_dir(temp2.path())
        .assert()
        .success();

    // Success if no panics/errors
}

#[test]
fn test_no_cross_contamination_after_cleanup() {
    // Test that cleaning up one test doesn't affect another
    let temp = TempDir::new().unwrap();

    common::init_forge_repo(&temp).unwrap();

    let test_file = temp.child("test.txt");
    test_file.write_str("content").unwrap();

    // Stage and link
    common::forge_cmd()
        .arg("stage")
        .arg(test_file.path())
        .current_dir(temp.path())
        .assert()
        .success();

    // Drop temp - this should clean up the test database
    drop(temp);

    // Create new temp and verify it starts fresh
    let temp2 = TempDir::new().unwrap();
    common::init_forge_repo(&temp2).unwrap();

    let output = common::forge_cmd()
        .arg("list")
        .current_dir(temp2.path())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should be empty or not contain files from previous test
    assert!(
        !stdout.contains("test.txt"),
        "New test should not see files from previous test!"
    );
}
