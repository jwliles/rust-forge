// Common test utilities shared across all tests

use assert_cmd::Command;
use assert_fs::TempDir;
use std::path::Path;

/// Create a test command for the forge binary
pub fn forge_cmd() -> Command {
    Command::cargo_bin("forge").expect("Failed to find forge binary")
}

/// Initialize a forge repository in a temporary directory
pub fn init_forge_repo(temp: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = forge_cmd();
    cmd.arg("init").current_dir(temp.path());
    cmd.assert().success();
    Ok(())
}

/// Create a test file with content in the given directory
pub fn create_test_file(
    dir: &Path,
    filename: &str,
    content: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let file_path = dir.join(filename);
    std::fs::write(&file_path, content)?;
    Ok(file_path)
}

/// Create a test directory structure
pub fn create_test_dir_tree(
    base: &Path,
    structure: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    for path in structure {
        let full_path = base.join(path);
        if path.ends_with('/') {
            std::fs::create_dir_all(&full_path)?;
        } else {
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&full_path, format!("Content of {}", path))?;
        }
    }
    Ok(())
}
