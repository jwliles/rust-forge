use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Use interactive mode
    #[arg(short = 'I', long)]
    interactive: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Heat (stage) files for symlinking
    Heat {
        /// Files to heat
        files: Vec<PathBuf>,
    },
    /// Create the symlinks for all heated files
    Forge,
    /// Remove symlinks for specific files
    Cool {
        /// Files to cool
        files: Vec<PathBuf>,
    },
    /// Manage profiles
    Profile {
        #[command(subcommand)]
        action: ProfileActions,
    },
}

#[derive(Subcommand)]
enum ProfileActions {
    /// Create a new profile
    Create {
        /// Profile name
        name: String,
    },
    /// List available profiles
    List,
    /// Switch to a profile
    Switch {
        /// Profile name
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Heat { files }) => {
            println!("Heating files: {:?}", files);
            // TODO: implement heat command
        }
        Some(Commands::Forge) => {
            println!("Forging symlinks");
            // TODO: implement forge command
        }
        Some(Commands::Cool { files }) => {
            println!("Cooling files: {:?}", files);
            // TODO: implement cool command
        }
        Some(Commands::Profile { action }) => match action {
            ProfileActions::Create { name } => {
                println!("Creating profile: {}", name);
                // TODO: implement profile create
            }
            ProfileActions::List => {
                println!("Listing profiles");
                // TODO: implement profile list
            }
            ProfileActions::Switch { name } => {
                println!("Switching to profile: {}", name);
                // TODO: implement profile switch
            }
        },
        None => {
            if cli.interactive {
                println!("Starting interactive mode");
                // TODO: implement interactive mode
            } else {
                println!("No command provided. Use --help for more information.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::prelude::*;
    use assert_fs::TempDir;
    use std::fs;

    #[test]
    fn test_basic_symlink_creation() {
        // Create temporary directories for source and target
        let source_dir = TempDir::new().unwrap();
        let target_dir = TempDir::new().unwrap();
        
        // Create a test file in the source directory
        let test_file = source_dir.child("test_config");
        test_file.write_str("test content").unwrap();
        
        // TODO: Replace with actual forge API call once implemented
        // For now, just test that we can create and verify symlinks
        let target_path = target_dir.path().join("test_config");
        #[cfg(unix)]
        std::os::unix::fs::symlink(test_file.path(), &target_path).unwrap();
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(test_file.path(), &target_path).unwrap();
        
        // Verify the symlink exists and points to the correct file
        assert!(target_path.exists());
        assert!(target_path.is_symlink());
        
        // Clean up is automatic with TempDir
    }
    
    #[test]
    fn test_profile_switching() {
        // Create temporary directories
        let source_dir = TempDir::new().unwrap();
        let target_dir = TempDir::new().unwrap();
        
        // Create profile1 files
        let profile1_dir = source_dir.child("profile1");
        profile1_dir.create_dir_all().unwrap();
        let profile1_file = profile1_dir.child("config");
        profile1_file.write_str("profile1 content").unwrap();
        
        // Create profile2 files
        let profile2_dir = source_dir.child("profile2");
        profile2_dir.create_dir_all().unwrap();
        let profile2_file = profile2_dir.child("config");
        profile2_file.write_str("profile2 content").unwrap();
        
        // TODO: Replace with actual forge API calls
        // For now just verify we can create and switch symlinks
        
        // Test "switching" to profile1
        let target_path = target_dir.path().join("config");
        if target_path.exists() {
            fs::remove_file(&target_path).unwrap();
        }
        #[cfg(unix)]
        std::os::unix::fs::symlink(profile1_file.path(), &target_path).unwrap();
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(profile1_file.path(), &target_path).unwrap();
        
        assert!(target_path.is_symlink());
        assert_eq!(fs::read_to_string(&target_path).unwrap(), "profile1 content");
        
        // Test "switching" to profile2
        fs::remove_file(&target_path).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(profile2_file.path(), &target_path).unwrap();
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(profile2_file.path(), &target_path).unwrap();
        
        assert!(target_path.is_symlink());
        assert_eq!(fs::read_to_string(&target_path).unwrap(), "profile2 content");
    }
    
    // Add CLI integration tests
    #[cfg(test)]
    mod cli_tests {
        use assert_cmd::Command;
        use assert_fs::TempDir;
        use assert_fs::prelude::*;
        
        #[test]
        fn test_cli_help() {
            let mut cmd = Command::cargo_bin("dotforge").unwrap();
            cmd.arg("--help");
            cmd.assert().success()
                .stdout(predicates::str::contains("Usage:"));
        }
        
        #[test]
        fn test_heat_command() {
            // Create a temporary directory for our test files
            let temp = TempDir::new().unwrap();
            let test_file = temp.child("test_file");
            test_file.touch().unwrap();
            
            let mut cmd = Command::cargo_bin("dotforge").unwrap();
            cmd.arg("heat").arg(test_file.path());
            
            // For now just check that the command runs without error
            // Later we'll check that the file is actually staged
            cmd.assert().success()
                .stdout(predicates::str::contains("Heating files:"));
        }
    }
}