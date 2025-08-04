use crate::config;
use crate::dotfile::DotFile;

pub fn list_dotfiles(profile: Option<&str>) -> Vec<DotFile> {
    // Fetch dotfiles from database
    match config::get_dotfiles(profile) {
        Ok(dotfiles) => dotfiles,
        Err(e) => {
            eprintln!("Error fetching dotfiles: {}", e);
            Vec::new()
        }
    }
}

pub fn print_dotfiles(profile: Option<&str>) {
    let dotfiles = list_dotfiles(profile);

    let profile_str = profile.unwrap_or("all profiles");
    println!("\nDotfiles ({})", profile_str);

    if dotfiles.is_empty() {
        println!("  No dotfiles found");
        return;
    }

    for dotfile in dotfiles {
        let profile_info = match dotfile.profile {
            Some(ref p) => format!(" (profile: {})", p),
            None => String::new(),
        };

        let status = crate::utils::ui::format_dotfile_status(dotfile.status);

        println!(
            "  [{}] {} → {}{}",
            status,
            dotfile.source.display(),
            dotfile.target.display(),
            profile_info
        );
    }
}
