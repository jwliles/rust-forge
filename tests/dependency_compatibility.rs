use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn forge(root: &Path, args: &[&str]) -> std::io::Result<Output> {
    Command::new(env!("CARGO_BIN_EXE_forge"))
        .current_dir(root)
        .env("FORGE_TEST_DB", root.join("config.db"))
        .env("FORGE_TEST_CONFIG_DIR", root.join("config"))
        .args(args)
        .output()
}

#[test]
fn installs_pack_created_before_dependency_upgrade() -> Result<(), Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    // Created by the pre-upgrade Forge 0.5.1 binary with zip 0.6.6 and toml 0.8.23.
    fs::write(
        temp.path().join("legacy.zip"),
        include_bytes!("fixtures/forge-0.5.1.zip.fixture"),
    )?;
    let output = forge(
        temp.path(),
        &["install", "legacy.zip", "--target", "installed"],
    )?;
    assert!(output.status.success(), "{:?}", output);
    assert_eq!(
        fs::read_to_string(temp.path().join("installed/original.txt"))?,
        "legacy pack content\n"
    );
    Ok(())
}

#[test]
fn creates_and_installs_pack_with_upgraded_dependencies() -> Result<(), Box<dyn std::error::Error>>
{
    let temp = tempfile::tempdir()?;
    fs::write(temp.path().join("sample.txt"), "round-trip content\n")?;
    for args in [
        vec!["init"],
        vec!["start", "packing", "compat"],
        vec!["pack", "--scope", "compat", "sample.txt"],
        vec!["seal", "--scope", "compat"],
    ] {
        let output = forge(temp.path(), &args)?;
        assert!(output.status.success(), "{:?}", output);
    }
    let archive = fs::read_dir(temp.path().join(".forge/archives"))?
        .next()
        .ok_or("no sealed archive")??
        .path();
    let archive = archive.to_str().ok_or("invalid archive path")?;
    let output = forge(temp.path(), &["install", archive, "--target", "installed"])?;
    assert!(output.status.success(), "{:?}", output);
    assert_eq!(
        fs::read_to_string(temp.path().join("installed/sample.txt"))?,
        "round-trip content\n"
    );
    Ok(())
}
