use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod cli;
mod config;
mod dotfile;
mod scanner;
mod symlink;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Use interactive mode
    #[arg(short = 'I', long)]
    interactive: bool,

    /// Enable verbose output
    #[arg(short = 'v', long)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a directory as a forge managed folder
    Init {
        /// Name for this forge repository (defaults to directory name)
        #[arg(short, long)]
        name: Option<String>,

        /// Directory to initialize (defaults to current directory)
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
    /// Unstage staged files by target path
    Unstage {
        /// Files or directories to unstage
        files: Vec<PathBuf>,
        /// Process directories recursively (all levels)
        #[arg(short, long)]
        recursive: bool,
        /// Maximum recursion depth for directories (overrides --recursive)
        #[arg(long)]
        depth: Option<usize>,
    },
    Purge {
        /// Folder to purge (if omitted, purges everything)
        folder: Option<PathBuf>,
        /// Process subfolders recursively
        #[arg(short, long, default_value_t = true)]
        recursive: bool,
    },

    /// Stage files or directories for tracking (temporary, requires linking to make permanent)
    Stage {
        /// Files or directories to stage
        files: Vec<PathBuf>,

        /// Process directories recursively (all levels)
        #[arg(short, long)]
        recursive: bool,

        /// Maximum recursion depth for directories (overrides --recursive)
        #[arg(long)]
        depth: Option<usize>,
    },
    /// Create symlinks for staged/tracked files
    Link {
        /// Files to link (if not specified, links all staged files)
        files: Vec<PathBuf>,
    },
    /// Remove symlinks but keep files in forge folder
    Unlink {
        /// Files to unlink
        files: Vec<PathBuf>,

        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
    /// Remove files from forge folder (keeps original files)
    Remove {
        /// Files to remove
        files: Vec<PathBuf>,

        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
    /// Delete files completely from the system
    Delete {
        /// Files to delete
        files: Vec<PathBuf>,

        /// Skip confirmation prompt (USE WITH CAUTION)
        #[arg(short, long)]
        yes: bool,
    },
    /// List tracked files or profiles
    List {
        /// List profiles instead of files
        #[arg(long)]
        profiles: bool,

        /// Filter by profile name
        #[arg(short, long)]
        profile: Option<String>,
    },
    /// Switch to a profile
    Switch {
        /// Profile name
        name: String,
    },
    /// Create a new profile at a specific location
    New {
        /// Create a new profile
        #[arg(short, long)]
        profile: String,

        /// Location for the new profile
        path: PathBuf,
    },
    /// Manage profiles (legacy, use switch/new/list commands instead)
    Profile {
        #[command(subcommand)]
        action: ProfileActions,
    },
    /// Start packing files for portable configuration bundles
    Start {
        #[command(subcommand)]
        action: StartActions,
    },
    /// Add files to an existing pack
    Pack {
        /// Files to add to the current pack
        files: Vec<PathBuf>,
        /// Pack scope (defaults to current working directory name)
        #[arg(short, long)]
        scope: Option<String>,
        /// Process directories recursively (all levels)
        #[arg(short, long)]
        recursive: bool,
        /// Maximum recursion depth for directories (overrides --recursive)
        #[arg(long)]
        depth: Option<usize>,
        /// Show what would be packed without actually packing
        #[arg(long)]
        dry_run: bool,
    },
    /// Seal the current pack into a portable archive
    Seal {
        /// Pack scope to seal (defaults to current working directory name)
        #[arg(short, long)]
        scope: Option<String>,
    },
    /// Install a sealed pack on a new system
    Install {
        /// Path to the pack archive (.zip file)
        archive: PathBuf,
        /// Skip conflict warnings and overwrite existing files
        #[arg(short, long)]
        force: bool,
        /// Skip files that already exist (opposite of --force)
        #[arg(long)]
        skip_existing: bool,
        /// Target directory for installation (defaults to current working directory)
        #[arg(short, long)]
        target: Option<PathBuf>,
        /// Map home directory paths to current user's home
        #[arg(long)]
        map_home: bool,
        /// Show what would be installed without actually installing
        #[arg(long)]
        dry_run: bool,
    },
    /// Restore a sealed pack to original locations on current system
    Restore {
        /// Path to the pack archive (.zip file)
        archive: PathBuf,
        /// Skip conflict warnings and overwrite existing files
        #[arg(short, long)]
        force: bool,
        /// Skip files that already exist (opposite of --force)
        #[arg(long)]
        skip_existing: bool,
        /// Test mode: restore to current directory using filenames only (safe for testing)
        #[arg(long)]
        test: bool,
        /// Show what would be restored without actually restoring
        #[arg(long)]
        dry_run: bool,
    },
    /// Update files in an existing pack
    Repack {
        /// Pack scope to repack (defaults to current working directory name)
        #[arg(short, long)]
        scope: Option<String>,
        /// Specific files to repack (defaults to all files in pack)
        files: Vec<PathBuf>,
    },
    /// Remove files from a pack
    Unpack {
        /// Files to remove from pack
        files: Vec<PathBuf>,
        /// Pack scope (defaults to current working directory name)
        #[arg(short, long)]
        scope: Option<String>,
    },
    /// Explain pack contents and installation plan
    Explain {
        /// Path to the pack archive (.zip file)
        archive: PathBuf,
        /// Show installation plan for install command (defaults to current directory)
        #[arg(long)]
        install: bool,
        /// Show restoration plan for restore command  
        #[arg(long)]
        restore: bool,
        /// Target directory for install plan preview
        #[arg(short, long)]
        target: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum StartActions {
    /// Start packing files into a portable bundle
    Packing {
        /// Unique identifier for this pack
        scope: String,
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

    // Set up verbose logging if requested
    if cli.verbose {
        unsafe {
            std::env::set_var("FORGE_VERBOSE", "1");
        }
    }

    match &cli.command {
        Some(Commands::Init { name, dir }) => {
            cli::commands::init_command(name.as_deref(), dir.as_deref());
        }
        Some(Commands::Stage {
            files,
            recursive,
            depth,
        }) => {
            cli::commands::stage_command(files, *recursive, *depth);
        }
        Some(Commands::Link { files }) => {
            cli::commands::link_command(files);
        }
        Some(Commands::Unlink { files, yes }) => {
            cli::commands::unlink_command(files, *yes);
        }
        Some(Commands::Remove { files, yes }) => {
            cli::commands::remove_command(files, *yes);
        }
        Some(Commands::Delete { files, yes }) => {
            cli::commands::delete_command(files, *yes);
        }
        Some(Commands::List { profile, profiles }) => {
            if *profiles {
                cli::commands::profile::list();
            } else {
                cli::commands::list_command(profile.as_deref());
            }
        }
        Some(Commands::Switch { name }) => {
            cli::commands::profile::switch(name);
        }
        Some(Commands::New { profile, path }) => {
            // Initialize the directory as a forge managed folder with the profile name
            cli::commands::init_command(Some(profile), Some(path.as_path()));
        }
        Some(Commands::Profile { action }) => match action {
            ProfileActions::Create { name } => {
                println!(
                    "Note: This command is deprecated, please use 'forge new --profile {}' instead",
                    name
                );
                cli::commands::profile::create(name);
            }
            ProfileActions::List => {
                println!(
                    "Note: This command is deprecated, please use 'forge list --profiles' instead"
                );
                cli::commands::profile::list();
            }
            ProfileActions::Switch { name } => {
                println!(
                    "Note: This command is deprecated, please use 'forge switch {}' instead",
                    name
                );
                cli::commands::profile::switch(name);
            }
        },
        Some(Commands::Start { action }) => match action {
            StartActions::Packing { scope } => {
                cli::commands::pack::start_packing(scope);
            }
        },
        Some(Commands::Pack {
            files,
            scope,
            recursive,
            depth,
            dry_run,
        }) => {
            cli::commands::pack::pack_files(files, scope.as_deref(), *recursive, *depth, *dry_run);
        }
        Some(Commands::Seal { scope }) => {
            cli::commands::pack::seal_pack(scope.as_deref());
        }
        Some(Commands::Install {
            archive,
            force,
            skip_existing,
            target,
            map_home,
            dry_run,
        }) => {
            cli::commands::pack::install_pack(
                archive,
                *force,
                *skip_existing,
                target.as_deref(),
                *map_home,
                *dry_run,
            );
        }
        Some(Commands::Restore {
            archive,
            force,
            skip_existing,
            test,
            dry_run,
        }) => {
            cli::commands::pack::restore_pack(archive, *force, *skip_existing, *test, *dry_run);
        }
        Some(Commands::Repack { scope, files }) => {
            cli::commands::pack::repack_files(scope.as_deref(), files);
        }
        Some(Commands::Unpack { files, scope }) => {
            cli::commands::pack::unpack_files(files, scope.as_deref());
        }
        Some(Commands::Explain {
            archive,
            install,
            restore,
            target,
        }) => {
            cli::commands::pack::explain_pack(archive, *install, *restore, target.as_deref());
        }
        None => {
            if cli.interactive {
                println!("Starting interactive mode");
                // TODO: implement interactive mode
            } else {
                println!("No command provided. Use --help for more information.");
            }
        }
        Some(Commands::Unstage {
            files,
            recursive,
            depth,
        }) => {
            cli::commands::unstage_command(files, *recursive, *depth);
        }
        Some(Commands::Purge { folder, recursive }) => {
            let folder_path = match folder {
                Some(f) => f.clone(),
                None => match config::get_active_managed_folder() {
                    Ok(Some((_name, path))) => path,
                    _ => {
                        eprintln!("No folder specified and no active managed folder found.");
                        return;
                    }
                },
            };
            cli::commands::purge_command_safe(&folder_path, *recursive);
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::TempDir;
    use assert_fs::prelude::*;
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
        assert_eq!(
            fs::read_to_string(&target_path).unwrap(),
            "profile1 content"
        );

        // Test "switching" to profile2
        fs::remove_file(&target_path).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(profile2_file.path(), &target_path).unwrap();
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(profile2_file.path(), &target_path).unwrap();

        assert!(target_path.is_symlink());
        assert_eq!(
            fs::read_to_string(&target_path).unwrap(),
            "profile2 content"
        );
    }

    // Add CLI integration tests
    #[cfg(test)]
    mod cli_tests {
        use assert_cmd::Command;
        use assert_fs::TempDir;
        use assert_fs::prelude::*;

        #[test]
        fn test_cli_help() {
            let mut cmd = Command::cargo_bin("forge").unwrap();
            cmd.arg("--help");
            cmd.assert()
                .success()
                .stdout(predicates::str::contains("Usage:"));
        }

        #[test]
        fn test_stage_command() {
            // Create a temporary directory for our test files
            let temp = TempDir::new().unwrap();
            let test_file = temp.child("test_file");
            test_file.touch().unwrap();

            // Initialize forge repository first
            let mut init_cmd = Command::cargo_bin("forge").unwrap();
            init_cmd.arg("init").current_dir(temp.path());
            init_cmd.assert().success();

            let mut cmd = Command::cargo_bin("forge").unwrap();
            cmd.arg("stage")
                .arg(test_file.path())
                .current_dir(temp.path());

            // For now just check that the command runs without error
            cmd.assert().success();
        }
    }
}
