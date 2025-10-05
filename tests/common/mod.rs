// Common test utilities shared across all tests

use assert_cmd::Command;
use assert_fs::TempDir;
use std::path::Path;
use std::sync::Mutex;

/// Global counter for test database isolation
static TEST_DB_COUNTER: Mutex<u32> = Mutex::new(0);

/// Test context that maintains database paths for a single test
pub struct TestContext {
    db_path: String,
    config_path: String,
}

impl TestContext {
    /// Create a new test context with unique database paths
    pub fn new() -> Self {
        let mut counter = TEST_DB_COUNTER.lock().unwrap();
        *counter += 1;
        let id = *counter;
        drop(counter); // Release lock

        let db_path = format!("/tmp/forge_test_{}.db", id);
        let config_path = format!("/tmp/forge_test_config_{}", id);

        // Clean up any existing test artifacts from previous runs
        let _ = std::fs::remove_file(&db_path);
        let _ = std::fs::remove_dir_all(&config_path);

        Self {
            db_path,
            config_path,
        }
    }

    /// Get the database path for this context
    pub fn db_path(&self) -> &str {
        &self.db_path
    }

    /// Get the config path for this context
    pub fn config_path(&self) -> &str {
        &self.config_path
    }

    /// Create a forge command with this test's isolated database
    pub fn forge_cmd(&self) -> Command {
        let mut cmd = Command::cargo_bin("forge").expect("Failed to find forge binary");
        cmd.env("FORGE_TEST_DB", &self.db_path);
        cmd.env("FORGE_TEST_CONFIG_DIR", &self.config_path);
        cmd
    }

    /// Initialize a forge repository in a temporary directory with isolated database
    pub fn init_forge_repo(&self, temp: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
        self.forge_cmd()
            .arg("init")
            .current_dir(temp.path())
            .assert()
            .success();
        Ok(())
    }
}

/// Create a test command for the forge binary with isolated database
/// DEPRECATED: Use TestContext instead for proper per-test isolation
pub fn forge_cmd() -> Command {
    TestContext::new().forge_cmd()
}

/// Initialize a forge repository in a temporary directory with isolated database
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
